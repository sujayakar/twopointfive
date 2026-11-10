// What can be said of objects without a picture: bun run tools/objects.ts [names]; exits 1 when a mark is missed. FAULT=name plants one
import { OBJECTS } from '../src/objects/index';
import { rules, alive } from '../src/objects/rules';
import { FAULTS } from '../src/objects/faults';
import { judged } from '../src/objects/marks';
const names = process.argv.slice(2), fault = FAULTS.find(f => f.name === process.env.FAULT); let missed = 0; if (process.env.FAULT && !fault) { console.error('no fault ' + process.env.FAULT); process.exit(2); }
for (const name of names.length ? names : Object.keys(OBJECTS)) {
  const t0 = performance.now(), kind = OBJECTS[name]; if (!kind) { console.error('no object ' + name); process.exit(2); } const m = fault && fault.of === name ? fault.plant(kind.make()) : kind.make(), r = rules(m), found = [...r.found, ...alive(m)];
  console.log(`${m.title}: ${r.counts.map((c, k) => `level ${k} ${c.pieces + c.moving} pieces${c.leaves ? ` ${c.leaves} leaves` : ''} ${(c.bytes / 1024).toFixed(0)} KB, from ${(m.levels[k]!.error / 3.754e-4).toFixed(0)} m`).join(' · ')} (${(performance.now() - t0).toFixed(0)} ms)`);
  for (const f of found) { const j = judged(f, m.levels[0]!.shape); if (!j.ok) missed++; if (!j.ok || process.env.ALL) console.log(`  ${j.ok ? 'ok  ' : 'MISS'} ${f.id.padEnd(14)} ${f.level === undefined ? ' ' : f.level} ${f.value.toFixed(2)}${f.unit} (${j.mark}) ${f.what}${f.note ? '; ' + f.note : ''}`); }
}
process.exit(missed ? 1 : 0);
