// THE KEPT PAGE (src/gpu/keep.ts): the tab's page keeps the GPU device and hands it to each load of the game's frame on a lease. What is checked here is the
// keeper's own logic over a stand-in device, with no browser: that a thing which never changes is made once a visit and handed back for an equal descriptor
// (a pipeline above all — across leases, and through a layout made anew by the next load), that what a load made of its own is destroyed when its lease ends
// and nothing it hung on the device outlives it (a listener, an open error scope, a method put over the device's, its `lost`), that a thing the device called
// invalid is not kept (a module with a compile error, a layout with a validation error, the stand-in made after a failed compile), that a lost device is dropped
// and the next lease opens another, and which window is a shell (`keepRole`). What it does to a real load is a page check (tools/qa/keeppage-page.cjs) and,
// for the seconds, a measurement on a machine with a GPU (docs/agents/render.md).
//   bun run tools/qa/keeppage-check.ts                    exit 0 = every check passed, 1 = one failed
//   KEEPPAGE_BREAK=1 bun run tools/qa/keeppage-check.ts   every maker goes straight to the device (nothing is kept): must FAIL
// the stand-in browser, in place BEFORE the module is loaded: a GPUDevice whose prototype makes things and counts them
const made: Record<string, number> = {}; const count = (k: string) => { made[k] = (made[k] ?? 0) + 1; };
// (a descriptor whose label or code says BAD is invalid: the device raises a validation error into the open error scope, or to its listeners, and a module
// reports a compile error. A thing made while an error scope is open is counted apart, as `shadow`: the keeper's own second making, which asks the device
// whether the first was valid)
const isBad = (d: unknown): boolean => /BAD/.test(String((d as { label?: string } | undefined)?.label ?? '') + String((d as { code?: string } | undefined)?.code ?? ''));
const open: (string | null)[] = []; let loud = 0;
class Thing { destroyed = false; constructor(readonly kind: string, readonly desc: unknown) { count(open.length && kind !== 'ShaderModule' ? 'shadow' : kind); if (isBad(desc) && kind !== 'ShaderModule') { if (open.length) open[open.length - 1] = 'invalid'; else loud++; } }
  get [Symbol.toStringTag]() { return 'GPU' + this.kind; } destroy() { this.destroyed = true; }
  getCompilationInfo() { return Promise.resolve({ messages: isBad(this.desc) ? [{ type: 'error' }] : [] }); } }
const flying: { desc: { label?: string }; done: (v: unknown) => void; fail: (e: unknown) => void }[] = [];
let scopes = 0, lostNow: (i: unknown) => void = () => { };
let holdQueue = false; const queued: (() => void)[] = [];   // (the GPU's queue: done at once, or held until the check lets it go)
class FakeQueue {   // (its calls are its prototype's, as a real queue's are)
  onSubmittedWorkDone(): Promise<void> { count('queueAsk'); return holdQueue ? new Promise<void>(r => { queued.push(r); }) : Promise.resolve(); }
  writeBuffer(..._a: unknown[]): void { count('write'); } writeTexture(..._a: unknown[]): void { count('write'); } submit(..._a: unknown[]): void { count('submit'); }
}
const QUEUE = new FakeQueue();
class FakeDevice extends EventTarget {
  #lost = new Promise<unknown>(r => { lostNow = r; });
  get lost() { return this.#lost; }
  onuncapturederror: unknown = null;
  createShaderModule(d: unknown) { return new Thing('ShaderModule', d); }
  createBindGroupLayout(d: unknown) { return new Thing('BindGroupLayout', d); }
  createPipelineLayout(d: unknown) { return new Thing('PipelineLayout', d); }
  createComputePipeline(d: unknown) { return new Thing('ComputePipeline', d); }
  createRenderPipeline(d: unknown) { return new Thing('RenderPipeline', d); }
  createComputePipelineAsync(d: { label?: string }) { count('asyncAsk'); return new Promise((done, fail) => { flying.push({ desc: d, done: () => done(new Thing('ComputePipeline', d)), fail }); }); }
  createRenderPipelineAsync(d: { label?: string }) { count('asyncAsk'); return new Promise((done, fail) => { flying.push({ desc: d, done: () => done(new Thing('RenderPipeline', d)), fail }); }); }
  createBuffer(d: unknown) { return new Thing('Buffer', d); }
  createTexture(d: unknown) { return new Thing('Texture', d); }
  createQuerySet(d: unknown) { return new Thing('QuerySet', d); }
  pushErrorScope() { scopes++; open.push(null); }
  popErrorScope() { scopes--; return Promise.resolve(open.pop() ?? null); }
  get queue() { return QUEUE; }   // (the prototype's, as a real device's is: a lease's start takes every own property off the device)
}
(globalThis as Record<string, unknown>).GPUDevice = FakeDevice;
const K = await import('../../src/gpu/keep');
export { };

let red = 0; const check = (ok: boolean, what: string) => { console.log(`${ok ? '- PASS' : '- FAIL'} — ${what}`); if (!ok) red++; };
const tick = () => new Promise<void>(r => setTimeout(r, 0));
const land = async () => { await tick(); for (const f of flying.splice(0)) f.done(null); await tick(); };
// (a pipeline the check waits for, every compile in flight let through first: under the red path an ask that should have been answered is a compile)
const got = async <T,>(p: Promise<T> | T): Promise<T> => { await land(); return p; };

// ---- which window is what
{ const R = K.keepRole, base = { word: null, tab: null, top: true, keeper: false, own: false, driven: false, onefile: false };
  check(R(base) === 'shell', 'a visitor\'s tab, no word: the shell');
  check(R({ ...base, driven: true }) === 'plain' && R({ ...base, driven: true, word: '1' }) === 'shell', 'a driven browser keeps its own page unless ?keeppage=1 asks');
  check(R({ ...base, proven: false }) === 'plain' && R({ ...base, proven: false, word: '1' }) === 'shell' && R({ ...base, proven: true }) === 'shell', 'a browser where a frame was never seen to draw with its parent\'s device keeps its own page unless ?keeppage=1 asks');
  check(R({ ...base, word: '0' }) === 'plain' && R({ ...base, tab: '0' }) === 'plain' && R({ ...base, tab: '0', word: '1' }) === 'shell', '?keeppage=0 is a whole page, and the tab\'s kept word too, until ?keeppage=1');
  check(R({ ...base, own: true }) === 'plain' && R({ ...base, onefile: true }) === 'plain' && R({ ...base, top: false }) === 'plain', 'the benchmark, the kit picture, the one-file page and a page in somebody else\'s frame keep their own page');
  check(R({ ...base, top: false, keeper: true }) === 'kept', 'a frame under its shell takes the shell\'s device');
  check(R({ ...base, top: false, keeper: true, own: true }) === 'leave' && R({ ...base, top: false, keeper: true, word: '0' }) === 'leave' && R({ ...base, top: false, keeper: false, stale: true }) === 'leave', 'a frame that must load whole, or whose shell is another build\'s, sends the tab to its address'); }

// ---- the keeper
let opens = 0; const devices: FakeDevice[] = [];
const keeper = K.makeKeeper(async () => { opens++; const d = new FakeDevice(); devices.push(d); return { adapter: { name: 'stand-in' } as unknown as GPUAdapter, device: d as unknown as GPUDevice, asks: ['one'] }; }, 'build-a', 321, 'reload');
type Dev = Record<string, (d?: unknown) => any> & { lost: Promise<unknown>; addEventListener: FakeDevice['addEventListener']; dispatchEvent: FakeDevice['dispatchEvent'] };
const one = await keeper.take(), d1 = one.device as unknown as Dev;
check(one.n === 1 && one.fresh && opens === 1 && one.shellMs === 321, 'the first lease opens the device');
check(one.waited === null && !made.queueAsk && keeper.drain && keeper.drainMs === K.KEEP_PAGE.drainMs, '…and waits on no queue: nothing was on it before');
const CODE = 'fn main() {}';
const m1 = d1.createShaderModule({ code: CODE, label: 'a' }), m1b = d1.createShaderModule({ code: CODE, label: 'a' }), m2 = d1.createShaderModule({ code: CODE + ' ', label: 'a' });
check(m1 === m1b && m1 !== m2 && made.ShaderModule === 2 && one.stats.modules === 2, 'a shader module: the same text and label, the same module; another text, another');
const bglDesc = () => ({ entries: [{ binding: 0, visibility: 4, buffer: { type: 'uniform' } }] });
const layoutOf = (dev: Dev) => dev.createPipelineLayout({ bindGroupLayouts: [dev.createBindGroupLayout(bglDesc())] });
const pipeDesc = (dev: Dev, entry = 'main') => ({ label: 'kernel', layout: layoutOf(dev), compute: { module: dev.createShaderModule({ code: CODE, label: 'a' }), entryPoint: entry, constants: { N: 4 } } });
const pA = d1.createComputePipelineAsync(pipeDesc(d1)), pB = d1.createComputePipelineAsync(pipeDesc(d1));
await tick(); check(flying.length === 1 && made.asyncAsk === 1, 'two asks for one pipeline while it compiles are one compile');
const a = await got(pA), b = await got(pB);
check(a === b && made.BindGroupLayout === 1 && made.PipelineLayout === 1, '…and one pipeline; the layout made twice from an equal descriptor is one layout');
const again = await got(d1.createComputePipelineAsync(pipeDesc(d1))), sync = d1.createComputePipeline(pipeDesc(d1));
check(again === a && sync === a && made.asyncAsk === 1 && made.ComputePipeline === 1, 'asked again, waiting or at once: the pipeline that is in, nothing compiled');
const other = d1.createComputePipelineAsync(pipeDesc(d1, 'other')); await land();
check(await got(other) !== a && made.asyncAsk === 2, 'another entry point is another pipeline');
check(one.stats.asked === 5 && one.stats.pipelines === 2, `the lease's count: 5 asked, 2 compiled (${one.stats.asked}, ${one.stats.pipelines})`);
// a compile that fails is not kept
const bad = d1.createRenderPipelineAsync({ label: 'broken', vertex: { module: m1, entryPoint: 'v' } }); bad.catch(() => { }); await tick(); flying.splice(0)[0].fail(new Error('no')); await tick();
const good = d1.createRenderPipelineAsync({ label: 'broken', vertex: { module: m1, entryPoint: 'v' } }); await land();
check((await good as Thing).kind === 'RenderPipeline' && await bad.then(() => false, () => true), 'a pipeline that failed is told to its caller and tried again at the next ask');
// …nor is the stand-in its caller makes at once after the failure (the page's own fallback)
{ const desc = () => ({ label: 'broken too', vertex: { module: m1, entryPoint: 'v' } }), asks = made.asyncAsk, pipes = one.stats.pipelines, asked = one.stats.asked;
  const fails = d1.createRenderPipelineAsync(desc()); fails.catch(() => { }); await tick(); flying.splice(0)[0].fail(new Error('no')); await tick();
  const stand = d1.createRenderPipeline(desc()), next = d1.createRenderPipelineAsync(desc()); await tick();
  check(made.asyncAsk === asks + 2 && flying.length === 1, 'the stand-in a caller makes at once after a failed compile is not kept: the next ask compiles');
  await land(); check(await next !== stand && one.stats.pipelines === pipes + 2 && one.stats.asked === asked + 2, '…and it is not counted as a pipeline of the load\'s'); }
// a thing the device called invalid is not kept
{ const mods = made.ShaderModule, badMod = d1.createShaderModule({ code: 'BAD', label: 'm' }); await tick(); await tick();
  check(d1.createShaderModule({ code: 'BAD', label: 'm' }) !== badMod && made.ShaderModule === mods + 2, 'a module with a compile error is not kept: the next load makes it again and hears the error');
  const lay = made.BindGroupLayout, shadows = made.shadow ?? 0, wasLoud = loud, badLay = d1.createBindGroupLayout({ label: 'BAD', entries: [] });
  check(loud === wasLoud + 1 && (made.shadow ?? 0) === shadows + 1, '(a layout the device calls invalid: the page hears the error once, and the keeper asks the device once more, in a scope of its own)');
  await tick(); await tick();
  check(d1.createBindGroupLayout({ label: 'BAD', entries: [] }) !== badLay && made.BindGroupLayout === lay + 2 && scopes === 0, 'a layout with a validation error is not kept');
  const fine = d1.createBindGroupLayout({ label: 'fine', entries: [] }); await tick(); await tick();
  check(d1.createBindGroupLayout({ label: 'fine', entries: [] }) === fine, '…and a valid one is');
  const pipes = made.RenderPipeline, badPipe = d1.createRenderPipeline({ label: 'BAD', vertex: { module: m1, entryPoint: 'v' } }); await tick(); await tick();
  check(d1.createRenderPipeline({ label: 'BAD', vertex: { module: m1, entryPoint: 'v' } }) !== badPipe && made.RenderPipeline === pipes + 2, 'a pipeline made at once that the device calls invalid is not kept'); }
// a compile still in flight when the load ends (the next section)
let heardSlow = 0; const slow1 = d1.createComputePipelineAsync(pipeDesc(d1, 'slow')); void slow1.then(() => { heardSlow++; }, () => { heardSlow++; }); await tick(); const slowAsks = made.asyncAsk;
const firstMade = { pipelines: one.stats.pipelines, modules: one.stats.modules };
// what the load makes of its own, and what it hangs on the device
const bufs = [d1.createBuffer({ size: 16 }), d1.createBuffer({ size: 32 }), d1.createTexture({ size: [4, 4] }), d1.createQuerySet({ count: 2 })] as Thing[];
let heard1 = 0, heard2 = 0; d1.addEventListener('uncapturederror', () => { heard1++; });
d1.pushErrorScope('validation'); d1.pushErrorScope('validation'); await d1.popErrorScope();
let wrapped = 0; const under = d1.createShaderModule.bind(d1); (d1 as Record<string, unknown>).createShaderModule = (x: unknown) => { wrapped++; return under(x); };
d1.createShaderModule({ code: CODE, label: 'a' });
let lost1: unknown = null; void d1.lost.then(i => { lost1 = i; }); const lostOf1 = d1.lost;
check(wrapped === 1 && scopes === 1, '(a load\'s own method over the device\'s is called while its lease runs; one error scope left open)');

// ---- the next load
const two = await keeper.take(), d2 = two.device as unknown as Dev;
check(two.n === 2 && !two.fresh && opens === 1 && two.device === one.device, 'the second lease is the same device, not opened again');
check(bufs.every(t => t.destroyed), 'the first load\'s buffers, texture and query set are destroyed when its lease ends');
check(scopes === 0, 'the error scope it left open is closed');
d2.addEventListener('uncapturederror', () => { heard2++; }); d2.dispatchEvent(new Event('uncapturederror'));
check(heard1 === 0 && heard2 === 1, 'its listener on the device is gone; the new load\'s hears');
d2.createShaderModule({ code: CODE, label: 'a' });
check(wrapped === 1, 'the method it put over the device\'s is gone');
check(d2.lost !== lostOf1, 'each lease has a `lost` of its own');
const slow2 = d2.createComputePipelineAsync(pipeDesc(d2, 'slow')); await tick();
check(made.asyncAsk === slowAsks && flying.length === 1, 'a compile in flight when its load ended answers the next load\'s ask for the same pipeline: one compile');
check((await got(slow2) as Thing).kind === 'ComputePipeline' && heardSlow === 0, '…which lands for the load that is there; the load that is gone hears nothing of it');
const before = { ...made };
const kept = await got(d2.createComputePipelineAsync(pipeDesc(d2))), keptSync = d2.createComputePipeline(pipeDesc(d2, 'other'));
check(kept === a && keptSync === await other && made.asyncAsk === before.asyncAsk && made.ComputePipeline === before.ComputePipeline && made.ShaderModule === before.ShaderModule && made.BindGroupLayout === before.BindGroupLayout && made.PipelineLayout === before.PipelineLayout,
  'THE POINT: the second load asks for its pipelines through layouts and modules it makes anew, and gets the first load\'s, nothing compiled');
check(two.stats.asked === 3 && two.stats.pipelines === 0 && two.stats.modules === 0, `…and its count says so: 3 asked, 0 compiled (${two.stats.asked}, ${two.stats.pipelines}, modules ${two.stats.modules})`);
const b2 = d2.createBuffer({ size: 8 }) as Thing; one.release();
check(!b2.destroyed, 'a lease that is over, released again, touches nothing of the load that holds the device');
// the record's words
const rec = K.keptRecord({ whole: 900, nth: 2 } as Record<string, unknown>, two), first = K.keptRecord({ whole: 9000 } as Record<string, unknown>, one);
check(rec.kept === 2 && (rec.made as { pipelines: number; asked: number }).pipelines === 0 && (rec.made as { asked: number }).asked === 3 && !('shell' in rec) && rec.whole === 900, 'a kept start\'s load record says which load it is and that nothing was compiled');
check(first.kept === 1 && first.shell === 321 && first.shellNav === 'reload' && (first.made as { pipelines: number }).pipelines === firstMade.pipelines && !('labels' in (first.made as object)) && !('shellNav' in rec), `the first load's says what it compiled, what the shell took and how the shell was loaded (${JSON.stringify(first.made)})`);
{ two.freeze(); const was = two.stats.pipelines; d2.createComputePipeline(pipeDesc(d2, 'after the boot')); const late = K.keptRecord({ whole: 900 } as Record<string, unknown>, two);
  check(two.stats.pipelines === was + 1 && (late.made as { pipelines: number }).pipelines === was, 'a record sent late says what the BOOT compiled, not what play compiled after it'); }
check(K.keptRecord({ whole: 1 } as Record<string, unknown>, null).kept === undefined && K.keptRecord(null, two) === null, 'a plain page\'s record, and no record, are handed back as they are');

// ---- a load that is over leaves nothing of its own on the device, at once (not at the next lease)
{ let mine = 0; (d2 as Record<string, unknown>).createTexture = () => { mine++; return null; }; two.release(); (two.device as unknown as Dev).createTexture({ size: [1, 1] });
  check(mine === 0, 'a method a load put over the device\'s is gone the moment its lease ends'); }
const twoB = await keeper.take(), d2b = twoB.device as unknown as Dev;
check(twoB.n === 3 && twoB.device === one.device && await got(d2b.createComputePipelineAsync(pipeDesc(d2b))) === a, '(the next lease on the same device still gets the first load\'s pipeline)');

// ---- a lost device
let lost2: unknown = null; void d2b.lost.then(i => { lost2 = i; }); lostNow({ reason: 'unknown', message: 'gone' }); await tick(); await tick();
check(lost1 === null && (lost2 as { message?: string } | null)?.message === 'gone', 'a lost device is told to the load that holds it, and to no load before it');
const three = await keeper.take(), d3 = three.device as unknown as Dev;
check(three.n === 4 && three.fresh && opens === 2 && three.device !== two.device && keeper.opened === 2, 'the lease after a loss opens another device');
const p3 = d3.createComputePipelineAsync(pipeDesc(d3)); await land();
check(await p3 !== a && three.stats.pipelines === 1, '…on which nothing is kept from the lost one: it compiles');

// ---- what a load hung on a compile still in flight is let go with its lease
{ const four = await keeper.take(), d4 = four.device as unknown as Dev;
  const hang = (dev: Dev, entry: string): WeakRef<object> => { const tok = { held: new Array(64).fill(0) }; void dev.createComputePipelineAsync(pipeDesc(dev, entry)).then(() => tok, () => tok); return new WeakRef(tok); };
  const freed = async (ref: WeakRef<object>): Promise<boolean> => { const gc = (globalThis as { Bun?: { gc(full: boolean): void } }).Bun?.gc; for (let i = 0; i < 6 && ref.deref() !== undefined; i++) { await tick(); gc?.(true); } return ref.deref() === undefined; };
  const ref = hang(d4, 'held'); await tick(); const was = !(await freed(ref)); four.release();
  check(flying.length === 1 && was && await freed(ref), 'what a load hung on a compile still in flight is held while its lease runs, and let go when the lease ends, not when the compile lands'); await land(); }

// ---- a load waits for what the load before it left in the GPU's queue
{ const asks0 = made.queueAsk ?? 0; type L = Awaited<ReturnType<typeof keeper.take>>;
  const five = await keeper.take(), d5 = five.device as unknown as Dev; const buf = d5.createBuffer({ size: 4 }) as Thing;
  check(!five.fresh && five.waited !== null && five.waited >= 0 && (made.queueAsk ?? 0) === asks0 + 1, 'a load on a device another load used asks the GPU to finish what is queued before it takes the device');
  holdQueue = true; let six: L | null = null; const taking = keeper.take().then(l => { six = l; return l; });
  await tick(); await tick(); await tick();
  check(six === null && queued.length === 1 && buf.destroyed, 'while the GPU is still on the last load\'s work the next load does not have the device; the last load\'s things are destroyed already');
  for (const go of queued.splice(0)) go(); const L6 = await taking;
  check(L6.n === five.n + 1 && L6.waited !== null && (K.keptRecord({ whole: 1 } as Record<string, unknown>, L6) as { waited?: number }).waited === Math.round(L6.waited), '…and has it once the queue is done; its record says how long it waited');
  // two loads at once, both waiting: the later one holds the device, and the earlier one's take ends in a refusal, not in a lease that ends the later one's
  const leases0 = keeper.leases; let early: unknown = null; const tA = keeper.take().then(l => l, e => { early = e; return null; }), tB = keeper.take();
  await tick(); await tick(); for (const go of queued.splice(0)) go();
  const LA = await tA, LB = await tB, bufB = (LB.device as unknown as Dev).createBuffer({ size: 4 }) as Thing; await tick(); await tick();
  check(LA === null && early instanceof Error && keeper.leases === leases0 + 1 && LB.n === leases0 + 1 && !bufB.destroyed, 'two loads at once through the wait: the later one has the device and keeps what it makes; the earlier one is refused');
  // a queue that never answers
  const capWas = keeper.drainMs; keeper.drainMs = 30; const t0 = performance.now(), seven = await keeper.take(), took = performance.now() - t0; keeper.drainMs = capWas;
  check(queued.length === 1 && took >= 25 && took < 2000 && (seven.waited ?? 0) >= 25, `a queue that never answers holds a load for the cap and no longer (${took.toFixed(0)} ms)`); queued.length = 0;
  // the switch
  keeper.drain = false; const asks1 = made.queueAsk ?? 0, eight = await keeper.take(); keeper.drain = true;
  check((made.queueAsk ?? 0) === asks1 && eight.waited === null && !('waited' in (K.keptRecord({ whole: 1 } as Record<string, unknown>, eight) as object)), 'with the wait switched off (?keepdrain=0) a load has the device at once, the queue is not asked and its record has no word of a wait');
  // the device lost while a load waits for its queue: the load gets another device, as it does for a loss between two loads
  const opens0 = opens, dead = eight.device; let nine: L | null = null; const t9 = keeper.take().then(l => { nine = l; return l; });
  await tick(); await tick(); const stillWaiting = nine === null && queued.length === 1;
  lostNow({ reason: 'unknown', message: 'gone in the wait' }); const L9 = await t9;
  check(stillWaiting && L9.fresh && opens === opens0 + 1 && L9.device !== dead && L9.waited !== null, 'a device lost while a load waits for its queue: the load is handed another device, opened for it'); queued.length = 0; holdQueue = false; L9.release(); }

// ---- the load's clock on its GPU calls (any page's): its time inside the calls that make or fill a buffer or a texture, and the queue's submit
{ const Q = QUEUE as unknown as Record<string, (...a: unknown[]) => void>, own = (o: object, m: string) => Object.prototype.hasOwnProperty.call(o, m);
  type Calls = { n: number; ms: number; mb: number; worst: number; what: string; by: Record<string, [number, number, number]> };
  const word = (l: Awaited<ReturnType<typeof keeper.take>> | null) => K.keptRecord({ whole: 1 } as Record<string, unknown>, l) as { calls?: Calls; kept?: number };
  const use = (d: Dev, buf: unknown): void => { d.createBuffer({ size: 64 }); d.createTexture({ size: [4, 4] }); Q.writeBuffer(buf, 0, new Float32Array(262144)); Q.writeBuffer(buf, 0, new Float32Array(262144), 0, 131072);
    Q.writeTexture({}, new Uint8Array(1048576), { offset: 0, bytesPerRow: 1024 }, [256, 512]); Q.writeTexture({}, new Uint8Array(1048576), { offset: 4096, bytesPerRow: 1024 }, { width: 4, height: 4 }); Q.submit([]); };
  // a whole page: a device of its own, nothing of a keeper's on it
  const plain = new FakeDevice() as unknown as Dev, w0 = made.write ?? 0;
  K.startCallClock(plain as unknown as GPUDevice, true, false); use(plain, {});
  const C = K.CALLS.now, by = word(null).calls?.by ?? {}; for (const k of ['createBuffer', 'createTexture', 'writeBuffer', 'writeTexture', 'submit']) by[k] ??= [0, 0, 0];
  check(!!C && C.n === 7 && Math.abs(C.mb - (1 + 0.5 + 0.5 + 4096 / 1048576)) < 1e-6 && C.ms >= 0 && C.what !== '' && (made.write ?? 0) === w0 + 4, 'a load\'s buffer, texture and submit calls are timed and still reach the device: 2 MB handed over in four writes (a rectangle of a field counts its own rows)');
  check(word(null).calls?.n === 7 && word(null).kept === undefined && by.createBuffer[0] === 1 && by.createTexture[0] === 1 && by.writeBuffer[0] === 2 && by.writeBuffer[2] === 1.5 && by.writeTexture[0] === 2 && by.writeTexture[2] === 0.5 && by.submit[0] === 1, '…and a WHOLE page\'s record says so, by call (`calls`), with no word of a kept page');
  const pages = (...a: unknown[]): void => { FakeQueue.prototype.submit.apply(QUEUE, a); }; Q.submit = pages;   // (a page's own wrapper over the queue: not the clock's to take off)
  K.keptFreeze(); use(plain, {});
  check(!own(QUEUE, 'writeBuffer') && !own(QUEUE, 'writeTexture') && !own(plain, 'createBuffer') && !own(plain, 'createTexture') && Q.submit === pages && K.CALLS.now?.n === 7, 'at the boot\'s freeze the clock stops: the device\'s and the queue\'s calls are their own again, what a page put over one stays, and a later call is not counted');
  delete (Q as Record<string, unknown>).submit;
  // a kept page: the keeper's own makers are on the device, and are there again when the clock stops
  const A = await keeper.take(), dA = A.device as unknown as Dev, keepers = dA.createBuffer;
  K.startCallClock(A.device, true, false); const timedMaker = dA.createBuffer, mineA = dA.createBuffer({ size: 8 }) as Thing; use(dA, mineA); K.keptFreeze();
  check(timedMaker !== keepers && dA.createBuffer === keepers && !own(QUEUE, 'submit') && word(A).calls?.n === 8 && word(A).kept === A.n, 'on a kept page the clock goes over the keeper\'s makers and they are back at the freeze; the record has `calls` beside `kept`');
  A.release(); check(mineA.destroyed, '…and what the load made under the clock was still the lease\'s to destroy');
  const B = await keeper.take(); K.startCallClock(B.device, true, false); const hadB = own(QUEUE, 'writeBuffer'), clockB = (B.device as unknown as Dev).createBuffer; B.release();
  check(hadB && !own(QUEUE, 'writeBuffer') && !own(QUEUE, 'submit') && (B.device as unknown as Dev).createBuffer !== clockB, 'a kept load that ends before its freeze leaves nothing of its clock on the device or its queue');
  // the play total: from the freeze on, the queue's three calls are counted, with the arguments they were given
  { const got: unknown[][] = [], P = FakeQueue.prototype, was = [P.writeBuffer, P.writeTexture, P.submit];
    P.writeBuffer = function (...a: unknown[]) { got.push(a); }; P.writeTexture = function (...a: unknown[]) { got.push(a); }; let refuse = false; P.submit = function (...a: unknown[]) { if (refuse) throw new Error('refused'); got.push(a); };
    K.startCallClock(plain as unknown as GPUDevice, true, true); const before = K.CALLS.play; Q.submit([]); K.keptFreeze();
    const data = new Float32Array(4), lay = { bytesPerRow: 16 }; got.length = 0; Q.writeBuffer('b', 8, data); Q.writeBuffer('b', 8, data, 1, 2); Q.writeTexture('t', data, lay, [1, 1]); Q.submit(['c']);
    const T = K.CALLS.play;
    check(before === null && !!T && T.n === 4 && T.ms >= 0 && K.CALLS.now?.n === 1 && got.length === 4 && got[0][0] === 'b' && got[0][1] === 8 && got[0][2] === data && got[0][3] === undefined && got[0][4] === undefined && got[1][3] === 1 && got[1][4] === 2 && got[2][2] === lay && (got[3][0] as unknown[])[0] === 'c',
      'from the boot\'s freeze a frame of play\'s queue calls are totalled (`CALLS.play`), each reaching the queue with the arguments it was given; the load\'s tally stays the load\'s');
    // a call that throws is still a call the page sat in
    refuse = true; let threw = false; try { Q.submit([]); } catch { threw = true; } refuse = false;
    check(threw && T?.n === 5, '…and a queue call that throws is counted, and still throws');
    // the clock started again takes the old total off the queue before anything else
    K.startCallClock(plain as unknown as GPUDevice, true, false); const cleared = K.CALLS.play === null; K.keptFreeze();   // (were the old total still there, the load's clock would have put it back at this freeze)
    check(cleared && K.CALLS.play === null && !own(QUEUE, 'writeBuffer') && !own(QUEUE, 'submit'), 'a clock started again takes the last play total off the queue; with the play total switched off (?playclock=0) nothing stays over the queue after the freeze');
    // …and a kept page's go with its lease; a kept load whose lease is over by its freeze puts nothing on the queue
    const E = await keeper.take(); K.KEPT.lease = E; K.startCallClock(E.device, true, true); K.keptFreeze(); const hadE = own(QUEUE, 'submit') && !!K.CALLS.play; E.release();
    check(hadE && !own(QUEUE, 'writeBuffer') && !own(QUEUE, 'writeTexture') && !own(QUEUE, 'submit'), 'a kept load\'s play total is taken off the queue at the end of its lease');
    const F = await keeper.take(); K.KEPT.lease = F; K.startCallClock(F.device, true, true); F.release(); K.keptFreeze();
    check(!F.alive && K.CALLS.play === null && !own(QUEUE, 'writeBuffer') && !own(QUEUE, 'submit'), 'a kept load whose lease is over by its freeze puts no play total on a queue that is no longer its');
    K.KEPT.lease = null;
    P.writeBuffer = was[0]; P.writeTexture = was[1]; P.submit = was[2]; }
  K.CALLS.stop(); K.startCallClock(plain as unknown as GPUDevice, false); use(plain, {}); K.keptFreeze();
  check(K.CALLS.now === null && K.CALLS.play === null && !own(QUEUE, 'writeBuffer') && !own(plain, 'createBuffer') && word(null).calls === undefined, 'with the clock switched off (?callclock=0) nothing is put over a call, in the load or after it, and the record has no `calls`'); }

// ---- the frame's side
{ const up = { [K.KEEP_PAGE.hook]: keeper } as Record<string, unknown>, win = { parent: up };
  check(K.keeperAbove(win, 'build-a')?.same === true && K.keeperAbove(win, 'build-b')?.same === false, 'a frame knows its shell, and a shell of another build');
  check(K.keeperAbove({ parent: {} }) === null && K.keeperAbove({ get parent(): unknown { throw new Error('another origin'); } }) === null, 'no shell above, or a window of another origin above: none');
  const self: { parent?: unknown } = {}; self.parent = self; check(K.keeperAbove(self) === null, 'a window with nothing above it has no shell');
  keeper.refused = true; check(K.keeperAbove(win, 'build-a')?.same === false, 'a shell whose device a browser would not draw with is not taken from again'); }

console.log(red ? `keeppage-check: ${red} FAILED` : 'keeppage-check: all passed'); process.exit(red ? 1 : 0);
