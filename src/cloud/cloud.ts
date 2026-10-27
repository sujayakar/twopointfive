// THE HOSTED PAGE'S BACKEND, from the page's side. A build that was GIVEN A BACKEND'S ADDRESS (tools/build.ts builds one in where the environment names it —
// `TPS_CLOUD_URL`; the hosted build's script sets it, tools/host/build.sh) can keep three things there, each in a file of its own beside this one:
//   tapes.ts      the kept tape of every page visit (what this browser already keeps for the replay viewer)
//   telemetry.ts  small records of a visit: the machine, a run's outcome, a minute's frame times, an error
//   prefs.ts      the browser's settings, so another browser handed this one's id opens with them
//   files.ts      the files the page saves for the player (a benchmark's report, a replay, a recording), into the backend's file storage
//   notes.ts      what the player wrote on a replay: the notes on a kept visit, the F7 shelf's saved windows
// A build with no address — the dev server, a checkout, the one-file build, every headless tool — has none of this: `cloudBoot` returns null, no id is made,
// nothing is asked of any server. THE BUILD AS IT WAS.
// WHO THE BROWSER IS: a random id it makes once and keeps (localStorage `twopointsix.cloud.pid`). There are no accounts. A link that carries an id
// (`?player=<id>`, `__cloud.link()`) hands it to another browser, which takes the id and the settings kept under it before the game reads a single key,
// and drops the word from its address.
// WHETHER IT SENDS (`sharing`): this browser's own choice where it has made one (localStorage `twopointsix.cloud.share` = '1' | '0'; `__cloud.share = …`),
// else `CLOUD.shareDefault` (on). A browser that sends BY THE DEFAULT ALONE — it made no choice, its address says nothing — sends nothing until the title
// page's notice has been on its screen once (`twopointsix.cloud.told`; ui/cloudnote.ts draws the line and says so through `__cloud.told()`): a page opened
// straight into a mission shows no title, and sends nothing until one has been shown. For one visit: `?cloud=0` sends nothing, `?cloud=1` sends. Each kind has its own word too: ?cloudtapes=0 ?cloudstats=0 ?cloudprefs=0.
// WHICH TAPES GO: only a visit that was played START TO END while the browser sent its tapes (`twopointsix.cloud.visits`, the list of them). A visit opened
// under `?cloud=0`, with the switch off, or during which the switch was off for a moment is never on the list, so no later visit sends it — turning the
// switch on sends the visits from the next page load on. The switch is read live: another tab that turns it off stops this one too.
// A LOOPBACK ADDRESS on the address bar (`?cloud=http://127.0.0.1:3210`) points this visit at a backend on this machine and sends to it: the page checks' way
// in (a stand-in, cloud/test/standin.ts), and a developer's. No other address is taken from the address bar.
// A LOCAL BUILD HAS A LOCAL BACKEND (the maintainer, 10-27: "we can make the local build use a local convex deployment"): a page with no address built in,
// served from this machine, asks its own server once what backend it runs (`cloudFind`: GET /qa/cloud — tools/serve.ts answers with the deployment
// `bun run dev` started in cloud/, or with nothing). A backend found that way ON THIS MACHINE is the developer's own doing: it sends with no notice to
// wait for, and the title carries no line about it. A hosted address named that way is a backend like a built-in one, notice and all. The switch and
// every word above still turn it off.
// A PAGE THAT DIED (`cloudFatal`, the maintainer 10-27: "some way to get crash telemetry and metrics would be great"): one `fatal` record, from whatever
// puts the error card up. It works where the game never started — no adapter, no device, nothing booted: the visit is named here and its `visit` record
// is what `navigator` says — and on a visit that is up, where it goes out at once on the live records. Same switch, same notice: a browser that has not
// seen the notice is told `waits`, and the card that shows the notice's line sends the record by saying `told()`.
// No game code is imported (src/game is one import cycle): main.ts hands in the visit's name, the kept tapes' store and closures over its own objects.
import type { KeptStore } from '../replay/keepstore';
import { wire, type Wire } from './wire';
import { TapeSync, TAPESYNC, type TapeNote, type TapeNotes } from './tapes';
import { PrefSync, PREFSYNC, prefsNow, type KeyStore } from './prefs';
import { Telemetry, TELEMETRY, type PlayFacts, type GpuPasses } from './telemetry';
import { reportSink, reportHeld, reportError, tapConsole, REPORT, type Report } from './report';
import { tabLoad, LOADTIME } from './loadtime';
import { FileSync, FILESYNC } from './files';
import { LOOPBACK, HTTPS_ADDR, LOCAL_HOST, bare } from './addr';
import { NoteSync, NOTESYNC, type NoteMarks } from './notes';
import { shelfNoted, shelfGet } from '../replay/shelf';

/** the backend's address as the build was given it (tools/build.ts `define`); not defined at all on the dev server and under bun */
declare const __TPS_CLOUD__: string | undefined;

/** `shareDefault`: what a browser that has made no choice does — ON since 2026-10-26 (the maintainer's pick: on, with a notice), and such a browser sends
 *  nothing until the notice has been on its screen (`told`, below). `tapes` / `telemetry` / `prefs` / `files`: the four kinds (a visit's ?cloudtapes=0 · ?cloudstats=0 · ?cloudprefs=0 · ?cloudfiles=0 puts one off).
 *  `adoptMs`: how long the boot waits on the backend where it must (a browser handed an id; one that shares its id) before it goes on without */
export const CLOUD = { shareDefault: true, tapes: true, telemetry: true, prefs: true, files: true, adoptMs: 4000, gpuLag: true, gpuLagEvery: 4 };

const LS_NOTES = 'twopointsix.cloud.notes';
const LS_PID = 'twopointsix.cloud.pid', LS_SHARE = 'twopointsix.cloud.share', LS_TAPES = 'twopointsix.cloud.tapes', LS_LINKED = 'twopointsix.cloud.linked', LS_REV = 'twopointsix.cloud.rev', LS_SENT = 'twopointsix.cloud.sent', LS_VISITS = 'twopointsix.cloud.visits', LS_TOLD = 'twopointsix.cloud.told';
const PID = /^[a-z0-9]{24,64}$/;
const off = (v: string | null): boolean => v !== null && /^(0|off|false|no)$/i.test(v);
/** (the check's red path: CLOUD_BREAK=1 in front of tools/qa/cloud-check.ts — a browser that sends by the default does not wait for the notice; a page has no process) */
const BREAK = (() => { try { return String((globalThis as { process?: { env?: Record<string, string | undefined> } }).process?.env?.CLOUD_BREAK ?? '') === '1'; } catch { return false; } })();
const ls = (): KeyStore | null => { try { return typeof localStorage !== 'undefined' ? localStorage : null; } catch { return null; } };
const lsGet = (k: string): string | null => { try { return ls()?.getItem(k) ?? null; } catch { return null; } };
const lsSet = (k: string, v: string): void => { try { ls()?.setItem(k, v); } catch { /* nothing kept */ } };
/** THE VISITS THAT MAY GO UP: page visits that booted while this browser sent its tapes and have sent them ever since (the newest `VISITS_MAX`; a visit
 *  that falls off the list is simply not sent any more). A list of what MAY go, not of what may not: a visit this file never saw boot — `?cloud=0`, a
 *  replay being played — is on no list and so is never sent. */
const VISITS_MAX = 200;
const visitsNow = (): string[] => { try { const v = JSON.parse(lsGet(LS_VISITS) ?? '[]'); return Array.isArray(v) ? v.filter(x => typeof x === 'string') : []; } catch { return []; } };
const visitMay = (sid: string, on: boolean): void => { const had = visitsNow(), a = had.filter(s => s !== sid); if (on) a.push(sid); if (on ? had[had.length - 1] === sid : a.length === had.length) return; lsSet(LS_VISITS, JSON.stringify(a.slice(-VISITS_MAX))); };
/** the settings as one text, and a short mark of it (what was last sent is remembered by its mark and the backend's count, not by holding the text twice) */
const prefsText = (store: KeyStore): string => JSON.stringify(prefsNow(store));
const mark = (s: string): string => { let h = 0x811c9dc5; for (let i = 0; i < s.length; i++) { h ^= s.charCodeAt(i); h = Math.imul(h, 0x01000193); } return `${(h >>> 0).toString(36)}.${s.length}`; };

const builtIn = (): string => { try { return typeof __TPS_CLOUD__ === 'string' && /^https:\/\//.test(__TPS_CLOUD__) ? bare(__TPS_CLOUD__) : ''; } catch { return ''; } };
/** the backend this page's own server said it runs ('' = none, or not asked yet): `cloudFind` */
let found = '', asked: Promise<void> | null = null;
/** A LOCAL BUILD ASKS ITS SERVER FOR ITS BACKEND, once (main.ts awaits it through `cloudAdopt`; the replay viewer through `cloudNotes`). Only a page served
 *  from this machine with no address built in asks; a static build's `qa/cloud` is an empty file. Never rejects; a second and a half at most */
export function cloudFind(): Promise<void> {
  return asked ??= (async () => {
    try {
      if (builtIn() || typeof location === 'undefined' || !LOCAL_HOST.test(location.hostname)) return;
      const r = await fetch('/qa/cloud', { cache: 'no-store', signal: AbortSignal.timeout(1500) }), t = r.ok ? (await r.text()).trim() : '';
      if (LOOPBACK.test(t) || HTTPS_ADDR.test(t)) found = bare(t);
    } catch { /* no server that knows: no backend */ }
  })();
}
/** (a check's: a page asks once, a check many times) */
export function cloudFound(url: string | null): void { found = url ?? ''; asked = url === null ? null : Promise.resolve(); }
/** the backend this visit talks to ('' = none): a loopback address on the address bar, else the one the build was given, else the one this page's own
 *  server runs; none at all under ?cloud=0 */
export function cloudUrl(q: URLSearchParams): string {
  const word = q.get('cloud');
  if (off(word)) return '';
  if (word && LOOPBACK.test(word)) return bare(word);
  return builtIn() || found;
}
/** this visit's backend is the developer's own, ON THIS MACHINE, named by this page's server: nothing is sent to anybody else, so there is no notice to
 *  wait for. A hosted address the dev server names (`TPS_DEV_CLOUD=https://…`) is a backend like a built-in one: the notice first */
export const cloudLocal = (q: URLSearchParams): boolean => !!found && LOOPBACK.test(found) && cloudUrl(q) === found;
/** this visit sends: the address bar's word, else the browser's own choice, else the default */
export function cloudSharing(q: URLSearchParams): boolean {
  const word = q.get('cloud');
  if (off(word)) return false;
  if (word === '1' || word === 'on' || (word !== null && LOOPBACK.test(word))) return true;
  const kept = lsGet(LS_SHARE);
  return kept === '1' ? true : kept === '0' ? false : CLOUD.shareDefault;
}
/** the sending is this browser's OWN doing — its address says so, or its player chose it (the Options row, a settings link) — and not the default's */
export function cloudChosen(q: URLSearchParams): boolean {
  const word = q.get('cloud');
  return !off(word) && (word === '1' || word === 'on' || (word !== null && LOOPBACK.test(word)) || lsGet(LS_SHARE) === '1' || cloudLocal(q));
}
/** this browser's id, made the first time it is wanted */
export function cloudPid(): string {
  const kept = lsGet(LS_PID); if (kept && PID.test(kept)) return kept;
  const a = new Uint8Array(20); crypto.getRandomValues(a);
  let pid = ''; for (const b of a) pid += (b % 36).toString(36) + (b >> 4).toString(16);   // (40 characters off 160 bits of the browser's dice)
  lsSet(LS_PID, pid); return pid;
}

/** what a browser is asked when it opens another's link (the browser's own dialog: nothing of the page is up yet) */
export const ADOPT_ASK = "Use the settings from this link? They'll replace the settings in this browser, and this browser will share its settings, replays and stats with the developer. You can turn that off in Options.";

/** BEFORE THE GAME BOOTS (main.ts calls it first): a browser handed an id by its address takes the id and its settings; a browser that shares its id with
 *  another takes the settings where the other saved newer ones. Both end in one replace of the page — true = the page is going, boot nothing. Costs nothing
 *  (no request, no wait) for a build with no backend, a visit that does not send, and a browser that never linked. */
export async function cloudAdopt(q: URLSearchParams): Promise<boolean> {
  tabMark(q);   // (the tab remembers whether this visit's address put the records off: a page it leads to has no words of its own — `tabSaysOff`)
  await cloudFind();
  const url = cloudUrl(q), store = ls(); if (!url || !store) return false;
  const handed = q.get('player');
  const strip = (): string => { const p = new URLSearchParams(location.search); p.delete('player'); const s = p.toString(); return location.pathname + (s ? `?${s}` : '') + location.hash; };
  try {
    if (handed !== null) {
      if (!PID.test(handed) || handed === lsGet(LS_PID)) { history.replaceState(null, '', strip()); if (handed === lsGet(LS_PID)) lsSet(LS_LINKED, '1'); return false; }
      // (the browser is ASKED, whatever it has: taking an id replaces its settings and makes it a browser that sends — its settings could not follow it
      // otherwise — and a link is a thing somebody else can hand it. No: the word is dropped and the page boots as it was)
      if (!confirm(ADOPT_ASK)) { history.replaceState(null, '', strip()); return false; }
      lsSet(LS_PID, handed); lsSet(LS_LINKED, '1'); lsSet(LS_SHARE, '1'); try { store.removeItem(LS_TAPES); } catch { /* */ }
      const P = new PrefSync(wire(url, { timeoutMs: CLOUD.adoptMs }), store, { pid: handed });
      await P.take(); if (P.stats.rev) { lsSet(LS_REV, String(P.stats.rev)); lsSet(LS_SENT, mark(prefsText(store))); }
      location.replace(strip()); return true;
    }
    if (lsGet(LS_LINKED) !== '1' || !cloudSharing(q) || !(cloudChosen(q) || lsGet(LS_TOLD) === '1') || off(q.get('cloudprefs'))) return false;
    const P = new PrefSync(wire(url, { timeoutMs: CLOUD.adoptMs }), store, { pid: cloudPid() });
    const r = await P.get(); if (!r || r.rev <= (Number(lsGet(LS_REV)) || 0)) return false;
    const changed = await P.take(r); lsSet(LS_REV, String(r.rev)); lsSet(LS_SENT, mark(prefsText(store)));
    if (changed) { location.reload(); return true; }
  } catch { /* the backend did not answer: the page boots on what this browser has */ }
  return false;
}

export interface CloudDeps {
  /** this page visit's name (ui/replaykey.ts session: the kept tape's own) */
  sid: string;
  /** the kept tapes' store (null: this page keeps none) */
  store: Pick<KeptStore, 'rows' | 'head' | 'piecesFrom'> | null;
  build: () => string | undefined;
  machine: () => unknown;
  facts: () => PlayFacts;
  /** the GPU as the page got it (what it can do, its limits: asked once, for the visit's record) and the engine's per-pass times (telemetry.ts) */
  gpu?: () => unknown;
  /** the boot's timeline once the boot is over (ui/boot.ts, already taken): null = not yet. `last` = the page is going, say what there is */
  boot?: (last: boolean) => unknown;
  /** the moment the last program built behind the start card was in, as a small record, once it has come (loadtime.ts laterRecord): null = not yet */
  later?: () => unknown;
  passes?: GpuPasses;
  /** the render passes among them: listed apart in the record (telemetry.ts) */
  renderPasses?: readonly string[];
}
/** what became of a file the page saved, in the door's own words (host/savefile.ts reads 'sent' and 'held' as taken, anything else as not): it is on its
 *  way (`sent`: queued and going, not yet arrived — `done` and `filesIdle` say when); it is `held` until the notice has been read and goes then, with
 *  nothing else for the player to do; this browser does not send files (`off`: its switch, ?cloud=0, ?cloudfiles=0); or this file cannot go (`no`:
 *  empty, past the bound, a kind that stays the player's own) */
export type Saved = 'sent' | 'held' | 'off' | 'no';
const savedWord = (t: 'now' | 'held' | null): Saved => t === 'now' ? 'sent' : t ?? 'no';
/** A REPLAY IS A REPLAY WHICHEVER ROAD IT TAKES: a saved file of kind 'replay' or 'f7' goes only under the replays' own word too (`?cloudtapes`), and one
 *  that says it is a cut of a visit of this browser (`meta.session`) only while that visit's tape may go — a window saved out of a visit played with the
 *  sending off stays with it, as its tape and its notes do. A replay that names no visit (a file opened in the viewer and sent by a press) is not judged */
const replayMay = (q: URLSearchParams, kind: string, meta?: Record<string, string>): boolean =>
  !(kind === 'replay' || kind === 'f7') || (CLOUD.tapes && !off(q.get('cloudtapes')) && (!meta?.session || visitsNow().includes(meta.session)));
/** the GPU device's queue, the part asked (main.ts hands `gpu.device.queue`) */
export interface GpuQueue { onSubmittedWorkDone(): Promise<unknown> }
export interface Cloud {
  readonly url: string; readonly pid: string; readonly wire: Wire;
  readonly tapes: TapeSync | null; readonly prefs: PrefSync; readonly telemetry: Telemetry; readonly files: FileSync; readonly notes: NoteSync | null;
  /** the page died on this visit (`cloudFatal` comes here once the visit is up): one `fatal` record, sent at once */
  fatal(report: FatalReport): FatalSent;
  /** a file the page saved for the player (the download door's sink, host/savefile.ts): sent up too where this browser sends (files.ts). TRUE = it is
   *  going up now; false = it is not (the switch, the kind, its size) or it waits for the notice and goes when that has been shown */
  file(name: string, blob: Blob, kind: string, meta?: Record<string, string>): boolean;
  /** THE SAME, WITH THE WHOLE ANSWER, for a door that no longer downloads (`Saved`): what became of the file now, and — `done`, once, later, only for a
   *  file that was taken ('now' or 'held') — whether it arrived */
  save(name: string, blob: Blob, kind: string, meta?: Record<string, string>, done?: (arrived: boolean) => void): Saved;
  /** every saved file that may go has gone or been let go (true), or `capMs` (15 s) is over, a sending failed, or what is left waits for the notice
   *  (false). For a page about to leave: the sending is a file's only copy now, and a navigation cuts it off */
  filesIdle(capMs?: number): Promise<boolean>;
  /** one drawn frame (main.ts's loop): its seconds, whether a run is live, the mission's stage word, the GPU's ms where a new reading came (telemetry.ts) */
  /** …and, where the page hands in its GPU queue, THE GPU'S WAIT ON A FRAME (telemetry.ts `lag`): on a live frame, with no wait already out, the queue is
   *  asked when this frame's work is done. One promise at a time, none on a page that does not send; `?gpulag=0` puts it off */
  frame(rawDt: number, live: boolean, stage: string, gpuMs: number, queue?: GpuQueue): void;
  /** everything that waits, now (the page checks' and the console's): tapes, settings, records */
  sync(): Promise<void>;
  hook(): unknown;
}

/** the marks of the notes lines that went (notes.ts), in localStorage */
const lsMarks = (): NoteMarks => ({
  get: () => { try { const v = JSON.parse(lsGet(LS_NOTES) ?? '{}'); return v && typeof v === 'object' ? v as Record<string, string> : {}; } catch { return {}; } },
  set: all => lsSet(LS_NOTES, JSON.stringify(all)),
});
/** the tape notes in localStorage */
const lsNotes = (): TapeNotes => ({
  get: () => { try { const v = JSON.parse(lsGet(LS_TAPES) ?? '{}'); return v && typeof v === 'object' ? v as Record<string, TapeNote> : {}; } catch { return {}; } },
  set: all => lsSet(LS_TAPES, JSON.stringify(all)),
});

/** a page that died: what its error card says, where it was (`stage`: 'adapter' · 'device' · 'boot' · 'frame' · …) and whatever the reporter knows as plain JSON */
export interface FatalReport { message: string; stack?: string; stage?: string; data?: unknown; machine?: unknown; build?: string; sid?: string }
/** `waits`: this browser has not been shown the notice — the record goes when `told()` is said (the card's notice line). `sent` settles true once it is there */
export interface FatalSent { waits: boolean; told(): void; sent: Promise<boolean> }
export const FATAL = { max: 3, stackLines: 16 };
/** the record itself: the message, the stack as lines (a record's words are cut at 400 characters each), the stage and the reporter's bag */
export const fatalData = (r: FatalReport): Record<string, unknown> => ({
  message: String(r.message ?? '').slice(0, 400), stack: String(r.stack ?? '').split('\n').map(l => l.trim()).filter(Boolean).slice(0, FATAL.stackLines),
  stage: String(r.stage ?? '').slice(0, 40), ...(r.data !== undefined ? { data: r.data } : {}) });
let live: ((r: FatalReport) => FatalSent) | null = null;   // (the visit that is up, once `cloudBoot` has made it)
let fatals = 0;
/** (a check's: a page boots once, a check many times) */
export function cloudForget(): void { reportSink(null); live = null; liveNotes = null; liveSave = null; liveFilesOn = null; pageNotes = null; pageFiles = null; pageFatal = null; pageName = ''; draining = false; fatals = 0; }
let liveNotes: NoteSync | null = null;
const machineNow = (): unknown => { try { const n = navigator as Navigator & { deviceMemory?: number }; return { userAgent: n.userAgent, threads: n.hardwareConcurrency, memoryGb: n.deviceMemory, screen: typeof screen !== 'undefined' ? [screen.width, screen.height] : undefined, dpr: typeof devicePixelRatio !== 'undefined' ? devicePixelRatio : undefined, gpuApi: 'gpu' in n }; } catch { return {}; } };
/** A PAGE THAT DIED says so, once (a few times at most a visit). null = nothing is or will be sent: no backend, the switch off, `?cloudstats=0`. Never throws */
export function cloudFatal(q: URLSearchParams, report: FatalReport): FatalSent | null {
  try {
    if (fatals >= FATAL.max) return null; fatals++;
    if (live) return live(report);
    if (pageFatal) return pageFatal(report);   // (a page that reports — `cloudReports` — says its death under the same name as its reports)
    const url = cloudUrl(q), store = ls(); if (!url || !store || !CLOUD.telemetry || !cloudSharing(q) || off(q.get('cloudstats'))) return null;
    const pid = cloudPid(), W = wire(url), dice = new Uint8Array(6); try { crypto.getRandomValues(dice); } catch { /* the clock alone */ }
    const sid = report.sid ?? `fatal-${Date.now().toString(36)}-${[...dice].map(b => b.toString(36)).join('')}`.slice(0, 60);
    let ready = lsGet(LS_TOLD) === '1' || cloudChosen(q) || BREAK, done: (ok: boolean) => void = () => { /* */ };
    const sent = new Promise<boolean>(r => { done = r; });
    const T = new Telemetry(W, { pid, sid, build: () => report.build, may: () => ready });
    const go = async (): Promise<void> => {
      try { await W.mutation('prefs:hello', { pid }); } catch { /* the record is tried all the same */ }
      T.add('visit', { ...visitFacts(report.machine ?? machineNow()), started: false });
      for (const r of reportHeld()) T.error(r.message, r);   // (what the page reported before it died, a backend's side never having been up to take it)
      T.add('fatal', fatalData(report)); await T.flush(true);
      done(T.stats.sent >= 2);
    };
    if (ready) void go();
    return { waits: !ready, told: () => { if (ready) return; ready = true; lsSet(LS_TOLD, '1'); void go(); }, sent };
  } catch { return null; }
}

/** A PAGE THAT IS NOT THE GAME sends what was written on it: the replay viewer, after a note is typed (it boots no backend's side of its own, so a note
 *  would otherwise wait for the next game page in this browser). One round of the notes' sync under the same rules — the browser's switch, the notice,
 *  the replays' word — over the store the caller hands in. Resolves true where nothing failed; never rejects, and asks nothing where the browser does not send */
export async function cloudNotes(q: URLSearchParams, store: Pick<KeptStore, 'rows'>): Promise<boolean> {
  try {
    await cloudFind();
    if (liveNotes) { const f = liveNotes.stats.failed; await liveNotes.round(); return liveNotes.stats.failed === f; }
    const url = notesUrl(q); if (!url) return false;
    // ONE ROUND AT A TIME, and one more after it where a note was saved meanwhile (typing saves every quarter second): two rounds side by side read the
    // same marks and could land the older words last
    // (a saved window whose visit is not kept goes up whole through the saved files, as on the game's page: this page files it under its own name)
    const file = (n: string, b: Blob, k: string, m?: Record<string, string>): boolean => { const F = pageFileSync(q); if (!F || F.add(n, b, k, m) !== 'now') return false; drain(F); return true; };
    const P = pageNotes ??= { sync: new NoteSync(wire(url), { rows: () => store.rows(), shelf: shelfNoted, shelfFile: shelfGet }, { pid: cloudPid(), marks: lsMarks(), only: sid => visitsNow().includes(sid), file }), busy: null, again: false };
    if (P.busy) { P.again = true; return P.busy; }
    P.busy = (async () => { let f = 0; do { P.again = false; f = P.sync.stats.failed; await P.sync.round(); } while (P.again); return P.sync.stats.failed === f; })().finally(() => { P.busy = null; });
    return await P.busy;
  } catch { return false; }
}
let pageNotes: { sync: NoteSync; busy: Promise<boolean> | null; again: boolean } | null = null;

let liveSave: Cloud['save'] | null = null, liveFilesOn: (() => boolean) | null = null;   // (the visit that is up: its files go under its own name, by its own rules)
let pageFiles: FileSync | null = null;
/** the saved files' way in for a page with no visit of its own (null: this browser does not send files from here). What it saves is filed under a name
 *  made for this page load. Call after `cloudFind` */
function pageFileSync(q: URLSearchParams): FileSync | null {
  const url = cloudUrl(q); if (!url || !ls() || !CLOUD.files || !cloudSharing(q) || off(q.get('cloudfiles')) || !(lsGet(LS_TOLD) === '1' || cloudChosen(q) || BREAK)) return null;
  if (pageFiles) return pageFiles;
  return pageFiles = new FileSync(wire(url), { pid: cloudPid(), sid: pageSid(), on: () => cloudSharing(q) });
}
/** the name of this page load, for a page with no visit of its own: what it saves and what it reports are filed under the one name */
let pageName = '';
function pageSid(): string {
  if (pageName) return pageName;
  const dice = new Uint8Array(6); try { crypto.getRandomValues(dice); } catch { /* the clock alone */ }
  return pageName = `page-${Date.now().toString(36)}-${[...dice].map(b => b.toString(36)).join('')}`.slice(0, 60);
}
/** (nobody else comes round on such a page: a sending that failed is tried again here until every file has gone or been let go — FILESYNC.tries each) */
function drain(F: FileSync): void {
  if (draining) return; draining = true;
  // (until nothing waits: every failed round counts against the file at the head, which is let go after FILESYNC.tries, so this ends — and every file's
  // saver hears how it ended, however many were handed over while the backend was away)
  void (async () => { try { for (let i = 0; F.waiting; i++) { await F.round(); if (F.waiting) await new Promise(r => setTimeout(r, FILESYNC.againMs * Math.min(i + 1, 4))); } } finally { draining = false; } })();
}
let draining = false;
/** THIS BROWSER WOULD SEND A SAVED FILE FROM THIS PAGE NOW (a backend, its switch, the files' word, the notice read or the sending its own choice): what a
 *  page asks before it draws a save button, game or no game. A file can still be refused for itself ('no': empty, past the bound, a level) */
export async function cloudSendsFiles(q: URLSearchParams): Promise<boolean> {
  try { await cloudFind(); return liveFilesOn ? liveFilesOn() : pageFileSync(q) !== null; } catch { return false; }
}
/** A FILE SAVED ON A PAGE THAT HAS NO GAME (the replay viewer, a listening page: no boot of the backend's side, so no sink for its door — host/savefile.ts),
 *  or by anybody who wants to wait for the end: one file, sent now. Resolves 'sent' once its row is on the backend, 'failed' where the sending gave up,
 *  'off' where this browser does not send files (no backend, the switch, ?cloud=0, ?cloudfiles=0, a browser that has not read the notice yet and is on a
 *  page that cannot show it), 'no' where the file cannot go (empty, past the bound, a level). Never rejects. On the game's own page it goes through the
 *  visit that is up (and a file held for the notice resolves when that has been read and the file has gone) */
export async function cloudFile(q: URLSearchParams, name: string, blob: Blob, kind: string, meta?: Record<string, string>): Promise<'sent' | 'failed' | 'off' | 'no'> {
  try {
    await cloudFind();
    const ended = (take: (done: (ok: boolean) => void) => Saved): Promise<'sent' | 'failed' | 'off' | 'no'> => new Promise(res => { const t = take(ok => res(ok ? 'sent' : 'failed')); if (t === 'off' || t === 'no') res(t); });
    if (liveSave) { const save = liveSave; return await ended(done => save(name, blob, kind, meta, done)); }
    const F = pageFileSync(q); if (!F || !replayMay(q, kind, meta)) return 'off';
    const end = ended(done => savedWord(F.add(name, blob, kind, meta, done)));
    drain(F);
    return await end;
  } catch { return 'failed'; }
}

/** `secs`: a page's report goes up this long after it was made (the ones that come with it ride along); `early`: errors nothing caught that are kept
 *  while the page is still asking whether it has a backend */
export const PAGE_REPORTS = { secs: 2, early: 8 };
/** what a page that is not a visit says of itself: `page`, the word its records are marked with ('replay' = the map viewer, 'tape' = the game's page
 *  playing a replay's tape); `uncaught`, it has no listeners of its own for an error nothing caught, so those are taken here; the build and the machine
 *  where the page knows them (a page that knows no build asks its server for the one small file a build carries, when it first has something to say) */
export interface PageReports { page: string; uncaught?: boolean; build?: () => string | undefined; machine?: () => unknown }
let pageFatal: ((r: FatalReport) => FatalSent) | null = null;
/** A PAGE THAT IS NOT A VISIT ANYBODY PLAYED REPORTS WHAT GOES WRONG ON IT (report.ts: `reportError` from any file, and the page's console, tapped here):
 *  the replay viewer, and the game's page while it plays a replay's tape — neither boots the backend's side, so until this their reports waited in the
 *  page and went nowhere. They are `error` records under a name made for this page load, behind one `visit` record marked `page`.
 *  UNDER THE RECORDS' OWN WORDS, and narrower than a visit's: a backend, the browser's switch, `?cloud=0`, `?cloudstats=0` on this page's address (a
 *  tape's page carries the OFF words its tape was recorded under: replay/engineplay.ts tapeSearch) OR on the address of the game's page this tab was
 *  on last (`tabSaysOff`: the viewer's own address carries no words), AND the notice already read in this browser or the sending its own choice — such
 *  a page cannot show the notice, so a browser that has not read it sends nothing from here and the reports stay in the page.
 *  NOTHING IS ASKED OF THE BACKEND UNTIL THERE IS A REPORT: a page with nothing to say makes no request. No frame is counted and no `end` is written, so
 *  a report here is its first coming and carries no count. Resolves true where this page's reports go up; never rejects. Once a page */
export async function cloudReports(q: URLSearchParams, page: PageReports): Promise<boolean> {
  try {
    tapConsole();
    // (an error nothing caught while the page is still asking its server for its backend is kept, a few, and taken once that is known)
    let take: ((msg: unknown, more: ReportMore) => void) | null = null; const pre: [unknown, ReportMore][] = [];
    const got = (msg: unknown, more: ReportMore): void => { if (take) take(msg, more); else if (pre.length < PAGE_REPORTS.early) pre.push([msg, more]); };
    if (page.uncaught && !live && !pageFatal) try {
      window.addEventListener('error', ev => { if (!/ResizeObserver loop/.test(ev.message)) got(ev.error ?? ev.message, { where: 'window.error' }); });
      window.addEventListener('unhandledrejection', ev => got(ev.reason, { where: 'unhandledrejection' }));
    } catch { /* no page (a check under bun) */ }
    await cloudFind();
    if (live || pageFatal) return true;
    const mayNow = (): boolean => cloudSharing(q) && !tabSaysOff() && (lsGet(LS_TOLD) === '1' || cloudChosen(q) || BREAK);
    const url = cloudUrl(q); if (!url || !ls() || !CLOUD.telemetry || off(q.get('cloudstats')) || !mayNow()) { pre.length = 0; take = () => { /* this page sends nothing */ }; return false; }
    const W = wire(url), pid = cloudPid();
    let on = true, greeted = false, timer: ReturnType<typeof setTimeout> | null = null, asked: string | undefined;
    const build = (): string | undefined => { let b: string | undefined; try { b = page.build?.(); } catch { /* */ } return b ?? asked; };
    const T = new Telemetry(W, { pid, sid: pageSid(), build, may: () => on });
    /** the page's own `visit` record, written before its first report and not before (no hello: the browser's row counts visits, and this is none) */
    const greet = (): void => {
      if (greeted) return; greeted = true;
      if (!build()) try { void fetch('/qa/commit', { cache: 'no-store' }).then(x => x.ok ? x.text() : '').then(t => { const c = t.trim(); if (/^[0-9a-f]{7,40}$/.test(c)) asked = c; }, () => { /* a build that carries none */ }); } catch { /* */ }
      let machine: unknown; try { machine = page.machine?.(); } catch { /* the browser's own few words */ }
      T.add('visit', { ...visitFacts(machine ?? machineNow()), page: String(page.page).slice(0, 40), started: false });
    };
    const soon = (): void => { if (timer !== null) return; try { timer = setTimeout(() => { timer = null; void T.flush(); }, PAGE_REPORTS.secs * 1000); } catch { void T.flush(); } };
    // (whatever waits is tried again on the next report: a sending that failed is not left for the page's goodbye)
    take = (msg, more) => { if (!on) return; greet(); T.error(msg, more); if (T.waiting) soon(); };
    reportSink(r => take!(r.message, r));
    for (const [msg, more] of pre.splice(0)) take(msg, more);
    pageFatal = r => {
      let done: (ok: boolean) => void = () => { /* */ }; const sent = new Promise<boolean>(res => { done = res; });
      if (!on) done(false); else { if (r.build && !build()) asked = r.build; greet(); const before = T.stats.sent; T.add('fatal', fatalData(r)); void T.flush(true).then(() => done(T.stats.sent > before)); }
      return { waits: false, told: () => { /* read already */ }, sent };
    };
    try {
      // (another tab's word is this browser's newest: a browser that says no there, or whose storage was cleared, stops sending from here; what waited is
      // dropped, the page's visit record with it, so it is written again if the browser says yes again)
      window.addEventListener('storage', ev => { if (ev.key === LS_SHARE || ev.key === LS_TOLD || ev.key === null) { on = mayNow(); if (!on) { greeted = false; void T.flush(); } } });
      window.addEventListener('pagehide', () => { void T.flush(true); });
      document.addEventListener('visibilitychange', () => { if (document.visibilityState === 'hidden') void T.flush(true); });
    } catch { /* no page (a check under bun) */ }
    return true;
  } catch { return false; }
}
type ReportMore = { where?: string; level?: string; stack?: string[]; data?: unknown };
/** THE TAB REMEMBERS ITS VISIT'S OFF WORD. `?cloud=0` and `?cloudstats=0` are words on the GAME's address; the pages a visit leads to in the same tab
 *  (the replay viewer after Shift+F7, the map opened from a replay) have addresses of their own with no words on them. So the game's page writes down
 *  in the tab (sessionStorage: this tab, until it closes) whether its address said the records are off, at every boot, and a page that reports asks it.
 *  A viewer opened by itself in a fresh tab has no such mark and goes by the browser's own switch. (The viewer's notes and its Send button go by rules
 *  of their own, as before: a note goes only for a visit that may go, a Send is a press) */
const SS_OFF = 'twopointsix.cloud.off';
function tabMark(q: URLSearchParams): void {
  // (only a visit somebody plays takes the mark off again: the game's page playing a tape — `?playtape`, replay/engineplay.ts — is one of the pages a
  // visit leads to, and its address is the tape's)
  try { if (off(q.get('cloud')) || off(q.get('cloudstats'))) sessionStorage.setItem(SS_OFF, '1'); else if (!q.has('playtape')) sessionStorage.removeItem(SS_OFF); } catch { /* no session storage: nothing remembered */ }
}
const tabSaysOff = (): boolean => { try { return typeof sessionStorage !== 'undefined' && sessionStorage.getItem(SS_OFF) === '1'; } catch { return false; } };
/** what every `visit` record says of the page: the machine, the ms since the page opened, its query without `player`, the browser's language */
function visitFacts(machine: unknown): Record<string, unknown> {
  let query = ''; try { const p = new URLSearchParams(location.search); p.delete('player'); query = p.toString(); } catch { /* */ }
  return { machine, gpu: null, bootMs: typeof performance !== 'undefined' ? Math.round(performance.now()) : 0, query, lang: typeof navigator !== 'undefined' ? navigator.language : '' };
}

/** the backend a notes round from this page would send to ('' = it would send nothing: no backend, the switch, the replays' word, the notice not read) */
const notesUrl = (q: URLSearchParams): string => { const url = cloudUrl(q); return url && ls() && CLOUD.tapes && cloudSharing(q) && !off(q.get('cloudtapes')) && (lsGet(LS_TOLD) === '1' || cloudChosen(q) || BREAK) ? url : ''; };
/** THE NOTES OF THIS VISIT GO UP BY THEMSELVES from this page: a notes round here would send a row of visit `sid` (the rules above, and the visit is one
 *  of this browser's own whose tape may go). False for a replay opened from a file, another browser's visit, a visit that may not go: what is written on
 *  one of those reaches the backend only if somebody sends it (`cloudFile`). Sends nothing. Await it (a local build asks its server first) */
export async function cloudCarries(q: URLSearchParams, sid: string | undefined): Promise<boolean> {
  try { await cloudFind(); return !!sid && !!notesUrl(q) && visitsNow().includes(sid); } catch { return false; }
}

/** THE PAGE'S BOOT of the backend's side (main.ts, once, after the replay key is up). null = this build or this visit has no backend: nothing was made */
export function cloudBoot(q: URLSearchParams, deps: CloudDeps): Cloud | null {
  const url = cloudUrl(q), store = ls(); if (!url || !store) return null;
  const pid = cloudPid(), W = wire(url);
  let share = cloudSharing(q);
  const kind = (word: string, on: boolean) => on && !off(q.get(word));
  // (a browser that sends by the default alone waits for the notice: `told`, kept in the browser, said by the title page's line through the hook)
  let chosen: boolean | null = null;   // (what this page's own row said this visit: it stands over the address bar's word)
  let told = lsGet(LS_TOLD) === '1', picked = cloudChosen(q);
  const ready = (): boolean => share && (told || chosen === true || picked || BREAK);
  // (asked every frame by the records: the address bar's words are read once, and the browser's rows only when one moves)
  const words = { tapes: kind('cloudtapes', true), stats: kind('cloudstats', true), prefs: kind('cloudprefs', true), files: kind('cloudfiles', true) };
  const may = { tapes: () => CLOUD.tapes && words.tapes && ready(), stats: () => CLOUD.telemetry && words.stats && ready(), prefs: () => CLOUD.prefs && words.prefs && ready(), files: () => CLOUD.files && words.files && ready() };
  // THIS VISIT'S TAPE GOES only while its switch has been on since its boot (the list above): on the list now if it is, and off it for good the moment it is
  // not. (Waiting for the notice is not the switch being off: the first visit's tape goes up whole once the notice has been shown.)
  const tapesOn = (): boolean => share && CLOUD.tapes && words.tapes;
  // (a browser that says no lets go of the whole list: a visit it played before and had not sent yet does not go up when it says yes again)
  if (!share) lsSet(LS_VISITS, '[]');
  let whole = tapesOn(); visitMay(deps.sid, whole);
  const tapes = deps.store ? new TapeSync(W, deps.store, { pid, notes: lsNotes(), may: may.tapes, only: sid => visitsNow().includes(sid) }) : null;
  const prefs = new PrefSync(W, store, { pid, may: may.prefs });
  const telemetry = new Telemetry(W, { pid, sid: deps.sid, build: deps.build, may: may.stats, passes: deps.passes, render: deps.renderPasses });
  // (a saved file is held while the switch is on and the notice has not been shown yet: a page opened straight onto a benchmark sends its report once it has)
  const filesOn = (): boolean => share && CLOUD.files && words.files;
  const files = new FileSync(W, { pid, sid: deps.sid, build: deps.build, may: may.files, on: filesOn });
  const save: Cloud['save'] = (name, blob, kind, meta, done) => filesOn() && replayMay(q, kind, meta) ? savedWord(files.add(name, blob, kind, meta, done)) : 'off';
  // (what the player wrote on a replay rides with the replays: their word, their list of visits that may go)
  const notes = deps.store ? new NoteSync(W, { rows: () => deps.store!.rows(), shelf: shelfNoted, shelfFile: shelfGet }, { pid, marks: lsMarks(), may: may.tapes, only: sid => visitsNow().includes(sid), file: (n, b, k, m) => files.add(n, b, k, m) === 'now' }) : null;
  const bootMs = typeof performance !== 'undefined' ? Math.round(performance.now()) : 0;   // (the page's open to the game being up, shaders and all)
  const facts = (): PlayFacts => { try { return deps.facts(); } catch { return { scenario: '', stage: '', live: false, stats: {}, preset: '', scale: 1, auto: false }; } };

  /** the switch moved (this page's own row, or another tab's: the `storage` event below) */
  const switched = (): void => { share = chosen ?? cloudSharing(q); picked = cloudChosen(q); if (!share) lsSet(LS_VISITS, '[]'); if (!tapesOn() && whole) { whole = false; visitMay(deps.sid, false); } void hello(); void files.round(); if (ready() || !share) for (const f of owed.splice(0)) f(); };
  /** THE NOTICE HAS BEEN SHOWN, for the first time in this browser. Only this visit's tape may go from here: a visit played before the notice was ever
   *  on this browser's screen (a page opened straight into a mission, then closed) stays in the browser */
  const nowTold = (): void => { if (told) return; told = true; lsSet(LS_TOLD, '1'); if (!picked && chosen !== true) lsSet(LS_VISITS, JSON.stringify(whole ? [deps.sid] : [])); void hello(); void files.round(); for (const f of owed.splice(0)) f(); };
  let greeted = false;
  // THE PAGE DIED: the record goes out at once (with what the visit had waiting), or — a browser still waiting for the notice — when the notice has been shown
  const owed: (() => void)[] = [];
  const fatal = (r: FatalReport): FatalSent => {
    let done: (ok: boolean) => void = () => { /* */ }; const sent = new Promise<boolean>(res => { done = res; });
    const go = (): void => { if (!may.stats()) { done(false); return; } const before = telemetry.stats.sent; telemetry.add('fatal', { ...fatalData(r), ...telemetry.totals }, facts().scenario); bootNote(true); void telemetry.flush().then(() => done(telemetry.stats.sent > before)); };
    const waits = share && CLOUD.telemetry && words.stats && !ready();
    if (waits) owed.push(go); else go();
    return { waits, told: nowTold, sent };
  };
  const hello = async (): Promise<void> => {
    if (!ready() || greeted) return; greeted = true;
    try {
      const r = await W.mutation<{ rev: number }>('prefs:hello', { pid });
      // (the backend's copy is this browser's own last save: the first look sends only if a key has changed since. A browser that shares its id took a newer one before the boot — cloudAdopt)
      const now = prefsText(store); if (r.rev > 0 && r.rev === (Number(lsGet(LS_REV)) || 0) && lsGet(LS_SENT) === mark(now)) prefs.settled(now);
    } catch { /* tried again by nothing: a visit that could not say hello still sends its tapes and records when it can */ }
    let query = ''; try { const p = new URLSearchParams(location.search); p.delete('player'); query = p.toString(); } catch { /* */ }
    let gpu: unknown = null; try { gpu = deps.gpu?.() ?? null; } catch (e) { reportError('cloud.deps.gpu', e); }
    telemetry.add('visit', { machine: deps.machine(), gpu, bootMs, query, lang: typeof navigator !== 'undefined' ? navigator.language : '' });
  };
  // THE REPORTER'S REPORTS (report.ts: `reportError` from any file, and the page's tapped console) are this visit's `error` records from here on; what
  // was reported before this boot comes over now. (CLOUD_BREAK: nobody asks for them)
  // A browser still waiting for the notice keeps them (a few) until it has been shown, as a dead page's record waits: a first visit's boot is not lost.
  const early: Report[] = [], earlyGo = (): void => { for (const r of early.splice(0)) telemetry.error(r.message, r); };
  if (!BREAK) reportSink(r => { if (may.stats()) telemetry.error(r.message, r); else if (share && CLOUD.telemetry && words.stats && early.length < REPORT.held) { if (!early.length) owed.push(earlyGo); early.push(r); } });
  // WHICH LOAD OF THE TAB this is is counted at the boot, not when the load's record is written: a load that never wrote one is still a load
  if (!BREAK) tabLoad();
  // THE GPU'S WAIT ON A FRAME (telemetry.ts `lag`): one question out at a time, and a new one only `gpuLagEvery` live frames after the last was asked (a
  // freeze of seconds is caught as well by a question every few frames as by one a frame); a wait the tab was hidden in is nobody's freeze and is dropped;
  // a queue that cannot be asked is not asked again
  let lagOn = CLOUD.gpuLag && !off(q.get('gpulag')) && !BREAK, lagOut = false, lagAway = false, lagGap = CLOUD.gpuLagEvery;
  const lagAsk = (queue: GpuQueue): void => {
    if (++lagGap < CLOUD.gpuLagEvery || lagOut) return;
    try {
      if (typeof document !== 'undefined' && document.hidden) return;
      lagOut = true; lagAway = false; lagGap = 0; const t0 = performance.now();
      queue.onSubmittedWorkDone().then(() => { lagOut = false; if (!lagAway) telemetry.lag(performance.now() - t0); }, () => { lagOut = false; });
    } catch { lagOut = false; lagOn = false; }
  };
  // THE BOOT'S RECORD, once: asked for where the records go up anyway (never in a frame), and held until both the boot is over and this browser sends
  let booted = false;
  const bootNote = (last = false): void => { if (booted || !greeted || !may.stats()) return; let b: unknown = null; try { b = deps.boot?.(last) ?? null; } catch (e) { reportError('cloud.deps.boot', e); } if (b === null) { if (!deps.boot) booted = true; return; } booted = true; telemetry.add('boot', b); if ((b as { later?: unknown }).later !== undefined) latered = true; };
  // …AND THE MOMENT THE PROGRAMS BUILT BEHIND THE START WERE IN, where that came after the load's record was written (loadtime.ts laterRecord): one
  // small record, looked for on the records' own beat and no longer than `laterMs` after the page opened
  let latered = !deps.later;
  const laterNote = (): void => {
    if (latered || !booted || !may.stats()) return;
    if (typeof performance !== 'undefined' && performance.now() > LOADTIME.laterMs) { latered = true; return; }
    let b: unknown = null; try { b = deps.later?.() ?? null; } catch (e) { reportError('cloud.deps.later', e); latered = true; }
    if (b !== null) { latered = true; telemetry.add('later', b); }
  };
  const lookPrefs = async (going = false): Promise<void> => { const before = prefs.stats.puts; await prefs.look(going); if (prefs.stats.puts !== before) { lsSet(LS_REV, String(prefs.stats.rev)); lsSet(LS_SENT, mark(prefsText(store))); } };
  const sync = async (): Promise<void> => { bootNote(); laterNote(); await Promise.all([tapes?.round(), lookPrefs(), telemetry.flush(), notes?.round()]); await files.round(); };

  void hello();
  try {
    setTimeout(() => { void tapes?.round(); void lookPrefs(); }, 3000);
    if (tapes) setInterval(() => { void tapes.round(); }, TAPESYNC.secs * 1000);
    setInterval(() => { void lookPrefs(); }, PREFSYNC.secs * 1000);
    setInterval(() => { bootNote(); laterNote(); void telemetry.flush(); }, TELEMETRY.secs * 1000);
    setInterval(() => { void files.round(); }, FILESYNC.secs * 1000);
    if (notes) { setTimeout(() => { void notes.round(); }, 5000); setInterval(() => { void notes.round(); }, NOTESYNC.secs * 1000); }
    // the tab is hidden or the page is going: what waits is asked now (a tab only hidden finishes it; a page that dies finishes the small ones the browser keeps alive, and the next visit sends the tape's tail)
    // (`hidden`: the tab is out of sight and may come back, so the records keep a short stretch open — telemetry.ts; a page that is going says all it has)
    const leave = (hidden: boolean) => { telemetry.away(); bootNote(true); laterNote(); void lookPrefs(true); void telemetry.flush(true, facts(), hidden); void tapes?.round(); void files.round(); };
    document.addEventListener('visibilitychange', () => { if (document.visibilityState === 'hidden') { lagAway = true; leave(true); } else telemetry.back(); });
    window.addEventListener('pagehide', () => leave(false));
    // (another tab's row is this browser's newest word: it stands over what this page's own row said earlier in the visit)
    window.addEventListener('storage', ev => {
      if (ev.key === LS_TOLD || ev.key === null) told = lsGet(LS_TOLD) === '1';
      if (ev.key === LS_SHARE || ev.key === null) { const v = lsGet(LS_SHARE); chosen = v === '1' ? true : v === '0' ? false : null; }
      if (ev.key === LS_SHARE || ev.key === LS_TOLD || ev.key === null) switched();
    });
    window.addEventListener('pageshow', e => { if ((e as PageTransitionEvent).persisted) telemetry.back(); });
    window.addEventListener('error', ev => { if (!/ResizeObserver loop/.test(ev.message)) telemetry.error(ev.error ?? ev.message, { where: 'window.error' }); });
    window.addEventListener('unhandledrejection', ev => telemetry.error(ev.reason, { where: 'unhandledrejection' }));
  } catch { /* no page (a check under bun drives round / look / flush itself) */ }

  const C: Cloud = {
    url, pid, wire: W, tapes, prefs, telemetry, files, notes, sync,
    file: (name, blob, kind, meta) => replayMay(q, kind, meta) && files.add(name, blob, kind, meta) === 'now',
    save,
    filesIdle: capMs => files.settle(capMs),
    fatal,
    frame: (rawDt, live, stage, gpuMs, queue) => { telemetry.frame(rawDt, live, stage, gpuMs, facts); if (live && queue && lagOn && may.stats()) lagAsk(queue); },
    hook: () => ({
      url, pid, sid: deps.sid,
      /** the backend is this page's own server's (a local build): the title says nothing about sending */
      local: cloudLocal(q),
      /** this browser sends (written for the browser: it holds from the next round on, and for every later visit) */
      get share() { return share; },
      set share(v: boolean) {
        chosen = !!v; const want = chosen ? '1' : '0'; lsSet(LS_SHARE, want);
        // (a browser whose storage is full must still be able to say no for good: the tapes' notes are let go to make the room)
        if (lsGet(LS_SHARE) !== want) { try { store.removeItem(LS_TAPES); } catch { /* */ } lsSet(LS_SHARE, want); }
        switched();
      },
      /** the notice has been on this browser's screen (ui/cloudnote.ts): a browser that sends by the default may begin */
      told: nowTold,
      get waits() { return share && !ready(); },
      /** a link that hands this browser's id — and so its settings — to another browser. Whoever has it can read and change them: it is a key.
       *  (Asked for from Options' own rows, the switch on screen beside it: a browser that was waiting for the notice has seen as much) */
      link: (): string => { nowTold(); lsSet(LS_LINKED, '1'); const u = new URL(location.href); u.search = ''; u.hash = ''; u.searchParams.set('player', pid); return u.href; },
      sync, stats: () => ({ wire: { ...W.stats }, tapes: tapes ? { ...tapes.stats } : null, prefs: { ...prefs.stats }, telemetry: { ...telemetry.stats }, files: { ...files.stats, waiting: files.waiting } }),
      summary: () => [`backend ${url} · this browser ${share ? 'sends' : 'does not send'}`, tapes?.summary() ?? 'tapes: this page keeps none', prefs.summary(), telemetry.summary(), files.summary(), notes?.summary() ?? 'notes: this page keeps none'].join('\n'),
      knobs: { CLOUD, TAPESYNC, PREFSYNC, TELEMETRY, FILESYNC, NOTESYNC },
    }),
  };
  live = fatal; liveNotes = notes; liveSave = save; liveFilesOn = filesOn;
  return C;
}
