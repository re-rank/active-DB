import { NextResponse } from "next/server";
import { db } from "@/lib/db";
import { instances } from "@/db/schema";
import { eq, and, isNull } from "drizzle-orm";
import {
  getAuthenticatedUserOrApiKey,
  errorResponse,
} from "@/lib/api-helpers";

type Params = { params: Promise<{ id: string }> };

// GET /api/v1/instances/:id — 상세 조회
export async function GET(request: Request, { params }: Params) {
  const { id } = await params;
  const auth = await getAuthenticatedUserOrApiKey(request);
  if (!auth) return errorResponse("UNAUTHORIZED", "Authentication required", 401);

  const [instance] = await db
    .select()
    .from(instances)
    .where(
      and(
        eq(instances.id, id),
        eq(instances.ownerId, auth.userId),
        isNull(instances.deletedAt),
      ),
    )
    .limit(1);

  if (!instance) return errorResponse("NOT_FOUND", "Instance not found", 404);

  return NextResponse.json({ instance });
}

// PATCH /api/v1/instances/:id — 수정
export async function PATCH(request: Request, { params }: Params) {
  const { id } = await params;
  const auth = await getAuthenticatedUserOrApiKey(request);
  if (!auth) return errorResponse("UNAUTHORIZED", "Authentication required", 401);

  const body = await request.json();
  const allowedFields = ["name", "config"] as const;
  const updates: Record<string, unknown> = { updatedAt: new Date() };

  for (const field of allowedFields) {
    if (body[field] !== undefined) updates[field] = body[field];
  }

  const [instance] = await db
    .update(instances)
    .set(updates)
    .where(
      and(
        eq(instances.id, id),
        eq(instances.ownerId, auth.userId),
        isNull(instances.deletedAt),
      ),
    )
    .returning();

  if (!instance) return errorResponse("NOT_FOUND", "Instance not found", 404);

  return NextResponse.json({ instance });
}

// DELETE /api/v1/instances/:id — 삭제 (soft delete)
export async function DELETE(request: Request, { params }: Params) {
  const { id } = await params;
  const auth = await getAuthenticatedUserOrApiKey(request);
  if (!auth) return errorResponse("UNAUTHORIZED", "Authentication required", 401);

  const [instance] = await db
    .update(instances)
    .set({ status: "terminating", deletedAt: new Date(), updatedAt: new Date() })
    .where(
      and(
        eq(instances.id, id),
        eq(instances.ownerId, auth.userId),
        isNull(instances.deletedAt),
      ),
    )
    .returning();

  if (!instance) return errorResponse("NOT_FOUND", "Instance not found", 404);

  // TODO: trigger de-provisioning

  return NextResponse.json({ instance });
}
