// The raster: ids and depth for the camera, ids of the clear layer, and the leaves as the sun sees them.
import type { Lab } from '../lab';
import type { Stage, Graph } from '../graph';
import code from './raster.wgsl' with { type: 'text' };
export const SUN_MAP = 2048;
export function rasterStage(lab: Lab, g: Graph): Stage {
  g.signal('id', 'r32uint', 'render').signal('idClear', 'r32uint', 'render').signal('depth', 'depth32float', 'render').signal('depthClear', 'depth32float', 'render').signal('sunLeaves', 'depth32float', [SUN_MAP, SUN_MAP]);
  return { name: 'raster', over: 'render', code: '', async make() {
    const d = g.g.device, V = GPUShaderStage.VERTEX, l1 = d.createBindGroupLayout({ entries: [{ binding: 0, visibility: V, buffer: { type: 'read-only-storage' } }] }), layout = d.createPipelineLayout({ bindGroupLayouts: [g.shared.layout, l1] });
    const text = g.prelude() + '@group(1) @binding(0) var<storage, read> lists: array<u32>;\n' + code, module = d.createShaderModule({ label: 'raster', code: text }); const bad = (await module.getCompilationInfo()).messages.find(m => m.type === 'error'); if (bad) throw new Error(`raster:${bad.lineNum} ${bad.message}\n${text.split('\n')[bad.lineNum - 1]}`);
    const depthStencil: GPUDepthStencilState = { format: 'depth32float', depthWriteEnabled: true, depthCompare: 'greater' }, mk = (vs: string, fs: string | null, cull: GPUCullMode = 'none') => d.createRenderPipelineAsync({ label: vs, layout, vertex: { module, entryPoint: vs }, ...(fs ? { fragment: { module, entryPoint: fs, targets: [{ format: 'r32uint' as const }] } } : {}), primitive: { topology: 'triangle-list', cullMode: cull }, depthStencil });
    const [box, fancy, leaf, clear, clearCut, sun] = await Promise.all([mk('vsBox', 'fsBox'), mk('vsFancy', 'fsFancy'), mk('vsLeaf', 'fsLeaf'), mk('vsClear', 'fsBox'), mk('vsClearCut', 'fsFancy'), mk('vsSun', null)]); const lists = d.createBindGroup({ layout: l1, entries: [{ binding: 0, resource: { buffer: lab.scene.lists } }] });
    const clearIds = [0, 0]; { const s = lab.scene.standing; let a = -1, b = 0; s.forEach((p, i) => { if (p.flags & 0x300) { if (a < 0) a = i; b = i + 1; } }); clearIds[0] = Math.max(0, a); clearIds[1] = b; }
    return enc => {
      const n = lab.scene.counts.pieces + lab.scene.counts.moving, ts = (name: string) => g.timer.pass(name) as GPURenderPassTimestampWrites | undefined;
      const pass = (label: string, color: string | null, depth: string, f: (p: GPURenderPassEncoder) => void) => { const p = enc.beginRenderPass({ label, colorAttachments: color ? [{ view: g.view(color), loadOp: 'clear', storeOp: 'store', clearValue: { r: 0, g: 0, b: 0, a: 0 } }] : [], depthStencilAttachment: { view: g.view(depth), depthLoadOp: 'clear', depthStoreOp: 'store', depthClearValue: 0 }, timestampWrites: ts(label) }); p.setBindGroup(0, g.shared.group()); p.setBindGroup(1, lists); f(p); p.end(); };
      const leaves = (p: GPURenderPassEncoder) => { for (const l of lab.scene.leafDraws) if (l.count) p.draw(l.leaves * 12, l.count, 0, l.first); };
      pass('raster', 'id', 'depth', p => { p.setPipeline(box); p.draw(36, n); p.setPipeline(fancy); p.draw(36, n); p.setPipeline(leaf); leaves(p); });
      pass('raster clear', 'idClear', 'depthClear', p => { p.setPipeline(clear); p.draw(36, clearIds[1]! - clearIds[0]!, 0, clearIds[0]); p.setPipeline(clearCut); p.draw(36, clearIds[1]! - clearIds[0]!, 0, clearIds[0]); });
      pass('sun leaves', null, 'sunLeaves', p => { p.setPipeline(sun); leaves(p); });
    };
  } };
}
