import { NextResponse } from "next/server";
import { db } from "@/lib/db";
import { apiKeys } from "@/db/schema";
import { eq, and } from "drizzle-orm";
import { getAuthenticatedUser, errorResponse } from "@/lib/api-helpers";

type Params = { params: Promise<{ keyId: string }> };

// DELETE /api/v1/api-keys/:keyId
export async function DELETE(request: Request, { params }: Params) {
  const { keyId } = await params;
  const user = await getAuthenticatedUser();
  if (!user) return errorResponse("UNAUTHORIZED", "Authentication required", 401);

  const [deleted] = await db
    .delete(apiKeys)
    .where(and(eq(apiKeys.id, keyId), eq(apiKeys.userId, user.id)))
    .returning();

  if (!deleted) return errorResponse("NOT_FOUND", "API key not found", 404);

  return NextResponse.json({ success: true });
}
