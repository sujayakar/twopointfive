// SHADERS · ROWS ON TRIAL (Options > Graphics, the last section; 2026-10-26). Chrome on Windows compiles every shader through DXC, which pastes each call into its caller: the
// big kernels take 5–25 s apiece there (gpu/device.ts THE COMPILE QUEUE has the measure), where Metal takes a fraction of that. Each row here is one way of spending less of
// that time, OFF until somebody with such a machine has tried it, and each is read ONCE, when this module is imported: the pipelines are made before any menu can open, so a
// row moved in Options is the NEXT load's. A switch of the page, not a rule of the game (no knob, no FROZEN_HOLDS row: every stand boots with all of them off).
// TWO ROWS START ON IN EVERY BROWSER (2026-10-27, `on: true`: Compact shaders and Separate shadow rays; the maintainer's pick, asked whether in a Windows
// browser only, everywhere, or not yet). What it rests on: a Windows machine loaded the two together fourteen times with no error, and a Mac drew each alone at
// the frame cost of the plain build. What nobody has: a plain load from a Windows machine to set the wait against, the two together timed on a Mac, any AMD or
// Intel card. On every stand of the harness (it is no browser) they start off as before. `?compactshaders=0&splitrays=0` is a load as it was; a row turned off
// in Options is kept off.
//   compact        ?compactshaders=1   render/kernels.ts: the ten kernels written a second way (one call site per heavy function), compiled instead of today's
//   cutWhenNeeded  ?cutwhenneeded=1    render/cutSwap.ts: the wall-cut twins are built the first time a wall is cut, not behind the start
//   firstFrame     ?firstframe=1       gpu/device.ts PIPE_QUEUE.later: the start waits only for the pipelines the first frame draws
//   splitRays      ?splitrays=1        render/gatherSplit.ts: the final gather as two smaller programs (its rays, then the light at what they hit) instead of one
//   whenNeeded     ?shaderswhenneeded=1  gpu/device.ts PIPE_QUEUE.held: the pipelines the first frame does not draw are built the first time a frame asks for one, not behind the start
// KEPT under its own key on this machine (only the rows that are not where they start); a URL word wins for that visit and is never written (as `?quality=` does). A row is LISTED only in a build that carries the code
// which reads it: that code says so with useShaderRow() when it is imported, so a build that has one part of the trial does not show a row that would do nothing.
// THE HARNESS (no browser): every row off, unless the environment gives a row's word as a held rule's is given, the URL word in capitals: TPS_COMPACTSHADERS=1,
// TPS_CUTWHENNEEDED=1, TPS_FIRSTFRAME=1, TPS_SPLITRAYS=1, TPS_SHADERSWHENNEEDED=1 in front of a headless tool.
// THIS MODULE IMPORTS NOTHING: gpu/ and render/ read it while they are themselves being imported, so it must always be whole before them.
export type ShaderRowKey = 'compact' | 'cutWhenNeeded' | 'firstFrame' | 'splitRays' | 'whenNeeded';
/** `late`: with the row on, some shaders are built after the start, and the picture is drawn another way until they are in. `on`: the row starts ON in a
 *  browser when nothing is kept and no word is given (never in the harness) */
interface ShaderRow { key: ShaderRowKey; url: string; label: string; describe: string; late?: boolean; on?: true }
const NEXT = 'This starts the next time you load the page.';
/** added to a row the address names: the address wins on every load of it, whatever the row is moved to */
const ADDRESS = 'This page\'s address is setting this for now. Take it out of the address to use what you choose here.';
const ROWS: ShaderRow[] = [
  { key: 'compact', url: 'compactshaders', label: 'Compact shaders', on: true,
    describe: `The renderer's biggest shaders are written so that each heavy step appears in them once, which makes them faster to build on Windows. The picture should look the same. Turn this off to use the older shaders. ${NEXT}` },
  { key: 'cutWhenNeeded', url: 'cutwhenneeded', label: 'Wall cut shaders when needed', late: true,
    describe: `The game builds the shaders for broken walls the first time something breaks, so you don't wait for them every time it starts. This is a trial for slow shader compiling on Windows. ${NEXT}` },
  { key: 'firstFrame', url: 'firstframe', label: 'Start on first frame shaders', late: true,
    describe: `The start waits only for the shaders the first picture needs and builds the rest behind it while you play. This is a trial for slow shader compiling on Windows. ${NEXT}` },
  { key: 'whenNeeded', url: 'shaderswhenneeded', label: 'Other shaders when needed', late: true,
    describe: `The start waits only for the shaders the first picture needs, like the row above, but the game doesn't build the rest behind it. It builds each one the first time you need it, like when the quality changes, and it draws the picture the old way until that shader's ready. This is a trial for slow shader compiling on Windows. ${NEXT}` },
  { key: 'splitRays', url: 'splitrays', label: 'Separate shadow rays', on: true,
    describe: `The bounce light is worked out by two smaller shaders instead of one big one, which makes them faster to build on Windows. The picture should look the same, except on Ultra, where you'll lose its extra bounce light pass while this is on. ${NEXT}` },
];
const LS = 'twopointsix.shaderrows';
type Kept = Partial<Record<ShaderRowKey, boolean>>;
/** this is a browser (the harness's own name is not a browser's: no row starts on there, whatever machine it runs on) */
const BROWSER = (() => { try { return /\bMozilla\//.test(String(globalThis.navigator?.userAgent ?? '')); } catch { return false; } })();
/** where a row stands with nothing kept and no word given */
const starts = (r: ShaderRow): boolean => r.on === true && BROWSER;
function kept(): Kept { try { const o = JSON.parse(globalThis.localStorage?.getItem(LS) ?? '{}'); return o && typeof o === 'object' ? o as Kept : {}; } catch { return {}; } }
/** (only the rows that are NOT where they start are written, and no line at all when none is: a row left at its start holds nothing, so a later change of
 *  where that row starts reaches this browser) */
function keep(k: Kept) {
  const out: Kept = {}; for (const r of ROWS) if (typeof k[r.key] === 'boolean' && k[r.key] !== starts(r)) out[r.key] = k[r.key];
  try { if (Object.keys(out).length) globalThis.localStorage?.setItem(LS, JSON.stringify(out)); else globalThis.localStorage?.removeItem(LS); } catch { /* no store */ }
}
/** a row's kept choice, or where it starts */
const chosen = (k: Kept, r: ShaderRow): boolean => typeof k[r.key] === 'boolean' ? k[r.key] === true : starts(r);
/** this visit's URL word for a row: true / false, or null = the address does not say (no page — the harness — says nothing) */
function urlWord(r: ShaderRow): boolean | null { try { const v = new URLSearchParams(globalThis.location?.search ?? '').get(r.url); return v === null ? null : v !== '0'; } catch { return null; } }

/** the harness's word for a row: TPS_<the URL word in capitals> in the environment, null = none given (a page has no environment) */
function envWord(r: ShaderRow): boolean | null { const v = (globalThis as { process?: { env?: Record<string, string | undefined> } }).process?.env?.[`TPS_${r.url.toUpperCase()}`]; return v === undefined ? null : v !== '0'; }

/** what THIS load was started with: the URL's word, else the environment's, else the kept choice, else where the row starts. Read by the code each row switches (never written after the import) */
export const SHADER_ROWS: Record<ShaderRowKey, boolean> = { compact: false, cutWhenNeeded: false, firstFrame: false, splitRays: false, whenNeeded: false };
{ const k = kept(); for (const r of ROWS) SHADER_ROWS[r.key] = urlWord(r) ?? envWord(r) ?? chosen(k, r); }

const used = new Set<ShaderRowKey>();
/** the code a row switches says it is in this build (at its own import), and gets the row's word for this load */
export function useShaderRow(key: ShaderRowKey): boolean { used.add(key); return SHADER_ROWS[key]; }
/** every row's URL word: what a run that goes from page to page must take along for its pages to be built the way the first was asked (ui/benchrun.ts) */
export const SHADER_ROW_WORDS: readonly string[] = ROWS.map(r => r.url);
/** the rows THIS load was started with, by their URL words, of the ones this build carries: what a report says its numbers were taken under (ui/bench.ts) */
export function shaderRowsOn(): string[] { return ROWS.filter(r => used.has(r.key) && SHADER_ROWS[r.key]).map(r => r.url); }
/** the name Options shows for a row's URL word (the word itself where this build has no such row: a report from another build still reads) */
export function shaderRowLabel(word: string): string { return ROWS.find(r => r.url === word)?.label ?? word; }
/** the row of that URL word builds some shaders after the start (a timing taken under it may be of the picture drawn meanwhile: ui/bench.ts says so) */
export function shaderRowLate(word: string): boolean { return ROWS.find(r => r.url === word)?.late === true; }

/** rows flipped in this visit (a flipped row shows the kept choice from then on; until then one the address names shows the address's word) */
const flipped = new Set<ShaderRowKey>();
/** Options > Graphics: the section and a row for each part of the trial this build carries (ui/options.ts hands the screen's own builders; both of its Graphics
 *  branches call this, the one for a page whose renderer is still compiling too — that wait is what the rows are for). Returns Reset to defaults' part: every row
 *  back where it starts. */
export function shaderRows(K: { sec(parent: HTMLElement, text: string): unknown; flip(parent: HTMLElement, label: string, get: () => boolean, set: (v: boolean) => void, describe: string): void }, body: HTMLElement): () => void {
  const rows = ROWS.filter(r => used.has(r.key));
  if (rows.length) K.sec(body, 'Shaders');
  for (const r of rows) K.flip(body, r.label, () => (flipped.has(r.key) ? null : urlWord(r)) ?? chosen(kept(), r), v => { const k = kept(); k[r.key] = v; keep(k); flipped.add(r.key); }, urlWord(r) === null ? r.describe : `${r.describe} ${ADDRESS}`);
  return () => { keep({}); for (const r of ROWS) flipped.add(r.key); };
}
