// THE MIRROR AND THE HIGHLIGHTS ARE TWO TERMS IN TWO TARGETS: the check behind reflect.wgsl outHi, common.wgsl RT_HI_FADE / rtHiFade and composite.wgsl
// upsampleHi (docs/agents/render.md has the rule at length).
//   bun run tools/qa/probes.ts highlights        (family `render`)
//   HIGHLIGHTS_BREAK=1     the highlights faded as the mirror is, at its cut-off, by a pixel's own roughness: must FAIL
//   PORTALMIRROR_BREAK=1   a ray that leaves by a lit opening brings its sky back whole, as it did: must FAIL
// The maintainer on his GPU, 11-04: "overcast dark room has specular sparkles". On the white roof's rolled skim (finish 0.45, High's cut-off 0.45) the lamp's
// highlight followed the floor's roughness map pixel for pixel (r = −0.72 in a still): the reflection term, rays and highlights in one texel, was faded to
// nothing over the 0.12 of roughness under the rays' cut-off by each pixel's own roughness, so a grain a twentieth smoother than its neighbour was lit and
// the neighbour was not.
// WHAT THIS CAN AND CANNOT SAY. No shader runs here. Line 1 holds the rule BY ITS TEXT in the three writings of the reflection pass, the two of the composite
// and the passes' layouts. Lines 2 to 7 are second writings of the shaders' arithmetic, each held to the shader by the text of the lines it writes again:
// the fade's pair and the page's word; a floor of grains under the old fade and the new; the texel's roughness and the nine-texel read; an opening counted
// once (a statement of the split and the text of its two halves: the sum is an identity, it cannot fail by itself); a metal's highlights. Whether a second writing is the first is as good as its pins. What the picture looks like is the
// page's to show.
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { ROOT } from './headless.ts';
export async function highlightsCheck(): Promise<boolean> {
  const BRK = process.env.HIGHLIGHTS_BREAK === '1', PB = process.env.PORTALMIRROR_BREAK === '1'; let all = true;
  const verdict = (ok: boolean, what: string, detail: string) => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}\n    ${detail}`); };
  console.log(`\n=== highlights: the mirror and the highlights are two terms in two targets${BRK ? ' (the highlights faded at the mirror\'s cut-off by a pixel\'s own roughness: must fail)' : PB ? ' (a doorway\'s sky whole in the mirror: must fail)' : ''} ===`);
  const src = (f: string) => readFileSync(resolve(ROOT, 'src', f), 'utf8'), sh = (f: string) => src(`render/shaders/${f}`);
  const count = (t: string, s: string) => t.split(s).length - 1;
  const ss = (e0: number, e1: number, x: number) => { const u = Math.min(Math.max((x - e0) / (e1 - e0), 0), 1); return u * u * (3 - 2 * u); };
  const passes = ['reflect.wgsl', 'reflect_p5.wgsl', 'compact/reflect_p5.wgsl'], comps = ['composite.wgsl', 'compact/composite.wgsl'];
  const cuts = [...src('ui/quality.ts').matchAll(/^  (ultra|high|medium):[^\n]*?rt: \{ on: true, div: \d+, rays: \d+, maxRough: ([0-9.]+),/gm)].map(x => [x[1], +x[2]] as [string, number]).sort((x, y) => x[1] - y[1]), high = cuts.findIndex(x => x[0] === 'high'), HI = high >= 0 ? cuts[high][1] : NaN;   // (the presets' cut-offs, off their table)
  // 1. the text
  const mainOf = (t: string) => { const k = t.indexOf('\nfn main('), j = t.indexOf('\n}\n', k); return k < 0 ? '' : t.slice(k, j < 0 ? undefined : j); };
  const heldP = passes.map(f => { const t = sh(f), m = mainOf(t), p5 = f.includes('p5'), tail = p5 ? ' if (noRec) { textureStore(hitOut, gid.xy, vec4u(0u)); } return; }' : ' return; }';
    return t.includes('@group(1) @binding(12) var outHi: texture_storage_2d<rgba16float, write>;') && m.length > 0
      && m.includes('  let hf = rtHiFade(maxR);') && m.includes('  if (rough > max(maxR, hf.y) + 0.15) { textureStore(outRefl, gid.xy, none); textureStore(outHi, gid.xy, vec4f(0.0));' + tail)
      && m.includes('    for (var j = 0u; j < rdiv * rdiv; j++) { let r = textureLoad(gMat, min(gid.xy * rdiv + vec2u(j % rdiv, j / rdiv), vec2u(frame.screen) - 1u), 0).x; if (abs(r - rough) < 0.15) { rs += r; rn += 1.0; } }\n    roughH = rs / max(rn, 1.0);')
      && m.includes('  let mirror = rough <= maxR; let lit = roughH <= hf.y;') && m.includes('  if (!mirror && !lit) { textureStore(outRefl, gid.xy, none); textureStore(outHi, gid.xy, vec4f(0.0, 0.0, 0.0, 1.0 + roughH));' + tail)
      && count(m, 'textureStore(outRefl, gid.xy, none); textureStore(outHi, gid.xy, vec4f(0.0));' + tail) === 4 && count(m, 'textureStore(outRefl, gid.xy, none);') === 5 && count(m, 'return;') === 6   // (five ways out with both targets written, and the one off the target's edge: no texel of either is left as the frame before wrote it)
      && m.includes('  let rays = select(0u, min(u32(frame.rtCfg.y), 4u), mirror);')
      && m.includes('  let kH = max(roughH * roughH, 0.0009) / alpha; let axh = ax * kH; let ayh = min(ay * kH, 1.0);')
      && m.includes('  textureStore(outRefl, gid.xy, select(none, vec4f(Lsum / f32(max(rays, 1u)), hitT0), mirror));\n  var Lhi = vec3f(0.0);\n  if (lit) {\n')
      && m.includes('Lhi += rtGlints(P, Po, N, V, T, B, axh, ayh, F0h, owner, select(1.0 - kEmit, 1.0, rays == 0u), ') && m.includes('Lhi += beamImage(Po, N, V, axh, ayh, F0h, ')
      && m.includes('  textureStore(outHi, gid.xy, vec4f(Lhi, 1.0 + roughH));') && !t.includes('Lspec') && t.includes('kAna: f32, pick: f32) -> vec3f {'); });
  const heldC = comps.map(f => { const t = sh(f);
    return t.includes('@group(1) @binding(18) var hiTex: texture_storage_2d<rgba16float, read>;')
      && t.includes('      var rs = mt.x; let hf = rtHiFade(maxR);\n      if (mt.x <= max(maxR, hf.y) + 0.1) { hi4 = upsampleHi(sPixf, z, Nl, mt.x); if (hi4.a >= 0.0) { rs = hi4.a; } }')
      && t.includes('      reflK = (1.0 - smoothstep(maxR - 0.12, maxR + 0.001, rs)) * frame.matCfg.y;\n      metalK = select(0.0, reflK * mt.y, frame.rtCfg.y >= 1.0);\n      hiK = (1.0 - smoothstep(hf.x, hf.y, rs)) * frame.matCfg.y;\n')
      && t.includes('      if (reflK > 0.0) { Lr += upsampleRefl(sPixf, z, Nl) * reflK; }\n      if (hiK > 0.0) { Lr += hi4.xyz * hiK; }\n      refl = lookShow(Lr * albedo4.a, shadeG); color += refl * glassT;')
      && count(t, 'upsampleHi(') === 2 && t.includes('fn upsampleHi(pixf: vec2f, z: f32, N: vec3f, r0: f32) -> vec4f {'); });   // (asked once: the debug view reads what the call read)
  const pt = src('render/passes.ts');
  const wired = pt.includes("{ binding: 12, visibility: CS, storageTexture: { access: 'write-only', format: 'rgba16float', viewDimension: '2d' } },   // out: the highlights (reflHi)")
    && pt.includes("{ binding: 18, visibility: FS, storageTexture: { access: 'read-only', format: 'rgba16float', viewDimension: '2d' } },") && count(pt, '{ binding: 18, resource: this.reflHiView }') === 2
    && pt.includes('{ binding: 12, resource: this.reflHiView } ];') && src('engine.ts').includes('K.spMaxRough = s.rtMaxRough;')
    && src('gpu/device.ts').includes("!lang.has('readonly_and_readwrite_storage_textures')");
  verdict(heldP.every(Boolean) && heldC.every(Boolean) && wired,
    'the three writings of the reflection pass write the rays to the mirror\'s target and the highlights to their own, each by its own test of the finish, the highlights made with the mean roughness of the texel\'s own block and that roughness handed on, and no way out of the pass leaves either target unwritten; both writings of the composite read the highlights once, fade both terms by the roughness that read hands back and add each by its own share; the passes bind the target to both, the denoiser\'s bound is the mirror\'s cut-off, and a browser whose shader language cannot read a storage texture is told so',
    [...passes.map((n, k) => `${n}: ${heldP[k] ? 'held' : 'NOT held'}`), ...comps.map((n, k) => `${n}: ${heldC[k] ? 'held' : 'NOT held'}`), `passes, engine and device: ${wired ? 'held' : 'NOT held'}`].join('; '));
  // 2. the pair and the page's word. The shader's own two lines are read; the prelude is asked for its constant with the word and without
  const cw = sh('common.wgsl'), pm = /^const RT_HI_FADE = vec2f\(([0-9.]+), ([0-9.]+)\);$/m.exec(cw), P0 = pm ? +pm[1] : NaN, P1 = pm ? +pm[2] : NaN;
  const fadeFn = cw.includes('fn rtHiFade(maxR: f32) -> vec2f { return mix(vec2f(maxR - 0.12, maxR + 0.001), RT_HI_FADE, RT_HI_REACH); }');
  const { highlightConfig, scenePrelude } = await import(`${ROOT}/src/render/frame.ts`);
  const was = highlightConfig.reach; let k1: string | undefined, k0: string | undefined; const word = (v: number) => { highlightConfig.reach = v; return /const RT_HI_REACH: f32 = ([0-9.]+);/.exec(scenePrelude())?.[1]; };
  try { k1 = word(1); k0 = word(0); } finally { highlightConfig.reach = was; }
  const mainTs = src('main.ts'), at = mainTs.indexOf("if (q.get('hireach') === '0') highlightConfig.reach = 0;"), eng = mainTs.indexOf('const engine = new Engine('), wiredWord = at > 0 && eng > at && /VISIT_FLAGS = \[[^\]]*'hireach'/.test(src('ui/replaykey.ts'));
  const REACH = BRK ? 0 : was, hiFade = (maxR: number): [number, number] => [maxR - 0.12 + (P0 - (maxR - 0.12)) * REACH, maxR + 0.001 + (P1 - (maxR + 0.001)) * REACH];   // (rtHiFade, written again)
  const D = (r: number) => 1 / (Math.PI * Math.pow(Math.max(r * r, 0.0009), 2));
  const [lo, hi] = hiFade(HI), peak = 0.04 * D(hi) * 0.25, paint = 0.5 / Math.PI;   // a lamp's highlight at its peak, seen and lit square on, on a paint of F0 0.04 (D ∕ 4), against what a paint of albedo 0.5 sends back of the same light
  verdict(cuts.length === 3 && fadeFn && was === 1 && k1 === '1.000' && k0 === '0.000' && wiredWord && hi - lo >= 0.2 && lo >= HI && hi <= 0.8 && peak < 0.1 * paint && cuts.every(c => { const f = hiFade(c[1]); return f[0] === lo && f[1] === hi; }),
    'one pair for every preset: a highlight is drawn whole on every finish High\'s mirror reaches and ends over a fifth of the roughness scale or more, where its peak is under a tenth of the light the paint beside it sends back; the page ships with that reach, and &hireach=0 (read before the engine is made, kept by a replay\'s visit) is the same path with the highlights fading where the mirror does',
    `the highlights fade from ${lo.toFixed(2)} to ${hi.toFixed(2)} on every preset (cut-offs for the mirror: ${cuts.map(x => `${x[0]} ${x[1]}`).join(', ')}); at ${hi.toFixed(2)} a highlight's peak is ${(100 * peak / paint).toFixed(1)}% of the paint's; the prelude's constant ${k1}, with the word ${k0}; wired ${wiredWord}`);
  // 3. A FLOOR OF GRAINS UNDER THE FADES. The white roof's skim on High: a finish of 0.45 whose every pixel is a grain somewhere from 0.38 to 0.51. A
  //    patch of it is made here, a texel's roughness taken as the pass takes it (the mean of its block of four, those within 0.15 of its own pixel's: line 4
  //    holds that loop's text), and the roughness a pixel is faded by read as the composite reads it (the nine texels by the spline, each by how like the
  //    pixel's own roughness its is). Then each pixel's share of the mirror and of the highlights, as it was (the pixel's own roughness, one fade) and now
  { let seed = 12345; const rnd = () => { seed = (Math.imul(seed, 1664525) + 1013904223) >>> 0; return seed / 4294967296; };
    const N = 48, px = Array.from({ length: N * N }, () => 0.38 + 0.13 * rnd()), T = N / 2, tex = new Float64Array(T * T);
    for (let ty = 0; ty < T; ty++) for (let tx = 0; tx < T; tx++) { const own = px[(2 * ty) * N + 2 * tx]; let s = 0, n = 0; for (let k = 0; k < 4; k++) { const r = px[(2 * ty + (k >> 1)) * N + 2 * tx + (k & 1)]; if (Math.abs(r - own) < 0.15) { s += r; n++; } } tex[ty * T + tx] = s / Math.max(n, 1); }
    const Wq = (d: number) => [0.5 * (0.5 - d) * (0.5 - d), 0.75 - d * d, 0.5 * (0.5 + d) * (0.5 + d)];
    const fin = (x: number, y: number): number => { const r0 = px[y * N + x], tu = (x + 0.5 - 0.5) / 2, tv = (y + 0.5 - 0.5) / 2, cx = Math.floor(tu + 0.5), cy = Math.floor(tv + 0.5), wx = Wq(tu - cx), wy = Wq(tv - cy); let a = 0, ws = 0;
      for (let k = 0; k < 9; k++) { const hx = cx + (k % 3) - 1, hy = cy + Math.floor(k / 3) - 1; if (hx < 0 || hy < 0 || hx >= T || hy >= T) continue; const r = tex[hy * T + hx], w = wx[k % 3] * wy[Math.floor(k / 3)] * (1 - ss(0.1, 0.2, Math.abs(r - r0))); a += r * w; ws += w; }
      return ws >= 0.2 ? a / ws : r0; };
    const fadeM = (r: number) => 1 - ss(HI - 0.12, HI + 0.001, r), fadeH = (r: number) => { const f = hiFade(HI); return 1 - ss(f[0], f[1], r); };
    const stat = (v: number[]) => { const m = v.reduce((a, b) => a + b, 0) / v.length; return { m, sd: Math.sqrt(v.reduce((a, b) => a + (b - m) * (b - m), 0) / v.length), lo: Math.min(...v), hi: Math.max(...v) }; };
    const inner: [number, number][] = []; for (let y = 4; y < N - 4; y++) for (let x = 4; x < N - 4; x++) inner.push([x, y]);
    const was = stat(inner.map(([x, y]) => fadeM(px[y * N + x]))), rsAll = inner.map(([x, y]) => (BRK ? px[y * N + x] : fin(x, y))), mir = stat(rsAll.map(fadeM)), hl = stat(rsAll.map(fadeH)), rsS = stat(rsAll);
    verdict(cuts.length === 3 && high >= 0 && was.lo === 0 && was.hi > 0.6 && was.sd > 0.15 && hl.lo > 0.999 && mir.sd < 0.25 * was.sd && rsS.sd < 0.012,
      'the fades draw no grain: on a floor whose every pixel is a grain of the white roof\'s skim, the old fade gave the pixels anything from none of the term to most of it, pixel by pixel; now every pixel has its highlights whole, and its share of the mirror (a finish at the cut-off itself has little of one) moves by under a quarter of what the term did',
      `the old share of the whole term: ${was.lo.toFixed(2)} to ${was.hi.toFixed(2)}, deviation ${was.sd.toFixed(3)} · the roughness a pixel is faded by now: ${rsS.lo.toFixed(3)} to ${rsS.hi.toFixed(3)}, deviation ${rsS.sd.toFixed(4)} (the grains': 0.38 to 0.51) · the highlights' share now ${hl.lo.toFixed(3)} to ${hl.hi.toFixed(3)} · the mirror's ${mir.lo.toFixed(2)} to ${mir.hi.toFixed(2)}, deviation ${mir.sd.toFixed(3)}`); }
  // 4. the texel's roughness and the nine-texel read, written again
  { const c0 = sh(comps[0]);
    const pins = comps.every(f => { const t = sh(f); return t.includes('  let c = floor(t + 0.5); let d = t - c; let di = i32(rdiv); let gi = i32(goff);')
      && t.includes('  var wx = vec3f(0.5 * (0.5 - d.x) * (0.5 - d.x), 0.75 - d.x * d.x, 0.5 * (0.5 + d.x) * (0.5 + d.x));') && t.includes('  var wy = vec3f(0.5 * (0.5 - d.y) * (0.5 - d.y), 0.75 - d.y * d.y, 0.5 * (0.5 + d.y) * (0.5 + d.y));')
      && t.includes('    let o = vec2i(k % 3, k / 3); let h = vec2i(c) + o - 1;') && t.includes('    let hi = textureLoad(hiTex, h);\n    if (hi.a <= 0.0) { continue; }')
      && t.includes('    let wf = wx[o.x] * wy[o.y] * (1.0 - smoothstep(0.1, 0.2, abs(hi.a - 1.0 - r0)));') && t.includes('    let w = wf * exp(-dz * dz) * pow(max(dot(textureLoad(gNormal, gpx(src), 0).xyz * 2.0 - 1.0, N), 0.0), 8.0);\n    acc += vec4f(hi.xyz, hi.a - 1.0) * w; wsum += w;') && t.includes('  return vec4f(acc.xyz / max(wsum, 0.2), select(-1.0, acc.w / max(wsum, 1e-9), wsum >= 0.2));\n}'); }) && c0.length > 0;
    const W = (d: number) => [0.5 * (0.5 - d) * (0.5 - d), 0.75 - d * d, 0.5 * (0.5 + d) * (0.5 + d)];
    const ds = [-0.5, -0.31, -0.1, 0, 0.2, 0.44, 0.5], sums = ds.map(d => W(d).reduce((a, b) => a + b, 0)), least = Math.min(...ds.flatMap(W));
    // one row of texels, a highlight with a hard shadow's edge in it (1, 1, 1, 0, 0, 0), read at every tenth of a texel: a ramp with no step in it
    const row = [1, 1, 1, 0, 0, 0], read = (x: number) => { const c = Math.floor(x + 0.5), w = W(x - c); return w[0] * row[c - 1] + w[1] * row[c] + w[2] * row[c + 1]; };
    const xs = Array.from({ length: 31 }, (_, k) => 1 + k * 0.1), vals = xs.map(read), steps = vals.slice(1).map((v, k) => vals[k] - v), mono = steps.every(s => s >= -1e-12), big = Math.max(...steps);
    const like = (texel: number, r0: number) => 1 - ss(0.1, 0.2, Math.abs(texel - r0));
    // the pass's block: a texel of four pixels of the skim, one of them another finish's
    const block = (px: number[], own: number) => { let s = 0, n = 0; for (const r of px) if (Math.abs(r - own) < 0.15) { s += r; n++; } return s / Math.max(n, 1); };
    const grain = block([0.38, 0.51, 0.44, 0.47], 0.51), seam = block([0.1, 0.12, 0.9, 0.9], 0.1), alone = block([0.9, 0.9, 0.9, 0.3], 0.3);
    const drawnOf = (acc: number, wsum: number) => acc / Math.max(wsum, 0.2), sliver = drawnOf(1 * 1e-4, 1e-4), corner = drawnOf(1 * 0.3, 0.3);   // (a floor's highlight of 1 read from a table's edge over it, and from the floor's own corner)
    verdict(pins && sums.every(s => Math.abs(s - 1) < 1e-12) && least >= 0 && mono && big < 0.13 && Math.abs(vals[0] - 1) < 1e-12 && Math.abs(vals[30]) < 1e-12 && like(0.44, 0.51) === 1 && like(0.1, 0.45) === 0 && Math.abs(grain - 0.45) < 1e-9 && Math.abs(seam - 0.11) < 1e-9 && alone === 0.3 && sliver < 0.001 && corner === 1,
      'a texel\'s highlights are made with its block\'s mean roughness of its own finish, and a pixel reads them off nine texels by weights that sum to one and are never negative, by which a hard edge on the texels is a ramp two texels wide with no step over an eighth; a texel of another finish in the same plane has no say, and a sliver of a neighbour\'s highlight is not scaled up to a whole one',
      `the spline's sums ${sums.map(s => s.toFixed(3)).join(' ')} · its least weight ${least.toFixed(3)} · the edge read at every tenth of a texel: from ${vals[0].toFixed(2)} to ${vals[30].toFixed(2)}, never rising, largest step ${big.toFixed(3)} · a grain 0.07 off its finish counts ${like(0.44, 0.51)}, a polish beside satin ${like(0.1, 0.45)} · the skim's block ${grain.toFixed(3)}, a polish at a matte wall's foot ${seam.toFixed(2)}, a texel alone in its block ${alone} · a floor's highlight read from a table's edge over it ${sliver.toFixed(4)}, from the floor's own corner ${corner}`); }
  // 5. AN OPENING'S SKY IS IN THE MIRROR OR IN THE HIGHLIGHTS, NEVER WHOLE IN BOTH (the text of the two halves; the sum below is the split said as arithmetic and
  //    is 1 by construction: what can fail here is a missed ray's line, in any of its four writings). The sky a ray leaves by a lit opening: the share of it the portal's light covers
  //    (common.wgsl skyKeep: 1 − its window) goes to the highlights on a finish that takes its lit panels from their analytic image (1 − kEmit), and the mirror
  //    keeps the rest. Written again from the shaders' two lines
  { const sites = passes.map(n => count(sh(n), 'skyRadiance(dir) * (1.0 - (1.0 - kEmit) * (1.0 - skyKeep(Po, dir))) + frame.weatherB.xyz;')), plain = passes.map(n => count(sh(n), 'skyRadiance(dir) + frame.weatherB.xyz;'));
    const keepLine = cw.includes('      keep = 1.0 - lightWindow(distance(O, lights[i].pos), lights[i].range); break;') && passes.every(n => sh(n).includes('fn emitTraceK(rough: f32) -> f32 { return 1.0 - smoothstep(0.2, 0.3, rough); }'));
    const kEmit = (r: number) => 1 - ss(0.2, 0.3, r), mirror = (r: number, w: number) => (PB ? 1 : 1 - (1 - kEmit(r)) * (1 - (1 - w))), hl = (r: number, w: number) => (1 - kEmit(r)) * w;   // (w = the portal's window at the point: what its analytic image is drawn by)
    const rows = [0.1, 0.25, 0.3, 0.45].flatMap(r => [0, 0.4, 1].map(w => mirror(r, w) + hl(r, w)));
    verdict(sites[0] === 1 && sites[1] === 2 && sites[2] === 1 && plain.every(v => v === 0) && keepLine && rows.every(v => Math.abs(v - 1) < 1e-9),
      'the sky a ray leaves by an opening that is a light is counted once between the two terms: every writing of a missed ray takes off the share the portal\'s light covers, by the finish\'s share of analytic image, and the two add up to the sky on a polish and on a satin floor, by the door and past the reach of its light',
      `the missed ray's line: ${sites.join(', ')} of 1, 2, 1 (none left plain: ${plain.every(v => v === 0)}) · the shares at finishes 0.1, 0.25, 0.3, 0.45 × the light's window 0, 0.4, 1: ${rows.map(v => v.toFixed(2)).join(' ')}`); }
  // 6. A METAL'S HIGHLIGHTS ARE A CONDUCTOR'S ONLY AS FAR AS ITS PAINT IS STEPPED BACK (reflect.wgsl F0h; composite.wgsl metalK: the same fade, strength and
  //    test of rays, written again from both)
  { const pin = passes.every(n => sh(n).includes('  let F0h = mix(vec3f(0.04), base, metalness * select(0.0, (1.0 - smoothstep(maxR - 0.12, maxR + 0.001, roughH)) * min(frame.matCfg.y, 1.0), frame.rtCfg.y >= 1.0));'))
      && comps.every(n => sh(n).includes('      reflK = (1.0 - smoothstep(maxR - 0.12, maxR + 0.001, rs)) * frame.matCfg.y;\n      metalK = select(0.0, reflK * mt.y, frame.rtCfg.y >= 1.0);'));
    const stepK = (r: number, m: number, strength: number, rays: number) => (rays >= 1 ? (1 - ss(m - 0.12, m + 0.001, r)) * Math.min(strength, 1) : 0);   // (the share of a metal's paint that is stepped back: metalK ∕ metalness)
    const f0 = (base: number, metal: number, r: number, m: number, strength = 1, rays = 1) => 0.04 + (base - 0.04) * metal * stepK(r, m, strength, rays), MED = cuts[0]?.[1] ?? NaN;
    const polished = f0(0.7, 1, 0.2, MED), satin = f0(0.7, 1, 0.5, MED), paintOn = f0(0.7, 0, 0.2, MED), noRays = f0(0.7, 1, 0.2, MED, 1, 0), halfStrength = f0(0.7, 1, 0.2, MED, 0.5), peakSatin = satin * D(0.5) * 0.25, lambert = 0.7 / Math.PI;
    verdict(pin && Math.abs(polished - 0.7) < 1e-9 && satin === 0.04 && paintOn === 0.04 && noRays === 0.04 && Math.abs(halfStrength - 0.37) < 1e-9 && peakSatin < 0.25 * lambert,
      'a metal under the mirror\'s cut-off has a conductor\'s highlights and a stepped-back paint, as it had; where its paint stands in whole for the metal (past the cut-off, or with no ray traced) its highlights are a coat\'s, and at half the reflections\' strength half way: at a conductor\'s strength a lamp\'s highlight on a satin steel on Medium was more than the paint under it',
      `F0 of a steel of 0.7 on Medium: ${polished.toFixed(2)} at finish 0.2, ${satin.toFixed(2)} at 0.5, ${noRays.toFixed(2)} with no ray traced, ${halfStrength.toFixed(2)} at half strength · a paint: ${paintOn.toFixed(2)} · a lamp's highlight at its peak on the satin steel: ${(100 * peakSatin / lambert).toFixed(0)}% of its paint's light (at a conductor's strength ${(100 * 0.7 * D(0.5) * 0.25 / lambert).toFixed(0)}%)`); }
  console.log(all ? '=== highlights: ALL PASS ===' : '=== highlights: FAILED ===');
  return all;
}
if (import.meta.main) process.exit((await highlightsCheck()) ? 0 : 1);
