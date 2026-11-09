// THE LIST OF WHAT THE SIMULATION READS, AS ONE LINE A NUMBER: rust/game_rules/src/rustknobs/base.rs had a vector in the order of rust/sim/src/knobs.rs's list, a proof it was written from
// that order, and the old name of each row in a comment. This writes each as `k[ITS_NAME] = where it comes from;`, which holds in any order and needs no proof. Run once.
//   bun run tools/fold/knobs-lines.ts
import { readFileSync, writeFileSync } from 'node:fs';
const BASE = 'rust/game_rules/src/rustknobs/base.rs', SIM = 'rust/sim/src/knobs.rs';
const sim = readFileSync(SIM, 'utf8'), body = sim.slice(sim.indexOf('table! { Knob, KNOBS, KNOB_NAMES;')), lines = body.slice(0, body.indexOf('\n}'));
const names = [...lines.matchAll(/^\s*(\w+) = "([^"]+)",/gm)].map(m => ({ name: m[1], path: m[2] }));
let s = readFileSync(BASE, 'utf8');
const row = /^\s*(.+?),\s*\/\/ (\d+) (\S+)\s*$/;
const out = s.split('\n'); let first = -1, last = -1, n = 0;
for (let i = 0; i < out.length; i++) { const m = row.exec(out[i]); if (!m) continue; if (first < 0) first = i; last = i; const at = +m[2], w = names[at];
  if (at !== n || !w || w.path !== m[3]) throw new Error(`line ${i + 1}: row ${at} of ${n} says ${m[3]}, the list ${w?.path}`);
  out[i] = `    k[${w.name}] = ${m[1]};`; n++; }
if (n !== names.length) throw new Error(`${n} rows for ${names.length} names`);
if (!/let v: \[f64; \d+\] = \[/.test(out[first - 1]) || out[last + 1].trim() !== '];') throw new Error('the vector is not where it was');
out[first - 1] = '    let k = ::sim::knobs::values();'; out.splice(last + 1, 3);   // (the closing, the place in memory, the copy)
s = out.join('\n');
const a = s.indexOf('    /// the list these lines were made from'), b = s.indexOf('    let t_life') ; if (a < 0 || b < a) throw new Error('the proof is not where it was');
s = s.slice(0, a) + s.slice(b);
s = s.replace("/// (src/game/rustknobs.ts:42 knobsToRust)\n", "/// every number the simulation reads, written before it reads them\n").replace(/use alloc::\{string::String, vec::Vec\};\n/, "use alloc::{string::String, vec::Vec};\nuse ::sim::knobs::*;\n");
writeFileSync(BASE, s);
console.log(`${n} rows`);
