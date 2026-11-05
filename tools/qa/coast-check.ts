// THE COAST SKIN (src/scene/gwotPalette.ts COAST_PALETTE, tools/levels/coastSkin.ts): are its paints its own, and is a kit piece in its coast clothes
// what it was to everything but the eye?
//   bun run tools/qa/coast-check.ts                  exit 0 = all pass, 1 = a line failed (named)
//   COAST_BREAK=1 bun run tools/qa/coast-check.ts    the red path: the skin lays every coast row as itself, never standing for what the kit's paint
//                                                    stood for (coastSkin.ts reads the word) — line 3 must FAIL
// WHAT IS HELD:
//   1 its own      every coast row, and every stand-in of it (one a thing a paint can be to a round), has numbers NO OTHER PALETTE'S paint or stand-in
//                  has (the kit's, the snow's and their wet lot, the office's) and no other coast row's: a round knows a paint by its numbers, and the
//                  coast rows join the table it reads only where a key is free (gwotPalette.ts gwotSimOf), so a taken key would be the other paint's
//                  stuff with no error. And each reads back as what it was laid to be
//   2 its words    every row says a stuff the look HAS (scene/stuff.ts; render/lookTex.ts LOOK_NAMES) or plain, and none says a stuff of the old
//                  setting (a mud wall). The rows that wait for a stuff the look does not have yet are named, with what they say meanwhile; and the
//                  stuffs of the look this skin never uses are listed (the look's table is nearly full: those are the ones a later stuff could take)
//   3 the same     the showroom (tools/levels/coast-yard.ts) built over the plain host and over the skin's is THE SAME LEVEL box for box (place,
//                  size, turn, flags, shape), and every box the kit laid is to the simulation's table, to a hit by its colour and to a blast what
//                  it was in the kit's own paint; a box that said a word of its own says it still
//   4 dressed      in the skinned showroom no box, trim or prop part wears a paint of the kit's palette (or one of its stand-ins), and none says
//                  'mud wall'
// No picture is judged here.
import type { Vec3 } from '../../src/math/vec';
import type { Box } from '../../src/scene/boxes';
import { GWOT_PALETTE, GWOT_SIM_NAMES, SNOW_PALETTE, COAST_PALETTE, COAST_SIM, gwotAs, gwotWet, coastAs, gwotSimOf, type GwotMat, type CoastMat } from '../../src/scene/gwotPalette';
import { OFFICE_PALETTE } from '../../src/scene/officeGen';
import { stuffOfName, BOX_STUFFS } from '../../src/scene/stuff';
import { LOOK_NAMES, lookStuffOf } from '../../src/render/lookTex';
import { surfOf } from '../../src/game/particleFx';
import { breakClassOf } from '../../src/game/furnblast';
import { build } from '../levels/coast-yard';
import { lines } from './kit-stand';

const BREAK = process.env.COAST_BREAK === '1', { check, failed } = lines();
const key = (a: readonly number[]) => a.join(','), ROWS = Object.keys(COAST_PALETTE) as CoastMat[], KIT = Object.keys(GWOT_PALETTE) as GwotMat[];
console.log(`\n=== coast: the coast skin's paints are its own, and a piece in them is what it was${BREAK ? ' (COAST_BREAK: every coast row laid as itself)' : ''} ===`);

// 1. its own numbers
{ const others = new Map<string, string>();
  for (const m of KIT) { others.set(key(GWOT_PALETTE[m]), m); for (const s of GWOT_SIM_NAMES) others.set(key(gwotAs(m, s)), `${m} as ${s}`); others.set(key(gwotWet(gwotAs(m, 'asphalt'))), `${m} wet`); }
  for (const [n, a] of Object.entries(SNOW_PALETTE)) { others.set(key(a), n); others.set(key(gwotWet(a)), `${n} wet`); }
  for (const [n, a] of Object.entries(OFFICE_PALETTE as Record<string, Vec3>)) others.set(key(a), `the office's ${n}`);
  const mine = new Map<string, string>(), taken: string[] = [], twice: string[] = [], misread: string[] = []; let n = 0;
  for (const m of ROWS) for (const s of [COAST_SIM[m], ...GWOT_SIM_NAMES]) { const a = coastAs(m, s), k = key(a); n++;
    if (others.has(k)) taken.push(`${m} as ${s} has the numbers of ${others.get(k)}`);
    if (mine.has(k) && mine.get(k) !== `${m} as ${s}`) twice.push(`${m} as ${s} and ${mine.get(k)}`); mine.set(k, `${m} as ${s}`);
    if (gwotSimOf(a) !== s) misread.push(`${m} as ${s} reads ${gwotSimOf(a)}`); }
  check(!taken.length && !twice.length && !misread.length, `1 every one of the ${ROWS.length} coast rows and of their stand-ins has numbers no other palette's paint has, no two of them share any, and each reads back as what it was laid to be`,
    `${n} paints asked against ${others.size} of the kit's, the snow's and the office's${[...taken, ...twice, ...misread].length ? ' · ' + [...taken, ...twice, ...misread].slice(0, 6).join(' · ') : ''}`); }

// 2. its words
{ const OLD = new Set(['mud wall', 'field stone', 'mud brick']), bad: string[] = [], used = new Set<string>();
  for (const m of ROWS) { const w = stuffOfName(m); if (w === undefined) bad.push(`${m} says nothing`); else if (OLD.has(w)) bad.push(`${m} says '${w}', a stuff of the old setting`); else if (w !== 'plain' && !BOX_STUFFS.includes(w)) bad.push(`${m} says '${w}', which is no stuff a box may say`); else used.add(w); }
  const WAITS: Partial<Record<CoastMat, string>> = { sand: 'sand', roofTile: 'roof tile' };
  const never = LOOK_NAMES.filter(nm => lookStuffOf(nm) > 0 && !used.has(nm));
  check(!bad.length, `2 every coast row says a stuff the look has, and none a stuff of the old setting`,
    `${ROWS.length} rows in ${used.size} words · waiting for a stuff the look does not have yet: ${Object.entries(WAITS).map(([m, w]) => `${m} (says '${stuffOfName(m)}', waits for ${w})`).join(', ')} · drawn plain until the look has leaves: ${ROWS.filter(m => stuffOfName(m) === 'waits').join(', ') || 'none'} · THE LOOK'S STUFFS NO ROW OF THIS SKIN SAYS: ${never.join(', ')} (a vehicle's body says carpaint itself, in any skin)${bad.length ? ' · ' + bad.join(' · ') : ''}`); }

// 3, 4. the showroom over the plain host and over the skin's
{ const plain = build(false).level, skin = build(true).level, P = plain.boxes as Box[], S = skin.boxes as Box[];
  const kitKeys = new Set<string>(); for (const m of KIT) { kitKeys.add(key(GWOT_PALETTE[m])); for (const s of GWOT_SIM_NAMES) kitKeys.add(key(gwotAs(m, s))); }
  const shapeOf = (b: Box) => JSON.stringify(b.shape ? { ...b.shape, with: b.shape.with?.map(t => ({ off: t.off, h: t.h, round: t.round, arc: t.arc, cut: t.cut })) } : null);
  const moved: string[] = [], unlike: string[] = [], unsaid: string[] = []; let kit = 0;
  if (P.length !== S.length) moved.push(`${P.length} boxes plain, ${S.length} skinned`);
  for (let i = 0; i < Math.min(P.length, S.length); i++) { const a = P[i], b = S[i];
    if (key(a.c) !== key(b.c) || key(a.h) !== key(b.h) || a.yaw !== b.yaw || a.flags !== b.flags || shapeOf(a) !== shapeOf(b)) moved.push(`box ${i}`);
    if (!kitKeys.has(key(a.albedo))) continue; kit++;   // (a box the kit laid in one of its rows or stand-ins: a tone is drawn only and is asked nothing by a round)
    const was = [gwotSimOf(a.albedo), surfOf(a.albedo), breakClassOf(a.albedo)], is = [gwotSimOf(b.albedo), surfOf(b.albedo), breakClassOf(b.albedo)];
    if (was.join() !== is.join()) unlike.push(`box ${i} [${skin === undefined ? '' : (b.name ?? '')}] was ${was.join(' / ')}, is ${is.join(' / ')}`);
    if (a.stuff === 'carpaint' && b.stuff !== 'carpaint') unsaid.push(`box ${i} said carpaint, says ${b.stuff}`); }
  check(!moved.length && !unlike.length && !unsaid.length && kit > 100, `3 the showroom over the skin's host is the plain one box for box, every box the kit laid in a row of its own is to the simulation's table, to a hit and to a blast what it was, and a vehicle's body still says it is one`,
    `${S.length} boxes, ${kit} of them the kit's own rows and stand-ins${moved.length ? ` · MOVED: ${moved.slice(0, 5).join(', ')}` : ''}${unlike.length ? ` · ${unlike.length} NOT WHAT THEY WERE: ${unlike.slice(0, 4).join(' · ')}` : ''}${unsaid.length ? ' · ' + unsaid.slice(0, 3).join(' · ') : ''}`);
  const left: string[] = [];
  S.forEach((b, i) => { if (kitKeys.has(key(b.albedo))) left.push(`box ${i}`); if (b.stuff === 'mud wall') left.push(`box ${i} says mud wall`); for (const t of b.shape?.with ?? []) if (t.albedo && kitKeys.has(key(t.albedo))) left.push(`a trim of box ${i}`); });
  for (const p of skin.props) for (const part of p.parts) if (kitKeys.has(key(part.albedo))) left.push(`a part of the prop at ${p.x.toFixed(1)}, ${p.z.toFixed(1)}`);
  check(BREAK || !left.length, `4 in the skinned showroom nothing wears a paint of the kit's palette, and nothing says mud wall`, `${S.length} boxes and ${skin.props.length} props asked${left.length ? ` · LEFT IN THE KIT'S PAINT: ${left.slice(0, 8).join(', ')}${left.length > 8 ? ` and ${left.length - 8} more` : ''}` : ''}`); }

console.log(failed() ? `=== coast: ${failed()} FAILED ===` : '=== coast: ALL PASS ===');
process.exit(failed() ? 1 : 0);
