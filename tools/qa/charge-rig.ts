// A WALL CHARGE SET AND SET OFF AS THE GAME IS PLAYED — for a check that needs a wall blown and is not itself about the setting.
//
// Until 2026-11-07 such checks made the charge by hand on the game object they hold: `C.set.push({ wall, t, by: 'probe', paid: 0 }); C.blow(C.set[0])`. That
// object is the tools' copy of the game since the page plays the Rust game: a row pushed onto it is a write that is never sent, the Rust game has no such
// charge, and setting it off opened nothing (or trapped, wallcharge/p_02.rs, before the write was counted as not sent). What crosses is what a player does,
// so that is what this does, by the game's own keys and rows and nothing else:
//   samSets      Sam's own row at the wall's foot ('breach the wall', the interaction key, stood on for setSecs);
//   samBlows     stood OUT of its blast, the tap of the interaction key on 'blow the charge' — his own charge goes off from his own action, not the
//                squad's key (ownblow.ts);   samBreach = the two;
//   squadBreach  a squad on the floor: the wall's order given to all (`SquadOrders.give`), a man sets it, the rest stand off, and the squad's key (F) is
//                the word once the order is armed.
// Each returns what happened, step by step, so a check can say WHERE a breach it needed did not come about instead of failing its own lines on a wall
// that never opened. Nothing here writes a switch or a table; the bag a man alone sets from is put as the game puts it for a man alone (`alone`).
import { ROOT } from './headless.ts';
const { DOORTOOLS } = await import(`${ROOT}/src/game/doortools.ts`);
const { WALL_ORDER, wallPt, wallTarget, safeRadius } = await import(`${ROOT}/src/game/wallcharge.ts`);

type Vec3 = [number, number, number];
const DT = 1 / 60;
/** the harness stand (headless.ts standUp), as far as this needs it */
export interface Stand { game: any; input: any; step(dt: number): void; cursorAt(p: number[]): boolean }
/** what a breach came to. `set`: the charge is on the wall · `blown`: it is no longer set · `hole`: the wall's own record of the gap (null = it held, or was
 *  never blown) · `wall`: where the charge settled (the game may slide it along the wall) · `why`: in words, the first step that did not come about */
export interface Breach { set: boolean; blown: boolean; hole: { du: number; half: number } | null; wall: any; why: string }

const flat = (a: number[], b: number[]) => Math.hypot(a[0] - b[0], a[2] - b[2]);
const steps = (W: Stand, n: number, each?: () => void) => { for (let i = 0; i < n; i++) { each?.(); W.step(DT); } };
const until = (W: Stand, pred: () => boolean, secs: number, each?: () => void): boolean => { for (let i = 0; i < secs * 60 && !pred(); i++) { each?.(); W.step(DT); } return pred(); };

/** a place for Sam out of the charge's blast: along the wall's own face and back off it, the nearest free floor where the game says he has no share of it */
function clearOf(g: any, w: any): Vec3 | null {
  const C = g.wallCharge, nav = g.col.nav, R = safeRadius(g, w), far = Math.max(R.standoff, R.reach) + 1.5;
  for (const k of [1, 1.5, 2, 3]) for (const [du, off] of [[far * k, 1.0], [-far * k, 1.0], [0, far * k], [far * k, far * k], [-far * k, far * k]] as [number, number][]) {
    const p = wallPt(w, du, off) as Vec3; if (nav.isBlocked(p[0], p[2])) continue;
    if (C.blastShare(w, p, R, false) === 0) return p; }
  return null;
}

/** Sam sets a charge on wall `w` with his own hands: stood at its foot with the cursor on it, the wall's own row given on the interaction key and stood on for
 *  setSecs. `each` is called before every step (a rig's own per-step staging: a defender held still, the player kept dark). On return `charge` is the game's
 *  own row of it (null where it did not come about, and `why` says which step). */
export function samSets(W: Stand, w: any, each?: () => void): Breach & { charge: any } {
  const g = W.game, C = g.wallCharge, D = g.doorTools, I = W.input, out: Breach & { charge: any } = { set: false, blown: false, hole: null, wall: w, why: '', charge: null };
  if (!w || !w.ok) { out.why = `no place for a charge on that wall${w?.why ? ` (${w.why})` : ''}`; return out; }
  D.alone(DOORTOOLS.charges); C.setCarried(null);   // (a man who sets it himself has the door tools' bag, and a wall takes its charges out of it)
  // (WITH A SQUAD ON THE FLOOR `alone` deals nothing: the bag is the roster's, and one wall may have emptied it. The game then offers no row, by its own
  //  rule (WallCharge.affordable). A second charge is then one that was BROUGHT, the kit's own count, as squadBreach puts it)
  if (!C.affordable) C.setCarried(1);
  const n0 = C.set.length, on: Vec3 = wallPt(w, 0, 0, 1.0) as Vec3;   // (a point of the wall's near face a metre up: where a cursor on that wall lies. Its top edge is no pick of it)
  // (WHERE HE STANDS: the game's own spot for the setting first, then the foot and a step along it either way. A man put where a thing stands at the wall's
  //  foot is pushed off it, and a step breaks the setting off ('charge not set'): the next place is tried, and `why` says what the last one came to)
  const nav = g.col.nav, places = [C.setSpot?.(w) ?? null, wallPt(w, 0, 0.55), wallPt(w, 0.4, 0.55), wallPt(w, -0.4, 0.55)].filter((p): p is Vec3 => !!p && !nav.isBlocked(p[0], p[2]));
  for (const p of places) {
    g.teleportPlayer([p[0], 0, p[2]]); steps(W, 90, () => { each?.(); W.cursorAt(on); });   // (the cursor held on the wall while the camera settles on him)
    if (!g.selectVerb('wall', 'charge')) { out.why = `no 'breach the wall' row at its foot (rows: ${(g.nearby as any[]).map(r => `${r.kind}:${r.verb ?? '-'}`).join(' ') || 'none'} · the bag: ${D.charges} charges, ${C.carried ?? 'no'} wall charges carried, roster ${D.roster?.length ?? '-'})`; continue; }
    I.pressed.add('Space'); W.step(DT);
    if (until(W, () => C.set.length > n0, DOORTOOLS.setSecs + 1, each)) { out.why = ''; break; }
    out.why = `the charge was not on the wall ${DOORTOOLS.setSecs + 1} s after the row was given`;
  }
  if (C.set.length <= n0) { if (!out.why) out.why = 'no free floor at its foot'; return out; }
  out.charge = C.set[C.set.length - 1]; out.set = true; out.wall = out.charge.wall; return out;
}
/** Sam sets off a charge his own hands set, from out of its blast: the tap of the interaction key on 'blow the charge', the only row out there (ownblow.ts:
 *  a bare tap never sets it off with him in its blast, and the squad's key is not his charge's). `from` = where he stands for it; left out, the nearest free
 *  floor the game says is clear of it. */
export function samBlows(W: Stand, charge: any, each?: () => void, from?: Vec3): Breach {
  const g = W.game, C = g.wallCharge, I = W.input, out: Breach = { set: C.set.includes(charge), blown: false, hole: null, wall: charge?.wall, why: '' };
  if (!out.set) { out.why = 'no such charge on a wall'; return out; }
  const p = from ?? clearOf(g, charge.wall); if (!p) { out.why = 'no free floor out of its blast to set it off from'; return out; }
  g.teleportPlayer(p); steps(W, 12, each);
  const row = (g.nearby as any[]).find(r => r.kind === 'charge' && r.verb === 'blow');
  if (!row) { out.why = `stood ${flat(p, wallPt(charge.wall, 0, 0)).toFixed(1)} m off, no 'blow the charge' row (rows: ${(g.nearby as any[]).map(r => `${r.kind}:${r.verb ?? '-'}`).join(' ') || 'none'})`; return out; }
  g.selectVerb('charge', 'blow'); I.pressed.add('Space'); I.keys.add('Space'); W.step(DT); I.keys.delete('Space'); steps(W, 3, each);
  out.blown = !C.set.includes(charge); if (!out.blown) { out.why = "the tap on 'blow the charge' set nothing off"; return out; }
  out.hole = C.holeAt(charge.wall) ?? null; if (!out.hole) out.why = 'blown, and the wall held'; return out;
}
/** both: set by his hands, set off by his own action from out of its blast */
export function samBreach(W: Stand, w: any, each?: () => void): Breach { const a = samSets(W, w, each); return a.charge ? samBlows(W, a.charge, each) : a; }

/** A squad on the floor: the wall's order given to all at wall point `at` (a point on the wall's line, as the cursor would pick it), a man sets it, and the
 *  squad's key is pressed once the order is armed. `secs` = how long the men may take to set it and stand off. */
export function squadBreach(W: Stand, at: Vec3, each?: () => void, secs = 90): Breach {
  const g = W.game, C = g.wallCharge, O = g.orders, I = W.input, out: Breach = { set: false, blown: false, hole: null, wall: null, why: '' };
  if (C.carried !== null && C.carried < 1) C.setCarried(2);   // (the kit's rule: a wall charge is brought, and a squad with none has nothing to set)
  const n0 = C.ops.length, why = O.give(WALL_ORDER, wallTarget(g, at, { kind: 'floor', at: O.freeSpot?.(at) ?? at }), 'all'), op = C.ops[C.ops.length - 1];
  if (why || C.ops.length === n0 || !op) { out.why = `the order was not taken${why ? ` ("${why}")` : ''}`; return out; }
  out.wall = op.wall;
  if (!until(W, () => op.phase === 'armed' || !C.ops.includes(op), secs, each) || op.phase !== 'armed') { out.why = `the order never armed (phase '${op.phase}'${op.ended ? `, ended '${op.ended}'` : ''})`; return out; }
  out.set = true; I.pressed.add(DOORTOOLS.blowKey); W.step(DT); steps(W, 3, each);
  out.blown = !C.set.some((c: any) => c.wall === op.wall || (c.wall?.index === op.wall.index && Math.abs(c.wall.u - op.wall.u) < 1e-6)); if (!out.blown) { out.why = 'the squad\'s key set nothing off'; return out; }
  out.hole = op.hole ?? C.holeAt(op.wall) ?? null; if (!out.hole) out.why = 'blown, and the wall held'; return out;
}
