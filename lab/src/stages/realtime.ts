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
import { sightStages, sightCode } from './sight';
import { torchStages, torchCode, torchView, torchLeaves } from './torches';
import airlight from './airlight.wgsl' with { type: 'text' };
import haze from './haze.wgsl' with { type: 'text' };
import type { Place } from '../scene/place';
import cellsCode from './cells.wgsl' with { type: 'text' };
import smokeMean from './smokemean.wgsl' with { type: 'text' };
import { lightRow, smokeRow, SMOKE_ROWS } from '../scene/blocks';
import gather from './gather.wgsl' with { type: 'text' };
import mirror from './mirror.wgsl' with { type: 'text' };
import temporal from './temporal.wgsl' with { type: 'text' };
import blur from './blur.wgsl' with { type: 'text' };
import air from './air.wgsl' with { type: 'text' };
import airblur from './airblur.wgsl' with { type: 'text' };
import tint from './tint.wgsl' with { type: 'text' };
import resolve from './resolve.wgsl' with { type: 'text' };
import { looseStages, LOOSE } from './loose';
import { rainStages, rainKnobs } from './raindraw';
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
/** the sum, whole (the teaching panel swaps this for a mask) */
export const WHOLE = 'fn part(k: u32) -> f32 { return 1.0; }\n';
export interface DenoisePart { name: string; stages(o: { signal: string; div: 1 | 2; cap: number; rounds: number; narrow?: boolean; fractions?: boolean; /** a signal that goes with it, by the same weights */ second?: string; /** of a mirror: 0 the surface's own, 1 the clear layer's */ layer?: number }, g: Graph): { stages: Stage[]; result: string; second?: string } }
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
    if (p.planes || p.curves || p.at[1] + p.half[1] < -0.6 || Math.max(...p.half) < 0.12) continue;
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
  stages({ signal, second, div, cap, rounds, narrow, fractions, layer }, g) {
    const size: Size = div === 2 ? 'half' : 'render', N = narrow ? 1 : 0, two = (code: string) => code.replace(/^\s*\/\/2 /gm, second ? '  ' : '  // '), S2 = second ?? ''; if (second) g.signal(S2 + 'H', 'rgba16float', size, true).signal(S2 + 'A', 'rgba16float', size).signal(S2 + 'B', 'rgba16float', size); g.signal(signal + 'H', 'rgba16float', size, true).signal(signal + 'M', 'rgba16float', size, true).signal(signal + 'A', 'rgba16float', size).signal(signal + 'B', 'rgba16float', size);
    const stages: Stage[] = [{ name: signal + ' history', over: size, reads: { [signal]: 'cur', [signal + 'H@prev']: 'hist', [signal + 'M@prev']: 'momPrev', surf: 'surf', 'surf@prev': 'surfPrev', aux: 'auxT', front: 'frontT', ...(second ? { [S2]: 'cur2', [S2 + 'H@prev']: 'hist2' } : {}) }, writes: { outH: signal + 'H', outM: signal + 'M', ...(second ? { outH2: S2 + 'H' } : {}) }, buffers: CHANGES, consts: { DIV: div, CAP: cap, SPREAD: narrow ? 1.6 : 1.25, NARROW: N, KIND: fractions ? 1 : 0, LAYER: layer ?? 0, AIR: 0 }, code: signals + 'fn airAt2(p: vec2u) -> vec2f { return vec2f(0.0); }\nfn airWas(p: vec2u) -> vec2f { return vec2f(0.0); }\n' + two(temporal).replace('CAP_NOW', fractions ? '@tune.visFrames' : narrow ? '@tune.mirrorFrames' : '@tune.bounceFrames') }];
    let from = signal + 'H', from2 = S2 + 'H'; for (let i = 0; i < rounds; i++) { const to = signal + (i % 2 ? 'B' : 'A'), to2 = S2 + (i % 2 ? 'B' : 'A'); stages.push({ name: signal + ' blur', over: size, reads: { [from]: 'src', [signal + 'M']: 'mom', surf: 'surf', front: 'frontT', ...(fractions ? { lampU: 'lampU' } : {}), ...(second ? { [from2]: 'src2' } : {}) }, writes: { outB: to, ...(second ? { outB2: to2 } : {}) }, buffers: CHANGES, consts: { DIV: div, STRIDE: 1 << i, SHARE: narrow ? 2.5 : 3.0, NARROW: N, KIND: fractions ? 1 : 0, LAYER: layer ?? 0 }, code: signals + (fractions ? 'fn lightOf(q: vec2i) -> vec4f { let t = textureLoad(lampU, q, 0); let u = lightAll(t); let f = lightFirst(t); let a = lum(u.rgb) + 1e-4; firstNow = f.a; return vec4f(a, u.a + 1e-4, a + lum(f.rgb), 1.0); }\nvar<private> firstNow = 0.0;\n' : 'fn lightOf(q: vec2i) -> vec4f { return vec4f(1.0); }\nvar<private> firstNow = 0.0;\n') + two(blur) }); from = to; from2 = to2; }
    return { stages, result: from, ...(second ? { second: from2 } : {}) };
  },
};
/** no memory of light at all: what a bounce hits is lit by nothing. Shows what the cache is worth */
export function noCache(): CachePart { const lib = 'const CELL = 0.32;\nstruct Cached { E: vec3f, sun: f32, n: f32 };\nconst EVERY_WAY = 0x80010003u;\nfn cacheRead(x: vec3f, n: vec3f, j: vec2f) -> Cached { var c: Cached; return c; }\nfn cacheSmooth(x: vec3f, n: vec3f) -> Cached { var c: Cached; return c; }\nfn cacheKeyOf(x: vec3f, n: vec3f, level: u32, again: u32) -> u32 { return 1u; }\nfn cacheFind(key: u32) -> u32 { return 1u; }\nfn cacheFound(key: u32, who: u32) {}\n'; return { name: 'none', lib: lib + 'fn cacheWants(x: vec3f, n: vec3f, moves: bool) -> u32 { return 0u; }\n', bind: {}, stages: [], async init(lab) { lab.buffer('hits', HITS * 16); } }; }
export const bilinear: UpscalePart = { name: 'bilinear', stages(g) { g.signal('picture', 'rgba16float', 'output', true); return { result: 'picture', stages: [{ name: 'upscale', over: 'output', reads: { color: 'color' }, writes: { outT: 'picture' }, buffers: { taauK: ['taauK', 'vec4f', 'uniform'] }, code: '@compute @workgroup_size(8, 8) fn main(@builtin(global_invocation_id) id: vec3u) { let o = frame.size.zw; if (f32(id.x) >= o.x || f32(id.y) >= o.y) { return; } textureStore(outT, id.xy, textureSampleLevel(color, linSamp, (vec2f(id.xy) + 0.5) / o, 0.0)); }' }] }; } };
export const noDenoise: DenoisePart = { name: 'none', stages: ({ signal, second }) => ({ stages: [], result: signal, ...(second ? { second } : {}) }) };
export const taauUp: UpscalePart = { name: 'taau', stages(g) { g.signal('picture', 'rgba16float', 'output', true); return { result: 'picture', stages: [{ name: 'upscale', over: 'output', reads: { color: 'color', 'picture@prev': 'hist', aux: 'auxT', airD: 'airD' }, writes: { outT: 'picture' }, buffers: { taauK: ['taauK', 'vec4f', 'uniform'] }, code: taau }] }; } };

/** where light is kept in the air: over the rooms and six metres round them */
const airGrid = (p: Place) => { const h = 0.3, lo = [0, 1, 2].map(k => Math.min(...p.rooms.map(r => r[k]!)) - (k === 1 ? 0.3 : 6)), hi = [0, 1, 2].map(k => Math.max(...p.rooms.map(r => r[k + 3]!)) + (k === 1 ? 1.5 : 6)); return { h, lo, n: lo.map((v, k) => Math.ceil((hi[k]! - v) / h)) }; };
const airLightCode = (p: Place) => { const g = airGrid(p); return `const AL_LO = vec3f(${g.lo.map(v => v.toFixed(3)).join(', ')}); const AL_N = vec3u(${g.n.map(v => v + 'u').join(', ')}); const AL_H = ${g.h}; const AL_MANY = ${g.n[0]! * g.n[1]! * g.n[2]!}u;\n`; };
function airLightStages(lab: Lab, bind: NonNullable<Stage['buffers']>, lib: string): Stage[] {
  const p = lab.building.place, g = airGrid(p), many = g.n[0]! * g.n[1]! * g.n[2]!, groups = Math.ceil(many / 64); lab.buffer('airKept', many * 16); if (!lab.graph.signals.has('airLight')) lab.graph.bind('airLight', lab.gpu.device.createTexture({ label: 'air light', dimension: '3d', size: g.n as [number, number, number], format: 'rgba16float', usage: GPUTextureUsage.TEXTURE_BINDING | GPUTextureUsage.STORAGE_BINDING | GPUTextureUsage.COPY_SRC }));
  return [{ name: 'air light', over: { groups: () => [Math.min(65535, groups), Math.ceil(groups / 65535), 1] }, code: signals + airLightCode(p) + lib + bounceLib + airlight, buffers: { ...bind, kept: ['airKept', 'array<vec4u>', 'read_write'] }, writes: { outLight: 'airLight' } }];
}
/** the light in the cells: one more look at some, then handed to where every ray reads it. `go`: is it their turn this frame? */
function cellStages(lab: Lab, bind: NonNullable<Stage['buffers']>, lib: string, go: () => boolean): Stage[] {
  const { many, at } = lab.litCells, bytes = Math.max(1, many) * 48, b = lab.scene.blocks, RW = GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_SRC | GPUBufferUsage.COPY_DST; lab.buffer('cellLight', bytes, RW); lab.buffer('cellsAt', Math.max(32, at.byteLength), GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST); lab.gpu.device.queue.writeBuffer(lab.graph.buffers.get('cellsAt')!, 0, at); lab.uniform('cellsMany', new Uint32Array([many, 0, 0, 0]));
  return [{ name: 'cells', over: { groups: () => [Math.ceil(many / 8), 1, 1] }, when: () => many > 0 && go(), code: signals + lib + bounceLib + cellsCode, buffers: { ...bind, field: ['cellLight', 'array<u32>', 'read_write'], places: ['cellsAt', 'array<vec4f>', 'read'], many: ['cellsMany', 'vec4u', 'uniform'] } },
    { name: 'cells', over: { groups: () => [1, 1, 1] }, code: '', async make() { return enc => { if (many && go()) enc.copyBufferToBuffer(lab.graph.buffers.get('cellLight')!, 0, lab.scene.extraBuffer, lightRow(b, lab.scene.blockRow) * 16, many * 48); }; } }];
}
/** a brick of smoke as one cell, for rays wider than it */
function smokeMeanStages(lab: Lab): Stage[] {
  const when = () => lab.smoke.on; lab.buffer('smokeMean', SMOKE_ROWS * 16, GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_SRC | GPUBufferUsage.COPY_DST);
  return [{ name: 'smoke mean', over: { groups: () => [SMOKE_ROWS, 1, 1] }, when, code: smokeMean, buffers: { mean: ['smokeMean', 'array<vec4f>', 'read_write'] } }, { name: 'smoke mean', over: { groups: () => [1, 1, 1] }, code: '', async make() { return enc => { if (when()) enc.copyBufferToBuffer(lab.graph.buffers.get('smokeMean')!, 0, lab.scene.extraBuffer, smokeRow(lab.scene.blocks, lab.scene.blockRow) * 16, SMOKE_ROWS * 16); }; } }];
}
/** `edit` changes a stage's text before it is built: for asking what a part of it costs */
export function realtime(o: { name?: string; cache?: CachePart; denoise?: DenoisePart; upscale?: UpscalePart; edit?: (s: Stage) => Stage } = {}): Method & { glow: string } {
  const cache = o.cache ?? hashCache(), dn = o.denoise ?? svgf, up = o.upscale ?? taauUp; let last = '', lastHour = -1, quiet = 0, fresh = true, prevOn = new Float32Array(64), turn = 0, since = 0, light = '', beat = 0, holes = 0, holeAge = 0;
  const m: Method & { glow: string; warm?: Promise<void> } = {
    name: o.name ?? 'realtime', result: 'picture', glow: 'rt',
    signals(g, lab) {
      g.signal('surf', 'rgba32uint', 'render', true).signal('aux', 'rgba16float', 'render').signal('emit', 'rgba16float', 'render').signal('front', 'rgba32uint', 'render');
      g.signal('bounceV', 'rgba16float', 'half').signal('vis', 'rgba16float', 'render').signal('lampU', 'rgba32uint', 'render').signal('bounce', 'rgba16float', 'half').signal('mirror', 'rgba16float', 'half').signal('shine', 'rgba16float', 'half').signal('lampT', 'rgba32uint', 'render').signal('tone', 'rgba16float', 'half').signal('lampC', 'rgba16float', 'render').signal('air', 'rgba16float', 'half').signal('haze', 'rgba16float', 'half').signal('airA', 'rgba16float', 'half').signal('airB', 'rgba16float', 'half').signal('airD', 'rgba16float', 'half', true).signal('airH', 'rgba16float', 'half', true).signal('airM', 'rgba16float', 'half', true).signal('color', 'rgba16float', 'render');
      lab.buffer('past', Math.max(1, lab.building.world.moving.length) * 64); lab.uniform('changes', new Float32Array(36)); lab.uniform('taauK', new Float32Array(4)); const pl = new Float32Array(72), ps = lab.building.world.plumes.slice(0, 8); ps.forEach((p, i) => { pl.set([...p.at, p.r], i * 4); pl.set([p.h, p.k, 0, 0], 36 + i * 4); }); pl[32] = ps.length; lab.uniform('plumes', pl);
    },
    stages(lab) {
      const g = lab.graph, c = { ...cache.bind, ...CHANGES };
      const raster = rasterStage(lab, g);
      const S = (name: string, code: string, over: Size, reads: Record<string, string>, writes: Record<string, string>, buffers: Stage['buffers'] = {}, consts?: Record<string, number>): Stage => ({ name, code: signals + code, over, reads, writes, buffers: { ...CHANGES, ...buffers }, ...(consts ? { consts } : {}) });
      const dVis = dn.stages({ signal: 'vis', div: 1, cap: 18, rounds: 3, fractions: true }, g), dB = dn.stages({ signal: 'bounce', second: 'bounceV', div: 2, cap: 36, rounds: 4 }, g), dM = dn.stages({ signal: 'mirror', div: 2, cap: 40, rounds: 4, narrow: true }, g), dT = dn.stages({ signal: 'tone', div: 2, cap: 24, rounds: 3 }, g), dS = dn.stages({ signal: 'shine', div: 2, cap: 26, rounds: 2, narrow: true, layer: 1 }, g), u = up.stages(g); m.result = u.result;
      const rains = rainStages(lab, g); const glows = [...glowStages(g, u.result, 'rt'), ...exposeStages(lab, g, u.result, 'rtGlow2', 'rt')];
      return [
        ...looseStages(lab, g), raster,
        S('first hit', surf, 'render', { id: 'idT', idClear: 'idC', depth: 'depthT' }, { outSurf: 'surf', outAux: 'aux', outEmit: 'emit', outFront: 'front' }, { past: ['past', 'array<vec4f>', 'read'], pastLeaves: ['pastLeaves', 'array<vec4f>', 'read'] }),
        S('water', waterpx, 'render', { idClear: 'idC', depth: 'depthT' }, { outSurf: 'surf', outAux: 'aux', outEmit: 'emit', outFront: 'front' }),
        ...cache.stages, ...cellStages(lab, c, cache.lib, () => turn === 0), ...lab.smoke.stages(g, c, cache.lib, bounceLib), ...smokeMeanStages(lab),
        ...sightStages(lab), S('lamps', torchView + torchLeaves + lamps, 'render', { surf: 'surf', front: 'frontT', sunLeaves: 'sunLeaves', torchLeaves: 'torchTex' }, { outVis: 'vis', outU: 'lampU', outC: 'lampC', outTone: 'tone' }, { torchK: ['torchK', 'array<vec4u, 7>', 'uniform'] }, { FINE: 4 }),
        S('gather', cache.lib + bounceLib + gather, 'half', { surf: 'surf', front: 'frontT' }, { outE: 'bounce', outV: 'bounceV' }, { ...c, hits: ['hits', 'array<vec4u>', 'read_write'] }, { HITS }),
        S('mirror', cache.lib + bounceLib + mirror, 'half', { surf: 'surf', front: 'frontT' }, { outM: 'mirror' }, c, { DIV: 2, LAYER: 0 }),
        S('shine', cache.lib + bounceLib + mirror, 'half', { surf: 'surf', front: 'frontT' }, { outM: 'shine' }, c, { DIV: 2, LAYER: 1 }),
        ...dVis.stages, ...dT.stages, ...dB.stages, ...dM.stages, ...dS.stages,
        ...airLightStages(lab, c, cache.lib), ...torchStages(lab), S('haze', airLightCode(lab.building.place) + torchCode + torchLeaves + haze, 'half', { surf: 'surf', airLight: 'light', torchLeaves: 'torchTex' }, { outA: 'haze' }, { torchMap: ['torchMap', 'array<f32>', 'read'], torchK: ['torchK', 'array<vec4u, 7>', 'uniform'] }), S('air', air, 'half', { surf: 'surf' }, { outA: 'air', outD: 'airD' }),
        S('air history', 'fn airAt2(p: vec2u) -> vec2f { return textureLoad(airD, p, 0).xy; }\nfn airWas(p: vec2u) -> vec2f { return textureLoad(airP, p, 0).xy; }\n' + temporal.replace(/^\s*\/\/2 /gm, '  // ').replace('CAP_NOW', 'f32(CAP)'), 'half', { airD: 'airD', 'airD@prev': 'airP', air: 'cur', 'airH@prev': 'hist', 'airM@prev': 'momPrev', surf: 'surf', 'surf@prev': 'surfPrev', aux: 'auxT', front: 'frontT' }, { outH: 'airH', outM: 'airM' }, {}, { DIV: 2, CAP: 16, SPREAD: 1.0, NARROW: 0, KIND: 0, LAYER: 0, AIR: 1 }),
        S('air blur', airblur, 'half', { airH: 'src', surf: 'surf', haze: 'haze' }, { outB: 'airA' }, {}, { STRIDE: 1 }), S('air blur', airblur, 'half', { airA: 'src', surf: 'surf', haze: 'haze' }, { outB: 'airB' }, {}, { STRIDE: 2 }),
        S('tint', tint, 'render', { surf: 'surf', lampU: 'lampU', [dVis.result]: 'vis', [dT.result]: 'tone' }, { outT: 'lampT' }),
        S('resolve', WHOLE + sightCode + resolve, 'render', { surf: 'surf', emit: 'emit', front: 'frontT', lampT: 'lampT', [dB.result]: 'bounce', [dB.second ?? 'bounceV']: 'bounceV', [dM.result]: 'mirror', [dS.result]: 'shine', lampC: 'lampC', airB: 'air' }, { outC: 'color' }, { sight: ['sight', 'array<f32>', 'read'], sightK: ['sightK', 'array<vec4f, 2>', 'uniform'] }),
        ...u.stages, ...rains, ...glows,
      ].map(s => o.edit ? o.edit(s) : s);
    },
    frame(lab) {
      rainKnobs(lab); lab.smoke.frame();
      // cells look every fourth frame (what a look costs is mostly waiting), every one, and with a short memory, while the light changes: doors, lamps, the hour, the weather
      { const now = [lab.lampsMode, lab.hour.toFixed(2), lab.mains, lab.moon.toFixed(1), lab.rainNow.toFixed(1), lab.snowNow.toFixed(1), lab.wet.toFixed(1), lab.snow.toFixed(1), lab.broken, ...lab.building.doors.map(d => d.open.toFixed(2)), ...lab.building.world.lamps.map((l, i) => /fire|flash/.test(l.name) || l.kind === 'spot' ? 0 : lab.scene.lampOn[i]!.toFixed(1))].join(); if (now !== light || fresh) { light = now; since = 0; } else since++; const hurry = since < 60; turn = hurry ? 0 : (turn + 1) % 4; if (turn === 0) beat++; lab.uniform('cellsMany', new Uint32Array([lab.litCells.many, beat, hurry && since > 0 ? 1 : 0, 0])); }
      // where each moving piece was; and what changed
      const mv = lab.building.world.moving, f = new Float32Array(mv.length * 16); for (const p of mv) { const s = lab.slots.get(p); if (s === undefined) continue; const q = lab.prevAt.get(p) ?? p, o = (s - lab.scene.piece0[2]) * 16; f.set(q.at, o); f.set(q.rot[0], o + 4); f.set(q.rot[1], o + 8); f.set(q.rot[2], o + 12); } if (mv.length) lab.gpu.device.queue.writeBuffer(lab.graph.buffers.get('past')!, 0, f);
      const ch = new Float32Array(36); let n = 0; for (const d of lab.building.doors) if (Math.abs(d.want - d.open) > 1e-3 && n < 8) { ch.set([d.hinge[0], d.hinge[1] + 1, d.hinge[2], 7.5], n * 4); n++; } if (lab.broken !== holes) { holes = lab.broken; holeAge = 0; } else holeAge++; if (holes && holeAge < 45 && n < 8) { ch.set([...lab.holes[0]!.at, 9], n * 4); n++; }
      let many = 0; lab.building.world.lamps.forEach((l, i) => { if (Math.abs(lab.scene.lampOn[i]! - prevOn[i]!) > 0.02 && l.kind !== 'spot' && l.name !== 'flash' && l.name !== 'fire') many++; if (Math.abs(lab.scene.lampOn[i]! - prevOn[i]!) > 0.02 && l.kind !== 'spot' && n < 8) { ch.set([...l.at, 9], n * 4); n++; } }); prevOn = lab.scene.lampOn.slice();
      const drift = lastHour < 0 ? 0 : Math.abs(lab.hour - lastHour), key = lab.lampsMode + lab.mains + lab.moon; lastHour = lab.hour; if (key !== last || drift > 0.02 || many > 5) { const turned = key !== last && last !== ''; last = key; quiet = 40; if (drift > 0.1 || many > 5 || turned) ch[33] = 1; }      /* (more lamps switched than can be told of one by one: everything forgets) */ quiet = Math.max(0, quiet - 1); ch[32] = 1 - Math.max(0.85 * Math.min(1, quiet / 24), Math.min(0.6, drift * 110)); lab.uniform('changes', ch);
      lab.uniform('taauK', new Float32Array([0.25 * (lab.exposure || 1), fresh ? 1 : 0, 0.2 + 0.8 * ch[32]!, 0])); fresh = false; cache.rush = quiet > 0 ? 3 : 0;
    },
    reset() { fresh = true; },
  };
  (m as { prepare?: (lab: Lab) => Promise<void> }).prepare = async lab => { await cache.init(lab); };
  return m;
}
