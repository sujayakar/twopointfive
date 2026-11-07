//! The functions nothing names any more.
//!
//! A free function of a body file is DEAD when its name stands nowhere in the Rust sources but where a function of
//! that name is defined and where a module's `mod.rs` hands it on. This is by NAME, across every crate under
//! rust/: two functions that share a name keep each other, and so does a field or a local of that name. It
//! can keep a dead function; it cannot delete a live one that is named in Rust source.
//!
//! Only what the fold made dead goes: a function that was dead before the fold is left, and listed.
//! A function only the tools' call surface (`call_gen.rs`) names is left too, and listed.

use proc_macro2::{TokenStream, TokenTree};
use std::collections::{HashMap, HashSet};
use std::ops::Range;
use std::path::{Path, PathBuf};
use syn::spanned::Spanned;

#[derive(Default, Clone)]
pub struct Counts {
    /// every identifier, by how often it stands
    total: HashMap<String, i64>,
    /// `fn NAME`
    defs: HashMap<String, i64>,
}

fn count(src: &str) -> Counts {
    let mut c = Counts::default();
    // (a file that cannot be read into tokens would count as naming nothing: stop instead)
    let ts = src.parse::<TokenStream>().unwrap_or_else(|e| panic!("fold: a Rust file under rust/ does not read as tokens ({e}): its names cannot be counted"));
    fn walk(ts: TokenStream, c: &mut Counts) {
        let mut prev_fn = false;
        for t in ts {
            match t {
                TokenTree::Ident(i) => {
                    let s = i.to_string();
                    if prev_fn {
                        *c.defs.entry(s.clone()).or_default() += 1;
                    }
                    prev_fn = s == "fn";
                    *c.total.entry(s).or_default() += 1;
                }
                TokenTree::Group(g) => {
                    prev_fn = false;
                    walk(g.stream(), c);
                }
                _ => prev_fn = false,
            }
        }
    }
    walk(ts, &mut c);
    c
}

fn add(into: &mut Counts, from: &Counts, sign: i64) {
    for (k, v) in &from.total {
        *into.total.entry(k.clone()).or_default() += sign * v;
    }
    for (k, v) in &from.defs {
        *into.defs.entry(k.clone()).or_default() += sign * v;
    }
}

/// The names a `mod.rs` hands on by name (`pub use p_01::{a, b}`): (name, where it stands, the part it is handed on from).
fn use_names(src: &str) -> Vec<(String, Range<usize>, String)> {
    let mut out = Vec::new();
    let Ok(f) = syn::parse_file(src) else { return out };
    fn walk(t: &syn::UseTree, part: &str, out: &mut Vec<(String, Range<usize>, String)>) {
        match t {
            syn::UseTree::Path(p) => {
                let own = p.ident.to_string();
                walk(&p.tree, if part.is_empty() { &own } else { part }, out)
            }
            syn::UseTree::Group(g) => g.items.iter().for_each(|x| walk(x, part, out)),
            syn::UseTree::Name(n) => out.push((n.ident.to_string(), n.ident.span().byte_range(), part.to_string())),
            syn::UseTree::Rename(r) => out.push((r.ident.to_string(), r.span().byte_range(), part.to_string())),
            syn::UseTree::Glob(_) => {}
        }
    }
    for it in &f.items {
        if let syn::Item::Use(u) = it {
            walk(&u.tree, "", &mut out);
        }
    }
    out
}

/// The Rust sources that are not body files, read once.
pub struct World {
    /// everything but the body files and the modules' mod.rs
    fixed: Counts,
    call_surface: HashMap<String, i64>,
    /// module -> its mod.rs (where, text)
    pub modrs: HashMap<String, (PathBuf, String)>,
}

/// A body file: a file of one of the game's modules that holds its code. Not the module's `mod.rs`, its `types.rs`
/// (the shapes every part shares) or its `missing.rs` (the functions nobody has written, as named panics).
pub fn is_body(p: &Path) -> bool {
    let n = p.file_name().and_then(|n| n.to_str()).unwrap_or("");
    let in_module = p.parent().and_then(|d| d.parent()).and_then(|d| d.file_name()).map(|d| d == "src").unwrap_or(false) && p.parent().and_then(|d| d.file_name()).map(|d| d != "bin").unwrap_or(false);
    in_module && n.ends_with(".rs") && !matches!(n, "mod.rs" | "types.rs" | "missing.rs")
}

pub fn rs_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(rd) = std::fs::read_dir(dir) else { return };
    let mut v: Vec<PathBuf> = rd.filter_map(|e| e.ok().map(|e| e.path())).collect();
    v.sort();
    for p in v {
        if p.is_dir() {
            if p.file_name().map(|n| n == "target").unwrap_or(false) {
                continue;
            }
            rs_files(&p, out);
        } else if p.extension().map(|e| e == "rs").unwrap_or(false) {
            out.push(p);
        }
    }
}

pub fn world(root: &Path, body_paths: &HashSet<PathBuf>) -> World {
    let mut all = Vec::new();
    rs_files(&root.join("rust"), &mut all);
    let mut w = World { fixed: Counts::default(), call_surface: HashMap::new(), modrs: HashMap::new() };
    let game_src = root.join("rust/game/src");
    for p in all {
        if body_paths.contains(&p) {
            continue;
        }
        let Ok(text) = std::fs::read_to_string(&p) else { continue };
        let name = p.file_name().and_then(|n| n.to_str()).unwrap_or("");
        if name == "mod.rs" && p.parent().and_then(|d| d.parent()) == Some(game_src.as_path()) {
            let module = p.parent().and_then(|d| d.file_name()).and_then(|n| n.to_str()).unwrap_or("").to_string();
            w.modrs.insert(module, (p.clone(), text));
            continue;
        }
        let c = count(&text);
        if name == "call_gen.rs" {
            w.call_surface = c.total.clone();
        }
        add(&mut w.fixed, &c, 1);
    }
    w
}

/// The free functions of a body file: (name, the item's range).
fn free_fns(text: &str) -> Vec<(String, Range<usize>)> {
    let mut v = Vec::new();
    let Ok(f) = syn::parse_file(text) else { return v };
    for it in &f.items {
        if let syn::Item::Fn(x) = it {
            v.push((x.sig.ident.to_string(), it.span().byte_range()));
        }
    }
    v
}

/// What a set of body texts counts to, kept so a run that changes a few files recounts only those.
#[derive(Clone)]
pub struct Cache {
    counts: Vec<Counts>,
    fns: Vec<Vec<(String, Range<usize>)>>,
    sum: Counts,
}

pub fn cache(w: &World, texts: &[String]) -> Cache {
    let counts: Vec<Counts> = texts.iter().map(|t| count(t)).collect();
    let fns = texts.iter().map(|t| free_fns(t)).collect();
    let mut sum = w.fixed.clone();
    for c in &counts {
        add(&mut sum, c, 1);
    }
    Cache { counts, fns, sum }
}

/// An item's range widened to its lines, the comment lines right above it, and one blank line after.
fn item_lines(src: &str, r: Range<usize>) -> Range<usize> {
    let mut start = src[..r.start].rfind('\n').map(|i| i + 1).unwrap_or(0);
    while start > 0 {
        let ps = src[..start - 1].rfind('\n').map(|i| i + 1).unwrap_or(0);
        let l = src[ps..start].trim();
        if l.starts_with("//") && !l.starts_with("//!") {
            start = ps;
        } else {
            break;
        }
    }
    let mut end = src[r.end..].find('\n').map(|i| r.end + i + 1).unwrap_or(src.len());
    if let Some(i) = src[end..].find('\n') {
        if src[end..end + i].trim().is_empty() {
            end += i + 1;
        }
    }
    start..end
}

/// `name` out of the list `mod.rs` hands on from the part `part` (`p_02`): never from another part's list, where a
/// function of the same name may be live.
pub fn drop_use_name(text: &str, name: &str, part: &str) -> String {
    let Some((_, r, _)) = use_names(text).into_iter().find(|(n, _, p)| n == name && p == part) else { return text.to_string() };
    let mut s = text.to_string();
    let after = &text[r.end..];
    let before = &text[..r.start];
    if let Some(rest) = after.strip_prefix(',') {
        let ws = rest.len() - rest.trim_start_matches(' ').len();
        s.replace_range(r.start..r.end + 1 + ws, "");
    } else if before.trim_end().ends_with(',') {
        let c = before.trim_end().len() - 1;
        s.replace_range(c..r.end, "");
    } else {
        // the list's only name: the whole line goes
        let ls = before.rfind('\n').map(|i| i + 1).unwrap_or(0);
        let le = after.find('\n').map(|i| r.end + i + 1).unwrap_or(text.len());
        s.replace_range(ls..le, "");
    }
    s
}

#[derive(Default)]
pub struct Swept {
    /// (module, function) deleted
    pub deleted: Vec<(String, String)>,
    /// of the deleted, those the first look found (nothing named them as the files stood); the rest were named
    /// only by a function deleted before them
    pub first: HashSet<(String, String)>,
    /// (module, function) that only the tools' call surface names
    pub surface_only: Vec<(String, String)>,
    /// module -> its mod.rs, where a deleted function's name left it
    pub modrs: HashMap<String, String>,
}

/// Delete dead functions until none is left. `texts` are the body files (`changed` = the ones that differ from
/// what `base` was counted on). `spare` are never deleted, and so keep what they name.
pub fn fixpoint(w: &World, base: &Cache, modules: &[String], stems: &[String], texts: &mut [String], changed: &[usize], spare: &HashSet<(String, String)>, delete: bool) -> Swept {
    let mut c = base.clone();
    for &i in changed {
        add(&mut c.sum, &c.counts[i], -1);
        c.counts[i] = count(&texts[i]);
        add(&mut c.sum, &c.counts[i], 1);
        c.fns[i] = free_fns(&texts[i]);
    }
    let mut modrs: HashMap<String, String> = HashMap::new();
    let mut out = Swept::default();
    let mut round = 0;
    loop {
        round += 1;
        // the mod.rs files as they stand
        let mut mc = Counts::default();
        let mut handed: HashMap<String, i64> = HashMap::new();
        for (m, (_, text)) in &w.modrs {
            let t = modrs.get(m).unwrap_or(text);
            add(&mut mc, &count(t), 1);
            for (n, _, _) in use_names(t) {
                *handed.entry(n).or_default() += 1;
            }
        }
        let refs = |n: &str| -> i64 { c.sum.total.get(n).copied().unwrap_or(0) + mc.total.get(n).copied().unwrap_or(0) - c.sum.defs.get(n).copied().unwrap_or(0) - mc.defs.get(n).copied().unwrap_or(0) - handed.get(n).copied().unwrap_or(0) };
        let mut dead: HashMap<usize, Vec<(String, Range<usize>)>> = HashMap::new();
        out.surface_only.clear();
        for (bi, fns) in c.fns.iter().enumerate() {
            for (name, range) in fns {
                let r = refs(name);
                let key = (modules[bi].clone(), name.clone());
                if r == 0 {
                    if !spare.contains(&key) {
                        dead.entry(bi).or_default().push((name.clone(), range.clone()));
                    }
                } else if r == w.call_surface.get(name).copied().unwrap_or(0) {
                    out.surface_only.push(key);
                }
            }
        }
        if dead.is_empty() || !delete {
            if !delete {
                for (bi, v) in dead {
                    for (n, _) in v {
                        out.deleted.push((modules[bi].clone(), n));
                    }
                }
            }
            break;
        }
        for (bi, mut v) in dead {
            v.sort_by_key(|f| std::cmp::Reverse(f.1.start));
            let mut t = texts[bi].clone();
            for (name, range) in v {
                let r = item_lines(&t, range);
                t.replace_range(r, "");
                if let Some((_, text)) = w.modrs.get(&modules[bi]) {
                    let cur = modrs.get(&modules[bi]).unwrap_or(text);
                    let next = drop_use_name(cur, &name, &stems[bi]);
                    if &next != text {
                        modrs.insert(modules[bi].clone(), next);
                    }
                }
                if round == 1 {
                    out.first.insert((modules[bi].clone(), name.clone()));
                }
                out.deleted.push((modules[bi].clone(), name));
            }
            add(&mut c.sum, &c.counts[bi], -1);
            c.counts[bi] = count(&t);
            add(&mut c.sum, &c.counts[bi], 1);
            c.fns[bi] = free_fns(&t);
            texts[bi] = t;
        }
    }
    out.modrs = modrs;
    out
}
