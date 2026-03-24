"use client";

import { Check } from "lucide-react";

const plans = [
  {
    id: "free", name: "Free", price: "$0", period: "/mo",
    features: ["1 instance", "500MB storage", "0.5 vCPU / 512MB", "10K API calls/day", "Daily backup"],
  },
  {
    id: "pro", name: "Pro", price: "$29", period: "/mo", popular: true,
    features: ["5 instances", "10GB/instance", "2 vCPU / 4GB", "1M API calls/day", "Hourly backup", "5 team members"],
  },
  {
    id: "enterprise", name: "Enterprise", price: "Custom", period: "",
    features: ["Unlimited instances", "Custom storage", "Dedicated resources", "Unlimited API calls", "Minute-level backup", "Unlimited team"],
  },
];

interface PlanSelectorProps {
  currentPlan?: string;
  onSelect: (planId: string) => void;
}

export function PlanSelector({ currentPlan, onSelect }: PlanSelectorProps) {
  return (
    <div className="grid gap-4 md:grid-cols-3">
      {plans.map((plan) => (
        <div
          key={plan.id}
          className={`relative rounded-lg border p-6 ${
            plan.popular ? "border-primary shadow-sm" : ""
          }`}
        >
          {plan.popular && (
            <span className="absolute -top-3 left-4 rounded-full bg-primary px-3 py-0.5 text-xs font-medium text-primary-foreground">
              Popular
            </span>
          )}
          <h3 className="text-lg font-semibold">{plan.name}</h3>
          <div className="mt-2">
            <span className="text-3xl font-bold">{plan.price}</span>
            <span className="text-muted-foreground">{plan.period}</span>
          </div>
          <ul className="mt-4 space-y-2">
            {plan.features.map((f) => (
              <li key={f} className="flex items-center gap-2 text-sm">
                <Check className="h-4 w-4 text-green-500" />
                {f}
              </li>
            ))}
          </ul>
          <button
            onClick={() => onSelect(plan.id)}
            disabled={currentPlan === plan.id}
            className={`mt-6 w-full rounded-md px-4 py-2 text-sm font-medium ${
              currentPlan === plan.id
                ? "border bg-muted text-muted-foreground"
                : "bg-primary text-primary-foreground hover:bg-primary/90"
            }`}
          >
            {currentPlan === plan.id ? "Current Plan" : "Select"}
          </button>
        </div>
      ))}
    </div>
  );
}
