// THE CAST (public/assets/chars/cast.gltf, tools/chars/README.md; src/anim/gltf.ts loadGltfCast):
//   cast-loader   the loader against a SYNTHETIC cast cut out of the mannequin (parts by region, the joint list shuffled, the slots dealt round): every vertex and triangle
//                 arrives, joints are re-mapped by NAME (the skinned rest pose is the mannequin's to the bit), matSel = slot + 16 × part, variants → masks; and files off
//                 contract are REFUSED (an unknown slot, a joint not on the skeleton, a variant wearing a part that is not there, 33 parts)
//   cast-file     the SHIPPED file: loads; operators, defenders and a civilian exist; every variant under the triangle budget and the union under its own; every part worn by
//                 somebody; feet on the floor, inside the mannequin's reach; every slot name legal (by loading); the mesh-shadow tree builds — its size printed
//   cast-fit      the round-cone proxy fit per VARIANT (render/sdfrig.ts fitRig(ch, mask)): finite, 1–30 cm, deterministic; an operator's trunk is not a defender's
//   bun run tools/qa/cast-probes.ts
import { characterSet, main, ROOT } from './here.ts';
import { writeFileSync, mkdtempSync, existsSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';

const { loadGltfCharacter, loadGltfCast, SLOT_NAMES, CAST_PART_STEP, CAST_MAX_PARTS, castSide, castFile: castFileName, CAST_FILE, CAST_FILE_LOOK } = await import(`${ROOT}/src/anim/gltf.ts`);
const { fitRig, fitRigCast, rigTable, RIG_SLOTS, RIG_PAD_MAX } = await import(`${ROOT}/src/render/sdfrig.ts`);
const { meshBvhFor } = await import(`${ROOT}/src/render/meshShadows.ts`);
const { MESH_MAX_CHARS } = await import(`${ROOT}/src/render/meshbvh.ts`);

let fails = 0;
const verdict = (ok: boolean, label: string, detail = '') => { if (!ok) fails++; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}${detail ? `\n    ${detail}` : ''}`); return ok; };
export const castFailures = () => fails;
const MANNEQUIN = 'file://' + ROOT + '/public/assets/ual/AnimationLibrary_Godot_Standard.gltf';
const { LOOK, setLookPlace, lookDress, lookOffice, placeOfQuery } = await import(`${ROOT}/src/scene/look.ts`);
const CAST_PATH = ROOT + '/public/assets/chars/' + castFileName();
const pathOf = (look: boolean) => ROOT + '/public/assets/chars/' + (look ? CAST_FILE_LOOK : CAST_FILE);
/** the budgets tools/chars builds to (README there): one man, and everything in the file */
const MAX_VARIANT_TRIS = 14000, MAX_UNION_TRIS = 45000;

// ---- a synthetic cast cut out of the mannequin: parts by the height of a triangle's centre, slots dealt round, the joint list reversed (so only a by-name mapping skins it right)
function synthCast(ch: any, opts: { slot?: string; joint?: string; wears?: string; parts?: number } = {}): string {
  const m = ch.mesh; const T = m.indices.length / 3;
  const names = opts.parts ? Array.from({ length: opts.parts }, (_, i) => `p${i}`) : ['legs', 'trunk', 'arms_head'];
  const partOf = (t: number) => { const y = (m.positions[m.indices[t * 3] * 3 + 1] + m.positions[m.indices[t * 3 + 1] * 3 + 1] + m.positions[m.indices[t * 3 + 2] * 3 + 1]) / 3; return opts.parts ? t % opts.parts : y < 0.95 ? 0 : y < 1.35 ? 1 : 2; };
  const J = ch.skin.joints.length; const order = Array.from({ length: J }, (_, k) => J - 1 - k);   // file joint f = base joint order[f]
  const inv = new Array(J); order.forEach((b, f) => { inv[b] = f; });
  const chunks: Uint8Array[] = []; let off = 0; const views: any[] = [], accessors: any[] = [];
  const push = (a: ArrayBufferView, comp: number, type: string, count: number) => { const u8 = new Uint8Array(a.buffer, a.byteOffset, a.byteLength); const pad = (4 - (u8.length % 4)) % 4; chunks.push(u8); if (pad) chunks.push(new Uint8Array(pad)); views.push({ buffer: 0, byteOffset: off, byteLength: u8.length }); accessors.push({ bufferView: views.length - 1, componentType: comp, type, count }); off += u8.length + pad; return accessors.length - 1; };
  const nodes: any[] = order.map(b => ({ name: opts.joint && b === 3 ? opts.joint : ch.nodes[ch.skin.joints[b]].name }));
  const meshes: any[] = [];
  names.forEach((pn, pi) => {
    // one primitive per part: its triangles, vertices re-indexed
    const map = new Map<number, number>(); const idx: number[] = [];
    for (let t = 0; t < T; t++) { if (partOf(t) !== pi) continue; for (let c = 0; c < 3; c++) { const v = m.indices[t * 3 + c]; let n = map.get(v); if (n === undefined) { n = map.size; map.set(v, n); } idx.push(n); } }
    const vs = [...map.keys()]; const n = vs.length;
    const P = new Float32Array(n * 3), N = new Float32Array(n * 3), W = new Float32Array(n * 4), Jn = new Uint8Array(n * 4);
    vs.forEach((v, k) => { for (let c = 0; c < 3; c++) { P[k * 3 + c] = m.positions[v * 3 + c]; N[k * 3 + c] = m.normals[v * 3 + c]; } for (let c = 0; c < 4; c++) { W[k * 4 + c] = m.weights[v * 4 + c]; Jn[k * 4 + c] = inv[m.joints[v * 4 + c]]; } });
    const prim = { attributes: { POSITION: push(P, 5126, 'VEC3', n), NORMAL: push(N, 5126, 'VEC3', n), JOINTS_0: push(Jn, 5121, 'VEC4', n), WEIGHTS_0: push(W, 5126, 'VEC4', n) }, indices: push(new Uint32Array(idx), 5125, 'SCALAR', idx.length), material: pi % SLOT_NAMES.length };
    meshes.push({ primitives: [prim] }); nodes.push({ name: pn, mesh: meshes.length - 1, skin: 0 });
  });
  const total = chunks.reduce((a, c) => a + c.length, 0); const bin = new Uint8Array(total); { let o = 0; for (const c of chunks) { bin.set(c, o); o += c.length; } }
  const materials = SLOT_NAMES.map((s: string, i: number) => ({ name: opts.slot && i === 1 ? opts.slot : s }));
  const variants: any = opts.parts ? { all: { side: 'operator', parts: names } } : { operator_1: { side: 'operator', parts: ['legs', 'trunk', 'arms_head'] }, defender_1: { side: 'defender', parts: ['legs', opts.wears ?? 'arms_head'] } };
  const g = { asset: { version: '2.0' }, scene: 0, scenes: [{ nodes: nodes.map((_, i) => i) }], nodes, meshes, materials, accessors, bufferViews: views, skins: [{ joints: order.map((_, f) => f) }],
    buffers: [{ byteLength: total, uri: 'data:application/octet-stream;base64,' + Buffer.from(bin).toString('base64') }], extras: { cast: { version: 1, variants } } };
  const dir = mkdtempSync(join(tmpdir(), 'cast-probe-')); const p = join(dir, 'cast.gltf'); writeFileSync(p, JSON.stringify(g)); return 'file://' + p;
}

export async function castLoader() {
  console.log('\n=== cast-loader: the cast loader against a synthetic file cut out of the mannequin ===');
  const base = await loadGltfCharacter(MANNEQUIN);
  const ch = await loadGltfCast(synthCast(base), base);
  const m = ch.mesh, bm = base.mesh;
  verdict(m.indices.length === bm.indices.length && ch.cast.parts.join() === 'legs,trunk,arms_head', `three parts, every triangle arrives (${m.indices.length / 3} of ${bm.indices.length / 3})`, `parts ${ch.cast.parts.join(' ')} · tris ${ch.cast.partTris.join(' ')}`);
  verdict(ch.nodes === base.nodes && ch.skin === base.skin && ch.clips === base.clips, 'skeleton, skin and clips are the mannequin\'s own objects');
  // by-name joint mapping: every cast vertex has the position / joints / weights of the mannequin vertex it was cut from (matched by position; the file's joint list was reversed)
  { const key = (P: Float32Array, v: number) => `${P[v * 3]},${P[v * 3 + 1]},${P[v * 3 + 2]}`; const at = new Map<string, number>(); for (let v = 0; v < bm.vertexCount; v++) at.set(key(bm.positions, v), v);
    let bad = 0; for (let v = 0; v < m.vertexCount; v++) { const b = at.get(key(m.positions, v)); if (b === undefined) { bad++; continue; } for (let c = 0; c < 4; c++) { if (m.weights[v * 4 + c] > 0 && (m.joints[v * 4 + c] !== bm.joints[b * 4 + c] || Math.abs(m.weights[v * 4 + c] - bm.weights[b * 4 + c]) > 1e-6)) { bad++; break; } } }
    verdict(bad === 0, 'joints re-mapped by name: every vertex carries its source vertex\'s joints and weights', `${bad} differ`); }
  { let ok = true; for (let v = 0; v < m.vertexCount; v++) { const ms = m.matSel[v]; const part = Math.floor(ms / CAST_PART_STEP), slot = ms % CAST_PART_STEP; if (part !== m.part[v] || slot !== part % SLOT_NAMES.length || slot >= SLOT_NAMES.length) ok = false; }
    verdict(ok, `matSel = slot + ${CAST_PART_STEP} × part on every vertex, part mirrored in mesh.part`); }
  { const op = ch.cast.variants.find((v: any) => v.name === 'operator_1'), df = ch.cast.variants.find((v: any) => v.name === 'defender_1');
    verdict(op?.mask === 0b111 && df?.mask === 0b101 && op.side === 'operator' && df.side === 'defender' && df.tris === ch.cast.partTris[0] + ch.cast.partTris[2], 'variants → part masks and triangle counts', `operator_1 ${op?.mask} · defender_1 ${df?.mask} (${df?.tris} tris)`);
    verdict(castSide(ch.cast, 'civilian').length === 2, 'a side with no variant of its own falls back to all of them'); }
  const refused = async (label: string, o: any, want: RegExp) => { let msg = ''; try { await loadGltfCast(synthCast(base, o), base); } catch (e: any) { msg = String(e?.message ?? e); } verdict(want.test(msg), `refused: ${label}`, msg || 'it loaded'); };
  await refused('a material that is not a slot', { slot: 'leather' }, /not a slot/);
  await refused('a joint that is not on the skeleton', { joint: 'DEF-tail' }, /not on the mannequin/);
  await refused('a variant wearing a part that is not there', { wears: 'cape' }, /not a part/);
  await refused(`${CAST_MAX_PARTS + 1} parts`, { parts: CAST_MAX_PARTS + 1 }, /at most/);
  { const full = await loadGltfCast(synthCast(base, { parts: CAST_MAX_PARTS }), base); verdict(full.cast.variants[0].mask === 0xffffffff, `${CAST_MAX_PARTS} parts load, the mask is all 32 bits`); }
}

export async function castFile() { await castFileOf(false); await castFileOf(true); }
async function castFileOf(look: boolean) {
  console.log(`\n=== cast-file: the shipped public/assets/chars/${look ? CAST_FILE_LOOK : CAST_FILE}${look ? ' (the look)' : ''} ===`);
  if (!verdict(existsSync(pathOf(look)), 'the file exists')) return;
  const ch = await characterSet(look); const cast = ch.cast, m = ch.mesh;
  const union = m.indices.length / 3;
  console.log(`  ${cast.parts.length} parts, ${m.vertexCount} vertices, ${union} triangles in all`);
  console.log('  ' + cast.parts.map((p: string, i: number) => `${p} ${cast.partTris[i]}`).join(' · '));
  for (const v of cast.variants) console.log(`  ${v.name.padEnd(14)} ${v.side.padEnd(9)} ${String(v.tris).padStart(6)} tris  ${v.parts.join(' ')}`);
  const n = (s: string) => cast.variants.filter((v: any) => v.side === s).length;
  verdict(n('operator') >= 5 && n('defender') >= 4 && n('civilian') >= 1, `five operators, four defenders and a civilian at least (${n('operator')} / ${n('defender')} / ${n('civilian')})`);
  { const rs = cast.variants.filter((v: any) => v.side === 'responder'); const alien = /^(op_|kit_|nvg_|helmet$|hat_|bandana|tank_top|jacket_open|shirt_loose|local_suit|waistcoat|patu|scarf|pakol|turban|kufi|sandals)/;
    verdict(rs.length >= 1 && rs.every((v: any) => !v.parts.some((p: string) => alien.test(p))) && new Set(rs.map((v: any) => v.parts.filter((p: string) => p !== 'moustache' && p !== 'rsp_radio' && !p.startsWith('hair_') && !p.startsWith('head')).join())).size === 1,
      `the response element has an outfit of its own: one set of clothes for all ${rs.length} (a face, hair and the radio one of them carries aside), none of the operators' kit, none of the irregulars' hats or loose shirts`, rs.map((v: any) => `${v.name}: ${v.parts.join(' ')}`).join(' · ')); }
  const over = cast.variants.filter((v: any) => v.tris > MAX_VARIANT_TRIS);
  verdict(over.length === 0, `every variant at or under ${MAX_VARIANT_TRIS} triangles (the mannequin is 13,743)`, over.map((v: any) => `${v.name} ${v.tris}`).join(' · '));
  verdict(union <= MAX_UNION_TRIS && m.vertexCount < 1 << 16, `the union at or under ${MAX_UNION_TRIS} triangles (${union}), under 65,536 vertices (${m.vertexCount})`);
  { let all = 0; for (const v of cast.variants) all |= v.mask; const idle = cast.parts.filter((_: string, i: number) => !((all >>> i) & 1)); verdict(idle.length === 0, 'every part is worn by some variant', idle.join(' ')); }
  { const masks = new Set(cast.variants.map((v: any) => v.mask)); verdict(masks.size === cast.variants.length, 'no two variants wear the same parts'); }
  verdict(m.bboxMin[1] > -0.02 && m.bboxMax[1] < 1.98 && m.bboxMin[0] > -1.02 && m.bboxMax[0] < 1.02 && m.bboxMin[2] > -0.5 && m.bboxMax[2] < 0.5, 'feet on the floor, inside the mannequin\'s reach', `x ${m.bboxMin[0].toFixed(3)}‥${m.bboxMax[0].toFixed(3)} · y ${m.bboxMin[1].toFixed(3)}‥${m.bboxMax[1].toFixed(3)} · z ${m.bboxMin[2].toFixed(3)}‥${m.bboxMax[2].toFixed(3)}`);
  { // every variant stands on the floor and has a head: its own vertices span the height
    const bad: string[] = []; for (const v of cast.variants) { let lo = 1e9, hi = -1e9; for (let i = 0; i < m.vertexCount; i++) if ((v.mask >>> m.part[i]) & 1) { const y = m.positions[i * 3 + 1]; lo = Math.min(lo, y); hi = Math.max(hi, y); } if (!(lo < 0.03 && hi > 1.7)) bad.push(`${v.name} ${lo.toFixed(2)}‥${hi.toFixed(2)}`); }
    verdict(bad.length === 0, 'every variant reaches from the floor to over 1.7 m', bad.join(' · ')); }
  { const used = new Set<number>(); for (let i = 0; i < m.vertexCount; i++) used.add(m.matSel[i] % CAST_PART_STEP); console.log('  slots in use: ' + [...used].sort().map(s => SLOT_NAMES[s]).join(' '));
    const need = ['skin', 'cloth_uniform', 'cloth_plain', 'webbing', 'boot', 'helmet'].filter(s => !used.has(SLOT_NAMES.indexOf(s))); verdict(need.length === 0, 'skin, both cloths, webbing, boot and helmet are all in use', need.join(' ')); }
  { // degenerate triangles and normals
    let degen = 0, badN = 0; const P = m.positions, I = m.indices;
    for (let t = 0; t < union; t++) { const a = I[t * 3] * 3, b = I[t * 3 + 1] * 3, c = I[t * 3 + 2] * 3; const ux = P[b] - P[a], uy = P[b + 1] - P[a + 1], uz = P[b + 2] - P[a + 2], vx = P[c] - P[a], vy = P[c + 1] - P[a + 1], vz = P[c + 2] - P[a + 2]; if (Math.hypot(uy * vz - uz * vy, uz * vx - ux * vz, ux * vy - uy * vx) < 1e-10) degen++; }
    for (let v = 0; v < m.vertexCount; v++) { const l = Math.hypot(m.normals[v * 3], m.normals[v * 3 + 1], m.normals[v * 3 + 2]); if (!(l > 0.9 && l < 1.1)) badN++; }
    verdict(degen <= union * 0.002 && badN === 0, `unit normals everywhere, zero-area triangles under 0.2 % (${degen})`, `${badN} bad normals`); }
  { const t0 = performance.now(); const bvh = meshBvhFor(m); const ms = performance.now() - t0; const mb = ((4 + MESH_MAX_CHARS * bvh.numNodes * 2 + MESH_MAX_CHARS * bvh.numTris * 3) * 16) / 1048576;
    verdict(bvh.numTris === union && bvh.maxDepth + 2 <= 64, `the mesh-shadow tree builds: ${bvh.numNodes} nodes over ${bvh.numTris} triangles, ${bvh.maxDepth + 1} levels, ${ms.toFixed(0)} ms — ${mb.toFixed(1)} MB for ${MESH_MAX_CHARS} men (the mannequin's is 7.4)`); }
}

export async function castFit() { await castFitOf(false); await castFitOf(true); }
async function castFitOf(look: boolean) {
  console.log(`\n=== cast-fit: each variant's round-cone proxy fit — ${look ? CAST_FILE_LOOK : CAST_FILE} ===`);
  if (!verdict(existsSync(pathOf(look)), 'the file exists')) return;
  const ch = await characterSet(look); const cast = ch.cast;
  const base = await loadGltfCharacter(MANNEQUIN); const mf = fitRig(base);
  const row = (name: string, f: any) => `  ${name.padEnd(14)} ` + f.slots.map((s: any) => `${s.name.replace(/upper|fore/, (m: any) => m[0])} ${(s.r0 * 100).toFixed(0)}/${(s.r1 * 100).toFixed(0)}`).join(' ');
  console.log(row('mannequin', mf));
  const fits = new Map<string, any>(); const bad: string[] = [], few: string[] = [];
  for (const v of cast.variants) { const f = fitRig(ch, v.mask); fits.set(v.name, f); console.log(row(v.name, f));
    for (const s of f.slots) { if (!(Number.isFinite(s.r0) && Number.isFinite(s.r1) && s.r0 >= 0.01 && s.r0 <= 0.3 && s.r1 >= 0.01 && s.r1 <= 0.3)) bad.push(`${v.name}.${s.name} ${s.r0.toFixed(3)}/${s.r1.toFixed(3)}`); if (s.n < 8) few.push(`${v.name}.${s.name} ${s.n}`); } }
  verdict(bad.length === 0, 'every radius finite and within 1–30 cm', bad.join(' · '));
  verdict(few.length === 0, 'every slot of every variant found vertices to fit (none fell back to the hand radius)', few.join(' · '));
  { const pads = cast.variants.map((v: any) => `${v.name} ${((fits.get(v.name).pad ?? -1) * 100).toFixed(0)}`); const off = cast.variants.filter((v: any) => { const p = fits.get(v.name).pad; return !(p >= 0 && p <= RIG_PAD_MAX); });
    verdict(off.length === 0 && mf.pad === undefined, `the cull sphere's pad per outfit is measured and capped (cm outside the cones: ${pads.join(' · ')}); none on the mannequin`, off.map((v: any) => v.name).join(' ')); }
  { const v = cast.variants[0]; const again = fitRig({ ...ch, mesh: { ...ch.mesh } }, v.mask); const f = fits.get(v.name); let same = true; for (let i = 0; i < RIG_SLOTS; i++) if (f.r0[i] !== again.r0[i] || f.r1[i] !== again.r1[i]) same = false; verdict(same, 'two solves of one variant agree bit for bit'); }
  { const op = castSide(cast, 'operator')[0], df = castSide(cast, 'defender')[0]; const a = fits.get(op.name), b = fits.get(df.name); let d = 0; for (let i = 0; i < RIG_SLOTS; i++) d = Math.max(d, Math.abs(a.r0[i] - b.r0[i]), Math.abs(a.r1[i] - b.r1[i]));
    verdict(op.mask === df.mask || d > 0.003, `an operator's proxies are not a defender's (largest radius difference ${(d * 1000).toFixed(0)} mm)`);
    console.log(rigTable(a).split('\n').map((l: string) => '  ' + l).join('\n')); }
  { const all = fitRig(ch); verdict(all.verts === ch.mesh.vertexCount, 'mask 0 on the cast = the union of every part (the shared fall-back fit)'); }
}

const seeded = (s: number) => () => { s |= 0; s = (s + 0x6d2b79f5) | 0; let t = Math.imul(s ^ (s >>> 15), 1 | s); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; };

if (import.meta.main) await main([castLoader, castFile, castFit], castFailures);
