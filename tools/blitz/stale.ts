// WHAT THE PUPPET HAS LET FALL BEHIND, AS A TABLE. The page and every headless tool read a TypeScript `Game` that the Rust game's state is applied onto,
// field by field from the wire's lists (tools/wire/fields.json). A field the tick moves that is on no list stays on the puppet as the level's start left
// it: nothing says so until something outside the tick reads it (a round's mark looking up its box), or a game is started again from the puppet.
// This tool finds them by playing: `RH_STALE=<file>` over any headless tool makes the host, as the process ends, export every game's puppet whole and hold
// it to the Rust game's whole state, and every RH_STALE_EVERY ticks (300) before that (tools/blitz/rust-host.ts `staleLook`); this reads the file and says,
// a field a line: which list it is on, in how many of the games and of the looks it differed, one of the places, and (with --readers) whether something the page runs reads it.
//   RH_STALE=/tmp/stale.tsv bun run tools/qa/playthrough.ts --scn night-breach --seeds 1
//   bun run tools/blitz/stale.ts /tmp/stale.tsv [--readers <a table of readers: a line that names Type.field counts>] [--all] [--tsv]
//   exit 0: no field on no list differed · 1: at least one did (--all also prints the fields on the every-tick list that differed: the host's dress step
//   and a tool's own writes since the last apply are among them, they are not faults by themselves)
// A RUN UNDER RH_STALE IS A LOOK, NOT A TALLY: the play is the same (night-breach seed 1: the digest series equal), but a look exports the puppet whole,
// and three of that tally's own reads of the puppet afterwards differed (which room a kill is counted in). Never keep a record made under the word.
// WHAT IT CANNOT SEE: a field no played game moved; a field of a struct the schema does not walk; a list that differs only in the order two sides number rows.
import { readFileSync } from 'node:fs';
const A = process.argv.slice(2), file = A.find(a => !a.startsWith('--')), has = (k: string) => A.includes(k), arg = (k: string) => { const i = A.indexOf(k); return i >= 0 ? A[i + 1] : undefined; };
if (!file) { console.error('stale.ts <file written under RH_STALE> [--readers FILE] [--all] [--tsv]'); process.exit(2); }
const L = JSON.parse(readFileSync(`${import.meta.dir}/puppet-lists.json`, 'utf8')) as { step: string[]; start: string[] }, STEP = new Set(L.step), START = new Set(L.start);
const readersFile = arg('--readers'), readers = readersFile ? readFileSync(readersFile, 'utf8') : null;
const listOf = (q: string) => STEP.has(q) ? 'every tick' : START.has(q) ? 'START ONLY' : 'NO LIST';
type Row = { q: string; games: number; places: number; eg: string; looks: number };
const rows = new Map<string, Row>(); let games = 0, held = 0, looks = 0; const notes: string[] = [];
for (const l of readFileSync(file, 'utf8').split('\n')) { if (!l) continue; const c = l.split('\t');
  if (c[0] === '# game') { games++; if (/^not held/.test(c[3] ?? '')) notes.push(`${c[1]} (${c[2]}): ${c[3]}`); else { held++; looks += parseInt(c[3] ?? '0') || 0; } continue; }
  if (c[0] === '# threw') { notes.push(`the compare threw: ${c[1]}`); continue; }
  const r = rows.get(c[0]) ?? { q: c[0], games: 0, places: 0, eg: c[2], looks: 0 }; r.games++; r.places += +c[1]; r.looks += +(c[3] ?? 1); rows.set(c[0], r); }
const all = [...rows.values()].map(r => ({ ...r, list: listOf(r.q), read: readers ? (readers.includes(r.q) || readers.includes(r.q.split('::').pop()!) ? 'read' : '-') : '?' }));
const off = all.filter(r => r.list !== 'every tick').sort((a, b) => a.list.localeCompare(b.list) || b.games - a.games || a.q.localeCompare(b.q)), on = all.filter(r => r.list === 'every tick').sort((a, b) => b.games - a.games);
const line = (r: typeof all[number]) => has('--tsv') ? [r.q, r.list, r.games, r.looks, r.places, r.read, r.eg].join('\t') : `  ${r.list.padEnd(10)} ${r.q}  · ${r.games} of ${held} games, ${r.looks} of ${looks} looks${readers ? ` · ${r.read === 'read' ? 'READ outside the tick' : 'no reader listed'}` : ''} · e.g. ${r.eg}`;
console.log(`stale: ${games} games in the file, ${held} held to their Rust game in ${looks} looks · ${off.length} fields that are not sent every tick differ on the puppet (${off.filter(r => r.list === 'NO LIST').length} on no list, ${off.filter(r => r.list === 'START ONLY').length} sent at a start only) · ${on.length} fields on the every-tick list differ`);
for (const r of off) console.log(line(r));
if (has('--all')) { console.log('on the every-tick list and differing (the dress step, a tool\'s writes since the last apply, or an apply that lost something):'); for (const r of on) console.log(line(r)); }
for (const n of notes.slice(0, 12)) console.log(`  note: ${n}`);
process.exit(off.length ? 1 : 0);
