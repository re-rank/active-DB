"use client";

interface ResultTableProps {
  data: unknown;
  executionTime?: number;
}

export function ResultTable({ data, executionTime }: ResultTableProps) {
  if (!data) return null;

  const jsonStr = JSON.stringify(data, null, 2);

  return (
    <div className="space-y-2">
      <div className="flex items-center justify-between text-sm">
        <h3 className="font-medium">Results</h3>
        {executionTime !== undefined && (
          <span className="text-xs text-muted-foreground">{executionTime}ms</span>
        )}
      </div>
      <pre className="max-h-[400px] overflow-auto rounded-lg border bg-muted/30 p-4 text-xs">
        {jsonStr}
      </pre>
    </div>
  );
}
