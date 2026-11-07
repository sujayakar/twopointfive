// THE NET GOES QUIET — the check behind src/game/netquiet.ts (NETQUIET.on / ?nonetquiet / TPS_NETQUIET=1; it needs MORALE on). Headless, on the compound's
// room clear (Sam alone: `levelFile=compound.json&roomclear`), the level's own squads and rooms AS THE LEVEL FILE DEALS THEM — no man is named by callsign:
// the squad's cases are the Yard's (its leader alone in the garage, four green men together in the shed), the jumpy man holding a room is the kitchen's, the
// room with doors on three walls is the sala; the last man of (Q6) / (Q7) is the garage's, stood on that room's SECOND mark (level data: from there he has a
// spot to back off to — where he is dealt, its first mark, his spot is where he stands, and that is (Q6d)'s man who stays put):
//   (Q1) a mate who does not answer shakes the one who asked (`unanswered`) and the men who heard the check (`heardIt`), a steady man by `steadyK` of it, each
//        man once
//   (Q2) the calls get worse: the name again (as the squad's own ask names him) · "why's nobody answering?" — still that, not the last line, while men of
//        OTHER squads are up on the net · "somebody talk to me!" only with nobody left on the whole net
//   (Q3) a body found: `body` to the finder, `bodyTold` to the men he tells, his line `bodyBeat` after; a finder with a shot in his ears asks nobody
//   (Q3b) one dead man is one shock: the fall he saw, the check nobody answered and the body found pay the largest of the three, not their sum
//   (Q4) a mate seen to fall: `fallUnseen` with no enemy in the watcher's view, `fallSeen` with one, nothing to a man looking away — and a green man whose
//        LEADER falls takes morale.ts's leaderShock once, not that and this
//   (Q5) the whip: a jumpy man holding a room has the NOISE as what he watches inside `whipShift` and his gun on it in well under a second; with the switch
//        off (same seed, same noise) he comes round to a doorway, later. A steady man does not whip. His reaction beat is the same number either way.
//   (Q5b) the whip is his EARS', not the world's: a noise through a wall is whipped to where his ears put it (the doorway), and his fix is never nearer the
//        truth than that; a mate's shot with a step of Sam's far out of earshot whips him nowhere; footfalls every 0.35 s do not flick his eyes back and forth
// NETQUIET_BREAK=1 = the rule's seams cut (NETQUIET_QA.broken: every feed returns on its first line, the last man's spot is never looked for, the room
// fight's watch seam hands back nothing; the switch left alone): (Q1)–(Q5) must fail.   exit 0 = all hold · 1 = one does not
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
const ROOT = resolve(import.meta.dir, '../..');
const DT = 1 / 60, f2 = (x: number) => (Number.isFinite(x) ? x.toFixed(2) : String(x)), DEG = Math.PI / 180;
const SEEDS = [71, 72, 73, 74, 75, 76], FIRE_S = 1.5;
function seeded(a: number) { return () => { a |= 0; a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ (a >>> 15), 1 | a); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; }; }
/** the compound's level file as it ships */
const compoundText = (): string => readFileSync(resolve(ROOT, 'levels', 'compound.json'), 'utf8');
const quiet = async <T>(fn: () => Promise<T> | T): Promise<T> => { const log = console.log, err = console.error, warn = console.warn; console.log = console.error = console.warn = () => {}; try { return await fn(); } finally { console.log = log; console.error = err; console.warn = warn; } };
const median = (a: number[]) => [...a].sort((x, y) => (Number.isNaN(x) ? 1e9 : x) - (Number.isNaN(y) ? 1e9 : y))[a.length >> 1];

export async function netquietCheck(): Promise<boolean> {
  const { toggleTarget } = await import(`${ROOT}/src/game/lighting.ts`);
  const { standUp } = (await import('./headless.ts')) as any;
  const M = await import(`${ROOT}/src/game/morale.ts`);
  const { MORALE, moraleOf, rungOf, trainingOf, leaderOf, moraleShock } = M;
  const NQ = await import(`${ROOT}/src/game/netquiet.ts`);
  const { NETQUIET, NETQUIET_STATS, NETQUIET_LOG, NETQUIET_QA, netQuietAsked, netQuietClosed, netQuietBody, netQuietFall, netQuietLook, netQuietPose, netQuietTick, netQuietOf, netQuietLines, netQuietSnapshot, netQuietRestore, netQuietSummary } = NQ;
  const { NETFEED, netFeedAsked, netFeedTake } = await import(`${ROOT}/src/game/netquietfeed.ts`);
  const { PLAYER_ID } = await import(`${ROOT}/src/game/consts.ts`);
  const { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`);
  const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
  const { startFromQuery } = await import(`${ROOT}/src/game/scenarioRun.ts`);
  const { killGuard } = await import(`${ROOT}/src/game/combat.ts`);
  const { squadOf, SQUADS, mate } = await import(`${ROOT}/src/game/enemysquads.ts`);
  const { knobRegistry } = await import(`${ROOT}/src/game/knobs.ts`);
  const { knobsSnapshot, applyKnobs } = await import(`${ROOT}/src/game/journal.ts`);
  const { FIGHT_SKILL } = await import(`${ROOT}/src/game/roomfight.ts`);
  const { LASTSTAND } = await import(`${ROOT}/src/game/laststand.ts`);
  const D = await import(`${ROOT}/src/game/dialogue.ts`);
  const { RUNNER, runnerBreak } = await import(`${ROOT}/src/game/runner.ts`);
  const { hasHim, OWN } = await import(`${ROOT}/src/game/owner.ts`);
  const { DARKDOOR } = await import(`${ROOT}/src/game/darkdoor.ts`);
  const BREAK = process.env.NETQUIET_BREAK === '1'; let all = true;
  const later = { runner: RUNNER.on, door: DARKDOOR.on };
  const copy = (o: unknown) => JSON.parse(JSON.stringify(o));
  const was = { morale: copy(MORALE), nq: copy(NETQUIET), rnd: Math.random, squads: SQUADS.on, callOut: LASTSTAND.callOut };
  const line = (ok: boolean, label: string, detail: string) => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); };
  console.log(`\n=== netquiet: the net goes quiet${BREAK ? ' (NETQUIET_BREAK: the rule\'s seams cut)' : ''} ===`);
  const flat = (a: number[], b: number[]) => Math.hypot(a[0] - b[0], a[2] - b[2]), wrap = (a: number) => Math.atan2(Math.sin(a), Math.cos(a));
  const yawTo = (a: number[], b: number[]) => Math.atan2(b[0] - a[0], b[2] - a[2]);
  const shockOf = (gd: any): number => moraleOf(gd)?.shock ?? 0;
  const fired = (gd: any): number => gd.weapon.shotsFired;
  const near = (a: number, b: number, e = 1e-6) => Math.abs(a - b) <= e;
  const LOWC = NETQUIET.crouch - 0.1;   // (down as far as the rule takes him)
  const P = (p: number[] | null | undefined) => p ? `${f2(p[0])},${f2(p[2])}` : 'none';

  /** the compound's room clear, Sam alone and out of everybody's sight; `on` = the switch; `training` = morale.ts's knob */
  const stage = async (seed: number, o: { on: boolean; training?: string; squads?: boolean; runner?: boolean }) => {
    Math.random = seeded(seed); MORALE.on = false; NETQUIET.on = false; NETQUIET_QA.broken = false; SQUADS.on = o.squads ?? was.squads;
    const W: any = await quiet(() => standUp({ level: loadLevel(parseLevelData(compoundText())) }));
    const g = W.game; MORALE.on = true; MORALE.nerve = 'normal'; MORALE.training = o.training ?? 'mixed'; NETQUIET.on = o.on; NETQUIET_QA.broken = BREAK && o.on;
    RUNNER.on = !!o.runner; DARKDOOR.on = false;   // (the run and the doorway rules held off but where a case asks for the first — after the stand-up, which re-reads the stand's words)
    startFromQuery(g, 'levelFile=compound.json&roomclear'); g.godMode = true;
    let lit = 0.05; const step = (n = 1) => { for (let i = 0; i < n; i++) { g.player.visibility = lit; W.step(DT); } };
    step(10);
    /** WHO IS WHO, by the level's own rooms (no man is named by his callsign): the `i`th defender the room clear deals a room, and a room's middle */
    const of = (room: string, i = 0): any => g.roomClear.rooms.find((r: any) => r.def.id === room)?.defenders[i], def = (gd: any) => g.fight.defs.find((x: any) => x.gd === gd);
    /** a room's `i`th mark, as the level file has it (a room has more marks than it is dealt men: where a second or third man of it would stand) */
    const mark = (room: string, i: number): number[] => g.roomClear.rooms.find((r: any) => r.def.id === room).def.marks[i].pos;
    const mid = (room: string): number[] => { const b = g.roomClear.rooms.find((r: any) => r.def.id === room).def.bounds; return [(b.x0 + b.x1) / 2, 0, (b.z0 + b.z1) / 2]; };
    const face = (gd: any, at: number[]) => { const y = yawTo(gd.char.pos, at); gd.char.bodyYaw = y; gd.char.aimYaw = y; gd.char.update(0); };
    const put = (gd: any, p: number[]) => { gd.char.pos = [p[0], 0, p[2]]; gd.char.vel = [0, 0, 0]; gd.path = []; gd.pathGoal = null; gd.speed = 0; gd.lastPos = [p[0], 0, p[2]]; const d = def(gd); if (d) d.station = [p[0], 0, p[2]]; gd.char.update(0); };
    /** a free spot `m` metres from `a` toward `b`, with a line from a man's eyes at `a` to it */
    const beside = (a: number[], b: number[], m: number): number[] => { const L = Math.max(1e-3, Math.hypot(b[0] - a[0], b[2] - a[2])); for (const k of [m, m + 0.4, m - 0.4, m + 0.8]) { const q = [a[0] + (b[0] - a[0]) / L * k, 0, a[2] + (b[2] - a[2]) / L * k]; if (!g.col.nav.isBlocked(q[0], q[2]) && !g.col.segmentBlocked([a[0], 1.5, a[2]], [q[0], 1.2, q[2]])) return q; } return a; };
    const openDoors = (E: any[]) => { for (const e of E) { const d = e.door, S = g.doors.swingOf(d); d.angle = S.swing * S.open; d.vel = 0; d.latched = false; d.closing = false; d.place(); } };
    return { W, g, step, of, mark, mid, def, face, put, beside, openDoors, light: (v: number) => { lit = v; } };
  };
  type Rig = Awaited<ReturnType<typeof stage>>;
  const listen = (R: Rig, who: any[], secs: number, each?: (t: number) => void) => {
    const said: { who: any; text: string; radio: boolean; t: number }[] = [], last = new Map<any, any>(who.map(h => [h, h.bubble?.t]));   // (the copy reuses a man's bubble: a new line is a new time on it)
    for (let i = 1; i <= Math.round(secs * 60); i++) { R.step(); for (const h of who) { const b = h.bubble; if (b && last.get(h) !== b.t) { last.set(h, b.t); said.push({ who: h, text: b.text, radio: !!b.radio, t: i * DT }); } } each?.(i * DT); }
    return said;
  };
  const words = (said: { text: string }[]) => said.map(s => `"${s.text}"`).join(' ') || 'nothing';
  const inPool = (kind: string, text: string, name = '') => netQuietLines(kind).some((l: string) => l.replace(/\{name\}/g, name) === text);
  const kOf = (g: any, gd: any) => trainingOf(g, gd) === 'steady' ? NETQUIET.steadyK : 1;
  const hostiles = (g: any) => g.guards.filter((x: any) => x.team === 'hostile' && x.state !== 'dead');
  /** Sam WALKS in through entry `e`: from 2.5 m outside it, straight through, to 2.5 m inside, at 1.4 m/s, lit, and stops where he is first seen — till the
   *  man fires or `secs` are up (moved by hand a frame at a time: he makes no footfall) */
  const walkIn = (R: Rig, T: any, e: any, secs = 6) => {
    const g = R.g, at = (s: number) => [e.at[0] - e.out[0] * s, 0, e.at[2] - e.out[1] * s];
    let s0 = 0; for (const s of [-2.5, -2, -1.5, -1, -0.5]) { const p = at(s); if (!g.col.nav.isBlocked(p[0], p[2])) { s0 = s; break; } }
    g.teleportPlayer(at(s0) as any); R.light(1);
    const n0 = fired(T), seen0 = T.lastSeenT; let seenAt = NaN, fireAt = NaN, depth = NaN, lowSeen = NaN, stateSeen = '';
    for (let i = 1, s = s0; i <= secs * 60 && Number.isNaN(fireAt); i++) {
      if (Number.isNaN(seenAt)) s = Math.min(2.5, s0 + i * DT * 1.4);   // (seen, he stops where he is)
      g.teleportPlayer(at(s) as any); R.step(); const t = i * DT;
      if (Number.isNaN(seenAt) && T.lastSeenT > seen0) { seenAt = t; depth = s; lowSeen = T.char.anim.crouch ?? 0; stateSeen = T.state; }
      if (fired(T) > n0) fireAt = t; }
    R.light(0.05);
    return { seenAt, fireAt, beat: fireAt - seenAt, depth, lowSeen, stateSeen };
  };

  try {

    // ---------------------------------------------------------------- (Q1) (Q2) the feeds and the lines
    { const R = await stage(61, { on: true, training: 'mixed' }), g = R.g, A = R.of('garage'), B = R.of('shed', 0), C = R.of('shed', 1), E = R.of('shed', 3), men = squadOf(g, A).men;   // (the Yard: its leader in the garage, three of its four in the shed)
      R.step(30);
      const tr = [A, B, C].map(x => trainingOf(g, x)), s0 = [A, B, C, E].map(shockOf);
      netQuietAsked(g, A, E, men); const s1 = [A, B, C, E].map(shockOf);
      netQuietAsked(g, A, E, men); const s2 = [A, B, C, E].map(shockOf);
      const K = NETQUIET, wantA = K.unanswered * kOf(g, A), wantB = K.heardIt * kOf(g, B), wantC = K.heardIt * kOf(g, C);
      line(near(s1[0] - s0[0], wantA) && near(s1[1] - s0[1], wantB) && near(s1[2] - s0[2], wantC) && s1[3] === s0[3] && s2.every((s, i) => s === s1[i]) && tr.includes('steady') && tr.includes('green') && netQuietOf(A)?.quiet === 1 && netQuietOf(B)?.quiet === 1,
        '(Q1) a man of the Yard does not answer its leader: the asker takes `unanswered`, the two who heard the check `heardIt`, a steady man steadyK of it, the silent man nothing — and asked again, nothing more',
        `${A.callsign} asks for ${E.callsign}, ${B.callsign} and ${C.callsign} hear it (${men.length} in the squad) · training ${tr.join('/')} · asker +${f2(s1[0] - s0[0])} (want ${f2(wantA)}) · hearers +${(s1[1] - s0[1]).toFixed(3)} / +${(s1[2] - s0[2]).toFixed(3)} (want ${wantB.toFixed(3)} / ${wantC.toFixed(3)}) · the silent man +${f2(s1[3] - s0[3])} · asked twice: +${f2(s2[0] - s1[0])} · quiet count ${netQuietOf(A)?.quiet}/${netQuietOf(B)?.quiet}`);
      // (Q2) the rungs: the third is 'why' again while other squads are on the net, the last line only with nobody left on it
      const gap = () => R.step(Math.round((K.barkSecs + 0.2) * 60));
      const said1 = listen(R, [A], K.askBeat + 0.6), l1 = said1[0];
      gap(); netQuietAsked(g, A, C, men); const said2 = listen(R, [A], K.askBeat + 0.6), l2 = said2[0];
      gap(); netQuietAsked(g, A, B, [A, B]); const said3 = listen(R, [A], K.askBeat + 0.6), l3 = said3[0], others = hostiles(g).filter((x: any) => !men.includes(x)).length;
      gap(); for (const x of hostiles(g)) if (x !== A) killGuard(g, x, [0, 0, 1], true, 'struck', 'player');
      const said4 = listen(R, [A], 8), l4 = said4.find(s => inPool('last', s.text));
      line(!!l1 && l1.radio && Math.abs(l1.t - K.askBeat) < 0.1 && !!l2 && inPool('why', l2.text) && l2.radio && !!l3 && inPool('why', l3.text) && others > 0 && !!l4 && l4.radio,
        '(Q2) the calls get worse, each a beat after its check, on the net: the name again (the squad\'s own name for him) · why is nobody answering · that again — NOT the last line — with his own squad silent but other squads\' men up · somebody talk to me, with nobody left on the whole net',
        `first: ${l1 ? `"${l1.text}" at ${f2(l1.t)} s` : words(said1)} · second: ${l2 ? `"${l2.text}"` : words(said2)} · his squad all quiet, ${others} others on the net: ${l3 ? `"${l3.text}"` : words(said3)} · nobody left: ${l4 ? `"${l4.text}"` : words(said4)} · quiet count ${netQuietOf(A)?.quiet}`); }

    // ---------------------------------------------------------------- (Q3) a body · (Q3b) one dead man, one shock
    { const R = await stage(61, { on: true, training: 'mixed' }), g = R.g, A = R.of('garage'), B = R.of('shed', 0), C = R.of('shed', 1), men = squadOf(g, A).men, K = NETQUIET;
      R.step(30);
      const b0 = [A, B, C].map(shockOf); netQuietBody(g, B, men); const b1 = [A, B, C].map(shockOf);
      const saidB = listen(R, [B], K.bodyBeat + 0.6), lb = saidB[0];
      C.witness.heard.lastShotT = g.time; netQuietBody(g, C, men); const owed = netQuietOf(C)?.say;
      line(near(b1[1] - b0[1], Math.min(K.body * kOf(g, B), MORALE.shockCap - b0[1])) && b1[0] - b0[0] > 0 && b1[0] - b0[0] <= K.bodyTold + 1e-9 && !!lb && inPool('body', lb.text) && Math.abs(lb.t - K.bodyBeat) < 0.1 && owed === -1,
        '(Q3) a body found: `body` to the finder, less to the men he tells, his line a beat after — and a finder with a shot in his ears asks nobody who heard a thing',
        `finder +${f2(b1[1] - b0[1])} (body ${K.body} × ${kOf(g, B)}, cap ${MORALE.shockCap}) · told +${(b1[0] - b0[0]).toFixed(3)} / +${(b1[2] - b0[2]).toFixed(3)} (bodyTold ${K.bodyTold}) · ${lb ? `"${lb.text}" at ${f2(lb.t)} s` : words(saidB)} · the man who heard the shot owes ${owed === -1 ? 'no line' : `line ${owed}`}`);
      // (Q3b) green men: A watches C fall with no enemy in view, B looks away; then C is asked for, then C is found — all in one frame (no decay between)
      const R2 = await stage(62, { on: true, training: 'green' }), g2 = R2.g, A2 = R2.of('shed', 0), B2 = R2.of('shed', 1), C2 = R2.of('shed', 2), men2 = squadOf(g2, A2).men;   // (three men of one room)
      R2.step(30); R2.put(B2, R2.beside(A2.char.pos, C2.char.pos, 1.5)); R2.step(2);
      for (const t of g2.targets) if (!t.on) toggleTarget(g2, t);
      R2.face(A2, C2.char.pos); { const p = B2.char.pos, q = C2.char.pos; R2.face(B2, [2 * p[0] - q[0], 0, 2 * p[2] - q[2]]); }
      const c0 = [A2, B2].map(shockOf); killGuard(g2, C2, [0, 0, 1], true, 'shot', 'player'); const c1 = [A2, B2].map(shockOf);
      netQuietAsked(g2, A2, C2, men2); const c2 = [A2, B2].map(shockOf);
      netQuietBody(g2, A2, men2); const c3 = [A2, B2].map(shockOf), knows = !!netQuietOf(A2)?.known.has(C2.char.id) && !!netQuietOf(B2)?.known.has(C2.char.id);
      line(near(c1[0] - c0[0], K.fallUnseen) && near(c3[0] - c0[0], K.fallUnseen) && near(c2[1] - c0[1], K.heardIt) && near(c3[1] - c0[1], K.bodyTold) && knows,
        '(Q3b) one dead man is one shock, the largest of the three: the man who saw him fall takes `fallUnseen` and nothing more for the check nobody answered or for finding him; the man who looked away takes `heardIt` for the check, and only the rest of `bodyTold` when told of the body',
        `watcher: fall +${f2(c1[0] - c0[0])}, then the check +${f2(c2[0] - c1[0])}, then the body +${f2(c3[0] - c2[0])} = ${f2(c3[0] - c0[0])} (the old sum: ${f2(K.fallUnseen + K.unanswered + K.body)}) · the other man: fall +${f2(c1[1] - c0[1])}, check +${f2(c2[1] - c1[1])}, told +${f2(c3[1] - c2[1])} = ${f2(c3[1] - c0[1])} (old sum ${f2(K.heardIt + K.bodyTold)}) · both know he is gone: ${knows}`); }

    // ---------------------------------------------------------------- (Q4) a mate seen to fall
    { const R = await stage(62, { on: true, training: 'green' }), g = R.g, A = R.of('shed', 0), B = R.of('shed', 1), C = R.of('shed', 2), K = NETQUIET;
      R.step(30); R.put(B, R.beside(A.char.pos, C.char.pos, 1.5)); R.step(2);
      const away = (gd: any, from: any) => { const p = gd.char.pos, q = from.char.pos; R.face(gd, [2 * p[0] - q[0], 0, 2 * p[2] - q[2]]); };
      // unseen shooter: A watches C fall, B looks away
      for (const t of g.targets) if (!t.on) toggleTarget(g, t);
      R.face(A, C.char.pos); away(B, C); const u0 = [A, B].map(shockOf), f0 = NETQUIET_STATS.unseen;
      killGuard(g, C, [0, 0, 1], true, 'shot', 'player'); const u1 = [A, B].map(shockOf);
      // a seen shooter: B has had an enemy in his eyes this second, and watches A fall
      R.step(2); R.face(B, A.char.pos); B.lastSeenT = g.time; const v0 = shockOf(B); killGuard(g, A, [0, 0, 1], true, 'shot', 'player'); const v1 = shockOf(B);
      // the leader's fall, on a mixed squad: the green man takes leaderShock, once
      const R2 = await stage(63, { on: true, training: 'mixed' }), g2 = R2.g, G = [R2.of('shed', 0)].find((x: any) => x && trainingOf(g2, x) === 'green' && leaderOf(g2, x)), L = G ? leaderOf(g2, G) : null;   // (a green man of the Yard, stood in the garage at his leader's side)
      R2.step(30); let led = NaN, ledWant = NaN;
      if (G && L) { R2.put(G, R2.beside(L.char.pos, R2.mid('garage'), 1.5)); R2.step(2); for (const t of g2.targets) if (!t.on) toggleTarget(g2, t); R2.face(G, L.char.pos); const l0 = shockOf(G); killGuard(g2, L, [0, 0, 1], true, 'shot', 'player'); led = shockOf(G) - l0; ledWant = Math.max(MORALE.leaderShock, K.fallUnseen); }
      line(near(u1[0] - u0[0], K.fallUnseen) && u1[1] === u0[1] && NETQUIET_STATS.unseen >= f0 + 1 && near(v1 - v0, K.fallSeen) && near(led, ledWant),
        '(Q4) a mate seen to fall: `fallUnseen` with no enemy in the watcher\'s view, nothing to the man looking away, `fallSeen` with an enemy in view — and a leader\'s fall is morale.ts\'s leaderShock once, not both',
        `no shooter seen: watcher +${f2(u1[0] - u0[0])} (fallUnseen ${K.fallUnseen}), the man looking away +${f2(u1[1] - u0[1])} · shooter in view: +${f2(v1 - v0)} (fallSeen ${K.fallSeen}) · the leader ${L?.callsign ?? '—'} falls in ${G?.callsign ?? '—'}'s eyes: +${f2(led)} (want ${f2(ledWant)}: leaderShock ${MORALE.leaderShock})`); }

    // ---------------------------------------------------------------- (Q5) the whip
    { const jumpyMan = async (on: boolean, jumpy = true) => { const R = await stage(64, { on, training: 'green' }), g = R.g, T = R.of('kitchen'), Df = R.def(T); R.step(30); if (jumpy) moraleShock(g, T, 0.4); R.step(20); return { R, g, T, Df, E: Df.room.entries as any[], p: T.char.pos as number[] }; };
      const run = async (on: boolean, jumpy: boolean) => {
        const { R, g, T, Df, E, p } = await jumpyMan(on, jumpy);
        // a footfall of Sam's inside his room, 2.4 m off, in the direction furthest from every doorway and well off his gun
        let best: number[] | null = null, bs = -1;
        for (let a = 0; a < 16; a++) { const y = a * Math.PI / 8, q = [p[0] + Math.sin(y) * 2.4, 0, p[2] + Math.cos(y) * 2.4];
          if (g.col.nav.isBlocked(q[0], q[2]) || g.col.segmentBlocked([p[0], 1.4, p[2]], [q[0], 1, q[2]])) continue;
          const offDoors = Math.min(...E.map((e: any) => Math.abs(wrap(yawTo(p, e.at) - y)))), offGun = Math.abs(wrap(T.char.aimYaw - y)); const s = Math.min(offDoors, offGun);
          if (s > bs) { bs = s; best = q; } }
        const noise = best!, w0 = NETQUIET_STATS.whips, react: number[] = [];
        g.events.push({ kind: 'step', pos: [noise[0], 0, noise[2]], time: g.time, loud: false, level: 1, who: PLAYER_ID });
        let ptAt = NaN, gunAt = NaN, doorAt = NaN;
        for (let i = 1; i <= 90; i++) { R.step(); const t = i * DT; react.push(T.reactT);
          if (Number.isNaN(ptAt) && flat(Df.pt, noise) < 0.3) ptAt = t;
          if (Number.isNaN(gunAt) && Math.abs(wrap(yawTo(T.char.pos, noise) - T.char.aimYaw)) < 0.25 && Math.abs(wrap(yawTo(T.char.pos, noise) - T.char.bodyYaw)) < 0.6) gunAt = t;
          if (Number.isNaN(doorAt) && E.some((e: any) => flat(Df.pt, e.at) < 0.3)) doorAt = t; }
        return { rung: rungOf(T), ptAt, gunAt, doorAt, whips: NETQUIET_STATS.whips - w0, off: bs, react: react.map(x => x.toFixed(3)).join(), heard: T.witness.heard.small };
      };
      const on = await run(true, true), calm = await run(true, false), K = NETQUIET;
      line(on.rung >= 1 && on.whips === 1 && on.ptAt <= K.whipShift + 0.1 && on.gunAt <= 0.6 && calm.whips === 0 && Number.isNaN(calm.ptAt) && on.off > 0.6,
        '(Q5) a jumpy man holding a room whips to a footfall inside it: the NOISE is what he watches inside whipShift and his gun and body are on it inside 0.6 s; a steady man does not whip',
        `noise ${f2(on.off / DEG)}° off every door and his gun · on (rung ${on.rung}, ${on.whips} whip): watches the noise at ${f2(on.ptAt)} s, gun on it at ${f2(on.gunAt)} s · steady man (rung ${calm.rung}): ${calm.whips} whips · he heard it: ${on.heard}`);
      // (Q5b) his ears', not the world's
      // 1: a loud scrape outside his room, behind a wall
      const w1 = await jumpyMan(true); let wall: number[] | null = null;
      for (let r = 3; r <= 9 && !wall; r += 0.5) for (let a = 0; a < 24; a++) { const y = a * Math.PI / 12, q = [w1.p[0] + Math.sin(y) * r, 0, w1.p[2] + Math.cos(y) * r], nav = w1.g.col.nav;
        if (nav.isBlocked(q[0], q[2]) || !w1.g.col.segmentBlocked([w1.p[0], 1.5, w1.p[2]], [q[0], 1, q[2]]) || w1.Df.room.cells.has(((q[2] / nav.cell) | 0) * nav.W + ((q[0] / nav.cell) | 0))) continue; wall = q; break; }
      w1.g.events.push({ kind: 'prop', pos: wall, time: w1.g.time, loud: true, level: 1, who: PLAYER_ID });
      let truest = Infinity, whipAt: number[] | null = null, ears: number[] | null = null;
      for (let i = 0; i < 60; i++) { w1.R.step(); const N = netQuietOf(w1.T); if (N?.whipAt && !whipAt) { whipAt = [...N.whipAt]; ears = w1.T.witness.heard.smallWhere ? [...w1.T.witness.heard.smallWhere] : null; } if (w1.T.lastKnown) truest = Math.min(truest, flat(w1.T.lastKnown, wall!)); }
      const earsOff = ears ? flat(ears, wall!) : NaN, wallOk = !!whipAt && !!ears && flat(whipAt, ears) < 0.05 && earsOff > 0.5 && truest >= earsOff - 0.05;
      // 2: a mate's shot he hears, and a faint step of Sam's far out of earshot in the same frame
      const w2 = await jumpyMan(true), mateMan = w2.R.of('shed'); let far: number[] = [2, 0, 2]; for (const c of [[2, 0, 2], [40, 0, 40], [2, 0, 40], [40, 0, 2]]) if (flat(c, w2.p) > flat(far, w2.p)) far = c;
      const wh0 = NETQUIET_STATS.whips, sm0 = w2.T.witness.heard.small, sh0 = w2.T.witness.heard.shots;
      w2.g.events.push({ kind: 'guardShot', pos: [...mateMan.char.pos], time: w2.g.time, loud: true, level: 1, who: mateMan.char.id });
      w2.g.events.push({ kind: 'step', pos: far, time: w2.g.time, loud: false, level: 0.1, who: PLAYER_ID });
      let nearFar = Infinity, whipped = false; for (let i = 0; i < 40; i++) { w2.R.step(); if (netQuietOf(w2.T)?.whipAt) whipped = true; nearFar = Math.min(nearFar, flat(w2.Df.pt, far), w2.T.lastKnown ? flat(w2.T.lastKnown, far) : Infinity); }
      const mateOk = !whipped && NETQUIET_STATS.whips === wh0 && w2.T.witness.heard.shots > sh0 && w2.T.witness.heard.small === sm0 && nearFar > 5;
      // 3: footfalls every 0.35 s at one spot in his room, behind his gun
      const w3 = await jumpyMan(true); let q3: number[] | null = null;
      for (let a = 0; a < 16 && !q3; a++) { const y = w3.T.char.aimYaw + Math.PI + a * Math.PI / 8, c = [w3.p[0] + Math.sin(y) * 2.4, 0, w3.p[2] + Math.cos(y) * 2.4]; if (!w3.g.col.nav.isBlocked(c[0], c[2]) && !w3.g.col.segmentBlocked([w3.p[0], 1.4, w3.p[2]], [c[0], 1, c[2]])) q3 = c; }
      let flips = 0, wasOn = false, onF = 0;
      for (let i = 0; i < 180; i++) { if (i % 21 === 0) w3.g.events.push({ kind: 'step', pos: [...q3!], time: w3.g.time, loud: false, level: 1, who: PLAYER_ID }); w3.R.step(); const o = flat(w3.Df.pt, q3!) < 0.3; if (o !== wasOn) flips++; wasOn = o; if (o) onF++; }
      // …and two noises 0.3 s apart at two places: the first is held `whipHold` (the second does not take his eyes inside it)
      const w4 = await jumpyMan(true), spots: number[][] = [];
      for (let a = 0; a < 16 && spots.length < 2; a++) { const y = w4.T.char.aimYaw + Math.PI / 2 + a * Math.PI / 8, c = [w4.p[0] + Math.sin(y) * 2.2, 0, w4.p[2] + Math.cos(y) * 2.2]; if (!w4.g.col.nav.isBlocked(c[0], c[2]) && !w4.g.col.segmentBlocked([w4.p[0], 1.4, w4.p[2]], [c[0], 1, c[2]]) && (!spots.length || flat(c, spots[0]) > 1.5)) spots.push(c); }
      let heldFirst = 0, tookSecond = NaN;
      if (spots.length === 2) for (let i = 0; i < 150; i++) { if (i === 0) w4.g.events.push({ kind: 'step', pos: [...spots[0]], time: w4.g.time, loud: false, level: 1, who: PLAYER_ID }); if (i === 18 || i === 80) w4.g.events.push({ kind: 'step', pos: [...spots[1]], time: w4.g.time, loud: false, level: 1, who: PLAYER_ID }); w4.R.step(); if (flat(w4.Df.pt, spots[0]) < 0.3) heldFirst++; if (Number.isNaN(tookSecond) && flat(w4.Df.pt, spots[1]) < 0.3) tookSecond = i * DT; }
      line(wallOk && mateOk && flips <= 2 && onF >= 150 && spots.length === 2 && heldFirst >= (K.whipHold - 0.05) * 60 && tookSecond > 1.3,
        '(Q5b) the whip is his ears\': a scrape through a wall is whipped to where HIS EARS put it (the doorway it came round), and his fix is never nearer the truth than that; a mate\'s shot, with a step of Sam\'s out of earshot in the same frame, whips him nowhere; footfalls every 0.35 s at one spot keep his eyes there (no flicking back to the door); a second noise 0.3 s after the first does not take his eyes inside whipHold',
        `wall: true source ${P(wall)}, his ears ${P(ears)} (${f2(earsOff)} m off it), whipped to ${P(whipAt)}; his fix never nearer the source than ${f2(truest)} m · mate's shot heard (+${w2.T.witness.heard.shots - sh0} shot, +${w2.T.witness.heard.small - sm0} small), Sam's step ${f2(flat(far, w2.p))} m off: whipped ${whipped}, nothing of his nearer it than ${f2(nearFar)} m · steps at 0.35 s: ${flips} flips in 3 s, on the noise ${onF}/180 frames · two places: the first held ${f2(heldFirst * DT)} s, the second taken at ${f2(tookSecond)} s (it was made at 0.30 s and again at 1.33 s)`); }

  } catch (e) { all = false; console.log(`- FAIL — the check threw\n    ${e instanceof Error ? e.stack : String(e)}`); }
  finally { RUNNER.on = later.runner; DARKDOOR.on = later.door; SQUADS.on = was.squads; LASTSTAND.callOut = was.callOut; Object.assign(MORALE, was.morale); Object.assign(NETQUIET, was.nq); NETQUIET_QA.broken = false; NETFEED.live = false; Math.random = was.rnd; }
  console.log(`=== netquiet: ${all ? 'ALL PASS' : 'FAIL'} ===`);
  return all;
}

if (import.meta.main) {
  process.exit((await netquietCheck()) ? 0 : 1);
}
