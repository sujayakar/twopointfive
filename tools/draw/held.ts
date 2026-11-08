// WHAT IS LEFT OF THE GAME THAT WAS TYPESCRIPT'S, AND WHAT HOLDS EACH FILE OF IT. The pages that stay (the game's page, which draws the Rust game, and the sound and music pages) are
// built from some files of src; every other file of src, and every level's script, is LEFT: it goes when nothing holds it. A file is held by what names it, at any depth, as code or only as
// a type: the old page, the replay viewer, a check, a level tool, the bridge, or a file of a page that stays that still names a type of it. A tool's string that spells a file's path (a check
// that reads its text) holds that one file.
//
//   bun run tools/draw/held.ts                 the counts: how much is left, by folder and by holder, and what nothing holds (it can go now)
//   bun run tools/draw/held.ts --free          the files nothing holds, one a line
//   bun run tools/draw/held.ts --why <file>    the shortest road from each kind of holder to that file
//   bun run tools/draw/held.ts --holders [kind or path]  the holders themselves (a check, a tool): what each reaches, and what nothing but it holds
//   bun run tools/draw/held.ts --doors <kind or path>  the left files those holders name themselves: by how many of them, and how many lines they would let go of with that door shut
//   bun run tools/draw/held.ts --without <path,…>  what goes free with those files or folders gone, and who still names one of them (a car's worth, before it is cut)
//   bun run tools/draw/held.ts --tsv <out>     a row a left file: lines, who holds it
// What it cannot see: a name put together at run time (`${n}.ts`), a folder read whole, a file of src that is not TypeScript. The type checks (src's and tools/qa's) see what it calls free wrongly.
// (vendor/typescript, or TS_LIB=<path to typescript.js>)
import { existsSync, readFileSync, readdirSync, statSync, writeFileSync } from 'node:fs';
import { dirname, join, relative, resolve } from 'node:path';

const ROOT = resolve(import.meta.dir, '../..'), ME = relative(ROOT, import.meta.path);
const TS_LIB = process.env.TS_LIB ?? join(ROOT, 'vendor/typescript/lib/typescript.js');
const stop = (why: string): never => { console.error(`held: ${why}`); process.exit(2); };
if (!existsSync(TS_LIB)) stop(`no TypeScript at ${TS_LIB} (vendor/typescript, or TS_LIB=<path to typescript.js>)`);
const ts = await import(TS_LIB).then(m => m.default ?? m) as typeof import('typescript');
const argv = process.argv.slice(2), flag = (n: string) => argv.includes(n), opt = (n: string): string | undefined => { const i = argv.indexOf(n); return i < 0 ? undefined : argv[i + 1] && !argv[i + 1].startsWith('--') ? argv[i + 1] : ''; };

/** the pages that stay, and the two that go: every html file's entry is one of these */
const PAGES = ['src/host/drawpage.ts', 'src/ui/startpage.ts', 'src/ui/loadbrief.ts', 'src/sounds/main.ts', 'src/music/main.ts'], GOING = ['src/main.ts', 'src/replay/viewer.ts'];
/** what holds a left file, by where it stands: the first that fits */
const KINDS: [string, RegExp][] = [['old page', /^src\/main\.ts$/], ['replay viewer', /^src\/replay\/viewer\.ts$/], ["pages' types", /^src\//], ['level tools', /^tools\/(levels\/|level-export\.ts$)/],
  ['checks', /^tools\/(qa\/|draw\/[\w-]+-check\.ts$)/], ['bridge', /^tools\/(blitz|wire)\//], ['multiplayer', /^tools\/mp\//], ['other tools', /^tools\//]];
const kindOf = (f: string): string => KINDS.find(([, re]) => re.test(f))![0];
const GAME = /^(src\/(game|sim)\/|src\/anim\/skeleton\.ts$|src\/host\/(rustgame|rustdress|standup|profile|loop|tween|ahead)\.ts$)/;

// ---- the files
const every: string[] = [];
const walk = (d: string): void => { for (const x of readdirSync(join(ROOT, d))) { if (x === 'node_modules' || x === 'target' || x.startsWith('.')) continue; const p = `${d}/${x}`; if (statSync(join(ROOT, p)).isDirectory()) walk(p); else if ((/\.(ts|mts|cjs|mjs|js)$/.test(x) || x === 'tsconfig.json') && p !== ME) every.push(p); } };
for (const d of ['src', 'tools', 'levels']) walk(d);
const there = new Set(every), text = new Map(every.map(f => [f, readFileSync(join(ROOT, f), 'utf8')])), lines = new Map([...text].map(([f, s]) => [f, s.split('\n').length - 1]));
for (const x of readdirSync(ROOT)) if (x.endsWith('.html')) for (const m of readFileSync(join(ROOT, x), 'utf8').matchAll(/<script[^>]+src="\.?\/?(src\/[^"]+\.ts)"/g)) if (!PAGES.includes(m[1]) && !GOING.includes(m[1])) stop(`${x}'s entry ${m[1]} is in neither list of pages`);
const find = (p: string): string | undefined => [p, `${p}.ts`, `${p}/index.ts`].find(x => there.has(x)), at = (f: string, sp: string) => find(relative(ROOT, resolve(ROOT, dirname(f), sp)));

// ---- what each names: as code or a type (held at any depth), and by a string that spells its path (that file alone)
const names = new Map<string, Set<string>>(), spells = new Map<string, Set<string>>();
const BRIDGE = new Map<string, string>(); { const s = text.get('tools/blitz/page-plugin.ts') ?? ''; for (const m of s.matchAll(/(\w[\w-]*)\s*:\s*'(?:\.\/)?([\w./-]+\.ts)'/g)) { const p = find(`tools/blitz/${m[2]}`); if (p) BRIDGE.set(m[1], p); } }
for (const f of every) {
  const out = new Set<string>(), said = new Set<string>(), s = text.get(f)!, put = (to: Set<string>, p?: string) => { if (p && p !== f) to.add(p); };
  if (f.endsWith('tsconfig.json')) { for (const m of s.matchAll(/"([^"*]+\.ts)"/g)) put(out, at(f, m[1])); }
  else {
    for (const i of ts.preProcessFile(s, true, true).importedFiles) if (i.fileName.startsWith('.')) put(out, at(f, i.fileName));
    for (const m of s.matchAll(/['"`]tps-bridge\/([\w-]+)['"`]/g)) { put(out, find('src/host/rustbridge.d.ts')); put(out, BRIDGE.get(m[1]) ?? find(`tools/blitz/${m[1]}`)); }
    if (f.startsWith('src/')) for (const m of s.matchAll(/['"]\/([\w-]+\.ts)['"]/g)) put(out, find(`${dirname(f)}/${m[1]}`));   // (a file handed to the bundler by its name beside this one)
    else for (const m of s.matchAll(/(['"`])((?:\$\{\w+\}\/)?[\w./-]+\.(?:ts|cjs|mjs))\1/g)) {
      const rooted = m[2].startsWith('${'), q = m[2].replace(/^\$\{\w+\}\//, '');
      if (rooted || q.startsWith('.')) put(out, rooted ? find(q) : at(f, q)); else put(said, find(q) ?? find(`src/${q}`) ?? at(f, q));
    }
  }
  names.set(f, out); spells.set(f, said);
}
const reach = (from: Iterable<string>, via?: Map<string, string>, shut?: string): Set<string> => { const seen = new Set(from), q = [...seen]; while (q.length) { const f = q.shift()!; for (const p of names.get(f) ?? []) if (!seen.has(p) && p !== shut) { seen.add(p); via?.set(p, f); q.push(p); } } return seen; };
const loc = (l: Iterable<string>) => { let n = 0; for (const f of l) n += lines.get(f) ?? 0; return n; };

// ---- what stays: what the pages are built from as code; and what those files name as a type, where nothing of the game is behind it (a file of declarations)
const scan = new Bun.Transpiler({ loader: 'ts' }), coded = (f: string): string[] => f.endsWith('.ts') ? scan.scanImports(text.get(f)!).filter(i => i.path.startsWith('.')).map(i => at(f, i.path)).filter((p): p is string => !!p) : [];
const stays = new Set(PAGES.filter(f => there.has(f)));
{ const q = [...stays]; while (q.length) for (const p of coded(q.shift()!)) if (!stays.has(p)) { stays.add(p); q.push(p); }
  const w = [...stays]; while (w.length) for (const p of names.get(w.shift()!)!) if (!stays.has(p) && ![...reach([p])].some(x => GAME.test(x) && !stays.has(x))) { stays.add(p); w.push(p); } }
const left = new Set(every.filter(f => (f.startsWith('src/') || f.startsWith('levels/')) && !stays.has(f)));
const typed = [...stays].filter(f => [...names.get(f)!].some(p => left.has(p)));   // (a file of a page that stays that names a type of what is left)
const typesOf = new Map(typed.map(f => [f, [...names.get(f)!].filter(p => left.has(p))]));
for (const f of stays) names.set(f, new Set([...names.get(f)!].filter(p => stays.has(p))));

interface Held { held: Map<string, Set<string>>; by: Map<string, Set<string>>; kinds: Map<string, string[]>; free: string[] }
/** who holds what, with the files `gone` says are gone */
function hold(gone: (f: string) => boolean): Held {
  const held = new Map<string, Set<string>>(), by = new Map<string, Set<string>>(), kinds = new Map<string, string[]>(), take = (r: string, l: Iterable<string>) => { const L = new Set([...l].filter(f => left.has(f) && !gone(f))); if (!L.size) return; held.set(r, L); const k = kindOf(r); if (!by.has(k)) by.set(k, new Set()); for (const f of L) by.get(k)!.add(f); };
  const keep = new Map(names); for (const f of every) if (gone(f)) names.set(f, new Set());
  for (const r of every) { if (gone(r)) continue; if (r.startsWith('tools/')) { const l = reach([r]); for (const f of [...l]) for (const p of f.startsWith('tools/') ? spells.get(f)! : []) l.add(p); take(r, l); } else if (GOING.includes(r)) take(r, reach([r])); else if (typesOf.has(r)) take(r, reach(typesOf.get(r)!)); }
  for (const [f, s] of keep) names.set(f, s);
  for (const [k, l] of by) for (const f of l) kinds.set(f, [...(kinds.get(f) ?? []), k]);
  return { held, by, kinds, free: [...left].filter(f => !gone(f) && !kinds.has(f)).sort() };
}
const none = () => false, H = hold(none), { held, by, kinds, free } = H;
const asked = (w: string): ((r: string) => boolean) => { if (!w) return () => true; const k = KINDS.some(([n]) => n === w); if (!k && !every.some(f => f === w || f.startsWith(w))) stop(`"${w}" is no kind of holder (${KINDS.map(([n]) => n).join(', ')}) and no path`); return r => k ? kindOf(r) === w : r.startsWith(w); };

if (flag('--free')) { for (const f of free) console.log(f); process.exit(0); }
const why = opt('--why');
if (why !== undefined) {
  const f = there.has(why) ? why : find(why) ?? find(`src/${why}`) ?? every.find(x => x.endsWith(`/${why}`)) ?? stop(`no file ${why}`);
  console.log(`${f}: ${lines.get(f)} lines, ${stays.has(f) ? 'built into a page that stays' : f.startsWith('tools/') ? 'a tool' : kinds.get(f)?.join(', ') ?? 'held by nothing'}`);
  const road = (from: string[]): string => { const via = new Map<string, string>(), seen = reach(from, via); let end = f; if (!seen.has(f)) { const by = [...seen].find(x => spells.get(x)?.has(f)); if (!by) return ''; end = by; } const r = [end]; for (let x = via.get(end); x; x = via.get(x)) r.unshift(x); return r.join(' > ') + (end === f ? '' : ` spells ${f}`); };
  if (stays.has(f)) console.log(`  a page that stays: ${road(PAGES)}`);
  for (const [k] of KINDS) { const R = [...held].filter(([r, l]) => kindOf(r) === k && l.has(f)).map(([r]) => r); if (R.length) console.log(`  ${k} (${R.length}): ${k === "pages' types" ? R.map(r => `${r} names ${typesOf.get(r)!.join(', ')}`).join(' · ') : road(R)}`); }
  process.exit(0);
}
const doors = opt('--doors');
if (doors !== undefined) {
  const ok = asked(doors), R = [...held.keys()].filter(ok), D = new Map<string, string[]>(); for (const r of R) for (const p of [...names.get(r)!, ...spells.get(r)!, ...(typesOf.get(r) ?? [])]) if (left.has(p)) D.set(p, [...(D.get(p) ?? []), r]);
  const from = (shut?: string) => [...new Set(R.flatMap(r => typesOf.has(r) ? typesOf.get(r)! : [r]))].filter(x => x !== shut), all = [...reach(from())].filter(f => left.has(f));
  const rows = [...D].map(([p, l]) => ({ p, l, n: loc(all) - loc([...reach(from(p), undefined, p)].filter(f => left.has(f))) })).sort((a, b) => b.n - a.n || b.l.length - a.l.length);
  for (const x of rows) console.log(`${String(x.n).padStart(6)} lines let go of with it shut · named by ${String(x.l.length).padStart(3)}  ${x.p}  (${x.l.slice(0, 3).map(r => r.replace(/^tools\//, '')).join(' ')}${x.l.length > 3 ? ' …' : ''})`);
  console.log(`${R.length} holders reach ${all.length} left files, ${loc(all)} lines`); process.exit(0);
}
const who = opt('--holders');
if (who !== undefined) {
  const ok = asked(who), count = new Map<string, number>(); for (const l of held.values()) for (const f of l) count.set(f, (count.get(f) ?? 0) + 1);
  const R = [...held].filter(([r]) => ok(r)).map(([r, l]) => ({ r, only: loc([...l].filter(f => count.get(f) === 1)), all: l.size, at: loc(l), direct: [...names.get(r)!, ...spells.get(r)!, ...(typesOf.get(r) ?? [])].filter(p => left.has(p)).length })).sort((a, b) => b.only - a.only || b.at - a.at);
  for (const x of R) console.log(`${String(x.only).padStart(6)} lines nothing else holds · reaches ${String(x.all).padStart(3)} files, ${String(x.at).padStart(6)} lines · names ${x.direct}  ${x.r} (${lines.get(x.r)} lines)`);
  console.log(`${R.length} holders, ${loc(R.map(x => x.r))} lines of their own`); process.exit(0);
}
const tsv = opt('--tsv'); if (tsv === '') stop('--tsv wants a file'); if (tsv) writeFileSync(tsv, ['file\tlines\theld by\tnamed by', ...[...left].sort().map(f => [f, lines.get(f), kinds.get(f)?.join(', ') ?? '', every.filter(x => names.get(x)!.has(f)).length].join('\t'))].join('\n') + '\n');

const src = every.filter(f => f.startsWith('src/') || f.startsWith('levels/'));
console.log(`held: ${src.length} files of src and levels, ${loc(src)} lines · ${stays.size} (${loc(stays)} lines) built into the pages that stay · LEFT ${left.size} files, ${loc(left)} lines`);
{ const d = new Map<string, string[]>(); for (const f of left) { const k = f.startsWith('levels/') ? 'levels' : f.split('/').length > 2 ? f.split('/').slice(0, 2).join('/') : 'src'; d.set(k, [...(d.get(k) ?? []), f]); } console.log(`  by folder: ${[...d].sort((a, b) => loc(b[1]) - loc(a[1])).map(([k, l]) => `${k} ${loc(l)}`).join(' · ')}`); }
for (const [k] of KINDS) { const l = by.get(k); if (!l) continue; const R = [...held.keys()].filter(r => kindOf(r) === k), only = [...l].filter(f => kinds.get(f)!.length === 1); console.log(`  held by the ${k}: ${l.size} files, ${loc(l)} lines, by ${R.length} file${R.length > 1 ? 's' : ''} (${loc(R)} lines of their own) · by nothing else ${only.length} files, ${loc(only)} lines`); }
console.log(`  held by nothing: ${free.length} files, ${loc(free)} lines${free.length ? ' (--free)' : ''}`);
const without = opt('--without');
if (without !== undefined) {
  const L = without.split(',').filter(Boolean), is = (f: string) => L.some(g => f === g || f.startsWith(g.endsWith('/') ? g : `${g}/`)); for (const g of L) if (!every.some(f => f === g || f.startsWith(g.endsWith('/') ? g : `${g}/`))) stop(`--without ${g}: no such file or folder`);
  const X = hold(is), fresh = X.free.filter(f => !free.includes(f)), cut = every.filter(is), dangling = every.filter(f => !is(f)).map(f => [f, [...names.get(f)!, ...spells.get(f)!, ...(typesOf.get(f) ?? [])].filter(is)] as const).filter(([, l]) => l.length);
  console.log(`without ${L.join(', ')} (${cut.length} files, ${loc(cut)} lines): ${fresh.length} files more are held by nothing, ${loc(fresh)} lines · ${[...held.keys()].filter(r => !is(r) && !X.held.has(r)).length} holders hold nothing any more`);
  for (const [k] of KINDS) if (X.by.get(k)) console.log(`  still held by the ${k}: ${X.by.get(k)!.size} files, ${loc(X.by.get(k)!)} lines, by ${[...X.held.keys()].filter(r => kindOf(r) === k).length}`);
  for (const f of fresh.slice(0, 60)) console.log(`  free: ${f}`); if (fresh.length > 60) console.log(`  … and ${fresh.length - 60} more`);
  for (const [f, l] of dangling.slice(0, 40)) console.log(`  STILL NAMED by ${f}: ${l.slice(0, 4).join(', ')}${l.length > 4 ? ` and ${l.length - 4} more` : ''}`); if (dangling.length > 40) console.log(`  … and by ${dangling.length - 40} files more`);
  process.exit(0);
}
