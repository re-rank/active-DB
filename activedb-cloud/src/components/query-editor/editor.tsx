"use client";

import { useState } from "react";
import dynamic from "next/dynamic";
import { Play } from "lucide-react";

const MonacoEditor = dynamic(() => import("@monaco-editor/react"), { ssr: false });

interface QueryEditorProps {
  instanceId: string;
  onResult?: (result: unknown) => void;
}

export function QueryEditor({ instanceId, onResult }: QueryEditorProps) {
  const [query, setQuery] = useState("QUERY example() =>\n  nodes <- N<User>\n  RETURN nodes");
  const [loading, setLoading] = useState(false);

  const handleRun = async () => {
    setLoading(true);
    try {
      const res = await fetch(`/api/v1/instances/${instanceId}/query`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ query }),
      });
      const data = await res.json();
      onResult?.(data);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="space-y-3">
      <div className="flex items-center justify-between">
        <h3 className="text-sm font-medium">AQL Editor</h3>
        <button
          onClick={handleRun}
          disabled={loading}
          className="flex items-center gap-1.5 rounded-md bg-primary px-3 py-1.5 text-xs font-medium text-primary-foreground hover:bg-primary/90 disabled:opacity-50"
        >
          <Play className="h-3 w-3" />
          {loading ? "Running..." : "Run"}
        </button>
      </div>
      <div className="rounded-lg border overflow-hidden">
        <MonacoEditor
          height="300px"
          language="rust"
          theme="vs-dark"
          value={query}
          onChange={(v) => setQuery(v ?? "")}
          options={{
            minimap: { enabled: false },
            fontSize: 13,
            lineNumbers: "on",
            scrollBeyondLastLine: false,
            automaticLayout: true,
          }}
        />
      </div>
    </div>
  );
}
