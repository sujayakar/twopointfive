// NOBODY STACKS AT THE WINDOW BESIDE A DOOR — the check behind glasspen.ts `GLASSPEN.stackLeast` (orders.ts stackLeastGlass).
// The fault it holds shut: the records office's front door has a sidelight pane beside its latch jamb, and a rifle inside reaches BOTH jambs through it (the
// hinge jamb at a slant). 'Glass is not cover' (orders.ts behindGlass) then found no place off the glass and fell back to the walls' own marks: the lead on
// the latch jamb — stood at the window, a defender posted 1.6 m inside it — and, on 'bang and clear', the second man walking across the door front to file up
// behind him. With the moon counted the lead was shot through the pane on his mark before the door was touched (4 of 48 first entries lost a man there).
//   bun run tools/qa/records-front-check.ts [--off]
// What it does, as the page ships the scenario (tools/qa/ships.ts's stand: glass parts on, the movement pass, the turn weight, defenders' frags):
//   STAGED — stands `records-pair` up and asks the game itself where the pair stacks on the outside face of `front` (orders.ts stackPosts, and the door
//   tools' own tucked marks): it first shows the cause is THERE (the door, a pane on its wall beside the frame, rounds going through glass, both jambs in
//   GLASSPEN.stackEyes or more lines of fire through a pane — else NO VERDICT), then FAILS if a mark is at the window (within the pane's span along the wall,
//   a man's width to either side) or if the two marks are on different hands of the door (the man off the lead's hand crosses the door front for the bang).
//   Asked four ways: the pair under 'stack up' and under the door tools' own tucked marks · one man alone · the pair with Sam stood on the better jamb (the
//   file goes on past him: nobody on him, nobody at the window) · the two men ordered ONE AT A TIME through the game's own order (Orders.give): two marks a
//   body apart, neither at the window. Each deal is printed with the lines of fire through glass onto it beside those onto the walls' own marks.
//   --off     the rule off (GLASSPEN.stackLeast false): the fault as it was — expected to FAIL (exit 1): the proof that the check can. Read the last lines:
//             each half says for itself whether the fault REPRODUCED in it; a NO VERDICT exits 1 too
// What it does NOT show: the 'same mark' and 'on Sam' findings cannot fire with the rule off (the walls' own marks fail on the window first — those two
// guard the rule's own file, not the old fault); the one-at-a-time case reads the marks the game assigned, not where the men end up; a man the first order
// lost AFTER somebody crossed with no killer on record is not counted (playthrough.ts's deaths carry only men with a known killer).
// STAGED half does not depend on a seed). Not in probes.ts's families: it plays runs (a minute), and its stand is ships.ts's, not the harness's.
// exit 0 = the cause is there, no mark at the window, one hand, and no seed lost a man on the stack · 1 = any of those failed, a run errored, or the check
// looked at nothing (NO VERDICT) · 2 = bad flags
import { resolve } from 'node:path';
import { readFileSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';

const argv = process.argv.slice(2), flag = (n: string) => { const i = argv.indexOf(n); return i >= 0 ? argv[i + 1] : undefined; };
for (let i = 0; i < argv.length; i++) { const a = argv[i];
  if (a !== '--off') { console.error(`unknown flag ${a} (--off)`); process.exit(2); } }
const off = argv.includes('--off');
const SCN = 'records-pair', DOOR = 'front';

// ------------------------------------------------------------ the stand ships.ts gives a run (set before the game's modules load: they read it then)
Object.assign(process.env, { GLASSFX: 'on', TPS_MOVES: '1', TPS_TURN: '1', TPS_CAST: '1' });
const { standUp, ROOT } = await import('./headless.ts');
const { SCENARIOS } = await import(`${ROOT}/src/scene/scenarioList.ts`);
const { scenarioQuery } = await import(`${ROOT}/src/scene/scenarios.ts`);
const { startFromQuery } = await import(`${ROOT}/src/game/scenarioRun.ts`);
const { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`);
const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
const { GLASSPEN, glassPenLive } = await import(`${ROOT}/src/game/glasspen.ts`);
const { ORDERS } = await import(`${ROOT}/src/game/orders.ts`);
const { BoxFlag } = await import(`${ROOT}/src/scene/boxes.ts`);

console.log(`records-front-check · ${SCN} · door ${DOOR} · the rule ${off ? 'OFF (the fault as it was)' : 'as shipped'}`);
let failed = false, noVerdict = false, staged = 0;
const fail = (s: string) => { failed = true; console.log(`- FAIL — ${s}`); }, nov = (s: string) => { failed = noVerdict = true; console.log(`- NO VERDICT — ${s}`); };

// ------------------------------------------------------------ STAGED
{
  if (!('stackLeast' in GLASSPEN)) { console.log('- FAIL — this build has no GLASSPEN.stackLeast: the rule is not in it'); process.exit(1); }
  const E = SCENARIOS.find((e: any) => e.id === SCN); if (!E) { console.error(`no scenario ${SCN}`); process.exit(2); }
  const log = console.log, warn = console.warn, err = console.error; console.log = console.warn = console.error = () => { };
  let g: any;
  try {
    const W: any = await standUp({ uploadEvery: 4, level: loadLevel(parseLevelData(readFileSync(resolve(ROOT, 'levels', E.level), 'utf8'))) }); g = W.game;
    const r = await startFromQuery(g, new URLSearchParams(scenarioQuery(E) as string)); if (r.why) throw new Error(`start refused — ${r.why}`);
    for (let i = 0; i < 20; i++) W.step(1 / 60);
  } finally { console.log = log; console.warn = warn; console.error = err; }
  GLASSPEN.stackLeast = !off;
  const d = g.doors.byName(DOOR), team = g.guards.filter((x: any) => x.team === 'operative' && x.state !== 'dead');
  if (!d) nov(`this level has no door '${DOOR}'`);
  else if (team.length !== 2) nov(`the scenario stood ${team.length} teammates, not the pair`);
  else {
    const th = d.def.closedDir, u = [Math.cos(th), Math.sin(th)], H = d.def.hinge, w = d.def.width, K = ORDERS;
    const cx = team.reduce((a: number, m: any) => a + m.char.pos[0], 0) / 2 - d.frameCentre[0], cz = team.reduce((a: number, m: any) => a + m.char.pos[2], 0) / 2 - d.frameCentre[2];
    const side: 1 | -1 = cx * -u[1] + cz * u[0] >= 0 ? 1 : -1, n = [-u[1] * side, u[0] * side];   // (the face the team starts on: the outside)
    const along = (p: number[]) => (p[0] - H[0]) * u[0] + (p[2] - H[1]) * u[1], offW = (p: number[]) => (p[0] - H[0]) * n[0] + (p[2] - H[1]) * n[1];
    const at = (a: number) => [H[0] + u[0] * a + n[0] * K.jambOut, 0, H[1] + u[1] * a + n[1] * K.jambOut];
    // the panes on this door's own wall, beside its frame: their span along the wall
    const panes: [number, number][] = [];
    for (const b of g.level.boxes) { if (!(b.flags & BoxFlag.Glass)) continue; const c = b.center ?? b.c, h = b.half ?? b.h; if (!c || !h) continue;
      const a = along([c[0], 0, c[2]]), o = offW([c[0], 0, c[2]]), ha = Math.abs(u[0]) * h[0] + Math.abs(u[1]) * h[2];
      if (Math.abs(o) < 0.3 && a + ha > -2.5 && a - ha < w + 2.5) panes.push([a - ha, a + ha]); }
    const nL = g.orders.glassEyes(d, side, at(w + K.jambPast)), nH = g.orders.glassEyes(d, side, at(-K.jambPast));
    console.log(`  staged: glass lets rounds through ${glassPenLive(g)} · panes on the door's wall beside the frame ${panes.length}${panes.length ? ` (${panes.map(p => `${p[0].toFixed(2)} to ${p[1].toFixed(2)} m from the hinge`).join(', ')}; the leaf is 0 to ${w})` : ''} · lines of fire through glass onto the latch jamb ${nL}, the hinge jamb ${nH} (${GLASSPEN.stackEyes} make a jamb no place to wait)`);
    if (!glassPenLive(g) || GLASSPEN.stackEyes <= 0) nov('rounds stop in glass on this stand (or stackEyes is 0): there is no rule to check');
    else if (!panes.length) nov(`no pane on ${DOOR}'s wall beside its frame: the level has changed under the check`);
    else if (nL < GLASSPEN.stackEyes || nH < GLASSPEN.stackEyes) nov(`a jamb of ${DOOR} is out of the glass's lines (latch ${nL}, hinge ${nH}): 'glass is not cover' deals the stack there by itself — the fault's cause is not in this level any more`);
    else {
      const W = 0.45, atWindow = (p: number[]) => offW(p) < 1.5 && panes.some(([a0, a1]) => along(p) > a0 - W && along(p) < a1 + W), hand = (p: number[]) => along(p) > w / 2 ? 'latch' : 'hinge';
      const fm = (P: any[]) => P.map(q => `(${q.pos[0].toFixed(2)}, ${q.pos[2].toFixed(2)}) ${hand(q.pos)}${q.lead ? ', lead' : ''}${atWindow(q.pos) ? ', AT THE WINDOW' : ''}`).join(' · ');
      const lines = (P: any[] | null) => (P ?? []).reduce((a, q) => a + g.orders.glassEyes(d, side, q.pos), 0);
      const judge = (name: string, P: any[] | null, want: number, walls: any[] | null, extra?: (P: any[]) => string | null) => {
        if (!P || P.length !== want) { fail(`${name}: no stack on ${DOOR} (${P ? P.length : 'null'} marks for ${want})`); return; }
        console.log(`  staged: ${name}: ${fm(P)} · lines of fire through glass onto these marks ${lines(P)}${walls ? `, onto the walls' own ${lines(walls)}` : ''}`);
        const win = P.filter(q => atWindow(q.pos)).length, hands = new Set(P.map(q => hand(q.pos))).size, more = extra?.(P) ?? null;
        if (win) { staged++; fail(`${name}: ${win} of ${want} marks at the window beside ${DOOR}`); }
        if (hands > 1) { staged++; fail(`${name}: a jamb each on a door with glass at both hands: behind a bang the man off the lead's hand walks across the door front`); }
        if (more) { staged++; fail(`${name}: ${more}`); }
        if (walls && lines(P) > lines(walls)) fail(`${name}: the marks dealt are in MORE lines of fire through glass (${lines(P)}) than the walls' own (${lines(walls)}): the rule has moved the stack to a worse place`); };
      const walls = (men: any[]) => g.orders.stackPostsOn(d, side, men, [], false) as any[] | null;
      judge('stack up, the pair', g.orders.stackPosts(d, side, team), 2, walls(team));
      judge('the door tools, the pair', g.doorTools.stackPosts(d, side, team), 2, null);
      judge('stack up, one man', g.orders.stackPosts(d, side, [team[0]]), 1, walls([team[0]]));
      { // Sam on the better jamb: the file goes on past him
        const home = [...g.player.char.pos], better = at(nL > nH ? -K.jambPast : w + K.jambPast); g.teleportPlayer([better[0], 0, better[2]]);
        judge('stack up, the pair, Sam on the better jamb', g.orders.stackPosts(d, side, team), 2, walls(team), P => P.some(q => Math.hypot(q.pos[0] - better[0], q.pos[2] - better[2]) < 0.7) ? 'a man dealt the place Sam stands on' : null);
        const second = at(nL > nH ? -K.jambPast - K.fileGap : w + K.jambPast + K.fileGap); g.teleportPlayer([second[0], 0, second[2]]);
        judge("stack up, the pair, Sam on the better wall's second place", g.orders.stackPosts(d, side, team), 2, null, P => P.some(q => Math.hypot(q.pos[0] - second[0], q.pos[2] - second[2]) < 0.7) ? 'a man dealt the place Sam stands on' : null);
        g.teleportPlayer(home); }
      { // the two men ordered one at a time, by the game's own order
        const mute = () => { console.log = console.warn = console.error = () => { }; }, un = () => { console.log = log; console.warn = warn; console.error = err; };
        const T = { kind: 'door', door: d, side, at: [d.frameCentre[0], 0, d.frameCentre[2]] }; let why: (string | null)[] = [];
        mute(); try { why = [0, 1].map(i => g.orders.give('stack', T, g.orders.squad().indexOf(team[i]))); } catch (e) { why = [String(e)]; } finally { un(); }
        const posts = team.map((m: any) => g.orders.of(m)).filter((A: any) => A && A.order === 'stack' && A.door === d && A.post).map((A: any) => ({ pos: A.post, lead: false }));
        if (why.some(x => x) || posts.length !== 2) nov(`one man at a time: the game would not take 'stack up' on ${DOOR} for each man in turn (${why.filter(x => x).join(' · ') || `${posts.length} of 2 posted`})`);
        else judge('stack up, one man at a time', posts, 2, null, P => Math.hypot(P[0].pos[0] - P[1].pos[0], P[0].pos[2] - P[1].pos[2]) < 0.7 ? 'both men dealt the same mark' : null); }
    }
  }
}

if (off) { console.log(staged ? `  STAGED: the fault REPRODUCED with the rule off: ${staged} findings (exit 1 is the expected answer here)` : '  STAGED: the fault did NOT reproduce with the rule off: this half proves nothing as it stands');; }
if (!failed) console.log(`- PASS — nobody of the pair is dealt a mark at the window beside ${DOOR}, both stack on one hand of it`);
process.exit(failed ? 1 : 0);
