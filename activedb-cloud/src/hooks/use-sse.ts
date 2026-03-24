"use client";

import { useState, useEffect } from "react";

export function useSSE<T>(url: string | null) {
  const [data, setData] = useState<T | null>(null);
  const [status, setStatus] = useState<"connecting" | "open" | "closed">(
    "connecting"
  );

  useEffect(() => {
    if (!url) return;
    const es = new EventSource(url);
    es.onmessage = (e) => setData(JSON.parse(e.data));
    es.onopen = () => setStatus("open");
    es.onerror = () => setStatus("closed");
    return () => es.close();
  }, [url]);

  return { data, status };
}
