import * as React from "react";
import {
  Activity,
  BarChart3,
  ChevronLeft,
  ChevronRight,
  Circle,
  Database,
  Keyboard,
  Menu,
  PanelsTopLeft,
  Settings,
} from "lucide-react";
import { useTranslation } from "react-i18next";
import {
  NavLink,
  Outlet,
  useLocation,
  useNavigate,
  useNavigationType,
} from "react-router-dom";

import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Separator } from "@/components/ui/separator";
import { cn } from "@/lib/utils";

const navItems = [
  {
    labelKey: "nav.overview",
    path: "/overview",
    icon: BarChart3,
  },
  {
    labelKey: "nav.events",
    path: "/events",
    icon: Keyboard,
  },
  {
    labelKey: "nav.windows",
    path: "/windows",
    icon: PanelsTopLeft,
  },
  {
    labelKey: "nav.settings",
    path: "/settings",
    icon: Settings,
  },
];

const pageTitles: Record<string, { titleKey: string }> = {
  "/overview": {
    titleKey: "page.overview.title",
  },
  "/events": {
    titleKey: "page.events.title",
  },
  "/events/:eventId": {
    titleKey: "page.eventDetail.title",
  },
  "/windows": {
    titleKey: "page.windows.title",
  },
  "/windows/:windowId": {
    titleKey: "page.windowDetail.title",
  },
  "/settings": {
    titleKey: "page.settings.title",
  },
};

export function AppShell() {
  const { t } = useTranslation();
  const [collapsed, setCollapsed] = React.useState(false);
  const location = useLocation();
  const page = getPageTitle(location.pathname);
  const isTablePage =
    location.pathname === "/events" || location.pathname === "/windows";
  const historyControls = useBrowserHistoryControls();

  return (
    <div className="flex h-screen flex-col overflow-hidden bg-background text-foreground">
      <header className="z-40 flex h-14 shrink-0 items-center border-b bg-background/95 px-3 backdrop-blur supports-[backdrop-filter]:bg-background/80 sm:px-4">
        <Button
          aria-label={
            collapsed ? t("common.expandSidebar") : t("common.collapseSidebar")
          }
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
            <div className="truncate text-sm font-semibold">
              {t("common.brand")}
            </div>
            <div className="hidden truncate text-xs text-muted-foreground sm:block">
              {t("common.horologionSubtitle")}
            </div>
          </div>
        </div>

        <div className="mr-2 flex items-center gap-1">
          <Button
            aria-label={t("common.goBack")}
            disabled={!historyControls.canGoBack}
            onClick={historyControls.goBack}
            size="icon"
            type="button"
            variant="ghost"
          >
            <ChevronLeft />
          </Button>
          <Button
            aria-label={t("common.goForward")}
            disabled={!historyControls.canGoForward}
            onClick={historyControls.goForward}
            size="icon"
            type="button"
            variant="ghost"
          >
            <ChevronRight />
          </Button>
        </div>

        <Badge className="hidden gap-1.5 sm:inline-flex" variant="success">
          <Circle className="size-2 fill-current" />
          {t("common.listenerReady")}
        </Badge>
      </header>

      <div className="flex min-h-0 flex-1 overflow-hidden">
        <aside
          className={cn(
            "h-full shrink-0 border-r bg-card transition-[width] duration-200",
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
                title={t(item.labelKey)}
              >
                <item.icon className="size-4 shrink-0" />
                <span
                  className={cn(
                    "truncate",
                    collapsed && "hidden",
                    "max-sm:hidden",
                  )}
                >
                  {t(item.labelKey)}
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
                  {t("common.localDuckDB")}
                </span>
              </div>
            </div>
          </nav>
        </aside>

        <main
          className={cn(
            "min-w-0 flex-1",
            isTablePage ? "overflow-hidden" : "overflow-auto",
          )}
        >
          <div
            className={cn(
              "mx-auto flex w-full max-w-7xl flex-col gap-5 p-4 sm:p-5 lg:p-6",
              isTablePage ? "h-full min-h-0 overflow-hidden" : "min-h-full",
            )}
          >
            <div className="flex min-w-0 shrink-0 flex-col gap-1">
              <h1 className="truncate text-2xl font-semibold tracking-normal">
                {t(page.titleKey)}
              </h1>
            </div>
            <div
              className={cn(
                "min-h-0 flex-1",
                isTablePage && "overflow-hidden",
              )}
            >
              <Outlet />
            </div>
          </div>
        </main>
      </div>
    </div>
  );
}

function getPageTitle(pathname: string) {
  if (pathname.startsWith("/events/")) {
    return pageTitles["/events/:eventId"];
  }

  if (pathname.startsWith("/windows/")) {
    return pageTitles["/windows/:windowId"];
  }

  return pageTitles[pathname] ?? pageTitles["/overview"];
}

function useBrowserHistoryControls() {
  const navigate = useNavigate();
  const navigationType = useNavigationType();
  const location = useLocation();
  const currentIndex = getHistoryIndex();
  const [maxIndex, setMaxIndex] = React.useState(currentIndex);

  React.useEffect(() => {
    const nextIndex = getHistoryIndex();

    setMaxIndex((value) => {
      if (navigationType === "PUSH") {
        return nextIndex;
      }

      return Math.max(value, nextIndex);
    });
  }, [location.key, navigationType]);

  return {
    canGoBack: currentIndex > 0,
    canGoForward: currentIndex < maxIndex,
    goBack: () => navigate(-1),
    goForward: () => navigate(1),
  };
}

function getHistoryIndex() {
  const state = window.history.state as { idx?: number } | null;
  return state?.idx ?? 0;
}
