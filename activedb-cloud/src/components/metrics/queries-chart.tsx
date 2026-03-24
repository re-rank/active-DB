"use client";

import { BarChart, Bar, XAxis, YAxis, CartesianGrid, Tooltip, ResponsiveContainer } from "recharts";

interface DataPoint {
  timestamp: string;
  value: number;
}

export function QueriesChart({ data }: { data: DataPoint[] }) {
  return (
    <div className="rounded-lg border p-4">
      <h3 className="text-sm font-medium mb-4">Queries / sec</h3>
      <ResponsiveContainer width="100%" height={200}>
        <BarChart data={data}>
          <CartesianGrid strokeDasharray="3 3" className="stroke-muted" />
          <XAxis dataKey="timestamp" className="text-xs" />
          <YAxis className="text-xs" />
          <Tooltip />
          <Bar dataKey="value" fill="hsl(var(--primary))" radius={[2, 2, 0, 0]} />
        </BarChart>
      </ResponsiveContainer>
    </div>
  );
}
