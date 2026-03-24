import { NextResponse } from "next/server";
import { headers } from "next/headers";
import { stripe } from "@/lib/stripe";
import { db } from "@/lib/db";
import { subscriptions, instances, users } from "@/db/schema";
import { eq } from "drizzle-orm";
import type Stripe from "stripe";

export async function POST(request: Request) {
  const body = await request.text();
  const headersList = await headers();
  const signature = headersList.get("stripe-signature");

  if (!signature) {
    return NextResponse.json({ error: "Missing signature" }, { status: 400 });
  }

  let event: Stripe.Event;
  try {
    event = stripe.webhooks.constructEvent(
      body,
      signature,
      process.env.STRIPE_WEBHOOK_SECRET!,
    );
  } catch {
    return NextResponse.json({ error: "Invalid signature" }, { status: 400 });
  }

  switch (event.type) {
    case "checkout.session.completed": {
      const session = event.data.object as Stripe.Checkout.Session;
      if (session.mode === "subscription" && session.subscription) {
        const sub = await stripe.subscriptions.retrieve(session.subscription as string);
        const userId = session.metadata?.userId;
        if (!userId) break;

        const plan = sub.items.data[0]?.price?.lookup_key ?? "pro";

        await db.insert(subscriptions).values({
          ownerType: "user",
          ownerId: userId,
          stripeSubscriptionId: sub.id,
          stripePriceId: sub.items.data[0]?.price?.id ?? "",
          plan,
          status: "active",
          currentPeriodStart: new Date(sub.current_period_start * 1000),
          currentPeriodEnd: new Date(sub.current_period_end * 1000),
        });
      }
      break;
    }

    case "invoice.paid": {
      const invoice = event.data.object as Stripe.Invoice;
      if (invoice.subscription) {
        // Reactivate suspended instances
        const [sub] = await db
          .select()
          .from(subscriptions)
          .where(eq(subscriptions.stripeSubscriptionId, invoice.subscription as string))
          .limit(1);

        if (sub) {
          await db
            .update(instances)
            .set({ status: "running", updatedAt: new Date() })
            .where(eq(instances.ownerId, sub.ownerId));
        }
      }
      break;
    }

    case "invoice.payment_failed": {
      const invoice = event.data.object as Stripe.Invoice;
      if (invoice.subscription) {
        const [sub] = await db
          .select()
          .from(subscriptions)
          .where(eq(subscriptions.stripeSubscriptionId, invoice.subscription as string))
          .limit(1);

        if (sub) {
          await db
            .update(subscriptions)
            .set({ status: "past_due", updatedAt: new Date() })
            .where(eq(subscriptions.id, sub.id));

          // Suspend instances after grace period (3 days handled by Stripe retry)
          await db
            .update(instances)
            .set({ status: "suspended", updatedAt: new Date() })
            .where(eq(instances.ownerId, sub.ownerId));
        }
      }
      break;
    }

    case "customer.subscription.deleted": {
      const sub = event.data.object as Stripe.Subscription;
      await db
        .update(subscriptions)
        .set({ status: "canceled", cancelAt: new Date(), updatedAt: new Date() })
        .where(eq(subscriptions.stripeSubscriptionId, sub.id));
      break;
    }

    case "customer.subscription.updated": {
      const sub = event.data.object as Stripe.Subscription;
      const plan = sub.items.data[0]?.price?.lookup_key ?? "pro";

      await db
        .update(subscriptions)
        .set({
          plan,
          status: sub.status === "active" ? "active" : sub.status as string,
          currentPeriodStart: new Date(sub.current_period_start * 1000),
          currentPeriodEnd: new Date(sub.current_period_end * 1000),
          updatedAt: new Date(),
        })
        .where(eq(subscriptions.stripeSubscriptionId, sub.id));
      break;
    }
  }

  return NextResponse.json({ received: true });
}
