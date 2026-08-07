// Minimal dev server: bundles index.html + viewer.html (+ TS, WGSL as text) via Bun's HTML imports, serves ./public statically.
import index from "../index.html";
import viewer from "../viewer.html";   // character / prop viewer (src/viewer.ts) — see VIEWER.md

const port = Number(process.env.PORT ?? 5173);
const server = Bun.serve({
  port,
  development: { hmr: false, console: true },
  routes: { "/": index, "/viewer": viewer },
  async fetch(req) {
    const url = new URL(req.url);
    const path = decodeURIComponent(url.pathname);
    if (path.includes("..")) return new Response("bad path", { status: 400 });
    if (path === "/viewer.html" || path === "/viewer/") return Response.redirect("/viewer" + url.search, 302);   // one canonical URL for the bundle route (keep ?quality=…)
    const f = Bun.file("./public" + path);
    if (await f.exists()) return new Response(f);
    return new Response("not found: " + path, { status: 404 });
  },
});
console.log(`dev server: http://localhost:${server.port}  ·  viewer: http://localhost:${server.port}/viewer`);
