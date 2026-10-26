// THE KEY NEVER PICKS A TOOL (src/game/autopilot.ts autoEntries, AUTOPILOT.plain). The maintainer, 10-26: "let's also remove the Q vs. F for deciding
// how to go through a room. F always just does a basic clear" — and a tool is asked for on the ring ("show how to tell the squad to use it with
// long press Q").
//   bun run tools/qa/plainentry-check.ts            (also `probes.ts plainentry`, in family squadorders)
//   TPS_PROFILE=shipped bun run tools/qa/plainentry-check.ts      — the same lines on the page's stand
// PASS / FAIL each, headless: the room clear on the office with its squad of four under god mode, ONE deal (seed 5, autopilot-check's: the rule has no
// die in it). The check raises the quick key's autopilot and the rule itself, so both stands ask the same lines. Every press is a one-frame press of
// the quick key, or of the word's key, with the harness cursor on the door.
//   1. stacked and set on a door with a room behind it nobody has stood in, three stuns carried: the squad's pick is the plain open and clear, and the
//      HUD's line for the key says so (with the rule off this is a stun: line 5).
//   2. the key on that door: they go in by open and clear, all four, and no stun is spent.
//   3. stacked there again, the WORD's key: the same open and clear, all four, no stun spent — the two keys give one entry.
//   4. stacked there again, 'bang and clear' given as the ring's row gives it: that entry starts — a tool is still theirs when it is asked for.
//   5. CONTROL — the rule off, the same stack: the pick is a stun for the room nobody has stood in, as it was.
//   6. CONTROL (plumbing) — a recording from before the key replays with it off; the frozen stand holds it off, the page's has it on, FROZEN_HOLDS has its row.
// PLAINENTRY_BREAK=1 = the switch left off and the same lines asked: 1 and 2 must fail.
//   exit 0 = all hold · 1 = one does not
import { standUp, ROOT } from './headless.ts';
const AP: any = await import(`${ROOT}/src/game/autopilot.ts`);
const { NIGHTBREACH } = await import(`${ROOT}/src/game/nightbreach.ts`);
const { ORDERS } = await import(`${ROOT}/src/game/orders.ts`);
const { DOORTOOLS } = await import(`${ROOT}/src/game/doortools.ts`);
const { knobsCopy, knobsBack } = await import(`${ROOT}/src/game/knobs.ts`);
const { applyKnobs, knobsSnapshot } = await import(`${ROOT}/src/game/journal.ts`);
const { FROZEN_HOLDS } = await import(`${ROOT}/src/host/profile.ts`);
const { AUTOPILOT, autoPick, autoHint } = AP;
const DT = 1 / 60, SEED = 5, BREAK = process.env.PLAINENTRY_BREAK === '1', VERBOSE = process.argv.includes('--verbose');
function seeded(a: number) { return () => { a |= 0; a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ (a >>> 15), 1 | a); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; }; }
const dist = (a: number[], b: number[]) => Math.hypot(a[0] - b[0], a[2] - b[2]);

export async function plainEntryCheck(): Promise<boolean> {
  let fails = 0;
  const line = (ok: boolean, what: string, got: string) => { console.log(`  ${ok ? 'PASS' : 'FAIL'}  ${what} — ${got}`); if (!ok) fails++; };
  const knobs0 = knobsCopy(), rnd0 = Math.random, squad0 = NIGHTBREACH.squad;
  console.log(`The key never picks a tool (src/game/autopilot.ts autoEntries, AUTOPILOT.plain)${BREAK ? ' — PLAINENTRY_BREAK=1: the switch left off, lines 1 and 2 must fail' : ''}`);
  try {
    const log = console.log, warn = console.warn; if (!VERBOSE) console.log = console.warn = () => { };
    let W: any, why: any, stood = false;
    try { Object.assign(NIGHTBREACH, { squad: 4 }); Math.random = seeded(SEED); W = await standUp(); stood = !!AUTOPILOT.plain; AUTOPILOT.on = true; AUTOPILOT.plain = !BREAK; W.game.godMode = true; why = W.game.roomClear.start(SEED); }
    finally { console.log = log; console.warn = warn; Object.assign(NIGHTBREACH, { squad: squad0 }); }
    const g = W.game, O = g.orders, T = g.doorTools, I = W.input;
    const quiet = <X,>(f: () => X): X => { if (VERBOSE) return f(); const l = console.log, w = console.warn; console.log = console.warn = () => { }; try { return f(); } finally { console.log = l; console.warn = w; } };
    const step = (n = 1) => quiet(() => { for (let i = 0; i < n; i++) { g.player.visibility = 0.05; W.step(DT); } });
    const until = (pred: () => boolean, secs: number) => { for (let i = 0; i < secs * 60 && !pred(); i++) step(); return pred(); };
    step(30); for (const m of O.squad()) m.god = true;
    const all = () => O.men('all'), N = all().length, stackDef = O.defs.find((d: any) => d.id === 'stack');
    if (why || N !== 4) { line(false, 'the room clear on the office with its squad of four', String(why ?? `${N} men`)); return false; }
    /** a point of that door's doorway the cursor on which IS that door (null: none) */
    const aimOf = (d: any): number[] | null => { const c = d.frameCentre, th = d.def.closedDir, nx = -Math.sin(th), nz = Math.cos(th);
      for (const y of [c[1], 0.05, 1.0, 1.8]) for (const k of [0, 0.35, -0.35]) { const p = [c[0] + nx * k, y, c[2] + nz * k]; W.cursorAt(p); step(2); const h = O.hover?.(); if (h?.kind === 'door' && h.door === d) return p; }
      return null; };
    // the door: the nearest whose far room nobody has stood in, that the four can stack on and the cursor can be put on
    const s = g.player.char.pos; let D: any = null, t: any = null;
    for (const d of [...g.doors.list].sort((a: any, b: any) => dist(a.frameCentre, s) - dist(b.frameCentre, s))) { const x = O.targetAt(d.frameCentre); if (x.kind !== 'door' || x.door !== d) continue;
      const far = g.cleared.roomBy(d, x.side > 0 ? -1 : 1); if (!far || g.cleared.isKnown(far)) continue; if (O.whyNot(stackDef, O.forMen(x, all()), all()) || !aimOf(d)) continue; D = d; t = x; break; }
    if (!D) { line(false, 'a door to stack on with a room behind it nobody has stood in', 'none'); return false; }
    const stacked = () => all().every((m: any) => O.of(m)?.order === 'stack' && O.of(m)?.door === D && O.of(m)?.arrived);
    /** all four stacked and set on the door, whatever went before (true: they are) */
    const stack = (): boolean => { until(() => T.ops.length === 0, 60); O.give('fallIn', O.targetAt(g.player.char.pos), 'all'); step(60); O.give('stack', O.forMen(O.targetAt(D.frameCentre), all()), 'all'); return until(stacked, 40) && (step(20), stacked()); };
    const opOn = () => T.ops.find((o: any) => o.door === D), far = () => g.cleared.roomBy(D, t.side > 0 ? -1 : 1), unknown = () => !!far() && !g.cleared.isKnown(far());

    // 1
    T.bangs = 3; const set1 = stack(), fresh1 = unknown(), pick = autoPick(g, O.targetAt(D.frameCentre)), hint = autoHint(g, O.targetAt(D.frameCentre)) ?? '';
    line(set1 && fresh1 && pick.id === 'openClear' && pick.men?.length === N && !/bang|stun/i.test(hint) && /clear/i.test(hint), '1. stacked on a door with a room nobody has stood in behind it, three stuns: their pick is the plain open and clear, and the HUD says so',
      `stacked and set: ${set1} · the room unknown: ${fresh1} · pick ${pick.id} (${pick.why}) for ${pick.men?.length ?? 0} · the HUD's line: '${hint}'`);
    // 2
    { const b0 = T.bangs, n0 = O.log.length, on = !!aimOf(D); I.pressed.add(ORDERS.quickKey); step(3); const row = O.log[O.log.length - 1], op = opOn();
      line(on && O.log.length === n0 + 1 && row?.order === 'openClear' && row.auto === true && row.who.length === N && op?.kind === 'openClear' && T.bangs === b0, '2. the key on that door: they go in by open and clear, all four, and no stun is spent',
        `the cursor on the door: ${on} · the order: ${row?.order} to ${row?.who?.length ?? 0} · the door tools: ${op?.kind ?? 'nothing'} · stuns ${b0} → ${T.bangs}`); }
    // 3 (the far room may be known by now: the word's entry never asked)
    { const set = stack(), b0 = T.bangs, had = new Set<any>(T.ops); I.pressed.add(DOORTOOLS.blowKey); step(3); const op = T.ops.find((o: any) => !had.has(o) && o.door === D);
      line(set && op?.kind === 'openClear' && op.men.length === N && T.bangs === b0, '3. stacked there again, the WORD\'s key: the same open and clear, all four, no stun spent',
        `stacked and set: ${set} · the door tools: ${op?.kind ?? 'nothing'} for ${op?.men?.length ?? 0} · stuns ${b0} → ${T.bangs}`); }
    // 4
    { const set = stack(); T.bangs = 3; const tm = O.forMen(O.targetAt(D.frameCentre), all()), no = O.give('bangClear', tm, 'all'); step(3); const op = opOn();
      line(set && no === null && op?.kind === 'bangClear', '4. stacked there again, \'bang and clear\' given as the ring\'s row gives it: that entry starts', `stacked and set: ${set} · the order: ${no ?? 'given'} · the door tools: ${op?.kind ?? 'nothing'}`); }
    // 5 (a door whose far room is still nobody's: the first door's is known by now, so the rule's own question is asked of the entries directly)
    { AUTOPILOT.plain = false; g.cleared.isKnown = ((f: any) => (r: any) => r === far() ? false : f.call(g.cleared, r))(g.cleared.isKnown); T.bangs = 3; const set = stack(), off = autoPick(g, O.targetAt(D.frameCentre));
      AUTOPILOT.plain = true; const on = autoPick(g, O.targetAt(D.frameCentre)); AUTOPILOT.plain = !BREAK;
      line(set && off.id === 'bangClear' && off.why === 'unknown' && on.id === 'openClear', '5. CONTROL — the rule off, the same stack and the room taken as nobody\'s: the pick is a stun, as it was',
        `stacked and set: ${set} · off: ${off.id} (${off.why}) · on: ${on.id}`); }
    // 6
    { AUTOPILOT.plain = true; const snap = JSON.parse(JSON.stringify(knobsSnapshot(g))), old = JSON.parse(JSON.stringify(snap)); delete old.tables.AUTOPILOT.plain;
      applyKnobs(g, old); const before = AUTOPILOT.plain; applyKnobs(g, snap); const on = AUTOPILOT.plain;
      const shipped = String(process.env.TPS_PROFILE ?? '') === 'shipped', row = FROZEN_HOLDS['AUTOPILOT.plain'];
      line(before === false && on === true && stood === shipped && /TPS_PLAINENTRY=1/.test(row ?? ''), '6. CONTROL (plumbing) — a recording from before the key replays with it off; the stands and the FROZEN_HOLDS row',
        `a file without the key: ${before} · one with it on: ${on} · this stand (${shipped ? 'the page\'s' : 'frozen'}) stood it ${stood} · the row: ${row ?? 'none'}`); }
  } finally { knobsBack(knobs0); Math.random = rnd0; }
  console.log(fails ? `=== plainentry-check: ${fails} FAIL ===` : '=== plainentry-check: ALL PASS ===');
  return !fails;
}

if (import.meta.main) process.exit((await plainEntryCheck()) ? 0 : 1);
