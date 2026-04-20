import { useQuery } from "@tanstack/react-query";
import { Database, type LucideIcon, Monitor, Moon, Server, Sun } from "lucide-react";

import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Separator } from "@/components/ui/separator";
import { getAppStatus } from "@/lib/mock-api";
import { type Theme, useTheme } from "@/lib/theme";
import { cn } from "@/lib/utils";

const themeOptions: Array<{
  value: Theme;
  label: string;
  description: string;
  icon: LucideIcon;
}> = [
  {
    value: "light",
    label: "Light",
    description: "Bright interface for daytime work.",
    icon: Sun,
  },
  {
    value: "dark",
    label: "Dark",
    description: "Low-glare interface for evening sessions.",
    icon: Moon,
  },
  {
    value: "system",
    label: "System",
    description: "Follow the operating system appearance.",
    icon: Monitor,
  },
];

export function SettingsPage() {
  const { theme, resolvedTheme, setTheme } = useTheme();
  const statusQuery = useQuery({
    queryKey: ["settings-status"],
    queryFn: getAppStatus,
  });
  const status = statusQuery.data;

  return (
    <div className="grid gap-5 xl:grid-cols-[minmax(0,1fr)_24rem]">
      <Card>
        <CardHeader>
          <CardTitle>Theme mode</CardTitle>
          <CardDescription>
            Switch between light, dark, and system-controlled appearance.
          </CardDescription>
        </CardHeader>
        <CardContent className="grid gap-3 sm:grid-cols-3">
          {themeOptions.map((option) => (
            <Button
              className={cn(
                "h-auto min-h-28 flex-col items-start justify-start gap-3 p-4 text-left",
                theme === option.value && "border-primary bg-accent text-accent-foreground",
              )}
              key={option.value}
              onClick={() => setTheme(option.value)}
              type="button"
              variant="outline"
            >
              <span className="flex w-full items-center justify-between gap-2">
                <span className="flex items-center gap-2 font-semibold">
                  <option.icon className="size-4" />
                  {option.label}
                </span>
                {theme === option.value ? (
                  <Badge variant="success">Active</Badge>
                ) : null}
              </span>
              <span className="text-wrap text-sm font-normal text-muted-foreground">
                {option.description}
              </span>
            </Button>
          ))}
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle>Runtime</CardTitle>
          <CardDescription>
            Placeholder values until the Tauri commands are connected.
          </CardDescription>
        </CardHeader>
        <CardContent className="flex flex-col gap-4">
          <InfoRow
            icon={Server}
            label="Run mode"
            value={status?.runMode ?? "loading"}
          />
          <Separator />
          <InfoRow
            icon={Monitor}
            label="Theme"
            value={`${theme} (${resolvedTheme})`}
          />
          <Separator />
          <InfoRow
            icon={Database}
            label="Version"
            value={status?.version ?? "loading"}
          />
        </CardContent>
      </Card>

      <Card className="xl:col-span-2">
        <CardHeader>
          <CardTitle>Database path</CardTitle>
          <CardDescription>
            The frontend currently uses mock data. This field will be wired to
            `src-tauri` later.
          </CardDescription>
        </CardHeader>
        <CardContent>
          <div className="rounded-md border bg-muted/40 p-3 font-mono text-sm text-muted-foreground">
            <div className="break-all">{status?.databasePath ?? "Loading..."}</div>
          </div>
        </CardContent>
      </Card>
    </div>
  );
}

function InfoRow({
  icon: Icon,
  label,
  value,
}: {
  icon: LucideIcon;
  label: string;
  value: string;
}) {
  return (
    <div className="flex items-start gap-3">
      <div className="flex size-9 shrink-0 items-center justify-center rounded-md bg-secondary text-secondary-foreground">
        <Icon className="size-4" />
      </div>
      <div className="min-w-0">
        <div className="text-sm text-muted-foreground">{label}</div>
        <div className="break-words text-sm font-medium">{value}</div>
      </div>
    </div>
  );
}
