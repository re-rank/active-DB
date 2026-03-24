import { db } from "@/lib/db";
import { instances } from "@/db/schema";
import { eq, and, isNull } from "drizzle-orm";
import { getAuthenticatedUserOrApiKey, errorResponse } from "@/lib/api-helpers";

type Params = { params: Promise<{ id: string }> };

// GET /api/v1/instances/:id/logs/stream — SSE 실시간 로그
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
    start(controller) {
      // TODO: connect to CloudWatch Logs tail or K8s log stream
      const send = (data: unknown) => {
        controller.enqueue(encoder.encode(`data: ${JSON.stringify(data)}\n\n`));
      };
      send({ message: "Log streaming connected", instanceId: id });
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
