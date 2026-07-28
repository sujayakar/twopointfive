import { defineConfig } from "vite";

export default defineConfig({
  server: {
    port: 5173,
    // Path tracing at high internal resolution benefits from a stable origin.
    host: "127.0.0.1",
  },
  build: {
    target: "esnext",
  },
  // .wgsl files are imported with ?raw and stitched by src/engine/shaders.ts
  assetsInclude: ["**/*.wgsl"],
});
