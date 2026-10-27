// FILES THE PAGE SAVED (src/cloud/files.ts): whatever a hosted build hands the player as a download (a benchmark's report, a replay, a recording) also
// goes into the deployment's file storage, with a row here that says whose it is. Three steps, as the service wants a file sent: `start` answers with a
// one-time address, the page POSTs the bytes there and is told the file's storage id, `put` writes the row. Written by the page, read back by a deploy key.
import { v } from "convex/values";
import { internalMutation, internalQuery, mutation } from "./_generated/server";
import { LIMITS, bad, count, pidOk, sidOk, word } from "./limits";

/** a file's name as a row keeps it: no folders, no odd characters */
const nameOk = (name: string): string =>
  name.length > 0 && name.length <= LIMITS.wordMax && /^[A-Za-z0-9][A-Za-z0-9._ ()+-]*$/.test(name) ? name : bad("name: not a file's name");

export const start = mutation({
  args: { pid: v.string(), sid: v.string(), name: v.string(), kind: v.string(), size: v.number() },
  handler: async (ctx, { pid, sid, name, kind, size }) => {
    pidOk(pid);
    sidOk(sid);
    nameOk(name);
    word(kind, "kind", 24);
    count(size, "size", LIMITS.fileMax, 1);
    const have = await ctx.db.query("files").withIndex("by_sid", q => q.eq("sid", sid)).take(LIMITS.filesMax + 1);
    if (have.length >= LIMITS.filesMax) bad(`files: a visit keeps ${LIMITS.filesMax}`);
    if (have.some(f => f.pid !== pid)) bad("sid: another browser's visit");
    return { url: await ctx.storage.generateUploadUrl() };
  },
});

export const put = mutation({
  args: {
    pid: v.string(),
    sid: v.string(),
    build: v.optional(v.string()),
    name: v.string(),
    kind: v.string(),
    type: v.string(),
    at: v.number(),
    storageId: v.id("_storage"),
  },
  handler: async (ctx, { pid, sid, build, name, kind, type, at, storageId }) => {
    pidOk(pid);
    sidOk(sid);
    nameOk(name);
    word(kind, "kind", 24);
    word(type, "type");
    if (build !== undefined) word(build, "build");
    if (!Number.isFinite(at)) bad("at: not a number");
    const meta = await ctx.db.system.get(storageId);
    if (!meta) return bad("storageId: no such file");
    // (the bytes are already here by the time their size can be read: a file past the bound is let go again)
    if (meta.size > LIMITS.fileMax || meta.size < 1) {
      await ctx.storage.delete(storageId);
      return bad(`size: not in 1..${LIMITS.fileMax}`);
    }
    if (await ctx.db.query("files").withIndex("by_storage", q => q.eq("storageId", storageId)).first()) bad("storageId: already a file");
    const have = await ctx.db.query("files").withIndex("by_sid", q => q.eq("sid", sid)).take(LIMITS.filesMax + 1);
    if (have.length >= LIMITS.filesMax || have.some(f => f.pid !== pid)) {
      await ctx.storage.delete(storageId);
      return bad(have.length >= LIMITS.filesMax ? `files: a visit keeps ${LIMITS.filesMax}` : "sid: another browser's visit");
    }
    await ctx.db.insert("files", { sid, pid, at, name, kind, type, size: meta.size, ...(build !== undefined ? { build } : {}), storageId });
    return { kept: true, size: meta.size };
  },
});

/** the files, newest first, each with an address its bytes can be fetched from (a deploy key's read); `since` = ms since 1970 */
export const list = internalQuery({
  args: { pid: v.optional(v.string()), kind: v.optional(v.string()), since: v.optional(v.number()), limit: v.optional(v.number()) },
  handler: async (ctx, { pid, kind, since, limit }) => {
    const n = Math.min(limit ?? 200, 1000), t0 = since ?? 0;
    const rows =
      pid !== undefined ? await ctx.db.query("files").withIndex("by_pid", q => q.eq("pid", pid).gte("at", t0)).order("desc").take(n)
      : kind !== undefined ? await ctx.db.query("files").withIndex("by_kind", q => q.eq("kind", kind).gte("at", t0)).order("desc").take(n)
      : (await ctx.db.query("files").order("desc").take(n)).filter(r => r.at >= t0);
    const out = [];
    for (const r of rows) if (kind === undefined || r.kind === kind) out.push({ ...r, url: await ctx.storage.getUrl(r.storageId) });
    return out;
  },
});

/** one file let go: its row and its bytes (a deploy key's) */
export const remove = internalMutation({
  args: { id: v.id("files") },
  handler: async (ctx, { id }) => {
    const row = await ctx.db.get(id);
    if (!row) return { removed: false };
    await ctx.storage.delete(row.storageId);
    await ctx.db.delete(id);
    return { removed: true };
  },
});
