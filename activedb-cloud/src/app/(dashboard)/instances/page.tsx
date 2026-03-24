import Link from "next/link";
import { Plus } from "lucide-react";
import { db } from "@/lib/db";
import { instances } from "@/db/schema";
import { eq, and, isNull } from "drizzle-orm";
import { getServerSession } from "next-auth";
import { authOptions } from "@/lib/auth";
import { redirect } from "next/navigation";
import { InstanceCard } from "@/components/instances/instance-card";

export default async function InstancesPage() {
  const session = await getServerSession(authOptions);
  if (!session?.user?.id) redirect("/login");

  const userInstances = await db
    .select()
    .from(instances)
    .where(
      and(
        eq(instances.ownerId, session.user.id),
        eq(instances.ownerType, "user"),
        isNull(instances.deletedAt),
      ),
    );

  return (
    <div>
      <div className="flex items-center justify-between mb-6">
        <h1 className="text-2xl font-bold">Instances</h1>
        <Link
          href="/dashboard/instances/new"
          className="flex items-center gap-1.5 rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground hover:bg-primary/90"
        >
          <Plus className="h-4 w-4" />
          New Instance
        </Link>
      </div>

      {userInstances.length === 0 ? (
        <div className="rounded-lg border border-dashed p-12 text-center">
          <p className="text-muted-foreground">No instances yet.</p>
          <Link
            href="/dashboard/instances/new"
            className="mt-4 inline-block text-sm text-primary hover:underline"
          >
            Create your first instance
          </Link>
        </div>
      ) : (
        <div className="grid gap-3">
          {userInstances.map((inst) => (
            <InstanceCard
              key={inst.id}
              id={inst.id}
              name={inst.name}
              status={inst.status}
              region={inst.region}
              tier={inst.tier}
            />
          ))}
        </div>
      )}
    </div>
  );
}
