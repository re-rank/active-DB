"use client";

import { useParams } from "next/navigation";
import { CpuChart } from "@/components/metrics/cpu-chart";
import { MemoryChart } from "@/components/metrics/memory-chart";
import { QueriesChart } from "@/components/metrics/queries-chart";
import useSWR from "swr";

const fetcher = (url: string) => fetch(url).then((r) => r.json());

export default function MetricsPage() {
  const { id } = useParams<{ id: string }>();
  const { data } = useSWR(`/api/v1/instances/${id}/metrics?period=1h`, fetcher, {
    refreshInterval: 30000,
  });

  const dataPoints = data?.dataPoints ?? [];

  return (
    <div className="mt-6 space-y-4">
      <h2 className="text-lg font-semibold">Metrics</h2>
      <div className="grid gap-4 lg:grid-cols-2">
        <CpuChart data={dataPoints.map((p: { timestamp: string; cpu_percent: number }) => ({ timestamp: p.timestamp, value: p.cpu_percent }))} />
        <MemoryChart data={dataPoints.map((p: { timestamp: string; memory_mb: number }) => ({ timestamp: p.timestamp, value: p.memory_mb }))} />
      </div>
      <QueriesChart data={dataPoints.map((p: { timestamp: string; queries_per_sec: number }) => ({ timestamp: p.timestamp, value: p.queries_per_sec }))} />
    </div>
  );
}
