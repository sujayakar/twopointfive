// WHAT FALLS AND WHAT IT LEAVES, THE TYPESCRIPT AGAINST THE RUST MODULE — the landings the simulation schedules and the litter map of broken glass
// (src/sim/fallshome.ts FallStore; the row RUST.falls: rust/sim/src/falls.rs), every rule taken both ways from the same store over a long random run of
// them, in both of its lists of landings (glass, thrown pieces): schedule (a list filling to its cap and past it), deposit, the landings due as a clock
// runs, a footfall's step, the floor told or not, clear.
//   bun run tools/qa/falls-ident.ts [--n 200000] [--seed 1]
// Two ways of asking, at once:
//   both   FallStore's own road for the checks (FALLS.both): each rule taken in the module with nothing kept, then in TypeScript from the same doubles,
//          the two answers and every double of the two stores compared
//   twin   a second store that never goes near the module, handed the same rules in the same order: after every rule its answer and every one of its
//          doubles (FALL_LEN of them) must be the first store's (which runs the module's rules for real every other rule)
// No game is stood up: a store over a block of its own (as a game's is over the game's, from the moment it is made). Every so often the row is DROPPED
// for a while under the run (the store's rules are then this file's, over the same doubles of the block) and the block is PARKED (four other blocks take
// the module's slots: the store's next rule in the module binds it again). A run is FAIR only if it met landings due in each list, a full list, crunches,
// glass off the world's floor, a floor not told for its gap, a list cleared while the other held landings that then stayed, the row dropped, and the
// block parked and found bound again.
// FALLS_BREAK=1 (or RUST_BREAK=1) is the red path: the module's store is handed glass one unit in the last place heavier, and the run must FAIL.
import { resolve } from 'path';
const ROOT = resolve(import.meta.dir, '../..');
const SIM: any = await import(`${ROOT}/src/sim/sim.ts`), ST: any = await import(`${ROOT}/src/sim/state.ts`), FH: any = await import(`${ROOT}/src/sim/fallshome.ts`);
const argv = process.argv.slice(2); const flag = (n: string) => { const i = argv.indexOf(n); return i >= 0 ? argv[i + 1] : undefined; };
const N = Number(flag('--n') ?? 200000), SEED = Number(flag('--seed') ?? 1), BREAK = process.env.FALLS_BREAK === '1' || process.env.RUST_BREAK === '1';
let s32 = SEED >>> 0; const rnd = () => { s32 = (s32 + 0x6D2B79F5) >>> 0; let t = s32; t = Math.imul(t ^ (t >>> 15), t | 1); t ^= t + Math.imul(t ^ (t >>> 7), t | 61); return ((t ^ (t >>> 14)) >>> 0) / 4294967296; };
{ const no = SIM.simWhyNot(); if (no) { console.log(`=== falls-ident: the module does not run here (${no}) — FAIL ===`); process.exit(1); } }
const x = SIM.sim(), sizes = [x.falls_cap(0), x.falls_cap(1), x.falls_cap(2), x.falls_cap(3), x.falls_cap(5)];
const laid = sizes[0] === FH.FALL_CELLS_X && sizes[1] === FH.FALL_CELLS_Z && sizes[2] === FH.FALL_MAXP && sizes[3] === Object.keys(FH.FP).length && sizes[4] === FH.FALL_LISTS && ST.simLayout().parts.find((p: any) => p.name === 'falls')?.len === FH.FALL_LEN * 8;
// the store under test (a part of a block, as a game's is: its rules the module's while the row is on) and its twin (over another block, TypeScript only)
const st = new ST.SimState(), A = new FH.FallStore(st), B = new FH.FallStore(new ST.SimState()), crowd = Array.from({ length: ST.simLayout().games }, () => new ST.SimState());
SIM.RUST.falls = true;
const doubles = (S: any): Float64Array => S.home.d.slice(S.base, S.base + FH.FALL_LEN);
const here = <T,>(f: () => T): T => { const was = SIM.RUST.falls; SIM.RUST.falls = false; try { return f(); } finally { SIM.RUST.falls = was; } };   // (the twin's rules are this file's, whatever the row says)
const MET = { due: 0, duePieces: 0, dust: 0, heard: 0, unheard: 0, full: 0, crunch: 0, offFloor: 0, dropped: 0, parked: 0, boundAgain: 0, clears: 0, keptOther: 0 };
let wasParked = false;
const pick = () => rnd() < 0.7 ? FH.FALL_GLASS : FH.FALL_PIECES;
let differ = 0, first = '', time = 0;
const heavier = (a: number) => BREAK ? a * (1 + 2 ** -52) : a;
const anywhere = (): [number, number] => rnd() < 0.9 ? [rnd() * 6 + 10, rnd() * 6 + 8] : rnd() < 0.5 ? [rnd() * 44 - 2, rnd() * 32 - 2] : [rnd() < 0.5 ? -0.01 : 40, rnd() * 28];
for (let i = 0; i < N && differ < 5; i++) {
  FH.FALLS.both = i % 2 === 0;   // (every other rule the module's for real: the twin is then the only witness)
  const r = rnd(); let a: unknown, b: unknown, what = '';
  if (r < 0.3) { const [px, pz] = anywhere(), t = time + (rnd() < 0.3 ? 5 + rnd() * 20 : rnd() * 1.5), name = rnd() < 0.5 ? 'glassTinkle' : 'glassShatter', gain = rnd() < 0.4 ? 0 : rnd(), rate = 0.8 + rnd() * 0.6, amt = rnd() < 0.1 ? 0 : rnd() * 0.05, who = Math.floor(rnd() * 40), ev = Math.floor(rnd() * 3), lv = rnd() < 0.8 ? 0 : 0.2 + rnd() * 0.3, cap = rnd() < 0.9 ? 48 : rnd() < 0.5 ? FH.FALL_MAXP : 200, l = pick();
    const py = rnd() * 2, nm = l === FH.FALL_PIECES && rnd() < 0.7 ? '' : name; what = 'schedule'; const dust = rnd() < 0.2 ? rnd() * 2 : 0; a = A.schedule(t, px, py, pz, nm, gain, rate, heavier(amt), who, ev, lv, cap, l, dust); b = here(() => B.schedule(t, px, py, pz, nm, gain, rate, amt, who, ev, lv, cap, l, dust)); if (!a) MET.full++;
    if (px < 0 || px >= 40 || pz < 0 || pz >= 28) MET.offFloor++; }
  else if (r < 0.45) { const [px, pz] = anywhere(), amt = rnd() < 0.1 ? -1 : rnd() * 0.08; what = 'deposit'; A.deposit(px, pz, heavier(amt)); here(() => B.deposit(px, pz, amt)); }
  else if (r < 0.7) { time += rnd() < 0.7 ? 1 / 60 : rnd() * 0.6; const lv = 0.3, gap = rnd() < 0.8 ? 0.4 : 0, l = pick(); what = 'due'; a = A.due(time, lv, gap, l); b = here(() => B.due(time, lv, gap, l)); for (const d of a as any[]) { if (l === FH.FALL_PIECES) MET.duePieces++; else MET.due++; if (d.dust > 0) MET.dust++; if (d.heard) MET.heard++; else MET.unheard++; } }
  else if (r < 0.93) { const [px, pz] = anywhere(), lv = rnd() * 0.6, cf = rnd() < 0.9 ? 0.012 : 0.2; what = 'step'; a = A.step(px, pz, lv, cf, 0.9, 0.6, 0.12); b = here(() => B.step(px, pz, lv, cf, 0.9, 0.6, 0.12)); if ((a as any).k > 0) MET.crunch++; }
  else if (r < 0.97) { const gap = rnd() * 0.5, l = pick(); what = 'heard'; a = A.heard(time, gap, l); b = here(() => B.heard(time, gap, l)); }
  else if (r < 0.973) { const only = rnd() < 0.5, l = pick(), other = A.waitingIn(1 - l); what = 'clear'; A.clear(only, l); here(() => B.clear(only, l)); MET.clears++; if (other > 0 && A.waitingIn(1 - l) === other && A.waitingIn(l) === 0) MET.keptOther++; }
  else if (r < 0.987) { what = 'the row dropped'; SIM.RUST.falls = false; MET.dropped++; }   // (for a while: its rules are this file's, over the same doubles, until it is raised below)
  else { what = 'the block parked'; for (const c of crowd) c.bind(); a = st.where; b = -1; MET.parked++; wasParked = true; }   // (every slot is another block's: the store's doubles are read where they are parked)
  if (!SIM.RUST.falls && rnd() < 0.05) SIM.RUST.falls = true;
  if (wasParked && st.where >= 0) { wasParked = false; MET.boundAgain++; }   // (a rule taken in the module since brought it back)
  const da = doubles(A), db = doubles(B); let bad = JSON.stringify(a) === JSON.stringify(b) ? '' : `answers ${JSON.stringify(a)} | ${JSON.stringify(b)}`;
  for (let k = 0; k < FH.FALL_LEN && !bad; k++) if (!Object.is(da[k], db[k])) bad = `double ${k}: ${da[k]} in the store under test, ${db[k]} in its twin`;
  if (bad) { differ++; if (!first) first = `rule ${i} (${what}): ${bad}`; }
}
FH.FALLS.both = false; SIM.RUST.falls = false;
const F = FH.FALLS, thin = (Object.keys(MET) as (keyof typeof MET)[]).filter(k => MET[k] === 0);
const ok = laid && differ === 0 && F.differ === 0 && F.asked > N / 4 && F.there > N / 4 && thin.length === 0;
console.log(`falls-ident: ${N} rules · the layout the two sides name alike: ${laid} (${sizes.join(' × ')}, ${FH.FALL_LEN} doubles) · ${F.asked} answered both ways, ${F.differ} differ${F.first ? ` (${F.first})` : ''} · ${F.there} the module's alone, the twin ${differ ? `PARTS: ${first}` : 'the same after every rule'}`);
console.log(`    met: ${(Object.keys(MET) as (keyof typeof MET)[]).map(k => `${k} ${MET[k]}`).join(', ')}${thin.length && !BREAK ? ` — NOT A FAIR RUN: never met ${thin.join(', ')}` : ''}`);
console.log(`=== falls-ident: ${ok ? 'PASS' : 'FAIL'}${BREAK ? ' (the red path: FAIL is what it must say)' : ''} ===`);
process.exit(ok ? 0 : 1);
