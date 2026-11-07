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
const TAINT: &[&str] = &["sim::state::state", "UnsafeCell", "unsafe ", "static mut", "AtomicU32", "AtomicUsize", "AtomicBool", "RefCell", "Cell<"];

fn is_body(p: &Path) -> bool {
    let n = p.file_name().and_then(|n| n.to_str()).unwrap_or("");
    (n.starts_with("p_") || n.starts_with("b_")) && n.ends_with(".rs")
}

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

/// Every `&mut T` parameter of a body function becomes `&T`. The state file lists what was demoted.
pub fn demote(root: &Path, state: &Path) -> Result<String, String> {
    let mut demoted: BTreeSet<String> = BTreeSet::new();
    let mut files = 0;
    for (path, rel, src) in bodies(root) {
        let file = syn::parse_file(&src).map_err(|e| format!("{rel} does not parse: {e}"))?;
        let mut cuts: Vec<usize> = Vec::new();
        for f in fns_of(&src, &file) {
            for (_, p, at, is_mut) in ref_params(f.sig) {
                if is_mut {
                    cuts.push(at);
                    demoted.insert(key(&rel, &f.name, f.ord, &p));
                }
            }
        }
        if cuts.is_empty() {
            continue;
        }
        cuts.sort();
        let mut s = src.clone();
        for at in cuts.into_iter().rev() {
            // `& mut T` or `&mut T`: the word and the blank after it
            // (`&mut T` loses `mut ` ; `&'a mut T` keeps the blank after its lifetime)
            let rest = &s[at..];
            let ws = rest.len() - rest.trim_start().len();
            if rest[ws..].starts_with("mut") {
                let after = &rest[ws + 3..];
                let ws2 = after.len() - after.trim_start().len();
                s.replace_range(at + ws..at + ws + 3 + ws2, "");
            }
        }
        std::fs::write(&path, s).map_err(|e| format!("cannot write {rel}: {e}"))?;
        files += 1;
    }
    save_state(state, &demoted);
    Ok(format!("{} `&mut` parameters made `&` in {files} body files", demoted.len()))
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

/// What can write without `&mut`: a function that names `sim::` (the block behind the slot), a static or a cell, or
/// hands back a `&mut`; and, to a fixed point, everything that uses one. A use is read off the function's own words,
/// inside a macro's too: `f` its own module's (else any module's of that name), `m::f` module m's, `T::f` and `x.f(…)`
/// any method of that name. A `macro_rules!` of the file is followed like a function. It errs toward the taint.
fn tainted_fns(files: &[(PathBuf, String, String)], parsed: &[syn::File], all: &[Vec<FnAt>]) -> HashSet<(String, String)> {
    use crate::testcut::{uses, Via};
    struct Node {
        module: String,
        name: String,
        file: usize,
        uses: HashSet<(Via, String, bool)>,
        text: String,
        tainted: bool,
    }
    let dirty = |text: &str| text.contains("sim::") || TAINT.iter().any(|t| text.contains(t));
    let mut nodes: Vec<Node> = Vec::new();
    let mut fn_keys: HashSet<(String, String)> = HashSet::new();
    let mut methods: HashSet<(String, String)> = HashSet::new();
    let modules: HashSet<String> = files.iter().map(|f| module_of(&f.1)).collect();
    for (fi, fns) in all.iter().enumerate() {
        let src = &files[fi].2;
        let m = module_of(&files[fi].1);
        for f in fns {
            let body = &src[f.block.span().byte_range()];
            let ret = match &f.sig.output {
                syn::ReturnType::Type(_, t) => src[t.span().byte_range()].contains("mut"),
                _ => false,
            };
            fn_keys.insert((m.clone(), f.name.clone()));
            if f.method {
                methods.insert((m.clone(), f.name.clone()));
            }
            nodes.push(Node { module: m.clone(), name: f.name.clone(), file: fi, uses: uses(body), text: body.to_string(), tainted: ret || dirty(body) });
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
    // the macros of each file, to look for `name!` in a body
    let mut file_macros: HashMap<usize, Vec<usize>> = HashMap::new();
    for (i, n) in nodes.iter().enumerate() {
        if n.name.ends_with('!') {
            file_macros.entry(n.file).or_default().push(i);
        }
    }
    loop {
        let tainted: HashSet<(String, String)> = nodes.iter().filter(|n| n.tainted).map(|n| (n.module.clone(), n.name.clone())).collect();
        let free_names: HashSet<&String> = tainted.iter().filter(|k| !methods.contains(*k)).map(|k| &k.1).collect();
        let method_names: HashSet<&String> = tainted.iter().filter(|k| methods.contains(*k)).map(|k| &k.1).collect();
        let mut grew: Vec<usize> = Vec::new();
        for (i, n) in nodes.iter().enumerate() {
            if n.tainted {
                continue;
            }
            let by_macro = file_macros.get(&n.file).map(|v| v.iter().any(|&j| nodes[j].tainted && j != i && n.text.contains(nodes[j].name.as_str()))).unwrap_or(false);
            // (a bare word that is not called is one of the module's own functions handed on as a value, or a local: only the first counts)
            let by_use = n.uses.iter().any(|(via, name, called)| match via {
                Via::Dot => *called && method_names.contains(name),
                Via::Path(q) if q == "super" || q == "self" || *q == n.module => tainted.contains(&(n.module.clone(), name.clone())),
                Via::Path(q) if modules.contains(q) => tainted.contains(&(q.clone(), name.clone())),
                Via::Path(_) => method_names.contains(name),
                Via::Bare => {
                    let own = (n.module.clone(), name.clone());
                    if fn_keys.contains(&own) { tainted.contains(&own) } else { *called && free_names.contains(name) }
                }
            });
            if by_macro || by_use {
                grew.push(i);
            }
        }
        if grew.is_empty() {
            return tainted.into_iter().filter(|k| !k.1.ends_with('!')).collect();
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
}

/// Put `mut` back where the compiler's errors (`diag`, cargo's JSON) and the calls say. With `taint`, also on
/// every function that can write past the compiler's sight and on every function that calls one.
pub fn restore(root: &Path, state: &Path, diag: Option<&str>, taint: bool) -> Result<Restored, String> {
    let mut demoted = load_state(state);
    let files = bodies(root);
    let parsed: Vec<syn::File> = files.iter().map(|(_, rel, src)| syn::parse_file(src).map_err(|e| format!("{rel} does not parse: {e}"))).collect::<Result<_, _>>()?;
    let mut back: BTreeSet<String> = BTreeSet::new();
    let mut out = Restored { by_error: 0, by_calls: 0, by_taint: 0, unplaced: Vec::new(), coarse: 0, left: 0 };
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
            if let Some(&fi) = rel_ix.get(file) {
                if let Some(f) = all[fi].iter().find(|f| f.lines.0 <= line && line <= f.lines.1) {
                    // (a parameter an earlier error of this round already put back is placed too)
                    let ps = demoted_params(fi, f, &demoted, &BTreeSet::new());
                    let mut hit: Vec<&String> = ps.iter().filter(|p| named.contains(p)).collect();
                    if hit.is_empty() {
                        let in_span = idents_in(&shown);
                        hit = ps.iter().filter(|p| in_span.contains(p)).collect();
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
        let tainted = tainted_fns(&files, &parsed, &all);
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
                    v[i] = is_mut || back.contains(&k);
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

    // write the `mut`s back
    for (fi, (path, rel, src)) in files.iter().enumerate() {
        let mut ins: Vec<usize> = Vec::new();
        for f in &all[fi] {
            for (_, p, at, is_mut) in ref_params(f.sig) {
                if !is_mut && back.contains(&key(rel, &f.name, f.ord, &p)) {
                    ins.push(at);
                }
            }
        }
        if ins.is_empty() {
            continue;
        }
        ins.sort();
        let mut s = src.clone();
        for at in ins.into_iter().rev() {
            if s[at..].starts_with(' ') {
                s.insert_str(at + 1, "mut ");
            } else {
                s.insert_str(at, "mut ");
            }
        }
        std::fs::write(path, s).map_err(|e| format!("cannot write {rel}: {e}"))?;
    }
    for k in &back {
        demoted.remove(k);
    }
    out.left = demoted.len();
    save_state(state, &demoted);
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
pub fn report(root: &Path, state: &Path, bare: Option<&str>, calls_json: Option<&str>) -> Result<(String, Option<String>), String> {
    use std::fmt::Write as _;
    let demoted = load_state(state);
    let files = bodies(root);
    let parsed: Vec<syn::File> = files.iter().map(|(_, rel, src)| syn::parse_file(src).map_err(|e| format!("{rel} does not parse: {e}"))).collect::<Result<_, _>>()?;
    let all: Vec<Vec<FnAt>> = files.iter().zip(parsed.iter()).map(|((_, _, src), f)| fns_of(src, f)).collect();
    let tainted = tainted_fns(&files, &parsed, &all);
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
        reads_json = Some(format!("{{\"rows\": [\n{}\n]}}\n", all_keys.iter().map(|k| format!("  {}", serde_json::to_string(k).unwrap())).collect::<Vec<_>>().join(",\n")));
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
