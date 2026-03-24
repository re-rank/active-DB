"use client";

import Link from "next/link";
import { Database, MapPin } from "lucide-react";
import { InstanceStatus } from "./instance-status";

interface InstanceCardProps {
  id: string;
  name: string;
  status: string;
  region: string;
  tier: string;
}

export function InstanceCard({ id, name, status, region, tier }: InstanceCardProps) {
  return (
    <Link
      href={`/dashboard/instances/${id}`}
      className="block rounded-lg border p-4 transition-colors hover:bg-muted/50"
    >
      <div className="flex items-start justify-between">
        <div className="flex items-center gap-3">
          <div className="flex h-10 w-10 items-center justify-center rounded-md bg-primary/10">
            <Database className="h-5 w-5 text-primary" />
          </div>
          <div>
            <h3 className="font-medium">{name}</h3>
            <div className="flex items-center gap-2 text-xs text-muted-foreground">
              <MapPin className="h-3 w-3" />
              {region}
              <span className="rounded bg-muted px-1.5 py-0.5 capitalize">{tier}</span>
            </div>
          </div>
        </div>
        <InstanceStatus status={status} />
      </div>
    </Link>
  );
}
