// 'radiohome' — THE HANDLER'S RADIO ASIDES ARE THE OFFICE NIGHT'S (WORDS.radioHome, dialogue.ts radioLive). The maintainer, off the stealth tutorial mission:
// "the 'subtle was twenty years ago' text with the handler keeps popping up -- should be confined to just the office level". Her dry lines and his answers
// (the drive, the rack, the floor's lights, a body found) were written for the solo night at the office, and were said on every level a solo night ran on:
// a tutorial mission restarted at a checkpoint heard the same pair again each time.
//   bun run tools/qa/radiohome-check.ts        (also `probes.ts radiohome`)
//   office    the office: the lights die and she says her line a beat later, he answers after it. Nothing is taken from the night it was written for.
//   away      three other levels (the stealth tutorial mission among them): the lights die, twelve seconds pass, and no line of hers or his is in the
//             strip; a beat asked for by hand is refused.
// 2026-11-07: the key is a settled rule (on). Gone with that: 'the same levels with the key down: she speaks, as the build did' and the whole of `harness`
// (the two stands' row and its word, a recording from before the key, the page's ?handlerall).
// RADIOHOME_BREAK=1 = `away` is asked of the office itself, where she does speak: away must FAIL.   exit 0 = all hold · 1 = one does not
// THE HARNESS HAS NO GPU and no overlay: what is read is the game's own message list, which the radio strip draws.
import { readFileSync } from 'node:fs';
import { standUp, ROOT } from './headless.ts';

const { LIGHT_IN, MAINS } = await import(`${ROOT}/src/sim/lightshome.ts`);
const D = await import(`${ROOT}/src/game/dialogue.ts`);
const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
const { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`);

const BREAK = process.env.RADIOHOME_BREAK === '1', DT = 1 / 60;
const AWAY = ['tut-quiet.json', 'quiet-yard.json', 'lights-out.json'];
let allPass = true;
const verdict = (ok: boolean, label: string, detail: string) => { allPass &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); };
async function quiet<T>(fn: () => Promise<T>): Promise<T> { const log = console.log; console.log = () => {}; try { return await fn(); } finally { console.log = log; } }

/** a level stood up with the player unseen; the lights die; `secs` later: her lines and his in the strip, and what a beat asked by hand answers */
async function night(file: string | null, secs = 12): Promise<{ hers: string[]; his: string[]; asked: boolean; office: boolean; stage: string }> {
  const W: any = await quiet(() => standUp(file ? { uploadEvery: 4, level: loadLevel(parseLevelData(readFileSync(`${ROOT}/levels/${file}`, 'utf8'))) } : { uploadEvery: 4 })), g = W.game;
  g.godMode = true; g.playerInvisible = true; g.player.visibility = 0.02; for (const x of g.guards) x.hold = true;
  for (let i = 0; i < 10; i++) W.step(DT);
  const from = g.messages.length;
  g.lightHome.in(LIGHT_IN.mains, null, MAINS.active | MAINS.permanent, g.time + 1e6, g.time, g.blackout.restoredAt);   // (the edge tickRadio reads: the lights dying)
  for (let i = 0; i < Math.round(secs * 60); i++) { W.step(DT); for (const x of g.guards) if (x.state === 'alert') x.state = 'patrol'; }
  const said = g.messages.slice(from).map((m: any) => String(m.text));
  const asked = D.handlerBeat(g, 'bodyKo');   // (a beat nothing in this run has sounded)
  return { hers: said.filter((t: string) => t.startsWith(`radio: ${D.HANDLER_TAG} ▸ `)), his: said.filter((t: string) => t.startsWith('radio: you ▸ ')), asked, office: D.isOffice(g.level), stage: g.mission.stage };
}

async function office() {
  console.log('\n=== office: the night she was written for still hears her ===');
  const N = await night(null), pool = D.HANDLER_LINES.blackout;
  const h = N.hers[0]?.slice(`radio: ${D.HANDLER_TAG} ▸ `.length), s = N.his[0]?.slice('radio: you ▸ '.length), row = pool.find((L: any) => L.h === h);
  verdict(N.office && N.hers.length === 1 && N.his.length === 1 && !!row && row.s === s, 'the office: the lights die, she says one line of the blackout pool and he answers with its pair', `office ${N.office} · hers ${JSON.stringify(N.hers)} · his ${JSON.stringify(N.his)}`);
}
async function away() {
  console.log('\n=== away: no other level hears her ===');
  const rows: string[] = []; let bad = 0;
  for (const file of AWAY) {
    const up = await night(BREAK ? null : file);
    const quietUp = up.hers.length === 0 && up.his.length === 0 && up.asked === false;
    if (up.office || !quietUp) bad++;
    rows.push(`${file} (office ${up.office}, mission ${up.stage}): ${up.hers.length} of hers, ${up.his.length} of his, a beat by hand ${up.asked ? 'sounded' : 'refused'}`);
  }
  verdict(bad === 0, `${AWAY.length} other levels, the lights dying on each: nothing of hers or his is in the strip in twelve seconds and a beat asked by hand is refused`, rows.join('\n    '));
}
/** the whole card (probes.ts 'radiohome'): true = every check passed. */
export async function radioHomeCheck(): Promise<boolean> {
  const rnd = Math.random; allPass = true;
  try { await office(); await away(); }
  finally { Math.random = rnd; }
  console.log(`\n=== radiohome: ${allPass ? 'ALL PASS' : 'FAILURES above'} ===`); return allPass;
}
if (import.meta.main) process.exit(await radioHomeCheck() ? 0 : 1);
