// A DEFENDER SEES WHAT HE WATCHES. A level's file stands men at marks and says what each one's gun rests on: `pos` and `aimAt` (a room clear's
// structure, scenarios.roomClear rooms[].marks; a rescue's captor, door men, guards and roamers; a plan's hostiles). The level's fight is built on those
// lines: the pass's tea room holds its yard through two windows. DRESSING NEVER BLINDS A DEFENDER (the dress goal's rule, 11-05: "a window a mark
// watches through, or a covered lane crosses, stays see-through"; the maintainer: "we don't want them to feel dumb"): a board nailed over that window, a
// shutter shut on it, a crate stood in the lane, and the man stares at wood. So a window's dressing has two forms in the kit (scene/gwotKit.ts boards,
// shutters, windowBlanket: 'shut' and 'open'), and a level lays the open one where a man watches.
// WHO IS ASKED: every thing in the file, outside its boxes and props, that says both a `pos` and an `aimAt` — nothing here names a kind of mission, so
// the next kind that stands a man with something to watch is asked the day it is written. Each is named by where the
// file has them (`roomClear.tea.marks#0`, `rescue.captor`, `rescue.guards#0`).
// WHAT IS HELD, on every level file that stands such a man:
//   1 the line     from each man's eye (1.5 m over his `pos`) to his `aimAt`, no static of the level stops a look (collision.ts segmentBlocked: glass
//                  passes, as it does for his eyes). Asked of the boxes as the file has them. DOORS ARE LEAVES THAT SWING AND ARE NOT ASKED: every
//                  leaf is taken out of the world for the questions (a man whose gun rests on a door looks at the doorway)
//   2 the ground   one line slips between two planks, so each man's LOOK is a fan: 21 lines from his eye to a man's shin, belt and head (0.5, 1.0, 1.5 m)
//                  at seven places across 3 m of the ground his gun rests on. How many are open is held in a file a level (tools/qa/fixtures/marksight/),
//                  and the check is RED WHEN A MAN SEES LESS than his file says: something was stood in his view. Seeing MORE is said and passes (run
//                  with --write to keep the new count); a level with no file is red. THE FILES ARE PER-LEVEL FIXTURES: whoever moves a level file so
//                  that a man sees less carries that level's file with the move and says why in it, on a line that starts `#` (--write keeps
//                  those lines, and says aloud every count it LOWERS).
// A LEVEL THAT STANDS NO SUCH MAN IS NOT JUDGED (the quiet missions, the tutorials without a room fight, the showrooms): the check names them, so a green
// is not read as a proof there.
//   bun run tools/qa/marksight-check.ts [level …] [--write]     exit 0 = pass · 1 = a FAIL line · 2 = an unknown level
//   MARKSIGHT_BREAK=1 …                                         the red path: a post is stood in the middle of every level's first line (never so wide that
//                                                               it holds the man's eye or his aim), and both lines must FAIL ON EVERY LEVEL JUDGED
// THE TWO STANDS ARE TWO LEVELS TO A LOOK (the page's has the lived-in rooms' furniture and the shapes), so a file holds a count for each: a line is
// `name <frozen> <shipped>`, and a run judges, and --write keeps, the column of the stand it is run on (TPS_PROFILE=shipped in front for the page's).
// The probe (probes.ts `marksight`, family `missions`) runs it on the page's stand, tools/levels/export.ts asks it of a level the day it is written, and
// level-needs.ts lists it among a level's own checks; run both stands by hand before a level file moves.
// AS FIRST RUN (11-05, the head before any level was dressed for it): 536 men in 41 level files, every line open on both stands.
import { readFileSync, readdirSync, existsSync, writeFileSync, mkdirSync } from 'node:fs';
import { standUp, ROOT } from './headless.ts';
import { quiet } from './answers.ts';

const BREAK = process.env.MARKSIGHT_BREAK === '1', COL = process.env.TPS_PROFILE === 'shipped' ? 1 : 0, STAND = COL ? 'shipped' : 'frozen';
const EYE = 1.5, HEIGHTS = [0.5, 1.0, 1.5], ACROSS = [-1.5, -1, -0.5, 0, 0.5, 1, 1.5], FAN = HEIGHTS.length * ACROSS.length, DIR = `${ROOT}/tools/qa/fixtures/marksight`;

interface Man { id: string; pos: number[]; aimAt: number[] }
/** every man the file stands with something to watch: whatever, outside the boxes and the props, says a `pos` and an `aimAt`. He is named by where the
 *  file has him, under `scenarios` without that word; one of a list by his place in it (`rescue.guards#0`), and a thing of a list that says an `id` by
 *  its id in place of the list's name (`roomClear.tea.marks#0`: the tea room's first mark, wherever the room stands in the list) */
function menOf(data: any): Man[] {
  const out: Man[] = [], triple = (v: unknown): v is number[] => Array.isArray(v) && v.length === 3 && v.every(x => typeof x === 'number');
  const walk = (o: any, path: string): void => {
    if (!o || typeof o !== 'object') return;
    if (Array.isArray(o)) { o.forEach((x, k) => walk(x, typeof x?.id === 'string' ? `${path.slice(0, path.lastIndexOf('.') + 1)}${x.id.replace(/\s/g, '_')}` : `${path}#${k}`)); return; }
    if (triple(o.pos) && triple(o.aimAt)) out.push({ id: path, pos: o.pos, aimAt: o.aimAt });
    for (const [k, v] of Object.entries(o)) walk(v, path ? `${path}.${k}` : k);
  };
  for (const [k, v] of Object.entries(data)) if (k === 'scenarios') walk(v, ''); else if (k !== 'boxes' && k !== 'props') walk(v, k);
  return out;
}

export async function markSightCheck(args: string[] = []): Promise<number> {
  const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
  const { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`);
  const write = args.includes('--write') && !BREAK, names = args.filter(a => !a.startsWith('--')).map(n => n.replace(/\.json$/, ''));
  const levels = names.length ? names : readdirSync(`${ROOT}/levels`).filter(f => f.endsWith('.json') && !f.includes('.lint.')).map(f => f.replace(/\.json$/, '')).sort();
  for (const n of levels) if (!existsSync(`${ROOT}/levels/${n}.json`)) { console.log(`marksight: no level ${n}`); return 2; }
  let stood = 0, marks = 0, shut = 0, less = 0, more = 0, nofile = 0, bit = 0; const rows1: string[] = [], rows2: string[] = [], wrote: string[] = [], lowered: string[] = [], unbit: string[] = [], unjudged: string[] = [];
  console.log(`\n=== marksight: a defender sees what he watches, on the ${STAND} stand${BREAK ? ' (MARKSIGHT_BREAK: a post in the middle of each level\'s first line)' : ''} ===`);
  for (const name of levels) {
    const data: any = parseLevelData(readFileSync(`${ROOT}/levels/${name}.json`, 'utf8')), men = menOf(data);
    if (!men.length) { unjudged.push(name); continue; }
    const level = loadLevel(data);
    if (BREAK) { const m = men[0], L = Math.hypot(m.aimAt[0] - m.pos[0], m.aimAt[2] - m.pos[2]), half = Math.min(0.8, L / 4);   // (a quarter of the line at most: a look that starts inside a box is not stopped by it)
      level.boxes.push({ ...level.boxes[0], c: [(m.pos[0] + m.aimAt[0]) / 2, 1.0, (m.pos[2] + m.aimAt[2]) / 2], h: [half, 1.0, half], yaw: 0, flags: 0, rot: undefined, shape: undefined, marks: undefined } as never); }
    const H: any = await quiet(() => standUp({ level })), col = H.game.col; stood++; col.dynamicBoxes = [];   // (the leaves out: game.ts lays every shut one over the statics at stand-up. This game is asked nothing else)
    const file = `${DIR}/${name}.txt`, lines = existsSync(file) ? readFileSync(file, 'utf8').split('\n') : [], notes = lines.slice(1).filter(l => l.startsWith('#')), held = new Map<string, number[]>(lines.filter(l => l && !l.startsWith('#')).map(l => { const [k, a, b] = l.split(' '); return [k, [Number(a), Number(b)]] as [string, number[]]; }));
    const now: string[] = [], bad1: string[] = [], bad2: string[] = [];
    for (const m of men) {
      marks++; const floor = m.pos[1], eye = [m.pos[0], floor + EYE, m.pos[2]], id = m.id;   // (his own floor: the ground he watches is taken to be on it)
      const blocked = col.segmentBlocked(eye, m.aimAt) as boolean;
      if (blocked) bad1.push(`${id} (from ${m.pos[0]}, ${m.pos[2]} to ${m.aimAt.join(', ')})`);
      const dx = m.aimAt[0] - eye[0], dz = m.aimAt[2] - eye[2], L = Math.hypot(dx, dz) || 1, px = -dz / L, pz = dx / L; let n = 0;
      for (const y of HEIGHTS) for (const s of ACROSS) if (!col.segmentBlocked(eye, [m.aimAt[0] + px * s, floor + y, m.aimAt[2] + pz * s])) n++;
      const row = held.get(id) ?? [NaN, NaN], was = row[COL]; row[COL] = n; now.push(`${id} ${Number.isNaN(row[0]) ? '-' : row[0]} ${Number.isNaN(row[1]) ? '-' : row[1]}`);
      if (was === undefined || Number.isNaN(was)) { if (existsSync(file) && !write) bad2.push(`${id}: no count for the ${STAND} stand in the file`); } else if (n < was) bad2.push(`${id}: ${n} of ${FAN}, the file says ${was}`); else if (n > was) more++;
    }
    if (!existsSync(file) && !write) { nofile++; rows2.push(`${name}: no file (--write)`); }
    if (bad1.length) { shut += bad1.length; rows1.push(`${name}: ${bad1.join(' · ')}`); }
    if (bad2.length) { less += bad2.length; rows2.push(`${name}: ${bad2.join(' · ')}`); if (write) lowered.push(`${name}: ${bad2.join(' · ')}`); }
    if (BREAK) { if (bad1.length && bad2.length) bit++; else unbit.push(name); }
    if (write) { const text = `# how many of ${FAN} lines each man of ${name} has open to the ground his gun rests on: his name, the frozen stand's count, the shipped stand's (tools/qa/marksight-check.ts --write, once on each stand; a line that starts # under this one is a note and is kept)\n${notes.length ? notes.join('\n') + '\n' : ''}${now.join('\n')}\n`;
      if (!existsSync(file) || readFileSync(file, 'utf8') !== text) { mkdirSync(DIR, { recursive: true }); writeFileSync(file, text); wrote.push(name); } }
  }
  const ok1 = shut === 0 && (stood > 0 || names.length > 0),   // (asked of named levels that stand no such man, there is nothing to hold: the line below names them)
    ok2 = write || (less === 0 && nofile === 0);
  // THE RED PATH'S OWN VERDICT: it must bite every level judged. Where it does not, the run says so and EXITS 0: a break word that passes is the fault to find
  if (BREAK) { console.log(`    (the red path: ${bit} of ${stood} levels are red on both lines${unbit.length ? ` — NOT BITTEN: ${unbit.join(', ')}` : ''})`); if (unbit.length) { console.log('=== marksight: THE RED PATH DID NOT BITE EVERY LEVEL (exit 0: a break word that passes) ==='); return 0; } }
  console.log(`- ${ok1 ? 'PASS' : 'FAIL'} — 1 each man's line from his eye to what his gun rests on is open (glass passes; the level's statics as the file has them)\n    ${stood} levels, ${marks} men, ${shut} blocked${rows1.length ? ' — ' + rows1.join(' · ') : ''}${unjudged.length ? `\n    NOT JUDGED, no man stood with something to watch: ${unjudged.join(', ')}` : ''}`);
  console.log(`- ${ok2 ? 'PASS' : 'FAIL'} — 2 no man sees less of the ground he watches than his level's file says (${FAN} lines to a man's shin, belt and head across 3 m of it)\n    ${write ? `--write: ${wrote.length ? 'wrote ' + wrote.join(', ') : 'every file was up to date'}${lowered.length ? `\n    LOWERED BY THIS WRITE (a man sees less than his file said: say why in the file, on a # line): ${lowered.join(' · ')}` : ''}` : `${less} men see less, ${nofile} levels have no file${rows2.length ? ' — ' + rows2.join(' · ') : ''}${more ? ` · ${more} men see MORE than their file says (keep it with --write)` : ''}`}`);
  console.log(`=== marksight: ${ok1 && ok2 ? 'ALL PASS' : 'FAIL'} ===`);
  return ok1 && ok2 ? 0 : 1;
}
if (import.meta.main) process.exit(await markSightCheck(process.argv.slice(2)));
