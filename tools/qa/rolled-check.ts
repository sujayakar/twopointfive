// A PRESSED THING'S ARRISES ARE ROLLED, AND A LACQUER MIRRORS MORE THAN PAINT: the check behind lookTex.ts STUFFS' `rolled` and `mirrors`, look.wgsl
// lookRoll / lookRollUp and the reflection pass's `coat` (docs/agents/render.md has the rule at length).
//   bun run tools/qa/probes.ts rolled        (family `render`)
//   ROLLED_BREAK=1   no stuff says either word (the second writing below is handed a table with neither): must FAIL
// The maintainer, of the cars: "we don't need to go full hyperrealistic but we should sell it more", and "how reflective is the car material, btw? it
// does feel like it should reflect more of the environment and be quite dynamic as we move around it". Drawn on a GPU, a stronger and smoother coat
// alone changed nothing, and a car drawn as a FULL mirror barely changed either: the game's camera looks down at about 50 degrees, so a flat top mirrors
// the empty sky 50 degrees up and a flat flank the ground 50 degrees down, and nothing a car is seen to reflect (the horizon: walls, lamps, a man's
// torch) is in either. A real panel has a rolled shoulder between the two; a stack of boxes has a knife edge.
// WHAT THIS CAN AND CANNOT SAY. No shader runs here. Line 1 is the table, read. Line 2 is the rule written a second time (lookRoll, the Edges row's
// rest, lookSurface's hold on the summed tilt and its dead zone, the guard that keeps a rolled normal toward the eye, lookRollUp) and asked. Line 3
// holds the shaders, the two preludes and the uniform's writer to that writing BY THEIR TEXT, so it is the first as far as those pins go. What a
// rolled arris LOOKS like, and what it catches from the game's camera, is a page's to show: the arithmetic in line 2's detail is said, not barred.
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { ROOT } from './headless.ts';
export async function rolledCheck(): Promise<boolean> {
  const BRK = process.env.ROLLED_BREAK === '1'; let all = true;
  const verdict = (ok: boolean, what: string, detail: string) => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}\n    ${detail}`); };
  console.log(`\n=== rolled: a pressed thing's arrises are rolled, and a lacquer mirrors more than paint${BRK ? ' (no stuff says either: must fail)' : ''} ===`);
  const text = (f: string) => readFileSync(resolve(ROOT, f), 'utf8'), flat = (t: string) => t.replace(/\/\/[^\n]*/g, '').replace(/\s+/g, ' ').trim();
  const T = await import(`${ROOT}/src/render/lookTex.ts`);
  const NAMES: string[] = T.LOOK_NAMES, ROLLED: number[] = BRK ? NAMES.map(() => 0) : T.LOOK_ROLLED, MIRRORS: number[] = BRK ? Array.from({ length: 32 }, () => 0.04) : T.LOOK_MIRRORS;
  const look = text('src/render/shaders/look.wgsl'), MAX = Number(/const LOOK_MAX_TILT: f32 = ([0-9.]+);/.exec(look)?.[1]);
  // 1. the table
  { const { LOOK_WORD } = await import(`${ROOT}/src/render/gbuffer.ts`);
    const word = T.lookStuffOf('carpaint') as number, layer = NAMES.indexOf('carpaint');
    const says = NAMES.filter((_, l) => ROLLED[l] > 0), shines = MIRRORS.map((m, w) => m !== T.LOOK_MIRROR_PLAIN ? w : -1).filter(w => w >= 0);
    const rest = MIRRORS.every((m, w) => w === word || m === T.LOOK_MIRROR_PLAIN), whole = (v: number) => v === +v.toFixed(4);   // (the prelude writes four places: a number with more would not be the shader's)
    verdict(ROLLED.length === NAMES.length && MIRRORS.length === T.LOOK_WORDS && T.LOOK_WORDS === LOOK_WORD.stuffs && T.LOOK_MIRROR_PLAIN === 0.04 && layer >= 0 && word === layer + 1 && ROLLED[layer] >= 0.08 && ROLLED[layer] <= 0.3
      && MIRRORS[word] > 0.04 && MIRRORS[word] <= 0.2 && rest && ROLLED.every(r => r >= 0 && r <= 0.3) && MIRRORS.every(m => m >= 0.02 && m <= 0.2 && whole(m)),
      'car paint says its arrises are rolled over a hand or more and its lacquer mirrors more than a plain coat; every other word a box may say, and no word, reads the plain coat',
      `rolled: ${says.map(n => `${n} ${ROLLED[NAMES.indexOf(n)]} m`).join(', ') || 'no stuff'} of ${NAMES.length} · mirrors: ${shines.map(w => `${NAMES[w - 1]} ${MIRRORS[w]}`).join(', ') || 'no stuff'}, the other ${MIRRORS.length - shines.length} of the word's ${T.LOOK_WORDS} the plain coat's ${T.LOOK_MIRROR_PLAIN} (${rest}) · every roll at most 0.3 m, every mirror in 0.02 to 0.2 and written whole in four places: a lacquer, not a metal`); }
  // the rule, a second time: look.wgsl lookRoll under the Edges row as the page opens, then lookSurface's dead zone and its hold on the summed tilt
  const { LOOK_DEFAULTS } = await import(`${ROOT}/src/render/gbuffer.ts`);
  const MIN = Number(/const LOOK_MIN_TILT: f32 = ([0-9.]+);/.exec(look)?.[1]), ROW = Math.min(2 * LOOK_DEFAULTS.edges * LOOK_DEFAULTS.clarity, 1);   // (lookCfg.b.y = edges × clarity, gbuffer.ts setLook)
  const c01 = (v: number) => Math.max(0, Math.min(1, v));
  const roll = (r: number, p: [number, number], h: [number, number], up: [number, number]): [number, number] => {
    if (r <= 0) return [0, 0];
    const e: [number, number] = [0, 0];
    for (let a = 0; a < 2; a++) e[a] = Math.abs(up[a]) > 0.5 ? c01(1 - (h[a] - p[a] * Math.sign(up[a])) / Math.min(r, 2 * h[a])) * Math.sign(up[a]) : c01(1 - (h[a] - Math.abs(p[a])) / Math.min(r, h[a])) * Math.sign(p[a]);
    return [e[0] * MAX * ROW, e[1] * MAX * ROW]; };
  /** the sine the surface's normal leans by, of a tilt (lookSurface: nothing under LOOK_MIN_TILT, never over LOOK_MAX_TILT) */
  const held = (e: [number, number]) => { const l = Math.hypot(e[0], e[1]); return l > MIN ? Math.min(l, MAX) : 0; };
  const R = ROLLED[NAMES.indexOf('carpaint')] ?? 0, deg = (s: number) => Math.asin(Math.max(-1, Math.min(1, s))) * 180 / Math.PI;
  // 2. the rule's shape
  { const H: [number, number] = [0.86, 0.25];   // a flank: 1.72 m long, half a metre tall, its second axis upright
    let down = 0, mono = true, jump = 0, last = roll(R, [0, -H[1]], H, [0, 1])[1], n = 0;
    for (let y = -H[1]; y <= H[1] + 1e-9; y += 0.002) { const e = roll(R, [0, y], H, [0, 1])[1]; n++; if (e < 0) down++; if (e < last - 1e-9) mono = false; jump = Math.max(jump, Math.abs(held([0, e]) - held([0, last]))); last = e; }
    const top = roll(R, [0, H[1]], H, [0, 1])[1], inR = roll(R, [0, H[1] - R], H, [0, 1])[1], foot = roll(R, [0, -H[1]], H, [0, 1])[1];
    const corner = roll(R, [H[0], 0], H, [0, 1])[0], mid = roll(R, [0, 0], H, [0, 1]);
    // a top: odd about its middle on both axes, whole at each edge, nothing in the middle of a wide one; at its corner two rolls meet and the sum is held to the look's own lean
    const T0: [number, number] = [0.6, 0.78], te = roll(R, [T0[0], 0], T0, [0, 0])[0], tw = roll(R, [-T0[0], 0], T0, [0, 0])[0], tm = roll(R, [0, 0], T0, [0, 0]), tc = roll(R, [T0[0], T0[1]], T0, [0, 0]);
    // a rail thinner than the roll: rolled across all of it, flat on its crest alone; and a slab shorter than the roll, upright: all of it, to nothing at its foot
    const rail = roll(R, [0.015, 0], [0.03, 1], [0, 0])[0], crest = roll(R, [0, 0], [0.03, 1], [0, 0])[0];
    const slabFoot = roll(R, [0, -0.04], [1, 0.04], [0, 1])[1], slabTop = roll(R, [0, 0.04], [1, 0.04], [0, 1])[1];
    // the guard (lookSurface): a roll that would turn the normal from the eye is taken back until the normal keeps a twentieth toward it, whatever the view
    const cosMax = Math.sqrt(1 - MAX * MAX); let worst = 1, asked = 0;
    for (let nv = 0.02; nv <= 1; nv += 0.02) for (let tv = -Math.sqrt(1 - nv * nv); tv <= 0; tv += 0.02) for (const w of [0.25, 0.5, 1]) {   // (nv: the face toward the eye; tv: the roll's own direction toward it; w: how far into the roll)
      let e = MAX * ROW * w; const away = e * tv; if (away < 0) e *= c01((cosMax * nv - 0.05) / -away);
      const s = held([e, 0]); worst = Math.min(worst, (Math.sqrt(1 - s * s) * nv + s * tv) / Math.min(nv, 0.05)); asked++; }
    // lookRollUp, on an upright cut plane: at most the look's lean of up, so a normal that had none stays under the fog of war's test of a top
    const upMost = Math.max(...[0, 0.05, 0.1, 0.2, 0.25].map(d => roll(R, [H[1] - d, 0], [H[1], 1], [1, 1])[0]));
    const lean = deg(held([0, top])), EYE = 50;
    verdict(Math.abs(MAX - 0.48) < 1e-9 && MIN > 0 && MIN <= 0.02 && ROW === 1 && R > 0 && down === 0 && mono && jump <= MIN + 0.007 && Math.abs(top - MAX) < 1e-9 && Math.abs(inR) < 1e-9 && foot === 0 && Math.abs(corner - MAX) < 1e-9 && mid[0] === 0 && mid[1] === 0
      && Math.abs(te - MAX) < 1e-9 && Math.abs(tw + MAX) < 1e-9 && tm[0] === 0 && tm[1] === 0 && Math.hypot(tc[0], tc[1]) > MAX && Math.abs(held(tc) - MAX) < 1e-9 && Math.abs(rail - MAX / 2) < 1e-9 && crest === 0 && slabFoot === 0 && Math.abs(slabTop - MAX) < 1e-9
      && worst >= 1 - 1e-9 && upMost <= MAX + 1e-9 && upMost < 0.5,
      'a flank turns up into its top and round an upright arris and never down at its foot; a top turns out at every edge; a thin rail or slab is rolled all across; the roll is whole as the page opens, held to the look\'s own lean, and never turns a face from the eye',
      `the look lets a normal lean ${deg(MAX).toFixed(1)}° (LOOK_MAX_TILT ${MAX}, nothing under ${MIN}); the Edges row as the page opens gives the roll ${ROW} of itself · a flank ${2 * H[1]} m tall, rolled ${R} m: ${lean.toFixed(1)}° at its top, 0 from ${R} m down, 0 at its foot, ${n} heights asked: ${down} turn down, rising all the way ${mono}, greatest step ${jump.toFixed(4)} (the dead zone's own) · its upright arris ${deg(corner).toFixed(1)}° · a top: ${deg(te).toFixed(1)}° and ${deg(tw).toFixed(1)}° at two facing edges, 0 in the middle, and at a corner the two sum to ${Math.hypot(tc[0], tc[1]).toFixed(2)} and are held to ${held(tc).toFixed(2)} · a 6 cm rail: ${deg(rail).toFixed(1)}° half way to its edge, 0 on its crest · an 8 cm slab's side: 0 at its foot, ${deg(slabTop).toFixed(1)}° at its top · the guard, ${asked} views: the normal keeps at least ${worst.toFixed(2)} of the lesser of what the face had toward the eye and a twentieth · on an upright cut plane at most ${upMost.toFixed(2)} of up (the fog of war calls 0.5 a top) · SAID, NOT BARRED: an eye ${EYE}° up sees a flat flank mirror ${-EYE}° (the ground at its foot) and a flat top ${EYE}° up the far sky; rolled ${lean.toFixed(1)}°, the flank's top strip mirrors up to ${(2 * lean - EYE).toFixed(1)}° and a top's far rim down to ${(EYE - 2 * lean).toFixed(1)}°, so both reach the horizon for any eye under ${(2 * lean).toFixed(0)}° up, and a level torch beside that eye wants a normal ${EYE / 2}° up`); }
  // 3. the shaders, the prelude and the uniform's writer, by their text
  { const WANT_ROLL = 'fn lookRoll(r: f32, p: vec2f, h: vec2f, up: vec2f) -> vec2f { if (r <= 0.0) { return vec2f(0.0); } var e = vec2f(0.0); for (var a = 0; a < 2; a++) { if (abs(up[a]) > 0.5) { e[a] = clamp(1.0 - (h[a] - p[a] * sign(up[a])) / min(r, 2.0 * h[a]), 0.0, 1.0) * sign(up[a]); } else { e[a] = clamp(1.0 - (h[a] - abs(p[a])) / min(r, h[a]), 0.0, 1.0) * sign(p[a]); } } return e * LOOK_MAX_TILT * min(2.0 * lookCfg.b.y, 1.0); }';
    const WANT_UP = 'fn lookRollUp(cls: u32, n: vec3f, y: f32, hy: f32, upW: vec3f) -> vec3f { if (cls == 0u) { return n; } let s = lookRoll(LOOK_ROLLED[clamp(i32(cls) - 1, 0, LOOK_LAYERS - 1)], vec2f(y, 0.0), vec2f(hy, 1.0), vec2f(1.0, 1.0)).x; if (s <= 0.0) { return n; } return normalize(n * sqrt(1.0 - s * s) + upW * s); }';
    const WANT_ASK = 'var rolled = 0.0; if (rolls && cls != 0u && n.y > -0.5) { rolled = LOOK_ROLLED[clamp(i32(cls) - 1, 0, LOOK_LAYERS - 1)]; } if (k2.y > 0.0 && (handled || rolled > 0.0)) {';
    const WANT_USE = 'if (rolled > 0.0) { var er = lookRoll(rolled, vec2f(pa, pb), vec2f(ha, hb), vec2f(TA.y, TB.y)); let V = normalize(frame.camPos - wp); let away = dot(TA * er.x + TB * er.y, V); if (away < 0.0) { er *= clamp((sqrt(1.0 - LOOK_MAX_TILT * LOOK_MAX_TILT) * dot(n, V) - 0.05) / -away, 0.0, 1.0); } e += er; } if (cls == 0u) { tilt = e; } else { tilt += vec2f(dot(TA * e.x + TB * e.y, T), dot(TA * e.x + TB * e.y, B)); } } let tl = length(tilt); if (tl > LOOK_MAX_TILT) { tilt *= LOOK_MAX_TILT / tl; } if (tl > LOOK_MIN_TILT) {';
    const WANT_CUT = 'if (LOOK && !cap && own >= 0 && !side) { let upW = quatRotate(boxRot(bi), vec3f(0.0, 1.0, 0.0)); if (abs(dot(normalize(v.normal), upW)) < 0.1) { n = lookRollUp(boxStuff(bi), n, v.lc.y, v.hf.y, upW); } }';
    const WANT_COAT = 'var coat = LOOK_MIRROR_PLAIN; if (LOOK && bidx != 0xFFFFFu) { coat += (LOOK_MIRRORS[boxStuff(bidx)] - LOOK_MIRROR_PLAIN) * beams.k.z; } let F0 = mix(vec3f(coat), base, metalness);';
    const lf = flat(look), gb = text('src/render/gbuffer.ts'), fr = text('src/render/frame.ts'), ps = text('src/render/passes.ts'), en = text('src/engine.ts'), cut = flat(text('src/render/shaders/gbuffer_cut.wgsl')), box = flat(text('src/render/shaders/gbuffer.wgsl'));
    const refl = ['reflect.wgsl', 'reflect_p5.wgsl', 'compact/reflect_p5.wgsl'].map(f => [f, flat(text(`src/render/shaders/${f}`))] as const);
    const coats = refl.filter(([, t]) => t.includes(WANT_COAT) && t.includes('let F0h = mix(vec3f(coat), base, metalness *') && !t.includes('vec3f(0.04)')).map(([f]) => f);
    const pins: [string, boolean][] = [
      ['look.wgsl lookRoll', lf.includes(WANT_ROLL)], ['look.wgsl lookRollUp', lf.includes(WANT_UP)], ['lookSurface asks the stuff only where its caller says the face is the box\'s own', lf.includes(WANT_ASK)],
      ['lookSurface rolls, guards the eye and adds it before the tilt is held', lf.includes(WANT_USE)],
      ['a box\'s own face is rolled, a round\'s, an arc\'s and an old break\'s are not', box.includes('lookW, true);') && cut.includes('lookW, !bent);') && cut.includes('let bent = side || oldCut;')],
      ['the cut pass asks lookRollUp on an upright cut plane, by the plane\'s own normal', cut.includes(WANT_CUT)],
      ['the look\'s prelude carries LOOK_ROLLED off the table', gb.includes('const LOOK_ROLLED = array<f32, ${LOOK_LAYERS}>(${LOOK_ROLLED.map(wgslNum).join(\', \')});')],
      ['a page without the look has lookRollUp as nothing', gb.includes('fn lookRollUp(cls: u32, n: vec3f, y: f32, hy: f32, upW: vec3f) -> vec3f { return n; }')],
      ['the scene prelude carries the plain coat and LOOK_MIRRORS off the table', fr.includes('const LOOK_MIRROR_PLAIN: f32 = ${LOOK_MIRROR_PLAIN.toFixed(4)}; const LOOK_MIRRORS = array<f32, ${LOOK_WORDS}>(${LOOK_MIRRORS.map(v => v.toFixed(4)).join(\', \')});')],
      ['the Finish row reaches the reflection pass, and only on the look\'s page', gb.includes('lookConfig.finish = lookAmounts.shine;') && en.includes('coats: lookConfig.on ? Math.max(0, Math.min(1, lookConfig.finish)) : 0 });') && ps.includes('F[6] = opts.coats;') && ps.includes('F[6] === Math.fround(opts.coats)')],
      ['all three reflection texts read the stuff\'s coat for the rays and the highlights', coats.length === 3] ];
    const bad = pins.filter(p => !p[1]).map(p => p[0]);
    verdict(bad.length === 0, 'the shaders\' rule is the one asked here: the look, the cut pass, both preludes, the Finish row\'s way to the reflection pass and its three texts', bad.length ? `NOT AS HELD: ${bad.join(' · ')}` : `${pins.length} texts as held (the reflection texts: ${coats.join(', ')})`); }
  console.log(`=== rolled: ${all ? 'ALL PASS' : 'FAILURES above'} ===`);
  return all;
}
if (import.meta.main) process.exit((await rolledCheck()) ? 0 : 1);
