// A BLAST IN THE SNOW, picture only. One blast over lying snow: THE CRATER into the prints' map; THE DIG where snow stands (a drift's hill, a bank against a wall), out to the ground or to
// whatever of it still stands solid; THE THROW, the loose snow about it going up as a burst of flakes; THE PUFF, a ring of powder low over the ground; THE AIR, a short hard shove to the
// wind's simulation. Every blast the game told of is heard (the `blasts` section); nothing here asks what threw it, and nothing of the game reads it.
import { clamp } from '../../math/vec';
import type { Particles } from '../../render/particles';
import { snowFieldAt, digSnowField, restoreSnowField, type SnowField } from '../../render/snowField';
import type { Box } from '../../scene/boxes';
import { pressCrater, type SnowTracks } from './tracks';
import { SNOW_LOOK } from './look';

/** The blast's numbers in snow. Authored data, not player switches — all LIVE (a blast reads them when it goes off). */
export const SNOWBLAST = {
  // ---- how big: the pan's radius = the blast's own reach × sizeK, held between the two, and less the higher over the snow it went off
  sizeK: 0.26,        // × the blast's destruction radius (a frag's 2.5 m → a pan 0.65 m across the half)
  sizeMin: 0.3,       // m
  sizeMax: 1.4,       // m
  high: 1.6,          // m over the snow at which a blast no longer marks it (0.3 m and under: the whole of it)
  // ---- the crater in the prints' map (snowTracks.ts pressCrater)
  lobe: 0.22,         // how far the pan's outline wanders off a circle (three slow lobes)
  rag: 0.07,          // … and its fine ragged edge
  floor: 1,           // how deep the pan is pressed (1 = to the prints' own floor)
  rim: 2.2,           // × a print's bank: the lip thrown up round it
  rays: 13,           // scoured grooves running out of it
  rayLen: 1.9,        // × the pan's radius: the longest of them
  rayWide: 0.17,      // × the pan's radius: their half width at the root
  clods: 46,          // lumps thrown about it
  clodFar: 3.2,       // × the pan's radius: how far the farthest lands
  clodR: 0.1,         // m: the biggest clod's radius
  // ---- the dig: the standing snow about it (snowField.ts digSnowField)
  dig: 1.5,           // × the pan's radius: a hill is dug to nothing this far out (and is itself again half as far on); 0 = hills are left standing
  digS: 1.5,          // s the dig is asked again each frame (a charge's hole takes the bank's foot a frame or more after the blast: what stood solid then no longer does); the texels are walked again only when what stands there changed
  // ---- the throw: near grains (they must be running: SNOW_LOOK.grains and wind enough to have any lying)
  throwS: 0.14,       // s the blast goes on taking lying grains
  throwShare: 0.5,    // the share of all lying grains it takes over that time
  throwR: 1.3,        // × the pan's radius: the disc they are lifted from
  out: 7,             // m/s outward, the fastest
  up: 3.2,            // m/s upward, the fastest
  // ---- the puff of powder: wisp records
  puff: 28,           // wisps a blast files (fewer when the ring is short of room)
  puffR: 1.2,         // × the pan's radius: the disc they are born over
  puffLife: 1.5,      // s
  puffSize: 0.8,      // m across
  puffOut: 4,         // m/s outward, the fastest (the air has them inside a fifth of a second)
  // ---- the shove to the wind's simulation
  air: 2.2,           // m/s of outflow at the puff's edge (× the game's air, as every source is: the grains fly at SNOWGRAINS.speedK × it)
  airR: 1.5,          // × the pan's radius: the puff's size
  airS: 0.3,          // s it lasts, falling off to nothing
  // ---- ONE SNOW IN ONE AIR (SNOW_LOOK.oneAir: there are no grains to throw): the loose snow goes up as a burst of FLAKES, the very snow that is falling, and the
  // shove above carries them out and round what they meet; they come down at a flake's own fall
  flakes: 180,        // flakes a blast throws up (fewer when the ring is short of room)
  flakeUp: 7,         // m/s upward, the fastest (a flake's own drag has it stopped inside a fifth of a second, a hand to a metre up)
  flakeLife: 3,       // s
};

/** a blast of the last half second: where, how big, how low, how old; what it still owes the particle pool */
interface Live { x: number; z: number; size: number; hk: number; t: number; serial: number; puffLeft: number; flakeLeft: number }
const LIVE_S = 0.5;
interface Dig { x: number; z: number; r: number; serial: number; t: number }

export class SnowBlasts {
  private live: Live[] = []; private digs: Dig[] = []; private restore = false; private serial = 0;
  /** the read-out a check takes */
  craters = 0; dug = 0;
  /** a restart: nothing is left in the air, and the hills as they were baked */
  clear(): void { this.live.length = 0; this.digs.length = 0; this.restore = true; }
  /** A blast of destruction radius `radius` at `at`: mark the snow if it went off over lying snow, low enough. True = it did. `puff` shoves the air */
  at(F: SnowField, T: SnowTracks | null, x: number, y: number, z: number, radius: number, puff: (x: number, z: number, out: number, sigma: number, secs: number) => void, wisp: number): boolean {
    const K = SNOWBLAST; if (!SNOW_LOOK.blast) return false;
    const size0 = clamp(radius * K.sizeK, K.sizeMin, K.sizeMax);
    // lying snow under it: at its middle, or under most of a ring a pan's width out (a blast against a wall's foot, in a doorway)
    const mid = snowFieldAt(F, x, z);
    let lies = mid.cover >= 0.5 ? 6 : 0;
    if (!lies) for (let k = 0; k < 6; k++) { const a = k * Math.PI / 3 + 0.4; if (snowFieldAt(F, x + Math.cos(a) * size0 * 0.7, z + Math.sin(a) * size0 * 0.7).cover >= 0.5) lies++; }
    if (lies < 3) return false;
    const hk = clamp(1 - (y - mid.mound - 0.3) / Math.max(0.1, K.high - 0.3), 0, 1);
    if (!(hk > 0.05)) return false;
    const size = size0 * (0.55 + 0.45 * hk), serial = ++this.serial;
    if (T) { pressCrater(T, x, z, size, serial, K); this.craters++; }
    if (K.dig > 0) { this.digs.push({ x, z, r: size * K.dig, serial, t: 0 }); if (this.digs.length > 16) this.digs.shift(); }
    this.live.push({ x, z, size, hk, t: 0, serial, puffLeft: Math.max(0, Math.round(K.puff * hk)), flakeLeft: Math.max(0, Math.round(K.flakes * hk)) });
    if (this.live.length > 8) this.live.shift();
    puff(x, z, K.air * hk, size * K.airR, K.airS);
    this.wispByte = wisp;
    return true;
  }
  private wispByte = 0;
  /** Once a tick, late: the hills a blast dug out. Returns the field's texels that changed, for the g-buffer, or null. A dig is asked again for SNOWBLAST.digS: the snow box that stood solid under the hill when the blast went off may be taken a tick or more later */
  dig(F: SnowField, boxes: readonly Box[], dt: number): [number, number, number, number] | null {
    let out: [number, number, number, number] | null = null;
    if (this.restore) { this.restore = false; if (restoreSnowField(F)) out = [0, 0, F.w - 1, F.h - 1]; }
    if (!this.digs.length) return out;
    for (const d of this.digs) {
      const r = digSnowField(F, boxes, d.x, d.z, d.r, d.serial, true);
      if (r) { this.dug++; out = out ? [Math.min(out[0], r[0]), Math.min(out[1], r[1]), Math.max(out[2], r[2]), Math.max(out[3], r[3])] : r; }
      d.t += Math.max(dt, 1e-4);
    }
    this.digs = this.digs.filter((d) => d.t < SNOWBLAST.digS);
    return out;
  }
  /** once a tick, before anything is filed: the live blasts age */
  step(dt: number): void {
    if (!this.live.length) return;
    for (const b of this.live) b.t += Math.max(0, dt);
    this.live = this.live.filter((b) => b.t < Math.max(LIVE_S, SNOWBLAST.throwS + 1e-6));
  }
  /** THE PUFF: what each live blast still owes of its powder, filed as wisp records as far as the tick's births allow. Returns how many it filed. */
  puffs(Pp: Particles, room: number): number {
    const K = SNOWBLAST; let filed = 0;
    for (const b of this.live) {
      const n = Math.min(b.puffLeft, Math.max(0, room - filed)); if (n <= 0) continue;
      const got = Pp.emit('rain', [b.x, 0.5, b.z], [0, -1, 0], n, { shape: 'rainDisc', radius: Math.max(0.2, b.size * K.puffR), speed: K.puffOut, speedJit: 0.5, life: K.puffLife, lifeJit: 0.35,
        size: K.puffSize, sizeJit: 0.4, lift: 0.45, aspect: 2, color: [0.04, 0.15, 0], top: this.wispByte, seed: (0x5b1a57 + b.serial * 7919 + b.puffLeft) >>> 0 });
      b.puffLeft -= Math.max(got, 0) || n;   // (a pool that takes none this tick is not asked again: the puff is of the moment)
      filed += Math.max(got, 0);
    }
    return filed;
  }
  /** THE THROW: what each live blast still owes of its loose snow, filed as FLAKE records, born low over the blast's disc and going up, as far as the tick's births allow */
  thrown(Pp: Particles, room: number, size: number, sizeJit: number): number {
    const K = SNOWBLAST; let filed = 0;
    for (const b of this.live) {
      const n = Math.min(b.flakeLeft, Math.max(0, room - filed)); if (n <= 0) continue;
      const got = Pp.emit('rain', [b.x, 0.45, b.z], [0, 1, 0], n, { shape: 'rainDisc', radius: Math.max(0.2, b.size * K.throwR), speed: K.flakeUp * (0.5 + 0.5 * b.hk), speedJit: 0.7, life: K.flakeLife, lifeJit: 0.3,
        size, sizeJit, lift: 0.4, seed: (0x7f1a4e + b.serial * 104729 + b.flakeLeft) >>> 0 });
      b.flakeLeft -= Math.max(got, 0) || n;
      filed += Math.max(got, 0);
    }
    return filed;
  }
}
