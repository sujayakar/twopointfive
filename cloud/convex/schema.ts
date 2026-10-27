// THE HOSTED PAGE'S BACKEND: what a hosted build of the game keeps on a server (the page's side is src/cloud/). No accounts: a browser is known by a random
// id it made and keeps (`pid`), a page visit by the name its kept tape already has (`sid`, src/ui/replaykey.ts). Four things are kept:
//   players     one row a browser: when it was first and last seen
//   prefs       one row a browser: its settings as the page keeps them (the `twopointsix.*` keys of its localStorage, as one JSON text)
//   visits      one row a page visit that was played: the kept tape's own row (src/replay/keepstore.ts KeptRow) and how many of its pieces are here
//   tapeChunks  the kept tape itself: a visit's head (seq -1) and its pieces, each a gzip member as the browser packed it, cut where one is past a document
//   telemetry   small records of a visit: the machine, a run's outcome, a minute's frame times, an error
//   files       one row a file the page saved for the player (a benchmark's report, a replay, a recording): its bytes are in the deployment's file storage
// Everything a page may call is in prefs.ts, tapes.ts, telemetry.ts and files.ts (public: a page has no secret to show); reading it back out is internal, for a deploy
// key (tools/cloud/pull.ts, `npx convex run`, the dashboard).
import { defineSchema, defineTable } from "convex/server";
import { v } from "convex/values";

export default defineSchema({
  players: defineTable({
    pid: v.string(),
    firstAt: v.number(),
    lastAt: v.number(),
    /** page visits that said hello */
    visits: v.number(),
  }).index("by_pid", ["pid"]),

  prefs: defineTable({
    pid: v.string(),
    /** counts up with every save: a page that reads a higher one than it last saw has something to take */
    rev: v.number(),
    at: v.number(),
    /** `{ "<localStorage key>": "<value>" }` as JSON text (a key's value is whatever text the page keeps under it) */
    data: v.string(),
  }).index("by_pid", ["pid"]),

  visits: defineTable({
    sid: v.string(),
    pid: v.string(),
    startedAt: v.string(),
    lastAt: v.string(),
    scenario: v.string(),
    url: v.string(),
    commit: v.optional(v.string()),
    seconds: v.number(),
    /** pieces the browser says the visit has, and the bytes they take there */
    pieces: v.number(),
    bytes: v.number(),
    /** the next piece number: one past the highest piece that is whole here */
    have: v.number(),
  })
    .index("by_sid", ["sid"])
    .index("by_pid", ["pid", "lastAt"])
    .index("by_lastAt", ["lastAt"]),

  tapeChunks: defineTable({
    sid: v.string(),
    /** the piece's number within its visit; -1 = the visit's head (the last one sent stands) */
    seq: v.number(),
    /** a body past one document is cut: this is part `part` of `parts`, laid end to end */
    part: v.number(),
    parts: v.number(),
    at: v.string(),
    body: v.bytes(),
  }).index("by_sid_seq", ["sid", "seq", "part"]),

  telemetry: defineTable({
    sid: v.string(),
    pid: v.string(),
    /** the page's clock, ms since 1970 */
    at: v.number(),
    /** 'visit' · 'run' · 'perf' · 'error' · 'end' (src/cloud/telemetry.ts says what each carries) */
    kind: v.string(),
    build: v.optional(v.string()),
    scenario: v.optional(v.string()),
    data: v.any(),
  })
    .index("by_sid", ["sid", "at"])
    .index("by_kind", ["kind", "at"])
    .index("by_pid", ["pid", "at"]),

  files: defineTable({
    sid: v.string(),
    pid: v.string(),
    /** the page's clock when the file was saved, ms since 1970 */
    at: v.number(),
    /** the name the player's own copy has */
    name: v.string(),
    /** what the page called it: 'bench' · 'replay' · … (the door's callers say, src/host/savefile.ts) */
    kind: v.string(),
    /** its media type, and its bytes as the storage counted them */
    type: v.string(),
    size: v.number(),
    build: v.optional(v.string()),
    /** the few facts the page put beside it (a benchmark's report, a commit), as one JSON text */
    meta: v.optional(v.string()),
    storageId: v.id("_storage"),
  })
    .index("by_sid", ["sid", "at"])
    .index("by_kind", ["kind", "at"])
    .index("by_pid", ["pid", "at"])
    .index("by_storage", ["storageId"]),
});
