import { NextResponse } from "next/server";
import { db } from "@/lib/db";
import { instances } from "@/db/schema";
import { eq, and, isNull } from "drizzle-orm";
import { getAuthenticatedUserOrApiKey, errorResponse } from "@/lib/api-helpers";

type Params = { params: Promise<{ id: string }> };

export async function POST(request: Request, { params }: Params) {
  const { id } = await params;
  const auth = await getAuthenticatedUserOrApiKey(request);
  if (!auth) return errorResponse("UNAUTHORIZED", "Authentication required", 401);

  const [instance] = await db
    .select()
    .from(instances)
    .where(and(eq(instances.id, id), eq(instances.ownerId, auth.userId), isNull(instances.deletedAt)))
    .limit(1);

  if (!instance) return errorResponse("NOT_FOUND", "Instance not found", 404);
  if (instance.status !== "running") {
    return errorResponse("CONFLICT", `Cannot stop instance in '${instance.status}' state`, 409);
  }

  const [updated] = await db
    .update(instances)
    .set({ status: "stopped", updatedAt: new Date() })
    .where(eq(instances.id, id))
    .returning();

  // TODO: trigger K8s scale down to 0

  return NextResponse.json({ instance: updated });
}
