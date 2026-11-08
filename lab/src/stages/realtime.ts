// THE REAL-TIME METHOD, as parts with fixed signatures. Swap one for another to study it:
//   cache     how light on surfaces is kept       (cacheRead / cacheFound, and stages that fill it)
//   denoise   signal → quieter signal
//   upscale   color at render size → picture at output size
import type { Method, Lab } from '../lab';
import type { Stage, Size, Graph } from '../graph';
import type { Piece } from '../scene/world';
import { F } from '../scene/world';
import { type V3, add, apply } from '../math';
import signals from '../wgsl/signals.wgsl' with { type: 'text' };
import cacheLib from '../wgsl/cache.wgsl' with { type: 'text' };
import surf from './surf.wgsl' with { type: 'text' };
import lamps from './lamps.wgsl' with { type: 'text' };
import bounceLib from '../wgsl/bounce.wgsl' with { type: 'text' };
import waterpx from './waterpx.wgsl' with { type: 'text' };
import gather from './gather.wgsl' with { type: 'text' };
import mirror from './mirror.wgsl' with { type: 'text' };
import temporal from './temporal.wgsl' with { type: 'text' };
import blur from './blur.wgsl' with { type: 'text' };
import air from './air.wgsl' with { type: 'text' };
import resolve from './resolve.wgsl' with { type: 'text' };
import taau from './taau.wgsl' with { type: 'text' };
import { glowStages } from './glow';
import { exposeStages } from './expose';
import { rasterStage } from './raster';
import claim from './cacheclaim.wgsl' with { type: 'text' };
import warm from './cachewarm.wgsl' with { type: 'text' };
import update from './cacheupdate.wgsl' with { type: 'text' };

const SLOTS = 262144, HITS = 1 << 20;
export const CHANGES: Record<string, [string, string, 'uniform']> = { changes: ['changes', 'Changes', 'uniform'] };
export interface CachePart { name: string; lib: string; bind: Stage['buffers']; stages: Stage[]; init(lab: Lab): Promise<void>; /** more rounds this frame: the light has changed everywhere */ rush?: number }
export interface DenoisePart { name: string; stages(o: { signal: string; div: 1 | 2; cap: number; rounds: number; narrow?: boolean; fractions?: boolean }, g: Graph): { stages: Stage[]; result: string } }
export interface UpscalePart { name: string; stages(g: Graph): { stages: Stage[]; result: string } }

/** light on surfaces, in a table */
export function hashCache(): CachePart {
  const bind: Stage['buffers'] = { keys: ['keys', 'array<atomic<u32>>', 'read_write'], cand: ['cand', 'array<atomic<u32>>', 'read_write'], cells: ['cells', 'array<vec4f>', 'read'] };
  const groups = { groups: (): [number, number, number] => [SLOTS / 64 / 8, 8, 1] }, lin = (t: string) => t.replaceAll('let s = id.x;', `let s = id.x + id.y * ${SLOTS / 8}u;`);
  const claimS: Stage = { name: 'cache claim', over: groups, buffers: { ...bind, norms: ['norms', 'array<u32>', 'read_write'], hits: ['hits', 'array<vec4u>', 'read'], fresh: ['fresh', 'array<vec4f>', 'read_write'] }, code: cacheLib + lin(claim) };
  const keep: Stage = { name: 'cache keep', over: groups, code: '', async make() { return (enc, g) => { if (keep.when && !keep.when()) return; enc.copyBufferToBuffer(g.buffers.get('fresh')!, 0, g.buffers.get('cells')!, 0, SLOTS * 32); }; } };
  const updateS: Stage = { name: 'cache', over: { groups: () => [SLOTS / 256, 1, 1] }, buffers: { ...bind, norms: ['norms', 'array<u32>', 'read'], fresh: ['fresh', 'array<vec4f>', 'read_write'], ...CHANGES }, code: signals + cacheLib + bounceLib + update };
  let at: [number, number] = [0, 0]; const warmS: Stage = { name: 'cache warm', over: { groups: () => [Math.ceil((at[1] - at[0]) / 64), 1, 1] }, buffers: { ...bind, hits: ['hits', 'array<vec4u>', 'read'], warmAt: ['warmAt', 'vec4u', 'uniform'] }, code: cacheLib + warm };
  const again = (k: number): Stage => ({ ...updateS, name: 'cache', when: () => (part.rush ?? 0) > k, code: updateS.code.replace('rngKey(key, 91u)', `rngKey(key, ${92 + k}u)`) });
  const part: CachePart = {
    name: 'hash', lib: cacheLib, bind, stages: [claimS, updateS, keep, ...[0, 1, 2].flatMap(k => [again(k), { ...keep, when: () => (part.rush ?? 0) > k }])],
    async init(lab) {
      lab.buffer('keys', SLOTS * 4); lab.buffer('cells', SLOTS * 32, GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_SRC | GPUBufferUsage.COPY_DST); lab.buffer('fresh', SLOTS * 32, GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_SRC | GPUBufferUsage.COPY_DST); lab.buffer('norms', SLOTS * 4); lab.buffer('hits', HITS * 16); lab.uniform('warmAt', new Uint32Array(4));
      const c = lab.buffer('cand', SLOTS * 4); lab.gpu.device.queue.writeBuffer(c, 0, new Uint32Array(SLOTS).fill(0xffffffff));
      await lab.graph.build([claimS, warmS, keep]);
      const pts = facePoints(lab), chunk = 49152; // 48 k at a time
      for (let a = 0; a < pts.length / 4; a += chunk) {
        const b = Math.min(pts.length / 4, a + chunk); at = [a, b]; lab.gpu.device.queue.writeBuffer(lab.graph.buffers.get('hits')!, 0, pts.buffer as ArrayBuffer, a * 16, (b - a) * 16); lab.uniform('warmAt', new Uint32Array([a, b, 0, 0]));
        const e = lab.gpu.device.createCommandEncoder(); lab.graph.run(e, [warmS, claimS, keep]); lab.gpu.device.queue.submit([e.finish()]);
      }
      (lab as never as { warmed: number }).warmed = pts.length / 4;
    },
  };
  return part;
}
/** points over every standing face a ray could meet, a hand's width apart: where light will be kept */
function facePoints(lab: Lab): Float32Array {
  const out: number[] = [], ps = lab.building.world.pieces, grid = new Map<string, Piece[]>(), key = (x: number, y: number, z: number) => `${Math.floor(x / 2)},${Math.floor(y / 2)},${Math.floor(z / 2)}`;
  const solid = ps.filter(p => !(p.flags & (F.GLASS | F.WATER | F.FAR)) && !p.quad && Math.max(...p.half) < 40);
  for (const p of solid) { const e = [0, 1, 2].map(k => Math.abs(p.rot[0][k]!) * p.half[0] + Math.abs(p.rot[1][k]!) * p.half[1] + Math.abs(p.rot[2][k]!) * p.half[2]); for (let x = Math.floor((p.at[0] - e[0]!) / 2); x <= Math.floor((p.at[0] + e[0]!) / 2); x++) for (let y = Math.floor((p.at[1] - e[1]!) / 2); y <= Math.floor((p.at[1] + e[1]!) / 2); y++) for (let z = Math.floor((p.at[2] - e[2]!) / 2); z <= Math.floor((p.at[2] + e[2]!) / 2); z++) { const k = `${x},${y},${z}`; let l = grid.get(k); if (!l) grid.set(k, l = []); l.push(p); } }
  const inside = (q: V3, not: Piece) => (grid.get(key(...q)) ?? []).some(p => { if (p === not) return false; const d: V3 = [q[0] - p.at[0], q[1] - p.at[1], q[2] - p.at[2]]; for (let k = 0; k < 3; k++) { const r = p.rot[k]!; if (Math.abs(d[0] * r[0] + d[1] * r[1] + d[2] * r[2]) > p.half[k]! + 0.002) return false; } return true; });
  const nrm = new Float32Array(1), nu = new Uint32Array(nrm.buffer), oct = (n: V3) => { const s = Math.abs(n[0]) + Math.abs(n[1]) + Math.abs(n[2]); let x = n[0] / s, y = n[2] / s; if (n[1] < 0) { const a = (1 - Math.abs(y)) * Math.sign(x || 1), b = (1 - Math.abs(x)) * Math.sign(y || 1); x = a; y = b; } const q = (v: number) => Math.floor((v * 0.5 + 0.5) * 65535 + 0.5) & 0xffff; nu[0] = (q(x) | (q(y) << 16)) >>> 0; return nrm[0]!; };
  const bits: number[] = [], step = 0.30;
  for (const p of solid) {
    if (p.planes || p.at[1] + p.half[1] < -0.6 || Math.max(...p.half) < 0.12) continue;
    for (let a = 0; a < 3; a++) for (const s of [-1, 1]) {
      const u = (a + 1) % 3, v = (a + 2) % 3, hu = p.half[u]!, hv = p.half[v]!; if (hu * hv < 0.004) continue; const n = [p.rot[a]![0] * s, p.rot[a]![1] * s, p.rot[a]![2] * s] as V3, nuN = Math.max(1, Math.round(2 * hu / step)), nvN = Math.max(1, Math.round(2 * hv / step));
      for (let i = 0; i < nuN; i++) for (let j = 0; j < nvN; j++) { const l: V3 = [0, 0, 0]; l[a] = s * p.half[a]!; l[u] = -hu + 2 * hu * (i + 0.5) / nuN; l[v] = -hv + 2 * hv * (j + 0.5) / nvN; const x = add(p.at, apply(p.rot, l)); if (x[1] < -0.02 || inside(add(x, [n[0] * 0.03, n[1] * 0.03, n[2] * 0.03]), p)) continue; oct(n); out.push(x[0], x[1], x[2], 0); bits.push(nu[0]!); }
    }
  }
  const f = new Float32Array(out), u = new Uint32Array(f.buffer); bits.forEach((b, i) => { u[i * 4 + 3] = b; }); return f;
}
/** history, then a few rounds of blur that know their surface */
export const svgf: DenoisePart = {
  name: 'svgf',
  stages({ signal, div, cap, rounds, narrow, fractions }, g) {
    const size: Size = div === 2 ? 'half' : 'render', N = narrow ? 1 : 0; g.signal(signal + 'H', 'rgba16float', size, true).signal(signal + 'M', 'rgba16float', size, true).signal(signal + 'A', 'rgba16float', size).signal(signal + 'B', 'rgba16float', size);
    const stages: Stage[] = [{ name: signal + ' history', over: size, reads: { [signal]: 'cur', [signal + 'H@prev']: 'hist', [signal + 'M@prev']: 'momPrev', surf: 'surf', 'surf@prev': 'surfPrev', aux: 'auxT', front: 'frontT' }, writes: { outH: signal + 'H', outM: signal + 'M' }, buffers: CHANGES, consts: { DIV: div, CAP: cap, SPREAD: narrow ? 1.6 : 1.25, NARROW: N, KIND: fractions ? 1 : 0 }, code: signals + temporal }];
    let from = signal + 'H'; for (let i = 0; i < rounds; i++) { const to = signal + (i % 2 ? 'B' : 'A'); stages.push({ name: signal + ' blur', over: size, reads: { [from]: 'src', [signal + 'M']: 'mom', surf: 'surf', front: 'frontT' }, writes: { outB: to }, buffers: CHANGES, consts: { DIV: div, STRIDE: 1 << i, SHARE: narrow ? 2.5 : 3.0, NARROW: N, KIND: fractions ? 1 : 0 }, code: signals + blur }); from = to; }
    return { stages, result: from };
  },
};
/** no memory of light at all: what a bounce hits is lit by nothing. Shows what the cache is worth */
export function noCache(): CachePart { const lib = 'struct Cached { E: vec3f, sun: f32, n: f32 };\nconst EVERY_WAY = 0x80010003u;\nfn cacheRead(x: vec3f, n: vec3f, j: vec2f) -> Cached { var c: Cached; return c; }\nfn cacheSmooth(x: vec3f, n: vec3f) -> Cached { var c: Cached; return c; }\nfn cacheKey(x: vec3f, n: vec3f, level: u32) -> u32 { return 1u; }\nfn cacheFind(key: u32) -> u32 { return 1u; }\nfn cacheFound(key: u32, who: u32) {}\n'; return { name: 'none', lib: lib + 'fn cacheWants(x: vec3f, n: vec3f, moves: bool) -> u32 { return 0u; }\n', bind: {}, stages: [], async init(lab) { lab.buffer('hits', HITS * 16); } }; }
export const bilinear: UpscalePart = { name: 'bilinear', stages(g) { g.signal('picture', 'rgba16float', 'output', true); return { result: 'picture', stages: [{ name: 'upscale', over: 'output', reads: { color: 'color' }, writes: { outT: 'picture' }, buffers: { taauK: ['taauK', 'vec4f', 'uniform'] }, code: '@compute @workgroup_size(8, 8) fn main(@builtin(global_invocation_id) id: vec3u) { let o = frame.size.zw; if (f32(id.x) >= o.x || f32(id.y) >= o.y) { return; } textureStore(outT, id.xy, textureSampleLevel(color, linSamp, (vec2f(id.xy) + 0.5) / o, 0.0)); }' }] }; } };
export const noDenoise: DenoisePart = { name: 'none', stages: ({ signal }) => ({ stages: [], result: signal }) };
export const taauUp: UpscalePart = { name: 'taau', stages(g) { g.signal('picture', 'rgba16float', 'output', true); return { result: 'picture', stages: [{ name: 'upscale', over: 'output', reads: { color: 'color', 'picture@prev': 'hist', aux: 'auxT' }, writes: { outT: 'picture' }, buffers: { taauK: ['taauK', 'vec4f', 'uniform'] }, code: taau }] }; } };

/** `edit` changes a stage's text before it is built: for asking what a part of it costs */
export function realtime(o: { name?: string; cache?: CachePart; denoise?: DenoisePart; upscale?: UpscalePart; edit?: (s: Stage) => Stage } = {}): Method & { glow: string } {
  const cache = o.cache ?? hashCache(), dn = o.denoise ?? svgf, up = o.upscale ?? taauUp; let last = '', lastHour = -1, quiet = 0, fresh = true, prevOn = new Float32Array(64);
  const m: Method & { glow: string; warm?: Promise<void> } = {
    name: o.name ?? 'realtime', result: 'picture', glow: 'rt',
    signals(g, lab) {
      g.signal('surf', 'rgba32uint', 'render', true).signal('aux', 'rgba16float', 'render').signal('emit', 'rgba16float', 'render').signal('front', 'rgba32uint', 'render');
      g.signal('vis', 'rgba16float', 'render').signal('lampU', 'rgba16float', 'render').signal('bounce', 'rgba16float', 'half').signal('mirror', 'rgba16float', 'half').signal('air', 'rgba16float', 'half').signal('airH', 'rgba16float', 'half', true).signal('airM', 'rgba16float', 'half', true).signal('color', 'rgba16float', 'render');
      lab.buffer('past', Math.max(1, lab.building.world.moving.length) * 64); lab.buffer('pastLeaves', Math.max(1, lab.building.world.instances.length) * 48); lab.uniform('changes', new Float32Array(36)); lab.uniform('taauK', new Float32Array(4)); const pl = new Float32Array(72), ps = lab.building.world.plumes.slice(0, 8); ps.forEach((p, i) => { pl.set([...p.at, p.r], i * 4); pl.set([p.h, p.k, 0, 0], 36 + i * 4); }); pl[32] = ps.length; lab.uniform('plumes', pl);
    },
    stages(lab) {
      const g = lab.graph, c = { ...cache.bind, ...CHANGES };
      const raster = rasterStage(lab, g);
      const S = (name: string, code: string, over: Size, reads: Record<string, string>, writes: Record<string, string>, buffers: Stage['buffers'] = {}, consts?: Record<string, number>): Stage => ({ name, code: signals + code, over, reads, writes, buffers: { ...CHANGES, ...buffers }, ...(consts ? { consts } : {}) });
      const dVis = dn.stages({ signal: 'vis', div: 1, cap: 18, rounds: 3, fractions: true }, g), dB = dn.stages({ signal: 'bounce', div: 2, cap: 36, rounds: 4 }, g), dM = dn.stages({ signal: 'mirror', div: 2, cap: 26, rounds: 3, narrow: true }, g), u = up.stages(g); m.result = u.result;
      const glows = [...glowStages(g, u.result, 'rt'), ...exposeStages(lab, g, u.result, 'rtGlow2', 'rt')];
      return [
        raster,
        S('first hit', surf, 'render', { id: 'idT', idClear: 'idC', depth: 'depthT' }, { outSurf: 'surf', outAux: 'aux', outEmit: 'emit', outFront: 'front' }, { past: ['past', 'array<vec4f>', 'read'], pastLeaves: ['pastLeaves', 'array<vec4f>', 'read'] }),
        S('water', waterpx, 'render', { idClear: 'idC', depth: 'depthT' }, { outSurf: 'surf', outAux: 'aux', outEmit: 'emit', outFront: 'front' }),
        ...cache.stages,
        S('lamps', lamps, 'render', { surf: 'surf', front: 'frontT', sunLeaves: 'sunLeaves' }, { outVis: 'vis', outU: 'lampU' }),
        S('gather', cache.lib + bounceLib + gather, 'half', { surf: 'surf', front: 'frontT' }, { outE: 'bounce' }, { ...c, hits: ['hits', 'array<vec4u>', 'read_write'] }, { HITS }),
        S('mirror', cache.lib + bounceLib + mirror, 'half', { surf: 'surf', front: 'frontT' }, { outM: 'mirror' }, c, { DIV: 2 }),
        ...dVis.stages, ...dB.stages, ...dM.stages,
        S('air', cache.lib + air, 'half', { surf: 'surf' }, { outA: 'air' }, { ...c, plumes: ['plumes', 'array<vec4f, 18>', 'uniform'] }),
        S('air history', temporal, 'half', { air: 'cur', 'airH@prev': 'hist', 'airM@prev': 'momPrev', surf: 'surf', 'surf@prev': 'surfPrev', aux: 'auxT', front: 'frontT' }, { outH: 'airH', outM: 'airM' }, {}, { DIV: 2, CAP: 16, SPREAD: 1.0, NARROW: 0, KIND: 0 }),
        S('resolve', resolve, 'render', { surf: 'surf', emit: 'emit', front: 'frontT', [dVis.result]: 'vis', lampU: 'lampU', [dB.result]: 'bounce', [dM.result]: 'mirror', airH: 'air' }, { outC: 'color' }),
        ...u.stages, ...glows,
      ].map(s => o.edit ? o.edit(s) : s);
    },
    frame(lab) {
      // where each moving piece was; and what changed
      const mv = lab.building.world.moving, f = new Float32Array(mv.length * 16); for (const p of mv) { const s = lab.slots.get(p); if (s === undefined) continue; const q = lab.prevAt.get(p) ?? p, o = (s - lab.scene.piece0[2]) * 16; f.set(q.at, o); f.set(q.rot[0], o + 4); f.set(q.rot[1], o + 8); f.set(q.rot[2], o + 12); } if (mv.length) lab.gpu.device.queue.writeBuffer(lab.graph.buffers.get('past')!, 0, f); lab.gpu.device.queue.writeBuffer(lab.graph.buffers.get('pastLeaves')!, 0, lab.scene.instWas as Float32Array<ArrayBuffer>);
      const ch = new Float32Array(36); let n = 0; for (const d of lab.building.doors) if (Math.abs(d.want - d.open) > 1e-3 && n < 8) { ch.set([d.hinge[0], d.hinge[1] + 1, d.hinge[2], 7.5], n * 4); n++; }
      lab.building.world.lamps.forEach((l, i) => { if (Math.abs(lab.scene.lampOn[i]! - prevOn[i]!) > 0.02 && l.kind !== 'spot' && n < 8) { ch.set([...l.at, 9], n * 4); n++; } }); prevOn = lab.scene.lampOn.slice();
      const drift = lastHour < 0 ? 0 : Math.abs(lab.hour - lastHour), key = lab.lampsMode; lastHour = lab.hour; if (key !== last || drift > 0.02) { last = key; quiet = 40; if (drift > 0.1) ch[33] = 1; } quiet = Math.max(0, quiet - 1); ch[32] = 1 - Math.max(0.85 * Math.min(1, quiet / 24), Math.min(0.6, drift * 110)); lab.uniform('changes', ch);
      lab.uniform('taauK', new Float32Array([0.25 * (lab.exposure || 1), fresh ? 1 : 0, 0.2 + 0.8 * ch[32]!, 0])); fresh = false; cache.rush = quiet > 0 ? 3 : 0;
    },
    reset() { fresh = true; },
  };
  (m as { prepare?: (lab: Lab) => Promise<void> }).prepare = async lab => { await cache.init(lab); };
  return m;
}
