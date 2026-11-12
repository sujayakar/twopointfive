// THE PAGE'S OFFSETS INTO WHAT ONLY A SHADER DECLARES: the WGSL struct is the declaration (a uniform block or a storage record that the page alone writes), and src/gpu/wgslrows.ts, its
// word offsets and its flag bits by name, is written from it. A shader's layout is said once; a writer of it says names, never numbers.
//   bun run tools/gpu/wgsl-rows.ts [--check]      write the file · --check: exit 1 where it would change
// WHAT THIS READS OF WGSL: `struct` declarations (a nested one too), scalars, vec2 to 4 of f32/u32/i32, mat4x4f, arrays with a literal size or one a `const` of the same file gives. The layout
// rules are WGSL's for the address space named below: an array's element in a uniform block is rounded up to 16 bytes, in a storage buffer to its own alignment.
import { readFileSync, writeFileSync, existsSync } from 'node:fs';
import { join, resolve } from 'node:path';

const ROOT = resolve(import.meta.dir, '../..'), OUT = join(ROOT, 'src/gpu/wgslrows.ts');
type Space = 'uniform' | 'storage';
/** the structs the page writes, and the name each goes by there (`also`: files a struct it holds is in) */
const STRUCTS: { file: string; struct: string; as: string; space: Space; also?: string[] }[] = [
  { file: 'src/render/shaders/common.wgsl', struct: 'Frame', as: 'FrameAt', space: 'uniform' },
  { file: 'src/smoke/smoke.wgsl', struct: 'SP', as: 'SpAt', space: 'uniform' },
  { file: 'src/smoke/wind_force.wgsl', struct: 'WindBlock', as: 'WindAt', space: 'uniform' },
  { file: 'src/render/shaders/air_field.wgsl', struct: 'AirU', as: 'AirAt', space: 'uniform' },
  { file: 'src/render/shaders/place_field.wgsl', struct: 'PlaceU', as: 'PlaceAt', space: 'uniform' },
  { file: 'src/render/shaders/powder_field.wgsl', struct: 'PowU', as: 'PowAt', space: 'uniform' },
  { file: 'src/render/shaders/outline.wgsl', struct: 'OutlineU', as: 'OutlineAt', space: 'uniform' },
  { file: 'src/render/shaders/composite.wgsl', struct: 'Beams', as: 'BeamsAt', space: 'uniform', also: ['src/render/shaders/fire.wgsl'] },
  { file: 'src/render/shaders/taau.wgsl', struct: 'TaauParams', as: 'TaauAt', space: 'uniform' },
  { file: 'src/render/shaders/particles_common.wgsl', struct: 'EmitBlock', as: 'EmitBlockAt', space: 'uniform' },
];
const FLAGS: { file: string; prefix: string; as: string; doc: string }[] = [
  { file: 'src/render/shaders/common.wgsl', prefix: 'FLAG_', as: 'FrameFlags', doc: 'what a frame\'s flags word raises (common.wgsl FLAG_*)' },
];
/** bits that stand in one block of consts with no prefix of their own: the line before the block names it */
const BLOCKS: { file: string; marker: string; as: string; doc: string }[] = [
  { file: 'src/render/shaders/common.wgsl', marker: 'LEVEL SWITCHES', as: 'LevelSw', doc: 'what a level raises, as bits of frame.levelCfg.x (common.wgsl lv)' },
];

const strip = (s: string): string => s.replace(/\/\/[^\n]*/g, '').replace(/\/\*[\s\S]*?\*\//g, '');
const die = (s: string): never => { console.error(`wgsl-rows: ${s}`); process.exit(1); };
const up = (n: number, a: number): number => Math.ceil(n / a) * a;
/** the commas of the top level (an array's type has its own) */
const split = (src: string): string[] => { const out: string[] = []; let d = 0, cur = ''; for (const c of src) { if (c === '<' || c === '(') d++; if (c === '>' || c === ')') d--; if (c === ',' && d === 0) { out.push(cur); cur = ''; } else cur += c; } out.push(cur); return out.map(s => s.trim()).filter(Boolean); };

export interface Member { name: string; type: string; word: number; words: number; stride: number }   // (stride: words from one element to the next of an array, 0 where it is none)
export interface Layout { name: string; members: Member[]; words: number; align: number }
export function layoutOf(text: string, name: string, space: Space, seen = new Map<string, Layout>()): Layout {
  const had = seen.get(name); if (had) return had;
  const src = strip(text), m = new RegExp(`struct ${name}\\s*\\{([^}]*)\\}`).exec(src) ?? die(`no struct ${name}`);
  const constOf = (e: string): number => { const t = e.trim().replace(/u$/, ''); if (/^\d+$/.test(t)) return +t; const c = new RegExp(`const ${t}\\s*:\\s*u32\\s*=\\s*(\\d+)u`).exec(src); return c ? +c[1] : die(`${name}: the size '${e.trim()}' is no literal of this file`); };
  const sizeOf = (t: string): { align: number; size: number; stride: number } => {
    t = t.trim();
    if (/^(f32|u32|i32)$/.test(t)) return { align: 4, size: 4, stride: 0 };
    let v = /^vec([234])[fui]$/.exec(t) ?? /^vec([234])<[fui]32>$/.exec(t); if (v) { const n = +v[1]; return n === 2 ? { align: 8, size: 8, stride: 0 } : { align: 16, size: n === 3 ? 12 : 16, stride: 0 }; }
    if (/^mat4x4(f|<f32>)$/.test(t)) return { align: 16, size: 64, stride: 0 };
    v = /^array<(.+),([^,]+)>$/.exec(t); if (v) { const el = sizeOf(v[1]), stride = up(el.size, space === 'uniform' ? Math.max(16, el.align) : el.align); return { align: space === 'uniform' ? Math.max(16, el.align) : el.align, size: stride * constOf(v[2]), stride: stride / 4 }; }
    if (new RegExp(`struct ${t}\\s*\\{`).test(src)) { const l = layoutOf(text, t, space, seen), size = up(l.words * 4, l.align); return { align: space === 'uniform' ? up(l.align, 16) : l.align, size: space === 'uniform' ? up(size, 16) : size, stride: 0 }; }
    return die(`${name}: a type this does not read: '${t}'`);
  };
  const members: Member[] = []; let off = 0, align = 4;
  for (const part of split(m[1])) {
    const p = /^(?:@\w+\([^)]*\)\s*)*([A-Za-z_]\w*)\s*:\s*(.+)$/.exec(part) ?? die(`${name}: '${part}'`), s = sizeOf(p[2]); off = up(off, s.align); align = Math.max(align, s.align);
    members.push({ name: p[1], type: p[2].replace(/\s+/g, ' ').trim(), word: off / 4, words: s.size / 4, stride: s.stride }); off += s.size;
  }
  const l: Layout = { name, members, words: up(off, align) / 4, align }; seen.set(name, l); return l;
}
const camel = (s: string): string => s.toLowerCase().split('_').map(w => w[0].toUpperCase() + w.slice(1)).join('');
const num = (t: string): number => { const s = t.trim().replace(/u\b/g, ''); const m = /^\(?\s*(\d+)\s*<<\s*(\d+)\s*\)?$/.exec(s); if (/^\d+$/.test(s)) return +s; if (/^0x[0-9a-f]+$/i.test(s)) return parseInt(s, 16); return m ? +m[1] * 2 ** +m[2] : die(`a bit this does not read: '${t}'`); };
export const bitsOf = (text: string, prefix: string): [string, number][] => [...text.matchAll(new RegExp(`^const (${prefix}[A-Z0-9_]+)\\s*:\\s*u32\\s*=\\s*([^;]+);`, 'gm'))].map(m => [m[1], num(m[2])]);
/** the consts between a comment line that says `marker` and the first line that is neither a const nor a comment */
export function blockOf(text: string, marker: string): [string, number][] {
  const lines = text.split('\n'), i = lines.findIndex(l => l.startsWith('//') && l.includes(marker)); if (i < 0) die(`no block '${marker}'`);
  const out: [string, number][] = []; for (const l of lines.slice(i + 1)) { const m = /^const ([A-Z0-9_]+)\s*:\s*u32\s*=\s*([^;]+);/.exec(l); if (m) out.push([m[1], num(m[2])]); else if (!l.startsWith('//')) break; } return out;
}
const pow2 = (v: number): boolean => v > 0 && v <= 0x80000000 && (v & (v - 1)) === 0;
function oneBits(as: string, bits: [string, number][]): void {
  if (!bits.length || bits.some(([, v]) => !pow2(v)) || new Set(bits.map(b => b[1])).size !== bits.length) die(`${as}: each a distinct single bit: ${bits.map(b => b.join(' ')).join(', ')}`);
}
export function written(): string {
  const L: string[] = ['// Written by tools/gpu/wgsl-rows.ts from the shaders\' own structs. To change one, change it in its shader and run that.', ''];
  const read = new Map<string, string>(), text = (f: string): string => { let t = read.get(f); if (t === undefined) read.set(f, t = readFileSync(join(ROOT, f), 'utf8')); return t; };
  const done = new Set<string>();
  const emit = (as: string, l: Layout, file: string): void => {
    if (done.has(as)) return; done.add(as);
    L.push(`/** ${file} struct ${l.name}, in words of four bytes */`, `export const enum ${as} {`);
    for (const m of l.members) L.push(`  /** ${m.type} */`, `  ${m.name} = ${m.word},`);
    L.push('  /** words it takes */', `  words = ${l.words},`, '}', '');
  };
  for (const { file, struct, as, space, also } of STRUCTS) {
    const seen = new Map<string, Layout>(), l = layoutOf([file, ...(also ?? [])].map(text).join('\n'), struct, space, seen); emit(as, l, file);
    for (const [n, e] of seen) if (n !== struct && !done.has(`${n}At`)) emit(`${n}At`, e, file);
  }
  for (const { file, prefix, as, doc } of FLAGS) { const bits = bitsOf(text(file), prefix); oneBits(as, bits); L.push(`/** ${doc} */`, `export const ${as} = {`, ...bits.map(([n, v]) => `  ${camel(n.slice(prefix.length))}: ${v},`), '} as const;', ''); }
  for (const { file, marker, as, doc } of BLOCKS) { const bits = blockOf(text(file), marker); oneBits(as, bits); L.push(`/** ${doc} */`, `export const ${as} = {`, ...bits.map(([n, v]) => `  ${n}: ${v},`), '} as const;', ''); }
  return L.join('\n');
}
if (import.meta.main) {
  const w = written(), now = existsSync(OUT) ? readFileSync(OUT, 'utf8') : '';
  if (process.argv.includes('--check')) { if (w !== now) { console.log('wgsl-rows: src/gpu/wgslrows.ts is not of these shaders (bun run tools/gpu/wgsl-rows.ts)'); process.exit(1); } console.log('wgsl-rows: src/gpu/wgslrows.ts is of these shaders'); }
  else if (w !== now) { writeFileSync(OUT, w); console.log(`wgsl-rows: src/gpu/wgslrows.ts written (${w.split('\n').length} lines)`); } else console.log('wgsl-rows: nothing to write');
}
