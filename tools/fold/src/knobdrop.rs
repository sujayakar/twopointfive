//! A field leaves the Rust source: a row of a knob table, a setting, any field of a struct.
//!
//! Its line of the struct, its `field: value` in every literal of that struct (the table's blank,
//! its page values, a literal a body writes), every pattern that takes it by name, and every statement that only sets it on a
//! value the text itself says is of that struct (`s.field = …;` where `s` is a parameter of the type, a `let s: Type`, or a
//! `let s = Type { … }` / `Type::f(…)` with `f` a function of the type that hands back `Self`), in the game's crates and the
//! crates that use them (layout.rs).
//! A read of the field that is left is then a compile error: the list is the fields nothing reads, so there is none.

use std::collections::{BTreeMap, HashMap, HashSet};
use std::ops::Range;
use std::path::Path;
use syn::spanned::Spanned;
use syn::visit::{self, Visit};

struct Finder<'a> {
    /// type -> fields to drop
    want: &'a HashMap<String, HashSet<String>>,
    /// type -> its functions that hand back the type itself (`fn zeroed() -> Self`)
    makers: &'a HashMap<String, HashSet<String>>,
    cuts: Vec<Range<usize>>,
    /// the cuts that are whole statements: their `;` goes with them
    stmts: HashSet<usize>,
    hit: HashSet<(String, String)>,
}

/// The type a pattern-and-type or an initialiser says a name is of, where it is one of the wanted types.
fn said_type(want: &HashMap<String, HashSet<String>>, makers: &HashMap<String, HashSet<String>>, ty: Option<&syn::Type>, init: Option<&syn::Expr>) -> Option<String> {
    if let Some(mut t) = ty {
        while let syn::Type::Reference(r) = t {
            t = &r.elem;
        }
        if let syn::Type::Path(p) = t {
            let n = last(&p.path);
            return want.contains_key(&n).then_some(n);
        }
        return None;
    }
    match init? {
        syn::Expr::Struct(s) => Some(last(&s.path)).filter(|n| want.contains_key(n)),
        syn::Expr::Call(c) => {
            let syn::Expr::Path(p) = &*c.func else { return None };
            let segs: Vec<String> = p.path.segments.iter().map(|s| s.ident.to_string()).collect();
            let [.., t, f] = segs.as_slice() else { return None };
            (want.contains_key(t) && makers.get(t).map(|m| m.contains(f)).unwrap_or(false)).then(|| t.clone())
        }
        _ => None,
    }
}

/// The statements of a function that only set a wanted field on a name the text says is of the type.
struct Sets<'a> {
    want: &'a HashMap<String, HashSet<String>>,
    makers: &'a HashMap<String, HashSet<String>>,
    /// name -> the type the text says it is of
    typed: HashMap<String, String>,
    cuts: Vec<Range<usize>>,
}

impl<'ast, 'a> Visit<'ast> for Sets<'a> {
    fn visit_local(&mut self, l: &'ast syn::Local) {
        let (pat, ty) = match &l.pat {
            syn::Pat::Type(t) => (&*t.pat, Some(&*t.ty)),
            p => (p, None),
        };
        if let syn::Pat::Ident(i) = pat {
            match said_type(self.want, self.makers, ty, l.init.as_ref().map(|i| &*i.expr)) {
                Some(t) => {
                    self.typed.insert(i.ident.to_string(), t);
                }
                // (the name bound again to something else: no longer known)
                None => {
                    self.typed.remove(&i.ident.to_string());
                }
            }
        }
        visit::visit_local(self, l);
    }
    fn visit_stmt(&mut self, s: &'ast syn::Stmt) {
        if let syn::Stmt::Expr(syn::Expr::Assign(a), Some(_)) = s {
            if let syn::Expr::Field(f) = &*a.left {
                if let (syn::Expr::Path(p), syn::Member::Named(m)) = (&*f.base, &f.member) {
                    if let Some(t) = p.path.get_ident().and_then(|i| self.typed.get(&i.to_string())) {
                        if self.want[t].contains(&m.to_string()) {
                            self.cuts.push(a.span().byte_range());
                            return;
                        }
                    }
                }
            }
        }
        visit::visit_stmt(self, s);
    }
}

impl<'a> Finder<'a> {
    fn sets(&mut self, sig: &syn::Signature, block: &syn::Block) {
        let mut s = Sets { want: self.want, makers: self.makers, typed: HashMap::new(), cuts: Vec::new() };
        for a in &sig.inputs {
            if let syn::FnArg::Typed(t) = a {
                if let (syn::Pat::Ident(i), Some(ty)) = (&*t.pat, said_type(self.want, self.makers, Some(&t.ty), None)) {
                    s.typed.insert(i.ident.to_string(), ty);
                }
            }
        }
        s.visit_block(block);
        for c in s.cuts {
            self.stmts.insert(c.start);
            self.cuts.push(c);
        }
    }
}

fn last(p: &syn::Path) -> String {
    p.segments.last().map(|s| s.ident.to_string()).unwrap_or_default()
}

impl<'ast, 'a> Visit<'ast> for Finder<'a> {
    fn visit_item_struct(&mut self, s: &'ast syn::ItemStruct) {
        if let Some(fs) = self.want.get(&s.ident.to_string()) {
            if let syn::Fields::Named(n) = &s.fields {
                for f in &n.named {
                    let name = f.ident.as_ref().map(|i| i.to_string()).unwrap_or_default();
                    if fs.contains(&name) || fs.contains(name.trim_start_matches("r#")) {
                        self.cuts.push(f.span().byte_range());
                        self.hit.insert((s.ident.to_string(), name));
                    }
                }
            }
        }
        visit::visit_item_struct(self, s);
    }
    fn visit_item_fn(&mut self, f: &'ast syn::ItemFn) {
        self.sets(&f.sig, &f.block);
        visit::visit_item_fn(self, f);
    }
    fn visit_impl_item_fn(&mut self, f: &'ast syn::ImplItemFn) {
        self.sets(&f.sig, &f.block);
        visit::visit_impl_item_fn(self, f);
    }
    fn visit_expr_struct(&mut self, e: &'ast syn::ExprStruct) {
        if let Some(fs) = self.want.get(&last(&e.path)) {
            for f in &e.fields {
                if let syn::Member::Named(n) = &f.member {
                    if fs.contains(&n.to_string()) {
                        self.cuts.push(f.span().byte_range());
                    }
                }
            }
        }
        visit::visit_expr_struct(self, e);
    }
    fn visit_pat_struct(&mut self, p: &'ast syn::PatStruct) {
        if let Some(fs) = self.want.get(&last(&p.path)) {
            for f in &p.fields {
                if let syn::Member::Named(n) = &f.member {
                    if fs.contains(&n.to_string()) {
                        self.cuts.push(f.span().byte_range());
                    }
                }
            }
        }
        visit::visit_pat_struct(self, p);
    }
}

/// `names`: `knobs::Type.field`, one each. Returns (fields taken out of their struct, places cut, names no struct had).
pub fn drop(root: &Path, names: &[String], write: bool) -> Result<(usize, usize, Vec<String>), String> {
    let mut want: HashMap<String, HashSet<String>> = HashMap::new();
    for n in names {
        let Some((ty, f)) = n.rsplit_once('.') else { return Err(format!("{n}: not `mod::Type.field`")) };
        let ty = ty.rsplit("::").next().unwrap_or(ty);
        want.entry(ty.to_string()).or_default().insert(f.to_string());
    }
    let mut files = Vec::new();
    // (the game's crates and every crate that uses one: wherever a table can be written out as a literal)
    for c in crate::layout::find(root)?.game_and_users() {
        crate::dead::rs_files(&root.join("rust").join(c).join("src"), &mut files);
    }
    // the functions of each wanted type that hand back the type itself: what one of them makes is of the type
    let mut makers: HashMap<String, HashSet<String>> = HashMap::new();
    let mut parsed: Vec<(std::path::PathBuf, String, syn::File)> = Vec::new();
    for p in files {
        let Ok(src) = std::fs::read_to_string(&p) else { continue };
        if !want.keys().any(|t| src.contains(t.as_str())) {
            continue;
        }
        let file = syn::parse_file(&src).map_err(|e| format!("{} does not parse: {e}", p.display()))?;
        for it in &file.items {
            let syn::Item::Impl(im) = it else { continue };
            let (None, syn::Type::Path(tp)) = (&im.trait_, &*im.self_ty) else { continue };
            let t = last(&tp.path);
            if !want.contains_key(&t) {
                continue;
            }
            for x in &im.items {
                let syn::ImplItem::Fn(f) = x else { continue };
                if let syn::ReturnType::Type(_, rt) = &f.sig.output {
                    if let syn::Type::Path(rp) = &**rt {
                        let r = last(&rp.path);
                        if r == "Self" || r == t {
                            makers.entry(t.clone()).or_default().insert(f.sig.ident.to_string());
                        }
                    }
                }
            }
        }
        parsed.push((p, src, file));
    }
    let mut hit: HashSet<(String, String)> = HashSet::new();
    let mut places = 0;
    let mut out: BTreeMap<std::path::PathBuf, String> = BTreeMap::new();
    for (p, src, file) in parsed {
        let mut f = Finder { want: &want, makers: &makers, cuts: Vec::new(), stmts: HashSet::new(), hit: HashSet::new() };
        f.visit_file(&file);
        if f.cuts.is_empty() {
            continue;
        }
        hit.extend(f.hit);
        let stmts = f.stmts;
        let mut cuts = f.cuts;
        cuts.sort_by_key(|c| std::cmp::Reverse(c.start));
        cuts.dedup();
        let mut s = src.clone();
        for r in cuts {
            // the field with its comma (a statement with its `;`), and its whole line where it stands alone on one (with the doc
            // lines right above it)
            let mut end = r.end;
            let rest = &s[end..];
            let ws = rest.len() - rest.trim_start_matches(' ').len();
            if rest[ws..].starts_with(if stmts.contains(&r.start) { ';' } else { ',' }) {
                end += ws + 1;
            }
            let mut start = r.start;
            let ls = s[..start].rfind('\n').map(|i| i + 1).unwrap_or(0);
            let le = s[end..].find('\n').map(|i| end + i + 1).unwrap_or(s.len());
            if s[ls..start].trim().is_empty() && s[end..le].trim().is_empty() {
                start = ls;
                end = le;
                while start > 0 {
                    let ps = s[..start - 1].rfind('\n').map(|i| i + 1).unwrap_or(0);
                    if s[ps..start].trim().starts_with("///") {
                        start = ps;
                    } else {
                        break;
                    }
                }
            } else {
                // inside a line: the blank after the comma goes too; the last field of a literal takes the comma before it
                let after = &s[end..];
                let ws2 = after.len() - after.trim_start_matches(' ').len();
                if s[end..].trim_start_matches(' ').starts_with('}') && s[..start].trim_end().ends_with(',') && !s[r.end..end].contains(',') {
                    start = s[..start].trim_end().len() - 1;
                } else {
                    end += ws2;
                }
            }
            s.replace_range(start..end, "");
            places += 1;
        }
        if syn::parse_file(&s).is_err() {
            return Err(format!("{} would not parse with the fields out", p.display()));
        }
        out.insert(p, s);
    }
    if write {
        for (p, s) in &out {
            std::fs::write(p, s).map_err(|e| format!("cannot write {}: {e}", p.display()))?;
        }
    }
    let mut missing: Vec<String> = Vec::new();
    for (t, fs) in &want {
        for f in fs {
            if !hit.contains(&(t.clone(), f.clone())) {
                missing.push(format!("{t}.{f}"));
            }
        }
    }
    missing.sort();
    Ok((hit.len(), places, missing))
}

/// A type leaves the Rust source: its declaration, its `impl`s and a free function that hands it back (`page_par`: the table's
/// page values, which `fields-drop` has left with no caller; a caller still there is the compile error that says so), in the game's crates and the crates that use them. `names`:
/// `mod::Type`, one each. It is for a type the last field of its kind has left (`fields-drop`): if anything is left that names the type, nothing is written and the places are said.
/// Returns (items cut, what still names a type).
pub fn drop_types(root: &Path, names: &[String], write: bool) -> Result<(usize, Vec<String>), String> {
    let want: HashSet<String> = names.iter().map(|n| n.rsplit("::").next().unwrap_or(n).to_string()).collect();
    let mut files = Vec::new();
    for c in crate::layout::find(root)?.game_and_users() {
        crate::dead::rs_files(&root.join("rust").join(c).join("src"), &mut files);
    }
    let mut items = 0;
    let mut out: BTreeMap<std::path::PathBuf, String> = BTreeMap::new();
    let mut still = Vec::new();
    for p in files {
        let Ok(src) = std::fs::read_to_string(&p) else { continue };
        if !want.iter().any(|t| src.contains(t.as_str())) {
            continue;
        }
        let file = syn::parse_file(&src).map_err(|e| format!("{} does not parse: {e}", p.display()))?;
        let mut cuts: Vec<Range<usize>> = Vec::new();
        for it in &file.items {
            let named = match it {
                syn::Item::Enum(x) => Some(x.ident.to_string()),
                syn::Item::Struct(x) => Some(x.ident.to_string()),
                syn::Item::Type(x) => Some(x.ident.to_string()),
                syn::Item::Impl(x) => match &*x.self_ty {
                    syn::Type::Path(tp) => Some(last(&tp.path)),
                    _ => None,
                },
                syn::Item::Fn(x) => match &x.sig.output {
                    syn::ReturnType::Type(_, t) => match &**t {
                        syn::Type::Path(tp) => Some(last(&tp.path)),
                        _ => None,
                    },
                    _ => None,
                },
                _ => None,
            };
            if named.map(|n| want.contains(&n)).unwrap_or(false) {
                cuts.push(it.span().byte_range());
            }
        }
        let mut s = src.clone();
        cuts.sort_by_key(|c| std::cmp::Reverse(c.start));
        for r in &cuts {
            // the item's whole lines, and one blank line after it
            let start = s[..r.start].rfind('\n').map(|i| i + 1).unwrap_or(0);
            let mut end = s[r.end..].find('\n').map(|i| r.end + i + 1).unwrap_or(s.len());
            if s[end..].starts_with('\n') {
                end += 1;
            }
            s.replace_range(start..end, "");
        }
        items += cuts.len();
        // what is left must not name the type: a word of the text, not a part of a longer name
        for t in &want {
            let mut at = 0;
            while let Some(i) = s[at..].find(t.as_str()) {
                let (a, b) = (at + i, at + i + t.len());
                let word = |c: Option<char>| c.map(|c| c.is_alphanumeric() || c == '_').unwrap_or(false);
                if !word(s[..a].chars().next_back()) && !word(s[b..].chars().next()) {
                    still.push(format!("{}:{} names {t}", p.strip_prefix(root).unwrap_or(&p).display(), s[..a].bytes().filter(|x| *x == b'\n').count() + 1));
                }
                at = b;
            }
        }
        if s != src {
            if syn::parse_file(&s).is_err() {
                return Err(format!("{} would not parse with the types out", p.display()));
            }
            out.insert(p, s);
        }
    }
    if write && still.is_empty() {
        for (p, s) in &out {
            std::fs::write(p, s).map_err(|e| format!("cannot write {}: {e}", p.display()))?;
        }
    }
    Ok((items, still))
}
