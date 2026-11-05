// THE LEAVES OFF THEIR HINGES, AS THEY ALWAYS FLEW — src/game/doorblast.ts's own step with every row of RUST off, held to the words it folded before a
// slab's numbers became a row of columns (2026-11-03; a slab was an object on its door until then): every wooden leaf of four levels torn out from
// either face, taken apart by gunfire, flown by the game's own entry (doorSlabs), filed as a checkpoint would file it and hung again, three thousand
// ticks a level; every number of every slab, every box it is posed as, every noise, the tallies and the log folded after each tick.
//   bun run tools/qa/slab-asitwas.ts                         the shipped stand
//   TPS_PROFILE=frozen bun run tools/qa/slab-asitwas.ts      the frozen one (its own words: the two stands differ in rules the slabs meet)
// The per-tick bits of a mission (game/digest.ts rawBits) fold a door's angle and nothing of its slab, so a mission played both ways says little of this
// step; tools/qa/slab-ident.ts holds the TypeScript to the Rust, and THIS holds the TypeScript to what it was. A change to the rule changes the words:
// say so where the change is made, and write the new ones here.
// THE WORDS ARE OF THE LEVELS AS THEY STOOD: a level is read by the git blob id of its file at the recording (`git rev-parse HEAD:levels/<name>`), so a
// level dressed or rebuilt since leaves this check alone. A new word is written with the blob it was folded over. The office is the game's own level
// (no file is handed to the stand for it), so its word does follow that level.
const STAND = process.env.TPS_PROFILE === 'frozen' ? 'frozen' : 'shipped';
process.env.TPS_PROFILE = STAND; process.env.TPS_CPUSENSE = '1';
for (const k of Object.keys(process.env)) if (/^TPS_RUST/.test(k) || k === 'TPS_EXACT') delete process.env[k];   // (the words are a stand's own: its maths, no row of RUST)
import { resolve } from 'path';
import { spawnSync } from 'node:child_process';
const ROOT = resolve(import.meta.dir, '../..');
const WAS: Record<string, { frozen: string; shipped: string; blob?: string }> = {
  'office.json': { frozen: 'qs3329', shipped: '37swmc' },
  'compound.json': { frozen: '1cr6h0h', shipped: '7hpfku', blob: 'cb36d7328bb8aa5ac67a996101ff4cea6668cdff' },
  'consulate-gwot.json': { frozen: 'qhlgkd', shipped: '62n6ih', blob: 'f86faaa599c67dfdc066cccc286775e924aabcb6' },
  'mousehole.json': { frozen: '23j3x9', shipped: '84y77m', blob: '49d62b2968cfdf16d1b5856095c0aadd3cf4eee0' },
};
let bad = 0;
const { standUp } = await import(`${ROOT}/tools/qa/headless.ts`);
const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
const DB: any = await import(`${ROOT}/src/game/doorblast.ts`);
const log = console.log, err = console.error, hush = (on: boolean) => { console.log = on ? () => {} : log; console.error = on ? () => {} : err; };
let s32 = 7; const rnd = () => { s32 = (s32 + 0x6D2B79F5) >>> 0; let t = s32; t = Math.imul(t ^ (t >>> 15), t | 1); t ^= t + Math.imul(t ^ (t >>> 7), t | 61); return ((t ^ (t >>> 14)) >>> 0) / 4294967296; };
const F = new Float64Array(1), U = new Uint32Array(F.buffer);
for (const file of Object.keys(WAS)) {
  const was = WAS[file], want = was[STAND]; let data: unknown = null;
  if (was.blob) {
    const got = spawnSync('git', ['-C', ROOT, 'cat-file', 'blob', was.blob], { encoding: 'utf8', maxBuffer: 1 << 28 });
    // (a clone that does not hold the bytes holds nothing of this level: red, never 'nothing to compare')
    if (got.status !== 0) { bad++; log(`${file.padEnd(22)} FAIL — this clone does not hold the blob ${was.blob.slice(0, 9)} its word was folded over (a shallow clone?)`); continue; }
    data = JSON.parse(got.stdout);
  }
  hush(true); const W: any = await standUp({ uploadEvery: 1000, ...(data ? { level: loadLevel(data) } : {}) }); hush(false);
  const g = W.game, DL: any[] = g.doors.list, wood = DL.filter(d => !d.def.steel); let h = 0x811c9dc5 >>> 0, n = 0, snaps = 0, torn = 0, landed = 0;
  const num = (x: number) => { F[0] = x; h = Math.imul(h ^ U[0], 0x01000193) >>> 0; h = Math.imul(h ^ U[1], 0x01000193) >>> 0; n++; };
  const str = (s: string) => { for (let i = 0; i < s.length; i++) h = Math.imul(h ^ s.charCodeAt(i), 0x01000193) >>> 0; };
  (g.doors as any).emit = (e: any) => { str(e.sound); num(DL.indexOf(e.door)); num(e.pos[0]); num(e.pos[1]); num(e.pos[2]); num(e.who); num(e.level); if (e.sound === 'bang') landed++; };
  DB.DOORBLAST.on = true; DB.DOORSHOT.on = true;
  const fold = () => { for (const d of DL) { const S = d.slab; num(d.off ? 1 : 0); num(d.angle); if (S) { for (let k = 0; k < 3; k++) { num(S.p[k]); num(S.v[k]); } num(S.a); num(S.spin); num(S.tip); num(S.tipV); num(S.side); num(S.rest ? 1 : 0); num(S.landed ? 1 : 0); num(S.who); num(S.t); num(S.ang); if (S.bits) for (const b of S.bits) num(b); }
      for (const b of [d.box, ...d.handleBoxes]) { for (let k = 0; k < 3; k++) { num(b.c[k]); num(b.h[k]); } num(b.yaw); num(b.flags); if (b.rot) for (const q of b.rot) num(q); num(b.marks === undefined ? 0 : 1); } }
    const A = DB.DOORBLAST_STATS; for (const k of Object.keys(A)) num(A[k]); str(DB.DOORBLAST_LOG[DB.DOORBLAST_LOG.length - 1] ?? ''); num(DB.DOORBLAST_LOG.length); };
  for (let r = 0; r < 3000; r++) {
    const DT = rnd() < 0.85 ? 1 / 60 : [1 / 30, 1 / 20, 1 / 120][Math.floor(rnd() * 3)];
    hush(true); g.col.tickOpen();
    try { g.sim?.settle?.(g);
      const k = rnd(), hung = wood.filter(d => !d.off), off = wood.length - hung.length;
      if (k < 0.08 && hung.length) { const d = hung[Math.floor(rnd() * hung.length)], [hx, hz] = d.def.hinge, a = d.def.closedDir + d.angle, s = rnd() < 0.5 ? 1 : -1, t = rnd() * d.def.width, o = 0.2 + rnd() * 1.2; if (DB.tearOff(g, d, [hx + Math.cos(a) * t - Math.sin(a) * o * s, 0.2 + rnd(), hz + Math.sin(a) * t + Math.cos(a) * o * s], 10, rnd())) torn++; }
      else if (k < 0.1 && hung.length) { const d = hung[Math.floor(rnd() * hung.length)], a = rnd() * 6.283; DB.splinter(g, d, [Math.cos(a), 0, Math.sin(a)], 10, [d.def.hinge[0], 1, d.def.hinge[1]]); }
      else if (k < 0.14) { const hd = DL.filter(d => !d.off); if (hd.length) { const d = hd[Math.floor(rnd() * hd.length)]; d.angle = (rnd() - 0.5) * 3; d.vel = 0; d.latched = false; d.place(); } }
      else if (k < 0.15) { for (const d of DL) if (d.off) { const s = DB.slabSnapshot(g, d); if (s) { snaps++; str(JSON.stringify(s)); } } }   // (what a checkpoint would file)
      else if (off && (k < 0.155 || off > wood.length * 0.7)) { for (const d of DL) if (d.off) { d.reset(); d.place(); } }
      DB.doorSlabs(g, DT); fold();
    } finally { g.col.tickShut(); hush(false); }
    g.time += DT;
  }
  const word = h.toString(36), ok = word === want && torn > 100 && landed > 20 && snaps > 20; if (!ok) bad++;
  log(`${file.padEnd(22)} ${ok ? 'PASS' : 'FAIL'} — ${wood.length} leaves, ${torn} torn, ${landed} slams, ${snaps} filed, ${n} numbers folded → ${word}${word === want ? '' : ` (it was ${want})`}`);
}
log(`=== slab-asitwas (the ${STAND} stand): ${bad ? 'FAIL' : 'PASS'} ===`);
process.exit(bad ? 1 : 0);
