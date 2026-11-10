// THE KEPT PAGE (2026-10-27): shaders compile once a visit. A mission start is a page load — a pick, a restart, a checkpoint restart and the way back to the list
// all load the page's own address with other flags, which is what makes every mission a fresh game: no module's state, timer, listener or buffer is left of the
// one before. But the browser's GPU device goes with the page, and with it every compiled pipeline: each load compiled the renderer's ~120 programs again (a
// Windows machine measured 9 s of a 9.5 s load, on every mission; Chrome keeps only a few megabytes of compiled shaders between loads, so a reload answers few of
// them). So the page a visitor opens is now a SHELL: it owns the device and shows the game in a frame of its own address that fills the window. The loads the game
// makes are loads of that FRAME — every line that asks for one is as it was — so a mission is still born in a JavaScript world of its own, while the device and
// what was compiled on it stay with the shell:
//   - the shell hands the frame's page its device (`Keeper.take`), one LEASE a load, as a VIEW of the device that is the lease's own (`viewOf`: the device
//     itself is never written to). The device's makers of things that never change once made — a shader
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
// frame drawing with its parent's device has not been seen (anything but the Chromium family). Always: the benchmark (its loads are what it times) and the
// one-file page. A frame that finds itself under a shell of another build, or that must load whole, sends the tab there; a
// frame sent to ANOTHER page of the site (the replay viewer, the listening pages) sends the tab there too, so that page has the address bar to itself.
// THE TEAM & KIT PICTURE IS ON THE SAME DEVICE (2026-11-07). It is a second game in a window inside the game's page, and it had a device of its own: every
// program compiled again when the step opened, and what it compiled went with it. Under a shell it now takes a lease BESIDE its page's, on a seat of its
// own (`Keeper.take(seat)`, `keepRole`'s `beside`): the visit's programs answer it, and the ones it adds (the picked mission's own) are there when Deploy
// loads that mission. Its lease ends with its window, with the next picture's, or with its page's. With no shell above (a driven page, `?keeppage=0`)
// it opens its own device, as it did.
// A page switch, not a rule of the game. This file imports nothing.

export const KEEP_PAGE = { /** the shell's and the frame's side must agree on this */ v: 2, hook: '__pageKeeper', word: 'keeppage', store: 'twopointsix.keeppage', frameId: 'kept-page',
  /** the seats a lease is taken on (`Keeper.take`): the game's page in the shell's frame, and a picture shown inside that page (under it: it goes with its page) */
  seat: 'page', pictureSeat: 'page/picture',
  /** the most bytes a visit keeps of what its pages worked out (`Lease.keepBytes`): the look's thirty maps are 84 MB */
  bytesCap: 192 * 1048576,
  /** `?keepdrain=0` on the tab's first address: a load does not wait for what the load before it left in the GPU's queue (`Keeper.drain`, as the kept page
   *  first was); `drainMs`: how long it waits at most, against a queue that never answers (a lost device ends the wait by itself, and a slow GPU's queue
   *  is waited out: past the cap the load would only freeze for the rest) */
  drainWord: 'keepdrain', drainMs: 300000,
  /** `?callclock=0` on a page's address: its calls that carry bytes to the GPU are not timed while it loads (`startCallClock`) */
  clockWord: 'callclock',
  /** `?playclock=0`: …and after the load, a frame of play's time inside the queue's calls is not totalled (`CALLS.play`) */
  playWord: 'playclock' };

export type KeepRole = 'shell' | 'kept' | 'beside' | 'leave' | 'plain';
/** what this window is. `word` = the address's `?keeppage` (null = none), `tab` = the word the tab keeps, `top` = no window above this one, `keeper` = there is
 *  a shell of this build above (for a page: the window above; for a picture: any window above), `stale` = the window above is a shell of another build,
 *  `own` = a page that keeps its own page (the benchmark and its scenes), `picture` = a second game shown INSIDE a page of the game (the Team & kit
 *  picture), `driven` = a script drives or steps the page, `proven` = a browser where a frame is known to draw with its parent's device (false = not
 *  known).
 *    shell = build the frame · kept = take the shell's device, on the page's seat · beside = take it on the picture's seat, beside the page that shows
 *    this window · leave = send the tab to this address, to load whole · plain = a page as it always was
 *  A picture never sends the tab anywhere and is never a shell: it draws with the shell's device where its page does, and with one of its own otherwise. */
export function keepRole(f: { word: string | null; tab: string | null; top: boolean; keeper: boolean; stale?: boolean; own: boolean; picture?: boolean; driven: boolean; proven?: boolean }): KeepRole {
  const off = f.word === '0' || (f.word === null && f.tab === '0');
  if (f.picture) return f.keeper && !off ? 'beside' : 'plain';
  if (f.keeper || f.stale) return off || f.own || f.stale ? 'leave' : 'kept';
  if (!f.top || off || f.own) return 'plain';
  return (f.driven || f.proven === false) && f.word !== '1' ? 'plain' : 'shell';
}

/** what one load asked of the device and what the browser really had to make for it */
export interface KeepStats { /** pipelines asked for */ asked: number; /** …of which the browser compiled (the rest were this visit's already) */ pipelines: number; /** shader modules the browser made */ modules: number; /** the first few compiled pipelines' labels: on a start after the first, what this mission needed that the visit had not built */ labels: string[] }
/** bytes a page worked out and the visit keeps (`Lease.bytes`, `Lease.keepBytes`): the arrays are the shell's own copies, to be read and never written */
export interface KeptBytes { bytes: Uint8Array[]; nums: number[] }
/** one page's hold on the kept device */
export interface Lease {
  adapter: GPUAdapter; asks: unknown[];
  /** THIS LEASE'S VIEW OF THE DEVICE (`viewOf`): what the page calls and may put its own functions over. It is not a GPUDevice to the browser */
  device: GPUDevice;
  /** …and the device itself, for the one call that must be handed it: a canvas's `configure`. Nothing else of a page touches it */
  real: GPUDevice;
  /** the seat it was taken on (`Keeper.take`), and which lease of that seat it is: 1 = the first */
  seat: string; n: number;
  /** true = the device was opened for this lease (the first, or the one after a loss) */
  fresh: boolean;
  /** ms this load waited for the GPU to finish what was queued before it (`drained`); null = it did not wait (a device opened for it, or the wait
   *  switched off) */
  waited: number | null;
  stats: KeepStats;
  /** ms the shell's page took before it made the frame, and the browser's word for the SHELL's load (navigate · reload · back_forward): the first load's record */
  shellMs: number; shellNav: string;
  /** the counts as they stood when the boot's own pipelines were in (`freeze`): what the load's record says, whenever it is sent */
  frozen: KeepStats | null;
  freeze(): void;
  /** WHAT THE VISIT HAS WORKED OUT ONCE, by a name: plain bytes a page made from nothing but its own code (the look's material maps: a third of a second
   *  of arithmetic each, the same on every level), so that a later load, and a picture beside this one, is handed them where it would make them again.
   *  `keepBytes` COPIES into arrays of the shell's own (an array a page made would hold that page's whole world after it is gone) and keeps the first
   *  said under a name; `bytes` = what is kept, or null. The name must say everything the bytes depend on: a build is one keeper, so the code is the same.
   *  Nothing of a device: a device lost keeps them. `KEEP_PAGE.bytesCap` in all; past it nothing more is kept */
  bytes(key: string): KeptBytes | null;
  keepBytes(key: string, bytes: ArrayLike<Uint8Array>, nums?: ArrayLike<number>): void;
  /** false once the lease is over (`release`, the next `take` of its seat, or the end of the lease of the seat it is under) */
  readonly alive: boolean;
  /** the load is over: what it made is destroyed, what it hung on the device is taken off (the next `take` of its seat does it for a load that never said) */
  release(): void;
}
export interface Keeper {
  v: number; build: string;
  /** set by a frame whose browser would not draw with the shell's device: the loads after it open their own */
  refused: boolean;
  /** leases handed out (every seat) · devices opened · the seats that hold a lease now · the bytes the visit keeps of what its pages worked out */
  leases: number; opened: number; readonly seats: string[]; readonly keptBytes: number;
  /** a load waits for what was left in the GPU's queue before it takes the device, `drainMs` at most (the shell's page sets them once) */
  drain: boolean; drainMs: number;
  /** A SEAT holds one lease at a time: a take of a seat ends the lease that was on it, and the leases of every seat under it (`a/b` is under `a`).
   *  Leases of different seats are held at once, each with its own view of the one device, so what one page compiled the other finds made. The game's
   *  page takes `KEEP_PAGE.seat`; a picture shown inside it takes `KEEP_PAGE.pictureSeat`, which is under it and so goes with its page */
  take(seat?: string): Promise<Lease>;
  /** the seat is given up by whoever owns its page (the start page dropping its picture's window): the lease on it ends, and a take of it still on
   *  its way (a window dropped while it booted: nobody is left to release what that take would be handed) is refused */
  release(seat: string): void;
}
interface Opened { adapter: GPUAdapter; device: GPUDevice; asks: unknown[] }
interface Live { n: number; seat: string; H: Held; alive: boolean; things: WeakRef<{ destroy(): void }>[]; heard: [string, unknown, unknown][]; scopes: number; lost: Promise<GPUDeviceLostInfo>; tell: (i: GPUDeviceLostInfo) => void; stats: KeepStats;
  /** the pipelines this load waits for: each ask's own promise, by a number, so that the END of the lease forgets them (`settle`) */
  waits: Map<number, [(v: unknown) => void, (e: unknown) => void]>; nextWait: number;
  /** what the page put over the device's and its queue's own (the views' `own`): emptied when the lease ends */
  own: Record<PropertyKey, unknown>[] }
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

/** A LEASE'S OWN VIEW of a thing the visit keeps (the device, its queue). Two pages hold the device at once (a mission's page and the Team & kit picture
 *  inside it), and a page puts functions of its own over the device's (the compile queue, the load's clock): written on the device itself they would be
 *  every holder's and would outlive their page. So a page is never handed the device: it is handed a view. A read finds what the page put there (`own`),
 *  else what the keeper answers for this lease (`base`: the makers that keep, the lease's `lost`), else the thing's own, a method bound to the thing and
 *  the same function at every read. A write stays in `own` (taking it off again uncovers the keeper's, never the raw device's), but for a property the
 *  keeper answers with a setter of its own (`onuncapturederror`: a listener the lease owns). NOTHING a page writes reaches the thing: another lease
 *  holds it too. The end of the lease empties `own`, so a view somebody still holds keeps nothing of a page that is gone. The browser does not take a view for the thing (a canvas is configured with `Lease.real`); to script it is one: `instanceof` holds. */
function viewOf<T extends object>(real: T, base: object, own: Record<PropertyKey, unknown>): T {
  const R = real as Record<PropertyKey, unknown>, B = base as Record<PropertyKey, unknown>, bound = new Map<PropertyKey, unknown>();
  return new Proxy(own, {
    get(t, k) {
      if (k in t) return t[k];
      if (k in B) return B[k];
      let f = bound.get(k); if (f !== undefined) return f;
      const v = R[k]; if (typeof v !== 'function') return v;
      bound.set(k, f = (v as (...a: unknown[]) => unknown).bind(real)); return f;
    },
    set(t, k, v) { const d = Object.getOwnPropertyDescriptor(B, k); if (d?.set) d.set.call(B, v); else t[k] = v; return true; },
    has: (t, k) => k in t || k in B || k in R,
    getPrototypeOf: () => Object.getPrototypeOf(real),
  }) as unknown as T;
}

/** THE SHELL'S SIDE. `open` = the adapter and the device, as a page opens them (gpu/device.ts openDevice); `build` = what a frame's page must say too. Every
 *  function made here belongs to the shell's window: a frame's page calls them, and nothing here keeps a thing a frame's page made but a GPU object. The
 *  device itself is never written to: each lease reads it through a view of its own (`viewOf`). */
export function makeKeeper(open: () => Promise<Opened>, build: string, shellMs = 0, shellNav = ''): Keeper {
  let held: Held | null = null, opening: Promise<Held> | null = null;
  /** what the visit's pages worked out once (`Lease.keepBytes`), and its size */
  const stash = new Map<string, KeptBytes>(); let stashed = 0;
  const bytesOf = (key: string): KeptBytes | null => BREAK ? null : stash.get(key) ?? null;
  const keepBytes = (key: string, bytes: ArrayLike<Uint8Array>, nums?: ArrayLike<number>): void => {
    if (BREAK || stash.has(key)) return;
    let size = 0; for (let i = 0; i < bytes.length; i++) size += bytes[i].byteLength;
    if (stashed + size > KEEP_PAGE.bytesCap) return;
    // (copies made HERE, in the shell's window, element by element: nothing kept is of the page that asked)
    const own: Uint8Array[] = [], n: number[] = [];
    for (let i = 0; i < bytes.length; i++) { const c = new Uint8Array(bytes[i].byteLength); c.set(bytes[i]); own.push(c); }
    for (let i = 0; nums && i < nums.length; i++) n.push(Number(nums[i]));
    stash.set(String(key), { bytes: own, nums: n }); stashed += size;
  };
  /** the lease in hand on each seat · how many times each seat was asked for · how many leases each was given */
  const live = new Map<string, Live>(), takes = new Map<string, number>(), given = new Map<string, number>();
  const end = (L: Live | null | undefined): void => {
    if (!L || !L.alive) return; L.alive = false;
    const d = L.H.o.device;
    for (const w of L.things) { try { w.deref()?.destroy(); } catch { /* gone already */ } } L.things.length = 0;
    for (const [type, fn, o] of L.heard) { try { EventTarget.prototype.removeEventListener.call(d, type, fn as EventListener, o as AddEventListenerOptions); } catch { /* */ } }
    for (; L.scopes > 0; L.scopes--) { try { GPUDevice.prototype.popErrorScope.call(d).catch(() => { /* */ }); } catch { break; } }
    L.heard.length = 0; L.waits.clear(); L.lost = NEVER; L.tell = () => { /* over */ };
    // (whatever the load put over the device's or the queue's own methods goes now: the view holds nothing of a page that is gone)
    for (const own of L.own) for (const k of Reflect.ownKeys(own)) { try { delete own[k]; } catch { /* */ } }
    // (…and a page that is gone hands the queue nothing: a boot still on its way when its frame was taken away goes on to its next wait, over buffers destroyed above)
    for (const k of ['submit', 'writeBuffer', 'writeTexture', 'copyExternalImageToTexture']) try { L.own[1][k] = NOT; } catch { /* */ }
    if (live.get(L.seat) === L) live.delete(L.seat);
  };
  /** the seat's lease and those of every seat under it: a picture inside a page goes with its page */
  const clear = (seat: string): void => { for (const L of [...live.values()]) if (L.seat === seat || L.seat.startsWith(seat + '/')) end(L); };
  /** WHAT THE KEEPER ANSWERS FOR ONE LEASE in place of the device's own (the lease's view reads these first): the makers of things that never change, which
   *  keep; the makers of what a load owns, which list; the listeners, scopes and `lost` that go with the lease; and the lease's view of the queue */
  const makers = (H: Held, L: Live, queueOwn: Record<PropertyKey, unknown>): object => {
    const d = H.o.device, P = GPUDevice.prototype;
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
    // a pipeline: the one that is in, else (an ask that can wait) the one compiling — two asks in flight are one compile, whichever leases they came on —
    // else a new one, kept once it is in. One that failed is not kept, nor is the stand-in its caller then makes at once (gpu/device.ts: the invalid object
    // the old code got): the next load asks again and hears the failure as a whole page does
    const compiled = (desc: unknown): void => { const S = L.stats; S.pipelines++; if (S.labels.length < 8) S.labels.push(String((desc as { label?: string } | undefined)?.label ?? '?').slice(0, 48)); };
    const now = <A, T>(kind: string, make: (desc: A) => T) => (desc: A): T => {
      const k0 = keyOf(desc, H), k = k0 === null ? null : kind + k0;
      if (k !== null && H.failed.has(k)) return make.call(d, desc);
      L.stats.asked++;
      if (k !== null && H.pipes.has(k)) return H.pipes.get(k) as T;
      compiled(desc);
      const v = make.call(d, desc);
      if (k !== null && !H.flying.has(k) && room()) { H.pipes.set(k, v); proved(make, desc, () => { if (H.pipes.get(k) === v) H.pipes.delete(k); }); }
      return v;
    };
    /** the load's own promise of a compile (`settle`) */
    const hand = <T,>(p: Promise<T>): Promise<T> => {
      if (!L.alive) return NEVER;
      const id = L.nextWait++, mine = new Promise<T>((res, rej) => { L.waits.set(id, [res as (v: unknown) => void, rej]); });
      settle(p, L, id); return mine;
    };
    const later = <A, T>(kind: string, make: (desc: A) => Promise<T>) => (desc: A): Promise<T> => {
      L.stats.asked++;
      const k0 = keyOf(desc, H), k = k0 === null ? null : kind + k0;
      if (k !== null && H.pipes.has(k)) return Promise.resolve(H.pipes.get(k) as T);
      if (k !== null && H.flying.has(k)) return hand(H.flying.get(k) as Promise<T>);
      compiled(desc);
      const p = make.call(d, desc);
      if (k !== null) { H.flying.set(k, p); p.then(v => { H.flying.delete(k); H.failed.delete(k); if (room()) H.pipes.set(k, v); }, () => { H.flying.delete(k); H.failed.add(k); }); }
      return hand(p);
    };
    // what a load makes of its own: listed, so the end of its lease frees it without waiting on the collector
    const mine = <A, T extends { destroy(): void }>(make: (desc: A) => T) => (desc: A): T => {
      const v = make.call(d, desc);
      if (L.alive) { if (L.things.length >= 512 && (L.things.length & 255) === 0) L.things = L.things.filter(w => w.deref() !== undefined); L.things.push(new WeakRef(v)); }
      return v;
    };
    const queue = (d as unknown as { queue?: object }).queue; let onerr: unknown = null;
    return {
      // (a module's own report says whether it compiled: nothing is made twice for it)
      createShaderModule: once('m', P.createShaderModule, (m, drop) => { L.stats.modules++; try { void m.getCompilationInfo().then(i => { if (i.messages.some(x => x.type === 'error')) drop(); }, drop); } catch { /* a stand-in with no report */ } }),
      createBindGroupLayout: once('b', P.createBindGroupLayout),
      createPipelineLayout: once('l', P.createPipelineLayout),
      createComputePipeline: now('c', P.createComputePipeline), createRenderPipeline: now('r', P.createRenderPipeline),
      createComputePipelineAsync: later('c', P.createComputePipelineAsync), createRenderPipelineAsync: later('r', P.createRenderPipelineAsync),
      createBuffer: mine(P.createBuffer), createTexture: mine(P.createTexture), createQuerySet: mine(P.createQuerySet),
      addEventListener: (type: string, fn: unknown, opts?: unknown): void => { if (L.alive) L.heard.push([type, fn, opts]); EventTarget.prototype.addEventListener.call(d, type, fn as EventListener, opts as AddEventListenerOptions); },
      pushErrorScope: (filter: GPUErrorFilter): void => { if (L.alive) L.scopes++; P.pushErrorScope.call(d, filter); },
      popErrorScope: (): Promise<GPUError | null> => { if (L.scopes > 0) L.scopes--; return P.popErrorScope.call(d); },
      // each lease its own `lost`: a load's reactions hang on a promise that goes with it, not on the device's
      get lost(): Promise<GPUDeviceLostInfo> { return L.lost; },
      // (…and its own `onuncapturederror`: the device has one such property and two holders, so a lease's is a listener of its own, gone with it)
      get onuncapturederror(): unknown { return onerr; },
      set onuncapturederror(fn: unknown) {
        if (typeof onerr === 'function') { try { EventTarget.prototype.removeEventListener.call(d, 'uncapturederror', onerr as EventListener); } catch { /* */ } L.heard = L.heard.filter(h => h[1] !== onerr); }
        onerr = typeof fn === 'function' ? fn : null;
        if (onerr && L.alive) { L.heard.push(['uncapturederror', onerr, undefined]); EventTarget.prototype.addEventListener.call(d, 'uncapturederror', onerr as EventListener); }
      },
      // (…and its own view of the queue: the load's clock on the queue's calls is the page's, `startCallClock`, and goes with it)
      ...(queue ? { queue: viewOf(queue, {}, queueOwn) } : {}),
    };
  };
  const device = async (): Promise<{ H: Held; fresh: boolean }> => {
    if (held) return { H: held, fresh: false };
    opening ??= open().then(o => {
      // (the device's own promise: a lost device is dropped, and whoever holds it then is told)
      const real = (Object.getOwnPropertyDescriptor(GPUDevice.prototype, 'lost')!.get as () => Promise<GPUDeviceLostInfo>).call(o.device);
      const H: Held = { o, gone: real, made: new Map(), pipes: new Map(), flying: new Map(), failed: new Set(), ids: new WeakMap(), next: 1 };
      held = H; K.opened++;
      void real.then(info => { if (held === H) held = null; for (const L of [...live.values()]) if (L.H === H) L.tell(info); });
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
    get seats(): string[] { return [...live.keys()]; }, get keptBytes(): number { return stashed; },
    release(seat: string): void { takes.set(seat, (takes.get(seat) ?? 0) + 1); clear(seat); },
    async take(seat: string = KEEP_PAGE.seat): Promise<Lease> {
      // (two takes of one seat at once: the LATER one holds it. The earlier is its page's last word, a page that is gone or going: it is refused at its
      // next step, so it can end no lease but the one it found and never becomes the holder behind the later one's back)
      const mine = (takes.get(seat) ?? 0) + 1, late = (): void => { if (mine !== takes.get(seat)) throw new Error('the kept page\'s device went to a later load'); };
      takes.set(seat, mine);
      clear(seat);
      let H: Held, fresh: boolean, waited: number | null = null;
      for (;;) {
        ({ H, fresh } = await device()); late();
        if (fresh) break;
        // (the queue is waited for when nobody holds the device: what a page that is GONE left there. A picture taking a seat beside its live page has
        // nothing to wait out: that page goes on feeding the queue, and on a slow adapter the picture would wait on its own page's frames)
        const ms = live.size === 0 ? await drained(H) : null; late();
        if (ms !== null) waited = (waited ?? 0) + ms;
        if (held === H) break;   // (else the device was lost while this load waited for its queue: another is opened, as for a loss between two loads)
      }
      clear(seat);
      let tell!: (i: GPUDeviceLostInfo) => void; const lost = new Promise<GPUDeviceLostInfo>(r => { tell = r; });
      const n = (given.get(seat) ?? 0) + 1; given.set(seat, n); K.leases++;
      const own: Record<PropertyKey, unknown> = Object.create(null) as Record<PropertyKey, unknown>, queueOwn: Record<PropertyKey, unknown> = Object.create(null) as Record<PropertyKey, unknown>;
      const L: Live = { n, seat, H, alive: true, things: [], heard: [], scopes: 0, lost, tell, stats: { asked: 0, pipelines: 0, modules: 0, labels: [] }, waits: new Map(), nextWait: 1, own: [own, queueOwn] };
      live.set(seat, L);
      const lease: Lease = { adapter: H.o.adapter, device: viewOf(H.o.device, makers(H, L, queueOwn), own), real: H.o.device, asks: H.o.asks, seat, n, fresh, waited, stats: L.stats, shellMs, shellNav, frozen: null, bytes: bytesOf, keepBytes, get alive() { return L.alive; }, freeze: () => { lease.frozen ??= { ...L.stats, labels: L.stats.labels.slice() }; }, release: () => end(L) };
      return lease;
    },
  };
  return K;
}

/** A PAGE'S SIDE: the nearest shell above this window (`same` = of this build, `depth` = how many windows up: 1 = the window above), or null. Never throws:
 *  a window of another origin above is no shell, and nothing is looked for past it */
export function keeperAbove(win: { parent?: unknown } = window, build?: string): { keeper: Keeper; same: boolean; depth: number } | null {
  try {
    let w = win as Record<string, unknown> & { parent?: unknown };
    for (let depth = 1; depth <= 4; depth++) {
      const up = w.parent as (Record<string, unknown> & { parent?: unknown }) | undefined; if (!up || up === w) return null;
      const k = up[KEEP_PAGE.hook] as Keeper | undefined;
      if (k && typeof k.take === 'function') return { keeper: k, same: k.v === KEEP_PAGE.v && (build === undefined || k.build === build) && !k.refused, depth };
      w = up;
    }
  } catch { /* another origin's window */ }
  return null;
}
/** the lease this page's device came on, or null on a page with a device of its own (gpu/device.ts initGpu sets it) */
/** …and `shelled`: this window is a frame of a kept page's shell (host/keeppage.ts sets it), whoever's device it draws with: the keys are the window's own,
 *  as on a page with nothing above it (host/input.ts) */
export const KEPT: { lease: Lease | null; shelled: boolean } = { lease: null, shelled: false };
/** THE LOAD'S CLOCK ON ITS GPU CALLS, any page's, kept or whole. A page thread that waits on the GPU process waits INSIDE a call that makes or fills a buffer
 *  or a texture, or hands the queue its work, and nothing else in a load's record can tell that wait from script. `n` calls, `ms` inside them in all, `mb`
 *  handed over, the `worst` one's ms and `what` it was, and `by` call the first three: [n, ms, mb] */
export interface KeepCalls { n: number; ms: number; mb: number; worst: number; what: string; by: Record<string, [number, number, number]> }
const NOT = (): void => { /* not running */ };
/** the page's clock: `now` = the load's tally (null = not timed), `stop` ends it (the boot's freeze). `play` = from the freeze on, running totals of the
 *  calls the page made of the queue's `writeBuffer`, `writeTexture` and `submit` and of the ms it spent inside them (null = not totalled): a frame's share
 *  is the difference of two reads of `play.ms`, which costs the reader nothing. A page thread blocked on the GPU process shows there; one blocked inside an
 *  encoder's call does not (those are not timed: there are too many a frame), nor one waiting in `createBuffer` or `createTexture`, so a small share is no
 *  proof that the frame's time was script. `startPlay` begins it (the boot's freeze) and `endPlay` takes it off */
export const CALLS: { now: KeepCalls | null; stop: () => void; play: { n: number; ms: number } | null; startPlay: () => void; endPlay: () => void } = { now: null, stop: NOT, play: null, startPlay: NOT, endPlay: NOT };
const sizeOf = (x: unknown): number => { const n = (x as { byteLength?: unknown } | null)?.byteLength; return typeof n === 'number' ? n : 0; };
/** the bytes one `writeBuffer(buffer, at, data, from?, size?)` hands over (`from` and `size` count the data's own elements) */
const written = (a: unknown[]): number => { const el = (a[2] as { BYTES_PER_ELEMENT?: number } | null)?.BYTES_PER_ELEMENT ?? 1; return typeof a[4] === 'number' ? a[4] * el : Math.max(0, sizeOf(a[2]) - (typeof a[3] === 'number' ? a[3] * el : 0)); };
/** …and one `writeTexture(to, data, layout, size)`: the rows the copy reads, never more than the data holds past its offset */
const textured = (a: unknown[]): number => {
  const lay = a[2] as { offset?: number; bytesPerRow?: number; rowsPerImage?: number } | null, z = a[3] as number[] | { height?: number; depthOrArrayLayers?: number } | null;
  const h = (Array.isArray(z) ? z[1] : z?.height) ?? 1, d = (Array.isArray(z) ? z[2] : z?.depthOrArrayLayers) ?? 1, whole = Math.max(0, sizeOf(a[1]) - (lay?.offset ?? 0));
  return lay?.bytesPerRow ? Math.min(whole, lay.bytesPerRow * ((lay.rowsPerImage ?? h) * (d - 1) + h)) : whole;
};
/** Start the page's clock on `device` (host/drawpage.ts, as soon as it has one; `on` = false, `?callclock=0`: not timed). The five calls are put over the device's and
 *  its queue's for as long as the clock runs, in THIS page's window, and what was there is put back at `CALLS.stop` (a kept page's device has the keeper's
 *  own makers there, a whole page's has nothing). A kept page's load that ends first loses them with its lease (`dress`) */
export function startCallClock(device: GPUDevice, on = wordOn(KEEP_PAGE.clockWord), play = wordOn(KEEP_PAGE.playWord)): void {
  CALLS.stop(); CALLS.endPlay(); CALLS.now = null; CALLS.play = null; CALLS.startPlay = NOT;
  if (!on || BREAK) return;
  const C: KeepCalls = { n: 0, ms: 0, mb: 0, worst: 0, what: '', by: {} }; let run = true;
  const put = (o: Record<string, unknown> | undefined, m: string, bytes: (a: unknown[]) => number): (() => void) => {
    const was = o?.[m]; if (!o || typeof was !== 'function') return () => { /* nothing put */ };
    const own = Object.prototype.hasOwnProperty.call(o, m), call = was as (...b: unknown[]) => unknown;
    const over = (...a: unknown[]): unknown => {
      if (!run) return call.apply(o, a);
      const t0 = performance.now();
      try { return call.apply(o, a); } finally {
        const ms = performance.now() - t0, mb = bytes(a) / 1048576, B = (C.by[m] ??= [0, 0, 0]);
        C.n++; C.ms += ms; C.mb += mb; B[0]++; B[1] += ms; B[2] += mb; if (ms > C.worst || C.what === '') { C.worst = ms; C.what = m; }
      }
    };
    try { o[m] = over; } catch { return () => { /* a thing that takes no property of ours is not timed */ }; }
    return () => { try { if (o[m] === over) { if (own) o[m] = was; else delete o[m]; } } catch { /* */ } };   // (ours only: what was put over it since is left)
  };
  const D = device as unknown as Record<string, unknown>, Q = (device as unknown as { queue?: Record<string, unknown> }).queue, none = (): number => 0;
  const offs = [put(D, 'createBuffer', none), put(D, 'createTexture', none), put(Q, 'writeBuffer', written), put(Q, 'writeTexture', textured), put(Q, 'submit', none)];
  CALLS.now = C; CALLS.stop = (): void => { run = false; for (const off of offs) off(); CALLS.stop = NOT; };
  // THE PLAY TOTAL (`CALLS.play`), from the boot's freeze: the queue's three calls stay under a clock of two reads of the time a call, with five arguments
  // named (the most any of them takes: nothing is made per call, and one a call was not given arrives as `undefined`, which is the same to the queue).
  // They stay for the page's life unless `endPlay` is called; a kept page's go with its lease too (`dress`), and a kept load whose lease is over by its
  // freeze puts nothing on a queue that is no longer its
  if (play) CALLS.startPlay = (): void => {
    CALLS.startPlay = NOT;
    if (!Q || (KEPT.lease && KEPT.lease.device === device && KEPT.lease.alive === false)) return;
    const P = { n: 0, ms: 0 }, taken: (() => void)[] = [], end = (): void => { for (const off of taken) off(); taken.length = 0; CALLS.endPlay = NOT; };
    for (const m of ['writeBuffer', 'writeTexture', 'submit']) {
      const was = Q[m]; if (typeof was !== 'function') { end(); return; }
      const own = Object.prototype.hasOwnProperty.call(Q, m), call = was as (...b: unknown[]) => unknown;
      const over = (a: unknown, b: unknown, c: unknown, d: unknown, e: unknown): unknown => { const t0 = performance.now(); try { return call.call(Q, a, b, c, d, e); } finally { P.ms += performance.now() - t0; P.n++; } };
      try { Q[m] = over; } catch { end(); return; }   // (a queue that takes no property of ours is not timed: what was put is taken off again)
      taken.push(() => { try { if (Q[m] === over) { if (own) Q[m] = was; else delete Q[m]; } } catch { /* */ } });
    }
    CALLS.play = P; CALLS.endPlay = end;
  };
}
function wordOn(word: string): boolean { try { return new URLSearchParams((globalThis as { location?: { search?: string } }).location?.search ?? '').get(word) !== '0'; } catch { return false; } }
/** the boot's own pipelines are in (host/drawpage.ts, where it freezes the boot's count): the record's `made` is the load's, however long the page then stays, and
 *  the load's clock on its GPU calls stops, and the play total starts */
export function keptFreeze(): void { KEPT.lease?.freeze(); CALLS.stop(); CALLS.startPlay(); }
/** a load's record, with the kept page's words: `kept` which load on this page, `made` what the browser compiled for it against what was asked, and on the first
 *  `shell`, the ms the shell took before the frame (the record's own clock starts at the frame's load), and `shellNav`, the browser's word for the shell's
 *  load (the frame's own `nav` is always a plain navigation); on a later one `waited`, the ms it waited for the GPU to finish what the load before it had
 *  queued (inside the record's own device time; no word = it did not wait: the wait switched off, or a device opened for it after a loss). EVERY page's
 *  record, a plain page's too, gets `calls` where the load's clock ran (`KeepCalls`, rounded); a plain page's is otherwise handed back as it is */
export function keptRecord<T extends Record<string, unknown> | null>(rec: T, lease: Lease | null = KEPT.lease): T {
  if (!rec) return rec;
  const C = CALLS.now, calls = C && C.n > 0 ? { calls: { n: C.n, ms: Math.round(C.ms), mb: Math.round(C.mb * 10) / 10, worst: Math.round(C.worst), what: C.what, by: Object.fromEntries(Object.entries(C.by).map(([k, v]) => [k, [v[0], Math.round(v[1]), Math.round(v[2] * 10) / 10]])) } } : null;
  if (!lease) return calls ? { ...rec, ...calls } : rec;
  const S = lease.frozen ?? lease.stats;
  return { ...rec, ...calls, kept: lease.n, made: { pipelines: S.pipelines, modules: S.modules, asked: S.asked, ...(lease.n > 1 && S.labels.length ? { labels: S.labels.slice() } : {}) }, ...(lease.n === 1 ? { shell: Math.round(lease.shellMs), ...(lease.shellNav ? { shellNav: lease.shellNav } : {}) } : lease.waited !== null ? { waited: Math.round(lease.waited) } : {}) };
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
