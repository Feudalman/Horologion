export const supportedLanguages = ["zh-CN", "en-US"] as const;

export type SupportedLanguage = (typeof supportedLanguages)[number];

export type I18nResources = Record<SupportedLanguage, Record<string, unknown>>;
