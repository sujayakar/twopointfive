// DROPS IT AND RUNS — the check behind src/game/runner.ts (RUNNER.on: on in the page, ?norunner; off on the frozen stand, TPS_RUNNER=1 beside TPS_MORALE=1).
// Standalone and through probes.ts:
//   bun run tools/qa/runner-check.ts          bun run tools/qa/probes.ts runner
// morale-check's staging: the compound's room clear, the guard hut's man 2.4 m behind his door in a hut with its lamps put out, the 40 mm's burst on the door —
// he is BROKEN (morale.ts), holds low on the hole; a magazine lobbed through it draws the rest of his own (the bait), and his change begins: the DRY WINDOW.
// The player is then stood in front of him. Where the rest of the garrison is differs by case. Headless, PASS / FAIL each, on sim state:
//   (1) broken, alone, dry, the player in his eyes 2.5 m off: he shouts it — spoken — BEFORE the weapon leaves his hand (no weapon item of his on the floor at
//       the shout, one `beatSecs` later), he is at least two metres from where he dropped it two seconds on, never faster than the guards' chase pace, and
//       not a round leaves his gun from the shout to the end
//   (3) no mate left alive: straight out (never the mates' leg) and off the floor
//   (4) …and with him the last man up, the room clear reads 'taken' while he is still alive and running
//   (7) broken but not dry (the player in his eyes before the bait) · dry, alone, the enemy in his eyes seven metres off: nobody runs
//   (9) every line of the file fits a bubble, names nothing off the blocklist, no compass word, none twice
//   (10) a man of the NIGHT STAFF (the default level's own roster, started through runnerBreak): never taken off the floor, nobody's free man on his round
//        (Guard.hold + pinned: what the chat, the needs and the greetings ask), no torch — and the game's own checkpoint (savestate.ts through the harness's
//        snapshot / restore) brings his run back: fled, the same phase, the same place to tell, still held
//   (11) AFTER GONE: two and a half seconds on from the frame he left the floor, the room clear's attempt, the night breach's staging count, the player
//        object and the head count are what they were (the scenario's "my cast was taken away" test does not restart the floor)
//   (16) his squad (enemysquads.ts) counts him out: the last man of it fled, the squad reads nobody up
//   (17) his shout that he is out has its time before the shout that he is done; from there to the frame he is off the floor nothing of his goes on the squad's net
// (The rest of his rule, each a floor of its own, was cut to keep this under a minute: the corner with no way out, the enemy a metre off, the team's eyes on him,
// a mate at his shoulder, the mates told, a green man who saw his leader fall. RUNNER.anyway is settled: both his guns go down, so one weapon on the floor is
// no longer asked.)
// RUNNER_BREAK=1 = the FLED MARK never sticks (wiped before every frame: the run itself goes on, and every seam that asks `gd.fled` is told no) — the
// counting, the team's eyes, the empty hand, the scenario's cast test and the squad's count all answer as the build without the rule: must fail.
// exit 0 = all hold · 1 = one does not
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
const ROOT = resolve(import.meta.dir, '../..');
type V3 = [number, number, number];
const DT = 1 / 60, f2 = (x: number) => (Number.isFinite(x) ? x.toFixed(2) : String(x));
function seeded(a: number) { return () => { a |= 0; a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ (a >>> 15), 1 | a); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; }; }
const quiet = async <T>(fn: () => Promise<T> | T): Promise<T> => { const log = console.log, err = console.error, warn = console.warn; console.log = console.error = console.warn = () => {}; try { return await fn(); } finally { console.log = log; console.error = err; console.warn = warn; } };

export async function runnerCheck(): Promise<boolean> {
  const { standUp } = (await import('./headless.ts')) as any;
  const { MORALE, moraleOf, rungOf } = await import(`${ROOT}/src/game/morale.ts`);
    const { toggleTarget } = await import(`${ROOT}/src/game/lighting.ts`);
  const { RUNNER, runnerOf, runnerLines, runnerBreak } = await import(`${ROOT}/src/game/runner.ts`);
  const { DOORBLAST } = await import(`${ROOT}/src/game/doorblast.ts`);
  const { pictureTell } = await import(`${ROOT}/src/game/picture.ts`);
  const { FRAG, blast } = await import(`${ROOT}/src/game/frag.ts`);
  const { DESTRUCTION } = await import(`${ROOT}/src/game/destructionSwitch.ts`);
  const { HE40 } = await import(`${ROOT}/src/game/he40.ts`);
  const { doorPt } = await import(`${ROOT}/src/game/doortools.ts`);
  const { killGuard } = await import(`${ROOT}/src/game/combat.ts`);
  const { PLAYER_ID } = await import(`${ROOT}/src/game/consts.ts`);
  const { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`);
  const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
  const { startFromQuery } = await import(`${ROOT}/src/game/scenarioRun.ts`);
  const D = await import(`${ROOT}/src/game/dialogue.ts`);
  const { squadsReport, squadsLive } = await import(`${ROOT}/src/game/enemysquads.ts`);
  const BREAK = process.env.RUNNER_BREAK === '1'; let all = true;
  const was = { runner: JSON.parse(JSON.stringify(RUNNER)), morale: JSON.parse(JSON.stringify(MORALE)), frag: JSON.parse(JSON.stringify(FRAG)), door: DOORBLAST.on, sw: DESTRUCTION.on, rnd: Math.random };
  const line = (ok: boolean, label: string, detail: string) => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); };
  console.log(`\n=== runner: a broken man caught dry drops it and runs${BREAK ? ' (RUNNER_BREAK: the fled mark never sticks)' : ''} ===`);
  const flat = (a: number[], b: number[]) => Math.hypot(a[0] - b[0], a[2] - b[2]);
  /** THE FIRST SLICE'S MEN (MORALE.training 'none': every man folds, fires into the dust and empties his magazine on the bait — under 'difficulty' a steady man
   *  copes trained and is never dry). After every stand-up. Case (12) alone sets a training: its man has a leader to lose */
  const first = () => { MORALE.training = 'none'; MORALE.nerve = 'normal'; };
  /** RUNNER_BREAK: the mark off every man before the frame (and off one already taken off the floor) */
  const wipe = (g: any, also: any[] = []) => { if (BREAK) for (const x of [...g.guards, ...also]) x.fled = false; };

  /** the hut's man broken and holding (morale-check's stage and its lone run), then baited dry: `mates` = where the rest of the
   *  garrison is — parked across the yard in the shed · the two nearest left on their own posts, the rest parked · every one of them down; `squad` = the
   *  player's team staged with him (blind until the case wants one of them); `shoulder` = a mate put two metres from him as his change begins */
  const stage = async (seed: number, mates: 'parked' | 'none' = 'parked', bait = true) => {
    Math.random = seeded(seed); Object.assign(RUNNER, was.runner);
    const W: any = await quiet(() => standUp({ level: loadLevel(parseLevelData(readFileSync(resolve(ROOT, 'levels', 'compound.json'), 'utf8'))) }));
    const g = W.game; DOORBLAST.on = true; DESTRUCTION.on = true; FRAG.doorStun = true; g.tune.chaos = 0.5; first();
    startFromQuery(g, 'levelFile=compound.json&roomclear'); g.godMode = true;
    const gone: any[] = [];
    let vis = 0.05; const step = (n = 1) => { for (let i = 0; i < n; i++) { g.player.visibility = vis; wipe(g, gone); W.step(DT); } };
    step(10);
    const d = g.doors.list.find((x: any) => x.def.name === 'hut_w'); d.reset(); d.angle = 0; d.vel = 0; d.latched = true; d.closing = false; d.place();
    const mid = doorPt(d, d.def.width / 2, 0, 1, 0);
    const men = g.guards.filter((x: any) => x.team === 'hostile' && x.state !== 'dead').sort((a: any, b: any) => flat(a.char.pos, mid) - flat(b.char.pos, mid));
    const T = men[0], ops: any[] = g.nightBreach.ops; gone.push(T);
    const inside: 1 | -1 = g.doors.side(d, T.char.pos), side = -inside as 1 | -1;
    const yard = doorPt(d, d.def.width / 2 + 9, 6, side, 0);
    g.teleportPlayer([yard[0], 0, yard[2]]);   // (the player well off the door's line: nobody of his looks in through the hole)
    for (const o of ops) o.dazzledUntil = 1e9;   // (the team sees nothing until the case that wants one of them)
    const put = (gd: any, p: number[]) => { gd.char.pos = [p[0], 0, p[2]]; gd.char.vel = [0, 0, 0]; gd.path = []; gd.pathGoal = null; gd.speed = 0; gd.lastPos = [p[0], 0, p[2]]; };
    const p = doorPt(d, d.def.width / 2, 2.4, inside, 0);
    men.forEach((o: any, i: number) => { if (o === T) return; if (mates === 'none') killGuard(g, o, [1, 0, 0], true, 'struck', 'guard'); else put(o, [4.8 + (i % 4) * 0.9, 0, 5.8 + Math.floor(i / 4) * 0.9]); });
    put(T, p);
    for (const t of g.targets) if (t.on && flat(t.pos, p) < 5) toggleTarget(g, t);
    step(40);
    const hole = doorPt(d, d.def.width / 2, HE40.standoff + 0.03, side, 1.0) as V3;
    blast(g, [hole[0], hole[1] - FRAG.lift, hole[2]], { who: PLAYER_ID, name: '40 mm grenade' });
    step(Math.round((T.staggerUntil - g.time + 3.5) * 60) + 150);
    const held = moraleOf(T)?.act === 'hold' && rungOf(T) === 2, mag0 = T.weapon.roundsReady;
    /** the weapon items of his on the floor */
    const guns = () => g.items.items.filter((it: any) => it.who === T.char.id && (it.kind === 'rifle' || it.kind === 'pistol'));
    /** the player stood `m` metres in front of him, on his line to the hole, lit */
    const front = (m: number) => { const tp = T.char.pos, l = flat(tp, hole) || 1, at: V3 = [tp[0] + (hole[0] - tp[0]) / l * m, 0, tp[2] + (hole[2] - tp[2]) / l * m]; g.teleportPlayer(at); vis = 1; return at; };
    const away = () => { g.teleportPlayer([yard[0], 0, yard[2]]); vis = 0.05; pictureTell(g, T, 'player', g.player.char.pos); };   // (a player who walked back out would have been WATCHED going: under picture.ts PICTURE.run the man plans by where he believes him, and a teleport is nothing he could have seen)
    let dry = false, outT = NaN;   // (outT: the game clock on the frame his change began — morale.ts's shout that he is out)
    if (bait) {
      const a = doorPt(d, d.def.width / 2, 1.2, side, 1.3), b = doorPt(d, d.def.width / 2, 0, side, 1.3), k = 4 / Math.max(1e-3, flat(a, b));
      g.items.spawn('mag', a, [(b[0] - a[0]) * k, 1.2, (b[2] - a[2]) * k], { who: PLAYER_ID, fill: 1 });
      for (let i = 0; i < 600 && !(T.reloadT >= 0); i++) step();
      dry = T.reloadT >= 0 && T.weapon.roundsReady === 0 && moraleOf(T)?.act === 'reload'; outT = g.time;
    }
    return { W, g, T, hole, step, guns, front, away, held, mag0, dry, outT };
  };
  type Rig = Awaited<ReturnType<typeof stage>>;
  /** step `secs`, noting every new bubble of `who`: [who, text, radio, sim seconds since the call began] */
  const listen = (R: Rig, who: any[], secs: number, each?: (t: number) => boolean | void) => {
    const said: { who: any; text: string; radio: boolean; t: number }[] = [], last = new Map<any, any>(who.map(h => [h, h.bubble?.t]));
    for (let i = 1; i <= Math.round(secs * 60); i++) { R.step(); for (const h of who) { const b = h.bubble; if (b && last.get(h) !== b.t) { last.set(h, b.t); said.push({ who: h, text: b.text, radio: !!b.radio, t: i * DT }); } } if (each?.(i * DT)) break; }
    return said;
  };
  const fired = (gd: any) => gd.weapon.shotsFired;
  const st = (gd: any) => { const S = runnerOf(gd); return S ? `${S.phase}${gd.fled ? ' fled' : ''}` : 'not taken'; };
  /** (11) what a restart would move, and whether `secs` on it is all as it was */
  const floorOf = (g: any) => ({ attempt: g.roomClear.attempt, staged: g.nightBreach.staged, player: g.player, n: g.guards.length });
  const afterGone = (R: { g: any; step: (n?: number) => void }, where: string, went: boolean) => { const g = R.g, a = floorOf(g); R.step(150); const b = floorOf(g);
    line(went && a.attempt === b.attempt && a.staged === b.staged && a.player === b.player && a.n === b.n,
      `(11) after gone, ${where}: 2.5 s on the floor has not been restarted — the same attempt, the same staging, the same player, the same head count`,
      `he left the floor ${went} · attempt ${a.attempt} → ${b.attempt} · stagings ${a.staged} → ${b.staged} · the player object the same ${a.player === b.player} · heads ${a.n} → ${b.n}`); };
  try {
    // (1) (6) (7: seven metres off) (10): the garrison parked across the yard
    { const R = await stage(21), T = R.T, g = R.g, n0 = fired(T);
      R.front(7); R.step(20); const farTaken = !!runnerOf(T), farDry = T.reloadT >= 0;
      const him = R.front(2.5), tp0 = [...T.char.pos];
      let dropAt = NaN, gunsAtSay = NaN, top = 0, dropPos = tp0;
      const said = listen(R, [T], 3.2, t => {
        if (Number.isNaN(gunsAtSay) && runnerOf(T)) gunsAtSay = R.guns().length;
        if (Number.isNaN(dropAt) && T.fled) { dropAt = t; dropPos = [...T.char.pos]; }
        if (T.fled) top = Math.max(top, T.speed);
        return !Number.isNaN(dropAt) && t >= dropAt + 2; });
      const done = said.find(s => runnerLines('done').includes(s.text)), moved = flat(T.char.pos, dropPos);
      line(R.held && R.dry && !!done && done.who === T && !done.radio && gunsAtSay === 0 && dropAt >= done!.t + RUNNER.beatSecs - 2 * DT && dropAt <= done!.t + RUNNER.beatSecs + 3 * DT,
        '(1) he says it, then he does it: the shout — spoken — with the weapon still in his hand, and the drop a beat later',
        done ? `"${done.text}"${done.radio ? ' (RADIO)' : ''} at ${f2(done.t)} s · weapon items of his at the shout ${gunsAtSay} · dropped at ${f2(dropAt)} s (beat ${RUNNER.beatSecs})` : `no such line (held ${R.held}, dry ${R.dry}, ${st(T)}) · said: ${said.map(s => `"${s.text}"`).join(' ') || 'nothing'}`);
      line(moved >= 2 && flat(T.char.pos, him) > flat(dropPos, him) + 1 && top <= g.tune.guardChase + 0.05 && fired(T) === n0 && T.state !== 'dead',
        '(1) he runs — two metres and more from the drop two seconds on, further from the man who broke him, never faster than the guards\' chase pace — and never fires again',
        `${st(T)} · ${f2(moved)} m from the drop · ${f2(flat(dropPos, him))} → ${f2(flat(T.char.pos, him))} m from the player · top speed ${f2(top)} (chase ${g.tune.guardChase}) · rounds since the shout ${fired(T) - n0}`);
      line(!farTaken && farDry, '(7) dry, alone, broken — and the enemy in his eyes seven metres off: he does not run', `taken ${farTaken} · mid-change ${farDry}`);
    }

    // (3) (4) nobody else left alive
    { const R = await stage(23, 'none'), T = R.T, g = R.g, rc0 = g.roomClear.end;
      const up = () => squadsReport(g).map((q: any) => `${q.name} ${q.up}/${q.men}`).join(' · '), ups = () => squadsReport(g).reduce((n: number, q: any) => n + q.up, 0), live = squadsLive(g), up0 = ups(), rep0 = up();
      R.front(2.5); let phases = '', takenAt = NaN, aliveAtTaken = false, runningAtTaken = false, goneAt = NaN, left = false;
      let sayT = NaN, outLine = T.bubble?.text ?? '';
      const said3 = listen(R, [T], 30, t => {
        const S = runnerOf(T); if (S && !phases.endsWith(S.phase)) phases += (phases ? ' → ' : '') + S.phase;
        if (S && Number.isNaN(sayT)) sayT = g.time - DT;   // (the frame of the shout: the tick ran before this frame's clock moved on)
        if (T.fled && !left) { left = true; R.away(); }
        if (Number.isNaN(takenAt) && g.roomClear.end === 'taken') { takenAt = t; aliveAtTaken = T.state !== 'dead' && g.guards.includes(T); runningAtTaken = !!S && T.fled && S.phase !== 'gone'; }
        if (S?.phase === 'gone' || !g.guards.includes(T)) { goneAt = t; return true; } });
      line(R.dry && !phases.includes('mates') && phases.includes('out') && !Number.isNaN(goneAt) && T.state !== 'dead' && !g.guards.includes(T),
        '(3) no mate left: straight out — never the mates\' leg — and off the floor once out and unseen', `${phases || 'never taken'} · off the floor at ${f2(goneAt)} s · state ${T.state}`);
      line(rc0 === null && !Number.isNaN(takenAt) && aliveAtTaken && runningAtTaken && takenAt < goneAt,
        '(4) the room clear reads \'taken\' while the fled man is still alive and running', `end before ${rc0} · 'taken' at ${f2(takenAt)} s (he left the floor at ${f2(goneAt)} s) · alive and on the floor then ${aliveAtTaken} · running ${runningAtTaken} · ${g.roomClear.endWhy ?? ''}`);
      { const iDone = said3.findIndex(x => runnerLines('done').includes(x.text)), after = iDone < 0 ? [] : said3.slice(iDone + 1), net = after.filter(x => x.radio), wait = sayT - R.outT;
        line(R.dry && iDone >= 0 && wait >= RUNNER.outSecs - 2 * DT && wait <= RUNNER.outSecs + 0.4,
          '(17) his shout that he is OUT has its time before the shout that he is done takes its place: `outSecs`, with the enemy in his eyes all the while — then the same beat, the same drop',
          `"${outLine}" as his change began · "${iDone >= 0 ? said3[iDone].text : 'NO SHOUT'}" ${f2(wait)} s later (outSecs ${RUNNER.outSecs})`);
        line(live && iDone >= 0 && net.length === 0 && !Number.isNaN(goneAt),
          '(17) with his squad staged: from the shout to the frame he is off the floor he puts nothing on the squad\'s net — no contact call takes his own words\' place',
          `squads staged ${live} · lines of his after the shout: ${after.map(x => `"${x.text}"${x.radio ? ' (RADIO)' : ''}`).join(' ') || 'none'} · off the floor at ${f2(goneAt)} s`); }
      line(live && up0 === 1 && ups() === 0, '(16) his squad counts him out: the last man of it up before the shout, nobody up once he has fled', `squads staged ${live} · before: ${rep0} · after: ${up()}`);
      afterGone(R, 'the room clear with nobody else alive', !Number.isNaN(goneAt) && !g.guards.includes(T)); }

    // (10) a man of the night staff, and the game's own checkpoint
    { Object.assign(RUNNER, was.runner); Math.random = seeded(41);
      const W: any = await quiet(() => standUp({})); const g = W.game; first(); g.godMode = true;
      const step = (n = 1) => { for (let i = 0; i < n; i++) { g.player.visibility = 0.05; wipe(g); W.step(DT); } };
      step(30);
      const staff = g.guards.filter((x: any) => x.team === 'guard' && x.state !== 'dead'), T = staff[0], idx = g.guards.indexOf(T), n0 = g.guards.length, units = g.guards.filter((x: any) => x.unit).length;
      const took = !!T && runnerBreak(g, T); let phases = '';
      for (let i = 0; i < 60 * 25; i++) { step(); const S = runnerOf(T) as any; if (S && !phases.endsWith(S.phase)) phases += (phases ? ' → ' : '') + S.phase; if (S?.phase === 'low' && S.script.arrived && i > 120) break; }
      const guns = g.items.items.filter((it: any) => it.who === T.char.id && (it.kind === 'rifle' || it.kind === 'pistol')).length;
      line(took && units === 0 && T.fled && g.guards.includes(T) && g.guards.length === n0 && T.hold === true && T.pinned === true && T.state === 'patrol' && !T.flashlight?.enabled && guns === 1,
        '(10) a man of the night staff: his weapon on the floor, never taken off it himself, nobody\'s free man on his round (held and pinned: no chat, no need, no greeting is his), no torch',
        `started ${took} · ${phases || 'never taken'} · on the floor ${g.guards.includes(T)} (${g.guards.length} of ${n0}) · hold ${T.hold} pinned ${T.pinned} state ${T.state} · light ${!!T.flashlight?.enabled} · weapon items of his ${guns}`);
      let snapWhat = 'never asked', snapOk = false;
      try { const S0 = runnerOf(T) as any, was0 = S0 ? { phase: S0.phase, fled: T.fled, from: [...S0.from], tries: S0.tries } : null;
        const snap = W.snapshot('runner-check', true), keyed = JSON.stringify(snap).includes('"runner"');
        step(90); W.restore(snap); const T1 = g.guards[idx], S1 = runnerOf(T1) as any, fledAtLoad = T1.fled; step(2);
        snapOk = !!was0 && was0.fled && !!S1 && S1.phase === was0.phase && T1.fled === true && fledAtLoad && flat(S1.from, was0.from) < 1e-6 && S1.tries === was0.tries && T1.hold === true && T1.pinned === true && T1.weapon.roundsReady === 0;
        snapWhat = `saved ${was0 ? `${was0.phase}${was0.fled ? ' fled' : ''}` : 'nothing'} · the slice in the file ${keyed} · loaded ${st(T1)} · the place to tell ${S1 ? f2(flat(S1.from, was0?.from ?? [0, 0, 0])) : '?'} m off · two frames on: hold ${T1.hold} pinned ${T1.pinned} rounds ready ${T1.weapon.roundsReady}`;
      } catch (e) { snapWhat = `the checkpoint threw: ${e instanceof Error ? e.message : String(e)}`; }
      line(snapOk, '(10) the game\'s own checkpoint (savestate.ts): his run comes back fled, the same phase, the same place to tell, still held, still empty-handed', snapWhat); }
    // (7) not dry · not alone
    { const R = await stage(26, 'parked', false), T = R.T; R.front(2.5); let taken = 0, wet = 0;
      for (let i = 0; i < 20; i++) { R.step(); if (T.weapon.roundsReady > 0 && T.reloadT < 0) { wet++; if (runnerOf(T)) taken++; } }
      line(R.held && R.mag0 >= 12 && wet >= 10 && taken === 0, '(7) broken, alone, the player in his eyes 2.5 m off — but rounds in his gun: he does not run', `held on the hole ${R.held} with ${R.mag0} in the gun · frames with rounds in it ${wet} of 20 · frames taken ${taken}`); }
    // (9) the words
    { const lines: string[] = runnerLines(), long = lines.filter(t => t.length > D.AIM_CHARS), blocked = lines.filter(t => D.blockedPhrase(t)), compass = lines.filter(t => /\b(north|south|east|west)/i.test(t)), twice = lines.filter((t, i) => lines.indexOf(t) !== i), tight = lines.filter(t => /\S—\s|\s—\S/.test(t));
      line(lines.length >= 6 && !long.length && !blocked.length && !compass.length && !twice.length && !tight.length && runnerLines('done').length >= 3 && runnerLines('mates').length >= 3,
        `(9) every line fits a bubble (≤ ${D.AIM_CHARS}), none on the originality blocklist, no compass word, none twice`, `${lines.length} lines · too long: ${long.join(' | ') || 'none'} · blocked: ${blocked.join(' | ') || 'none'} · twice: ${twice.join(' | ') || 'none'}`); }

  } catch (e) { all = false; console.log(`- FAIL — the check threw\n    ${e instanceof Error ? e.stack : String(e)}`); }
  finally { Object.assign(RUNNER, was.runner); Object.assign(MORALE, was.morale); Object.assign(FRAG, was.frag); DOORBLAST.on = was.door; DESTRUCTION.on = was.sw; Math.random = was.rnd; }
  console.log(`=== runner: ${all ? 'ALL PASS' : 'FAILURES above'} ===`);
  return all;
}

if (import.meta.main) process.exit((await runnerCheck()) ? 0 : 1);
