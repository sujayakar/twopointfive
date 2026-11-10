// THE LAB: the state of everything, and one frame after another.

import { openGpu, makeBuffer, readBack, type Gpu } from './gpu/device';
import { Graph } from './gpu/graph';
import { componentsOf, pack, unpack, type Struct, type Value } from './gpu/layout';
import { choiceConstants, knobStruct, onKnob, packKnobs, valueOf } from './page/knobs';
import { Present, type Panel } from './page/present';
import { Rays } from './page/rays';
import { AT_SIZE, Views } from './page/views';
import { addStages } from './stages/all';
import { Frame } from './stages/frame';
import { eyeOf, type Eye } from './world/camera';
import { World, WORLD_CONSTANTS } from './world/world';
import type { Camera, RoomSpec } from './world/room';
import { add, normalize, scale } from './world/vec';
import math from './wgsl/math.wgsl';
import dice from './wgsl/dice.wgsl';
import ray from './wgsl/ray.wgsl';
import sky from './wgsl/sky.wgsl';
import lampsCode from './wgsl/lamps.wgsl';
import material from './wgsl/material.wgsl';
import surface from './wgsl/surface.wgsl';
import frameCode from './wgsl/frame.wgsl';
import show from './wgsl/show.wgsl';

export type Layout = 'one' | 'two' | 'four' | 'wipe';
const GRID: Record<Layout, [number, number]> = { one: [1, 1], two: [2, 1], four: [2, 2], wipe: [1, 1] };
export const panelsIn: Record<Layout, number> = { one: 1, two: 2, four: 4, wipe: 2 };

export interface Pinned {
  name: string;
  about: string;
  struct: Struct;
  values: Record<string, Value>;
}

/** a low-discrepancy walk over the pixel: where in it this frame's ray goes */
function jitterOf(frame: number): [number, number] {
  const halton = (base: number) => {
    let f = 1;
    let out = 0;
    for (let i = (frame % 16) + 1; i > 0; i = Math.floor(i / base)) {
      f /= base;
      out += f * (i % base);
    }
    return out - 0.5;
  };
  return [halton(2), halton(3)];
}

export class Lab {
  readonly graph: Graph;
  readonly world: World;
  readonly views: Views;
  readonly rays: Rays;
  layout: Layout = 'one';
  /** what each panel shows, by view id */
  shown: string[] = ['hit_x.normal', 'hit_x.matte', 'hit_x.position', 'hit_x.roughness'];
  wipe = 0.5;
  zoom = 1;
  middle: [number, number] = [0.5, 0.5];
  camera!: Camera;
  ghost: Camera | null = null;
  playing = false;
  /** the frame stands still; only the pictures of it are drawn again */
  frozen = false;
  private oneMore = 0;
  pinned: [number, number] | null = null;
  tick = 0;
  frames = 0;
  truthCount = 0;
  ready = false;
  /** the stages of the last frame that was made anew */
  made: string[] = [];
  onError: (text: string) => void = () => {};

  private knobs: Struct;
  private frameBuffer: GPUBuffer;
  private knobBuffer: GPUBuffer;
  private present: Present;
  private before: Eye | null = null;
  private jitterBefore: [number, number] = [0, 0];
  private cameraKey = '';
  private worldVersion = -1;
  private startTruthOver = true;

  private constructor(readonly gpu: Gpu, readonly canvas: HTMLCanvasElement) {
    this.graph = new Graph(gpu, () => `${choiceConstants()}\n${WORLD_CONSTANTS}\n${math}`);
    this.world = new World(gpu, this.graph);
    this.views = new Views(this.graph);
    this.rays = new Rays(gpu, this.graph);
    this.addModules();
    addStages(this.graph, this.views);
    this.knobs = knobStruct();
    this.frameBuffer = makeBuffer(gpu, 'frame', Frame.stride, GPUBufferUsage.UNIFORM);
    this.knobBuffer = makeBuffer(gpu, 'knobs', this.knobs.stride, GPUBufferUsage.UNIFORM);
    this.graph.addShared('frame', { type: 'Frame', structs: [Frame], access: 'uniform', buffer: () => this.frameBuffer });
    this.graph.addShared('knobs', { type: 'Knobs', structs: [this.knobs], access: 'uniform', buffer: () => this.knobBuffer });
    this.present = new Present(gpu);
    onKnob((k) => {
      if (k.resetsTruth) this.startTruthOver = true;
    });
  }

  static async open(canvas: HTMLCanvasElement, onError: (text: string) => void): Promise<Lab> {
    const gpu = await openGpu(canvas, onError);
    const lab = new Lab(gpu, canvas);
    lab.onError = onError;
    return lab;
  }

  private addModules(): void {
    const g = this.graph;
    g.addModule('frame', { code: frameCode, needs: [], shared: ['frame', 'knobs'] });
    g.addModule('dice', { code: dice, needs: [], shared: ['frame', 'knobs', 'tables'] });
    g.addModule('ray', { code: ray, needs: [], shared: ['world', 'nodes', 'pieces', 'planes'] });
    g.addModule('sky', { code: sky, needs: [], shared: ['world'] });
    g.addModule('lamps', { code: lampsCode, needs: [], shared: ['world', 'lamps'] });
    g.addModule('material', { code: material, needs: [], shared: ['tables'] });
    g.addModule('surface', { code: surface, needs: ['ray', 'material'], shared: ['materials', 'lamps', 'objects', 'pieces'] });
    g.addModule('show', { code: show + AT_SIZE, needs: [], shared: [] });
  }

  openRoom(room: RoomSpec, sliders?: Record<string, number>): void {
    const sameRoom = this.world.room?.id === room.id;
    this.world.open(room, sliders);
    if (!sameRoom) {
      this.resetCamera();
      this.tick = 0;
      this.frozen = false;
      this.pinned = null;
    }
    this.world.place(this.tick / 60);
    this.startTruthOver = true;
    this.graph.forget();
  }
  resetCamera(): void {
    this.camera = structuredClone(this.world.room.camera);
    this.ghost = null;
  }
  step(): void {
    this.oneMore = 1;
  }

  /** the panels' places on the canvas, device pixels */
  panelRects(): { x: number; y: number; width: number; height: number }[] {
    const [columns, rows] = GRID[this.layout];
    const GAP = 2;
    const width = Math.floor((this.canvas.width - GAP * (columns - 1)) / columns);
    const height = Math.floor((this.canvas.height - GAP * (rows - 1)) / rows);
    const rects = [];
    for (let r = 0; r < rows; r++) {
      for (let c = 0; c < columns; c++) rects.push({ x: c * (width + GAP), y: r * (height + GAP), width, height });
    }
    return this.layout === 'wipe' ? [rects[0]!, rects[0]!] : rects;
  }

  /** a canvas pixel as a place in the picture, 0 to 1 */
  pictureAt(x: number, y: number): [number, number] | null {
    for (const r of this.panelRects()) {
      const u = (x - r.x) / r.width;
      const v = (y - r.y) / r.height;
      if (u < 0 || v < 0 || u >= 1 || v >= 1) continue;
      const at: [number, number] = [this.middle[0] + (u - 0.5) / this.zoom, this.middle[1] + (v - 0.5) / this.zoom];
      return at.every((a) => a >= 0 && a < 1) ? at : null;
    }
    return null;
  }

  frame(): void {
    const { graph, gpu, world } = this;
    const rect = this.panelRects()[0]!;
    if (rect.width < 8 || rect.height < 8) return;
    const output: [number, number] = [rect.width, rect.height];
    const sharp = valueOf('render_scale');
    const render: [number, number] = [Math.max(8, Math.round(output[0] * sharp)), Math.max(8, Math.round(output[1] * sharp))];
    if (graph.resize(render, output)) this.startTruthOver = true;

    const runs = !this.frozen || this.oneMore > 0;
    if (this.oneMore > 0) this.oneMore--;
    if (runs && this.playing && world.room.moves) {
      this.tick++;
      if (world.place(this.tick / 60)) this.startTruthOver = true;
    } else if (runs) {
      world.place(this.tick / 60);
    }
    const cameraKey = JSON.stringify(this.camera);
    if (cameraKey !== this.cameraKey || world.version !== this.worldVersion) this.startTruthOver = true;
    this.cameraKey = cameraKey;
    this.worldVersion = world.version;

    const encoder = gpu.device.createCommandEncoder();
    if (this.startTruthOver) {
      graph.clear(encoder, 'truth');
      this.truthCount = 0;
      this.startTruthOver = false;
    }
    if (runs) this.rays.start(encoder);
    this.writeUniforms(render, output, runs);
    const views = this.shown.slice(0, panelsIn[this.layout]);
    const wanted = views.map((id) => this.views.bufferOf(id));
    this.ready = graph.run(encoder, wanted, runs ? undefined : (name) => name.startsWith('view '));
    if (runs) this.made = graph.ran;
    const ran = runs && graph.ran.includes('truth');
    const panels: Panel[] = this.panelRects().map((r, i) => ({
      ...r,
      picture: graph.buffer(wanted[i]!),
      middle: this.middle,
      zoom: this.zoom,
      from: this.layout === 'wipe' && i === 1 ? this.wipe : 0,
      to: this.layout === 'wipe' && i === 0 ? this.wipe : 1,
    }));
    this.present.draw(encoder, panels, output);
    const after = graph.timer.end(encoder);
    gpu.device.queue.submit([encoder.finish()]);
    after();
    if (runs) {
      if (ran) this.truthCount++;
      this.frames++;
      graph.flip();
    }
  }

  /** the eye whose picture a panel shows */
  eyeOfPanel(i: number): Eye {
    const aspect = this.graph.outputSize[0] / this.graph.outputSize[1];
    return eyeOf(this.ghost && this.shown[i] === 'ghost' ? this.ghost : this.camera, aspect);
  }

  private writeUniforms(render: [number, number], output: [number, number], runs: boolean): void {
    const now = eyeOf(this.camera, output[0] / output[1]);
    const before = this.before ?? now;
    const jitter: [number, number] = valueOf('jitter') ? jitterOf(this.frames) : [0, 0];
    const ghost = this.ghost ? eyeOf(this.ghost, output[0] / output[1]) : now;
    const bytes = new ArrayBuffer(Frame.stride);
    pack(Frame, new DataView(bytes), 0, {
      eye: now.eye, forward: now.forward, right: now.right, up: now.up,
      eye_before: before.eye, forward_before: before.forward, right_before: before.right, up_before: before.up,
      ghost_eye: ghost.eye, ghost_forward: ghost.forward, ghost_right: ghost.right, ghost_up: ghost.up,
      ghost_on: this.ghost ? 1 : 0,
      number: this.frames, tick: this.tick, seconds: this.tick / 60, truth_number: this.truthCount,
      render_size: render, half_size: this.graph.halfSize, output_size: output,
      pinned_on: this.pinned ? 1 : 0,
      pinned: this.pinned ? [Math.floor(this.pinned[0] * render[0]), Math.floor(this.pinned[1] * render[1])] : [0, 0],
      jitter, jitter_before: this.jitterBefore,
    });
    const { queue } = this.gpu.device;
    queue.writeBuffer(this.frameBuffer, 0, bytes);
    queue.writeBuffer(this.knobBuffer, 0, packKnobs(this.knobs));
    if (runs) {
      this.before = now;
      this.jitterBefore = jitter;
    }
  }

  /** the pinned pixel's value in every buffer that was made this frame */
  async readPinned(): Promise<Pinned[]> {
    if (!this.pinned) return [];
    const { graph } = this;
    const out: Pinned[] = [];
    const names = new Set(
      this.made.flatMap((name) => {
        const stage = graph.stages.find((s) => s.name === name)!;
        const writes = Array.isArray(stage.writes) ? stage.writes : Object.values(stage.writes);
        return writes.map((w) => graph.resolve(w));
      }),
    );
    for (const name of names) {
      if (name.startsWith('view ')) continue;
      const spec = graph.spec(name);
      if (typeof spec.size === 'object') continue;
      const [w, h] = graph.pixels(spec.size);
      const index = Math.floor(this.pinned[1] * h) * w + Math.floor(this.pinned[0] * w);
      const bytes = await readBack(this.gpu, graph.buffer(name), index * spec.struct.stride, spec.struct.stride);
      out.push({ name, about: spec.about, struct: spec.struct, values: unpack(spec.struct, bytes) });
    }
    return out;
  }

  /** the mean of a field over the picture's pixels, and their spread: for checks */
  async statistics(name: string, key: string): Promise<{ mean: number[]; spread: number[]; least: number[]; most: number[] }> {
    const spec = this.graph.spec(name);
    const count = this.graph.elements(spec.size);
    const data = await readBack(this.gpu, this.graph.buffer(name), 0, count * spec.struct.stride);
    const n = componentsOf(spec.struct.fields[key]!.type);
    const sum = Array(n).fill(0), square = Array(n).fill(0), least = Array(n).fill(Infinity), most = Array(n).fill(-Infinity);
    for (let i = 0; i < count; i++) {
      for (let c = 0; c < n; c++) {
        const v = data.getFloat32(i * spec.struct.stride + spec.struct.offsets[key]! + 4 * c, true);
        sum[c] += v;
        square[c] += v * v;
        least[c] = Math.min(least[c], v);
        most[c] = Math.max(most[c], v);
      }
    }
    const mean = sum.map((s) => s / count);
    return { mean, spread: square.map((s, c) => Math.sqrt(Math.max(0, s / count - mean[c]! ** 2))), least, most };
  }

  /** the way through a place in the picture */
  toward(at: [number, number]): [number, number, number] {
    const e = eyeOf(this.camera, this.graph.outputSize[0] / this.graph.outputSize[1]);
    const x = at[0] * 2 - 1;
    const y = 1 - at[1] * 2;
    return normalize(add(e.forward, add(scale(e.right, x), scale(e.up, y))));
  }
}
