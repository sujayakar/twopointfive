// THE WRITE BY PATH'S CHECK. A real game plays to a frame and goes into the Rust game. Then every plain field the walk can reach (the first of each list, one row of each table) is poked with the value it already holds: a route that went to the wrong field writes another value there, and the two exports part. Then a few real writes, on both sides, compared the same way. Needs the face built with face_write.
//   bun run --preload ./tools/blitz/expose.ts tools/blitz/poke-check.ts <scn>:<seed>:<frame> [--wasm FILE]
// Exit 0 the two sides the same after all of it · 1 they part · 3 could not run. POKE_BREAK=1: the real writes go to the puppet only, and it must go red.
import { readFileSync } from 'node:fs';
import { exportGame, SCHEMA, parse, bookOf } from './export';
import { makePoke } from './poke';
import { decode } from './paths';
const argv = process.argv.slice(2); const [scn, seed, F] = argv[0].split(':'); const arg = (n: string, d: string) => { const i = argv.indexOf(n); return i < 0 ? d : argv[i + 1]; };
const { knobRegistry } = await import('../../src/game/knobs.ts'); const reg = knobRegistry();
const { Game } = await import('../../src/game/game.ts'); const { play } = await import('../qa/playthrough.ts'); class Stop extends Error {}
const WASM = arg('--wasm', new URL('../../rust/game_face/target/wasm32-unknown-unknown/release/game_face.wasm', import.meta.url).pathname);
const X: any = new WebAssembly.Instance(new WebAssembly.Module(readFileSync(WASM)), {}).exports; const u8 = () => new Uint8Array(X.memory.buffer);
const hand = <T>(b: Uint8Array, f: (p: number, n: number) => T): T => { const p = X.face_alloc(b.length || 1); u8().set(b, p); try { return f(p, b.length); } finally { X.face_free(p, b.length || 1); } };
const say = (x: string) => process.stdout.write(x + '\n'); let code = 0;
function check(g: any) { g.sim.settle?.(g); const start = exportGame(g, reg), w: Uint32Array = g.sim.w, block = new Uint8Array(w.buffer, w.byteOffset, w.byteLength).slice();
  { const n = 48 * 1048576, p = X.face_alloc(n); X.face_free(p, n); } const h = hand(start, (p, n) => hand(block, (bp, bn) => X.face_new(p, n, bp, bn))); if (!h) throw new Error('face_new: no free slot');
  const rust = () => { X.face_export(h); return u8().slice(X.face_out_ptr() >>> 0, (X.face_out_ptr() >>> 0) + X.face_out_len()); };
  let sends = 0, sent = 0; const poke = makePoke(g, reg, b => { sends++; sent += b.length; hand(b, (p, n) => X.face_write(h, p, n)); });
  const diff = (a: Uint8Array, b: Uint8Array) => { const A = decode(a), MB = new Map(decode(b).map((r: any) => [r.path, r])), D = new Map<string, number>(), eg: string[] = []; for (const r of A as any[]) { if (r.q && SCHEMA.serial[r.q]) continue; const o: any = MB.get(r.path); if (!o || o.bits !== r.bits) { D.set(r.q ?? r.path, (D.get(r.q ?? r.path) ?? 0) + 1); if (eg.length < 6) eg.push(`${r.path}: ${r.bits} / ${o?.bits}`); } } return { D, eg }; };
  const base = diff(start, rust()); say(`before any write, the export against Rust's: ${base.D.size} fields differ${base.D.size ? ': ' + [...base.D.keys()].slice(0, 30).join(' ') : ''}`); const baseQ = new Set(base.D.keys());
  // every plain field, with what it holds
  let tried = 0, skipped = 0, failed: string[] = []; const seenT = new Set<string>(); const roots: [any, string][] = [[g, SCHEMA.root]]; const bk = bookOf(g)!; for (const t of SCHEMA.tables) { const L = bk.listOf(t.id); if (L.length) roots.push([L[0], t.ty.replace(/^alloc::boxed::Box<(.*)>$/, '$1')]); }
  const walk = (root: any, ty: string, o: any, path: (string | number)[], d: number) => { const T = SCHEMA.types[ty]; if (!T || T.k !== 'struct' || !o || typeof o !== 'object' || d > 5) return; for (const f of T.fields) { const q = `${T.mod}::${T.name}.${f.rust}`; if (f.echo || f.expr !== undefined && f.expr !== 'o') { skipped++; continue; } const P = [...path, f.ts ?? f.rust]; const v = f.expr === 'o' ? o : o[f.ts]; if (f.expr !== 'o' && (v === undefined || typeof v === 'function')) { skipped++; continue; } if (f.expr !== 'o' && !seenT.has(q)) { seenT.add(q); tried++; try { poke(root, P, v); } catch (e) { failed.push(`${q}: ${String(e).slice(0, 100)}`); } }
        const N = parse(f.ty); const inner = N.k === 'opt' ? N.of : N; if (inner.k === 'type') walk(root, inner.path, v, P, d + 1); else if (inner.k === 'vec' && inner.of.k === 'type' && Array.isArray(v) && v.length) walk(root, inner.of.path, v[0], [...P, 0], d + 1); } };
  for (const [r, ty] of roots) walk(r, ty, r, [], 0);
  say(`poked ${tried} fields with what they hold in ${sends} writes, ${sent} bytes; ${skipped} passed over (Rust's own or worked out); ${failed.length} refused`); for (const x of failed.slice(0, 12)) say('  ' + x);
  const after1 = diff(exportGame(g, reg), rust()); const new1 = [...after1.D.keys()].filter(q => !baseQ.has(q)); say(`after them: ${new1.length} fields differ that did not before${new1.length ? ': ' + new1.slice(0, 40).join(' ') : ''}`); for (const e of after1.eg.slice(0, 4)) say('  ' + e); if (new1.length) code = 1;
  // real writes
  const BRK = process.env.POKE_BREAK === '1'; const real = makePoke(g, reg, b => { if (!BRK) hand(b, (p, n) => X.face_write(h, p, n)); });
  real(g, 'aiEnabled', !g.aiEnabled); real(g.player, 'launchHeld', null); real(g.player, 'visibility', 0.25); const gd = (g.guards as any[]).find(x => x.script?.goal) ?? g.guards[0]; if (gd.script?.goal) real(gd, ['script', 'goal'], [1, 0, 2]); real(gd, 'stuckT', 1.5); real(g.player, 'wall', null); real(g, 'infiniteAmmo', !g.infiniteAmmo);
  const after2 = diff(exportGame(g, reg), rust()); const new2 = [...after2.D.keys()].filter(q => !baseQ.has(q)); say(`after the real writes: ${new2.length} fields differ${new2.length ? ': ' + new2.slice(0, 20).join(' ') : ''}`); if (new2.length) code = 1; }
const was = Game.prototype.update; let n = 0; Game.prototype.update = function (this: any, ...a: unknown[]) { n++; if (n === +F) { try { check(this); } catch (e) { say(`could not run: ${(e as any)?.stack ?? e}`); process.exit(3); } throw new Stop(); } return (was as any).apply(this, a); };
console.log = () => {}; await (play as any)(scn, +seed, 300, false, {}).catch((e: unknown) => { if (!(e instanceof Stop)) { say(`could not run: ${String(e)}`); process.exit(3); } }); process.exit(code);
