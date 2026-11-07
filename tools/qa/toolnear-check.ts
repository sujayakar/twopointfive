// A DOOR ORDER'S LEAD NEVER STANDS IN THE OPENING TO WORK THE HANDLE (src/game/handleside.ts HANDLE.toolNear, nearStand), headless:
//   bun run tools/qa/toolnear-check.ts        (probes.ts handleside-tools, in family handleside; HANDLEPIE_BREAK=1 = the threshold's pie judged as the handle's
//   work: the 'clear from outside' lines must FAIL)
// THE RULE IS SETTLED (2026-11-07): nothing here writes it, the handle rule or the one-way doors, and the lines that asked the other side are out (the rule
// off, the two stands' row and word, a recording from before the key). TOOLNEAR_BREAK, which left the rule off, went with them.
// The fault it holds off, read off the maintainer's play tapes (the floodlit yard 044521 at 0:53.8 and 4:42.0, lesson 5 051816 twice, the barracks 053220 at
// 1:35.1): on a door order at the face a leaf opens AWAY from, the man who works the handle walked a metre from his stack mark to the handle's place — which
// stands in the middle of the opening — shoved the leaf, and was shot there or on his first step away by a defender with his gun on that door.
// Per door (the floodlit yard's front door, the barracks' galley back door) and per row of the door's tools as shipped (open and clear, bang and clear, clear
// from outside): the scenario stood up, every defender taken off the floor (the question is where the lead stands, not who wins), the team put outside the door
// on its latch side and given the row. From the order until the men go in (the hold: until it has held three seconds on the word):
//   · the lead's body is never in the opening's corridor: judged by his body's EDGE (his middle less HANDLE.body, his radius) past the latch jamb, within 2 m of the door's face
//     (11-06: but while 'clear from outside' PIES the threshold, DOORTOOLS.thrPie: there his edge may be in the corridor and his middle may not)
//   · the leaf comes open
//   · no walk to the handle's old place is dealt (HANDLE_STATS.toolWalks 0), and a step past the jamb or a shove from the mark is (toolNears + toolPosts)
//   · the entry happens: the men cross and the order ends 'in' (the hold: the same door's entry row is then given, and they cross)
// and beside them: the drill's own man and the response's (toolSpot asked without the order's word) get the handle's place as before.
// No GPU: nothing here looks at a picture.
import { ROOT } from './headless.ts';
import { stage } from './bang-tally.ts';
const { HANDLE, HANDLE_STATS, handleSpot, handleAt, toolSpot } = await import(`${ROOT}/src/game/handleside.ts`);
const { frameOf } = await import(`${ROOT}/src/game/guards.ts`);
const { DOORTOOLS } = await import(`${ROOT}/src/game/doortools.ts`);
const CBM = await import(`${ROOT}/src/game/combat.ts`);

const DOORS: { scn: string; level: string; door: string }[] = [{ scn: 'floodlit-four', level: 'the floodlit yard', door: 'front' }, { scn: 'barracks-four', level: 'the barracks', door: 'k_out' }];
const ROWS: { id: string; word: string; hold?: boolean }[] = [{ id: 'openClear', word: 'open and clear' }, { id: 'bangClear', word: 'bang and clear' }, { id: 'thresholdHold', word: 'clear from outside', hold: true }];
const flat = (a: number[], b: number[]) => Math.hypot(a[0] - b[0], a[2] - b[2]);
const f2 = (x: number) => (Number.isFinite(x) ? x.toFixed(2) : '—');
const quiet = async <T>(fn: () => T | Promise<T>): Promise<T> => { const log = console.log, err = console.error, warn = console.warn; console.log = console.error = console.warn = () => {}; try { return await fn(); } finally { console.log = log; console.error = err; console.warn = warn; } };
type Verdict = (ok: boolean, label: string, detail: string) => void;
function family(title: string): { verdict: Verdict; done: () => boolean } {
  console.log(`\n=== ${title} ===`); let all = true;
  return { verdict: (ok, label, detail) => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); }, done: () => { console.log(`=== ${title.split(':')[0]}: ${all ? 'ALL PASS' : 'FAILURES above'} ===`); return all; } };
}

interface Run { why: string | null; note: string; lead: string; mark: string; step: number; old: number; past: number; where: string; /** his nearest while he pies the threshold (DOORTOOLS.thrPie): the pie's own stance, printed and not judged */ pie: number; opened: boolean; crossed: number; ended: string; walks: number; nears: number; posts: number; drill: string }
/** one order on one door: the team outside on the latch side, the lead watched from the order until the men go in */
async function play(scn: string, door: string, row: { id: string; hold?: boolean }): Promise<Run> {
  const S: any = await quiet(() => stage(scn, 1)); const g = S.g, O = g.orders, D = g.doorTools, nav = g.col.nav;
  const R: Run = { why: null, note: '', lead: '', mark: '', step: 0, old: 0, past: Infinity, where: '', pie: Infinity, opened: false, crossed: 0, ended: '', walks: 0, nears: 0, posts: 0, drill: '' };
  for (const x of g.guards) if (x.state !== 'dead' && x.team === 'hostile') await quiet(() => CBM.killGuard(g, x, [0, 0, 1], true));
  const d = g.doors.list.find((x: any) => x.def.name === door); if (!d) { R.note = `no door '${door}'`; return R; }
  const F = frameOf(d), W: number = F.W, pt = (along: number, off: number, side: number): number[] => [F.hx + F.ux * along + F.nx * side * off, 0, F.hz + F.uz * along + F.nz * side * off];
  const side = ([1, -1] as const).find(s => !g.doors.pullFace(d, pt(W / 2, 0.5, s))); if (!side) { R.note = 'no face it opens away from'; return R; }
  const rel = (p: number[]) => { const rx = p[0] - F.hx, rz = p[2] - F.hz; return { a: rx * F.ux + rz * F.uz, o: (rx * F.nx + rz * F.nz) * side }; };
  // the team a few steps out, on the latch side first (nobody's walk to the stack crosses the front of the door)
  const gate = pt(W / 2, 0.7, side), team: any[] = O.squad().filter((x: any) => x.state !== 'dead'), spots: number[][] = [];
  for (const lat of [1.1, 1.9, 2.7, 3.5, 0.4, -0.4, -1.1, -1.9]) for (const back of [2.4, 3.4, 4.4, 5.4]) { const p = pt(W / 2 + lat, back, side);
    if (!nav.isBlocked(p[0], p[2]) && !g.col.solidBlocked([p[0], 1.1, p[2]], [gate[0], 1.1, gate[2]]) && !spots.some(s => flat(s, p) < 0.8)) spots.push(p); }
  if (spots.length < team.length + 1) { R.note = `no floor outside for the team (${spots.length} spots)`; return R; }
  team.forEach((gd, i) => { const p = spots[i]; gd.char.pos[0] = p[0]; gd.char.pos[2] = p[2]; gd.char.vel = [0, 0, 0]; gd.path = []; gd.pathGoal = null; gd.char.bodyYaw = gd.char.aimYaw = Math.atan2(-F.nx * side, -F.nz * side); });
  await quiet(() => g.teleportPlayer(spots[team.length] as any)); await quiet(() => S.step(30));
  for (const k of Object.keys(HANDLE_STATS)) HANDLE_STATS[k] = 0;
  R.why = await quiet(() => O.give(row.id, O.targetAt(d.frameCentre), 'all')); const op = D.ops[0];
  if (R.why !== null || !op) { R.note = `refused: ${R.why ?? 'no order under way'}`; R.why ??= 'no order'; return R; }
  const watch = () => { const lead = op.lead; if (!lead || lead.state === 'dead') return;
    if (!R.mark) { const P = op.post.get(lead); if (P) { const m = rel(P.pos), H = handleSpot(g, d, side); R.lead = lead.callsign; R.old = H ? flat(P.pos, H.pos) : NaN;
      R.mark = `${f2(m.a - W)} m past the latch jamb, ${f2(flat(P.pos, handleAt(d)))} m from the handle`;
      const dr = toolSpot(g, d, side, P.pos, true); R.drill = !H ? 'no place' : dr && flat(dr.pos, H.pos) < 0.01 ? 'same' : dr ? `(${f2(dr.pos[0])}, ${f2(dr.pos[2])}) for (${f2(H.pos[0])}, ${f2(H.pos[2])})` : 'none dealt'; } }
    if (op.kick?.hand && !R.step) R.step = flat(lead.char.pos, op.kick.pos);
    // (THE THRESHOLD'S PIE IS NOT THE HANDLE'S WORK. 'Clear from outside' opens the leaf and then holds the threshold; where the men PIE it (DOORTOOLS.thrPie,
    // the page's stand) the lead slices across the opening on purpose: the arc's deepest cut is 0.08 m in, less the lean's 0.15, so his hips stop short of
    // his jamb's edge and his body's edge is 0.25 m into the corridor (0.40 without the lean). This line read that as the handle worked from the opening.
    // While the order is in its threshold stage with the pie on, his nearest is kept apart and held to the pie's own reach, PIE_REACH below: his body's
    // edge may be in the corridor, his middle may not (the tape's fault was -0.59 m). With the pie off he holds the jamb and is judged through the hold,
    // as he was. HANDLEPIE_BREAK=1 = the pie given no reach of its own: judged as the handle's.)
    const pied = op.phase === 'thr' && DOORTOOLS.thrPie && process.env.HANDLEPIE_BREAK !== '1';
    const r = rel(lead.char.pos); if (r.o > 0 && r.o < 2 && r.a > -HANDLE.nearBody) { const past = r.a - W - HANDLE.body; if (pied) R.pie = Math.min(R.pie, past); else if (past < R.past) { R.past = past; R.where = `${op.phase} · ${O.of(lead)?.there ?? '-'}`; } }
    if (!d.latched && Math.abs(d.angle) >= 0.6) R.opened = true; };
  await quiet(() => { let held = 0;
    for (let i = 0; i < 60 * 60 && D.ops[0] === op && op.phase !== 'enter' && op.crossed.size === 0; i++) { S.step(); watch();
      if (row.hold && op.beats.some((b: any) => b.what.startsWith('holding · on your word')) && ++held > 180) break; } });
  R.walks = HANDLE_STATS.toolWalks; R.nears = HANDLE_STATS.toolNears; R.posts = HANDLE_STATS.toolPosts;
  // the entry: the order's own (the hold: the door's entry row, given to the men holding it)
  let last = op;
  if (row.hold) { const w = await quiet(() => O.give('openClear', O.targetAt(d.frameCentre), 'all')); if (w !== null) R.note = `the entry row refused: ${w}`; last = D.ops[0] ?? op; }
  await quiet(() => { let idle = 0; for (let i = 0; i < 60 * 90; i++) { S.step(); if (!d.latched && Math.abs(d.angle) >= 0.6) R.opened = true; if (!D.ops.length && ++idle > 30) break; } });
  R.crossed = last.crossed.size; R.ended = (last as any).ended ?? 'not ended';
  return R;
}

export async function toolnearCheck(): Promise<boolean> {
  const F = family(`handleside-tools: a door order's lead works the handle from past the latch jamb, never from the opening`);
  const keep = { post: HANDLE.toolPost, rnd: Math.random, dt: JSON.parse(JSON.stringify(DOORTOOLS)) };
  try {
    HANDLE.toolPost = 0;
    let drill = '';
    for (const X of DOORS) for (const row of ROWS) {
      const R = await play(X.scn, X.door, row); drill ||= R.drill;
      const PIE_REACH = HANDLE.body + 0.05;   // (how far into the corridor a man's EDGE may come while he pies the threshold: his middle stays out of it)
      const ok = R.why === null && !R.note && R.walks === 0 && R.nears + R.posts >= 1 && R.past >= 0 && R.pie >= -PIE_REACH && R.opened && R.crossed >= 1 && R.ended === 'in';
      F.verdict(ok, `${X.level}, ${X.door} door · ${row.word}: the lead works the handle from past the latch jamb, the leaf opens, the men go in`,
        R.note && R.why !== null ? R.note : `${R.lead}'s mark ${R.mark} · he steps ${R.nears ? `${f2(R.step)} m (the walk to the handle's place was ${f2(R.old)} m)` : R.posts ? 'nowhere: shoved from his mark' : R.walks ? `${f2(R.step)} m to the handle's place, in the opening` : 'nowhere'} · his body's nearest to the opening: ${R.past === Infinity ? 'never by the door' : `${f2(R.past)} m ${R.past >= 0 ? 'clear of' : 'INSIDE'} its corridor (${R.where})`}${R.pie === Infinity ? '' : ` · pieing the threshold after it: ${f2(R.pie)} m (the pie's reach is ${f2(-PIE_REACH)})${R.pie >= -PIE_REACH ? '' : ' HIS MIDDLE IN THE OPENING'}`} · leaf ${R.opened ? 'open' : 'SHUT'} · over the frame line ${R.crossed} · ended '${R.ended}' · walks to the old place ${R.walks}, steps past the jamb ${R.nears}, from the mark ${R.posts}${R.note ? ` · ${R.note}` : ''}`);
    }
    F.verdict(drill === 'same', 'asked without the order\'s word (a man who opens a door with no door order), the rule on, the place dealt from that mark is the handle\'s own as before', drill || 'no mark was read');
    return F.done();
  } finally { HANDLE.toolPost = keep.post; Math.random = keep.rnd; Object.assign(DOORTOOLS, keep.dt); }
}

if (import.meta.main) process.exit((await toolnearCheck()) ? 0 : 1);
