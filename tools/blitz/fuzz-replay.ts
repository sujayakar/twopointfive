// A KEPT FUZZ RUN PLAYED AGAIN ON THE RUST GAME ALONE (tools/blitz/fuzz-replay.sh; PLAY.md, "Kept runs"). The kept folder holds the Rust side of a run that
// was equal to the TypeScript tick by every measure: a line a tick (`rust-*.txt`: the die's count, the host's half and the orders given, a hash of each
// part of the block) and a hash of the whole state every half minute (`rust-hashes.txt`). This holds a new play's folder to it and names the first tick
// whose line differs (which of the die, the host's half, the parts), the perturbations before it, and the first whole state whose hash differs.
//   bun run tools/blitz/fuzz-replay.ts <kept folder> <new folder> [--json]
// Exit 0: every tick's line and every whole state's hash equal, as many of each · 1: something differs · 3: a folder has no tick line.
import { readFileSync, readdirSync, existsSync } from 'node:fs';
const [kept, fresh] = process.argv.slice(2), asJson = process.argv.includes('--json');
if (!kept || !fresh) { console.error('fuzz-replay.ts <kept folder> <new folder> [--json]'); process.exit(64); }
const file = (d: string) => { const L = existsSync(d) ? readdirSync(d).filter(f => /^rust-.*\.txt$/.test(f) && !/-(say|half|hashes)\.txt$/.test(f)) : []; return L[0] ? `${d}/${L[0]}` : ''; };
// (a tick written twice is a tick a kept game stood again in: its last line counts)
const ticks = (f: string): string[] => { const out: string[] = []; if (!f) return out; for (const l of readFileSync(f, 'utf8').split('\n')) { if (!l) continue; const k = l.slice(0, l.indexOf(' ')); if (out.length && out[out.length - 1].startsWith(k + ' ')) out[out.length - 1] = l; else out.push(l); } return out; };
const A = ticks(file(kept)), B = ticks(file(fresh));
if (!A.length || !B.length) { console.log(`fuzz-replay: ${!A.length ? `the kept folder ${kept}` : `the new play ${fresh}`} has no tick line`); process.exit(3); }
const NAMES = ['head', 'dice', 'doors', 'falls', 'props', 'slabs', 'items', 'men', 'lights', 'weather', 'smoke', 'panes'];
let at = 0; const n = Math.min(A.length, B.length); while (at < n && A[at] === B[at]) at++;
const what: string[] = []; let tick = 0;
if (at < n) { const a = A[at].split(' '), b = B[at].split(' '); tick = +a[0]; if (a[1] !== b[1]) what.push(`the die (${a[1]} kept, ${b[1]} now)`); if (a[2] !== b[2]) what.push('the host\'s half or the orders given'); for (let j = 3; j < a.length; j++) if (a[j] !== b[j]) what.push(NAMES[j - 3] ?? `part ${j - 3}`); }
else if (A.length !== B.length) { tick = n + 1; what.push(`the kept run played ${A.length} ticks, this one ${B.length}`); }
const hashes = (d: string) => existsSync(`${d}/rust-hashes.txt`) ? readFileSync(`${d}/rust-hashes.txt`, 'utf8').split('\n').filter(Boolean) : [];
const HA = hashes(kept), HB = hashes(fresh); let hAt = 0; while (hAt < Math.min(HA.length, HB.length) && HA[hAt] === HB[hAt]) hAt++;
const whole = hAt < Math.max(HA.length, HB.length) ? +(HA[hAt] ?? HB[hAt]).split(' ')[0] : 0;
// the acts before the first tick that differs (the perturbations' or the soak's own lines; frame f is tick f + 1)
const log = existsSync(`${fresh}/rust.log`) ? readFileSync(`${fresh}/rust.log`, 'utf8').split('\n') : [], acts = tick ? log.map(l => l.match(/^\s*\[seed \d+\] ([\d.]+)s f(\d+) (.*)$/)).filter((m): m is RegExpMatchArray => !!m && +m[2] < tick && +m[2] >= tick - 600).slice(-4).map(m => `f${m[2]} ${m[3].slice(0, 160)}`) : [];
const out = { kept, fresh, ticks: { kept: A.length, now: B.length }, first: tick, what, wholeStates: { kept: HA.length, now: HB.length, firstDiffers: whole }, acts };
if (asJson) console.log(JSON.stringify(out));
else if (!tick && !whole) console.log(`fuzz-replay: THE SAME PLAY: all ${A.length} ticks' lines (the die, the host's half, the orders, 12 parts of the block) and all ${HA.length} whole states' hashes`);
else { console.log(`fuzz-replay: ${tick ? `THE FIRST TICK THAT DIFFERS IS ${tick}: ${what.join(', ')}` : `every tick's line is equal (${A.length})`}; ${whole ? `the first whole state that differs is tick ${whole}'s` : `the whole states' hashes are equal${tick ? ' up to there' : ''} (${Math.min(HA.length, HB.length)})`}`); for (const a of acts) console.log(`    before it: ${a}`); }
process.exit(tick || whole ? 1 : 0);
