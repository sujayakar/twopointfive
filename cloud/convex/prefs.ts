// A BROWSER'S SETTINGS, kept beside the browser's own copy (src/cloud/prefs.ts). Whoever holds a pid reads and writes that pid's row: the id is the
// only key there is (no accounts), which is why it is dice and why a link that carries one is to be handed over like a key.
import { v } from "convex/values";
import { internalQuery, mutation, query } from "./_generated/server";
import { LIMITS, bad, pidOk } from "./limits";

/** a page visit says hello: the browser's row is made or its last-seen moved */
export const hello = mutation({
  args: { pid: v.string() },
  handler: async (ctx, { pid }) => {
    pidOk(pid);
    const now = Date.now();
    const p = await ctx.db.query("players").withIndex("by_pid", q => q.eq("pid", pid)).unique();
    if (p) await ctx.db.patch(p._id, { lastAt: now, visits: p.visits + 1 });
    else await ctx.db.insert("players", { pid, firstAt: now, lastAt: now, visits: 1 });
    const kept = await ctx.db.query("prefs").withIndex("by_pid", q => q.eq("pid", pid)).unique();
    return { rev: kept?.rev ?? 0 };
  },
});

export const get = query({
  args: { pid: v.string() },
  handler: async (ctx, { pid }) => {
    pidOk(pid);
    const kept = await ctx.db.query("prefs").withIndex("by_pid", q => q.eq("pid", pid)).unique();
    return kept ? { rev: kept.rev, at: kept.at, data: kept.data } : null;
  },
});

/** the browser's settings as they stand now. `data` must be JSON of a flat map of texts; the new rev comes back */
export const put = mutation({
  args: { pid: v.string(), data: v.string() },
  handler: async (ctx, { pid, data }) => {
    pidOk(pid);
    if (data.length > LIMITS.prefsMax) bad(`prefs: longer than ${LIMITS.prefsMax}`);
    let map: unknown;
    try {
      map = JSON.parse(data);
    } catch {
      bad("prefs: not JSON");
    }
    if (!map || typeof map !== "object" || Array.isArray(map) || Object.values(map).some(x => typeof x !== "string"))
      bad("prefs: not a map of texts");
    const kept = await ctx.db.query("prefs").withIndex("by_pid", q => q.eq("pid", pid)).unique();
    const rev = (kept?.rev ?? 0) + 1;
    if (kept) await ctx.db.patch(kept._id, { rev, at: Date.now(), data });
    else await ctx.db.insert("prefs", { pid, rev, at: Date.now(), data });
    return { rev };
  },
});

/** the browsers seen, the last `limit` made, newest seen first (a deploy key's read) */
export const players = internalQuery({
  args: { limit: v.optional(v.number()) },
  handler: async (ctx, { limit }) => {
    // (newest made first, a bounded read: the table is one row a browser and anybody can say hello)
    const rows = await ctx.db.query("players").order("desc").take(Math.min(limit ?? 500, 2000));
    return rows.sort((a, b) => b.lastAt - a.lastAt);
  },
});
