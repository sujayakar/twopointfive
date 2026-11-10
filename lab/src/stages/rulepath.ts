// The path tracer with only the rays after its first hit obeying the rule: what the rule itself costs in truth, with nothing kept and nothing filtered.
import { pathTracer } from './path';
import type { Method, Lab } from '../lab';
/** `s`: how fast a bounce ray widens */
export function ruled(s: number, o: { cells?: boolean } = {}) {
  return (code: string) => {
    let c = swap(code, 'leafMode = 0u;', `leafMode = 0u; blkOn = ${o.cells !== false}; foot(gW, gS);`);
    c = swap(c, 'let h = nearest(ro, rd, 1e-3, 2000.0, 0u, cut);', 'let h = nearest(ro, rd, 1e-3, 2000.0, 0u, cut); fromCell = NOTHING; gW = select(gW + gS * min(h.t, 200.0), 2.0 * spread0 * h.t, gS <= 0.0);');
    c = swap(c, 'L += f * visible(', 'if (gS > 0.0) { foot(gW, select(gLampWide / max(far, 0.1), 2.0 * frame.sunDir.w, q == 0u)); } else { foot(0.0, 0.0); } L += f * visible(');
    c = swap(c, 'let a = aimLamp(u32(k.x), s.x, rnd2()); d = a.dir;', 'let a = aimLamp(u32(k.x), s.x, rnd2()); gLampWide = select(2.0 * lamps[u32(k.x)].area, sqrt(lamps[u32(k.x)].area), lamps[u32(k.x)].kind == 0u); d = a.dir;');
    c = swap(c, 'if (rnd() < pc) { let l = coatDir(s, V, rnd2());', `if (rnd() < pc) { gS = min(gS + 0.003 + 1.5 * s.rough * s.rough, ${s}); let l = coatDir(s, V, rnd2());`);
    c = swap(c, 'else if (isPaint(s.kind) && rnd() < 0.5) {', `else if (isPaint(s.kind) && rnd() < 0.5) { gS = ${s};`);
    c = swap(c, 'var n = s.n; var side = 1.0; var alb = m * (1.0 - wf);', `gS = ${s}; var n = s.n; var side = 1.0; var alb = m * (1.0 - wf);`);
    c = swap(c, 'thr *= 0.99; clear = false; fp = 1.0;', `thr *= 0.99; gS = ${s}; clear = false; fp = 1.0;`);
    c = swap(c, '/** lamps at a surface:', 'var<private> gW = 0.0; var<private> gS = 0.0; var<private> gLampWide = 0.0;\n/** lamps at a surface:');
    return c;
  };
}
function swap(code: string, a: string, b: string) { if (!code.includes(a)) throw new Error('the reference has no ' + a); return code.replace(a, () => b); }
export const rulePath = (name: string, edit: (c: string) => string): Method => { const m = pathTracer({ bounces: 40, name, edit }), was = m.stages; m.stages = function (lab: Lab) { return was.call(this, lab).map(s => s.name === 'reference' ? { ...s, dice: true } : s); }; return m; };
/** from the second surface on, every surface is as the kept light takes it: one even matte that sends back all it returns (or only its matte's share, or only without its bumps) */
const evenly = (how: 'all' | 'matte' | 'flat') => (code: string) => {
  let c = swap(code, 'let glossy = s.rough < 0.25;', how === 'flat' ? 'if (depth > 0u && !water) { s.n = s.ng; } let glossy = s.rough < 0.25;' : `lamb = depth > 0u && !water && !any(s.through > vec3f(0.0)); if (lamb) { let r = ${how === 'all' ? 'returnsAll(s)' : 'matteOf(s) * matteMean(s.kind, s.albedo)'}; s.albedo = r; s.metal = 0.0; s.f0 = 0.0; s.rough = 1.0; s.kind = 0u; s.n = s.ng; V = s.n; } let glossy = s.rough < 0.25;`);
  if (how !== 'flat') { c = c.replaceAll('fresnel(coatF0(s), ', 'fres(s, ').replace('if (q == 0u || !glossy) { f += coatOf', 'if ((q == 0u || !glossy) && !lamb) { f += coatOf'); c = swap(c, '0.08, 0.92), 1.0, wm < 1e-4);', '0.08, 0.92), select(1.0, 0.0, lamb), wm < 1e-4 || lamb);'); }
  return swap(c, '/** lamps at a surface:', 'var<private> lamb = false; fn fres(s: Surf, c: f32) -> vec3f { return select(fresnel(coatF0(s), c), vec3f(0.0), lamb); }\nfn returnsAll(s: Surf) -> vec3f { let f = coatF0(s); let r = s.rough; let a = f * (0.955 / (1.0 + 1.346 * pow(r, 3.67))) + vec3f(0.0476 * exp(-3.55 * r * r)); let lf = lum(f); return matteOf(s) * matteMean(s.kind, s.albedo) * (1.0 - lf - (1.0 - lf) / 21.0) + a; }\n/** lamps at a surface:');
};
export const ruleMethods: Record<string, () => Method> = {
  ...Object.fromEntries((['all', 'matte', 'flat'] as const).map(h => [`even${h}ref`, () => rulePath(`even${h}ref`, evenly(h))])),
  ...Object.fromEntries([1, 0.5, 0.25, 0.125, 0.0625].flatMap(s => [[`rule${s}ref`, () => rulePath(`rule${s}ref`, ruled(s))], [`gone${s}ref`, () => rulePath(`gone${s}ref`, ruled(s, { cells: false }))]])),
};
