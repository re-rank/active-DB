import { NextResponse } from "next/server";
import { randomBytes } from "crypto";
import { db } from "@/lib/db";
import { orgInvites, orgMembers } from "@/db/schema";
import { eq, and } from "drizzle-orm";
import { getAuthenticatedUser, errorResponse } from "@/lib/api-helpers";

type Params = { params: Promise<{ orgId: string }> };

// POST /api/v1/organizations/:orgId/invites
export async function POST(request: Request, { params }: Params) {
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

  const { email, role = "developer" } = await request.json();
  if (!email) return errorResponse("INVALID_REQUEST", "Email is required", 400);

  const token = randomBytes(32).toString("base64url");
  const expiresAt = new Date(Date.now() + 7 * 24 * 60 * 60 * 1000); // 7 days

  const [invite] = await db
    .insert(orgInvites)
    .values({ orgId, email, role, token, invitedBy: user.id, expiresAt })
    .returning();

  // TODO: send email with invite link

  return NextResponse.json({ invite }, { status: 201 });
}
