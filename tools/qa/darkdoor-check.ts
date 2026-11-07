// NOT IN THERE ALONE — the check behind src/game/darkdoor.ts (DARKDOOR.on: on in the page, ?nodarkdoor; off on the frozen stand, TPS_DARKDOOR=1 — and it acts
// only under MORALE.on). Standalone and through probes.ts:
//   bun run tools/qa/darkdoor-check.ts          bun run tools/qa/probes.ts darkdoor
// THE OFFICE at night, the player out of everybody's sight. The break room's lamps are out on the night shift (calmtorch-check's dark room); its west door
// stands open onto the lit cubicles. A man is stood six metres up the cubicles from that door, made suspicious of a spot three metres inside the break room
// (or put on an alert chase of a fix there), and walked at it by the floor's own rules; the other two of the shift are stood across the floor — held where
// they stand ('nobody free') or left free. THE COMPOUND's room clear for the squads. Headless, PASS / FAIL each, on sim state, every number the default:
//   (1) a green man, alone, nobody to call: he stops a stride short of the frame, off the middle of the doorway, not in the opening; the line — to himself,
//       spoken — is said as he stops; he never crosses the frame while he waits; his aim goes across the opening and back, more than once
//   (2) a mate in earshot: the line is the call, spoken; the mate answers, spoken, walks to him, says his line at his shoulder, and both cross the frame
//       within seconds of it, the mate's fix the first man's goal
//   (3) a mate across the floor: the call and the answer go on the net; he comes the same
//   (4) a steady man in the same place walks straight in: no line — and who is green does not move when the roster does
//   (5) the same room with its lamps burning: the green man walks straight in
//   (6) nobody to come: after `waitSecs` he says so once, is let go with his fix where he stands, and is back on his route inside half a minute
//   (7) …and sent at the same room again: he does not get a silent pass — he says he is going in, stands the beat, and goes in slower than he walked up
//   (10) every line of the file fits a bubble, names nothing off the blocklist, no compass word, none twice
//   (12) an alert chase of a fix inside: he balks the same, waits the wait out, gives it up to a search where he stands — and if that search takes him
//       through the door after all it is the slow entry, said first, never a walk straight in
// It raises what it needs itself (both switches, who is green), so it passes on the frozen stand and under TPS_PROFILE=shipped.
// DARKDOOR_BREAK=1 = no room is ever dark to the rule (the rule broken, the switch left alone — and left ON for the off line): every line with a balk in it
// must fail — (1) (2) (3) (6) (7) (12).   exit 0 = all hold · 1 = one does not
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
const ROOT = resolve(import.meta.dir, '../..');
const DT = 1 / 60, f2 = (x: number) => (Number.isFinite(x) ? x.toFixed(2) : String(x));
function seeded(a: number) { return () => { a |= 0; a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ (a >>> 15), 1 | a); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; }; }
/** the compound's level file as it ships */
const compoundText = (): string => readFileSync(resolve(ROOT, 'levels', 'compound.json'), 'utf8');
const quiet = async <T>(fn: () => Promise<T> | T): Promise<T> => { const log = console.log, err = console.error, warn = console.warn; console.log = console.error = console.warn = () => {}; try { return await fn(); } finally { console.log = log; console.error = err; console.warn = warn; } };

export async function darkdoorCheck(): Promise<boolean> {
  const { standUp } = (await import('./headless.ts')) as any;
  const DD = await import(`${ROOT}/src/game/darkdoor.ts`);
  const { DARKDOOR, DARKDOOR_STATS, darkDoorOf, darkDoorLines, darkDoorGreen, darkDoorSummary, darkRoom } = DD;
  const { MORALE } = await import(`${ROOT}/src/game/morale.ts`);
  const { toggleTarget } = await import(`${ROOT}/src/game/lighting.ts`);
  const { HOUR } = await import(`${ROOT}/src/game/hours.ts`);
  const { doorPt } = await import(`${ROOT}/src/game/doortools.ts`);
  const { frameOf, crossesDoor, resetChase, lightEstimateAt } = await import(`${ROOT}/src/game/guards.ts`);
  const { digestGame, hashDigest } = await import(`${ROOT}/src/game/digest.ts`);
  const { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`);
  const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
  const { startFromQuery } = await import(`${ROOT}/src/game/scenarioRun.ts`);
  const { squadsReport, squadMen, SQUADS } = await import(`${ROOT}/src/game/enemysquads.ts`);
  const D = await import(`${ROOT}/src/game/dialogue.ts`);
  const BREAK = process.env.DARKDOOR_BREAK === '1'; let all = true;
  const was = { dd: JSON.parse(JSON.stringify(DARKDOOR)), morale: MORALE.on, squads: SQUADS.on, rnd: Math.random };
  const { RUNNER } = await import(`${ROOT}/src/game/runner.ts`); const train0 = MORALE.training, nerve0 = MORALE.nerve, runner0 = RUNNER.on;
  /** WHO IS GREEN is morale.ts's (trainingOf). These cases were staged on a leader and green men: 'mixed' = the first man of a squad steady and the rest green; the night staff by temperament, an unnamed man green on the roster's odd places — whatever the difficulty the stand is on. The run rule held off (runner-check.ts is its own). After every stand-up, which re-reads the stand's words */
  const { NETQUIET: NQ_ } = await import(`${ROOT}/src/game/netquiet.ts`); const nq0_ = NQ_.on;   // (the net goes quiet, a later rule: held off here, after every stand-up — netquiet-check.ts holds it with the rest)
  const mixed = () => { MORALE.training = 'mixed'; MORALE.nerve = 'normal'; RUNNER.on = false; NQ_.on = false; };
  const line = (ok: boolean, label: string, detail: string) => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); };
  console.log(`\n=== darkdoor: a green man alone does not walk into a dark room${BREAK ? ' (DARKDOOR_BREAK: no room is dark to the rule)' : ''} ===`);
  const flat = (a: number[], b: number[]) => Math.hypot(a[0] - b[0], a[2] - b[2]);
  const put = (gd: any, p: number[], face?: number[]) => { gd.char.pos = [p[0], 0, p[2]]; gd.char.vel = [0, 0, 0]; gd.path = []; gd.pathGoal = null; gd.speed = 0; gd.lastPos = [p[0], 0, p[2]]; gd.stallRef = null;
    if (face) gd.char.bodyYaw = gd.char.aimYaw = Math.atan2(face[0] - p[0], face[2] - p[2]); };
  const openUp = (d: any, a = 1.4) => { d.reset(); d.angle = a; d.vel = 0; d.latched = false; d.closing = false; d.place(); };
  const shut = (d: any) => { d.reset(); d.angle = 0; d.vel = 0; d.latched = true; d.closing = false; d.place(); };
  /** a door's frame as the check measures it: `out` = the face the man starts on. perp: metres off the frame line on that face (− = through); along: where
   *  along the frame (0 ‥ W = the opening's span); aimAlong: where the line of a man's aim cuts the frame; through: his stride since the last call cut the
   *  frame inside the opening, from that face */
  const frame = (d: any, out: 1 | -1) => { const F = frameOf(d), prev = new Map<any, number[]>();
    const perp = (p: number[]) => ((p[0] - F.hx) * F.nx + (p[2] - F.hz) * F.nz) * out, along = (p: number[]) => (p[0] - F.hx) * F.ux + (p[2] - F.hz) * F.uz;
    const aimAlong = (gd: any) => { const p = gd.char.pos, a = gd.char.aimYaw, far: number[] = [p[0] + Math.sin(a) * 12, 0, p[2] + Math.cos(a) * 12];
      const da = (p[0] - F.hx) * F.nx + (p[2] - F.hz) * F.nz, db = (far[0] - F.hx) * F.nx + (far[2] - F.hz) * F.nz; if (da * db >= 0) return NaN; const t = da / (da - db);
      return along([p[0] + (far[0] - p[0]) * t, 0, p[2] + (far[2] - p[2]) * t]); };
    const through = (gd: any) => { const p0 = prev.get(gd) ?? [...gd.char.pos], p = [...gd.char.pos]; prev.set(gd, p); const c = flat(p0, p) > 1e-5 && flat(p0, p) < 1 ? crossesDoor(p0, p, d) : null; return !!c && c.side === out; };
    return { F, perp, along, aimAlong, through }; };
  /** suspicious of a spot, as a noise there leaves a man: the floor's own walk to a fix */
  const sendTo = (T: any, goal: number[]) => { T.state = 'suspicious'; T.awareness = 0.5; T.reactT = 0; T.searchT = 0; T.lastKnown = [goal[0], 0, goal[2]]; T.path = []; T.pathGoal = null; T.stallRef = null; };

  /** the office at night; `man` = the roster index of the man walked at the break room; `others`: 'held' = the other two stand where they are put, on a
   *  scripted hold (nobody free to come) · 'near' = one of them free, seven metres behind him · 'far' = one free across the floor · 'both' = both free;
   *  `lit` = the break room's lamps put on; `shut` = the door shut and latched; `on` / `morale` = the switches */
  const stage = async (seed: number, o: { man?: number; others?: 'held' | 'near' | 'far' | 'both'; lit?: boolean; shut?: boolean; on?: boolean; morale?: boolean; who?: string; walk?: boolean } = {}) => {
    Math.random = seeded(seed); HOUR.id = 'night'; HOUR.sun = 1; Object.assign(DARKDOOR, was.dd); DARKDOOR.on = false; MORALE.on = false;
    const W: any = await quiet(() => standUp()); const g = W.game; g.playerInvisible = true; g.godMode = true;
    MORALE.on = o.morale ?? true; mixed(); DARKDOOR.on = o.on ?? true; DARKDOOR.who = o.who ?? 'rest'; if (BREAK) DARKDOOR.darkShare = 2;
    const step = (n = 1) => { for (let i = 0; i < n; i++) W.step(DT); };
    step(10);
    const d = g.doors.list.find((x: any) => x.def.name === 'breakroom_w'), mid = doorPt(d, d.def.width / 2, 0, 1, 0);
    const out: 1 | -1 = g.cleared.roomBy(d, 1) === g.cleared.roomAt([28, 0, 21]) ? -1 : 1, room = g.cleared.roomBy(d, -out);   // (`out` = the cubicles' face of the frame: the break room is on the other)
    const lamps = () => { for (const t of g.targets) if (!t.on && g.cleared.roomAt(t.pos) === room) toggleTarget(g, t); };
    if (o.lit) { lamps(); step(30); }
    if (o.shut) shut(d);
    const T = g.guards[o.man ?? 1], rest = g.guards.filter((x: any) => x !== T);
    const start = doorPt(d, d.def.width / 2, 6, out, 0), goal = doorPt(d, d.def.width / 2, 3, -out as 1 | -1, 0);
    const far = [[7, 0, 11.2], [12, 0, 11.2]];   // (the corridor's west end: two rooms and a wall from the break room's west door)
    rest.forEach((x: any, i: number) => { put(x, far[i]); x.hold = true; });
    const mate = rest[1], third = rest[0];
    if (o.others === 'near' || o.others === 'both') { mate.hold = false; put(mate, doorPt(d, d.def.width / 2, 13, out, 0)); }
    if (o.others === 'far') mate.hold = false;
    if (o.others === 'both') third.hold = false;
    const send = () => sendTo(T, goal);
    /** an alert chase of the fix inside, his last sighting six seconds old */
    const chase = () => { T.state = 'alert'; T.awareness = 1; T.reactT = 0; T.fireCd = 0.3; T.drawn = true; T.lastKnown = [goal[0], 0, goal[2]]; T.lastSeenT = g.time - 6; T.searchPlan = null; T.path = []; T.pathGoal = null; resetChase(T); };
    put(T, start, mid); if (o.walk !== false) send();
    return { W, g, d, T, rest, mate, third, room, out, start, goal, step, send, chase, lamps, ...frame(d, out) };
  };
  type Rig = Awaited<ReturnType<typeof stage>>;
  type Said = { who: any; text: string; radio: boolean; t: number };
  /** step `secs`, noting every new bubble of `who`; `each` true = stop */
  const listen = (R: { step: (n?: number) => void }, who: any[], secs: number, each?: (t: number) => boolean | void) => {
    const said: Said[] = [], last = new Map<any, any>(who.map(h => [h, h.bubble?.t]));   // (the copy reuses a man's bubble: a new line is a new time on it)
    for (let i = 1; i <= Math.round(secs * 60); i++) { R.step(); for (const h of who) { const b = h.bubble; if (b && last.get(h) !== b.t) { last.set(h, b.t); said.push({ who: h, text: b.text, radio: !!b.radio, t: i * DT }); } } if (each?.(i * DT)) break; }
    return said;
  };
  const of = <S extends { text: string }>(said: S[], kind: string): S[] => said.filter(s => darkDoorLines(kind).includes(s.text));
  const mine = <S extends { text: string }>(said: S[]): S[] => said.filter(s => darkDoorLines().includes(s.text));
  const kinds = (said: { text: string }[]) => mine(said).map(s => (['call', 'self', 'copy', 'with', 'nobody', 'alone'] as const).find(k => darkDoorLines(k).includes(s.text)) ?? '?').join(' ');

  try {
    // (1) green, alone, nobody to call
    { const R = await stage(21, { others: 'held' }), T = R.T, K = DARKDOOR; let crossed = false, atLine: number[] | null = null, stopAt = -1, passes = 0, lastEnd = 0, minP = Infinity, n = 0;
      const dark = darkRoom(R.g, R.room), green = darkDoorGreen(R.g, T);
      const said = listen(R, [T], 14, t => { const p = T.char.pos, pp = R.perp(p); if (R.through(T)) crossed = true; minP = Math.min(minP, pp);
        const S = darkDoorOf(T); if (S?.act === 'balk' && !atLine) { atLine = [...p]; stopAt = t; }
        if (S?.act === 'balk' && S.at) { n++; const aa = R.aimAlong(T), end = aa < R.F.W * 0.25 ? -1 : aa > R.F.W * 0.75 ? 1 : 0; if (end && end !== lastEnd) { if (lastEnd) passes++; lastEnd = end; } } });
      const self = of(said, 'self')[0], S = darkDoorOf(T), p = T.char.pos, off = Math.abs(R.along(p) - R.F.W / 2);
      line(green && dark.dark && !!self && !self.radio && mine(said).length === 1 && Math.abs(self.t - stopAt) < 0.05 && !!atLine && R.perp(atLine) <= K.stopM + 0.3 && !crossed && S?.act === 'balk' && n > 300 && minP >= 0.9 && R.perp(p) >= 0.9 && R.perp(p) <= K.stopM && off > 0.15 && passes >= 3,
        '(1) a green man alone, nobody to call: stopped a stride short of the frame and off its middle, a word to himself as he stops, never through, his aim across the opening and back',
        `green ${green} · the room ${f2(dark.share)} dark (${dark.dark ? 'DARK' : 'lit'}) · '${self?.text ?? 'no line'}'${self ? ` ${self.radio ? 'on the net' : 'spoken'} at ${f2(self.t)} s, ${f2(atLine ? R.perp(atLine) : NaN)} m short of the frame` : ''} · after 14 s: ${S?.act ?? 'not met'}, ${f2(R.perp(p))} m off the frame, ${f2(off)} m off its middle · nearest he came ${f2(minP)} m · passes of his aim across it ${passes} · ${darkDoorSummary(R.g)}`); }
    // (2) a mate in earshot · (3) a mate across the floor
    for (const [n, where, seed] of [[2, 'near', 22], [3, 'far', 30]] as const) {
      const R = await stage(seed, { others: where }), T = R.T, m = R.mate; let tT = -1, tM = -1, goalSame = false, withAt = -1, d0 = NaN;
      const said = listen(R, [T, m], 60, t => { if (withAt < 0 && darkDoorOf(m)?.act === 'with') withAt = t; if (Number.isNaN(d0) && darkDoorOf(T)?.called) d0 = flat(m.char.pos, T.char.pos);   // (how far off his man is as he calls)
        if (withAt >= 0 && !goalSame && m.lastKnown && flat(m.lastKnown, R.goal) < 0.6) goalSame = true;
        if (R.through(T) && tT < 0) tT = t; if (R.through(m) && tM < 0) tM = t; return tT >= 0 && tM >= 0; });
      const call = of(said, 'call')[0], copy = of(said, 'copy')[0], w = of(said, 'with')[0], netted = d0 > DARKDOOR.earM;
      line(kinds(said) === 'call copy with' && call.who === T && call.radio === netted && (n === 2 ? !netted : netted) && copy.who === m && copy.radio === netted && copy.t > call.t && w.who === m && !w.radio && tT > w.t && tM > w.t && tT - w.t < 8 && tM - w.t < 10 && goalSame,
        n === 2 ? '(2) a mate in earshot: the call spoken, the answer spoken, he walks to him, says his line at his shoulder, and both go through within seconds — bound for the first man\'s goal'
          : '(3) a mate across the floor: the call and the answer on the net, and he comes the same',
        `lines: ${kinds(said) || 'none'} · '${call?.text ?? '—'}' ${call?.radio ? 'on the net' : 'spoken'} at ${f2(call?.t ?? NaN)} s, ${m.callsign} ${f2(d0)} m off · '${copy?.text ?? '—'}' ${copy?.radio ? 'on the net' : 'spoken'} at ${f2(copy?.t ?? NaN)} s · '${w?.text ?? '—'}' at ${f2(w?.t ?? NaN)} s · through the frame: ${T.callsign} ${f2(tT)} s, ${m.callsign} ${f2(tM)} s · the mate's fix on the first man's goal ${goalSame}`); }
    // (4) a steady man, and the deal is kept · (5) the room lit
    { const R = await stage(23, { man: 0, others: 'held' }), T = R.T, g = R.g, b0 = DARKDOOR_STATS.balks; let tT = -1; const dark = darkRoom(g, R.room);
      const deal = () => g.guards.map((x: any) => darkDoorGreen(g, x) ? 'g' : 's').join(''), d0 = deal();
      const said = listen(R, [T], 12, t => { if (tT < 0 && R.through(T)) tT = t; return tT >= 0; });
      const gone = g.guards.shift(), d1 = deal(); g.guards.unshift(gone);   // (the roster a man short: the two left are who they were)
      line(d0 === 'sgs' && d1 === 'gs' && (dark.dark || BREAK) && tT >= 0 && tT < 8 && !mine(said).length && DARKDOOR_STATS.balks === b0,
        '(4) a steady man in the same place walks straight in, no line — and who is green is dealt once: the roster a man short, the others are who they were',
        `${T.callsign}: dealt ${d0} (s steady, g green), with the first man off the roster ${d1} · the room ${f2(dark.share)} dark · through the frame at ${f2(tT)} s · lines of this file ${mine(said).length} · balks ${DARKDOOR_STATS.balks - b0}`); }
    { const R = await stage(24, { others: 'held', lit: true }), T = R.T, b0 = DARKDOOR_STATS.balks; let tT = -1; const dark = darkRoom(R.g, R.room), green = darkDoorGreen(R.g, T);
      const said = listen(R, [T], 12, t => { if (tT < 0 && R.through(T)) tT = t; return tT >= 0; });
      line(green && !dark.dark && tT >= 0 && tT < 8 && !mine(said).length && DARKDOOR_STATS.balks === b0, '(5) the same room with its lamps burning: the green man walks straight in',
        `${T.callsign}: green ${green} · the room ${f2(dark.share)} dark (${dark.dark ? 'DARK' : 'lit'}) · through the frame at ${f2(tT)} s · lines of this file ${mine(said).length}`); }
    // (6) nobody to come · (7) sent at the room again
    { const R = await stage(25, { others: 'held' }), T = R.T, K = DARKDOOR; let crossed = false, freeAt = -1, calmAt = -1, fixAt = NaN, met = false;
      const said = listen(R, [T], K.waitSecs + 34, t => { if (R.through(T)) crossed = true;
        if (darkDoorOf(T)?.act === 'balk') met = true; if (met && freeAt < 0 && darkDoorOf(T)?.act === 'none') { freeAt = t; } if (freeAt >= 0 && t - freeAt <= 2 * DT + 1e-9 && T.lastKnown) fixAt = flat(T.lastKnown, T.char.pos);
        if (freeAt >= 0 && calmAt < 0 && T.state === 'patrol') calmAt = t; return calmAt >= 0; });
      const nb = of(said, 'nobody'), self = of(said, 'self')[0];
      line(kinds(said) === 'self nobody' && Math.abs(nb[0].t - self.t - K.waitSecs) < 0.6 && !crossed && Math.abs(freeAt - nb[0].t) < 0.1 && fixAt < 0.6 && calmAt >= 0 && calmAt - freeAt < 30,
        '(6) nobody to come: he waits the wait out on his own face of the frame, says so once, is let go with his fix where he stands, and is back on his route inside half a minute',
        `lines: ${kinds(said) || 'none'} · '${nb[0]?.text ?? 'no line'}' at ${f2(nb[0]?.t ?? NaN)} s (stopped at ${f2(self?.t ?? NaN)} s; the wait is ${K.waitSecs} s) · through the frame ${crossed} · let go at ${f2(freeAt)} s, his fix ${f2(fixAt)} m from his feet · on patrol again at ${f2(calmAt)} s`);
      put(T, R.start, R.d.frameCentre); R.send(); let tT = -1, vIn = 0, vUp = 0, soloAt = -1, movedAt = -1; const p0: number[] = [];
      const again = listen(R, [T], 16, t => { const v = Math.hypot(T.char.vel[0], T.char.vel[2]), pp = R.perp(T.char.pos), S = darkDoorOf(T); if (pp > 3.5) vUp = Math.max(vUp, v);
        if (S?.act === 'solo') { if (soloAt < 0) { soloAt = t; p0.push(...T.char.pos); } else if (movedAt < 0 && flat(T.char.pos, p0) > 0.3) movedAt = t; if (pp < 0.8) vIn = Math.max(vIn, v); }
        if (R.through(T)) { tT = t; return true; } });
      const al = of(again, 'alone')[0];
      line(kinds(again) === 'alone' && !al.radio && Math.abs(al.t - soloAt) < 0.05 && movedAt - soloAt >= K.nerveSecs - 0.3 && tT > al.t && vIn > 0.2 && vIn < vUp * (K.slowK + 0.15),
        `(7) sent at the same room again inside ${K.fedUpSecs} s (the default numbers): no silent pass — he says it, stands the beat, and goes in alone slower than he walked up`,
        `lines: ${kinds(again) || 'none'} · '${al?.text ?? 'no line'}' at ${f2(al?.t ?? NaN)} s · on the move again ${f2(movedAt - soloAt)} s later (the beat is ${K.nerveSecs} s) · through the frame at ${f2(tT)} s · fastest in the doorway ${f2(vIn)} m/s against ${f2(vUp)} m/s walking up (the bar is ${f2(K.slowK)} of it) · ${DARKDOOR_STATS.quits} given up, ${DARKDOOR_STATS.solos} nerved`); }
    // (10) the words
    { const lines: string[] = darkDoorLines(), long = lines.filter(t => t.length > D.AIM_CHARS), blocked = lines.filter(t => D.blockedPhrase(t)), compass = lines.filter(t => /\b(north|south|east|west)/i.test(t)), twice = lines.filter((t, i) => lines.indexOf(t) !== i), tight = lines.filter(t => /\S—\s|\s—\S/.test(t));
      line(lines.length >= 18 && !long.length && !blocked.length && !compass.length && !twice.length && !tight.length && (['call', 'self', 'copy', 'with', 'nobody', 'alone'] as const).every(k => darkDoorLines(k).length >= 3),
        `(10) every line fits a bubble (≤ ${D.AIM_CHARS}), none on the originality blocklist, no compass word, none twice`, `${lines.length} lines · too long: ${long.join(' | ') || 'none'} · blocked: ${blocked.join(' | ') || 'none'} · twice: ${twice.join(' | ') || 'none'}`); }
    // (12) an alert chase: the balk, the give-up, and what the search does next
    { const R = await stage(31, { others: 'held' }), T = R.T, K = DARKDOOR; R.chase(); let crossedWaiting = false, quitAt = -1, stateAt = '', fixAt = NaN, tT = -1, soloBefore = false, vIn = 0;
      const said = listen(R, [T], K.waitSecs + 50, t => { const S = darkDoorOf(T), thr = R.through(T);
        if (quitAt < 0) { if (thr) crossedWaiting = true; if (DARKDOOR_STATS.quits > 0 && S?.act === 'none' && of([{ text: T.bubble?.text ?? '' }], 'nobody').length) { quitAt = t; stateAt = T.state; fixAt = T.lastKnown ? flat(T.lastKnown, T.char.pos) : NaN; } }
        else { if (S?.act === 'solo') { soloBefore = true; if (R.perp(T.char.pos) < 0.8) vIn = Math.max(vIn, Math.hypot(T.char.vel[0], T.char.vel[2])); } if (thr && tT < 0) { tT = t; return true; } } });
      const first = mine(said)[0], nb = of(said, 'nobody')[0], al = of(said, 'alone')[0], after = tT < 0 ? 'he did not go through that door again' : `through at ${f2(tT)} s, nerved first ${soloBefore}, '${al?.text ?? 'no line'}', ${f2(vIn)} m/s in the doorway`;
      line(!!first && (first === of(said, 'self')[0]) && !!nb && !crossedWaiting && stateAt === 'search' && fixAt < 0.6 && (tT < 0 || (soloBefore && !!al && al.t < tT && vIn < 2)),
        '(12) an alert chase of a fix inside: he balks the same, waits it out, and the chase is given up to a search where he stands — which goes through that door slowly and said first, or not at all',
        `lines: ${kinds(said) || 'none'} · stopped at ${f2(first?.t ?? NaN)} s · '${nb?.text ?? 'no line'}' at ${f2(nb?.t ?? NaN)} s · through the frame before it ${crossedWaiting} · then '${stateAt}', his fix ${f2(fixAt)} m from his feet · after: ${after}`); }
  } catch (e) { all = false; console.log(`- FAIL — the check threw\n    ${e instanceof Error ? e.stack : String(e)}`); }
  finally { Object.assign(DARKDOOR, was.dd); MORALE.on = was.morale; MORALE.training = train0; MORALE.nerve = nerve0; RUNNER.on = runner0; NQ_.on = nq0_; SQUADS.on = was.squads; Math.random = was.rnd; HOUR.id = 'night'; }
  console.log(`=== darkdoor: ${all ? 'ALL PASS' : 'FAILURES above'} ===`);
  return all;
}

if (import.meta.main) process.exit((await darkdoorCheck()) ? 0 : 1);
