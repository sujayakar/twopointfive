// Faces that share a plane: bun run tools/faces.ts; exits 1 when there are any.
import { makeBuilding } from '../src/scene/building';
import { faceFights } from '../src/scene/faces';
const f = faceFights(makeBuilding().world.pieces); console.log(`${f.length} pairs of faces share a plane`); for (const x of f.slice(0, 20)) console.log(x.gap.toFixed(5), x.a.p.mat, x.a.at.map(v => v.toFixed(2)).join(), x.a.n.map(v => v.toFixed(2)).join(), x.a.hu, x.a.hv, '|', x.b.p.mat, x.b.at.map(v => v.toFixed(2)).join(), x.b.hu, x.b.hv); process.exit(f.length ? 1 : 0);
