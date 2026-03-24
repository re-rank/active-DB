import { NextResponse } from "next/server";
import { db } from "@/lib/db";
import { usageRecords, instances } from "@/db/schema";
import { eq, and, isNull, gte } from "drizzle-orm";
import { getAuthenticatedUser, errorResponse } from "@/lib/api-helpers";

// GET /api/v1/billing/usage
export async function GET() {
  const user = await getAuthenticatedUser();
  if (!user) return errorResponse("UNAUTHORIZED", "Authentication required", 401);

  const startOfMonth = new Date();
  startOfMonth.setDate(1);
  startOfMonth.setHours(0, 0, 0, 0);

  const userInstances = await db
    .select({ id: instances.id })
    .from(instances)
    .where(and(eq(instances.ownerId, user.id), isNull(instances.deletedAt)));

  const instanceIds = userInstances.map((i) => i.id);

  if (instanceIds.length === 0) {
    return NextResponse.json({ usage: { apiCalls: 0, storageBytes: 0, networkBytes: 0 } });
  }

  // Aggregate usage for current billing period
  const records = await db
    .select()
    .from(usageRecords)
    .where(gte(usageRecords.periodStart, startOfMonth));

  const filtered = records.filter((r) => instanceIds.includes(r.instanceId));

  const totals = filtered.reduce(
    (acc, r) => ({
      apiCalls: acc.apiCalls + (r.apiCalls ?? 0),
      storageBytes: acc.storageBytes + (r.storageBytes ?? 0),
      networkBytes: acc.networkBytes + (r.networkBytes ?? 0),
    }),
    { apiCalls: 0, storageBytes: 0, networkBytes: 0 },
  );

  return NextResponse.json({ usage: totals });
}
