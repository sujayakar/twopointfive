// THE LIBRARY'S CATALOGUE: every object a level can lay and every paint it can wear, ONCE, written from the kits themselves.
//   bun run tools/levels/library.ts                 the catalogue's summary, and what breaks the library's rules today
//   bun run tools/levels/library.ts --write         write docs/library.md (the kits' pieces and paints: it changes only when a kit or a showroom does)
//   bun run tools/levels/library.ts --check         write nothing; exit 1 if docs/library.md is not what the kits give now, or a rule is broken
//   bun run tools/levels/library.ts --uses FILE     which level lays which piece, as a page (it changes with every generator, so it is no file of the repo)
//   bun run tools/levels/library.ts --pictures DIR  a PNG of each piece (DIR/<kit>.<piece>.png), drawn from its boxes (libraryDraw.ts)
//   bun run tools/levels/library.ts --json FILE     everything below as data (a gallery page and the dressing tasks' tables read it)
// HOW IT KNOWS. A kit hands its pieces out through src/scene/library.ts `pieces(kit, fns)`; with a watcher set, each piece laid is told. So this file
//   1. builds EVERY generator under tools/levels with the watcher on (and runs a look file's `finish` on what its plain generator built): which piece each
//      level lays, how often, and with what arguments;
//   2. takes for each piece the call a SHOWROOM makes (levels/gwot-yard.json stands the look's kit, levels/library-hall.json the older kits'), else the first
//      call any level makes, and lays that one call alone in a fresh kit: the boxes, loose parts and lamps it adds are the piece — its size, what it costs,
//      what it is to a man, its paints; and, from the door again, WHOSE piece it is: a piece that lays nothing of its own, only one piece of another kit
//      (the lived kit's chair is the office's, named as a room's lived thing), is that piece, and is not a second way of building the name;
//   3. reads what the piece is FOR, and its arguments, from the comment and the line that define it in the kit's own file.
// Nothing about a piece is typed here: the one table below says where each kit lives and how to stand it up empty.
// THE LIBRARY'S RULES (--check): every piece stands in a showroom; every piece says what it is; no piece tops out where a body is stopped on floor the nav
// grid calls free (0.20 to 0.25 m: tools/qa/lived-check.ts holds the same band on a level); every kit that hands pieces out has its row here; docs/library.md
// is up to date.
import { basename, resolve } from 'node:path';
import { existsSync, mkdirSync, readFileSync, readdirSync, writeFileSync } from 'node:fs';
import type { Vec3 } from '../../src/math/vec';
import { BoxFlag, type Box } from '../../src/scene/boxes';
import type { Level, LightDef, PropDef } from '../../src/scene/level';
import { watchPieces } from '../../src/scene/library';
import { officeKit, buildOfficeProcedural, OFFICE_PALETTE } from '../../src/scene/officeGen';
import { KIT_PALETTE } from '../../src/scene/levelKit';
import { gwotKit, gwotOfOffice } from '../../src/scene/gwotKit';
import { GWOT_PALETTE, GWOT_FINISH, GWOT_SIM, GWOT_SIM_NAMES, GWOT_SURFACE, SNOW_PALETTE, SNOW_SIM, ROUGH_CLOTH, gwotAs, type GwotMat, type SnowMat } from '../../src/scene/gwotPalette';
import { findOctagons } from '../../src/scene/round';
import { ALL_MISSIONS } from '../../src/scene/scenarioList';
import { encodePng } from '../qa/png';
import { livedKit, LIVED_PAINT } from './livedKit';
import { livedPieces, INK } from './livedPieces';
import { livedQuiet } from './lived-quiet';
import { livedTrain } from './lived-train';
import { drawBoxes, type DrawBox } from './libraryDraw';

const ROOT = resolve(import.meta.dir, '../..'), HERE = import.meta.dir;
const SHOWROOMS = ['gwot-yard', 'library-hall'], PAGE = 'docs/library.md';
const argv = process.argv.slice(2), flag = (f: string) => argv.includes(f);
const valueOf = (f: string) => { const i = argv.indexOf(f); if (i < 0) return undefined; const v = argv[i + 1]; if (!v || v.startsWith('--')) { console.error(`${f} wants a path`); process.exit(2); } return v; };
const firstLine = (e: unknown) => String((e as Error).message ?? e).split('\n')[0];

// ------------------------------------------------------------------ the kits: where each lives, and how to stand one up empty ----
interface Stood { fns: Record<string, unknown>; boxes: Box[]; props: PropDef[]; lights: LightDef[]; switchables: Level['switchables']; seal?: () => void }
const listsOf = (K: ReturnType<typeof officeKit>) => ({ boxes: K.boxes, props: K.props, lights: K.lights, switchables: K.switchables });
const KITS: { name: string; file: string; stand(): Stood }[] = [
  { name: 'gwotKit', file: 'src/scene/gwotKit.ts', stand() { const K = officeKit({ groups: [] }); return { fns: gwotKit(gwotOfOffice(K)), ...listsOf(K) }; } },
  { name: 'officeKit', file: 'src/scene/officeGen.ts', stand() { const K = officeKit({ groups: [] }); return { fns: K, ...listsOf(K) }; } },
  { name: 'livedKit', file: 'tools/levels/livedKit.ts', stand() { const K = officeKit({ groups: [] }), L = livedKit(K); L.room('show'); return { fns: L, ...listsOf(K) }; } },
  { name: 'livedPieces', file: 'tools/levels/livedPieces.ts', stand() {
    const K = officeKit({ groups: [] }), P = livedPieces({ add: K.add, grp: K.grp, prop: K.prop }, OFFICE_PALETTE as unknown as Parameters<typeof livedPieces>[1]); P.room('show');
    return { fns: P, ...listsOf(K) }; } },
  { name: 'livedQuiet', file: 'tools/levels/lived-quiet.ts', stand() { const K = officeKit({ groups: [] }), Q = livedQuiet(K, gwotKit(gwotOfOffice(K)).M); Q.room('show'); return { fns: Q, ...listsOf(K) }; } },
  { name: 'livedTrain', file: 'tools/levels/lived-train.ts', stand() { const K = officeKit({ groups: [] }), L = livedTrain({ G: gwotKit(gwotOfOffice(K)), K, add: K.add, grp: K.grp }); L.place('show'); return { fns: L, ...listsOf(K), seal: L.done }; } },
];

// ------------------------------------------------------------------ 1. every generator, watched ----
type Call = { level: string; args: readonly unknown[] };
const pieceNames = new Map<string, Set<string>>();                // kit → its pieces
const calls = new Map<string, Call[]>();                          // 'kit.piece' → the calls a generator made itself (not from inside another piece)
let building = ''; const open: string[] = [];                     // (building '' = nobody's: a call is told and not kept)
watchPieces({
  kit(kit, names) { const s = pieceNames.get(kit) ?? new Set<string>(); names.forEach(n => s.add(n)); pieceNames.set(kit, s); },
  open(kit, name, args) { if (!open.length && building) { const k = `${kit}.${name}`; (calls.get(k) ?? calls.set(k, []).get(k)!).push({ level: building, args }); } open.push(name); },
  close() { open.pop(); },
});
const levels: string[] = [], unbuilt: { level: string; why: string }[] = [];
for (const f of readdirSync(HERE).filter(f => f.endsWith('.ts')).sort()) {
  if (!/^export (?:function|const) build\b/m.test(readFileSync(resolve(HERE, f), 'utf8'))) continue;   // (a file with no build() is a kit or a tool: importing export.ts would run it)
  const gen = await import(resolve(HERE, f)), name = basename(f, '.ts');
  // A LOOK FILE (`<name>-gwot.ts`) builds with its plain level's generator, which is counted under the plain level's name; what the look file lays itself is
  // its `finish` (the skin's rugs, skirts and patches, its own dressing), run here on a level built unheard
  const look = typeof gen.finish === 'function';
  building = look ? '' : name; open.length = 0;
  try {
    const built = gen.build();
    if (look) { if (built.scenarios) built.level.scenarios = built.scenarios; building = name; gen.finish(built.level, built.groups); }   // (the file's own block first, as export.ts hands it over)
    levels.push(name);
  } catch (e) { unbuilt.push({ level: name, why: firstLine(e) }); }
}
building = 'office'; open.length = 0; buildOfficeProcedural({ groups: [] }); levels.push('office');   // (the office is built by its own function, not a file under tools/levels)
building = ''; for (const k of KITS) k.stand();                   // a kit no generator stood up still says its pieces
watchPieces(null);
levels.sort();
const onList = new Set(ALL_MISSIONS.filter(s => !s.offList).map(s => (s.level ?? 'office.json').replace(/\.json$/, '')));
/** a level file's name as its generator's (the two differ for one level) */
const fileOf = (level: string) => level.replace(/^blocksquads/, 'block-squads').replace(/-gwot$/, '');

// ------------------------------------------------------------------ paints: every colour that has a name, by its numbers ----
const key = (a: Vec3) => a.map(v => Number(v).toFixed(4)).join(',');
interface PaintRow { table: string; name: string; albedo: Vec3; rough?: number; metal?: boolean; sim?: string; surface?: string }
const PAINT_TABLES: { table: string; file: string; what: string; rows: PaintRow[] }[] = [
  { table: 'GWOT_PALETTE', file: 'src/scene/gwotPalette.ts', what: 'the look\'s own paints: the only table the look draws as materials', rows: (Object.keys(GWOT_PALETTE) as GwotMat[]).map(n => ({ table: 'GWOT_PALETTE', name: n, albedo: GWOT_PALETTE[n], rough: GWOT_FINISH[n]?.rough ?? ROUGH_CLOTH, metal: GWOT_FINISH[n]?.metal, sim: GWOT_SIM[n], surface: GWOT_SURFACE[n] })) },
  { table: 'SNOW_PALETTE', file: 'src/scene/gwotPalette.ts', what: 'lying snow as paint, for a level built in winter', rows: (Object.keys(SNOW_PALETTE) as SnowMat[]).map(n => ({ table: 'SNOW_PALETTE', name: n, albedo: SNOW_PALETTE[n], sim: SNOW_SIM[n] })) },
  { table: 'OFFICE_PALETTE', file: 'src/scene/officeGen.ts', what: 'the office\'s paints: the names the simulation reads a box\'s stuff by', rows: Object.entries(OFFICE_PALETTE).map(([n, a]) => ({ table: 'OFFICE_PALETTE', name: n, albedo: a })) },
  { table: 'KIT_PALETTE', file: 'src/scene/levelKit.ts', what: 'the map kit\'s copy of the office\'s paints', rows: Object.entries(KIT_PALETTE as Record<string, Vec3>).map(([n, a]) => ({ table: 'KIT_PALETTE', name: n, albedo: a })) },
  { table: 'LIVED_PAINT', file: 'tools/levels/livedKit.ts', what: 'colours of the older levels\' lived things that the office has no name for', rows: Object.entries(LIVED_PAINT).map(([n, a]) => ({ table: 'LIVED_PAINT', name: n, albedo: a })) },
  { table: 'INK', file: 'tools/levels/livedPieces.ts', what: 'accents of the older levels\' furniture', rows: Object.entries(INK).map(([n, a]) => ({ table: 'INK', name: n, albedo: a })) },
];
/** a colour two tables hold is the first table's row (the map kit's copies are the office's) */
const paintByKey = new Map<string, PaintRow>();
for (const t of PAINT_TABLES) for (const r of t.rows) if (!paintByKey.has(key(r.albedo))) paintByKey.set(key(r.albedo), r);
// a palette paint standing for another stuff (gwotPalette.ts gwotAs) is the same paint a hair off: it keeps its row
for (const m of Object.keys(GWOT_PALETTE) as GwotMat[]) for (const s of GWOT_SIM_NAMES) { const k = key(gwotAs(m, s)); if (!paintByKey.has(k)) paintByKey.set(k, paintByKey.get(key(GWOT_PALETTE[m]))!); }
/** a row's name as the page writes it: the bare word where one colour has it, `TABLE.word` where two tables give the word to different colours */
const wordColours = new Map<string, Set<string>>();
for (const t of PAINT_TABLES) for (const r of t.rows) (wordColours.get(r.name) ?? wordColours.set(r.name, new Set()).get(r.name)!).add(key(r.albedo));   // (over every table: INK.iron is the office's asphalt by its numbers, and still a second colour called iron)
const rowName = (r: PaintRow) => wordColours.get(r.name)!.size > 1 ? `${r.table}.${r.name}` : r.name;
/** a colour's row, with how far it is darkened from it (`mud × 0.60`: k under 1), or neither: a colour written as numbers */
function paintOf(a: Vec3): { row?: PaintRow; k?: number } {
  const row = paintByKey.get(key(a)); if (row) return { row };
  for (const r of new Set(paintByKey.values())) {
    if (r.albedo.some(v => v <= 0)) continue;
    const k = a[0] / r.albedo[0]; if (k < 0.3 || k >= 1 || a.some((v, i) => Math.abs(v - r.albedo[i] * k) > 6e-4)) continue;
    return { row: r, k };
  }
  return {};
}

// ------------------------------------------------------------------ 3. what a piece is for: the kit's own words ----
interface Said { doc: string; params: string; section: string; line: number }
function readKit(file: string, names: string[]): Map<string, Said> {
  const lines = readFileSync(resolve(ROOT, file), 'utf8').split('\n'), out = new Map<string, Said>();
  // a piece is defined as `const name = (…) =>`, `function name(…)`, or, inside the object a kit returns, `name(…) {` / `name: (…) =>`. (A line that only
  // declares it, an interface's `name(…): void;`, has no body and is passed over.)
  const named = (name: string) => new RegExp(`^\\s*(?:const\\s+${name}\\s*=\\s*|function\\s+${name}\\s*)(?:<[^(]*>)?\\(`);
  const member = (name: string) => new RegExp(`^\\s*${name}\\s*(?::\\s*)?\\(.*(?:=>|\\{)`);
  const at = new Map<string, number>();
  for (const n of names) { let i = lines.findIndex(l => named(n).test(l)); if (i < 0) i = lines.findIndex(l => member(n).test(l)); if (i >= 0) at.set(n, i); }
  const byLine = new Map([...at].map(([n, i]) => [i, n]));
  const docAt = (i: number): string => {
    const j = i - 1;
    if (j >= 0 && lines[j].trimEnd().endsWith('*/')) { let k = j; while (k > 0 && !lines[k].includes('/**')) k--; return lines.slice(k, j + 1).join(' ').replace(/\/\*\*|\*\//g, '').replace(/\s+\*\s+/g, ' ').replace(/\s+/g, ' ').trim(); }
    if (j >= 0 && byLine.has(j)) return docAt(j);                  // a twin written on the next line (wallZ under wallX) is said by its twin
    return '';
  };
  for (const [n, i] of at) {
    // its section: the kit's own heading over it (`// ---- stores & cover ----`)
    let section = ''; for (let j = i; j >= 0; j--) { const m = lines[j].match(/^\s*\/\/ -{3,}\s*(.*?)\s*-*\s*$/); if (m) { section = m[1].replace(/\s*[(:].*$/, '').trim(); break; } }
    // its arguments: from the definition's first bracket to the bracket that closes it, however many lines that is
    const text = lines.slice(i).join(' '), from = text.indexOf('('); let depth = 0, end = -1;
    for (let c = from; c < text.length && end < 0; c++) { if ('([{'.includes(text[c])) depth++; else if (')]}'.includes(text[c]) && --depth === 0) end = c; }
    out.set(n, { doc: docAt(i), params: end < 0 ? '' : paramsOf(text.slice(from + 1, end)), section, line: i + 1 });
  }
  return out;
}
/** `x: number, colour: GwotMat = 'drumBlue'` → `x, colour = 'drumBlue'` */
function paramsOf(src: string): string {
  const parts: string[] = []; let depth = 0, cur = '', quote = '';
  for (const ch of src) {
    if (quote) { cur += ch; if (ch === quote) quote = ''; continue; }
    if (ch === '\'' || ch === '"' || ch === '`') { quote = ch; cur += ch; continue; }
    if ('([{<'.includes(ch)) depth++; else if (')]}>'.includes(ch) && !cur.endsWith('=')) depth--;
    if (ch === ',' && depth === 0) { parts.push(cur); cur = ''; } else cur += ch;
  }
  if (cur.trim()) parts.push(cur);
  return parts.map(p => {
    const name = p.trim().match(/^[\w$]+/)?.[0] ?? '?'; let d = 0, eq = -1;
    for (let i = 0; i < p.length; i++) { const ch = p[i]; if ('([{'.includes(ch)) d++; else if (')]}'.includes(ch)) d--; else if (ch === '=' && d === 0 && p[i + 1] !== '>') { eq = i; break; } }
    return eq < 0 ? name : `${name} = ${p.slice(eq + 1).trim()}`;
  }).join(', ');
}

// ------------------------------------------------------------------ 2. each piece laid alone, and measured ----
const NS = BoxFlag.NoShadow, r2 = (v: number) => Math.round(v * 100) / 100;
export interface Piece {
  kit: string; name: string; id: string; section: string; doc: string; params: string; line: number;
  /** the level whose call was laid: a showroom, another level, or none (no level lays it) */
  sample: string | null; error?: string;
  /** the piece this one IS (`officeKit.chair`): it lays nothing of its own, only that one piece of another kit, taken into this kit */
  is?: string;
  /** WHAT IT IS TO A MAN, read off its boxes as laid: 'solid' (feet, sight, rounds and sound meet it, up to `reach` m) · 'raised' (solid, but nothing of it
   *  comes down to the floor: it was laid on furniture or on a wall) · 'glass' (sight passes, feet and sound do not) · 'walk-over' (solid, under 0.2 m) ·
   *  'loose' (a body he can shove) · 'drawn' (no ray and no foot meets it) · 'overhead' (solid, all of it over 1.5 m) */
  kind: string; reach: number; lit: boolean;
  /** length × width × its own height (top less foot), of the call that was laid */
  size: [number, number, number];
  count: { solid: number; drawn: number; loose: number; parts: number; marks: number; round: number; lights: number; fixtures: number };
  /** its paints by row (most boxes first), and the colours it wears that are in no table */
  paints: string[]; unnamed: string[]; inBand: boolean;
  /** level → how many the generator lays */
  uses: Record<string, number>;
  rows: PaintRow[]; boxes: DrawBox[];
}
const partBoxes = (p: PropDef): DrawBox[] => { const cs = Math.cos(p.yaw), sn = Math.sin(p.yaw); return p.parts.map(q => ({ c: [p.x + cs * q.off[0] + sn * q.off[2], q.off[1], p.z - sn * q.off[0] + cs * q.off[2]] as Vec3, h: q.h, yaw: p.yaw + q.yaw, albedo: q.albedo, flags: 0, marks: q.marks })); };
/** the smallest rectangle on the ground that holds every box, tried along each box's own heading */
function footprint(boxes: DrawBox[]): [number, number] {
  let best: [number, number] = [0, 0], area = Infinity;
  for (const yaw of new Set(boxes.map(b => r2(((b.yaw % (Math.PI / 2)) + Math.PI / 2) % (Math.PI / 2))))) {
    const cs = Math.cos(yaw), sn = Math.sin(yaw); let u0 = Infinity, u1 = -Infinity, v0 = Infinity, v1 = -Infinity;
    for (const b of boxes) { const c = Math.cos(b.yaw), s = Math.sin(b.yaw); for (const sx of [-1, 1]) for (const sz of [-1, 1]) {
      const x = b.c[0] + c * sx * b.h[0] + s * sz * b.h[2], z = b.c[2] - s * sx * b.h[0] + c * sz * b.h[2], u = x * cs - z * sn, v = x * sn + z * cs;
      u0 = Math.min(u0, u); u1 = Math.max(u1, u); v0 = Math.min(v0, v); v1 = Math.max(v1, v); } }
    if ((u1 - u0) * (v1 - v0) < area) { area = (u1 - u0) * (v1 - v0); best = [Math.max(u1 - u0, v1 - v0), Math.min(u1 - u0, v1 - v0)]; }
  }
  return best;
}
function measure(kitName: string, name: string, said: Said | undefined, stand: () => Stood): Piece {
  const id = `${kitName}.${name}`, mine = calls.get(id) ?? [], uses: Record<string, number> = {};
  for (const c of mine) uses[c.level] = (uses[c.level] ?? 0) + 1;
  const sample = mine.find(c => SHOWROOMS.includes(c.level)) ?? mine[0];
  const piece: Piece = { kit: kitName, name, id, section: said?.section ?? '', doc: said?.doc ?? '', params: said?.params ?? '', line: said?.line ?? 0, sample: sample?.level ?? null,
    kind: '', reach: 0, lit: false, size: [0, 0, 0], count: { solid: 0, drawn: 0, loose: 0, parts: 0, marks: 0, round: 0, lights: 0, fixtures: 0 }, paints: [], unnamed: [], inBand: false, uses, rows: [], boxes: [] };
  if (!sample) return piece;
  // (stood up and laid with the door watched: `inside` is each piece laid straight from inside this one, with the lists' lengths as it opened and closed)
  let S!: Stood, depth = 0; const inside: { id: string; from: number[]; to: number[] }[] = [];
  const lengths = () => [S.boxes.length, S.props.length, S.lights.length, S.switchables.length], same = (a: number[], b: number[]) => a.every((v, i) => v === b[i]);
  watchPieces({ kit() {}, open(kit, laid) { if (depth++ === 1) inside.push({ id: `${kit}.${laid}`, from: lengths(), to: [] }); }, close() { if (--depth === 1) inside[inside.length - 1].to = lengths(); } });
  let before: number[], after: number[];
  try { S = stand(); before = lengths(); (S.fns[name] as (...a: unknown[]) => unknown)(...sample.args); after = lengths(); S.seal?.(); } catch (e) { piece.error = firstLine(e); return piece; } finally { watchPieces(null); }
  const n = { b: before[0], p: before[1], l: before[2], s: before[3] };
  if (inside.length === 1 && inside[0].id !== id && same(inside[0].from, before) && same(inside[0].to, after)) piece.is = inside[0].id;
  const statics = S.boxes.slice(n.b), props = S.props.slice(n.p), lights = S.lights.slice(n.l), fixtures = S.switchables.slice(n.s);
  const parts = props.flatMap(partBoxes), all: DrawBox[] = [...statics.map(b => ({ c: b.c, h: b.h, yaw: b.yaw, albedo: b.albedo, flags: b.flags, emissive: b.emissive, marks: b.marks })), ...parts];
  if (!all.length) return piece;
  const top = (b: DrawBox) => b.c[1] + b.h[1], bottom = (b: DrawBox) => b.c[1] - b.h[1];
  // what stops a body: a solid from 0.2 m up whose underside is under 1.5 m (the game's own two numbers: scene/collision.ts)
  const solid = statics.filter(b => !(b.flags & NS)), inWay = solid.filter(b => top(b) >= 0.2 - 1e-9 && bottom(b) < 1.5);
  const foot = Math.min(...all.map(bottom)), [len, wid] = footprint(all);
  piece.boxes = all;
  piece.count = { solid: solid.length, drawn: statics.length - solid.length, loose: props.length, parts: parts.length, marks: all.reduce((k, b) => k + (b.marks?.length ?? 0), 0),
    round: findOctagons(statics).length, lights: lights.length, fixtures: fixtures.length };
  piece.size = [r2(len), r2(wid), r2(Math.max(...all.map(top)) - foot)];
  piece.lit = lights.length + fixtures.length > 0;
  piece.reach = inWay.length ? r2(Math.max(...inWay.map(top))) : 0;
  piece.kind = statics.some(b => b.flags & BoxFlag.Glass) ? 'glass' : inWay.length ? (foot >= 0.25 ? 'raised' : 'solid') : props.length ? 'loose' : solid.some(b => top(b) < 0.2) ? 'walk-over' : solid.length ? 'overhead' : 'drawn';
  // THE BAND: a top in 0.20 to 0.25 m counts where it is the piece's top there. A lower course of sandbags is under the next one, and a sack's ear a hand's
  // width out from under the sack above it stands on a cell the pile shuts anyway: lived-check's five probes, asked of the piece's own in-the-way boxes in
  // place of the baked floor
  const NEAR = 0.1;
  const under = (b: Box, x: number, z: number) => { const cs = Math.cos(b.yaw), sn = Math.sin(b.yaw), dx = x - b.c[0], dz = z - b.c[2]; return Math.abs(cs * dx - sn * dz) <= b.h[0] + NEAR && Math.abs(sn * dx + cs * dz) <= b.h[2] + NEAR; };
  const bare = (b: Box) => { const cs = Math.cos(b.yaw), sn = Math.sin(b.yaw); return [[0, 0], [-1, -1], [1, -1], [-1, 1], [1, 1]].some(([u, v]) => {
    const x = b.c[0] + cs * u * b.h[0] + sn * v * b.h[2], z = b.c[2] - sn * u * b.h[0] + cs * v * b.h[2]; return !inWay.some(o => o !== b && top(o) >= 0.25 - 1e-9 && under(o, x, z)); }); };
  piece.inBand = inWay.some(b => top(b) < 0.25 - 1e-9 && bare(b));
  const worn = new Map<string, { row: PaintRow; n: number; k?: number }>(), unnamed = new Set<string>();
  for (const b of all) { const p = paintOf(b.albedo);
    if (!p.row) { unnamed.add(`(${b.albedo.map(v => +Number(v).toFixed(3)).join(', ')})`); continue; }
    const word = p.k === undefined ? rowName(p.row) : `${rowName(p.row)} × ${p.k.toFixed(2)}`, w = worn.get(word) ?? { row: p.row, n: 0, k: p.k }; w.n++; worn.set(word, w); }
  piece.paints = [...worn].sort((a, b) => b[1].n - a[1].n || a[0].localeCompare(b[0])).map(([w]) => w); piece.unnamed = [...unnamed].sort();
  piece.rows = [...new Set([...worn.values()].map(w => w.row))];
  return piece;
}
const catalogue: Piece[] = [];
for (const k of KITS) {
  const names = [...(pieceNames.get(k.name) ?? [])], said = readKit(k.file, names);
  for (const n of names) catalogue.push(measure(k.name, n, said.get(n), k.stand));
}

// ------------------------------------------------------------------ the library's rules ----
const problems: string[] = [], plural = (n: number, one: string, many: string) => `${n} ${n === 1 ? one : many}`;
const notShown = catalogue.filter(p => !SHOWROOMS.some(s => s in p.uses)), unsaid = catalogue.filter(p => !p.doc), banded = catalogue.filter(p => p.inBand), broken = catalogue.filter(p => p.error);
const noRow = [...pieceNames.keys()].filter(k => !KITS.some(r => r.name === k)), noPieces = KITS.filter(r => !pieceNames.get(r.name)?.size).map(r => r.name);
if (notShown.length) problems.push(`${plural(notShown.length, 'piece does', 'pieces do')} not stand in a showroom (tools/levels/${SHOWROOMS.join('.ts, ')}.ts): ${notShown.map(p => p.id).join(', ')}`);
if (unsaid.length) problems.push(`${plural(unsaid.length, 'piece does', 'pieces do')} not say what it is (a /** comment */ over its definition in the kit's file): ${unsaid.map(p => p.id).join(', ')}`);
if (banded.length) problems.push(`${plural(banded.length, 'piece tops', 'pieces top')} out between 0.20 and 0.25 m, where a body is stopped on floor the nav grid calls free: ${banded.map(p => p.id).join(', ')}`);
if (broken.length) problems.push(`${plural(broken.length, 'piece', 'pieces')} could not be laid alone: ${broken.map(p => `${p.id} (${p.error})`).join(', ')}`);
if (noRow.length) problems.push(`${plural(noRow.length, 'kit hands', 'kits hand')} pieces out and has no row in tools/levels/library.ts KITS: ${noRow.join(', ')}`);
if (noPieces.length) problems.push(`${plural(noPieces.length, 'row', 'rows')} of KITS named no kit that hands pieces out (a kit renamed, or a name mistyped in its pieces() call): ${noPieces.join(', ')}`);
if (unbuilt.length) problems.push(`${plural(unbuilt.length, 'generator', 'generators')} did not build: ${unbuilt.map(u => `${u.level} (${u.why})`).join(', ')}`);

// ------------------------------------------------------------------ the pages ----
const KIND_WORD: Record<string, string> = { solid: 'solid', raised: 'off the floor', glass: 'glass', 'walk-over': 'walked over', loose: 'loose', drawn: 'drawn only', overhead: 'overhead', '': 'not laid' };
const toAMan = (p: Piece) => `${KIND_WORD[p.kind]}${p.kind === 'solid' ? ` to ${p.reach}` : ''}${p.lit ? ', lit' : ''}`;
const usesOf = (p: Piece) => Object.keys(p.uses).filter(l => !SHOWROOMS.includes(l)).sort();
const cell = (s: string) => s.replace(/\|/g, '\\|');
const costOf = (p: Piece) => [p.count.solid && `${p.count.solid} solid`, p.count.drawn && `${p.count.drawn} drawn`, p.count.loose && `${p.count.loose} loose (${plural(p.count.parts, 'part', 'parts')})`, p.count.round && `${p.count.round} round`,
  p.count.marks && plural(p.count.marks, 'marking', 'markings'), p.count.lights && plural(p.count.lights, 'lamp', 'lamps'), p.count.fixtures && plural(p.count.fixtures, 'fixture', 'fixtures')].filter(Boolean).join(', ');
const sectionsOf = (kit: string) => [...new Set(catalogue.filter(p => p.kit === kit).map(p => p.section))];
const nPaints = new Set(paintByKey.values()).size;

/** docs/library.md: what the kits ARE. Nothing in it comes from a level but the two showrooms, so it is stale only when a kit or a showroom changed */
function page(): string {
  const L: string[] = [];
  L.push('# The library: every object and every paint, once', '',
    'WRITTEN BY `bun run tools/levels/library.ts --write` FROM THE KITS. Do not edit: change the kit, then write it again (`--check` is red when this page is stale).', '',
    `${catalogue.length} pieces in ${KITS.length} kits, ${nPaints} named paints in ${PAINT_TABLES.length} tables.`,
    'A piece is something a level asks a kit for by name. Each row is one call laid alone: the call a showroom makes.', '',
    '- **To a man**, read off the boxes as laid: *solid to h* = feet, sight, rounds and sound meet it, up to h metres; *off the floor* = solid, but nothing of it comes',
    '  down to the floor (it is laid on furniture or on a wall); *glass* = sight passes it, feet and sound do not; *walked over* = solid but under 0.2 m;',
    '  *loose* = a body a man shoves and a blast throws; *drawn only* = no ray and no foot meets it; *lit* = it adds a lamp or a lit fixture.',
    '- **Size** is length × width × height in metres of the call that was laid. A wall, a rug or a line is as long as it was asked for.',
    '- **Boxes** is what the piece costs: solid boxes, drawn-only boxes, loose things and their parts, round things the round rule finds, marking bitmaps, lamps.',
    '- **Paints** are rows of the tables under Paints below, most boxes first; `row × 0.60` is a row darkened; a colour a kit writes as numbers shows as its numbers.',
    `- **The showrooms**: ${SHOWROOMS.map(s => `\`?levelFile=${s}.json&solo&hour=midday\``).join(' and ')}. Between them one of each piece stands, and its line in the`,
    '  showroom\'s generator is the call to copy.',
    '- Which level lays which piece is not on this page (it moves with every generator): `bun run tools/levels/library.ts --uses FILE` writes it.', '');
  L.push('## The kits', '', '| Kit | File | Pieces | Its sections |', '|---|---|---|---|');
  for (const k of KITS) L.push(`| ${k.name} | \`${k.file}\` | ${catalogue.filter(p => p.kit === k.name).length} | ${sectionsOf(k.name).filter(Boolean).join(' · ')} |`);
  L.push('');
  // the same word in more than one kit: the library's own to-do list
  const byName = new Map<string, Piece[]>(); for (const p of catalogue) (byName.get(p.name) ?? byName.set(p.name, []).get(p.name)!).push(p);
  // (a kit's piece that IS another kit's piece is no second way of building it: a name counts once for each piece that is its own)
  const ways = (ps: Piece[]) => [...new Map(ps.map(p => [p.is ?? p.id, p])).values()];
  const twice = [...byName].map(([n, ps]) => [n, ways(ps)] as const).filter(([, ps]) => ps.length > 1).sort((a, b) => a[0].localeCompare(b[0]));
  L.push('## One name, more than one piece', '', 'The same word in more than one kit: each is built its own way, in its own paint. This is the list the library has to bring down to one piece a name.', '',
    '| Name | Kits | Sizes |', '|---|---|---|');
  for (const [n, ps] of twice) L.push(`| ${n} | ${ps.map(p => p.is ? `${p.kit} (${p.is})` : p.kit).join(', ')} | ${ps.map(p => p.sample ? p.size.join(' × ') : 'not laid').join(' · ')} |`);
  L.push('');
  const taken = catalogue.filter(p => p.is);
  L.push('## One piece, handed out by more than one kit', '', 'A piece that lays nothing of its own, only one piece of another kit: the same piece, taken into that kit (a lived kit names it as one of a room\'s lived things).',
    'It is no second way of building the name.', '', '| Piece | Is |', '|---|---|');
  for (const p of taken) L.push(`| ${p.id} | ${p.is} |`);
  L.push('');
  for (const k of KITS) {
    L.push(`## ${k.name}`, '', `\`${k.file}\``, '');
    for (const s of sectionsOf(k.name)) {
      if (s) L.push(`### ${k.name}: ${s}`, '');
      L.push('| Piece | What it is | To a man | Size | Boxes | Paints | Stands in |', '|---|---|---|---|---|---|---|');
      for (const p of catalogue.filter(q => q.kit === k.name && q.section === s))
        L.push(`| **${p.name}**(${cell(p.params)}) | ${cell(p.doc) || '(not said)'}${p.is ? ` *(is \`${p.is}\`)*` : ''} | ${toAMan(p)} | ${p.sample ? p.size.join(' × ') : ''} | ${p.error ? `could not be laid alone: ${cell(p.error)}` : costOf(p)} | ${cell([...p.paints, ...p.unnamed].join(', '))} | ${SHOWROOMS.filter(r => r in p.uses).join(', ') || 'no showroom'} |`);
      L.push('');
    }
  }
  L.push('## Paints', '', 'Every colour with a name, by table. *Stands for* is what the simulation reads the paint as (a round, a charge and a blast go by it); *underfoot* is its footstep surface.',
    'Pieces counts the catalogue\'s pieces that wear the row, darkened or not, as laid above.', '');
  const wears = new Map<PaintRow, number>(); for (const p of catalogue) for (const r of p.rows) wears.set(r, (wears.get(r) ?? 0) + 1);
  for (const t of PAINT_TABLES) {
    L.push(`### ${t.table}`, '', `\`${t.file}\`: ${t.what}.`, '', '| Paint | Colour (linear) | Finish | Stands for | Underfoot | Pieces |', '|---|---|---|---|---|---|');
    for (const r of t.rows) { const first = paintByKey.get(key(r.albedo))!;
      L.push(`| ${r.name} | ${r.albedo.join(', ')} | ${r.rough === undefined ? '' : `rough ${r.rough}${r.metal ? ', bare metal' : ''}`} | ${r.sim ?? ''} | ${r.surface ?? ''} | ${first === r ? wears.get(r) ?? 0 : `the same colour as ${first.table}.${first.name}`} |`); }
    L.push('');
  }
  const clash = [...wordColours].filter(([, cs]) => cs.size > 1).map(([w]) => w).sort();
  L.push('### One paint name, more than one colour', '', '| Name | Tables and colours |', '|---|---|');
  for (const w of clash) L.push(`| ${w} | ${PAINT_TABLES.flatMap(t => t.rows).filter(r => r.name === w).map(r => `${r.table} (${r.albedo.join(', ')})`).join(' · ')} |`);
  const loose = new Map<string, string[]>(); for (const p of catalogue) for (const u of p.unnamed) (loose.get(u) ?? loose.set(u, []).get(u)!).push(p.id);
  L.push('', '### Colours written as numbers', '', 'A colour a kit writes by hand, in no table. The look can give such a box no material.', '', '| Colour (linear) | Pieces |', '|---|---|');
  for (const [c, ids] of [...loose].sort((a, b) => b[1].length - a[1].length || a[0].localeCompare(b[0]))) L.push(`| ${c} | ${ids.join(', ')} |`);
  L.push('', '## What the rules say today', '', ...(problems.length ? problems.map(p => `- ${p}`) : ['- Nothing: every piece stands in a showroom, says what it is and tops out clear of the band the nav grid cannot see.']), '');
  return L.join('\n');
}
/** which level lays which piece: true of the generators as they are this minute */
function usesPage(): string {
  const L: string[] = [], played = levels.filter(l => !SHOWROOMS.includes(l));
  const kitsOf = (level: string) => KITS.filter(k => catalogue.some(p => p.kit === k.name && level in p.uses)).map(k => k.name);
  L.push('# The library: which level lays which piece', '', 'Written by `bun run tools/levels/library.ts --uses FILE`. A piece laid from inside another piece is counted as the outer one;',
    'a look file (`<name>-gwot`) is counted for what its own finish step lays on top of its plain level. The four levels built on `src/scene/levelKit.ts`',
    '(defuse, floodlit, mousehole, records) lay the office\'s pieces through plain functions no kit hands out, so those calls are not counted.', '',
    '## The generators', '', '| Level | On the mission list | Kits it lays from | Pieces laid |', '|---|---|---|---|');
  for (const l of played) L.push(`| ${l} | ${onList.has(fileOf(l)) ? 'yes' : ''} | ${kitsOf(l).join(', ')} | ${catalogue.reduce((n, p) => n + (p.uses[l] ?? 0), 0)} |`);
  L.push('', '## The pieces', '', '| Piece | Levels | Laid |', '|---|---|---|');
  for (const p of catalogue) { const ls = usesOf(p); L.push(`| ${p.id} | ${ls.length ? `${ls.length}: ${ls.map(l => `${l} ${p.uses[l]}`).join(', ')}` : 'a showroom only'} | ${ls.reduce((n, l) => n + p.uses[l], 0)} |`); }
  L.push('');
  return L.join('\n');
}

// ------------------------------------------------------------------ out ----
const text = page(), OUT = resolve(ROOT, PAGE);
const pictures = valueOf('--pictures'), json = valueOf('--json'), uses = valueOf('--uses');
if (pictures) { mkdirSync(pictures, { recursive: true }); for (const p of catalogue) if (p.boxes.length) writeFileSync(resolve(pictures, `${p.id}.png`), encodePng(drawBoxes(p.boxes), 240, 180)); }
if (json) writeFileSync(json, JSON.stringify({ showrooms: SHOWROOMS, levels: levels.map(l => ({ level: l, onList: onList.has(fileOf(l)) })), kits: KITS.map(k => ({ name: k.name, file: k.file })),
  pieces: catalogue.map(({ boxes: _boxes, rows: _rows, ...p }) => ({ ...p, toAMan: toAMan(p as Piece), cost: costOf(p as Piece) })), paints: PAINT_TABLES, problems }, null, 1));
if (uses) writeFileSync(uses, usesPage());
if (flag('--write')) { writeFileSync(OUT, text); console.log(`wrote ${PAGE}`); }
const stale = !existsSync(OUT) || readFileSync(OUT, 'utf8') !== text;
console.log(`library: ${catalogue.length} pieces in ${KITS.length} kits (${KITS.map(k => `${k.name} ${catalogue.filter(p => p.kit === k.name).length}`).join(', ')}), ${nPaints} named paints, ${levels.length} generators`);
for (const p of problems) console.log(`  RULE: ${p}`);
if (stale && !flag('--write')) console.log(`  ${PAGE} is ${existsSync(OUT) ? 'not what the kits give now' : 'missing'}: bun run tools/levels/library.ts --write`);
if (flag('--check')) { const red = problems.length > 0 || stale; console.log(red ? 'FAIL library' : 'PASS library'); process.exit(red ? 1 : 0); }
