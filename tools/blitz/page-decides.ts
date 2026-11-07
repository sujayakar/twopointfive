// WHERE THE PAGE'S TYPESCRIPT STILL DECIDES, AS A LIST. The Rust game (rust/game_rules) runs every rule; the TypeScript `Game` is a copy the bridge fills
// after each step, for the picture, the sound, the HUD and the menus to read. A line of TypeScript that reads that copy and then ACTS (keeps a record of
// its own, writes the game's state, says a line, stores or sends a result), or that works out again what a Rust rule already answers, is a rule the page
// holds: two machines of one game can disagree about it, and a Rust rule that moves leaves it behind. This program lists every such place by reading the
// tree with the TypeScript checker. Nothing is found by a function's name.
//
//   bun run tools/blitz/page-decides.ts                 the counts by sort and by kind (--kinds: every kind)
//   bun run tools/blitz/page-decides.ts --tsv out.tsv   a row a function: its sort, its kind, when it runs, what it does, the reason
//   bun run tools/blitz/page-decides.ts --md out.md     the same as a page, a table a sort (the sorts a person made, and what is left)
//   bun run tools/blitz/page-decides.ts --why <file>:<function>[,…]  how the frame and the page reach it, and each act with its line
//   bun run tools/blitz/page-decides.ts --hubs [n]      the largest branches of the frame's reach (to find a stand or a menu that is not yet a stop)
//   bun run tools/blitz/page-decides.ts --doors         the deciding rows the page's own code walks in through, each with how many stand behind it
//   bun run tools/blitz/page-decides.ts --check         exit 1 while a row is UNSORTED or sorted `decides` or `throws`, or a line of the sorts' file sorts
//                                                       nothing, or sorts more rows than a person has read
//   bun run tools/blitz/page-decides.ts --pin           write, on each line of the sorts' file that is not `decides`, how many rows it sorts now
//   (vendor/typescript, or TS_LIB=<path to typescript.js>)
//
// A ROW is a function with a body under src/ (a method, a top-level function, a top-level table with the closures written in it, a file's own top-level
// lines; outside the simulation's folders also a function written inside another, by name) that does at least one ACT:
//   keeps      writes a field or a module's variable declared in the simulation's folders that the bridge does NOT carry: a record only TypeScript has
//   overwrites writes a field the bridge carries: the Rust game's own state, written on the copy (the next apply puts it back, or the host sends it on)
//   says       pushes a line to the game's log or a man's bubble (a write of `Game.messages` or `Guard.bubble`, or a call of a function that makes one)
//   stores     writes the browser's store, or sends to the backend (`localStorage`, `fetch`)
//   crosses    hands the Rust game something from outside a step (`crossing`, the module's `face_…` exports): the host's own road
// or that, with no act,
//   JUDGES     sits in the simulation's folders, reads the game's carried state and answers a value it works out (a comparison, a branch, a loop:
//              a field merely handed on is no judgement), while a Rust function of its name exists
//              (rust/game_rules, or a row of the call surface, tools/blitz/calls.json): the same rule in two languages. THE PAGE INSTALLS NO CALL SURFACE
//              (src/game/crossing.ts `setSurface` is called by the tools' preload only), so on the page the TypeScript answers, on the copy, where a
//              headless tool that asks the same function between ticks is answered by the Rust game.
//   (a judge that is ONE LINE and hands on one thing with at most a default or one comparison, calling nothing that decides, is sorted `reads`: it is
//    how the page reads the game's record, and no rule)
//   THROWS     has only the crossing for a body (`crossToRust`, `goneWithTick`): with no surface it throws by name if the page calls it
// NOT ACTS, by ruling: a write of a tally (`…_STATS`, a `stats` record), of a field the schema calls scratch, of a field of an object the function has
// just made itself, and a class's own first values (its constructor and field lines).
//
// WHEN IT RUNS decides who sorts a row:
//   every frame   the frame's roots reach it (tools/blitz/page-decides.roots.json: the loop's frame, the Rust host's step with its dress step, what the
//                 out events call, a timer's closure), or the census saw it run in a page play once in four frames or more (tools/blitz/
//                 delete-census.json, taken on an older tree, joined by file and name: it only ADDS). THESE ARE THE ROWS A PERSON SORTS, in
//                 tools/blitz/page-decides.rows.tsv (a line a row or a pattern of rows: pattern, sort, kind, reason, rows when last read), as one of:
//                   decides  to move into the Rust game             moving   its move is already under way (the line says which)
//                   picture  the picture's or the sound's own       host     the host's own plumbing (the bridge, saves, the frame's loop)
//                   hand     the player's or the developer's own hand, on a timer
//                 A row nothing sorts is UNSORTED. A pattern sorts a whole file where the file is of one nature, so a line that is not `decides` is
//                 pinned to the number of rows a person read (--pin): a new function there makes the line GROW, and the check is red until it is read.
//   start or hand the game's page reaches it, but not from a frame's root: it runs as a game is stood or put back, or on a key, a menu or a panel.
//                 Sorted by the program (a line of the sorts' file gives it its kind, never its sort). How a game is stood is not this list's.
//   other page    only the workbench, the viewer, the editor, the replay viewer or the sound pages reach it: they play no Rust game
//   dead          no page reaches it (a host of tools/ may: they are not read here)
//
// WHAT IT CANNOT SEE: a write through a computed key (`obj[key] = v`) other than a literal's; a name off a value typed `any` that no cast explains is
// joined to every field or method of that name (marked `~`); an act inside a string a page script evaluates; a judgement written in place in the HUD's
// own code with no function of the simulation's under it (src/ui reads the copy and compares: it can state nothing the copy does not hold, but it can
// repeat a rule); a function of the simulation's that judges and has no Rust function of its name (it is TypeScript's alone: a view, or a rule nobody ported).
import { readFileSync, writeFileSync, existsSync, readdirSync, statSync } from 'node:fs';
import { join, resolve, relative } from 'node:path';

const ROOT = resolve(import.meta.dir, '../..');
const TS_LIB = process.env.TS_LIB ?? join(ROOT, 'vendor/typescript/lib/typescript.js');
if (!existsSync(TS_LIB)) { console.error(`no TypeScript at ${TS_LIB} (vendor/typescript, or TS_LIB=<path to typescript.js>)`); process.exit(2); }
const tsMod: any = await import(TS_LIB); const ts: any = tsMod.default ?? tsMod; const SK = ts.SyntaxKind;
const ARGV = process.argv.slice(2), flag = (n: string) => ARGV.includes(n), opt = (n: string) => { const i = ARGV.indexOf(n); return i < 0 ? undefined : ARGV[i + 1]; };
const say = (s: string) => process.stderr.write(s + '\n');

/** the simulation's folders: where the game's state and its rules were written */
const SIM = /^src\/(game|anim|math|sim|scene)\//;
/** the game's page starts here; every other entry is a page that plays no Rust game */
const GAME_ENTRY = 'src/main.ts';
/** THE FRAME'S ROOTS, by name and with why (each must be a function of this tree), and the bridge's files whose imports from src/ run inside a step */
const ROOTS = JSON.parse(readFileSync(join(import.meta.dir, 'page-decides.roots.json'), 'utf8')) as { frame: { id: string; why: string }[]; stepFiles: { file: string; why: string }[]; stops: { id: string; why: string; behind: string }[] };

// ---- the tree
const cfg = ts.readConfigFile(join(ROOT, 'tsconfig.json'), ts.sys.readFile), parsed = ts.parseJsonConfigFileContent(cfg.config, ts.sys, ROOT);
const program = ts.createProgram([...parsed.fileNames, ...ROOTS.stepFiles.map(s => join(ROOT, s.file))], { ...parsed.options, allowImportingTsExtensions: true }), checker = program.getTypeChecker();   // (the bridge's files that run inside a step are read as the page's own code)
const rel = (f: string) => relative(ROOT, f).replace(/\\/g, '/');
const STEP_FILES = new Set(ROOTS.stepFiles.map(s => s.file));
const files = program.getSourceFiles().filter((f: any) => !f.isDeclarationFile && (rel(f.fileName).startsWith('src/') || STEP_FILES.has(rel(f.fileName))));

// ---- what the bridge carries: a class's field by its TypeScript name, and a module's top-level names
const SCHEMA = JSON.parse(readFileSync(join(import.meta.dir, 'schema.json'), 'utf8'));
const LISTS = JSON.parse(readFileSync(join(import.meta.dir, 'puppet-lists.json'), 'utf8')) as { step: string[]; start: string[] };
const STEP = new Set(LISTS.step), START = new Set(LISTS.start);
/** `Class.field` → how it is carried: 'tick' (every step), 'start' (at a start or a load), 'whole' (only on a whole apply) */
const CARRIED = new Map<string, string>(), CARRIED_NAMES = new Set<string>();
for (const [q, T] of Object.entries(SCHEMA.types) as [string, any][]) { if (!T.fields) continue; const cls = T.cls ?? T.name, mod = q.replace(/^crate::/, '').split('::')[0];
  for (const F of T.fields) { const w = `${q.replace(/^crate::/, '')}.${F.rust}`, how = STEP.has(w) ? 'tick' : START.has(w) ? 'start' : 'whole'; const k = `${mod}::${cls}.${F.ts ?? F.rust}`, was = CARRIED.get(k); if (!was || how === 'tick' || (how === 'start' && was === 'whole')) CARRIED.set(k, how); CARRIED_NAMES.add(`${cls}.${F.ts ?? F.rust}`); } }
const TOP = new Set<string>(); for (const [f, names] of Object.entries(SCHEMA.top ?? {}) as [string, string[]][]) for (const n of names) TOP.add(`${f}:${n}`);
/** fields the schema calls scratch (a query's own working space), by the TypeScript's name */
const SCRATCH = new Set<string>(); for (const q of Object.keys(SCHEMA.scratch ?? {})) { const i = q.lastIndexOf('.'), T = SCHEMA.types[`crate::${q.slice(0, i)}`], F = T?.fields?.find((x: any) => x.rust === q.slice(i + 1)); if (T && F) SCRATCH.add(`${q.split('::')[0]}::${T.cls ?? T.name}.${F.ts ?? F.rust}`); }
/** the call surface: a function of the simulation's that has a Rust function, by file and key */
const CALLS = JSON.parse(readFileSync(join(import.meta.dir, 'calls.json'), 'utf8')) as { rows: { key: string; file: string; rust: string }[] };
const SURFACE = new Map<string, string>(); for (const r of CALLS.rows) SURFACE.set(`${r.file}:${r.key}`, r.rust.replace(/^crate::/, ''));
/** THE RUST GAME'S FUNCTIONS, by module: a TypeScript function `Class.method` of src/game/x.ts is `class_method` of rust/game_rules/src/x (1,047 of the call
 *  surface's 1,072 rows are named so; the rest are found by their row) */
const RUST_RULES = join(ROOT, 'rust/game_rules/src'), RUST_FNS = new Map<string, Set<string>>();
/** … and the ones Rust has only a name for: a function whose body is `crate::no_body(…)` (a module's `missing.rs`: not written on purpose) is no twin */
const RUST_MISSING = new Set<string>();
for (const mod of readdirSync(RUST_RULES)) { const dir = join(RUST_RULES, mod); if (!statSync(dir).isDirectory()) continue; const set = new Set<string>(); for (const f of readdirSync(dir)) if (f.endsWith('.rs')) { const text = readFileSync(join(dir, f), 'utf8'); for (const m of text.matchAll(/\bfn\s+(\w+?)(?:__body)?\s*[<(]/g)) set.add(m[1]); for (const m of text.matchAll(/\bfn\s+(\w+)\s*[<(][^{;]*\{\s*crate::no_body\(/g)) RUST_MISSING.add(`${mod}::${m[1]}`); } RUST_FNS.set(mod, set); }
const snake = (s: string) => s.replace(/([a-z0-9])([A-Z])/g, '$1_$2').replace(/([A-Z]+)([A-Z][a-z])/g, '$1_$2').toLowerCase();
const rustModOf = (file: string) => { const m = /^src\/(game|anim|math|sim|scene)\/(.*)\.ts$/.exec(file); if (!m) return ''; const base = snake(m[2].replace(/\//g, '_')); return m[1] === 'game' ? base : `${m[1]}_${base}`; };
const rustTwin = (u: { id: string; file: string; name: string }): string => { const surf = SURFACE.get(u.id.replace(/#\d+$/, '')); if (surf) return RUST_MISSING.has(surf) ? '' : surf; const mod = rustModOf(u.file), fns = RUST_FNS.get(mod); if (!fns) return ''; const fn = u.name.split('.').map(snake).join('_'); return fns.has(fn) && !RUST_MISSING.has(`${mod}::${fn}`) ? `${mod}::${fn}` : ''; };
/** how the bridge carries a field, by the module its type is declared in: a type of the same name in another file is another type (`State` is in thirty) */
const carriedOf = (f: { key: string; file: string }): string | undefined => CARRIED.get(`${rustModOf(f.file)}::${f.key}`);
const scratchOf = (f: { key: string; file: string }): boolean => SCRATCH.has(`${rustModOf(f.file)}::${f.key}`);
/** THE CENSUS: what was seen to run as TypeScript in a page play while the Rust game stepped, once in four frames or more over the play (it only adds;
 *  less than that is a start's work spread over the play's frames) */
const CENSUS_FLOOR = 0.25;
const CENSUS = new Map<string, number>(); { const f = join(import.meta.dir, 'delete-census.json'); if (existsSync(f)) for (const s of JSON.parse(readFileSync(f, 'utf8')).seen as { file: string; key: string; page?: number; perFrame?: number }[]) if ((s.perFrame ?? 0) >= CENSUS_FLOOR) CENSUS.set(`${s.file}:${s.key}`, s.perFrame!); }

// ---- units: a function with a body, named by its file and its own name
type Act = { kind: 'keeps' | 'overwrites' | 'says' | 'stores' | 'crosses'; what: string; line: number; loose?: boolean };
interface Unit { id: string; file: string; name: string; line: number; lines: number; acts: Act[]; refs: Set<string>; loose: Set<string>; reads: Set<string>; answers: boolean; works: number; top: boolean; init: boolean; stub: string; timer: boolean; cls?: any; lit?: any; acc?: 'get' | 'set'; replaced?: boolean; callsOut: Map<string, number>; kept: Set<string> }
const UNITS = new Map<string, Unit>(), unitOfNode = new Map<any, Unit>();
/** every class's members by name (with the class, for the question "could this value be one of those"), and every function written as a member of an
 *  object in the page's own code: for a call the checker resolves only to a type's member */
const BY_NAME = new Map<string, Unit[]>(), LIT_BY_NAME = new Map<string, Unit[]>();
const push = (m: Map<string, Unit[]>, k: string, u: Unit) => { if (!m.has(k)) m.set(k, []); m.get(k)!.push(u); };
const mk = (node: any, file: string, name: string, top = false): Unit => { let id = `${file}:${name}`, n = 1; while (UNITS.has(id) && !top) id = `${file}:${name}#${++n}`; let u = UNITS.get(id);
  if (!u) { const sf = node.getSourceFile(), a = sf.getLineAndCharacterOfPosition(node.getStart()).line + 1, b = sf.getLineAndCharacterOfPosition(node.getEnd()).line + 1; u = { id, file, name, line: a, lines: b - a + 1, acts: [], refs: new Set(), loose: new Set(), reads: new Set(), answers: false, works: 0, top, init: /\.\(fields\)$/.test(name), stub: '', timer: false, callsOut: new Map(), kept: new Set() }; UNITS.set(id, u); }
  unitOfNode.set(node, u); return u; };
const isFn = (n: any) => n.kind === SK.FunctionDeclaration || n.kind === SK.MethodDeclaration || n.kind === SK.Constructor || n.kind === SK.GetAccessor || n.kind === SK.SetAccessor || n.kind === SK.ArrowFunction || n.kind === SK.FunctionExpression;
const nameOf = (n: any): string => n.name ? (n.name.text ?? n.name.getText()) : '';
/** a closure handed to one of these runs again and again with no hand behind it (`setTimeout` runs once, for whoever set it: no root) */
const TIMERS = new Set(['setInterval', 'requestAnimationFrame']);
/** OUTSIDE THE SIMULATION'S FOLDERS a function written inside another is a unit of its own where it has a name (a nested function, a constant given one,
 *  a member of an object) or is handed to a timer: the page's `main` stands the game AND builds the closures the frame calls, and they are not one thing */
const nest = (body: any, file: string, parent: Unit) => { const v = (n: any) => {
    let u: Unit | null = null;
    if (n.kind === SK.FunctionDeclaration && n.body) u = mk(n, file, `${parent.name} › ${nameOf(n)}`);
    else if (n.kind === SK.VariableDeclaration && n.initializer && isFn(n.initializer) && n.name.kind === SK.Identifier) u = mk(n.initializer, file, `${parent.name} › ${n.name.text}`);
    else if ((n.kind === SK.PropertyAssignment && isFn(n.initializer)) || (n.kind === SK.MethodDeclaration && n.body && n.parent.kind === SK.ObjectLiteralExpression)) { u = mk(n.kind === SK.PropertyAssignment ? n.initializer : n, file, `${parent.name} › {${nameOf(n)}}`); u.lit = n.parent; push(LIT_BY_NAME, nameOf(n), u); }
    else if (n.kind === SK.CallExpression && TIMERS.has(n.expression.kind === SK.Identifier ? n.expression.text : n.expression.kind === SK.PropertyAccessExpression ? n.expression.name.text : '') && n.arguments[0] && isFn(n.arguments[0])) { const t = mk(n.arguments[0], file, `${parent.name} › (a timer)`); t.timer = true; parent.refs.add(t.id); nest(n.arguments[0].body, file, t); for (const a of n.arguments.slice(1)) v(a); v(n.expression); return; }
    if (u) { parent.refs.add(u.id); const b = n.kind === SK.VariableDeclaration || n.kind === SK.PropertyAssignment ? n.initializer.body : n.body; if (b) nest(b, file, u); return; }
    ts.forEachChild(n, v); }; if (body) v(body); };
for (const sf of files) { const file = rel(sf.fileName), sim = SIM.test(file); mk(sf, file, '(the file)', true);
  const member = (cn: string, m: any, node: any, cls: any) => { const u = mk(node, file, `${cn}.${nameOf(m)}`); u.cls = cls; if (m.kind === SK.GetAccessor) u.acc = 'get'; else if (m.kind === SK.SetAccessor) u.acc = 'set'; push(BY_NAME, nameOf(m), u); if (!sim) nest(node.body, file, u); };
  const visitTop = (st: any, as?: string) => {
    if (st.kind === SK.FunctionDeclaration && st.body) { const u = mk(st, file, nameOf(st) || 'default'); if (!sim) nest(st.body, file, u); return; }
    if (st.kind === SK.ClassDeclaration || st.kind === SK.ClassExpression) { const cn = as ?? (nameOf(st) || 'default'); const init = mk(st, file, `${cn}.(fields)`);
      for (const m of st.members) { if ((m.kind === SK.MethodDeclaration || m.kind === SK.GetAccessor || m.kind === SK.SetAccessor) && m.body) member(cn, m, m, st);
        else if (m.kind === SK.Constructor && m.body) { unitOfNode.set(m, init); if (!sim) nest(m.body, file, init); }
        else if (m.kind === SK.PropertyDeclaration && m.initializer && isFn(m.initializer)) member(cn, m, m.initializer, st); }
      return; }
    if (st.kind === SK.VariableStatement) { for (const d of st.declarationList.declarations) if (d.initializer && d.name.kind === SK.Identifier) { let hasFn = false; const look = (x: any) => { if (hasFn) return; if (isFn(x) || x.kind === SK.ClassExpression) { hasFn = true; return; } ts.forEachChild(x, look); }; look(d.initializer);
      if (hasFn) { if (d.initializer.kind === SK.ClassExpression) visitTop(d.initializer, d.name.text); else { const u = mk(d.initializer, file, d.name.text); if (!sim) nest(isFn(d.initializer) ? d.initializer.body : d.initializer, file, u); } } } return; }
    if (st.kind === SK.ExportAssignment && st.expression && isFn(st.expression)) { mk(st.expression, file, 'default'); return; }
    if (!sim) nest(st, file, UNITS.get(`${file}:(the file)`)!);
  };
  for (const st of sf.statements) visitTop(st); }
const unitAt = (n: any): Unit => { for (let x = n; x; x = x.parent) { const u = unitOfNode.get(x); if (u) return u; } throw new Error('no unit'); };
say(`page-decides: ${files.length} files, ${UNITS.size} functions`);

// ---- a declaration → the unit that is its body, and a property declaration → its field name
const fieldOf = (d: any): { key: string; file: string } | null => {
  if (!d) return null; const file = rel(d.getSourceFile().fileName); if (!SIM.test(file)) return null;
  if (d.kind === SK.PropertyDeclaration || d.kind === SK.PropertySignature || (d.kind === SK.Parameter && d.parent?.kind === SK.Constructor && ts.getCombinedModifierFlags(d) & (ts.ModifierFlags.Public | ts.ModifierFlags.Private | ts.ModifierFlags.Protected | ts.ModifierFlags.Readonly))) {
    if (d.initializer && isFn(d.initializer)) return null;   // (a function kept in a field is a method)
    const owner = d.kind === SK.Parameter ? d.parent.parent : d.parent; let on = owner?.name ? nameOf(owner) : '';
    if (!on) { /* a type written in place: named by the path down to it */ const path: string[] = []; for (let x = owner; x; x = x.parent) { if ((x.kind === SK.PropertySignature || x.kind === SK.PropertyDeclaration || x.kind === SK.VariableDeclaration || x.kind === SK.TypeAliasDeclaration || x.kind === SK.InterfaceDeclaration || x.kind === SK.ClassDeclaration) && x.name) { path.unshift(nameOf(x)); if (x.kind !== SK.PropertySignature && x.kind !== SK.PropertyDeclaration) break; } } on = path.join('.'); }
    return { key: `${on}.${nameOf(d)}`, file }; }
  if (d.kind === SK.PropertyAssignment || d.kind === SK.ShorthandPropertyAssignment) { /* a row of a module's table */ if (d.kind === SK.PropertyAssignment && isFn(d.initializer)) return null; const path: string[] = [nameOf(d)]; let x = d.parent; for (; x; x = x.parent) { if (x.kind === SK.PropertyAssignment) path.unshift(nameOf(x)); else if (x.kind === SK.VariableDeclaration) { path.unshift(nameOf(x)); break; } else if (isFn(x) || x.kind === SK.ClassDeclaration) return null; }
    if (!x || x.parent?.parent?.parent?.kind !== SK.SourceFile) return null; return { key: path.join('.'), file }; }
  return null; };
/** a module's own variable (a `let` or a table at a file's top level in the simulation's folders) */
const topVarOf = (d: any): { key: string; file: string } | null => { if (!d || d.kind !== SK.VariableDeclaration || d.parent?.parent?.parent?.kind !== SK.SourceFile) return null; const file = rel(d.getSourceFile().fileName); return SIM.test(file) ? { key: nameOf(d), file } : null; };
const declUnit = (d: any): Unit | null => { if (!d) return null;
  if ((d.kind === SK.VariableDeclaration || d.kind === SK.PropertyDeclaration || d.kind === SK.PropertyAssignment) && d.initializer) return unitOfNode.get(d.initializer) ?? null;
  return unitOfNode.get(d) ?? null; };
const symOf = (n: any): any => { let s = checker.getSymbolAtLocation(n); if (s && s.flags & ts.SymbolFlags.Alias) { try { s = checker.getAliasedSymbol(s); } catch { /* an alias with no target */ } } return s; };
const strip = (x: any) => { while (x && (x.kind === SK.ParenthesizedExpression || x.kind === SK.NonNullExpression || x.kind === SK.AsExpression || x.kind === SK.SatisfiesExpression || x.kind === SK.TypeAssertionExpression)) x = x.expression; return x; };
const loosely = (t: any): boolean => !t || !!(t.flags & (ts.TypeFlags.Any | ts.TypeFlags.Unknown)) || (!!t.aliasSymbol && t.aliasSymbol.name === 'Record' && !!(t.aliasTypeArguments?.[1]?.flags & (ts.TypeFlags.Any | ts.TypeFlags.Unknown)));
const typeAt = (n: any): any => { try { return checker.getTypeAtLocation(n); } catch { return null; } };
let GAME_T: any; const gameType = (): any => { if (GAME_T === undefined) { GAME_T = null; const sf = program.getSourceFile(join(ROOT, 'src/game/game.ts')); for (const st of sf?.statements ?? []) if (st.kind === SK.ClassDeclaration && nameOf(st) === 'Game') { const cs = checker.getSymbolAtLocation((st as any).name); if (cs) GAME_T = checker.getDeclaredTypeOfSymbol(cs); } } return GAME_T; };
/** THE TYPE UNDER A CAST: `const G = g as unknown as Record<string, any>; G.par.tick()` reads `par` of a Game and `tick` of what a Game's `par` is */
const typeUnder = (e: any, depth = 0): any => { if (!e || depth > 6) return null; const x = strip(e), t = typeAt(x);
  if (t && !loosely(t)) return t;
  if (x.kind === SK.Identifier) { const d = symOf(x)?.declarations?.[0]; if (d?.kind === SK.VariableDeclaration && d.initializer && (d.parent.flags & ts.NodeFlags.Const)) return typeUnder(d.initializer, depth + 1);
    if (d?.kind === SK.Parameter && x.text === 'g' && STEP_FILES.has(rel(d.getSourceFile().fileName))) return gameType();   // (the bridge's files take the game as an untyped `g`)
    return null; }
  if (x.kind === SK.PropertyAccessExpression) { const T = typeUnder(x.expression, depth + 1); if (!T) return null; const p = checker.getPropertyOfType(checker.getApparentType(checker.getNonNullableType(T)), x.name.text); if (!p) return null; const pt = checker.getTypeOfSymbol(p); return loosely(pt) ? null : pt; }
  return null; };
const memberUnder = (n: any, name: string): any[] => { const T = typeUnder(n.expression); if (!T) return []; const A = checker.getNonNullableType(T), each = A.isUnion?.() ? A.types : [A], out: any[] = []; for (const t of each) { const p = checker.getPropertyOfType(checker.getApparentType(t), name); for (const d of p?.declarations ?? []) out.push(d); } return out; };
/** WHO IS HANDED WHERE A TYPE IS ASKED FOR. A call through a type's member (`ctx.hud.update()`) reaches a class only where a value of that class is
 *  somewhere handed to that type (an argument, a member of an object, an assignment, an answer, a cast, `implements`), and a function written in an
 *  object only where that object was written for that type. A class that merely has a method of the name is not reached. */
const HANDED = new Set<string>();
const tids = (C: any): number[] => { if (!C || loosely(C)) return []; const N = checker.getNonNullableType(C); return (N.isUnion?.() ? N.types : [N]).filter((t: any) => !loosely(t)).map((t: any) => t.id); };
const hand = (e: any, C: any) => { const ids = tids(C); if (!ids.length) return; const T = typeAt(e), each = T?.isUnion?.() ? T.types : [T]; for (const t of each) { const cs = t?.symbol; if (cs && cs.flags & ts.SymbolFlags.Class) for (const id of ids) HANDED.add(`${ts.getSymbolId(cs)}:${id}`); } };
const ctxOf = (e: any): any => { try { return checker.getContextualType(e); } catch { return null; } };
const classId = (u: Unit): number => { const cs = u.cls?.name ? checker.getSymbolAtLocation(u.cls.name) : u.cls?.symbol; return cs ? ts.getSymbolId(cs) : -1; };
/** the calls through a type's member, settled once every hand-over has been read */
const PENDING: { u: Unit; name: string; recv: number[]; runs: boolean; write: boolean }[] = [];

// ---- the walk
/** the language's own method names: off an untyped value they are taken for an array's, a map's or a string's, never a class's of this tree */
const OWN_NAMES = new Set(['push', 'pop', 'shift', 'unshift', 'splice', 'slice', 'sort', 'reverse', 'fill', 'set', 'get', 'has', 'delete', 'add', 'clear', 'keys', 'values', 'entries', 'forEach', 'map', 'filter', 'find', 'findIndex', 'some', 'every', 'reduce', 'includes', 'indexOf', 'join', 'concat', 'flat', 'flatMap', 'at', 'toString', 'toFixed', 'split', 'replace', 'trim', 'startsWith', 'endsWith', 'padStart', 'padEnd', 'match', 'test', 'exec', 'then', 'catch', 'finally', 'call', 'apply', 'bind', 'next', 'subarray', 'copyWithin', 'charCodeAt', 'toLowerCase', 'toUpperCase', 'localeCompare', 'valueOf', 'length']);
const WORKS = new Set(['some', 'every', 'find', 'findIndex', 'findLast', 'filter', 'reduce', 'includes', 'indexOf']);
const MUTATORS = new Set(['push', 'pop', 'shift', 'unshift', 'splice', 'sort', 'reverse', 'fill', 'set', 'delete', 'add', 'clear', 'copyWithin']);
const SAY_FIELDS = new Set(['Game.messages', 'Guard.bubble']);
/** is `n` the target of a write: `n = v`, `n += v`, `n++`, `delete n`, a destructuring's target */
const writeKind = (n: any): boolean => { const p = n.parent; if (!p) return false;
  if (p.kind === SK.BinaryExpression && p.left === n && p.operatorToken.kind >= SK.FirstAssignment && p.operatorToken.kind <= SK.LastAssignment) return true;
  if ((p.kind === SK.PrefixUnaryExpression || p.kind === SK.PostfixUnaryExpression) && (p.operator === SK.PlusPlusToken || p.operator === SK.MinusMinusToken)) return true;
  if (p.kind === SK.DeleteExpression) return true;
  if (p.kind === SK.ParenthesizedExpression || p.kind === SK.NonNullExpression || p.kind === SK.AsExpression) return writeKind(p);
  if (p.kind === SK.ArrayLiteralExpression || p.kind === SK.PropertyAssignment || p.kind === SK.ShorthandPropertyAssignment || p.kind === SK.SpreadElement) { let x = p; while (x.parent && (x.parent.kind === SK.ArrayLiteralExpression || x.parent.kind === SK.ObjectLiteralExpression || x.parent.kind === SK.PropertyAssignment || x.parent.kind === SK.SpreadElement)) x = x.parent; const q = x.parent; return !!q && q.kind === SK.BinaryExpression && q.left === x && q.operatorToken.kind === SK.EqualsToken && (x.kind === SK.ArrayLiteralExpression || x.kind === SK.ObjectLiteralExpression) && !(p.kind === SK.PropertyAssignment && p.name === n); }
  return false; };
/** `n.push(…)`, `n.length = 0`, `n[i] = v`, `n.set(…)`: the thing n holds is changed in place (a list's or a map's own method, not a class's of that name) */
const mutated = (n: any): boolean => { let p = n.parent; while (p && (p.kind === SK.NonNullExpression || p.kind === SK.ParenthesizedExpression || p.kind === SK.AsExpression)) { n = p; p = p.parent; } if (!p) return false;
  if (p.kind === SK.PropertyAccessExpression && p.expression === n) { const m = p.name.text; if (MUTATORS.has(m) && p.parent?.kind === SK.CallExpression && p.parent.expression === p) { const d = symOf(p.name)?.declarations?.[0] ?? memberUnder(p, m)[0]; return !d || d.getSourceFile().isDeclarationFile || rel(d.getSourceFile().fileName) === 'src/math/side.ts'; } return m === 'length' && writeKind(p); }   // (the side tables of src/math/side.ts are maps and sets with their own `set` / `add`)
  if (p.kind === SK.ElementAccessExpression && p.expression === n) return writeKind(p);
  return false; };
const lineOf = (n: any) => n.getSourceFile().getLineAndCharacterOfPosition(n.getStart()).line + 1;
/** fields of the simulation's by bare name, for a name off a value nothing types */
const FIELDS_BY_NAME = new Map<string, Set<string>>();
for (const sf of files) { if (!SIM.test(rel(sf.fileName))) continue; const v = (n: any) => { if (n.kind === SK.PropertyDeclaration || n.kind === SK.PropertySignature) { const f = fieldOf(n); if (f) { const k = nameOf(n); if (!FIELDS_BY_NAME.has(k)) FIELDS_BY_NAME.set(k, new Set()); FIELDS_BY_NAME.get(k)!.add(f.key); } } ts.forEachChild(n, v); }; v(sf); }
/** the leftmost name of an access chain (`a` of `a.b[0].c`), or `this` */
const rootOf = (n: any): any => { let x = n; while (x && (x.kind === SK.PropertyAccessExpression || x.kind === SK.ElementAccessExpression || x.kind === SK.NonNullExpression || x.kind === SK.ParenthesizedExpression || x.kind === SK.AsExpression)) x = x.expression; return x; };
/** an object the function has just made itself: a local given a literal or a `new` (and `this`, in a class's own first lines) */
const madeHere = (u: Unit, n: any): boolean => { const r = rootOf(n.expression ?? n); if (!r) return false; if (r.kind === SK.ThisKeyword) return u.init; if (r.kind !== SK.Identifier) return r.kind === SK.ObjectLiteralExpression || r.kind === SK.ArrayLiteralExpression || r.kind === SK.NewExpression;
  const d = symOf(r)?.declarations?.[0]; if (!d || d.kind !== SK.VariableDeclaration || !d.initializer || d.parent?.parent?.parent?.kind === SK.SourceFile) return false; const i = strip(d.initializer);
  return i.kind === SK.ObjectLiteralExpression || i.kind === SK.ArrayLiteralExpression || i.kind === SK.NewExpression || (i.kind === SK.CallExpression && /^(JSON\.parse|structuredClone)$/.test(i.expression.getText())); };   // (… or a copy it has just taken)
const TALLY = /(_STATS|Stats|(^|\.)stats)(\.|$)/;
const noteWrite = (u: Unit, key: string, node: any, loose = false, f?: { key: string; file: string }) => {
  if (TALLY.test(key) || (f && scratchOf(f)) || madeHere(u, node)) return;
  const how = f ? carriedOf(f) : CARRIED_NAMES.has(key) ? 'by name' : undefined;
  if (SAY_FIELDS.has(key)) u.acts.push({ kind: 'says', what: key, line: lineOf(node), loose });
  else if (how) u.acts.push({ kind: 'overwrites', what: `${key} (${how})`, line: lineOf(node), loose });
  else u.acts.push({ kind: 'keeps', what: key, line: lineOf(node), loose }); };

/** A CLOSURE KEPT IN A FIELD (`S.notes.push(() => this.hudNotes())`, `x.hook = () => …`) runs when whoever reads the field calls it, not where it was
 *  written: it is a unit of its own, and a function that reads the field reaches it */
const STORED = new Map<string, Unit[]>();
for (const sf of files) { const file = rel(sf.fileName); const v = (n: any) => {
    let target: any = null, fns: any[] = [];
    if (n.kind === SK.CallExpression && n.expression.kind === SK.PropertyAccessExpression && (n.expression.name.text === 'push' || n.expression.name.text === 'unshift' || n.expression.name.text === 'add' || n.expression.name.text === 'set')) { target = strip(n.expression.expression); fns = n.arguments.filter(isFn); }
    else if (n.kind === SK.BinaryExpression && n.operatorToken.kind === SK.EqualsToken) { target = strip(n.left); const r = strip(n.right); fns = isFn(r) ? [r] : r.kind === SK.ArrayLiteralExpression ? r.elements.filter(isFn) : []; }
    if (target?.kind === SK.PropertyAccessExpression && fns.length) { let decls: any[] = symOf(target.name)?.declarations ?? []; if (!decls.length) decls = memberUnder(target, target.name.text); const f = decls.map(fieldOf).find(Boolean);
      if (f) for (const F of fns) if (!unitOfNode.has(F)) { const parent = unitAt(F), u = mk(F, file, `${parent.name} › (kept in ${f.key})`), k = `${f.file}:${f.key}`; parent.refs.add(u.id); if (!STORED.has(k)) STORED.set(k, []); STORED.get(k)!.push(u); } }
    ts.forEachChild(n, v); }; v(sf); }

for (const sf of files) {
  const walk = (n: any) => {
    const u = unitAt(n);
    if (n.kind === SK.ReturnStatement && n.expression) { let x = n.parent; while (x && !isFn(x)) x = x.parent; if (x && unitOfNode.get(x) === u) u.answers = true; }
    if (n.kind === SK.ArrowFunction && n.body.kind !== SK.Block && unitOfNode.get(n) === u) u.answers = true;
    // (what makes an answer a judgement and not a field handed on: a comparison, a branch, a loop, sums, a search of a list)
    if (n.kind === SK.IfStatement || n.kind === SK.ConditionalExpression || n.kind === SK.SwitchStatement || n.kind === SK.ForStatement || n.kind === SK.ForOfStatement || n.kind === SK.WhileStatement || (n.kind === SK.BinaryExpression && ((n.operatorToken.kind >= SK.LessThanToken && n.operatorToken.kind <= SK.ExclamationEqualsEqualsToken) || n.operatorToken.kind === SK.AmpersandAmpersandToken || n.operatorToken.kind === SK.BarBarToken || n.operatorToken.kind === SK.QuestionQuestionToken || (n.operatorToken.kind >= SK.PlusToken && n.operatorToken.kind <= SK.PercentToken)))
      || n.kind === SK.DoStatement || n.kind === SK.ForInStatement || (n.kind === SK.PrefixUnaryExpression && n.operator === SK.ExclamationToken)
      || (n.kind === SK.CallExpression && n.expression.kind === SK.PropertyAccessExpression && (WORKS.has(n.expression.name.text) || (n.expression.expression.kind === SK.Identifier && n.expression.expression.text === 'Math')))) u.works++;
    if (n.kind === SK.Identifier && n.parent && !(n.parent.kind === SK.PropertyAccessExpression && n.parent.name === n) && !(n.parent.kind === SK.PropertyAssignment && n.parent.name === n) && !((n.parent.kind === SK.VariableDeclaration || n.parent.kind === SK.FunctionDeclaration || n.parent.kind === SK.ClassDeclaration || n.parent.kind === SK.MethodDeclaration || n.parent.kind === SK.Parameter || n.parent.kind === SK.PropertyDeclaration || n.parent.kind === SK.PropertySignature) && n.parent.name === n)) {
      const s = ts.isPartOfTypeNode(n) ? undefined : symOf(n), isCall = n.parent.kind === SK.CallExpression && n.parent.expression === n;
      if (n.parent.kind === SK.ShorthandPropertyAssignment && n.parent.name === n) { let v = checker.getShorthandAssignmentValueSymbol(n.parent); if (v && v.flags & ts.SymbolFlags.Alias) { try { v = checker.getAliasedSymbol(v); } catch { /* an alias with no target */ } } for (const d of v?.declarations ?? []) { const t = declUnit(d); if (t && t !== u) u.refs.add(t.id); } }   // (`{ sin, cos }` hands the functions on)
      if (isCall && TIMERS.has(n.text)) { /* a function handed to a recurring timer by its name */ const a0 = n.parent.arguments[0]; if (a0?.kind === SK.Identifier) for (const d of symOf(a0)?.declarations ?? []) { const t = declUnit(d); if (t) t.timer = true; } }
      for (const d of s?.declarations ?? []) { const t = (d.kind === SK.ClassDeclaration || d.kind === SK.ClassExpression) && !(n.parent.kind === SK.ExpressionWithTypeArguments) ? null : declUnit(d);   // (a class named is not its constructor run: `new` is, below)
        if (t && t !== u) { u.refs.add(t.id); if (isCall) u.callsOut.set(t.id, lineOf(n)); }
        const tv = topVarOf(d); if (tv) { const id = `${tv.file}:${tv.key}`, carried = TOP.has(id);
          if ((writeKind(n) || mutated(n)) && !TALLY.test(tv.key)) u.acts.push({ kind: carried ? 'overwrites' : 'keeps', what: `${tv.key} of ${tv.file.replace(/^src\//, '')}${carried ? ' (a module\'s, carried)' : ' (a module\'s)'}`, line: lineOf(n) });
          else if (carried) u.reads.add(id); }
        if (d.kind === SK.ShorthandPropertyAssignment) { const f = fieldOf(d); if (f && (writeKind(n) || mutated(n))) noteWrite(u, f.key, n, false, f); } }
      if (n.text === 'localStorage' || n.text === 'sessionStorage') { const p = n.parent; if (p.kind === SK.PropertyAccessExpression && (p.name.text === 'setItem' || p.name.text === 'removeItem' || p.name.text === 'clear')) u.acts.push({ kind: 'stores', what: `${n.text}.${p.name.text}`, line: lineOf(n) }); }
      if (n.text === 'fetch' && isCall) u.acts.push({ kind: 'stores', what: 'fetch', line: lineOf(n) });
      if (isCall) {
        if (n.text === 'crossToRust' || n.text === 'goneWithTick') { /* the function's whole answer (a guard for a tool's tap may stand before it): it has no body of its own */
          let root: any = null; for (let x = n.parent; x; x = x.parent) if (unitOfNode.get(x) === u) { root = x; break; } const b = root?.body, call = n.parent, last = b?.kind === SK.Block ? b.statements[b.statements.length - 1] : null;
          if (b && (b.kind !== SK.Block ? strip(b) === call : b.statements.length <= 2 && last?.kind === SK.ReturnStatement && !!last.expression && strip(last.expression) === call)) u.stub = n.text; }
        else if (n.text === 'crossing') { const a0 = n.parent.arguments[0]; u.acts.push({ kind: 'crosses', what: a0 && ts.isStringLiteralLike(a0) ? `crossing ${a0.text}` : a0?.kind === SK.TemplateExpression ? `crossing ${a0.head.text}…` : 'crossing', line: lineOf(n) }); } }
    }
    if (n.kind === SK.PropertyAccessExpression || (n.kind === SK.ElementAccessExpression && n.argumentExpression && ts.isStringLiteralLike(n.argumentExpression))) {
      const nameNode = n.kind === SK.PropertyAccessExpression ? n.name : n.argumentExpression, pname: string = nameNode.text; const called = n.parent?.kind === SK.CallExpression && n.parent.expression === n;
      let decls: any[] = (symOf(nameNode)?.declarations ?? []).filter((d: any) => d.kind !== SK.IndexSignature), under = false;
      if (!decls.length && loosely(typeAt(n.expression))) { const m = memberUnder(n, pname); if (m.length) { decls = m; under = true; } }
      if (/^face_/.test(pname) && called) u.acts.push({ kind: 'crosses', what: pname, line: lineOf(n) });
      const real = decls.filter((d: any) => !d.getSourceFile().isDeclarationFile);
      const wr = writeKind(n), plainWrite = wr && n.parent.kind === SK.BinaryExpression && n.parent.operatorToken.kind === SK.EqualsToken;
      for (const d of real) { if ((d.kind === SK.SetAccessor && !wr) || (d.kind === SK.GetAccessor && plainWrite)) continue;   // (a read runs the getter, a write the setter: not both)
        const t = declUnit(d); if (t && t !== u) { u.refs.add(t.id); if (called || d.kind === SK.GetAccessor || d.kind === SK.SetAccessor) u.callsOut.set(t.id, lineOf(n)); }
        if (d.kind === SK.MethodSignature || (d.kind === SK.MethodDeclaration && !d.body) || d.kind === SK.PropertySignature) { /* a member of a type: the classes that fit the receiver, and the page's own objects with a function of the name */
          PENDING.push({ u, name: pname, recv: tids(under ? typeUnder(n.expression) : typeAt(n.expression)), runs: called || d.kind === SK.MethodSignature || !!(d.type && d.type.kind === SK.FunctionType), write: wr }); }
        if (t && writeKind(n) && SK.MethodDeclaration === d.kind) t.replaced = true;   // (a method given a new body on the object: the host hangs its own, src/host/rustgame.ts rustAttach)
        const f = fieldOf(d); if (f) { if (wr || mutated(n)) noteWrite(u, f.key, n, under, f); else { if (carriedOf(f)) u.reads.add(f.key); if (STORED.has(`${f.file}:${f.key}`)) u.kept.add(`${f.file}:${f.key}`); } } }
      if (!decls.length && loosely(typeAt(n.expression))) { /* a name off a value nothing types */
        if (!OWN_NAMES.has(pname) && (called || (n.parent?.kind === SK.PropertyAccessExpression && (n.parent.name.text === 'call' || n.parent.name.text === 'bind' || n.parent.name.text === 'apply')))) { for (const c of BY_NAME.get(pname) ?? []) u.loose.add(c.id); for (const c of LIT_BY_NAME.get(pname) ?? []) u.loose.add(c.id); }
        const cands = FIELDS_BY_NAME.get(pname);
        if (cands && (writeKind(n) || mutated(n))) { const carried = [...cands].filter(k => CARRIED_NAMES.has(k)), pick = carried.length ? carried : [...cands], n0 = u.acts.length; noteWrite(u, pick.length === 1 ? pick[0] : `${pname} of ${pick.slice(0, 3).join(' | ')}${pick.length > 3 ? ` | +${pick.length - 3}` : ''}`, n, true); if (pick.length > 1 && carried.length && u.acts.length > n0 && u.acts[n0].kind === 'keeps') u.acts[n0].kind = 'overwrites'; }
        else if (cands) for (const k of cands) if (CARRIED_NAMES.has(k)) { u.reads.add(k); break; } } }
    if (n.kind === SK.NewExpression) { const s = symOf(n.expression); for (const d of s?.declarations ?? []) { const t = declUnit(d); if (t && t !== u) u.refs.add(t.id); } }
    // (the hand-overs)
    if ((n.kind === SK.CallExpression || n.kind === SK.NewExpression) && n.arguments) for (const a of n.arguments) hand(a, ctxOf(a));
    else if (n.kind === SK.PropertyAssignment && n.parent.kind === SK.ObjectLiteralExpression) hand(n.initializer, ctxOf(n.initializer));
    else if (n.kind === SK.ShorthandPropertyAssignment) { const C = ctxOf(n.parent), p = C && checker.getPropertyOfType(checker.getNonNullableType(C), n.name.text); if (p) hand(n.name, checker.getTypeOfSymbol(p)); }
    else if ((n.kind === SK.VariableDeclaration || n.kind === SK.PropertyDeclaration || n.kind === SK.Parameter) && n.type && n.initializer) hand(n.initializer, checker.getTypeFromTypeNode(n.type));
    else if (n.kind === SK.BinaryExpression && n.operatorToken.kind === SK.EqualsToken) hand(n.right, typeAt(n.left));
    else if (n.kind === SK.ReturnStatement && n.expression) hand(n.expression, ctxOf(n.expression));
    else if (n.kind === SK.ArrowFunction && n.body.kind !== SK.Block) hand(n.body, ctxOf(n.body));
    else if (n.kind === SK.AsExpression) hand(strip(n.expression), checker.getTypeFromTypeNode(n.type));
    else if (n.kind === SK.ClassDeclaration) for (const h of n.heritageClauses ?? []) for (const t of h.types) { const cs = n.name && checker.getSymbolAtLocation(n.name); if (cs) for (const id of tids(checker.getTypeAtLocation(t))) HANDED.add(`${ts.getSymbolId(cs)}:${id}`); }
    ts.forEachChild(n, walk); };
  walk(sf); }

for (const u of UNITS.values()) for (const k of u.kept) for (const c of STORED.get(k) ?? []) if (c !== u) u.loose.add(c.id);
for (const P of PENDING) { /* a method runs where the member is called (or is a function to hand on); a getter where it is read, a setter where it is written */
  const named = [...(BY_NAME.get(P.name) ?? []), ...(LIT_BY_NAME.get(P.name) ?? [])].filter(c => c.acc ? (c.acc === 'set') === P.write : P.runs);
  if (!P.recv.length) { for (const c of named) P.u.loose.add(c.id); continue; }   // (a receiver nothing types: by the name)
  for (const c of named) { if (c.lit) { const ids = tids(ctxOf(c.lit)); if (ids.some(i => P.recv.includes(i))) P.u.loose.add(c.id); } else { const k = classId(c); if (P.recv.some(i => HANDED.has(`${k}:${i}`))) P.u.loose.add(c.id); } } }

// ---- who says: a function that writes the log or a bubble is a sayer, so is a `say` / `msg` that only hands on to one; who calls a sayer says
{ const sayers = new Set<string>(); for (const u of UNITS.values()) if (u.acts.some(a => a.kind === 'says')) sayers.add(u.id);
  for (let grew = true; grew;) { grew = false; for (const u of UNITS.values()) if (!sayers.has(u.id) && /(^|\.)(say|msg)$/.test(u.name) && [...u.callsOut.keys()].some(t => sayers.has(t))) { sayers.add(u.id); grew = true; } }
  for (const u of UNITS.values()) for (const [t, line] of u.callsOut) if (sayers.has(t) && !sayers.has(u.id)) u.acts.push({ kind: 'says', what: `through ${UNITS.get(t)!.name}`, line }); }

// ---- reach
const reachFrom = (roots: Iterable<string>, stop: Set<string> = new Set()): Map<string, string | null> => { const from = new Map<string, string | null>(); const q: string[] = []; for (const r of roots) if (!from.has(r) && !stop.has(r)) { from.set(r, null); q.push(r); }
  for (let i = 0; i < q.length; i++) { const id = q[i], u = UNITS.get(id)!; for (const t of [...u.refs, ...u.loose]) if (!from.has(t) && !stop.has(t) && UNITS.has(t) && !UNITS.get(t)!.top) { from.set(t, id); q.push(t); } } return from; };   // (a file's own top-level lines run once, as it loads: naming the module does not run them again)
/** the files a page's entry loads, by its imports */
const importsOf = (entry: string): Set<string> => { const seen = new Set<string>(), q = [entry]; while (q.length) { const f = q.pop()!; if (seen.has(f)) continue; seen.add(f); const sf = program.getSourceFile(join(ROOT, f)); if (!sf) continue;
  const v = (n: any) => { const spec = (n.kind === SK.ImportDeclaration || n.kind === SK.ExportDeclaration) && n.moduleSpecifier ? n.moduleSpecifier : n.kind === SK.CallExpression && n.expression.kind === SK.ImportKeyword && n.arguments[0] && ts.isStringLiteralLike(n.arguments[0]) ? n.arguments[0] : null;
    if (spec && !(n.kind === SK.ImportDeclaration && n.importClause?.isTypeOnly)) { const r = ts.resolveModuleName(spec.text, sf.fileName, parsed.options, ts.sys).resolvedModule; if (r && !r.isExternalLibraryImport) { const t = rel(r.resolvedFileName); if (t.startsWith('src/')) q.push(t); } } ts.forEachChild(n, v); }; v(sf); } return seen; };
const gameFiles = importsOf(GAME_ENTRY);
const tops = [...UNITS.values()].filter(u => u.top);
const pageRoots = tops.filter(u => gameFiles.has(u.file) || SIM.test(u.file)).map(u => u.id), otherRoots = tops.filter(u => !gameFiles.has(u.file) && !SIM.test(u.file)).map(u => u.id);
const frameRoots: string[] = [];
for (const r of ROOTS.frame) { if (!UNITS.has(r.id)) { console.error(`page-decides.roots.json names ${r.id}, which is no function of this tree`); process.exit(2); } frameRoots.push(r.id); }
for (const u of UNITS.values()) if (STEP_FILES.has(u.file) && !u.top) frameRoots.push(u.id);
for (const u of UNITS.values()) if (u.timer && gameFiles.has(u.file)) frameRoots.push(u.id);
/** WHERE THE FRAME'S REACH STOPS (page-decides.roots.json `stops`, each with why and with what stands behind it): the functions that stand a game or put one
 *  back (a show that restarts its level stands a game, and what runs then is a start), the hand's own (a menu, the keys the frame polls: what a key opens
 *  runs once, for the hand that pressed it), a save, and the networked page's own walk. A function reached past one stop and by no other road is sorted by
 *  that stop's word. */
const stops = new Set<string>(); for (const r of ROOTS.stops) { if (!UNITS.has(r.id)) { console.error(`page-decides.roots.json names ${r.id}, which is no function of this tree`); process.exit(2); } stops.add(r.id); }
const reachPage = reachFrom(pageRoots), reachFrame = reachFrom(frameRoots, stops), reachOther = reachFrom(otherRoots);
const BEHINDS = [...new Set(ROOTS.stops.map(r => r.behind))], reachBehind = new Map<string, Map<string, string | null>>(); for (const b of BEHINDS) reachBehind.set(b, reachFrom(frameRoots, new Set(ROOTS.stops.filter(r => r.behind !== b).map(r => r.id))));
if (flag('--hubs')) { /* which functions the frame's reach hangs on: how many it alone brings in (to find a stand that is not named yet) */ const kids = new Map<string, string[]>(); for (const [id, from] of reachFrame) if (from) { if (!kids.has(from)) kids.set(from, []); kids.get(from)!.push(id); } const size = new Map<string, number>(); const sz = (id: string): number => { let n = 1; for (const k of kids.get(id) ?? []) n += sz(k); size.set(id, n); return n; }; for (const r of frameRoots) if (reachFrame.get(r) === null) sz(r);
  console.log(`the frame reaches ${reachFrame.size} functions; the largest branches:`); for (const [id, n] of [...size].sort((a, b) => b[1] - a[1]).slice(0, +(opt('--hubs') ?? 60) || 60)) console.log(`  ${String(n).padStart(5)}  ${id.replace(/^src\//, '')}   ← ${String(reachFrame.get(id) ?? 'a root').replace(/^src\//, '')}`); process.exit(0); }
const when = (u: Unit): string => { const id = u.id.replace(/#\d+$/, ''), c = CENSUS.get(id); if (reachFrame.has(u.id)) return c ? `every frame (the frame reaches it; seen ${c} a frame)` : 'every frame (the frame reaches it)'; if (c) return `every frame (seen ${c} a frame)`; { const b = BEHINDS.filter(b => reachBehind.get(b)!.has(u.id)); if (b.length) return `behind ${b.join(', ')}`; } if (reachPage.has(u.id)) return 'start or hand'; if (reachOther.has(u.id)) return 'other page'; return 'dead'; };

// ---- rows
const HAND_FILE = join(import.meta.dir, 'page-decides.rows.tsv');
const HAND = existsSync(HAND_FILE) ? readFileSync(HAND_FILE, 'utf8').split('\n').filter(l => l && !l.startsWith('#')).map(l => l.split('\t')).map(c => ({ pat: c[0], sort: c[1], kind: c[2] ?? '', reason: c[3] ?? '', pin: c[4] === undefined || c[4] === '' ? -1 : +c[4], used: 0 })) : [];
const SORTS = new Set(['decides', 'moving', 'picture', 'hand', 'host']);
for (const h of HAND) if (!SORTS.has(h.sort) || !h.reason) { console.error(`page-decides.rows.tsv: ${h.pat}: "${h.sort}" is no sort (${[...SORTS].join(', ')}), or the line has no reason`); process.exit(2); }
const glob = (pat: string) => new RegExp('^' + pat.replace(/[.+^${}()|[\]\\]/g, '\\$&').replace(/\*/g, '.*') + '$');
const HANDRE = HAND.map(h => ({ h, re: glob(h.pat) }));
interface Row { sort: string; kind: string; when: string; unit: Unit; judges: string; twin: string; reason: string }
const rows: Row[] = [];
/** reads the game's carried state: itself, or through a getter or a helper of the simulation's it calls (four steps down) */
const readsMemo = new Map<string, boolean>();
const readsGame = (u: Unit, depth = 0): boolean => { if (u.reads.size) return true; if (depth >= 4) return false; const m = readsMemo.get(u.id); if (m !== undefined) return m; readsMemo.set(u.id, false); let r = false; for (const t of u.callsOut.keys()) { const c = UNITS.get(t); if (c && SIM.test(c.file) && readsGame(c, depth + 1)) { r = true; break; } } readsMemo.set(u.id, r); return r; };
for (const u of UNITS.values()) { if (u.file === 'src/game/crossing.ts') continue;
  const twin = SIM.test(u.file) && !u.stub ? rustTwin(u) : '';
  const judges = !u.acts.length && twin && u.answers && readsGame(u) && u.works > 0 ? twin : '';
  if (!u.acts.length && !judges && !u.stub) continue;
  const w = when(u), frame = w.startsWith('every frame');
  if (u.stub && (!reachFrame.has(u.id) || u.replaced)) continue;   // (a crossing the frame cannot reach is no row: a start or a hand that calls it is listed by the deletion's own tables)
  let sort = frame ? (u.stub ? 'throws' : 'UNSORTED') : w.startsWith('behind') ? (w === 'behind the networked page' ? 'networked page' : w === 'behind a save' ? 'save' : w === 'behind a tape\'s page' ? 'other page' : 'start or hand') : w, kind = '';
  let reason = frame ? '' : w.startsWith('behind') ? `the frame reaches it only past ${w.slice(7)} (page-decides.roots.json says where its reach stops, and why)` : w === 'start or hand' ? 'no root of the frame reaches it: it runs as the page loads or a game is stood, or on a key, a click or a tool\'s hook' : w === 'other page' ? 'reached only from a page that plays no Rust game' : 'no page reaches it';
  for (const { h, re } of HANDRE) if (re.test(u.id)) { kind = h.kind; if (frame) { h.used++; if (!u.stub) { sort = h.sort; reason = h.reason; } } break; }   // (a crossing the frame reaches stays `throws`, whatever its file's line says)
  rows.push({ sort, kind: kind || u.file.replace(/^src\//, '').replace(/\.ts$/, ''), when: w, unit: u, judges, twin, reason }); }
// A READ OF THE RECORD is no decision: a one-line function of the simulation's that hands on one thing with at most a default or one comparison (`get locked()
// { return this.g(DC.locked) !== 0; }`, `pairOf(gd) { return gd.unit?.pair ?? null; }`) and calls nothing that decides. It is how the page reads the game's
// record, and it stays when the page only draws. Found by the program (never by a line of the sorts' file), and only among rows that judge: a row that acts
// is never one. A row that calls a deciding row is that row's pass-through and goes with it.
{ const READ_WORKS = 2, cand = new Map(rows.filter(r => r.sort === 'decides' && !r.unit.acts.length && r.judges && r.unit.lines <= 1 && r.unit.works <= READ_WORKS).map(r => [r.unit.id, r])), deciding = new Set(rows.filter(r => r.sort === 'decides' || r.sort === 'UNSORTED' || r.sort === 'throws').map(r => r.unit.id));
  for (let again = true; again;) { again = false; for (const [id, r] of cand) if ([...r.unit.callsOut.keys()].some(t => t !== id && deciding.has(t) && !cand.has(t))) { cand.delete(id); again = true; } }
  for (const r of cand.values()) { r.sort = 'reads'; r.reason = 'a one-line read of the game\'s record (a field handed on with at most a default or one comparison, calling nothing that decides): how the page reads what the game keeps'; } }
const ORDER = ['UNSORTED', 'throws', 'decides', 'moving', 'reads', 'hand', 'host', 'picture', 'networked page', 'save', 'start or hand', 'other page', 'dead'];
rows.sort((a, b) => ORDER.indexOf(a.sort) - ORDER.indexOf(b.sort) || a.kind.localeCompare(b.kind) || a.unit.id.localeCompare(b.unit.id));
const actsText = (r: Row, max = 6) => { const by = new Map<string, Set<string>>(); for (const a of r.unit.acts) { if (!by.has(a.kind)) by.set(a.kind, new Set()); by.get(a.kind)!.add((a.loose ? '~' : '') + a.what); } const parts = [...by].map(([k, v]) => `${k} ${[...v].slice(0, max).join(', ')}${v.size > max ? `, +${v.size - max}` : ''}`); if (r.judges) parts.push(`judges (${r.unit.reads.size ? `reads ${r.unit.reads.size} carried fields` : 'reads carried state through what it calls'}; Rust has ${r.judges})`); if (r.unit.stub) parts.push(`THROWS on the page (${r.unit.stub})`); return parts.join(' · ') + (r.twin && !r.judges ? ` · [Rust has ${r.twin}]` : ''); };

for (const why of (opt('--why') ?? '').split(',').filter(Boolean)) { const u = UNITS.get(why) ?? [...UNITS.values()].find(x => x.id.endsWith(':' + why) || x.name === why); if (!u) { console.error(`no function ${why}`); continue; }
  console.log(`${u.id} (line ${u.line}, ${u.lines} lines) · ${when(u)}`); for (const [label, R] of [['the frame', reachFrame], ['the game\'s page', reachPage], ['another page', reachOther]] as const) { if (!R.has(u.id)) { console.log(`  ${label}: does not reach it`); continue; } const chain: string[] = []; for (let x: string | null | undefined = u.id; x; x = R.get(x)) chain.push(`${x.replace(/^src\//, '')}${x !== u.id && UNITS.get(R.get(chain.length ? chain[chain.length - 1] : x) ?? '') ? '' : ''}`); console.log(`  ${label}: ${chain.reverse().join('  →  ')}`); }
  for (const a of u.acts) console.log(`  ${u.file}:${a.line}  ${a.kind} ${a.loose ? '~' : ''}${a.what}`); if (u.reads.size) console.log(`  reads ${u.reads.size} carried fields: ${[...u.reads].slice(0, 12).join(', ')}`); const t = rustTwin(u); if (t) console.log(`  Rust has ${t}`); }
if (opt('--why')) process.exit(0);

if (flag('--doors')) { /* THE DOORS: of the rows that decide, the ones the page's own code walks in through (its caller on the frame's road is no deciding row),
   each with how many deciding rows stand behind it. A door is one question the page asks: what a view of the game has to answer in its place. */
  const dec = new Map(rows.filter(r => r.sort === 'decides').map(r => [r.unit.id, r])), behind = new Map<string, number>();
  for (const id of dec.keys()) { let d = id; for (let up = reachFrame.get(d); up && dec.has(up); up = reachFrame.get(d)) d = up; behind.set(d, (behind.get(d) ?? 0) + 1); }
  console.log(`${dec.size} rows decide, behind ${behind.size} doors:`); for (const [d, n] of [...behind].sort((x, y) => dec.get(x[0])!.kind.localeCompare(dec.get(y[0])!.kind) || y[1] - x[1])) { const r = dec.get(d)!; console.log(`  ${String(n).padStart(4)}  ${r.kind}\t${d.replace(/^src\//, '')}\t← ${String(reachFrame.get(d) ?? 'seen in the frame').replace(/^src\//, '')}\t${r.unit.acts.length ? 'acts' : 'judges'}`); } process.exit(0); }
const bySort = new Map<string, number>(); for (const r of rows) bySort.set(r.sort, (bySort.get(r.sort) ?? 0) + 1);
console.log(`page-decides: ${rows.length} functions under src/ act on the game's copy, judge it again or would throw · ` + ORDER.filter(s => bySort.has(s)).map(s => `${s} ${bySort.get(s)}`).join(' · '));
if (flag('--kinds') || !(opt('--tsv') || opt('--md'))) { for (const s of ORDER) { const R = rows.filter(r => r.sort === s); if (!R.length) continue; const k = new Map<string, number>(); for (const r of R) k.set(r.kind, (k.get(r.kind) ?? 0) + 1); const all = flag('--kinds') || s === 'UNSORTED' || s === 'decides' || s === 'throws'; console.log(`  ${s}: ${[...k].sort((a, b) => b[1] - a[1]).slice(0, all ? 1000 : 12).map(([n, c]) => `${n} ${c}`).join(', ')}${k.size > 12 && !all ? `, … (${k.size} kinds)` : ''}`); } }
// (a line of the sorts' file that sorts no every-frame row any more is to come out; and a line that is not `decides` says how many rows it sorted when a
//  person last read them (its fifth column, written by --pin): more than that, and somebody has to read the new ones)
const unused = HAND.filter(h => !h.used); if (unused.length) console.log(`  ${unused.length} lines of page-decides.rows.tsv sort no every-frame row any more (take them out): ${unused.slice(0, 12).map(h => h.pat).join(', ')}`);
const grown = HAND.filter(h => h.sort !== 'decides' && h.used > h.pin); if (grown.length && !flag('--pin')) console.log(`  ${grown.length} lines of page-decides.rows.tsv sort more rows than when they were last read (read the new rows, then --pin): ${grown.slice(0, 12).map(h => `${h.pat} ${h.pin < 0 ? 'never read' : h.pin}→${h.used}`).join(', ')}`);
if (flag('--pin')) { const by = new Map(HAND.map(h => [h.pat, h])); writeFileSync(HAND_FILE, readFileSync(HAND_FILE, 'utf8').split('\n').map(l => { if (!l || l.startsWith('#')) return l; const c = l.split('\t'), h = by.get(c[0])!; return h.sort === 'decides' ? c.slice(0, 4).join('\t') : [...c.slice(0, 4), String(h.used)].join('\t'); }).join('\n')); console.log(`  page-decides.rows.tsv: the counts written (${HAND.filter(h => h.sort !== 'decides').length} lines)`); }
const tsvTo = opt('--tsv'); if (tsvTo) writeFileSync(tsvTo, ['sort\tkind\tfile\tfunction\tline\tlines\twhen it runs\twhat it does\treason\tworks', ...rows.map(r => [r.sort, r.kind, r.unit.file, r.unit.name, r.unit.line, r.unit.lines, r.when, actsText(r, 40), r.reason, r.unit.works].join('\t'))].join('\n') + '\n');
const mdTo = opt('--md'); if (mdTo) { const L: string[] = []; for (const s of ORDER) { const R = rows.filter(r => r.sort === s); if (!R.length) continue; if (s === 'start or hand' || s === 'other page' || s === 'dead' || s === 'networked page' || s === 'save') { const k = new Map<string, number>(); for (const r of R) k.set(r.kind, (k.get(r.kind) ?? 0) + 1); L.push(`## ${s} (${R.length})`, '', `Sorted by the program: ${R[0].reason}. By file: ${[...k].sort((a, b) => b[1] - a[1]).map(([n, c]) => `${n} ${c}`).join(', ')}.`, ''); continue; }
    L.push(`## ${s} (${R.length})`, '', '| kind | function | what it does | reason |', '|---|---|---|---|'); for (const r of R) L.push(`| ${r.kind} | \`${r.unit.file.replace(/^src\//, '')}\` ${r.unit.name.replace(/\|/g, '\\|')} | ${actsText(r).replace(/\|/g, '\\|')} | ${r.reason} |`); L.push(''); } writeFileSync(mdTo, L.join('\n')); }
if (flag('--check')) process.exit((bySort.get('UNSORTED') ?? 0) + (bySort.get('decides') ?? 0) + (bySort.get('throws') ?? 0) + unused.length + grown.length ? 1 : 0);
