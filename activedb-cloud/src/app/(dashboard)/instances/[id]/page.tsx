import { db } from "@/lib/db";
import { instances } from "@/db/schema";
import { eq } from "drizzle-orm";
import { notFound } from "next/navigation";
import { Breadcrumb } from "@/components/layout/breadcrumb";
import { InstanceStatus } from "@/components/instances/instance-status";
import Link from "next/link";
import { Activity, ScrollText, Settings, Terminal, Play, Square, RotateCw } from "lucide-react";

type Params = { params: Promise<{ id: string }> };

export default async function InstanceDetailPage({ params }: Params) {
  const { id } = await params;

  const [instance] = await db
    .select()
    .from(instances)
    .where(eq(instances.id, id))
    .limit(1);

  if (!instance) notFound();

  const tabs = [
    { label: "Metrics", href: `/dashboard/instances/${id}/metrics`, icon: Activity },
    { label: "Logs", href: `/dashboard/instances/${id}/logs`, icon: ScrollText },
    { label: "Query", href: `/dashboard/instances/${id}/query`, icon: Terminal },
    { label: "Settings", href: `/dashboard/instances/${id}/settings`, icon: Settings },
  ];

  return (
    <div>
      <Breadcrumb
        items={[
          { label: "Instances", href: "/dashboard/instances" },
          { label: instance.name },
        ]}
      />

      {/* Header */}
      <div className="mt-4 flex items-start justify-between">
        <div>
          <h1 className="text-2xl font-bold">{instance.name}</h1>
          <div className="mt-1 flex items-center gap-3 text-sm text-muted-foreground">
            <InstanceStatus status={instance.status} />
            <span>{instance.region}</span>
            <span className="capitalize">{instance.tier}</span>
          </div>
        </div>
        <div className="flex gap-2">
          {instance.status === "stopped" && (
            <ActionButton instanceId={id} action="start" icon={<Play className="h-4 w-4" />} label="Start" />
          )}
          {instance.status === "running" && (
            <>
              <ActionButton instanceId={id} action="restart" icon={<RotateCw className="h-4 w-4" />} label="Restart" />
              <ActionButton instanceId={id} action="stop" icon={<Square className="h-4 w-4" />} label="Stop" variant="destructive" />
            </>
          )}
        </div>
      </div>

      {/* Connection Info */}
      {instance.endpoint && (
        <div className="mt-4 rounded-lg border bg-muted/30 p-4">
          <h3 className="text-sm font-medium mb-1">Endpoint</h3>
          <code className="text-sm text-muted-foreground">{instance.endpoint}</code>
        </div>
      )}

      {/* Tabs */}
      <nav className="mt-6 flex gap-1 border-b">
        {tabs.map((tab) => (
          <Link
            key={tab.label}
            href={tab.href}
            className="flex items-center gap-1.5 border-b-2 border-transparent px-4 py-2.5 text-sm text-muted-foreground hover:text-foreground hover:border-foreground/20"
          >
            <tab.icon className="h-4 w-4" />
            {tab.label}
          </Link>
        ))}
      </nav>
    </div>
  );
}

function ActionButton({
  instanceId,
  action,
  icon,
  label,
  variant,
}: {
  instanceId: string;
  action: string;
  icon: React.ReactNode;
  label: string;
  variant?: "destructive";
}) {
  return (
    <form action={`/api/v1/instances/${instanceId}/${action}`} method="POST">
      <button
        type="submit"
        className={`flex items-center gap-1.5 rounded-md px-3 py-1.5 text-sm font-medium ${
          variant === "destructive"
            ? "border border-red-300 text-red-600 hover:bg-red-50"
            : "border hover:bg-muted"
        }`}
      >
        {icon}
        {label}
      </button>
    </form>
  );
}
