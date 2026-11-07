// THE CALL SURFACE BY KEY, FOR THE NAMES LEFT IN THE SIMULATION'S SOURCE (src/game/crossing.ts; tools/blitz/DELETION.md). A stub and a guard name their row by
// its key; the surface (surface.ts, and expose.ts's stand-in before a game is stood) takes a row's number. This file is the table between the two, read from
// calls.json and taps.json as they stand, and it answers through whatever `globalThis.__tpsCall` is at the moment of the call, as the lines expose.ts used
// to write at load did. Loaded by the bridge's preload wherever a word asks for the Rust game. Written by tools/blitz/delete-tick.ts.
import { readFileSync } from 'node:fs';
import { setSurface, RUST_PASS } from '../../src/game/crossing.ts';
const CALLS = JSON.parse(readFileSync(new URL('./calls.json', import.meta.url).pathname, 'utf8')), TAPS: any[] = JSON.parse(readFileSync(new URL('./taps.json', import.meta.url).pathname, 'utf8'));
const n = new Map<string, number>(); for (const r of CALLS.rows) n.set(r.key, (n.get(r.key) ?? 0) + 1);
const ROW = new Map<string, number>(); for (const r of CALLS.rows) ROW.set(r.stub ?? (n.get(r.key)! > 1 ? `${r.key}@${r.file}` : r.key), r.id);
const TAP = new Map<string, number>(); for (const t of TAPS) TAP.set(t.key, t.id);
const G = globalThis as any;
setSurface({
  call(key, self, args) { const c = G.__tpsCall; if (!c) return RUST_PASS; const row = ROW.get(key); if (row === undefined) throw new Error(`${key}: calls.json has no row of that key`); const r = c(row, self, args); return r === c.PASS ? RUST_PASS : r; },
  tapFiring(key) { const c = G.__tpsCall, id = TAP.get(key); return id !== undefined && !!c?.tapFiring?.(id); },
  tapAnswer() { return G.__tpsCall.tapAnswer(); },
  newGame() { G.__tpsCall?.newGame?.(); },
});
