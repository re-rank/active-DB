import { NextResponse } from "next/server";
import { db } from "@/lib/db";
import { instances } from "@/db/schema";
import { eq, and, isNull } from "drizzle-orm";
import {
  getAuthenticatedUserOrApiKey,
  errorResponse,
} from "@/lib/api-helpers";

// GET /api/v1/instances — 목록 조회
export async function GET(request: Request) {
  const auth = await getAuthenticatedUserOrApiKey(request);
  if (!auth) return errorResponse("UNAUTHORIZED", "Authentication required", 401);

  const results = await db
    .select()
    .from(instances)
    .where(
      and(
        eq(instances.ownerId, auth.userId),
        eq(instances.ownerType, "user"),
        isNull(instances.deletedAt),
      ),
    );

  return NextResponse.json({ instances: results });
}

// POST /api/v1/instances — 인스턴스 생성
export async function POST(request: Request) {
  const auth = await getAuthenticatedUserOrApiKey(request);
  if (!auth) return errorResponse("UNAUTHORIZED", "Authentication required", 401);

  const body = await request.json();
  const { name, region = "us-east-1", plan = "free", config: instanceConfig = {} } = body;

  if (!name || typeof name !== "string" || name.length < 1) {
    return errorResponse("INVALID_REQUEST", "Instance name is required", 400);
  }

  const tierLimits: Record<string, { cpu: string; memory: string; storage: string }> = {
    free: { cpu: "500m", memory: "512Mi", storage: "500Mi" },
    pro: { cpu: "2000m", memory: "4Gi", storage: "10Gi" },
    enterprise: { cpu: "8000m", memory: "32Gi", storage: "100Gi" },
  };

  const limits = tierLimits[plan] ?? tierLimits.free;

  const [instance] = await db
    .insert(instances)
    .values({
      name,
      ownerType: "user",
      ownerId: auth.userId,
      region,
      tier: plan,
      status: "provisioning",
      cpuLimit: limits.cpu,
      memoryLimit: limits.memory,
      storageSize: limits.storage,
      config: instanceConfig,
    })
    .returning();

  // TODO: trigger provisioning via provisioner

  return NextResponse.json({ instance }, { status: 201 });
}
