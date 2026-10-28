// SMALL RECORDS OF A PAGE VISIT (src/cloud/telemetry.ts says what each kind carries): the machine it ran on, how a run ended, a minute's frame times,
// an error. Written by the page, read back by a deploy key.
import { v } from "convex/values";
import { internalQuery, mutation } from "./_generated/server";
import { LIMITS, bad, pidOk, plain, sidOk, word } from "./limits";

/** a value's text with every object's keys in order: two records that say the same read the same, however their keys were written */
const canon = (x: unknown): string => JSON.stringify(x, (_k, o) => o && typeof o === "object" && !Array.isArray(o) ? Object.fromEntries(Object.keys(o).sort().map(k => [k, o[k]])) : o);

export const put = mutation({
  args: {
    pid: v.string(),
    sid: v.string(),
    build: v.optional(v.string()),
    events: v.array(v.object({ at: v.number(), kind: v.string(), scenario: v.optional(v.string()), data: v.any() })),
  },
  handler: async (ctx, { pid, sid, build, events }) => {
    pidOk(pid);
    sidOk(sid);
    if (build !== undefined) word(build, "build");
    if (events.length > LIMITS.eventsMax) bad(`events: more than ${LIMITS.eventsMax}`);
    // (a visit is its browser's: where its tape is here under another id, records under its name are refused too)
    const visit = await ctx.db.query("visits").withIndex("by_sid", q => q.eq("sid", sid)).unique();
    if (visit && visit.pid !== pid) bad("sid: another browser's visit");
    let kept = 0;
    for (const e of events) {
      word(e.kind, "kind");
      if (e.scenario !== undefined) word(e.scenario, "scenario");
      if (!Number.isFinite(e.at)) bad("at: not a number");
      plain(e.data ?? null, "data");
      if (JSON.stringify(e.data ?? null).length > LIMITS.eventMax) bad(`data: larger than ${LIMITS.eventMax}`);
      // (THE SAME RECORD TWICE IS KEPT ONCE: a call that did not come back to a leaving page may still have got here — the browser finishes it without
      // the page — and the page, which cannot know, sends its records again. A real visit's last `end` and `perf` were here twice. Same visit, same moment
      // on the page's clock, same kind, same words = the record already held)
      const had = await ctx.db.query("telemetry").withIndex("by_sid", q => q.eq("sid", sid).eq("at", e.at)).collect(), text = canon(e.data ?? null);
      if (had.some(r => r.kind === e.kind && canon(r.data ?? null) === text)) continue;
      await ctx.db.insert("telemetry", { sid, pid, at: e.at, kind: e.kind, ...(build !== undefined ? { build } : {}), ...(e.scenario !== undefined ? { scenario: e.scenario } : {}), data: e.data ?? null });
      kept++;
    }
    return { kept };
  },
});

/** records, newest first (a deploy key's read): one visit's, one kind's, or all; `since` = ms since 1970 */
export const list = internalQuery({
  args: { sid: v.optional(v.string()), kind: v.optional(v.string()), since: v.optional(v.number()), limit: v.optional(v.number()) },
  handler: async (ctx, { sid, kind, since, limit }) => {
    const n = Math.min(limit ?? 200, 2000), t0 = since ?? 0;
    if (sid !== undefined) {
      const rows = await ctx.db.query("telemetry").withIndex("by_sid", q => q.eq("sid", sid).gte("at", t0)).order("desc").take(n);
      return kind === undefined ? rows : rows.filter(r => r.kind === kind);
    }
    if (kind !== undefined) return await ctx.db.query("telemetry").withIndex("by_kind", q => q.eq("kind", kind).gte("at", t0)).order("desc").take(n);
    const rows = await ctx.db.query("telemetry").order("desc").take(n);
    return rows.filter(r => r.at >= t0);
  },
});
