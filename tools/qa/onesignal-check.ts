// TWO DOORS ON ONE WORD (src/game/doortools.ts goMen) and NO CAN INTO A ROOM OUR OWN MEN HOLD (src/game/doortools.ts mateBehind,
// DOORTOOLS.tossMate). The maintainer, 10-22, of a pair on each of the guardroom's two doors: "simultaneous breach on mission 2 didn't
// really work. and the second team threw a flashbang when our guys were already on the inside".
//   bun run tools/qa/onesignal-check.ts [--seeds 1,2]      (also `probes.ts onesignal`, in family squadorders)
//   TPS_PROFILE=shipped bun run tools/qa/onesignal-check.ts — the same lines on the page's stand
// The relay post (levels/tut-plan.json, a floor kept for the checks: no mission plays it), the room clear with the squad of four, everybody under
// god mode (the lines are about what is ORDERED and when, not who wins). The check raises the ring and the rule itself on either stand.
//   1. a pair stacked on each door of the guardroom, both standing on their marks, ONE press of the word: the two entries are on one word of the
//      door tools, the doors open within a second and the first men through the two doors are within a second, no teammate is dazzled. (Plan mode
//      laid these two stacks as chains and its own switch put them on one word; the mode is gone, and the word's key always did this for two
//      stacks: this line is what is left of that one, asked of the key itself.)
//   2. a bang and clear given to one pair, a man of the other pair inside the room by the time the can is to leave the hand: it stays in the kit, the
//      beat says who ('no bang · … is in there'), they go in, he is not dazzled;
//   3. CONTROL: the same order with nobody of ours inside throws its can as it did;
//   4. CONTROL (plumbing): a recording from before the key replays with it off, one made with it on replays on;
//   5. CONTROL (the stands): the frozen stand holds it off, the page's has it on, and FROZEN_HOLDS carries its row with its word.
// TOSSMATE_BREAK=1 = tossMate left off: line 2 must fail.   exit 0 = all hold · 1 = one does not
import { readFileSync } from 'node:fs';
import { ROOT, standUp } from './headless.ts';
const { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`);
const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
const { startFromQuery } = await import(`${ROOT}/src/game/scenarioRun.ts`);
const { RAILS } = await import(`${ROOT}/src/game/rails.ts`);
const { CMDRING } = await import(`${ROOT}/src/game/cmdring.ts`);
const { AUTOPILOT } = await import(`${ROOT}/src/game/autopilot.ts`);
const { DOORTOOLS, offOf, doorPt } = await import(`${ROOT}/src/game/doortools.ts`);
const { knobsCopy, knobsBack } = await import(`${ROOT}/src/game/knobs.ts`);
const VERBOSE = process.argv.includes('--verbose'), BREAK_MATE = process.env.TOSSMATE_BREAK === '1', LEVEL = 'tut-plan', DT = 1 / 60;
const SEEDS = (() => { const i = process.argv.indexOf('--seeds'); return i > 0 ? process.argv[i + 1].split(',').map(Number) : [1, 2]; })();
type V = number[];
let stood: { mate: boolean } | null = null;   // (each rule as the stand first stood it up, before the check raises it: the page's stand raises its switches once, at load)
const f2 = (n: number) => n.toFixed(2);

async function rig(seed: number, mate: boolean) {
  const log = console.log, warn = console.warn, err = console.error; if (!VERBOSE) console.log = console.warn = console.error = () => { };
  let W: any, started: any;
  try {
    const level = loadLevel(parseLevelData(readFileSync(`${ROOT}/levels/${LEVEL}.json`, 'utf8')));
    W = await standUp({ uploadEvery: 4, level, seed }); stood ??= { mate: DOORTOOLS.tossMate };
    // (the ring as the page has it)
    RAILS.on = false; Object.assign(CMDRING, { on: true, tapNow: true, stay: true }); AUTOPILOT.on = true;
    DOORTOOLS.tossMate = mate;
    started = startFromQuery(W.game, `?levelFile=${LEVEL}.json&roomclear=1&squad4`);
  } finally { console.log = log; console.warn = warn; console.error = err; }
  const g = W.game, O = g.orders, D = g.doorTools;
  const quiet = <T,>(f: () => T): T => { if (VERBOSE) return f(); const l = console.log, w = console.warn; console.log = console.warn = () => { }; try { return f(); } finally { console.log = l; console.warn = w; } };
  const R: any = { W, g, O, D, I: W.input, why: started?.why as string | undefined, post: null as null | (() => void),
    step(n = 1) { quiet(() => { for (let i = 0; i < n; i++) { g.player.visibility = 0.05; W.step(DT); R.post?.(); } }); },
    until(pred: () => boolean, secs: number) { for (let i = 0; i < secs * 60 && !pred(); i++) R.step(); return pred(); } };
  g.godMode = true; R.step(30); for (const m of O.squad()) m.god = true;
  const room = g.roomClear?.rooms.find((r: any) => r.def.id === 'guardroom'), B = room?.def.bounds, mid: V = B ? [(B.x0 + B.x1) / 2, 0, (B.z0 + B.z1) / 2] : [0, 0, 0];
  /** an order's target on that door, from the yard: the room is its far side */
  R.target = (id: string) => { const d = g.doors.list.find((x: any) => x.def.name === id); if (!d || !B) return null; const side = offOf(d, mid, 1) < 0 ? 1 : -1; return { kind: 'door', at: [d.frameCentre[0], 0, d.frameCentre[2]], door: d, side }; };
  return R;
}

interface OneRun { seed: number; why: string | null; kinds: string[]; worded: boolean; open: number[]; cross: number[]; bursts: number; dazzled: string[] }
/** line 1: a pair stacked on each door, both on their marks, one word */
async function playOne(seed: number): Promise<OneRun> {
  const R = await rig(seed, true), g = R.g, O = R.O, D = R.D, out: OneRun = { seed, why: null, kinds: [], worded: false, open: [], cross: [], bursts: 0, dazzled: [] };
  if (R.why) { out.why = R.why; return out; }
  const pairs: any[][] = [O.men('red'), O.men('blue')], ts = [R.target('guard'), R.target('guard_side')];
  if (pairs.some(p => p.length !== 2) || ts.some(t => !t)) { out.why = `no two pairs or no two doors (${pairs.map(p => p.length).join(' + ')} men)`; return out; }
  for (let i = 0; i < 2; i++) { O.give('stack', ts[i], undefined, { men: pairs[i] }); R.step(2); }
  const on = (i: number) => pairs[i].every((m: any) => O.of(m)?.order === 'stack' && O.of(m)?.door === ts[i].door && O.of(m)?.arrived);
  if (!R.until(() => on(0) && on(1), 60)) { out.why = `never both stacked (${[0, 1].map(i => pairs[i].map((m: any) => `${O.of(m)?.order ?? 'none'}${O.of(m)?.arrived ? '' : '…'}`).join(' ')).join(' | ')})`; return out; }
  R.step(30);
  const had = new Set<any>(D.ops), t0 = g.time, squad: any[] = O.squad(), fresh = () => D.ops.filter((o: any) => !had.has(o)), kept: any[] = [], crossT = new Map<any, number>(), dz = new Set<string>(), seen = new Set<any>(g.events);
  const words0 = D.stats.words; R.I.pressed.add(DOORTOOLS.blowKey); R.step();
  // (ONE word given, and both entries were told it — read off the tools' own count and each entry's beats: the word itself is taken off an entry as its
  //  men step in, the very tick it is given where nothing holds them a moment, as on the frozen stand)
  { const e = fresh().filter((o: any) => o.kind === 'openClear' || o.kind === 'bangClear'); out.worded = e.length === 2 && D.stats.words - words0 === 1 && e.every((o: any) => o.beats.some((b: any) => b.what === 'the word')); }
  R.post = () => { for (const o of fresh()) { if (!kept.includes(o)) kept.push(o);
      if (!crossT.has(o) && o.men.some((m: any) => m.state !== 'dead' && offOf(o.door, m.char.pos, o.side) < -0.15)) crossT.set(o, g.time - t0); }
    for (const e of g.events) { if (seen.has(e)) continue; seen.add(e); if (e.kind === 'bang') out.bursts++; }
    for (const m of squad) if (g.time < m.dazzledUntil) dz.add(m.callsign); };
  R.post(); R.until(() => kept.length >= 2 && kept.every(o => crossT.has(o)) && g.time - Math.max(...kept.map(o => crossT.get(o)! + t0)) > 3, 30); R.post = null;
  const ent = kept.filter(o => o.kind === 'openClear' || o.kind === 'bangClear');
  out.kinds = kept.map(o => `${o.kind} ${o.door.def.name}`);
  out.open = ent.map(o => o.openT - t0); out.cross = ent.map(o => crossT.has(o) ? crossT.get(o)! : NaN); out.dazzled = [...dz];
  return out;
}

interface MateRun { seed: number; why: string | null; who: string; beats: string[]; kit0: number; kit1: number; bursts: number; crossed: boolean; dazzled: boolean }
/** lines 2 and 3: a bang and clear for one pair on the front door; `inside`: a man of the other pair stood in the room once the door is open */
async function playMate(seed: number, inside: boolean): Promise<MateRun> {
  const R = await rig(seed, !BREAK_MATE), g = R.g, O = R.O, D = R.D, out: MateRun = { seed, why: null, who: '', beats: [], kit0: D.bangs, kit1: D.bangs, bursts: 0, crossed: false, dazzled: false };
  if (R.why) { out.why = R.why; return out; }
  const red: any[] = O.men('red'), mate: any = O.men('blue')[0], t = R.target('guard'); if (red.length !== 2 || !mate || !t) { out.why = 'no pair, no other man or no door'; return out; }
  const d = t.door, side = t.side, inn = doorPt(d, d.def.width / 2, 0.45, -side, 1.1), nav = g.col.nav; let at: V | null = null;
  for (const [deep, lat] of [[2.0, 0], [2.6, 0], [1.6, 0], [2.0, 0.8], [2.0, -0.8], [2.6, 1.0], [2.6, -1.0]]) { const p = doorPt(d, d.def.width / 2 + lat, deep, -side, 0);
    if (!nav.isBlocked(p[0], p[2]) && offOf(d, p, side) < -0.5 && !g.col.solidBlocked(inn, [p[0], 1.1, p[2]])) { at = [p[0], 0, p[2]]; break; } }
  if (!at) { out.why = 'no floor for him inside the room'; return out; }
  const op = D.start('bangClear', red, t, false); if (typeof op === 'string') { out.why = op; return out; }
  out.who = mate.callsign; out.kit0 = D.bangs + (op.bangHeld ? 1 : 0);
  const seen = new Set<any>(g.events); let crossT = NaN;
  R.post = () => { for (const e of g.events) { if (seen.has(e)) continue; seen.add(e); if (e.kind === 'bang') out.bursts++; }
    if (inside && !isNaN(op.openT)) { mate.char.pos[0] = at![0]; mate.char.pos[2] = at![2]; }   // (he went in by the room's other door since the row was given: there from the tick this door opens)
    if (g.time < mate.dazzledUntil) out.dazzled = true;
    if (isNaN(crossT) && op.men.some((m: any) => m.state !== 'dead' && offOf(d, m.char.pos, side) < -0.15)) crossT = g.time; };
  R.until(() => g.time - crossT >= 3, 60); R.post = null;
  out.beats = op.beats.map((b: any) => b.what); out.kit1 = D.bangs; out.crossed = crossT > 0;
  return out;
}

export async function onesignalCheck(): Promise<boolean> {
  let all = true; const keep = { on: RAILS.on, knobs: knobsCopy(), ap: AUTOPILOT.on };
  const verdict = (ok: boolean, label: string, detail: string) => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); };
  console.log(`two doors on one word, and no can into a room our own men hold (levels/${LEVEL}.json, the guardroom; seeds ${SEEDS.join(', ')})${BREAK_MATE ? ' — TOSSMATE_BREAK=1: line 2 must fail' : ''}`);
  try {
    // ---- 1
    { const runs: OneRun[] = []; for (const s of SEEDS) runs.push(await playOne(s));
      const gap = (a: number[]) => Math.abs(a[0] - a[1]);
      const ok = (r: OneRun) => !r.why && r.worded && r.open.length === 2 && r.kinds.filter(k => k.startsWith('openClear')).length === 2 && gap(r.open) <= 1.0 && r.cross.every(c => c >= 0) && gap(r.cross) <= 1.0 && r.bursts === 0 && !r.dazzled.length;
      verdict(runs.every(ok), 'ONE WORD, TWO DOORS: a pair stacked on each door of the guardroom, both on their marks, the word pressed once — two plain entries on one word, the doors open within a second, the first men through the two doors are within a second of each other, no can is thrown and nobody of ours is dazzled',
        runs.map(r => r.why ? `seed ${r.seed}: NOT RUN — ${r.why}` : `seed ${r.seed}: ${r.kinds.join(' + ') || 'no entry'} · ${r.worded ? 'one word' : 'NO shared word'} · doors open ${r.open.map(f2).join(' and ')} s after the press · first man through at ${r.cross.map(f2).join(' and ')} s (${f2(gap(r.cross))} s apart) · bursts ${r.bursts} · dazzled of ours: ${r.dazzled.join(', ') || 'nobody'}`).join('\n    ')); }
    // ---- 2 and 3
    { const ins: MateRun[] = [], outs: MateRun[] = []; for (const s of SEEDS) { ins.push(await playMate(s, true)); outs.push(await playMate(s, false)); }
      const say = (r: MateRun, inside: boolean) => r.why ? `seed ${r.seed}: NOT RUN — ${r.why}` : `seed ${r.seed}: ${inside ? `${r.who} on the room's floor as the door opens` : 'nobody of ours inside'} · ${r.beats.filter(b => /^(shove|kick|open|toss:|bang|no bang|go)/.test(b)).join(' → ') || 'no beat'} · kit ${r.kit0} → ${r.kit1}, ${r.bursts} burst · ${r.crossed ? 'they went in' : 'NOBODY went in'}${inside ? ` · ${r.who} ${r.dazzled ? 'DAZZLED' : 'not dazzled'}` : ''}`;
      verdict(ins.every(r => !r.why && r.beats.includes(`no bang · ${r.who} is in there`) && !r.beats.some(b => /^toss:/.test(b)) && r.kit1 === r.kit0 && r.bursts === 0 && r.crossed && !r.dazzled),
        `A TEAMMATE INSIDE BY THE TIME THE CAN IS TO LEAVE THE HAND (DOORTOOLS.tossMate${BREAK_MATE ? ' — TOSSMATE_BREAK: left off' : ''}): the can stays in the kit, the beat says who ('no bang · … is in there'), nothing bursts, they go in, he is not dazzled`, ins.map(r => say(r, true)).join('\n    '));
      verdict(outs.every(r => !r.why && r.beats.some(b => /^toss:/.test(b)) && r.kit1 === r.kit0 - 1 && r.bursts === 1 && r.crossed),
        `CONTROL (the rule changes nothing here: holds${BREAK_MATE ? ', as now,' : ''} under TOSSMATE_BREAK too) — nobody of ours inside: the same order throws its can, one burst, then they go in`, outs.map(r => say(r, false)).join('\n    ')); }
    // ---- 4
    { const J: any = await import(`${ROOT}/src/game/journal.ts`), g = (await rig(SEEDS[0], true)).g, snap = () => JSON.parse(JSON.stringify(J.knobsSnapshot(g)));
      DOORTOOLS.tossMate = true; const old = snap(); delete old.flags.tossMate; if (old.tables.DOORTOOLS) delete old.tables.DOORTOOLS.tossMate; J.applyKnobs(g, old); const before = DOORTOOLS.tossMate;
      DOORTOOLS.tossMate = true; const on = snap(); DOORTOOLS.tossMate = false; J.applyKnobs(g, on); const kept = DOORTOOLS.tossMate;
      DOORTOOLS.tossMate = false; const off = snap(); DOORTOOLS.tossMate = true; J.applyKnobs(g, off); const keptOff = DOORTOOLS.tossMate;
      verdict(before === false && kept === true && keptOff === false, 'CONTROL (plumbing, not the rule: no break word touches it) — a recording from before the key replays with DOORTOOLS.tossMate off; one made with it on replays on, off off',
        `from before the key: tossMate ${before} · recorded on: ${kept} · recorded off: ${keptOff}`); }
    // ---- 5
    { const { FROZEN_HOLDS } = await import(`${ROOT}/src/host/profile.ts`), shipped = process.env.TPS_PROFILE === 'shipped', want = (w: string | undefined) => w !== undefined ? w === '1' : shipped;
      const row = FROZEN_HOLDS['DOORTOOLS.tossMate'];
      verdict(!!stood && stood.mate === want(process.env.TPS_TOSSMATE) && !!row?.includes('TPS_TOSSMATE=1'), `CONTROL (the stands) — this stand (${shipped ? 'the page\'s' : 'frozen'}) stands the rule up ${shipped ? 'on' : 'off'} unless a word says otherwise, and FROZEN_HOLDS carries its row`,
        `tossMate ${stood?.mate} · the row: '${row}'`); }
  } finally { RAILS.on = keep.on; knobsBack(keep.knobs); AUTOPILOT.on = keep.ap; }
  console.log(all ? 'onesignal: all hold' : 'onesignal: FAIL');
  return all;
}
if (import.meta.main) process.exit((await onesignalCheck()) ? 0 : 1);
