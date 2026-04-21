import { useQuery } from "@tanstack/react-query";
import {
  Database,
  Languages,
  type LucideIcon,
  Monitor,
  Moon,
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
import { getAppSettings } from "@/lib/api";
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
  const settings = settingsQuery.data;

  return (
    <div className="grid gap-5 xl:grid-cols-[minmax(0,1fr)_24rem]">
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
