import { NextResponse } from "next/server";
import { db } from "@/lib/db";
import { users } from "@/db/schema";
import { eq } from "drizzle-orm";
import { stripe } from "@/lib/stripe";
import { getAuthenticatedUser, errorResponse } from "@/lib/api-helpers";

// GET /api/v1/billing/invoices
export async function GET() {
  const user = await getAuthenticatedUser();
  if (!user) return errorResponse("UNAUTHORIZED", "Authentication required", 401);

  const [dbUser] = await db.select().from(users).where(eq(users.id, user.id)).limit(1);
  if (!dbUser?.stripeCustomerId) {
    return NextResponse.json({ invoices: [] });
  }

  const invoices = await stripe.invoices.list({
    customer: dbUser.stripeCustomerId,
    limit: 20,
  });

  const items = invoices.data.map((inv) => ({
    id: inv.id,
    amount: inv.amount_due,
    currency: inv.currency,
    status: inv.status,
    invoiceUrl: inv.hosted_invoice_url,
    pdfUrl: inv.invoice_pdf,
    created: inv.created,
    periodStart: inv.period_start,
    periodEnd: inv.period_end,
  }));

  return NextResponse.json({ invoices: items });
}
