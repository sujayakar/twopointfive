// THE FILES THE PAGE SAVES, SENT UP (cloud/convex/files.ts): whatever a hosted build hands the player as a download — a benchmark's report, a replay, a
// recording — also goes into the backend's file storage, so nobody has to send the file by hand (the maintainer, 10-27: "can you make it so benchmark
// results upload automatically? can use convex file storage" · "we should do that for everything that gets downloaded"). The page's one door for a
// download (src/host/savefile.ts) hands each file here; the player's own copy is the door's business and is not touched.
// NOTHING IS DOWNLOADED ANY MORE (the maintainer, 10-27: "we should get rid of local downloads entirely"; his pick for a file that cannot go: no download,
// the page says it was not sent, the editor keeps saving its level files): the backend is the one place a saved file goes, so whoever hands one over can
// hear what became of it (`done`: it arrived, or it was let go).
// A file is sent the way the service takes one: `files:start` answers with a one-time address, the bytes are POSTed there, `files:put` writes the row.
// A browser that is still waiting for the notice HOLDS its files (a few, in memory: a page opened straight onto a benchmark) and sends them once it may;
// a browser whose switch is off is handed nothing to hold. Nothing here is asked in a frame. No game code and no DOM: the check drives it under bun.
import { WireError, type Wire } from './wire';

/** `max`: a file past this many bytes stays the player's alone (the backend refuses it too). `hold` / `holdBytes`: what waits at most, oldest let go
 *  first. `tries`: a file whose sending failed this often is let go. `againMs`: how soon a page with no rounds of its own tries a failed sending again (cloud.ts). `timeoutMs` + a ms for every `perMs` bytes: how long the bytes may take */
export const FILESYNC = { max: 32 * 1024 * 1024, hold: 8, holdBytes: 192 * 1024 * 1024, tries: 4, secs: 20, againMs: 1500, timeoutMs: 30000, perMs: 25, metaMax: 16000,
  /** A TEXT FILE IS PACKED on its way (gzip; JSON, ndjson, plain text of `packFrom` bytes or more): with nothing downloaded a recorder's long take, some
   *  50 MB of JSON, would otherwise be past `max` and kept nowhere; packed it is about a tenth. `max` bounds what is SENT (the backend's bound), `rawMax`
   *  what is held (never more than `holdBytes`: a file that could not be held alone must not be taken): a text file past it stays the player's. The row carries the packed file under `<name>.gz` with `packed: 'gzip'` beside it, and the
   *  pull (tools/cloud/pull.ts) writes it out as it was saved */
  packFrom: 256 * 1024, rawMax: 192 * 1024 * 1024,
  /** kinds that stay the player's alone: a level somebody drew in the editor is their own work, not a record of play, and the maintainer has not been asked for it */
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
  /** (a check's: its own packer, or null for a browser that has none) */
  pack?: ((b: Blob) => Promise<Blob>) | null;
}
interface Held { name: string; kind: string; blob: Blob; at: number; tries: number; meta?: string; done?: (arrived: boolean) => void; packed?: Blob; plain?: boolean }
const TOO_BIG = new Error('the file is past the bound, packed as it is');
/** a file whose bytes are text: worth packing */
const texty = (name: string, blob: Blob): boolean => /^(application\/(json|x-ndjson)|text\/)/.test(blob.type) || (!blob.type && /\.(json|ndjson|txt|csv|log)$/i.test(name));
/** `name`.gz inside the backend's bound on a name: a long name loses the end of its stem, never its head or its extension */
const gzName = (name: string): string => { const dot = name.lastIndexOf('.'), ext = dot > 0 && name.length - dot <= 12 ? name.slice(dot) : ''; return `${name.slice(0, name.length - ext.length).slice(0, 117 - ext.length)}${ext}.gz`; };
const gzip = async (b: Blob): Promise<Blob> => new Blob([await new Response(b.stream().pipeThrough(new CompressionStream('gzip'))).arrayBuffer()], { type: 'application/gzip' });
/** what the sink answers: the file is going up `now`, is `held` for the notice, or (null) stays the player's alone */
export type Taken = 'now' | 'held' | null;

/** a name the backend takes: no folders, plain characters */
export const fileName = (name: string): string => (name.split(/[\\/]/).pop() ?? '').replace(/[^A-Za-z0-9._ ()+-]/g, '_').replace(/^[^A-Za-z0-9]+/, '').slice(0, 120) || 'file';

export class FileSync {
  private q: Held[] = []; private going: Promise<void> | null = null; private sending: Held | null = null;
  readonly stats = { asked: 0, sent: 0, bytes: 0, dropped: 0, failed: 0, lastError: '' };
  constructor(private w: Wire, private o: FileOpts) {}

  get waiting(): number { return this.q.length; }

  /** THE DOOR'S SINK: a file the page has saved for the player, and the few facts its saver put beside it (`meta`: short words, kept as one JSON text —
   *  a benchmark's report, a commit). Returns at once and never throws; the sending is in the background. The answer says what became of it. `done` is
   *  told once, later, how a file that was taken ended: true = its row is on the backend, false = it was let go (refused, failed too often, the switch
   *  went off, too many waiting). It is never told of a file that was not taken (null) */
  add(name: string, blob: Blob, kind: string, meta?: Record<string, string>, done?: (arrived: boolean) => void): Taken {
    try {
      if ((this.o.on && !this.o.on()) || FILESYNC.skip.includes(kind)) return null;
      this.stats.asked++;
      if (!(blob instanceof Blob) || blob.size < 1 || blob.size > (this.packer() && texty(name, blob) ? FILESYNC.rawMax : FILESYNC.max)) { this.stats.dropped++; return null; }
      let words: string | undefined; try { const t = meta ? JSON.stringify(meta) : ''; if (t.length > 2 && t.length <= FILESYNC.metaMax) words = t; } catch { /* a file with no facts beside it */ }
      this.q.push({ name: fileName(name), kind: String(kind || 'file').slice(0, 24), blob, at: (this.o.now ?? Date.now)(), tries: 0, ...(words ? { meta: words } : {}), ...(done ? { done } : {}) });
      let held = this.q.reduce((a, f) => a + f.blob.size, 0);
      // (the oldest is let go — but never the one whose bytes are on their way: it is about to arrive)
      while (this.q.length > 1 && (this.q.length > FILESYNC.hold || held > FILESYNC.holdBytes)) {   // (never the file just taken: its answer says it is held)
        const old = this.q[0] === this.sending && this.q.length > 1 ? this.q.splice(1, 1)[0] : this.q.shift()!; held -= old.blob.size; this.stats.dropped++; this.tell(old, false); }
      const now = !this.o.may || this.o.may();
      void this.round();
      return now ? 'now' : 'held';
    } catch { return null; /* a file that could not be held is the player's alone */ }
  }

  private packer(): ((b: Blob) => Promise<Blob>) | null { return this.o.pack !== undefined ? this.o.pack : typeof CompressionStream !== 'undefined' ? gzip : null; }
  /** NOTHING MORE CAN GO RIGHT NOW: resolves once every file that may go has gone or been let go (true), or — false — when `capMs` is over, a sending
   *  failed (it waits for the next round), or what is left waits for the notice. For a page about to leave (a card's Close): the sending is the only
   *  copy, and leaving cuts it off. Never rejects */
  async settle(capMs = 15000): Promise<boolean> {
    let over = false, t: ReturnType<typeof setTimeout> | undefined; const cap = new Promise<void>(r => { t = setTimeout(() => { over = true; r(); }, Math.max(0, capMs)); });
    try {
      while (this.q.length && !over && !(this.o.may && !this.o.may()) && !(this.o.on && !this.o.on())) { const failed = this.stats.failed; await Promise.race([this.round(), cap]); if (this.stats.failed !== failed) break; }
    } catch { /* said by the answer */ } finally { if (t !== undefined) clearTimeout(t); }
    return this.q.length === 0;
  }
  private tell(f: Held, arrived: boolean): void { const d = f.done; f.done = undefined; try { d?.(arrived); } catch { /* the listener's own trouble */ } }

  /** send what waits, oldest first; a file that fails stays for the next round, a file the backend refuses is let go. Never rejects */
  round(): Promise<void> {
    // (one round at a time: whoever asks while one is going waits for that one)
    return this.going ??= this.run().finally(() => { this.going = null; });
  }
  private async run(): Promise<void> {
    {
      while (this.q.length) {
        if (this.o.on && !this.o.on()) { for (const f of this.q.splice(0)) this.tell(f, false); break; }   // (the switch went off: what was held is let go)
        if (this.o.may && !this.o.may()) break;
        const f = this.q[0];
        try { this.sending = f; try { await this.send(f); } finally { this.sending = null; } if (this.q[0] === f) this.q.shift(); this.stats.sent++; this.stats.bytes += f.blob.size; this.tell(f, true); }
        catch (e) {
          this.stats.failed++; this.stats.lastError = String((e as Error | undefined)?.message ?? e).slice(0, 200);
          if ((e instanceof WireError && e.refused) || e === TOO_BIG || ++f.tries >= FILESYNC.tries) { if (this.q[0] === f) this.q.shift(); this.stats.dropped++; this.tell(f, false); continue; }
          break;
        }
      }
    }
  }

  private async send(f: Held): Promise<void> {
    const { pid, sid } = this.o, pack = this.packer();
    // (packed once, kept for the next try; a browser with no packer sends the file as it is)
    if (!f.packed && !f.plain && pack && f.blob.size >= FILESYNC.packFrom && texty(f.name, f.blob)) {
      // (a packer that fails is not a sending that failed: the file goes as it is where it fits)
      try { const p = await pack(f.blob); if (p.size > 0 && p.size < f.blob.size) f.packed = p; else f.plain = true; } catch { f.plain = true; }
    }
    const body = f.packed ?? f.blob, name = f.packed ? gzName(f.name) : f.name, type = f.packed ? 'application/gzip' : f.blob.type || 'application/octet-stream';
    if (body.size > FILESYNC.max) throw TOO_BIG;
    let meta = f.meta; if (f.packed) try { const t = JSON.stringify({ ...(f.meta ? JSON.parse(f.meta) as Record<string, string> : {}), packed: 'gzip', rawBytes: String(f.blob.size), rawType: f.blob.type || '' }); if (t.length <= FILESYNC.metaMax) meta = t; else meta = JSON.stringify({ packed: 'gzip' }); } catch { meta = JSON.stringify({ packed: 'gzip' }); }
    const { url } = await this.w.mutation<{ url: string }>('files:start', { pid, sid, name, kind: f.kind, size: body.size });
    const stop = typeof AbortController !== 'undefined' ? new AbortController() : null, t = stop ? setTimeout(() => stop.abort(), FILESYNC.timeoutMs + Math.round(body.size / FILESYNC.perMs)) : null;
    let storageId: unknown;
    try {
      const res = await (this.o.fetch ?? fetch)(url, { method: 'POST', headers: { 'Content-Type': type }, body, ...(stop ? { signal: stop.signal } : {}) });
      if (!res.ok) throw new Error(`the bytes were not taken: HTTP ${res.status}`);
      storageId = (await res.json() as { storageId?: unknown })?.storageId;
    } finally { if (t) clearTimeout(t); }
    if (typeof storageId !== 'string' || !storageId) throw new Error('the bytes were not taken: no storage id');
    if (BREAK) return;
    const build = this.o.build?.();
    await this.w.mutation('files:put', { pid, sid, ...(build ? { build: build.slice(0, 120) } : {}), name, kind: f.kind, type: type.slice(0, 120), at: f.at, storageId, ...(meta ? { meta } : {}) });
  }

  summary(): string { const s = this.stats; return `files: ${s.sent} sent (${(s.bytes / 1024).toFixed(0)} KB) · ${this.q.length} waiting · ${s.dropped} let go${s.lastError ? ` · last error: ${s.lastError}` : ''}`; }
}
