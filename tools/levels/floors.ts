// A LEVEL'S FLOOR: a level dressed in the default look is two generators, `tools/levels/<name>-floor.ts` (what a man collides with, hides behind and sees
// through) and `tools/levels/<name>.ts` (the skin over it, tools/levels/gwotSkin.ts), and one file. The checks that hold the skin to "paint only" ask this
// for the file the floor alone would make: the exporter's own path with the skin left out (export.ts --floor), written to a folder of its own.
import { spawnSync } from 'node:child_process';
import { mkdtempSync, readdirSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join, resolve } from 'node:path';

export const floorLevels = (): string[] => readdirSync(import.meta.dir).filter(f => f.endsWith('-floor.ts')).map(f => f.replace(/-floor\.ts$/, '')).sort();

/** the floors of these levels, made now: the path of each one's file. They go when the process does. */
export function floorFiles(names: string[] = floorLevels()): (name: string) => string {
  const dir = mkdtempSync(join(tmpdir(), 'floors-')); process.on('exit', () => rmSync(dir, { recursive: true, force: true }));
  for (const n of names) {
    const p = spawnSync('bun', ['run', resolve(import.meta.dir, 'export.ts'), n, '--floor', dir], { encoding: 'utf8' });
    if (p.status !== 0) throw new Error(`the floor of ${n} did not export: ${p.stderr || p.stdout}`);
  }
  return name => join(dir, `${name}.json`);
}
