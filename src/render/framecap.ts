// FRAME CAPTURE AND THE RENDERER'S TRACE (docs/agents/render.md, "A CAPTURE IS EVERY TARGET OF A FEW FRAMES").
//
// Two things a picture fault needs and a screenshot does not hold:
//
//  · EVERY TARGET the renderer wrote, as the numbers it wrote, for a few frames in a row: the g-buffer, each light class before and after its
//    denoiser, every history. A fault is then followed from the picture back to the first target that has it, and a flash to the frame it starts on.
//  · WHAT THE RENDERER DID around that moment: every change of a piece of its state (the size it draws at, a quality step, its targets re-made, a
//    history restarted, a pass shed) with the frame it happened on, and each frame's length.
//
// There is no list of targets here. `watchTextures` sees every texture the device makes; a capture takes each live 2D one that may be copied from
// (the render targets ask for COPY_SRC where they are made: passes.ts SU / RT, gbuffer.ts) and names the rest in `skipped`. And there is no list of
// events: the page hands `state()`, a flat record of what the renderer is set to, and a change of any key IS an event. A new target or a new piece
// of state is in the next capture with nothing written for it.
//
// A capture is THE WHOLE PICTURE over 2 frames, from the frame after it is asked for (the maintainer, 11-03, of a window at the cursor: "can you make
// the captures for the entire frame? i don't want to have to think about the cursor"; two frames, so that the reader can say what moved). Every target
// of a frame is about 290 MB at 1654 × 1000 and some 300 MB at 4K's internal sizes, about a third of that gzipped. Asked with `whole: false` it is a
// WINDOW (256 output pixels square at `at`, the same part of the picture in a target of any size) over 8 frames: a tenth the size, for a thing that
// flickers in one place. It is one gzipped file (`packCapture`),
// sent to the dev server (tools/serve.ts POST /capture, written under captures/) or downloaded where there is none, and read with
// `bun run tools/qa/capture-read.ts <file>`: a strip of every target over the frames, what moved in each, and the trace.
//
// The copies are taken after the frame is submitted, so a frame is captured as it was drawn; the run costs frame time while it lasts and nothing
// when it does not (the trace is a few numbers a frame). Nothing here is read by the game.

/** bytes a texel of the formats a target here may have (a format not listed is named in `skipped`) */
export const TEXEL_BYTES: Readonly<Record<string, number>> = {
  r8unorm: 1, r8uint: 1, rg8unorm: 2, rg8uint: 2, rgba8unorm: 4, 'rgba8unorm-srgb': 4, bgra8unorm: 4, 'bgra8unorm-srgb': 4, rgba8uint: 4,
  r16float: 2, r16uint: 2, rg16float: 4, rg16uint: 4, rgba16float: 8, rgba16uint: 8,
  r32float: 4, r32uint: 4, rg32float: 8, rg32uint: 8, rgba32float: 16, rgba32uint: 16,
  rgb10a2unorm: 4, rg11b10ufloat: 4, rgb9e5ufloat: 4, depth32float: 4,
};
export type StateV = string | number | boolean;
export interface CapEntry { label: string; id: number; format: string; full: [number, number]; rect: [number, number, number, number]; at: number; bytes: number }
export interface CapFrame { n: number; t: number; dt: number; cpu: number; state: Record<string, StateV>; gpu?: Record<string, number>; entries: CapEntry[] }
export interface TraceEvent { n: number; t: number; kind: string; text: string }
export interface TraceDump { events: TraceEvent[]; first: number; t: number[]; dt: number[]; cpu: number[] }
export interface CapWindow { u: number; v: number; px: number; whole: boolean }
export interface CapMeta { version: 1; name: string; why: string; page: string; adapter: string; out: [number, number]; window: CapWindow; frames: CapFrame[]; skipped: { label: string; why: string }[]; trace: TraceDump }
export interface CapOpts { frames: number; px: number; whole: boolean; at: [number, number]; why: string }

/** The part of a target that a capture's window covers. A target shaped like the picture (any size) gives the same part of the picture: `px` output
 *  pixels square about (u, v), scaled to its own grid and kept inside it; `whole`, or a target of another shape (a map, a table), gives all of it. */
export function cropRect(tw: number, th: number, ow: number, oh: number, w: CapWindow): [number, number, number, number] {
  const screen = ow > 0 && oh > 0 && Math.abs(tw * oh - th * ow) <= 0.03 * th * ow + ow + oh;
  if (w.whole || !screen) return [0, 0, tw, th];
  const cw = Math.min(tw, Math.max(8, Math.round(w.px * tw / ow))), ch = Math.min(th, Math.max(8, Math.round(w.px * th / oh)));
  const x = Math.min(tw - cw, Math.max(0, Math.round(w.u * tw - cw / 2))), y = Math.min(th - ch, Math.max(0, Math.round(w.v * th - ch / 2)));
  return [x, y, cw, ch];
}

const MAGIC = 'TPSCAP1\n';
/** one file: the magic line, the meta's length (u32 LE), the meta as JSON, then every entry's texels end to end (CapEntry.at / .bytes into that part) */
export function captureParts(meta: CapMeta, blobs: Uint8Array[]): Uint8Array[] {
  const head = new TextEncoder().encode(JSON.stringify(meta)), len = new Uint8Array(4); new DataView(len.buffer).setUint32(0, head.length, true);
  return [new TextEncoder().encode(MAGIC), len, head, ...blobs];
}
export function packCapture(meta: CapMeta, blobs: Uint8Array[]): Uint8Array {
  const parts = captureParts(meta, blobs), out = new Uint8Array(parts.reduce((n, p) => n + p.length, 0)); let at = 0; for (const p of parts) { out.set(p, at); at += p.length; }
  return out;
}
export function unpackCapture(file: Uint8Array): { meta: CapMeta; texels: Uint8Array } {
  if (new TextDecoder().decode(file.subarray(0, MAGIC.length)) !== MAGIC) throw new Error('not a capture (gunzip it first: the file on disk is gzipped)');
  const n = new DataView(file.buffer, file.byteOffset, file.byteLength).getUint32(MAGIC.length, true), at = MAGIC.length + 4;
  return { meta: JSON.parse(new TextDecoder().decode(file.subarray(at, at + n))) as CapMeta, texels: file.subarray(at + n) };
}

// ---- every texture the device makes ----
const made: WeakRef<GPUTexture>[] = [], gone = new WeakSet<GPUTexture>(), ids = new WeakMap<GPUTexture, number>(); let nextId = 1, watched = false;
/** From here on every texture this device makes is known to a capture (held weakly: a texture the page lets go of is not kept alive). Once, before the engine. */
export function watchTextures(device: GPUDevice): void {
  if (watched) return; watched = true;
  const create = device.createTexture.bind(device);
  device.createTexture = (d: GPUTextureDescriptor): GPUTexture => {
    const t = create(d); made.push(new WeakRef(t)); ids.set(t, nextId++);
    const destroy = t.destroy.bind(t); t.destroy = () => { gone.add(t); destroy(); };
    if (made.length > 2048) { let k = 0; for (const r of made) { const x = r.deref(); if (x && !gone.has(x)) made[k++] = r; } made.length = k; }
    return t;
  };
}
export function liveTextures(): GPUTexture[] { const out: GPUTexture[] = []; for (const r of made) { const t = r.deref(); if (t && !gone.has(t)) out.push(t); } return out; }

interface Held { buf: GPUBuffer; items: { e: CapEntry; at: number; bpr: number; srcW: number; srcH: number }[] }
/** a run's read-back buffers, let go (a capture that failed or was stopped: a whole frame's are hundreds of MB) */
function free(r: Run): void { for (const held of r.held) for (const h of held) { try { h.buf.destroy(); } catch { /* gone already */ } } r.held = []; }
interface Run { o: CapOpts; name: string; out: [number, number]; frames: CapFrame[]; held: Held[][]; skipped: Map<string, string> }
export interface FrameCapHost {
  /** what the renderer is set to, flat: a change of any key is a trace event, and the whole record goes into each captured frame */
  state: () => Record<string, StateV>;
  /** the output size the picture is drawn at (a window is measured in its pixels) */
  out: () => [number, number];
  /** the passes' own timings, where the page has them */
  gpu?: () => Record<string, number>;
  /** a texture that is parked, not drawn to (a pool's): left out */
  parked?: (t: GPUTexture) => boolean;
  adapter: string;
  /** where a finished capture goes; resolves to where it went, for the status line */
  save: (name: string, gz: Uint8Array) => Promise<string>;
  /** each line of what a capture is doing, as it is said (the page's message line; the status line and the console have it anyway) */
  said?: (line: string) => void;
}

const RING = 900, EVENTS = 400, CHUNK = 96 << 20, PLAIN_MAX = 4 << 20;
export class FrameCap {
  /** one line for the HUD while there is something to say (a run under way, where the last one went) */
  status = ''; statusUntil = 0;
  readonly events: TraceEvent[] = [];
  private n = 0; private last: Record<string, StateV> | null = null; private ema = 0; private lastT = 0;
  private rt = new Float64Array(RING); private rdt = new Float32Array(RING); private rcpu = new Float32Array(RING);
  private taken = 0; private run: Run | null = null; private armed: { o: CapOpts; on: string[] | null } | null = null; private busy = false;
  constructor(private device: GPUDevice, private host: FrameCapHost) {}
  get active(): boolean { return !!this.run || this.busy; }
  get waiting(): boolean { return !!this.armed; }

  note(kind: string, text: string, t = performance.now()): void { this.events.push({ n: this.n, t: Math.round(t), kind, text }); if (this.events.length > EVENTS) this.events.splice(0, this.events.length - EVENTS); }
  private opts(o: Partial<CapOpts>): CapOpts { const whole = o.whole !== false, num = (x: unknown, d: number) => typeof x === 'number' && Number.isFinite(x) ? x : d, u = (x: unknown) => Math.min(1, Math.max(0, num(x, 0.5)));
    return { frames: Math.max(1, Math.min(64, Math.round(num(o.frames, whole ? 2 : 8)))), px: Math.max(16, Math.round(num(o.px, 256))), whole, at: [u(o.at?.[0]), u(o.at?.[1])], why: o.why ?? 'asked' }; }
  /** capture from the next drawn frame on */
  start(o: Partial<CapOpts> = {}): string {
    if (this.active) return this.say('a capture is already under way');
    const c = this.opts(o), d = new Date(), z = (x: number) => String(x).padStart(2, '0');
    this.run = { o: c, name: `cap-${d.getFullYear()}${z(d.getMonth() + 1)}${z(d.getDate())}-${z(d.getHours())}${z(d.getMinutes())}${z(d.getSeconds())}-${++this.taken}`,   // (the count: two in one second are two files)
      out: this.host.out(), frames: [], held: [], skipped: new Map() };
    this.armed = null; return this.say(`● capture: ${c.frames} frame${c.frames > 1 ? 's' : ''}, ${c.whole ? 'the whole picture' : `${c.px} px at the cursor`}`, 60);
  }
  /** capture from the frame the renderer next changes something on (`on`: only these kinds, e.g. ['long'] for a long frame; none = any change of its state) */
  arm(o: Partial<CapOpts> = {}, on: string[] | null = null): string { if (this.active) return this.say('a capture is already under way'); this.armed = { o: this.opts(o), on }; return this.say(`capture armed: it starts when ${on ? on.join(' / ') : 'the renderer changes anything'}`, 3600); }
  /** put down whatever is armed or being taken (frames already copied are let go; a capture already being read back finishes) */
  stop(): string { const r = this.run; this.armed = null; this.run = null; if (r) free(r); return this.say(r ? 'capture stopped' : 'capture not armed'); }
  private say(s: string, secs = 8): string { this.status = s; this.statusUntil = performance.now() + secs * 1000; try { this.host.said?.(s); } catch { /* the page's line is not ours to fail on */ } return s; }
  private failed(r: Run, e: unknown): void { free(r); const s = this.say(`capture failed: ${(e as Error)?.message ?? e}`, 20); console.error(`[framecap] ${s}`); }

  /** once a drawn frame, after it is submitted: the wall clock in ms (a frame's length is the time since the last one) and the CPU's milliseconds in the renderer */
  frame(t: number, cpuMs: number): void {
    this.n++; const ms = this.lastT > 0 ? t - this.lastT : 0; this.lastT = t;
    const s = this.host.state(); let hit = '';
    if (this.last) for (const k in s) if (s[k] !== this.last[k]) { this.note(k, `${this.last[k]} → ${s[k]}`, t); if (!this.armed?.on || this.armed.on.includes(k)) hit ||= k; }
    this.last = s;
    if (this.ema > 0 && ms > Math.max(50, 2.5 * this.ema)) { this.note('long', `${ms.toFixed(0)} ms against ${this.ema.toFixed(1)}`, t); if (this.armed?.on?.includes('long')) hit ||= 'long'; }
    this.ema = this.ema > 0 ? this.ema + (Math.min(ms, 4 * this.ema) - this.ema) * 0.05 : ms;
    const i = this.n % RING; this.rt[i] = t; this.rdt[i] = ms; this.rcpu[i] = cpuMs;
    if (this.armed && hit && !this.active) { const a = this.armed; this.start({ ...a.o, why: `armed: ${hit} ${this.events[this.events.length - 1]?.text ?? ''}` }); }
    const r = this.run; if (!r) return;
    try { this.copy(r, t, ms, cpuMs, s); } catch (e) { this.run = null; this.failed(r, e); return; }
    if (r.frames.length >= r.o.frames) { this.run = null; this.busy = true; void this.finish(r).catch(e => this.failed(r, e)).finally(() => { this.busy = false; }); }
  }

  trace(): TraceDump {
    const k = Math.min(this.n, RING), first = this.n - k + 1, t: number[] = [], dt: number[] = [], cpu: number[] = [];
    for (let f = first; f <= this.n; f++) { const i = f % RING; t.push(Math.round(this.rt[i])); dt.push(+this.rdt[i].toFixed(2)); cpu.push(+this.rcpu[i].toFixed(2)); }
    return { events: this.events.slice(), first, t, dt, cpu };
  }

  private copy(r: Run, t: number, dt: number, cpu: number, state: Record<string, StateV>): void {
    const [ow, oh] = r.out, win: CapWindow = { u: r.o.at[0], v: r.o.at[1], px: r.o.px, whole: r.o.whole };
    const fr: CapFrame = { n: this.n, t: Math.round(t), dt: +dt.toFixed(2), cpu: +cpu.toFixed(2), state, gpu: this.host.gpu?.(), entries: [] };
    const want: { tex: GPUTexture; e: CapEntry; bpr: number; rows: number; depth: boolean }[] = []; const seen = new Map<string, number>();
    for (const tex of liveTextures()) {
      const label = tex.label || `(unnamed ${tex.format})`, skip = (why: string) => { r.skipped.set(`${label} ${tex.width}×${tex.height}${tex.depthOrArrayLayers > 1 ? `×${tex.depthOrArrayLayers}` : ''} ${tex.format}`, why); };
      if (this.host.parked?.(tex)) continue;
      if (tex.dimension !== '2d' || tex.depthOrArrayLayers > 1) { skip('a volume or a stack of layers'); continue; }
      if (tex.sampleCount > 1) { skip('multisampled'); continue; }
      const bpp = TEXEL_BYTES[tex.format]; if (!bpp) { skip('a format with no row here'); continue; }
      if (!(tex.usage & GPUTextureUsage.COPY_SRC)) { skip('made without COPY_SRC'); continue; }
      const depth = tex.format === 'depth32float';   // (a depth target is copied whole or not at all: its window is cut on the CPU)
      const rect = cropRect(tex.width, tex.height, ow, oh, win);
      if (!win.whole && rect[2] === tex.width && rect[3] === tex.height && tex.width * tex.height * bpp > PLAIN_MAX) { skip('not shaped like the picture, and big'); continue; }
      const k = (seen.get(label) ?? 0) + 1; seen.set(label, k);
      const e: CapEntry = { label: k > 1 ? `${label}#${k}` : label, id: ids.get(tex) ?? 0, format: tex.format, full: [tex.width, tex.height], rect, at: 0, bytes: rect[2] * rect[3] * bpp };
      const cw = depth ? tex.width : rect[2], rows = depth ? tex.height : rect[3];
      want.push({ tex, e, bpr: Math.ceil(cw * bpp / 256) * 256, rows, depth }); fr.entries.push(e);
    }
    const enc = this.device.createCommandEncoder({ label: 'framecap' }), held: Held[] = [];
    for (let i = 0; i < want.length;) {
      let size = 0, j = i; while (j < want.length && (j === i || size + want[j].bpr * want[j].rows <= CHUNK)) { size += want[j].bpr * want[j].rows; j++; }
      const buf = this.device.createBuffer({ label: 'framecap', size, usage: GPUBufferUsage.COPY_DST | GPUBufferUsage.MAP_READ }), h: Held = { buf, items: [] }; let at = 0;
      for (; i < j; i++) {
        const w = want[i], [x, y, cw, ch] = w.e.rect;
        enc.copyTextureToBuffer(w.depth ? { texture: w.tex, aspect: 'depth-only' } : { texture: w.tex, origin: [x, y] }, { buffer: buf, offset: at, bytesPerRow: w.bpr, rowsPerImage: w.rows }, w.depth ? [w.tex.width, w.tex.height] : [cw, ch]);
        h.items.push({ e: w.e, at, bpr: w.bpr, srcW: w.depth ? w.tex.width : cw, srcH: w.rows }); at += w.bpr * w.rows;
      }
      held.push(h);
    }
    this.device.queue.submit([enc.finish()]);
    r.frames.push(fr); r.held.push(held);
  }

  private async finish(r: Run): Promise<void> {
    this.say(`capture: reading ${r.frames.length} frame${r.frames.length > 1 ? 's' : ''} back`, 120);
    const blobs: Uint8Array[] = []; let at = 0;
    for (const held of r.held) for (const h of held) {
      await h.buf.mapAsync(GPUMapMode.READ); const src = new Uint8Array(h.buf.getMappedRange());
      for (const it of h.items) {
        const [x, y, cw, ch] = it.e.rect, bpp = it.e.bytes / (cw * ch), whole = it.srcW !== cw || it.srcH !== ch, out = new Uint8Array(it.e.bytes);
        for (let row = 0; row < ch; row++) { const s = it.at + (whole ? y + row : row) * it.bpr + (whole ? x * bpp : 0); out.set(src.subarray(s, s + cw * bpp), row * cw * bpp); }
        it.e.at = at; at += out.length; blobs.push(out);
      }
      h.buf.unmap(); h.buf.destroy(); h.items = [];
    }
    r.held = [];
    const meta: CapMeta = { version: 1, name: r.name, why: r.o.why, page: typeof location !== 'undefined' ? location.pathname + location.search : '', adapter: this.host.adapter, out: r.out,
      window: { u: r.o.at[0], v: r.o.at[1], px: r.o.px, whole: r.o.whole }, frames: r.frames, skipped: [...r.skipped].map(([label, why]) => ({ label, why })), trace: this.trace() };
    const gz = new Uint8Array(await new Response(new Blob(captureParts(meta, blobs) as BlobPart[]).stream().pipeThrough(new CompressionStream('gzip'))).arrayBuffer());   // (the parts streamed: a whole 4K frame is some 300 MB of texels, not to be held three times)
    const where = await this.host.save(r.name, gz);
    this.say(`capture ${r.name}: ${r.frames.length} frame${r.frames.length > 1 ? 's' : ''}, ${r.frames[0]?.entries.length ?? 0} targets, ${(gz.length / 1048576).toFixed(1)} MB ${where}`, 30);
    console.info(`[framecap] ${this.status}${r.skipped.size ? ` · not taken: ${[...r.skipped.keys()].join(', ')}` : ''} · read it with: bun run tools/qa/capture-read.ts captures/${r.name}.tpscap.gz`);
  }
}
