// WHERE GAME SERVERS RUN: a RUNNER is anything that answers four calls, and `call` below is the only place that makes them (THE SEAM).
//   start   { game, build, setup, control, owner, secret, idleS, maxS }  →  { id, address }     start one server for this game; `address` is ws:// or wss://
//   stop    { id }                                                       →  { ok }              stop it (stopping one that is gone is not an error)
//   list    { owner }                                                    →  { servers: [{ id, game, owner }] }   every server of this owner it carries now
//   ready   { build }                                                    →  { ok }              get ready to start servers of this build (what takes long, once)
// A runner says which BUILDS its servers can be, and a game is made only on a runner of the page's own build: server and clients are the same build.
// `control` is where the server sends its heartbeats (this deployment's /mp/heartbeat, http.ts). `owner` is WHOSE the server is, as one plain word (this
// deployment's own host name): a runner may carry servers of more than one deployment, and a deployment only ever stops its own.
// TWO RUNNERS are behind it, told apart by the runner's address and by nothing else:
//   http(s)://…    the calls go over HTTP (POST <url>/<call> with the body as JSON, `Authorization: Bearer <key>`). The dev server is one
//                  (tools/mp/runner.ts: a child process a game); it registers itself with the backend on its own machine.
//   modal:<app>    the calls go to Modal with this deployment's token (modal.ts: a sandbox a game). The hosted build's script registers it (`hosted`).
// THE SWEEP is how a server is stopped, and the only way: list what the runner carries, and stop every server of ours that no live game names. A game
// that ended, a game that ended while its server was still starting, a server whose start was never recorded: all are one case to it.
import { v } from "convex/values";
import { internal } from "./_generated/api";
import { internalAction, internalMutation, internalQuery, type ActionCtx, type MutationCtx } from "./_generated/server";
import { LIMITS, bad, count, word } from "./limits";
import { env, live, unsettle } from "./games";

const RUNNER = {
  /** servers one runner may be registered to carry */
  capMax: 64,
  /** the hosted runner's name, its Modal app, and how many servers it carries where nothing says */
  hosted: "modal",
  app: "twopointsix-mp",
  hostedCap: 4,
  /** the builds the hosted runner keeps serving: this deploy's and the ones before it (a page open across a deploy, a deploy rolled back) */
  buildsKept: 3,
  /** how long a call over HTTP may take, ms: a start waits for the server to answer, the others do not */
  startMs: 120_000,
  callMs: 30_000,
};
type Where = { url: string; key: string };
type Listed = { id: string; game: string; owner: string };
type What = "start" | "stop" | "list" | "ready";
const MODAL_AT = /^modal:([A-Za-z0-9][A-Za-z0-9_-]{0,62})$/;

const site = (): string => env("CONVEX_SITE_URL") ?? "";
/** where this deployment's servers send their heartbeats: the service names its own HTTP address in every function's environment */
const control = (): string => `${site()}/mp/heartbeat`;
/** whose a server is: this deployment's host name as one word */
const owner = (): string => site().replace(/^https?:\/\//, "").replace(/[^A-Za-z0-9.-]/g, "_").slice(0, 63);

/** THE SEAM: one call to a runner, whatever it is made of */
async function call<T>(ctx: ActionCtx, r: Where, what: What, body: Record<string, unknown>): Promise<T> {
  const modal = MODAL_AT.exec(r.url);
  if (modal) return (await ctx.runAction(internal.modal.call, { app: modal[1], what, body })) as T;
  const stop = new AbortController(), t = setTimeout(() => stop.abort(), what === "start" || what === "ready" ? RUNNER.startMs : RUNNER.callMs);
  let res: Response;
  try {
    res = await fetch(`${r.url}/${what}`, {
      method: "POST",
      headers: { Authorization: `Bearer ${r.key}`, "Content-Type": "application/json" },
      body: JSON.stringify(body),
      signal: stop.signal,
    });
  } finally {
    clearTimeout(t);
  }
  if (!res.ok) throw new Error(`${res.status} ${(await res.text()).slice(0, 160)}`);
  return (await res.json()) as T;
}
const said = (e: unknown): string => String((e as Error)?.message ?? e).slice(0, 200);

/** a runner's row is made or brought up to date. What its old address carried may be nobody's: swept. What its newest build needs is asked for now */
async function put(ctx: MutationCtx, r: { name: string; url: string; key: string; builds: string[]; cap: number }): Promise<void> {
  const row = { ...r, at: Date.now() };
  const old = await ctx.db.query("runners").withIndex("by_name", q => q.eq("name", r.name)).unique();
  if (old) await ctx.db.patch(old._id, row);
  else await ctx.db.insert("runners", { ...row, dirty: 0, swept: 0 });
  await unsettle(ctx, r.name);
  await ctx.scheduler.runAfter(0, internal.runners.ready, { name: r.name });
}
async function drop(ctx: MutationCtx, name: string): Promise<void> {
  const old = await ctx.db.query("runners").withIndex("by_name", q => q.eq("name", name)).unique();
  if (old) await ctx.db.delete(old._id);
}

/** a runner that answers over HTTP makes itself known, or says what it is now (its address, its key, its build, its cap) */
export const register = internalMutation({
  args: { name: v.string(), url: v.string(), key: v.string(), build: v.string(), cap: v.number() },
  handler: async (ctx, { name, url, key, build, cap }): Promise<void> => {
    if (!LIMITS.id.test(word(name, "name"))) bad("name: not an id");
    if (!/^https?:\/\/[^\s/]+(\/\S*)?$/.test(word(url, "url", LIMITS.lineMax))) bad("url: not an address");
    if (word(key, "key").length < LIMITS.pidMin) bad("key: too short");
    await put(ctx, { name, url: url.replace(/\/+$/, ""), key, builds: [word(build, "build")], cap: count(cap, "cap", RUNNER.capMax) });
  },
});

/** THE HOSTED BUILD'S RUNNER, said at every deploy (tools/host/build.sh, with the commit the page was built from): a deployment that has a Modal token
 *  runs this build's servers on Modal, and still those of the builds before it; `cap` of them at most (else `MP_CAP` of its environment, else 4; 0 =
 *  none). A deployment that has no token runs no servers. Answers which */
export const hosted = internalMutation({
  args: { build: v.string(), cap: v.optional(v.number()), app: v.optional(v.string()) },
  handler: async (ctx, { build, cap, app }): Promise<"modal" | "no Modal token"> => {
    if (!env("MODAL_TOKEN_ID") || !env("MODAL_TOKEN_SECRET")) {
      await drop(ctx, RUNNER.hosted);
      return "no Modal token";
    }
    const url = `modal:${app ?? RUNNER.app}`;
    if (!MODAL_AT.test(url)) bad("app: not a name");
    if (!/^[0-9a-f]{7,40}$/.test(build)) bad("build: not a commit");
    const old = await ctx.db.query("runners").withIndex("by_name", q => q.eq("name", RUNNER.hosted)).unique();
    const builds = [build, ...(old && old.url === url ? old.builds.filter(b => b !== build) : [])].slice(0, RUNNER.buildsKept);
    const word = env("MP_CAP"), n = cap ?? (word === undefined || word === "" ? RUNNER.hostedCap : Number(word));
    await put(ctx, { name: RUNNER.hosted, url, key: "", builds, cap: count(n, "cap", RUNNER.capMax) });
    return "modal";
  },
});

/** a runner is gone: no game is made on it from now on (its live games end by the clocks) */
export const unregister = internalMutation({
  args: { name: v.string() },
  handler: async (ctx, { name }): Promise<void> => await drop(ctx, name),
});

/** the runners and how full each is (a deploy key's read: `npx convex run runners:list`). No key is in it */
export const list = internalQuery({
  args: {},
  handler: async ctx => {
    const out = [];
    for (const { key: _k, ...r } of await ctx.db.query("runners").collect())
      out.push({ ...r, live: (await live(ctx, r.name)).length });
    return out;
  },
});

export const one = internalQuery({
  args: { name: v.string() },
  handler: async (ctx, { name }): Promise<(Where & { dirty: number; builds: string[] }) | null> => {
    const r = await ctx.db.query("runners").withIndex("by_name", q => q.eq("name", name)).unique();
    return r ? { url: r.url, key: r.key, dirty: r.dirty, builds: r.builds } : null;
  },
});

/** a runner that was just registered is asked to get ready for its newest build. One that cannot now does it at its first start instead */
export const ready = internalAction({
  args: { name: v.string() },
  handler: async (ctx, { name }): Promise<void> => {
    const r = await ctx.runQuery(internal.runners.one, { name });
    if (r?.builds.length) await call(ctx, r, "ready", { build: r.builds[0] }).catch(e => console.warn(`runner ${name} is not ready: ${said(e)}`));
  },
});

/** a sweep that began at `dirty` went through whole */
export const sweptTo = internalMutation({
  args: { name: v.string(), dirty: v.number() },
  handler: async (ctx, { name, dirty }): Promise<void> => {
    const r = await ctx.db.query("runners").withIndex("by_name", q => q.eq("name", name)).unique();
    if (r && r.swept < dirty) await ctx.db.patch(r._id, { swept: dirty });
  },
});

/** START A GAME'S SERVER (games.ts `make` schedules it): the runner is asked, and what it answers is written; a runner that cannot fails the game */
export const start = internalAction({
  args: { game: v.id("games") },
  handler: async (ctx, { game }): Promise<void> => {
    const job = await ctx.runQuery(internal.games.job, { game });
    if (!job) return;
    try {
      const r = await call<{ id?: unknown; address?: unknown }>(ctx, job.runner, "start", { ...job.body, control: control(), owner: owner() });
      if (typeof r.id !== "string" || !r.id || typeof r.address !== "string" || !/^wss?:\/\/\S+$/.test(r.address)) throw new Error("not a server's id and address");
      await ctx.runMutation(internal.games.started, { game, server: r.id, address: r.address });
    } catch (e) {
      // (a start that failed half way may have left a server behind: the game's end sweeps the runner)
      await ctx.runMutation(internal.games.unstarted, { game, why: said(e) });
    }
  },
});

/** THE SWEEP. The list is read BEFORE the live games: a game is live before its server is asked for and never live again once it is over, so a server
 *  listed now whose game is not live afterwards is one nobody wants. A sweep that could not finish leaves the runner unsettled, and the reaper asks again */
export const sweep = internalAction({
  args: { name: v.string() },
  handler: async (ctx, { name }): Promise<void> => {
    const r = await ctx.runQuery(internal.runners.one, { name });
    if (!r) return;
    const mine = owner();
    let servers: Listed[];
    try {
      servers = (await call<{ servers: Listed[] }>(ctx, r, "list", { owner: mine })).servers;
    } catch (e) {
      console.warn(`runner ${name} could not be swept (list): ${said(e)}`);
      return;
    }
    const live = new Set(await ctx.runQuery(internal.games.wanted, { runner: name }));
    let whole = true;
    for (const s of servers)
      if (s.owner === mine && !live.has(s.game))
        try {
          await call(ctx, r, "stop", { id: s.id });
        } catch (e) {
          whole = false;
          console.warn(`runner ${name} did not stop server ${s.id}: ${said(e)}`);
        }
    if (whole) await ctx.runMutation(internal.runners.sweptTo, { name, dirty: r.dirty });
  },
});
