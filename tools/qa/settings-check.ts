// THE PICTURE'S SETTINGS ARE NOT THE GAME'S (src/game/env.ts). The engine's settings are the picture's: a player moves them (Brightness, the bracket keys, his
// quality tier, a panel's slider), two players of one game have two sets, a server has its own, and a recording carries four of them. A sum of the
// simulation that reads one is a run that turns on whose machine it is played on.
//   bun run tools/qa/settings-check.ts                    every line below; exit 0 = all pass · 1 = a line failed · 2 = a run could not be made
//   bun run tools/qa/settings-check.ts --static           `base` and `reads` alone (no game is stood: a second) — `probes.ts settings-reads`, family journal
//   SETTINGS_BREAK=1 bun run tools/qa/settings-check.ts   the red path: the list below loses its first file's rows, and the turned runs turn the GAME'S OWN
//                                                         numbers where they should turn the picture's — `reads` and `one run` must FAIL
// THE LINES:
//   base      the game's own numbers start where the picture's defaults are (env.ts ENV against src/settings.ts): one model for the picture and the men
//   reads     no file of the simulation (src/game, scene, anim, math, sim and render/reach.ts: rollback-check's ground) READS a setting of the engine's
//             outside the rows of KNOWN below (file · key · how many · why). A read more than a row counts is named with its lines, and a row that
//             counts more than there are is named too, so the list only ever shrinks. What the scan reads: `…settings.key`, a cast's
//             `(…settings as T).key`, `alias.key` where `alias = …engine.settings` (to the end of the block it is declared in), a destructuring off
//             the settings. A write (`= `, `+=`, `++`) is not a read: the game tells the picture. Any other use of the engine's settings as a value
//             (handed on whole to a call, indexed by a variable) is named as `*` and needs a row. It is a scan of the text, not of the types: what a
//             helper in another folder reads of settings it was handed is not seen (the `*` at the call that hands them over is).
//   hour      (a game stood here, on this stand) the game's exposure is the night's with the picture's turned to a quarter; under an hour it is the hour's
//             own, and the picture was told the same number; a turn of the picture under the hour moves the picture's alone; the show's day gives the
//             day's; and each way back gives the night's again (hours.ts exposureOf, against what putIn and setDaylight write)
//   one run   ONE scripted night mission with a squad in goggles (barracks-four, seed 1, the shipped stand: the frozen one has no goggles on anybody) played
//             plain and with the PICTURE turned three ways (QA_PICTURE, tools/qa/headless.ts: Brightness at each end of its slider, the tube's gain
//             cut): the same digest, end and second. And the line can go red: the same mission with the GAME'S OWN exposure turned (--set ENV) is
//             another run.
// Not game code: it reads files, stands one game and starts play-throughs.
import { readdirSync, readFileSync, mkdtempSync } from 'node:fs';
import { resolve, join, relative } from 'node:path';
import { tmpdir } from 'node:os';

const ROOT = resolve(import.meta.dir, '../..'), BREAK = process.env.SETTINGS_BREAK === '1';
let fails = 0; const line = (ok: boolean, what: string) => { console.log(`- ${ok ? 'PASS' : 'FAIL'} — ${what}`); if (!ok) fails++; };

/** THE READS THAT STAND TODAY: `file under src` → key → [how many reads, why they may stay or what moves them]. Three kinds:
 *    PICTURE   the read ends in the picture (a draw, a light the renderer alone is handed, a panel's line, a value kept to hand back to the picture)
 *    CARRIED   a recipe and a recording carry the setting (journal.ts EnvSnap) and a networked page lays the server's over its own before every tick
 *    TO MOVE   the simulation reads it and nothing carries it: src/game/env.ts is where it goes (dated; the owner named)
 *  A row holds a COUNT: one more read of a key a file already has a row for is red like a new key (a sum reading `exposure` again in hours.ts would
 *  otherwise hide under the row that lets the picture's bookkeeping read it), and one fewer is red until the count is lowered. Key `*` = uses the scan
 *  cannot read (the settings indexed by a variable, handed on whole); key `(all)` = every use in the file, counted together, for a file that is all of
 *  one kind. A row is a debt or a reason, never a habit: adding one, or raising a count, needs the same sentence.
 *  WHO EMPTIES IT: the multiplayer goal owns every TO MOVE and TO JUDGE row no other owner is named on, a mend each on the pattern of env.ts. */
type Row = [count: number, why: string];
const KNOWN: Record<string, Record<string, Row>> = {
  'game/hours.ts': {
    exposure: [3, 'PICTURE: what the hour found is kept (putIn) and handed back to the picture with the player\'s turn of Brightness (takeOut); the panel\'s line prints it'],
    hazeDensity: [1, 'PICTURE: kept and handed back when the hour goes'],
    skyZenith: [2, 'one PICTURE (kept, handed back), one TO MOVE (2026-11-05, with the hour\'s port, weather\'s owner): skyLum reads the sky\'s colours for the light on a point'],
    skyHorizon: [2, 'as skyZenith'],
  },
  'game/lighting.ts': {
    exposure: [1, 'PICTURE: what the show\'s day found, kept and handed back'],
    hazeDensity: [1, 'PICTURE: as exposure'], skyZenith: [1, 'PICTURE: kept and handed back (the read that reaches a sum is hours.ts\'s)'], skyHorizon: [1, 'PICTURE: as skyZenith'],
    emissiveScale: [1, 'CARRIED, TO MOVE (2026-11-05, after the lights\' port): what a fixture gives a light question'],
  },
  'game/fogofwar.ts': { emissiveScale: [2, 'CARRIED, TO MOVE (2026-11-05, with lighting.ts\'s): the fog\'s lamps'] },
  'game/guards.ts': { flashlightRadius: [3, 'CARRIED, TO MOVE (2026-11-05): a torch\'s radius, part of the light on a point'] },
  'game/player.ts': { flashlightRadius: [1, 'CARRIED, TO MOVE (2026-11-05): as guards.ts'] },
  'game/debris.ts': { debrisCap: [1, 'CARRIED, TO MOVE (2026-11-05): the pool\'s size; with DEBRIS.dress on the pool is dressing'] },
  'game/journal.ts': { '(all)': [5, 'CARRIED: this is the carrying (envSnapshot reads the settings a file keeps, applyEnvSettings lays them back)'] },
  'game/irlight.ts': { indirectScale: [1, 'TO MOVE (2026-11-05, the senses\' owner has taken it): the share of bounced light in what a man\'s fan calls dark floor'] },
  'game/fire.ts': { fireSmoke: [1, 'TO JUDGE (2026-11-05): whether the fire\'s smoke is drawn by the solver, on a line that also reads the picture\'s camera'] },
  'game/weather.ts': { gloss: [3, 'PICTURE: the wet ground\'s gloss, kept and handed back'] },
  'game/game.ts': {
    decals: [1, 'TO JUDGE (2026-11-05): whether the decal layer runs; the digest counts decals'],
    markings: [1, 'CARRIED: the floor markings\' lights'], markingTexel: [1, 'PICTURE: the markings\' texture'],
    debugView: [1, 'PICTURE: a debug view of the draw'], meshShadows: [1, 'PICTURE: the draw block'], rtMeshHits: [1, 'PICTURE: the draw block'], sdfRigSmooth: [1, 'PICTURE: the draw block'], sharpDecouple: [1, 'PICTURE: the draw block'],
    wetChars: [1, 'PICTURE: the draw block (whether a wet man is shaded wet)'],
  },
  'game/irlaser.ts': { debugView: [1, 'PICTURE: a debug view of the draw'] },
  'game/charLook.ts': { charMats: [1, 'PICTURE: the men\'s materials'] },
  'game/particleFx.ts': { gpuParticles: [1, 'PICTURE: whether the particle pass exists'] },
  'game/snowFx.ts': { '(all)': [7, 'PICTURE: the flakes that are drawn (the particle rain\'s own settings: count, radius, how it answers a blast)'] },
  'game/rainFx.ts': { '(all)': [9, 'PICTURE: the drops that are drawn (the particle rain\'s own settings)'] },
  'game/airSim.ts': { airSim: [1, 'TO JUDGE (2026-11-05): whether the air field runs; AIR_SIM decides what play reads of it'] },
};
/** the simulation's ground, as tools/qa/rollback-check.ts's static pass has it */
const SIM_DIRS = ['src/game', 'src/scene', 'src/anim', 'src/math', 'src/sim'], SIM_FILES = ['src/render/reach.ts'];
if (BREAK) delete KNOWN[Object.keys(KNOWN)[0]];

/** the code of a file with every comment and string blanked (same length, same lines); a template's `${…}` is code and stays */
export function codeOf(src: string): string {
  const out = src.split(''), n = src.length; let i = 0;
  const blank = (a: number, b: number) => { for (let k = a; k < b; k++) if (out[k] !== '\n') out[k] = ' '; };
  const quote = (q: string) => { const a = i++; while (i < n && src[i] !== q && src[i] !== '\n') i += src[i] === '\\' ? 2 : 1; i++; blank(a + 1, i - 1); };
  function code(until: string | null): void {
    while (i < n) {
      const c = src[i], d = src[i + 1];
      if (until && c === until) return;
      if (c === '/' && d === '/') { const a = i; while (i < n && src[i] !== '\n') i++; blank(a, i); }
      else if (c === '/' && d === '*') { const a = i; i = src.indexOf('*/', i + 2); i = i < 0 ? n : i + 2; blank(a, i); }
      else if (c === '\'' || c === '"') quote(c);
      else if (c === '`') { i++; let a = i;
        while (i < n && src[i] !== '`') { if (src[i] === '\\') i += 2; else if (src[i] === '$' && src[i + 1] === '{') { blank(a, i); i += 2; code('}'); i++; a = i; } else i++; }
        blank(a, i); i++; }
      else if (c === '{') { i++; code('}'); i++; }
      else i++;
    }
  }
  code(null); return out.join('');
}
export interface Use { file: string; key: string; at: number }
/** `text` past a leading ` as <type>`, the type read to its end at the first `,` `;` or `)` outside every bracket of its own (no cast: `text` as it is) */
function afterType(text: string): string {
  const m = /^\s+as\s+/.exec(text); if (!m) return text;
  let depth = 0; for (let k = m[0].length; k < text.length; k++) { const c = text[k]; if (c === '{' || c === '<' || c === '(' || c === '[') depth++; else if (c === '}' || c === '>' || c === ']') depth--; else if (c === ')') { if (depth === 0) return text.slice(k); depth--; } else if ((c === ',' || c === ';') && depth === 0) return text.slice(k); }
  return '';
}
/** every read of a setting in one file's text (`keys`: the settings there are), and every use of the settings this scan cannot read (`*`) */
export function usesOf(file: string, src: string, keys: Set<string>): Use[] {
  const C = codeOf(src), found = new Map<string, Use>(), lineOf = (p: number) => C.slice(0, p).split('\n').length;
  const isWrite = (p: number) => /^\s*(=(?!=)|\+=|-=|\*=|\/=|\+\+|--)/.test(C.slice(p, p + 6));
  const put = (key: string, p: number) => { if (key !== '*' && (!keys.has(key) || isWrite(p))) return; found.set(`${key}@${p}`, { file, key, at: lineOf(p) }); };
  for (const m of C.matchAll(/\bsettings\b/g)) {
    const p = m.index!, before = C.slice(Math.max(0, p - 200), p), after = C.slice(p + 8, p + 200);
    const direct = /^\??\.\s*([A-Za-z_]\w*)/.exec(after); if (direct) { put(direct[1], p + 8 + direct[0].length); continue; }   // …settings.key
    const cast = /^\s+as\s+[^();]*\)\??\.\s*([A-Za-z_]\w*)/.exec(after); if (cast) { put(cast[1], p + 8 + cast[0].length); continue; }   // (…settings as T).key
    if (/^\s*\??\s*:/.test(after) || !/(\.|\bengine\??\.)\s*$/.test(before)) continue;   // (a member's declaration in a type, a parameter of that name: not the engine's settings as a value)
    if (/^\s*(&&|\|\||\?\?|[!=]==?\s*(undefined|null)\b)/.test(after) || /(!|\btypeof\s)\s*[\w$.?()\s]*$/.test(before.slice(-60).split(/[;,{(]/).pop()!)) continue;   // (asked only whether they are there)
    const bare = /^\s*[,;]/.test(afterType(after));   // the whole of them, bound to a name (a cast to a type between is passed over)
    const destr = bare ? /\{([^{}]*)\}\s*=\s*[^=;,]*$/.exec(before) : null;
    if (destr) { for (const name of destr[1].split(',')) { const k = name.split(':')[0].trim(); if (k) put(k, p); } continue; }
    const alias = bare ? /(?:^|[\s,({;])([A-Za-z_$][\w$]*)\s*(?::[^=,;{}]+)?=\s*[^=;,]*$/.exec(before) : null;
    if (alias) {   // `S = g.engine.settings`: every `S.key` to the end of the block
      let depth = 0, end = C.length; for (let k = p; k < C.length; k++) { if (C[k] === '{') depth++; else if (C[k] === '}' && --depth < 0) { end = k; break; } }
      const A = alias[1].replace(/\$/g, '\\$'), body = C.slice(p, end);
      for (const u of body.matchAll(new RegExp(`(?<![\\w$.])${A}\\??\\.\\s*([A-Za-z_]\\w*)`, 'g'))) put(u[1], p + u.index! + u[0].length);
      for (const u of body.matchAll(new RegExp(`(?<![\\w$.])${A}(?![\\w$])(?!\\s*\\??\\.\\s*[A-Za-z_])(?!\\s*(?::[^=,;{}]+)?=[^=])`, 'g'))) put('*', p + u.index!);   // (the alias itself as a value: indexed by a variable, handed on)
      continue;
    }
    put('*', p);   // handed on whole, indexed by a variable, tested and passed along: this scan cannot say what is read of them
  }
  return [...found.values()];
}
const walk = (dir: string): string[] => readdirSync(dir, { withFileTypes: true }).flatMap(e => e.isDirectory() ? walk(join(dir, e.name)) : e.name.endsWith('.ts') ? [join(dir, e.name)] : []);

/** `base` and `reads`: no game is stood (`probes.ts settings-reads`) */
export async function settingsReads(): Promise<boolean> {
  const before = fails;
  const { ENV } = await import(`${ROOT}/src/game/env.ts`), { defaultSettings } = await import(`${ROOT}/src/settings.ts`);
  const D = defaultSettings() as Record<string, unknown>, keys = new Set(Object.keys(D));
  line(ENV.exposure === D.exposure && ENV.tubeGain === D.nvGain, `base: the game's own numbers are the picture's defaults (exposure ${ENV.exposure} against ${D.exposure}, tube gain ${ENV.tubeGain} against ${D.nvGain})`);
  // (the scan against forms it must read and forms it must not: a lint that reads nothing is green for ever)
  const T = (src: string) => usesOf('t.ts', src, keys).map(u => u.key).sort().join(' ');
  const forms: [string, string][] = [
    ['function f(g) { return g.engine.settings.exposure * 2; }', 'exposure'], ['function f(g) { g.engine.settings.exposure = 2; g.engine.settings.exposure *= 2; }', ''],
    ['function f(g) { const S = g.engine.settings; S.nvGain = 1; return S.exposure; } function h(S) { return S.nvGain; }', 'exposure'],
    ['function f(g) { const S = g.engine.settings as T; const q = foo(1).exposure; return S.nvGain; }', 'nvGain'],
    ['function f(g) { const { exposure, nvGain: gg } = g.engine.settings; return exposure + gg; }', 'exposure nvGain'],
    ['function f(g) { return (g.engine.settings as { nvGain?: number }).nvGain; }', 'nvGain'], ['function f(g) { return helper(g.engine.settings); }', '*'],
    ['function f(g) { const k = "exposure"; return g.engine.settings[k]; }', '*'], ['function f(g) { return `${g.engine.settings.exposure}`; } // g.engine.settings.nvGain', 'exposure'],
    ['function f(e: { settings?: { nvGain: number } }) { const settings = 1; return settings; }', ''],
    ['function f(g) { const S = g.engine.settings as { nvGain?: number; exposure: number }; return S.nvGain; }', 'nvGain'], ['function f(g, k) { const S = g.engine.settings as Record<string, unknown>; return [S[k], S.exposure, S.exposure]; }', '* exposure exposure'], ['function f(e) { if (e.engine.settings && e.engine.settings.exposure === 2) return 1; }', 'exposure'],
  ];
  const wrong = forms.filter(([src, want]) => T(src) !== want);
  line(wrong.length === 0, `the scan reads its own forms: ${forms.length - wrong.length} of ${forms.length}${wrong.length ? ` — WRONG: ${wrong.map(([src, want]) => `"${src}" read as [${T(src)}], wanted [${want}]`).join(' · ')}` : ''}`);
  const SRC = join(ROOT, 'src'), files = [...SIM_DIRS.flatMap(d => walk(join(ROOT, d))), ...SIM_FILES.map(f => join(ROOT, f))];
  const uses = files.flatMap(f => usesOf(relative(SRC, f).split('\\').join('/'), readFileSync(f, 'utf8'), keys));
  // (a file's uses by the row that answers for them: its `(all)` row if it has one, else the key's)
  const got = new Map<string, number[]>(); for (const u of uses) { const k = `${u.file} ${KNOWN[u.file]?.['(all)'] ? '(all)' : u.key}`; got.set(k, [...(got.get(k) ?? []), u.at]); }
  const rows = Object.entries(KNOWN).flatMap(([f, r]) => Object.entries(r).map(([k, [n]]) => [`${f} ${k}`, n] as const)), want = new Map(rows);
  const more = [...got].filter(([k, at]) => at.length > (want.get(k) ?? 0)), fewer = rows.filter(([k, n]) => (got.get(k)?.length ?? 0) < n);
  line(more.length === 0 && fewer.length === 0, `reads: ${uses.length} reads of the picture's settings in ${files.length} files of the simulation (${SIM_DIRS.join(', ')}, ${SIM_FILES.join(', ')}), each counted on a list of ${rows.length} rows`
    + (more.length ? `\n    NEW READS (the simulation may not read the picture's settings: src/game/env.ts says where a number of the game's own goes): ${more.map(([k, at]) => `${k}: ${at.length} at line ${at.join(', ')}${want.has(k) ? ` (its row allows ${want.get(k)})` : ' (no row)'}`).join(' · ')}` : '')
    + (fewer.length ? `\n    FEWER READS THAN A ROW SAYS (lower its count, or take the row off at none): ${fewer.map(([k, n]) => `${k}: ${got.get(k)?.length ?? 0} of ${n}`).join(' · ')}` : ''));
  return fails === before;
}

/** `hour`: one game stood here, its exposure against the hour, the show's day and a turned picture */
async function hourLine(): Promise<void> {
  const { standUp } = await import('./headless.ts'), { ENV } = await import(`${ROOT}/src/game/env.ts`);
  const { exposureOf, applyHour, HOURS } = await import(`${ROOT}/src/game/hours.ts`), { setDaylight } = await import(`${ROOT}/src/game/lighting.ts`);
  const W = await standUp({}), g = W.game, S = g.engine.settings as { exposure: number }, said: string[] = []; let ok = true;
  const hold = (what: string, got: number, want: number) => { if (got !== want) ok = false; said.push(`${what} ${got}${got === want ? '' : ` (WANTED ${want})`}`); };
  applyHour(g, 'night'); const night = S.exposure;
  S.exposure = night * 0.25; hold('the night\'s, the picture at a quarter:', exposureOf(g), ENV.exposure); S.exposure = night;
  applyHour(g, 'midday'); hold('at midday:', exposureOf(g), HOURS.midday.exposure); hold('the picture was told', S.exposure, HOURS.midday.exposure);
  S.exposure *= 0.5; hold('the picture halved under the hour:', exposureOf(g), HOURS.midday.exposure);
  applyHour(g, 'night'); hold('the night back:', exposureOf(g), ENV.exposure); S.exposure = night;
  const D = g.level.daylight as { exposure: number } | undefined;
  if (D) { setDaylight(g, true); hold('the show\'s day:', exposureOf(g), D.exposure); S.exposure *= 0.5; hold('the picture halved under it:', exposureOf(g), D.exposure); setDaylight(g, false); hold('the night after it:', exposureOf(g), ENV.exposure); }
  else { ok = false; said.push('THIS LEVEL HAS NO DAY: the show\'s half was not asked'); }
  line(ok, `hour: the game's exposure follows what the game keeps and never the picture's — ${said.join(' · ')}`);
}

/** `one run`: play-throughs of one night mission, the picture turned and the game's own turned */
async function oneRun(): Promise<void> {
  interface Run { digest?: string; end?: string; why?: string; t?: number }
  const DIR = mkdtempSync(join(tmpdir(), 'settings-check-')), SCN = 'barracks-four:1';
  async function play(name: string, picture: object | null, own: object | null): Promise<Run | null> {
    const out = join(DIR, `${name}.json`), env: Record<string, string> = { ...process.env as Record<string, string>, TPS_PROFILE: 'shipped', PT_DIGEST: '1' };
    delete env.QA_PICTURE; if (picture) env.QA_PICTURE = JSON.stringify(picture);
    const p = Bun.spawn([process.execPath, 'run', 'tools/qa/playthrough.ts', '--one', SCN, '--out', out, ...(own ? ['--set', JSON.stringify({ ENV: own })] : [])], { cwd: ROOT, env, stdout: 'ignore', stderr: 'pipe' });
    const [err, code] = await Promise.all([new Response(p.stderr).text(), p.exited]);
    let r: Run | null = null; try { const j = JSON.parse(readFileSync(out, 'utf8')); r = (Array.isArray(j) ? j[0] : j) as Run; } catch { /* no file */ }
    if (!r || r.end === 'error' || !r.digest) { console.log(`  (the ${name} run made no result, exit ${code}: ${(r?.why ?? err).trim().split('\n').slice(0, 2).join(' / ')})`); return null; }
    return r;
  }
  const said = (r: Run | null) => r ? `${r.digest} (${r.end ?? 'no end'} at ${r.t} s)` : 'no run';
  // (Brightness at the two ends of its slider — Options > Display, 0.5 to 2.5 — and the tube's gain cut with it: under the break the same numbers go to the game's own)
  const TURNS: [string, { exposure?: number; nvGain?: number }][] = [['Brightness down', { exposure: 0.5 }], ['Brightness up', { exposure: 2.5 }], ['the tube\'s gain cut', { exposure: 2.5, nvGain: 100 }]];
  const ownOf = (t: { exposure?: number; nvGain?: number }) => ({ ...(t.exposure !== undefined ? { exposure: t.exposure } : {}), ...(t.nvGain !== undefined ? { tubeGain: t.nvGain } : {}) });
  const [plain, other, ...turned] = await Promise.all([play('plain', null, null), play('own', null, { exposure: 0.5 }), ...TURNS.map(([, t], i) => BREAK ? play(`turn${i}`, null, ownOf(t)) : play(`turn${i}`, t, null))]);
  if (!plain || !other || turned.some(r => !r)) { console.log('settings-check: a run could not be made'); process.exit(2); }
  const same = (a: Run, b: Run) => a.digest === b.digest && a.t === b.t && a.end === b.end;
  line(turned.every(r => same(plain, r!)) && !same(plain, other),
    `one run: ${SCN} on the shipped stand, plain ${said(plain)}; with ${BREAK ? 'THE GAME\'S OWN numbers' : 'the picture'} turned: ${TURNS.map(([n], i) => `${n} ${same(plain, turned[i]!) ? 'the same run' : `ANOTHER RUN ${said(turned[i])}`}`).join(' · ')}`
    + `; and with the game's own exposure at 0.5 ${same(plain, other) ? 'STILL THE SAME RUN (this mission no longer turns on the tube: the line above cannot go red, pick another)' : `another run, ${said(other)}`}`);
}

if (import.meta.main) {
  await settingsReads();
  if (!process.argv.includes('--static')) { await hourLine(); await oneRun(); }
  console.log(fails ? `settings-check: ${fails} line${fails === 1 ? '' : 's'} FAILED${BREAK ? ' (SETTINGS_BREAK=1: `reads` and `one run` must)' : ''}` : 'settings-check: every line passed');
  process.exitCode = fails ? 1 : 0; process.exit(fails ? 1 : 0);
}
