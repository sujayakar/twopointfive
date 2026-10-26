// QUICK ORDERS ARE THE WHOLE SQUAD'S (src/game/orders.ts whole() / input / ringInput / targetUnder, CMDRING.wholeSquad). The
// maintainer, 10-24: "quick Q mode should always do the whole squad. you should only be able to issue commands to parts of the squad in plan mode".
// (Plan mode is gone from the game since: a part of the squad is Shift + the key's, goals-check.ts. The lines that picked in the mode went with it.)
//   bun run tools/qa/wholesquad-check.ts            (also `probes.ts wholesquad`, in family squadorders)
//   TPS_PROFILE=shipped bun run tools/qa/wholesquad-check.ts      — the same lines on the page's stand
// PASS / FAIL each, headless: the room clear on the office with its squad of four under god mode, ONE deal (seed 3: the rule has no die in it), the
// ring up while its key is down, with the switch raised here, so both stands ask the same lines.
//   1. a choice left on a pair is everyone's again on the next frame, and the key tapped goes to all four.
//   2. a click on a man with the ring up picks nobody (the ring that stays, as the page has it): the orders are still everyone's.
//   3. the who key steps nobody, on a man or off him.
//   4. the key tapped with the cursor on a teammate is not his alone: the order goes to all four.
//   5. CONTROL — the switch off: the click on a man makes the orders his, and the tap goes to him alone.
//   6. CONTROL (plumbing) — a recording from before the key replays with it off; the frozen stand holds it off, the page's has it on, FROZEN_HOLDS has its row.
// WHOLESQUAD_BREAK=1 = the switch left off and the same lines asked: 1 to 4 must fail.
//   exit 0 = all hold · 1 = one does not
import { standUp, ROOT } from './headless.ts';

const { CMDRING } = await import(`${ROOT}/src/game/cmdring.ts`);
const { ORDERS } = await import(`${ROOT}/src/game/orders.ts`);
const { knobsCopy, knobsBack } = await import(`${ROOT}/src/game/knobs.ts`);
const { applyKnobs, knobsSnapshot } = await import(`${ROOT}/src/game/journal.ts`);
const { FROZEN_HOLDS } = await import(`${ROOT}/src/host/profile.ts`);
const { NIGHTBREACH } = await import(`${ROOT}/src/game/nightbreach.ts`);
const { AUTOPILOT } = await import(`${ROOT}/src/game/autopilot.ts`);
const BREAK = process.env.WHOLESQUAD_BREAK === '1';
const DT = 1 / 60, dist = (a: number[], b: number[]) => Math.hypot(a[0] - b[0], a[2] - b[2]);
function seeded(a: number) { return () => { a |= 0; a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ (a >>> 15), 1 | a); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; }; }
/** the room clear on the office with its squad of four: the ring that goes with its key, Shift + the key held off (goals-check.ts has that pair) */
async function rig(seed: number) {
  const squad0 = NIGHTBREACH.squad; let W: any;
  try {
    Object.assign(NIGHTBREACH, { squad: 4 });
    Math.random = seeded(seed);
    W = await standUp();
    Object.assign(CMDRING, { on: true, hint: true, hands: true, stay: false, tapNow: false, only: false, goals: false, goalRing: false }); AUTOPILOT.on = true;
    W.game.godMode = true;
    W.game.roomClear.start(seed);
  } finally { Object.assign(NIGHTBREACH, { squad: squad0 }); }
  const game = W.game, O = game.orders, input = W.input;
  const R: any = { W, game, O, input,
    get team(): any[] { return O.squad(); },
    step(n = 1) { for (let i = 0; i < n; i++) { game.player.visibility = 0.05; W.step(DT); } },
    until(pred: () => boolean, secs: number) { for (let i = 0; i < secs * 60 && !pred(); i++) R.step(); return pred(); },
    aim(p: number[]) { W.cursorAt([p[0], p[1], p[2]]); R.step(2); return O.hover?.() ?? null; },
    press(code: string) { input.pressed.add(code); R.step(); },
    /** a left click on a man (his chest: the ray meets his own body) */
    clickMan(m: any) { W.cursorAt([m.char.pos[0], 1.0, m.char.pos[2]]); R.step(); input.clicked |= 1; input.buttons |= 1; R.step(); input.buttons &= ~1; R.step(); },
    tail() { return O.log[O.log.length - 1]; },
    /** everything put away and the men back round Sam */
    home() { input.keys.clear(); input.buttons = 0; O.cmdRing = null; O.close(); O.who = 'all';
      O.give('fallIn', O.targetAt(game.player.char.pos), 'all'); R.until(() => game.doorTools.ops.length === 0, 40); R.until(() => R.team.every((m: any) => dist(m.char.pos, game.player.char.pos) < 4.5), 25); R.step(30); },
  };
  R.step(30);
  for (const m of R.team) m.god = true;
  return R;
}
function floorSpot(R: any): number[] | null {
  const s = R.game.player.char.pos;
  for (const [dx, dz] of [[2.5, 0], [-2.5, 0], [0, 2.5], [0, -2.5], [2, 2], [-2, -2], [2, -2], [-2, 2], [3.5, 0], [0, 3.5], [-3.5, 0], [0, -3.5]]) { const p = [s[0] + dx, 0.04, s[2] + dz];
    const h = R.aim(p); if (h?.kind === 'floor' && !h.wall && !h.thing && R.team.every((m: any) => dist(m.char.pos, p) > 1.2) && R.O.defaultFor(h)?.id === 'move' && R.O.whyNot(R.O.defaultFor(h), h) === null) return p; }
  return null;
}
/** the stand's own word for the switch, read before any rig holds it */
let stood: boolean | null = null;

export async function wholesquadCheck(): Promise<boolean> {
  let fails = 0;
  const line = (ok: boolean, what: string, got: string) => { console.log(`  ${ok ? 'PASS' : 'FAIL'}  ${what} — ${got}`); if (!ok) fails++; };
  const knobs0 = knobsCopy(), rnd0 = Math.random, K = CMDRING;
  console.log(`quick orders are the whole squad's (src/game/orders.ts whole(), CMDRING.wholeSquad)${BREAK ? ' — WHOLESQUAD_BREAK=1: the switch left off, lines 1 to 4 must fail' : ''}`);
  try {
    { await standUp(); stood = !!K.wholeSquad; }
    const R: any = await rig(3), O = R.O, g = R.game, input = R.input, KEY = K.key, T = (): any[] => R.team, N = T().length;
    const name = (w: any): string => typeof w === 'number' ? O.squad()[w]?.callsign ?? `man ${w}` : String(w);
    /** the key tapped where the cursor is: who took the order it gave (nobody: none was given) */
    const tap = (): string[] => { const n0 = O.log.length; input.keys.add(KEY); input.pressed.add(KEY); R.step(2); input.keys.delete(KEY); R.step(12); const row = R.tail();
      const who = O.log.length > n0 && row ? (row.who ?? []).map(String) : []; O.cmdRing = null; O.close(); return who; };
    const onFloor = () => { const s = floorSpot(R); if (s) R.aim(s); return !!s; };
    const chest = (m: any) => { R.W.cursorAt([m.char.pos[0], 1.0, m.char.pos[2]]); R.step(); };
    const set = (on: boolean) => { K.wholeSquad = on; };

    // 1
    set(!BREAK); R.home();
    { O.who = 'blue'; R.step(); const next = O.who, ok = onFloor(), took = tap();
      line(ok && next === 'all' && took.length === N && N === 4, '1. a choice left on a pair is everyone\'s on the next frame, and the key tapped goes to all four',
        `left on blue: ${name(next)} a frame later · the tap went to ${took.length} of ${N} (${took.join(', ') || 'nobody'})`); }
    // 2
    R.home();
    { const stay0 = K.stay; K.stay = true; onFloor(); input.keys.add(KEY); input.pressed.add(KEY); R.until(() => !!O.cmdRing, 2); input.keys.delete(KEY); R.step(6); const up = !!O.cmdRing;
      const n0 = O.log.length; R.clickMan(T()[2]); R.step(6); const who = O.who, row = O.log.length > n0 ? R.tail() : null, took = row ? (row.who ?? []).length : -1;
      O.cmdRing = null; O.close(); K.stay = stay0;
      line(up && who === 'all' && (took < 0 || took === N), '2. a click on a man with the ring up picks nobody: the orders are still everyone\'s',
        `the ring up: ${up} · a click on ${T()[2].callsign}: orders to ${name(who)} · ${took < 0 ? 'no order given by it' : `the click gave ${row.order} to ${took} of ${N}`}`); }
    // 3
    R.home();
    { chest(T()[1]); R.press(ORDERS.whoKey); R.step(2); const onMan = O.who; onFloor(); R.press(ORDERS.whoKey); R.step(2); const off = O.who;
      line(onMan === 'all' && off === 'all', '3. the who key steps nobody, on a man or off him', `pressed on ${T()[1].callsign}: ${name(onMan)} · pressed on the floor: ${name(off)}`); }
    // 4
    R.home();
    { chest(T()[2]); const h = O.hover?.(), took = tap();
      line(!(h?.kind === 'none' && h.man) && took.length === N, '4. the key tapped with the cursor on a teammate is not his alone: the order goes to all four',
        `the cursor on ${T()[2].callsign} reads as ${h?.kind === 'none' ? (h.man ? 'that man alone' : 'the squad itself') : h?.kind ?? 'nothing'} · the tap went to ${took.length} of ${N}`); }
    // 5
    set(false); R.home();
    { const stay0 = K.stay; K.stay = true; onFloor(); input.keys.add(KEY); input.pressed.add(KEY); R.until(() => !!O.cmdRing, 2); input.keys.delete(KEY); R.step(6);
      R.clickMan(T()[2]); R.step(6); const his = O.who; O.cmdRing = null; O.close(); K.stay = stay0; onFloor(); const took = tap();
      line(typeof his === 'number' && took.length === 1, '5. CONTROL — the switch off: the click on a man makes the orders his, and the tap goes to him alone',
        `a click on a man with the ring up: orders to ${name(his)} · the tap went to ${took.length} of ${N} (${took.join(', ')})`); }
    R.home();
    // 6
    { K.wholeSquad = true; const snap = JSON.parse(JSON.stringify(knobsSnapshot(g))), old = JSON.parse(JSON.stringify(snap)); delete old.tables.CMDRING.wholeSquad;
      applyKnobs(g, old); const before = K.wholeSquad; applyKnobs(g, snap); const on = K.wholeSquad;
      const shipped = String(process.env.TPS_PROFILE ?? '') === 'shipped', row = FROZEN_HOLDS['CMDRING.wholeSquad'];
      line(before === false && on === true && stood === shipped && /TPS_WHOLESQUAD=1/.test(row ?? ''), '6. CONTROL (plumbing) — a recording from before the key replays with it off; the stands and the FROZEN_HOLDS row',
        `a file without the key: ${before} · one with it on: ${on} · this stand (${shipped ? 'the page\'s' : 'frozen'}) stood it ${stood} · the row: ${row ?? 'none'}`); }
  } finally { knobsBack(knobs0); Math.random = rnd0; }

  console.log(fails ? `=== wholesquad-check: ${fails} FAIL ===` : '=== wholesquad-check: ALL PASS ===');
  return !fails;
}

if (import.meta.main) process.exit((await wholesquadCheck()) ? 0 : 1);
