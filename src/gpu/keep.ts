// THE KEPT PAGE (2026-10-27): shaders compile once a visit. A mission start is a page load — a pick, a restart, a checkpoint restart and the way back to the list
// all load the page's own address with other flags, which is what makes every mission a fresh game: no module's state, timer, listener or buffer is left of the
// one before. But the browser's GPU device goes with the page, and with it every compiled pipeline: each load compiled the renderer's ~120 programs again (a
// Windows machine measured 9 s of a 9.5 s load, on every mission; Chrome keeps only a few megabytes of compiled shaders between loads, so a reload answers few of
// them). So the page a visitor opens is now a SHELL: it owns the device and shows the game in a frame of its own address that fills the window. The loads the game
// makes are loads of that FRAME — every line that asks for one is as it was — so a mission is still born in a JavaScript world of its own, while the device and
// what was compiled on it stay with the shell:
//   - the shell hands the frame's page its device (`Keeper.take`), one LEASE a load. The device's makers of things that never change once made — a shader
//     module, a bind group layout, a pipeline layout, a pipeline — answer an ask they have answered before with the same object (keyed by the whole descriptor),
//     so a second mission's ~120 asks come back at once and its boot counts them in as ever. NOTHING MOVES INTO PLAY: a load asks for what it always asked for,
//     before its first frame; a program this visit has not built yet (another level family, another weather) compiles behind that mission's load, as it did.
//   - what a load makes that DOES change — buffers, textures, query sets — is its own: listed by lease and destroyed when the lease ends, so two missions' worth
//     of memory is never held. Its listeners on the device, its `lost` promise and an error scope it left open go with it too, and so does any method it put
//     over the device's own (the compile queue's): nothing of a load that is gone can be called by, or kept alive by, the device.
//   - a device that is really lost is dropped: the next lease opens another (and compiles again).
//   - a compile still in flight when a load ends does not hold that load: the load waited on a promise of its own, which its lease's end forgets (`settle`).
//   - a thing the device made INVALID is not kept: a module with a compile error, a layout or pipeline the device raised a validation error for, a pipeline
//     whose compile failed. The next load makes it again and hears the error a whole page hears.
// WHERE IT IS OFF (`keepRole`): `?keeppage=0` (kept for the tab; `?keeppage=1` takes it back) = every load a whole page, as it was. By itself, unless
// `?keeppage=1` asks: a page a script drives or steps (navigator.webdriver, `?paused`: the page tools read the page's own window), and a browser where a
// frame drawing with its parent's device has not been seen (anything but the Chromium family). Always: the benchmark (its loads are what it times), the kit
// picture's own window and the one-file page. A frame that finds itself under a shell of another build, or that must load whole, sends the tab there; a
// frame sent to ANOTHER page of the site (the replay viewer, the workbench) sends the tab there too, so that page has the address bar to itself.
// A page switch, not a rule of the game: no knob, no row of the frozen stand — the harness has no page. This file imports nothing.

export const KEEP_PAGE = { /** the shell's and the frame's side must agree on this */ v: 1, hook: '__pageKeeper', word: 'keeppage', store: 'twopointsix.keeppage', frameId: 'kept-page',
  /** `?keepdrain=0` on the tab's first address: a load does not wait for what the load before it left in the GPU's queue (`Keeper.drain`, as the kept page
   *  first was); `drainMs`: how long it waits at most, against a queue that never answers (a lost device ends the wait by itself, and a slow GPU's queue
   *  is waited out: past the cap the load would only freeze for the rest) */
  drainWord: 'keepdrain', drainMs: 300000 };

export type KeepRole = 'shell' | 'kept' | 'leave' | 'plain';
/** what this window is. `word` = the address's `?keeppage` (null = none), `tab` = the word the tab keeps, `top` = no window above this one, `keeper` = the window
 *  above is a shell of this build, `stale` = it is a shell of another, `own` = a page that keeps its own page (the benchmark, the kit picture), `driven` = a
 *  script drives or steps the page, `proven` = a browser where a frame is known to draw with its parent's device (false = not known), `onefile` = the
 *  one-file page.
 *    shell = build the frame · kept = take the shell's device · leave = send the tab to this address, to load whole · plain = a page as it always was */
export function keepRole(f: { word: string | null; tab: string | null; top: boolean; keeper: boolean; stale?: boolean; own: boolean; driven: boolean; onefile: boolean; proven?: boolean }): KeepRole {
  const off = f.word === '0' || (f.word === null && f.tab === '0');
  if (f.keeper || f.stale) return off || f.own || f.stale ? 'leave' : 'kept';
  if (!f.top || off || f.own || f.onefile) return 'plain';
  return (f.driven || f.proven === false) && f.word !== '1' ? 'plain' : 'shell';
}

/** what one load asked of the device and what the browser really had to make for it */
export interface KeepStats { /** pipelines asked for */ asked: number; /** …of which the browser compiled (the rest were this visit's already) */ pipelines: number; /** shader modules the browser made */ modules: number; /** the first few compiled pipelines' labels: on a start after the first, what this mission needed that the visit had not built */ labels: string[] }
/** one load's hold on the kept device */
export interface Lease {
  adapter: GPUAdapter; device: GPUDevice; asks: unknown[];
  /** which load on this kept page: 1 = the first */
  n: number;
  /** true = the device was opened for this lease (the first, or the one after a loss) */
  fresh: boolean;
  /** ms this load waited for the GPU to finish what the load before it had queued (`drained`); null = it did not wait (a device opened for it, or the
   *  wait switched off) */
  waited: number | null;
  stats: KeepStats;
  /** ms the shell's page took before it made the frame, and the browser's word for the SHELL's load (navigate · reload · back_forward): the first load's record */
  shellMs: number; shellNav: string;
  /** the counts as they stood when the boot's own pipelines were in (`freeze`): what the load's record says, whenever it is sent */
  frozen: KeepStats | null;
  freeze(): void;
  /** the load is over: what it made is destroyed, what it hung on the device is taken off (the next `take` does it for a load that never said) */
  release(): void;
}
export interface Keeper {
  v: number; build: string;
  /** set by a frame whose browser would not draw with the shell's device: the loads after it open their own */
  refused: boolean;
  leases: number; opened: number;
  /** a load waits for what the load before it left in the GPU's queue before it takes the device, `drainMs` at most (the shell's page sets them once) */
  drain: boolean; drainMs: number;
  take(): Promise<Lease>;
}
interface Opened { adapter: GPUAdapter; device: GPUDevice; asks: unknown[] }
interface Live { n: number; alive: boolean; things: WeakRef<{ destroy(): void }>[]; heard: [string, unknown, unknown][]; scopes: number; lost: Promise<GPUDeviceLostInfo>; tell: (i: GPUDeviceLostInfo) => void; stats: KeepStats;
  /** the pipelines this load waits for: each ask's own promise, by a number, so that the END of the lease forgets them (`settle`) */
  waits: Map<number, [(v: unknown) => void, (e: unknown) => void]>; nextWait: number }
const NEVER = new Promise<never>(() => { /* a lease that is over hears nothing */ });
/** A compile in flight outlives the load that asked for it (the next load's ask for the same pipeline is answered by it): the load is handed a promise of
 *  ITS OWN, reached from the compile only through the lease's table. When the lease ends the table is emptied, and nothing the gone page hung on its
 *  promise (its whole JavaScript world, by way of one callback) is held until the compile lands. (A function of the module's, so that what the compile's
 *  promise holds is the lease and a number, and nothing else by accident of scope.) */
function settle(p: Promise<unknown>, L: Live, id: number): void {
  p.then(v => { const w = L.waits.get(id); if (w) { L.waits.delete(id); w[0](v); } }, e => { const w = L.waits.get(id); if (w) { L.waits.delete(id); w[1](e); } });
}
/** a device and what was made on it for good: by descriptor, the modules and layouts (`made`), the pipelines that are in (`pipes`) and the ones compiling (`flying`) */
interface Held { o: Opened; /** the device's own `lost` */ gone: Promise<unknown>; made: Map<string, unknown>; pipes: Map<string, unknown>; flying: Map<string, Promise<unknown>>; /** pipelines whose compile failed: the stand-in their caller makes at once is not kept */ failed: Set<string>; ids: WeakMap<object, number>; next: number }
/** how many things a device keeps at most: past it a new one is made and handed over, not kept (a visit through every level, weather and quality stays far under) */
const KEEP_CAP = 4096;

/** (the checks' red path: KEEPPAGE_BREAK=1 in front of tools/qa/keeppage-check.ts — every maker straight to the device, nothing kept; a page has no process) */
const BREAK = (() => { try { return String((globalThis as { process?: { env?: Record<string, string | undefined> } }).process?.env?.KEEPPAGE_BREAK ?? '') === '1'; } catch { return false; } })();
const tag = (v: unknown): string => Object.prototype.toString.call(v);
/** a descriptor as a string: plain data as JSON, a GPU object by the number it was given when it was first seen (the same object, the same number) */
function keyOf(desc: unknown, H: { ids: WeakMap<object, number>; next: number }): string | null {
  if (BREAK) return null;
  try {
    return JSON.stringify(desc, (_k, v: unknown) => {
      if (v === null || typeof v !== 'object' || !tag(v).startsWith('[object GPU')) return v;
      let id = H.ids.get(v); if (id === undefined) { id = H.next++; H.ids.set(v, id); }
      return `\u0001${id}`;
    }) ?? null;
  } catch { return null; }
}

/** THE SHELL'S SIDE. `open` = the adapter and the device, as a page opens them (gpu/device.ts openDevice); `build` = what a frame's page must say too. Every
 *  function made here belongs to the shell's window: a frame's page calls them, and nothing here keeps a thing a frame's page made but a GPU object. */
export function makeKeeper(open: () => Promise<Opened>, build: string, shellMs = 0, shellNav = ''): Keeper {
  let held: Held | null = null, cur: Live | null = null, opening: Promise<Held> | null = null, takes = 0;
  const end = (L: Live | null): void => {
    if (!L || !L.alive) return; L.alive = false;
    const d = held?.o.device;
    for (const w of L.things) { try { w.deref()?.destroy(); } catch { /* gone already */ } } L.things.length = 0;
    if (d) {
      for (const [type, fn, o] of L.heard) { try { EventTarget.prototype.removeEventListener.call(d, type, fn as EventListener, o as AddEventListenerOptions); } catch { /* */ } }
      try { d.onuncapturederror = null; } catch { /* */ }
      for (; L.scopes > 0; L.scopes--) { try { GPUDevice.prototype.popErrorScope.call(d).catch(() => { /* */ }); } catch { break; } }
    }
    L.heard.length = 0; L.waits.clear(); L.lost = NEVER; L.tell = () => { /* over */ }; if (cur === L) cur = null;
    if (held && cur === null) dress(held);   // (…and whatever the load put over the device's own methods goes now, not at the next lease: the device holds nothing of a page that is gone)
  };
  /** the device's own methods under ours, and nothing a load put over them */
  const dress = (H: Held): void => {
    const d = H.o.device, D = d as unknown as Record<string, unknown>, P = GPUDevice.prototype;
    for (const k of Object.getOwnPropertyNames(d)) { try { delete D[k]; } catch { /* */ } }
    const room = (): boolean => H.made.size + H.pipes.size < KEEP_CAP;
    /** was what `make(desc)` gave VALID? Asked of the device with a second, thrown-away making inside an error scope of our own (the first went to the page's
     *  scopes and listeners as ever; an equal second one costs the device a lookup). An error = `drop()` */
    const proved = <A,>(make: (desc: A) => unknown, desc: A, drop: () => void): void => {
      try { P.pushErrorScope.call(d, 'validation'); try { make.call(d, desc); } finally { void P.popErrorScope.call(d).then(e => { if (e) drop(); }, () => { /* the device is gone */ }); } } catch { drop(); }
    };
    /** a maker of a thing that never changes: the same descriptor, the same thing */
    const once = <A, T>(kind: string, make: (desc: A) => T, after?: (v: T, drop: () => void) => void) => (desc: A): T => {
      const k0 = keyOf(desc, H), k = k0 === null ? null : kind + k0;
      if (k !== null && H.made.has(k)) return H.made.get(k) as T;
      const v = make.call(d, desc);
      if (k !== null && room()) { H.made.set(k, v); const drop = (): void => { if (H.made.get(k) === v) H.made.delete(k); }; if (after) after(v, drop); else proved(make, desc, drop); }
      return v;
    };
    // (a module's own report says whether it compiled: nothing is made twice for it)
    D.createShaderModule = once('m', P.createShaderModule, (m, drop) => { if (cur) cur.stats.modules++; try { void m.getCompilationInfo().then(i => { if (i.messages.some(x => x.type === 'error')) drop(); }, drop); } catch { /* a stand-in with no report */ } });
    D.createBindGroupLayout = once('b', P.createBindGroupLayout);
    D.createPipelineLayout = once('l', P.createPipelineLayout);
    // a pipeline: the one that is in, else (an ask that can wait) the one compiling — two asks in flight are one compile — else a new one, kept once it is in.
    // One that failed is not kept, nor is the stand-in its caller then makes at once (gpu/device.ts: the invalid object the old code got): the next load
    // asks again and hears the failure as a whole page does
    const compiled = (desc: unknown): void => { const S = cur?.stats; if (!S) return; S.pipelines++; if (S.labels.length < 8) S.labels.push(String((desc as { label?: string } | undefined)?.label ?? '?').slice(0, 48)); };
    const now = <A, T>(kind: string, make: (desc: A) => T) => (desc: A): T => {
      const k0 = keyOf(desc, H), k = k0 === null ? null : kind + k0;
      if (k !== null && H.failed.has(k)) return make.call(d, desc);
      if (cur) cur.stats.asked++;
      if (k !== null && H.pipes.has(k)) return H.pipes.get(k) as T;
      compiled(desc);
      const v = make.call(d, desc);
      if (k !== null && !H.flying.has(k) && room()) { H.pipes.set(k, v); proved(make, desc, () => { if (H.pipes.get(k) === v) H.pipes.delete(k); }); }
      return v;
    };
    /** the load's own promise of a compile (`settle`) */
    const hand = <T,>(p: Promise<T>): Promise<T> => {
      const L = cur; if (!L) return NEVER;
      const id = L.nextWait++, mine = new Promise<T>((res, rej) => { L.waits.set(id, [res as (v: unknown) => void, rej]); });
      settle(p, L, id); return mine;
    };
    const later = <A, T>(kind: string, make: (desc: A) => Promise<T>) => (desc: A): Promise<T> => {
      if (cur) cur.stats.asked++;
      const k0 = keyOf(desc, H), k = k0 === null ? null : kind + k0;
      if (k !== null && H.pipes.has(k)) return Promise.resolve(H.pipes.get(k) as T);
      if (k !== null && H.flying.has(k)) return hand(H.flying.get(k) as Promise<T>);
      compiled(desc);
      const p = make.call(d, desc);
      if (k !== null) { H.flying.set(k, p); p.then(v => { H.flying.delete(k); H.failed.delete(k); if (room()) H.pipes.set(k, v); }, () => { H.flying.delete(k); H.failed.add(k); }); }
      return hand(p);
    };
    D.createComputePipeline = now('c', P.createComputePipeline); D.createRenderPipeline = now('r', P.createRenderPipeline);
    D.createComputePipelineAsync = later('c', P.createComputePipelineAsync); D.createRenderPipelineAsync = later('r', P.createRenderPipelineAsync);
    // what a load makes of its own: listed, so the end of its lease frees it without waiting on the collector
    const mine = <A, T extends { destroy(): void }>(make: (desc: A) => T) => (desc: A): T => {
      const v = make.call(d, desc), L = cur;
      if (L) { if (L.things.length >= 512 && (L.things.length & 255) === 0) L.things = L.things.filter(w => w.deref() !== undefined); L.things.push(new WeakRef(v)); }
      return v;
    };
    D.createBuffer = mine(P.createBuffer); D.createTexture = mine(P.createTexture); D.createQuerySet = mine(P.createQuerySet);
    D.addEventListener = (type: string, fn: unknown, opts?: unknown): void => { cur?.heard.push([type, fn, opts]); EventTarget.prototype.addEventListener.call(d, type, fn as EventListener, opts as AddEventListenerOptions); };
    D.pushErrorScope = (filter: GPUErrorFilter): void => { if (cur) cur.scopes++; P.pushErrorScope.call(d, filter); };
    D.popErrorScope = (): Promise<GPUError | null> => { if (cur && cur.scopes > 0) cur.scopes--; return P.popErrorScope.call(d); };
    // each lease its own `lost`: a load's reactions hang on a promise that goes with it, not on the device's
    Object.defineProperty(d, 'lost', { configurable: true, get: () => cur?.lost ?? new Promise<GPUDeviceLostInfo>(() => { /* nobody holds the device */ }) });
  };
  const device = async (): Promise<{ H: Held; fresh: boolean }> => {
    if (held) return { H: held, fresh: false };
    opening ??= open().then(o => {
      // (the device's own promise: a lost device is dropped, and whoever holds it then is told)
      const real = (Object.getOwnPropertyDescriptor(GPUDevice.prototype, 'lost')!.get as () => Promise<GPUDeviceLostInfo>).call(o.device);
      const H: Held = { o, gone: real, made: new Map(), pipes: new Map(), flying: new Map(), failed: new Set(), ids: new WeakMap(), next: 1 };
      held = H; K.opened++;
      void real.then(info => { if (held === H) held = null; cur?.tell(info); });
      return H;
    }).finally(() => { opening = null; });
    return { H: await opening, fresh: true };
  };
  /** THE QUEUE THE LAST LOAD LEFT. One device is one queue: the frames a mission had submitted when its page went are still the GPU's to do, where a whole
   *  page load drops them with its device. A load that took the device at once then froze at its first big upload until the GPU had caught up (seen on a
   *  software adapter: 72 s inside the Game's constructor, in queue.writeBuffer, with the queue 75 s behind at the pick). So the wait is taken here, where
   *  the page is live and the load's record counts it as the device's time. Resolves with the ms it took, or null where there is no wait to take; never
   *  rejects. It ends when the queue is done, when the device is lost (`take` then opens another), or after `Keeper.drainMs` */
  const drained = async (H: Held): Promise<number | null> => {
    const q = (H.o.device as { queue?: { onSubmittedWorkDone?: () => Promise<unknown> } }).queue;
    if (BREAK || !K.drain || typeof q?.onSubmittedWorkDone !== 'function') return null;
    const t0 = performance.now(); let cap: ReturnType<typeof setTimeout> | undefined;
    try { await Promise.race([q.onSubmittedWorkDone(), H.gone, new Promise<void>(r => { cap = setTimeout(r, K.drainMs); })]); } catch { /* a queue that fails is a queue that is not waited for */ }
    clearTimeout(cap);
    return performance.now() - t0;
  };
  const K: Keeper = {
    v: KEEP_PAGE.v, build, refused: false, leases: 0, opened: 0, drain: true, drainMs: KEEP_PAGE.drainMs,
    async take(): Promise<Lease> {
      // (two takes at once: the LATER one holds the device. The earlier is its page's last word, a page that is gone or going: it is refused at its next
      // step, so it can end no lease but the one it found and never becomes the holder behind the later one's back)
      const mine = ++takes, late = (): void => { if (mine !== takes) throw new Error('the kept page\'s device went to a later load'); };
      end(cur);
      let H: Held, fresh: boolean, waited: number | null = null;
      for (;;) {
        ({ H, fresh } = await device()); late();
        if (fresh) break;
        const ms = await drained(H); late();
        if (ms !== null) waited = (waited ?? 0) + ms;
        if (held === H) break;   // (else the device was lost while this load waited for its queue: another is opened, as for a loss between two loads)
      }
      end(cur);
      let tell!: (i: GPUDeviceLostInfo) => void; const lost = new Promise<GPUDeviceLostInfo>(r => { tell = r; });
      const L: Live = { n: ++K.leases, alive: true, things: [], heard: [], scopes: 0, lost, tell, stats: { asked: 0, pipelines: 0, modules: 0, labels: [] }, waits: new Map(), nextWait: 1 };
      cur = L; dress(H);
      const lease: Lease = { adapter: H.o.adapter, device: H.o.device, asks: H.o.asks, n: L.n, fresh, waited, stats: L.stats, shellMs, shellNav, frozen: null, freeze: () => { lease.frozen ??= { ...L.stats, labels: L.stats.labels.slice() }; }, release: () => end(L) };
      return lease;
    },
  };
  return K;
}

/** THE FRAME'S SIDE: the shell above this window (`same` = of this build), or null. Never throws: a window of another origin above is no shell */
export function keeperAbove(win: { parent?: unknown } = window, build?: string): { keeper: Keeper; same: boolean } | null {
  try {
    const up = win.parent as (Record<string, unknown> & { parent?: unknown }) | undefined; if (!up || up === win) return null;
    const k = up[KEEP_PAGE.hook] as Keeper | undefined; if (!k || typeof k.take !== 'function') return null;
    return { keeper: k, same: k.v === KEEP_PAGE.v && (build === undefined || k.build === build) && !k.refused };
  } catch { return null; }
}
/** the lease this page's device came on, or null on a page with a device of its own (gpu/device.ts initGpu sets it) */
/** …and `shelled`: this window is a frame of a kept page's shell (host/keeppage.ts sets it), whoever's device it draws with: the keys are the window's own,
 *  as on a page with nothing above it (game/input.ts) */
export const KEPT: { lease: Lease | null; shelled: boolean } = { lease: null, shelled: false };
/** the boot's own pipelines are in (main.ts, where it freezes the boot's count): the record's `made` is the load's, however long the page then stays */
export function keptFreeze(): void { KEPT.lease?.freeze(); }
/** a load's record, with the kept page's words: `kept` which load on this page, `made` what the browser compiled for it against what was asked, and on the first
 *  `shell`, the ms the shell took before the frame (the record's own clock starts at the frame's load), and `shellNav`, the browser's word for the shell's
 *  load (the frame's own `nav` is always a plain navigation); on a later one `waited`, the ms it waited for the GPU to finish what the load before it had
 *  queued (inside the record's own device time; no word = it did not wait: the wait switched off, or a device opened for it after a loss). A plain page's
 *  record is handed back as it is */
export function keptRecord<T extends Record<string, unknown> | null>(rec: T, lease: Lease | null = KEPT.lease): T {
  if (!rec || !lease) return rec;
  const S = lease.frozen ?? lease.stats;
  return { ...rec, kept: lease.n, made: { pipelines: S.pipelines, modules: S.modules, asked: S.asked, ...(lease.n > 1 && S.labels.length ? { labels: S.labels.slice() } : {}) }, ...(lease.n === 1 ? { shell: Math.round(lease.shellMs), ...(lease.shellNav ? { shellNav: lease.shellNav } : {}) } : lease.waited !== null ? { waited: Math.round(lease.waited) } : {}) };
}

/** THE SHELL'S PAGE: the frame over everything, the address and the title following the frame's, the keys going to it. `doc` is the shell's own document */
export function showKeptFrame(doc: Document, href: string): HTMLIFrameElement {
  const win = doc.defaultView!, f = doc.createElement('iframe'), home = win.location.pathname;
  f.id = KEEP_PAGE.frameId; f.title = doc.title; f.allowFullscreen = true;
  f.style.cssText = 'position:fixed;inset:0;width:100%;height:100%;border:0;margin:0;padding:0;background:#000;z-index:2147483647;display:block';
  for (const el of Array.from(doc.body.children)) (el as HTMLElement).style.display = 'none';   // (the page's own canvas and cards: the frame's are the ones seen)
  const toFrame = (): void => { try { if (doc.activeElement !== f) f.contentWindow?.focus(); } catch { /* */ } };
  f.addEventListener('load', () => {
    try {
      const to = f.contentWindow!.location;
      // (another page of the site in the frame — a browser with no way to stop the frame's load on its way there, host/keeppage.ts — is the tab's to load)
      if (to.origin === win.location.origin && to.pathname !== home && to.href !== 'about:blank') { win.location.replace(to.href); return; }
      if (to.origin === win.location.origin && to.href !== win.location.href) win.history.replaceState(win.history.state, '', to.href);
      const t = f.contentDocument?.title; if (t) doc.title = t;
    } catch { /* a page of another origin in the frame: the address stays */ }
    toFrame();
  });
  win.addEventListener('focus', toFrame); win.addEventListener('keydown', toFrame, true); win.addEventListener('pointerdown', toFrame, true);
  // (no `src`: a frame is not given the address of the page it stands in by that road — browsers differ on how many times a page may name itself — so the
  // frame is made empty and sent there, as any of the game's own loads sends it)
  doc.body.appendChild(f);
  try { f.contentWindow!.location.replace(href); } catch { f.src = href; }
  toFrame();
  return f;
}
