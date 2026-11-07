// The 'by-eye' probe: LIT TO WHOM (sneak.ts SNEAK.byEye — litSeen). Run through probes.ts:
//   bun run tools/qa/probes.ts by-eye        (family: sneak)
// PASS / FAIL each, headless. The pair-up rig's open strip at night (pairup-probes.ts rig): one defender stood looking down it, the partner at its far
// end, and the partner's OWN METER planted the way a GPU page reads an open night shadow (the software renderer and the CPU's estimate read it near 0; the
// maintainer's HUD showed 14 to 27 of 100). The probe asks the sneak's own map (sneakWalk: what the partner's way to a man is planned over) which steps
// down the strip are WATCHED and what a walk across it costs, then gives the order and watches his feet:
//   · only the steps near enough for THAT eye to gain on a crouched man are watched — no further than the guards' own arithmetic allows for the meter
//     planted (the probe's own sum); the brighter the meter the further out
//   · a bright meter is watched all the way out · a meter under the lit number is watched nowhere
//   · a walk across the far end costs nothing; a walk across the near end costs something
//   · an upright man is watched further out than a crouched one (the eye's own posture term)
//   · THE FOLLOWER (the office's corridor, the pair squadded up and silent as sneak-probes.ts has them; one defender stood at its far end looking down it,
//     every meter never under 0.16, Sam walked low to 8.7 m from him): the partner comes up to his place behind Sam
//   · the pair lesson itself (tut-pair, its scripted player, hard), Sam's meter never under 0.27 as the lesson runner's --floor has it, plays to its end
// The rule is settled on (SNEAK.byEye, and SNEAK.stalkAmbient with it): the probe asks the page's side alone and writes neither. Until 2026-11-07 every
// strip was read twice, switch off and on ("with the switch off the flaw is there", "none that was not watched before", "either way", the follower hanging
// back with it off, "off, nothing reads its number"), and two lines asked the stand's word and a recording from before the key: out.
// What it cannot say: what a GPU reads (the meter here is the probe's hand; nothing was seen on a GPU, nobody has played it). Not staged: a kit card that
// makes a man plainer to see (kitload.ts loadSeen, counted in the gate), an alerted eye, a door frame's funnel.
// The probe's own red path: BYEYE_BREAK=1 bun run tools/qa/probes.ts by-eye must FAIL. The word is the PROBE'S now (the game's copy read it in sneak.ts,
// which no longer plays): with it set the meter the probe plants is half a unit brighter than the line says, so the dim strips are watched all the way out.
import { ROOT } from './headless.ts';
import { rig } from './pairup-probes.ts';
const SN: any = await import(`${ROOT}/src/game/sneak.ts`);
const SNEAK = SN.SNEAK;
const BREAK = process.env.BYEYE_BREAK === '1';

type Verdict = (ok: boolean, label: string, detail: string) => void;
function family(title: string): { verdict: Verdict; done: () => boolean } {
  console.log(`\n=== ${title} ===`); let all = true;
  return { verdict: (ok, label, detail) => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); }, done: () => { console.log(`=== ${title.split(':')[0]}: ${all ? 'ALL PASS' : 'FAILURES above'} ===`); return all; } };
}
const flat = (a: number[], b: number[]) => Math.hypot(a[0] - b[0], a[2] - b[2]);
const WEST = -Math.PI / 2, EAST = Math.PI / 2, Z = 1.5, DX = 10.5, STEP = 0.5, FAR = 15.5;
/** the guards' own arithmetic for a crouched man (guards.ts sightGainOn, as sneak.ts eyeRate has it), by the probe's own hand: the furthest a man this lit is gained on */
function reach(g: any, lit: number, floor: number): number {
  let far = 0; for (let d = 1.8; d <= 16; d += 0.05) if (g.tune.detectRate * Math.pow(lit, 1.5) * Math.max(0.2, Math.min(1, 1 - (d - 3) / 13)) * 0.75 > floor) far = d; return far; }

interface Stage { R: any; g: any; m: any; F: any; D: any; H: { v: number | null } }
/** the watcher D looking east down the strip from DX; the other man F stood behind him, looking away; the partner at the strip's far end */
async function stage(): Promise<Stage> { const fx = 4.5, mx = 26.6;
  const R: any = await rig({ up: true }), g = R.game, m = R.mate, F = R.tango(1), D = R.tango(2); R.only(F, D);
  R.pin(F, { at: [fx, 0, Z], yaw: WEST, state: 'patrol', aw: 0 }); R.pin(D, { at: [DX, 0, Z], yaw: EAST, state: 'patrol', aw: 0 }); g.teleportPlayer([mx + 3, 0, Z]); R.put(m, [mx, 0, Z], WEST);
  const id = m.char.id, la = g.lightAt, H = { v: null as number | null }, E = (v: number) => g.tune.visFloor * Math.exp(v / g.tune.visSlope);
  g.lightAt = (k: string) => H.v !== null && (k === `unitC${id}` || k === `unitH${id}`) ? E(H.v) : la(k);   // (the partner's meter, planted: what the renderer would answer for him)
  R.step(60 * 4); return { R, g, m, F, D, H };
}

export async function byEye(): Promise<boolean> {
  const F = family('by-eye: a lit step is seen only where the eye that has the line would gain on a man there');
  {
    const S = await stage(), { g, m } = S, EV = (v: number) => g.tune.visFloor * Math.exp(v / g.tune.visSlope);
    const strip = (v0: number, low = true) => { const v = BREAK ? Math.min(1, v0 + 0.5) : v0; m.visibility = v; m.irradiance = EV(v); const W = SN.sneakWalk(g, m, low); const row: boolean[] = [];
      for (let d = 2; d <= FAR + 1e-6; d += STEP) row.push(!!W.watched(DX + d, Z));
      const walk = (d: number) => W.cost([[DX + d, 0, Z - 0.3], [DX + d, 0, Z + 0.9]], 1.0) as number;
      let far = 0; row.forEach((w, i) => { if (w) far = 2 + i * STEP; }); return { row, far, n: row.filter(Boolean).length, near: walk(6), end: walk(14) }; };
    const txt = (r: { row: boolean[] }) => r.row.map(w => w ? 'W' : '.').join('');
    const dark = strip(0.05), dim = strip(0.27), mid = strip(0.4), bright = strip(0.7);
    const all = dim.row.length, cap = reach(g, 0.27, SNEAK.seenRate), capMid = reach(g, 0.4, SNEAK.seenRate);
    F.verdict(Math.abs(dim.far - cap) <= STEP && dim.row.every((w, i) => w === (2 + i * STEP <= dim.far)), 'a dim meter (0.27): only the near steps are watched, as far as the guards\' own arithmetic gains on a crouched man this lit and no further',
      `2 to ${FAR} m in front of the watcher, a step every ${STEP} m: ${txt(dim)} · watched out to ${dim.far} m (the probe's own sum for a meter of 0.27 over a rate of ${SNEAK.seenRate}: ${cap.toFixed(1)} m)`);
    F.verdict(mid.far > dim.far && Math.abs(mid.far - capMid) <= STEP, '… the brighter the meter the further out (0.4)', `${txt(mid)} · out to ${mid.far} m (the probe's sum: ${capMid.toFixed(1)} m)`);
    F.verdict(bright.n === all && dark.far < 3, 'a bright meter (0.7) is watched all the way out; a meter under the lit number (0.05) nowhere past his elbow', `0.7: ${bright.n} of ${all} · 0.05: ${txt(dark)}`);
    F.verdict(dim.end === 0 && dim.near > 0, 'a walk across the far end of his view (14 m) costs nothing; across the near end (6 m) something', `at 14 m: ${dim.end.toFixed(3)} · at 6 m: ${dim.near.toFixed(3)}`);
    { const up = strip(0.27, false);
      F.verdict(up.far > dim.far && up.far < FAR, 'an upright man is watched further out than a crouched one', `upright ${txt(up)} out to ${up.far} m · crouched out to ${dim.far} m`); }
    // … THE FOLLOWER: the office's corridor, one defender at its far end looking down it, every meter never under FLOOR
    const follow = async () => { const { Run, squadUp }: any = await import('./pairsneak.ts'), FLOOR = 0.16, log = console.log; console.log = () => { };
      try { const R: any = await Run.start('roomclear=10&squad=1', 3, { sneak: true }); R.stopOnNotice = false; squadUp(R); const g2 = R.g;
        const v0 = g2.visibilityOf.bind(g2); g2.visibilityOf = (E: number) => Math.max(FLOOR, v0(E));   // (what the page's meter reads in an open night shadow, for every meter and every estimate)
        const H = R.hostiles[4], c = H.char, at = R.free([29.5, 0, 11.2]); Object.defineProperty(c, 'chestDir', { get: () => [-1, 0, 0], set: () => { }, configurable: true }); c.headDir = () => [-1, 0, 0];
        let peak = 0, held = 0; const step0 = R.step.bind(R);
        R.step = (n = 1) => { for (let i = 0; i < n; i++) { c.pos[0] = at[0]; c.pos[2] = at[2]; c.vel[0] = c.vel[2] = 0; H.aimYaw = WEST; step0(1); peak = Math.max(peak, H.awareness ?? 0);
          const m2 = R.mate; if (m2 && SN.sneaking(g2, m2) && SN.sneakWord(g2, m2)) held++; } };
        R.crouch(true); const there = !!R.go(R.free([21, 0, 11.2]), 60); held = 0; R.wait(20); const m2 = R.mate;
        return { there, behind: flat(m2.char.pos, R.pos), eye: flat(m2.char.pos, at), sam: flat(R.pos, at), held: held / 60, peak, word: String(SN.sneakWord(g2, m2)), noticed: !!R.noticed };
      } finally { console.log = log; } };
    const fOn = await follow();
    const say = (o: Awaited<ReturnType<typeof follow>>) => `${o.behind.toFixed(1)} m behind Sam, ${o.eye.toFixed(1)} m from the eye, held ${o.held.toFixed(0)} of the 20 s Sam stood (${o.word}); the defender's awareness peaked ${o.peak.toFixed(2)}`;
    F.verdict(fOn.there && fOn.behind <= 3 && fOn.held < 2 && !fOn.noticed, 'the follower, down a corridor a defender looks along, every meter never under 0.16: he comes up behind Sam, and the defender does not notice',
      `Sam low, ${fOn.sam.toFixed(1)} m from the defender · ${say(fOn)}`);
    // … and the pair lesson played by its script with Sam's meter never under 0.27 (the page's reading of a low wall's shadow at night on the high
    // graphics preset is about 0.25; the lowest preset and the headless estimate read far under it)
    const lesson = async () => { const { runLesson } = await import('./lesson-run.ts');
      const log = console.log; console.log = () => { };
      try { const out = await runLesson('tut-pair', { hard: true, floor: 0.27 }); return { code: out.code as number, steps: out.steps.length as number, secs: out.secs as number }; }
      finally { console.log = log; } };
    const lOn = await lesson();
    F.verdict(lOn.code === 0, 'the pair lesson with Sam\'s meter never under 0.27 plays to its end', `${lOn.code === 0 ? 'played to its end' : 'did NOT end'} (${lOn.steps} steps, ${lOn.secs.toFixed(0)} s)`);
    return F.done();
  }
}
