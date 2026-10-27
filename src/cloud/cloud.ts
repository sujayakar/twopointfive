// THE HOSTED PAGE'S BACKEND, from the page's side. A build that was GIVEN A BACKEND'S ADDRESS (tools/build.ts builds one in where the environment names it —
// `TPS_CLOUD_URL`; the hosted build's script sets it, tools/host/build.sh) can keep three things there, each in a file of its own beside this one:
//   tapes.ts      the kept tape of every page visit (what this browser already keeps for the replay viewer and the hand-over)
//   telemetry.ts  small records of a visit: the machine, a run's outcome, a minute's frame times, an error
//   prefs.ts      the browser's settings, so another browser handed this one's id opens with them
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
import { FileSync, FILESYNC } from './files';

/** the backend's address as the build was given it (tools/build.ts `define`); not defined at all on the dev server and under bun */
declare const __TPS_CLOUD__: string | undefined;

/** `shareDefault`: what a browser that has made no choice does — ON since 2026-10-26 (the maintainer's pick: on, with a notice), and such a browser sends
 *  nothing until the notice has been on its screen (`told`, below). `tapes` / `telemetry` / `prefs` / `files`: the four kinds (a visit's ?cloudtapes=0 · ?cloudstats=0 · ?cloudprefs=0 · ?cloudfiles=0 puts one off).
 *  `adoptMs`: how long the boot waits on the backend where it must (a browser handed an id; one that shares its id) before it goes on without */
export const CLOUD = { shareDefault: true, tapes: true, telemetry: true, prefs: true, files: true, adoptMs: 4000 };

const LS_PID = 'twopointsix.cloud.pid', LS_SHARE = 'twopointsix.cloud.share', LS_TAPES = 'twopointsix.cloud.tapes', LS_LINKED = 'twopointsix.cloud.linked', LS_REV = 'twopointsix.cloud.rev', LS_SENT = 'twopointsix.cloud.sent', LS_VISITS = 'twopointsix.cloud.visits', LS_TOLD = 'twopointsix.cloud.told';
const PID = /^[a-z0-9]{24,64}$/, LOOPBACK = /^http:\/\/(127\.0\.0\.1|localhost|\[::1\])(:\d+)?\/?$/;
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

/** the backend this visit talks to ('' = none): a loopback address on the address bar, else the one the build was given; none at all under ?cloud=0 */
export function cloudUrl(q: URLSearchParams): string {
  const word = q.get('cloud');
  if (off(word)) return '';
  if (word && LOOPBACK.test(word)) return word.replace(/\/$/, '');
  try { return typeof __TPS_CLOUD__ === 'string' && /^https:\/\//.test(__TPS_CLOUD__) ? __TPS_CLOUD__.replace(/\/+$/, '') : ''; } catch { return ''; }
}
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
  return !off(word) && (word === '1' || word === 'on' || (word !== null && LOOPBACK.test(word)) || lsGet(LS_SHARE) === '1');
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
  /** the kept tapes' store (null: this page keeps none) and how to bring it up to the moment */
  store: Pick<KeptStore, 'rows' | 'head' | 'piecesFrom'> | null;
  build: () => string | undefined;
  machine: () => unknown;
  facts: () => PlayFacts;
  /** the GPU as the page got it (what it can do, its limits: asked once, for the visit's record) and the engine's per-pass times (telemetry.ts) */
  gpu?: () => unknown;
  /** the boot's timeline once the boot is over (ui/boot.ts, already taken): null = not yet. `last` = the page is going, say what there is */
  boot?: (last: boolean) => unknown;
  passes?: GpuPasses;
  /** the render passes among them: listed apart in the record (telemetry.ts) */
  renderPasses?: readonly string[];
}
export interface Cloud {
  readonly url: string; readonly pid: string; readonly wire: Wire;
  readonly tapes: TapeSync | null; readonly prefs: PrefSync; readonly telemetry: Telemetry; readonly files: FileSync;
  /** the page died on this visit (`cloudFatal` comes here once the visit is up): one `fatal` record, sent at once */
  fatal(report: FatalReport): FatalSent;
  /** a file the page saved for the player (the download door's sink, host/savefile.ts): sent up too where this browser sends (files.ts). TRUE = it is
   *  going up now; false = it is not (the switch, the kind, its size) or it waits for the notice and goes when that has been shown */
  file(name: string, blob: Blob, kind: string, meta?: Record<string, string>): boolean;
  /** one drawn frame (main.ts's loop): its seconds, whether a run is live, the mission's stage word, the GPU's ms where a new reading came (telemetry.ts) */
  frame(rawDt: number, live: boolean, stage: string, gpuMs: number): void;
  /** everything that waits, now (the page checks' and the console's): tapes, settings, records */
  sync(): Promise<void>;
  hook(): unknown;
}

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
export function cloudForget(): void { live = null; fatals = 0; }
const machineNow = (): unknown => { try { const n = navigator as Navigator & { deviceMemory?: number }; return { userAgent: n.userAgent, threads: n.hardwareConcurrency, memoryGb: n.deviceMemory, screen: typeof screen !== 'undefined' ? [screen.width, screen.height] : undefined, dpr: typeof devicePixelRatio !== 'undefined' ? devicePixelRatio : undefined, gpuApi: 'gpu' in n }; } catch { return {}; } };
/** A PAGE THAT DIED says so, once (a few times at most a visit). null = nothing is or will be sent: no backend, the switch off, `?cloudstats=0`. Never throws */
export function cloudFatal(q: URLSearchParams, report: FatalReport): FatalSent | null {
  try {
    if (fatals >= FATAL.max) return null; fatals++;
    if (live) return live(report);
    const url = cloudUrl(q), store = ls(); if (!url || !store || !CLOUD.telemetry || !cloudSharing(q) || off(q.get('cloudstats'))) return null;
    const pid = cloudPid(), W = wire(url), dice = new Uint8Array(6); try { crypto.getRandomValues(dice); } catch { /* the clock alone */ }
    const sid = report.sid ?? `fatal-${Date.now().toString(36)}-${[...dice].map(b => b.toString(36)).join('')}`.slice(0, 60);
    let ready = lsGet(LS_TOLD) === '1' || cloudChosen(q) || BREAK, done: (ok: boolean) => void = () => { /* */ };
    const sent = new Promise<boolean>(r => { done = r; });
    const T = new Telemetry(W, { pid, sid, build: () => report.build, may: () => ready });
    const go = async (): Promise<void> => {
      try { await W.mutation('prefs:hello', { pid }); } catch { /* the record is tried all the same */ }
      let query = ''; try { const p = new URLSearchParams(location.search); p.delete('player'); query = p.toString(); } catch { /* */ }
      T.add('visit', { machine: report.machine ?? machineNow(), gpu: null, bootMs: typeof performance !== 'undefined' ? Math.round(performance.now()) : 0, query, lang: typeof navigator !== 'undefined' ? navigator.language : '', started: false });
      T.add('fatal', fatalData(report)); await T.flush(true);
      done(T.stats.sent >= 2);
    };
    if (ready) void go();
    return { waits: !ready, told: () => { if (ready) return; ready = true; lsSet(LS_TOLD, '1'); void go(); }, sent };
  } catch { return null; }
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
  const files = new FileSync(W, { pid, sid: deps.sid, build: deps.build, may: may.files, on: () => share && CLOUD.files && words.files });
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
    let gpu: unknown = null; try { gpu = deps.gpu?.() ?? null; } catch { /* */ }
    telemetry.add('visit', { machine: deps.machine(), gpu, bootMs, query, lang: typeof navigator !== 'undefined' ? navigator.language : '' });
  };
  // THE BOOT'S RECORD, once: asked for where the records go up anyway (never in a frame), and held until both the boot is over and this browser sends
  let booted = false;
  const bootNote = (last = false): void => { if (booted || !greeted || !may.stats()) return; let b: unknown = null; try { b = deps.boot?.(last) ?? null; } catch { /* */ } if (b === null) { if (!deps.boot) booted = true; return; } booted = true; telemetry.add('boot', b); };
  const lookPrefs = async (going = false): Promise<void> => { const before = prefs.stats.puts; await prefs.look(going); if (prefs.stats.puts !== before) { lsSet(LS_REV, String(prefs.stats.rev)); lsSet(LS_SENT, mark(prefsText(store))); } };
  const sync = async (): Promise<void> => { bootNote(); await Promise.all([tapes?.round(), lookPrefs(), telemetry.flush(), files.round()]); };

  void hello();
  try {
    setTimeout(() => { void tapes?.round(); void lookPrefs(); }, 3000);
    if (tapes) setInterval(() => { void tapes.round(); }, TAPESYNC.secs * 1000);
    setInterval(() => { void lookPrefs(); }, PREFSYNC.secs * 1000);
    setInterval(() => { bootNote(); void telemetry.flush(); }, TELEMETRY.secs * 1000);
    setInterval(() => { void files.round(); }, FILESYNC.secs * 1000);
    // the tab is hidden or the page is going: what waits is asked now (a tab only hidden finishes it; a page that dies finishes the small ones the browser keeps alive, and the next visit sends the tape's tail)
    // (`hidden`: the tab is out of sight and may come back, so the records keep a short stretch open — telemetry.ts; a page that is going says all it has)
    const leave = (hidden: boolean) => { telemetry.away(); bootNote(true); void lookPrefs(true); void telemetry.flush(true, facts(), hidden); void tapes?.round(); void files.round(); };
    document.addEventListener('visibilitychange', () => { if (document.visibilityState === 'hidden') leave(true); else telemetry.back(); });
    window.addEventListener('pagehide', () => leave(false));
    // (another tab's row is this browser's newest word: it stands over what this page's own row said earlier in the visit)
    window.addEventListener('storage', ev => {
      if (ev.key === LS_TOLD || ev.key === null) told = lsGet(LS_TOLD) === '1';
      if (ev.key === LS_SHARE || ev.key === null) { const v = lsGet(LS_SHARE); chosen = v === '1' ? true : v === '0' ? false : null; }
      if (ev.key === LS_SHARE || ev.key === LS_TOLD || ev.key === null) switched();
    });
    window.addEventListener('pageshow', e => { if ((e as PageTransitionEvent).persisted) telemetry.back(); });
    window.addEventListener('error', ev => { if (!/ResizeObserver loop/.test(ev.message)) telemetry.error(ev.error ?? ev.message); });
    window.addEventListener('unhandledrejection', ev => telemetry.error(ev.reason));
  } catch { /* no page (a check under bun drives round / look / flush itself) */ }

  const C: Cloud = {
    url, pid, wire: W, tapes, prefs, telemetry, files, sync,
    file: (name, blob, kind, meta) => files.add(name, blob, kind, meta) === 'now',
    fatal,
    frame: (rawDt, live, stage, gpuMs) => telemetry.frame(rawDt, live, stage, gpuMs, facts),
    hook: () => ({
      url, pid, sid: deps.sid,
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
      summary: () => [`backend ${url} · this browser ${share ? 'sends' : 'does not send'}`, tapes?.summary() ?? 'tapes: this page keeps none', prefs.summary(), telemetry.summary(), files.summary()].join('\n'),
      knobs: { CLOUD, TAPESYNC, PREFSYNC, TELEMETRY, FILESYNC },
    }),
  };
  live = fatal;
  return C;
}
