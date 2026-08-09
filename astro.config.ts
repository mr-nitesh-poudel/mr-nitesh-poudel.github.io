import { defineConfig } from "astro/config";
import solidJs from "@astrojs/solid-js";
import { site } from "./src/data/site";

export default defineConfig({
  site: site.url,
  output: "static",
  devToolbar: { enabled: false },
  integrations: [solidJs()],
});
