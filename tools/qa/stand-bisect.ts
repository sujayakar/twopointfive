// WHICH RULE TURNS A CHECK'S LINE RED. A check that is green on the frozen stand and red on the page's is red because of one of the rows the frozen stand holds
// off (src/host/profile.ts FROZEN_HOLDS), and usually because of one or two of them. This finds the fewest of those rows' words that flip the line, by raising
// subsets of them on the frozen stand (delta debugging: a dozen runs of the check for one word, a few dozen for a pair or three).
//   bun run tools/qa/stand-bisect.ts [--line REGEX] [--green] -- <command …>
//     the command        what runs the check, e.g.  bun run tools/qa/probes.ts orders-cover
//     --line REGEX       the verdict is one line's: red = a line that matches REGEX says FAIL (or carries a ✗). Without it, the verdict is the command's exit code
//     --green            the line is red on the plain frozen stand already: find the fewest words that turn it GREEN (a rule the page has that mends it)
//   e.g.  bun run tools/qa/stand-bisect.ts --line 'lights out: in the open' -- bun run tools/qa/probes.ts teams-dark     →  red with: TPS_IRLIGHT=1
// What the answer is for: the row's own check says what the rule does, and the line is then either a promise the rule ended (the line is mended to read the
// rule, or goes with the rule's off path) or a fault the rule brought (its owner's). A line that no set of words flips is not a held row's doing: the answer
// says so, with what the shipped stand itself makes of it (a row both stands write only as headless.ts loads, the kit, the cast).
// Exit 0 = found · 1 = nothing to find, or not a held row · 2 = usage.
import { resolve } from 'node:path';

const ROOT = resolve(import.meta.dir, '../..');
const argv = process.argv.slice(2), dash = argv.indexOf('--');
if (dash < 0 || dash === argv.length - 1) { console.error('usage: bun run tools/qa/stand-bisect.ts [--line REGEX] [--green] -- <command …>'); process.exit(2); }
const opts = argv.slice(0, dash), cmd = argv.slice(dash + 1), green = opts.includes('--green');
const lineRe = opts.includes('--line') ? new RegExp(opts[opts.indexOf('--line') + 1]) : null;

// the words: the first WORD=value of each row's note in FROZEN_HOLDS (a row whose note names no word of its own has none to raise)
const { FROZEN_HOLDS } = await import(`${ROOT}/src/host/profile.ts`);
const words: string[] = [];
for (const note of Object.values(FROZEN_HOLDS as Record<string, string>)) { const m = /^([A-Z][A-Z0-9_]*=[A-Za-z0-9-]+)/.exec(note); if (m && !words.includes(m[1])) words.push(m[1]); }

type Verdict = 'red' | 'green' | 'missing';
const memo = new Map<string, Verdict>(); let runs = 0;
function verdict(raised: string[], profile = 'frozen'): Verdict {
  const key = profile + ' ' + [...raised].sort().join(' '); const had = memo.get(key); if (had) return had;
  runs++;
  const env: Record<string, string> = { PATH: process.env.PATH ?? '', HOME: process.env.HOME ?? '', TPS_PROFILE: profile }; for (const w of raised) { const i = w.indexOf('='); env[w.slice(0, i)] = w.slice(i + 1); }
  const p = Bun.spawnSync(cmd, { cwd: ROOT, env, stdout: 'pipe', stderr: 'pipe' });
  let v: Verdict;
  if (!lineRe) v = p.exitCode === 0 ? 'green' : 'red';
  else { const hit = (p.stdout.toString() + p.stderr.toString()).split('\n').filter(l => lineRe.test(l)); v = !hit.length ? 'missing' : hit.some(l => /\bFAIL\b|✗/.test(l)) ? 'red' : 'green'; }   // (missing: the run died before the line, or the words changed what the check prints)
  memo.set(key, v); return v;
}
const flips = (raised: string[]): boolean => verdict(raised) === (green ? 'green' : 'red');

const plain = verdict([]), all = verdict(words);
console.log(`the plain frozen stand: ${plain} · with all ${words.length} words raised: ${all}`);
if (plain === 'missing') { console.log('the line is not printed on the frozen stand: nothing to bisect'); process.exit(1); }
if (plain === (green ? 'green' : 'red')) { console.log(`already ${plain} on the plain frozen stand: ${green ? 'nothing to find' : 'no held row explains it (--green finds a rule that mends it)'}`); process.exit(1); }
if (!flips(words)) { console.log(`raising every word does not flip it: not one of the held rows' doing (the shipped stand itself: ${verdict([], 'shipped')})`); process.exit(1); }

/** delta debugging: the smallest list every word of which is needed */
function ddmin(ws: string[]): string[] {
  let n = 2;
  while (ws.length >= 2) {
    const size = Math.max(1, Math.floor(ws.length / n)), parts: string[][] = []; for (let i = 0; i < ws.length; i += size) parts.push(ws.slice(i, i + size));
    let cut = false;
    for (const p of parts) if (flips(p)) { ws = p; n = 2; cut = true; break; }
    if (!cut) for (const p of parts) { const rest = ws.filter(w => !p.includes(w)); if (rest.length && flips(rest)) { ws = rest; n = Math.max(n - 1, 2); cut = true; break; } }
    if (!cut) { if (n >= ws.length) break; n = Math.min(ws.length, n * 2); }
  }
  return ws;
}
const found = ddmin(words);
const rowsOf = (w: string) => Object.entries(FROZEN_HOLDS as Record<string, string>).filter(([, note]) => note.startsWith(w)).map(([row]) => row).join(', ');
console.log(`${green ? 'green' : 'red'} with: ${found.join(' ')}   (${runs} runs of the check)`);
for (const w of found) console.log(`  ${w} raises ${rowsOf(w)}`);
process.exit(0);
