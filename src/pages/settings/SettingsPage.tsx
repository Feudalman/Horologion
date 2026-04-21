import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import {
  Database,
  EarOff,
  Languages,
  type LucideIcon,
  Monitor,
  Moon,
  HardDrive,
  Radio,
  Server,
  Sun,
} from "lucide-react";
import { useTranslation } from "react-i18next";

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
import {
  getAppSettings,
  getAppStatus,
  getDatabaseFileSize,
  startListener,
  stopListener,
  type AppStatus,
} from "@/lib/api";
import { type SupportedLanguage } from "@/lib/i18n";
import { type Theme, useTheme } from "@/lib/theme";
import { cn } from "@/lib/utils";

const themeOptions: Array<{
  value: Theme;
  labelKey: string;
  descriptionKey: string;
  icon: LucideIcon;
}> = [
  {
    value: "light",
    labelKey: "settings.theme.light",
    descriptionKey: "settings.theme.lightDescription",
    icon: Sun,
  },
  {
    value: "dark",
    labelKey: "settings.theme.dark",
    descriptionKey: "settings.theme.darkDescription",
    icon: Moon,
  },
  {
    value: "system",
    labelKey: "settings.theme.system",
    descriptionKey: "settings.theme.systemDescription",
    icon: Monitor,
  },
];

const languageOptions: Array<{
  value: SupportedLanguage;
  labelKey: string;
  descriptionKey: string;
}> = [
  {
    value: "zh-CN",
    labelKey: "settings.language.zh",
    descriptionKey: "settings.language.zhDescription",
  },
  {
    value: "en-US",
    labelKey: "settings.language.en",
    descriptionKey: "settings.language.enDescription",
  },
];

export function SettingsPage() {
  const { i18n, t } = useTranslation();
  const { theme, resolvedTheme, setTheme } = useTheme();
  const settingsQuery = useQuery({
    queryKey: ["app-settings"],
    queryFn: getAppSettings,
  });
  const statusQuery = useQuery({
    queryKey: ["app-status"],
    queryFn: getAppStatus,
    refetchInterval: 5_000,
  });
  const databaseSizeQuery = useQuery({
    queryKey: ["database-file-size"],
    queryFn: getDatabaseFileSize,
    refetchInterval: 10_000,
  });
  const settings = settingsQuery.data;
  const status = statusQuery.data;
  const databaseSize = databaseSizeQuery.data;
  const startListenerMutation = useListenerMutation(startListener);
  const stopListenerMutation = useListenerMutation(stopListener);
  const listenerActionPending =
    startListenerMutation.isPending || stopListenerMutation.isPending;

  return (
    <div className="grid gap-5 pb-1 xl:grid-cols-[minmax(0,1fr)_24rem]">
      <Card>
        <CardHeader>
          <CardTitle>{t("settings.theme.title")}</CardTitle>
          <CardDescription>
            {t("settings.theme.description")}
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
                  {t(option.labelKey)}
                </span>
                {theme === option.value ? (
                  <Badge variant="success">{t("common.active")}</Badge>
                ) : null}
              </span>
              <span className="text-wrap text-sm font-normal text-muted-foreground">
                {t(option.descriptionKey)}
              </span>
            </Button>
          ))}
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle>{t("settings.language.title")}</CardTitle>
          <CardDescription>
            {t("settings.language.description")}
          </CardDescription>
        </CardHeader>
        <CardContent className="grid gap-3">
          {languageOptions.map((option) => (
            <Button
              className={cn(
                "h-auto justify-start gap-3 p-4 text-left",
                i18n.language === option.value &&
                  "border-primary bg-accent text-accent-foreground",
              )}
              key={option.value}
              onClick={() => void i18n.changeLanguage(option.value)}
              type="button"
              variant="outline"
            >
              <Languages className="size-4 shrink-0" />
              <span className="min-w-0 flex-1">
                <span className="block font-semibold">{t(option.labelKey)}</span>
                <span className="block text-wrap text-sm font-normal text-muted-foreground">
                  {t(option.descriptionKey)}
                </span>
              </span>
              {i18n.language === option.value ? (
                <Badge className="shrink-0" variant="success">
                  {t("common.active")}
                </Badge>
              ) : null}
            </Button>
          ))}
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle>{t("settings.runtime.title")}</CardTitle>
          <CardDescription>
            {t("settings.runtime.description")}
          </CardDescription>
        </CardHeader>
        <CardContent className="flex flex-col gap-4">
          <InfoRow
            icon={Server}
            label={t("settings.runtime.runMode")}
            value={
              settings?.runMode
                ? t(`settings.runtime.mode.${settings.runMode}`)
                : t("common.loading")
            }
          />
          <Separator />
          <InfoRow
            icon={Monitor}
            label={t("settings.runtime.theme")}
            value={`${t(`settings.theme.${theme}`)} (${t(
              `settings.runtime.resolvedTheme.${resolvedTheme}`,
            )})`}
          />
          <Separator />
          <InfoRow
            icon={Database}
            label={t("settings.runtime.version")}
            value={settings?.version ?? t("common.loading")}
          />
          <Separator />
          <InfoRow
            icon={HardDrive}
            label={t("settings.runtime.databaseSize")}
            value={
              databaseSize
                ? formatFileSize(
                    databaseSize.sizeBytes,
                    i18n.language,
                    t("settings.database.inMemory"),
                  )
                : t("common.loading")
            }
          />
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle>{t("settings.listener.title")}</CardTitle>
          <CardDescription>
            {t("settings.listener.description")}
          </CardDescription>
        </CardHeader>
        <CardContent className="flex flex-col gap-4">
          <InfoRow
            icon={status?.listenerRunning ? Radio : EarOff}
            label={t("settings.listener.status")}
            value={
              status
                ? status.listenerRunning
                  ? t("settings.listener.running")
                  : t("settings.listener.stopped")
                : t("common.loading")
            }
          />
          <div className="flex flex-wrap gap-2">
            <Button
              disabled={listenerActionPending || status?.listenerRunning === true}
              onClick={() => startListenerMutation.mutate()}
              type="button"
            >
              <Radio />
              {t("settings.listener.start")}
            </Button>
            <Button
              disabled={listenerActionPending || status?.listenerRunning === false}
              onClick={() => stopListenerMutation.mutate()}
              type="button"
              variant="outline"
            >
              <EarOff />
              {t("settings.listener.stop")}
            </Button>
          </div>
        </CardContent>
      </Card>

      <Card className="xl:col-span-2">
        <CardHeader>
          <CardTitle>{t("settings.database.title")}</CardTitle>
          <CardDescription>
            {t("settings.database.description")}
          </CardDescription>
        </CardHeader>
        <CardContent>
          <div className="rounded-md border bg-muted/40 p-3 font-mono text-sm text-muted-foreground">
            <div className="break-all">
              {settings
                ? settings.databasePath ?? t("settings.database.inMemory")
                : t("common.loading")}
            </div>
          </div>
        </CardContent>
      </Card>
    </div>
  );
}

function useListenerMutation(action: () => Promise<AppStatus>) {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: action,
    onSuccess: (nextStatus) => {
      queryClient.setQueryData(["app-status"], nextStatus);
    },
  });
}

function formatFileSize(
  bytes: number | null,
  locale: string,
  emptyText: string,
) {
  if (bytes === null) {
    return emptyText;
  }

  if (bytes < 1024) {
    return new Intl.NumberFormat(locale).format(bytes) + " B";
  }

  const units = ["KB", "MB", "GB", "TB"];
  let value = bytes / 1024;
  let unitIndex = 0;

  while (value >= 1024 && unitIndex < units.length - 1) {
    value /= 1024;
    unitIndex += 1;
  }

  return `${new Intl.NumberFormat(locale, {
    maximumFractionDigits: value >= 10 ? 1 : 2,
  }).format(value)} ${units[unitIndex]}`;
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
