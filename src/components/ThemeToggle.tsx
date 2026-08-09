import { createSignal, onMount } from "solid-js";

type Theme = "light" | "dark";

function currentTheme(): Theme {
  if (typeof window === "undefined") return "dark";

  const explicitTheme = document.documentElement.dataset.theme;
  if (explicitTheme === "light" || explicitTheme === "dark") return explicitTheme;

  return window.matchMedia("(prefers-color-scheme: dark)").matches
    ? "dark"
    : "light";
}

export default function ThemeToggle() {
  const [theme, setTheme] = createSignal<Theme>("dark");

  onMount(() => setTheme(currentTheme()));

  const toggleTheme = () => {
    const nextTheme: Theme = theme() === "dark" ? "light" : "dark";
    document.documentElement.dataset.theme = nextTheme;
    window.localStorage.setItem("theme", nextTheme);
    setTheme(nextTheme);
  };

  return (
    <button
      class="theme-toggle"
      type="button"
      aria-label={`Switch to ${theme() === "dark" ? "light" : "dark"} theme`}
      aria-pressed={theme() === "dark"}
      onClick={toggleTheme}
    >
      <svg class="theme-toggle__icon theme-toggle__icon--sun" viewBox="0 0 24 24" aria-hidden="true">
        <circle cx="12" cy="12" r="4" />
        <path d="M12 2v2M12 20v2M4.93 4.93l1.41 1.41M17.66 17.66l1.41 1.41M2 12h2M20 12h2M4.93 19.07l1.41-1.41M17.66 6.34l1.41-1.41" />
      </svg>
      <svg class="theme-toggle__icon theme-toggle__icon--moon" viewBox="0 0 24 24" aria-hidden="true">
        <path d="M20.7 14.2A8 8 0 0 1 9.8 3.3 8 8 0 1 0 20.7 14.2Z" />
      </svg>
    </button>
  );
}
