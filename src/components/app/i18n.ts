import type { I18nResources } from "@/lib/i18n/types";

export const appShellI18n = {
  "zh-CN": {
    common: {
      horologionSubtitle: "本地活动时间线",
      listenerReady: "监听器就绪",
      localDuckDB: "本地 DuckDB",
    },
    nav: {
      overview: "概览",
      settings: "设置",
    },
    page: {
      overview: {
        title: "概览",
        subtitle: "查看活动捕获状态、最近事件和应用维度信号。",
      },
      settings: {
        title: "设置",
        subtitle: "配置主题、语言，并查看运行模式、版本和数据库信息。",
      },
    },
  },
  "en-US": {
    common: {
      horologionSubtitle: "Local activity timeline",
      listenerReady: "Listener ready",
      localDuckDB: "Local DuckDB",
    },
    nav: {
      overview: "Overview",
      settings: "Settings",
    },
    page: {
      overview: {
        title: "Overview",
        subtitle: "Activity capture, recent events, and app-level signal.",
      },
      settings: {
        title: "Settings",
        subtitle: "Theme, language, runtime, version, and local database information.",
      },
    },
  },
} satisfies I18nResources;
