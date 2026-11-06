// THE HOST'S SIDE OF THE MISSION LOOP (README.md): the bridge's walk with its book readable, and ONE PART of a game as bytes, so that the host's half of a tick
// is a couple of kilobytes and not a whole state (a whole state grows by the frame's boxes every tick).
import { plugin } from 'bun';
// (export.ts keeps its book and its writers private, and it is written by the skeleton's program: nothing is added to the file. These lines are added to the
// module as it is loaded, in memory.)
const MORE = `
export const bookOf = (g: object) => BOOKS.get(g);
/** the bytes of values of the given schema types, written by the walk's own writers against the game's book (the book must exist: one exportGame first) */
export function exportPart(g: any, knobs: Record<string, unknown>, parts: [string, any][]): Uint8Array { G = g; BK = BOOKS.get(g)!; B = helpers(g, knobs); const o = new Out(); for (const [ty, v] of parts) wr(parse(ty), v, o); return o.view().slice(); }
/** a field of the game by its Rust path (['m', 'dialogue', 'room_rects']): its schema type and its value now */
export function fieldAt(g: any, knobs: Record<string, unknown>, path: string[]): { ty: string; v: any } { G = g; BK = BOOKS.get(g)!; B = helpers(g, knobs); let ty: string = SCHEMA.root, v: any = g;
  for (const p of path) { const T = SCHEMA.types[ty.replace(/^alloc::boxed::Box<(.*)>$/, '$1')]; if (!T || T.k !== 'struct') throw new Error('fieldAt: ' + ty + ' is no struct'); const f = T.fields.find((x: any) => x.rust === p); if (!f) throw new Error('fieldAt: no ' + p + ' in ' + ty);
    v = f.expr !== undefined ? compiled(f.expr)(v, G, B) : (f.ts !== undefined ? v[f.ts] : undefined); ty = f.ty; }
  return { ty, v }; }
`;
plugin({ name: 'blitz-books', setup(build) {
  build.onLoad({ filter: /[\\/]tools[\\/]blitz[\\/]export\.ts$/ }, async ({ path }) => {
    const text = await Bun.file(path).text();
    for (const name of ['BOOKS', 'helpers', 'Out', 'wr', 'compiled']) if (!new RegExp(`\\b(const|function|class) ${name}\\b`).test(text)) throw new Error(`tools/blitz/host.ts: export.ts no longer has ${name}`);
    return { contents: text + MORE, loader: 'ts' };
  });
} });
const X: any = await import('./export.ts');
export const exportGame: (g: any, knobs: Record<string, unknown>) => Uint8Array = X.exportGame;
export const report: () => Map<string, { n: number; eg: string }> = X.report;
/** the bridge's book of a game that has been exported once: `ids.get('DoorId')` (object → ID), `listOf('GuardId')` (a table's rows in ID order) */
export const bookOf: (g: object) => any = X.bookOf;
export const exportPart: (g: any, knobs: Record<string, unknown>, parts: [string, any][]) => Uint8Array = X.exportPart;
export const fieldAt: (g: any, knobs: Record<string, unknown>, path: string[]) => { ty: string; v: any } = X.fieldAt;
