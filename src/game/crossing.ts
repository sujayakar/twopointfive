// THE CROSSING FROM A TYPESCRIPT NAME TO THE RUST GAME'S RULE (tools/blitz/DELETION.md). The simulation steps in Rust (rust/game); what is left of it
// in TypeScript is the puppet's data, the start of a game, the picture's readers, and these names. A function whose TypeScript went with the tick and that
// something outside the tick still calls is one line: `return crossToRust('Class.method', this, [args]) as any`. A function that kept its body and is on the
// call surface opens with the surface's guard: between two ticks under a host the call crosses, anywhere else its TypeScript runs. Written by
// tools/blitz/delete-tick.ts; the surface itself (rows by key, the bytes, the host) is tools/blitz's and is handed in by `setSurface`. No surface, as on
// the page: a guard passes, a tap never fires, and a crossing throws by name.
export const RUST_PASS: unique symbol = Symbol('run the TypeScript');
export interface Surface {
  /** hand the call to the Rust game: its answer, or RUST_PASS where it does not cross now (no host stands, the host's own lines are running, a row Rust has not got) */
  call(key: string, self: unknown, args: unknown[]): unknown;
  /** a tool's replacement of this function is calling through to the original: answer from the record */
  tapFiring(key: string): boolean; tapAnswer(): unknown;
  /** a new game is standing: nothing crosses to the last game's host */
  newGame(): void;
}
export let RUST_SURFACE: Surface | null = null;
export function setSurface(s: Surface | null): void { RUST_SURFACE = s; }
/** the body of a rule only the Rust game has */
export function crossToRust(key: string, self: unknown, args: unknown[]): unknown {
  if (!RUST_SURFACE) throw new Error(`${key}: a rule that has no TypeScript body was called where no call surface is installed (the page, or a tool run without the bridge's preload): only the Rust game has it. If a start or a menu needs it as TypeScript, the cure is a row in tools/blitz/delete-keep.json and the deletion run again (tools/blitz/DELETION.md)`);
  const r = RUST_SURFACE.call(key, self, args);
  if (r === RUST_PASS) throw new Error(`${key}: a start called a rule that has no TypeScript body. Only the Rust game has it, and this call did not cross: no host of a Rust game stands for it yet (a start), or it was made inside the host's own lines (an apply, the dress step), or on a row the Rust game has not got. The census of starts never saw this call (tools/blitz/starts.json); the cure is to add the start that makes it to that table, or a row in tools/blitz/delete-keep.json, and run the deletion again (tools/blitz/DELETION.md)`);
  return r; }
/** the body of a function that went with the tick and has no row on the call surface */
export function goneWithTick(key: string): never { throw new Error(`${key}: this function's TypeScript went with the tick, and the call surface has no row for it (tools/blitz/DELETION.md lists who named it)`); }
