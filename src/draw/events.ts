// WHAT A TICK DID THAT IS DRAWN ONCE, played: particles thrown and kicked, the air pushed, smoke let go. Every row carries the id that names it (its tick, its kind,
// its place among that kind), and an id is played once, however often its tick is run.
import type { EmitOpts, ImpulseOpts, Particles, PKind } from '../render/particles';
import type { SmokeSystem } from '../smoke/smoke';
import type { SmokeEmitter } from '../smoke/types';
import type { Vec3 } from '../math/vec';
import type { Drawn } from './drawn';
import type { AudioEngine } from '../audio/audio';
import type { FloorMat } from '../audio/floors';
import { AirPush, Anchor, Emit, EMIT_SHAPES, FLOORS, Footfall, Heard, Kick, PKINDS, Roll, Section, SmokeSource, SMOKE_KINDS, Sound } from './sections';

/** the ids played in the last `KEPT` ticks, by kind */
const KEPT = 120;
class Played {
  private by = new Map<number, Set<number>>();
  /** is this id new (and it is played from here on) */
  once(kind: number, tick: number, serial: number): boolean {
    let s = this.by.get(tick); if (!s) { this.by.set(tick, s = new Set()); for (const t of this.by.keys()) if (t < tick - KEPT || t > tick + KEPT) this.by.delete(t); }
    const id = kind * 65536 + serial; if (s.has(id)) return false;
    s.add(id); return true;
  }
  /** another game, or the same one put back: its ticks are run for the first time again */
  forget(): void { this.by.clear(); }
}
const LOOK = 1, CONFINED = 2, RIDES_VELOCITY = 4;
const LATTICES = [undefined, 'fine', 'coarse'] as const, SHAPE_LINE = 1, SHAPE_BODY = 2, SHAPE_OUT = 4;
const said = (v: number): number | undefined => v === v ? v : undefined;

export class DrawEvents {
  private played = new Played();
  private anchors = new Map<number, { pos: Vec3; dir: Vec3 }>();
  private names: string[] = [];
  private rolls = new Map<number, { pos: Vec3; k: number; t: number }>();
  constructor(private D: Drawn, private particles: Particles | null, private smoke: SmokeSystem, private audio: AudioEngine, private nameOf: (i: number) => string) {}
  cut(): void { this.played.forget(); this.anchors.clear(); this.rolls.clear(); }
  private v3(o: number): Vec3 { const f = this.D.f; return [f[o], f[o + 1], f[o + 2]]; }
  private v3said(o: number): Vec3 | undefined { return this.D.f[o] === this.D.f[o] ? this.v3(o) : undefined; }

  /** the tick that has just run */
  from(): void {
    this.ride(); this.sounds();
    const D = this.D, f = D.f, u = D.u, P = this.particles;
    if (P) {
      for (let i = 0, o = D.at[Section.emits]; i < D.n[Section.emits]; i++, o += Emit.row) if (this.played.once(Section.emits, u[o + Emit.tick], u[o + Emit.serial])) {
        const m = o + Emit.more, w: EmitOpts = { speed: f[o + Emit.speed], life: f[o + Emit.life], size: f[o + Emit.size], shape: u[o + Emit.shape] ? EMIT_SHAPES[u[o + Emit.shape] - 1] as EmitOpts['shape'] : undefined, spread: said(f[m]), speedJit: said(f[m + 1]), lifeJit: said(f[m + 2]), sizeJit: said(f[m + 3]), lift: said(f[m + 4]), radius: said(f[m + 5]), posJit: said(f[m + 6]), top: said(f[m + 7]), aspect: said(f[m + 8]), tumble: said(f[m + 9]), nrmJit: said(f[m + 10]), steam: said(f[m + 11]), ember: said(f[m + 12]), vadd: this.v3said(o + Emit.vadd), color: this.v3said(o + Emit.color), owner: u[o + Emit.owner], seed: u[o + Emit.seed] === 0xffffffff ? undefined : u[o + Emit.seed] };
        const was = P.ours; P.ours = u[o + Emit.ours] !== 0; P.emit(PKINDS[u[o + Emit.kind]] as PKind, this.v3(o + Emit.pos), this.v3(o + Emit.dir), u[o + Emit.count], w); P.ours = was; }
      for (let i = 0, o = D.at[Section.kicks]; i < D.n[Section.kicks]; i++, o += Kick.row) if (this.played.once(Section.kicks, u[o + Kick.tick], u[o + Kick.serial])) {
        const k = u[o + Kick.kinds], w: ImpulseOpts = { up: said(f[o + Kick.up]), bias: this.v3said(o + Kick.bias), eye: this.v3said(o + Kick.eye), occlude: u[o + Kick.occlude] !== 0, kinds: k ? PKINDS.filter((_, n) => k >>> n & 1) as PKind[] : undefined, seed: u[o + Kick.seed] === 0xffffffff ? undefined : u[o + Kick.seed] };
        P.impulse(this.v3(o + Kick.pos), f[o + Kick.radius], f[o + Kick.strength], w); }
    }
    for (let i = 0, o = D.at[Section.sources]; i < D.n[Section.sources]; i++, o += SmokeSource.row) if (this.played.once(Section.sources, u[o + SmokeSource.tick], u[o + SmokeSource.serial])) this.source(o);
    for (let i = 0, o = D.at[Section.pushes]; i < D.n[Section.pushes]; i++, o += AirPush.row) if (u[o + AirPush.shape] <= SHAPE_BODY && this.played.once(Section.pushes, u[o + AirPush.tick], u[o + AirPush.serial])) {
      const shape = u[o + AirPush.shape], len = f[o + AirPush.len], vel = this.v3(o + AirPush.vel);
      this.smoke.push({ pos: this.v3(o + AirPush.pos), dir: this.v3(o + AirPush.dir), speed: Math.hypot(vel[0], vel[1], vel[2]), radius: f[o + AirPush.radius], lattice: LATTICES[u[o + AirPush.lattice]], line: shape === SHAPE_LINE ? len : undefined, body: shape === SHAPE_BODY ? len : undefined, push: said(f[o + AirPush.hard]) }); }
  }
  /** the sounds of the tick that has just run, after what they follow is known */
  private sounds(): void {
    const D = this.D, A = this.audio, h = D.at[Section.heard], t = D.f[h + Heard.time];
    this.rolls.forEach(r => { r.t = -1; });
    for (let i = 0, o = D.at[Section.rolls]; i < D.n[Section.rolls]; i++, o += Roll.row) { const v = D.u[o + Roll.voice]; let r = this.rolls.get(v); if (!r) this.rolls.set(v, r = { pos: [0, 0, 0], k: 0, t: 0 }); r.pos[0] = D.f[o + Roll.at]; r.pos[2] = D.f[o + Roll.at + 1]; r.k = D.f[o + Roll.hard]; r.t = t; }
    for (const [v, r] of this.rolls) if (r.t < 0) this.rolls.delete(v);
    A.rain(D.f[h + Heard.rain]);
    // (each call below asks the module, whose memory may have grown: the views are looked at afresh after it)
    for (let i = 0, o = D.at[Section.sounds]; i < D.n[Section.sounds]; i++, o += Sound.row, D.fresh()) if (this.played.once(Section.sounds, D.u[o + Sound.tick], D.u[o + Sound.serial])) {
      const f = D.f, u = D.u, n = u[o + Sound.name], voice = u[o + Sound.follows] - 1, pos = u[o + Sound.placed] ? this.v3(o + Sound.pos) : null, y = f[o + Sound.pos + 1], gain = said(f[o + Sound.gain]);
      const opts = voice >= 0 ? { follow: () => { const r = this.rolls.get(voice); if (!r) return null; r.pos[1] = y; return r; } } : { rate: said(f[o + Sound.rate]), duration: said(f[o + Sound.duration]), lowpass: said(f[o + Sound.lowpass]), pan: said(f[o + Sound.pan]), wet: said(f[o + Sound.wet]) };
      A.play(this.names[n] ??= this.nameOf(n), pos, gain, opts); }
    for (let i = 0, o = D.at[Section.footfalls]; i < D.n[Section.footfalls]; i++, o += Footfall.row, D.fresh()) if (this.played.once(Section.footfalls, D.u[o + Footfall.tick], D.u[o + Footfall.serial])) { const m = D.u[o + Footfall.floor]; A.footstep(this.v3(o + Footfall.pos), D.f[o + Footfall.loud], D.u[o + Footfall.own] !== 0, m ? FLOORS[m - 1] as FloorMat : undefined); }
  }
  /** a source of smoke, with the push of the air that goes with it, as today's solver takes them: one emitter */
  private source(o: number): void {
    const D = this.D, f = D.f, u = D.u, n = u[o + SmokeSource.push], a = n ? D.at[Section.pushes] + (n - 1) * AirPush.row : -1, says = u[o + SmokeSource.says], key = u[o + SmokeSource.anchor];
    const vel: Vec3 = a < 0 ? [0, 0, 0] : this.v3(a + AirPush.vel);
    const e: SmokeEmitter = { pos: this.v3(o + SmokeSource.pos), dir: a < 0 ? [0, 1, 0] : this.v3(a + AirPush.dir), speed: Math.hypot(vel[0], vel[1], vel[2]), radius: f[o + SmokeSource.radius], density: f[o + SmokeSource.density], temperature: f[o + SmokeSource.heat], ttl: f[o + SmokeSource.ttl], age: f[o + SmokeSource.age],
      kind: SMOKE_KINDS[u[o + SmokeSource.kind]] as SmokeEmitter['kind'], lattice: LATTICES[u[o + SmokeSource.lattice]], attack: said(f[o + SmokeSource.attack]), jitter: said(f[o + SmokeSource.jitter]), prio: said(f[o + SmokeSource.prio]), minVoxels: said(f[o + SmokeSource.minCells]), tint: this.v3said(o + SmokeSource.tint),
      look: (says & LOOK) !== 0 || undefined, confined: (says & CONFINED) !== 0 || undefined, trackVel: (says & RIDES_VELOCITY) !== 0 || undefined, mode: a >= 0 && u[a + AirPush.shape] === SHAPE_OUT ? 'burst' : undefined, push: a < 0 ? undefined : said(f[a + AirPush.hard]), expand: a < 0 ? undefined : said(f[a + AirPush.expand]) };
    if (key) e.track = () => this.anchors.get(key) ?? null;
    this.smoke.emit(e);
  }
  /** where what the live sources ride is now */
  private ride(): void {
    const D = this.D, A = this.anchors; A.clear();
    for (let i = 0, o = D.at[Section.anchors]; i < D.n[Section.anchors]; i++, o += Anchor.row) A.set(D.u[o + Anchor.key], { pos: this.v3(o + Anchor.pos), dir: this.v3(o + Anchor.dir) });
  }
}
