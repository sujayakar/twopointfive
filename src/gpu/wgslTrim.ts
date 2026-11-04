// A SHADER MODULE'S TEXT, CUT TO WHAT IT USES. Every module that touches the scene is compiled behind the whole scene prelude (render/frame.ts scenePrelude:
// the world's constants, common.wgsl's tracing, every smoke read, the character rig) — some 180 KB of WGSL in front of a kernel that may read a twentieth of it.
// Sixty-odd modules a boot made 12 MB of text for the browser's WGSL front end to read, check and carry into each pipeline's compile — on the GPU process's own
// thread, the one that also draws the browser (a play test, 10-19: hitches while the shaders compile, a freeze of most of a second on a fast machine) — and
// made every module's text change whenever ANY switch at the head of the prelude did, so a level in another weather was sixty new texts to the browser's
// compiled-shader cache though most kernels never read the weather.
//
// trimWgsl keeps, in their order and character for character (comments aside), the module-scope declarations an entry point can reach — by name: a declaration
// is kept when a kept one mentions its name anywhere in its text — and drops the rest. What is always kept: every entry point (@compute / @vertex / @fragment),
// every `override` (a pipeline may be handed a constant for one its entry point never reads), every directive and const_assert, and anything this file does not
// recognise. WGSL's module scope has no order and no side effects, and an implementation compiles one entry point's reachable declarations: the program a
// pipeline runs is the same program. Mentions are matched as bare words, so a local that shares a global's name only ever keeps MORE.
//
// `?shadertrim=0` = every module as it was, whole. tools/qa/shadertrim-check.ts holds the rule to the shipped shaders; the page's own boot (every pipeline
// made, no GPU error) is what says the compiler agrees.
export const SHADER_TRIM = { on: true };
try { if (new URLSearchParams(location.search).get('shadertrim') === '0') SHADER_TRIM.on = false; } catch { /* no page (the harness) */ }

// …AND CUT TO WHAT ITS CONSTANT SWITCHES LEAVE ON. The scene prelude's switches are module-scope constants (`const WX_KINDS: bool = false;`, render/frame.ts), read
// once at boot, and the kernels branch on them: `if (WX_KINDS) { … }`. The rule above keeps a declaration that anything kept MENTIONS, so the dead side of such a
// branch kept everything it calls: a trial that is off on this page was still written into every program that could have run it, and Chrome on Windows pastes
// every call into its caller before it drops the branch (tools/qa/shaderinline-check.ts counts it that way). The fold cuts those branches out of the text first.
// In a function's body an `if` statement is read whole, with its `else if`s and its `else`, and each test is read left to right with the module's bool constants
// in hand (`!`, `&&`, `||`, parentheses; a constant written as a test of other constants counts): an arm whose test is false goes, an arm whose test is true
// becomes the statement's `else` and what stood after it goes, and where every arm before it went the statement is that block alone. When that block cannot be
// passed (it ends in a return, break or continue) what follows it in the same block goes too, up to a loop's `continuing`: it never runs, and a compiler says so
// of every line. (`discard` is passed: the function runs on after it and still owes its return.)
// What is left is the same program: every test cut was decided by constants of that very text. A test the constants do not decide BEFORE anything else is asked
// is left as written (`x && OFF` runs x first), and so are a name the function declares itself (a parameter or a local of a constant's name), a `select`, and
// an `if` with an attribute in front of it. A test may be written with or without its outer parentheses.
// THE CONSTANTS STAY. A constant that decided a branch of a kept function is kept with it, as the mention kept it before the fold: the text still says which
// switches it was cut under, and whatever reads a text for a switch (the compile queue's background lane looks for `const CUT_BOXES: bool = true;`,
// gpu/device.ts) reads what it always did.
// A function is folded when the trim first keeps it, never before: most of the prelude's functions are not kept by any one module.
// `?shaderfold=0` = no branch is cut (the trim's own rule alone, as it was). Nothing is cut with the trim off. tools/qa/shaderfold-check.ts holds the rule to
// small texts of every form and to the shaders the engine boots with; that the folded text COMPILES, with no warning, is tools/qa/wgsl-compile.cjs's to say,
// and that it draws the same frame tools/qa/draw-equal.cjs's (`--with shaderfold=0` against none, on the software adapter).
export const SHADER_FOLD = { on: true };
try { if (new URLSearchParams(location.search).get('shaderfold') === '0') SHADER_FOLD.on = false; } catch { /* no page (the harness) */ }

interface Decl { text: string; name: string | null; root: boolean; entry: boolean; fn: boolean; ids: readonly string[] }
/** by declaration text: the prelude's few hundred declarations are read once, whatever number of modules carry them. Its keys are slices of the modules' own
 *  texts and hold those alive, so it is emptied once the boot's modules are made (trimForget) and whenever it has grown past a boot's worth */
const seen = new Map<string, Decl>();
export function trimForget(): void { seen.clear(); folded.clear(); }
const WORD = /[A-Za-z_][A-Za-z0-9_]*/g;
const NAMED = new Set(['fn', 'struct', 'alias', 'const', 'override', 'var', 'let']);
const STAGE = new Set(['compute', 'vertex', 'fragment']);

function read(text: string): Decl {
  let d = seen.get(text); if (d) return d;
  if (seen.size > 8192) seen.clear();
  // attributes first: @name or @name(…), any number
  let i = 0; const n = text.length; let entry = false;
  const ws = () => { while (i < n && text.charCodeAt(i) <= 32) i++; };
  const word = () => { const a = i; while (i < n) { const c = text.charCodeAt(i); if ((c >= 48 && c <= 57) || (c >= 65 && c <= 90) || (c >= 97 && c <= 122) || c === 95) i++; else break; } return text.slice(a, i); };
  for (ws(); text[i] === '@'; ws()) { i++; ws(); if (STAGE.has(word())) entry = true; ws(); if (text[i] === '(') { let depth = 0; do { if (text[i] === '(') depth++; else if (text[i] === ')') depth--; i++; } while (i < n && depth > 0); } }
  const kw = word(); let name: string | null = null;
  if (NAMED.has(kw)) { ws(); if (kw === 'var' && text[i] === '<') { let depth = 0; do { if (text[i] === '<') depth++; else if (text[i] === '>') depth--; i++; } while (i < n && depth > 0); ws(); } name = word() || null; }
  const ids = [...new Set(text.match(WORD) ?? [])];
  d = { text, name, root: name === null || kw === 'override' || (kw === 'fn' && entry), entry: kw === 'fn' && entry, fn: kw === 'fn', ids };
  seen.set(text, d); return d;
}

/** the module-scope declarations of `code` in order, comments removed: each ends at a `;` outside braces or at the `}` that closes its body */
export function wgslDecls(code: string): string[] {
  const out: string[] = []; const n = code.length; let parts: string[] = [], from = 0, depth = 0, any = false;
  const flush = (to: number) => { if (to > from) parts.push(code.slice(from, to)); };
  const end = (to: number) => { flush(to); from = to; const t = parts.join('').trim(); parts = []; any = false; if (t) out.push(t); };
  for (let i = 0; i < n; i++) {
    const c = code.charCodeAt(i);
    if (c === 47 /* / */) { const c2 = code.charCodeAt(i + 1);
      if (c2 === 47) { flush(i); while (i < n && code.charCodeAt(i) !== 10) i++; from = i; i--; continue; }   // a line comment: up to (not with) its newline
      if (c2 === 42 /* * */) { flush(i); let k = 1; i += 2; while (i < n && k > 0) { const a = code.charCodeAt(i), b = code.charCodeAt(i + 1); if (a === 47 && b === 42) { k++; i += 2; } else if (a === 42 && b === 47) { k--; i += 2; } else i++; } parts.push(' '); from = i; i--; continue; }   // a block comment (they nest): one space in its place
    }
    if (c === 59 /* ; */ && depth === 0) { if (any) end(i + 1); else { flush(i); from = i + 1; } continue; }   // (a `;` with nothing before it — the optional one after a struct's `}` — is dropped)
    if (c > 32) any = true;
    if (c === 123 /* { */) depth++;
    else if (c === 125 /* } */) { depth--; if (depth === 0) end(i + 1); }
  }
  end(n);
  return out;
}

// ---- the fold
/** a test read with the constants in hand, left to right as it runs: true or false where the constants decide it before anything else is asked, null otherwise.
 *  `!`, `&&`, `||`, parentheses, `true`, `false` and names; anything else is something only the GPU knows. Left to right matters: `OFF && x` is false and
 *  `ON || x` is true whatever x is, but `x && OFF` runs x first, so it is left alone. */
function foldTest(expr: string, val: (name: string) => boolean | undefined): boolean | null {
  let i = 0; const n = expr.length;
  const ws = () => { while (i < n && expr.charCodeAt(i) <= 32) i++; };
  /** past one operand this does not read: to the next `&&` / `||` / `)` at this depth */
  const skip = () => { let d = 0; for (; i < n; i++) { const c = expr[i]; if (c === '(' || c === '[') d++; else if (c === ')' || c === ']') { if (d === 0) return; d--; } else if (d === 0 && (c === '&' || c === '|') && expr[i + 1] === c) return; } };
  const unary = (): boolean | null => {
    ws();
    if (expr[i] === '!' && expr[i + 1] !== '=') { i++; const v = unary(); return v === null ? null : !v; }
    if (expr[i] === '(') { i++; const v = or(); ws(); if (expr[i] === ')') { i++; return tail(v); } skip(); return null; }
    const m = /^[A-Za-z_][A-Za-z0-9_]*/.exec(expr.slice(i));
    if (m) { i += m[0].length; const v = m[0] === 'true' ? true : m[0] === 'false' ? false : val(m[0]); return tail(v === undefined ? null : v); }
    skip(); return null;
  };
  /** an operand is only its value if nothing follows it but the next `&&` / `||` / `)` (a name in `NAME == x` or `f(NAME)` is not the test) */
  const tail = (v: boolean | null): boolean | null => { ws(); if (i >= n || expr[i] === ')' || ((expr[i] === '&' || expr[i] === '|') && expr[i + 1] === expr[i])) return v; skip(); return null; };
  const and = (): boolean | null => { let v = unary(); for (ws(); expr[i] === '&' && expr[i + 1] === '&'; ws()) { i += 2; const r = unary(); v = v === false ? false : v === null ? null : r; } return v; };
  const or = (): boolean | null => { let v = and(); for (ws(); expr[i] === '|' && expr[i + 1] === '|'; ws()) { i += 2; const r = and(); v = v === true ? true : v === null ? null : r; } return v; };
  const v = or(); ws(); return i >= n ? v : null;
}
/** a module's bool constants by name: `const NAME: bool = <a test of literals and other such constants>;` (render/frame.ts writes the switches as literals; a
 *  kernel may name a combination of them). Exported for the check, which reads the same constants. */
export function boolConsts(decls: readonly string[]): Map<string, boolean> {
  const val = new Map<string, boolean>(), open: [string, string][] = [];
  for (const d of decls) { if (d.charCodeAt(0) !== 99 /* c */) continue; const m = /^const\s+([A-Za-z_][A-Za-z0-9_]*)\s*:\s*bool\s*=\s*([^;]+);$/.exec(d); if (m) open.push([m[1], m[2]]); }
  for (let again = true; again;) { again = false; for (let k = open.length - 1; k >= 0; k--) { const v = foldTest(open[k][1], g => val.get(g)); if (v !== null) { val.set(open[k][0], v); open.splice(k, 1); again = true; } } }
  return val;
}
/** the index of the bracket that closes the one at `open` (`{` or `(`; -1: none) */
function closing(t: string, open: number): number { const o = t.charCodeAt(open), c = o === 123 ? 125 : 41; let d = 0; for (let i = open; i < t.length; i++) { const x = t.charCodeAt(i); if (x === o) d++; else if (x === c && --d === 0) return i; } return -1; }
const blank = (t: string, i: number): number => { while (i < t.length && t.charCodeAt(i) <= 32) i++; return i; };
/** an if statement read whole from its `if` at `at`: every `test { block }` of the chain (a test as written, in parentheses or not) and its last
 *  `else { block }`, or null where it is not written that way */
function chainAt(t: string, at: number): { arms: { test: string; block: string }[]; last: string | null; end: number } | null {
  const arms: { test: string; block: string }[] = []; let i = at;
  for (;;) {
    i = blank(t, i + 2);
    let bo = -1; for (let k = i, d = 0; k < t.length; k++) { const c = t.charCodeAt(k); if (c === 40 || c === 91) d++; else if (c === 41 || c === 93) { if (--d < 0) break; } else if (c === 59 || c === 125) break; else if (c === 123 && d === 0) { bo = k; break; } }   // (a test holds no brace and no `;`)
    if (bo <= i) return null;
    const bc = closing(t, bo); if (bc < 0) return null;
    arms.push({ test: t.slice(i, bo).trim(), block: t.slice(bo, bc + 1) });
    const e = blank(t, bc + 1); if (!t.startsWith('else', e) || /[A-Za-z0-9_]/.test(t[e + 4] ?? '')) return { arms, last: null, end: bc + 1 };
    const n = blank(t, e + 4);
    if (t[n] === '{') { const c = closing(t, n); return c < 0 ? null : { arms, last: t.slice(n, c + 1), end: c + 1 }; }
    if (!t.startsWith('if', n) || /[A-Za-z0-9_]/.test(t[n + 2] ?? '')) return null;
    i = n;
  }
}
/** where the statement that starts at `i` (blanks skipped) ends, inside the block text `t`: an `if` with its whole chain, anything else up to its `;` or to the
 *  brace that closes its own block (-1: not read) */
function stmtEnd(t: string, i: number): number {
  if (t.startsWith('if', i) && !/[A-Za-z0-9_]/.test(t[i + 2] ?? '')) { const C = chainAt(t, i); if (C) return C.end; }
  let d = 0;
  for (let k = i; k < t.length; k++) { const c = t.charCodeAt(k);
    if (c === 123 || c === 40 || c === 91) d++;
    else if (c === 125 || c === 41 || c === 93) { d--; if (d < 0) return -1; if (d === 0 && c === 125) return k + 1; }
    else if (c === 59 && d === 0) return k + 1; }
  return -1;
}
/** a statement cannot be passed: a `return`, `break` or `continue`; a bare block that holds one at its top; an `if` with an `else` whose every arm is such a
 *  block. Not a `discard`: what follows it still runs (the fragment is thrown away when the function is done), and a function that returns a value still owes it */
function leaves(st: string): boolean {
  if (st[0] === '{') return leavesAt(st) >= 0;
  if (st.startsWith('if') && !/[A-Za-z0-9_]/.test(st[2] ?? '')) { const C = chainAt(st, 0); return !!C && C.end === st.length && C.last !== null && leavesAt(C.last) >= 0 && C.arms.every(a => leavesAt(a.block) >= 0); }
  return /^(?:return\b[^{}]*;|(?:break|continue)\s*;)$/.test(st);
}
/** in the block text `{ … }`: where its first statement that cannot be passed ends (-1: every statement can be, or the block is not read) */
function leavesAt(block: string): number {
  for (let i = blank(block, 1); i < block.length - 1;) { const e = stmtEnd(block, i); if (e < 0 || e > block.length - 1) return -1; if (leaves(block.slice(i, e))) return e; i = blank(block, e); }
  return -1;
}
/** `t` with what follows a statement that cannot be passed cut out of every block (it never runs, and a compiler says so of every such line). A loop's
 *  `continuing { }` stands at the end of the loop's block and runs after a `continue`: the cut stops in front of it. A tail that is not read is left whole. */
function cutTails(t: string): string {
  for (let from = t.indexOf('{'); from >= 0; from = t.indexOf('{', from + 1)) {
    const c = closing(t, from); if (c < 0) continue;
    const e = leavesAt(t.slice(from, c + 1)); if (e < 0) continue;
    let to = from + e, ok = true;
    for (let i = blank(t, to); i < c;) { if (t.startsWith('continuing', i) && !/[A-Za-z0-9_]/.test(t[i + 10] ?? '')) break; const n = stmtEnd(t, i); if (n < 0 || n > c) { ok = false; break; } to = n; i = blank(t, n); }
    if (!ok || to === from + e) continue;
    t = t.slice(0, from + e) + ' ' + t.slice(blank(t, to));
  }
  return t;
}
/** one function's text with every branch its constants decide cut to the side that runs (the head of this file has the rule) */
function foldFn(text: string, val: (name: string) => boolean | undefined, used?: Set<string>): string {
  let t = text;
  for (let from = t.indexOf('{'); from >= 0;) {
    const at = t.indexOf('if', from); if (at < 0) break; from = at + 2;
    if (/[A-Za-z0-9_]/.test(t[at - 1] ?? '') || /[A-Za-z0-9_]/.test(t[at + 2] ?? '')) continue;      // (a name that holds the letters)
    let b = at - 1; while (b >= 0 && t.charCodeAt(b) <= 32) b--;
    if (t[b] !== ';' && t[b] !== '{' && t[b] !== '}') continue;                                       // (an `else if` is read with its chain; an attribute in front: left as written)
    const C = chainAt(t, at); if (!C) continue;
    const kept: { test: string; block: string }[] = []; let last = C.last, changed = false;
    for (const a of C.arms) { const v = foldTest(a.test, val); if (v === null) { kept.push(a); continue; }
      changed = true; if (used) for (const w of a.test.match(WORD) ?? []) if (val(w) !== undefined) used.add(w);   // (the constants that decided it: kept with the function, the head of this file)
      if (v) { last = a.block; break; } }
    if (!changed) continue;
    const put = kept.length ? kept.map((a, k) => `${k ? 'else ' : ''}if ${a.test} ${a.block}`).join(' ') + (last !== null ? ` else ${last}` : '') : last ?? '';
    t = t.slice(0, at) + put + t.slice(C.end);
    from = at;                                                                                        // (what took its place is read too: a branch inside the kept side)
  }
  return t === text ? text : cutTails(t);
}
/** by declaration text: which of the module's constant names this function declares for itself (a parameter, a local: not the module's), and its folded
 *  text by the values of the constants it tests. A prelude function is folded once for all the modules that carry it under the same switches (emptied with
 *  the trim's own book) */
const folded = new Map<string, { own: Map<string, boolean>; by: Map<string, Decl> }>();
/** a function as the trim keeps it under the module's constants `val`: its branches folded, and the constants that decided them among its mentions */
function foldOne(d: Decl, val: ReadonlyMap<string, boolean>): Decl {
  if (!d.fn || !val.size) return d;
  let book = folded.get(d.text), sig = '', mine: Map<string, boolean> | null = null;
  for (const w of d.ids) { const v = val.get(w); if (v === undefined) continue;
    if (!book) { if (folded.size > 8192) folded.clear(); folded.set(d.text, book = { own: new Map(), by: new Map() }); }
    let own = book.own.get(w);
    if (own === undefined) { const open = d.text.indexOf('{'); book.own.set(w, own = new RegExp(`\\b${w}\\s*:`).test(d.text.slice(0, Math.max(0, open))) || new RegExp(`\\b(?:let|var|const)\\b\\s*(?:<[^>]*>)?\\s*${w}\\b`).test(d.text)); }
    if (own) continue;
    (mine ??= new Map()).set(w, v); sig += `${w}${v ? 1 : 0},`; }
  if (!mine || !book) return d;
  let f = book.by.get(sig);
  if (!f) { const used = new Set<string>(), m = mine, text = foldFn(d.text, g => m.get(g), used);
    f = text === d.text ? d : { ...d, text, ids: [...new Set([...read(text).ids, ...used])] }; book.by.set(sig, f); }
  return f;
}
/** `code`, a module's text, with its constant switches' dead branches cut out of every function (comments go with it, as in the trim). For a check or a tool
 *  that reads the fold alone: the page's text is trimWgsl's, which folds only what it keeps. */
export function foldWgsl(code: string): string { const texts = wgslDecls(code), val = boolConsts(texts); return texts.map(t => foldOne(read(t), val).text).join('\n') + '\n'; }

/** `code` with every module-scope declaration no entry point can reach removed (the head of this file has the rule). Text with no entry point in it — a
 *  fragment of a module, not a module — comes back whole. `fold`: a kept function's constant switches' dead branches are cut (foldOne), so what only they
 *  reach is never kept. */
export function trimWgsl(code: string, fold = false): string {
  const texts = wgslDecls(code), raw = texts.map(read);
  if (!raw.some(d => d.entry)) return code;
  const val = fold ? boolConsts(texts) : null, got: (Decl | undefined)[] = [];
  const of = (i: number): Decl => got[i] ??= val ? foldOne(raw[i], val) : raw[i];
  const at = new Map<string, number[]>(); raw.forEach((d, i) => { if (d.name) { const l = at.get(d.name); if (l) l.push(i); else at.set(d.name, [i]); } });
  const keep = new Uint8Array(raw.length), todo: number[] = [];
  raw.forEach((d, i) => { if (d.root) { keep[i] = 1; todo.push(i); } });
  while (todo.length) { const d = of(todo.pop()!); for (const id of d.ids) { const l = at.get(id); if (l) for (const j of l) if (!keep[j]) { keep[j] = 1; todo.push(j); } } }
  const out: string[] = []; raw.forEach((_d, i) => { if (keep[i]) out.push(of(i).text); });
  return out.join('\n') + '\n';
}

/** how much the page's modules were cut, summed since the page opened (window.__boot.shaderText reads it): characters handed in, characters compiled, ms spent cutting */
export const trimTotals = { modules: 0, before: 0, after: 0, ms: 0 };
/** makeShader's step (gpu/device.ts): the text to compile for `code` */
export function shaderText(code: string): string {
  const t = typeof performance !== 'undefined' ? performance.now() : 0;
  const out = SHADER_TRIM.on ? trimWgsl(code, SHADER_FOLD.on) : code;
  trimTotals.modules++; trimTotals.before += code.length; trimTotals.after += out.length; if (typeof performance !== 'undefined') trimTotals.ms += performance.now() - t;
  return out;
}
