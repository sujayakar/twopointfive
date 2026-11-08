// A WALL CHARGE KILLS WHOEVER STANDS AT IT — the check behind WALLKILL (src/game/wallcharge.ts; a settled rule since 2026-11-07: every tool stands the
// game with it on. Gone with its off side: the second blast 'with the rule off' beside every line and what was read off it, and (6) the recording and stand line).
//   bun run tools/qa/probes.ts wallkill        (family `wallcharge`)        WALLKILL_BREAK=1 = the charge is set and never set off: must FAIL
// THE CHARGE is set by Sam's own hands (charge-rig.ts samSets: one pushed onto the tools' copy of the game by hand is a write the Rust game never gets), the
// men and the player are then stood where the line wants them, and it is set off through the module's own trigger.
// the maintainer, 10-19, god mode off, 0.9 m from a charge he had set on a wall when he blew it — the log said 'you were too close' and he walked on:
// "blowing a wall charge with me right next to it should kill me". And his standing rule: a blast does to the player what it does to anyone.
// THE SITE: the mousehole's long wall by the floor point 12.7, 7.0, the charge set from the yard (the wall shot check's first site), each blast on a fresh
// stand with its room clear dealt. Headless: who is down and who is blind is the game's own state; the body thrown and the frame are the page's to show.
import { behindWall } from './squadwords';
import { readFileSync } from 'node:fs';
import { standUp, ROOT } from './headless.ts';
import { samSets } from './charge-rig.ts';
type Vec3 = [number, number, number];
export async function wallKillCheck(): Promise<boolean> {
  const BRK = process.env.WALLKILL_BREAK === '1'; let all = true;
  const verdict = (ok: boolean, what: string, detail: string) => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}\n    ${detail}`); };
  console.log(`\n=== wallkill: a wall charge kills whoever stands at it${BRK ? ' (WALLKILL_BREAK: the charge is never set off: must fail)' : ''} ===`);
  const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`); const { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`);
  const { DESTRUCTION } = await import(`${ROOT}/src/game/destructionSwitch.ts`);
  const { WALLCHARGE, WALLSHOT, WALLKILL, wallAt, wallPt, safeRadius } = await import(`${ROOT}/src/game/wallcharge.ts`); const { DOORTOOLS } = await import(`${ROOT}/src/game/doortools.ts`);
  const { startFromQuery } = await import(`${ROOT}/src/game/scenarioRun.ts`);
  const { knobsCopy, knobsBack } = await import(`${ROOT}/src/game/knobs.ts`);
  const knobs0 = knobsCopy(), destr0 = DESTRUCTION.on, log = console.log, data = readFileSync(`${ROOT}/levels/mousehole.json`, 'utf8');
  const raise = () => { DESTRUCTION.on = true; WALLCHARGE.on = true; };
  const f = (v: number) => v.toFixed(2), near = (a: number, b: number, e = 0.05) => Math.abs(a - b) <= e, lerp = (a: number, b: number, t: number) => a + (b - a) * Math.min(1, Math.max(0, t));
  const AT: Vec3 = [12.7, 0, 7.0], FROM: Vec3 = [12.7, 0, 6.0];
  type Man = { who: string; at: Vec3 };
  /** a charge on the wall (set by his own hands), then the named men and the player stood where asked; set off by the module's own trigger. Who is down, who is blind, and what the trigger's row
   *  said of them before it went. */
  const shot = async (men: Man[], sam: Vec3, opt: { god?: boolean; team?: string; held?: boolean; plan?: boolean } = {}) => { raise(); console.log = () => { };
    try { const W: any = await standUp({ level: loadLevel(parseLevelData(data)), seed: 1 }); raise(); const g = W.game; g.tune.chaos = 0.5;
      await startFromQuery(g, new URLSearchParams(opt.team ?? 'roomclear&squad4')); g.aiEnabled = false; g.godMode = true; for (let i = 0; i < 4; i++) W.step(1 / 60);
      const w0 = wallAt(g, AT, FROM); if (!w0 || !w0.ok) return null;
      const put = samSets(W, w0), c = put.charge, w = put.wall; if (!c) throw new Error(`Sam could not set the charge: ${put.why}`);   // (god mode while he sets it; as the line asks from here on)
      g.godMode = !!opt.god; if (opt.held) g.tune.chaos = 0;   // (chaos 0 from here: the engine opens no wall — the charge goes off and the wall holds. Not before: at chaos 0 no wall offers a charge)
      const of = (who: string) => g.guards.find((x: any) => x.callsign === who), pl = g.player;
      // (everybody the line does not name stands off the map: nobody wanders into the blast)
      g.guards.forEach((gd: any, i: number) => { if (!men.some(m => m.who === gd.callsign)) { gd.char.pos[0] += 300 + i; } });
      for (const m of men) { const gd = of(m.who); if (!gd) return null; gd.char.pos[0] = m.at[0]; gd.char.pos[2] = m.at[2]; gd.dazzledUntil = 0; gd.staggerUntil = 0; }
      pl.char.pos[0] = sam[0]; pl.char.pos[2] = sam[2]; pl.dazzledUntil = 0; pl.fireCd = 0;
      const c0w = wallPt(w, 0, 0), d0 = new Map(men.map(m => [m.who, Math.hypot(m.at[0] - c0w[0], m.at[2] - c0w[2])])), dS = Math.hypot(sam[0] - c0w[0], sam[2] - c0w[2]);   // (from the charge's own place on the wall, as the blast measures — and where each stood as it went: a body is thrown)
      W.step(1 / 60);   // (one step: where everybody was put is the game's before the trigger)
      const hits0 = pl.hitsLeft, warned: string[] = [...g.wallCharge.tooClose(c), ...behindWall(g)];
      const res: any = BRK ? null : opt.plan ? g.wallCharge.blow(c, false) : g.wallCharge.blow(c), R = safeRadius(g, w), last = { ...g.wallCharge.lastBlow };
      const row = (who: string) => { const gd = of(who); return { who, team: gd.team as string, dead: gd.state === 'dead', kind: gd.downKind as string | null, blind: Math.max(0, gd.dazzledUntil - g.time), folded: Math.max(0, gd.staggerUntil - g.time), dist: d0.get(who)! }; };
      const out = { g, hole: !!res?.hole, res, reach: R.reach as number, men: men.map(m => row(m.who)), big: !!g.orders.big, warned, said: g.messages.slice(-2).map((m: any) => m.text as string).join(' / '),
        sam: { down: !!pl.down, hits: pl.hitsLeft as number, hits0, blind: Math.max(0, pl.dazzledUntil - g.time), cd: pl.fireCd as number, dist: dS, last }, later: true };
      for (let i = 0; i < 45; i++) W.step(1 / 60);   // (the game runs on over whoever it put down: nothing throws, and three quarters of a second on the down are down)
      out.later = out.sam.down === !!pl.down && out.men.every(m => m.dead === (of(m.who).state === 'dead'));
      return out;
    } finally { console.log = log; } };
  const K = DOORTOOLS, nearM: number = WALLKILL.nearM, CLEAR: Vec3 = [5.0, 0, 3.0];
  const nearSecs = (dist: number, reach: number) => lerp(WALLCHARGE.safeSecs[0], WALLCHARGE.safeSecs[1], dist / reach);
  try {
    // (1) his run: the player 0.9 m from the charge on its own side, god mode off
    { const sam: Vec3 = [12.7 + 0.37, 0, 7.0 - 0.9];
      const on = await shot([], sam), god = await shot([], sam, { god: true });
      if (!on || !god) verdict(false, 'the long wall takes a charge where the wall shot check sets its first', 'no wall place there: the level changed, and this check needs its site again');
      else {
        verdict(on.sam.dist < nearM && on.sam.down && on.sam.hits === 0 && on.sam.last.samDown === true && on.later && !/too close/.test(on.said) && on.warned.includes('you') && on.hole,
          'his run: the player right at his own charge when it goes is killed; the trigger\'s row named him before it went',
          `${f(on.sam.dist)} m from the charge (the reach that kills on its own side ${nearM} m) · down ${on.sam.down} · warned: ${on.warned.join(', ') || 'nobody'} · the log: '${on.said}' · still down three quarters of a second on ${on.later}`);
        verdict(!god.sam.down && god.sam.hits === god.sam.hits0 && god.sam.cd > 0, 'god mode keeps him up: he is rocked',
          `down ${god.sam.down} · no round for ${f(god.sam.cd)} s · blind ${f(god.sam.blind)} s`); } }
    // (2) a step and a half back he lives, and gets what he got
    { const sam: Vec3 = [12.7, 0, 7.0 - (nearM + 0.5)]; const on = await shot([], sam);
      if (!on) verdict(false, 'the same wall, the player further back', 'no wall place there');
      else verdict(on.sam.dist >= nearM && on.sam.dist < on.reach && !on.sam.down && near(on.sam.blind, nearSecs(on.sam.dist, on.reach), 0.1),
        'past that reach and inside the safety radius the player lives: blind the seconds the radius gives a man there',
        `${f(on.sam.dist)} m from the charge (kills inside ${nearM} m, the radius ${on.reach} m) · down ${on.sam.down} · blind ${f(on.sam.blind)} s (the radius's ${f(nearSecs(on.sam.dist, on.reach))} s) · no round ${f(on.sam.cd)} s`); }
    // (3) what it does to the player it does to anyone: one of ours and a defender at the charge, one of ours a step and a half back
    { const men: Man[] = [{ who: 'Red One', at: [12.7 - 0.5, 0, 7.0 - 0.8] }, { who: 'Tango 1', at: [12.7 + 0.6, 0, 7.0 - 0.9] }, { who: 'Red Two', at: [12.7, 0, 7.0 - (nearM + 0.6)] }];
      const on = await shot(men, CLEAR);
      if (!on) verdict(false, 'the same wall, three men staged on the charge\'s side', 'no wall place or no such men: the level changed');
      else { const [a, b, c] = on.men;
        verdict(a.team === 'operative' && b.team !== 'operative' && a.dist < nearM && b.dist < nearM && a.dead && b.dead && a.kind === 'shot' && b.kind === 'shot' && on.later && on.res?.lost === 1 && on.sam.last.lost === 1 && on.warned.includes('Red One'),
          'anyone: one of ours and a defender standing at the charge are killed, as the player is — dead, not out cold; the row named ours before it went',
          `${a.who} ${f(a.dist)} m: dead ${a.dead} (went down '${a.kind}') · ${b.who} ${f(b.dist)} m: dead ${b.dead} ('${b.kind}') · ours it killed, by the blast's own count: ${on.res?.lost} · warned: ${on.warned.join(', ')}`);
        verdict(c.dist >= nearM && !c.dead && c.blind > 0 && !on.sam.down && on.sam.blind === 0,
          'one of ours a step and a half back is blind and no worse, and the player clear of the radius feels nothing',
          `${c.who} ${f(c.dist)} m: dead ${c.dead}, blind ${f(c.blind)} s · the player ${f(on.sam.dist)} m off: down ${on.sam.down}, blind ${f(on.sam.blind)} s`); } }
    // (4) behind the wall it opens: the player and one of ours inside the reach that always killed a defender there; one of ours past it
    { const men: Man[] = [{ who: 'Red One', at: [12.7 + 0.5, 0, 8.3] }, { who: 'Tango 1', at: [12.7 - 0.6, 0, 8.4] }, { who: 'Red Two', at: [13.6, 0, 9.8] }], sam: Vec3 = [12.7, 0, 8.4];
      const on = await shot(men, sam);
      if (!on) verdict(false, 'the same wall, men staged in the room behind it', 'no wall place or no such men: the level changed');
      else { const [a, b, c] = on.men;
        verdict(on.hole && on.big && a.dist < K.knockM && b.dist < K.knockM && on.sam.dist < K.knockM && on.sam.down && a.dead && b.dead && a.kind === b.kind && on.later && on.warned.includes('you') && on.warned.includes('Red One'),
          'behind the wall, inside the reach that puts a defender there down: the player and one of ours go down with him, ours by the same knock as his; the row named both before it went',
          `the reach ${K.knockM} m · the player ${f(on.sam.dist)} m: down ${on.sam.down} · ${a.who} ${f(a.dist)} m: down ${a.dead} ('${a.kind}') · ${b.who} ${f(b.dist)} m: down ${b.dead} ('${b.kind}') · warned: ${on.warned.join(', ')}`);
        verdict(c.dist >= K.knockM && !c.dead && near(c.blind, WALLSHOT.oursSecs) && c.folded === 0,
          'past that reach one of ours behind the wall is blind as one of ours and not folded (a team of four)',
          `${c.who} ${f(c.dist)} m: blind ${f(c.blind)} s (the table ${WALLSHOT.oursSecs}), folded ${f(c.folded)} s`); } }
    // (4b) a wall that held is still a wall: the man behind it lives, the man at the charge does not
    { const men: Man[] = [{ who: 'Red One', at: [12.7 - 0.5, 0, 7.0 - 0.8] }, { who: 'Red Two', at: [12.7 + 0.5, 0, 8.3] }], sam: Vec3 = [12.7, 0, 8.4]; const on = await shot(men, sam, { held: true });
       }
    // (4c) a plan's own clock is no press of the player's: its blast is the blast as it was
    { const men: Man[] = [{ who: 'Red One', at: [12.7 - 0.5, 0, 7.0 - 0.8] }], sam: Vec3 = [12.7 + 0.37, 0, 7.0 - 0.9]; const on = await shot(men, sam, { plan: true }), prs = await shot(men, sam);
      if (!on || !prs) verdict(false, 'the same wall, blown as a plan\'s clock blows it', 'no wall place or no such men');
      else verdict(!on.sam.down && !on.men[0].dead && on.men[0].blind > 0 && on.sam.cd > 0 && prs.sam.down && prs.men[0].dead, 'blown by a plan\'s own clock (no press of the player\'s) the charge does what it did: the player and the man at it are rocked and blind, up; the same two under his own press are killed',
        `the plan's clock: the player down ${on.sam.down}, ${on.men[0].who} dead ${on.men[0].dead} (blind ${f(on.men[0].blind)} s) · his press: down ${prs.sam.down}, dead ${prs.men[0].dead}`); }
    // (5) a team of two: its one man behind the wall, past the reach that kills, is one of ours too
    { const team = 'roomclear', probe = await shot([], CLEAR, { team }), mate = probe?.g.guards.find((x: any) => x.team === 'operative')?.callsign as string | undefined;
      const men: Man[] = mate ? [{ who: mate, at: [13.6, 0, 9.8] }] : [], on = mate ? await shot(men, CLEAR, { team }) : null;
      if (!on) verdict(false, 'the room clear deals a team of two on this level', `no teammate found (${mate ?? 'none'})`);
      else { const a = on.men[0];
        verdict(!on.big && a.dist >= K.knockM && !a.dead && near(a.blind, WALLSHOT.oursSecs) && a.folded === 0,
          'a team of two: its man behind the wall past the reach that kills is blind as one of ours and not folded',
          `${a.who} ${f(a.dist)} m, a team of more than two ${on.big}: blind ${f(a.blind)} s (ours ${WALLSHOT.oursSecs}), folded ${f(a.folded)} s`); } }
    // (7) the squad's own charge: the setter's word waits until the charge he set cannot kill him
    { const { rig } = (await import('./outerwall-probes.ts')) as any, { WALL_ORDER, wallTarget } = await import(`${ROOT}/src/game/wallcharge.ts`);
      const flat = (a: Vec3, b: Vec3) => Math.hypot(a[0] - b[0], a[2] - b[2]);
      const run = async () => { console.log = () => { }; try { const R = await rig('mousehole-four', 1), g = R.game, C = R.C, O = g.orders; raise(); C.setCarried(5);
          let op: any = null; for (const x of [12, 12.6, 11.4, 13.2, 10.8, 13.8, 10.2]) { const p: Vec3 = [x, 0, 7.0]; O.give(WALL_ORDER, wallTarget(g, p, { kind: 'floor', at: O.freeSpot(p) ?? p }), 'all'); op = C.ops[0]; if (op) break; }
          if (!op) return null; const lead = op.lead, at = () => flat(lead.char.pos, wallPt(op.wall, 0, 0)); let saidT = NaN, saidM = NaN; const say0 = g.say.bind(g);
          g.say = (gd: any, text: string, ...a: any[]) => { if (text === 'charge set.' && isNaN(saidT)) { saidT = g.time; saidM = at(); } return say0(gd, text, ...a); };
          if (!R.until(() => C.set.length === 1, 60)) return null; const setT = g.time, setM = at();
          R.until(() => !isNaN(saidT) || op.phase === 'armed', 30); const res = !isNaN(saidT) ? C.blow(C.nearest()) : null; R.step();
          return { setM, saidM, wait: saidT - setT, dead: lead.state === 'dead', lost: res?.lost ?? -1, who: lead.callsign as string };
        } finally { console.log = log; } };
      const on = await run();
      if (!on) verdict(false, 'the ward\'s four set a charge on the wall when told', 'the order was not taken or the charge never went on');
      else verdict(on.setM < nearM && on.saidM >= nearM && on.wait > 0.2 && !on.dead && on.lost === 0,
        'the man the squad sends to set it says "charge set." once he is past the reach that kills, and a press on his word loses nobody',
        `${on.who}: ${f(on.setM)} m from the charge as it goes on; his word ${f(on.wait)} s later at ${f(on.saidM)} m (kills inside ${nearM}), blown on the word: dead ${on.dead}, of ours lost ${on.lost}`); }
  } finally { knobsBack(knobs0); DESTRUCTION.on = destr0; console.log = log; }
  console.log(`=== wallkill: ${all ? 'ALL PASS' : 'FAILURES above'} ===`);
  return all;
}
