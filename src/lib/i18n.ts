import i18n from "i18next";
import { initReactI18next } from "react-i18next";

export const supportedLanguages = ["zh-CN", "en-US"] as const;
export type SupportedLanguage = (typeof supportedLanguages)[number];

const resources = {
  "zh-CN": {
    translation: {
      common: {
        active: "已启用",
        app: "应用",
        brand: "Horologion",
        collapseSidebar: "折叠侧边栏",
        databasePending: "数据库待连接",
        databaseReady: "数据库就绪",
        expandSidebar: "展开侧边栏",
        horologionSubtitle: "本地活动时间线",
        listenerReady: "监听器就绪",
        loading: "加载中",
        localDuckDB: "本地 DuckDB",
        noEventsYet: "暂无事件",
        type: "类型",
        value: "值",
        window: "窗口",
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
      events: {
        kind: {
          key_press: "按键按下",
          key_release: "按键释放",
          button_press: "鼠标按下",
          button_release: "鼠标释放",
          wheel: "滚轮",
        },
      },
      settings: {
        theme: {
          title: "主题模式",
          description: "在浅色、深色和跟随系统之间切换。",
          light: "浅色",
          lightDescription: "适合日间工作的明亮界面。",
          dark: "深色",
          darkDescription: "适合夜间使用的低眩光界面。",
          system: "跟随系统",
          systemDescription: "使用操作系统当前外观设置。",
        },
        language: {
          title: "语言",
          description: "当前仅支持中文和英文。",
          zh: "中文",
          zhDescription: "使用中文界面。",
          en: "English",
          enDescription: "Use the English interface.",
        },
        runtime: {
          title: "运行信息",
          description: "这些占位值后续会由 Tauri command 提供。",
          runMode: "运行模式",
          theme: "主题",
          version: "版本",
          mode: {
            development: "开发",
            production: "生产",
            test: "测试",
          },
          resolvedTheme: {
            light: "浅色",
            dark: "深色",
          },
        },
        database: {
          title: "数据库路径",
          description:
            "前端当前使用 mock 数据。这个字段后续会接入 `src-tauri`。",
        },
      },
    },
  },
  "en-US": {
    translation: {
      common: {
        active: "Active",
        app: "App",
        brand: "Horologion",
        collapseSidebar: "Collapse sidebar",
        databasePending: "Database pending",
        databaseReady: "Database ready",
        expandSidebar: "Expand sidebar",
        horologionSubtitle: "Local activity timeline",
        listenerReady: "Listener ready",
        loading: "Loading",
        localDuckDB: "Local DuckDB",
        noEventsYet: "No events yet",
        type: "Type",
        value: "Value",
        window: "Window",
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
      events: {
        kind: {
          key_press: "Key press",
          key_release: "Key release",
          button_press: "Button press",
          button_release: "Button release",
          wheel: "Wheel",
        },
      },
      settings: {
        theme: {
          title: "Theme mode",
          description: "Switch between light, dark, and system-controlled appearance.",
          light: "Light",
          lightDescription: "Bright interface for daytime work.",
          dark: "Dark",
          darkDescription: "Low-glare interface for evening sessions.",
          system: "System",
          systemDescription: "Follow the operating system appearance.",
        },
        language: {
          title: "Language",
          description: "Only Chinese and English are supported for now.",
          zh: "中文",
          zhDescription: "使用中文界面。",
          en: "English",
          enDescription: "Use the English interface.",
        },
        runtime: {
          title: "Runtime",
          description: "Placeholder values until the Tauri commands are connected.",
          runMode: "Run mode",
          theme: "Theme",
          version: "Version",
          mode: {
            development: "Development",
            production: "Production",
            test: "Test",
          },
          resolvedTheme: {
            light: "Light",
            dark: "Dark",
          },
        },
        database: {
          title: "Database path",
          description:
            "The frontend currently uses mock data. This field will be wired to `src-tauri` later.",
        },
      },
    },
  },
} as const;

const savedLanguage = localStorage.getItem("horologion-language");
const initialLanguage = supportedLanguages.includes(savedLanguage as SupportedLanguage)
  ? (savedLanguage as SupportedLanguage)
  : "zh-CN";

void i18n.use(initReactI18next).init({
  resources,
  lng: initialLanguage,
  fallbackLng: "zh-CN",
  interpolation: {
    escapeValue: false,
  },
});

document.documentElement.lang = initialLanguage;

i18n.on("languageChanged", (language) => {
  if (supportedLanguages.includes(language as SupportedLanguage)) {
    localStorage.setItem("horologion-language", language);
    document.documentElement.lang = language;
  }
});

export default i18n;
