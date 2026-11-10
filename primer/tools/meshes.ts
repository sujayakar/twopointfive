import { pebble, column } from '../src/mesh/shapes';
import { makeLevels, cutAt } from '../src/mesh/clusters';
for (const [name, m] of [['pebble', pebble(1, 4)], ['column', column(160, 160)]] as const) {
  const t = performance.now();
  const l = makeLevels(m);
  console.log(name, m.triangles.length, 'tris', Math.round(performance.now() - t), 'ms', l.levels, 'levels', l.clusters.length, 'clusters', l.points.length, 'points');
  for (let k = 0; k < l.levels; k++) { const c = l.clusters.filter((x) => x.level === k); console.log('  level', k, c.length, c.reduce((s, x) => s + x.triangles.length, 0), 'err', Math.min(...c.map((x) => x.error)).toFixed(5), Math.max(...c.map((x) => x.error)).toFixed(5)); }
  for (const e of [0, 0.0005, 0.002, 0.005, 0.02, 0.05]) {
    const cut = cutAt(l, e);
    const edges = new Map<string, number>();
    for (const t of cut) for (let k = 0; k < 3; k++) { const a = t[k]!, b = t[(k + 1) % 3]!; const key = a < b ? a + ',' + b : b + ',' + a; edges.set(key, (edges.get(key) ?? 0) + 1); }
    const open = [...edges.values()].filter((n) => n === 1).length, full = (() => { const ed = new Map<string, number>(); for (const t of m.triangles) for (let k = 0; k < 3; k++) { const a = t[k]!, b = t[(k + 1) % 3]!; const key = a < b ? a + ',' + b : b + ',' + a; ed.set(key, (ed.get(key) ?? 0) + 1); } return [...ed.values()].filter((n) => n === 1).length; })();
    console.log('  cut', e, cut.length, 'open edges', open, '(finest', full + ')');
  }
}
