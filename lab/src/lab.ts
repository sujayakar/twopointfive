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

export const TICK = 1 / 60;
export interface Method { name: string; signals(g: Graph, lab: Lab): void; stages(lab: Lab): Stage[]; frame?(lab: Lab): void; result: string; reset?(lab: Lab): void; prepare?(lab: Lab): Promise<void> }

export class Lab {
  gpu!: Gpu; graph!: Graph; scene!: SceneGpu; sky!: Sky; water!: Water; camera = new Camera(); building!: Building;
  tick = 0; frameNo = 0; seed = 0; hour = 15.2; hourNow!: Hour; level = 0; cut = 1e9; wantCut = 1e9; knobs = [0, 0, 0, 0]; lampsMode: 'auto' | 'on' | 'off' = 'auto'; wind: V3 = norm([1, 0, 0.35]); exposure = 0; paused = false;
  private prev: ReturnType<Camera['frame']> | null = null; prevAt = new Map<Piece, { at: V3; rot: Piece['rot'] }>(); slots = new Map<Piece, number>(); jitter: [number, number] = [0, 0]; private prevJitter: [number, number] = [0, 0];
  methods = new Map<string, { m: Method; stages: Stage[] }>(); current = ''; doorsAuto = false; torch: { on: boolean; at: V3; dir: V3 } | null = null; jittered = true; haze = 1; rain = 0; rainNow = 0; wet = 0; steam = 1; lampGroup: Record<string, number> = {};
  async open(canvas: HTMLCanvasElement | null) {
    this.gpu = await openGpu(canvas); this.building = makeBuilding(); this.sky = new Sky(this.gpu); await this.sky.init();
    this.water = new Water(this.gpu); await this.water.init(); this.scene = new SceneGpu(this.gpu, this.building.world, this.sky.tex, this.water.tex);
    this.graph = new Graph(this.gpu, () => math + '\n' + sceneCode + '\n' + wavesCode + '\n' + surfaceCode + '\n', { layout: this.scene.layout, group: this.scene.group });
    this.pose(); for (const p of this.building.world.moving) this.prevAt.set(p, { at: p.at, rot: p.rot });
  }
  async use(m: Method) {
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
    for (const p of b.people) p.step(TICK); this.camera.step(TICK); this.pose();
  }
  /** the world at a tick, from the start (only what has a memory is stepped) */
  seek(tick: number) { while (this.walked < tick) { this.walked++; for (const p of this.building.people) p.step(TICK); } this.tick = tick; }
  private walked = 0;
  pose() {
    const b = this.building, t = this.tick * TICK, on = this.scene?.lampOn; for (const d of b.doors) poseDoor(d);
    const h = hourOf(this.hour, 0.85, this.rainNow), dark = this.lampsMode === 'on' ? 1 : this.lampsMode === 'off' ? 0 : h.night, inside = this.lampsMode === 'auto' ? Math.max(dark, 0.45) : dark; this.hourNow = h;
    b.world.lamps.forEach((l, i) => { if (on && l.kind !== 'spot') on[i] = l.on * (l.name === 'terrace' || l.name === 'pool' ? dark : l.name === 'blue' ? Math.max(inside, 0.9) : inside) * (this.lampGroup[l.name] ?? 1); });
    for (const p of b.people) if (p.torch >= 0) { const l = b.world.lamps[p.torch]!; l.at = p.torchAt; l.v = p.torchDir; if (on) on[p.torch] = h.night > 0.5 ? 1 : 0; } void t;
  }
  /** the frame's numbers to the GPU */
  private send() {
    const [rw, rh] = this.graph.render, [ow, oh] = this.graph.output, c = this.camera.frame(ow / oh), p = this.prev ?? c, b = this.building, h = this.hourNow; this.sky.update(h);
    const f = new Float32Array(128), u = new Uint32Array(f.buffer); let o = 0; const v = (a: ArrayLike<number>, w = 0) => { f.set([a[0]!, a[1]!, a[2]!, w], o); o += 4; };
    v(c.pos); v(c.right); v(c.up); v(c.fwd); v(p.pos); v(p.right); v(p.up); v(p.fwd); v(h.sun, h.radius); v(h.sunE, 1);
    f.set([rw, rh, ow, oh], o); o += 4; f.set([this.jitter[0], this.jitter[1], this.prevJitter[0], this.prevJitter[1]], o); o += 4; f.set([this.tick * TICK, this.cut, this.exposure, h.haze * this.haze], o); o += 4; f.set([this.knobs[0]!, this.knobs[1]!, this.knobs[2]!, h.night * 2.2], o); o += 4;
    this.slots = this.scene.moved(this.tick * TICK, this.wind); this.scene.sendLamps();
    u.set([Math.min(64, b.world.lamps.length), Math.min(32, b.world.openings.length), this.frameNo, this.seed], o); o += 4; u.set(this.scene.per, o); o += 4; u.set(this.scene.node0, o); o += 4; u.set(this.scene.stepOf, o); o += 4; u.set(this.scene.piece0, o); o += 4; f.set(b.pool, o); o += 4; f.set([b.poolTop, b.poolDeep, 0, 0], o); o += 4; f.set([this.wet, this.rainNow, this.steam, Math.atan2(this.wind[2], this.wind[0])], o);
    this.gpu.device.queue.writeBuffer(this.scene.frame, 0, f); this.prev = c; this.prevJitter = this.jitter;
  }
  /** draw one frame with the current method; `then` may add its own passes (the canvas) */
  draw(then?: (e: GPUCommandEncoder, result: GPUTexture) => void) {
    const { m, stages } = this.methods.get(this.current)!; if (this.jittered) { const n = (this.frameNo % 16) + 1, hal = (i: number, b: number) => { let r = 0, k = 1 / b; while (i > 0) { r += (i % b) * k; i = Math.floor(i / b); k /= b; } return r; }; this.jitter = [hal(n, 2) - 0.5, hal(n, 3) - 0.5]; } else this.jitter = [0, 0];
    this.send(); m.frame?.(this); const e = this.gpu.device.createCommandEncoder(); this.graph.timer.begin(); this.water.run(e, this.tick * TICK, this.hourNow.sun, this.building.poolDeep); this.graph.run(e, stages); then?.(e, this.graph.texture(m.result)); const done = this.graph.timer.end(e);
    this.gpu.device.queue.submit([e.finish()]); done?.(); this.graph.flip(); this.frameNo++; for (const p of this.building.world.moving) this.prevAt.set(p, { at: p.at, rot: p.rot });
  }
  buffer(name: string, bytes: number, usage = GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_SRC) { let b = this.graph.buffers.get(name); if (!b || b.size < bytes) { b = buffer(this.gpu, name, bytes, usage); this.graph.buffers.set(name, b); } return b; }
  uniform(name: string, data: Float32Array | Uint32Array) { const b = this.graph.buffers.get(name) ?? this.buffer(name, Math.max(16, data.byteLength), GPUBufferUsage.UNIFORM); this.gpu.device.queue.writeBuffer(b, 0, data.buffer as ArrayBuffer, data.byteOffset, data.byteLength); return b; }
  read = (signal: string) => readTexture(this.gpu, this.graph.texture(signal)); readBuffer = (name: string) => readBuffer(this.gpu, this.graph.buffers.get(name)!);
}
