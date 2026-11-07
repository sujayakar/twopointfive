// A MAN THE SQUAD IS SHOOTING AT IS IN THE PICTURE (src/game/fogofwar.ts FOG.squadFire) — the maintainer, 10-11: "i think in fog of war mode we should still
// see enemies when your squad is actually shooting at them. otherwise it feels a bit weird to see them shooting at nothing."
//
//   bun run tools/qa/squadfire-check.ts [level.json] [--verbose]      (probes.ts squadfire; SQUADFIRE_BREAK=1 = the shooter is stood beside the player, dazzled, and never fires: must fail)
//
// Staged on a room clear with the four, the fog on: one teammate stood where he has a line of fire to a defender the player has no line to, the player at his
// start with his back turned, the defenders held where they stand. The room fight does the shooting (its own rounds, its own count of them: Op.rounds / Op.tgt).
//   'only my own eyes'   every such round's man is in the picture on the frame it leaves, never 'seen' (so no laser, nothing for the fog's readers of
//                        seen()), nobody else is shown by it, he is gone again a second after the last round, and a man the rounds drop stays in the
//                        picture as a body
//   hard's cones, standard's all round   the shooter's own eyes lift the fog: the man is SEEN on every round
// The rule is settled on: the check asks the page's side alone and writes no switch. Until 2026-11-07 it also ran each staging with the rule off ('the
// fault is there to fix', 'the same with the rule off', the switch thrown mid-fire, 'rule on or off') and asked the frozen stand's word; and it held two
// later rules off for its staging, the room clear's re-deal (ROOMCLEAR.reroll) and the unfound squad's hold (HOLDFIRE.on), both settled on: the teammate
// is now staged under them, as the page plays him.
import { standUp, ROOT } from './headless.ts';
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';

export async function squadfireCheck(): Promise<boolean> {
  const DT = 1 / 60, argv = process.argv.slice(2), file = argv.find(a => a.endsWith('.json')) ?? 'records.json', verbose = argv.includes('--verbose');
  const BREAK = process.env.SQUADFIRE_BREAK === '1';
  const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
  const { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`);
  const { startFromQuery } = await import(`${ROOT}/src/game/scenarioRun.ts`);
  const { FOG } = await import(`${ROOT}/src/game/fogofwar.ts`);
  const seeded = (a: number) => () => { a |= 0; a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ (a >>> 15), 1 | a); t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t; return ((t ^ (t >>> 14)) >>> 0) / 4294967296; };
  const log = console.log;
  const quiet = async <T>(fn: () => T | Promise<T>): Promise<T> => { const e = console.error, w = console.warn; console.log = console.error = console.warn = () => { }; try { return await fn(); } finally { console.log = log; console.error = e; console.warn = w; } };
  let fails = 0;
  const verdict = (ok: boolean, what: string, detail = '') => { if (!ok) fails++; log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}`); if (detail) log(`    ${detail}`); };

  const lvl = loadLevel(parseLevelData(readFileSync(resolve(ROOT, 'levels', file), 'utf8')));
  const W: any = await quiet(() => standUp({ level: lvl })); const g = W.game; g.godMode = true;
  const sysRandom = Math.random;
  Math.random = seeded(7);   // (every restart below is on the same dice)
  await quiet(() => startFromQuery(g, new URLSearchParams('roomclear&squad4')));
  const FW = g.fogOfWar; FW.on = true;
  const step = (n = 1) => { for (let i = 0; i < n; i++) W.step(DT); };
  await quiet(() => step(30));
  const spawn = [...g.player.char.pos] as number[];
  const nav = g.col.nav ?? g.nav; const walk = (x: number, z: number) => { const cx = Math.floor(x / nav.cell), cz = Math.floor(z / nav.cell); return cx >= 0 && cz >= 0 && cx < nav.W && cz < nav.H && !nav.blocked[cz * nav.W + cx]; };
  const foesOf = () => g.guards.filter((x: any) => x.team !== 'operative') as any[];
  const chestOf = (h: any) => [h.char.pos[0], 1.2, h.char.pos[2]];
  const samLine = (h: any) => !g.col.segmentBlocked([spawn[0], 1.6, spawn[2]], chestOf(h)) || !g.col.segmentBlocked([spawn[0], 1.6, spawn[2]], [h.char.pos[0], 1.6, h.char.pos[2]]);
  /** the scene, found once on the level as it deals: a defender the player has no line to from his start, and floor 3‥6 m from him with a line of fire */
  const scene = (): { name: string; P: number[] } | null => { for (const H of foesOf()) { if (H.state === 'dead' || samLine(H)) continue; const hp = H.char.pos, chest = chestOf(H);
    for (let r = 3; r <= 6; r += 0.5) for (let a = 0; a < 16; a++) { const x = hp[0] + Math.cos(a * Math.PI / 8) * r, z = hp[2] + Math.sin(a * Math.PI / 8) * r;
      if (walk(x, z) && !g.col.shotBlocked([x, 1.4, z], chest, 0.05) && !g.col.segmentBlocked([x, 1.6, z], chest)) return { name: H.callsign, P: [x, z] }; } }
    return null; };
  const SC = scene();
  if (!SC) { verdict(false, `${file}: a defender the player has no line to from his start, with floor a teammate can shoot him from, was found to stage on`); log('\n=== squadfire: 1 FAIL ==='); Math.random = sysRandom; return false; }

  interface Row { f: number; at: string; seen: boolean; drawn: boolean; hidden: boolean }
  interface Out { rounds: Row[]; /** frames a living defender nobody fired at inside the rule's time, and no eye had, was in the picture past the linger */ others: number; byFire: number;
    /** after the shooter was taken away: frames from his last round to the man leaving the picture (-1 = he never was in it, or never left) */ gone: number; lastAt: string;
    /** the dropped man: in the picture three seconds after he went down */ body: boolean | null; bodyOf: string }
  /** one staging: `secs` of the teammate firing, then (away) he is taken off and the picture watched for two seconds more */
  async function run(cone: boolean, own: boolean, o: { secs?: number; mortal?: boolean; away?: boolean } = {}): Promise<Out> {
    const R: Out = { rounds: [], others: 0, byFire: 0, gone: -1, lastAt: '', body: null, bodyOf: '' };
    Math.random = seeded(7); await quiet(() => { g.restartEncounter(); FW.on = true; FOG.cone = cone; FOG.own = own; step(30); });
    const men = g.orders.men('all'), M = men[0], foes = foesOf(), H = foes.find(x => x.callsign === SC!.name); if (!M || !H || samLine(H)) return R;   // (the scene is not the one found: no rounds, and the lines below say so)
    const sam = g.player.char, lastF = new Map<any, number>(), sawF = new Map<any, number>(), opN = new Map<any, number>(); let last = 0, f = 0, downAt = -1, down: any = null;
    const hold = (fire: boolean) => { for (const x of foes) { x.hold = true; x.pinned = true; x.heardUpTo = 1e12; x.god = !o.mortal; }
      sam.pos[0] = spawn[0]; sam.pos[2] = spawn[2]; sam.aimYaw = sam.bodyYaw = Math.atan2(-(H.char.pos[0] - spawn[0]), -(H.char.pos[2] - spawn[2]));
      M.god = true; if (fire) { M.char.pos[0] = SC!.P[0]; M.char.pos[2] = SC!.P[1]; if (f < 5) M.char.aimYaw = M.char.bodyYaw = Math.atan2(H.char.pos[0] - SC!.P[0], H.char.pos[2] - SC!.P[1]); }
      else { M.char.pos[0] = spawn[0] + 0.8; M.char.pos[2] = spawn[2]; M.dazzledUntil = g.time + 5; } };
    const watch = () => { const O = g.fight.opOf(M), n = O?.rounds ?? 0;
      if (O && n > last && O.tgt) { const T = O.tgt; R.rounds.push({ f, at: T.callsign, seen: FW.seen(T), drawn: FW.drawn(T), hidden: FW.hidden(T) }); lastF.set(T, f); R.lastAt = T.callsign; }
      last = n; R.byFire += FW.stats.byFire;
      for (const P of g.fight.ops as any[]) { const k = P.rounds, w = opN.get(P); opN.set(P, k); if (w !== undefined && k > w && P.tgt) lastF.set(P.tgt, f); }   // (any teammate's round counts for 'somebody fired at him', not only the staged shooter's)
      for (const x of foes) { if (x.state === 'dead') { if (!down && lastF.has(x)) { down = x; downAt = f; R.bodyOf = x.callsign; } continue; }
        if (FW.seen(x)) { sawF.set(x, f); continue; }
        if (!FW.hidden(x) && f - (lastF.get(x) ?? -1e9) > FOG.squadFireSecs * 60 + 2 && f - (sawF.get(x) ?? -1e9) > FOG.linger * 60 + 2) R.others++; } };
    await quiet(() => { for (; f < 60 * (o.secs ?? 4); f++) { hold(!BREAK); W.step(DT); watch(); } });
    if (o.away) { const T = foes.find(x => x.callsign === R.lastAt), f0 = lastF.get(T) ?? f; let was = T ? !FW.hidden(T) : false;
      await quiet(() => { for (let k = 0; k < 150; k++, f++) { hold(false); W.step(DT); if (g.fight.opOf(M)?.rounds !== last) { last = g.fight.opOf(M)?.rounds ?? last; continue; }
        if (T && was && FW.hidden(T) && !FW.seen(T)) { R.gone = f - f0; break; } } });
      if (!was) R.gone = -1; }
    if (o.mortal && down) { await quiet(() => { for (; f < downAt + 180; f++) { hold(false); W.step(DT); } }); R.body = !FW.hidden(down) && FW.drawn(down); }
    return R;
  }
  const say = (R: Out) => { const u = R.rounds.filter(r => !r.seen); return `${R.rounds.length} rounds · ${u.length} at a man no eye of the fog's had · of those in the picture ${u.filter(r => r.drawn && !r.hidden).length}, out of it ${u.filter(r => !r.drawn || r.hidden).length}`; };
  log(`${file}: ${g.orders.men('all')[0]?.callsign} stood at (${SC.P[0].toFixed(1)}, ${SC.P[1].toFixed(1)}) with a line of fire to ${SC.name} · the player at his start, back turned, no line to him`);

  // ---- 'only my own eyes'
  { const on = await run(false, true, { away: true });
    const uOn = on.rounds.filter(r => !r.seen);
    if (verbose) for (const r of on.rounds) log(`    f ${r.f} at ${r.at} · seen ${r.seen} drawn ${r.drawn} hidden ${r.hidden}`);
    verdict(uOn.length >= 3 && uOn.every(r => r.drawn && !r.hidden), 'only my own eyes: every round a teammate fires at a man the player\'s eyes lack finds that man in the picture on the frame it leaves', say(on));
    verdict(uOn.length >= 3 && on.rounds.every(r => !r.seen), 'he is drawn, not seen: seen() stays false for him (no laser, nothing for the readers of seen())', `${on.rounds.filter(r => r.seen).length} of ${on.rounds.length} rounds found their man 'seen'`);
    verdict(on.others === 0, 'only the man fired at: no other defender is put in the picture by it', `frames a defender nobody had fired at inside the rule's time was in the picture: ${on.others}`);
    const lo = FOG.squadFireSecs * 60 - 2, hi = (FOG.squadFireSecs + 0.2) * 60;
    verdict(on.gone >= lo && on.gone <= hi, `the rounds stop (the shooter taken away): the man leaves the picture ${FOG.squadFireSecs} s after the last of them`, on.gone < 0 ? 'he was not in the picture, or never left it' : `${on.lastAt} left the picture ${(on.gone / 60).toFixed(2)} s after the last round`);
  }
  // ---- the rounds drop him: the body stays
  { const on = await run(false, true, { mortal: true, secs: 6 });
    verdict(on.body === true, 'a man the squad\'s rounds drop while he is in the picture by this rule stays in it as a body', on.body === null ? 'no defender went down in six seconds of fire' : `${on.bodyOf} · in the picture three seconds after he went down: ${on.body}`); }
  // ---- the rungs where the squad's own eyes lift the fog
  for (const [label, cone] of [['hard\'s cones', true], ['standard\'s all round', false]] as const) {
    const on = await run(cone, false);
    // (under cones the rule can still hold a man a second past the shooter's gaze leaving him — he turned to the next man: counted, not judged)
    verdict(on.rounds.length >= 3 && on.rounds.every(r => r.seen && r.drawn),
      `${label}: the shooter's own eyes have the man he fires at on every round`, `${say(on)} · frames a man was kept in the picture by the rule after the eyes left him ${on.byFire}`); }

  Math.random = sysRandom;
  log(fails ? `\n=== squadfire: ${fails} FAIL ===` : '\n=== squadfire: ALL PASS ===');
  return fails === 0;
}
if (import.meta.main) process.exit((await squadfireCheck()) ? 0 : 1);
