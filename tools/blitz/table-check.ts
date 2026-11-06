// THE TABLE HELD TO THE PROBE RUNNER'S OWN LIST OF NAMES (run by hand after a probe is added, renamed or deleted):
//   bun run tools/blitz/table-check.ts
// tools/blitz/qa-kinds.json (the table TPS_GAME=table reads, tools/blitz/word.ts) names every name `tools/qa/probes.ts list` knows, as a probe with its
// kind, as a family that only runs its members, or as a name the census found no line for. RED (exit 1) when probes.ts lists a name the table lacks (a check
// of that name is stood on the TypeScript tick and counted on the closing line as having no row), or the table has a name probes.ts lost (a dead row, a pin
// of a probe that is gone). Mend: tools/blitz/qa-census.py --kinds tools/blitz/qa-kinds.json (its header says how).
import { readFileSync, readdirSync } from 'node:fs';
const ROOT = new URL('../..', import.meta.url).pathname;
const p = Bun.spawnSync([process.execPath, 'run', 'tools/qa/probes.ts', 'list'], { cwd: ROOT, env: { ...process.env, TPS_GAME: 'none' } });
if (p.exitCode !== 0) { console.log(`table-check: \`probes.ts list\` exited ${p.exitCode}\n${p.stderr.toString().slice(0, 600)}`); process.exit(2); }
const known = new Set<string>();
for (const l of p.stdout.toString().split('\n')) { const m = l.match(/^\s+([\w()-]+):? (.*)$/); if (!m || l.startsWith('the ')) continue; const fam = m[1].replace(/:$/, ''); if (fam !== '(stand-alone)') known.add(fam); for (const x of m[2].split(/\s+/).filter(Boolean)) known.add(x); }
const said = Number(p.stdout.toString().match(/^the (\d+) known names/)?.[1] ?? NaN);
const K = JSON.parse(readFileSync(`${ROOT}/tools/blitz/qa-kinds.json`, 'utf8')) as { probes: Record<string, string>; pins?: Record<string, string[]>; families?: string[]; unread?: string[] };
// (a check may RUN under a name the list does not have: `if (on('morale', 'all')) await run('training', …)`. Such a row is alive while the runner's files still say the name)
let SRC = ''; const ranAs = (n: string): boolean => { SRC ||= readdirSync(`${ROOT}/tools/qa`).filter(f => f === 'probes.ts' || f.endsWith('-probes.ts')).map(f => readFileSync(`${ROOT}/tools/qa/${f}`, 'utf8')).join('\n'); return SRC.includes(`'${n}'`); };
const RUNNER = new Set(['all', 'all-quick', 'list']);   // (the runner's own words, not checks)
const rows = new Set([...Object.keys(K.probes), ...(K.families ?? []), ...(K.unread ?? [])]);
const lacks = [...known].filter(n => !rows.has(n) && !RUNNER.has(n)).sort(), lost = [...rows].filter(n => !known.has(n) && !RUNNER.has(n) && !ranAs(n)).sort(), deadPins = [...Object.keys(K.pins ?? {}), ...Object.keys((K as any).waits ?? {})].filter(n => !(n in K.probes)).sort();
const twice = Object.keys(K.probes).filter(n => (K.families ?? []).includes(n) || (K.unread ?? []).includes(n));
const line = (ok: boolean, what: string, detail: string) => console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}: ${detail}`);
line(known.size + 1 === said || known.size === said, 'the list read whole', `${known.size} names read, the runner says ${said}`);
line(!lacks.length, 'every name probes.ts lists has a row', lacks.length ? `${lacks.length} without: ${lacks.slice(0, 30).join(' ')}${lacks.length > 30 ? ' …' : ''}` : `${Object.keys(K.probes).length} probes by kind, ${(K.families ?? []).length} families, ${(K.unread ?? []).length} names the census found no line for (stood on the TypeScript tick, counted as no row)`);
line(!lost.length, 'every row is a name probes.ts lists', lost.length ? `${lost.length} lost: ${lost.slice(0, 30).join(' ')}${lost.length > 30 ? ' …' : ''}` : 'none lost');
line(!deadPins.length && !twice.length, 'every pin and every wait is of a probe with a row, and no name has two rows', deadPins.length || twice.length ? `pins of no probe: ${deadPins.join(' ') || 'none'} · twice: ${twice.join(' ') || 'none'}` : `${Object.keys(K.pins ?? {}).length} pins, ${Object.keys((K as any).waits ?? {}).length} waits`);
const red = (known.size + 1 !== said && known.size !== said) || lacks.length || lost.length || deadPins.length || twice.length;
console.log(red ? 'table-check: RED' : 'table-check: green'); process.exit(red ? 1 : 0);
