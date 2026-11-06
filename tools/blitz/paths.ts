// THE STATE BRIDGE, READING IT BACK: a blob of export.ts or of rust/game's io.rs as a flat list of (path, bits), with real names, and the difference of two blobs: every
// path that differs, grouped by the module that declares the field. That list is the work left to mend.
//   bun run tools/blitz/paths.ts a.bin b.bin [--json]
import { SCHEMA, parse, type Node } from './export';

class In { dv: DataView; p = 0; constructor(public b: Uint8Array) { this.dv = new DataView(b.buffer, b.byteOffset, b.byteLength); }
  u8() { return this.b[this.p++]; } u16() { const x = this.dv.getUint16(this.p, true); this.p += 2; return x; } u32() { const x = this.dv.getUint32(this.p, true); this.p += 4; return x; } hex(n: number) { let s = ''; for (let i = 0; i < n; i++) s += this.b[this.p + i].toString(16).padStart(2, '0'); this.p += n; return s; } }
const DEC = new TextDecoder();
export type Row = { path: string; bits: string; mod: string };
/** the field being read holds an object's identity serial (schema.serial, which serial-fields.json lists): two games number those apart by design, so its value reads as `serial`, and a map or set keyed by one reads by its place */
let SER = '', KEYSER = false;
function rd(n: Node, i: In, path: string, mod: string, out: Row[]): string {
  const leaf = (bits: string) => { out.push({ path, bits: SER && bits !== 'None' ? 'serial' : bits, mod }); return bits; };
  switch (n.k) {
    case 'f64': { const v = i.dv.getFloat64(i.p, true); const h = i.hex(8); return leaf(`${v} ${h}`); }
    case 'f32': { const v = i.dv.getFloat32(i.p, true); const h = i.hex(4); return leaf(`${v} ${h}`); }
    case 'bool': return leaf(i.u8() ? 'true' : 'false'); case 'u8': case 'i8': return leaf(String(i.u8())); case 'u16': case 'i16': return leaf(String(i.u16())); case 'u32': case 'i32': case 'usize': return leaf(String(i.u32())); case 'u64': case 'i64': return leaf(i.hex(8));
    case 'id': return leaf(`${n.id}(${i.u32()})`);
    case 'str': { const L = i.u32(); const s = DEC.decode(i.b.subarray(i.p, i.p + L)); i.p += L; return leaf(JSON.stringify(s)); }
    case 'unit': return '';
    case 'opt': { if (!i.u8()) return leaf('None'); return rd(n.of, i, path, mod, out); }
    case 'vec': { const L = i.u32(); out.push({ path: `${path}.len`, bits: String(L), mod }); for (let k = 0; k < L; k++) rd(n.of, i, `${path}[${k}]`, mod, out); return ''; }
    case 'arr': for (let k = 0; k < n.n; k++) rd(n.of, i, `${path}[${k}]`, mod, out); return '';
    case 'tup': n.of.forEach((x: Node, k: number) => rd(x, i, `${path}.${k}`, mod, out)); return '';
    case 'map': { const L = i.u32(); out.push({ path: `${path}.len`, bits: String(L), mod }); const ks = KEYSER; KEYSER = false; for (let k = 0; k < L; k++) { const K: Row[] = []; rd(n.K, i, '', mod, K); rd(n.V, i, ks ? `${path}{#${k}}` : `${path}{${K.map(x => x.bits).join(',')}}`, mod, out); } return ''; }
    case 'set': { const L = i.u32(); out.push({ path: `${path}.len`, bits: String(L), mod }); const ks = KEYSER; KEYSER = false; for (let k = 0; k < L; k++) { const K: Row[] = []; rd(n.K, i, '', mod, K); out.push({ path: ks ? `${path}{#${k}}` : `${path}{${K.map(x => x.bits).join(',')}}`, bits: 'in', mod }); } return ''; }
    case 'type': { const T = SCHEMA.types[n.path];
      if (T.k === 'struct') { for (const f of T.fields) { const q = `${T.mod}::${T.name}.${f.rust}`, s0 = SER, k0 = KEYSER; if (SCHEMA.serial?.[q]) SER = q; KEYSER = !!SCHEMA.serial?.[`${q}{}`]; rd(parse(f.ty), i, `${path}.${f.rust}`, T.mod, SCHEMA.scratch?.[q] ? [] : out); SER = s0; KEYSER = k0; } return ''; }   // (schema.scratch: a memo no rule reads back: carried, not compared)
      if (T.k === 'enum') { const w = i.u16(); return leaf(`${T.name}::${T.words[w]}`); }
      if (T.k === 'data') { const a = T.vars[i.u16()]; out.push({ path: `${path}#`, bits: a.tag, mod }); if (a.ty) rd(parse(a.ty), i, `${path}.${a.tag}`, mod, out); return ''; }
      if (T.k === 'handEnum') { const a = T.arms[i.u16()]; out.push({ path: `${path}#`, bits: a, mod }); const t = a.match(/\((.*)\)/)?.[1]; if (t) rd(parse(t), i, `${path}.${a.split('(')[0]}`, mod, out); return ''; }
    }
  }
  throw new Error(`bridge: no reader for ${n.k} at ${path}`);
}
export function decode(b: Uint8Array): Row[] {
  const i = new In(b), out: Row[] = []; rd({ k: 'type', path: SCHEMA.root }, i, 'g', 'game', out);
  for (const t of SCHEMA.tables) { const L = i.u32(); const N = parse(t.ty); const nm = t.table.replace(/^tab\./, 'tab.'); out.push({ path: `g.${nm}.len`, bits: String(L), mod: t.mod ?? 'game' }); for (let k = 0; k < L; k++) rd(N, i, `g.${nm}[${k}]`, t.mod ?? 'game', out); }
  out.push({ path: 'g.serial_next', bits: String(i.u32()), mod: 'game' });
  // the floats of poses no pool holds: carried for an import, and no comparison reads them (a Rust pose lives in the pool, so its list stands as it was imported)
  for (let n = i.u32(); n > 0; n--) { i.u32(); const L = i.u32(); i.p += L * 4; }
  if (i.p !== b.length) throw new Error(`bridge: ${b.length - i.p} bytes left over`);
  return out;
}
/** every path that differs, by the module that declares the field */
export function diff(a: Uint8Array, b: Uint8Array): Map<string, { path: string; a: string; b: string }[]> {
  const A = decode(a), B = decode(b), M = new Map<string, { path: string; bits: string; mod: string }>(); for (const r of B) M.set(r.path, r);
  const out = new Map<string, { path: string; a: string; b: string }[]>(); const add = (mod: string, path: string, x: string, y: string) => { if (!out.has(mod)) out.set(mod, []); out.get(mod)!.push({ path, a: x, b: y }); };
  const seen = new Set<string>(); for (const r of A) { seen.add(r.path); const q = M.get(r.path); if (!q) add(r.mod, r.path, r.bits, '(absent)'); else if (q.bits !== r.bits) add(r.mod, r.path, r.bits, q.bits); }
  for (const r of B) if (!seen.has(r.path)) add(r.mod, r.path, '(absent)', r.bits);
  return out;
}
if (import.meta.main) {
  const { readFileSync } = await import('node:fs'); const argv = process.argv.slice(2);
  const D = diff(new Uint8Array(readFileSync(argv[0])), new Uint8Array(readFileSync(argv[1])));
  if (argv.includes('--json')) console.log(JSON.stringify(Object.fromEntries(D))); else for (const [m, L] of [...D].sort((x, y) => y[1].length - x[1].length)) { console.log(`${m}: ${L.length}`); for (const r of L.slice(0, 40)) console.log(`  ${r.path}  ${r.a} | ${r.b}`); }
  process.exit(D.size ? 1 : 0);
}
