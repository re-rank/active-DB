"use client";

import { AreaChart, Area, XAxis, YAxis, CartesianGrid, Tooltip, ResponsiveContainer } from "recharts";

interface DataPoint {
  timestamp: string;
  value: number;
}

export function MemoryChart({ data }: { data: DataPoint[] }) {
  return (
    <div className="rounded-lg border p-4">
      <h3 className="text-sm font-medium mb-4">Memory Usage (MB)</h3>
      <ResponsiveContainer width="100%" height={200}>
        <AreaChart data={data}>
          <CartesianGrid strokeDasharray="3 3" className="stroke-muted" />
          <XAxis dataKey="timestamp" className="text-xs" />
          <YAxis className="text-xs" />
          <Tooltip />
          <Area type="monotone" dataKey="value" stroke="hsl(var(--primary))" fill="hsl(var(--primary) / 0.1)" strokeWidth={2} />
        </AreaChart>
      </ResponsiveContainer>
    </div>
  );
}
