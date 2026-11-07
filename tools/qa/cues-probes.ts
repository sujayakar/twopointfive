// The 'cues' family (game/cues.ts — the defenders' net on the radio strip, ranked and rate-limited, and a line for the decisions they took in silence), run
// through probes.ts:   bun run tools/qa/probes.ts cues | cues-lines | cues-run
// PASS / FAIL each, all headless. No picture and no sound here: 'drawn' is the strip's own row list (game.view.radio, what the HUD draws), not a screen.
//   cues-lines   every new line of the table falls in the class it was written for, and the defenders' existing calls fall where the ranking puts them
//   (the strip's own rules, a unit's line gets a row, the same words twice, a class inside its gap, a full strip: rust/game_start/tests/radio.rs)
//   cues-run     the team met in the mess on levels/block-squads.json: no cue is ever written on a Guard, the net reaches the strip, silent decisions get
//                a line, Sam down is told to the floor, a place on the net is the level's own. CUESRUN_BREAK=1 = Sam's god mode is left on: must fail
// The rule is settled (CUES.on): no line asks the switch off and nothing here writes it (until 2026-11-07 cues-strip's last line and half of every line of
// cues-run asked the message ring as it was before the rule, and cues-run's first line held the run with the switch on to the run with it off).
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { standUp, ROOT } from './headless.ts';
const { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`);
const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
const { startFromQuery } = await import(`${ROOT}/src/game/scenarioRun.ts`);
const { cueClass, cueLines } = await import(`${ROOT}/src/game/cues.ts`);
const { stripRows } = await import('./heard.ts');
const { roomName, placeName } = await import(`${ROOT}/src/game/dialogue.ts`);

const DT = 1 / 60;
function seeded(a: number) { return () => { a |= 0; a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ (a >>> 15), 1 | a); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; }; }
type Verdict = (ok: boolean, label: string, detail: string) => void;
function family(title: string): { verdict: Verdict; done: () => boolean } {
  console.log(`\n=== ${title} ===`); let all = true;
  return { verdict: (ok, label, detail) => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); }, done: () => { console.log(`=== ${title.split(':')[0]}: ${all ? 'ALL PASS' : 'FAILURES above'} ===`); return all; } };
}

/** the row of the ranking each new line belongs in (rebuild/telegraphing/decisions.md §7: 1 coming · 2 stood down · 3 holding on purpose · 4 the net knows · 5 fell back · 6 still alert; 0 = spoken, not a net line) */
const WANT: Record<string, number> = { help: 1, stay: 3, stayOurs: 3, huntOver: 2, calm: 2, cover: 2, still: 6, fellBack: 5, planted: 5, pinnedIn: 5, squadHears: 4, squadCopies: 4, squadWaits: 3, squadCopiesWaits: 3, squadStays: 3, squadQuiet: 2, surprised: 0, flinch: 0 };
/** the defenders' existing radio lines, as the 48 logged runs heard them, and the row of the ranking each belongs in */
const OLD: [string, number][] = [
  ['moving round — coming at him from the mess', 1], ['going around — coming at him from the mess', 1], ["copy — he's got one of ours. stay locked down. box him in. no shot unless it's clean", 1], ['Bravo: two going around — keep them there!', 1], ['coming up behind him. keep him where he is', 1], ["they're coming out — get around them, go!", 1], ['set', 1], ['go go go', 1], ['Bravo: two going round — keep them there!', 1], ['Bravo: moving to Alpha — 3 of us!', 1], ["can't wait — pushing, go", 1],
  ["he's down — hold where you are", 2],
  ['Charlie: holding — lock the doors, let them come!', 3], ["heard. they'll find this room ready.", 3], ["copy. we'll be ready for them.", 3], ['we hear it. getting ready for them here.', 3],
  ['Alpha: contact — mess!', 4], ['shots fired! on me!', 4], ['that was a shot — find him!', 4], ["he's moved — the mess", 4],
];

export async function cuesLines(): Promise<boolean> {
  const F = family('cues-lines: every line falls in the row of the ranking it was written for');
  const L = cueLines() as { kind: string; cls: number; text: string }[], net = L.filter(l => l.cls > 0);
  const bad = L.filter(l => l.cls !== WANT[l.kind]);
  F.verdict(net.length >= 14 && !bad.length && new Set(L.map(l => l.kind)).size === Object.keys(WANT).length, 'the new lines: each kind in its row (the table carries the class; the words do not decide it)', bad.length ? bad.map(l => `${l.kind} '${l.text}' is ${l.cls}, wanted ${WANT[l.kind]}`).join(' · ') : `${net.length} net lines + ${L.length - net.length} spoken over ${Object.keys(WANT).length} kinds`);
  const texts = L.map(l => l.text);
  F.verdict(new Set(texts).size === texts.length && L.filter(l => !l.cls).every(l => l.text.length <= 12), 'no line is in the table twice; the spoken ones are a breath long', `${texts.length} lines, ${L.length - net.length} spoken`);
  const off = OLD.filter(([t, c]) => cueClass(t) !== c);
  F.verdict(!off.length, "the defenders' existing calls fall where the ranking puts them", off.length ? off.map(([t, c]) => `'${t}' → ${cueClass(t)}, wanted ${c}`).join(' · ') : `${OLD.length} lines`);
  return F.done();
}

async function rig(seed: number) {
  Math.random = seeded(seed);
  const log = console.log, err = console.error, warn = console.warn; let W: any;
  try { console.log = console.error = console.warn = () => { };
    W = await standUp({ uploadEvery: 4, level: loadLevel(parseLevelData(readFileSync(resolve(ROOT, 'levels', 'block-squads.json'), 'utf8'))) });
    await startFromQuery(W.game, new URLSearchParams(`?levelFile=block-squads.json&roomclear=${seed}&squad4`));
  } finally { console.log = log; console.error = err; console.warn = warn; }
  return W;
}
export async function cuesRun(): Promise<boolean> {
  const F = family('cues-run: a played floor — no cue is written on a man, the net and the silent decisions reach the strip, Sam down is told to the floor');
  const W = await rig(2), g = W.game; g.godMode = true;
  const cueTexts = new Set<string>(); let cueOnMan = 0; const strip = new Set<string>(); let hold = '';
  const frame = (_i: number) => {
    for (const r of stripRows(g) as { text: string }[]) strip.add(r.text);
    for (const l of g.cues.log) if (l.kind !== 'heard' && l.kind !== 'raw') cueTexts.add(l.text.replace(/^Tango \d+: /, ''));
    for (const gd of g.guards) { if (gd.bubble && cueTexts.has(gd.bubble.text)) cueOnMan++; if (gd.bubble && /hold where you are|back to your posts/.test(gd.bubble.text)) hold = gd.bubble.text; }
  };
  for (let i = 0; i < 60; i++) { g.player.visibility = 0.05; W.step(DT); frame(i); }
  // Sam is stood where the first squad's first posted man is aiming — read off the level's own marks, so a rebuilt floor moves the staging with it
  // (a fixed point in the old mess was out of every eye on the rebuilt block: nobody called anything and three lines went red)
  const rc = g.level.scenarios?.roomClear, aim = rc?.rooms?.find((r: any) => r.id === rc.squads?.[0]?.rooms?.[0])?.marks?.[0]?.aimAt;
  if (!aim) throw new Error('cues-run: block-squads.json has no first squad room with a mark to stage Sam in front of');
  // (THE TEAM IS TOLD TO HOLD WHERE IT STANDS before Sam goes. The page's squad opens following Sam: it walked after him
  // onto the squad's room, dropped the three men who had a line to that spot and lost two of its own, and with god mode off nobody was left to see
  // Sam: 38 s lit on the mark, not a round at him, never down, so the line his fall is owed was never said. The held team still fights from where it
  // stands; the case rests on the men with a line to Sam being left to it, and says the order's answer when he never falls.
  // CUESRUN_BREAK=1 = Sam's god mode is left on: he never falls. =follow = no hold: the team follows as the stand has it.)
  const heldWhy = g.orders?.active && process.env.CUESRUN_BREAK !== 'follow' ? g.orders.give('hold', g.orders.targetAt(g.player.char.pos), 'all') : 'not given';
  g.teleportPlayer([aim[0], 0, aim[2]]);
  for (let i = 60; i < 12 * 60; i++) { g.player.visibility = 1; W.step(DT); frame(i); }
  g.godMode = process.env.CUESRUN_BREAK === '1';   // …and now their rounds count: Sam drops where he stands, and the floor's fight is over
  for (let i = 12 * 60; i < 50 * 60; i++) { if (!g.player.down) g.player.visibility = 1; W.step(DT); frame(i); }
  const A = { strip, cueOnMan, stats: { ...g.cues.stats }, kinds: { ...g.cues.kinds }, hold, down: !!g.player.down, heldWhy: String(heldWhy ?? 'taken'), place: placeName(g.level, [2, 0, 2]) + ' / ' + roomName(g.level, [2, 0, 2]) };
  F.verdict(A.cueOnMan === 0, "no cue is ever written on a man (Guard.bubble is the bark clocks' to read)", `${A.cueOnMan} frames`);
  F.verdict([...A.strip].some(t => /^radio: Alpha: contact/.test(t)) && A.stats.heardDrawn >= 3, "the defenders' net reaches the strip", `${A.stats.heardDrawn} of ${A.stats.heard} lines drawn (${[...A.strip].slice(0, 3).join(' · ')}…)`);
  F.verdict(A.stats.cues >= 2 && A.stats.cueDrawn >= 1, 'silent decisions get a line', `${JSON.stringify(A.kinds)}, ${A.stats.cueDrawn} on the strip`);
  // (with Sam known down the floor's own words are owed: an empty string passed while his fall was not read)
  F.verdict(A.down && /hold where you are/.test(A.hold), 'Sam down: the floor is told to hold where it is', `${A.down ? '' : `STAGING: Sam never went down (the team's 'hold' was ${A.heldWhy}) · `}'${A.hold}'`);
  F.verdict(/^outside the [\w ]+ \/ the [\w ]+$/.test(A.place) && !/car park|back offices/.test(A.place), "a place on the net, and the dialogue's room noun, are the level's own", `'${A.place}'`);
  return F.done();
}
