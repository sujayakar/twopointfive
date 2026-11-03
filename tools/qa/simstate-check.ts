// A GAME'S STATE IN THE RUST MODULE (src/sim/state.ts SimState, rust/sim/src/state.rs; docs/agents/rust.md "State in the module is state of the game"): is a
// block the same state wherever it stands, does the snapshot carry it, and is the first system at home in it (the game's own die, RUST.dice) the die it was?
//   bun run tools/qa/simstate-check.ts                      every line below; exit 0 = all pass, 1 = one failed (named)
//   TPS_PROFILE=shipped bun run tools/qa/simstate-check.ts  the same on the shipped stand
//   SIMSTATE_BREAK=1 bun run tools/qa/simstate-check.ts     the red path: a block parked to make room is left behind (zeros come back), and a take put back
//                                                           writes nothing — `slots`, `take and put`, `snapshot` and `another game` must FAIL
// WHAT IS HELD:
//   layout     the module's parts are the ones src/sim/state.ts names, end to end, with a layout word
//   dice       a die in a block rolled by the module gives mulberry32's own numbers, draw for draw, and counts its draws (seeds × draws below)
//   slots      MORE games than the module has slots, their dice rolled turn about: every block that gives way is parked and brought back, and each die
//              still gives its own seed's numbers — a block is the same state in any slot and out of the module
//   take and put  a take, some draws, the take put back: the next draws are the ones that followed the take; a take since a take that nothing changed after
//              IS that take (nothing copied); a take put into ANOTHER block gives the same draws there
//   row        a seeded game played with RUST.dice on is the game with it off after every tick (the raw bits), with the die at home in the block all the
//              way; and the row raised and dropped under a RUNNING game (every 40 ticks) is the same game again
//   snapshot   with the row on: a snapshot, a second of play, the snapshot put back, the second again — the state's word after every tick is what it was
//              the first time, and the die stands where the take left it
//   another game  …and the same snapshot put into a FRESH game (its own block, its own slot) plays that second to the same raw bits
//   across a row change  a snapshot holds WHERE the die was: one taken before the row moved it into the block and one taken after are each put back over
//              a game that has since gone the other way, and play on as they did
//   no views   a typed array over the module's memory kept in the state is named by the snapshot (`simViews`): state is reached through `game.sim` alone
// Not game code; nothing here needs a GPU, and nothing here has been seen on one.
import { standUp, ROOT } from './headless.ts';
const S: any = await import(`${ROOT}/src/sim/sim.ts`), ST: any = await import(`${ROOT}/src/sim/state.ts`), D: any = await import(`${ROOT}/src/math/dice.ts`);
const RB: any = await import(`${ROOT}/src/game/rollback.ts`), DG: any = await import(`${ROOT}/src/game/digest.ts`);
const BREAK = process.env.SIMSTATE_BREAK === '1', DT = 1 / 60;
let fails = 0;
const verdict = (ok: boolean, label: string, detail: string) => { if (!ok) fails++; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); };
const F = new Float64Array(1), U = new Uint32Array(F.buffer), same = (a: number, b: number): boolean => { F[0] = a; const l = U[0], h = U[1]; F[0] = b; return l === U[0] && h === U[1]; };

const no = S.simWhyNot();
if (no) { console.log(`- FAIL — the module does not start here: ${no}`); process.exit(1); }
console.log(`state in the Rust module · profile ${process.env.TPS_PROFILE ?? 'frozen'}${BREAK ? ' · SIMSTATE_BREAK=1 (the red path)' : ''}`);
if (BREAK) {
  const P = ST.SimState.prototype, put = P.put;
  P.park = function (this: any) { this.own = new ArrayBuffer(this.w.length * 4); this.slot = -1; this.view(); };   // (what stood in the slot is left there: zeros come back)
  P.put = function (this: any, c: unknown[]) { if (c.length === 0) put.call(this, c); };   // (a take put back writes nothing)
}
const x = S.sim(), L = ST.simLayout();

// ---- layout
{ let at = 0, tight = true; for (const p of L.parts) { if (p.off !== at || p.len % 8) tight = false; at += p.len; }
  let trap = false; try { x.dice_roll(L.games); } catch { trap = true; }   // (a slot that is not one: the module traps, it does not answer for slot 0)
  const none = new ST.SimState(); S.RUST.dice = true; none.settle({ dice: null }); S.RUST.dice = false;   // (a game with no die of its own has nothing to move: no block)
  verdict(L.parts.length === ST.SIM_PARTS.length && tight && at === L.bytes && L.word !== 0 && L.games >= 2 && trap && !none.open, 'layout: the module\'s parts are the ones src/sim/state.ts names, end to end; a slot that is not one traps; a game with no die gets no block',
    `${L.parts.map((p: any) => `${p.name} ${p.off}+${p.len}`).join(' · ')} · ${L.bytes} bytes a block · ${L.games} slots · layout ${L.word.toString(16)} (state.ts names ${ST.SIM_PARTS.join(' ')}) · slot ${L.games} traps: ${trap} · no die, no block: ${!none.open}`); }

/** a die at home in a block of its own (as a game's is once its tick has opened with the row on) */
function homed(seed: number): any { const d = D.newDice(seed), st = new ST.SimState(), g = { dice: d }; S.RUST.dice = true; st.settle(g); S.RUST.dice = false; return { d, st, g }; }
const draw = (h: any): number => x.dice_roll(h.st.bind());

// ---- dice
{ const SEEDS = 4000, DRAWS = 250; let differ = 0, first = '', counts = 0;
  for (let k = 0; k < SEEDS; k++) { const seed = k < 8 ? [0, 1, -1, 0x7fffffff, 0x80000000 | 0, 0x6D2B79F5, 0xffffffff | 0, 42][k] : Math.imul(k, 0x9E3779B1) ^ (k << 7), h = homed(seed), own = D.mulberry32(seed);
    for (let i = 0; i < DRAWS; i++) { const a = draw(h), b = own(); if (!same(a, b)) { if (!differ) first = `seed ${seed} draw ${i}: ${a} | ${b}`; differ++; } }
    if (h.d.n !== DRAWS) counts++; h.st.settle(h.g); }   // (the row is off again: the die moves out, and its block is given up)
  verdict(differ === 0 && counts === 0, 'dice: a die in a block, rolled by the module, gives mulberry32\'s numbers draw for draw and counts them', `${SEEDS} seeds × ${DRAWS} draws · ${differ} differ · ${counts} counts off${first ? ' · first: ' + first : ''}`); }

// ---- slots
{ const N = L.games * 2 + 1, hs = Array.from({ length: N }, (_, i) => homed(1000 + i)), owns = hs.map((_, i) => D.mulberry32(1000 + i)); let differ = 0; const t0 = { ...ST.SIM_STATE_TALLY };
  for (let round = 0; round < 300; round++) for (let i = 0; i < N; i++) { const k = (i * 7 + round * 3) % N; for (let j = 0; j <= (round + k) % 3; j++) if (!same(draw(hs[k]), owns[k]())) differ++; }
  const parked = ST.SIM_STATE_TALLY.parked - t0.parked, reads = hs.every((h: any, i: number) => { const t = { dice: (1000 + i) | 0 }; for (let k = 0; k < h.d.n; k++) D.roll(t); return h.d.n > 0 && h.d.s === t.dice; });   // (the word a TypeScript die on the same seed stands on after as many draws)
  verdict(differ === 0 && parked > N && reads, 'slots: more games than the module has slots, rolled turn about, each keep their own die through every park and return', `${N} blocks over ${L.games} slots · ${parked} parked and brought back · ${differ} draws differ · read from TypeScript wherever they stand: ${reads}`);
  for (const h of hs) h.st.settle(h.g); }

// ---- take and put
{ const h = homed(77), t = h.st.take(); const a: number[] = []; for (let i = 0; i < 20; i++) a.push(draw(h));
  const t2 = h.st.take(t), t3 = h.st.take(t2); h.st.put(t); let back = 0; for (let i = 0; i < 20; i++) if (!same(draw(h), a[i])) back++;
  const o = homed(5); o.st.put(t); let there = 0; for (let i = 0; i < 20; i++) if (!same(draw(o), a[i])) there++;
  verdict(back === 0 && there === 0 && t2 !== t && t3 === t2 && h.d.n === 20, 'take and put: a take put back gives the draws that followed it, here and in another block; a take of a block nothing has changed is the take before',
    `${back} of 20 differ after the put back · ${there} of 20 in the other block · a changed block is a new take: ${t2 !== t} · an unchanged one is the same take: ${t3 === t2}`);
  h.st.settle(h.g); o.st.settle(o.g); }

// ---- the game: the row on is the row off; raised and dropped under a running game
process.env.TPS_SEED = '7';
const TICKS = 420;
/** frame `f` of a fight that rolls: Sam, unkillable, put in front of the first man of the shift at frame 30; he draws, and fires from a second on */
function drive(W: any, f: number): void {
  const g = W.game, at = 30;
  if (f === at) { g.godMode = true; const gd = g.guards[0], yaw = gd.char.bodyYaw; g.teleportPlayer([gd.char.pos[0] + Math.sin(yaw) * 3.5, 0, gd.char.pos[2] + Math.cos(yaw) * 3.5]); W.input.pressed.add('KeyE'); }
  const on = f >= at + 60; if (on && (f - at) % 10 === 0) W.input.clicked |= 1; W.input.buttons = on ? W.input.buttons | 1 : W.input.buttons & ~1;
}
async function play(row: (f: number) => boolean): Promise<{ words: number[]; homed: number; s: number; n: number; W: any }> {
  S.RUST.dice = false; const W: any = await standUp({ seed: 7 }), g = W.game; let word = DG.RAW_BITS_SEED, at = 0; const words: number[] = [];
  for (let f = 0; f < TICKS; f++) { S.RUST.dice = row(f); drive(W, f);
    W.step(DT); word = DG.rawBits(g, word); words.push(word); if (g.dice.home) at++; }
  S.RUST.dice = false;
  return { words, homed: at, s: g.dice.s, n: g.dice.n, W };
}
const firstOff = (a: number[], b: number[]): number => { for (let i = 0; i < a.length; i++) if (a[i] !== b[i]) return i; return -1; };
{ const off = await play(() => false), on = await play(() => true), flip = await play(f => (f / 40 | 0) % 2 === 0);
  const a = firstOff(off.words, on.words), b = firstOff(off.words, flip.words);
  verdict(a < 0 && b < 0 && on.homed === TICKS && off.homed === 0 && flip.homed > 0 && flip.homed < TICKS && on.s === off.s && flip.s === off.s && on.n === off.n && flip.n === off.n && off.n > 0,
    'row: a seeded game with its die in the module is the game without, after every tick — and with the row raised and dropped under it as it runs',
    `${TICKS} ticks, ${off.n} draws · on: ${a < 0 ? 'the same bits' : 'parts at tick ' + a}, the die at home for ${on.homed} ticks (off: ${off.homed}) · flipped every 40: ${b < 0 ? 'the same bits' : 'parts at tick ' + b}, at home for ${flip.homed} · the die ends on ${on.s} / ${off.s} / ${flip.s}`); }

// ---- snapshot, another game, no views
{ const W: any = await standUp({ seed: 7 }), g = W.game; S.RUST.dice = true;   // (a stand-up puts every row where its stand has it: raised after)
  let f = 0; for (; f < 200; f++) { drive(W, f); W.step(DT); }
  const snap = RB.takeSnapshot(g), s0 = g.dice.s, n0 = g.dice.n, h0 = RB.stateHash(g);
  const first: string[] = [], bits: number[] = []; let word = DG.RAW_BITS_SEED;
  const keys = { buttons: W.input.buttons };
  for (let k = 0; k < 60; k++) { drive(W, f + k); W.step(DT); first.push(RB.stateHash(g)); word = DG.rawBits(g, word); bits.push(word); }
  const moved = g.dice.n - n0;
  RB.restoreSnapshot(snap); const stood = g.dice.s === s0 && g.dice.n === n0 && RB.stateHash(g) === h0 && !!g.dice.home;
  W.input.buttons = keys.buttons;
  let part = -1; for (let k = 0; k < 60; k++) { drive(W, f + k); W.step(DT); if (part < 0 && RB.stateHash(g) !== first[k]) part = k; }
  verdict(stood && part < 0 && moved > 0 && snap.simViews === 0, 'snapshot: with the die in the module, a snapshot put back is the state it was, and the second after it plays as it did the first time',
    `the die back where the take left it and the state\'s word the same: ${stood} · ${moved} draws in the second · ${part < 0 ? '60 ticks, the same word after each' : 'the word parts at tick ' + part} · views over the module\'s memory in the state: ${snap.simViews}`);
  // another game: stood up on the same seed, stepped once (what a game hangs on its host at its first step is then its own), handed the snapshot
  RB.restoreSnapshot(snap);
  const W2: any = await standUp({ seed: 7 }), g2 = W2.game; S.RUST.dice = true; W2.step(DT);
  RB.restoreSnapshot(snap, g2); W2.input.buttons = keys.buttons;
  const own = g2.sim !== g.sim && g2.dice.home.st === g2.sim && g2.dice.s === s0 && g2.dice.n === n0; let w2 = DG.RAW_BITS_SEED, part2 = -1;
  for (let k = 0; k < 60; k++) { drive(W2, f + k); W2.step(DT); w2 = DG.rawBits(g2, w2); if (part2 < 0 && w2 !== bits[k]) part2 = k; }
  verdict(own && part2 < 0, 'another game: the snapshot put into a fresh game stands in that game\'s own block and plays the same second', `its own block, the die where the take left it: ${own} · ${part2 < 0 ? '60 ticks, the same raw bits after each' : 'the raw bits part at tick ' + part2}`);
  // across a row change: a snapshot taken with the die in its TypeScript fields, put back after the row has moved it into the block, and the other way round
  { S.RUST.dice = false; const W3: any = await standUp({ seed: 7 }), g3 = W3.game; let f3 = 0; const go = (n: number): string[] => { const ws: string[] = []; for (let k = 0; k < n; k++) { drive(W3, f3++); W3.step(DT); ws.push(RB.stateHash(g3)); } return ws; };
    go(150); const out = RB.takeSnapshot(g3), fOut = f3, kOut = W3.input.buttons, sOut = g3.dice.s; const refOut = go(40);
    S.RUST.dice = true; go(40); const inn = RB.takeSnapshot(g3), fIn = f3, kIn = W3.input.buttons, sIn = g3.dice.s; const refIn = go(40), homeIn = !!g3.dice.home;
    RB.restoreSnapshot(out); f3 = fOut; W3.input.buttons = kOut; S.RUST.dice = false; const a = !g3.dice.home && !g3.sim.open && g3.dice.s === sOut, againOut = go(40);
    RB.restoreSnapshot(inn); f3 = fIn; W3.input.buttons = kIn; S.RUST.dice = true; const b = !!g3.dice.home && g3.dice.home.st === g3.sim && g3.dice.s === sIn, againIn = go(40);
    const same = (x: string[], y: string[]) => x.every((w, i) => w === y[i]);
    verdict(a && b && homeIn && same(refOut, againOut) && same(refIn, againIn), 'across a row change: a snapshot from before the die moved in, and one from after, each put back under the row it was taken under, play as they did',
      `taken out of the block, put back over a game with a block: the die in its fields, no block left: ${a}, 40 ticks the same word: ${same(refOut, againOut)} · taken in the block, put back over a game without: at home again: ${b}, 40 ticks the same word: ${same(refIn, againIn)}`); }
  // a view over the module's memory kept in the state
  g.strayView = new Uint32Array(x.memory.buffer, x.state_addr(0), 4); const s3 = RB.takeSnapshot(g), s4 = RB.takeSnapshot(g, s3); delete g.strayView;
  let refused = false; try { RB.restoreSnapshot(s3); } catch { refused = true; }
  verdict(s3.simViews === 1 && s4.simViews === 1 && refused, 'no views: a typed array over the module\'s memory kept in the state is named by the snapshot, by one taken since it too, and such a snapshot is not put back',
    `named: ${s3.simViews} (one was planted) · by the take since: ${s4.simViews} · the put back refused: ${refused}`);
  S.RUST.dice = false; }

console.log(fails ? `\n${fails} FAILED` : '\nall pass');
process.exit(fails ? 1 : 0);
