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
} satisfies I18nResources;
