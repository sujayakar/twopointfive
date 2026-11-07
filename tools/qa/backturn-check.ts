// tools/qa/backturn-check.ts — a man walking backwards on the door behind, let go of, keeps his way while he turns
// (cover.ts COVER.backTurn: turnFrom / turnOut).
//
// Seen on the registry level's walk with four behind Sam: the rear man walks backwards with his gun on the opening
// behind; the deal lets go of that opening (a wall is in its line now, or a mate has it); the order's own pose is
// 'walk forward', and a man walks forward along his BODY, which is still turned to the rear. He went a quarter of a
// metre back the way he came, hooked round and went on, three quarters of a second gone, every time. With the rule
// his feet keep the path while his body comes round, then he walks forward.
//
//   (1) the registry walk with four: he turns on his way — the turn-out is used, no let-go ends more than 0.12 m against his path;
//   (2) and the walk holds: every man ends with Sam, stuck man-frames under the walk's own one percent.
//
// SINCE 2026-11-07 the rule is settled on and this check does not write it: the stand's line, the "rule off: the fault
// is there" walk and what the rule-on walk was compared with (a quarter of those metres, no less path made), and the
// recording from before the key and the page's ?backturn=0 are out.
//
//   bun run tools/qa/backturn-check.ts [--verbose]
//   BACKTURN_BREAK=1 …                                    (the walk is made with one teammate, who is nobody's rear man: line (1) must fail)
import { guarded, family } from './thresholdmenu-probes.ts';
const { walk } = (await import('./registry-probes.ts')) as any;
const BREAK = !!process.env.BACKTURN_BREAK, VERBOSE = process.argv.includes('--verbose');
const SECS = 0.8;   // how long after a let-go his steps are read against the way he was going

interface Go { who: string; t: number; against: number; along: number }
interface Out { w: any; goes: Go[]; turned: number }
/** the registry walk with four behind Sam: every let-go of a man walking backwards with his body turned from his path (his job gone, nothing kept), and the metres he then
 *  walks AGAINST his own path (each frame's step, on the line from where he stood to the corner of his path he was walking at), summed over SECS. Against
 *  the PATH, not the way he was going when let go: Sam turning round turns the file's paths round, which is what lets go of most of these openings */
async function run(): Promise<Out> {
  const was = new Map<any, { back: boolean; away: boolean; wp: number[] | null; pos: number[] }>(), open: { m: any; go: Go; t0: number }[] = [], goes: Go[] = []; let G: any = null;
  const w = await walk(BREAK ? 1 : 4, (g: any, men: any[]) => {
    G = g;
    const C = g.cover as any, t = g.time;
    for (const m of men) { const J = C.jobs.get(m), k = C.kept.get(m), b = was.get(m);
      if (b?.back && b.away && !J && !k && m.state !== 'dead') { const go = { who: m.callsign, t, against: 0, along: 0 }; goes.push(go); open.push({ m, go, t0: t }); } }
    for (let i = open.length - 1; i >= 0; i--) { const o = open[i], b = was.get(o.m)!, p = o.m.char.pos;
      if (b.wp) { const dx = b.wp[0] - b.pos[0], dz = b.wp[2] - b.pos[2], L = Math.hypot(dx, dz); if (L > 0.15) { const a = ((p[0] - b.pos[0]) * dx + (p[2] - b.pos[2]) * dz) / L; o.go.along += a; if (a < 0) o.go.against -= a; } }
      if (t - o.t0 > SECS) open.splice(i, 1); }
    for (const m of men) { const J = C.jobs.get(m), k = C.kept.get(m), p = m.char.pos, wp = m.path?.[m.pathI]; 
      was.set(m, { back: !!(J ?? k?.J)?.back && !!m.script?.strafe, away: !!wp && Math.cos(m.char.bodyYaw - Math.atan2(wp[0] - p[0], wp[2] - p[2])) < 0.7, wp: wp ? [wp[0], wp[1], wp[2]] : null, pos: [p[0], p[1], p[2]] }); }
  });
  return { w, goes, turned: G.cover.stats.turned };
}

export const backturnCheck = guarded(async () => {
  const F = family('backturn: a man walking backwards on the door behind, let go of, keeps his way while he turns');
  const rnd0 = Math.random;
  try {
    const on = await run();
    const sum = (o: Out) => o.goes.reduce((a, x) => a + x.against, 0), worst = (o: Out) => o.goes.reduce((a, x) => Math.max(a, x.against), 0), made = (o: Out) => o.goes.reduce((a, x) => a + x.along, 0) / Math.max(1, o.goes.length), say = (o: Out) => `${o.goes.length} let-goes · ${sum(o).toFixed(2)} m against his path in all · worst ${worst(o).toFixed(2)} m · ${made(o).toFixed(2)} m of path made in the ${SECS} s after, on average · the turn-out used ${o.turned} times`;
    if (VERBOSE) console.log(`    ${on.goes.map(x => `${x.t.toFixed(1)} ${x.who} ${x.against.toFixed(2)}`).join(' · ')}`);
    F.verdict(on.goes.length >= 10 && on.turned >= 10 && worst(on) <= 0.12, 'the registry walk with four: a man let go of mid back-pedal turns on his way — ten let-goes or more, the turn-out used as often, and no let-go worse than 0.12 m against his path', say(on));
    const st = (o: Out) => 100 * o.w.stuck / (o.w.n * o.w.frames), fin = (o: Out) => `${o.w.secs.toFixed(1)} s · ends ${o.w.end.map((d: number) => d.toFixed(2)).join(' / ')} m from Sam · stuck ${st(o).toFixed(2)} % · two within half a metre on ${o.w.jam} frames${o.w.short ? ` · ${o.w.short.slice(0, 120)}` : ''}`;
    F.verdict(on.w.alive && !on.w.short && on.w.end.every((d: number) => d < 5.5) && on.w.doorway.length === 0 && st(on) < 1, 'and the walk holds: Sam gets everywhere, every man ends with him, stuck man-frames under the walk\'s own one percent (a rear man stood on his own place far behind a stopped Sam: the file\'s length, not this rule)', fin(on));
  } finally { Math.random = rnd0; }
  return F.done();
});

if (import.meta.main) process.exit((await backturnCheck()) ? 0 : 1);
