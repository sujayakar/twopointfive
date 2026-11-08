// The 'calls' family — THE CALLS TAKEN WHILE THE MAINTAINER WAS AWAY (rebuild/away-build/calls.md): each open call an earlier page left for him, taken by
// judgment, shipped as a switch with its other side one flick away. Run through probes.ts:
//   bun run tools/qa/probes.ts calls | calls-shipped | calls-lastseen
// THE CALLS ARE SETTLED NOW (tools/qa/switches.json; 2026-11-07), and the family no longer holds an other side. Four probes went with it: calls-menu,
// calls-leave and calls-held walked the threshold's page behind X, 5, 4 and its held page with a row switch flicked (the page has neither: 'clear from
// outside' is one plain order on the tools' row 4), and calls-rally asked ROOMFIGHT.rallyK on a floor stood as one pool (the page deals every floor into
// squads). PASS / FAIL each, all headless. No picture here.
//   calls-shipped   every call of the list ships at its pick, and is a key the Team feel panel keeps
//   calls-lastseen  FOG.lastSeen turned up to 4: an enemy seen and then lost keeps a mark where he was last seen, ageing, gone after 4 s, never while he
//                   is in sight. What decides the mark while the row is 0 is the 'you're told' lever: tools/qa/told-check.ts
import { ROOT } from './headless.ts';
import { stage, quiet, guarded, family } from './thresholdmenu-probes.ts';
const { FOG } = await import(`${ROOT}/src/game/fogofwar.ts`);
const { SCENARIOS } = await import(`${ROOT}/src/scene/scenarioList.ts`);
const { feelRange, feelDefaults } = await import(`${ROOT}/src/game/teamfeel.ts`);
const lastSeen = (g: any): any[] => g.view.marks.seen.map((m: any) => ({ ...m, gd: g.guards.find((x: any) => x.char.id === m.man) }));   // (what the HUD is told)

const scn = (id: string) => SCENARIOS.find((e: any) => e.id === id);
const flat = (a: number[], b: number[]) => Math.hypot(a[0] - b[0], a[2] - b[2]);
/** THE LIST: table, key, the value it ships at, and the other side */
const SHIPPED: [string, string, unknown, string][] = [
  ['DOORTOOLS', 'thrHoldFirst', true, 'false: step in by itself on key 1'], ['DOORTOOLS', 'thrBangFirst', true, 'false: plain step in on key 1 (as built)'],
  ['DOORTOOLS', 'thrOutsideRow', true, 'false: clear from outside off the menu'], ['DOORTOOLS', 'thrLeaveFallIn', false, 'true: leave it = fall in'],
  ['DOORTOOLS', 'thrHandsOff', false, 'true: no what-next on the step-in form'], ['DOORTOOLS', 'thrHeldAll', false, 'true: held page only when all hold'],
  ['DOORTOOLS', 'thrMenuWord', true, 'false: F with another menu up is not the word'], ['DOORTOOLS', 'thrBangs', 2, '1: one can a room'], ['DOORTOOLS', 'thrLight', 1, '2: both jamb men lit'],
  ['DOORTOOLS', 'thrSlice', false, 'true: the opener slices'], ['DOORTOOLS', 'charges', 2, 'kit'], ['DOORTOOLS', 'bangs', 3, 'kit'],
  ['ROOMFIGHT', 'rally', true, 'false: nobody leaves his room'], ['ROOMFIGHT', 'rallyK', 1, 'under 1: fewer leave'], ['FOG', 'lastSeen', 0, 'seconds of a last-seen mark'],
  ['COVER', 'on', true, 'false: nobody watches openings unasked'], ['ORDERS', 'doorRay', true, 'false: the old pick'], ['LASER', 'enemy', false, 'true: enemies show lasers'],
];

// ---------------------------------------------------------------- every call ships at its pick
export const callsShipped = guarded(async () => {
  const { verdict, done } = family('calls-shipped: every call taken ships at its pick and is kept per machine');
  const D = feelDefaults() as Record<string, Record<string, unknown>>;
  for (const [t, k, want, other] of SHIPPED) { const got = D[t]?.[k], range = feelRange(t, k);
    verdict(got === want && range !== undefined, `${t}.${k} ships ${JSON.stringify(want)} and is kept per machine`, `shipped ${JSON.stringify(got)} · ${range === undefined ? 'NOT KEPT (teamfeel.ts)' : range === null ? 'a switch' : `a slider ${JSON.stringify(range)}`} · the other side — ${other}`); }
  return done();
});

// ---------------------------------------------------------------- the last-seen mark
// (until 2026-11-07 this ran with the 'you're told' lever held down, and a second line asked 'lastSeen 0: no mark'. The lever is settled up, and at 0 it
//  decides the mark: that is told-check.ts's to hold)
export const callsLastSeen = guarded(async () => {
  const { verdict, done } = family('calls-lastseen: FOG.lastSeen turned up — an enemy seen and then lost keeps a mark where he was last seen for that long');
  const E = scn('room-clear-pair');
  const run = async (secs: number) => {
    const R = await stage(E, 1), g = R.g; FOG.lastSeen = secs; g.fogOfWar.on = true; g.aiEnabled = false;   // (nobody moves or fights: the fog's own answer, a man seen and then not)
    const team = R.team; for (const m of team) m.dazzledUntil = 1e9;   // (the team's own eyes shut: Sam's is the only one, where the probe puts it)
    const start = [...g.player.char.pos], hostiles = g.guards.filter((x: any) => x.team === 'hostile' && x.state !== 'dead');
    await quiet(() => R.step(30)); const h = hostiles.find((x: any) => g.fogOfWar.hidden(x)); if (!h) return { ok: false, why: 'no defender out of the picture at the start' } as any; const hp = [...h.char.pos]; const before = lastSeen(g).length, hid0 = g.fogOfWar.hidden(h);
    // Sam put a stride from him (a ghost: not seen, AI off), then back where he started
    let near: number[] | null = null; for (const [dx, dz] of [[1.2, 0], [-1.2, 0], [0, 1.2], [0, -1.2], [0.9, 0.9], [-0.9, -0.9], [0.9, -0.9], [-0.9, 0.9]]) { const p = [hp[0] + dx, 0, hp[2] + dz]; if (!g.col.nav.isBlocked(p[0], p[2]) && !g.col.solidBlocked([p[0], 1.5, p[2]], [hp[0], 1.5, hp[2]])) { near = p; break; } }
    if (!near) return { ok: false, why: 'no floor beside the man' } as any;
    R.aim = [hp[0], 1.0, hp[2]]; g.teleportPlayer(near); await quiet(() => R.step(20)); const seen = g.fogOfWar.seen(h), during = lastSeen(g).length;
    R.aim = null; g.teleportPlayer(start); await quiet(() => R.step(45)); const lost = g.fogOfWar.hidden(h), marks = lastSeen(g), m = marks.find((x: any) => x.gd === h);
    const age1 = m?.age ?? -1, off = m ? flat(m.at, hp) : -1;
    await quiet(() => R.step(Math.round(Math.max(1, secs) * 60))); const after = lastSeen(g).length;
    return { ok: true, before, hid0, seen, during, lost, n: marks.length, every: marks.every((x: any) => g.fogOfWar.hidden(x.gd) && x.gd.state !== 'dead' && x.age < secs), age1, off, after };
  };
  const on = await run(4);
  if (!on.ok) { verdict(false, 'the staging', on.why); return done(); }
  verdict(on.before === 0 && on.hid0 && on.seen && on.during === 0 && on.lost && on.n >= 1 && on.every && on.age1 > 0.2 && on.age1 < 1.5 && on.off < 0.3 && on.after === 0,
    'lastSeen 4: unseen, no mark; in sight, no mark; lost, a mark where he stood, ageing (marks only on men out of the picture); gone inside four seconds more',
    `before ${on.before} (hidden ${on.hid0}) · Sam beside him: seen ${on.seen}, marks ${on.during} · Sam gone: hidden ${on.lost}, marks ${on.n}, ${on.age1.toFixed(2)} s old, ${on.off.toFixed(2)} m from where he stands · 4 s on: ${on.after}`);
  return done();
});
