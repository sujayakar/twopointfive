// ASKING THE BROWSER FOR A GPU, AND A START THAT GETS NONE. The page used to ask once, for the fast card (`powerPreference: 'high-performance'`), and a
// browser that answered with nothing left one line on the red card: "No WebGPU adapter available." Nobody could tell from that line whether the browser has
// no WebGPU at all, has it and keeps the card back (a driver it does not trust, graphics acceleration turned off), or would have handed over a card had it
// been asked without a preference. Now:
//   · an empty answer to the fast-card ask is followed by a second ask with NO preference, and the game starts on the card that gives — never on the browser's
//     software adapter, which is written down and left (`?gpuask=0` = one ask, and the one line, as it was). Whether a machine exists whose first ask is empty and whose second is not is NOT KNOWN: nobody here has one;
//   · when both are empty the page finds out what it can before the card goes up — whether the browser's own software adapter answers (asked for the record
//     only, never used: the game cannot be played on it), which card and driver WebGL names, the browser and the system — and the card carries it, with a
//     button that copies it, so a player can send what the machine said.
// The facts are plain JSON (`NoGpuError.facts`): the backend's side can send them as they are where this browser sends.
// This file imports nothing and touches the page only inside `pageEnv`: the check drives the rest under bun (tools/qa/nogpu-check.ts).

/** `on`: a second ask and the gathered card (the page's `?gpuask=0` puts it off). `waitMs`: how long an ask made after the first may take before it is
 *  written down as unanswered (the first ask is waited for as long as it takes, as it always was) */
export const GPU_ASK = { on: true, waitMs: 4000, /** … and the software ask, made for the record only */ softMs: 2000 };
try { if (new URLSearchParams(location.search).get('gpuask') === '0') GPU_ASK.on = false; } catch { /* no page (the harness) */ }

export type AskHow = 'the fast card' | 'any card' | 'software';
const OPTS: Record<AskHow, GPURequestAdapterOptions | undefined> = { 'the fast card': { powerPreference: 'high-performance' }, 'any card': undefined, software: { forceFallbackAdapter: true } };
/** one ask and its answer: `got` = the adapter's name ('' = nothing), `fallback` = the browser calls it its software adapter, `error` = the ask threw or ran out of time */
export interface AskRow { how: AskHow; got: string; fallback?: boolean; error?: string }
export interface NoGpuFacts {
  /** the browser has `navigator.gpu` at all */
  webgpu: boolean;
  /** the page is on a secure address (WebGPU is given to no other) */
  secure: boolean;
  asks: AskRow[];
  /** what WebGL says of the machine: the card and driver by name where the browser tells; null = no WebGL either */
  webgl: { version: string; vendor: string; renderer: string; software: boolean } | null;
  agent: string;
  /** the system and the browser's full version, where the browser tells ('' elsewhere) */
  system: string;
  threads: number; memoryGb: number; screen: string;
}
export class NoGpuError extends Error {
  constructor(message: string, readonly facts: NoGpuFacts) { super(message); this.name = 'NoGpuError'; }
}

export interface GpuLike { requestAdapter(o?: GPURequestAdapterOptions): Promise<GPUAdapter | null> }
/** what the page can say of itself with no GPU (`pageEnv` on a page; the check hands in its own) */
export interface PageEnv { secure: boolean; agent: string; threads: number; memoryGb: number; screen: string; webgl(): NoGpuFacts['webgl']; system(): Promise<string> }

/** the adapter's name as the page has always written it (benchmarks and visit records are grouped by this string: its spacing stays) */
export const adapterName = (a: GPUAdapter): string => { const i = a.info; return `${i?.vendor ?? ''} ${i?.architecture ?? ''} ${i?.description ?? ''}`.trim(); };
/** the browser calls it its software adapter (said on the adapter's info, and on the adapter itself by older browsers) */
export const adapterFallback = (a: GPUAdapter): boolean => (a.info as { isFallbackAdapter?: boolean } | undefined)?.isFallbackAdapter === true || (a as { isFallbackAdapter?: boolean }).isFallbackAdapter === true;
const short = (e: unknown): string => String((e as Error | undefined)?.message ?? e).slice(0, 200) || 'an error with no message';
const within = <T>(p: Promise<T>, ms: number): Promise<T> => new Promise<T>((res, rej) => {
  const t = setTimeout(() => rej(new Error(`no answer in ${Math.round(ms / 1000)} s`)), ms);
  p.then(v => { clearTimeout(t); res(v); }, e => { clearTimeout(t); rej(e); });
});

/** never rejects: `threw` holds what the ask threw, as the browser threw it */
async function ask(gpu: GpuLike, how: AskHow, waitMs = 0): Promise<{ adapter: GPUAdapter | null; row: AskRow; threw?: unknown }> {
  try {
    const p = gpu.requestAdapter(OPTS[how]), adapter = (await (waitMs > 0 ? within(p, waitMs) : p)) ?? null;
    if (!adapter) return { adapter: null, row: { how, got: '' } };
    return { adapter, row: { how, got: adapterName(adapter) || 'an adapter with no name', ...(adapterFallback(adapter) ? { fallback: true } : {}) } };
  } catch (e) { return { adapter: null, row: { how, got: '', error: short(e) }, threw: e }; }
}

/** the page's own facts. Every read is guarded: a browser that will not say leaves its row empty */
export function pageEnv(): PageEnv {
  const nav = (() => { try { return navigator as Navigator & { deviceMemory?: number; userAgentData?: { getHighEntropyValues?(h: string[]): Promise<Record<string, unknown>> } }; } catch { return null; } })();
  return {
    secure: (() => { try { return isSecureContext === true; } catch { return false; } })(),
    agent: (() => { try { return String(nav?.userAgent ?? ''); } catch { return ''; } })(),
    threads: (() => { try { return Number(nav?.hardwareConcurrency) || 0; } catch { return 0; } })(),
    memoryGb: (() => { try { return Number(nav?.deviceMemory) || 0; } catch { return 0; } })(),
    screen: (() => { try { return `${screen.width}x${screen.height} at ${Math.round((devicePixelRatio || 1) * 100) / 100}`; } catch { return ''; } })(),
    webgl: () => {
      try {
        const c = document.createElement('canvas'), gl = (c.getContext('webgl2') ?? c.getContext('webgl')) as WebGLRenderingContext | null; if (!gl) return null;
        const ext = gl.getExtension('WEBGL_debug_renderer_info');
        const vendor = String((ext && gl.getParameter(ext.UNMASKED_VENDOR_WEBGL)) || gl.getParameter(gl.VENDOR) || ''), renderer = String((ext && gl.getParameter(ext.UNMASKED_RENDERER_WEBGL)) || gl.getParameter(gl.RENDERER) || '');
        const version = String(gl.getParameter(gl.VERSION) || '');
        try { gl.getExtension('WEBGL_lose_context')?.loseContext(); } catch { /* the context goes with the canvas */ }
        return { version: version.slice(0, 80), vendor: vendor.slice(0, 120), renderer: renderer.slice(0, 240), software: softwareName(renderer) };
      } catch { return null; }
    },
    system: async () => {
      try {
        const get = nav?.userAgentData?.getHighEntropyValues; if (!get) return '';
        const v = await within(get.call(nav!.userAgentData, ['platform', 'platformVersion', 'architecture', 'bitness', 'fullVersionList']), 1500);
        const brands = Array.isArray(v.fullVersionList) ? (v.fullVersionList as { brand?: string; version?: string }[]).filter(b => b?.brand && !/not.?a.?brand/i.test(b.brand)).map(b => `${b.brand} ${b.version ?? ''}`.trim()).join(', ') : '';
        return [[v.platform, v.platformVersion, v.architecture, v.bitness].filter(Boolean).join(' '), brands].filter(Boolean).join(' · ').slice(0, 300);
      } catch { return ''; }
    },
  };
}
/** a WebGL renderer the browser draws itself, with no card under it */
export const softwareName = (renderer: string): boolean => /swiftshader|llvmpipe|softpipe|software|basic render/i.test(renderer);

/** THE ASK. An adapter, or a `NoGpuError` that holds what the machine said. `said` is told when the second ask is what answered (the page's console) */
export async function pickAdapter(gpu: GpuLike | undefined | null, env: () => PageEnv = pageEnv, on = GPU_ASK.on, said: (line: string) => void = () => {}): Promise<{ adapter: GPUAdapter; asks: AskRow[] }> {
  // (`last`: one more ask to make for the record while the system is being asked for its name, so the card waits for the slower of the two and no longer)
  const fail = async (message: string, asks: AskRow[], last?: Promise<{ row: AskRow }>): Promise<never> => {
    if (!on) throw new Error(message);
    const e = env(), [system, more] = await Promise.all([within(e.system(), GPU_ASK.softMs).catch(() => ''), last]);
    throw new NoGpuError(message, { webgpu: !!gpu, secure: e.secure, asks: more ? [...asks, more.row] : asks, webgl: (() => { try { return e.webgl(); } catch { return null; } })(), agent: e.agent.slice(0, 400), system, threads: e.threads, memoryGb: e.memoryGb, screen: e.screen });
  };
  if (!gpu) return fail('WebGPU not available (navigator.gpu missing). Use a current Chrome/Edge/Safari.', []);
  const first = await ask(gpu, 'the fast card');
  if (first.adapter) return { adapter: first.adapter, asks: [first.row] };
  // (as it was, an ask that THREW reached the card as the browser threw it)
  if (!on) throw first.threw !== undefined ? first.threw : new Error('No WebGPU adapter available.');
  const any = await ask(gpu, 'any card', GPU_ASK.waitMs);
  // (the browser's software adapter is not a card: the game cannot be played on it, and a player is better served by the card that says why. A browser whose
  // FIRST answer is that adapter starts on it as before: a test box does)
  if (any.adapter && any.row.fallback) return fail('No WebGPU adapter available.', [first.row, any.row]);
  if (any.adapter) { said(`WebGPU: the ask for the fast card came back empty; the game runs on what an ask with no preference gave (${any.row.got})`); return { adapter: any.adapter, asks: [first.row, any.row] }; }
  return fail('No WebGPU adapter available.', [first.row, any.row], ask(gpu, 'software', GPU_ASK.softMs));
}

/** THE CARD'S WORDS for a start with no GPU: `say` to the player, `details` = what the machine said, line by line (the button copies `say`'s first line and these) */
export function noGpuWords(message: string, f: NoGpuFacts): { say: string; details: string } {
  const why = !f.webgpu ? (f.secure ? "Your browser doesn't have WebGPU, and the game needs it. Try the newest Chrome or Edge."
      : "This page isn't on a secure address, so your browser won't give it WebGPU.")
    : f.webgl?.software || !f.webgl ? "Your browser isn't using your graphics card. Turn on graphics acceleration in your browser's settings and restart the browser."
    : "Your browser wouldn't give the game your graphics card. Close every window of your browser and open it again, and update your graphics driver if that doesn't help.";
  const say = `The game couldn't start.\n${why}\nIf it still won't start, press Copy and send that to the developer.`;
  const rows = [message,
    ...f.asks.map(a => `asked for ${a.how}: ${a.error !== undefined ? `failed (${a.error})` : a.got ? `${a.got}${a.fallback ? ' (the browser\'s software adapter)' : ''}` : 'nothing'}`),
    f.webgl ? `WebGL: ${[f.webgl.version, f.webgl.vendor, f.webgl.renderer].filter(Boolean).join(' · ')}${f.webgl.software ? ' (drawn in software)' : ''}` : 'WebGL: none',
    f.agent ? `browser: ${f.agent}` : '', f.system ? `system: ${f.system}` : '',
    `page: ${[f.secure ? 'secure address' : 'NOT a secure address', f.threads ? `${f.threads} threads` : '', f.memoryGb ? `${f.memoryGb} GB as the browser rounds it` : '', f.screen].filter(Boolean).join(' · ')}`];
  return { say, details: rows.filter(Boolean).join('\n') };
}
