// Three rules for a room cleared from outside (all three settled on; this file writes none of them since 2026-11-07), from the maintainer's tape of 10-18
// (training 5, the kitchen's door: the pair held a diagonal each for three seconds, said 'holding on the door' with a defender stood square in front of
// it — in the wedge neither diagonal shows — and both were shot stepping in; "men should try to pie from the outside as much as they can before considering
// their job done … I think we could add some element of leaning to the AI"):
//   DOORTOOLS.thrPie    both jamb men CUT THE OPENING toward its middle, a few degrees a step, until the outside shows no more floor; only then is the door
//                       quiet and 'holding on the door' said
//   DOORTOOLS.thrLean   the man cutting LEANS — head and gun past the edge, hips short of the station — and takes the cuts that look down the door's own
//                       line on a knee (teammates only)
//   DOORTOOLS.thrPieOne ONE man cuts (the maintainer, 10-19, of the two cutting and kneeling in step: "really only one guy should be doing it. one guy holds
//                       one angle while the other pies and then does the lean? … just do the lean, not the crouch"): the lead walks the cuts square, the
//                       far jamb's man stays on his diagonal, the lead leans once when his cuts are done, nobody kneels
//   bun run tools/qa/thrpie-check.ts            (probes.ts thrpie; THRPIE_BREAK=1 must FAIL — the man of the bones line is never asked to lean;
//                                                THRPIEONE_BREAK=1 must FAIL — the order is never given, so nobody cuts)
// OUT 2026-11-07, each the other side of a settled switch: (1) the rule off, the build as it was; (2) both jamb men cutting (thrPieOne off), and its frame
// line and apart readings (asked now of the one-man runs); the lean on every cut and the last cut on a knee (thrPieOne off), and the word given to a man on
// a knee; the comparisons with the slice off and with the lean off in the one-man lines; (4) the recording from before the keys and the page's off flags.
// Headless, on the tape's own level and door, the order given as the menu's row gives it ('clear from outside' = thresholdHold). The defender is the level's
// own man in that room, where the level stands him. Nothing here says how any of it LOOKS: the lean is measured off the bones, never seen on a GPU.
import { ROOT } from './headless.ts';
import { yardRow } from './yards.ts';
import { stage, quiet, guarded, family } from './thresholdmenu-probes.ts';
const { DOORTOOLS, doorPt, alongOf, offOf } = await import(`${ROOT}/src/game/doortools.ts`);
const { knobsCopy, knobsBack } = await import(`${ROOT}/src/game/knobs.ts`) as any;

const BREAK = !!process.env.THRPIE_BREAK, BREAK1 = !!process.env.THRPIEONE_BREAK;
const LEVEL = 'train-orders', DOOR = 'hut_d', SAM = [18.5, 0, 14.2];
const flat = (a: number[], b: number[]) => Math.hypot(a[0] - b[0], a[2] - b[2]);
const f2 = (v: number) => v.toFixed(2);

interface Man { name: string; stations: number; lead: number; gun: number; knee: number; leanAsked: boolean; leanAfter: number; kneeAfter: number; walked: number; /** the stations he was sent to, and those of them his hips stood on */ sent: Set<number>; stood: Set<number>; fin: boolean; /** he was asked to lean while still on his first spot (his diagonal, hips not drawn back) */ lean0: boolean; /** how far he moved in the two seconds before the door was called held */ drift: number; /** the whole hold, whether he has cuts or not: the lowest he got, the times he began a lean, the cut he was on when he first did, whether he went on to another cut after it, and the furthest he got from where he stood when the first cut was taken */ low: number; leans: number; leanAt: number; cutAfterLean: boolean; strayed: number; /** frames he was asked to lean, and the furthest his head got past his hips toward the middle while he was */ leanFrames: number; leanHead: number; /** the furthest his hips stood in from his own jamb's edge of the opening, metres (negative: beside the opening, not in front of it) */ across: number }
interface Out { why: string | null; told: number; unseen: number; cells: number; foe: string; foeSeen: boolean; foeDown: boolean; lost: number; over: number; men: Man[]; apart: number; beats: string; g: any; /** K.word: the word was given with a man mid-cut on a knee — the op's phase, and who still leaned or knelt outside, four seconds on */ word: { given: boolean; phase: string; bent: string[] } }

/** the tape's door, the order given, run until the lead says the door is held (or `secs`) and a second more */
async function held(K: { blind?: boolean; /** the word is given while a man is out on his lean */ wordLean?: boolean }, secs = 40): Promise<Out> {
  const E = yardRow(LEVEL); if (!E) throw new Error(`no yard ${LEVEL}`);
  const R = await stage(E, 1), g = R.g, d = g.doors.byName(DOOR); if (!d) throw new Error(`no door ${DOOR}`);
  const on = await quiet(() => R.place(SAM, d)); if (!on) throw new Error(`X does not open on ${DOOR} from the spot`);
  const t = g.orders.targetUnder(R.input, R.W.cam, R.W.canvas), side = t.side as 1 | -1, W = d.def.width;
  const why = BREAK1 ? 'not given (THRPIEONE_BREAK)' : await quiet(() => g.orders.give('thresholdHold', t)) as string | null;
  const mid = doorPt(d, W / 2, 2.5, -side as 1 | -1);
  const foe = g.guards.filter((x: any) => x.team === 'hostile' && x.state !== 'dead').sort((a: any, b: any) => flat(a.char.pos, mid) - flat(b.char.pos, mid))[0];
  const team = () => R.team as any[], t0 = g.time;
  const rec = new Map<any, Man & { trail: number[][]; leaning: boolean; home: number[] | null }>(); let cutting = false; const word = { given: false, phase: '', bent: [] as string[] }; let op: any = null, told = NaN, unseen = -1, cells = 0, over = Infinity, foeSeen = false, foeDown = false, lostAtTold = 0, apart = Infinity;
  const jambOf = new Map<any, number>();   // (the jamb a man began on: one man's cuts may carry him past the door's middle)
  const inw = (m: any): number[] => { if (!jambOf.has(m)) jambOf.set(m, alongOf(d, m.char.pos) < W / 2 ? 1 : -1); const s = jambOf.get(m)!, a = doorPt(d, 0, 0, side), b = doorPt(d, 1, 0, side); return [(b[0] - a[0]) * s, 0, (b[2] - a[2]) * s]; };
  const tick = () => {
    op ??= R.D.ops.find((o: any) => o.door === d) ?? null; const T = op?.thr; if (!T) return;
    if (K.blind && op.search) op.search = undefined;   // (no model of the room's floor: every cut is walked, the ones down the door's own line too)
    for (const m of T.jamb as any[]) { const s = op.slice.get(m);
      let r = rec.get(m); if (!r) { r = { name: m.callsign, stations: s ? s.pts.length : 0, lead: -9, gun: -9, knee: 0, leanAsked: false, leanAfter: 0, kneeAfter: 0, walked: 0, sent: new Set(), stood: new Set(), fin: false, lean0: false, drift: NaN, low: 0, leans: 0, leanAt: -1, cutAfterLean: false, strayed: 0, leanFrames: 0, leanHead: -9, across: -9, trail: [], leaning: false, home: null }; rec.set(m, r); }
      if (m.state === 'dead') continue;
      if (isNaN(told)) { const ln = !!m.script?.lean; r.low = Math.max(r.low, m.char.anim.crouch); if (ln && !r.leaning) { r.leans++; if (r.leanAt < 0) r.leanAt = s ? s.i : 0; } r.leaning = ln; if (s && r.leanAt >= 0 && s.i > r.leanAt) r.cutAfterLean = true;
        if (ln) { r.leanFrames++; const u = inw(m), h = m.char.bones.head, hp = m.char.bones.hips ?? m.char.pos; if (h) r.leanHead = Math.max(r.leanHead, (h[0] - hp[0]) * u[0] + (h[2] - hp[2]) * u[2]); }
        if (s && s.i > 0) cutting = true; if (cutting) { r.home ??= [m.char.pos[0], 0, m.char.pos[2]]; r.strayed = Math.max(r.strayed, flat(m.char.pos, r.home)); } }
      if (isNaN(told)) { r.trail.push([m.char.pos[0], 0, m.char.pos[2]]); if (r.trail.length > 120) r.trail.shift(); }
      if (!s) continue;
      r.sent.add(s.i); if (flat(m.char.pos, s.pts[s.i]) < 0.15) r.stood.add(s.i); r.across = Math.max(r.across, (inw(m), jambOf.get(m)! > 0 ? alongOf(d, m.char.pos) : W - alongOf(d, m.char.pos))); r.fin = !!s.pie?.fin; if (s.i === 0 && !s.pie?.fin && m.script?.lean) r.lean0 = true;
      const u = inw(m), c = m.char, h = c.bones.head, hp = c.bones.hips ?? c.pos, mz = c.muzzle;
      if (isNaN(told) && s.i > 0) { if (h) r.lead = Math.max(r.lead, (h[0] - hp[0]) * u[0] + (h[2] - hp[2]) * u[2]); if (mz) r.gun = Math.max(r.gun, (mz[0] - hp[0]) * u[0] + (mz[2] - hp[2]) * u[2]); r.knee = Math.max(r.knee, c.anim.crouch); if (m.script?.lean) r.leanAsked = true; }
      r.walked = Math.max(r.walked, s.i); }
    if (K.wordLean && !word.given && (T.jamb as any[]).some(m => op.slice.get(m)?.pie && !op.slice.get(m).pie.fin && (rec.get(m)?.leanFrames ?? 0) > 20)) { word.given = true; R.W.input.pressed.add('KeyF'); }
    if (isNaN(told)) { for (const m of team()) if (m.state !== 'dead') over = Math.min(over, offOf(d, m.char.pos, side), m.char.bones.head ? offOf(d, m.char.bones.head, side) : Infinity);
      if (foe && (op.beats ?? []).some((b: any) => String(typeof b === 'string' ? b : b.what ?? b.text ?? '').includes(`seen · ${foe.callsign}`))) foeSeen = true;
      if (T.told) { told = g.time - t0; unseen = op.search?.left ?? -1; cells = op.search?.pts.length ?? 0; foeDown = !!foe && foe.state === 'dead'; lostAtTold = team().filter(m => m.state === 'dead').length; for (const r of rec.values()) if (r.trail.length) r.drift = flat(r.trail[0], r.trail[r.trail.length - 1]);
        const J = (T.jamb as any[]).filter(m => m.state !== 'dead'); if (J.length > 1) apart = flat(J[0].char.pos, J[1].char.pos); } } };
  await quiet(() => { for (let i = 0; i < secs * 60 && isNaN(told) && !word.given; i++) { R.step(); tick(); } R.step(word.given ? 240 : 90); });
  if (word.given) { word.phase = String(op?.phase ?? op?.ended ?? ''); word.bent = team().filter(m => m.state !== 'dead' && (m.script?.lean || (m.char.anim.crouch > 0.5 && offOf(d, m.char.pos, side) > 0))).map(m => m.callsign); }
  for (const [m, r] of rec) { r.leanAfter = Math.abs(m.script?.lean ?? 0); r.kneeAfter = m.char.anim.crouch; }
  return { why: why ?? null, told, unseen, cells, foe: foe ? `${foe.callsign} ${f2(flat(foe.char.pos, mid))} m off the door's own line, 2.5 m in` : 'nobody', foeSeen, foeDown, lost: lostAtTold, over, men: [...rec.values()], apart,
    beats: (op?.beats ?? []).map((b: any) => typeof b === 'string' ? b : b.what ?? b.text ?? '').join(' | '), g, word };
}
/** a teammate stood in the open with his gun in his shoulder, leaned each way through an order's own field: where his head, his muzzle, the middle of his
 *  trunk and his hips are, to his right of where his hips were */
async function pose() {
  const E = yardRow(LEVEL), R = await stage(E, 1), g = R.g, m = R.team[0], c = m.char, p = [c.pos[0], 0, c.pos[2]];
  let yaw = 0; const hold = (lean: number) => g.orders.assign(m, { order: 'move', post: p, face: yaw, watch: [p[0] + Math.sin(yaw) * 6, 1.2, p[2] + Math.cos(yaw) * 6], stance: 'none', upper: 'aim', going: 'x', there: 'x', ...(lean ? { lean } : {}) });
  for (const y of [0, 1.57, 3.14, -1.57, 0.8, -0.8, 2.4, -2.4]) { yaw = y; await quiet(() => { hold(0); R.step(60); }); if (!m.muzzleDown) break; }   // (a way to face with nobody and nothing in front of the gun)
  const h0 = [...(c.bones.hips ?? c.pos)], right = [-Math.cos(yaw), Math.sin(yaw)], lat = (q: number[] | undefined) => q ? (q[0] - h0[0]) * right[0] + (q[2] - h0[2]) * right[1] : NaN;
  const at = async (lean: number) => { const o = { head: 0, gun: 0, trunk: 0, hips: 0 }, N = 60;   // (settled, then the mean of a second, each off his hips of that frame: he breathes and shifts his weight)
    await quiet(() => { hold(lean); R.step(90); for (let i = 0; i < N; i++) { R.step(); const hp = lat(c.bones.hips ?? c.pos); o.head += (lat(c.bones.head) - hp) / N; o.gun += (lat(c.muzzle) - hp) / N; o.trunk += (lat(c.bones.spine2) - hp) / N; o.hips += hp / N; } }); return o; };
  const mid = await at(0), r = await at(BREAK ? 0 : 1), l = await at(BREAK ? 0 : -1), back = await at(0);
  return { up: !m.muzzleDown, mid: mid.head, right: r.head, left: l.head, back: back.head, gunRight: r.gun, gunLeft: l.gun, trunk: Math.max(Math.abs(r.trunk - mid.trunk), Math.abs(l.trunk - mid.trunk)), hips: Math.max(Math.abs(r.hips), Math.abs(l.hips)) };
}
const menOf = (o: Out) => o.men.map(m => `${m.name}: ${m.stations} stations, sent to ${[...m.sent].map(i => i + 1).join(',')}, stood on ${[...m.stood].map(i => i + 1).join(',')}, done ${m.fin}, head ${f2(m.lead)} m past his hips toward the middle, knee ${f2(m.knee)}`).join(' · ');

export const thrpieCheck = guarded(async () => {
  const F = family('thrpie: one jamb man slices a doorway before the door is called held, and leans once when his cuts are done');
  const knobs0 = knobsCopy();
  try {
    const one = await held({}), oneDeep = await held({ blind: true }), oneWord = await held({ wordLean: true });
    if (process.env.THRPIE_SHOW) for (const [n, o] of [['one', one], ['oneDeep', oneDeep], ['oneWord', oneWord]] as [string, Out][]) process.stderr.write(`${n}: why ${o.why} told ${f2(o.told)} unseen ${o.unseen}/${o.cells} foe ${o.foe} seen ${o.foeSeen} down ${o.foeDown} lost ${o.lost} over ${f2(o.over)} apart ${f2(o.apart)}\n   ${menOf(o)}\n   ${o.beats.slice(0, 600)}\n`);
    // (1) one man cuts, the other holds; one lean when the cuts are done; no knee
    { const leadOf = (o: Out) => o.men.find(m => m.stations > 0), farOf = (o: Out) => o.men.find(m => m.stations === 0);
      const L = leadOf(one), Fr = farOf(one), LD = leadOf(oneDeep), FD = farOf(oneDeep), say = (o: Out) => o.men.map(m => `${m.name}: ${m.stations ? `sent to ${[...m.sent].map(i => i + 1).join(',')} of ${m.stations}, stood on ${[...m.stood].map(i => i + 1).join(',')}` : `no cuts, strayed ${f2(m.strayed)} m`}, lowest ${f2(m.low)}, leans ${m.leans}`).join(' · ');
      F.verdict(one.why === null && !isNaN(one.told) && one.told < DOORTOOLS.thrCap - 2 && !isNaN(oneDeep.told) && oneDeep.told < one.told + 4 && !!LD && (L?.across ?? 9) < 0.05 && LD.across < 0.05 && one.men.length === 2 && !!L && !!Fr && L.stations >= 4 && L.sent.size >= 2 && [...L.sent].every(i => L.stood.has(i)) && L.fin && Fr.strayed < 0.3 && one.lost === 0
        && !!LD && !!FD && LD.walked + 1 === LD.stations && FD.strayed < 0.3 && [one, oneDeep].every(o => o.men.every(m => m.low < 0.1)),
        'ONE man cuts: the lead walks the cuts, his hips on every one he is sent to and never in front of the opening, and is done before the door is called; the far jamb\'s man has no cuts and does not leave his diagonal once the cutting begins; nobody goes on a knee, not even with every cut walked; nobody is lost',
        `told at ${f2(one.told)} s, ${f2(oneDeep.told)} s with every cut walked · ${one.unseen} of ${one.cells} cells unseen · the man in the middle: seen ${one.foeSeen}, down ${one.foeDown} · lost ${one.lost} · ${say(one)} · every cut walked: ${say(oneDeep)}`);
      F.verdict(!!L && !!Fr && !!LD && L.leans === 1 && !L.cutAfterLean && L.leanAt === L.walked && L.leanAt > 0 && LD.leans === 1 && !LD.cutAfterLean && LD.leanAt === LD.walked && Fr.leans === 0 && !!FD && FD.leans === 0 && [one, oneDeep].every(o => o.men.every(m => m.leanAfter === 0))
        && L.leanFrames >= 45 && L.leanHead > 0 && oneWord.word.given && oneWord.word.phase !== 'thr' && oneWord.word.bent.length === 0,
        'the lean comes once, when his cuts are done: the lead leans one time, on the last cut he took, and takes no cut after it — held long enough to be seen, his head past his hips toward the middle; the man holding never leans; once the door is held everybody stands square; the word given while he is out on the lean: the order goes on and nobody outside is still leaned four seconds later',
        `the lead leaned ${L?.leans} time(s), first on cut ${(L?.leanAt ?? -2) + 1}, the last he took ${(L?.walked ?? -2) + 1}, a cut after it: ${L?.cutAfterLean} · every cut walked: ${LD?.leans} time(s) on cut ${(LD?.leanAt ?? -2) + 1} of ${LD?.stations} · the man holding leaned ${Fr?.leans} / ${FD?.leans} · after: ${one.men.map(m => m.leanAfter).join(', ')} · his hips at most ${f2(L?.across ?? NaN)} m in from his jamb's edge (${f2(LD?.across ?? NaN)} with every cut walked) · the lean held ${L?.leanFrames} frames, head ${f2(L?.leanHead ?? NaN)} m past his hips · the word on the lean: given ${oneWord.word.given}, the order then ${oneWord.word.phase}, still bent ${oneWord.word.bent.join(', ') || 'nobody'}`); }
    // (the frame line and the gap between the two: asked of both jamb men cutting until 2026-11-07, of the one-man runs since)
    F.verdict(one.over > 0.3 && oneDeep.over > 0.3 && [one, oneDeep].every(o => Number.isFinite(o.apart) && o.apart >= DOORTOOLS.thrPieApart - 0.05),
      'nobody\'s hips or head cross the frame line (or come within 0.3 m of it) while it is worked from outside, and the two end no nearer each other than a man is wide — every cut walked too',
      `nearest the frame line: ${f2(one.over)} m (every cut ${f2(oneDeep.over)}) · apart at the end: ${[one, oneDeep].map(o => f2(o.apart)).join(', ')} m`);
    // (2) the lean is in the bones
    const P = await pose();
    F.verdict(P.up && P.right - P.mid > 0.08 && P.mid - P.left > 0.08 && P.gunRight > P.right - 0.02 && Math.abs(P.trunk) < 0.06 && Math.abs(P.hips) < 0.08 && Math.abs(P.back - P.mid) < 0.04,
      'the lean is in the bones: a man with his gun up, leaned to a side, has his head a hand or more that way of where it was, his hips and the middle of his trunk where they were (the columns a round is tested against stay); leaned to his gun side the muzzle is out past his head; told to stand square he is back',
      `head right of his hips (a second's mean): square ${f2(P.mid)} m · leaned right ${f2(P.right)} · leaned left ${f2(P.left)} · square again ${f2(P.back)} · muzzle leaned right ${f2(P.gunRight)}, left ${f2(P.gunLeft)} · the trunk's middle moved ${f2(P.trunk)} m, the hips ${f2(P.hips)} m`);
  } finally { knobsBack(knobs0); }
  return F.done();
});

if (import.meta.main) process.exit((await thrpieCheck()) ? 0 : 1);
