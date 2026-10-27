// EVERY FILE THE PAGE SAVES goes out through this one door (2026-10-27): a recorder's take, a benchmark's result, a session journal, a saved replay, an
// exported checkpoint, a level from the editor. It began as the one place the page clicked an <a download>, with a listener (`SAVED.sink`) told of each file
// so that a hosted build's backend could keep the same one (the maintainer: "can you make it so benchmark results upload automatically?", "we should do that
// for everything that gets downloaded"). THEN, the same day: "we should get rid of local downloads entirely", and his pick for a file that cannot go: no
// download, the page says it was not sent, the editor keeps saving its level files. So a file now goes to the LISTENER ALONE and the browser is handed
// nothing, for every kind that has given its download up (`SAVE_UP`): the recorder's take, the benchmark's result, the session journal and an exported checkpoint have. A kind still
// on `SAVE_UP.local` is downloaded as before and told to the listener too: the editor's level (the one kind that stays), and a replay
// until the spot that saves it says where the file went instead of "downloaded" (it is another module's button: it takes its kind off the list with its
// own words). `?downloads=1` is the door as it was, for a developer: every file downloaded, the listener told.
// THE DOOR ANSWERS WHAT BECAME OF THE FILE (`Saved`, and later `done`: it arrived, or it was let go), and whoever saved it says so to the player in plain
// words (`savedWords`): there is no file in a downloads folder to notice any more, so a save that went nowhere must not look like one that went.
// WHO LISTENS IS NOT THIS FILE'S: the listener is set from outside (main.ts, to the backend's file intake where a build has one), and whether a file may go
// is the listener's to say, by its own rules. With nobody listening nothing is sent: the one-file build and every headless tool set no sink.
// A PAGE IS ONE BUNDLE: the replay viewer's page and the workbench have a door of their own, and no listener unless that page sets one.
// THIS MODULE IMPORTS NOTHING: ui/, replay/ and editor/ all save through it.
export type SavedKind = 'perf' | 'bench' | 'session' | 'replay' | 'checkpoint' | 'level';
/** a file as it went out. `meta`: short facts of it for whoever keeps it (the build's commit, what a take was of), never the file again */
export interface SavedFile { kind: SavedKind; name: string; blob: Blob; meta?: Record<string, string> }
/** what became of a saved file: 'sent' = the listener is sending it now (not that it has arrived), 'held' = the listener keeps it, in memory, and sends it
 *  once it may (its browser has not been shown the notice yet), false = it is going nowhere (nobody listens, the browser does not send, too large) */
export type Saved = 'sent' | 'held' | false;
/** … and how it ended, for a saver that is told later (`done`): 'arrived' = it is on the backend; false = it was let go after all */
export type Fate = Saved | 'arrived';
/** told once, later, of a file the listener took ('sent' or 'held'): it arrived, or it was let go. Never told of a file that was not taken */
export type SavedDone = (arrived: boolean) => void;
/** who is told of every file (null: nobody). Called once and never waited for: it answers at once what it does with the file. `true` reads as 'sent';
 *  anything that is not 'sent', 'held' or true reads as false. What it throws is dropped, and so is the rejection of a promise it hands back */
export const SAVED: { sink: ((f: SavedFile, done?: SavedDone) => Saved | boolean | void) | null } = { sink: null };
/** `on`: a saved file goes to the listener alone, and the browser is handed none (`?downloads=1` puts it off: every file is downloaded and the listener told,
 *  as it was). `local`: the kinds that are downloaded all the same */
export const SAVE_UP: { on: boolean; local: SavedKind[] } = { on: true, local: ['level', 'replay'] };
try { if (new URLSearchParams(location.search).get('downloads') === '1') SAVE_UP.on = false; } catch { /* no page (the harness) */ }
/** a file of this kind is handed to the browser as a download, whatever the listener does with it */
export const downloads = (kind: SavedKind): boolean => !SAVE_UP.on || SAVE_UP.local.includes(kind);
/** the page is served from this machine (or there is no page: a native host, the harness), where a shot server may be up (tools/shot-server.ts) */
export const onThisMachine = (): boolean => typeof location === 'undefined' || /^(localhost|127\.0\.0\.1|\[::1\])$/.test(location.hostname);
/** a file is worth making at all: somebody listens, or the browser is handed it (a large take is not put together for nobody) */
export const wanted = (kind: SavedKind): boolean => !!SAVED.sink || downloads(kind);
export interface SaveOpts {
  meta?: Record<string, string | undefined>;
  /** how long a download's address is kept alive for the browser to read (ms, 2000): a large file is still being written when the click returns */
  holdMs?: number;
  done?: SavedDone;
}

/** tell the listener of a file, with no download, and answer what it does with it. Never throws */
export function tellSaved(kind: SavedKind, name: string, blob: Blob, meta0?: Record<string, string | undefined>, done?: SavedDone): Saved {
  const sink = SAVED.sink; if (!sink) return false;
  try {
    const meta: Record<string, string> = {}; for (const [k, v] of Object.entries(meta0 ?? {})) if (typeof v === 'string' && v) meta[k] = v;
    // (a file's end is told once, and only of a file that was taken: a listener that refuses it and calls back all the same is not heard. A listener
    // may tell the end before it has answered — a file let go the moment it was handed over — and the saver then hears it after the answer)
    let took: Saved = false, answered = false, ended = false, early: boolean | null = null;
    const tell = (arrived: boolean): void => { if (ended || took === false) return; ended = true; try { done!(arrived); } catch { /* the saver's own trouble */ } };
    const said: unknown = sink({ kind, name, blob, ...(Object.keys(meta).length ? { meta } : {}) }, done && (arrived => { if (answered) tell(arrived); else early ??= arrived; }));
    if (said && typeof (said as PromiseLike<unknown>).then === 'function') (said as PromiseLike<unknown>).then(undefined, () => { /* its own trouble */ });
    took = said === true || said === 'sent' ? 'sent' : said === 'held' ? 'held' : false; answered = true;
    if (early !== null) { const e: boolean = early; queueMicrotask(() => tell(e)); }
    return took;
  } catch { return false; }   // (the listener's own trouble: nothing was taken)
}
function download(name: string, blob: Blob, holdMs = 2000): void {
  const a = document.createElement('a'), url = URL.createObjectURL(blob);
  try { a.href = url; a.download = name; document.body.appendChild(a); a.click(); } catch (e) { URL.revokeObjectURL(url); try { a.remove(); } catch { /* never in the page */ } throw e; }
  setTimeout(() => { URL.revokeObjectURL(url); a.remove(); }, holdMs);
}
/** what the door did with a file: `went` = the listener's answer, `local` = the browser was handed it as a download */
export interface Went { went: Saved; local: boolean }
/** save `blob` under `name`: to the listener, and to the browser as a download where the kind still has one (`downloads`). Answers what was done with it.
 *  THROWS only where a download was owed and could not be asked for (no page, the browser refused), as each spot's own anchor did, and the listener is not
 *  told then: a caller that marks a file as handed over must not go on as if it had been */
export function saveFile(kind: SavedKind, name: string, blob: Blob, o: SaveOpts = {}): Went {
  const local = downloads(kind); if (local) download(name, blob, o.holdMs);
  return { went: tellSaved(kind, name, blob, o.meta, o.done), local };
}
/** what became of a saved file that was not downloaded, said to the player. `what` opens the sentence ("The recording"); `many` = it names more than one thing */
export const savedWords = (what: string, went: Fate, many = false): string =>
  went === 'arrived' ? `${what} ${many ? 'were' : 'was'} sent to the developer.` : went === 'sent' ? `${what} ${many ? 'are' : 'is'} being sent to the developer.`
    : went === 'held' ? `${what} will be sent to the developer.` : `${what} ${many ? "weren't" : "wasn't"} sent.`;
