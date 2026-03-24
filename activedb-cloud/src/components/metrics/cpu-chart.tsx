"use client";

import { LineChart, Line, XAxis, YAxis, CartesianGrid, Tooltip, ResponsiveContainer } from "recharts";

interface DataPoint {
  timestamp: string;
  value: number;
}

export function CpuChart({ data }: { data: DataPoint[] }) {
  return (
    <div className="rounded-lg border p-4">
      <h3 className="text-sm font-medium mb-4">CPU Usage (%)</h3>
      <ResponsiveContainer width="100%" height={200}>
        <LineChart data={data}>
          <CartesianGrid strokeDasharray="3 3" className="stroke-muted" />
          <XAxis dataKey="timestamp" className="text-xs" />
          <YAxis domain={[0, 100]} className="text-xs" />
          <Tooltip />
          <Line type="monotone" dataKey="value" stroke="hsl(var(--primary))" strokeWidth={2} dot={false} />
        </LineChart>
      </ResponsiveContainer>
    </div>
  );
}
