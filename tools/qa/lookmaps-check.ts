// THE MATERIAL MAPS (src/render/lookTex.ts, shaders/look.wgsl, STYLE.md's Surfaces): what a map may and may not do, held by number.
//   bun run tools/qa/lookmaps-check.ts            exit 0 = every line passes
//   LOOKMAPS_BREAK=1 bun run tools/qa/lookmaps-check.ts     the maps as they were (the look row `even` at 0), each left as dark as it fell, and a wet thing's stuff
//                                                           asked of the colour it is handed: must FAIL
// No game is stood up: the maps are arithmetic and the rest is the g-buffer's own code on a device that only counts what it is handed. The picture is not looked at
// here (a software page and then a GPU do that); these are the rules a picture cannot show you are broken until a level is lit by them.
import { readFileSync } from 'node:fs';
import { ROOT } from './headless.ts';
const { lookLayer, lookMip, LOOK_LAYERS, LOOK_PAINTED, LOOK_NAMES, LOOK_N, LOOK_ROUGH, LOOK_TILE } = await import(`${ROOT}/src/render/lookTex.ts`);
const { GBuffer, LOOK_OF, LOOK_MATCH, LOOK_DEFAULTS, lookStuff, lookWordOf, lookAmounts } = await import(`${ROOT}/src/render/gbuffer.ts`);
const { GWOT_PALETTE } = await import(`${ROOT}/src/scene/gwotPalette.ts`);

const BRK = process.env.LOOKMAPS_BREAK === '1';
let red = false;
const check = (ok: boolean, what: string, detail = '') => { red ||= !ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}${detail ? `\n    ${detail}` : ''}`); };
console.log(`\n=== lookmaps: what a material map may do${BRK ? ' (the maps as they were: must fail)' : ''} ===`);

const N = 256; type Layer = { a: Uint8Array; b: Uint8Array; tone: number; hollow: number };
const layers: Layer[] = Array.from({ length: LOOK_LAYERS }, (_, l) => lookLayer(l, N, !BRK)), was: Layer[] = BRK ? layers : Array.from({ length: LOOK_LAYERS }, (_, l) => lookLayer(l, N, false));
const meanOf = (a: Uint8Array) => { const s = [0, 0, 0]; for (let i = 0; i < a.length; i += 4) { s[0] += a[i]; s[1] += a[i + 1]; s[2] += a[i + 2]; } return s.map(v => v / (a.length / 4) / 127.5); };
const lum = (r: number, g: number, b: number) => 0.2126 * r + 0.7152 * g + 0.0722 * b, valueOf = (a: Uint8Array) => { const m = meanOf(a); return lum(m[0], m[1], m[2]); };
const hollowsOf = (b: Uint8Array) => { let s = 0; for (let i = 3; i < b.length; i += 4) s += 0.5 + b[i] / 255; return s / (b.length / 4); };   // (the factor look.wgsl draws: 0.5 + the channel)

// 1. a painted layer is as bright as its paint, and keeps its own hue
{ const off: string[] = []; let worst = 0, hue = 0, hueAt = '';
  for (let l = 0; l < LOOK_PAINTED; l++) { const v = valueOf(layers[l].a), d = Math.abs(v - 1); worst = Math.max(worst, d); if (d > 0.004) off.push(`${LOOK_NAMES[l]} × ${v.toFixed(3)}`);
    const m = meanOf(layers[l].a), m0 = meanOf(was[l].a), k = lum(m[0], m[1], m[2]) / lum(m0[0], m0[1], m0[2]), h = Math.max(...m.map((c, i) => Math.abs(c - m0[i] * k))); if (h > hue) { hue = h; hueAt = LOOK_NAMES[l]; } }
  check(off.length === 0 && hue < 0.006, `each of the ${LOOK_PAINTED} painted layers is as bright as its paint (its mean value within 0.004 of 1) by ONE number a layer: a map varies a paint and never moves its value, and a blue steel stays as blue as it was written`,
    off.length ? off.join(' · ') : `furthest ${worst.toFixed(4)}; a channel moved against the others by ${hue.toFixed(4)} at most (${hueAt})`); }
// 2. the hollows darken as much as the high places lighten, so the two together are the paint however far the Cavities row is turned
{ const kp = LOOK_DEFAULTS.paint, off: string[] = []; let worst = 0, at = '', plain = 0;
  const drawn = (l: number, kc: number) => { const { a, b } = layers[l]; let s = 0;
    for (let i = 0; i < a.length; i += 4) s += lum(1 + kp * (a[i] / 127.5 - 1), 1 + kp * (a[i + 1] / 127.5 - 1), 1 + kp * (a[i + 2] / 127.5 - 1)) * (1 + kc * (b[i + 3] / 255 - 0.5));
    return s / (a.length / 4); };
  for (let l = 0; l < LOOK_PAINTED; l++) { plain = Math.max(plain, Math.abs(hollowsOf(layers[l].b) - 1));
    for (const kc of [LOOK_DEFAULTS.cavity * LOOK_DEFAULTS.clarity, 1, 1.5]) { const v = drawn(l, kc), d = Math.abs(v - 1); if (d > worst) { worst = d; at = `${LOOK_NAMES[l]} at ${kc}`; } if (d > 0.004) off.push(`${LOOK_NAMES[l]} at ${kc} × ${v.toFixed(3)}`); } }
  check(off.length === 0, `drawn as the page draws it (paint × hollows), each painted layer is as bright as its paint within 0.004, with the Cavities row where the page opens it, at 1 and at 1.5: the shade in a joint is paid for by the high places, weighed by the paint on them`,
    off.length ? off.join(' · ') : `furthest ${worst.toFixed(4)} (${at}); counted texel for texel the hollows alone are off 1 by up to ${plain.toFixed(3)}, which is how far a layer's high places are its pale ones`); }
// 3. … and the masonry's hollows, whose paint is its own
{ const off: string[] = [];
  for (let l = LOOK_PAINTED; l < LOOK_LAYERS; l++) { const { a, b } = layers[l]; let s = 0, w = 0; for (let i = 0; i < a.length; i += 4) { const L = lum(a[i], a[i + 1], a[i + 2]); s += L * (0.5 + b[i + 3] / 255); w += L; } if (Math.abs(s / w - 1) > 0.004) off.push(`${LOOK_NAMES[l]} × ${(s / w).toFixed(3)}`); }
  check(off.length === 0, 'the masonry\'s hollows are no second darkening either: weighed by the stone\'s or the brick\'s own colour they average 1 (within 0.004)', off.join(' · ')); }
// 4. the row's off word is the picture as it was, and the men's cloth is that picture whichever way the row stands
{ const CLOTH = (LOOK_NAMES as readonly string[]).indexOf('cloth'), m = layers[CLOTH], w = was[CLOTH]; let dp = 0, dh = 0;
  for (let i = 0; i < m.a.length; i += 4) { for (let c = 0; c < 3; c++) dp = Math.max(dp, Math.abs(Math.min(2, m.a[i + c] / 127.5 * m.tone) - w.a[i + c] / 127.5)); dh = Math.max(dh, Math.abs(2 * (m.b[i + 3] / 255 - m.hollow) - 2 * w.b[i + 3] / 255)); }
  const src = readFileSync(`${ROOT}/src/render/shaders/look.wgsl`, 'utf8');
  check(was.every(l => l.tone === 1 && l.hollow === 0) && CLOTH >= 0 && new RegExp(`let tp = lookTap\\(${CLOTH}, vec2f\\(S\\.u, S\\.v\\)`).test(src) && dp < 0.02 && dh < 0.02 && (BRK || (m.tone !== 1 && m.hollow > 0))
    && /tp\.a\.rgb \* 2\.0 \* lookCfg\.h\.x, 0\.55 \* K\) \* mix\(1\.0, 0\.6 \+ 0\.8 \* \(tp\.b\.a - lookCfg\.h\.y\), 0\.6 \* K\)/.test(src),
    'with the row at 0 no layer is touched; and the men\'s cloth, which was tuned on a GPU against the cloth layer as it fell, is handed that layer back: times its tone, less its hollow, it is the old layer to a texel\'s rounding',
    `cloth: tone ${m.tone.toFixed(3)}, hollow ${m.hollow.toFixed(3)}; the paint comes back within ${dp.toFixed(4)}, the hollows within ${dh.toFixed(4)}`); }
// 5. … and the masonry is not paint
{ const m = [LOOK_PAINTED, LOOK_PAINTED + 1].map(l => meanOf(layers[l].a));
  check(LOOK_LAYERS === LOOK_PAINTED + 2 && LOOK_NAMES[LOOK_PAINTED] === 'field stone' && LOOK_NAMES[LOOK_PAINTED + 1] === 'mud brick' && m.every(v => Math.max(...v.map((c: number) => Math.abs(c - 1))) > 0.05),
    'the two masonry layers keep a colour of their own (stone and brick under a mud wall are another stuff than its paint)', m.map((v, i) => `${LOOK_NAMES[LOOK_PAINTED + i]} × ${v.map((c: number) => c.toFixed(2)).join(' ')}`).join(' · ')); }
// 6. no seam: the step across a tile's edge is a step like the ones inside it
{ const bad: string[] = []; let worst = 0, worstAt = '';
  for (let l = 0; l < LOOK_LAYERS; l++) for (const [nm, ch] of [['height', 3], ['paint', 1]] as const) for (const axis of [0, 1]) {
    const a = layers[l].a, line = new Float64Array(N);
    for (let p = 0; p < N; p++) { let s = 0; for (let q = 0; q < N; q++) { const i0 = axis ? (p * N + q) : (q * N + p), i1 = axis ? (((p + 1) % N) * N + q) : (q * N + (p + 1) % N); s += Math.abs(a[i0 * 4 + ch] - a[i1 * 4 + ch]); } line[p] = s / N; }
    let inside = 0; for (let p = 0; p < N - 1; p++) inside = Math.max(inside, line[p]);
    const r = line[N - 1] / (inside + 0.5); if (r > worst) { worst = r; worstAt = `${LOOK_NAMES[l]} ${nm} ${axis ? 'up' : 'along'}`; }
    if (line[N - 1] > 1.25 * inside + 0.5) bad.push(`${LOOK_NAMES[l]} ${nm} ${axis ? 'up' : 'along'}: ${line[N - 1].toFixed(1)} at the edge, ${inside.toFixed(1)} the most inside`);
  }
  check(bad.length === 0, 'no layer has a seam: across a tile\'s edge the height and the paint step no more than they do inside the tile', bad.length ? bad.join(' · ') : `nearest ${worstAt}, ${worst.toFixed(2)} of the largest step inside`); }
// 6b. old paint is chipped, not blotched: no bare patch reaches further than a hand and little of the tile is bare (the maintainer, 11-04, of patches up to
// 36 cm on painted lids: "splotches on here look a bit weird, overdone"; that tile, the commit before this line, reads 35.9 cm and 5.1 % here)
{ const l = (LOOK_NAMES as readonly string[]).indexOf('paint'), a = layers[l].a, bare = new Uint8Array(N * N), seen = new Uint8Array(N * N); let n = 0, wide = 0;
  for (let i = 0; i < N * N; i++) if (lum(a[i * 4], a[i * 4 + 1], a[i * 4 + 2]) / 127.5 < 0.72) { bare[i] = 1; n++; }
  for (let i = 0; i < N * N; i++) { if (!bare[i] || seen[i]) continue; let x0 = 0, x1 = 0, y0 = 0, y1 = 0; const todo: [number, number, number][] = [[i, 0, 0]]; seen[i] = 1;   // (a patch: bare texels that touch, across the tile's edge too; its size is how far it reaches along and up)
    while (todo.length) { const [j, ox, oy] = todo.pop()!, x = j % N, y = (j / N) | 0; x0 = Math.min(x0, ox); x1 = Math.max(x1, ox); y0 = Math.min(y0, oy); y1 = Math.max(y1, oy);
      for (let dy = -1; dy <= 1; dy++) for (let dx = -1; dx <= 1; dx++) { const k = ((y + dy + N) % N) * N + (x + dx + N) % N; if (bare[k] && !seen[k]) { seen[k] = 1; todo.push([k, ox + dx, oy + dy]); } } }
    wide = Math.max(wide, (Math.max(x1 - x0, y1 - y0) + 1) * LOOK_TILE / N); }
  const share = n / (N * N);
  check(wide <= 0.12 && share <= 0.02 && share >= 0.002, 'old paint is chipped, not blotched: its widest bare patch reaches no further than a hand (12 cm) and between 0.2 % and 2 % of the tile is bare', `widest ${(wide * 100).toFixed(1)} cm, ${(share * 100).toFixed(2)} % bare`); }
// 7. a map carries no light: the normal is a direction, the finish stays about the layer's own, and a smaller copy is the same paint
{ const bad: string[] = [];
  for (let l = 0; l < LOOK_LAYERS; l++) { const b = layers[l].b; let long = 0, rs = 0;
    for (let i = 0; i < b.length; i += 4) { const x = b[i] / 127.5 - 1, y = b[i + 1] / 127.5 - 1; if (x * x + y * y > 1.02) long++; rs += b[i + 2] / 255; }
    const rough = rs / (b.length / 4);
    if (long) bad.push(`${LOOK_NAMES[l]}: ${long} normals longer than 1`);
    if (Math.abs(rough - LOOK_ROUGH[l]) > 0.2) bad.push(`${LOOK_NAMES[l]}: finish ${rough.toFixed(2)} written round ${LOOK_ROUGH[l]}`);
    let a = layers[l].a, n = N; const m0 = meanOf(a); while (n > 8) { a = lookMip(a, n, false); n >>= 1; } const m1 = meanOf(a);
    if (Math.max(...m0.map((v, c) => Math.abs(v - m1[c]))) > 0.012) bad.push(`${LOOK_NAMES[l]}: paint × ${m0.map(v => v.toFixed(3)).join(' ')} at full size, × ${m1.map(v => v.toFixed(3)).join(' ')} at 8 texels`);
  }
  check(bad.length === 0, 'every normal is a direction, every finish is about its layer\'s own, and a layer read from far off is the same paint as up close', bad.join(' · ')); }
// 8. what a box is made of is the box's own word, decided from its authored paint
{ const P = GWOT_PALETTE as Record<string, number[]>, names = Object.keys(P), of = LOOK_OF as Record<string, number>;
  const wrong = names.filter(m => lookStuff(P[m]) !== (of[m] ?? 0)).map(m => `${m} is drawn as class ${lookStuff(P[m])}, its row says ${of[m] ?? 0}`);
  check(wrong.length === 0, `every one of the palette's ${names.length} paints is drawn as its own row's stuff (${Object.keys(of).length} have a material, the rest are plain)`, wrong.join(' · '));
  // a paint worn dark in one piece is still its stuff, unless the worn colour IS another paint: then it is that paint, as it is to a round. Counted, so a new paint that adds one is seen
  const taken: string[] = [];
  for (const p of Object.keys(of)) for (const q of names) { if (q === p || (of[q] ?? 0) === of[p]) continue;
    for (let k = LOOK_MATCH.darkLo + 0.0025; k < LOOK_MATCH.darkHi; k += 0.0025) if (P[p].every((v, c) => Math.abs(v * k - P[q][c]) < LOOK_MATCH.near)) { taken.push(`${p} × ${k.toFixed(2)} = ${q}`); break; } }
  const worn = (m: string, k: number) => lookStuff(P[m].map(v => v * k));
  check(worn('dust', 0.7) === of.dust && worn('mud', 0.84) === of.mud && worn('slab', 0.6) === of.slab && worn('gravel', 0.5) === 0 && taken.length <= 10,
    'a paint worn dark in one piece (a trodden patch, a door\'s wear) is still its stuff; a worn colour that is another paint\'s own is that paint, and there are no more such pairs than there were',
    `${taken.length} pairs: ${taken.join(' · ')}`);
  // on a real level, through the real pack: the word above the finish, the finish under it, what moves, and a number that is the box's own
  const { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`), { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`), { standUp } = await import('./headless.ts'), { matWord, makeBox, MAT_STRIDE } = await import(`${ROOT}/src/scene/boxes.ts`);
  const stand = async (file: string) => { const l0 = console.log, w0 = console.warn; console.log = console.warn = () => { };
    const W: any = await standUp({ level: loadLevel(parseLevelData(readFileSync(`${ROOT}/levels/${file}.json`, 'utf8'))), seed: 1 }); for (let i = 0; i < 4; i++) W.step(1 / 60);
    console.log = l0; console.warn = w0; return W; };
  { const world = (await stand('smoke-lab')).game.engine.world, word = (i: number) => world.matData[i * MAT_STRIDE + 7], all = () => [...world.statics, ...world.dynamics];
    world.forgetSent(); world.upload(); const plain = all().every((b: any, i: number) => word(i) === Math.fround(matWord(b)));
    world.setStuffOf(lookWordOf()); world.upload();   // (on the page the engine sets it before the first box is packed; the setter has every static packed again)
    const nS = world.statics.length, boxes = all(); let classed = 0, finishOff = 0, classOff = 0, movesOff = 0, saltOff = 0; const salts = new Set<number>(), mine = new Map<any, number>();
    boxes.forEach((b: any, i: number) => { const lw = Math.floor(word(i) / 8), c = lw & 15, mv = (lw & 16) !== 0, salt = lw >> 5;
      if (c !== lookStuff(b.albedo)) classOff++; if (Math.abs(word(i) - 8 * lw - matWord(b)) > 2e-3) finishOff++; if (c > 0) classed++; if (mv !== (i >= nS)) movesOff++; if (salt > 90) saltOff++; if (mv) { salts.add(salt); mine.set(b, lw); } });
    // a wall goes: a static is appended, and every box after it has a new place in the list
    world.addStatic(makeBox({ c: [1, 0.5, 1], h: [0.2, 0.5, 0.2], albedo: [0.3, 0.3, 0.3] })); world.upload();
    let jumped = 0; world.dynamics.forEach((b: any, k: number) => { if (Math.floor(word(world.statics.length + k) / 8) !== mine.get(b)) jumped++; });
    check(plain && classOff === 0 && finishOff === 0 && classed > boxes.length / 2 && movesOff === 0 && saltOff === 0 && mine.size > 4 && salts.size > 2 && jumped === 0 && world.statics.length === nS + 1,
      'on a level, through the real pack: with nobody asking, the finish word is as it ever was; on the look page every box carries its stuff above its finish and the finish reads back (to a 500th: the word shares the float); a box past the statics, and no other, says it moves; and when a static is appended no mover\'s word changes, though every mover\'s place in the list does',
      `smoke-lab: ${boxes.length} boxes, ${classed} with a material, ${classOff} not their paint's, ${finishOff} finishes moved, ${mine.size} movers with ${salts.size} numbers between them, ${movesOff} wrong about moving, ${jumped} changed when a static was added`); }
  // the rain, by the game's own weather: a level whose lot is gravel and stands wet
  { const g = (await stand('consulate-gwot')).game, world = g.engine.world, st = world.statics as any[];
    const li = st.findIndex(b => b.c[1] < 0 && b.c[1] + b.h[1] > -0.01 && b.h[0] > 18 && b.h[2] > 12), lot = st[li], dry = li < 0 ? undefined : g.weather.dryPaint(lot);
    const cls = (i: number) => Math.floor(world.matData[i * MAT_STRIDE + 7] / 8) & 15;
    world.setStuffOf(lookWordOf()); world.upload(); const blind = li < 0 ? -1 : cls(li);
    world.setStuffOf(lookWordOf((b: any) => g.weather.dryPaint(b))); world.upload(); const told = li < 0 ? -1 : cls(li);
    // … and a picked rain, which darkens what it wets for the picture alone (Box.wetK): the paint is untouched, the colour a corner is handed is not. The level
    // is packed DRY first and each thing's word kept, then the rain is picked and it is packed again: the sentence itself, word against word
    const W2 = await stand('compound-gwot'), g2 = W2.game, w2 = g2.engine.world, st2 = w2.statics as any[], cls2 = (i: number) => Math.floor(w2.matData[i * MAT_STRIDE + 7] / 8) & 15;
    w2.setStuffOf(lookWordOf((b: any) => g2.weather.dryPaint(b))); w2.upload(); const dryWord = new Map<any, number>(st2.map((b, i) => [b, cls2(i)]));
    g2.weather.setKind('rain'); for (let i = 0; i < 10; i++) W2.step(1 / 60);
    w2.setStuffOf(lookWordOf((b: any) => g2.weather.dryPaint(b))); w2.upload();
    const wetOnly = st2.filter(b => b.wetK !== undefined);
    // WHAT IS WET: every flat thing under open sky (game/wetGround.ts), not the open ground alone. So a wet thing may be a thing of plain paint laid on the ground
    // (a weed, a rug, a mat, a stain): of no stuff dry, and of none wet. The line's claim is asked of the wet things that ARE of a stuff (there must be some: a
    // darker gravel or concrete is read as nothing), and the GROUND itself (a slab whose middle is below the floor, as the lot above is found) is always of one.
    // A wet metal thing is handed its own colour (wetK 1).
    // LOOKMAPS_BREAK=1 holds a wet thing to the stuff of the colour it is handed in place of the word it carried dry: the line must fail.
    const dryOf = (b: any) => BRK ? lookStuff(b.albedo.map((v: number) => v * b.wetK)) : dryWord.get(b);
    const wetStuff = wetOnly.filter(b => (dryWord.get(b) ?? 0) > 0), wetOff = wetOnly.filter(b => { const c = cls2(st2.indexOf(b)); return c !== dryOf(b) || c !== lookStuff(b.albedo); }).length;
    const bare = wetOnly.filter(b => lookStuff(b.albedo) === 0), bareGround = bare.filter(b => b.c[1] < 0).length, ground = wetOnly.filter(b => b.c[1] < 0).length;
    const darker = wetOnly.filter(b => { const m = st2.indexOf(b) * MAT_STRIDE; return Math.abs(w2.matData[m] - b.albedo[0] * b.wetK) < 1e-4 && (b.metal ? b.wetK === 1 : b.wetK < 1); }).length;
    check(li >= 0 && !!dry && lookStuff(dry) === of.gravel && lookStuff(lot.albedo) !== of.gravel && told === of.gravel && blind !== of.gravel && wetStuff.length > 0 && ground > 0 && wetOff === 0 && bareGround === 0 && darker === wetOnly.length,
      'rain does not change what a thing is made of: the game\'s own weather darkens a gravel lot in place, and the lot is still gravel because its word is asked of the paint it had dry (asked of the colour it has now, it was not); and each thing a picked rain wets for the picture alone is handed to the shader darker and carries the word it carried dry, its paint\'s (what the darker colour of a concrete slab or of gravel would be read as is the line above: nothing); a wet thing of no stuff is a thing of plain paint laid on the ground, which had none dry, and never the ground itself',
      li < 0 ? 'consulate-gwot: no lot found' : `consulate-gwot: the lot's paint now ${lot.albedo.map((v: number) => v.toFixed(3)).join(' ')} (class ${lookStuff(lot.albedo)}), dry ${dry ? [...dry].map((v: number) => v.toFixed(3)).join(' ') : 'not kept'} (class ${dry ? lookStuff(dry) : '?'}); packed as ${told}, and as ${blind} with the weather not asked; compound-gwot under a picked rain: ${wetOnly.length} things wet for the picture alone, ${darker} handed darker (a metal one its own colour), ${wetStuff.length} of a stuff, ${wetOff} with another word than dry; ${bare.length} things of plain paint laid on the ground, ${ground} slabs of the ground wet and ${bareGround} of them of no stuff`); }
  const gsrc = readFileSync(`${ROOT}/src/render/shaders/gbuffer.wgsl`, 'utf8'), csrc = readFileSync(`${ROOT}/src/render/shaders/common.wgsl`, 'utf8'), lsrc = readFileSync(`${ROOT}/src/render/shaders/look.wgsl`, 'utf8'), msrc = readFileSync(`${ROOT}/src/main.ts`, 'utf8');
  check(/if \(LOOK\) \{ o\.look = boxStuff\(bi\); \}/.test(gsrc) && !/lookClass\(o\.albedo\)/.test(gsrc) && /fn boxLook\(i: u32\) -> u32 \{ return u32\(boxMat\[i \* 4u \+ 1u\]\.w \* 0\.125\); \}/.test(csrc) && /fn boxStuff\(i: u32\) -> u32 \{ return boxLook\(i\) & 15u; \}/.test(csrc)
    && /if \(LOOK\) \{ w -= 8\.0 \* floor\(w \* 0\.125\); \}/.test(csrc) && /let word = boxLook\(bi\); let moves = \(word & 16u\) != 0u;/.test(lsrc) && /let salt = word >> 5u;/.test(lsrc) && /\|\| moves\) \{/.test(lsrc) && !/bi % (7|13)u/.test(lsrc)
    && /mix\(1\.0, 0\.5 \+ cav, k2\.z\)/.test(lsrc) && /setStuffOf\(lookWordOf\(b => game\.weather\.dryPaint\(b\)[,)]/.test(msrc),
    'the shader reads the box\'s word and takes it off the finish: no box is told what it is made of by the colour a corner is handed or how to lay its map by its place in the list, the hollows are drawn round a half, and the page asks the weather for a soaked slab\'s dry paint'); }
// 9. the g-buffer's half, on a device that only counts: nothing is made in the constructor's turn, every layer and mip is written by the fill, and the cloth's tone
//    and hollow are the block's last row once the cloth layer is made
{ const writes: { tex: string; mip: number; layer: number; n: number }[] = [], bufs: { off: number; data: Float32Array }[] = []; let made = 0, cfgSize = 0;
  const dev: any = { createTexture: (d: any) => { made++; return { label: d.label, createView: () => ({}) }; }, createSampler: () => ({}), createBuffer: (d: any) => { if (d.label === 'lookCfg') cfgSize = d.size; return { label: d.label, size: d.size }; },
    queue: { writeTexture: (t: any, _d: any, _l: any, size: number[]) => { if (t.mipLevel === 0 && !writes.some(w => w.layer === t.origin[2])) firstAt.push(yields); writes.push({ tex: t.texture.label, mip: t.mipLevel, layer: t.origin[2], n: size[0] }); }, writeBuffer: (b: any, off: number, data: Float32Array) => { if (b.label === 'lookCfg') bufs.push({ off, data: new Float32Array(data) }); } } };
  let yields = 0; const firstAt: number[] = [];   // (how many messages the fill had waited on when each layer's first texel was written. A message is a task of its own in a browser: the card draws and a compiled pipeline lands before it. Counting the loop's turns from here says nothing under bun, which hands a port a thousand messages in one go)
  const G: any = Object.create(GBuffer.prototype); G.device = dev; G.lookFillMs = 0; G.lookMen = [1, 0];
  G.makeLook();
  const atMake = writes.length, block = bufs[0]?.data.length ?? 0, first = bufs[0]?.data.slice(block - 4, block - 2);
  const src = readFileSync(`${ROOT}/src/render/shaders/look.wgsl`, 'utf8'), struct = /struct LookCfg \{([^}]*)\}/.exec(src)?.[1] ?? '', rows = (struct.match(/vec4f/g) ?? []).length;
  const MC = globalThis.MessageChannel; (globalThis as any).MessageChannel = class extends MC { constructor() { super(); yields++; } };
  const even0 = lookAmounts.even; lookAmounts.even = BRK ? 0 : 1; await G.fillLook(); lookAmounts.even = even0; (globalThis as any).MessageChannel = MC;
  let mips = 1; for (let n = LOOK_N; n > 1; n >>= 1) mips++;
  const perLayer = new Map<number, number>(); for (const w of writes) perLayer.set(w.layer, (perLayer.get(w.layer) ?? 0) + 1);
  check(made === 2 && atMake === 0 && writes.length === LOOK_LAYERS * mips * 2 && [...perLayer.values()].every(v => v === mips * 2) && firstAt.length === LOOK_LAYERS && firstAt.every((t, i) => i === 0 ? t >= 1 : t > firstAt[i - 1]),
    `the maps cost the boot's first turn nothing: no texel is written where the textures are made, and the fill writes ${LOOK_LAYERS} layers × ${mips} sizes × 2 maps with a wait on the event loop before each layer`,
    `${made} textures, ${atMake} writes at the making, ${writes.length} after the fill, the fill had waited on ${firstAt.slice(0, 4).join(', ')} … messages at each layer's first texel (${firstAt.filter((t, i) => i > 0 ? t <= firstAt[i - 1] : t < 1).length} layers with no wait before them), ${G.lookFillMs.toFixed(0)} ms`);
  const last = bufs[bufs.length - 1]?.data.slice(block - 4, block - 2), full = lookLayer((LOOK_NAMES as readonly string[]).indexOf('cloth'), LOOK_N, !BRK) as Layer;
  check(rows * 16 === cfgSize && block * 4 === cfgSize && first?.[0] === 1 && first?.[1] === 0 && Math.abs((last?.[0] ?? 0) - full.tone) < 1e-6 && Math.abs((last?.[1] ?? -1) - full.hollow) < 1e-6 && bufs.length === 2 && LOOK_DEFAULTS.even === 1,
    'the block is the shader\'s struct to the byte, and its last row is the cloth layer\'s tone and hollow: 1 and 0 until that layer is made (the men drawn against nothing yet), then what the fill found, written once',
    `block ${cfgSize} bytes, ${rows} rows in the shader, ${block} numbers written, ${bufs.length} writes; the last row ${first ? [...first].join(' ') : '?'} at the making, ${last ? [...last].map(v => v.toFixed(3)).join(' ') : '?'} after the fill`);
}
console.log(red ? '\nlookmaps: RED' : '\nlookmaps: all pass');
process.exit(red ? 1 : 0);
