import { getServerSession } from "next-auth";
import { authOptions } from "@/lib/auth";
import { redirect } from "next/navigation";
import Link from "next/link";
import { Key, CreditCard } from "lucide-react";

export default async function SettingsPage() {
  const session = await getServerSession(authOptions);
  if (!session?.user) redirect("/login");

  return (
    <div className="max-w-2xl">
      <h1 className="text-2xl font-bold mb-6">Settings</h1>

      {/* Account Info */}
      <section className="space-y-4 mb-8">
        <h2 className="text-lg font-semibold">Account</h2>
        <div className="rounded-lg border p-4 space-y-3">
          <div>
            <label className="block text-xs text-muted-foreground">Name</label>
            <span className="text-sm">{session.user.name}</span>
          </div>
          <div>
            <label className="block text-xs text-muted-foreground">Email</label>
            <span className="text-sm">{session.user.email}</span>
          </div>
        </div>
      </section>

      {/* Quick Links */}
      <section className="space-y-3">
        <Link
          href="/settings/api-keys"
          className="flex items-center gap-3 rounded-lg border p-4 hover:bg-muted/50"
        >
          <Key className="h-5 w-5 text-muted-foreground" />
          <div>
            <div className="font-medium text-sm">API Keys</div>
            <div className="text-xs text-muted-foreground">Manage API keys for CLI and SDK access</div>
          </div>
        </Link>
        <Link
          href="/settings/billing"
          className="flex items-center gap-3 rounded-lg border p-4 hover:bg-muted/50"
        >
          <CreditCard className="h-5 w-5 text-muted-foreground" />
          <div>
            <div className="font-medium text-sm">Billing</div>
            <div className="text-xs text-muted-foreground">Plan, usage, and invoices</div>
          </div>
        </Link>
      </section>
    </div>
  );
}
