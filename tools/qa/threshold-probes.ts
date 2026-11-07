// The 'threshold' family (game/doortools.ts — kind 'threshold': a room worked from the threshold before anybody goes in), run through probes.ts:
//   bun run tools/qa/probes.ts threshold | threshold-pair | threshold-four | threshold-hold | threshold-first | threshold-doors | threshold-leaf | threshold-last | threshold-hand | threshold-play
// PASS / FAIL each, all headless. Every probe here gives the stage the way a console would (its rows on the door menu are thresholdmenu-probes.ts's) —
// `game.doorTools.thresholdOn(<door name>, 'all')` — on a start-card scenario staged exactly as the page stages it (scenarioList.ts → scenarioRun.ts).
// The harness has no light meter (what a man sees of another is guards.ts's direct-light estimate), no smoke, no picture: nothing here says how it LOOKS.
//   threshold-pair      the pair (the records office, and the office's room clear), a held room's door: the order is taken; when the leaf
//                       moves a man stands on each jamb, outside; the stage's phases come in order — stacked → threshold → held → (toss → bang →) step in →
//                       first man over the line → hand-over → searched → in; NO man of the order is over the frame line, nor on the room's floor, on any frame
//                       before 'step in'; rounds are fired at the room's men from outside it; the blind pockets get a can from a man who is not the first man
//                       in, bursting inside the room, before anybody crosses; every point stepped in to is 2.5 m or less inside; and the order ends as any
//                       entry does — the floor looked over by the entry's own search, the men left holding the room
//   threshold-four      the same with the squad of four (the office's four-man room clear) — and the two men who are on neither jamb are outside
//                       with their eyes spare for the squad's back (orders.ts Assignment.spare) for as long as the room is worked
//   threshold-hold      the OTHER order of it — work the room from the threshold and HOLD the jambs until told (thresholdOn(door, who, side, true)) — with the
//                       pair (the records office): the door gone quiet, the lead says so and NOBODY goes in, no can is spent, for ten seconds past the moment the
//                       plain order would have gone; then bang and step in (row 2 of the door's tools: a can bursts inside before anybody crosses)
//   threshold-first     DOORTOOLS.thrFirst — the floor gone loud in the team's ears (the stager's fight.hotT): the leaf opened from the jamb, the first of the
//                           stage's cans in, its burst let run dry, and only then the jamb men out onto their diagonals; a quiet floor gets the stage as it was
//                           (one can a door, on the word too)
//   threshold-doors     every door of the levels the seven scenarios play on, both faces: wherever the stack's own test passes (floor at a jamb), the stage
//                       has somewhere shallow to step in to (2.5 m or less inside); the count of faces with a man on EACH jamb and with one jamb only is printed
//   threshold-leaf      DOORTOOLS.thrDoorsLeaf: a man left on another door of the room is not put where that door's own leaf, stood open, hides it from
//                       him — the depot (THRDOORSLEAF_BREAK=1 = each such man stood behind that leaf by hand: red)
//   threshold-hand      DOORTOOLS.thrHand — a threshold held, every man put off the door from a second before Sam says 'bang, then step in' until a second
//                       after (what a grenade by the stack does to them): with the rule the can waits for a man back at the door — not the lead — and no
//                       longer than thrHandWait in all. THRHAND_BREAK=1 = nobody is run off the door: must fail
//   threshold-last      DOORTOOLS.thrLast (the captor's last warning heard at the door): the registry's hostage door staged by hand — the stack goes
//                       without a man still walking up (the far-jamb man: given the lead's wall, never across the door; the lead: the door handed to
//                       the man on his jamb; Sam in the place: the next one), nothing held, one can
import { readFileSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { resolve } from 'node:path';
import { standUp, ROOT } from './headless.ts';
const { DOORTOOLS, doorPt } = await import(`${ROOT}/src/game/doortools.ts`);
const { GLASSPEN } = await import(`${ROOT}/src/game/glasspen.ts`);
const { killGuard } = await import(`${ROOT}/src/game/combat.ts`);
const { SCENARIOS } = await import(`${ROOT}/src/scene/scenarioList.ts`);
const { scenarioQuery } = await import(`${ROOT}/src/scene/scenarios.ts`);
const { startFromQuery } = await import(`${ROOT}/src/game/scenarioRun.ts`);
const { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`);
const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
const { feelTables } = await import(`${ROOT}/src/game/teamfeel.ts`);

const DT = 1 / 60;
function seeded(a: number) { return () => { a |= 0; a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ (a >>> 15), 1 | a); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; }; }
const f1 = (x: number) => (Number.isFinite(x) ? x.toFixed(1) : String(x));
const flat = (a: number[], b: number[]) => Math.hypot(a[0] - b[0], a[2] - b[2]);
type Verdict = (ok: boolean, label: string, detail: string) => void;
function family(title: string): { verdict: Verdict; done: () => boolean } {
  console.log(`\n=== ${title} ===`); let all = true;
  return { verdict: (ok, label, detail) => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); }, done: () => { console.log(`=== ${title.split(':')[0]}: ${all ? 'ALL PASS' : 'FAILURES above'} ===`); return all; } };
}
/** every probe leaves the feel tables (teamfeel.ts: DOORTOOLS among them) as it found them, pass, fail or throw */
const guarded = (fn: () => Promise<boolean>) => async () => { const T = feelTables(), keep = JSON.parse(JSON.stringify(T)); try { return await fn(); } finally { YARDOUT = LEAFBREAK = false; for (const n of Object.keys(keep)) Object.assign(T[n], keep[n]); } };
async function quiet<T>(fn: () => T | Promise<T>): Promise<T> { const log = console.log, err = console.error, warn = console.warn; console.log = console.error = console.warn = () => { }; try { return await fn(); } finally { console.log = log; console.error = err; console.warn = warn; } }
const levelOfFile = (file: string): any => { const data = parseLevelData(readFileSync(resolve(ROOT, 'levels', file), 'utf8')); return loadLevel(data); };

// a door's own frame (doortools.ts keeps these to itself): metres along hinge → latch, and off the frame line on face `side`
const alongOf = (d: any, p: number[]) => { const th = d.def.closedDir; return (p[0] - d.def.hinge[0]) * Math.cos(th) + (p[2] - d.def.hinge[1]) * Math.sin(th); };
const offOf = (d: any, p: number[], side: number) => { const th = d.def.closedDir; return (Math.cos(th) * (p[2] - d.def.hinge[1]) - Math.sin(th) * (p[0] - d.def.hinge[0])) * side; };

/** THE YARD'S GUNS OUT OF IT, for the runs staged while this is set: on a door another room watches (the depot's `a_s` on the floodlit yard — heldEntries) every
 *  defender who is not at home in the room behind the door is taken off the floor as the order is given. threshold-four and threshold-first ask what the
 *  THRESHOLD does with the room; with those guns in it who is left on the jambs is the yard fight's dice, and which of three seeds holds a wipe moved with any
 *  change that shifted a walk by half a second (three flips of threshold-four in a day, none of them a door rule's). */
let YARDOUT = false;
/** threshold-leaf's red path (THRDOORSLEAF_BREAK): each man the entry left on another door is stood behind that door's open leaf by hand before he is read */
let LEAFBREAK = false;
/** a start-card scenario staged as the page stages it, under a seed (the playthrough's own recipe: the page's chaos, Sam in god mode and pinned dark) */
async function stage(scn: string, seed: number) {
  const E = SCENARIOS.find((e: any) => e.id === scn); if (!E) throw new Error(`no scenario '${scn}'`);
  Math.random = seeded(0x9137 + seed * 7919);
  const W: any = await quiet(() => standUp({ uploadEvery: 4, ...(E.level ? { level: levelOfFile(E.level) } : {}) })); const g = W.game;
  g.godMode = true; g.tune.chaos = 0.5;
  let q = scenarioQuery(E) as string; if (E.kind === 'roomclear') q = q.replace(/roomclear(=\d+)?/, `roomclear=${seed}`);
  const { why } = await quiet(() => startFromQuery(g, new URLSearchParams(q))); if (why) throw new Error(`${scn}: start refused — ${why}`);
  const R = { E, W, g, O: g.orders, D: g.doorTools, post: null as null | (() => void),
    step(n = 1) { for (let i = 0; i < n; i++) { g.player.visibility = 0.05; W.step(DT); R.post?.(); } },
    get team(): any[] { return R.O.squad(); }, get hostiles(): any[] { return g.guards.filter((x: any) => x.team === 'hostile'); } };
  await quiet(() => R.step(20));
  return R;
}
type Rig = Awaited<ReturnType<typeof stage>>;

/** the rooms that still hold a living defender at home (the room fight's own model), each with its entries — nearest the team first.
 *  `unwatched`: only the entries a HOLD can be asked of — the door whose only say is the room's behind it:
 *   · no living defender of ANOTHER room has a line, from where he is posted, to the floor the squad works it from (`watchedBy`). On the motor-pool depot the
 *     nearest door, the crew room's `a_s`, is on the floodlit yard under the comms hut's west slit and the store's hatch (tools/levels/depot.ts: 'the obvious
 *     one, and the watched one'): four men stacked on it are in a fire fight across the yard before the room behind the door has said anything, and who is
 *     left on the jambs is that fight's dice — on either stand;
 *   · the order takes it from the face the held room is NOT on (`outFace`): that depot's `a_e` is nearer the team through the crew room itself than round by
 *     the battery room, so an order on it is worked from the crew room's own floor, into the wash room.
 *  What is left there is the store's `c_w`, from the bay. The records office's `front` passes both and is picked as it was. */
function heldEntries(R: Rig, unwatched = false): { room: any; ent: any }[] {
  const g = R.g, rooms = new Set<any>(), mid = R.team[0]?.char.pos ?? g.player.char.pos;
  for (const d of g.fight.defs) if (d.room && d.room.entries.length && d.room.cells.size <= 500 && d.gd.state !== 'dead') rooms.add(d.room);
  const out: { room: any; ent: any }[] = []; for (const room of rooms) for (const ent of room.entries) out.push({ room, ent });
  return out.sort((a, b) => flat(a.ent.at, mid) - flat(b.ent.at, mid)).filter(x => !unwatched || (outFace(R, x.ent) && !watchedBy(R, x.room, x.ent).length));
}
/** the face an order of the whole team takes `ent`'s door from (doortools.ts thresholdOn's own choice, Sam put outside as doorRun puts him) is the entry's outside */
function outFace(R: Rig, ent: any): boolean {
  const d = ent.door, out = offOf(d, [ent.at[0] + ent.out[0], 0, ent.at[2] + ent.out[1]], 1) >= 0 ? 1 : -1, men = R.O.men('all');
  return (R.O.big && men.length ? R.O.sideFor(d, men, out) : out) === out;
}
/** the living defenders at home in a room OTHER than `room` with an open line, from where they stand, to the floor the squad works `ent` from: the opening's
 *  face and the jamb and rear marks either side of it (the ones on open floor in the face's own sight) */
function watchedBy(R: Rig, room: any, ent: any): string[] {
  const g = R.g, nav = g.col.nav, lx = -ent.out[1], lz = ent.out[0], c = [ent.at[0] + ent.out[0] * 0.7, 1.2, ent.at[2] + ent.out[1] * 0.7];
  const pts = [-2.2, -1.3, 0, 1.3, 2.2].map(l => [c[0] + lx * l, 1.2, c[2] + lz * l]).filter(p => !nav.isBlocked(p[0], p[2]) && !g.col.solidBlocked(c, p));
  return g.fight.defs.filter((d: any) => d.room !== room && d.gd.state !== 'dead' && pts.some(p => !g.col.segmentBlocked([d.gd.char.pos[0], 1.5, d.gd.char.pos[2]], p))).map((d: any) => d.gd.callsign ?? 'a man');
}

interface Run {
  scn: string; seed: number; door: string; n: number; why: string | null;
  beats: { t: number; what: string }[]; ended: string; phases: string[];
  /** when the leaf moved: each man's place in the door's frame (along hinge → latch, off the line on the squad's face), who leads */
  atOpen: { who: string; along: number; off: number; lead: boolean; spare: boolean; glass?: number }[]; W: number;
  /** GLASS IS NOT COVER (orders.ts glassEyes; 0 while rounds stop in glass): the room's lines THROUGH GLASS onto each man's place as the leaf moved (atOpen.glass) and
   *  onto the jamb across the opening from the lead's · a can that went in first: was the floor loud in a man of the order's ears as it was dealt (roomfight.ts hotFor) */
  glassJamb?: number; firstHot?: boolean;
  /** the probe's OWN watch, not the module's beats: per man the first frame he was over the frame line, or on the room's floor */
  firstIn: Map<string, number>; inBefore: number;
  /** rounds fired by the order's men, with where the shooter stood · their men who dropped, by whom · every blast */
  shots: { t: number; who: string; outside: boolean }[]; downs: { t: number; who: string; by: string; byOutside: boolean }[]; bangs: { t: number; inside: number }[];
  /** the order's own men lost to a defender who is NOT at home in the room behind the door (another room's, the yard's): who, by whom */
  yard: { who: string; by: string }[];
  bangs0: number; bangs1: number; pods: { who: string; deep: number }[]; holding: number; alive: number; left: number; cells: number; doorMen: { who: string; door: string; arrived: boolean; dist: number; line: boolean; inRoom: boolean; /** that door's own leaf, stood open on its stop, lies across his line to it */ leaf: boolean }[];
  /** man-frames the rear men spent with their eyes spare while the room was worked, of the man-frames they could have */
  spare: number; spareOf: number; secs: number;
  /** the order given to HOLD: when the lead said the door had gone quiet, when Sam said what next and what came back (null: taken), the kit's bangs and the
   *  order's phase on that frame · at the end: each man still up, how far off the frame line on the squad's face, and what he is doing */
  toldT: number; tellT: number; tellR: string | null; tellBangs: number; tellPhase: string; endAt: { who: string; off: number; doing: string }[];
  /** DOORTOOLS.thrFirst: the furthest any man of the order got off his mark on the wall — beyond where he stood as the leaf moved — before the stage sent the jamb men out onto their
   *  diagonals (the beat 'out onto the diagonals …' / 'no can first …'; NaN: the stage never held them back) · the kit's cans on that frame */
  heldBack: number; outBangs: number;
  /** each can as it left: who threw it and how far from the frame's centre he stood on that frame (DOORTOOLS.thrHand) */
  tossAt: { who: string; near: number; lead: string }[];
  /** how long the hold had run when it first ended (with doorRun's `frag`: the first time after the grenade), who of the order was not at the door or on his mark that frame, and who was not the frame 'step in' was said;
   *  doorRun's `frag`: when the grenade was put down (game time) and who ran from it */
  thrEndT: number; offAtEnd?: string[]; offAtStep?: string[]; fragT: number; fragRan: string[];
  /** … and how far into the hold every living man was first back at the door or on his mark with nobody still running (the probe's own look, rule or no rule) */
  backT: number;
}
/** given to hold: what Sam says `after` seconds past the lead's 'holding' — a row of the door's tools, or the word's key for one frame */
interface Hold { tell: 'step' | 'bang' | 'word'; after: number; /** the word given a second time, this long after the first */ again?: number }
const tOf = (r: Run, re: RegExp) => r.beats.find(b => re.test(b.what))?.t ?? NaN;
const idx = (r: Run, re: RegExp) => r.beats.findIndex(b => re.test(b.what));

/** ONE DOOR WORKED FROM THE THRESHOLD: the scenario staged, the commander stood a few metres off the door on the squad's side of it, the stage ordered by the
 *  door's name, and everything watched until the order ends. `door` null: the nearest entry of a held room that takes the order. `away` (with `hold`): from
 *  a second before Sam says what next until that many seconds after, every man of the order is kept RUN OFF THE DOOR — stood on floor beyond a throw of the
 *  frame, as a grenade landing by the stack leaves them (frag.ts fragFlee), and not to be dropped while there; then let go to walk back to his own mark. */
async function doorRun(scn: string, seed: number, door: string | null = null, cap = 90, word = NaN, hold: Hold | null = null, hot = false, unwatched = false, away = 0, frag = NaN): Promise<Run> {
  const R = await stage(scn, seed), g = R.g, D = R.D, men0: any[] = [...R.team];
  const FR: any = isNaN(frag) ? null : await import(`${ROOT}/src/game/frag.ts`); if (FR) FR.FRAG.on = true;   // (`frag`: a defender's grenade is put down — at the lead's feet that many s before Sam speaks to a held door, or at the rear-most man's that far into a hold that steps in by itself; the men get off it by their own rule — frag.ts fragFlee — and are kept from its burst)
  const out: Run = { scn, seed, door: door ?? '', n: men0.length, why: null, beats: [], ended: '', phases: [], atOpen: [], W: 0, firstIn: new Map(), inBefore: 0, shots: [], downs: [], bangs: [], yard: [], bangs0: D.bangs, bangs1: D.bangs, pods: [], doorMen: [], holding: 0, alive: 0, left: NaN, cells: NaN, spare: 0, spareOf: 0, secs: 0, toldT: NaN, tellT: NaN, tellR: null, tellBangs: NaN, tellPhase: '', endAt: [], heldBack: NaN, outBangs: NaN, tossAt: [], fragT: NaN, fragRan: [], thrEndT: NaN, backT: NaN };
  let op: any = null, room: any = null; const stoodAt = new Map<any, number[]>(); let canFirst = false;
  for (const { room: rm, ent } of heldEntries(R, unwatched)) { const d = ent.door; if (door && d.def.name !== door) continue;
    // the commander: off the opening's line, on the face the room is NOT on (a ghost — he is put there, he does not walk)
    const lx = -ent.out[1], lz = ent.out[0]; let sam: number[] | null = null;
    for (const [back, lat] of [[3.4, 1.6], [3.4, -1.6], [4.4, 1.2], [4.4, -1.2], [2.8, 2.2], [2.8, -2.2], [2.6, 0]]) { const p = [ent.at[0] + ent.out[0] * back + lx * lat, 0, ent.at[2] + ent.out[1] * back + lz * lat]; if (!g.col.nav.isBlocked(p[0], p[2]) && !g.col.solidBlocked([p[0], 1.1, p[2]], [ent.at[0] + ent.out[0] * 0.7, 1.1, ent.at[2] + ent.out[1] * 0.7])) { sam = p; break; } }
    if (!sam) continue; g.teleportPlayer(sam);
    const r = D.thresholdOn(d.def.name, 'all', undefined, !!hold); if (typeof r === 'string') { out.why = `${d.def.name}: ${r}`; continue; }
    op = r; room = rm; out.door = d.def.name; out.why = null;
    if (YARDOUT) for (const h of R.hostiles) if (h.state !== 'dead' && g.fight.defs.find((x: any) => x.gd === h)?.room !== rm) killGuard(g, h, [0, 0, 1], true, 'struck');   // (in the frame the order is given: nobody has moved)
    break; }
  if (!op) { out.why ??= 'no held room with a door the order could be given on'; return out; }
  const d = op.door, side = op.side, W = d.def.width, nav = g.col.nav; out.W = W;
  const sent: any[] = [...op.men], ids = new Map<number, any>(sent.map((m: any) => [m.char.id, m])), seen = new Set<any>(g.events), was = new Map<any, string>(R.hostiles.map((h: any) => [h, h.state]));
  const over = (p: number[]) => (offOf(d, p, side) < -0.15 && alongOf(d, p) > -0.6 && alongOf(d, p) < W + 0.6) || room.cells.has(Math.floor(p[2] / nav.cell) * nav.W + Math.floor(p[0] / nav.cell));
  let stepT = NaN, thrT = NaN, told2 = false; const podded = new Set<any>();
  // (`away`: a place for each man beyond a throw of the frame, on the squad's face, with a walk back to the lead's mark that is no tour of the floor)
  const godWas = new Map<any, boolean>(); let awayAt: number[][] | null | undefined; const awaySpots = (): number[][] | null => { const got: number[][] = [], fc = d.frameCentre, back = op.post.get(op.lead)?.pos ?? fc;
    for (let r = DOORTOOLS.throwReach + 1.5; r <= DOORTOOLS.throwReach + 5 && got.length < sent.length; r += 0.75) for (let a = 0; a < 24 && got.length < sent.length; a++) { const th = a / 24 * Math.PI * 2, p = [fc[0] + Math.sin(th) * r, 0, fc[2] + Math.cos(th) * r];
      if (offOf(d, p, side) < 0.8 || nav.isBlocked(p[0], p[2]) || got.some(o => flat(o, p) < 1) || flat(p, g.player.char.pos) < 1) continue;
      const path = nav.findPath(p, back); if (!path?.length || flat(path[path.length - 1], back) > 0.8) continue;
      let L = 0, q = p; for (const w of path) { L += flat(q, w); q = w; } if (L <= 2 * r + 2) got.push(p); }
    return got.length >= sent.length ? got : null; };
  R.post = () => { const t = g.time;
    if (hot && op.phase === 'stack') g.fight.hotT = t;   // (the floor has gone loud in the team's ears: the stager's own mark, kept fresh until the stack is made)
    if (word < 0 && op.thr?.first === 'can' && op.phase === 'fuse') { word = NaN; R.W.input.pressed.add(DOORTOOLS.blowKey); }   // (word < 0: the commander's word while the can that went in first is still in the air)
    if (op.phase === 'thr') { if (isNaN(thrT)) thrT = t; if (t - thrT >= word) { word = NaN; R.W.input.pressed.add(DOORTOOLS.blowKey); } }   // (the commander's word, that far into the hold)
    if (out.phases[out.phases.length - 1] !== op.phase && D.ops.includes(op)) { out.phases.push(op.phase);
      if (op.phase !== 'stack' && !out.atOpen.length) out.atOpen = sent.filter((m: any) => m.state !== 'dead').map((m: any) => ({ who: m.callsign, along: alongOf(d, m.char.pos), off: offOf(d, m.char.pos, side), lead: m === op.lead, spare: false, glass: g.orders.glassEyes(d, side, m.char.pos) }));
      if (op.phase !== 'stack' && out.glassJamb === undefined) { const L = out.atOpen.find(m => m.lead); out.glassJamb = L ? g.orders.glassEyes(d, side, doorPt(d, W - L.along, L.off, side)) : 0; }   // (the lead's own place, mirrored across the opening: the jamb a pair on one wall left bare)
 }
    if (op.thr && !stoodAt.size) for (const m of sent) if (m.state !== 'dead') stoodAt.set(m, [...m.char.pos]);   // (the frame the stage begins: the leaf moving — a booted door's lead is back on his jamb by his own order, he does not count as having shown)
    if (op.thr?.first === 'can' && !canFirst) out.firstHot = sent.some((m: any) => m.state !== 'dead' && g.fight.hotFor(m));
    if (op.thr?.first === 'can') canFirst = true;
    if (canFirst && stoodAt.size && isNaN(out.outBangs)) { if (op.beats.some((x: any) => /^(out onto the diagonals|no can first)/.test(x.what))) out.outBangs = D.bangs;
      else if (op.thr?.first === 'can' || op.thr?.first === 'burst') for (const [m, p] of stoodAt) { const mark = op.post.get(m)?.pos ?? p; if (m.state !== 'dead') out.heldBack = Math.max(isNaN(out.heldBack) ? 0 : out.heldBack, flat(m.char.pos, mark) - flat(p, mark)); } }   // (off his mark on the wall by more than he was as the leaf moved: the lead who booted it is walking BACK to his)
    if (isNaN(stepT)) { const b = op.beats.find((x: any) => /^step in/.test(x.what)); if (b) stepT = b.t; }
    if (hold && isNaN(out.toldT)) { const b = op.beats.find((x: any) => /^holding · on your word/.test(x.what)); if (b) out.toldT = b.t; }
    if (hold && isNaN(out.tellT) && t - out.toldT >= hold.after) { out.tellT = t; out.tellBangs = D.bangs; out.tellPhase = D.ops.includes(op) ? op.phase : '(over)';
      if (hold.tell === 'word') R.W.input.pressed.add(DOORTOOLS.blowKey); else out.tellR = R.O.give(hold.tell === 'bang' ? 'bangClear' : 'openClear', R.O.targetAt(d.frameCentre), 'all'); }   // (rows 2 and 1 of the door's tools, what the page gives: on men holding it from outside they are 'bang, then go in' and 'go in, no bang')
    if (hold?.again !== undefined && !isNaN(out.tellT) && !told2 && t - out.tellT >= hold.again) { told2 = true; R.W.input.pressed.add(DOORTOOLS.blowKey); }
    { const tosses = op.beats.filter((x: any) => /^toss:/.test(x.what)); for (let k = out.tossAt.length; k < tosses.length; k++) { const who = tosses[k].what.slice(5).split(' · ')[0], m = sent.find((x: any) => x.callsign === who); out.tossAt.push({ who, near: m ? flat(m.char.pos, d.frameCentre) : NaN, lead: op.lead.callsign }); } }   // (read BEFORE the men are put back off the door below: where the frame itself left the thrower)
    const back = (m: any) => { const p = m.char.pos, post = op.mine.get(m)?.post; return (flat(p, d.frameCentre) < DOORTOOLS.throwReach && offOf(d, p, side) > 0) || (!!post && flat(p, post) <= 0.6); };
    { if (isNaN(out.thrEndT) && !isNaN(thrT) && (isNaN(frag) || !isNaN(out.fragT)) && (op.phase !== 'thr' || !D.ops.includes(op))) { out.thrEndT = t - thrT; out.offAtEnd = sent.filter((m: any) => m.state !== 'dead' && !back(m)).map((m: any) => `${m.callsign} ${flat(m.char.pos, d.frameCentre).toFixed(1)} m`); }
      if (!isNaN(stepT) && !out.offAtStep) out.offAtStep = sent.filter((m: any) => m.state !== 'dead' && !back(m)).map((m: any) => `${m.callsign} ${flat(m.char.pos, d.frameCentre).toFixed(1)} m`); }
    if (FR) {
      if (isNaN(out.fragT) && op.phase === 'thr' && (hold ? t - out.toldT >= hold.after - frag : t - thrT >= Math.max(0, frag))) { const far = (m: any) => flat(m.char.pos, d.frameCentre), lp = (hold ? op.lead : sent.filter((m: any) => m.state !== 'dead').sort((a: any, b: any) => far(b) - far(a))[0] ?? op.lead).char.pos; out.fragT = t;   // (at the lead's feet before Sam speaks; at the rear-most man's in a hold that ends by itself — the lead's own slice already keeps that hold while HE is off it)
        for (const m of sent) if (m.state !== 'dead') { godWas.set(m, !!m.god); m.god = true; }
        g.items.spawn('frag', [lp[0], 0.6, lp[2]], [0, 0, 0], { fuse: FRAG_FUSE, who: R.hostiles[0].char.id }); }
      if (!isNaN(out.fragT)) { for (const m of sent) if (m.state !== 'dead' && FR.fragRunning(g, m) && !out.fragRan.includes(m.callsign)) out.fragRan.push(m.callsign);
        if (isNaN(out.backT) && out.fragRan.length && sent.every((m: any) => m.state === 'dead' || (!FR.fragRunning(g, m) && back(m)))) out.backT = t - thrT;
        if (godWas.size && t - out.fragT > FRAG_FUSE + 0.5) { for (const [m, was] of godWas) m.god = was; godWas.clear(); }
 } }
    if (away > 0 && hold && t - out.toldT >= hold.after - 1) {   // (run off a second BEFORE Sam speaks — the thrower is picked with nobody at the door, as under a grenade — and until `away` s after)
      const off = isNaN(out.tellT) || t - out.tellT < away;
      if (off) { if (awayAt === undefined) { awayAt = awaySpots(); if (!awayAt) out.why ??= 'no floor beyond a throw of the door to run the men off to'; }
        if (awayAt) sent.forEach((m: any, k: number) => { if (m.state === 'dead') return; if (!godWas.has(m)) godWas.set(m, !!m.god); m.god = true; const q = [awayAt![k][0], 0, awayAt![k][2]]; m.char.pos = q; m.char.vel = [0, 0, 0]; m.path = []; m.pathGoal = null; m.speed = 0; m.lastPos = [q[0], 0, q[2]]; }); }   // (put, as frag-probes.ts puts a man: no stale walk, speed or last place carried over the move)
      else if (godWas.size) { for (const [m, was] of godWas) m.god = was; godWas.clear(); } }   // (let go: each man as mortal as he was)
    for (const [m, p] of op.pod as Map<any, any>) if (!podded.has(m)) { podded.add(m); out.pods.push({ who: m.callsign, deep: -offOf(d, p.pos, side) }); }   // (the point he is FIRST sent to: the search may move him on to dead floor after)
    for (const m of sent) { if (m.state === 'dead') continue;
      if (over(m.char.pos)) { if (!out.firstIn.has(m.callsign)) out.firstIn.set(m.callsign, t); if (isNaN(stepT)) out.inBefore++; }
      if (op.thr && op.phase !== 'enter' && D.ops.includes(op) && op.rear.has(m)) { out.spareOf++; if (R.O.of(m)?.spare) out.spare++; } }
    for (const e of g.events) { if (seen.has(e)) continue; seen.add(e);
      if (e.kind === 'guardShot' && ids.has(e.who)) out.shots.push({ t: e.time, who: ids.get(e.who).callsign, outside: offOf(d, e.pos, side) > 0 });
      if (e.kind === 'bang' && !e.frag) out.bangs.push({ t: e.time, inside: -offOf(d, e.pos, side) }); }   // (the squad's own cans: a defender's grenade — frag.ts, the same event kind with `frag` on it — is no can of the kit's, wherever it bursts)
    for (const h of R.hostiles) { if (was.get(h) !== 'dead' && h.state === 'dead') { const k = ids.get(h.killerId); if (k) out.downs.push({ t, who: h.callsign ?? 'a man', by: k.callsign, byOutside: offOf(d, k.char.pos, side) > 0 }); } was.set(h, h.state); } };
  const t0 = g.time;
  await quiet(() => { for (let i = 0; i < cap * 60 && D.ops.includes(op); i++) R.step(); R.step(hold ? 240 : 30); });   // (told to leave the door: watched four seconds on — nobody drifts in after)
  R.post = null; for (const [m, was] of godWas) m.god = was; godWas.clear();   // (an order over before its men were let go: each as mortal as he was)
  out.endAt = sent.filter((m: any) => m.state !== 'dead').map((m: any) => ({ who: m.callsign, off: offOf(d, m.char.pos, side), doing: R.O.of(m)?.there ?? '(no order)' }));
  out.beats = op.beats.map((b: any) => ({ t: b.t, what: b.what })); out.ended = op.ended ?? '(still under way)'; out.secs = g.time - t0; out.bangs1 = D.bangs;
  out.alive = sent.filter((m: any) => m.state !== 'dead').length; out.holding = sent.filter((m: any) => m.state !== 'dead' && /^holding (the room$|· )/.test(R.O.of(m)?.there ?? '')).length;   // (a man the entry left on one of the room's OTHER doors — DOORTOOLS.thrDoors — holds the room from there: 'holding · <door>')
  if (op.search) { out.left = op.search.left; out.cells = op.search.pts.length; }
  for (const m of sent) { if (m.state !== 'dead') continue; const k = R.hostiles.find((h: any) => h.char.id === m.killerId); if (k && g.fight.defs.find((x: any) => x.gd === k)?.room !== room) out.yard.push({ who: m.callsign, by: k.callsign ?? 'a man' }); }
  // the room's OTHER doors (DOORTOOLS.thrDoors): the men the entry's last act put on them, watched until each is on his mark (a walker has a few metres to go)
  const onDoor = () => sent.filter((m: any) => m.state !== 'dead' && D.held.get(m)?.door && R.O.of(m) === D.held.get(m).A);
  if (onDoor().length) { await quiet(() => { for (let i = 0; i < 15 * 60 && onDoor().some((m: any) => !R.O.of(m).arrived); i++) R.step(); });
    if (LEAFBREAK) for (const m of onDoor()) { const o = D.held.get(m).door, stop = o.swing > 0 ? o.maxA : o.minA, th = o.def.closedDir + stop + Math.sign(stop) * 0.35; m.char.pos[0] = o.def.hinge[0] + Math.cos(th) * o.def.width * 0.7; m.char.pos[2] = o.def.hinge[1] + Math.sin(th) * o.def.width * 0.7; }
    out.doorMen = onDoor().map((m: any) => { const o = D.held.get(m).door, fc = o.frameCentre, p = m.char.pos, l = flat(p, fc) || 1, e = room.entries.find((x: any) => x.door === o);
      // (the line: to the floor a step inside the frame, on the room's face — a point 0.4 m from the frame's middle TOWARD him lies in the jamb for a man
      //  stood down the door's own wall, and read 'no line' of a man who has one. The leaf: the hinge, its tip, his line, with the leaf stood anywhere
      //  from where a leaf pushed open comes to rest (1.45 rad, doors.ts) to its stop — this file's own sum, on where he ended, not the rule's on
      //  where it sent him)
      const at = e ? [fc[0] - e.out[0] * 0.4, 1.2, fc[2] - e.out[1] * 0.4] : [fc[0] + (p[0] - fc[0]) / l * 0.4, 1.2, fc[2] + (p[2] - fc[2]) / l * 0.4];
      const stop = o.swing > 0 ? o.maxA : o.minA, h = o.def.hinge, side = (ax: number, az: number, bx: number, bz: number, cx: number, cz: number) => Math.sign((bx - ax) * (cz - az) - (bz - az) * (cx - ax));
      const across = (a: number) => { const th = o.def.closedDir + a, tip = [h[0] + Math.cos(th) * (o.def.width + 0.05), h[1] + Math.sin(th) * (o.def.width + 0.05)];
        return side(p[0], p[2], at[0], at[2], h[0], h[1]) !== side(p[0], p[2], at[0], at[2], tip[0], tip[1]) && side(h[0], h[1], tip[0], tip[1], p[0], p[2]) !== side(h[0], h[1], tip[0], tip[1], at[0], at[2]); };
      let leaf = !o.off && across(stop); for (let a = Math.min(1.45, Math.abs(stop)); !o.off && !leaf && a < Math.abs(stop); a += 0.1) leaf = across(Math.sign(stop) * a);
      return { who: m.callsign, door: o.def.name, arrived: !!R.O.of(m).arrived, dist: l, leaf, line: !g.col.segmentBlocked([p[0], 1.5, p[2]], at), inRoom: room.cells.has(Math.floor(p[2] / nav.cell) * nav.W + Math.floor(p[0] / nav.cell)) }; }); }
  return out;
}
/** how many men the step in sent (a pair that has lost a man steps in with one: he threw, and he is first) */
/** threshold-grenade: the fuse of the grenade doorRun's `frag` puts down */
const FRAG_FUSE = 2.4;
const stepped = (r: Run) => (r.beats.find(b => /^step in · /.test(b.what))?.what.slice(10).split(', ').length ?? 0);
const line = (r: Run) => `${r.scn}:${r.seed} ${r.door} (${r.n} men)`;
const story = (r: Run) => r.beats.map(b => `${f1(b.t - r.beats[0].t)} ${b.what}`).join(' → ');

/** the checks every run must pass (structure), and what it showed of the two things a run can only show when the room gives it the chance (a defender in
 *  sight from outside; floor nobody could see from the jambs) */
function judge(verdict: Verdict, runs: Run[], four: boolean) {
  const ran = runs.filter(r => !r.why);
  verdict(ran.length === runs.length, 'the order is taken on a held room\'s door in every run (thresholdOn by the door\'s name: the order under way, not a refusal)', runs.map(r => r.why ? `${line(r)}: REFUSED — ${r.why}` : line(r)).join(' · '));
  // a man on each jamb as the leaf moves, everybody outside
  const jambs = (r: Run) => { const hi = r.atOpen.filter(m => m.along > r.W && m.along < r.W + 1.6 && m.off > 0 && m.off < 1.3), lo = r.atOpen.filter(m => m.along < 0 && m.along > -1.6 && m.off > 0 && m.off < 1.3); return { hi, lo, out: r.atOpen.every(m => m.off > 0) }; };
  // GLASS IS NOT COVER (orders.ts stackPosts, GLASSPEN.stackEyes — live in the page, 0 lines while rounds stop in glass): a jamb the room's rifles reach through a
  // sidelight is nobody's place, so at such a door the stack is ONE wall's — the lead on his jamb, the rest filed behind him down it, the stage saying 'no floor at
  // the other jamb'. Where both jambs are in such lines (stackLeast) it is the hand FEWER of them reach. Asked there: the bare jamb is in those lines, every man's
  // own place is in fewer, the places dealt summed are stackLeastK or less of a man on each jamb, and the file is the lead's wall's (each within 1.6 m of the man ahead).
  const edge = (r: Run, m: Run['atOpen'][number]) => m.along > r.W / 2 ? m.along - r.W : -m.along, pane = (r: Run) => GLASSPEN.stackEyes > 0 && (r.glassJamb ?? 0) >= GLASSPEN.stackEyes;
  const oneWall = (r: Run) => { const L = r.atOpen.find(m => m.lead), file = [...r.atOpen].sort((a, b) => edge(r, a) - edge(r, b));
    return !!L && pane(r) && file[0] === L && r.beats.some(b => /^threshold ·/.test(b.what) && b.what.includes('no floor at the other jamb'))
      && file.every((m, k) => (m.along > r.W / 2) === (L.along > r.W / 2) && m.off > 0 && m.off < 1.3 && edge(r, m) - (k ? edge(r, file[k - 1]) : 0) > 0 && edge(r, m) - (k ? edge(r, file[k - 1]) : 0) < 1.6 && (m.glass ?? 0) < r.glassJamb!)
      && file.reduce((s, m) => s + (m.glass ?? 0), 0) <= GLASSPEN.stackLeastK * (r.glassJamb! + (L.glass ?? 0)); };
  verdict(ran.every(r => { const j = jambs(r); return (j.hi.length >= 1 && j.lo.length >= 1 && j.out && j.hi.some(m => m.lead) !== j.lo.some(m => m.lead)) || oneWall(r); }), 'as the leaf moves a man stands on EACH jamb — within 1.6 m of his edge of the opening, on the wall — the lead one of them, and every man of the order is outside the frame line' + (ran.some(pane) ? ' · a jamb the room reaches through glass is left bare (GLASSPEN.stackEyes): there the lead holds his own and the rest file behind him on that wall, every place in fewer such lines' : ''),
    ran.map(r => `${line(r)}: ${r.atOpen.map(m => `${m.who}${m.lead ? ' (lead)' : ''} ${m.along > r.W ? `latch +${f1(m.along - r.W)}` : m.along < 0 ? `hinge +${f1(-m.along)}` : `IN THE OPENING ${f1(m.along)}`} · ${f1(m.off)} off${pane(r) ? ` · ${m.glass ?? 0} through glass` : ''}`).join(', ')}${pane(r) ? ` · the other jamb: ${r.glassJamb} lines through glass` : ''}`).join('\n    '));
  // A RUN SHOT OFF THE DOOR FROM OUTSIDE THE ROOM: every man of the order lost before any step in, each to a defender who is not at home in the room behind the
  // door (the depot's a_s on the floodlit yard — heldEntries: 'who is left on the jambs is that fight's dice, on either stand'). The lines from the phases on
  // ask what the THRESHOLD did with the room; such a run never had a hold to ask of. Passed over there, said here — and never the many, nor a whole floor.
  const shotOff = (r: Run) => r.n > 0 && r.yard.length === r.n && idx(r, /^step in/) < 0 && r.ended === 'nobody left on it', kept = ran.filter(r => !shotOff(r)), off = ran.filter(shotOff);
  if (off.length) verdict(off.length * 3 <= ran.length && new Set(kept.map(r => r.scn)).size === new Set(ran.map(r => r.scn)).size, 'A RUN SHOT OFF THE DOOR FROM OUTSIDE THE ROOM is passed over by the lines below (the phases, the pockets, the step in, the hand-over): every man of the order lost before any step in, each to a defender who is not at home in the room behind the door — a door another room watches, where who is left on the jambs is that fight\'s dice. Never more than one run in three, and never every run of a floor',
    off.map(r => `${line(r)}: ${r.yard.map(y => `${y.who} by ${y.by}`).join(', ')} · ended '${r.ended}' after ${f1(r.secs)} s`).join('\n    '));
  // the phases, in order
  const firstCan = (r: Run) => DOORTOOLS.thrFirst > 0 && r.beats.some(b => /^threshold ·.* · a can first$/.test(b.what));
  const ordered = (r: Run) => { const seq = ['^stacked$', '^threshold ·', '^held ', '^step in', '^cross:', '^handover$', '^searched ·', '^in$'].map(s => idx(r, new RegExp(s))); const toss = idx(r, /^toss:/), bang = idx(r, /^bang ·/), none = idx(r, /^no bang ·/);
    const mono = seq.every((x, i) => x >= 0 && (i === 0 || x > seq[i - 1]));
    const can = toss >= 0 ? toss > seq[2] && bang > toss && bang < seq[3] : none > seq[2] && none < seq[3];
    if (firstCan(r)) {   // (DOORTOOLS.thrFirst, a floor already loud as the leaf moves: the can goes in AHEAD of the hold — its own order, and nothing else out of place)
      const out = idx(r, /^out onto the diagonals behind the can$/), later = r.beats.map((b, k) => /^toss:/.test(b.what) ? k : -1).filter(k => k > toss);
      return mono && (DOORTOOLS.thrFirst >= 2 || r.firstHot === true) && / · first · /.test(r.beats[toss]?.what ?? '') && toss > seq[1] && bang > toss && out > bang && out < seq[2]
        && later.every(k => k > seq[2] && k < seq[3] && r.beats.slice(k, seq[3]).some(b => /^bang ·/.test(b.what))) && !later.length
        && /^stack,(kick,)?pocket,fuse,pocket,thr,pocket,(fuse,pocket,)*(wait,)?enter$/.test(r.phases.join(',')); }
    return mono && can && /^stack,(kick,)?thr,pocket,(fuse,pocket,)*(wait,)?enter$/.test(r.phases.join(',')); };
  verdict(kept.every(ordered), 'the phases come in order: stacked → threshold (the jambs held) → held → the pockets (toss → bang, or \'no bang\' said why) → step in → first man over the line → hand-over → searched → in' + (ran.some(firstCan) ? ' · on a floor already loud a can goes in first (thrFirst): threshold → toss → bang → out onto the diagonals → held, and on as written' : ''),
    kept.map(r => `${line(r)} [${r.phases.join(' ')}]: ${story(r)}`).join('\n    '));
  // nobody in before the step in — by the probe's own watch of every man's feet
  verdict(ran.every(r => r.inBefore === 0 && [...r.firstIn.values()].every(t => t >= tOf(r, /^step in/))), 'NOBODY is over the frame line, nor on the room\'s floor, on any frame before \'step in\' (every man of the order watched every frame — not the module\'s own beats)',
    ran.map(r => `${line(r)}: ${r.inBefore} man-frames inside before the step in at ${f1(tOf(r, /^step in/) - r.beats[0].t)} s · first feet inside ${[...r.firstIn].map(([w, t]) => `${w} ${f1(t - r.beats[0].t)} s`).join(', ') || 'nobody'} · worked from outside for ${f1(tOf(r, /^step in/) - tOf(r, /^threshold ·/))} s`).join('\n    '));
  // engaged from outside
  const outside = (r: Run) => { const ts = tOf(r, /^step in/); return { rounds: r.shots.filter(s => s.t < ts && s.outside).length, in: r.shots.filter(s => s.t < ts && !s.outside).length, down: r.downs.filter(x => x.t < ts && x.byOutside).length, contact: r.beats.filter(b => /^contact:/.test(b.what) && b.t < ts).length }; };
  verdict(ran.some(r => outside(r).rounds > 0 && outside(r).contact > 0) && ran.every(r => outside(r).in === 0), 'the room\'s men are ENGAGED FROM OUTSIDE: in at least one run a man at the door has one of theirs in his sight and rounds go in through the opening before anybody steps in — and no round before the step in is fired from inside, in any run',
    ran.map(r => { const o = outside(r); return `${line(r)}: ${o.contact} at the door with a man in sight · ${o.rounds} rounds from outside · ${o.down} of theirs down from out there`; }).join('\n    '));
  // the blind pockets
  const banged = kept.filter(r => idx(r, /^toss:/) >= 0);
  const pocket = (r: Run) => { const ts = tOf(r, /^step in/), first = r.beats.find(b => /^cross:/.test(b.what))?.what.slice(6), throwers = r.beats.filter(b => /^toss:/.test(b.what)).map(b => b.what.slice(5).split(' · ')[0]);
    return { throwers, first, ok: r.bangs.length === throwers.length && r.bangs.every(b => b.inside > 0.5 && b.t < ts) && r.bangs0 - r.bangs1 === throwers.length && (stepped(r) < 2 || throwers.every(w => w !== first)) && throwers.length <= DOORTOOLS.thrBangs }; };
  verdict(banged.length > 0 && banged.every(r => pocket(r).ok) && kept.every(r => idx(r, /^toss:/) >= 0 || idx(r, /^no bang ·/) >= 0), 'THE BLIND POCKETS ARE BANGED: a can from the kit\'s own count, thrown by a man who is not the first man in (two or more stepping in), bursts inside the room before anybody crosses — in at least one run; a run with no can says why',
    kept.map(r => { const p = pocket(r), lit = r.beats.filter(b => /^pocket banged/.test(b.what)).map(b => b.what.replace('pocket banged · ', '')).join('; '); return `${line(r)}: ${p.throwers.length ? `${p.throwers.join(', ')} threw · ${r.bangs.map(b => `burst ${f1(b.inside)} m inside`).join(', ')} · ${lit || 'no floor model'} · bangs ${r.bangs0} → ${r.bangs1} · first man in ${p.first}` : r.beats.find(b => /^no bang/.test(b.what))?.what}`; }).join('\n    '));
  // the shallow step
  verdict(kept.every(r => r.pods.length > 0 && r.pods.every(p => p.deep > 0.2 && p.deep <= 2.5 + 1e-6) && (idx(r, /^toss:/) < 0 || tOf(r, /^cross:/) >= Math.max(...r.bangs.map(b => b.t)) + DOORTOOLS.afterBang - 0.05)), 'THE STEP IN IS SHALLOW: every point a man is sent to is inside the room and 2.5 m or less from the frame line (the search may send a man on to dead floor after) — and behind a can nobody crosses before the bang\'s own count',
    kept.map(r => `${line(r)}: ${r.pods.map(p => `${p.who} ${f1(p.deep)} m in`).join(', ')}${r.bangs.length ? ` · first over the line ${f1(tOf(r, /^cross:/) - Math.max(...r.bangs.map(b => b.t)))} s after the last burst` : ''}`).join('\n    '));
  // the hand-over
  verdict(kept.every(r => r.ended === 'in' && r.holding === r.alive && (isNaN(r.cells) || idx(r, /^searched ·/) > idx(r, /^handover$/))), 'THE HAND-OVER: from the step in it is the entry\'s own room clear — the floor looked over by its search, the order ended \'in\', every man of it still up left holding the room',
    kept.map(r => `${line(r)}: ended '${r.ended}' after ${f1(r.secs)} s · ${r.holding} of ${r.alive} up holding the room (${r.n - r.alive} lost) · ${isNaN(r.cells) ? 'no floor model' : `${r.left} of ${r.cells} cells left unseen`} · ${r.beats.find(b => /^held /.test(b.what))?.what}`).join('\n    '));
  if (four) verdict(ran.every(r => r.atOpen.length - 2 >= 1 && r.spareOf > 0 && r.spare >= 0.8 * r.spareOf), 'the squad of four: the men on neither jamb keep the squad\'s back — their eyes spare for the way a man could come at it — while the room is worked (until one turns in to throw, and the step in)',
    ran.map(r => `${line(r)}: ${r.atOpen.length - 2} behind the jamb men · eyes spare ${r.spare} of ${r.spareOf} man-frames`).join('\n    '));
}

export const thresholdPair = guarded(async () => {
  const { verdict, done } = family('threshold-pair: the pair works a held room from the threshold — the records office and the office');
  const runs: Run[] = []; for (const scn of ['records-pair', 'room-clear-pair']) runs.push(await doorRun(scn, 1));
  judge(verdict, runs, false);
  await worded(verdict, 'records-pair');
  return done();
});
/** the word (doorTools.go — F) a second into the hold: the hold is cut short there, and the rest follows as ever */
async function worded(verdict: Verdict, scn: string) {
  const r = await doorRun(scn, 1, null, 90, 1), i = (re: RegExp) => idx(r, re), held = r.beats.find(b => /^held /.test(b.what)), secs = held ? parseFloat(held.what.slice(5)) : NaN;
  verdict(!r.why && i(/^the word$/) > i(/^threshold ·/) && i(/^held /) > i(/^the word$/) && secs < 2 && i(/^step in/) > i(/^held /) && r.inBefore === 0 && r.ended === 'in',
    'THE WORD cuts the hold short: given a second in, the jambs are let go there and then (well under the shortest hold), the pockets and the step in follow, nobody in before it', `${line(r)}: ${story(r)} · ended ${r.ended}`);
}
export const thresholdFour = guarded(async () => {
  const { verdict, done } = family('threshold-four: the squad of four works a held room from the threshold — the office');
  YARDOUT = true;
  const runs: Run[] = []; runs.push(await doorRun('room-clear-four', 1));
  judge(verdict, runs, true);
  await worded(verdict, 'room-clear-four');
  return done();
});

/** A CAN FIRST ON A HOT DOOR (DOORTOOLS.thrFirst): the same doors with the floor gone loud in the team's ears, and the switch's three positions */
export const thresholdFirst = guarded(async () => {
  const { verdict, done } = family('threshold-first: on a floor that has gone loud a can goes in before anybody shows in the opening (DOORTOOLS.thrFirst)');
  const scns = ['records-pair'], i = (r: Run, re: RegExp) => idx(r, re), cans = (r: Run) => r.beats.filter(b => /^toss:/.test(b.what)).length;
  YARDOUT = true;
  const hot: Run[] = []; for (const scn of scns) hot.push(await doorRun(scn, 1, null, 90, NaN, null, true));
  const first = (r: Run) => !r.why && / · a can first$/.test(r.beats.find(b => /^threshold ·/.test(b.what))?.what ?? '');
  verdict(hot.every(first), 'the floor hot, the stage says so as the leaf moves: \'threshold · … · a can first\'', hot.map(r => `${line(r)}: ${r.why ?? r.beats.find(b => /^threshold ·/.test(b.what))?.what}`).join('\n    '));
  const seq = (r: Run) => { const thr = i(r, /^threshold ·/), toss = i(r, /^toss:[^·]+ · first · /), bang = i(r, /^bang ·/), out = i(r, /^out onto the diagonals behind the can$/), held = i(r, /^held /), step = i(r, /^step in/);
    return thr >= 0 && toss > thr && bang > toss && out > bang && held > out && step > held && tOf(r, /^out onto/) - r.bangs[0]?.t >= DOORTOOLS.afterBang - 0.05; };
  verdict(hot.every(seq), 'THE ORDER OF IT: the leaf moves → the first can (toss · first) → its bang → \'out onto the diagonals behind the can\', no sooner than a bang\'s own count after the burst → the hold → step in',
    hot.map(r => `${line(r)} [${r.phases.join(' ')}]: ${story(r)}`).join('\n    '));
  verdict(hot.every(r => r.heldBack <= 0.5 && r.inBefore === 0), 'NOBODY SHOWS IN THE OPENING BEFORE IT: from the leaf moving until they are sent out, no man of the order gets half a metre further off his mark on the wall than he stood as it moved — and nobody is over the frame line before the step in',
    hot.map(r => `${line(r)}: furthest any man got off his mark before 'out' ${f1(r.heldBack)} m · ${r.inBefore} man-frames inside before the step in`).join('\n    '));
  verdict(hot.every(r => r.bangs0 - r.outBangs === 1 && cans(r) <= DOORTOOLS.thrBangs && r.bangs0 - r.bangs1 === cans(r) && r.bangs.length === cans(r) && r.bangs.every(b => b.inside > 0.5)), 'THE CAN IS ONE OF THE STAGE\'S, FROM THE KIT: one spent by the time they step out, never more than thrBangs for the door, every one burst inside the room, the kit down by what was thrown',
    hot.map(r => `${line(r)}: kit ${r.bangs0} → ${r.outBangs} at 'out' → ${r.bangs1} · ${cans(r)} thrown · ${r.bangs.map(b => `burst ${f1(b.inside)} m inside`).join(', ')}`).join('\n    '));
  // a can that went in first is the door's only can — and on the word too
  verdict(hot.every(r => cans(r) === 1), 'THAT CAN IS THE DOOR\'S CAN: one thrown and no more, pockets or not', hot.map(r => `${line(r)}: ${cans(r)} thrown`).join('\n    '));
  verdict(hot.every(r => r.ended === 'in' && r.holding === r.alive), 'the rest is the stage\'s own: ended \'in\', every man of it still up left holding the room', hot.map(r => `${line(r)}: ended '${r.ended}' after ${f1(r.secs)} s · ${r.holding} of ${r.alive} up holding (${r.n - r.alive} lost)`).join('\n    '));
  // a quiet floor gets the stage as it was · held until told: the cans are Sam's to call
  const plain = (r: Run) => !r.why && !first(r) && i(r, /^(out onto the diagonals|no can first)/) < 0 && isNaN(r.heldBack);
  const quiet1 = await doorRun('records-pair', 1);
  verdict(plain(quiet1), 'a quiet floor gets the stage as it was', `quiet floor: ${story(quiet1)}`);
  // (the runs that give the word from code go last: that call the Rust game may not answer, and the lines above are asked first)
  // the word given while that can is in the air: kept, and taken the moment they step out — no hold is sat through first
  const early = await doorRun('records-pair', 1, null, 90, -1, null, true), eOut = tOf(early, /^out onto the diagonals behind the can$/), eWord = tOf(early, /^the word$/);
  verdict(first(early) && i(early, /^the word$/) >= 0 && cans(early) === 1 && early.ended === 'in', 'ONE CAN A DOOR, ON THE WORD TOO: the word given on a door whose can went in first is taken, and no second is thrown', `${line(early)}: ${cans(early)} thrown on the word: ${story(early)}`);
  verdict(first(early) && eWord >= eOut - 1e-6 && eWord - eOut < 0.05 && i(early, /^held /) > i(early, /^the word$/) && tOf(early, /^held /) - eOut < 0.05 && early.ended === 'in',
    'THE WORD WHILE THE CAN IS IN THE AIR is kept: nobody is on a diagonal yet — it is taken as they step out behind the burst, the hold cut short there and then',
    `hot floor, the word at the toss: ${story(early)}`);
  return done();
});

/** THE OTHER ORDER: work it from the threshold and hold until told — and everything Sam can then say */
export const thresholdHold = guarded(async () => {
  const { verdict, done } = family('threshold-hold: worked from the threshold and HELD until told — the pair (the records office) — then bang and step in');
  const AFTER = 10, scn = 'records-pair';
  const bang = await doorRun(scn, 1, null, 120, NaN, { tell: 'bang', after: AFTER }, false, true);
  const all = [bang], i = (r: Run, re: RegExp) => idx(r, re), rel = (r: Run, t: number) => f1(t - r.beats[0].t);
  const tossed = (r: Run) => r.beats.filter(b => /^toss:/.test(b.what)).length, feet = (r: Run) => [...r.firstIn].map(([w, t]) => `${w} ${rel(r, t)} s`).join(', ') || 'nobody';
  verdict(all.every(r => !r.why && /· to hold$/.test(r.beats.find(b => /^threshold ·/.test(b.what))?.what ?? '')), 'the order is taken, to hold, in every run (thresholdOn(door, who, side, true))', all.map(r => r.why ? `${line(r)}: REFUSED — ${r.why}` : line(r)).join(' · '));
  // the hold itself: quiet said once, then nothing until told
  verdict([bang].every(r => r.toldT > 0 && r.tellT - r.toldT >= AFTER - 0.05 && r.tellPhase === 'thr' && r.tellBangs === r.bangs0 && [...r.firstIn.values()].every(t => t > r.tellT) && r.beats.filter(b => /^holding · on your word/.test(b.what)).length === 1 && !r.beats.some(b => b.t < r.tellT && /^(held |toss:|step in|no bang)/.test(b.what))),
    `THE HOLD: the door gone quiet, the lead says so once ('holding · on your word' — the moment the plain order goes on by itself) and for ${AFTER} s more the order is still on the jambs: nobody over the frame line or on the room's floor, no can spent, no pockets, no step in`,
    [bang].map(r => `${line(r)}: threshold at ${rel(r, tOf(r, /^threshold ·/))} s · 'holding' at ${rel(r, r.toldT)} s · told at ${rel(r, r.tellT)} s with the order in '${r.tellPhase}', bangs ${r.bangs0} → ${r.tellBangs} · first feet inside: ${feet(r)} · ${r.shots.filter(s => s.t < r.tellT && s.outside).length} rounds from outside while holding`).join('\n    '));
  // bang, then step in
  verdict([bang].every(r => r.tellR === null && i(r, /^told · bang and step in$/) >= 0 && tossed(r) >= 1 && r.bangs.length === tossed(r) && r.bangs0 - r.bangs1 === tossed(r) && r.bangs.every(b => b.inside > 0.5 && b.t > r.tellT && b.t < tOf(r, /^step in/)) && tOf(r, /^cross:/) >= Math.max(...r.bangs.map(b => b.t)) + DOORTOOLS.afterBang - 0.05 && r.inBefore === 0 && r.ended === 'in'),
    'TOLD TO BANG AND STEP IN (row 2 of the door\'s tools): a can from the kit goes in and bursts inside the room before anybody crosses — one at the least, blind pocket or not — then the step in behind the bang\'s own count',
    [bang].map(r => `${line(r)}: ${story(r)} · ${r.bangs.map(b => `burst ${f1(b.inside)} m inside`).join(', ')} · bangs ${r.bangs0} → ${r.bangs1} · ended '${r.ended}'`).join('\n    '));
  return done();
});

/** Nobody is left on another door of the room where that door's own leaf, stood open, hides it from him. The depot, the first
 *  held door the yard does not watch, the yard's guns out, three seeds. THRDOORSLEAF_BREAK=1: each such man is stood behind that leaf by hand — red. */
export const thresholdLeaf = guarded(async () => {
  const { verdict, done } = family('threshold-leaf: a man left on another door of the room is never put where that door\'s own leaf, stood open, hides it');
  YARDOUT = true; LEAFBREAK = process.env.THRDOORSLEAF_BREAK === '1';
  const play = async () => [await doorRun('depot-four', 1, null, 90, NaN, null, false, true)];
  const tell = (rs: Run[]) => rs.map(r => `${r.scn}:${r.seed} ${r.door}: ${r.doorMen.length ? r.doorMen.map(m => `${m.who} on ${m.door} from ${f1(m.dist)} m${m.arrived ? '' : ' · NOT THERE'}${m.line ? '' : ' · NO LINE'}${m.inRoom ? '' : ' · OUTSIDE THE ROOM'}${m.leaf ? ' · BEHIND ITS LEAF' : ''}`).join(', ') : 'nobody on another door'} · ended '${r.ended}'`).join('\n    ');
  const on = await play(), men = (rs: Run[]) => rs.flatMap(r => r.doorMen);
  verdict(men(on).length > 0 && men(on).every(m => m.arrived && m.line && m.inRoom && !m.leaf) && on.every(r => r.ended === 'in'),
    'every man left on another door is on his mark in the room with a line to it that its open leaf does not cross', tell(on));
  return done();
});

export const thresholdDoors = guarded(async () => {
  const { verdict, done } = family('threshold-doors: every door of the seven scenarios\' levels, both faces — wherever a stack fits, the stage has shallow floor to step in to');
  const lines: string[] = []; let bad = 0, faces = 0, both = 0, one = 0, none = 0;
  for (const E of SCENARIOS as any[]) {
    const R = await stage(E.id, 1), g = R.g, D = R.D, men = R.team; let f = 0, b2 = 0, o1 = 0, no = 0; const worst: string[] = [];
    for (const d of g.doors.list) for (const side of [1, -1] as const) {
      const posts = D.stackPosts(d, side, men); if (!posts) { no++; continue; } f++;
      const W = d.def.width, lead = posts.filter((P: any) => P.lead), split = lead.some((P: any) => alongOf(d, P.pos) > W / 2) && lead.some((P: any) => alongOf(d, P.pos) < W / 2); if (split) b2++; else o1++;
      const pods = D.shallowPods(d, side, Math.max(2, men.length)), deep = pods.map((p: any) => -offOf(d, p.pos, side));
      if (!pods.length || deep.some((x: number) => x <= 0 || x > 2.5 + 1e-6)) { bad++; worst.push(`${d.def.name}/${side > 0 ? '+' : '−'}: ${pods.length ? deep.map(f1).join(', ') + ' m in' : 'NO FLOOR INSIDE'}`); } }
    faces += f; both += b2; one += o1; none += no;
    lines.push(`${E.id} (${men.length} men, ${g.doors.list.length} doors): ${f} faces take a stack — a man on each jamb on ${b2}, one jamb only on ${o1} — ${no} have no floor at either${worst.length ? ` · BAD: ${worst.join(' · ')}` : ''}`);
  }
  verdict(bad === 0 && faces > 0, 'on every face of every door where the stack\'s own test passes, the stage has points to step in to, all inside the room and 2.5 m or less from the frame line', `${faces} faces over seven scenarios · ${both} with a man on each jamb, ${one} with one jamb only (the lead slices; nobody holds a far jamb that has no floor) · ${none} no stack at all\n    ${lines.join('\n    ')}`);
  return done();
});

/** playthrough.ts in a child process with these env words over the caller's, seeds `seeds` of every scenario: its runs */
async function playthrough(env: Record<string, string>, seeds: string, set?: Record<string, Record<string, unknown>>, scn?: string, drop: string[] = []): Promise<{ runs: any[]; err: string }> {
  const E: Record<string, string | undefined> = { ...process.env, PT_TOOL: '', PT_DIGEST: '', PT_STACKWAIT: '', PT_PAUSE: '', PT_NOISE: '', PT_DESTR: '0', ...env }; for (const k of drop) delete E[k];   // (`drop`: words the run is to have NO say on — the stand's own value plays)
  const file = resolve(tmpdir(), `threshold-probe-${process.pid}-${Object.keys(env).join('-')}${set ? `-${JSON.stringify(set).length}${JSON.stringify(set).replace(/[^a-z0-9]/gi, '').slice(-24)}` : ''}.json`);
  const pr = Bun.spawn(['bun', 'run', resolve(ROOT, 'tools/qa/playthrough.ts'), '--seeds', seeds, '--jobs', '4', '--json', file, ...(scn ? ['--scn', scn] : []), ...(set ? ['--set', JSON.stringify(set)] : [])], { stdout: 'ignore', stderr: 'pipe', env: E });   // (PT_DESTR 0: these runs are held against words frozen before the destruction goal — its one switch stays off, as every other switch since is pinned)
  const err = await new Response(pr.stderr).text(); await pr.exited;
  try { const runs = JSON.parse(readFileSync(file, 'utf8')).runs; rmSync(file); return { runs, err }; } catch { return { runs: [], err }; }
}
/** NO CAN FROM A HAND THAT IS NOT AT THE DOOR. Staged: a threshold given to hold, the men run off the door a second before Sam says
 *  'bang, then step in' (doorRun's `away`: what a grenade landing by the stack does to them), and what the can then does — and past its wait, and
 *  with nobody run off at all. THRHAND_BREAK=1: nobody is run off the door — red. */
export const thresholdHand = guarded(async () => {
  const { verdict, done } = family('threshold-hand: a threshold\'s can is thrown only by a man within reach of the frame, and waits for one');
  const BREAK = process.env.THRHAND_BREAK === '1', K = DOORTOOLS, reach = K.throwReach, hold: Hold = { tell: 'bang', after: 2 };
  const DOORS: [string, string | null][] = [['records-pair', null]];
  const STEP = 0.05;   // (the probe looks at the thrower once the frame is over: a walking man's stride after the rule's own look at him)
  const cans = (r: Run) => r.beats.filter(b => /^toss:/.test(b.what)).length, has = (r: Run, re: RegExp) => idx(r, re) >= 0, firstIn = (r: Run) => r.beats.find(b => /^cross:/.test(b.what))?.what.slice(6);
  const leadOf = (r: Run) => r.beats.find(b => /^ordered · /.test(b.what))?.what.split(' · lead ')[1] ?? '';
  const tell = (r: Run) => `${line(r)}: ${r.why ?? `${r.tossAt.map(x => `${x.who} threw from ${x.near.toFixed(2)} m${x.who === x.lead ? ' (the lead)' : ''}`).join(', ') || 'no can'} · lead ${leadOf(r)} · first man in ${firstIn(r) ?? '—'} · kit ${r.bangs0} → ${r.bangs1} · ${r.alive} of ${r.n} up · ended '${r.ended}' · ${story(r)}`}`;
  const on: Run[] = [], late: Run[] = [];
  const AWAY = BREAK ? 0 : 1;   // (s after Sam speaks that the men are let go: they have most of the wait to walk back in)
  for (const [scn, door] of DOORS) {
    on.push(await doorRun(scn, 1, door, 120, NaN, hold, false, false, AWAY)); late.push(await doorRun(scn, 1, door, 120, NaN, hold, false, false, K.thrHandWait + 3));
  }
  const WAITS = /^can waits: no hand at the door/, LATE = /^can waited /;   // (the whole prefix: 'can waits: the door has come shut' is bangWay's)
  const waited = (r: Run) => tOf(r, /^toss:/) - tOf(r, WAITS);
  verdict(on.every(r => !r.why && has(r, WAITS) && !has(r, LATE) && r.tossAt.length >= 1 && r.tossAt.every(x => x.near < reach + STEP) && r.bangs.length === cans(r) && r.bangs.every(b => b.inside > 0.5) && (stepped(r) < 2 || r.tossAt.every(x => x.who !== x.lead)) && r.ended === 'in'),
    `THE CAN WAITS FOR A HAND AT THE DOOR: 'can waits' is said, nothing is thrown while nobody is within ${reach} m of the frame, and once the men are back it is thrown by a man who is — not the lead while two or more step in — bursts inside the room, and the order ends 'in' (who crosses first, with the men still walking back as they step in, is not this rule's: printed, not asked)`, on.map(r => `waited ${f1(waited(r))} s · ${tell(r)}`).join('\n    '));
  const NONE = /^no bang · nobody at the door/;
  verdict(late.every(r => !r.why && has(r, WAITS) && has(r, NONE) && cans(r) === 0 && r.bangs.length === 0 && r.bangs1 === r.bangs0 && tOf(r, /^step in/) >= tOf(r, NONE)),
    `… AND NO LONGER THAN thrHandWait (${K.thrHandWait} s): nobody back by then — 'no bang · nobody at the door' is said, nothing is thrown, the kit's count is what it was, and the order goes on to its step in (how an entry with no can then ends is the room fight's, and printed)`, late.map(r => `${tell(r)}`).join('\n    '));
  return done();
});

/** DOORTOOLS.thrLast, STAGED BY HAND on the registry's hostage door: the order given, one jamb man put ON his mark and the other 3 m or more down THAT man's
 *  wall (to reach his own jamb he would walk across the opening), the captor's clock put at its last step. `late` 'far' = the far-jamb man is the late one,
 *  'lead' = the man who opens it is; `tucked`: Sam stands in the first place behind the man on his mark (the place a late man would be given). */
async function lastStaged(seed: number, late: 'far' | 'lead', tucked = false) {
  const R = await stage('rescue-registry', seed), g = R.g, D = R.D, RS = g.rescue, K = DOORTOOLS;
  const out = { why: null as string | null, beats: [] as { t: number; what: string }[], t0: 0, late: '', early: '', lead0: '', lead1: '', sameWall: false, samGap: NaN, crossed: false, tossed: 0, held: NaN, stackedAt: NaN, lateOn: NaN, shot: 0, alive: [0, 0], flag: [false, false] };
  const hit = heldEntries(R).find(x => x.ent.door.def.name === RS.data?.room.door); if (!hit) { out.why = 'the hostage room has no entry the room fight knows'; return out; }
  const ent = hit.ent, d = ent.door, W = d.def.width, nav = g.col.nav, lx = -ent.out[1], lz = ent.out[0];
  g.teleportPlayer([ent.at[0] + ent.out[0] * 3.4 + lx * 0.2, 0, ent.at[2] + ent.out[1] * 3.4 + lz * 0.2]);
  // (the door alone is the test: the men of theirs who are not in the hostage room are taken off the floor, and the two at the door cannot be dropped)
  const inRoom = (q: number[]) => hit.room.cells.has(Math.floor(q[2] / nav.cell) * nav.W + Math.floor(q[0] / nav.cell));
  for (const h of R.hostiles) if (h.state !== 'dead' && !inRoom(h.char.pos)) killGuard(g, h, [0, 0, 1], true, 'struck');
  for (const m of R.team) m.god = true;
  const op: any = D.thresholdOn(d.def.name, 'all'); if (typeof op === 'string') { out.why = op; return out; }
  const side = op.side, lead: any = op.lead, far: any = op.men.find((m: any) => m !== lead && op.post.get(m)?.lead && (alongOf(d, op.post.get(m).pos) > W / 2) !== (alongOf(d, op.post.get(lead).pos) > W / 2));
  if (!far) { out.why = 'one jamb only has floor here'; return out; }
  const L: any = late === 'lead' ? lead : far, E: any = late === 'lead' ? far : lead, PE = op.post.get(E).pos, aE = alongOf(d, PE), dir = aE > W / 2 ? 1 : -1, oE = offOf(d, PE, side);
  let from: number[] | null = null;
  for (const back of [4, 3.5, 4.5, 3, 5, 6]) { for (const o of [1.25, 2, 0.8, oE, 2.75]) { const q = doorPt(d, aE + dir * back, o, side); if (!nav.isBlocked(q[0], q[2]) && !g.col.solidBlocked([PE[0], 0.45, PE[2]], [q[0], 0.45, q[2]])) { from = q; break; } } if (from) break; }
  if (!from) for (const o of [4, 3.5, 4.5, 3]) { const q = doorPt(d, W / 2, o, side); if (!nav.isBlocked(q[0], q[2])) { from = q; break; } }   // (no run of wall on that side — a nook: he comes from straight out in front)
  if (!from) { out.why = 'no floor to start the late man from'; return out; }
  E.char.pos = [PE[0], 0, PE[2]]; L.char.pos = [from[0], 0, from[2]];
  if (tucked) g.teleportPlayer(doorPt(d, aE + dir * K.bangFile, oE, side));
  if (isNaN(RS.t0)) RS.t0 = g.time; RS.resolve = 0.85;   // (his last step: 'about to' from the next tick, the round `resolveSecs` × 0.15 + his aim away)
  out.late = L.callsign; out.early = E.callsign; out.lead0 = lead.callsign; out.t0 = g.time;
  let inT = NaN;
  R.post = () => { const t = g.time;
    if (isNaN(inT) && op.beats.some((x: any) => /^step in/.test(x.what))) inT = t;
    if (isNaN(out.lateOn) && flat(L.char.pos, op.post.get(L)?.pos ?? L.char.pos) < 0.6) out.lateOn = t - out.t0;
    if (isNaN(inT) && L.state !== 'dead') { const a = alongOf(d, L.char.pos), o = offOf(d, L.char.pos, side); if (a > 0.1 && a < W - 0.1 && o > -0.15 && o < 2.5) out.crossed = true; } };
  await quiet(() => { for (let i = 0; i < 40 * 60 && D.ops.includes(op) && isNaN(inT); i++) R.step(); R.step(30); });
  R.post = null;
  const PL = op.post.get(L)?.pos ?? L.char.pos, sam = g.player.char.pos;
  out.beats = op.beats.map((x: any) => ({ t: x.t - out.t0, what: x.what })); out.lead1 = op.lead.callsign; out.flag = [!!op.lastStack, !!op.lastHold];
  out.sameWall = (alongOf(d, PL) > W / 2) === (aE > W / 2); out.samGap = flat(PL, sam);
  (out as any).fate = [L, E].map((m: any) => `${m.callsign}: ${m.state}, ${op.men.includes(m) ? 'on the order' : 'off it'}, '${R.O.of(m)?.there ?? R.O.of(m)?.order ?? 'no order'}' at ${m.char.pos.map((x: number) => x.toFixed(1)).join(',')}`).join(' · ');
  out.tossed = out.beats.filter(x => /^toss:/.test(x.what)).length; out.held = +(out.beats.find(x => /^held /.test(x.what))?.what.match(/^held ([\d.]+) s/)?.[1] ?? NaN);
  out.stackedAt = out.beats.find(x => x.what === 'stacked')?.t ?? NaN;
  const H = (RS.hostages ?? []).map((h: any) => h.gd ?? h); out.alive = [H.filter((h: any) => h.state !== 'dead').length, H.length]; out.shot = RS.executed.size;
  return out;
}
/** DOORTOOLS.thrLast — THE CAPTOR'S LAST WARNING IS HEARD AT THE DOOR. The door staged by hand (lastStaged): the rule's own geometry, whichever man is the late one. */
export const thresholdLast = guarded(async () => {
  const { verdict, done } = family('threshold-last: a hostage room\'s captor at his last step — the men at its door neither wait for a man still walking up nor hold');
  const f1 = (x: number) => isNaN(x) ? '—' : x.toFixed(1);
  const tell = (r: Awaited<ReturnType<typeof lastStaged>>) => r.why ? `not staged — ${r.why}` : `late ${r.late}, on his mark ${r.early}, lead ${r.lead0}${r.lead1 !== r.lead0 ? ` → ${r.lead1}` : ''} · stacked ${f1(r.stackedAt)} s, held ${f1(r.held)} s, ${r.tossed} can(s) · the late man's place ${r.sameWall ? 'on the same wall' : 'across the opening'}, ${f1(r.samGap)} m from Sam${r.crossed ? ', WALKED ACROSS THE DOOR' : ''} · hostages ${r.alive[0]}/${r.alive[1]}\n      ${(r as any).fate}\n      ${r.beats.filter(x => !/^(contact|down from)/.test(x.what)).slice(0, 9).map(x => `${f1(x.t)} ${x.what}`).join(' → ')}`;
  const A = await lastStaged(3, 'far'), B = await lastStaged(3, 'lead'), C = await lastStaged(3, 'far', true);
  const went = (r: typeof A) => !r.why && r.flag[0] && r.flag[1] && r.beats.some(x => x.what.startsWith(`last warning · not waiting for ${r.late}`)) && r.stackedAt < 1.5 && r.held < 0.5 && r.tossed === 1;
  verdict(went(A) && A.lead1 === A.lead0 && A.sameWall && !A.crossed,
    'staged, the far-jamb man 3 m or more down the lead\'s wall: the lead opens without him, nothing is held, ONE can — and the late man is given a place behind the lead on that wall, never walking across the door', tell(A));
  verdict(went(B) && B.lead1 === B.early && B.beats.some(x => x.what.includes(`the door is ${B.early}'s`)) && B.beats.some(x => x.what === 'kick'),
    '… the LEAD the late one: the door is handed to the man on his jamb, who opens it (the old lead files in behind him where that wall has floor — at this door\'s nook it has none, and he keeps his jamb)', tell(B));
  verdict(went(C) && C.sameWall && C.samGap >= 0.7 && !C.crossed,
    '… with Sam stood in the first place behind the lead: the late man is given the next one, not Sam\'s', tell(C));
  console.log(`  (staged: the captor's round — ${A.shot} shot, ${A.alive[0]}/${A.alive[1]} alive)`);
  return done();
});

/** A GRENADE AT A HELD DOOR: the order waits for its men who ran.
 *  THRFRAGHOLD_BREAK=1 = no grenade is put down: red. */
export const thresholdGrenade = guarded(async () => {
  const { verdict, done } = family('threshold-grenade: a grenade by a held door — the order waits for its men who ran');
  const K = DOORTOOLS, reach = K.throwReach, hold: Hold = { tell: 'bang', after: 2 }, word: Hold = { tell: 'word', after: 2 };
  const cans = (r: Run) => r.beats.filter(b => /^toss:/.test(b.what)).length, has = (r: Run, re: RegExp) => idx(r, re) >= 0;
  const STEP = 0.05, wait = K.thrHandWait, scn = 'records-pair', STEPIN = /^step in/;
  const tellF = (r: Run) => `${line(r)}: ${r.why ?? `ran ${r.fragRan.join(', ') || 'nobody'} · as 'step in' was said ${r.offAtStep?.join(', ') || 'nobody'} off the door · ${r.tossAt.map(x => `${x.who} threw from ${x.near.toFixed(2)} m`).join(', ') || 'no can'} · ${r.alive} of ${r.n} up · ended '${r.ended}' · ${story(r)}`}`;
  const RAN = /^grenade on the stack · /, WAIT = /^the door waits: /, BACK = /^the men are back/, SPENT = /^on without .* · the order's wait is spent$/, LEAD = process.env.THRFRAGHOLD_BREAK === '1' ? NaN : 1;   // (the grenade is put down LEAD s before Sam speaks: the men are running as he speaks)
  const f = await doorRun(scn, 1, null, 120, NaN, hold, false, false, 0, LEAD);
  verdict(!f.why && f.fragRan.length >= 1 && has(f, RAN) && has(f, WAIT) && has(f, BACK) && !has(f, SPENT) && !!f.offAtStep && f.offAtStep.length === 0 && tOf(f, /^toss:|^step in/) >= tOf(f, BACK) && cans(f) >= 1 && f.tossAt.every(x => x.near < reach + STEP) && tOf(f, /^toss:/) - tOf(f, WAIT) < wait + K.thrTurn + 0.5 && f.ended === 'in',
    `THE DOOR WAITS FOR ITS MEN: 'the door waits' is said, no can goes and nobody steps in until 'the men are back', a can goes within the order's one wait and every can is thrown from within ${reach} m of the frame, 'step in' is said with every living man at the door or on his mark, and the order ends 'in'`, tellF(f));
  const w = await doorRun(scn, 1, null, 120, NaN, word, false, false, 0, LEAD);
  verdict(!w.why && w.fragRan.length >= 1 && has(w, RAN) && has(w, WAIT) && has(w, BACK) && !has(w, SPENT) && tOf(w, STEPIN) >= tOf(w, BACK) && cans(w) === 0 && !!w.offAtStep && w.offAtStep.length === 0,
    "THE WORD WAITS FOR THE MEN WHO RAN: 'the door waits' is said, nobody steps in until 'the men are back', no can is spent, and 'step in' is said with every living man at the door or on his mark (how the entry with no can then ends is the room fight's, and printed)", `step in ${f1(tOf(w, STEPIN) - w.tellT)} s after the word · ${tellF(w)}`);
  return done();
});
