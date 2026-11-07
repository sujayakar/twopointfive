// THE STAND A PROBE IS OWED BY ITS NAME. Some probes were written against a rule's build before it — a card's one gun row, the grab on its key, the knife — and
// the rule since has a check of its own on both stands. Such a probe runs with the newer rule held off on EVERY stand, as the frozen stand holds it. Which probe is
// owed which words is this file's one table, and `asProbe(name, fn)` is the one way a probe is run under them: the runner (probes.ts run) calls it for every probe,
// and a check file that can be run by itself calls it from its own main (`asProbes` for a file of several). So `bun run tools/qa/<x>-check.ts` and
// `bun run tools/qa/probes.ts <x>` are one stand. (They were two: the runner held the words, the file's own main did not, and a check was red run as a file and
// green in its family — sticky-check and reachup-check were both found so on one day, and twenty more files stood the same way.)
// holds-check.ts holds this file to its word: a new row here, or a new main in a file that holds a probe of the table, is red there until that main runs the
// probe through this file by its name. (A check file none of whose probes is in the table is not read there: run alone its parts have no fence between them.)
//
// A hold is a stand word set to '0' (src/host/standup.ts reads the word as each game is stood up) and, where the probe reads the rule's table before it stands a
// game up, the table's row put off as well. A word whoever runs the tool set himself wins: `TPS_PACING=1 bun run tools/qa/noshove-check.ts` asks the check with
// the pacing on (and `TPS_PACING=0` in front is the hold itself, said by hand: a stand-up raises a row for '1' alone).
import { resolve } from 'node:path';
import { FOLD, foldLost, foldSpeaker, foldOutside, foldLine, foldFor, foldMode } from './fold.ts';
const ROOT = resolve(import.meta.dir, '../..');   // (not the harness's own: a tool that names its stand before it loads the harness can still import this file first)

/** One held row: the stand word, the knob table and the row's key (src/game/knobs.ts's registry), and whether the row is put off at once as well as the word
 *  (`now`: a probe that reads the table before it stands a game up) or by the word alone, at the next stand-up. */
export type Hold = { readonly word: string; readonly table: string; readonly key: string; readonly now: boolean };
const row = (word: string, table: string, key = 'on', now = true): Hold => ({ word, table, key, now });
const atStandUp = (word: string, table: string, key = 'on'): Hold => row(word, table, key, false);

/** The table: the probe names (as probes.ts dispatches them) and what each is owed. A name can stand in several rows. `list` is the row's name: the one the
 *  area files (docs/agents) call it by, from when each row was a list of its own in probes.ts. A name is written one of three ways: 'reachup' = that probe;
 *  'close-*' = every probe whose name begins so; 'sticky(-*)' = both (the probe of that name and every probe that begins with it and a dash). */
export const HOLDS: readonly { readonly list: string; readonly names: readonly string[]; readonly holds: readonly Hold[] }[] = [
  // FOURTEEN ROWS LEFT THIS TABLE ON 2026-11-07. Each held OFF only rules that are settled rows now: constants of the game, held at their page value in every
  // tool on the page's stand (tools/qa/fold.ts), where the hold was refused and the probe ran on the page's model whatever the table said; on the old stand
  // those rows are off already and the hold did nothing. The probes they named are not mended by leaving: each was written for an older model of the kit, the
  // grab, the knife, the menu's rows or the squad's pace, and is to be restaged on the game as it is or deleted. WHO WAS OWED WHAT, all 119 names:
  // `git show cd215879:tools/qa/holds.ts` (the 20 kit probes still here were owed the deal and the gun rows too). The two rows left hold rows that are still
  // switches: the big slot, and the squad's pacing (an open decision of the maintainer's).
  // THE KIT CARDS AS THEY WERE. These probes set a wall charge or the breaching shotgun through rows of their own and count pouches for them — the
  // card of the build before the big slot (src/game/kit.ts BIGSLOT, on in the page: one big row, the shotgun off the weapon row, big things free of pouches).
  // The card as the page has it is bigslot-check.ts's, on both stands. (owncharge-kit reads both cards itself: the rows a thing first, the big slot at its foot.)
  { list: 'CARDS_AS_THEY_WERE', names: ['kit(-*)', 'pouch(-*)', 'shotgun(-*)', 'squadsmoke(-*)'], holds: [row('TPS_BIGSLOT', 'BIGSLOT')] },
  // …and the pace a teammate takes off the player (src/game/teampace.ts TEAMPACE.pacing, on in the page: his walk is the player's own, not the flat 0.8 of it)
  // for two probes whose FAULT arm was staged at the flat walk: leafstall's found case (rule off, he treads against the back of the open leaf for a second and
  // a half; at the player's walk he is through it in 0.7 s) and noshove's room keeper (rule off, the entry's timing stands a man across the player and the
  // keeper walks out of his way; at the player's walk the entry lands otherwise). Their mend lines are green with the pace on (TPS_PACING=1 in front shows it:
  // only the fault arm goes red).
  { list: 'PACE_AS_IT_WAS', names: ['leafstall', 'noshove'], holds: [atStandUp('TPS_PACING', 'TEAMPACE', 'pacing')] },
];

/** whether a name written in the table (one of its three ways) is this probe's */
export function named(entry: string, probe: string): boolean {
  if (entry.endsWith('(-*)')) { const b = entry.slice(0, -4); return probe === b || probe.startsWith(`${b}-`); }
  return entry.endsWith('-*') ? probe.startsWith(entry.slice(0, -1)) : probe === entry;
}
/** What the probe of this name is owed: every row of the table its name stands in, a word once. */
export function holdsOf(name: string): Hold[] {
  const out: Hold[] = [];
  for (const r of HOLDS) if (r.names.some(e => named(e, name))) for (const h of r.holds) if (!out.some(o => o.word === h.word)) out.push(h);
  return out;
}
/** One row of the table by its name, for a check that holds a rule off round a block of its own lines the way a probe of that row is held. */
export function heldList(list: string): readonly Hold[] { const r = HOLDS.find(x => x.list === list); if (!r) throw new Error(`holds.ts: no row '${list}' (${HOLDS.map(x => x.list).join(' ')})`); return r.holds; }

/** Put these holds on and hand back what takes them off again. (Not headless.ts's `hold`, which is the Rust rows' arm of a check.) A word nobody set is set to '0' (and deleted after); a word whoever runs the tool set himself is
 *  left as he set it — '1' is his to ask and nothing of that row is touched; anything else is the hold itself, as the stand-up reads it (a game stood up has
 *  the row on for '1' alone), so the row is put off all the same: a child of a held probe stands where its parent stands. */
export async function holdOff(holds: readonly Hold[]): Promise<() => void> {
  const env = process.env, { knobRegistry } = await import(`${ROOT}/src/game/knobs.ts`), R = knobRegistry() as Record<string, Record<string, unknown> | undefined>;
  const put: (() => void)[] = [];
  const back = (): void => { for (const b of put.reverse()) b(); put.length = 0; };
  for (const h of holds) { const given = env[h.word];
    const T = R[h.table]; if (!T || typeof T[h.key] !== 'boolean') { back(); throw new Error(`holds.ts: ${h.table}.${h.key} (${h.word}) is no switch of the knob registry — a retired row is taken out of the table`); }
    if (given === '1') continue;
    const was = T[h.key]; if (given === undefined) env[h.word] = '0'; if (h.now) T[h.key] = false;
    put.push(() => { if (given === undefined) delete env[h.word]; if (h.now) T[h.key] = was; }); }
  return back;
}

/** Run one probe as the runner runs it, whoever calls: the words its name is owed held (holdOff, above), and round it the fence that keeps one probe's leftovers
 *  out of the next of the same process — every knob table copied before and put back after (src/game/knobs.ts: a key a probe added is dropped, every value
 *  restored in place; the cut-pieces switch is one of them), 'the plain night wears your card' (kit.ts KIT_SOLO, a flag of the visit in no knob table), and the
 *  loud card first in the table again (kit.ts kitMission: a probe that staged a quiet run leaves your stealth card first). Throws, after everything else is
 *  back, if the tables cannot be put back. */
let fenceDepth = 0;
/** mulberry32, the harness's die (headless.ts `seeded`; said again here because this file loads before the harness may) */
function mulberry(a: number): () => number { return () => { a |= 0; a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ (a >>> 15), 1 | a); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; }; }
export async function asProbe<T>(name: string, fn: () => T | Promise<T>): Promise<T> {
  const { knobsCopy, knobsBack } = await import(`${ROOT}/src/game/knobs.ts`), kit = await import(`${ROOT}/src/game/kit.ts`);
  // (tools/qa/fold.ts: one line a probe from whoever runs the probes, the outermost. Under the runner, whose run() says it, this says nothing.)
  const says = fenceDepth === 0 && foldSpeaker('probe'); if (says) { const o = foldOutside(`before ${name}`); if (o) console.log(o); }   // (a probe run inside another's fence is the outer one's)
  const foldWas = fenceDepth === 0 ? foldMode() : null;   // (fold.ts foldFor: the fold on or off as this fence opens, put back FIRST as it closes, so the tables go back in the mode they were copied in)
  const soloOn = kit.KIT_SOLO.on, knobs = knobsCopy(), release = await holdOff(holdsOf(name));
  // (THE DIE IS PART OF THE FENCE: every probe begins on die 1 and whatever die it leaves in `Math.random` is taken out after it, so a probe that stands a game
  // with no seed stands the same game alone, after any other probe, and on every run — headless.ts ENTROPY)
  const dieWas = Math.random; if (fenceDepth === 0) Math.random = mulberry(1);
  let threw: unknown, failed = false, red = false; fenceDepth++;
  try { const r = await fn(); red = (r as unknown) === false; return r; }
  catch (e) { failed = true; red = true; threw = e; throw e; }
  finally {
    foldFor(foldWas);
    if (fenceDepth === 1) Math.random = dieWas;
    try { kit.kitMission(null); } catch { /* (as the runner always ran it: a throw here is not the probe's verdict) */ }
    release(); kit.KIT_SOLO.on = soloOn;
    fenceDepth--; if (FOLD.on) { try { foldLost(); } catch { /* the put-back below still runs */ } }   // (tools/qa/fold.ts: a settled row's hold lost to a branch replaced whole is found BEFORE the tables go back, so it is named with the value the probe left it at)
    try { knobsBack(knobs); if (FOLD.on) foldLost(); }   // (…and after: the put-back makes again a branch the probe deleted, plain)
    catch (e: any) { throw new Error(`the knob tables could not be put back after '${name}' — ${e?.stack ?? e}${failed ? ` (and the probe itself threw — ${(threw as any)?.stack ?? threw})` : ''}`); }
    finally { if (says) console.log(foldLine(name, red)); }
  }
}

/** A check file's own main: its probes under the names the runner calls them by, each run as the runner runs it. `want` = the names asked for (none = all, in
 *  the table's order; a main hands it `wanted()`), each whole or without its family ('kit' for 'sticky-kit'). A probe that throws is said and the rest go on, as under the runner. A name
 *  the file has not is said and nothing is run: null (exitCode: 2), so that a mistyped name is never a green run of nothing. ONE THING THE RUNNER DOES MORE: it
 *  also fails a probe that prints a FAIL or an error line and returns true; a file's own verdict is what its probes return. */
export async function asProbes(probes: Readonly<Record<string, () => boolean | Promise<boolean>>>, want: readonly string[] = []): Promise<boolean | null> {
  const names = Object.keys(probes), short = (n: string): string => n.slice(n.indexOf('-') + 1);
  const picked = want.map(w => names.find(n => n === w) ?? names.find(n => short(n) === w)), unknown = want.filter((_, i) => !picked[i]);
  if (unknown.length) { console.log(`unknown: ${unknown.join(' ')} (one or more of: ${names.join(' ')})`); return null; }
  let ok = true;
  for (const name of names) { if (want.length && !picked.includes(name)) continue;
    let r = false; try { r = await asProbe(name, probes[name]); } catch (e: any) { console.log(`error: probe '${name}' threw — ${e?.stack ?? e}`); }
    ok &&= r; }
  return ok;
}
/** the names asked for on a command line: its words that are no flag */
export function wanted(argv: readonly string[] = process.argv.slice(2)): string[] { return argv.filter(a => !a.startsWith('-')); }
/** a main's exit code, the runner's own: 0 all passed · 1 one failed · 2 a name the file has not */
export function exitCode(ok: boolean | null): number { return ok === null ? 2 : ok ? 0 : 1; }
