// THE WHOLE STATES OF A FUZZ RUN, EVERY HALF MINUTE (tools/blitz/fuzz.sh, fuzz-squad.sh; PLAY.md, "The fuzz"). The die and the block are the bodies and the
// dice: the chronicle, a talk's record, an order's words, a gun's rows are in neither, and a play can stay equal in both for minutes after the two games
// have parted in what is not a body. So each side keeps its whole state every 1,800 ticks (`<word>-t<k>.bin`, RH_DUMP) and this holds every pair to each
// other by path, canonically (diff.ts --live: tables numbered by reach from the live game), and writes
//   dumps.txt        a line a dump: `<tick> <paths not known> <paths differing> | <the not-known by module>`
//   diff-<tick>.txt  a dump that has a path not known: diff.ts's own lines
//   rust-hashes.txt  a line a dump of the Rust side: `<tick> <bytes> <hash>` (what a later run of the Rust game alone is held to: fuzz-replay.sh)
// The states are deleted afterwards unless --keep. Exit 0: every pair has 0 paths not known · 1: some pair has one · 3: no pair.
//   bun run tools/blitz/fuzz-dumps.ts <folder> [--keep] [--upto <tick>]
import { readdirSync, readFileSync, writeFileSync, unlinkSync, existsSync } from 'node:fs';
const dir = process.argv[2], keep = process.argv.includes('--keep'), upto = (() => { const i = process.argv.indexOf('--upto'); return i < 0 ? Infinity : +process.argv[i + 1]; })();
if (!dir) { console.error('fuzz-dumps.ts <folder> [--keep] [--upto <tick>]'); process.exit(64); }
const here = import.meta.dir, ticks = (w: string) => readdirSync(dir).map(f => f.match(new RegExp(`^${w}-t(\\d+)\\.bin$`))?.[1]).filter((x): x is string => !!x).map(Number).sort((a, b) => a - b);
const T = new Set(ticks('ts')), R = ticks('rust'), canon = `${readFileSync(`${here}/canon.txt`, 'utf8').trim()},FirearmId`;
const hashes: string[] = [], lines: string[] = []; let bad = 0, pairs = 0;
// (the first load of a save, by the reference's own timeline: frame f is tick f + 1. From there the modules' own logs and tallies are known to differ: known-loaded.txt)
const loadAt = (() => { if (!existsSync(`${dir}/ts.log`)) return Infinity; for (const l of readFileSync(`${dir}/ts.log`, 'utf8').split('\n')) { const m = l.match(/^\s*\[seed \d+\] [\d.]+s f(\d+) (.*)$/); if (m && /STIM load: .*put back|the checkpoint loaded/.test(m[2])) return +m[1] + 1; } return Infinity; })();
for (const k of R) { const b = readFileSync(`${dir}/rust-t${k}.bin`); hashes.push(`${k} ${b.length} ${Bun.hash(b).toString(36)}`); }
if (R.length) writeFileSync(`${dir}/rust-hashes.txt`, hashes.join('\n') + '\n');
for (const k of R) { if (!T.has(k) || k > upto) continue; pairs++;
  const p = Bun.spawnSync(['bun', 'run', `${here}/diff.ts`, `${dir}/ts-t${k}.bin`, `${dir}/rust-t${k}.bin`, '--live', '--canon', canon, '--known', k >= loadAt ? `${here}/known.txt,${here}/known-loaded.txt` : `${here}/known.txt`, '--show', '400', '--why'], { stdout: 'pipe', stderr: 'pipe' });
  const out = p.stdout.toString() + p.stderr.toString(), m = out.match(/^(\d+) paths differ, (\d+) known, (\d+) NOT KNOWN(?:: ([^;]*))?/);
  if (!m) { lines.push(`${k} ? ? | the diff did not run: ${out.trim().split('\n').pop()?.slice(0, 200)}`); bad++; writeFileSync(`${dir}/diff-${k}.txt`, out); continue; }
  const stale = out.match(/by reach from the live game[^:]*: (\d+ \/ \d+)/)?.[1] ?? '0 / 0';
  lines.push(`${k} ${m[3]} ${m[1]} | ${m[4] ?? ''} | counted, not reported (ts / rust): ${stale}${k >= loadAt ? ` | after the load at tick ${loadAt}: the modules' logs and tallies known` : ''}`);
  if (+m[3] > 0) { bad++; writeFileSync(`${dir}/diff-${k}.txt`, out); } else if (existsSync(`${dir}/diff-${k}.txt`)) unlinkSync(`${dir}/diff-${k}.txt`); }
writeFileSync(`${dir}/dumps.txt`, lines.join('\n') + (lines.length ? '\n' : ''));
if (!keep) for (const f of readdirSync(dir)) if (/^(ts|rust)-t\d+\.bin$/.test(f)) unlinkSync(`${dir}/${f}`);
const first = lines.find(l => +l.split(' ')[1] > 0 || l.split(' ')[1] === '?');
console.log(`the whole states, every half minute: ${pairs} pairs (ticks ${R.filter(k => T.has(k) && k <= upto).join(', ') || 'none'}); ${pairs && !bad ? '0 paths not known in every one' : !pairs ? 'NO PAIR' : `${bad} with paths not known; the first: tick ${first}`}`);
process.exit(!pairs ? 3 : bad ? 1 : 0);
