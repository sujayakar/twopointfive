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
// else `CLOUD.shareDefault`. For one visit: `?cloud=0` sends nothing, `?cloud=1` sends. Each kind has its own word too: ?cloudtapes=0 ?cloudstats=0 ?cloudprefs=0.
// WHICH TAPES GO: only a visit that was played START TO END while the browser sent its tapes (`twopointsix.cloud.visits`, the list of them). A visit opened
// under `?cloud=0`, with the switch off, or during which the switch was off for a moment is never on the list, so no later visit sends it — turning the
// switch on sends the visits from the next page load on. The switch is read live: another tab that turns it off stops this one too.
// A LOOPBACK ADDRESS on the address bar (`?cloud=http://127.0.0.1:3210`) points this visit at a backend on this machine and sends to it: the page checks' way
// in (a stand-in, cloud/test/standin.ts), and a developer's. No other address is taken from the address bar.
// No game code is imported (src/game is one import cycle): main.ts hands in the visit's name, the kept tapes' store and closures over its own objects.
import type { KeptStore } from '../replay/keepstore';
import { wire, type Wire } from './wire';
import { TapeSync, TAPESYNC, type TapeNote, type TapeNotes } from './tapes';
import { PrefSync, PREFSYNC, prefsNow, type KeyStore } from './prefs';
import { Telemetry, TELEMETRY, type PlayFacts } from './telemetry';

/** the backend's address as the build was given it (tools/build.ts `define`); not defined at all on the dev server and under bun */
declare const __TPS_CLOUD__: string | undefined;

/** `shareDefault`: what a browser that has made no choice does. `tapes` / `telemetry` / `prefs`: the three kinds (a visit's ?cloudtapes=0 … puts one off).
 *  `adoptMs`: how long the boot waits on the backend where it must (a browser handed an id; one that shares its id) before it goes on without */
export const CLOUD = { shareDefault: false, tapes: true, telemetry: true, prefs: true, adoptMs: 4000 };

const LS_PID = 'twopointsix.cloud.pid', LS_SHARE = 'twopointsix.cloud.share', LS_TAPES = 'twopointsix.cloud.tapes', LS_LINKED = 'twopointsix.cloud.linked', LS_REV = 'twopointsix.cloud.rev', LS_SENT = 'twopointsix.cloud.sent', LS_VISITS = 'twopointsix.cloud.visits';
const PID = /^[a-z0-9]{24,64}$/, LOOPBACK = /^http:\/\/(127\.0\.0\.1|localhost|\[::1\])(:\d+)?\/?$/;
const off = (v: string | null): boolean => v === '0' || v === 'off' || v === 'false';
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
    if (lsGet(LS_LINKED) !== '1' || !cloudSharing(q) || off(q.get('cloudprefs'))) return false;
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
}
export interface Cloud {
  readonly url: string; readonly pid: string; readonly wire: Wire;
  readonly tapes: TapeSync | null; readonly prefs: PrefSync; readonly telemetry: Telemetry;
  /** one drawn frame (main.ts's loop) */
  frame(rawDt: number): void;
  /** everything that waits, now (the page checks' and the console's): tapes, settings, records */
  sync(): Promise<void>;
  hook(): unknown;
}

/** the tape notes in localStorage */
const lsNotes = (): TapeNotes => ({
  get: () => { try { const v = JSON.parse(lsGet(LS_TAPES) ?? '{}'); return v && typeof v === 'object' ? v as Record<string, TapeNote> : {}; } catch { return {}; } },
  set: all => lsSet(LS_TAPES, JSON.stringify(all)),
});

/** THE PAGE'S BOOT of the backend's side (main.ts, once, after the replay key is up). null = this build or this visit has no backend: nothing was made */
export function cloudBoot(q: URLSearchParams, deps: CloudDeps): Cloud | null {
  const url = cloudUrl(q), store = ls(); if (!url || !store) return null;
  const pid = cloudPid(), W = wire(url);
  let share = cloudSharing(q);
  const kind = (word: string, on: boolean) => on && !off(q.get(word));
  const may = { tapes: () => share && kind('cloudtapes', CLOUD.tapes), stats: () => share && kind('cloudstats', CLOUD.telemetry), prefs: () => share && kind('cloudprefs', CLOUD.prefs) };
  // THIS VISIT'S TAPE GOES only while it has been sending since its boot (the list above): on the list now if it sends, and off it for good the moment it does not
  let whole = may.tapes(); visitMay(deps.sid, whole);
  const tapes = deps.store ? new TapeSync(W, deps.store, { pid, notes: lsNotes(), may: may.tapes, only: sid => visitsNow().includes(sid) }) : null;
  const prefs = new PrefSync(W, store, { pid, may: may.prefs });
  const telemetry = new Telemetry(W, { pid, sid: deps.sid, build: deps.build, may: may.stats });
  const facts = (): PlayFacts => { try { return deps.facts(); } catch { return { scenario: '', stage: '', live: false, stats: {}, preset: '', scale: 1, auto: false }; } };

  /** the switch moved (this page's own row, or another tab's: the `storage` event below) */
  let chosen: boolean | null = null;   // (what this page's own row said this visit: it stands over the address bar's word)
  const switched = (): void => { share = chosen ?? cloudSharing(q); if (!may.tapes() && whole) { whole = false; visitMay(deps.sid, false); } if (share) void hello(); };
  let greeted = false;
  const hello = async (): Promise<void> => {
    if (!share || greeted) return; greeted = true;
    try {
      const r = await W.mutation<{ rev: number }>('prefs:hello', { pid });
      // (the backend's copy is this browser's own last save: the first look sends only if a key has changed since. A browser that shares its id took a newer one before the boot — cloudAdopt)
      const now = prefsText(store); if (r.rev > 0 && r.rev === (Number(lsGet(LS_REV)) || 0) && lsGet(LS_SENT) === mark(now)) prefs.settled(now);
    } catch { /* tried again by nothing: a visit that could not say hello still sends its tapes and records when it can */ }
    let query = ''; try { const p = new URLSearchParams(location.search); p.delete('player'); query = p.toString(); } catch { /* */ }
    telemetry.add('visit', { machine: deps.machine(), query, lang: typeof navigator !== 'undefined' ? navigator.language : '' });
  };
  const lookPrefs = async (going = false): Promise<void> => { const before = prefs.stats.puts; await prefs.look(going); if (prefs.stats.puts !== before) { lsSet(LS_REV, String(prefs.stats.rev)); lsSet(LS_SENT, mark(prefsText(store))); } };
  const sync = async (): Promise<void> => { await Promise.all([tapes?.round(), lookPrefs(), telemetry.flush()]); };

  void hello();
  try {
    setTimeout(() => { void tapes?.round(); void lookPrefs(); }, 3000);
    if (tapes) setInterval(() => { void tapes.round(); }, TAPESYNC.secs * 1000);
    setInterval(() => { void lookPrefs(); }, PREFSYNC.secs * 1000);
    setInterval(() => { void telemetry.flush(); }, TELEMETRY.secs * 1000);
    // the tab is hidden or the page is going: what waits is asked now (a tab only hidden finishes it; a page that dies finishes the small ones the browser keeps alive, and the next visit sends the tape's tail)
    const leave = () => { void lookPrefs(true); void telemetry.flush(true, facts()); void tapes?.round(); };
    document.addEventListener('visibilitychange', () => { if (document.visibilityState === 'hidden') leave(); else telemetry.back(); });
    window.addEventListener('pagehide', leave);
    window.addEventListener('storage', ev => { if (ev.key === LS_SHARE || ev.key === null) switched(); });
    window.addEventListener('pageshow', e => { if ((e as PageTransitionEvent).persisted) telemetry.back(); });
    window.addEventListener('error', ev => { if (!/ResizeObserver loop/.test(ev.message)) telemetry.error(ev.error ?? ev.message); });
    window.addEventListener('unhandledrejection', ev => telemetry.error(ev.reason));
  } catch { /* no page (a check under bun drives round / look / flush itself) */ }

  const C: Cloud = {
    url, pid, wire: W, tapes, prefs, telemetry, sync,
    frame: rawDt => telemetry.frame(rawDt, facts()),
    hook: () => ({
      url, pid, sid: deps.sid,
      /** this browser sends (written for the browser: it holds from the next round on, and for every later visit) */
      get share() { return share; },
      set share(v: boolean) { chosen = !!v; lsSet(LS_SHARE, chosen ? '1' : '0'); switched(); },
      /** a link that hands this browser's id — and so its settings — to another browser. Whoever has it can read and change them: it is a key */
      link: (): string => { lsSet(LS_LINKED, '1'); const u = new URL(location.href); u.search = ''; u.hash = ''; u.searchParams.set('player', pid); return u.href; },
      sync, stats: () => ({ wire: { ...W.stats }, tapes: tapes ? { ...tapes.stats } : null, prefs: { ...prefs.stats }, telemetry: { ...telemetry.stats } }),
      summary: () => [`backend ${url} · this browser ${share ? 'sends' : 'does not send'}`, tapes?.summary() ?? 'tapes: this page keeps none', prefs.summary(), telemetry.summary()].join('\n'),
      knobs: { CLOUD, TAPESYNC, PREFSYNC, TELEMETRY },
    }),
  };
  return C;
}
