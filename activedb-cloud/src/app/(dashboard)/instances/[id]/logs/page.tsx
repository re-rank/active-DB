"use client";

import { useParams } from "next/navigation";
import { useSSE } from "@/hooks/use-sse";
import { useState } from "react";

interface LogEntry {
  timestamp: string;
  level: string;
  message: string;
}

export default function LogsPage() {
  const { id } = useParams<{ id: string }>();
  const [streaming, setStreaming] = useState(false);
  const { data } = useSSE<LogEntry>(
    streaming ? `/api/v1/instances/${id}/logs/stream` : null,
  );
  const [logs, setLogs] = useState<LogEntry[]>([]);

  // Append new log entries from SSE
  if (data && !logs.find((l) => l.timestamp === data.timestamp && l.message === data.message)) {
    setLogs((prev) => [...prev.slice(-500), data]);
  }

  return (
    <div className="mt-6 space-y-4">
      <div className="flex items-center justify-between">
        <h2 className="text-lg font-semibold">Logs</h2>
        <button
          onClick={() => setStreaming(!streaming)}
          className={`rounded-md px-3 py-1.5 text-sm font-medium ${
            streaming
              ? "bg-red-100 text-red-700 hover:bg-red-200"
              : "bg-primary text-primary-foreground hover:bg-primary/90"
          }`}
        >
          {streaming ? "Stop Streaming" : "Start Streaming"}
        </button>
      </div>
      <div className="rounded-lg border bg-black/95 p-4 font-mono text-xs text-green-400 h-[500px] overflow-y-auto">
        {logs.length === 0 ? (
          <span className="text-gray-500">No logs yet. Start streaming to see real-time logs.</span>
        ) : (
          logs.map((log, i) => (
            <div key={i} className="leading-relaxed">
              <span className="text-gray-500">{log.timestamp}</span>{" "}
              <span className={log.level === "error" ? "text-red-400" : log.level === "warn" ? "text-yellow-400" : "text-green-400"}>
                [{log.level}]
              </span>{" "}
              {log.message}
            </div>
          ))
        )}
      </div>
    </div>
  );
}
