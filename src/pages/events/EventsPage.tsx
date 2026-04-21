import * as React from "react";
import { keepPreviousData, useQuery } from "@tanstack/react-query";
import { Search, X } from "lucide-react";
import { useTranslation } from "react-i18next";
import { useNavigate } from "react-router-dom";

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
  type InputEventKind,
  type InputEventSortBy,
  type SortDirection,
  listInputEvents,
} from "@/lib/api";
import { formatCompactDateTime } from "@/lib/format";

const eventKinds: InputEventKind[] = [
  "key_press",
  "key_release",
  "button_press",
  "button_release",
  "wheel",
];

type SortOption = {
  value: string;
  sortBy: InputEventSortBy;
  sortDirection: SortDirection;
  labelKey: string;
};

const sortOptions: SortOption[] = [
  {
    value: "occurred_at:desc",
    sortBy: "occurred_at",
    sortDirection: "desc",
    labelKey: "eventsPage.sort.newest",
  },
  {
    value: "occurred_at:asc",
    sortBy: "occurred_at",
    sortDirection: "asc",
    labelKey: "eventsPage.sort.oldest",
  },
  {
    value: "app_name:asc",
    sortBy: "app_name",
    sortDirection: "asc",
    labelKey: "eventsPage.sort.appName",
  },
  {
    value: "kind:asc",
    sortBy: "kind",
    sortDirection: "asc",
    labelKey: "eventsPage.sort.kind",
  },
  {
    value: "value:asc",
    sortBy: "value",
    sortDirection: "asc",
    labelKey: "eventsPage.sort.value",
  },
];

const defaultSort = sortOptions[0];

export function EventsPage() {
  const { t } = useTranslation();
  const navigate = useNavigate();
  const [page, setPage] = React.useState(1);
  const [size, setSize] = React.useState(50);
  const [draftAppName, setDraftAppName] = React.useState("");
  const [appName, setAppName] = React.useState("");
  const [kind, setKind] = React.useState<InputEventKind | "">("");
  const [sortValue, setSortValue] = React.useState(defaultSort.value);
  const sort = sortOptions.find((option) => option.value === sortValue) ?? defaultSort;

  const eventsQuery = useQuery({
    queryKey: ["input-events", page, size, appName, kind, sort.value],
    queryFn: () =>
      listInputEvents({
        page,
        size,
        appName,
        kind: kind || undefined,
        sortBy: sort.sortBy,
        sortDirection: sort.sortDirection,
      }),
    placeholderData: keepPreviousData,
  });

  const data = eventsQuery.data;
  const events = data?.list ?? [];

  function applyFilters(event: React.FormEvent<HTMLFormElement>) {
    event.preventDefault();
    setPage(1);
    setAppName(draftAppName.trim());
  }

  function clearFilters() {
    setDraftAppName("");
    setAppName("");
    setKind("");
    setSortValue(defaultSort.value);
    setPage(1);
  }

  return (
    <Card>
      <CardHeader className="gap-4">
        <div className="flex flex-col gap-3 lg:flex-row lg:items-center lg:justify-between">
          <CardTitle>{t("eventsPage.table.title")}</CardTitle>
          <form
            className="grid gap-2 sm:grid-cols-[minmax(0,1fr)_11rem_11rem_auto_auto] lg:w-auto"
            onSubmit={applyFilters}
          >
            <Input
              aria-label={t("eventsPage.filters.appName")}
              onChange={(event) => setDraftAppName(event.target.value)}
              placeholder={t("eventsPage.filters.appNamePlaceholder")}
              value={draftAppName}
            />
            <select
              aria-label={t("eventsPage.filters.kind")}
              className="h-9 rounded-md border border-input bg-background px-3 text-sm shadow-sm focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
              onChange={(event) => {
                setKind(event.target.value as InputEventKind | "");
                setPage(1);
              }}
              value={kind}
            >
              <option value="">{t("eventsPage.kind.all")}</option>
              {eventKinds.map((eventKind) => (
                <option key={eventKind} value={eventKind}>
                  {t(`events.kind.${eventKind}`)}
                </option>
              ))}
            </select>
            <select
              aria-label={t("eventsPage.filters.sort")}
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
              {t("eventsPage.filters.apply")}
            </Button>
            <Button onClick={clearFilters} size="sm" type="button" variant="outline">
              <X />
              {t("eventsPage.filters.clear")}
            </Button>
          </form>
        </div>
      </CardHeader>

      <CardContent className="flex flex-col gap-4">
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
              <TableHead className="min-w-36">{t("eventsPage.table.app")}</TableHead>
              <TableHead className="min-w-64">{t("eventsPage.table.window")}</TableHead>
              <TableHead className="min-w-32">{t("eventsPage.table.collector")}</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {events.length === 0 ? (
              <TableRow>
                <TableCell className="text-center text-muted-foreground" colSpan={6}>
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
                  <TableCell className="max-w-56 truncate font-mono text-xs">
                    {event.value}
                  </TableCell>
                  <TableCell className="font-medium">{event.appName}</TableCell>
                  <TableCell className="max-w-80 truncate text-muted-foreground">
                    {event.windowTitle}
                  </TableCell>
                  <TableCell className="whitespace-nowrap text-muted-foreground">
                    {event.collectorName} {event.collectorVersion}
                  </TableCell>
                </TableRow>
              ))
            )}
          </TableBody>
        </Table>

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

export function EventDetailPlaceholderPage() {
  const { t } = useTranslation();

  return (
    <Card>
      <CardHeader>
        <CardTitle>{t("eventsPage.detail.title")}</CardTitle>
      </CardHeader>
      <CardContent className="text-sm text-muted-foreground">
        {t("eventsPage.detail.description")}
      </CardContent>
    </Card>
  );
}
