// The 'charmats' family (render/charMaterials.ts: a man shaded per material slot). The shading itself needs a GPU; what
// a headless bun can hold is everything the shader is HANDED and the promise that off is off:
//   charmats-slots    the loaded mesh's slot data: the mannequin (no slot-named material) takes the stand-in — every vertex a slot, the ten it deals all with surface,
//                     cavity finite in −1‥1 with both signs present, every proxy's weights summing to 1 — the material-name matcher, and a mesh whose materials ARE
//                     named after slots dealt by name with the unmatched one reported
//   charmats-tables   every side × slot resolves to a colour in 0‥1 and a finish the lobe can carry; the same man the same rows twice (no dice but his id); the
//                     operators one issue (cloth identical man to man) and the defenders not (≥ 3 shirts over eleven men); the two sides apart from above
//   charmats-shader   the WGSL and the TS agree on the row count / slot count, and the off path's statements are still there verbatim
//   bun run tools/qa/charmats-probes.ts
import { main, ROOT } from './here.ts';
import { readFileSync } from 'node:fs';
const CM = await import(`${ROOT}/src/render/charMaterials.ts`);
const { setLookPlace } = await import(`${ROOT}/src/scene/look.ts`); setLookPlace(false);   // (the first cast's tables, the office's: the look's are asked of by name below)
const { loadGltfCharacter } = await import(`${ROOT}/src/anim/gltf.ts`);

type Verdict = (ok: boolean, label: string, detail: string) => void;
function family(title: string): { verdict: Verdict; done: () => boolean } {
  console.log(`\n=== ${title} ===`); let all = true;
  return { verdict: (ok, label, detail) => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); }, done: () => { console.log(`=== ${title.split(':')[0]}: ${all ? 'ALL PASS' : 'FAILURES above'} ===`); return all; } };
}
const mannequin = () => loadGltfCharacter(`file://${ROOT}/public/assets/ual/AnimationLibrary_Godot_Standard.gltf`);
const lum = (c: number[]) => 0.2126 * c[0] + 0.7152 * c[1] + 0.0722 * c[2];
const f3 = (x: number) => x.toFixed(3);

// ---------------------------------------------------------------- charmats-slots
export async function charmatsSlots() {
  const F = family('charmats-slots: what the mesh gives the shader — a slot and a cavity per vertex, a slot mix per shadow proxy (charMaterials.ts charSlots)');
  const ch = await mannequin(); const d = CM.charSlots(ch); const V = ch.mesh.vertexCount; const N = CM.CHAR_SLOT_COUNT;
  { const f: string[] = [];
    if (d.authored) f.push('the mannequin read as authored (its materials are M_Main / M_Joints)');
    if (d.slot.length !== V || d.cavity.length !== V) f.push(`lengths ${d.slot.length}/${d.cavity.length} vs ${V} vertices`);
    let bad = 0, pos = 0, neg = 0; for (let v = 0; v < V; v++) { if (!(d.slot[v] < N)) bad++; const c = d.cavity[v]; if (!(c >= -1 && c <= 1)) bad++; if (c > 0.15) pos++; if (c < -0.15) neg++; }
    if (bad) f.push(`${bad} vertices with a slot ≥ ${N} or a cavity outside −1‥1`);
    if (!(pos > 50 && neg > 50)) f.push(`cavity one-sided: ${pos} creases, ${neg} ridges past ±0.15`);
    const tot = d.area.reduce((n: number, a: number) => n + a, 0);
    const want = ['skin', 'cloth_uniform', 'cloth_plain', 'webbing', 'boot', 'helmet', 'metal_dark', 'rubber', 'lens', 'accent'];
    for (const n of want) if (!(d.area[CM.CHAR_SLOTS.indexOf(n)] > 0)) f.push(`the stand-in deals no surface to ${n}`);
    if (!(tot > 1.2 && tot < 3)) f.push(`surface ${tot.toFixed(2)} m² (a man is ~1.6‥2.2)`);
    F.verdict(f.length === 0, 'the mannequin: stand-in slots for every vertex, the ten it deals all with surface (it has no `eye`), cavity in range and two-sided', f.join('; ') || `${V} vertices · ${tot.toFixed(2)} m² · ` + CM.CHAR_SLOTS.map((n: string, s: number) => `${n} ${(100 * d.area[s] / tot).toFixed(1)}%`).join(' · ') + ` · cavity ${pos} creases / ${neg} ridges`); }
  { const f: string[] = []; const rows: string[] = [];
    for (let p = 0; p < 15; p++) { let s = 0, top = 0; for (let k = 0; k < N; k++) { const w = d.capWeights[p * N + k]; s += w; if (w > d.capWeights[p * N + top]) top = k; } if (Math.abs(s - 1) > 1e-4) f.push(`proxy ${p} sums to ${s}`); rows.push(`${p}:${CM.CHAR_SLOTS[top]}`); }
    const topOf = (p: number) => { let top = 0; for (let k = 0; k < N; k++) if (d.capWeights[p * N + k] > d.capWeights[p * N + top]) top = k; return CM.CHAR_SLOTS[top]; };
    if (topOf(1) !== 'webbing') f.push(`the chest's main slot is ${topOf(1)} (want the carrier: webbing)`);
    if (topOf(9) !== 'cloth_uniform' || topOf(12) !== 'cloth_uniform') f.push(`the thighs' main slot ${topOf(9)} / ${topOf(12)} (want cloth_uniform)`);
    if (!['boot', 'rubber'].includes(topOf(11))) f.push(`the left shoe's main slot is ${topOf(11)}`);
    if (!(d.capWeights[4 * N + CM.CHAR_SLOTS.indexOf('helmet')] > 0.2 && d.capWeights[4 * N + CM.CHAR_SLOTS.indexOf('skin')] > 0.2)) f.push('the head proxy is not a mix of helmet and skin');
    F.verdict(f.length === 0, 'every shadow proxy carries a slot mix summing to 1, and the right one leads (chest = carrier, thighs = trousers, shoes = boot, head = helmet + skin)', f.join('; ') || rows.join(' ')); }
  { const f: string[] = [];
    const cases: [string, string | null][] = [['skin', 'skin'], ['M_Skin', 'skin'], ['cloth_uniform.001', 'cloth_uniform'], ['Cloth-Plain', 'cloth_plain'], ['mat_webbing', 'webbing'], ['op_helmet', 'helmet'], ['skin_face', 'skin'], ['metal_dark', 'metal_dark'], ['Lens', 'lens'], ['accent', 'accent'], ['M_Main', null], ['M_Joints', null], ['', null], ['clothing', null]];
    for (const [name, want] of cases) { const s = CM.slotOfMaterialName(name); const got = s < 0 ? null : CM.CHAR_SLOTS[s]; if (got !== want) f.push(`'${name}' → ${got} (want ${want})`); }
    // a mesh that keeps the contract: the mannequin's two primitives renamed
    const m2 = { ...ch.mesh, materialNames: ['cloth_uniform', 'Trim_Stuff'] }; const d2 = CM.charSlots({ ...ch, mesh: m2 });
    if (!d2.authored) f.push('a mesh with a slot-named material did not read as authored');
    let wrong = 0; for (let v = 0; v < V; v++) if (d2.slot[v] !== (ch.mesh.matIndex[v] === 0 ? CM.CHAR_SLOTS.indexOf('cloth_uniform') : CM.CHAR_SLOTS.indexOf('cloth_plain'))) wrong++;
    if (wrong) f.push(`${wrong} vertices not in their material's slot (unmatched → cloth_plain)`);
    if (d2.unmatched.join() !== 'Trim_Stuff') f.push(`unmatched reported as [${d2.unmatched}]`);
    { // a CAST of part meshes as the models' loader makes them: part per vertex, slot in matSel's low four bits — two 'parts': everything (trousers), and the head again as a helmet
      const part = new Uint8Array(V), ms = new Float32Array(V); const top = ch.mesh.bboxMax[1];
      for (let v = 0; v < V; v++) { const head = ch.mesh.positions[v * 3 + 1] > top - 0.2; part[v] = head ? 1 : 0; ms[v] = (head ? 5 : 1) + 16 * part[v]; }
      const mc = { ...ch.mesh, part, matSel: ms }; const dc = CM.charSlots({ ...ch, mesh: mc });
      if (!CM.meshAuthored(mc) || !dc.authored) f.push('a cast of parts did not read as authored');
      let bad = 0; for (let v = 0; v < V; v++) if (dc.slot[v] !== (part[v] ? 5 : 1)) bad++; if (bad) f.push(`${bad} cast vertices not in matSel's slot`);
      const all = dc.weights(0), body = dc.weights(1), both = dc.weights(3); const H = CM.CHAR_SLOTS.indexOf('helmet');
      if (!(all.area[H] > 0) || body.area[H] !== 0 || both.area[H] !== all.area[H]) f.push(`weights per part mask: helmet area all ${all.area[H]} / body only ${body.area[H]} / both ${both.area[H]}`);
      if (dc.weights(1) !== body) f.push('weights(mask) not cached'); }
    const m3 = { ...ch.mesh, materialNames: ['Body', 'Joints', 'metal_dark'] }; if (CM.meshAuthored(m3)) f.push('a slot name on a material the mesh never uses made it authored');
    const mi = ch.mesh.matIndex.slice(); mi.fill(255, 0, 300); const m4 = { ...ch.mesh, materialNames: ['skin', 'helmet'], matIndex: mi }; const d4 = CM.charSlots({ ...ch, mesh: m4 });
    if (d4.slot[0] !== CM.CHAR_SLOTS.indexOf('cloth_plain') || !d4.unmatched.some((u: string) => u.includes('no material'))) f.push('a primitive with no material was not dealt plain cloth and reported');
    F.verdict(f.length === 0, 'material names → slots (prefixes, suffixes, case); a mesh that names them is dealt BY NAME with strays reported; names the mesh does not use do not count', f.join('; ') || `${cases.length} names · authored mesh: ${d2.unmatched.length} unmatched material reported`); }
  return F.done();
}

// ---------------------------------------------------------------- charmats-tables
export async function charmatsTables() {
  const F = family('charmats-tables: the material tables per side resolve, repeat, and tell the sides apart (charMaterials.ts CHAR_MATS / writeCharPalette)');
  CM.resetCharMats();
  const OP: [number[], number[]] = [[0.07, 0.09, 0.1], [0.07, 0.2, 0.6]], DEF: [number[], number[]] = [[0.36, 0.33, 0.24], [0.09, 0.08, 0.05]];
  { const f: string[] = []; let rows = 0;
    const SIDES = ['operator', 'defender', 'responder', 'civilian'];
    for (const side of SIDES) for (let id = 1; id <= 40; id += 3) {
      const pal = new Float32Array(CM.CHAR_MAT_FLOATS); CM.writeCharPalette(pal, 0, side, side === 'operator' ? OP[0] : DEF[0], side === 'operator' ? OP[1] : DEF[1], id); rows++;
      if (pal[0] !== 1 + SIDES.indexOf(side)) f.push(`${side} ${id}: header ${pal[0]}`);   // the header's row is the side's own look (1 to 4; a suit's is 5)
      for (let s = 0; s < CM.CHAR_SLOT_COUNT; s++) { const p = 8 + s * 8; const a = [pal[p], pal[p + 1], pal[p + 2]];
        if (!a.every(x => x >= 0 && x <= 1 && Number.isFinite(x))) f.push(`${side} ${id} ${CM.CHAR_SLOTS[s]} albedo ${a}`);
        if (!(pal[p + 3] >= 0.03 && pal[p + 3] <= 0.65)) f.push(`${side} ${CM.CHAR_SLOTS[s]} rough ${pal[p + 3]} outside the lobe's 0.03‥0.65`);
        if (!(pal[p + 4] >= 0 && pal[p + 4] <= 2)) f.push(`${side} ${CM.CHAR_SLOTS[s]} gain ${pal[p + 4]} outside 0‥2`);
        if (!(pal[p + 5] >= 0 && pal[p + 5] <= 1 && pal[p + 6] >= 0 && pal[p + 6] <= 1 && pal[p + 7] >= 0 && pal[p + 7] <= 1)) f.push(`${side} ${CM.CHAR_SLOTS[s]} soak/grime/wear out of 0‥1`); }
      for (let k = 8 + CM.CHAR_SLOT_COUNT * 8; k < CM.CHAR_MAT_FLOATS; k++) if (pal[k] !== 0) f.push(`${side} ${id}: spare float ${k} = ${pal[k]}`);
    }
    F.verdict(f.length === 0, 'every side × slot × man: a colour in 0‥1, a roughness and a gain the lobe can carry, the spare rows zero', [...new Set(f)].slice(0, 6).join('; ') || `${rows} palettes`); }
  { const f: string[] = [];
    const pal = (side: string, t: number[][], id: number, standIn = false) => { const p = new Float32Array(CM.CHAR_MAT_FLOATS); CM.writeCharPalette(p, 0, side, t[0], t[1], id, standIn); return p; };
    const slot = (p: Float32Array, n: string) => { const o = 8 + CM.CHAR_SLOTS.indexOf(n) * 8; return [p[o], p[o + 1], p[o + 2]]; };
    for (let id = 1; id <= 12; id++) { const a = pal('defender', DEF, id), b = pal('defender', DEF, id); if (a.join() !== b.join()) f.push(`defender ${id} differs call to call`); }
    const opCloth = new Set<string>(), defShirt = new Set<string>(), defHead = new Set<string>();
    let opTop = 0, defTop = 0; const N = 11;
    for (let id = 10; id < 10 + N; id++) {
      const o = pal('operator', OP, id), d = pal('defender', DEF, id);
      opCloth.add(slot(o, 'cloth_uniform').map(f3).join()); defShirt.add(String(CM.slotPick('defender', 'cloth_plain', id))); defHead.add(String(CM.slotPick('defender', 'helmet', id)));
      // what the game camera sees most of from above: head, shoulders (shirt), chest (carrier)
      opTop += (lum(slot(o, 'helmet')) + lum(slot(o, 'cloth_uniform')) + lum(slot(o, 'webbing'))) / 3; defTop += (lum(slot(d, 'cloth_plain')) + lum(slot(d, 'webbing'))) / 2;
    }
    if (opCloth.size !== 1) f.push(`operators' uniform cloth differs man to man (${opCloth.size} colours)`);
    if (defShirt.size < 3 || defShirt.has('-1')) f.push(`only ${defShirt.size} shirts dealt over ${N} defenders [${[...defShirt]}]`);
    if (defHead.size < 3) f.push(`only ${defHead.size} heads over ${N} defenders`);
    if (!(defTop / N > 2 * (opTop / N))) f.push(`from above the sides are too close: operators ${f3(opTop / N)} vs defenders ${f3(defTop / N)} mean luminance`);
    const isTrim = (c: number[]) => c.map(f3).join() === OP[1].map(f3).join();
    const mk = slot(pal('operator', OP, 3), 'cloth_plain'); if (!isTrim(mk)) f.push(`an operator's marking (cloth_plain on the models: band, brassards) ${mk.map(f3)} is not his trim ${OP[1]}`);
    { const si = pal('operator', OP, 3, true); if (slot(si, 'cloth_plain').join() !== slot(si, 'cloth_uniform').join() || !isTrim(slot(si, 'accent'))) f.push('on the stand-in an operator\'s shirt is not his uniform, or his armband not his trim'); }
    const dl = slot(pal('defender', DEF, 3), 'lens'), ds = slot(pal('defender', DEF, 3), 'skin'); if (dl.join() !== ds.join()) f.push('a defender\'s lens faces are not his skin (he has no eye-pro)');   // (under the look the slot is his scarf / shawl: its own check below)
    // the look's defender palettes: nothing reads as a pair's marking (no cloth saturated unless it is a brown — red over green over blue with green at 0.72 of red or more —,
    // and on a head or a shoulder never green or blue the top channel), and no warm suit within 0.6 of a stop of the look's ground (mud brick, luminance 0.37)
    {
      const D = CM.CHAR_MATS_LOOK.defender!.slots as Record<string, { pick?: number[][] }>;
      for (const n of ['cloth_plain', 'cloth_uniform', 'helmet', 'accent', 'lens']) for (const c of D[n].pick ?? []) {
        const mx = Math.max(...c), sat = (mx - Math.min(...c)) / mx, brown = c[0] >= c[1] && c[1] >= c[2] && c[1] >= 0.72 * c[0];
        if (sat > (brown ? 0.56 : 0.45)) f.push(`the look's defender ${n} ${c} is a saturated colour (${f3(sat)}): it reads as a marking`);
        if (n !== 'cloth_plain' && n !== 'cloth_uniform' && (c[1] > c[0] + 0.005 || c[2] > c[0] + 0.005)) f.push(`the look's defender ${n} ${c} (head / shoulders) has green or blue its top channel`);
        if (n === 'cloth_plain' && c[0] > c[1] + 0.01 && lum(c) > 0.24 && lum(c) < 0.55) f.push(`the look's defender suit ${c} is a warm cloth at luminance ${f3(lum(c))}: inside 0.24‥0.55 it vanishes on mud brick`);
      }
      if (!(D.cloth_plain.pick ?? []).length) f.push('the look\'s defender suits are not a pick table');
    }
    // the suits of two pairs sit nearer each other than their raw tints do (the pull toward one issue colour), and the pair still shows on the marking
    { const RED: number[][] = [[0.13, 0.06, 0.06], [0.6, 0.07, 0.05]], BLUE: number[][] = [[0.06, 0.075, 0.13], [0.07, 0.2, 0.6]];
      const dist = (x: number[], y: number[]) => Math.hypot(x[0] - y[0], x[1] - y[1], x[2] - y[2]);
      const r = pal('operator', RED, 20), b = pal('operator', BLUE, 22), raw = dist(RED[0], BLUE[0]), worn = dist(slot(r, 'cloth_uniform'), slot(b, 'cloth_uniform'));
      if (!(worn < 0.75 * raw)) f.push(`two pairs' suits are as far apart as their raw tints (${f3(worn)} vs ${f3(raw)}): the pull toward the issue colour is gone`);
      if (!(dist(slot(r, 'accent'), slot(b, 'accent')) > 0.4)) f.push('the two pairs\' markings (accent: band, brassards) are not clearly apart'); }
    // each other side's ACCENT: on the cast its own row (the irregulars' pale naturals, never the colour of the top it is worn with; a tie darker than its shirt),
    // on the stand-in the shirt's row (the mannequin's accent is an armband region)
    { const dist = (x: number[], y: number[]) => Math.max(Math.abs(x[0] - y[0]), Math.abs(x[1] - y[1]), Math.abs(x[2] - y[2]));
      let near = 0; for (let id = 1; id <= 140; id++) { const d = pal('defender', DEF, id); if (dist(slot(d, 'accent'), slot(d, 'cloth_plain')) < 0.1 || (lum(slot(d, 'accent')) < lum(slot(d, 'cloth_plain')))) near++; }
      if (near) f.push(`${near} of 140 defenders wear a tee / straw hat the colour of their top (or darker than it)`);
      const QRF: number[][] = [[0.1, 0.11, 0.13], [0.05, 0.055, 0.06]], CIV: number[][] = [[0.78, 0.62, 0.08], [0.1, 0.12, 0.2]];
      { const c = pal('civilian', CIV, 7);   // (an odd id: the cast's office worker — the even ones are the cleaner, whose accent is her pale apron and kerchief)
        if (!(lum(slot(c, 'accent')) < 0.9 * lum(slot(c, 'cloth_plain')))) f.push(`a civilian's tie (accent) is not darker than his shirt`);
        const k = pal('civilian', CIV, 8); if (!(lum(slot(k, 'accent')) > 4 * lum(slot(k, 'cloth_uniform')))) f.push('the cleaner\'s kerchief and apron (accent, an even id) are not pale against her trousers'); }
      // the response team's accent is its white mark (helmet crown, vest back; the tie on that slot is under the vest): well clear of the helmet and the rig it sits on
      { const c = pal('responder', QRF, 7);
        if (!(lum(slot(c, 'accent')) > 3 * lum(slot(c, 'helmet')) && lum(slot(c, 'accent')) > 3 * lum(slot(c, 'webbing')))) f.push('the response team\'s mark (accent) does not stand out from its helmet and vest'); }
      // the response team's cap is not the black of the hair under it (hair is the `rubber` row), and is no lighter than their shirt
      { const r = pal('responder', QRF, 7);
        if (!(lum(slot(r, 'helmet')) > 2 * lum(slot(r, 'rubber')) && lum(slot(r, 'helmet')) <= lum(slot(r, 'cloth_plain')))) f.push('the response team\'s cap is hair-black, or lighter than their shirt'); }
      for (const [side, T] of [['defender', DEF], ['responder', QRF], ['civilian', CIV]] as [string, number[][]][]) { const s = pal(side, T, 7, true);
        if (slot(s, 'accent').join() !== slot(s, 'cloth_plain').join()) f.push(`on the stand-in a ${side}'s accent (the mannequin's armband) is not his shirt`); } }
    F.verdict(f.length === 0, 'no dice but the man\'s id; operators one issue with their trim as the marking, two pairs\' suits pulled together; defenders motley (shirts, heads) with bare eyes and a pale accent; a civilian\'s tie darker than his shirt, the cleaner\'s kerchief pale, the response team\'s mark clear of its helmet and vest; the response team\'s cap off hair-black; the stand-in\'s accent the shirt; the sides > 2× apart in value from above', f.join('; ') || `operators' cloth 1 colour · ${defShirt.size} shirts / ${defHead.size} heads over ${N} defenders · seen from above ${f3(opTop / N)} vs ${f3(defTop / N)}`); }
  { const f: string[] = [];
    { const P = (id: number) => { const p = new Float32Array(CM.CHAR_MAT_FLOATS); CM.writeCharPalette(p, 0, 'responder', [0.1, 0.11, 0.13], [0.05, 0.05, 0.05], id); const at = (sl: string) => { const o = 8 + CM.CHAR_SLOTS.indexOf(sl) * 8; return [p[o], p[o + 1], p[o + 2]].map(f3).join(); }; return at('cloth_plain') + ' / ' + at('cloth_uniform'); };
      // one issue, two tones: the duty shirt is the suit's colour and the trousers its dark half, the same on every man
      if (P(30) !== P(31) || P(30) !== [0.1, 0.11, 0.13].map(f3).join() + ' / ' + [0.05, 0.05, 0.05].map(f3).join()) f.push(`the responders' cloth is not their issue's two colours man for man (${P(30)} vs ${P(31)})`); }
    F.verdict(f.length === 0, 'the response team wears one issue in two tones', f.join('; ') || 'one duty shirt, one pair of trousers'); }
  return F.done();
}

// ---------------------------------------------------------------- charmats-off

// ---------------------------------------------------------------- charmats-shader
export async function charmatsShader() {
  const F = family('charmats-shader: the WGSL and the TS agree (gbuffer.wgsl × charMaterials.ts)');
  const src = readFileSync(`${ROOT}/src/render/shaders/gbuffer.wgsl`, 'utf8'); const f: string[] = [];
  const rows = /const CHAR_MAT_ROWS: u32 = (\d+)u;/.exec(src)?.[1]; if (Number(rows) !== CM.CHAR_MAT_ROWS) f.push(`CHAR_MAT_ROWS ${rows} in WGSL vs ${CM.CHAR_MAT_ROWS}`);
  if (!src.includes(`min(u32(slotGeo.x + 0.5), ${CM.CHAR_SLOT_COUNT - 1}u)`)) f.push(`vsSkin does not clamp the slot to ${CM.CHAR_SLOT_COUNT - 1}`);
  if (CM.CHAR_MAT_ROWS < 2 + 2 * CM.CHAR_SLOT_COUNT) f.push('the rows do not hold the slots');
  F.verdict(f.length === 0, 'row and slot counts agree', f.join('; ') || `${rows} rows · ${CM.CHAR_SLOT_COUNT} slots`);
  // THE MEN'S NOISE HOLDS NO LOOP AND NO LOCAL ARRAY, ON PURPOSE (look.wgsl manNoise, manH4: the eight corners of a cell hashed four at a time by vector
  // arithmetic). The same eight values as the one-corner hash it replaced, by its own integer steps written again here; MANLOOP_BREAK=1 = one step of
  // the four-at-a-time hash changed, and this must fail
  { const look = readFileSync(`${ROOT}/src/render/shaders/look.wgsl`, 'utf8'), at = look.indexOf('fn manNoise(p: vec3f) -> f32 {'), body = at < 0 ? '' : look.slice(at, look.indexOf('\n}', at)), h4 = /fn manH4\(h0: vec4u\) -> vec4f \{([^\n]*)\}/.exec(look)?.[1] ?? '';
    const brk = process.env.MANLOOP_BREAK === '1', M3 = brk ? 277803739 : 277803737;
    const one = (x: number, y: number, z: number) => { let h = (Math.imul((x + 4096) >>> 0, 73856093) ^ Math.imul((y + 4096) >>> 0, 19349663) ^ Math.imul((z + 4096) >>> 0, 83492791)) >>> 0; h = (Math.imul(h, 747796405) + 2891336453) >>> 0; h = Math.imul(((h >>> ((h >>> 28) + 4)) ^ h) >>> 0, 277803737) >>> 0; return ((h ^ (h >>> 22)) >>> 8) / 16777216; };
    const four = (h0: number) => { let h = (Math.imul(h0, 747796405) + 2891336453) >>> 0; h = Math.imul(((h >>> ((h >>> 28) + 4)) ^ h) >>> 0, M3) >>> 0; return ((h ^ (h >>> 22)) >>> 8) / 16777216; };
    let off = 0, n = 0;
    for (let k = 0; k < 4000; k++) { const ix = ((k * 7919) % 9001) - 4500, iy = ((k * 104729) % 7001) - 3500, iz = ((k * 1299709) % 5003) - 2500, xs = [ix, ix + 1, ix, ix + 1], ys = [iy, iy, iy + 1, iy + 1];
      for (let c = 0; c < 4; c++) { const xy = (Math.imul((xs[c] + 4096) >>> 0, 73856093) ^ Math.imul((ys[c] + 4096) >>> 0, 19349663)) >>> 0; for (const dz of [0, 1]) { n++; if (four((xy ^ Math.imul((iz + dz + 4096) >>> 0, 83492791)) >>> 0) !== one(xs[c], ys[c], iz + dz)) off++; } } }
    const text = body.length > 0 && !/\bfor\s*\(|\bloop\b|\bwhile\b|array</.test(body) && !/\bfor\s*\(|array</.test(h4) && h4.includes('var h = h0 * 747796405u + vec4u(2891336453u); h = ((h >> ((h >> vec4u(28u)) + vec4u(4u))) ^ h) * 277803737u; return vec4f((h ^ (h >> vec4u(22u))) >> vec4u(8u)) * (1.0 / 16777216.0);') && body.includes('let xy = (vec4u(vec4i(i.x, i.x + 1, i.x, i.x + 1) + vec4i(4096)) * 73856093u) ^ (vec4u(vec4i(i.y, i.y, i.y + 1, i.y + 1) + vec4i(4096)) * 19349663u);') && body.includes('let a = manH4(xy ^ vec4u(u32(i.z + 4096) * 83492791u)); let b = manH4(xy ^ vec4u(u32(i.z + 4097) * 83492791u));') && body.includes('return mix(mix(mix(a.x, a.y, u.x), mix(a.z, a.w, u.x), u.y), mix(mix(b.x, b.y, u.x), mix(b.z, b.w, u.x), u.y), u.z);');
    F.verdict(text && off === 0, 'the men\'s noise hashes a cell\'s eight corners four at a time with no loop and no local array, and they are the eight values the one-corner hash gave', `${n} corners, ${off} differ · the text ${text ? 'holds' : 'has moved'}`); }
  return F.done();
}

// ---------------------------------------------------------------- charmats-cast

if (import.meta.main) await main([charmatsSlots, charmatsTables, charmatsShader]);
