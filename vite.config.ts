import { defineConfig } from "vite";

export default defineConfig({
  server: {
    port: 5173,
    // Path tracing at high internal resolution benefits from a stable origin.
    host: "127.0.0.1",
  },
  build: {
    target: "esnext",
    // Real pages rather than client-side routes: tools/headless/run.py serves
    // dist/ with a plain static handler and no history fallback, so a routed
    // /demo/smoke would 404 under the harness that has to be able to drive it.
    rollupOptions: {
      input: {
        main: "index.html",
        smoke: "demo/smoke.html",
        indirect: "demo/indirect.html",
        grenades: "demo/grenades.html",
        dynamics: "demo/dynamics.html",
      },
    },
  },
  // .wgsl files are imported with ?raw and stitched by src/engine/shaders.ts
  assetsInclude: ["**/*.wgsl"],
});
