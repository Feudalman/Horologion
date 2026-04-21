import * as React from "react";
import { keepPreviousData, useQuery } from "@tanstack/react-query";
import { ArrowLeft, Hash, Search, X } from "lucide-react";
import { useTranslation } from "react-i18next";
import { Link, useNavigate, useParams } from "react-router-dom";

import { PaginationControls } from "@/components/app/PaginationControls";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import {
  type ObservedWindowSortBy,
  type SortDirection,
  getObservedWindow,
  listInputEvents,
  listObservedWindows,
} from "@/lib/api";
import { formatCompactDateTime } from "@/lib/format";

type SortOption = {
  value: string;
  sortBy: ObservedWindowSortBy;
  sortDirection: SortDirection;
  labelKey: string;
};

const sortOptions: SortOption[] = [
  {
    value: "last_seen_at:desc",
    sortBy: "last_seen_at",
    sortDirection: "desc",
    labelKey: "windowsPage.sort.active",
  },
  {
    value: "first_seen_at:asc",
    sortBy: "first_seen_at",
    sortDirection: "asc",
    labelKey: "windowsPage.sort.earliest",
  },
  {
    value: "event_count:desc",
    sortBy: "event_count",
    sortDirection: "desc",
    labelKey: "windowsPage.sort.mostEvents",
  },
  {
    value: "app_name:asc",
    sortBy: "app_name",
    sortDirection: "asc",
    labelKey: "windowsPage.sort.appName",
  },
];

const defaultSort = sortOptions[0];

function formatWindowSize(width: number | null, height: number | null) {
  if (width === null || height === null) {
    return "-";
  }

  return `${Math.round(width)} x ${Math.round(height)}`;
}

export function WindowsPage() {
  const { i18n, t } = useTranslation();
  const navigate = useNavigate();
  const [page, setPage] = React.useState(1);
  const [size, setSize] = React.useState(50);
  const [draftAppName, setDraftAppName] = React.useState("");
  const [draftContextHash, setDraftContextHash] = React.useState("");
  const [appName, setAppName] = React.useState("");
  const [contextHash, setContextHash] = React.useState("");
  const [sortValue, setSortValue] = React.useState(defaultSort.value);
  const sort = sortOptions.find((option) => option.value === sortValue) ?? defaultSort;

  const windowsQuery = useQuery({
    queryKey: ["observed-windows", page, size, appName, contextHash, sort.value],
    queryFn: () =>
      listObservedWindows({
        page,
        size,
        appName,
        contextHash,
        sortBy: sort.sortBy,
        sortDirection: sort.sortDirection,
      }),
    placeholderData: keepPreviousData,
  });

  const data = windowsQuery.data;
  const windows = data?.list ?? [];

  function applyFilters(event: React.FormEvent<HTMLFormElement>) {
    event.preventDefault();
    setPage(1);
    setAppName(draftAppName.trim());
    setContextHash(draftContextHash.trim());
  }

  function clearFilters() {
    setDraftAppName("");
    setDraftContextHash("");
    setAppName("");
    setContextHash("");
    setSortValue(defaultSort.value);
    setPage(1);
  }

  return (
    <Card className="flex h-full min-h-0 flex-col">
      <CardHeader className="shrink-0 gap-4">
        <div className="flex flex-col gap-3 xl:flex-row xl:items-center xl:justify-between">
          <CardTitle>{t("windowsPage.table.title")}</CardTitle>
          <form
            className="grid gap-2 md:grid-cols-[minmax(0,1fr)_minmax(0,1fr)_11rem_auto_auto]"
            onSubmit={applyFilters}
          >
            <Input
              aria-label={t("windowsPage.filters.appName")}
              onChange={(event) => setDraftAppName(event.target.value)}
              placeholder={t("windowsPage.filters.appNamePlaceholder")}
              value={draftAppName}
            />
            <div className="relative">
              <Hash className="pointer-events-none absolute left-3 top-1/2 size-4 -translate-y-1/2 text-muted-foreground" />
              <Input
                aria-label={t("windowsPage.filters.contextHash")}
                className="pl-9"
                onChange={(event) => setDraftContextHash(event.target.value)}
                placeholder={t("windowsPage.filters.contextHashPlaceholder")}
                value={draftContextHash}
              />
            </div>
            <select
              aria-label={t("windowsPage.filters.sort")}
              className="h-9 rounded-md border border-input bg-background px-3 text-sm shadow-sm focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
              onChange={(event) => {
                setSortValue(event.target.value);
                setPage(1);
              }}
              value={sortValue}
            >
              {sortOptions.map((option) => (
                <option key={option.value} value={option.value}>
                  {t(option.labelKey)}
                </option>
              ))}
            </select>
            <Button size="sm" type="submit">
              <Search />
              {t("windowsPage.filters.apply")}
            </Button>
            <Button onClick={clearFilters} size="sm" type="button" variant="outline">
              <X />
              {t("windowsPage.filters.clear")}
            </Button>
          </form>
        </div>
      </CardHeader>

      <CardContent className="flex min-h-0 flex-1 flex-col gap-4">
        <div className="min-h-0 flex-1 overflow-auto">
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead className="min-w-36">{t("windowsPage.table.app")}</TableHead>
                <TableHead className="min-w-72">{t("windowsPage.table.window")}</TableHead>
                <TableHead>{t("windowsPage.table.eventCount")}</TableHead>
                <TableHead>{t("windowsPage.table.processId")}</TableHead>
                <TableHead>{t("windowsPage.table.size")}</TableHead>
                <TableHead className="min-w-36 whitespace-nowrap">
                  {t("windowsPage.table.firstSeen")}
                </TableHead>
                <TableHead className="min-w-36 whitespace-nowrap">
                  {t("windowsPage.table.lastSeen")}
                </TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              {windows.length === 0 ? (
                <TableRow>
                  <TableCell className="text-center text-muted-foreground" colSpan={7}>
                    {windowsQuery.isFetching
                      ? t("common.loading")
                      : t("windowsPage.table.empty")}
                  </TableCell>
                </TableRow>
              ) : (
                windows.map((window) => (
                  <TableRow
                    className="cursor-pointer"
                    key={window.id}
                    onClick={() => navigate(`/windows/${window.id}`)}
                    onKeyDown={(keyboardEvent) => {
                      if (keyboardEvent.key === "Enter") {
                        navigate(`/windows/${window.id}`);
                      }
                    }}
                    role="link"
                    tabIndex={0}
                  >
                    <TableCell className="font-medium">{window.appName}</TableCell>
                    <TableCell className="max-w-96 truncate text-muted-foreground">
                      {window.title}
                    </TableCell>
                    <TableCell>{window.eventCount.toLocaleString(i18n.language)}</TableCell>
                    <TableCell className="text-muted-foreground">
                      {window.processId ?? "-"}
                    </TableCell>
                    <TableCell className="whitespace-nowrap text-muted-foreground">
                      {formatWindowSize(window.width, window.height)}
                    </TableCell>
                    <TableCell className="whitespace-nowrap text-muted-foreground">
                      {formatCompactDateTime(window.firstSeenAt)}
                    </TableCell>
                    <TableCell className="whitespace-nowrap text-muted-foreground">
                      {formatCompactDateTime(window.lastSeenAt)}
                    </TableCell>
                  </TableRow>
                ))
              )}
            </TableBody>
          </Table>
        </div>

        <PaginationControls
          onPageChange={setPage}
          onSizeChange={(nextSize) => {
            setSize(nextSize);
            setPage(1);
          }}
          page={data?.page ?? page}
          pages={data?.pages ?? 0}
          size={size}
          total={data?.total ?? 0}
        />
      </CardContent>
    </Card>
  );
}

export function WindowDetailPlaceholderPage() {
  const { i18n, t } = useTranslation();
  const navigate = useNavigate();
  const { windowId } = useParams();
  const parsedWindowId = Number(windowId);
  const [page, setPage] = React.useState(1);
  const [size, setSize] = React.useState(25);
  const windowQuery = useQuery({
    queryKey: ["observed-window", parsedWindowId],
    queryFn: () => getObservedWindow(parsedWindowId),
    enabled: Number.isFinite(parsedWindowId),
  });
  const eventsQuery = useQuery({
    queryKey: ["window-events", parsedWindowId, page, size],
    queryFn: () =>
      listInputEvents({
        page,
        size,
        windowId: parsedWindowId,
        sortBy: "occurred_at",
        sortDirection: "desc",
      }),
    enabled: Number.isFinite(parsedWindowId),
    placeholderData: keepPreviousData,
  });
  const window = windowQuery.data;
  const events = eventsQuery.data?.list ?? [];

  if (windowQuery.isLoading) {
    return (
      <Card>
        <CardContent className="p-6 text-sm text-muted-foreground">
          {t("common.loading")}
        </CardContent>
      </Card>
    );
  }

  if (!window) {
    return (
      <Card>
        <CardContent className="p-6 text-sm text-muted-foreground">
          {t("windowsPage.detail.notFound")}
        </CardContent>
      </Card>
    );
  }

  return (
    <div className="flex h-full min-h-0 flex-col gap-5 overflow-auto pb-1">
      <div>
        <Button asChild size="sm" variant="outline">
          <Link to="/windows">
            <ArrowLeft />
            {t("windowsPage.detail.back")}
          </Link>
        </Button>
      </div>

      <section className="grid gap-5 xl:grid-cols-[minmax(0,1fr)_24rem]">
        <Card>
          <CardHeader>
            <CardTitle>{t("windowsPage.detail.title")}</CardTitle>
          </CardHeader>
          <CardContent className="grid gap-4 sm:grid-cols-2">
            <DetailItem label={t("windowsPage.detail.windowId")} value={`#${window.id}`} />
            <DetailItem label={t("windowsPage.table.app")} value={window.appName} />
            <DetailItem
              label={t("windowsPage.table.window")}
              value={window.title || "-"}
            />
            <DetailItem
              label={t("windowsPage.table.eventCount")}
              value={window.eventCount.toLocaleString(i18n.language)}
            />
            <DetailItem
              label={t("windowsPage.table.firstSeen")}
              value={formatCompactDateTime(window.firstSeenAt)}
            />
            <DetailItem
              label={t("windowsPage.table.lastSeen")}
              value={formatCompactDateTime(window.lastSeenAt)}
            />
          </CardContent>
        </Card>

        <Card>
          <CardHeader>
            <CardTitle>{t("windowsPage.detail.process")}</CardTitle>
          </CardHeader>
          <CardContent className="flex flex-col gap-4">
            <DetailItem
              label={t("windowsPage.table.processId")}
              value={window.processId ? String(window.processId) : "-"}
            />
            <DetailItem
              label={t("windowsPage.detail.processPath")}
              value={window.processPath ?? "-"}
            />
            <DetailItem
              label={t("windowsPage.table.size")}
              value={formatWindowSize(window.width, window.height)}
            />
            <DetailItem
              label={t("windowsPage.detail.position")}
              value={formatWindowPosition(window.x, window.y)}
            />
            <DetailItem
              label={t("windowsPage.detail.contextHash")}
              value={window.contextHash}
            />
          </CardContent>
        </Card>
      </section>

      <Card className="flex min-h-[24rem] flex-1 flex-col">
        <CardHeader className="shrink-0">
          <CardTitle>{t("windowsPage.detail.relatedEvents")}</CardTitle>
        </CardHeader>
        <CardContent className="flex min-h-0 flex-1 flex-col gap-4">
          <div className="min-h-0 flex-1 overflow-auto">
            <Table>
              <TableHeader>
                <TableRow>
                  <TableHead className="min-w-36 whitespace-nowrap">
                    {t("eventsPage.table.time")}
                  </TableHead>
                  <TableHead className="whitespace-nowrap">
                    {t("eventsPage.table.type")}
                  </TableHead>
                  <TableHead>{t("eventsPage.table.value")}</TableHead>
                  <TableHead className="min-w-32">
                    {t("eventsPage.table.collector")}
                  </TableHead>
                </TableRow>
              </TableHeader>
              <TableBody>
                {events.length === 0 ? (
                  <TableRow>
                    <TableCell className="text-center text-muted-foreground" colSpan={4}>
                      {eventsQuery.isFetching
                        ? t("common.loading")
                        : t("eventsPage.table.empty")}
                    </TableCell>
                  </TableRow>
                ) : (
                  events.map((event) => (
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
                        {formatCompactDateTime(event.occurredAt)}
                      </TableCell>
                      <TableCell className="whitespace-nowrap">
                        <Badge className="whitespace-nowrap" variant="outline">
                          {t(`events.kind.${event.kind}`)}
                        </Badge>
                      </TableCell>
                      <TableCell className="max-w-72 truncate font-mono text-xs">
                        {event.value}
                      </TableCell>
                      <TableCell className="whitespace-nowrap text-muted-foreground">
                        {event.collectorName} {event.collectorVersion}
                      </TableCell>
                    </TableRow>
                  ))
                )}
              </TableBody>
            </Table>
          </div>

          <PaginationControls
            onPageChange={setPage}
            onSizeChange={(nextSize) => {
              setSize(nextSize);
              setPage(1);
            }}
            page={eventsQuery.data?.page ?? page}
            pages={eventsQuery.data?.pages ?? 0}
            size={size}
            total={eventsQuery.data?.total ?? 0}
          />
        </CardContent>
      </Card>
    </div>
  );
}

function DetailItem({ label, value }: { label: string; value: string }) {
  return (
    <div className="min-w-0">
      <div className="text-sm text-muted-foreground">{label}</div>
      <div className="break-words text-sm font-medium">{value}</div>
    </div>
  );
}

function formatWindowPosition(x: number | null, y: number | null) {
  if (x === null || y === null) {
    return "-";
  }

  return `${Math.round(x)}, ${Math.round(y)}`;
}
