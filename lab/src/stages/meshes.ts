// MESHES IN THE PICTURE: picked on the GPU, drawn into the same ids and depth as everything else. A stage beside raster.ts, put in after it.
import type { Lab, Method } from '../lab';
import type { Graph, Stage } from '../graph';
import { buffer } from '../gpu';
import { F } from '../scene/world';
import { rasterStage } from './raster';
import pickCode from './meshpick.wgsl' with { type: 'text' };
import drawCode from './meshdraw.wgsl' with { type: 'text' };

/** turned while looking: pixels of error; or a fixed error in metres; what is left out; how corners are fetched */
export const RASTER = { tau: 0.5, fixed: 0, cone: true, hide: true, share: true, /** only the heroes, for measuring */ heroes: false, /** what was picked stays, wherever the camera goes */ freeze: false };
const QUEUE = 1 << 19, MOST = 1 << 17, INDICES = 1 << 24, GROUPS = 256;
/** indices of what was picked (expand), and what is to be drawn of it (say): first all that was seen at once, then what the second look found */
const shareCode = `@group(1) @binding(0) var<storage, read> slots: array<vec4u>;
@group(1) @binding(1) var<storage, read_write> n: array<atomic<u32>>;
@group(1) @binding(2) var<storage, read_write> indices: array<u32>;
@group(1) @binding(3) var<uniform> part: vec4u;       /* which look, slots and indices at most */
@group(1) @binding(4) var<storage, read_write> args: array<u32>;
var<workgroup> base: u32; var<workgroup> span: vec2u;
@compute @workgroup_size(128) fn expand(@builtin(workgroup_id) g: vec3u, @builtin(local_invocation_index) t: u32, @builtin(num_workgroups) groups: vec3u) {
  if (t == 0u) { span = vec2u(select(0u, atomicLoad(&n[27]), part.x == 1u), min(atomicLoad(&n[24]), part.y)); } let todo = workgroupUniformLoad(&span);
  for (var s = todo.x + g.x; s < todo.y; s += groups.x) {
    let r = slots[s * 4u + 3u]; let c = r.w & 255u; if (t == 0u) { base = atomicAdd(&n[30], c * 3u); } let b = workgroupUniformLoad(&base);
    if (t < c && b + t * 3u + 2u < part.z) { let ix = mrowU(r.y + (t >> 2u))[t & 3u]; for (var k = 0u; k < 3u; k++) { indices[b + t * 3u + k] = (s << 8u) | ((ix >> (8u * k)) & 255u); } }
    workgroupBarrier();
  }
}
@compute @workgroup_size(1) fn say() {
  let a = select(0u, atomicLoad(&n[27]), part.x == 1u); let b = min(atomicLoad(&n[24]), part.y); let i = select(0u, atomicLoad(&n[28]), part.x == 1u); let j = min(atomicLoad(&n[30]), part.z); atomicStore(&n[27], b); atomicStore(&n[28], j);
  let most = max(max(f32(b) / f32(part.y), f32(j) / f32(part.z)), select(0.0, 2.0, atomicLoad(&n[29]) != 0u)); let was = max(bitcast<f32>(atomicLoad(&n[31])), 1.0); atomicStore(&n[31], bitcast<u32>(clamp(was * select(select(1.0, 0.985, most < 0.75), 1.2, most > 0.92), 1.0, 64.0)));
  args[0] = (b - a) * 384u; args[1] = 1u; args[2] = a * 384u; args[3] = 0u; args[4] = j - i; args[5] = 1u; args[6] = i; args[7] = 0u; args[8] = 0u;
}`;
export interface Drawn { slots: number; triangles: number; hid: number; full: number; /** times coarser than asked, so that no list is full */ coarser: number; rounds: number[] }

export function meshStage(lab: Lab, g: Graph): Stage {
  return { name: 'meshes', over: 'render', code: '', async make() {
    const d = g.g.device, w = lab.building.world.meshes!, S = GPUBufferUsage.STORAGE, C = GPUShaderStage.COMPUTE, mk = (n: string, bytes: number, usage = S) => buffer(g.g, n, bytes, usage), shared = (n: string, bytes: number, more = 0) => lab.buffer(n, bytes, S | GPUBufferUsage.COPY_SRC | more);
    const floor = new Set(w.objects.filter(o => o.levels.length > 1).map(o => o.row)), all = lab.scene.standing.flatMap((p, i) => p.flags & F.MESH ? [i] : []).sort((a, b) => +floor.has(lab.scene.standing[a]!.mesh!) - +floor.has(lab.scene.standing[b]!.mesh!)), heroes = all.filter(i => !floor.has(lab.scene.standing[i]!.mesh!)).length, placed = mk('placed', Math.max(16, all.length * 4)); d.queue.writeBuffer(placed, 0, new Uint32Array(all));
    const lists = [shared('meshListA', QUEUE * 8), shared('meshListB', QUEUE * 8)], n = shared('meshN', 128), slots = shared('meshSlots', MOST * 64), again = shared('meshAgain', MOST * 16), args = shared('meshArgs', 64, GPUBufferUsage.INDIRECT), indices = shared('meshIndices', INDICES * 4, GPUBufferUsage.INDEX);
    // ---- how far everything is, coarser and coarser: the farthest of each square
    const hzbGroups = new Map<string, GPUBindGroup>(); let hzb: GPUTexture | null = null, hzbViews: GPUTextureView[] = [], hzbSize: [number, number] = [0, 0];
    const hzbLayout = (first: boolean) => d.createBindGroupLayout({ entries: [{ binding: 0, visibility: C, texture: first ? { sampleType: 'depth' } : { sampleType: 'unfilterable-float' } }, { binding: 1, visibility: C, storageTexture: { access: 'write-only', format: 'r32float' } }] });
    const hl = [hzbLayout(true), hzbLayout(false)], hp = await Promise.all([true, false].map((first, i) => d.createComputePipelineAsync({ layout: d.createPipelineLayout({ bindGroupLayouts: [hl[i]!] }), compute: { module: d.createShaderModule({ code: `@group(0) @binding(0) var src: ${first ? 'texture_depth_2d' : 'texture_2d<f32>'}; @group(0) @binding(1) var dst: texture_storage_2d<r32float, write>;
fn at(p: vec2i) -> f32 { let m = vec2i(textureDimensions(src)) - 1; return textureLoad(src, min(p, m), 0)${first ? '' : '.x'}; }
@compute @workgroup_size(8, 8) fn main(@builtin(global_invocation_id) id: vec3u) { let o = textureDimensions(dst); if (any(id.xy >= o)) { return; } let odd = (textureDimensions(src) & vec2u(1u)) == vec2u(1u); let many = vec2i(select(vec2u(2u), vec2u(3u), odd & (id.xy + 1u == o))); var f = 1e9; for (var y = 0; y < many.y; y++) { for (var x = 0; x < many.x; x++) { f = min(f, at(vec2i(id.xy) * 2 + vec2i(x, y))); } } textureStore(dst, id.xy, vec4f(f, 0.0, 0.0, 0.0)); }` }) } })));
    const fit = () => { const [rw, rh] = g.render, s: [number, number] = [Math.max(1, rw >> 1), Math.max(1, rh >> 1)]; if (hzb && s[0] === hzbSize[0] && s[1] === hzbSize[1]) return false; hzb?.destroy(); hzbSize = s; const mips = Math.floor(Math.log2(Math.max(...s))) + 1; hzb = d.createTexture({ label: 'hzb', size: s, format: 'r32float', mipLevelCount: mips, usage: GPUTextureUsage.TEXTURE_BINDING | GPUTextureUsage.STORAGE_BINDING }); hzbViews = Array.from({ length: mips }, (_, i) => hzb!.createView({ baseMipLevel: i, mipLevelCount: 1 })); groups = null; hzbGroups.clear(); return true; };
    const reduce = (p: GPUComputePassEncoder) => { hzbViews.forEach((v, i) => { const k = i ? 1 : 0, key = `${i}:${i ? '' : g.texture('depth').label}`; let b = hzbGroups.get(key); if (!b || i === 0) hzbGroups.set(key, b = d.createBindGroup({ layout: hl[k]!, entries: [{ binding: 0, resource: i ? hzbViews[i - 1]! : g.view('depth') }, { binding: 1, resource: v }] })); p.setPipeline(hp[k]!); p.setBindGroup(0, b); p.dispatchWorkgroups(Math.ceil(Math.max(1, hzbSize[0] >> i) / 8), Math.ceil(Math.max(1, hzbSize[1] >> i) / 8)); }); };
    // ---- picking
    const bl = (binding: number, type: GPUBufferBindingType): GPUBindGroupLayoutEntry => ({ binding, visibility: C, buffer: { type } }), pickL = d.createBindGroupLayout({ entries: [bl(0, 'uniform'), bl(1, 'read-only-storage'), bl(2, 'storage'), bl(3, 'read-only-storage'), bl(4, 'storage'), bl(5, 'storage'), bl(6, 'storage'), { binding: 7, visibility: C, texture: { sampleType: 'unfilterable-float' } }] });
    const pickText = g.prelude() + `@group(1) @binding(0) var<uniform> meshK: MeshK;\n@group(1) @binding(1) var<storage, read> placed: array<u32>;\n@group(1) @binding(2) var<storage, read_write> n: array<atomic<u32>>;\n@group(1) @binding(3) var<storage, read> listA: array<vec2u>;\n@group(1) @binding(4) var<storage, read_write> listB: array<vec2u>;\n@group(1) @binding(5) var<storage, read_write> slots: array<vec4u>;\n@group(1) @binding(6) var<storage, read_write> again: array<vec4u>;\n@group(1) @binding(7) var hzb: texture_2d<f32>;\n` + pickCode, pickM = await compiled(d, 'mesh pick', pickText);
    const pickLayout = d.createPipelineLayout({ bindGroupLayouts: [g.shared.layout, pickL] }), [seed, round, second] = await Promise.all(['seed', 'round', 'second'].map(entryPoint => d.createComputePipelineAsync({ label: entryPoint, layout: pickLayout, compute: { module: pickM, entryPoint } })));
    const ROUNDS = w.depth, K = Array.from({ length: ROUNDS + 2 }, (_, i) => mk('meshK' + i, 48, GPUBufferUsage.UNIFORM)); let groups: GPUBindGroup[] | null = null;
    const bind = () => groups ??= K.map((k, i) => d.createBindGroup({ layout: pickL, entries: [k, placed, n, lists[i & 1]!, lists[(i & 1) ^ 1]!, slots, again].map((b, binding) => ({ binding, resource: { buffer: b } }) as GPUBindGroupEntry).concat({ binding: 7, resource: hzb!.createView() }) }));
    // ---- indices, where corners are shared
    const shareL = d.createBindGroupLayout({ entries: [bl(0, 'read-only-storage'), bl(1, 'storage'), bl(2, 'storage'), bl(3, 'uniform'), bl(4, 'storage')] }), shareM = await compiled(d, 'mesh share', g.prelude() + shareCode), shareLayout = d.createPipelineLayout({ bindGroupLayouts: [g.shared.layout, shareL] }), [expand, say] = await Promise.all(['expand', 'say'].map(entryPoint => d.createComputePipelineAsync({ label: entryPoint, layout: shareLayout, compute: { module: shareM, entryPoint } })));
    const shareG = [0, 1].map(phase => { const k = mk('shareK' + phase, 16, GPUBufferUsage.UNIFORM); d.queue.writeBuffer(k, 0, new Uint32Array([phase, MOST, INDICES, 0])); return d.createBindGroup({ layout: shareL, entries: [slots, n, indices, k, args].map((b, binding) => ({ binding, resource: { buffer: b } })) }); });
    // ---- drawing
    const drawL = d.createBindGroupLayout({ entries: [{ binding: 0, visibility: GPUShaderStage.VERTEX, buffer: { type: 'read-only-storage' } }] }), drawM = await compiled(d, 'mesh draw', g.prelude() + '@group(1) @binding(0) var<storage, read> slots: array<vec4u>;\n' + drawCode), drawG = d.createBindGroup({ layout: drawL, entries: [{ binding: 0, resource: { buffer: slots } }] });
    const pipe = (entryPoint: string, put = 'put') => d.createRenderPipelineAsync({ label: entryPoint, layout: d.createPipelineLayout({ bindGroupLayouts: [g.shared.layout, drawL] }), vertex: { module: drawM, entryPoint }, fragment: { module: drawM, entryPoint: put, targets: [{ format: 'r32uint' }] }, primitive: { topology: 'triangle-list', cullMode: 'back', frontFace: 'ccw' }, depthStencil: { format: 'depth32float', depthWriteEnabled: true, depthCompare: 'greater' } }), [pull, share, pullCut, shareCut] = await Promise.all([pipe('pull'), pipe('share'), pipe('pull', 'putCut'), pipe('share', 'putCut')]);
    const most = Math.max(...w.objects.map(o => o.most)); let frames = 0;
    return enc => {
      fit(); const ts = (name: string) => g.timer.pass(name), stamp = (name: string) => { const t = ts(name); return t ? { timestampWrites: t } : {}; }, hide = RASTER.hide && !RASTER.freeze && frames++ > 0, many = RASTER.heroes ? heroes : all.length;
      for (let i = 0; i < K.length; i++) { const f = new Float32Array(12), u = new Uint32Array(f.buffer); f[0] = RASTER.tau; f[1] = RASTER.fixed; u[2] = many; u[3] = MOST; f[4] = RASTER.cone ? 1 : 0; f[5] = hide ? 1 : 0; u[6] = Math.max(0, i - 1); u[7] = QUEUE; f[8] = hzbSize[0]; f[9] = hzbSize[1]; f[10] = hzbViews.length; d.queue.writeBuffer(K[i]!, 0, f); }
      const B = bind(), held = RASTER.freeze && frames > 2; /* (held: all that was picked is said again, as one) */ if (held) enc.clearBuffer(n, 108, 16); else enc.clearBuffer(n, 0, 124);
      if (!held) { const p = enc.beginComputePass({ label: 'mesh pick', ...stamp('mesh pick') }); p.setBindGroup(0, g.shared.group()); p.setPipeline(seed!); p.setBindGroup(1, B[0]!); p.dispatchWorkgroups(Math.ceil(most / 64), many); p.setPipeline(round!); for (let i = 0; i < ROUNDS; i++) { p.setBindGroup(1, B[i + 1]!); p.dispatchWorkgroups(GROUPS); } p.end(); }
      const draw = (name: string, phase: 0 | 1) => {
        { const p = enc.beginComputePass({ label: name + ' indices', ...stamp(name + ' indices') }); p.setBindGroup(0, g.shared.group()); p.setBindGroup(1, shareG[phase]!); if (RASTER.share) { p.setPipeline(expand!); p.dispatchWorkgroups(1024); } p.setPipeline(say!); p.dispatchWorkgroups(1); p.end(); }
        const p = enc.beginRenderPass({ label: name, colorAttachments: [{ view: g.view('id'), loadOp: 'load', storeOp: 'store' }], depthStencilAttachment: { view: g.view('depth'), depthLoadOp: 'load', depthStoreOp: 'store' }, ...(stamp(name) as { timestampWrites?: GPURenderPassTimestampWrites }) }); p.setBindGroup(0, g.shared.group()); p.setBindGroup(1, drawG);
        if (RASTER.share) { p.setPipeline(lab.cut < 1e8 ? shareCut! : share!); p.setIndexBuffer(indices, 'uint32'); p.drawIndexedIndirect(args, 16); } else { p.setPipeline(lab.cut < 1e8 ? pullCut! : pull!); p.drawIndirect(args, 0); } p.end();
      };
      draw('mesh draw', 0);
      { const p = enc.beginComputePass({ label: 'mesh depth', ...stamp('mesh depth') }); reduce(p); if (hide) { p.setBindGroup(0, g.shared.group()); p.setPipeline(second!); p.setBindGroup(1, B[0]!); p.dispatchWorkgroups(GROUPS); } p.end(); }
      if (hide) draw('mesh second', 1);
    };
  } };
}
/** the raster alone: for measuring it */
export const rasterOnly = (): Method => ({ name: 'rasteronly', result: 'id', signals() {}, stages: lab => [rasterStage(lab, lab.graph), meshStage(lab, lab.graph)] });
async function compiled(d: GPUDevice, label: string, code: string): Promise<GPUShaderModule> { const m = d.createShaderModule({ label, code }), bad = (await m.getCompilationInfo()).messages.filter(x => x.type === 'error'); if (bad.length) { const lines = code.split('\n'); throw new Error(bad.slice(0, 5).map(x => `${label}:${x.lineNum}:${x.linePos} ${x.message}\n   ${lines[x.lineNum - 1]!.slice(Math.max(0, x.linePos - 80), x.linePos + 120)}`).join('\n')); } return m; }
export async function drawn(lab: Lab): Promise<Drawn> { const u = new Uint32Array(await lab.readBuffer('meshN')); return { slots: u[24]!, triangles: u[25]!, hid: u[26]!, full: u[29]!, coarser: Math.max(1, new Float32Array(u.buffer)[31]!), rounds: [...u.slice(0, 10)] }; }
/** a method with meshes in it: the stage goes in after the raster, and the first hit learns whose ids these are */
export function withMeshes<M extends Method>(m: M): M {
  const base = m.stages.bind(m);
  m.stages = lab => { const out: Stage[] = []; for (const s of base(lab)) { if (s.name === 'first hit' && lab.building.world.meshes) out.push({ ...s, buffers: { ...s.buffers, meshSlots: ['meshSlots', 'array<vec4u>', 'read'] }, code: firstHit(s.code) }); else if (s.name === 'resolve' && lab.building.world.meshes) out.push({ ...s, code: resolved(s.code) }); else out.push(s); if (s.name === 'raster' && lab.building.world.meshes) out.push(meshStage(lab, lab.graph)); } return out; };
  return m;
}
const firstHit = (code: string) => { const at = '  } else if (a != 0u) {\n'; if (code.split(at).length !== 2) throw new Error('first hit has changed'); return code.replace(at, `  } else if ((a & 0x40000000u) != 0u) {
    let r = meshSlots[((a >> 7u) & 0x7fffffu) * 4u + 3u]; h.piece = r.z >> 11u; h.code = MESH_HIT | ((r.z & 2047u) << mrowU(pieces[h.piece].planes).y) | ((r.w >> 8u) << 7u) | (a & 127u); h.t = 0.25 / (textureLoad(depthT, id.xy, 0) * dot(rd, frame.camFwd.xyz));
` + at).replace('textureStore(outSurf, id.xy, packPx(h.t, s));', 'var kept = packPx(h.t, s); if ((kept.w & (1u << 26u)) == 0u) { kept.w |= meshKind(h) << 27u; } textureStore(outSurf, id.xy, kept);'); };
/** what a level lost of its light is taken off there: of the sun, and of all that comes from round about */
const resolved = (code: string) => { const put = (c: string, from: string, to: string) => { if (c.split(from).length !== 2) throw new Error('resolve has changed: ' + from); return c.replace(from, to); }; return put(put(put(code, 'let s = pxSurf(p); let lt =', 'let s = pxSurf(p); var lost = vec2f(1.0); if (!p.leaf && frame.knobs.x >= 0.0) { lost = meshLost(textureLoad(surf, id.xy, 0).w >> 27u, 2.0 * length(frame.camUp.xyz) / frame.size.y * p.t, V); } let lt ='), 'b *= @tune.bounce; let rv', 'b *= @tune.bounce * lost.y; let rv'), 'var sun = frame.sunE.xyz * vz.a', 'var sun = frame.sunE.xyz * lost.x * vz.a'); };
