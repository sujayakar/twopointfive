// EVERY FILE THE PAGE HANDS THE BROWSER goes out through this one door (2026-10-27): a recorder's take, a session journal, a saved replay, the kept replays'
// hand-over, an exported checkpoint, a level from the editor. The door does what each of those spots did by hand, an <a download> clicked once, and then
// tells whoever listens (`SAVED.sink`) what went out, so a hosted build's backend can keep the same file (the maintainer: "can you make it so benchmark
// results upload automatically?", "we should do that for everything that gets downloaded"). A thing that was never a download can be told to the same
// listener by itself (`tellSaved`: a benchmark's result, a few KB, which must not hang on its take of tens of MB).
// WHO LISTENS IS NOT THIS FILE'S: the listener is set from outside (src/cloud, where a build has a backend and its browser sends), and whether a file may go
// is the listener's to say, by its own rules. With nobody listening it is the build as it was: the dev server, a checkout, the one-file build and every
// headless tool set no sink. A PAGE IS ONE BUNDLE: the replay viewer's page and the workbench have a door of their own, and no listener unless that page
// sets one.
// THIS MODULE IMPORTS NOTHING: ui/, replay/ and editor/ all save through it.
export type SavedKind = 'perf' | 'bench' | 'session' | 'replay' | 'dump' | 'checkpoint' | 'level';
/** a file as it went out. `meta`: short facts of it for whoever keeps it (the build's commit, what a take was of), never the file again */
export interface SavedFile { kind: SavedKind; name: string; blob: Blob; meta?: Record<string, string> }
/** who is told of every file that goes out (null: nobody). Called once, after the download was asked for, and never waited for: it answers at once, and only
 *  an answer of `false` says it does NOT take the file (its browser does not send, the file is too large). What it throws is dropped, and so is the
 *  rejection of a promise it hands back */
export const SAVED: { sink: ((f: SavedFile) => boolean | void) | null } = { sink: null };
export interface SaveOpts {
  meta?: Record<string, string | undefined>;
  /** how long the blob's address is kept alive for the browser to read (ms, 2000): a large file is still being written when the click returns */
  holdMs?: number;
  /** true: the listener is not told (a file that holds what the listener's own rules keep in this browser) */
  quiet?: boolean;
}

/** tell the listener of a file, with no download: true = a listener took it (it is being sent, not that it arrived). Never throws */
export function tellSaved(kind: SavedKind, name: string, blob: Blob, meta0?: Record<string, string | undefined>): boolean {
  const sink = SAVED.sink; if (!sink) return false;
  try {
    const meta: Record<string, string> = {}; for (const [k, v] of Object.entries(meta0 ?? {})) if (typeof v === 'string' && v) meta[k] = v;
    const took: unknown = sink({ kind, name, blob, ...(Object.keys(meta).length ? { meta } : {}) });
    if (took && typeof (took as PromiseLike<unknown>).then === 'function') (took as PromiseLike<unknown>).then(undefined, () => { /* its own trouble */ });
    return took !== false;
  } catch { return false; }   // (the listener's own trouble: nothing was taken)
}
/** hand `blob` to the browser as a download named `name`, then tell the listener. THROWS where the download could not be asked for (no page, the browser
 *  refused), as each spot's own anchor did: a caller that marks a file as handed over must not go on as if it had been. Returns whether a listener took it */
export function saveFile(kind: SavedKind, name: string, blob: Blob, o: SaveOpts = {}): boolean {
  const a = document.createElement('a'), url = URL.createObjectURL(blob);
  try { a.href = url; a.download = name; document.body.appendChild(a); a.click(); } catch (e) { URL.revokeObjectURL(url); try { a.remove(); } catch { /* never in the page */ } throw e; }
  setTimeout(() => { URL.revokeObjectURL(url); a.remove(); }, o.holdMs ?? 2000);
  return o.quiet ? false : tellSaved(kind, name, blob, o.meta);
}
