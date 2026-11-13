// THE SNOW OF A LEVEL BUILT FOR IT, as the picture has it: the field it lies in, baked from the statics as told; the prints and marks pressed into it from the men's and the things' rows;
// the craters of the blasts the game told of; the flakes, wisps and breath in the air. Every part is the page's own: a restart begins it again, and a load begins it on the level as it stands.
import type { FollowCamera } from '../../host/camera';
import { buildSnowField, snowCfgRows, type SnowField } from '../../render/snowField';
import type { Particles } from '../../render/particles';
import type { SmokeSystemLike } from '../../smoke/types';
import type { Box } from '../../scene/boxes';
import type { Engine } from '../../engine';
import type { Drawn } from '../../draw/drawn';
import type { DrawWinds } from '../../draw/winds';
import { FootFlag } from '../../draw/bits';
import { SNOW_KINDS } from '../../draw/names';
import { Foot, Section, SnowBlast, SnowWord } from '../../draw/sections';
import type { AirPicture } from '../air';
import type { Roofs } from '../rain';
import { SnowBlasts } from './blast';
import { SnowBreath } from './breath';
import { SnowFx, wispLookByte } from './fx';
import { SnowThings } from './marks';
import { SnowSight } from './sight';
import { clearSnowTracks, formSnowTracks, makeSnowTracks, TRACK_RES, updateSnowTracks, type Man, type SnowTracks } from './tracks';
import { SNOW_LOOK } from './look';

/** the wind a snow level's ripples lie along (rust/game_rules/src/air/weather_kinds.rs WEATHER_PRESETS.snow) */
const RIPPLE: [number, number] = [0.8, 0.3];

/** the men's rows, as one object a man that lasts */
export function readMen(D: Drawn, L: Man[], made: Map<number, Man>): void {
  const f = D.f, u = D.u; L.length = D.n[Section.feet];
  const v = (m: number[], o: number) => { m[0] = f[o]; m[1] = f[o + 1]; m[2] = f[o + 2]; };
  for (let i = 0, o = D.at[Section.feet]; i < L.length; i++, o += Foot.row) {
    const id = u[o + Foot.id], says = u[o + Foot.says], z = (): number[] => [0, 0, 0];
    let m = made.get(id); if (!m) made.set(id, m = { id, alive: false, hidden: false, dead: false, pos: z(), vel: z(), bodyYaw: 0, hips: z(), head: z(), chest: z(), footL: z(), footR: z(), toeL: z(), toeR: z(), facing: z() });
    m.alive = (says & FootFlag.Alive) !== 0; m.hidden = (says & FootFlag.Hidden) !== 0; m.dead = (says & FootFlag.Dead) !== 0; m.bodyYaw = f[o + Foot.yaw];
    v(m.pos, o + Foot.pos); v(m.vel, o + Foot.vel); v(m.hips, o + Foot.hips); v(m.head, o + Foot.head); v(m.chest, o + Foot.chest); v(m.footL, o + Foot.footL); v(m.footR, o + Foot.footR); v(m.toeL, o + Foot.toeL); v(m.toeR, o + Foot.toeR); v(m.facing, o + Foot.facing);
    L[i] = m;
  }
}

/** every static is told what snow it is: the one reading the field, the air and the dig all ask */
export function sayOf(D: Drawn, B: Box[]): void { for (let i = 0, o = D.at[Section.snowWords], n = Math.min(B.length, D.n[Section.snowWords]); i < n; i++, o += SnowWord.row) { const w = D.u[o + SnowWord.w]; B[i].snow = w ? SNOW_KINDS[w - 1] : undefined; } }

export class SnowPicture {
  field: SnowField | null = null; tracks: SnowTracks | null = null;
  readonly blasts = new SnowBlasts(); readonly things: SnowThings; readonly breath: SnowBreath; readonly fx: SnowFx; readonly sight: SnowSight;
  private men: Man[] = []; private made = new Map<number, Man>(); private said = -1;
  constructor(private E: Engine, private D: Drawn, private boxes: Box[], P: Particles | null, smoke: SmokeSystemLike, private air: AirPicture, roofs: Roofs, winds: DrawWinds) {
    this.things = new SnowThings(D); this.breath = new SnowBreath(P, smoke, roofs.under); this.sight = new SnowSight(D, boxes);
    this.fx = new SnowFx(E, D, P, air, roofs, winds, this.blasts);
    this.say(); this.bake();
  }
  private say(): void { sayOf(this.D, this.boxes); }
  private bake(): void {
    const E = this.E;
    this.field = SNOW_LOOK.on ? buildSnowField({ boxes: this.boxes }, { wind: RIPPLE }) : null;
    this.tracks = SNOW_LOOK.tracks && this.field ? makeSnowTracks() : null;
    E.gbuffer.setSnowField(this.field, snowCfgRows(this.field, this.tracks ? TRACK_RES : 0));
    E.gbuffer.setSnowTracks(this.tracks);
  }
  /** the game is another moment than the one last drawn */
  cut(): void { this.said = -1; this.fx.cut(); this.breath.clear(); this.blasts.clear(); this.things.clear(); if (this.tracks) clearSnowTracks(this.tracks); }
  /** a tick has run */
  tick(dt: number, cam: FollowCamera): void {
    const D = this.D, F = this.field; this.say(); readMen(D, this.men, this.made); this.sight.fresh();
    for (let i = 0, o = D.at[Section.blasts]; i < D.n[Section.blasts]; i++, o += SnowBlast.row) { const id = D.u[o + SnowBlast.tick] * 4096 + D.u[o + SnowBlast.serial]; if (id <= this.said) continue; this.said = id;
      if (F) this.blasts.at(F, this.tracks, D.f[o + SnowBlast.at], D.f[o + SnowBlast.at + 1], D.f[o + SnowBlast.at + 2], D.f[o + SnowBlast.radius], (x, z, a, s, t) => this.air.puff(x, z, a, s, t), wispLookByte()); }
    this.fx.tick(dt, cam, this.tracks);
    this.breath.update(this.men, dt, cam.focus);
    if (!F) return;
    const E = this.E, dug = this.blasts.dig(F, this.boxes, dt); if (dug) E.gbuffer.setSnowFieldRect(F, dug);
    const T = this.tracks; if (!T) return;
    updateSnowTracks(this.men, T, F, dt, this.sight);
    if (SNOW_LOOK.marks) this.things.update(T, F, dt, this.sight.active ? (x, z) => this.sight.groundLook(x, z) !== 2 : null);
    formSnowTracks(T, dt);
    if (T.dirty) { if (T.rects.length > 0 && T.rects.length <= 24) for (const r of T.rects) E.gbuffer.setSnowTracks(T, r); else E.gbuffer.setSnowTracks(T, T.dirty); T.dirty = null; T.rects.length = 0; }
  }
  /** the frame's weather, the sky's section just written over it: the disc the flakes wrap in */
  dress(): void { const nf = this.E.weather.nearFade, d = this.fx.disc; nf[0] = d[0]; nf[1] = d[1]; nf[2] = d[2]; nf[3] = d[3]; }
}
