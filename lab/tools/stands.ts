// Does everything stand on something, and is nothing inside anything else: bun run tools/stands.ts; exits 1 when not.
import { makeBuilding } from '../src/scene/building';
import { makeStreet } from '../src/scene/street';
import { makeForest } from '../src/scene/forest';
import { makePass } from '../src/scene/pass';
import { floating, clashes } from '../src/scene/stand';
let bad = 0; for (const [name, make] of [['rooftop', makeBuilding], ['street', makeStreet], ['forest', makeForest], ['pass', makePass]] as const) {
  const b = make(), t0 = performance.now(), at = (p: { at: number[] }) => p.at.map(v => v.toFixed(2)).join(' ');
  for (const l of b.levels) { const f = floating(b.world, l.cut); bad += f.length; console.log(`${name}, ${l.name}: ${f.length} things stand on nothing`); for (const c of f.slice(0, Number(process.env.N ?? 60))) { const p = c[0]!; console.log('  ', c.length, 'pieces', p.thing?.name ?? '', p.rig ?? '', 'mat', p.mat, 'at', at(p), 'half', p.half.map(v => v.toFixed(3)).join(' '), p.flags & 1024 ? 'cut' : 'not cut'); } }
  const c = clashes(b.world), seen = new Map<string, number>(); bad += c.length; console.log(`${name}: ${c.length} pairs of things' pieces are in each other`); for (const x of c) { const k = x.a.thing!.name + ' / ' + x.b.thing!.name; if ((seen.get(k) ?? 0) < 3) console.log('  ', k, 'by', x.by.toFixed(3), 'at', at(x.a), '|', at(x.b)); seen.set(k, (seen.get(k) ?? 0) + 1); } console.log(`(${(performance.now() - t0).toFixed(0)} ms)`);
} process.exit(bad ? 1 : 0);
