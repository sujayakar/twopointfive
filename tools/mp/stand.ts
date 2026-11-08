// STANDING A NETWORKED GAME HEADLESS (bun): the two stand-ups a game server and a headless client need (src/net/wire.ts says what a recipe is).
//   freshRecipe(setup)   a game stood as THE PAGE would stand it for an address (the shipped rules, the page's switches for that query, the scenario started),
//                        with the tables a networked game needs forced on (wire.ts NET_TABLES) and its recipe taken (game/runtape.ts recipeOf). A setup's own
//                        tables are a host's settings: each key must be a knob this build has, of the knob's kind, or the setup is refused. That game is
//                        thrown away: run it in a process of its own (tools/mp/setup.ts).
//   standRecipe(recipe)  THE game a server or a client plays: stood from the recipe the way a replay's re-run stands a tape's (tools/qa/rerun.ts, a tape of no
//                        steps), every seat refereed. A recipe without the networked tables, or one whose mission does not come up here, is refused. ONE a
//                        process: the stand writes this process's knob tables and module state.
// The server stands its own game from the recipe too, never from the fresh one: every machine comes to its game by the same door.
import { resolve } from 'node:path';
import type { Recipe } from '../../src/net/recipe.ts';
import type { Headless } from '../qa/headless.ts';
import { wordsKept, type Stood } from './stood.ts';
import type { RunTape } from '../../src/replay/format.ts';

const ROOT = resolve(import.meta.dir, '../..');

/** what whoever makes a game says of it: the page's address for the mission (its level file, its scenario, its squad), the die, and optionally knob tables laid
 *  over the page's own (the host's settings: a difficulty, Team feel rows). The setup record's `scenario`, `seed` and `knobs` (tools/mp/server.ts) */
export interface Setup { search: string; seed: number; tables?: Record<string, Record<string, unknown>>; }

const plain = (v: unknown): v is Record<string, unknown> => v !== null && typeof v === 'object' && !Array.isArray(v);
const kind = (v: unknown): string => v === null ? 'null' : Array.isArray(v) ? 'list' : typeof v;
/** why a setup's table cannot be laid over the knob table `dst` ('' = it can): every key a knob the table has itself, of the knob's own kind */
function layWhy(dst: Record<string, unknown>, src: Record<string, unknown>, path: string): string {
  for (const k of Object.keys(src)) { const at = `${path}.${k}`, a = dst[k], b = src[k];
    if (!Object.hasOwn(dst, k) || typeof a === 'function') return `${at} is not a knob of this build`;
    if (plain(a) && plain(b)) { const why = layWhy(a, b, at); if (why) return why; }
    else if (a !== null && kind(a) !== kind(b)) return `${at} is a ${kind(a)}, not a ${kind(b)}`; }   // (a knob that stands at null takes a value: K.assignDeep)
  return '';
}
/** A LEVEL FILE NAMED BY AN ADDRESS IS A NAME, never a path: the address comes from whoever asked for the game (a lobby, a server's recipe), and what it
 *  names is read off this machine's disk → why not ('' = it is one of levels/) */
export function levelWhy(file: string | undefined): string { return file === undefined || /^[A-Za-z0-9][A-Za-z0-9_-]*\.json$/.test(file) ? '' : `levelFile=${file.slice(0, 60)} is not the name of a level file`; }

export async function freshRecipe(setup: Setup): Promise<Recipe> {
  if (!Number.isInteger(setup.seed) || setup.seed < 0 || setup.seed > 0xFFFFFFFF) throw new Error(`a setup's seed is a whole number from 0 to 4294967295, not ${setup.seed}`);   // (the die's seed is 32 bits: a bigger one would fold onto another game's)
  process.env.TPS_PROFILE = 'shipped';   // (read by the harness as it loads: before the import below)
  const q = new URLSearchParams(setup.search), levelFile = q.get('levelFile') ?? undefined;
  { const why = levelWhy(levelFile); if (why) throw new Error(`the setup's address: ${why}`); }
  const P = await import(`${ROOT}/src/host/profile.ts`), K = await import(`${ROOT}/src/game/knobs.ts`);
  const { standUp } = await import('../qa/headless.ts'), { levelOf } = await import('../qa/rerun.ts');
  const { activeScenario } = await import(`${ROOT}/src/game/scenarioRun.ts`), { recipeOf } = await import(`${ROOT}/src/game/runtape.ts`), { NET_TABLES } = await import(`${ROOT}/src/net/recipe.ts`);
  P.shippedBoot(); P.pageSwitches(q);
  const reg = K.knobsNamed() as Record<string, Record<string, unknown>>, asked = setup.tables ?? {};
  if (!plain(asked)) throw new Error('a setup\'s tables are an object of knob tables');
  for (const [name, t] of Object.entries(asked)) { const why = !Object.hasOwn(reg, name) ? `${name} is not a knob table of this build` : !plain(t) ? `${name} is not a table` : layWhy(reg[name], t, name); if (why) throw new Error(`the setup's tables: ${why}`); }
  const tables = () => { for (const src of [asked, NET_TABLES as Record<string, Record<string, unknown>>]) for (const [name, t] of Object.entries(src)) K.assignDeep(reg[name], t); };
  tables();
  const level = levelFile ? await levelOf(levelFile) : undefined;
  const W = await standUp({ seed: setup.seed >>> 0, address: setup.search, tables, dress: true, ...(level ? { level } : {}) });   // (the address is the stand's from its first line, as the page's is: what a page reads of it before the game is made — the weather the game is born in, the difficulty, a host's Hard being the server's Hard — and how many seats, src/game/seats.ts seatsOfSearch. `tables`: the setup's and the networked game's over what the address wrote)
  tables();
  const r = W.start(); if (r.why) throw new Error(`the mission did not start: ${r.why}`);
  if (!activeScenario(W.game)) throw new Error(`the address ${setup.search} names no mission`);
  tables();
  return { ...recipeOf(W.game), search: setup.search, ...(levelFile ? { levelFile } : {}), stand: 'page' };
}

/** a game stood from a recipe, with the copy of it this process keeps */
export interface StoodCopy extends Stood { W: Headless }

export async function standRecipe(recipe: Recipe): Promise<StoodCopy> {
  process.env.RH_QUIET = '1';   // (the host's quiet run is asked for by name, tools/blitz/rust-host.ts quietBegin: a networked game's machines step in them)
  const { recipeWhy, fromWire } = await import(`${ROOT}/src/net/recipe.ts`);
  { const why = recipeWhy(recipe) || levelWhy((recipe as { levelFile?: string }).levelFile); if (why) throw new Error(`no networked game can be stood from this recipe: ${why}`); }   // (…a recipe a server sent names its level the same way)
  // (the men's dress follows the place, read off the address before the cast is loaded: main.ts's line, so every machine of one game stands the same men)
  { const L = await import(`${ROOT}/src/scene/look.ts`); L.setLookPlace(L.placeOfQuery(new URLSearchParams((recipe as { search?: string }).search ?? ''))); }
  const { rerun } = await import('../qa/rerun.ts');
  const { TAPE_VERSION } = await import(`${ROOT}/src/replay/format.ts`);
  const tape: RunTape = { v: TAPE_VERSION, ticks: 0, recipe, dt: [], view: [], cam: [], input: [], lens: [], x: [] };
  const F = { v: 1, game: 'twopointsix', kind: 'replay', savedAt: '', why: 'net', level: { fp: '', boxes: 0 }, flags: {}, hz: 60, seconds: 0, samples: [], cast: [], notes: [], map: { doors: [] }, rerun: tape };
  // (`scan: false`: once it is stood NOTHING writes this game between two ticks. Every change comes as a seat's record through netStep, so the host of the
  //  Rust game does not walk the puppet for a tool's writes before a step or a look ahead: about 3 ms of each on a 4 core box)
  // (`dress: true`: A NETWORKED GAME STANDS AS A PAGE DOES ON EVERY MACHINE. The harness's own stand puts the dressing rows down (src/game/dress.ts), and
  //  in the Rust game those rows are not picture alone: with them down the debris pool hands out no chunk and no glass plate, which are rows of the game's
  //  box table, and a storm's wind source reads nothing. A player's page has them up, and no recipe carries them)
  //  (`draws: false`: the rows are up for the game's sake. Nothing is drawn here, so the page's dress step, which hands the men, the wind and the air
  //  to a renderer, is not run on this process's copy: it cost a two-seat client most of its tick)
  const R = await rerun(F as never, { toN: 0, scan: false, dress: true, draws: false });
  if (R.refuse || !R.W) throw new Error(`the game could not be stood from its recipe: ${R.refuse}`);
  const W = R.W, g = W.game;
  const { netStep, netStand, netWord, netWatch, netQuiet, netWhole, netAhead } = await import(`${ROOT}/src/net/step.ts`), { wordTick, NET_WORD_SEED } = await import(`${ROOT}/src/net/words.ts`), { activeScenario } = await import(`${ROOT}/src/game/scenarioRun.ts`);
  if (activeScenario(g) !== recipe.scenario) throw new Error(`the recipe's mission (${recipe.scenario}) did not come up here (${activeScenario(g) ?? 'none'} is up): ${(R.report?.warnings ?? []).find((w: string) => w.includes('did not start')) ?? 'another build or level than the recipe\'s'}`);
  netStand(g);
  const keep = wordsKept();
  let word: number = NET_WORD_SEED, watch: string | null = null;
  let ticks = 0;
  const out: StoodCopy = { W, seats: g.seats.length, warnings: R.report?.warnings ?? [], get ticks() { return ticks; },
    step(cmds) { netStep(g, W.engine, cmds.map(fromWire)); word = netWord(g, word); ticks++; if (wordTick(ticks - 1)) { watch = netWatch(g); const A = keep && netWhole(g); if (A) keep!(ticks, A); } },
    show(seat) { g.localSeat = g.seats[seat]; },
    ahead: (seat, cmds) => netAhead(g, g.seats[seat], cmds),
    quiet: on => netQuiet(g, on), watch: () => watch,
    begin: () => { (g as unknown as { rustHost?: { startNow?(): void } }).rustHost?.startNow?.(); },
    word: () => word.toString(36), ended: () => { const V = g.verdict.result; return V ? { won: V.won, end: V.word, why: V.why } : null; } };
  return out;
}
