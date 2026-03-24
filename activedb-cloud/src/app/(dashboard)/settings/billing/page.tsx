"use client";

import useSWR from "swr";
import { Breadcrumb } from "@/components/layout/breadcrumb";
import { PlanSelector } from "@/components/billing/plan-selector";
import { UsageMeter } from "@/components/billing/usage-meter";
import { ExternalLink } from "lucide-react";

const fetcher = (url: string) => fetch(url).then((r) => r.json());

export default function BillingPage() {
  const { data: subData } = useSWR("/api/v1/billing/subscription", fetcher);
  const { data: usageData } = useSWR("/api/v1/billing/usage", fetcher);
  const { data: invoiceData } = useSWR("/api/v1/billing/invoices", fetcher);

  const currentPlan = subData?.subscription?.plan ?? "free";
  const usage = usageData?.usage ?? { apiCalls: 0, storageBytes: 0, networkBytes: 0 };

  const planLimits: Record<string, { apiCalls: number; storage: number }> = {
    free: { apiCalls: 10000, storage: 500 * 1024 * 1024 },
    pro: { apiCalls: 1000000, storage: 10 * 1024 * 1024 * 1024 },
    enterprise: { apiCalls: Infinity, storage: Infinity },
  };
  const limits = planLimits[currentPlan] ?? planLimits.free;

  const handleSelectPlan = async (planId: string) => {
    if (planId === "enterprise") {
      window.open("mailto:sales@activedb.dev", "_blank");
      return;
    }
    const res = await fetch("/api/v1/billing/subscription", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ priceId: `price_${planId}` }),
    });
    const { url } = await res.json();
    if (url) window.location.href = url;
  };

  const handleManageBilling = async () => {
    const res = await fetch("/api/v1/billing/portal");
    const { url } = await res.json();
    if (url) window.location.href = url;
  };

  return (
    <div className="max-w-3xl">
      <Breadcrumb items={[{ label: "Settings", href: "/settings" }, { label: "Billing" }]} />
      <h1 className="text-2xl font-bold mt-4 mb-6">Billing</h1>

      {/* Usage */}
      <section className="mb-8 space-y-4">
        <h2 className="text-lg font-semibold">Usage This Month</h2>
        <div className="rounded-lg border p-4 space-y-4">
          <UsageMeter label="API Calls" current={usage.apiCalls} limit={limits.apiCalls} unit="calls" />
          <UsageMeter label="Storage" current={usage.storageBytes} limit={limits.storage} unit="bytes" />
        </div>
      </section>

      {/* Plan */}
      <section className="mb-8 space-y-4">
        <h2 className="text-lg font-semibold">Plan</h2>
        <PlanSelector currentPlan={currentPlan} onSelect={handleSelectPlan} />
      </section>

      {/* Manage */}
      {currentPlan !== "free" && (
        <section className="mb-8">
          <button
            onClick={handleManageBilling}
            className="flex items-center gap-1.5 rounded-md border px-4 py-2 text-sm hover:bg-muted"
          >
            <ExternalLink className="h-4 w-4" />
            Manage Payment Method
          </button>
        </section>
      )}

      {/* Invoices */}
      <section className="space-y-4">
        <h2 className="text-lg font-semibold">Invoices</h2>
        <div className="rounded-lg border">
          {(invoiceData?.invoices ?? []).length === 0 ? (
            <div className="p-4 text-sm text-muted-foreground">No invoices yet.</div>
          ) : (
            <table className="w-full text-sm">
              <thead className="border-b bg-muted/30">
                <tr>
                  <th className="px-4 py-2 text-left font-medium">Date</th>
                  <th className="px-4 py-2 text-left font-medium">Amount</th>
                  <th className="px-4 py-2 text-left font-medium">Status</th>
                  <th className="px-4 py-2 text-right font-medium">Invoice</th>
                </tr>
              </thead>
              <tbody>
                {(invoiceData?.invoices ?? []).map((inv: { id: string; amount: number; currency: string; status: string; invoiceUrl: string; created: number }) => (
                  <tr key={inv.id} className="border-b last:border-0">
                    <td className="px-4 py-2">{new Date(inv.created * 1000).toLocaleDateString()}</td>
                    <td className="px-4 py-2">${(inv.amount / 100).toFixed(2)} {inv.currency.toUpperCase()}</td>
                    <td className="px-4 py-2 capitalize">{inv.status}</td>
                    <td className="px-4 py-2 text-right">
                      <a href={inv.invoiceUrl} target="_blank" rel="noopener" className="text-primary hover:underline">
                        View
                      </a>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          )}
        </div>
      </section>
    </div>
  );
}
