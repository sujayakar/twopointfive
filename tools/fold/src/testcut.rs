//! The unit tests of code the game never runs.
//!
//! The compiler's reading without the tests (`sharp.sh --dry --no-tests`) names what only a unit test still uses.
//! `cut` deletes the TESTS that are there for that code, and nothing else: the compiler's pass, run again, then
//! finds what those tests alone held up and deletes it, and keeps whatever a test that stays still reaches.
//!
//! A test goes when it uses something only the tests use and calls no live function of a body file, itself or
//! through the helpers of its test module. A test that does both is a test of live code that leans on a helper
//! only tests use: it stays, and so does the helper.
//! A use is matched to its item by how it is written (`f(…)` its own module's, `m::f(…)` module m's, `T::f(…)` a
//! method of T, `x.f(…)` any method of that name), so it errs toward keeping: where a name could be a live item, it is.
//!
//! `mend` takes a name out of a `use` the compiler can no longer resolve (a test module that imported a function
//! the pass deleted).

use crate::sharp::Named;
use proc_macro2::{Delimiter, TokenStream, TokenTree};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::ops::Range;
use std::path::{Path, PathBuf};
use syn::spanned::Spanned;

fn is_body(p: &Path) -> bool {
    let n = p.file_name().and_then(|n| n.to_str()).unwrap_or("");
    (n.starts_with("p_") || n.starts_with("b_")) && n.ends_with(".rs")
}

fn is_test_mod(m: &syn::ItemMod) -> bool {
    m.attrs.iter().any(|a| a.path().is_ident("cfg") && a.meta.require_list().map(|l| l.tokens.to_string().contains("test")).unwrap_or(false))
}

fn is_test_fn(f: &syn::ItemFn) -> bool {
    f.attrs.iter().any(|a| a.path().is_ident("test"))
}

fn line_of(src: &str, at: usize) -> usize {
    src[..at].bytes().filter(|b| *b == b'\n').count() + 1
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

/// How a name is written where it is used.
#[derive(Clone, PartialEq, Eq, Hash)]
enum Via {
    /// `f(…)`, `NAME`
    Bare,
    /// `q::f(…)`: the word before the `::`
    Path(String),
    /// `x.f(…)`
    Dot,
}

/// Every name a piece of source uses, each with how it is written and whether it is called (a parenthesis follows).
fn uses(src: &str) -> HashSet<(Via, String, bool)> {
    fn walk(ts: TokenStream, out: &mut HashSet<(Via, String, bool)>) {
        let v: Vec<TokenTree> = ts.into_iter().collect();
        let punct = |i: usize, c: char| matches!(v.get(i), Some(TokenTree::Punct(p)) if p.as_char() == c);
        for (i, t) in v.iter().enumerate() {
            match t {
                TokenTree::Ident(id) => {
                    let called = matches!(v.get(i + 1), Some(TokenTree::Group(g)) if g.delimiter() == Delimiter::Parenthesis);
                    let via = if i >= 1 && punct(i - 1, '.') {
                        Via::Dot
                    } else if i >= 3 && punct(i - 1, ':') && punct(i - 2, ':') {
                        match &v[i - 3] {
                            TokenTree::Ident(q) => Via::Path(q.to_string()),
                            _ => Via::Path(String::new()),
                        }
                    } else {
                        Via::Bare
                    };
                    out.insert((via, id.to_string(), called));
                }
                TokenTree::Group(g) => walk(g.stream(), out),
                _ => {}
            }
        }
    }
    let mut out = HashSet::new();
    if let Ok(ts) = src.parse::<TokenStream>() {
        walk(ts, &mut out);
    }
    out
}

/// What the body files hold outside their test modules, each marked dead (only tests use it) or live.
#[derive(Default)]
struct Items {
    /// (module, name) -> is one of that name live there
    free: HashMap<(String, String), bool>,
    /// (the impl's type, name) -> live
    methods: HashMap<(String, String), bool>,
    /// name -> is a method of that name live anywhere
    method_names: HashMap<String, bool>,
    /// name -> is a free function, constant or type of that name live anywhere
    free_names: HashMap<String, bool>,
    modules: HashSet<String>,
}

impl Items {
    fn put(m: &mut HashMap<(String, String), bool>, k: (String, String), live: bool) {
        *m.entry(k).or_insert(false) |= live;
    }
    /// Some(true) live, Some(false) only the tests use it, None: no item of a body file
    fn resolve(&self, module: &str, via: &Via, name: &str, called: bool) -> Option<bool> {
        match via {
            Via::Dot => if called { self.method_names.get(name).copied() } else { None },
            Via::Path(q) if q == "super" || q == "self" || q == module => self.free.get(&(module.to_string(), name.to_string())).copied(),
            Via::Path(q) if self.modules.contains(q) => self.free.get(&(q.clone(), name.to_string())).copied(),
            Via::Path(q) => self.methods.get(&(q.clone(), name.to_string())).copied(),
            // (its own module's, else one brought in by a `use`: any module's of that name)
            Via::Bare => self.free.get(&(module.to_string(), name.to_string())).copied().or_else(|| self.free_names.get(name).copied()),
        }
    }
}

fn bodies(root: &Path) -> Vec<(PathBuf, String, String)> {
    let game = root.join("rust/game");
    let mut all = Vec::new();
    crate::dead::rs_files(&game.join("src"), &mut all);
    all.into_iter().filter(|p| is_body(p)).filter_map(|p| std::fs::read_to_string(&p).ok().map(|t| (p.clone(), p.strip_prefix(&game).unwrap_or(&p).to_string_lossy().to_string(), t))).collect()
}

#[derive(Default)]
pub struct Cut {
    /// `module::test (n lines)` deleted
    pub tests: Vec<String>,
    /// tests that call something only tests use and stay, each with the live function that keeps it
    pub stay: Vec<String>,
    pub mods: usize,
    pub lines: usize,
}

/// Delete the tests that are there for what `named` (the reading without the tests) calls dead.
pub fn cut(root: &Path, named: &Named, write: bool) -> Result<Cut, String> {
    let files = bodies(root);
    let parsed: Vec<syn::File> = files.iter().map(|(_, rel, src)| syn::parse_file(src).map_err(|e| format!("{rel} does not parse: {e}"))).collect::<Result<_, _>>()?;
    // what the body files hold outside their test modules: dead (only the tests use it) or live
    let mut items = Items::default();
    for ((path, rel, src), f) in files.iter().zip(parsed.iter()) {
        let module = path.parent().and_then(|d| d.file_name()).and_then(|n| n.to_str()).unwrap_or("").to_string();
        items.modules.insert(module.clone());
        let d: HashSet<(usize, &str)> = named.by_file.get(rel).map(|v| v.iter().map(|(l, n, _)| (*l, n.as_str())).collect()).unwrap_or_default();
        fn walk(list: &[syn::Item], src: &str, module: &str, d: &HashSet<(usize, &str)>, items: &mut Items) {
            for it in list {
                let live = |id: &syn::Ident| !d.contains(&(line_of(src, id.span().byte_range().start), id.to_string().as_str()));
                let id = match it {
                    syn::Item::Fn(x) => Some(&x.sig.ident),
                    syn::Item::Const(x) => Some(&x.ident),
                    syn::Item::Static(x) => Some(&x.ident),
                    syn::Item::Struct(x) => Some(&x.ident),
                    syn::Item::Enum(x) => Some(&x.ident),
                    syn::Item::Type(x) => Some(&x.ident),
                    _ => None,
                };
                if let Some(id) = id {
                    let l = live(id);
                    Items::put(&mut items.free, (module.to_string(), id.to_string()), l);
                    *items.free_names.entry(id.to_string()).or_insert(false) |= l;
                    continue;
                }
                match it {
                    syn::Item::Impl(x) => {
                        let ty = match &*x.self_ty {
                            syn::Type::Path(p) => p.path.segments.last().map(|s| s.ident.to_string()).unwrap_or_default(),
                            _ => String::new(),
                        };
                        for ii in &x.items {
                            let id = match ii {
                                syn::ImplItem::Fn(m) => &m.sig.ident,
                                syn::ImplItem::Const(m) => &m.ident,
                                _ => continue,
                            };
                            let l = live(id);
                            Items::put(&mut items.methods, (ty.clone(), id.to_string()), l);
                            *items.method_names.entry(id.to_string()).or_insert(false) |= l;
                        }
                    }
                    syn::Item::Mod(m) if !is_test_mod(m) => {
                        if let Some((_, inner)) = &m.content {
                            walk(inner, src, module, d, items);
                        }
                    }
                    _ => {}
                }
            }
        }
        walk(&f.items, src, &module, &d, &mut items);
    }

    let mut out = Cut::default();
    for ((path, rel, src), f) in files.iter().zip(parsed.iter()) {
        let module = path.parent().and_then(|d| d.file_name()).and_then(|n| n.to_str()).unwrap_or("").to_string();
        let mut cuts: Vec<Range<usize>> = Vec::new();
        fn mods<'a>(items: &'a [syn::Item], out: &mut Vec<&'a syn::ItemMod>) {
            for it in items {
                if let syn::Item::Mod(m) = it {
                    if is_test_mod(m) {
                        out.push(m);
                    } else if let Some((_, inner)) = &m.content {
                        mods(inner, out);
                    }
                }
            }
        }
        let mut tms = Vec::new();
        mods(&f.items, &mut tms);
        for m in tms {
            let Some((_, items_of)) = &m.content else { continue };
            // the helpers of this test module, each with what it uses
            let mut helper: HashMap<String, HashSet<(Via, String, bool)>> = HashMap::new();
            let mut tests: Vec<&syn::ItemFn> = Vec::new();
            for it in items_of {
                if let syn::Item::Fn(x) = it {
                    if is_test_fn(x) {
                        tests.push(x);
                    } else {
                        helper.insert(x.sig.ident.to_string(), uses(&src[x.span().byte_range()]));
                    }
                }
            }
            let mut gone = 0;
            for t in &tests {
                let mut all = uses(&src[t.span().byte_range()]);
                // through the module's own helpers, to a fixed point
                let mut seen: HashSet<String> = HashSet::new();
                loop {
                    let next: Vec<String> = all.iter().filter(|(via, n, called)| *called && *via == Via::Bare && helper.contains_key(n) && !seen.contains(n)).map(|u| u.1.clone()).collect();
                    if next.is_empty() {
                        break;
                    }
                    for h in next {
                        all.extend(helper[&h].iter().cloned());
                        seen.insert(h);
                    }
                }
                let mut dead: Vec<&String> = Vec::new();
                let mut keeps: Vec<&String> = Vec::new();
                for (via, n, called) in &all {
                    if *via == Via::Bare && helper.contains_key(n) {
                        continue;
                    }
                    match items.resolve(&module, via, n, *called) {
                        Some(true) if *called => keeps.push(n),
                        // (`x.f(…)` says nothing of whose `f` it is: it can keep a test, never doom one)
                        Some(false) if *via != Via::Dot => dead.push(n),
                        _ => {}
                    }
                }
                if dead.is_empty() {
                    continue;
                }
                let name = format!("{module}::{}", t.sig.ident);
                keeps.sort();
                keeps.dedup();
                if let Some(k) = keeps.first() {
                    out.stay.push(format!("{name}: also calls {k}{}", if keeps.len() > 1 { format!(" and {} more", keeps.len() - 1) } else { String::new() }));
                    continue;
                }
                let r = item_lines(src, t.span().byte_range());
                out.tests.push(format!("{name} ({} lines)", src[r.clone()].lines().count()));
                cuts.push(r);
                gone += 1;
            }
            // a test module with no test left goes whole
            if gone > 0 && gone == tests.len() {
                let r = item_lines(src, m.span().byte_range());
                cuts.retain(|c| !(r.start <= c.start && c.end <= r.end));
                cuts.push(r);
                out.mods += 1;
            }
        }
        if cuts.is_empty() {
            continue;
        }
        cuts.sort_by_key(|c| std::cmp::Reverse(c.start));
        let mut s = src.clone();
        for r in cuts {
            out.lines += s[r.clone()].lines().count();
            s.replace_range(r, "");
        }
        if write {
            std::fs::write(path, s).map_err(|e| format!("cannot write {rel}: {e}"))?;
        }
    }
    out.tests.sort();
    out.stay.sort();
    Ok(out)
}

/// Take out of its `use` each name the compiler could not resolve (`diag`: cargo's JSON of a check that failed).
/// Returns (names taken out, errors that are not an unresolved import of a body file).
pub fn mend(root: &Path, diag: &str, write: bool) -> Result<(usize, Vec<String>), String> {
    let game = root.join("rust/game");
    // file -> (line, name)
    let mut want: BTreeMap<String, HashSet<(usize, String)>> = BTreeMap::new();
    let mut other: Vec<String> = Vec::new();
    for l in diag.lines() {
        let Ok(v) = serde_json::from_str::<serde_json::Value>(l) else { continue };
        if v["reason"] != "compiler-message" || v["message"]["level"] != "error" {
            continue;
        }
        let msg = v["message"]["message"].as_str().unwrap_or("");
        if msg.starts_with("aborting due to") || msg.starts_with("could not compile") {
            continue;
        }
        let code = v["message"]["code"]["code"].as_str().unwrap_or("");
        let span = v["message"]["spans"].as_array().and_then(|a| a.iter().find(|s| s["is_primary"] == true));
        let Some(s) = span else { other.push(msg.to_string()); continue };
        let file = s["file_name"].as_str().unwrap_or("").to_string();
        let line = s["line_start"].as_u64().unwrap_or(0) as usize;
        if code != "E0432" || !is_body(Path::new(&file)) {
            other.push(format!("{file}:{line} {msg}"));
            continue;
        }
        let (hs, he) = (s["text"][0]["highlight_start"].as_u64().unwrap_or(1) as usize, s["text"][0]["highlight_end"].as_u64().unwrap_or(1) as usize);
        let shown: String = s["text"][0]["text"].as_str().unwrap_or("").chars().skip(hs.saturating_sub(1)).take(he.saturating_sub(hs)).collect();
        // (the span is the name, or the whole path of a plain `use a::b;`: its last word is the name)
        let name = shown.rsplit("::").next().unwrap_or("").trim().to_string();
        want.entry(file).or_default().insert((line, name));
    }
    let mut n = 0;
    for (file, list) in want {
        let path = game.join(&file);
        let src = std::fs::read_to_string(&path).map_err(|e| format!("cannot read {}: {e}", path.display()))?;
        let f = syn::parse_file(&src).map_err(|e| format!("{file} does not parse: {e}"))?;
        // every name a `use` of the file brings in: (name, the name's range, the whole item's range, is it the item's only name)
        let mut uses: Vec<(String, Range<usize>, Range<usize>, bool)> = Vec::new();
        fn tree(t: &syn::UseTree, v: &mut Vec<(String, Range<usize>)>) {
            match t {
                syn::UseTree::Path(p) => tree(&p.tree, v),
                syn::UseTree::Group(g) => g.items.iter().for_each(|x| tree(x, v)),
                syn::UseTree::Name(x) => v.push((x.ident.to_string(), x.ident.span().byte_range())),
                syn::UseTree::Rename(x) => v.push((x.ident.to_string(), x.span().byte_range())),
                syn::UseTree::Glob(_) => {}
            }
        }
        fn walk(items: &[syn::Item], uses: &mut Vec<(String, Range<usize>, Range<usize>, bool)>) {
            for it in items {
                match it {
                    syn::Item::Use(u) => {
                        let r = it.span().byte_range();
                        let mut v = Vec::new();
                        tree(&u.tree, &mut v);
                        let alone = v.len() == 1;
                        for (n, nr) in v {
                            uses.push((n, nr, r.clone(), alone));
                        }
                    }
                    syn::Item::Mod(m) => {
                        if let Some((_, inner)) = &m.content {
                            walk(inner, uses);
                        }
                    }
                    _ => {}
                }
            }
        }
        walk(&f.items, &mut uses);
        let mut cuts: Vec<Range<usize>> = Vec::new();
        for (line, name) in &list {
            let hit = uses.iter().find(|(n, nr, _, _)| n == name && line_of(&src, nr.start) == *line);
            let Some((_, nr, ir, alone)) = hit else {
                other.push(format!("{file}:{line} an unresolved import of `{name}` this program could not find in a `use`"));
                continue;
            };
            if *alone {
                let ls = src[..ir.start].rfind('\n').map(|i| i + 1).unwrap_or(0);
                let le = src[ir.end..].find('\n').map(|i| ir.end + i + 1).unwrap_or(src.len());
                // (only where the `use` stands alone on its line)
                if src[ls..ir.start].trim().is_empty() && src[ir.end..le].trim().is_empty() {
                    cuts.push(ls..le);
                } else {
                    cuts.push(ir.clone());
                }
            } else {
                let after = &src[nr.end..];
                let before = &src[..nr.start];
                if let Some(rest) = after.strip_prefix(',') {
                    let ws = rest.len() - rest.trim_start_matches(' ').len();
                    cuts.push(nr.start..nr.end + 1 + ws);
                } else if before.trim_end().ends_with(',') {
                    cuts.push(before.trim_end().len() - 1..nr.end);
                } else {
                    cuts.push(nr.clone());
                }
            }
            n += 1;
        }
        cuts.sort_by_key(|c| std::cmp::Reverse(c.start));
        cuts.dedup();
        let mut s = src.clone();
        for r in cuts {
            s.replace_range(r, "");
        }
        if write && s != src {
            std::fs::write(&path, s).map_err(|e| format!("cannot write {file}: {e}"))?;
        }
    }
    Ok((n, other))
}
