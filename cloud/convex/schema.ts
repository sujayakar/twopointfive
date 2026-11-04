// THE HOSTED PAGE'S BACKEND: what a hosted build of the game keeps on a server (the page's side is src/cloud/). No accounts: a browser is known by a random
// id it made and keeps (`pid`), a page visit by the name its kept tape already has (`sid`, src/ui/replaykey.ts). Four things are kept:
//   players     one row a browser: when it was first and last seen
//   prefs       one row a browser: its settings as the page keeps them (the `twopointsix.*` keys of its localStorage, as one JSON text)
//   visits      one row a page visit that was played: the kept tape's own row (src/replay/keepstore.ts KeptRow) and how many of its pieces are here
//   tapeChunks  the kept tape itself: a visit's head (seq -1) and its pieces, each a gzip member as the browser packed it, cut where one is past a document
//   telemetry   small records of a visit: the machine, a run's outcome, a minute's frame times, an error
//   files       one row a file the page saved for the player (a benchmark's report, a replay, a recording): its bytes are in the deployment's file storage
//   notes       what a player wrote on a replay: a line a kept visit's notes, a line a saved window of the F7 shelf
//   games       one row a co-op game: its code, its setup record, its state, where its server answers, who holds which seat (games.ts)
//   runners     one row a place game servers run: where it answers, how many it may carry, which build its servers are (runners.ts)
// Everything a page may call is in prefs.ts, tapes.ts, telemetry.ts, files.ts and notes.ts (public: a page has no secret to show); reading it back out is internal, for a deploy
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
    /** what the page called it: 'bench' · 'perf' · 'benchperf' · 'journal' · 'checkpoint' from the download door's callers (src/host/savefile.ts), 'replay' · 'f7' from the
     *  callers of `cloudFile` (src/cloud/cloud.ts: a replay window with no shelf to keep it, the engine replay's Send) */
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

  notes: defineTable({
    pid: v.string(),
    /** `visit:<sid>` = the notes on a kept visit · `f7:<session>:<savedAt>` = a saved window of the F7 shelf with its notes */
    key: v.string(),
    /** the page's clock when the line last went, ms since 1970 */
    at: v.number(),
    /** the line as the hand-over format has it (src/replay/kept.ts: `{k:'notes'}` · `{k:'f7'}`), as JSON text */
    text: v.string(),
  }).index("by_pid_key", ["pid", "key"]),

  games: defineTable({
    /** what a player types to join: letters only, unique among the games that are live */
    code: v.string(),
    build: v.string(),
    /** the runner that carries its server (runners.name) */
    runner: v.string(),
    /** the setup record as the host sent it: `{ scenario, seed, knobs, seats }` (the server is handed it with `game`, `build` and `seatKeys` added) */
    setup: v.any(),
    /** a seat proves itself to the server with its key; the server proves its heartbeats with `secret`. Neither is in any public answer but a seat's own key */
    seatKeys: v.array(v.string()),
    secret: v.string(),
    /** 'starting' (a server was asked for) · 'waiting' · 'playing' (the server's own words) · 'ended' · 'failed'. The first three are LIVE and count against the cap */
    state: v.union(v.literal("starting"), v.literal("waiting"), v.literal("playing"), v.literal("ended"), v.literal("failed")),
    /** why it ended or failed: 'server' (it said so itself) · 'idle' · 'lifetime' · 'silent' · 'never started' · 'start failed'; and for the last,
     *  what the runner said (never in a public answer) */
    why: v.optional(v.string()),
    detail: v.optional(v.string()),
    /** where its server answers (ws:// or wss://) and the runner's own id for it: '' until the runner has started it */
    address: v.string(),
    server: v.string(),
    /** who holds which seat: a pid, '' = free */
    holders: v.array(v.string()),
    /** the last heartbeat's words: which seats are connected, the tick, and at its end the outcome */
    connected: v.array(v.boolean()),
    tick: v.number(),
    outcome: v.optional(v.any()),
    /** ms since 1970: made, last heard from, last heard from with somebody connected (the idle clock), over */
    createdAt: v.number(),
    heardAt: v.number(),
    seatedAt: v.number(),
    endedAt: v.optional(v.number()),
  })
    .index("by_code", ["code"])
    .index("by_state", ["state", "runner"]),

  runners: defineTable({
    name: v.string(),
    /** where it answers `start` · `stop` · `list`, and the key a call to it must show */
    url: v.string(),
    key: v.string(),
    /** the builds its servers can be, the newest first: a game of any other build is not made here */
    builds: v.array(v.string()),
    /** how many servers it may carry at once */
    cap: v.number(),
    /** counts up whenever what it carries may be more than the live games (a game over, a new address); `swept` = the count the last whole sweep began
     *  at. The difference is how many servers may still run that no live game names, and they count against the cap until a sweep has gone through */
    dirty: v.number(),
    swept: v.number(),
    at: v.number(),
  }).index("by_name", ["name"]),
});
