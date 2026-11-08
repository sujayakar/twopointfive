// EVERY THING THAT MOVES OVER LYING SNOW MARKS IT: a pushed prop's parts plough furrows, a thrown thing dents where it lands and lies in a small pit, a chunk of debris dents each time it comes
// down. How hard (`force`, the forming's and the kick's own scale) is by weight and speed. Only where snow lies, only on the ground. A thing on floor the team does not see marks nothing
// while it is there, and it is not caught up afterwards. The picture's own: it reads the things' rows and writes the prints' map; it rolls no die.
import { snowFieldAt, type SnowField } from '../../render/snowField';
import type { Drawn } from '../../draw/drawn';
import { Section, Thing } from '../../draw/sections';
import { stampMark, pressGroove, pressClods, hash3, type SnowTracks } from './tracks';

/** The marks' numbers (never seen on a GPU, never seen moving: the maintainer's to move; `__snow.marks` on the console, all LIVE — they reach the next mark).
 *  Authored data, not player switches. */
export const SNOWMARKS = {
  // ---- a pushed prop
  minV: 0.08,         // m/s: a thing moving slower than this cuts nothing new
  step: 0.05,         // m: a furrow is laid in strokes this long
  jump: 0.7,          // m: a thing found further than this from where it last was has been PUT there (a restart): no furrow between
  reach: 0.1,         // m: a part whose underside is within this of the floor reaches the snow
  leg: 0.07,          // m: a part no wider than this (half) either way is a LEG: it cuts to the floor whatever the thing weighs
  legMin: 0.028,      // m: a leg's groove is never thinner than this (half): the map's texel is 3 cm
  wide: 0.3,          // m: the widest stroke (half) a broad part ploughs with
  deepKg: 14,         // kg: a broad part presses to a boot's depth at this weight, and rides higher under it (a 3 kg chair's seat would: a fifth)
  mess: 0.7,          // 0‥1: how ragged a furrow's edges are (a print's own SNOWTRACKS.mess scale)
  clods: 1.6,         // lumps thrown aside per stroke, × force
  force0: 0.15,       // a thing's force: this …
  forceKg: 0.45,      // … + this × its weight ÷ deepKg (no more than 1) …
  forceV: 0.2,        // … + this × its speed in m/s (no more than 2)
  quick: 0.35,        // × the time a mark's floor takes to sink: a furrow is cut as the thing goes
  kickEvery: 0.14,    // m of travel between two kicks of loose snow off one thing
  // ---- a thrown thing, a piece of debris
  landV: 0.7,         // m/s: coming down slower than this it dents nothing (it was set down)
  roll: 0.15,         // m: a thing ROLLING over the snow lays its shallow groove in strokes this long …
  rollDeep: 0.35,     // … this deep of a boot's print (0‥1), with a walker's-toe of force
  dent: 1.5,          // × a thrown thing's own radius = its dent's
  least: 0.02,        // m: a dent smaller than this is not pressed (a spent case)
  most: 0.4,          // m: … and none is wider than this (half)
  skid: 0.6,          // m: two landings of one thing closer than this are joined by the groove it skidded
  rest: 1.15,         // × its radius: the pit a thrown thing lies in when it stops
  landForce: 0.15,    // a landing's force: this …
  landForceV: 0.13,   // … + this × how fast it came down, m/s …
  landForceR: 2.5,    // … + this × its size in m (so a grenade lobbed high lands about as hard as a sprinter's boot, a door's panel harder, a magazine hardly)
  itemMess: 0.5,      // 0‥1: how ragged a dent's lip is
  dentClods: 4,       // lumps thrown about a dent, × force
};

interface Seen { x: number; z: number; vy: number; sp: number; rest: boolean; mx: number; mz: number; marked: boolean; }
const clamp = (v: number, a: number, b: number) => Math.max(a, Math.min(b, v));
const lies = (field: SnowField, x: number, z: number) => snowFieldAt(field, x, z).cover >= 0.5;
type Dark = ((x: number, z: number) => boolean) | null;

/** one thing's landing: the dent, the skid from its last one, the lumps about it and the snow it kicks up */
function land(T: SnowTracks, K: typeof SNOWMARKS, s: Seen, x: number, z: number, r: number, v: number, mess: number) {
  if (r < K.least) return;
  const rr = Math.min(r, K.most), force = K.landForce + K.landForceV * v + K.landForceR * rr, serial = T.marks++;
  const skid = s.marked && Math.hypot(x - s.mx, z - s.mz) < K.skid ? [s.mx, s.mz] : null;
  const deep = clamp(0.35 + 0.5 * force, 0.3, 1);
  stampMark(T, force, (body, thrown) => {
    if (skid) pressGroove(body, skid[0], skid[1], x, z, rr * 0.7, rr * 0.7, deep * 0.6, mess);
    pressGroove(body, x, z, x, z, rr, rr, deep, mess);
    const nc = Math.round(K.dentClods * force); if (nc > 0) pressClods(thrown, x, z, nc, rr + 0.03, rr + 0.08 + 0.2 * force, 0, 0, serial);
  }, { x, z, wide: Math.max(0.08, rr) });
  s.mx = x; s.mz = z; s.marked = true;
}

export class SnowThings {
  private propSeen = new Map<number, Float32Array>(); private itemSeen = new Map<number, Seen>(); private chunkSeen = new Map<number, Seen>();
  constructor(private D: Drawn) {}
  /** a restart: nothing has been seen */
  clear(): void { this.propSeen.clear(); this.itemSeen.clear(); this.chunkSeen.clear(); }
  /** Once a tick, after the boots: the things' marks */
  update(T: SnowTracks, field: SnowField, dt: number, dark: Dark): void {
    if (!(dt > 0)) return;
    const D = this.D, f = D.f, u = D.u, n = D.n[Section.things], K = SNOWMARKS;
    for (let i = 0, o = D.at[Section.things]; i < n;) {
      const kind = u[o + Thing.kind], key = u[o + Thing.key];
      if (kind === 0) { let parts = 1; while (i + parts < n && u[o + parts * Thing.row + Thing.kind] === 0 && u[o + parts * Thing.row + Thing.key] === key) parts++; this.prop(T, field, dark, o, parts); i += parts; o += parts * Thing.row; continue; }
      const x = f[o + Thing.c], y = f[o + Thing.c + 1], z = f[o + Thing.c + 2], vx = f[o + Thing.vel], vy = f[o + Thing.vel + 1], vz = f[o + Thing.vel + 2], rest = (u[o + Thing.says] & 1) !== 0, on = (u[o + Thing.says] & 2) !== 0, sp = Math.hypot(vx, vz);
      if (kind === 1) this.thrown(T, field, dark, key, x, y, z, vy, sp, f[o + Thing.radius], rest, on);
      else this.chunk(T, field, dark, key, x, y, z, vy, sp, f[o + Thing.h], f[o + Thing.h + 1], f[o + Thing.h + 2], rest, on);
      i++; o += Thing.row;
    }
  }
  private prop(T: SnowTracks, field: SnowField, dark: Dark, o0: number, n: number): void {
    const D = this.D, f = D.f, u = D.u, K = SNOWMARKS, R = Thing.row, key = u[o0 + Thing.key], px = f[o0 + Thing.at], pz = f[o0 + Thing.at + 1];
    let st = this.propSeen.get(key);
    if (!st) { st = new Float32Array(n * 2 + 1); for (let i = 0; i < n; i++) { st[i * 2] = f[o0 + i * R + Thing.c]; st[i * 2 + 1] = f[o0 + i * R + Thing.c + 2]; } this.propSeen.set(key, st); return; }
    if (u[o0 + Thing.says] & 1) return;
    const hid = !!dark && dark(px, pz), vx = f[o0 + Thing.vel], vz = f[o0 + Thing.vel + 2], speed = Math.hypot(vx, vz), radius = f[o0 + Thing.radius], mass = f[o0 + Thing.mass], sp = speed + Math.abs(f[o0 + Thing.spin]) * radius;
    const heavy = clamp(mass / Math.max(0.1, K.deepKg), 0, 1), force = K.force0 + K.forceKg * heavy + K.forceV * Math.min(2, sp);
    let strokes: number[] | null = null, far = 0;
    for (let i = 0; i < n; i++) { const o = o0 + i * R;
      if (f[o + Thing.foot] > K.reach) continue;
      const x = f[o + Thing.c], z = f[o + Thing.c + 2], lx = st[i * 2], lz = st[i * 2 + 1], d = Math.hypot(x - lx, z - lz);
      if (d < K.step) continue;
      st[i * 2] = x; st[i * 2 + 1] = z;
      if (hid || d > K.jump || sp < K.minV || !lies(field, x, z)) continue;
      (strokes ??= []).push(i, lx, lz); if (d > far) far = d;
    }
    if (!strokes) return;
    const S = strokes, serial = T.marks++;
    st[n * 2] += far;
    let kick: { x: number; z: number; dx: number; dz: number; wide: number } | null = null;
    if (st[n * 2] >= K.kickEvery && sp > 1e-3) { st[n * 2] = 0; const ux = vx / Math.max(speed, 1e-3), uz = vz / Math.max(speed, 1e-3); kick = { x: px + ux * radius * 0.6, z: pz + uz * radius * 0.6, dx: ux, dz: uz, wide: Math.max(0.1, radius * 0.7) }; }
    stampMark(T, force, (body, thrown) => {
      for (let k = 0; k < S.length; k += 3) {
        const i = S[k], o = o0 + i * R, h0 = f[o + Thing.h], h2 = f[o + Thing.h + 2], x = f[o + Thing.c], z = f[o + Thing.c + 2], lx = S[k + 1], lz = S[k + 2];
        const along = h0 >= h2, long = Math.max(h0, h2), short = Math.min(h0, h2);
        if (long <= K.leg) { pressGroove(body, lx, lz, x, z, Math.max(K.legMin, short), Math.max(K.legMin, short), 1, K.mess * 0.6); }
        else {
          // a broad foot: its own footprint swept along, as a row of strokes down its long side
          const r = clamp(short, K.legMin, K.wide), run = Math.max(0, long - r), m = Math.min(5, 1 + Math.ceil(run / Math.max(r, 0.04)));
          const yaw = f[o + Thing.yaw], cs = Math.cos(yaw), sn = Math.sin(yaw), ax = along ? cs : sn, az = along ? -sn : cs;
          const deep = clamp(0.2 + 0.8 * heavy, 0.2, 1);
          for (let j = 0; j < m; j++) { const a = m > 1 ? (j / (m - 1) * 2 - 1) * run : 0; pressGroove(body, lx + ax * a, lz + az * a, x + ax * a, z + az * a, r, r, deep, K.mess); }
        }
        const nc = Math.round(K.clods * force * (0.5 + hash3(serial, i, 0x51))); if (nc > 0) pressClods(thrown, x, z, nc, Math.max(short, 0.05) + 0.03, Math.max(short, 0.05) + 0.1 + 0.12 * force, 0, 0, serial * 8 + i);
      }
    }, kick, K.quick);
  }
  private thrown(T: SnowTracks, field: SnowField, dark: Dark, key: number, x: number, y: number, z: number, vy: number, spd: number, radius: number, settled: boolean, on: boolean): void {
    const K = SNOWMARKS; let s = this.itemSeen.get(key);
    if (!s) { this.itemSeen.set(key, { x, z, vy, sp: spd, rest: settled, mx: 0, mz: 0, marked: false }); return; }
    const low = y - radius < K.reach && !on, S = s;
    if (low && lies(field, x, z) && !(dark && dark(x, z))) {
      // (a LANDING is "it was coming down and no longer is")
      const came = s.vy < -K.landV && vy > s.vy * 0.5;
      if (came) land(T, K, s, x, z, radius * K.dent, Math.hypot(s.vy, s.sp * 0.4), K.itemMess);
      else if (settled && !s.rest) { const r = radius * K.rest; if (r >= K.least) { T.marks++; stampMark(T, 0.3, body => { if (S.marked && Math.hypot(x - S.mx, z - S.mz) < K.skid) pressGroove(body, S.mx, S.mz, x, z, r * 0.7, r * 0.7, 0.4, K.itemMess); pressGroove(body, x, z, x, z, Math.min(r, K.most), Math.min(r, K.most), 0.6, K.itemMess * 0.6); }); s.mx = x; s.mz = z; s.marked = true; } }
      else if (!settled && s.marked && radius * 0.7 >= K.least) {
        // ROLLING on from its last mark: a shallow groove behind it, a stroke at a time (further off than a skid's reach it has been in the air: no groove across)
        const d = Math.hypot(x - s.mx, z - s.mz);
        if (d >= K.roll) { const r = Math.min(radius * 0.7, K.most), mx = s.mx, mz = s.mz; if (d < K.skid) { T.marks++; stampMark(T, 0.2, body => pressGroove(body, mx, mz, x, z, r, r, K.rollDeep, K.itemMess), null, K.quick); } s.mx = x; s.mz = z; }
      }
    }
    s.x = x; s.z = z; s.vy = vy; s.sp = spd; s.rest = settled;
  }
  private chunk(T: SnowTracks, field: SnowField, dark: Dark, key: number, x: number, y: number, z: number, vy: number, spd: number, h0: number, h1: number, h2: number, rest: boolean, on: boolean): void {
    const K = SNOWMARKS; let s = this.chunkSeen.get(key);
    if (!s) { this.chunkSeen.set(key, { x, z, vy, sp: spd, rest, mx: 0, mz: 0, marked: false }); return; }
    const hi = Math.max(h0, h1, h2), lo = Math.min(h0, h1, h2), mid = h0 + h1 + h2 - hi - lo, low = y < hi + K.reach && !on;
    if (low && lies(field, x, z) && !(dark && dark(x, z))) {
      const r = Math.sqrt(hi * mid);   // (it lies on its broad face)
      const came = s.vy < -K.landV && vy > s.vy * 0.5;   // (it was coming down and no longer is: it hit)
      if (came) land(T, K, s, x, z, r, Math.hypot(s.vy, s.sp * 0.4), K.itemMess);
      else if (rest && !s.rest && r >= K.least && !(s.marked && Math.hypot(x - s.mx, z - s.mz) < r * 0.5)) land(T, K, s, x, z, r, K.landV, K.itemMess);
    }
    s.x = x; s.z = z; s.vy = vy; s.sp = spd; s.rest = rest;
  }
}
