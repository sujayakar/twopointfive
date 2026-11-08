// A WALK TRIES THE DOOR (src/game/orders.ts ORDERS.walkTries, walkTry / walkStacks; src/game/doortools.ts walkTried) — a 'move there' whose only way in
// runs at a door locked against the team that nobody has tried is not refused with the door's name any more: the men walk up to it, the man on the jamb
// tries the handle, and the order ends there the way a door order ends 'found shut' — the same words, said once, the men left stacked on the door. Only
// then does the team know, and the same order is refused as it always was. It acts where DOORTOOLS.testDoor is on; the probe sets that key itself.
//   bun run tools/qa/probes.ts move-walktries       (family 'walktries'; TPS_PROFILE=shipped for the page's stand, TPS_WALKTRIES=1 raises the key on the frozen one)
// Headless, staged the same on either stand; nothing here looks at a picture or hears a voice (the lines are read off Game.say, the menu off Orders.hud()).
//   walk     key on, the same: not refused, not greyed; the men walk (never faster than a plain 'move' walks them); one man tries the leaf; the door
//            orders' own words are said once and logged; every man of the order is left stacked on that door, on his mark three seconds on, on his own
//            side of it, nobody inside the room, nobody moving after; the team knows, and the same order is now refused and greyed
import { standUp, ROOT } from './headless.ts';
const { NIGHTBREACH } = await import(`${ROOT}/src/game/nightbreach.ts`);
const { ROOMCLEAR } = await import(`${ROOT}/src/game/roomclear.ts`);
const { DOORTOOLS } = await import(`${ROOT}/src/game/doortools.ts`);
const { ORDERS, sideOf, doorWords } = await import(`${ROOT}/src/game/orders.ts`);

const DT = 1 / 60;
function seeded(a: number) { return () => { a |= 0; a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ (a >>> 15), 1 | a); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; }; }
const f2 = (x: number) => (Number.isFinite(x) ? x.toFixed(2) : String(x));
const at = (p: number[]) => `(${p[0].toFixed(2)}, ${p[2].toFixed(2)})`;
const dist = (a: number[], b: number[]) => Math.hypot(a[0] - b[0], a[2] - b[2]);
const quiet = async <T>(fn: () => T | Promise<T>): Promise<T> => { const l = console.log, e = console.error, w = console.warn; console.log = console.error = console.warn = () => { }; try { return await fn(); } finally { console.log = l; console.error = e; console.warn = w; } };
const DT_DEFAULTS = JSON.parse(JSON.stringify(DOORTOOLS)), RC_DEFAULTS = { ...ROOMCLEAR }, NB_DEFAULTS = { ...NIGHTBREACH }, ORD_DEFAULTS = { ...ORDERS, followBack: [...ORDERS.followBack] };
const back = () => { Object.assign(DOORTOOLS, JSON.parse(JSON.stringify(DT_DEFAULTS))); Object.assign(ROOMCLEAR, RC_DEFAULTS); Object.assign(NIGHTBREACH, NB_DEFAULTS); Object.assign(ORDERS, ORD_DEFAULTS, { followBack: [...ORD_DEFAULTS.followBack] }); };

// the office (the room clear's level). Sam stands down the corridor, off every lock's rows but the server door's: at a door's rows he would KNOW it
// (doortools.ts testStacks). INSIDE: floor in the manager's room, a stride and a half in from its corridor door. CONF: floor in the conference room,
// whose door stands open
const AWAY: number[] = [19.5, 0, 11.2], INSIDE: number[] = [25.75, 0, 8.5], CONF: number[] = [9.8, 0, 8.0], STORE: number[] = [32.75, 0, 8.0];
const NOT_LOCKED = /: it's not locked\.$/, NO_ROOM = 'no room on that door.';
const SHUT_LINE = /holding on it — your call\.$/;

interface Man { c: string; order: string; door: string; there: string; post: number[] | null; pos: number[]; side: number; /** he still carries the walk's `tries` */ tries: boolean; arrived: boolean; dead: boolean }
interface Row { off: boolean; text: string }
interface Run { row0: Row | null; why: string | null; said: string[]; menAt0: Man[]; halt: Man[]; later: Man[]; row1: Row | null; why2: string | null; end: Man[]; secs: number; top: number; walked: number[]; log: string[]; halted: number; tested: number;
  knew0: boolean; knew1: boolean; /** frames a walker still to try the door read 'in position' on the HUD */ lied: number; shutStill: boolean; frame: number[]; samSide: number; name: string; who: string[] }

/** one game: the room clear staged quiet (its defenders dazzled every frame: this is about feet), Sam down the corridor, the two keys set AFTER the stand-up
 *  (the frozen stand pins both off, the page's raises both — here each run is told) */
/** STEPPED UNTIL THE MEN HAVE SETTLED: `min` frames at the least (three seconds, the flat wait these stagings had), then on until nobody has moved for a
 *  whole second, `cap` frames at the most. Frames stepped: at `cap` they never settled. `how`: WEDGESETTLE_BREAK's two red paths. */
function settle(step: () => void, pos: () => number[][], how = '', min = 180, cap = 600): number {
  if (how === '1') return 0;
  let n = 0; for (let p = pos(), still = 0; n < cap && (n < min || (how !== 'flat' && still < 60)); n++) { step(); const q = pos(); still = Math.max(...p.map((x, i) => dist(x, q[i]))) < 1e-4 ? still + 1 : 0; p = q; }
  return n;
}
async function rig(key: boolean, squad: 2 | 4, testDoor = true) {
  back(); Object.assign(NIGHTBREACH, { squad }); Math.random = seeded(5);
  const W: any = await quiet(() => standUp()); const g = W.game, O = g.orders, D = g.doorTools, NB = g.nightBreach;
  g.godMode = true; await quiet(() => g.roomClear.start(5));
  DOORTOOLS.testDoor = testDoor; ORDERS.walkTries = key;
  const said: string[] = [], say = g.say.bind(g);
  g.say = (gd: any, text: string, ...rest: any[]) => { said.push(`${gd.callsign}: ${text}`); return say(gd, text, ...rest); };
  const step = (n = 1) => { for (let i = 0; i < n; i++) { g.player.visibility = 0.05; for (const h of NB.hostiles) if (h.state !== 'dead') h.dazzledUntil = g.time + 5; W.step(DT); } };
  // (the page's squad opens following Sam, the frozen stand's with no order: told to hold where it stands, the same on both, before anything is read)
  await quiet(() => { step(30); g.teleportPlayer(AWAY); step(5); O.give('hold', O.targetAt(g.player.char.pos), 'all'); step(60); });
  return { g, O, D, said, step, door: (name: string): any => g.doors.list.find((d: any) => d.def.name === name) };
}
type Rig = Awaited<ReturnType<typeof rig>>;
/** what a staging does part-way through the walk: `[seconds after the order, what]` */
type Mid = [number, (R: Rig, d: any, sent: any[]) => void];
interface Opts { squad?: 2 | 4; who?: any; mids?: Mid[]; testDoor?: boolean; /** done once the game stands, before anything is asked */ before?: (R: Rig, d: any) => void
}

/** `mode`: 'locked' the manager's room, nobody has tried its door · 'known' the same after a plain 'stack up' has tried it · 'ring' the same with the
 *  door's key on Sam's ring · 'open' the conference room, its door standing open. 'move there' is given to `who`, read, played out, and given once more. */
async function play(mode: 'locked' | 'known' | 'ring' | 'open', key: boolean, o: Opts = {}): Promise<Run> {
  const who = o.who ?? 'all', mids = [...(o.mids ?? [])], R = await rig(key, o.squad ?? 2, o.testDoor ?? true), { g, O, D, said } = R, d = R.door('manager'), samSide = sideOf(d, g.player.char.pos), spot = mode === 'open' ? CONF : INSIDE;
  const team: any[] = O.men('all'), sent: any[] = O.men(who);
  const men = (l: any[] = team): Man[] => l.map((m: any) => { const A = O.of(m); return { c: m.callsign, order: String(A?.order ?? '-'), door: String(A?.door?.def.name ?? '-'), there: String(A?.there ?? '-'), post: A?.post ? [A.post[0], 0, A.post[2]] : null, pos: [m.char.pos[0], 0, m.char.pos[2]], side: sideOf(d, m.char.pos), tries: !!A?.tries, arrived: !!A?.arrived, dead: m.state === 'dead' }; });
  /** the menu opened on that spot as the player opens it: its 'move there' row — greyed or not, and its words */
  const row = (): Row | null => { const was = O.who; O.who = who; try { if (!O.open(O.targetAt(spot))) return null; const r = O.hud().menu?.rows.find((x: any) => String(x.opts[0]?.text ?? '').startsWith('move there')); return r ? { off: !!r.off, text: String(r.opts[0].text) } : null; } finally { O.close(); O.who = was; } };
  if (mode === 'known') await quiet(() => { O.give('stack', O.targetAt(d.frameCentre), who); for (let i = 0; i < 60 * 30 && !D.knownShut(d, samSide); i++) R.step(); R.step(60); });
  if (mode === 'ring') g.player.keys.add(d.lockId);
  o.before?.(R, d);
  const knew0 = D.knownShut(d, samSide), n0 = said.length, log0 = D.log.length, halted0 = D.stats.halted, tested0 = D.stats.tested, menAt0 = men();
  const row0 = row(), why = await quiet(() => O.give('move', O.targetAt(spot), who));
  // played until nobody has been on a 'move' short of his mark for half a second (30 s at most; `mids` done on their second): how long, the fastest any man went, how far each walked
  let secs = 0, top = 0, lied = 0; const walked = team.map(() => 0), last = team.map((m: any) => [...m.char.pos]);
  const going = () => mids.length > 0 || team.some((m: any) => { const A = O.of(m); return m.state !== 'dead' && A?.order === 'move' && !A.arrived; });
  await quiet(() => { for (let i = 0, still = 0; i < 60 * 30 && still < 30; i++) { while (mids.length && secs >= mids[0][0]) mids.shift()![1](R, d, sent); R.step(); secs += DT; still = going() ? 0 : still + 1;
    for (const m of team) if (O.of(m)?.tries && O.statusOf(m) !== 'moving') lied++;
    team.forEach((m: any, k: number) => { const s = dist(m.char.pos, last[k]); walked[k] += s; if (sent.includes(m)) top = Math.max(top, s / DT); last[k] = [...m.char.pos]; }); } });
  const halt = men(); await quiet(() => { settle(() => R.step(), () => team.map((m: any) => [...m.char.pos])); }); const later = men(), heard = said.slice(n0);   // (settled, as the wedge's case reads its four: three seconds at the least)
  const knew1 = D.knownShut(d, samSide), row1 = row(), why2 = await quiet(() => O.give('move', O.targetAt(spot), who)); await quiet(() => R.step(120));
  return { row0, why: why ?? null, said: heard, menAt0, halt, later, row1, why2: why2 ?? null, end: men(), secs, top, walked, log: D.log.slice(log0).map((l: any) => String(l.what)), halted: D.stats.halted - halted0, tested: D.stats.tested - tested0,
    knew0, knew1, lied, shutStill: !!d.locked && !!d.latched, frame: [d.frameCentre[0], 0, d.frameCentre[2]], samSide, name: doorWords(d), who: sent.map((m: any) => m.callsign) };
}

/** TWO LOCKS ON THE WAY: the pair walked into the storage room (its door unlocked for that), the door then shut and locked by hand against the room's
 *  face with a lock of its own; 'move there' to the manager's room; a quarter of a second into the walk (the jamb is a stride off) that lock's key is put on Sam's ring. The storage door then
 *  reads free at the handle — and the manager's door, still locked and never tried, is the next one on the way. */
async function second(key: boolean, begun: boolean) {
  const R = await rig(key, 2), { g, O, D, said } = R, st = R.door('storage'), mg = R.door('manager'), team: any[] = O.men('all'), corridor = sideOf(mg, g.player.char.pos);
  // (`begun`: THE FIGHT IS BEGUN, by the rule's own stamp for Sam's 'go' (holdfire.ts holdFireDrill, which does nothing where the rule is off), kept
  // standing through both walks. A defender stands in the storage room the pair is walked into, blind for good by the rig. Where a squad nobody has found
  // holds its round on a man who has not noticed it (HOLDFIRE.on, the page's stand) the man with eyes on him says 'eyes on. holding.' every eight seconds,
  // both men come onto the door's marks and neither is ever counted there: orders.ts counts a man arrived with his gun on the door's point, and the
  // stamp that counts a man holding his round as there is the STACK order's alone. So the walk's try is never made, forty seconds and counting. With the
  // fight begun he shoots, as the old stand's man always did, and the case plays the same hot floor on both stands. Not begun is the 'held' line's.)
  const { holdFireDrill } = await import(`${ROOT}/src/game/holdfire.ts`); const hot = () => { if (begun) holdFireDrill(g); };
  hot();
  st.locked = false;
  await quiet(() => { O.give('move', O.targetAt(STORE), 'all'); for (let i = 0; i < 60 * 30 && !team.every((m: any) => O.of(m)?.arrived); i++) { if (i % 60 === 0) hot(); R.step(); } R.step(60);
    st.angle = 0; st.vel = 0; st.closing = false; st.latched = true; st.locked = true; st.keySide = g.doors.side(st, team[0].char.pos); st.lockId = 'probe'; R.step(30); });
  const inStore = team.every((m: any) => sideOf(st, m.char.pos) === sideOf(st, STORE)), n0 = said.length, log0 = D.log.length, tested0 = D.stats.tested;
  hot(); const why = await quiet(() => O.give('move', O.targetAt(INSIDE), 'all')); let keyed = false, inMgr = 0;
  const going = () => !keyed || team.some((m: any) => { const A = O.of(m); return A?.order === 'move' && !A.arrived; });
  await quiet(() => { for (let i = 0, still = 0; i < 60 * 40 && still < 30; i++) { if (i % 60 === 0) hot(); if (!keyed && i >= 15) { g.player.keys.add('probe'); keyed = true; } R.step(); still = going() ? 0 : still + 1;
    for (const m of team) if (m.char.pos[2] < mg.frameCentre[2] - 0.3 && Math.abs(m.char.pos[0] - mg.frameCentre[0]) < 3.2) inMgr++; } R.step(180); });
  const men = team.map((m: any) => { const A = O.of(m); return { c: m.callsign, order: String(A?.order ?? '-'), door: String(A?.door?.def.name ?? '-'), there: String(A?.there ?? '-'), on: !!A?.post && dist(m.char.pos, A.post) < 0.6, side: sideOf(mg, m.char.pos), pos: [...m.char.pos] }; });
  return { inStore, why: why ?? null, said: said.slice(n0), log: D.log.slice(log0).map((l: any) => String(l.what)), tested: D.stats.tested - tested0, men, corridor, inMgr, mgShut: !!mg.locked && !!mg.latched, name: doorWords(mg) };
}

/** the Mousehole with the four and nobody to fight, its store door wedged against the corridor's face; 'move there' to all four for a spot inside the store */
async function wedge(key: boolean) {
  const BT: any = await import('./bang-tally.ts'), { killGuard } = await import(`${ROOT}/src/game/combat.ts`);
  const S: any = await quiet(() => BT.stage('mousehole-four', 1)); const g = S.g, O = g.orders, D = g.doorTools;
  for (const x of g.guards) if (x.state !== 'dead' && x.team === 'hostile') await quiet(() => killGuard(g, x, [0, 0, 1], true));
  DOORTOOLS.testDoor = true; ORDERS.walkTries = key;
  const d = g.doors.list.find((x: any) => x.def.name === 'store'), c = d.frameCentre, th = d.def.closedDir, spot = [c[0] - Math.sin(th) * -1.5, 0, c[2] + Math.cos(th) * -1.5];   // (a stride and a half in on the leaf's −1 face: the store)
  d.locked = true; d.keySide = 1; d.lockId = `wedge:${d.def.name}`;
  const said: string[] = [], say = g.say.bind(g); g.say = (gd: any, text: string, ...rest: any[]) => { said.push(`${gd.callsign}: ${text}`); return say(gd, text, ...rest); };
  const team: any[] = O.men('all'), def = O.defs.find((x: any) => x.id === 'move'), pos = () => team.map((m: any) => [m.char.pos[0], 0, m.char.pos[2]]);
  const row0 = O.whyNot(def, O.targetAt(spot), team), at0 = pos(), why = await quiet(() => O.give('move', O.targetAt(spot), 'all'));
  // (played until the line is said — 30 s at most — and then until the four have SETTLED, `settle` above. The file's last man is still coming onto his
  // mark as the first tries the handle: at the squad's own pace, which the page's stand has, he is 9 cm from where he will stand three seconds on, and the
  // 'nobody moving after' below read his last step, 5 cm of it.)
  let frames = 0;
  await quiet(() => { for (let i = 0; i < 60 * 30 && !said.some(x => SHUT_LINE.test(x)); i++) S.step(1); frames = settle(() => S.step(1), pos, process.env.WEDGESETTLE_BREAK ?? ''); });
  const men = team.map((m: any) => { const A = O.of(m); return { c: m.callsign, order: String(A?.order ?? '-'), door: String(A?.door?.def.name ?? '-'), there: String(A?.there ?? '-'), on: !!A?.post && dist(m.char.pos, A.post) < 0.6, side: sideOf(d, m.char.pos) }; }), at1 = pos();
  await quiet(() => S.step(120)); const at2 = pos(), row1 = O.whyNot(def, O.targetAt(spot), team);
  return { row0, why: why ?? null, said, men, row1, settle: frames / 60, name: doorWords(d), moved: Math.max(...at0.map((p, i) => dist(p, at1[i]))), after: Math.max(...at1.map((p, i) => dist(p, at2[i]))), knows: D.knownShut(d, 1) };
}

const gap = (a: Man[], b: Man[]) => a.length === b.length && a.length > 0 ? Math.max(...a.map((m, i) => dist(m.pos, b[i].pos))) : Infinity;
const same = (a: Man[], b: Man[]) => gap(a, b) < 1e-6 && a.every((m, i) => m.order === b[i].order && m.door === b[i].door && m.there === b[i].there);
const line = (l: Man[]) => l.map(m => `${m.c} ${at(m.pos)} [${m.order} · ${m.there}]`).join(', ');
const rowWords = (r: Row | null) => r ? `${r.off ? 'GREYED' : 'live'} "${r.text}"` : 'NO ROW';

export async function moveWalkTries(): Promise<boolean> {
  const rnd = Math.random;   // (every staging seeds it; put back with the knob tables)
  try {
    console.log('\n=== move-walktries: a \'move there\' whose way runs at a door nobody has tried walks up and tries it (ORDERS.walkTries; DOORTOOLS.testDoor set here) ==='); let all = true;
    const verdict = (ok: boolean, label: string, detail: string) => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); };
    const BREAK = process.env.WALKTRIES_BREAK === '1'; if (BREAK) console.log('  (WALKTRIES_BREAK: ORDERS.walkTries is left off in this probe\'s key-on runs — the KEY ON lines must fail)');
    const on = await play('locked', !BREAK), ref = await play('open', false);
    const refusal = `${on.name} is locked.`;
    // ---- the mend
    const K = `KEY ON${BREAK ? ' (WALKTRIES_BREAK: left off)' : ''}`;
    verdict(!on.knew0 && on.why === null && !!on.row0 && !on.row0.off && !/locked|wedged/.test(on.row0.text) && !on.said.some(s => /locked|wedged/.test(s) && !SHUT_LINE.test(s)),
      `${K} — nobody has tried the door: 'move there' is taken, the menu's row is live and names no lock, and no line names one before a hand is on the door`,
      `give('move') → ${JSON.stringify(on.why)} · menu row: ${rowWords(on.row0)} · said: ${on.said.join(' / ') || 'nothing'}`);
    const shutSaid = on.said.filter(s => SHUT_LINE.test(s)), tries = on.log.filter(l => / tried manager: /.test(l));
    verdict(on.walked.every(w => w > 1) && on.top > 0.5 && on.top <= ref.top + 0.05 && on.tested === 1 && tries.length === 1 && on.lied === 0,
      `${K} — the men walk to it, none faster than a plain 'move' walks a man (through the open conference door, key off), reading 'moving' until the handle is tried, and one man tries the leaf`,
      `read anything but 'moving' before the try: ${on.lied} frames · walked ${on.walked.map(f2).join(' / ')} m in ${f2(on.secs)} s · fastest ${f2(on.top)} m/s (a plain move: ${f2(ref.top)} m/s) · handles tried ${on.tested} · log: ${tries.join(' | ') || 'nothing'}`);
    verdict(shutSaid.length === 1 && /: locked\. holding on it — your call\.$/.test(shutSaid[0]) && on.halted === 1 && on.log.includes('move manager: tried the door · locked · halted') && on.log.includes('move manager: end: found shut'),
      `${K} — the order ends with the door orders' own words, said once by one man, and their log lines: tried, halted, 'found shut'`,
      `said: ${on.said.join(' / ') || 'nothing'} · halted ${on.halted} · log: ${on.log.join(' | ') || 'nothing'}`);
    { const stacked = (m: Man) => m.order === 'stack' && m.door === 'manager' && m.there === `stacked · ${on.name} · locked` && !!m.post && dist(m.pos, m.post) < 0.6 && dist(m.post, on.frame) < 2.6 && m.side === on.samSide;
      const told = (m: Man) => m.order === 'stack' && m.door === 'manager' && m.there === `stacked · ${on.name} · locked` && m.side === on.samSide;
      verdict(on.halt.length === 2 && on.halt.every(told) && on.later.every(stacked) && gap(on.later, on.end) < 0.05 && on.shutStill,
        `${K} — every man of the order is left stacked on that door ('stacked · <door> · locked', the halted door order's own carriage) as the line is said; three seconds on each stands on his mark, within 2.6 m of the frame on the corridor's side, nobody in the room, the leaf still shut and locked; and nobody moves in the two seconds after that`,
        `${on.later.map(m => `${m.c} ${at(m.pos)} mark ${m.post ? `${at(m.post)} ${f2(dist(m.post, on.frame))} m off the frame` : 'NONE'} side ${m.side} (Sam's ${on.samSide}) [${m.order} · ${m.there}]`).join(', ')} · as the line is said: ${line(on.halt)} · moved in the last 2 s: ${f2(gap(on.later, on.end))} m · door locked and shut: ${on.shutStill}`); }
    verdict(on.knew1 && on.why2 === refusal && !!on.row1?.off && on.row1.text.includes(refusal) && same(on.later, on.end),
      `${K} — the team knows now: the same order is refused and its row greyed with the door's name, and the men stay stacked`,
      `team knows the door: ${on.knew1} · give('move') again → ${JSON.stringify(on.why2)} · menu row: ${rowWords(on.row1)} · after it: ${line(on.end)}`);
    console.log(`=== move-walktries: ${all ? 'ALL PASS' : 'FAILURES above'} ===`);
    return all;
  } finally { Math.random = rnd; back(); }
}
