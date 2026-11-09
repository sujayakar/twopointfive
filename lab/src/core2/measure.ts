// Rays through every cell of the block, from all six sides, see what stands in it: how much they meet, what color it is, how tightly it can be boxed, how much of it is leaves.
import type { Lab } from '../lab';
import { buffer, readBuffer } from '../gpu';
import { cellRow, FINEST } from './blocks';
import { secondCore } from './core';

const code = (grid: number) => `
@group(1) @binding(0) var<storage, read> places: array<vec4f>; @group(1) @binding(1) var<storage, read_write> made: array<u32>; @group(1) @binding(2) var<uniform> most: vec4u;
const GRID = ${grid}u; const RAYS = ${grid * grid}u;
var<workgroup> met: array<atomic<u32>, 6>; var<workgroup> red: array<atomic<u32>, 6>; var<workgroup> green: array<atomic<u32>, 6>; var<workgroup> blue: array<atomic<u32>, 6>; var<workgroup> leafy: atomic<u32>; var<workgroup> grass: atomic<u32>; var<workgroup> low: array<atomic<u32>, 3>; var<workgroup> high: array<atomic<u32>, 3>;
fn sends(s: Surf) -> vec3f { let f = coatF0(s); let r = s.rough; let a = f * (0.955 / (1.0 + 1.346 * pow(r, 3.67))) + vec3f(0.0476 * exp(-3.55 * r * r)); let lf = lum(f); return min(matteOf(s) * matteMean(s.kind, s.albedo) * (1.0 - lf - (1.0 - lf) / 21.0) + a, vec3f(1.0)); }
/** is there nothing that stands for good between two places? */
fn together(a: vec3f, b: vec3f) -> bool { let d = b - a; let l = length(d); if (l < 0.03) { return true; } lim = vec4f(0.0, 0.0, 0.0, 1e9); leafMode = 2u; let h = nearest(a, d / l, 0.002, l - 0.002, GLASS | WATER, 1e9); return h.piece == NOTHING; }
var<workgroup> firstHit: atomic<u32>; var<workgroup> refAt: vec3f; var<workgroup> joinedTo: atomic<u32>; var<workgroup> total: atomic<u32>;
@compute @workgroup_size(64) fn main(@builtin(workgroup_id) wg: vec3u, @builtin(local_invocation_index) li: u32) {
  let cell = min(wg.x + wg.y * 32768u, most.x - 1u); let at = places[cell]; let level = u32(at.w); let h = FINEST * f32(1u << level);
  if (li < 6u) { atomicStore(&met[li], 0u); atomicStore(&red[li], 0u); atomicStore(&green[li], 0u); atomicStore(&blue[li], 0u); } if (li < 3u) { atomicStore(&low[li], 255u); atomicStore(&high[li], 0u); } if (li == 6u) { atomicStore(&leafy, 0u); atomicStore(&firstHit, 999u); } if (li == 7u) { atomicStore(&grass, 0u); atomicStore(&joinedTo, 0u); atomicStore(&total, 0u); } workgroupBarrier();
  var hit: array<vec3f, 6>; var color: array<vec3f, 6>; var kind: array<u32, 6>; var got = false;          /* kind: 0 nothing, 1 solid, 2 leaf, 3 grass */
  rngKey(cell * 64u + li, 77u);
  for (var k = 0u; k < 6u; k++) {
    kind[k] = 0u; if (li >= RAYS) { continue; } leafMode = 0u; blkOn = false; blk = vec4f(1e9); lim = vec4f(1e9, select(0.0, 1e9, level > 0u), 0.0, 0.0);   /* (only what this level holds) */
    let a = k >> 1u; let b = (a + 1u) % 3u; let c = (a + 2u) % 3u; let j = (vec2f(f32(li % GRID), f32(li / GRID)) + rnd2()) / f32(GRID); var o = at.xyz; o[b] += j.x * h; o[c] += j.y * h; var d = vec3f(0.0); if ((k & 1u) == 0u) { o[a] += h; d[a] = -1.0; } else { d[a] = 1.0; }      /* (even: seen from the + side) */
    let q = nearest(o, d, 0.0, h, GLASS | WATER, 1e9); if (q.piece == NOTHING) { continue; }
    let leaf = (q.piece & LEAF_HIT) != 0u; let s = surfaceAt(o, d, q, 1.0); color[k] = select(sends(s), s.albedo, leaf); kind[k] = 1u; if (leaf) { kind[k] = select(2u, 3u, s.kind == 20u || ((insts[q.piece & 0x7fffffffu].tint >> 25u) & 1u) != 0u); } hit[k] = o + d * (q.t - 0.004); got = true; atomicAdd(&total, 1u);
  }
  if (got) { atomicMin(&firstHit, li); } workgroupBarrier(); if (li == atomicLoad(&firstHit)) { for (var k = 0u; k < 6u; k++) { if (kind[k] > 0u) { refAt = hit[k]; break; } } } workgroupBarrier();
  // a wall may part what is in it: then it holds the side that has more of it
  var same: array<bool, 6>; for (var k = 0u; k < 6u; k++) { same[k] = false; if (kind[k] > 0u) { same[k] = together(refAt, hit[k]); if (same[k]) { atomicAdd(&joinedTo, 1u); } } } workgroupBarrier(); let want = atomicLoad(&joinedTo) * 2u >= atomicLoad(&total);
  for (var k = 0u; k < 6u; k++) {
    if (kind[k] == 0u || same[k] != want) { continue; } let v = vec3u(clamp(color[k], vec3f(0.0), vec3f(1.0)) * 4095.0); atomicAdd(&met[k], 1u); atomicAdd(&red[k], v.x); atomicAdd(&green[k], v.y); atomicAdd(&blue[k], v.z); if (kind[k] > 1u) { atomicAdd(&leafy, 1u); } if (kind[k] > 2u) { atomicAdd(&grass, 1u); }
    let x = clamp((hit[k] - at.xyz) / h, vec3f(0.0), vec3f(0.9999)); for (var m = 0u; m < 3u; m++) { let z = u32(x[m] * 256.0); atomicMin(&low[m], z); atomicMax(&high[m], z); }
  }
  workgroupBarrier(); if (li != 0u || wg.x + wg.y * 32768u >= most.x) { return; }
  var n: array<f32, 6>; var all = 0.0; for (var k = 0u; k < 6u; k++) { n[k] = f32(atomicLoad(&met[k])); all += n[k]; } var out: array<u32, 8>; if (all < 0.5) { for (var k = 0u; k < 8u; k++) { made[cell * 8u + k] = 0u; } return; }
  var lo = vec3u(0u); var hi = vec3u(0u); for (var m = 0u; m < 3u; m++) { lo[m] = atomicLoad(&low[m]); hi[m] = max(atomicLoad(&high[m]), lo[m]); }
  let wide = vec3f(hi - lo + vec3u(1u)) / 256.0; var tau = vec3u(0u);
  for (var a = 0u; a < 3u; a++) { let face = wide[(a + 1u) % 3u] * wide[(a + 2u) % 3u]; let c = min(max(n[2u * a], n[2u * a + 1u]) / f32(RAYS) / max(face, 1e-3), 0.9984); tau[a] = u32(clamp(-log(1.0 - c) * (255.0 / 6.4) + 0.5, 0.0, 255.0)); }
  out[0] = tau.x | (tau.y << 8u) | (tau.z << 16u) | (u32(f32(atomicLoad(&leafy)) / all * 63.0) << 26u); out[1] = lo.x | (lo.y << 8u) | (lo.z << 16u) | (select(0u, 1u, atomicLoad(&grass) * 2u > atomicLoad(&leafy)) << 24u); out[2] = hi.x | (hi.y << 8u) | (hi.z << 16u);
  var mean = vec3f(0.0); for (var k = 0u; k < 6u; k++) { mean += vec3f(f32(atomicLoad(&red[k])), f32(atomicLoad(&green[k])), f32(atomicLoad(&blue[k]))); } mean /= all;
  for (var k = 0u; k < 6u; k++) { let c = select(mean, vec3f(f32(atomicLoad(&red[k])), f32(atomicLoad(&green[k])), f32(atomicLoad(&blue[k]))) / max(n[k], 1.0), n[k] > 0.5) / 4095.0; let v = sqrt(c); let w = u32(v.x * 31.0 + 0.5) | (u32(v.y * 63.0 + 0.5) << 5u) | (u32(v.z * 31.0 + 0.5) << 11u); let z = 3u + (k >> 1u); out[z] = out[z] | (w << select(0u, 16u, (k & 1u) == 0u)); }
  for (var k = 0u; k < 8u; k++) { made[cell * 8u + k] = out[k]; }
}`;
/** `most`: rays a side of each cell, squared; eight is as many as there are threads */
export async function measureBlocks(lab: Lab, most8 = 8): Promise<string> {
  const grid = Math.min(8, most8);
  const { scene, gpu } = lab, B = scene.blocks, d = gpu.device, C = GPUShaderStage.COMPUTE; if (!B.many) return 'no cells';
  const places = buffer(gpu, 'cells at', B.cells, GPUBufferUsage.STORAGE), made = buffer(gpu, 'cells made', B.many * 32, GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_SRC), most = buffer(gpu, 'cells many', new Uint32Array([B.many, 0, 0, 0]), GPUBufferUsage.UNIFORM), t0 = performance.now();
  const l1 = d.createBindGroupLayout({ entries: [{ binding: 0, visibility: C, buffer: { type: 'read-only-storage' } }, { binding: 1, visibility: C, buffer: { type: 'storage' } }, { binding: 2, visibility: C, buffer: { type: 'uniform' } }] });
  const module = d.createShaderModule({ label: 'cells', code: secondCore(lab.graph.prelude()) + code(grid) }), bad = (await module.getCompilationInfo()).messages.filter(m => m.type === 'error'); if (bad.length) throw new Error(bad.map(m => `cells:${m.lineNum}:${m.linePos} ${m.message}`).slice(0, 5).join('\n'));
  const pipe = await d.createComputePipelineAsync({ label: 'cells', layout: d.createPipelineLayout({ bindGroupLayouts: [scene.layout, l1] }), compute: { module, entryPoint: 'main' } }), bg = d.createBindGroup({ layout: l1, entries: [{ binding: 0, resource: { buffer: places } }, { binding: 1, resource: { buffer: made } }, { binding: 2, resource: { buffer: most } }] });
  const e = d.createCommandEncoder(), p = e.beginComputePass(); p.setPipeline(pipe); p.setBindGroup(0, scene.group()); p.setBindGroup(1, bg); p.dispatchWorkgroups(Math.min(32768, B.many), Math.ceil(B.many / 32768)); p.end(); d.queue.submit([e.finish()]);
  const r = new Uint32Array(await readBuffer(gpu, made)); for (const b of [places, made, most]) b.destroy();
  // each that has something in it keeps light; the empty are forgotten
  const at: number[] = [], slot = new Uint32Array(B.many); let lit = 0;
  for (let i = 0; i < B.many; i++) { const level = B.cells[i * 4 + 3]!; if (!(r[i * 8]! & 0xffffff)) { B.bricks[B.said[i]!] = 0; continue; } slot[i] = ++lit; const h = FINEST * 2 ** level, y = r[i * 8 + 1]!, z = r[i * 8 + 2]!, lo = [y & 255, (y >> 8) & 255, (y >> 16) & 255], hi = [z & 255, (z >> 8) & 255, (z >> 16) & 255]; for (let k = 0; k < 3; k++) at.push(B.cells[i * 4 + k]! + h * (lo[k]! + hi[k]! + 1) / 512); at.push(h); for (let k = 0; k < 3; k++) at.push(h * (hi[k]! + 1 - lo[k]!) / 256); at.push(i + 1); }
  for (let i = 0; i < B.many; i++) { const s = slot[i]!; if (s >= 1 << 17) throw new Error('more cells than a word can name'); r[i * 8] = (r[i * 8]! | ((s >>> 15) << 24)) >>> 0; r[i * 8 + 1] = (r[i * 8 + 1]! | (((s >>> 8) & 127) << 25)) >>> 0; r[i * 8 + 2] = (r[i * 8 + 2]! | ((s & 255) << 24)) >>> 0; }
  const green = new Uint8Array(B.bricks.length / 588); for (let i = 0; i < B.many; i++) if (r[i * 8]! & 0xffffff && r[i * 8]! >>> 26) green[Math.floor(B.said[i]! / 588)] = 1; for (let i = 0; i < B.page.length; i++) { const p = B.page[i]! & 0x7fffffff; B.page[i] = p ? (p | (green[p - 1] ? 0x80000000 : 0)) >>> 0 : 0; } scene.writeExtra(scene.blockRow, new Uint8Array(B.page.buffer));
  // an empty cell says how far round it there is nothing (within its brick), so that rays go by in strides
  const far = new Uint8Array(512);
  for (let b = 0; b < B.bricks.length; b += 588) for (const [from, per] of [[0, 8], [512, 4]] as const) {
    const n = per ** 3; for (let i = 0; i < n; i++) far[i] = B.bricks[b + from + i] ? 0 : per; const pull = (i: number, x: number, y: number, z: number): void => { for (let c = -1; c <= 1; c++) for (let e = -1; e <= 1; e++) for (let f = -1; f <= 1; f++) { const X = x + f, Y = y + e, Z = z + c; if (X < 0 || Y < 0 || Z < 0 || X >= per || Y >= per || Z >= per) continue; const v = far[X + per * (Y + per * Z)]! + 1; if (v < far[i]!) far[i] = v; } };
    for (let i = 0; i < n; i++) pull(i, i % per, Math.floor(i / per) % per, Math.floor(i / per / per)); for (let i = n - 1; i >= 0; i--) pull(i, i % per, Math.floor(i / per) % per, Math.floor(i / per / per));
    for (let i = 0; i < n; i++) if (far[i]) B.bricks[b + from + i] = (0x80000000 | far[i]!) >>> 0;
  }
  scene.writeExtra(scene.blockRow + B.page.length / 4, new Uint8Array(B.bricks.buffer)); scene.writeExtra(cellRow(B, scene.blockRow), new Uint8Array(r.buffer)); const places2 = new Float32Array(at), pu = new Uint32Array(places2.buffer); for (let i = 0; i < lit; i++) pu[i * 8 + 7] = at[i * 8 + 7]!; lab.litCells = { many: lit, at: places2 };
  let full = 0, leafy = 0; const by = [0, 0, 0, 0]; for (let i = 0; i < B.many; i++) if (r[i * 8]! & 0xffffff) { full++; by[B.cells[i * 4 + 3]!]!++; if (r[i * 8]! >>> 31) leafy++; }
  return `${full} of ${B.many} cells have something in them (${by.join(', ')} by level), ${leafy} mostly leaves; ${B.bricks.length / 588} bricks; ${((B.page.length + B.bricks.length + B.many * 8) * 4 / 1e6).toFixed(2)} MB; ${(performance.now() - t0).toFixed(0)} ms`;
}
