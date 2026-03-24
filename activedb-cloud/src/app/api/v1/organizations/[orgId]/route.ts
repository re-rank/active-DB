import { NextResponse } from "next/server";
import { db } from "@/lib/db";
import { organizations, orgMembers } from "@/db/schema";
import { eq, and } from "drizzle-orm";
import { getAuthenticatedUser, errorResponse } from "@/lib/api-helpers";

type Params = { params: Promise<{ orgId: string }> };

// GET /api/v1/organizations/:orgId
export async function GET(request: Request, { params }: Params) {
  const { orgId } = await params;
  const user = await getAuthenticatedUser();
  if (!user) return errorResponse("UNAUTHORIZED", "Authentication required", 401);

  const [membership] = await db
    .select()
    .from(orgMembers)
    .where(and(eq(orgMembers.orgId, orgId), eq(orgMembers.userId, user.id)))
    .limit(1);

  if (!membership) return errorResponse("FORBIDDEN", "Not a member of this organization", 403);

  const [org] = await db.select().from(organizations).where(eq(organizations.id, orgId)).limit(1);
  if (!org) return errorResponse("NOT_FOUND", "Organization not found", 404);

  return NextResponse.json({ organization: { ...org, role: membership.role } });
}

// PATCH /api/v1/organizations/:orgId
export async function PATCH(request: Request, { params }: Params) {
  const { orgId } = await params;
  const user = await getAuthenticatedUser();
  if (!user) return errorResponse("UNAUTHORIZED", "Authentication required", 401);

  const [membership] = await db
    .select()
    .from(orgMembers)
    .where(and(eq(orgMembers.orgId, orgId), eq(orgMembers.userId, user.id)))
    .limit(1);

  if (!membership || !["admin", "owner"].includes(membership.role)) {
    return errorResponse("FORBIDDEN", "Admin access required", 403);
  }

  const { name } = await request.json();
  const [updated] = await db
    .update(organizations)
    .set({ name, updatedAt: new Date() })
    .where(eq(organizations.id, orgId))
    .returning();

  return NextResponse.json({ organization: updated });
}
