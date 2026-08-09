import { createSignal } from "solid-js";
import { site } from "../data/site";

type CopyStatus = "ready" | "copied" | "failed";

export default function CopyEmail() {
  const [status, setStatus] = createSignal<CopyStatus>("ready");
  let resetTimer: number | undefined;

  const copyEmail = async () => {
    window.clearTimeout(resetTimer);

    try {
      await navigator.clipboard.writeText(site.email);
      setStatus("copied");
    } catch {
      setStatus("failed");
    }

    resetTimer = window.setTimeout(() => setStatus("ready"), 1800);
  };

  const label = () => {
    if (status() === "copied") return "Email address copied";
    if (status() === "failed") return "Could not copy email address";
    return "Copy email address";
  };

  return (
    <button class="copy-email" type="button" aria-label={label()} onClick={copyEmail}>
      {status() === "copied" ? "Copied!" : status() === "failed" ? "Try again" : "Copy"}
    </button>
  );
}
