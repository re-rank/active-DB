"use client";

import { useParams, useRouter } from "next/navigation";
import { useState } from "react";

export default function InviteAcceptPage() {
  const { token } = useParams<{ token: string }>();
  const router = useRouter();
  const [status, setStatus] = useState<"idle" | "loading" | "error">("idle");

  const handleAccept = async () => {
    setStatus("loading");
    const res = await fetch(`/api/v1/invites/${token}/accept`, { method: "POST" });
    if (res.ok) {
      const { orgId } = await res.json();
      router.push(`/org/members?orgId=${orgId}`);
    } else {
      setStatus("error");
    }
  };

  return (
    <main className="flex min-h-screen items-center justify-center">
      <div className="w-full max-w-sm space-y-6 rounded-lg border p-8 text-center">
        <h1 className="text-2xl font-bold">Organization Invite</h1>
        <p className="text-sm text-muted-foreground">
          You've been invited to join an organization on ActiveDB Cloud.
        </p>
        {status === "error" && (
          <p className="text-sm text-red-600">Invite not found or expired.</p>
        )}
        <button
          onClick={handleAccept}
          disabled={status === "loading"}
          className="w-full rounded-md bg-primary px-4 py-2.5 text-sm font-medium text-primary-foreground hover:bg-primary/90 disabled:opacity-50"
        >
          {status === "loading" ? "Accepting..." : "Accept Invite"}
        </button>
      </div>
    </main>
  );
}
