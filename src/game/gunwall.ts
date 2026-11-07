import { SideMap, GameSideMap } from '../math/side';
import type { Vec3 } from '../math/vec';
import { v3, clamp, lerp } from '../math/vec';
import { BoxFlag } from '../scene/boxes';
import type { Game } from './game';
import type { Character } from './character';
import './character';
import { solidBetween } from './combat';
import { glassPenLive } from './glasspen';

/** A RAISED WEAPON COMES DOWN AT A WALL. A shouldered rifle's muzzle stands 0.63 m ahead of the man who holds it and an aimed pistol's 0.64; a man can stand
 *  with his centre 0.3 m off a wall — so every man who turned to cover a line with a wall, a jamb or a door leaf inside that reach was DRAWN with his gun in it
 *  or out through its far face (39 play-throughs of the one build: 3.1 % of all armed man-frames; nine in ten of them the shouldered carriage). Rounds have a
 *  rule for the pressed muzzle (combat.ts armThrough: the round dies in the panel) and so has the weapon light (lighting.ts LIGHTWALL); this is the GUN's:
 *
 *  1. THE LOW READY. A gun that is up (the aim; the high port of a stack or the high ready) and whose own line meets a static or a leaf inside its reach is
 *     brought to the low ready — through Guard.muzzleDown, the bit a colleague in the line already sets (guards.ts muzzleCheck): the pose, every fire gate and
 *     the visible laser read it, so nothing new has to learn about a lowered gun. Two lines are asked (walledNow): the gun AS DRAWN once it is up and settled
 *     (exact at a jamb's edge), and otherwise the carriage he WANTS, worked out from his feet and his aim (a gun already down would report nothing in the way
 *     if it were asked as drawn, and come straight back up) — with a lead for the speed he closes at and a band between down and up, so it comes down before
 *     the wall and does not flutter at the edge of the reach. A man whose point of aim IS that surface (within `atM` of where the line meets it: the shotgun
 *     on a lock — doortools.ts gunStep) keeps it up while the muzzle itself is short of it: that muzzle is there on purpose. One that would stand IN the
 *     surface comes down like any other (a man walking at the door he watches).
 *     A MAN IN A FIGHT (`hot`: the alert branch, a script's `over`) is treated more narrowly: his gun comes down only once it has stood IN a panel a round
 *     would have died in anyway (armThrough's own test), and he then moves for a line (guards.ts `poked`; roomfight.ts walks on / steps aside).
 *  2. THE TUCK. The low ready itself reaches 0.43‥0.51 m — still past a wall he is pressed to. CharacterAnimator.gunWall (0‥2, from how much room the low
 *     carriage's line has) pulls the low carriages in toward character.ts WALL_TUCK: at 1 the muzzle is inside his own collision circle, and from 1 to 2 the
 *     hand is drawn further back still (WALL_TUCK.over) for a torso twisted toward the wall.
 *
 *  Sam: the same test, the same low ready (player.ts), and no round while it is down — but for the BREACHING SHOTGUN LAID ON A LOCK (`breachAim`: the raised
 *  gun's own line meets a locked, latched leaf where a shell takes its keep): that gun stays up however close he stands, pressed to the leaf included, and the
 *  shell goes — as a teammate's on the same lock always did. A breach shot at a lock always fires. Pinned men are left alone (they cannot step for a line), and so is
 *  everybody while a showcase has the floor (its choreography was written against guns that stay up).
 *  Off (the table's own value, so a host that never says otherwise is unchanged): nothing asked, muzzleDown and gunWall never touched — the game as it was.
 *  The page turns it on (src/host/profile.ts pageSwitches; ?nogunwall = off) and so does the harness's shipped stand; its frozen stand holds it off
 *  (src/host/standup.ts; TPS_GUNWALL=1 turns it on there).
 *  `downM` / `upM`: the room past the wanted muzzle under which the gun comes down / over which it may come back up, metres. `leadS`: seconds of his closing
 *  speed added to both (the carriage takes about that long to change). `tuckM`: the room the tucked muzzle keeps; `tuckLeadS`: its lead.
 *  `fightS`: a fight anywhere on the floor (a man gone alert, a scripted man with one in his sights) makes EVERY man a man in a fight for this long after it
 *  was last seen — the calm man's margins bring a gun down a stride early, and a stride early at a jamb with defenders two rooms off is a shot not taken
 *  (0 = only the man who has a target himself).
 *  `mark`: the player's aim ring wears a bar while his gun cannot get a round out (player.ts gunShut; the overlay's, nothing in the game reads it — the
 *  words a dead click files are in no list the page draws). Off here; the page turns it on (?nogunmark = off). */
export const GUNWALL = { on: false, downM: 0.03, upM: 0.12, leadS: 0.2, atM: 0.5, tuckM: 0.04, tuckLeadS: 0.12, fightS: 5, mark: false };
/** QA: line tests made, guns brought down (spells) and for how long (man-seconds), frames a low carriage was pulled in — tools/qa/gunwall-watch.ts reads it; nothing in the game does */
export const GUNWALL_STATS = { asked: 0, lowered: 0, loweredS: 0, tucked: 0 };

/** which raised carriage is asked about: the aim (shouldered / the two-hand pistol aim) or the high port (the stack's, the high ready's) */
export type GunUp = 'aim' | 'port';
/** a gun found in a wall as drawn: where he stood (x, z), aimed (yaw, pitch) and how tall (the gun shoulder over his feet), the segment that found it, and by which test */
interface Hold { x: number; z: number; yaw: number; pitch: number; sh: number; a: Vec3; b: Vec3; hot: boolean }
interface St { low: boolean; /** Game.time of the last answer */ t: number; /** Sam's low ready is this rule's */ mine: boolean; hold: Hold | null;
  /** seconds a man in a fight has had his settled gun in a panel */ wood: number; /** this frame's answer let the gun go that the last one held */ rose: boolean;
  /** his feet's own speed over the floor (Character.vel is the COMMANDED velocity: a man queued behind another has it and stands still): the last sample and when */ px: number; pz: number; vt: number; vx: number; vz: number }
const ST = new SideMap<Character, St>('gunwall.ST');
const st = (c: Character): St => { let s = ST.get(c); if (!s) ST.set(c, s = { low: false, t: -1, mine: false, hold: null, wood: 0, rose: false, px: c.pos[0], pz: c.pos[2], vt: -1, vx: 0, vz: 0 }); return s; };

const STAND_SH = 1.4, CROUCH_SH = 0.99;   // the gun shoulder's height standing / crouched (the mannequin's, measured): how far down he is, for the carriages' crouched numbers
const TUCK_REACH = 0.235;                 // where WALL_TUCK leaves the muzzle ahead of his feet at gunWall 1 (measured: 0.21‥0.235 over the FN2000, the AKS-74U and the pistol)
const FRESH_S = 0.05;                       // an answer older than this is nobody's (a man nobody asked about this frame or last: dead, held, pinned)
const TELEPORT_M = 0.25;                    // further than this in a frame is not a step (teleportPlayer, a staged man): no speed read off it
/** the floor's fight: when one was last seen, asked once a frame (GUNWALL.fightS) */
const FIGHT = new GameSideMap<Game, { t: number; until: number }>('gunwall.FIGHT');

/** His frame: forward and right on the floor along `yaw`, the gun shoulder's height over his feet, how crouched that says he is (0‥1) */
function frame(c: Character, yaw: number) {
  const f: Vec3 = [Math.sin(yaw), 0, Math.cos(yaw)], r: Vec3 = [-Math.cos(yaw), 0, Math.sin(yaw)];
  const sh = (c.bones.upperArmR?.[1] ?? c.pos[1] + STAND_SH) - c.pos[1];
  return { f, r, sh, k: clamp((STAND_SH - sh) / (STAND_SH - CROUCH_SH), 0, 1) };
}
const at = (o: Vec3, F: { f: Vec3; r: Vec3 }, fwd: number, right: number, y: number): Vec3 =>
  [o[0] + F.f[0] * fwd + F.r[0] * right, o[1] + y, o[2] + F.f[2] * fwd + F.r[2] * right];
/** his feet's speed over the floor, sampled once a frame by whoever asks first */
function track(g: Game, c: Character, S: St): void {
  if (S.vt === g.time) return;
  const h = g.time - S.vt;
  const dx = c.pos[0] - S.px, dz = c.pos[2] - S.pz;
  if (h > 1e-5 && h < 0.1 && Math.hypot(dx, dz) < TELEPORT_M) { S.vx = dx / h; S.vz = dz / h; } else S.vx = S.vz = 0;   // (a gap, the clock gone back — a load —, or put somewhere else: standing)
  S.px = c.pos[0]; S.pz = c.pos[2]; S.vt = g.time;
}
/** how fast he closes along `f`, m/s (never negative: backing off buys nothing early) */
const closing = (S: St, f: Vec3) => Math.max(0, S.vx * f[0] + S.vz * f[2]);

/** The line a RAISED gun lies along on heading `yaw`, from his feet (metres off the measured carriages, standing → crouched): `a` at the hand end, `b` at the
 *  muzzle. The aim: level with his eye line, on his gun side, pitched as he aims. The port: the long gun slanting up across his chest, the pistol level at
 *  the chin. These are COPIES of what character.ts's carriage tables (LONG, LONG_CROUCH, STANCES) come out as on the mannequin — `probes.ts gunwall`'s
 *  gun-model card holds them to the posed gun, so a re-tuned table that moves a muzzle is a red there and not a rule that silently stopped fitting. */
function raisedLine(c: Character, up: GunUp, yaw: number, o: Vec3 = c.pos): { a: Vec3; b: Vec3 } {
  const F = frame(c, yaw), long = c.anim.longGun;
  if (up === 'aim') {
    const reach = (long ? 0.11 : 0.385) + c.muzzleAhead, side = long ? lerp(0.2, 0.16, F.k) : lerp(0.16, 0.02, F.k), y = F.sh - (long ? 0.11 : lerp(0.02, 0.22, F.k));
    const p = clamp(c.aimPitch, -0.7, 0.7);
    return { a: at(o, F, 0, side, y), b: at(o, F, reach * Math.cos(p), side, y + reach * Math.sin(p)) };
  }
  return long ? { a: at(o, F, 0.15, 0.12, F.sh - 0.28), b: at(o, F, 0.44, -0.065, F.sh + 0.13) } : { a: at(o, F, 0, 0.07, F.sh + 0.04), b: at(o, F, 0.465, 0.07, F.sh + 0.07) };
}
/** how far ahead of his feet the low ready leaves the muzzle (untucked) */
const lowReach = (c: Character) => c.anim.longGun ? 0.145 + 0.7 * c.muzzleAhead : 0.241 + 0.74 * c.muzzleAhead;
/** QA (tools/qa/gunwall-probes.ts gun-model): the worked-out carriages, to be held against the posed gun */
export const gunWallModel = { raisedLine, lowReach, TUCK_REACH };

/** …the same answer with nothing counted (player.ts gunShut: the overlay asks it every frame, and a QA tally is not the picture's to move) */
export function woodBetween(g: Game, a: Vec3, b: Vec3): boolean {
  if (!solidBetween(g, a, b)) return false;
  if (!glassPenLive(g)) return true;
  const d = v3.sub(b, a), L = v3.len(d), h = L > 1e-3 ? g.col.raycast(a, v3.scale(d, 1 / L), L) : null;
  return !(h && h.index < 1e6 && (g.col.boxes[h.index].flags & BoxFlag.Glass));
}

/** the last answer, read only, if it is this frame's or the last's (roomfight.ts: a gun on its man that a wall holds down is no shot from here) */
export function gunWallIs(g: Game, c: Character): boolean {
  if (!GUNWALL.on) return false;
  const S = ST.get(c); return !!S && S.low && g.time - S.t < FRESH_S;
}
/** checkpoints (savestate.ts): whose gun the wall holds down right now and what holds it — the band's memory, so a loaded game answers as the saved one
 *  would have. A row a man: his character id, then (if he has one) his `hold`'s twelve numbers. Only answers still fresh: nobody dead, held or pinned.
 *  A last row [-1, seconds] while the floor's fight is still remembered. */
export function gunWallSnapshot(g: Game): number[][] | undefined {
  if (!GUNWALL.on) return undefined;
  const rows: number[][] = [];
  for (const c of [g.player.char, ...g.guards.map(gd => gd.char)]) {
    if (!c.alive || !gunWallIs(g, c)) continue;
    const H = ST.get(c)!.hold; rows.push(H ? [c.id, H.x, H.z, H.yaw, H.pitch, H.sh, ...H.a, ...H.b, H.hot ? 1 : 0] : [c.id]);
  }
  const F = FIGHT.get(g); if (F && F.until > g.time) rows.push([-1, F.until - g.time]);   // (the floor's fight, still remembered: a row of its own under an id nobody has)
  return rows.length ? rows : undefined;
}
/** a restart (cleanrestart.ts): the fight the floor remembered was the last try's */
export function gunWallFresh(g: Game): void { FIGHT.delete(g); }
export function gunWallRestore(g: Game, rows: number[][] | undefined): void {
  if (!GUNWALL.on) return;
  const fr = rows?.find(x => x[0] === -1); FIGHT.set(g, { t: -1, until: fr ? g.time + fr[1] : -1 });
  for (const c of [g.player.char, ...g.guards.map(gd => gd.char)]) {
    const r = rows?.find(x => x[0] === c.id), S = r ? st(c) : ST.get(c); if (!S) continue;
    S.low = !!r; S.t = r ? g.time - 1e-3 : -1;   // (fresh, and not yet this frame's answer)
    S.wood = 0; S.vt = -1; S.rose = false;
    S.hold = r && r.length >= 13 ? { x: r[1], z: r[2], yaw: r[3], pitch: r[4], sh: r[5], a: [r[6], r[7], r[8]], b: [r[9], r[10], r[11]], hot: r[12] === 1 } : null;
  }
}
