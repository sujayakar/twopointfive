// THE LAB: one world, one camera, one clock in ticks, and a list of stages that turn them into a picture. Everything the picture depends on is a pure
// function of (tick, inputs so far, seed), so a run can be played again exactly and two methods can be shown the same frames.
import { CARDS } from './stages/torches';
import { openGpu, buffer, readTexture, readBuffer, type Gpu } from './gpu';
import { Graph, type Stage } from './graph';
import { SceneGpu, SPARE } from './scenegpu';
import { Sky, hourOf, type Hour } from './sky';
import { Camera } from './camera';
import { makeBuilding, poseDoor, type Building } from './scene/building';
import { F, type Piece } from './scene/world';
import { makeStreet } from './scene/street';
import { makeForest } from './scene/forest';
import { makePass } from './scene/pass';
import { makeMeshes } from './scene/meshes';
import { makeCorner } from './scene/corner';
import { knockOut } from './mesh/knock';
import { meshed } from './mesh/code';
import { strain, type Strain } from './scene/strain';
import { placeCode } from './scene/place';
import { Sway } from './sway';
import { skyMap, skyCode, tread, TROD } from './scene/skymap';
import { blockCode } from './scene/blocks';
import { mapBlocks } from './blockmap';
import { Air, airCode, airRows, BREEZE } from './scene/air';
import { type V3, clamp, norm } from './math';
import math from './wgsl/math.wgsl' with { type: 'text' };
import sceneCode from './wgsl/scene.wgsl' with { type: 'text' };
import raysCode from './wgsl/rays.wgsl' with { type: 'text' };
import keptCode from './wgsl/kept.wgsl' with { type: 'text' };
import surfaceCode from './wgsl/surface.wgsl' with { type: 'text' };
import { Ripples, rippleCode, rippleRows } from './scene/waves';
import wavesCode from './wgsl/waves.wgsl' with { type: 'text' };
import { Water } from './water';
import { defaults, graded, KNOBS } from './tune';
import { Smoke } from './stages/smoke';
import { Near } from './scene/near';
import { mapLamps, lampsAgain, lampMapCode } from './lampmap';
import { mapJoins, joinAgain, joinCode } from './joins';
import { breach, type Breach } from './scene/breach';
import { Body } from './scene/rigid';

export const TICK = 1 / 60;
export const SCENES: Record<string, () => Building> = { rooftop: makeBuilding, street: makeStreet, forest: makeForest, pass: makePass };
/** scenes for a study, which the page does not list */
export const STUDIES: Record<string, () => Building> = { meshes: makeMeshes, corner: makeCorner };
export interface Method { name: string; signals(g: Graph, lab: Lab): void; stages(lab: Lab): Stage[]; frame?(lab: Lab): void; result: string; reset?(lab: Lab): void; prepare?(lab: Lab): Promise<void> }

export class Lab {
  gpu!: Gpu; graph!: Graph; scene!: SceneGpu; sky!: Sky; water!: Water; camera = new Camera(); building!: Building;
  /** is what the character cannot see drawn as remembered? */ fog = false;
  tick = 0; frameNo = 0; seed = 0; /** scene.wgsl: otherDice */ otherDice = 0; hour = 15.2; hourNow!: Hour; level = 0; cut = 1e9; wantCut = 1e9; knobs = [0, 0, 0, 0]; lampsMode: 'auto' | 'on' | 'off' = 'auto'; wind: V3 = norm([1, 0, 0.35]); exposure = 0; paused = false;
  private prev: ReturnType<Camera['frame']> | null = null; prevAt = new Map<Piece, { at: V3; rot: Piece['rot'] }>(); slots = new Map<Piece, number>(); jitter: [number, number] = [0, 0]; private prevJitter: [number, number] = [0, 0];
  /** the block's cells that keep light (blockmap.ts), and what was found of them */ litCells = { many: 0, at: new Float32Array(8) }; blocksSaid = '';
  methods = new Map<string, { m: Method; stages: Stage[] }>(); current = ''; doorsAuto = false; torch: { on: boolean; at: V3; dir: V3 } | null = null; jittered = true; haze = 1; /** how much of the moon there is, and is there power */ moon = 1; mains = true; /** how much snow lies, and how hard it falls */ snow = 0; /** what lies with none falling */ lies = 0; snowfall = 0; snowNow = 0; readonly trod = new Uint8Array(TROD.n[0] * TROD.n[1]); rain = 0; rainNow = 0; wet = 0; /** how wet the ground stays with no rain falling */ soaked = 0; steam = 1; lampGroup: Record<string, number> = {}; readonly tune = defaults(); readonly smoke = new Smoke(this); readonly lampOff = new Set<number>();
  async open(canvas: HTMLCanvasElement | null, scene = 'rooftop', load: Strain | null = null) {
    this.gpu = await openGpu(canvas); this.building = (SCENES[scene] ?? STUDIES[scene] ?? SCENES.rooftop!)(); if (load) strain(this.building, load); for (let i = 0; i < SPARE.move; i++) this.spares.push(this.building.world.add({ at: [0, -100, 0], half: [0.01, 0.01, 0.01], mat: 0, flags: F.MOVES, rig: 'spare' })); for (let i = 0; i < 3; i++) this.building.world.lamps.push({ kind: 'sphere', at: [0, -60, 0], u: [0.2, 0, 0], v: [0, 0, 0], radiance: [330, 125, 27], name: 'fire', on: 0 }); this.building.world.lamps.push({ kind: 'sphere', at: [0, -60, 0], u: [0.05, 0, 0], v: [0, 0, 0], radiance: [16000, 11000, 5600], name: 'flash', on: 0 }); this.air = new Air(this.building.place, this.building.world, this.building.doors); this.near = new Near(this.building.world.pieces); this.leaves = this.building.doors.flatMap(d => d.leaf); this.sky = new Sky(this.gpu); await this.sky.init();
    this.ripples = new Ripples(this.building.pool, this.building.poolTop); this.building.bodies.ripples = this.ripples; this.water = new Water(this.gpu, this.ripples); await this.water.init(); this.scene = new SceneGpu(this.gpu, this.building.world, this.sky.tex, this.water.tex, airRows(this.air.cells), skyMap(this.building.solid, this.building.world.moving), rippleRows(this.ripples));
    this.graph = new Graph(this.gpu, () => math + '\n' + placeCode(this.building.place) + airCode(this.air) + blockCode(this.scene.blocks, this.scene.blockRow) + (this.building.world.meshes ? meshed.scene(sceneCode + raysCode, this.scene.meshRow, this.building.world.meshes.lost, Math.max(...this.building.world.meshes.objects.map(o => o.stack))) : sceneCode + raysCode) + keptCode + skyCode(this.scene.skyRow, this.scene.trodRow) + rippleCode(this.ripples, `exF(${this.scene.ripRow}u + (i >> 2u))[i & 3u]`, 'frame.more.w') + '\n' + wavesCode + '\n' + lampMapCode + joinCode + '\n' + (this.building.world.meshes ? meshed.surface(surfaceCode) : surfaceCode) + '\n', { layout: this.scene.layout, group: this.scene.group });
    this.graph.buffers.set('pastLeaves', this.scene.pastLeaves); this.scene.openFloor = this.building.place.openFloor; { const t = this.building.player.body.torch; if (t >= 0) { this.building.world.lamps[t]!.at = [0, -60, 0]; this.scene.lampOn[t] = 0; } } this.pose(); for (const p of this.building.world.moving) this.prevAt.set(p, { at: p.at, rot: p.rot });
  }
  /** in the page: what moves is drawn between its last two ticks (alpha), and the camera goes by the clock, so that motion is even at any frame rate. A run leaves this off */
  smooth = false; alpha = 1; private wasPose = new Float64Array(0); private nowPose = new Float64Array(0);
  private keepPose() { const n = this.building.world.moving.length * 12 + this.building.people.length * 6; if (this.nowPose.length !== n) { this.nowPose = new Float64Array(n); this.wasPose = new Float64Array(n); this.savePose(this.nowPose); } const w = this.wasPose; this.wasPose = this.nowPose; this.nowPose = w; this.savePose(this.nowPose); }
  private savePose(to: Float64Array) { let o = 0; for (const p of this.building.world.moving) { to.set(p.at, o); to.set(p.rot[0], o + 3); to.set(p.rot[1], o + 6); to.set(p.rot[2], o + 9); o += 12; } for (const p of this.building.people) { const l = p.torch >= 0 ? this.building.world.lamps[p.torch] : null; if (l) { to.set(l.at, o); to.set(l.v, o + 3); } o += 6; } }
  private setPose(f: (i: number) => number) { const g = (o: number): V3 => [f(o), f(o + 1), f(o + 2)]; let o = 0; for (const p of this.building.world.moving) { p.at = g(o); const x = norm(g(o + 3)), y0 = g(o + 6), k = x[0] * y0[0] + x[1] * y0[1] + x[2] * y0[2], y = norm([y0[0] - x[0] * k, y0[1] - x[1] * k, y0[2] - x[2] * k]); p.rot = [x, y, [x[1] * y[2] - x[2] * y[1], x[2] * y[0] - x[0] * y[2], x[0] * y[1] - x[1] * y[0]]]; o += 12; } for (const p of this.building.people) { const l = p.torch >= 0 ? this.building.world.lamps[p.torch] : null; if (l) { l.at = g(o); l.v = norm(g(o + 3)); } o += 6; } }
  private between() { const a = this.wasPose, b = this.nowPose; if (!this.smooth || this.alpha >= 1 || a.length === 0 || a.length !== b.length) return; let far = 0; for (let i = 0; i < a.length; i += 12) far = Math.max(far, Math.abs(a[i]! - b[i]!)); if (far > 1.5) return; const k = this.alpha; this.setPose(i => a[i]! + (b[i]! - a[i]!) * k); }
  /** how far the mean wind has gone, as drawn */
  get gone() { return this.air.gone - (this.smooth ? 1 - this.alpha : 0) * TICK * this.air.strength * BREEZE; }
  /** seconds, as drawn */
  get time() { return (this.tick - (this.smooth ? 1 - this.alpha : 0)) * TICK; }
  air!: Air; private near!: Near; private leaves: Piece[] = []; private mapped = false;
  /** where the eye that the picture is exposed for looks: a point of the world, or a person (by number); null = the middle of the picture */
  eye: V3 | number | null = null;
  eyeAt(): V3 | null { return typeof this.eye === 'number' ? ((p: V3): V3 => [p[0], p[1] + 1.4, p[2]])(this.building.people[this.eye]!.pos) : this.eye; }
  project(x: V3): [number, number] | null { const c = this.camera.frame(this.graph.output[0] / this.graph.output[1]), v: V3 = [x[0] - c.pos[0], x[1] - c.pos[1], x[2] - c.pos[2]], d = (a: ArrayLike<number>) => v[0] * a[0]! + v[1] * a[1]! + v[2] * a[2]!, q = (a: ArrayLike<number>) => a[0]! ** 2 + a[1]! ** 2 + a[2]! ** 2, z = d(c.fwd); if (z < 0.1) return null; return [d(c.right) / q(c.right) / z * 0.5 + 0.5, 0.5 - d(c.up) / q(c.up) / z * 0.5]; }
  async use(m: Method) {
    if (!this.mapped) { this.mapped = true; this.send(); this.prev = null; await mapLamps(this); await mapJoins(this); this.blocksSaid = await mapBlocks(this, /swift/i.test(this.gpu.info) ? 3 : 8); await this.sway.init(this, this.scene.instRest, this.scene.instData, this.scene.pastLeaves); }
    if (!this.methods.has(m.name)) { m.signals(this.graph, this); await m.prepare?.(this); const stages = m.stages(this); await this.graph.build(stages); this.methods.set(m.name, { m, stages }); }
    this.current = m.name; m.reset?.(this); return m;
  }
  resize(render: [number, number], output: [number, number]) { if (this.graph.resize(render, output)) { for (const { m } of this.methods.values()) m.reset?.(this); this.frameNo = 0; } }
  playing = false; hiddenOf = new Map<Building['people'][0], number>();
  /** take or leave the character */
  play(on: boolean) { const b = this.building, p = b.player; if (on === this.playing) return; this.playing = on; if (on) { p.torch = this.hourNow.night > 0.4; if (!b.people.includes(p.body)) b.people.push(p.body); this.eye = b.people.indexOf(p.body); } else { b.people.splice(b.people.indexOf(p.body), 1); for (const q of p.body.body) q.at = [0, -100, 0]; if (p.body.torch >= 0) { b.world.lamps[p.body.torch]!.at = [0, -60, 0]; if (this.scene) this.scene.lampOn[p.body.torch] = 0; } this.eye = null; for (const q of b.people) this.hiddenOf.delete(q); } this.nowPose = new Float64Array(0); }
  /** feet that have come down press the snow */
  private prints() { for (const p of this.building.people) for (const f of p.feet) { const k = f.down; if (k && !this.stood.get(f)) { for (const [at, n] of tread(this.trod, f.at[0], f.at[2], f.yaw, 1)) this.scene.writeTrod(at, this.trod.subarray(at, at + n)); } this.stood.set(f, k); } } private stood = new Map<object, boolean>();
  /** whom the character sees */
  private notice() {
    const b = this.building, me = b.player.body, low = me.drive!.low, eye: V3 = [me.pos[0], me.pos[1] + 1.62 - 0.45 * low, me.pos[2]]; for (const p of b.people) { if (p === me) continue; let sees = false; for (const h of [1.6, 1.0, 0.3]) { const to: V3 = [p.pos[0] - eye[0], p.pos[1] + h - eye[1], p.pos[2] - eye[2]], l = Math.hypot(...to) || 1, d: V3 = [to[0] / l, to[1] / l, to[2] / l], f = me.facing, c = (to[0] * f[0] + to[2] * f[2]) / (Math.hypot(to[0], to[2]) || 1); if (l < 2.6 || c > -0.15) if (b.solid.ray(eye, d, l, true) >= l - 0.35) { sees = true; break; } } const was = this.hiddenOf.get(p) ?? 0; this.hiddenOf.set(p, sees ? 0 : Math.min(1, was + TICK / 0.35)); if (this.hiddenOf.get(p)! >= 1) for (const q of p.body) q.at = [q.at[0], q.at[1] - 400, q.at[2]]; }
  }
  private shots() { const b = this.building, p = b.player, l = b.world.lamps.findIndex(q => q.name === 'flash'); if (l < 0) return; const s = p.shots[p.shots.length - 1], on = !!s && s.age < 0.045; b.world.lamps[l]!.at = on ? [s!.at[0] + s!.dir[0] * 0.12, s!.at[1] + s!.dir[1] * 0.12, s!.at[2] + s!.dir[2] * 0.12] : [0, -60, 0]; b.world.lamps[l]!.on = on ? 1 : 0; this.scene.lampOn[l] = on ? 1 : 0; if (p.hit) { this.smoke.puff([s!.at[0] + s!.dir[0] * 0.10, s!.at[1] + s!.dir[1] * 0.10, s!.at[2] + s!.dir[2] * 0.10], 0.16, 14); this.smoke.puff([p.hit.at[0] - p.hit.dir[0] * 0.06, p.hit.at[1] - p.hit.dir[1] * 0.06, p.hit.at[2] - p.hit.dir[2] * 0.06], 0.18, 20); b.bodies.strike(s!.at, s!.dir, s!.far, 3.2); } }
  /** one tick of the world: doors, people, trees, the section, the camera */
  step() {
    const b = this.building; this.tick++; this.walked++;
    if (this.doorsAuto) b.doors.forEach((d, i) => { d.want = Math.sin(this.tick * TICK * 0.45 + i * 1.9) > 0.1 ? 1 : 0; });
    for (const d of b.doors) { const v = clamp(d.want - d.open, -TICK * 0.9, TICK * 0.9); d.open += v; }
    this.cut += clamp(this.wantCut - this.cut, -TICK * 6, TICK * 6); if (this.wantCut > 1e8) this.cut = this.cut < 9.5 ? Math.min(9.5, this.cut + TICK * 6) : 1e9; else if (this.cut > 1e8) this.cut = 9.4;
    this.snow += clamp((this.snowfall > 0 ? 1 : this.lies) - this.snow, -TICK * 0.05, TICK * 0.12); this.snowNow += clamp(this.snowfall - this.snowNow, -TICK * 0.35, TICK * 0.35); this.rainNow += clamp(this.rain - this.rainNow, -TICK * 0.35, TICK * 0.35); this.wet += clamp((this.rain > 0 ? 1 : this.soaked) - this.wet, -TICK * 0.06, TICK * 0.25);
    for (const p of b.people) if (p !== b.player.body) p.step(TICK); if (this.playing) { b.player.step(TICK, this.camera.yaw); if (this.fog) this.notice(); this.shots(); } if (this.snow > 0.2) this.prints(); this.swing(this.tick); if (!this.smooth) this.camera.step(TICK); this.pose(); if (this.smooth) this.keepPose();
  }
  /** the world at a tick, from the start (only what has a memory is stepped) */
  airMs = 0; private readonly sway = new Sway();
  private swing(tick: number) { const t0 = performance.now(); this.stir(); this.air.blow(this.wind, TICK); this.airMs = this.airMs * 0.95 + 0.05 * (performance.now() - t0); this.wade(); for (const h of this.building.hung) h.step(TICK, tick * TICK, this.air); this.ripples.step(TICK); }
  ripples!: Ripples;
  /** legs in the water */
  private wade() { const r = this.ripples; this.building.player.wading = 0; if (!r.any) return; for (const p of this.building.people) { if (p.pos[0] < r.rect[0] - 0.4 || p.pos[0] > r.rect[2] + 0.4 || p.pos[2] < r.rect[1] - 0.4 || p.pos[2] > r.rect[3] + 0.4 || p.pos[1] > r.top) continue; for (const f of p.feet) { const deep = r.top - (f.at[1] + f.lift); if (deep > 0 && f.at[0] > r.rect[0] && f.at[0] < r.rect[2] && f.at[2] > r.rect[1] && f.at[2] < r.rect[3]) r.sink(f.at[0], f.at[2], 0.006 * Math.min(deep, 0.8), 0.15); if (p === this.building.player.body) this.building.player.wading = Math.max(this.building.player.wading, deep); } } }
  seek(tick: number) { while (this.walked < tick) { this.walked++; for (const p of this.building.people) if (p !== this.building.player.body) p.step(TICK); this.swing(this.walked); } this.tick = tick; }
  /** what moves the air itself: bodies, leaves of doors, vents, fires */
  private stir() {
    const P = this.air.pushes; P.length = 0; for (const p of this.building.people) if (Math.hypot(p.vel[0], p.vel[2]) > 0.2) P.push({ at: [p.pos[0], p.pos[1] + 0.9, p.pos[2]], r: 0.5, v: [p.vel[0], 0, p.vel[2]], grip: 8 });
    for (const d of this.building.doors) { const l = d.leaf[0]!, w = this.prevAt.get(l); if (!w) continue; const v = [0, 1, 2].map(k => (l.at[k]! - w.at[k]!) / TICK) as V3; if (Math.hypot(...v) > 0.1 && Math.hypot(...v) < 6) P.push({ at: l.at, r: 0.7, v, grip: 8 }); }
    if (this.steam > 0) for (const p of this.building.world.plumes) P.push({ at: [p.at[0], p.at[1] + 0.3, p.at[2]], r: p.r, v: [0, 3.0, 0], grip: 6, heat: 1.5 }); for (const f of this.smoke.fires) P.push({ at: [f.at[0], f.at[1] + 0.9, f.at[2]], r: 0.6 * f.size, v: [0, 3.0, 0], grip: 8, heat: 2.5 });
  }
  private walked = 0;
  pose() {
    const b = this.building, t = this.tick * TICK, on = this.scene?.lampOn; for (const d of b.doors) poseDoor(d);
    const h = hourOf(this.hour, 0.85, Math.max(this.rainNow, 0.85 * this.snowNow), this.moon, this.mains), dark = this.lampsMode === 'on' ? 1 : this.lampsMode === 'off' ? 0 : h.night, inside = this.lampsMode === 'auto' ? Math.max(dark, 0.45) : dark; this.hourNow = h;
    b.world.lamps.forEach((l, i) => { if (on && l.kind !== 'spot' && l.name !== 'fire' && l.name !== 'flash') on[i] = l.on * (l.name === 'terrace' || l.name === 'pool' ? dark : l.name === 'blue' ? Math.max(inside, 0.9) : inside) * (this.mains || l.name === 'exit' ? 1 : 0) * (l.name === 'exit' ? 1 : this.lampGroup[l.name] ?? 1) * (this.lampOff.has(i) ? 0 : 1); });
    for (const p of b.people) if (p.torch >= 0) { const l = b.world.lamps[p.torch]!; const chest: V3 = [p.pos[0], p.pos[1] + 1.25, p.pos[2]], to: V3 = [p.torchAt[0] - chest[0], p.torchAt[1] - chest[1], p.torchAt[2] - chest[2]], len = Math.hypot(...to) || 1, dir: V3 = [to[0] / len, to[1] / len, to[2] / len], r = this.near.reach(chest, dir, len + 0.06, this.leaves) - 0.06; l.at = r < len ? [chest[0] + dir[0] * r, chest[1] + dir[1] * r, chest[2] + dir[2] * r] : p.torchAt; l.v = p.torchDir; if (on) on[p.torch] = (p === b.player.body ? b.player.torch : h.night > 0.5 && !this.lampOff.has(p.torch)) ? 1 : 0; } void t;
  }
  /** the frame's numbers to the GPU */
  private send() {
    const [rw, rh] = this.graph.render, [ow, oh] = this.graph.output, c = this.camera.frame(ow / oh), p = this.prev ?? c, b = this.building, h = this.hourNow; this.sky.update(h);
    const f = new Float32Array(160), u = new Uint32Array(f.buffer); let o = 0; const v = (a: ArrayLike<number>, w = 0) => { f.set([a[0]!, a[1]!, a[2]!, w], o); o += 4; };
    v(c.pos); v(c.right); v(c.up); v(c.fwd); v(p.pos); v(p.right); v(p.up); v(p.fwd); v(h.sun, h.radius); v(h.sunE, this.torches > 0 ? CARDS : 0);
    f.set([rw, rh, ow, oh], o); o += 4; f.set([this.jitter[0], this.jitter[1], this.prevJitter[0], this.prevJitter[1]], o); o += 4; f.set([this.time, this.cut, this.exposure, h.haze * this.haze], o); o += 4; f.set([this.knobs[0]!, this.knobs[1]!, this.scene.joinRow, h.night * 2.2 * (this.mains ? 1 : 0)], o); o += 4;
    this.slots = (this.scene.sendAir(this.air), this.scene.sendRipples(this.ripples), this.scene.moved()); this.scene.cull(c, this.cut); this.scene.sendLamps();
    u.set([Math.min(64, b.world.lamps.length), Math.min(32, b.world.openings.length), this.frameNo, this.seed], o); o += 4; u.set(this.scene.per, o); o += 4; u.set(this.scene.node0, o); o += 4; u.set(this.scene.stepOf, o); o += 4; u.set(this.scene.piece0, o); o += 4; f.set(b.pool, o); o += 4; f.set([b.poolTop, b.poolDeep, this.gone, this.scene.airRow], o); o += 4; f.set([this.wet, this.rainNow, this.mains ? 1 : 0, Math.atan2(this.air.way[2], this.air.way[0])], o); o += 4; f.set([this.smoke.on ? 1 : 0, this.scene.noiseAt, this.otherDice, this.air.strength], o); o += 4; { const me = b.player.body.pos; this.tune.set(this.playing ? [me[0], me[1] + 1.0, me[2]] : [0, -999, 0], 25); } f.set(this.look(), o); f.set([this.snow, this.snowNow, Math.max(0, Math.min(0.1, this.time - this.drawnAt)), this.ripples.live ? 1 : 0], o + 32); this.drawnAt = this.time;
    this.gpu.device.queue.writeBuffer(this.scene.frame, 0, f); this.prev = c; this.prevJitter = this.jitter;
  }
  /** this frame's numbers, for a study that runs its stages itself */ sendFrame() { this.send(); this.frameNo++; }
  /** the knobs as they are now: the level's look at this hour, on what is turned */ look(): Float32Array { const f = this.looked; f.set(this.tune); graded(this.building.place.grade, this.hour, f); return f; } private looked = new Float32Array(32);
  /** how many torches have a picture of their own this frame */ torches = 0;
  sendMs = 0; private drawnAt = 0;
  /** draw one frame with the current method; `then` may add its own passes (the canvas) */
  draw(then?: (e: GPUCommandEncoder, result: GPUTexture) => void) {
    const { m, stages } = this.methods.get(this.current)!; if (this.jittered) { const n = (this.frameNo % 16) + 1, hal = (i: number, b: number) => { let r = 0, k = 1 / b; while (i > 0) { r += (i % b) * k; i = Math.floor(i / b); k /= b; } return r; }; this.jitter = [hal(n, 2) - 0.5, hal(n, 3) - 0.5]; } else this.jitter = [0, 0];
    const t0 = performance.now(); this.between(); this.smoke.flames(this.time); if (this.playing && this.fog) { const me = this.building.player.body, low = me.drive!.low, f = me.facing; this.uniform('sightK', new Float32Array([me.pos[0], me.pos[1] + 1.62 - 0.45 * low, me.pos[2], 1, f[0], 0, f[2], 0])); } else if (this.graph.buffers.has('sightK')) this.uniform('sightK', new Float32Array(8)); this.send(); if (this.remap) { if (this.remap === 8) joinAgain(this, this.holes[0]!); this.remap--; lampsAgain(this, this.remap, 8); } m.frame?.(this); this.sendMs = performance.now() - t0; const e = this.gpu.device.createCommandEncoder(); this.graph.timer.begin(); this.sway.run(e, this); this.water.run(e, this.time, this.hourNow.sun, this.building.poolDeep); this.graph.run(e, stages); then?.(e, this.graph.texture(m.result)); const done = this.graph.timer.end(e);
    this.gpu.device.queue.submit([e.finish()]); done?.(); this.graph.flip(); this.frameNo++; for (const p of this.building.world.moving) this.prevAt.set(p, { at: p.at, rot: p.rot });
  }
  /** how many holes there have been, and where the last were */
  broken = 0; readonly holes: { at: V3; r: number }[] = []; private readonly spares: Piece[] = []; private rubble: { body: Body; pieces: Piece[] }[] = [];
  /** blow a hole of about r where a ray from a along d first meets a wall */
  blow(a: V3, d: V3, r: number, force = 1, clean = false): Breach | null {
    const t0 = performance.now();    const b = this.building, w = b.world, far = b.solid.ray(a, d, 80, true); if (far >= 80) return null; const hit: V3 = [a[0] + d[0] * far, a[1] + d[1] * far, a[2] + d[2] * far], k = breach(w, hit, d, r, 7 + this.broken); if (!k || !this.scene.change(k.gone, k.come)) return null;
    const was = new Set(k.gone); w.pieces = w.pieces.filter(p => !was.has(p)).concat(k.come); b.solid.change(k.gone, k.come); this.near.change(k.gone, k.come); this.air.change((c, q) => b.solid.near(c, q), k.lo, k.hi); for (const l of k.lamps) { w.lamps[l]!.on = 0; this.scene.lampOn[l] = 0; }
    // the rubble: bodies made of what is kept for it (the oldest gives way)
    let seed = 1 + this.broken; const rnd = () => { seed = (seed * 16807) % 2147483647; return seed / 2147483647; }, fresh: Piece[] = [];
    for (const chunk of clean ? [] : k.rubble) { while (this.spares.length < chunk.length) { const old = this.rubble.shift(); if (!old) break; b.bodies.drop(old.body); for (const p of old.pieces) { p.at = [0, -100, 0]; this.spares.push(p); } } if (this.spares.length < chunk.length) break;
      const ps = chunk.map(c => { const p = this.spares.pop()!; Object.assign(p, { planes: undefined, marks: undefined, tint: undefined, seed: undefined }, c); return p; }), body = new Body(ps, ps[0]!.at, ps[0]!.rot, { dense: 1500, rough: 0.7 }); fresh.push(...ps); this.rubble.push({ body, pieces: ps }); b.bodies.add(body);
      const out: V3 = [body.x[0] - k.at[0], body.x[1] - k.at[1], body.x[2] - k.at[2]], l = Math.max(0.2, Math.hypot(...out)), q = force * (2.2 + 2.6 * rnd()) * (1.2 - 0.6 * Math.min(1, l / r)); body.v = [0, 1, 2].map(i => k.n[i]! * q + out[i]! / l * 1.1 * rnd() + (i === 1 ? 0.9 * rnd() : 0)) as V3; body.w = [rnd() * 8 - 4, rnd() * 8 - 4, rnd() * 8 - 4]; body.pose(); for (const p of ps) this.prevAt.set(p, { at: p.at, rot: p.rot }); }
    { const R = b.place.rooms, inRoom = (s: number) => { const x = [0, 1, 2].map(i => k.at[i]! + k.n[i]! * s); return R.some(q => [0, 1, 2].every(i => x[i]! > q[i]! && x[i]! < q[i + 3]!)); }, before = inRoom(-0.3), behind = inRoom(k.thick + 0.3); if (before !== behind && w.openings.length < 32) { w.openings.push({ at: k.at, u: [k.u[0] * r * 0.85, k.u[1] * r * 0.85, k.u[2] * r * 0.85], v: [k.v[0] * r * 0.85, k.v[1] * r * 0.85, k.v[2] * r * 0.85], n: before ? [-k.n[0], -k.n[1], -k.n[2]] : k.n }); this.scene.sendOpenings(); } }
    if (fresh.length) { this.scene.remade(fresh); for (const s of [-0.5, 0.1, k.thick + 0.6]) this.smoke.puff([0, 1, 2].map(i => k.at[i]! + k.n[i]! * s) as V3, r, 30); } for (const o of b.bodies.all) { const e: V3 = [o.x[0] - k.at[0], o.x[1] - k.at[1], o.x[2] - k.at[2]], l = Math.hypot(...e); if (l > r + 1.8 || o.parked) continue; o.wake(); if (this.rubble.some(q => q.body === o && fresh.includes(q.pieces[0]!))) continue; const push = force * 4.5 / (0.6 + l) / Math.max(1, o.m / 12); o.v = [0, 1, 2].map(i => o.v[i]! + (e[i]! / Math.max(l, 0.1) + k.n[i]! * 0.5) * push + (i === 1 ? 0.3 * push : 0)) as V3; } this.broken++; this.holes.unshift({ at: k.at, r }); if (this.holes.length > 4) this.holes.pop(); this.nowPose = new Float64Array(0); this.remap = 8; this.knocked = w.meshes ? knockOut(this, k) : 0; this.blowMs = performance.now() - t0; return k;
  }
  /** of what walls are faced with, how many the last hole took */ knocked = 0; blowMs = 0; private remap = 0;
  buffer(name: string, bytes: number, usage = GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_SRC) { let b = this.graph.buffers.get(name); if (!b || b.size < bytes) { b = buffer(this.gpu, name, bytes, usage); this.graph.buffers.set(name, b); } return b; }
  uniform(name: string, data: Float32Array | Uint32Array) { const b = this.graph.buffers.get(name) ?? this.buffer(name, Math.max(16, data.byteLength), GPUBufferUsage.UNIFORM); this.gpu.device.queue.writeBuffer(b, 0, data.buffer as ArrayBuffer, data.byteOffset, data.byteLength); return b; }
  read = (signal: string) => readTexture(this.gpu, this.graph.texture(signal)); readBuffer = (name: string) => readBuffer(this.gpu, this.graph.buffers.get(name)!);
}
