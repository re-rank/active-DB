import { NextResponse } from "next/server";
import { randomBytes, createHash } from "crypto";
import { db } from "@/lib/db";
import { apiKeys } from "@/db/schema";
import { eq, and } from "drizzle-orm";
import { getAuthenticatedUser, errorResponse } from "@/lib/api-helpers";

type Params = { params: Promise<{ keyId: string }> };

// POST /api/v1/api-keys/:keyId/rotate
export async function POST(request: Request, { params }: Params) {
  const { keyId } = await params;
  const user = await getAuthenticatedUser();
  if (!user) return errorResponse("UNAUTHORIZED", "Authentication required", 401);

  const rawKey = `ak_live_${randomBytes(24).toString("base64url")}`;
  const keyHash = createHash("sha256").update(rawKey).digest("hex");
  const keyPrefix = rawKey.slice(0, 12);

  const [updated] = await db
    .update(apiKeys)
    .set({ keyHash, keyPrefix })
    .where(and(eq(apiKeys.id, keyId), eq(apiKeys.userId, user.id)))
    .returning();

  if (!updated) return errorResponse("NOT_FOUND", "API key not found", 404);

  return NextResponse.json({ id: updated.id, name: updated.name, key: rawKey });
}
