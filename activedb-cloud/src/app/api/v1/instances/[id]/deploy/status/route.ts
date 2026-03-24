import { db } from "@/lib/db";
import { deployments, instances } from "@/db/schema";
import { eq, and, isNull, desc } from "drizzle-orm";
import { getAuthenticatedUserOrApiKey, errorResponse } from "@/lib/api-helpers";

type Params = { params: Promise<{ id: string }> };

// GET /api/v1/instances/:id/deploy/status — SSE 배포 상태
export async function GET(request: Request, { params }: Params) {
  const { id } = await params;
  const auth = await getAuthenticatedUserOrApiKey(request);
  if (!auth) return errorResponse("UNAUTHORIZED", "Authentication required", 401);

  const [instance] = await db
    .select()
    .from(instances)
    .where(and(eq(instances.id, id), eq(instances.ownerId, auth.userId), isNull(instances.deletedAt)))
    .limit(1);

  if (!instance) return errorResponse("NOT_FOUND", "Instance not found", 404);

  const encoder = new TextEncoder();
  const stream = new ReadableStream({
    async start(controller) {
      const send = (data: unknown) => {
        controller.enqueue(encoder.encode(`data: ${JSON.stringify(data)}\n\n`));
      };

      // Poll deployment status
      const poll = async () => {
        const [latest] = await db
          .select()
          .from(deployments)
          .where(eq(deployments.instanceId, id))
          .orderBy(desc(deployments.startedAt))
          .limit(1);

        if (latest) {
          send({ stage: latest.status, deploymentId: latest.id });
          if (latest.status === "completed" || latest.status === "failed") {
            controller.close();
            return;
          }
        }
        setTimeout(poll, 2000);
      };

      poll();
    },
  });

  return new Response(stream, {
    headers: {
      "Content-Type": "text/event-stream",
      "Cache-Control": "no-cache",
      Connection: "keep-alive",
    },
  });
}
