import type { I18nResources } from "@/lib/i18n/types";

export const overviewI18n = {
  "zh-CN": {
    overview: {
      metrics: {
        totalEvents: "总事件数",
        totalEventsCaption: "当前本地样例中的捕获事件",
        keyboardEvents: "键盘事件",
        keyboardEventsCaption: "按下与释放信号",
        pointerEvents: "鼠标事件",
        pointerEventsCaption: "按钮与滚轮活动",
        lastEvent: "最近事件",
        listenerReceiving: "监听器正在接收事件",
        listenerWaiting: "等待监听器事件",
      },
      recentEvents: {
        title: "最近事件",
        description: "当前使用占位数据，后续会接入 Tauri 查询接口。",
        time: "时间",
      },
      topApplications: {
        title: "活跃应用",
        description: "按活动窗口所属应用统计的事件分布。",
      },
    },
  },
  "en-US": {
    overview: {
      metrics: {
        totalEvents: "Total events",
        totalEventsCaption: "Captured in the current local sample",
        keyboardEvents: "Keyboard events",
        keyboardEventsCaption: "Press and release signals",
        pointerEvents: "Pointer events",
        pointerEventsCaption: "Buttons and wheel activity",
        lastEvent: "Last event",
        listenerReceiving: "Listener is receiving events",
        listenerWaiting: "Waiting for listener",
      },
      recentEvents: {
        title: "Recent events",
        description: "Placeholder data for the upcoming Tauri query interface.",
        time: "Time",
      },
      topApplications: {
        title: "Top applications",
        description: "Activity distribution by active window application.",
      },
    },
  },
} satisfies I18nResources;
