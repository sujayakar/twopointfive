// THE SUN'S GLINT: the check behind the scene prelude's SUN_GLINT (src/render/frame.ts sunGlintConfig; on on the look's page, ?nosunglint at boot = as it
// was; no game table, nothing for a recording to carry).
//   bun run tools/qa/probes.ts sunglint        (family `render`)        SUNGLINT_BREAK=1 = the rule off and the same lines asked: must FAIL
//   Also here, since it is the same function's text: NO LAMP'S HIGHLIGHT IS CREDITED UNSEEN (the prelude's GLINT_PICK, lines 4 and 5 below; GLINTPICK_BREAK=1
//   asks the pick's lines of the half credit it replaces: must FAIL)
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
  // 4. NO LAMP'S HIGHLIGHT IS CREDITED UNSEEN (the prelude's GLINT_PICK, render/frame.ts glintPickConfig; the look's page, &glintpick=1, off unless asked;
  //    GLINTPICK_BREAK=1 = the half credit asked the same lines: must FAIL). The word, then the text in the three passes
  { const { glintPickConfig } = await import(`${ROOT}/src/render/frame.ts`); const was = [lookConfig.on, glintPickConfig.on]; let a: string | undefined, b: string | undefined, c: string | undefined;
    const word = (look: boolean, sw: boolean) => { lookConfig.on = look; glintPickConfig.on = sw; return /const GLINT_PICK: bool = (true|false);/.exec(scenePrelude())?.[1]; };
    try { a = word(true, true); b = word(true, false); c = word(false, true); } finally { lookConfig.on = was[0]; glintPickConfig.on = was[1]; }
    const wired = /if \(q\.get\('glintpick'\) === '1'\) glintPickConfig\.on = true;/.test(src('main.ts')) && /VISIT_FLAGS = \[[^\]]*'glintpick'/.test(src('ui/replaykey.ts')) && /LOOK_PAGE_WORDS[^\n]*'glintpick'/.test(src('scene/look.ts'));
    const cut = (t: string) => { const k = t.indexOf('fn rtGlints('), j = t.indexOf('\n}\n', k); return k < 0 || j < 0 ? '' : t.slice(k, j); }, f = texts.map(cut), t0 = f[0];
    const lines = t0.includes('var poolW = 0.0; var pu = pick;')
      && t0.includes('      var c = g;\n      if (w > g0.w) { c = g2; g2 = g1; g1 = g0; g0 = g; } else if (w > g1.w) { c = g2; g2 = g1; g1 = g; } else if (w > g2.w) { c = g2; g2 = g; }\n      if (c.w > 0.0) { rest += c.spec; let first = poolW == 0.0; poolW += c.w; if (first) { g3 = c; } else { let p = c.w / poolW; if (pu < p) { g3 = c; pu = pu / p; } else { pu = (pu - p) / (1.0 - p); } } }\n      continue;\n    }\n    if (w > g0.w) { rest += g3.spec;')
      && t0.includes('  if (GLINT_PICK) { g3.spec = rest; rest = vec3f(0.0); }') && t0.includes('  var G = rest * 0.5;') && t0.includes('else if (k == 3u) { g = g3; }') && t0.includes('G += g.spec * tr;') && texts.every(x => /\nconst RT_GLINT_RAYS: u32 = 4u;/.test(x)) && (t0.match(/if \(GLINT_PICK/g) ?? []).length === 2 && (t0.match(/rnd\(\)|hash13\(/g) ?? []).length === 0;
    const call = texts.every(t => (t.match(/rtGlints\(P, Po, N, V, T, B, axh, ayh, F0h, owner, select\(1\.0 - kEmit, 1\.0, rays == 0u\), fract\(cpx \+ f32\(frame\.frameIdx % 4096u\) \* 0\.7548777\)\) \* frame\.matCfg\.x;/g) ?? []).length === 1 && (t.match(/let cpx = rnd\(\); let cpy = rnd\(\);/g) ?? []).length === 1 && /rngInit\(gid\.xy, 0x51f7u\);\n  let cpx = rnd\(\)/.test(t));
    verdict(was[1] === false && a === 'true' && b === 'false' && c === 'false' && wired && f.every(x => x.length > 0 && x === t0) && lines && call,
      'the pick ships DOWN: the prelude says it only on the look\'s page with the word, and &glintpick=1 is wired (the page raises the switch before the first pipeline, a replay\'s visit keeps the word, the look\'s page counts it its own); the three passes hold one text of it: three slots by strength, what leaves them or never joined them summed and one of it kept by its weight with ONE number handed on (the first without asking, and no division of a weight by itself), that one\'s ray answering for the sum, in front of the lines as they were; the number is the caller\'s, the pixel\'s own that is the same every frame stepped by the golden ratio, and no draw is added to the kernel\'s stream',
      `the switch as shipped ${was[1]} · look page with the word ${a} · without ${b} · another page ${c} · wired ${wired} · one text ${f.every(x => x === t0)} · its lines ${lines} · the caller's number ${call}`); }
  // 5. the pick's arithmetic, written again as the shader has it, in colour (a lamp = its highlight's rgb, its weight the luminance, and whether it is seen).
  //    The rule as it was: the four strongest by their rays, the rest at half unseen. With the word: three by their rays, the rest summed, one of the rest
  //    kept by its weight (the one number, handed on), and the sum drawn if that one is seen. The number over the frames is the shader's own: a pixel's
  //    fixed number stepped by 0.7548777 a frame, in single floats
  { const PICK = process.env.GLINTPICK_BREAK !== '1', F = Math.fround; type Lamp = { c: number[]; v: boolean }; const lum = (c: number[]) => 0.2126 * c[0] + 0.7152 * c[1] + 0.0722 * c[2];
    const add = (a: number[], b: number[], k = 1) => [a[0] + k * b[0], a[1] + k * b[1], a[2] + k * b[2]];
    const was = (lamps: Lamp[]) => { const s = [...lamps].sort((p, q) => lum(q.c) - lum(p.c)); let G = [0, 0, 0]; s.slice(4).forEach(l => { G = add(G, l.c, 0.5); }); s.slice(0, 4).forEach(l => { if (l.v) G = add(G, l.c); }); return G; };
    const now = (lamps: Lamp[], u: number) => { if (!PICK) return was(lamps);
      const g: (Lamp | null)[] = [null, null, null]; let pick: Lamp | null = null, poolW = 0, pu = u, rest = [0, 0, 0]; const W = (l: Lamp | null) => l ? lum(l.c) : 0;
      for (const l of lamps) { let c: Lamp | null = l; const w = W(l);
        if (w > W(g[0])) { c = g[2]; g[2] = g[1]; g[1] = g[0]; g[0] = l; } else if (w > W(g[1])) { c = g[2]; g[2] = g[1]; g[1] = l; } else if (w > W(g[2])) { c = g[2]; g[2] = l; }
        if (c && W(c) > 0) { rest = add(rest, c.c); const first = poolW === 0; poolW += W(c); if (first) pick = c; else { const p = W(c) / poolW; if (pu < p) { pick = c; pu = pu / p; } else pu = (pu - p) / (1 - p); } } }
      let G = [0, 0, 0]; for (const x of g) if (x && x.v) G = add(G, x.c); if (pick && pick.v) G = add(G, rest); return G; };
    const frames = (h: number, n: number) => Array.from({ length: n }, (_, k) => { const x = F(F(h) + F(F(k % 4096) * F(0.7548777))); return F(x - Math.floor(x)); });   // (fract(cpx + f32(frameIdx % 4096) * 0.7548777), as the caller writes it)
    const mean = (lamps: Lamp[], h: number, n: number) => { let t = [0, 0, 0]; for (const u of frames(h, n)) t = add(t, now(lamps, u), 1 / n); return t; }, truth = (lamps: Lamp[]) => lamps.reduce((t, l) => l.v ? add(t, l.c) : t, [0, 0, 0]);
    const grey = (w: number, v = true): Lamp => ({ c: [w, w, w], v }), err = (a: number[], b: number[]) => Math.abs(lum(a) - lum(b)) / Math.max(lum(b), 1e-9), eq = (a: number[], b: number[]) => a.every((x, i) => x === b[i]);
    // four or fewer: every order of four, a tie among them, and fewer: the same numbers as before, and the truth
    const perms = (a: Lamp[]): Lamp[][] => a.length <= 1 ? [a] : a.flatMap((x, i) => perms([...a.slice(0, i), ...a.slice(i + 1)]).map(r => [x, ...r]));
    const fourSets: Lamp[][] = [[grey(0.9), grey(0.2, false), grey(0.5), grey(0.05)], [grey(0.5), grey(0.5, false), grey(0.5), grey(0.2)], [{ c: [1, 0.5, 0.1], v: true }, { c: [0.2, 0.3, 1], v: true }, grey(0.3, false)], [grey(0.7)]];
    let same = true, tried = 0; for (const set of fourSets) for (const o of perms(set)) for (const u of [0, 0.37, 0.999]) { tried++; const n = now(o, u), t = truth(o); if (Math.abs(lum(n) - lum(t)) > 1e-12 || Math.abs(lum(was(o)) - lum(t)) > 1e-12) same = false; }
    const sodium = [1.0, 0.55, 0.12], white = [0.9, 0.9, 0.9], blue = [0.3, 0.45, 1.0], k = (c: number[], s: number) => c.map(x => x * s);
    const hall: Lamp[] = [1, 0.8, 0.6].map(w => grey(w)).concat([{ c: k(sodium, 0.5), v: true }, { c: k(white, 0.4), v: true }, { c: k(blue, 0.3), v: true }]);   // six in plain sight, the rest in three colours
    const wall: Lamp[] = hall.map((l, i) => i === 3 || i === 5 ? { c: l.c, v: false } : l);                                                            // the two weakest of them behind a wall
    const mixed: Lamp[] = [grey(0.3, false), grey(1), { c: k(blue, 0.05), v: true }, { c: k(sodium, 0.6), v: false }, grey(0.4), grey(0.8), { c: k(white, 0.5), v: false }, grey(0.2)];
    const steady = [0.03, 0.31, 0.62, 0.97].every(u => eq(now(hall, u), now(hall, 0.5))) && err(now(hall, 0.5), truth(hall)) < 1e-12 && Math.max(...now(hall, 0.5).map((x, i) => Math.abs(x - truth(hall)[i]))) < 1e-12;
    const hs = [0.0, 0.123, 0.5, 0.777, 0.9999], long = [hall, wall, mixed].map(l => Math.max(...hs.map(h => err(mean(l, h, 4096), truth(l))))), short = Math.max(...hs.map(h => err(mean(wall, h, 16), truth(wall))));
    const turned = Math.max(...hs.map(h => err(mean([...mixed].reverse(), h, 4096), truth(mixed)))), old = [hall, wall].map(l => (lum(was(l)) - lum(truth(l))) / lum(truth(l)));
    verdict(same && steady && long.every(x => x < 5e-3) && turned < 5e-3 && short < 0.12 && old[0] < -0.05 && old[1] > 0.05,
      'four lamps or fewer, in every order and with a tie: the highlight is what it was, and what the lamps truly give; more than four: with all in sight every frame is the same frame, colours and all, and the whole sum where the half was short; with some hidden the frames\' mean by the shader\'s own numbers is what the seen lamps give, in any order, where the half let a hidden lamp glint',
      `four or fewer the same over ${tried} orders and numbers ${same} · six in sight steady and whole ${steady} · the mean's error over 4096 frames (the worst of ${hs.length} pixels): six in sight ${(long[0] * 100).toFixed(2)}%, two hidden ${(long[1] * 100).toFixed(2)}%, eight mixed ${(long[2] * 100).toFixed(2)}% (turned round ${(turned * 100).toFixed(2)}%) · two hidden over 16 frames ${(short * 100).toFixed(1)}% · the half's error: six in sight ${(old[0] * 100).toFixed(1)}%, two hidden +${(old[1] * 100).toFixed(1)}%`); }
  console.log(all ? '=== sunglint: ALL PASS ===' : '=== sunglint: FAILURES above ===');
  return all;
}
if (import.meta.main) process.exit(await sunGlintCheck() ? 0 : 1);
