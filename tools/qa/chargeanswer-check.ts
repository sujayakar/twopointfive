// A DOOR CHARGE ORDER IS CARRIED OUT OR ANSWERED — src/game/doortools.ts DOORTOOLS.chargeAnswer (`knownOpen`, `short`, `chargeGone`). Off the mission
// cards' dry run of the two hostage rescues (11-04): the order was given from the mission's start, a defender came through that door and locked it
// behind him while the pair were still twenty metres off, the order ended there and then on "door's open.", and the pair walked up to the shut door
// and stood at it — one of them reading 'setting the charge' — until the captor's clock ran out. Nothing was charged, nobody said so.
//   [TPS_PROFILE=shipped] bun run tools/qa/chargeanswer-check.ts [--seed N] [--verbose]        exit 0 = holds, 1 = it does not
// A pair (the office's room clear) and four (the compound's), every defender off the floor but one far from the door, the fog of war on, the order
// given from where the mission puts the men, on a door the game itself picks: the farthest one a charge order is taken on whose leaf, put open, the
// picture does not show open (fogofwar.ts doorHeld — nobody of the team has a line to it). Every frame of every staging is watched for the two things
// a dropped order looks like:
//   SILENT   a charge order leaves the book with its charge not blown, a man of it alive and still on it, and no teammate speaking on that frame
//   STALE    a living teammate carries a charge order's assignment ('setting the charge', 'stood off') with no charge order in the book behind it
// and the stagings are the ways a charge order can come to nothing:
//   (1) the leaf swings open and shut again with the picture never showing it open: the order is still theirs, the charge goes on, it is armed
//   (2) the leaf opens unseen and stays open: the order ends within two frames of the picture showing it open, the setter says so, the men hold
//   (3) the leaf comes open under a set charge: the charge is off, the setter says so, the men hold
//   (4) the bag is emptied while the setter is at the lock: he says so, the men hold
//   (5) refused as it is given (the door open with the fog off, so the picture has it open · nothing in the bag): a man says why, nobody moves
//   (6) a door nothing happens to: charged and armed, and beat for beat as with the rule off
//   (7) plumbing: a recording from before the key replays with the rule off · the stand stands it up as its row says
// CHARGEANSWER_BREAK=1 leaves the rule off for (1) to (4): they must fail.
import { ROOT } from './headless.ts';
import { stage } from './bang-tally.ts';
const { DOORTOOLS, isOpen } = await import(`${ROOT}/src/game/doortools.ts`);
const { sideOf } = await import(`${ROOT}/src/game/orders.ts`);
const { FOG } = await import(`${ROOT}/src/game/fogofwar.ts`);
const CBM = await import(`${ROOT}/src/game/combat.ts`);

const args = process.argv.slice(2), flag = (n: string) => { const i = args.indexOf(n); return i >= 0 ? args[i + 1] : undefined; };
const VERBOSE = args.includes('--verbose'), SEED = Number(flag('--seed') ?? 1), BREAK = process.env.CHARGEANSWER_BREAK === '1';
const flat = (a: number[], b: number[]) => Math.hypot(a[0] - b[0], a[2] - b[2]);
const quiet = async <T>(fn: () => T | Promise<T>): Promise<T> => { if (VERBOSE) return await fn(); const log = console.log, err = console.error, warn = console.warn; console.log = console.error = console.warn = () => {}; try { return await fn(); } finally { console.log = log; console.error = err; console.warn = warn; } };
const SCNS = ['room-clear-pair', 'compound-four'];
let stood: boolean | null = null;   // (the rule as the stand first stood it up, before the check sets it)

/** a scenario stood up for this check: the fog on, the rule as asked, the floor quiet but for one defender far from `door` (when given) */
async function floor(scn: string, rule: boolean, door?: string) {
  const S: any = await quiet(() => stage(scn, SEED)); stood ??= DOORTOOLS.chargeAnswer; DOORTOOLS.chargeAnswer = rule;
  const g = S.g; g.fogOfWar.on = true; FOG.doors = true;
  const team: any[] = g.orders.squad().filter((x: any) => x.state !== 'dead'), d = door ? g.doors.list.find((x: any) => x.def.name === door) : null;
  const hostiles: any[] = g.guards.filter((x: any) => x.state !== 'dead' && x.team === 'hostile'), from = d ? d.frameCentre : team[0].char.pos;
  const by = [...hostiles].sort((a, b) => flat(b.char.pos, from) - flat(a.char.pos, from))[0];   // (one left standing, the farthest: the mission is not over)
  for (const x of hostiles) if (x !== by) await quiet(() => CBM.killGuard(g, x, [0, 0, 1], true));
  const put = (x: any, open: boolean) => { if (open) { x.latched = false; x.angle = (x.swing || 1) * 1.4; } else { x.angle = 0; x.latched = true; } x.vel = 0; x.closing = false; };
  const target = (x: any) => ({ kind: 'door', at: [x.frameCentre[0], 0, x.frameCentre[2]], door: x, side: sideOf(x, team[0].char.pos) });
  return { S, g, team, d, put, target };
}
/** the door this scenario is staged on: the farthest from the men that takes a charge order and that the picture does not show open when it is put open */
async function pick(scn: string): Promise<string | null> {
  const { S, g, team, put, target } = await floor(scn, true); await quiet(() => S.step(30));
  const doors: any[] = g.doors.list.filter((x: any) => !x.def.steel && !x.off && !isOpen(x)).sort((a: any, b: any) => flat(b.frameCentre, team[0].char.pos) - flat(a.frameCentre, team[0].char.pos));
  for (const x of doors) { const was = { a: x.angle, l: x.latched };
    put(x, true); await quiet(() => S.step(3)); const held = g.fogOfWar.doorHeld(x); x.angle = was.a; x.latched = was.l; await quiet(() => S.step(3));
    if (held && !(await quiet(() => g.orders.give('charge', target(x), 'all')))) return x.def.name; }
  return null;
}

type How = 'swing' | 'open' | 'off' | 'bag' | 'none';
interface Run { note: string; given: string | null; silent: string[]; stale: string[]; said: string[]; beats: string[]; ended: string; armed: boolean; set: number; shown: boolean; late: number; moved: number; left: string[] }

/** one staging: the charge ordered from the mission's start, `how` done to it, watched to its end (or `secs`) */
async function run(scn: string, door: string, rule: boolean, how: How, pre: 'open' | 'empty' | null = null, secs = 90): Promise<Run> {
  const R: Run = { note: '', given: null, silent: [], stale: [], said: [], beats: [], ended: '', armed: false, set: 0, shown: false, late: NaN, moved: 0, left: [] };
  const { S, g, team, d, put, target } = await floor(scn, rule, door), O = g.orders, D = g.doorTools, F = g.fogOfWar; if (!d) { R.note = `no door ${door}`; return R; }
  const spoke: { t: number; who: any; text: string }[] = []; { const say = g.say.bind(g); g.say = (gd: any, text: string, ...r: any[]) => { spoke.push({ t: g.time, who: gd, text }); return say(gd, text, ...r); }; }
  if (pre === 'open') { put(d, true); F.on = false; }   // (nothing hidden: the picture is the floor as it stands, and has the leaf open)
  if (pre === 'empty') { D.sync(); D.charges = 0; }
  await quiet(() => S.step(30));
  const at0 = team.map(m => [m.char.pos[0], 0, m.char.pos[2]]), n0 = spoke.length;
  R.given = await quiet(() => O.give('charge', target(d), 'all')) as string | null;
  if (pre) { await quiet(() => S.step(120)); R.moved = Math.max(...team.map((m, i) => flat(m.char.pos, at0[i]))); R.said = spoke.slice(n0).filter(x => team.includes(x.who)).map(x => `${x.who.callsign}: ${x.text}`); R.ended = D.ops.some((o: any) => o.kind === 'charge') ? 'an order in the book' : 'no order'; return R; }
  if (R.given) { R.note = `refused: ${R.given}`; return R; }
  const op: any = D.ops[D.ops.length - 1]; if (!op || op.kind !== 'charge' || op.door !== d) { R.note = 'no charge order on the door'; return R; }
  let live: any[] = [op], openAt = NaN, shownAt = NaN, endAt = NaN, did = false; const staleT = new Map<any, number>(), t0 = g.time;
  const frame = () => {
    // SILENT: an order gone from the book this frame, short of its charge, a man of it alive and on it, and no teammate's voice on the frame
    for (const o of live) if (!D.ops.includes(o)) { if (o === op) endAt = g.time;
      if (o.ended !== 'in' && (o.men as any[]).some(m => m.state !== 'dead') && !spoke.some(x => x.t === g.time && team.includes(x.who))) R.silent.push(`${(g.time - t0).toFixed(2)} end: ${o.ended}`); }
    live = D.ops.filter((o: any) => o.kind === 'charge');
    // STALE: a charge order's assignment on a living man with no charge order behind it, for more than a second
    for (const m of team) { const A = O.of(m), bad = m.state !== 'dead' && !!A && A.order === 'charge' && !live.some((o: any) => o.mine.get(m) === A);
      if (!bad) { staleT.delete(m); continue; } if (!staleT.has(m)) staleT.set(m, g.time);
      if (g.time - staleT.get(m)! > 1 && !R.stale.some(x => x.startsWith(m.callsign))) R.stale.push(`${m.callsign} '${A.arrived ? A.there : A.going}' from ${(staleT.get(m)! - t0).toFixed(2)}`); }
    // (the picture: does it show the leaf open — fogofwar.ts, what the player is drawn)
    if (!isNaN(openAt) && isOpen(d) && !F.doorHeld(d)) { if (how === 'swing') R.shown = true; if (isNaN(shownAt)) shownAt = g.time; }
  };
  await quiet(() => { for (let f = 0; f < secs * 60; f++) {
    if ((how === 'swing' || how === 'open') && f === 30) { put(d, true); openAt = g.time; }
    if (how === 'open' && !isNaN(openAt) && isNaN(endAt) && !isOpen(d)) put(d, true);   // (held open: a stand's own closer is not what this line is about)
    if (how === 'swing' && !isNaN(openAt) && !did && g.time - openAt >= 2.5) { put(d, false); did = true; }
    if (how === 'off' && !did && op.phase === 'armed' && g.time - op.t0 > 0.5) { put(d, true); did = true; }
    if (how === 'bag' && !did && D.ops.includes(op) && !isNaN(op.setT)) { D.charges = 0; did = true; }
    S.step(1); frame();
    if (op.phase === 'armed' && D.ops.includes(op) && how !== 'off' && g.time - op.t0 > 1) break;
    if (!isNaN(endAt) && g.time - endAt > 3) break;
  } });
  R.said = spoke.slice(n0).filter(x => team.includes(x.who)).map(x => `${(x.t - t0).toFixed(1)} ${x.who.callsign}: ${x.text}`);
  R.beats = op.beats.map((b: any) => `${(b.t - t0).toFixed(1)} ${b.what}`); R.ended = D.ops.includes(op) ? `still ${op.phase}` : `ended: ${op.ended}`; R.armed = D.ops.includes(op) && op.phase === 'armed'; R.set = D.set.filter((c: any) => c.door === d).length;
  R.late = endAt - shownAt; R.left = team.filter(m => m.state !== 'dead').map(m => { const A = O.of(m); return A ? `${A.order}: ${A.arrived ? A.there : A.going}` : 'none'; });
  return R;
}

export async function chargeanswerCheck(): Promise<boolean> {
  let ok = true; const line = (pass: boolean, what: string, more = '') => { console.log(`- ${pass ? 'PASS' : 'FAIL'} — ${what}`); if (more) console.log(`    ${more}`); if (!pass) ok = false; };
  const keep = { rule: DOORTOOLS.chargeAnswer, doors: FOG.doors }, RULE = !BREAK, off = BREAK ? ' — CHARGEANSWER_BREAK: the rule left off' : '';
  const tell = (R: Run) => R.note ? R.note : `${R.ended} · charges on the door ${R.set} · silent ${R.silent.length ? R.silent.join(', ') : 'none'} · stale ${R.stale.length ? R.stale.join(', ') : 'none'} · the men after: ${R.left.join(' / ')} · said: ${R.said.length ? R.said.join(' / ') : 'nothing'}${VERBOSE ? ' · ' + R.beats.join(' | ') : ''}`;
  const clean = (R: Run) => !R.note && !R.silent.length && !R.stale.length, says = (R: Run, re: RegExp) => R.said.some(x => re.test(x)), holds = (R: Run) => R.left.length > 0 && R.left.every(x => x.startsWith('hold: '));
  try {
    for (const scn of SCNS) { const door = await pick(scn); if (!door) { line(false, `${scn}: no door a charge order is taken on that the picture does not show from the mission's start`); continue; } const w = `${scn} ${door}`;
      const a = await run(scn, door, RULE, 'swing');
      line(clean(a) && a.armed && a.set === 1 && !a.shown, `(1) the leaf swings open and shut again and the picture never shows it open (${w}${off}): the charge goes on and is armed`, `${tell(a)} · shown open ${a.shown}`);
      const b = await run(scn, door, RULE, 'open');
      line(clean(b) && b.ended === 'ended: the door opened' && b.set === 0 && says(b, /door's open\.$/) && b.late >= 0 && b.late <= 2 / 60 + 1e-6 && holds(b), `(2) the leaf opens unseen and stays open (${w}${off}): the order ends as the picture shows it open, the setter says so, the men hold on the door`, `${tell(b)} · ended ${isNaN(b.late) ? 'with the picture never showing it open' : `${b.late.toFixed(2)} s after the picture showed it`}`);
      const c = await run(scn, door, RULE, 'off');
      line(clean(c) && c.ended === 'ended: the charge is gone' && c.set === 0 && says(c, /charge is off/) && holds(c), `(3) the leaf comes open under a set charge (${w}${off}): the setter says the charge is off, the men hold on the door`, tell(c));
      const e = await run(scn, door, RULE, 'bag');
      line(clean(e) && e.ended === 'ended: no charges' && e.set === 0 && says(e, /no charges left/) && holds(e), `(4) the bag emptied while he is at the lock (${w}${off}): he says so, the men hold on the door`, tell(e));
      if (BREAK) continue;
      const f = await run(scn, door, true, 'none', 'open'), h = await run(scn, door, true, 'none', 'empty');
      line(f.given === "it's open." && f.said.some(x => x.endsWith("it's open.")) && f.moved < 0.5 && f.ended === 'no order', `(5) refused as it is given, the door open in the picture (${w}): a man says so and nobody moves`, f.note || `give → ${f.given} · said: ${f.said.join(' / ') || 'nothing'} · moved ${f.moved.toFixed(2)} m · ${f.ended}`);
      line(h.given === 'no charges left.' && h.said.some(x => x.endsWith('no charges left.')) && h.moved < 0.5 && h.ended === 'no order', `(5) refused as it is given, nothing in the bag (${w}): a man says so and nobody moves`, h.note || `give → ${h.given} · said: ${h.said.join(' / ') || 'nothing'} · moved ${h.moved.toFixed(2)} m · ${h.ended}`);
      const n = await run(scn, door, true, 'none'), n0 = await run(scn, door, false, 'none');
      line(clean(n) && n.armed && n.set === 1 && n.beats.join('|') === n0.beats.join('|'), `(6) a door nothing happens to (${w}): charged and armed, nothing silent, nothing stale, beat for beat as with the rule off`, `on ${tell(n)} · off ${n0.beats.join(' | ')}`);
    }
    if (!BREAK) {
      { const J: any = await import(`${ROOT}/src/game/journal.ts`), g = (await floor(SCNS[0], true)).g, snap = () => JSON.parse(JSON.stringify(J.knobsSnapshot(g)));
        DOORTOOLS.chargeAnswer = true; const old = snap(); if (old.tables.DOORTOOLS) delete old.tables.DOORTOOLS.chargeAnswer; J.applyKnobs(g, old); const before = DOORTOOLS.chargeAnswer;
        DOORTOOLS.chargeAnswer = true; const on = snap(); DOORTOOLS.chargeAnswer = false; J.applyKnobs(g, on); const kept = DOORTOOLS.chargeAnswer;
        DOORTOOLS.chargeAnswer = false; const no = snap(); DOORTOOLS.chargeAnswer = true; J.applyKnobs(g, no); const keptOff = DOORTOOLS.chargeAnswer;
        line(before === false && kept === true && keptOff === false, '(7) a recording from before the key replays with the rule off; one made with it on replays on, off off', `from before the key: ${before} · recorded on: ${kept} · recorded off: ${keptOff}`); }
      { const { FROZEN_HOLDS } = await import(`${ROOT}/src/host/profile.ts`), shipped = process.env.TPS_PROFILE === 'shipped', word = process.env.TPS_CHARGEANSWER, want = word !== undefined ? word === '1' : shipped, row = FROZEN_HOLDS['DOORTOOLS.chargeAnswer'];
        line(stood === want && !!row?.includes('TPS_CHARGEANSWER=1'), `(7) this stand (${shipped ? "the page's" : 'frozen'}) stands the rule up ${want ? 'on' : 'off'}, and FROZEN_HOLDS carries its row`, `stood up ${stood} · the row: '${row}'`); }
    }
  } finally { DOORTOOLS.chargeAnswer = keep.rule; FOG.doors = keep.doors; }
  return ok;
}

if (import.meta.main) { const ok = await chargeanswerCheck(); console.log(`=== chargeanswer: ${ok ? 'ALL PASS' : 'FAILED'} ===`); process.exit(ok ? 0 : 1); }
