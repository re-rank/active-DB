import { NextResponse } from "next/server";
import { db } from "@/lib/db";
import { orgMembers } from "@/db/schema";
import { eq, and } from "drizzle-orm";
import { getAuthenticatedUser, errorResponse } from "@/lib/api-helpers";

type Params = { params: Promise<{ orgId: string; userId: string }> };

// PATCH /api/v1/organizations/:orgId/members/:userId — 역할 변경
export async function PATCH(request: Request, { params }: Params) {
  const { orgId, userId } = await params;
  const user = await getAuthenticatedUser();
  if (!user) return errorResponse("UNAUTHORIZED", "Authentication required", 401);

  const [myMembership] = await db
    .select()
    .from(orgMembers)
    .where(and(eq(orgMembers.orgId, orgId), eq(orgMembers.userId, user.id)))
    .limit(1);

  if (!myMembership || !["admin", "owner"].includes(myMembership.role)) {
    return errorResponse("FORBIDDEN", "Admin access required", 403);
  }

  const { role } = await request.json();
  const validRoles = ["viewer", "developer", "admin"];
  if (!validRoles.includes(role)) {
    return errorResponse("INVALID_REQUEST", "Invalid role", 400);
  }

  const [updated] = await db
    .update(orgMembers)
    .set({ role })
    .where(and(eq(orgMembers.orgId, orgId), eq(orgMembers.userId, userId)))
    .returning();

  if (!updated) return errorResponse("NOT_FOUND", "Member not found", 404);

  return NextResponse.json({ member: updated });
}

// DELETE /api/v1/organizations/:orgId/members/:userId — 멤버 제거
export async function DELETE(request: Request, { params }: Params) {
  const { orgId, userId } = await params;
  const user = await getAuthenticatedUser();
  if (!user) return errorResponse("UNAUTHORIZED", "Authentication required", 401);

  const [myMembership] = await db
    .select()
    .from(orgMembers)
    .where(and(eq(orgMembers.orgId, orgId), eq(orgMembers.userId, user.id)))
    .limit(1);

  if (!myMembership || !["admin", "owner"].includes(myMembership.role)) {
    return errorResponse("FORBIDDEN", "Admin access required", 403);
  }

  // Cannot remove owner
  const [targetMembership] = await db
    .select()
    .from(orgMembers)
    .where(and(eq(orgMembers.orgId, orgId), eq(orgMembers.userId, userId)))
    .limit(1);

  if (targetMembership?.role === "owner") {
    return errorResponse("CONFLICT", "Cannot remove organization owner", 409);
  }

  await db
    .delete(orgMembers)
    .where(and(eq(orgMembers.orgId, orgId), eq(orgMembers.userId, userId)));

  return NextResponse.json({ success: true });
}
