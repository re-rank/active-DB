import { NextResponse } from "next/server";
import { randomBytes, createHash } from "crypto";
import { db } from "@/lib/db";
import { apiKeys } from "@/db/schema";
import { eq } from "drizzle-orm";
import { getAuthenticatedUser, errorResponse } from "@/lib/api-helpers";

// GET /api/v1/api-keys — 목록
export async function GET() {
  const user = await getAuthenticatedUser();
  if (!user) return errorResponse("UNAUTHORIZED", "Authentication required", 401);

  const keys = await db
    .select({
      id: apiKeys.id,
      name: apiKeys.name,
      keyPrefix: apiKeys.keyPrefix,
      lastUsed: apiKeys.lastUsed,
      expiresAt: apiKeys.expiresAt,
      createdAt: apiKeys.createdAt,
    })
    .from(apiKeys)
    .where(eq(apiKeys.userId, user.id));

  return NextResponse.json({ apiKeys: keys });
}

// POST /api/v1/api-keys — 생성
export async function POST(request: Request) {
  const user = await getAuthenticatedUser();
  if (!user) return errorResponse("UNAUTHORIZED", "Authentication required", 401);

  const { name } = await request.json();
  if (!name) return errorResponse("INVALID_REQUEST", "Key name is required", 400);

  const rawKey = `ak_live_${randomBytes(24).toString("base64url")}`;
  const keyHash = createHash("sha256").update(rawKey).digest("hex");
  const keyPrefix = rawKey.slice(0, 12);

  const [key] = await db
    .insert(apiKeys)
    .values({ userId: user.id, name, keyHash, keyPrefix })
    .returning();

  return NextResponse.json({ id: key.id, name: key.name, key: rawKey }, { status: 201 });
}
