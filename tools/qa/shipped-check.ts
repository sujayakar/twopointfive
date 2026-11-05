#!/usr/bin/env bun
// THE PAGE'S STAND AND THE OLD ONE, CHECKED WITHOUT A PAGE (src/host/profile.ts) — `bun run tools/qa/shipped-check.ts`, and `probes.ts shipped-profile` (family shipped):
//   1. the harness stood up frozen and stood up shipped (TPS_PROFILE=shipped) differ in exactly the rows FROZEN_HOLDS names — a pin added to the harness and not
//      to that list, or a row listed that no longer differs, is red, and says which;
//   2. the same on a mission's own stand (a squad of four started on a level file), so a rule that only a staging writes is seen too;
//   3. the play-through's own as-shipped lines (playthrough.ts: the doors, the chaos) write what the shipped stand already has — under the shipped profile the
//      play-through adds nothing of its own;
//   4. the stand's NAME: no word and 'shipped' are the page's stand, 'frozen' the old one, any other word is refused (src/host/standEnv.ts standOf, the
//      one rule), and the harness's own reading of the environment (profile.ts harnessProfile) says the same;
//   5. every tool that reads a `.TPS_PROFILE` loads tools/qa/stand.ts before its own lines, or names its stand itself first (earlyReaders below).
// STAND_BREAK=1 = the red path of 4 and 5: no word is expected to be the old stand, and stand.ts is forgotten (every reader is early).
// Each stand is its own bun (the pins run as headless.ts loads), with every word either stand reads cleared from its environment first.
// That the SHIPPED stand is the page's is the other half, and needs the page: `node tools/qa/shipped-page.cjs` (software adapter, ~3 minutes a boot).
// Exit 0 = all pass, 1 = a check failed.
import { dirname, join, resolve } from 'node:path';
import { existsSync, mkdtempSync, readdirSync, readFileSync, rmSync, statSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { dumpDiff, type StandDump } from './stand-dump.ts';
import './stand.ts';   // (the stand's word, written down before this file reads it)
import { standOf } from '../../src/host/standEnv.ts';
const ROOT = resolve(import.meta.dir, '../..'), BREAK = process.env.STAND_BREAK === '1';
const MISSION = 'levelFile=compound.json&roomclear&squad4';

/** THE TOOLS THAT READ THE STAND'S WORD BEFORE IT IS WRITTEN: a tool's file that reads a `.TPS_PROFILE` (off `process.env` or off any other name for it) and
 *  neither loads stand.ts first (a chain of STATIC imports to it: those load before the file's own lines; headless.ts is such a chain; `import type` loads
 *  nothing) nor names its stand itself before it reads (its first mention is a write: `… ??= 'shipped'`, `… = 'shipped'`). Such a tool, run with no word,
 *  would call the page's stand "frozen". A .cjs page driver cannot load stand.ts, so one that reads the word is named whatever it imports. Also named: a
 *  file that DOES load stand.ts and then writes `??= …'frozen'…`, which can no longer take. STAND_BREAK=1 forgets stand.ts, so every reader is one.
 *  WHAT IT CANNOT SEE: it reads text. A tool that clears the word for a child, or gives a child an environment without it, asks for the page's stand
 *  and is not seen here: a tool that wants the old stand of a child says 'frozen'. */
export function earlyReaders(root = ROOT): string[] {
  const walk = (d: string, out: string[] = []): string[] => { for (const n of readdirSync(d)) { const p = join(d, n); if (statSync(p).isDirectory()) { if (!/^(fixtures?|node_modules)$/.test(n)) walk(p, out); } else if (/\.(ts|cjs|mjs|js)$/.test(n)) out.push(p); } return out; };   // (a fixture folder is data: a tree a tool is tried on)
  const code = (f: string) => readFileSync(f, 'utf8').split('\n').filter(l => !/^\s*(\/\/|\*|\/\*)/.test(l)).join('\n');
  const imports = (f: string): string[] => [...code(f).matchAll(/^\s*(?:import|export)\s(?!type\b)(?:[^;'"]*?\sfrom\s+)?['"](\.[^'"]+)['"]/gm)].map(m => resolve(dirname(f), m[1])).filter(existsSync);
  const STANDFILE = resolve(root, 'tools/qa/stand.ts'), memo = new Map<string, boolean>();
  const loads = (f: string, seen = new Set<string>()): boolean => { if (f === STANDFILE) return !BREAK; if (!f.endsWith('.ts')) return false; if (memo.has(f)) return memo.get(f)!; if (seen.has(f)) return false; seen.add(f); const r = imports(f).some(i => loads(i, seen)); memo.set(f, r); return r; };
  const READ = /[\w\])]\??\.TPS_PROFILE\b|\[['"]TPS_PROFILE['"]\]/, NAMED = /^[\w\])]\??\.TPS_PROFILE\s*(\?\?=|=(?!=))/, LATE = /\.TPS_PROFILE\s*\?\?=[^;\n]*'frozen'/;
  return walk(resolve(root, 'tools')).sort().filter(f => { if (f === STANDFILE) return false; const c = code(f), at = c.search(READ); if (at < 0) return false;
    return loads(f) ? LATE.test(c) : !NAMED.test(c.slice(at)); }).map(f => f.slice(root.length + 1));
}

async function stand(profile: 'frozen' | 'shipped', query: string, dir: string): Promise<StandDump> {
  const out = resolve(dir, `${profile}-${query ? 'mission' : 'plain'}.json`);
  const pr = Bun.spawn(['bun', 'run', resolve(import.meta.dir, 'stand-dump.ts'), '--profile', profile, '--query', query, '--out', out], { stdout: 'ignore', stderr: 'pipe' });   // (--profile clears the invoking shell's stand words in the child)
  const [err, code] = await Promise.all([new Response(pr.stderr).text(), pr.exited]);
  if (code !== 0) throw new Error(`stand-dump (${profile}${query ? `, ${query}` : ''}) failed: ${err.slice(-600)}`);
  return JSON.parse(readFileSync(out, 'utf8')) as StandDump;
}

export async function shippedCheck(): Promise<boolean> {
  const { FROZEN_HOLDS, STAND_ENV, harnessProfile } = await import(`${ROOT}/src/host/profile.ts`) as { FROZEN_HOLDS: Record<string, string>; STAND_ENV: readonly string[]; harnessProfile: () => string };
  let ok = true; const check = (pass: boolean, what: string) => { console.log(`- ${pass ? 'PASS' : 'FAIL'} — ${what}`); if (!pass) ok = false; };
  const dir = mkdtempSync(resolve(tmpdir(), 'shipped-check-'));
  try {
    const listed = Object.keys(FROZEN_HOLDS).sort();
    for (const [name, query] of [['the plain stand', ''], ['a mission staged (the compound, four)', MISSION]] as const) {
      const [fr, sh] = await Promise.all([stand('frozen', query, dir), stand('shipped', query, dir)]);
      const rows = dumpDiff(fr, sh), got = rows.map(r => r[0]).sort();
      const unlisted = got.filter(k => !listed.includes(k)), stale = listed.filter(k => !got.includes(k));
      check(unlisted.length === 0 && stale.length === 0, `${name}: frozen and shipped differ in exactly FROZEN_HOLDS' ${listed.length} rows`
        + (unlisted.length ? ` — the harness holds these and the list does not name them: ${unlisted.map(k => { const r = rows.find(x => x[0] === k)!; return `${k} (${JSON.stringify(r[1])} frozen, ${JSON.stringify(r[2])} shipped)`; }).join(' · ')}` : '')
        + (stale.length ? ` — listed, and the two stands agree on them: ${stale.join(' · ')}` : ''));
      if (!query) {
        const T = sh.tables as Record<string, Record<string, unknown>>, tune = sh.tune as Record<string, unknown>;
        const want: [string, unknown, unknown][] = [['DOORWAY.oneWay', T.DOORWAY?.oneWay, true], ['DOORWAY.exitsOut', T.DOORWAY?.exitsOut, false], ['DOORWAY.frameHolds', T.DOORWAY?.frameHolds, true],
          ['DOORSTAY.on', T.DOORSTAY?.on, true], ['DOORMANNERS.on', T.DOORMANNERS?.on, true], ['DOORBLAST.on', T.DOORBLAST?.on, true], ['tune.chaos', tune.chaos, 0.5]];
        const off = want.filter(w => w[1] !== w[2]);
        check(off.length === 0, `the play-through's own as-shipped lines write what the shipped stand already has (${want.length} rows)${off.length ? ` — not so: ${off.map(w => `${w[0]} is ${JSON.stringify(w[1])}, the play-through writes ${JSON.stringify(w[2])}`).join(' · ')}` : ''}`);
      }
    }
    { // every word a harness pin answers to is on the list the comparisons clear (src/host/standEnv.ts) — else a stand compared here is the invoking shell's
      const words = new Set<string>(); for (const f of ['src/host/standup.ts', 'tools/qa/headless.ts', 'tools/qa/playthrough.ts']) for (const m of readFileSync(resolve(ROOT, f), 'utf8').matchAll(/pinned\('([A-Z_]+)'\)/g)) words.add(m[1]);
      const missing = [...words].filter(w => !STAND_ENV.includes(w));
      check(words.size > 0 && missing.length === 0, `every word a harness pin answers to (${words.size}: ${[...words].sort().join(' ')}) is in STAND_ENV${missing.length ? ` — missing: ${missing.join(' ')}` : ''}`); }
    { // the stand's name: the one rule (standEnv.ts standOf), and the harness's reading of the environment is that rule
      const env = process.env, was = env.TPS_PROFILE, said = (f: () => string): string => { try { return f(); } catch { return 'refused'; } };
      const rows = (['', 'shipped', 'frozen', 'Shipped', 'froze', '1'] as const).map(w => { if (w) env.TPS_PROFILE = w; else delete env.TPS_PROFILE; return [w || '(no word)', said(() => standOf(w || undefined)), said(() => harnessProfile())] as const; });
      if (was === undefined) delete env.TPS_PROFILE; else env.TPS_PROFILE = was;
      const want = [BREAK ? 'frozen' : 'shipped', 'shipped', 'frozen', 'refused', 'refused', 'refused'], off = rows.filter((r, i) => r[1] !== want[i] || r[2] !== want[i]);
      check(off.length === 0, `no word and 'shipped' are one stand, the page's; 'frozen' is the old one; any other word is refused${off.length ? ` — not so: ${off.map(r => `${r[0]}: the rule says ${r[1]}, the harness ${r[2]}`).join(' · ')}` : ''}`); }
    { const early = earlyReaders();
      check(early.length === 0, `every tool that reads the stand's word loads stand.ts before its own lines (a static import, or headless.ts), or names its stand first${early.length ? ` — not so (${early.length}): ${early.join(' · ')}` : ''}`); }
    console.log(`  (the rows, and the word that turns each on for a frozen run:\n${listed.map(k => `     ${k.padEnd(24)} ${FROZEN_HOLDS[k]}`).join('\n')})`);
  } finally { rmSync(dir, { recursive: true, force: true }); }
  return ok;
}

if (import.meta.main) process.exit(await shippedCheck() ? 0 : 1);
