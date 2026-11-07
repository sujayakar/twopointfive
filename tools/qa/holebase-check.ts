// THE BOTTOM LAYER OF A PILE A BLOWN HOLE TAKES GOES WITH IT — the check behind FURN.mouth.base (src/game/furnblast.ts mouth; a settled rule since 2026-11-07:
// every tool stands the game with it on. Gone with its off side: 'with it off the four bottom sacks lie there as they did', and the recording and stand line).
//   bun run tools/qa/probes.ts holebase        (family `furnblast`)        HOLEBASE_BREAK=1 = every stand at chaos 0, where a charge opens nothing: must FAIL
// the maintainer, 10-17, of two wall charges on the pass (levels/pass.json): "one set of guys couldn't pathfind through the wall breach". The store keeps sacks
// in piles against that wall, each layer 0.2 m thick. The blast took the layers above and left the bottom one lying in the hole: the hole rule passes over
// what is no higher than a step (top at or under 0.2 m), and the furniture's own bands never list a thing that low. The feet's side of it is UNDERFOOT
// (collision.ts, its own check `underfoot`); this is the blast's side.
// THE SITE: his own (the store's outer wall by the floor point 31.3, 5.0, set from the yard side), on a fresh stand of the level with the page's destruction
// switches (and its lived-in rooms, LIVED.on: the piles are the store's own dressing, and the frozen stand holds that off): a charge set as the order sets it
// and blown as F blows it. Then what is left of the two piles by the hole, read off the level's own boxes.
// THE BARS, on the same site with the piles reshaped by the check before the level stands: the right pile's bottom layer made a MAT (0.06 m thick, under the rule's baseThick, the layers
// above let down onto it) and a thing the hole rule never takes (a block up past its height bar) stood on the left pile's bottom layer. Both bottom layers
// must stay. The low bank at the wall's foot (top 0.17 m, nothing on it) must stay at every site.
import { readFileSync } from 'node:fs';
import { standUp, ROOT } from './headless.ts';
import { samBreach } from './charge-rig.ts';
type Vec3 = [number, number, number];
export async function holeBaseCheck(): Promise<boolean> {
  const BRK = process.env.HOLEBASE_BREAK === '1'; let all = true;
  const verdict = (ok: boolean, what: string, detail: string) => { all &&= ok; console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}\n    ${detail}`); };
  console.log(`\n=== holebase: the bottom layer of a pile a blown hole takes goes with it${BRK ? ' (HOLEBASE_BREAK: chaos 0, a charge opens nothing: must fail)' : ''} ===`);
  const { loadLevel } = await import(`${ROOT}/src/scene/levelLoad.ts`); const { parseLevelData } = await import(`${ROOT}/src/scene/levelData.ts`);
  const { FURN } = await import(`${ROOT}/src/game/furnblast.ts`); const { DESTRUCTION, FURNBLAST } = await import(`${ROOT}/src/game/destructionSwitch.ts`);
  const { WALLCHARGE, wallAt } = await import(`${ROOT}/src/game/wallcharge.ts`); const { BoxFlag } = await import(`${ROOT}/src/scene/boxes.ts`) as any;
  const { LIVED } = await import(`${ROOT}/src/scene/lived.ts`);
  const was = { l: LIVED.on, d: DESTRUCTION.on, f: FURNBLAST.on, m: FURN.mouth.on, w: WALLCHARGE.on, o: WALLCHARGE.outer }, log = console.log;
  const data = readFileSync(`${ROOT}/levels/pass.json`, 'utf8'), GONE = BoxFlag.NoPrimary | BoxFlag.NoShadow;
  const raise = () => { LIVED.on = true; DESTRUCTION.on = true; FURNBLAST.on = true; FURN.mouth.on = true; WALLCHARGE.on = true; WALLCHARGE.outer = true; };
  const at: Vec3 = [31.3, 0, 5.0], from: Vec3 = [31.3, 0, 3.6];
  /** the two piles by his hole and the bank at the wall's foot, by where the level puts them */
  const near = (b: any) => Math.abs(b.c[0] - 30.8) < 1.3 && b.c[2] > 5.2 && b.c[2] < 6.3 && b.h[0] < 0.5 && b.h[2] < 0.5 && b.c[1] + b.h[1] < 1.0;
  const sack = (b: any) => near(b) && Math.abs(b.h[1] - 0.1) < 0.02, base = (b: any) => near(b) && b.c[1] - b.h[1] < 0.02, right = (b: any) => b.c[0] > 30.9;
  const bank = (b: any) => Math.abs(b.c[0] - 30.6) < 0.2 && Math.abs(b.c[2] - 4.63) < 0.1 && b.h[0] > 2 && b.c[1] + b.h[1] < 0.2;
  const blown = async (edit?: (L: any) => void) => { raise(); console.log = () => { };
    try { const L: any = loadLevel(parseLevelData(data)); edit?.(L); const W: any = await standUp({ level: L, seed: 1 }); raise(); const g = W.game; g.godMode = true; g.aiEnabled = false; g.tune.chaos = BRK ? 0 : 0.5; for (let i = 0; i < 4; i++) W.step(1 / 60);
      const boxes: any[] = g.col.boxes, piles = boxes.filter(near), n0 = piles.length, b0 = piles.filter(base), bk = boxes.find(bank);
      const w = wallAt(g, at, from); if (!w || !w.ok) return null;
      const res = samBreach(W, w); for (let i = 0; i < 60; i++) W.step(1 / 60);   // (set by Sam's own hands from the yard, set off by his own action: charge-rig.ts)
      const stays = (b: any) => !(b.flags & GONE);
      return { g, hole: !!res.hole, breach: res.why, n0, bases: b0.length, left: piles.filter(stays).length, baseLeft: b0.filter(stays).length, baseLeftR: b0.filter(b => right(b) && stays(b)).length, baseLeftL: b0.filter(b => !right(b) && stays(b)).length,
        above: piles.filter(b => !base(b)).length, aboveLeft: piles.filter(b => !base(b) && stays(b)).length, aboveLeftR: piles.filter(b => !base(b) && right(b) && stays(b)).length, aboveGoneL: piles.filter(b => !base(b) && !right(b) && !stays(b)).length, bank: !!bk, bankStays: !!bk && stays(bk), tall: boxes.filter(b => near({ ...b, c: [b.c[0], 0.3, b.c[2]], h: [b.h[0], 0.1, b.h[2]] }) && b.c[1] + b.h[1] > 2.3 && stays(b)).length };
    } finally { console.log = log; } };
  try {
    // (1) his breach
    const on = await blown();
    if (!on) verdict(false, "the store's outer wall takes a charge where his was set", 'no wall place there: the level changed, and this check needs its site again');
    else {
      verdict(on.hole && on.n0 >= 12 && on.bases === 4 && on.left === 0 && on.bankStays,
        'his breach: nothing of the two sack piles is left by the hole, bottom layer included; the low bank at the wall\'s foot, which carried nothing, stays',
        `a hole ${on.hole} · ${on.n0} sacks in the two piles, ${on.bases} of them the bottom layer → left ${on.left} (bottom layer ${on.baseLeft}) · the bank stays ${on.bankStays}`);
      // (2) the bars: a mat, and a low thing with something on it the rule never takes
      const bars = await blown((L: any) => { const bx: any[] = L.boxes, pile = bx.filter(sack);
        for (const b of pile.filter(right)) { if (base(b)) { b.c[1] = 0.03; b.h[1] = 0.03; } else b.c[1] -= 0.14; }   // the right pile: its bottom layer a mat, the rest let down onto it
        const up = pile.filter(b => !right(b) && Math.abs(b.c[1] - 0.3) < 0.02)[0]; if (up) { up.h[1] = 1.2; up.c[1] = 1.4; } });   // the left pile: a block from 0.2 m up past the rule's height bar on its bottom layer
      if (!bars) verdict(false, 'the bars: the same wall takes the charge with the piles reshaped', 'no wall place there');
      else verdict(bars.hole && bars.baseLeftR === 2 && bars.aboveLeftR === 0 && bars.baseLeftL === 2 && bars.aboveGoneL >= 1 && bars.tall === 1 && bars.bankStays,
        'the bars: a mat that carried a taken thing stays (no body to it), and a low thing stays with what stands on it when that is a thing the hole rule never takes',
        `a hole ${bars.hole} · the right pile's bottom layer made 0.06 m thick, the layers above it left ${bars.aboveLeftR}: of its 2, ${bars.baseLeftR} stay · the left pile's bottom layer under a block up to 2.6 m and ${bars.aboveGoneL} sack(s) the blast took: of its 2, ${bars.baseLeftL} stay, the block stays ${bars.tall === 1} · the bank stays ${bars.bankStays}`);
    }
  } finally { LIVED.on = was.l; DESTRUCTION.on = was.d; FURNBLAST.on = was.f; FURN.mouth.on = was.m; WALLCHARGE.on = was.w; WALLCHARGE.outer = was.o; console.log = log; }
  console.log(`=== holebase: ${all ? 'ALL PASS' : 'FAILURES above'} ===`);
  return all;
}
