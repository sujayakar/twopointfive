// The raster: ids and depth for the camera, ids of the clear layer, and the leaves as the sun sees them.
import type { Lab } from '../lab';
import type { Stage, Graph } from '../graph';
import { buffer } from '../gpu';
import code from './raster.wgsl' with { type: 'text' };
export const SUN_MAP = 2048;
export function rasterStage(lab: Lab, g: Graph): Stage {
  g.signal('id', 'r32uint', 'render').signal('idClear', 'r32uint', 'render').signal('depth', 'depth32float', 'render').signal('depthClear', 'depth32float', 'render').signal('sunLeaves', 'depth32float', [SUN_MAP, SUN_MAP]);
  return { name: 'raster', over: 'render', code: '', async make() {
    const d = g.g.device, V = GPUShaderStage.VERTEX, l1 = d.createBindGroupLayout({ entries: [{ binding: 0, visibility: V, buffer: { type: 'read-only-storage' } }] }), layout = d.createPipelineLayout({ bindGroupLayouts: [g.shared.layout, l1] });
    const text = g.prelude() + '@group(1) @binding(0) var<storage, read> lists: array<u32>;\n' + code, module = d.createShaderModule({ label: 'raster', code: text }); const bad = (await module.getCompilationInfo()).messages.find(m => m.type === 'error'); if (bad) throw new Error(`raster:${bad.lineNum} ${bad.message}\n${text.split('\n')[bad.lineNum - 1]}`);
    const depthStencil: GPUDepthStencilState = { format: 'depth32float', depthWriteEnabled: true, depthCompare: 'greater' }, mk = (vs: string, fs: string | null, cull: GPUCullMode = 'none') => d.createRenderPipelineAsync({ label: vs, layout, vertex: { module, entryPoint: vs }, ...(fs ? { fragment: { module, entryPoint: fs, targets: [{ format: 'r32uint' as const }] } } : {}), primitive: { topology: 'triangle-list', cullMode: cull }, depthStencil });
    const [box, fancy, leaf, sun] = await Promise.all([mk('vsBox', 'fsBox'), mk('vsBox', 'fsFancy'), mk('vsLeaf', 'fsLeaf'), mk('vsSun', null)]); const lists = d.createBindGroup({ layout: l1, entries: [{ binding: 0, resource: { buffer: lab.scene.lists } }] });
    // corners shared by the triangles round them: a box is eight, a leaf six, a leaf to the sun four
    const most = Math.max(...lab.building.world.clusters.map(c => c.leaves.length)), index = (n: number, per: number, tri: number[]) => { const u = new Uint32Array(n * tri.length); for (let i = 0; i < n; i++) tri.forEach((k, j) => { u[i * tri.length + j] = i * per + k; }); return buffer(g.g, 'index', u, GPUBufferUsage.INDEX); };
    const boxI = index(1, 8, [0, 2, 1, 1, 2, 3, 4, 5, 6, 5, 7, 6, 0, 1, 4, 1, 5, 4, 2, 6, 3, 3, 6, 7, 0, 4, 2, 2, 4, 6, 1, 3, 5, 3, 7, 5]), leafI = index(most, 6, [0, 1, 2, 1, 3, 2, 2, 3, 4, 3, 5, 4]), sunI = index(most, 4, [0, 1, 2, 1, 3, 2]);
    return enc => {
      const D = lab.scene.draws, ts = (name: string) => g.timer.pass(name) as GPURenderPassTimestampWrites | undefined;
      const pass = (label: string, color: string | null, depth: string, f: (p: GPURenderPassEncoder) => void) => { const p = enc.beginRenderPass({ label, colorAttachments: color ? [{ view: g.view(color), loadOp: 'clear', storeOp: 'store', clearValue: { r: 0, g: 0, b: 0, a: 0 } }] : [], depthStencilAttachment: { view: g.view(depth), depthLoadOp: 'clear', depthStoreOp: 'store', depthClearValue: 0 }, timestampWrites: ts(label) }); p.setBindGroup(0, g.shared.group()); p.setBindGroup(1, lists); f(p); p.end(); };
      const boxes = (p: GPURenderPassEncoder, [first, n]: [number, number], pipe: GPURenderPipeline) => { if (n) { p.setPipeline(pipe); p.setIndexBuffer(boxI, 'uint32'); p.drawIndexed(36, n, 0, 0, first); } }, leaves = (p: GPURenderPassEncoder, list: typeof lab.scene.leafDraws, pipe: GPURenderPipeline, ix: GPUBuffer, per: number) => { p.setPipeline(pipe); p.setIndexBuffer(ix, 'uint32'); for (const l of list) if (l.count) p.drawIndexed(l.leaves * per, l.count, 0, 0, l.first); };
      pass('raster', 'id', 'depth', p => { boxes(p, D.plain, box); leaves(p, lab.scene.leafDraws, leaf, leafI, 12); boxes(p, D.fancy, fancy); });
      pass('raster clear', 'idClear', 'depthClear', p => { boxes(p, D.clear, box); boxes(p, D.clearFancy, fancy); });
      pass('sun leaves', null, 'sunLeaves', p => leaves(p, lab.scene.sunDraws, sun, sunI, 6));
    };
  } };
}
