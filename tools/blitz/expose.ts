// THE STATE BRIDGE'S WINDOW ON A MODULE'S OWN VARIABLES. A module's top-level `let`s and tables are private, and only part of them are registered with side.ts (a table
// that is built at load and never written is rightly exempt there, but the Rust game needs it all the same). This preload adds one line at the end of every source file
// of the simulation's folders, with no effect on what runs: a getter for each top-level name the schema's walk asks for, kept on globalThis.__tpsTop[<file>]. A tool's only: never in the page.
//   bun --preload ./tools/blitz/expose.ts run tools/blitz/bridge-check.ts
import { plugin } from 'bun';
import { readFileSync } from 'node:fs';
const SCHEMA = JSON.parse(readFileSync(`${import.meta.dir}/schema.json`, 'utf8'));
// THE CALL SURFACE'S GUARDS (surface.ts): a line at the top of each listed function's body, by the place the checker found it. A file that has moved since is refused by name
const CALLS = JSON.parse(readFileSync(`${import.meta.dir}/calls.json`, 'utf8'));
// WHAT A GREEN WAS MADE OF: how many calls crossed to the Rust game, and how many ran the TypeScript's body with no host standing, on a row Rust has not got, or as a function not on the surface. Printed as the process ends, so no green is read as Rust's that was not. Until a game is stood the guards answer here
if (process.env.TPS_GAME === 'rust') { const T: any = { crossed: 0, noHost: 0, noRow: 0, unlisted: 0, closed: 0, who: { noHost: new Map(), noRow: new Map(), unlisted: new Map(), closed: new Map() }, standing: true, pass(k: string, how: string) { if (T.standing) return; T[how]++; T.who[how].set(k, (T.who[how].get(k) ?? 0) + 1); } }; (globalThis as any).__tpsTally = T;
  const top = (m: Map<string, number>) => [...m].sort((a, b) => b[1] - a[1]).slice(0, process.env.TPS_GAME_SAY ? 40 : 5).map(([k, n]) => `${k} ${n}`).join(', ');
  process.on('exit', () => { process.stderr.write(`the call surface: ${T.crossed} crossed · ${T.noHost} ran the TypeScript with no host (${T.who.noHost.size} functions: ${top(T.who.noHost)}) · ${T.noRow} on a row Rust has not got (${top(T.who.noRow)}) · ${T.unlisted} not on the surface (${T.who.unlisted.size} functions: ${top(T.who.unlisted)}) · ${T.closed} ran the TypeScript with the surface closed (${T.who.closed.size} functions: ${top(T.who.closed)})\n`); });
  const none: any = (row: number) => { T.pass(CALLS.rows[row].key, 'noHost'); return none.PASS; }; none.PASS = Symbol('run the TypeScript'); none.passed = (k: string) => T.pass(k, 'noHost'); none.tapFiring = () => false; none.newGame = () => { T.standing = true; }; (globalThis as any).__tpsCall = none; }
function guarded(rel: string, text: string): string { const F = CALLS.files[rel], H = CALLS.hooks[rel], Tp = CALLS.taps?.[rel], Co = CALLS.counted?.[rel]; if (!F && !H && !Tp && !Co) return text; const len = F?.len ?? H?.[0].len ?? Tp?.[0].len ?? Co[0].len; if (text.length !== len) throw new Error(`the call surface: ${rel} is not the file the table was written from (${text.length} characters, the table says ${len}): generate again`);
  const E: { pos: number; line: string }[] = []; for (const g of F?.guards ?? []) E.push({ pos: g.pos, line: `{ const __c = (globalThis as any).__tpsCall; if (__c) { const __r = __c(${g.id}, ${g.method ? 'this' : 'undefined'}, [${g.params.join(', ')}]); if (__r !== __c.PASS) return __r; } }` });
  if (rel === 'src/game/game.ts') { const at = text.indexOf('constructor(public engine: HostApi'), open = at < 0 ? -1 : text.indexOf(') {\n', at); if (open < 0) throw new Error('the call surface: no Game constructor in game.ts'); E.push({ pos: open + 3, line: `(globalThis as any).__tpsCall?.newGame?.();` }); }   // A NEW GAME IS STANDING: until its host attaches, nothing crosses to the last game's
  for (const t of Tp ?? []) E.push({ pos: t.pos, line: `{ const __c = (globalThis as any).__tpsCall; if (__c?.tapFiring?.(${t.id})) return __c.tapAnswer() as any; }` });
  for (const k of Co ?? []) E.push({ pos: k.pos, line: `(globalThis as any).__tpsCall?.passed?.(${JSON.stringify(k.key)});` });
  for (const h of H ?? []) E.push({ pos: h.pos, line: `{ const __c = (globalThis as any).__tpsCall; if (__c?.inHook?.()) return ${JSON.stringify(h.answer ?? null)} as any; }` }); E.sort((a, b) => b.pos - a.pos);
  let out = text; for (const e of E) out = out.slice(0, e.pos) + ' ' + e.line + ' ' + out.slice(e.pos); return out; }
// WHAT THE TOOLS OF THIS PROCESS WRITE (scan.ts): the names written in each file of a tool as it loads (tools/, the bridge's own files apart): `x.name = `, `x.name[i] += `, `x.name.push(`, and a local that names a field, `const p = x.name; p[0] = `
const TOOL_WRITES = { names: new Set<string>(), version: 0 }; (globalThis as any).__tpsToolWrites = TOOL_WRITES;
function toolWrites(t: string): void { const N = TOOL_WRITES.names, was = N.size; for (const m of t.matchAll(/\.([A-Za-z_$][\w$]*)(?:\[[^\]\n]*\])*\s*(?:=|\+=|-=|\*=|\/=|\|\|=|&&=|\?\?=|\|=|&=|\+\+|--)(?![=>])/g)) N.add(m[1]); for (const m of t.matchAll(/\.([A-Za-z_$][\w$]*)\.(?:push|splice|pop|shift|unshift|set|add|delete|clear|fill|sort|length\s*=)/g)) N.add(m[1]);
  for (const m of t.matchAll(/(?:const|let)\s+([A-Za-z_$][\w$]*)\s*=\s*[^;\n]*?\.([A-Za-z_$][\w$]*)\s*(?:[;,\n])/g)) { const x = m[1].replace(/\$/g, '\\$'); if (new RegExp(`\\b${x}(?:\\[[^\\]\\n]*\\])+\\s*(?:=|\\+=|-=|\\*=)(?![=>])|\\b${x}\\.(?:push|splice|pop|shift|unshift|set|add|delete|clear|fill|sort)\\(`).test(t)) N.add(m[2]); }
  if (N.size !== was) TOOL_WRITES.version++; }
plugin({ name: 'tps-expose', setup(build) {
  build.onLoad({ filter: /\/tools\/(?!blitz\/(export|import|scan|surface|expose|puppet-apply|out-dispatch|classes|host|rust-host|poke|ask|track)\.ts).*\.ts$/ }, async ({ path }) => { const text = await Bun.file(path).text(); toolWrites(text); return { contents: text, loader: 'ts' }; });
  build.onLoad({ filter: /\/src\/(game|anim|scene|sim|math|smoke)\/.*\.ts$/ }, async ({ path }) => {
    const text0 = await Bun.file(path).text(); const rel = path.slice(path.indexOf('/src/') + 1); const text = process.env.TPS_GAME === 'rust' ? guarded(rel, text0) : text0; const names: string[] = SCHEMA.top[rel] ?? []; if (!names.length) return { contents: text, loader: 'ts' };
    const isLet = (n: string) => new RegExp(`^(?:export )?let [^\\n]*\\b${n}\\b\\s*(?:[:=,;])`, 'm').test(text); const body = names.map(n => `get ${n}() { return ${n}; }` + (isLet(n) ? `, set ${n}(v) { ${n} = v; }` : '')).join(', ');
    return { contents: `${text}\n;((globalThis as any).__tpsTop ??= {})[${JSON.stringify(rel)}] = { ${body} };\n`, loader: 'ts' };
  });
} });
