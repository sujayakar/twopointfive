//! A knob row leaves the Rust source.
//!
//! `tools/wire/wire.ts drop` takes a field off the wire (the table, io.rs, the page's half). This takes the same
//! field out of the Rust source: its line of the struct in knobs.rs, and its `field: value` in every literal of that
//! struct (the table's blank, its page values, a literal a body writes) and in every pattern that takes it by name.
//! A read of the field that is left is then a compile error: the list is the rows nothing reads, so there is none.

use std::collections::{BTreeMap, HashMap, HashSet};
use std::ops::Range;
use std::path::Path;
use syn::spanned::Spanned;
use syn::visit::{self, Visit};

struct Finder<'a> {
    /// type -> fields to drop
    want: &'a HashMap<String, HashSet<String>>,
    cuts: Vec<Range<usize>>,
    hit: HashSet<(String, String)>,
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
    for c in ["rust/game/src", "rust/game_run/src", "rust/game_face/src"] {
        crate::dead::rs_files(&root.join(c), &mut files);
    }
    let mut hit: HashSet<(String, String)> = HashSet::new();
    let mut places = 0;
    let mut out: BTreeMap<std::path::PathBuf, String> = BTreeMap::new();
    for p in files {
        let name = p.file_name().and_then(|n| n.to_str()).unwrap_or("");
        // (io.rs and puppet.rs are the wire's own, written by its tool)
        if matches!(name, "io.rs" | "puppet.rs") {
            continue;
        }
        let Ok(src) = std::fs::read_to_string(&p) else { continue };
        if !want.keys().any(|t| src.contains(t.as_str())) {
            continue;
        }
        let file = syn::parse_file(&src).map_err(|e| format!("{} does not parse: {e}", p.display()))?;
        let mut f = Finder { want: &want, cuts: Vec::new(), hit: HashSet::new() };
        f.visit_file(&file);
        if f.cuts.is_empty() {
            continue;
        }
        hit.extend(f.hit);
        let mut cuts = f.cuts;
        cuts.sort_by_key(|c| std::cmp::Reverse(c.start));
        cuts.dedup();
        let mut s = src.clone();
        for r in cuts {
            // the field with its comma, and its whole line where it stands alone on one (with the doc lines right above it)
            let mut end = r.end;
            let rest = &s[end..];
            let ws = rest.len() - rest.trim_start_matches(' ').len();
            if rest[ws..].starts_with(',') {
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
