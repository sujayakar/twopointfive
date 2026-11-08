// The 'glasspen' probe family (`bun run tools/qa/probes.ts glasspen | glasspen-player | glasspen-ai | glasspen-rule | glasspen-lines | glasspen-near`;
// all ride 'glass' too): ROUNDS THROUGH GLASS (src/game/glasspen.ts, rebuild/glass/penetration.md) — a round that meets a sheet of glass goes on through
// it with some of its energy gone and a small turn, and hits what is behind, whoever fired it; the line of FIRE (col.shotBlocked → guards.ts clearShot) answers by
// the same toll, so a man shoots through a window exactly when his round would get through it. Staged on the real game stood up headless, PASS / FAIL each with
// the numbers. The rule and its row "defenders shoot through glass" are settled: nothing here writes them or asks the game without them (until 2026-11-07
// `ai` had a block with the row off and one with the switch off, and `lines` two lines against the switch off).
import { standUp, ROOT } from './headless.ts';
import { floatPartition } from './office-pane.ts';
const { v3 } = await import(`${ROOT}/src/math/vec.ts`);
const { glassPanes, isSheet } = await import(`${ROOT}/src/game/glass.ts`);
const P = await import(`${ROOT}/src/game/glasspen.ts`);
const { GLASSPEN, GLASSPEN_STATS, GLASSPEN_HOOKS, glassPenLive, roundPath, paneToll, paneType, roundEnergy, holeCount } = P;
const { fireWeapon, armThrough } = await import(`${ROOT}/src/game/combat.ts`);
const { glassBreakLive } = await import(`${ROOT}/src/game/glassbreak.ts`);
const { WEAPONS } = await import(`${ROOT}/src/game/weapons.ts`);
const { resetChase, clearShot, shotOrigin } = await import(`${ROOT}/src/game/guards.ts`);

type Vec3 = [number, number, number];
const DT = 1 / 60;
function seeded(a: number) { return () => { a |= 0; a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ (a >>> 15), 1 | a); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; }; }
const f2 = (x: number) => x.toFixed(2), f3 = (x: number) => x.toFixed(3);
type Verdict = (ok: boolean, label: string, detail: string) => void;
function mkVerdict(): { verdict: Verdict; all: () => boolean } { let allPass = true; return { verdict: (ok, label, detail) => { allPass &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); }, all: () => allPass }; }
/** run `fn` with the rule's numbers in `knobs`, each put back whatever happens, and the crossing hook with them */
async function withPen<T>(fn: () => Promise<T>, knobs: Record<string, number> = {}): Promise<T> {
  const old: Record<string, number> = {}; for (const k in knobs) { old[k] = (GLASSPEN as any)[k]; (GLASSPEN as any)[k] = knobs[k]; }
  const hook = GLASSPEN_HOOKS.crossed;
  try { return await fn(); } finally { GLASSPEN_HOOKS.crossed = hook; for (const k in old) (GLASSPEN as any)[k] = old[k]; }
}

function parkFrom(game: any, keep: number) {
  game.guards.slice(keep).forEach((o: any, i: number) => { o.hold = true; o.pinned = true; o.dazzledUntil = 1e9; o.heardUpTo = 1e12; o.char.pos = [37.5, 0, 25 + i * 1.2]; o.char.vel = [0, 0, 0]; o.path = []; o.pathGoal = null; o.flashlight.enabled = false; o.char.update(0); });
}
function placeGuard(gd: any, p: Vec3, face: Vec3, hold = true) {
  gd.char.pos = v3.copy(p); gd.char.vel = [0, 0, 0]; gd.speed = 0; gd.char.bodyYaw = gd.char.aimYaw = Math.atan2(face[0] - p[0], face[2] - p[2]); gd.char.aimPitch = 0;
  gd.state = 'patrol'; gd.awareness = 0; gd.lastKnown = null; gd.reactT = 0; gd.searchT = 0; gd.task = null; gd.bodyDuty = null; gd.path = []; gd.pathGoal = null; gd.hold = hold; gd.pinned = false; gd.dazzledUntil = -1; gd.heardUpTo = 0;
  gd.char.update(0); gd.char.update(DT);
}
/** a man who stands where he is put and notices nothing (the target of a staged round) */
function dummy(gd: any, p: Vec3, face: Vec3) { placeGuard(gd, p, face); gd.pinned = true; gd.dazzledUntil = 1e9; gd.heardUpTo = 1e12; gd.flashlight.enabled = false; }
function armAlert(gd: any, game: any, fix: Vec3) {
  gd.hold = false; gd.pinned = false; gd.dazzledUntil = -1; gd.state = 'alert'; gd.awareness = 1; gd.reactT = 0; gd.fireCd = 0.2; gd.weapon.rearm(); gd.reloadT = -1;
  gd.lastKnown = v3.copy(fix); gd.lastSeenT = game.time; gd.ledT = game.time; gd.stuckT = 0; resetChase(gd);
}
async function floor(seed: number, sam: Vec3) {
  Math.random = seeded(seed);
  const W = await standUp(); const { game } = W; floatPartition(game);
  game.godMode = true; game.aiEnabled = true; game.playerInvisible = false; game.infiniteAmmo = true;
  game.teleportPlayer(v3.copy(sam)); game.player.visibility = 0; game.player.crouch = false;
  for (let i = 0; i < 10; i++) W.step(DT);
  game.player.visibility = 0;
  return W;
}
function gunUp(W: any, slot: 'pistol' | 'rifle') { const pl = W.game.player; pl.slot = slot; pl.holstered = false; for (let i = 0; i < 40; i++) W.step(DT); }
const chestOf = (c: any): Vec3 => { const tr = c.bones.spine2 ?? c.bones.hips; return [tr ? tr[0] : c.pos[0], c.pos[1] + 1.25, tr ? tr[2] : c.pos[2]]; };
/** the office's panes by type: the conference partition (z 10, x < 14: plain float), the manager's sidelight (laminated), the lobby frontage (tempered) */
function officePanes(game: any) {
  const all: number[] = glassPanes(game).filter((bi: number) => isSheet(game.level.boxes[bi]));
  const of = (t: string) => all.filter((bi: number) => paneType(game, game.level.boxes[bi]) === t);
  const conf = all.filter((bi: number) => { const b = game.level.boxes[bi]; return Math.abs(b.c[2] - 10) < 0.05 && b.c[0] < 14; }).sort((a: number, b: number) => game.level.boxes[a].c[0] - game.level.boxes[b].c[0]);
  return { all, conf, annealed: of('annealed'), tempered: of('tempered'), laminated: of('laminated') };
}
const lastOf = (game: any, ev0: number, kind: string) => game.events.slice(ev0).filter((e: any) => e.kind === kind).pop();
const stars = (game: any) => game.decals.list.filter((d: any) => d.kind === 'glassHit');
/** the game's blows on glass, counted over every pane, not one pane's: a blow the fracture model files draws its own cracks and has no star */
const hitsOn = (game: any): number => glassStateOf(game).hits;
const { glassState: glassStateOf } = await import(`${ROOT}/src/game/glass.ts`);


// ================================================================ (b) an AI shooter
export async function glasspenAi(): Promise<boolean> {
  console.log('\n=== glasspen-ai: a man alert on the corridor with Sam lit and still in the conference room, only the pane between them — his line of FIRE is clear through the glass by his own gun\'s energy, he stands and fires through it, and his rounds reach Sam; with the switch off the same man has no shot there (the base) ===');
  const { verdict, all } = mkVerdict();
  const duel = async (seed: number, secs: number) => {
    const sam: Vec3 = [12.0, 0, 7.0], at: Vec3 = [12.0, 0, 11.2];
    const W = await floor(seed, sam); const { game, audioCounts } = W;
    parkFrom(game, 1); const gd = game.guards[0];
    for (const dd of game.doors.list) dd.reset();
    game.teleportPlayer(v3.copy(sam)); game.player.visibility = 0.85; game.puppet = { goal: v3.copy(sam), aim: [at[0], 1.2, at[2]] };
    placeGuard(gd, at, sam, false); gd.char.anim.upper = 'aim'; gd.char.update(0); gd.char.update(DT); armAlert(gd, game, game.player.char.pos);
    const lineAt0 = clearShot(game, gd.char, chestOf(game.player.char)), sightAt0 = !game.col.segmentBlocked(shotOrigin(gd.char), chestOf(game.player.char)), solidAt0 = game.col.solidBlocked(shotOrigin(gd.char), chestOf(game.player.char));
    let rounds = 0, through = 0, firstHit = -1, ground = 0, last = v3.copy(gd.char.pos); const t0 = game.time, c0 = audioCounts.get('glassCrack') ?? 0; let seenEv = game.events.length; const p0 = v3.copy(gd.char.pos);
    for (let f = 0; f < 60 * secs; f++) {
      W.step(DT);
      for (const e of game.events) { if (e.kind !== 'guardShot' || e.time <= t0 || (e as any).__seen) continue; (e as any).__seen = true; rounds++; if (e.to[2] < 9.9) through++; }
      if (game.player.hitFlash > 0.94 && firstHit < 0) firstHit = game.time - t0;
      ground += v3.distXZ(gd.char.pos, last); last = v3.copy(gd.char.pos);
    }
    void seenEv; game.puppet = null;
    return { game, gd, lineAt0, sightAt0, solidAt0, rounds, through, firstHit, ground, moved: v3.distXZ(p0, gd.char.pos), cracks: (audioCounts.get('glassCrack') ?? 0) - c0 };
  };
  await withPen(async () => {
    const R = await duel(931, 5);
    verdict(R.lineAt0 && R.sightAt0 && R.solidAt0, 'from where he stands: he SEES Sam through the pane, the solid walk (sound, feet) is still stopped by it, and his line of FIRE is clear — clearShot asks the toll of his own gun\'s round',
      `sight ${R.sightAt0 ? 'clear' : 'blocked'} · solid ${R.solidAt0 ? 'blocked' : 'clear'} · clearShot ${R.lineAt0} · gun ${R.gd.weapon.def.id} energy ${f2(roundEnergy(R.gd.weapon.def))}`);
    verdict(R.rounds >= 1 && R.through >= 1 && R.firstHit >= 0 && R.firstHit < 4 && R.cracks >= 1 && R.moved < 1.5,
      'he stands his ground and FIRES through the glass: rounds leave, at least one flies on past the pane, a round of his reaches Sam inside four seconds, the glass is heard — he did not walk round for the door',
      `rounds ${R.rounds} (${R.through} stopped beyond the pane) · first round in Sam ${R.firstHit >= 0 ? f2(R.firstHit) + ' s' : 'never'} · cracks ${R.cracks} · ground ${f2(R.ground)} m, ends ${f2(R.moved)} m from his start, ${R.gd.state}`);
    // one deliberate round, no dice: a guard's steady round at Sam's chest through the pane lands in him
    const W = await floor(977, [12.0, 0, 7.0]); const { game } = W; parkFrom(game, 1); const gd = game.guards[0]; placeGuard(gd, [12.0, 0, 11.2], [12.0, 0, 7.0]); gd.char.anim.upper = 'aim'; for (let i = 0; i < 20; i++) W.step(DT);
    const ev0 = game.events.length; game.player.hitFlash = 0; fireWeapon(game, gd.char, chestOf(game.player.char), false, { steady: true });
    const shot = lastOf(game, ev0, 'guardShot');
    verdict(game.player.hitFlash === 1 && !!shot && shot.to[2] < 9.9 && !!lastOf(game, ev0, 'glass'), 'one steady 9 mm from the corridor at Sam\'s chest: through the pane and into him (hit registered), the pane starred on the way', `stopped (${shot?.to.map(f2)}) · hitFlash ${game.player.hitFlash}`);
  });
  return all();
}

// ================================================================ (c) the rule's numbers
export async function glasspenRule(): Promise<boolean> {
  console.log('\n=== glasspen-rule: the toll by gun and glass — a 9 mm goes through plain and tempered panes and DIES in the laminated sidelight, a rifle round goes through that too; a graze stops; the fourth plain pane stops a 9 mm; a round spent in a pane is the old stop ===');
  const { verdict, all } = mkVerdict();
  await withPen(async () => {
    const W = await floor(5150, [20, 0, 11.3]); const { game } = W; parkFrom(game, 0);
    const O = officePanes(game);
    verdict(O.conf.length === 6 && O.annealed.length >= 6 && O.tempered.length >= 1 && O.laminated.length >= 1, 'the office has all three kinds of sheet (typed by the fracture model: GlassBreak.typeOf)', `sheets ${O.all.length}: annealed ${O.annealed.length} · tempered ${O.tempered.length} · laminated ${O.laminated.length}`);
    // the table: a square-on crossing of one pane of each type by each row
    const sq = (bi: number, e: number, nth = 0, slant = 0) => { const b = game.level.boxes[bi]; const n: Vec3 = b.h[0] < b.h[2] ? [1, 0, 0] : [0, 0, 1]; const t: Vec3 = b.h[0] < b.h[2] ? [0, 0, 1] : [1, 0, 0]; const d = v3.normalize(v3.add(v3.scale(n, Math.cos(slant)), v3.scale(t, Math.sin(slant)))); return paneToll(game, bi, v3.mad(b.c, n, -Math.min(b.h[0], b.h[2])), d, e, nth); };
    const rows: string[] = []; const T: Record<string, Record<string, number>> = {};
    for (const id of ['fn2000', 'aks74u', 'fiveseven', 'guard9mm']) { const e = roundEnergy((WEAPONS as any)[id]); T[id] = {}; let row = `${id.padEnd(9)} e ${f2(e)}`; for (const [ty, list] of [['annealed', O.annealed], ['tempered', O.tempered], ['laminated', O.laminated]] as const) { const r = sq(list[0], e); T[id][ty] = r ? r.eOut : -1; row += ` · ${ty} ${r ? `→ ${f2(r.eOut)} (cone ${f2(GLASSPEN.deflectDeg * r.take / e)}°)` : 'SPENT'}`; } rows.push(row); }
    verdict(T.guard9mm.annealed > 0 && T.guard9mm.tempered > 0 && T.guard9mm.laminated < 0 && T.fiveseven.laminated > 0 && T.fn2000.laminated > 0 && T.aks74u.laminated > 0 && T.fn2000.annealed > T.fn2000.tempered && T.fn2000.tempered > T.fn2000.laminated,
      'one 12 mm pane square-on: every gun through plain and tempered glass; the 9 mm service pistol is SPENT in laminated, the 5.7, the carbine and the rifle get through it; plain costs least, laminated most', rows.join('\n    '));
    // slant: more glass in the way, then the graze
    const a30 = sq(O.annealed[0], 1, 0, Math.PI / 6), a70 = sq(O.annealed[0], 1, 0, 70 * Math.PI / 180), a82 = sq(O.annealed[0], 1, 0, 82 * Math.PI / 180), a0 = sq(O.annealed[0], 1);
    verdict(!!a0 && !!a30 && !!a70 && a30.take > a0.take && a70.take > a30.take && a70.take <= a0.take * GLASSPEN.obliqueMax + 1e-9 && a82 === null, 'a slanted crossing pays for the longer way through (capped at ×' + GLASSPEN.obliqueMax + '), and past ~80° off the normal the round skids and is spent in the sheet', `toll at 0° ${f3(a0!.take)} · 30° ${f3(a30!.take)} · 70° ${f3(a70!.take)} · 82° ${a82 ? f3(a82.take) : 'spent'}`);
    // pane after pane
    let e = roundEnergy(WEAPONS.guard9mm), n9 = 0; for (let k = 0; k < 8; k++) { const r = sq(O.annealed[0], e, k); if (!r) break; e = r.eOut; n9++; }
    let er = roundEnergy(WEAPONS.fn2000), nr = 0; for (let k = 0; k < 8; k++) { const r = sq(O.annealed[0], er, k); if (!r) break; er = r.eOut; nr++; }
    let el = roundEnergy(WEAPONS.fn2000), nl = 0; for (let k = 0; k < 8; k++) { const r = sq(O.laminated[0], el, k); if (!r) break; el = r.eOut; nl++; }
    verdict(n9 === 3 && nr === GLASSPEN.maxPanes && nl === 2, 'in a row: a 9 mm gets through three plain panes, a rifle round through the cap of ' + GLASSPEN.maxPanes + ', and through two laminated sheets — never a pane more', `9 mm: ${n9} plain panes (${f2(e)} left) · rifle: ${nr} plain (${f2(er)} left), ${nl} laminated (${f2(el)} left)`);
    // the real thing at the laminated sidelight: a 9 mm stops IN it (the old stop: star, crack), a rifle round goes on
    const bi = O.laminated.find((i: number) => { const q = game.level.boxes[i]; return Math.abs(q.c[2] - 10) < 0.05 && q.c[0] > 26 && q.c[0] < 27; }) ?? O.laminated[0], b = game.level.boxes[bi]; const nz = b.h[0] >= b.h[2];   // (normal along z if the pane runs along x)
    const from: Vec3 = nz ? [b.c[0], b.c[1], b.c[2] + 1.5] : [b.c[0] + 1.5, b.c[1], b.c[2]], to: Vec3 = nz ? [b.c[0], b.c[1], b.c[2] - 1.5] : [b.c[0] - 1.5, b.c[1], b.c[2]];
    const rd = v3.normalize(v3.sub(to, from));
    const p9 = roundPath(game, from, rd, 60, roundEnergy(WEAPONS.guard9mm)), pr = roundPath(game, from, rd, 60, roundEnergy(WEAPONS.fn2000));
    const hit = game.col.raycast(from, rd, 60);
    verdict(p9 === null && !!hit && hit.index === bi && !!pr && pr.cross.length >= 1 && pr.cross[0].bi === bi && pr.len > hit.t + 0.05, `the manager's sidelight (pane ${bi}, laminated), a line square through it: the 9 mm's flight is the old straight line ending IN the pane (no path), the rifle round's crosses it and flies on`, `9 mm path ${p9 === null ? 'none (stops in pane at ' + f2(hit?.t ?? -1) + ' m)' : 'crossed'} · rifle crossed ${pr?.cross.length} pane(s), stopped at ${f2(pr?.len ?? -1)} m, ${f2(pr?.cross[0].eOut ?? 0)} left`);
    // …and the line of fire says the same of each gun
    const e9 = roundEnergy(WEAPONS.guard9mm), eR = roundEnergy(WEAPONS.fn2000); const near = v3.mad(from, rd, 2.2);
    verdict(game.col.shotBlocked(from, near, 0.02, e9) && !game.col.shotBlocked(from, near, 0.02, eR) && game.col.shotBlocked(from, near) && !game.col.segmentBlocked(from, near), 'the line of FIRE across that sidelight: blocked for the 9 mm (and for a question asked with no gun: the cautious default), clear for the rifle; sight passes as ever', `shotBlocked 9 mm ${game.col.shotBlocked(from, near, 0.02, e9)} · rifle ${game.col.shotBlocked(from, near, 0.02, eR)} · no gun ${game.col.shotBlocked(from, near)} · sight blocked ${game.col.segmentBlocked(from, near)}`);
    // a round spent in a pane through the real fireWeapon: the pane keeps its star, nothing flies on
    const gd = game.guards[0]; placeGuard(gd, nz ? [b.c[0], 0, b.c[2] + 1.6] : [b.c[0] + 1.6, 0, b.c[2]], nz ? [b.c[0], 0, b.c[2] - 2] : [b.c[0] - 2, 0, b.c[2]]); gd.char.anim.upper = 'aim'; for (let i = 0; i < 20; i++) W.step(DT);
    const ev0 = game.events.length, s0 = hitsOn(game), spent0 = GLASSPEN_STATS.spent; const m = gd.char.muzzle; const aim: Vec3 = v3.mad(b.c, v3.normalize(v3.sub(b.c, m)), 2);   // (through the middle of the pane, from wherever his muzzle rides)
    fireWeapon(game, gd.char, aim, false, { steady: true }); const shot = lastOf(game, ev0, 'guardShot');
    const stopD = shot ? Math.abs(nz ? shot.to[2] - b.c[2] : shot.to[0] - b.c[0]) : 9;
    verdict(gd.weapon.def.id === 'guard9mm' && stopD < 0.03 && hitsOn(game) === s0 + 1 && GLASSPEN_STATS.spent === spent0 + 1, 'a guard\'s 9 mm fired at it for real: stopped IN the laminated sheet (within 3 cm of its plane), one blow on the pane, counted as spent', `gun ${gd.weapon.def.id} · stopped ${f3(stopD)} m from the pane's plane at (${shot?.to.map(f2)}) · blows +${hitsOn(game) - s0}`);
  });
  return all();
}


// ================================================================ (e) what the pane never hears of, and the muzzle past the glass
export async function glasspenNear(): Promise<boolean> {
  console.log('\n=== glasspen-near: a man on the NEAR side of the pane takes the round and the glass is untouched; a man pressed to the far face is hit by the round as it leaves the glass; Sam pressed to the glazing with his muzzle past it fires on into the room; the restart forgets the holes ===');
  const { verdict, all } = mkVerdict();
  await withPen(async () => {
    // near side: Sam in the corridor, the man between him and the glass
    { const W = await floor(7301, [7.5, 0, 12.4]); const { game } = W; parkFrom(game, 1); const gd = game.guards[0]; dummy(gd, [7.5, 0, 10.9], [7.5, 0, 12.4]); for (let i = 0; i < 5; i++) W.step(DT); gunUp(W, 'pistol');
      const ev0 = game.events.length, s0 = stars(game).length, x0 = GLASSPEN_STATS.crossed; fireWeapon(game, game.player.char, chestOf(gd.char), true); const shot = lastOf(game, ev0, 'shot');
      verdict(gd.state === 'dead' && !!shot && shot.to[2] > 10.2 && stars(game).length === s0 && GLASSPEN_STATS.crossed === x0 && !lastOf(game, ev0, 'glass'), 'the man stands in FRONT of the pane on the round\'s line: he takes it, and the pane behind him has no star, no crack event, no hole', `stopped (${shot?.to.map(f2)}) · guard ${gd.state} · stars +${stars(game).length - s0}`); }
    // far side, pressed to the glass
    { const W = await floor(7302, [7.5, 0, 11.6]); const { game } = W; parkFrom(game, 1); const gd = game.guards[0]; dummy(gd, [7.5, 0, 9.82], [7.5, 0, 11.6]); for (let i = 0; i < 5; i++) W.step(DT); gunUp(W, 'pistol');
      const ev0 = game.events.length, s0 = hitsOn(game); fireWeapon(game, game.player.char, chestOf(gd.char), true); const shot = lastOf(game, ev0, 'shot');
      verdict(gd.state === 'dead' && hitsOn(game) === s0 + 1 && !!shot && shot.to[2] < 10.0 && shot.to[2] > 9.4, 'a man standing AGAINST the far face (18 cm off the glass): the round marks the pane and is in him as it leaves it — it does not pass through him', `stopped (${shot?.to.map(f2)}) · guard ${gd.state} · blows +${hitsOn(game) - s0}`); }
    // …and a round that leaves the glass OVER the head of a man backed onto it passes him (the contact column's crown pad is for a muzzle pressed down on a crouched man)
    { const W = await floor(7304, [7.5, 0, 11.6]); const { game } = W; parkFrom(game, 1); const gd = game.guards[0]; dummy(gd, [7.5, 0, 9.74], [7.5, 0, 6]); for (let i = 0; i < 5; i++) W.step(DT); gunUp(W, 'pistol');
      const top = gd.char.bones.headTop?.[1] ?? 1.8; const m = game.player.char.muzzle; const over: Vec3 = [7.5, top + 0.15, 9.99]; const aim = v3.mad(m, v3.normalize(v3.sub(over, m)), 6);
      const ev0 = game.events.length; fireWeapon(game, game.player.char, aim, true); const shot = lastOf(game, ev0, 'shot');
      verdict(gd.state !== 'dead' && !!shot && v3.dist(shot.to, over) > 0.5, 'a round through the pane 15 cm over the crown of a man standing with his back to the glass: it passes over him and flies on', `crown ${f2(top)} m · stopped (${shot?.to.map(f2)}) · guard ${gd.state}`); }
    // Sam pressed to the glazing: the muzzle already past the sheet
    { const W = await floor(7303, [7.5, 0, 10.42]); const { game } = W; parkFrom(game, 1); const gd = game.guards[0]; dummy(gd, [7.5, 0, 6.8], [7.5, 0, 11.6]);
      game.puppet = { goal: [7.5, 0, 10.42], aim: [7.5, 1.3, 6.8] }; gunUp(W, 'pistol'); for (let i = 0; i < 30; i++) W.step(DT);
      const pc = game.player.char; let poked = armThrough(game, pc); let z = 10.42;
      for (let k = 0; k < 12 && !poked; k++) { z -= 0.03; game.teleportPlayer([7.5, 0, z]); game.puppet = { goal: [7.5, 0, z], aim: [7.5, 1.3, 6.8] }; for (let i = 0; i < 12; i++) W.step(DT); poked = armThrough(game, pc); }
      if (!poked) console.log(`    (could not stage a muzzle past the glass: muzzle z ${f2(pc.muzzle[2])} with Sam at z ${f2(pc.pos[2])} — the collision keeps him off the sheet; the rule is covered by the unit call below)`);
      const ev0 = game.events.length, s0 = hitsOn(game), a0 = GLASSPEN_STATS.viaArm; if (poked) fireWeapon(game, pc, chestOf(gd.char), true); const shot = lastOf(game, ev0, 'shot');
      verdict(!poked || (!!shot && shot.to[2] < 9.0 && hitsOn(game) === s0 + 1 && GLASSPEN_STATS.viaArm === a0 + 1 && gd.state === 'dead'), poked ? 'Sam pressed to the glazing, muzzle past the sheet: the round is NOT put into the panel at the muzzle — the pane is starred at the arm and the round flies on into the man in the room' : 'muzzle-past-the-glass: not stageable on this floor (collision keeps the muzzle short of the sheet) — skipped', poked ? `Sam z ${f2(pc.pos[2])} muzzle z ${f2(pc.muzzle[2])} · stopped (${shot?.to.map(f2)}) · stars +${stars(game).length - s0} · guard ${gd.state}` : '');
      // …and with a man standing against the far face, the muzzle inside him: the contact shot counts (the panel between the two chests is the glass the muzzle is through)
      if (poked) { const g2 = game.guards[1]; const mz = pc.muzzle; dummy(g2, [mz[0], 0, mz[2] - 0.1], [mz[0], 0, 12]); const a1 = GLASSPEN_STATS.viaArm; fireWeapon(game, pc, [mz[0], mz[1], 6.8], true);
        verdict(g2.state === 'dead' && GLASSPEN_STATS.viaArm === a1 + 1, 'muzzle through the glass and pressed into a man standing against its far face: he is hit where he stands', `2nd man ${g2.state} at (${g2.char.pos.map(f2)}) · muzzle (${mz.map(f2)})`); }
      // a checkpoint keeps the holes with the stars: a forced snapshot, a load, the same holes
      { game.puppet = null; for (let i = 0; i < 3; i++) W.step(DT); const before = officePanes(game).conf.map((i: number) => holeCount(game, i)); const snap = W.snapshot('glasspen', true); const hasKey = !!snap && Array.isArray((snap as any).glassPen);
        game.puppet = null; const ok = snap ? W.restore(snap) : null; const after = officePanes(game).conf.map((i: number) => holeCount(game, i));
        verdict(!poked || (hasKey && before.join() === after.join() && before.some((n: number) => n > 0)), 'a checkpoint written with holes in the glass carries them (glassPen slice) and a load puts them back, pane for pane', `holes by pane before [${before}] after [${after}] · slice in file ${hasKey} · restore ${JSON.stringify(ok ?? null).slice(0, 60)}`); }
      // …but not while the fracture model is live: a load leaves every pane whole there, and a hole with no mark on the glass would be a cheap way through it
      { const snap2: any = W.snapshot('glasspen-live', true); const chaos0 = game.tune.chaos, gb0 = game.tune.glassBreak; game.tune.chaos = 0.5; game.tune.glassBreak = true;
        const live = glassBreakLive(game); game.puppet = null; if (snap2) W.restore(snap2); const after2 = officePanes(game).conf.map((i: number) => holeCount(game, i));
        const open2 = officePanes(game).conf.map((i: number) => game.glassBreak.openFraction(i)); game.tune.chaos = chaos0; game.tune.glassBreak = gb0;
        verdict(!poked || (live && Array.isArray(snap2?.glassPen) && after2.every((n: number) => n === 0) && open2.every((o: number) => o === 0)), 'the same file loaded with the fracture model live: every pane whole AND no through-hole put back (both come back or neither)', `fracture live ${live} · slice in file ${Array.isArray(snap2?.glassPen)} · holes after [${after2}] · open [${open2}]`);
        game.puppet = null; if (snap2) W.restore(snap2); const back = officePanes(game).conf.map((i: number) => holeCount(game, i));
        verdict(!poked || back.some((n: number) => n > 0), '…and loaded again with the fracture off: the holes are back (the file was not changed by the refusal)', `holes [${back}]`); }
      // the restart forgets holes and stars alike
      const bi = officePanes(game).conf.find((i: number) => holeCount(game, i) > 0);
      if (bi === undefined) { fireWeapon(game, pc, [7.5, 1.3, 6.8], true); }
      const withHoles = officePanes(game).conf.filter((i: number) => holeCount(game, i) > 0).length;
      game.puppet = null; game.restartEncounter();
      verdict(withHoles >= 1 && officePanes(game).conf.every((i: number) => holeCount(game, i) === 0) && stars(game).length === 0, 'restartEncounter: the holes go with the stars', `panes holed before ${withHoles} · after 0 · stars ${stars(game).length}`); }
  });
  return all();
}
