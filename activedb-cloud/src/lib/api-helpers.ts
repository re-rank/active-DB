import { getServerSession } from "next-auth";
import { NextResponse } from "next/server";
import { authOptions } from "./auth";
import { db } from "./db";
import { apiKeys } from "@/db/schema";
import { eq } from "drizzle-orm";
import { createHash } from "crypto";

export function errorResponse(
  code: string,
  message: string,
  status: number,
) {
  return NextResponse.json({ error: { code, message, status } }, { status });
}

export async function getAuthenticatedUser() {
  const session = await getServerSession(authOptions);
  if (!session?.user?.id) return null;
  return session.user;
}

export async function getAuthenticatedUserOrApiKey(
  request: Request,
) {
  // 1. Check session
  const session = await getServerSession(authOptions);
  if (session?.user?.id) return { userId: session.user.id, via: "session" as const };

  // 2. Check API key
  const apiKey = request.headers.get("x-api-key");
  if (apiKey) {
    const keyHash = createHash("sha256").update(apiKey).digest("hex");
    const [key] = await db
      .select()
      .from(apiKeys)
      .where(eq(apiKeys.keyHash, keyHash))
      .limit(1);

    if (key) {
      // Update last used
      await db
        .update(apiKeys)
        .set({ lastUsed: new Date() })
        .where(eq(apiKeys.id, key.id));
      return { userId: key.userId, via: "api_key" as const };
    }
  }

  return null;
}
