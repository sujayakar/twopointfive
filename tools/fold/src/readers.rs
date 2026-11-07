//! Whether a function writes, said by its signature and held by the compiler.
//!
//! `demote` turns every `&mut T` parameter of a body file's function into `&T`. `cargo check` then names each place
//! that needed the `mut` (a write, a `&mut` borrow, a call of a function that still asks for `&mut`). `restore` puts
//! the `mut` back on the parameter each error is about, then follows the calls: a function that hands its
//! parameter to a `&mut` parameter of another needs its own `mut` back too. The driver (readers.sh) checks again
//! until the compiler names nothing. What is left without `mut` is a reader by the compiler's word.
//!
//! One thing the compiler cannot see: `sim::state::state(slot)` hands out the simulation block from a number, so a
//! function can write the block holding only `&Game`. A function that names `sim::` with the game's slot is kept
//! a writer by this program, and so is everything that hands it the game (TAINT below).

use crate::dead::rs_files;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::path::{Path, PathBuf};
use syn::spanned::Spanned;
use syn::visit::{self, Visit};
use syn::{Expr, FnArg, Pat, Type};

/// Words in a function's body that mean it can write without `&mut Game`.
const TAINT: &[&str] = &["UnsafeCell", "unsafe ", "unsafe{", "static mut", "AtomicU32", "AtomicUsize", "AtomicBool", "Ordering::", "RefCell", "Cell<"];

use crate::dead::is_body;

fn is_test_mod(m: &syn::ItemMod) -> bool {
    m.attrs.iter().any(|a| a.path().is_ident("cfg") && a.meta.require_list().map(|l| l.tokens.to_string().contains("test")).unwrap_or(false))
}

/// One function of a body file.
struct FnAt<'a> {
    name: String,
    /// which of the file's functions of that name, in order
    ord: usize,
    sig: &'a syn::Signature,
    block: &'a syn::Block,
    lines: (usize, usize),
    /// the lines of its signature, up to where its body opens
    sig_lines: (usize, usize),
    /// a function of an `impl`
    method: bool,
}

fn line_of(src: &str, at: usize) -> usize {
    src[..at].bytes().filter(|b| *b == b'\n').count() + 1
}

fn fns_of<'a>(src: &str, file: &'a syn::File) -> Vec<FnAt<'a>> {
    let mut out: Vec<FnAt<'a>> = Vec::new();
    let mut seen: HashMap<String, usize> = HashMap::new();
    fn walk<'a>(items: &'a [syn::Item], src: &str, out: &mut Vec<FnAt<'a>>, seen: &mut HashMap<String, usize>) {
        for it in items {
            match it {
                syn::Item::Fn(f) => {
                    let name = f.sig.ident.to_string();
                    let ord = *seen.entry(name.clone()).and_modify(|n| *n += 1).or_insert(0);
                    let r = it.span().byte_range();
                    out.push(FnAt { name, ord, sig: &f.sig, block: &f.block, lines: (line_of(src, r.start), line_of(src, r.end)), sig_lines: (line_of(src, r.start), line_of(src, f.block.span().byte_range().start)), method: false });
                }
                syn::Item::Impl(x) if x.trait_.is_none() => {
                    for ii in &x.items {
                        if let syn::ImplItem::Fn(f) = ii {
                            let name = f.sig.ident.to_string();
                            let ord = *seen.entry(name.clone()).and_modify(|n| *n += 1).or_insert(0);
                            let r = ii.span().byte_range();
                            out.push(FnAt { name, ord, sig: &f.sig, block: &f.block, lines: (line_of(src, r.start), line_of(src, r.end)), sig_lines: (line_of(src, r.start), line_of(src, f.block.span().byte_range().start)), method: true });
                        }
                    }
                }
                syn::Item::Mod(m) if !is_test_mod(m) => {
                    if let Some((_, inner)) = &m.content {
                        walk(inner, src, out, seen);
                    }
                }
                _ => {}
            }
        }
    }
    walk(&file.items, src, &mut out, &mut seen);
    out
}

/// (parameter name, where its type's `&` ends, where a `mut ` stands or would stand, has `mut` now)
fn ref_params(sig: &syn::Signature) -> Vec<(usize, String, usize, bool)> {
    let mut v = Vec::new();
    for (i, a) in sig.inputs.iter().enumerate() {
        let FnArg::Typed(t) = a else { continue };
        let Pat::Ident(p) = &*t.pat else { continue };
        let Type::Reference(r) = &*t.ty else { continue };
        // the place right after `&` (and a lifetime, if one is written)
        let at = match &r.lifetime {
            Some(l) => l.span().byte_range().end,
            None => r.and_token.span().byte_range().end,
        };
        v.push((i, p.ident.to_string(), at, r.mutability.is_some()));
    }
    v
}

/// key of a demoted parameter: `file|function|ord|parameter`
fn key(file: &str, name: &str, ord: usize, param: &str) -> String {
    format!("{file}|{name}|{ord}|{param}")
}

fn bodies(root: &Path) -> Vec<(PathBuf, String, String)> {
    let game = root.join("rust/game");
    let mut all = Vec::new();
    rs_files(&game.join("src"), &mut all);
    all.into_iter().filter(|p| is_body(p)).filter_map(|p| std::fs::read_to_string(&p).ok().map(|t| (p.clone(), p.strip_prefix(&game).unwrap_or(&p).to_string_lossy().to_string(), t))).collect()
}

fn load_state(path: &Path) -> BTreeSet<String> {
    std::fs::read_to_string(path).map(|t| t.lines().filter(|l| !l.is_empty()).map(|l| l.to_string()).collect()).unwrap_or_default()
}
fn save_state(path: &Path, s: &BTreeSet<String>) {
    let _ = std::fs::write(path, s.iter().cloned().collect::<Vec<_>>().join("\n") + "\n");
}

/// Where the files are kept as they were before the pass: every text the pass writes is made from these.
fn orig_dir(state: &Path) -> PathBuf {
    state.parent().unwrap_or(Path::new(".")).join("orig")
}
fn soft_path(state: &Path) -> PathBuf {
    state.with_extension("soft")
}

/// The body files as they were before the pass: (where the file is, its path from rust/game, its text then).
fn pristine(root: &Path, state: &Path) -> Result<Vec<(PathBuf, String, String)>, String> {
    let game = root.join("rust/game");
    let dir = orig_dir(state);
    let mut out = Vec::new();
    for (path, rel, _) in bodies(root) {
        let t = std::fs::read_to_string(dir.join(&rel)).map_err(|e| format!("no copy of {rel} from before the pass ({e}): run readers-demote first"))?;
        out.push((game.join(&rel), rel, t));
        let _ = path;
    }
    Ok(out)
}

/// A read spelled as a write, and its shared twin.
const SOFT: &[(&str, &str)] = &[
    (".as_mut()", ".as_ref()"),
    (".get_mut(", ".get("),
    (".iter_mut()", ".iter()"),
    (".last_mut()", ".last()"),
    (".first_mut()", ".first()"),
    (".values_mut()", ".values()"),
    (".as_deref_mut()", ".as_deref()"),
    ("&mut ", "&"),
    ("ref mut ", "ref "),
];

/// `src` with every comment, string and character blanked, byte for byte, so a word is looked for in code alone.
fn code_only(src: &str) -> Vec<u8> {
    let b = src.as_bytes();
    let mut o = b.to_vec();
    let mut i = 0;
    let blank = |o: &mut Vec<u8>, from: usize, to: usize| {
        for k in from..to.min(o.len()) {
            if o[k] != b'\n' {
                o[k] = b' ';
            }
        }
    };
    while i < b.len() {
        if b[i] == b'/' && i + 1 < b.len() && b[i + 1] == b'/' {
            let e = src[i..].find('\n').map(|j| i + j).unwrap_or(b.len());
            blank(&mut o, i, e);
            i = e;
        } else if b[i] == b'/' && i + 1 < b.len() && b[i + 1] == b'*' {
            let mut depth = 1;
            let mut j = i + 2;
            while j < b.len() && depth > 0 {
                if b[j] == b'/' && j + 1 < b.len() && b[j + 1] == b'*' {
                    depth += 1;
                    j += 2;
                } else if b[j] == b'*' && j + 1 < b.len() && b[j + 1] == b'/' {
                    depth -= 1;
                    j += 2;
                } else {
                    j += 1;
                }
            }
            blank(&mut o, i, j);
            i = j;
        } else if b[i] == b'r' && i + 1 < b.len() && (b[i + 1] == b'"' || b[i + 1] == b'#') && (i == 0 || !(b[i - 1].is_ascii_alphanumeric() || b[i - 1] == b'_')) {
            // a raw string: r"…", r#"…"#
            let mut j = i + 1;
            let mut hashes = 0;
            while j < b.len() && b[j] == b'#' {
                hashes += 1;
                j += 1;
            }
            if j < b.len() && b[j] == b'"' {
                let close = format!("\"{}", "#".repeat(hashes));
                let e = src[j + 1..].find(&close).map(|k| j + 1 + k + close.len()).unwrap_or(b.len());
                blank(&mut o, i, e);
                i = e;
            } else {
                i += 1;
            }
        } else if b[i] == b'"' {
            let mut j = i + 1;
            while j < b.len() && b[j] != b'"' {
                j += if b[j] == b'\\' { 2 } else { 1 };
            }
            blank(&mut o, i, j + 1);
            i = j + 1;
        } else if b[i] == b'\'' {
            // a character (`'x'`, `'\n'`), not a lifetime (`'a`)
            if i + 2 < b.len() && b[i + 1] == b'\\' {
                let e = src[i + 2..].find('\'').map(|k| i + 2 + k + 1).unwrap_or(b.len());
                blank(&mut o, i, e);
                i = e;
            } else if let Some(c) = src[i + 1..].chars().next() {
                let e = i + 1 + c.len_utf8();
                if e < b.len() && b[e] == b'\'' {
                    blank(&mut o, i, e + 1);
                    i = e + 1;
                } else {
                    i += 1;
                }
            } else {
                i += 1;
            }
        } else {
            i += 1;
        }
    }
    o
}

/// One read spelled as a write: where it stands in the text from before the pass.
struct Site {
    off: usize,
    len: usize,
    to: String,
    line: usize,
}

/// The functions that have a reading twin (`f` beside `f_read`), by where they are: a module of the game (called there by
/// the bare name) or `sim::m` (called as `sim::m::f(…)`). A call of one is a site: it may become a call of its twin.
type Twins = HashMap<String, Vec<String>>;

fn twins_of(root: &Path, files: &[(PathBuf, String, String)]) -> Twins {
    // module -> name -> what it hands back, as written
    let mut names: HashMap<String, HashMap<String, String>> = HashMap::new();
    let mut add = |module: String, src: &str| {
        if let Ok(f) = syn::parse_file(src) {
            for x in fns_of(src, &f) {
                if !x.method {
                    let ret = match &x.sig.output {
                        syn::ReturnType::Type(_, t) => src[t.span().byte_range()].to_string(),
                        _ => String::new(),
                    };
                    names.entry(module.clone()).or_default().insert(x.name.clone(), ret);
                }
            }
        }
    };
    for (_, rel, src) in files {
        add(module_of(rel), src);
    }
    for (m, src) in sim_files(root) {
        add(m, &src);
    }
    let mut out = Twins::new();
    for (m, set) in names {
        // `f` hands back a `&'static mut`, `f_read` the same thing shared
        let mut v: Vec<String> = set.iter().filter(|(n, ret)| ret.contains("&'static mut") && set.get(&format!("{n}_read")).map(|r| is_read_twin(&format!("{n}_read"), r)).unwrap_or(false)).map(|(n, _)| n.clone()).collect();
        v.sort();
        if !v.is_empty() {
            out.insert(m, v);
        }
    }
    out
}

/// The sites of a body file: inside a function's body or one of the file's own `macro_rules!`, never in a test module.
fn sites_of(src: &str, file: &syn::File, module: &str, twins: &Twins) -> Vec<Site> {
    let mut ranges: Vec<std::ops::Range<usize>> = Vec::new();
    fn walk(items: &[syn::Item], ranges: &mut Vec<std::ops::Range<usize>>) {
        for it in items {
            match it {
                syn::Item::Fn(f) => ranges.push(f.block.span().byte_range()),
                syn::Item::Impl(x) => {
                    for ii in &x.items {
                        if let syn::ImplItem::Fn(f) = ii {
                            ranges.push(f.block.span().byte_range());
                        }
                    }
                }
                syn::Item::Macro(m) if m.ident.is_some() => ranges.push(it.span().byte_range()),
                syn::Item::Mod(m) if !is_test_mod(m) => {
                    if let Some((_, inner)) = &m.content {
                        walk(inner, ranges);
                    }
                }
                _ => {}
            }
        }
    }
    walk(&file.items, &mut ranges);
    let code = code_only(src);
    let mut out = Vec::new();
    for r in ranges {
        let text = &code[r.clone()];
        for (from, to) in SOFT {
            let pat = from.as_bytes();
            let mut i = 0;
            while i + pat.len() <= text.len() {
                if &text[i..i + pat.len()] == pat {
                    let at = r.start + i;
                    // (`ref mut` is a word of its own, not the end of one; `&'a mut` is not `&mut`)
                    let word_ok = !from.starts_with("ref") || at == 0 || !(code[at - 1].is_ascii_alphanumeric() || code[at - 1] == b'_');
                    if word_ok {
                        out.push(Site { off: at, len: pat.len(), to: to.to_string(), line: line_of(src, at) });
                    }
                    i += pat.len();
                } else {
                    i += 1;
                }
            }
        }
        // a call of a function that has a reading twin: `wx(` its own module's, `sim::weather::of(` the sim's
        let word = |at: usize| at == 0 || !(code[at - 1].is_ascii_alphanumeric() || code[at - 1] == b'_');
        let mut find = |pat: &str, name_at: usize, name: &str, bare: bool| {
            let p = pat.as_bytes();
            let mut i = 0;
            while i + p.len() <= text.len() {
                if &text[i..i + p.len()] == p {
                    let at = r.start + i;
                    // (a bare name: not a method, not another module's, not the tail of a longer word)
                    let ok = word(at) && (!bare || at == 0 || !matches!(code[at - 1], b'.' | b':'));
                    if ok {
                        out.push(Site { off: at + name_at, len: name.len(), to: format!("{name}_read"), line: line_of(src, at) });
                    }
                    i += p.len();
                } else {
                    i += 1;
                }
            }
        };
        for (m, names) in twins {
            for n in names {
                if m == module {
                    find(&format!("{n}("), 0, n, true);
                } else if let Some(sm) = m.strip_prefix("sim::") {
                    let pre = format!("sim::{sm}::");
                    find(&format!("{pre}{n}("), pre.len(), n, false);
                }
            }
        }
    }
    out.sort_by_key(|s| s.off);
    out.dedup_by_key(|s| s.off);
    out
}

fn soft_key(rel: &str, off: usize) -> String {
    format!("{rel}|{off}")
}

/// The file as the pass has it now: its text from before, with each parameter in `demoted` made `&` and each site in `soft`
/// made its shared twin. Words change inside their lines; no line is added or lost.
fn render(src: &str, file: &syn::File, rel: &str, demoted: &BTreeSet<String>, soft: &BTreeSet<String>, twins: &Twins) -> String {
    let mut edits: Vec<(usize, usize, String)> = Vec::new();
    for f in fns_of(src, file) {
        for (_, p, at, is_mut) in ref_params(f.sig) {
            if is_mut && demoted.contains(&key(rel, &f.name, f.ord, &p)) {
                // `&mut T` loses `mut ` ; `&'a mut T` keeps the blank after its lifetime
                let rest = &src[at..];
                let ws = rest.len() - rest.trim_start().len();
                if rest[ws..].starts_with("mut") {
                    let after = &rest[ws + 3..];
                    let ws2 = after.len() - after.trim_start().len();
                    edits.push((at + ws, 3 + ws2, String::new()));
                }
            }
        }
    }
    if !soft.is_empty() {
        for s in sites_of(src, file, &module_of(rel), twins) {
            if soft.contains(&soft_key(rel, s.off)) {
                edits.push((s.off, s.len, s.to));
            }
        }
    }
    edits.sort_by_key(|e| std::cmp::Reverse(e.0));
    let mut out = src.to_string();
    for (at, len, to) in edits {
        out.replace_range(at..at + len, &to);
    }
    out
}

/// Every `&mut T` parameter of a body function becomes `&T`; with `soften`, every read spelled as a write becomes its
/// shared twin too. The files as they were are kept beside the state file, which lists what was changed.
pub fn demote(root: &Path, state: &Path, soften: bool) -> Result<String, String> {
    let mut demoted: BTreeSet<String> = BTreeSet::new();
    let mut soft: BTreeSet<String> = BTreeSet::new();
    let dir = orig_dir(state);
    let _ = std::fs::remove_dir_all(&dir);
    let mut files = 0;
    let before = bodies(root);
    let twins = if soften { twins_of(root, &before) } else { Twins::new() };
    for (path, rel, src) in before {
        let keep = dir.join(&rel);
        if let Some(d) = keep.parent() {
            std::fs::create_dir_all(d).map_err(|e| format!("cannot make {}: {e}", d.display()))?;
        }
        std::fs::write(&keep, &src).map_err(|e| format!("cannot write {}: {e}", keep.display()))?;
        let file = syn::parse_file(&src).map_err(|e| format!("{rel} does not parse: {e}"))?;
        for f in fns_of(&src, &file) {
            for (_, p, _, is_mut) in ref_params(f.sig) {
                if is_mut {
                    demoted.insert(key(&rel, &f.name, f.ord, &p));
                }
            }
        }
        if soften {
            for s in sites_of(&src, &file, &module_of(&rel), &twins) {
                soft.insert(soft_key(&rel, s.off));
            }
        }
        let now = render(&src, &file, &rel, &demoted, &soft, &twins);
        if now != src {
            if syn::parse_file(&now).is_err() {
                return Err(format!("{rel} would not parse after the pass's edits"));
            }
            std::fs::write(&path, now).map_err(|e| format!("cannot write {rel}: {e}"))?;
            files += 1;
        }
    }
    save_state(state, &demoted);
    save_state(&soft_path(state), &soft);
    Ok(format!("{} `&mut` parameters made `&`{} in {files} body files", demoted.len(), if soften { format!(" and {} reads spelled as writes made shared", soft.len()) } else { String::new() }))
}

/// The names a call's argument is rooted in: `g`, `&mut g.x`, `g.tab[i].y`, `*g` are all `g`.
fn root_ident(e: &Expr) -> Option<String> {
    match e {
        Expr::Path(p) if p.path.segments.len() == 1 => Some(p.path.segments[0].ident.to_string()),
        Expr::Reference(r) => root_ident(&r.expr),
        Expr::Unary(u) => root_ident(&u.expr),
        Expr::Paren(p) => root_ident(&p.expr),
        Expr::Field(f) => root_ident(&f.base),
        Expr::Index(i) => root_ident(&i.expr),
        Expr::MethodCall(m) => root_ident(&m.receiver),
        Expr::Try(t) => root_ident(&t.expr),
        _ => None,
    }
}

/// The calls of a body: (callee's module if written, callee's name, the root name of each argument).
struct Calls {
    out: Vec<(Option<String>, String, Vec<Option<String>>)>,
}
impl<'ast> Visit<'ast> for Calls {
    fn visit_expr_call(&mut self, c: &'ast syn::ExprCall) {
        if let Expr::Path(p) = &*c.func {
            let segs: Vec<String> = p.path.segments.iter().map(|s| s.ident.to_string()).collect();
            if let Some(name) = segs.last() {
                let module = if segs.len() >= 2 { Some(segs[segs.len() - 2].clone()) } else { None };
                self.out.push((module, name.clone(), c.args.iter().map(root_ident).collect()));
            }
        }
        visit::visit_expr_call(self, c);
    }
}

fn idents_in(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    for ch in text.chars() {
        if ch.is_alphanumeric() || ch == '_' {
            cur.push(ch);
        } else if !cur.is_empty() {
            out.push(std::mem::take(&mut cur));
        }
    }
    if !cur.is_empty() {
        out.push(cur);
    }
    out
}

type CallsOf = Vec<Vec<Vec<(Option<String>, String, Vec<Option<String>>)>>>;

fn module_of(rel: &str) -> String {
    Path::new(rel).parent().and_then(|d| d.file_name()).and_then(|n| n.to_str()).unwrap_or("").to_string()
}

fn calls_of(all: &[Vec<FnAt>]) -> CallsOf {
    all.iter().map(|fns| fns.iter().map(|f| { let mut c = Calls { out: Vec::new() }; c.visit_block(f.block); c.out }).collect()).collect()
}

/// A function of the sim crate or a helper of a body file that hands out a block to READ: named `…_read`, and what it hands
/// back is `&'static` and no `mut` (a function that only has `read` in its name is not one). It is clean by its name: the taint does not look
/// inside it (its `unsafe` is the shared twin of the `mut` one's). It is clean by its name: the taint does not look inside it (its `unsafe` is the shared twin of the `mut` one's).
fn is_read_twin(name: &str, ret: &str) -> bool {
    name.ends_with("_read") && ret.contains("&'static") && !ret.contains("mut")
}

/// The sim crate's files: (its module as the bodies write it, `sim::doors`; the text).
fn sim_files(root: &Path) -> Vec<(String, String)> {
    let mut all = Vec::new();
    rs_files(&root.join("rust/sim/src"), &mut all);
    all.into_iter().filter_map(|p| {
        let stem = p.file_stem().and_then(|n| n.to_str()).unwrap_or("").to_string();
        std::fs::read_to_string(&p).ok().map(|t| (if stem == "lib" { "sim".to_string() } else { format!("sim::{stem}") }, t))
    }).collect()
}

/// What can write without `&mut`: a function that is `unsafe` or names a static or a cell, or hands back a `&mut` (so every
/// function that hands out a block of the sim behind a slot number); and, to a fixed point, everything that uses one. The sim
/// crate's own functions are followed like the bodies', so a call into the sim taints only when what it calls can write.
/// A use is read off the function's own words, inside a macro's too: `f` its own module's (else any module's of that name),
/// `m::f` module m's, `sim::m::f` the sim's, `T::f` and `x.f(…)` any method of that name. A `macro_rules!` of the file is
/// followed like a function. A call into the sim that this cannot find is taken for a write. It errs toward the taint.
fn tainted_fns(root: &Path, files: &[(PathBuf, String, String)], parsed: &[syn::File], all: &[Vec<FnAt>]) -> HashSet<(String, String)> {
    use crate::testcut::{uses, Via};
    struct Node {
        module: String,
        name: String,
        file: usize,
        uses: HashSet<(Via, String, bool)>,
        text: String,
        tainted: bool,
    }
    let dirty = |text: &str| TAINT.iter().any(|t| text.contains(t));
    let mut nodes: Vec<Node> = Vec::new();
    let mut fn_keys: HashSet<(String, String)> = HashSet::new();
    let mut methods: HashSet<(String, String)> = HashSet::new();
    let mut modules: HashSet<String> = files.iter().map(|f| module_of(&f.1)).collect();
    for (fi, fns) in all.iter().enumerate() {
        let src = &files[fi].2;
        let m = module_of(&files[fi].1);
        for f in fns {
            let body = &src[f.block.span().byte_range()];
            let ret = match &f.sig.output {
                syn::ReturnType::Type(_, t) => src[t.span().byte_range()].to_string(),
                _ => String::new(),
            };
            fn_keys.insert((m.clone(), f.name.clone()));
            if f.method {
                methods.insert((m.clone(), f.name.clone()));
            }
            let clean = is_read_twin(&f.name, &ret);
            nodes.push(Node { module: m.clone(), name: f.name.clone(), file: fi, uses: if clean { HashSet::new() } else { uses(body) }, text: if clean { String::new() } else { body.to_string() }, tainted: !clean && (ret.contains("mut") || dirty(body)) });
        }
        // the file's own macros: `macro_rules! name { … }`, as a function named `name!`
        fn macros(items: &[syn::Item], src: &str, out: &mut Vec<(String, String)>) {
            for it in items {
                match it {
                    syn::Item::Macro(x) => {
                        if let Some(id) = &x.ident {
                            out.push((format!("{id}!"), src[it.span().byte_range()].to_string()));
                        }
                    }
                    syn::Item::Mod(md) => {
                        if let Some((_, inner)) = &md.content {
                            macros(inner, src, out);
                        }
                    }
                    _ => {}
                }
            }
        }
        let mut ms = Vec::new();
        macros(&parsed[fi].items, src, &mut ms);
        for (name, text) in ms {
            nodes.push(Node { module: m.clone(), name, file: fi, uses: uses(&text), tainted: dirty(&text), text });
        }
    }
    // the sim crate: every function of its files (its tests left out), under `sim::file`
    let sims = sim_files(root);
    let sim_parsed: Vec<Option<syn::File>> = sims.iter().map(|(_, t)| syn::parse_file(t).ok()).collect();
    for (si, (m, src)) in sims.iter().enumerate() {
        modules.insert(m.clone());
        let Some(file) = &sim_parsed[si] else { continue };
        for f in fns_of(src, file) {
            let body = &src[f.block.span().byte_range()];
            let ret = match &f.sig.output {
                syn::ReturnType::Type(_, t) => src[t.span().byte_range()].to_string(),
                _ => String::new(),
            };
            fn_keys.insert((m.clone(), f.name.clone()));
            if f.method {
                methods.insert((m.clone(), f.name.clone()));
            }
            let clean = is_read_twin(&f.name, &ret);
            nodes.push(Node { module: m.clone(), name: f.name.clone(), file: files.len() + si, uses: if clean { HashSet::new() } else { uses(body) }, text: if clean { String::new() } else { body.to_string() }, tainted: !clean && (ret.contains("mut") || dirty(body)) });
        }
    }
    // the macros of each file, to look for `name!` in a body
    let mut file_macros: HashMap<usize, Vec<usize>> = HashMap::new();
    for (i, n) in nodes.iter().enumerate() {
        if n.name.ends_with('!') {
            file_macros.entry(n.file).or_default().push(i);
        }
    }
    loop {
        let tainted: HashSet<(String, String)> = nodes.iter().filter(|n| n.tainted).map(|n| (n.module.clone(), n.name.clone())).collect();
        // (a bare call of a function brought in by a `use`: any of that name. A body may bring in the sim's; the sim only its own)
        let free_names: HashSet<&String> = tainted.iter().filter(|k| !methods.contains(*k)).map(|k| &k.1).collect();
        let sim_names: HashSet<&String> = tainted.iter().filter(|k| !methods.contains(*k) && k.0.starts_with("sim")).map(|k| &k.1).collect();
        let method_names: HashSet<&String> = tainted.iter().filter(|k| methods.contains(*k)).map(|k| &k.1).collect();
        let mut grew: Vec<usize> = Vec::new();
        for (i, n) in nodes.iter().enumerate() {
            if n.tainted {
                continue;
            }
            let in_sim = n.module.starts_with("sim");
            let by_macro = file_macros.get(&n.file).map(|v| v.iter().any(|&j| nodes[j].tainted && j != i && n.text.contains(nodes[j].name.as_str()))).unwrap_or(false);
            // (a bare word that is not called is one of the module's own functions handed on as a value, or a local: only the first counts)
            let by_use = n.uses.iter().any(|(via, name, called)| match via {
                Via::Dot => *called && method_names.contains(name),
                Via::Path(segs) => {
                    let q = segs.last().map(|s| s.as_str()).unwrap_or("");
                    // which module the path names: the function's own, one of the sim's, one of the game's
                    // (the sim's first: a body of `doors` that writes `sim::doors::update` means the sim's, not its own module's)
                    let target: Option<String> = if segs.iter().any(|s| s == "sim") || (in_sim && modules.contains(&format!("sim::{q}"))) {
                        Some(if q == "sim" { "sim".to_string() } else { format!("sim::{q}") })
                    } else if q == "super" || q == "self" || q == n.module.rsplit("::").next().unwrap_or("") {
                        Some(n.module.clone())
                    } else if in_sim && q == "crate" {
                        Some("sim".to_string())
                    } else if modules.contains(q) {
                        Some(q.to_string())
                    } else {
                        None
                    };
                    match target {
                        Some(m) => {
                            let k = (m, name.clone());
                            // (a call into the sim that is no function this found there: taken for a write)
                            if fn_keys.contains(&k) { tainted.contains(&k) } else { *called && k.0.starts_with("sim") && !in_sim }
                        }
                        None => method_names.contains(name),
                    }
                }
                Via::Bare => {
                    let own = (n.module.clone(), name.clone());
                    if fn_keys.contains(&own) { tainted.contains(&own) } else { *called && if in_sim { sim_names.contains(name) } else { free_names.contains(name) } }
                }
            });
            if by_macro || by_use {
                grew.push(i);
            }
        }
        if grew.is_empty() {
            return tainted.into_iter().filter(|k| !k.1.ends_with('!') && !k.0.starts_with("sim")).collect();
        }
        for i in grew {
            nodes[i].tainted = true;
        }
    }
}

pub struct Restored {
    pub by_error: usize,
    pub by_calls: usize,
    pub by_taint: usize,
    pub unplaced: Vec<String>,
    /// parameters put back a whole file at a time, where an error could not be placed on a function
    pub coarse: usize,
    pub left: usize,
    /// softened sites made `mut` again, and how many still stand
    pub soft_back: usize,
    pub soft_left: usize,
}

/// Put `mut` back where the compiler's errors (`diag`, cargo's JSON) and the calls say. With `taint`, also on
/// every function that can write past the compiler's sight and on every function that calls one.
pub fn restore(root: &Path, state: &Path, diag: Option<&str>, taint: bool) -> Result<Restored, String> {
    let mut demoted = load_state(state);
    let mut soft = load_state(&soft_path(state));
    let files = pristine(root, state)?;
    let parsed: Vec<syn::File> = files.iter().map(|(_, rel, src)| syn::parse_file(src).map_err(|e| format!("{rel} does not parse: {e}"))).collect::<Result<_, _>>()?;
    let mut back: BTreeSet<String> = BTreeSet::new();
    let mut out = Restored { by_error: 0, by_calls: 0, by_taint: 0, unplaced: Vec::new(), coarse: 0, left: 0, soft_back: 0, soft_left: 0 };
    // the softened sites that still stand, by file and line
    let mut soft_at: HashMap<(usize, usize), Vec<String>> = HashMap::new();
    let twins = if soft.is_empty() { Twins::new() } else { twins_of(root, &files) };
    if !soft.is_empty() {
        for (fi, (_, rel, src)) in files.iter().enumerate() {
            for st in sites_of(src, &parsed[fi], &module_of(rel), &twins) {
                let k = soft_key(rel, st.off);
                if soft.contains(&k) {
                    soft_at.entry((fi, st.line)).or_default().push(k);
                }
            }
        }
    }
    let mut soft_back: BTreeSet<String> = BTreeSet::new();
    // each file's own macros: (name, its lines), for a softened place inside one
    let mut macros_of: Vec<Vec<(String, (usize, usize))>> = Vec::new();
    for (fi, (_, _, src)) in files.iter().enumerate() {
        fn walk(items: &[syn::Item], src: &str, out: &mut Vec<(String, (usize, usize))>) {
            for it in items {
                match it {
                    syn::Item::Macro(m) => {
                        if let Some(id) = &m.ident {
                            let r = it.span().byte_range();
                            out.push((id.to_string(), (line_of(src, r.start), line_of(src, r.end))));
                        }
                    }
                    syn::Item::Mod(m) => {
                        if let Some((_, inner)) = &m.content {
                            walk(inner, src, out);
                        }
                    }
                    _ => {}
                }
            }
        }
        let mut v = Vec::new();
        walk(&parsed[fi].items, src, &mut v);
        macros_of.push(v);
    }
    let src_lines: Vec<Vec<&str>> = files.iter().map(|f| f.2.lines().collect()).collect();
    // every function, by file
    let all: Vec<Vec<FnAt>> = files.iter().zip(parsed.iter()).map(|((_, _, src), f)| fns_of(src, f)).collect();
    let rel_ix: HashMap<&str, usize> = files.iter().enumerate().map(|(i, (_, rel, _))| (rel.as_str(), i)).collect();
    let mut by_name: HashMap<&str, Vec<(usize, usize)>> = HashMap::new();
    for (fi, fns) in all.iter().enumerate() {
        for (k, f) in fns.iter().enumerate() {
            by_name.entry(f.name.as_str()).or_default().push((fi, k));
        }
    }
    // (a span of an error inside a macro is the macro's own text: the place it was called from is what the error is about)
    fn outer(s: &serde_json::Value) -> &serde_json::Value {
        let mut s = s;
        while !s["expansion"].is_null() && !s["expansion"]["span"].is_null() {
            s = &s["expansion"]["span"];
        }
        s
    }
    fn in_game(file: &str) -> &str {
        file.rfind("rust/game/").map(|i| &file[i + "rust/game/".len()..]).unwrap_or(file)
    }
    let demoted_params = |fi: usize, f: &FnAt, demoted: &BTreeSet<String>, back: &BTreeSet<String>| -> Vec<String> {
        ref_params(f.sig).into_iter().filter(|(_, p, _, _)| { let k = key(&files[fi].1, &f.name, f.ord, p); demoted.contains(&k) && !back.contains(&k) }).map(|(_, p, _, _)| p).collect()
    };

    // 1. what the compiler named
    if let Some(text) = diag {
        for l in text.lines() {
            let Ok(v) = serde_json::from_str::<serde_json::Value>(l) else { continue };
            if v["reason"] != "compiler-message" || v["message"]["level"] != "error" {
                continue;
            }
            let msg = v["message"]["message"].as_str().unwrap_or("");
            if msg.starts_with("aborting due to") || msg.starts_with("could not compile") {
                continue;
            }
            let Some(spans) = v["message"]["spans"].as_array() else { continue };
            let Some(s) = spans.iter().find(|s| s["is_primary"] == true) else { continue };
            let s = outer(s);
            // (a path into the game crate, however the check that met it was started)
            let file = in_game(s["file_name"].as_str().unwrap_or(""));
            let line = s["line_start"].as_u64().unwrap_or(0) as usize;
            let (hs, he) = (s["text"][0]["highlight_start"].as_u64().unwrap_or(1) as usize, s["text"][0]["highlight_end"].as_u64().unwrap_or(1) as usize);
            let shown: String = s["text"][0]["text"].as_str().unwrap_or("").chars().skip(hs.saturating_sub(1)).take(he.saturating_sub(hs)).collect();
            // the place the error names, in backticks: `*g`, `g.time`, `s.list`
            let named: Vec<String> = msg.split('`').skip(1).step_by(2).flat_map(|t| idents_in(t).into_iter().take(1)).collect();
            let mut placed = false;
            // 0. a softened site on a line the error points at (its own, a note's, the macro's own text): the site is `mut` again,
            //    and that is the whole answer to this error for this round
            if !soft_at.is_empty() {
                let notes = v["message"]["children"].as_array().map(|c| c.iter().flat_map(|c| c["spans"].as_array().cloned().unwrap_or_default()).collect::<Vec<_>>()).unwrap_or_default();
                let mut at: Vec<(usize, usize)> = Vec::new();
                for sp in spans.iter().chain(notes.iter()) {
                    for one in [sp, outer(sp)] {
                        if let Some(&fj) = rel_ix.get(in_game(one["file_name"].as_str().unwrap_or(""))) {
                            let (a, b) = (one["line_start"].as_u64().unwrap_or(0) as usize, one["line_end"].as_u64().unwrap_or(0) as usize);
                            for l in a..=b.max(a) {
                                at.push((fj, l));
                            }
                        }
                    }
                }
                for k in at.iter().filter_map(|k| soft_at.get(k)).flatten() {
                    soft_back.insert(k.clone());
                    placed = true;
                }
                // a macro of the file called on one of those lines (`let st = stand!(g, gd);` and then `st.to = …`): the softened
                // places inside that macro
                if !placed {
                    for &(fj, l) in &at {
                        let text = src_lines[fj].get(l.wrapping_sub(1)).copied().unwrap_or("");
                        for (name, (a, b)) in &macros_of[fj] {
                            if text.contains(&format!("{name}!")) {
                                for ml in *a..=*b {
                                    if let Some(ks) = soft_at.get(&(fj, ml)) {
                                        for k in ks {
                                            soft_back.insert(k.clone());
                                            placed = true;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                if placed {
                    continue;
                }
            }
            if let Some(&fi) = rel_ix.get(file) {
                if let Some(f) = all[fi].iter().find(|f| f.lines.0 <= line && line <= f.lines.1) {
                    // (a parameter an earlier error of this round already put back is placed too)
                    let ps = demoted_params(fi, f, &demoted, &BTreeSet::new());
                    let mut hit: Vec<&String> = ps.iter().filter(|p| named.contains(p)).collect();
                    if hit.is_empty() {
                        let in_span = idents_in(&shown);
                        hit = ps.iter().filter(|p| in_span.contains(p)).collect();
                    }
                    if hit.is_empty() && !soft_at.is_empty() {
                        // the error is about a local, not a parameter: every softened site of the function is `mut` again
                        let mut any = false;
                        for l in f.lines.0..=f.lines.1 {
                            if let Some(ks) = soft_at.get(&(fi, l)) {
                                for k in ks {
                                    soft_back.insert(k.clone());
                                    any = true;
                                }
                            }
                        }
                        // none in the function: then in a macro of the file it calls
                        if !any {
                            let body = src_lines[fi][f.lines.0 - 1..f.lines.1.min(src_lines[fi].len())].join("\n");
                            for (name, (a, b)) in &macros_of[fi] {
                                if body.contains(&format!("{name}!")) {
                                    for ml in *a..=*b {
                                        if let Some(ks) = soft_at.get(&(fi, ml)) {
                                            for k in ks {
                                                soft_back.insert(k.clone());
                                                any = true;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        if any {
                            continue;
                        }
                    }
                    if hit.is_empty() {
                        hit = ps.iter().collect();
                    }
                    for p in hit {
                        back.insert(key(file, &f.name, f.ord, p));
                        placed = true;
                    }
                    // a function handed on as a value where a `&mut` one is wanted: the function named in the span
                    for n in idents_in(&shown) {
                        if let Some(list) = by_name.get(n.as_str()) {
                            for &(fj, k) in list {
                                for p in demoted_params(fj, &all[fj][k], &demoted, &back) {
                                    back.insert(key(&files[fj].1, &all[fj][k].name, all[fj][k].ord, &p));
                                    placed = true;
                                }
                            }
                        }
                    }
                }
            }
            // a span of the error (its own, a note's, a help's) that lies on a function's signature: that function is
            // what was handed on as a value where a `&mut` one is wanted ("found signature defined here")
            let notes = v["message"]["children"].as_array().map(|c| c.iter().flat_map(|c| c["spans"].as_array().cloned().unwrap_or_default()).collect::<Vec<_>>()).unwrap_or_default();
            for sp in spans.iter().chain(notes.iter()) {
                if sp["is_primary"] == true && msg.contains("cannot") {
                    continue;
                }
                let sf = in_game(sp["file_name"].as_str().unwrap_or(""));
                let sl = sp["line_start"].as_u64().unwrap_or(0) as usize;
                let label = sp["label"].as_str().unwrap_or("");
                if !label.contains("signature") {
                    continue;
                }
                if let Some(&fj) = rel_ix.get(sf) {
                    if let Some(f) = all[fj].iter().find(|f| f.sig_lines.0 <= sl && sl <= f.sig_lines.1) {
                        for p in demoted_params(fj, f, &demoted, &BTreeSet::new()) {
                            back.insert(key(sf, &f.name, f.ord, &p));
                            placed = true;
                        }
                    }
                }
            }
            // a function handed on as a value, named whole in a note: "found fn item `… {control::runner::transition}`"
            {
                let mut texts: Vec<&str> = vec![msg];
                if let Some(c) = v["message"]["children"].as_array() {
                    texts.extend(c.iter().filter_map(|c| c["message"].as_str()));
                }
                for t in texts {
                    let mut rest = t;
                    while let Some(i) = rest.find('{') {
                        let Some(j) = rest[i..].find('}') else { break };
                        let path: Vec<&str> = rest[i + 1..i + j].split("::").collect();
                        rest = &rest[i + j + 1..];
                        if path.len() < 2 || !path.iter().all(|p| !p.is_empty() && p.chars().all(|c| c.is_alphanumeric() || c == '_')) {
                            continue;
                        }
                        let (name, q) = (path[path.len() - 1], path[path.len() - 2]);
                        if let Some(list) = by_name.get(name) {
                            for &(fj, k) in list {
                                let rel = &files[fj].1;
                                let stem = Path::new(rel).file_stem().and_then(|n| n.to_str()).unwrap_or("");
                                if stem == q || module_of(rel) == q {
                                    for p in demoted_params(fj, &all[fj][k], &demoted, &BTreeSet::new()) {
                                        back.insert(key(rel, &all[fj][k].name, all[fj][k].ord, &p));
                                        placed = true;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            if !placed {
                // outside a body function (a table in a generated file, another crate): every function it names
                for n in idents_in(&shown).into_iter().chain(msg.split('`').skip(1).step_by(2).flat_map(idents_in)) {
                    if let Some(list) = by_name.get(n.as_str()) {
                        for &(fj, k) in list {
                            for p in demoted_params(fj, &all[fj][k], &demoted, &back) {
                                back.insert(key(&files[fj].1, &all[fj][k].name, all[fj][k].ord, &p));
                                placed = true;
                            }
                        }
                    }
                }
            }
            if !placed {
                // the last resort, so that a run always ends: every parameter of the file the error stands in
                if let Some(&fi) = rel_ix.get(file) {
                    for f in &all[fi] {
                        for p in demoted_params(fi, f, &demoted, &back) {
                            back.insert(key(file, &f.name, f.ord, &p));
                            out.coarse += 1;
                            placed = true;
                        }
                    }
                }
            }
            if !placed {
                out.unplaced.push(format!("{file}:{line} {}", msg.chars().take(160).collect::<String>()));
            }
        }
        out.by_error = back.len();
    }

    // the calls of every function, for 2 and 3
    let calls = calls_of(&all);

    // 2. what can write without `&mut`: a function that names `sim::` (the block behind the slot), a static or a
    //    cell, or hands back a `&mut`; and, to a fixed point, every function that calls one. By (module, name): two
    //    functions of a module that share a name taint each other.
    if taint {
        let tainted = tainted_fns(root, &files, &parsed, &all);
        for (fi, fns) in all.iter().enumerate() {
            let m = module_of(&files[fi].1);
            for f in fns {
                if tainted.contains(&(m.clone(), f.name.clone())) {
                    for p in demoted_params(fi, f, &demoted, &back) {
                        back.insert(key(&files[fi].1, &f.name, f.ord, &p));
                        out.by_taint += 1;
                    }
                }
            }
        }
    }

    // 3. who hands a parameter on to a `&mut` parameter: to a fixed point
    //    (module, name) -> for each position, does the function ask `&mut` there (as the files stand, plus `back`)
    loop {
        let mut asks: HashMap<(String, String), Vec<bool>> = HashMap::new();
        for (fi, fns) in all.iter().enumerate() {
            let m = module_of(&files[fi].1);
            for f in fns {
                let mut v = vec![false; f.sig.inputs.len()];
                for (i, p, _, is_mut) in ref_params(f.sig) {
                    let k = key(&files[fi].1, &f.name, f.ord, &p);
                    // (`mut` as the pass has it now: written so before, and not made `&` since, or put back in this round)
                    v[i] = is_mut && (!demoted.contains(&k) || back.contains(&k));
                }
                let e = asks.entry((m.clone(), f.name.clone())).or_insert_with(|| vec![false; v.len()]);
                for (i, b) in v.iter().enumerate() {
                    if i < e.len() {
                        e[i] |= *b;
                    } else {
                        e.push(*b);
                    }
                }
            }
        }
        let mut grew = false;
        for (fi, fns) in all.iter().enumerate() {
            let m = module_of(&files[fi].1);
            for (k, f) in fns.iter().enumerate() {
                let ps = demoted_params(fi, f, &demoted, &back);
                if ps.is_empty() {
                    continue;
                }
                for (cm, cn, args) in &calls[fi][k] {
                    let target = match cm.as_deref() {
                        None | Some("super") | Some("self") => (m.clone(), cn.clone()),
                        Some(x) => (x.to_string(), cn.clone()),
                    };
                    let Some(want) = asks.get(&target) else { continue };
                    for (i, a) in args.iter().enumerate() {
                        if let Some(a) = a {
                            if want.get(i).copied().unwrap_or(false) && ps.contains(a) {
                                if back.insert(key(&files[fi].1, &f.name, f.ord, a)) {
                                    grew = true;
                                    out.by_calls += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
        if !grew {
            break;
        }
    }

    // the files as the pass has them now
    for k in &back {
        demoted.remove(k);
    }
    for k in &soft_back {
        soft.remove(k);
    }
    for (fi, (path, rel, src)) in files.iter().enumerate() {
        let now = render(src, &parsed[fi], rel, &demoted, &soft, &twins);
        if std::fs::read_to_string(path).map(|t| t != now).unwrap_or(true) {
            std::fs::write(path, now).map_err(|e| format!("cannot write {rel}: {e}"))?;
        }
    }
    out.left = demoted.len();
    out.soft_back = soft_back.len();
    out.soft_left = soft.len();
    save_state(state, &demoted);
    save_state(&soft_path(state), &soft);
    Ok(out)
}

/// Readers by module, and the rows of the call surface whose function no longer asks `&mut Game`.
/// The functions that take `&Game` as the files stand: `module::name`, one a line.
pub fn readers_now(root: &Path) -> Result<Vec<String>, String> {
    let mut out = Vec::new();
    for (_, rel, src) in bodies(root) {
        let file = syn::parse_file(&src).map_err(|e| format!("{rel} does not parse: {e}"))?;
        let module = Path::new(&rel).parent().and_then(|d| d.file_name()).and_then(|n| n.to_str()).unwrap_or("").to_string();
        for f in fns_of(&src, &file) {
            for a in &f.sig.inputs {
                let FnArg::Typed(t) = a else { continue };
                let Type::Reference(r) = &*t.ty else { continue };
                if let Type::Path(p) = &*r.elem {
                    if p.path.segments.last().map(|s| s.ident == "Game").unwrap_or(false) && r.mutability.is_none() {
                        out.push(format!("{module}::{}", f.name));
                    }
                }
            }
        }
    }
    out.sort();
    out.dedup();
    Ok(out)
}

/// The report, and (with `calls`, the text of tools/blitz/calls.json) the rows of the call surface whose function
/// cannot write the game, as JSON: `{"rows": [key, …]}`.
/// The keys of the contract file: `rows` holds a key as a string, or a row with its `key`.
fn contract_keys(c: &serde_json::Value) -> Vec<String> {
    let Some(rows) = c["rows"].as_array() else { return Vec::new() };
    rows.iter().filter_map(|r| r.as_str().or_else(|| r["key"].as_str())).map(str::to_string).collect()
}

pub fn report(root: &Path, state: &Path, bare: Option<&str>, calls_json: Option<&str>) -> Result<(String, Option<String>), String> {
    use std::fmt::Write as _;
    let demoted = load_state(state);
    let files = bodies(root);
    let parsed: Vec<syn::File> = files.iter().map(|(_, rel, src)| syn::parse_file(src).map_err(|e| format!("{rel} does not parse: {e}"))).collect::<Result<_, _>>()?;
    let all: Vec<Vec<FnAt>> = files.iter().zip(parsed.iter()).map(|((_, _, src), f)| fns_of(src, f)).collect();
    let tainted = tainted_fns(root, &files, &parsed, &all);
    // by module: functions, taking the game, of those `&Game`, of those sure readers
    let mut by_module: BTreeMap<String, (usize, usize, usize, usize)> = BTreeMap::new();
    // a name is a reader only if every function of that module and name is one
    let mut readers: HashSet<(String, String)> = HashSet::new();
    let mut not_readers: HashSet<(String, String)> = HashSet::new();
    let mut lying: Vec<String> = Vec::new();
    let mut no_game: HashSet<(String, String)> = HashSet::new();
    for (fi, fns) in all.iter().enumerate() {
        let module = module_of(&files[fi].1);
        for f in fns {
            let e = by_module.entry(module.clone()).or_default();
            e.0 += 1;
            let mut game: Option<bool> = None;
            let mut any_mut = false;
            for a in &f.sig.inputs {
                let FnArg::Typed(t) = a else { any_mut |= matches!(a, FnArg::Receiver(r) if r.mutability.is_some()); continue };
                // (a `mut` anywhere in a parameter's type: `&mut T`, `Option<&mut T>`, `&mut dyn FnMut(…)`)
                any_mut |= files[fi].2[t.ty.span().byte_range()].contains("mut ");
                let Type::Reference(r) = &*t.ty else { continue };
                any_mut |= r.mutability.is_some();
                if let Type::Path(p) = &*r.elem {
                    if p.path.segments.last().map(|s| s.ident == "Game").unwrap_or(false) {
                        game = Some(r.mutability.is_some());
                    }
                }
            }
            let k = (module.clone(), f.name.clone());
            let taint = tainted.contains(&k);
            match game {
                Some(is_mut) => {
                    e.1 += 1;
                    if !is_mut {
                        e.2 += 1;
                        if taint {
                            lying.push(format!("{module}::{}", f.name));
                        }
                    }
                    if !is_mut && !any_mut && !taint {
                        e.3 += 1;
                        readers.insert(k);
                    } else {
                        not_readers.insert(k);
                    }
                }
                None => {
                    if any_mut || taint {
                        not_readers.insert(k);
                    } else {
                        no_game.insert(k);
                    }
                }
            }
        }
    }
    lying.sort();
    lying.dedup();
    let mut o = String::new();
    let t = by_module.values().fold((0, 0, 0, 0), |a, b| (a.0 + b.0, a.1 + b.1, a.2 + b.2, a.3 + b.3));
    let _ = writeln!(o, "# Who writes the game, by signature\n");
    let _ = writeln!(o, "functions in body files: {}; taking the game: {}; of those `&Game`: {} ({}%)", t.0, t.1, t.2, t.2 * 100 / t.1.max(1));
    let _ = writeln!(o, "SURE READERS (take `&Game`, no `&mut` parameter of any type, and neither name `sim::`, a static or a cell nor call a function that does): {} ({}%)", t.3, t.3 * 100 / t.1.max(1));
    let _ = writeln!(o, "`&mut` parameters of any type made `&` and held: {}", demoted.len());
    let _ = writeln!(o, "\n## By module: functions taking the game, `&Game` among them, sure readers among those\n");
    let mut v: Vec<_> = by_module.iter().filter(|(_, e)| e.1 > 0).collect();
    v.sort_by_key(|(_, e)| std::cmp::Reverse(e.1));
    for (m, e) in v {
        let _ = writeln!(o, "- {m}: {} take the game, {} `&Game`, {} sure readers ({}%)", e.1, e.2, e.3, e.3 * 100 / e.1.max(1));
    }
    let _ = writeln!(o, "\n## Take `&Game` and can write past it ({})\n", lying.len());
    let _ = writeln!(o, "Each names `sim::` (the block behind the game's slot), a static or a cell, or calls a function that does. The taint is by the word,\nso a pure query of `sim::` is here too: the signature says reader, this program cannot.\n");
    let _ = writeln!(o, "{}", lying.join(", "));
    // the call surface: a row of calls.json is `"rust": "crate::module::name"` under its key
    let mut reads_json = None;
    if let Some(text) = calls_json {
        let v: serde_json::Value = serde_json::from_str(text).map_err(|e| format!("calls.json: {e}"))?;
        let rows = v["rows"].as_array().ok_or("calls.json has no rows")?;
        let (mut keys, mut pure_keys, mut with_game) = (Vec::new(), Vec::new(), 0);
        for r in rows {
            let (Some(key), Some(rust)) = (r["key"].as_str(), r["rust"].as_str()) else { continue };
            let mut it = rust.trim_start_matches("crate::").split("::");
            let k = (it.next().unwrap_or("").to_string(), it.next().unwrap_or("").to_string());
            if readers.contains(&k) || not_readers.contains(&k) {
                with_game += 1;
            }
            if readers.contains(&k) && !not_readers.contains(&k) {
                keys.push(key.to_string());
            } else if no_game.contains(&k) && !not_readers.contains(&k) && !readers.contains(&k) {
                pure_keys.push(key.to_string());
            }
        }
        let _ = writeln!(o, "\n## Rows of the call surface whose function is a sure reader ({} of {} rows; {} more take no game and write nothing)\n", keys.len(), rows.len(), pure_keys.len());
        let _ = writeln!(o, "{}\n\n### Take no game, no `&mut`, and name no `sim::`, static or cell ({})\n\n{}", keys.join(", "), pure_keys.len(), pure_keys.join(", "));
        let _ = with_game;
        let mut all_keys: Vec<&String> = keys.iter().chain(pure_keys.iter()).collect();
        all_keys.sort();
        let note = "The keys of calls.json's rows whose Rust function cannot write the game: it takes the game by a shared borrow, has no `&mut` anywhere in its parameters, and neither it nor anything it uses is unsafe, names a static or a cell, or is handed a block of the sim to write (the compiler's proof, then the taint of tools/fold/readers.sh). After a crossing of one the host exports and applies nothing. Written by `tools/fold/readers.sh --reads`: run it again when bodies change.";
        reads_json = Some(format!("{{\"_\": {}, \"rows\": [\n{}\n]}}\n", serde_json::to_string(note).unwrap(), all_keys.iter().map(|k| format!("  {}", serde_json::to_string(k).unwrap())).collect::<Vec<_>>().join(",\n")));
        // rows that read BY CONTRACT (tools/blitz/reads-contract.json, kept by hand): one this pass now proves has its place in the
        // file written here and leaves that one
        if let Ok(text) = std::fs::read_to_string(root.join("tools/blitz/reads-contract.json")) {
            let c: serde_json::Value = serde_json::from_str(&text).map_err(|e| format!("reads-contract.json: {e}"))?;
            let proved = contract_keys(&c).into_iter().filter(|k| all_keys.iter().any(|a| *a == k)).collect::<Vec<_>>();
            if !proved.is_empty() {
                let _ = writeln!(o, "\n## PROVED NOW: take these rows out of tools/blitz/reads-contract.json ({})\n\nEach read by contract; the compiler and the taint now say the same, so the row is in the reads file this pass writes.\n\n{}", proved.len(), proved.join(", "));
            }
        }
    }
    // the compiler's bare answer, taken before the taint: a map of who touches the Game struct, not a proof of who moves nothing
    if let Some(b) = bare {
        let names: Vec<&str> = b.lines().filter(|l| !l.is_empty()).collect();
        let only: Vec<&str> = names.iter().copied().filter(|n| { let mut it = n.split("::"); let k = (it.next().unwrap_or("").to_string(), it.next().unwrap_or("").to_string()); !readers.contains(&k) }).collect();
        let _ = writeln!(o, "\n## The compiler's bare answer: `&Game` before the taint ({}), of which {} are not sure readers\n", names.len(), only.len());
        let _ = writeln!(o, "The second number is the functions that touch no field of the Game struct for writing, yet take another `&mut`, name `sim::`, a static\nor a cell, or call one that does.\n");
        let _ = writeln!(o, "{}", only.join(", "));
    }
    Ok((o, reads_json))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// a small tree under the system's temp folder: (path from the root, text)
    fn tree(name: &str, files: &[(&str, &str)]) -> PathBuf {
        let root = std::env::temp_dir().join(format!("fold-readers-test-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        for (p, t) in files {
            let f = root.join(p);
            std::fs::create_dir_all(f.parent().unwrap()).unwrap();
            std::fs::write(f, t).unwrap();
        }
        root
    }

    fn taint_of(root: &Path) -> HashSet<(String, String)> {
        let files = bodies(root);
        let parsed: Vec<syn::File> = files.iter().map(|(_, _, src)| syn::parse_file(src).unwrap()).collect();
        let all: Vec<Vec<FnAt>> = files.iter().zip(parsed.iter()).map(|((_, _, src), f)| fns_of(src, f)).collect();
        tainted_fns(root, &files, &parsed, &all)
    }

    const SIM_DOORS: &str = "static mut IO: [f64; 4] = [0.0; 4];\nfn io() -> &'static mut [f64; 4] { unsafe { &mut *core::ptr::addr_of_mut!(IO) } }\npub fn update(g: usize, w: usize) -> u32 { let o = io(); o[0] = 1.0; 0 }\npub fn cap(t: u32) -> usize { 4 }\n";
    const SIM_STATE: &str = "pub struct State { pub doors: [f64; 4] }\nstatic mut ALL: [State; 1] = [State { doors: [0.0; 4] }];\npub fn state(g: usize) -> &'static mut State { unsafe { &mut (*core::ptr::addr_of_mut!(ALL))[g] } }\npub fn state_read(g: usize) -> &'static State { unsafe { &(*core::ptr::addr_of!(ALL))[g] } }\n";

    fn has(t: &HashSet<(String, String)>, m: &str, f: &str) -> bool {
        t.contains(&(m.to_string(), f.to_string()))
    }

    /// The case that got through once: the game's module `doors` steps the sim's `doors`. The path names the sim's function, not
    /// one of the module's own, and the step writes the block: the caller is no reader, whatever its parameter says.
    #[test]
    fn a_step_of_the_sim_taints_its_caller_in_a_module_of_the_same_name() {
        let root = tree("step", &[
            ("rust/game/src/doors/p_01.rs", "pub fn doors_update_here(g: &Game, dt: f64) { let w = g.slot; let _ = ::sim::doors::update(g.sim, w); }\npub fn door_cap(g: &Game) -> usize { ::sim::doors::cap(5) }\npub fn door_angle(g: &Game, i: usize) -> f64 { ::sim::state::state_read(g.sim).doors[i] }\npub fn door_set(g: &Game, i: usize, v: f64) { ::sim::state::state(g.sim).doors[i] = v; }\npub fn door_step_twice(g: &Game) { doors_update_here(g, 0.0); }\n"),
            ("rust/sim/src/doors.rs", SIM_DOORS),
            ("rust/sim/src/state.rs", SIM_STATE),
        ]);
        let t = taint_of(&root);
        assert!(has(&t, "doors", "doors_update_here"), "a step of the sim is a write");
        assert!(has(&t, "doors", "door_step_twice"), "and so is calling one that steps it");
        assert!(has(&t, "doors", "door_set"), "the block handed out to write taints");
        assert!(!has(&t, "doors", "door_cap"), "a function of the sim that touches nothing is clean");
        assert!(!has(&t, "doors", "door_angle"), "the reading twin is clean");
        let _ = std::fs::remove_dir_all(&root);
    }

    /// A function of the sim brought in by a `use` and called by its bare name is followed too.
    #[test]
    fn a_bare_call_of_an_imported_step_taints() {
        let root = tree("bare", &[
            ("rust/game/src/props/p_01.rs", "use ::sim::doors::update;\npub fn props_step(g: &Game) { update(g.sim, 0); }\npub fn props_count(g: &Game) -> usize { 3 }\n"),
            ("rust/sim/src/doors.rs", SIM_DOORS),
        ]);
        let t = taint_of(&root);
        assert!(has(&t, "props", "props_step"));
        assert!(!has(&t, "props", "props_count"));
        let _ = std::fs::remove_dir_all(&root);
    }

    /// `read` in a name is no licence: only a function that hands back `&'static` and no `mut` is a reading twin.
    #[test]
    fn the_contract_file_gives_its_keys_as_strings_or_rows() {
        let c: serde_json::Value = serde_json::from_str(r#"{"_": "note", "rows": ["a", {"key": "b", "contract": "puts back what it takes", "check": "x"}, 3]}"#).unwrap();
        assert_eq!(contract_keys(&c), vec!["a".to_string(), "b".to_string()]);
        assert!(contract_keys(&serde_json::json!({})).is_empty());
    }

    #[test]
    fn a_function_only_named_read_is_no_twin() {
        let root = tree("named", &[
            ("rust/game/src/sensors/p_01.rs", "static mut N: f64 = 0.0;\npub fn sensors_read(g: &Game) -> f64 { unsafe { N += 1.0; N } }\npub fn lights_read(g: &Game) -> &'static [f64; 4] { unsafe { &*(0 as *const [f64; 4]) } }\npub fn first(g: &Game) -> f64 { lights_read(g)[0] }\npub fn tick(g: &Game) -> f64 { sensors_read(g) }\n"),
        ]);
        let t = taint_of(&root);
        assert!(has(&t, "sensors", "sensors_read"));
        assert!(has(&t, "sensors", "tick"));
        assert!(!has(&t, "sensors", "lights_read"));
        assert!(!has(&t, "sensors", "first"));
        let _ = std::fs::remove_dir_all(&root);
    }
}
