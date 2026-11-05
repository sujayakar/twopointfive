// THE LIBRARY'S DOOR. Every kit that lays things in a level (scene/gwotKit.ts, scene/officeGen.ts officeKit, tools/levels/livedKit.ts, livedPieces.ts,
// lived-quiet.ts, lived-train.ts) hands its PIECES out through `pieces(kit, fns)`: the things a level asks for by name (a drum, a cot, a yard wall), as against
// the kit's tools (a raw box, a frame, a paint table), which it returns beside them as it always did. So a kit says which of its functions are pieces in ONE
// place, its own return, and tools/levels/library.ts can stand a kit up, watch a generator build and write the catalogue (docs/library.md) from what was
// laid, with no list of pieces typed anywhere else.
// NOBODY WATCHING (a level being exported, a check, the page): `pieces` returns the object it was given. No wrapper exists, so no level file, digest or
// frame can differ because this file does.
// No imports: scene/officeGen.ts is in the page's bundle and takes this with it.

export interface PieceWatch {
  /** a kit was stood up: its name and the names of its pieces */
  kit(kit: string, names: string[]): void;
  /** a piece is about to be laid. A piece laid from inside another (a `drums` laying each `drum`, a lived chair laying the office's) is told too, after its
   *  parent's `open` and before its `close`: the watcher keeps the depth */
  open(kit: string, name: string, args: readonly unknown[]): void;
  close(kit: string, name: string): void;
}
let watch: PieceWatch | null = null;
/** watch every kit stood up from now on (null = stop). Tools only: set it before the generator's build() runs */
export function watchPieces(w: PieceWatch | null): void { watch = w; }

/** a kit's pieces by name (see the head) */
export function pieces<T extends Record<string, unknown>>(kit: string, fns: T): T {
  const w = watch; if (!w) return fns;
  const out: Record<string, unknown> = {};
  for (const [name, f] of Object.entries(fns)) {
    out[name] = typeof f !== 'function' ? f : (...args: unknown[]) => { w.open(kit, name, args); try { return (f as (...a: unknown[]) => unknown).apply(fns, args); } finally { w.close(kit, name); } };
  }
  w.kit(kit, Object.keys(fns).filter(n => typeof fns[n] === 'function'));
  return out as T;
}
