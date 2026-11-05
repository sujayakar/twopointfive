// A LOOK AHEAD THAT WRITES NOTHING — where a seat's man would be after records the game has not had yet. A client of a networked game is shown the game as the
// server last answered it, a few ticks behind its own keys; its own man is drawn where those keys will have taken him, or the game feels late (src/net/). This is
// the one call that answers it.
//
// It is a walk on a COPY of him, by the tick's own lines: player.ts hands its stride, stance and facing a body (`Legs`) — the tick hands them the player, this
// a copy of his place, speed, facing and stance — and aimweight.ts its swing; each record is taken as a tick would take it (cmd.ts honestCopy) at the clock that
// tick will run at, with the world asked aside (Game.aside). Nothing of the game is written: not the man, not his seat, not a table, not the records handed in.
// The first cut stepped the real man through updatePlayer and put him back, which was exact for a walk and wrong the tick something of his came due inside it
// (a can leaving his hand was spawned into the world and never taken back: two machines went out of step) — a function that cannot write has no such tick.
//
// WHAT IT WALKS: a man whose legs are his own (player.ts legsOwn) — the walk, the sprint, the crouch and the quick crouch walk, into walls and frames and the
// men in his way as the tick's own stride meets them. WHAT IT DOES NOT: while his legs are not his own he is answered where he stands (`free` false); a beat
// that begins inside the records (a kick, a grab, a wall taken by leaning on it, the selector opened) is walked through until the game has it; a door's leaf
// and loose furniture do not stop him (the tick meets those after the player's step, in the doors' and the props' own: a run at a shut leaf is answered
// through it until the game says otherwise); his aim goes to the cursor's own point, not to the man or the lamp the reticle would snap to (the snap is the
// tick's: it writes the reticle); the wheel's pace notch and a shot fired inside the records are not read. The ORDER the shared pieces are called in is
// written here and in updatePlayer: a rule added between them there is added here.
// Check: tools/qa/seats-check.ts (`ahead`: a walk, a sprint and a crouch land where the whole game then puts him, to the bit; `twin`: a fight played with and
// without a look ahead every tick is the same game, word for word).
import { Seat, PlayerCmd, honestCopy, loadLens } from './cmd';
import { Vec3, wrapAngle } from '../math/vec';
import { Legs, legsOwn, keysWay, crouchKey, sprintStance, strideStep, sprintFacing, hipsTurn, gunRaised, leafFacing, aimEase, aimLevel, cursorHit, cursorPoint, TWOGEARS } from './player';
import { aimWeighs, aimTop, aimSwing } from './aimweight';
import { noShoveBegin } from './noshove';
import { samRooted } from './samstun';
import { isHurt } from './condition';
import { TAPSTEP } from './tapstep';

/** where he would be: copies, the caller's to keep */
export interface Ahead {
  pos: Vec3; vel: Vec3; aimYaw: number; bodyYaw: number; crouch: boolean;
  /** what his seat's camera would follow with him there (Game.followPos) */
  follow: Vec3;
  /** the world point under the newest record's cursor (the tick's aim before the reticle snaps it to a man or a lamp) — with no record, the aim the game has for him */
  aim: Vec3 | null;
  /** false: his legs are not his own (player.ts legsOwn) and every answer is where the game has him */
  free: boolean;
}

/** `seat`'s man walked through `cmds` (oldest first, `dt` seconds each) from where the game has him. `path`, when given, is filled with his place after each
 *  record (`path[i]` after `cmds[i]`; its length is set). No record: where the game has him. The game, the seat and the records are as they were on return. */
export function walkAhead(seat: Seat, cmds: readonly PlayerCmd[], dt: number, path?: Vec3[]): Ahead {
  const v = seat.view, pl = v.player, ch = pl.char, n = cmds.length, free = legsOwn(v);
  const m: Legs = { char: { pos: [ch.pos[0], ch.pos[1], ch.pos[2]], vel: [ch.vel[0], ch.vel[1], ch.vel[2]], bodyYaw: ch.bodyYaw, aimYaw: ch.aimYaw }, safe: pl.safe ? [pl.safe[0], pl.safe[1], pl.safe[2]] : null,
    crouch: pl.crouch, sprinting: pl.sprinting, dash: pl.dash, backpedal: pl.backpedal, speedSm: pl.speedSm, pace: TWOGEARS.on ? 1 : pl.pace, dragging: null, haul: null,
    shoved: [0, 0], shovedT: -1, shovedFrom: [0, 0], down: pl.down, doorRipUntil: pl.doorRipUntil, doorBashAt: pl.doorBashAt, doorBashLean: pl.doorBashLean };
  let aim: Vec3 | null = v.aimPoint ? [v.aimPoint[0], v.aimPoint[1], v.aimPoint[2]] : null;
  if (path) path.length = n;
  if (free && n > 0) v.aside(() => {
    const c = m.char, cam = v.asideLens(), swing = { vel: pl.aimw.vel }, weighs = aimWeighs(v), top = weighs ? aimTop(pl) : 0; let weighed = pl.aimw.live, now = v.time;
    for (let i = 0; i < n; i++) {
      now += dt;   // (the tick's clock has moved on before it reads him: game.ts stepSeats)
      const r = honestCopy(cmds[i], seat.remote ? c.pos : null), has = (k: string) => r.keys.includes(k);
      loadLens(cam, r.view);
      // the cursor's point, before he moves (updateAim)
      const u = r.mx / Math.max(1, r.w), w = r.my / Math.max(1, r.h), ray = cam.ray(u, w);
      aim = cursorPoint(cam, u, w, ray, cursorHit(v, ray, ch.id)) ?? aimLevel(c);
      // his keys, his stance, his stride (updatePlayer's movement block for a man whose legs are his own)
      noShoveBegin(v, m);
      let mx = 0, mz = 0;
      if (!pl.listOpen && !samRooted(v, now)) { const { fwd, right } = cam.planarBasis(); [mx, mz] = keysWay(fwd, right, has('KeyW'), has('KeyS'), has('KeyD'), has('KeyA')); }
      const ml = Math.hypot(mx, mz); if (ml > 0) { mx /= ml; mz /= ml; }
      if (r.pressed.includes('KeyC')) crouchKey(v, m);
      const wantSprint = !isHurt(v, pl, now) && (has('ShiftLeft') || has('ShiftRight')) && ml > 0;
      sprintStance(v, m, wantSprint, false);
      if (TAPSTEP.on && m.crouch && !wantSprint) { c.vel = [0, 0, 0]; m.speedSm = 0; }   // (tapstep.ts: a crouch that begins inside the records — his keys are steps from here, not walked)
      else strideStep(v, m, mx, mz, ml, null, null, dt, now);
      // his facing: the run's in a sprint, else the aim's swing to the cursor and the hips after it
      if (m.sprinting) { sprintFacing(v, m, dt, now); weighed = false; }
      else { const dx = aim[0] - c.pos[0], dz = aim[2] - c.pos[2], d = Math.hypot(dx, dz);
        if (weighs) { if (!weighed) swing.vel = 0; weighed = true;
          if (d > 0.25 && dt > 0) { const err0 = wrapAngle(Math.atan2(dx, dz) - c.aimYaw); c.aimYaw += err0 - aimSwing(swing, err0, top, dt); } else swing.vel = 0; }
        else if (d > 0.25) c.aimYaw = aimEase(c.aimYaw, [dx, 0, dz], dt);
        const aiming = gunRaised(v, pl, now, m);
        hipsTurn(m, Math.hypot(c.vel[0], c.vel[2]), aiming, false, leafFacing(ch.anim.doorPush, aiming), false, dt); }
      if (path) path[i] = [c.pos[0], c.pos[1], c.pos[2]];
    }
  });
  else if (path) for (let i = 0; i < n; i++) path[i] = [ch.pos[0], ch.pos[1], ch.pos[2]];
  const c = m.char, f = v.followPos(c.pos);
  return { pos: c.pos, vel: c.vel, aimYaw: c.aimYaw, bodyYaw: c.bodyYaw, crouch: m.crouch, follow: [f[0], f[1], f[2]], aim, free };
}
