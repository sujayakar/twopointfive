// THE LAB: one world, one camera, one clock in ticks, and a list of stages that turn them into a picture. Everything the picture depends on is a pure
// function of (tick, inputs so far, seed), so a run can be played again exactly and two methods can be shown the same frames.
import { openGpu, buffer, readTexture, readBuffer, type Gpu } from './gpu';
import { Graph, type Stage } from './graph';
import { SceneGpu } from './scenegpu';
import { Sky, hourOf, type Hour } from './sky';
import { Camera } from './camera';
import { makeBuilding, poseDoor, type Building } from './scene/building';
import type { Piece } from './scene/world';
import { type V3, clamp, norm } from './math';
import math from './wgsl/math.wgsl' with { type: 'text' };
import sceneCode from './wgsl/scene.wgsl' with { type: 'text' };
import surfaceCode from './wgsl/surface.wgsl' with { type: 'text' };
import wavesCode from './wgsl/waves.wgsl' with { type: 'text' };
import { Water } from './water';
import { defaults } from './tune';
import { Smoke } from './stages/smoke';
import { Near } from './scene/near';
import { mapLamps, lampMapCode } from './lampmap';

export const TICK = 1 / 60;
export interface Method { name: string; signals(g: Graph, lab: Lab): void; stages(lab: Lab): Stage[]; frame?(lab: Lab): void; result: string; reset?(lab: Lab): void; prepare?(lab: Lab): Promise<void> }

export class Lab {
  gpu!: Gpu; graph!: Graph; scene!: SceneGpu; sky!: Sky; water!: Water; camera = new Camera(); building!: Building;
  tick = 0; frameNo = 0; seed = 0; hour = 15.2; hourNow!: Hour; level = 0; cut = 1e9; wantCut = 1e9; knobs = [0, 0, 0, 0]; lampsMode: 'auto' | 'on' | 'off' = 'auto'; wind: V3 = norm([1, 0, 0.35]); exposure = 0; paused = false;
  private prev: ReturnType<Camera['frame']> | null = null; prevAt = new Map<Piece, { at: V3; rot: Piece['rot'] }>(); slots = new Map<Piece, number>(); jitter: [number, number] = [0, 0]; private prevJitter: [number, number] = [0, 0];
  methods = new Map<string, { m: Method; stages: Stage[] }>(); current = ''; doorsAuto = false; torch: { on: boolean; at: V3; dir: V3 } | null = null; jittered = true; haze = 1; rain = 0; rainNow = 0; wet = 0; steam = 1; lampGroup: Record<string, number> = {}; dense = 1; readonly tune = defaults(); readonly smoke = new Smoke(this); readonly lampOff = new Set<number>();
  async open(canvas: HTMLCanvasElement | null) {
    this.gpu = await openGpu(canvas); this.building = makeBuilding(); this.near = new Near(this.building.world.pieces); this.leaves = this.building.doors.flatMap(d => d.leaf); this.sky = new Sky(this.gpu); await this.sky.init();
    this.water = new Water(this.gpu); await this.water.init(); this.scene = new SceneGpu(this.gpu, this.building.world, this.sky.tex, this.water.tex);
    this.graph = new Graph(this.gpu, () => math + '\n' + sceneCode + '\n' + wavesCode + '\n' + lampMapCode + '\n' + surfaceCode + '\n', { layout: this.scene.layout, group: this.scene.group });
    this.pose(); for (const p of this.building.world.moving) this.prevAt.set(p, { at: p.at, rot: p.rot });
  }
  /** in the page: what moves is drawn between its last two ticks (alpha), and the camera goes by the clock, so that motion is even at any frame rate. A run leaves this off */
  smooth = false; alpha = 1; private wasPose = new Float64Array(0); private nowPose = new Float64Array(0);
  private keepPose() { const n = this.building.world.moving.length * 12 + this.building.people.length * 6; if (this.nowPose.length !== n) { this.nowPose = new Float64Array(n); this.wasPose = new Float64Array(n); this.savePose(this.nowPose); } const w = this.wasPose; this.wasPose = this.nowPose; this.nowPose = w; this.savePose(this.nowPose); }
  private savePose(to: Float64Array) { let o = 0; for (const p of this.building.world.moving) { to.set(p.at, o); to.set(p.rot[0], o + 3); to.set(p.rot[1], o + 6); to.set(p.rot[2], o + 9); o += 12; } for (const p of this.building.people) { const l = p.torch >= 0 ? this.building.world.lamps[p.torch] : null; if (l) { to.set(l.at, o); to.set(l.v, o + 3); } o += 6; } }
  private setPose(f: (i: number) => number) { const g = (o: number): V3 => [f(o), f(o + 1), f(o + 2)]; let o = 0; for (const p of this.building.world.moving) { p.at = g(o); const x = norm(g(o + 3)), y0 = g(o + 6), k = x[0] * y0[0] + x[1] * y0[1] + x[2] * y0[2], y = norm([y0[0] - x[0] * k, y0[1] - x[1] * k, y0[2] - x[2] * k]); p.rot = [x, y, [x[1] * y[2] - x[2] * y[1], x[2] * y[0] - x[0] * y[2], x[0] * y[1] - x[1] * y[0]]]; o += 12; } for (const p of this.building.people) { const l = p.torch >= 0 ? this.building.world.lamps[p.torch] : null; if (l) { l.at = g(o); l.v = norm(g(o + 3)); } o += 6; } }
  private between() { const a = this.wasPose, b = this.nowPose; if (!this.smooth || this.alpha >= 1 || a.length === 0 || a.length !== b.length) return; let far = 0; for (let i = 0; i < a.length; i += 12) far = Math.max(far, Math.abs(a[i]! - b[i]!)); if (far > 1.5) return; const k = this.alpha; this.setPose(i => a[i]! + (b[i]! - a[i]!) * k); }
  /** seconds, as drawn */
  get time() { return (this.tick - (this.smooth ? 1 - this.alpha : 0)) * TICK; }
  private near!: Near; private leaves: Piece[] = []; private mapped = false;
  /** where the eye that the picture is exposed for looks: a point of the world, or a person (by number); null = the middle of the picture */
  eye: V3 | number | null = null;
  eyeAt(): V3 | null { return typeof this.eye === 'number' ? ((p: V3): V3 => [p[0], p[1] + 1.4, p[2]])(this.building.people[this.eye]!.pos) : this.eye; }
  project(x: V3): [number, number] | null { const c = this.camera.frame(this.graph.output[0] / this.graph.output[1]), v: V3 = [x[0] - c.pos[0], x[1] - c.pos[1], x[2] - c.pos[2]], d = (a: ArrayLike<number>) => v[0] * a[0]! + v[1] * a[1]! + v[2] * a[2]!, q = (a: ArrayLike<number>) => a[0]! ** 2 + a[1]! ** 2 + a[2]! ** 2, z = d(c.fwd); if (z < 0.1) return null; return [d(c.right) / q(c.right) / z * 0.5 + 0.5, 0.5 - d(c.up) / q(c.up) / z * 0.5]; }
  async use(m: Method) {
    if (!this.mapped) { this.mapped = true; this.send(); this.prev = null; await mapLamps(this); }
    if (!this.methods.has(m.name)) { m.signals(this.graph, this); await m.prepare?.(this); const stages = m.stages(this); await this.graph.build(stages); this.methods.set(m.name, { m, stages }); }
    this.current = m.name; m.reset?.(this); return m;
  }
  resize(render: [number, number], output: [number, number]) { if (this.graph.resize(render, output)) { for (const { m } of this.methods.values()) m.reset?.(this); this.frameNo = 0; } }
  /** one tick of the world: doors, people, trees, the section, the camera */
  step() {
    const b = this.building; this.tick++; this.walked++;
    if (this.doorsAuto) b.doors.forEach((d, i) => { d.want = Math.sin(this.tick * TICK * 0.45 + i * 1.9) > 0.1 ? 1 : 0; });
    for (const d of b.doors) { const v = clamp(d.want - d.open, -TICK * 0.9, TICK * 0.9); d.open += v; }
    this.cut += clamp(this.wantCut - this.cut, -TICK * 6, TICK * 6); if (this.wantCut > 1e8) this.cut = this.cut < 9.5 ? Math.min(9.5, this.cut + TICK * 6) : 1e9; else if (this.cut > 1e8) this.cut = 9.4;
    this.rainNow += clamp(this.rain - this.rainNow, -TICK * 0.35, TICK * 0.35); this.wet += clamp((this.rain > 0 ? 1 : 0) - this.wet, -TICK * 0.06, TICK * 0.25);
    for (const p of b.people) p.step(TICK); if (!this.smooth) this.camera.step(TICK); this.pose(); if (this.smooth) this.keepPose();
  }
  /** the world at a tick, from the start (only what has a memory is stepped) */
  seek(tick: number) { while (this.walked < tick) { this.walked++; for (const p of this.building.people) p.step(TICK); } this.tick = tick; }
  private walked = 0;
  pose() {
    const b = this.building, t = this.tick * TICK, on = this.scene?.lampOn; for (const d of b.doors) poseDoor(d);
    const h = hourOf(this.hour, 0.85, this.rainNow), dark = this.lampsMode === 'on' ? 1 : this.lampsMode === 'off' ? 0 : h.night, inside = this.lampsMode === 'auto' ? Math.max(dark, 0.45) : dark; this.hourNow = h;
    b.world.lamps.forEach((l, i) => { if (on && l.kind !== 'spot') on[i] = l.on * (l.name === 'terrace' || l.name === 'pool' ? dark : l.name === 'blue' ? Math.max(inside, 0.9) : inside) * (this.lampGroup[l.name] ?? 1) * (this.lampOff.has(i) ? 0 : 1); });
    for (const p of b.people) if (p.torch >= 0) { const l = b.world.lamps[p.torch]!; const chest: V3 = [p.pos[0], p.pos[1] + 1.25, p.pos[2]], to: V3 = [p.torchAt[0] - chest[0], p.torchAt[1] - chest[1], p.torchAt[2] - chest[2]], len = Math.hypot(...to) || 1, dir: V3 = [to[0] / len, to[1] / len, to[2] / len], r = this.near.reach(chest, dir, len + 0.06, this.leaves) - 0.06; l.at = r < len ? [chest[0] + dir[0] * r, chest[1] + dir[1] * r, chest[2] + dir[2] * r] : p.torchAt; l.v = p.torchDir; if (on) on[p.torch] = h.night > 0.5 && !this.lampOff.has(p.torch) ? 1 : 0; } void t;
  }
  /** the frame's numbers to the GPU */
  private send() {
    const [rw, rh] = this.graph.render, [ow, oh] = this.graph.output, c = this.camera.frame(ow / oh), p = this.prev ?? c, b = this.building, h = this.hourNow; this.sky.update(h);
    const f = new Float32Array(128), u = new Uint32Array(f.buffer); let o = 0; const v = (a: ArrayLike<number>, w = 0) => { f.set([a[0]!, a[1]!, a[2]!, w], o); o += 4; };
    v(c.pos); v(c.right); v(c.up); v(c.fwd); v(p.pos); v(p.right); v(p.up); v(p.fwd); v(h.sun, h.radius); v(h.sunE, 1);
    f.set([rw, rh, ow, oh], o); o += 4; f.set([this.jitter[0], this.jitter[1], this.prevJitter[0], this.prevJitter[1]], o); o += 4; f.set([this.time, this.cut, this.exposure, h.haze * this.haze], o); o += 4; f.set([this.knobs[0]!, this.knobs[1]!, this.knobs[2]!, h.night * 2.2], o); o += 4;
    this.slots = this.scene.moved(this.time, this.wind); this.scene.cull(c, this.cut); this.scene.sendLamps();
    u.set([Math.min(64, b.world.lamps.length), Math.min(32, b.world.openings.length), this.frameNo, this.seed], o); o += 4; u.set(this.scene.per, o); o += 4; u.set(this.scene.node0, o); o += 4; u.set(this.scene.stepOf, o); o += 4; u.set(this.scene.piece0, o); o += 4; f.set(b.pool, o); o += 4; f.set([b.poolTop, b.poolDeep, 0, 0], o); o += 4; f.set([this.wet, this.rainNow, this.steam, Math.atan2(this.wind[2], this.wind[0])], o); o += 4; f.set([this.smoke.on ? 1 : 0, this.scene.noiseAt, this.dense, 0], o); o += 4; f.set(this.tune, o);
    this.gpu.device.queue.writeBuffer(this.scene.frame, 0, f); this.prev = c; this.prevJitter = this.jitter;
  }
  sendMs = 0;
  /** draw one frame with the current method; `then` may add its own passes (the canvas) */
  draw(then?: (e: GPUCommandEncoder, result: GPUTexture) => void) {
    const { m, stages } = this.methods.get(this.current)!; if (this.jittered) { const n = (this.frameNo % 16) + 1, hal = (i: number, b: number) => { let r = 0, k = 1 / b; while (i > 0) { r += (i % b) * k; i = Math.floor(i / b); k /= b; } return r; }; this.jitter = [hal(n, 2) - 0.5, hal(n, 3) - 0.5]; } else this.jitter = [0, 0];
    const t0 = performance.now(); this.between(); this.send(); m.frame?.(this); this.sendMs = performance.now() - t0; const e = this.gpu.device.createCommandEncoder(); this.graph.timer.begin(); this.water.run(e, this.time, this.hourNow.sun, this.building.poolDeep); this.graph.run(e, stages); then?.(e, this.graph.texture(m.result)); const done = this.graph.timer.end(e);
    this.gpu.device.queue.submit([e.finish()]); done?.(); this.graph.flip(); this.frameNo++; for (const p of this.building.world.moving) this.prevAt.set(p, { at: p.at, rot: p.rot });
  }
  buffer(name: string, bytes: number, usage = GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_SRC) { let b = this.graph.buffers.get(name); if (!b || b.size < bytes) { b = buffer(this.gpu, name, bytes, usage); this.graph.buffers.set(name, b); } return b; }
  uniform(name: string, data: Float32Array | Uint32Array) { const b = this.graph.buffers.get(name) ?? this.buffer(name, Math.max(16, data.byteLength), GPUBufferUsage.UNIFORM); this.gpu.device.queue.writeBuffer(b, 0, data.buffer as ArrayBuffer, data.byteOffset, data.byteLength); return b; }
  read = (signal: string) => readTexture(this.gpu, this.graph.texture(signal)); readBuffer = (name: string) => readBuffer(this.gpu, this.graph.buffers.get(name)!);
}
