// THE CAMERA RUNS ITSELF (host/autoframe.ts has the mode's own half: the table, the frame, the turn, the sway, the tuning). This is what the page before the one that draws knows of what there is to
// frame: the scene read off its copy of the game. The page that draws is told the same in a row (rust/game_draw shot.rs). This file goes with the copy.
import { smoothstep, type Vec3 } from '../math/vec';
import type { Game, Guard } from '../game/game';
import type { FollowCamera } from '../game/camera';
import type { Input } from '../game/input';
import type { Door } from '../game/doors';
import type { DoorOp } from '../game/doortools';
import { roomRead, type RoomRead } from '../game/roomread';
import { isHostile } from '../game/teams';
import { closeMode, closeSubject } from '../game/close';
import { stalkOn, stalkWait } from '../game/stalk';
import { samWork } from '../game/handwork';
import { AUTOCAM, AUTOCAM_STATS, frameBegin, frameRest, frameState, walkKey, type AutoFrame, type AutoScene } from './autoframe';
export * from './autoframe';

/** the mode's state, with what this page's read keeps of the game. Nothing of the game's but the soft lock's man, who is asked for again ten times a second and checked against the floor's men before he is used */
export interface AutoCam extends AutoFrame {
  /** the space round him as last read, the cell it was read from and when */
  space: RoomRead | null; cell: number; spaceT: number;
  /** …and the middle of that space where it is a room and not a hall, worked out as it was read */
  mid: Vec3 | null;
  /** hostile men the picture shows, as last counted */
  shown: number;
  /** each door order's beat: the phase it was last seen in and when that began */
  ops: Map<DoorOp, { phase: string; t: number }>;
  /** the way out that had the frame as his feet stopped: the opening's place and the floor past it */
  way: { at: Vec3; beyond: Vec3 } | null;
  /** who of the hostile men he could shoot, as last asked, and when */
  seen: Vec3[]; seenT: number;
  /** when each hostile man was last in his line, and the men of the fight as last asked */
  lineT: Map<object, number>; fight: Vec3[];
  /** the soft lock: its man as last asked (null: nobody, or a fight), whether it is a fight (somebody the picture shows is onto him, or was within `lockCalm`) and when somebody last was */
  lockGd: Guard | null; lockFight: boolean; lockFightT: number;
  /** the last scene read (the check's and the debugger's look) */
  scene: AutoScene | null;
}
export function autoCam(): AutoCam {
  return { ...frameState(), space: null, cell: -1, spaceT: -1e9, mid: null, shown: 0, ops: new Map(), way: null, seen: [], seenT: -1e9, lineT: new Map(), fight: [], lockGd: null, lockFight: false, lockFightT: -1e9, scene: null };
}
/** the mode came on: nothing of an earlier run is kept */
export function autoBegin(A: AutoCam, cam: FollowCamera): void { frameBegin(A, cam); A.way = null; A.ops.clear(); A.space = null; A.cell = -1; A.lineT.clear(); A.fight.length = 0; A.lockGd = null; A.lockFight = false; A.lockFightT = -1e9; }
/** the view is somebody else's for now */
export function autoRest(A: AutoCam, cam?: FollowCamera): void { frameRest(A, cam); A.way = null; A.lockGd = null; }

/** the view is Sam's own and the mode may frame it: no show's man in his place, nobody else's hands on him, and him on his feet */
export function autoFree(g: Game): boolean { return !g.touring && !g.puppet && !g.player.down && !g.rails?.skyOn; }

const flat = (a: Vec3, b: Vec3) => Math.hypot(a[0] - b[0], a[2] - b[2]);
/** the floor a stride past a door's frame, on the face away from `from` */
function past(d: Door, from: Vec3, m: number): Vec3 {
  const th = d.def.closedDir, nx = -Math.sin(th), nz = Math.cos(th), s = nx * (from[0] - d.def.hinge[0]) + nz * (from[2] - d.def.hinge[1]) >= 0 ? 1 : -1, c = d.frameCentre;
  return [c[0] - s * nx * m, 0, c[2] - s * nz * m];
}

/** THE SOFT LOCK'S MAN, where he stands NOW (he walks between the askings of read()): the man being stalked (`st`), or else the one the last asking
 *  chose — a man he could take (close.ts `closeSubject`) who is not onto him; with a man in Sam's arm, no place (the frame as it is) and the camera in
 *  full. Nobody in a fight, and but for a stalk nobody while he is not sneaking (the close-in mode's list, close.ts `modeNow`, less the wall hold —
 *  the wait at a corner is a sneak — and with a throw being aimed: kept by hand beside that list). `wait`: the held click is the corner wait's (it stills nothing, as a stalk's). */
export function autoLock(A: AutoCam, g: Game, st: Guard | null, wait = false): AutoScene['lock'] {
  const pl = g.player; if (A.lockFight || pl.down) return null;
  if (pl.holding) return { at: null, s: 1, stalk: false };
  const busy = g.touring || pl.rifleUp || pl.sprinting || !!pl.dragging || !!pl.kick || !!pl.picking || !!pl.wanding || !!pl.coding || !!samWork(g) || !!pl.throwPreview || !!pl.launchHeld;
  const ok = (m: Guard | null) => !!m && g.guards.includes(m) && closeSubject(g, m) && m.state !== 'alert';   // (still a man of this floor: a load swaps them between askings)
  const L = ok(st) ? st! : busy ? null : A.lockGd; if (!ok(L)) return null;
  const p = pl.char.pos, q = L!.char.pos; return { at: [q[0], 0, q[2]], s: smoothstep(AUTOCAM.lockM, AUTOCAM.lockNearM, flat(p, q)), stalk: L === st || wait };
}

/** THE SCENE, read off the game as the tick left it. It asks and writes nothing (the space round him is kept on `A`, asked again when he changes cell). */
export function autoRead(A: AutoCam, g: Game, input: Input, dt: number): AutoScene {
  const K = AUTOCAM, pl = g.player, c = pl.char, p = c.pos, now = g.time; AUTOCAM_STATS.reads++;
  // ---- the men he could shoot: hostile, on their feet, in the picture (the fog's word) and in his line — asked ten times a second
  if (now - A.seenT >= 0.1 || now < A.seenT) {
    A.seenT = now; A.seen.length = 0; A.fight.length = 0; A.shown = 0; if (A.lineT.size > g.guards.length) A.lineT.clear(); const eye: Vec3 = [p[0], 1.5, p[2]], fog = g.fogOfWar;
    let lockD = K.lockM, onto = false; const had = A.lockGd; A.lockGd = null;
    for (const gd of g.guards) {
      if (gd.state === 'dead' || gd.held || !isHostile('player', gd.team)) continue;
      if (gd.state === 'alert') onto = true;   // (the soft lock's fight: a man onto him anywhere, in the picture or not — the one shooting from the far end)
      const q = gd.char.pos; if (flat(p, q) > K.fightM || fog?.hidden(gd)) continue;
      const inLine = !g.ask.blocked(eye, [q[0], 1.3, q[2]]); if (inLine) { A.seen.push([q[0], 0, q[2]]); A.lineT.set(gd, now); }
      const last = A.lineT.get(gd); if (last !== undefined && now >= last && now - last <= K.swayLinger) A.fight.push([q[0], 0, q[2]]);   // (a man of the fight: the sway's)
      // the soft lock's man: the nearest in his line who is not onto him — the one it has until another is clearly nearer, and through a moment out of the line
      if (gd.state !== 'alert' && (inLine || gd === had && last !== undefined && now >= last && now - last <= K.lockLinger)) { const d = flat(p, q) - (gd === had ? K.lockStick : 0); if (d < lockD) { lockD = d; A.lockGd = gd; } }
      if (inLine || flat(p, q) <= K.nearM) A.shown++;   // (behind cover but this near: a step puts him in the line)
    }
    // (somebody is onto him, or was a moment ago: that is a fight, and the fight's rules frame it)
    if (onto || now < A.lockFightT) A.lockFightT = onto ? now : -1e9; A.lockFight = now >= A.lockFightT && now - A.lockFightT < (onto ? Infinity : K.lockCalm); if (A.lockFight) A.lockGd = null;
  }
  const lmb = input.lmb(), lock = autoLock(A, g, lmb ? stalkOn(g) : null, lmb && !!stalkWait(g));   // (a stalk, and the wait at a corner, live only under the held click)
  // ---- the squad: the newest beat — a door order's phase as it changes (the order given, its men set, the word, the entry), or men just sent somewhere
  let beat: AutoScene['beat'] = null;
  if (g.orders?.active) {
    let bestT = -1e9; const fade = (t: number) => { const age = now - t; return age < 0 ? 0 : age <= K.beatSecs ? 1 : Math.max(0, 1 - (age - K.beatSecs) / K.beatFade); };
    const ops = g.doorTools?.ops ?? [];
    for (const op of ops) {
      let B = A.ops.get(op); if (!B) A.ops.set(op, B = { phase: op.phase, t: Math.max(op.t0, now - K.beatSecs) }); else if (B.phase !== op.phase) { B.phase = op.phase; B.t = now; }
      const w = fade(B.t); if (w <= 0 || B.t <= bestT) continue;
      const walkUp = op.phase === 'stack' || op.phase === 'ready' || op.phase === 'place' || op.phase === 'standoff' || op.phase === 'armed';
      const at = walkUp ? [op.door.frameCentre[0], 0, op.door.frameCentre[2]] as Vec3 : past(op.door, op.lead.char.pos, K.beyondM); if (flat(p, at) > K.beatM) continue;
      beat = { at, w }; bestT = B.t;
    }
    if (A.ops.size > ops.length) for (const op of A.ops.keys()) if (!ops.includes(op)) A.ops.delete(op);
    // (men sent to a mark with no door order of their own: the middle of their marks)
    let n = 0, sx = 0, sz = 0, t0 = -1e9;
    for (const gd of g.orders.men()) { const S = g.orders.of(gd); if (!S?.post || fade(S.t0) <= 0 || ops.some(o => o.men.includes(gd))) continue; if (S.t0 > t0 + 1e-6) { t0 = S.t0; n = 0; sx = sz = 0; } if (S.t0 >= t0 - 1e-6) { n++; sx += S.post[0]; sz += S.post[2]; } }
    if (n && t0 > bestT) { const at: Vec3 = [sx / n, 0, sz / n]; if (flat(p, at) <= K.beatM) beat = { at, w: fade(t0) }; }
  } else if (A.ops.size) A.ops.clear();
  // ---- the space round him: asked again when he has changed cell (five times a second at the most), and once a second where he stands (a wall may have gone)
  const nav = g.col.nav, cell = Math.floor(p[0] / nav.cell) + Math.floor(p[2] / nav.cell) * nav.W, age = now - A.spaceT;
  if (!A.space || age < 0 || age >= 1 || (cell !== A.cell && age >= 0.2)) {
    const R = A.space = roomRead(g, p, 7); A.cell = cell; A.spaceT = now; A.mid = null;
    if (R.room > 0 && R.cells.length) {   // its middle, where it is a room and not a hall
      let x0 = Infinity, x1 = -Infinity, z0 = Infinity, z1 = -Infinity;
      for (const i of R.cells) { const x = (i % nav.W + 0.5) * nav.cell, z = (Math.floor(i / nav.W) + 0.5) * nav.cell; if (x < x0) x0 = x; if (x > x1) x1 = x; if (z < z0) z0 = z; if (z > z1) z1 = z; }
      if (Math.max(x1 - x0, z1 - z0) <= K.roomMaxM) A.mid = [(x0 + x1) / 2, 0, (z0 + z1) / 2];
    }
  }
  const R = A.space;
  // ---- the door: the one under his hand, or the way out he is walking at (kept while he stands by it)
  let door: AutoScene['door'] = null;
  const speed = Math.hypot(c.vel[0], c.vel[2]), still = !walkKey(input) && speed < 0.3;
  if (g.useDoor) { door = { at: past(g.useDoor, p, K.beyondM), w: 1 }; A.way = null; }
  else {
    let best = 0;
    if (speed > K.doorMps) for (const w of R.ways) {
      if (w.dist > K.doorM) continue; const dx = w.o.at[0] - p[0], dz = w.o.at[2] - p[2], d = Math.hypot(dx, dz); if (d < 1e-3) continue;
      const toward = (c.vel[0] * dx + c.vel[2] * dz) / (d * speed); if (toward < K.doorAim) continue;
      const s = Math.sqrt(Math.max(0, 1 - d / K.doorM)) * toward; if (s > best) { best = s; A.way = { at: [w.o.at[0], 0, w.o.at[2]], beyond: [w.beyond[0], 0, w.beyond[2]] }; }
    }
    if (A.way) { const d = flat(p, A.way.at); if (best > 0) door = { at: A.way.beyond, w: Math.min(1, best * 1.5) }; else if (d <= K.doorKeepM) door = { at: A.way.beyond, w: 0.8 }; else A.way = null; }
  }
  const room: AutoScene['room'] = A.mid ? { at: A.mid, w: 1 } : null, inRoom = !!A.mid;
  const sky = !g.weather?.underRoof(p[0], p[2]);
  const zoomK = (closeMode(g) || K.lockIn > 0 && K.zoomAmt > 0 && !!pl.holding && !A.lockFight ? K.closeK : inRoom ? K.roomK : sky ? K.skyK : 1) * (pl.sup.v > K.fireAt ? K.fireK : 1);
  const free = !pl.holding && !pl.dragging && !pl.throwPreview && !pl.launchHeld && !pl.wall && !g.rails?.locks('camera') && g.lesson?.step?.done?.is !== 'view';
  return A.scene = { sam: [p[0], p[1], p[2]], threats: A.seen, beat, door, room, zoomK, hold: input.buttons !== 0, still, free, vel: [c.vel[0], c.vel[2]], fight: A.fight, shown: A.shown, zoomFree: !g.rails?.locks('zoom'), firedAgo: now - pl.lastFireT, lock, onto: A.lockFight };
}
