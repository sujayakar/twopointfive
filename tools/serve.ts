// Minimal dev server: bundles index.html + workbench.html (+ viewer.html / editor.html, now forwards into it; TS, WGSL as text) via Bun's HTML imports, serves ./public statically,
// and answers the level editor's file API under /levels/ (tools/levels-api.ts: index, read, validated write into levels/ — loopback + same-origin only).
// Listens on LOOPBACK only (HOST=… overrides that on purpose): a dev server bound to 0.0.0.0 was reachable — its write API included — from the whole LAN.
import index from "../index.html";
import viewer from "../viewer.html";   // a FORWARD into /workbench#mode=chars (the character / prop viewer is its Characters mode) — see VIEWER.md
import editor from "../editor.html";   // a FORWARD into /workbench#mode=level (the level editor is its Level mode) — see docs/level-format.md 'Editor'
import replay from "../replay.html";   // the REPLAY VIEWER (src/replay/viewer.ts): a replay saved with F7 as a map with a scrubber and notes
import sounds from "../sounds.html";   // the LISTENING PAGE (src/sounds/main.ts): every sound the game makes, one row each, through the game's own mixer graph
import music from "../music.html";   // the SCORE'S LISTENING PAGE (src/music/main.ts): every piece of the score on one row, played by the game's own sequencer and voices
import workbench from "../workbench.html";   // the WORKBENCH: editor + viewer in one shell (src/workbench.ts → src/workbench/, src/editor/, src/viewer/)
import { handleLevels } from "./levels-api.ts";
import { localCloud } from "./cloud/local.ts";
import { devRunner, type DevRunner } from "./mp/runner.ts";
import { writePieces } from "./qa/capture-pieces.ts";

const COMMIT = (() => { try { const r = Bun.spawnSync(["git", "rev-parse", "--short=12", "HEAD"]); return r.success ? new TextDecoder().decode(r.stdout).trim() : ""; } catch { return ""; } })();   // (read once at start: a dev server outlives commits, but the journal only wants "about which build")
let CLOUD: ReturnType<typeof localCloud> = null;   // the local build's backend (tools/cloud/local.ts), started below once the port is ours: null on a plain start — `bun run dev` asks for one (TPS_LOCAL_CLOUD=1), TPS_DEV_CLOUD names one that is already up
let MP: DevRunner | null = null;   // co-op's game servers on this machine (tools/mp/runner.ts): the local backend asks this server to start and stop them. null where no backend was asked for
const port = Number(process.env.PORT ?? 5173);
const hostname = process.env.HOST ?? "127.0.0.1";
const server = Bun.serve({
  port, hostname, maxRequestBodySize: 512 << 20,   // (a frame capture of a whole 4K frame, gzipped: POST /capture below, which says who may send one)
  development: { hmr: false, console: true },
  routes: {
    "/": index, "/viewer": viewer, "/editor": editor, "/workbench": workbench, "/replay": replay, "/sounds": sounds, "/music": music,
    "/look": req => { const u = new URL(req.url); return Response.redirect("/?look&" + (u.searchParams.has("levelFile") ? "" : "roomclear&squad4&levelFile=pass.json&hud&") + u.search.slice(1), 302); },   // THE LOOK PROTOTYPE (src/render/shaders/look.wgsl): the game's page with ?look, on the pass unless the link names its own mission
    "/sneaklab": req => { const q = new URL(req.url).search.slice(1); return Response.redirect("/?levelFile=sneak-lab.json&solo&sneaklab" + (q ? "&" + q : ""), 302); },   // the sneak lab: the close-in on a man, its numbers live (src/host/sneaklab.ts)
    "/firelab": req => { const q = new URL(req.url).search.slice(1); return Response.redirect("/?levelFile=pass.json&roomclear&squad4&nofow&firelab" + (q ? "&" + q : ""), 302); },   // the fire lab: beside the pass's brazier, the flame's numbers live (src/host/firelab.ts)
    "/smokelab": req => { const q = new URL(req.url).search.slice(1), has = (k: string) => new RegExp("(^|&)" + k + "=").test(q); return Response.redirect((has("levelFile") ? "/?look&hud&smokelab" : "/?look&levelFile=smoke-lab.json&solo&hud&smokelab" + (has("hour") ? "" : "&hour=dusk") + (has("weather") ? "" : "&weather=clear")) + (q ? "&" + q : ""), 302); },   // the smoke lab: the look page on the lab's own floor (tools/levels/smoke-lab.ts) at a dry dusk, with the solver's bricks and curls drawn and its numbers live (src/host/smokelab.ts); an hour or a weather on the address wins, and a levelFile there opens that level instead (/smokelab?levelFile=pass.json&roomclear&squad4)
    "/gait": req => Response.redirect("/workbench" + new URL(req.url).search + "#mode=chars&scene=sam/walk", 302),   // the gait lab is the workbench's Characters ▸ Sam ▸ walk / jog / … now; old links land there
  },
  async fetch(req) {
    const url = new URL(req.url);
    let path: string; try { path = decodeURIComponent(url.pathname); } catch { return new Response("bad path encoding", { status: 400 }); }
    if (path.includes("..")) return new Response("bad path", { status: 400 });
    if (path === "/index.html") return Response.redirect("/" + url.search + url.hash, 302);   // the game is '/', but a built page's neighbour link (the editor's ▶ play: 'index.html?level=local') must open here too
    if (path === "/viewer.html" || path === "/viewer/") return Response.redirect("/viewer" + url.search, 302);   // one canonical URL per route (keep ?quality=…); /viewer and /editor themselves forward into /workbench in the page
    if (path === "/editor.html" || path === "/editor/") return Response.redirect("/editor" + url.search + url.hash, 302);
    if (path === "/replay.html" || path === "/replay/") return Response.redirect("/replay" + url.search, 302);
    if (path === "/sounds.html" || path === "/sounds/") return Response.redirect("/sounds" + url.search, 302);
    if (path === "/music.html" || path === "/music/") return Response.redirect("/music" + url.search, 302);
    if (path === "/workbench.html" || path === "/workbench/") return Response.redirect("/workbench" + url.search + url.hash, 302);
    if (path === "/gaitlab.html" || path === "/gait/") return Response.redirect("/workbench" + url.search + "#mode=chars&scene=sam/walk", 302);
    { const mp = await MP?.handle(req); if (mp) return mp; }   // /mp/runner/…: the backend's calls, by the runner's key
    if (path === "/capture" && req.method === "POST") {   // a frame capture from the page (src/render/framecap.ts, F6): one gzipped file under captures/ (gitignored); read with tools/qa/capture-read.ts
      // who is asking, as the level API asks (tools/levels-api.ts): a loopback Host, our own Origin, and a header no other site's page can send without a preflight this server never answers
      const host = (req.headers.get("host") ?? "").replace(/:\d+$/, ""), origin = req.headers.get("origin");
      if (!/^(localhost|127\.0\.0\.1|\[::1\])$/.test(host) || (origin !== null && origin !== url.origin) || req.headers.get("x-frame-capture") !== "1") return new Response("refused", { status: 403 });
      const name = url.searchParams.get("name") ?? ""; if (!/^cap-[\w-]{1,60}$/.test(name)) return new Response("bad name", { status: 400 });
      const to = Bun.file(`./captures/${name}.tpscap.gz`); if (await to.exists()) return new Response("there is one of that name", { status: 409 });
      const body = new Uint8Array(await req.arrayBuffer()); await Bun.write(to, body);
      // (a capture too big to hand on in one file is written a second time in pieces, tools/qa/capture-pieces.ts. The capture is saved by then: pieces that
      // could not be written, a full disk, are said on this console and cut later by hand; the page is still told it was captured)
      let pieces = 0; try { pieces = writePieces("./captures", name, body).length; } catch (e) { console.error(`capture: its pieces were not written (${e}); cut it with: bun run tools/qa/capture-pieces.ts captures/${name}.tpscap.gz`); }
      console.log(`capture: captures/${name}.tpscap.gz${pieces ? ` (and in ${pieces} pieces, ${name}.part1of${pieces}.tpscap.gz …)` : ''}  ·  bun run tools/qa/capture-read.ts captures/${name}.tpscap.gz`);
      return new Response(pieces ? `captured in ${pieces} pieces` : "captured");
    }
    const lv = await handleLevels(req); if (lv) return lv;
    if (path === "/qa/commit") return new Response(COMMIT, { headers: { "Cache-Control": "no-store", "Content-Type": "text/plain" } });   // the commit this server was started on (the session journal files it with a recording: src/ui/sessionrec.ts)
    if (path === "/qa/cloud") return new Response(CLOUD?.url() ?? "", { headers: { "Cache-Control": "no-store", "Content-Type": "text/plain" } });   // the backend this server runs for the page, or nothing (src/cloud/cloud.ts cloudFind asks once a page load)
    if (/^\/qa\/[\w.-]+\.js$/.test(path)) { const qa = Bun.file("./tools" + path); if (await qa.exists()) return new Response(qa, { headers: { "Cache-Control": "no-store" } }); }   // the in-page measurement harnesses (dev only, never bundled): `await import('/qa/perf-harness.js')` → window.__perf, '/qa/perf-frames.js' → window.__pf
    { const m = path.match(/^\/qa\/shader\/([a-z_0-9]+)\.wgsl$/); if (m) { const sf = Bun.file(`./src/render/shaders/${m[1]}.wgsl`); if (await sf.exists()) return new Response(sf, { headers: { "Cache-Control": "no-store", "Content-Type": "text/plain" } }); } }   // a pass shader's current source on disk, for the page's hot-swap hook (main.ts __hotVol): edit, fetch, recompile, A/B — without rebooting a staged scene
    const f = Bun.file("./public" + path);
    if (await f.exists()) return new Response(f);
    return new Response("not found: " + path, { status: 404 });
  },
});
// (after the port is ours: a server that could not start must not leave a backend running behind it)
CLOUD = localCloud();
if (CLOUD) MP = devRunner({ cloud: () => CLOUD?.url() ?? "", origin: `http://${hostname === "0.0.0.0" ? "127.0.0.1" : hostname}:${server.port}`, build: COMMIT });
if (CLOUD) for (const sig of ["SIGINT", "SIGTERM", "SIGHUP"] as const) process.on(sig, () => { MP?.stop(); CLOUD?.stop(); process.exit(0); });
console.log(`dev server: http://${hostname === "127.0.0.1" ? "localhost" : hostname}:${server.port}${hostname === "127.0.0.1" || hostname === "localhost" ? "" : "  (HOST override: NOT loopback-only)"}  ·  workbench: /workbench (level · materials · characters · effects; /viewer /editor /gait forward into it)  ·  level files: /levels/`);
