import { useQuery } from "@tanstack/react-query";
import {
  Activity,
  Clock,
  Keyboard,
  type LucideIcon,
  MousePointer,
} from "lucide-react";

import { Badge } from "@/components/ui/badge";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import {
  getActivitySummary,
  getAppStatus,
  listRecentEvents,
  type InputEventPreview,
} from "@/lib/mock-api";

const eventKindLabel: Record<InputEventPreview["kind"], string> = {
  key_press: "Key press",
  key_release: "Key release",
  button_press: "Button press",
  button_release: "Button release",
  wheel: "Wheel",
};

function formatDateTime(value?: string) {
  if (!value) {
    return "No events yet";
  }

  return new Intl.DateTimeFormat("zh-CN", {
    dateStyle: "medium",
    timeStyle: "medium",
  }).format(new Date(value));
}

function compactNumber(value?: number) {
  return new Intl.NumberFormat("en-US", {
    notation: "compact",
    maximumFractionDigits: 1,
  }).format(value ?? 0);
}

export function OverviewPage() {
  const statusQuery = useQuery({
    queryKey: ["app-status"],
    queryFn: getAppStatus,
  });
  const summaryQuery = useQuery({
    queryKey: ["activity-summary"],
    queryFn: getActivitySummary,
  });
  const eventsQuery = useQuery({
    queryKey: ["recent-events"],
    queryFn: listRecentEvents,
  });

  const status = statusQuery.data;
  const summary = summaryQuery.data;
  const events = eventsQuery.data ?? [];

  return (
    <div className="flex flex-col gap-5">
      <section className="grid gap-4 sm:grid-cols-2 xl:grid-cols-4">
        <MetricCard
          icon={Activity}
          label="Total events"
          value={compactNumber(summary?.totalEvents)}
          caption="Captured in the current local sample"
        />
        <MetricCard
          icon={Keyboard}
          label="Keyboard events"
          value={compactNumber(summary?.keyEvents)}
          caption="Press and release signals"
        />
        <MetricCard
          icon={MousePointer}
          label="Pointer events"
          value={compactNumber((summary?.buttonEvents ?? 0) + (summary?.wheelEvents ?? 0))}
          caption="Buttons and wheel activity"
        />
        <MetricCard
          icon={Clock}
          label="Last event"
          value={status ? formatDateTime(status.lastEventAt) : "Loading"}
          caption={status?.listenerRunning ? "Listener is receiving events" : "Waiting for listener"}
          valueClassName="text-lg"
        />
      </section>

      <section className="grid gap-5 xl:grid-cols-[minmax(0,1fr)_22rem]">
        <Card className="min-w-0">
          <CardHeader className="flex flex-col gap-3 sm:flex-row sm:items-center sm:justify-between">
            <div>
              <CardTitle>Recent events</CardTitle>
              <CardDescription>
                Placeholder data for the upcoming Tauri query interface.
              </CardDescription>
            </div>
            <Badge
              className="w-fit gap-1.5"
              variant={status?.databaseReady ? "success" : "secondary"}
            >
              <span className="size-2 rounded-full bg-current" />
              {status?.databaseReady ? "Database ready" : "Database pending"}
            </Badge>
          </CardHeader>
          <CardContent>
            <Table>
              <TableHeader>
                <TableRow>
                  <TableHead className="min-w-36">Time</TableHead>
                  <TableHead>Type</TableHead>
                  <TableHead>Value</TableHead>
                  <TableHead className="min-w-36">App</TableHead>
                  <TableHead className="min-w-64">Window</TableHead>
                </TableRow>
              </TableHeader>
              <TableBody>
                {events.map((event) => (
                  <TableRow key={event.id}>
                    <TableCell className="whitespace-nowrap text-muted-foreground">
                      {formatDateTime(event.occurredAt)}
                    </TableCell>
                    <TableCell>
                      <Badge variant="outline">{eventKindLabel[event.kind]}</Badge>
                    </TableCell>
                    <TableCell className="font-mono text-xs">{event.value}</TableCell>
                    <TableCell className="font-medium">{event.appName}</TableCell>
                    <TableCell className="max-w-72 truncate text-muted-foreground">
                      {event.windowTitle}
                    </TableCell>
                  </TableRow>
                ))}
              </TableBody>
            </Table>
          </CardContent>
        </Card>

        <Card>
          <CardHeader>
            <CardTitle>Top applications</CardTitle>
            <CardDescription>
              Activity distribution by active window application.
            </CardDescription>
          </CardHeader>
          <CardContent className="flex flex-col gap-4">
            {(summary?.topApps ?? []).map((app) => (
              <div className="flex flex-col gap-2" key={app.appName}>
                <div className="flex items-center justify-between gap-3">
                  <span className="truncate text-sm font-medium">{app.appName}</span>
                  <span className="shrink-0 text-sm text-muted-foreground">
                    {compactNumber(app.eventCount)}
                  </span>
                </div>
                <div className="h-2 rounded-full bg-muted">
                  <div
                    className="h-full rounded-full bg-primary"
                    style={{ width: `${app.share}%` }}
                  />
                </div>
              </div>
            ))}
          </CardContent>
        </Card>
      </section>
    </div>
  );
}

function MetricCard({
  icon: Icon,
  label,
  value,
  caption,
  valueClassName,
}: {
  icon: LucideIcon;
  label: string;
  value: string;
  caption: string;
  valueClassName?: string;
}) {
  return (
    <Card>
      <CardHeader className="flex flex-row items-center justify-between gap-3 pb-3">
        <CardTitle className="text-sm font-medium text-muted-foreground">
          {label}
        </CardTitle>
        <div className="flex size-9 items-center justify-center rounded-md bg-secondary text-secondary-foreground">
          <Icon className="size-4" />
        </div>
      </CardHeader>
      <CardContent>
        <div className={valueClassName ?? "text-2xl font-semibold"}>{value}</div>
        <p className="mt-1 text-sm text-muted-foreground">{caption}</p>
      </CardContent>
    </Card>
  );
}
