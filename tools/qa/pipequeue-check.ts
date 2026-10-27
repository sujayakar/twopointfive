// THE COMPILE QUEUE (src/gpu/device.ts PIPE_QUEUE): every …Async pipeline goes to the browser through a queue, so that a burst of slow compiles cannot hold every
// one of the browser's GPU workers and stall the drawing of the page. What is checked here is the queue's own logic over a stand-in device, with no browser: how many are
// in flight during the boot and after it, which one goes next, that a failed or broken creation gives its slot back, and that a pipeline's time is counted from its turn.
// What a queue does to a real load is a measurement on a Windows machine (docs/agents/render.md), not a check.
//   bun run tools/qa/pipequeue-check.ts                    exit 0 = every check passed, 1 = one failed
//   PIPEQUEUE_BREAK=1 bun run tools/qa/pipequeue-check.ts  the device is left as the browser made it (?pipequeue=0): must FAIL
import { PIPE_QUEUE, pipeQueue, pipeQueueFor, installPipeQueue, computePipelineAsync, laterPipelineAsync, laterProgress, pipelineProgress } from '../../src/gpu/device';

let red = 0; const check = (ok: boolean, what: string) => { console.log(`${ok ? '- PASS' : '- FAIL'} — ${what}`); if (!ok) red++; };
const tick = () => new Promise<void>(r => setTimeout(r, 0));

// the stand-in: every creation is held until the check lets it go
interface Held { label: string; done: (v?: unknown) => void; fail: (e: unknown) => void }
const started: Held[] = [];
const stub = {
  createShaderModule: (d: { code: string }) => ({ code: d.code }),
  createComputePipelineAsync: (d: { label?: string; compute: { module: unknown }; answer?: unknown; throws?: boolean }) => {
    if (d.throws) throw new Error('refused');
    if (d.compute === undefined) throw new TypeError('no compute stage');
    if ('answer' in d) return d.answer;
    return new Promise((done, fail) => { started.push({ label: d.label ?? '?', done, fail }); });
  },
  createRenderPipelineAsync: (d: { label?: string }) => new Promise((done, fail) => { started.push({ label: d.label ?? '?', done, fail }); }),
  createComputePipeline: (d: { label?: string }) => ({ sync: d.label }),
};
const dev = stub as unknown as GPUDevice & { createComputePipelineAsync(d: object): Promise<unknown> };
// ---- where it is on: a browser on Windows, and wherever the page's word asks for it
const WIN = 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/155.0.0.0 Safari/537.36';
const MAC = 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/155.0.0.0 Safari/537.36';
const LINUX = 'Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) HeadlessChrome/141.0.0.0 Safari/537.36';
const BREAK = process.env.PIPEQUEUE_BREAK === '1';
check(pipeQueueFor(WIN, BREAK ? '0' : null) === true, 'a browser on Windows gets the queue with no word asked');
check(pipeQueueFor(MAC, null) === false && pipeQueueFor(LINUX, null) === false, "… a Mac and the tools' own browser do not: every creation straight to the browser, as it was");
check(pipeQueueFor(WIN, '0') === false, '?pipequeue=0 puts it off on Windows');
check(pipeQueueFor(MAC, BREAK ? '0' : '1') === true && pipeQueueFor(LINUX, BREAK ? '0' : '6') === true, '?pipequeue=1 (or a number in flight) puts it on anywhere');
if (!BREAK) installPipeQueue(dev);

const CUT = 'const CUT_BOXES: bool = true;\n';
const mod = (bytes: number, cut = false) => dev.createShaderModule({ code: (cut ? CUT : '') + 'x'.repeat(bytes) });
const ask = (label: string, bytes: number, cut = false) => dev.createComputePipelineAsync({ label, compute: { module: mod(bytes, cut) } });
const labels = () => started.map(h => h.label).join(' ');
const finish = async (label: string) => { const i = started.findIndex(h => h.label === label); if (i < 0) return false; started.splice(i, 1)[0].done(label); await tick(); return true; };
const drain = async () => { for (let n = 0; n < 64 && (started.length || pipeQueue.waiting.length); n++) { for (const h of started.splice(0)) h.done(h.label); await tick(); } };

// ---- the boot: a limit, the biggest first
PIPE_QUEUE.boot = 3;
{ const got: string[] = [];
  for (const [l, b] of [['s1', 10], ['big', 900], ['s2', 20], ['mid', 500], ['huge', 2000], ['s3', 30]] as const) void ask(l, b).then(v => got.push(String(v)));
  await tick();
  check(started.length === 3, `boot: six asked in one go, three handed to the browser (${started.length})`);
  check(labels() === 'huge big mid', `… and they are the three biggest modules, biggest first (${labels()})`);
  check(pipeQueue.waiting.length === 3 && pipeQueue.maxWaiting >= 6, `… with three waiting and the most that ever waited kept (${pipeQueue.waiting.length}, ${pipeQueue.maxWaiting})`);
  await finish('big');
  check(labels() === 'huge mid s3', `one done: the biggest of the rest takes its place (${labels()})`);
  await drain();
  check(got.length === 6 && got.includes('s1') && pipeQueue.inflight === 0, `every one of them came back with its own pipeline (${got.length} of 6, ${pipeQueue.inflight} left in flight)`); }
{ PIPE_QUEUE.boot = 0; const n = pipeQueue.limit(); check(n >= 4, `boot limit left to the machine: its threads less two, never under four (${n})`); PIPE_QUEUE.boot = 3; }

// ---- a creation that fails, throws, or is malformed gives its slot back and reaches the caller as a rejection
{ let why = ''; const p = ask('fails', 10).catch(e => { why = String(e); }); await tick();
  started.splice(0)[0]?.fail('lost'); await p;
  check(why === 'lost' && pipeQueue.inflight === 0, `a rejected creation rejects for its caller and frees its slot (${why || 'no rejection'}, ${pipeQueue.inflight} in flight)`); }
{ let why = ''; let threw = false;
  try { await dev.createComputePipelineAsync({ label: 'throws', throws: true, compute: { module: mod(10) } }).catch(e => { why = String(e?.message ?? e); }); } catch { threw = true; }
  check(!threw && why === 'refused' && pipeQueue.inflight === 0, `a creation the browser throws on is a rejection, and its slot is free (${threw ? 'threw' : why}, ${pipeQueue.inflight} in flight)`); }
{ let why = ''; let threw = false;
  try { await dev.createComputePipelineAsync({ label: 'broken' }).catch(e => { why = String(e?.message ?? e); }); } catch { threw = true; }
  check(!threw && why === 'no compute stage' && pipeQueue.inflight === 0, `a descriptor with no stage is the browser's to reject, never a throw of the queue's (${threw ? 'threw' : why})`); }
{ const v = await dev.createComputePipelineAsync({ label: 'plain', answer: 7, compute: { module: mod(10) } }); await tick();
  check(v === 7 && pipeQueue.inflight === 0, `an answer that is no promise (a stand-in device) comes through and frees its slot (${v}, ${pipeQueue.inflight} in flight)`); }

// ---- a time is counted from the pipeline's turn, not from the asking (on a clock the check moves by hand: no wall time in a verdict)
{ PIPE_QUEUE.boot = 1; pipelineProgress.reset();
  const realNow = performance.now; let now = 1000; performance.now = () => now;
  const a = computePipelineAsync(dev, { label: 'long', compute: { module: mod(900) } } as unknown as GPUComputePipelineDescriptor);
  const b = computePipelineAsync(dev, { label: 'short', compute: { module: mod(10) } } as unknown as GPUComputePipelineDescriptor);
  await tick(); now += 120; await finish('long'); await tick(); now += 10; await finish('short'); await Promise.all([a, b]);
  performance.now = realNow;
  const ms = (l: string) => pipelineProgress.times.find(t => t.label === l)?.ms ?? -1;
  check(ms('long') === 120, `the long one's time is its own (${ms('long')} ms of the 120 it was held)`);
  check(ms('short') === 10, `the short one behind it is not charged the wait (${ms('short')} ms: 10 of its own, the 120 it waited left out)`);
  PIPE_QUEUE.boot = 3; }

// ---- THE LATER LANE (laterPipelineAsync: what the row 'Start on first frame shaders' leaves out of the start): held while a pipeline of the boot's own waits or is in
// flight, not counted on the start card, handed over once those are all in
const later = (label: string, bytes: number, extra: object = {}) => laterPipelineAsync(dev, { label, compute: { module: mod(bytes) }, ...extra } as unknown as GPUComputePipelineDescriptor);
{ PIPE_QUEUE.boot = 3; pipelineProgress.reset(); const got: string[] = [];
  void later('laterHuge', 5000).then(v => got.push(String(v))); void later('laterSmall', 10).then(v => got.push(String(v)));
  const b1 = computePipelineAsync(dev, { label: 'boot1', compute: { module: mod(100) } } as unknown as GPUComputePipelineDescriptor), b2 = computePipelineAsync(dev, { label: 'boot2', compute: { module: mod(50) } } as unknown as GPUComputePipelineDescriptor);
  await tick();
  check(labels() === 'boot1 boot2', `boot: a LATER pipeline is not handed over while the boot's own wait or are in flight, however big its module (${labels()})`);
  check(pipelineProgress.total === 2, `the start card counts the boot's two and neither LATER one (${pipelineProgress.total})`);
  await finish('boot1');
  check(labels() === 'boot2', `… nor while one of the boot's is still in flight (${labels()})`);
  await finish('boot2'); await Promise.all([b1, b2]);
  check(labels() === 'laterHuge', `once the boot's own are all in the first of them starts, on a page that never settles too (${labels()})`);
  await finish('laterHuge');
  check(labels() === 'laterSmall', `… and until the page settles they go one at a time: nothing of theirs may hold a worker the boot could still ask for (${labels()})`);
  await drain();
  check(got.length === 2 && pipeQueue.inflight === 0 && pipeQueue.inflightLow === 0 && pipelineProgress.total === 2, `both arrive, their slots are free and the count has not moved (${got.join(' ')}, ${pipeQueue.inflight} in flight, ${pipelineProgress.total} counted)`); }
{ let v: unknown = 'never answered'; try { v = await later('laterFails', 10, { throws: true }); } catch { v = 'thrown'; /* (the device as the browser made it throws here: PIPEQUEUE_BREAK) */ }
  check(v === null && pipeQueue.inflight === 0 && laterProgress.pending === 0, `a LATER creation that fails hands back NOTHING (its caller keeps the fallback it draws with), frees its slot and is no longer counted as out (${JSON.stringify(v)}, ${laterProgress.pending} out)`); }

// ---- after the boot: at most `after` in flight, at most `low` of them the background set's, the foreground first
pipeQueue.settle();
{ for (let i = 0; i < 7; i++) void ask(`f${i}`, 100 + i);
  await tick();
  check(labels() === 'f0 f1 f2 f3', `after the boot: seven asked, ${PIPE_QUEUE.after} in flight, in the order asked (${labels()})`);
  await drain(); }
{ for (let i = 0; i < 5; i++) void ask(`cut${i}`, 500, true);
  await tick();
  check(labels() === 'cut0 cut1', `the background set alone: ${PIPE_QUEUE.low} in flight, the other slots kept free (${labels()})`);
  void ask('fg0', 10); void ask('fg1', 10); void ask('fg2', 10); await tick();
  check(labels() === 'cut0 cut1 fg0 fg1', `a pass's own pipelines start at once beside them (${labels()})`);
  await finish('cut0');
  check(labels() === 'cut1 fg0 fg1 fg2', `a freed slot goes to the waiting foreground one before the next of the background (${labels()})`);
  await finish('fg0');
  check(labels() === 'cut1 fg1 fg2 cut2', `… and to the background when no foreground one waits (${labels()})`);
  await drain();
  check(pipeQueue.inflight === 0 && pipeQueue.inflightLow === 0 && pipeQueue.waiting.length === 0, 'everything asked for was made: nothing in flight, nothing waiting'); }
{ void dev.createRenderPipelineAsync({ label: 'render', vertex: { module: mod(10) }, fragment: { module: mod(10, true), targets: [] } } as unknown as GPURenderPipelineDescriptor);
  void ask('cutA', 10, true); void ask('cutB', 10, true); await tick();
  check(labels() === 'render cutA', `a render pipeline whose fragment is the background set's counts as background (${labels()})`);
  await drain(); }

{ for (let i = 0; i < 4; i++) void later(`late${i}`, 10);
  await tick();
  check(labels() === 'late0 late1', `after the boot a LATER pipeline is the background's: ${PIPE_QUEUE.low} in flight (${labels()})`);
  void ask('fgA', 10); await tick();
  check(labels() === 'late0 late1 fgA', `… and a pass's own pipeline starts beside them (${labels()})`);
  await drain();
  check(pipeQueue.inflight === 0 && pipeQueue.inflightLow === 0 && pipeQueue.waiting.length === 0, 'the LATER ones were all made: nothing in flight, nothing waiting'); }

console.log(red ? `${red} FAILED` : 'all passed');
process.exit(red ? 1 : 0);
