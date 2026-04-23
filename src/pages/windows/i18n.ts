import type { I18nResources } from "@/lib/i18n/types";

export const windowsI18n = {
  "zh-CN": {
    windowsPage: {
      filters: {
        apply: "搜索",
        clear: "清空",
        search: "搜索应用名",
        searchPlaceholder: "搜索应用名",
        sort: "排序",
      },
      sort: {
        active: "最近活跃",
        earliest: "最早出现",
        mostEvents: "事件最多",
        appName: "应用名称",
      },
      table: {
        app: "应用",
        empty: "暂无窗口上下文",
        eventCount: "事件数",
        firstSeen: "首次出现",
        lastSeen: "最近出现",
        processId: "进程 ID",
        size: "尺寸",
        window: "窗口",
      },
      detail: {
        title: "窗口详情",
        contextHash: "上下文 Hash",
        notFound: "没有找到这个窗口上下文。",
        position: "位置",
        process: "进程信息",
        processPath: "进程路径",
        relatedEvents: "关联事件",
        windowId: "窗口 ID",
      },
    },
  },
  "en-US": {
    windowsPage: {
      filters: {
        apply: "Search",
        clear: "Clear",
        search: "Search app name",
        searchPlaceholder: "Search app name",
        sort: "Sort",
      },
      sort: {
        active: "Recently active",
        earliest: "First seen",
        mostEvents: "Most events",
        appName: "App name",
      },
      table: {
        app: "App",
        empty: "No window contexts",
        eventCount: "Events",
        firstSeen: "First seen",
        lastSeen: "Last seen",
        processId: "PID",
        size: "Size",
        window: "Window",
      },
      detail: {
        title: "Window detail",
        contextHash: "Context hash",
        notFound: "Window context not found.",
        position: "Position",
        process: "Process",
        processPath: "Process path",
        relatedEvents: "Related events",
        windowId: "Window ID",
      },
    },
  },
} satisfies I18nResources;
