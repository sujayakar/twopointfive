// STEALTH POSES (src/game/stealthpose.ts — the feed in stealthfeed.ts, the animator's half in character.ts stealthLayer / stealthPicture), headless:
//   bun run tools/qa/stealthpose-check.ts [idle|walk|rise|creep|cable|standdown …]      (probes.ts stealthpose)
// The switch and its halves are settled on: the check asks the page's side alone and writes no switch. Until 2026-11-07 every part was played twice, switch
// on and off, and held to the off run ("his place the same on every frame", "bones.head never higher than off", "within 2 cm of the switch-off run", the
// hands lower than off on the way up, back in the layer against off, the right hand where off has it, the head 10 cm under the kneel's and never higher than
// off, the crouch again after the look, the gun hand where the build's carriage has it); those lines went with the off run, with the nod's key-off line and
// the probe stealthpose-harness (the two stands' row, the stand word, the page's flag). What is left holds a number of its own:
//   idle       crouched at a stand: both wrists forward of the knees, between 0.15 m off the floor and the higher knee, never a held frame
//   walk       the crouch walk: the hand opposite the forward foot is the forward hand; wrists never under the floor
//   rise       standing up and crouching down on the move: upright the layer is still fed
//   creep      the close-in mode near a man: the free (left) hand carried low, leading its own foot, travelling with every step
//   cable      the look under a door: the pose itself — the drawn skeleton and `bones` are one (what is drawn is what the game reads), the head below the
//              shoulder line, the right wrist low at the gap, the lock work's flag and the wand's look up; a goggle nod inside the look shows
//   standdown  the pistol drawn from the crouch: the layer is out inside 0.3 s
// STEALTHPOSE_BREAK=1 = Sam is stood on his feet where a part asks for the crouch, and at the door the wand's row is never taken: idle, walk, creep, cable
// and standdown must fail. The stagings are tools/qa/stealth-strip.ts's own.
import { ROOT } from './headless.ts';
import { asProbes, exitCode, wanted } from './holds.ts';   // (run alone, each part runs as the probe the runner calls it by: one stand both ways)
const { KITANIM_STATS } = await import(`${ROOT}/src/game/kitanim.ts`);
const { m4 } = await import(`${ROOT}/src/math/vec.ts`);
const ST: any = await import('./stealth-strip.ts');

type P3 = number[];
type Verdict = (ok: boolean, label: string, detail: string) => void;
function family(title: string): { verdict: Verdict; done: () => boolean } {
  console.log(`\n=== ${title} ===`); let all = true;
  return { verdict: (ok, label, detail) => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); }, done: () => { console.log(`=== ${title.split(':')[0]}: ${all ? 'ALL PASS' : 'FAILURES above'} ===`); return all; } };
}
const hushed = async <T>(fn: () => Promise<T> | T): Promise<T> => { const log = console.log, err = console.error, warn = console.warn; console.log = console.error = console.warn = () => { }; try { return await fn(); } finally { console.log = log; console.error = err; console.warn = warn; } };
const BREAK = process.env.STEALTHPOSE_BREAK === '1';
const DT = 1 / 60, EAST = Math.PI / 2;
const cm = (x: number) => (Number.isFinite(x) ? (x * 100).toFixed(1) : String(x)), f2 = (x: number) => (Number.isFinite(x) ? x.toFixed(2) : String(x));
const dist = (a: P3, b: P3) => Math.hypot(a[0] - b[0], a[1] - b[1], a[2] - b[2]);
/** one frame of a run: his place, the game's bones (world, and in his own frame: left, up, forward), the DRAWN skeleton's head / shoulders / right wrist */
interface Rec { pos: P3; head: P3; chest: P3; hL: P3; hR: P3; kL: P3; kR: P3; fL: P3; fR: P3; wHead: P3; wChest: P3; dHead: P3; dShoL: P3; dShoR: P3; dHR: P3; fed: boolean; made: boolean; wA: number; lockpick: boolean; wanding: boolean; creep: boolean; ready: string | null }
interface Run { recs: Rec[] }
function rec(g: any): Rec {
  const c = g.player.char, F = ST.frameOf(c), n = c.anim.nodes, D = (k: string): P3 => F.rel(m4.transformPoint(c.world, c.skel.nodePos(n[k]))), B = (k: string): P3 => F.rel(c.bones[k]);
  return { pos: [...c.pos], head: B('head'), chest: B('chest'), hL: B('handL'), hR: B('handR'), kL: D('shinL'), kR: D('shinR'), fL: D('footL'), fR: D('footR'), wHead: [...c.bones.head], wChest: [...c.bones.chest],
    dHead: D('head'), dShoL: D('upperArmL'), dShoR: D('upperArmR'), dHR: D('handR'), fed: c.anim.stealth != null, made: (c.anim as any).sp != null, wA: (c.anim as any).sp?.wA ?? 0, lockpick: !!c.anim.lockpick, wanding: !!g.player.wanding, creep: !!c.anim.creep, ready: c.anim.ready ?? null };
}
type Part = 'idle' | 'walk' | 'rise' | 'creep' | 'cable' | 'draw';
/** a part played from a fresh stand-up on its own seed */
async function play(part: Part): Promise<Run> {
  const recs: Rec[] = []; let g: any;
  const snap = () => { recs.push(rec(g)); };
  if (part === 'cable') {
    const S = await ST.atDoor(7105); g = S.g; snap();
    if (!BREAK && !S.begin()) throw new Error("cable: the wand's row was not offered");
    snap(); for (let i = 0; i < 100; i++) { S.W.step(DT); snap(); }
    S.W.input.pressed.add('Space'); for (let i = 0; i < 60; i++) { S.W.step(DT); snap(); }
  } else {
    const S = await ST.lobby(7100 + ['idle', 'walk', 'rise', 'creep', 'cable', 'draw'].indexOf(part), !BREAK, part === 'creep' ? [5.6, 0, 19.5] : [6.5, 0, 19.5]); g = S.g;
    const go = (n: number) => { for (let i = 0; i < n; i++) { S.step(1); snap(); } };
    if (part === 'idle') go(150);
    else if (part === 'walk') { S.W.input.keys.add('KeyD'); go(150); }
    else if (part === 'rise') { S.W.input.keys.add('KeyD'); go(50); g.player.crouch = false; go(70); g.player.crouch = true; go(60); }
    else if (part === 'creep') { const A = g.guards[0]; ST.place(A, [10.2, 0, 19.5], EAST); A.hold = true; A.pinned = true; S.W.input.keys.add('KeyD'); let n = 0; while (g.player.char.pos[0] < 10.2 - 1.3 && n++ < 500) go(1); }
    else { go(30); S.W.input.pressed.add('KeyE'); go(40); }
  }
  return { recs };
}
const cache = new Map<string, Promise<Run>>();
export const run = (part: Part) => { let r = cache.get(part); if (!r) cache.set(part, r = hushed(() => play(part))); return r; };

/** the line every part holds */
function common(F: { verdict: Verdict }, part: string, on: Run) {
  let low = Infinity; for (const a of on.recs) low = Math.min(low, a.hL[1], a.hR[1]);
  F.verdict(low > 0.02, `${part}: neither wrist is ever under the floor`, `${on.recs.length} frames · lowest wrist ${cm(low)} cm up`);
}

export async function stealthposeIdle(): Promise<boolean> {
  const F = family('stealthpose-idle: crouched at a stand, hands empty — the arms hang forward of the knees, and never a held frame');
  const rnd = Math.random;
  try {
    const on = await run('idle'); common(F, 'idle', on);
    const side = (rs: Rec[]) => { let fwd = Infinity, lo = Infinity, hi = -Infinity; for (const r of rs) { const kz = Math.max(r.kL[2], r.kR[2]), ky = Math.max(r.kL[1], r.kR[1]); fwd = Math.min(fwd, r.hL[2] - kz, r.hR[2] - kz); lo = Math.min(lo, r.hL[1], r.hR[1]); hi = Math.max(hi, r.hL[1] - ky, r.hR[1] - ky); } return { fwd, lo, hi }; };
    const A = side(on.recs.slice(20));
    F.verdict(A.fwd > 0, 'both wrists are forward of the front knee on every frame', `the nearer wrist is ${cm(A.fwd)} cm in front of it`);
    F.verdict(A.lo >= 0.15 && A.hi <= 0, 'both wrists are between 0.15 m off the floor and the higher knee', `lowest ${cm(A.lo)} cm up, highest ${cm(A.hi)} cm against the higher knee`);
    const span = (rs: Rec[], k: 'hL' | 'hR' | 'wHead') => { let m = 0; for (const r of rs) m = Math.max(m, dist(r[k], rs[0][k])); return m; };
    const mv = Math.min(span(on.recs.slice(20), 'hL'), span(on.recs.slice(20), 'hR'));
    let still = 0, runL = 0; for (let i = 21; i < on.recs.length; i++) { runL = dist(on.recs[i].hL, on.recs[i - 1].hL) < 1e-5 ? runL + 1 : 0; still = Math.max(still, runL); }
    F.verdict(mv > 0.008 && still < 6, 'never a held frame: each wrist drifts more than 8 mm over the cycle, and is never still six frames running', `wrists travel ≥ ${cm(mv)} cm · longest run of still frames ${still}`);
    F.verdict(on.recs.slice(30).every(r => r.wA > 0.9), 'the layer is fully in from half a second on', `least weight ${f2(Math.min(...on.recs.slice(30).map(r => r.wA)))}`);
  } finally { Math.random = rnd; }
  return F.done();
}

export async function stealthposeWalk(): Promise<boolean> {
  const F = family('stealthpose-walk: the crouch walk, hands empty — the hand opposite the forward foot is the forward hand');
  const rnd = Math.random;
  try {
    const on = await run('walk'); common(F, 'walk', on);
    const tally = (rs: Rec[]) => { let steps = 0, ok = 0, held = 0, sgn = 0, worst = Infinity;
      for (let i = 40; i < rs.length; i++) { const r = rs[i], d = r.fR[2] - r.fL[2], s = Math.abs(d) > 0.3 ? Math.sign(d) : 0; held = s !== 0 && s === sgn ? held + 1 : 0; sgn = s;
        if (held === 8) { steps++; const lead = s * (r.hL[2] - r.hR[2]); worst = Math.min(worst, lead); if (lead > 0.1) ok++; } }   // (s > 0: the right foot is ahead, so the LEFT hand leads)
      return { steps, ok, worst }; };
    const A = tally(on.recs);
    F.verdict(A.steps >= 3 && A.ok === A.steps, 'on every step the hand opposite the forward foot is more than 10 cm ahead of the other', `${A.ok} of ${A.steps} steps, least lead ${cm(A.worst)} cm`);
    let hi = -Infinity; for (const r of on.recs.slice(40)) { const ky = Math.max(r.kL[1], r.kR[1], 0.5); hi = Math.max(hi, r.hL[1] - ky, r.hR[1] - ky); }
    F.verdict(hi <= 0.1, 'the hands stay low: no wrist more than 10 cm over knee height', `highest ${cm(hi)} cm against the higher knee`);
  } finally { Math.random = rnd; }
  return F.done();
}

export async function stealthposeRise(): Promise<boolean> {
  const F = family('stealthpose-rise: standing up and crouching down on the move — arms only');
  const rnd = Math.random;
  try {
    const on = await run('rise'); common(F, 'rise', on);
    F.verdict(on.recs.slice(110, 120).every(r => r.wA > 0.9), 'upright, hands empty, the layer is still fed', `weight ${f2(on.recs[115].wA)}`);
  } finally { Math.random = rnd; }
  return F.done();
}

export async function stealthposeCreep(): Promise<boolean> {
  const F = family('stealthpose-creep: the close-in mode near a man — the free hand leads');
  const rnd = Math.random;
  try {
    const on = await run('creep'); common(F, 'creep', on);
    // (the frames the close-in mode has him and the knife's tell does not yet: in reach the tell's own arms are the build's. The free hand goes out LOW
    //  with its own foot and comes back as he passes over it)
    const lead = (rs: Rec[]) => { let y = 0, n = 0, out = 0, no = 0, back = 0, nb = 0, lo = Infinity, hi = -Infinity;
      for (const r of rs.slice(12)) if (r.creep && !r.ready) { const z = r.hL[2] - r.chest[2]; y += r.hL[1]; n++; lo = Math.min(lo, z); hi = Math.max(hi, z); if (r.fL[2] > 0.1) { out += z; no++; } else if (r.fL[2] < -0.3) { back += z; nb++; } }
      return { y: y / Math.max(1, n), n, out: out / Math.max(1, no), back: back / Math.max(1, nb), travel: hi - lo }; };
    const A = lead(on.recs.slice(84));
    F.verdict(A.n >= 20 && A.y < 0.5 && A.y > 0.25, 'in the mode and out of reach, the free (left) hand is carried LOW: between shin and knee height', `${A.n} frames · ${cm(A.y)} cm up on average`);
    F.verdict(A.out > A.back + 0.1 && A.out > 0.3, 'it leads its own foot: out in front while the left foot is forward, back while that foot is behind him', `${cm(A.out)} cm ahead of the chest with the foot forward, ${cm(A.back)} cm with it behind`);
    F.verdict(A.travel > 0.15, 'so it is never a held hand: it travels with every step', `${cm(A.travel)} cm of travel`);
  } finally { Math.random = rnd; }
  return F.done();
}

export async function stealthposeCable(): Promise<boolean> {
  const F = family("stealthpose-cable: the look under a door — a squat bowed to the gap, the pose itself (drawn and read as one)");
  const rnd = Math.random;
  try {
    const on = await run('cable');
    common(F, 'cable', on);
    { let dOn = 0; const j = (r: Rec) => dist(r.dHead, r.head); const j0 = j(on.recs[0]); for (const r of on.recs) dOn = Math.max(dOn, Math.abs(j(r) - j0));   // (frame 0: at the crouch, before the row)
      F.verdict(dOn < 1e-4, 'on every frame the DRAWN head joint is where bones.head has it (one skeleton: what is drawn is what the game reads)', `drawn joint to bones.head: ${cm(j0)} cm of the head's own, never more than ${dOn.toExponential(1)} m off that`); }
    F.verdict(on.recs[60].lockpick && on.recs[60].wanding, "a second in, the lock work's flag and the wand's look are up", `lockpick ${on.recs[60].lockpick}, wanding ${on.recs[60].wanding}`);
    const a = on.recs[60], sho = Math.min(a.dShoL[1], a.dShoR[1]);
    F.verdict(a.dHead[1] < sho, 'a second in, the head is bowed below the shoulder line', `drawn head ${cm(a.dHead[1])} cm up · shoulders ${cm(sho)} cm`);
    F.verdict(a.dHR[1] < 0.35 && a.dHR[2] > a.kL[2] - 0.05, 'the right wrist is low at the gap', `${cm(a.dHR[1])} cm up, ${cm(a.dHR[2])} cm forward`);
    const t = on.recs.findIndex((r, i) => i > 1 && r.dHead[1] < sho + 0.03);
    F.verdict(t > 0 && t <= 40, 'he is down in it inside two thirds of a second of the row', `frame ${t}`);
    // a goggle NOD inside the look (the hands are the look's, so a flip there is the nod): the look at the gap must not take it back out
    { type NodRun = { dir: P3; down: number; head: P3 }[];
      const nodRun = async (o: { flip: 'down' | 'up' | null; at?: number }): Promise<NodRun> => hushed(async () => {
        // (the flip is asked for with the N KEY, as he asks: the wand reads it while he looks — WAND.nv — and kit handling makes it a nod, his hands being on the cable)
        const S = await ST.atDoor(7105), pl = S.g.player, c = pl.char; if (o.flip === 'up') pl.nv = true;   // (goggles up: they are down before he kneels)
        if (!S.begin()) throw new Error("cable: the wand's row was not offered");
        const at = o.at ?? 70, out: NodRun = [], nods0 = KITANIM_STATS.nv.nod;
        for (let i = 0; i < at + 36; i++) { if (i === at && o.flip) S.W.input.pressed.add('KeyN'); S.W.step(DT);
          if (i === at && o.flip && (pl.nv !== (o.flip === 'down') || KITANIM_STATS.nv.nod !== nods0 + 1)) throw new Error(`cable: N during the look did not flip the goggles by a nod (nv ${pl.nv}, nods ${KITANIM_STATS.nv.nod - nods0})`);
          if (i >= at) { const h = c.headDir(); out.push({ dir: [h[0], h[1], h[2]], down: Math.asin(Math.max(-1, Math.min(1, -h[1]))) * 180 / Math.PI, head: [...c.bones.head] as P3 }); } }
        return out; });
      const ang = (a: P3, b: P3) => Math.acos(Math.max(-1, Math.min(1, a[0] * b[0] + a[1] * b[1] + a[2] * b[2]))) * 180 / Math.PI;
      const cmp = (fl: NodRun, st: NodRun) => { let peak = 0, steeper = -Infinity, joint = 0, late = 0, step = 0, down = 0;
        for (let i = 0; i < fl.length; i++) { const d = ang(fl[i].dir, st[i].dir); peak = Math.max(peak, d); steeper = Math.max(steeper, fl[i].down - st[i].down); joint = Math.max(joint, dist(fl[i].head, st[i].head)); down = Math.max(down, fl[i].down);
          if (i > 0) step = Math.max(step, ang(fl[i].dir, fl[i - 1].dir)); if (i >= 30) late = Math.max(late, d); }
        return { peak, steeper, joint, late, step, down }; };
      const said = (m: ReturnType<typeof cmp>, st: NodRun) => `head turned ${m.peak.toFixed(1)}° at most from the same frames with no flip · ${m.steeper <= 0 ? 'never steeper' : `${m.steeper.toFixed(2)}° steeper at most`} (the look: ${st[0].down.toFixed(0)}° down) · bones.head ${(m.joint * 1000).toFixed(2)} mm off · ${m.step.toFixed(1)}° in a frame at most · ${m.late.toFixed(2)}° left half a second on`;
      const still = await nodRun({ flip: null });
      for (const way of ['down', 'up'] as const) { const m = cmp(await nodRun({ flip: way }), still);   // (goggles down or up, the head is posed the same before the flip: one run with no flip stands for both)
        F.verdict(m.peak > 12 && m.steeper < 0.5 && m.joint < 0.001 && m.step < 7 && m.late < 0.5, `night vision flipped ${way} inside the look: the nod SHOWS (the look at the gap does not take it back out) as a lift of the head off the gap — never nearer straight down than the look itself — the head joint stays put, no frame jumps, and it is over when the nod is`, said(m, still)); }
      // …and a nod the look comes in over (the flip as the row is taken) stays the nod it began as: seen, never through straight down, no frame jumps
      { const st0 = await nodRun({ flip: null, at: 2 }), flx = await nodRun({ flip: 'down', at: 2 }), m = cmp(flx, st0); let own = 0; for (let i = 1; i < st0.length; i++) own = Math.max(own, ang(st0[i].dir, st0[i - 1].dir));
        F.verdict(m.peak > 8 && m.down < 84 && m.joint < 0.005 && m.step < own + 6.5 && m.late < 0.5, 'flipped as the look begins, the nod is seen on the way in (neither taken out nor cancelled half way), his gaze stops short of straight down, the head joint stays within half a centimetre and no frame turns the head by more than the look and the nod do between them',
          `${said(m, st0)} · steepest ${m.down.toFixed(1)}° down · the look alone turns it ${own.toFixed(1)}° in a frame at most`); } }
  } finally { Math.random = rnd; }
  return F.done();
}

export async function stealthposeStanddown(): Promise<boolean> {
  const F = family('stealthpose-standdown: the pistol drawn from the crouch — the layer is out inside 0.3 s');
  const rnd = Math.random;
  try {
    const on = await run('draw'); common(F, 'draw', on);
    const k = 30, before = on.recs[k - 1].wA, at = on.recs.findIndex((r, i) => i >= k && r.wA <= 0.01);
    F.verdict(before > 0.9 && at >= k && at - k <= 18, 'E from the crouch idle: the layer was in, and is out within 18 frames of the key', `weight before ${f2(before)} · out ${at < 0 ? 'never' : `${at - k} frames after`}`);
  } finally { Math.random = rnd; }
  return F.done();
}

/** the file's probes, by the names the runner calls them */
const SUBS: Record<string, () => Promise<boolean>> = { 'stealthpose-idle': stealthposeIdle, 'stealthpose-walk': stealthposeWalk, 'stealthpose-rise': stealthposeRise, 'stealthpose-creep': stealthposeCreep, 'stealthpose-cable': stealthposeCable, 'stealthpose-standdown': stealthposeStanddown };
if (import.meta.main) {
  if (BREAK) console.log("STEALTHPOSE_BREAK=1: Sam is on his feet where a part asks for the crouch, the wand's row never taken — idle / walk / creep / cable / standdown must fail");
  const ok = await asProbes(SUBS, wanted());   // (a name the file has not: said, nothing run, exit 2)
  if (ok !== null) console.log(ok ? '\n=== stealthpose: ALL PASS ===' : '\n=== stealthpose: FAILED ===');
  process.exit(exitCode(ok));
}
