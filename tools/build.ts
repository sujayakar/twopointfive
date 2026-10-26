// Static production build into ./dist (bundle + copy public/ + the level files). Two real pages — the game (index.html) and the WORKBENCH (workbench.html: level editor + character viewer in one
// shell) — plus the forwards that keep old links working: viewer.html → workbench.html#mode=chars, editor.html → workbench.html#mode=level (both plain pages, bundled as they are),
// gaitlab.html (written here) → the workbench's Characters ▸ Sam ▸ walk.
// A HOSTED BUILD'S TWO EXTRAS, both from the environment and both absent on a plain `bun run build`:
//   TPS_CLOUD_URL          the backend's address (cloud/, a Convex deployment), built into the page as `__TPS_CLOUD__` (src/cloud/cloud.ts): with none the page has no backend at all
//   VERCEL_GIT_COMMIT_SHA  (else `git rev-parse` of a clean checkout) → dist/qa/commit, the file the dev server answers /qa/commit with: a recording made on the build names it
import { $ } from "bun";
const cloud = (process.env.TPS_CLOUD_URL ?? "").trim();
if (cloud && !/^https:\/\/[\w.-]+(:\d+)?\/?$/.test(cloud)) { console.error(`build — TPS_CLOUD_URL is not an https address: ${cloud}`); process.exit(1); }
await $`rm -rf dist`;
const res = await Bun.build({ entrypoints: ["./index.html", "./workbench.html", "./viewer.html", "./editor.html", "./replay.html", "./sounds.html", "./music.html"], outdir: "./dist", minify: true, target: "browser", sourcemap: "linked", loader: { ".wgsl": "text" }, ...(cloud ? { define: { __TPS_CLOUD__: JSON.stringify(cloud) } } : {}) });
if (!res.success) { for (const m of res.logs) console.error(m); process.exit(1); }
await $`cp -R public/. dist/`;
await $`mkdir -p dist/levels`; await $`cp ${(await Array.fromAsync(new Bun.Glob("levels/*.json").scan())).sort()} dist/levels/`;   // (every mission's level file, fetched as /levels/<file>: without them a mission with a level of its own fell back to the office on a static host)
await Bun.write("dist/gaitlab.html", `<!doctype html><meta charset="utf-8"><title>twopointsix — gait lab → workbench</title>
<meta http-equiv="refresh" content="0; url=workbench.html#mode=chars&scene=sam/walk">
<script>location.replace('workbench.html' + location.search + '#mode=chars&scene=sam/walk')</script>
<p style="font:12px ui-monospace,monospace;color:#9cb;background:#000">The gait lab moved into the workbench: <a href="workbench.html#mode=chars&scene=sam/walk">workbench.html#mode=chars&amp;scene=sam/walk</a></p>
`);
const commit = await (async () => {
  const env = (process.env.VERCEL_GIT_COMMIT_SHA ?? "").trim(); if (/^[0-9a-f]{7,40}$/.test(env)) return env.slice(0, 12);
  try { if ((await $`git status --porcelain`.quiet().text()).trim()) return ""; return (await $`git rev-parse --short=12 HEAD`.quiet().text()).trim(); } catch { return ""; }   // (uncommitted changes are no commit's build)
})();
if (commit) await Bun.write("dist/qa/commit", commit);
console.log(`backend: ${cloud || "none (the page talks to nobody)"} · commit: ${commit || "none"}`);
console.log("built:", [...res.outputs.map(o => o.path), "dist/gaitlab.html (forward)"].join("\n"));
