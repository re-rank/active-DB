"use client";

import { useState } from "react";
import { useRouter } from "next/navigation";

const plans = [
  { id: "free", name: "Free", cpu: "0.5 vCPU", memory: "512MB", storage: "500MB", price: "$0/mo" },
  { id: "pro", name: "Pro", cpu: "2 vCPU", memory: "4GB", storage: "10GB", price: "$29/mo" },
  { id: "enterprise", name: "Enterprise", cpu: "Dedicated", memory: "Custom", storage: "Custom", price: "Custom" },
];

const regions = [
  { id: "us-east-1", label: "US East (Virginia)" },
  { id: "us-west-2", label: "US West (Oregon)" },
  { id: "eu-west-1", label: "EU (Ireland)" },
  { id: "ap-northeast-1", label: "Asia Pacific (Tokyo)" },
];

export function CreateInstanceForm() {
  const router = useRouter();
  const [name, setName] = useState("");
  const [region, setRegion] = useState("us-east-1");
  const [plan, setPlan] = useState("free");
  const [loading, setLoading] = useState(false);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setLoading(true);

    const res = await fetch("/api/v1/instances", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ name, region, plan }),
    });

    if (res.ok) {
      const { instance } = await res.json();
      router.push(`/dashboard/instances/${instance.id}`);
    }
    setLoading(false);
  };

  return (
    <form onSubmit={handleSubmit} className="max-w-2xl space-y-6">
      {/* Step 1: Basic Info */}
      <div className="space-y-4">
        <h2 className="text-lg font-semibold">Basic Info</h2>
        <div>
          <label className="block text-sm font-medium mb-1">Instance Name</label>
          <input
            type="text"
            value={name}
            onChange={(e) => setName(e.target.value)}
            placeholder="my-graph-db"
            required
            className="w-full rounded-md border bg-background px-3 py-2 text-sm"
          />
        </div>
        <div>
          <label className="block text-sm font-medium mb-1">Region</label>
          <select
            value={region}
            onChange={(e) => setRegion(e.target.value)}
            className="w-full rounded-md border bg-background px-3 py-2 text-sm"
          >
            {regions.map((r) => (
              <option key={r.id} value={r.id}>{r.label}</option>
            ))}
          </select>
        </div>
      </div>

      {/* Step 2: Plan Selection */}
      <div className="space-y-4">
        <h2 className="text-lg font-semibold">Select Plan</h2>
        <div className="grid grid-cols-3 gap-4">
          {plans.map((p) => (
            <button
              key={p.id}
              type="button"
              onClick={() => setPlan(p.id)}
              className={`rounded-lg border p-4 text-left transition-colors ${
                plan === p.id ? "border-primary bg-primary/5" : "hover:bg-muted/50"
              }`}
            >
              <div className="font-medium">{p.name}</div>
              <div className="mt-1 text-xs text-muted-foreground space-y-0.5">
                <div>{p.cpu}</div>
                <div>{p.memory} RAM</div>
                <div>{p.storage}</div>
              </div>
              <div className="mt-2 font-semibold text-sm">{p.price}</div>
            </button>
          ))}
        </div>
      </div>

      <button
        type="submit"
        disabled={loading || !name}
        className="rounded-md bg-primary px-6 py-2.5 text-sm font-medium text-primary-foreground hover:bg-primary/90 disabled:opacity-50"
      >
        {loading ? "Creating..." : "Create Instance"}
      </button>
    </form>
  );
}
