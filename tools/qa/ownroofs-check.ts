// A LEVEL'S OWN STORM RAINS BY THAT LEVEL'S OWN ROOFS (src/game/weather.ts syncRoofs, weatherKinds.ts WX_OWN.roofs).
//
// The maintainer on the records office at night, 10-11: "rain is busted on records office". With no weather picked, the night storm was cut by the OFFICE's two
// rectangles (INTERIOR 4‥36 × 4‥24, the stair tower) on every level: the records office's whole car park lies inside the first, so no rain was drawn on it, nobody
// standing there got wet, no storm wind blew there — and the rain showed only on the strip of ground outside the office's footprint. The rule now: a level whose
// roofs are not the office's two has its storm cut by its own ceiling slabs, as a picked weather's always was.
//
//   bun run tools/qa/ownroofs-check.ts [--verbose]      exit 0 = all pass, 1 = a FAIL line
//   OWNROOFS_BREAK=1 …   = the open ground is read against the office's two rectangles in place of the level's own roofs (the fault as he met it): must fail
// 2026-11-07: the rule is settled (on), and a write of its other side is refused. Gone with that: the second rig of every level with the rule off and what
// was laid beside it ('large without it', 'dry without it', the frame and the storm record 'without'), 'the switch thrown on a running game follows', and
// 'rule on or off' of `same`. The car park's point is found by the office's two rectangles themselves. The rig stands with the dressing rows up, as the page
// has them: the weather's own rows are dressing.
//   bun run tools/qa/probes.ts ownroofs      (also under `weather`)
//
//   open      every lattice point of a level's ground with open sky over it (an upward ray meets nothing solid; half a metre of slack at a roof's edge) is rained on, and every point under a ceiling
//             slab is not: the share the footprints get wrong is ~0 (the records office, the compound, the barracks, the depot)
//   frame     what the renderer is handed: the first two roofs as the two rectangles, the third and fourth packed, the rain rate as ever, no fog
//   wet       a man stood on the records office's car park is soaked; under the building's roof he stays dry
//   wind      the storm's wind is cut by the same roofs
//   gust      an outside door the office's stair-tower rule kept still (the warehouse's fire exit) blows in when it stands open, like every other door onto open air
//   same      the office, and the level built in its shell (the block), keep the office's two rectangles to the bit
//   quiet     a quiet level keeps its dry, still night
//   kind      a picked weather takes over and hands back
import { standUp, ROOT } from './headless.ts';
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';

const DT = 1 / 60;

export async function ownRoofsCheck(o: { verbose?: boolean } = {}): Promise<boolean> {
  const { INTERIOR, ANNEX } = await import(`${ROOT}/src/game/weather.ts`);
  const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`);
  const { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`);
  const { WORLD } = await import(`${ROOT}/src/scene/world.ts`);
  const BREAK = process.env.OWNROOFS_BREAK === '1';
  const quiet = async <T>(fn: () => T | Promise<T>): Promise<T> => { if (o.verbose) return await fn(); const l = console.log, e = console.error, w = console.warn; console.log = console.error = console.warn = () => { }; try { return await fn(); } finally { console.log = l; console.error = e; console.warn = w; } };
  let fails = 0, checks = 0;
  const verdict = (ok: boolean, label: string, detail: string) => { checks++; if (!ok) fails++; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${label}\n    ${detail}`); };
  const eq = (a: readonly number[], b: readonly number[]) => a.length === b.length && a.every((v, i) => v === b[i]);

  async function rig(file: string) {
    const lvl = loadLevel(parseLevelData(readFileSync(resolve(ROOT, 'levels', file), 'utf8')));
    const W: any = await quiet(() => standUp({ level: lvl, dress: true })); const g = W.game; g.godMode = true; g.aiEnabled = false;
    const step = (n = 1) => { for (let i = 0; i < n; i++) W.step(DT); };
    step(3);
    return { W, g, step, wx: g.weather as any, frame: () => { const F = (g.engine as any).weather; return { ...F, interior: [...F.interior], annex: [...F.annex] } as any; } };
  }
  /** half a metre or more inside one of the office's two rectangles: what every level's storm was cut by before the rule */
  const inOld = (x: number, z: number) => [INTERIOR, ANNEX].some(r => x > r[0] + 0.5 && z > r[1] + 0.5 && x < r[2] - 0.5 && z < r[3] - 0.5);
  /** the ground's lattice (1 m, a metre in from the world's edge): open sky / under a ceiling slab by an upward ray, against what the footprints say */
  function tally(R: Awaited<ReturnType<typeof rig>>) {
    const { g, wx } = R; let open = 0, openDry = 0, roofed = 0, roofedRain = 0; let first: number[] | null = null; const dry: string[] = [];
    for (let x = WORLD.min[0] + 1.5; x < WORLD.max[0] - 1; x += 1) for (let z = WORLD.min[2] + 1.5; z < WORLD.max[2] - 1; z += 1) {
      const sky = !g.col.solidBlocked([x, 0.3, z], [x, WORLD.ceilingY + 6, z]);
      const slab = !sky && !g.col.solidBlocked([x, 0.3, z], [x, WORLD.ceilingY - 0.4, z]) && [-0.45, 0.45].every(d => g.col.solidBlocked([x + d, WORLD.ceilingY - 0.4, z + d], [x + d, WORLD.ceilingY + 0.6, z + d]));
      const under = BREAK ? inOld(x, z) : wx.underRoof(x, z);
      // (a footprint is its slab padded 0.16 m, the third and fourth packed outward to 0.2 m: an open point counts as kept dry only half a metre inside one)
      const deep = under && [[-0.5, 0], [0.5, 0], [0, -0.5], [0, 0.5]].every(d => BREAK ? inOld(x + d[0], z + d[1]) : wx.underRoof(x + d[0], z + d[1]));
      if (sky) { open++; if (deep) { openDry++; dry.push(`${x},${z}`); } if (x > 5 && !first && inOld(x, z)) first = [x, z]; } else if (slab) { roofed++; if (!under) roofedRain++; }
    }
    return { open, openDry, roofed, roofedRain, first, dry };
  }

  try {
    // ---- open / frame / wet / wind: three levels of their own shape
    for (const file of ['records.json', 'compound.json', 'barracks.json', 'depot.json']) {   // (the depot was built in the office's shell until its floor was rebuilt, 10-13: it is a level of its own shape now)
      const on = await rig(file); const t = tally(on); const F = on.frame(); const roofs = on.wx.roofs as number[][];
      if (o.verbose && t.dry.length) console.log(`    ${file}: open points the level's roofs keep dry: ${t.dry.join(' · ')}`);
      verdict(on.wx.ownStorm === true && t.openDry === 0 && t.roofedRain <= Math.ceil(t.roofed * 0.02),
        `open — ${file}: rain over all open ground, none under its roofs`,
        `${t.openDry} of ${t.open} open-sky points kept dry, ${t.roofedRain} of ${t.roofed} roofed points rained on · own storm ${on.wx.ownStorm}`);
      verdict(F.rain > 0 && eq(F.interior, roofs[0]) && eq(F.annex, roofs[1]) && F.roofC !== 0 && (roofs.length > 2) === (F.roofC !== 1) && (roofs.length > 3) === (F.roofD !== 0) && F.fog === 0,
        `frame — ${file}: the renderer is handed the level's ${roofs.length} roofs, the storm's own rain, no fog`,
        `rain ${F.rain} · rect 1 ${F.interior.map((v: number) => v.toFixed(2)).join(', ')} · rect 2 ${F.annex.map((v: number) => v.toFixed(2)).join(', ')} · packed ${F.roofC} / ${F.roofD} · fog ${F.fog}`);
      const S = (on.g.wind as any).storm;
      const want = roofs.length <= 2 ? roofs[0] : [Math.min(...roofs.map(r => r[0])), Math.min(...roofs.map(r => r[1])), Math.max(...roofs.map(r => r[2])), Math.max(...roofs.map(r => r[3]))];
      verdict(on.wx.ownStorm === true && eq(S.interior, want) && !eq(S.interior, INTERIOR),
        `wind — ${file}: the storm's air is kept out by the same roofs`, `storm record ${S.interior.map((v: number) => v.toFixed(2)).join(', ')}`);
      if (file === 'records.json') {
        // a man on the car park (the first open point inside the office's old rectangle), then a man under the building's own roof
        const soak = (R: typeof on, p: number[]) => { const c = R.g.player.char; c.wet = 0; c.beads = 0; R.g.teleportPlayer([p[0], 0, p[1]]); R.step(240); return c.wet as number; };
        const lot = t.first; const r0 = roofs[0]; const inside = [(r0[0] + r0[2]) / 2, (r0[1] + r0[3]) / 2];
        const wOn = lot ? soak(on, lot) : -1;
        verdict(!!lot && wOn > 0.3, `wet — a man on the records office's car park is rained on`, lot ? `four seconds at (${lot.map(v => v.toFixed(1)).join(', ')}): coat ${wOn.toFixed(2)}` : 'no open point of the car park lies inside the office\'s old rectangle');
        const dOn = soak(on, inside);
        verdict(dOn === 0, `wet — under the building's roof he stays dry`, `four seconds at (${inside.map(v => v.toFixed(1)).join(', ')}): coat ${dOn.toFixed(2)}`);
        // kind: a picked weather takes over and hands back
        const wx = on.wx; const k1 = wx.setKind('rain'); const own1 = wx.ownStorm, n1 = wx.roofs.length; wx.setKind(null); on.step(1); const own2 = wx.ownStorm, F2 = on.frame();
        verdict(k1 && !own1 && n1 === roofs.length && own2 && eq(F2.interior, roofs[0]),
          `kind — a picked weather takes over and hands back`, `rain picked: own ${own1}, ${n1} roofs · unpicked: own ${own2}`);
      }
    }
    // ---- gust: an outside door the office's stair tower rule left without a gust has one under the level's own roofs
    { const gusts = (R: Awaited<ReturnType<typeof rig>>) => ((R.g.wind as any).all as { why: string; name: string }[]).filter(s => s.why === 'door').map(s => s.name);
      const on = await rig('warehouse.json'); const b = gusts(on);
      verdict(b.includes('door:fire_exit'), `gust — warehouse.json: the fire exit's gust is there under the level's own roofs`, `doors with a gust: ${b.join(', ')}`); }
    // ---- same: the office and the levels in its shell
    for (const file of ['office.json', 'block.json']) {
      const on = await rig(file); const F = on.frame(); const t = tally(on);
      verdict(on.wx.ownStorm === false && on.wx.roofs[0] === INTERIOR && on.wx.roofs[1] === ANNEX && eq(F.interior, INTERIOR) && eq(F.annex, ANNEX) && F.roofC === 0 && F.roofD === 0,
        `same — ${file}: the office's two rectangles, as ever`, `own ${on.wx.ownStorm} · rect 1 ${F.interior.join(', ')} · packed ${F.roofC} · ${t.openDry} of ${t.open} open points dry, ${t.roofedRain} of ${t.roofed} roofed rained on`);
    }
    // ---- quiet: a quiet level's night stays dry and still
    { const on = await rig('bazaar.json'); const F = on.frame();
      verdict(on.wx.ownStorm === false && F.rain === 0 && on.wx.storm === 0, `quiet — bazaar.json keeps its dry, still night`, `own ${on.wx.ownStorm} · rain ${F.rain} · storm ${on.wx.storm}`); }
  } finally { /* nothing of the game's is written here */ }
  console.log(fails ? `ownroofs: ${fails} of ${checks} FAILED` : `ownroofs: ALL PASS (${checks} checks)`);
  return fails === 0;
}

if (import.meta.main) { const ok = await ownRoofsCheck({ verbose: process.argv.includes('--verbose') }); process.exit(ok ? 0 : 1); }
