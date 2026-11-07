// Static production build into ./dist (bundle + copy public/ + the level files): the game (index.html) and the tool pages (the replay viewer, sounds, music).
// A HOSTED BUILD'S TWO EXTRAS, both from the environment and both absent on a plain `bun run build`:
//   TPS_CLOUD_URL          the backend's address (cloud/, a Convex deployment), built into the page as `__TPS_CLOUD__` (src/cloud/cloud.ts): with none the page has no backend at all
//   VERCEL_GIT_COMMIT_SHA  (else `git rev-parse` of a clean checkout) → dist/qa/commit, the file the dev server answers /qa/commit with: a recording made on the build names it
import { $ } from "bun";
import pageBridge from "./blitz/page-plugin.ts";   // the page's way to the state bridge under ?rust=1 (tools/blitz/page-plugin.ts)
import { faceWasm } from "./blitz/face-build.ts";
const cloud = (process.env.TPS_CLOUD_URL ?? "").trim();
if (cloud && !/^https:\/\/[\w.-]+(:\d+)?\/?$/.test(cloud)) { console.error(`build — TPS_CLOUD_URL is not an https address: ${cloud}`); process.exit(1); }
await $`rm -rf dist`;
const res = await Bun.build({ entrypoints: ["./index.html", "./replay.html", "./sounds.html", "./music.html"], outdir: "./dist", minify: true, target: "browser", sourcemap: "linked", loader: { ".wgsl": "text" }, plugins: [pageBridge], ...(cloud ? { define: { __TPS_CLOUD__: JSON.stringify(cloud) } } : {}) });
if (!res.success) { for (const m of res.logs) console.error(m); process.exit(1); }
await $`cp -R public/. dist/`;
await $`mkdir -p dist/levels`; await $`cp ${(await Array.fromAsync(new Bun.Glob("levels/*.json").scan())).sort()} dist/levels/`;   // (every mission's level file, fetched as /levels/<file>: without them a mission with a level of its own fell back to the office on a static host)
{ const face = await faceWasm(l => console.log(l));   // THE RUST GAME'S MODULE (?rust=1): copied beside the page where it is built or can be; a build without it plays the TypeScript game, and says so under the word
  if ("path" in face) await $`cp ${face.path} dist/game_face.wasm`;
  else if (process.env.FACE_BUILD === "1") { console.error(`build — FACE_BUILD=1 and the Rust game's module could not be built on this machine: ${face.why}`); process.exit(1); }   // (a hosted build makes its own module or fails: it never goes out without one, and never with the checked-in one)
  else console.log(`game_face.wasm: not in this build (${face.why})`); }
await Bun.write("dist/look.html", `<!doctype html><meta charset="utf-8"><title>twopointsix — materials prototype</title>
<script>location.replace('index.html?look&' + (new URLSearchParams(location.search).has('levelFile') ? '' : 'roomclear&squad4&levelFile=pass.json&hud&') + location.search.slice(1))</script>`);   // THE LOOK PROTOTYPE: the game's page with ?look (tools/serve.ts /look)
const commit = await (async () => {
  const env = (process.env.VERCEL_GIT_COMMIT_SHA ?? "").trim(); if (/^[0-9a-f]{7,40}$/.test(env)) return env.slice(0, 12);
  try { if ((await $`git status --porcelain`.quiet().text()).trim()) return ""; return (await $`git rev-parse --short=12 HEAD`.quiet().text()).trim(); } catch { return ""; }   // (uncommitted changes are no commit's build)
})();
if (commit) await Bun.write("dist/qa/commit", commit);
await Bun.write("dist/qa/cloud", "");   // (a built page opened from this machine asks its server what backend it runs — src/cloud/cloud.ts cloudFind: a static build runs none)
console.log(`backend: ${cloud || "none (the page talks to nobody)"} · commit: ${commit || "none"}`);
console.log("built:", res.outputs.map(o => o.path).join("\n"));
