import type { I18nResources } from "@/lib/i18n/types";

export const settingsI18n = {
  "zh-CN": {
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
        description: "由 Tauri command 提供的当前应用运行状态。",
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
        description: "当前应用连接的数据库位置。",
        inMemory: "内存数据库",
      },
    },
  },
  "en-US": {
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
        description: "Current application runtime state from Tauri commands.",
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
        description: "Database location used by the current application.",
        inMemory: "In-memory database",
      },
    },
  },
} satisfies I18nResources;
