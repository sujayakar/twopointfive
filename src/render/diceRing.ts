// THE LOOK PROTOTYPE'S STEADY BOUNCE — the radiance cache's light, held still by day (the look page's row `steady`; shaders/dice_ring.wgsl).
// The maintainer on his GPU, 11-01, the pass at midday with doors open: "lots of temporal instability for midday, also strange shimmering on metallic
// barrels", "lots of temporal instability on the guy's back too", "overall night, dawn, and dusk look pretty good. midday and overcast are the broken ones".
// THE FAULT. The cache (render/rc.ts) looks along a fixed set of directions, shifted a little every frame so that over time it sees between them, and what
// everything reads (the dice: the gather's second bounce and its outlier bound, a wet coat's sheen, the smoke, the light meter, the look page's exposure a
// place) is a running mean of about three frames of it. At night a room's light is its lamps, broad and near, and three frames agree. By day the lamps are
// drawn at the hour's share and a room is lit by what a door or a window lets in: small, far, thousands of times brighter than the room, caught by one
// frame's directions and missed by the next. Measured on the software adapter in the pass's store with its door open at midday: the light the cache holds at
// Sam went up and down by a fifth from frame to frame.
// THE RULE. The shift is one of a FEW points, taken in turn, and the dice is the plain mean of the cache's last bake at each of them. Nothing in a still
// scene changes from one frame to the next then: each frame replaces one bake with the same bake. A real change of light comes in evenly over as many
// frames as there are points. The points are more the further the hour's lamps are under its daylight (ringPoints): none at night (the cache as it was),
// four at dawn and dusk, eight on a grey day and at midday.
// HOW. The cache bakes whole into its dice (its own running mean off: rc.setDiceBlend(1)); step() copies that bake into its slot of a texture that holds
// one bake a point and writes the mean of the slots held back into the dice. Everything downstream binds the dice as before.
// PICTURE AND METER: the light meter and the smoke read the dice, so on the look page the row changes what they read (steadier, a few frames later).
// It takes no die and no wall clock and imports nothing from src/game. NEVER SEEN ON A GPU.
import { makeBuffer, makeShader, computePipelineAsync } from '../gpu/device';
import ringSrc from './shaders/dice_ring.wgsl' with { type: 'text' };

/** the most points a ring holds (its texture is this many dice tall) */
export const RING_MAX = 8;
/** how many jitter points the cache's mean is taken over at an hour whose lamps are drawn at `lampShare` of their night strength (1 = a night, or nobody
 *  says), with the row at `row`: by 2 x row x log10(1 / share), under 2 = no ring (the cache's own mean), under 6 = four points, else eight. Four or
 *  eight and nothing between: the cache refreshes its far levels over 1, 2 or 4 frames on some tiers (settings.rcPeriods), and a round of the points
 *  must be a whole number of those for a still scene to repeat. Pure. */
export function ringPoints(lampShare: number, row: number): number {
  if (!(lampShare < 1) || !(row > 0)) return 0;
  const q = 2 * row * Math.log10(1 / Math.max(lampShare, 1e-6)); return q < 2 ? 0 : q < 6 ? 4 : RING_MAX;
}
/** the point frame `jf` of the cache's clock takes, of `n` */
export function ringSlot(jf: number, n: number): number { return n > 0 ? ((Math.floor(jf) % n) + n) % n : 0; }
/** the slots held after `slot` is written: all dropped first when the cache starts over or the count of points changed (`fresh`). Pure. */
export function ringMask(mask: number, slot: number, n: number, fresh: boolean): number { return ((fresh ? 0 : mask) | (1 << slot)) & ((1 << n) - 1); }

export class DiceRing {
  ready = false;
  readonly whenReady: Promise<void>;
  /** means written since it was built, and the slots held now (a tool's) */
  stepped = 0; mask = 0; points = 0;
  private readonly device: GPUDevice; private readonly bgl: GPUBindGroupLayout; private readonly ubo: GPUBuffer;
  private pipe: GPUComputePipeline | null = null; private ring: GPUTexture | null = null; private bind: GPUBindGroup | null = null; private of: GPUTexture | null = null;
  private readonly u = new Uint32Array(4);

  constructor(device: GPUDevice) {
    this.device = device; const C = GPUShaderStage.COMPUTE;
    this.bgl = device.createBindGroupLayout({ label: 'dice ring', entries: [
      { binding: 0, visibility: C, buffer: { type: 'uniform' } }, { binding: 1, visibility: C, texture: { sampleType: 'float', viewDimension: '3d' } },
      { binding: 2, visibility: C, storageTexture: { access: 'write-only', format: 'rgba16float', viewDimension: '3d' } }] });
    this.ubo = makeBuffer(device, 16, GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST, 'dice ring uniforms');
    const layout = device.createPipelineLayout({ label: 'dice ring', bindGroupLayouts: [this.bgl] }), module = makeShader(device, ringSrc, 'dice_ring');
    this.whenReady = computePipelineAsync(device, { label: 'dice ring mean', layout, compute: { module, entryPoint: 'ringMean' } }).then(p => {
      // (a compile the device refused comes back as an INVALID pipeline, and setting one on the frame's encoder would void the frame: tried once on an
      // encoder of its own under an error scope, as the place field's are, render/placeField.ts)
      const scoped = typeof device.pushErrorScope === 'function' && typeof device.popErrorScope === 'function' && typeof GPUValidationError !== 'undefined';
      if (!scoped) { this.pipe = p; this.ready = true; return; }
      device.pushErrorScope('validation');
      try { const enc = device.createCommandEncoder({ label: 'dice ring pipeline tried' }), pass = enc.beginComputePass(); pass.setPipeline(p); pass.end(); device.queue.submit([enc.finish()]); } catch { /* the scope says */ }
      return Promise.resolve(device.popErrorScope()).then(err => {
        if (err instanceof GPUValidationError) console.warn('dice ring: not built on this device', err.message); else { this.pipe = p; this.ready = true; }
      }, () => { /* a lost device: its own handler speaks */ });
    });
  }

  /** the ring for this cache's dice: another cache (another preset, a level's own grid) has another size, and the ring starts over for it */
  private bindTo(dice: GPUTexture): void {
    if (this.of === dice) return; this.of = dice; this.ring?.destroy(); this.mask = 0;
    this.ring = this.device.createTexture({ label: 'dice ring', size: [dice.width, dice.height * RING_MAX, dice.depthOrArrayLayers], dimension: '3d', format: 'rgba16float', usage: GPUTextureUsage.TEXTURE_BINDING | GPUTextureUsage.COPY_DST });
    this.bind = this.device.createBindGroup({ label: 'dice ring', layout: this.bgl, entries: [{ binding: 0, resource: { buffer: this.ubo } }, { binding: 1, resource: this.ring.createView() }, { binding: 2, resource: dice.createView() }] });
  }

  /** a ring of this cache's dice fits one texture (RING_MAX of them one under another): asked before the cache's own mean is put off for it */
  fits(dice: GPUTexture): boolean { return dice.height * RING_MAX <= (this.device.limits?.maxTextureDimension3D ?? 2048); }
  /** After the cache baked this frame WHOLE into `dice` (its own mean off): keep that bake as point `slot` of `n` and write the mean of the points held
   *  back into `dice`. `fresh` = the cache started over this frame (rc.reprimed, read before its encode): what was held is dropped. False = not ready. */
  step(enc: GPUCommandEncoder, dice: GPUTexture, slot: number, n: number, fresh: boolean): boolean {
    if (!this.ready || !this.pipe || n < 2 || n > RING_MAX || !this.fits(dice)) return false;
    this.bindTo(dice);
    this.mask = ringMask(this.mask, slot, n, fresh || n !== this.points); this.points = n;
    const w = dice.width, h = dice.height, d = dice.depthOrArrayLayers;
    enc.copyTextureToTexture({ texture: dice }, { texture: this.ring!, origin: [0, slot * h, 0] }, [w, h, d]);
    this.u[0] = n; this.u[1] = this.mask; this.u[2] = h; this.u[3] = 0; this.device.queue.writeBuffer(this.ubo, 0, this.u);
    const pass = enc.beginComputePass({ label: 'dice ring mean' }); pass.setPipeline(this.pipe); pass.setBindGroup(0, this.bind!);
    pass.dispatchWorkgroups(Math.ceil(w / 4), Math.ceil(h / 4), Math.ceil(d / 4)); pass.end(); this.stepped++;
    return true;
  }
  /** nothing is held (the row went down, the hour became a night): the next step starts a ring afresh */
  drop(): void { this.mask = 0; this.points = 0; }
  destroy(): void { this.ring?.destroy(); this.ring = null; this.bind = null; this.of = null; this.ubo.destroy(); }
}
