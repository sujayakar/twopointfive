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
// the game holds at another value, or that the game reads through a list of keys, and says why (exit 3: the other rows are still done).
// It does not touch docs: `--list` prints the retired rows as lines for a write-up.
//
//   bun run tools/retire/retire.ts ORDERS.fileSpread=true NOSHOVE.on=true          a dry run: the report, nothing written
//   bun run tools/retire/retire.ts --rows rows.json --write [--report out.json]    rows.json = [{ "row": "ORDERS.fileSpread", "value": true }, …]
//   … --pure locks,allowed      functions the caller vouches only read, so a condition that calls one beside a retired row can go (see VOUCHED)
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
const rows: Row[] = [];
{ const f = arg('--rows'); if (f) for (const r of JSON.parse(readFileSync(f, 'utf8')) as Row[]) rows.push(r);
  for (const a of argv) { const m = /^([A-Za-z_][\w.]*)=(.*)$/.exec(a); if (!m || a.startsWith('--')) continue; const raw = m[2], q = /^'(.*)'$|^"(.*)"$/.exec(raw);
    if (raw !== 'true' && raw !== 'false' && !(raw.trim() !== '' && Number.isFinite(Number(raw))) && !q) { console.error(`${a}: a value is true, false, a number or a 'quoted word' (a row asked for by a typo must not be folded to the wrong branch)`); process.exit(2); }
    rows.push({ row: m[1], value: raw === 'true' ? true : raw === 'false' ? false : q ? q[1] ?? q[2] : Number(raw) }); } }
const DROPS: string[] = [], DROP_AT: string[] = []; for (let i = 0; i < argv.length; i++) { if (argv[i] === '--drop' && argv[i + 1]) DROPS.push(argv[i + 1]); if (argv[i] === '--drop-at' && argv[i + 1]) DROP_AT.push(argv[i + 1]); }
{ const f = arg('--drop-lines'); if (f) for (const l of JSON.parse(readFileSync(f, 'utf8')) as string[]) DROP_AT.push(l); }
if (!rows.length && !DROPS.length && !DROP_AT.length) { console.error('no rows: ROW=value … or --rows file.json (or --drop file.ts:name …)'); process.exit(2); }

// ---- the program: src, tools and the types, one language service over all of it (a row's uses in a check are found with its uses in the game)
function walk(dir: string, out: string[]): string[] {
  for (const n of readdirSync(dir)) { if (n === 'node_modules' || n === 'vendor' || n.startsWith('.')) continue; const p = join(dir, n), st = statSync(p); if (p === join(ROOT, 'tools/retire/fixture')) continue;   // (the check's own small tree is not this program's)
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
type FilePlan = { subs: Map<any, Value>; del: Set<any>; unassign: Set<any>; bare: Set<any>; dropped: Span[] };   // bare: a `const x = f();` nothing reads any more — f() still runs, only the name goes
const plans = new Map<string, FilePlan>();
const plan = (f: string) => { let p = plans.get(f); if (!p) { p = { subs: new Map(), del: new Set(), unassign: new Set(), bare: new Set(), dropped: [] }; plans.set(f, p); } return p; };
const report = { retired: [] as string[], refused: [] as { row: string; why: string }[], notes: [] as string[], toolWrites: [] as { row: string; at: string; kind: string; text: string }[],
  dead: [] as string[], words: [] as string[], standWords: [] as string[], hooks: [] as string[], files: 0 };
const note = (s: string) => { report.notes.push(s); };

// ---- small readings of the tree
const unparen = (n: any): any => { while (n && (n.kind === SK.ParenthesizedExpression || n.kind === SK.AsExpression || n.kind === SK.NonNullExpression || n.kind === SK.SatisfiesExpression)) n = n.expression; return n; };
const upParen = (n: any): any => { while (n.parent && n.parent.kind === SK.ParenthesizedExpression) n = n.parent; return n; };
const PURE_CALLS = /^(pinned|envOf|String|Number|Boolean|Number\.isFinite|q\.get|q\.has|Q\.get|Q\.has|qs\.get|qs\.has)$/;
const PURE_METHODS = new Set(['has', 'get', 'includes', 'indexOf', 'startsWith', 'endsWith', 'test', 'at', 'slice', 'some', 'every', 'find', 'findIndex', 'filter', 'map', 'join', 'keys', 'values', 'entries', 'toFixed', 'toString', 'trim', 'toLowerCase', 'toUpperCase', 'abs', 'min', 'max', 'floor', 'ceil', 'round', 'sign', 'hypot', 'sqrt', 'isArray', 'isFinite']);
const CALLBACK_METHODS = new Set(['some', 'every', 'find', 'findIndex', 'filter', 'map']);   // (the reading methods above that CALL what they are handed)
let pureDepth = 0;
/** a function whose own body only reads: an expression that does, or consts that only read and then a return that does (`const lk = (k) => locks.has(k)`) */
function pureFn(f: any): boolean { f = unparen(f); if (!f || pureDepth >= 4) return false;
  if (f.kind === SK.Identifier) { if (/^(Boolean|String|Number)$/.test(f.text)) return true; const d = checker.getSymbolAtLocation(f)?.valueDeclaration;
    const g = d?.kind === SK.VariableDeclaration || d?.kind === SK.PropertyDeclaration ? unparen(d.initializer) : d?.kind === SK.FunctionDeclaration || d?.kind === SK.MethodDeclaration ? d : null; return !!g && g.kind !== SK.Identifier && pureFn(g); }
  if (!(f.kind === SK.ArrowFunction || f.kind === SK.FunctionExpression || f.kind === SK.FunctionDeclaration || f.kind === SK.MethodDeclaration) || !f.body || f.getSourceFile().isDeclarationFile) return false;
  pureDepth++; try {
    if (f.body.kind !== SK.Block) return pure(f.body);
    const sts: any[] = f.body.statements, last = sts[sts.length - 1];
    return !!last && last.kind === SK.ReturnStatement && pure(last.expression) && sts.slice(0, -1).every(st => st.kind === SK.VariableStatement && !!(st.declarationList.flags & ts.NodeFlags.Const) && st.declarationList.declarations.every((d2: any) => pure(d2.initializer)));
  } finally { pureDepth--; } }
/** a call that only reads: one of the reading methods above (and, where the method calls what it is handed, a function that only reads), or a local function
 *  whose own body only reads */
function pureCall(n: any): boolean { const c = n.expression;
  if (VOUCHED.size && (c.kind === SK.Identifier ? VOUCHED.has(c.text) : c.kind === SK.PropertyAccessExpression && VOUCHED.has(c.name.text))) return (c.kind === SK.Identifier || pure(c.expression)) && n.arguments.every(pure);
  if (c.kind === SK.PropertyAccessExpression && PURE_METHODS.has(c.name.text)) { const calls = CALLBACK_METHODS.has(c.name.text);
    return pure(c.expression) && n.arguments.every((a: any, i: number) => { const u = unparen(a); return (u && ts.isFunctionLike(u)) || (calls && i === 0) ? pureFn(u) : pure(a); }); }
  if (c.kind === SK.Identifier || c.kind === SK.PropertyAccessExpression) { if (c.kind === SK.PropertyAccessExpression && !pure(c.expression)) return false;
    return pureFn(c.kind === SK.Identifier ? c : (() => { const d = checker.getSymbolAtLocation(c.name)?.valueDeclaration; return d?.kind === SK.VariableDeclaration || d?.kind === SK.PropertyDeclaration ? unparen(d.initializer) : d?.kind === SK.FunctionDeclaration || d?.kind === SK.MethodDeclaration ? d : null; })()) && n.arguments.every(pure); }
  return false; }
function pure(n: any): boolean {
  if (!n) return true;
  switch (n.kind) {
    case SK.CallExpression: { if (pureCall(n)) return true; const c = n.expression.getText(); const url = /URLSearchParams\([^()]*\)\.(get|has)$/.test(c) || PURE_CALLS.test(c); return url && n.arguments.every(pure) && (url && !PURE_CALLS.test(c) ? true : pure(n.expression.expression ?? null) || PURE_CALLS.test(c)); }
    case SK.NewExpression: return /^(URLSearchParams|Set|Map|WeakSet|WeakMap|Array)$/.test(n.expression.getText()) && (n.arguments ?? []).every(pure);   // (making an empty or a filled container does nothing else)
    case SK.AwaitExpression: case SK.YieldExpression: case SK.DeleteExpression: case SK.TaggedTemplateExpression: return false;
    case SK.PostfixUnaryExpression: return false;
    case SK.PrefixUnaryExpression: if (n.operator === SK.PlusPlusToken || n.operator === SK.MinusMinusToken) return false; break;
    case SK.BinaryExpression: if (n.operatorToken.kind >= SK.FirstAssignment && n.operatorToken.kind <= SK.LastAssignment) return false; break;
    case SK.ArrowFunction: case SK.FunctionExpression: case SK.MethodDeclaration: case SK.GetAccessor: case SK.SetAccessor: case SK.FunctionDeclaration: return true;   // (MAKING a function does nothing: calling it is pureCall's question)
  }
  let ok = true; n.forEachChild((c: any) => { if (ok && !pure(c)) ok = false; }); return ok;
}
const lit = (v: Value) => typeof v === 'string' ? JSON.stringify(v).replace(/^"|"$/g, "'") : String(v);
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
type Use = { row: Row; node: any; sf: any; write: boolean; def: boolean };
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
for (const r of rows) {
  const parts = r.row.split('.');
  if (parts[0] === 'tune') { report.refused.push({ row: r.row, why: 'a row of Game.tune: not a knob table (retire it by hand)' }); continue; }
  const decls = tableDecl(parts[0]);
  if (decls.length !== 1) { report.refused.push({ row: r.row, why: decls.length ? `${decls.length} exported tables named ${parts[0]}` : `no exported table named ${parts[0]} under src` }); continue; }
  let prop: any = null, obj = decls[0].initializer;
  for (const k of parts.slice(1)) { prop = propIn(obj, k); if (!prop) break; obj = prop.initializer; }
  if (!prop) { const o = unparen(decls[0].initializer), acc = o?.kind === SK.ObjectLiteralExpression && o.properties.some((p: any) => (p.kind === SK.GetAccessor || p.kind === SK.SetAccessor) && p.name?.text === parts[1]);
    report.refused.push({ row: r.row, why: acc ? `${parts[0]}.${parts[1]} is a getter and a setter: writing it does something, so it is not a plain switch (${at(decls[0].getSourceFile(), decls[0].getStart())})` : `no key ${parts.slice(1).join('.')} in ${parts[0]}'s literal (${at(decls[0].getSourceFile(), decls[0].getStart())})` }); continue; }
  literal.set(r.row, unparen(prop.initializer)?.getText?.() ?? prop.getText());
  { const L = unparen(prop.initializer), kind = !L ? 'none' : L.kind === SK.TrueKeyword || L.kind === SK.FalseKeyword ? 'boolean' : L.kind === SK.NumericLiteral || (L.kind === SK.PrefixUnaryExpression && L.operand.kind === SK.NumericLiteral) ? 'number' : L.kind === SK.StringLiteral || L.kind === SK.NoSubstitutionTemplateLiteral ? 'string' : 'other';
    if (kind !== typeof r.value) { report.refused.push({ row: r.row, why: kind === 'other' || kind === 'none' ? `its own value is not a plain true, false, number or word (${literal.get(r.row)!.slice(0, 60)}): retire it by hand` : `asked for ${JSON.stringify(r.value)}, and the row is a ${kind} (${literal.get(r.row)})` }); continue; } }
  rowDef.set(r.row, prop);
  const sf = prop.getSourceFile(), refs = service.findReferences(sf.fileName, prop.name.getStart(sf)) ?? [];
  const seen = new Set<string>();
  for (const g of refs) for (const e of g.references) { const k = `${e.fileName}:${e.textSpan.start}`; if (seen.has(k)) continue; seen.add(k);
    const rsf = sfOf(e.fileName); if (!rsf) continue; const node = ts.getTokenAtPosition(rsf, e.textSpan.start);
    uses.push({ row: r, node, sf: rsf, write: !!e.isWriteAccess, def: node === prop.name }); }
}

// (a tool takes the game's tables through `await import(\`${ROOT}/src/…\`)`, which TypeScript reads as `any`: there the row is found by its name — TABLE.key on a
// thing called TABLE, or on `<module>.TABLE` — and only where that thing has no type of its own)
{ const byTable = new Map<string, Row[]>(); for (const r of rows) if (rowDef.has(r.row)) { const t = r.row.split('.')[0]; byTable.set(t, [...(byTable.get(t) ?? []), r]); }
  const seenTok = new Set<any>(uses.map(u => u.node));
  for (const f of files) { const text = textOf(f); let any = false; for (const t of byTable.keys()) if (text.includes(t)) { any = true; break; } if (!any) continue; const sf = sfOf(f); if (!sf) continue;
    // (another name for the table in this file — `const KK = K`, `const KK = M.K`, `const { K: KK } = …` — is the table)
    const alias = new Map<string, string>(); { const names = (n: any) => { if (n.kind === SK.VariableDeclaration && n.name.kind === SK.Identifier && n.initializer) { const i = unparen(n.initializer), t = i?.kind === SK.Identifier ? i.text : i?.kind === SK.PropertyAccessExpression ? i.name.text : null; if (t && byTable.has(t) && n.name.text !== t) alias.set(n.name.text, t); }
        if (n.kind === SK.BindingElement && n.propertyName?.kind === SK.Identifier && n.name.kind === SK.Identifier && byTable.has(n.propertyName.text) && n.name.text !== n.propertyName.text) alias.set(n.name.text, n.propertyName.text);
        n.forEachChild(names); }; names(sf); }
    const visit = (n: any) => { if (n.kind === SK.PropertyAccessExpression && n.name.kind === SK.Identifier) { const chain: string[] = []; let e = n;
          while (e.kind === SK.PropertyAccessExpression) { chain.unshift(e.name.text); e = e.expression; if (byTable.has(chain[0]) && chain.length > 1) break; }
          const root = e.kind === SK.Identifier ? alias.get(e.text) ?? e.text : null, table = root && byTable.has(root) ? root : byTable.has(chain[0]) && chain.length > 1 ? chain[0] : null;
          if (table) { const keys = table === root ? chain : chain.slice(1), path = `${table}.${keys.join('.')}`, r = byTable.get(table)!.find(x => x.row === path);
            if (r && !seenTok.has(n.name)) { let base = n; for (let i = 0; i < keys.length; i++) base = base.expression; const ty = checker.getTypeAtLocation(base);
              if (ty.flags & ts.TypeFlags.Any) { seenTok.add(n.name); const up = upParen(n).parent; uses.push({ row: r, node: n.name, sf, write: up.kind === SK.BinaryExpression && upParen(up.left) === upParen(n) && up.operatorToken.kind === SK.EqualsToken, def: false }); } } } }
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
type Act = { use: Use; kind: 'read' | 'delStmt' | 'delProp' | 'delMember' | 'unassign' | 'const' | 'setInit'; node: any };
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
for (const u of uses) {
  const n = u.node, p = n.parent, where = at(u.sf, n.getStart(u.sf)), tool = isTool(u.sf.fileName);
  if (u.row.hold) {   // a held number: its literal becomes the page's value, the page's and the stand's own writes of it go, every read and every tool is left alone
    if (u.def) { if (p.initializer) acts.push({ use: u, kind: 'setInit', node: p.initializer }); continue; }
    const acc = (p.kind === SK.PropertyAccessExpression && p.name === n) ? upParen(p) : null, as = acc?.parent;
    if (as && as.kind === SK.BinaryExpression && as.left === acc && as.operatorToken.kind === SK.EqualsToken && /^(src\/host\/(profile|standup)\.ts|tools\/qa\/headless\.ts)$/.test(rel(u.sf.fileName))) acts.push({ use: u, kind: 'unassign', node: as });
    continue; }
  if (u.def) { acts.push({ use: u, kind: 'delProp', node: p }); continue; }
  if (p.kind === SK.PropertySignature && p.name === n) { acts.push({ use: u, kind: 'delMember', node: p }); continue; }
  if ((p.kind === SK.PropertyAssignment || p.kind === SK.ShorthandPropertyAssignment) && p.name === n) {   // the row written through an object (a preset, an Object.assign)
    const i = p.kind === SK.PropertyAssignment ? unparen(p.initializer) : null, v = !i ? undefined : i.kind === SK.TrueKeyword ? true : i.kind === SK.FalseKeyword ? false : i.kind === SK.NumericLiteral ? Number(i.text) : i.kind === SK.StringLiteral ? i.text : undefined;
    if (!tool && v !== u.row.value) { refuse.set(u.row.row, `${where}: an object literal holds another value for it than the page's (${p.getText().slice(0, 60)}): wherever that object is put over the table, the game writes the row`); continue; }
    acts.push({ use: u, kind: 'delProp', node: p }); note(`${where}: ${u.row.row} set through an object literal: the key is taken out (${p.getText().slice(0, 60)})`); continue; }
  const access = (p.kind === SK.PropertyAccessExpression && p.name === n) || (p.kind === SK.ElementAccessExpression && p.argumentExpression === n) ? p : null;
  if (!access) { refuse.set(u.row.row, `${where}: a use this tool does not read (${SK[p.kind]}: ${p.getText().slice(0, 80)})`); continue; }
  const outer = upParen(access), op = outer.parent;
  if (op.kind === SK.DeleteExpression) {   // (`delete file.tables.T.key`: a check making a file from before the key — the key is gone, so is the line)
    if (op.parent.kind === SK.ExpressionStatement) acts.push({ use: u, kind: 'delStmt', node: op.parent }); else acts.push({ use: u, kind: 'const', node: op });
    report.toolWrites.push({ row: u.row.row, at: where, kind: 'other', text: (stmtOf(op) ?? op).getText().slice(0, 100) }); continue; }
  if (op.kind === SK.BinaryExpression && op.left === outer && op.operatorToken.kind >= SK.FirstAssignment && op.operatorToken.kind <= SK.LastAssignment) {
    if (op.operatorToken.kind !== SK.EqualsToken) { refuse.set(u.row.row, `${where}: a compound write (${op.getText().slice(0, 80)})`); continue; }
    const file = rel(u.sf.fileName), fn = ts.findAncestor(op, (x: any) => ts.isFunctionLike(x)), rhs = unparen(op.right), shown = (stmtOf(op) ?? op).getText().slice(0, 100);
    const side = tool || /^src\/(viewer|editor|workbench)/.test(file);   // (a tool, or the workbench: neither is the game)
    // the debugger's own toggle for this row: a call that is a statement, handed a function that writes the row and another that reads it → the whole panel row goes
    const callSt = fn && (fn.kind === SK.ArrowFunction || fn.kind === SK.FunctionExpression) && fn.parent.kind === SK.CallExpression && fn.parent.parent.kind === SK.ExpressionStatement ? fn.parent.parent : null;
    if (callSt && !side && !/^src\/ui\/options/.test(file) && fn.parent.arguments.some((a: any) => a !== fn && ts.isFunctionLike(a) && readIn(a, u.row.row))) { acts.push({ use: u, kind: 'delStmt', node: callSt }); continue; }
    // a debug hook's setter (`set on(v) { ROW = !!v; }` in an object handed to the window): the setter and its getter go, and the hook is said (a page tool may use it)
    if (fn && fn.kind === SK.SetAccessor && fn.parent.kind === SK.ObjectLiteralExpression && !side) { acts.push({ use: u, kind: 'delProp', node: fn });
      const g = fn.parent.properties.find((q: any) => q.kind === SK.GetAccessor && q.name.getText() === fn.name.getText()); if (g) acts.push({ use: u, kind: 'delProp', node: g });
      report.hooks.push(`${where}: ${(fn.parent.parent.getText() as string).slice(0, 50).replace(/\s+/g, ' ')} … .${fn.name.getText()}`); continue; }
    if (side) { const v = rhs.kind === SK.TrueKeyword ? true : rhs.kind === SK.FalseKeyword ? false : undefined;
      report.toolWrites.push({ row: u.row.row, at: where, kind: v === undefined ? 'other' : v === u.row.value ? 'same' : 'opposite', text: shown }); acts.push({ use: u, kind: 'unassign', node: op }); continue; }
    if (/^src\/ui\/options/.test(file)) { refuse.set(u.row.row, `${where}: Options writes it: the player owns this row`); continue; }
    if (file === 'src/ui/bindings.ts' || bootWrite(op)) { acts.push({ use: u, kind: 'unassign', node: op }); continue; }   // (the debugger's panel putting it down beside another row, the page's address, the stand's hold, the journal's line for an old file)
    refuse.set(u.row.row, `${where}: the game writes it as it runs: state, not a switch (${shown})`); continue;
  }
  if ((op.kind === SK.PrefixUnaryExpression || op.kind === SK.PostfixUnaryExpression) && (op.operator === SK.PlusPlusToken || op.operator === SK.MinusMinusToken)) { refuse.set(u.row.row, `${where}: ++ or -- on the row`); continue; }
  // the debugger's getter: `() => ROW` handed to a call that is a statement and that also holds the setter → the same statement goes (found from the setter); a lone getter folds
  acts.push({ use: u, kind: 'read', node: access });
}
for (const [k, v] of twinWhy) refuse.set(k, `the Rust goal's: ${v}`);
// ---- a row reached by a KEY THAT IS NOT WRITTEN OUT (`TABLE[k]`, k walking a list of words), and a row NAMED AS DATA beside its table's name (the registry's
// name for a table is how a panel's list of rows, or what a machine keeps of it, says which table: `rows('TABLE', [{ k: 'key', … }])`, `TABLE: { key: … }`,
// `Object.assign(KEPT.TABLE, { key: … })`). Typed code that names a key some other way stops compiling when the row goes, and both typechecks are the proof's.
const keyLists: { el: any; row: string }[] = [], named: { node: any; row: string }[] = []; const namedAt: string[] = [];
{ const byTable = new Map<string, Row[]>(); for (const r of rows) if (rowDef.has(r.row) && !r.hold && r.row.split('.').length === 2) { const t = r.row.split('.')[0]; byTable.set(t, [...(byTable.get(t) ?? []), r]); }
  const rowOf = (t: string, k: string) => byTable.get(t)?.find(x => x.row === `${t}.${k}`) ?? null;
  const strList = (e: any, depth = 0): any => { e = unparen(e); if (!e) return null;
    if (e.kind === SK.ArrayLiteralExpression) return e.elements.length && e.elements.every((x: any) => x.kind === SK.StringLiteral) ? e : null;
    if (e.kind === SK.Identifier && depth < 3) { const d = checker.getSymbolAtLocation(e)?.valueDeclaration; if (d?.kind === SK.VariableDeclaration && d.initializer) return strList(d.initializer, depth + 1); }
    return null; };
  /** the list of words a key walks — `for (const k of LIST)`, `LIST.forEach(k => …)` and the like — or null */
  const keySource = (arg: any): any => { arg = unparen(arg); if (arg?.kind !== SK.Identifier) return null; const d = checker.getSymbolAtLocation(arg)?.valueDeclaration; if (!d) return null;
    if (d.kind === SK.VariableDeclaration && d.parent.parent?.kind === SK.ForOfStatement) return strList(d.parent.parent.expression);
    if (d.kind === SK.Parameter && ts.isFunctionLike(d.parent) && d.parent.parameters[0] === d) { const call = upParen(d.parent).parent;
      if (call?.kind === SK.CallExpression && unparen(call.arguments[0]) === d.parent && call.expression.kind === SK.PropertyAccessExpression && /^(forEach|map|filter|some|every|find|findIndex)$/.test(call.expression.name.text)) return strList(call.expression.expression); }
    return null; };
  /** is this expression THE TABLE (its own name, an import of it, or — in a tool, where it came through an untyped import — a thing of that name)? A property
   *  of other data that is merely called the same (`recording.tables.TABLE`) is not. */
  const declOf = new Map<string, any>(); for (const t of byTable.keys()) declOf.set(t, tableDecl(t)[0]);
  const tableOf = (x: any): string | null => { x = unparen(x); const nm = x?.kind === SK.Identifier ? x : x?.kind === SK.PropertyAccessExpression ? x.name : null; if (!nm || !byTable.has(nm.text)) return null;
    let sym = checker.getSymbolAtLocation(nm); if (sym && (sym.flags & ts.SymbolFlags.Alias)) sym = checker.getAliasedSymbol(sym);
    return sym?.valueDeclaration === declOf.get(nm.text) || (checker.getTypeAtLocation(x).flags & ts.TypeFlags.Any) ? nm.text : null; };
  const calledAs = (x: any): string | null => { x = unparen(x); return x?.kind === SK.PropertyAccessExpression && byTable.has(x.name.text) ? x.name.text : null; };
  const keyName = (p: any): string | null => (p.kind === SK.PropertyAssignment || p.kind === SK.ShorthandPropertyAssignment) && p.name && (p.name.kind === SK.Identifier || p.name.kind === SK.StringLiteral) ? p.name.text : null;
  for (const f of files) { const text = textOf(f); if (![...byTable.keys()].some(t => text.includes(t))) continue; const sf = sfOf(f); if (!sf) continue; const tool = isTool(f);
    const say = (n: any, row: string) => { named.push({ node: n, row }); namedAt.push(`${at(sf, n.getStart(sf))} ${row}: ${(n.getText() as string).slice(0, 70).replace(/\s+/g, ' ')}`); };
    const visit = (n: any) => {
      if (n.kind === SK.ElementAccessExpression && n.argumentExpression && n.argumentExpression.kind !== SK.StringLiteral && n.argumentExpression.kind !== SK.NumericLiteral) { const t = tableOf(n.expression);
        if (t) { const list = keySource(n.argumentExpression), outer = upParen(n), op = outer.parent, write = op.kind === SK.BinaryExpression && op.left === outer && op.operatorToken.kind === SK.EqualsToken, where = at(sf, n.getStart(sf)), shown = ((stmtOf(n) ?? n).getText() as string).slice(0, 100);
          if (list) { for (const e of list.elements) { const r = rowOf(t, e.text); if (!r) continue;
              if (!tool && !write) refuse.set(r.row, `${where}: read through a list of keys, and a list without the key may not be the same game (${shown}): fold that by hand first`);
              else { keyLists.push({ el: e, row: r.row }); if (tool) report.toolWrites.push({ row: r.row, at: where, kind: 'other', text: shown }); } } }
          else if (!tool) note(`${where}: ${t} is ${write ? 'written' : 'read'} by a key worked out as it runs (${shown}): a retired row that can be named there wants a hand`); } }
      if (n.kind === SK.CallExpression) {
        // rows('TABLE', [{ k: 'key', … }, …]): a list of the table's rows by name
        n.arguments.forEach((a: any, i: number) => { if (a.kind !== SK.StringLiteral || !byTable.has(a.text)) return; for (const b of n.arguments.slice(i + 1)) { const arr = unparen(b); if (arr?.kind !== SK.ArrayLiteralExpression) continue;
            for (const el of arr.elements) { const o = unparen(el), k = o?.kind === SK.StringLiteral ? o.text : o?.kind === SK.ObjectLiteralExpression ? o.properties.map((q: any) => q.kind === SK.PropertyAssignment && unparen(q.initializer)?.kind === SK.StringLiteral ? unparen(q.initializer).text : null).find((w: string | null) => w !== null && !!rowOf(a.text, w)) : null; const r = k ? rowOf(a.text, k) : null; if (r) say(el, r.row); } } });
        // Object.assign(TABLE, { key: … }) writes the row; Object.assign(<something>.TABLE, { key: … }) names it
        if (n.expression.getText() === 'Object.assign' && n.arguments.length >= 2) { const tgt = unparen(n.arguments[0]), real = tableOf(tgt), t = real ?? calledAs(tgt);
          if (t) for (const b of n.arguments.slice(1)) { const o = unparen(b); if (o?.kind !== SK.ObjectLiteralExpression) continue; for (const q of o.properties) { const k = keyName(q), r = k ? rowOf(t, k) : null; if (!r) continue;
                if (real && !tool && !bootWrite({ getSourceFile: () => sf, right: q, parent: n.parent })) refuse.set(r.row, `${at(sf, q.getStart(sf))}: the game writes it as it runs: state, not a switch (${n.getText().slice(0, 80)})`);
                else { say(q, r.row); if (tool) report.toolWrites.push({ row: r.row, at: at(sf, q.getStart(sf)), kind: 'other', text: n.getText().slice(0, 100) }); } } } } }
      // TABLE: { key: …, … } in a literal: what is kept, shown or allowed of the table, by name
      if (!tool && n.kind === SK.PropertyAssignment && n.name && (n.name.kind === SK.Identifier || n.name.kind === SK.StringLiteral) && byTable.has(n.name.text)) { const o = unparen(n.initializer);
        if (o?.kind === SK.ObjectLiteralExpression) for (const q of o.properties) { const k = keyName(q), r = k ? rowOf(n.name.text, k) : null; if (r) say(q, r.row); } }
      n.forEachChild(visit); }; visit(sf); } }
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
  if (a.kind === 'read' || a.kind === 'setInit') P.subs.set(a.node, a.use.row.value); else if (a.kind === 'const') P.subs.set(a.node, true); else if (a.kind === 'unassign') P.unassign.add(a.node); else P.del.add(a.node); }

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

// ---- --drop-at file.ts:line[:words | :column] …: a check's line to take out (the innermost statement on that line holding those words, or the statement the
// call at that column is in — a stack frame's place): the off arm of a retired switch, found by running the check before and after. What only it used goes by step 4.
for (const spec of DROP_AT) { const m = /^([^:]+):(\d+)(?::(.*))?$/.exec(spec); if (!m) { console.error(`--drop-at ${spec}: file:line[:words]`); process.exit(2); }
  const file = join(ROOT, m[1]), sf = sfOf(file), line = +m[2] - 1, words = m[3]; if (!sf) { console.error(`--drop-at ${spec}: no such file`); process.exit(2); }
  let best: any = null;
  if (words && /^\d+$/.test(words)) {   // file:line:column — the place a stack frame gives for the call that printed the line: the statement that call is in
    const tok = ts.getTokenAtPosition(sf, sf.getPositionOfLineAndCharacter(line, Math.max(0, +words - 1))); best = ts.findAncestor(tok, (x: any) => x.kind === SK.ExpressionStatement) ?? null;
    if (best && sf.getLineAndCharacterOfPosition(best.getStart(sf)).line > line) best = null; }
  else { const visit = (n: any) => { const a = sf.getLineAndCharacterOfPosition(n.getStart(sf)).line, b = sf.getLineAndCharacterOfPosition(n.end).line; if (a > line || b < line) return;
      if (n.kind === SK.ExpressionStatement && (!words || (n.getText() as string).includes(words))) best = n; n.forEachChild(visit); }; visit(sf); }
  if (!best) { note(`--drop-at ${spec}: no statement there (a line that is returned or kept in a name wants a hand)`); dropMissed.push(spec); continue; }
  plan(file).del.add(best); droppedLines.push(`${m[1]}:${m[2]} ${(best.getText() as string).slice(0, 120)}`); }

// ---- 3. fold. `konst` = the value an expression is known to have once the rows are their values; `render` = its text after the fold.
const constLines: string[] = []; const emptied = new Set<string>();   // (a loop left with nothing in it: said, for a hand)
const localConst = new Map<any, Value>();   // a `const x = <now a constant>` declaration → its value (its uses fold too, and it goes)
function konst(n: any, P: FilePlan): { v: Value } | null {
  if (!n) return null;
  if (P.subs.has(n)) return { v: P.subs.get(n)! };
  switch (n.kind) {
    case SK.ParenthesizedExpression: return konst(n.expression, P);
    case SK.TrueKeyword: return { v: true }; case SK.FalseKeyword: return { v: false };
    case SK.PrefixUnaryExpression: if (n.operator === SK.ExclamationToken) { const c = konst(n.operand, P); return c ? { v: !c.v } : null; } return null;
    case SK.Identifier: { if (!touched(n, P)) return null; const s = checker.getSymbolAtLocation(n), d = s?.valueDeclaration; return d && localConst.has(d) && n !== d.name ? { v: localConst.get(d)! } : null; }
    case SK.BinaryExpression: { const k = n.operatorToken.kind;
      if (k === SK.AmpersandAmpersandToken) { const l = konst(n.left, P); if (l && !l.v) return { v: l.v }; const r = konst(n.right, P); if (l && r) return { v: r.v }; if (r && !r.v && pure(n.left) && (boolCtx(n) || isBoolType(n.left))) return { v: false }; return null; }
      if (k === SK.BarBarToken) { const l = konst(n.left, P); if (l && l.v) return { v: l.v }; const r = konst(n.right, P); if (l && r) return { v: r.v }; if (r && r.v === true && pure(n.left) && (boolCtx(n) || isBoolType(n.left))) return { v: true }; return null; }
      if (k === SK.QuestionQuestionToken) { const l = konst(n.left, P); return l ? { v: l.v } : null; }
      if (k === SK.EqualsEqualsEqualsToken || k === SK.ExclamationEqualsEqualsToken) { const l = konst(n.left, P), r = konst(n.right, P); return l && r ? { v: (l.v === r.v) === (k === SK.EqualsEqualsEqualsToken) } : null; }
      return null; }
    case SK.ConditionalExpression: { const c = konst(n.condition, P); return c ? konst(c.v ? n.whenTrue : n.whenFalse, P) : null; }
  }
  return null;
}
/** does this node hold anything the run changes? (most of a file does not: its text is copied) */
const marks = new Map<string, number[]>();
function touched(n: any, P: FilePlan): boolean { const sf = n.getSourceFile(), m = marks.get(sf.fileName); if (!m) return false; const a = n.pos, b = n.end;
  let lo = 0, hi = m.length; while (lo < hi) { const mid = (lo + hi) >> 1; if (m[mid] < a) lo = mid + 1; else hi = mid; } return lo < m.length && m[lo] < b; }
function remark(): void { marks.clear(); for (const [f, P] of plans) { const m: number[] = []; for (const n of P.subs.keys()) m.push(n.getStart()); for (const n of P.del) m.push(n.getStart()); for (const n of P.unassign) m.push(n.getStart()); for (const n of P.bare) m.push(n.getStart()); m.sort((x, y) => x - y); marks.set(f, m); } }
const drop = (n: any, P: FilePlan) => { P.dropped.push({ start: n.getStart(), end: n.end }); };
const hasLexical = (b: any) => b.statements.some((s: any) => (s.kind === SK.VariableStatement && (s.declarationList.flags & (ts.NodeFlags.Let | ts.NodeFlags.Const))) || s.kind === SK.ClassDeclaration || s.kind === SK.FunctionDeclaration);
const multiOK = (st: any) => { const p = st.parent; return p && (p.kind === SK.Block || p.kind === SK.SourceFile || p.kind === SK.ModuleBlock || p.kind === SK.CaseClause || p.kind === SK.DefaultClause); };
const EMPTY = '\u0000';   // a statement that is gone (its line is tidied when the file is put together)
function render(n: any, P: FilePlan): string {
  const sf = n.getSourceFile(), text = sf.text as string;
  if (P.del.has(n)) { drop(n, P); return EMPTY; }
  if (!touched(n, P)) return text.slice(n.getStart(sf), n.end);
  if (ts.isExpression(n) || n.kind === SK.Identifier) { const c = konst(n, P); if (c && n.kind !== SK.TrueKeyword && n.kind !== SK.FalseKeyword) { drop(n, P); const t = lit(c.v), p = n.parent;
      // (a number where a name stood: `3.toFixed()` and `a--1` are not what was meant)
      return typeof c.v === 'number' && ((p.kind === SK.PropertyAccessExpression && p.expression === n) || (c.v < 0 && (p.kind === SK.BinaryExpression || p.kind === SK.PrefixUnaryExpression || p.kind === SK.PostfixUnaryExpression))) ? `(${t})` : t; } }
  switch (n.kind) {
    case SK.BinaryExpression: { const k = n.operatorToken.kind;
      if (P.unassign.has(n)) { drop(n.left, P); return render(n.right, P); }   // (`ROW = x` inside a chain or an expression: what is left is x)
      if (k === SK.AmpersandAmpersandToken || k === SK.BarBarToken) { const and = k === SK.AmpersandAmpersandToken, l = konst(n.left, P), r = konst(n.right, P);
        if (l && !!l.v === and) return render(n.right, P);                                                    // true && x → x · false || x → x
        if (r && !!r.v === and && (boolCtx(n) || isBoolType(n.left))) return render(n.left, P); }             // x && true → x · x || false → x (where only its truth is read)
      break; }
    case SK.ConditionalExpression: { const c = konst(n.condition, P); if (c) { drop(c.v ? n.whenFalse : n.whenTrue, P); const keep = c.v ? n.whenTrue : n.whenFalse, t = render(keep, P);
        if (unparen(keep)?.kind === SK.ObjectLiteralExpression && keep.kind === SK.ObjectLiteralExpression && ((n.parent.kind === SK.ArrowFunction && n.parent.body === n) || n.parent.kind === SK.ExpressionStatement)) return `(${t})`;   // (an object where a condition stood, as an arrow's whole body or a statement's start: bare, its braces would be a block)
        return keep.kind === SK.BinaryExpression || keep.kind === SK.ConditionalExpression || keep.kind === SK.ArrowFunction ? needsParens(n) ? `(${t})` : t : t; } break; }
    case SK.ParenthesizedExpression: { const inner = render(n.expression, P), e = unparen(n.expression);   // (x) left bare by a fold keeps its brackets only where it needs them
      if (inner !== text.slice(n.expression.getStart(sf), n.expression.end) && /^[\w.$]+$|^!?[\w.$]+\([^()]*\)$/.test(inner) && e) return inner; return `(${inner})`; }
    case SK.IfStatement: { const c = konst(n.expression, P);
      if (c) { const keep = c.v ? n.thenStatement : n.elseStatement, gone = c.v ? n.elseStatement : n.thenStatement; if (gone) drop(gone, P); drop(n.expression, P);
        if (!keep) return EMPTY; const t = render(keep, P); if (t === EMPTY) return EMPTY;
        if (keep.kind === SK.Block && !hasLexical(keep) && multiOK(n)) { const inner = t.replace(/^\{\s?/, '').replace(/\s?\}$/, ''); return inner.trim() ? inner : EMPTY; }
        return t; }
      const then = render(n.thenStatement, P), els = n.elseStatement ? render(n.elseStatement, P) : null, cond = render(n.expression, P);
      const thenGone = then === EMPTY || /^\{\s*\}$/.test(then), elseGone = els === null || els === EMPTY || /^\{\s*\}$/.test(els);
      if (thenGone && elseGone) { if (pure(n.expression)) { drop(n.expression, P); return EMPTY; } return `${cond};`; }
      if (thenGone) return `if (!(${cond})) ${els}`;
      const head = text.slice(n.getStart(sf), n.expression.getStart(sf)), mid = text.slice(n.expression.end, n.thenStatement.getStart(sf));
      return head + cond + mid + then + (elseGone ? '' : text.slice(n.thenStatement.end, n.elseStatement.getStart(sf)) + els); }
    case SK.WhileStatement: { const c = konst(n.expression, P); if (c && !c.v) { drop(n, P); return EMPTY; } break; }   // (`while (false) …` never turns)
    case SK.TryStatement: { const body = render(n.tryBlock, P); if (/^\{\s*\}$/.test(body.replace(new RegExp(EMPTY, 'g'), '')) && !n.finallyBlock) { drop(n, P); return EMPTY; } break; }
    case SK.Block: case SK.SourceFile: case SK.CaseClause: case SK.DefaultClause: case SK.ModuleBlock: return spliceStatements(n, P);
    case SK.ExpressionStatement: { { let x = unparen(n.expression), cut = false; while (x.kind === SK.BinaryExpression && P.unassign.has(x)) { x = unparen(x.right); cut = true; }
        if (cut && !(x.kind === SK.BinaryExpression && x.operatorToken.kind === SK.EqualsToken) && pure(x)) { drop(n, P); return EMPTY; } }   // (every target of the write is gone and what was written does nothing by itself)
      const e = render(n.expression, P); if (e === EMPTY) return EMPTY; const c = konst(n.expression, P); if (c) return EMPTY;
      // a check's line that asked only what the switch was set to (`line(before === false && carried, …)`, `F.verdict(…)`): its condition is a constant now, it holds nothing, it goes
      const call = unparen(n.expression); if (isTool(sf.fileName) && call.kind === SK.CallExpression && call.arguments.length >= 2 && konst(call.arguments[0], P) && !P.subs.has(call.arguments[0]) && call.arguments[0].kind !== SK.TrueKeyword && call.arguments[0].kind !== SK.FalseKeyword) {
        constLines.push(`${at(sf, n.getStart(sf))} ${n.getText().slice(0, 140)}`); drop(n, P); return EMPTY; }
      break; }
    case SK.VariableStatement: { if (P.bare.has(n)) { const d = n.declarationList.declarations[0]; P.dropped.push({ start: d.name.getStart(sf), end: d.name.end }); const t = render(d.initializer, P); return (/^(\{|function\b|class\b)/.test(t) ? `(${t})` : t) + ';'; }
      const kept = n.declarationList.declarations.filter((d: any) => !goneDecl(d, P)); if (!kept.length) { drop(n, P); return EMPTY; }
      if (kept.length !== n.declarationList.declarations.length) { const kw = text.slice(n.getStart(sf), n.declarationList.declarations[0].getStart(sf)); return kw + kept.map((d: any) => render(d, P)).join(', ') + ';'; } break; }
    case SK.VariableDeclarationList: { const kept = n.declarations.filter((d: any) => !goneDecl(d, P)); if (kept.length !== n.declarations.length && kept.length) { const kw = text.slice(n.getStart(sf), n.declarations[0].getStart(sf)); return kw + kept.map((d: any) => render(d, P)).join(', '); } break; }
    case SK.ObjectLiteralExpression: case SK.ArrayLiteralExpression: case SK.TypeLiteral: case SK.NamedImports: case SK.NamedExports: case SK.ClassDeclaration: case SK.ClassExpression: return spliceList(n, P);
    case SK.TemplateExpression: return splice(n, P).replace(/\$\{'([^'\\`${}]*)'\}/g, '$1').replace(/\$\{(true|false|-?\d+(?:\.\d+)?)\}/g, '$1');   // (a span that became a word is the word)
    case SK.ImportDeclaration: { const t = splice(n, P), m = /^import\s*(type\s*)?\{[\s,]*\}\s*from\s*(['"][^'"]+['"]);?/.exec(t.replace(new RegExp(EMPTY, 'g'), '')); if (m) return m[1] ? EMPTY : `import ${m[2]};`; return t; }   // (its last name gone: the module is still loaded, in the place it was — src/game is one import cycle and the order is the game's)
  }
  return splice(n, P);
}
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
    const r = P.del.has(m) ? null : render(m, P), empty = /^\.\.\.\(?\[\s*\]\)?$/;
    if (r === null || (m.kind === SK.SpreadElement && empty.test(r) && !empty.test(text.slice(s, m.end)))) { drop(m, P); out += text.slice(pos, Math.max(pos, m.jsDoc?.length ? m.jsDoc[0].getStart(sf) : s)) + EMPTY; pos = m.end; const after = /^\s*[,;]?[ \t]?/.exec(text.slice(pos, n.end))!; const sep = after[0]; if (/[,;]/.test(sep)) pos += sep.length; else if (i === list.length - 1) out = out.replace(new RegExp(`,(\\s*${EMPTY})$`), '$1');   // (the last member gone: the comma before it goes too)
      continue; }
    out += text.slice(pos, s) + r; pos = m.end; }
  return out + text.slice(pos, n.end); }

// ---- 4. what only the dropped text used goes too, and a const that became a constant folds on: round after round until nothing more goes
const DECL_KINDS = new Set([SK.ClassDeclaration, SK.FunctionDeclaration, SK.MethodDeclaration, SK.PropertyDeclaration, SK.VariableDeclaration, SK.PropertyAssignment, SK.ImportSpecifier, SK.TypeAliasDeclaration, SK.InterfaceDeclaration, SK.GetAccessor, SK.EnumDeclaration]);
const inSpans = (spans: Span[], pos: number) => spans.some(s => pos >= s.start && pos < s.end);
/** is [a, b) inside something this run takes out whole? (a plan's own list as spans, made again only when the list has grown) */
const delSpans = new Map<FilePlan, Span[]>();
function inDel(P: FilePlan | undefined, a: number, b = a + 1): boolean { if (!P) return false; let sp = delSpans.get(P); if (!sp || sp.length !== P.del.size) { sp = [...P.del].map(x => ({ start: x.getStart(), end: x.end })); delSpans.set(P, sp); } return sp.some(x => a >= x.start && b <= x.end); }
const unread = new Map<any, string>();   // a value nothing reads any more whose making still happens: where, for the report
function renderAll(): Map<string, string> { remark(); constLines.length = 0; emptied.clear(); const out = new Map<string, string>();
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
          if (!toks.some(tok => tok.parent.kind === SK.ShorthandPropertyAssignment)) { localConst.set(n, c.v); grew = true; for (const tok of toks) plan(tok.getSourceFile().fileName).subs.set(tok, c.v); } } }   // (`{ x }` wants the name: such a const stays, a constant)
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
  let h = o?.parent; while (h && (h.kind === SK.AsExpression || h.kind === SK.SatisfiesExpression || h.kind === SK.ParenthesizedExpression)) h = h.parent; return !!h && h.kind === SK.VariableDeclaration; }

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
for (const [f, t] of rendered) { const n = tidy(t); if (n !== textOf(f)) finalText.set(f, n); }

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
const leftovers: string[] = []; { const re = /\|\| true\b|&& false\b|\btrue \|\| |\bfalse && |\(true\)|\(false\)/g;
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
if (report.hooks.length) console.log(`  debug hooks that lose a getter and a setter (a page tool may use them): ${report.hooks.length}`);
{ const live2 = new Set(report.retired), n = namedAt.filter(l => live2.has(l.split(' ')[1].replace(/:$/, ''))); if (n.length) console.log(`  rows named as data beside their table's name (a panel's list, what a machine keeps: taken out): ${n.length}${VERBOSE ? '\n    ' + n.join('\n    ') : ''}`); }
if (unread.size) console.log(`  a value nothing reads any more whose making still happens (the name went, the call stays: read each): ${unread.size}${VERBOSE ? '\n    ' + [...unread.values()].join('\n    ') : ''}`);
if (emptied.size) console.log(`  a loop left with nothing in it (kept, for its head: wants a hand): ${emptied.size}${VERBOSE ? ' · ' + [...emptied].join(' ') : ''}`);
console.log(`  a check's line that only read the switch (taken out; the lines that fed it want a hand): ${constLines.length}`);
if (VERBOSE) { for (const h of report.hooks) console.log(`    hook ${h}`); for (const c of constLines) console.log(`    line ${c}`); for (const w of byKind('opposite')) console.log(`    opposite ${w.at} ${w.row}: ${w.text}`); for (const w of byKind('other')) console.log(`    other ${w.at} ${w.row}: ${w.text}`); for (const d of report.dead) console.log(`    dead ${d}`); for (const n of report.notes) console.log(`    note ${n}`); }
{ const out = arg('--report'); if (out) writeFileSync(out, JSON.stringify({ ...report, constLines, leftovers, droppedLines, dropMissed, named: namedAt.filter(l => report.retired.includes(l.split(' ')[1].replace(/:$/, ''))), unread: [...unread.values()], emptied: [...emptied] }, null, 1)); }
if (flag('--list')) for (const r of report.retired) console.log(`- \`${r}\``);
if (WRITE) { for (const [f, t] of finalText) writeFileSync(f, t); console.log(`  written: ${finalText.size} files`); } else console.log('  (a dry run: --write writes it)');
process.exit(report.refused.length ? 3 : 0);   // (3 = some rows were refused and the rest done: a script that asked for all of them should not read that as done)
