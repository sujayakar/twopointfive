// A WRITE BY PATH (blitz/skeleton/CHANGES-15.md): the page writes game state outside the tick in many small places. Here one call writes a carried field on the puppet and on the Rust game, with no face call a field:
//   const poke = makePoke(g, knobs, send);    send = (bytes) => void: the face's call of io::poke
//   poke(g, 'aiEnabled', false);  poke(g.player, 'launchHeld', null);  poke(guard, ['script', 'goal'], [1, 0, 2]);  poke(g, ['m', 'dialogue', 'x'], v);
// The first argument is the game or a row of a table. The path is by the TypeScript's field names (or the schema's Rust names); a list takes a number, a map its key, and an Option is stepped through by itself.
// It throws and sends nothing where a field has no way to be written like this: not carried, or worked out from other things by the walk. A method call is not this: it runs a rule, so it goes to the Rust game by name (ask.ts `give`) or stays on the puppet.
import { SCHEMA, parse, bookOf, compiledExpr, exportPart, helpersOf } from './export';
type N = any;
export function makePoke(g: any, knobs: Record<string, unknown>, send: (b: Uint8Array) => void) {
  const bk = bookOf(g); if (!bk) throw new Error('poke: this game has never been exported, so no row has an ID: export it once first');
  const rows = new Map<object, [number, number]>(); let seen = -1;
  const rowOf = (o: object): [number, number] | null => { const n = SCHEMA.tables.reduce((a: number, t: any) => a + bk.listOf(t.id).length, 0); if (n !== seen) { seen = n; SCHEMA.tables.forEach((t: any, k: number) => bk.listOf(t.id).forEach((r, i) => rows.set(r, [k, i]))); } return rows.get(o) ?? null; };
  return function poke(obj: any, path: string | (string | number)[], value: unknown): void {
    const P = typeof path === 'string' ? [path] : path; let N: N, o = obj; const head = new Uint8Array(8), hv = new DataView(head.buffer); let headLen: number;
    if (obj === g) { N = { k: 'type', path: SCHEMA.root }; hv.setUint32(0, 0xffffffff, true); headLen = 4; } else { const at = rowOf(obj); if (!at) throw new Error('poke: the first argument is neither the game nor a row of a table'); N = parse(SCHEMA.tables[at[0]].ty); hv.setUint32(0, at[0], true); hv.setUint32(4, at[1], true); headLen = 8; }
    const way: [string | N, unknown][] = []; const u16 = (x: number) => way.push(['u16', x]), u32 = (x: number) => way.push(['u32', x]), u8 = (x: number) => way.push(['u8', x]); const B = helpersOf(g, knobs); let set: ((v: unknown) => void) | null = null;
    for (let i = 0; i < P.length; i++) { const s = P[i]; while (N.k === 'opt') { u8(1); N = N.of; }
      if (N.k === 'type') { const T = SCHEMA.types[N.path]; if (T.k !== 'struct') throw new Error(`poke: ${T.name} has no fields`); const f = T.fields.find((x: any) => x.ts === s) ?? T.fields.find((x: any) => x.rust === s); if (!f) throw new Error(`poke: ${T.name} carries no field ${String(s)}`); u16(T.fields.indexOf(f)); const h = o, q = `${T.mod}::${T.name}.${f.rust}`; if (f.echo) throw new Error(`poke: ${q} is Rust's own: the puppet holds no copy to write`);
        const plain = f.expr?.match(/^o\.([A-Za-z_$][\w$]*)(?: \?\? .*)?$/)?.[1];   // (the walk reads a field of another name)
        if (f.expr === undefined || plain) { const key = plain ?? f.ts; o = h?.[key]; set = v => { h[key] = v; }; } else { const top = f.expr.match(/^B\.top\("[^"]+", "[^"]+", "([^"]*)"\)$/); o = f.expr === 'o' ? h : compiledExpr(f.expr)(h, g, B); set = top ? (v => { const V: any = B.varOf(top[1]); if (!V) throw new Error(`poke: no setter for ${top[1]}`); V.set(v); }) : (() => { throw new Error(`poke: ${q} is worked out by the walk, so it has no way to be written`); }); if (!top && i === P.length - 1) set(undefined); } N = parse(f.ty); continue; }
      if (N.k === 'vec' || N.k === 'arr') { if (typeof s !== 'number') throw new Error('poke: a list takes a number'); u32(s); const h = o; o = h[s]; set = v => { h[s] = v; }; N = N.of; continue; }
      if (N.k === 'map') { u8(1); way.push([N.K, s]); const h = o; o = h?.get ? h.get(s) : h?.[s]; set = v => { if (h.set) h.set(s, v); else h[s] = v; }; N = N.V; continue; }
      throw new Error(`poke: no way down through a ${N.k}`); }
    if (N.k === 'type' && SCHEMA.types[N.path].k === 'struct') u16(0xffff); else if (N.k === 'vec' || N.k === 'arr') u32(0xffffffff); else if (N.k === 'opt' || N.k === 'map') u8(0);
    way.push([N, value]); const rest = exportPart(g, knobs, way as any), b = new Uint8Array(headLen + rest.length); b.set(head.subarray(0, headLen)); b.set(rest, headLen);
    if (!set) throw new Error('poke: an empty path'); set(value); send(b); }; }
