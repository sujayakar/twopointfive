// THE DELETION OF THE TYPESCRIPT TICK, AS A PROGRAM (tools/blitz/DELETION.md). The Rust game (rust/game) is the game that steps; the TypeScript `Game` stays as
// the puppet the picture, the HUD, the menus and the tools read. This program takes out of the simulation's folders every function that nothing left
// standing can reach, and turns a rule that a tool calls between ticks into the call of the Rust rule of the same name. It decides by REACHABILITY, read
// from the TypeScript checker over the whole tree (src/ and tools/), never by a function's name or a list kept by hand.
//
//   bun run tools/blitz/delete-tick.ts                     a dry run: the classes with their counts, nothing written
//   bun run tools/blitz/delete-tick.ts --write             the deletion, in place (then both type checks: the program does not run them)
//   bun run tools/blitz/delete-tick.ts --note              tools/blitz/DELETION.md's tables on stdout · --json out.json the whole plan, a row a declaration
//   bun run tools/blitz/delete-tick.ts --why Class.method  the chain of references that keeps a function (or says that nothing does)
//   bun run tools/blitz/delete-tick.ts --bridges [n]       the references out of the page and the hosts that keep the most, by what each alone keeps
//   bun run tools/blitz/delete-tick.ts --instrument        THE CENSUS'S TREE: nothing deleted, the surface's lines written into the source, and a counter
//                                                          at the top of every function (tools/blitz/delete-census.sh runs the plays on it)
//   bun run tools/blitz/delete-tick.ts --readers out.tsv   who still reads each field of the simulation's classes and tables, after the deletion's plan
//   --census file.json   what the census saw run (default tools/blitz/delete-census.json; `--no-census`: a host's loose references are followed too)
//   --census-settles-all the census settles the page's loose references as it does a host's (deletes more; rests on the census having played every path)
//
// WHAT STAYS REACHABLE (the roots), and how far each reaches:
//   the page      every file of src/ outside the simulation's folders, the game's page and the pages with no Rust game alike (the workbench, the replay
//                 viewer, the sound pages). The page has no call surface: what it calls on the puppet runs as TypeScript, so a function it reaches
//                 keeps its BODY, and so does everything that body reaches.
//   the hosts     the files of tools/ that a page or a host loads (the bridge's generated files, the harness's host, the dev server, the build, the
//                 game server): found by following imports from the entry points in `HOST_ENTRIES`. They run inside the host's own lines: BODY.
//   the module    a simulation file's own top-level statements, and the top-level names the bridge's schema reads through its window (schema.json `top`).
//   a tool        every other file of tools/ (the checks). A tool runs between ticks, where a call of a rule crosses to the Rust game: a function it
//                 names keeps its SIGNATURE. With a row on the call surface (calls.json) its body becomes the crossing; with none it is GONE: it throws,
//                 by name, and the tools that named it are listed.
//   Game.stepSeats  is the tick. Its body goes; a host hangs its own step on the game object.
// EXACT AND LOOSE. A reference the checker resolves to a declaration is exact. So are: a member of an interface for every class that says it implements
// it; an object handed where another type is asked for (the members that type names); a cast (`x as unknown as T`: a flow, private members too); a name
// read through a cast or through a constant given one, by the type under the cast; a name read off `any` through two or more steps that are a path from
// the `Game` class; a name read off a module loaded by a written path. What is left is LOOSE, and who makes the reference decides what it holds:
//   the page and the simulation's own kept code: by the checker ALONE, no census asked. A name off a value typed `any` holds the classes' members of that
//     name where the read could run one (a call, a test for it; a read that only looks at a value is a read of data); a member of a type a class answers
//     only by its shape holds that class's where an untyped value is handed to the type somewhere (elsewhere `handed` has the reference exactly).
//   a host's own lines (the bridge's files, which read the puppet by name on every tick of every play): THE CENSUS settles it. A candidate it saw run
//     keeps its body; one it never saw keeps at most a crossing, unless the read tests for the function.
//   a tool: nothing. The tool is listed.
// THE CENSUS (the plays of the Rust game on a tree with a counter in every function) otherwise only ADDS: what it saw run as TypeScript stays, however
// it was reached. Nothing the page can reach is deleted on the census's word: it has seen ticks, starts and a few flows of the page, not every draw.
// A function reached with its body AND on the call surface keeps the surface's guard as a line of its own source (expose.ts wrote it at load, by a place
// in the file that this program's edits move).
import { readFileSync, writeFileSync, readdirSync, statSync, existsSync } from 'node:fs';
import { join, resolve, relative, dirname } from 'node:path';
import { createHash } from 'node:crypto';

const ROOT = process.env.DELETE_ROOT ? resolve(process.env.DELETE_ROOT) : resolve(import.meta.dir, '../..');
const TS_LIB = process.env.TS_LIB ?? join(ROOT, 'vendor/typescript/lib/typescript.js');
if (!existsSync(TS_LIB)) { console.error(`no TypeScript at ${TS_LIB} (vendor/typescript, or TS_LIB=<path to typescript.js>)`); process.exit(2); }
const tsMod: any = await import(TS_LIB); const ts: any = tsMod.default ?? tsMod; const SK = ts.SyntaxKind;
const ARGV = process.argv.slice(2), flag = (n: string) => ARGV.includes(n), opt = (n: string) => { const i = ARGV.indexOf(n); return i < 0 ? undefined : ARGV[i + 1]; };
const say = (s: string) => process.stderr.write(s + '\n');

// ---- the census's own step: its runs' lines made into the file the plan reads (no tree is read for this)
//   bun run tools/blitz/delete-tick.ts --census-merge <ids.json> <ran.jsonl> [<ran.jsonl> …]      → tools/blitz/delete-census.json (or --census file)
if (flag('--census-merge')) { const i = ARGV.indexOf('--census-merge'), rest = ARGV.slice(i + 1).filter((a, k, A) => !a.startsWith('--') && A[k - 1] !== '--census'), ids = JSON.parse(readFileSync(rest[0], 'utf8'));
  const name = new Map<number, [string, string]>(); for (const [id, file, key] of ids.units) name.set(id, [file, key]);
  const seen = new Map<number, { n: number; page: number; perFrame: number }>(), runs: string[] = [];
  for (const f of rest.slice(1)) for (const line of readFileSync(f, 'utf8').split('\n')) { if (!line) continue; const r = JSON.parse(line); runs.push(`${r.page ? `page ${r.frames} frames ` : ''}${String(r.argv).replace(/\/tmp\/[^ ]*/g, '…').replace(/^.*\/tools\//, 'tools/')}`);
    for (const [id, n] of r.ran) { let s = seen.get(id); if (!s) seen.set(id, s = { n: 0, page: 0, perFrame: 0 }); s.n += n; if (r.page) { s.page += n; if (r.frames > 0) s.perFrame = Math.max(s.perFrame, n / r.frames); } } }
  /* (a run is named by its tool and words: no path of the machine it ran on, no address of its server) */ const out = { fingerprint: ids.fingerprint, runs: runs.map(r => r.replace(/https?:\/\/[^/ ]+/g, '').replace(/(^|[ =])\/[^ ]*\/([^ /]+)/g, '$1$2')).sort(), seen: [...seen].map(([id, s]) => ({ file: name.get(id)![0], key: name.get(id)![1], n: s.n, ...(s.page ? { page: s.page, perFrame: Math.round(s.perFrame * 100) / 100 } : {}) })).sort((a, b) => a.file < b.file ? -1 : a.file > b.file ? 1 : a.key < b.key ? -1 : a.key > b.key ? 1 : 0) };
  const to = opt('--census') ?? join(ROOT, 'tools/blitz/delete-census.json'); writeFileSync(to, JSON.stringify(out, null, 0).replace(/\},\{/g, '},\n{') + '\n'); say(`the census: ${out.seen.length} functions seen in ${runs.length} runs → ${to}`); process.exit(0); }

// ---- the tree
/** the simulation's folders: where this program may delete */
const SIM = /^src\/(game|anim|math|sim|scene)\//;
/** THE ENTRY POINTS OF THE HOSTS in tools/: a file of tools/ is a host's when one of these, or a file of src/, loads it */
const HOST_ENTRIES = ['tools/blitz/rust-host.ts', 'tools/blitz/page-plugin.ts', 'tools/serve.ts', 'tools/build.ts', 'tools/build-one.ts', 'tools/mp/server.ts', 'tools/mp/runner.ts'];
/** the bridge's files as the page names them (page-plugin.ts FILES): `tps-bridge/<name>` */
const BRIDGE_ALIAS = /^tps-bridge\/(.*)$/;
/** the module the stubs call (written by this program on `--write`) */
const CROSSING = 'src/game/crossing.ts';
const rel = (p: string) => relative(ROOT, p).split('\\').join('/');
function walk(dir: string, out: string[]): string[] { if (!existsSync(dir)) return out; for (const n of readdirSync(dir)) { if (n === 'node_modules' || n === 'fixture' || n === 'fixtures' || n.startsWith('.')) continue; const p = join(dir, n); if (statSync(p).isDirectory()) walk(p, out); else if (/\.ts$/.test(n)) out.push(p); } return out; }
const FILES = [...walk(join(ROOT, 'src'), []), ...walk(join(ROOT, 'tools'), []), ...walk(join(ROOT, 'types'), [])];
const cfg = ts.readConfigFile(join(ROOT, 'tsconfig.json'), ts.sys.readFile).config;
const OPTS = ts.convertCompilerOptionsFromJson({ ...cfg.compilerOptions, allowImportingTsExtensions: true, noEmit: true }, ROOT).options;
const t0 = performance.now();
const program = ts.createProgram(FILES, OPTS), checker = program.getTypeChecker();
say(`read ${FILES.length} files in ${((performance.now() - t0) / 1000).toFixed(1)} s`);
const sfByRel = new Map<string, any>(); for (const sf of program.getSourceFiles()) sfByRel.set(rel(sf.fileName), sf);
const simFiles = program.getSourceFiles().filter((sf: any) => SIM.test(rel(sf.fileName)));
/** the tree this plan was read from: every simulation file's bytes. A census names it, and is refused on another. */
const FINGERPRINT = (() => { const h = createHash('sha256'); for (const sf of [...simFiles].sort((a: any, b: any) => a.fileName < b.fileName ? -1 : 1)) { if (rel(sf.fileName) === CROSSING) continue; h.update(rel(sf.fileName)); h.update('\0'); h.update(sf.text); h.update('\0'); } return h.digest('hex').slice(0, 16); })();

// ---- the tables the surface was made from
const CALLS = JSON.parse(readFileSync(join(ROOT, 'tools/blitz/calls.json'), 'utf8'));
const SCHEMA = JSON.parse(readFileSync(join(ROOT, 'tools/blitz/schema.json'), 'utf8'));
const TAPS: any[] = JSON.parse(readFileSync(join(ROOT, 'tools/blitz/taps.json'), 'utf8'));
if (Object.keys(CALLS.hooks ?? {}).length) { console.error('calls.json lists hooks: this program writes no line for one'); process.exit(2); }
/** a row's word in a stub: its key, with its file where two rows share the key */
const stubKey = (() => { const n = new Map<string, number>(); for (const r of CALLS.rows) n.set(r.key, (n.get(r.key) ?? 0) + 1); return (r: any): string => r.stub ?? (n.get(r.key)! > 1 ? `${r.key}@${r.file}` : r.key); })();

// ---- units: what can stay or go as one thing
type Kind = 'fn' | 'method' | 'accessor' | 'ctor' | 'field' | 'var' | 'class' | 'type' | 'stmt';
interface Edge { to?: Unit; name?: string; iface?: any; at: number; file: string; how?: string; /** a loose read that tests for the function, and would not throw if it were gone */ soft?: boolean; /** a loose read that only looks at the value (`o.round !== undefined`, `f(smp.input)`): of data, never a call of a method */ bare?: boolean }
interface Unit { id: number; file: string; key: string; name: string; kind: Kind; nodes: any[]; cls?: Unit; members?: Unit[]; /** a function: its body can go and its signature stay */ fnLike: boolean; isStatic: boolean;
  sig: Edge[]; body: Edge[]; /** what the type of its answer names, where the answer's type is not written (found when its body is to go) */ ret?: Edge[]; row?: any; guard?: any; tap?: any; lines: number }
const UNITS: Unit[] = [], unitOfNode = new Map<any, Unit>(), /** class members by name */ byName = new Map<string, Unit[]>(), /** top-level functions, variables and classes by name: what a module loaded untyped is read for */ topByName = new Map<string, Unit[]>();
/** every declaration a name read off a value typed `any` could mean */
const looseOf = (name: string): Unit[] => [...(byName.get(name) ?? []), ...(topByName.get(name) ?? [])];
const lineOf = (sf: any, p: number) => sf.getLineAndCharacterOfPosition(p).line + 1;
const lineCount = (n: any) => { const sf = n.getSourceFile(); return lineOf(sf, n.end) - lineOf(sf, n.getStart(sf)) + 1; };
const isStatic = (n: any) => !!(ts.getCombinedModifierFlags(n) & ts.ModifierFlags.Static);
const isFnInit = (i: any) => !!i && (i.kind === SK.ArrowFunction || i.kind === SK.FunctionExpression);
const nameText = (n: any): string => n == null ? '' : n.kind === SK.ComputedPropertyName ? `[${n.expression.getText()}]` : (n.text ?? String(n.escapedText ?? n.getText()));
function unit(file: string, key: string, name: string, kind: Kind, node: any, fnLike: boolean, cls?: Unit): Unit {
  // (overloads, and a getter with its setter, are one unit of several nodes)
  const st = isStatic(node), same = cls ? cls.members!.find(u => u.name === name && u.kind === kind && u.isStatic === st && kind !== 'stmt') : kind === 'fn' ? UNITS.find(u => u.file === file && u.kind === 'fn' && u.name === name && !u.cls) : undefined;
  if (same) { same.nodes.push(node); unitOfNode.set(node, same); same.lines += lineCount(node); return same; }
  const u: Unit = { id: UNITS.length, file, key, name, kind, nodes: [node], cls, fnLike, isStatic: st, sig: [], body: [], lines: lineCount(node) };
  UNITS.push(u); unitOfNode.set(node, u); if (cls) cls.members!.push(u);
  if (kind === 'method' || kind === 'accessor' || kind === 'field') { let a = byName.get(name); if (!a) byName.set(name, a = []); a.push(u); }
  else if (!cls && name && kind !== 'stmt' && kind !== 'type') { let a = topByName.get(name); if (!a) topByName.set(name, a = []); a.push(u); }
  return u; }
for (const sf of simFiles) { const file = rel(sf.fileName); if (file === CROSSING) continue;
  for (const st of sf.statements) {
    if (st.kind === SK.FunctionDeclaration && st.name) unit(file, st.name.text, st.name.text, 'fn', st, true);
    else if (st.kind === SK.ClassDeclaration && st.name) { const C = unit(file, st.name.text, st.name.text, 'class', st, false); C.members = [];
      for (const m of st.members) { const n = nameText(m.name);
        if (m.kind === SK.MethodDeclaration) unit(file, `${C.name}.${n}`, n, 'method', m, true, C);
        else if (m.kind === SK.GetAccessor || m.kind === SK.SetAccessor) unit(file, `${C.name}.${n}`, n, 'accessor', m, true, C);
        else if (m.kind === SK.Constructor) unit(file, `${C.name}.constructor`, 'constructor', 'ctor', m, false, C);
        else if (m.kind === SK.PropertyDeclaration) unit(file, `${C.name}.${n}`, n, 'field', m, isFnInit(m.initializer), C);
        else unit(file, `${C.name}.<static>`, '<static>', 'stmt', m, false, C); } }
    else if (st.kind === SK.VariableStatement) for (const d of st.declarationList.declarations) {
      if (d.name.kind === SK.Identifier) unit(file, d.name.text, d.name.text, 'var', d, isFnInit(d.initializer));
      else unit(file, `<pattern ${d.name.getText().slice(0, 40)}>`, '', 'var', d, false); }
    else if (st.kind === SK.InterfaceDeclaration || st.kind === SK.TypeAliasDeclaration || st.kind === SK.EnumDeclaration || st.kind === SK.ModuleDeclaration) unit(file, nameText(st.name), nameText(st.name), 'type', st, false);
    else if (st.kind === SK.ImportDeclaration || st.kind === SK.ExportDeclaration || st.kind === SK.ImportEqualsDeclaration) { /* pruned by what is left */ }
    else unit(file, `<statement ${file}:${lineOf(sf, st.getStart(sf))}>`, '', 'stmt', st, false);
  } }
/** the unit a declaration belongs to: the class member or top-level declaration that holds it */
function unitOfDecl(d: any): Unit | undefined { for (let n = d; n; n = n.parent) { const u = unitOfNode.get(n); if (u) return u; if (n.kind === SK.SourceFile) return undefined; } return undefined; }
/** a function's body, where it has one to lose */
function bodyOf(n: any): any { if (n.kind === SK.PropertyDeclaration || n.kind === SK.VariableDeclaration) return isFnInit(n.initializer) ? n.initializer.body : undefined; return n.body; }
const fnOf = (n: any): any => (n.kind === SK.PropertyDeclaration || n.kind === SK.VariableDeclaration) ? n.initializer : n;
const implOf = (u: Unit): any => u.nodes.filter(n => bodyOf(n)).pop();

// ---- the call surface's rows and taps, by file and key
{ const rows = new Map<string, any>(); for (const r of CALLS.rows) rows.set(`${r.file}\n${r.key}`, r);
  const guards = new Map<number, any>(); for (const F of Object.values<any>(CALLS.files)) for (const g of F.guards ?? []) guards.set(g.id, g);
  const taps = new Map<string, any>(); for (const t of TAPS) taps.set(`${t.file}\n${t.key}`, t);
  let ng = 0, nt = 0;
  for (const u of UNITS) { const r = rows.get(`${u.file}\n${u.key}`); if (r && guards.has(r.id) && u.fnLike) { u.row = r; u.guard = guards.get(r.id); ng++; } const t = taps.get(`${u.file}\n${u.key}`); if (t && u.fnLike) { u.tap = t; nt++; } }
  if (ng !== guards.size) { const got = new Set(UNITS.filter(u => u.guard).map(u => u.row.id)); console.error(`calls.json guards ${guards.size} functions and ${ng} were found by key: ${[...guards.keys()].filter(i => !got.has(i)).slice(0, 12).map(i => { const r = CALLS.rows.find((x: any) => x.id === i); return `${r?.key} (${r?.file})`; }).join(', ')}`); process.exit(2); }
  const tapsHere = Object.values<any>(CALLS.taps ?? {}).reduce((a, l) => a + l.length, 0); if (nt < tapsHere) { const got = new Set(UNITS.filter(u => u.tap).map(u => u.tap.id)); console.error(`calls.json places ${tapsHere} taps and ${nt} were found by key: ${TAPS.filter(t => !got.has(t.id)).map(t => `${t.key} (${t.file})`).join(', ')}`); process.exit(2); } }

// ---- references
const pos = (file: string, p: number) => `${file}:${lineOf(sfByRel.get(file), p)}`;
const STATS = { dyn: 0, pathTyped: 0, castTyped: 0, castNone: 0, softHeld: 0, fed: 0 };
function targetOf(sym: any): any { if (!sym) return undefined; if (sym.flags & ts.SymbolFlags.Alias) { try { sym = checker.getAliasedSymbol(sym); } catch { return undefined; } } return sym; }
const GAME = UNITS.find(u => u.kind === 'class' && u.key === 'Game' && u.file === 'src/game/game.ts');
const GAME_TYPE = GAME ? checker.getDeclaredTypeOfSymbol(checker.getSymbolAtLocation(GAME.nodes[0].name)) : undefined;
/** `x.a.b.c` with `x` typed any: the names from the root, or null where the chain is not plain names off an identifier */
function chainOf(e: any, hops: number): string[] | null { const names: string[] = [];
  for (;;) { while (e.kind === SK.NonNullExpression || e.kind === SK.ParenthesizedExpression || e.kind === SK.AsExpression || e.kind === SK.TypeAssertionExpression) e = e.expression;
    if (e.kind === SK.PropertyAccessExpression) { names.unshift(e.name.text); e = e.expression; continue; }
    if (e.kind !== SK.Identifier) return null;
    // (a constant is the expression it was given: `const o = P.g.orders; o.give(…)` reads `orders.give`)
    const init = hops < 4 ? constInit(e) : undefined, pre = init ? chainOf(init, hops + 1) : null; return pre ? [...pre, ...names] : names; } }
function anyChain(nameNode: any): string[] | null { const c = chainOf(nameNode.parent.expression, 0); return c ? [...c, nameNode.text] : null; }
/** the expression a constant was given (`const fog = g.fogOfWar as unknown as Record<string, any>`): a name read off the constant is read off that */
function constInit(e: any): any { if (e?.kind !== SK.Identifier) return undefined; const d = checker.getSymbolAtLocation(e)?.valueDeclaration; return d?.kind === SK.VariableDeclaration && d.initializer && (d.parent.flags & ts.NodeFlags.Const) && d.name.kind === SK.Identifier ? d.initializer : undefined; }
/** A NAME READ THROUGH A CAST: `(x as any).a.b`, `(x as unknown as Record<string, any>).m`, and the same through a constant. The checker has no symbol for
 *  the names after the cast, but the expression under it still has its type, and the names are read off THAT: the property's own symbol where the type
 *  declares the whole path, `'none'` where it declares no such name (`(g as any).pair?.waiting?.()` on a game with no `pair`: a read of nothing the
 *  simulation declares, whatever a later build may hang there), undefined where no cast with a known type is under the chain. */
function castRooted(nameNode: any): any { const names: string[] = [nameNode.text]; let e = nameNode.parent.expression, hops = 0;
  for (;;) { let cast = false;
    while (e.kind === SK.NonNullExpression || e.kind === SK.ParenthesizedExpression || e.kind === SK.AsExpression || e.kind === SK.TypeAssertionExpression) { if (e.type) cast = true; e = e.expression; }
    if (cast) { let t: any; try { t = checker.getNonNullableType(checker.getTypeAtLocation(e)); } catch { return undefined; }
      if (t && !(t.flags & (ts.TypeFlags.Any | ts.TypeFlags.Unknown)) && (t.flags & ts.TypeFlags.Object) && !checker.getSignaturesOfType(t, ts.SignatureKind.Construct).length) { let prop: any;
        for (let i = 0; i < names.length; i++) { prop = checker.getPropertyOfType(t, names[i]); if (!prop) { const C = t.symbol && classOfSym.get(t.symbol); return C && !subDeclares(C, names[i]) ? 'none' : undefined; } /* (a class's object has what the class, or a class under it, declares: a type that is no class says nothing of what else its object has) */ if (i + 1 < names.length) { t = checker.getNonNullableType(checker.getTypeOfSymbol(prop)); if (t.flags & (ts.TypeFlags.Any | ts.TypeFlags.Unknown)) return undefined; } }
        return prop; } }
    if (e.kind === SK.PropertyAccessExpression) { names.unshift(e.name.text); e = e.expression; continue; }
    const init = hops++ < 4 ? constInit(e) : undefined; if (!init) return undefined; e = init; } }
function gamePathFrom(names: string[]): any { let t = GAME_TYPE, prop: any; for (const n of names) { if (!t) return undefined; prop = checker.getPropertyOfType(checker.getNonNullableType(t), n); if (!prop) return undefined; t = checker.getTypeOfSymbol(prop); } return prop; }
/** the last two names or more of the chain, where they are a path from the game (`P.g.cues.stats`: `cues.stats`) */
function gamePath(names: string[]): any { for (let i = 0; i + 2 <= names.length; i++) { const p = gamePathFrom(names.slice(i)); if (p) return p; } return undefined; }
/** the simulation's classes, by the checker's symbol */
const SIM_CLASS_SYMS = new Set<any>(), classOfSym = new Map<any, Unit>(); for (const u of UNITS) if (u.kind === 'class') { const sy = checker.getSymbolAtLocation(u.nodes[0].name); SIM_CLASS_SYMS.add(sy); classOfSym.set(sy, u); }
/** AN OBJECT HANDED WHERE ANOTHER TYPE IS ASKED FOR. `loop({ demo })` hands a `Demo` where a `DemoLike` is asked for, and from then on whoever holds
 *  the `DemoLike` calls `Demo`'s methods through names the checker resolves to the interface. So at every place a value whose type is one of the
 *  simulation's classes is written where the type asked for is another one (an interface, a type written in place, a union of them), every member
 *  that type names is a use of the class's member of that name. This is exact: the checker says both types at that place. */
const memberOf = (C: Unit, name: string): Unit | undefined => { for (const K of [C, ...allSupersLazy(C)]) { const m = K.members!.find(x => x.name === name && x.kind !== 'ctor' && !x.isStatic); if (m) return m; } return undefined; };
let allSupersLazy: (C: Unit) => Iterable<Unit> = () => [], allSubsLazy: (C: Unit) => Iterable<Unit> = () => [];
const subDeclares = (C: Unit, name: string): boolean => { for (const D of allSubsLazy(C)) if (D.members!.some(x => x.name === name)) return true; return false; };
const flowMemo = new Map<string, Unit[]>();
function handed(n: any, out: Edge[], sf: any, file: string): void {
  let t: any; try { t = checker.getTypeAtLocation(n); } catch { return; } const C = t?.symbol && classOfSym.get(t.symbol); if (!C || !(t.flags & ts.TypeFlags.Object)) return;
  if (checker.getSignaturesOfType(t, ts.SignatureKind.Construct).length) return;   // (the class itself, not one of its objects)
  // (A CAST IS A FLOW, whatever the checker thinks of the two types: `g.destr as unknown as { doorsFor?(…): … }` hands the class's object where the type
  // written last is asked for, private members too. The chain of casts is climbed to its last word.)
  let ct: any, cast = false; const p = n.parent; try { if (p.kind === SK.ShorthandPropertyAssignment) { const o = checker.getContextualType(p.parent), pr = o && checker.getPropertyOfType(checker.getNonNullableType(o), n.text); ct = pr && checker.getTypeOfSymbol(pr); }
    else { let top = n, last: any; for (let q = top.parent; q && q.expression === top && (q.kind === SK.AsExpression || q.kind === SK.TypeAssertionExpression || q.kind === SK.SatisfiesExpression || q.kind === SK.ParenthesizedExpression || q.kind === SK.NonNullExpression); top = q, q = q.parent) if (q.type) last = q.type;
      if (last) { ct = checker.getTypeFromTypeNode(last); cast = true; } else ct = checker.getContextualType(n); } } catch { return; }
  if (!ct || ct === t || (ct.flags & (ts.TypeFlags.Any | ts.TypeFlags.Unknown))) return;
  for (const T0 of ct.types ?? [ct]) { const T = cast && T0 ? checker.getNonNullableType(T0) : T0; if (!T || T === t || !(T.flags & (ts.TypeFlags.Object | ts.TypeFlags.Intersection)) || (T.symbol && classOfSym.has(T.symbol))) continue;
    const key = `${C.id}:${(T as any).id}`; let ms = flowMemo.get(key);
    if (!ms) { ms = []; for (const pr of checker.getPropertiesOfType(T)) { const m = memberOf(C, pr.name); if (m) ms.push(m); } flowMemo.set(key, ms); }
    for (const m of ms) out.push({ to: m, at: n.getStart(sf), file, how: 'has its object handed where a type that names it is asked for' }); } }
/** `import(\`${ROOT}/src/game/x.ts\`)`: the module, where the path is one substitution and then a path under the repository written out */
function dynModule(call: any): any { const a = call.arguments?.[0]; if (!a) return undefined; let lit: string | undefined;
  if (a.kind === SK.TemplateExpression && a.head.text === '' && a.templateSpans.length === 1) lit = a.templateSpans[0].literal.text;
  else if (a.kind === SK.BinaryExpression && a.operatorToken.kind === SK.PlusToken && ts.isStringLiteralLike(a.right)) lit = a.right.text;
  if (!lit || !/^\/(src|tools|levels)\/.+\.ts$/.test(lit)) return undefined; const sf = sfByRel.get(lit.slice(1)); return sf ? checker.getSymbolAtLocation(sf) : undefined; }
const isDynImport = (n: any) => n.kind === SK.CallExpression && n.expression.kind === SK.ImportKeyword;
/** the module a value is, where the value is `await import(…)` of a written path, perhaps in brackets or cast */
function moduleOfExpr(e: any): any { while (e && (e.kind === SK.ParenthesizedExpression || e.kind === SK.AwaitExpression || e.kind === SK.AsExpression || e.kind === SK.NonNullExpression || e.kind === SK.TypeAssertionExpression)) e = e.expression; return e && isDynImport(e) ? dynModule(e) : undefined; }
/** the variables of a file that hold such a module: `const M: any = await import(…)` */
const modVarsOf = new Map<any, Map<any, any>>();
function modVars(sf: any): Map<any, any> { let m = modVarsOf.get(sf); if (m) return m; m = new Map(); modVarsOf.set(sf, m);
  const v = (n: any): void => { if (n.kind === SK.VariableDeclaration && n.name.kind === SK.Identifier && n.initializer) { const mod = moduleOfExpr(n.initializer); if (mod) { const s = checker.getSymbolAtLocation(n.name); if (s) m!.set(s, mod); } } ts.forEachChild(n, v); }; v(sf); return m; }
const exportOf = (mod: any, name: string): any => mod?.exports?.get(ts.escapeLeadingUnderscores(name)) ?? checker.getExportsOfModule(mod).find((x: any) => x.name === name);
/** A READ THAT TESTS WHETHER THE FUNCTION IS THERE, and so would not throw if it were gone: `x.f?.()`, `typeof x.f === 'function'`, `x.f ? … : …`,
 *  `x.f && x.f()`, `if (x.f)`, `x.f ?? other`, `!x.f`. A plain call of a function that is gone throws, and the gate of starts and the checks see a throw;
 *  a value handed on (`const f = x.f`) throws where it is called. Nothing sees a branch quietly not taken. */
function softRead(n: any): boolean { let x = n.parent;
  if (!(x.kind === SK.PropertyAccessExpression && x.name === n) && !(x.kind === SK.ElementAccessExpression && x.argumentExpression === n)) return false;
  let par = x.parent; while (par.kind === SK.ParenthesizedExpression || par.kind === SK.NonNullExpression) { x = par; par = par.parent; }
  if (par.kind === SK.CallExpression) return par.expression === x && !!par.questionDotToken;
  if (par.kind === SK.TypeOfExpression) return true;
  if (par.kind === SK.PrefixUnaryExpression) return par.operator === SK.ExclamationToken;
  if (par.kind === SK.BinaryExpression) { const op = par.operatorToken.kind; return par.left === x && (op === SK.AmpersandAmpersandToken || op === SK.BarBarToken || op === SK.QuestionQuestionToken); }
  if (par.kind === SK.ConditionalExpression) return par.condition === x;
  if (par.kind === SK.IfStatement || par.kind === SK.WhileStatement || par.kind === SK.DoStatement) return par.expression === x;
  return false; }
/** A READ THAT COULD RUN THE FUNCTION: a call of it (`x.f(…)`, `x.f?.(…)`), a `.call` / `.apply` / `.bind` on it, or a test for it (softRead: the call
 *  follows the test). Any other read of a name off an untyped value looks at a value: level data, an options object, a pipeline. A method is not run by it. */
function runRead(n: any): boolean { if (softRead(n)) return true; let x = n.parent;
  if (!(x.kind === SK.PropertyAccessExpression && x.name === n) && !(x.kind === SK.ElementAccessExpression && x.argumentExpression === n)) return true;   // (a binding: the name goes on as a variable)
  let par = x.parent; while (par.kind === SK.ParenthesizedExpression || par.kind === SK.NonNullExpression) { x = par; par = par.parent; }
  if ((par.kind === SK.CallExpression || par.kind === SK.NewExpression || par.kind === SK.TaggedTemplateExpression) && (par.expression === x || par.tag === x)) return true;
  return par.kind === SK.PropertyAccessExpression && par.expression === x && /^(call|apply|bind)$/.test(par.name.text); }
/** every reference made inside `node`, but for the subtrees `skip` names */
function refs(node: any, out: Edge[], skip?: (n: any) => boolean): void {
  const sf = node.getSourceFile(), file = rel(sf.fileName);
  const add = (sym: any, at: any, how?: string) => { sym = targetOf(sym); if (!sym) return;
    for (const d of sym.declarations ?? []) {
      const u = unitOfDecl(d); if (u) out.push({ to: u, at: at.getStart(sf), file, how });
      // (a member of an interface or of a type written in place: whatever class is handed where that type is asked for answers the call)
      if ((d.kind === SK.MethodSignature || d.kind === SK.PropertySignature) && (d.parent.kind === SK.InterfaceDeclaration || d.parent.kind === SK.TypeLiteral)) out.push({ iface: d.parent, name: nameText(d.name), at: at.getStart(sf), file, soft: softRead(at) }); } };
  const loose = (name: string, n: any) => {
    if (n.parent.kind === SK.PropertyAccessExpression) {
      const r = castRooted(n); if (r === 'none') { STATS.castNone++; return; } if (r) { STATS.castTyped++; add(r, n, 'reads it off an object seen through a cast, by the type under the cast'); return; }
      const ch = anyChain(n); if (ch && ch.length >= 2) { const p = gamePath(ch); if (p) { STATS.pathTyped++; add(p, n, 'reads it off a value typed any, by a path that is the game\'s'); return; } } }
    out.push({ name, at: n.getStart(sf), file, soft: softRead(n), bare: !runRead(n) }); };
  const MV = modVars(sf), LOADS = 'reads it off a module it loads by a written path';
  /** the module an expression is: a variable that holds one, or the load itself */
  const modOf = (e: any): any => { while (e && (e.kind === SK.ParenthesizedExpression || e.kind === SK.NonNullExpression || e.kind === SK.AsExpression)) e = e.expression; if (e?.kind === SK.Identifier) { const sy = checker.getSymbolAtLocation(e); return sy ? MV.get(sy) : undefined; } return moduleOfExpr(e); };
  const visit = (n: any): void => {
    if (skip?.(n)) return;
    if (n.kind === SK.ImportDeclaration || n.kind === SK.ImportEqualsDeclaration) return;
    if (n.kind === SK.PropertyAccessExpression || n.kind === SK.ThisKeyword || n.kind === SK.NewExpression || n.kind === SK.CallExpression || n.kind === SK.ElementAccessExpression) handed(n, out, sf, file);
    // a module loaded by a written path is typed `any`: what is read off it is read by name, and the name is that module's export
    if (n.kind === SK.PropertyAccessExpression) { const mod = modOf(n.expression); if (mod) { const x = exportOf(mod, n.name.text); if (x) add(x, n.name, LOADS); ts.forEachChild(n.expression, visit); return; } }
    if (n.kind === SK.VariableDeclaration && n.name.kind === SK.ObjectBindingPattern && n.initializer) { const mod = modOf(n.initializer); if (mod) { for (const el of n.name.elements) { const nm = el.propertyName ?? el.name; if (nm.kind === SK.Identifier && !el.dotDotDotToken) { const x = exportOf(mod, nm.text); if (x) add(x, nm, LOADS); } if (el.initializer) visit(el.initializer); } visit(n.initializer); return; } }
    if (n.kind === SK.Identifier || n.kind === SK.PrivateIdentifier) {
      const p = n.parent;
      if (n.kind === SK.Identifier && ((p as any).name !== n || p.kind === SK.ShorthandPropertyAssignment) && p.kind !== SK.QualifiedName && p.kind !== SK.TypeReference && p.kind !== SK.ExpressionWithTypeArguments && p.kind !== SK.ImportSpecifier && p.kind !== SK.ExportSpecifier) handed(n, out, sf, file);
      // (a property of an object written where a type is asked for names that type's property: `{ typeOf: … }` handed as a part of a class is a use of the class's `typeOf`)
      if ((p.kind === SK.PropertyAssignment || p.kind === SK.ShorthandPropertyAssignment || p.kind === SK.MethodDeclaration) && p.name === n && p.parent.kind === SK.ObjectLiteralExpression) { const ct = checker.getContextualType(p.parent); const prop = ct && checker.getPropertyOfType(checker.getNonNullableType(ct), n.text); if (prop) add(prop, n, 'writes a property of that name where its type is asked for'); if (p.kind !== SK.ShorthandPropertyAssignment) return; }
      if (p.kind === SK.ShorthandPropertyAssignment && p.name === n) add(checker.getShorthandAssignmentValueSymbol(p), n);
      else if (p.kind === SK.BindingElement && p.name === n && !p.propertyName && p.parent.kind === SK.ObjectBindingPattern) { const t = checker.getTypeAtLocation(p.parent); const prop = t?.getProperty?.(n.text); if (prop) add(prop, n); else if (t && (t.flags & ts.TypeFlags.Any)) out.push({ name: n.text, at: n.getStart(sf), file }); }
      else { const sym = checker.getSymbolAtLocation(n);
        if (sym) add(sym, n);
        // (a name read off a value typed `any`: the checker cannot say whose it is)
        else if (p.kind === SK.PropertyAccessExpression && p.name === n) loose(n.text ?? String(n.escapedText), n);
        else if (p.kind === SK.BindingElement && p.propertyName === n) out.push({ name: n.text, at: n.getStart(sf), file }); }
      return; }
    if ((n.kind === SK.StringLiteral || n.kind === SK.NoSubstitutionTemplateLiteral) && n.parent.kind === SK.ElementAccessExpression && n.parent.argumentExpression === n) { const sym = checker.getSymbolAtLocation(n); if (sym) add(sym, n); else out.push({ name: n.text, at: n.getStart(sf), file, soft: softRead(n), bare: !runRead(n) }); return; }
    if (n.kind === SK.ElementAccessExpression && n.argumentExpression && n.argumentExpression.kind !== SK.StringLiteral && n.argumentExpression.kind !== SK.NumericLiteral) STATS.dyn++;
    ts.forEachChild(n, visit); };
  visit(node); }
// ---- class hierarchy: a call of a base's method is a call of every override; a class that says `implements I` answers I's members
const supers = new Map<Unit, Unit[]>(), subs = new Map<Unit, Unit[]>(), implementers = new Map<any, Unit[]>(), ifaceSubs = new Map<any, any[]>();
const push = <K, V>(m: Map<K, V[]>, k: K, v: V) => { const a = m.get(k); if (a) a.push(v); else m.set(k, [v]); };
for (const C of UNITS) if (C.kind === 'class') for (const h of C.nodes[0].heritageClauses ?? []) for (const t of h.types) { const s = targetOf(checker.getSymbolAtLocation(t.expression)); for (const d of s?.declarations ?? []) { const B = unitOfNode.get(d); if (B && B.kind === 'class') { push(supers, C, B); push(subs, B, C); } else if (d.kind === SK.InterfaceDeclaration) push(implementers, d, C); } }
for (const sf of program.getSourceFiles()) { if (sf.isDeclarationFile) continue; const v = (n: any): void => { if (n.kind === SK.InterfaceDeclaration) for (const h of n.heritageClauses ?? []) for (const t of h.types) { const s = targetOf(checker.getSymbolAtLocation(t.expression)); for (const d of s?.declarations ?? []) if (d.kind === SK.InterfaceDeclaration) push(ifaceSubs, d, n); } ts.forEachChild(n, v); }; for (const st of sf.statements) v(st); }
const allSubs = (C: Unit, out = new Set<Unit>()): Set<Unit> => { for (const s of subs.get(C) ?? []) if (!out.has(s)) { out.add(s); allSubs(s, out); } return out; };
const allSupers = (C: Unit, out = new Set<Unit>()): Set<Unit> => { for (const s of supers.get(C) ?? []) if (!out.has(s)) { out.add(s); allSupers(s, out); } return out; };
allSupersLazy = C => allSupers(C); allSubsLazy = C => allSubs(C);
/** the classes that SAY they implement an interface (or one that extends it), with their subclasses */
const saidCache = new Map<any, Set<Unit>>();
function saidImplementers(I: any): Set<Unit> { let s = saidCache.get(I); if (s) return s; s = new Set(); saidCache.set(I, s); const seen = new Set<any>(), go = (i: any) => { if (seen.has(i)) return; seen.add(i); for (const C of implementers.get(i) ?? []) { s!.add(C); for (const D of allSubs(C)) s!.add(D); } for (const j of ifaceSubs.get(i) ?? []) go(j); }; go(I); return s; }
const assignable = new Map<string, boolean>();
function answers(C: Unit, iface: any): boolean { const k = `${C.id}:${iface.pos}:${iface.getSourceFile().fileName}`; let v = assignable.get(k); if (v !== undefined) return v;
  try { const ct = checker.getDeclaredTypeOfSymbol(checker.getSymbolAtLocation(C.nodes[0].name)); const it = iface.kind === SK.InterfaceDeclaration ? checker.getDeclaredTypeOfSymbol(checker.getSymbolAtLocation(iface.name)) : checker.getTypeFromTypeNode(iface); v = !!checker.isTypeAssignableTo(ct, it); } catch { v = true; }
  assignable.set(k, v); return v; }

for (const u of UNITS) for (const n of u.nodes) {
  if (u.kind === 'class') { refs(n, u.sig, x => x.parent === n && ts.isClassElement(x)); continue; }
  const b = u.fnLike ? bodyOf(n) : undefined;
  if (!b) { refs(n, u.sig); continue; }
  refs(n, u.sig, x => x === b); refs(b, u.body); }

// ---- a function whose body goes keeps the type of its answer: where none is written, the checker's, written out
const TF = ts.TypeFormatFlags;
/** the function node of a unit that has no return type written and could carry one (a setter and a constructor answer nothing) */
const untyped = (n: any): any => { const f = fnOf(n); return f && bodyOf(n) && !f.type && n.kind !== SK.SetAccessor && n.kind !== SK.Constructor ? f : undefined; };
function retTypeOf(f: any): any { try { const sig = checker.getSignatureFromDeclaration(f); return sig ? checker.getReturnTypeOfSignature(sig) : undefined; } catch { return undefined; } }
/** the declarations a type names, to a small depth: they must stay for the written type to mean anything */
function typeDecls(t: any, out: Set<any>, d = 0, seen = new Set<any>()): void { if (!t || d > 4 || seen.has(t)) return; seen.add(t);
  for (const s of [t.aliasSymbol, t.symbol]) for (const x of s?.declarations ?? []) out.add(x);
  for (const a of t.aliasTypeArguments ?? []) typeDecls(a, out, d + 1, seen);
  for (const a of t.types ?? []) typeDecls(a, out, d + 1, seen);
  if (t.flags & ts.TypeFlags.Object) { try { for (const a of checker.getTypeArguments(t) ?? []) typeDecls(a, out, d + 1, seen); } catch { /* not a reference */ }
    if (!t.aliasSymbol && (!t.symbol || (t.symbol.flags & (ts.SymbolFlags.TypeLiteral | ts.SymbolFlags.ObjectLiteral)))) { for (const p of checker.getPropertiesOfType(t).slice(0, 60)) typeDecls(checker.getTypeOfSymbol(p), out, d + 1, seen); for (const sg of checker.getSignaturesOfType(t, ts.SignatureKind.Call)) { typeDecls(checker.getReturnTypeOfSignature(sg), out, d + 1, seen); for (const p of sg.parameters) typeDecls(checker.getTypeOfSymbol(p), out, d + 1, seen); } } } }
function retEdges(u: Unit): Edge[] { if (u.ret) return u.ret; u.ret = []; if (!u.fnLike) return u.ret;
  for (const n of u.nodes) { const f = untyped(n); if (!f) continue; const ds = new Set<any>(); typeDecls(retTypeOf(f), ds); for (const d of ds) { const to = unitOfDecl(d); if (to && to !== u) u.ret.push({ to, at: n.getStart(), file: u.file, how: 'is named by the type of its answer' }); } }
  return u.ret; }
/** the answer's type as text for that place in that file, or '' where it cannot be written there */
function retText(f: any, file: string): string { const t = retTypeOf(f); if (!t) return ''; let s: string; try { s = checker.typeToString(t, f, TF.NoTruncation); } catch { return ''; }
  if (/\bunique symbol\b|\.\.\. \d+ more \.\.\.|\btypeof this\b|\bthis\b/.test(s)) return '';
  // (a type the file has no name for is written through its module: the checker gives the module's whole path)
  s = s.replace(/import\("([^"]+)"\)/g, (_m, p: string) => { if (!p.startsWith('/')) return `import('${p}')`; let to = relative(dirname(join(ROOT, file)), p).split('\\').join('/'); if (!to.startsWith('.')) to = './' + to; return `import('${to}')`; });
  return s; }

// ---- who a file of the tree is: the page, a host, or a tool
type Who = 'page' | 'host' | 'tool';
const HOSTS = new Set<string>();
const resolveFrom = (from: string, spec: string): string | undefined => { const a = BRIDGE_ALIAS.exec(spec); let p = a ? join(ROOT, 'tools/blitz', a[1]) : spec.startsWith('.') ? resolve(dirname(join(ROOT, from)), spec) : undefined; if (!p) return undefined; for (const c of [p, p + '.ts', join(p, 'index.ts')]) { const r = rel(c); if (sfByRel.has(r)) return r; } return undefined; };
const importsOf = (file: string): string[] => { const sf = sfByRel.get(file), out: string[] = []; if (!sf) return out; const v = (n: any): void => {
      if ((n.kind === SK.ImportDeclaration || n.kind === SK.ExportDeclaration) && n.moduleSpecifier && !n.importClause?.isTypeOnly) { const r = resolveFrom(file, n.moduleSpecifier.text); if (r) out.push(r); }
      else if (n.kind === SK.CallExpression && n.expression.kind === SK.ImportKeyword && n.arguments[0] && ts.isStringLiteralLike(n.arguments[0])) { const r = resolveFrom(file, n.arguments[0].text); if (r) out.push(r); }
      ts.forEachChild(n, v); }; v(sf); return out; };
{ const todo = [...HOST_ENTRIES.filter(f => sfByRel.has(f)), ...[...sfByRel.keys()].filter(f => f.startsWith('src/'))], seen = new Set<string>(todo);
  while (todo.length) { const f = todo.pop()!; if (f.startsWith('tools/')) HOSTS.add(f); for (const i of importsOf(f)) if (!seen.has(i)) { seen.add(i); todo.push(i); } } }
const whoOf = (file: string): Who => file.startsWith('src/') ? 'page' : HOSTS.has(file) ? 'host' : 'tool';
/** THE HOST'S OWN LINES: the hosts of tools/, and the files of src/ that load the bridge (the page's host of the Rust game) with what they load beside
 *  them outside the simulation (the dress step). What these name runs where nothing crosses. */
const HOST_LINES = new Set<string>(HOSTS);
{ const bridgeUsers = [...sfByRel.keys()].filter(f => f.startsWith('src/') && !SIM.test(f) && sfByRel.get(f).statements.some((st: any) => st.kind === SK.ImportDeclaration && BRIDGE_ALIAS.test(st.moduleSpecifier.text)));
  for (const f of bridgeUsers) { HOST_LINES.add(f); for (const st of sfByRel.get(f).statements) if (st.kind === SK.ImportDeclaration && !st.importClause?.isTypeOnly && st.moduleSpecifier.text.startsWith('./')) { const r = rel(resolve(dirname(join(ROOT, f)), st.moduleSpecifier.text)) + '.ts'; if (sfByRel.has(r) && !SIM.test(r)) HOST_LINES.add(r); } } }
/** THE TYPES AN UNTYPED VALUE IS HANDED TO, outside the tools: an interface or a type written in place that some value typed `any` (or `unknown`, by a
 *  cast) is written where it is asked for. Only for these can a class answer "by its shape" without the checker having seen its object handed over
 *  (`host.skin: unknown`, then `skin as Partial<TweenSkin>`): everywhere else an object reaches such a type at a place the checker types on both
 *  sides, which `handed` reads exactly. */
const FED = new Set<any>();
{ const mark = (ct: any, d = 0): void => { if (!ct || d > 2) return; for (const T0 of ct.types ?? [ct]) { let T: any; try { T = checker.getNonNullableType(T0); } catch { continue; } if (!T || !(T.flags & (ts.TypeFlags.Object | ts.TypeFlags.Intersection))) continue;
      if (T.types) { mark(T, d + 1); continue; } for (const pr of checker.getPropertiesOfType(T)) for (const dd of pr.declarations ?? []) if ((dd.kind === SK.MethodSignature || dd.kind === SK.PropertySignature) && (dd.parent.kind === SK.InterfaceDeclaration || dd.parent.kind === SK.TypeLiteral)) FED.add(dd.parent); } };
  // (… and on from there: a value whose own type is one of those, written where ANOTHER type is asked for, carries the untyped object with it. Passes
  // until nothing is added.)
  const fedType = (t: any): boolean => { for (const T of t.types ?? [t]) { if (!T || !(T.flags & ts.TypeFlags.Object) || (T.symbol && classOfSym.has(T.symbol))) continue; for (const d of [...(T.symbol?.declarations ?? []), ...(T.aliasSymbol?.declarations ?? [])]) if (FED.has(d) || (d.kind === SK.TypeAliasDeclaration && FED.has(d.type))) return true; } return false; };
  const untyped = (e: any, cast: boolean): boolean => { try { const t = checker.getTypeAtLocation(e); return !!t && (!!(t.flags & (cast ? ts.TypeFlags.Any | ts.TypeFlags.Unknown : ts.TypeFlags.Any)) || (FED.size > 0 && fedType(t))); } catch { return false; } };
  for (let pass = 0, was = -1; pass < 4 && was !== FED.size; pass++) { was = FED.size; for (const sf of program.getSourceFiles()) { const file = rel(sf.fileName); if (sf.isDeclarationFile || file.startsWith('..') || whoOf(file) === 'tool') continue;
    const v = (n: any): void => { const p = n.parent;
      if ((n.kind === SK.AsExpression || n.kind === SK.TypeAssertionExpression) && n.type.kind !== SK.AnyKeyword && n.type.kind !== SK.UnknownKeyword) { let e = n.expression; while (e.kind === SK.AsExpression || e.kind === SK.TypeAssertionExpression || e.kind === SK.ParenthesizedExpression || e.kind === SK.NonNullExpression) e = e.expression; if (untyped(e, true)) { try { mark(checker.getTypeFromTypeNode(n.type)); } catch { /* no type */ } } }
      // (a value typed `any` written where a type is asked for: an argument, an initializer, the right of an assignment, an answer, a property of an object)
      else if (p && ((p.kind === SK.CallExpression || p.kind === SK.NewExpression) && p.arguments?.includes(n) || (p.kind === SK.VariableDeclaration || p.kind === SK.PropertyDeclaration || p.kind === SK.Parameter || p.kind === SK.PropertyAssignment) && p.initializer === n || p.kind === SK.BinaryExpression && p.right === n && p.operatorToken.kind === SK.EqualsToken || p.kind === SK.ReturnStatement || p.kind === SK.ArrowFunction && p.body === n || p.kind === SK.ArrayLiteralExpression || p.kind === SK.ShorthandPropertyAssignment)
        && n.kind !== SK.ObjectLiteralExpression && n.kind !== SK.ArrowFunction && n.kind !== SK.FunctionExpression && untyped(n, false)) { try { const ct = p.kind === SK.BinaryExpression ? checker.getTypeAtLocation(p.left) : checker.getContextualType(n); if (ct && !(ct.flags & (ts.TypeFlags.Any | ts.TypeFlags.Unknown))) mark(ct); } catch { /* no contextual type */ } }
      ts.forEachChild(n, v); }; v(sf); } }
  STATS.fed = FED.size; }
interface RootEdge { file: string; who: Who; e: Edge }
const ROOT_EDGES: RootEdge[] = [];
for (const sf of program.getSourceFiles()) { const file = rel(sf.fileName); if (SIM.test(file) || sf.isDeclarationFile || file.startsWith('..')) continue;
  const who = whoOf(file), out: Edge[] = []; refs(sf, out);
  // (an import of a name is a use of its declaration, whether the file goes on to use it or not: the file is not edited)
  for (const st of sf.statements) if (st.kind === SK.ImportDeclaration && st.importClause) { const c = st.importClause; const names = [c.name, ...(c.namedBindings?.elements?.map((x: any) => x.name) ?? [])].filter(Boolean);
    for (const n of names) { const s = targetOf(checker.getSymbolAtLocation(n)); for (const d of s?.declarations ?? []) { const u = unitOfDecl(d); if (u) out.push({ to: u, at: n.getStart(sf), file, how: 'imports it' }); } } }
  for (const e of out) ROOT_EDGES.push({ file, who, e }); }
/** the simulation's files that a tool writes: their first line says so */
const TOOL_WRITTEN = new Set<string>(simFiles.filter((sf: any) => /^\/\/ WRITTEN BY tools\//.test(sf.text)).map((sf: any) => rel(sf.fileName)));
/** THE TICK: its body is the Rust game's. */
const TICK = UNITS.find(u => u.key === 'Game.stepSeats' && u.file === 'src/game/game.ts');
if (!TICK || !GAME) { console.error('no Game.stepSeats in src/game/game.ts: this is not the tree the program was written for'); process.exit(2); }
say(`${UNITS.length} declarations, ${ROOT_EDGES.length} references from outside the simulation, ${HOSTS.size} host files of tools/; graph in ${((performance.now() - t0) / 1000).toFixed(1)} s`);

// ---- the census: what was seen to run as TypeScript while the Rust game stepped
interface Census { fingerprint: string; runs: string[]; seen: { file: string; key: string; n: number; page?: number; perFrame?: number }[] }
const CENSUS_FILE = opt('--census') ?? join(ROOT, 'tools/blitz/delete-census.json');
let CENSUS: Census | null = null; const SEEN = new Map<Unit, { n: number; page?: number; perFrame?: number }>();
if (flag('--exact-only')) CENSUS = { fingerprint: FINGERPRINT, runs: [], seen: [] };
else if (!flag('--no-census') && existsSync(CENSUS_FILE)) { CENSUS = JSON.parse(readFileSync(CENSUS_FILE, 'utf8'));
  if (CENSUS!.fingerprint !== FINGERPRINT && !flag('--stale-census')) { console.error(`the census (${rel(CENSUS_FILE)}) was taken on another tree (its simulation files hash to ${CENSUS!.fingerprint}, this tree's to ${FINGERPRINT}): take it again (tools/blitz/delete-census.sh), or --stale-census to use it by key, or --no-census`); process.exit(2); }
  const by = new Map<string, Unit>(); for (const u of UNITS) by.set(`${u.file}\n${u.key}`, u);
  for (const s of CENSUS!.seen) { const u = by.get(`${s.file}\n${s.key}`); if (u) SEEN.set(u, s); } }

/** KEPT BY HAND (tools/blitz/delete-keep.json, `[{ file, key, why }]`): a function the census had no play for and that must keep its body, with the reason written beside it. The way to mend a gap of the census; every row is printed in the note. */
const KEEP: { u: Unit; why: string }[] = [];
{ const f = join(ROOT, 'tools/blitz/delete-keep.json'); if (existsSync(f)) for (const k of JSON.parse(readFileSync(f, 'utf8'))) { const u = UNITS.find(x => x.file === k.file && x.key === k.key); if (!u) { console.error(`delete-keep.json names ${k.key} (${k.file}), which this tree has not got`); process.exit(2); } KEEP.push({ u, why: String(k.why) }); } }

// ---- reach
/** `--census-settles-all`: a loose reference made by the page or by the simulation's own kept code is settled by the census too, as a host's is. It deletes
 *  about a third more, and rests on the census having played every path that makes such a call, a draw's and a panel's too. Not the default. */
const CENSUS_ALL = flag('--census-settles-all');
// (each of the three readings below can be put off to see what it is worth: nothing else reads these words)
const SHAPE_BY_FLOW = !flag('--shape-anywhere'), MEMBERS_ONLY = !flag('--tops-as-members'), BARE_IS_DATA = !flag('--bare-runs');
interface Why { from: Unit | string; at: string; how: string; /** made by the signature of `from`, which stands at any reach */ sig?: boolean }
interface Plan { reach: Uint8Array; why: (Why | undefined)[]; why1: (Why | undefined)[]; /** the tools that name it exactly */ namedBy: Map<Unit, Set<string>>; /** where the page or a host reads its name loosely */ looseMiss: Map<Unit, Set<string>>; /** the tools that read its name off a value typed any */ looseTools: Map<Unit, Set<string>>; /** the places in tools that read a function with no row in a way that does not throw once it is gone */ softTools: Map<Unit, Set<string>> }
/** WHAT THE TICK REACHES: every declaration `Game.stepSeats` leads to by exact references, overrides and said interfaces. A function in here is the
 *  tick's own; one outside it (a level's builder, the level format, a lookup the menus use) is not, and the deletion of the tick has no claim on it. */
const OF_TICK = new Set<Unit>();
{ const q: Unit[] = [TICK!]; OF_TICK.add(TICK!); const add = (u?: Unit) => { if (u && !OF_TICK.has(u)) { OF_TICK.add(u); q.push(u); } };
  while (q.length) { const u = q.pop()!; for (const e of [...u.sig, ...u.body]) { if (e.to) add(e.to); else if (e.iface) { const said = saidImplementers(e.iface); for (const m of byName.get(e.name!) ?? []) if (m.cls && said.has(m.cls)) add(m); } }
    if (u.cls && (u.kind === 'method' || u.kind === 'accessor' || u.kind === 'field')) for (const D of allSubs(u.cls)) for (const m of D.members!) if (m.name === u.name && m.kind !== 'ctor') add(m); } }
/** REACH: 0 nothing · 1 its signature (the body goes) · 2 its body */
function plan(skipRoot?: (r: RootEdge) => boolean): Plan {
  const reach = new Uint8Array(UNITS.length), why: (Why | undefined)[] = new Array(UNITS.length), why1: (Why | undefined)[] = new Array(UNITS.length), queue: Unit[] = [], namedBy = new Map<Unit, Set<string>>(), looseMiss = new Map<Unit, Set<string>>(), looseTools = new Map<Unit, Set<string>>(), softTools = new Map<Unit, Set<string>>();
  const note = (m: Map<Unit, Set<string>>, u: Unit, v: string, cap = 1e9) => { let s = m.get(u); if (!s) m.set(u, s = new Set()); if (s.size < cap) s.add(v); };
  let SIG = false;
  const hit = (u: Unit, k: 1 | 2, from: Unit | string, at: string, how: string) => { if (!u.fnLike && k < 2) k = 2; if (reach[u.id] >= k) return; if (!reach[u.id]) why1[u.id] = { from, at, how, sig: SIG }; if (k >= 2 && reach[u.id] < 2) why[u.id] = { from, at, how, sig: SIG }; reach[u.id] = k; queue.push(u); };
  // A LOOSE REFERENCE names every declaration of that name (or every class that answers a type by its shape). WHO MAKES IT decides what it holds:
  //   the page, or the simulation's own kept code: every candidate, its body. Whatever the page can reach stays, by the checker alone: the census has
  //     seen ticks and starts and a few flows of the page, never every draw and every panel, and no headless check here draws.
  //   a host's own lines (the bridge's files: the apply, the export, the out events), which read the puppet by name through values typed `any` on every
  //     tick of every play: the census settles it. A candidate it saw run keeps its body; one it never saw is not held by the name (with a row on the
  //     surface it stays as a crossing), unless the read is one that TESTS for the function (softRead), which holds it as the page's would.
  //   a tool: nothing but a crossing where there is a row. The tool is listed, and the place where its read would not throw.
  const settled = (file: string) => !!CENSUS && (CENSUS_ALL || HOST_LINES.has(file));
  const loose = (u: Unit, e: Edge, k: 1 | 2, from: Unit | string, at: string, how: string) => {
    if (u.fnLike && settled(e.file) && !SEEN.has(u) && !e.soft) { note(looseMiss, u, at, 6); if (u.guard) hit(u, 1, from, at, how + ' (loose, from a host\'s own lines: the census never saw it run; it is on the surface, and crosses)'); return; }
    if (e.soft && settled(e.file)) STATS.softHeld++; hit(u, k, from, at, how + (e.soft ? ' (a read that tests for it)' : '')); };
  const toolLoose = (u: Unit, file: string, at: string, how: string, soft?: boolean) => { note(looseTools, u, file); if (soft && u.fnLike && !u.guard) note(softTools, u, at, 8); if (u.guard || !u.fnLike) hit(u, 1, file, at, how); };
  const follow = (e: Edge, k: 1 | 2, from: Unit | string, tool = false) => { const at = pos(e.file, e.at);
    if (e.to) { if (tool) note(namedBy, e.to, e.file); hit(e.to, k, from, at, e.how ?? 'names it'); return; }
    if (e.iface) { const said = saidImplementers(e.iface); for (const m of byName.get(e.name!) ?? []) { if (!m.cls || m.isStatic) continue;
        if (said.has(m.cls)) { if (tool) note(namedBy, m, e.file); hit(m, k, from, at, `calls ${e.name} on an interface its class implements`); }
        // (BY ITS SHAPE: only where an untyped value is handed to that type somewhere outside the tools (FED). Elsewhere the object got there at a place
        // the checker types, and `handed` has the exact reference.)
        else if (answers(m.cls, e.iface)) { const how = `${e.soft ? 'reads' : 'calls'} ${e.name} on a type its class answers by shape`; if (tool) toolLoose(m, e.file, at, how, e.soft); else if (FED.has(e.iface) || !SHAPE_BY_FLOW) loose(m, e, k, from, at, how + ', and an untyped value is handed to that type'); } } return; }
    // (OFF A VALUE TYPED `any`. A property of an object is never a module's own function, but for a host that reads a module by name: outside the
    // hosts the candidates are the classes' members. And a method is not run by a read that only looks at a value.)
    const host = HOST_LINES.has(e.file);
    for (const m of tool || host || !MEMBERS_ONLY ? looseOf(e.name!) : byName.get(e.name!) ?? []) { const how = `reads .${e.name} off a value typed any`; if (tool) toolLoose(m, e.file, at, how, e.soft); else if (e.bare && m.fnLike && BARE_IS_DATA && !host) continue; else loose(m, e, k, from, at, how); } };
  // A TOOL'S EXACT REFERENCE. A function on the surface keeps its signature and crosses. One with no row has no Rust function to cross to: where it is
  // not the tick's own it stays as it is; where it is the tick's own (a rule's inner step that a check calls by name) it goes with the tick, its
  // signature left to throw by name, and the tools that named it are listed.
  for (const r of ROOT_EDGES) { if (skipRoot?.(r)) continue; const tool = r.who === 'tool', t = r.e.to; follow(r.e, tool && (!t || t.guard || (t.fnLike && OF_TICK.has(t))) ? 1 : 2, r.file, tool); }
  for (const u of UNITS) if (u.kind === 'stmt' && !u.cls) hit(u, 2, 'the module', u.file, 'a top-level statement');
  // A FILE A TOOL WRITES (its first line says so: `// WRITTEN BY tools/… — never by hand`: the built module, a table written from a schema) is that
  // tool's, and its check reads the file's shape: nothing in it is edited, whatever reaches what
  for (const u of UNITS) if (TOOL_WRITTEN.has(u.file)) hit(u, 2, 'a file a tool writes', u.file, 'its file is written by a tool, never by hand');
  for (const [file, names] of Object.entries<string[]>(SCHEMA.top)) for (const n of new Set(names)) { const u = UNITS.find(x => x.file === file && x.name === n && !x.cls); if (u) hit(u, 2, 'the bridge', 'tools/blitz/schema.json', 'a top-level name the schema reads'); }
  // THE CENSUS ADDS, AND TAKES NOTHING AWAY BUT A HOST'S LOOSE NAME: what it saw run as TypeScript stays, however it was reached (a computed key, a
  // function stored and called by a table: what the checker cannot follow)
  for (const u of SEEN.keys()) if (u !== TICK) hit(u, 2, 'the census', rel(CENSUS_FILE), 'was seen to run as TypeScript while the Rust game stepped');
  for (const k of KEEP) hit(k.u, 2, 'kept by hand', 'tools/blitz/delete-keep.json', k.why);
  while (queue.length) { const u = queue.pop()!, k = reach[u.id] as 1 | 2;
    SIG = true;   // (what follows from a declaration standing at all, whatever its reach)
    if (u.cls) hit(u.cls, 2, u, u.file, 'its class');
    if (u.kind === 'class') for (const m of u.members!) if (m.kind === 'ctor' || m.kind === 'stmt' || (m.kind === 'field' && !m.fnLike)) hit(m, 2, u, u.file, 'the class as data');
    // (virtual dispatch: an override answers a call of what it overrides; an override that stays keeps the declaration it overrides)
    if (u.cls && (u.kind === 'method' || u.kind === 'accessor' || u.kind === 'field')) {
      for (const D of allSubs(u.cls)) for (const m of D.members!) if (m.name === u.name && m.kind !== 'ctor') hit(m, k, u, u.file, 'overrides it');
      for (const B of allSupers(u.cls)) for (const m of B.members!) if (m.name === u.name && m.kind !== 'ctor') hit(m, 1, u, u.file, 'is overridden by it'); }
    for (const e of u.sig) if (e.to !== u) follow(e, 2, u);   // (what a signature names is whole)
    if (k === 1) for (const e of retEdges(u)) follow(e, 2, u);   // (its body goes: the type of its answer is written in its place)
    SIG = false;   // (a declaration's own name is in its signature: not a use of it)
    if (k >= 2 && u !== TICK) for (const e of u.body) follow(e, 2, u); }
  if (reach[TICK!.id] < 1) reach[TICK!.id] = 1;
  return { reach, why, why1, namedBy, looseMiss, looseTools, softTools }; }
const P = plan();

// ---- the classes
type Class = 'kept' | 'kept+guard' | 'crossing' | 'gone' | 'deleted';
const classIn = (p: Plan, u: Unit): Class => u === TICK ? 'gone' : p.reach[u.id] === 0 ? 'deleted' : p.reach[u.id] >= 2 ? (u.guard ? 'kept+guard' : 'kept') : u.guard ? 'crossing' : 'gone';
const classOf = (u: Unit): Class => classIn(P, u);
const fnUnits = UNITS.filter(u => u.fnLike);

// ---- WHO STILL READS A FIELD (--readers out.tsv): for the cleanup of what the bridge carries onto the puppet every tick. A row a field, by the
// TypeScript's own name (`Class.field`, `Interface.field`, `TABLE.key`): where the code that is left after the deletion reads it, by the kind of reader,
// and where it is written. A reader in the simulation's folders counts where its function keeps a body in the plan; "seen on the page" says the census
// saw that function run in a page play. Read from the tree BEFORE the deletion (this program does not run on a tree it has been through).
if (opt('--readers')) {
  const kindOf = (file: string): string => SIM.test(file) ? 'sim' : /^src\/(render|gpu|smoke|dress)\/|^src\/engine\.ts$|^src\/host\/(rustdress|tween|ahead|loop|autocam)\.ts$/.test(file) ? 'picture' : /^src\/(audio|sounds|music)\//.test(file) ? 'sound' : /^src\/ui\/|^src\/host\/hud\.ts$/.test(file) ? 'hud' : /^src\/(editor|viewer|workbench|replay)\/|^src\/(workbench|viewerGizmo)\.ts$/.test(file) ? 'otherPages' : /^src\/(net|cloud)\//.test(file) ? 'net' : file.startsWith('src/') ? 'host' : HOSTS.has(file) ? 'bridge' : 'tool';
  const KINDS = ['picture', 'sound', 'hud', 'host', 'simKept', 'otherPages', 'net', 'bridge', 'tool', 'gone'] as const;
  interface Row { decl: string; n: Record<string, number>; files: Map<string, number>; seenOnPage: number; writesKept: number; writesGone: number; writeFiles: Set<string> }
  const rows = new Map<string, Row>();
  /** the name a field is declared under: its class, interface or type's name, or the table's, and the path of keys down to it */
  const ownerPath = (d: any): string | undefined => { const parts: string[] = []; let n = d;
    for (let hop = 0; n && hop < 12; hop++) { const nm = n.name && (n.name.kind === SK.Identifier || n.name.kind === SK.StringLiteral || n.name.kind === SK.PrivateIdentifier) ? n.name.text ?? String(n.name.escapedText) : undefined;
      if (n.kind === SK.PropertyDeclaration || n.kind === SK.PropertySignature || n.kind === SK.PropertyAssignment || n.kind === SK.ShorthandPropertyAssignment || n.kind === SK.GetAccessor || n.kind === SK.SetAccessor || n.kind === SK.Parameter || n.kind === SK.MethodSignature) { if (nm === undefined) return undefined; parts.unshift(nm); n = n.parent; if (n.kind === SK.Constructor) n = n.parent; continue; }
      if (n.kind === SK.ClassDeclaration || n.kind === SK.InterfaceDeclaration || n.kind === SK.TypeAliasDeclaration || n.kind === SK.VariableDeclaration || n.kind === SK.EnumDeclaration) { if (nm === undefined) return undefined; parts.unshift(nm); return parts.join('.'); }
      if (n.kind === SK.TypeLiteral || n.kind === SK.ObjectLiteralExpression || n.kind === SK.AsExpression || n.kind === SK.SatisfiesExpression || n.kind === SK.ParenthesizedExpression || n.kind === SK.ArrayType || n.kind === SK.UnionType || n.kind === SK.IntersectionType || n.kind === SK.ParenthesizedType || n.kind === SK.TypeReference || n.kind === SK.TypeOperator || n.kind === SK.ArrayLiteralExpression) { n = n.parent; continue; }
      return undefined; } return undefined; };
  const isField = (d: any) => d.kind === SK.PropertyDeclaration || d.kind === SK.PropertySignature || d.kind === SK.PropertyAssignment || d.kind === SK.ShorthandPropertyAssignment || d.kind === SK.GetAccessor || (d.kind === SK.Parameter && d.parent.kind === SK.Constructor && (d.modifiers?.length ?? 0) > 0);
  const note = (sym: any, at: any, sf: any, file: string, where: string, unit: Unit | undefined) => { sym = targetOf(sym); if (!sym) return;
    // (read, written, or both)
    let x = at.parent; if (x.kind !== SK.PropertyAccessExpression && x.kind !== SK.ElementAccessExpression) x = at; let par = x.parent; while (par && (par.kind === SK.ParenthesizedExpression || par.kind === SK.NonNullExpression)) { x = par; par = par.parent; }
    const asg = par?.kind === SK.BinaryExpression && par.left === x && par.operatorToken.kind >= SK.FirstAssignment && par.operatorToken.kind <= SK.LastAssignment, plain = asg && par.operatorToken.kind === SK.EqualsToken;
    const inc = (par?.kind === SK.PrefixUnaryExpression || par?.kind === SK.PostfixUnaryExpression) && (par.operator === SK.PlusPlusToken || par.operator === SK.MinusMinusToken);
    const reads = !plain, writes = asg || inc;
    for (const d of sym.declarations ?? []) { if (!isField(d)) continue; const dsf = d.getSourceFile(); if (dsf.isDeclarationFile) continue; const df = rel(dsf.fileName); if (!SIM.test(df)) continue; const key = ownerPath(d); if (!key) continue;
      let r = rows.get(key); if (!r) rows.set(key, r = { decl: df, n: Object.fromEntries(KINDS.map(k => [k, 0])), files: new Map(), seenOnPage: 0, writesKept: 0, writesGone: 0, writeFiles: new Set() });
      if (reads) { r.n[where]++; if (where !== 'gone' && where !== 'tool' && where !== 'bridge') r.files.set(file, (r.files.get(file) ?? 0) + 1); if (where === 'simKept' && unit && SEEN.get(unit)?.page) r.seenOnPage++; }
      if (writes) { if (where === 'gone') r.writesGone++; else if (where !== 'tool') { r.writesKept++; r.writeFiles.add(file); } } } };
  const walk = (node: any, sf: any, file: string, where: string, unit: Unit | undefined, skip?: (n: any) => boolean) => { const v = (n: any): void => { if (skip?.(n)) return;
      if (n.kind === SK.Identifier || n.kind === SK.PrivateIdentifier || n.kind === SK.StringLiteral) { const p = n.parent;
        const isName = (p.kind === SK.PropertyAccessExpression && p.name === n) || (p.kind === SK.ElementAccessExpression && p.argumentExpression === n) || (p.kind === SK.BindingElement && (p.propertyName === n || (p.name === n && !p.propertyName && p.parent.kind === SK.ObjectBindingPattern)));
        if (isName) { let sym: any; try { sym = p.kind === SK.BindingElement && p.name === n ? checker.getTypeAtLocation(p.parent)?.getProperty?.(n.text) : checker.getSymbolAtLocation(n); } catch { sym = undefined; }
          if (!sym && p.kind === SK.PropertyAccessExpression) { const r = castRooted(n); if (r && r !== 'none') sym = r; else { const ch = anyChain(n); if (ch && ch.length >= 2) sym = gamePath(ch); } }
          if (sym) note(sym, n, sf, file, where, unit); } return; }
      ts.forEachChild(n, v); }; v(node); };
  // the simulation's own code, a declaration at a time: a reader there survives where its declaration keeps a body
  for (const u of UNITS) for (const n of u.nodes) { const sf = n.getSourceFile(), file = rel(sf.fileName), kept = P.reach[u.id] >= 2;
    if (u.kind === 'class') { walk(n, sf, file, 'simKept', u, x => x.parent === n && ts.isClassElement(x)); continue; }
    const b = u.fnLike ? bodyOf(n) : undefined; if (!b) { walk(n, sf, file, kept || !u.fnLike ? 'simKept' : 'gone', u); continue; }
    walk(n, sf, file, 'simKept', u, x => x === b); walk(b, sf, file, kept && u !== TICK ? 'simKept' : 'gone', u); }
  for (const sf of program.getSourceFiles()) { const file = rel(sf.fileName); if (SIM.test(file) || sf.isDeclarationFile || file.startsWith('..')) continue; walk(sf, sf, file, kindOf(file), undefined); }
  const cols = ['field', 'declared in', 'read by surviving code', 'picture', 'sound', 'hud or menu', 'page host', 'simulation code that stays', 'of those, seen running in a page play', 'other pages', 'net', 'the bridge', 'tools', 'code that went', 'written by code that stays', 'first files that read it', 'files that write it'];
  const out = [cols.join('\t')]; let yes = 0;
  for (const [key, r] of [...rows].sort((a, b) => a[0] < b[0] ? -1 : 1)) { const live = r.n.picture + r.n.sound + r.n.hud + r.n.host + r.n.simKept + r.n.otherPages + r.n.net; if (live) yes++;
    out.push([key, r.decl, live ? 'yes' : 'no', r.n.picture, r.n.sound, r.n.hud, r.n.host, r.n.simKept, r.seenOnPage, r.n.otherPages, r.n.net, r.n.bridge, r.n.tool, r.n.gone, r.writesKept, [...r.files].sort((a, b) => b[1] - a[1]).slice(0, 3).map(([f, c]) => `${f} (${c})`).join(', '), [...r.writeFiles].slice(0, 3).join(', ')].join('\t')); }
  writeFileSync(opt('--readers')!, out.join('\n') + '\n'); say(`the readers: ${rows.size} fields declared in the simulation's folders, ${yes} read by code that stays, ${rows.size - yes} read by none → ${opt('--readers')}`); process.exit(0); }
const count = (us: Unit[], p = P) => { const c: Record<string, [number, number]> = {}; for (const u of us) { const k = classIn(p, u); (c[k] ??= [0, 0])[0]++; c[k][1] += u.lines; } return c; };
/** the references that hold a declaration, one after another back to a root. A step made by a signature goes on from why that signature stands at all. */
function steps(u: Unit): { u: Unit; w?: Why }[] { const out: { u: Unit; w?: Why }[] = []; let x: Unit = u, top = true; const seen = new Set<string>();
  for (;;) { const w = top ? (P.why[x.id] ?? P.why1[x.id]) : P.why1[x.id]; out.push({ u: x, w }); if (!w || typeof w.from === 'string') return out; const k = `${w.from.id}:${w.sig ? 1 : 2}`; if (seen.has(k)) return out; seen.add(k); x = w.from; top = !w.sig; } }
const chain = (u: Unit): string[] => steps(u).map(({ u: x, w }) => `${x.key} (${x.file}) ← ${w ? `${typeof w.from === 'string' ? w.from : w.from.key} ${w.how}${w.sig ? ' by standing at all' : ''} at ${w.at}` : 'nothing reaches it'}`);
/** the root a kept declaration hangs from: the file (or 'the module', 'the bridge', 'the census') at the end of its chain */
const rootOf = (u: Unit): string => { const s = steps(u), w = s[s.length - 1].w; return w && typeof w.from === 'string' ? w.from : '?'; };

function summary(): string[] { const L: string[] = [];
  L.push(`tree ${FINGERPRINT} · census: ${CENSUS ? `${SEEN.size} functions seen in ${CENSUS.runs.length} runs` : 'NONE (every loose reference followed)'}`);
  L.push(`functions of the simulation's folders: ${fnUnits.length} (${fnUnits.reduce((a, u) => a + u.lines, 0)} lines)`);
  const c = count(fnUnits); for (const k of ['deleted', 'crossing', 'gone', 'kept+guard', 'kept']) L.push(`  ${k.padEnd(12)} ${String(c[k]?.[0] ?? 0).padStart(6)} functions ${String(c[k]?.[1] ?? 0).padStart(7)} lines`);
  const o = count(UNITS.filter(u => !u.fnLike)); L.push(`other declarations: ${UNITS.length - fnUnits.length}: kept ${o.kept?.[0] ?? 0} (${o.kept?.[1] ?? 0} lines), deleted ${o.deleted?.[0] ?? 0} (${o.deleted?.[1] ?? 0} lines)`);
  const kept = fnUnits.filter(u => P.reach[u.id] >= 2), byRoot = new Map<string, [number, number]>(); for (const u of kept) { const r = rootOf(u), w = r.startsWith('src/') ? 'the page' : r.startsWith('tools/') ? 'the hosts' : r; const a = byRoot.get(w) ?? [0, 0]; a[0]++; a[1] += u.lines; byRoot.set(w, a); }
  L.push(`kept with a body, by the root of the first chain found: ${[...byRoot].sort((a, b) => b[1][0] - a[1][0]).map(([r, [n, l]]) => `${r} ${n} (${l} lines)`).join(' · ')}`);
  L.push(`loose candidates left as a signature: ${P.looseMiss.size} · names read off any and typed as a path from the game: ${STATS.pathTyped}, through a cast by the type under it: ${STATS.castTyped} (${STATS.castNone} of a name the class has not got) · types an untyped value is handed to: ${STATS.fed} · a host's reads that test for a function, held: ${STATS.softHeld}; in tools, listed: ${P.softTools.size} · element reads by a computed key (not followed): ${STATS.dyn}`);
  return L; }


// ---- the writer
/** THE MODULE THE STUBS CALL. It names no other module: a stub is one line of a function's body, never a top-level use (src/game is one import cycle). */
const CROSSING_TS = `// THE CROSSING FROM A TYPESCRIPT NAME TO THE RUST GAME'S RULE (tools/blitz/DELETION.md). The simulation steps in Rust (rust/game); what is left of it
// in TypeScript is the puppet's data, the start of a game, the picture's readers, and these names. A function whose TypeScript went with the tick and that
// something outside the tick still calls is one line: \`return crossToRust('Class.method', this, [args]) as any\`. A function that kept its body and is on the
// call surface opens with the surface's guard: between two ticks under a host the call crosses, anywhere else its TypeScript runs. Written by
// tools/blitz/delete-tick.ts; the surface itself (rows by key, the bytes, the host) is tools/blitz's and is handed in by \`setSurface\`. No surface, as on
// the page: a guard passes, a tap never fires, and a crossing throws by name.
export const RUST_PASS: unique symbol = Symbol('run the TypeScript');
export interface Surface {
  /** hand the call to the Rust game: its answer, or RUST_PASS where it does not cross now (no host stands, the host's own lines are running, a row Rust has not got) */
  call(key: string, self: unknown, args: unknown[]): unknown;
  /** a tool's replacement of this function is calling through to the original: answer from the record */
  tapFiring(key: string): boolean; tapAnswer(): unknown;
  /** a new game is standing: nothing crosses to the last game's host */
  newGame(): void;
}
export let RUST_SURFACE: Surface | null = null;
export function setSurface(s: Surface | null): void { RUST_SURFACE = s; }
/** the body of a rule only the Rust game has */
export function crossToRust(key: string, self: unknown, args: unknown[]): unknown {
  if (!RUST_SURFACE) throw new Error(\`\${key}: a rule that has no TypeScript body was called where no call surface is installed (the page, or a tool run without the bridge's preload): only the Rust game has it. If a start or a menu needs it as TypeScript, the cure is a row in tools/blitz/delete-keep.json and the deletion run again (tools/blitz/DELETION.md)\`);
  const r = RUST_SURFACE.call(key, self, args);
  if (r === RUST_PASS) throw new Error(\`\${key}: a start called a rule that has no TypeScript body. Only the Rust game has it, and this call did not cross: no host of a Rust game stands for it yet (a start), or it was made inside the host's own lines (an apply, the dress step), or on a row the Rust game has not got. The census of starts never saw this call (tools/blitz/starts.json); the cure is to add the start that makes it to that table, or a row in tools/blitz/delete-keep.json, and run the deletion again (tools/blitz/DELETION.md)\`);
  return r; }
/** the body of a function that went with the tick and has no row on the call surface */
export function goneWithTick(key: string): never { throw new Error(\`\${key}: this function's TypeScript went with the tick, and the call surface has no row for it (tools/blitz/DELETION.md lists who named it)\`); }
`;
interface Ed { s: number; e: number; t: string }
type Mode = 'delete' | 'instrument';
function write(mode: Mode): { files: number; text: Map<string, string>; ids?: unknown } {
  const classNow = (u: Unit): Class => mode === 'instrument' ? (u.guard ? 'kept+guard' : 'kept') : classOf(u);
  const edits = new Map<string, Ed[]>(), uses = new Map<string, Set<string>>(), mustDrop = new Map<string, Set<string>>();
  // A MODULE THAT MUST STILL LOAD: one the bridge reads top-level names from (its window is a line added as the file loads: a file nobody imports has no
  // window, and the start's export throws), or one with top-level statements of its own. Where every name of an import of such a module goes, the
  // import stays as a bare `import './x'`, so the modules that load, and their order, are the ones that loaded before.
  const loads = new Set<string>(Object.keys(SCHEMA.top)); for (const u of UNITS) if (u.kind === 'stmt' && !u.cls) loads.add(u.file);
  const mustLoad = new Map<string, Set<string>>();
  for (const sf of simFiles) { const file = rel(sf.fileName); for (const st of sf.statements) { if (st.kind !== SK.ImportDeclaration || !st.importClause || st.importClause.isTypeOnly) continue;
      const mod = checker.getSymbolAtLocation(st.moduleSpecifier), to = mod?.declarations?.[0]?.getSourceFile?.(); if (!to || !loads.has(rel(to.fileName))) continue;
      const c = st.importClause, names = [c.name, c.namedBindings?.kind === SK.NamespaceImport ? c.namedBindings.name : undefined, ...(c.namedBindings?.elements?.filter((x: any) => !x.isTypeOnly).map((x: any) => x.name) ?? [])].filter(Boolean);
      if (names.some((n: any) => { const sy = targetOf(checker.getSymbolAtLocation(n)); return !!sy && !!(sy.flags & ts.SymbolFlags.Value); })) { let m = mustLoad.get(file); if (!m) mustLoad.set(file, m = new Set()); m.add(st.moduleSpecifier.text); } } }
  const ed = (file: string, s: number, e: number, t: string) => { let a = edits.get(file); if (!a) edits.set(file, a = []); a.push({ s, e, t }); };
  const use = (file: string, ...names: string[]) => { let a = uses.get(file); if (!a) uses.set(file, a = new Set()); for (const n of names) a.add(n); };
  const q = (s: string) => `'${s.replace(/\\/g, '\\\\').replace(/'/g, "\\'")}'`;
  /** where a declaration's own text begins, with the comment that sits on it; and where it ends, with a comment that trails it on its line */
  const span = (n: any, keepHead: boolean): [number, number] => { const sf = n.getSourceFile(), text: string = sf.text; let s = n.getStart(sf); const full = n.getFullStart();
    if (!keepHead) { const cr: any[] = ts.getLeadingCommentRanges(text, full) ?? [];
      for (let i = cr.length - 1; i >= 0; i--) { if ((text.slice(cr[i].end, s).match(/\n/g)?.length ?? 0) >= 2) break; if (full > 0 && !text.slice(full, cr[i].pos).includes('\n')) break; s = cr[i].pos; } }
    while (s > 0 && (text[s - 1] === ' ' || text[s - 1] === '\t')) s--; if (s > 0 && text[s - 1] === '\n') s--; if (s > 0 && text[s - 1] === '\r') s--;
    let e = n.end; const tr: any[] = ts.getTrailingCommentRanges(text, e) ?? []; if (tr.length) e = tr[tr.length - 1].end;
    return [s, e]; };
  const drop = (n: any) => { const sf = n.getSourceFile(), first = n.parent.kind === SK.SourceFile && sf.statements[0] === n; const [s, e] = span(n, first); ed(rel(sf.fileName), s, e, ''); };
  const dead = (u: Unit) => classNow(u) === 'deleted';
  let nIds = 0;
  for (const u of UNITS) {
    if (u.cls && dead(u.cls)) continue;
    const c = classNow(u), file = u.file;
    if (c === 'deleted') {
      if (u.kind !== 'var') { for (const n of u.nodes) drop(n); continue; }
      continue; }   // (a `var` goes with its statement, below)
    if (!u.fnLike) continue;
    const impl = implOf(u); if (!impl) continue;   // (declared only: an abstract method, an overload list with no body)
    const sf = impl.getSourceFile(), tapLine = u.tap ? ` if (RUST_SURFACE?.tapFiring(${q(u.tap.key)})) return RUST_SURFACE.tapAnswer() as any;` : '';
    if (u.tap) use(file, 'RUST_SURFACE');
    const self = u.guard?.method ? 'this' : 'undefined', call = u.row ? `(${q(stubKey(u.row))}, ${self}, [${u.guard.params.join(', ')}])` : '';
    if (c === 'crossing' || c === 'gone') {
      for (const n of u.nodes) { const b = bodyOf(n); if (!b) continue; const f = fnOf(n), isSet = n.kind === SK.SetAccessor, never = f.type?.kind === SK.NeverKeyword;
        // (the answer's type, where the source left it to the body that is going)
        { const uf = untyped(n), ty = uf ? retText(uf, file) : ''; if (ty) { let at = (f.equalsGreaterThanToken ?? b).getStart(sf); while (at > 0 && /\s/.test(sf.text[at - 1])) at--; if (sf.text[at - 1] === ')') ed(file, at, at, `: ${ty}`); } }
        const expr = c === 'crossing' ? `crossToRust${call} as any` : never || isSet ? `goneWithTick(${q(u.key)})` : `goneWithTick(${q(u.key)}) as any`;
        use(file, c === 'crossing' ? 'crossToRust' : 'goneWithTick');
        if (b.kind === SK.Block) ed(file, b.getStart(sf), b.end, isSet ? `{ ${expr}; }` : `{${tapLine} return ${expr}; }`);
        else ed(file, b.getStart(sf), b.end, expr); }
      continue; }
    // kept: the surface's lines at the top of the body, where it has any; the census's counter
    const guardLine = c === 'kept+guard' ? ` { const __s = RUST_SURFACE; if (__s) { const __r = __s.call${call}; if (__r !== RUST_PASS) return __r as any; } }` : '';
    if (guardLine) use(file, 'RUST_SURFACE', 'RUST_PASS');
    const counter = mode === 'instrument' ? ` (globalThis as any).__tpsRan?.(${u.id});` : ''; if (counter) nIds++;
    const top = tapLine + guardLine + counter; if (!top) continue;
    for (const n of u.nodes) { const b = bodyOf(n); if (!b) continue;
      if (b.kind === SK.Block) ed(file, b.getStart(sf) + 1, b.getStart(sf) + 1, top);
      else if (tapLine || guardLine) ed(file, b.getStart(sf), b.end, `{${top} return ${b.getText(sf)}; }`);
      else ed(file, b.getStart(sf), b.end, `((globalThis as any).__tpsRan?.(${u.id}), ${b.getText(sf)})`); } }
  // a variable statement: whole where every name of it goes, else the runs of names that go
  for (const sf of simFiles) { const file = rel(sf.fileName); for (const st of sf.statements) { if (st.kind !== SK.VariableStatement) continue; const ds: any[] = st.declarationList.declarations, gone_ = ds.map(d => { const u = unitOfNode.get(d); return !!u && dead(u); });
      if (gone_.every(Boolean)) { drop(st); continue; } if (!gone_.some(Boolean)) continue;
      for (let i = 0; i < ds.length; i++) { if (!gone_[i]) continue; let j = i; while (j + 1 < ds.length && gone_[j + 1]) j++; if (i === 0) ed(file, ds[0].getStart(sf), ds[j + 1].getStart(sf), ''); else ed(file, ds[i - 1].end, ds[j].end, ''); i = j; } } }
  // `Game`'s constructor says a new game is standing (expose.ts wrote this line at load)
  { const ctor = GAME!.members!.find(m => m.kind === 'ctor'), b = ctor?.nodes[0].body; if (!b) throw new Error('no constructor of Game with a body'); ed(GAME!.file, b.getStart() + 1, b.getStart() + 1, ' RUST_SURFACE?.newGame();'); use(GAME!.file, 'RUST_SURFACE'); }
  // an export list, or an import, that names a declaration which went
  for (const sf of simFiles) { const file = rel(sf.fileName); for (const st of sf.statements) {
      if (st.kind === SK.ExportDeclaration && st.exportClause?.kind === SK.NamedExports) { const els: any[] = st.exportClause.elements, keep = els.filter(x => { const s = targetOf(checker.getSymbolAtLocation(x.name)); const us = (s?.declarations ?? []).map(unitOfDecl).filter(Boolean) as Unit[]; return !us.length || us.some(u => !dead(u) && !(u.cls && dead(u.cls))); });
        if (keep.length === els.length) continue; if (!keep.length) drop(st); else ed(file, st.exportClause.getStart(sf), st.exportClause.end, `{ ${keep.map(x => x.getText(sf)).join(', ')} }`); }
      if (st.kind === SK.ImportDeclaration && st.importClause) { const c = st.importClause; for (const n of [c.name, ...(c.namedBindings?.elements?.map((x: any) => x.name) ?? [])].filter(Boolean)) { const s = targetOf(checker.getSymbolAtLocation(n)); const us = (s?.declarations ?? []).map(unitOfDecl).filter(Boolean) as Unit[]; if (us.length && us.every(u => dead(u) || !!(u.cls && dead(u.cls)))) { let m = mustDrop.get(file); if (!m) mustDrop.set(file, m = new Set()); m.add(n.text); } } } } }
  // apply, from the end of each file
  const out = new Map<string, string>();
  for (const [file, E] of edits) { const sf = sfByRel.get(file); let text: string = sf.text; E.sort((a, b) => b.s - a.s || b.e - a.e);
    for (let i = 0; i + 1 < E.length; i++) if (E[i + 1].e > E[i].s) throw new Error(`${file}: two edits overlap at ${lineOf(sf, E[i + 1].s)} and ${lineOf(sf, E[i].s)}`);
    for (const e of E) text = text.slice(0, e.s) + e.t + text.slice(e.e);
    out.set(file, text); }
  // second pass, on the new text: an import nothing reads any more goes, and the stubs' own import comes
  for (const [file, text0] of out) { let text = text0; const sf = ts.createSourceFile(file, text, ts.ScriptTarget.ES2022, true), used = new Set<string>(), must = mustDrop.get(file) ?? new Set<string>();
    const v = (n: any): void => { if (n.kind === SK.ImportDeclaration) return; if (n.kind === SK.Identifier) used.add(n.text); ts.forEachChild(n, v); }; v(sf);
    const E: Ed[] = []; let lastImport = 0;
    for (const st of sf.statements) { if (st.kind !== SK.ImportDeclaration) continue; lastImport = st.end; const c = st.importClause; if (!c) continue;
      const live = (n: any) => !must.has(n.text) && used.has(n.text); const def = c.name && live(c.name) ? c.name.text : '', nb = c.namedBindings;
      const ns = nb?.kind === SK.NamespaceImport && live(nb.name) ? `* as ${nb.name.text}` : '', els: any[] = nb?.kind === SK.NamedImports ? nb.elements.filter((x: any) => live(x.name)) : [];
      const had = (c.name ? 1 : 0) + (nb?.kind === SK.NamespaceImport ? 1 : 0) + (nb?.kind === SK.NamedImports ? nb.elements.length : 0), have = (def ? 1 : 0) + (ns ? 1 : 0) + els.length;
      if (have === had) continue;
      if (!have) { if (mustLoad.get(file)?.has(st.moduleSpecifier.text)) { E.push({ s: st.getStart(sf), e: st.end, t: `import ${st.moduleSpecifier.getText(sf)};` }); continue; } const [s, e] = span(st, sf.statements[0] === st); E.push({ s, e, t: '' }); continue; }
      const parts = [def, ns, els.length ? `{ ${els.map((x: any) => x.getText(sf)).join(', ')} }` : ''].filter(Boolean).join(', ');
      E.push({ s: c.getStart(sf), e: c.end, t: `${c.isTypeOnly ? 'type ' : ''}${parts}` }); }
    E.sort((a, b) => b.s - a.s || b.e - a.e); for (const e of E) text = text.slice(0, e.s) + e.t + text.slice(e.e);
    // (the stubs' import, placed on the text as it now is: after the last import left, or before the first statement)
    const need = uses.get(file); if (need?.size) { const own = sfByRel.get(file).identifiers as Map<string, string>; for (const n of need) if (own?.has(n)) throw new Error(`${file} has a name of its own spelt ${n}: the stubs' import would collide with it`);
      let to = relative(dirname(file), CROSSING.replace(/\.ts$/, '')).split('\\').join('/'); if (!to.startsWith('.')) to = './' + to; const line = `import { ${[...need].sort().join(', ')} } from '${to}';`;
      const sf2 = ts.createSourceFile(file, text, ts.ScriptTarget.ES2022, true), imports = sf2.statements.filter((x: any) => x.kind === SK.ImportDeclaration), last = imports[imports.length - 1];
      if (last) { let at = last.end; const tr: any[] = ts.getTrailingCommentRanges(text, at) ?? []; if (tr.length) at = tr[tr.length - 1].end; text = text.slice(0, at) + `\n${line}` + text.slice(at); }
      else { const at = sf2.statements.length ? sf2.statements[0].getStart(sf2) : text.length; text = text.slice(0, at) + `${line}\n` + text.slice(at); } }
    out.set(file, text); }
  out.set(CROSSING, CROSSING_TS);
  return { files: out.size, text: out, ids: mode === 'instrument' ? { fingerprint: FINGERPRINT, n: nIds, units: UNITS.filter(u => u.fnLike).map(u => [u.id, u.file, u.key]) } : undefined }; }

/** THE SURFACE'S KEYS FOR THE STUBS (tools/blitz/crossing-keys.ts, written on --write): the rows and taps by key, handed to src/game/crossing.ts */
const KEYS_TS = `// THE CALL SURFACE BY KEY, FOR THE NAMES LEFT IN THE SIMULATION'S SOURCE (src/game/crossing.ts; tools/blitz/DELETION.md). A stub and a guard name their row by
// its key; the surface (surface.ts, and expose.ts's stand-in before a game is stood) takes a row's number. This file is the table between the two, read from
// calls.json and taps.json as they stand, and it answers through whatever \`globalThis.__tpsCall\` is at the moment of the call, as the lines expose.ts used
// to write at load did. Loaded by the bridge's preload wherever a word asks for the Rust game. Written by tools/blitz/delete-tick.ts.
import { readFileSync } from 'node:fs';
import { setSurface, RUST_PASS } from '../../src/game/crossing.ts';
const CALLS = JSON.parse(readFileSync(new URL('./calls.json', import.meta.url).pathname, 'utf8')), TAPS: any[] = JSON.parse(readFileSync(new URL('./taps.json', import.meta.url).pathname, 'utf8'));
const n = new Map<string, number>(); for (const r of CALLS.rows) n.set(r.key, (n.get(r.key) ?? 0) + 1);
const ROW = new Map<string, number>(); for (const r of CALLS.rows) ROW.set(r.stub ?? (n.get(r.key)! > 1 ? \`\${r.key}@\${r.file}\` : r.key), r.id);
const TAP = new Map<string, number>(); for (const t of TAPS) TAP.set(t.key, t.id);
const G = globalThis as any;
setSurface({
  call(key, self, args) { const c = G.__tpsCall; if (!c) return RUST_PASS; const row = ROW.get(key); if (row === undefined) throw new Error(\`\${key}: calls.json has no row of that key\`); const r = c(row, self, args); return r === c.PASS ? RUST_PASS : r; },
  tapFiring(key) { const c = G.__tpsCall, id = TAP.get(key); return id !== undefined && !!c?.tapFiring?.(id); },
  tapAnswer() { return G.__tpsCall.tapAnswer(); },
  newGame() { G.__tpsCall?.newGame?.(); },
});
`;
/** THE CENSUS'S COUNTER (tools/blitz/delete-census-preload.ts in the census's tree only) */
const CENSUS_PRELOAD_TS = `// THE CENSUS'S COUNTER, in the census's tree only (tools/blitz/delete-tick.ts --instrument). Every function of the simulation's folders opens with
// \`__tpsRan?.(id)\`; this counts, and as the process ends adds one line to $TPS_RAN_OUT: who ran, and how often.
import { appendFileSync } from 'node:fs';
const N = new Map<number, number>(); (globalThis as any).__tpsRan = (id: number) => { N.set(id, (N.get(id) ?? 0) + 1); };
const out = process.env.TPS_RAN_OUT; let said = false;
const dump = () => { if (said || !out || !N.size) return; said = true; try { appendFileSync(out, JSON.stringify({ argv: process.argv.slice(1).join(' ').slice(0, 300), word: process.env.TPS_GAME ?? '', ran: [...N] }) + '\\n'); } catch { /* the census is not the tool's */ } };
process.on('exit', dump); process.on('beforeExit', dump);
`;
function patch(file: string, find: string | RegExp, to: string, text: Map<string, string>): void { const p = join(ROOT, file), was = text.get(file) ?? readFileSync(p, 'utf8'), now = was.replace(find, to); if (now === was) throw new Error(`${file} no longer holds ${String(find)}: the deletion needs that line changed`); text.set(file, now); }
function writeOut(mode: Mode): void {
  if (existsSync(join(ROOT, CROSSING))) { console.error(`${CROSSING} is there: this tree has been through the program already. Run it on a tree that still has the tick.`); process.exit(2); }
  const W = write(mode), T = W.text;
  // the surface's lines are in the source now: the table's places (which expose.ts wrote them by, and which every edit above has moved) go
  { const c = JSON.parse(readFileSync(join(ROOT, 'tools/blitz/calls.json'), 'utf8')); c.files = {}; c.taps = {}; c.counted = {}; T.set('tools/blitz/calls.json', JSON.stringify(c)); }
  T.set('tools/blitz/crossing-keys.ts', KEYS_TS);
  patch('tools/blitz/preload.ts', "await import('./expose.ts');", "await import('./expose.ts'); await import('./crossing-keys.ts');", T);
  // THE RUST GAME IS THE ONLY GAME (not in the census's tree, which plays under the word as the tree before it does): the page loads its host with no
  // word, behind the card a browser with no WebAssembly already gets, and a headless tool's `standUp` stands the Rust game with no word
  if (mode === 'delete') {
    // (the stubs' own module has one module variable: its row in the list of what a rollback snapshot need not hold)
    patch('tools/qa/rollback-exempt.ts', "export const ROLLBACK_EXEMPT: Record<string, string> = {\n", "export const ROLLBACK_EXEMPT: Record<string, string> = {\n  'src/game/crossing.ts RUST_SURFACE': 'CONFIG · the call surface a tool\\'s preload hands in once, before any game is stood (tools/blitz/crossing-keys.ts); null in the page; no rule reads it, a stub calls through it',\n", T); }
  if (mode === 'delete' && !flag('--keep-word')) {
    // THE WORD GOES. The Rust game is the page's game whatever the address says. The three addresses whose step was the TypeScript tick's (a co-op game
    // hosted or joined, a tape played back) have no game on this build until their Rust paths are here: one plain card, before anything boots.
    patch('src/main.ts', "  const rustWord = q.get('rust'), rustGame = rustWord === '1' || (rustWord !== '0' && ![NETPLAY, NET_JOIN, PLAYTAPE].some(k => q.has(k))) ? await import('./host/rustgame') : null;\n  if (rustGame) await rustGame.rustBoot();",
      "  // (THE ONLY GAME. A co-op game hosted or joined and a tape played back each stepped the TypeScript tick, which is gone, and their own paths onto the Rust game are\n  // not here yet: those three addresses get a plain card before anything boots)\n  if ([NETPLAY, NET_JOIN, PLAYTAPE].some(k => q.has(k))) throw Object.assign(new Error('this address has no game on this build yet'), { name: 'NoGameHere' });\n  const rustGame = await import('./host/rustgame');\n  await rustGame.rustBoot();", T);
    try { patch('src/main.ts', "`?rust=0` = the TypeScript game, for as long as that is still here (nothing of the\n  // module is loaded then). A page that is a game server's client or its door (`?net=`, `?join=`) or that plays a tape (`?playtape=`) is the TypeScript game's as\n  // well until those have been played on the Rust one: `?rust=1` asks it of them all the same\n", "There is no other game.\n", T); } catch { /* (a comment: its wording is not the program's to insist on) */ }
    patch('src/main.ts', "if (e instanceof NoWasmError) showFatal(NO_WASM_WORDS,", "if ((e as Error)?.name === 'NoGameHere') showFatal(\"This link opens a co-op game or a replay, and this build can't play those yet. You can still play every mission on your own from the title.\"); else if (e instanceof NoWasmError) showFatal(NO_WASM_WORDS,", T);
    patch('tools/blitz/word.ts', "export const DEFAULT_WORD: string = 'none';", "export const DEFAULT_WORD: string = 'rust';", T); }
  if (mode === 'instrument') { T.set('tools/blitz/delete-census-preload.ts', CENSUS_PRELOAD_TS); T.set('tools/blitz/delete-census-ids.json', JSON.stringify(W.ids));
    patch('bunfig.toml', 'preload = ["./tools/blitz/preload.ts"]', 'preload = ["./tools/blitz/delete-census-preload.ts", "./tools/blitz/preload.ts"]', T); }
  for (const file of T.keys()) if (TOOL_WRITTEN.has(file)) throw new Error(`${file} is written by a tool and the deletion would edit it: it must not`);
  for (const [file, text] of T) writeFileSync(join(ROOT, file), text);
  say(`${mode === 'instrument' ? 'the census\'s tree' : 'the deletion'}: ${T.size} files written under ${ROOT}`); }

// ---- the note
function noteText(): { md: string; tsv: string; lost: string } { const L: string[] = [], c = count(fnUnits), o = count(UNITS.filter(u => !u.fnLike)), n = (k: string) => c[k]?.[0] ?? 0, l = (k: string) => c[k]?.[1] ?? 0;
  const simLines = simFiles.reduce((a: number, sf: any) => a + sf.getLineStarts().length, 0), W = write('delete'), after = [...W.text].filter(([f]) => SIM.test(f)).reduce((a, [, t]) => a + t.split('\n').length, 0) + simFiles.filter((sf: any) => !W.text.has(rel(sf.fileName))).reduce((a: number, sf: any) => a + sf.getLineStarts().length, 0);
  const kept = fnUnits.filter(u => P.reach[u.id] >= 2), group = (r: string) => r.startsWith('src/') ? 'the page' : HOSTS.has(r) ? 'the hosts' : r.startsWith('tools/') ? 'a tool, and no Rust function to cross to' : r;
  const byRoot = new Map<string, [number, number]>(); for (const u of kept) { const w = group(rootOf(u)), a = byRoot.get(w) ?? [0, 0]; a[0]++; a[1] += u.lines; byRoot.set(w, a); }
  const byFile = new Map<string, [number, number]>(); for (const u of kept) { const r = rootOf(u), a = byFile.get(r) ?? [0, 0]; a[0]++; a[1] += u.lines; byFile.set(r, a); }
  L.push('## The plan on this tree', '', `Tree \`${FINGERPRINT}\` (a hash of the simulation's files). Census: ${CENSUS ? `${SEEN.size} functions seen to run as TypeScript in ${CENSUS.runs.length} plays of the Rust game` : 'NONE: every loose reference was followed'}.`, '',
    `The simulation's folders hold ${simLines} lines before and ${after} after (${simLines - after} fewer). Of their ${fnUnits.length} functions (${fnUnits.reduce((a, u) => a + u.lines, 0)} lines of them):`, '',
    '| class | functions | lines | what becomes of it |', '|---|---|---|---|',
    `| tick only | ${n('deleted')} | ${l('deleted')} | removed: nothing left standing reaches it |`,
    `| crossing | ${n('crossing')} | ${l('crossing')} | the name and signature stay, the body is the call of the Rust rule |`,
    `| kept, on the surface | ${n('kept+guard')} | ${l('kept+guard')} | the body stays and opens with the surface's guard: a tool's call between ticks crosses, a start's runs the TypeScript |`,
    `| kept | ${n('kept')} | ${l('kept')} | the body stays: a start, the picture's step, a reader over the puppet, or a function the Rust game has no row for |`,
    `| gone | ${n('gone')} | ${l('gone')} | the signature stays and the body throws by name (\`Game.stepSeats\`, and a declaration an override that stays still needs) |`, '',
    `Other declarations (classes as data, tables, types): ${o.kept?.[0] ?? 0} stay, ${o.deleted?.[0] ?? 0} go (${o.deleted?.[1] ?? 0} lines).`, '',
    '### What keeps the kept bodies', '', 'By the root at the end of the first chain of references found (a function many roots reach is counted under one):', '', '| root | functions | lines |', '|---|---|---|',
    ...[...byRoot].sort((a, b) => b[1][0] - a[1][0]).map(([r, [k, m]]) => `| ${r} | ${k} | ${m} |`), '',
    'The files that root the most:', '', '| file | functions | lines |', '|---|---|---|', ...[...byFile].sort((a, b) => b[1][0] - a[1][0]).slice(0, 25).map(([r, [k, m]]) => `| ${r} | ${k} | ${m} |`), '');
  // what the page runs every frame
  const perFrame = [...SEEN].filter(([, s]) => (s.perFrame ?? 0) >= 0.5).sort((a, b) => b[1].perFrame! - a[1].perFrame!);
  if (perFrame.length) { const onSurface = perFrame.filter(([u]) => u.guard);
    L.push('### What the page runs every frame', '', `The census's page plays ran ${perFrame.length} functions of the simulation's folders at least once every two frames; ${onSurface.length} of them are on the call surface. A crossing costs milliseconds, so each of these stays a TypeScript reader over the puppet:`, '',
      '| function | file | calls a frame |', '|---|---|---|', ...onSurface.slice(0, 80).map(([u, s]) => `| ${u.key} | ${u.file} | ${s.perFrame} |`), ''); }
  // loose candidates left
  const looseRows = [...P.looseMiss].filter(([u]) => P.reach[u.id] < 2).sort((a, b) => a[0].key < b[0].key ? -1 : 1);
  L.push('### A host\'s loose names the census did not see', '', `${looseRows.length} functions are named only loosely by the bridge's own files (a name read off a value typed \`any\`, or a type answered by shape) and were never seen to run in a play. With a row on the surface such a function crosses; with none it goes unless something else keeps it. If one of these places does mean that function, the call throws by name in the first play that reaches it. The first 60:`, '',
    '| function | class now | read at |', '|---|---|---|', ...looseRows.slice(0, 60).map(([u, at]) => `| ${u.key} (${u.file}) | ${classOf(u)} | ${[...at].slice(0, 3).join(', ')} |`), '');
  // kept, and never seen to run: what a later cleanup has to go by
  const unseen = kept.filter(u => !SEEN.has(u)); if (CENSUS) L.push('### Kept, and never seen to run', '', `${unseen.length} of the ${kept.length} functions that keep a body (${unseen.reduce((a, u) => a + u.lines, 0)} lines) were never seen to run as TypeScript in any play of the census. The checker reaches each from the page or a host, so it stays: most are what only a draw, a panel, the workbench or a start nobody played would run, and the rest are dead in practice. \`delete-plan.tsv\` has them (the \`seen\` column is empty): the list a cleanup with a real GPU would start from.`, '');
  // THE CHECKS THAT LOST A FUNCTION THEY CALLED: a row a tool and a function, with how the tool names it
  const lostRows: string[] = ['tool\tfunction\tfile\thow the tool names it\twhat the function is now'], lostBy = new Map<string, { gone: Set<string>; name: Set<string>; test: number }>(), L3 = (f: string) => { let x = lostBy.get(f); if (!x) lostBy.set(f, x = { gone: new Set(), name: new Set(), test: 0 }); return x; };
  for (const [u, files] of P.namedBy) if (u.fnLike && classOf(u) === 'gone') for (const f of files) { L3(f).gone.add(u.key); lostRows.push([f, u.key, u.file, 'by a reference the checker resolves', 'its signature, which throws by name'].join('\t')); }
  for (const [u, files] of P.looseTools) if (u.fnLike && P.reach[u.id] < 2 && !u.guard) for (const f of files) { L3(f).name.add(u.name); lostRows.push([f, u.key, u.file, 'by its name alone, off a value typed any (it may mean another object\'s)', classOf(u) === 'gone' ? 'its signature, which throws by name' : 'gone'].join('\t')); }
  for (const [u, places] of P.softTools) if (P.reach[u.id] < 2) for (const at of places) { L3(at.replace(/:\d+$/, '')).test++; lostRows.push([at, u.key, u.file, 'by its name alone, in a TEST for it: the check does not throw, it takes the other branch', classOf(u) === 'gone' ? 'its signature (the test still finds a function)' : 'gone'].join('\t')); }
  const exactTools = [...lostBy].filter(([, x]) => x.gone.size).sort((a, b) => b[1].gone.size - a[1].gone.size);
  L.push('### The checks that lost a function they called', '', `Nothing is restaged here: this is the list to fix forward from. \`tools/blitz/delete-lost.tsv\` has every row (a tool, a function, how the tool names it).`, '',
    `- ${exactTools.length} files of tools/ name, by a reference the checker resolves, a function of the tick that the Rust game has no row for (${new Set(exactTools.flatMap(([, x]) => [...x.gone])).size} functions). Each keeps its signature and throws by name when the check calls it.`,
    `- ${[...lostBy].filter(([, x]) => x.name.size).length} files read, off a game typed \`any\`, a name that matches a function which went. By name only: the file may mean another object's method of that name.`,
    `- ${[...lostBy].filter(([, x]) => x.test).length} files TEST for such a name (\`g.f?.()\`, \`typeof g.f === 'function'\`) at ${[...lostBy].reduce((a, [, x]) => a + x.test, 0)} places: a check that takes the other branch quietly. These are the ones to read first.`, '',
    'The files that name the most functions which now throw:', '', '| tool | functions that throw by name |', '|---|---|', ...exactTools.slice(0, 40).map(([f, x]) => `| ${f} | ${[...x.gone].slice(0, 8).join(', ')}${x.gone.size > 8 ? ` … (${x.gone.size})` : ''} |`), '');
  if (KEEP.length) L.push('### Kept by hand', '', '| function | why |', '|---|---|', ...KEEP.map(k => `| ${k.u.key} (${k.u.file}) | ${k.why} |`), '');
  const tsv = ['class\tlines\tfile\tfunction\tseen\tkept by'].concat(fnUnits.map(u => { const st = steps(u), w = st[0].w; return [classOf(u), u.lines, u.file, u.key, SEEN.get(u)?.n ?? '', w ? `${typeof w.from === 'string' ? w.from : w.from.key} ${w.how} (${w.at})` : ''].join('\t'); })).join('\n') + '\n';
  return { md: L.join('\n'), tsv, lost: lostRows.join('\n') + '\n' }; }
if (flag('--note')) { const N = noteText(), f = join(ROOT, 'tools/blitz/DELETION.md'), mark = '<!-- the tables below are written by tools/blitz/delete-tick.ts --note -->';
  const was = existsSync(f) ? readFileSync(f, 'utf8') : `# The deletion of the TypeScript tick\n\n`; const head = was.includes(mark) ? was.slice(0, was.indexOf(mark)) : was.replace(/\n*$/, '\n\n');
  writeFileSync(f, head + mark + '\n\n' + N.md); writeFileSync(join(ROOT, 'tools/blitz/delete-plan.tsv'), N.tsv); writeFileSync(join(ROOT, 'tools/blitz/delete-lost.tsv'), N.lost); say(`the note: tools/blitz/DELETION.md, tools/blitz/delete-plan.tsv and tools/blitz/delete-lost.tsv`); process.exit(0); }
if (opt('--why')) { const k = opt('--why')!; for (const u of UNITS.filter(u => u.key === k)) { console.log(`${u.key} (${u.file}): ${classOf(u)}${SEEN.has(u) ? ` · the census saw it run ${SEEN.get(u)!.n} times` : ''}`); for (const l of chain(u)) console.log('   ' + l); } process.exit(0); }
if (flag('--bridges')) { const N = Number(opt('--bridges')) || 40;
  // (what each file outside the simulation ALONE keeps: the plan again without its references)
  const base = fnUnits.filter(u => P.reach[u.id] >= 2).length, byFile = new Map<string, number>(); for (const u of fnUnits) if (P.reach[u.id] >= 2) { const r = rootOf(u); byFile.set(r, (byFile.get(r) ?? 0) + 1); }
  const rows: string[] = [];
  for (const [f] of [...byFile].sort((a, b) => b[1] - a[1]).slice(0, N)) { if (!sfByRel.has(f)) { rows.push(`${String(byFile.get(f)).padStart(6)} first-chain              ${f}`); continue; } const q = plan(r => r.file === f); const n = fnUnits.filter(u => q.reach[u.id] >= 2).length; rows.push(`${String(byFile.get(f)).padStart(6)} first-chain ${String(base - n).padStart(6)} alone  ${f}`); }
  console.log(rows.join('\n')); process.exit(0); }
for (const l of summary()) console.log(l);
if (flag('--instrument')) writeOut('instrument'); else if (flag('--write')) writeOut('delete');
if (opt('--json')) writeFileSync(opt('--json')!, JSON.stringify(UNITS.map(u => { const w = P.why[u.id]; return { key: u.key, file: u.file, kind: u.kind, fn: u.fnLike, cls: classOf(u), lines: u.lines, row: u.row?.id, seen: SEEN.get(u)?.n, why: w ? { from: typeof w.from === 'string' ? w.from : w.from.key, at: w.at, how: w.how } : undefined, root: P.reach[u.id] ? rootOf(u) : undefined, namedBy: [...(P.namedBy.get(u) ?? [])].slice(0, 60), loose: [...(P.looseMiss.get(u) ?? [])], looseTools: [...(P.looseTools.get(u) ?? [])].slice(0, 20) }; })));
/** (for the bridges' report) what one declaration reaches by exact references alone, with no barrier */
function closureOf(u0: Unit): { fns: number; lines: number } { const seen = new Set<Unit>([u0]), q = [u0]; let fns = 0, lines = 0; while (q.length) { const u = q.pop()!; if (u.fnLike) { fns++; lines += u.lines; } for (const e of [...u.sig, ...u.body]) if (e.to && !seen.has(e.to) && e.to !== TICK) { seen.add(e.to); q.push(e.to); } if (u.kind === 'class') for (const m of u.members!) if ((m.kind === 'ctor' || (m.kind === 'field' && !m.fnLike)) && !seen.has(m)) { seen.add(m); q.push(m); } if (u.cls && !seen.has(u.cls)) { seen.add(u.cls); q.push(u.cls); } } return { fns, lines }; }
const ROOT_EDGES_X = ROOT_EDGES;
export { ROOT_EDGES_X, closureOf, UNITS, P, classOf, chain, rootOf, program, checker, ts, SK, ROOT, TICK, GAME, SIM, CROSSING, rel, CALLS, TAPS, SEEN, CENSUS, FINGERPRINT, sfByRel, simFiles, stubKey, bodyOf, fnOf, implOf, lineOf, flag, opt, say, fnUnits, type Unit };
