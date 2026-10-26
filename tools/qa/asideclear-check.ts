// 'Fall in', Sam walking back at a man: the spot he steps aside to is never in a doorway (game/orders.ts ORDERS.asideClear, aside).
//   bun run tools/qa/asideclear-check.ts [--verbose]      (probes.ts asideclear, in family squadorders; ASIDECLEAR_BREAK=1 must FAIL)
// Found on compound-four as shipped: Sam walked west along the office's wall past the open door c_n, where a man of his file stood. "Sam is walking AT
// him": the man stepped off Sam's line on the side he was already on — into the doorway, in the leaf's swing. Sam pulled at the leaf across his way and
// it would not shut; and for as long as Sam worked at that leaf he was still walking at the man, so the man kept the doorway.
//   1. THE SPOTS: on three squad levels, round every door (as found, and with its angle set to 1.6 rad either way — the angle alone, nothing stepped: the
//      leaf's collider stays where it was found, and a one-way door is asked past its stop), a man put on a grid by the frame and Sam walking at him from
//      eight sides: the spot orders.ts aside() answers. Rule off: some of them are in a doorway's opening or a leaf's swing (the fault). Rule on: none is
//      (by the rule's own two questions: this row proves the wiring, row 2 the walk), and where the old spot was clear the answer is the very same spot.
//   2. THE FOUND CASE, staged the same way on either stand: c_n stood open (either way), one man on fall in put beside the door, the other three held on
//      the start, Sam walked west along the wall for eight seconds. Rule off: that man stands in the doorway on the spot he stepped aside to. Rule on: he
//      never stands in a doorway's way on such a spot (he steps to the room's side of Sam's line, or goes on to his place in the file).
//   3. the journal: a recording from before the key replays with it off; one that carries it keeps its own word, on or off
// The doorway manners are raised for the staging on either stand (the frozen stand holds them off; the page has them).
// Headless: nothing here says how it looks.
import { ROOT } from './headless.ts';
import { guarded, family } from './thresholdmenu-probes.ts';
const { rig } = (await import('./outerwall-probes.ts')) as any;
const { ORDERS } = await import(`${ROOT}/src/game/orders.ts`);
const { DOORMANNERS, doorwayInWay } = await import(`${ROOT}/src/game/doorway.ts`);
const { DESTRUCTION } = await import(`${ROOT}/src/game/destructionSwitch.ts`);
const { WALLCHARGE } = await import(`${ROOT}/src/game/wallcharge.ts`);
const { DOORSTAY } = await import(`${ROOT}/src/game/doors.ts`);
const VERBOSE = process.argv.includes('--verbose');
const flat = (a: number[], b: number[]) => Math.hypot(a[0] - b[0], a[2] - b[2]);
const fmt = (p: number[]) => `(${p[0].toFixed(2)}, ${p[2].toFixed(2)})`;

/** 1: every spot aside() answers round every door of `scn` — [asked, answered, in a doorway's way, differs from `ref` where `ref` was clear] */
async function spots(scn: string, on: boolean, ref: Map<string, number[] | null> | null, keep: Map<string, number[] | null>): Promise<[number, number, number, number, string]> {
  const R = await rig(scn, 5), g = R.game, O = R.O as any; DOORMANNERS.on = true; ORDERS.asideClear = on;
  for (const h of R.hostiles) if (h.state !== 'dead') R.put(h, [h.char.pos[0] + 400, 0, h.char.pos[2]]);
  const man = R.up[0]; if (!man) return [0, 0, 0, 0, 'no man up'];
  O.give('fallIn', O.targetAt(g.player.char.pos), 'all'); R.step(3);
  const sam = g.player.char.pos, bad = (q: number[]) => O.inFrame(q) || doorwayInWay(g, q);
  let asked = 0, got = 0, inWay = 0, moved = 0, first = '';
  for (const d of g.doors.list as any[]) {
    const was = { a: d.angle, l: d.latched };
    for (const st of [null, 1.6, -1.6]) {
      if (st !== null) { d.latched = false; d.angle = st; }
      const c = d.frameCentre;
      for (let ix = -4; ix <= 4; ix++) for (let iz = -4; iz <= 4; iz++) {
        const m = [c[0] + ix * 0.35, 0, c[2] + iz * 0.35]; if (g.col.nav.isBlocked(m[0], m[2])) continue;
        for (let k = 0; k < 8; k++) {
          const a = k * Math.PI / 4, s = [m[0] - Math.sin(a) * 1.1, 0, m[2] - Math.cos(a) * 1.1];
          if (g.col.nav.isBlocked(s[0], s[2]) || g.col.solidBlocked([s[0], 0.45, s[2]], [m[0], 0.45, m[2]])) continue;
          sam[0] = s[0]; sam[2] = s[2]; man.char.pos[0] = m[0]; man.char.pos[2] = m[2]; O.samV = 1.5; O.samDir = [Math.sin(a), 0, Math.cos(a)]; O.asideMem.delete(man);
          const q = O.aside(man) as number[] | null, key = `${d.def.name}|${st}|${ix}|${iz}|${k}`; asked++; keep.set(key, q ? [...q] : null);
          if (q) { got++; if (bad(q)) { inWay++; if (!first) first = `${d.def.name}${st === null ? '' : ` stood at ${st} rad`}: the man at ${fmt(m)}, Sam at ${fmt(s)} → ${fmt(q)}`; } }
          if (ref) { const r = ref.get(key); if (r && !bad(r) && (!q || flat(q, r) > 1e-6)) moved++; }
        }
      }
      d.angle = was.a; d.latched = was.l;
    }
  }
  return [asked, got, inWay, moved, first];
}

interface Case { ok: boolean; inDoor: number; stood: number; aside: number; say: string }
/** 2: the found case on compound-four — `inDoor`: frames (of 480) the man stands within 0.3 m of the spot he is stepping aside to, that spot in a doorway's way ·
 *  `stood`: frames he stands still in a doorway's way, whatever he is walking to */
async function found(on: boolean, leaf: number): Promise<Case> {
  const R = await rig('compound-four', 5), g = R.game, O = R.O as any; DOORMANNERS.on = true; DOORSTAY.on = true; ORDERS.asideClear = on;
  for (const h of R.hostiles) if (h.state !== 'dead') R.put(h, [h.char.pos[0] + 400, 0, h.char.pos[2]]);
  const d = (g.doors.list as any[]).find(x => x.def.name === 'c_n'); if (!d) return { ok: false, inDoor: 0, stood: 0, aside: 0, say: 'no door c_n' };
  const open = () => { d.latched = false; d.angle = leaf; };
  const man = R.up[0], idx = O.roster.indexOf(man); if (!man || idx < 0) return { ok: false, inDoor: 0, stood: 0, aside: 0, say: 'no man up' };
  O.give('hold', O.targetAt(g.player.char.pos), 'all'); R.step(3);   // (the other three stay on the start: one man and Sam at the door, as it was found)
  open(); g.teleportPlayer([32.2, 0, 15.25]); R.put(man, [30.72, 0, 14.65]); R.step(3); O.give('fallIn', O.targetAt(g.player.char.pos), idx); open(); R.step(2);
  const to = [29.8, 0, 15.3], bad = (q: number[]) => O.inFrame(q) || doorwayInWay(g, q);
  let inDoor = 0, aside = 0, stood = 0, last = [...man.char.pos]; const I = R.input, KEYS = ['KeyW', 'KeyA', 'KeyS', 'KeyD'];
  for (let f = 0; f < 480; f++) {
    for (const k of KEYS) I.keys.delete(k);
    const sam = g.player.char.pos, dx = to[0] - sam[0], dz = to[2] - sam[2], dd = Math.hypot(dx, dz);   // (read every frame: the game hands out a new position)
    if (dd > 0.2) { const { fwd, right } = R.W.cam.planarBasis(), a = (dx * fwd[0] + dz * fwd[2]) / dd, b = (dx * right[0] + dz * right[2]) / dd, T = 0.3827; if (a > T) I.keys.add('KeyW'); if (a < -T) I.keys.add('KeyS'); if (b > T) I.keys.add('KeyD'); if (b < -T) I.keys.add('KeyA'); }
    R.step();
    if (f >= 30 && flat(man.char.pos, last) < 0.004 && bad(man.char.pos)) stood++; last = [...man.char.pos];   // (the first half second is his own: he was put down by the frame)
    { const q = O.asideMem.get(man)?.q, A = O.of(man); if (q && A && A.post === null && flat(A.goal, q) <= 0.05) { aside++; if (bad(q) && flat(man.char.pos, q) < 0.3) inDoor++; } }
  }
  for (const k of KEYS) I.keys.delete(k);
  const sam = g.player.char.pos;
  return { ok: true, inDoor, stood, aside, say: `Sam at ${fmt(sam)} · leaf ${d.angle.toFixed(2)} rad · ${(R.up as any[]).map(m => `${m.callsign} ${fmt(m.char.pos)}`).join(' · ')}` };
}

export const asideclearCheck = guarded(async () => {
  const F = family('asideclear: fall in — a step aside is never into a doorway');
  const keep = { m: DOORMANNERS.on, s: ORDERS.asideClear, st: DOORSTAY.on, rnd: Math.random, d: DESTRUCTION.on, w: WALLCHARGE.on, o: WALLCHARGE.outer };
  try {
    // ---- 1. the spots
    let offWay = 0, onWay = 0, moved = 0, asked = 0, onGot = 0, offGot = 0, first = '';
    for (const scn of ['compound-four', 'room-clear-four', 'depot-four']) {
      const A = new Map<string, number[] | null>(), B = new Map<string, number[] | null>();
      const off = await spots(scn, false, null, A), on = await spots(scn, true, A, B);
      asked += off[0]; offGot += off[1]; offWay += off[2]; onGot += on[1]; onWay += on[2]; moved += on[3]; if (!first) first = off[4];
      console.log(`  ${scn}: ${off[0]} asked · rule off ${off[1]} spots, ${off[2]} in a doorway's way · rule on ${on[1]} spots, ${on[2]} in a doorway's way, ${on[3]} moved off a clear spot`);
    }
    F.verdict(offWay > 0, 'rule off, the fault is there: Sam walking at a man by a door, some spots he steps aside to are in a doorway\'s opening or a leaf\'s swing', `${offWay} of ${offGot} spots · the first: ${first}`);
    F.verdict(onWay === 0 && onGot > 0, 'rule on: no spot he steps aside to is in a doorway\'s opening or a leaf\'s swing', `${onWay} of ${onGot} spots (${asked} asked)`);
    F.verdict(moved === 0, 'rule on: where the old spot was clear of every door, the spot is the very same one', `${moved} moved`);
    // ---- 2. the found case
    for (const leaf of [1.92, -1.92]) { const off = await found(false, leaf), on = await found(true, leaf), how = `c_n stood open at ${leaf} rad`;
      if (VERBOSE) { console.log(`  off: ${off.say} · aside ${off.aside} frames · on that spot in the doorway ${off.inDoor} · still in a doorway ${off.stood}`); console.log(`  on:  ${on.say} · aside ${on.aside} frames · on that spot in the doorway ${on.inDoor} · still in a doorway ${on.stood}`); }
      F.verdict(off.ok && off.inDoor >= 30, `the found case, rule off, ${how}: the man beside the door stands in the doorway on the spot he stepped aside to (half a second or more of eight)`, `${off.inDoor} frames · ${off.say}`);
      F.verdict(on.ok && on.inDoor === 0 && on.stood === 0, `the found case, rule on, ${how}: he never stands in a doorway's way — not on a spot he stepped aside to, not on any other`, `${on.inDoor} frames on such a spot · ${on.stood} frames still in a doorway (rule off ${off.stood}) · stepping aside ${on.aside} frames · ${on.say}`); }
    // ---- 3. the journal
    { const J = await import(`${ROOT}/src/game/journal.ts`), R = await rig('room-clear-four', 5), snap = () => JSON.parse(JSON.stringify(J.knobsSnapshot(R.game)));
      ORDERS.asideClear = true; const old = snap(); if (old.tables.ORDERS) delete old.tables.ORDERS.asideClear; if (old.flags) delete old.flags.asideClear; J.applyKnobs(R.game, old); const a = ORDERS.asideClear;
      ORDERS.asideClear = true; const yes = snap(); ORDERS.asideClear = false; J.applyKnobs(R.game, yes); const b = ORDERS.asideClear;
      ORDERS.asideClear = false; const no = snap(); ORDERS.asideClear = true; J.applyKnobs(R.game, no); const c = ORDERS.asideClear;
      F.verdict(a === false && b === true && c === false, 'the journal: a recording from before the key replays with it off, one made with it on replays on, one made with it off replays off', `old → ${a} · recorded on → ${b} · recorded off → ${c}`); }
  } finally { DOORMANNERS.on = keep.m; ORDERS.asideClear = keep.s; DOORSTAY.on = keep.st; Math.random = keep.rnd; DESTRUCTION.on = keep.d; WALLCHARGE.on = keep.w; WALLCHARGE.outer = keep.o; }
  return F.done();
});

if (import.meta.main) process.exit((await asideclearCheck()) ? 0 : 1);
