// THE SUN'S GLINT: the check behind the scene prelude's SUN_GLINT (src/render/frame.ts sunGlintConfig; on on the look's page, ?nosunglint at boot = as it
// was; no game table, nothing for a recording to carry).
//   bun run tools/qa/probes.ts sunglint        (family `render`)        SUNGLINT_BREAK=1 = the rule off and the same lines asked: must FAIL
// The maintainer, 11-04, of a car's gloss paint beside two pictures of the real thing: "mm doesn't look good. we don't need to go full hyperrealistic but we
// should sell it more". Every lamp's highlight on a smooth finish was added analytically (shaders/reflect.wgsl rtGlints) and the brightest light of all, a
// directional one, was skipped; the sky a reflection ray returns has no sun in it. So by day nothing smooth showed the sun.
// WHAT THIS CAN AND CANNOT SAY. No shader runs here. The rule is held BY ITS TEXT in the three reflection passes (they must stay in step), the prelude's word
// is asked as the page writes it on the look's page, off it, and with the switch down, its off word's wiring is read, and the sun's arithmetic is written
// again below and asked what a glint is: brightest where the face looks half way between the eye and the sun, nothing on a face turned from the sun, on a
// smooth finish brighter than a white wall beside it, never over its bound at a grazing eye or on a metal, and fading out with no step. What the picture
// looks like is the page's to show. Never seen on a GPU.
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { ROOT } from './headless.ts';
export async function sunGlintCheck(): Promise<boolean> {
  const BRK = process.env.SUNGLINT_BREAK === '1'; let all = true;
  const verdict = (ok: boolean, what: string, detail: string) => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}\n    ${detail}`); };
  console.log(`\n=== sunglint: a smooth finish catches the sun${BRK ? ' (the rule off: must fail)' : ''} ===`);
  const read = (f: string) => readFileSync(resolve(ROOT, 'src/render/shaders', f), 'utf8'), src = (f: string) => readFileSync(resolve(ROOT, 'src', f), 'utf8');
  const texts = [read('reflect.wgsl'), read('reflect_p5.wgsl'), read('compact/reflect_p5.wgsl')];
  const { scenePrelude, lookConfig, sunGlintConfig } = await import(`${ROOT}/src/render/frame.ts`);
  // 1. the prelude's word: true on the look's page with the switch up, false with the switch down, false off the look's page; the switch itself ships up, and
  //    its off word is wired: main.ts lowers it for ?nosunglint, a replay's visit keeps the word, and the look's page knows it for one of its own
  { const was = [lookConfig.on, sunGlintConfig.on]; let a: string | undefined, b: string | undefined, c: string | undefined;
    const word = (look: boolean, sw: boolean) => { lookConfig.on = look; sunGlintConfig.on = sw; return /const SUN_GLINT: bool = (true|false);/.exec(scenePrelude())?.[1]; };
    try { a = word(true, was[1] && !BRK); b = word(true, false); c = word(false, true); } finally { lookConfig.on = was[0]; sunGlintConfig.on = was[1]; }
    const wired = /if \(q\.has\('nosunglint'\)\) sunGlintConfig\.on = false;/.test(src('main.ts')) && /VISIT_FLAGS = \[[^\]]*'nosunglint'/.test(src('ui/replaykey.ts')) && /LOOK_PAGE_WORDS[^\n]*'nosunglint'/.test(src('scene/look.ts'));
    verdict(was[1] === true && a === 'true' && b === 'false' && c === 'false' && wired, 'the switch ships up; the prelude says the rule on the look\'s page with the switch as it ships, and nowhere else; and ?nosunglint is wired (the page lowers the switch, a replay\'s visit keeps the word, the look\'s page counts it its own)', `the switch ${was[1]} · look page ${a} · switch down ${b} · another page ${c} · the off word wired ${wired}`); }
  // 2. the text: the sun's head, its four numbers and rtGlints are ONE text in the three passes; without the word the sun's flag is a constant false and the
  //    lamps' skip is the old one; with it the sun is taken out of the contest before the four slots (never an unshadowed credit), has its own turn of the
  //    one shadow loop (no call site more), and is faded in and bounded by its own light
  { const cut = (s: string) => { const i = s.indexOf("// THE SUN'S GLINT"), k = s.indexOf('fn rtGlints('), j = s.indexOf('\n}\n', k); return i < 0 || k < 0 || j < 0 ? '' : s.slice(i, j); };
    const parts = texts.map(cut), a = parts[0], same = a.length > 0 && parts.every(x => x === a);
    const flag = a.includes('let far = SUN_GLINT && kind == 2u;') && a.includes('if ((kind == 2u && !far) || (!far && li.radius <= 0.0) || lightTransient(li)) { continue; }');
    const iFar = a.indexOf('    if (far) {'), iCont = a.indexOf('      continue;\n    }\n', iFar), iSlots = a.indexOf('if (w > g0.w) { rest += g3.spec;');
    const out = iFar > 0 && iCont > iFar && iSlots > iCont && a.slice(iFar, iCont).includes('if (g.w > gs.w) { gs = g; }') && !a.slice(iFar, iCont).includes('rest');
    const own = a.slice(iFar, iCont).includes('smoothstep(RT_SUN_CUT, 3.0 * RT_SUN_CUT, w / e) * min(1.0, RT_SUN_PEAK * e / w)') && a.includes('RT_SUN_ALPHA - min(ax, ay)') && a.includes('g.dist = clamp((WORLD_MAX.y - Po.y) / max(Ld.y, 0.02), 0.5, RT_SUN_FAR);');
    const turn = a.includes('for (var k = 0u; k < RT_GLINT_RAYS + 1u; k++) {') && a.includes('var g = g0; if (k == RT_GLINT_RAYS) { g = gs; } else if (k == 1u)') && (a.match(/occluded\(/g) ?? []).length === 1 && (a.match(/smokeTransmittance\(/g) ?? []).length === 1 && (a.match(/lightFogK\(/g) ?? []).length === 1;
    verdict(same && flag && out && own && turn, 'the three reflection passes hold one text of the sun\'s rules, its numbers and the highlights\' loop: without the word the flag is a constant false and the lamps\' skip the old one; the sun leaves the loop before the four slots and is never credited unshadowed; it is faded in, bounded by its own light, floored and short; and it has a turn of the one shadow loop, with no call site more of the walk, the smoke or the fog\'s classes',
      `one text in all three ${same} · the flag and the skip ${flag} · out of the contest ${out} · its own rules ${own} · its turn, one call site each ${turn}`); }
  // 3. the arithmetic, written again (rtGlints' own lines for a directional light): GGX D, height-correlated Smith, Schlick F in the face's frame, then the
  //    sun's fade and bound. Per unit of the sun's irradiance
  { const num = (name: string) => Number(new RegExp(`const ${name}: f32 = ([0-9.]+);`).exec(texts[0])?.[1] ?? NaN), CUT = num('RT_SUN_CUT'), FLOOR = num('RT_SUN_ALPHA'), PEAK = num('RT_SUN_PEAK');
    const lam = (s: number[], a: number) => 0.5 * (Math.sqrt(1 + a * a * (s[0] * s[0] + s[1] * s[1]) / Math.max(s[2] * s[2], 1e-6)) - 1);
    const D = (h: number[], a: number) => { const e = (h[0] * h[0] + h[1] * h[1]) / (a * a) + h[2] * h[2]; return 1 / (Math.PI * a * a * e * e); };
    const nrm = (v: number[]) => { const l = Math.hypot(v[0], v[1], v[2]); return [v[0] / l, v[1] / l, v[2] / l]; };
    const smooth = (a: number, b: number, x: number) => { const t = Math.min(1, Math.max(0, (x - a) / (b - a))); return t * t * (3 - 2 * t); };
    /** the sun's glint on a finish of roughness `rough` and normal reflectance f0: eye V and light L in the face's frame, disc angle `disc` */
    const glint = (V: number[], L: number[], rough: number, disc: number, f0 = 0.04) => {
      if (BRK || L[2] <= 0 || V[2] <= 0) return 0;
      const a0 = Math.max(rough * rough, 0.0009), a = Math.min(a0 + Math.max(0.5 * disc, FLOOR - a0), 1), H = nrm([V[0] + L[0], V[1] + L[1], V[2] + L[2]]), nv = Math.max(V[2], 0.05);
      const vis = 1 / (4 * nv * L[2] * (1 + lam([V[0], V[1], nv], a0) + lam(L, a)) + 1e-4);
      const vh = Math.min(1, Math.max(0, V[0] * H[0] + V[1] * H[1] + V[2] * H[2])), F = f0 + (1 - f0) * Math.pow(1 - vh, 5);
      const w = L[2] * F * D(H, a) * vis; return w < 1e-5 ? 0 : w * smooth(CUT, 3 * CUT, w) * Math.min(1, PEAK / w); };
    const disc = 0.012, eye = nrm([0, -0.57, 0.82]);   // the play camera's own height over a floor; the sun at its mirror, then turned away from it in steps
    const at = (deg: number) => { const m = [0, 0.57, 0.82], t = deg * Math.PI / 180; return nrm([Math.sin(t) * m[2], m[1], Math.cos(t) * m[2]]); };
    const peak = glint(eye, at(0), 0.2, disc), off10 = glint(eye, at(10), 0.2, disc), off60 = glint(eye, at(60), 0.2, disc), back = glint(eye, [0, 0.6, -0.8], 0.2, disc);
    const wall = 0.8 * 0.82 / Math.PI, rough45 = glint(eye, at(0), 0.45, disc), cloud = glint(eye, at(0), 0.45, 0.25);
    const graze = nrm([0, -0.995, 0.1]), rim = glint(graze, [0, 0.995, 0.1], 0.2, disc), steel = glint(eye, at(0), 0.25, disc, 0.9);
    let step = 0; for (let d = 0; d < 90; d += 0.25) step = Math.max(step, Math.abs(glint(eye, at(d + 0.25), 0.45, disc) - glint(eye, at(d), 0.45, disc)));   // the rim of a soft sheen: no jump
    verdict(CUT > 0 && FLOOR > 0 && PEAK > 0 && peak > off10 && off10 > off60 && off60 === 0 && back === 0 && peak > 3 * wall && rough45 > 0 && rough45 < peak && cloud < rough45 && rim <= PEAK + 1e-9 && steel <= PEAK + 1e-9 && rim > peak && step < 0.01,
      'a glint is brightest where the face looks half way between the eye and the sun and falls away to nothing; a face turned from the sun has none; on a smooth finish (0.2) it is several times a white wall and at the roughness cut (0.45) a soft sheen, softer still under cloud; a grazing eye and a metal stop at the sun\'s own bound; and the sheen\'s rim is a fade, not a step',
      `per unit of the sun: at the mirror ${peak.toFixed(2)} · 10° off ${off10.toFixed(3)} · 60° off ${off60} · turned away ${back} · a white wall ${wall.toFixed(2)} · at 0.45 ${rough45.toFixed(3)}, under cloud ${cloud.toFixed(4)} · a grazing eye ${rim.toFixed(2)}, steel ${steel.toFixed(2)} of the bound ${PEAK} · the sheen's largest step a quarter degree ${step.toFixed(4)} · cut ${CUT}, floor ${FLOOR}`); }
  console.log(all ? '=== sunglint: ALL PASS ===' : '=== sunglint: FAILURES above ===');
  return all;
}
if (import.meta.main) process.exit(await sunGlintCheck() ? 0 : 1);
