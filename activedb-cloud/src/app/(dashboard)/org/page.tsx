"use client";

import { useState } from "react";
import { useRouter } from "next/navigation";
import useSWR from "swr";
import Link from "next/link";
import { Users, Plus } from "lucide-react";

const fetcher = (url: string) => fetch(url).then((r) => r.json());

export default function OrgPage() {
  const router = useRouter();
  const { data } = useSWR("/api/v1/organizations", fetcher);
  const [showCreate, setShowCreate] = useState(false);
  const [name, setName] = useState("");
  const [slug, setSlug] = useState("");

  const handleCreate = async (e: React.FormEvent) => {
    e.preventDefault();
    const res = await fetch("/api/v1/organizations", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ name, slug }),
    });
    if (res.ok) {
      const { organization } = await res.json();
      router.push(`/org/members?orgId=${organization.id}`);
    }
  };

  const orgs = data?.organizations ?? [];

  return (
    <div className="max-w-2xl">
      <div className="flex items-center justify-between mb-6">
        <h1 className="text-2xl font-bold">Organizations</h1>
        <button
          onClick={() => setShowCreate(true)}
          className="flex items-center gap-1.5 rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground hover:bg-primary/90"
        >
          <Plus className="h-4 w-4" />
          New Organization
        </button>
      </div>

      {showCreate && (
        <form onSubmit={handleCreate} className="mb-6 rounded-lg border p-4 space-y-3">
          <input
            type="text"
            value={name}
            onChange={(e) => { setName(e.target.value); setSlug(e.target.value.toLowerCase().replace(/\s+/g, "-")); }}
            placeholder="Organization name"
            className="w-full rounded-md border bg-background px-3 py-2 text-sm"
          />
          <input
            type="text"
            value={slug}
            onChange={(e) => setSlug(e.target.value)}
            placeholder="slug"
            className="w-full rounded-md border bg-background px-3 py-2 text-sm"
          />
          <div className="flex gap-2">
            <button type="submit" className="rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground">
              Create
            </button>
            <button type="button" onClick={() => setShowCreate(false)} className="rounded-md border px-4 py-2 text-sm">
              Cancel
            </button>
          </div>
        </form>
      )}

      <div className="space-y-2">
        {orgs.map((org: { id: string; name: string; slug: string; role: string }) => (
          <Link
            key={org.id}
            href={`/org/members?orgId=${org.id}`}
            className="flex items-center justify-between rounded-lg border p-4 hover:bg-muted/50"
          >
            <div className="flex items-center gap-3">
              <Users className="h-5 w-5 text-muted-foreground" />
              <div>
                <div className="font-medium text-sm">{org.name}</div>
                <div className="text-xs text-muted-foreground">@{org.slug}</div>
              </div>
            </div>
            <span className="rounded bg-muted px-2 py-0.5 text-xs capitalize">{org.role}</span>
          </Link>
        ))}
      </div>
    </div>
  );
}
