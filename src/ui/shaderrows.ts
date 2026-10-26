// SHADERS · ROWS ON TRIAL (Options > Graphics, the last section; 2026-10-26). Chrome on Windows compiles every shader through DXC, which pastes each call into its caller: the
// big kernels take 5–25 s apiece there (gpu/device.ts THE COMPILE QUEUE has the measure), where Metal takes a fraction of that. Each row here is one way of spending less of
// that time, OFF until somebody with such a machine has tried it, and each is read ONCE, when this module is imported: the pipelines are made before any menu can open, so a
// row moved in Options is the NEXT load's. A switch of the page, not a rule of the game (no knob, no FROZEN_HOLDS row: every stand boots with all of them off).
//   compact        ?compactshaders=1   render/kernels.ts: the ten kernels written a second way (one call site per heavy function), compiled instead of today's
//   cutWhenNeeded  ?cutwhenneeded=1    render/cutSwap.ts: the wall-cut twins are built the first time a wall is cut, not behind the start
//   firstFrame     ?firstframe=1       gpu/device.ts PIPE_QUEUE.later: the start waits only for the pipelines the first frame draws
// KEPT under its own key on this machine; a URL word wins for that visit and is never written (as `?quality=` does). A row is LISTED only in a build that carries the code
// which reads it: that code says so with useShaderRow() when it is imported, so a build that has one part of the trial does not show a row that would do nothing.
// THE HARNESS (no page): every row off, unless the environment gives a row's word as a held rule's is given, the URL word in capitals: TPS_COMPACTSHADERS=1,
// TPS_CUTWHENNEEDED=1, TPS_FIRSTFRAME=1 in front of a headless tool.
// THIS MODULE IMPORTS NOTHING: gpu/ and render/ read it while they are themselves being imported, so it must always be whole before them.
export type ShaderRowKey = 'compact' | 'cutWhenNeeded' | 'firstFrame';
interface ShaderRow { key: ShaderRowKey; url: string; label: string; describe: string }
const NEXT = 'This starts the next time you load the page.';
/** added to a row the address names: the address wins on every load of it, whatever the row is moved to */
const ADDRESS = 'This page\'s address is setting this for now. Take it out of the address to use what you choose here.';
const ROWS: ShaderRow[] = [
  { key: 'compact', url: 'compactshaders', label: 'Compact shaders',
    describe: `The renderer's biggest shaders are written a second way, so that each heavy step appears in them once. This is a trial for slow shader compiling on Windows, and the picture should look the same. ${NEXT}` },
  { key: 'cutWhenNeeded', url: 'cutwhenneeded', label: 'Wall cut shaders when needed',
    describe: `The game builds the shaders for broken walls the first time something breaks, so you don't wait for them every time it starts. This is a trial for slow shader compiling on Windows. ${NEXT}` },
  { key: 'firstFrame', url: 'firstframe', label: 'Start on first frame shaders',
    describe: `The start waits only for the shaders the first picture needs and builds the rest behind it while you play. This is a trial for slow shader compiling on Windows. ${NEXT}` },
];
const LS = 'twopointsix.shaderrows';
type Kept = Partial<Record<ShaderRowKey, boolean>>;
function kept(): Kept { try { const o = JSON.parse(globalThis.localStorage?.getItem(LS) ?? '{}'); return o && typeof o === 'object' ? o as Kept : {}; } catch { return {}; } }
function keep(k: Kept) { try { if (ROWS.some(r => k[r.key])) globalThis.localStorage?.setItem(LS, JSON.stringify(k)); else globalThis.localStorage?.removeItem(LS); } catch { /* no store */ } }
/** this visit's URL word for a row: true / false, or null = the address does not say (no page — the harness — says nothing) */
function urlWord(r: ShaderRow): boolean | null { try { const v = new URLSearchParams(globalThis.location?.search ?? '').get(r.url); return v === null ? null : v !== '0'; } catch { return null; } }

/** the harness's word for a row: TPS_<the URL word in capitals> in the environment, null = none given (a page has no environment) */
function envWord(r: ShaderRow): boolean | null { const v = (globalThis as { process?: { env?: Record<string, string | undefined> } }).process?.env?.[`TPS_${r.url.toUpperCase()}`]; return v === undefined ? null : v !== '0'; }

/** what THIS load was started with: the URL's word, else the environment's, else the kept choice, else off. Read by the code each row switches (never written after the import) */
export const SHADER_ROWS: Record<ShaderRowKey, boolean> = { compact: false, cutWhenNeeded: false, firstFrame: false };
{ const k = kept(); for (const r of ROWS) SHADER_ROWS[r.key] = urlWord(r) ?? envWord(r) ?? k[r.key] === true; }

const used = new Set<ShaderRowKey>();
/** the code a row switches says it is in this build (at its own import), and gets the row's word for this load */
export function useShaderRow(key: ShaderRowKey): boolean { used.add(key); return SHADER_ROWS[key]; }

/** rows flipped in this visit (a flipped row shows the kept choice from then on; until then one the address names shows the address's word) */
const flipped = new Set<ShaderRowKey>();
/** Options > Graphics: the section and a row for each part of the trial this build carries (ui/options.ts hands the screen's own builders; both of its Graphics
 *  branches call this, the one for a page whose renderer is still compiling too — that wait is what the rows are for). Returns Reset to defaults' part: every row off. */
export function shaderRows(K: { sec(parent: HTMLElement, text: string): unknown; flip(parent: HTMLElement, label: string, get: () => boolean, set: (v: boolean) => void, describe: string): void }, body: HTMLElement): () => void {
  const rows = ROWS.filter(r => used.has(r.key));
  if (rows.length) K.sec(body, 'Shaders');
  for (const r of rows) K.flip(body, r.label, () => (flipped.has(r.key) ? null : urlWord(r)) ?? kept()[r.key] === true, v => { const k = kept(); k[r.key] = v; keep(k); flipped.add(r.key); }, urlWord(r) === null ? r.describe : `${r.describe} ${ADDRESS}`);
  return () => { keep({}); for (const r of ROWS) flipped.add(r.key); };
}
