// The world on the GPU: buffers every ray walks, and the shared bind group. What stands is sent once; what moves is sent each frame.
import type { Gpu } from './gpu';
import { buffer } from './gpu';
import { buildBvh } from './bvh';
import { type World, type Piece, PIECE_FLOATS, boundsOf, packPiece, packCluster, F } from './scene/world';
import { MATERIALS } from './scene/materials';
import { artRect, drawArt } from './scene/art';
import { swayed } from './scene/plants';
import type { V3 } from './math';
import { cross, len, norm } from './math';
import { blueNoise } from './noise';

export class SceneGpu {
  readonly layout: GPUBindGroupLayout; private bg!: GPUBindGroup;
  readonly frame: GPUBuffer; private nodes: GPUBuffer; readonly pieces: GPUBuffer; private extra: GPUBuffer; private insts: GPUBuffer; readonly lamps: GPUBuffer; private openings: GPUBuffer; private mats: GPUBuffer;
  readonly tree: [number, number, number, number]; readonly base: [number, number]; readonly counts: { pieces: number; moving: number; leaves: number; instances: number };
  private clusterAt: { node: number; per: number; leaf: number }[] = []; private maxMoving: number; private maxInst: number; private nodeData: ArrayBuffer;
  readonly lampOn: Float32Array; readonly standing: Piece[];
  constructor(private g: Gpu, readonly world: World, sky: GPUTexture) {
    const d = g.device, S = GPUBufferUsage.STORAGE, U = GPUBufferUsage.UNIFORM;
    // ---- what stands: pieces in the tree's order
    const bs = buildBvh(world.pieces.map(boundsOf), 2), order = this.standing = bs.order.map(i => world.pieces[i]!); this.maxMoving = world.moving.length; this.maxInst = world.instances.length;
    // ---- leaves, planes, marks share one buffer
    const packed = world.clusters.map(packCluster); let leafRows = 0; for (const p of packed) leafRows += p.data.byteLength / 16;
    const rows: number[] = [], planeAt = new Map<Piece, number>(), markAt = new Map<Piece, number>(); let row = leafRows;
    for (const p of [...order, ...world.moving]) { if (p.planes?.length) { planeAt.set(p, row); for (const [n, dd] of p.planes) { rows.push(...n, dd); row++; } } if (p.marks?.length) { markAt.set(p, row); for (const m of p.marks) { const a = artRect(m.art); rows.push(m.at[0], m.at[1], m.size[0], m.size[1], 0, a[0], a[1], 0, 0, 0, 0, 0); const o = rows.length - 8; rows[o] = m.turn ?? 0; rows[o + 1] = a[0]; rows[o + 2] = a[1]; rows[o + 3] = a[2]; rows[o + 4] = m.color[0]; rows[o + 5] = m.color[1]; rows[o + 6] = m.color[2]; rows[o + 7] = m.face; row += 3; } } }
    const ex = new Float32Array((leafRows + rows.length / 4) * 4); let at = 0; for (const p of packed) { ex.set(new Float32Array(p.data), at); at += p.data.byteLength / 4; } ex.set(rows, leafRows * 4);
    // bits of packed leaves must survive: copied as words
    { const u = new Uint32Array(ex.buffer); let o = 0; for (const p of packed) { u.set(new Uint32Array(p.data), o); o += p.data.byteLength / 4; } }
    this.extra = buffer(g, 'extra', ex, S);
    // ---- nodes: standing ×8, clusters ×8, moving ×8, instances ×8
    const perMoving = Math.max(1, this.maxMoving * 2), perInst = Math.max(1, this.maxInst * 2); let n = bs.perOctant * 8; let leaf = 0;
    for (const p of packed) { this.clusterAt.push({ node: n, per: p.bvh.perOctant, leaf }); n += p.bvh.perOctant * 8; leaf += p.data.byteLength / 32; }
    const movingAt = n; n += perMoving * 8; const instAt = n; n += perInst * 8;
    this.nodeData = new ArrayBuffer(n * 32); { const u = new Uint8Array(this.nodeData); u.set(new Uint8Array(bs.nodes), 0); packed.forEach((p, i) => u.set(new Uint8Array(p.bvh.nodes), this.clusterAt[i]!.node * 32)); }
    this.nodes = buffer(g, 'nodes', new Uint8Array(this.nodeData), S);
    this.pieces = buffer(g, 'pieces', (order.length + Math.max(1, this.maxMoving)) * PIECE_FLOATS * 4, S);
    this.insts = buffer(g, 'insts', Math.max(1, this.maxInst) * 64, S);
    { const f = new Float32Array(order.length * PIECE_FLOATS), u = new Uint32Array(f.buffer); order.forEach((p, i) => packPiece(p, f, u, i * PIECE_FLOATS, planeAt.get(p) ?? 0, markAt.get(p) ?? 0)); d.queue.writeBuffer(this.pieces, 0, f); }
    this.planeAt = planeAt; this.markAt = markAt; this.tree = [bs.perOctant, perMoving, perInst, order.length]; this.base = [movingAt, instAt];
    this.counts = { pieces: order.length, moving: this.maxMoving, leaves: leaf, instances: this.maxInst };
    // ---- the rest
    this.frame = buffer(g, 'frame', 512, U); this.lamps = buffer(g, 'lamps', 64 * 80, U); this.lampOn = new Float32Array(64).fill(1);
    const op = new Float32Array(32 * 16); world.openings.slice(0, 32).forEach((o, i) => { op.set([...o.at, 4 * len(o.u) * len(o.v), ...o.u, 0, ...o.v, 0, ...norm(o.n), 0], i * 16); }); this.openings = buffer(g, 'openings', op, U);
    const mt = new Float32Array(64 * 12), mu = new Uint32Array(mt.buffer); MATERIALS.forEach((m, i) => { const o = i * 12; mt.set(m.base, o); mu[o + 3] = m.kind; mt[o + 4] = m.rough; mt[o + 5] = m.metal ?? 0; mt[o + 6] = m.f0 ?? 0.04; mt[o + 7] = m.glow ?? 0; mt.set(m.b2 ?? [0, 0, 0], o + 8); mt[o + 11] = m.scale ?? 1; }); this.mats = buffer(g, 'mats', mt, U);
    const art = drawArt(), mips = 8, artTex = d.createTexture({ label: 'art', size: [2048, 2048], format: 'rgba8unorm', mipLevelCount: mips, usage: GPUTextureUsage.TEXTURE_BINDING | GPUTextureUsage.COPY_DST | GPUTextureUsage.RENDER_ATTACHMENT });
    let src: HTMLCanvasElement = art; for (let i = 0; i < mips; i++) { d.queue.copyExternalImageToTexture({ source: src }, { texture: artTex, mipLevel: i, premultipliedAlpha: false }, [src.width, src.height]); const c: HTMLCanvasElement = document.createElement('canvas'); c.width = src.width / 2; c.height = src.height / 2; const x = c.getContext('2d')!; x.imageSmoothingQuality = 'high'; x.drawImage(src, 0, 0, c.width, c.height); src = c; }
    const N = 64, a = blueNoise(N, 3), b = blueNoise(N, 9), nz = new Uint16Array(N * N * 2); const half = (v: number) => { const f = new Float32Array([v]), u = new Uint32Array(f.buffer)[0]!, e = ((u >> 23) & 255) - 112; return e <= 0 ? 0 : (e << 10) | ((u >> 13) & 1023); }; for (let i = 0; i < N * N; i++) { nz[i * 2] = half(a[i]!); nz[i * 2 + 1] = half(b[i]!); }
    const noise = d.createTexture({ label: 'noise', size: [N, N], format: 'rg16float', usage: GPUTextureUsage.TEXTURE_BINDING | GPUTextureUsage.COPY_DST }); d.queue.writeTexture({ texture: noise }, nz, { bytesPerRow: N * 4 }, [N, N]);
    const C = GPUShaderStage.COMPUTE, ro = (binding: number): GPUBindGroupLayoutEntry => ({ binding, visibility: C, buffer: { type: 'read-only-storage' } }), un = (binding: number): GPUBindGroupLayoutEntry => ({ binding, visibility: C, buffer: { type: 'uniform' } }), tx = (binding: number): GPUBindGroupLayoutEntry => ({ binding, visibility: C, texture: {} });
    this.layout = d.createBindGroupLayout({ label: 'scene', entries: [un(0), ro(1), ro(2), ro(3), ro(4), un(5), un(6), un(7), { binding: 8, visibility: C, sampler: {} }, tx(9), tx(10), tx(11)] });
    const bf = (binding: number, buffer: GPUBuffer): GPUBindGroupEntry => ({ binding, resource: { buffer } });
    this.bg = d.createBindGroup({ layout: this.layout, entries: [bf(0, this.frame), bf(1, this.nodes), bf(2, this.pieces), bf(3, this.extra), bf(4, this.insts), bf(5, this.lamps), bf(6, this.openings), bf(7, this.mats), { binding: 8, resource: d.createSampler({ magFilter: 'linear', minFilter: 'linear', mipmapFilter: 'linear' }) }, { binding: 9, resource: sky.createView() }, { binding: 10, resource: artTex.createView() }, { binding: 11, resource: noise.createView() }] });
  }
  private planeAt: Map<Piece, number>; private markAt: Map<Piece, number>;
  group = () => this.bg;
  /** what moves this frame: pieces and leaf clusters, with trees over them. `slots` maps each piece to where it lies, for motion */
  moved(t: number, wind: V3): Map<Piece, number> {
    const d = this.g.device, w = this.world, slots = new Map<Piece, number>();
    if (w.moving.length) {
      const b = buildBvh(w.moving.map(boundsOf), 2), f = new Float32Array(w.moving.length * PIECE_FLOATS), u = new Uint32Array(f.buffer);
      b.order.forEach((src, i) => { const p = w.moving[src]!; packPiece(p, f, u, i * PIECE_FLOATS, this.planeAt.get(p) ?? 0, this.markAt.get(p) ?? 0); slots.set(p, this.tree[3] + i); }); d.queue.writeBuffer(this.pieces, this.tree[3] * PIECE_FLOATS * 4, f);
      this.tree[1] = b.perOctant; d.queue.writeBuffer(this.nodes, this.base[0] * 32, b.nodes);
    }
    if (w.instances.length) {
      const posed = w.instances.map(i => swayed(i, t, wind)), b = buildBvh(w.instances.map((i, k) => { const e = i.scale * (w.clusters[i.cluster]!.kind === 1 ? 2.1 : 1.22), a = posed[k]!.at; return { mn: [a[0] - e, a[1] - e, a[2] - e], mx: [a[0] + e, a[1] + e, a[2] + e] } as never; }), 1);
      const f = new Float32Array(w.instances.length * 16), u = new Uint32Array(f.buffer), q = (x: number) => Math.max(0, Math.min(255, Math.round(x * 255)));
      b.order.forEach((src, i) => { const n = w.instances[src]!, p = posed[src]!, o = i * 16, k = 1 / n.scale, c = this.clusterAt[n.cluster]!; for (let a = 0; a < 3; a++) { const r = p.rot[a]!; f[o + a * 4] = r[0] * k; f[o + a * 4 + 1] = r[1] * k; f[o + a * 4 + 2] = r[2] * k; f[o + a * 4 + 3] = -(r[0] * p.at[0] + r[1] * p.at[1] + r[2] * p.at[2]) * k; } u[o + 12] = c.node; u[o + 13] = c.per; u[o + 14] = c.leaf; u[o + 15] = (q(n.tint[0]) | (q(n.tint[1]) << 8) | (q(n.tint[2]) << 16)) >>> 0; });
      d.queue.writeBuffer(this.insts, 0, f); this.tree[2] = b.perOctant; d.queue.writeBuffer(this.nodes, this.base[1] * 32, b.nodes);
    }
    return slots;
  }
  sendLamps() {
    const f = new Float32Array(64 * 20), u = new Uint32Array(f.buffer);
    this.world.lamps.slice(0, 64).forEach((l, i) => { const o = i * 20, n = l.n ? l.n : l.kind === 'spot' ? norm(l.v) : norm(cross(l.u, l.v)), area = l.kind === 'rect' ? 4 * len(l.u) * len(l.v) : len(l.u); const on = this.lampOn[i]!, bright = Math.max(...l.radiance) * on * (l.kind === 'rect' ? area : Math.PI * area * area);
      f.set(l.at, o); u[o + 3] = l.kind === 'rect' ? 0 : l.kind === 'sphere' ? 1 : 2; f.set(l.u, o + 4); f[o + 7] = l.cos ?? -1; f.set(l.v, o + 8); f[o + 11] = area; f.set(n, o + 12); f[o + 15] = on; f.set(l.radiance, o + 16); f[o + 19] = Math.min(40, Math.sqrt(bright / 0.004)); });
    this.g.device.queue.writeBuffer(this.lamps, 0, f); void F; void ([] as V3[]);
  }
}
