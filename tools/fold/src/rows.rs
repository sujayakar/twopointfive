//! The list of switch rows (tools/qa/switches.json) laid over the knob tables of rust/game/src/knobs.rs:
//! which Rust path each row is, what type it has, and the text of its page value.

use std::collections::{BTreeMap, HashMap};
use syn::{Expr, Fields, Item, Type};

#[derive(Clone, Debug, PartialEq)]
pub enum Kind {
    Settled,
    Open,
    Player,
}

#[derive(Clone, Debug)]
pub struct Row {
    /// the row's name in the list: `BELIEF.on`
    pub name: String,
    pub kind: Kind,
    /// the page value as the knob tables write it: `true`, `14.0`, `crate::quietfog::QuietRung::Team`
    pub value: String,
    pub bool_value: Option<bool>,
}

/// One struct of the knob tables: its fields in order, each with the last segment of its type.
pub type Tree = HashMap<String, Vec<(String, String)>>;

pub struct Rows {
    /// `belief.on` -> the row
    pub by_path: BTreeMap<String, Row>,
    pub tree: Tree,
    /// a table's struct -> its path from the root (`BeliefK` -> `belief`), where one struct is one table
    pub type_path: HashMap<String, Vec<String>>,
    /// rows of the list with no field in the Rust tables (the host's, the picture's, a table keyed by a name)
    pub not_in_rust: Vec<String>,
    /// rows whose page value in the list is not the value the Rust tables start with
    pub value_differs: Vec<String>,
}

fn last_seg(t: &Type) -> String {
    match t {
        Type::Path(p) => p.path.segments.last().map(|s| s.ident.to_string()).unwrap_or_default(),
        Type::Reference(r) => last_seg(&r.elem),
        Type::Paren(p) => last_seg(&p.elem),
        _ => String::new(),
    }
}

/// `lightSnapPx` -> `light_snap_px`
fn snake(s: &str) -> String {
    let c: Vec<char> = s.chars().collect();
    let mut o = String::new();
    for i in 0..c.len() {
        let ch = c[i];
        if ch.is_ascii_uppercase() {
            let prev_low = i > 0 && (c[i - 1].is_ascii_lowercase() || c[i - 1].is_ascii_digit());
            let next_low = i + 1 < c.len() && c[i + 1].is_ascii_lowercase();
            let prev_up = i > 0 && c[i - 1].is_ascii_uppercase();
            if prev_low || (prev_up && next_low) {
                o.push('_');
            }
            o.push(ch.to_ascii_lowercase());
        } else {
            o.push(ch);
        }
    }
    o
}

fn doc_of(attrs: &[syn::Attribute]) -> String {
    let mut s = String::new();
    for a in attrs {
        if a.path().is_ident("doc") {
            if let syn::Meta::NameValue(nv) = &a.meta {
                if let Expr::Lit(l) = &nv.value {
                    if let syn::Lit::Str(t) = &l.lit {
                        s.push_str(&t.value());
                        s.push('\n');
                    }
                }
            }
        }
    }
    s
}

/// The values a `page_*` function's struct literal gives, by path under it.
fn literal_values(e: &Expr, src: &str, at: &mut Vec<String>, out: &mut HashMap<String, String>) {
    use syn::spanned::Spanned;
    match e {
        Expr::Struct(s) => {
            for f in &s.fields {
                if let syn::Member::Named(n) = &f.member {
                    at.push(n.to_string());
                    literal_values(&f.expr, src, at, out);
                    at.pop();
                }
            }
        }
        _ => {
            out.insert(at.join("."), src[e.span().byte_range()].to_string());
        }
    }
}

pub fn load(switches_json: &str, knobs_rs: &str) -> Result<Rows, String> {
    let file = syn::parse_file(knobs_rs).map_err(|e| format!("knobs.rs does not parse: {e}"))?;
    let mut tree: Tree = HashMap::new();
    // the TypeScript table a struct was written from: `AIM` -> `AimK`
    let mut by_const: HashMap<String, String> = HashMap::new();
    let mut page_fns: HashMap<String, &syn::ItemFn> = HashMap::new();
    for it in &file.items {
        match it {
            Item::Struct(s) => {
                let mut fields = Vec::new();
                if let Fields::Named(n) = &s.fields {
                    for f in &n.named {
                        fields.push((f.ident.as_ref().unwrap().to_string(), last_seg(&f.ty)));
                    }
                }
                tree.insert(s.ident.to_string(), fields);
                // "(TypeScript: inline at src/game/aim.ts:31 AIM, src/game/aim.ts:31)"
                let d = doc_of(&s.attrs);
                if let Some(i) = d.find("TypeScript:") {
                    let rest = &d[i..];
                    for w in rest.split_whitespace() {
                        let w = w.trim_end_matches(|c| c == ',' || c == ')');
                        if !w.is_empty() && w.chars().all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_') && w.chars().next().unwrap().is_ascii_uppercase() {
                            by_const.entry(w.to_string()).or_insert(s.ident.to_string());
                            break;
                        }
                    }
                }
            }
            Item::Fn(f) => {
                let n = f.sig.ident.to_string();
                if n.starts_with("page_") {
                    page_fns.insert(n, f);
                }
            }
            _ => {}
        }
    }
    let root = tree.get("Knobs").ok_or("knobs.rs has no struct Knobs")?.clone();
    // one struct, one table: a struct met twice under the root is no table's own
    let mut type_path: HashMap<String, Vec<String>> = HashMap::new();
    let mut seen_twice: Vec<String> = Vec::new();
    fn walk(tree: &Tree, ty: &str, at: &mut Vec<String>, out: &mut HashMap<String, Vec<String>>, twice: &mut Vec<String>) {
        if let Some(fs) = tree.get(ty) {
            if out.insert(ty.to_string(), at.clone()).is_some() {
                twice.push(ty.to_string());
            }
            for (f, t) in fs {
                at.push(f.clone());
                walk(tree, t, at, out, twice);
                at.pop();
            }
        }
    }
    walk(&tree, "Knobs", &mut Vec::new(), &mut type_path, &mut seen_twice);
    for t in &seen_twice {
        type_path.remove(t);
    }
    // the page values, by path
    let mut values: HashMap<String, String> = HashMap::new();
    for (f, _t) in &root {
        if let Some(pf) = page_fns.get(&format!("page_{f}")) {
            if let Some(syn::Stmt::Expr(e, None)) = pf.block.stmts.last() {
                let mut at = vec![f.clone()];
                literal_values(e, knobs_rs, &mut at, &mut values);
            }
        }
    }
    let table_field: HashMap<&str, &str> = root.iter().map(|(f, t)| (t.as_str(), f.as_str())).collect();

    let json: serde_json::Value = serde_json::from_str(switches_json).map_err(|e| format!("switches.json: {e}"))?;
    let list = json["rows"].as_array().ok_or("switches.json has no rows")?;
    let mut rows = Rows { by_path: BTreeMap::new(), tree: tree.clone(), type_path, not_in_rust: Vec::new(), value_differs: Vec::new() };
    for r in list {
        let name = r["row"].as_str().ok_or("a row with no name")?.to_string();
        let kind = match r["kind"].as_str().unwrap_or("") {
            "settled" => Kind::Settled,
            "open decision" => Kind::Open,
            _ => Kind::Player,
        };
        let parts: Vec<&str> = name.split('.').collect();
        let Some(st) = by_const.get(parts[0]) else { rows.not_in_rust.push(name); continue };
        let Some(field) = table_field.get(st.as_str()) else { rows.not_in_rust.push(name); continue };
        let mut path = vec![field.to_string()];
        let mut ty = st.clone();
        let mut ok = true;
        for p in &parts[1..] {
            let want = snake(p);
            let fs = tree.get(&ty);
            let hit = fs.and_then(|fs| fs.iter().find(|(f, _)| *f == want || *f == format!("{want}_") || *f == format!("r#{want}")));
            match hit {
                Some((f, t)) => {
                    path.push(f.clone());
                    ty = t.clone();
                }
                None => {
                    ok = false;
                    break;
                }
            }
        }
        if !ok || tree.contains_key(&ty) {
            rows.not_in_rust.push(name);
            continue;
        }
        let key = path.join(".");
        let Some(value) = values.get(&key).cloned() else { rows.not_in_rust.push(name); continue };
        let page = &r["page"];
        let bool_value = if ty == "bool" { value.parse::<bool>().ok() } else { None };
        let num_value = if ty == "f64" { value.parse::<f64>().ok() } else { None };
        let same = match page {
            serde_json::Value::Bool(b) => bool_value == Some(*b),
            serde_json::Value::Number(n) => num_value.is_some() && n.as_f64() == num_value,
            serde_json::Value::String(s) => value.rsplit("::").next().map(|v| v.to_lowercase()) == Some(s.to_lowercase()),
            _ => false,
        };
        if !same {
            rows.value_differs.push(format!("{name}: the list says {page}, the tables start with {value}"));
        }
        rows.by_path.insert(key, Row { name, kind, value, bool_value });
    }
    Ok(rows)
}
