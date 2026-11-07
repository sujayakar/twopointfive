// IN A HOLD: THE RIGHT CLICK IS THE CHOKE (src/game/close.ts CLOSE.heldClicks — player.ts updateHold, interact.ts), headless:
//   bun run tools/qa/heldclicks-check.ts            (probes.ts heldclicks; HELDCLICKS_BREAK=1 = Sam's hands are swapped, the pistol drawn where a line wants the
//                                                    arm hold and holstered where it wants the gun's: every line must fail)
//   b  arm · RMB             the choke: the arm closes, out cold and breathing
//   d  gun hold              LMB = one round past him, RMB = the OCP
//   e  a held civilian       LMB and RMB change nothing
// The rule is settled (CLOSE.heldClicks), and so is no-knife (CLOSE.noKnife), which took the left click's knife out of the arm hold: nothing here writes a
// switch. Until 2026-11-07 the check also asked the arm hold with the switch off (c, half of g), held the gun hold's run to the same run with it off (d), a
// recording from before the key (h) and the stands' words and the page's flag (i); and it asked THE KNIFE: a (the left click kills, and what a witness
// saw), x (both buttons in one frame choose neither the knife nor the choke), m (the click that shuts an order menu is not the knife), f (the kill is a
// commit), s (Sam shot down either side of the cut) and g (the row's marks 'LMB  knife — lethal', the status line under the knife). Those asked a build from
// before no-knife and went by ruling. So did two things of `b`, written before the right button's tap and hold (CLOSE.rmbHold, settled on: a click in the
// arm closes it at once, no commit beat — tools/qa/rmbhold-check.ts `again` holds that): its clause that the click starts a wind-up (windT) and its line
// "exactly as the Space row's choke: the same frames" (on the page the click closes 1 frame on, the row 20). Nothing here looks at a picture.
import { standUp, ROOT } from './headless.ts';
import { asProbe } from './holds.ts';   // (run alone, each part runs as the probe the runner calls it by: one stand both ways)

const { v3 } = await import(`${ROOT}/src/math/vec.ts`);
const { startGrab } = await import(`${ROOT}/src/game/player.ts`);
const { isBreathing } = await import(`${ROOT}/src/game/game.ts`);

type P3 = [number, number, number];
type Verdict = (ok: boolean, label: string, detail: string) => void;
function family(title: string): { verdict: Verdict; done: () => boolean } {
  console.log(`\n=== ${title} ===`); let all = true;
  return { verdict: (ok, label, detail) => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); }, done: () => { console.log(`=== ${title.split(':')[0]}: ${all ? 'ALL PASS' : 'FAILURES above'} ===`); return all; } };
}
const hushed = async <T>(fn: () => Promise<T> | T): Promise<T> => { const log = console.log, err = console.error, warn = console.warn; console.log = console.error = console.warn = () => { }; try { return await fn(); } finally { console.log = log; console.error = err; console.warn = warn; } };
const BREAK = process.env.HELDCLICKS_BREAK === '1';
const DT = 1 / 60, EAST = Math.PI / 2, LMB = 1, RMB = 4;
const f3 = (x: number) => (Number.isFinite(x) ? x.toFixed(3) : String(x)), J = (x: unknown) => JSON.stringify(x);
function seeded(a: number) { return () => { a |= 0; a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ (a >>> 15), 1 | a); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; }; }
function place(gd: any, p: P3, yaw: number) { gd.char.pos = v3.copy(p); gd.char.vel = [0, 0, 0]; gd.speed = 0; gd.char.bodyYaw = gd.char.aimYaw = yaw; gd.char.aimPitch = 0; gd.path = []; gd.pathGoal = null; gd.char.update(0); gd.char.update(DT); }
function park(gd: any, i: number) { gd.hold = true; gd.pinned = true; gd.dazzledUntil = 1e9; gd.heardUpTo = 1e12; place(gd, [37.5, 0, 6 + i * 1.5], EAST); }
interface Stage { W: any; g: any; pl: any; A: any; msgs: string[]; ocps: number[]; step: (n?: number) => void }
/** a fresh stand-up on one seed: every man parked blind and deaf on the lot but the one to be held — the corridor man planted calm facing east, or (civ) the
 *  first standing civilian where she is — Sam 0.7 m behind with the pistol drawn or holstered, the grab made, the hold settled */
async function held(seed: number, opts: { drawn?: boolean; civ?: boolean } = {}): Promise<Stage | null> {
  Math.random = seeded(seed);
  const W: any = await hushed(() => standUp()); const g = W.game, pl = g.player;
  g.godMode = true; pl.visibility = 0.02; pl.slot = 'pistol'; pl.holstered = BREAK ? !!opts.drawn : !opts.drawn; pl.pistol.lightOn = false;
  const step = (n = 1) => { for (let i = 0; i < n; i++) { W.step(DT); pl.visibility = 0.02; } };
  step(5);
  const men = [...g.guards]; let A = men[0];
  if (opts.civ) { const cv = (g.civilians ?? []).find((x: any) => x.civ && x.state !== 'dead'); if (!cv) return null; A = cv; men.forEach((m: any, i: number) => park(m, i)); place(A, [A.char.pos[0], 0, A.char.pos[2]], EAST); }
  else { men.slice(1).forEach((m: any, i: number) => park(m, i)); place(A, [14, 0, 11.2], EAST); A.wp = 1; }
  g.teleportPlayer([A.char.pos[0] - 0.7, 0, A.char.pos[2]]); pl.char.bodyYaw = pl.char.aimYaw = EAST; W.cam.yaw = 0; W.cam.rebuild?.(); step(1);
  const msgs: string[] = []; const msg0 = g.msg.bind(g); g.msg = (t: string) => { msgs.push(t); return msg0(t); };
  const ocps: number[] = []; const ocp0 = g.ocp.bind(g); g.ocp = (t: any) => { ocps.push(g.time); return ocp0(t); };
  W.audioLog = [];
  await hushed(() => startGrab(g, A));
  for (let i = 0; i < 120 && pl.holding && pl.holding.phase !== 'held'; i++) step(1);
  step(30);   // (well past the reach: the hand is on its tool and no question is in the air)
  if (pl.holding?.phase !== 'held' || pl.holding.g !== A) return null;
  W.cursorAt([A.char.pos[0] + 6, 1.3, A.char.pos[2]]);   // (the reticle on open floor ahead of the pair: nothing of anybody's under it)
  return { W, g, pl, A, msgs, ocps, step };
}
/** what a run came to, as one line: the hold, the man, Sam, the gun, what was said and heard */
function sig(S: Stage): string {
  const { g, pl, A, W } = S, H = pl.holding, p = pl.char.pos, a = A.char.pos;
  return J({ hold: H ? [H.variant, H.phase, f3(H.t), f3(H.windT)] : null, man: [A.state, A.downKind, A.civ?.state ?? null], sam: [f3(p[0]), f3(p[2]), f3(pl.char.bodyYaw)], at: [f3(a[0]), f3(a[2])],
    gun: [pl.pistol.roundsReady, f3(pl.lastFireT), f3(g.ocpCooldown)], ocps: S.ocps.length, msgs: S.msgs.length, sounds: (W.audioLog as any[]).map(x => x.name).join(','), events: g.eventSeq });
}
const click = (S: Stage, bits: number) => { S.W.input.clicked |= bits; S.step(1); };
/** frames until the hold is over (−1 = not inside `cap`) */
function untilOver(S: Stage, cap: number, each?: (f: number) => void): number { for (let f = 1; f <= cap; f++) { S.step(1); each?.(f); if (!S.pl.holding) return f; } return -1; }
const rowLines = (S: Stage): string[] => S.g.nearby.filter((r: any) => r.kind === 'held').map((r: any) => String(r.line2));

export async function heldClicksCheck(): Promise<boolean> {
  const F = family("heldclicks: a man in his arm — the right click is the choke; the gun's hold and a civilian as they were");
  const rnd = Math.random;
  if (BREAK) console.log("HELDCLICKS_BREAK=1: Sam's hands are swapped (drawn for the arm hold, holstered for the gun's) — every line must fail");
  try {
    // ---- b: the arm hold, the right click
    { const S = await held(902); if (!S) F.verdict(false, 'b · staging', 'the arm hold never settled'); else {
        click(S, RMB); let chS = -1; const nS = untilOver(S, 400, f => { if (chS < 0 && S.pl.holding?.phase === 'choke') chS = f; });
        const A = S.A;
        F.verdict(chS > 0 && nS > 0 && A.state === 'dead' && isBreathing(A) && A.downKind === 'choked' && !S.pl.holding,
          'b · the arm hold, RMB: the arm closes, and he is out cold — breathing — with the hold over',
          `'choke' ${chS} frames on · out ${nS} frames on · ${A.callsign} ${A.state} / ${A.downKind} · breathing ${isBreathing(A)}`); } }

    // ---- d: the gun hold — LMB a round past him, RMB the OCP
    for (const [name, bits] of [['LMB', LMB], ['RMB', RMB]] as [string, number][]) {
      const S = await held(904, { drawn: true }), bad = !S || S.pl.holding.variant !== 'gun'; let on: { shots: number; ocps: number; phase: string; alive: boolean } | null = null;
      if (S && !bad) { const r0 = S.pl.pistol.roundsReady; click(S, bits); S.step(60); on = { shots: r0 - S.pl.pistol.roundsReady, ocps: S.ocps.length, phase: String(S.pl.holding?.phase), alive: S.A.state !== 'dead' }; }
      const did = bits === LMB ? on?.shots === 1 && on.ocps === 0 : on?.ocps === 1 && on.shots === 0;
      F.verdict(!!on && did && on.phase === 'held' && on.alive, `d · the gun hold, ${name}: ${bits === LMB ? 'one round past him' : 'the OCP'} (still held, the man alive)`,
        !on ? 'staging: the gun hold never settled' : `${on.shots} round(s), ${on.ocps} OCP · phase ${on.phase}`); }

    // ---- e: a held civilian — the clicks are never hers
    { const out: string[] = []; let who = '', bad = false, rows: string[] = [];
      for (const bits of [0, LMB, RMB]) { const S = await held(905, { civ: true }); if (!S) { bad = true; break; } who = `${S.A.callsign} (the ${S.A.civ.label})`; if (!bits) rows = rowLines(S); click(S, bits); S.step(90); out.push(sig(S)); }
      F.verdict(!bad && out.length === 3 && out[0] === out[1] && out[0] === out[2] && /"held"/.test(out[0] ?? '') && !rows.some(r => /LMB|RMB/.test(r)), 'e · a held civilian: LMB, RMB and no click are one and the same run (still held, alive), and her rows carry no marks',
        bad ? 'staging: no civilian on this stand, or she was never held' : `${who} · none = LMB ${out[0] === out[1]} · none = RMB ${out[0] === out[2]} · ${out[0]}`); }

  } finally { Math.random = rnd; }
  return F.done();
}

if (import.meta.main) {
  const ok = await asProbe('heldclicks', heldClicksCheck);
  console.log(ok ? '\n=== heldclicks: ALL PASS ===' : '\n=== heldclicks: FAILED ==='); process.exit(ok ? 0 : 1);
}
