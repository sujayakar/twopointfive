// CO-OP GAMES, the control plane's table: a game is a code two players share, a setup record, and one game server somewhere (runners.ts starts it and
// stops it; this file never knows where). A page makes one (`make`), another joins it by its code (`join`), both read it (`get`) until its server
// answers, and then talk to the server, not to this. The server tells this file how it is with a heartbeat (http.ts → `heard`).
// TWO BOUNDS ARE HELD HERE, and neither trusts a page or a server:
//   the cap   `make` counts the live games of a runner and writes the new one IN ONE MUTATION, so two pages asking at once cannot both take the last place
//   the end   `reap` (crons.ts, twice a minute) ends a game whose server never answered, went silent, has had nobody connected, or is past its lifetime.
//             A server ends itself by the same clocks (it is handed them); this is what holds when it does not.
// A game that is over is never live again. Its row stays.
import { v } from "convex/values";
import { internal } from "./_generated/api";
import type { Doc } from "./_generated/dataModel";
import { internalMutation, internalQuery, mutation, query, type MutationCtx, type QueryCtx } from "./_generated/server";
import { LIMITS, bad, count, pidOk, plain, word } from "./limits";

export const MP = {
  /** seats a game may have */
  seatsMax: 2,
  /** a setup record and an outcome, as JSON text */
  setupMax: 16 * 1024,
  outcomeMax: 16 * 1024,
  /** a code: this many of these letters (no vowels: it spells nothing; no digits: a menu shows it) */
  codeLen: 5,
  codeLetters: "BCDFGHJKLMNPQRSTVWXZ",
  codeTries: 8,
  tickMax: 2 ** 31,
};
const LIVE = ["starting", "waiting", "playing"] as const;
type Game = Doc<"games">;
const isLive = (g: Game): boolean => (LIVE as readonly string[]).includes(g.state);

/** a word of the deployment's environment */
export const env = (k: string): string | undefined => (globalThis as { process?: { env?: Record<string, string | undefined> } }).process?.env?.[k];
const seconds = (k: string, d: number): number => {
  const n = Number(env(k));
  return Number.isFinite(n) && n > 0 ? n : d;
};
/** THE CLOCKS, in seconds (a deployment's environment may set any of them): a server asked for must be heard from within `start`; a server not heard from
 *  for `silent` is gone; a server nobody has been connected to for `idle`, or older than `max`, ends itself, and is ended here one `silent` later if it
 *  did not */
export const policy = () => ({
  start: seconds("MP_START_S", 120),
  silent: seconds("MP_SILENT_S", 45),
  idle: seconds("MP_IDLE_S", 60),
  max: seconds("MP_MAX_S", 3600),
});
/** (the check's red paths, tools/qa/mp-control-check.ts: MPCAP_BREAK=1 = a game is made without counting; MPREAP_BREAK=1 = the reaper does nothing.
 *  Dead words on the service: it names its own address in every function's environment, and a word of a deployment's environment breaks nothing) */
const broken = (k: string): boolean => !env("CONVEX_CLOUD_URL") && env(k) === "1";

const dice = (n: number): Uint8Array => crypto.getRandomValues(new Uint8Array(n));
const key = (): string => Array.from(dice(24), b => b.toString(16).padStart(2, "0")).join("");
const newCode = (): string => {
  const L = MP.codeLetters, fair = 256 - (256 % L.length);
  let out = "";
  while (out.length < MP.codeLen) for (const b of dice(MP.codeLen * 2)) if (b < fair && out.length < MP.codeLen) out += L[b % L.length];
  return out;
};

/** the live games, a runner's or all of them */
export async function live(ctx: QueryCtx, runner?: string): Promise<Game[]> {
  const out: Game[] = [];
  for (const state of LIVE)
    out.push(
      ...(await ctx.db
        .query("games")
        .withIndex("by_state", q => (runner === undefined ? q.eq("state", state) : q.eq("state", state).eq("runner", runner)))
        .collect()),
    );
  return out;
}
/** the game a code names: the live one, else the one made last */
async function byCode(ctx: QueryCtx, code: string): Promise<Game | null> {
  if (code.length !== MP.codeLen) return null;
  const rows = await ctx.db.query("games").withIndex("by_code", q => q.eq("code", code)).collect();
  return rows.find(isLive) ?? rows.sort((a, b) => b.createdAt - a.createdAt)[0] ?? null;
}
/** what a page is told of a game. Only who holds a seat is told that seat's key and where the server is */
const view = (g: Game, pid: string) => {
  const seat = g.holders.indexOf(pid);
  return {
    code: g.code,
    state: g.state,
    why: g.why ?? "",
    build: g.build,
    seats: g.holders.map(h => h !== ""),
    connected: g.connected,
    seat,
    seatKey: seat < 0 ? "" : g.seatKeys[seat],
    address: seat < 0 ? "" : g.address,
    outcome: g.outcome ?? null,
  };
};
export type GameView = ReturnType<typeof view>;

/** a runner may be carrying more than the live games now: count it up, and sweep it (runners.ts `sweep` stops what no live game names) unless the
 *  caller asks for the sweep itself */
export async function unsettle(ctx: MutationCtx, runner: string, sweepNow = true): Promise<void> {
  const r = await ctx.db.query("runners").withIndex("by_name", q => q.eq("name", runner)).unique();
  if (!r) return;
  await ctx.db.patch(r._id, { dirty: r.dirty + 1 });
  if (sweepNow) await ctx.scheduler.runAfter(0, internal.runners.sweep, { name: runner });
}
/** THE ONE WAY A GAME ENDS: its row says so and why, and its runner is swept */
async function retire(ctx: MutationCtx, g: Game, state: "ended" | "failed", why: string, more: { outcome?: unknown; detail?: string; sweepNow?: boolean } = {}): Promise<void> {
  await ctx.db.patch(g._id, {
    state, why, endedAt: Date.now(),
    ...(more.outcome === undefined ? {} : { outcome: more.outcome }),
    ...(more.detail === undefined ? {} : { detail: more.detail }),
  });
  await unsettle(ctx, g.runner, more.sweepNow ?? true);
}

/** MAKE A GAME: the host's page hands over the setup record; a server is asked for; the code and the host's seat come back. Refused with `full: …` when
 *  every runner of this build carries its cap, with `build: …` when no runner runs this build (the page is older or newer than the servers: load it again) */
export const make = mutation({
  args: {
    pid: v.string(),
    build: v.string(),
    setup: v.object({ scenario: v.string(), seed: v.number(), knobs: v.any(), seats: v.array(v.any()) }),
  },
  handler: async (ctx, { pid, build, setup }): Promise<GameView> => {
    pidOk(pid);
    word(build, "build");
    word(setup.scenario, "scenario");
    if (!Number.isFinite(setup.seed)) bad("seed: not a number");
    count(setup.seats.length, "seats", MP.seatsMax, 1);
    plain(setup.knobs, "knobs");
    plain(setup.seats, "seats");
    if (JSON.stringify(setup).length > MP.setupMax) bad(`setup: longer than ${MP.setupMax}`);

    const runners = (await ctx.db.query("runners").collect()).filter(r => r.builds.includes(build));
    if (!runners.length) bad("build: no game server runs this build");
    // (THE CAP: the count and the insert below are one transaction, so what is read here is what stands when the game is written. A place is held by
    // every live game, and by every game over whose server no sweep has yet seen gone: `dirty - swept`)
    let runner: Doc<"runners"> | null = null;
    for (const r of runners.sort((a, b) => (a.name < b.name ? -1 : 1)))
      if (broken("MPCAP_BREAK") || (await live(ctx, r.name)).length + (r.dirty - r.swept) < r.cap) {
        runner = r;
        break;
      }
    if (!runner) return bad("full: every game server is taken");

    let code = "";
    for (let i = 0; i < MP.codeTries && !code; i++) {
      const c = newCode();
      if (!(await ctx.db.query("games").withIndex("by_code", q => q.eq("code", c)).collect()).some(isLive)) code = c;
    }
    if (!code) bad("code: none free");
    const now = Date.now(), n = setup.seats.length;
    const id = await ctx.db.insert("games", {
      code, build, runner: runner.name, setup,
      seatKeys: Array.from({ length: n }, key), secret: key(),
      state: "starting", address: "", server: "",
      holders: Array.from({ length: n }, (_, i) => (i === 0 ? pid : "")),
      connected: Array.from({ length: n }, () => false), tick: 0,
      createdAt: now, heardAt: now, seatedAt: now,
    });
    await ctx.scheduler.runAfter(0, internal.runners.start, { game: id });
    return view((await ctx.db.get(id))!, pid);
  },
});

/** JOIN BY CODE: the first free seat is this browser's (a browser that holds one already gets that one back: a page loaded again is the same player).
 *  Refused with `code: …` (no live game has it), `build: …` (the page is another build than the game), `full: …` (every seat is held) */
export const join = mutation({
  args: { pid: v.string(), code: v.string(), build: v.string() },
  handler: async (ctx, { pid, code, build }): Promise<GameView> => {
    pidOk(pid);
    word(build, "build");
    const g = await byCode(ctx, word(code, "code").toUpperCase());
    if (!g || !isLive(g)) return bad("code: no game has this code");
    if (g.build !== build) bad("build: the game is another build than this page");
    let seat = g.holders.indexOf(pid);
    if (seat < 0) {
      seat = g.holders.indexOf("");
      if (seat < 0) bad("full: every seat is taken");
      const holders = g.holders.slice();
      holders[seat] = pid;
      await ctx.db.patch(g._id, { holders });
      return view({ ...g, holders }, pid);
    }
    return view(g, pid);
  },
});

/** a game's state, by its code (a page that made or joined one reads this until its server answers, and after). null = no such game */
export const get = query({
  args: { pid: v.string(), code: v.string() },
  handler: async (ctx, { pid, code }): Promise<GameView | null> => {
    pidOk(pid);
    const g = await byCode(ctx, word(code, "code").toUpperCase());
    return g ? view(g, pid) : null;
  },
});

/** what the runner is asked to start, and the runner (runners.ts `start`). null = the game is not waiting for a server */
export const job = internalQuery({
  args: { game: v.id("games") },
  handler: async (ctx, { game }) => {
    const g = await ctx.db.get(game);
    if (!g || g.state !== "starting" || g.server) return null;
    const r = await ctx.db.query("runners").withIndex("by_name", q => q.eq("name", g.runner)).unique();
    if (!r) return null;
    const p = policy();
    return {
      runner: { url: r.url, key: r.key },
      body: {
        game: g._id as string,
        build: g.build,
        setup: JSON.stringify({ game: g._id, build: g.build, ...(g.setup as object), seatKeys: g.seatKeys }),
        secret: g.secret,
        idleS: p.idle,
        maxS: p.max,
      },
    };
  },
});

/** the runner started the game's server: where it answers. A game that ended meanwhile has a server nobody wants, and its runner is swept */
export const started = internalMutation({
  args: { game: v.id("games"), server: v.string(), address: v.string() },
  handler: async (ctx, { game, server, address }): Promise<void> => {
    const g = await ctx.db.get(game);
    if (!g) return;
    if (!isLive(g)) return await unsettle(ctx, g.runner);
    await ctx.db.patch(game, { server: word(server, "server", LIMITS.lineMax), address: word(address, "address", LIMITS.lineMax) });
  },
});

/** the runner could not start it. What it said is kept for a deploy key's read; a page is told only that the start failed. A game that was over
 *  already may still have got a server out of a start that failed half way: its runner is swept, as in `started` */
export const unstarted = internalMutation({
  args: { game: v.id("games"), why: v.string() },
  handler: async (ctx, { game, why }): Promise<void> => {
    const g = await ctx.db.get(game);
    if (!g) return;
    if (isLive(g)) await retire(ctx, g, "failed", "start failed", { detail: why.slice(0, LIMITS.lineMax) });
    else await unsettle(ctx, g.runner);
  },
});

/** A HEARTBEAT (http.ts): 'ok' · 'gone' = no live game has this id, and a server told so stops · 'no' = not this game's secret */
export const heard = internalMutation({
  args: { game: v.string(), secret: v.string(), state: v.string(), tick: v.number(), seats: v.array(v.boolean()), outcome: v.optional(v.any()) },
  handler: async (ctx, { game, secret, state, tick, seats, outcome }): Promise<"ok" | "gone" | "no"> => {
    const id = ctx.db.normalizeId("games", game);
    const g = id ? await ctx.db.get(id) : null;
    if (!g) return "gone";
    if (g.secret !== secret) return "no";
    if (!isLive(g)) return "gone";
    if (state !== "waiting" && state !== "playing" && state !== "ended") return bad("state: not a state a server is in");
    count(tick, "tick", MP.tickMax);
    if (seats.length !== g.holders.length) bad("seats: not this game's seats");
    if (outcome !== undefined) {
      plain(outcome, "outcome");
      if (JSON.stringify(outcome).length > MP.outcomeMax) bad(`outcome: longer than ${MP.outcomeMax}`);
    }
    const now = Date.now();
    // (the idle clock starts when the server is first heard from, and again whenever somebody is connected)
    const seatedAt = g.state === "starting" || seats.some(Boolean) ? now : g.seatedAt;
    if (state === "ended") {
      await ctx.db.patch(g._id, { tick, connected: seats, heardAt: now });
      await retire(ctx, g, "ended", "server", { outcome });
    } else await ctx.db.patch(g._id, { state, tick, connected: seats, heardAt: now, seatedAt });
    return "ok";
  },
});

/** THE REAPER (crons.ts): every live game is held to the clocks, and a runner whose last sweep did not finish is swept again */
export const reap = internalMutation({
  args: {},
  handler: async (ctx): Promise<{ live: number; ended: number }> => {
    const p = policy(), now = Date.now(), games = await live(ctx);
    if (broken("MPREAP_BREAK")) return { live: games.length, ended: 0 };
    const past = (at: number, s: number): boolean => now - at > s * 1000;
    let ended = 0;
    for (const g of games) {
      const why = g.state === "starting" ? (past(g.createdAt, p.start) ? "never started" : "")
        : past(g.heardAt, p.silent) ? "silent"
        : past(g.seatedAt, p.idle + p.silent) ? "idle"
        : past(g.createdAt, p.max + p.silent) ? "lifetime" : "";
      if (!why) continue;
      // (a server that stopped answering has failed; one that was stopped by a clock has ended. The sweeps are asked for below, one a runner)
      await retire(ctx, g, why === "idle" || why === "lifetime" ? "ended" : "failed", why, { sweepNow: false });
      ended++;
    }
    for (const r of await ctx.db.query("runners").collect())
      if (r.dirty !== r.swept) await ctx.scheduler.runAfter(0, internal.runners.sweep, { name: r.name });
    return { live: games.length - ended, ended };
  },
});

/** the ids of a runner's live games (runners.ts `sweep`: a server of the runner's that names none of these is stopped) */
export const wanted = internalQuery({
  args: { runner: v.string() },
  handler: async (ctx, { runner }): Promise<string[]> => (await live(ctx, runner)).map(g => g._id as string),
});

/** the games, newest first (a deploy key's read: `npx convex run games:list`): with what a runner said of a failed start, and no key and no secret */
export const list = internalQuery({
  args: { limit: v.optional(v.number()) },
  handler: async (ctx, { limit }) =>
    (await ctx.db.query("games").order("desc").take(Math.min(limit ?? 50, 500))).map(({ seatKeys: _k, secret: _s, ...g }) => g),
});
