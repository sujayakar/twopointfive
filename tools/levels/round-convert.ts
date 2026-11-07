// A LEVEL FILE OF VERSION 1 BECOMES VERSION 2: a round thing the file wrote as four slabs through one centre, 45 degrees apart, is written as the one box it is
// (BoxData.round, docs/level-format.md). The game and its lint recognise nothing: this is the one place that knows the old shape, and it runs once on a file.
//   bun run tools/levels/round-convert.ts <file.json> [more files]     rewrites each file in place; a file already at version 2 is left alone
// What each round becomes is what the game used to make of it as a level was stood up, to the bit: the first slab's middle and height, radius 1.026 x the slabs'
// half length (the circle with the octagon's area), no turn, the lid as a disc and the band on the four sides. Unnamed boxes are named by their place in the file,
// so every unnamed box after a round takes a new name, as the exporter would give it, and whatever names it follows. A list that names all four slabs of a round
// names the one box; anything else that names a slab the file loses stops the conversion, and nothing is written.
import { readFileSync, writeFileSync } from 'node:fs';
import { resolve } from 'node:path';
const ROOT = resolve(import.meta.dir, '../..');
const { MARKING_TEXEL, MAX_MARKS_PER_BOX, texelOf } = await import(`${ROOT}/src/scene/markings.ts`);
const { LEVEL_DATA_VERSION, serializeLevelData, validateLevelData } = await import(`${ROOT}/src/scene/levelData.ts`);

type Mark = { face: string; res?: 'fine' | 'base' | 'coarse'; at: [number, number]; size: [number, number]; pal: unknown[]; cells: string[] };
type Slab = { id: string; name?: string; c: number[]; h: number[]; yaw?: number; rot?: unknown; flags?: string[]; shape?: unknown; material?: string; albedo?: number[]; markings?: Mark[]; [k: string]: unknown };
type Doc = { version: number; name: string; meta?: { markTexel?: number }; boxes: Slab[]; materials?: Record<string, { albedo: number[] }>; [k: string]: unknown };

/** the round's radius over the slabs' half length: the circle with the octagon's own area, the square root of (8 tan 22.5 degrees / pi) */
const R_OF_SLAB = 1.026;
const TAN8 = Math.tan(Math.PI / 8);
const near = (a: number, b: number, e = 1e-4) => Math.abs(a - b) < e;
const yawOf = (b: Slab) => b.yaw ?? 0;
const flagged = (b: Slab) => !!b.flags?.length;

/** four plain slabs through one centre, 45 degrees apart, tan 22.5 degrees as deep as they are long, in one paint, on one floor, their tops stepped down by one small
 *  step (2 mm from the kit, 3 mm from two older generators) */
function octagonAt(B: readonly Slab[], i: number, paint: (b: Slab) => readonly number[]): boolean {
  const a = B[i];
  if (i + 3 >= B.length || a.rot || flagged(a) || !near(a.h[2], a.h[0] * TAN8, a.h[0] * 0.01)) return false;
  const top = a.c[1] + a.h[1], step = top - (B[i + 1].c[1] + B[i + 1].h[1]); if (!(step > 0.0005 && step < 0.0045)) return false;
  const pa = paint(a);
  for (let k = 1; k < 4; k++) {
    const b = B[i + k];
    if (b.rot || flagged(b) || !near(b.c[0], a.c[0]) || !near(b.c[2], a.c[2]) || !near(b.h[0], a.h[0]) || !near(b.h[2], a.h[2])) return false;
    if (!near(yawOf(b) - yawOf(a), k * Math.PI / 4, 1e-3) || !near(b.c[1] - b.h[1], a.c[1] - a.h[1], 1e-3) || !near(b.c[1] + b.h[1], top - step * k, 2e-4)) return false;
    const pb = paint(b); if (pb[0] !== pa[0] || pb[1] !== pa[1] || pb[2] !== pa[2]) return false;
  }
  return true;
}
/** the octagons of a file's boxes, as the index of each one's first slab */
export function findOctagons(d: Doc): number[] {
  const paint = (b: Slab): readonly number[] => b.albedo ?? d.materials?.[b.material!]?.albedo ?? [NaN, NaN, NaN];   // (NaN: a box with no paint is like no other)
  const out: number[] = [];
  for (let i = 0; i + 3 < d.boxes.length; i++) if (octagonAt(d.boxes, i, paint)) { out.push(i); i += 3; }
  return out;
}
/** a round patch on a lid (a tyre's hole, an oven's mouth): the slab's own +y mark was cut to the slab, so it is drawn again as a disc of radius R */
function discMark(like: Mark, R: number, texel: number): Mark {
  const t = texelOf(like, texel), N = Math.min(32, 2 * Math.ceil(R / t) + 2), cells: string[] = [];
  for (let j = N - 1; j >= 0; j--) { let row = ''; for (let i = 0; i < N; i++) row += Math.hypot(i + 0.5 - N / 2, j + 0.5 - N / 2) * t < R ? 'a' : '.'; cells.push(row); }
  return { face: '+y', ...(like.res && like.res !== 'base' ? { res: like.res } : {}), at: [-N / 2, -N / 2], size: [N, N], pal: [like.pal[0]], cells };
}
const SIDES = ['+x', '-x', '+z', '-z'];
/** the one box a round's four slabs are */
function roundOf(slabs: Slab[], texel: number): Slab {
  const a = slabs[0], R = a.h[0] * R_OF_SLAB;
  const band = slabs.flatMap(b => b.markings ?? []).find(m => m.face === '+z' || m.face === '-z');   // a slab's long face carries the whole band (a drum's ribs); its short ends carry a cut of it
  const lid = (a.markings ?? []).find(m => m.face === '+y');
  const marks: Mark[] = [];
  if (lid && lid.pal.length === 1) marks.push(discMark(lid, R * 0.5, texel));   // (a lid of two paints is a drum's: its rim is the shader's, its bung is let go)
  if (band) for (const f of SIDES) if (marks.length < MAX_MARKS_PER_BOX) marks.push({ ...band, face: f });
  const { yaw: _yaw, markings: _marks, ...rest } = a;
  return { ...rest, h: [R, a.h[1], R], round: true, ...(marks.length ? { markings: marks } : {}) };
}
const POSITIONAL = /^box_\d+$/;
const idAt = (i: number) => `box_${String(i).padStart(4, '0')}`;

/** a version 1 document as version 2. Throws, naming the box and the place, where a reference cannot be carried */
export function convertLevel(d: Doc): { data: Doc; rounds: number } {
  if (d.version !== 1) throw new Error(`${d.name}: version ${d.version} is not the version this converts`);
  const texel = d.meta?.markTexel ?? MARKING_TEXEL, firsts = findOctagons(d);
  const speckOf = new Map<string, string>(), fourOf = new Map<string, string[]>();   // (a slab that leaves the file → its round's first slab; the first slab → all four ids)
  const out: Slab[] = [];
  for (let i = 0, f = 0; i < d.boxes.length; i++) {
    if (firsts[f] !== i) { out.push(d.boxes[i]); continue; }
    const slabs = d.boxes.slice(i, i + 4);
    for (const s of slabs.slice(1)) { speckOf.set(s.id, slabs[0].id); if (s.name !== slabs[0].name) throw new Error(`${d.name}: ${s.id} says a name its round's first slab does not`); }
    fourOf.set(slabs[0].id, slabs.map(s => s.id));
    out.push(roundOf(slabs, texel)); i += 3; f++;
  }
  const rename = new Map<string, string>();
  out.forEach((b, i) => { if (POSITIONAL.test(b.id)) { const id = idAt(i); if (id !== b.id) rename.set(b.id, id); b.id = id; } });
  const named = (id: string) => rename.get(id) ?? id;
  /** carry every box id in a value: in a list that names all four slabs of a round, they become the one id at the first slab's place */
  const carry = (v: unknown, where: string): unknown => {
    if (typeof v === 'string') { if (speckOf.has(v)) throw new Error(`${d.name}: ${where} names ${v}, a slab of the round at ${speckOf.get(v)}: say the one box`); return named(v); }
    if (Array.isArray(v)) {
      const gone = new Set<string>();
      for (const x of v) if (typeof x === 'string' && fourOf.has(x) && fourOf.get(x)!.every(id => v.includes(id))) for (const id of fourOf.get(x)!.slice(1)) gone.add(id);
      return v.filter(x => !(typeof x === 'string' && gone.has(x))).map((x, i) => carry(x, `${where}[${i}]`));
    }
    if (v && typeof v === 'object') return Object.fromEntries(Object.entries(v).map(([k, x]) => [k, carry(x, `${where}.${k}`)]));
    return v;
  };
  const data: Doc = { ...d };
  for (const k of Object.keys(d)) if (k !== 'boxes' && k !== 'materials' && k !== 'meta' && k !== 'version' && k !== 'name') data[k] = carry(d[k], k);
  data.boxes = out; data.version = LEVEL_DATA_VERSION;
  return { data, rounds: firsts.length };
}

if (import.meta.main) {
  const files = process.argv.slice(2);
  if (!files.length) { console.error('usage: bun run tools/levels/round-convert.ts <file.json> [more files]'); process.exit(2); }
  let bad = 0;
  for (const file of files) {
    const d = JSON.parse(readFileSync(file, 'utf8')) as Doc;
    if (d.version === LEVEL_DATA_VERSION) { console.log(`${file}: already version ${LEVEL_DATA_VERSION}`); continue; }
    try {
      const { data, rounds } = convertLevel(d), errs = validateLevelData(data);
      if (errs.length) throw new Error(`${d.name}: the converted file does not validate: ${errs.slice(0, 8).join(' · ')}`);
      writeFileSync(file, serializeLevelData(data));
      console.log(`${file}: ${rounds} round thing${rounds === 1 ? '' : 's'}, ${data.boxes.length} boxes`);
    } catch (e) { console.error(`${file}: ${(e as Error).message}`); bad++; }
  }
  if (bad) process.exit(1);
}
