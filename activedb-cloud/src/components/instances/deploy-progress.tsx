"use client";

import { useSSE } from "@/hooks/use-sse";

const stages = ["pending", "building", "deploying", "completed"] as const;

interface DeployEvent {
  stage: string;
  deploymentId: string;
}

export function DeployProgress({ instanceId }: { instanceId: string }) {
  const { data, status } = useSSE<DeployEvent>(
    `/api/v1/instances/${instanceId}/deploy/status`,
  );

  const currentIndex = stages.indexOf((data?.stage ?? "pending") as typeof stages[number]);

  return (
    <div className="space-y-3">
      <div className="flex items-center gap-2 text-sm">
        {status === "connecting" && <span className="text-muted-foreground">Connecting...</span>}
        {status === "open" && data && (
          <span className="font-medium capitalize">{data.stage}</span>
        )}
      </div>
      <div className="flex gap-1">
        {stages.map((stage, i) => (
          <div
            key={stage}
            className={`h-2 flex-1 rounded-full ${
              i <= currentIndex
                ? i === currentIndex && stage !== "completed"
                  ? "bg-yellow-500 animate-pulse"
                  : "bg-green-500"
                : "bg-muted"
            }`}
          />
        ))}
      </div>
      <div className="flex justify-between text-xs text-muted-foreground">
        {stages.map((s) => (
          <span key={s} className="capitalize">{s}</span>
        ))}
      </div>
    </div>
  );
}
