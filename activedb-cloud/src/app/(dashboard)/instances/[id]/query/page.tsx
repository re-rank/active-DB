"use client";

import { useParams } from "next/navigation";
import { useState } from "react";
import { QueryEditor } from "@/components/query-editor/editor";
import { ResultTable } from "@/components/query-editor/result-table";

export default function QueryPage() {
  const { id } = useParams<{ id: string }>();
  const [result, setResult] = useState<unknown>(null);

  return (
    <div className="mt-6 space-y-6">
      <h2 className="text-lg font-semibold">Query Editor</h2>
      <QueryEditor instanceId={id} onResult={setResult} />
      <ResultTable data={result} />
    </div>
  );
}
