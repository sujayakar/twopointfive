// WHAT A PLAYER WROTE ON A REPLAY (src/cloud/notes.ts): the notes put on a kept visit in the viewer, and the saved windows of the F7 shelf with theirs.
// Each is one LINE of the kept tapes' hand-over format (src/replay/kept.ts: `{k:'notes'}`, `{k:'f7'}`), kept as the text the page made, under a key the
// page chose (`visit:<sid>`, `f7:<session>:<savedAt>`). The newest text under a key stands. Written by the page, read back by a deploy key.
import { v } from "convex/values";
import { internalQuery, mutation } from "./_generated/server";
import { LIMITS, bad, pidOk, word } from "./limits";

export const put = mutation({
  args: { pid: v.string(), items: v.array(v.object({ key: v.string(), at: v.number(), text: v.string() })) },
  handler: async (ctx, { pid, items }) => {
    pidOk(pid);
    if (items.length > LIMITS.notesMax) bad(`items: more than ${LIMITS.notesMax}`);
    let made = 0;
    for (const it of items) {
      word(it.key, "key");
      if (!/^(visit|f7):[A-Za-z0-9_.:+-]*$/.test(it.key)) bad("key: not a note's key");
      if (!Number.isFinite(it.at)) bad("at: not a number");
      word(it.text, "text", LIMITS.noteMax);
      let line: unknown;
      try {
        line = JSON.parse(it.text);
      } catch {
        bad("text: not JSON");
      }
      const k = (line as { k?: unknown } | null)?.k;
      if (k !== "notes" && k !== "f7") bad("text: not a notes line");
      const kept = await ctx.db.query("notes").withIndex("by_pid_key", q => q.eq("pid", pid).eq("key", it.key)).unique();
      if (kept) await ctx.db.patch(kept._id, { at: it.at, text: it.text });
      else {
        // (a browser keeps so many keys: the store it reads them from keeps fewer visits than this)
        if (made === 0 && (await ctx.db.query("notes").withIndex("by_pid_key", q => q.eq("pid", pid)).take(LIMITS.noteKeys + 1)).length > LIMITS.noteKeys) bad(`notes: a browser keeps ${LIMITS.noteKeys}`);
        await ctx.db.insert("notes", { pid, key: it.key, at: it.at, text: it.text });
        made++;
      }
    }
    return { kept: items.length };
  },
});

/** the lines, newest first (a deploy key's read): one browser's or all; `since` = ms since 1970 */
export const list = internalQuery({
  args: { pid: v.optional(v.string()), since: v.optional(v.number()), limit: v.optional(v.number()) },
  handler: async (ctx, { pid, since, limit }) => {
    const n = Math.min(limit ?? 500, 2000), t0 = since ?? 0;
    const rows = pid !== undefined ? await ctx.db.query("notes").withIndex("by_pid_key", q => q.eq("pid", pid)).take(2000) : await ctx.db.query("notes").order("desc").take(2000);
    return rows.filter(r => r.at >= t0).sort((a, b) => b.at - a.at).slice(0, n);
  },
});
