import type { I18nResources } from "@/lib/i18n/types";

export const eventsI18n = {
  "zh-CN": {
    eventsPage: {
      filters: {
        appName: "应用名",
        appNamePlaceholder: "精确匹配应用名",
        apply: "应用筛选",
        clear: "清空",
        kind: "事件类型",
        sort: "排序",
      },
      kind: {
        all: "全部类型",
      },
      sort: {
        newest: "最新事件",
        oldest: "最早事件",
        appName: "应用名称",
        kind: "事件类型",
        value: "事件值",
      },
      table: {
        app: "应用",
        collector: "采集器",
        empty: "暂无事件记录",
        time: "时间",
        title: "输入事件",
        type: "类型",
        value: "值",
        window: "窗口",
      },
      detail: {
        title: "事件详情",
        description: "事件详情页将在下一步接入。",
      },
    },
  },
  "en-US": {
    eventsPage: {
      filters: {
        appName: "App name",
        appNamePlaceholder: "Exact app name",
        apply: "Apply filters",
        clear: "Clear",
        kind: "Event type",
        sort: "Sort",
      },
      kind: {
        all: "All types",
      },
      sort: {
        newest: "Newest events",
        oldest: "Oldest events",
        appName: "App name",
        kind: "Event type",
        value: "Event value",
      },
      table: {
        app: "App",
        collector: "Collector",
        empty: "No input events",
        time: "Time",
        title: "Input events",
        type: "Type",
        value: "Value",
        window: "Window",
      },
      detail: {
        title: "Event detail",
        description: "The event detail page will be wired in next.",
      },
    },
  },
} satisfies I18nResources;
