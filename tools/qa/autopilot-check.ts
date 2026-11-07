// B IS THE SQUAD'S AUTOPILOT (src/game/autopilot.ts, AUTOPILOT.on): the quick key through the game's own input, headless.
//   bun run tools/qa/autopilot-check.ts            (also `probes.ts autopilot`, in family squadorders)
// PASS / FAIL each: the room clear on the office with its squad of four, everybody under god mode (the lines are about what is ORDERED, not who wins).
// Every B is a one-frame press of the quick key with the harness cursor on the thing; what is read back is the order book, the door tools' ops and the
// plan the overlay is handed (`floorPlan(g)`).
//   exit 0 = all hold · 1 = one does not
import { standUp, ROOT } from './headless.ts';

const AP: any = await import(`${ROOT}/src/game/autopilot.ts`);
const FP: any = await import(`${ROOT}/src/game/floorplan.ts`);
const { NIGHTBREACH } = await import(`${ROOT}/src/game/nightbreach.ts`);
const { ORDERS } = await import(`${ROOT}/src/game/orders.ts`);
const { AUTOPILOT, AUTOPILOT_STATS, autoPick } = AP;
const { PLAN, floorPlan } = FP;

const DT = 1 / 60;
function seeded(a: number) { return () => { a |= 0; a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ (a >>> 15), 1 | a); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; }; }
const dist = (a: number[], b: number[]) => Math.hypot(a[0] - b[0], a[2] - b[2]);
const f1 = (x: number) => (Number.isFinite(x) ? x.toFixed(1) : '—');
const VERBOSE = process.argv.includes('--verbose');

async function rig(seed: number, squad = 4) {
  const squad0 = NIGHTBREACH.squad; let W: any, why: any;
  try {
    Object.assign(NIGHTBREACH, { squad });
    Math.random = seeded(seed);
    W = await standUp();
    W.game.godMode = true;
    why = W.game.roomClear.start(seed);
  } finally { Object.assign(NIGHTBREACH, { squad: squad0 }); }
  const game = W.game, O = game.orders, input = W.input;
  const R = { W, game, O, why,
    get team(): any[] { return O.squad(); },
    step(n = 1) { for (let i = 0; i < n; i++) { game.player.visibility = 0.05; W.step(DT); } },
    until(pred: () => boolean, secs: number) { for (let i = 0; i < secs * 60 && !pred(); i++) R.step(); return pred(); },
    /** B, one frame, with the cursor on p — and what the cursor was on, as the game's own input reads it */
    quick(p: number[]) { W.cursorAt([p[0], p[1], p[2]]); R.step(2); const on = O.hover?.() ?? null; input.pressed.add(ORDERS.quickKey); R.step(); return on; },
    /** the HUD's B line with the cursor on p */
    hudB(p: number[]): string { W.cursorAt([p[0], p[1], p[2]]); R.step(2); return (O.hud().status?.[0]?.opts ?? []).find((o: any) => o.key === 'B')?.text ?? ''; },
    tail() { return O.log[O.log.length - 1]; },
    listed() { return (floorPlan(game).men as any[]).length; },
    stacked() { return R.team.filter((m: any) => m.state !== 'dead').every((m: any) => O.of(m)?.order === 'stack' && O.of(m)?.arrived); },
    /** everybody back on Sam, nothing of the door tools running */
    home() { O.give('fallIn', O.targetAt(game.player.char.pos), 'all'); R.until(() => game.doorTools.ops.length === 0, 40); R.step(60); },
  };
  R.step(30);
  for (const m of R.team) m.god = true;
  return R;
}
/** the doors by how near Sam they are, each with the target the orders make of it */
function doorsNear(R: any): { d: any; t: any }[] {
  const s = R.game.player.char.pos, out: { d: any; t: any }[] = [];
  for (const d of [...R.game.doors.list].sort((a: any, b: any) => dist(a.frameCentre, s) - dist(b.frameCentre, s))) { const t = R.O.targetAt(d.frameCentre); if (t.kind === 'door' && t.door === d) out.push({ d, t }); }
  return out;
}
/** a point of that door's doorway the cursor on which IS that door, as the game's own input reads it (null: none — a door off the screen, behind a wall
 *  as drawn, or with a teammate's body under every such point) */
function aimOf(R: any, d: any): number[] | null {
  const c = d.frameCentre, th = d.def.closedDir, nx = -Math.sin(th), nz = Math.cos(th);
  for (const y of [c[1], 0.05, 1.0, 1.8]) for (const k of [0, 0.35, -0.35]) { const p = [c[0] + nx * k, y, c[2] + nz * k]; R.W.cursorAt(p); R.step(2); const h = R.O.hover?.(); if (h?.kind === 'door' && h.door === d) return p; }
  return null;
}
const pointsAt = (R: any, d: any) => !!aimOf(R, d);
/** B on that door through the game's own input where the cursor can be put on it; else the input's own line asked of the target (orders.ts input()) */
function quickDoor(R: any, d: any): 'key' | 'call' {
  // (the press goes in the frame after the cursor was seen to be on the door: a man walking through the doorway a moment later is not what was pressed on)
  if (aimOf(R, d)) { R.W.input.pressed.add(ORDERS.quickKey); R.step(); return 'key'; }
  const O = R.O, t = O.targetAt(d.frameCentre); if (!(AUTOPILOT.on && AP.autoQuick(R.game, t))) { const def = O.defaultFor(t); if (def) O.give(def.id, t); } R.step();
  return 'call';
}
const farRoom = (g: any, t: any) => g.cleared.roomBy(t.door, t.side > 0 ? -1 : 1);

let fails = 0;
const line = (ok: boolean, name: string, detail: string) => { if (!ok) fails++; console.log(`  ${ok ? 'PASS' : 'FAIL'}  ${name} — ${detail}`); };

export async function autopilotCheck(): Promise<boolean> {
  fails = 0;
  const keep = { rnd: Math.random };
  console.log("autopilot-check: B is the squad's autopilot");
  try {
    const R = await rig(5), g = R.game, O = R.O, T = g.doorTools;
    if (R.why) { line(false, 'the room clear stands up', String(R.why)); return false; }
    const all = () => O.men('all'), stackDef = O.defs.find((d: any) => d.id === 'stack');

    // the door: the nearest one whose far room nobody has stood in, that the four can stack on and that the cursor on its doorway is on
    let pick: { d: any; t: any } | null = null, w: string | null = 'no door';
    for (const x of doorsNear(R)) { const far = farRoom(g, x.t); if (!far || g.cleared.isKnown(far)) continue; w = O.whyNot(stackDef, O.forMen(x.t, all()), all()); if (!w && !pointsAt(R, x.d)) w = 'the cursor on it is not on it'; if (!w) { pick = x; break; } }
    if (!pick) { line(false, 'a door to stack on with an unknown room behind it', `none: ${w}`); return false; }
    const D = pick.d, at = D.frameCentre;

    // 1. B on a door nobody is on: stack up, as ever — and its plan is drawn, for showSecs, then gone
    const on1 = R.quick(aimOf(R, D) ?? at), row1 = R.tail(), n0 = R.listed();
    R.step(Math.round((PLAN.freshSecs + 0.5) * 60)); const nMid = R.listed();
    R.step(Math.round((AUTOPILOT.showSecs - PLAN.freshSecs) * 60)); const nEnd = R.listed();
    line(on1?.kind === 'door' && on1.door === D && row1?.order === 'stack' && row1.auto === true && n0 === 4 && nMid === 4 && nEnd === 0,
      'B on a door nobody is on: stack up, the plan drawn for its own time, then gone', `cursor on ${on1?.kind ?? 'nothing'} · order ${row1?.order} · stamped ${!!row1?.auto} · listed ${n0} at once, ${nMid} at ${f1(PLAN.freshSecs + 0.5)} s, ${nEnd} at ${f1(AUTOPILOT.showSecs + 0.5)} s`);
    R.home();
    // 5. a tool given on the word to one pair: B on that door is the word, for the men on it
    {
      let door: any = null, why: string | null = 'no door';
      for (const x of doorsNear(R)) { if (!pointsAt(R, x.d)) continue; why = O.give('openClear', x.t, 'red', { onWord: true }); if (!why) { door = x.d; break; } }
      R.step(30);
      const waits = () => T.ops.find((o: any) => o.door === door && !o.word);
      const before = !!waits(), wants = door ? autoPick(g, O.targetAt(door.frameCentre)) : null, len = O.log.length;
      if (door) quickDoor(R, door); R.step(2);
      const went = !waits(), row = R.tail(), red = O.men('red').map((m: any) => m.callsign).sort().join();
      line(!!door && before && wants?.word === true && went && O.log.length === len + 1 && row.order === 'go' && row.auto === true && [...row.who].sort().join() === red,
        'a door with a tool waiting on the word: B is the word, to the men on it', door ? `waiting ${before} · pick ${wants?.word ? 'the word' : wants?.id} · after B: ${went ? 'went' : 'STILL waiting'} · order ${row?.order} to ${row?.who.join(', ')}` : `refused everywhere: ${why}`);
      R.until(() => T.ops.length === 0, 60); R.home();
    }

    // 6. a charge set on a door: B on it blows it
    {
      let door: any = null, why: string | null = 'no door';
      for (const x of doorsNear(R)) { why = O.give('charge', x.t, 'blue'); if (!why) { door = x.d; break; } }
      const isSet = R.until(() => T.set.length > 0, 40); R.step(5);
      const wants = door ? autoPick(g, O.targetAt(door.frameCentre)) : null;
      const how = door ? quickDoor(R, door) : 'call', blown = R.until(() => T.set.length === 0, 6);
      line(!!door && isSet && (wants?.word === true || wants?.id === 'charge') && blown, 'a charge set on a door: B on it blows it', door ? `set ${isSet} · pick ${wants?.word ? 'the word' : `${wants?.id} (${wants?.why})`} · B (${how}): ${T.set.length} set after` : `refused everywhere: ${why}`);
      R.step(120); R.until(() => T.ops.length === 0, 30); R.home();
    }

    // 7. the squad's own change of plan is drawn for a moment (SquadOrders.show — doortools.ts haltAt calls it); a plain re-assign is not
    {
      const sp = g.player.char.pos; O.give('hold', O.targetAt(sp), 'all'); R.step(Math.round((AUTOPILOT.showSecs + 1) * 60));
      const m0 = R.team[0], A0 = O.of(m0), quiet = R.listed();
      const again = () => O.assign(m0, { order: A0.order, post: A0.post ?? [sp[0] + 1, 0, sp[2]], face: A0.face, going: A0.going, there: A0.there });
      again(); R.step(2); const re = R.listed();
      again(); O.show([m0]); R.step(2); const shown = R.listed();   // (the module that re-plans says so the frame it does: doortools.ts haltAt)
      R.step(Math.round((AUTOPILOT.showSecs + 0.5) * 60)); const gone = R.listed();
      line(quiet === 0 && re === 0 && shown === 1 && gone === 0, 'their own change of plan is drawn for a moment and goes; a silent re-assign is not drawn', `before ${quiet} · re-assigned ${re} · told ${shown} · ${f1(AUTOPILOT.showSecs + 0.5)} s on ${gone}`);
    }

    if (VERBOSE) console.log('  stats', JSON.stringify(AUTOPILOT_STATS));
  } catch (e) { line(false, 'the lines threw', e instanceof Error ? (e.stack ?? e.message).split('\n').slice(0, 4).join(' · ') : String(e)); }
  finally { Math.random = keep.rnd; }
  console.log(fails ? `autopilot-check: ${fails} FAIL` : 'autopilot-check: ALL PASS');
  return fails === 0;
}

if (import.meta.main) { const ok = await autopilotCheck(); process.exit(ok ? 0 : 1); }
