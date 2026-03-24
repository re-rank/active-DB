import { NextResponse } from "next/server";
import { db } from "@/lib/db";
import { subscriptions, users } from "@/db/schema";
import { eq } from "drizzle-orm";
import { stripe } from "@/lib/stripe";
import { getAuthenticatedUser, errorResponse } from "@/lib/api-helpers";

// GET /api/v1/billing/subscription
export async function GET() {
  const user = await getAuthenticatedUser();
  if (!user) return errorResponse("UNAUTHORIZED", "Authentication required", 401);

  const [sub] = await db
    .select()
    .from(subscriptions)
    .where(eq(subscriptions.ownerId, user.id))
    .limit(1);

  return NextResponse.json({ subscription: sub ?? null });
}

// POST /api/v1/billing/subscribe
export async function POST(request: Request) {
  const user = await getAuthenticatedUser();
  if (!user) return errorResponse("UNAUTHORIZED", "Authentication required", 401);

  const { priceId } = await request.json();
  if (!priceId) return errorResponse("INVALID_REQUEST", "priceId is required", 400);

  // Get or create Stripe customer
  const [dbUser] = await db.select().from(users).where(eq(users.id, user.id)).limit(1);
  let customerId = dbUser?.stripeCustomerId;

  if (!customerId) {
    const customer = await stripe.customers.create({
      email: dbUser?.email ?? undefined,
      metadata: { userId: user.id },
    });
    customerId = customer.id;
    await db.update(users).set({ stripeCustomerId: customerId }).where(eq(users.id, user.id));
  }

  const session = await stripe.checkout.sessions.create({
    customer: customerId,
    mode: "subscription",
    line_items: [{ price: priceId, quantity: 1 }],
    success_url: `${process.env.NEXT_PUBLIC_APP_URL}/settings/billing?success=true`,
    cancel_url: `${process.env.NEXT_PUBLIC_APP_URL}/settings/billing?canceled=true`,
    metadata: { userId: user.id },
  });

  return NextResponse.json({ url: session.url });
}
