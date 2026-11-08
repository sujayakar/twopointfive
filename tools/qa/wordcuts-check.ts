// tools/qa/wordcuts-check.ts — the word that spends no can waits for the cuts (doortools.ts DOORTOOLS.wordCuts: cutsOwe, wordStepIn, wordOwed).
//
// Seen in a lesson's sixteen runs on the page's stand: the kitchen of the orders lesson held from outside, its one defender awake, aimed at the door and
// stood in the band neither jamb's diagonal shows. The word (F) a second and a half into the hold, while both jamb men were still slicing the doorway,
// walked them off their cuts to the door's middle at once, and they were shot in its width before either had seen him, on ten of sixteen. Left to finish
// the cuts the same pair finds him and drops him from the frame. With the rule the word given early is owed to the end of the cuts.
//
// The scene is that lesson's yard with no lesson (levels/depot.json stood up as a room-clear floor, LESSON off; nothing of a scripted player):
// every defender but the kitchen's taken off the floor as the order is given, the kitchen's own man put on alert and pinned on his mark, with the door as what
// he knows, as the pair sets on the jambs (the fight for the other houses had him so in the lesson), the pair told to hold the kitchen's door from outside by code
// (doorTools.thresholdOn(door, 'all', side, true)), then the word (doorTools.go) WAIT_S after they are on the jambs, and the door watched for twelve
// seconds.
//   (1) the word is taken and waits ('the word waits for the cuts'), the men finish, 'the cuts are done · in', and only then the step in; no can thrown;
//       nobody of the pair lost;
//   (2) the word AGAIN a second later: in at once, the cuts not waited for (the player's say stands);
//   (3) the cap at half a second: in when the cap is spent, said as that.
// THE RULE IS SETTLED AND NOT WRITTEN HERE (2026-11-07). Out: the stand's row, 'rule off: they step in there and then' and 'no more lost than with the rule
// off' (the same seeds played with the rule off), 'off, nothing reads its number', the recording from before the key and ?wordcuts=0; and two things that
// were refused or had no job left: LESSON.on written off for the yard, and the frozen stand's four words raised for the runs.
//
//   bun run tools/qa/wordcuts-check.ts            (also `probes.ts word-cuts`, family doortools)
//   WORDCUTS_BREAK=1 …                            (the game's own hook: the rule's body left out, the same lines asked: must fail)
// What it cannot say: one door, one room, three seeds, the order given by code, the defender woken and pinned by hand; who wins at the frame is the fight's dice and is counted, not promised. Not
// staged: a captor's last warning, two doors on one word, a hold whose men do not slice. Never seen on a GPU, nobody has played it.
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { standUp, ROOT } from './headless.ts';
import { guarded, family } from './thresholdmenu-probes.ts';
const { DOORTOOLS } = (await import(`${ROOT}/src/game/doortools.ts`)) as any;
const { killGuard } = (await import(`${ROOT}/src/game/combat.ts`)) as any;
const { startFromQuery } = (await import(`${ROOT}/src/game/scenarioRun.ts`)) as any;
const { parseLevelData } = (await import(`${ROOT}/src/scene/levelData.ts`)) as any;
const { loadLevel } = (await import(`${ROOT}/src/scene/levelLoad.ts`)) as any;
const { knobsCopy, knobsBack } = (await import(`${ROOT}/src/game/knobs.ts`)) as any;

const DT = 1 / 60, WAIT_S = 1.5, WATCH_S = 12, SEEDS = (process.env.WORDCUTS_SEEDS ?? '2,3,5').split(',').map(Number), YARD = 'depot.json', DOOR = 'b_n';
const KITCHEN = { x0: 24.12, x1: 29.38, z0: 9.62, z1: 14.88 };   // (the comms room: one door)
function seeded(a: number) { return () => { a |= 0; a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ (a >>> 15), 1 | a); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; }; }
async function quiet<T>(fn: () => T | Promise<T>): Promise<T> { const log = console.log, err = console.error, warn = console.warn; console.log = console.error = console.warn = () => { }; try { return await fn(); } finally { console.log = log; console.error = err; console.warn = warn; } }

interface Run { why: string; sliced0: boolean; foeWas: string; beats: { t: number; what: string }[]; lost: number; foeDown: number; cans: number }
const at = (R: Run, re: RegExp, nth = 0) => R.beats.filter(b => re.test(b.what))[nth]?.t ?? NaN;

/** the yard stood up, the hold given, then the word `WAIT_S` after the men are on the jambs (and again `again` seconds after, if given), the door watched */
async function play(seed: number, o: { again?: number; cap?: number } = {}): Promise<Run> {
  const R: Run = { why: '', sliced0: false, foeWas: '', beats: [], lost: 0, foeDown: NaN, cans: 0 };
  Math.random = seeded(0x9137 + seed * 7919);
  const W: any = await quiet(() => standUp({ uploadEvery: 4, level: loadLevel(parseLevelData(readFileSync(resolve(ROOT, 'levels', YARD), 'utf8'))) })), g = W.game;
  if (o.cap !== undefined) DOORTOOLS.wordCutsCap = o.cap;
  g.godMode = true; g.tune.chaos = 0.5;
  const { why } = await quiet(() => startFromQuery(g, new URLSearchParams(`roomclear=${seed}`))); if (why) { R.why = `start refused: ${why}`; return R; }
  const step = (n = 1) => { for (let i = 0; i < n; i++) { g.player.visibility = 0.05; W.step(DT); } }, D = g.doorTools;
  await quiet(() => step(20));
  const inK = (x: any) => x.char.pos[0] > KITCHEN.x0 && x.char.pos[0] < KITCHEN.x1 && x.char.pos[2] > KITCHEN.z0 && x.char.pos[2] < KITCHEN.z1;
  const op0 = g.doors.list.find((d: any) => d.def.name === DOOR); if (!op0) { R.why = `no door '${DOOR}'`; return R; }
  const foe: any[] = g.guards.filter((x: any) => x.team === 'hostile' && x.state !== 'dead' && inK(x)); if (!foe.length) { R.why = 'nobody alive in the kitchen'; return R; }
  for (const h of g.guards) if (h.team === 'hostile' && h.state !== 'dead' && !foe.includes(h)) killGuard(g, h, [0, 0, 1], true, 'struck');   // (the other houses' men off the floor in the frame the order is given: nobody has moved)
  const op: any = D.thresholdOn(DOOR, 'all', undefined, true); if (typeof op === 'string') { R.why = `the hold refused: ${op}`; return R; }
  await quiet(() => { for (let k = 0; k < 60 * 60 && op.phase !== 'thr' && D.ops.includes(op); k++) step(); }); if (op.phase !== 'thr') { R.why = `the pair never held the kitchen's door (the order: ${D.ops.includes(op) ? op.phase : 'gone'})`; return R; }
  for (const e of foe) { e.state = 'alert'; e.awareness = 1; e.pinned = true; e.lastKnown = [op0.frameCentre[0], 0, op0.frameCentre[2]]; }   // (awake, on his mark and on his door from the moment they are set on the jambs, as the fight for the other houses had him in the lesson: pinned, so an alert man does not walk out to the door he knows of)
  await quiet(() => step(Math.round(WAIT_S * 60)));
  const men: any[] = [...op.men], cans = D.bangs, nb = op.beats.length, t0 = g.time; R.sliced0 = !!op.thr.sliced; R.foeWas = foe.map((e: any) => e.state).join('/');
  let againT = o.again !== undefined ? t0 + o.again : Infinity;
  await quiet(() => { W.input.pressed.add('KeyF');
    for (let k = 0; k < (WATCH_S + 1) * 60; k++) { step(); const t = g.time - t0;
      if (g.time >= againT) { againT = Infinity; W.input.pressed.add('KeyF'); }
      if (Number.isNaN(R.foeDown) && foe.every((e: any) => e.state === 'dead')) R.foeDown = t;
      if (t > WATCH_S || men.every((m: any) => m.state === 'dead')) break; } });
  R.beats = op.beats.slice(nb).map((b: any) => ({ t: b.t - t0, what: b.what })); R.lost = men.filter((m: any) => m.state === 'dead').length; R.cans = cans - D.bangs;
  return R;
}

export const wordCutsCheck = guarded(async () => {
  const F = family('word-cuts: the word that spends no can waits for the cuts');
  const knobs0 = knobsCopy(), rnd0 = Math.random;
  try {
    const f = (n: number) => Number.isNaN(n) ? '—' : n.toFixed(2);
    const say = (s: number, R: Run) => `seed ${s}: ${R.why || `his man '${R.foeWas}' at the word · the word +${f(at(R, /^the word$/))} · waits +${f(at(R, /^the word waits for the cuts$/))} · cuts over +${f(at(R, /^the cuts (are done|not done)/))} · held counted +${f(at(R, /^held /))} · his man down +${f(R.foeDown)} · ${R.lost} of the pair lost · ${R.cans} can(s)`}`;
    const on: Run[] = []; for (const s of SEEDS) on.push(await play(s));
    F.verdict(on.every(R => !R.why && !R.sliced0 && at(R, /^the word waits for the cuts$/) - at(R, /^the word$/) < 0.1 && at(R, /^the cuts are done · in$/) > at(R, /^the word waits/) + 0.3 && at(R, /^held /) >= at(R, /^the cuts are done · in$/) && R.cans === 0 && R.lost === 0),
      `the word ${WAIT_S} s into the hold (the men still slicing): it is taken and waits, the men finish their cuts, then the step in; no can; nobody of the pair lost`,
      on.map((R, i) => say(SEEDS[i], R)).join('\n    '));
    { const R = await play(SEEDS[0], { again: 1 });
      F.verdict(!R.why && at(R, /^the word waits/) < 0.1 && Math.abs(at(R, /^the word$/, 1) - 1) < 0.15 && at(R, /^held /) - at(R, /^the word$/, 1) < 0.1 && Number.isNaN(at(R, /^the cuts (are done|not done)/)),
        'the word again a second later: in at once, the cuts not waited for', `${say(SEEDS[0], R)} · the second word +${f(at(R, /^the word$/, 1))}`); }
    { const R = await play(SEEDS[0], { cap: 0.5 });
      F.verdict(!R.why && at(R, /^the word waits/) < 0.1 && Math.abs(at(R, /^the cuts not done in 0\.5 s · in$/) - 0.5) < 0.15 && at(R, /^held /) >= at(R, /^the cuts not done/),
        'the cap at half a second: in when the cap is spent, said as that', say(SEEDS[0], R)); }
  } finally { knobsBack(knobs0); Math.random = rnd0; }
  return F.done();
});

if (import.meta.main) process.exit((await wordCutsCheck()) ? 0 : 1);
