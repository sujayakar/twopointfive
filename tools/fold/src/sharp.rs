//! The compiler's own reading of what the game never runs.
//!
//! `prep` writes a copy of rust/ in which every item of a body file is private to the crate, unless its name
//! stands in the crates that use the game (rust/game_face, rust/game_run).
//! With the crate's `allow(dead_code)` lifted, `cargo check` on that copy names every function, constant and
//! static that nothing live reaches: the generated call surface, the wire and the tables stay public, so
//! what they reach is live. Checked with `--tests`, the unit tests are live too, so what only a test uses
//! stays; checked without, it is named. `apply` deletes what the check named from the real body files,
//! never an item inside a test module.
//!
//! A line of the copy is the same line of the real file: `prep` changes words inside lines, never their count.

use proc_macro2::{TokenStream, TokenTree};
use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::path::{Path, PathBuf};
use syn::spanned::Spanned;

fn idents(src: &str, out: &mut HashSet<String>) {
    fn walk(ts: TokenStream, out: &mut HashSet<String>) {
        for t in ts {
            match t {
                TokenTree::Ident(i) => {
                    out.insert(i.to_string());
                }
                TokenTree::Group(g) => walk(g.stream(), out),
                _ => {}
            }
        }
    }
    if let Ok(ts) = src.parse::<TokenStream>() {
        walk(ts, out);
    }
}

fn is_test_mod(m: &syn::ItemMod) -> bool {
    m.attrs.iter().any(|a| a.path().is_ident("cfg") && a.meta.require_list().map(|l| l.tokens.to_string().contains("test")).unwrap_or(false))
}

fn is_body(p: &Path) -> bool {
    let n = p.file_name().and_then(|n| n.to_str()).unwrap_or("");
    (n.starts_with("p_") || n.starts_with("b_")) && n.ends_with(".rs")
}

/// `pub` -> `pub(crate)` on every item of a body file whose name is not in `keep`, and on its own `pub use`s.
/// An item whose name IS in `keep` stays as it is and is told to the compiler as used (`#[allow(dead_code)]`, which
/// makes it and what it reaches live): a private function only a unit test names would else be called dead.
fn privatise(src: &str, keep: &HashSet<String>) -> String {
    let Ok(f) = syn::parse_file(src) else { return src.to_string() };
    // (where, what to put there)
    let mut at: Vec<(usize, &'static str)> = Vec::new();
    fn one(v: &syn::Visibility, name: &syn::Ident, head: usize, keep: &HashSet<String>, at: &mut Vec<(usize, &'static str)>) {
        if keep.contains(&name.to_string()) {
            at.push((head, "#[allow(dead_code)] "));
        } else if let syn::Visibility::Public(p) = v {
            at.push((p.span.byte_range().end, "(crate)"));
        }
    }
    /// where an item's own words begin, after its attributes and comments
    fn head(v: &syn::Visibility, first: proc_macro2::Span) -> usize {
        match v {
            syn::Visibility::Inherited => first.byte_range().start,
            other => other.span().byte_range().start,
        }
    }
    fn items(list: &[syn::Item], keep: &HashSet<String>, at: &mut Vec<(usize, &'static str)>) {
        for it in list {
            match it {
                syn::Item::Fn(x) => {
                    let first = x.sig.constness.map(|t| t.span).or(x.sig.asyncness.map(|t| t.span)).or(x.sig.unsafety.map(|t| t.span)).unwrap_or(x.sig.fn_token.span);
                    one(&x.vis, &x.sig.ident, head(&x.vis, first), keep, at)
                }
                syn::Item::Const(x) => one(&x.vis, &x.ident, head(&x.vis, x.const_token.span), keep, at),
                syn::Item::Static(x) => one(&x.vis, &x.ident, head(&x.vis, x.static_token.span), keep, at),
                syn::Item::Use(x) => {
                    if let syn::Visibility::Public(p) = &x.vis {
                        at.push((p.span.byte_range().end, "(crate)"));
                    }
                }
                syn::Item::Impl(x) => {
                    for ii in &x.items {
                        match ii {
                            syn::ImplItem::Fn(m) => {
                                let first = m.sig.constness.map(|t| t.span).or(m.sig.asyncness.map(|t| t.span)).or(m.sig.unsafety.map(|t| t.span)).unwrap_or(m.sig.fn_token.span);
                                if x.trait_.is_none() {
                                    one(&m.vis, &m.sig.ident, head(&m.vis, first), keep, at)
                                }
                            }
                            syn::ImplItem::Const(m) => {
                                if x.trait_.is_none() {
                                    one(&m.vis, &m.ident, head(&m.vis, m.const_token.span), keep, at)
                                }
                            }
                            _ => {}
                        }
                    }
                }
                syn::Item::Mod(m) if !is_test_mod(m) => {
                    if let Some((_, inner)) = &m.content {
                        items(inner, keep, at);
                    }
                }
                _ => {}
            }
        }
    }
    items(&f.items, keep, &mut at);
    at.sort();
    let mut s = src.to_string();
    for (a, what) in at.into_iter().rev() {
        s.insert_str(a, what);
    }
    s
}

fn copy_tree(from: &Path, to: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(to)?;
    for e in std::fs::read_dir(from)? {
        let e = e?;
        let p = e.path();
        let name = e.file_name();
        if name == "target" || name == "prebuilt" {
            continue;
        }
        if p.is_dir() {
            copy_tree(&p, &to.join(&name))?;
        } else {
            std::fs::copy(&p, to.join(&name))?;
        }
    }
    Ok(())
}

/// Write the private copy under `scratch/rust`.
pub fn prep(root: &Path, scratch: &Path) -> Result<String, String> {
    let to = scratch.join("rust");
    let _ = std::fs::remove_dir_all(&to);
    copy_tree(&root.join("rust"), &to).map_err(|e| format!("cannot copy rust/ to {}: {e}", to.display()))?;
    // names that make an item live from outside the crate's own non-test code
    let mut keep: HashSet<String> = HashSet::new();
    let mut users = Vec::new();
    crate::dead::rs_files(&root.join("rust/game_face"), &mut users);
    crate::dead::rs_files(&root.join("rust/game_run"), &mut users);
    for p in &users {
        if let Ok(t) = std::fs::read_to_string(p) {
            idents(&t, &mut keep);
        }
    }
    let from_users = keep.len();
    let mut all = Vec::new();
    crate::dead::rs_files(&to.join("game/src"), &mut all);
    let mut n = 0;
    for p in &all {
        let Ok(t) = std::fs::read_to_string(p) else { continue };
        let name = p.file_name().and_then(|n| n.to_str()).unwrap_or("");
        let mut out = t.clone();
        if is_body(p) {
            out = privatise(&out, &keep);
        } else if name == "mod.rs" {
            // a module hands its parts on to the crate, not to the world
            out = out.lines().map(|l| if l.starts_with("pub use p_") || l.starts_with("pub use b_") { l.replacen("pub use", "pub(crate) use", 1) } else { l.to_string() }).collect::<Vec<_>>().join("\n") + "\n";
        }
        if out.contains("dead_code") {
            out = out.replace("#![allow(dead_code)]", "").replace("#[allow(dead_code)]", "").replace("dead_code, ", "").replace(", dead_code", "");
        }
        if out != t {
            n += 1;
            std::fs::write(p, out).map_err(|e| format!("cannot write {}: {e}", p.display()))?;
        }
    }
    Ok(format!("the private copy is {} ({n} files changed in it; {} names kept public for rust/game_face and rust/game_run)", to.display(), from_users))
}

#[derive(Default)]
pub struct Named {
    /// file (from rust/game) -> (line, name, what the compiler said)
    pub by_file: BTreeMap<String, Vec<(usize, String, String)>>,
}

/// What `cargo check --message-format=json` on the private copy named dead.
pub fn read_diag(text: &str) -> Named {
    let mut out = Named::default();
    for l in text.lines() {
        let Ok(v) = serde_json::from_str::<serde_json::Value>(l) else { continue };
        if v["reason"] != "compiler-message" || v["message"]["code"]["code"] != "dead_code" {
            continue;
        }
        let msg = v["message"]["message"].as_str().unwrap_or("").to_string();
        let Some(spans) = v["message"]["spans"].as_array() else { continue };
        for s in spans.iter().filter(|s| s["is_primary"] == true) {
            let file = s["file_name"].as_str().unwrap_or("").to_string();
            let line = s["line_start"].as_u64().unwrap_or(0) as usize;
            let (hs, he) = (s["text"][0]["highlight_start"].as_u64().unwrap_or(1) as usize, s["text"][0]["highlight_end"].as_u64().unwrap_or(1) as usize);
            let text = s["text"][0]["text"].as_str().unwrap_or("");
            let name: String = text.chars().skip(hs.saturating_sub(1)).take(he.saturating_sub(hs)).collect();
            out.by_file.entry(file).or_default().push((line, name, msg.clone()));
        }
    }
    out
}

pub struct Applied {
    /// `module::name`, with its kind
    pub deleted: Vec<(String, String, usize)>,
    /// named dead, and not an item of a body file this pass deletes (a field, a variant, a generated file's)
    pub listed: Vec<String>,
    pub lines: usize,
}

fn line_of(src: &str, at: usize) -> usize {
    src[..at].bytes().filter(|b| *b == b'\n').count() + 1
}

fn item_lines(src: &str, r: std::ops::Range<usize>) -> std::ops::Range<usize> {
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

/// Delete the named items from the real body files. Returns what went; writes only with `write`.
pub fn apply(root: &Path, named: &Named, write: bool) -> Result<Applied, String> {
    let mut out = Applied { deleted: Vec::new(), listed: Vec::new(), lines: 0 };
    let game = root.join("rust/game");
    // mod.rs -> (name, the part it was in)
    let mut modrs: BTreeMap<PathBuf, BTreeSet<(String, String)>> = BTreeMap::new();
    for (file, list) in &named.by_file {
        let path = game.join(file);
        if !is_body(&path) {
            for (line, name, msg) in list {
                out.listed.push(format!("{file}:{line} {name}: {msg}"));
            }
            continue;
        }
        let src = std::fs::read_to_string(&path).map_err(|e| format!("cannot read {}: {e}", path.display()))?;
        let f = syn::parse_file(&src).map_err(|e| format!("{file} does not parse: {e}"))?;
        let module = path.parent().and_then(|d| d.file_name()).and_then(|n| n.to_str()).unwrap_or("").to_string();
        let want: HashSet<(usize, &str)> = list.iter().map(|(l, n, _)| (*l, n.as_str())).collect();
        let mut hit: HashSet<(usize, String)> = HashSet::new();
        let mut cut: Vec<(std::ops::Range<usize>, String, &'static str, bool)> = Vec::new();
        fn walk<'a>(items: &'a [syn::Item], src: &str, want: &HashSet<(usize, &str)>, hit: &mut HashSet<(usize, String)>, cut: &mut Vec<(std::ops::Range<usize>, String, &'static str, bool)>) {
            for it in items {
                let (ident, kind): (Option<&syn::Ident>, &'static str) = match it {
                    syn::Item::Fn(x) => (Some(&x.sig.ident), "function"),
                    syn::Item::Const(x) => (Some(&x.ident), "constant"),
                    syn::Item::Static(x) => (Some(&x.ident), "static"),
                    syn::Item::Struct(x) => (Some(&x.ident), "struct"),
                    syn::Item::Enum(x) => (Some(&x.ident), "enum"),
                    syn::Item::Type(x) => (Some(&x.ident), "type"),
                    _ => (None, ""),
                };
                if let Some(id) = ident {
                    let key = (line_of(src, id.span().byte_range().start), id.to_string());
                    if want.contains(&(key.0, key.1.as_str())) {
                        hit.insert(key.clone());
                        cut.push((it.span().byte_range(), key.1, kind, kind == "function"));
                    }
                    continue;
                }
                match it {
                    syn::Item::Impl(x) => {
                        let mut gone = 0;
                        let mut inner = Vec::new();
                        for ii in &x.items {
                            let id = match ii {
                                syn::ImplItem::Fn(m) => Some(&m.sig.ident),
                                syn::ImplItem::Const(m) => Some(&m.ident),
                                _ => None,
                            };
                            if let Some(id) = id {
                                let key = (line_of(src, id.span().byte_range().start), id.to_string());
                                if want.contains(&(key.0, key.1.as_str())) {
                                    hit.insert(key.clone());
                                    gone += 1;
                                    inner.push((ii.span().byte_range(), key.1, "method", false));
                                }
                            }
                        }
                        if gone > 0 && gone == x.items.len() {
                            cut.push((it.span().byte_range(), format!("impl ({})", inner.iter().map(|i| i.1.clone()).collect::<Vec<_>>().join(", ")), "impl", false));
                        } else {
                            cut.extend(inner);
                        }
                    }
                    syn::Item::Mod(m) if !is_test_mod(m) => {
                        if let Some((_, inner)) = &m.content {
                            walk(inner, src, want, hit, cut);
                        }
                    }
                    _ => {}
                }
            }
        }
        walk(&f.items, &src, &want, &mut hit, &mut cut);
        for (line, name, msg) in list {
            if !hit.contains(&(*line, name.clone())) {
                out.listed.push(format!("{file}:{line} {name}: {msg}"));
            }
        }
        if cut.is_empty() {
            continue;
        }
        cut.sort_by_key(|c| std::cmp::Reverse(c.0.start));
        let mut s = src.clone();
        for (r, name, kind, free_fn) in cut {
            let lr = item_lines(&s, r);
            let n = s[lr.clone()].lines().count();
            s.replace_range(lr, "");
            out.lines += n;
            out.deleted.push((format!("{module}::{name}"), kind.to_string(), n));
            let _ = free_fn;
            if kind != "method" && kind != "impl" {
                let part = path.file_stem().and_then(|n| n.to_str()).unwrap_or("").to_string();
                modrs.entry(path.parent().unwrap().join("mod.rs")).or_default().insert((name, part));
            }
        }
        if write {
            std::fs::write(&path, s).map_err(|e| format!("cannot write {}: {e}", path.display()))?;
        }
    }
    // a module that handed a deleted function on by name no longer does
    for (p, names) in modrs {
        let Ok(mut t) = std::fs::read_to_string(&p) else { continue };
        let was = t.clone();
        for n in &names {
            t = crate::dead::drop_use_name(&t, &n.0, &n.1);
        }
        if write && t != was {
            let _ = std::fs::write(&p, t);
        }
    }
    Ok(out)
}
