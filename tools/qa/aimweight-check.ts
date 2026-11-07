// THE PLAYER'S AIM HAS WEIGHT — the check behind src/game/aimweight.ts (AIMWEIGHT.on: on in the page, ?noaimweight; off on the frozen stand, TPS_AIMWEIGHT=1).
//   bun run tools/qa/aimweight-check.ts        (also `probes.ts aimweight`)
// One stood-up game (Sam alone in the corridor, the rifle up: fn2000-probes.ts stage), the switch thrown by hand, the cursor moved as a mouse moves it and
// every trigger pull a real click:
//   (1) a flick to the bearing behind him is a SWING: no frame turns the gun faster than the top rate × the gun's handling, it never passes the point, and it
//       lands exactly on the cursor's bearing; the time it took is printed
//   (2) the pistol swings faster than the rifle, empty hands faster than the pistol
//   (3) a small correction (10°) still lands in a quarter second
//   (4) the CIRCLE (aimGunPoint) trails the cursor's point through the swing and is the cursor's point, to the bit, once it has landed
//   (5) a round fired mid-swing goes where the circle is, not where the cursor is; a round fired once it has landed goes at the cursor as the build's did
//   (6) a sprint drives his facing itself: nothing is weighed, the circle is the cursor's, and the swing afterwards starts from rest
//   (7) a checkpoint taken mid-swing carries the swing: the frames after a load are the frames without one
//   (9) the shot rule asked of its own function: landed, a far level round and a launcher's lead on a crossing man come back untouched; mid-swing a target is
//       turned by exactly what the gun still owes, its range, height and lead kept
//   (10) the launcher: a 40 mm flicked and tapped mid-swing is NOT sent off the cursor — it waits, its arc drawn, and goes at the cursor the frame the circle
//       is on it (kept, unsent and undrawn, through a wall the gun comes down at on the way: the shipped stand); past `launchWait` it goes where the circle is; with `launchWait` 0 it leaves on the release frame, off the cursor, as the weight first shipped;
//       a tap with the circle settled goes on the release frame as ever
//   (8) the switch off: the gun eases onto the cursor by the build's own rule, there is no circle apart from the cursor, a click on the frame of a flick is
//       aimed at the cursor
// AIMWEIGHT_BREAK=1 = the switch left off for the 'on' lines: (1) (4) (5) (10) must fail.   exit 0 = all hold · 1 = one does not
import { resolve } from 'node:path';
const ROOT = resolve(import.meta.dir, '../..');

export async function aimweightCheck(): Promise<boolean> {
  const { stage, eventTap } = await import('./fn2000-probes.ts');
  const seeded = (a: number) => () => { a |= 0; a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ (a >>> 15), 1 | a); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; };
  const { AIMWEIGHT, aimGunPoint, aimGunOn } = await import(`${ROOT}/src/game/aimweight.ts`);
  const { handlingOf } = await import(`${ROOT}/src/game/weapons.ts`);
  const DT = 1 / 60; let all = true; const BREAK = process.env.AIMWEIGHT_BREAK === '1';
  const line = (ok: boolean, s: string, detail = '') => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${s}${detail ? `\n    ${detail}` : ''}`); };
  console.log('\n=== aimweight: the player\'s gun turns at a limited rate and the aiming circle trails the cursor ===');
  const AT = [21.5, 0, 11.5] as [number, number, number];
  const W: any = await stage(4301, true, AT); const { game, input } = W; const pl = game.player, c = pl.char;
  const was = { ...AIMWEIGHT }; AIMWEIGHT.on = !BREAK;
  const wrap = (a: number) => { a = (a + Math.PI) % (2 * Math.PI); if (a <= 0) a += 2 * Math.PI; return a - Math.PI; };
  const AHEAD = [AT[0] + 5, 1.2, AT[2]], BEHIND = [AT[0] - 5, 1.2, AT[2]];
  const wishYaw = () => Math.atan2(game.aimPoint[0] - c.pos[0], game.aimPoint[2] - c.pos[2]);
  const errNow = () => wrap(wishYaw() - c.aimYaw);
  const hold = (p: number[], n: number, click = false) => { for (let i = 0; i < n; i++) { W.cursorAt(p); if (click && i === 0) input.clicked |= 1; W.step(DT); } };
  const settle = (p: number[]) => { hold(p, 150); };
  /** flick the cursor to `p` and follow the swing: frames to land (|err| < 0.5°), the fastest frame (rad/s), whether it ever passed the point */
  const swing = (p: number[], cap = 240) => {
    let frames = -1, top = 0, passed = false, sign = 0, e0 = 0;
    for (let i = 0; i < cap; i++) {
      const y0 = c.aimYaw; W.cursorAt(p); W.step(DT);
      const e = errNow(); if (i === 0) { e0 = wrap(e + wrap(c.aimYaw - y0)); sign = Math.sign(e); } else if (sign && Math.sign(e) === -sign && Math.abs(e) > 2e-3) passed = true;
      top = Math.max(top, Math.abs(wrap(c.aimYaw - y0)) / DT);
      if (Math.abs(e) < 0.5 * Math.PI / 180) { frames = i + 1; break; }
    }
    return { frames, secs: frames / 60, top, passed, e0 };
  };
  const deg = (r: number) => (r * 180 / Math.PI).toFixed(1);
  try {
    // (1) the rifle, a half turn
    settle(AHEAD);
    const hR = Math.pow(handlingOf(pl.rifle.def), AIMWEIGHT.heft), capR = AIMWEIGHT.rate * hR;
    const s1 = swing(BEHIND); hold(BEHIND, 30); const landed = errNow();
    line(pl.rifleUp && s1.frames > 0 && s1.top <= capR * 1.03 && s1.top >= capR * 0.7 && !s1.passed && s1.secs >= Math.PI / capR && s1.secs < 0.9 && Math.abs(landed) < 1e-12,
      'the rifle up, the cursor flicked to the bearing behind him: the gun swings round at no more than the top rate × the rifle\'s handling, never passes the point, and lands exactly on the cursor\'s bearing',
      `half turn in ${s1.secs.toFixed(2)} s (${s1.frames} frames) · fastest frame ${s1.top.toFixed(2)} rad/s against a top of ${capR.toFixed(2)} (rate ${AIMWEIGHT.rate} × handling ${handlingOf(pl.rifle.def)} ^ ${AIMWEIGHT.heft}) · passed the point ${s1.passed} · left over ${landed}`);
    // (3) a small correction
    { settle(AHEAD); const p = [AT[0] + 3, 0, AT[2] + 3 * Math.tan(10 * Math.PI / 180)]; settle([AT[0] + 3, 0, AT[2]]); const s = swing(p);
      line(s.frames > 0 && Math.abs(s.e0) > 5 * Math.PI / 180 && s.secs <= 0.25 && !s.passed, 'a ten degree correction lands inside a quarter second, without passing the point', `${deg(Math.abs(s.e0))}° asked · ${s.secs.toFixed(2)} s · fastest frame ${s.top.toFixed(2)} rad/s`); }
    // (4) the circle trails, then IS the cursor's point
    { settle(AHEAD); let far = 0, offN = 0; const on0 = aimGunOn(game);
      for (let i = 0; i < 12; i++) { W.cursorAt(BEHIND); W.step(DT); const gp = aimGunPoint(game); if (gp) far = Math.max(far, Math.hypot(gp[0] - game.aimPoint[0], gp[2] - game.aimPoint[2])); if (!aimGunOn(game)) offN++; }
      hold(BEHIND, 120); const gp = aimGunPoint(game), ap = game.aimPoint;
      const same = !!gp && gp[0] === ap[0] && gp[1] === ap[1] && gp[2] === ap[2];
      line(on0 && far > 2 && offN >= 8 && same && aimGunOn(game), 'the circle (where the gun points) trails the cursor through the swing and is the cursor\'s own point, to the bit, once it has landed',
        `furthest apart ${far.toFixed(2)} m · off the cursor on ${offN} of the first 12 frames · landed on the same point ${same}`); }
    // (5) where a round goes
    { settle(AHEAD); pl.fireCd = 0; pl.rifle.chamber = 1; pl.rifle.mag = 20; const tap = eventTap(game); tap.evs.length = 0;
      hold(BEHIND, 3); W.cursorAt(BEHIND); input.clicked |= 1; W.step(DT);   // the click lands four frames into the swing
      const gunYaw = c.aimYaw, cur = wishYaw(); tap.pump(); const sh = tap.evs.filter((e: any) => e.kind === 'shot' && e.from && e.to).at(-1);
      const shotYaw = sh ? Math.atan2(sh.to[0] - sh.from[0], sh.to[2] - sh.from[2]) : NaN;
      const offGun = Math.abs(wrap(shotYaw - gunYaw)), offCur = Math.abs(wrap(shotYaw - cur));
      hold(BEHIND, 120); tap.evs.length = 0; pl.fireCd = 0; W.cursorAt(BEHIND); input.clicked |= 1; W.step(DT); tap.pump();
      const sh2 = tap.evs.filter((e: any) => e.kind === 'shot' && e.from && e.to).at(-1);
      const shot2 = sh2 ? Math.atan2(sh2.to[0] - sh2.from[0], sh2.to[2] - sh2.from[2]) : NaN; const off2 = Math.abs(wrap(shot2 - wishYaw()));
      line(!!sh && offGun < 0.2 && offCur > 1 && !!sh2 && off2 < 0.12, 'a round fired mid-swing goes where the circle is, not where the cursor is; fired once the circle has landed it goes at the cursor',
        `mid-swing: the round left ${deg(offGun)}° off the gun's heading and ${deg(offCur)}° off the cursor's bearing · landed: ${deg(off2)}° off the cursor's bearing`); }
    // (2) by weapon: pistol, then empty hands
    { settle(AHEAD); input.pressed.add('Digit1'); hold(AHEAD, 90); const pistol = pl.pistolUp; const sP = swing(BEHIND);
      settle(AHEAD); input.pressed.add('KeyE'); hold(AHEAD, 90); const bare = !pl.gunInHand(); const sB = swing(BEHIND);
      input.pressed.add('Digit2'); hold(AHEAD, 120);
      line(pistol && bare && sP.frames > 0 && sB.frames > 0 && sP.secs < s1.secs && sB.secs < sP.secs && sP.top <= AIMWEIGHT.rate * 1.03 && sB.top <= AIMWEIGHT.rate * AIMWEIGHT.bare * 1.03 && sP.top > s1.top * 1.15,
        'the pistol swings faster than the rifle, empty hands faster than the pistol',
        `half turn: rifle ${s1.secs.toFixed(2)} s · pistol ${sP.secs.toFixed(2)} s (fastest ${sP.top.toFixed(2)} rad/s) · empty hands ${sB.secs.toFixed(2)} s (fastest ${sB.top.toFixed(2)} rad/s)`); }
    // (6) a sprint is not weighed
    { settle(AHEAD); input.keys.add('ShiftLeft'); input.keys.add('KeyD'); let weighed = 0;
      for (let i = 0; i < 30; i++) { W.cursorAt(BEHIND); W.step(DT); if (aimGunPoint(game)) weighed++; }
      input.keys.delete('ShiftLeft'); input.keys.delete('KeyD'); const sprinted = Math.hypot(c.pos[0] - AT[0], c.pos[2] - AT[2]);
      W.cursorAt(BEHIND); W.step(DT);
      game.teleportPlayer(AT); hold(AHEAD, 150);
      line(sprinted > 0.5 && weighed === 0 && pl.aimw.live, 'a sprint drives his facing itself: nothing is weighed while it runs (the circle is the cursor\'s), and the frame he stops it is weighed again',
        `ran ${sprinted.toFixed(2)} m · weighed on ${weighed} of 30 sprint frames · weighed again on the first frame after ${pl.aimw.live}`); }
    // (7) a checkpoint mid-swing
    { settle(AHEAD); hold(BEHIND, 5); let s: any = null, why = '';
      try { s = W.snapshot('aimweight', true); } catch (e: any) { why = String(e?.message ?? e); }
      const run = () => { Math.random = seeded(77); const ys: number[] = []; for (let i = 0; i < 40; i++) { W.cursorAt(BEHIND); W.step(DT); ys.push(c0().aimYaw); } return ys; };
      const c0 = () => game.player.char;
      if (!s) line(false, 'a checkpoint taken mid-swing carries the swing', `no checkpoint: ${why}`);
      else {
        const carried = Array.isArray(s.player?.aw) && s.player.aw.length === 3; const a = run(); W.restore(s); const v = game.player.aimw.vel; const b = run();
        const bare = JSON.parse(JSON.stringify(s)); delete bare.player.aw; W.restore(bare); const n = run();   // the same file without the slice: the swing starts over from rest
        const apart = (x: number[], y: number[]) => { let m = 0; for (let i = 0; i < x.length; i++) m = Math.max(m, Math.abs(wrap(x[i] - y[i]))); return m; };
        const worst = apart(a, b), without = apart(a, n);
        line(carried && Math.abs(v) > 0.5 && worst < 1e-3 && without > 20 * worst && without > 0.02, 'a checkpoint taken mid-swing carries the swing: the forty frames after a load are the forty frames without one (to what a file keeps of his heading)',
          `slice in the file ${carried} · rate put back ${v.toFixed(2)} rad/s · largest difference in the gun's heading over 40 frames ${worst.toExponential(1)} rad · with the slice taken out of the file ${without.toExponential(1)} rad`);
      }
      hold(AHEAD, 150); }
  } finally { Object.assign(AIMWEIGHT, was); }
  console.log(all ? '\nALL PASS' : '\nFAILED');
  return all;
}

if (import.meta.main) { const ok = await aimweightCheck(); process.exit(ok ? 0 : 1); }
