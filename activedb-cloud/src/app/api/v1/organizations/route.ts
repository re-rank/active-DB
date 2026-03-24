import { NextResponse } from "next/server";
import { db } from "@/lib/db";
import { organizations, orgMembers } from "@/db/schema";
import { eq } from "drizzle-orm";
import { getAuthenticatedUser, errorResponse } from "@/lib/api-helpers";

// GET /api/v1/organizations — 목록
export async function GET() {
  const user = await getAuthenticatedUser();
  if (!user) return errorResponse("UNAUTHORIZED", "Authentication required", 401);

  const memberships = await db
    .select({ orgId: orgMembers.orgId, role: orgMembers.role })
    .from(orgMembers)
    .where(eq(orgMembers.userId, user.id));

  if (memberships.length === 0) {
    return NextResponse.json({ organizations: [] });
  }

  const orgIds = memberships.map((m) => m.orgId);
  const orgs = await db.select().from(organizations);
  const filtered = orgs.filter((o) => orgIds.includes(o.id));

  const result = filtered.map((org) => ({
    ...org,
    role: memberships.find((m) => m.orgId === org.id)?.role,
  }));

  return NextResponse.json({ organizations: result });
}

// POST /api/v1/organizations — 생성
export async function POST(request: Request) {
  const user = await getAuthenticatedUser();
  if (!user) return errorResponse("UNAUTHORIZED", "Authentication required", 401);

  const { name, slug } = await request.json();
  if (!name || !slug) return errorResponse("INVALID_REQUEST", "name and slug are required", 400);

  const [org] = await db
    .insert(organizations)
    .values({ name, slug, ownerId: user.id })
    .returning();

  // Add owner as member
  await db.insert(orgMembers).values({ orgId: org.id, userId: user.id, role: "owner" });

  return NextResponse.json({ organization: org }, { status: 201 });
}
