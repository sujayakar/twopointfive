// The 'fog' family (game/fogofwar.ts — the fog of war try behind ?fow / under ?roomclear), run through probes.ts:
//   bun run tools/qa/probes.ts fog | fog-off | fog-sight | fog-smoke | fog-rules | fog-cone | fog-identity   [--verbose]
// PASS / FAIL each, all headless (the Game of headless.ts; the night breach staged with game.nightBreach.start() as the page's flag stages it, so the team is the
// real pair and the men to hide are the real defenders behind the break room's shut doors). What the harness CAN hold: who is seen, by whose eye, and what the
// game then hands the renderer (the skin instances, the shadow proxies, the rig boxes — captured at the calls). What it CANNOT: anything the GPU pass draws (the
// dimming of unseen floor and walls is shader text — compile-checked in a page, judged by eye on a real GPU), and smoke on a line: the Rust game's asks stay in its
// own table and the headless host hands it no answers (only the page's host does, src/host/rustgame.ts), so nothing planted here reaches it.
//   fog-off       the switch off (the default): nothing hidden, the renderer's block off, no sensor asked — with the night breach staged and fought
//   fog-sight     the switch on: at the staging every defender is behind a wall / a shut leaf from EVERY eye of the team → unseen, and not in the drawn set (no skin
//                 instance, no shadow proxy, no rig box of his); the door opened with Sam before it → the men he has a line to are seen and drawn, the others not;
//                 then the real breach with Sam back in the aisle: every frame the module's answer per living defender equals an oracle written here (any eye of
//                 the team, a clear line to chest or head), and at some frame a defender is seen by a teammate while Sam himself has no line
//   fog-smoke     a defender in plain view through the open door: seen in clear air; the Rust game's own count of its asks (FW.stats.smokeAsks): some a frame while
//                 a line is clear, none with the knob at 0, at most six a frame with more pairs wanted than that. Until 2026-11-07 it planted the probe's answers
//                 (opaque, thin, late, stale, silent) on the copy's table, which the headless host does not hand over. What smoke does to a man is asked once, with a
//                 real canister under the game's own senses (SENSE.cpu): unseen and out of the picture while the line is clear. The late, stale and silent answers are not asked.
//   fog-rules     the smaller rules: a man who dies unseen stays out of the picture until somebody has a line to the body, then stays in it for good; the linger;
//                 a field under 360° turns an eye's back into fog (and its near field does not); a dazzled teammate is no eye; a show running stands it aside;
//                 the reticle does not find a hidden man under the cursor (it does with the switch off); a hidden man's spoken line goes to the log; the
//                 picture's eye is over the feet at a stance height, not the bobbing head
//   fog-cone      HARD'S FOG (FOG.cone — difficulty.ts writes it with the level): standard leaves every eye all round and hard raises the cone; under it the renderer's block carries the wedge, the near field and the soft edges (none of them without it); a
//                 man in a clear line is seen exactly when he stands inside some eye's cone or near field — an oracle written here, with Sam turned to face
//                 him, turned his back, and the teammates' cones counted or (FOG.own) not; with a rifle in the shoulder Sam's cone lies along his aim, not
//                 along his bladed chest
import { standUp, ROOT } from './headless.ts';
import { sample, swept } from './sweep.ts';
const { NIGHTBREACH } = await import(`${ROOT}/src/game/nightbreach.ts`);
const { FOG, thingKeyOf } = await import(`${ROOT}/src/game/fogofwar.ts`);
const { digestGame } = await import(`${ROOT}/src/game/digest.ts`);
const { killGuard, fireWeapon } = await import(`${ROOT}/src/game/combat.ts`);
const { MKEY } = await import(`${ROOT}/src/scene/boxes.ts`);
const { say, spawnUnit } = await import(`${ROOT}/src/game/guards.ts`);

const DT = 1 / 60;
const VERBOSE = process.argv.includes('--verbose');
function seeded(a: number) { let n = 0; const f = () => { n++; a |= 0; a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ (a >>> 15), 1 | a); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; }; (f as any).draws = () => n; return f; }
type Verdict = (ok: boolean, label: string, detail: string) => void;
function family(title: string): { verdict: Verdict; done: () => boolean } {
  console.log(`\n=== ${title} ===`); let all = true;
  return { verdict: (ok, label, detail) => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); }, done: () => { console.log(`=== ${title.split(':')[0]}: ${all ? 'ALL PASS' : 'FAILURES above'} ===`); return all; } };
}
const NB_DEFAULTS = { ...NIGHTBREACH }, FOG_DEFAULTS = { ...FOG };
const names = (l: any[]) => l.map(x => x.callsign).join(', ') || '—';

/** A game with the night breach staged (or the plain night floor), the fog switch as asked, and the picture's lists captured at the calls: per frame, the ids
 *  handed to the skin renderer, the owners given shadow proxies, and the owners of the rig boxes in the dynamic list. */
async function rig(seed: number, o: { fog?: boolean; breach?: boolean; chaos?: number } = {}) {
  Object.assign(NIGHTBREACH, NB_DEFAULTS); Object.assign(FOG, FOG_DEFAULTS);
  const dice = seeded(seed); Math.random = dice;
  const W: any = await standUp({ dress: true }); const game = W.game; const FW = game.fogOfWar; const NB = game.nightBreach;
  game.tune.chaos = o.chaos ?? 0;
  const inst: number[] = [], caps: number[] = []; let finished = -1;
  const si = game.chars.setInstance.bind(game.chars), fi = game.chars.finish.bind(game.chars), ac = W.engine.world.addCharacterCapsules.bind(W.engine.world);
  game.chars.setInstance = (i: number, j: any, t: any, t2: any, owner: number, wet: any) => { inst[i] = owner; return si(i, j, t, t2, owner, wet); };
  game.chars.finish = (n: number) => { finished = n; return fi(n); };
  W.engine.world.addCharacterCapsules = (owner: number, ...rest: any[]) => { caps.push(owner); return ac(owner, ...rest); };
  if (o.fog) FW.on = true;
  const why = o.breach === false ? null : NB.start();
  const R = {
    W, game, FW, NB, why, dice, askedFrame: 0,
    /** askedFrame: the smoke lines the Rust game asked this frame, by its own count */
    step(n = 1) { for (let i = 0; i < n; i++) { caps.length = 0; inst.length = 0; const a0 = FW.stats.smokeAsks; game.player.visibility = 0.05; W.step(DT); R.askedFrame = FW.stats.smokeAsks - a0; } },
    /** ids in the picture this frame: skin instances (the first `finished` slots) and shadow-proxy owners */
    instIds: () => inst.slice(0, finished), capIds: () => caps.slice(),
    /** owners of the rig boxes (guns, torches, holsters, goggles — combat.ts handProps keys them MKEY.rig) in this frame's dynamic list */
    rigOwners: () => new Set(game.dynBoxes.filter((b: any) => b.mkey !== undefined && (b.mkey >>> 24) === (MKEY.rig >>> 24)).map((b: any) => b.owner & 0xff)),
    inPicture(gd: any) { const id = gd.char.id; return { inst: R.instIds().includes(id), caps: R.capIds().includes(id), rig: R.rigOwners().has(id) }; },
    /** stood open by `a` on the leaf's positive side, as this rig always stood it (the frozen stand's two-way leaves: unchanged) — or, under one-way doors
     *  (DOORWAY.oneWay: the page's rule, the shipped stand's), on the side THIS leaf is dealt to swing: `sided` turns the angle round and the stops are the leaf's
     *  own, so a leaf whose positive stop is 0 there still opens. (So the two stands test through the same doorway with the leaf on opposite faces of it.) */
    openDoor(name: string, a = 1.45) { const d = game.doors.byName(name); d.angle = Math.max(d.minA, Math.min(d.maxA, d.sided(a))); d.latched = false; d.vel = 0; d.quietT = -3600; return d; },
    /** shut and latched, as the staging has it (NOT Door.reset(): this leaf is authored standing open) */
    shutDoor(name: string) { const d = game.doors.byName(name); d.angle = 0; d.vel = 0; d.latched = true; d.closing = false; d.quietT = 10; return d; },
    /** Sam put somewhere in the aisle west of the plan's door from where he has a line to `h` through the opening (the door must be open): candidates on a grid
     *  pre-checked with the collision's own segment test (and a quarter metre of slack round the spot), the first one where the module then really sees the man wins. Null = no such spot. */
    spotFor(h: any): number[] | null {
      const chest = h.char.bones.chest ?? [h.char.pos[0], 1.1, h.char.pos[2]];
      for (let x = 27.0; x >= 23.5; x -= 0.5) for (let z = 15.5; z <= 21.0; z += 0.25) {
        if ([[0, 0], [0.25, 0], [-0.25, 0], [0, 0.25], [0, -0.25]].some(([ox, oz]) => game.col.segmentBlocked([x + ox, 1.5, z + oz], chest))) continue;   // (with room to turn his head in)
        game.teleportPlayer([x, 0, z]); R.step(6);
        if (Math.hypot(game.player.char.pos[0] - x, game.player.char.pos[2] - z) < 0.3 && FW.seen(h)) return [x, 0, z];
      }
      return null;
    },
  };
  return R;
}
/** THE ORACLE, written here apart from the module: the team's eyes (Sam; every living operative who is not dazzled), and a living man seen when any of them has
 *  a line clear of statics and leaves to his chest or his head (collision.ts segmentBlocked — the sight test, glass passed). Returns who sees him. */
function oracle(game: any, gd: any): { any: boolean; sam: boolean; mates: boolean } {
  const head = (c: any) => c.bones.head ?? [c.pos[0], c.pos[1] + 1.65, c.pos[2]];
  const chest = gd.char.bones.chest ?? [gd.char.pos[0], gd.char.pos[1] + 1.1, gd.char.pos[2]], hd = gd.char.bones.head ?? [gd.char.pos[0], gd.char.pos[1] + 1.6, gd.char.pos[2]];
  const line = (eye: number[]) => !(game.col.segmentBlocked(eye, chest) && game.col.segmentBlocked(eye, hd));
  const sam = line(head(game.player.char));
  let mates = false; for (const o of game.guards) if (o.team === 'operative' && o.state !== 'dead' && !(game.time < o.dazzledUntil) && line(head(o.char))) mates = true;
  return { any: sam || mates, sam, mates };
}

// ================================================================================================== fog-off
export async function fogOff(): Promise<boolean> {
  const { verdict, done } = family('fog-off: the switch off (the default) — nothing hidden, nothing asked, the renderer\'s block off');
  const R = await rig(11, { fog: false, chaos: 0.5 }); const { game, FW, NB } = R;
  verdict(FW.on === false && !FW.active && R.why === null, 'a fresh game has the switch off', `on ${FW.on} · active ${FW.active} · night breach ${R.why ?? 'staged'}`);
  let hid = 0, frameOn = 0, short = 0, fowAsks = 0;
  const watch = (n: number) => { for (let i = 0; i < n; i++) { R.step(); hid += FW.hiddenChars.size; if (FW.frame.on) frameOn++; fowAsks += R.askedFrame; const want = 1 + game.guards.length; if (R.instIds().length !== want) short++; for (const gd of game.guards) if (FW.hidden(gd) || !FW.seen(gd) || !FW.drawn(gd)) hid++; } };
  watch(60); NB.order(); watch(240); NB.order(); watch(60 * 12);
  verdict(hid === 0 && frameOn === 0 && short === 0 && fowAsks === 0, 'through the staging, both orders and the fight: everybody drawn every frame, the block off, no sensor asked', `hidden-frames ${hid} · frames with the block on ${frameOn} · frames with a man missing from the skin list ${short} · fow segments asked ${fowAsks} · ended ${NB.state}`);
  verdict(FW.summary() === 'fog of war off' && FW.eyes.length === 0, 'the panel line says off, no eyes kept', FW.summary());
  return done();
}

// ================================================================================================== fog-sight
export async function fogSight(): Promise<boolean> {
  const { verdict, done } = family('fog-sight: a man no eye of the team has a line to is unseen and not drawn; a line from anybody on the team shows him');
  const R = await rig(12, { fog: true }); const { game, FW, NB } = R; const P = game.level.breach;
  R.step(30);
  { // the staging: three men behind a wall and two shut doors from all three eyes
    const hs = NB.hostiles as any[]; const eyes = FW.eyes.length;
    const perEye = hs.map(h => oracle(game, h)); const pics = hs.map(h => R.inPicture(h));
    verdict(R.why === null && FW.active && eyes === 3 && FW.frame.on && FW.frame.count === 3, 'on, with the team as its eyes: Sam and the two operatives', `${FW.summary()} · eyes ${eyes} (ids ${FW.eyes.map((e: any) => e.id)}) · block on ${FW.frame.on}, ${FW.frame.count} eyes`);
    verdict(perEye.every(o => !o.sam && !o.mates), 'the oracle: no eye of the team has a line to any defender (wall, both doors shut)', perEye.map((o, i) => `${hs[i].callsign}: sam ${o.sam} mates ${o.mates}`).join(' · '));
    verdict(hs.every(h => !FW.seen(h) && !FW.drawn(h) && FW.hidden(h)), 'the module: all three unseen, not drawn, hidden', hs.map(h => `${h.callsign}: seen ${FW.seen(h)} drawn ${FW.drawn(h)}`).join(' · '));
    verdict(pics.every(p => !p.inst && !p.caps && !p.rig), 'the drawn set: no skin instance, no shadow proxy, no rig box of any of them', pics.map((p, i) => `${hs[i].callsign}: inst ${p.inst} proxies ${p.caps} rig ${p.rig}`).join(' · '));
    const team = [{ char: game.player.char, callsign: 'Sam' }, ...NB.ops]; const tp = team.map(t => R.inPicture(t));
    verdict(tp.every(p => p.inst && p.caps) && R.instIds().length === 3 && tp.slice(1).every(p => p.rig), 'the team is drawn as ever: three skin instances, three sets of proxies, the pair\'s rifles', `instances [${R.instIds()}] · proxies [${R.capIds()}] · rig owners [${[...R.rigOwners()]}]`);
    verdict(NB.ops.every((o: any) => FW.seen(o) && FW.drawn(o) && !FW.hidden(o)), 'a teammate is never hidden', names(NB.ops));
  }
  { // the door opened, Sam in front of it: whoever he has a line to appears, the rest stay out
    game.teleportPlayer([25.2, 0, 17.6]); R.openDoor(P.door); R.step(20);
    const hs = NB.hostiles as any[]; const O = hs.map(h => oracle(game, h));
    const agree = hs.every((h, i) => FW.seen(h) === O[i].any); const some = O.some(o => o.any), notAll = O.some(o => !o.any);
    verdict(some && agree, 'the door open and Sam before it: the men with a line to them are seen — the module agrees with the oracle man for man', hs.map((h, i) => `${h.callsign}: oracle ${O[i].any} module ${FW.seen(h)}`).join(' · ') + ` · door ${game.doors.byName(P.door).angle.toFixed(2)}`);
    const pics = hs.map(h => R.inPicture(h));
    verdict(hs.every((h, i) => pics[i].inst === O[i].any && pics[i].caps === O[i].any && pics[i].rig === O[i].any), 'the drawn set follows: skin, proxies and gun for the seen, nothing for the unseen', pics.map((p, i) => `${hs[i].callsign}: inst ${p.inst} proxies ${p.caps} rig ${p.rig}`).join(' · ') + (notAll ? '' : ' (all three happen to be in the line)'));
    R.shutDoor(P.door); game.teleportPlayer([...NIGHTBREACH.samAt]); R.step(40);
    verdict(hs.every(h => !FW.seen(h) && FW.hidden(h)), 'the door shut again and Sam back in the aisle: all three out of the picture again (past the linger)', hs.map(h => `${h.callsign}: seen ${FW.seen(h)} hidden ${FW.hidden(h)}`).join(' · '));
  }
  { // the real breach, Sam standing in the aisle: module ≡ oracle every frame, and a teammate's line alone shows a man
    NB.restage(); R.step(30); NB.order(); R.step(60 * 7); NB.order();
    let frames = 0, bad = 0, firstBad = '', mateOnly = 0, samLine = 0, seenFrames = 0, drawnWrong = 0;
    for (let i = 0; i < 60 * 14; i++) {
      R.step(); frames++;
      for (const h of NB.hostiles as any[]) {
        if (h.state === 'dead') continue;
        const o = oracle(game, h); const m = FW.seen(h);
        if (o.any !== m) { bad++; firstBad ||= `t ${game.time.toFixed(2)} ${h.callsign}: oracle ${o.any} (sam ${o.sam}, mates ${o.mates}) module ${m}`; }
        if (o.sam) samLine++; if (m) seenFrames++; if (o.mates && !o.sam && m) mateOnly++;
        const p = R.inPicture(h); if (m && !(p.inst && p.caps)) drawnWrong++; if (FW.hidden(h) && (p.inst || p.caps || p.rig)) drawnWrong++;
      }
    }
    verdict(bad === 0, `the breach (stack, go, entry — ${frames} frames): the module\'s answer per living defender equals the oracle\'s every frame`, bad ? `${bad} disagreements, first: ${firstBad}` : `0 disagreements · defender-frames seen ${seenFrames} · ended ${NB.state}`);
    verdict(mateOnly > 0, 'a teammate getting a line shows a man Sam himself has no line to (sight is the team\'s)', `defender-frames seen by a teammate with Sam blind to them: ${mateOnly} · with a line from Sam: ${samLine}`);
    verdict(drawnWrong === 0, 'and the drawn set followed every frame: a seen man has his skin and proxies, a hidden man nothing', `${drawnWrong} wrong defender-frames`);
  }
  return done();
}

// ================================================================================================== fog-smoke
export async function fogSmoke(): Promise<boolean> {
  const { verdict, done } = family('fog-smoke: the asks stay inside their budget; a real plume hides the man');
  const R = await rig(13, { fog: true }); const { game, FW, NB } = R; const P = game.level.breach;
  game.teleportPlayer([25.2, 0, 17.6]); R.openDoor(P.door); R.step(20);
  const hs = (NB.hostiles as any[]); const inView = hs.filter(h => FW.seen(h));
  verdict(inView.length > 0 && R.askedFrame > 0, 'clear air: the men in the line are seen, and the lines are asked of the probe', `seen: ${names(inView)} · segments asked this frame ${R.askedFrame}`);
  FOG.smoke = 0; R.step(5);
  verdict(inView.every(h => FW.seen(h)) && R.askedFrame === 0, 'the knob at 0: nothing is asked of the probe', `seen ${inView.filter(h => FW.seen(h)).length}/${inView.length} · asked ${R.askedFrame}`);
  Object.assign(FOG, FOG_DEFAULTS);
  { // THE BUDGET: three eyes, everybody in the open with a line to everybody (more pairs wanted than the six asks a frame)
    const R2 = await rig(15, { fog: true }); const g2 = R2.game, F2 = R2.FW; g2.aiEnabled = false;
    const put = (gd: any, x: number, z: number) => { gd.char.pos[0] = x; gd.char.pos[2] = z; };
    const hs2 = [...(R2.NB.hostiles as any[]), spawnUnit(g2, { team: 'hostile', pos: [17, 0, 19], yaw: 0, name: 'Tango 4', light: false }), spawnUnit(g2, { team: 'hostile', pos: [17, 0, 20], yaw: 0, name: 'Tango 5', light: false })];   // (five men × their two nearest eyes = ten pairs wanted, six asked a frame: the turn-taking is exercised)
    const stage = () => { g2.teleportPlayer([9.6, 0, 20.3]); hs2.forEach((h, i) => put(h, 14 + i * 0.9, 18.4 + (i % 3) * 0.8)); };
    let most = 0; const run2 = (n: number) => { for (let i = 0; i < n; i++) { stage(); R2.step(); most = Math.max(most, R2.askedFrame); } };
    run2(20);
    const lines = hs2.map(h => oracle(g2, h)); const open = hs2.every(h => F2.seen(h));
    verdict(F2.eyes.length === 3 && lines.every(o => o.any) && lines.filter(o => o.mates).length >= 4 && open, 'staged in the open: five men with lines from the team\'s three eyes (more pairs than a frame\'s asks), clear air → all seen', `eyes ${F2.eyes.length} · ${hs2.map((h, i) => `${h.callsign}: sam ${lines[i].sam} mates ${lines[i].mates} seen ${F2.seen(h)}`).join(' · ')}`);
    verdict(most > 0 && most <= 6, 'the asks: only for lines already clear of walls, at most six a frame', `most in a frame ${most}`);
  }
  { // A REAL PLUME, under the game's own senses (SENSE.cpu: the Rust game answers its own smoke lines, which a headless game has nothing else to answer): a canister let off on the line
    const { SENSE } = await import(`${ROOT}/src/game/sense.ts`), was = SENSE.cpu; SENSE.cpu = true;
    try { const Q = await rig(13, { fog: true }); const g3 = Q.game, F3 = Q.FW; g3.teleportPlayer([25.2, 0, 17.6]); Q.openDoor(g3.level.breach.door); Q.step(20);
      const h = (Q.NB.hostiles as any[]).find(x => F3.seen(x)), sam = g3.player.char.pos, hid0 = F3.stats.smokeHid;
      if (!h) verdict(false, 'a canister on the line: a man in the line to test it on', 'nobody seen through the open door');
      else { g3.items.spawn('smoke', [(h.char.pos[0] + sam[0]) / 2, 0.1, (h.char.pos[2] + sam[2]) / 2], [0, 0, 0], { fuse: 0.05 });
        let lost = -1; for (let i = 0; i < 60 * 6 && lost < 0; i++) { Q.step(); if (!F3.seen(h)) lost = i; } Q.step(60);
        verdict(lost >= 0 && !F3.seen(h) && F3.hidden(h) && !Q.inPicture(h).inst && oracle(g3, h).any && F3.stats.smokeHid > hid0, 'a smoke canister let off on the line, under the game\'s own senses: the man is unseen, hidden, out of the skin list — while the line itself is still clear of walls and leaves (it is the smoke that hides him)',
          `${h.callsign} lost ${lost} frames after it was let off · seen ${F3.seen(h)} hidden ${F3.hidden(h)} · line ${oracle(g3, h).any} · lines the smoke hid ${F3.stats.smokeHid - hid0}`); }
    } finally { SENSE.cpu = was; }
  }
  return done();
}

// ================================================================================================== fog-rules
export async function fogRules(): Promise<boolean> {
  const { verdict, done } = family('fog-rules: bodies, the linger, a field under 360°, a dazzled teammate, a show running');
  const R = await rig(14, { fog: true }); const { game, FW, NB } = R; const P = game.level.breach;
  R.step(30);
  const hs = NB.hostiles as any[];
  { // a man who dies unseen stays out; seen lying there, he is in for good
    const h = hs[0]; killGuard(game, h, [1, 0, 0], true, 'struck', 'player'); R.step(90);
    verdict(h.state === 'dead' && !FW.drawn(h) && FW.hidden(h) && !R.inPicture(h).inst, 'a man who goes down unseen is not drawn', `${h.callsign} ${h.state} · drawn ${FW.drawn(h)}`);
    game.teleportPlayer([25.2, 0, 17.6]); R.openDoor(P.door); R.step(20);
    const sawBody = FW.seen(h);
    R.shutDoor(P.door); game.teleportPlayer([...NIGHTBREACH.samAt]); R.step(60);
    verdict(sawBody && !FW.seen(h) && FW.drawn(h) && !FW.hidden(h) && R.inPicture(h).inst, 'once the team has had a line to the body it stays drawn, line or no line', `had a line ${sawBody} · now seen ${FW.seen(h)} drawn ${FW.drawn(h)} in the skin list ${R.inPicture(h).inst}`);
  }
  { // the linger
    FOG.linger = 0.5; R.openDoor(P.door); R.step(2);
    const h = hs.find(x => x.state !== 'dead' && R.spotFor(x));
    if (!h) verdict(false, 'the linger: a living man in the line to test it on', 'no spot in the aisle with a line to a living defender through the open door');
    else {
      R.shutDoor(P.door); game.teleportPlayer([...NIGHTBREACH.samAt]);
      let seenFor = 0, drawnFor = 0; for (let i = 0; i < 90; i++) { R.step(); if (FW.seen(h)) seenFor++; if (FW.drawn(h)) drawnFor++; }
      const lingered = (drawnFor - seenFor) * DT;
      verdict(Math.abs(lingered - 0.5) < 0.06 && !FW.drawn(h), 'a man stays drawn FOG.linger past the last frame he was seen, then goes', `${h.callsign}: seen ${seenFor} frames more, drawn ${drawnFor} → lingered ${lingered.toFixed(2)} s (knob 0.5)`);
    }
    Object.assign(FOG, FOG_DEFAULTS);
  }
  { // a field under 360°: Sam's back is fog; the near field is not
    R.openDoor(P.door); R.step(2);
    const h = hs.find(x => x.state !== 'dead' && R.spotFor(x));
    if (!h) verdict(false, 'the field: a living man in the line to test it on', 'no spot in the aisle with a line to a living defender through the open door');
    else {
      const pc = game.player.char;
      const lookDot = () => { const e = FW.eyes[0]; const dx = h.char.pos[0] - e.pos[0], dz = h.char.pos[2] - e.pos[2]; const d = Math.hypot(dx, dz), L = Math.hypot(e.look[0], e.look[2]); return (dx * e.look[0] + dz * e.look[2]) / (d * L); };
      // (headless the cursor steers Sam's aim: it is held ten degrees to one side of the man while he settles, so his eye — the mean of a bladed chest and his
      //  head, some forty degrees round from his rifle's line — comes to rest a few tens of degrees off him. Left to the cursor's place at stand-up it rests wherever
      //  that stand's camera leaves it, and on the shipped stand that was 174° off, where no wedge wider than the angle is still under 360°. The test then reads which
      //  way his eye REALLY looks, and sets the wedge a little wider, then a little narrower, than the angle the man stands off that look — in, then out, on the
      //  same line of sight; `aimed` holds the staging: far enough off his look to narrow the wedge inside it, near enough that the wider one is a wedge)
      const SIDE = -10 * Math.PI / 180, sc = Math.cos(SIDE), ss = Math.sin(SIDE);
      for (let i = 0; i < 60; i++) { const dx = h.char.pos[0] - pc.pos[0], dz = h.char.pos[2] - pc.pos[2], d = Math.hypot(dx, dz) || 1;
        R.W.cursorAt([pc.pos[0] + (dx * sc - dz * ss) / d * 3, 1.1, pc.pos[2] + (dx * ss + dz * sc) / d * 3]); R.step(); }
      FOG.nearM = 1;
      const off = () => Math.acos(Math.max(-1, Math.min(1, lookDot()))) * 180 / Math.PI;
      const a0 = off(); FOG.fovDeg = 2 * (a0 + 12); R.step(); const a1 = off(), s1 = FW.seen(h), want1 = a1 < FOG.fovDeg / 2, cosWide = FW.frame.looks[2], fovWide = FOG.fovDeg;
      FOG.fovDeg = 2 * (a1 - Math.min(12, a1 / 2)); R.step(); const a2 = off(), s2 = FW.seen(h), want2 = a2 < FOG.fovDeg / 2, cosNarrow = FW.frame.looks[2];
      const aimed = a0 > 2 && a0 < 150;
      verdict(aimed && s1 && want1 && !s2 && !want2 && Math.abs(cosWide - Math.cos(fovWide / 2 * Math.PI / 180)) < 1e-5 && cosNarrow > cosWide, 'a field under 360°: a man with a clear line is seen inside the wedge about where the eye looks and not outside it (and the renderer\'s block carries the wedge)', `he stands ${a1.toFixed(0)}° off Sam's look${aimed ? '' : ` (the staging wants 2°‥150°: at ${a0.toFixed(0)}° there is no wedge to test)`} · field ${fovWide.toFixed(0)}° → seen ${s1} · field ${FOG.fovDeg.toFixed(0)}° → seen ${s2} · block cos ${cosWide.toFixed(2)} / ${cosNarrow.toFixed(2)} near ${FW.frame.looks[3]}`);
      FOG.nearM = 50; R.step(2);
      verdict(FW.seen(h), 'inside the near field an eye sees all round', `near field 50 m: seen ${FW.seen(h)}`);
      Object.assign(FOG, FOG_DEFAULTS); R.step(2);
      verdict(FW.seen(h) && FW.frame.looks[2] <= -1.5, 'back at 360°: seen whichever way he faces (the block says all round)', `seen ${FW.seen(h)} · block cos ${FW.frame.looks[2]}`);
      if (VERBOSE) console.log(`    [field] sam alive ${pc.alive} at ${pc.pos.map((v: number) => v.toFixed(2))} · ${h.callsign} ${h.state} at ${h.char.pos.map((v: number) => v.toFixed(2))} · door ${game.doors.byName(P.door).angle.toFixed(2)} · oracle ${JSON.stringify(oracle(game, h))}`);
    }
    R.shutDoor(P.door); game.teleportPlayer([...NIGHTBREACH.samAt]); R.step(40);
  }
  { // a dazzled teammate is no eye
    const op = NB.ops[0]; const n0 = FW.eyes.length; op.dazzledUntil = game.time + 1; R.step(2); const n1 = FW.eyes.length; R.step(70); const n2 = FW.eyes.length;
    verdict(n0 === 3 && n1 === 2 && n2 === 3 && !FW.hidden(op), 'a dazzled operative is no eye while it lasts (and is himself still drawn)', `eyes ${n0} → ${n1} dazzled → ${n2} after`);
  }
  { // a show stands it aside
    const live = hs.filter(x => x.state !== 'dead'); const before = live.every(x => FW.hidden(x));
    game.touring = true; R.step(2); const aside = !FW.active && FW.hiddenChars.size === 0 && !FW.frame.on && live.every(x => R.inPicture(x).inst);
    game.touring = false; R.step(2);
    verdict(before && aside && FW.active && live.every(x => FW.hidden(x)), 'under a show (the tour, a showcase) it stands aside: everybody drawn, the block off — and comes back after', `hidden before ${before} · aside ${aside} · ${FW.summary()}`);
  }
  { // the reticle: a hidden man under the cursor is nobody to snap to — with the switch off (the game as it was) the exact ray finds him through the wall
    const Q = await rig(16, { fog: true }); const g3 = Q.game, F3 = Q.FW; Q.step(30); const h = (Q.NB.hostiles as any[])[0];
    const chest = () => h.char.bones.chest ?? [h.char.pos[0], 1.2, h.char.pos[2]];
    const on = Q.W.cursorAt(chest()); Q.step(3); const hid = F3.hidden(h), aimOn = g3.aimGuard;
    F3.on = false; Q.W.cursorAt(chest()); Q.step(3); const aimOff = g3.aimGuard;
    verdict(on && hid && aimOn !== h && aimOff === h, 'the cursor over a hidden man behind the wall: no ring on him with the fog on (off, as on the base, the exact ray names him)', `hidden ${hid} · aimGuard with the fog on: ${aimOn?.callsign ?? 'nobody'} · off: ${aimOff?.callsign ?? 'nobody'}`);
    // a hidden man's spoken line: the log, not a bubble over where he stands
    g3.teleportPlayer([25.2, 0, 17.6]); F3.on = true; Q.step(90);   // (Sam up to the shut door: the man behind it is on the screen, and hidden)
    const logged: string[] = []; const m0 = g3.msg.bind(g3); g3.msg = (t: string) => { logged.push(t); return m0(t); };
    const hid2 = F3.hidden(h); say(g3, h, 'hold the door'); const hiddenLine = logged.some(t => t.includes('hold the door'));
    F3.on = false; Q.step(2); say(g3, h, 'watch the door'); const shownLine = logged.some(t => t.includes('watch the door'));
    verdict(hid2 && hiddenLine && !shownLine, 'a hidden man on the screen speaks: the line goes to the log (with the switch off the same line is a bubble over him, as ever)', `hidden ${hid2} · fog on → logged ${hiddenLine} · fog off → logged ${shownLine}`);
  }
  { // the picture's eye: over the feet at a stance height — not the head bone
    const Q = await rig(17, { fog: true }); const g4 = Q.game, F4 = Q.FW; Q.step(30); const pc = g4.player.char;
    const eyeY = () => F4.frame.eyes[1] - pc.pos[1]; const overFeet = () => Math.hypot(F4.frame.eyes[0] - pc.pos[0], F4.frame.eyes[2] - pc.pos[2]);
    let lo = 9, hi = -9, off = 0; for (let i = 0; i < 90; i++) { Q.W.input.keys.add('KeyD'); Q.W.input.keys.add('ShiftLeft'); Q.step(); lo = Math.min(lo, eyeY()); hi = Math.max(hi, eyeY()); off = Math.max(off, overFeet()); }
    const stand = eyeY(); Q.W.input.keys.clear(); g4.player.crouch = true; for (let i = 0; i < 60; i++) { g4.player.crouch = true; pc.anim.crouchTarget = 1; Q.step(); } const low = eyeY();
    verdict(hi - lo < 1e-4 && Math.abs(stand - 1.55) < 1e-4 && off < 1e-4 && low < 1.2 && low >= 0.95 - 1e-4 && F4.frame.eyes[3] === pc.id, 'the mask\'s eye for Sam: 1.55 m over his feet through a second and a half of running (the head bone bobs; this does not), lower crouched, his id beside it', `standing / running ${lo.toFixed(3)}‥${hi.toFixed(3)} m · off his feet ${off.toFixed(3)} m · crouched ${low.toFixed(2)} m · crouch shown ${pc.anim.crouch.toFixed(2)}`);
  }
  return done();
}

// ================================================================================================== fog-filed
/** FOG.filed: a checkpoint carries what the fog holds another way than the world stands (a door, a lamp and its face, furniture held or left out) and it is the
 *  picture's again after the load, whose restart forgets all the fog knew. FOGFILED_BREAK=1 loads each file with its fog slice cut out: the picture after
 *  each load has it all as it stands, and every line that asks for the memory is red.
 *  The row and the fog rules it stands on are settled on and no longer written. Until 2026-11-07 this probe also asked 'with the row off the file has no
 *  key', 'the row off at the load: the build as it was', 'the furniture rule put off and on two frames after a load', and the address, the journal and
 *  the two stands. Also until that day: the lines that laid the fog's own doorSeen / thingSeen / spotSeen over the copy (the Rust game does not ask them: 'an
 *  eye has them', 'a look inside the half second', 'furniture left out'), and those that restarted the game after a load by hand (the file's load is the
 *  copy's, and the export of a start over it throws; the last TypeScript game refused the file outright): 'the fog off at the load', 'not an opening half
 *  second', 'a body'. What is left is the file and the load's picture. */
export async function fogFiled(): Promise<boolean> {
  const { verdict, done } = family('fog-filed: a checkpoint carries what the fog holds another way than the world stands');
  const brk = process.env.FOGFILED_BREAK === '1';
  const R = await rig(71, { fog: true, breach: false }); const { game, FW, W } = R; const world = W.engine.world, F = FW as any;
  const SV = await import(`${ROOT}/src/game/savestate.ts`);
  game.aiEnabled = false; R.step(45);
  const sum = (e: number[] | undefined) => e ? e[0] + e[1] + e[2] : -1, sam = game.player.char.pos, away = (x: number, z: number) => Math.hypot(x - sam[0], z - sam[2]);
  const statics = () => game.engine.world.statics as any[], dyn = () => world.dynamics as any[];
  const save = () => JSON.parse(JSON.stringify(SV.saveState(game, { label: 'fog', kind: 'harness', force: true })));
  const load = (file: any, slice = !brk) => { const f = JSON.parse(JSON.stringify(file)); if (!slice) delete f.fog; SV.loadState(game, f); };
  const fresh = () => { game.restartEncounter(); game.aiEnabled = false; R.step(45); };   // (the level as it starts, the fog's memory empty, past the opening half second)
  // ---- the scene: a door swung open, a lamp put out and a piece of furniture shoved, each where nobody on the team has a look
  const doors = game.doors.list as any[], di = doors.map((d, i) => i).filter(i => !F.doorSeen(doors[i], doors[i].angle) && Math.abs(doors[i].angle) < 0.01).sort((a, b) => away(doors[b].def.hinge[0], doors[b].def.hinge[1]) - away(doors[a].def.hinge[0], doors[a].def.hinge[1]))[0];
  const targets = game.targets as any[], ti = targets.findIndex(t => (t.light || t.areaLights) && t.boxes.length > 0 && sum(t.baseEmissive) > 0 && t.on && !t.broken && !(t.disabledUntil > 0) && !FW.lampSeen(t));
  const props = game.props.all as any[], unseen = props.map((p, i) => i).filter(i => !FW.spotSeen(props[i].x, props[i].z)).sort((a, b) => away(props[b].x, props[b].z) - away(props[a].x, props[a].z)), pi = unseen[0];
  if (di === undefined || ti < 0 || unseen.length < 2) { verdict(false, 'the scene: a door, a lit lamp with a face and two pieces of furniture, each out of the team\'s sight', `door ${di} · lamp ${ti} · furniture ${unseen.length}`); Object.assign(FOG, FOG_DEFAULTS); return done(); }
  const lampName = targets[ti].name as string, doorName = doors[di].def.name as string, nothing = () => JSON.stringify(FW.snapshot()) === '{}', key = (i: number) => thingKeyOf(props[i].boxes[0]);
  const glow = targets[ti].boxes.map((bi: number) => sum(statics()[bi].emissive));
  const homeOf = (i: number) => props[i].boxes.map((b: any) => [...b.c]), home = homeOf(pi);
  const ghostOf = (p: any) => dyn().filter(b => !p.boxes.includes(b) && p.boxes.some((q: any) => q.mkey === b.mkey)), liveIn = (p: any) => p.boxes.some((b: any) => dyn().includes(b));
  const atHome = (i: number, h: number[][]) => { const g = ghostOf(props[i]); return g.length === props[i].boxes.length && g.every((b: any, k: number) => Math.hypot(b.c[0] - h[k][0], b.c[1] - h[k][1], b.c[2] - h[k][2]) < 1e-6) && !liveIn(props[i]); };
  const swing = (d: any) => { d.angle = d.maxA > 0.5 ? Math.min(d.maxA, 1.5) : Math.max(d.minA, -1.5); d.vel = 0; d.latched = false; d.quietT = -3600; d.closing = false; d.place(); };
  const shove = (i: number) => { const p = props[i]; p.x += 0.7; p.z += 0.3; p.yaw += 0.6; p.dirty = true; p.place(); };
  swing(doors[di]); game.lightHome.stage(targets[ti], false, null); shove(pi); R.step(3);
  const picture = () => { const d = doors[di], t = targets[ti];
    return { door: FW.doorHeld(d) && Math.abs(FW.doorShown(d)) < 0.01 && Math.abs(d.angle) > 1, lamp: t.boxes.every((bi: number, i: number) => Math.abs(sum(statics()[bi].fogEmit) - glow[i]) < 1e-6 && sum(statics()[bi].emissive) === 0), thing: atHome(pi, home) }; };
  const plain = () => { const d = doors[di], t = targets[ti], p = props[pi];
    return { door: !FW.doorHeld(d) && Math.abs(FW.doorShown(d) - d.angle) < 1e-9, lamp: t.boxes.every((bi: number) => statics()[bi].fogEmit === undefined), thing: ghostOf(p).length === 0 && p.boxes.every((b: any) => dyn().includes(b)) }; };
  const all = (o: { door: boolean; lamp: boolean; thing: boolean }) => o.door && o.lamp && o.thing, say = (o: { door: boolean; lamp: boolean; thing: boolean }) => `door ${o.door}, lamp ${o.lamp}, furniture ${o.thing}`;
  const same = doors === game.doors.list && targets === game.targets && props === game.props.all;   // (asked again after the loads: the lists this probe holds are the game's)
  const before = picture(), recWas = { ...F.thingRecs.get(key(pi)) };
  const file = save(), fg = file.fog;
  { // the file
    const d = fg?.doors?.find((e: any) => e[0] === doorName), l = fg?.lamps?.find((e: any) => e[0] === lampName), th = fg?.things?.find((e: any) => e[0] === key(pi));
    const rows = !!l && l[1] === 1 && Array.isArray(l[2]) && l[2].length === targets[ti].boxes.length && l[2].every((v: number[], i: number) => Math.abs(sum(v) - glow[i]) < 1e-6);
    const poses = !!th && th[1].length === home.length && th[1].every((q: number[], i: number) => q.length === 4 && Math.hypot(q[0] - home[i][0], q[1] - home[i][1], q[2] - home[i][2]) < 1e-6);
    const only = (fg?.doors?.length ?? 0) === 1 && (fg?.lamps?.length ?? 0) === 1 && (fg?.things?.length ?? 0) === 1 && fg?.out === undefined;
    FW.on = false; const noFog = FW.snapshot() === null; FW.on = true;
    verdict(all(before) && !!d && Math.abs(d[1]) < 0.01 && rows && poses && only && noFog,
      'a door swung open, a lamp put out and furniture shoved where nobody looks: the picture holds each as last seen, and the checkpoint files those three and nothing else — the door by its name with the angle shown, the lamp by its name as known lit with its boxes\' emission as they glowed, the furniture by its key with its parts where they stood; with the fog off the file has no key',
      `the picture before the save: ${say(before)} · the file: doors ${JSON.stringify(fg?.doors)} · lamps ${fg?.lamps?.length ?? 0} ('${l?.[0]}', as it glowed: ${rows}) · things ${fg?.things?.length ?? 0} (at the old pose: ${poses}) · those alone: ${only} · fog off → none: ${noFog}`);
  }
  { // the load
    load(file);
    const again = JSON.stringify(save().fog) === JSON.stringify(fg);   // (before any frame: a save straight after the load says what the file said)
    R.step(2); const first = picture(); const r = F.thingRecs.get(key(pi)), rec = !!r && Math.abs(r.x - recWas.x) + Math.abs(r.z - recWas.z) + Math.abs(r.ox - recWas.ox) + Math.abs(r.oz - recWas.oz) < 1e-9;
    R.step(60); const later = picture();   // (… and past the opening half second, in which everything else is taken as it stands)
    const kept = JSON.stringify(save().fog) === JSON.stringify(fg), lists = doors === game.doors.list && targets === game.targets && props === game.props.all;
    verdict(same && lists && again && all(first) && rec && all(later) && kept, 'loaded (the restart under it forgets all the fog knew): a save before the first frame says what the file said; the picture two frames in and a second later holds the three as before the save, the furniture\'s record with the middle and the long side it had (where "is the team looking at it" is asked), and a save then files them the same',
      `save(load) ≡ the file's fog: ${again} · two frames in: ${say(first)} · its record's middle and side as before: ${rec} · a second later: ${say(later)} · saved again the same: ${kept}`);
  }
  { // as written
    const fsrc = await import('node:fs'), fsrcText = fsrc.readFileSync(`${ROOT}/src/game/fogofwar.ts`, 'utf8'), pageRows = /const FOG_PAGE = \{([^}]*)\}/.exec(fsrcText)?.[1] ?? 'filed', page = !/filed/.test(pageRows), moduleOn = /\n  filed: true,\n/.test(fsrcText);   // (the replay page's look has no row for it: it is not the picture alone)
    verdict(moduleOn && page, 'written: on in the module, and no row of the replay page\'s fog look (it is not the picture alone)', `the module's on: ${moduleOn} · the page look without it: ${page}`);
  }
  Object.assign(FOG, FOG_DEFAULTS);
  return done();
}

// ================================================================================================== fog-things
/** what the handed-over list did since the last call: the keyed loose things (not leaves, kit, devices, staging boxes) whose pose in the PICTURE's list changed
 *  this frame while the floor under them is not in the team's sight — the leak rule 4d closes. One watcher per game; call it once a frame after the step. */
function thingLeakWatch(W: any) {
  const game = W.game, FW = game.fogOfWar;
  let prev = new Map<number, string>();
  return (): { leaks: number; moved: number; names: string[] } => {
    const cur = new Map<number, string>(), of = new Map<number, any[]>(), tests = FW.thingStats.tests;
    for (const b of W.engine.world.dynamics as any[]) { if (b.mkey === undefined) continue; const tk = thingKeyOf(b); if (tk < 0 || (b.mkey >>> 24) === 4) continue;
      cur.set(tk, (cur.get(tk) ?? '') + `${b.c[0].toFixed(3)},${b.c[1].toFixed(3)},${b.c[2].toFixed(3)},${b.yaw.toFixed(3)}${b.rot ? ',' + b.rot.map((x: number) => x.toFixed(3)).join(',') : ''};`);
      const a = of.get(tk); if (a) a.push(b); else of.set(tk, [b]); }
    let leaks = 0, moved = 0; const names: string[] = [];
    for (const [tk, sig] of cur) { const was = prev.get(tk); if (was === undefined || was === sig) continue; moved++;
      const bs = of.get(tk)!; if (!FW.seesThing(bs)) { leaks++; if (names.length < 6) names.push(bs[0].name ?? `key ${bs[0].mkey.toString(16)}`); } }
    FW.thingStats.tests = tests;   // (the watcher's own questions are not the rule's)
    prev = cur; return { leaks, moved, names };
  };
}
/** (FOG.things is settled on and no longer written. Until 2026-11-07 this probe also asked 'the switch off: the game's own list, as it was before the
 *  rule', the leak count 'without it they do (the fault)', and the two stands' word; and it raised the destruction switch, settled on, for the panel.) */
export async function fogThings(): Promise<boolean> {
  const { verdict, done } = family('fog-things: a loose thing that moves where nobody on the team sees it stays in the picture where it was last seen');
  const R = await rig(43, { fog: true }); const { game, FW, W } = R; const world = W.engine.world;
  R.step(45);
  const own: any[] = []; const sameList = () => { own.length = 0; game.doors.boxes(own); game.props.boxes(own); const a = world.dynamics as any[]; return own.every((x, i) => x === a[i]); };   // (the leaves and the furniture lead the list: the game's own boxes, in order)
  const has = (b: any) => (world.dynamics as any[]).includes(b);
  const ghostOf = (p: any) => (world.dynamics as any[]).filter(b => !p.boxes.includes(b) && p.boxes.some((q: any) => q.mkey === b.mkey));
  { // a calm floor: the list is the game's own, box for box
    verdict(sameList() && FW.thingStats.held === 0 && FW.thingStats.out === 0, 'a calm floor: the picture is handed the game\'s own boxes, every one, in order (nothing remembered, nothing left out)',
      `${world.dynamics.length} boxes handed, the first ${own.length} the leaves' and the furniture's own · held ${FW.thingStats.held} · out ${FW.thingStats.out} · sight tests that frame ${FW.thingStats.tests}`);
  }
  // a piece of furniture nobody sees, the furthest from the player
  const sam = game.player.char.pos;
  const far = (game.props.props as any[]).filter(p => !FW.spotSeen(p.x, p.z)).sort((a, b) => Math.hypot(b.x - sam[0], b.z - sam[2]) - Math.hypot(a.x - sam[0], a.z - sam[2]));
  const p = far[0]; const home = { x: p.x, z: p.z, yaw: p.yaw }; const c0 = p.boxes.map((b: any) => [...b.c]);
  const shove = (x: number, z: number, yaw: number) => { p.x = x; p.z = z; p.yaw = yaw; p.dirty = true; p.place(); };
  {
    shove(home.x + 0.7, home.z + 0.3, home.yaw + 0.6); R.step(1);
    const g1 = ghostOf(p); const live1 = p.boxes.some(has);
    const atHome = g1.length === p.boxes.length && g1.every((b: any, i: number) => Math.hypot(b.c[0] - c0[i][0], b.c[1] - c0[i][1], b.c[2] - c0[i][2]) < 1e-6);
    R.step(90); const g2 = ghostOf(p); const stays = g2.length === p.boxes.length && g2.every((b: any, i: number) => b === g1[i] && Math.hypot(b.c[0] - c0[i][0], b.c[2] - c0[i][2]) < 1e-6);
    const heldRecs = [...FW.thingRecs.values()].filter((r: any) => r.held).length;   // (the record's own word: thingStats.held reads 0 on the copy)
    verdict(far.length > 0 && !live1 && atHome && stays && heldRecs === 1, 'furniture shoved 0.76 m where nobody sees it: the picture keeps it where it stood (its own boxes are not handed over; copies at the old pose are), and it stays there',
      `${far.length} props out of sight · '${p.def.name}' ${Math.hypot(home.x - sam[0], home.z - sam[2]).toFixed(1)} m from the player · live boxes in the picture ${live1} · ${g1.length}/${p.boxes.length} remembered parts at the old pose ${atHome} · after 1.5 s still ${stays} · held ${heldRecs}`);
    FW.on = false; R.step(1); const fogOff = sameList() && p.boxes.every(has); FW.on = true; R.step(1);
    verdict(fogOff, 'the fog of war off: the renderer is handed the game\'s own boxes', `the game's own ${fogOff}`);
  }
  { // somebody comes to look: it is where it really is
    const start: [number, number, number] = [sam[0], 0, sam[2]]; let seen = false, from = '';
    FW.on = true; R.step(40);   // (the fog back on began a new memory at the shoved pose — put it out of sight again from a known one)
    shove(home.x, home.z, home.yaw); R.step(1); const heldBack = !p.boxes.some(has);
    for (let k = 0; k < 16 && !seen; k++) { const a = k * Math.PI / 8, d = k < 8 ? 1.3 : 2.2; game.teleportPlayer([home.x + Math.cos(a) * d, 0, home.z + Math.sin(a) * d]); R.step(4);
      const q = game.player.char.pos; if (FW.spotSeen(p.x, p.z) && Math.hypot(q[0] - home.x, q[2] - home.z) < 3.5) { seen = true; from = `(${q[0].toFixed(1)}, ${q[2].toFixed(1)})`; } }
    R.step(4); const liveNow = p.boxes.every(has) && ghostOf(p).length === 0;
    shove(home.x + 0.2, home.z, home.yaw + 0.2); R.step(1); const liveMoving = p.boxes.every(has) && ghostOf(p).length === 0;
    verdict(heldBack && seen && liveNow && liveMoving, 'the player walks up to it: the picture has it where it really stands, and while he watches it moves in the picture as it moves',
      `moved back unseen: held ${heldBack} · seen from ${from || '—'} ${seen} · live ${liveNow} · shoved in his sight: live ${liveMoving}`);
    // it leaves while he is looking at the place: gone from there, and not drawn where nobody sees it
    R.step(3); const away = far[far.length > 3 ? 3 : far.length - 1]; shove(away.x + 0.9, away.z + 0.9, 0); R.step(2);
    const seenThere = FW.spotSeen(p.x, p.z); const nowhere = !p.boxes.some(has) && ghostOf(p).length === 0;
    verdict(seenThere || nowhere, 'it is taken away to where nobody sees while he looks at the place it stood: it is in neither place in the picture', `at its new place seen ${seenThere} · in the picture ${!nowhere}`);
    game.teleportPlayer(start); shove(home.x, home.z, home.yaw); R.step(8);
  }
  { // (until 2026-11-07: a one-frame effect, a thing dropped out of sight and a fallen light's pieces by the floor they lie on, red on the last TypeScript game as on this one)
      // the six, against a real panel: break one and read the boxes fixtureBreak.ts makes for it — the body is its last six, parts 0‥5, and nothing before them is
      const { fixtureBreakBoxes } = await import(`${ROOT}/src/game/fixtureBreak.ts`);
      let got: any[] = []; let which = -1;
      for (let k = 0; k < game.targets.length && which < 0; k++) { const t = game.targets[k]; if (t.broken) continue; game.lightHome.stage(t, null, true); const o: any[] = []; fixtureBreakBoxes(game, o, 0); game.lightHome.stage(t, null, false); fixtureBreakBoxes(game, [], 0);
        if (o.length > 6) { got = o; which = k; } }
      const parts = got.map(x => x.mkey & 31), tail = parts.slice(-6), head = parts.slice(0, -6);
      verdict(which >= 0 && tail.join() === '0,1,2,3,4,5' && head.length > 0 && head.every(x => x >= 6) && new Set(got.map(x => thingKeyOf(x))).size === head.length + 1,
        'a real broken panel\'s boxes: what came off it first, each a thing by itself, then its body — six boxes, parts 0 to 5, one thing (the count the fog\'s rule is written against)',
        `target ${which} · ${got.length} boxes · parts ${parts.join(' ')} · things ${new Set(got.map(x => thingKeyOf(x))).size}`);
  }
  { // the leak count: furniture thrown about all over the floor for ten seconds
    const run = async () => { const Q = await rig(44, { fog: true }); Q.step(45); const watch = thingLeakWatch(Q.W); watch(); let leaks = 0, moved = 0, tests = 0, held = 0; const names = new Set<string>();
      const dice = seeded(7); const ps = Q.game.props.props as any[];
      for (let f = 0; f < 600; f++) { if (f % 20 === 0) for (let k = 0; k < 6; k++) { const q = ps[Math.floor(dice() * ps.length)]; const a = dice() * Math.PI * 2; q.vx += Math.cos(a) * 2.2; q.vz += Math.sin(a) * 2.2; q.w += (dice() - 0.5) * 6; q.wake(); }
        Q.step(1); const r = watch(); leaks += r.leaks; moved += r.moved; r.names.forEach(x => names.add(x)); tests = Math.max(tests, Q.FW.thingStats.tests); held = Math.max(held, Q.FW.thingStats.held); }
      return { leaks, moved, tests, held, names: [...names].slice(0, 5).join(', ') }; };
    const on = await run();
    verdict(on.leaks === 0 && on.moved > 0, 'ten seconds of furniture shoved about all over the floor: not one thing moves in the picture where the team does not see',
      `${on.leaks} leaks of ${on.moved} moves in the picture, ≤ ${on.held} things held, ≤ ${on.tests} sight tests a frame${on.names ? ` (${on.names})` : ''}`);
  }
  Object.assign(FOG, FOG_DEFAULTS);
  return done();
}

// ================================================================================================== fog-flat
/** THE FOG'S OWN LIGHT IS ON WHERE THE FOG IS (FOG.flat; 'you can see enemies flashlights in fog of war'): the block handed to the renderer carries the level
 *  whenever the fog runs, and the shader's unseen pixel under a level takes nothing from the live frame. The picture itself is not seen here (no GPU).
 *  FOGFLAT_BREAK=1 = the fog is put off before the block is read: must FAIL.
 *  The level is settled (and the full picture, FOG.full, which this probe held off to reach the own light): neither is written any more, and the block is
 *  asked for the level as the page has it. Until 2026-11-07 the probe wrote 0.2 and asked for it, and also asked the row at 0 ('the live frame dimmed, as
 *  it was'), ?fogflat on the address, a recording from before the key and the two stands. */
export async function fogFlat(): Promise<boolean> {
  const { verdict, done } = family('fog-flat: a room nobody on the team sees is drawn under the fog\'s own light, not the live frame dimmed');
  const brk = process.env.FOGFLAT_BREAK === '1';
  const R = await rig(43, { fog: true }); const { FW } = R;
  if (brk) FW.on = false;
  { // the block
    R.step(30); const onLvl = FW.frame.flat, onFog = FW.frame.on && FW.frame.count > 0;
    verdict(onFog && onLvl === FOG.flat && FOG.flat > 0, 'under the fog the renderer\'s block carries the level',
      `fog on ${FW.frame.on}, ${FW.frame.count} eyes · the level ${FOG.flat} · handed: ${onLvl}`);
  }
  { // the shader: under a level, the unseen pixel is built from the paint alone
    const fs = await import('node:fs'); const src: string = fs.readFileSync(`${ROOT}/src/render/shaders/fogofwar.wgsl`, 'utf8'), up: string = fs.readFileSync(`${ROOT}/src/render/fogofwar.ts`, 'utf8');
    const a = src.indexOf('if (fow.soft.w > 0.0) {'), b = a < 0 ? -1 : src.indexOf('\n  }', a); const body = a < 0 || b < 0 ? '' : src.slice(a, b).split('\n').map(l => l.replace(/\/\/.*$/, '')).join('\n');
    const paint = /textureLoad\(fowAlbedo/.test(body) && /unseen\s*=/.test(body), live = /\bc\.rgb\b|\bc\b\s*[*+]/.test(body), wired = /flat = full \? F\.fullFlat : F\.flat;/.test(up) && /U\[o \+ 7\] = flat;/.test(up);   // (the level, unless the full picture — rule 4g — is what is drawn)
    verdict(paint && !live && wired, 'fogofwar.wgsl: with a level set the unseen pixel is the G-buffer paint under the fixed light and reads nothing of the live frame; the level reaches the uniform',
      `the branch reads the paint and writes unseen: ${paint} · reads the live colour: ${live} · render/fogofwar.ts writes F.flat to soft.w: ${wired}`);
  }
  Object.assign(FOG, FOG_DEFAULTS);
  return done();
}

// ================================================================================================== fog-lit
/** THE FOG IS LIT AS THE TEAM LAST KNEW IT (FOG.lit; 'fog of war looks very flat' · 'would be nice for it to have some RT lighting, maybe just based on last known
 *  state'): under the fog the block handed to the renderer carries the gain and a list of lamps made of the level's fixtures and the sun or moon alone — no torch,
 *  weapon light or flash; a fixture that changes where nobody looks stays in the list as remembered until an eye has the floor under it; the mains are as they
 *  are; the shader's lit branch reads the paint and that list and nothing of the live frame, and its shadow walk passes every box a man carries. The picture
 *  itself is not seen here (no GPU). FOGLIT_BREAK=1 = the fog is put off before the block is read: must FAIL.
 *  The gain, the own light and the full picture (FOG.full, which this probe held off to reach the lit fog) are settled and no longer written. Until
 *  2026-11-07 the probe also asked the row at 0 and the own light at 0 ('the fog as it was'), ?foglit on the address, a recording from before the key
 *  and the two stands. */
export async function fogLit(): Promise<boolean> {
  const { verdict, done } = family('fog-lit: a room nobody on the team sees is lit by the level\'s own lamps as the team last knew them, and by nothing live');
  const brk = process.env.FOGLIT_BREAK === '1';
  const R = await rig(47, { fog: true }); const { game, FW, W } = R;
  if (brk) FW.on = false;
  const F = FW.frame;
  const lightsOf = (t: any): any[] => [...(t.light ? [t.light] : []), ...((t.areaLights ?? []).map((a: any) => a.light))];
  const fixtures: any[] = game.targets.filter((t: any) => t.light || t.areaLights);
  { // the block
    R.step(30); const g1 = F.lit, n1 = F.lightCount, fogOn = F.on && F.count > 0;
    const fill = F.litFill, wantFill = FOG.flat * FOG.litFill;
    verdict(fogOn && g1 === 1 && n1 > 0 && Math.abs(fill - wantFill) < 1e-9 && fill < FOG.flat, 'under the fog the renderer\'s block carries the gain, the level\'s lamps and a fill under the own light\'s level',
      `fog on ${F.on}, ${F.count} eyes · gain ${g1}, ${n1} lamps of ${fixtures.length} fixtures that carry light, fill ${fill.toFixed(3)} of own light ${FOG.flat}`);
  }
  // (until 2026-11-07 the block's rows were read here, a lamp by its place, and a torch and a flash looked for among them: the copy is not given the rows, only their count.
  //  What an enemy's torch and a flash are classed is fog-full's)
  { // last known: a lit fixture nobody sees is switched off — the fog keeps it lit; the frame an eye has the floor under it, it goes. One the team sees goes at once.
    const seenAt = (t: any) => FW.lampSeen(t);
    const litOnes = fixtures.filter((t: any) => t.on && !t.broken && lightsOf(t).some((l: any) => l.enabled && l.intensity > 0));
    const far = litOnes.find((t: any) => !seenAt(t)), near = litOnes.find((t: any) => seenAt(t));
    let kept = false, held = 0, nearGone = !near;
    if (far) { const n0 = F.lightCount; game.lightHome.stage(far, false, null); R.step(3); kept = F.lightCount === n0; held = FW.litStats.held; game.lightHome.stage(far, true, null); FW.reset(); R.step(3); }
    if (near) { const n0 = F.lightCount; game.lightHome.stage(near, false, null); R.step(2); nearGone = F.lightCount < n0; game.lightHome.stage(near, true, null); R.step(2); }
    verdict(!!far && kept && held >= 1 && nearGone, 'a lit lamp switched off where the team has no look at it (not the floor under it, not the fixture, not floor it lights) stays lit in the fog (and is counted as held); a lamp the team sees follows at once',
      `${litOnes.length} lit fixtures in the list, one unseen: ${far ? far.name : '—'} · off, unseen → the list's count unchanged: ${kept} (held ${held}) · a seen one (${near ? near.name : 'none in sight: not asked'}) off → gone: ${nearGone}`);
  }
  { // the mains are as they are: the trip is told to the whole floor
    const { setBlackout } = await import(`${ROOT}/src/game/lighting.ts`);
    const fed = fixtures.filter((t: any) => t.mains && t.on && !t.broken), n1 = F.lightCount;
    setBlackout(game, 3600, 'debug'); R.step(3); const n2 = F.lightCount;
    game.endBlackout(); R.step(3); const n3 = F.lightCount;
    verdict(fed.length > 0 && n2 < n1 && n3 === n1, 'with the mains tripped the mains-fed lamps leave the list (its count); with them back it is as it was',
      `${fed.length} mains-fed lamps (${n1} lamps in the list) · mains down: ${n2} lamps · mains back: ${n3}`);
  }
  { // the shader: the lit branch is the paint under the list — nothing of the live frame, and no shadow from what a man carries
    const fs = await import('node:fs'); const src: string = fs.readFileSync(`${ROOT}/src/render/shaders/fogofwar.wgsl`, 'utf8'), up: string = fs.readFileSync(`${ROOT}/src/render/fogofwar.ts`, 'utf8'), common: string = fs.readFileSync(`${ROOT}/src/render/shaders/common.wgsl`, 'utf8');
    const strip = (t: string) => t.split('\n').map(l => l.replace(/\/\/.*$/, '')).join('\n');
    const fnBody = (text: string, name: string) => { const a = text.indexOf(`fn ${name}(`), b = a < 0 ? -1 : text.indexOf('\n}', a); return a < 0 || b < 0 ? '' : strip(text.slice(a, b)); };
    const a = src.indexOf('if (fow.soft.w > 0.0) {'), b = a < 0 ? -1 : src.indexOf('\n  }', a); const branch = a < 0 || b < 0 ? '' : strip(src.slice(a, b));
    const lit = fnBody(src, 'fowLit'), hit = fnBody(src, 'fowLitHit'), open = fnBody(src, 'fowLitOpen'), all = branch + lit + hit + open;
    const paint = /let paint = textureLoad\(fowAlbedo/.test(branch) && /if \(fow\.lit\.x > 0\.0\) \{ lit = fowLit\(P, N, paint, form\); \}/.test(branch) && /unseen\s*=/.test(branch) && /fowLights\.l\[/.test(lit) && /fow\.lit\.x \* fow\.cfg\.y/.test(lit);
    const live = /\bc\.rgb\b|\bc\b\s*[*+]|fowSrc|\blights\[|frame\.numLights|directIrradiance|occludedT?\(|occludedTBoxes|traceCapsules|meshShadow|smoke/.test(all);
    const owners = (hit.match(/if \(boxOwner\([^)]*\) != 0u\) \{ continue; \}/g) ?? []).length;
    const wired = /U\[o \+ 8\] = full \? 1 : F\.lit;/.test(up) && /writeBuffer\(this\.lbuf!, 0, F\.lights\.buffer/.test(up);
    verdict(paint && !live && owners === 2 && wired, 'fogofwar.wgsl: under the own light the lit branch is the G-buffer paint under the fog\'s own list of lamps (× the fog\'s dim); it reads no live colour, no live light, no capsule and no smoke, and its shadow walk passes every owned box (the globals and the cells); the gain and the list reach the pass',
      `the branch reads the paint and the list: ${paint} · anything live in it: ${live} · owned boxes passed at ${owners} of 2 places · render/fogofwar.ts writes the gain and the lamps: ${wired}`);
    // … and that walk is common.wgsl's occludedTBoxes line for line, but for its head, the capsule cap and the owner rule (a fix to the one must reach the other)
    const lines = (t: string) => t.split('\n').map(l => l.trim()).filter(l => l.length > 0).slice(1);
    const A = lines(fnBody(common, 'occludedTBoxes')), B = lines(hit); const onlyA = A.filter(l => !B.includes(l)), onlyB = B.filter(l => !A.includes(l));
    const okA = ['if (tb.x > tb.y || tb.y < 0.0) { return tCap; }', 'let tmax = select(min(tmaxIn, tb.y), tCap, tCap >= 0.0);', 'if (skipsOwner(boxOwner(b), skipOwner)) { continue; }', 'if (r.x >= 0.0 && (bestInCell < 0.0 || r.x < bestInCell) && !fogThinBox(b, bi)) { bestInCell = r.x; }', 'return tCap;'];   // (… and a box a fog eye passes: FOG.thin — the lit fog's walk is a lamp's shadow, never an eye's)
    const okB = ['if (tb.x > tb.y || tb.y < 0.0) { return -1.0; }', 'let tmax = min(tmaxIn, tb.y);', 'if (boxOwner(boxGeo[bi]) != 0u) { continue; }', 'if (boxOwner(b) != 0u) { continue; }', 'if (r.x >= 0.0 && (bestInCell < 0.0 || r.x < bestInCell)) { bestInCell = r.x; }', 'return -1.0;'];
    const same = A.length > 30 && onlyA.every(l => okA.includes(l)) && onlyB.every(l => okB.includes(l)) && okB.every(l => B.includes(l));
    verdict(same, 'the lit fog\'s shadow walk (fowLitHit) is occludedTBoxes line for line, but for the capsule cap it does not take and the owned boxes it passes',
      `${A.length} lines there, ${B.length} here · only there: ${onlyA.length} (${onlyA.filter(l => !okA.includes(l)).length} not on the list) · only here: ${onlyB.length} (${onlyB.filter(l => !okB.includes(l)).length} not on the list)`);
  }
  Object.assign(FOG, FOG_DEFAULTS);
  return done();
}

// ================================================================================================== fog-full
// THE FOG GETS THE FULL PICTURE, LESS WHAT THE TEAM SHOULD NOT SEE (game/fogofwar.ts rule 4g, FOG.full). Headless this can ask everything but the picture: the
// classes the lights are handed over with, the words the renderer uploads, the lamp memory, and the text of every shader that evaluates a light. What the
// renderer makes of them is tools/qa/fog-lit-stills.cjs --torches (a real page, the software adapter): fog pixels that move when the enemies' torches come on.
//   FOGFULL_BREAK=1 → the fog is put off before anything is asked: no class on any light — the first four lines FAIL.
// The row, the own light and the lit fog are settled and no longer written. Until 2026-11-07 the probe also asked 'with the row off the block is the lit
// fog's alone, and no light and no team is left with the upload', ?fogfull on the address, a recording from before the key and the two stands.
export async function fogFull(): Promise<boolean> {
  const { verdict, done } = family('fog-full: the fog is the whole picture dimmed, and a light the team should not see there is cut where it is made');
  const brk = process.env.FOGFULL_BREAK === '1';
  const R = await rig(53, { fog: true }); const { game, FW, W } = R;
  const { LIGHT_FOG_SEEN, LIGHT_FOG_UNSEEN } = await import(`${ROOT}/src/render/lights.ts`);
  if (brk) FW.on = false;
  const F = FW.frame, L = W.engine.lights, all = (): any[] => L.lights;
  const lightsOf = (t: any): any[] => [...(t.light ? [t.light] : []), ...((t.areaLights ?? []).map((a: any) => a.light))];
  const fixtures: any[] = game.targets.filter((t: any) => t.light || t.areaLights);
  const word = (l: any): number => l.slot >= 0 ? (L as any).u32[l.slot * 16 + 3] >>> 0 : -1;
  const foes: any[] = game.guards.filter((gd: any) => !FW.onTeam(gd) && gd.flashlight), mates: any[] = game.guards.filter((gd: any) => FW.onTeam(gd) && gd.flashlight);
  { // the block
    const fsrc = await import('node:fs'); const up: string = fsrc.readFileSync(`${ROOT}/src/render/fogofwar.ts`, 'utf8'), eng: string = fsrc.readFileSync(`${ROOT}/src/engine.ts`, 'utf8');
    R.step(30); const on1 = F.full, fill1 = F.fullFlat, flat1 = F.flat, lit1 = F.lit, n1 = F.lightCount, team1 = L.fogTeam.on, fogOn = F.on && F.count > 0;
    FOG.fullFill = 0.5; R.step(3); const fillHalf = F.fullFlat; FOG.fullFill = 0;
    R.step(5);
    const picks = /const full = F\.full && F\.sight, flat = full \? F\.fullFlat : F\.flat;/.test(up) && /U\[o \+ 14\] = full \? 1 : 0;/.test(up) && /this\.fogFrame\.sight = this\.fogSight\.live;/.test(eng);
    verdict(fogOn && on1 === true && team1 === true && fill1 === 0 && Math.abs(fillHalf - FOG.flat * 0.5) < 1e-9 && flat1 === FOG.flat && lit1 > 0 && n1 > 0 && picks,
      'under the fog the block asks for the full picture, with its fill as a share of the own light (none unless asked), and still carries the lit fog\'s rows — which the pass draws for as long as the renderer\'s sight table is not live (so nothing live shows while it compiles, or if it never does)',
      `fog on ${F.on}, ${F.count} eyes · row on: full ${on1}, the team handed to the upload ${team1}, fill ${fill1}, at share 0.5 → ${fillHalf.toFixed(3)} of ${FOG.flat}, the lit fog's rows kept: own light ${flat1}, gain ${lit1}, ${n1} lamps · the pass picks by F.full && F.sight, which the engine writes from the table: ${picks}`);
  }
  { // the classes: what an enemy carries and every flash that is not a teammate's own light only what the team sees; the building's lamps, the sky and the team's own lights are as they were
    for (const gd of [...foes, ...mates]) { gd.flashlight.enabled = true; gd.flashlight.intensity = Math.max(gd.flashlight.intensity, 50); }
    const at: [number, number, number] = [game.player.char.pos[0], 1.2, game.player.char.pos[2]];
    R.step(1);   // (the three flashes below are born AFTER the fog's turn, as a tracer's ember and a blast's afterglow are: they must be classed all the same on the frame they are born)
    for (const gd of [...foes, ...mates]) { gd.flashlight.enabled = true; gd.flashlight.intensity = Math.max(gd.flashlight.intensity, 50); }
    const blast = L.flash(at, [1, 0.8, 0.5], 400, 12, 5, 0), mine = L.flash(at, [1, 0.8, 0.5], 400, 12, 5, game.player.char.id), theirs = foes[0] ? L.flash(at, [1, 0.8, 0.5], 400, 12, 5, foes[0].char.id) : null;
    const cls = (l: any): number => L.fogClass(l);
    const foeCls = foes.filter((gd: any) => cls(gd.flashlight) === 1).length, mateCls = mates.filter((gd: any) => cls(gd.flashlight)).length, samCls = game.player.light ? cls(game.player.light) : 0;
    const sky = all().filter((l: any) => l.kind === 2), skyCls = sky.filter((l: any) => cls(l)).length;
    const lamps = fixtures.flatMap(lightsOf), lampCls = lamps.filter((l: any) => cls(l) || l.fogGhost).length;
    L.upload(game.time);
    const seenBit = (l: any) => word(l) >= 0 && (word(l) & LIGHT_FOG_SEEN) !== 0, anyBit = (l: any) => word(l) >= 0 && (word(l) & (LIGHT_FOG_SEEN | LIGHT_FOG_UNSEEN)) !== 0;
    const foeUp = foes.filter((gd: any) => seenBit(gd.flashlight)).length, clean = [...mates.map((gd: any) => gd.flashlight), ...sky, ...lamps, mine, ...(game.player.light ? [game.player.light] : [])].filter(anyBit).length;
    const ok = foes.length > 0 && foeCls === foes.length && mateCls === 0 && samCls === 0 && skyCls === 0 && lampCls === 0 && seenBit(blast) && !anyBit(mine) && word(mine) >= 0 && (!theirs || seenBit(theirs)) && foeUp === foes.length && clean === 0;
    const flashes = `a blast's flash ${seenBit(blast)}, Sam's own ${anyBit(mine)}, an enemy's ${theirs ? seenBit(theirs) : '—'}`;
    L.remove(blast); L.remove(mine); if (theirs) L.remove(theirs);
    verdict(ok, 'every enemy\'s torch, a blast\'s flash and an enemy\'s muzzle flash are uploaded as lights of what the team sees alone — the flashes on the very frame they are born, after the fog\'s own turn; a teammate\'s torch, Sam\'s own light and flash, the sun or moon and every lamp of the building as it stands carry no class',
      `${foes.length} enemies with a torch, classed ${foeCls}, uploaded with the bit ${foeUp} · ${mates.length} teammates' torches classed ${mateCls}, Sam's ${samCls} · born after the fog's turn, uploaded with the bit: ${flashes} · ${sky.length} sky lights classed ${skyCls} · ${lamps.length} lamps' lights classed ${lampCls} · words with a bit among those that should have none: ${clean}`);
  }
  { // last known: a lamp put out where nobody looks is still lit in the fog's picture (and only there); once the team has a look it is out; lit again unseen it lights only what the team sees
    const litOnes = fixtures.filter((t: any) => t.on && !t.broken && lightsOf(t).some((l: any) => l.enabled && l.intensity > 0));
    const far = litOnes.find((t: any) => !FW.lampSeen(t)), near = litOnes.find((t: any) => FW.lampSeen(t));
    let ghost = false, up = false, nearGhost = false, held = 0;
    if (far) {
      game.lightHome.stage(far, false, null); R.step(3); L.upload(game.time); const ls = lightsOf(far); held = FW.fullStats.held;
      ghost = ls.every((l: any) => !(l.enabled && l.intensity > 0) && l.fogGhost > 0);
      up = ls.every((l: any) => word(l) >= 0 && (word(l) & LIGHT_FOG_UNSEEN) !== 0 && (L as any).f32[l.slot * 16 + 8] + (L as any).f32[l.slot * 16 + 9] + (L as any).f32[l.slot * 16 + 10] > 0);
      game.lightHome.stage(far, true, null);
      FW.reset(); R.step(3);
    }
    if (near) { game.lightHome.stage(near, false, null); R.step(2); L.upload(game.time); nearGhost = lightsOf(near).some((l: any) => l.fogGhost > 0 || l.slot >= 0); game.lightHome.stage(near, true, null); R.step(2); }
    verdict(!!far && ghost && up && held >= 1 && !nearGhost, 'a lit lamp switched off where the team has no look at it is uploaded all the same, at its own strength, as a light of what the team does NOT see (and counted as held); a lamp the team sees is simply out',
      `${litOnes.length} lit fixtures, one unseen: ${far ? far.name : '—'} · off, unseen → kept for the fog: ${ghost}, uploaded with the bit and its light: ${up} (held ${held}) · a seen one (${near ? near.name : 'none in sight: not asked'}) off → kept: ${nearGhost}`);
  }
  { // the mains are as they are: the trip is told to the whole floor
    const { setBlackout } = await import(`${ROOT}/src/game/lighting.ts`);
    const fed = fixtures.filter((t: any) => t.mains && t.on && !t.broken);
    setBlackout(game, 3600, 'debug'); R.step(3); L.upload(game.time); const ghosts = fed.filter((t: any) => lightsOf(t).some((l: any) => l.fogGhost > 0 || l.slot >= 0)).length;
    game.endBlackout(); R.step(3); const again = fed.filter((t: any) => lightsOf(t).some((l: any) => l.fog || l.fogGhost)).length;
    verdict(!brk && fed.length > 0 && ghosts === 0 && again === 0, 'with the mains tripped no mains-fed lamp is kept lit for the fog; with them back none carries a class',
      `${fed.length} mains-fed lamps · mains down, kept lit for the fog: ${ghosts} · mains back, with a class: ${again}`);
  }
  { // the shaders: every place the picture evaluates a light asks the class; the game's own reading does not
    const fs = await import('node:fs'); const dir = `${ROOT}/src/render/shaders`, rd = (f: string): string => fs.readFileSync(`${dir}/${f}`, 'utf8');
    const strip = (t: string) => t.split('\n').map(l => l.replace(/\/\/.*$/, '')).join('\n');
    const fnBody = (text: string, name: string) => { const a = text.indexOf(`fn ${name}(`), b = a < 0 ? -1 : text.indexOf('\n}', a); return a < 0 || b < 0 ? '' : strip(text.slice(a, b)); };
    const common = rd('common.wgsl'), ev = fnBody(common, 'evalLight'), tru = fnBody(common, 'evalLightTrue'), k = fnBody(common, 'lightFogK'), seen = fnBody(common, 'fogSeenAt');
    const gate = /evalLightRaw\(li, P, N\)/.test(ev) && /lightFogK\(li, P, N\)/.test(ev) && /LIGHT_FOG_UNSEEN/.test(tru) && /evalLightRaw\(li, P, N\)/.test(tru) && !/lightFogK|fogSeenAt/.test(tru) && /fogSeenAt\(P, N\)/.test(k) && /@group\(0\) @binding\(11\) var<uniform> fogSight: FogSightT;/.test(common) && /cfg\.y != 1\.0\) \{ return 1\.0; \}/.test(seen)
      && /if \(say\.z != say\.w\) \{[^}]*occludedTBoxes\(e\.xyz, [^;]*, say\.y, u32\(e\.w \+ 0\.5\), -1\.0\) < 0\.0; \}/.test(seen) && !/occludedTBoxes|fogSeenAt/.test(fnBody(common, 'fogSeenBins')) && !/occludedTBoxes/.test(fnBody(common, 'fogEyeSay'));
    const files: string[] = fs.readdirSync(dir).filter((f: string) => f.endsWith('.wgsl'));
    const raw = files.filter(f => f !== 'common.wgsl' && /evalLightRaw\(/.test(strip(rd(f)))), truth = files.filter(f => f !== 'common.wgsl' && /evalLightTrue\(/.test(strip(rd(f))));
    const own = ['reflect.wgsl', 'cards.wgsl'].filter(f => !/lightFogK\(/.test(strip(rd(f))));   // the places that work a light out for themselves (a glint, a torch's bounce card where its footprint falls)
    const frameTs: string = fs.readFileSync(`${ROOT}/src/render/frame.ts`, 'utf8'), up: string = fs.readFileSync(`${ROOT}/src/render/fogofwar.ts`, 'utf8'), fog = rd('fogofwar.wgsl'), sight = rd('fogsight.wgsl');
    const bins = (t: string, re: RegExp) => Number((t.match(re) ?? [])[1]);
    const vol = strip(rd('volumetrics.wgsl')), draw = strip(rd('particles_draw.wgsl')), smokeFn = fnBody(common, 'fogSmoke');
    const air = (vol.match(/let dl = dustDensity\(X, onMask, true\); let dens = fogSmoke\(dl\.d, X\);/g) ?? []).length === 1 && !/let dens = (smokeDensity|dl\.d;)/.test(vol) && /fogSeenAt\(X, vec3f\(0\.0\)\)/.test(smokeFn)
      && /if \(\(li\.kind & \(?LIGHT_FOG_UNSEEN(?: \| LIGHT_[A-Z_]+)*\)?\) != 0u\) \{ continue; \}/.test(fnBody(common, 'directIrradiance'))   // (other kinds may share the skip: an infrared light does)
      && /if \(kind != P_RAIN && \(p\.bits & PB_OURS\) == 0u && !\(\(p\.bits & PB_KEPT\) != 0u && fogKeptOn\(\)\) && fogSeenBins\(p\.pos\) < 0\.5\) \{ return cull\(\); \}/.test(draw) && /flux \*= lightFogK\(li, x, hit\.n\);/.test(strip(rd('cards.wgsl'))) && /fk\[q\] = lightFogK\(li, ro \+ rd \* select\(select\(s1, 0\.5 \* \(s0 \+ s1\), q == 1u\), s0, q == 0u\), vec3f\(0\.0\)\);/.test(vol);   // (the beam's three places, asked in one loop round one call)
    const wired = /binding: 11, visibility: all, buffer: \{ type: 'uniform' \}/.test(frameTs) && /binding: 11, resource: \{ buffer: b\.fogSight \}/.test(frameTs) && /U\[o \+ 14\] = full \? 1 : 0;/.test(up)
      && /unseen = select\(own, unseen \+ own, fow\.lit2\.z > 0\.5\);/.test(strip(fog)) && /let rd = vec3f\(cos\(a\), 0\.0, sin\(a\)\);\s*t = occludedTBoxes\(e\.xyz, rd, 1e9, u32\(e\.w \+ 0\.5\), -1\.0\);/.test(strip(sight))
      && bins(common, /const FOG_SIGHT_BINS: i32 = (\d+);/) === bins(up, /export const FOG_BINS = (\d+);/) && bins(common, /dist: array<vec4f, (\d+)>/) * 4 === bins(up, /export const FOG_BINS = (\d+);/) * bins(up, /export const FOG_EYES = (\d+);/) && /sightOut\[k \* u32\(FOG_SIGHT_BINS\) \+ x\] = t;/.test(strip(sight));
    verdict(gate && raw.length === 0 && truth.length === 1 && truth[0] === 'probe.wgsl' && own.length === 0 && wired && air,
      'the picture\'s evalLight is the light under its fog class (asked of the sight table; 1 while none is written) and the game\'s reading (probe.wgsl alone) is the light as it is; the glints and the torch cards (where the footprint falls) ask the class themselves, and a beam the class cuts takes no place among a ray\'s beams, and a lamp lit only in the fog\'s picture gives the cascades (whose bounce the meter reads) nothing; the table is bound to every pass (binding 11, a uniform), written by the eye ray the fog pass walks — which is walked again wherever the table\'s two rays beside a spot disagree — and the fog pass adds its fill to the live frame under the full picture; the picture\'s march takes every plume\'s density through fogSmoke (which cuts it over floor the table says nobody sees unless the head says the air is drawn there — fog-air), and no particle but the weather\'s and the team\'s own doing (fog-ours) is drawn over such floor; ',
      `evalLight / evalLightTrue / lightFogK as written: ${gate} · other files calling evalLightRaw: ${raw.join(', ') || 'none'} · calling evalLightTrue: ${truth.join(', ') || 'none'} · own-work files without the class: ${own.join(', ') || 'none'} · binding, block, fill and table ray: ${wired} · smoke (the one march's read), particles, the card and the beams' places: ${air}`);
  }
  Object.assign(FOG, FOG_DEFAULTS);
  return done();
}

// ================================================================================================== fog-identity
// ================================================================================================== fog-cone
/** (HARD.cone, the calls and the pair starting fallen in are settled on and no longer written. Until 2026-11-07 this probe also asked 'HARD.cone off = hard
 *  without it' and 'nothing with the calls off', put the calls off while it turned Sam to face the man, and stood the pair on the staging's own marks
 *  (START.fallIn held off): the calls and the team's cones are now asked of the pair as the page starts them, fallen in.) */
export async function fogCone(): Promise<boolean> {
  const { verdict, done } = family('fog-cone: on hard an eye is a cone — a man behind every back is fog; standard is all round, as it was');
  const { HARD, setDifficulty, difficultyNow, difficultyWords, setConeVisit, difficultyHere } = await import(`${ROOT}/src/game/difficulty.ts`);
  const { PRESSURE } = await import(`${ROOT}/src/game/pressure.ts`); const { DIGIN } = await import(`${ROOT}/src/game/digin.ts`);
  const HARD0 = { ...HARD }, P0 = JSON.parse(JSON.stringify(PRESSURE)), D0 = JSON.parse(JSON.stringify(DIGIN));
  const DEGR = Math.PI / 180;
  try {
    { // the level writes the row
      setDifficulty('standard', false); const std = { cone: FOG.cone, own: FOG.own };
      setDifficulty('hard', false); const hard = { cone: FOG.cone, own: FOG.own, now: difficultyNow(), words: difficultyWords('hard') };
      FOG.cone = false; const still = difficultyNow();   // (the Tab row flipped by hand: the level's name stays)
      FOG.own = true; setDifficulty('hard', false); const own = { cone: FOG.cone, own: FOG.own }; setDifficulty('standard', false); const ownStd = FOG.own; FOG.own = false;   // (the option is nobody's level: neither writes it)
      // for a visit (?fogcone / ?fogcone=0, the Tab row): it stands through a level written again and a scenario's start
      setConeVisit(true); setDifficulty('standard', false); const vOnStd = FOG.cone; setConeVisit(false); setDifficulty('hard', false); difficultyHere('the depot'); const vOffHard = FOG.cone; setConeVisit(null); const vNone = FOG.cone;
      verdict(vOnStd && !vOffHard && vNone, 'for a visit: the cone asked for on standard stands through the level written again, the cone refused on hard stands through a scenario\'s start; let go, the level decides again',
        `?fogcone then standard → ${vOnStd} · ?fogcone=0 then hard + a start → ${vOffHard} · let go on hard → ${vNone}`);
      Object.assign(HARD, HARD0); setDifficulty('standard', false); const back = { cone: FOG.cone, own: FOG.own };
      verdict(!std.cone && !std.own && hard.cone && !hard.own && hard.now === 'hard' && still === 'hard' && own.cone && own.own && ownStd && !back.cone && !back.own && /cone/.test(hard.words) && !/cone/.test(difficultyWords('standard')),
        'standard leaves the row down, hard raises it (and says so in its words); the own-eyes option is no level\'s (neither writes it); back at standard the cone is down again',
        `standard cone ${std.cone} · hard cone ${hard.cone} own ${hard.own} (reads '${hard.now}', '${still}' with the row flipped by hand) · the option on: hard leaves own ${own.own}, standard ${ownStd} · standard again ${back.cone}`);
    }
    const R = await rig(41, { fog: true }); const { game, FW, NB } = R; const P = game.level.breach; const W = R.W;
    R.step(30);
    { // the renderer's block
      const a = { cos: FW.frame.looks[2], near: FW.frame.looks[3], si: FW.frame.softIn, so: FW.frame.softOut, ns: FW.frame.nearSoft, eyes: FW.eyes.length };
      FOG.cone = true; R.step(2);
      const b = { cos: FW.frame.looks[2], near: FW.frame.looks[3], si: FW.frame.softIn, so: FW.frame.softOut, ns: FW.frame.nearSoft, eyes: FW.eyes.length };
      FOG.own = true; R.step(2); const eyesOwn = FW.eyes.length, countOwn = FW.frame.count; FOG.own = false; R.step(2);
      const half = FOG.coneDeg / 2;
      verdict(a.cos <= -1.5 && a.si === 0 && a.so === 0 && a.ns === 0 && Math.abs(b.cos - Math.cos(half * DEGR)) < 1e-5 && b.near === FOG.coneNearM && b.si > b.cos && b.cos > b.so && Math.abs(b.si - Math.cos((half - FOG.coneSoftDeg / 2) * DEGR)) < 1e-5 && b.ns === FOG.coneNearSoftM && a.eyes === 3 && b.eyes === 3 && eyesOwn === 1 && countOwn === 1,
        'the block: all round and no soft edge without the cone; with it the wedge\'s cosine, the near field and a soft band either side of the edge; FOG.own leaves one eye',
        `without: cos ${a.cos}, soft ${a.si}/${a.so}/${a.ns} · with: cos ${b.cos.toFixed(3)} (${FOG.coneDeg}°), near ${b.near} m, soft ${b.si.toFixed(3)} > ${b.cos.toFixed(3)} > ${b.so.toFixed(3)}, rim ${b.ns} m · eyes ${a.eyes} → own ${eyesOwn}`);
    }
    // THE ORACLE for the cone, written here: a living man is seen when some eye has a clear line to his chest or head AND he stands inside that eye's near field or
    // within half the cone of where it looks (the eyes' places and looks are the module's; the angle and the lines are asked here)
    const coneOracle = (h: any) => { const chest = h.char.bones.chest ?? [h.char.pos[0], 1.1, h.char.pos[2]], hd = h.char.bones.head ?? [h.char.pos[0], 1.6, h.char.pos[2]];
      const per = FW.eyes.map((e: any) => { const dx = chest[0] - e.pos[0], dz = chest[2] - e.pos[2], d = Math.hypot(dx, dz); const ang = Math.acos(Math.max(-1, Math.min(1, (dx * e.look[0] + dz * e.look[2]) / (d * Math.hypot(e.look[0], e.look[2]))))) / DEGR;
        const line = !(game.col.segmentBlocked(e.pos, chest) && game.col.segmentBlocked(e.pos, hd)); return { id: e.id, d, ang, line, sees: line && (d < FOG.coneNearM || ang < FOG.coneDeg / 2) }; });
      return { any: per.some((x: any) => x.sees), per }; };
    const say = (o: any) => o.per.map((x: any) => `eye ${x.id}: ${x.ang.toFixed(0)}° off, ${x.d.toFixed(1)} m, line ${x.line} → ${x.sees}`).join(' · ');
    R.openDoor(P.door); R.step(2);
    const hs = NB.hostiles as any[]; const h = hs.find(x => x.state !== 'dead' && R.spotFor(x));
    if (!h) verdict(false, 'the cone: a living man in the line to test it on', 'no spot in the aisle with a line to a living defender through the open door');
    else {
      game.godMode = true; const pc = game.player.char; const at = () => h.char.bones.chest ?? [h.char.pos[0], 1.1, h.char.pos[2]];
      const face = (away: boolean) => { const c = at(); const dx = c[0] - pc.pos[0], dz = c[2] - pc.pos[2], d = Math.hypot(dx, dz) || 1, k = away ? -3 : 1; for (let i = 0; i < 50; i++) { W.cursorAt(away ? [pc.pos[0] + dx / d * k, 1.1, pc.pos[2] + dz / d * k] : c); R.step(); } };
      // Sam alone (FOG.own): facing the man, then with his back to him
      FOG.cone = true; FOG.own = true;
      face(false); const o1 = coneOracle(h), s1 = FW.seen(h), pic1 = R.inPicture(h);
      face(true); R.step(30); const o2 = coneOracle(h), s2 = FW.seen(h), d2 = FW.drawn(h), pic2 = R.inPicture(h);
      verdict(o1.any && s1 && pic1.inst && !o2.any && !s2 && !d2 && !pic2.inst && !pic2.caps && !pic2.rig && o2.per[0].line && o2.per[0].ang > FOG.coneDeg / 2 && o2.per[0].d > FOG.coneNearM,
        'his own eyes alone: facing the man he is seen and drawn; his back turned on the same clear line, the man is fog — not drawn, no proxies, no gun',
        `facing: ${say(o1)} → module ${s1} · back turned: ${say(o2)} → module seen ${s2} drawn ${d2}, in the picture ${JSON.stringify(pic2)}`);
      // all round again (the cone off) with his back still turned: seen — the status quo
      FOG.cone = false; R.step(2); const sStd = FW.seen(h); FOG.cone = true; R.step(2);
      verdict(sStd && !FW.seen(h), 'the same frame without the cone (standard): he is seen with Sam\'s back to him', `cone off → seen ${sStd} · cone on → seen ${FW.seen(h)}`);
      // the near field: inside it all round
      const near0 = FOG.coneNearM; FOG.coneNearM = 50; R.step(2); const sNear = FW.seen(h); FOG.coneNearM = near0; R.step(2);
      verdict(sNear && !FW.seen(h), 'inside the near field an eye sees all round', `near field 50 m → seen ${sNear} · ${near0} m → seen ${FW.seen(h)}`);
      { // (the sides: his own left hand is on his left) (until 2026-11-07 the calls were asked here too, red on the last TypeScript game as on this one)
        FOG.own = true; R.step(2); const aL = pc.bones.upperArmL, aR = pc.bones.upperArmR, gz = FW.eyes[0].look; let sideL = '?';   // (his left shoulder's side of the line he looks down: the across part of right shoulder → left shoulder)
        if (aL && aR) { const sx = aL[0] - aR[0], sz = aL[2] - aR[2], al = sx * gz[0] + sz * gz[2], px = sx - gz[0] * al, pz = sz - gz[2] * al, pl = Math.hypot(px, pz) || 1; sideL = FW.sideOf([pc.pos[0] + px / pl * 5, 1, pc.pos[2] + pz / pl * 5]); }
        verdict(sideL === 'left', 'the sides are the player\'s man\'s own: his left hand is on his left', `a point out to his left shoulder's side reads '${sideL}'`); }
      // the team's cones (own off): the module agrees with the oracle every frame while Sam turns away and back, and at some frame a teammate's cone alone shows him
      FOG.own = false; let agree = 0, frames = 0, mateOnly = 0, none = 0; const bad: string[] = [];
      for (const away of [true, false, true]) { const c0 = at(); const dx = c0[0] - pc.pos[0], dz = c0[2] - pc.pos[2], d = Math.hypot(dx, dz) || 1, k = away ? -3 : 1;
        for (let i = 0; i < 70; i++) { W.cursorAt(away ? [pc.pos[0] + dx / d * k, 1.1, pc.pos[2] + dz / d * k] : at()); R.step(); if (h.state === 'dead') break; const o = coneOracle(h), s = FW.seen(h); frames++; if (o.any === s) agree++; else if (bad.length < 3) bad.push(`t ${game.time.toFixed(2)}: oracle ${o.any} module ${s} (${say(o)})`); if (o.any && !o.per[0].sees) mateOnly++; if (!o.any) none++; } }
      verdict(frames > 0 && agree === frames, 'the team\'s cones together: the module agrees with the oracle every frame while Sam turns away, back and away again',
        bad.length ? bad.join('\n    ') : `${agree}/${frames} frames · a teammate\'s cone alone showed him on ${mateOnly} · nobody\'s on ${none}`);
      // Sam's gaze with a rifle in the shoulder lies along his aim
      { FOG.own = true; W.input.pressed.add('Digit2'); R.step(); for (let i = 0; i < 100; i++) { W.cursorAt(at()); R.step(); }
        const e = FW.eyes[0]; const ap = game.aimPoint; const ax = ap[0] - pc.pos[0], az = ap[2] - pc.pos[2]; const wrap = (a: number) => ((a + 540) % 360) - 180, yaw = (x: number, z: number) => Math.atan2(x, z) / DEGR;
        const hdv = pc.headDir(), lerp = [(pc.chestDir[0] + hdv[0]) / 2, (pc.chestDir[2] + hdv[2]) / 2];
        const offGaze = Math.abs(wrap(yaw(e.look[0], e.look[2]) - yaw(ax, az))), offLerp = Math.abs(wrap(yaw(lerp[0], lerp[1]) - yaw(ax, az)));
        verdict(game.player.rifleUp && pc.anim.longShown === 'shoulder' && offGaze < 6 && FW.seen(h), 'a rifle in the shoulder: his cone lies along his aim (his eyes are on his sights), not along his bladed chest',
          `rifle up ${game.player.rifleUp}, stance ${pc.anim.longShown} · the cone\'s look is ${offGaze.toFixed(1)}° off his aim (chest and head between them: ${offLerp.toFixed(0)}° off) · the man he aims at seen ${FW.seen(h)}`); }
    }
  } finally { setConeVisit(null); Object.assign(HARD, HARD0); Object.assign(PRESSURE, P0); Object.assign(DIGIN, D0); Object.assign(FOG, FOG_DEFAULTS); }
  return done();
}

export async function fogIdentity(): Promise<boolean> {
  const { verdict, done } = family('fog-identity: it reads and never writes — the fight and the night floor digest the same with it on as off');
  const fight = async (seed: number, fog: boolean) => {
    const R = await rig(seed, { fog, chaos: 0.5 }); const { NB, game } = R;
    R.step(40); NB.order(); R.step(60 * 7); NB.order(); R.step(60 * 14);
    return { d: JSON.stringify(digestGame(game)), draws: (R.dice as any).draws(), state: NB.state as string, hid: R.FW.stats.hidden, frames: R.FW.stats.frames };
  };
  const rows: string[] = []; let same = true, ran = true;
  for (const seed of [21, 22, 23]) { const a = await fight(seed, false), b = await fight(seed, true); same &&= a.d === b.d && a.draws === b.draws; ran &&= b.frames > 0 && a.frames === 0; rows.push(`seed ${seed}: ${a.state} / ${b.state}, dice ${a.draws} / ${b.draws}, digest ${a.d === b.d ? 'same' : 'DIFFERENT'} (${a.d.length} B)`); }
  verdict(same && ran, 'the default night breach, three seeds, 21 s each at chaos 0.5: digest and dice the same on as off', rows.join(' · '));
  const floor = async (fog: boolean) => {
    const R = await rig(31, { fog, breach: false }); const { game, FW } = R; let hid = 0, tested = 0;
    for (let i = 0; i < 60 * 180; i++) { R.step(); hid += FW.stats.hidden * (fog ? 1 : 0); tested = Math.max(tested, FW.stats.tested); }
    return { d: JSON.stringify(digestGame(game)), draws: (R.dice as any).draws(), hid, tested, eyes: FW.eyes.length };
  };
  const a = await floor(false), b = await floor(true);
  verdict(a.d === b.d && a.draws === b.draws && b.hid > 0 && b.eyes === 1, 'three minutes of the ordinary night floor (the shift and the civilians, Sam alone as the eye): the same on as off — while men were being hidden', `digest ${a.d === b.d ? 'same' : 'DIFFERENT'} (${a.d.length} B) · dice ${a.draws} / ${b.draws} · judged ${b.tested} men a frame, hidden man-frames ${b.hid} · eyes ${b.eyes}`);
  if (VERBOSE) console.log(a.d.slice(0, 400));
  return done();
}

// THE FOG'S SOFT EDGE (game/fogofwar.ts FOG.edge — the maintainer, 10-15: 'fog of war edges shouldn't be so sharp') and THE FOG KEEPS ITS COLOUR (FOG.desat 0 —
// the same day: 'having fog of war mute the colors is unfortunate'). The band is drawn by the fog pass alone (shaders/fogofwar.wgsl fowEdge, over the sight
// table), so headless this asks (1) what reaches the renderer, (2) how it is wired — that no program that lights a pixel carries it, and that the shader's text
// is the one the replica below was written against (its hash) — and (3) THE RULES THEMSELVES on a plan replica of fowEdge, statement for statement, over boxes
// in plan (edgeReplica: change it with the shader): the answer is the hard line's wherever no sight edge is near; it crosses a line cast over open floor
// smoothly, 0.5 on the line; a wall keeps its line on both of its sides; the line a pillar casts up a far wall fades; a man coming round a corner is not dimmed
// much and a pocket nobody sees stays dark; a stack of men has one man's line; nothing brightens behind a partition; and next to no hard step is left (pairs of spots 2 cm apart). Then the address, the
// journal and the stands. What the renderer makes of it is tools/qa/fog-lit-stills.cjs --edge (a real page, the software adapter).
//   FOGEDGE_BREAK=1 → the fog is put off before the block is read, and the replica is run with no band: the block's line and the replica's line FAIL.
// The band and the fog's colour are settled and no longer written. Until 2026-11-07 the probe also asked the block at other widths (0.8, −1, 0: 'the hard
// line'), ?fogedge / ?fogdesat on the address, a recording from before the keys replayed under the old look, and the two stands.
// What the replica is NOT: the cones (every eye all round), eye heights and the top-face pad, a man's own kit skipped, f32 arithmetic, and the grid walk.
type PlanBox = { x0: number; z0: number; x1: number; z1: number };
function edgeReplica(HMAX: number, JLO: number, JHI: number, SLANT: number, RUN: number, THIN: number) {
  const BINS = 2048, TAU = Math.PI * 2;
  const rayBoxes = (boxes: PlanBox[], ox: number, oz: number, dx: number, dz: number, tmax: number): number => { let best = -1; for (const b of boxes) { const ix = 1 / (dx || 1e-30), iz = 1 / (dz || 1e-30);
    const ta = (b.x0 - ox) * ix, tb = (b.x1 - ox) * ix, tc = (b.z0 - oz) * iz, td = (b.z1 - oz) * iz, tN = Math.max(Math.min(ta, tb), Math.min(tc, td)), tF = Math.min(Math.max(ta, tb), Math.max(tc, td));
    if (tN > tF || tF < 0 || tN > tmax) continue; const t = tN < 0 ? 0 : tN; if (best < 0 || t < best) best = t; } return best; };
  // (fogsight.wgsl: a bearing's level ray to the NEAREST box, or the world's edge)
  const table = (boxes: PlanBox[], e: number[]): Float32Array => { const D = new Float32Array(BINS); for (let j = 0; j < BINS; j++) { const a = (j + 0.5) / BINS * TAU - Math.PI, t = rayBoxes(boxes, e[0], e[1], Math.cos(a), Math.sin(a), 1e9); D[j] = t < 0 ? 1e9 : t; } return D; };
  const sstep = (a: number, b: number, x: number) => { const t = Math.min(1, Math.max(0, (x - a) / (b - a))); return t * t * (3 - 2 * t); };
  const bin = (D: Float32Array, j: number) => Math.max(D[j & (BINS - 1)], 1e-3);   // (fowBin)
  // (fowRel)
  const clamp = (x: number, a: number, b: number) => Math.max(a, Math.min(b, x));
  const rel = (D: Float32Array, b: number, s: number): number => { const ib = 1 / bin(D, b), ic = 1 / bin(D, b - s);
    const before = ic + clamp(ic - 1 / bin(D, b - 2 * s), -RUN * ic, RUN * ic), after = ib + clamp(ib - 1 / bin(D, b + s), -RUN * ib, RUN * ib);
    return Math.min(ib - before, after - ic) / ib; };
  // (fowFirstShort)
  const firstShort = (D: Float32Array, j0: number, s: number, H: number, need: number): number[] => { let o = 0;
    for (let it = 0; it <= HMAX; it++) { if (o > H) break; const b = j0 + s * o, bw = b & (BINS - 1);
      if ((bw & 3) === (s > 0 ? 0 : 3) && o + 3 <= H) { const g = bw & ~3; if (D[g] >= need && D[g + 1] >= need && D[g + 2] >= need && D[g + 3] >= need) { o += 4; continue; } }
      const rb = bin(D, bw); if (rb < need) { const r = rel(D, b, s); if (r > JLO) return [o, rb, r]; }
      o += 1; }
    return [-1, 0, 0]; };
  // (fowFirstClear)
  const firstClear = (D: Float32Array, j0: number, s: number, H: number, need: number, perBin: number): number => { let o = 0;
    for (let it = 0; it <= HMAX; it++) { if (o > H) break; const bw = (j0 + s * o) & (BINS - 1);
      if ((bw & 3) === (s > 0 ? 0 : 3) && o + 3 <= H) { const g = bw & ~3, l4 = need - SLANT * (o + 4) * perBin; if (D[g] < l4 && D[g + 1] < l4 && D[g + 2] < l4 && D[g + 3] < l4) { o += 4; continue; } }
      if (D[bw] >= need - SLANT * (o + 1) * perBin) return o;
      o += 1; }
    return -1; };
  // (fowEdge)
  const edge = (D: Float32Array, e: number[], q: number[], need: number, clear: boolean, band0: number): number => {
    const hard = clear ? 1 : 0; if (band0 <= 0) return hard;
    const tox = q[0] - e[0], toz = q[1] - e[1], d = Math.hypot(tox, toz); if (d < 0.05) return hard;
    const perBin = d * TAU / BINS, H = Math.max(1, Math.min(Math.ceil(0.5 * band0 / perBin), HMAX));
    const u = (Math.atan2(toz, tox) / TAU + 0.5) * BINS - 0.5, j0 = Math.floor(u + 0.5), fr = u - j0;
    const most = 2 * H * perBin; let v = hard;
    for (let side = 0; side < 2; side++) { const s = side === 1 ? 1 : -1;
      if (clear) { const f = firstShort(D, j0, s, H, need); if (f[0] < 0) continue;
        const be = Math.min(most, band0 * sstep(JLO, JHI, f[2]) * sstep(0, 1, (need - f[1]) / band0)); if (be < 1e-4) continue;
        v = Math.min(v, 1 + (sstep(0, 1, 0.5 + Math.max(0, f[0] - 0.5 - s * fr) * perBin / be) - 1) * sstep(THIN / 3, THIN, be / perBin));
      } else { const o = firstClear(D, j0, s, H, need, perBin); if (o < 0) continue;
        const c = j0 + s * o, rb = bin(D, c - s); if (rb >= need) continue;
        const ic = 1 / bin(D, c); if ((ic + o * clamp(ic - 1 / bin(D, c + s), -RUN * ic, RUN * ic)) * need > 1) continue;
        const be = Math.min(most, band0 * sstep(JLO, JHI, rel(D, c - s, -s)) * sstep(0, 1, (need - rb) / band0)); if (be < 1e-4) continue;
        v = Math.max(v, sstep(0, 1, 0.5 - Math.max(0, o - 0.5 - s * fr) * perBin / be) * sstep(THIN / 3, THIN, be / perBin)); } }
    return v; };
  // (fowSeen, every eye all round: the pixel's own ray, then the band — the most any eye gives)
  const seen = (boxes: PlanBox[], Ds: Float32Array[], eyes: number[][], q: number[], band0: number, pad = 0.06): number => { let best = 0;
    for (let k = 0; k < eyes.length; k++) { const e = eyes[k], tx = q[0] - e[0], tz = q[1] - e[1], d = Math.hypot(tx, tz); if (d <= pad + 0.02) return 1;
      const clear = rayBoxes(boxes, e[0], e[1], tx / d, tz / d, d - pad) < 0;
      best = Math.max(best, edge(Ds[k], e, q, d - pad, clear, band0)); if (best >= 1) return 1; }
    return best; };
  return { table, seen };
}
/** the shader text the replica above mirrors (fogofwar.wgsl from the soft edge's constants to the end of fowSeen), white space squeezed: a change there without
 *  the replica, and this number, changed with it is red */
const EDGE_TEXT_HASH = 1049378601;
export async function fogEdge(): Promise<boolean> {
  const { verdict, done } = family('fog-edge: the fog comes in over a band across a sight edge, not on a hard line, and keeps its colour');
  const brk = process.env.FOGEDGE_BREAK === '1';
  const R = await rig(53, { fog: true }); const { FW } = R;
  const { FOG_EDGE_ON } = await import(`${ROOT}/src/game/fogofwar.ts`);
  const F = FW.frame;
  if (brk) FW.on = false;
  { // the block
    R.step(20); const on1 = F.edge, fogOn = F.on && F.count > 0, grey = F.desat;
    verdict(fogOn && on1 === FOG_EDGE_ON && on1 > 0 && grey === 0 && (FOG_DEFAULTS as any).desat === 0, 'the fog\'s block carries the band every frame, the page\'s width — and no greying: the fog keeps its colour',
      `fog on: ${fogOn} · the band in the block: ${on1} (the page's ${FOG_EDGE_ON}) · the greying in the block: ${grey} (the module's ${(FOG_DEFAULTS as any).desat})`);
  }
  const fsrc = await import('node:fs'); const rd = (f: string): string => fsrc.readFileSync(`${ROOT}/src/render/${f}`, 'utf8');
  const pass = rd('shaders/fogofwar.wgsl'); const num = (name: string): number => +(new RegExp(`const ${name}: (?:f32|i32) = ([0-9.]+);`).exec(pass)?.[1] ?? NaN);
  const HMAX = num('FOW_EDGE_BINS'), JLO = num('FOW_EDGE_JLO'), JHI = num('FOW_EDGE_JHI'), SLANT = num('FOW_EDGE_SLANT'), RUN = num('FOW_EDGE_RUN'), THIN = num('FOW_EDGE_THIN');
  { // the wiring, as written
    const up = rd('fogofwar.ts'), dir = `${ROOT}/src/render/shaders`, sight = rd('shaders/fogsight.wgsl');
    const head = /U\[o \+ 3\] = Math\.max\(0, F!\.edge\);/.test(up);
    const a = pass.indexOf('const FOW_EDGE_BINS'), b = pass.indexOf('// ---- THE FOG LIT AS THE TEAM LAST KNEW IT'), text = a >= 0 && b > a ? pass.slice(a, b).replace(/\s+/g, ' ').trim() : '';
    let h = 2166136261; for (let i = 0; i < text.length; i++) { h ^= text.charCodeAt(i); h = Math.imul(h, 16777619) >>> 0; }
    const offIsHard = /let hard = select\(0\.0, 1\.0, clear\);\s*let band0 = fogSight\.cfg\.w;\s*if \(band0 <= 0\.0 \|\| fogSight\.cfg\.y != 1\.0\) \{ return hard; \}/.test(pass) && (pass.match(/best = max\(best, w \* fowEdge\(i32\(k\), e, q\.xz, d - pad, (?:false|clear)\)\);/g) ?? []).length === 2;
    const others: string[] = (fsrc.readdirSync(dir) as string[]).filter(f => f.endsWith('.wgsl') && f !== 'fogofwar.wgsl' && /fowEdge\(|fowFirstShort\(|fowFirstClear\(/.test(fsrc.readFileSync(`${dir}/${f}`, 'utf8')));
    const nearest = /for \(var again = 0; again < 8 && t > 0\.002; again\+\+\) \{ let nearer = occludedTBoxes\(e\.xyz, rd, t - 0\.001, u32\(e\.w \+ 0\.5\), -1\.0\); if \(nearer < 0\.0\) \{ break; \} t = nearer; \}/.test(sight);
    const consts = [HMAX, JLO, JHI, SLANT, RUN, THIN].every(Number.isFinite);
    verdict(head && h === EDGE_TEXT_HASH && offIsHard && others.length === 0 && nearest && consts, 'the band rides the sight table\'s head; the shader\'s text is the one the replica was written against; at 0, or with no table written, every spot answers as the hard line did; no other program carries it — a light\'s fog class is cut on the line, as it was; and the sight table keeps the NEAREST box on a bearing',
      `the head's word: ${head} · the text's hash: ${h} (the replica's ${EDGE_TEXT_HASH}) · 0 or no table → the hard line: ${offIsHard} · other shaders naming it: ${others.join(', ') || 'none'} · the table asks again short of each answer: ${nearest} · the shader's numbers (bearings ${HMAX}, step ${JLO} ‥ ${JHI}, slant ${SLANT}, run ${RUN}, thin ${THIN}): ${consts}`);
  }
  { // the rules, on the plan replica
    const { table, seen } = edgeReplica(HMAX, JLO, JHI, SLANT, RUN, THIN), band = brk ? 0 : FOG_EDGE_ON;
    const box = (x0: number, z0: number, x1: number, z1: number): PlanBox => ({ x0, z0, x1, z1 });
    const line = (f: (t: number) => number[], a: number, b: number, st: number): number[][] => { const r: number[][] = []; for (let t = a; t <= b + 1e-9; t += st) r.push(f(t)); return r; };
    const ask = (boxes: PlanBox[], eyes: number[][], pts: number[][], bnd = band): number[] => { const Ds = eyes.map(e => table(boxes, e)); return pts.map(p => seen(boxes, Ds, eyes, p, bnd)); };
    const mono = (v: number[], up: boolean) => v.every((x, i) => i === 0 || (up ? x >= v[i - 1] - 1e-6 : x <= v[i - 1] + 1e-6)), f2 = (x: number) => x.toFixed(2);
    // a 0.4 m pillar 4 m off, open floor 8 m off: across one side of its shadow, from well inside the seen floor to the shadow's middle
    const pillar = [box(4, -0.2, 4.4, 0.2)], across = ask(pillar, [[0, 0]], line(z => [8, z], -0.8, -0.1, 0.02)), hardAcross = ask(pillar, [[0, 0]], line(z => [8, z], -0.8, -0.1, 0.02), 0);
    const iLine = hardAcross.indexOf(0), atLine = iLine > 0 ? [across[iLine - 1], across[iLine]] : [NaN, NaN];
    const fade = across[0] === 1 && mono(across, false) && atLine[0] > 0.5 && atLine[0] < 0.62 && atLine[1] <= 0.5 + 1e-6 && atLine[1] > 0.38 && across.filter(x => x > 0.05 && x < 0.95).length >= 12;
    const core = ask(pillar, [[0, 0]], line(z => [8, z], -0.1, 0.1, 0.02));   // the shadow's middle, 0.8 m wide there: a pocket nobody sees
    const beside = ask(pillar, [[0, 0]], [...line(x => [x, 0.3], 3.6, 4.0, 0.02), ...line(x => [x, -0.3], 3.6, 4.0, 0.02)]);   // floor beside the pillar's near face: seen, and the line it casts has not begun
    // walls: the floor at the foot of a wall seen along its length, and BEHIND it; a room's corner from a man at the wall; behind both of its walls
    const along = [box(-5, 1, 30, 1.3)], foot = [3, 10, 20].flatMap(x => ask(along, [[0, 0]], line(z => [x, z], 0.3, 0.95, 0.05))), behind = [3, 10, 20].flatMap(x => ask(along, [[0, 0]], line(z => [x, z], 1.37, 2.0, 0.05)));
    const corner = [box(10, -5, 10.3, 5.3), box(0, 5, 10, 5.3)], face = [...ask(corner, [[2, 4.6]], line(z => [9.94, z], 3, 4.9, 0.1)), ...ask(corner, [[2, 4.6]], line(x => [x, 4.94], 6, 9.9, 0.3)), ...ask(corner, [[2, 0]], line(t => [9.9 - t, 4.9 - t], 0, 0.6, 0.1))];
    const outside = [...ask(corner, [[2, 4.6]], line(z => [10.4, z], 3, 5.6, 0.2)), ...ask(corner, [[2, 4.6]], line(x => [x, 5.4], 6, 10.4, 0.4))];
    const wallKept = foot.every(x => x === 1) && face.every(x => x === 1) && behind.every(x => x === 0) && outside.every(x => x === 0);
    // a pillar's shadow up a far wall seen at a slant: both of its edges, from the seen face into the shadow
    const far = [box(10, -20, 10.3, 20), box(4.8, -3.2, 5.2, -2.8)], upWall = ask(far, [[0, -8]], line(z => [9.94, z], 2.2, 3.6, 0.03)), lowWall = ask(far, [[0, -8]], line(z => [9.94, z], 0.3, 1.5, 0.03));
    const onWall = upWall[0] < 0.02 && upWall[upWall.length - 1] > 0.98 && mono(upWall, true) && upWall.filter(x => x > 0.05 && x < 0.95).length >= 8 && lowWall[0] > 0.98 && lowWall[lowWall.length - 1] < 0.02 && mono(lowWall, false) && lowWall.filter(x => x > 0.05 && x < 0.95).length >= 6;
    // a man coming round a corner 7 m off (wall x 7 ‥ 7.3, z < 0; eye (0, −2)): 5, 15 and 25 cm into view
    const cz = 0.6 * 2 / 7, man = ask([box(7, -10, 7.3, 0)], [[0, -2]], [0.05, 0.15, 0.25].map(z => [7.6, z + cz]));
    // five men stacked on one line past a wall's end read as one man does, either side of the line (the most any eye gives: the line does not creep into the fog)
    const end = [box(3, -10, 3.2, 0)], stackAt = [[6, 1.5 - 0.15], [6, 1.5 - 0.05], [6, 1.5 + 0.05], [6, 1.5 + 0.15]], one = ask(end, [[0, -1.5]], stackAt), five = ask(end, [[0, -1.5], [-1, -2], [-2, -2.5], [-0.5, -1.75], [-1.5, -2.25]], stackAt);
    const stack = five.every((x, i) => Math.abs(x - one[i]) < 0.02) && one[1] < 0.5 && one[2] > 0.5;
    // the sliver two men 1.2 m apart leave behind a pillar: dark on the hard line, a shallow dip with the band
    const sliver = ask(pillar, [[0, -0.6], [0, 0.6]], [[6.2, 0]])[0], sliverHard = ask(pillar, [[0, -0.6], [0, 0.6]], [[6.2, 0]], 0)[0];
    // the line a pillar casts, meeting a partition: nothing behind the partition is brightened
    let tick = 0; for (const th of [0.05, 0.2, 0.3]) { const part = [box(8, -10, 8 + th, 10), box(4, -0.2, 4.4, 0.2)], pts: number[][] = []; for (let x = 8 + th + 0.07; x < 8 + th + 0.6; x += 0.03) for (let z = -1.2; z < 0; z += 0.01) pts.push([x, z]); tick = Math.max(tick, ...ask(part, [[0, 0]], pts)); }
    // a leaf's end a quarter metre from the eye: round the eye at 0.7 ‥ 1.5 m no step but the line's own (the search there is under half a band wide)
    let pop = 0; { const leaf = [box(0.22, -3, 0.27, 0)], e = [0, -0.12], D = [table(leaf, e)]; for (const r of [0.7, 0.8, 1.0, 1.2, 1.5]) { let prev = -1, prevHard = -1; for (let a = -0.9; a < 1.5; a += 0.0005) { const q = [e[0] + r * Math.cos(a), e[1] + r * Math.sin(a)], v = seen(leaf, D, [e], q, band), h = seen(leaf, D, [e], q, 0); if (prev >= 0 && h === prevHard) pop = Math.max(pop, Math.abs(v - prev)); prev = v; prevHard = h; } } }
    const ajar = Math.max(...ask([box(3, -10, 3.2, -0.05), box(3, 0.05, 3.2, 10)], [[0, 0]], line(z => [6, z], -0.2, 0.2, 0.01)));   // a door ajar 10 cm
    // hard steps left: pairs of spots 2 cm apart, both outside every box, whose answers differ by more than 0.25 — against the hard line's own count
    const stepsIn = (boxes: PlanBox[], eyes: number[][], x0: number, x1: number, z0: number, z1: number): number[] => { const Ds = eyes.map(e => table(boxes, e)), st = 0.02, inBox = (p: number[]) => boxes.some(b => p[0] > b.x0 - 0.07 && p[0] < b.x1 + 0.07 && p[1] > b.z0 - 0.07 && p[1] < b.z1 + 0.07); let n = 0, n0 = 0;
      for (let x = x0; x <= x1; x += st) for (let z = z0; z <= z1; z += st) { const p = [x, z]; if (inBox(p)) continue; const a = seen(boxes, Ds, eyes, p, band), a0 = seen(boxes, Ds, eyes, p, 0);
        for (const q of [[x + st, z], [x, z + st]]) { if (inBox(q)) continue; if (Math.abs(seen(boxes, Ds, eyes, q, band) - a) > 0.25) n++; if (Math.abs(seen(boxes, Ds, eyes, q, 0) - a0) > 0.25) n0++; } }
      return [n, n0]; };
    const st = [stepsIn(pillar, [[0, 0]], 3, 9, -1.5, 1.5), stepsIn([box(3, -10, 3.3, 0)], [[0, -1.5]], 2, 8, -1, 3), stepsIn([box(5, -10, 5.2, -0.45), box(5, 0.45, 5.2, 10)], [[0, 1.5]], 4, 9, -3, 2), stepsIn([box(3, -10, 3.3, 0)], [[0, -1.5], [-1.5, -1.5]], 2, 8, -1, 3), stepsIn([box(20, -5, 20.3, 5.3), box(0, 5, 20, 5.3), box(0, -5.3, 20, -5)], [[2, 4.7]], 14, 21, 2, 6)];
    const left = st.reduce((n, x) => n + x[0], 0), was = st.reduce((n, x) => n + x[1], 0);
    verdict(fade && core.every(x => x < 0.02) && beside.every(x => x === 1) && wallKept && onWall && man[0] > 0.55 && man[1] > 0.8 && man[2] > 0.95 && stack && sliverHard === 0 && sliver > 0.3 && sliver < 0.5 && tick < 0.02 && pop < 0.04 && ajar > 0.6 && left * 20 < was,
      'on the plan replica of the shader\'s rule: a line cast across open floor is crossed smoothly, half on the line; the middle of a pocket nobody sees stays dark and floor beside the pillar itself stays seen; a wall seen along its length and a room\'s corner keep their line, on the seen side and behind; the line a pillar casts up a far wall fades on both of its edges; a man coming round a corner is over half seen 5 cm in and whole 25 cm in; five men stacked on a line have one man\'s line; the sliver two men leave behind a pillar is a shallow dip; nothing behind a partition is brightened where a pillar\'s line meets it; no step pops in beside a leaf\'s end a quarter metre from the eye; a door ajar 10 cm still shows its middle; and under a twentieth of the hard line\'s steps are left',
      `open floor: ${f2(across[0])} → ${f2(across[across.length - 1])}, falling ${mono(across, false)}, ${f2(atLine[0])} | ${f2(atLine[1])} either side of the line, ${across.filter(x => x > 0.05 && x < 0.95).length} steps between · pocket's middle at most ${f2(Math.max(...core))} · beside the pillar at least ${f2(Math.min(...beside))} · walls kept: ${wallKept} · up a far wall: ${f2(upWall[0])} → ${f2(upWall[upWall.length - 1])} rising ${mono(upWall, true)} and ${f2(lowWall[0])} → ${f2(lowWall[lowWall.length - 1])} falling ${mono(lowWall, false)} · a man 5 / 15 / 25 cm round a corner: ${man.map(f2).join(' / ')} · one man / five stacked, 15 and 5 cm either side: ${one.map(f2).join(' ')} / ${five.map(f2).join(' ')} · the sliver: ${f2(sliver)} (hard ${sliverHard}) · behind a partition, most: ${f2(tick)} · the largest step beside a leaf's end: ${pop.toFixed(3)} · ajar 10 cm, most: ${f2(ajar)} · steps left ${left} of the hard line's ${was} (${st.map(x => x.join('/')).join(' · ')})`);
  }
  { // the engine replay asks for the page's own fog look
    const epl: string = fsrc.readFileSync(`${ROOT}/src/replay/engineplay.ts`, 'utf8'), visitWords = /VISIT_FLAGS = \[([^\]]*)\]/.exec(fsrc.readFileSync(`${ROOT}/src/ui/replaykey.ts`, 'utf8'))?.[1] ?? '';
    // (the replays goal's files, read loosely: its calls may move and its list may grow — what must hold is that it asks as it attaches, after its tables and after a FOG table row, and that the visit keeps both words)
    const asked = (epl.match(/g\.fogOfWar\.pageLook\(\)/g) ?? []).length >= 3 && /this\.tables\(\);\s*g\.fogOfWar\.pageLook\(\)/.test(epl) && /name === 'FOG'\)\s*g\.fogOfWar\.pageLook\(\)/.test(epl) && /'fogedge'/.test(visitWords) && /'fogdesat'/.test(visitWords);
    verdict(asked, 'as written: the engine replay asks for the page\'s own fog look as it takes the frame, after its first tables and after every FOG table of the tape\'s, and the visit\'s own ?fogedge / ?fogdesat are kept',
      `the replay asks (attaching, after its tables, after a FOG table row) and keeps the visit's words: ${asked}`);
  }
  Object.assign(FOG, FOG_DEFAULTS);
  return done();
}

// THE AIR IS THE PLACE (game/fogofwar.ts FOG.air — the maintainer, 10-23, on a fire with no flame in the fog: 'I think we should at least show everything in the
// environment visible at the start of the level all the time?'): smoke and dust are drawn over floor the team does not see too. The march is shader text
// (common.wgsl fogSmoke, read by volumetrics.wgsl, its dust look too), so headless this asks (1) what reaches the renderer — the block's row every frame, and the
// sight table's head as render/fogofwar.ts writes it, (2) the wiring as written — fogSmoke leaves a density alone while the head says so and cuts it by
// fogSeenAt otherwise, every march of the smoke takes its density through it, and nothing else in the shaders reads the row, (3) WHAT A HIDDEN MAN FIRES IS
// NOT FILED: the fog's airHides says so of a man out of the picture and of nobody else, a shot told to leave its gas out files none and gives the same flash,
// and combat.ts asks it of every shooter, (4) that the fight is the same either way: the night breach ordered in and fought with it on digests the same as
// with it off, dice and all, then the address, the journal (and the engine replay's page look) and the stands. What a plume looks like under the fog is a
// real page's to show (the software adapter) and a GPU's to judge.
//   FOGAIR_BREAK=1 → the fog is put off before the block is read: the block's line FAILS.
// The row and the full picture are settled on and no longer written. Until 2026-11-07 the probe also asked the row off ('everybody files gas, as it was'),
// the block without the full picture, ?fogair on the address, a recording from before the key, the two stands, and held the fight to the same digest and
// dice with the smoke drawn as without.
export async function fogAir(): Promise<boolean> {
  const { verdict, done } = family('fog-air: smoke and dust are drawn under the fog of war too');
  const brk = process.env.FOGAIR_BREAK === '1';
  const R = await rig(57, { fog: true }); const { game, FW } = R;
  if (brk) FW.on = false;
  const fsrc = await import('node:fs'); const rd = (f: string): string => fsrc.readFileSync(`${ROOT}/src/render/${f}`, 'utf8');
  const strip = (t: string) => t.split('\n').map(l => l.replace(/\/\/.*$/, '')).join('\n');
  const F = FW.frame;
  { // the block
    R.step(20); const on1 = F.air, fogOn = F.on && F.count > 0 && !!F.full;
    const up = rd('fogofwar.ts'), head = /U\[o \+ 7\] = \(F!\.air \? 1 : 0\) \+ \(thin \? 2 : 0\) \+ \(F!\.kept \? 4 : 0\);/.test(up) && /air: false/.test(/export function makeFogFrame\(\)[^\n]*/.exec(up)?.[0] ?? '');
    verdict(fogOn && on1 === true && head, 'the fog\'s block carries the row every frame, and the sight table\'s head takes it as its last float (a frame with no fog block says off)',
      `fog on with the full picture: ${fogOn} · the row in the block: ${on1} · the head's write and the empty frame's row as written: ${head}`);
  }
  { // the shaders
    const common = strip(rd('shaders/common.wgsl')), a = common.indexOf('fn fogSmoke('), body = a < 0 ? '' : common.slice(a, common.indexOf('\n', a));
    const rule = /fn fogSmoke\(dens: f32, X: vec3f\) -> f32 \{ if \(dens <= 0\.002 \|\| fogSmokeOn\(\)\) \{ return dens; \} return dens \* fogSeenAt\(X, vec3f\(0\.0\)\); \}/.test(body)
      && /fn fogSmokeOn\(\) -> bool \{ return \(u32\(fogSight\.soft\.w \+ 0\.5\) & 1u\) != 0u; \}/.test(common);   // (the head's last float is a whole number of two bits since FOG.thin rides it too: bit 0 is the air's)
    const vol = strip(rd('shaders/volumetrics.wgsl')), marches = (vol.match(/let dens = [^;]*;/g) ?? []), through = marches.filter(m => /fogSmoke\(/.test(m)).length;
    const twinThrough = vol.split('let dl = dustDensity(X, onMask, true); let dens = fogSmoke(dl.d, X);').length - 1;   // (the march reads the dust's drawn density, and that goes through fogSmoke too)
    const dir = `${ROOT}/src/render/shaders`, others = (fsrc.readdirSync(dir) as string[]).filter(f => f.endsWith('.wgsl') && f !== 'common.wgsl' && /fogSight\.soft\.w/.test(strip(fsrc.readFileSync(`${dir}/${f}`, 'utf8'))));
    verdict(rule && marches.length === 1 && through === 1 && twinThrough === 1 && others.length === 0, 'as written: fogSmoke hands a density back whole while the sight table\'s head says the air is drawn, and cut by what the team sees otherwise; the smoke\'s march (one on every tier) takes its density through it, the dust look\'s drawn density; no other shader reads the row',
      `fogSmoke as expected: ${rule} · marches in volumetrics.wgsl: ${marches.length}, through fogSmoke: ${through} · the dust look's density through it: ${twinThrough} · other shaders reading the row: ${others.join(', ') || 'none'}`);
  }
  { // what a hidden man fires is not filed
    R.step(2);
    const gs: any[] = game.guards.filter((x: any) => x.state !== 'dead'), hid = gs.find((x: any) => FW.hidden(x)), mate = gs.find((x: any) => FW.onTeam(x));
    const onHid = !!hid && FW.airHides(hid.char), onMate = !!mate && FW.airHides(mate.char), onSam = FW.airHides(game.player.char);
    const fx = await import(`${ROOT}/src/game/effects.ts`); let filed = 0; const sink: any = { emit() { filed++; }, push() {}, inSmokeDomain: () => false };
    const shot = (gas: boolean | undefined) => { filed = 0; const L = fx.spawnShot(sink, null, [0, 1.4, 0], [0, 0, 1], { seed: 7, gas }); return { filed, lights: JSON.stringify(L) }; };
    const plain = shot(undefined), kept = shot(true), left = shot(false);
    const asks = /gas: !g\.fogOfWar\.airHides\(shooter\)/.test(fsrc.readFileSync(`${ROOT}/src/game/combat.ts`, 'utf8'));
    verdict(!!hid && !!mate && onHid && !onMate && !onSam && plain.filed > 0 && kept.filed === plain.filed && left.filed === 0 && left.lights === plain.lights && asks,
      'a man out of the picture files no gun gas while smoke is drawn under the fog — a teammate and the player always do; a shot told to leave its gas out files nothing and gives the same flash; combat.ts asks of every shooter',
      `a hidden defender: ${hid ? hid.callsign : 'NONE'} → ${onHid} · a teammate: ${mate ? mate.callsign : 'NONE'} → ${onMate} · the player → ${onSam} · emitters a shot files: ${plain.filed}, said to keep its gas ${kept.filed}, to leave it out ${left.filed} · the same flash: ${left.lights === plain.lights} · combat.ts asks: ${asks}`);
  }
  Object.assign(FOG, FOG_DEFAULTS);
  return done();
}

// THIN THINGS THAT STAND FREE DO NOT CUT THE FOG'S SIGHT (game/fogofwar.ts FOG.thin; scene/thin.ts — the maintainer, 10-23, of a taken room on The Pass:
// 'weird vertical striations due to fog of war + squad's vision'). Headless this asks (1) THE MARKS, on every level file: what scene/thin.ts marks, that no
// marked box stands within 30 cm of a door's shut leaf (a jamb or a stop passed would open every shut door), and on The Pass what the marks are; (2) THE
// SQUAD'S OWN LINES on The Pass stood up, through the game's real collision: from three spots outside the long room's windows and one inside it, a level line
// a bearing — how many slivers of 'not seen' the row takes away, that it never shortens a line, and that the fog's own test (FogOfWar.sightBlocked) passes the
// stovepipe with the row and not without, a wall never; the callers as written; (3) THE BLOCK: the row and the marked statics' bits reach the
// renderer's frame, renewed only when they change, and the sight table's head and size as written; (4) THE SHADERS AS WRITTEN: the one test, asked of a box
// already hit by the two walks an eye's ray takes and of the named box, raised only round the fog's own rays and put back; (5) the address; (6) the journal,
// and that the replay page leaves the row to the tape; (7) the stands; (8) a fought breach with and without it, two seeds. FOGTHIN_BREAK=1 = the marks never
// made: lines 1, 2 and 3 are red. NOT ASKED: the picture — no headless run executes a shader. NEVER SEEN ON A GPU.
// The row is settled on and no longer written: (2) asks the fog's own test as the page has it, (5) and (7) are gone, (6) is the page's rows as written
// and (8) one fought breach a seed. Until 2026-11-07 the probe also asked the fog's test and the block 'without the row', ?fogthin on the address, a
// recording from before the key, the two stands, and held the fight to the same digest and dice without the row as with it.
export async function fogThin(): Promise<boolean> {
  const { verdict, done } = family('fog-thin: thin things that stand free do not cut the fog of war\'s sight');
  const brk = process.env.FOGTHIN_BREAK === '1';
  const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`); const { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`);
  const { BoxFlag, makeBox } = await import(`${ROOT}/src/scene/boxes.ts`); const { thinLevel, THIN_WIDTH } = await import(`${ROOT}/src/scene/thin.ts`); const { StaticCollision } = await import(`${ROOT}/src/scene/collision.ts`); const { levelToData } = await import(`${ROOT}/src/scene/levelLoad.ts`); const { roundLevel } = await import(`${ROOT}/src/scene/round.ts`);
  const { makeFogFrame, FOG_THIN_WORDS, FOG_SIGHT_BYTES, FOG_BINS, FOG_EYES } = await import(`${ROOT}/src/render/fogofwar.ts`); const { WORLD } = await import(`${ROOT}/src/scene/world.ts`);
  const fsrc = await import('node:fs'); const rd = (f: string): string => fsrc.readFileSync(`${ROOT}/src/render/${f}`, 'utf8');
  const strip = (t: string) => t.split('\n').map(l => l.replace(/\/\/.*$/, '')).join('\n');
  const quiet = async <T>(fn: () => T | Promise<T>): Promise<T> => { const l = console.log, e = console.error, w = console.warn; console.log = console.error = console.warn = () => { }; try { return await fn(); } finally { console.log = l; console.error = e; console.warn = w; } };
  const levelOf = (file: string) => quiet(() => loadLevel(parseLevelData(fsrc.readFileSync(`${ROOT}/levels/${file}`, 'utf8'))));
  { // the marks, on every level
    const every = (fsrc.readdirSync(`${ROOT}/levels`) as string[]).filter(f => f.endsWith('.json')).sort(), files = [...new Set(['pass.json', ...sample(every, 6)])];   // (The Pass is what the line's last words ask of)
    let levels = 0, marked = 0, withMarks = 0, atDoors = 0, wide = 0, beside = 0, doors = 0, doorLines = 0, doorLeaks = 0, bySize = 0, exported = 0; const bad: string[] = []; let pass = '';
    for (const f of files) { let L: any; try { L = await levelOf(f); } catch { continue; }
      if (!L?.boxes?.length) continue; levels++;
      for (const on of [true, false]) {   // (a level's round things as one box each, and as the kit's slabs: the marks are made after either)
        roundLevel(L.boxes, on, L.markTexel); const n = brk ? 0 : thinLevel(L.boxes, L.doors ?? []); if (brk) for (const b of L.boxes) b.flags &= ~BoxFlag.Thin;
        const th = L.boxes.filter((b: any) => b.flags & BoxFlag.Thin);
        if (on) {   // the level's own collision with every door's leaf stood in it SHUT (a plain box, hinge to latch), and short lines ACROSS each door's plane inside its frame's span (30 cm past either end), from either side, at three heights: none may be stopped without the marks and pass with them — that is a shut door leaking. The same lines with every narrow solid box marked BY SIZE ALONE (the first version's rule) do leak: the count's teeth.
          const leaves = (L.doors ?? []).map((d: any) => makeBox({ c: [d.hinge[0] + Math.cos(d.closedDir) * d.width / 2, 1.02, d.hinge[1] + Math.sin(d.closedDir) * d.width / 2], h: [d.width / 2, 1.02, 0.022], yaw: -d.closedDir }));
          const C = new StaticCollision([...L.boxes, ...leaves]);
          const across = (): number => { let n = 0;
            for (const d of (L.doors ?? [])) { const [hx, hz] = d.hinge, ux = Math.cos(d.closedDir), uz = Math.sin(d.closedDir), W = d.width;
              for (const side of [-1, 1]) for (const te of [-0.3, -0.1, 0.1, W / 2, W - 0.1, W + 0.1, W + 0.3]) for (const out of [0.25, 0.6]) { const ex = hx + ux * te - uz * out * side, ez = hz + uz * te + ux * out * side;
                for (const y of [0.3, 1.0, 1.9]) for (const deep of [0.2, 0.5]) for (let i = 0; i <= 12; i++) { const tt = -0.3 + (W + 0.6) * i / 12, far: number[] = [hx + ux * tt + uz * deep * side, y, hz + uz * tt - ux * deep * side]; doorLines++;
                  if (C.segmentBlocked([ex, y, ez], far, 0.02, BoxFlag.Glass) && !C.segmentBlocked([ex, y, ez], far, 0.02, BoxFlag.Glass | BoxFlag.Thin)) { n++; if (bad.length < 4 && !sizing) bad.push(`${f}: a line across door ${d.name}'s frame passes with the marks`); } } } }
            return n; };
          doors += (L.doors ?? []).length; let sizing = false; doorLeaks += across();
          const keep = L.boxes.map((b: any) => b.flags); sizing = true;
          for (const b of L.boxes) if (!(b.flags & (BoxFlag.Glass | BoxFlag.NoShadow)) && !b.rot && !(b.cut && b.cut.length) && 2 * Math.max(b.h[0], b.h[2]) < THIN_WIDTH) b.flags |= BoxFlag.Thin;
          bySize += across(); L.boxes.forEach((b: any, k: number) => { b.flags = keep[k]; });
          try { await quiet(() => levelToData(L, {})); exported++; } catch (e: any) { if (bad.length < 4) bad.push(`${f}: the marked level does not export (${String(e?.message ?? e).slice(0, 60)})`); }   // (the mark is derived: the level's file never carries it)
        }
        if (on) { marked += th.length; if (th.length) withMarks++; if (f === 'pass.json') pass = `${n}: ` + th.map((b: any) => `${(2 * b.h[0]).toFixed(2)} × ${(2 * b.h[2]).toFixed(2)} at (${b.c[0].toFixed(1)}, ${b.c[2].toFixed(1)})`).join(', '); }
        for (const b of th) {
          if (2 * Math.max(b.h[0], b.h[2]) >= THIN_WIDTH || b.rot || (b.flags & (BoxFlag.Glass | BoxFlag.NoShadow))) wide++;
          // nothing solid beside it, asked again here in the plainest way: no other solid box's plan AABB (as boxes.ts takes it) within 3 cm of its own over a quarter of its height — bar a box as narrow that shares its middle (a round thing's slabs)
          const a0 = b.c[1] - b.h[1], a1 = b.c[1] + b.h[1];
          for (const o of L.boxes) { if (o === b || (o.flags & (BoxFlag.Glass | BoxFlag.NoShadow)) || o.rot || Math.abs(o.yaw) > 1e-6 || Math.abs(b.yaw) > 1e-6) continue;
            if (Math.min(a1, o.c[1] + o.h[1]) - Math.max(a0, o.c[1] - o.h[1]) <= Math.max(0.02, 0.25 * (a1 - a0))) continue;
            if (Math.abs(o.c[0] - b.c[0]) > o.h[0] + b.h[0] + 0.03 || Math.abs(o.c[2] - b.c[2]) > o.h[2] + b.h[2] + 0.03) continue;
            if (2 * Math.max(o.h[0], o.h[2]) < THIN_WIDTH && Math.abs(o.c[0] - b.c[0]) <= b.h[0] && Math.abs(o.c[2] - b.c[2]) <= b.h[2]) continue;
            beside++; if (bad.length < 4) bad.push(`${f}: a marked box at (${b.c[0].toFixed(2)}, ${b.c[2].toFixed(2)}) has one beside it`); }
          for (const d of (L.doors ?? [])) {   // within 30 cm of the shut leaf's line, hinge to latch
            const ux = Math.cos(d.closedDir), uz = Math.sin(d.closedDir), px = b.c[0] - d.hinge[0], pz = b.c[2] - d.hinge[1], t = Math.max(-0.1, Math.min(d.width + 0.1, px * ux + pz * uz));
            if (Math.hypot(px - ux * t, pz - uz * t) < 0.3 && a0 <= 2.2) { atDoors++; if (bad.length < 4) bad.push(`${f}: a marked box at door ${d.name}`); } }
        }
      }
      roundLevel(L.boxes, false, L.markTexel);
    }
    verdict(levels >= swept(5, 20) && marked >= 6 && withMarks >= 3 && atDoors === 0 && wide === 0 && beside === 0 && doors >= swept(10, 100) && doorLines > swept(10000, 100000) && doorLeaks === 0 && bySize > swept(5, 100) && exported === levels && /0\.10 × 0\.10/.test(pass) && /0\.08 × 0\.19|0\.19 × 0\.08/.test(pass),
      'every level file, its round things as one box each and as slabs: what is marked thin and free is narrow, unrotated and solid, has nothing solid beside it (asked again here, plainly), and none stands at a door\'s shut leaf; through each level\'s own collision, its doors shut, no line across a door\'s frame passes with the marks that was stopped without (marked by size alone, the first version\'s way, they do); a marked level still exports — and The Pass\'s marks are its stovepipes and its windows\' bars',
      `levels read: ${levels} · marked in all: ${marked}, on ${withMarks} levels · marked at a door: ${atDoors} · lines across ${doors} shut doors' frames: ${doorLines / 2} each way of marking, let through by the marks: ${doorLeaks}, by size alone: ${bySize} · levels that export marked: ${exported} · not narrow / rotated / not solid: ${wide} · with something beside: ${beside}${bad.length ? ' — ' + bad.join('; ') : ''} · The Pass: ${pass || 'none'}`);
  }
  // The Pass stood up: the game's own collision world and the fog's own test
  const P: any = await quiet(async () => standUp({ level: await levelOf('pass.json'), seed: 7 })); const pg = P.game, PF = pg.fogOfWar, col = pg.col, statics: any[] = pg.engine.world.statics;
  if (brk) for (const b of statics) b.flags &= ~BoxFlag.Thin;
  { // the squad's own lines
    const EYE = 1.62, BINS = 1024, FAR = 45;
    /** how far a level line from (ex, ez) along bearing k goes before the game's sight test stops it (bisection on the segment's length: blocked is monotone in it) */
    const reach = (ex: number, ez: number, k: number, through: number): number => { const a = (k + 0.5) / BINS * Math.PI * 2 - Math.PI, dx = Math.cos(a), dz = Math.sin(a);
      if (!col.segmentBlocked([ex, EYE, ez], [ex + dx * FAR, EYE, ez + dz * FAR], 0.02, through)) return FAR;
      let lo = 0, hi = FAR; for (let i = 0; i < 12; i++) { const m = (lo + hi) / 2; if (col.segmentBlocked([ex, EYE, ez], [ex + dx * m, EYE, ez + dz * m], 0.02, through)) hi = m; else lo = m; } return lo; };
    const eyes: [number, number][] = [[14.9, 12.5], [19.3, 12.5], [27.6, 12.5], [17.5, 8.5]];
    const rows: string[] = []; let slivers = 0, shorter = 0, each = true;
    for (const [ex, ez] of eyes) { let n = 0, was = false, gained = 0;
      for (let k = 0; k < BINS; k++) { const a = reach(ex, ez, k, BoxFlag.Glass), b = reach(ex, ez, k, BoxFlag.Glass | BoxFlag.Thin); if (b < a - 0.02) shorter++; const g = b > a + 0.3; if (g) gained++; if (g && !was) n++; was = g; }
      slivers += n; each &&= n >= 1; rows.push(`eye (${ex}, ${ez}): ${n} slivers over ${gained} of ${BINS} bearings`); }
    // the fog's own test, across a stovepipe, a wall and a door's jamb
    const pipe = statics.find(b => Math.abs(b.c[0] - 15.7) < 0.01 && Math.abs(b.c[2] - 7.2) < 0.01 && Math.abs(b.h[0] - 0.05) < 0.01);
    const sb = (a: number[], b: number[]) => PF.sightBlocked(a, b) as boolean;
    const pa = [15.7 - 1.0, EYE, 7.2], pb = [15.7 + 1.0, EYE, 7.2];
    const pipeOn = !!pipe && sb(pa, pb), plainOff = !!pipe && col.segmentBlocked(pa, pb);
    const wallOn = sb([19.3, EYE, 12.5], [19.3, 0.2, 10.5]);   // (down through the wall under the window)
    // every door of the level, as it stands shut: from a step out on either side, by its hinge and by its latch, a fan of lines at the frame — none goes further with the row than without
    let doorLines = 0, doorLeaks = 0;
    for (const d of pg.doors.list as any[]) { const [hx, hz] = d.def.hinge, ux = Math.cos(d.def.closedDir), uz = Math.sin(d.def.closedDir), W = d.def.width;
      for (const f of [0.02, 0.98]) for (const side of [-1, 1]) { const ex = hx + ux * W * f - uz * 0.6 * side, ez = hz + uz * W * f + ux * 0.6 * side;
        for (let i = -12; i <= 12; i++) { const a = Math.atan2(-ux * side, uz * side) + i * 0.06, far: number[] = [ex + Math.cos(a) * 6, 1.0, ez + Math.sin(a) * 6]; doorLines++;
          if (col.segmentBlocked([ex, 1.0, ez], far, 0.02, BoxFlag.Glass) && !col.segmentBlocked([ex, 1.0, ez], far, 0.02, BoxFlag.Glass | BoxFlag.Thin)) doorLeaks++; } } }
    const src = fsrc.readFileSync(`${ROOT}/src/game/fogofwar.ts`, 'utf8'), calls = (src.match(/this\.sightBlocked\(/g) ?? []).length, raw = (src.match(/col\.segmentBlocked\(/g) ?? []).length;
    const helper = /private sightBlocked\(a: Vec3, b: Vec3\): boolean \{ return this\.g\.col\.segmentBlocked\(a, b, 0\.02, fogThrough\(\)\); \}/.test(src) && /export function fogThrough\(\): number \{ return FOG\.thin \? BoxFlag\.Glass \| BoxFlag\.Thin : BoxFlag\.Glass; \}/.test(src);
    verdict(each && slivers >= 8 && shorter === 0 && !!pipe && !pipeOn && plainOff && wallOn && doorLines > 300 && doorLeaks === 0 && helper && calls === 8 && raw === 2,
      'The Pass stood up, the game\'s own sight test: from outside each of the long room\'s three windows and from inside it the marks take slivers of \'not seen\' away and shorten no line; the fog\'s own test passes the stovepipe (which stops the plain line) and not a wall; at every door of the level, shut, no line at its frame from a step out goes further with the row; every eye\'s line in fogofwar.ts goes through that one test (the lamp\'s reach to its own floor, which is no eye\'s, does not)',
      `${rows.join(' · ')} · lines made shorter: ${shorter} · across the stovepipe: the plain test blocked ${plainOff}, the fog's ${pipeOn} · through the wall: ${wallOn} · lines at ${pg.doors.list.length} shut doors' frames: ${doorLines}, further with the row: ${doorLeaks} · the helper as written: ${helper} · its callers: ${calls - 1} · segmentBlocked left bare: ${raw - 1}`);
  }
  { // the block
    const F = makeFogFrame(); const want = new Uint32Array(FOG_THIN_WORDS); let n = 0; statics.forEach((b, i) => { if (b.flags & BoxFlag.Thin) { want[i >> 5] |= 1 << (i & 31); n++; } });
    const same = (a: Uint32Array, b: Uint32Array) => a.every((v, i) => v === b[i]);
    PF.thinBlock(F); const on1 = F.thin, bits1 = same(F.thinBits, want), stamped = F.thinCount === statics.length && F.thinSerial === (pg.engine.world.geomSerial ?? 0);
    const guard = /const W = this\.world, thin = F!\.thin && \(!W \|\| \(W\.geomSerial === F!\.thinSerial && W\.statics\.length === F!\.thinCount\)\);/.test(rd('fogofwar.ts')) && /this\.fogSight\.world = this\.world;/.test(fsrc.readFileSync(`${ROOT}/src/engine.ts`, 'utf8'));
    const one = statics.find(b => b.flags & BoxFlag.Thin), oi = statics.indexOf(one); if (one) one.flags &= ~BoxFlag.Thin; PF.thinBlock(F); const gone = oi >= 0 && ((F.thinBits[oi >> 5] >>> (oi & 31)) & 1) === 0 && !same(F.thinBits, want); if (one) one.flags |= BoxFlag.Thin; PF.thinBlock(F); const bits4 = same(F.thinBits, want);
    // a bare stud in an opened wall is marked as it is made (wallbreak.ts), and the renderer's upload goes by what the bits ARE (a new encounter's frame starts from nothing: no count to fall behind)
    const studMark = /p\.name = 'stud'; p\.flags \|= BoxFlag\.Thin;/.test(fsrc.readFileSync(`${ROOT}/src/game/wallbreak.ts`, 'utf8'));
    const up = rd('fogofwar.ts'), common = strip(rd('shaders/common.wgsl'));
    const head = /U\[o \+ 7\] = \(F!\.air \? 1 : 0\) \+ \(thin \? 2 : 0\) \+ \(F!\.kept \? 4 : 0\);/.test(up) && /this\.uwords\.set\(F!\.thinBits, SIGHT_FLOATS_BYTES \/ 4\);/.test(up) && /if \(this\.stand && thin\) \{ const T = F!\.thinBits, was = this\.standBits;/.test(up) && /if \(!same\) \{ this\.device\.queue\.writeBuffer\(this\.stand, SIGHT_HEAD \+ SIGHT_FLOATS_BYTES, T\); \(this\.standBits \?\?= new Uint32Array\(T\.length\)\)\.set\(T\); \}/.test(up)
      && /thin: false, thinBits: new Uint32Array\(FOG_THIN_WORDS\), thinSerial: -1, thinCount: -1/.test(/export function makeFogFrame\(\)[^\n]*/.exec(up)?.[0] ?? '') && /this\.fogSight\.stand = this\.fogSightStand;/.test(fsrc.readFileSync(`${ROOT}/src/engine.ts`, 'utf8'));
    const words = Number((common.match(/thin: array<vec4u, (\d+)>/) ?? [])[1]) * 4, reach = Number((common.match(/return bi < (\d+)u && \(\(fogSight\.thin\[/) ?? [])[1]), size = FOG_SIGHT_BYTES === FOG_BINS * FOG_EYES * 4 + FOG_EYES * 32 + 32 + FOG_THIN_WORDS * 4;
    verdict(n >= 6 && on1 === true && bits1 && stamped && guard && gone && bits4 && head && studMark && words === FOG_THIN_WORDS && reach === FOG_THIN_WORDS * 32 && WORLD.maxBoxes <= FOG_THIN_WORDS * 32 && size,
      'the fog\'s block carries the row and, a bit a box, exactly the statics marked this frame (a mark taken off is gone from them the next fill; a breached wall\'s stud is marked as it is made) — and the sight table takes both: the head\'s word, the bits behind it in the table and in the stand-in its own pass reads, uploaded when the bits differ, and only while the box list is the one they were read off (its serial and count ride the block: a frame drawn after a wall is rebuilt and before the next fill passes nothing); the struct\'s list as long as the renderer\'s, the shader\'s reach its bit count, and that no less than the box world holds',
      `marked statics on The Pass: ${n} of ${statics.length} · the row: ${on1} · the bits are the marks: ${bits1} · a mark off: its bit gone ${gone}, back on: the marks again ${bits4} · wallbreak.ts marks a stud: ${studMark} · the shader's reach: ${reach} boxes, the box world's ${WORLD.maxBoxes} · the block says which list: ${stamped} · the upload asks: ${guard} · the writes as written: ${head} · the shader's list: ${words} words, the renderer's ${FOG_THIN_WORDS} · the table's size: ${size}`);
  }
  { // the shaders
    const common = strip(rd('shaders/common.wgsl')), fow = strip(rd('shaders/fogofwar.wgsl')), sight = strip(rd('shaders/fogsight.wgsl'));
    const test = /fn fogThinBox\(b: BoxGeo, bi: u32\) -> bool \{\s*if \(!fogEyeThin\) \{ return false; \}\s*if \(boxOwner\(b\) != 0u\) \{ return true; \}\s*return bi < 4096u && \(\(fogSight\.thin\[bi >> 7u\]\[\(bi >> 5u\) & 3u\] >> \(bi & 31u\)\) & 1u\) != 0u;\s*\}/.test(common);
    const on = /fn fogThinOn\(\) -> bool \{ return fogSight\.cfg\.y == 1\.0 && \(u32\(fogSight\.soft\.w \+ 0\.5\) & 2u\) != 0u; \}/.test(common);
    const a = common.indexOf('fn occludedTBoxes('), walk = a < 0 ? '' : common.slice(a, common.indexOf('\n}', a));
    const inWalk = (walk.match(/if \(r\.x >= 0\.0 && \(bestInCell < 0\.0 \|\| r\.x < bestInCell\) && !fogThinBox\(b, bi\)\) \{ bestInCell = r\.x; \}/g) ?? []).length;
    const twin = (fow.match(/if \(r\.x >= 0\.0 && \(bestInCell < 0\.0 \|\| r\.x < bestInCell\) && !fogThinBox\(b, bi\)\) \{ bestInCell = r\.x; bestBox = f32\(bi\) \+ 1\.0; \}/g) ?? []).length, hint = /skipsOwner\(boxOwner\(b\), owner\) \|\| fogThinBox\(b, bi\)\) \{ return false; \}/.test(fow);
    const s0 = common.indexOf('fn fogSeenAt('), seenAt = s0 < 0 ? '' : common.slice(s0, common.indexOf('\n}', s0));
    const inSeen = /let thinWas = fogEyeThin; fogEyeThin = fogThinOn\(\);/.test(seenAt) && /fogEyeThin = thinWas;\s*fogSeenVal = best;\s*return best;/.test(seenAt) && (seenAt.match(/return /g) ?? []).length === 3;   // (its two early returns come before the raise)
    const main = /fogEyeThin = fogThinOn\(\);\s*let seen = fowSeen\(P, N\);\s*fogEyeThin = false;/.test(fow), pre = /fogEyeThin = fogThinOn\(\);\s*let r = fowWalk\(/.test(fow);
    const kern = /fogEyeThin = \(u32\(sight\.soft\.w \+ 0\.5\) & 2u\) != 0u;\s*let e = sight\.eyes\[k\];/.test(sight);
    const writes = [common, fow, sight].map(t => (t.match(/fogEyeThin = /g) ?? []).length), dir = `${ROOT}/src/render/shaders`;
    const elsewhere = (fsrc.readdirSync(dir) as string[]).filter(f => f.endsWith('.wgsl') && !['common.wgsl', 'fogofwar.wgsl', 'fogsight.wgsl'].includes(f) && /fogEyeThin|fogThinBox|fogThinOn/.test(strip(fsrc.readFileSync(`${dir}/${f}`, 'utf8'))));
    verdict(test && on && inWalk === 1 && twin === 1 && hint && inSeen && main && pre && kern && writes.join() === '2,3,1' && elsewhere.length === 0,
      'as written: one test (a box somebody carries, or one whose bit the table carries), asked of a box already hit by the two walks an eye\'s ray takes and of the fog pass\'s named box; raised only round the fog\'s own rays (fogSeenAt\'s walk, the fog pass\'s rays and its named-box table, the sight table\'s kernel) and put back where another ray could follow; the row is read only over a table written this frame; no other shader names it',
      `the test: ${test} · the row's read: ${on} · occludedTBoxes asks it of a hit: ${inWalk} · the fog pass's twin: ${twin} · its named box: ${hint} · fogSeenAt raises and puts back: ${inSeen} · the fog pass's rays: ${main} · its table: ${pre} · the kernel: ${kern} · writes of the private in common / fogofwar / fogsight: ${writes.join(' / ')} · other shaders naming it: ${elsewhere.join(', ') || 'none'}`);
  }
  { // the replay page leaves the row to the tape
    const pageRows = /const FOG_PAGE = \{ desat: FOG\.desat, edge: FOG\.edge, air: FOG\.air, kept: FOG\.kept \};/.test(fsrc.readFileSync(`${ROOT}/src/game/fogofwar.ts`, 'utf8'));
    const visit = /VISIT_FLAGS = \[([^\]]*)\]/.exec(fsrc.readFileSync(`${ROOT}/src/ui/replaykey.ts`, 'utf8'))?.[1] ?? '', kept = /'fogthin'/.test(visit);
    verdict(pageRows && !kept, 'as written: the replay page leaves the row to the tape (no row of its fog look, no visit word)', `the page's look as written: ${pageRows} · the visit keeps ?fogthin: ${kept}`);
  }
  { // a fight on a level with marked boxes
    const fight = async (seed: number) => { const Q = await rig(seed, { fog: true, chaos: 0.5 }); const marks = (Q.game.engine.world.statics as any[]).filter(b => b.flags & BoxFlag.Thin).length;
      Q.step(40); Q.NB.order(); Q.step(60 * 7); Q.NB.order(); Q.step(60 * 14);
      return { state: Q.NB.state as string, thin: Q.FW.frame.thin, marks }; };
    const rows: string[] = []; let fought = true, told = true, marks = 0;
    for (const seed of sample([23, 24], 1)) { const b = await fight(seed); marks = b.marks; fought &&= b.state !== 'staged'; told &&= b.thin === true; rows.push(`seed ${seed}: ${b.state}`); }
    verdict(fought && told && marks > 0, 'the night breach under the fog\'s full picture, the team ordered in, 21 s, on a level with marked boxes: it is fought and the frame carries the row',
      `${rows.join(' · ')} · marked statics on this level: ${marks}`);
  }
  Object.assign(FOG, FOG_DEFAULTS);
  return done();
}

// MORE THAN FIVE EYES LIFT THE FOG (game/fogofwar.ts FOG.eyes — the fourth of the fog's principles: 'to see more, get more eyes on it'; with a squad of four
// the fog's five places were full, so a sticky camera that sent was drawn and saw nothing). Headless this asks (1) THE ROOM: the renderer's block, its sight
// table, the three shaders and the hint pass's counting twin carry seven, the sight pass walks the live rows, and the table still fits a uniform block;
// (2) THE CASE IT WAS BUILT FOR: the room clear with a SQUAD OF FOUR and three cameras sending, the newest by a man nobody sees — blind at five, the sixth
// eye at six, two eyes at seven, never over the room whatever is asked; (3) the same four with no camera at five and at seven: the same game; (4) the
// address; (5) the journal, and that the replay page leaves the row to the tape; (6) the stands; (7) a fought breach at 5 and at 7 with nobody past the
// fifth place: the same run. NEVER SEEN ON A GPU.
// The row is settled at seven and no longer written (nor the cameras' switch). Until 2026-11-07 (2) also asked the count at five ('the build as it was'),
// at six, and at 99, no end, no number, 0 and 6.9; (3), (4), (6) and the digest half of (7) asked five against seven, the address and the two stands; (5)
// asked a recording from before the key. What is left of (2) is the page's seven and its one live feed, and of (7) that nobody is past the fifth place.
export async function fogEyes(): Promise<boolean> {
  const { verdict, done } = family('fog-eyes: more than five eyes lift the fog of war');
  const { FOG_EYES_ON, FOG_EYES_WAS } = await import(`${ROOT}/src/game/fogofwar.ts`);
  const { makeFogFrame, FOG_SIGHT_BYTES, FOG_BINS, FOG_EYES } = await import(`${ROOT}/src/render/fogofwar.ts`);
  const fsrc = await import('node:fs'); const rd = (f: string): string => fsrc.readFileSync(`${ROOT}/src/render/${f}`, 'utf8').split('\n').map(l => l.replace(/\/\/.*$/, '')).join('\n');
  { // the room
    const fow = rd('shaders/fogofwar.wgsl'), sight = rd('shaders/fogsight.wgsl'), common = rd('shaders/common.wgsl');
    const arr = (t: string) => [Number((t.match(/eyes: array<vec4f, (\d+)>/) ?? [])[1]), Number((t.match(/looks: array<vec4f, (\d+)>/) ?? [])[1])];
    const a = arr(fow), b = arr(sight), c = arr(common), dist = Number((common.match(/dist: array<vec4f, (\d+)>/) ?? [])[1]), guard = Number((sight.match(/\|\| k >= (\d+)u\) \{ return; \}/) ?? [])[1]);
    const F = makeFogFrame(), twin = Number((fsrc.readFileSync(`${ROOT}/src/render/shaders/fogofwar.wgsl`, 'utf8').match(/\/\/#C textureStore\(fowHintOut, vec2i\(i32\(gid\.x\), i32\(gid\.y\) \+ (\d+)\)/) ?? [])[1]);
    const live = /p\.dispatchWorkgroups\(FOG_BINS \/ 64, count\); p\.end\(\);/.test(fsrc.readFileSync(`${ROOT}/src/render/fogofwar.ts`, 'utf8'));
    verdict(twin === FOG_EYES && live && FOG_EYES === 7 && FOG_EYES_ON === FOG_EYES && (FOG_DEFAULTS as any).eyes === FOG_EYES_ON && FOG_EYES_WAS === 5 && [...a, ...b, ...c].every(n => n === FOG_EYES) && dist === FOG_BINS * FOG_EYES / 4 && guard === FOG_EYES && F.eyes.length === FOG_EYES * 4 && F.looks.length === FOG_EYES * 4 && FOG_SIGHT_BYTES <= 65536 && FOG_SIGHT_BYTES % 16 === 0,
      'the room: the renderer\'s block and frame, the sight table and the three shaders that read them all carry seven eyes, the table\'s pass has room for seven rows and walks the live ones, the counting twin of the hint pass writes its costs under the seventh row, and the table still fits a uniform block; the module ships the row at seven',
      `FOG_EYES ${FOG_EYES} · the page's ${FOG_EYES_ON}, as it was ${FOG_EYES_WAS} · fogofwar.wgsl ${a.join('/')} · fogsight.wgsl ${b.join('/')}, its guard ${guard} · common.wgsl ${c.join('/')}, dist ${dist} of ${FOG_BINS * FOG_EYES / 4} · the frame's rows ${F.eyes.length / 4} · the counting twin's row ${twin} · the sight pass walks the live rows: ${live} · the table ${FOG_SIGHT_BYTES} B of 65536`);
  }
  // (until 2026-11-07: the room clear with a squad of four and three cameras sending, red on the last TypeScript game as on this one)
  { // a fight with no camera out: nobody past the fifth place
    const fight = async (seed: number) => { const Q = await rig(seed, { fog: true, chaos: 0.5 }); Q.step(40); Q.NB.order(); Q.step(60 * 7); Q.NB.order(); Q.step(60 * 14);
      return { state: Q.NB.state as string, most: Q.FW.eyes.length }; };
    const rows: string[] = []; let fought = true, most = 0;
    for (const seed of [23, 24]) { const b = await fight(seed); most = Math.max(most, b.most); fought &&= b.state !== 'staged'; rows.push(`seed ${seed}: ${b.state}`); }
    verdict(fought && most <= 5, 'the night breach under the fog, the team ordered in, two seeds, 21 s each, no camera out: nobody is past the fifth place',
      `${rows.join(' · ')} · eyes at the end: ${most}`);
  }
  Object.assign(FOG, FOG_DEFAULTS);
  return done();
}

// WHAT THE TEAM DOES IS ALWAYS SHOWN (game/fogofwar.ts FOG.ours — the maintainer, of the tutorial's wall charge: 'still can't see the wall shot detonate').
// Headless this asks (1) WHOSE a doing is: the player's and a teammate's are the team's, a defender's and nobody's are not, and nothing is with the row off;
// (2) THE LIGHTS: a flash somebody made without carrying it takes his class (lights.ts RtLight.fogBy) — the team's lights the fog's picture, a defender's
// and an ownerless one only what the team sees, as ever — asked of flashes made here and of the real stun charge (combat.ts detonateStun) set off where
// nobody sees, by the player and by a defender; the other sites as written (the frag, the wall charge's afterglow, a round's strike, a tracer's ember);
// (3) THE LOOSE BOXES: a one-frame box a teammate owns (a tracer's streak) over floor nobody sees is handed to the renderer, a defender's and nobody's
// are not, and the same of a chunk of RUBBLE by who threw it (debris.ts Chunk.who), in flight and once it lies; (4) THE PARTICLES: what is filed while one of the team is the doer carries the mark, what a defender files does not, a doing inside a doing
// keeps its cause, and the mark is back off afterwards — on a stand-in for the GPU system, round a real round fired by a teammate and by a defender, and a
// teammate's round that stops in a man nobody sees (nothing marked off him, no flash given the shooter) — and
// the record's bit, the kernel's and the draw's lines as written; (5) the address, the journal (and that the replay leaves the row as the tape has it), the stands, and the fight digesting
// the same either way. What a blast looks like under the fog is a real page's to show and a GPU's to judge.
//   FOGOURS_BREAK=1 → every doing the probe stages for the player or a teammate is staged as nobody's: whose, the lights', the boxes' and the particles' lines FAIL.
// The row and the full picture are settled on and no longer written. Until 2026-11-07 every line also asked the row off ('nothing is with the row off',
// 'with the row off every one of them does, as it was', 'none of it with the row off', the mark 'off 00', the rubble 'the row then put off'), and (5)
// asked ?fogours on the address, a recording from before the key, the two stands, and the fight by digest and dice with the doings shown as without. Also until
// that day: (2) red on the last TypeScript game as on this one; (3) and (4) staged by a box list, a stand-in for the particles and a wrapped hush handed to the fog
// on the copy, which the Rust game does not ask. What is left is whose a doing is, and the record's bit as written.
export async function fogOurs(): Promise<boolean> {
  const { verdict, done } = family('fog-ours: what the team does is always shown under the fog of war');
  const brk = process.env.FOGOURS_BREAK === '1';
  const R = await rig(59, { fog: true }); const { game, FW } = R;
  const fsrc = await import('node:fs'), src = (f: string): string => fsrc.readFileSync(`${ROOT}/src/${f}`, 'utf8');
  const PID = brk ? 0 : game.player.char.id;
  R.step(60);   // (past the first half second, in which the fog takes every thing as it stands)
  const gs: any[] = game.guards.filter((x: any) => x.state !== 'dead'), hid = gs.find((x: any) => FW.hidden(x)), mate = gs.find((x: any) => FW.onTeam(x));
  const HID = hid?.char.id ?? 0, MATE = brk ? 0 : mate?.char.id ?? 0;
  { // whose
    const on = [FW.oursId(PID), FW.oursId(MATE), FW.oursId(HID), FW.oursId(0), FW.oursId(undefined)];
    verdict(!!hid && !!mate && on[0] && on[1] && !on[2] && !on[3] && !on[4], 'the player\'s doing and a teammate\'s are the team\'s own; a defender\'s and nobody\'s are not',
      `the player ${on[0]} · a teammate (${mate ? mate.callsign : 'NONE'}) ${on[1]} · a hidden defender (${hid ? hid.callsign : 'NONE'}) ${on[2]} · nobody ${on[3]} / ${on[4]}`);
  }
  { // the particles, as written
    const ts = src('render/particles.ts'), em = src('render/shaders/particles.wgsl'), pc = src('render/shaders/particles_common.wgsl'), dr = src('render/shaders/particles_draw.wgsl');
    const bits = [...pc.matchAll(/const (PB_[A-Z]+): u32 = (\d+)u;/g)].filter(m => m[1] !== 'PB_KIND').map(m => Number(m[2])), clash = bits.some((b, i) => bits.some((c, j) => i !== j && (b & c) !== 0)) || bits.some(b => b <= 15 || (b > 255 && b < 65536));   // (clear of the kind's four bits and of the owner's eight, 8‥15: a flag is under them or over them — PB_KEPT is bit 16)
    const text = /waited: 0, ours: this\.ours \}\);/.test(ts) && /\| \(ring << 16\) \| \(e\.ours \? 1 << 18 : 0\)\) >>> 0;/.test(ts) && /let ours = \(rec\.k\.x >> 18u\) & 1u;/.test(em) && /p\.bits = kind \| PB_NEW \| \(owner << 8u\) \| \(ours \* PB_OURS\);/.test(em)
      && /const PB_OURS: u32 = 128u;/.test(pc) && !clash && (em.match(/&= ~\(PB_ASLEEP \| PB_OURS \| PB_KEPT\)/g) ?? []).length === 3 && /if \(kind != P_RAIN && \(p\.bits & PB_OURS\) == 0u && !\(\(p\.bits & PB_KEPT\) != 0u && fogKeptOn\(\)\) && fogSeenBins\(p\.pos\) < 0\.5\) \{ return cull\(\); \}/.test(dr);
    verdict(text,
      'the record carries the bit, the kernel puts it on the particle, no other particle bit shares it, a particle that has lain and is woken loses it (the kernel\'s three wakes), and the draw leaves a marked particle uncut',
      `the record, the kernel, the bits and the draw as written: ${text} (particle bits ${bits.join(', ')})`);
  }
  Object.assign(FOG, FOG_DEFAULTS);
  return done();
}

// WHAT OTHERS LEFT BEHIND STAYS AS THE TEAM LAST SAW IT (game/fogofwar.ts FOG.kept — the third of the fog's principles; the picture alone, under the full
// picture). Headless this asks (1) A LAMP'S FACE: a lit lamp with a face, put out where the team has no look at it, keeps the level's own emission on its
// boxes for the PICTURE (Box.fogEmit, which the upload alone reads) while the game's Box.emissive is the real lamp's, dark; the frame an eye has it the face
// is the lamp's again; lit again unseen the face is drawn dark; a lamp the team sees, the row off, a fog that is not the full picture: nothing is written;
// (2) LITTER AT REST, as written: the frame carries the row, the sight table's head carries its bit, and the particles' kernel marks a resting particle on
// seen floor with the bit the draw already passes under the fog; (3) the address; (4) the journal; (5) the stands; (6) a fought breach with the row and
// without: the same run by digest and dice, both seeds — it is the picture alone.   FOGKEPT_BREAK=1 → the fog is put off before anything is asked:
// the face's and the frame's lines FAIL. NEVER SEEN ON A GPU: the upload's two reads and the kernel's line are held as text here, and by the page boot.
// The row and the full picture are settled on and no longer written. Until 2026-11-07 (1) also asked 'the row off (the ghost light still kept, as it was)'
// and 'a fog that is not the full picture: nothing is written', (2) the frame without the row, (3) to (5) the address, a recording from before the key and
// the two stands, and (6) held the fight to the same digest and dice without the row as with it; what is left of (6) is that a face is kept in a fight.
export async function fogKept(): Promise<boolean> {
  const { verdict, done } = family('fog-kept: what others left behind stays as the team last saw it in the fog\'s picture');
  const brk = process.env.FOGKEPT_BREAK === '1';
  const R = await rig(63, { fog: true }); const { game, FW } = R;
  if (brk) FW.on = false;
  const fsrc = await import('node:fs'); const src = (f: string): string => fsrc.readFileSync(`${ROOT}/src/${f}`, 'utf8'), strip = (t: string) => t.split('\n').map(l => l.replace(/\/\/.*$/, '')).join('\n');
  R.step(30);
  const statics = game.engine.world.statics as any[], sum = (e: number[] | undefined) => e ? e[0] + e[1] + e[2] : -1;
  { // a lamp's face
    const faced = (game.targets as any[]).filter(t => (t.light || t.areaLights) && t.boxes.length > 0 && sum(t.baseEmissive) > 0 && t.on && !t.broken && !(t.disabledUntil > 0));
    const far = faced.find(t => !FW.lampSeen(t)), near = faced.find(t => FW.lampSeen(t));
    const face = (t: any) => { const bs = t.boxes.map((bi: number) => statics[bi]); return { shown: bs.every((b: any) => b.fogEmit !== undefined), none: bs.every((b: any) => b.fogEmit === undefined && b.fogMark === undefined),
      lit: bs.every((b: any) => sum(b.fogEmit) > 0 && b.fogMark === 1), dark: bs.every((b: any) => sum(b.fogEmit) === 0 && b.fogMark === 0), real: bs.reduce((n: number, b: any) => n + sum(b.emissive), 0) as number }; };
    const { setBlackout } = await import(`${ROOT}/src/game/lighting.ts`);
    let asGlowed = false, blackout = true, lifted = false, relaid = false; const wasGlow = far ? far.boxes.map((bi: number) => sum(statics[bi].emissive)) : [];
    let before = false, kept = false, realDark = false, ghost = false, fresh = false, nearNone = true;
    if (far) {
      before = face(far).none && face(far).real > 0;
      game.lightHome.stage(far, false, null); R.step(3); { const f = face(far); kept = f.shown && f.lit; realDark = f.real === 0; asGlowed = far.boxes.every((bi: number, i: number) => Math.abs(sum(statics[bi].fogEmit) - wasGlow[i]) < 1e-6);
        FW.viewer = true; game.fogLift(true); lifted = face(far).none; game.fogBack(); relaid = face(far).shown && face(far).lit; FW.viewer = false;   // (the viewer's picture without the fog: the lamps as they are — and the step's faces again after it)
        if (far.mains) { setBlackout(game, 3600, 'debug'); R.step(3); blackout = face(far).none; game.endBlackout(); R.step(3); blackout &&= face(far).shown; R.step(360); } }   // (… and the supply's restrike run out, so the lamp glows whole again for what follows)
      game.lightHome.stage(far, true, null); FW.reset(); R.step(3); game.lightHome.stage(far, false, null); R.step(3);
      { ghost = [far.light, ...(far.areaLights ?? []).map((a: any) => a.light)].filter(Boolean).every((l: any) => l.fogGhost > 0); }
      game.lightHome.stage(far, true, null);
      FW.reset(); R.step(3); fresh = face(far).none;
    }
    if (near) { game.lightHome.stage(near, false, null); R.step(2); nearNone = face(near).none; game.lightHome.stage(near, true, null); R.step(2); }
    const boxes = src('scene/boxes.ts'), reads = (boxes.match(/em = b\.fogEmit \?\? b\.emissive/g) ?? []).length, gains = (boxes.match(/b\.fogMark \?\? b\.markGain \?\? 1/g) ?? []).length, raw = (strip(boxes).match(/b\.emissive\[/g) ?? []).length;
    const others = (fsrc.readdirSync(`${ROOT}/src/game`) as string[]).filter(f => f.endsWith('.ts') && f !== 'fogofwar.ts' && /fogEmit|fogMark/.test(strip(src(`game/${f}`))));
    verdict(!!far && before && kept && asGlowed && lifted && relaid && blackout && realDark && ghost && fresh && nearNone && reads === 2 && gains === 2 && raw === 0 && others.length === 0,
      'a lit lamp with a face, switched off where the team has no look at it: its boxes keep for the picture the emission they glowed with while the team knew it lit, and their markings their gain; the viewer\'s picture without the fog shows the lamps as they are and the faces are back after it; a blackout, known to all, keeps no lit face; while the game\'s own emission of them is the real lamp\'s, dark, and its light is the fog\'s ghost; a fresh encounter, a lamp the team sees: nothing is written. The upload reads the picture\'s emission in its two places and no bare one, and no other game module reads the picture\'s',
      `${faced.length} lit lamps with a face, one unseen: ${far ? far.name : '—'} · before: untouched ${before} · off, unseen → face lit ${kept}, as it glowed ${asGlowed}, fog lifted for a draw → untouched ${lifted}, back after ${relaid}, blackout (${far?.mains ? 'on the mains' : 'not on the mains: not asked'}) → none then back ${blackout}, the game's emission dark ${realDark}, ghost light ${ghost} · fresh encounter → untouched ${fresh} · a seen one (${near ? near.name : 'none in sight: not asked'}) off → untouched ${nearNone} · boxes.ts reads fogEmit ${reads} of 2, fogMark ${gains} of 2, a bare b.emissive[ ${raw} · other game modules reading them: ${others.join(', ') || 'none'}`);
  }
  { // litter at rest, as written
    R.step(2); const on = FW.frame.kept;
    const up = src('render/fogofwar.ts'), common = strip(src('render/shaders/common.wgsl')), kern = strip(src('render/shaders/particles.wgsl')), draw = strip(src('render/shaders/particles_draw.wgsl'));
    const head = /U\[o \+ 7\] = \(F!\.air \? 1 : 0\) \+ \(thin \? 2 : 0\) \+ \(F!\.kept \? 4 : 0\);/.test(up);
    const asks = /fn fogKeptOn\(\) -> bool \{ return fogSight\.cfg\.y == 1\.0 && \(u32\(fogSight\.soft\.w \+ 0\.5\) & 4u\) != 0u; \}/.test(common);
    const marks = /else if \(fogKeptOn\(\) && \(p\.bits & PB_KEPT\) == 0u && fogSeenBins\(p\.pos\) >= 0\.5\) \{ p\.bits \|= PB_KEPT; \}/.test(kern), once = (kern.match(/fogKeptOn\(\)/g) ?? []).length === 1;
    const pcs = strip(src('render/shaders/particles_common.wgsl')), bitsOf = [...pcs.matchAll(/const PB_(\w+): u32 = (\d+)u;/g)].map(m => [m[1], Number(m[2])] as [string, number]), keptBit = bitsOf.find(b => b[0] === 'KEPT')?.[1] ?? 0;
    const own = keptBit > 0 && (keptBit & 0xffff) === 0 && bitsOf.every(b => b[0] === 'KEPT' || (b[1] & keptBit) === 0);   // (its own bit: past the kind, the flags and the owner's eight)
    const resting = /\} else if \(asleep\) \{[\s\S]{0,700}fogKeptOn\(\)/.test(kern), woken = (kern.match(/&= ~\(PB_ASLEEP \| PB_OURS \| PB_KEPT\)/g) ?? []).length === 3;
    const drawn = /if \(kind != P_RAIN && \(p\.bits & PB_OURS\) == 0u && !\(\(p\.bits & PB_KEPT\) != 0u && fogKeptOn\(\)\) && fogSeenBins\(p\.pos\) < 0\.5\) \{ return cull\(\); \}/.test(draw);
    const dir = `${ROOT}/src/render/shaders`, elsewhere = (fsrc.readdirSync(dir) as string[]).filter(f => f.endsWith('.wgsl') && !['common.wgsl', 'particles.wgsl', 'particles_draw.wgsl'].includes(f) && /fogKeptOn/.test(strip(fsrc.readFileSync(`${dir}/${f}`, 'utf8'))));
    verdict(on === true && head && asks && marks && once && own && resting && woken && drawn && elsewhere.length === 0,
      'litter at rest, as written: the frame carries the row and the sight table\'s head its bit (so it is the full picture\'s alone); the particles\' kernel, in its resting branch and nowhere else, marks a particle that lies on floor the team sees with a bit of its own (PB_KEPT, clear of the kind, the flags and the owner), which the draw passes under the fog only while the row is on (so the row put off mid-game cuts them again); the three places that wake a particle take the bit off; no other shader asks the row',
      `the frame carries the row: ${on} · the head's bit: ${head} · common.wgsl fogKeptOn: ${asks} · the kernel's line: ${marks}, once: ${once}, its own bit (${keptBit}): ${own}, in the resting branch: ${resting} · woken takes it off in 3 places: ${woken} · the draw passes the bit: ${drawn} · other shaders asking: ${elsewhere.join(', ') || 'none'}`);
  }
  { // a fight: a face is kept
    const fight = async (seed: number) => { const Q = await rig(seed, { fog: true, chaos: 0.5 }); let faces = 0;
      Q.step(40); const lamp = (Q.game.targets as any[]).find(t => (t.light || t.areaLights) && t.boxes.length > 0 && t.baseEmissive[0] + t.baseEmissive[1] + t.baseEmissive[2] > 0 && t.on && !t.broken && !Q.FW.lampSeen(t)); if (lamp) Q.game.lightHome.stage(lamp, false, null);   // (a lamp the fog has as lit, put out where nobody looks: the row has a face to keep while the fight goes)
      Q.step(3); faces = (Q.game.engine.world.statics as any[]).filter(b => b.fogEmit !== undefined && b.fogEmit[0] + b.fogEmit[1] + b.fogEmit[2] > 0).length; Q.NB.order(); Q.step(60 * 7); Q.NB.order(); Q.step(60 * 14);
      return { state: Q.NB.state as string, kept: Q.FW.frame.kept, faces, lamp: lamp?.name as string | undefined }; };
    const rows: string[] = []; let fought = true, told = true;
    for (const seed of [23, 24]) { const b = await fight(seed); fought &&= b.state !== 'staged'; told &&= b.kept === true && b.faces > 0;
      rows.push(`seed ${seed} (${b.lamp ?? 'no lamp unseen'} put out): ${b.state}, faces kept: ${b.faces}`); }
    verdict(fought && told, 'the night breach under the fog\'s full picture with a lamp put out where nobody looks, the team ordered in, two seeds, 21 s each: it is fought, the frame carries the row and the lamp\'s face is kept',
      rows.join(' · '));
  }
  Object.assign(FOG, FOG_DEFAULTS);
  return done();
}
