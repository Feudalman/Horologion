import type { I18nResources } from "@/lib/i18n/types";

export const commonI18n = {
  "zh-CN": {
    common: {
      active: "已启用",
      app: "应用",
      brand: "Horologion",
      collapseSidebar: "折叠侧边栏",
      databasePending: "数据库待连接",
      databaseReady: "数据库就绪",
      expandSidebar: "展开侧边栏",
      loading: "加载中",
      noEventsYet: "暂无事件",
      type: "类型",
      value: "值",
      window: "窗口",
    },
    pagination: {
      go: "跳转",
      jumpTo: "跳至",
      next: "下一页",
      previous: "上一页",
      rowsPerPage: "每页",
      summary: "第 {{page}} / {{pages}} 页，共 {{total}} 条",
    },
    events: {
      kind: {
        key_press: "按键按下",
        key_release: "按键释放",
        button_press: "鼠标按下",
        button_release: "鼠标释放",
        wheel: "滚轮",
      },
    },
  },
  "en-US": {
    common: {
      active: "Active",
      app: "App",
      brand: "Horologion",
      collapseSidebar: "Collapse sidebar",
      databasePending: "Database pending",
      databaseReady: "Database ready",
      expandSidebar: "Expand sidebar",
      loading: "Loading",
      noEventsYet: "No events yet",
      type: "Type",
      value: "Value",
      window: "Window",
    },
    pagination: {
      go: "Go",
      jumpTo: "Jump to",
      next: "Next",
      previous: "Previous",
      rowsPerPage: "Rows",
      summary: "Page {{page}} of {{pages}}, {{total}} total",
    },
    events: {
      kind: {
        key_press: "Key press",
        key_release: "Key release",
        button_press: "Button press",
        button_release: "Button release",
        wheel: "Wheel",
      },
    },
  },
} satisfies I18nResources;
