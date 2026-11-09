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
@compute @workgroup_size(64) fn main(@builtin(workgroup_id) wg: vec3u, @builtin(local_invocation_index) li: u32) {
  let cell = wg.x + wg.y * 32768u; if (cell >= most.x) { return; } let at = places[cell]; let level = u32(at.w); let h = FINEST * f32(1u << level);
  if (li < 6u) { atomicStore(&met[li], 0u); atomicStore(&red[li], 0u); atomicStore(&green[li], 0u); atomicStore(&blue[li], 0u); } if (li < 3u) { atomicStore(&low[li], 255u); atomicStore(&high[li], 0u); } if (li == 6u) { atomicStore(&leafy, 0u); } if (li == 7u) { atomicStore(&grass, 0u); } workgroupBarrier();
  leafMode = 0u; blkOn = false; blk = vec4f(1e9); lim = vec4f(1e9, select(0.0, 1e9, level > 0u), select(0.0, 1e9, level > 1u), 0.0);   /* (only what this level holds) */
  if (li < RAYS) {
    rngKey(cell * 64u + li, 77u);
    for (var k = 0u; k < 6u; k++) {
      let a = k >> 1u; let b = (a + 1u) % 3u; let c = (a + 2u) % 3u; let j = (vec2f(f32(li % GRID), f32(li / GRID)) + rnd2()) / f32(GRID); var o = at.xyz; o[b] += j.x * h; o[c] += j.y * h; var d = vec3f(0.0); if ((k & 1u) == 0u) { o[a] += h; d[a] = -1.0; } else { d[a] = 1.0; }      /* (even: seen from the + side) */
      let q = nearest(o, d, 0.0, h, GLASS | WATER, 1e9); if (q.piece == NOTHING) { continue; }
      let leaf = (q.piece & LEAF_HIT) != 0u; let s = surfaceAt(o, d, q, 1.0); let e = select(sends(s), s.albedo, leaf); let v = vec3u(clamp(e, vec3f(0.0), vec3f(1.0)) * 4095.0);
      atomicAdd(&met[k], 1u); atomicAdd(&red[k], v.x); atomicAdd(&green[k], v.y); atomicAdd(&blue[k], v.z); if (leaf) { atomicAdd(&leafy, 1u); if (s.kind == 20u || ((insts[q.piece & 0x7fffffffu].tint >> 25u) & 1u) != 0u) { atomicAdd(&grass, 1u); } }
      let x = clamp((o + d * q.t - at.xyz) / h, vec3f(0.0), vec3f(0.9999)); for (var m = 0u; m < 3u; m++) { let z = u32(x[m] * 255.0); atomicMin(&low[m], z); atomicMax(&high[m], z); }
    }
  }
  workgroupBarrier(); if (li != 0u) { return; }
  var n: array<f32, 6>; var all = 0.0; for (var k = 0u; k < 6u; k++) { n[k] = f32(atomicLoad(&met[k])); all += n[k]; } var out: array<u32, 8>; if (all < 0.5) { for (var k = 0u; k < 8u; k++) { made[cell * 8u + k] = 0u; } return; }
  var lo = vec3u(0u); var hi = vec3u(0u); for (var m = 0u; m < 3u; m++) { let z = atomicLoad(&low[m]); let y = atomicLoad(&high[m]); lo[m] = z / 16u; hi[m] = max(y / 16u, lo[m]); }
  let wide = vec3f(hi - lo + vec3u(1u)) / 16.0; var tau = vec3u(0u);
  for (var a = 0u; a < 3u; a++) { let face = wide[(a + 1u) % 3u] * wide[(a + 2u) % 3u]; let c = min(max(n[2u * a], n[2u * a + 1u]) / f32(RAYS) / max(face, 1e-3), 0.9984); tau[a] = u32(clamp(-log(1.0 - c) * (255.0 / 6.4) + 0.5, 0.0, 255.0)); }
  out[0] = tau.x | (tau.y << 8u) | (tau.z << 16u) | (u32(f32(atomicLoad(&leafy)) / all * 255.0) << 24u); out[1] = lo.x | (lo.y << 4u) | (lo.z << 8u) | (hi.x << 12u) | (hi.y << 16u) | (hi.z << 20u) | (select(0u, 1u, atomicLoad(&grass) * 2u > atomicLoad(&leafy)) << 24u);
  var mean = vec3f(0.0); for (var k = 0u; k < 6u; k++) { mean += vec3f(f32(atomicLoad(&red[k])), f32(atomicLoad(&green[k])), f32(atomicLoad(&blue[k]))); } mean /= all;
  for (var k = 0u; k < 6u; k++) { let c = select(mean, vec3f(f32(atomicLoad(&red[k])), f32(atomicLoad(&green[k])), f32(atomicLoad(&blue[k]))) / max(n[k], 1.0), n[k] > 0.5) / 4095.0; let v = sqrt(c); let w = u32(v.x * 31.0 + 0.5) | (u32(v.y * 63.0 + 0.5) << 5u) | (u32(v.z * 31.0 + 0.5) << 11u); let z = 2u + (k >> 1u); out[z] = out[z] | (w << select(0u, 16u, (k & 1u) == 0u)); }
  for (var k = 0u; k < 8u; k++) { made[cell * 8u + k] = out[k]; }
}`;
export async function measureBlocks(lab: Lab, grid = 8): Promise<string> {
  const { scene, gpu } = lab, B = scene.blocks, d = gpu.device, C = GPUShaderStage.COMPUTE; if (!B.many) return 'no cells';
  const places = buffer(gpu, 'cells at', B.cells, GPUBufferUsage.STORAGE), made = buffer(gpu, 'cells made', B.many * 32, GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_SRC), most = buffer(gpu, 'cells many', new Uint32Array([B.many, 0, 0, 0]), GPUBufferUsage.UNIFORM), t0 = performance.now();
  const l1 = d.createBindGroupLayout({ entries: [{ binding: 0, visibility: C, buffer: { type: 'read-only-storage' } }, { binding: 1, visibility: C, buffer: { type: 'storage' } }, { binding: 2, visibility: C, buffer: { type: 'uniform' } }] });
  const module = d.createShaderModule({ label: 'cells', code: secondCore(lab.graph.prelude()) + code(grid) }), bad = (await module.getCompilationInfo()).messages.filter(m => m.type === 'error'); if (bad.length) throw new Error(bad.map(m => `cells:${m.lineNum}:${m.linePos} ${m.message}`).slice(0, 5).join('\n'));
  const pipe = await d.createComputePipelineAsync({ label: 'cells', layout: d.createPipelineLayout({ bindGroupLayouts: [scene.layout, l1] }), compute: { module, entryPoint: 'main' } }), bg = d.createBindGroup({ layout: l1, entries: [{ binding: 0, resource: { buffer: places } }, { binding: 1, resource: { buffer: made } }, { binding: 2, resource: { buffer: most } }] });
  const e = d.createCommandEncoder(), p = e.beginComputePass(); p.setPipeline(pipe); p.setBindGroup(0, scene.group()); p.setBindGroup(1, bg); p.dispatchWorkgroups(Math.min(32768, B.many), Math.ceil(B.many / 32768)); p.end(); d.queue.submit([e.finish()]);
  const r = new Uint32Array(await readBuffer(gpu, made)); for (const b of [places, made, most]) b.destroy();
  // who keeps light: each that has something in it, from 0.32 m up (a finer one reads the one it is in); and the empty are forgotten
  const at: number[] = [], slot = new Uint32Array(B.many); let lit = 0;
  for (let i = 0; i < B.many; i++) { const level = B.cells[i * 4 + 3]!; if (!(r[i * 8]! & 0xffffff)) { B.bricks[B.said[i]!] = 0; continue; } if (!level) continue; slot[i] = ++lit; const h = FINEST * 2 ** level, y = r[i * 8 + 1]!, lo = [y & 15, (y >> 4) & 15, (y >> 8) & 15], hi = [(y >> 12) & 15, (y >> 16) & 15, (y >> 20) & 15]; for (let k = 0; k < 3; k++) at.push(B.cells[i * 4 + k]! + h * (lo[k]! + hi[k]! + 1) / 32); at.push(h); for (let k = 0; k < 3; k++) at.push(h * (hi[k]! + 1 - lo[k]!) / 16); at.push(i + 1); }
  for (let i = 0; i < B.many; i++) if (B.cells[i * 4 + 3] === 0) { const w = B.said[i]!, b = Math.floor(w / 588) * 588, s = w - b, up = B.bricks[b + 512 + ((s & 7) >> 1) + 4 * ((((s >> 3) & 7) >> 1) + 4 * (s >> 7))]!; slot[i] = up ? slot[up - 1]! : 0; }
  for (let i = 0; i < B.many; i++) r[i * 8 + 6] = slot[i]!;
  const green = new Uint8Array(B.bricks.length / 588); for (let i = 0; i < B.many; i++) if (r[i * 8]! & 0xffffff && r[i * 8]! >>> 24) green[Math.floor(B.said[i]! / 588)] = 1; for (let i = 0; i < B.page.length; i++) { const p = B.page[i]! & 0x7fffffff; B.page[i] = p ? (p | (green[p - 1] ? 0x80000000 : 0)) >>> 0 : 0; } scene.writeExtra(scene.blockRow, new Uint8Array(B.page.buffer));
  scene.writeExtra(scene.blockRow + B.page.length / 4, new Uint8Array(B.bricks.buffer)); scene.writeExtra(cellRow(B, scene.blockRow), new Uint8Array(r.buffer)); const places2 = new Float32Array(at), pu = new Uint32Array(places2.buffer); for (let i = 0; i < lit; i++) pu[i * 8 + 7] = at[i * 8 + 7]!; lab.litCells = { many: lit, at: places2 };
  let full = 0, leafy = 0; const by = [0, 0, 0, 0]; for (let i = 0; i < B.many; i++) if (r[i * 8]! & 0xffffff) { full++; by[B.cells[i * 4 + 3]!]!++; if (r[i * 8]! >>> 24 > 127) leafy++; }
  return `${full} of ${B.many} cells have something in them (${by.join(', ')} by level), ${leafy} mostly leaves; ${B.bricks.length / 588} bricks; ${((B.page.length + B.bricks.length + B.many * 8) * 4 / 1e6).toFixed(2)} MB; ${(performance.now() - t0).toFixed(0)} ms`;
}
