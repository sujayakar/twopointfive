// The path tracer with only the rays after its first hit obeying the rule: what the rule itself costs in truth, with nothing kept and nothing filtered.
import { pathTracer } from './path';
import type { Method, Lab } from '../lab';
/** `s`: how fast a bounce ray widens. `direct`: shadow rays at what the eye sees obey it too */
export function ruled(s: number, o: { cells?: boolean; only?: string } = {}) {
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
export const ruleMethods: Record<string, () => Method> = {
  ...Object.fromEntries([1, 0.5, 0.25, 0.125, 0.0625].flatMap(s => [[`rule${s}ref`, () => rulePath(`rule${s}ref`, ruled(s))], [`gone${s}ref`, () => rulePath(`gone${s}ref`, ruled(s, { cells: false }))]])),
};
