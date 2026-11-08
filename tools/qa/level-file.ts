// IS A LEVEL'S FILE WHAT ITS GENERATOR GIVES NOW. One answer, the exporter's own: `bun run tools/levels/export.ts <name> --check` builds the level as it
// would write it, every step of the export after the generator (the doors fitted, the ways out, the floor swept, the word each box says it does, the
// dressing, the look twin beside it) and compares the text with the file, writing nothing. Three level checks (the registry's, the consulate's, the
// depot's) each built the generator's level themselves and walked those steps after it one by one, and each step the exporter gained turned all three red
// with the files right: the last was the word every box says it does, stamped at the export (175, 75 and 84 'diffs' on files the exporter calls up to
// date, on both stands). 
// LEVELFILE_BREAK=1 = the red path: the level is asked for without its dressing (`--bare`), which the file has, so the answer is STALE.
import { resolve } from 'node:path';
const ROOT = resolve(import.meta.dir, '../..');
export function exporterCheck(name: string): { ok: boolean; said: string } {
  const args = [name, '--check', ...(process.env.LEVELFILE_BREAK === '1' ? ['--bare'] : [])];
  const ex = Bun.spawnSync(['bun', 'run', resolve(ROOT, 'tools/levels/export.ts'), ...args], { cwd: ROOT, stdout: 'pipe', stderr: 'pipe', env: process.env });
  const lines = (new TextDecoder().decode(ex.stderr) + '\n' + new TextDecoder().decode(ex.stdout)).split('\n').map(l => l.trim()).filter(Boolean);
  const told = lines.filter(l => /^--check:|STALE|REFUSED|^usage:|[Ee]rror/.test(l)), said = (told.length ? told : lines.slice(-2)).join(' · ').replaceAll(ROOT + '/', '');
  return { ok: ex.exitCode === 0, said: `export.ts ${args.join(' ')}: exit ${ex.exitCode} · ${said.slice(0, 400)}` };
}
