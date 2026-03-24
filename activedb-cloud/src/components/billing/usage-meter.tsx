"use client";

interface UsageMeterProps {
  label: string;
  current: number;
  limit: number;
  unit: string;
}

export function UsageMeter({ label, current, limit, unit }: UsageMeterProps) {
  const percentage = limit > 0 ? Math.min((current / limit) * 100, 100) : 0;
  const isWarning = percentage > 80;
  const isCritical = percentage > 95;

  return (
    <div className="space-y-2">
      <div className="flex items-center justify-between text-sm">
        <span className="font-medium">{label}</span>
        <span className="text-muted-foreground">
          {current.toLocaleString()} / {limit.toLocaleString()} {unit}
        </span>
      </div>
      <div className="h-2 rounded-full bg-muted">
        <div
          className={`h-full rounded-full transition-all ${
            isCritical ? "bg-red-500" : isWarning ? "bg-yellow-500" : "bg-primary"
          }`}
          style={{ width: `${percentage}%` }}
        />
      </div>
      <div className="text-xs text-muted-foreground text-right">{percentage.toFixed(0)}%</div>
    </div>
  );
}
