import { NextResponse } from "next/server";
import { db } from "@/lib/db";
import { instances, deployments } from "@/db/schema";
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

  const body = await request.json();
  const { imageTag } = body;

  if (!imageTag) return errorResponse("INVALID_REQUEST", "imageTag is required", 400);

  const [deployment] = await db
    .insert(deployments)
    .values({
      instanceId: id,
      triggeredBy: auth.userId,
      imageTag,
      status: "pending",
    })
    .returning();

  // TODO: trigger build pipeline (S3 upload → CodeBuild → ECR → K8s rolling update)

  return NextResponse.json({ deployment }, { status: 201 });
}
