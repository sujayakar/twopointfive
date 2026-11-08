// A FAR GLIMPSE IS NOT YET A PLACE TO WALK TO — the check behind src/game/guards.ts `GUARD_EYE.farCalm` / `farFaint` (a settled rule: the game has no other
// side of it, so nothing here asks one. Until 2026-11-07 lines a, b, c and f each carried a "rule off" half, and g, h and i asked the far sight off, a
// recording from before the key and the old stand's word). The fault it holds shut: a lit man glimpsed from past the old 16 m, far too faintly to
// make a man wary, still wrote his `lastKnown` — and when the mains went (lighting.ts setBlackout puts every patrolling man to 0.34) he walked off his post
// to it, in the dark the far reach was never to change.
//   bun run tools/qa/probes.ts farcalm                  (also in the `farsight` family and all-quick)
//   FARCALM_BREAK=1 bun run tools/qa/farcalm-check.ts   Sam stood at 12 m, inside the old reach, where a faint glimpse IS a fix: a, b and c must FAIL (exit 1)
// What it asks, on the far sight check's own stage (the office's open south strip, one man planted facing east, Sam's light meter fed by hand):
//   a  glimpse    Sam lit at 20 m for a moment: the calm man's awareness rises, he stays calm, and he has NO fix
//   b  the edge   then the blackout's own bump (0.34), the meter a fifth of a second behind the dark: wary, and still no fix 5 s on
//   c  the walk   the same edge with the man free to walk: he ends it where he stood
//   d  the tip    Sam left standing lit at 20 m until the sight itself has him wary (meter 1 and 0.85): the first fix lands at the 'huh?' line, on Sam
//   d2 watching   a man already wary (the blackout's bump, or a noise 40 degrees off the line to Sam: his eyes go with his head, so a noise behind him would turn them off Sam) who goes on watching Sam lit at 20 m: his fix is Sam from `farFixAt`,
//                 before he is alert — he turns and goes, he does not stand till CONTACT
//   d3 again      over that line every glimpse counts: Sam dark for a moment and lit again 2 m on, the fix is on him again
//   e  hunting    an alert man takes a far glimpse as his fix at once
//   f  near       inside 16 m a faint glimpse writes the fix on the frame it lands
// exit 0 = all hold · 1 = one does not
import { standUp, ROOT } from './headless.ts';
const { v3 } = await import(`${ROOT}/src/math/vec.ts`);
const { mulberry32 } = await import(`${ROOT}/src/math/dice.ts`);
const { GUARD_EYE } = await import(`${ROOT}/src/game/guards.ts`);
type Vec3 = [number, number, number];
const BREAK = process.env.FARCALM_BREAK === '1';
const DT = 1 / 60, EAST = Math.PI / 2, AT: Vec3 = [4, 0, 2.5], f2 = (n: number) => n.toFixed(2), FAR = BREAK ? 12 : 20, BUMP = 0.34;

function place(gd: any, p: Vec3, yaw: number) { gd.char.pos = v3.copy(p); gd.char.vel = [0, 0, 0]; gd.speed = 0; gd.char.bodyYaw = gd.char.aimYaw = yaw; gd.char.aimPitch = 0; gd.path = []; gd.pathGoal = null; gd.char.update(0); gd.char.update(DT); }
async function stage() {
  const W: any = await standUp(); const g = W.game;
  g.godMode = true; g.player.visibility = 0.02; g.player.slot = 'pistol'; g.player.holstered = true;
  for (let i = 0; i < 5; i++) W.step(DT);
  const [A, B, C] = g.guards;
  for (const [gd, i] of [[B, 0], [C, 1]] as [any, number][]) { gd.hold = true; place(gd, [37.5, 0, 6 + i * 1.5], EAST); gd.heardUpTo = 1e12; }
  for (const h of [0.3, 1.1, 1.65]) if (g.col.segmentBlocked([AT[0], h, AT[2]], [AT[0] + 28, h, AT[2]])) throw new Error(`farcalm: the south strip is no longer a clear 28 m at ${h} m up — the level moved under the check`);
  return { W, g, A };
}

export async function farcalmCheck(): Promise<boolean> {
  let fails = 0, checks = 0;
  const say = (ok: boolean, what: string) => { checks++; if (!ok) fails++; console.log(`  ${ok ? 'PASS' : 'FAIL'}  ${what}`); };
  const log = console.log, die = Math.random; console.log = () => { }; let S: any; Math.random = mulberry32(7);
  try { S = await stage(); } finally { console.log = log; }
  console.log(`\n=== FAR GLIMPSE: no place to walk to until it is what has him looking (GUARD_EYE.farCalm)${BREAK ? ' — FARCALM_BREAK=1: Sam stood at 12 m, where a faint glimpse is a fix: a, b and c must fail' : ''} ===`);
  try {
    const { W, g, A } = S;
    const sam = (d: number, lit: number) => { g.teleportPlayer([AT[0] + d, 0, AT[2]]); g.player.crouch = false; g.player.visibility = lit; };
    const run = (frames: number, lit: number, each?: () => void) => { for (let i = 0; i < frames; i++) { W.step(DT); g.player.visibility = lit; each?.(); } };
    const calm = (hold = true) => { A.hold = hold; A.wp = 1; A.heardUpTo = 1e12; A.awareness = 0; A.state = 'patrol'; A.lastKnown = null; A.reactT = 0; A.searchT = 0; A.sightT = -100; A.task = null; A.wait = 1e6; place(A, AT, EAST); };
    const off = (p: Vec3 | null) => p ? f2(v3.distXZ(p, g.player.char.pos)) + ' m off Sam' : 'none';
    const onSam = (p: Vec3 | null) => !!p && v3.distXZ(p, g.player.char.pos) < 0.1;
    /** Sam lit at 20 m until the calm man has taken a little of him in (never to the 'huh?' line) */
    const glimpse = (hold = true) => { calm(hold); sam(FAR, 0.02); run(3, 0.02); calm(hold); sam(FAR, 1); let n = 0; while (A.awareness < 0.08 && n < 240) { run(1, 1); n++; } return n; };
    // --- a
    { const n = glimpse(), aw = A.awareness, st = A.state, fix = A.lastKnown;
      say(aw >= 0.08 && aw < 0.3 && st === 'patrol' && fix === null, `a · glimpse: lit at ${FAR} m for ${f2(n / 60)} s the calm man takes him in (awareness ${f2(aw)}, ${st}) and has no fix (${off(fix)})`); }
    // --- b · the blackout's edge, as lighting.ts setBlackout leaves a patrolling man: awareness to 0.34 — and the meter a fifth of a second behind the dark (game.ts damps it at 6 a second)
    const LAG = 12, FIX = GUARD_EYE.farFixAt as number;
    const edge = (hold: boolean) => { glimpse(hold); A.awareness = Math.max(A.awareness, BUMP); A.reactT = 1.0; let wary = false, fix = false, top = 0; const each = () => { wary ||= A.state === 'suspicious'; fix ||= !!A.lastKnown; top = Math.max(top, A.awareness); }; run(LAG, 0.85, each); const lag = top; run(300 - LAG, 0.02, each); return { wary, fix, lag, last: A.lastKnown as Vec3 | null, x: A.char.pos[0] as number }; };
    { const on = edge(true);
      say(on.wary && on.lag > BUMP && on.lag < FIX && !on.fix, `b · the edge: put to ${BUMP} as the mains going does, his meter ${f2(LAG / 60)} s behind the dark (seen in it: awareness to ${f2(on.lag)}) — wary ${on.wary}, a fix at any frame of the 5 s ${on.fix} (${off(on.last)})`); }
    // --- c
    { const on = edge(false);
      say(Math.abs(on.x - AT[0]) < 0.5, `c · the walk: free to walk, 5 s on he stands ${f2(Math.abs(on.x - AT[0]))} m from his post`); }
    // --- d · the sight itself tips a calm man: at two lights (0.85: a rate under the 'seen' line, the unseen decay pulling against it)
    { const tip = (lit: number) => { calm(); sam(FAR, lit); let n = 0, awAtFix = -1; while (A.state === 'patrol' && n < 2400) { run(1, lit); n++; if (awAtFix < 0 && A.lastKnown) awAtFix = A.awareness; } return { n, tipped: A.state === 'suspicious', awAtFix, at: onSam(A.lastKnown), d: off(A.lastKnown) }; };
      const a = tip(1), b = tip(0.85);
      say([a, b].every(t => t.tipped && t.at && t.awAtFix >= 0.29) || BREAK, `d · the tip: lit at ${FAR} m the sight itself has him wary after ${f2(a.n / 60)} s (meter 1) / ${f2(b.n / 60)} s (0.85) — his first fix lands at awareness ${f2(a.awAtFix)} / ${f2(b.awAtFix)}, never under the 'huh?' line, and on Sam (${a.d} / ${b.d})`); }
    // --- d2 · a wary man who goes on watching: his fix from `farFixAt`, before he is alert — from the blackout's bump (no fix) and from a noise 40° off the line to him (its own fix)
    { const watch = (noise: Vec3 | null) => { calm(); sam(FAR, 0.02); run(3, 0.02); calm(); A.state = 'suspicious'; A.awareness = noise ? 0.45 : BUMP; A.lastKnown = noise ? v3.copy(noise) : null; A.reactT = 0.5; sam(FAR, 1);
        let early = false, fixF = -1, awFix = 0, alertF = -1; for (let f = 1; f <= 600 && alertF < 0; f++) { const before = A.awareness; run(1, 1); if (fixF < 0 && onSam(A.lastKnown)) { fixF = f; awFix = A.awareness; if (before < FIX - 0.02) early = true; } if (A.state === 'alert') alertF = f; } return { early, fixF, awFix, alertF }; };
      const dark = watch(null), heard = watch([AT[0] + 3 * Math.cos(40 * Math.PI / 180), 0, AT[2] + 3 * Math.sin(40 * Math.PI / 180)]);
      say([dark, heard].every(w => w.fixF > 0 && !w.early && w.awFix >= FIX && (w.alertF < 0 || w.fixF < w.alertF)) && dark.fixF > 6, `d2 · watching: wary at ${BUMP} with no fix, Sam lit at ${FAR} m — nothing for ${f2(dark.fixF / 60)} s, then his fix is Sam (awareness ${f2(dark.awFix)}, the line ${FIX}), ${dark.alertF < 0 ? 'never alert' : f2((dark.alertF - dark.fixF) / 60) + ' s before he is alert'}; wary at 0.45 on a noise 40° off the line to Sam: the noise's spot, then Sam after ${f2(heard.fixF / 60)} s`);
      // …and once over the line every glimpse counts: Sam gone dark for a fifth of a second, lit again 2 m on
      calm(); A.state = 'suspicious'; A.awareness = FIX + 0.1; A.lastKnown = null; A.reactT = 0.5; sam(FAR, 1); run(3, 1); const first = onSam(A.lastKnown); run(12, 0.02); sam(FAR + 2, 1); run(3, 1);
      say(first && onSam(A.lastKnown), `d3 · again: over the line he takes the glimpse at once (${first}); Sam dark for 0.2 s and lit again 2 m on — the fix is on him again (${off(A.lastKnown)})`); }
    // --- e
    { calm(); A.state = 'alert'; A.awareness = 1; sam(FAR, 1); run(2, 1);
      say(onSam(A.lastKnown), `e · hunting: an alert man takes the glimpse at ${FAR} m as his fix at once (${off(A.lastKnown)})`); }
    // --- f
    { const near = () => { calm(); sam(12, 0.02); run(3, 0.02); calm(); sam(12, 1); run(2, 1); return { fix: onSam(A.lastKnown), aw: A.awareness as number }; }; const a = near();
      say(a.fix && a.aw > 0 && a.aw < 0.3, `f · near: inside 16 m (12 m, two frames) the faint glimpse writes his fix (fix ${a.fix}, awareness ${a.aw.toFixed(4)})`); }
  } finally { Math.random = die; }
  console.log(`=== far glimpse: ${fails ? `${fails} of ${checks} FAILED` : `ALL PASS (${checks})`} ===`);
  return fails === 0;
}

if (import.meta.main) process.exit((await farcalmCheck()) ? 0 : 1);
