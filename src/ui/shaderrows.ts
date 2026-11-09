// HOW THE LOAD SPENDS ITS SHADER TIME (2026-10-26). Chrome on Windows compiles every shader through DXC, which pastes each call into its caller: the big kernels take
// 5–25 s apiece there (gpu/device.ts THE COMPILE QUEUE has the measure), where Metal takes a fraction of that. A word here is read ONCE, when this module is imported: the
// pipelines are made before any menu can open. A switch of the page, not a rule of the game.
// THE ROWS ARE GONE FROM OPTIONS (2026-11-08). Compact shaders and Separate shadow rays started on in every browser from 2026-10-27 (the maintainer's pick, "Everywhere"; "i'd generally
// prefer to have a single path when possible"), and since 2026-11-07 they are not rows: the ten big kernels have the one writing (each heavy function called from one
// place; shaders/compact/ and render/kernels.ts are gone) and the gather is its two programs (render/gatherSplit.ts). Their words on an address (`?compactshaders`, `?splitrays`) do nothing.
// THE FIRST-FRAME START IS ON ONLY WHERE A PERSON SITS: a browser a script drives (navigator.webdriver, every page tool) starts with it off,
// because a tool's first picture and first timing must be the settings' own and not a stand-in's (some forty tools poll for a hook and
// begin at once). A tool that wants the player's start asks for it: `?firstframe=1`. `?firstframe=0` is a load that waits for every pipeline (a machine nobody has read
// may need it). What nobody has: ANY time on a real GPU for the first-frame start in this build (docs/agents/render.md has what was seen on its
// author's Windows machine, and the mend made since).
// TWO MORE WENT EARLIER (2026-10-27): Wall cut shaders when needed and Other shaders when needed built a shader the first time a frame asked for it, and the maintainer
// does not want a compile that waits for play to ask ("i think we shouldn't do truly lazy compilation where then have stutters during gameplay").
//   firstFrame     ?firstframe=1       gpu/device.ts PIPE_QUEUE.later: the start waits only for the pipelines the first frame draws
// A URL word wins for that visit. A word counts in a report only in a build that carries the code which reads it: that code says so with useShaderRow() when it is imported.
// A TOOL (no browser): off, unless the environment gives its word, the URL word in capitals: TPS_FIRSTFRAME=1 in front of a headless tool.
// THIS MODULE IMPORTS NOTHING: gpu/ and render/ read it while they are themselves being imported, so it must always be whole before them.
export type ShaderRowKey = 'firstFrame';
interface ShaderRow { key: ShaderRowKey; url: string; label: string }
const ROWS: ShaderRow[] = [
  { key: 'firstFrame', url: 'firstframe', label: 'Start on first frame shaders' },
];
/** this is a browser (a tool's own name is not a browser's: nothing starts on there, whatever machine it runs on) */
const BROWSER = (() => { try { return /\bMozilla\//.test(String(globalThis.navigator?.userAgent ?? '')); } catch { return false; } })();
/** a script is driving this browser (a page tool's Chromium says so; a person's does not) */
const DRIVEN = (() => { try { return globalThis.navigator?.webdriver === true; } catch { return false; } })();
/** where a row stands with no word given */
const starts = (): boolean => BROWSER && !DRIVEN;
/** this visit's URL word for a row: true / false, or null = the address does not say (no page says nothing) */
function urlWord(r: ShaderRow): boolean | null { try { const v = new URLSearchParams(globalThis.location?.search ?? '').get(r.url); return v === null ? null : v !== '0'; } catch { return null; } }

/** a tool's word for a row: TPS_<the URL word in capitals> in the environment, null = none given (a page has no environment) */
function envWord(r: ShaderRow): boolean | null { const v = (globalThis as { process?: { env?: Record<string, string | undefined> } }).process?.env?.[`TPS_${r.url.toUpperCase()}`]; return v === undefined ? null : v !== '0'; }

/** what THIS load was started with: the URL's word, else the environment's, else where the row starts. Read by the code each row switches (never written after the import) */
export const SHADER_ROWS: Record<ShaderRowKey, boolean> = { firstFrame: false };
for (const r of ROWS) SHADER_ROWS[r.key] = urlWord(r) ?? envWord(r) ?? starts();

const used = new Set<ShaderRowKey>();
/** the code a row switches says it is in this build (at its own import), and gets the row's word for this load */
export function useShaderRow(key: ShaderRowKey): boolean { used.add(key); return SHADER_ROWS[key]; }
/** every row's URL word: what a run that goes from page to page must take along for its pages to be built the way the first was asked (ui/benchrun.ts) */
export const SHADER_ROW_WORDS: readonly string[] = ROWS.map(r => r.url);
/** the rows THIS load was started with, by their URL words, of the ones this build carries: what a report says its numbers were taken under (ui/bench.ts) */
export function shaderRowsOn(): string[] { return ROWS.filter(r => used.has(r.key) && SHADER_ROWS[r.key]).map(r => r.url); }
/** the name a report shows for a row's URL word (the word itself where this build has no such row: a report from another build still reads) */
export function shaderRowLabel(word: string): string { return ROWS.find(r => r.url === word)?.label ?? RETIRED.get(word) ?? word; }
/** the rows that are no longer rows, by their old URL words: a report made before 2026-11-07 names them, and still reads */
const RETIRED = new Map([['compactshaders', 'Compact shaders'], ['splitrays', 'Separate shadow rays']]);
/** …their words: what a reader of an OLD record's address still takes for a shader row (tools/cloud/errors.ts) */
export const RETIRED_ROW_WORDS: readonly string[] = [...RETIRED.keys()];
