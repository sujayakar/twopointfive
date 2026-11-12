// THE WORDS OF A FILE, its comments and its strings, and the files they name that are not in the tree (tools/qa/docs-check.ts asks it of every file of rust, src, tools and cloud).
//   bun run tools/qa/comments.ts      each such word, by file and line; exit 1 where there is one

export type Lang = 'rs' | 'ts' | 'wgsl' | 'hash';
/** a file's language by its ending, where its comments are read */
export const LANG: Readonly<Record<string, Lang>> = { rs: 'rs', ts: 'ts', cjs: 'ts', mjs: 'ts', js: 'ts', wgsl: 'wgsl', py: 'hash', sh: 'hash', toml: 'hash' };
/** an expression may begin after these, so a slash there opens a pattern and no division */
const OPENS = /[(,=:[!&|?{};+\-*%<>~^]/, WORD = /\b(return|typeof|case|await|else|do)$/;

export interface Words { line: number; text: string; /** else it is a string, or a stretch of a template */ comment: boolean }
/** every comment of a text, with the line it begins on: what stands in a string, a template or a pattern is none */
export const comments = (s: string, lang: Lang): Words[] => words(s, lang).filter(w => w.comment);
/** every comment and every string of a text, in their order */
export function words(s: string, lang: Lang): Words[] {
  const out: Words[] = [], n = s.length, holes: number[] = []; let i = 0, line = 1, prev = '';
  if (lang === 'hash') { s.split('\n').forEach((l, k) => { const m = l.match(/(?:^|\s)#(.*)$/); if (m) out.push({ line: k + 1, text: m[1], comment: true }); }); return out; }
  const to = (j: number): void => { for (let k = i; k < j && k < n; k++) if (s[k] === '\n') line++; i = j; };
  const string = (a: number, b: number): void => { out.push({ line, text: s.slice(a, b), comment: false }); };
  /** past a template's text: its end, or a hole in it */
  const template = (j: number): void => { while (j < n && s[j] !== '`') { if (s[j] === '\\') { j += 2; continue; } if (s[j] === '$' && s[j + 1] === '{') { string(i, j); holes.push(1); to(j + 2); prev = '{'; return; } j++; } string(i, j); to(j + 1); prev = '`'; };
  while (i < n) { const c = s[i];
    if (c === '/' && s[i + 1] === '/') { const e = s.indexOf('\n', i), j = e < 0 ? n : e; out.push({ line, text: s.slice(i, j), comment: true }); to(j); continue; }
    if (c === '/' && s[i + 1] === '*') { let d = 1, j = i + 2; while (j < n && d) { if (lang === 'rs' && s.startsWith('/*', j)) { d++; j += 2; } else if (s.startsWith('*/', j)) { d--; j += 2; } else j++; } out.push({ line, text: s.slice(i, j), comment: true }); to(j); continue; }
    if (lang === 'rs') {
      const raw = /^b?r(#*)"/.exec(s.slice(i, i + 12)); if (raw && !/\w/.test(s[i - 1] ?? '')) { const j = s.indexOf(`"${raw[1]}`, i + raw[0].length); string(i + raw[0].length, j < 0 ? n : j); to(j < 0 ? n : j + 1 + raw[1].length); continue; }
      if (c === "'") { const ch = /^'(?:\\(?:u\{[^}]*\}|x..|.)|[^'\\])'/u.exec(s.slice(i, i + 14)); to(i + (ch ? ch[0].length : 1)); continue; }
      if (c === '"') { let j = i + 1; while (j < n && s[j] !== '"') j += s[j] === '\\' ? 2 : 1; string(i + 1, j); to(j + 1); continue; }
    } else if (lang === 'ts') {
      if (c === '"' || c === "'") { let j = i + 1; while (j < n && s[j] !== c && s[j] !== '\n') j += s[j] === '\\' ? 2 : 1; string(i + 1, j); to(j + 1); prev = c; continue; }
      if (c === '`') { template(i + 1); continue; }
      if (holes.length && c === '{') holes[holes.length - 1]++;
      if (holes.length && c === '}' && --holes[holes.length - 1] === 0) { holes.pop(); template(i + 1); continue; }
      if (c === '/' && (!prev || OPENS.test(prev) || WORD.test(s.slice(Math.max(0, i - 12), i).trimEnd()))) { let j = i + 1, set = false; while (j < n && s[j] !== '\n' && (set || s[j] !== '/')) { if (s[j] === '\\') j++; else if (s[j] === '[') set = true; else if (s[j] === ']') set = false; j++; } to(j + 1); prev = '/'; continue; }
    }
    if (!/\s/.test(c)) prev = c === '!' && /[\w)\]]/.test(s[i - 1] ?? '') && s[i + 1] !== '=' ? 'x' : c;   // (a word's own `!` ends it: a slash after one divides)
    to(i + 1); }
  return out; }

const NAMED = /(?:[\w.-]+\/)*[A-Za-z_][\w-]*(?:\.[\w-]+)*\.(?:ts|rs|wgsl|cjs|mjs|py|md)\b/g;
/** names a check plants to see that it finds them */
const PLANTED: Readonly<Record<string, readonly string[]>> = { 'tools/qa/docs-check.ts': ['docs/not-here.md', 'tools/qa/none-check.ts', 'src/game/nobody.ts', 'docs/internal/nowhere.md'], 'tools/qa/level-needs.ts': ['tools/levels/not-there.ts'], 'tools/qa/rust-rules-check.ts': ['rust/sim/src/planted.rs', 'sim/src/planted.rs'] };
/** the files a text names that none of these paths is, or ends in. A name that stands for many (a star, angle brackets), an address or an absolute path is nobody's; a page, a script or (`pathed`) any name counts with a folder before it */
export function notThere(paths: readonly string[]): (text: string, pathed?: boolean) => string[] {
  const ends = new Map<string, string[]>(); for (const p of paths) { const b = p.slice(p.lastIndexOf('/') + 1), l = ends.get(b); if (l) l.push(p); else ends.set(b, [p]); }
  return (text, pathed = false) => { const out: string[] = []; for (const m of text.matchAll(NAMED)) { const before = text.slice(0, m.index), r = m[0].replace(/^(?:\.{1,2}\/)+/, '');
    if (/[*<>{}$][^\s`'"(),]*$/.test(before) || /\/$/.test(before) || r.startsWith('-') || !r.includes('/') && (pathed || /\.(?:cjs|mjs|py|md)$/.test(r))) continue;
    if (!(ends.get(r.slice(r.lastIndexOf('/') + 1)) ?? []).some(p => !r.includes('/') || p === r || p.endsWith(`/${r}`))) out.push(r); } return out; }; }

/** every comment and every string of these files that names a file that is gone, as `file:line: name`. A tool's strings name files of its own making: there a folder must stand before it */
export function goneInWords(paths: readonly string[], read: (path: string) => string): string[] {
  const gone = notThere(paths), out: string[] = [];
  for (const f of paths) { const lang = LANG[f.slice(f.lastIndexOf('.') + 1)]; if (!lang || !/^(?:rust|src|tools|cloud)\//.test(f) || /_generated\//.test(f) || f === 'tools/qa/comments.ts') continue;
    for (const c of words(read(f), lang)) c.text.split('\n').forEach((l, k) => { for (const g of gone(l, !c.comment && !/^(?:rust|src)\//.test(f))) if (!PLANTED[f]?.includes(g)) out.push(`${f}:${c.line + k}: ${g}${c.comment ? '' : ' (a string)'}`); }); }
  return out; }

if (import.meta.main) {
  const { execFileSync } = await import('node:child_process'), { existsSync, readFileSync } = await import('node:fs'), root = `${import.meta.dir}/../..`;
  const paths: string[] = (execFileSync('git', ['-C', root, 'ls-files', '--cached', '--others', '--exclude-standard'], { encoding: 'utf8', maxBuffer: 64 << 20 }) as string).split('\n').filter(f => f && existsSync(`${root}/${f}`));
  const bad = goneInWords(paths, f => readFileSync(`${root}/${f}`, 'utf8')); for (const b of bad) console.log(b);
  process.exit(bad.length ? 1 : 0);
}
