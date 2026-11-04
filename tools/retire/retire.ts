// RETIRE A SWITCH. A rule ships as a switch (CLAUDE.md, "A new rule is a switch"): a row of a knob table, an address word that puts it off, a stand word that
// raises it on the frozen stand, a FROZEN_HOLDS line, a debugger toggle, a journal line for files from before the key. Once the rule is settled all of that is
// a second way through the code that nobody takes. This tool takes the rows to retire with the value each one has in the page, and leaves ONE way:
//   - every read of the row becomes its value and the code is folded round it (`if (ROW && x) A else B` → `if (x) A else B`; `ROW ? a : b` → `a`; `!ROW` → false …),
//   - a local `const` that became a constant by that is folded on in the same way and goes,
//   - every write of the row goes: the page's address word (profile.ts pageSwitches, or a module's own read of the address), the stand's hold (standup.ts,
//     headless.ts), the journal's line for an old file, the debugger's toggle, and a tool's own raise or put-back,
//   - the row goes from its table, from FROZEN_HOLDS, and its stand word from STAND_ENV when no hold names it any more; an address word nothing reads any more
//     goes from every list of words (ui/replaykey.ts VISIT_FLAGS …),
//   - what only the dropped branch used goes with it (a function, a method, a table's number, an import, a kept copy in a tool: every use of it was in text
//     this run removed), to a fixed point.
// It REFUSES a row the game itself writes while it runs (that is state, not a switch), that the player owns (a row of Options), that an object literal in
// the game holds at another value, that the game reads through a list of keys, on another thing of the table's type or as a module loads, and a row whose
// value in the page — by `--page dump.json`, what a stand-up of the page left every table at — is not the one asked for (`--trust` vouches for one), and
// says why (exit 3: the other rows are still done).
// It never folds a call away because of its name: a call is harmless only where it is one of the language's own reading methods, a function whose body
// only reads, or one the caller vouches for (`--pure`).
// It does not touch docs: `--list` prints the retired rows as lines for a write-up.
//
//   TPS_PROFILE=shipped bun run tools/qa/stand-dump.ts --out page.json             what the page gives every row (the tool holds each value asked for to it)
//   bun run tools/retire/retire.ts --page page.json ORDERS.fileSpread=true NOSHOVE.on=true          a dry run: the report, nothing written
//   bun run tools/retire/retire.ts --page page.json --rows rows.json --write [--report out.json]    rows.json = [{ "row": "ORDERS.fileSpread", "value": true }, …]
//   … --pure locks,allowed      functions the caller vouches only read, so a condition that calls one beside a retired row can go (see VOUCHED)
//   … --trust ROW,ROW           rows whose value is the page's on the caller's word, where the tool cannot read the page's write of it (see TRUSTED)
//   … --late ROW,ROW            rows a refusal said are read as a module loads, in a function the walk reached, and that the caller has read the chain of and vouches are not (see LATE)
//   … --drop file.ts:name       a declaration out whole, with the statements that call it · --drop-at file.ts:line[:words]  one statement of a check
// THE PROOF OF A RUN is not this tool's: both typechecks, then the page's stand run for run the same before and after (tools/qa/playthrough.ts digests under
// TPS_PROFILE=shipped), and the frozen stand after equal to the frozen stand before with the retired rows' stand words raised. docs/agents/qa.md, "Retiring a switch".
// THE TOOL'S OWN CHECK: tools/retire/retire-check.ts runs it on a small tree of its own (tools/retire/fixture) and holds that every case there gives the same
// answers, draws the same dice and counts the same calls after the fold as before it, and that the folded tree still compiles.
//
// TypeScript's own compiler does the reading (vendor/typescript, as `bun run typecheck`; TS_LIB=<path to typescript.js> names another).
import { readFileSync, writeFileSync, readdirSync, statSync, existsSync } from 'node:fs';
import { join, resolve, relative } from 'node:path';

const ROOT = process.env.RETIRE_ROOT ? resolve(process.env.RETIRE_ROOT) : resolve(import.meta.dir, '../..');   // (RETIRE_ROOT: another tree laid out as this one — tools/retire/retire-check.ts runs the tool on its own small one)
const TS_LIB = process.env.TS_LIB ?? join(ROOT, 'vendor/typescript/lib/typescript.js');
if (!existsSync(TS_LIB)) { console.error(`no TypeScript at ${TS_LIB} (vendor/typescript, or TS_LIB=<path to typescript.js>)`); process.exit(2); }
const tsMod: any = await import(TS_LIB); const ts: any = tsMod.default ?? tsMod;
const SK = ts.SyntaxKind;

type Value = boolean | number | string;
type Row = { row: string; value: Value; hold?: boolean };   // hold: a NUMBER the frozen stand keeps at an old value — the hold and the page's write go, the knob stays at `value`
type Span = { start: number; end: number };

// ---- arguments
const argv = process.argv.slice(2), flag = (n: string) => argv.includes(n), arg = (n: string) => { const i = argv.indexOf(n); return i < 0 ? null : argv[i + 1] ?? null; };
const WRITE = flag('--write'), VERBOSE = flag('--verbose');
/** `--pure name,name`: functions and methods the caller vouches only READ (a query the tool cannot see to the bottom of: `rails.locks(k)`): a condition that calls
 *  one of them beside a retired row can then be folded away. The command line is the record of what was vouched for. */
const VOUCHED = new Set((arg('--pure') ?? '').split(',').map(x => x.trim()).filter(Boolean));
/** `--trust ROW,ROW`: rows whose value the caller vouches IS the page's, where `--page`'s dump does not hold the row, or its write rides a row that stays and
 *  the caller has read that this is meant. Again the command line is the record. */
const TRUSTED = new Set((arg('--trust') ?? '').split(',').map(x => x.trim()).filter(Boolean));
/** --late ROW,ROW: the caller has read the chain of calls a refusal named and vouches that the row is NOT read before the page writes it (the walk follows every call a function could make, not the ones it does make as the module loads). Only a read inside a function is vouched for: one at a file's own top level is load time whatever anyone says. */
const LATE = new Set((arg('--late') ?? '').split(',').map(x => x.trim()).filter(Boolean));
const rows: Row[] = [];
{ const f = arg('--rows'); if (f) for (const r of JSON.parse(readFileSync(f, 'utf8')) as Row[]) rows.push(r);
  for (const a of argv) { const m = /^([A-Za-z_][\w.]*)=(.*)$/.exec(a); if (!m || a.startsWith('--')) continue; const raw = m[2], q = /^'(.*)'$|^"(.*)"$/.exec(raw);
    if (raw !== 'true' && raw !== 'false' && !(raw.trim() !== '' && Number.isFinite(Number(raw))) && !q) { console.error(`${a}: a value is true, false, a number or a 'quoted word' (a row asked for by a typo must not be folded to the wrong branch)`); process.exit(2); }
    rows.push({ row: m[1], value: raw === 'true' ? true : raw === 'false' ? false : q ? q[1] ?? q[2] : Number(raw) }); } }
const DROPS: string[] = [], DROP_AT: string[] = []; for (let i = 0; i < argv.length; i++) { if (argv[i] === '--drop' && argv[i + 1]) DROPS.push(argv[i + 1]); if (argv[i] === '--drop-at' && argv[i + 1]) DROP_AT.push(argv[i + 1]); }
{ const f = arg('--drop-lines'); if (f) for (const l of JSON.parse(readFileSync(f, 'utf8')) as string[]) DROP_AT.push(l); }
if (!rows.length && !DROPS.length && !DROP_AT.length) { console.error('no rows: ROW=value … or --rows file.json (or --drop file.ts:name …)'); process.exit(2); }

// ---- the program: src, tools and the types, one language service over all of it (a row's uses in a check are found with its uses in the game)
/** every .ts under a folder, in sorted order (the order files are read in is the order a row's reasons are found in) */
function walk(dir: string, out: string[]): string[] {
  for (const n of readdirSync(dir).sort()) { if (n === 'node_modules' || n === 'vendor' || n.startsWith('.')) continue; const p = join(dir, n), st = statSync(p); if (p === join(ROOT, 'tools/retire/fixture')) continue;   // (the check's own small tree is not this program's)
    if (st.isDirectory()) walk(p, out); else if (/\.ts$/.test(n)) out.push(p); }
  return out;
}
const files = [...walk(join(ROOT, 'src'), []), ...walk(join(ROOT, 'tools'), []), ...(existsSync(join(ROOT, 'types')) ? walk(join(ROOT, 'types'), []) : [])];
const options = { ...ts.convertCompilerOptionsFromJson(JSON.parse(readFileSync(join(ROOT, 'tsconfig.json'), 'utf8')).compilerOptions, ROOT).options, allowImportingTsExtensions: true, noEmit: true };
const texts = new Map<string, string>(); const textOf = (f: string) => { let t = texts.get(f); if (t === undefined) { t = readFileSync(f, 'utf8'); texts.set(f, t); } return t; };
const host = { getScriptFileNames: () => files, getScriptVersion: () => '1', getScriptSnapshot: (f: string) => existsSync(f) ? ts.ScriptSnapshot.fromString(textOf(f)) : undefined,
  getCurrentDirectory: () => ROOT, getCompilationSettings: () => options, getDefaultLibFileName: (o: any) => ts.getDefaultLibFilePath(o), fileExists: ts.sys.fileExists, readFile: ts.sys.readFile,
  readDirectory: ts.sys.readDirectory, directoryExists: ts.sys.directoryExists, getDirectories: ts.sys.getDirectories };
const service = ts.createLanguageService(host, ts.createDocumentRegistry());
const program = service.getProgram(), checker = program.getTypeChecker();
const rel = (f: string) => relative(ROOT, f);
const isTool = (f: string) => rel(f).startsWith('tools/');
const sfOf = (f: string) => program.getSourceFile(f);
const at = (sf: any, pos: number) => `${rel(sf.fileName)}:${sf.getLineAndCharacterOfPosition(pos).line + 1}`;

// ---- what the run decides, by file
type FilePlan = { subs: Map<any, Value>; del: Set<any>; unassign: Set<any>; bare: Set<any>; hole: Set<any>; dropped: Span[] };   // hole: a place in a pattern left empty (`[, x] = kept`)   // bare: a `const x = f();` nothing reads any more — f() still runs, only the name goes
const plans = new Map<string, FilePlan>();
/** the journal's `if`s whose condition asks a recording for a retired row's own word */
const recordingIfs = new Set<any>();
const plan = (f: string) => { let p = plans.get(f); if (!p) { p = { subs: new Map(), del: new Set(), unassign: new Set(), bare: new Set(), hole: new Set(), dropped: [] }; plans.set(f, p); } return p; };
const report = { retired: [] as string[], refused: [] as { row: string; why: string }[], notes: [] as string[], toolWrites: [] as { row: string; at: string; kind: string; text: string }[],
  dead: [] as string[], words: [] as string[], standWords: [] as string[], hooks: [] as string[], controls: [] as string[], files: 0 };
const note = (s: string) => { report.notes.push(s); };

// ---- small readings of the tree
const unparen = (n: any): any => { while (n && (n.kind === SK.ParenthesizedExpression || n.kind === SK.AsExpression || n.kind === SK.NonNullExpression || n.kind === SK.SatisfiesExpression)) n = n.expression; return n; };
const upParen = (n: any): any => { while (n.parent && n.parent.kind === SK.ParenthesizedExpression) n = n.parent; return n; };
const PURE_CALLS = /^(pinned|envOf|String|Number|Boolean|Number\.isFinite)$/;
/** is this name the language's own (declared by TypeScript's library files, or a types package), not the game's? `Map.get` is; a game object's `get` is not */
const libOwn = (sym: any): boolean => { const ds: any[] = sym?.declarations ?? []; return ds.length > 0 && ds.every(d => { const f = d.getSourceFile(); return program.isSourceFileDefaultLibrary(f) || /\/node_modules\//.test(f.fileName); }); };
const PURE_METHODS = new Set(['has', 'get', 'includes', 'indexOf', 'startsWith', 'endsWith', 'test', 'at', 'slice', 'some', 'every', 'find', 'findIndex', 'filter', 'map', 'join', 'keys', 'values', 'entries', 'toFixed', 'toString', 'trim', 'toLowerCase', 'toUpperCase', 'abs', 'min', 'max', 'floor', 'ceil', 'round', 'sign', 'hypot', 'sqrt', 'isArray', 'isFinite', 'parse', 'stringify', 'trunc', 'pow', 'isInteger', 'isNaN', 'concat', 'split', 'charCodeAt', 'padStart', 'padEnd']);
const CALLBACK_METHODS = new Set(['some', 'every', 'find', 'findIndex', 'filter', 'map']);   // (the reading methods above that CALL what they are handed)
let pureDepth = 0;
/** every getter and every method (or function kept under a key) of the game's own, by NAME, and every class by the class it extends. A name read through
 *  an interface (or on an untyped thing) may be a getter or any object's method at run time: a READ of it is left out only where no getter of that name in
 *  the game writes anything (writesFn), a CALL of it only where every body of that name only reads. A method called on a class may be a subclass's: there
 *  the subclasses' bodies are asked too. Where the name is declared as plain data, or as the one getter or method it is, that declaration alone is asked. */
const namedThings = (() => { let made: { getters: Map<string, any[]>; methods: Map<string, any[]>; subs: Map<any, any[]> } | null = null; return () => { if (made) return made; made = { getters: new Map(), methods: new Map(), subs: new Map() };
    const put = (m: Map<string, any[]>, k: string, d: any) => { const l = m.get(k); if (l) l.push(d); else m.set(k, [d]); };
    for (const f of files) { const sf = sfOf(f); if (!sf || sf.isDeclarationFile || (!textOf(f).includes('get ') && !textOf(f).includes('('))) continue;
      const visit = (n: any) => { const nm = n.name && (n.name.kind === SK.Identifier || n.name.kind === SK.StringLiteral) ? n.name.text : null;
        if (n.kind === SK.ClassDeclaration || n.kind === SK.ClassExpression) for (const h of n.heritageClauses ?? []) if (h.token === SK.ExtendsKeyword && h.types[0]) { let b = checker.getSymbolAtLocation(h.types[0].expression); if (b && (b.flags & ts.SymbolFlags.Alias)) b = checker.getAliasedSymbol(b);
            const bd = b?.valueDeclaration; if (bd) { const l = made!.subs.get(bd); if (l) l.push(n); else made!.subs.set(bd, [n]); } }
        if (nm && n.kind === SK.GetAccessor) put(made!.getters, nm, n);
        else if (nm && n.kind === SK.MethodDeclaration && n.body) put(made!.methods, nm, n);
        else if (nm && (n.kind === SK.PropertyAssignment || n.kind === SK.PropertyDeclaration) && n.initializer && ts.isFunctionLike(unparen(n.initializer))) put(made!.methods, nm, unparen(n.initializer));
        n.forEachChild(visit); }; visit(sf); }
    return made; }; })();
/** does this function WRITE something that is not its own — an assignment, a ++, a delete on anything but its locals — itself, or in a function it calls
 *  that can be found by its name (two calls deep)? What a READ of a property is asked: a getter that only works something out is harmless to leave out; one
 *  that counts, draws or keeps something is not. (A call is asked more: pureFn must see that it only reads.) */
const writeMemo = new Map<any, boolean>();
/** does running this getter (or what it calls, three calls deep) change anything but its own locals? What it cannot see into counts as a change: a call
 *  deeper than that, a function a `let` or a parameter holds, a thing of the game's made anew, a method known only by an interface. Of the language's own:
 *  the reading methods, `Math`, a few plain converters, a new Map or Set, and a method that changes one of the getter's OWN locals change nothing outside. */
const PLAIN_GLOBALS = new Set(['Number', 'String', 'Boolean', 'parseInt', 'parseFloat', 'isNaN', 'isFinite', 'BigInt', 'Array', 'Object', 'structuredClone', 'encodeURIComponent', 'decodeURIComponent']);
function writesFn(f: any, depth = 3): boolean { f = unparen(f); if (!f || !f.body || !ts.isFunctionLike(f)) return false; const key = depth === 3 ? f : null; if (key && writeMemo.has(key)) return writeMemo.get(key)!;
  const own = (x: any): boolean => { x = unparen(x); if (x?.kind !== SK.Identifier) return false; const d = checker.getSymbolAtLocation(x)?.valueDeclaration; return !!d && d.getStart() >= f.getStart() && d.end <= f.end; };
  const symOf = (nm: any): any => { let sym = nm ? checker.getSymbolAtLocation(nm) : null; if (sym && (sym.flags & ts.SymbolFlags.Alias)) sym = checker.getAliasedSymbol(sym); return sym; };
  let hit = false; const visit = (n: any): void => { if (hit) return;
    if (n !== f && ts.isFunctionLike(n)) return;
    if (n.kind === SK.BinaryExpression && n.operatorToken.kind >= SK.FirstAssignment && n.operatorToken.kind <= SK.LastAssignment && !own(n.left)) { hit = true; return; }
    if ((n.kind === SK.PrefixUnaryExpression || n.kind === SK.PostfixUnaryExpression) && (n.operator === SK.PlusPlusToken || n.operator === SK.MinusMinusToken) && !own(n.operand)) { hit = true; return; }
    if (n.kind === SK.DeleteExpression || n.kind === SK.TaggedTemplateExpression || n.kind === SK.AwaitExpression || n.kind === SK.YieldExpression) { hit = true; return; }
    if (n.kind === SK.CallExpression || n.kind === SK.NewExpression) { const e = unparen(n.expression), sym = symOf(e.kind === SK.Identifier ? e : e.kind === SK.PropertyAccessExpression ? e.name : null);
      const seen = (sym?.declarations ?? []).filter((d: any) => !d.getSourceFile().isDeclarationFile).map(boundTo).filter((g: any) => g && ts.isFunctionLike(g) && g.body);
      if (seen.length && n.kind === SK.CallExpression) { if (depth <= 0 || seen.some((g: any) => g !== f && writesFn(g, depth - 1))) { hit = true; return; } }
      else if (sym && libOwn(sym)) { const onMath = e.kind === SK.PropertyAccessExpression && unparen(e.expression).kind === SK.Identifier && unparen(e.expression).text === 'Math';
        const harmless = n.kind === SK.NewExpression ? (e.kind === SK.Identifier && (e.text === 'Map' || e.text === 'Set' || e.text === 'Array')) : e.kind === SK.Identifier ? PLAIN_GLOBALS.has(e.text) : e.kind === SK.PropertyAccessExpression && (onMath || PURE_METHODS.has(e.name.text) || own(e.expression));
        if (!harmless) { hit = true; return; } }
      else { hit = true; return; } }   // (it cannot be seen into)
    if (n.kind === SK.PropertyAccessExpression) { const gs = (symOf(n.name)?.declarations ?? []).filter((d: any) => d.kind === SK.GetAccessor && !d.getSourceFile().isDeclarationFile && d.body && d !== f);
      if (gs.length && (depth <= 0 || gs.some((g: any) => writesFn(g, depth - 1)))) { hit = true; return; } }
    n.forEachChild(visit); }; visit(f.body);
  if (key) writeMemo.set(key, hit); return hit; }
const pureMemo = new Map<any, boolean>();
const pureOpen: any[] = [];   // (the things being read now: an answer found while one is open may lean on it, and is kept only once it has closed harmless)
const pureThing = (d: any): boolean => { const had = pureMemo.get(d); if (had !== undefined) return had; if (pureOpen.includes(d)) return true;   // (`get on() { return T.on; }` reads a thing of its own name: taken for harmless while it is read)
  pureOpen.push(d); const v = pureFn(d); pureOpen.pop(); if (!pureOpen.length || !v) pureMemo.set(d, v); return v; };
/** a function whose own body only reads: an expression that does, or consts that only read and then a return that does (`const lk = (k) => locks.has(k)`) */
/** what a name is bound to for good: a `const`'s (or a readonly field's) own value, a declared function or method. A `let` may hold another function by now. */
const boundTo = (d: any): any => !d ? null : d.kind === SK.VariableDeclaration ? ((d.parent.flags & ts.NodeFlags.Const) ? unparen(d.initializer) : null)
  : d.kind === SK.PropertyDeclaration ? ((d.modifiers ?? []).some((m: any) => m.kind === SK.ReadonlyKeyword) ? unparen(d.initializer) : null)
  : d.kind === SK.FunctionDeclaration || d.kind === SK.MethodDeclaration || d.kind === SK.GetAccessor ? d : null;
/** a function that only reads: nothing in its body writes anything but its own locals, and every call in it only reads (nested functions it makes are not
 *  run by making them). `const lk = (k) => locks.has(k)`, `get live() { if (this.t > 0) return true; return !this.done; }` */
function pureFn(f: any): boolean { f = unparen(f); if (!f || pureDepth >= 12) return false;
  if (f.kind === SK.Identifier) { if (/^(Boolean|String|Number)$/.test(f.text)) return true; const g = boundTo(checker.getSymbolAtLocation(f)?.valueDeclaration); return !!g && g.kind !== SK.Identifier && pureFn(g); }
  if (!(f.kind === SK.ArrowFunction || f.kind === SK.FunctionExpression || f.kind === SK.FunctionDeclaration || f.kind === SK.MethodDeclaration || f.kind === SK.GetAccessor) || !f.body || f.getSourceFile().isDeclarationFile) return false;
  if (!(f.parameters ?? []).every((p: any) => !p.initializer || pure(p.initializer))) return false;   // (a default worked out at the call is part of the call)
  pureDepth++; try {
    if (f.body.kind !== SK.Block) return pure(f.body);
    const own = (x: any): boolean => { x = unparen(x); if (x?.kind !== SK.Identifier) return false; const d = checker.getSymbolAtLocation(x)?.valueDeclaration; return !!d && d.getStart() >= f.body.getStart() && d.end <= f.body.end; };   // (a local of this function)
    let ok = true; const visit = (n: any): void => { if (!ok) return;
      switch (n.kind) {
        case SK.BinaryExpression: if (n.operatorToken.kind >= SK.FirstAssignment && n.operatorToken.kind <= SK.LastAssignment) { if (!own(n.left)) { ok = false; return; } visit(n.right); return; } break;
        case SK.PrefixUnaryExpression: case SK.PostfixUnaryExpression: if (n.operator === SK.PlusPlusToken || n.operator === SK.MinusMinusToken) { if (!own(n.operand)) ok = false; return; } break;
        case SK.CallExpression: case SK.NewExpression: case SK.PropertyAccessExpression: case SK.ElementAccessExpression: case SK.AwaitExpression: case SK.YieldExpression: case SK.DeleteExpression: case SK.TaggedTemplateExpression: if (!pure(n)) ok = false; return;
        case SK.ThrowStatement: ok = false; return;
        case SK.ArrowFunction: case SK.FunctionExpression: case SK.FunctionDeclaration: case SK.ClassDeclaration: case SK.ClassExpression: return;   // (made, not run: a call of it is asked when it is called)
      }
      n.forEachChild(visit); };
    f.body.forEachChild(visit); return ok;
  } finally { pureDepth--; } }
/** a call that only reads: one of the reading methods above (and, where the method calls what it is handed, a function that only reads), or a local function
 *  whose own body only reads */
function pureCall(n: any): boolean { const c = n.expression;
  if (VOUCHED.size && (c.kind === SK.Identifier ? VOUCHED.has(c.text) : c.kind === SK.PropertyAccessExpression && VOUCHED.has(c.name.text))) return (c.kind === SK.Identifier || pure(c.expression)) && n.arguments.every(pure);
  // (a reading method by its NAME is one only where the name is the language's own: `store.get(k)` on a thing of the game's may count, draw or send)
  if (c.kind === SK.PropertyAccessExpression && PURE_METHODS.has(c.name.text) && libOwn(checker.getSymbolAtLocation(c.name))) { const calls = CALLBACK_METHODS.has(c.name.text);
    return pure(c.expression) && n.arguments.every((a: any, i: number) => { const u = unparen(a); return (u && ts.isFunctionLike(u)) || (calls && i === 0) ? pureFn(u) : pure(a); }); }
  if (c.kind === SK.Identifier) return pureFn(c) && n.arguments.every(pure);
  // a method: its own body only reads, and so does every other body of that name in the game (the one that runs may be an override, or another object's)
  if (c.kind === SK.PropertyAccessExpression) { if (!pure(c.expression) || !n.arguments.every(pure)) return false;
    const msym = checker.getSymbolAtLocation(c.name); if (msym && libOwn(msym)) return false;   // (the language's own, and not one of its reading methods above: `sort`, a Map's `delete`)
    const d = msym?.valueDeclaration, own = boundTo(d);
    if (!own) { const all = namedThings().methods.get(c.name.text) ?? []; return all.length > 0 && all.length <= 8 && all.every(pureThing); }   // (through an interface, or untyped: every body of that name, where they are few enough to read)
    if (!pureThing(own)) return false;
    // (a method of a class may be an override's at run time: the subclasses' bodies of the same name, all the way down)
    const below = (cls: any): boolean => (namedThings().subs.get(cls) ?? []).every((sub: any) => sub.members.every((m: any) => !(m.kind === SK.MethodDeclaration && m.name?.getText() === c.name.text) || pureThing(m)) && below(sub));
    return d.kind !== SK.MethodDeclaration || !(d.parent.kind === SK.ClassDeclaration || d.parent.kind === SK.ClassExpression) || below(d.parent); }
  return false; }
function pure(n: any): boolean {
  if (!n) return true;
  switch (n.kind) {
    case SK.CallExpression: { if (pureCall(n)) return true; const c = n.expression.getText(); return PURE_CALLS.test(c) && n.arguments.every(pure); }
    // (a property may be a getter of the game's own, which is a call — also where the name is typed as plain data: harmless only where every getter of that name only reads)
    case SK.PropertyAccessExpression: case SK.ElementAccessExpression: { const nm = n.kind === SK.PropertyAccessExpression ? n.name.text : n.argumentExpression?.kind === SK.StringLiteral ? n.argumentExpression.text : null;
      if (nm === null) break;   // (a key worked out as it runs: a list's index, a table's row)
      const sym = checker.getSymbolAtLocation(n.kind === SK.PropertyAccessExpression ? n.name : n.argumentExpression); if (sym && libOwn(sym)) break;
      const ds: any[] = (sym?.declarations ?? []).filter((d: any) => !d.getSourceFile().isDeclarationFile), gs = ds.filter(d => d.kind === SK.GetAccessor);
      if (gs.length) { const under = (cls: any): any[] => (namedThings().subs.get(cls) ?? []).flatMap((sub: any) => [...sub.members.filter((m: any) => m.kind === SK.GetAccessor && m.name?.getText() === nm), ...under(sub)]);   // (… or a subclass's getter of the same name: the one that runs may be that)
        if ([...gs, ...gs.flatMap(g => g.parent.kind === SK.ClassDeclaration || g.parent.kind === SK.ClassExpression ? under(g.parent) : [])].some(g => writesFn(g))) return false; break; }   // (a read of it is left out only where it writes nothing)
      if (ds.length && ds.every(d => d.kind !== SK.PropertySignature && d.kind !== SK.MethodSignature && d.kind !== SK.IndexSignature)) break;   // (declared as plain data, a field, a method: reading it does nothing)
      if ((namedThings().getters.get(nm) ?? []).some(g => !isTool(g.getSourceFile().fileName) && writesFn(g))) return false; break; }   // (through an interface, or untyped: any getter of that name in the game)
    case SK.NewExpression: return /^(URLSearchParams|Set|Map|WeakSet|WeakMap|Array)$/.test(n.expression.getText()) && (n.arguments ?? []).every(pure);   // (making an empty or a filled container does nothing else)
    case SK.AwaitExpression: case SK.YieldExpression: case SK.DeleteExpression: case SK.TaggedTemplateExpression: return false;
    case SK.PostfixUnaryExpression: return false;
    case SK.PrefixUnaryExpression: if (n.operator === SK.PlusPlusToken || n.operator === SK.MinusMinusToken) return false; break;
    case SK.BinaryExpression: if (n.operatorToken.kind >= SK.FirstAssignment && n.operatorToken.kind <= SK.LastAssignment) return false; break;
    case SK.ArrowFunction: case SK.FunctionExpression: case SK.MethodDeclaration: case SK.GetAccessor: case SK.SetAccessor: case SK.FunctionDeclaration: return true;   // (MAKING a function does nothing: calling it is pureCall's question)
  }
  let ok = true; n.forEachChild((c: any) => { if (ok && !pure(c)) ok = false; }); return ok;
}
/** a literal's own value: true, false, a number (a minus before it too), a word — or undefined for anything else */
const litValue = (e: any): Value | undefined => { e = unparen(e); if (!e) return undefined;
  return e.kind === SK.TrueKeyword ? true : e.kind === SK.FalseKeyword ? false : e.kind === SK.NumericLiteral ? Number(e.text) : e.kind === SK.StringLiteral || e.kind === SK.NoSubstitutionTemplateLiteral ? e.text
    : e.kind === SK.PrefixUnaryExpression && (e.operator === SK.MinusToken || e.operator === SK.PlusToken) && unparen(e.operand)?.kind === SK.NumericLiteral ? (e.operator === SK.MinusToken ? -1 : 1) * Number(unparen(e.operand).text) : undefined; };
const lit = (v: Value) => typeof v === 'string' ? (/['\\]/.test(v) ? JSON.stringify(v) : `'${JSON.stringify(v).slice(1, -1).replace(/\\"/g, '"')}'`) : String(v);
const isBoolType = (n: any) => { try { const t = checker.getTypeAtLocation(n); return !!(t.flags & (ts.TypeFlags.Boolean | ts.TypeFlags.BooleanLiteral)) || (t.isUnion?.() && t.types.every((u: any) => u.flags & ts.TypeFlags.BooleanLiteral)); } catch { return false; } };
/** is this expression's value only ever read as true or false (a condition, a `!`, an operand of a chain that is)? */
function boolCtx(n: any): boolean {
  n = upParen(n); const p = n.parent; if (!p) return false;
  if ((p.kind === SK.IfStatement || p.kind === SK.WhileStatement || p.kind === SK.DoStatement) && p.expression === n) return true;
  if (p.kind === SK.ForStatement && p.condition === n) return true;
  if (p.kind === SK.ConditionalExpression && p.condition === n) return true;
  if (p.kind === SK.PrefixUnaryExpression && p.operator === SK.ExclamationToken) return true;
  if (p.kind === SK.BinaryExpression && (p.operatorToken.kind === SK.AmpersandAmpersandToken || p.operatorToken.kind === SK.BarBarToken)) return boolCtx(p);
  return false;
}

// ---- 1. find each row's declaration and every use of it
type Use = { row: Row; node: any; sf: any; write: boolean; def: boolean; named?: boolean };   // named: found by the table's NAME on an untyped thing (a tool's import)
/** the knob registry's own imports say which module a table lives in (two modules may export one name: src/game/knobs.ts is the list a row's name is read against) */
const REGISTRY: Map<string, string> = (() => { const m = new Map<string, string>(), f = join(ROOT, 'src/game/knobs.ts'); if (!existsSync(f)) return m;
  for (const im of textOf(f).matchAll(/^import\s*\{([^}]*)\}\s*from\s*'([^']+)'/gm)) for (const n of im[1].split(',')) { const name = n.trim().split(/\s+as\s+/)[0]; if (name) m.set(name, resolve(join(ROOT, 'src/game'), im[2]) + '.ts'); }
  return m; })();
function tableDecl(name: string): any[] {
  const found: any[] = [], home = REGISTRY.get(name);
  for (const f of home ? [home] : files) { if (isTool(f)) continue; const sf = sfOf(f); if (!sf || !textOf(f).includes(name)) continue;
    for (const st of sf.statements) if (st.kind === SK.VariableStatement && st.modifiers?.some((m: any) => m.kind === SK.ExportKeyword))
      for (const d of st.declarationList.declarations) if (d.name.kind === SK.Identifier && d.name.text === name && d.initializer) found.push(d); }
  return found;
}
function propIn(obj: any, key: string): any { obj = unparen(obj); if (!obj || obj.kind !== SK.ObjectLiteralExpression) return null;
  return obj.properties.find((p: any) => (p.kind === SK.PropertyAssignment || p.kind === SK.ShorthandPropertyAssignment) && p.name && (p.name.text === key)) ?? null; }
const uses: Use[] = []; const rowDef = new Map<string, any>(); const literal = new Map<string, string>();
const moduleVal = new Map<string, Value>(), tableOfRow = new Map<string, any>();   // a row's own value as its module loads · its table's declaration
for (const r of rows) {
  const parts = r.row.split('.');
  if (parts[0] === 'tune') { report.refused.push({ row: r.row, why: 'a row of Game.tune: not a knob table (retire it by hand)' }); continue; }
  const decls = tableDecl(parts[0]);
  if (decls.length !== 1) { report.refused.push({ row: r.row, why: decls.length ? `${decls.length} exported tables named ${parts[0]}` : `no exported table named ${parts[0]} under src` }); continue; }
  if (checker.getTypeAtLocation(decls[0].name).getStringIndexType?.()) { report.refused.push({ row: r.row, why: `${parts[0]} is typed by an index (a Record of rows by name): a read of one row cannot be found by its name, so nothing would be folded (${at(decls[0].getSourceFile(), decls[0].getStart())})` }); continue; }
  let prop: any = null, obj = decls[0].initializer;
  for (const k of parts.slice(1)) { prop = propIn(obj, k); if (!prop) break; obj = prop.initializer; }
  if (!prop) { const o = unparen(decls[0].initializer), acc = o?.kind === SK.ObjectLiteralExpression && o.properties.some((p: any) => (p.kind === SK.GetAccessor || p.kind === SK.SetAccessor) && p.name?.text === parts[1]);
    report.refused.push({ row: r.row, why: acc ? `${parts[0]}.${parts[1]} is a getter and a setter: writing it does something, so it is not a plain switch (${at(decls[0].getSourceFile(), decls[0].getStart())})` : `no key ${parts.slice(1).join('.')} in ${parts[0]}'s literal (${at(decls[0].getSourceFile(), decls[0].getStart())})` }); continue; }
  literal.set(r.row, unparen(prop.initializer)?.getText?.() ?? prop.getText());
  { const L = unparen(prop.initializer), lv = litValue(prop.initializer), kind = !L ? 'none' : lv === undefined ? 'other' : typeof lv;
    if (kind !== typeof r.value) { report.refused.push({ row: r.row, why: kind === 'other' || kind === 'none' ? `its own value is not a plain true, false, number or word (${literal.get(r.row)!.slice(0, 60)}): retire it by hand` : `asked for ${JSON.stringify(r.value)}, and the row is a ${kind} (${literal.get(r.row)})` }); continue; } }
  rowDef.set(r.row, prop); tableOfRow.set(r.row, decls[0]);
  moduleVal.set(r.row, litValue(prop.initializer)!);
  const sf = prop.getSourceFile(), refs = service.findReferences(sf.fileName, prop.name.getStart(sf)) ?? [];
  const seen = new Set<string>();
  for (const g of refs) for (const e of g.references) { const k = `${e.fileName}:${e.textSpan.start}`; if (seen.has(k)) continue; seen.add(k);
    const rsf = sfOf(e.fileName); if (!rsf) continue; const node = ts.getTokenAtPosition(rsf, e.textSpan.start);
    uses.push({ row: r, node, sf: rsf, write: !!e.isWriteAccess, def: node === prop.name }); }
}

// (a tool takes the game's tables through `await import(\`${ROOT}/src/…\`)`, which TypeScript reads as `any`: there the row is found by its name — TABLE.key on a
// thing called TABLE, or on `<module>.TABLE` — and only where that thing has no type of its own)
const untypedInGame: { row: string; where: string; text: string }[] = [];
{ const byTable = new Map<string, Row[]>(); for (const r of rows) if (rowDef.has(r.row)) { const t = r.row.split('.')[0]; byTable.set(t, [...(byTable.get(t) ?? []), r]); }
  const seenTok = new Set<any>(uses.map(u => u.node));
  for (const f of files) { if (rel(f) === 'src/game/journal.ts') continue;   // (the journal reads a recording's own words under the tables' names: its lines have their own rule, below)
    const game = !isTool(f);   // (in the game an untyped thing with a key called TABLE may be a recording's or a save's own words, or the table itself reached without a type: the tool cannot tell which, so the row is refused)
    const text = textOf(f); let any = false; for (const t of byTable.keys()) if (text.includes(t)) { any = true; break; } if (!any) continue; const sf = sfOf(f); if (!sf) continue;
    // (another name for the table in this file — `const KK = K`, `const KK = M.K`, `const { K: KK } = …` — is the table)
    const alias = new Map<string, string>(); { const names = (n: any) => { if (n.kind === SK.VariableDeclaration && n.name.kind === SK.Identifier && n.initializer) { const i = unparen(n.initializer), t = i?.kind === SK.Identifier ? i.text : i?.kind === SK.PropertyAccessExpression ? i.name.text : null; if (t && byTable.has(t) && n.name.text !== t) alias.set(n.name.text, t); }
        if (n.kind === SK.BindingElement && n.propertyName?.kind === SK.Identifier && n.name.kind === SK.Identifier && byTable.has(n.propertyName.text) && n.name.text !== n.propertyName.text) alias.set(n.name.text, n.propertyName.text);
        n.forEachChild(names); }; names(sf); }
    const visit = (n: any) => { if (n.kind === SK.PropertyAccessExpression && n.name.kind === SK.Identifier) { const chain: string[] = []; let e = n;
          while (e.kind === SK.PropertyAccessExpression) { chain.unshift(e.name.text); e = e.expression; if (byTable.has(chain[0]) && chain.length > 1) break; }
          const root = e.kind === SK.Identifier ? alias.get(e.text) ?? e.text : null, table = root && byTable.has(root) ? root : byTable.has(chain[0]) && chain.length > 1 ? chain[0] : null;
          if (table) { const keys = table === root ? chain : chain.slice(1), path = `${table}.${keys.join('.')}`, r = byTable.get(table)!.find(x => x.row === path);
            if (r && !seenTok.has(n.name)) { let base = n; for (let i = 0; i < keys.length; i++) base = base.expression; const ty = checker.getTypeAtLocation(base);
              if ((ty.flags & ts.TypeFlags.Any) && game) { seenTok.add(n.name); untypedInGame.push({ row: r.row, where: at(sf, n.getStart(sf)), text: (n.getText() as string).slice(0, 80).replace(/\s+/g, ' ') }); }
              else if (ty.flags & ts.TypeFlags.Any) { seenTok.add(n.name); const up = upParen(n).parent; uses.push({ row: r, node: n.name, sf, write: up.kind === SK.BinaryExpression && upParen(up.left) === upParen(n) && up.operatorToken.kind === SK.EqualsToken, def: false, named: true }); } } } }
      n.forEachChild(visit); }; visit(sf); } }

{ const out = arg('--uses'); if (out) {   // only the reading: where each row is used (for the census of switches)
    const by: Record<string, { files: Record<string, number>; writes: number; reads: number; literal?: string; fns: string[] }> = {};
    for (const u of uses) { const b = by[u.row.row] ??= { files: {}, writes: 0, reads: 0, literal: literal.get(u.row.row), fns: [] }; const f = rel(u.sf.fileName); b.files[f] = (b.files[f] ?? 0) + 1; if (u.write) b.writes++; else b.reads++;
      if (/src\/game\/(character|guards)\.ts$/.test(f)) { const fn = ts.findAncestor(u.node, (x: any) => (x.kind === SK.MethodDeclaration || x.kind === SK.FunctionDeclaration) && x.name); const nm = fn ? fn.name.getText() : '(top)'; if (!b.fns.includes(nm)) b.fns.push(nm); } }
    writeFileSync(out, JSON.stringify({ uses: by, refused: report.refused }, null, 1)); console.log(`uses of ${Object.keys(by).length} rows → ${out}`); process.exit(0); } }

// ---- the rows that are half of a Rust twin are not this tool's (docs/agents/rust.md; the list is the Rust goal's, 2026-11-04): a row read in a file whose
// functions the module also holds, or named by rust/sim/src/knobs.rs, is retired in both languages on a train of that goal
const TWIN_FILES = new Set(['src/scene/collision.ts', 'src/scene/boxMirror.ts', 'src/game/fogofwar.ts', 'src/anim/skeleton.ts', 'src/math/dice.ts', 'src/math/exact.ts', 'src/game/sense.ts', 'src/game/doors.ts', 'src/game/doorblast.ts', 'src/game/shards.ts', 'src/sim/fallshome.ts', 'src/game/props.ts', 'src/game/throwables.ts']);
const twinFn = (n: any) => { const f = rel(n.getSourceFile().fileName); if (!/^src\/game\/(character|guards)\.ts$/.test(f)) return false; const fn = ts.findAncestor(n, (x: any) => (x.kind === SK.MethodDeclaration || x.kind === SK.FunctionDeclaration) && x.name); const nm = fn?.name?.getText?.() ?? ''; return /Own$/.test(nm) || nm === 'eyeContactHere' || nm === 'lampsHere'; };
const RUST_KNOBS = existsSync(join(ROOT, 'rust/sim/src/knobs.rs')) ? readFileSync(join(ROOT, 'rust/sim/src/knobs.rs'), 'utf8') : '';
const twinWhy = new Map<string, string>();
if (!flag('--twins')) { for (const r of rows) if (RUST_KNOBS.includes(`"${r.row}"`)) twinWhy.set(r.row, 'named by rust/sim/src/knobs.rs');
  for (const u of uses) { if (u.def || twinWhy.has(u.row.row)) continue; const f = rel(u.sf.fileName); if (TWIN_FILES.has(f) || twinFn(u.node)) twinWhy.set(u.row.row, `${at(u.sf, u.node.getStart(u.sf))}: read in code that has a Rust twin`); } }

// ---- 2. sort the uses: a read to fold, a write to take out, the row's own line — or a reason to refuse the row
const refuse = new Map<string, string>(); const droppedLines: string[] = [], dropMissed: string[] = [];
for (const x of untypedInGame) refuse.set(x.row, `${x.where}: read on an untyped thing under the table's name (${x.text}): that may be a recording's own words or the table itself reached without a type, and the tool cannot tell which. Fold it by hand first`);
/** the journal's reads of a recording's own word for a row (see the check before the fold) */
const recordingReads: { row: string; access: any; where: string }[] = [];
/** a row refused for a read at load: every such read is said, not the last one found */
const loadRefused = new Set<string>();
const refuseLoad = (row: string, msg: string) => { if (loadRefused.has(row)) { const had = refuse.get(row)!; if (had.length < 1200) refuse.set(row, `${had} · ALSO ${msg}`); } else if (!refuse.has(row)) { refuse.set(row, msg); loadRefused.add(row); } };
type Act = { use: Use; kind: 'read' | 'readLoad' | 'delStmt' | 'delProp' | 'delMember' | 'unassign' | 'const' | 'setInit' | 'hole'; node: any };
/** is this row read anywhere inside the node? */
const readIn = (n: any, row: string) => uses.some(x => x.row.row === row && !x.write && x.node.getSourceFile() === n.getSourceFile() && x.node.getStart() >= n.getStart() && x.node.end <= n.end);
const acts: Act[] = [];
const stmtOf = (n: any): any => { while (n && !(n.kind >= SK.FirstStatement && n.kind <= SK.LastStatement) && n.kind !== SK.Block) n = n.parent; return n && n.kind !== SK.Block ? n : null; };
/** a write is the page's or the stand's own if what it writes, or a condition it sits under (up to its function), reads the address or the environment */
function bootWrite(assign: any): boolean {
  const f = rel(assign.getSourceFile().fileName);
  if (/^src\/host\/(profile|standup)\.ts$/.test(f) || f === 'src/game/journal.ts') return true;
  const boots = (t: string) => /\b(q|Q|qs)\.(get|has)\(|URLSearchParams|process\??\.env|pinned\(|envOf\(\)|localStorage|\bkept\b/.test(t);
  if (boots(assign.right.getText())) return true;
  for (let n = assign.parent; n && !ts.isFunctionLike(n) && n.kind !== SK.SourceFile; n = n.parent) if (n.kind === SK.IfStatement && boots(n.expression.getText())) return true;
  return false;
}
/** the keys from the table down to what this expression is — [] for the table itself (its own name, an import of it, another `const` name for it, whatever
 *  it is cast to), ['pall'] for `TABLE.pall` or a `const` that holds it — or null where it is not the table or a part of it */
function livePath(x: any, decl: any, depth = 0): string[] | null { x = unparen(x); if (!x || depth > 4) return null;
  if (x.kind === SK.Identifier || x.kind === SK.PropertyAccessExpression) { const nm = x.kind === SK.Identifier ? x : x.name; let sym = checker.getSymbolAtLocation(nm); if (sym && (sym.flags & ts.SymbolFlags.Alias)) sym = checker.getAliasedSymbol(sym);
    const d = sym?.valueDeclaration; if (d === decl) return [];
    if (x.kind === SK.Identifier) { if (d?.kind === SK.VariableDeclaration && (d.parent.flags & ts.NodeFlags.Const) && d.initializer) return livePath(d.initializer, decl, depth + 1);
      // (in a tool: a thing of the table's own name and the table's own type, however it was brought in — `const { TABLE } = await import(…) as typeof import(…)`)
      const called = d?.kind === SK.BindingElement && d.propertyName?.kind === SK.Identifier ? d.propertyName.text : x.text;   // (`const { TABLE: K } = …`: K is what the module calls TABLE)
      return d && isTool(x.getSourceFile().fileName) && called === decl.name.text && checker.getTypeAtLocation(x) === checker.getTypeAtLocation(decl.name) ? [] : null; }
    const up = livePath(x.expression, decl, depth); return up ? [...up, x.name.text] : null; }
  if (x.kind === SK.ElementAccessExpression && x.argumentExpression?.kind === SK.StringLiteral) { const up = livePath(x.expression, decl, depth); return up ? [...up, x.argumentExpression.text] : null; }
  return null; }
const isLive = (x: any, decl: any): boolean => livePath(x, decl)?.length === 0;
/** is this a copy of the table taken as its module loads (`X_DEFAULTS = JSON.parse(JSON.stringify(X))`, `{ ...X }`)? 'deep' = every level is a copy */
function snapshotOf(x: any, decl: any): 'deep' | 'shallow' | null { x = unparen(x); if (x?.kind !== SK.Identifier) return null;
  let sym = checker.getSymbolAtLocation(x); if (sym && (sym.flags & ts.SymbolFlags.Alias)) sym = checker.getAliasedSymbol(sym);
  const d = sym?.valueDeclaration; if (!d || d.kind !== SK.VariableDeclaration || !(d.parent.flags & ts.NodeFlags.Const) || d.parent.parent?.parent?.kind !== SK.SourceFile || !d.initializer) return null;
  let i = unparen(d.initializer); if (i.kind === SK.CallExpression && i.expression.getText() === 'Object.freeze' && i.arguments.length === 1) i = unparen(i.arguments[0]);
  if (i.kind === SK.CallExpression) { const c = i.expression.getText(), a = i.arguments.length === 1 ? unparen(i.arguments[0]) : null;
    if (c === 'structuredClone' && a && isLive(a, decl)) return 'deep';
    if (c === 'JSON.parse' && a?.kind === SK.CallExpression && a.expression.getText() === 'JSON.stringify' && a.arguments.length === 1 && isLive(a.arguments[0], decl)) return 'deep';
    return null; }
  return i.kind === SK.ObjectLiteralExpression && i.properties.length === 1 && i.properties[0].kind === SK.SpreadAssignment && isLive(i.properties[0].expression, decl) ? 'shallow' : null; }
/** A FUNCTION WRITTEN WHERE IT IS HANDED OVER: who runs it, and when? 'now' = called on the spot (`(() => …)()`), or handed to one of the language's own
 *  that calls what it is handed before it returns (`LIST.map(k => …)`, `new Promise(res => …)`); 'later' = handed to something of the language's or the
 *  browser's that keeps it (a listener, a timer, a `.then`); 'callee' = handed to a function of the game's own, which may run it at once or keep it;
 *  null = not handed to anything (the value of a name, a field, a key). */
const SYNC_CALLBACK = new Set([...CALLBACK_METHODS, 'forEach', 'reduce', 'reduceRight', 'flatMap', 'sort', 'toSorted', 'findLast', 'findLastIndex', 'from', 'replace', 'replaceAll']);
const calleeSym = (q: any): any => { const e = unparen(q.expression), nm = e.kind === SK.PropertyAccessExpression ? e.name : e.kind === SK.Identifier ? e : null; return nm ? checker.getSymbolAtLocation(nm) : null; };
const calleeName = (q: any): string => { const e = unparen(q.expression); return (q.kind === SK.NewExpression ? 'new ' : '') + (e.kind === SK.PropertyAccessExpression ? e.name.text : e.kind === SK.Identifier ? e.text : '(…)'); };
const runsHanded = (q: any): boolean => { const e = unparen(q.expression); return q.kind === SK.NewExpression ? e.kind === SK.Identifier && e.text === 'Promise' : e.kind === SK.PropertyAccessExpression && SYNC_CALLBACK.has(e.name.text); };
const handedTo = (fn: any): 'now' | 'later' | 'callee' | null => { if (fn.kind !== SK.ArrowFunction && fn.kind !== SK.FunctionExpression) return null; const f = upParen(fn), q = f.parent;
  if (!q || (q.kind !== SK.CallExpression && q.kind !== SK.NewExpression)) return null; if (q.expression === f) return 'now'; if (!(q.arguments ?? []).includes(f)) return null;
  const sym = calleeSym(q); if (sym && libOwn(sym)) return runsHanded(q) ? 'now' : 'later';
  return 'callee'; };
/** what a thing waits for before it runs: null with no function round it (it runs as its module loads); else the function that must be CALLED first, or the
 *  class whose object must be made (a field of its objects). A function run where it is written ('now', above) puts nothing off. */
const keeper = (n: any): any => { for (let p = n.parent; p; p = p.parent) {
    if (p.kind === SK.PropertyDeclaration && !(p.modifiers ?? []).some((m: any) => m.kind === SK.StaticKeyword)) return p.parent;
    if (!ts.isFunctionLike(p)) continue;
    if (handedTo(p) === 'now') { p = upParen(p); continue; }   // (go on up from the call that runs it)
    return p; }
  return null; };
/** THE FUNCTIONS THAT RUN AS A MODULE LOADS: everything a module's own top-level statements call, make (`new C`: its fields and constructor, and its base's),
 *  read through a getter, or hand to a function of the game's own (which may run it at once) — and what those do in turn, to the end. Each is followed by
 *  the declaration its name leads to; a call through an interface or a value handed about is not followed, and what is handed to a listener, a timer or a
 *  promise waits. The walk follows every call a function COULD make, not the ones it does make: a refusal names the chain, and `--late` takes the caller's
 *  word against it. The file a page's script tag names is that page's program — it writes the switches and boots the game after — and is not walked. */
let loadFnsMemo: Map<any, string> | null = null;
function loadFns(): Map<any, string> { if (loadFnsMemo) return loadFnsMemo; const out = loadFnsMemo = new Map<any, string>(), queue: any[] = [];
  const pages = new Set<string>(); for (const h of readdirSync(ROOT)) if (h.endsWith('.html')) for (const m of readFileSync(join(ROOT, h), 'utf8').matchAll(/<script[^>]*\ssrc=["'](?:\.?\/)?([^"']+\.ts)["']/g)) pages.add(m[1]);
  const fnOf = (x: any): any => { x = unparen(x); return x && (x.kind === SK.ArrowFunction || x.kind === SK.FunctionExpression) ? x : null; };
  const bodies = (d: any, made: boolean, deep = 0): any[] => { if (!d || deep > 3 || d.getSourceFile().isDeclarationFile) return [];
    if (d.kind === SK.ClassDeclaration || d.kind === SK.ClassExpression) { if (!made) return []; const outB: any[] = [d, ...d.members.filter((m: any) => m.kind === SK.Constructor && m.body)];
      for (const h of d.heritageClauses ?? []) if (h.token === SK.ExtendsKeyword) for (const t of h.types) outB.push(...named(t.expression, true, deep + 1));
      return outB; }
    if ((d.kind === SK.FunctionDeclaration || d.kind === SK.MethodDeclaration || d.kind === SK.GetAccessor) && d.body) return [d];
    if (d.kind === SK.ShorthandPropertyAssignment) { const v = checker.getShorthandAssignmentValueSymbol(d); return (v?.declarations ?? []).flatMap((x: any) => bodies(x, made, deep + 1)); }   // (`{ init }`: the function of that name)
    if ((d.kind === SK.VariableDeclaration || d.kind === SK.PropertyDeclaration || d.kind === SK.PropertyAssignment) && d.initializer) { const i = unparen(d.initializer); if (fnOf(i)) return [i]; if (i.kind === SK.ClassExpression) return bodies(i, made, deep + 1);
      if (i.kind === SK.Identifier || i.kind === SK.PropertyAccessExpression) return named(i, made, deep + 1); }   // (`{ init: start }`, `const go = start`: the function under its other name)
    return []; };
  const named = (x: any, made: boolean, deep = 0): any[] => { x = unparen(x); const at0 = x.kind === SK.PropertyAccessExpression ? x.name : x; if (at0.kind !== SK.Identifier && at0.kind !== SK.PrivateIdentifier) return [];
    let sym = checker.getSymbolAtLocation(at0); if (sym && (sym.flags & ts.SymbolFlags.Alias)) sym = checker.getAliasedSymbol(sym); return (sym?.declarations ?? []).flatMap((d: any) => bodies(d, made, deep)); };
  const label = (fn: any): string => fn.kind === SK.Constructor ? `new ${fn.parent.name?.text ?? '(a class)'}` : fn.kind === SK.ClassDeclaration || fn.kind === SK.ClassExpression ? `the fields of ${fn.name?.text ?? 'a class'}` : fn.kind === SK.GetAccessor ? `get ${fn.name.getText()}` : fn.name?.getText?.() ?? (fn.parent?.name?.getText?.() as string | undefined) ?? 'a function';
  const mark = (fn: any, why: string) => { if (out.has(fn)) return; out.set(fn, why); queue.push(fn); };
  const scan = (root: any, whyOf: (n: any, name: string) => string) => {
    const go = (n: any): void => {
      if (n !== root && ts.isFunctionLike(n) && handedTo(n) !== 'now') return;   // (kept: it waits for its call — or is marked where it is handed over, below)
      if (n !== root && (n.kind === SK.ClassDeclaration || n.kind === SK.ClassExpression)) {   // (a class written here: what is static runs now, the rest when an object is made)
        for (const h of n.heritageClauses ?? []) go(h); for (const m of n.members) { if (m.kind === SK.PropertyDeclaration && (m.modifiers ?? []).some((x: any) => x.kind === SK.StaticKeyword) && m.initializer) go(m.initializer); else if (m.kind === SK.ClassStaticBlockDeclaration) go(m.body); } return; }
      if (n.kind === SK.CallExpression || n.kind === SK.NewExpression) {
        for (const fn of named(n.expression, n.kind === SK.NewExpression)) mark(fn, whyOf(n, label(fn)));
        const sym = calleeSym(n), lib = !!sym && libOwn(sym), runs = !lib || runsHanded(n);   // (a function of the game's own may run what it is handed at once; of the language's, only the ones that say so)
        if (runs) for (const a of n.arguments ?? []) { const ua = unparen(a);
          if (ua.kind === SK.Identifier || ua.kind === SK.PropertyAccessExpression) for (const fn of named(ua, false)) mark(fn, whyOf(n, `${label(fn)} (handed to ${calleeName(n)})`));
          else if (fnOf(ua) && !lib) mark(ua, whyOf(n, `a function handed to ${calleeName(n)}`)); } }
      else if (n.kind === SK.PropertyAccessExpression && n.name.kind === SK.Identifier) {   // (a getter read is a call of its body)
        let sym = checker.getSymbolAtLocation(n.name); if (sym && (sym.flags & ts.SymbolFlags.Alias)) sym = checker.getAliasedSymbol(sym);
        for (const d of sym?.declarations ?? []) if (d.kind === SK.GetAccessor && d.body && !d.getSourceFile().isDeclarationFile) mark(d, whyOf(n, label(d))); }
      n.forEachChild(go); };
    if (root.kind === SK.ClassDeclaration || root.kind === SK.ClassExpression) { for (const m of root.members) if (m.kind === SK.PropertyDeclaration && !(m.modifiers ?? []).some((x: any) => x.kind === SK.StaticKeyword) && m.initializer) go(m.initializer); }
    else root.forEachChild(go); };
  for (const f of files) { if (isTool(f) || pages.has(rel(f))) continue; const sf = sfOf(f); if (!sf) continue; scan(sf, (n, name) => `${at(sf, n.getStart(sf))} ${name}`); }
  const step = (why: string, name: string): string => why.endsWith('…') ? why : why.split(' → ').length >= 8 ? `${why} → …` : `${why} → ${name}`;   // (the chain of calls from the top-level statement, by name, for whoever reads the refusal; a long one says it was cut)
  while (queue.length) { const fn = queue.shift(), why = out.get(fn)!; scan(fn, (_n, name) => step(why, name)); }
  return out; }
/** does this run as its module loads? '' = it stands at a file's own top level; a chain = it stands in a function the walk above reached; null = no. */
const atLoadWhy = (n: any): string | null => { const k = keeper(n); if (!k) return ''; const w = loadFns().get(k); return w === undefined ? null : w; };
const atLoad = (n: any): boolean => atLoadWhy(n) !== null;
const viaCall = (n: any): string => { const w = atLoadWhy(n); return w ? ` (in what the module runs as it loads: ${w})` : ''; };
/** `--late ROW`: the caller's word that a read the walk reached does not happen before the page writes the row. Never for a read at a file's own top level. */
const vouchedLate = (row: string, n: any): boolean => LATE.has(row) && !!atLoadWhy(n);
/** what a write gives: in `A = B = x` both are given x (B is written there, not read) */
const given = (as: any): any => { let r = as.right; for (let x = unparen(r); x?.kind === SK.BinaryExpression && x.operatorToken.kind === SK.EqualsToken; x = unparen(r)) r = x.right; return r; };
const pageWrites: { row: string; node: any; rhs: any }[] = [], loadWrite = new Set<string>(), loadRead = new Map<string, string>();
for (const u of uses) {
  const n = u.node, p = n.parent, where = at(u.sf, n.getStart(u.sf)), tool = isTool(u.sf.fileName);
  if (u.row.hold) {   // a held number: its literal becomes the page's value, the page's and the stand's own writes of it go, every read and every tool is left alone
    if (u.def) { if (p.initializer) acts.push({ use: u, kind: 'setInit', node: p.initializer }); continue; }
    const acc = (p.kind === SK.PropertyAccessExpression && p.name === n) || (p.kind === SK.ElementAccessExpression && p.argumentExpression === n) ? upParen(p) : null, as = acc?.parent;
    const written = !!as && as.kind === SK.BinaryExpression && as.left === acc && as.operatorToken.kind >= SK.FirstAssignment && as.operatorToken.kind <= SK.LastAssignment;
    // (… but a read made as the module loads, or off a copy taken then, read the OLD value and would now read the page's)
    if (!tool && acc && moduleVal.get(u.row.row) !== u.row.value && !written) { const decl = tableOfRow.get(u.row.row), mine = livePath(p, decl)?.join('.') === u.row.row.split('.').slice(1).join('.');
      const shownH = ((stmtOf(p) ?? p).getText() as string).slice(0, 80).replace(/\s+/g, ' ');
      if (mine && atLoad(p)) { if (vouchedLate(u.row.row, p)) note(`--late ${u.row.row}: taken on your word that this is not read before the page writes it: ${where}${viaCall(p)} (${shownH})`);
        else { refuseLoad(u.row.row, `${where}: read as its module loads${viaCall(p)}, where it is still ${literal.get(u.row.row)}: making the table's own value the page's would change what that reads (${shownH})`); continue; } }
      else if (!mine && rel(u.sf.fileName) !== 'src/game/journal.ts') { refuse.set(u.row.row, `${where}: read on another thing of the table's type (a copy taken at load?), where it is still ${literal.get(u.row.row)}: making the table's own value the page's would change what that reads (${shownH})`); continue; } }
    if (as && as.kind === SK.BinaryExpression && as.left === acc && as.operatorToken.kind === SK.EqualsToken && /^(src\/host\/(profile|standup)\.ts|tools\/qa\/headless\.ts)$/.test(rel(u.sf.fileName))) { acts.push({ use: u, kind: 'unassign', node: as });
      if (rel(u.sf.fileName) === 'src/host/profile.ts') pageWrites.push({ row: u.row.row, node: as, rhs: given(as) }); }
    continue; }
  if (u.def) { acts.push({ use: u, kind: 'delProp', node: p }); continue; }
  if (p.kind === SK.PropertySignature && p.name === n) { acts.push({ use: u, kind: 'delMember', node: p }); continue; }
  if ((p.kind === SK.PropertyAssignment || p.kind === SK.ShorthandPropertyAssignment) && p.name === n) {   // the row written through an object (a preset, an Object.assign)
    const v = p.kind === SK.PropertyAssignment ? litValue(p.initializer) : undefined;
    if (!tool && v !== u.row.value) { refuse.set(u.row.row, `${where}: an object literal holds another value for it than the page's (${p.getText().slice(0, 60)}): wherever that object is put over the table, the game writes the row`); continue; }
    acts.push({ use: u, kind: 'delProp', node: p }); note(`${where}: ${u.row.row} set through an object literal: the key is taken out (${p.getText().slice(0, 60)})`); continue; }
  const access = (p.kind === SK.PropertyAccessExpression && p.name === n) || (p.kind === SK.ElementAccessExpression && p.argumentExpression === n) ? p : null;
  if (!access) { refuse.set(u.row.row, `${where}: a use this tool does not read (${SK[p.kind]}: ${p.getText().slice(0, 80)})`); continue; }
  const outer = upParen(access), op = outer.parent, isWrite = op.kind === SK.BinaryExpression && op.left === outer && op.operatorToken.kind >= SK.FirstAssignment && op.operatorToken.kind <= SK.LastAssignment;
  // `[ROW, other] = kept`: a write of the row through a pattern. In a tool its place in the pattern is left empty; in the game that is the game writing the row
  if (outer.parent.kind === SK.ArrayLiteralExpression) { const pat = upParen(outer.parent), as = pat.parent; if (as.kind === SK.BinaryExpression && as.left === pat && as.operatorToken.kind === SK.EqualsToken) {
      if (!tool) { refuse.set(u.row.row, `${where}: the game writes it as it runs: state, not a switch (${(as.getText() as string).slice(0, 80)})`); continue; }
      acts.push({ use: u, kind: 'hole', node: outer }); report.toolWrites.push({ row: u.row.row, at: where, kind: 'other', text: (as.getText() as string).slice(0, 100) }); continue; } }
  // the same key on a thing that is not the table (a copy taken at load, a recording's own words, a parameter of the table's type) is not the page's value
  { let base = access; for (let i = 1; i < u.row.row.split('.').length; i++) base = base?.expression; const decl = tableOfRow.get(u.row.row), file = rel(u.sf.fileName), shown = ((stmtOf(access) ?? access).getText() as string).slice(0, 90).replace(/\s+/g, ' ');
    if (!u.named && livePath(access, decl)?.join('.') !== u.row.row.split('.').slice(1).join('.')) {
      if (tool) { note(`${where}: ${u.row.row} read on another thing of the table's type in a tool (left as it is): ${shown}`); continue; }
      if (file === 'src/game/journal.ts') { if (!isWrite) recordingReads.push({ row: u.row.row, access, where }); continue; }   // (a recording's own word for the row: it goes with the journal's line for an old file, or the typecheck says where it stands)
      const snap = !isWrite && base ? snapshotOf(base, decl) : null;
      if (snap && (snap === 'deep' || u.row.row.split('.').length === 2)) { acts.push({ use: u, kind: 'readLoad', node: access }); loadRead.set(u.row.row, where); continue; }   // (a copy taken as the module loaded holds the row's own value, not the page's)
      refuse.set(u.row.row, `${where}: the same key on another thing of the table's type (${shown}): that is not the table, so its value there is not the page's. Fold it by hand first`); continue; }
    if (!tool && atLoad(access)) { if (isWrite) loadWrite.add(u.row.row); else if (moduleVal.get(u.row.row) !== u.row.value) {
        if (vouchedLate(u.row.row, access)) note(`--late ${u.row.row}: taken on your word that this is not read before the page writes it: ${where}${viaCall(access)} (${shown})`);
        else { refuseLoad(u.row.row, `${where}: read as its module loads${viaCall(access)}, before the page has written it (${shown}): there it is still ${literal.get(u.row.row)}. Fold it by hand first`); continue; } } } }
  if (op.kind === SK.DeleteExpression) {   // (`delete file.tables.T.key`: a check making a file from before the key — the key is gone, so is the line)
    if (op.parent.kind === SK.ExpressionStatement) acts.push({ use: u, kind: 'delStmt', node: op.parent }); else acts.push({ use: u, kind: 'const', node: op });
    report.toolWrites.push({ row: u.row.row, at: where, kind: 'other', text: (stmtOf(op) ?? op).getText().slice(0, 100) }); continue; }
  if (op.kind === SK.BinaryExpression && op.left === outer && op.operatorToken.kind >= SK.FirstAssignment && op.operatorToken.kind <= SK.LastAssignment) {
    if (op.operatorToken.kind !== SK.EqualsToken) { refuse.set(u.row.row, `${where}: a compound write (${op.getText().slice(0, 80)})`); continue; }
    const file = rel(u.sf.fileName), fn = ts.findAncestor(op, (x: any) => ts.isFunctionLike(x)), rhs = unparen(op.right), shown = (stmtOf(op) ?? op).getText().slice(0, 100);
    const side = tool;   // (a tool is not the game)
    // the debugger's own toggle for this row: a call that is a statement, handed a function that writes the row and another that reads it → the whole panel row goes
    const callSt = fn && (fn.kind === SK.ArrowFunction || fn.kind === SK.FunctionExpression) && fn.parent.kind === SK.CallExpression && fn.parent.parent.kind === SK.ExpressionStatement ? fn.parent.parent : null;
    if (callSt && !side && !/^src\/ui\/options/.test(file) && fn.parent.arguments.some((a: any) => a !== fn && ts.isFunctionLike(a) && readIn(a, u.row.row))) { acts.push({ use: u, kind: 'delStmt', node: callSt }); continue; }
    // … or kept as data: `{ label, get: () => ROW, set: v => { ROW = v; } }` in a list of a lab's switches → that entry goes
    { const part = !fn ? null : fn.kind === SK.MethodDeclaration ? fn : (fn.kind === SK.ArrowFunction || fn.kind === SK.FunctionExpression) && upParen(fn).parent.kind === SK.PropertyAssignment ? upParen(fn).parent : null, obj = part?.parent;
      if (obj?.kind === SK.ObjectLiteralExpression && !side && !/^src\/ui\/options/.test(file) && upParen(obj).parent.kind === SK.ArrayLiteralExpression
        && part.name?.getText() === 'set' && obj.properties.some((q: any) => q !== part && q.name?.getText() === 'get' && (q.kind === SK.MethodDeclaration || (q.kind === SK.PropertyAssignment && ts.isFunctionLike(unparen(q.initializer)))) && readIn(q, u.row.row))) {   // (get and set by name: a thing with an `enter()` that writes the row and an `active()` that reads it is the game's own state)
        acts.push({ use: u, kind: 'delProp', node: upParen(obj) }); report.controls.push(`${where} ${u.row.row}: ${(obj.getText() as string).slice(0, 70).replace(/\s+/g, ' ')}`); continue; } }
    // a debug hook's setter (`set on(v) { ROW = !!v; }` in an object handed to the window): the setter and its getter go, and the hook is said (a page tool may use it)
    if (fn && fn.kind === SK.SetAccessor && fn.parent.kind === SK.ObjectLiteralExpression && !side) { acts.push({ use: u, kind: 'delProp', node: fn });
      const g = fn.parent.properties.find((q: any) => q.kind === SK.GetAccessor && q.name.getText() === fn.name.getText()); if (g) acts.push({ use: u, kind: 'delProp', node: g });
      report.hooks.push(`${where}: ${(fn.parent.parent.getText() as string).slice(0, 50).replace(/\s+/g, ' ')} … .${fn.name.getText()}`); continue; }
    if (side) { const v = litValue(rhs);
      report.toolWrites.push({ row: u.row.row, at: where, kind: v === undefined ? 'other' : v === u.row.value ? 'same' : 'opposite', text: shown }); acts.push({ use: u, kind: 'unassign', node: op }); continue; }
    if (/^src\/ui\/options/.test(file)) { refuse.set(u.row.row, `${where}: Options writes it: the player owns this row`); continue; }
    if (file === 'src/ui/bindings.ts' || bootWrite(op)) { acts.push({ use: u, kind: 'unassign', node: op });   // (the debugger's panel putting it down beside another row, the page's address, the stand's hold, the journal's line for an old file)
      if (!/^src\/(host\/standup|game\/journal|ui\/bindings)\.ts$/.test(file)) pageWrites.push({ row: u.row.row, node: op, rhs: given(op) }); continue; }
    refuse.set(u.row.row, `${where}: the game writes it as it runs: state, not a switch (${shown})`); continue;
  }
  if ((op.kind === SK.PrefixUnaryExpression || op.kind === SK.PostfixUnaryExpression) && (op.operator === SK.PlusPlusToken || op.operator === SK.MinusMinusToken)) { refuse.set(u.row.row, `${where}: ++ or -- on the row`); continue; }
  const sw = tool ? null : ts.findAncestor(outer, (x: any) => x.parent?.kind === SK.SwitchStatement && x.parent.expression === x || ts.isFunctionLike(x) || (x.kind >= SK.FirstStatement && x.kind <= SK.LastStatement));
  if (sw && sw.parent?.kind === SK.SwitchStatement && sw.parent.expression === sw) { const op = sw.parent; refuse.set(u.row.row, `${where}: the subject of a switch (${(op.getText() as string).slice(0, 60).replace(/\s+/g, ' ')}): its cases are not folded by this tool. Fold it by hand first`); continue; }
  // the debugger's getter: `() => ROW` handed to a call that is a statement and that also holds the setter → the same statement goes (found from the setter); a lone getter folds
  acts.push({ use: u, kind: 'read', node: access });
}
for (const [k, v] of twinWhy) refuse.set(k, `the Rust goal's: ${v}`);
for (const [row, where] of loadRead) if (loadWrite.has(row) && !refuse.has(row)) refuse.set(row, `${where}: read on a copy of the table taken as its module loads, and the module also writes the row as it loads: which came first is not this tool's to say`);
// ---- a row reached by a KEY THAT IS NOT WRITTEN OUT (`TABLE[k]`, k walking a list of words), and a row NAMED AS DATA beside its table's name (the registry's
// name for a table is how a panel's list of rows, or what a machine keeps of it, says which table: `rows('TABLE', [{ k: 'key', … }])`, `TABLE: { key: … }`,
// `Object.assign(KEPT.TABLE, { key: … })`). Typed code that names a key some other way stops compiling when the row goes, and both typechecks are the proof's.
const keyLists: { el: any; row: string }[] = [], named: { node: any; row: string }[] = []; const namedAt: string[] = [];
{ const byTable = new Map<string, Row[]>(); for (const r of rows) if (rowDef.has(r.row) && !r.hold) { const t = r.row.split('.')[0]; byTable.set(t, [...(byTable.get(t) ?? []), r]); }
  const rowOf = (t: string, k: string) => byTable.get(t)?.find(x => x.row === `${t}.${k}`) ?? null;   // (k: a key, or keys a level down joined by dots)
  /** the entries of `{ key: …, part: { key: … } }` that are rows of the table: [the entry, its row] */
  const rowsIn = (t: string, o: any, pre = ''): [any, Row][] => { const out: [any, Row][] = []; for (const q of o.properties) { const k = keyName(q); if (!k) continue; const r = rowOf(t, pre + k);
      if (r) out.push([q, r]); else { const sub = q.kind === SK.PropertyAssignment ? unparen(q.initializer) : null; if (sub?.kind === SK.ObjectLiteralExpression) out.push(...rowsIn(t, sub, `${pre}${k}.`)); } } return out; };
  const strList = (e: any, depth = 0): any => { e = unparen(e); if (!e) return null;
    if (e.kind === SK.ArrayLiteralExpression) return e.elements.length && e.elements.every((x: any) => x.kind === SK.StringLiteral) ? e : null;
    if (e.kind === SK.Identifier && depth < 3) { const d = checker.getSymbolAtLocation(e)?.valueDeclaration; if (d?.kind === SK.VariableDeclaration && d.initializer) return strList(d.initializer, depth + 1); }
    return null; };
  /** the list of words a key walks — `for (const k of LIST)`, `LIST.forEach(k => …)` and the like — or null */
  const keySource = (arg: any): any => { arg = unparen(arg); if (arg?.kind !== SK.Identifier) return null; const d = checker.getSymbolAtLocation(arg)?.valueDeclaration; if (!d) return null;
    if (d.kind === SK.VariableDeclaration && d.parent.parent?.kind === SK.ForOfStatement) return withSource(strList(d.parent.parent.expression), d.parent.parent.expression);
    if (d.kind === SK.Parameter && ts.isFunctionLike(d.parent) && d.parent.parameters[0] === d) { const call = upParen(d.parent).parent;
      if (call?.kind === SK.CallExpression && unparen(call.arguments[0]) === d.parent && call.expression.kind === SK.PropertyAccessExpression && /^(forEach|map|filter|some|every|find|findIndex)$/.test(call.expression.name.text)) return withSource(strList(call.expression.expression), call.expression.expression); }
    return null; };
  /** a list of keys that has a NAME and is read somewhere else too: taking a key out of it changes that other reader */
  const sharedName = new Map<any, string>();
  const withSource = (arr: any, from: any): any => { if (!arr) return null; const f = unparen(from); if (f?.kind === SK.Identifier) { const d = checker.getSymbolAtLocation(f)?.valueDeclaration;
      if (d?.name && service.findReferences(d.getSourceFile().fileName, d.name.getStart())?.flatMap((g: any) => g.references).filter((e: any) => !e.isDefinition).length > 1) sharedName.set(arr, f.text); } return arr; };
  /** is this expression THE TABLE (its own name, an import of it, or — in a tool, where it came through an untyped import — a thing of that name)? A property
   *  of other data that is merely called the same (`recording.tables.TABLE`) is not. */
  const declOf = new Map<string, any>(); for (const t of byTable.keys()) declOf.set(t, tableDecl(t)[0]);
  const tableOf = (x: any): string | null => { x = unparen(x); const nm = x?.kind === SK.Identifier ? x : x?.kind === SK.PropertyAccessExpression ? x.name : null; if (!nm || !byTable.has(nm.text)) return null;
    let sym = checker.getSymbolAtLocation(nm); if (sym && (sym.flags & ts.SymbolFlags.Alias)) sym = checker.getAliasedSymbol(sym);
    return sym?.valueDeclaration === declOf.get(nm.text) || (checker.getTypeAtLocation(x).flags & ts.TypeFlags.Any) ? nm.text : null; };
  const calledAs = (x: any): string | null => { x = unparen(x); return x?.kind === SK.PropertyAccessExpression && byTable.has(x.name.text) ? x.name.text : null; };
  /** a row named in a map under its table's name: 'names' = the entry says something ABOUT the row (null, a label, a list) or holds the page's own value, and goes with it;
   *  'writes' = it holds another value of the row's own kind, so wherever the map is laid over the table the game writes the row */
  const mapEntry = (q: any, r: Row): 'names' | 'writes' => { const i = q.kind === SK.PropertyAssignment ? unparen(q.initializer) : null, v = litValue(i);
    if (v !== undefined) return typeof v === typeof r.value && v !== r.value ? 'writes' : 'names';
    return !i || i.kind === SK.NullKeyword || i.kind === SK.ArrayLiteralExpression || i.kind === SK.ObjectLiteralExpression || i.kind === SK.ArrowFunction || i.kind === SK.FunctionExpression ? 'names' : 'writes'; };   // (null, a list, a record, a function say something ABOUT the row; a name or a sum may be a value for it)
  const keyName = (p: any): string | null => (p.kind === SK.PropertyAssignment || p.kind === SK.ShorthandPropertyAssignment) && p.name && (p.name.kind === SK.Identifier || p.name.kind === SK.StringLiteral) ? p.name.text : null;
  for (const f of files) { const text = textOf(f); if (![...byTable.keys()].some(t => text.includes(t))) continue; const sf = sfOf(f); if (!sf) continue; const tool = isTool(f);
    const say = (n: any, row: string) => { named.push({ node: n, row }); namedAt.push(`${at(sf, n.getStart(sf))} ${row}: ${(n.getText() as string).slice(0, 70).replace(/\s+/g, ' ')}`); };
    const visit = (n: any) => {
      if (n.kind === SK.ElementAccessExpression && n.argumentExpression && n.argumentExpression.kind !== SK.StringLiteral && n.argumentExpression.kind !== SK.NumericLiteral) { const t = tableOf(n.expression);
        if (t) { const list = keySource(n.argumentExpression), outer = upParen(n), op = outer.parent, write = op.kind === SK.BinaryExpression && op.left === outer && op.operatorToken.kind === SK.EqualsToken, where = at(sf, n.getStart(sf)), shown = ((stmtOf(n) ?? n).getText() as string).slice(0, 100);
          if (list) { for (const e of list.elements) { const r = rowOf(t, e.text); if (!r) continue;
              if (!tool && !write) refuse.set(r.row, `${where}: read through a list of keys, and a list without the key may not be the same game (${shown}): fold that by hand first`);
              else if (!tool && write && !bootWrite(op)) refuse.set(r.row, `${where}: the game writes it as it runs, through a list of keys: state, not a switch (${shown})`);
              else if (sharedName.has(list)) { if (!tool) refuse.set(r.row, `${where}: written through a list of keys that something else reads too (${sharedName.get(list)}): take the key out of that list by hand first`); else note(`${where}: ${r.row} is in a list of keys a tool reads in two places (${sharedName.get(list)}): left as it is`); }
              else { keyLists.push({ el: e, row: r.row }); if (tool) report.toolWrites.push({ row: r.row, at: where, kind: 'other', text: shown }); } } }
          else if (!tool) { const kt = checker.getTypeAtLocation(n.argumentExpression), names: string[] = (kt.isUnion?.() ? kt.types : [kt]).filter((x: any) => x.isStringLiteral?.()).map((x: any) => x.value as string);
            for (const nm of names) { const r = rowOf(t, nm); if (r) refuse.set(r.row, `${where}: ${write ? 'written' : 'read'} by a key whose type names it ('${names.slice(0, 5).join("' | '")}'${names.length > 5 ? ' …' : ''}: ${shown}): fold that by hand first`); }
            if (!names.length) note(`${where}: ${t} is ${write ? 'written' : 'read'} by a key worked out as it runs (${shown}): a retired row that can be named there wants a hand`); } } }
      if (n.kind === SK.CallExpression) {
        // rows('TABLE', [{ k: 'key', … }, …]): a list of the table's rows by name
        // … and the same list handed the table itself: knobRows(sec, TABLE as unknown as KnobTable, …, [{ k: 'key', … }, …]). The key is the entry's `k` (or `key`, `row`): a label that reads like a key is a label.
        if (n.expression.getText() !== 'Object.assign') n.arguments.forEach((a: any, i: number) => { const t: string | null = a.kind === SK.StringLiteral ? (byTable.has(a.text) ? a.text : null) : tableOf(a); if (!t) return; for (const b of n.arguments.slice(i + 1)) { const arr = unparen(b); if (arr?.kind !== SK.ArrayLiteralExpression) continue;
            for (const el of arr.elements) { const o = unparen(el), kp = o?.kind === SK.ObjectLiteralExpression ? o.properties.find((q: any) => q.kind === SK.PropertyAssignment && /^(k|key|row)$/.test(q.name?.text ?? '') && unparen(q.initializer)?.kind === SK.StringLiteral) : null,
                k = o?.kind === SK.StringLiteral ? o.text : kp ? unparen(kp.initializer).text : null; const r = k ? rowOf(t, k) : null; if (!r) continue;
                // (bare words beside the table ITSELF are keys some helper reads the table by — `anyOf(TABLE, ['a', 'b'])`: a read through a list of keys)
                if (o.kind === SK.StringLiteral && a.kind !== SK.StringLiteral) { if (!tool) refuse.set(r.row, `${at(sf, el.getStart(sf))}: read through a list of keys handed to a call with the table (${(n.getText() as string).slice(0, 80).replace(/\s+/g, ' ')}): fold that by hand first`); else note(`${at(sf, el.getStart(sf))}: ${r.row} is in a list of keys a tool hands a call with the table: left as it is`); continue; }
                say(el, r.row); } } });
        // Object.assign(TABLE, { key: … }) writes the row; Object.assign(<something>.TABLE, { key: … }) names it
        if (n.expression.getText() === 'Object.assign' && n.arguments.length >= 2) { const tgt = unparen(n.arguments[0]), real = tableOf(tgt), t = real ?? calledAs(tgt);
          if (t) for (const b of n.arguments.slice(1)) { const o = unparen(b); if (o?.kind !== SK.ObjectLiteralExpression) continue; for (const [q, r] of rowsIn(t, o)) {
                if (real && !tool && !bootWrite({ getSourceFile: () => sf, right: q, parent: n.parent })) refuse.set(r.row, `${at(sf, q.getStart(sf))}: the game writes it as it runs: state, not a switch (${n.getText().slice(0, 80)})`);
                else if (real && !tool) { say(q, r.row); pageWrites.push({ row: r.row, node: n, rhs: q.kind === SK.PropertyAssignment ? q.initializer : q.name }); }   // (the page's own write of it, through an object)
                else if (!real && !tool && mapEntry(q, r) === 'writes') refuse.set(r.row, `${at(sf, q.getStart(sf))}: a map of tables' rows holds another value for it than the page's (${n.getText().slice(0, 70)}): wherever that map is put over the table, the game writes the row`);
                else { say(q, r.row); if (tool) report.toolWrites.push({ row: r.row, at: at(sf, q.getStart(sf)), kind: 'other', text: n.getText().slice(0, 100) }); } } } } }
      // TABLE: { key: …, … } in a literal: what is kept, shown or allowed of the table, by name
      if (!tool && n.kind === SK.PropertyAssignment && n.name && (n.name.kind === SK.Identifier || n.name.kind === SK.StringLiteral) && byTable.has(n.name.text)) { const o = unparen(n.initializer);
        if (o?.kind === SK.ObjectLiteralExpression) for (const [q, r] of rowsIn(n.name.text, o)) {
            if (mapEntry(q, r) === 'writes') refuse.set(r.row, `${at(sf, q.getStart(sf))}: a map of tables' rows holds another value for it than the page's (${(n.getText() as string).slice(0, 70).replace(/\s+/g, ' ')}): wherever that map is put over the table, the game writes the row`); else say(q, r.row); } }
      n.forEachChild(visit); }; visit(sf); } }
// ---- THE VALUE ASKED FOR MUST BE THE PAGE'S, and the page says what that is: `--page dump.json` is every table as a stand-up of the page left it
// (`TPS_PROFILE=shipped bun run tools/qa/stand-dump.ts --out dump.json`: the shipped stand IS the page with an empty address, held to the real page by
// tools/qa/shipped-page.cjs). A row the dump gives another value, or does not hold, is refused: folding it would be a guess (`--trust ROW` takes the caller's
// word). Reading the page's writes off the code was tried and is not done: their order, an early return and a helper are the code's to run, not this tool's.
// One thing the dump cannot say: the page's write of a row may RIDE a row that stays (`A = B && !q.has('noa')`): with B's own word on the address A was off
// too, and a folded A no longer is. Such a row is refused where the row it rides is not retired in the same run.
{ const pageFile = arg('--page'), dump: any = pageFile ? JSON.parse(readFileSync(pageFile, 'utf8')) : null, tables: any = dump ? dump.tables ?? dump : null;
  const useOf = new Map<any, Use>(); for (const u of uses) if (!u.def) useOf.set(u.node, u);
  const declOfTable = new Map<any, string>(); for (const name of REGISTRY.keys()) { const d = tableDecl(name); if (d.length === 1) declOfTable.set(d[0], name); }
  /** a row of a knob table read in this expression that is not retired in this run, or null */
  const rides = (e: any): string | null => { let hit: string | null = null; const visit = (n: any) => { if (hit) return;
      if (n.kind === SK.PropertyAccessExpression || n.kind === SK.ElementAccessExpression) { const tok = n.kind === SK.PropertyAccessExpression ? n.name : n.argumentExpression, u = useOf.get(tok);
        if (u) { if (refuse.has(u.row.row) || u.row.hold) hit = u.row.row; }
        else for (const [d, name] of declOfTable) { const path = livePath(n, d); if (path && path.length) { const full = `${name}.${path.join('.')}`, par = n.parent; if (!((par.kind === SK.PropertyAccessExpression || par.kind === SK.ElementAccessExpression) && par.expression === n) && !rowDef.has(full)) hit = full; break; } } }
      n.forEachChild(visit); }; visit(e); return hit; };
  for (let again = true; again;) { again = false;
    for (const r of rows) { if (!rowDef.has(r.row) || refuse.has(r.row) || TRUSTED.has(r.row)) continue; let why = '';
      if (!tables) why = `no --page dump was given, so its value in the page is not known (TPS_PROFILE=shipped bun run tools/qa/stand-dump.ts --out dump.json)`;
      else { let v: any = tables; for (const k of r.row.split('.')) v = v === null || typeof v !== 'object' ? undefined : v[k];
        if (v === undefined) why = `the page's dump does not hold it (a table the registry does not list?)`;
        else if (v !== r.value) why = `the page gives it ${JSON.stringify(v)} (${pageFile}), and ${JSON.stringify(r.value)} was asked for`; }
      if (!why) for (const w of pageWrites) { if (w.row !== r.row) continue; const on = rides(w.rhs); if (on && on !== r.row) { why = `${at(w.node.getSourceFile(), w.node.getStart())}: the page's write of it rides ${on}, which stays (${(w.node.getText() as string).slice(0, 80).replace(/\s+/g, ' ')}): with that row's own word on the address this one was off too`; break; } }
      if (why) { refuse.set(r.row, `${why}. --trust ${r.row} takes your word for the value`); again = true; } } } }
// THE JOURNAL'S LINE FOR AN OLD RECORDING — `if (the recording has no word for ROW) ROW = off;` — goes whole with the row. But once the row is gone NO
// recording has its word, so whatever else that question guards would run for every recording made from now on: the line may only go where everything under
// its `if` is the write of a row that retires in this run. Anything else (a row that stays written there, the word read into a name, a question that is not
// an `if`'s) refuses the row. Asked again while it refuses anything: a row refused here may be what another line's `if` writes.
{ const useAt = new Map<any, Use>(); for (const u of uses) if (!u.def) useAt.set(u.node, u);
  const stays = (st: any): string | null => { if (!st) return null; if (st.kind === SK.Block) { for (const x of st.statements) { const w = stays(x); if (w) return w; } return null; }
    if (st.kind === SK.ExpressionStatement) { const e = unparen(st.expression); if (e.kind === SK.BinaryExpression && e.operatorToken.kind === SK.EqualsToken) { const l = unparen(e.left), nm = l.kind === SK.PropertyAccessExpression ? l.name : l.kind === SK.ElementAccessExpression ? l.argumentExpression : null, u = nm ? useAt.get(nm) : undefined;
        if (u && !u.row.hold && rowDef.has(u.row.row) && !refuse.has(u.row.row)) return null; } }
    return (st.getText() as string).slice(0, 70).replace(/\s+/g, ' '); };
  for (let again = true; again;) { again = false; recordingIfs.clear();
    for (const r of recordingReads) { if (refuse.has(r.row)) continue; const iff = ts.findAncestor(r.access, (x: any) => x.kind === SK.IfStatement) as any, asked = !!iff && r.access.getStart() >= iff.expression.getStart() && r.access.end <= iff.expression.end;
      const left = !asked ? null : stays(iff.thenStatement) ?? stays(iff.elseStatement);
      if (!asked) refuse.set(r.row, `${r.where}: the journal reads a recording's own word for the row somewhere that is not the question of an \`if\` (${((stmtOf(r.access) ?? r.access).getText() as string).slice(0, 80).replace(/\s+/g, ' ')}): with the row gone no recording has that word. Fold that line by hand first`);
      else if (left) refuse.set(r.row, `${r.where}: the journal's line for a recording from before the row also does something that stays (${left}): with the row gone no recording has its word, and that would run for every one. Fold that line by hand first`);
      else recordingIfs.add(iff);
      if (refuse.has(r.row)) again = true; } } }
for (const r of rows) { const why = refuse.get(r.row); if (why) report.refused.push({ row: r.row, why }); else if (rowDef.has(r.row)) report.retired.push(r.row); }
const live = new Set(report.retired);
{ const emptied = (list: any[], gone: Set<any>) => list.length > 0 && list.every(x => gone.has(x));
  const byArr = new Map<any, Set<any>>(); for (const k of keyLists) if (live.has(k.row)) { plan(k.el.getSourceFile().fileName).del.add(k.el); byArr.set(k.el.parent, (byArr.get(k.el.parent) ?? new Set()).add(k.el)); }
  // a list every key of which is gone takes its own loop with it, where the loop is written on the list itself (`for (const k of ['a', 'b'] as const) …`, `['a', 'b'].forEach(…)`); a named list stays, empty
  for (const [arr, gone] of byArr) { if (!emptied(arr.elements, gone)) continue; const up = upParen((() => { let x = arr; while (x.parent.kind === SK.AsExpression || x.parent.kind === SK.ParenthesizedExpression) x = x.parent; return x; })()), p = up.parent;
    const st = p.kind === SK.ForOfStatement && p.expression === up ? p : p.kind === SK.PropertyAccessExpression && p.parent.kind === SK.CallExpression && p.parent.parent.kind === SK.ExpressionStatement ? p.parent.parent : null;
    if (st) plan(st.getSourceFile().fileName).del.add(st); }
  const byList = new Map<any, Set<any>>(); for (const k of named) if (live.has(k.row)) { plan(k.node.getSourceFile().fileName).del.add(k.node); byList.set(k.node.parent, (byList.get(k.node.parent) ?? new Set()).add(k.node)); }
  // a call left holding an empty list or an empty object (`rows('TABLE', [])`, `Object.assign(KEPT.TABLE, {})`) says nothing any more and goes, where it is a statement
  for (const [lst, gone] of byList) { if (!emptied(lst.elements ?? lst.properties, gone)) continue; const call = upParen(lst).parent; if (call?.kind === SK.CallExpression && call.parent.kind === SK.ExpressionStatement && call.arguments.some((a: any) => unparen(a) === lst)) plan(call.getSourceFile().fileName).del.add(call.parent); } }
for (const a of acts) { if (!live.has(a.use.row.row)) continue; const P = plan(a.use.sf.fileName);
  if (a.kind === 'read' || a.kind === 'setInit') P.subs.set(a.node, a.use.row.value); else if (a.kind === 'readLoad') P.subs.set(a.node, moduleVal.get(a.use.row.row)!); else if (a.kind === 'const') P.subs.set(a.node, true); else if (a.kind === 'unassign') P.unassign.add(a.node); else if (a.kind === 'hole') P.hole.add(a.node); else P.del.add(a.node); }

// ---- --drop file.ts:name …: a declaration to take out whole (a check that holds nothing, a tool's dead function). What only it used goes with it by the rule of
// step 4; a statement that still calls it goes too, and is said (the probe runner's line for a check that is gone).
for (const spec of DROPS) { const [f, name] = spec.split(':'), file = join(ROOT, f), sf = sfOf(file); if (!sf) { console.error(`--drop ${spec}: no such file in the program`); process.exit(2); }
  let target: any = null, nameNode: any = null;
  for (const st of sf.statements) { if ((st.kind === SK.FunctionDeclaration || st.kind === SK.ClassDeclaration) && st.name?.text === name) { target = st; nameNode = st.name; }
    if (st.kind === SK.VariableStatement) for (const d of st.declarationList.declarations) if (d.name.kind === SK.Identifier && d.name.text === name) { target = st.declarationList.declarations.length === 1 ? st : d; nameNode = d.name; } }
  if (!target) { console.error(`--drop ${spec}: no top-level ${name} in ${f}`); process.exit(2); }
  plan(file).del.add(target);
  for (const g of service.findReferences(file, nameNode.getStart(sf)) ?? []) for (const e of g.references) { const rsf = sfOf(e.fileName); if (!rsf) continue; const tok = ts.getTokenAtPosition(rsf, e.textSpan.start);
    if (rsf === sf && tok.getStart() >= target.getStart() && tok.end <= target.end) continue;
    const spec2 = ts.findAncestor(tok, (x: any) => x.kind === SK.ImportSpecifier || x.kind === SK.ExportSpecifier); if (spec2) { plan(e.fileName).del.add(spec2); continue; }
    const up = upParen(tok); if (up.parent.kind === SK.ArrayLiteralExpression) { plan(e.fileName).del.add(up); note(`${at(rsf, up.getStart(rsf))}: ${name} taken out of a list: ${up.parent.getText().slice(0, 110)}`); continue; }
    let st: any = tok; while (st.parent && !(st.parent.kind === SK.SourceFile || st.parent.kind === SK.Block || st.parent.kind === SK.ModuleBlock || st.parent.kind === SK.CaseClause || st.parent.kind === SK.DefaultClause)) st = st.parent;
    plan(e.fileName).del.add(st); note(`${at(rsf, st.getStart(rsf))}: called ${name}, so it goes: ${st.getText().slice(0, 110)}`); } }

// ---- --drop-at file.ts:line[:words | :column] …: a check's line to take out (the statement at that column — a stack frame's place — or the one on that line
// holding those words): the off arm of a retired switch, found by running the check before and after. What only it used goes by step 4. Only a statement that
// is nothing but the call goes; anything else is said as NOT FOUND, for a hand.
for (const spec of DROP_AT) { const m = /^([^:]+):(\d+)(?::(.*))?$/.exec(spec); if (!m) { console.error(`--drop-at ${spec}: file:line[:words | :column]`); process.exit(2); }
  const file = join(ROOT, m[1]), sf = sfOf(file), line = +m[2] - 1, words = m[3]; if (!sf) { console.error(`--drop-at ${spec}: no such file`); process.exit(2); }
  if (!isTool(file)) { console.error(`--drop-at ${spec}: a check's line (under tools/): the game's own code is not taken out by a place`); process.exit(2); }
  const text = sf.text as string, starts = sf.getLineStarts(), from = starts[line] ?? text.length, to = starts[line + 1] ?? text.length;
  // the place: the column a stack frame gives · the words on that line · the line's first word
  let pos = -1;
  if (words && /^\d+$/.test(words)) pos = Math.min(to - 1, from + Math.max(0, +words - 1));
  else if (words) { pos = text.slice(from, to).indexOf(words); pos = pos < 0 ? -1 : from + pos;
    if (pos < 0) { let best: any = null; const visit = (n: any) => { if (n.getStart(sf) >= to || n.end <= from) return; if (n.kind === SK.ExpressionStatement && (n.getText(sf) as string).includes(words) && (!best || n.end - n.getStart(sf) < best.end - best.getStart(sf))) best = n; n.forEachChild(visit); }; visit(sf);   // (a statement over several lines: the words are on another of them)
      if (best) pos = best.getStart(sf) + (best.getText(sf) as string).indexOf(words); } }
  else { const lead = /^\s*/.exec(text.slice(from, to))![0].length; pos = from + lead; }
  // the statement: the innermost one round that place, and it must be a statement that is nothing but a call (a verdict that is returned, kept in a name or the
  // body of an arrow is part of something else: taking the statement round it out would take that with it)
  const tok = pos < 0 ? null : ts.getTokenAtPosition(sf, pos), st = tok && ts.findAncestor(tok, (x: any) => (x.kind >= SK.FirstStatement && x.kind <= SK.LastStatement) || ts.isFunctionLike(x) || x.kind === SK.ClassDeclaration);
  let why = !st || st.kind !== SK.ExpressionStatement ? 'no statement of its own there (a line that is returned, kept in a name or an arrow\'s whole body wants a hand)' : '';
  if (!why) { let e = unparen(st.expression); if (e.kind === SK.AwaitExpression) e = unparen(e.expression);
    // (a helper that passes a verdict on — `function verdict(ok, what) { line(ok, what); }` — is every check's line at once: its place is its caller's)
    const fn = ts.findAncestor(st, (x: any) => ts.isFunctionLike(x)), params = new Set<any>((fn?.parameters ?? []).map((q: any) => checker.getSymbolAtLocation(q.name)).filter(Boolean));
    const isParam = (x: any) => { x = unparen(x); return x?.kind === SK.Identifier && params.has(checker.getSymbolAtLocation(x)); };
    if (e.kind === SK.CallExpression && params.size && (e.arguments.some(isParam) || e.arguments.some((a: any) => unparen(a)?.kind === SK.TemplateExpression && /PASS/.test(a.getText(sf)) && unparen(a).templateSpans.some((ts2: any) => isParam(ts2.expression))))) why = 'a helper that passes a verdict on: the line to take out is where the helper was called';
    // (and it must BE one verdict: printed by this very statement, or a call of something that prints what it is handed. A call that prints a line a case —
    // `table();` — is a whole group of lines)
    else if (e.kind !== SK.CallExpression) why = 'not a call';
    else { const own = st.getText(sf) as string, printsHere = /\bconsole\.(log|error|info)\b/.test(e.expression.getText(sf)) && /PASS|FAIL/.test(own);
      if (!printsHere && !verdictOfArgs(e)) why = 'the call there is not one verdict (it prints nothing it is handed: a group of lines, or something else): by hand'; } }
  if (why) { note(`--drop-at ${spec}: ${why}`); dropMissed.push(spec); continue; }
  plan(file).del.add(st); droppedLines.push(`${m[1]}:${m[2]} ${(st.getText(sf) as string).slice(0, 120)}`); }

// ---- 3. fold. `konst` = the value an expression is known to have once the rows are their values; `render` = its text after the fold.
const constLines = new Set<string>(); const emptied = new Set<string>();
/** is this call a check's VERDICT (the function it calls writes PASS or FAIL; or, where the function came through an untyped import, it is named as one)? A call
 *  that sets the stage for the next line (`setFlag(!ROW, 'night')`) takes a constant too, and must stay */
function saysVerdict(call: any): boolean { const c = call.expression, nm = c.kind === SK.Identifier ? c : c.kind === SK.PropertyAccessExpression ? c.name : null; if (!nm) return false;
  let sym = checker.getSymbolAtLocation(nm); if (sym && (sym.flags & ts.SymbolFlags.Alias)) sym = checker.getAliasedSymbol(sym); const d = sym?.valueDeclaration ?? sym?.declarations?.[0];
  if (d && !d.getSourceFile().isDeclarationFile) { const t = d.getText() as string; if (/PASS/.test(t) && /FAIL/.test(t)) return true; }
  return /^(line|check|verdict|ok|expect|assert)$/i.test(nm.text); }   // (reached through a type, or a helper that passes it on: by its name)
/** … and is the verdict it prints fed by what it is HANDED (`line(ok, label)`), not worked out inside it (`table()` printing a line a case)? Only the first is
 *  one line of a check; the second is a whole group, and taking its call out takes every line of the group. Unknown where the function cannot be read: by name. */
function verdictOfArgs(call: any): boolean { const c = call.expression, nm = c.kind === SK.Identifier ? c : c.kind === SK.PropertyAccessExpression ? c.name : null; if (!nm || !call.arguments.length) return false;
  let sym = checker.getSymbolAtLocation(nm); if (sym && (sym.flags & ts.SymbolFlags.Alias)) sym = checker.getAliasedSymbol(sym); let d = sym?.valueDeclaration ?? sym?.declarations?.[0];
  if (d?.kind === SK.VariableDeclaration || d?.kind === SK.PropertyAssignment || d?.kind === SK.PropertyDeclaration) d = unparen(d.initializer);
  if (!d || d.getSourceFile().isDeclarationFile || !ts.isFunctionLike(d) || !d.body) return /^(line|check|verdict|ok|expect|assert)$/i.test(nm.text);
  const params = new Set<any>(d.parameters.map((q: any) => checker.getSymbolAtLocation(q.name)).filter(Boolean)); let fed = false, prints = false;
  const visit = (n: any) => { if (fed) return; if (n !== d && ts.isFunctionLike(n)) return;
    if (n.kind === SK.ExpressionStatement || n.kind === SK.ReturnStatement || n.kind === SK.VariableStatement || n.kind === SK.IfStatement) { const t = n.getText() as string; if (/PASS/.test(t) && /FAIL/.test(t)) { prints = true; const uses2 = (x: any): boolean => (x.kind === SK.Identifier && params.has(checker.getSymbolAtLocation(x))) || !!ts.forEachChild(x, (y: any) => uses2(y) || undefined); if (uses2(n)) { fed = true; return; } } }
    n.forEachChild(visit); }; visit(d.body);
  return fed || (!prints && /^(line|check|verdict|ok|expect|assert)$/i.test(nm.text)); }   // (a loop left with nothing in it: said, for a hand)
const localConst = new Map<any, Value>();   // a `const x = <now a constant>` declaration → its value (its uses fold too, and it goes)
function konst(n: any, P: FilePlan): { v: Value } | null {
  if (!n) return null;
  if (P.subs.has(n)) return { v: P.subs.get(n)! };
  switch (n.kind) {
    case SK.ParenthesizedExpression: return konst(n.expression, P);
    case SK.TrueKeyword: return { v: true }; case SK.FalseKeyword: return { v: false };
    case SK.NumericLiteral: return { v: Number(n.text) }; case SK.StringLiteral: case SK.NoSubstitutionTemplateLiteral: return { v: n.text };
    case SK.PrefixUnaryExpression: { const c = konst(n.operand, P); if (!c) return null; return n.operator === SK.ExclamationToken ? { v: !c.v } : n.operator === SK.MinusToken && typeof c.v === 'number' ? { v: -c.v } : n.operator === SK.PlusToken && typeof c.v === 'number' ? c : null; }
    case SK.Identifier: { if (!touched(n, P)) return null; const s = checker.getSymbolAtLocation(n), d = s?.valueDeclaration; return d && localConst.has(d) && n !== d.name ? { v: localConst.get(d)! } : null; }
    case SK.BinaryExpression: { const k = n.operatorToken.kind;
      if (k === SK.AmpersandAmpersandToken) { const l = konst(n.left, P); if (l && !l.v) return { v: l.v }; const r = konst(n.right, P); if (l && r) return { v: r.v }; if (r && !r.v && pure(n.left) && (boolCtx(n) || (r.v === false && isBoolType(n.left)))) return { v: false }; return null; }   // (x && 0 is false OR 0: one thing only where its truth alone is read)
      if (k === SK.BarBarToken) { const l = konst(n.left, P); if (l && l.v) return { v: l.v }; const r = konst(n.right, P); if (l && r) return { v: r.v }; if (r && r.v && pure(n.left) && (boolCtx(n) || (r.v === true && isBoolType(n.left)))) return { v: true }; return null; }
      if (k === SK.QuestionQuestionToken) { const l = konst(n.left, P); return l ? { v: l.v } : null; }   // (a row is never null: what stands after ?? is never reached)
      if (k === SK.EqualsEqualsEqualsToken || k === SK.ExclamationEqualsEqualsToken) { const l = konst(n.left, P), r = konst(n.right, P); return l && r ? { v: (l.v === r.v) === (k === SK.EqualsEqualsEqualsToken) } : null; }
      if (k === SK.LessThanToken || k === SK.GreaterThanToken || k === SK.LessThanEqualsToken || k === SK.GreaterThanEqualsToken) { const l = konst(n.left, P), r = konst(n.right, P); if (!l || !r || typeof l.v !== 'number' || typeof r.v !== 'number') return null;
        return { v: k === SK.LessThanToken ? l.v < r.v : k === SK.GreaterThanToken ? l.v > r.v : k === SK.LessThanEqualsToken ? l.v <= r.v : l.v >= r.v }; }
      return null; }
    case SK.ConditionalExpression: { const c = konst(n.condition, P); return c ? konst(c.v ? n.whenTrue : n.whenFalse, P) : null; }
  }
  return null;
}
/** does this node hold anything the run changes? (most of a file does not: its text is copied) */
const marks = new Map<string, number[]>();
function touched(n: any, P: FilePlan): boolean { const sf = n.getSourceFile(), m = marks.get(sf.fileName); if (!m) return false; const a = n.pos, b = n.end;
  let lo = 0, hi = m.length; while (lo < hi) { const mid = (lo + hi) >> 1; if (m[mid] < a) lo = mid + 1; else hi = mid; } return lo < m.length && m[lo] < b; }
function remark(): void { marks.clear(); for (const [f, P] of plans) { const m: number[] = []; for (const n of P.subs.keys()) m.push(n.getStart()); for (const n of P.del) m.push(n.getStart()); for (const n of P.unassign) m.push(n.getStart()); for (const n of P.bare) m.push(n.getStart()); for (const n of P.hole) m.push(n.getStart()); for (const st of sfOf(f)?.statements ?? []) if (st.kind === SK.ImportDeclaration) m.push(st.getStart());   // (every import of a file the run changes is asked whether its module still loads)
    m.sort((x, y) => x - y); marks.set(f, m); } }
const drop = (n: any, P: FilePlan) => { P.dropped.push({ start: n.getStart(), end: n.end }); };
const hasLexical = (b: any) => b.statements.some((s: any) => (s.kind === SK.VariableStatement && (s.declarationList.flags & (ts.NodeFlags.Let | ts.NodeFlags.Const))) || s.kind === SK.ClassDeclaration || s.kind === SK.FunctionDeclaration);
const multiOK = (st: any) => { const p = st.parent; return p && (p.kind === SK.Block || p.kind === SK.SourceFile || p.kind === SK.ModuleBlock || p.kind === SK.CaseClause || p.kind === SK.DefaultClause); };
const EMPTY = '\u0000';   // a statement that is gone (its line is tidied when the file is put together)
function render(n: any, P: FilePlan): string {
  const sf = n.getSourceFile(), text = sf.text as string;
  if (P.del.has(n)) { drop(n, P); return EMPTY; }
  if (P.hole.has(n)) { drop(n, P); return ''; }
  if (!touched(n, P)) return text.slice(n.getStart(sf), n.end);
  if (ts.isExpression(n) || n.kind === SK.Identifier) { const c = konst(n, P); if (c && n.kind !== SK.TrueKeyword && n.kind !== SK.FalseKeyword) { drop(n, P); const t = lit(c.v), p = n.parent;
      // (a number where a name stood: `3.toFixed()` and `a--1` are not what was meant)
      return typeof c.v === 'number' && ((p.kind === SK.PropertyAccessExpression && p.expression === n) || (c.v < 0 && (p.kind === SK.BinaryExpression || p.kind === SK.PrefixUnaryExpression || p.kind === SK.PostfixUnaryExpression))) ? `(${t})` : glue(n, t); } }
  switch (n.kind) {
    case SK.BinaryExpression: { const k = n.operatorToken.kind;
      if (P.unassign.has(n)) { drop(n.left, P); return bare(n, render(n.right, P)); }   // (`ROW = x` inside a chain or an expression: what is left is x)
      if (k === SK.CommaToken) { const L = unparen(n.left); if (L.kind === SK.BinaryExpression && P.unassign.has(L) && pure(L.right)) { drop(n.left, P); return render(n.right, P); } }   // (`(ROW = x, y)`: what is left is y)
      if (k === SK.AmpersandAmpersandToken || k === SK.BarBarToken) { const and = k === SK.AmpersandAmpersandToken, l = konst(n.left, P), r = konst(n.right, P);
        if (l && !!l.v === and) { drop(n.left, P); return bare(n, render(n.right, P)); }                      // true && x → x · false || x → x
        if (!and && madeThing(n.left, P)) { drop(n.right, P); return bare(n, render(n.left, P)); }            // { … } || x → { … } (a thing just made is never false)
        if (r && !!r.v === and && (boolCtx(n) || (r.v === and && isBoolType(n.left)))) { drop(n.right, P); return bare(n, render(n.left, P)); } }   // x && true → x · x || false → x (where only its truth is read: `x && 3` is not x)
      break; }
    case SK.ConditionalExpression: { const c = konst(n.condition, P); if (c) { drop(c.v ? n.whenFalse : n.whenTrue, P); drop(n.condition, P); const keep = c.v ? n.whenTrue : n.whenFalse, t = render(keep, P);
        if (bare(n, t) !== t) return bare(n, t);
        return keep.kind === SK.BinaryExpression || keep.kind === SK.ConditionalExpression || keep.kind === SK.ArrowFunction ? needsParens(n) ? `(${t})` : t : t; } break; }
    case SK.ParenthesizedExpression: { const inner = render(n.expression, P), e = unparen(n.expression);   // (x) left bare by a fold keeps its brackets only where it needs them
      if (inner !== text.slice(n.expression.getStart(sf), n.expression.end) && /^[\w.$]+$|^[\w.$]+\([^()]*\)$/.test(inner) && !/^\d/.test(inner) && e) return glue(n, inner); return `(${inner})`; }   // (a number keeps them: `3.toFixed()` is not what was meant)
    case SK.ArrowFunction: { if (n.body.kind === SK.Block) break; const whole = splice(n, P), b = render(n.body, P);   // (a body left starting with a brace would be read as a block: `() => ({ a: 1 })`)
      return /^\s*\{/.test(b) && whole.endsWith(b) ? `${whole.slice(0, whole.length - b.length)}(${b})` : whole; }
    case SK.IfStatement: { const c = konst(n.expression, P);
      if (c) { const keep = c.v ? n.thenStatement : n.elseStatement, gone = c.v ? n.elseStatement : n.thenStatement; if (gone) drop(gone, P); drop(n.expression, P);
        if (!keep) return EMPTY; const t = render(keep, P); if (t === EMPTY) return EMPTY;
        if (keep.kind === SK.Block && !hasLexical(keep) && multiOK(n)) { const inner = t.replace(/^\{\s?/, '').replace(/\s?\}$/, ''); return inner.trim() ? inner : EMPTY; }
        return t; }
      const then = render(n.thenStatement, P), els = n.elseStatement ? render(n.elseStatement, P) : null, cond = render(n.expression, P);
      const thenGone = then === EMPTY || /^\{\s*\}$/.test(then), elseGone = els === null || els === EMPTY || /^\{\s*\}$/.test(els);
      if (thenGone && elseGone) { if (pure(n.expression) || recordingIfs.has(n)) { drop(n.expression, P); return EMPTY; } return `${cond};`; }   // (the journal's line for an old file asks the recording for the retired row's own word: with its write gone, the question goes too)
      if (thenGone) return `if (!(${cond})) ${els}`;
      const head = text.slice(n.getStart(sf), n.expression.getStart(sf)), mid = text.slice(n.expression.end, n.thenStatement.getStart(sf));
      // (an `if` inside this one, unbraced, that this run left without its `else`, or turned round: this one's own `else` would become the inner one's)
      const own = !elseGone && n.thenStatement.kind !== SK.Block && then !== text.slice(n.thenStatement.getStart(sf), n.thenStatement.end) && /\bif\s*\(/.test(then) ? `{ ${then} }` : then;
      return head + cond + mid + own + (elseGone ? '' : text.slice(n.thenStatement.end, n.elseStatement.getStart(sf)) + els); }
    case SK.WhileStatement: { const c = konst(n.expression, P); if (c && !c.v) { drop(n, P); return EMPTY; } break; }   // (`while (false) …` never turns)
    case SK.TryStatement: { const body = render(n.tryBlock, P); if (/^\{\s*\}$/.test(body.replace(new RegExp(EMPTY, 'g'), '')) && !n.finallyBlock) { drop(n, P); return EMPTY; } break; }
    case SK.Block: case SK.SourceFile: case SK.CaseClause: case SK.DefaultClause: case SK.ModuleBlock: return spliceStatements(n, P);
    case SK.ExpressionStatement: { { let x = unparen(n.expression), cut = false; while (x.kind === SK.BinaryExpression && P.unassign.has(x)) { x = unparen(x.right); cut = true; }
        if (cut && !(x.kind === SK.BinaryExpression && x.operatorToken.kind === SK.EqualsToken) && pure(x)) { drop(n, P); return EMPTY; }   // (every target of the write is gone and what was written does nothing by itself)
        const d0 = unparen(n.expression); if (d0.kind === SK.BinaryExpression && d0.operatorToken.kind === SK.EqualsToken && unparen(d0.left).kind === SK.ArrayLiteralExpression && unparen(d0.left).elements.length > 0 && unparen(d0.left).elements.every((e2: any) => P.hole.has(upParen(e2)) || P.hole.has(e2)) && pure(d0.right)) { drop(n, P); return EMPTY; } }
      const e = render(n.expression, P); if (e === EMPTY) return EMPTY; const c = konst(n.expression, P); if (c) return EMPTY;
      // a check's line that asked only what the switch was set to (`line(before === false && carried, …)`, `F.verdict(…)`): its condition is a constant now, it holds nothing, it goes
      const call = unparen(n.expression); if (isTool(sf.fileName) && call.kind === SK.CallExpression && call.arguments.length >= 2 && konst(call.arguments[0], P) && !P.subs.has(call.arguments[0]) && call.arguments[0].kind !== SK.TrueKeyword && call.arguments[0].kind !== SK.FalseKeyword && saysVerdict(call)) {
        constLines.add(`${at(sf, n.getStart(sf))} ${n.getText().slice(0, 140)}`); drop(n, P); return EMPTY; }
      break; }
    case SK.VariableStatement: { if (P.bare.has(n)) { const d = n.declarationList.declarations[0]; P.dropped.push({ start: d.name.getStart(sf), end: d.name.end }); const t = render(d.initializer, P); return (/^(\{|function\b|class\b)/.test(t) ? `(${t})` : t) + ';'; }
      const kept = n.declarationList.declarations.filter((d: any) => !goneDecl(d, P)); if (!kept.length) { drop(n, P); return EMPTY; } for (const d of n.declarationList.declarations) if (goneDecl(d, P)) drop(d, P);
      if (kept.length !== n.declarationList.declarations.length) { const kw = text.slice(n.getStart(sf), n.declarationList.declarations[0].getStart(sf)); return kw + kept.map((d: any) => render(d, P)).join(', ') + ';'; } break; }
    case SK.VariableDeclarationList: { const kept = n.declarations.filter((d: any) => !goneDecl(d, P)); for (const d of n.declarations) if (goneDecl(d, P)) drop(d, P); if (kept.length !== n.declarations.length && kept.length) { const kw = text.slice(n.getStart(sf), n.declarations[0].getStart(sf)); return kw + kept.map((d: any) => render(d, P)).join(', '); } break; }
    case SK.ObjectLiteralExpression: case SK.ArrayLiteralExpression: case SK.TypeLiteral: case SK.NamedImports: case SK.NamedExports: case SK.ClassDeclaration: case SK.ClassExpression: return spliceList(n, P);
    case SK.TemplateExpression: return splice(n, P).replace(/\$\{'([^'\\`${}]*)'\}/g, '$1').replace(/\$\{(true|false|-?\d+(?:\.\d+)?)\}/g, '$1');   // (a span that became a word is the word)
    // An import LOADS its module where one of its names is used as a value (a transpiler drops an import all of whose names are types, or unused). If it
    // loaded the module before this run and would not after it — its last name gone, or only names of types left — it stays as a bare import, in the place it
    // was: src/game is one import cycle and the order its modules load in is the game's.
    case SK.ImportDeclaration: { const t = splice(n, P), clause = n.importClause, emptyNow = /^import\s*(type\s*)?\{[\s,]*\}\s*from\s*['"][^'"]+['"];?/.test(t.replace(new RegExp(EMPTY, 'g'), ''));
      if (!clause || clause.isTypeOnly) return emptyNow ? EMPTY : t;
      const names: { name: any; node: any }[] = []; if (clause.name) names.push({ name: clause.name, node: clause }); const nb = clause.namedBindings; if (nb?.kind === SK.NamespaceImport) names.push({ name: nb.name, node: nb }); if (nb?.kind === SK.NamedImports) for (const el of nb.elements) if (!el.isTypeOnly) names.push({ name: el.name, node: el });
      const loaded = names.some(x => valueUses(x.name).length > 0), loads = names.some(x => !P.del.has(x.node) && valueUses(x.name).some((tok: any) => !(inSpans(lastDropped.get(sf.fileName) ?? [], tok.getStart(sf)) || inDel(P, tok.getStart(sf)))));
      if (loaded && !loads) return emptyNow ? `import ${n.moduleSpecifier.getText(sf)};` : `import ${n.moduleSpecifier.getText(sf)}; ${t}`;
      return emptyNow ? EMPTY : t; }
  }
  return splice(n, P);
}
/** text put where a bracketed expression stood, right after a word (`typeof(x)`, `return(x)`): a space keeps the two apart */
const glue = (n: any, t: string): string => n.kind === SK.ParenthesizedExpression && /[\w$]/.test((n.getSourceFile().text as string)[n.getStart() - 1] ?? '') && /^[\w$'"`]/.test(t) ? ' ' + t : t;
/** text put where an expression stood, as an arrow's whole body or a statement's start: an object's (or a function's) own first word would be read as a block (or a declaration) */
const bare = (n: any, t: string): string => (n.parent.kind === SK.ArrowFunction && n.parent.body === n && /^\s*\{/.test(t)) || (n.parent.kind === SK.ExpressionStatement && /^\s*(\{|function\b|class\b)/.test(t)) ? `(${t})` : t;
/** does this expression, once folded, come to a thing just made (an object, a list, a function): never false, never null? */
function madeThing(n: any, P: FilePlan): boolean { n = unparen(n); if (!n) return false;
  if (n.kind === SK.ObjectLiteralExpression || n.kind === SK.ArrayLiteralExpression || n.kind === SK.ArrowFunction || n.kind === SK.FunctionExpression) return true;
  if (n.kind === SK.BinaryExpression && n.operatorToken.kind === SK.AmpersandAmpersandToken) { const l = konst(n.left, P); return !!l && !!l.v && madeThing(n.right, P); }
  if (n.kind === SK.ConditionalExpression) { const c = konst(n.condition, P); return !!c && madeThing(c.v ? n.whenTrue : n.whenFalse, P); }
  return false; }
const needsParens = (n: any) => { const p = n.parent; return p && (p.kind === SK.BinaryExpression || p.kind === SK.PrefixUnaryExpression || p.kind === SK.PropertyAccessExpression || p.kind === SK.CallExpression && p.expression === n || p.kind === SK.ConditionalExpression && p.condition === n || p.kind === SK.TemplateSpan && false); };
const goneDecl = (d: any, P: FilePlan) => P.del.has(d) || localConst.has(d);
/** does control always leave here? (a return, a throw, a break, a continue; a block that ends in one; an `if` this run settled whose kept branch does) */
function alwaysJumps(st: any, P: FilePlan): boolean { if (!st) return false;
  if (st.kind === SK.ReturnStatement || st.kind === SK.ThrowStatement || st.kind === SK.BreakStatement || st.kind === SK.ContinueStatement) return true;
  if (st.kind === SK.Block) return st.statements.length > 0 && alwaysJumps(st.statements[st.statements.length - 1], P);
  if (st.kind === SK.IfStatement) { const c = konst(st.expression, P); if (c) return alwaysJumps(c.v ? st.thenStatement : st.elseStatement, P); return !!st.elseStatement && alwaysJumps(st.thenStatement, P) && alwaysJumps(st.elseStatement, P); }
  return false; }
/** a run of statements: what follows an `if` this run settled into a plain return (or throw, break, continue) can no longer be reached, and goes with the branch that went */
function spliceStatements(n: any, P: FilePlan): string { const sf = n.getSourceFile(), text = sf.text as string; const list: any[] = n.statements; if (!list) return splice(n, P);
  let out = '', pos = n.kind === SK.SourceFile ? 0 : n.getStart(sf), dead = false;
  for (const st of list) { const s = st.getStart(sf);
    if (dead && st.kind !== SK.FunctionDeclaration) { drop(st, P); out += text.slice(pos, st.jsDoc?.length ? Math.max(pos, st.jsDoc[0].getStart(sf)) : s) + EMPTY; pos = st.end; continue; }
    const r = render(st, P), doc = r === EMPTY && st.jsDoc?.length ? st.jsDoc[0].getStart(sf) : s; out += text.slice(pos, Math.max(pos, doc)) + r; pos = st.end;
    if (st.kind === SK.IfStatement && konst(st.expression, P) && alwaysJumps(st, P)) dead = true; }
  return out + text.slice(pos, n.end); }
/** a node's own text with each child's text put in its place */
function splice(n: any, P: FilePlan): string { const sf = n.getSourceFile(), text = sf.text as string; let out = '', pos = n.kind === SK.SourceFile ? 0 : n.getStart(sf);
  const kids: any[] = []; n.forEachChild((c: any) => { kids.push(c); });
  for (const c of kids) { if (c.kind === SK.JSDoc || c.end <= pos && c.pos < pos) continue; const s = c.getStart(sf); if (s < pos) continue; let r = render(c, P);
    // (the whole body of a loop or a label gone: the loop still turns — its head may draw a die or count — so it keeps an empty body, and is said)
    if (r === EMPTY && n.statement === c && (n.kind === SK.ForStatement || n.kind === SK.ForOfStatement || n.kind === SK.ForInStatement || n.kind === SK.WhileStatement || n.kind === SK.DoStatement || n.kind === SK.LabeledStatement || n.kind === SK.WithStatement)) { r = '{ }'; emptied.add(at(sf, n.getStart(sf))); }
    const doc = r === EMPTY && c.jsDoc?.length ? c.jsDoc[0].getStart(sf) : s;   // (a declaration that goes takes its own /** … */ with it)
    out += text.slice(pos, Math.max(pos, doc)) + r; pos = c.end; }
  return out + text.slice(pos, n.end); }
/** a list (an object's keys, an array's elements, a type's members, an import's names, a class's members): a member that is gone takes its comma with it */
function spliceList(n: any, P: FilePlan): string { const sf = n.getSourceFile(), text = sf.text as string;
  const list: any[] = n.properties ?? n.elements ?? n.members; let out = '', pos = n.getStart(sf);
  if (n.kind === SK.ClassDeclaration || n.kind === SK.ClassExpression) { if (list.length) { out = text.slice(pos, list[0].getFullStart()); pos = list[0].getFullStart(); }
    for (const m of list) { if (P.del.has(m)) { drop(m, P); const from = m.jsDoc?.length ? m.jsDoc[0].getStart(sf) : m.getStart(sf); out += text.slice(pos, Math.max(pos, from)) + EMPTY; pos = m.end; } else { const s = m.getStart(sf); out += text.slice(pos, s) + render(m, P); pos = m.end; } }
    return out + text.slice(pos, n.end); }
  for (let i = 0; i < list.length; i++) { const m = list[i], s = m.getStart(sf);
    // (a member that goes takes its own /** … */ with it; `...(cond ? [] : [row])` folded to `...[]` is a member that went)
    const r = P.del.has(m) ? null : render(m, P), empty = m.kind === SK.SpreadAssignment ? /^\.\.\.\(?\s*(\{\s*\}|null|undefined)\s*\)?$/ : /^\.\.\.\(?\[\s*\]\)?$/;   // (`...[]` in a list, `...{}` or `...null` in an object: nothing)
    if (r === null || ((m.kind === SK.SpreadElement || m.kind === SK.SpreadAssignment) && empty.test(r) && !empty.test(text.slice(s, m.end)))) { drop(m, P); out += text.slice(pos, Math.max(pos, m.jsDoc?.length ? m.jsDoc[0].getStart(sf) : s)) + EMPTY; pos = m.end; const after = /^\s*[,;]?[ \t]?/.exec(text.slice(pos, n.end))!; const sep = after[0]; if (/[,;]/.test(sep)) pos += sep.length; else if (i === list.length - 1) out = out.replace(new RegExp(`,(\\s*${EMPTY})$`), '$1');   // (the last member gone: the comma before it goes too)
      continue; }
    out += text.slice(pos, s) + r; pos = m.end; }
  return out + text.slice(pos, n.end); }

// ---- 4. what only the dropped text used goes too, and a const that became a constant folds on: round after round until nothing more goes
const DECL_KINDS = new Set([SK.ClassDeclaration, SK.FunctionDeclaration, SK.MethodDeclaration, SK.PropertyDeclaration, SK.VariableDeclaration, SK.PropertyAssignment, SK.ImportSpecifier, SK.TypeAliasDeclaration, SK.InterfaceDeclaration, SK.GetAccessor, SK.EnumDeclaration]);
const inSpans = (spans: Span[], pos: number) => spans.some(s => pos >= s.start && pos < s.end);
/** is [a, b) inside something this run takes out whole? (a plan's own list as spans, made again only when the list has grown) */
const delSpans = new Map<FilePlan, Span[]>();
function inDel(P: FilePlan | undefined, a: number, b = a + 1): boolean { if (!P) return false; let sp = delSpans.get(P); if (!sp || sp.length !== P.del.size) { sp = [...P.del].map(x => ({ start: x.getStart(), end: x.end })); delSpans.set(P, sp); } return sp.some(x => a >= x.start && b <= x.end); }
const lastDropped = new Map<string, Span[]>();   // what the pass before this one dropped, by file (an import asks it whether its names are still used)
/** the uses of an imported name, in its own file, that need the VALUE (not a type's position): what makes a transpiler keep the import. One walk of a file
 *  finds them for all its imports. */
const valueUseCache = new Map<string, Map<any, any[]>>();
function valueUses(name: any): any[] { const sf = name.getSourceFile(); let by = valueUseCache.get(sf.fileName);
  if (!by) { by = new Map(); valueUseCache.set(sf.fileName, by);
    const inType = (tok: any): boolean => { for (let x = tok.parent; x; x = x.parent) {
        if (x.kind === SK.ExpressionWithTypeArguments) { const h = x.parent; return !(h.kind === SK.HeritageClause && h.token === SK.ExtendsKeyword && (h.parent.kind === SK.ClassDeclaration || h.parent.kind === SK.ClassExpression)); }
        if (ts.isTypeNode(x) || x.kind === SK.InterfaceDeclaration || x.kind === SK.TypeAliasDeclaration || x.kind === SK.ImportDeclaration) return true; } return false; };
    const visit = (n: any) => { if (n.kind === SK.ImportDeclaration) return;
      if (n.kind === SK.Identifier) { const p = n.parent; let sym = p.kind === SK.ShorthandPropertyAssignment && p.name === n ? checker.getShorthandAssignmentValueSymbol(p) : p.kind === SK.ExportSpecifier ? checker.getExportSpecifierLocalTargetSymbol(p) : checker.getSymbolAtLocation(n);
        if (sym && (sym.flags & ts.SymbolFlags.Alias) && (sym.declarations ?? []).some((d: any) => d.getSourceFile() === sf && (d.kind === SK.ImportSpecifier || d.kind === SK.ImportClause || d.kind === SK.NamespaceImport)) && !inType(n)) by!.set(sym, [...(by!.get(sym) ?? []), n]); }
      n.forEachChild(visit); }; visit(sf); }
  return by.get(checker.getSymbolAtLocation(name)) ?? []; }
const unread = new Map<any, string>();   // a value nothing reads any more whose making still happens: where, for the report
function renderAll(): Map<string, string> { for (const [f, P] of plans) lastDropped.set(f, P.dropped); remark(); constLines.clear(); emptied.clear(); const out = new Map<string, string>();
  for (const [f, P] of plans) { P.dropped = []; const sf = sfOf(f); out.set(f, splice(sf, P)); } return out; }
function refsOf(decl: any): { fileName: string; start: number; def: boolean }[] { const sf = decl.getSourceFile(), name = decl.name; if (!name) return [];
  const out: { fileName: string; start: number; def: boolean }[] = [];
  for (const g of service.findReferences(sf.fileName, name.getStart(sf)) ?? []) for (const e of g.references) out.push({ fileName: e.fileName, start: e.textSpan.start, def: !!e.isDefinition }); return out; }
let rendered = renderAll();
for (let round = 0; round < 12; round++) {
  let grew = false;
  // (a) consts that became constants
  for (const [f, P] of plans) { const sf = sfOf(f); const visit = (n: any) => { if (!touched(n, P)) return;
      if (n.kind === SK.VariableDeclaration && !localConst.has(n) && n.initializer && n.name.kind === SK.Identifier && (n.parent.flags & ts.NodeFlags.Const) && n.parent.parent.kind === SK.VariableStatement
        && !(n.parent.parent.modifiers ?? []).some((m: any) => m.kind === SK.ExportKeyword)) { const c = konst(n.initializer, P);
        if (c && typeof c.v === 'boolean') { const toks = refsOf(n).map(r => ts.getTokenAtPosition(sfOf(r.fileName), r.start)).filter(tok => tok !== n.name);
          if (!toks.some(tok => tok.parent.kind === SK.ShorthandPropertyAssignment || tok.parent.kind === SK.ExportSpecifier || !!ts.findAncestor(tok, (x: any) => x.kind === SK.TypeQuery))) { localConst.set(n, c.v); grew = true; for (const tok of toks) plan(tok.getSourceFile().fileName).subs.set(tok, c.v); } } }   // (`{ x }`, `export { x }` and `typeof x` want the name: such a const stays, a constant)
      n.forEachChild(visit); }; visit(sf); }
  // (b) declarations every use of which is in text this run dropped
  const cand = new Set<any>();
  for (const [f, P] of plans) { const sf = sfOf(f); if (!P.dropped.length) continue;
    const visit = (n: any) => { if (n.end <= P.dropped[0].start && false) return; if (n.kind === SK.Identifier && inSpans(P.dropped, n.getStart(sf))) { const s = checker.getSymbolAtLocation(n);
          const target = s && (s.flags & ts.SymbolFlags.Alias) ? checker.getAliasedSymbol(s) : null;   // (an imported name: the import here, and what it names where it is made)
          for (const d of [...(s?.declarations ?? []), ...(target?.declarations ?? [])]) if (DECL_KINDS.has(d.kind) && !d.getSourceFile().isDeclarationFile && !d.getSourceFile().fileName.includes('node_modules')) cand.add(d); }
      n.forEachChild(visit); };
    for (const s of P.dropped) { const start = ts.getTokenAtPosition(sf, s.start); let top = start; while (top.parent && top.parent.getStart(sf) >= s.start && top.parent.end <= s.end) top = top.parent; visit(top); } }
  for (const d of cand) { const dsf = d.getSourceFile(), D = plan(dsf.fileName); if (D.del.has(d) || localConst.has(d) || unread.has(d)) continue;
    if (d.kind === SK.VariableDeclaration && (d.name.kind !== SK.Identifier || !(d.parent.kind === SK.VariableDeclarationList && d.parent.parent.kind === SK.VariableStatement))) continue;   // (a catch's or a loop's own variable is the catch's and the loop's; a pattern's names are read one by one)
    if (inSpans(D.dropped, d.getStart()) || inDel(D, d.getStart(), d.end)) continue;   // (already inside text that is going)
    if (d.kind === SK.PropertyAssignment && !tableKeyOrLocal(d)) continue;
    if (aliveByName(d)) continue;   // (its name still stands somewhere in text that stays: not dead, and no need to ask the compiler)
    const refs = refsOf(d); let n = 0, allGone = true;
    for (const r of refs) { if (d.kind === SK.ImportSpecifier && r.fileName !== dsf.fileName) continue;   // (an import is used where it is imported: what it names, and every other file's import of that, are not uses of THIS import)
      const rsf = sfOf(r.fileName); if (!rsf) { allGone = false; break; } const tok = ts.getTokenAtPosition(rsf, r.start);
      if (tok === d.name) continue; if (within(tok, d)) continue; n++;
      const Q = plans.get(r.fileName); if (!Q || !(inSpans(Q.dropped, r.start) || inDel(Q, r.start))) { allGone = false; break; } }
    // what made it may still have to happen (a die drawn for it, a call that does something): then only the NAME goes — `const j = draw();` → `draw();` — and it is said
    if (allGone && n > 0 && (d.kind === SK.VariableDeclaration || d.kind === SK.PropertyDeclaration || d.kind === SK.PropertyAssignment) && d.initializer && !pure(d.initializer)) { unread.set(d, `${at(dsf, d.getStart())} ${(d.getText() as string).slice(0, 90).replace(/\s+/g, ' ')}`);
      if (d.kind === SK.VariableDeclaration && d.parent.declarations.length === 1) { D.bare.add(d.parent.parent); grew = true; }
      continue; }
    if (allGone && n > 0) { const target = d.kind === SK.VariableDeclaration && d.parent.declarations.length === 1 && d.parent.parent.kind === SK.VariableStatement ? d.parent.parent : d;
      D.del.add(target); if (target !== d) D.del.add(d); report.dead.push(`${at(dsf, d.getStart())} ${SK[d.kind]} ${d.name?.getText?.() ?? ''}`); grew = true; } }
  if (!grew) break;
  rendered = renderAll();
}
rendered = renderAll();   // (once more: an import asks what the pass BEFORE it dropped, and now that is the last pass's own)
/** a cheap first answer to 'is it still used': does the declaration's NAME appear as a word anywhere outside itself and outside what this run takes out? Its own
 *  file first (a local's every use is there); the other files only for something another file could name. True = alive for sure enough to skip it; false = ask. */
function goneAt(f: string, pos: number): boolean { const Q = plans.get(f); return !!Q && (inSpans(Q.dropped, pos) || inDel(Q, pos)); }
function aliveByName(d: any): boolean { const name = d.name?.text; if (!name || !/^[\w$]+$/.test(name)) return false; const dsf = d.getSourceFile(), re = new RegExp(`(?<![\\w$])${name.replace(/\$/g, '\\$')}(?![\\w$])`, 'g');
  const scan = (f: string): boolean => { const t = textOf(f); if (!t.includes(name)) return false; re.lastIndex = 0; let m: RegExpExecArray | null;
    const sf = sfOf(f); if (!sf) return false;
    while ((m = re.exec(t))) { const pos = m.index; if (f === dsf.fileName && pos >= d.getStart() && pos < d.end) continue; if (goneAt(f, pos)) continue;
      const tok = ts.getTokenAtPosition(sf, pos); if (tok.kind === SK.Identifier && tok.getStart(sf) === pos) return true; }   // (the word in a comment or a string is not a use)
    return false; };
  if (scan(dsf.fileName)) return true;
  const local = d.kind === SK.ImportSpecifier || (!!ts.findAncestor(d.parent, (x: any) => ts.isFunctionLike(x)) && d.kind !== SK.PropertyAssignment && d.kind !== SK.MethodDeclaration && d.kind !== SK.PropertyDeclaration);   // (an import's name is this file's own: the same word where it is exported is not a use of the import — and an import nothing uses is one the bundler drops, which moves when its module loads)
  if (local) return false;
  for (const f of files) if (f !== dsf.fileName && scan(f)) return true;
  return false; }
function within(tok: any, d: any): boolean { return tok.getSourceFile() === d.getSourceFile() && tok.getStart() >= d.getStart() && tok.end <= d.end; }
/** a key of an object literal goes with its last use only where the literal is a knob table or a tool's own local (a kept copy): not a key of data handed elsewhere */
function tableKeyOrLocal(p: any): boolean { let o = p.parent; while (o && o.kind === SK.ObjectLiteralExpression && o.parent.kind === SK.PropertyAssignment) o = o.parent.parent;
  let h = o?.parent; while (h && (h.kind === SK.AsExpression || h.kind === SK.SatisfiesExpression || h.kind === SK.ParenthesizedExpression)) h = h.parent; if (!h || h.kind !== SK.VariableDeclaration || h.name.kind !== SK.Identifier) return false;
  /** does every use of this name that stays go on to name a key of it (`W.a`, `W['a']`)? One read whole (Object.values(W), { ...W }, a loop over it) counts every key it has */
  const keyedOnly = (decl: any): boolean => refsOf(decl).every(r => { const rsf = sfOf(r.fileName); if (!rsf) return false; const tok = ts.getTokenAtPosition(rsf, r.start); if (tok === decl.name || goneAt(r.fileName, r.start)) return true;
    const acc = tok.parent.kind === SK.PropertyAccessExpression && tok.parent.name === tok ? tok.parent : tok, up = upParen(acc), q = up.parent;   // (a key a level down is reached as `W.part`: it is `W.part` that must go on to a key)
    return (q.kind === SK.PropertyAccessExpression && q.expression === up) || (q.kind === SK.ElementAccessExpression && q.expression === up && q.argumentExpression?.kind === SK.StringLiteral); });
  // the object that holds the key directly: a part of a bigger one (`W.part.key`) is read whole wherever `W.part` is; the top one wherever `W` is — except a knob
  // table, which the registry and a recording read whole by design: they read whatever rows it has
  // (every holder on the way up is asked, not the nearest alone: `Object.values(W)` hands out `W.part` whole too)
  for (let holder = p.parent.parent; holder.kind === SK.PropertyAssignment; holder = holder.parent.parent) if (!keyedOnly(holder)) return false;
  if (!REGISTRY.has(h.name.text)) return keyedOnly(h);
  return !namedAsWord(p); }
/** a knob table's key is also reached by its NAME as a word (a panel's list of rows, a recording's keys, a preset): is the key's name a quoted word anywhere in
 *  text that stays? Then the key stays, though nothing that stays reads it as `TABLE.key`. */
function namedAsWord(p: any): boolean { const name = p.name?.text; if (!name || !/^[\w$]+$/.test(name)) return true; const re = new RegExp(`(['"\`])(?:[\\w$]+\\.)*${name.replace(/\$/g, '\\$')}(?:\\.[\\w$]+)*\\1`, 'g');   // (a panel names a row a level down by its path: 'guard.settleSecs' names both keys)
  for (const f of files) { const t = textOf(f); if (!t.includes(name)) continue; re.lastIndex = 0; let m: RegExpExecArray | null; const sf = sfOf(f); if (!sf) continue;
    while ((m = re.exec(t))) { if (goneAt(f, m.index)) continue; const tok = ts.getTokenAtPosition(sf, m.index); if ((tok.kind === SK.StringLiteral || tok.kind === SK.NoSubstitutionTemplateLiteral) && tok.getStart(sf) === m.index) return true; } }
  return false; }

// ---- 5. put each file together: a line whose code is all gone goes whole, with the comment that trailed it
function tidy(t: string): string {
  const lines = t.split('\n'), out: string[] = [];
  for (const line of lines) { if (!line.includes(EMPTY)) { out.push(line); continue; }
    const code = line.replace(new RegExp(` ${EMPTY} `, 'g'), ' ').replace(new RegExp(EMPTY, 'g'), '');
    if (/^\s*(\/\/.*)?$/.test(code)) continue;        // nothing left on the line but the comment that trailed it: the line goes
    out.push(code.replace(/[ \t]+$/, '')); }          // (the line is never cut at a `//`: one may stand inside a string)
  return out.join('\n');
}
const finalText = new Map<string, string>();
for (const [f, t] of rendered) { const n = stillLoads(f, tidy(t)); if (n !== textOf(f)) finalText.set(f, n); }
/** A SECOND LOOK AT THE IMPORTS, on the finished text and by its words alone (the first look, as the file is rendered, goes by what the run knows it
 *  dropped): an import that loaded its module before — one of its names was used as a value — and none of whose names is used as a value in the text as it
 *  now stands gets a bare import in front of it. One too many does nothing: the module loads there either way. */
function stillLoads(f: string, t: string): string { const was = sfOf(f); if (!was) return t; const now = ts.createSourceFile(f, t, ts.ScriptTarget.ES2022, true);
  const loadedBefore = new Set<string>(); for (const st of was.statements) { if (st.kind !== SK.ImportDeclaration || !st.importClause || st.importClause.isTypeOnly) continue; const c = st.importClause, names: any[] = [];
    if (c.name) names.push(c.name); if (c.namedBindings?.kind === SK.NamespaceImport) names.push(c.namedBindings.name); if (c.namedBindings?.kind === SK.NamedImports) for (const el of c.namedBindings.elements) if (!el.isTypeOnly) names.push(el.name);
    if (names.some(nm => valueUses(nm).length > 0)) loadedBefore.add(st.moduleSpecifier.text); }
  if (!loadedBefore.size) return t;
  // every word of the finished text that stands where a value is read (not a type, not a key, not a name being made)
  const used = new Set<string>(); const inType = (x: any): boolean => { for (let y = x.parent; y; y = y.parent) { if (y.kind === SK.ExpressionWithTypeArguments) return !(y.parent.kind === SK.HeritageClause && y.parent.token === SK.ExtendsKeyword && (y.parent.parent.kind === SK.ClassDeclaration || y.parent.parent.kind === SK.ClassExpression)); if (ts.isTypeNode(y) || y.kind === SK.InterfaceDeclaration || y.kind === SK.TypeAliasDeclaration) return true; } return false; };
  const visit = (n: any) => { if (n.kind === SK.ImportDeclaration) return; if (n.kind === SK.Identifier) { const p = n.parent;
      const isKey = (p.kind === SK.PropertyAccessExpression && p.name === n) || ((p.kind === SK.PropertyAssignment || p.kind === SK.MethodDeclaration || p.kind === SK.PropertyDeclaration || p.kind === SK.PropertySignature || p.kind === SK.GetAccessor || p.kind === SK.SetAccessor || p.kind === SK.EnumMember) && p.name === n);
      const isDecl = (p.kind === SK.VariableDeclaration || p.kind === SK.Parameter || p.kind === SK.BindingElement || p.kind === SK.FunctionDeclaration || p.kind === SK.ClassDeclaration) && p.name === n;
      if (!isKey && !isDecl && !inType(n)) used.add(n.text); }
    n.forEachChild(visit); }; visit(now);
  const adds: { at: number; text: string }[] = [], bareNow = new Set<string>(now.statements.filter((st: any) => st.kind === SK.ImportDeclaration && !st.importClause).map((st: any) => st.moduleSpecifier.text));
  const seenMod = new Map<string, any[]>(); for (const st of now.statements) if (st.kind === SK.ImportDeclaration && st.importClause && !st.importClause.isTypeOnly) seenMod.set(st.moduleSpecifier.text, [...(seenMod.get(st.moduleSpecifier.text) ?? []), st]);
  for (const [mod, sts] of seenMod) { if (!loadedBefore.has(mod) || bareNow.has(mod)) continue; const names: string[] = [];
    for (const st of sts) { const c = st.importClause; if (c.name) names.push(c.name.text); if (c.namedBindings?.kind === SK.NamespaceImport) names.push(c.namedBindings.name.text); if (c.namedBindings?.kind === SK.NamedImports) for (const el of c.namedBindings.elements) if (!el.isTypeOnly) names.push(el.name.text); }
    if (!names.some(nm => used.has(nm))) adds.push({ at: sts[0].getStart(now), text: `import ${sts[0].moduleSpecifier.getText(now)}; ` }); }
  for (const a of adds.sort((x, y) => y.at - x.at)) t = t.slice(0, a.at) + a.text + t.slice(a.at);
  return t; }

// ---- 6. the lists that name a row or a word as text: FROZEN_HOLDS, STAND_ENV, the lists of address words
const cur = (f: string) => finalText.get(f) ?? textOf(f);
const PROFILE = join(ROOT, 'src/host/profile.ts'), STANDENV = join(ROOT, 'src/host/standEnv.ts');
if (existsSync(PROFILE)) { const whole = cur(PROFILE), from = whole.indexOf('export const FROZEN_HOLDS'); let t = from < 0 ? '' : whole.slice(from);   // (the list's own lines, nothing above it)
  for (const r of report.retired) { const re = new RegExp(`^[ \\t]*'${r.replace(/\./g, '\\.')}':.*\\n`, 'm'); if (re.test(t)) t = t.replace(re, ''); }
  if (from >= 0 && t !== whole.slice(from)) finalText.set(PROFILE, whole.slice(0, from) + t); }
if (existsSync(PROFILE) && existsSync(STANDENV)) { // a stand word goes when no hold reads it (`pinned('W')`, the environment's W) and no FROZEN_HOLDS line names it; a comment that names it keeps nothing
  const body = files.map(f => f === STANDENV ? '' : cur(f)).join('\n'), bodyWas = files.map(f => textOf(f)).join('\n'); let t = cur(STANDENV); const prof = cur(PROFILE), holds = prof.slice(prof.indexOf('export const FROZEN_HOLDS'));
  for (const w of [...t.slice(t.indexOf('export const STAND_ENV')).matchAll(/'([A-Z][A-Z0-9_]+)'/g)].map(m => m[1])) {
    if (new RegExp(`pinned\\('${w}'\\)|env(?:Of\\(\\))?\\??\\.${w}\\b|\\[['"\`]${w}['"\`]\\]|\\bE\\.${w}\\b`).test(body) || new RegExp(`['"\`][^\\n]*\\b${w}\\b`).test(holds)) continue;
    const was = new RegExp(`pinned\\('${w}'\\)`).test(bodyWas); if (!was) continue;   // (only a word whose hold this run took out)
    t = t.replace(new RegExp(`'${w}',\\s?|,\\s?'${w}'(?=\\s*\\])`), ''); report.standWords.push(w); }
  if (t !== cur(STANDENV)) finalText.set(STANDENV, t); }
{ // an address word that was read before this run and is read by nothing after it: out of every array of words under src
  const readsOf = (t: string) => new Set([...t.matchAll(/(?:\b(?:q|Q|qs)|\))\.(?:get|has)\(\s*'([A-Za-z0-9_]+)'\s*\)/g)].map(m => m[1]));
  const before = new Set<string>(), after = new Set<string>(); for (const f of files) { if (isTool(f)) continue; for (const w of readsOf(textOf(f))) before.add(w); for (const w of readsOf(cur(f))) after.add(w); }
  const gone = [...before].filter(w => !after.has(w)); report.words = gone;
  // (a list of address words is a list most of whose words the page reads from its address: 'response' in a level's list of keys, 'flinch' in a list of acts, are other words)
  const goneSet = new Set(gone), wordList = (a: any) => { const ws = a.elements.filter((x: any) => x.kind === SK.StringLiteral); return ws.length >= 3 && ws.filter((x: any) => before.has(x.text)).length >= 0.8 * ws.length; };
  if (gone.length) for (const f of files) { if (isTool(f)) continue; let t = cur(f); if (!gone.some(w => t.includes(`'${w}'`))) continue; const sfNow = ts.createSourceFile(f, t, ts.ScriptTarget.ES2022, true); const cuts: Span[] = [];
    const visit = (n: any) => { if (n.kind === SK.StringLiteral && goneSet.has(n.text)) { if (n.parent.kind === SK.ArrayLiteralExpression && wordList(n.parent)) { let s = n.getStart(sfNow), e = n.end; const rest = /^\s*,[ \t]?/.exec(t.slice(e)); if (rest) e += rest[0].length; else { const pre = /,\s*$/.exec(t.slice(0, s)); if (pre && !cuts.some(c => c.end === s)) s -= pre[0].length; } cuts.push({ start: s, end: e }); }
        else note(`${rel(f)}:${sfNow.getLineAndCharacterOfPosition(n.getStart(sfNow)).line + 1}: the word '${n.text}' is named here and read from the address nowhere any more`); }
      n.forEachChild(visit); }; visit(sfNow);
    for (const c of cuts.sort((a, b) => b.start - a.start)) t = t.slice(0, c.start) + t.slice(c.end);
    if (cuts.length) finalText.set(f, t); } }

// ---- a constant the fold could not take out of a condition (a call it cannot prove harmless stands beside it): said, for a hand
const leftovers: string[] = []; { const re = /\|\| true\b|&& false\b|\btrue \|\| |\bfalse && |\(true\)|\(false\)|\bswitch \((?:'[^']*'|-?\d+(?:\.\d+)?)\)/g;
  for (const [f, t] of finalText) { const was = (textOf(f).match(re) ?? []).length, now = (t.match(re) ?? []).length; if (now > was) { const ls = t.split('\n'); ls.forEach((l, i) => { if (re.test(l) && !textOf(f).includes(l)) leftovers.push(`${rel(f)}:${i + 1}`); re.lastIndex = 0; }); } } }

// ---- 7. say it, and write it
report.files = finalText.size;
const byKind = (k: string) => report.toolWrites.filter(w => w.kind === k);
console.log(`retire: ${report.retired.length} of ${rows.length} rows · ${finalText.size} files change · ${report.dead.length} declarations go with the dropped branches`);
for (const r of report.refused) console.log(`  REFUSED ${r.row}: ${r.why}`);
console.log(`  address words gone: ${report.words.join(' ') || '(none)'}`);
console.log(`  stand words gone: ${report.standWords.join(' ') || '(none)'}`);
console.log(`  a tool set the row to the value it now always has (taken out): ${byKind('same').length} · put it back from a kept copy or a parameter (taken out; read the check): ${byKind('other').length} · ASKED FOR THE RETIRED PATH (the arm that follows wants a hand): ${byKind('opposite').length}`);
if (leftovers.length) console.log(`  a constant left in a condition beside a call (wants a hand): ${leftovers.length}${VERBOSE ? ' · ' + leftovers.join(' ') : ''}`);
if (droppedLines.length || dropMissed.length) console.log(`  check lines taken out by name: ${droppedLines.length}${dropMissed.length ? ` · NOT FOUND (want a hand): ${dropMissed.length} · ${dropMissed.slice(0, 12).join(' ')}` : ''}`);
{ const live3 = new Set(report.retired), c = report.controls.filter(l => live3.has(l.split(' ')[1].replace(/:$/, ''))); if (c.length) console.log(`  a lab's control for the row, kept as data ({ get, set } in a list: its entry is taken out): ${c.length}${VERBOSE ? '\n    ' + c.join('\n    ') : ''}`); }
if (report.hooks.length) console.log(`  debug hooks that lose a getter and a setter (a page tool may use them): ${report.hooks.length}`);
{ const live2 = new Set(report.retired), n = namedAt.filter(l => live2.has(l.split(' ')[1].replace(/:$/, ''))); if (n.length) console.log(`  rows named as data beside their table's name (a panel's list, what a machine keeps: taken out): ${n.length}${VERBOSE ? '\n    ' + n.join('\n    ') : ''}`); }
if (unread.size) console.log(`  a value nothing reads any more whose making still happens (the name went, the call stays: read each): ${unread.size}${VERBOSE ? '\n    ' + [...unread.values()].join('\n    ') : ''}`);
if (emptied.size) console.log(`  a loop left with nothing in it (kept, for its head: wants a hand): ${emptied.size}${VERBOSE ? ' · ' + [...emptied].join(' ') : ''}`);
console.log(`  a check's line that only read the switch (taken out; the lines that fed it want a hand): ${constLines.size}`);
if (VERBOSE) { const badG = [...namedThings().getters].filter(([, ds]) => ds.some(d => !isTool(d.getSourceFile().fileName) && writesFn(d))).map(([k]) => k);
  console.log(`    properties whose read is never left out (a getter of that name in the game writes something): ${badG.join(' ') || '(none)'}`); }
for (const n of report.notes) if (n.startsWith('--late ')) console.log(`  ${n}`);   // (what was taken on the caller's word is always said)
if (VERBOSE) { for (const h of report.hooks) console.log(`    hook ${h}`); for (const c of constLines) console.log(`    line ${c}`); for (const w of byKind('opposite')) console.log(`    opposite ${w.at} ${w.row}: ${w.text}`); for (const w of byKind('other')) console.log(`    other ${w.at} ${w.row}: ${w.text}`); for (const d of report.dead) console.log(`    dead ${d}`); for (const n of report.notes) if (!n.startsWith('--late ')) console.log(`    note ${n}`); }
{ const out = arg('--report'); if (out) writeFileSync(out, JSON.stringify({ ...report, constLines: [...constLines], leftovers, droppedLines, dropMissed, named: namedAt.filter(l => report.retired.includes(l.split(' ')[1].replace(/:$/, ''))), unread: [...unread.values()], emptied: [...emptied] }, null, 1)); }
if (flag('--list')) for (const r of report.retired) console.log(`- \`${r}\``);
if (WRITE) { for (const [f, t] of finalText) writeFileSync(f, t); console.log(`  written: ${finalText.size} files`); } else console.log('  (a dry run: --write writes it)');
process.exit(report.refused.length ? 3 : 0);   // (3 = some rows were refused and the rest done: a script that asked for all of them should not read that as done)
