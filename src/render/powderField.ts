// THE LOOK PROTOTYPE'S LOOSE SNOW — the powder the wind carries over lying snow and the puff a boot throws, as ONE LAYER carried by the game's own air
// (shaders/powder_field.wgsl has the model; render/airField.ts is the air). The maintainer, 10-28, of the first try: the blowing snow "doesn't look like it
// reacts to walls, and it seems to imply a wind velocity that doesn't match the wind velocity for the falling snow"; the puffs "look like particle effects".
// So the powder has no wind of its own and a puff is not a sprite: both are stuff in one field over the level's floor plan, moved by the air the flakes ride,
// and drawn on the pixel under them with that pixel's own light and shadow (composite.wgsl lookPowder).
// PICTURE ONLY: nothing in the simulation reads it, it takes no die and no wall clock, and it imports nothing from src/game. What lives here: the two textures,
// the numbers (POWDER), a step's encoding (one dispatch and a copy, one uniform write), the kicks' short lives, and the read-back for the test page.
// NEVER SEEN ON A GPU: every number below was judged on Chromium's software adapter, from pictures of the read-back field (tools/qa/powder-field-page.cjs).
import { makeBuffer, makeShader, computePipelineAsync } from '../gpu/device';
import type { AirField } from './airField';
import { AIR_RES } from './airField';
import powSrc from './shaders/powder_field.wgsl' with { type: 'text' };

/** cells per metre (0.1 m cells: a thread of powder is a hand wide) */
export const POW_RES = 10;
/** kicks alive at once at most (powder_field.wgsl PowU.kicks) */
export const POW_KICKS = 16;
/** the powder's numbers (authored data, live: read every step) */
export const POWDER = {
  velK: 1.08,        // × the air field's velocity: the flakes' own height share a hand over the ground (particles.wgsl snowAir: 0.55 at the ground → 1 at 2.5 m, ÷ 0.55)
  life: 12,           // s powder stays up in air that moves
  lifeStill: 1.4,    // s in still air (a lee): it drops out
  pick: 1.0,         // column picked up a second at a full crest in a full gust (1 = hides 63 % of the ground seen from above)
  liftPick: 1.5,     // column lifted a second per m/s of the air's lift (a wall's windward foot)
  gateLo: 0.6,       // × the ambient wind: air slower than this lifts nothing …
  gateHi: 1.15,      // … and at this it lifts in full (a jet round a corner is over it; the open wind is between)
  patch: 4.5,        // m: a gust patch's size
  crest: 0.28,       // m: a crest's size (the thread it gives is about this wide)
  crestCut: 0.84,    // 0‥1: the share of the ground that is NOT a crest
  change: 0.11,      // 1/s: how fast patches and crests come and go where they stand
  low: 0.12,         // m: the height wind-lifted powder is put in at
  liftHigh: 0.6,     // m more per m/s of lift
  sink: 5,           // s: the height's own life (powder sinks as it travels)
  clump: 1.3,        // 1/s per unit of column: thick powder falls out faster than thin (a puff's middle goes in a second; what is left of it the wind keeps)
  seed: 3,
  // A KICK (kick(): a boot, a body, a blast), by its force (a walker's boot about 0.45, a sprinter's 1.2, a blast 2)
  kickColumn: 1.8,   // column at its middle per unit of force, summed over its life
  kickSigma: 0.09,   // m: how wide it starts …
  kickGrow: 0.3,     // … + this × force by the end of its life
  kickSecs: 0.35,    // s it goes on swelling
  kickHigh: 0.2,     // m: the height it is thrown to …
  kickHighK: 0.22,   // … + this × force
  kickGo: 0.5,       // m/s per unit of force its middle travels the way the boot was going, dying over its life
};

const U_HEAD = 32, U_WORDS = U_HEAD + POW_KICKS * 8;   // powder_field.wgsl PowU: 128 bytes, then the kicks (2 × vec4f each)
interface Kick { x: number; z: number; force: number; age: number; dx: number; dz: number; seed: number }

export class PowderField {
  readonly w: number; readonly h: number;
  /** what the picture reads: rgba16float, w × h, texel (i, j) = cell (i, j)'s centre (world x = (i + ½) / POW_RES): r = the powder's column, g = column × height (m),
   *  b, a = the air's velocity there. The same view object for the instance's life. */
  readonly view: GPUTextureView;
  ready = false;
  readonly whenReady: Promise<void>;
  /** kicks taken since it was built, and those dropped because sixteen were alive */
  kicked = 0; dropped = 0;
  private readonly device: GPUDevice;
  private readonly cur: GPUTexture; private readonly nxt: GPUTexture; private readonly ubo: GPUBuffer;
  private pipe: GPUComputePipeline | null = null; private readonly bind: GPUBindGroup;
  private readonly u = new ArrayBuffer(U_WORDS * 4); private readonly uf = new Float32Array(this.u); private readonly uu = new Uint32Array(this.u);
  private readonly kicks: Kick[] = [];
  private readonly airW: number; private readonly airH: number;
  private dead = false;

  constructor(device: GPUDevice, air: Pick<AirField, 'w' | 'h' | 'view' | 'cells'>) {
    this.device = device; this.airW = air.w; this.airH = air.h;
    const k = POW_RES / AIR_RES, w = this.w = Math.max(1, Math.round(air.w * k)), h = this.h = Math.max(1, Math.round(air.h * k));
    const T = GPUTextureUsage;
    this.cur = device.createTexture({ label: 'powder', size: [w, h], format: 'rgba16float', usage: T.TEXTURE_BINDING | T.COPY_DST | T.COPY_SRC });
    this.nxt = device.createTexture({ label: 'powder next', size: [w, h], format: 'rgba16float', usage: T.STORAGE_BINDING | T.COPY_SRC });
    this.view = this.cur.createView();
    this.ubo = makeBuffer(device, U_WORDS * 4, GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST, 'powder uniforms');
    const C = GPUShaderStage.COMPUTE;
    const bgl = device.createBindGroupLayout({ label: 'powder', entries: [
      { binding: 0, visibility: C, buffer: { type: 'uniform' } }, { binding: 1, visibility: C, texture: { sampleType: 'unfilterable-float' } }, { binding: 2, visibility: C, texture: { sampleType: 'uint' } },
      { binding: 3, visibility: C, texture: { sampleType: 'unfilterable-float' } }, { binding: 4, visibility: C, storageTexture: { access: 'write-only', format: 'rgba16float' } }] });
    const layout = device.createPipelineLayout({ label: 'powder', bindGroupLayouts: [bgl] }); const module = makeShader(device, powSrc, 'powder_field');
    this.whenReady = computePipelineAsync(device, { label: 'powder step', layout, compute: { module, entryPoint: 'powderStep' } }).then(p => { this.pipe = p; this.ready = true; });
    this.bind = device.createBindGroup({ label: 'powder', layout: bgl, entries: [
      { binding: 0, resource: { buffer: this.ubo } }, { binding: 1, resource: air.view }, { binding: 2, resource: air.cells },
      { binding: 3, resource: this.view }, { binding: 4, resource: this.nxt.createView() }] });
    this.reset();
  }

  /** A KICK at (x, z) metres: a round of powder put in there over POWDER.kickSecs, by its `force` (0 = nothing), its middle carried a little the way (dx, dz)
   *  the boot was going. Sixteen at once; one more drops the oldest. */
  kick(x: number, z: number, force: number, dx = 0, dz = 0): void {
    if (this.dead || !(force > 0) || !Number.isFinite(x + z + force)) return;
    const l = Math.hypot(dx, dz), n = l > 1e-4 && Number.isFinite(l) ? 1 / l : 0;
    if (this.kicks.length >= POW_KICKS) { this.kicks.shift(); this.dropped++; }
    this.kicks.push({ x, z, force: Math.min(force, 2.5), age: 0, dx: dx * n, dz: dz * n, seed: (this.kicked * 7.31) % 64 }); this.kicked++;
  }

  /** one step, encoded on `enc` (one compute pass and a copy), AFTER the air's own step of the same frame. `ambient` = the speed of the wind the air field was
   *  handed this frame (m/s); `time` = the caller's clock; `pickK` = × what the wind picks up (0 = kicks only); `airLive` false = the air field is not being stepped this
   *  frame, so what its texture still holds is a moment that is gone: the powder takes the air for still (nothing is carried, and it drops out as in a lee).
   *  dt ≤ 0 encodes nothing; dt is clamped to 1/30. ONE step per submitted encoder (the uniforms are a queue write). */
  step(enc: GPUCommandEncoder, dt: number, time: number, ambient: number, pickK = 1, airLive = true): void {
    if (this.dead || !this.pipe || !(dt > 0)) return;
    dt = Math.min(dt, 1 / 30);
    const P = POWDER, f = this.uf, u = this.uu, keep = (life: number) => Math.exp(-dt / Math.max(life, 1e-3));
    f[0] = dt; f[1] = time; f[2] = airLive ? P.velK : 0; f[3] = POW_RES;
    f[4] = keep(P.life); f[5] = keep(P.lifeStill); f[6] = Math.max(0, P.pick * pickK) * dt; f[7] = Math.max(0, P.liftPick * pickK) * dt;
    f[8] = Number.isFinite(ambient) ? Math.max(0, ambient) : 0; f[9] = P.gateLo; f[10] = Math.max(P.gateHi, P.gateLo + 1e-3); f[11] = keep(P.sink);
    f[12] = Math.max(P.patch, 0.2); f[13] = Math.max(P.crest, 1 / POW_RES); f[14] = P.crestCut; f[15] = P.change;
    f[16] = P.low; f[17] = P.liftHigh; f[18] = P.seed; f[19] = Math.max(0, P.clump) * dt;
    u[20] = this.w; u[21] = this.h; u[22] = this.airW; u[23] = this.airH;
    f[28] = AIR_RES; f[29] = 0; f[30] = 0; f[31] = 0;
    // the kicks: each gives its column over its life, most at the start, as a round that swells and travels a little
    let n = 0; const T = Math.max(P.kickSecs, 1e-3);
    for (const k of this.kicks) {
      const a0 = k.age / T, a1 = Math.min(1, (k.age + dt) / T), share = (2 * a1 - a1 * a1) - (2 * a0 - a0 * a0);   // (the integral of 2(1 − a) over this step)
      const go = P.kickGo * k.force * (1 - a0) * dt; k.x += k.dx * go; k.z += k.dz * go; k.age += dt;
      const o = U_HEAD + n * 8; f[o] = k.x; f[o + 1] = k.z; f[o + 2] = P.kickSigma + P.kickGrow * k.force * Math.sqrt(a1); f[o + 3] = P.kickColumn * k.force * share;
      f[o + 4] = P.kickHigh + P.kickHighK * k.force; f[o + 5] = k.seed; f[o + 6] = 0; f[o + 7] = 0; n++;
    }
    for (let i = this.kicks.length - 1; i >= 0; i--) if (this.kicks[i].age >= T) this.kicks.splice(i, 1);
    u[24] = n; u[25] = 0; u[26] = 0; u[27] = 0;
    this.device.queue.writeBuffer(this.ubo, 0, this.u, 0, (U_HEAD + n * 8) * 4);
    const pass = enc.beginComputePass({ label: 'powder' }); pass.setBindGroup(0, this.bind); pass.setPipeline(this.pipe);
    pass.dispatchWorkgroups(Math.ceil(this.w / 8), Math.ceil(this.h / 8)); pass.end();
    enc.copyTextureToTexture({ texture: this.nxt }, { texture: this.cur }, [this.w, this.h]);
  }

  /** no powder anywhere, no kick alive (a level restart) */
  reset(): void {
    if (this.dead) return; this.kicks.length = 0;
    this.device.queue.writeTexture({ texture: this.cur }, new Uint16Array(this.w * this.h * 4), { bytesPerRow: this.w * 8 }, [this.w, this.h]);
  }

  destroy(): void { if (this.dead) return; this.dead = true; this.cur.destroy(); this.nxt.destroy(); this.ubo.destroy(); }

  /** the field read back (the test page): Float32Array of w * h * 4, texel (i, j) at (j * w + i) * 4 */
  async readBack(): Promise<Float32Array> {
    const w = this.w, h = this.h, res = new Float32Array(w * h * 4); if (this.dead) return res;
    const row = Math.ceil(w * 8 / 256) * 256;
    const buf = this.device.createBuffer({ label: 'powder read-back', size: row * h, usage: GPUBufferUsage.COPY_DST | GPUBufferUsage.MAP_READ });
    const enc = this.device.createCommandEncoder(); enc.copyTextureToBuffer({ texture: this.cur }, { buffer: buf, bytesPerRow: row }, [w, h]); this.device.queue.submit([enc.finish()]);
    await buf.mapAsync(GPUMapMode.READ);
    const src = new Uint16Array(buf.getMappedRange());
    for (let j = 0; j < h; j++) { const o = j * (row >> 1); for (let i = 0; i < w * 4; i++) res[j * w * 4 + i] = half(src[o + i]); }
    buf.unmap(); buf.destroy(); return res;
  }
}
function half(h: number): number {
  const s = (h & 0x8000) ? -1 : 1, e = (h >> 10) & 31, m = h & 1023;
  if (e === 0) return s * m * 2 ** -24;
  if (e === 31) return m ? NaN : s * Infinity;
  return s * (1 + m / 1024) * 2 ** (e - 15);
}
