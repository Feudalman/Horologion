import { useQuery } from "@tanstack/react-query";
import {
  Activity,
  Clock,
  Keyboard,
  type LucideIcon,
  MousePointer,
} from "lucide-react";
import { useTranslation } from "react-i18next";
import { useNavigate } from "react-router-dom";

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
} from "@/lib/api";
import { formatCompactDateTime } from "@/lib/format";

function compactNumber(value: number | undefined, locale: string) {
  return new Intl.NumberFormat(locale, {
    notation: "compact",
    maximumFractionDigits: 1,
  }).format(value ?? 0);
}

export function OverviewPage() {
  const { i18n, t } = useTranslation();
  const navigate = useNavigate();
  const locale = i18n.language;
  const statusQuery = useQuery({
    queryKey: ["app-status"],
    queryFn: getAppStatus,
    refetchInterval: 5_000,
  });
  const summaryQuery = useQuery({
    queryKey: ["activity-summary"],
    queryFn: getActivitySummary,
    refetchInterval: 5_000,
  });
  const eventsQuery = useQuery({
    queryKey: ["recent-events"],
    queryFn: listRecentEvents,
    refetchInterval: 5_000,
  });

  const status = statusQuery.data;
  const summary = summaryQuery.data;
  const events = eventsQuery.data ?? [];

  return (
    <div className="flex flex-col gap-5 pb-1">
      <section className="grid gap-4 sm:grid-cols-2 xl:grid-cols-4">
        <MetricCard
          icon={Activity}
          label={t("overview.metrics.totalEvents")}
          value={compactNumber(summary?.totalEvents, locale)}
          caption={t("overview.metrics.totalEventsCaption")}
        />
        <MetricCard
          icon={Keyboard}
          label={t("overview.metrics.keyboardEvents")}
          value={compactNumber(summary?.keyEvents, locale)}
          caption={t("overview.metrics.keyboardEventsCaption")}
        />
        <MetricCard
          icon={MousePointer}
          label={t("overview.metrics.pointerEvents")}
          value={compactNumber(
            (summary?.buttonEvents ?? 0) + (summary?.wheelEvents ?? 0),
            locale,
          )}
          caption={t("overview.metrics.pointerEventsCaption")}
        />
        <MetricCard
          icon={Clock}
          label={t("overview.metrics.lastEvent")}
          value={
            status
              ? formatCompactDateTime(
                  status.lastEventAt,
                  t("common.noEventsYet"),
                )
              : t("common.loading")
          }
          caption={
            status?.listenerRunning
              ? t("overview.metrics.listenerReceiving")
              : t("overview.metrics.listenerWaiting")
          }
          valueClassName="text-lg"
        />
      </section>

      <section className="grid gap-5 xl:grid-cols-[minmax(0,1fr)_22rem]">
        <Card className="min-w-0">
          <CardHeader className="flex flex-col gap-3 sm:flex-row sm:items-center sm:justify-between">
            <div>
              <CardTitle>{t("overview.recentEvents.title")}</CardTitle>
              <CardDescription>
                {t("overview.recentEvents.description")}
              </CardDescription>
            </div>
            <Badge
              className="w-fit gap-1.5"
              variant={status?.databaseReady ? "success" : "secondary"}
            >
              <span className="size-2 rounded-full bg-current" />
              {status?.databaseReady
                ? t("common.databaseReady")
                : t("common.databasePending")}
            </Badge>
          </CardHeader>
          <CardContent>
            <Table>
              <TableHeader>
                <TableRow>
                  <TableHead className="min-w-36 whitespace-nowrap">
                    {t("overview.recentEvents.time")}
                  </TableHead>
                  <TableHead className="whitespace-nowrap">{t("common.type")}</TableHead>
                  <TableHead>{t("common.value")}</TableHead>
                  <TableHead className="min-w-36">{t("common.app")}</TableHead>
                  <TableHead className="min-w-64">
                    {t("common.window")}
                  </TableHead>
                </TableRow>
              </TableHeader>
              <TableBody>
                {events.map((event) => (
                  <TableRow
                    className="cursor-pointer"
                    key={event.id}
                    onClick={() => navigate(`/events/${event.id}`)}
                    onKeyDown={(keyboardEvent) => {
                      if (keyboardEvent.key === "Enter") {
                        navigate(`/events/${event.id}`);
                      }
                    }}
                    role="link"
                    tabIndex={0}
                  >
                    <TableCell className="whitespace-nowrap text-muted-foreground">
                      {formatCompactDateTime(
                        event.occurredAt,
                        t("common.noEventsYet"),
                      )}
                    </TableCell>
                    <TableCell className="whitespace-nowrap">
                      <Badge className="whitespace-nowrap" variant="outline">
                        {t(`events.kind.${event.kind}`)}
                      </Badge>
                    </TableCell>
                    <TableCell className="font-mono text-xs">{event.value}</TableCell>
                    <TableCell className="font-medium">{event.appName}</TableCell>
                    <TableCell className="max-w-72 truncate text-muted-foreground">
                      {event.windowId ? (
                        <button
                          className="max-w-full truncate text-left underline-offset-4 hover:text-foreground hover:underline"
                          onClick={(clickEvent) => {
                            clickEvent.stopPropagation();
                            navigate(`/windows/${event.windowId}`);
                          }}
                          type="button"
                        >
                          {event.windowTitle}
                        </button>
                      ) : (
                        event.windowTitle
                      )}
                    </TableCell>
                  </TableRow>
                ))}
              </TableBody>
            </Table>
          </CardContent>
        </Card>

        <Card>
          <CardHeader>
            <CardTitle>{t("overview.topApplications.title")}</CardTitle>
            <CardDescription>
              {t("overview.topApplications.description")}
            </CardDescription>
          </CardHeader>
          <CardContent className="flex flex-col gap-4">
            {(summary?.topApps ?? []).map((app) => (
              <div className="flex flex-col gap-2" key={app.appName}>
                <div className="flex items-center justify-between gap-3">
                  <span className="truncate text-sm font-medium">{app.appName}</span>
                  <span className="shrink-0 text-sm text-muted-foreground">
                    {compactNumber(app.eventCount, locale)}
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
