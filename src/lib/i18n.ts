import i18n from "i18next";
import { initReactI18next } from "react-i18next";

import { appShellI18n } from "@/components/app/i18n";
import { commonI18n } from "@/lib/i18n/common";
import {
  supportedLanguages,
  type I18nResources,
  type SupportedLanguage,
} from "@/lib/i18n/types";
import { eventsI18n } from "@/pages/events/i18n";
import { overviewI18n } from "@/pages/overview/i18n";
import { settingsI18n } from "@/pages/settings/i18n";
import { windowsI18n } from "@/pages/windows/i18n";

export { supportedLanguages, type SupportedLanguage };

const modules = [
  commonI18n,
  appShellI18n,
  overviewI18n,
  eventsI18n,
  windowsI18n,
  settingsI18n,
];

const resources = modules.reduce<I18nResources>((merged, moduleResources) => {
  for (const language of supportedLanguages) {
    merged[language] = deepMerge(merged[language], moduleResources[language]);
  }

  return merged;
}, emptyResources());

const savedLanguage = localStorage.getItem("horologion-language");
const initialLanguage = supportedLanguages.includes(savedLanguage as SupportedLanguage)
  ? (savedLanguage as SupportedLanguage)
  : "zh-CN";

void i18n.use(initReactI18next).init({
  resources: Object.fromEntries(
    supportedLanguages.map((language) => [
      language,
      {
        translation: resources[language],
      },
    ]),
  ),
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

function emptyResources(): I18nResources {
  return {
    "zh-CN": {},
    "en-US": {},
  };
}

function deepMerge(
  target: Record<string, unknown>,
  source: Record<string, unknown>,
): Record<string, unknown> {
  const output = { ...target };

  for (const [key, sourceValue] of Object.entries(source)) {
    const targetValue = output[key];

    if (isRecord(targetValue) && isRecord(sourceValue)) {
      output[key] = deepMerge(targetValue, sourceValue);
    } else {
      output[key] = sourceValue;
    }
  }

  return output;
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null && !Array.isArray(value);
}

export default i18n;
