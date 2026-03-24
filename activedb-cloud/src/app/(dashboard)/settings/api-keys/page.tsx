"use client";

import { useState } from "react";
import useSWR, { mutate } from "swr";
import { Trash2, Copy, Plus } from "lucide-react";
import { Breadcrumb } from "@/components/layout/breadcrumb";

const fetcher = (url: string) => fetch(url).then((r) => r.json());

export default function ApiKeysPage() {
  const { data } = useSWR("/api/v1/api-keys", fetcher);
  const [newKeyName, setNewKeyName] = useState("");
  const [createdKey, setCreatedKey] = useState<string | null>(null);
  const [creating, setCreating] = useState(false);

  const handleCreate = async () => {
    if (!newKeyName) return;
    setCreating(true);
    const res = await fetch("/api/v1/api-keys", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ name: newKeyName }),
    });
    const result = await res.json();
    setCreatedKey(result.key);
    setNewKeyName("");
    setCreating(false);
    mutate("/api/v1/api-keys");
  };

  const handleDelete = async (keyId: string) => {
    await fetch(`/api/v1/api-keys/${keyId}`, { method: "DELETE" });
    mutate("/api/v1/api-keys");
  };

  return (
    <div className="max-w-2xl">
      <Breadcrumb items={[{ label: "Settings", href: "/settings" }, { label: "API Keys" }]} />
      <h1 className="text-2xl font-bold mt-4 mb-6">API Keys</h1>

      {/* Create */}
      <div className="flex gap-2 mb-6">
        <input
          type="text"
          value={newKeyName}
          onChange={(e) => setNewKeyName(e.target.value)}
          placeholder="Key name (e.g. production)"
          className="flex-1 rounded-md border bg-background px-3 py-2 text-sm"
        />
        <button
          onClick={handleCreate}
          disabled={creating || !newKeyName}
          className="flex items-center gap-1.5 rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground hover:bg-primary/90 disabled:opacity-50"
        >
          <Plus className="h-4 w-4" />
          Create
        </button>
      </div>

      {/* Show newly created key */}
      {createdKey && (
        <div className="mb-6 rounded-lg border border-green-200 bg-green-50 p-4">
          <p className="text-sm font-medium text-green-800 mb-2">
            Key created. Copy it now — it won't be shown again.
          </p>
          <div className="flex items-center gap-2">
            <code className="flex-1 rounded bg-white p-2 text-xs">{createdKey}</code>
            <button
              onClick={() => navigator.clipboard.writeText(createdKey)}
              className="rounded p-1.5 hover:bg-green-100"
            >
              <Copy className="h-4 w-4" />
            </button>
          </div>
        </div>
      )}

      {/* Key List */}
      <div className="space-y-2">
        {(data?.apiKeys ?? []).map((key: { id: string; name: string; keyPrefix: string; lastUsed: string | null; createdAt: string }) => (
          <div key={key.id} className="flex items-center justify-between rounded-lg border p-3">
            <div>
              <div className="text-sm font-medium">{key.name}</div>
              <div className="text-xs text-muted-foreground">
                {key.keyPrefix}... · Created {new Date(key.createdAt).toLocaleDateString()}
                {key.lastUsed && ` · Last used ${new Date(key.lastUsed).toLocaleDateString()}`}
              </div>
            </div>
            <button
              onClick={() => handleDelete(key.id)}
              className="rounded p-1.5 text-muted-foreground hover:bg-red-50 hover:text-red-600"
            >
              <Trash2 className="h-4 w-4" />
            </button>
          </div>
        ))}
      </div>
    </div>
  );
}
