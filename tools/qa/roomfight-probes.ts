// The room fight (src/game/roomfight.ts): both sides of a room clear fighting on their own read. `bun run tools/qa/probes.ts roomfight` = the family.
//
//   roomfight-entry     the same seed, the same three men in the break room, the noise at a different door: they end up watching — and with their guns on — the
//                       door it came from, not the one a plan names; a door merely TRIED by a teammate is heard and watched; a bang moves the watch
//   roomfight-flank     one defender who has had a door kicked: a teammate through THAT door meets a man with his gun on it, a teammate through the room's other
//                       door meets his shoulder — sixteen seeds each: the unwatched entry drops him without a round coming back far more often
//   roomfight-nextroom  a defender posted in the corridor outside: with a fight in the break room he can hear (real rounds) he comes; with none he stands; with
//                       gunfire where his side has nobody he stays too
//   roomfight-sight     a teammate fires at a defender he has a lit, smoke-free sight of — and not at a lit one through smoke (the
//                       renderer's answer planted on the pair's key), nor at one behind his back
//   roomfight-eyes      ROOMFIGHT.sights: a teammate with his rifle in the shoulder sees every man along his aim with a clear line; the entry dip and its
//                       eyes (ROOMFIGHT_EYES_BREAK=1 = the red path: the dip's eyes are left down)
//   roomfight-friends   the night breach at its default over fifteen seeds with Sam idle, and eight with Sam (god mode) walked into the room behind the charge:
//                       no teammate's round passes within a body's breadth of Sam or of the other teammate; and nothing in those runs was staged — the drill
//                       fired no round of its own (BreachDrill.rounds 0, no eyes / shot / guaranteed beat), every defender down was dropped by a round
//   roomfight-duel      the even meeting: one teammate, one defender, both roused, face to face in the same light, twenty-four seeds — the teammate wins
//                       clearly more often than not, and the defender still wins some (lethality is the same both ways)
// Headless there is no renderer: the light on a man is the CPU estimate of the direct light (guards.ts lightEstimateAt), the smoke between two men is clear unless
// a probe plants the segment's answer, and nothing here says anything about what the picture shows.
// 2026-11-07, SETTLED SWITCHES ARE NOT WRITTEN HERE. Gone: roomfight-off whole (ROOMFIGHT.on false); of roomfight-eyes the lines with ROOMFIGHT.sights off or
// sightsFoe on ('switch off (as before)', 'nothing he saw before is lost', 'his reading of Sam is the same with the switch on and off', and both lines of the
// other side's eye); of roomfight-sight the dark run ('the room dark and no weapon light: not a round' — the bare-eyes rule, asked with the goggles put off:
// nvg.ts took it away). The two rig pins are out as well: where a probe has a lone teammate open fire by himself, the fight is BEGUN by the hold-fire rule's
// own stamp (holdfire.ts holdFireDrill) where the rule was written off; and roomfight-nextroom no longer has rounds stop in the door leaf (WALLPEN.on).
import { standUp, ROOT } from './headless.ts';
import { sample } from './sweep.ts';
const { v3 } = await import(`${ROOT}/src/math/vec.ts`);
const { ROOMFIGHT, FIGHT_SKILL } = await import(`${ROOT}/src/game/roomfight.ts`);
const { NIGHTBREACH } = await import(`${ROOT}/src/game/nightbreach.ts`);
const { spawnUnit, engage, canSeeChar } = await import(`${ROOT}/src/game/guards.ts`);
const { order } = await import(`${ROOT}/src/game/squad.ts`);
const { fireWeapon, detonateStun } = await import(`${ROOT}/src/game/combat.ts`);
const { holdFireDrill } = await import(`${ROOT}/src/game/holdfire.ts`);

const DT = 1 / 60;
function seeded(a: number) { return () => { a |= 0; a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ (a >>> 15), 1 | a); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; }; }
const f2 = (x: number) => (Number.isFinite(x) ? x.toFixed(2) : String(x));
type Verdict = (ok: boolean, label: string, detail: string) => void;
function family(title: string): { verdict: Verdict; done: () => boolean } {
  console.log(`\n=== ${title} ===`); let all = true;
  return { verdict: (ok, label, detail) => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); }, done: () => { console.log(`=== ${title.split(':')[0]}: ${all ? 'ALL PASS' : 'FAILURES above'} ===`); return all; } };
}
const KNOBS0 = { ...ROOMFIGHT }, SKILL0 = JSON.parse(JSON.stringify(FIGHT_SKILL, (_k, v) => v === Infinity ? '∞' : v), (_k, v) => v === '∞' ? Infinity : v), NB0 = { ...NIGHTBREACH };
const restore = () => { Object.assign(ROOMFIGHT, KNOBS0); for (const k of ['operative', 'hostile'] as const) Object.assign(FIGHT_SKILL[k], JSON.parse(JSON.stringify(SKILL0[k], (_k, v) => v === Infinity ? '∞' : v), (_k, v) => v === '∞' ? Infinity : v)); Object.assign(NIGHTBREACH, NB0); };
/** a probe whose staging has teammates open fire by themselves: floor() begins the fight by the hold-fire rule's own stamp after each stand-up (the squad is
 *  no longer unfound: holdfire.ts holdFireDrill). It put the rule off until 2026-11-07 */
let noHold = false;
const guarded = (fn: () => Promise<boolean>) => async () => { try { return await fn(); } finally { restore(); noHold = false; } };
const wrap = (a: number) => { a = (a + Math.PI) % (2 * Math.PI); if (a <= 0) a += 2 * Math.PI; return a - Math.PI; };
const up = (l: any[]) => l.filter(x => x.state !== 'dead').length;

interface Shot { who: number; team: string; from: number[]; to: number[]; t: number; blind: boolean }
/** what every rig's step keeps: rounds by side (off the 'guardShot' events), each one's flight, and the nearest any OPERATIVE round's flight came to a friend's body */
function tracker(W: any) {
  const g = W.game; const seen = new Set<any>();
  const T = { shots: [] as Shot[], by: {} as Record<string, number>, friendMin: Infinity, friendWho: '', samHit: false, bad: '',
    scan() {
      for (const e of g.events) {
        if (seen.has(e)) continue; seen.add(e);
        if (e.kind !== 'guardShot' || !e.from || !e.to) continue;
        const w = g.guards.find((x: any) => x.char.id === e.who); const team = w ? w.team : '?';
        T.shots.push({ who: e.who, team, from: [...e.from], to: [...e.to], t: e.time, blind: !!e.blind }); T.by[team] = (T.by[team] ?? 0) + 1;
        if (team !== 'operative') continue;
        // a friend's body against the flight, in plan, where the round is between his soles and his crown
        const friends: [string, any][] = [['Sam', g.player.char], ...g.guards.filter((x: any) => x.team === 'operative' && x !== w && x.state !== 'dead').map((x: any) => [x.callsign, x.char] as [string, any])];
        for (const [name, ch] of friends) {
          if (name === 'Sam' && g.player.down) continue;
          const h = ch.bones.hips ?? ch.pos; const dx = e.to[0] - e.from[0], dz = e.to[2] - e.from[2]; const L2 = dx * dx + dz * dz; if (L2 < 1e-6) continue;
          const t = Math.max(0, Math.min(1, ((h[0] - e.from[0]) * dx + (h[2] - e.from[2]) * dz) / L2));
          const y = e.from[1] + (e.to[1] - e.from[1]) * t; if (y < ch.pos[1] - 0.05 || y > ch.pos[1] + 1.9) continue;
          const d = Math.hypot(e.from[0] + dx * t - h[0], e.from[2] + dz * t - h[2]);
          if (d < T.friendMin) { T.friendMin = d; T.friendWho = `${w?.callsign} past ${name} at ${f2(e.time)} s`; }
        }
      }
      if (g.player.down || g.player.hitFlash > 0) T.samHit = true;
      for (const x of g.guards) if (!T.bad && !x.char.pos.every(Number.isFinite)) T.bad = `${x.callsign} at ${x.char.pos}`;
    } };
  return T;
}

/** A floor of our own: the night game stood up under a seed, the shift benched as the night breach benches it (off the list, torches out of the light set), no
 *  civilians / response team / devices, a fresh encounter — then whoever the probe spawns. Sam is parked dark at his spawn in the cubicles. */
async function floor(seed: number, o: { lit?: boolean; chaos?: number } = {}) {
  Math.random = seeded(seed);
  const W: any = await standUp(); const g = W.game; if (noHold) holdFireDrill(g);
  for (const gd of g.guards) { gd.flashlight.enabled = false; g.engine.lights.remove(gd.flashlight); }
  g.guards = []; g.tune.civilians = false; g.tune.qrf = false; g.tune.devices = false; g.tune.chaos = o.chaos ?? 0;
  g.restartEncounter();
  for (const n of ['breakroom_w', 'breakroom_n', 'breakroom_ext', 'fire_exit']) { const d = g.doors.byName(n); if (d) { d.reset(); d.angle = 0; d.vel = 0; d.latched = true; d.closing = false; } }   // the room's doors shut (the night floor authors the west one ajar; the night breach shuts it the same way)
  if (o.lit) g.setGroup('breakroom', true);
  const T = tracker(W);
  const R = { W, g, T, vis: 0.05, plant: null as null | (() => void),
    spawn(team: 'operative' | 'hostile', pos: number[], facing: number[], opts: { light?: boolean; name?: string } = {}) { return spawnUnit(g, { team, pos: [pos[0], 0, pos[2]], yaw: v3.yawTo([pos[0], 0, pos[2]], [facing[0], 0, facing[2]]), light: opts.light ?? false, name: opts.name }); },
    step(n = 1) { for (let i = 0; i < n; i++) { g.player.visibility = R.vis; R.plant?.(); W.step(DT); T.scan(); } },
    until(pred: () => boolean, secs: number) { for (let i = 0; i < secs * 60 && !pred(); i++) R.step(); return pred(); },
    door(name: string) { const d = g.doors.byName(name); if (!d) throw new Error(`no door ${name}`); return d; },
    kick(name: string, who: any) { const d = R.door(name); g.events.push({ kind: 'kick', pos: [d.frameCentre[0], 0, d.frameCentre[2]], time: g.time, loud: true, level: 1.0, who: who.char.id }); },
  };
  return R;
}
/** the night breach's own staging under a seed (its rig, cut down): the tracker on it, F presses, Sam kept dark unless the probe says otherwise */
async function breach(seed: number, knobs: Record<string, unknown> = {}, chaos = 0.5) {
  Object.assign(NIGHTBREACH, NB0, knobs); Math.random = seeded(seed);
  const W: any = await standUp(); const g = W.game; const NB = g.nightBreach; g.tune.chaos = chaos;
  const why = NB.start(); if (why) throw new Error(`night breach refused: ${why}`);
  const T = tracker(W);
  const R = { W, g, NB, T, vis: 0.05, each: null as null | (() => void),
    step(n = 1) { for (let i = 0; i < n; i++) { g.player.visibility = R.vis; R.each?.(); W.step(DT); T.scan(); } },
    press(code = 'KeyF') { W.input.pressed.add(code); R.step(); },
    until(pred: () => boolean, secs: number) { for (let i = 0; i < secs * 60 && !pred(); i++) R.step(); return pred(); },
    beat(what: string): number { const b = NB.drill?.beats.find((x: any) => x.what === what); return b ? b.t : NaN; } };
  return R;
}
const MARKS = (g: any) => g.level.breach.hostiles as { pos: number[]; aimAt: number[] }[];
const aimOff = (gd: any, at: number[]) => Math.abs(wrap(v3.yawTo(gd.char.pos, [at[0], 0, at[2]]) - gd.char.aimYaw)) * 57.3;

// ================================================================ entry: the noise decides what they watch
export const roomfightEntry = guarded(async () => {
  const { verdict, done } = family('roomfight-entry: the same men, the same seed, a different door — a different room to walk into');
  const OUT: Record<string, number[]> = { breakroom_w: [26.4, 0, 15.4], breakroom_n: [28.4, 0, 11.1] };   // outside each door and well to one side of it: behind the wall, out of the leaf's reach (a man stood AT a door has it open)
  const run = async (doorName: string, how: 'kick' | 'try' | 'bang') => {
    const R = await floor(31); const { g } = R; const F = g.fight;
    const defs = MARKS(g).map((m, i) => R.spawn('hostile', m.pos, m.aimAt, { name: `Tango ${i + 1}` }));
    const op = R.spawn('operative', OUT[doorName], R.door(doorName).frameCentre, { name: 'Blue One' }); order(op, { goal: null, upper: 'none', stance: 'lowReady' });
    R.step(90);   // calm: they stand as they were put
    const calm = defs.map((d: any) => F.watchOf(d)?.door ?? 'front');
    if (how === 'kick') R.kick(doorName, op);
    else if (how === 'try') { const d = R.door(doorName); g.events.push({ kind: 'door', pos: [d.frameCentre[0], 0, d.frameCentre[2]], time: g.time, loud: false, level: 0.3, who: op.char.id }); }   // the handle tried from outside: doors.ts's own 'rattle' event, blamed on him
    else { const d = R.door(doorName); const it = g.items.spawn('flash', [d.frameCentre[0], 0.25, d.frameCentre[2] + (doorName === 'breakroom_n' ? 0.9 : 0)], [0, 0, 0], { fuse: 9, heading: 0 }); it.fired = true; F.blindFire = false; FIGHT_SKILL.hostile.bangBreak = 0; detonateStun(g, it); }
    if (how === 'bang') { R.step(60); for (let i = 0; i < 60 * 14 && defs.some((d: any) => g.time < d.dazzledUntil + 2); i++) R.step(); }   // (until every man has had his sight back two seconds: the canister's blindness is combat.ts FLASH's now — up to seven seconds — where a fixed seven-second wait caught the nearest man still blind)
    else R.step(60 * 3);
    return { R, g, defs, op, calm, watch: defs.map((d: any) => F.watchOf(d)?.settled ?? 'front'), glancing: defs.filter((d: any) => F.watchOf(d)?.glancing).length, yaw: defs.map((d: any) => d.char.aimYaw as number), off: defs.map((d: any) => aimOff(d, R.door(doorName).frameCentre)), state: defs.map((d: any) => d.state as string), shut: Math.abs(R.door(doorName).angle) < 0.05 };
  };
  const W = await run('breakroom_w', 'kick'), N = await run('breakroom_n', 'kick');
  const turned = W.yaw.map((y: number, i: number) => Math.abs(wrap(y - N.yaw[i])) * 57.3);
  verdict(W.calm.every((c: string) => c === 'front') && N.calm.every((c: string) => c === 'front'), 'calm, nobody watches a door: each stands the way he was put', `west run ${W.calm.join(' ')} · north run ${N.calm.join(' ')}`);
  verdict(W.watch.every((w: string) => w === 'breakroom_w') && N.watch.every((w: string) => w === 'breakroom_n'), 'a boot on a door: every man watches THAT door (his own ears put it there)', `kick west → ${W.watch.join(' ')} · kick north → ${N.watch.join(' ')} · states ${W.state.join('/')} · ${N.state.join('/')}`);
  verdict(W.off.every((o: number) => o < 25) && N.off.every((o: number) => o < 25) && turned.filter((t: number) => t > 40).length >= 2, 'and their guns are on it: the same seed, the two doors — at least two men end up facing more than 40° apart',
    `aim off the kicked door: west ${W.off.map((o: number) => o.toFixed(0) + '°').join(' ')} · north ${N.off.map((o: number) => o.toFixed(0) + '°').join(' ')} · between the runs ${turned.map((t: number) => t.toFixed(0) + '°').join(' ')}`);
  const T = await run('breakroom_n', 'try');
  verdict(T.watch.filter((w: string) => w === 'breakroom_n').length >= 2 && T.state.every((s: string) => s !== 'dead'), 'a door only TRIED by a teammate (the handle, no boot) is heard: they come round to it', `watch ${T.watch.join(' ')} · states ${T.state.join('/')} · aim off it ${T.off.map((o: number) => o.toFixed(0) + '°').join(' ')}`);
  const B = await run('breakroom_n', 'bang');
  // (it was all three: the canister's blindness is combat.ts FLASH's now, up to seven seconds, and the man blind longest comes back to a noise that old with two
  //  mates already on that door — ROOMFIGHT.crowd leaves him free for another. With crowd 0 all three settle on it. Nobody settles on a DIFFERENT door)
  verdict(B.watch.filter((w: string) => w === 'breakroom_n').length >= 2 && B.watch.every((w: string) => w === 'breakroom_n' || w === 'front'), 'a bang at the north door (no blind fire, nobody through it): once they can see again they are watching it — at least two of the three, the third on it or left free by the two (never settled on another door)', `the entry each fears ${B.watch.join(' ')} · eyes wandering off it at this moment: ${B.glancing} of 3 (FIGHT_SKILL.hostile.hold / glance) · aim off it ${B.off.map((o: number) => o.toFixed(0) + '°').join(' ')}`);
  return done();
});

// ================================================================ flank: the door he is not watching
export const roomfightFlank = guarded(async () => {
  const { verdict, done } = family('roomfight-flank: a defender watching one door is caught out through the other');
  const SEEDS = sample(Array.from({ length: 16 }, (_, i) => 41 + i), 4);
  const one = async (seed: number, via: 'breakroom_w' | 'breakroom_n') => {
    const R = await floor(seed); const { g } = R; const m = MARKS(g)[0];
    const def = R.spawn('hostile', m.pos, m.aimAt, { name: 'Tango 1' });
    const from = via === 'breakroom_w' ? [24.4, 0, 17.6] : [26.6, 0, 11.1], goal = via === 'breakroom_w' ? [29.6, 0, 17.6] : [30.6, 0, 14.2];   // each starts some five metres of walking from his door, out of sight of the room
    const op = R.spawn('operative', from, R.door(via).frameCentre, { name: 'Blue One', light: true });
    order(op, { goal: null, upper: 'none', stance: 'lowReady' }); R.step(60);
    R.kick('breakroom_w', op); R.step(30); R.kick('breakroom_w', op); R.step(120);   // the west door booted twice and holding: that is the door he fears
    const watch = g.fight.watchOf(def)?.door;
    order(op, { goal: [goal[0], 0, goal[2]], speed: 2.6, upper: 'aim', stance: 'none', aimAt: [32.0, 1.2, 18.0], light: true, hot: true });   // through his door at a walk, his muzzle (and his light) on the middle of the room as the drill's centre check has it — the same order either way
    const t0 = g.time; R.until(() => def.state === 'dead' || op.state === 'dead', 14); R.step(30);
    const back = R.T.shots.filter(s => s.team === 'hostile' && s.t >= t0).length;
    return { watch, defDown: def.state === 'dead', opDown: op.state === 'dead', back, surprised: g.fight.stats.surprised, bad: R.T.bad };
  };
  const tally = async (via: 'breakroom_w' | 'breakroom_n') => {
    let defDown = 0, opDown = 0, clean = 0, back = 0, surprised = 0, watched = 0, bad = ''; const rows: string[] = [];
    for (const seed of SEEDS) { const o = await one(seed, via); if (o.defDown) defDown++; if (o.opDown) opDown++; if (o.defDown && !o.opDown && o.back === 0) clean++; back += o.back; surprised += o.surprised; if (o.watch === 'breakroom_w') watched++; if (o.bad) bad = o.bad;
      rows.push(`${seed}:${o.defDown ? 'D' : '-'}${o.opDown ? 'O' : '-'}${o.back}`); }
    return { defDown, opDown, clean, back, surprised, watched, bad, rows };
  };
  const Wt = await tally('breakroom_w'), Nt = await tally('breakroom_n'); const n = SEEDS.length;
  verdict(Wt.watched === n && Nt.watched === n && !Wt.bad && !Nt.bad, 'in every run he is watching the west door when the teammate comes (two boots on it)', `watching west: ${Wt.watched} + ${Nt.watched} of ${2 * n}`);
  verdict(Nt.clean > Wt.clean && Nt.clean >= n * 0.75 && Nt.opDown < Wt.opDown + 1 && Nt.back < Wt.back, 'through the door he is NOT watching he is dropped without a round coming back far more often than through the one he is',
    `north (unwatched): defender down ${Nt.defDown}/${n}, clean (no round back) ${Nt.clean}, teammate lost ${Nt.opDown}, his rounds ${Nt.back}, caught looking away ${Nt.surprised}\n    west (watched):    defender down ${Wt.defDown}/${n}, clean ${Wt.clean}, teammate lost ${Wt.opDown}, his rounds ${Wt.back}, caught looking away ${Wt.surprised}\n    per seed (D defender down · O teammate down · rounds back) north ${Nt.rows.join(' ')}\n    west  ${Wt.rows.join(' ')}`);
  verdict(Wt.back > 0, 'and the watched door is no walkover: the man with his gun on it gets rounds off at the teammate in it', `rounds back through the watched door over ${n} runs: ${Wt.back} · teammates lost ${Wt.opDown}`);
  return done();
});





// ================================================================ duel: the even meeting
export const roomfightDuel = guarded(async () => {
  const { verdict, done } = family('roomfight-duel: one teammate, one defender, both roused, the same light — the trained man wins most, not all');
  const A = [29.6, 0, 20.6], B = [34.4, 0, 20.6]; const SEEDS = Array.from({ length: 24 }, (_, i) => 81 + i);
  let opWins = 0, defWins = 0, both = 0, none = 0; const firstOp: number[] = [], firstDef: number[] = []; let lo = '', bad = '';
  for (const seed of SEEDS) {
    const R = await floor(seed, { lit: true }); const { g } = R; g.fight.blindFire = false;
    const h = R.spawn('hostile', B, A, { name: 'Tango 1' }); const op = R.spawn('operative', A, B, { name: 'Blue One' });
    order(op, { goal: null, upper: 'aim', stance: 'none', aimAt: [B[0], 1.2, B[2]] });
    h.state = 'alert'; h.awareness = 1; h.lastKnown = [A[0], 0, A[2]]; g.fight.hotT = 0.5;   // both roused (the defender alert; the floor 'hot' for the teammate, as after any shot) and looking down the same line at each other…
    R.plant = () => { if (g.time < 0.5) { h.dazzledUntil = g.time + 0.02; op.dazzledUntil = g.time + 0.02; } };   // …and neither sees the other for the first half second, so both start on the same frame
    FIGHT_SKILL.hostile.bangBreak = 0; FIGHT_SKILL.hostile.daze = [0, 0];   // (the half second above is this probe's blindfold, not a flash: no daze behind it — a bang's daze is the door tools' business, doortools-firstman)
    R.until(() => h.state === 'dead' || op.state === 'dead', 12); R.step(20);
    const o = op.state === 'dead', d = h.state === 'dead'; if (d && !o) opWins++; else if (o && !d) defWins++; else if (o && d) both++; else none++;
    const fo = R.T.shots.find(s => s.team === 'operative'), fd = R.T.shots.find(s => s.team === 'hostile'); if (fo) firstOp.push(fo.t - 0.5); if (fd) firstDef.push(fd.t - 0.5);
    lo = `light on the teammate ${f2(op.visibility)} · on the defender ${f2(h.visibility)}`; if (R.T.bad) bad = R.T.bad;
  }
  const mean = (a: number[]) => a.length ? a.reduce((x, y) => x + y, 0) / a.length : NaN; const n = SEEDS.length;
  verdict(opWins >= n * 0.65 && !bad, `the teammate wins clearly more often than not (≥ 65 % of ${n})`, `teammate ${opWins} · defender ${defWins} · both down ${both} · neither ${none} · ${lo}`);
  verdict(defWins + both >= 1 && none === 0, 'and it is still a fight with live rounds both ways: the defender takes him in some runs, and every run ends with a man down', `defender won or traded in ${defWins + both} of ${n} · undecided ${none}`);
  verdict(firstOp.length > firstDef.length && mean(firstOp) < 0.8, 'the gap is in the skill table: the teammate gets the first round off in most runs, inside a second', `a first round after they can see — teammate: ${firstOp.length} runs, mean ${f2(mean(firstOp))} s · defender: ${firstDef.length} runs, mean ${f2(mean(firstDef))} s (whoever is hit first fires none) · FIGHT_SKILL operative react ${FIGHT_SKILL.operative.react} settleK ${FIGHT_SKILL.operative.settleK} spreadK ${FIGHT_SKILL.operative.spreadK} · hostile react ${FIGHT_SKILL.hostile.react} settleK ${FIGHT_SKILL.hostile.settleK} spreadK ${FIGHT_SKILL.hostile.spreadK}`);
  return done();
});

// ================================================================ orders: what the module borrows it gives back, and whom it leaves alone
export const roomfightOrders = guarded(async () => {
  const { verdict, done } = family('roomfight-orders: an order under an engagement is never written over; a man let off his post is left alone');
  noHold = true;   // (each rig stages a lone teammate who opens fire by himself on men who have not noticed him — their blindness and his are planted, no can is heard — with no shot or order of Sam's: the unfound squad's hold, holdfire.ts, which the shipped stand raises, would hold every round; who fires first is holdfire-check's question)
  const A = [29.6, 0, 20.6], B = [34.4, 0, 20.6];
  { // the order's own aim / carriage / facing while his muzzle is his own
    const R = await floor(91, { lit: true }); const { g } = R; g.fight.blindFire = false; FIGHT_SKILL.hostile.bangBreak = 0;
    const h = R.spawn('hostile', B, A, { name: 'Tango 1' }); const op = R.spawn('operative', A, [A[0], 0, A[2] - 4], { name: 'Blue One' });
    const X = [A[0], 1.2, A[2] - 4];
    order(op, { goal: null, upper: 'none', stance: 'lowReady', aimAt: X, face: Math.PI });   // gun down, looking away from the man (he is 90° off: inside the cone's near reach? no — five metres: outside it)
    R.plant = () => { h.dazzledUntil = g.time + 5; }; R.step(30);
    const quiet = (R.T.by.operative ?? 0) === 0 && !op.script.over;
    order(op, { aimAt: [B[0], 1.2, B[2]] }); let during: any = null;   // the order turns his eyes to the man: he sees him and his muzzle becomes his own
    R.until(() => { if (op.script?.over && !during) during = { upper: op.script.upper, stance: op.script.stance, face: op.script.face, anim: op.char.anim.upper, drawn: op.drawn }; return h.state === 'dead'; }, 6);
    R.step(90);
    verdict(quiet && !!during && during.upper === 'none' && during.stance === 'lowReady' && during.face === Math.PI && during.anim === 'aim' && during.drawn,
      'while he engages, the order on his script is exactly as ordered (the gun comes up through the overlay, not by rewriting it)', `before he saw him: no round, no overlay ${quiet} · during: script upper ${during?.upper} stance ${during?.stance} face ${f2(during?.face)} · shown upper ${during?.anim} drawn ${during?.drawn}`);
  }
  { // guards.ts engage() lets a held defender off his post: this module does not put him back on it
    const R = await floor(93, { lit: true }); const { g } = R; g.fight.blindFire = false;
    const h = R.spawn('hostile', B, A, { name: 'Tango 1' }); const op = R.spawn('operative', [29.0, 0, 13.4], [29.0, 0, 9], { name: 'Blue One' });
    order(op, { goal: null, upper: 'none', stance: 'lowReady' });
    R.step(30); const adopted = !!g.fight.watchOf(h) && h.hold && h.pinned;
    engage(g, [op], [h]); R.step(3);
    verdict(adopted && !h.hold && !h.pinned && !g.fight.watchOf(h) && h.skill === null, 'a defender let off his post by engage() stays off it (not held and pinned again the next frame) and is off the roster', `adopted first ${adopted} · after engage(): hold ${h.hold} pinned ${h.pinned} · on the roster ${!!g.fight.watchOf(h)} · skill ${h.skill === null ? 'none' : 'still set'}`);
  }
  return done();
});
