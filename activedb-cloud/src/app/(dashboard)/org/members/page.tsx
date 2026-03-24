"use client";

import { useSearchParams } from "next/navigation";
import useSWR, { mutate } from "swr";
import { useState } from "react";
import { Breadcrumb } from "@/components/layout/breadcrumb";
import { Trash2, Mail } from "lucide-react";

const fetcher = (url: string) => fetch(url).then((r) => r.json());

export default function MembersPage() {
  const searchParams = useSearchParams();
  const orgId = searchParams.get("orgId") ?? "";
  const { data } = useSWR(orgId ? `/api/v1/organizations/${orgId}/members` : null, fetcher);
  const [inviteEmail, setInviteEmail] = useState("");

  const handleInvite = async () => {
    if (!inviteEmail || !orgId) return;
    await fetch(`/api/v1/organizations/${orgId}/invites`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ email: inviteEmail }),
    });
    setInviteEmail("");
  };

  const handleRemove = async (userId: string) => {
    await fetch(`/api/v1/organizations/${orgId}/members/${userId}`, { method: "DELETE" });
    mutate(`/api/v1/organizations/${orgId}/members`);
  };

  const handleRoleChange = async (userId: string, role: string) => {
    await fetch(`/api/v1/organizations/${orgId}/members/${userId}`, {
      method: "PATCH",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ role }),
    });
    mutate(`/api/v1/organizations/${orgId}/members`);
  };

  const members = data?.members ?? [];

  return (
    <div className="max-w-2xl">
      <Breadcrumb items={[{ label: "Organization", href: "/org" }, { label: "Members" }]} />
      <h1 className="text-2xl font-bold mt-4 mb-6">Members</h1>

      {/* Invite */}
      <div className="flex gap-2 mb-6">
        <input
          type="email"
          value={inviteEmail}
          onChange={(e) => setInviteEmail(e.target.value)}
          placeholder="Email address"
          className="flex-1 rounded-md border bg-background px-3 py-2 text-sm"
        />
        <button
          onClick={handleInvite}
          disabled={!inviteEmail}
          className="flex items-center gap-1.5 rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground hover:bg-primary/90 disabled:opacity-50"
        >
          <Mail className="h-4 w-4" />
          Invite
        </button>
      </div>

      {/* Member List */}
      <div className="space-y-2">
        {members.map((m: { userId: string; name: string; email: string; role: string; avatarUrl: string }) => (
          <div key={m.userId} className="flex items-center justify-between rounded-lg border p-3">
            <div className="flex items-center gap-3">
              {m.avatarUrl && (
                <img src={m.avatarUrl} alt="" className="h-8 w-8 rounded-full" />
              )}
              <div>
                <div className="text-sm font-medium">{m.name ?? m.email}</div>
                <div className="text-xs text-muted-foreground">{m.email}</div>
              </div>
            </div>
            <div className="flex items-center gap-2">
              <select
                value={m.role}
                onChange={(e) => handleRoleChange(m.userId, e.target.value)}
                disabled={m.role === "owner"}
                className="rounded-md border bg-background px-2 py-1 text-xs"
              >
                <option value="viewer">Viewer</option>
                <option value="developer">Developer</option>
                <option value="admin">Admin</option>
                {m.role === "owner" && <option value="owner">Owner</option>}
              </select>
              {m.role !== "owner" && (
                <button
                  onClick={() => handleRemove(m.userId)}
                  className="rounded p-1.5 text-muted-foreground hover:bg-red-50 hover:text-red-600"
                >
                  <Trash2 className="h-4 w-4" />
                </button>
              )}
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
