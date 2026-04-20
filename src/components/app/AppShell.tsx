import * as React from "react";
import {
  Activity,
  BarChart3,
  Circle,
  Database,
  Menu,
  Settings,
} from "lucide-react";
import { NavLink, Outlet, useLocation } from "react-router-dom";

import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Separator } from "@/components/ui/separator";
import { cn } from "@/lib/utils";

const navItems = [
  {
    label: "Overview",
    path: "/overview",
    icon: BarChart3,
  },
  {
    label: "Settings",
    path: "/settings",
    icon: Settings,
  },
];

const pageTitles: Record<string, { title: string; subtitle: string }> = {
  "/overview": {
    title: "Overview",
    subtitle: "Activity capture, recent events, and app-level signal.",
  },
  "/settings": {
    title: "Settings",
    subtitle: "Theme, runtime, version, and local database information.",
  },
};

export function AppShell() {
  const [collapsed, setCollapsed] = React.useState(false);
  const location = useLocation();
  const page = pageTitles[location.pathname] ?? pageTitles["/overview"];

  return (
    <div className="min-h-screen bg-background text-foreground">
      <header className="sticky top-0 z-40 flex h-14 items-center border-b bg-background/95 px-3 backdrop-blur supports-[backdrop-filter]:bg-background/80 sm:px-4">
        <Button
          aria-label={collapsed ? "Expand sidebar" : "Collapse sidebar"}
          className="mr-2"
          onClick={() => setCollapsed((value) => !value)}
          size="icon"
          type="button"
          variant="ghost"
        >
          <Menu />
        </Button>

        <div className="flex min-w-0 flex-1 items-center gap-3">
          <div className="flex size-8 shrink-0 items-center justify-center rounded-md bg-primary text-primary-foreground">
            <Activity className="size-4" />
          </div>
          <div className="min-w-0">
            <div className="truncate text-sm font-semibold">Horologion</div>
            <div className="hidden truncate text-xs text-muted-foreground sm:block">
              Local activity timeline
            </div>
          </div>
        </div>

        <Badge className="hidden gap-1.5 sm:inline-flex" variant="success">
          <Circle className="size-2 fill-current" />
          Listener ready
        </Badge>
      </header>

      <div className="flex min-h-[calc(100vh-3.5rem)]">
        <aside
          className={cn(
            "sticky top-14 h-[calc(100vh-3.5rem)] shrink-0 border-r bg-card transition-[width] duration-200",
            collapsed ? "w-16" : "w-60",
            "max-sm:w-16",
          )}
        >
          <nav className="flex h-full flex-col gap-1 p-2">
            {navItems.map((item) => (
              <NavLink
                className={({ isActive }) =>
                  cn(
                    "group flex h-10 items-center gap-3 rounded-md px-3 text-sm font-medium text-muted-foreground transition-colors hover:bg-accent hover:text-accent-foreground",
                    isActive && "bg-accent text-accent-foreground",
                    collapsed && "justify-center px-0",
                    "max-sm:justify-center max-sm:px-0",
                  )
                }
                key={item.path}
                to={item.path}
                title={item.label}
              >
                <item.icon className="size-4 shrink-0" />
                <span
                  className={cn(
                    "truncate",
                    collapsed && "hidden",
                    "max-sm:hidden",
                  )}
                >
                  {item.label}
                </span>
              </NavLink>
            ))}

            <div className="mt-auto">
              <Separator className="mb-2" />
              <div
                className={cn(
                  "flex items-center gap-3 rounded-md px-3 py-2 text-xs text-muted-foreground",
                  collapsed && "justify-center px-0",
                  "max-sm:justify-center max-sm:px-0",
                )}
              >
                <Database className="size-4 shrink-0" />
                <span className={cn(collapsed && "hidden", "max-sm:hidden")}>
                  Local DuckDB
                </span>
              </div>
            </div>
          </nav>
        </aside>

        <main className="min-w-0 flex-1">
          <div className="mx-auto flex w-full max-w-7xl flex-col gap-5 p-4 sm:p-5 lg:p-6">
            <div className="flex min-w-0 flex-col gap-1">
              <h1 className="truncate text-2xl font-semibold tracking-normal">
                {page.title}
              </h1>
              <p className="text-sm text-muted-foreground">{page.subtitle}</p>
            </div>
            <Outlet />
          </div>
        </main>
      </div>
    </div>
  );
}
