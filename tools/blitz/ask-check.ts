// THE MENU'S QUESTIONS, BOTH SIDES. A real game plays to a frame; its state goes into the Rust game; then for every order row and every target it fits, the row's own TypeScript closure and the Rust game (io::ask, by way of ask.ts) answer the same question. Needs the face built with face_ask.
//   bun run --preload ./tools/blitz/expose.ts tools/blitz/ask-check.ts <scn>:<seed>:<frame> [--wasm FILE] [--show 40]
// Exit 0 every answer the same · 1 some differ · 3 could not run. A row whose TypeScript closure is absent asks nothing of it: absent is what Rust's "no such arm" answers.
import { readFileSync } from 'node:fs';
import { exportGame, SCHEMA } from './export';
import { decode } from './paths';
import { installAsks, ASK } from './ask';
const argv = process.argv.slice(2); const [scn, seed, F] = argv[0].split(':'); const arg = (n: string, d: string) => { const i = argv.indexOf(n); return i < 0 ? d : argv[i + 1]; };
const { knobRegistry } = await import('../../src/game/knobs.ts'); const reg = knobRegistry();
const { Game } = await import('../../src/game/game.ts'); const { play } = await import('../qa/playthrough.ts'); class Stop extends Error {}
const WASM = arg('--wasm', new URL('../../rust/game_face/target/wasm32-unknown-unknown/release/game_face.wasm', import.meta.url).pathname);
const X: any = new WebAssembly.Instance(new WebAssembly.Module(readFileSync(WASM)), { env: { face_roll: () => 0 } }).exports; const u8 = () => new Uint8Array(X.memory.buffer);
const hand = <T>(b: Uint8Array, f: (p: number, n: number) => T): T => { const p = X.face_alloc(b.length || 1); u8().set(b, p); try { return f(p, b.length); } finally { X.face_free(p, b.length || 1); } };
const say = (x: string) => process.stdout.write(x + '\n'); let code = 0;
const same = (a: unknown, b: unknown) => JSON.stringify(a) === JSON.stringify(b);
function check(g: any) { g.sim.settle?.(g); const start = exportGame(g, reg), w: Uint32Array = g.sim.w, block = new Uint8Array(w.buffer, w.byteOffset, w.byteLength).slice();
  { const n = 48 * 1048576, p = X.face_alloc(n); X.face_free(p, n); }
  const h = hand(start, (p, n) => hand(block, (bp, bn) => X.face_new(p, n, bp, bn))); if (!h) throw new Error('face_new: no free slot');
  const BRK = process.env.ASK_BREAK === '1';   // (the check's own check: asked of the wrong row, it must go red)
  const ask = (kind: number, row0: number, seat: number, b: Uint8Array) => { const row = BRK ? (row0 + 1) % g.orders.defs.length : row0; hand(b, (p, n) => X.face_ask(h, kind, row, seat, p, n)); return u8().slice(X.face_out_ptr(), X.face_out_ptr() + X.face_out_len()); };
  // the TypeScript's own closures, kept before the stubs go on the same rows
  const S = g.orders, rows: any[] = S.defs.slice(), own = new Map(rows.map(d => [d, { label: d.label, can: d.can, only: d.only, valid: d.valid, gone: d.gone, issue: d.issue, when: d.standsIn?.when }])); const posture = S.posture;
  // the questions are asked over the game as it stands, and the stubs do not go on `rows`' closures until each TypeScript answer is taken
  const P = g.player?.char?.pos ?? [0, 0, 0]; const targets: any[] = [S.targetAt([P[0], P[1], P[2]])]; const doors: any[] = g.doors?.list ?? []; const near = doors.map(d => ({ d, r: Math.hypot(d.pos?.[0] - P[0], d.pos?.[2] - P[2]) })).filter(x => x.r === x.r).sort((a, b) => a.r - b.r).slice(0, 10);
  for (const { d } of near) targets.push(S.targetAt([d.pos[0], d.pos[1], d.pos[2]]), { kind: 'door', at: d.pos, door: d, side: 1 }, { kind: 'door', at: d.pos, door: d, side: -1 }); targets.push({ kind: 'none', at: [P[0], P[1], P[2]] });
  const foe = (g.guards as any[]).find(x => x.team === 'hostile' && !x.dead); if (foe) targets.push({ kind: 'foe', at: foe.char?.pos ?? P, foe });
  const T: { kind: string; row: string; t: string; ts: unknown; ru: () => unknown }[] = []; const asks = installAsks(g, reg, ask, () => {}); const stub = new Map(rows.map(d => [d, { label: d.label, can: d.can, only: d.only, valid: d.valid, gone: d.gone, when: d.standsIn?.when }]));
  const q = (kind: string, d: any, t: any, ts: () => unknown, ru: () => unknown) => { let a: unknown; try { a = ts(); } catch (e) { a = `threw ${String(e).slice(0, 60)}`; } T.push({ kind, row: d.id, t: t.kind + (t.door ? `#${doors.indexOf(t.door)}` : ''), ts: a, ru }); };
  for (const t of targets) { let men: any[]; try { men = S.recipients(t); } catch { men = []; } for (const d of rows) { if (!(d.targets as string[]).includes(t.kind) && !(d.listed ?? []).includes(t.kind)) continue; const O = own.get(d)!, Y = stub.get(d)!;
      q('label', d, t, () => typeof O.label === 'function' ? O.label(t) : O.label, () => Y.label(t));
      for (const m of men.slice(0, 4)) q('can', d, t, () => O.can ? O.can(S, m, t) : true, () => Y.can(S, m, t));
      q('only', d, t, () => (O.only ? O.only(S, men, t) : men).map((x: any) => g.guards.indexOf(x)), () => Y.only(S, men, t).map((x: any) => g.guards.indexOf(x)));
      q('valid', d, t, () => O.valid ? O.valid(S, t, men) : true, () => Y.valid(S, t, men));
      q('gone', d, t, () => O.gone ? !!O.gone(S, t, men) : false, () => Y.gone(S, t, men));
      if (O.when) q('when', d, t, () => !!O.when(S, t, men), () => Y.when(S, t, men)); } }
  if (posture) { const Q = S.posture; for (const d of rows) { T.push({ kind: 'mark', row: d.id, t: '', ts: posture.mark(d), ru: () => Q.mark(d) }); } T.push({ kind: 'word', row: '', t: '', ts: posture.word(), ru: () => Q.word() }); }
  let n = 0; const bad: Record<string, number> = {}, ok: Record<string, number> = {}, eg: string[] = []; for (const r of T) { n++; let b: unknown; try { b = r.ru(); } catch (e) { b = `threw ${String(e).slice(0, 90)}: ${String(X.face_panic_len ? new TextDecoder().decode(u8().subarray(X.face_panic_ptr(), X.face_panic_ptr() + X.face_panic_len())) : '').slice(0, 200)}`; X.face_panic_clear?.(); } if (same(r.ts, b)) ok[r.kind] = (ok[r.kind] ?? 0) + 1; else { bad[r.kind] = (bad[r.kind] ?? 0) + 1; if (eg.length < +arg('--show', '40')) eg.push(`  ${r.kind} ${r.row} on ${r.t}: TypeScript ${JSON.stringify(r.ts)?.slice(0, 90)} Rust ${JSON.stringify(b)?.slice(0, 220)}`); } }
  if (arg('--sample', '')) { const seen = new Set<string>(); for (const r of T) { const k = `${r.kind}:${JSON.stringify(r.ts)}`; if (seen.has(k) || seen.size > +arg('--sample', '0')) continue; seen.add(k); say(`  ${r.kind} ${r.row} ${r.t}: ${JSON.stringify(r.ts)?.slice(0, 80)}`); } }
  say(`${scn}:${seed}:${F}: ${rows.length} rows, ${targets.length} targets, ${n} questions; the same ${JSON.stringify(ok)}; differ ${JSON.stringify(bad)}`); for (const l of eg) say(l); if (Object.keys(bad).length) code = 1;
  // ---- an order, on both sides: the row's own closures back on the TypeScript's rows first
  if (!process.env.ASK_NO_GIVE) { const rust = () => { X.face_export(h); return u8().slice(X.face_out_ptr() >>> 0, (X.face_out_ptr() >>> 0) + X.face_out_len()); }; const diff = (a: Uint8Array, b: Uint8Array) => { const A = decode(a), MB = new Map(decode(b).map((r: any) => [r.path, r])), D = new Map<string, string>(); for (const r of A as any[]) { if (r.q && (SCHEMA.serial[r.q] || SCHEMA.scratch[r.q])) continue; const o: any = MB.get(r.path); if (!o || o.bits !== r.bits) if (!D.has(r.q ?? r.path)) D.set(r.q ?? r.path, `${r.path}: ${r.bits} / ${o?.bits}`); } return D; };
    for (const d of rows) { const O = own.get(d)!; d.label = O.label; d.can = O.can; d.only = O.only; d.valid = O.valid; d.gone = O.gone; d.issue = O.issue; if (d.standsIn) d.standsIn.when = O.when; } S.posture = posture; const base = new Set(diff(exportGame(g, reg), rust()).keys());
    const door = near[0]?.d; const G: [string, any][] = [['hold', { kind: 'none', at: [P[0], P[1], P[2]] }], ['fallIn', { kind: 'none', at: [P[0], P[1], P[2]] }], ['move', S.targetAt([P[0] + 3, P[1], P[2]])], ...(door ? [['cover', S.targetAt([door.pos[0], door.pos[1], door.pos[2]])], ['stack', S.targetAt([door.pos[0], door.pos[1], door.pos[2]])]] as [string, any][] : [])];
    for (const [id, t] of G) { const a = S.give(id, t), b = asks.give(S, id, t); const now = diff(exportGame(g, reg), rust()); const fresh = [...now].filter(([q]) => !base.has(q)); say(`give ${id} on ${t.kind}: TypeScript ${JSON.stringify(a)} Rust ${JSON.stringify(b)}; ${fresh.filter(([q]) => !/new_leg/.test(q)).length} fields part, past new_leg: ${fresh.filter(([q]) => !/new_leg/.test(q)).slice(0, 8).map(([, x]) => x).join(' · ')}`); if (JSON.stringify(a) !== JSON.stringify(b)) code = 1; for (const [q] of fresh) base.add(q); } }
  return;
  say(`${scn}:${seed}:${F}: ${rows.length} rows, ${targets.length} targets, ${n} questions; the same ${JSON.stringify(ok)}; differ ${JSON.stringify(bad)}`); for (const l of eg) say(l); if (Object.keys(bad).length) code = 1; void ASK; void asks; }
const was = Game.prototype.update; let n = 0; Game.prototype.update = function (this: any, ...a: unknown[]) { n++; if (n === +F) { try { check(this); } catch (e) { say(`could not run: ${String(e)}`); process.exit(3); } throw new Stop(); } return (was as any).apply(this, a); };
console.log = () => {}; await (play as any)(scn, +seed, 300, false, {}).catch((e: unknown) => { if (!(e instanceof Stop)) { say(`could not run: ${String(e)}`); process.exit(3); } }); process.exit(code);
