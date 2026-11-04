// WebGPU device bootstrap + small helpers.
import { shaderText } from './wgslTrim';
import { reportError, reportWarning, reportWarningLine } from '../cloud/report';   // (a leaf that imports nothing: the line is printed as before and is one of the visit's records)
import { useShaderRow } from '../ui/shaderrows';
import { NoGpuError, adapterFallback, adapterName, noGpuWords, pickAdapter, type NoGpuFacts } from './adapter';
import { KEPT, type Keeper, type Lease } from './keep';   // (a leaf: the kept page's shell and its leases)
export interface Gpu {
  device: GPUDevice;
  adapterInfo: string;
  /** the adapter is the browser's software fallback: no GPU does the work (absent where the browser does not say) */
  adapterFallback?: boolean;
  canvas: HTMLCanvasElement;
  ctx: GPUCanvasContext;
  format: GPUTextureFormat;
  /** the canvas's texture may be sampled by a pass (initGpu `canvasRead`) */
  canvasRead?: boolean;
  hasTimestamps: boolean;
  linearSampler: GPUSampler;
}

// ---- A LOST DEVICE. The browser takes a page's device away in two very different ways. One is the page's own end: a load to another address, a closed tab, the
// Team & kit picture's window dropped — the device is 'destroyed' (Chrome: 'A valid external Instance reference no longer exists.') and nothing is wrong. The other is
// a real loss: the driver fell over, the GPU was reset or switched, the GPU process died. The first used to put the red card up on a page that was already leaving
// (and print an error for every kit picture closed); the second put the same card up and left it there, though a fresh load of the page is all it takes. Now: the
// first says nothing; the second loads the page again ONCE, and the card goes up — with the adapter, the browser's reason and how far the boot had got on it, so a
// report of it can be read — for a second loss inside a minute, or for a third in this tab however far apart (a machine that loses the device every load must reach
// the card, not load for ever). `?gpuretry=0` = the card at once for either, as it was.
export const GPU_LOST = { on: true, /** a second loss this soon after a retry is the card's */ windowMs: 60_000, /** retries a tab gets before every loss is the card's */ tries: 2, key: 'twopointsix.gpuLost' };
try { if (new URLSearchParams(location.search).get('gpuretry') === '0') GPU_LOST.on = false; } catch { /* no page (the harness) */ }
/** what the card says beside the browser's message: the adapter (initGpu), and where the boot stood (main.ts keeps `stage` up to date; '' once the game is up) */
export const gpuStory = { adapter: '', stage: '', /** this load compiles its shaders a few at a time, and why (A GENTLE SECOND TRY below; '' = it does not) */ gentle: '',
  /** this load draws smoke and the weather's air with the plain program, and why (PLAIN AIR below; '' = it does not) */ plainAir: '' };
let leaving = false;
try { addEventListener('pagehide', () => { leaving = true; }); addEventListener('beforeunload', () => { leaving = true; }); addEventListener('pageshow', () => { leaving = false; }); } catch { /* no page */ }   // (pageshow: a page handed back as it was, or a leave that was called off, is not leaving)
export type LostAct = 'quiet' | 'reload' | 'card';
/** what a lost device asks for: `reason` as the browser gave it, `leaving` = the page is on its way out, `last` = when this tab last retried (ms, null = never),
 *  `tries` = how many retries this tab has made */
export function lostAct(reason: string | undefined, leaving: boolean, last: number | null, now: number, retry: boolean, on = GPU_LOST.on, tries = last === null ? 0 : 1): LostAct {
  if (!on) return 'card';
  if (leaving || reason === 'destroyed') return 'quiet';
  return retry && tries < GPU_LOST.tries && (last === null || now - last >= GPU_LOST.windowMs) ? 'reload' : 'card';
}
/** 'count:when', as both of this file's notes are stored (the retry's in the tab, the gentle one in the browser); null = none, or not one of them */
function countWhen(raw: string | null): { n: number; at: number } | null { const [n, at] = (raw ?? '').split(':').map(Number); return n >= 1 && at > 0 ? { n, at } : null; }
/** the card's words for a loss: the browser's message, then one line of what this page knows (`retried` = retries this tab made before it) */
export function lostWords(message: string, reason: string | undefined, retried: number | boolean): string {
  const n = Number(retried);
  const more = [gpuStory.adapter, reason ? `reason: ${reason}` : '', gpuStory.stage, gpuStory.gentle ? `shaders compiled ${gentleWords()}` : '', gpuStory.plainAir ? 'smoke and fog drawn the plain way' : '', n > 1 ? `the page was loaded again ${n} times already` : n ? 'the page was loaded again once already' : ''].filter(Boolean).join(' · ');
  return `GPU device lost: ${message}${more ? `\n${more}` : ''}`;
}
function onLost(info: GPUDeviceLostInfo, retry: boolean) {
  // the tab's retries so far: 'count:when'
  let last: number | null = null, tries = 0;
  try { const w = countWhen(sessionStorage.getItem(GPU_LOST.key)); if (w) { tries = w.n; last = w.at; } } catch { /* no store: no retry is remembered, so none is made */ retry = false; }
  const act = lostAct(info.reason, leaving, last, Date.now(), retry, GPU_LOST.on, tries);
  if (act === 'quiet') { console.info(`WebGPU device gone (${info.reason ?? 'no reason given'}): ${info.message}`); return; }
  // (what the browser had in hand when the device went, and whether the next load of this browser is a gentle one — A GENTLE SECOND TRY below. `act` cannot say
  // that the loss is real: under ?gpuretry=0 the page's own end is the card's too)
  const flying = lostFlying(true), real = !leaving && info.reason !== 'destroyed';
  const facts = { reason: info.reason ?? '', ...flyingFacts(flying), ...(gpuStory.gentle ? { gentle: GENTLE.flight } : {}), ...(gpuStory.plainAir ? { plainAir: 1 } : {}) };
  // (… and whether it draws its smoke and fog the plain way — PLAIN AIR below: the one program in hand was the volumetric pass's)
  let plainNext = false;   // (… said on the card only once the note is written: a store that refuses it leaves the next load as this one was)
  if (plainAirNote(real, retry, flying, !!gpuStory.plainAir)) try { localStorage.setItem(PLAIN_AIR.key, gentleWrite(plainAsked.note, Date.now(), PLAIN_AIR.forget)); plainNext = true; } catch { /* no store */ }
  if (gentleNote(real, retry, compilingNow(flying), !!gpuStory.gentle)) try { localStorage.setItem(GENTLE.key, gentleWrite(gentleStored(), Date.now())); } catch { /* no store: the next load is as this one was */ }
  reportError('gpu.device.lost', 'WebGPU device lost: ' + info.message, facts);
  if (act === 'reload') { try { sessionStorage.setItem(GPU_LOST.key, `${tries + 1}:${Date.now()}`); location.reload(); return; } catch { /* the card instead */ } }
  showFatal(GPU_LOST.on ? lostWords(info.message, info.reason, retry ? tries : 0) + (plainNext ? `\n${PLAIN_AIR.nextWords}` : '') : 'GPU device lost: ' + info.message, { report: { message: 'GPU device lost: ' + info.message, stage: 'device', data: { ...facts, retried: retry ? tries : 0, ...(plainNext ? { plainNext: 1 } : {}) } } });
}

/** The adapter and the device, as this page opens them. `gpu` = whose (a kept page's shell opens them with its own, so they outlive a frame's load: gpu/keep.ts) */
export async function openDevice(gpu: GPU | null = 'gpu' in navigator ? navigator.gpu : null): Promise<{ adapter: GPUAdapter; device: GPUDevice; asks: unknown[] }> {
  // (the fast card, else whatever an ask with no preference gives, else an error that holds what the machine said — gpu/adapter.ts)
  const { adapter, asks } = await pickAdapter(gpu, undefined, undefined, line => console.info(line));
  const want: GPUFeatureName[] = ['timestamp-query', 'float32-filterable'];
  const feats = want.filter(f => adapter.features.has(f));
  const lim = adapter.limits;
  const device = await adapter.requestDevice({
    requiredFeatures: feats,
    requiredLimits: {
      maxStorageBufferBindingSize: Math.min(lim.maxStorageBufferBindingSize, 1 << 30),
      maxBufferSize: Math.min(lim.maxBufferSize, 1 << 30),
      maxComputeInvocationsPerWorkgroup: lim.maxComputeInvocationsPerWorkgroup,
      maxComputeWorkgroupSizeX: lim.maxComputeWorkgroupSizeX,
      maxStorageBuffersPerShaderStage: lim.maxStorageBuffersPerShaderStage,
      maxStorageTexturesPerShaderStage: lim.maxStorageTexturesPerShaderStage,
      maxSampledTexturesPerShaderStage: lim.maxSampledTexturesPerShaderStage,
      maxColorAttachmentBytesPerSample: lim.maxColorAttachmentBytesPerSample,
    },
  });
  return { adapter, device, asks };
}
/** the shell's device for this load (gpu/keep.ts), or null: no shell above, or it could not be had. A start with no GPU is this page's own error, so its card
 *  reads as it does on any page (the shell's is an error of another window's making) */
async function takeKept(keeper: Keeper | null | undefined): Promise<Lease | null> {
  if (!keeper) return null;
  try { return await keeper.take(); }
  catch (e) {
    const x = e as { name?: string; message?: string; facts?: NoGpuFacts } | undefined;
    if (x?.name === 'NoGpuError' && x.facts) throw new NoGpuError(String(x.message), x.facts);
    reportWarningLine('gpu.keep.take', 'the kept page\'s device could not be had; this load opens its own —', e); return null;
  }
}
/** `retry`: a real loss of the device loads the page again once before the card goes up (the game page asks for it; a tool page with work in it does not).
 *  `canvasRead`: the canvas's own texture may be SAMPLED by a pass as well as drawn into (render/savepic.ts boxes a frame down to a save's picture on the GPU);
 *  not asked = the canvas configured exactly as it always was. `keeper`: the shell above a kept page's frame (gpu/keep.ts): the device is its, on a lease that
 *  ends with this load; none = a device of this page's own, as it always was. */
export async function initGpu(canvas: HTMLCanvasElement, opts: { retry?: boolean; canvasRead?: boolean; keeper?: Keeper | null } = {}): Promise<Gpu> {
  let lease = await takeKept(opts.keeper);
  let opened = lease ?? await openDevice();
  const ctx = canvas.getContext('webgpu') as GPUCanvasContext;
  const format = navigator.gpu.getPreferredCanvasFormat();
  const conf = (device: GPUDevice) => ctx.configure({ device, format, alphaMode: 'opaque', ...(opts.canvasRead ? { usage: GPUTextureUsage.RENDER_ATTACHMENT | GPUTextureUsage.TEXTURE_BINDING } : {}) });
  // (a browser that will not draw into this window's canvas with another window's device says so here: the lease is handed back, the shell told, and this
  // load and the ones after it open their own — the page as it was, in a frame)
  if (lease) try { conf(lease.device); } catch (e) { reportWarningLine('gpu.keep.refused', 'this browser would not draw with the kept page\'s device; every load opens its own —', e); lease.release(); opts.keeper!.refused = true; lease = null; opened = await openDevice(); }
  const { adapter, device, asks } = opened;
  if (lease) { KEPT.lease = lease; const mine = lease; addEventListener('pagehide', ev => { if (ev.isTrusted && !(ev as PageTransitionEvent).persisted) mine.release(); }); } else conf(device);
  watchPipes(device);
  if (PIPE_QUEUE.on) installPipeQueue(device);
  if (gpuStory.gentle) reportWarning('gpu.gentle', `shaders compile ${gentleWords()} on this load`, { why: gpuStory.gentle, flight: GENTLE.flight });   // (one of the visit's records: a load that got through this way can be told from one that never needed it)
  // (PLAIN AIR below: decided here, where the adapter is first known and before any pass asks which program its march is)
  { let ua = ''; try { ua = navigator.userAgent; } catch { /* no page */ }
    const now = Date.now(), row = plainAirRow(ua, adapterName(adapter)), note = plainAsked.note;
    gpuStory.plainAir = plainAirFor(plainAsked.word, note, now, row);
    if (gpuStory.plainAir) reportWarning('gpu.plainair', 'smoke and fog are drawn the plain way on this load', { why: gpuStory.plainAir, ...(row ? { row } : {}), ...(note ? { noted: note.n, hours: Math.round((now - note.at) / 3_600_000) } : {}) }); }
  device.lost.then(info => onLost(info, !!opts.retry));
  device.addEventListener('uncapturederror', (ev) => { reportError('gpu.uncaptured', 'WebGPU error: ' + (ev as GPUUncapturedErrorEvent).error.message); });
  const feats = [...device.features] as GPUFeatureName[];
  const info = adapter.info;
  // (a start the second ask gave says so in the adapter's own line: the benchmark's report and a visit's record carry that line, so such a machine can be found)
  gpuStory.adapter = adapterName(adapter) + (asks.length > 1 ? ' (given to the second ask)' : '');
  return {
    device, canvas, ctx, format, canvasRead: !!opts.canvasRead,
    adapterInfo: gpuStory.adapter,
    adapterFallback: adapterFallback(adapter),
    hasTimestamps: feats.includes('timestamp-query'),
    linearSampler: device.createSampler({ magFilter: 'linear', minFilter: 'linear', mipmapFilter: 'nearest', addressModeU: 'clamp-to-edge', addressModeV: 'clamp-to-edge', addressModeW: 'clamp-to-edge' }),
  };
}

/** WHO IS TOLD OF EVERY RED CARD (the backend's side sets `tell` in main.ts where the build has one; nobody else): what went up, and the card, so a line can be
 *  put on it. `stage`: 'adapter' = a start with no GPU, 'device' = a lost device; none = any other throw. `data` is plain JSON. A listener that throws is dropped */
export interface FatalReport { message: string; stack?: string; stage?: string; where?: string; adapter?: string; data?: unknown }
export const FATAL: { tell: ((r: FatalReport, card: HTMLElement) => void) | null } = { tell: null };
/** `more.details`: lines under the message that the player can copy with a button (what the machine said: a start with no GPU). `more.report`: what the
 *  listener is told in place of the card's own words */
export function showFatal(msg: string, more?: { details?: string; report?: Partial<FatalReport> }) {
  const el = document.getElementById('fatal'); if (!el) return;
  el.style.display = 'flex'; el.style.flexDirection = ''; el.style.gap = ''; el.textContent = msg;
  if (more?.details) try {
    const details = more.details;
    el.textContent = ''; el.style.flexDirection = 'column'; el.style.gap = '16px';
    const say = document.createElement('div'), pre = document.createElement('div'), b = document.createElement('button');
    say.textContent = msg; say.style.cssText = 'max-width:64ch;line-height:1.5';
    // (a layer the text size zooms: a window-height length is divided by the zoom — ui/textsize.ts)
    pre.textContent = details; pre.style.cssText = 'max-width:min(96ch,100%);max-height:calc(40vh / var(--tz, 1));overflow:auto;text-align:left;font:12px/1.5 ui-monospace,Menlo,Consolas,monospace;color:#e9b8b8;user-select:text;-webkit-user-select:text;overflow-wrap:anywhere';
    b.type = 'button'; b.textContent = 'Copy'; b.style.cssText = 'font:inherit;color:#100;background:#fcc;border:0;border-radius:3px;padding:6px 18px;cursor:pointer';
    const text = `${msg.split('\n')[0]}\n${details}`;
    b.onclick = () => {
      // INSIDE the click, the way every browser allows: the whole text in a field of its own, selected and copied. Only where that is refused is the clipboard
      // asked (it answers later); and where both are, the lines are left selected for the player's own copy key
      let ok = false;
      try { const t = document.createElement('textarea'); t.value = text; t.readOnly = true; t.style.cssText = 'position:fixed;left:0;top:0;width:1px;height:1px;opacity:0'; el.appendChild(t); t.select(); ok = document.execCommand('copy'); t.remove(); } catch { /* the clipboard next */ }
      const select = () => { try { const r = document.createRange(); r.selectNodeContents(pre); const s = getSelection(); s?.removeAllRanges(); s?.addRange(r); } catch { /* nothing to select with */ } };
      const failed = () => { select(); b.textContent = "Your browser wouldn't copy it. The lines are selected, so copy them with your keyboard."; };
      if (ok) { b.textContent = 'Copied'; return; }
      try { navigator.clipboard.writeText(text).then(() => { b.textContent = 'Copied'; }, failed); } catch { failed(); }
    };
    el.append(say, pre, b);
  } catch { el.textContent = `${msg}\n\n${more.details}`; }
  const tell = FATAL.tell; if (!tell) return;
  try { tell({ message: msg.split('\n')[0].slice(0, 400), stack: msg, ...(gpuStory.stage ? { where: gpuStory.stage } : {}), ...(gpuStory.adapter ? { adapter: gpuStory.adapter } : {}), ...more?.report }, el); } catch { /* the card is up: that is what matters */ }
}
/** the card for a start that threw: a start with no GPU says what the machine said, anything else its stack as before */
export function showFatalError(e: unknown) {
  if (e instanceof NoGpuError) { const w = noGpuWords(e.message, e.facts); showFatal(w.say, { details: w.details, report: { message: e.message, stack: e.stack, stage: 'adapter', data: e.facts } }); return; }
  const err = e as Error | undefined;
  showFatal(String(err?.stack ?? e), { report: { message: String(err?.message ?? e).slice(0, 400), ...(err?.stack ? { stack: err.stack } : {}) } });
}

export function makeBuffer(device: GPUDevice, size: number, usage: GPUBufferUsageFlags, label?: string): GPUBuffer {
  return device.createBuffer({ size: Math.max(16, (size + 3) & ~3), usage, label });
}
export function makeBufferWithData(device: GPUDevice, data: ArrayBufferView, usage: GPUBufferUsageFlags, label?: string): GPUBuffer {
  const buf = device.createBuffer({ size: Math.max(16, (data.byteLength + 3) & ~3), usage: usage | GPUBufferUsage.COPY_DST, label });
  device.queue.writeBuffer(buf, 0, data.buffer, data.byteOffset, data.byteLength);
  return buf;
}
/** queue.writeBuffer(buf, offset, now's first `words` words) — unless `sent`, the caller's own record of what that range of the buffer holds word for word, already says so
 *  (then nothing is issued; true = it wrote). Only for a range NOTHING else writes, the GPU included, on a buffer that is never re-made under the record: a fresh buffer is all
 *  zeros and so is a fresh record. Words are compared as bits (a float's NaN or −0 is just a pattern). */
export function writeWordsIfChanged(device: GPUDevice, buf: GPUBuffer, offset: number, now: Uint32Array, sent: Uint32Array, words = now.length): boolean {
  let w = 0; while (w < words && now[w] === sent[w]) w++;
  if (w === words) return false;
  device.queue.writeBuffer(buf, offset, now.buffer, now.byteOffset, words * 4);
  for (; w < words; w++) sent[w] = now[w];
  return true;
}
// ---- ASYNC PIPELINE CREATION (the boot path). A synchronous createComputePipeline() returns at once on the page's thread, but Chrome's GPU process then
// compiles the kernel (WGSL → MSL → the Metal compiler) on ITS main thread the moment the first submit needs it — the thread that also composites the
// browser's own UI. Ninety-odd kernels with the whole scene prelude on a cold shader cache froze all of Chrome for seconds on first load (the maintainer's
// 'big hang when loading the page for the first time'). The …Async() forms compile on the GPU process's worker threads and resolve when the pipeline is
// READY, so nothing that reaches a submit ever waits on a compiler and the browser stays live while the start card counts them in. Every module keeps its
// layouts / buffers / bind-group layouts in its (cheap, synchronous) constructor and creates its pipelines in an `async init()` through these two helpers.
// A creation that REJECTS (a compile / validation failure) is logged and falls back to the synchronous call — which hands back the same invalid object the
// old code got, so a broken optional kernel degrades to the validation errors it always produced instead of a dead boot.
/** counts pipelines requested / resolved through the helpers below since the last reset — the start card's 'compiling shaders k / N' line and window.__boot read it */
export const pipelineProgress = {
  total: 0, done: 0, failed: 0,
  /** per pipeline, ms from the browser being asked for it to the pipeline being ready (with THE COMPILE QUEUE below that is when its turn came, not when the game asked: the wait in
   *  the queue is not a compile; several are still in flight at once, so a time includes the workers it shared) — the boot line's and the benchmark's 'slowest' */
  times: [] as { label: string; ms: number }[],
  /** called after every resolution (main.ts repaints the boot card's status line off it) */
  onChange: null as null | ((done: number, total: number, label: string) => void),
  reset() { this.total = 0; this.done = 0; this.failed = 0; this.times.length = 0; },
};
/** when the queue handed a descriptor to the browser (performance.now()); none for a creation that never waited in it */
const pipeStarted = new WeakMap<object, number>();
function track<T>(label: string, p: Promise<T>, desc: object): Promise<T> {
  const P = pipelineProgress; P.total++; const t = typeof performance !== 'undefined' ? performance.now() : 0;
  return p.then(v => { P.done++; if (P.times.length < 512 && typeof performance !== 'undefined') P.times.push({ label, ms: performance.now() - (pipeStarted.get(desc) ?? t) }); P.onChange?.(P.done, P.total, label); return v; });
}
// ---- THE COMPILE QUEUE (2026-10-26, measured on a Windows machine: RTX 3080, Ryzen 3700X, Chrome 155, D3D12 + DXC). Chrome compiles every …Async pipeline on ONE pool
// of worker threads in its GPU process (15 workers on 16 hardware threads), and that pool is SHARED with Chrome's own drawing of the page: Skia Graphite rasterizes the
// HTML (the HUD, the cards, the menus) through Dawn and makes its little pipelines on the same workers. On D3D12 a big kernel of ours is a DXC compile of 5–25 s (the
// gathers, the march, the composites: everything in them is inlined into one function), so a burst of twenty of them holds every worker for that long — and Graphite's
// pipelines, asked for while the burst runs, wait behind it. The page then cannot be drawn at all: the trace of a played floor showed the raster's flush blocked 3–7 s at a
// time while the second (cut-box) set compiled after the boot, the picture frozen with it (15 s in one take). Metal compiles in a fraction of that, so the burst never
// starved anything on a Mac. So every …Async creation goes through this queue: at most `boot` in flight while the start card counts the boot's pipelines in (biggest module
// first, so the slowest compiles are not the last to start), and after the boot (`pipeQueue.settle()`, main.ts) at most `after` in flight, of which at most `low` are the
// background set's (modules compiled behind the cut prelude: `const CUT_BOXES: bool = true;`) — first come first served among the rest, so a pass's first-use pipeline
// is not stuck behind twenty background twins. A switch of the page, not a rule of the game (no knob, no FROZEN_HOLDS row: every stand compiles the same programs, only
// in a different order); `?pipequeue=0` = every creation straight to the browser, as it was.
// ON ONLY WHERE IT WAS MEASURED: a browser on Windows (`pipeQueueFor`). Anywhere else the compiles are quick and nothing was ever starved, while the queue has a price:
// the background set is ready later (on the software adapter the page tools run on it was not in 21 minutes after the page was), and the first use of a pass waits its
// turn. So a Mac and the tools get every creation straight to the browser, as they always did, until the queue is measured there. `?pipequeue=1` = on wherever the page
// is (how a Mac or a tool tries it); a number over 1 = on, with that many in flight at the boot.
export const PIPE_QUEUE = { on: false, /** in flight while the boot's count is open (0 = the machine's threads less two, at least 4) */ boot: 0, /** in flight after the boot */ after: 4, /** …of which background (cut-set) compiles */ low: 2,
  /** LATER (2026-10-26): the twins no first frame draws with the default settings (laterPipelineAsync below) are left out of the boot and compiled after it in the background lane; false = the boot waits for them too, as it always did. The row 'Start on first frame shaders' (ui/shaderrows.ts, `?firstframe`: on where a person sits, off in the harness and on a page a script drives), read once at import; the Team & kit picture and the workbench lower it before they build (main.ts, workbench/main.ts) */ later: useShaderRow('firstFrame') };
/** whether the queue is on for a browser that calls itself `userAgent`, given the page's `?pipequeue` word (null = none) */
export function pipeQueueFor(userAgent: string, word: string | null): boolean { return word === '0' ? false : word !== null && /^\d+$/.test(word) ? true : /\bWindows\b/.test(userAgent); }
try { const v = new URLSearchParams(location.search).get('pipequeue'); PIPE_QUEUE.on = pipeQueueFor(navigator.userAgent, v); if (v && /^\d+$/.test(v) && +v > 1) PIPE_QUEUE.boot = +v; } catch { /* no page (the harness) */ }
// ---- A GENTLE SECOND TRY (2026-10-29, one machine's records from the hosted build: a Radeon RX 9070 XT on Windows, Chrome 156, 16 threads). Its device was lost three
// times in a minute ('A valid external Instance reference no longer exists.', reason unknown: the browser's GPU process had gone): once 7.5 s into the boot's compiles with
// 17 of 105 in, once 8.1 s into a mission's eight with the five heaviest kernels still out, after which the browser gave the page no adapter at all. One load between them
// compiled all 105 in 10 s. So the same programs come through or do not, and what was in hand each time was the burst the queue allows at the boot: the machine's threads
// less two, fourteen there. WHY the process goes is NOT known (its memory, or the driver under many compiles at once, are guesses): nothing here was seen on that machine.
// What the page can do blind: a browser that loses its device WHILE SHADERS COMPILE notes it (localStorage: how many times now, and when), and every load of that
// browser for a while after puts the queue on wherever it is and hands the browser `flight` compiles at a time, the boot, a mission's start and the background set
// alike. For `days` after a first such loss, so that one loss a healthy machine had for its own reasons (a driver update, a sleep) costs it a day of slow loads and no
// more; for `daysAgain` once the burst has lost the device again within `forget` days, which is a machine saying it every time. A loss on a load that was ALREADY
// gentle writes nothing: it is no word on the burst. Slower by the heavy kernels' sum (a minute where the burst took ten seconds), on a machine whose other choice
// was the red card. The lost device's report names what was in hand (`flying`), so a gentle load that is lost too says which program it was. The card and a warning
// record ('gpu.gentle') say when a load is gentle. `?gentle=0` = no note is written or read, as it was; `?gentle=1` = this load is gentle. A `?pipequeue` word is the
// page's own say and wins. Not noted on a page that asked for no retry (a benchmark, the kit picture, a page a script drives: a tool's page losing the software
// adapter must not slow the next tool's); such a page still READS the note, so a benchmark on a noted browser times a gentle load and its records say so.
export const GENTLE = { on: true, /** compiles in flight at once on a gentle load */ flight: 1, /** how long a first such loss keeps a browser gentle */ days: 1, /** … and one that came again */ daysAgain: 30, /** a loss this long after the last one is a first again */ forget: 180, key: 'twopointsix.gentle' };
/** the browser's note: how many losses while compiling there have been, and when the last was (ms) */
export interface GentleNote { n: number; at: number }
/** the note as the store holds it ('count:when'); null = none, or not one of ours */
export function gentleRead(raw: string | null): GentleNote | null { return countWhen(raw); }
/** the note a loss leaves over `was` */
export function gentleWrite(was: GentleNote | null, now: number, forget = GENTLE.forget): string { return `${was && now >= was.at && now - was.at < forget * 86_400_000 ? Math.min(9, was.n + 1) : 1}:${now}`; }
/** whether a note of this kind still holds: `days` after a first loss, `daysAgain` after one that came again (PLAIN AIR's note is read the same way) */
export function noteYoung(note: GentleNote | null, now: number, days: number, daysAgain: number): boolean { return note !== null && now >= note.at && now - note.at < (note.n > 1 ? daysAgain : days) * 86_400_000; }
/** whether a load is gentle: the page's `?gentle` word, else the browser's note if it is young enough for its count */
export function gentleFor(word: string | null, note: GentleNote | null, now: number, on = GENTLE.on): boolean {
  if (!on || word === '0') return false;
  return word === '1' || noteYoung(note, now, GENTLE.days, GENTLE.daysAgain);
}
/** whether a lost device leaves the note: a `real` loss (not the page's own end) on a page that retries, while shaders were compiling, on a load that was not `gentle` already */
export function gentleNote(real: boolean, retry: boolean, compiling: boolean, gentle: boolean, on = GENTLE.on): boolean { return on && real && retry && compiling && !gentle; }
/** the queue as a gentle load has it (exported for tools/qa/pipequeue-check.ts) */
export function gentleQueue(why: string) { PIPE_QUEUE.on = true; PIPE_QUEUE.boot = PIPE_QUEUE.after = PIPE_QUEUE.low = GENTLE.flight; gpuStory.gentle = why; }
function gentleWords(): string { return GENTLE.flight === 1 ? 'one at a time' : `${GENTLE.flight} at a time`; }
function gentleStored(): GentleNote | null { try { return gentleRead(localStorage.getItem(GENTLE.key)); } catch { return null; /* no store: no note */ } }
try {
  const q = new URLSearchParams(location.search), w = q.get('gentle'); if (w === '0') GENTLE.on = false;
  const note = gentleStored();
  if (q.get('pipequeue') === null && gentleFor(w, note, Date.now())) gentleQueue(w === '1' ? 'the page was asked to (?gentle=1)' : `this browser lost its GPU device while they compiled, ${Math.round((Date.now() - note!.at) / 3_600_000)} h ago (${note!.n} such so far)`);
} catch { /* no page (the harness) */ }
// ---- PLAIN AIR (2026-10-30, the same machine's second visit, on the build with the gentle try in it). The burst lost its device again (six programs in hand), the load
// after it was gentle, and the device was lost again with five programs made and ONE in hand: `vol`, the volumetric pass's march (shaders/volumetrics.wgsl: the smoke's
// march with its shadow rays, the haze's lamp cones, the rain, the weather's air). `vol` was in hand at every loss of that machine whose record names anything. So it is
// not how many compile at once: that driver's process goes while it compiles that program. NOT known: why, and whether it is that program alone (the two gathers behind
// it in the order were never tried by themselves; one title load of the first visit did make all 105). Nothing can be seen of it here.
// What the page can do blind: not hand that driver the program. A load in plain air gives the volumetric pass shaders/vol_plain.wgsl in its place (render/passes.ts
// traceJobs): the smoke's density marched and lit by the radiance cache alone, the weather's air the same way, not one ray in it; the froxel twin, the dust twin and the
// K = 0 twin of the real march are not made at all, so every tier draws with the one plain program and dust is drawn as plain smoke. What is lost there is look: no
// self-shadowed, lamp-lit plume, no rain streaks or splashes from this pass, no lit sky under lightning, and on the tiers whose haze is this pass's (high, ultra) no
// shafts from a lamp or the sun (the froxel tiers' shafts are the froxel kernels' and stay). What is kept is what the game is played by: smoke and a weather's air stand where the men's eyes are cut by them.
// A load is in plain air when (1) the page says so (`?plainair=1`), (2) this browser noted it: a real loss, on a page that retries, with exactly ONE program in hand and
// that one of the pass's (`labels`) — the note is the gentle one's kind ('count:when'): it holds `days` after a first such loss, so that a device a healthy machine
// lost for its own reasons with that program in hand costs it a day of plain smoke, and `daysAgain` once the real march has lost the device again within `forget`
// days; after that the real march is tried again (a driver may have been mended) — or (3) the adapter is one the records already name (`adapters`: a browser on
// Windows whose adapter's name starts so), which spares such a machine the lost loads the note costs. The row has no such second look: a machine it names is never
// handed the program, so only a `?plainair=0` load of such a machine can say the row is no longer needed. It is a guess from ONE machine.
// On every tier the haze's beams are then the froxel kernels' (main.ts: plain air forces the froxel path as `?froxel` does; `?nofroxel` still wins), since the plain
// program draws none. `?plainair=0` = none of the three, and no note written. The card of the loss that wrote the note says the next load is plain; a plain load says so in a warning
// record ('gpu.plainair') and in its lost device's facts (`plainAir`), so a machine that still loses its device names the NEXT program with this one out of the way.
export const PLAIN_AIR = { on: true, /** how long a first such loss keeps a browser in plain air */ days: 1, /** … and one that came again */ daysAgain: 30, /** a loss this long after the last one is a first again */ forget: 180, key: 'twopointsix.plainAir',
  /** the volumetric pass's programs, by label (render/passes.ts) */ labels: ['vol', 'volFroxel', 'volDust', 'volK0'],
  /** adapters the records name: a browser on Windows whose adapter's name (gpu/adapter.ts adapterName: vendor, architecture, description) starts with one of these */ adapters: ['amd rdna-4'],
  /** the last line of the card of the loss that wrote the note */ nextWords: 'Load the page again and the game will skip the shader your graphics driver stopped on. Smoke and fog will look plainer.' };
/** the row of `adapters` a browser's adapter answers to ('' = none): a browser on Windows, the adapter's name starting with the row */
export function plainAirRow(userAgent: string, adapter: string): string { return /\bWindows\b/.test(userAgent) ? PLAIN_AIR.adapters.find(a => adapter.toLowerCase().startsWith(a)) ?? '' : ''; }
/** why a load is in plain air ('' = it is not; a record's data keeps 80 characters of it): the page's `?plainair` word, else the browser's note while it is young enough for its count, else the adapter's `row` */
export function plainAirFor(word: string | null, note: GentleNote | null, now: number, row: string, on = PLAIN_AIR.on): string {
  if (word === '0' || !on) return '';
  if (word === '1') return 'the page was asked to (?plainair=1)';
  if (noteYoung(note, now, PLAIN_AIR.days, PLAIN_AIR.daysAgain)) return 'this browser lost its device with only the volumetric program in hand';
  return row ? "the adapter's driver is known to stop on the volumetric program" : '';
}
/** whether a lost device leaves the note: a `real` loss on a page that retries, with exactly one program in hand, that one the volumetric pass's, on a load that was not `plain` already */
export function plainAirNote(real: boolean, retry: boolean, inHand: string[], plain: boolean, on = PLAIN_AIR.on): boolean { return on && real && retry && !plain && inHand.length === 1 && PLAIN_AIR.labels.includes(inHand[0]); }
/** this load's volumetric pass is the plain program (read by render/passes.ts when it builds, after initGpu) */
export function plainAir(): boolean { return !!gpuStory.plainAir; }
/** the page's word and the browser's note, read once as the page loads; initGpu decides with them, where the adapter is known */
const plainAsked: { word: string | null; note: GentleNote | null } = { word: null, note: null };
try {
  plainAsked.word = new URLSearchParams(location.search).get('plainair'); if (plainAsked.word === '0') PLAIN_AIR.on = false;
  try { plainAsked.note = gentleRead(localStorage.getItem(PLAIN_AIR.key)); } catch { /* no store: no note */ }
} catch { /* no page (the harness) */ }
/** the pipelines the browser has been handed and has not answered, by label */
const flying: string[] = [];
function fly(label: string): () => void { flying.push(label); let out = false; return () => { if (out) return; out = true; const i = flying.indexOf(label); if (i >= 0) flying.splice(i, 1); }; }
/** the first of a burst of failed creations keeps what was in hand at that moment: a lost device rejects everything in flight BEFORE it says it is lost (the
 *  records: every one of them in the same millisecond as the loss), and the queue then hands the dead device whatever waited, one rejection after another — those
 *  are not what the device was lost with. A burst lasts while failures keep coming within `gapMs` of each other, and is what a loss is told for `keepMs` after its
 *  last (the creations have fallen back by then and nothing is out). `held` = a loss has read it: nothing that fails after that is of any burst */
const fallen = { last: -Infinity, labels: [] as string[], gapMs: 1000, keepMs: 10_000, held: false };
const clock = () => typeof performance !== 'undefined' ? performance.now() : Date.now();
function fell() { if (fallen.held) return; const t = clock(); if (t - fallen.last > fallen.gapMs) fallen.labels = flying.slice(); fallen.last = t; }
/** what the browser had in hand when the device went: the burst that just failed, else whatever is still out. `hold` = the device IS lost (onLost): the answer stands from here */
export function lostFlying(hold = false): string[] {
  if (fallen.held) return fallen.labels.slice();
  const out = clock() - fallen.last <= fallen.keepMs ? fallen.labels.slice() : flying.slice();
  if (hold) { fallen.held = true; fallen.labels = out.slice(); }
  return out;
}
/** the labels as a record keeps them: a report's data holds strings of 80 characters and no lists (cloud/report.ts), so up to three such strings and the count */
export function flyingFacts(labels: string[]): Record<string, unknown> {
  if (!labels.length) return {};
  const rows: string[] = []; let row = '';
  for (const l of labels) { if (row && row.length + 1 + l.length > 80) { rows.push(row); row = ''; if (rows.length === 3) break; } row = row ? `${row} ${l}` : l.slice(0, 80); }
  if (row && rows.length < 3) rows.push(row);
  return { flyingOf: labels.length, flying: rows[0], ...(rows[1] ? { flying2: rows[1] } : {}), ...(rows[2] ? { flying3: rows[2] } : {}) };
}
const watched = new WeakSet<object>();
/** every …Async creation of `device` is counted as in hand from the moment the browser is given it to the moment it answers — beneath the queue, so one that waits
 *  its turn there is not counted until its turn, and whoever calls the device directly is counted like the helpers below. The browser's own answer is handed back
 *  untouched (a throw is a throw, a stand-in's non-promise is itself) */
export function watchPipes(device: GPUDevice) {
  if (watched.has(device)) return; watched.add(device);
  const D = device as unknown as Record<string, unknown>;
  const watch = (make: (desc: object) => unknown) => (desc: object) => {
    const land = fly(String((desc as { label?: string } | null)?.label ?? '?'));
    let p: unknown; try { p = make(desc); } catch (e) { land(); throw e; }
    if (p && typeof (p as Promise<unknown>).then === 'function') (p as Promise<unknown>).then(land, () => { fell(); land(); }); else land();
    return p;
  };
  D.createComputePipelineAsync = watch(device.createComputePipelineAsync.bind(device) as (desc: object) => unknown);
  D.createRenderPipelineAsync = watch(device.createRenderPipelineAsync.bind(device) as (desc: object) => unknown);
}
function compilingNow(inHand: string[]): boolean { return gpuStory.stage.startsWith('while compiling') || inHand.length > 0 || laterProgress.pending > 0 || pipeQueue.waiting.length > 0; }
interface PipeJob { cost: number; low: boolean; later: boolean; seq: number; go: () => void }
/** the descriptors laterPipelineAsync handed the browser: the queue keeps them out of the boot and puts them in the background lane after it */
const laterDescs = new WeakSet<object>();
/** the queue's state (window.__pipeQueue on the page: in flight, waiting, the phase, the most that ever waited) */
export const pipeQueue = {
  phase: 'boot' as 'boot' | 'after', inflight: 0, inflightLow: 0, waiting: [] as PipeJob[], maxWaiting: 0, seq: 0,
  /** NOBODY SEES THE FRAMES (a mission's brief is up and its START not yet live: ui/settle.ts): after the boot the queue still lets as many compile as the
   *  boot did, the background set's among them. What the two smaller numbers spare is the drawing of a page somebody is looking at */
  rushed: false,
  limit(): number { return this.phase === 'boot' || this.rushed ? (PIPE_QUEUE.boot || Math.max(4, ((typeof navigator !== 'undefined' && navigator.hardwareConcurrency) || 8) - 2)) : PIPE_QUEUE.after; },
  /** the boot's pipelines are in: from here at most `after` in flight, `low` of them background */
  settle() { this.phase = 'after'; this.pump(); },
  rush(on: boolean) { if (this.rushed === on) return; this.rushed = on; this.pump(); },
  pump() {
    while (this.waiting.length && this.inflight < this.limit()) {
      let pick = -1;
      for (let i = 0; i < this.waiting.length; i++) {
        const j = this.waiting[i];
        if (j.later && this.phase === 'boot' && (this.inflight > 0 || this.waiting.some(w => !w.later))) continue;   // (a LATER twin never takes a worker from the boot's own pipelines: before settle() only once they are all done — a page that never settles still gets it)
        if (j.low && this.phase === 'after' && !this.rushed && this.inflightLow >= PIPE_QUEUE.low) continue;
        if (pick < 0) { pick = i; continue; }
        const p = this.waiting[pick];
        const better = this.phase === 'boot' ? j.cost > p.cost : (p.low && !j.low) || (p.low === j.low && j.seq < p.seq);   // boot: biggest first · after: the foreground first, then in order
        if (better) pick = i;
      }
      if (pick < 0) return;
      const job = this.waiting.splice(pick, 1)[0]; this.inflight++; if (job.low) this.inflightLow++; job.go();
    }
  },
};
/** exported for tools/qa/pipequeue-check.ts, which drives the queue over a stand-in device */
export function installPipeQueue(device: GPUDevice) {
  watchPipes(device);   // (beneath the queue: what is in hand is counted at a creation's turn)
  const D = device as unknown as Record<string, unknown>;
  const meta = new WeakMap<GPUShaderModule, { cost: number; low: boolean }>();
  const csm = device.createShaderModule.bind(device);
  D.createShaderModule = (desc: GPUShaderModuleDescriptor) => { const m = csm(desc); meta.set(m, { cost: desc.code.length, low: desc.code.includes('const CUT_BOXES: bool = true;') }); return m; };
  const of = (mods: (GPUShaderModule | undefined)[]) => { let cost = 0, low = false; for (const m of new Set(mods)) { const x = m && meta.get(m); if (x) { cost += x.cost; low ||= x.low; } } return { cost, low }; };
  const queued = <T,>(desc: object, mods: (GPUShaderModule | undefined)[], make: () => Promise<T>, later = false): Promise<T> => new Promise<T>((resolve, reject) => {
    const { cost, low } = of(mods); const Q = pipeQueue;
    const job: PipeJob = { cost, low: low || later, later, seq: Q.seq++, go: () => {
      if (desc && typeof desc === 'object' && typeof performance !== 'undefined') pipeStarted.set(desc, performance.now());
      let p: Promise<T>; try { p = Promise.resolve(make()); } catch (e) { p = Promise.reject(e); }   // (a throw or a stand-in's non-thenable answer must not keep the slot)
      p.then(resolve, reject).finally(() => { Q.inflight--; if (job.low) Q.inflightLow--; Q.pump(); });
    } };
    Q.waiting.push(job); Q.maxWaiting = Math.max(Q.maxWaiting, Q.waiting.length);
    queueMicrotask(() => Q.pump());   // (a burst made in one task is all queued before the first pick: the biggest of it starts first)
  });
  const cpa = device.createComputePipelineAsync.bind(device), rpa = device.createRenderPipelineAsync.bind(device);
  D.createComputePipelineAsync = (desc: GPUComputePipelineDescriptor) => queued(desc, [desc?.compute?.module], () => cpa(desc), laterDescs.has(desc));   // (a broken descriptor is the browser's to reject, as it was)
  D.createRenderPipelineAsync = (desc: GPURenderPipelineDescriptor) => queued(desc, [desc?.vertex?.module, desc?.fragment?.module], () => rpa(desc), laterDescs.has(desc));
  try { (window as unknown as Record<string, unknown>).__pipeQueue = pipeQueue; } catch { /* no page */ }
}

/** createComputePipelineAsync with progress accounting; on rejection logs and returns the synchronous (invalid-but-usable-as-before) pipeline. (Promise.resolve round the call: the QA
 *  harness's Proxy device answers with a non-thenable stand-in, which then simply resolves.) */
export function computePipelineAsync(device: GPUDevice, desc: GPUComputePipelineDescriptor): Promise<GPUComputePipeline> {
  return track(desc.label ?? 'compute', Promise.resolve(device.createComputePipelineAsync(desc)).catch(e => { pipelineProgress.failed++; reportError('gpu.pipeline', `[pipeline] ${desc.label ?? '?'}: ${e?.message ?? e}`, { label: desc.label ?? '?' }); return device.createComputePipeline(desc); }), desc);
}
/** A pipeline the boot does NOT wait for (PIPE_QUEUE.later): a twin that no first frame draws with the default settings and whose pass has a fallback until it is in (render/passes.ts
 *  traceJobs says which and why). Not counted on the start card; the queue holds it until the boot's count is frozen and then compiles it in the background lane. On rejection,
 *  logged, and NULL: its caller keeps the fallback it has been drawing with (an invalid pipeline put in its place would draw nothing where the fallback draws the picture). */
export function laterPipelineAsync(device: GPUDevice, desc: GPUComputePipelineDescriptor): Promise<GPUComputePipeline | null> {
  laterDescs.add(desc);
  return laterDone(desc.label, Promise.resolve(device.createComputePipelineAsync(desc)));
}
/** laterPipelineAsync's render twin */
export function laterRenderPipelineAsync(device: GPUDevice, desc: GPURenderPipelineDescriptor): Promise<GPURenderPipeline | null> {
  laterDescs.add(desc);
  return laterDone(desc.label, Promise.resolve(device.createRenderPipelineAsync(desc)));
}
/** how many LATER pipelines have been asked for and are not in yet: while it is above 0 some pass is drawing with a fallback, so a frame's GPU times are not the settings' own
 *  (ui/quality.ts calibrateTick waits for 0 before it measures the machine). Always 0 with PIPE_QUEUE.later off: nothing is ever asked for through this lane. */
export const laterProgress = { pending: 0 };
const laterWaits: (() => void)[] = [];
/** resolves once no LATER pipeline is still compiling (at once when none is): what a script's `__ready` waits on (main.ts; the game itself never does). Asked a tick after the
 *  count reaches 0, so one later pipeline that is asked for as another lands does not let the waiters go between the two. */
export function laterIdle(): Promise<void> { return laterProgress.pending === 0 ? Promise.resolve() : new Promise(r => laterWaits.push(r)); }
function laterDone<T>(label: string | undefined, p: Promise<T>): Promise<T | null> {
  laterProgress.pending++;
  return p.catch((e): null => { reportError('gpu.pipeline', `[pipeline] ${label ?? '?'}: ${e?.message ?? e}`, { label: label ?? '?', later: true }); return null; }).finally(laterLet);
}
function laterLet() { if (--laterProgress.pending === 0 && laterWaits.length) setTimeout(() => { if (laterProgress.pending === 0) for (const w of laterWaits.splice(0)) w(); }, 0); }
/** counted as out from NOW, for work that will ask for a LATER pipeline but has something to wait for first (the dust twin's module is checked before its
 *  pipeline is asked for): without it the count could read 0 in between and `laterIdle` let its waiters go a program early. The call handed back lets it go, once. */
export function laterHold(): () => void { laterProgress.pending++; let held = true; return () => { if (held) { held = false; laterLet(); } }; }
/** createRenderPipelineAsync, likewise */
export function renderPipelineAsync(device: GPUDevice, desc: GPURenderPipelineDescriptor): Promise<GPURenderPipeline> {
  return track(desc.label ?? 'render', Promise.resolve(device.createRenderPipelineAsync(desc)).catch(e => { pipelineProgress.failed++; reportError('gpu.pipeline', `[pipeline] ${desc.label ?? '?'}: ${e?.message ?? e}`, { label: desc.label ?? '?' }); return device.createRenderPipeline(desc); }), desc);
}

/** createRenderPipelineAsync for a pipeline the caller can do WITHOUT: counted like the others, but a rejection hands back null (never the invalid synchronous pipeline) so the
 *  caller can tell that THIS one failed and fall back — `orElse` words the fallback on the one console line */
export function renderPipelineOrNull(device: GPUDevice, desc: GPURenderPipelineDescriptor, orElse = 'left out'): Promise<GPURenderPipeline | null> {
  return track(desc.label ?? 'render', Promise.resolve<GPURenderPipeline | null>(device.createRenderPipelineAsync(desc)).catch(e => { pipelineProgress.failed++; reportError('gpu.pipeline', `[pipeline] ${desc.label ?? '?'}: ${e?.message ?? e} — ${orElse}`, { label: desc.label ?? '?' }); return null; }), desc);
}

/** Compile a WGSL module; logs compile messages with line context. */
export function makeShader(device: GPUDevice, source: string, label: string): GPUShaderModule {
  const code = shaderText(source);   // (cut to what its entry points reach — gpu/wgslTrim.ts; a message's line is a line of THIS text, printed below it)
  const mod = device.createShaderModule({ code, label });
  mod.getCompilationInfo().then(info => {
    for (const m of info.messages) {
      const lines = code.split('\n'); const ctxLine = lines[m.lineNum - 1] ?? '';
      const s = `[WGSL ${m.type}] ${label}:${m.lineNum}:${m.linePos} ${m.message}\n  > ${ctxLine.trim()}`;
      if (m.type === 'error') reportError('gpu.shader.compile', s); else reportWarningLine('gpu.shader.compile', s);
    }
  });
  return mod;
}
