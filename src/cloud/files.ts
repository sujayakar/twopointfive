// THE FILES THE PAGE SAVES, SENT UP (cloud/convex/files.ts): whatever a hosted build hands the player as a download — a benchmark's report, a replay, a
// recording — also goes into the backend's file storage, so nobody has to send the file by hand (the maintainer, 10-27: "can you make it so benchmark
// results upload automatically? can use convex file storage" · "we should do that for everything that gets downloaded"). The page's one door for a
// download (src/host/savefile.ts) hands each file here; the player's own copy is the door's business and is not touched.
// A file is sent the way the service takes one: `files:start` answers with a one-time address, the bytes are POSTed there, `files:put` writes the row.
// A browser that is still waiting for the notice HOLDS its files (a few, in memory: a page opened straight onto a benchmark) and sends them once it may;
// a browser whose switch is off is handed nothing to hold. Nothing here is asked in a frame. No game code and no DOM: the check drives it under bun.
import { WireError, type Wire } from './wire';

/** `max`: a file past this many bytes stays the player's alone (the backend refuses it too). `hold` / `holdBytes`: what waits at most, oldest let go
 *  first. `tries`: a file whose sending failed this often is let go. `timeoutMs` + a ms for every `perMs` bytes: how long the bytes may take */
export const FILESYNC = { max: 32 * 1024 * 1024, hold: 8, holdBytes: 64 * 1024 * 1024, tries: 4, secs: 20, timeoutMs: 30000, perMs: 25, metaMax: 16000,
  /** kinds that stay the player's alone: the title's notice names replays, stats and settings, and a level somebody drew in the editor is none of them */
  skip: ['level'] as string[] };

/** (the check's red path: CLOUD_BREAK=1 in front of tools/qa/cloud-check.ts — the bytes go up and their row is never written; a page has no process) */
const BREAK = (() => { try { return String((globalThis as { process?: { env?: Record<string, string | undefined> } }).process?.env?.CLOUD_BREAK ?? '') === '1'; } catch { return false; } })();

export interface FileOpts {
  pid: string; sid: string;
  build?: () => string | undefined;
  /** this browser sends files now (its switch, the kind's word, the notice) */
  may?: () => boolean;
  /** its switch and the kind's word alone: a file is HELD while this is true and `may` is not yet */
  on?: () => boolean;
  fetch?: typeof fetch;
  now?: () => number;
}
interface Held { name: string; kind: string; blob: Blob; at: number; tries: number; meta?: string }
/** what the sink answers: the file is going up `now`, is `held` for the notice, or (null) stays the player's alone */
export type Taken = 'now' | 'held' | null;

/** a name the backend takes: no folders, plain characters */
export const fileName = (name: string): string => (name.split(/[\\/]/).pop() ?? '').replace(/[^A-Za-z0-9._ ()+-]/g, '_').replace(/^[^A-Za-z0-9]+/, '').slice(0, 120) || 'file';

export class FileSync {
  private q: Held[] = []; private going: Promise<void> | null = null;
  readonly stats = { asked: 0, sent: 0, bytes: 0, dropped: 0, failed: 0, lastError: '' };
  constructor(private w: Wire, private o: FileOpts) {}

  get waiting(): number { return this.q.length; }

  /** THE DOOR'S SINK: a file the page has saved for the player, and the few facts its saver put beside it (`meta`: short words, kept as one JSON text —
   *  a benchmark's report, a commit). Returns at once and never throws; the sending is in the background. The answer says what became of it */
  add(name: string, blob: Blob, kind: string, meta?: Record<string, string>): Taken {
    try {
      if ((this.o.on && !this.o.on()) || FILESYNC.skip.includes(kind)) return null;
      this.stats.asked++;
      if (!(blob instanceof Blob) || blob.size < 1 || blob.size > FILESYNC.max) { this.stats.dropped++; return null; }
      let words: string | undefined; try { const t = meta ? JSON.stringify(meta) : ''; if (t.length > 2 && t.length <= FILESYNC.metaMax) words = t; } catch { /* a file with no facts beside it */ }
      this.q.push({ name: fileName(name), kind: String(kind || 'file').slice(0, 24), blob, at: (this.o.now ?? Date.now)(), tries: 0, ...(words ? { meta: words } : {}) });
      let held = this.q.reduce((a, f) => a + f.blob.size, 0);
      while (this.q.length > FILESYNC.hold || held > FILESYNC.holdBytes) { held -= this.q.shift()!.blob.size; this.stats.dropped++; }
      const now = !this.o.may || this.o.may();
      void this.round();
      return now ? 'now' : 'held';
    } catch { return null; /* a file that could not be held is the player's alone */ }
  }

  /** send what waits, oldest first; a file that fails stays for the next round, a file the backend refuses is let go. Never rejects */
  round(): Promise<void> {
    // (one round at a time: whoever asks while one is going waits for that one)
    return this.going ??= this.run().finally(() => { this.going = null; });
  }
  private async run(): Promise<void> {
    {
      while (this.q.length) {
        if (this.o.on && !this.o.on()) { this.q.length = 0; break; }   // (the switch went off: what was held is let go)
        if (this.o.may && !this.o.may()) break;
        const f = this.q[0];
        try { await this.send(f); this.q.shift(); this.stats.sent++; this.stats.bytes += f.blob.size; }
        catch (e) {
          this.stats.failed++; this.stats.lastError = String((e as Error | undefined)?.message ?? e).slice(0, 200);
          if ((e instanceof WireError && e.refused) || ++f.tries >= FILESYNC.tries) { if (this.q[0] === f) this.q.shift(); this.stats.dropped++; continue; }
          break;
        }
      }
    }
  }

  private async send(f: Held): Promise<void> {
    const { pid, sid } = this.o, type = f.blob.type || 'application/octet-stream';
    const { url } = await this.w.mutation<{ url: string }>('files:start', { pid, sid, name: f.name, kind: f.kind, size: f.blob.size });
    const stop = typeof AbortController !== 'undefined' ? new AbortController() : null, t = stop ? setTimeout(() => stop.abort(), FILESYNC.timeoutMs + Math.round(f.blob.size / FILESYNC.perMs)) : null;
    let storageId: unknown;
    try {
      const res = await (this.o.fetch ?? fetch)(url, { method: 'POST', headers: { 'Content-Type': type }, body: f.blob, ...(stop ? { signal: stop.signal } : {}) });
      if (!res.ok) throw new Error(`the bytes were not taken: HTTP ${res.status}`);
      storageId = (await res.json() as { storageId?: unknown })?.storageId;
    } finally { if (t) clearTimeout(t); }
    if (typeof storageId !== 'string' || !storageId) throw new Error('the bytes were not taken: no storage id');
    if (BREAK) return;
    const build = this.o.build?.();
    await this.w.mutation('files:put', { pid, sid, ...(build ? { build: build.slice(0, 120) } : {}), name: f.name, kind: f.kind, type: type.slice(0, 120), at: f.at, storageId, ...(f.meta ? { meta: f.meta } : {}) });
  }

  summary(): string { const s = this.stats; return `files: ${s.sent} sent (${(s.bytes / 1024).toFixed(0)} KB) · ${this.q.length} waiting · ${s.dropped} let go${s.lastError ? ` · last error: ${s.lastError}` : ''}`; }
}
