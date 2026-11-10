// The meshes, built and looked into: bun run tools/meshes.ts [pebble subdivisions] [column splits]
// Every level closed, every cut without a crack, how long it took, how far levels stand off the finest.
import { pebble, column } from '../src/mesh/shapes';
import { buildDag, cutAt, errorFor, type Dag } from '../src/mesh/dag';
import { edges, type Mesh } from '../src/mesh/mesh';
const N = +(process.argv[2] ?? 5), K = +(process.argv[3] ?? 1);
let wrong = 0; const say = (tri: Uint32Array) => { const e = edges(tri); if (e.open) wrong++; return `${e.open ? 'BAD: ' : ''}open edges ${e.open}, touching ${e.touch}`; };
const look = (name: string, make: () => Mesh) => {
  let t = performance.now(); const m = make(), tm = performance.now() - t; t = performance.now(); const d: Dag = buildDag(m), td = performance.now() - t;
  let mn = [1e9, 1e9, 1e9], mx = [-1e9, -1e9, -1e9]; for (let i = 0; i < m.pos.length; i++) { mn[i % 3] = Math.min(mn[i % 3]!, m.pos[i]!); mx[i % 3] = Math.max(mx[i % 3]!, m.pos[i]!); }
  const all = d.clusters.reduce((a, c) => a + c.tri.length / 3, 0), roots = d.clusters.filter(c => c.parentError === Infinity);
  console.log(`${name}: ${d.triangles} triangles made in ${tm.toFixed(0)} ms (size ${mx.map((v, k) => (v - mn[k]!).toFixed(3)).join(' x ')}), ${say(m.tri)}; ${d.clusters.length} clusters, ${d.levels} levels, ${all} triangles in all, in ${td.toFixed(0)} ms (${(d.triangles / td * 1000 / 1e6).toFixed(2)} M a second); ${(all / d.clusters.length).toFixed(1)} a cluster; left ${roots.reduce((a, c) => a + c.tri.length / 3, 0)} in ${roots.length}`);
  for (let k = 0; k < 8; k++) { const e = errorFor(d, d.triangles / 4 ** k), c = cutAt(d, e), tri = new Uint32Array(c.reduce((a, q) => a + q.tri.length, 0)); let o = 0; for (const q of c) { tri.set(q.tri, o); o += q.tri.length; } console.log(`   1/${4 ** k}: error ${(e * 1000).toFixed(4)} mm, apart ${(Math.max(...c.map(q => q.apart)) * 1000).toFixed(4)}, ${tri.length / 3} triangles in ${c.length} clusters, ${say(tri)}, full ${(tri.length / 3 / c.length / 128 * 100).toFixed(0)} %`); }
  // errors only grow upward
  let bad = 0; for (const c of d.clusters) if (c.parentError <= c.error) bad++; if (bad) { wrong++; console.log('BAD: errors that do not grow', bad); }
};
look('pebble', () => pebble(3, N)); look('column', () => column(false, 0.011, K)); look('drum', () => column(true, 0.011, K));
process.exit(wrong ? 1 : 0);
