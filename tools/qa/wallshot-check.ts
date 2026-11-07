// A WALL SHOT'S SMOKE IS THINNER AND ITS SHOCK FILLS THE ROOM — the check behind WALLSHOT (src/game/wallcharge.ts; a settled rule since 2026-11-07:
// every tool stands the game with it on. Gone with its off side: the second blast 'with the rule off' beside every line and what was read off it, (3) the
// pall laid against the rule-off pall, and (4) the recording and stand line).
//   bun run tools/qa/probes.ts wallshot        (family `wallcharge`)        WALLSHOT_BREAK=1 = the charge is set and never set off: must FAIL
// THE CHARGE is set by Sam's own hands (charge-rig.ts samSets: one pushed onto the tools' copy of the game by hand is a write the Rust game never gets), the
// men and the player are then stood where the line wants them, and it is set off through the module's own trigger.
// the maintainer, 10-19, of a room clear on the mousehole taken through blown walls: "the smoke after a wall shot makes it too hard -- we should probably (1)
// turn down its density a little and (2) make it so everyone in the room gets more of a shock". In his run the one man in the first room was folded 2.0 s,
// came up still blind inside the pall and shot the first man through the hole 2.7 s after the blast.
// THE SITES: his own level and two of the walls he blew (the long wall of the row of rooms, by the floor points 12.7, 7.0 and 24.9, 7.0, set from the yard),
// each on a fresh stand with its room clear dealt for four. Headless: what the cloud hides is the page's to say, and the pall's density is asked nowhere here
// since its line went (it laid the pall against the rule-off one).
import { readFileSync } from 'node:fs';
import { standUp, ROOT } from './headless.ts';
import { samSets } from './charge-rig.ts';
type Vec3 = [number, number, number];
export async function wallShotCheck(): Promise<boolean> {
  const BRK = process.env.WALLSHOT_BREAK === '1'; let all = true;
  const verdict = (ok: boolean, what: string, detail: string) => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}\n    ${detail}`); };
  console.log(`\n=== wallshot: a wall shot's smoke is thinner and its shock fills the room${BRK ? ' (WALLSHOT_BREAK: the charge is never set off: must fail)' : ''} ===`);
  const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`); const { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`);
  const { DESTRUCTION } = await import(`${ROOT}/src/game/destructionSwitch.ts`);
  const { WALLCHARGE, WALLSHOT, wallAt } = await import(`${ROOT}/src/game/wallcharge.ts`); const { DOORTOOLS } = await import(`${ROOT}/src/game/doortools.ts`);
  const { startFromQuery } = await import(`${ROOT}/src/game/scenarioRun.ts`);
  const { knobsCopy, knobsBack } = await import(`${ROOT}/src/game/knobs.ts`);
  const knobs0 = knobsCopy(), destr0 = DESTRUCTION.on, log = console.log, data = readFileSync(`${ROOT}/levels/mousehole.json`, 'utf8');
  const raise = () => { DESTRUCTION.on = true; WALLCHARGE.on = true; };
  const lerp = (a: number, b: number, t: number) => a + (b - a) * Math.min(1, Math.max(0, t)), near = (a: number, b: number, e = 0.02) => Math.abs(a - b) <= e, f = (v: number) => v.toFixed(2);
  type Man = { who: string; at?: Vec3 };
  /** a charge on the wall by `at`, set from `from` by his own hands, then the named men stood where asked; set off by the module's own trigger. What each man and the player then carry, and
   *  every emitter the blast asked of the solver (`pall`: by the pall's own call). */
  const shot = async (at: Vec3, from: Vec3, men: Man[], sam?: Vec3, opt: { tune?: () => void; watch?: string } = {}) => { raise(); opt.tune?.(); console.log = () => { };
    try { const W: any = await standUp({ level: loadLevel(parseLevelData(data)), seed: 1 }); raise(); const g = W.game; g.godMode = true; g.tune.chaos = 0.5;
      await startFromQuery(g, new URLSearchParams('roomclear&squad4')); g.aiEnabled = false; for (let i = 0; i < 4; i++) W.step(1 / 60);
      const w0 = wallAt(g, at, from); if (!w0 || !w0.ok) return null;
      const stood: Vec3 = [...g.player.char.pos] as Vec3, put = samSets(W, w0), c = put.charge, w = put.wall; if (!c) throw new Error(`Sam could not set the charge: ${put.why}`);
      g.player.char.pos[0] = stood[0]; g.player.char.pos[2] = stood[2];   // (back where the start put him, unless the line stands him somewhere)
      const of = (who: string) => g.guards.find((x: any) => x.callsign === who);
      for (const m of men) { const gd = of(m.who); if (!gd) return null; if (m.at) { gd.char.pos[0] = m.at[0]; gd.char.pos[2] = m.at[2]; } gd.dazzledUntil = 0; gd.staggerUntil = 0; }
      if (sam) { g.player.char.pos[0] = sam[0]; g.player.char.pos[2] = sam[2]; } g.player.dazzledUntil = 0;
      W.step(1 / 60);   // (one step: where everybody was put is the game's before the trigger)
      const res: any = BRK ? null : g.wallCharge.blow(c);
      const row = (who: string) => { const gd = of(who), p = gd.char.pos; return { who, dead: gd.state === 'dead', blind: Math.max(0, gd.dazzledUntil - g.time), folded: Math.max(0, gd.staggerUntil - g.time), room: g.cleared.roomAt(p), dist: Math.hypot(p[0] - w.u, p[2] - at[2]), state: gd.state as string }; };
      const rows = men.map(m => row(m.who)), samT = Math.max(0, g.player.dazzledUntil - g.time); let walked = -1, state = '';
      if (opt.watch) { const gd = of(opt.watch), p0 = [...gd.char.pos], until = gd.staggerUntil - 0.1; g.aiEnabled = true; walked = 0; while (g.time < until) { W.step(1 / 60); walked = Math.max(walked, Math.hypot(gd.char.pos[0] - p0[0], gd.char.pos[2] - p0[2])); } state = gd.state; }
      return { g, hole: !!res?.hole, walked, state, men: rows, sam: samT, big: !!g.orders.big };
    } finally { console.log = log; } };
  const K = DOORTOOLS, S = WALLSHOT, roomM0: number = WALLSHOT.roomM;
  try {
    // (1) his first wall: the one man in the room, where his run had him, a line to the hole
    { const at: Vec3 = [12.7, 0, 7.0], from: Vec3 = [12.7, 0, 6.0], men: Man[] = [{ who: 'Tango 1', at: [13.6, 0, 9.8] }, { who: 'Tango 9', at: [12.7, 0, 8.4] }];
      const on = await shot(at, from, men);
      if (!on) verdict(false, 'the long wall takes a charge where his first was set', 'no wall place or no such man there: the level changed, and this check needs its site again');
      else { const a = on.men[0], t = a.dist / K.stunM, wantOn = lerp(S.stunSecs[0], S.stunSecs[1], t);
        verdict(on.hole && !a.dead && near(a.folded, S.staggerSecs) && near(a.blind, wantOn, 0.1),
          'his first room: the man with a line to the hole is folded and blind by the table\'s own numbers, and no more hurt',
          `${f(a.dist)} m from the charge · folded ${f(a.folded)} s (the table ${S.staggerSecs}), blind ${f(a.blind)} s (${f(wantOn)}) · he is '${a.state}'`);
        const k1 = on.men[1];
        verdict(k1.dist < K.knockM && k1.dead, 'the reach that kills: a man with a line inside it, behind the wall when it goes, is killed',
          `${f(k1.dist)} m from the charge (the reach ${K.knockM} m): dead ${k1.dead}`); } }
    // (1b) his second wall: the room's own furniture stands against its far face (no floor to the room map for a metre in), and one of its men, at his own
    //      post, has no line to the hole
    { const at: Vec3 = [19.2, 0, 7.0], from: Vec3 = [19.2, 0, 6.0], men: Man[] = [{ who: 'Tango 3' }, { who: 'Tango 2' }];
      const on = await shot(at, from, men, undefined, { watch: 'Tango 3' });
      if (!on) verdict(false, 'the long wall takes a charge where his second was set', 'no wall place or no such man there: the level changed, and this check needs its site again');
      else { const [c1, l1] = on.men;
        verdict(on.hole && c1.room > 0 && c1.room === l1.room && near(c1.folded, S.staggerSecs) && near(c1.blind, 0) && near(l1.folded, S.staggerSecs) && on.walked >= 0 && on.walked < 0.05 && on.state !== 'patrol',
          'his second room, furniture against the wall\'s far face: the room is still named, and its man with no line to the hole is folded — and with the game running on he stands rooted through it, alerted',
          `the man with no line ${f(c1.dist)} m off (room ${c1.room}): folded ${f(c1.folded)} s, blind ${f(c1.blind)} s · the room's lined man: folded ${f(l1.folded)} s · the game run on to the end of his fold: he moved ${f(on.walked)} m, '${on.state}'`); } }
    // (2) his third wall: a room with a corner the hole cannot see. A man there, a man with a line, a man in the next room, one of ours and the player beyond the wall
    { const at: Vec3 = [24.9, 0, 7.0], from: Vec3 = [24.9, 0, 6.0];
      const men: Man[] = [{ who: 'Tango 5', at: [26.2, 0, 9.8] }, { who: 'Tango 6', at: [22.25, 0, 10.75] }, { who: 'Tango 2', at: [20.3, 0, 9.8] }, { who: 'Blue Two', at: [25.6, 0, 9.4] }], sam: Vec3 = [24.4, 0, 9.6];
      // (the corner: found on the room as the blast leaves it — a floor cell of the lined man's room that no point of the hole's mouth sees, the nearest such)
      const scout = await shot(at, from, men.slice(0, 1)); let corner: Vec3 | null = null;
      if (scout?.hole) { const g = scout.g, nav = g.col.nav, room = scout.men[0].room, mouth: Vec3[] = [-1.2, -0.8, -0.4, 0, 0.4, 0.8, 1.2].flatMap(du => [0.45, 0.9].map(d => [at[0] + du, 1.1, 7.12 + d] as Vec3)); let best = 1e9;
        for (let z = 0; z < nav.H; z++) for (let x = 0; x < nav.W; x++) { const p: Vec3 = [(x + 0.5) * nav.cell, 0, (z + 0.5) * nav.cell], d = Math.hypot(p[0] - at[0], p[2] - at[2]);
          if (!room || g.cleared.roomAt(p) !== room || d < 2.6 || d >= best || !mouth.every(m => g.col.solidBlocked(m, [p[0], 1.1, p[2]]))) continue; best = d; corner = p; } }
      if (corner) men[1].at = corner;
      const on = corner ? await shot(at, from, men, sam) : null;
      if (!on) verdict(false, 'the long wall takes a charge where his third was set, and its room has a corner the hole cannot see', `no wall place, no such man or no such corner there (a corner: ${!!corner}): the level changed, and this check needs its site again`);
      else { const [l1, c1, n1, o1] = on.men;
        verdict(on.hole && c1.room === l1.room && c1.room > 0 && near(c1.folded, S.staggerSecs) && near(c1.blind, 0) && !c1.dead,
          'everyone in the room: a defender in the room with NO line to the hole is folded the table\'s seconds — and not blinded (the flash needs the line), not hurt',
          `the corner man ${f(c1.dist)} m off, in room ${c1.room} with the lined man (room ${l1.room}) · folded ${f(c1.folded)} s (the table ${S.staggerSecs}), blind ${f(c1.blind)} s · he is '${c1.state}'`);
        verdict(near(l1.folded, S.staggerSecs) && n1.room !== l1.room && near(n1.folded, 0) && near(n1.blind, 0),
          'the room and no further: the man with a line is folded the table\'s seconds, and a man in the NEXT room is untouched',
          `the lined man ${f(l1.dist)} m off: folded ${f(l1.folded)} s, blind ${f(l1.blind)} s · the next room's man (room ${n1.room}, ${f(n1.dist)} m off): folded ${f(n1.folded)} s, blind ${f(n1.blind)} s`);
        verdict(on.big && near(o1.blind, S.oursSecs) && near(o1.folded, 0) && near(on.sam, S.oursSecs),
          'ours and the player the same: one of ours beyond the wall with a line is blind the same seconds as the player standing there, both by the table, and neither is folded',
          `one of ours ${f(o1.dist)} m off: blind ${f(o1.blind)} s (the table ${S.oursSecs}), folded ${f(o1.folded)} s · the player: blind ${f(on.sam)} s`);
        const short = await shot(at, from, men, sam, { tune: () => { WALLSHOT.roomM = Math.max(1, c1.dist - 1); } }), cs = short?.men[1]; WALLSHOT.roomM = roomM0;
        verdict(!!cs && c1.dist < roomM0 && near(cs.folded, 0) && near(short!.men[0].folded, S.staggerSecs), 'the room\'s shock has a reach: with the table\'s reach drawn in short of the corner man he is untouched again, the lined man folded as before',
          `the reach ${roomM0} m, the corner man ${f(c1.dist)} m off: folded ${f(c1.folded)} s · the reach set to ${f(Math.max(1, c1.dist - 1))} m: folded ${f(cs?.folded ?? -1)} s, the lined man ${f(short?.men[0].folded ?? -1)} s`);
      } }
  } finally { knobsBack(knobs0); DESTRUCTION.on = destr0; console.log = log; }
  console.log(`=== wallshot: ${all ? 'ALL PASS' : 'FAILURES above'} ===`);
  return all;
}
