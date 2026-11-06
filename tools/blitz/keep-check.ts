// A GAME KEPT AND PUT BACK, A SLOT GIVEN UP AND TAKEN AGAIN, A START REFUSED (tools/blitz/PLAY.md, "A game kept"). Run under a word:
//   tools/blitz/rust.sh rust bun run tools/blitz/keep-check.ts        (also under `ts`: the same lines on the TypeScript tick, the reference)
// 1 snapshot   the harness's own snapshot (`W.snapshot` / `W.restore`): ten seconds of play after the snapshot, then the snapshot put back and the same ten
//              seconds again: one digest a second, equal on all ten. Under `rust` the put-back must have been the kept game (`face_back`), not a new start.
// 2 controller the save controller's own save and load (`saves.write` by hand, `saves.load` of its key): the same claim
// 3 slots      six games stood and stepped in turn (rust/sim has four slots), then the first stepped again: its digests go on as a game that never
//              gave its slot up (held to a seventh game played straight through). Under `rust` it must have stood again from its kept bytes.
// 4 refused    (rust only) a game whose kept bytes were let go is not stepped again: the step throws and names this file
// Exit 0 all lines pass · 1 a line failed.
import { standUp, seedGame, ROOT } from '../qa/headless.ts';
const { digestGame } = await import(`${ROOT}/src/game/digest.ts`);
const { hashStr } = await import(`${ROOT}/src/game/dialogue.ts`);
const DT = 1 / 60, word = process.env.TPS_GAME_BY ?? process.env.TPS_GAME ?? '';
let fails = 0;
const line = (ok: boolean, what: string, detail: string) => { if (!ok) fails++; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}\n    ${detail}`); };
const secs = (W: any, n: number): string[] => { const out: string[] = []; for (let s = 0; s < n; s++) { for (let i = 0; i < 60; i++) W.step(DT); out.push(hashStr(JSON.stringify(digestGame(W.game))).toString(16)); } return out; };
const same = (a: string[], b: string[]) => { let n = 0; while (n < a.length && a[n] === b[n]) n++; return n; };
const calls = (W: any, re: RegExp): number => { let n = 0; for (const [k, c] of (W.game.rustHost?.calls ?? new Map()) as Map<string, number>) if (re.test(k)) n += c; return n; };
// (nobody at the keys: the harness's restore puts the camera on the man, so a moving camera would be another aim and another game on either tick)
const walk = (_W: any) => {};
// (a file of the game's fields does not hold the die: each future names its stream as it starts, as the checkpoint probes do. A kept Rust game holds its
// die, and the helper seeds that one too.)
const mulberry = (a: number) => () => { a |= 0; a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ (a >>> 15), 1 | a); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; };
const stream = (W: any) => { seedGame(W.game, mulberry(31), 31); };

console.log(`=== keep-check under ${word || 'no word'} ===`);
{ const W = await standUp({ seed: 7 }); walk(W); secs(W, 6);
  const s = W.snapshot(); stream(W); const A = secs(W, 10), startsA = W.game.rustHost?.starts ?? 0;
  W.restore(s); walk(W); stream(W); const B = secs(W, 10), H = W.game.rustHost;
  line(same(A, B) === 10, 'snapshot: ten seconds after a snapshot, then the snapshot put back and the same ten seconds', `equal seconds ${same(A, B)} of 10 · ${A[9]} | ${B[9]}`);
  if (H) line(H.starts === startsA && calls(W, /a snapshot put back: the kept game/) === 1, 'snapshot: the put-back was the kept Rust game standing again, with no new start', `starts ${H.starts} (was ${startsA}) · backs ${H.backs} · keeps ${H.keepsN}`); }
{ const W = await standUp({ seed: 7 }); walk(W); secs(W, 6);
  const r = W.game.saves.write(W.game, 'manual', 'by hand'); stream(W); const A = secs(W, 10), startsA = W.game.rustHost?.starts ?? 0;
  const why = r.ok ? W.game.saves.load(W.game, r.info.key) : r.why; walk(W); stream(W); const B = secs(W, 10), H = W.game.rustHost;
  line(r.ok && !why && same(A, B) === 10, 'controller: a save by hand, ten seconds, its load, the same ten seconds', `saved ${r.ok ? r.info.key : r.why} · load ${why ?? 'ok'} · equal seconds ${same(A, B)} of 10 · ${A[9]} | ${B[9]}`);
  if (H) line(H.starts === startsA && H.backs === 1, 'controller: the load was the kept Rust game standing again, with no new start', `starts ${H.starts} (was ${startsA}) · backs ${H.backs} · keeps ${H.keepsN} · in memory ${!!r.info?.mem}`); }
{ const Ws: any[] = []; for (let i = 0; i < 6; i++) { const W = await standUp({ seed: 11 }); walk(W); secs(W, 2); Ws.push(W); }
  const first = Ws[0], H = first.game.rustHost, B = secs(first, 5);
  const ref = await standUp({ seed: 11 }); walk(ref); secs(ref, 2); const A = secs(ref, 5);
  line(same(A, B) === 5, 'slots: the first of six games, stepped again after the others, goes on as a game played straight through', `equal seconds ${same(A, B)} of 5 · ${A[4]} | ${B[4]}`);
  if (H) line(H.resteps === 1 && H.starts === 1, 'slots: it stood again from its own kept bytes (no new start from the puppet)', `gave its slot up ${H.evicted} · stood again ${H.resteps} · starts ${H.starts}`);
  if (H) { // (seven more games push the second game's kept bytes out: the last six that gave a slot up keep theirs)
    for (let i = 0; i < 9; i++) { const W = await standUp({ seed: 11 }); secs(W, 1); }
    let threw = ''; try { Ws[1].step(DT); } catch (e) { threw = String((e as Error).message); }
    line(/re-step is refused/.test(threw) && /keep-check\.ts/.test(threw), 'refused: a game whose kept bytes were let go is not stepped again, and the refusal names the tool\'s line', threw.slice(0, 260) || 'it stepped'); } }
console.log(`=== keep-check: ${fails ? `${fails} FAIL` : 'ALL PASS'} ===`);
process.exit(fails ? 1 : 0);
