"use client";

const statusConfig: Record<string, { color: string; label: string; animate?: boolean }> = {
  provisioning: { color: "bg-yellow-500", label: "Provisioning", animate: true },
  running: { color: "bg-green-500", label: "Running" },
  stopped: { color: "bg-gray-400", label: "Stopped" },
  suspended: { color: "bg-red-500", label: "Suspended" },
  terminating: { color: "bg-orange-500", label: "Terminating", animate: true },
  terminated: { color: "bg-gray-600", label: "Terminated" },
  error: { color: "bg-red-600", label: "Error" },
};

export function InstanceStatus({ status }: { status: string }) {
  const config = statusConfig[status] ?? { color: "bg-gray-400", label: status };

  return (
    <span className="inline-flex items-center gap-1.5 text-xs font-medium">
      <span
        className={`h-2 w-2 rounded-full ${config.color} ${config.animate ? "animate-pulse" : ""}`}
      />
      {config.label}
    </span>
  );
}
