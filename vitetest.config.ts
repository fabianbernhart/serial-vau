import { mergeConfig, defineConfig } from "vitest/config";

import { fileURLToPath } from "node:url";

export default defineConfig({
  test: {
    environment: "jsdom",
    root: fileURLToPath(new URL("./", import.meta.url)),
    server: {
      deps: {
        inline: ["vuetify"],
      },
    },
  },
});
