// THE KEPT TAPES, on the server: a page hands over the pieces its browser already keeps (src/replay/keeper.ts files one every few seconds; src/cloud/tapes.ts
// sends what is not here yet). A piece is immutable and numbered, so sending one twice writes it once; the head is the one thing that is replaced (the last
// one stands, as in the browser's store). Reading a tape back out is internal: a deploy key's (tools/cloud/pull.ts lays a visit's chunks end to end into the
// same file the browser's own hand-over makes, which tools/qa/replay-dump.ts and the replay viewer already read).
import { v } from "convex/values";
import { internalQuery, mutation, query } from "./_generated/server";
import { LIMITS, amount, bad, count, pidOk, sidOk, word } from "./limits";

const chunk = v.object({
  /** -1 = the head */
  seq: v.number(),
  part: v.number(),
  parts: v.number(),
  at: v.string(),
  body: v.bytes(),
});

/** a visit's row as the browser has it, and some chunks of its tape. Comes back with how many pieces of the visit are here now */
export const put = mutation({
  args: {
    pid: v.string(),
    sid: v.string(),
    row: v.object({
      startedAt: v.string(),
      lastAt: v.string(),
      scenario: v.string(),
      url: v.string(),
      commit: v.optional(v.string()),
      seconds: v.number(),
      pieces: v.number(),
      bytes: v.number(),
    }),
    chunks: v.array(chunk),
  },
  handler: async (ctx, { pid, sid, row, chunks }) => {
    pidOk(pid);
    sidOk(sid);
    word(row.startedAt, "startedAt");
    word(row.lastAt, "lastAt");
    word(row.scenario, "scenario");
    word(row.url, "url", LIMITS.lineMax);
    if (row.commit !== undefined) word(row.commit, "commit");
    amount(row.seconds, "seconds");
    count(row.pieces, "pieces", LIMITS.seqMax);
    amount(row.bytes, "bytes");
    if (chunks.length > LIMITS.chunksMax) bad(`chunks: more than ${LIMITS.chunksMax}`);

    const visit = await ctx.db.query("visits").withIndex("by_sid", q => q.eq("sid", sid)).unique();
    // (a visit is its browser's: another pid may not write into it)
    if (visit && visit.pid !== pid) bad("sid: another browser's visit");
    let have = visit?.have ?? 0;
    for (const c of chunks) {
      count(c.seq, "seq", LIMITS.seqMax, -1);
      count(c.parts, "parts", LIMITS.partsMax, 1);
      count(c.part, "part", c.parts - 1);
      word(c.at, "at");
      if (c.body.byteLength > LIMITS.chunkMax) bad(`chunk: larger than ${LIMITS.chunkMax}`);
      const old = await ctx.db
        .query("tapeChunks")
        .withIndex("by_sid_seq", q => q.eq("sid", sid).eq("seq", c.seq).eq("part", c.part))
        .unique();
      if (c.seq < 0) {
        // the head: a newer one takes the older one's place, part by part (parts the newer one does not have go); an older one than the head that is
        // here writes nothing, whichever of its parts it is
        const cur = await ctx.db.query("tapeChunks").withIndex("by_sid_seq", q => q.eq("sid", sid).eq("seq", -1)).first();
        if (cur && cur.at > c.at) continue;
        if (old) await ctx.db.replace(old._id, { sid, ...c });
        else await ctx.db.insert("tapeChunks", { sid, ...c });
        if (c.part === c.parts - 1) {
          const rest = await ctx.db
            .query("tapeChunks")
            .withIndex("by_sid_seq", q => q.eq("sid", sid).eq("seq", -1).gte("part", c.parts))
            .collect();
          for (const r of rest) await ctx.db.delete(r._id);
        }
        continue;
      }
      if (old) continue;
      await ctx.db.insert("tapeChunks", { sid, ...c });
      // (a piece counts once its LAST part is here — the page sends a piece's parts in order — so a browser that lost its place and asks `have` begins
      // again at a piece that was only half sent)
      if (c.part === c.parts - 1) have++;
    }
    if (visit) await ctx.db.patch(visit._id, { ...row, have });
    else await ctx.db.insert("visits", { sid, pid, ...row, have });
    return { have };
  },
});

/** how many pieces of these visits are here (a page asks before it sends, so a browser whose own count was lost does not send a tape twice) */
export const have = query({
  args: { pid: v.string(), sids: v.array(v.string()) },
  handler: async (ctx, { pid, sids }) => {
    pidOk(pid);
    if (sids.length > LIMITS.chunksMax) bad(`sids: more than ${LIMITS.chunksMax}`);
    const out: Record<string, number> = {};
    for (const sid of sids) {
      const visit = await ctx.db.query("visits").withIndex("by_sid", q => q.eq("sid", sidOk(sid))).unique();
      if (visit && visit.pid === pid) out[sid] = visit.have;
    }
    return out;
  },
});

/** the visits kept, newest first (a deploy key's read). `since`: only those last written at or after this ISO moment; `pid`: one browser's */
export const list = internalQuery({
  args: { since: v.optional(v.string()), pid: v.optional(v.string()), limit: v.optional(v.number()) },
  handler: async (ctx, { since, pid, limit }) => {
    const n = Math.min(limit ?? 100, 1000);
    if (pid !== undefined)
      return await ctx.db
        .query("visits")
        .withIndex("by_pid", q => (since ? q.eq("pid", pid).gte("lastAt", since) : q.eq("pid", pid)))
        .order("desc")
        .take(n);
    return await ctx.db
      .query("visits")
      .withIndex("by_lastAt", q => (since ? q.gte("lastAt", since) : q))
      .order("desc")
      .take(n);
  },
});

/** a visit's chunks in order from (`seq`, `part`) on, at most `CHUNKS_A_READ` of them (a deploy key's read): the head is seq -1, so `seq: -1, part: 0`
 *  starts with it. `next` = where to ask from next, null when that was the last. Bounded by COUNT, not by pieces: a chunk is at most `chunkMax`, so one
 *  read stays inside what a function may read and return however large a piece somebody sent */
const CHUNKS_A_READ = 8;
export const chunks = internalQuery({
  args: { sid: v.string(), seq: v.number(), part: v.optional(v.number()) },
  handler: async (ctx, { sid, seq, part }) => {
    const got = await ctx.db
      .query("tapeChunks")
      .withIndex("by_sid_seq", q => q.eq("sid", sid).eq("seq", seq).gte("part", part ?? 0))
      .take(CHUNKS_A_READ);
    if (got.length < CHUNKS_A_READ)
      got.push(
        ...(await ctx.db
          .query("tapeChunks")
          .withIndex("by_sid_seq", q => q.eq("sid", sid).gt("seq", seq))
          .take(CHUNKS_A_READ - got.length)),
      );
    const last = got[got.length - 1];
    return {
      chunks: got.map(c => ({ seq: c.seq, part: c.part, parts: c.parts, at: c.at, body: c.body })),
      next: got.length === CHUNKS_A_READ ? { seq: last.seq, part: last.part + 1 } : null,
    };
  },
});
