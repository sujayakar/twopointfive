// ONE BLAST, ONE FLOOD (src/game/wallreact.ts WALLREACT.blastFlood): a blast that opens TWO walls files two rows, and the first was flooded with the second
// hole already in the floor — the fill ran on through it into a room the row never touched and stopped there as 'the same room', a room whose door list
// was still its old one; a defender's evidence was carried onto that short list and what he knew of every other door was dropped. With the rule the rows of
// one frame are flooded as one.
//
//   bun run tools/qa/blastflood-check.ts          a wall place where one charge opens two walls and three rooms become one (looked for on the compound, the
//                                                 block and the warehouse, by the blast's own plan and then by trying it), a defender either side, a
//                                                 different figure of evidence a door: every figure is still his after the blast
// A settled rule: nothing here writes it or asks the game without it. Until 2026-11-07 the stage was FOUND by the fault (the same blast with the rule off,
// where the figures are lost), two charges apart were laid against a flood a row, and there were lines for the stand, a recording from before the key and
// the page's off flag. BLASTFLOOD_BREAK was the rule never switched on: it is gone with them, and the check has no red path of its own now.
import { ROOT } from './headless.ts';
import { rig, spots, blow } from './wallreact-probes.ts';
const { v3 } = await import(`${ROOT}/src/math/vec.ts`);
const { knobsCopy, knobsBack } = await import(`${ROOT}/src/game/knobs.ts`);
type Vec3 = [number, number, number];
const flat = (a: number[], b: number[]) => Math.hypot(a[0] - b[0], a[2] - b[2]), f1 = (x: number) => x.toFixed(1);

/** the stage on place `k` of a level's two-wall places: what the defender on the `a` side knew before, and what he knows after */
async function stage(level: string, k: number) {
  const R = await rig(level, 6); const g = R.g, Fb = R.F, nav = g.col.nav;
  const two = spots(R).filter(s => { const ra = Fb.roomAt(s.a), rb = Fb.roomAt(s.b); if (!ra || !rb || ra === rb) return false;
    return g.destr.wallsFor([s.mid[0] - s.n[0] * 0.3, 1.0, s.mid[2] - s.n[2] * 0.3] as Vec3).some((o: any) => o.kind === 'wall' && o.id !== s.id); });   // (the blast's own plan: this wall and another)
  const S = two[k], defs = R.defs(); if (!S || defs.length < 2) return { places: two.length, got: null };
  const [D1, D2] = [...defs].sort((x, y) => flat(x.gd.char.pos, S.mid) - flat(y.gd.char.pos, S.mid));
  const pa = v3.mad(v3.mad(S.mid, S.n, 2.4), S.u, 2.2) as Vec3, pb = v3.mad(v3.mad(S.mid, S.n, -2.4), S.u, -2.2) as Vec3;
  const inRoom = (c: Vec3[], r: any) => c.find(p => !nav.isBlocked(p[0], p[2]) && Fb.roomAt(p) === r) ?? c[c.length - 1];
  const p1 = inRoom([pa, v3.mad(S.mid, S.n, 2.4) as Vec3, S.a], Fb.roomAt(S.a)), p2 = inRoom([pb, v3.mad(S.mid, S.n, -2.4) as Vec3, S.b], Fb.roomAt(S.b));
  R.post(D1, p1, S.a); R.post(D2, p2, S.b); R.step(0.5);
  D1.ev = D1.room.entries.map((_: any, i: number) => 0.1 * (i + 1));   // (a different figure a door)
  const before = new Map<any, number>(D1.room.entries.map((e: any, i: number) => [e.door, D1.ev[i]]));
  const b0 = R.wr().breaches.length, res = blow(R, S); R.step(0.1);
  const E = D1.room.entries, kept = [...before].filter(([d, v]) => { const i = E.findIndex((e: any) => e.door === d); return i >= 0 && (D1.ev[i] ?? NaN) >= v - 0.05; }).length;   // (at least what he had: the bang is heard through doors and adds its own)
  return { places: two.length, got: { at: S.mid, how: `charge at (${(S.mid[0] - S.n[0] * 0.3).toFixed(2)}, ${(S.mid[2] - S.n[2] * 0.3).toFixed(2)}) · ${D1.gd.callsign} at (${p1[0].toFixed(2)}, ${p1[2].toFixed(2)}) facing (${S.a[0].toFixed(2)}, ${S.a[2].toFixed(2)}) · ${D2.gd.callsign} at (${p2[0].toFixed(2)}, ${p2[2].toFixed(2)}) facing (${S.b[0].toFixed(2)}, ${S.b[2].toFixed(2)})`, wall: S.id, walls: res.walls, breaches: R.wr().breaches.length - b0, doors0: before.size, doors: E.length, kept, same: D1.room === D2.room && D1.room === Fb.roomAt(S.a) && D1.room === Fb.roomAt(S.b), evLen: D1.ev.length === E.length && D2.ev.length === D2.room.entries.length, who: D1.gd.callsign,
    after: E.map((e: any, i: number) => `${e.door.def.name}=${f1(D1.ev[i] ?? NaN)}`).join(' '), was: [...before].map(([d, v]: any) => `${d.def.name}=${f1(v)}`).join(' ') } };
}

/** two charges in ONE frame at two places with nothing between them — each opens one wall between two rooms of its own: which rooms are one afterwards, as
 *  a signature over every wall place's two sides (a flood of the two together must come out as a flood of each) */
async function apart(level: string) {
  const R = await rig(level, 6); const g = R.g, Fb = R.F, all = spots(R);
  const lone = all.filter(s => (s.rank ?? 0) >= 1 && Fb.roomAt(s.a) && Fb.roomAt(s.b) && Fb.roomAt(s.a) !== Fb.roomAt(s.b)), S1 = lone[0];
  const S2 = S1 ? lone.filter(s => flat(s.mid, S1.mid) >= 6 && new Set([Fb.roomAt(S1.a), Fb.roomAt(S1.b), Fb.roomAt(s.a), Fb.roomAt(s.b)]).size === 4).sort((x, y) => flat(y.mid, S1.mid) - flat(x.mid, S1.mid))[0] : undefined;
  if (!S1 || !S2) return null;
  const pts = all.flatMap(s => [s.a, s.b]); blow(R, S1); blow(R, S2); R.step(0.1); void g;
  const ids = new Map<any, number>(), sig = pts.map(p => { const r = Fb.roomAt(p); if (!r) return '-'; if (!ids.has(r)) ids.set(r, ids.size); return ids.get(r); }).join('');
  return { level, sig, one1: Fb.roomAt(S1.a) === Fb.roomAt(S1.b), one2: Fb.roomAt(S2.a) === Fb.roomAt(S2.b), two: Fb.roomAt(S1.a) !== Fb.roomAt(S2.a), far: flat(S1.mid, S2.mid), rooms: ids.size };
}

export async function blastfloodCheck(): Promise<boolean> {
  console.log(`\n=== blastflood: a blast that opens two walls floods the rooms once — a defender keeps what he knew of every door ===`);
  let all = true; const verdict = (ok: boolean, label: string, detail: string) => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); };
  const knobs0 = knobsCopy(), rnd0 = Math.random;
  const log = console.log, quiet = async <T>(fn: () => Promise<T>): Promise<T> => { console.log = () => { }; try { return await fn(); } finally { console.log = log; } };
  try {
    // THE STAGE: a place where one charge opens two walls beside a defender who knows two doors or more, looked for by trying the two-wall places in turn
    let pick: { level: string; k: number; got: NonNullable<Awaited<ReturnType<typeof stage>>['got']> } | null = null, tried = 0; const seen: string[] = [];
    for (const level of ['compound', 'block', 'warehouse']) { if (pick) break;
      for (let k = 0; k < 6 && !pick; k++) { const r = await quiet(() => stage(level, k)); if (!r.got) { seen.push(`${level}: ${r.places} two-wall places`); break; } tried++;
        if (r.got.walls >= 2 && r.got.doors0 >= 2) pick = { level, k, got: r.got }; } }
    verdict(!!pick, 'the staging: a wall place where one charge opens two walls beside a defender who knows two doors or more', pick ? `${pick.level}, wall #${pick.got.wall} at (${f1(pick.got.at[0])}, ${f1(pick.got.at[2])}) · ${pick.got.how} · walls opened ${pick.got.walls}, breaches ${pick.got.breaches} · ${pick.got.who} knew ${pick.got.doors0} doors · ${tried} places tried` : `no such place in ${tried} tried (${seen.join(' · ') || 'three floors'})`);
    if (pick) { const G = pick.got;
      verdict(G.walls >= 2 && G.same && G.evLen, 'the defenders either side hold the SAME room and each has a figure a door', `walls opened ${G.walls}, breaches ${G.breaches} · the same room ${G.same} · evidence as long as the doors ${G.evLen} (${G.doors} doors)`);
      verdict(G.kept === G.doors0, 'what he knew of every door is still his, door by door', `${G.who}: ${G.kept} of ${G.doors0} kept · before ${G.was} · after ${G.after}`); }
    // TWO BLASTS THAT HAVE NOTHING TO DO WITH EACH OTHER, in one frame: flooded together, nothing is joined that stood apart
    { type Apart = Awaited<ReturnType<typeof apart>>; let A = null as Apart; for (const level of ['block', 'warehouse', 'compound']) { A = await quiet(() => apart(level)); if (A) break; }
      verdict(!!A && A.one1 && A.one2 && A.two, 'two charges in one frame at two places apart, one wall each: each pair of rooms is one, and the two pairs are not each other', A ? `${A.level}, ${f1(A.far)} m apart · first pair one ${A.one1} · second pair one ${A.one2} · the two apart ${A.two} · rooms over the floor's wall places ${A.rooms}` : 'no two lone-wall places 6 m apart between four rooms on any of the three floors'); }
  } finally { console.log = log; knobsBack(knobs0); Math.random = rnd0; }
  console.log(`=== blastflood: ${all ? 'ALL PASS' : 'FAILURES above'} ===`);
  return all;
}
if (import.meta.main) process.exit((await blastfloodCheck()) ? 0 : 1);
