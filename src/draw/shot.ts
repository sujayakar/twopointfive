// WHAT THERE IS TO FRAME, as the camera that runs itself takes it (host/autoframe.ts AutoScene), from its row (rust/game_draw shot.rs). One scene is kept and written over: no object a tick.
import { AUTOCAM_STATS, type AutoScene } from '../host/autoframe';
import type { Vec3 } from '../math/vec';
import type { Drawn } from './drawn';
import { Section, Shot, ShotMan } from './sections';

export function shotScene(): AutoScene {
  return { sam: [0, 0, 0], threats: [], beat: null, door: null, room: null, zoomK: 1, hold: false, still: false, free: false, vel: [0, 0], fight: [], shown: 0, zoomFree: false, firedAgo: 0, lock: null, onto: false };
}

const three = (v: Vec3, f: Float32Array, o: number): Vec3 => { v[0] = f[o]; v[1] = f[o + 1]; v[2] = f[o + 2]; return v; };
/** a list of men on the floor, its arrays kept */
function men(to: Vec3[], D: Drawn, section: number): void {
  const n = D.n[section], at = D.at[section];
  to.length = Math.min(to.length, n);
  for (let i = 0; i < n; i++) { const o = at + i * ShotMan.row, v = to[i] ??= [0, 0, 0]; v[0] = D.f[o + ShotMan.x]; v[1] = 0; v[2] = D.f[o + ShotMan.z]; }
}
/** a thing framed: where it is and how much of the frame it has, or nothing */
const framed = (kept: AutoScene['beat'], D: Drawn, o: number, w: number): AutoScene['beat'] => {
  const share = D.f[w]; if (!(share > 0)) return null;
  const k = kept ?? { at: [0, 0, 0], w: 0 }; three(k.at, D.f, o); k.w = share; return k;
};

/** the row of the last tick, on `S`: false where there is none (the mode is off, or the view is not his own to frame) */
export function shotFrom(S: AutoScene, D: Drawn): boolean {
  if (!D.n[Section.shot]) return false;
  AUTOCAM_STATS.reads++;
  const o = D.at[Section.shot], f = D.f, u = D.u;
  three(S.sam, f, o + Shot.at);
  men(S.threats, D, Section.shotThreats); men(S.fight, D, Section.shotFight);
  S.beat = framed(S.beat, D, o + Shot.beat, o + Shot.beatW);
  S.door = framed(S.door, D, o + Shot.door, o + Shot.doorW);
  S.room = framed(S.room, D, o + Shot.room, o + Shot.roomW);
  S.zoomK = f[o + Shot.zoomK]; S.hold = u[o + Shot.hold] !== 0; S.still = u[o + Shot.still] !== 0; S.free = u[o + Shot.free] !== 0;
  S.vel[0] = f[o + Shot.vel]; S.vel[1] = f[o + Shot.vel + 1];
  S.shown = u[o + Shot.shown]; S.zoomFree = u[o + Shot.zoomFree] !== 0; S.firedAgo = f[o + Shot.firedAgo]; S.onto = u[o + Shot.onto] !== 0;
  const kind = u[o + Shot.lock];
  if (!kind) S.lock = null;
  else { const L = S.lock ??= { at: null, s: 0, stalk: false }; L.at = kind === 1 ? three(L.at ?? [0, 0, 0], f, o + Shot.lockAt) : null; L.s = f[o + Shot.lockS]; L.stalk = u[o + Shot.lockStalk] !== 0; }
  return true;
}
