"use client";

import Link from "next/link";
import { Database, Settings, Users, BookOpen } from "lucide-react";

const navigation = [
  { name: "Instances", href: "/dashboard", icon: Database },
  { name: "Settings", href: "/settings", icon: Settings },
  { name: "Organization", href: "/org", icon: Users },
  { name: "Docs", href: "https://docs.activedb.dev", icon: BookOpen, external: true },
];

export function Sidebar() {
  return (
    <aside className="hidden w-64 border-r bg-muted/40 lg:block">
      <div className="flex h-full flex-col">
        <div className="flex h-14 items-center border-b px-4">
          <span className="text-lg font-bold">ActiveDB</span>
        </div>
        <nav className="flex-1 space-y-1 p-2">
          {navigation.map((item) => (
            <Link
              key={item.name}
              href={item.href}
              className="flex items-center gap-3 rounded-md px-3 py-2 text-sm hover:bg-muted"
              {...(item.external ? { target: "_blank", rel: "noopener" } : {})}
            >
              <item.icon className="h-4 w-4" />
              {item.name}
            </Link>
          ))}
        </nav>
      </div>
    </aside>
  );
}
