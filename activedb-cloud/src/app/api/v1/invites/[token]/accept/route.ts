import { NextResponse } from "next/server";
import { db } from "@/lib/db";
import { orgInvites, orgMembers } from "@/db/schema";
import { eq, and, isNull, gt } from "drizzle-orm";
import { getAuthenticatedUser, errorResponse } from "@/lib/api-helpers";

type Params = { params: Promise<{ token: string }> };

// POST /api/v1/invites/:token/accept
export async function POST(request: Request, { params }: Params) {
  const { token } = await params;
  const user = await getAuthenticatedUser();
  if (!user) return errorResponse("UNAUTHORIZED", "Authentication required", 401);

  const [invite] = await db
    .select()
    .from(orgInvites)
    .where(
      and(
        eq(orgInvites.token, token),
        isNull(orgInvites.acceptedAt),
        gt(orgInvites.expiresAt, new Date()),
      ),
    )
    .limit(1);

  if (!invite) return errorResponse("NOT_FOUND", "Invite not found or expired", 404);

  // Add user to org
  await db.insert(orgMembers).values({
    orgId: invite.orgId,
    userId: user.id,
    role: invite.role,
  });

  // Mark invite as accepted
  await db
    .update(orgInvites)
    .set({ acceptedAt: new Date() })
    .where(eq(orgInvites.id, invite.id));

  return NextResponse.json({ success: true, orgId: invite.orgId });
}
