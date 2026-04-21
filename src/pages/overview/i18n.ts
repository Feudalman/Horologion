import type { I18nResources } from "@/lib/i18n/types";

export const overviewI18n = {
  "zh-CN": {
    overview: {
      metrics: {
        totalEvents: "总事件数",
        totalEventsCaption: "数据库中已捕获的事件",
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
        description: "从 Tauri 查询接口读取的最新输入记录。",
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
        totalEventsCaption: "Captured in the database",
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
        description: "Latest input records from the Tauri query interface.",
        time: "Time",
      },
      topApplications: {
        title: "Top applications",
        description: "Activity distribution by active window application.",
      },
    },
  },
} satisfies I18nResources;
