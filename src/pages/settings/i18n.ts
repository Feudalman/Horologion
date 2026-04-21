import type { I18nResources } from "@/lib/i18n/types";

export const settingsI18n = {
  "zh-CN": {
    settings: {
      theme: {
        title: "主题模式",
        light: "浅色",
        lightDescription: "适合日间工作的明亮界面。",
        dark: "深色",
        darkDescription: "适合夜间使用的低眩光界面。",
        system: "跟随系统",
        systemDescription: "使用操作系统当前外观设置。",
      },
      language: {
        title: "语言",
        zh: "中文",
        en: "English",
      },
      listener: {
        title: "监听器",
        status: "当前状态",
        running: "正在监听",
        stopped: "已停用",
        start: "启动监听",
        stop: "停用监听",
      },
      runtime: {
        title: "运行信息",
        runMode: "运行模式",
        theme: "主题",
        version: "版本",
        databaseSize: "数据库大小",
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
        inMemory: "内存数据库",
      },
    },
  },
  "en-US": {
    settings: {
      theme: {
        title: "Theme mode",
        light: "Light",
        lightDescription: "Bright interface for daytime work.",
        dark: "Dark",
        darkDescription: "Low-glare interface for evening sessions.",
        system: "System",
        systemDescription: "Follow the operating system appearance.",
      },
      language: {
        title: "Language",
        zh: "中文",
        en: "English",
      },
      listener: {
        title: "Listener",
        status: "Current status",
        running: "Listening",
        stopped: "Stopped",
        start: "Start listener",
        stop: "Stop listener",
      },
      runtime: {
        title: "Runtime",
        runMode: "Run mode",
        theme: "Theme",
        version: "Version",
        databaseSize: "Database size",
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
        inMemory: "In-memory database",
      },
    },
  },
} satisfies I18nResources;
