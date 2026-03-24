import { db } from "@/lib/db";
import { instances } from "@/db/schema";
import { eq } from "drizzle-orm";
import { notFound } from "next/navigation";

type Params = { params: Promise<{ id: string }> };

export default async function InstanceSettingsPage({ params }: Params) {
  const { id } = await params;
  const [instance] = await db.select().from(instances).where(eq(instances.id, id)).limit(1);
  if (!instance) notFound();

  return (
    <div className="mt-6 max-w-2xl space-y-8">
      <h2 className="text-lg font-semibold">Instance Settings</h2>

      {/* General */}
      <section className="space-y-4">
        <h3 className="text-sm font-medium">General</h3>
        <div className="rounded-lg border p-4 space-y-3">
          <div>
            <label className="block text-xs text-muted-foreground">Instance ID</label>
            <code className="text-sm">{instance.id}</code>
          </div>
          <div>
            <label className="block text-xs text-muted-foreground">Region</label>
            <span className="text-sm">{instance.region}</span>
          </div>
          <div>
            <label className="block text-xs text-muted-foreground">Plan</label>
            <span className="text-sm capitalize">{instance.tier}</span>
          </div>
          <div>
            <label className="block text-xs text-muted-foreground">Resources</label>
            <span className="text-sm">
              CPU: {instance.cpuLimit} / Memory: {instance.memoryLimit} / Storage: {instance.storageSize}
            </span>
          </div>
        </div>
      </section>

      {/* Danger Zone */}
      <section className="space-y-4">
        <h3 className="text-sm font-medium text-red-600">Danger Zone</h3>
        <div className="rounded-lg border border-red-200 p-4">
          <div className="flex items-center justify-between">
            <div>
              <h4 className="text-sm font-medium">Delete Instance</h4>
              <p className="text-xs text-muted-foreground">
                This will permanently delete this instance and all its data.
              </p>
            </div>
            <form action={`/api/v1/instances/${instance.id}`} method="DELETE">
              <button
                type="submit"
                className="rounded-md border border-red-300 px-3 py-1.5 text-sm font-medium text-red-600 hover:bg-red-50"
              >
                Delete
              </button>
            </form>
          </div>
        </div>
      </section>
    </div>
  );
}
