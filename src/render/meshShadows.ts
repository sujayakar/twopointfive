// Mesh-traced character shadows, GPU half (settings.meshShadows, FrameFlags.MeshShadows; the CPU topology is meshbvh.ts, the walk is meshShadowT /
// meshAnyHitChar in common.wgsl). Every man is skinned from one mesh, so one BVH topology (built once on the CPU over the rest pose) serves all of them, and a
// man's own tree is that one less the parts he does not wear (meshbvh.ts wornTree). Per frame three compute dispatches a listed man: skin the vertices he wears,
// box his leaves (their corners written beside them), refit the rest bottom-up — if a bone of his has moved since his tree was made; every shadow ray that reaches him walks his tree instead of testing his 15 capsules.
//
// ONE storage buffer (`buf`, bound read-only at scene binding 13 as array<vec4u>; floats travel as their bit patterns) holds what the walk reads:
//   [k]                          man k's tree: (N = nodes, where they begin, where his corners begin, T = triangles); zeros = an empty tree (the walk ends at once)
//   [nodes + i·2 + 0]            node i: (aabbMin.xyz as bits, skip)         skip = next node when this one is missed / done
//   [nodes + i·2 + 1]                    (aabbMax.xyz as bits, triStart | triCount << 24)   count 0 = internal (children i+1 and, for the refit alone, WornTree.nodes' right)
//   [corners + t·3 + j]          corner j of triangle t, skinned this frame: (pos.xyz as bits, 0)
// k is the character's index in this frame's capsule list = its skin instance (rust/game_rules/src/tick/game writes both from one list; scene.charBounds[k] culls it,
// capsules[k·capsPerChar] carries its owner id). A slot has room for the whole mesh's tree, whatever the man in it wears.
import { makeBuffer, makeBufferWithData, makeShader, computePipelineAsync } from '../gpu/device';
import { GltfCharacter, MeshData } from '../anim/gltf';
import { JOINT_STRIDE, type CharacterRenderer } from '../anim/characters';
import { MeshBvh, WornTree, buildMeshBvh, wornTree, topologyDigest, LEAF_MAX, LEAF_PAD, MESH_MAX_CHARS } from './meshbvh';

export { MESH_MAX_CHARS };
const NODE_BASE = MESH_MAX_CHARS;    // vec4u elements before the first node (a header a man)
const SLOT = 12;                     // u32 words of a slot's record (`slots`): what is his in the pool and in the buffers
const DUE = MESH_MAX_CHARS * SLOT;   // … and after the records, the slots whose trees are made this frame
const BONES = JOINT_STRIDE * 16;      // words of a man's joint matrices
const REFIT_WG = 256;                // one workgroup of this size refits one character's tree, level by level

const bvhCache = new WeakMap<MeshData, MeshBvh>();   // one build per mesh per process
/** The shared topology for a loaded character mesh (built on first use: ~80 ms for the mannequin). */
export function meshBvhFor(mesh: MeshData): MeshBvh {
  let b = bvhCache.get(mesh);
  if (!b) { b = buildMeshBvh(mesh.positions, mesh.indices); bvhCache.set(mesh, b); }
  return b;
}

// (THE MESH'S OWN SIZES ARE DATA: a tree's counts and where its parts lie are the slot's record. A level whose men are another cast is the SAME module and the same
// programs, and compiles nothing. They are read from `slots` and not from the mesh buffer's own headers because that buffer is written here: a loop bound read from it
// would not be uniform to the compiler, and the refit's barrier must stand in uniform control flow.)
const skinRefitSrc = () => /* wgsl */`
const JOINT_STRIDE: u32 = ${JOINT_STRIDE}u; const LEAF_PAD: f32 = ${LEAF_PAD}; const SLOT: u32 = ${SLOT}u; const DUE: u32 = ${DUE}u;
@group(0) @binding(0) var<storage, read_write> mbuf: array<vec4u>;
@group(0) @binding(1) var<storage, read> rest: array<vec4u>;         // 2 per vertex: (rest pos.xyz bits, joints j0 | j1<<8 | j2<<16 | j3<<24), (weights.xyzw bits)
@group(0) @binding(2) var<storage, read> pool: array<u32>;           // every worn tree's lists (meshbvh.ts WornTree), one after another
@group(0) @binding(3) var<storage, read> jointMats: array<mat4x4f>;  // CharacterRenderer.jointMats: instance k's skinning matrices at k·JOINT_STRIDE (what vsSkin rasterises with)
@group(0) @binding(4) var<storage, read> slots: array<u32>;          // SLOT words a slot: the counts (vertices, leaves, steps), then where in the pool its vertices, corners, nodes, order and steps begin, then where its own skinned vertices, nodes and corners do; from DUE on, the slots to do this frame
@group(0) @binding(5) var<storage, read_write> skinned: array<vec4f>;
fn slotWord(k: u32, w: u32) -> u32 { return slots[k * SLOT + w]; }

// one thread per vertex he wears (gid.y = which of the slots to do): the same linear blend vsSkin does, so the traced mesh is the rasterised one
@compute @workgroup_size(64) fn skin(@builtin(global_invocation_id) gid: vec3u) {
  let k = slots[DUE + gid.y];
  if (gid.x >= slotWord(k, 0u)) { return; }
  let vi = pool[slotWord(k, 3u) + gid.x];
  let r0 = rest[vi * 2u]; let r1 = rest[vi * 2u + 1u];
  let p = vec4f(bitcast<vec3f>(r0.xyz), 1.0); let w = bitcast<vec4f>(r1); let j = r0.w;
  let base = k * JOINT_STRIDE;
  let m = jointMats[base + (j & 255u)] * w.x + jointMats[base + ((j >> 8u) & 255u)] * w.y + jointMats[base + ((j >> 16u) & 255u)] * w.z + jointMats[base + (j >> 24u)] * w.w;
  skinned[slotWord(k, 8u) + gid.x] = vec4f((m * p).xyz, 0.0);
}
fn putNode(k: u32, ni: u32, mn: vec3f, mx: vec3f) {
  let o = slotWord(k, 5u) + ni * 4u; let at = slotWord(k, 9u) + ni * 2u;
  mbuf[at] = vec4u(bitcast<vec3u>(mn), pool[o]);
  mbuf[at + 1u] = vec4u(bitcast<vec3u>(mx), pool[o + 1u] | (pool[o + 2u] << 24u));
}
// one thread per leaf: its corners, where the walk reads them, and their box (+ LEAF_PAD)
@compute @workgroup_size(64) fn leaves(@builtin(global_invocation_id) gid: vec3u) {
  let k = slots[DUE + gid.y];
  if (gid.x >= slotWord(k, 1u)) { return; }
  let ni = pool[slotWord(k, 6u) + gid.x]; let o = slotWord(k, 5u) + ni * 4u;
  let c0 = pool[o + 1u] * 3u; let c1 = c0 + pool[o + 2u] * 3u;
  var mn = vec3f(1e30); var mx = vec3f(-1e30);
  for (var c = c0; c < c1; c++) { let v = skinned[slotWord(k, 8u) + pool[slotWord(k, 4u) + c]].xyz; mbuf[slotWord(k, 10u) + c] = vec4u(bitcast<vec3u>(v), 0u); mn = min(mn, v); mx = max(mx, v); }
  putNode(k, ni, mn - vec3f(LEAF_PAD), mx + vec3f(LEAF_PAD));
}
// one workgroup per character to do (wid.x): the steps over the leaves in turn, a storage barrier between two — a node's children are of earlier steps, so they are final by its own
@compute @workgroup_size(${REFIT_WG}) fn refit(@builtin(workgroup_id) wid: vec3u, @builtin(local_invocation_index) lid: u32) {
  let k = slots[DUE + wid.x]; let steps = slotWord(k, 2u); let nodes = slotWord(k, 5u); let order = slotWord(k, 6u); let begin = slotWord(k, 7u); let nb = slotWord(k, 9u);
  for (var s = 1u; s < steps; s++) {
    let e = pool[begin + s + 1u];
    for (var q = pool[begin + s] + lid; q < e; q += ${REFIT_WG}u) {
      let ni = pool[order + q];
      let l = nb + (ni + 1u) * 2u; let r = nb + pool[nodes + ni * 4u + 3u] * 2u;
      putNode(k, ni, min(bitcast<vec3f>(mbuf[l].xyz), bitcast<vec3f>(mbuf[r].xyz)), max(bitcast<vec3f>(mbuf[l + 1u].xyz), bitcast<vec3f>(mbuf[r + 1u].xyz)));
    }
    storageBarrier();
  }
}
`;

/** a worn tree and where its lists begin in the pool */
interface Kept { tree: WornTree; verts: number; corners: number; nodes: number; order: number; steps: number }

export class MeshShadows {
  readonly bvh: MeshBvh;
  /** topology checksum */
  readonly digest: string;
  /** the one buffer the scene group binds (layout above) */
  readonly buf: GPUBuffer;
  readonly bytes: number;
  readonly triBase: number;
  private pipes: GPUComputePipeline[] = []; private bg: GPUBindGroup | null = null;
  private mod: GPUShaderModule; private pl: GPUPipelineLayout; private layout: GPUBindGroupLayout;
  private restBuf: GPUBuffer; private slotBuf: GPUBuffer; private skinnedBuf: GPUBuffer; private poolBuf: GPUBuffer | null = null;
  private pool = new Uint32Array(0); private poolSent = 0; private kept = new Map<number, Kept>();
  private slots = new Uint32Array(DUE + MESH_MAX_CHARS); private madeOf = new Uint32Array(MESH_MAX_CHARS * BONES); private heads = new Uint32Array(MESH_MAX_CHARS * 4); private slotMask: number[] = [];
  private mesh: MeshData;
  /** characters skinned + refit by the last encode() (the HUD / panel read it), and the nodes and triangles of all that are listed */
  lastChars = 0; lastNodes = 0; lastTris = 0;

  constructor(private device: GPUDevice, ch: GltfCharacter, private chars: CharacterRenderer) {
    const mesh = this.mesh = ch.mesh; const bvh = this.bvh = meshBvhFor(mesh); this.digest = topologyDigest(bvh);
    const N = bvh.numNodes, T = bvh.numTris, V = mesh.vertexCount;
    if (ch.skin.joints.length > 256 || JOINT_STRIDE > 256) throw new Error('mesh shadows: joint indices are packed as bytes');
    if (T >= 1 << 24 || LEAF_MAX > 255) throw new Error('mesh shadows: triStart | count << 24 packing');
    this.triBase = NODE_BASE + MESH_MAX_CHARS * N * 2;
    this.bytes = (this.triBase + MESH_MAX_CHARS * T * 3) * 16;
    const S = GPUBufferUsage.STORAGE;
    this.buf = makeBuffer(device, this.bytes, S | GPUBufferUsage.COPY_DST, 'meshShadows');   // (zeros: every slot an empty tree)
    // static inputs: rest vertices (pos bits + packed joints, weight bits)
    const rest = new Uint32Array(V * 8); { const f = new Float32Array(rest.buffer); const jn = mesh.joints;
      for (let v = 0; v < V; v++) { f[v * 8] = mesh.positions[v * 3]; f[v * 8 + 1] = mesh.positions[v * 3 + 1]; f[v * 8 + 2] = mesh.positions[v * 3 + 2]; rest[v * 8 + 3] = (jn[v * 4] | (jn[v * 4 + 1] << 8) | (jn[v * 4 + 2] << 16) | (jn[v * 4 + 3] << 24)) >>> 0; f.set(mesh.weights.subarray(v * 4, v * 4 + 4), v * 8 + 4); } }
    this.restBuf = makeBufferWithData(device, rest, S, 'meshRest');
    this.slotBuf = makeBuffer(device, this.slots.byteLength, S | GPUBufferUsage.COPY_DST, 'meshSlots');
    this.skinnedBuf = makeBuffer(device, MESH_MAX_CHARS * V * 16, S, 'meshSkinned');
    const CS = GPUShaderStage.COMPUTE, ro: GPUBufferBindingLayout = { type: 'read-only-storage' }, rw: GPUBufferBindingLayout = { type: 'storage' };
    this.layout = device.createBindGroupLayout({ label: 'meshShadows', entries: [rw, ro, ro, ro, ro, rw].map((buffer, binding) => ({ binding, visibility: CS, buffer })) });
    this.mod = makeShader(device, skinRefitSrc(), 'meshShadows');
    this.pl = device.createPipelineLayout({ bindGroupLayouts: [this.layout] });   // (the three programs: init(), awaited by the page before the first frame)
  }

  /** the programs, compiled off the GPU process's main thread (gpu/device.ts computePipelineAsync) — the page awaits it before the first frame */
  async init(): Promise<void> {
    if (this.pipes.length) return;
    this.pipes = await Promise.all(['skin', 'leaves', 'refit'].map(entryPoint => computePipelineAsync(this.device, { label: `mesh_${entryPoint}`, layout: this.pl, compute: { module: this.mod, entryPoint } })));
  }

  /** WGSL source of the passes (for a compile check through the page's __wgsl with no prelude) */
  source(): string { return skinRefitSrc(); }

  /** the tree of what `mask` says is worn (0 = the mannequin: everything), its lists in the pool (made when the first man wears it) */
  private treeOf(mask: number): Kept {
    let k = this.kept.get(mask);
    if (!k) {
      const part = this.mesh.part, tree = wornTree(this.bvh, this.mesh.vertexCount, part && mask ? v => ((mask >>> part[v]) & 1) === 1 : () => true), lists = [tree.verts, tree.corners, tree.nodes, tree.order, tree.steps];
      const was = this.pool, at: number[] = []; let end = was.length; for (const l of lists) { at.push(end); end += l.length; }
      this.pool = new Uint32Array(end); this.pool.set(was); lists.forEach((l, i) => this.pool.set(l, at[i]));
      this.kept.set(mask, k = { tree, verts: at[0], corners: at[1], nodes: at[2], order: at[3], steps: at[4] });
    }
    return k;
  }

  /** Skin + refit those of the first `numChars` characters' trees whose men have moved a bone since (their joint matrices must already be in jointMats for this frame).
   *  Encode BEFORE any pass that traces (the cascades come first in engine.render). */
  encode(enc: GPUCommandEncoder, numChars: number, ts?: GPUComputePassTimestampWrites) {
    const n = Math.min(Math.max(0, numChars | 0), MESH_MAX_CHARS), q = this.device.queue; let due = 0, verts = 0, leaves = 0, told = false; this.lastNodes = this.lastTris = 0;
    for (let k = 0; k < n; k++) {
      const mask = this.chars.partsOf(k), K = this.treeOf(mask), t = K.tree, dressed = this.slotMask[k] === mask; this.lastNodes += t.numNodes; this.lastTris += t.numTris;
      if (!this.moved(k) && dressed) continue;
      if (this.slots[DUE + due] !== k) { this.slots[DUE + due] = k; told = true; }
      due++; verts = Math.max(verts, t.verts.length); leaves = Math.max(leaves, t.leaves);
      if (dressed) continue;
      this.slotMask[k] = mask; told = true;
      const nodeAt = NODE_BASE + k * this.bvh.numNodes * 2, triAt = this.triBase + k * this.bvh.numTris * 3;
      this.slots.set([t.verts.length, t.leaves, Math.max(0, t.steps.length - 1), K.verts, K.corners, K.nodes, K.order, K.steps, k * this.mesh.vertexCount, nodeAt, triAt, 0], k * SLOT);
      this.heads.set([t.numNodes, nodeAt, triAt, t.numTris], k * 4);
    }
    this.lastChars = due; if (!due) return;
    if (told) { q.writeBuffer(this.slotBuf, 0, this.slots); q.writeBuffer(this.buf, 0, this.heads); }
    if (this.poolSent !== this.pool.length) {
      const words = this.pool;
      if (!this.poolBuf || this.poolBuf.size < words.byteLength) { this.poolBuf?.destroy(); this.poolBuf = makeBuffer(this.device, Math.ceil(words.byteLength * 1.5 / 16) * 16, GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST, 'meshPool'); this.bg = null; }
      q.writeBuffer(this.poolBuf, 0, words); this.poolSent = this.pool.length;
    }
    this.bg ??= this.device.createBindGroup({ label: 'meshShadowsBG', layout: this.layout, entries: [this.buf, this.restBuf, this.poolBuf!, this.chars.jointMats, this.slotBuf, this.skinnedBuf].map((buffer, binding) => ({ binding, resource: { buffer } })) });
    const p = enc.beginComputePass({ label: 'meshShadows', timestampWrites: ts });
    p.setBindGroup(0, this.bg);
    p.setPipeline(this.pipes[0]); p.dispatchWorkgroups(Math.ceil(verts / 64), due);
    p.setPipeline(this.pipes[1]); p.dispatchWorkgroups(Math.ceil(leaves / 64), due);
    p.setPipeline(this.pipes[2]); p.dispatchWorkgroups(due);
    p.end();
  }
  /** whether a bone of the man in slot k is elsewhere than his slot's tree was made from, which is then kept */
  private moved(k: number): boolean {
    const now = this.chars.bones, was = this.madeOf, o = k * BONES; if (!now) return true;
    const at = this.chars.bonesAt + o; let i = 0; while (i < BONES && now[at + i] === was[o + i]) i++;
    if (i === BONES) return false;
    for (; i < BONES; i++) was[o + i] = now[at + i];
    return true;
  }
}
