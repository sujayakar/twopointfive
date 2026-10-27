// A START WITH NO GPU (src/gpu/adapter.ts GPU_ASK): the ask for the fast card, the second ask with no preference when the first is empty, and what the page
// gathers for the card when both are. The browser is a stand-in that answers each kind of ask as the case says: no browser and no GPU are needed, and none
// of this has been seen on a machine whose browser keeps its card back.
//   bun run tools/qa/nogpu-check.ts              exit 0 = every check passed, 1 = one failed
//   NOGPU_BREAK=1 bun run tools/qa/nogpu-check.ts     the page asks as if the switch were off (?gpuask=0): must FAIL
import { FATAL, gpuStory, showFatal, showFatalError, type FatalReport } from '../../src/gpu/device';
import { GPU_ASK, NoGpuError, adapterName, noGpuWords, pickAdapter, softwareName, type GpuLike, type NoGpuFacts, type PageEnv } from '../../src/gpu/adapter';

const on = process.env.NOGPU_BREAK === '1' ? false : true;
let red = 0; const check = (ok: boolean, what: string) => { console.log(`${ok ? '- PASS' : '- FAIL'} — ${what}`); if (!ok) red++; };
const card = (vendor: string, description: string, fallback = false, old = false) => ({ info: { vendor, architecture: 'x9', description, ...(old ? {} : { isFallbackAdapter: fallback }) }, ...(old ? { isFallbackAdapter: fallback } : {}) }) as unknown as GPUAdapter;
type Answer = GPUAdapter | null | 'throw' | 'never';
/** a browser that answers the fast-card ask, the plain ask and the software ask as told, and keeps the asks it was given */
const browser = (fast: Answer, any: Answer, soft: Answer) => {
  const asked: string[] = [];
  const gpu: GpuLike = { requestAdapter: o => {
    const how = o?.forceFallbackAdapter ? 'soft' : o?.powerPreference ? 'fast' : 'any'; asked.push(how);
    const a = how === 'fast' ? fast : how === 'any' ? any : soft;
    return a === 'throw' ? Promise.reject(new Error('the GPU process is gone')) : a === 'never' ? new Promise(() => {}) : Promise.resolve(a);
  } };
  return { gpu, asked };
};
const env = (over: Partial<PageEnv> = {}) => (): PageEnv => ({ secure: true, agent: 'SomeBrowser/9 (a test)', threads: 4, memoryGb: 4, screen: '1920x1080 at 1',
  webgl: () => ({ version: 'WebGL 2.0', vendor: 'Acme', renderer: 'ANGLE (Acme, Acme Fast Card 2 Direct3D11)', software: false }), system: async () => 'Windows 5.0.0 x86 64 · SomeBrowser 9.0.1', ...over });
const thrown = async (p: Promise<unknown>): Promise<unknown> => { try { await p; return null; } catch (e) { return e; } };
const waitWas = GPU_ASK.waitMs, softWas = GPU_ASK.softMs; GPU_ASK.waitMs = 60; GPU_ASK.softMs = 60;

check(GPU_ASK.on === true, 'the switch ships on');
{ const b = browser(card('acme', 'Fast Card 2'), null, null), r = await pickAdapter(b.gpu, env(), on);
  check(adapterName(r.adapter) === 'acme x9 Fast Card 2' && b.asked.join() === 'fast' && r.asks.length === 1, 'a browser that gives the fast card is asked once, as ever'); }
{ const b = browser(null, card('acme', 'Slow Card'), null), said: string[] = [], e = await thrown(pickAdapter(b.gpu, env(), on, l => said.push(l)).then(r => {
    check(adapterName(r.adapter) === 'acme x9 Slow Card' && b.asked.join() === 'fast,any', 'an empty answer to the fast-card ask is followed by an ask with no preference, and its card is used');
    check(r.asks.length === 2 && r.asks[0].got === '' && r.asks[1].how === 'any card' && said.length === 1 && said[0].includes('Slow Card'), '… both asks are on the record, and the page is told which ask answered');
  }));
  check(e === null, '… and nothing is thrown'); }
{ const b = browser('throw', card('acme', 'Slow Card'), null), e = await thrown(pickAdapter(b.gpu, env(), on));
  check(e === null && b.asked.join() === 'fast,any', 'a fast-card ask that THROWS is followed by the second ask too'); }
{ const b = browser(null, null, card('google', 'SwiftShader Device', true)), e = await thrown(pickAdapter(b.gpu, env(), on));
  check(e instanceof NoGpuError && e.message === 'No WebGPU adapter available.', 'no card from either ask: the start fails with the old line');
  check(b.asked.join() === 'fast,any,soft', '… after the software adapter was asked for, for the record');
  const f = (e as NoGpuError).facts as NoGpuFacts | undefined;
  check(!!f && f.webgpu && f.secure && f.asks.length === 3 && f.asks[2].got === 'google x9 SwiftShader Device' && f.asks[2].fallback === true, '… the facts hold the three asks, the software adapter by name');
  check(!!f && f.webgl?.renderer.includes('Acme Fast Card 2') === true && f.agent.includes('SomeBrowser') && f.system.includes('Windows') && f.threads === 4, '… the card WebGL names, the browser and the system');
  check(!!f && JSON.stringify(JSON.parse(JSON.stringify(f))) === JSON.stringify(f) && JSON.stringify(f).length < 4000, '… as plain JSON, small');
  if (f) { const w = noGpuWords((e as Error).message, f);
    check(w.say.startsWith("The game couldn't start.") && w.say.includes("wouldn't give the game your graphics card") && w.say.includes('press Copy'), "the card says the browser kept the card back, and what to do");
    check(['No WebGPU adapter available.', 'asked for the fast card: nothing', 'asked for any card: nothing', "asked for software: google x9 SwiftShader Device (the browser's software adapter)", 'WebGL: WebGL 2.0 · Acme · ANGLE (Acme, Acme Fast Card 2 Direct3D11)', 'browser: SomeBrowser/9', 'system: Windows', 'secure address · 4 threads'].every(l => w.details.includes(l)), '… and under it, line by line, what the machine said');
    check(!/[1-9]\d* GB or more|undefined|null|\[object/.test(w.details), '… with no stray word in it'); } }
{ const b = browser(null, 'never', 'throw'), t0 = Date.now(), e = await thrown(pickAdapter(b.gpu, env(), on));
  const f = e instanceof NoGpuError ? e.facts : null;
  check(!!f && /no answer in/.test(f.asks[1].error ?? '') && f.asks[2].error === 'the GPU process is gone' && Date.now() - t0 < 2000, 'a second ask the browser never answers is given up on, and the card still goes up with the failed asks named');
  if (f) check(/asked for any card: failed \(no answer in/.test(noGpuWords('x', f).details) && noGpuWords('x', f).details.includes('asked for software: failed (the GPU process is gone)'), '… in the lines too'); }
{ const e = await thrown(pickAdapter(browser(null, null, null).gpu, env({ webgl: () => ({ version: 'WebGL 2.0', vendor: 'Google Inc. (Google)', renderer: 'ANGLE (Google, Vulkan 1.3.0 (SwiftShader Device), SwiftShader driver)', software: true }) }), on));
  const w = e instanceof NoGpuError ? noGpuWords(e.message, e.facts) : null;
  check(!!w && w.say.includes("isn't using your graphics card") && w.say.includes('graphics acceleration') && w.details.includes('(drawn in software)'), 'a browser that draws WebGL in software is told to turn graphics acceleration on'); }
{ const e = await thrown(pickAdapter(null, env(), on));
  check(e instanceof NoGpuError && /navigator\.gpu missing/.test(e.message) && e.facts.webgpu === false && e.facts.asks.length === 0, 'a browser with no WebGPU at all: the old line, with the facts');
  if (e instanceof NoGpuError) check(noGpuWords(e.message, e.facts).say.includes("doesn't have WebGPU"), '… and the card says the browser has none');
  const e2 = await thrown(pickAdapter(null, env({ secure: false }), on));
  if (e2 instanceof NoGpuError) check(noGpuWords(e2.message, e2.facts).say.includes("isn't on a secure address") && noGpuWords(e2.message, e2.facts).details.includes('NOT a secure address'), '… or that the page is not on a secure address, where that is why'); else check(false, '… or that the page is not on a secure address'); }
{ const e = await thrown(pickAdapter(browser(null, null, null).gpu, env({ webgl: () => { throw new Error('no canvas'); } }), on));
  check(e instanceof NoGpuError && e.facts.webgl === null && noGpuWords(e.message, e.facts).details.includes('WebGL: none'), 'a page that cannot even be asked about WebGL still gets its card'); }
{ const b = browser(null, card('google', 'SwiftShader Device', true), null), e = await thrown(pickAdapter(b.gpu, env(), on));
  check(e instanceof NoGpuError && b.asked.join() === 'fast,any' && e.facts.asks[1].fallback === true, "a second ask answered by the browser's software adapter is not a start: the card, with that adapter on the record");
  const b1 = browser(card('google', 'SwiftShader Device', true), null, null), r = await thrown(pickAdapter(b1.gpu, env(), on));
  check(r === null && b1.asked.join() === 'fast', '… while a browser whose FIRST answer is that adapter starts on it, as before (a test box)'); }
// (the switch off: one ask, a plain error, nothing gathered — the build as it was)
{ const b = browser(null, card('acme', 'Slow Card'), null), e = await thrown(pickAdapter(b.gpu, env(), false));
  check(e instanceof Error && !(e instanceof NoGpuError) && e.message === 'No WebGPU adapter available.' && b.asked.join() === 'fast', 'switch off (?gpuask=0): one ask and the one line, as it was');
  const e2 = await thrown(pickAdapter(null, env(), false));
  check(e2 instanceof Error && !(e2 instanceof NoGpuError) && /navigator\.gpu missing/.test(e2.message), '… and the old line for a browser with no WebGPU');
  const e3 = await thrown(pickAdapter(browser('throw', null, null).gpu, env(), false));
  check(e3 instanceof Error && e3.message === 'the GPU process is gone' && !(e3 instanceof NoGpuError), '… and an ask that throws reaches the card as the browser threw it'); }
{ const b = browser(null, card('google', 'SwiftShader Device', true, true), null), e = await thrown(pickAdapter(b.gpu, env(), on));
  check(e instanceof NoGpuError && e.facts.asks[1].fallback === true, 'a browser that says "software" on the adapter itself (an older one) is read the same'); }
{ const never: GpuLike = { requestAdapter: o => o?.powerPreference ? Promise.resolve(null) : new Promise(() => {}) }, t0 = Date.now(), e = await thrown(pickAdapter(never, env({ system: () => new Promise(() => {}) as Promise<string> }), on));
  check(e instanceof NoGpuError && Date.now() - t0 < 1000 && e.facts.system === '' && e.facts.asks.length === 3 && e.facts.asks.slice(1).every(a => /no answer in/.test(a.error ?? '')), 'asks nobody answers, and a system that never says its name: the card still goes up, and is not kept waiting for each in turn'); }
{ const e = await thrown(pickAdapter({ requestAdapter: () => Promise.reject(new Error('')) }, env(), on));
  check(e instanceof NoGpuError && /asked for the fast card: failed \(an error with no message\)/.test(noGpuWords(e.message, e.facts).details), 'an ask that threw with no words is still written down as failed'); }
check(softwareName('ANGLE (Google, Vulkan 1.3.0 (SwiftShader Device), SwiftShader driver)') && softwareName('Microsoft Basic Render Driver') && softwareName('llvmpipe (LLVM 15)') && !softwareName('ANGLE (AMD, AMD Radeon RX 6800 XT Direct3D11 vs_5_0 ps_5_0, D3D11)') && !softwareName('Apple M4 Max'), 'software renderers are told from cards by name');
// THE CARD (gpu/device.ts showFatal), on a stand-in page: the three parts, the button, and whoever listens for red cards
{ type El = { style: Record<string, string>; textContent: string; kids: El[]; append(...k: El[]): void; appendChild(k: El): void; remove(): void; select(): void; onclick: (() => void) | null; type: string; value: string; readOnly: boolean };
  const mk = (): El => ({ style: {}, textContent: '', kids: [], append(...k) { this.kids.push(...k); }, appendChild(k) { this.kids.push(k); }, remove() {}, select() {}, onclick: null, type: '', value: '', readOnly: false });
  const fatal = mk(); let copied = 0, copyOk = true;
  const g = globalThis as Record<string, unknown>, docWas = g.document;
  g.document = { getElementById: (id: string) => id === 'fatal' ? fatal : null, createElement: () => mk(), execCommand: () => { copied++; return copyOk; }, createRange: () => ({ selectNodeContents() {} }) };
  const told: FatalReport[] = []; FATAL.tell = r => { told.push(r); };
  const facts = (await thrown(pickAdapter(browser(null, null, null).gpu, env(), true)) as NoGpuError);
  gpuStory.stage = 'while setting up'; showFatalError(facts);
  const [say, pre, b] = fatal.kids;
  check(fatal.style.display === 'flex' && fatal.kids.length === 3 && say.textContent.startsWith("The game couldn't start.") && pre.textContent.includes('asked for any card: nothing') && b.textContent === 'Copy', 'the card for a start with no GPU has its words, the lines and a Copy button');
  check(told.length === 1 && told[0].message === 'No WebGPU adapter available.' && told[0].stage === 'adapter' && told[0].where === 'while setting up' && (told[0].data as { asks?: unknown[] })?.asks?.length === 3 && /NoGpuError|No WebGPU/.test(told[0].stack ?? ''), '… and the listener is told the plain error, the stage and the facts, not the sentence for the player');
  b.onclick?.(); check(copied === 1 && b.textContent === 'Copied', 'a press of Copy copies inside the click');
  copyOk = false; b.onclick?.(); check(/wouldn't copy it/.test(b.textContent), '… and a browser that refuses is said so, with the lines left selected');
  fatal.kids.length = 0; gpuStory.stage = ''; showFatalError(new Error('boom'));
  check(/boom/.test(fatal.textContent) && fatal.kids.length === 0 && told.length === 2 && told[1].message === 'boom' && told[1].stage === undefined && told[1].where === undefined, 'any other throw: its stack on the card as before, and the listener told its message');
  showFatal('GPU device lost: x\nmore', { report: { message: 'GPU device lost: x', stage: 'device' } });
  check(told.length === 3 && told[2].stage === 'device' && told[2].stack === 'GPU device lost: x\nmore', 'a lost device is told as one');
  FATAL.tell = () => { throw new Error('the listener fell over'); };
  let threw = false; try { showFatal('still up'); } catch { threw = true; }
  check(!threw && fatal.textContent === 'still up', 'a listener that throws does not take the card down');
  FATAL.tell = null; showFatal('nobody listens'); check(fatal.textContent === 'nobody listens' && told.length === 3, 'with no listener the card is the card, as it was');
  g.document = docWas; }
GPU_ASK.waitMs = waitWas; GPU_ASK.softMs = softWas;
console.log(red ? `${red} FAILED` : 'all passed');
process.exit(red ? 1 : 0);
