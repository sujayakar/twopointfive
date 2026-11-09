// THE JUDGE OF A LEVEL'S FILE, asked of the native program (rust/game_start: `levels judge`): the game's own reader (file.rs) and the rules a file can be held to by itself (faults.rs). No page
// carries it. What is stood on the built floor is not asked: the other questions of `levels`, tools/qa/level-lint.ts. The files there are, `gate` judges (its row `level file`).
import { homedir } from 'node:os';
import { join, resolve } from 'node:path';
import type { LevelData } from './levelData';

/** every number of a value that a file cannot say (a file would have null there) */
function oddNumbers(v: unknown, at: string, out: string[]): void {
  if (typeof v === 'number') { if (!Number.isFinite(v)) out.push(`${at}: ${v} is no number a file can say`); return; }
  if (Array.isArray(v)) v.forEach((x, i) => oddNumbers(x, `${at}[${i}]`, out));
  else if (v && typeof v === 'object') for (const [k, x] of Object.entries(v)) oddNumbers(x, at ? `${at}.${k}` : k, out);
}
/** EVERY FAULT OF THE LEVEL, a line each by its place in the file (none: it is a level): a value that is not what its place wants, a member nothing in the game reads, an id that points nowhere, a
 *  number out of its range. */
export function levelFaults(d: LevelData): string[] {
  const odd: string[] = []; oddNumbers(d, '', odd);
  if (odd.length) return odd;
  const p = Bun.spawnSync(['cargo', 'run', '--release', '--quiet', '--bin', 'levels', '--', 'judge'], {
    cwd: resolve(import.meta.dir, '../../../rust/game_start'), stdin: new TextEncoder().encode(JSON.stringify(d)), stdout: 'pipe', stderr: 'pipe', env: { ...process.env, PATH: `${join(homedir(), '.cargo/bin')}:${process.env.PATH ?? ''}` },
  });
  if (p.exitCode !== 0 && p.exitCode !== 1 || p.exitCode === 1 && !p.stdout.length) throw new Error(`the judge of a level's file did not run (${p.stderr.toString().trim().split('\n').pop() || 'no cargo'})`);
  return p.stdout.toString().split('\n').filter(l => l);
}
