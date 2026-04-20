import * as React from "react";

type Theme = "light" | "dark" | "system";

type ThemeProviderValue = {
  theme: Theme;
  resolvedTheme: "light" | "dark";
  setTheme: (theme: Theme) => void;
};

const ThemeProviderContext = React.createContext<ThemeProviderValue | null>(null);

function getSystemTheme() {
  return window.matchMedia("(prefers-color-scheme: dark)").matches
    ? "dark"
    : "light";
}

export function ThemeProvider({ children }: { children: React.ReactNode }) {
  const [theme, setThemeState] = React.useState<Theme>(() => {
    return (localStorage.getItem("horologion-theme") as Theme | null) ?? "system";
  });
  const [resolvedTheme, setResolvedTheme] = React.useState<"light" | "dark">(
    () => (theme === "system" ? getSystemTheme() : theme),
  );

  React.useEffect(() => {
    const media = window.matchMedia("(prefers-color-scheme: dark)");

    const applyTheme = () => {
      const nextTheme = theme === "system" ? getSystemTheme() : theme;
      setResolvedTheme(nextTheme);
      document.documentElement.classList.toggle("dark", nextTheme === "dark");
    };

    applyTheme();
    media.addEventListener("change", applyTheme);

    return () => media.removeEventListener("change", applyTheme);
  }, [theme]);

  const setTheme = React.useCallback((nextTheme: Theme) => {
    localStorage.setItem("horologion-theme", nextTheme);
    setThemeState(nextTheme);
  }, []);

  return (
    <ThemeProviderContext.Provider value={{ theme, resolvedTheme, setTheme }}>
      {children}
    </ThemeProviderContext.Provider>
  );
}

export function useTheme() {
  const context = React.useContext(ThemeProviderContext);

  if (!context) {
    throw new Error("useTheme must be used within ThemeProvider");
  }

  return context;
}

export type { Theme };
