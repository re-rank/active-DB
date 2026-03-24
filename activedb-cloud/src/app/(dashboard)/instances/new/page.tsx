import { Breadcrumb } from "@/components/layout/breadcrumb";
import { CreateInstanceForm } from "@/components/instances/create-form";

export default function NewInstancePage() {
  return (
    <div>
      <Breadcrumb
        items={[
          { label: "Instances", href: "/dashboard/instances" },
          { label: "Create Instance" },
        ]}
      />
      <h1 className="text-2xl font-bold mt-4 mb-6">Create Instance</h1>
      <CreateInstanceForm />
    </div>
  );
}
