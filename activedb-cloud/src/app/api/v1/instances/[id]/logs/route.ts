import { NextResponse } from "next/server";
import { db } from "@/lib/db";
import { instances } from "@/db/schema";
import { eq, and, isNull } from "drizzle-orm";
import { getAuthenticatedUserOrApiKey, errorResponse } from "@/lib/api-helpers";

type Params = { params: Promise<{ id: string }> };

// GET /api/v1/instances/:id/logs?since=...&limit=100&level=error
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

  // TODO: fetch from CloudWatch Logs
  return NextResponse.json({ instanceId: id, logs: [] });
}
