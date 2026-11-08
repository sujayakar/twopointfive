// THE MEN WITH SAM STEP BACK FROM A SET WALL CHARGE — the check behind WALLCHARGE.menClear (src/game/wallcharge.ts clearOf, asked by orders.ts of a man on
// 'fall in'; a settled rule since 2026-11-07, as are the safety radius and charges on outer walls it stands on: a tool stands the game with all three on).
//   bun run tools/qa/menclear-check.ts        (also `probes.ts menclear`)
// The maintainer's own case (The pass, 10-16, the trigger's row reading TOO CLOSE with two of his men named: "after i place a wall charge, my guys should
// move away with standoff"): the four on 'fall in', Sam at the foot of the bunk room's long wall in the yard with a charge of his own on it, the men bunched
// beside him.
//   (3) seconds after his charge goes on: the row names nobody but Sam, every man stands out of the blast on a mark of his own, and none of them ran;
//   (4) Sam walks clear: the men leave their marks and fall in on him;
//   (5) Sam back at the wall: they stand off again;
// (1) the stand's line, (2) the rule off 'the fault as he met it' and (8) the recording from before the key and the page's flag went on 2026-11-07: the rule's
// off side cannot be staged any more.
// MENCLEAR_BREAK=1 = the four are told to HOLD where they are bunched, not to fall in (the rule is for the men on fall in): the lines must fail.   exit 0 = all hold · 1 = one does not
// Headless: lines are the collision's own. Never seen on a GPU; nobody has played it.
import { standUp, ROOT } from './headless.ts';
import { samSets, samBlows } from './charge-rig.ts';
import { readFileSync } from 'node:fs';
const { WALLCHARGE, wallAt, wallPt, safeRadius } = await import(`${ROOT}/src/game/wallcharge.ts`);
const { DESTRUCTION } = await import(`${ROOT}/src/game/destructionSwitch.ts`);
const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
const { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`);
const { startFromQuery } = await import(`${ROOT}/src/game/scenarioRun.ts`);
const { SILENT, setSilent, squadSilent } = await import(`${ROOT}/src/game/silent.ts`);
const { SNEAK, sneaking } = await import(`${ROOT}/src/game/sneak.ts`);
type Vec3 = [number, number, number];

/** the charge's place on the wall's yard face (wallAt takes a point within WALLCHARGE.pickM of it), where Sam walks clear to, and the bunch: each man's place off the setter's spot */
const CHARGE: Vec3 = [31.28, 0, 18.14], AWAY: Vec3 = [20.5, 0, 21.5];
const BUNCH: [number, number][] = [[-0.9, 0.5], [0.9, 0.5], [-0.5, 1.4], [0.6, 1.5]];

export async function menclearCheck(): Promise<boolean> {
  const BREAK = process.env.MENCLEAR_BREAK === '1';
  console.log(`\n=== menclear: the men with Sam step back from a charge set on a wall${BREAK ? ' — MENCLEAR_BREAK: the men told to hold, not to fall in' : ''} ===`);
  let all = true; const line = (ok: boolean, label: string, detail = '') => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}${detail ? `\n    ${detail}` : ''}`); };
  const destr0 = DESTRUCTION.on, rnd0 = Math.random;
  const log = console.log, quiet = async <T>(fn: () => Promise<T>): Promise<T> => { console.log = () => { }; try { return await fn(); } finally { console.log = log; } };
  const flat = (a: number[], b: number[]) => Math.hypot(a[0] - b[0], a[2] - b[2]), at = (p: number[]) => `(${p[0].toFixed(1)}, ${p[2].toFixed(1)})`;
  const data = parseLevelData(readFileSync(`${ROOT}/levels/pass.json`, 'utf8'));
  try {
    /** The pass, the four in the yard on 'fall in', bunched at the foot of the wall beside Sam; every defender out of the way; then his charge on the wall */
    const stage = async (silent = false) => {
      DESTRUCTION.on = true; WALLCHARGE.on = true;   // (the destruction switch raised as the wall charge's own probes raise it)
      const W: any = await quiet(() => standUp({ level: loadLevel(data), seed: 1 })); const g = W.game; g.godMode = true; g.tune.chaos = 0.5;
      const why = await quiet(async () => startFromQuery(g, '?levelFile=pass.json&roomclear&squad4').why); if (why) throw new Error(`start refused — ${why}`);
      if (silent) { SILENT.on = true; SNEAK.on = true; setSilent(g, true); }   // (the squad silent, as his was: the sneak's own carriage)
      const O = g.orders, C = g.wallCharge, nav = g.col.nav, men: any[] = O.men('all'), peak = new Map<any, number>();
      /** stepped a 60th at a time, each man's fastest metre a second kept (over a tenth of a second: a shove is not his pace) */
      const step = (secs: number) => { for (let i = 0; i < Math.round(secs * 6); i++) { const was = men.map(m => [m.char.pos[0], 0, m.char.pos[2]]);
        for (let k = 0; k < 10; k++) W.step(1 / 60); men.forEach((m, j) => peak.set(m, Math.max(peak.get(m) ?? 0, flat(m.char.pos, was[j]) * 6))); } };
      const put = (gd: any, p: number[]) => { gd.char.pos[0] = p[0]; gd.char.pos[2] = p[2]; gd.path = []; gd.pathGoal = null; gd.speed = 0; };
      for (const h of g.guards.filter((h: any) => h.team === 'hostile')) put(h, [h.char.pos[0] + 400, 0, h.char.pos[2]]);
      const Wl = wallAt(g, CHARGE), spot: Vec3 | null = Wl ? (C as any).setSpot(Wl) : null; if (!Wl || !Wl.ok || !spot) return null;
      const bunch = () => men.forEach((m, i) => { const b = BUNCH[i % BUNCH.length], q = nav.nearestFreePoint(spot[0] + b[0], spot[2] + b[1]); put(m, [q[0], 0, q[2]]); });
      const sam = (p: Vec3) => { g.teleportPlayer(p); };
      sam(spot); bunch(); step(0.5); O.give(BREAK ? 'hold' : 'fallIn', O.targetAt(g.player.char.pos), 'all'); bunch(); step(1.5); peak.clear();
      /** his charge on the wall, by his own hands (charge-rig.ts: one pushed onto the tools' copy by hand is a write the Rust game never gets); `at` = another place on it */
      const set = (at: any = Wl) => { const r = samSets(W, at); if (!r.charge && at === Wl) throw new Error(`Sam could not set his charge: ${r.why}`); set.why = r.why; return r.charge; }; set.why = '';
      /** …and off it again the only way a charge leaves a wall: set off by his own action, from out of its blast */
      const off = (c: any) => { const r = samBlows(W, c); if (!r.blown) throw new Error(`Sam could not set his charge off: ${r.why}`); };
      const hit = (p: number[]) => C.blastShare(Wl, p, undefined, false) > 0;
      const where = () => men.map(m => `${m.callsign} ${at(m.char.pos)}${hit(m.char.pos) ? ' IN' : ''}`).join(' · ');
      return { W, g, O, C, men, Wl, spot, step, put, sam, set, off, hit, where, peak };
    };

    const S = await stage(); if (!S) { line(false, 'the bunk room\'s long wall takes a charge from the yard'); return false; }
    const brisk: number = S.O.knobs.brisk, R = safeRadius(S.g, S.Wl);
    // (3) seconds after his charge goes on
    { const in0 = S.men.filter(m => S.hit(m.char.pos)).length, c = S.set(); let took = 0; while (took < 6 && S.C.tooClose(c).join() !== 'you') { S.step(0.5); took += 0.5; } S.step(6 - took);
      const close: string[] = S.C.tooClose(c), out = S.men.every(m => !S.hit(m.char.pos)), apart = S.men.every((m, i) => S.men.every((o, j) => j <= i || flat(m.char.pos, o.char.pos) > 0.5));
      const top = Math.max(...S.men.map(m => S.peak.get(m) ?? 0));
      const marks0 = S.men.map(m => { const a = (S.C as any).away.get(m); return a ? `${at(a.at)} ${flat(a.at, wallPt(S.Wl, 0, 0)).toFixed(2)} m` : '–'; }).join(' ');
      const off = S.men.every(m => S.C.blastShare(S.Wl, m.char.pos, { ...R, reach: R.standoff - 0.3 }, false) === 0);   // (at the stand-off or out of its line, not only past the reach)
      line(in0 >= 2 && close.join() === 'you' && out && off && apart && S.C.stats.cleared === in0 && top <= brisk * 1.1,
        'six seconds after his charge goes on the wall the row names nobody but him, every man stands at the stand-off or out of its line on a place of his own, each dealt a mark once, and nobody went faster than the pace a man steps out of Sam\'s way at',
        `in the blast before: ${in0} of ${S.men.length} · too close: ${close.join(', ') || 'nobody'} · ${S.where()} · at the stand-off: ${off} · marks ${marks0} · clear after ${took.toFixed(1)} s · stepped back ${S.C.stats.cleared} · fastest ${top.toFixed(2)} m/s (brisk ${brisk}) · reach ${R.reach} m, stand-off ${R.standoff} m`);
      // (4) Sam walks clear: they leave their marks and fall in on him
      const stood = S.C.stats.cleared; S.sam(AWAY); S.step(9); const held = (S.C as any).away.size, far = S.men.map(m => flat(m.char.pos, S.g.player.char.pos));
      line(stood >= 2 && held === 0 && far.every(d => d < 6), 'Sam well clear of it: nine seconds later nobody is kept on a mark and the four are with him', `stood off before: ${stood} · kept on marks: ${held} · from Sam: ${far.map(d => d.toFixed(1)).join(', ')} m`);
      // (5) Sam back at the wall: they stand off again
      S.sam(S.spot); S.step(9); const close2: string[] = S.C.tooClose(c);
      line(close2.join() === 'you' && S.men.every(m => !S.hit(m.char.pos)), 'Sam back at the foot of the wall: nine seconds later the row names nobody but him', `too close: ${close2.join(', ') || 'nobody'} · ${S.where()}`);
    }
  } finally { console.log = log; DESTRUCTION.on = destr0; Math.random = rnd0; }
  console.log(`=== menclear: ${all ? 'ALL PASS' : 'FAILURES above'} ===`);
  return all;
}
if (import.meta.main) process.exit((await menclearCheck()) ? 0 : 1);
