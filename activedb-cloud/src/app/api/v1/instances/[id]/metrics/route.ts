import { NextResponse } from "next/server";
import { db } from "@/lib/db";
import { instances } from "@/db/schema";
import { eq, and, isNull } from "drizzle-orm";
import { getAuthenticatedUserOrApiKey, errorResponse } from "@/lib/api-helpers";

type Params = { params: Promise<{ id: string }> };

// GET /api/v1/instances/:id/metrics?period=1h&metrics=cpu,memory,queries
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

  const url = new URL(request.url);
  const period = url.searchParams.get("period") ?? "1h";

  // TODO: fetch from Prometheus/CloudWatch
  // For now, return stub data
  return NextResponse.json({
    instanceId: id,
    period,
    dataPoints: [],
  });
}
