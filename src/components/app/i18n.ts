import type { I18nResources } from "@/lib/i18n/types";

export const appShellI18n = {
  "zh-CN": {
    common: {
      horologionSubtitle: "本地活动时间线",
      listenerReady: "监听器就绪",
      localDuckDB: "本地 DuckDB",
    },
    nav: {
      events: "事件",
      overview: "概览",
      settings: "设置",
      windows: "窗口",
    },
    page: {
      eventDetail: {
        title: "事件详情",
        subtitle: "查看单条输入事件的完整上下文。",
      },
      events: {
        title: "事件",
        subtitle: "分页浏览输入事件，并按类型、应用和时间顺序筛选。",
      },
      overview: {
        title: "概览",
        subtitle: "查看活动捕获状态、最近事件和应用维度信号。",
      },
      settings: {
        title: "设置",
        subtitle: "配置主题、语言，并查看运行模式、版本和数据库信息。",
      },
      windowDetail: {
        title: "窗口详情",
        subtitle: "查看活动窗口上下文和关联事件。",
      },
      windows: {
        title: "窗口",
        subtitle: "按窗口上下文查看应用、标题、事件数量和最近活动。",
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
      events: "Events",
      overview: "Overview",
      settings: "Settings",
      windows: "Windows",
    },
    page: {
      eventDetail: {
        title: "Event Detail",
        subtitle: "Inspect the full context for one input event.",
      },
      events: {
        title: "Events",
        subtitle: "Browse input events with server-side paging, filters, and sorting.",
      },
      overview: {
        title: "Overview",
        subtitle: "Activity capture, recent events, and app-level signal.",
      },
      settings: {
        title: "Settings",
        subtitle: "Theme, language, runtime, version, and local database information.",
      },
      windowDetail: {
        title: "Window Detail",
        subtitle: "Inspect an active window context and its related events.",
      },
      windows: {
        title: "Windows",
        subtitle: "Review window contexts by app, title, event count, and recency.",
      },
    },
  },
} satisfies I18nResources;
