// The raster: ids and depth for the camera, ids of the clear layer, and the leaves as the sun sees them.
import type { Lab } from '../lab';
import type { Stage, Graph } from '../graph';
import { buffer } from '../gpu';
import code from './raster.wgsl' with { type: 'text' };
import { pickTorches, torchView, TORCH } from './torches';
export const SUN_MAP = 2048;
export function rasterStage(lab: Lab, g: Graph): Stage {
  g.signal('id', 'r32uint', 'render').signal('idClear', 'r32uint', 'render').signal('depth', 'depth32float', 'render').signal('depthClear', 'depth32float', 'render').signal('sunLeaves', 'depth32float', [SUN_MAP, SUN_MAP]).signal('torchLeaves', 'depth32float', [TORCH.tile * 4, TORCH.tile * 2]);
  return { name: 'raster', over: 'render', code: '', async make() {
    const d = g.g.device, V = GPUShaderStage.VERTEX, l1 = d.createBindGroupLayout({ entries: [{ binding: 0, visibility: V, buffer: { type: 'read-only-storage' } }] }), l2 = d.createBindGroupLayout({ entries: [{ binding: 0, visibility: V, buffer: { type: 'uniform' } }] }), layout = d.createPipelineLayout({ bindGroupLayouts: [g.shared.layout, l1] }), layoutT = d.createPipelineLayout({ bindGroupLayouts: [g.shared.layout, l1, l2] });
    const text = g.prelude() + '@group(1) @binding(0) var<storage, read> lists: array<u32>;\n@group(2) @binding(0) var<uniform> torch: vec4u;\n' + torchView + code, module = d.createShaderModule({ label: 'raster', code: text }); const bad = (await module.getCompilationInfo()).messages.find(m => m.type === 'error'); if (bad) throw new Error(`raster:${bad.lineNum} ${bad.message}\n${text.split('\n')[bad.lineNum - 1]}`);
    const depthStencil: GPUDepthStencilState = { format: 'depth32float', depthWriteEnabled: true, depthCompare: 'greater' }, mk = (vs: string, fs: string | null, cull: GPUCullMode = 'none', lay = layout) => d.createRenderPipelineAsync({ label: vs, layout: lay, vertex: { module, entryPoint: vs }, ...(fs ? { fragment: { module, entryPoint: fs, targets: [{ format: 'r32uint' as const }] } } : {}), primitive: { topology: 'triangle-list', cullMode: cull }, depthStencil });
    const [box, fancy, leaf, sun, torch] = await Promise.all([mk('vsBox', 'fsBox'), mk('vsBox', 'fsFancy'), mk('vsLeaf', 'fsLeaf'), mk('vsSun', null), mk('vsTorch', null, 'none', layoutT)]); const which = Array.from({ length: TORCH.most }, (_, k) => lab.uniform('torchAt' + k, new Uint32Array(4))), whose = which.map(b => d.createBindGroup({ layout: l2, entries: [{ binding: 0, resource: { buffer: b } }] })); const lists = d.createBindGroup({ layout: l1, entries: [{ binding: 0, resource: { buffer: lab.scene.lists } }] });
    // corners shared by the triangles round them: a box is eight, a leaf six, a leaf to the sun four
    const most = Math.max(...lab.building.world.clusters.map(c => c.leaves.length)), index = (n: number, per: number, tri: number[]) => { const u = new Uint32Array(n * tri.length); for (let i = 0; i < n; i++) tri.forEach((k, j) => { u[i * tri.length + j] = i * per + k; }); return buffer(g.g, 'index', u, GPUBufferUsage.INDEX); };
    const boxI = index(1, 8, [0, 2, 1, 1, 2, 3, 4, 5, 6, 5, 7, 6, 0, 1, 4, 1, 5, 4, 2, 6, 3, 3, 6, 7, 0, 4, 2, 2, 4, 6, 1, 3, 5, 3, 7, 5]), leafI = index(most, 6, [0, 1, 2, 1, 3, 2, 2, 3, 4, 3, 5, 4]), sunI = index(most, 4, [0, 1, 2, 1, 3, 2]);
    return enc => {
      const D = lab.scene.draws, ts = (name: string) => g.timer.pass(name) as GPURenderPassTimestampWrites | undefined;
      const pass = (label: string, color: string | null, depth: string, f: (p: GPURenderPassEncoder) => void) => { const p = enc.beginRenderPass({ label, colorAttachments: color ? [{ view: g.view(color), loadOp: 'clear', storeOp: 'store', clearValue: { r: 0, g: 0, b: 0, a: 0 } }] : [], depthStencilAttachment: { view: g.view(depth), depthLoadOp: 'clear', depthStoreOp: 'store', depthClearValue: 0 }, timestampWrites: ts(label) }); p.setBindGroup(0, g.shared.group()); p.setBindGroup(1, lists); f(p); p.end(); };
      const boxes = (p: GPURenderPassEncoder, [first, n]: [number, number], pipe: GPURenderPipeline) => { if (n) { p.setPipeline(pipe); p.setIndexBuffer(boxI, 'uint32'); p.drawIndexed(36, n, 0, 0, first); } }, leaves = (p: GPURenderPassEncoder, list: typeof lab.scene.leafDraws, pipe: GPURenderPipeline, ix: GPUBuffer, per: number) => { p.setPipeline(pipe); p.setIndexBuffer(ix, 'uint32'); for (const l of list) if (l.count) p.drawIndexed(l.leaves * per, l.count, 0, 0, l.first); };
      pass('raster', 'id', 'depth', p => { boxes(p, D.plain, box); leaves(p, lab.scene.leafDraws, leaf, leafI, 12); boxes(p, D.fancy, fancy); });
      pass('raster clear', 'idClear', 'depthClear', p => { boxes(p, D.clear, box); boxes(p, D.clearFancy, fancy); });
      const on = pickTorches(lab); lab.torches = on.length; on.forEach((l, k) => lab.uniform('torchAt' + k, new Uint32Array([l, k, 0, 0])));
      if (on.length && lab.scene.sunDraws.some(q => q.count)) pass('torch leaves', null, 'torchLeaves', p => { for (let k = 0; k < on.length; k++) { p.setBindGroup(2, whose[k]!); p.setViewport((k & 3) * TORCH.tile, (k >> 2) * TORCH.tile, TORCH.tile, TORCH.tile, 0, 1); p.setScissorRect((k & 3) * TORCH.tile, (k >> 2) * TORCH.tile, TORCH.tile, TORCH.tile); leaves(p, lab.scene.sunDraws, torch, sunI, 6); } });
      pass('sun leaves', null, 'sunLeaves', p => leaves(p, lab.scene.sunDraws, sun, sunI, 6));
    };
  } };
}
