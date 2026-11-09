// The world on the GPU: buffers every ray walks, and the shared bind group. What stands is sent once; what moves is sent each frame.
import type { Gpu } from './gpu';
import { buffer } from './gpu';
import { buildBvh, refitBvh, refitFlat, type Bvh } from './bvh';
import { type World, type Piece, PIECE_FLOATS, boundsOf, packPiece, packBend, packCluster, F } from './scene/world';
import { MATERIALS } from './scene/materials';
import { artRect, drawArt, layArt, ART_SIZE } from './scene/art';
import { swayed } from './scene/plants';
import type { Air } from './scene/air';
import type { V3 } from './math';
import { cross, len, norm } from './math';
import { blueNoise } from './noise';
import { LM } from './lampmap';
import { LOOSE } from './stages/loose';
import ltc from './ltc.json';
import { JOIN_ROWS } from './joins';

export class SceneGpu {
  readonly layout: GPUBindGroupLayout; private bg!: GPUBindGroup;
  readonly frame: GPUBuffer; readonly nodes: GPUBuffer; readonly pieces: GPUBuffer; private extra: GPUBuffer; readonly insts: GPUBuffer; readonly lamps: GPUBuffer; private openings: GPUBuffer; private mats: GPUBuffer;
  readonly per: [number, number, number, number]; readonly node0: [number, number, number, number]; readonly piece0: [number, number, number, number]; readonly stepOf: [number, number, number, number]; private moveTree: Bvh | null = null; private moveAge = 0; private instTree: Bvh | null = null; readonly lists: GPUBuffer; readonly leafDraws: { first: number; count: number; leaves: number }[] = []; readonly counts: { pieces: number; moving: number; leaves: number; instances: number };
  private clusterAt: { node: number; per: number; leaf: number }[] = []; private maxMoving: number; private maxInst: number; private nodeData: ArrayBuffer;
  readonly lampOn: Float32Array; /** where the grid of what is joined starts in extra */ readonly joinRow: number; /** the coarse air */ readonly airRow: number; private airSent = -1; sendAir(a: Air) { if (this.airSent === a.version) return; this.airSent = a.version; this.g.device.queue.writeBuffer(this.extra, this.airRow * 16, a.packed); } openFloor = 1e9; writeExtra(row: number, d: Uint8Array) { this.g.device.queue.writeBuffer(this.extra, row * 16, d.buffer as ArrayBuffer, d.byteOffset, d.byteLength); }
  /** where the dice's blue noise starts in extra */ readonly noiseAt: number; readonly lampTex: GPUTexture; readonly smokeTex: GPUTexture; readonly smokePage: GPUTexture; /** the one leaf every loose one is */ readonly looseLeaf: number = 0; readonly standing: Piece[];
  constructor(private g: Gpu, readonly world: World, sky: GPUTexture, caustic: GPUTexture, airRowsN = 0) {
    const d = g.device, S = GPUBufferUsage.STORAGE, U = GPUBufferUsage.UNIFORM;
    // ---- what stands: pieces in the tree's order
    const sets = [world.pieces.filter(p => p.flags & F.CUT), world.pieces.filter(p => !(p.flags & F.CUT))], bs = sets.map(s => buildBvh(s.map(boundsOf), 2)), order = this.standing = sets.flatMap((s, k) => bs[k]!.order.map(i => s[i]!)); this.maxMoving = world.moving.length; this.maxInst = world.instances.length;
    // ---- leaves, planes, marks share one buffer
    if (!world.clusters.some(c => c.leaves.length === 1)) world.clusters.push({ leaves: [{ at: [0, 0, 0], n: [0, 1, 0], t: [1, 0, 0], size: 1, tone: 0.55 }], kind: 0 });
    const packed = world.clusters.map(packCluster); let leafRows = 0; for (const p of packed) leafRows += p.data.byteLength / 16;
    const rows: number[] = [], planeAt = new Map<Piece, number>(), markAt = new Map<Piece, number>(); let row = leafRows;
    layArt([...order, ...world.moving].flatMap(p => p.marks ?? [])); for (const p of [...order, ...world.moving]) { if (p.planes?.length) { planeAt.set(p, row); for (const [n, dd] of p.planes) { rows.push(...n, dd); row++; } } if (p.marks?.length) { markAt.set(p, row); for (const m of p.marks) { const a = artRect(m.art); rows.push(m.at[0], m.at[1], m.size[0], m.size[1], a[0], a[1], a[2], a[3], m.color[0], m.color[1], m.color[2], m.face); row += 3; } } }
    const NZ = 64, na = blueNoise(NZ, 3), nb = blueNoise(NZ, 9); this.noiseAt = leafRows + rows.length / 4; for (let i = 0; i < NZ * NZ; i++) rows.push(na[i]!, nb[i]!); for (let i = 0; i < 256; i++) { const o = i * 6; rows.push(ltc[o]!, ltc[o + 1]!, ltc[o + 2]!, ltc[o + 3]!, ltc[o + 4]!, ltc[o + 5]!, 0, 0); }
    this.joinRow = leafRows + rows.length / 4; this.airRow = this.joinRow + JOIN_ROWS; const ex = new Float32Array((this.airRow + airRowsN) * 4); let at = 0; for (const p of packed) { ex.set(new Float32Array(p.data), at); at += p.data.byteLength / 4; } ex.set(rows, leafRows * 4);
    // bits of packed leaves must survive: copied as words
    { const u = new Uint32Array(ex.buffer); let o = 0; for (const p of packed) { u.set(new Uint32Array(p.data), o); o += p.data.byteLength / 4; } }
    this.extra = buffer(g, 'extra', ex, S);
    this.sigma = world.clusters.map(c => { let a = 0; for (const l of c.leaves) if (!l.wood) a += l.size * l.size * 4 * (c.kind === 1 ? 0.06 : 0.46) * (2 / 3); return Math.max(1, Math.min(63, Math.round(16 * (c.kind === 1 ? 1.3 : 0.5 * a / 4.19 * 0.75)))); });
    // ---- nodes: standing ×8, clusters ×8, moving ×8, instances ×8
    const perMoving = Math.max(1, this.maxMoving * 2), perInst = Math.max(1, this.maxInst * 2); const rest = bs[0]!.perOctant * 8; let n = rest + bs[1]!.perOctant * 8; let leaf = 0;
    for (const p of packed) { this.clusterAt.push({ node: n, per: p.bvh.perOctant, leaf }); n += p.bvh.perOctant * 8; leaf += p.data.byteLength / 32; }
    const movingAt = n; n += perMoving; const instAt = n; n += perInst;
    this.nodeData = new ArrayBuffer(n * 32); { const u = new Uint8Array(this.nodeData); u.set(new Uint8Array(bs[0]!.nodes), 0); u.set(new Uint8Array(bs[1]!.nodes), rest * 32); packed.forEach((p, i) => u.set(new Uint8Array(p.bvh.nodes), this.clusterAt[i]!.node * 32)); }
    this.nodes = buffer(g, 'nodes', new Uint8Array(this.nodeData), S);
    this.pieces = buffer(g, 'pieces', (order.length + Math.max(1, this.maxMoving)) * PIECE_FLOATS * 4, S);
    this.insts = buffer(g, 'insts', (Math.max(1, this.maxInst) + LOOSE) * 64, S); this.lists = buffer(g, 'lists', (order.length + this.maxMoving + (this.maxInst + LOOSE) * 2 + 4) * 4, S); this.listData = new Uint32Array(order.length + this.maxMoving + (this.maxInst + LOOSE) * 2 + 4); this.box = new Float32Array(order.length * 5); order.forEach((p, i) => { const b = boundsOf(p); this.box.set([p.at[0], p.at[1], p.at[2], Math.hypot(...p.half), (b.mx[1] - b.mn[1]) / 2], i * 5); }); this.instData = new Float32Array(this.maxInst * 16);
    { const f = new Float32Array(order.length * PIECE_FLOATS), u = new Uint32Array(f.buffer); order.forEach((p, i) => packPiece(p, f, u, i * PIECE_FLOATS, planeAt.get(p) ?? 0, markAt.get(p) ?? 0)); d.queue.writeBuffer(this.pieces, 0, f); }
    this.planeAt = planeAt; this.markAt = markAt; this.per = [bs[0]!.perOctant, bs[1]!.perOctant, this.maxMoving ? perMoving : 0, this.maxInst ? perInst : 0]; /* (an empty tree is no nodes: a walk of one that was never filled would not end) */ this.stepOf = [bs[0]!.perOctant, bs[1]!.perOctant, 0, 0]; this.node0 = [0, rest, movingAt, instAt]; this.piece0 = [0, sets[0]!.length, order.length, 0];
    this.looseLeaf = leaf - 1; this.moveBox = new Float32Array(this.maxMoving * 6); this.moveF = new Float32Array(this.maxMoving * PIECE_FLOATS); this.moveU = new Uint32Array(this.moveF.buffer);
    this.counts = { pieces: order.length, moving: this.maxMoving, leaves: leaf, instances: this.maxInst };
    // ---- the rest
    this.frame = buffer(g, 'frame', 512, U); this.lamps = buffer(g, 'lamps', 64 * 80, U); this.lampOn = new Float32Array(64).fill(1);
    const op = new Float32Array(32 * 16); world.openings.slice(0, 32).forEach((o, i) => { op.set([...o.at, 4 * len(o.u) * len(o.v), ...o.u, 0, ...o.v, 0, ...norm(o.n), 0], i * 16); }); this.openings = buffer(g, 'openings', op, U);
    const mt = new Float32Array(64 * 12), mu = new Uint32Array(mt.buffer); MATERIALS.forEach((m, i) => { const o = i * 12; mt.set(m.base, o); mu[o + 3] = m.kind; mt[o + 4] = m.rough; mt[o + 5] = m.metal ?? 0; mt[o + 6] = m.f0 ?? 0.04; mt[o + 7] = m.glow ?? 0; mt.set(m.b2 ?? [0, 0, 0], o + 8); mt[o + 11] = m.scale ?? 1; }); this.mats = buffer(g, 'mats', mt, U);
    const art = drawArt(), mips = 8, artTex = d.createTexture({ label: 'art', size: ART_SIZE, format: 'rgba8unorm', mipLevelCount: mips, usage: GPUTextureUsage.TEXTURE_BINDING | GPUTextureUsage.COPY_DST | GPUTextureUsage.RENDER_ATTACHMENT });
    let src: HTMLCanvasElement = art; for (let i = 0; i < mips; i++) { d.queue.copyExternalImageToTexture({ source: src }, { texture: artTex, mipLevel: i, premultipliedAlpha: false }, [src.width, src.height]); const c: HTMLCanvasElement = document.createElement('canvas'); c.width = src.width / 2; c.height = src.height / 2; const x = c.getContext('2d')!; x.imageSmoothingQuality = 'high'; x.drawImage(src, 0, 0, c.width, c.height); src = c; }
    const C = GPUShaderStage.COMPUTE | GPUShaderStage.VERTEX | GPUShaderStage.FRAGMENT, ro = (binding: number): GPUBindGroupLayoutEntry => ({ binding, visibility: C, buffer: { type: 'read-only-storage' } }), un = (binding: number): GPUBindGroupLayoutEntry => ({ binding, visibility: C, buffer: { type: 'uniform' } }), tx = (binding: number): GPUBindGroupLayoutEntry => ({ binding, visibility: C, texture: {} });
    this.layout = d.createBindGroupLayout({ label: 'scene', entries: [un(0), ro(1), ro(2), ro(3), ro(4), un(5), un(6), un(7), { binding: 8, visibility: C, sampler: {} }, tx(9), tx(10), tx(12), { binding: 13, visibility: C, sampler: {} }, { binding: 14, visibility: C, texture: { viewDimension: '3d' } }, { binding: 15, visibility: C, texture: { viewDimension: '3d' } }, { binding: 16, visibility: C, texture: { viewDimension: '3d', sampleType: 'uint' } }] });
    this.smokeTex = d.createTexture({ label: 'smoke', dimension: '3d', size: [160, 160, 120], format: 'rgba16float', usage: GPUTextureUsage.TEXTURE_BINDING | GPUTextureUsage.STORAGE_BINDING }); this.smokePage = d.createTexture({ label: 'smoke page', dimension: '3d', size: [75, 24, 60], format: 'r32uint', usage: GPUTextureUsage.TEXTURE_BINDING | GPUTextureUsage.STORAGE_BINDING });
    this.lampTex = d.createTexture({ label: 'lamp reach', dimension: '3d', size: [LM.cells[0], LM.cells[1], LM.cells[2] * LM.layers], format: 'rgba8unorm', usage: GPUTextureUsage.TEXTURE_BINDING | GPUTextureUsage.STORAGE_BINDING });
    const bf = (binding: number, buffer: GPUBuffer): GPUBindGroupEntry => ({ binding, resource: { buffer } });
    this.bg = d.createBindGroup({ layout: this.layout, entries: [bf(0, this.frame), bf(1, this.nodes), bf(2, this.pieces), bf(3, this.extra), bf(4, this.insts), bf(5, this.lamps), bf(6, this.openings), bf(7, this.mats), { binding: 8, resource: d.createSampler({ magFilter: 'linear', minFilter: 'linear', mipmapFilter: 'linear' }) }, { binding: 9, resource: sky.createView() }, { binding: 10, resource: artTex.createView() }, { binding: 12, resource: caustic.createView() }, { binding: 13, resource: d.createSampler({ magFilter: 'linear', minFilter: 'linear', addressModeU: 'repeat', addressModeV: 'repeat' }) }, { binding: 14, resource: this.lampTex.createView() }, { binding: 15, resource: this.smokeTex.createView() }, { binding: 16, resource: this.smokePage.createView() }] });
  }
  private planeAt: Map<Piece, number>; private markAt: Map<Piece, number>;
  private instData!: Float32Array<ArrayBuffer>;
  /** the leaf clusters' frames at the last frame drawn: three rows each */
  instWas = new Float32Array(0); private keepInst() { for (let i = 0; i < this.maxInst; i++) this.instWas.set(this.instData.subarray(i * 16, i * 16 + 12), i * 12); }
  /** each cluster's density as a medium, in sixteenths */
  private sigma: number[] = [];
  private listData!: Uint32Array<ArrayBuffer>; private box!: Float32Array;
  /** what the raster draws this frame, as ranges of lists: plain boxes, boxes whose true surface is found a pixel at a time, the same two for what is clear; leaf clusters; all of them, for the sun */
  readonly draws: Record<'plain' | 'fancy' | 'clear' | 'clearFancy', [number, number]> = { plain: [0, 0], fancy: [0, 0], clear: [0, 0], clearFancy: [0, 0] }; readonly sunDraws: { first: number; count: number; leaves: number }[] = [];
  /** pick what is in view (and under the section) */
  cull(c: { pos: ArrayLike<number>; right: ArrayLike<number>; up: ArrayLike<number>; fwd: ArrayLike<number> }, cut: number) {
    const { pos, right: r, up: u, fwd: w } = c, tx = Math.hypot(r[0]!, r[1]!, r[2]!), ty = Math.hypot(u[0]!, u[1]!, u[2]!), kx = Math.sqrt(1 + tx * tx), ky = Math.sqrt(1 + ty * ty);
    const seen = (x: number, y: number, z: number, rad: number) => { const a = x - pos[0]!, b = y - pos[1]!, d = z - pos[2]!, f = a * w[0]! + b * w[1]! + d * w[2]!; return f > -rad && f * tx - Math.abs(a * r[0]! + b * r[1]! + d * r[2]!) / tx > -rad * kx && f * ty - Math.abs(a * u[0]! + b * u[1]! + d * u[2]!) / ty > -rad * ky; };
    const L: Record<string, number[]> = { plain: [], fancy: [], clear: [], clearFancy: [] }, put = (p: Piece, i: number, x: number, y: number, z: number, rad: number, hy: number) => { const cuts = cut < 1e8 && !!(p.flags & F.CUT); if ((cuts && y - hy > cut) || (p.flags & F.OPEN && cut < this.openFloor)) return; if (!seen(x, y, z, rad)) return; const fancy = !!p.quad || !!p.planes?.length || !!(p.flags & F.WATER) || (cuts && y + hy > cut); L[p.flags & (F.GLASS | F.WATER) ? (fancy ? 'clearFancy' : 'clear') : fancy ? 'fancy' : 'plain']!.push(i); };
    const B = this.box; this.standing.forEach((p, i) => { const o = i * 5; put(p, i, B[o]!, B[o + 1]!, B[o + 2]!, B[o + 3]!, B[o + 4]!); });
    const m = this.world.moving; if (this.moveTree) this.moveTree.order.forEach((src, i) => { const p = m[src]!; if (p.at[1] < -50) return; const hy = Math.abs(p.rot[0][1]) * p.half[0] + Math.abs(p.rot[1][1]) * p.half[1] + Math.abs(p.rot[2][1]) * p.half[2]; put(p, this.piece0[2] + i, p.at[0], p.at[1], p.at[2], Math.hypot(...p.half), hy); });
    const D = this.listData; let o = 0; for (const k of ['plain', 'fancy', 'clear', 'clearFancy'] as const) { this.draws[k] = [o, L[k]!.length]; D.set(L[k]!, o); o += L[k]!.length; }
    this.leafDraws.length = this.sunDraws.length = 0; const wld = this.world, order = this.instTree?.order ?? [];
    for (const leaves of new Set(wld.clusters.map(c => c.leaves.length))) { if (leaves === 1) { for (let i = 0; i < LOOSE; i++) D[o + i] = this.maxInst + i; this.leafDraws.push({ first: o, count: LOOSE, leaves }); this.sunDraws.push({ first: o, count: LOOSE, leaves }); o += LOOSE; continue; } const all: number[] = [], near: number[] = []; order.forEach((src, i) => { const n = wld.instances[src]!, c = wld.clusters[n.cluster]!; if (c.leaves.length !== leaves) return; all.push(i); const a = this.posed[src]!.at, e = n.scale * (c.kind ? 2.1 : 1.22); if (((n.open && cut > this.openFloor) || a[1] - e < cut) && seen(a[0], a[1], a[2], e)) near.push(i); });
      this.leafDraws.push({ first: o, count: near.length, leaves }); D.set(near, o); o += near.length; this.sunDraws.push({ first: o, count: all.length, leaves }); D.set(all, o); o += all.length; }
    this.g.device.queue.writeBuffer(this.lists, 0, D, 0, o);
  }
  private posed: { at: V3; rot: Piece['rot'] }[] = [];
  group = () => this.bg;
  /** what moves this frame: pieces and leaf clusters, with trees over them. `slots` maps each piece to where it lies, for motion */
  moved(t: number, air: Air): Map<Piece, number> {
    const d = this.g.device, w = this.world;
    if (w.moving.length) {
      const n = w.moving.length, mb = this.moveBox; for (let i = 0, o = 0; i < n; i++, o += 6) { const p = w.moving[i]!, h = p.half, r = p.rot, x = r[0], y = r[1], z = r[2]; for (let k = 0; k < 3; k++) { const e = Math.abs(x[k]!) * h[0] + Math.abs(y[k]!) * h[1] + Math.abs(z[k]!) * h[2] + 1e-4; mb[o + k] = p.at[k]! - e; mb[o + 3 + k] = p.at[k]! + e; } }
      // the tree keeps its shape while it is still a good one: its bounds are made anew every frame
      let fresh = false; if (this.moveTree) { const a = refitFlat(this.moveTree, mb); if (++this.moveAge > 30 && a > this.moveArea * 1.7) fresh = true; } else fresh = true;
      if (fresh) { this.moveTree = buildBvh(Array.from({ length: n }, (_, i) => ({ mn: [mb[i * 6]!, mb[i * 6 + 1]!, mb[i * 6 + 2]!], mx: [mb[i * 6 + 3]!, mb[i * 6 + 4]!, mb[i * 6 + 5]!] })), 2, 1); this.moveAge = 0; this.moveArea = refitFlat(this.moveTree, mb); this.moveSlots = new Map(); this.moveTree.order.forEach((src, i) => { const p = w.moving[src]!; packPiece(p, this.moveF, this.moveU, i * PIECE_FLOATS, this.planeAt.get(p) ?? 0, this.markAt.get(p) ?? 0); this.moveSlots.set(p, this.piece0[2] + i); }); }
      const t = this.moveTree!, f = this.moveF; for (let i = 0; i < n; i++) { const p = w.moving[t.order[i]!]!, o = i * PIECE_FLOATS, c = p.at, r = p.rot; for (let k = 0; k < 3; k++) { const q = r[k]!, m = o + k * 4; f[m] = q[0]; f[m + 1] = q[1]; f[m + 2] = q[2]; f[m + 3] = -(q[0] * c[0] + q[1] * c[1] + q[2] * c[2]); } if (p.bend) packBend(p.bend, this.moveU, o); }
      d.queue.writeBuffer(this.pieces, this.piece0[2] * PIECE_FLOATS * 4, f); this.per[2] = t.perOctant; d.queue.writeBuffer(this.nodes, this.node0[2] * 32, t.nodes);
    }
    if (w.instances.length) {
      const posed = this.posed = w.instances.map(i => swayed(i, t, air)), bd = (k: number) => { const i = w.instances[k]!, tuft = w.clusters[i.cluster]!.kind === 1, e = i.scale * (tuft ? 2.1 : 1.22), a = posed[k]!.at; return { mn: [a[0] - e, a[1] - (tuft ? 0.1 : e), a[2] - e], mx: [a[0] + e, a[1] + e, a[2] + e] } as never; };
      if (!this.instTree) this.instTree = buildBvh(w.instances.map((_, k) => bd(k)), 1, 1); else refitBvh(this.instTree, bd); const b = this.instTree;
      const f = this.instData, u = new Uint32Array(f.buffer), first = this.instWas.length === 0; if (!first) this.keepInst(); const q = (x: number) => Math.max(0, Math.min(255, Math.round(x * 255)));
      b.order.forEach((src, i) => { const n = w.instances[src]!, p = posed[src]!, o = i * 16, k = 1 / n.scale, c = this.clusterAt[n.cluster]!; for (let a = 0; a < 3; a++) { const r = p.rot[a]!; f[o + a * 4] = r[0] * k; f[o + a * 4 + 1] = r[1] * k; f[o + a * 4 + 2] = r[2] * k; f[o + a * 4 + 3] = -(r[0] * p.at[0] + r[1] * p.at[1] + r[2] * p.at[2]) * k; } u[o + 12] = c.node; u[o + 13] = c.per; u[o + 14] = c.leaf; u[o + 15] = (q(n.tint[0]) | (q(n.tint[1]) << 8) | (q(n.tint[2]) << 16) | (n.open ? 1 << 24 : 0) | (w.clusters[n.cluster]!.kind === 1 ? 1 << 25 : 0) | (this.sigma[n.cluster]! << 26)) >>> 0; });
      if (first) { this.instWas = new Float32Array(this.maxInst * 12); this.keepInst(); } d.queue.writeBuffer(this.insts, 0, f); this.per[3] = b.perOctant; d.queue.writeBuffer(this.nodes, this.node0[3] * 32, b.nodes);
    }
    return this.moveSlots;
  }
  private moveBox!: Float32Array; private moveF!: Float32Array<ArrayBuffer>; private moveU!: Uint32Array; private moveArea = 0; private moveSlots = new Map<Piece, number>();
  sendLamps() {
    const f = new Float32Array(64 * 20), u = new Uint32Array(f.buffer);
    this.world.lamps.slice(0, 64).forEach((l, i) => { const o = i * 20, n = l.n ? l.n : l.kind === 'spot' ? norm(l.v) : norm(cross(l.u, l.v)), area = l.kind === 'rect' ? 4 * len(l.u) * len(l.v) : len(l.u); const on = this.lampOn[i]!, bright = Math.max(...l.radiance) * on * (l.kind === 'rect' ? area : Math.PI * area * area);
      f.set(l.at, o); u[o + 3] = l.kind === 'rect' ? 0 : l.kind === 'sphere' ? 1 : 2; f.set(l.u, o + 4); f[o + 7] = l.cos ?? -1; f.set(l.v, o + 8); f[o + 11] = area; f.set(n, o + 12); f[o + 15] = on; f.set(l.radiance, o + 16); f[o + 19] = Math.min(40, Math.sqrt(bright / 0.004)); });
    this.g.device.queue.writeBuffer(this.lamps, 0, f); void F; void ([] as V3[]);
  }
}
