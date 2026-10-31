// THE BREAK LAB'S BURSTS ARE THE GAME'S OWN (src/host/breaklab.ts labBurst): the same function the lab's click calls, on a level stood up headless.
//   bun run tools/qa/breaklab-check.ts [level.json …]          (default: gwot-yard.json, the lab's own yard, and pass.json)
//   TPS_PROFILE=shipped in front = the stand the page opens on (an outer wall takes a charge only there: WALLCHARGE.outer)
//   BREAKLAB_BREAK=1 = every charge clicked a metre and a half off its wall or door: the lines that ask for a hole and a leaf off must fail
// Lines, a level: a wall charge clicked on a partition holes it · on an outer wall holes it (the page's stand; on the frozen stand it must say why not) ·
// a door charge clicked at a door takes the leaf off (the frozen stand, where a charge tears no leaf off: throws it open on its hinges) · a grenade is frag.ts's blast (its report is in the frag log) and breaks what stands by it · a charge
// clicked in the open opens no wall and says so. Logic only: the flash, the pall and the sound are made by the same calls and are not looked at here.
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
const ROOT = resolve(import.meta.dir, '../..'), DT = 1 / 60, BREAK = process.env.BREAKLAB_BREAK === '1';
async function quiet<T>(fn: () => Promise<T> | T): Promise<T> { const l = console.log, w = console.warn, e = console.error; console.log = console.warn = console.error = () => {}; try { return await fn(); } finally { console.log = l; console.warn = w; console.error = e; } }

const { standUp } = await import('./headless.ts');
const { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`);
const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
const { DESTRUCTION, FURNBLAST } = await import(`${ROOT}/src/game/destructionSwitch.ts`);
const { THINGS } = await import(`${ROOT}/src/game/things.ts`);
const { FURN_STATS } = await import(`${ROOT}/src/game/furnblast.ts`);
const { FRAG_LOG } = await import(`${ROOT}/src/game/frag.ts`);
const { WALLCHARGE, wallAt } = await import(`${ROOT}/src/game/wallcharge.ts`);
const { labBurst, BREAKLAB } = await import(`${ROOT}/src/host/breaklab.ts`);
const SHIPPED = process.env.TPS_PROFILE === 'shipped';

async function stage(file: string) {
  DESTRUCTION.on = true; FURNBLAST.on = true;
  const W: any = await quiet(() => standUp({ uploadEvery: 1, level: loadLevel(parseLevelData(readFileSync(resolve(ROOT, 'levels', file), 'utf8')), { dressing: true, lived: true }), seed: 1 }));
  THINGS.on = true; THINGS.rings.on = true;
  const g = W.game; g.tune.chaos = 0.5; g.godMode = true; g.aiEnabled = false; g.playerInvisible = true;
  for (let f = 0; f < 5; f++) W.step(DT);
  return { W, g };
}
let ok = true; const line = (pass: boolean, what: string, detail = '') => { console.log(`- ${pass ? 'PASS' : 'FAIL'} — ${what}${detail ? `\n    ${detail}` : ''}`); if (!pass) ok = false; };
const one = (s: string) => s.split('\n').join(' · ');
const was = [THINGS.on, THINGS.rings.on, DESTRUCTION.on, FURNBLAST.on];
try {
  for (const file of process.argv.slice(2).length ? process.argv.slice(2) : ['gwot-yard.json', 'pass.json']) {
    console.log(`\n=== breaklab: the lab's bursts are the game's own (${file}, ${SHIPPED ? "the page's stand" : 'the frozen stand'}) ===`);
    // (the walls a charge takes, as the game's own pick names them from where the player stands: the first partition and the first outer wall)
    const find = async () => { const S = await stage(file), g = S.g; let part: any = null, outer: any = null, refused: any = null;
      for (let i = 0; i < g.col.boxes.length && !(part && outer); i++) { const b = g.col.boxes[i], w = wallAt(g, [b.c[0], 0, b.c[2]]); if (!w || w.index !== i) continue;
        if (!w.ok) { refused ??= w; continue; } if (w.outer) outer ??= w; else part ??= w; }
      return { S, g, part, outer, refused }; };
    const off = (w: any, d: number): number[] => { const b = w.box; return w.alongX ? [w.u, 0, b.c[2] + w.side * (b.h[2] + d)] : [b.c[0] + w.side * (b.h[0] + d), 0, w.u]; };
    const D = BREAK ? 1.5 : 0.3;   // (a click on the floor at the wall's foot, inside the lab's reach for a wall — or, broken, well outside it)
    for (const kind of ['part', 'outer'] as const) {
      const F = await find(), w = F[kind], name = kind === 'part' ? 'a partition' : 'an outer wall';
      if (!w) { if (kind === 'outer' && !SHIPPED) { const r = F.refused; const said = r ? labBurst(F.g, 'wall charge', off(r, 0.3)) : ''; line(!r || /not set/.test(said), `a wall charge clicked on a wall the frozen stand refuses is not set, and the lab says why`, r ? one(said) : 'no such wall on this level'); }
        else console.log(`- (no ${name} a charge takes on this level)`); continue; }
      const blown0 = F.g.wallCharge.stats.blown, holes0 = F.g.wallCharge.stats.holes, said = labBurst(F.g, 'wall charge', off(w, D)); for (let f = 0; f < 30; f++) F.S.W.step(DT);
      const st = F.g.wallCharge.stats, B = F.g.destr.walls.breakableAt(w.index), open = B ? B.open : 0;   // (THIS wall's own cells: a click that found some other wall is not this line's)
      line(st.blown === blown0 + 1 && st.holes === holes0 + 1 && open > 0 && /is holed/.test(said) && F.g.wallCharge.set.length === 0, `a wall charge clicked at the foot of ${name} is set on it and blown by the wall charge's own trigger, and the wall is holed`, `wall box ${w.index}, cells of it open ${open} · the trigger's count ${st.blown - blown0}, holes ${st.holes - holes0}, charges left set ${F.g.wallCharge.set.length} · ${one(said)}`);
    }
    { const S = await stage(file), g = S.g, d = g.doors.list.find((x: any) => !x.off && !x.def.steel);
      if (!d) console.log('- (no wooden door on this level)');
      else { const at = [d.frameCentre[0] + (BREAK ? 1.5 : 0.2), 0, d.frameCentre[2] + (BREAK ? 1.5 : 0.2)], said = labBurst(g, 'door charge', at); for (let f = 0; f < 30; f++) S.W.step(DT);
        const moved = Math.abs(d.angle) > 0.05 || Math.abs(d.vel) > 0.05;
        line((SHIPPED ? d.off === true && /leaf is off/.test(said) : (d.off || moved) && /set on the door/.test(said)) && g.doorTools.set.length === 0, `a door charge clicked at a door is set on it and blown by the door charge's own trigger, and the leaf ${SHIPPED ? 'comes off' : 'is thrown open (this stand tears none off)'}`, `door '${d.def.name}' off ${d.off}, angle ${d.angle.toFixed(2)} · charges left set ${g.doorTools.set.length} · ${one(said)}`); } }
    { const S = await stage(file), g = S.g, T = g.destr.things.all().find((t: any) => t.via === 'furniture' && t.hold === 'stands' && t.mat === 'wood' && !t.ghost);
      if (!T) console.log('- (no wooden furniture stands on this level)');
      else { const at = [(T.a.x0 + T.a.x1) / 2, 0, T.a.z1 + 0.3], n0 = FRAG_LOG.length, b0 = FURN_STATS.statics + FURN_STATS.razed, ev0 = g.events.length, said = labBurst(g, 'grenade', at); const ev = g.events.slice(ev0).map((e: any) => e.kind);
        for (let f = 0; f < 30; f++) S.W.step(DT);
        line(FRAG_LOG.length === n0 + 1 && FURN_STATS.statics + FURN_STATS.razed > b0, `a grenade is the hand grenade's own blast (its report is in the frag log) and breaks the thing it lies by`, `frag reports ${FRAG_LOG.length - n0} · furniture boxes broken ${FURN_STATS.statics + FURN_STATS.razed - b0} · events it raised: ${[...new Set(ev)].join(', ') || 'none'} · ${one(said)}`); } }
    { const S = await stage(file), g = S.g; let at: number[] | null = null;   // (open floor: no wall the lab would pick within two metres)
      const p = g.player.char.pos; for (let r = 0; r < 12 && !at; r++) for (let k = 0; k < 8 && !at; k++) { const q = [p[0] + Math.cos(k * 0.785) * r, 0, p[2] + Math.sin(k * 0.785) * r]; const pm = WALLCHARGE.pickM; WALLCHARGE.pickM = 2; const w = wallAt(g, q); WALLCHARGE.pickM = pm; if (!w) at = q; }
      if (!at) console.log('- (no open floor found near the start)');
      else { const b0 = g.wallCharge.stats.blown, ev0 = g.events.length, said = labBurst(g, 'wall charge', at), bang = g.events.slice(ev0).some((e: any) => e.kind === 'bang' && e.charge);
        line(g.wallCharge.stats.blown === b0 && /no wall under the cursor/.test(said) && bang, `a wall charge clicked in the open is its blast alone with the charge's borrowed report, and the lab says to click on a wall`, `the trigger's count ${g.wallCharge.stats.blown - b0} · a charge's bang raised ${bang} · ${one(said)}`); } }
    console.log(`  (the lab's reach for a wall ${BREAKLAB.wallM} m, for a door ${BREAKLAB.doorM} m)`);
  }
} finally { [THINGS.on, THINGS.rings.on, DESTRUCTION.on, FURNBLAST.on] = was; }
console.log(`\n=== breaklab-check: ${ok ? 'ALL PASS' : 'FAILURES above'} ===`);
process.exit(ok ? 0 : 1);
