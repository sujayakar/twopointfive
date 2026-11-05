// THE LOOK PROTOTYPE'S PLACE FIELD — which PLACE every point of the floor plan is in, as one number: the light that falls on its floor (log2), the same
// from a room's window to its far end and another on the other side of its wall. The one exposure model (render/lookexpo.ts, shaders/place_common.wgsl)
// shows every place by that number against the number of the place Sam stands in. Before it the composite asked the radiance cache at each PIXEL (four cells
// and their sight lines; that read is gone from the kernel since 11-04, composite.wgsl lookPlaceLevels): a room's own fall-off from its window was then part of "the place", and bringing each spot up by its own darkness
// flattened the room, which is what the rows this model replaces kept correcting (the maintainer, 10-31: "i worry that we're finding these cases one by one
// and not fixing them systematically").
// WHAT LIVES HERE: the field's buffer and its solve (shaders/place_field.wgsl), and Sam's eye, eased on the GPU from the field under his feet. THE MAP it
// is solved on (which cells are wall: the level's tall boxes and each shut door's line; an open door's cells say how far it has swung) is scene/placePlan.ts's since 11-04: plain arithmetic
// that the game's CPU light may read too. PICTURE ONLY, this file: nothing in the simulation reads the field, it takes no die and no wall clock, and it
// imports nothing from src/game.
// THE BUFFER IS READ AS A UNIFORM by the composite: the fragment stage has no sampled texture left to bind (render/passes.ts compEntries).
// NEVER SEEN ON A GPU: judged on Chromium's software adapter, from stills (tools/qa/look-sheet.cjs) and the read-back map.
import { makeBuffer, makeShader, computePipelineAsync } from '../gpu/device';
import type { Box } from '../scene/boxes';
import { WORLD } from '../scene/world';
import { PLACE, PLACE_W, PLACE_H, PLACE_RES, PlaceMap, type PlaceGap } from '../scene/placePlan';
import placeSrc from './shaders/place_field.wgsl' with { type: 'text' };

/** the plan's own table and size, for whoever reads the field's buffer beside them (the plan itself, and what a wall or a doorway is: scene/placePlan.ts) */
export { PLACE, PLACE_W, PLACE_H, PLACE_RES } from '../scene/placePlan';
/** the words after the cells: Sam's eye, 1 once the field is live, the open air's low level and its middle (place_field.wgsl placeEye) */
export const PLACE_WORDS = 4;
/** where the field's SECOND BLOCK begins, a number a cell again: the brightest place a stride away (place_field.wgsl hand; PLACE.near) */
export const PLACE_NEAR = PLACE_W * PLACE_H + PLACE_WORDS;
/** the buffer's size in bytes: the cells, the words after them and the second block, rounded up to whole vec4f (place_common.wgsl PlaceBuf) */
export const PLACE_BYTES = Math.ceil((PLACE_NEAR + PLACE_W * PLACE_H) / 4) * 16;
const NONE = new WeakMap<GPUDevice, GPUBuffer>();
/** the zeros that say "no field" to a program that binds one (never written), one a device: what the composite, the smoke march and the particles' splat
 *  are handed off the look page and until the field is live */
export function placeNone(device: GPUDevice): GPUBuffer { let b = NONE.get(device); if (!b) { b = device.createBuffer({ label: 'placeNone', size: PLACE_BYTES, usage: GPUBufferUsage.UNIFORM }); NONE.set(device, b); } return b; }
const U_BYTES = 64;
/** the open air's table, a word each: 64 counts, 64 sums and the brightest place (place_field.wgsl placeSky writes it, placeEye reads and empties it) */
export const SKY_WORDS = 129;
export class PlaceField {
  /** what the composite binds as a uniform: PLACE_W × PLACE_H levels (log2; −1000 = a wall's cell or a doorway's: no level), then PLACE_WORDS words: the eye's level, 1 once the field is live,
   *  THE OPEN AIR's LOW level on this map (PLACE.skyShare: the level a tenth of the cells under the open sky are darker than; a walled yard's and not open
   *  country's) and its MIDDLE (the level half of them are darker than) */
  readonly buffer: GPUBuffer;
  ready = false;
  readonly whenReady: Promise<void>;
  /** maps drawn and sweeps encoded since it was built (a tool's) */
  drawn = 0; swept = 0;
  readonly mask = new Uint8Array(PLACE_W * PLACE_H);
  private readonly device: GPUDevice;
  private readonly maskTex: GPUTexture; private readonly ubo: GPUBuffer[] = []; private readonly skyBuf: GPUBuffer;
  private sweep: GPUComputePipeline | null = null; private sky: GPUComputePipeline | null = null; private eye: GPUComputePipeline | null = null; private readonly bind: GPUBindGroup[] = [];
  private readonly own = new PlaceMap(); private sentMap: PlaceMap | null = null; private sentDrawn = -1; private dead = false; private fresh = true;
  private readonly bgl: GPUBindGroupLayout; private readonly maskView: GPUTextureView; private dice: GPUTextureView | null = null;
  private readonly u = new ArrayBuffer(U_BYTES); private readonly uf = new Float32Array(this.u); private readonly uu = new Uint32Array(this.u);

  constructor(device: GPUDevice, dice: GPUTextureView) {
    this.device = device; const B = GPUBufferUsage, C = GPUShaderStage.COMPUTE;
    this.buffer = makeBuffer(device, PLACE_BYTES, B.STORAGE | B.UNIFORM | B.COPY_DST | B.COPY_SRC, 'place field');
    this.skyBuf = makeBuffer(device, SKY_WORDS * 4, B.STORAGE | B.COPY_SRC, 'place sky');   // (zeros as made: an empty table)
    this.maskTex = device.createTexture({ label: 'place map', size: [PLACE_W, PLACE_H], format: 'r8uint', usage: GPUTextureUsage.TEXTURE_BINDING | GPUTextureUsage.COPY_DST });
    const bgl = device.createBindGroupLayout({ label: 'place', entries: [
      { binding: 0, visibility: C, buffer: { type: 'uniform' } }, { binding: 1, visibility: C, texture: { sampleType: 'float', viewDimension: '3d' } },
      { binding: 3, visibility: C, texture: { sampleType: 'uint' } }, { binding: 4, visibility: C, buffer: { type: 'storage' } }, { binding: 5, visibility: C, buffer: { type: 'storage', minBindingSize: SKY_WORDS * 4 } }] });   // (the table's size is held where the pipelines are made, which the trial below reaches, and not first at a frame's dispatch)
    const layout = device.createPipelineLayout({ label: 'place', bindGroupLayouts: [bgl] }), module = makeShader(device, placeSrc, 'place_field');
    this.whenReady = Promise.all([
      computePipelineAsync(device, { label: 'place sweep', layout, compute: { module, entryPoint: 'placeSweep' } }).then(p => { this.sweep = p; }),
      computePipelineAsync(device, { label: 'place sky', layout, compute: { module, entryPoint: 'placeSky' } }).then(p => { this.sky = p; }),
      computePipelineAsync(device, { label: 'place eye', layout, compute: { module, entryPoint: 'placeEye' } }).then(p => { this.eye = p; }),
    ]).then(() => {
      // a compile the device refused comes back as an INVALID pipeline (gpu/device.ts computePipelineAsync), and setting one on the frame's encoder would void
      // the whole frame. So each is set once on an encoder of its own, under an error scope: an error there leaves the field off (every place is then the open
      // air and none takes a gain, composite.wgsl lookPlaceLevels) and the frame is never handed either.
      const scoped = typeof device.pushErrorScope === 'function' && typeof device.popErrorScope === 'function' && typeof GPUValidationError !== 'undefined';
      if (!scoped) { this.ready = true; return; }
      device.pushErrorScope('validation');
      try { const enc = device.createCommandEncoder({ label: 'place pipelines tried' }), pass = enc.beginComputePass(); pass.setPipeline(this.sweep!); pass.setPipeline(this.sky!); pass.setPipeline(this.eye!); pass.end(); device.queue.submit([enc.finish()]); } catch { /* the scope says */ }
      return Promise.resolve(device.popErrorScope()).then(err => {
        if (err instanceof GPUValidationError) { this.sweep = this.sky = this.eye = null; console.warn('place field: not built on this device', err.message); } else this.ready = true;
      }, () => { /* a lost device: its own handler speaks */ });
    });
    this.bgl = bgl; this.maskView = this.maskTex.createView();
    for (let k = 0; k < 2; k++) this.ubo.push(makeBuffer(device, U_BYTES, B.UNIFORM | B.COPY_DST, `place uniforms ${k}`));   // (one uniform block a colour: both sweeps of a frame are encoded before either block could be written again)
    this.bindTo(dice);
    this.clear();
  }

  /** the cache's dice volume the field reads: a rebuilt cache (another preset) has another, and the two groups are made again for it */
  private bindTo(dice: GPUTextureView): void {
    if (this.dice === dice) return; this.dice = dice; this.bind.length = 0;
    for (let k = 0; k < 2; k++) this.bind.push(this.device.createBindGroup({ label: `place ${k}`, layout: this.bgl, entries: [
      { binding: 0, resource: { buffer: this.ubo[k] } }, { binding: 1, resource: dice }, { binding: 3, resource: this.maskView }, { binding: 4, resource: { buffer: this.buffer } }, { binding: 5, resource: { buffer: this.skyBuf } }] }));
  }

  /** the map follows the level as it stands: drawn again only when a wall, a pane's box or a leaf has changed. True when it was drawn. */
  setBoxes(statics: readonly Box[], dynamics: readonly Box[], gaps: readonly PlaceGap[] = []): boolean { this.own.update(statics, dynamics, gaps); return this.setMap(this.own); }
  /** … from a map the caller keeps (the engine's, which the wall sides read too). True when it was sent. */
  setMap(map: PlaceMap): boolean {
    if (this.dead || (map === this.sentMap && map.drawn === this.sentDrawn)) return false;
    this.sentMap = map; this.sentDrawn = map.drawn; this.mask.set(map.mask); this.drawn++;
    this.device.queue.writeTexture({ texture: this.maskTex }, this.mask, { bytesPerRow: PLACE_W }, [PLACE_W, PLACE_H]); return true;
  }

  /** this frame's sweeps and the eye, encoded on `enc` AFTER the radiance cache's own pass (`dice` = its volume's view as it is this frame). `sam` = where Sam stands (world x, z; null = nobody: the eye
   *  keeps what it has); `snap` = the eye takes its reading outright, and the field is solved to rest first (a first frame, a restart, a still). */
  step(enc: GPUCommandEncoder, dice: GPUTextureView, dt: number, sam: readonly [number, number] | null, snap: boolean): void {
    if (this.dead || !this.sweep || !this.sky || !this.eye) return;
    this.bindTo(dice);
    const P = PLACE, first = this.fresh || snap; this.fresh = false;
    const kappa = (P.reach * PLACE_RES) ** 2, ease = (s: number) => (dt > 0 ? 1 - Math.exp(-Math.min(dt, 0.25) / Math.max(s, 1e-3)) : 0);
    for (let k = 0; k < 2; k++) {
      const f = this.uf, u = this.uu; u[0] = k; u[1] = first ? 1 : 0; f[2] = kappa; f[3] = P.omega;
      f[4] = sam ? (sam[0] - WORLD.rcMin[0]) * PLACE_RES : -1; f[5] = sam ? (sam[1] - WORLD.rcMin[2]) * PLACE_RES : -1; f[6] = ease(P.open); f[7] = ease(P.close);
      f[8] = Math.min(0.95, Math.max(0.05, (P.y - WORLD.rcMin[1]) / WORLD.rcSize[1])); f[9] = P.dark; f[10] = 0; f[11] = ease(P.skyEase); f[12] = P.skyShare; f[13] = P.near; f[14] = P.nearDoor;
      this.device.queue.writeBuffer(this.ubo[k], 0, this.u);
    }
    const n = first ? P.settle : P.sweeps, gx = Math.ceil(PLACE_W / 8), gy = Math.ceil(PLACE_H / 8);
    const pass = enc.beginComputePass({ label: 'place' }); pass.setPipeline(this.sweep);
    for (let i = 0; i < n; i++) for (let k = 0; k < 2; k++) { pass.setBindGroup(0, this.bind[k]); pass.dispatchWorkgroups(gx, gy); }
    pass.setPipeline(this.sky); pass.setBindGroup(0, this.bind[0]); pass.dispatchWorkgroups(gx, gy);   // (the open air counted by every cell at once, into the table the eye reads)
    pass.setPipeline(this.eye); pass.dispatchWorkgroups(1);   // (every frame: it also keeps the live word and the brightest place)
    pass.end(); this.swept += n;
  }

  /** every cell without an answer and the eye without a reading: how the field starts. (A restart needs none of it: the snap that follows solves to rest from whatever is there.) */
  private clear(): void {
    const f = new Float32Array(PLACE_BYTES / 4).fill(-1000); f[PLACE_W * PLACE_H + 1] = 0;
    this.device.queue.writeBuffer(this.buffer, 0, f); this.fresh = true;
  }

  /** the open air's table read back (a tool's): SKY_WORDS words, all zero between frames */
  async readSky(): Promise<Uint32Array> {
    if (this.dead) return new Uint32Array(SKY_WORDS);
    const buf = this.device.createBuffer({ label: 'place sky read-back', size: SKY_WORDS * 4, usage: GPUBufferUsage.COPY_DST | GPUBufferUsage.MAP_READ });
    const enc = this.device.createCommandEncoder(); enc.copyBufferToBuffer(this.skyBuf, 0, buf, 0, SKY_WORDS * 4); this.device.queue.submit([enc.finish()]);
    await buf.mapAsync(GPUMapMode.READ); const res = new Uint32Array(buf.getMappedRange().slice(0)); buf.unmap(); buf.destroy(); return res;
  }

  destroy(): void { if (this.dead) return; this.dead = true; this.buffer.destroy(); this.skyBuf.destroy(); this.maskTex.destroy(); for (const b of this.ubo) b.destroy(); }

  /** the field read back (a tool's): PLACE_W × PLACE_H levels, then the eye, the live word and the open air's two, then from PLACE_NEAR the second block */
  async readBack(): Promise<Float32Array> {
    if (this.dead) return new Float32Array(PLACE_BYTES / 4);
    const buf = this.device.createBuffer({ label: 'place read-back', size: PLACE_BYTES, usage: GPUBufferUsage.COPY_DST | GPUBufferUsage.MAP_READ });
    const enc = this.device.createCommandEncoder(); enc.copyBufferToBuffer(this.buffer, 0, buf, 0, PLACE_BYTES); this.device.queue.submit([enc.finish()]);
    await buf.mapAsync(GPUMapMode.READ); const res = new Float32Array(buf.getMappedRange().slice(0)); buf.unmap(); buf.destroy(); return res;
  }
}
