// The drops, drawn over the finished picture at its own size (nothing is kept of them from frame to frame): light added, and how much is covered.
import { BREEZE } from '../scene/air';
import type { Lab } from '../lab';
import type { Graph, Stage } from '../graph';
import code from './rain.wgsl' with { type: 'text' };
export const RAIN_SIDE = 160;
const draw = `
struct Drop { head: vec4f, tail: vec4f, light: vec4f };
@group(1) @binding(0) var<storage, read> drops: array<Drop>; @group(1) @binding(1) var surf: texture_2d<u32>;
struct V { @builtin(position) p: vec4f, @location(0) uv: vec2f, @location(1) @interpolate(flat) light: vec4f, @location(2) @interpolate(flat) kind: vec2f, @location(3) t: f32 };
fn clipOf(x: vec3f) -> vec4f { let v = x - frame.camPos.xyz; let z = dot(v, frame.camFwd.xyz); let r = frame.camRight.xyz; let u = frame.camUp.xyz; return vec4f(dot(v, r) / dot(r, r), dot(v, u) / dot(u, u), 0.25, z); }
@vertex fn vs(@builtin(vertex_index) vi: u32, @builtin(instance_index) i: u32) -> V {
  var o: V; let d = drops[i]; o.p = vec4f(0.0, 0.0, -1.0, 1.0); if (d.light.a <= 0.0) { return o; } let c = vec2f(f32(vi & 1u), f32(vi >> 1u)); o.uv = c * 2.0 - 1.0; o.light = d.light; o.kind = vec2f(d.tail.w, d.tail.x); var x: vec3f;
  if (d.tail.w > 0.5) { let px = distance(d.head.xyz, frame.camPos.xyz) * 2.0 * length(frame.camUp.xyz) / frame.size.w; let r = max(d.head.w, 1.1 * px); o.light.a *= min(1.0, d.head.w / r * 1.4); x = d.head.xyz + vec3f(o.uv.x * r, 0.012, o.uv.y * r); }
  else { let axis = normalize(d.head.xyz - d.tail.xyz); let side = normalize(cross(axis, d.head.xyz - frame.camPos.xyz)); x = mix(d.tail.xyz, d.head.xyz, c.y) + side * (o.uv.x * d.head.w); }
  o.p = clipOf(x); o.t = distance(x, frame.camPos.xyz); return o;
}
@fragment fn fs(v: V) -> @location(0) vec4f {
  let q = vec2u(v.p.xy / frame.size.zw * frame.size.xy); let t = bitcast<f32>(textureLoad(surf, min(q, vec2u(frame.size.xy) - 1u), 0).x); if (v.t > t + 0.06) { discard; } var a = v.light.a;
  if (v.kind.x > 0.5) { let r = length(v.uv); let ring = exp(-pow((r - 0.75) * 5.0, 2.0)); let dot0 = exp(-r * r * 9.0) * (1.0 - v.kind.y); a *= max(ring, dot0); }
  else { a *= (1.0 - v.uv.x * v.uv.x) * smoothstep(-1.0, 0.2, v.uv.y) * (1.0 - smoothstep(0.86, 1.0, v.uv.y)); }
  return vec4f(v.light.rgb * a, a);
}`;
export function rainStages(lab: Lab, g: Graph): Stage[] {
  g.signal('rain', 'rgba16float', 'output'); lab.buffer('drops', RAIN_SIDE * RAIN_SIDE * 48); lab.uniform('rainK', new Float32Array(4)); lab.uniform('rainAt', new Float32Array(4));
  const on = () => lab.rainNow > 0.004;
  return [
    { name: 'rain', over: { groups: () => [Math.ceil(RAIN_SIDE * RAIN_SIDE / 64), 1, 1] }, when: on, buffers: { drops: ['drops', 'array<Drop>', 'read_write'], rainK: ['rainK', 'vec4f', 'uniform'], rainAt: ['rainAt', 'vec4f', 'uniform'] }, code },
    { name: 'rain draw', over: 'output', code: '', async make() {
      const d = g.g.device, V = GPUShaderStage.VERTEX, l1 = d.createBindGroupLayout({ entries: [{ binding: 0, visibility: V, buffer: { type: 'read-only-storage' } }, { binding: 1, visibility: GPUShaderStage.FRAGMENT, texture: { sampleType: 'uint' } }] }), text = g.prelude() + draw, module = d.createShaderModule({ label: 'rain', code: text });
      const bad = (await module.getCompilationInfo()).messages.find(m => m.type === 'error'); if (bad) throw new Error(`rain draw:${bad.lineNum} ${bad.message}\n${text.split('\n')[bad.lineNum - 1]?.slice(0, 160)}`);
      const pipe = await d.createRenderPipelineAsync({ label: 'rain', layout: d.createPipelineLayout({ bindGroupLayouts: [g.shared.layout, l1] }), vertex: { module, entryPoint: 'vs' }, fragment: { module, entryPoint: 'fs', targets: [{ format: 'rgba16float', blend: { color: { srcFactor: 'one', dstFactor: 'one' }, alpha: { srcFactor: 'one', dstFactor: 'one-minus-src-alpha' } } }] }, primitive: { topology: 'triangle-strip' } });
      let was = false; return enc => {
        const now = on(); if (!now && !was) return; was = now; const p = enc.beginRenderPass({ label: 'rain draw', colorAttachments: [{ view: g.view('rain'), loadOp: 'clear', storeOp: 'store', clearValue: { r: 0, g: 0, b: 0, a: 0 } }], timestampWrites: g.timer.pass('rain draw') as GPURenderPassTimestampWrites | undefined });
        if (now) { p.setPipeline(pipe); p.setBindGroup(0, g.shared.group()); p.setBindGroup(1, d.createBindGroup({ layout: l1, entries: [{ binding: 0, resource: { buffer: g.buffers.get('drops')! } }, { binding: 1, resource: g.view('surf') }] })); p.draw(4, RAIN_SIDE * RAIN_SIDE); } p.end();
      };
    } },
  ];
}
/** which columns there is rain in: round what is looked at, as finely as the view asks */
export function rainKnobs(lab: Lab) {
  const c = lab.camera, want = Math.max(5, Math.min(46, c.dist * 0.62)) * 2 / RAIN_SIDE, s = 0.1 * 2 ** (Math.round(Math.log2(want / 0.1) * 2) / 2);
  lab.uniform('rainK', new Float32Array([0, Math.max(0.06, s), RAIN_SIDE, lab.air.strength * BREEZE * 1.4])); lab.uniform('rainAt', new Float32Array([c.look[0], c.look[2], 0, 0]));
}
