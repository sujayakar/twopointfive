// TEAMMATES FALLING IN NEVER SHOVE YOU — the check behind src/game/noshove.ts (NOSHOVE.on: on in the page, ?noshove=0; off on the frozen stand, TPS_NOSHOVE=1).
//   bun run tools/qa/noshove-check.ts [--verbose]      (also `probes.ts noshove`, family squadorders)
// Headless; nothing here looks at a picture. room-clear-four with the other side put far off the floor; the player stands with no key down unless a line says he walks:
//   1. THE CASE IT WAS FOUND IN, staged small: 'fall in' with one man a pace from the player and his place on the player's far side: he is not moved, the man goes
//      round him and is in his place;
//   2. THE WALK FROM A DISTANCE, square on and a foot off his line (the 'near enough already' rule held off so the man walks to his exact place): the same;
//   3. NO ROOM TO GO ROUND (the player stood in a passage): the man squeezes past and the player is not moved;
//   5. LEFT STANDING IN THE PLAYER'S KEEP-OFF (set down on top of each other): the man steps back out, the player stays;
//   6. THE PLAYER STILL CANNOT WALK THROUGH A FOLLOWER: walked at one under his own keys (stepping out of his way held off), he gets no nearer than his keep-off;
//   7. A SHOVE IS NOT HIS WALKING: a teammate on ANOTHER order walks into the standing player: he is still shouldered (that man is not falling in), but the pace the
//      squad reads off him stays under the 'walking' bar and a follower standing in the line of the shove does not step aside.
import { ROOT } from './headless.ts';
import { asProbe } from './holds.ts';   // (run alone, each part runs as the probe the runner calls it by: one stand both ways)
import { guarded, family } from './thresholdmenu-probes.ts';
const { rig } = (await import('./outerwall-probes.ts')) as any;
const { NOSHOVE, NOSHOVE_STATS } = await import(`${ROOT}/src/game/noshove.ts`);
const { ORDERS } = await import(`${ROOT}/src/game/orders.ts`);
const { KEEP_OFF } = await import(`${ROOT}/src/game/player.ts`);
const VERBOSE = process.argv.includes('--verbose'), SCN = 'room-clear-four';
type V = number[];
const flat = (a: V, b: V) => Math.hypot(a[0] - b[0], a[2] - b[2]);
const f2 = (x: number) => (Number.isFinite(x) ? x.toFixed(2) : '—'), f3 = (x: number) => (Number.isFinite(x) ? x.toFixed(3) : '—');
const say = (s: string) => { if (VERBOSE) console.log(`    ${s}`); };

/** how far a body can stand from `p` along (dx, dz) before a static, at shin height (m, up to 3) */
const clear = (g: any, p: V, dx: number, dz: number) => { const h = g.col.raycast([p[0], 0.45, p[2]], [dx, 0, dz], 3); return h ? h.t : 3; };
const DIRS = Array.from({ length: 8 }, (_, i) => [Math.sin(i * Math.PI / 4), Math.cos(i * Math.PI / 4)]);

/** the scenario stood up with the other side far off the floor and the four fallen in; floor found for the lines: an OPEN spot (a body's room and more
 *  every way, 3 m of floor east and west of it) and a TIGHT one (a passage: no room to pass a man standing in its middle, 2.6 m of floor each way along it) */
async function stand() {
  const R = await rig(SCN, 5), g = R.game;
  for (const k of Object.keys(NOSHOVE_STATS)) (NOSHOVE_STATS as any)[k] = 0;
  for (const h of R.hostiles) if (h.state !== 'dead') R.put(h, [h.char.pos[0] + 400, 0, h.char.pos[2]]);
  const home = [...g.player.char.pos]; R.gather(home, 20); R.step(60);
  const nav = g.col.nav, free = (p: V) => !nav.isBlocked(p[0], p[2]);
  const cells: V[] = []; for (let x = home[0] - 14; x <= home[0] + 14; x += 0.25) for (let z = home[2] - 14; z <= home[2] + 14; z += 0.25) { const p = [x, 0, z]; if (free(p) && Number.isFinite(R.pathLen(home, p))) cells.push(p); }
  cells.sort((a, b) => flat(a, home) - flat(b, home));
  const line = (p: V, ax: number, az: number, m: number) => { for (let k = 0.25; k <= m; k += 0.25) for (const s of [1, -1]) { const q = [p[0] + ax * k * s, 0, p[2] + az * k * s]; if (!free(q) || g.col.solidBlocked([p[0], 0.45, p[2]], [q[0], 0.45, q[2]])) return false; } return true; };
  let open: { p: V; ax: number; az: number } | null = null, tight: { p: V; ax: number; az: number } | null = null;
  for (const p of cells) {
    if (!open && DIRS.every(d => clear(g, p, d[0], d[1]) >= 1.7)) for (const [ax, az] of [[1, 0], [0, 1]]) if (!open && line(p, ax, az, 3.25)) open = { p, ax, az };
    if (!tight) for (const [ax, az] of [[1, 0], [0, 1]]) { const l = clear(g, p, az, -ax), r = clear(g, p, -az, ax);
      if (!tight && l < NOSHOVE.ring + 0.25 && r < NOSHOVE.ring + 0.25 && l > 0.38 && r > 0.38 && line(p, ax, az, 2.5)) tight = { p, ax, az }; }
    if (open && tight) break;
  }
  if (!open) throw new Error('noshove: no open floor near the start');
  const men = () => R.up as any[];
  /** the player set down at `p` with the trail forgotten, the men put where `at` says, 'fall in' given: the man asked for is the one nearest the player */
  const deal = (p: V, at: (i: number) => V) => {
    g.teleportPlayer([p[0] + 60, 0, p[2]]); R.step(2); men().forEach((m, i) => R.put(m, at(i)));
    g.teleportPlayer([p[0], 0, p[2]]); R.step(2); men().forEach((m, i) => { R.put(m, at(i)); m.char.vel = [0, 0, 0]; });
    const why = R.O.give('fallIn', R.O.targetAt(g.player.char.pos), 'all'); if (why) throw new Error(`noshove: 'fall in' refused — ${why}`);
  };
  return { R, g, home, open, tight, men, deal };
}

interface Walk { shoved: number; nearest: number; samV: number; passed: boolean; arrived: boolean; end: number; who: string; stats: Record<string, number>; legs: number; stuck: number }
/** a man `d` from the player on the far side of him from the other three (who stand off along the axis), `off` to the side of the line through him;
 *  'fall in'; `secs` played with no key down. Where: the open spot, or the passage. null: no such floor */
async function walk(o: { d: number; off?: number; tight?: boolean; nearOff?: boolean; secs?: number }): Promise<Walk | null> {
  const S = await stand(), { R, g } = S, at = o.tight ? S.tight : S.open; if (!at) return null;
  const nearM = ORDERS.nearM;
  if (o.nearOff) ORDERS.nearM = 99;   // (nobody is 'near enough already': every man walks to his exact place)
  try {
    const { p, ax, az } = at, px = az, pz = -ax, off = o.off ?? 0;
    S.deal(p, i => i === 0 ? [p[0] - ax * o.d + px * off, 0, p[2] - az * o.d + pz * off]
      : o.tight ? [p[0] + ax * (1.7 + 0.27 * i), 0, p[2] + az * (1.7 + 0.27 * i)] : [p[0] + ax * (2.2 + 0.45 * i) + px * (i - 2) * 0.9, 0, p[2] + az * (2.2 + 0.45 * i) + pz * (i - 2) * 0.9]);
    const m = S.men()[0], s0 = [...g.player.char.pos], A = () => R.O.of(m);
    let shoved = 0, nearest = 9, samV = 0, passed = false, legs = 0; const frames = (o.secs ?? 5) * 60, stuck0 = m.stuckN;
    for (let f = 0; f < frames; f++) {
      R.step(); const sp = g.player.char.pos, mp = m.char.pos;
      shoved = Math.max(shoved, flat(sp, s0)); nearest = Math.min(nearest, flat(mp, sp)); samV = Math.max(samV, (R.O as any).samV);
      if ((mp[0] - sp[0]) * ax + (mp[2] - sp[2]) * az > 0.3) passed = true;
      if (f >= frames - 120) legs = Math.max(legs, m.char.anim.speed);   // (the pace his legs cycle at over the last two seconds)
      if (f % 20 === 0) say(`f${f} ${m.callsign} (${f2(mp[0])}, ${f2(mp[2])}) ${f2(flat(mp, sp))} m off the player (${f3(sp[0])}, ${f3(sp[2])}) v${f2(m.speed)} goal ${A()?.goal ? `(${f2(A().goal[0])}, ${f2(A().goal[2])})` : '-'} · the squad reads his pace as ${f2((R.O as any).samV)}`);
    }
    return { shoved, nearest, samV, passed, arrived: !!A()?.arrived, end: flat(m.char.pos, g.player.char.pos), who: m.callsign, stats: { ...NOSHOVE_STATS }, legs, stuck: m.stuckN - stuck0 };
  } finally { ORDERS.nearM = nearM; }
}

/** lines 5, 6 on one stand-up: set down on top of each other · the player walked at a follower */
async function close() {
  const S = await stand(), { R, g } = S, { p, ax, az } = S.open!, px = az, pz = -ax;
  const sam = () => g.player.char.pos, spread = (i: number) => [p[0] + ax * (2.2 + 0.45 * i) + px * (i - 2) * 0.9, 0, p[2] + az * (2.2 + 0.45 * i) + pz * (i - 2) * 0.9];
  // 5. set down on top of each other: the man 0.3 m from the player, on the others' side of him (his place is not across the player)
  S.deal(p, i => i === 0 ? [p[0] + ax * 0.3, 0, p[2] + az * 0.3] : spread(i));
  const s5 = [...sam()], m5 = S.men()[0]; let apart = 0, moved5 = 0;
  for (let f = 0; f < 120; f++) { R.step(); moved5 = Math.max(moved5, flat(sam(), s5)); } apart = flat(m5.char.pos, sam());
  // 6. the player walked at a follower who stands (stepping out of his way held off): he is stopped at his keep-off, and does not come out the far side
  const asideM = ORDERS.asideM; ORDERS.asideM = 0; let nearest6 = 9, past6 = 9, back6 = 0;
  try {
    S.deal(p, i => i === 0 ? [p[0] + ax * 1.2, 0, p[2] + az * 1.2] : [p[0] + ax * (3 + 0.45 * i) + px * (i - 2) * 0.9, 0, p[2] + az * (3 + 0.45 * i) + pz * (i - 2) * 0.9]); R.step(60);
    const m6 = S.men()[0], to = [p[0] + ax * 3, 0, p[2] + az * 3];
    R.walk(to, 2.5, () => { nearest6 = Math.min(nearest6, flat(m6.char.pos, sam())); return false; });
    // … and walked past one's shoulder (the man a hand to the side of his line): he slides by at his keep-off, and the man is not made to step back
    S.deal(p, i => i === 0 ? [p[0] + ax * 1.2 + px * 0.3, 0, p[2] + az * 1.2 + pz * 0.3] : [p[0] + ax * (3 + 0.45 * i) + px * (i - 2) * 0.9, 0, p[2] + az * (3 + 0.45 * i) + pz * (i - 2) * 0.9]); R.step(60);
    const m7 = S.men()[0], back0 = NOSHOVE_STATS.back;
    R.walk(to, 2.5, () => { past6 = Math.min(past6, flat(m7.char.pos, sam())); return false; }); back6 = NOSHOVE_STATS.back - back0;
  } finally { ORDERS.asideM = asideM; }
  return { apart, moved5, nearest6, past6, back6 };
}

/** line 7: a teammate on a 'move' order walks through the standing player's spot, a foot to one side; a follower stands in the line of the shove */
async function pushed() {
  const S = await stand(), { R, g } = S, { p, ax, az } = S.open!, px = az, pz = -ax;
  const from = [p[0] - ax * 2.5 - px * 0.15, 0, p[2] - az * 2.5 - pz * 0.15], to = [p[0] + ax * 2.5 - px * 0.15, 0, p[2] + az * 2.5 - pz * 0.15];
  S.deal(p, i => i === 0 ? from : i === 1 ? [p[0] + px * 1.5, 0, p[2] + pz * 1.5] : [p[0] + ax * 1.2 * (i - 2.5) * 2 - px * 2.2, 0, p[2] + az * 1.2 * (i - 2.5) * 2 - pz * 2.2]);
  const W = S.men()[0], F = S.men()[1], idx = R.team.indexOf(W);
  const why = R.O.give('move', R.O.targetAt(to), idx); if (why) throw new Error(`noshove: 'move there' refused — ${why}`);
  const s0 = [...g.player.char.pos], f0 = [...F.char.pos]; let shoved = 0, samV = 0, moved = 0, nearest = 9;
  for (let f = 0; f < 300; f++) { R.step(); shoved = Math.max(shoved, flat(g.player.char.pos, s0)); samV = Math.max(samV, (R.O as any).samV); moved = Math.max(moved, flat(F.char.pos, f0)); nearest = Math.min(nearest, flat(W.char.pos, g.player.char.pos));
    if (f % 20 === 0) say(`f${f} ${W.callsign} ${f2(flat(W.char.pos, g.player.char.pos))} m off the player, who is ${f3(flat(g.player.char.pos, s0))} m off his spot · pace read ${f2((R.O as any).samV)} · ${F.callsign} ${f2(flat(F.char.pos, f0))} m off his`); }
  return { shoved, samV, moved, nearest, follower: !!R.O.of(F) && R.O.of(F).post === null, mover: R.O.of(W)?.order, kept: NOSHOVE_STATS.notWalked as number };
}

export const noshoveCheck = guarded(async () => {
  const F = family('noshove: teammates falling in never shove you');
  const keep = { rnd: Math.random };
  try {
    // ---- 1
    { const a = (await walk({ d: 0.58 }))!;
      F.verdict(a.shoved < 0.01 && a.samV < 0.05 && a.passed && a.nearest >= 0.55 && a.stats.round > 0, "the case it was found in: 'fall in' with a man a pace from the standing player, his place on the far side: the player is not moved, the man goes round",
        `the player moved ${f3(a.shoved)} m, ${a.who} no nearer than ${f2(a.nearest)} m and round him ${a.passed}, pace read ${f2(a.samV)} · frames turned round him ${a.stats.round}`); }
    // ---- 2
    const line = (a: Walk, what: string) => F.verdict(a.shoved < 0.01 && a.samV < 0.05 && a.passed && a.arrived && a.nearest >= NOSHOVE.ring - 0.03 && a.stats.round > 0,
      what, `the player moved ${f3(a.shoved)} m, ${a.who} no nearer than ${f2(a.nearest)} m, round him ${a.passed} and in his place ${a.arrived} (${f2(a.end)} m off), pace read ${f2(a.samV)} · frames turned round him ${a.stats.round}`);
    line((await walk({ d: 2.5, nearOff: true }))!, 'the walk from a distance, square on: he goes round the player and is in his place, the player not moved');
    line((await walk({ d: 2.5, off: 0.3, nearOff: true }))!, 'the walk from a distance, a foot off his line: the same');
    // ---- 3
    { const a = await walk({ d: 2.4, nearOff: true, tight: true });
      if (!a) F.verdict(false, 'no room to go round: a passage to stand the player in', 'no passage near the start narrow enough that a man cannot pass one standing in its middle');
      else F.verdict(a.shoved < 0.01 && a.passed && a.stats.through > 0 && a.samV < 0.05, 'no room to go round (the player stood in a passage): the man squeezes past and the player is not moved',
          `the player moved ${f3(a.shoved)} m, ${a.who} past him ${a.passed}, frames squeezing past ${a.stats.through}`); }
    // ---- 5, 6
    { const a = await close();
      F.verdict(a.moved5 < 0.01 && a.apart >= KEEP_OFF - 0.02, 'set down on top of each other: the man steps back out and the player stays',
        `the player moved ${f3(a.moved5)} m, ${f2(a.apart)} m apart after two seconds`);
      F.verdict(a.nearest6 >= KEEP_OFF - 0.02 && a.past6 >= KEEP_OFF - 0.004 && a.past6 < KEEP_OFF + 0.1 && a.back6 === 0, 'the player still cannot walk through a follower: walked at one under his own keys he gets no nearer than his keep-off, and walked past one\'s shoulder he slides by at it without the man being made to step back',
        `walked at him, no nearer than ${f2(a.nearest6)} m (his keep-off is ${KEEP_OFF}); past his shoulder, ${f3(a.past6)} m, frames the man stepped back ${a.back6}`); }
    // ---- 7
    { const a = await pushed();
      F.verdict(a.follower && a.mover === 'move' && a.shoved > 0.1 && a.samV < 0.3 && a.moved < 0.1 && a.kept > 0,
        'a shove is not his walking: a teammate on another order walks into the standing player: still shouldered, but his pace reads under the walking bar and a follower in the line of the shove stays',
        `the player moved ${f2(a.shoved)} m, pace read ${f2(a.samV)} m/s, the follower moved ${f2(a.moved)} m, frames a shove was kept out ${a.kept}`); }
  } finally { Math.random = keep.rnd; }
  return F.done();
});

if (import.meta.main) process.exit((await asProbe('noshove', noshoveCheck)) ? 0 : 1);
