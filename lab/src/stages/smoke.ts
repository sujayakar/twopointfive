// Smoke, as stages: shut faces, move, push, make incompressible, light, hand over to rays.
import type { Lab } from '../lab';
import type { Graph, Stage } from '../graph';
import type { V3 } from '../math';
import head from './smoke.wgsl' with { type: 'text' };
import signals from '../wgsl/signals.wgsl' with { type: 'text' };
export const SMOKE = { n: [128, 40, 96] as const, h: 0.2, at: [-12.8, 0, -9.6] as const, rounds: 14 };
const CELLS = SMOKE.n[0] * SMOKE.n[1] * SMOKE.n[2], OVER = { groups: (): [number, number, number] => [SMOKE.n[0] / 8, SMOKE.n[1] / 4, SMOKE.n[2] / 4] };
const cell = `@compute @workgroup_size(8, 4, 4) fn main(@builtin(global_invocation_id) id: vec3u) { let c = vec3i(id); let i = ix(c);`;
const SHUT = `
/** is the way from c one step along axis a (s = ±1) shut? The grid's own edge is open */
fn shut(c: vec3i, a: u32, s: i32) -> bool { var q = c; if (s < 0) { q[a] -= 1; } if (any(q < vec3i(0)) || any(q >= N)) { return false; } return ((faces[ix(q)] >> (3u + a)) & 1u) != 0u; }`;
const facesCode = `
struct Box { r0: vec4f, r1: vec4f, r2: vec4f, half: vec4f };
fn cuts(b: Box, a: vec3f, e: vec3f) -> bool { let o = vec3f(dot(b.r0.xyz, a) + b.r0.w, dot(b.r1.xyz, a) + b.r1.w, dot(b.r2.xyz, a) + b.r2.w); let d = vec3f(dot(b.r0.xyz, e), dot(b.r1.xyz, e), dot(b.r2.xyz, e)); let inv = 1.0 / select(d, vec3f(1e-9), abs(d) < vec3f(1e-9)); let h = b.half.xyz + 0.02; let p = (-h - o) * inv; let q = (h - o) * inv; return max(max3(min(p, q)), 0.0) <= min(min3(max(p, q)), 1.0); }
${cell}
  var f = faces[i] & 7u; let x = mid(c); fineLeaves = false; noLeaves = true;
  if (smokeK.y > 0.5) { f = 0u; for (var a = 0u; a < 3u; a++) { var d = vec3f(0.0); d[a] = 1.0; let h = nearest(x, d, 0.0, H, 0x4000u | WATER, 1e9); if ((h.piece != NOTHING && (h.piece & LEAF_HIT) == 0u) || h.gt < H) { f |= 1u << a; } } }
  var g = f; for (var k = 0u; k < u32(smokeK.z); k++) { for (var a = 0u; a < 3u; a++) { var e = vec3f(0.0); e[a] = H; if (cuts(doors[k], x, e)) { g |= 1u << a; } } }
  faces[i] = f | (g << 3u);
}`;
const moveCode = SHUT + `
/** what stands at c + d (d in cells, each under one): the eight cells round it, and none that is behind a wall */
fn gatherAt(c: vec3i, d0: vec3f) -> vec4f { let d = clamp(d0, vec3f(-0.999), vec3f(0.999)); let s = vec3i(select(vec3f(-1.0), vec3f(1.0), d > vec3f(0.0))); let w = abs(d); var acc = vec4f(0.0); gHeat = 0.0;
  for (var k = 0u; k < 8u; k++) { var q = c; var wt = 1.0; for (var a = 0u; a < 3u; a++) { let on = ((k >> a) & 1u) == 1u; wt *= select(1.0 - w[a], w[a], on); if (on && !shut(q, a, s[a])) { q[a] += s[a]; } } if (wt <= 0.0 || !inside(q)) { continue; } let j = ix(q); acc += a0[j] * wt; gHeat += h0[j] * wt; } return acc; }
var<private> gHeat: f32;
${cell}
  let dt = smokeK.x; let v = a0[i].xyz; let here = gatherAt(c, -v * (dt / H)); b0[i] = vec4f(here.xyz, here.w * exp(-dt / 70.0)); h1[i] = gHeat * exp(-dt / 2.5);
}`;
const pushCode = SHUT + `
fn vel(c: vec3i, o: vec3i, me: vec3f) -> vec3f { let q = c + o; if (!inside(q)) { return me; } return b0[ix(q)].xyz; }
fn curl(c: vec3i) -> vec3f { if (!inside(c)) { return vec3f(0.0); } let m = b0[ix(c)].xyz; let x = vel(c, vec3i(1, 0, 0), m) - vel(c, vec3i(-1, 0, 0), m); let y = vel(c, vec3i(0, 1, 0), m) - vel(c, vec3i(0, -1, 0), m); let z = vel(c, vec3i(0, 0, 1), m) - vel(c, vec3i(0, 0, -1), m); return vec3f(y.z - z.y, z.x - x.z, x.y - y.x) / (2.0 * H); }
${cell}
  let dt = smokeK.x; var s = b0[i]; var heat = h1[i]; let x = mid(c); var v = s.xyz;
  // swirls that a coarse grid would lose are spun up again where there is smoke
  if (s.w > 0.003) { let w = curl(c); let g = vec3f(length(curl(c + vec3i(1, 0, 0))) - length(curl(c - vec3i(1, 0, 0))), length(curl(c + vec3i(0, 1, 0))) - length(curl(c - vec3i(0, 1, 0))), length(curl(c + vec3i(0, 0, 1))) - length(curl(c - vec3i(0, 0, 1)))); let l = length(g); if (l > 1e-4) { v += cross(g / l, w) * (dt * 0.9 * H * 2.0); } }
  v.y += dt * (heat * 1.5 - s.w * 0.03);
  // the open air carries it off
  let out = select(0.0, 1.0, openAir(x)); let wind = vec3f(cos(frame.weather.w), 0.0, sin(frame.weather.w)) * 1.1; v = mix(v, wind, out * (1.0 - exp(-dt * 1.2))); s.w *= 1.0 - out * (1.0 - exp(-dt * 0.05)); v *= exp(-dt * 0.10);
  for (var k = 0u; k < u32(smokeK.w); k++) { let e = sources[k * 2u]; let f = sources[k * 2u + 1u]; let d = x - e.xyz; let r = length(d); if (r < e.w) { let q = 1.0 - r / e.w; let sp = hash33(vec3f(vec2f(c.xz), floor(frame.misc.x * 12.0) + f32(k))) - 0.5; s.w = min(s.w + f.x * q * dt, 9.0); heat = min(heat + f.y * q * dt, 2.5); v += (normalize(d + vec3f(0.0, 0.12, 0.0)) * 5.0 + sp * 9.0) * (q * dt * f.z); } }
  let sp = length(v); v *= min(1.0, (0.9 * H / dt) / max(sp, 1e-4)); a0[i] = vec4f(v, s.w); h0[i] = heat;
}`;
const FACE = SHUT + `
/** the velocity through c's far face along a */
fn face(c: vec3i, a: u32) -> f32 { var q = c; q[a] += 1; if (c[a] < 0) { return a0[ix(q)][a]; } if (q[a] >= N[a]) { return a0[ix(c)][a]; } if (shut(c, a, 1)) { return 0.0; } return 0.5 * (a0[ix(c)][a] + a0[ix(q)][a]); }`;
const divCode = FACE + cell + ` var d = 0.0; for (var a = 0u; a < 3u; a++) { var q = c; q[a] -= 1; d += face(c, a) - face(q, a); }
  // a canister makes room for what it makes: it pushes the air before it
  for (var k = 0u; k < u32(smokeK.w); k++) { let e = sources[k * 2u]; let l = distance(mid(c), e.xyz); if (l < e.w) { d -= sources[k * 2u + 1u].w * (1.0 - l / e.w); } }
  div[i] = d; press[i] *= 0.8; }`;
const solveCode = SHUT + `
@compute @workgroup_size(8, 4, 4) fn main(@builtin(global_invocation_id) id: vec3u) { var c = vec3i(id); c.x = c.x * 2 + ((c.y + c.z + PARITY) & 1); let i = ix(c); var s = 0.0; var n = 0.0;
  for (var a = 0u; a < 3u; a++) { for (var k = -1; k < 2; k += 2) { if (shut(c, a, k)) { continue; } n += 1.0; var q = c; q[a] += k; if (inside(q)) { s += press[ix(q)]; } } } if (n > 0.0) { let p = (s - div[i]) / n; press[i] = mix(press[i], p, 1.55); } }`;
const freeCode = FACE + `
fn pr(c: vec3i) -> f32 { if (!inside(c)) { return 0.0; } return press[ix(c)]; }
fn freed(c: vec3i, a: u32) -> f32 { if (inside(c)) { if (shut(c, a, 1)) { return 0.0; } } var q = c; q[a] += 1; return face(c, a) - (pr(q) - pr(c)); }
${cell} let was = a0[i]; var v = vec3f(0.0); for (var a = 0u; a < 3u; a++) { var q = c; q[a] -= 1; var sh = 0.0; if (shut(c, a, -1)) { sh = 1.0; } let near = select(freed(q, a), 0.0, sh > 0.5); let d = 0.5 * (freed(c, a) + near) - 0.5 * (face(c, a) + select(face(q, a), 0.0, sh > 0.5)); v[a] = was[a] + d; if (sh > 0.5 && shut(c, a, 1)) { v[a] = 0.0; } else if (sh > 0.5) { v[a] = max(v[a], 0.0) * 0.5 + 0.5 * freed(c, a); } else if (shut(c, a, 1)) { v[a] = min(v[a], 0.0) * 0.5 + 0.5 * near; } } b0[i] = vec4f(v, was.w); }`;
const lightCode = `
${cell}
  let s = b0[i]; let was = glow[i]; if (s.w < 0.004) { glow[i] = vec4f(0.0); return; }
  rngKey(i, 61u); fineLeaves = false; byReach = true; let x = mid(c) + (vec3f(rnd(), rnd(), rnd()) - 0.5) * H; var L = vec3f(0.0); var d = vec3f(0.0); var far = 400.0;
  for (var k = 0u; k < 2u; k++) { var e = vec3f(0.0); if (k == 0u) { if (max3(frame.sunE.xyz) <= 0.0) { continue; } d = coneDir(frame.sunDir.xyz, frame.sunDir.w, rnd2()); e = frame.sunE.xyz; far = 400.0; } else { let l = drawLamp(x, vec3f(0.0)); if (l.x < 0.0) { continue; } let q = aimLamp(u32(l.x), x, rnd2()); d = q.dir; far = q.dist - 0.03; e = min(q.E * l.y, vec3f(200.0)); } if (max3(e) <= 0.0) { continue; } L += e * visible(x, d, 1e-3, far) * (1.0 / (4.0 * PI)); }
  let u = rnd2(); let z = u.x * 2.0 - 1.0; let r = sqrt(max(1.0 - z * z, 0.0)); d = vec3f(r * cos(6.2832 * u.y), z, r * sin(6.2832 * u.y)); let h = nearest(x, d, 1e-3, 80.0, 0u, 1e9); let sm = smokeSeen(x, d, h.t); var B = vec3f(0.0);
  if (h.piece == NOTHING) { B = skyLod(d, 5.0); } else { let q = surfaceMean(x, d, h); let leaf = (h.piece & LEAF_HIT) != 0u; let own = !leaf && (pieces[h.piece].lamp & 255u) != 255u; B = select(q.glow, vec3f(0.0), own) + sentBack(q, leaf, rnd2(), d); }
  L += min(B * h.thru * sm.a + sm.rgb, vec3f(40.0)); let st = steady(x); let m = min(was.w + 1.0, mix(4.0, 22.0, st)); let o = finite4(vec4f(mix(was.rgb, L, 1.0 / m), m), 1e4); glow[i] = o;
}`;
const packCode = `
${cell} let s = b0[i]; let o = glow[i]; pack[i] = vec2u(pack2x16float(o.rg), pack2x16float(vec2f(o.b, select(0.0, s.w, s.w > 0.001)))); if (s.w < 0.004) { return; } let b = u32(c.x / 8 + 16 * (c.y / 8 + 5 * (c.z / 8))); atomicOr(&occ[b >> 5u], 1u << (b & 31u)); }`;
export class Smoke {
  on = false; sources: { at: V3; born: number; last: number; rate: number }[] = []; private lastTick = -1; private doorKey = ''; private init = true; dt = 0; private faceNow = false;
  constructor(private lab: Lab) {}
  /** a canister where it lies: it burns for `last` seconds */
  drop(at: V3, last = 14, rate = 1) { this.on = true; this.sources.push({ at: [at[0], Math.max(at[1], 0.1) + 0.18, at[2]], born: this.lab.tick, last, rate }); if (this.sources.length > 8) this.sources.shift(); }
  clear() { this.sources = []; const d = this.lab.gpu.device, g = this.lab.graph; for (const n of ['smokeA', 'smokeB', 'smokeHeatA', 'smokeHeatB', 'smokePress', 'smokeGlow', 'smokePack']) { const b = g.buffers.get(n); if (b) { const e = d.createCommandEncoder(); e.clearBuffer(b); d.queue.submit([e.finish()]); } } const e = d.createCommandEncoder(); const p = g.buffers.get('smokePack'); if (p) this.copy(e); e.clearBuffer(this.lab.scene.smokeOcc); d.queue.submit([e.finish()]); this.on = false; }
  copy(e: GPUCommandEncoder) { e.copyBufferToTexture({ buffer: this.lab.graph.buffers.get('smokePack')!, bytesPerRow: SMOKE.n[0] * 8, rowsPerImage: SMOKE.n[1] }, { texture: this.lab.scene.smokeTex }, [...SMOKE.n]); e.copyBufferToBuffer(this.lab.graph.buffers.get('smokeOcc')!, 0, this.lab.scene.smokeOcc, 0, 128); }
  frame() {
    const lab = this.lab; if (!this.on) { this.dt = 0; return; } const d = Math.max(0, Math.min(4, lab.tick - this.lastTick)); this.dt = this.lastTick < 0 ? 1 / 60 : d / 60; this.lastTick = lab.tick;
    const key = lab.building.doors.map(x => x.open.toFixed(3)).join(); this.faceNow = this.init || key !== this.doorKey; this.doorKey = key;
    const boxes = lab.building.doors.map(x => x.leaf[0]!), f = new Float32Array(16 * 16); boxes.slice(0, 16).forEach((p, i) => { for (let k = 0; k < 3; k++) { const r = p.rot[k]!; f.set([r[0], r[1], r[2], -(r[0] * p.at[0] + r[1] * p.at[1] + r[2] * p.at[2])], i * 16 + k * 4); } f.set(p.half, i * 16 + 12); }); lab.uniform('smokeDoors', f);
    const s = new Float32Array(64), live = this.sources.filter(q => lab.tick - q.born < q.last * 60); live.forEach((q, i) => { const age = (lab.tick - q.born) / 60, k = Math.min(1, age / 0.6) * Math.min(1, (q.last - age) / 2); s.set([...q.at, 0.5, 170 * q.rate * k, 1.1 * k, 0.9 * k, 3.4 * q.rate * k], i * 8); }); lab.uniform('smokeSources', s);
    lab.uniform('smokeK', new Float32Array([this.dt, this.init ? 1 : 0, boxes.length, live.length])); this.init = false;
  }
  stages(g: Graph, cacheBind: NonNullable<Stage['buffers']>, cacheLib: string, bounceLib: string): Stage[] {
    const lab = this.lab, B = (n: string, bytes: number) => lab.buffer(n, bytes, GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_SRC | GPUBufferUsage.COPY_DST); B('smokeA', CELLS * 16); B('smokeB', CELLS * 16); B('smokeHeatA', CELLS * 4); B('smokeHeatB', CELLS * 4); B('smokeFaces', CELLS * 4); B('smokePress', CELLS * 4); B('smokeDiv', CELLS * 4); B('smokeGlow', CELLS * 16); B('smokePack', CELLS * 8); B('smokeOcc', 128);
    lab.uniform('smokeK', new Float32Array(4)); lab.uniform('smokeDoors', new Float32Array(256)); lab.uniform('smokeSources', new Float32Array(64)); lab.uniform('changes', new Float32Array(36));
    const go = () => this.on && this.dt > 0, faces: NonNullable<Stage['buffers']> = { faces: ['smokeFaces', 'array<u32>', 'read'] }, K: NonNullable<Stage['buffers']> = { smokeK: ['smokeK', 'vec4f', 'uniform'] }, rw = (n: string, t = 'array<vec4f>'): [string, string, 'read_write'] => [n, t, 'read_write'], ro = (n: string, t = 'array<vec4f>'): [string, string, 'read'] => [n, t, 'read'];
    const S = (name: string, code: string, buffers: NonNullable<Stage['buffers']>, over: Stage['over'] = OVER, consts?: Record<string, number>): Stage => ({ name, over, code: head + code, buffers, when: go, ...(consts ? { consts } : {}) });
    const half = { groups: (): [number, number, number] => [SMOKE.n[0] / 16, SMOKE.n[1] / 4, SMOKE.n[2] / 4] };
    return [
      { ...S('smoke faces', facesCode, { faces: rw('smokeFaces', 'array<u32>'), ...K, doors: ['smokeDoors', 'array<Box, 16>', 'uniform'] }), when: () => go() && this.faceNow },
      S('smoke move', moveCode, { a0: ro('smokeA'), h0: ro('smokeHeatA', 'array<f32>'), b0: rw('smokeB'), h1: rw('smokeHeatB', 'array<f32>'), ...faces, ...K }),
      S('smoke push', pushCode, { b0: ro('smokeB'), h1: ro('smokeHeatB', 'array<f32>'), a0: rw('smokeA'), h0: rw('smokeHeatA', 'array<f32>'), ...faces, ...K, sources: ['smokeSources', 'array<vec4f, 16>', 'uniform'] }),
      S('smoke press', divCode, { a0: ro('smokeA'), div: rw('smokeDiv', 'array<f32>'), press: rw('smokePress', 'array<f32>'), ...faces, ...K, sources: ['smokeSources', 'array<vec4f, 16>', 'uniform'] }),
      ...Array.from({ length: SMOKE.rounds * 2 }, (_, i) => S('smoke press', solveCode, { div: ro('smokeDiv', 'array<f32>'), press: rw('smokePress', 'array<f32>'), ...faces }, half, { PARITY: i & 1 })),
      S('smoke free', freeCode, { a0: ro('smokeA'), press: ro('smokePress', 'array<f32>'), b0: rw('smokeB'), ...faces }),
      { name: 'smoke clear', over: OVER, code: '', async make() { return enc => { if (go()) enc.clearBuffer(g.buffers.get('smokeOcc')!); }; } },
      { ...S('smoke light', '', {}), reads: { sunLeaves: 'sunLeaves' }, code: signals + cacheLib + bounceLib + head + lightCode, buffers: { ...cacheBind, b0: ro('smokeB'), glow: rw('smokeGlow'), changes: ['changes', 'Changes', 'uniform'] } },
      S('smoke pack', packCode, { b0: ro('smokeB'), glow: ro('smokeGlow'), pack: rw('smokePack', 'array<vec2u>'), occ: rw('smokeOcc', 'array<atomic<u32>>') }),
      { name: 'smoke hand', over: OVER, code: '', async make() { return (enc: GPUCommandEncoder) => { if (go()) { enc.copyBufferToBuffer(g.buffers.get('smokeB')!, 0, g.buffers.get('smokeA')!, 0, CELLS * 16); lab.smoke.copy(enc); } }; } },
    ];
  }
}
