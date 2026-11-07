// tools/qa/traildoors-check.ts — a follower goes by the doors Sam took (orders.ts ORDERS.trailDoors: trailWay).
//
// Seen on the registry level's walk with four behind Sam, on the page's stand: at the copy room's gallery door the lead follower gives way to the leaf,
// the men behind him go in past him, and by the time it is his turn Sam is out of the room's far door and into the hall. His place is in the hall,
// the hall is nearer by the gallery than through the copy room, and he walks round the room the file went through, apart from his partner. With the
// rule he walks Sam's trail to his place: in by the gallery door and out by the hall door.
//
// The fault on the walk as it falls needs the page's own door rules to make him lose his turn, so the walk here is staged by hand:
// the lead follower is HELD where he stands from the frame Sam steps into the copy room until Sam is well into the hall, then let go.
//   held and let go: he walks through the copy room, and ends with Sam. (The hand-out itself, orders.ts trailWay, is the Rust game's: no check here can wrap it.)
//
//   bun run tools/qa/traildoors-check.ts          (also `probes.ts trail-doors`, family registry)
// What it cannot say: one level, one seed, the script's Sam at a walk. Not staged: a locked door on the trail (the walk unlocks every door), and an
// opening with no door in it, which the rule does not count as a way of its own. Never seen on a GPU, nobody has played it.
import { ROOT } from './headless.ts';
import { guarded, family } from './thresholdmenu-probes.ts';
const { walk } = (await import('./registry-probes.ts')) as any;
const { REGISTRY_ROOMS } = (await import(`${ROOT}/tools/levels/registry.ts`)) as any;
const { knobsCopy, knobsBack } = (await import(`${ROOT}/src/game/knobs.ts`)) as any;

type Rect = { x0: number; x1: number; z0: number; z1: number };
const inRoom = (p: number[], r: Rect) => p[0] > r.x0 && p[0] < r.x1 && p[2] > r.z0 && p[2] < r.z1;
const flat = (a: number[], b: number[]) => Math.hypot(a[0] - b[0], a[2] - b[2]);
const COPY: Rect = REGISTRY_ROOMS.copy, HALL: Rect = REGISTRY_ROOMS.hall;
class Enough extends Error { }
const AFTER = 9;   // seconds watched after he is let go

interface Held { who: string; heldS: number; inCopy: number; gap: number; at: number[]; g: any; from: number[] }
/** the walk up to the copy room, the lead follower held from the frame Sam is in it until Sam is a metre and a half into the hall, then AFTER seconds more */
async function held(): Promise<Held> {
  const H: Held = { who: '', heldS: 0, inCopy: 0, gap: 99, at: [0, 0, 0], g: null, from: [0, 0, 0] }; let lead: any = null, pin: number[] | null = null, phase = 0, t1 = 0;
  try { await walk(4, (g: any, men: any[]) => {
    const sam = g.player.char.pos, O = g.orders;
    if (!H.g) H.g = g;
    if (phase === 0 && inRoom(sam, COPY)) { phase = 1; lead = men.find((m: any) => O.of(m)?.follow === 0) ?? men[0]; H.who = lead.callsign; pin = [...lead.char.pos]; H.from = [...pin]; }
    if (phase === 1) { const c = lead.char; c.pos[0] = pin![0]; c.pos[2] = pin![2]; c.vel[0] = c.vel[2] = 0; H.heldS += 1 / 60;
      if (inRoom(sam, HALL) && sam[0] > HALL.x0 + 1.5) { phase = 2; t1 = g.time; } }
    if (phase === 2) { if (inRoom(lead.char.pos, COPY)) H.inCopy++; H.gap = flat(lead.char.pos, sam); H.at = [...lead.char.pos]; if (g.time - t1 > AFTER) throw new Enough(); }
  }); } catch (e) { if (!(e instanceof Enough)) throw e; }
  return H;
}

export const trailDoorsCheck = guarded(async () => {
  const F = family('trail-doors: a follower goes by the doors Sam took');
  const knobs0 = knobsCopy(), rnd0 = Math.random, log = console.log;
  try {
    const say = (h: Held) => `${h.who} held ${h.heldS.toFixed(1)} s at (${h.from[0].toFixed(1)}, ${h.from[2].toFixed(1)}) · in the ${AFTER} s after: ${(h.inCopy / 60).toFixed(1)} s in the copy room, ${h.gap.toFixed(1)} m from Sam at the end`;
    const on = await held();
    F.verdict(on.heldS > 1 && on.inCopy > 30 && on.gap < 5.5, 'the lead follower held at the copy room\'s door and let go with Sam in the hall: he walks through the copy room, and ends with Sam', say(on));
  } finally { console.log = log; knobsBack(knobs0); Math.random = rnd0; }
  return F.done();
});

if (import.meta.main) process.exit((await trailDoorsCheck()) ? 0 : 1);
