// The 'debrief' family (run through probes.ts: `bun run tools/qa/probes.ts debrief` — also in 'all'): the run's chronicle and the debrief made of it
// (rust/game_rules/src/chronicle, src/ui/debrief.ts). What is held here, of the Rust game's record as the page reads it (the words, the score and the caps are held natively: rust/game_start/tests/debrief.rs, chronicle's own tests):
//   debrief-run    (a) a scripted run writes the expected timeline, in order, with the right rooms, and scores in the expected band with the expected badges —
//                      seen once by a staged corridor man → he loses Sam → gives the search up → the floor goes up (a radio line with it) → grabbed, questioned to
//                      his key, choked out → a door kicked in → a muffled shot a second man hears; and a quiet run — a lock picked, a panel OCP'd, an exit sign
//                      shot out, a canister thrown, the mission walked to its end — that comes out GHOST with the quiet badges lit and the end card asked for;
//                  (b) restartEncounter empties it (timeline, path, counters) and the tour's flag keeps it empty;
//   debrief-rooms  (c) roomOf() names thirteen known points right (the eight rooms + the four sides of the grounds) and the level's spawn / rack / exfil marks;
//   debrief-dom    (e) the card builds headlessly on a small fake DOM off a finished run: rating + score, the stats grid, eight chips, a timeline row per entry,
//                      a pin per pinned entry + the three marks, and the copy button's line.
import { standUp, ROOT } from './headless.ts';
import { endFor } from './verbs.ts';
const { v3 } = await import(`${ROOT}/src/math/vec.ts`);
const { SCORE } = await import(`${ROOT}/src/game/chronicle.ts`);
const { roomOf } = await import(`${ROOT}/src/game/rooms.ts`);
const { debriefOf, rowsOf, rowLine } = await import('./debrief-read.ts');
/** the four words of a graded run (rust/game_rules/src/chronicle bands) */
const BANDS = ['GHOST', 'SHADOW', 'OPERATIVE', 'ASSAULT'];
const { startGrab, startKick, startPicking, doThrow } = await import(`${ROOT}/src/game/player.ts`);
const { fireWeapon } = await import(`${ROOT}/src/game/combat.ts`);
const { rollcallState } = await import(`${ROOT}/src/game/guards.ts`);
const { HOLD } = await import(`${ROOT}/src/game/character.ts`);
const { DRIVE_SECS } = await import(`${ROOT}/src/game/consts.ts`);

type Vec3 = [number, number, number];
const DT = 1 / 60;
function seeded(a: number) { return () => { a |= 0; a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ (a >>> 15), 1 | a); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; }; }
function place(gd: any, p: Vec3, yaw: number) { gd.char.pos = v3.copy(p); gd.char.vel = [0, 0, 0]; gd.speed = 0; gd.char.bodyYaw = gd.char.aimYaw = yaw; gd.char.aimPitch = 0; gd.path = []; gd.pathGoal = null; gd.char.update(0); gd.char.update(DT); }
/** parked on the north verge (grass, outside the walls, far from every fixture the runs touch), held, facing north, deaf */
function park(gd: any, i: number) { gd.hold = true; place(gd, [8 + i * 3, 0, 1.6], Math.PI); gd.heardUpTo = 1e12; gd.flashlight.enabled = false; }
const f1 = (x: number) => x.toFixed(1);
const steps = (W: any, secs: number, until?: () => boolean) => { const n = Math.round(secs / DT); for (let i = 0; i < n; i++) { W.step(DT); if (until?.()) return true; } return !until; };
/** is `want` a subsequence of `got` (other rows may sit between) — returns the first wanted kind that could not be matched, or '' */
function subseqMiss(got: string[], want: string[]): string { let i = 0; for (const k of got) if (i < want.length && k === want[i]) i++; return i < want.length ? want[i] : ''; }
const kindsOf = (game: any): string[] => game.chronicle.events.map((e: any) => e.kind);
const verdictLine = (ok: boolean, label: string, detail: string, fails: string[]) => console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}${fails.length ? '\n    ✗ ' + fails.join('\n    ✗ ') : ''}`);

// ================================================================ (a) + (b)
export async function debriefRun() {
  console.log('\n=== debrief-run: a scripted run writes the expected timeline (kinds in order, rooms), the score lands in the expected band with the expected badges; a quiet run to the end comes out GHOST and asks for the card; a restart empties it ===');
  let allPass = true;
  // ---------------------------------------------------------------- (a1) the loud half: seen → lost → calm → the floor up → grab / question / key / choke → a door kicked in → a muffled shot heard
  let carried: any = null;
  {
    Math.random = seeded(9051);
    const W = await standUp(); const { game } = W; const fails: string[] = []; const notes: string[] = [];
    game.player.visibility = 0.02; game.godMode = true; game.player.slot = 'pistol'; game.player.holstered = true;
    steps(W, 0.1);
    const [A, B, Cc] = game.guards;   // Kowalski (corridor), Novak (cubicles), Reyes (lobby / break room)
    park(B, 1); park(Cc, 2);
    A.hold = false; place(A, [14, 0, 11.2], Math.PI / 2); A.wp = 1; A.heardUpTo = game.eventSeq;
    rollcallState(game).nextAt = game.time + 900;
    // seen: Sam planted 4 m in front of him, lit
    game.teleportPlayer([18, 0, 11.2]); game.player.crouch = false; steps(W, DT); game.player.visibility = 1.0;
    const seenAt = game.time;
    const spotted = steps(W, 6, () => game.mission.stats.spotted > 0);
    if (!spotted) fails.push(`never spotted in 6 s lit in front of ${A.callsign} (his state ${A.state}, awareness ${f1(A.awareness)})`);
    notes.push(`spotted after ${f1(game.time - seenAt)} s (${A.callsign} ${A.state})`);
    // gone: across the floor, dark — he runs to the spot, loses him, searches, gives up; the floor goes up a step on it
    game.teleportPlayer([33, 0, 20]); game.player.visibility = 0.02; steps(W, DT);
    const lostBy = steps(W, 40, () => kindsOf(game).includes('lost'));
    const calmBy = steps(W, 45, () => kindsOf(game).includes('calm'));
    const upBy = steps(W, 10, () => game.escalation >= 1);
    if (!lostBy || !calmBy || !upBy) fails.push(`the hunt did not run its course: lost ${lostBy} calm ${calmBy} floor-up ${upBy} (${A.callsign} ${A.state}, escalation ${game.escalation})`);
    notes.push(`hunt over at +${f1(game.time - seenAt)} s, floor ${game.escalation}`);
    steps(W, 1.0);
    // grabbed from behind, questioned until the door (and his key) comes, choked out
    place(A, [16, 0, 11.2], Math.PI / 2); A.hold = false;
    game.teleportPlayer([15.2, 0, 11.2]); game.player.crouch = true; steps(W, DT); game.player.visibility = 0.02;
    startGrab(game, A);
    const held = steps(W, 3, () => game.player.holding?.phase === 'held');
    if (!held) fails.push(`the grab never settled (holding ${game.player.holding?.phase ?? 'none'}, ${A.callsign} ${A.state})`);
    game.player.fHeld = false; steps(W, DT);
    let gave: string | null = null; const kinds: string[] = [];
    for (let i = 0; i < 8 && gave !== 'door'; i++) { const p = game.interrogate(A, seeded(60 + i)); gave = p?.kind ?? null; if (p?.kind) kinds.push(p.kind); steps(W, 0.5); }
    if (gave !== 'door') fails.push(`eight exchanges never brought the keyed door (gave [${kinds.join(',') || 'nothing'}])`);
    if (!game.player.keys.size) fails.push('no key came off him');
    notes.push(`questioned: gave [${kinds.join(',') || '—'}], keys [${[...game.player.keys].join(',')}]`);
    if (game.player.holding) { endFor(W, HOLD.secs.chokeHold + HOLD.secs.choke + 0.3); steps(W, 0.6); }
    if (A.state !== 'dead' || A.downKind !== 'choked') fails.push(`the choke did not put him down breathing (${A.state} / ${A.downKind})`);
    // a door kicked in (nobody able to hear it: the parked pair are deaf, the corridor man is out)
    const storage = game.doors.byName('storage');
    game.teleportPlayer([32.6, 0, 10.9]); game.player.crouch = false; game.player.char.bodyYaw = Math.PI; steps(W, 2 * DT);
    startKick(game, storage);
    const kicked = steps(W, 2.0, () => storage.lockBroken); steps(W, 0.8);
    if (!kicked) fails.push(`the kick never landed (kick ${!!game.player.kick}, lockBroken ${storage.lockBroken})`);
    // a muffled shot three metres behind the cubicle man (un-deafened, planted facing away): he hears it, looks into it, raises nothing
    B.heardUpTo = game.eventSeq; place(B, [20, 0, 18], Math.PI); B.hold = true;
    game.teleportPlayer([20, 0, 21]); game.player.crouch = true; game.player.slot = 'pistol'; game.player.holstered = false; steps(W, 0.3); game.player.visibility = 0.02;
    const alertsBefore = game.mission.stats.alerts;
    fireWeapon(game, game.player.char, [20, 0.4, 23.6], true);
    steps(W, 2 * DT); game.playerInvisible = true;   // (heard: from here he turns to a man under the cubicles' lamps, and what he then sees is not the question)
    steps(W, 1.5); game.playerInvisible = false;
    if (game.mission.stats.alerts !== alertsBefore || B.state === 'alert') fails.push(`the muffled shot raised an alarm (alerts ${alertsBefore} → ${game.mission.stats.alerts}, ${B.callsign} ${B.state})`);
    // ---- the timeline
    const ev = game.chronicle.events as any[]; const kindsGot = kindsOf(game);
    const WANT = ['contact', 'spotted', 'lost', 'calm', 'alarm', 'grab', 'interrogated', 'keyTaken', 'ko', 'doorKicked', 'shot', 'heard'];
    const miss = subseqMiss(kindsGot, WANT);
    if (miss) fails.push(`timeline kinds [${kindsGot.join(' ')}] — '${miss}' missing / out of order (want the subsequence ${WANT.join(' › ')})`);
    const first = (k: string) => ev.find(e => e.kind === k);
    const roomWant: Record<string, string> = { contact: 'corridor', spotted: 'corridor', grab: 'corridor', ko: 'corridor', doorKicked: 'corridor', shot: 'cubicles', heard: 'cubicles' };
    for (const [k, r] of Object.entries(roomWant)) { const e = first(k); if (e && e.room !== r) fails.push(`'${k}' happened in '${e.room}' (want '${r}')`); }
    { const e = first('contact'); if (e && (e.data.who !== A.callsign || e.data.cause !== 'sight')) fails.push(`contact row: ${JSON.stringify(e.data)} (want ${A.callsign} by sight)`); }
    { const e = first('spotted'); if (e && (e.data.by !== A.callsign || e.data.lvl !== 0)) fails.push(`spotted row: ${JSON.stringify(e.data)} (want by ${A.callsign}, floor calm)`); }
    { const e = first('alarm'); if (e && (!e.data.up || e.data.stage !== 1 || e.data.cause !== 'search')) fails.push(`alarm row: ${JSON.stringify(e.data)} (want ▲ heightened, cause search)`); }
    if (!ev.some(e => e.kind === 'radio' && e.t >= (first('alarm')?.t ?? 1e9) - 1e-6)) fails.push('no radio row with / after the floor going up (the order goes over the net)');
    { const e = first('ko'); if (e && (e.data.who !== A.callsign || e.data.method !== 'choke')) fails.push(`ko row: ${JSON.stringify(e.data)} (want ${A.callsign} choked)`); }
    { const e = first('doorKicked'); if (e && e.data.door !== 'storage') fails.push(`doorKicked row: ${JSON.stringify(e.data)}`); }
    { const e = first('heard'); if (e && (e.data.what !== 'a muffled shot' || !String(e.data.by).includes(B.callsign) || e.data.react === 'alerted')) fails.push(`heard row: ${JSON.stringify(e.data)} (want a muffled shot · ${B.callsign} investigates)`); }
    { const e = first('shot'); if (e && (e.data.near !== false || e.data.into !== 'geometry')) fails.push(`shot row: ${JSON.stringify(e.data)} (want into geometry, nobody near)`); }
    if (ev.some(e => e.kind === 'kill' || e.kind === 'bodyFound' || e.kind === 'down')) fails.push(`rows that cannot have happened: ${kindsGot.filter(k => k === 'kill' || k === 'bodyFound' || k === 'down').join(',')}`);
    for (let i = 1; i < ev.length; i++) if (ev[i].t < ev[i - 1].t - 1e-9) { fails.push(`timeline not in time order at row ${i} (${ev[i - 1].kind} then ${ev[i].kind})`); break; }
    // ---- the numbers, the score, the badges
    const D = debriefOf(game); const c = D.counts;
    if (c.seen !== 1 || c.alarms !== 1 || c.esc1 !== 1 || c.esc2 !== 0 || c.kos !== 1 || c.kills !== 0 || c.kicked !== 1 || c.picked !== 0 || c.keys !== 1 || c.shots !== 1 || c.shotsNearMen !== 0 || c.bodies !== 0) fails.push(`counts: seen ${c.seen} alarms ${c.alarms} esc ${c.esc1}/${c.esc2} kos ${c.kos} kills ${c.kills} kicked ${c.kicked} picked ${c.picked} keys ${c.keys} shots ${c.shots} near ${c.shotsNearMen} bodies ${c.bodies} (want 1 1 1/0 1 0 1 0 1 1 0 0)`);
    const wantScore = 100 - SCORE.seen - SCORE.alarm - SCORE.heightened - SCORE.ko;
    if (D.score !== wantScore || D.rating !== 'SHADOW' || D.state !== 'sofar') fails.push(`score ${D.score} ${D.rating} (${D.state}) — want ${wantScore} SHADOW sofar; docked: ${D.lines.map((l: any) => `−${l.pts} ${l.label}`).join(', ')}`);
    const on = new Set(D.badges.filter((b: any) => b.on).map((b: any) => b.id));
    for (const [id, want] of [['nokills', true], ['pacifist', true], ['unseen', false], ['quiet', false], ['locksmith', false], ['talkative', false], ['speedrun', false], ['lightsout', false]] as const) if (on.has(id) !== want) fails.push(`badge '${id}' ${on.has(id) ? 'lit' : 'dim'} (want ${want ? 'lit' : 'dim'})`);
    if (game.chronicle.pathN < 4) fails.push(`path has ${game.chronicle.pathN} points after ${f1(c.elapsed)} s of run`);
    if (!/^twopointsix — SHADOW \d+ · \d\d:\d\d · seen 1 · alarms 1 · /.test(D.summary) || !/run so far/.test(D.summary)) fails.push(`summary line: "${D.summary}"`);
    notes.push(`timeline (${ev.length}): ${rowsOf(game).map(rowLine).join(' | ')}`);
    notes.push(`→ ${D.rating} ${D.score} · badges [${[...on].join(', ')}] · path ${game.chronicle.pathN} pts`);
    const ok = fails.length === 0; allPass &&= ok;
    verdictLine(ok, '(a1) seen once by the corridor man → lost → he gives up → floor ▲ heightened (+ the radio) → grabbed, questioned to his key, choked out → the storage door kicked in → a muffled shot the cubicle man hears: those rows in that order in those rooms, counts 1/1/1/1ko/1kick/1key/1shot, score = 100 − seen − alarm − heightened − ko = SHADOW, no kills + pacifist lit, the rest dim', notes.join('\n    '), fails);
    carried = W;
  }
  // ---------------------------------------------------------------- (b) restart empties it; the tour's flag keeps it empty
  {
    const W = carried; const { game } = W; const fails: string[] = [];
    const old = game.chronicle;
    game.restartEncounter(); steps(W, DT);
    const C = game.chronicle;
    void old;   // (the chronicle is held to being EMPTY, below: whether it is another object is the tools' copy's own business, where a record is written in place)
    if (C.events.length || C.pathN > 1 || C.dropped || C.questioned.size || Object.entries(C.counts).some(([, v]) => v !== 0 && v !== false && v !== '')) fails.push(`after restart: ${C.events.length} rows, ${C.pathN} path pts, counts ${JSON.stringify(C.counts)}`);
    if (game.mission.stats.spotted || game.mission.stats.alerts || debriefOf(game).score !== 100) fails.push(`after restart the mission stats / score are not fresh (${JSON.stringify(game.mission.stats)}, ${debriefOf(game).score})`);
    // under the tour's flag nothing lands: a shot, a generic row, the path
    game.quietUtility = true; const n0 = C.events.length, p0 = C.pathN, shots0 = game.mission.stats.shots;
    game.player.slot = 'pistol'; game.player.holstered = false; steps(W, 0.2);
    fireWeapon(game, game.player.char, [game.player.char.pos[0] + 3, 0.5, game.player.char.pos[2]], true);
    steps(W, 2.0);
    if (C.events.length !== n0 || C.pathN !== p0 || game.mission.stats.shots !== shots0) fails.push(`under quietUtility: rows ${n0} → ${C.events.length}, path ${p0} → ${C.pathN}, shots ${shots0} → ${game.mission.stats.shots} (want nothing to move)`);
    game.quietUtility = false; steps(W, 1.2);
    if (C.pathN <= p0) fails.push('with the flag down again the path did not resume');
    const ok = fails.length === 0; allPass &&= ok;
    verdictLine(ok, '(b) restartEncounter hands the game a fresh, empty chronicle (rows, path, counters, questioned) with fresh stats; under the tour\'s quietUtility a shot and the path sample land nowhere; the flag down again, the path resumes', `rows ${C.events.length}, path ${C.pathN}, loads ${C.counts.loads}`, fails);
  }
  // ---------------------------------------------------------------- (a2) the quiet half, to the end: pick, OCP, a sign shot out, a canister, the mission → GHOST, the card asked for
  {
    Math.random = seeded(9052);
    const W = await standUp(); const { game, level } = W; const fails: string[] = []; const notes: string[] = [];
    game.player.visibility = 0.02; game.player.slot = 'pistol'; game.player.holstered = true;
    steps(W, 0.1);
    game.guards.forEach((gd: any, i: number) => park(gd, i)); rollcallState(game).nextAt = game.time + 900;
    // the server door's lock, picked
    const server = game.doors.byName('server');
    game.teleportPlayer([17.6, 0, 10.75]); game.player.crouch = true; game.player.char.bodyYaw = Math.PI; steps(W, 2 * DT);
    startPicking(game, server);
    const picked = steps(W, 9, () => !server.locked); steps(W, 0.3);
    if (!picked) fails.push(`the pick never opened it (picking ${!!game.player.picking}, pick ${f1(server.pick)}, locked ${server.locked})`);
    // a corridor panel, OCP'd
    const panel = game.targets.find((t: any) => t.kind === 'emissive' && t.interactive && t.group === 'corridor' && /^panel_/.test(t.name) && t.factor > 0);
    if (!panel) fails.push('staging: no lit corridor panel to OCP'); else { game.ocp(panel); if (panel.disabledUntil <= game.time) fails.push('the OCP did not take'); }
    // the west exit sign, shot out from four metres
    const sign = game.targets.find((t: any) => t.name === 'exit_sign_w');
    game.teleportPlayer([8.5, 0, 11.1]); game.player.crouch = false; game.player.char.bodyYaw = -Math.PI / 2; game.player.holstered = false; steps(W, 0.3);
    if (!sign) fails.push('staging: no exit_sign_w target'); else { fireWeapon(game, game.player.char, sign.pos, true); steps(W, 0.2); if (!sign.broken) fails.push('the round did not break the exit sign'); }
    // a smoke canister into the lobby
    { const pc = game.player.char, fw = pc.forward(), hand = [pc.pos[0] + fw[0] * 0.32, pc.pos[1] + (game.player.crouch ? 1.05 : 1.5), pc.pos[2] + fw[2] * 0.32];   // (where a throw leaves his hand: player.ts kept no function for it when the tick went)
      const sol = game.items.solve(hand, [8.5, 0.1, 15]); doThrow(game, sol, 'smoke'); const thrown = steps(W, 2.0, () => kindsOf(game).includes('thrown')); if (!thrown) fails.push('the canister never left his hand'); }
    // the mission: into the server room, the drive, out through the fire exit
    const M = level.mission;
    for (const lamp of ['server', 'server_glow', 'server_leds']) game.setGroup(lamp, false);   // (the room's lamps light Sam 0.77 under the room's camera, and this run is the one nobody sees)
    game.player.holstered = true; game.teleportPlayer([18.0, 0, 8.9]); steps(W, 0.3);
    if (game.mission.stage !== 'drive') fails.push(`in the server room the stage is ${game.mission.stage} (want drive)`);
    game.teleportPlayer([M.rack.front[0], 0, M.rack.front[2] + 0.65]); game.player.crouch = false; steps(W, 2 * DT);
    W.cursorAt(M.rack.front); steps(W, DT);
    let pressed = false;
    for (let i = 0; i < Math.round((DRIVE_SECS + 2.5) / DT) && !game.mission.pulled; i++) { W.cursorAt(M.rack.front); if (!pressed && game.selectVerb('objective', 'drive')) { W.input.pressed.add('Space'); pressed = true; } W.step(DT); }
    if (!game.mission.pulled) fails.push(`the drive did not come out (pressed ${pressed}, driveT ${f1(game.mission.driveT)}, stage ${game.mission.stage}, Sam ${f1(Math.hypot(game.player.char.pos[0] - M.rack.front[0], game.player.char.pos[2] - M.rack.front[2]))} m off the rack)`);
    game.teleportPlayer([36.9, 0, 20.6]); steps(W, 0.3);
    if (game.mission.stage !== 'done') fails.push(`past the fire exit the stage is ${game.mission.stage} (want done)`);
    if (!game.endCards) fails.push('the game told the host of no end card');
    // ---- the timeline, the numbers, the badges
    const ev = game.chronicle.events as any[]; const kindsGot = kindsOf(game);
    const WANT = ['lockPicked', 'ocp', 'lightShot', 'shot', 'thrown', 'objective', 'objective', 'objective', 'objective'];
    const miss = subseqMiss(kindsGot, WANT); if (miss) fails.push(`timeline kinds [${kindsGot.join(' ')}] — '${miss}' missing / out of order (want ${WANT.join(' › ')})`);
    const objs = ev.filter(e => e.kind === 'objective');
    if (objs.length !== 4 || objs[0].room !== 'server room' || !/server room/.test(objs[0].data.text) || !/hands on/.test(objs[1]?.data.text ?? '') || !/drive is out/.test(objs[2]?.data.text ?? '') || !/fire exit/.test(objs[3]?.data.text ?? '') || objs[3]?.room !== 'east side') fails.push(`objective rows: ${objs.map(o => `${o.room}: ${o.data.text}`).join(' | ')}`);
    { const e = ev.find(x => x.kind === 'lockPicked'); if (e && (e.data.door !== 'server' || e.room !== 'corridor')) fails.push(`lockPicked row ${e.room} ${JSON.stringify(e.data)}`); }
    { const e = ev.find(x => x.kind === 'lightShot'); if (e && (!/exit sign/.test(String(e.data.name)) || e.room !== 'corridor')) fails.push(`lightShot row ${e.room} ${JSON.stringify(e.data)}`); }
    { const e = ev.find(x => x.kind === 'ocp'); if (e && !/corridor ceiling panel/.test(String(e.data.what))) fails.push(`ocp row ${JSON.stringify(e.data)}`); }
    if (ev.some(e => ['contact', 'spotted', 'alarm', 'heard', 'kill', 'ko', 'down'].includes(e.kind))) fails.push(`rows a quiet run cannot have: ${kindsGot.join(' ')}`);
    const D = debriefOf(game); const c = D.counts;
    if (c.picked !== 1 || c.kicked !== 0 || c.ocps !== 1 || c.lightsShot !== 1 || c.thrown !== 1 || c.shots !== 1 || c.seen || c.alarms || c.kills || c.kos) fails.push(`counts: picked ${c.picked} kicked ${c.kicked} ocps ${c.ocps} lights ${c.lightsShot} thrown ${c.thrown} shots ${c.shots} seen ${c.seen} alarms ${c.alarms}`);
    if (D.score !== 100 - SCORE.light || D.rating !== 'GHOST' || D.state !== 'done') fails.push(`score ${D.score} ${D.rating} ${D.state} (want ${100 - SCORE.light} GHOST done)`);
    const on = new Set(D.badges.filter((b: any) => b.on).map((b: any) => b.id));
    for (const [id, want] of [['unseen', true], ['quiet', true], ['nokills', true], ['pacifist', true], ['locksmith', true], ['speedrun', true], ['lightsout', false], ['talkative', false]] as const) if (on.has(id) !== want) fails.push(`badge '${id}' ${on.has(id) ? 'lit' : 'dim'} (want ${want ? 'lit' : 'dim'})`);
    if (!/^twopointsix — GHOST 99 · 00:\d\d · seen 0 · alarms 0 · bodies found 0 · KOs 0 · kills 0 · shots 1 · OCP 1 · doors kicked 0 \/ picked 1 · .*locksmith/.test(D.summary) || /run so far|went down/.test(D.summary)) fails.push(`summary line: "${D.summary}"`);
    if (!/GHOST 99/.test((game.view.mission?.text ?? ''))) fails.push(`objective line once done: "${(game.view.mission?.text ?? '')}"`);
    // nothing more is written once done: a shot now changes neither the rows nor the shots
    { const n0 = ev.length, s0 = game.mission.stats.shots; game.player.holstered = false; steps(W, 0.2); fireWeapon(game, game.player.char, [30, 0.5, 20.6], true); steps(W, 0.6); if (game.chronicle.events.length !== n0 || game.mission.stats.shots !== s0) fails.push(`after 'done' a shot still wrote (rows ${n0} → ${game.chronicle.events.length}, shots ${s0} → ${game.mission.stats.shots})`); }
    notes.push(`timeline (${ev.length}): ${rowsOf(game).map(rowLine).join(' | ')}`);
    notes.push(`→ ${D.rating} ${D.score} in ${f1(c.elapsed)} s · badges [${[...on].join(', ')}] · path ${game.chronicle.pathN} pts · "${D.summary}"`);
    const ok = fails.length === 0; allPass &&= ok;
    verdictLine(ok, '(a2) the quiet run to the end — the server lock picked, a corridor panel OCP\'d, the west exit sign shot out, a smoke canister, into the server room, the drive pulled by its row, out past the fire exit: those rows in order (objectives in the right rooms), 99 GHOST done, never seen / no alarms / no kills / pacifist / locksmith / speedrun lit, the end card raised, and nothing written after', notes.join('\n    '), fails);
    (globalThis as any).__debriefDoneWorld = W;   // (e) builds the card off this finished run
  }
  console.log(`=== debrief-run: ${allPass ? 'ALL PASS' : 'FAILURES above'} ===`);
  return allPass;
}

// ================================================================ (c) rooms
export async function debriefRooms() {
  console.log('\n=== debrief-rooms: roomOf() names known floor points ===');
  const cases: [Vec3, string][] = [
    [[6, 0, 11.1], 'corridor'], [[34, 0, 11.4], 'corridor'], [[8, 0, 18], 'lobby'], [[20, 0, 18], 'cubicles'], [[32, 0, 18], 'break room'],
    [[18, 0, 7], 'server room'], [[8, 0, 7], 'conference room'], [[26, 0, 7], "manager's office"], [[33, 0, 7], 'storage'],
    [[20, 0, 26], 'south lot'], [[38, 0, 15], 'east side'], [[20, 0, 2], 'north side'], [[2, 0, 15], 'west side'],
  ];
  const fails: string[] = [];
  for (const [p, want] of cases) { const got = roomOf(p); if (got !== want) fails.push(`(${p[0]}, ${p[2]}) → '${got}' (want '${want}')`); }
  // the level's own marks agree: the spawn is in the lobby, the rack in the server room, the fire exit's outside step on the east side
  const W = (globalThis as any).__debriefDoneWorld ?? await standUp(); const L = W.level;
  if (roomOf(L.playerSpawn) !== 'lobby') fails.push(`playerSpawn → ${roomOf(L.playerSpawn)}`);
  if (roomOf(L.mission.rack.front) !== 'server room') fails.push(`rack front → ${roomOf(L.mission.rack.front)}`);
  if (roomOf([L.mission.exfilX + 0.2, 0, L.mission.exfilZ]) !== 'east side') fails.push(`exfil → ${roomOf([L.mission.exfilX + 0.2, 0, L.mission.exfilZ])}`);
  const ok = fails.length === 0;
  verdictLine(ok, `(c) ${cases.length} points across the eight rooms and the four sides of the grounds, plus the level's spawn / rack / exfil marks`, cases.map(([p, w]) => `(${p[0]},${p[2]}) ${w}`).join(' · '), fails);
  console.log(`=== debrief-rooms: ${ok ? 'ALL PASS' : 'FAILURES above'} ===`);
  return ok;
}

// ================================================================ (c4) a row that leans loud is not graded as a sneak
/** a game stood on a level file with the list's own row up on it (`search`: the row's address) */
async function standRow(file: string, search: string): Promise<any> {
  const { readFileSync } = await import('node:fs'), { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`), { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`), { startFromQuery } = await import(`${ROOT}/src/game/scenarioRun.ts`);
  const log = console.log, warn = console.warn; console.log = console.warn = () => { };
  try { const W: any = await standUp({ level: loadLevel(parseLevelData(readFileSync(`${ROOT}/levels/${file}`, 'utf8'))), seed: 1 }); const r = startFromQuery(W.game, search); W.why = r.why ?? ''; W.step(DT);   /* (the record is the Rust game's: it stands from the first tick) */ return W; }
  finally { console.log = log; console.warn = warn; }
}
const LOUD_BREAK = process.env.LOUDCARD_BREAK === '1';

// ================================================================ (e) the card, headless
export async function debriefDom() {
  console.log('\n=== debrief-dom: the card builds on a fake DOM off a finished run ===');
  const fails: string[] = []; const notes: string[] = [];
  // ---- a small fake DOM: enough Element for ui/debrief.ts (createElement, className/classList, textContent, append, title, style, onclick, canvas w/h, no 2d context)
  class El {
    nodeType = 1; children: any[] = []; parentElement: El | null = null; style: Record<string, string> = {}; title = ''; width = 0; height = 0; onclick: null | (() => void) = null; onmousemove: unknown = null; _text = '';
    cls = new Set<string>();
    classList = { add: (...c: string[]) => c.forEach(x => this.cls.add(x)), remove: (...c: string[]) => c.forEach(x => this.cls.delete(x)), contains: (c: string) => this.cls.has(c), toggle: (c: string, f?: boolean) => { const on = f ?? !this.cls.has(c); if (on) this.cls.add(c); else this.cls.delete(c); return on; } };
    constructor(public tagName: string) {}
    get className() { return [...this.cls].join(' '); } set className(v: string) { this.cls = new Set(v.split(/\s+/).filter(Boolean)); }
    appendChild(c: any) { this.children.push(c); c.parentElement = this; return c; }
    append(...cs: any[]) { for (const c of cs) this.appendChild(typeof c === 'string' ? Object.assign(new El('#text'), { nodeType: 3, _text: c }) : c); }
    get textContent(): string { return this.nodeType === 3 ? this._text : this._text + this.children.map(c => c.textContent).join(''); }
    set textContent(v: string) { this.children = []; this._text = v; }
    getContext() { return null; } getBoundingClientRect() { return { left: 0, top: 0, width: 400, height: 280 }; }
    focus() {} setAttribute(k: string, v: string) { (this as any)[k] = v; }
    all(sel: (e: El) => boolean, out: El[] = []): El[] { for (const c of this.children) { if (c.nodeType === 1) { if (sel(c)) out.push(c); c.all(sel, out); } } return out; }
  }
  const G = globalThis as any; const hadDoc = 'document' in G; const prevDoc = G.document;
  G.document = { createElement: (t: string) => new El(t.toUpperCase()) };
  let copied = ''; const prevNav = G.navigator;
  try { Object.defineProperty(G, 'navigator', { value: { clipboard: { writeText: async (s: string) => { copied = s; } } }, configurable: true, writable: true }); } catch { /* bun's navigator may be read-only: the button then falls back to showing the line */ }
  try {
    const { renderDebrief, pinnedRow } = await import(`${ROOT}/src/ui/debrief.ts`);
    if (!G.__debriefDoneWorld) await debriefRun();   // (the card off a run with rows on it: alone, this probe would hold an empty card)
    const W = G.__debriefDoneWorld; if (!W) throw new Error('the scripted run left no world'); W.step(DT);   // (the record is asked of the Rust game: a tick stands it again, its slot given up to the probes between)
    const game = W.game;
    const host = new El('DIV'); const btnA = new El('BUTTON'); btnA.textContent = 'Again';
    const ui = renderDebrief(host as any, game, [btnA as any]);
    const root: El = host.children[0];
    if (!root || !root.cls.has('db')) fails.push('no .db root appended to the host');
    const by = (c: string) => root.all(e => e.cls.has(c));
    const D = debriefOf(game);
    { const r = by('db-rating')[0]; const word = r?.children.find((x: El) => x.tagName === 'B')?.textContent; const score = by('db-score')[0]?.textContent; if (word !== D.rating || score !== String(D.score)) fails.push(`header shows ${word} ${score} (want ${D.rating} ${D.score})`); notes.push(`header ${word} ${score}`); }
    { const t1 = by('db-t1')[0]?.textContent; if (t1 !== (D.state === 'done' ? 'mission complete' : D.state === 'down' ? "you're down" : 'run so far')) fails.push(`title '${t1}' for state ${D.state}`); }
    { const { NO_DEBRIEF } = await import(`${ROOT}/src/view/debriefview.ts`), V = game.debriefView(), keys = (o: object) => Object.keys(o).sort().join(' ');   // (the record is read by name and typed `any` on the way: a name the page reads and the game does not send is an empty cell on the card, and no compiler says so)
      for (const [what, got, want] of [['the record', V, NO_DEBRIEF], ['the debrief', V.debrief, NO_DEBRIEF.debrief], ['the counts', V.debrief.counts, NO_DEBRIEF.debrief.counts]] as [string, object, object][]) if (keys(got) !== keys(want)) fails.push(`${what} reads [${keys(got)}], the page's shape says [${keys(want)}]`); if (!V.rows.length) fails.push('the card was built off a run with no rows'); }
    { const n = by('st').length; if (n < 14) fails.push(`stats grid has ${n} cells (want ≥ 14)`); notes.push(`${n} stat cells`); }
    { const chips = by('chip'); const lit = chips.filter(c => c.cls.has('on')).map(c => c.textContent); if (chips.length !== 9) fails.push(`${chips.length} badge chips (want 9: eight + the civilians' unseen-by-staff)`); const wantLit = D.badges.filter((b: any) => b.on).map((b: any) => b.name); if (lit.join('|') !== wantLit.join('|')) fails.push(`lit chips [${lit.join(', ')}] vs badges on [${wantLit.join(', ')}]`); notes.push(`chips lit [${lit.join(', ')}]`); }
    { const rows = by('ev').filter(r => !r.cls.has('empty')); const said = rowsOf(game), n = said.length; if (rows.length !== n) fails.push(`${rows.length} timeline rows for ${n} entries`); const first = rows[0]?.textContent ?? ''; if (n && !first.includes(said[0].text)) fails.push(`first row "${first}" vs "${rowLine(said[0])}"`); notes.push(`${rows.length} timeline rows`); }
    { const pins = by('pin'), marks = root.all(e => e.cls.has('mark') && e.tagName === 'I'); const want = rowsOf(game).filter((e: any) => pinnedRow(e)).length; if (pins.length !== want) fails.push(`${pins.length} pins for ${want} pinned entries`); if (marks.length !== 3) fails.push(`${marks.length} marks (want spawn / rack / exit)`); { const lines = rowsOf(game).filter((e: any) => pinnedRow(e)).map(rowLine).join(' | '), titles = pins.map(p => p.title).join(' | '); if (titles !== lines) fails.push(`the pins' hover lines [${titles}] (want the rows' [${lines}])`); } if (pins.some(p => !/%$/.test(p.style.left ?? '') || !/%$/.test(p.style.top ?? ''))) fails.push('a pin without a percent position'); notes.push(`${pins.length} pins, ${marks.length} marks`); }
    { const cv = root.all(e => e.tagName === 'CANVAS')[0]; if (!cv || cv.width < 100 || cv.height < 60) fails.push(`canvas ${cv?.width}×${cv?.height}`); else notes.push(`canvas ${cv.width}×${cv.height}`); }
    { const btns = by('btn'); const copy = btns.find(b => b.cls.has('db-copy')); if (!root.all(e => e === (btnA as any)).length) fails.push("the host's button was not placed"); if (!copy) fails.push('no copy-summary button'); else { try { copy.onclick?.(); await new Promise(r => setTimeout(r, 5)); } catch (e) { fails.push(`copy button threw: ${e}`); } const V = game.verdict?.result, wantCopy = V && (await import(`${ROOT}/src/ui/verdictCard.ts`)).squadVerdict(V) ? (await import(`${ROOT}/src/game/verdict.ts`)).verdictLine(V, game.verdict.kept) : D.summary; if (ui.copyText !== wantCopy) fails.push('copyText is not the summary line'); if (copied && copied !== wantCopy) fails.push(`clipboard got "${copied}"`); notes.push(`copy → ${copied ? 'clipboard' : 'fallback line'}: "${ui.copyText.slice(0, 60)}…"`); } }
    // and the 'run so far' shape off a fresh world (empty timeline row, sofar title)
    { const W2 = await standUp(); W2.step(DT); const host2 = new El('DIV'); renderDebrief(host2 as any, W2.game); const root2: El = host2.children[0]; const empty = root2.all(e => e.cls.has('empty')); const t1 = root2.all(e => e.cls.has('db-t1'))[0]?.textContent; if (empty.length !== 1 || t1 !== 'run so far') fails.push(`fresh world: ${empty.length} empty-row(s), title '${t1}'`); notes.push(`fresh: '${t1}', ${root2.all(e => e.cls.has('pin')).length} pins`); }
    // and a row that leans loud draws no rating, no score, no docked line, no badges, and its numbers untoned (ui/debrief.ts over Debrief.scored — debrief-loud has the rule)
    {
      { const row4 = LOUD_BREAK ? 'tut-quiet.json' : 'tut-road.json', W4 = await standRow(row4, `?levelFile=${row4}&quiet`), cells = (r: El) => r.all(e => e.cls.has('st') && /^(seen|alarms|floor up|bodies found|kills|doors kicked|called in|checkpoint loads|civilians saw you|…and told them)/.test(e.textContent)), toned = (r: El) => cells(r).filter(e => e.cls.has('good') || e.cls.has('bad') || e.cls.has('warn')).length;
        { const st = W4.game.mission.stats, n = W4.game.chronicle.counts; st.bodies = 1; st.kills = 2; n.kicked = 1; n.devices = 1; n.loads = 1; n.civSeen = 1; n.hits = 1; }   // (a run that would tone every one of them, and one cell that keeps its red: a hit taken)
        const h4 = new El('DIV'); const ui4 = renderDebrief(h4 as any, W4.game); const r4: El = h4.children[0];
        const drawn = ['db-rating', 'db-score', 'db-t3', 'chip'].filter(c => r4.all(e => e.cls.has(c)).length), t2 = r4.all(e => e.cls.has('db-t2'))[0]?.textContent ?? '';
        const hits = (r: El) => r.all(e => e.cls.has('st') && /^hits taken/.test(e.textContent) && e.cls.has('bad')).length;
        if (!r4.cls.has('db-loud') || drawn.length || toned(r4) || cells(r4).length !== 10 || hits(r4) !== 1 || r4.all(e => e.cls.has('db-badges')).length || BANDS.some(w => t2.includes(w) || ui4.copyText.includes(w))) fails.push(`a loud row's card: class ${r4.cls.has('db-loud')}, still drawn [${drawn.join(' ')}], ${toned(r4)} of ${cells(r4).length} sneak cells toned, head "${t2}", copy "${ui4.copyText.slice(0, 40)}"`);
        notes.push(`loud row: head "${t2}"`); } }
  } catch (e: any) { fails.push(`building the card threw: ${e?.stack ?? e}`); }
  finally { if (hadDoc) G.document = prevDoc; else delete G.document; try { Object.defineProperty(G, 'navigator', { value: prevNav, configurable: true, writable: true }); } catch { /* leave it */ } }
  const ok = fails.length === 0;
  verdictLine(ok, '(e) the card off a finished run on a fake DOM: rating + score in the header, the right title, ≥ 14 stat cells, 9 chips lit as the badges are, a timeline row per entry, a pin per pinned entry (titled, percent-placed) + 3 marks, a sized canvas, the host\'s buttons + a working copy button; and a fresh world renders "run so far" with the empty row', notes.join(' · '), fails);
  console.log(`=== debrief-dom: ${ok ? 'ALL PASS' : 'FAILURES above'} ===`);
  return ok;
}
