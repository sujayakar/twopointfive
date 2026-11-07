// The 'frameholds' family: A FRAME HOLDS A BOOT FROM THE FACE ITS DOOR OPENS TOWARD (src/game/doors.ts DOORWAY.frameHolds; the maintainer, 10-08: "for doors just make
// it realistic"). A shut leaf rests on its frame's stop on the face it opens toward: a boot there does not open it — and, the same fact from inside, a wedge or a
// dragged desk holds only a door that swings toward the man who put it there (src/game/digin.ts).
//   bun run tools/qa/probes.ts frameholds | frameholds-tools | frameholds-sam | frameholds-wedge
// All headless; nothing here looks at a picture. FRAMEHOLDS_BREAK=1 (tools, sam) leaves every door hung as it was dealt: lines of the held face must fail.
// 2026-11-07: the frame's rule is settled, and so are the door rules beside it (doors stay where they are left, door manners, off the hinges, the door tested
// before it is forced, the silent squad's picks). Every line used to be run twice, the rule on and the same staging with it off: the off staging is gone from
// all three probes, with the lines that asked only it. GONE WHOLE: `frameholds-boot` (it drove the TypeScript doors alone, kick by kick, with men made up for
// them: that class's body went with the tick, and the Rust game's doors are driven by its tick); `stay-open-digin` (its rule, DIGIN.shutOpen, is settled OFF,
// so every line of it asked the other side); in tools, the silent squad with its picks switched off, the gate's `willKick` (gone with the tick) and the charge
// with off-hinges switched off; in wedge, the stagings on doors with closers and the dock doors' hold test (`wedgeHolds`, gone with the tick). 'No noise' was
// counted through a wrap of the doors' emit on the tools' copy, which the Rust tick never calls: those clauses went. What is said is read from the game's own
// log (heard.ts); a door's wedge from the door's own state (locked, and not by the level).
//   frameholds-tools   the squad's door tools on the office (the manager's door hung the other way, so its locked face is the one it opens toward): 'open and
//                      clear' by a LOUD squad picks the cylinder and goes in — no boot, nothing broken; a keypad door there greys the entry rows with the
//                      reason and the order is refused in the same words, the charge row still live, the words changing when no charge is left or one is
//                      set; a lock no pick opens thrown while they walk ends the order at the door; a charge set on that face still opens it. The squad
//                      stands with its big slots emptied by hand: a man with the hammer or the breaching shotgun takes such a lock himself
//   frameholds-sam     the player: his kick row on that face is greyed with the reason and a tap on it starts no kick; the pick row still works; a sprint into
//                      it starts no kick; from the push face his rows as ever
//   frameholds-wedge   the last three defenders of the compound ordered into one room (LASTSTAND.regroup, an open switch: raised here), that room's doors hung
//                      to open OUT of it: nobody wedges one and each door passed over is counted; hung to open INTO it: wedged; one out and the rest in
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { standUp, ROOT } from './headless.ts';
import { sample } from './sweep.ts';
import { tapVerb } from './verbs.ts';
import { listen } from './heard.ts';
const { DOORTOOLS } = await import(`${ROOT}/src/game/doortools.ts`);
const { NIGHTBREACH } = await import(`${ROOT}/src/game/nightbreach.ts`);
const { ORDERS } = await import(`${ROOT}/src/game/orders.ts`);
const { killGuard } = await import(`${ROOT}/src/game/combat.ts`);
const { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`);
const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
const { startFromQuery } = await import(`${ROOT}/src/game/scenarioRun.ts`);
const { LASTSTAND, lastStandReport } = await import(`${ROOT}/src/game/laststand.ts`);
const { DIGIN_STATS } = await import(`${ROOT}/src/game/digin.ts`);
const { KIT } = await import(`${ROOT}/src/game/kit.ts`);

const DT = 1 / 60, BREAK = process.env.FRAMEHOLDS_BREAK === '1';   // (the red path: no door is hung the other way, so no locked face is one a door opens toward)
function seeded(a: number) { return () => { a |= 0; a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ (a >>> 15), 1 | a); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; }; }
type Verdict = (ok: boolean, label: string, detail: string) => void;
function family(title: string): { verdict: Verdict; done: () => boolean } {
  console.log(`\n=== ${title} ===`); let all = true;
  return { verdict: (ok, label, detail) => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); }, done: () => { console.log(`=== ${title.split(':')[0]}: ${all ? 'ALL PASS' : 'FAILURES above'} ===`); return all; } };
}
const quiet = async <T>(fn: () => Promise<T> | T): Promise<T> => { const log = console.log, err = console.error, warn = console.warn; console.log = console.error = console.warn = () => {}; try { return await fn(); } finally { console.log = log; console.error = err; console.warn = warn; } };
const clone = (o: any) => JSON.parse(JSON.stringify(o));
const KEEP = { DOORTOOLS: clone(DOORTOOLS), NIGHTBREACH: { ...NIGHTBREACH }, LASTSTAND: { ...LASTSTAND }, kit: KIT.members.map((m: any) => ({ ...m })) };
const reset = () => { Object.assign(DOORTOOLS, clone(KEEP.DOORTOOLS)); Object.assign(NIGHTBREACH, KEEP.NIGHTBREACH); Object.assign(LASTSTAND, KEEP.LASTSTAND); KIT.members.splice(0, KIT.members.length, ...KEEP.kit.map((m: any) => ({ ...m }))); };
const guarded = (fn: () => Promise<boolean>) => async () => { try { return await fn(); } catch (e) { console.log(`- FAIL — the probe threw\n    ${e instanceof Error ? e.stack : String(e)}`); return false; } finally { reset(); } };
const f2 = (x: number) => (Number.isFinite(x) ? x.toFixed(2) : '—');
function levelOfFile(file: string): any { const data = parseLevelData(readFileSync(resolve(ROOT, 'levels', file), 'utf8')); return loadLevel(data); }
async function boot(file?: string): Promise<any> { Math.random = seeded(7); return quiet(() => standUp({ uploadEvery: 4, ...(file ? { level: levelOfFile(file) } : {}) })); }
/** a point off a door's face: `along` m from the hinge along the closed leaf, `o` m out of face `f` (Doors.side's sign) */
const off = (d: any, f: number, o: number, along = d.def.width * 0.55): [number, number, number] => { const c = d.def.closedDir, ux = Math.cos(c), uz = Math.sin(c); return [d.def.hinge[0] + ux * along - uz * o * f, 0, d.def.hinge[1] + uz * along + ux * o * f]; };
/** the face (Doors.side's sign) a shut door opens TOWARD */
const pullSide = (g: any, d: any): 1 | -1 => g.doors.pullFace(d, off(d, 1, 0.8)) ? 1 : -1;
/** hang the door so the face `f` is the one it opens toward */
const hangToward = (g: any, d: any, f: number) => { if (BREAK) return; if (pullSide(g, d) !== f) d.setSwing(-d.swing as 1 | -1); };
const fresh = (d: any, lock = false) => { d.reset(); d.angle = 0; d.vel = 0; d.latched = true; d.closing = false; d.locked = lock; d.place(); };
// ---------------------------------------------------------------- the room clear on the office, its squad of four (as silent-probes.ts stands it)
const AISLE: number[] = [15, 0, 17.6];
async function rig(seed: number, o: { charges?: number; big?: boolean } = {}) {
  reset(); Object.assign(NIGHTBREACH, { squad: 4 }); if (o.charges !== undefined) DOORTOOLS.charges = o.charges;
  // (THE SQUAD'S BIG SLOTS EMPTIED BY HAND before the game stands, as on Team & kit: a man with the hammer or the breaching shotgun takes a lock on the face its door
  //  opens toward (bigkit.ts), and these lines ask what a squad does that has neither. `big: true` leaves the kit as issued)
  if (!o.big) for (let i = 0; i < KIT.members.length; i++) KIT.members[i] = { ...KIT.members[i], big: 'none', bare: true };
  Math.random = seeded(seed);
  const W: any = await quiet(() => standUp()); const game = W.game, input = W.input;
  const RC = game.roomClear, NB = game.nightBreach, O = game.orders, D = game.doorTools;
  game.godMode = true;
  const ears = listen(game);
  const why = RC.start(seed); const seen = new Set<any>();
  const R = { W, game, input, O, D, why, kicks: 0, note: '',
    /** what the log has shown, read now (heard.ts) */
    get said(): string[] { ears.poll(); return ears.said; },
    get team(): any[] { return O.squad(); },
    step(n = 1) { for (let i = 0; i < n; i++) { game.player.visibility = 0.05; W.step(DT); ears.poll();
      for (const e of game.events) { if (seen.has(e)) continue; seen.add(e); if (e.kind === 'kick') R.kicks++; } } },
    until(pred: () => boolean, secs: number) { for (let i = 0; i < secs * 60 && !pred(); i++) R.step(); return pred(); },
    door(name: string): any { return game.doors.list.find((d: any) => d.def.name === name); },
    target(name: string) { return O.targetAt(R.door(name).frameCentre); },
    give(id: string, name: string): string | null { return O.give(id, R.target(name)); },
    /** an order given on the floor (the posture rows) */
    give2(id: string): string | null { return O.give(id, O.targetAt(AISLE)); },
    beats(op: any): string { return (op?.beats ?? []).map((b: any) => b.what).join(' | '); },
    /** X on the door, its tools page read off the panel, Esc */
    tools(name: string): { id: string; text: string; off: boolean; why: string | null }[] {
      const p = R.door(name).frameCentre; W.cursorAt([p[0], 2.2, p[2]]); input.pressed.add(ORDERS.key); R.step(); R.step(24);   // (the cursor on its lintel: nothing stands between that and the camera)
      const opener = O.menu?.rows.find((r: any) => r.opens); if (!opener) { R.note = `menu ${O.menu ? `${O.menu.target.kind}: ${O.menu.rows.map((r: any) => r.id).join(' ')}` : 'did not come up'}`; input.pressed.add('Escape'); R.step(); return []; }
      input.pressed.add(`Digit${opener.key}`); R.step(); R.step(4);
      const M = O.menu, body = (O.hud().menu?.rows ?? []).slice(1).filter((r: any) => r.kind !== 'cancel');
      const rows = M && M.page === opener.opens ? M.rows.map((d: any, i: number) => ({ id: d.id as string, text: String(body[i]?.opts?.[0]?.text ?? ''), off: !!body[i]?.off, why: (M.why[i] ?? null) as string | null })) : [];
      for (let i = 0; i < 3 && O.menu; i++) { input.pressed.add('Escape'); R.step(); }
      return rows; },
  };
  R.step(30);
  for (const h of game.guards) if (h.team === 'hostile' && h.state !== 'dead') killGuard(game, h, [0, 0, 1], true);   // (nothing but the orders decides what follows)
  R.step(60 * 4);
  return R;
}
/** the face of that door the team's order lands on, and the door hung so that face is / is not the one it opens toward */
const faceTeam = (R: any, name: string, toward: boolean) => { const d = R.door(name), side = R.target(name).side as 1 | -1; hangToward(R.game, d, toward ? side : -side); return { d, side }; };

export const frameholdsTools = guarded(async () => {
  const F = family('frameholds-tools: the squad at a door locked on the face it opens toward');
  const WORDS = 'opens toward us. no boot. charge it.';
  // (a) a cylinder lock: 'open and clear' by a loud squad
  { const R = await rig(7); const { d, side } = faceTeam(R, 'manager', true); const locked = d.locked, held = R.game.doors.bootHolds(d, off(d, side, 0.6));
    const w = R.give('openClear', 'manager'); const op = R.D.ops[0]; R.until(() => R.D.ops.length === 0 || (op?.crossed?.size ?? 0) > 0, 70); R.step(30);
    const picks = R.D.stats.picks as number, crossed = op?.crossed?.size ?? 0, saidPick = R.said.filter((t: string) => /opens toward us\. picking it/.test(t)).length;
    F.verdict(locked && held && w === null && R.kicks === 0 && picks === 1 && !d.lockBroken && !d.locked && saidPick === 1 && crossed > 0,
      "a loud squad told 'open and clear' at a cylinder lock on the face the door opens toward: nobody boots it — the lead says so, picks it, the keep is whole and they go in",
      `locked ${locked} · the frame holds ${held} · refused ${w} · boots ${R.kicks} · picks ${picks} · keep broken ${!!d.lockBroken} · said it ${saidPick} · over the line ${crossed} · ${R.beats(op)}`); }
  // (c) a lock no pick opens thrown while they walk (the order was good when given): it ends at the door, in words, nobody boots
  { const R = await rig(7); const { d } = faceTeam(R, 'manager', true); d.locked = false;
    const w = R.give('openClear', 'manager'); const op = R.D.ops[0]; R.step(20); d.locked = true; (d as any).lockId = `wedge:${d.def.name}`;
    R.until(() => R.D.ops.length === 0 || (op?.crossed?.size ?? 0) > 0, 60); R.step(30);
    const noBoot = R.D.stats.noBoot as number, said = R.said.filter((t: string) => t.includes('opens toward us. no boot')).length;
    F.verdict(w === null && R.kicks === 0 && noBoot === 1 && said === 1 && R.D.ops.length === 0 && /opens toward us/.test(op?.ended ?? '') && !d.lockBroken,
      'locked by something no pick opens while they walked to it: the lead says it opens toward them and the order ends at the door, like steel — no boot',
      `boots ${R.kicks} · counted ${noBoot} · said ${said} · ended: ${op?.ended} · ${R.beats(op)}`); }
  // ((d) 'the same door hung the way it was dealt …: the boot as ever' went 2026-11-07: it asked the squad of before the handle is tried (the rig held that rule
  //  off for it). A locked door found shut ends the first order at the door now, whichever way it is hung.)
  // (e) a charge set on the face it opens toward still opens it: off its hinges
  { const R = await rig(7); const { d } = faceTeam(R, 'manager', true); const w = R.give('charge', 'manager');
    const set = R.until(() => !!R.D.chargeOn(d), 60); R.step(60 * 3); const did = R.D.blow(R.D.chargeOn(d)); R.step(60 * 2);
    F.verdict(w === null && set && !!did && d.lockBroken && d.off, 'a charge set on that face: the leaf comes off', `refused ${w} · set ${set} · blown ${!!did} · off its hinges ${!!d.off} · angle ${f2(d.angle)} · keep broken ${!!d.lockBroken}`); }
  return F.done();
});

// ---------------------------------------------------------------- frameholds-sam
export const frameholdsSam = guarded(async () => {
  const F = family('frameholds-sam: the player at a door locked on the face it opens toward');
  // (2026-11-07: the frame's rule is settled. Gone with that: the third staging with the rule off and every '(off: …)' half. What he is told is read from the
  //  game's own log (heard.ts). 'No noise' was counted through a wrap of the doors' emit on the tools' copy, which the Rust tick never calls: that clause went.)
  const stage = async (toward: boolean) => { reset(); const W = await boot(), g = W.game; g.godMode = true; g.playerInvisible = true; g.tune.civilians = false; g.guards.length = 0;
    const d = g.doors.list.find((x: any) => x.def.name === 'manager'), ks = d.keySide as number; hangToward(g, d, toward ? ks : -ks); fresh(d, true);
    const ears = listen(g), step = () => { W.step(DT); ears.poll(); };
    const at = () => { for (let i = 0; i < 90 && (g.player.kick || g.player.char.anim.kicking); i++) step(); fresh(d, true); g.teleportPlayer(off(d, ks, 0.9, d.def.width * 0.7)); g.player.crouch = false; g.player.fHeld = false; W.step(DT); W.cursorAt(d.pos); W.step(DT); W.step(DT); };
    let yaw = 0; at(); const rows = (g.nearby as any[]).filter(r => r.kind === 'door' && r.door === d), kick = rows.find(r => r.verb === 'kick'), pick = rows.find(r => r.verb === 'pick');
    // a tap on the kick row
    const had = tapVerb(W, 'door', 'kick'); let kicked = !!g.player.kick; for (let i = 0; i < 100; i++) { step(); kicked ||= !!g.player.kick; }
    const tap = { had, kicked, broken: !!d.lockBroken, open: Math.abs(d.angle) };
    // a sprint into it: the camera turned so W runs him square at the leaf, W + Shift held from 1.8 m off (the corridor's furniture allows no more)
    at(); g.teleportPlayer(off(d, ks, 1.8, d.def.width * 0.7)); W.step(DT);
    const c = d.def.closedDir, ux = Math.cos(c), uz = Math.sin(c), out = (): number => Math.abs(-(g.player.char.pos[0] - d.def.hinge[0]) * uz + (g.player.char.pos[2] - d.def.hinge[1]) * ux);   // (metres off the plane of the shut leaf)
    const want = [uz * ks, -ux * ks], far = out(); { let best = -2; for (let i = 0; i < 360; i++) { W.cam.yaw = i * Math.PI / 180; W.cam.rebuild(); const f = W.cam.planarBasis().fwd, dot = f[0] * want[0] + f[2] * want[1]; if (dot > best) { best = dot; yaw = W.cam.yaw; } } }
    let sprintKick = false, near = Infinity, fastest = 0;
    W.input.keys.add('KeyW'); W.input.keys.add('ShiftLeft');
    for (let i = 0; i < 90; i++) { W.cam.yaw = yaw; W.cam.rebuild(); W.step(DT); sprintKick ||= !!g.player.kick; fastest = Math.max(fastest, g.player.char.vel[0] * want[0] + g.player.char.vel[2] * want[1]); near = Math.min(near, out()); }
    W.input.keys.delete('KeyW'); W.input.keys.delete('ShiftLeft'); for (let i = 0; i < 100; i++) W.step(DT);
    const ran = far > 1.2 && near < 0.75 && fastest > 1.5 ? 1 : 0, sprint = `from ${f2(far)} m off the leaf to ${f2(near)} m at up to ${f2(fastest)} m/s into it`;
    // the pick still works from that face
    at(); const picking = tapVerb(W, 'door', 'pick'); let picked = false; for (let i = 0; i < 60 * 7 && !picked; i++) { W.step(DT); picked = !d.locked; }
    ears.poll(); return { d, kick, pick, tap, sprintKick, ran, sprint, picking, picked, said: ears.said, holds: g.doors.bootHolds(d, off(d, ks, 0.9)) };
  };
  const A = await stage(true), P = await stage(false);
  const line = (r: any) => r ? `${r.off ? '(off) ' : ''}${r.line2}` : 'no row';
  F.verdict(!!A.kick && A.kick.off && /it opens toward you, so that won't work/.test(A.kick.line2) && !/sprint/.test(A.kick.line2) && !!A.pick && !A.pick.off,
    'his kick row at that door is greyed and says why; the pick row beside it is live', `${line(A.kick)} | ${line(A.pick)}`);
  F.verdict(A.tap.had && !A.tap.kicked && !A.tap.broken && A.tap.open < 0.01 && A.said.some(t => /a kick won't move it/.test(t)),
    'a tap on the greyed row: a word, no kick thrown, the lock whole', `kick started ${A.tap.kicked} · keep broken ${A.tap.broken} · open ${f2(A.tap.open)} · said: ${A.said.filter(t => /kick|boot/.test(t)).join(' / ') || 'nothing of it'}`);
  F.verdict(A.ran === 1 && !A.sprintKick && !A.d.lockBroken, 'a sprint into that face starts no kick: he stops on it as on any locked door', `ran at it ${A.sprint}, a kick started ${A.sprintKick}`);
  F.verdict(A.picking && A.picked && !A.d.lockBroken, 'and the pick opens it from there as from any face', `pick started ${A.picking} · unlocked ${A.picked}`);
  F.verdict(!P.holds && !!P.kick && !P.kick.off && P.tap.kicked && P.tap.broken && P.ran === 1 && P.sprintKick, 'the same door hung the way it was dealt (locked on the face it opens away from): the kick row live, the tap a kick, the sprint a kick', `${line(P.kick)} · tap kicked ${P.tap.kicked}, keep broken ${P.tap.broken} · sprint kick ${P.sprintKick}`);
  return F.done();
});

// ---------------------------------------------------------------- frameholds-wedge
// The last stand on the compound (laststand-probes.ts's staging: five on eleven, the team left on its start mark, the defenders taken down by hand to three, who
// are ordered into one room and dig in — a last stand wedges ANY door of its room). The moment the room is named, every door of it is hung to open OUT of the
// room (or INTO it: the control), and the three are left to it. Doors stay where they are left (a settled rule), so the three walk in by the room's doors and
// leave them standing open: the probe is the hand that shuts each door of the room behind them (the game's own pull, whenever nobody living is within 2 m of
// the doorway), and the hold test then decides.
async function standRig(seed: number) {
  reset(); Object.assign(LASTSTAND, { regroup: true }); Math.random = seeded(seed);   // (regroup: the last stand's order into one room, an open switch off in the page — raised BEFORE the stand-up, so the game starts with it)
  return quiet(async () => { const W = await standUp({ uploadEvery: 4, level: loadLevel(parseLevelData(readFileSync(resolve(ROOT, 'levels', 'compound.json'), 'utf8'))) }); await startFromQuery(W.game, new URLSearchParams(`?levelFile=compound.json&roomclear=${seed}&squad4`)); return W; });
}
async function standWedges(seed: number, hang: 'out' | 'in' | number) {   // (a number: that entry of the room alone opens out of it, the rest into it)
  const W: any = await standRig(seed); try {
    const g = W.game, step = (secs: number, each?: () => void) => { for (let i = 0; i < Math.round(secs * 60); i++) { g.player.visibility = 0.05; W.step(DT); each?.(); } };
    step(1); const kept: any[] = [];
    for (const R of g.roomClear.rooms) { const stay = ['office', 'bunk', 'back'].includes(R.def.id) ? R.defenders[0] : null; if (stay) kept.push(stay); for (const h of R.defenders.slice()) if (h !== stay && h.state !== 'dead') killGuard(g, h, [1, 0, 0], true); }
    const s0 = { ...DIGIN_STATS }; let room: any = null, hung: string[] = [], setT = -1, n = 0;
    while (n < 150 * 60 && (setT < 0 || n < setT * 60 + 300)) step(1 / 60, () => { n++; const R = lastStandReport(g);
      if (!room && R.room) { const there = kept.find((h: any) => h.state !== 'dead' && g.roomClear.roomAt(h.char.pos) === R.room); room = (g.fight.defs as any[]).find(D => D.gd === there)?.room ?? null;
        if (room) for (const [i, e] of room.entries.entries()) { const inside: [number, number, number] = [e.at[0] - e.out[0] * 0.8, 0, e.at[2] - e.out[1] * 0.8], toward = g.doors.pullFace(e.door, inside), wantOut = typeof hang === 'number' ? i === hang : hang === 'out';
          if (toward === wantOut && Math.abs(e.door.angle) < 0.05) e.door.setSwing(-e.door.swing); hung.push(`${e.door.def.name} ${g.doors.pullFace(e.door, inside) ? 'in' : 'out'}`); } }
      if (room) for (const e of room.entries) if (!e.door.latched && !e.door.off && !g.guards.some((h: any) => h.state !== 'dead' && Math.hypot(h.char.pos[0] - e.at[0], h.char.pos[2] - e.at[2]) < 2)) g.doors.pullTo(e.door, 0);
      if (setT < 0 && R.said.some((l: any) => l.kind.startsWith('set'))) setT = n / 60; });
    const d = (k: string) => (DIGIN_STATS as any)[k] - (s0 as any)[k];
    const on: string[] = room ? (room.entries as any[]).filter(e => e.door.locked && !e.door.def.locked && !e.door.lockBroken).map(e => e.door.def.name as string) : [];   // (a wedge is a lock from the inside: a door locked that the level did not lock)
    return { room: lastStandReport(g).room as string | null, hung, n: room ? room.entries.length as number : 0, on, wedged: room ? on.length : -1, braced: d('braced'), away: d('wedgedAway'), passed: d('opensAway'), setT };
  } finally { reset(); }
}
export const frameholdsWedge = guarded(async () => {
  const F = family('frameholds-wedge: defenders wedge only a door that opens toward them (the last stand on the compound, its room\'s doors hung out of it)');
  // (the deal: one on which the room's men get their furniture to a door, so that the hold test is what decides. FH_SEED=<n> asks another deal)
  const SEED = Number(process.env.FH_SEED ?? 1);
  const line = (r: any) => `room ${r.room} (${r.hung.join(', ')}) · ${r.wedged} doors wedged, ${r.braced} wedges placed, ${r.away} on a door opening away, ${r.passed} doors passed over · all set at ${f2(r.setT)} s`;
  // the mixed room: one door of it opens out, the rest in — the wedges go on the doors a wedge can hold, whichever door they are expected by
  for (const k of sample([1, 0], 1)) { const M = await standWedges(SEED, k), out = M.hung.find((h: string) => h.endsWith('out'))?.split(' ')[0];
    F.verdict(M.n >= 2 && !!out && M.hung.filter((h: string) => h.endsWith('out')).length === 1 && M.wedged === M.n - 1 && !M.on.includes(out) && M.away === 0 && M.passed === 1,
      `one door of the room (${out}) opens out of it, the rest into it: every other door is wedged, that one is passed over`, `${line(M)} · wedged: ${M.on.join(', ')}`); }
  return F.done();
});
