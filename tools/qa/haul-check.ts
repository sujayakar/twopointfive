// 'haul' — THE CHOKE HAULS HIM UP AND BACK (TAKEDOWN.haul, character.ts HOLD.haul). The maintainer, off the stealth tutorial mission: "the choking animation
// needs to be more intense. in chaos theory sam has him in a chokehold and then lifts him up and back out of the air a bit when choking". The choke was posed
// standing: the elbow closed, Sam leaned back four degrees, and the man sagged on his own two feet until he was lowered.
//   bun run tools/qa/haul-check.ts        (also `probes.ts haul`)
// The key is settled on: the check asks the page's side alone and writes no switch. Every line that held the haul against the same choke with the key down
// went with that run (his feet, his head and Sam's neck against where the key down has them; the heads never closer than with it down; set down before he
// is lowered; the key put down mid-haul; the hold broken and a man grabbed again), and so did the harness part. What is left holds a number of its own:
//   heads     at the haul's height (0.4, 0.65 and 0.9 s in) the two heads are over 22 cm apart (a first cut lifted his cap through Sam's head).
//   pops      the whole choke: no joint of either man moves more than 5 cm in a frame.
// HAUL_BREAK=1 = the man is never grabbed, so no choke runs: the staging line must FAIL.   exit 0 = all hold · 1 = one does not
// THE HARNESS HAS NO GPU: bones are read, nothing is drawn. Pictures of the same run: tools/qa/close-stills.ts --scene choke.
import { standUp, ROOT } from './headless.ts';
import { asProbe } from './holds.ts';   // (run alone, each part runs as the probe the runner calls it by: one stand both ways)

const { startGrab } = await import(`${ROOT}/src/game/player.ts`);

type P3 = number[];
const BREAK = process.env.HAUL_BREAK === '1', DT = 1 / 60, EAST = Math.PI / 2, LOBBY: P3 = [10.2, 0, 19.5];
const f3 = (n: number) => n.toFixed(3), cm = (n: number) => (n * 100).toFixed(1);
let allPass = true;
const verdict = (ok: boolean, label: string, detail: string) => { allPass &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); };
async function quiet<T>(fn: () => Promise<T>): Promise<T> { const log = console.log; console.log = () => {}; try { return await fn(); } finally { console.log = log; } }
function seeded(a: number) { return () => { a |= 0; a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ (a >>> 15), 1 | a); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; }; }

interface Frame { t: number; feet: [number, number]; head: number; samBack: number; apart: number; wrist: number; hands: [number, number]; roots: P3; joints: Float32Array[]; bones: Record<string, P3>[] }
/** the lobby's open floor (close-stills.ts's staging): one man stood facing east, Sam behind him, the grab, a second and a half held, the choke — a frame
 *  read every frame of it from the frame the choke's phase starts. */
async function chokeRun(secs = 3): Promise<{ frames: Frame[]; outAt: number }> {
  Math.random = seeded(7303); const W: any = await quiet(() => standUp()), g = W.game;
  g.godMode = true; g.aiEnabled = false; g.player.visibility = 0.02; g.player.slot = 'pistol'; g.player.holstered = true;
  for (let i = 0; i < 5; i++) W.step(DT);
  const place = (gd: any, p: P3) => { gd.char.pos = [...p]; gd.char.vel = [0, 0, 0]; gd.speed = 0; gd.char.bodyYaw = gd.char.aimYaw = EAST; gd.char.aimPitch = 0; gd.path = []; gd.pathGoal = null; gd.hold = true; gd.pinned = true; gd.char.update(0); gd.char.update(DT); };
  const [A, ...rest] = g.guards; rest.forEach((gd: any, i: number) => { place(gd, [37.5, 0, 6 + i * 1.5]); gd.heardUpTo = 1e12; }); place(A, LOBBY); A.wp = 1;
  W.cam.yaw = 0; W.cam.rebuild?.();
  const pl = g.player.char, step = (n = 1) => { for (let i = 0; i < n; i++) { W.cursorAt([pl.pos[0] + Math.sin(pl.bodyYaw) * 6, 1.0, pl.pos[2] + Math.cos(pl.bodyYaw) * 6]); W.step(DT); g.player.visibility = 0.02; } };
  g.teleportPlayer([A.char.pos[0] - 0.8, 0, A.char.pos[2]]); g.player.crouch = true; pl.bodyYaw = pl.aimYaw = EAST; W.cursorAt([A.char.pos[0], 1.2, A.char.pos[2]]); for (let i = 0; i < 60; i++) { W.cursorAt([A.char.pos[0], 1.2, A.char.pos[2]]); W.step(DT); g.player.visibility = 0.02; }
  if (!BREAK) startGrab(g, A); step(90);
  const at = (c: any, node: string): P3 => { const p = c.skel.nodePos(c.anim.nodes[node]), w = c.world; return [w[0] * p[0] + w[4] * p[1] + w[8] * p[2] + w[12], w[1] * p[0] + w[5] * p[1] + w[9] * p[2] + w[13], w[2] * p[0] + w[6] * p[1] + w[10] * p[2] + w[14]]; };
  const frames: Frame[] = []; let outAt = -1, t = -1;
  if (!g.selectVerb('held', 'choke')) return { frames, outAt }; W.input.pressed.add('Space');
  for (let f = 0; f < (secs + 0.6) * 60; f++) {
    const H = g.player.holding; if (t < 0 && H && H.phase === 'choke') t = 0;
    step();
    if (t < 0) continue; t += DT; if (outAt < 0 && !g.player.holding) outAt = t;
    const hn = at(pl, 'head'), hh = at(A.char, 'head'), nk = at(A.char, 'neck'), wr = at(pl, 'handL'), sn = at(pl, 'neck'), hl = at(A.char, 'handL'), hr = at(A.char, 'handR'), dist = (a: P3, b: P3) => Math.hypot(a[0] - b[0], a[1] - b[1], a[2] - b[2]);
    frames.push({ t, feet: [at(A.char, 'footL')[1], at(A.char, 'footR')[1]], head: hh[1], samBack: pl.pos[0] - sn[0], apart: Math.hypot(hn[0] - hh[0], hn[1] - hh[1], hn[2] - hh[2]), wrist: Math.hypot(wr[0] - nk[0], wr[1] - nk[1], wr[2] - nk[2]), hands: [dist(hl, wr), dist(hr, wr)],
      roots: [...pl.pos, ...A.char.pos], joints: [Float32Array.from(pl.skel.jointData), Float32Array.from(A.char.skel.jointData)], bones: [pl, A.char].map((c: any) => Object.fromEntries(Object.keys(c.anim.nodes).filter(k => typeof c.anim.nodes[k] === 'number').map(k => [k, at(c, k)]))) });
    if (t > secs) break;
  }
  return { frames, outAt };
}
const frameAt = (R: { frames: Frame[] }, t: number) => R.frames[Math.min(R.frames.length - 1, Math.max(0, Math.round(t * 60) - 1))];

async function haul() {
  console.log('\n=== heads · pops: the choke as the page runs it ===');
  const on = await chokeRun();
  if (on.frames.length < 150) { verdict(false, '(staging) the choke ran', `frames ${on.frames.length}`); return; }
  { let least = Infinity; const n = Math.round(1.5 * 60); for (let i = 0; i < n; i++) least = Math.min(least, on.frames[i].apart);
    const top = Math.min(frameAt(on, 0.4).apart, frameAt(on, 0.65).apart, frameAt(on, 0.9).apart);
    verdict(top >= 0.22, 'the two heads are over 22 cm apart at the haul\'s height (0.4, 0.65 and 0.9 s in)', `at its height ${cm(top)} cm · closest in the first second and a half ${cm(least)} cm · his feet ${[0.4, 0.65, 0.9].map(t => `${cm(Math.min(...frameAt(on, t).feet))} cm`).join(', ')} up, his head ${[0.4, 0.65, 0.9].map(t => f3(frameAt(on, t).head)).join(', ')} m`); }
  { let pop = 0, at = ''; const end = Math.round(on.outAt * 60) - 2;
    for (let i = 1; i < end; i++) for (const w of [0, 1]) { const a = on.frames[i].bones[w], b = on.frames[i - 1].bones[w]; for (const k of Object.keys(a)) { const d = Math.hypot(a[k][0] - b[k][0], a[k][1] - b[k][1], a[k][2] - b[k][2]); if (d > pop) { pop = d; at = `${w ? 'the man' : 'Sam'}'s ${k}, ${f3(on.frames[i].t)} s in`; } } }
    verdict(pop <= 0.05 && end > 100, 'the whole choke: no joint of either man moves more than 5 cm in a frame', `fastest ${cm(pop)} cm (${at}) · he is out ${f3(on.outAt)} s in`); }
}

/** the whole card (probes.ts 'haul'): true = every check passed */
export async function haulCheck(): Promise<boolean> {
  const rnd = Math.random; allPass = true;
  try { await haul(); }
  finally { Math.random = rnd; }
  console.log(`\n=== haul: ${allPass ? 'ALL PASS' : 'FAILURES above'} ===`); return allPass;
}
if (import.meta.main) process.exit(await asProbe('haul', haulCheck) ? 0 : 1);
