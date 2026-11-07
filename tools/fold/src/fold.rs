//! One pass over one body file: where a settled row is read, what can be folded now, what is left.
//!
//! The pass never rewrites a tree. It reads the file with `syn`, and answers with text edits by byte range; the
//! driver applies the innermost ones and reads the file again, until a pass has nothing to say. So every rule
//! here looks at source that is already as folded as its insides can be.
//!
//! WHAT THE FOLD MADE IS MARKED. A literal written in place of a row carries `MARK` behind it, and so does a
//! block the fold emptied and a `return` it laid bare. Every rule but the first asks for the mark: an `if true`
//! or a `let debug = false` that a hand wrote is left alone. The marks are taken out at the end.

use crate::rows::Rows;
use proc_macro2::{TokenStream, TokenTree};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::ops::Range;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::visit::{self, Visit};
use syn::{BinOp, Block, Expr, FnArg, Member, Pat, Stmt, Token, Type, UnOp};

pub const MARK: &str = "/*@F*/";

#[derive(Clone, Debug)]
pub struct Edit {
    pub start: usize,
    pub end: usize,
    pub text: String,
    pub rule: &'static str,
}

/// What a function has become, for its callers.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FnFact {
    /// its body is empty now: a call of it does nothing
    NoOp,
    /// its body is one yes or no
    Const(bool),
}

/// (module, function) -> what it has become
pub type FnTable = HashMap<(String, String), FnFact>;

#[derive(Clone, Debug)]
pub struct KPath {
    pub ty: String,
    pub path: Vec<String>,
}

#[derive(Clone, Debug)]
enum Bind {
    Other,
    Game,
    /// a copy of, or a borrow of, a knob table (`let k = g.k.cover;`, `k: &CoverK`); true = `&mut`
    Alias(KPath, bool),
    /// `let fast = true/*@F*/;`
    BoolLit(bool),
    /// a parameter typed with a table's struct (`k: &CoverK`): it may be the game's table or a caller's own copy,
    /// so a read through it is never folded; a write through it still keeps the row out
    Param(KPath),
}

struct Binding {
    name: String,
    kind: Bind,
    uses: usize,
    /// the `let` statement, where the fold may delete it once nothing uses it
    decl: Option<Range<usize>>,
}

#[derive(Default, Debug)]
pub struct Report {
    /// row path -> reads met (of rows the fold takes)
    pub reads: BTreeMap<String, usize>,
    /// row path -> where it is written (`file:line what`)
    pub writes: BTreeMap<String, Vec<String>>,
    /// table path -> where it is borrowed `&mut` or assigned whole
    pub table_writes: BTreeMap<String, Vec<String>>,
    /// reads this pass could not take, each with its place and why
    pub left: Vec<String>,
    /// row path -> reads met, of rows this run does not fold
    pub other_reads: BTreeMap<String, usize>,
    /// every leaf of the tables (a row or a number), by path -> reads met
    pub leaf_reads: BTreeMap<String, usize>,
    /// a whole table used as a value (copied out, handed on, compared), by path -> times
    pub table_uses: BTreeMap<String, usize>,
    /// row path -> file -> reads of that row met there (taken rows)
    pub read_files: BTreeMap<String, BTreeMap<String, usize>>,
}

pub struct Pass<'a> {
    pub src: &'a str,
    pub file: &'a str,
    pub module: &'a str,
    pub rows: &'a Rows,
    /// the rows this run folds: yes-or-no rows, by path
    pub take: &'a HashSet<String>,
    pub fns: &'a FnTable,
    /// functions each module defines, for a bare call's meaning
    pub module_fns: &'a HashMap<String, HashMap<String, usize>>,
    /// (module, function): its body is a plain reading, with no effects (`pure_fns`)
    pub pure_fns: &'a HashSet<(String, String)>,
    /// true: only count and report, substitute nothing
    pub scan_only: bool,
    pub edits: Vec<Edit>,
    pub report: Report,
    scopes: Vec<Vec<Binding>>,
    impl_ty: Option<String>,
    paren_inner: HashMap<(usize, usize), Range<usize>>,
    delim: HashSet<(usize, usize)>,
    receiver: HashSet<(usize, usize)>,
    claimed: HashSet<(usize, usize)>,
    in_test: usize,
    fn_bodies: HashSet<(usize, usize)>,
    logic_operand: HashSet<(usize, usize)>,
    line_starts: Vec<usize>,
}

fn is_block_like(e: &Expr) -> bool {
    matches!(e, Expr::If(_) | Expr::Match(_) | Expr::Block(_) | Expr::Loop(_) | Expr::While(_) | Expr::ForLoop(_) | Expr::Unsafe(_))
}

fn is_atomic(e: &Expr) -> bool {
    matches!(e, Expr::Path(_) | Expr::Lit(_) | Expr::Call(_) | Expr::MethodCall(_) | Expr::Field(_) | Expr::Index(_) | Expr::Paren(_) | Expr::Macro(_) | Expr::Tuple(_) | Expr::Array(_)) || is_block_like(e)
}

fn diverges(s: &Stmt) -> bool {
    matches!(s, Stmt::Expr(e @ (Expr::Return(_) | Expr::Break(_) | Expr::Continue(_)), _) if expr_attrs(e).is_empty())
}

/// A number written with no type of its own (`0.1`, `-2`, `3 * 0.5`): what it is depends on where it stands.
/// a bare number written with a point (`1.0`), as against a whole one (`1`)
fn has_float(e: &Expr) -> bool {
    match e {
        Expr::Lit(l) => matches!(&l.lit, syn::Lit::Float(_)),
        Expr::Paren(p) => has_float(&p.expr),
        Expr::Unary(u) => has_float(&u.expr),
        Expr::Binary(b) => has_float(&b.left) || has_float(&b.right),
        _ => false,
    }
}

fn bare_number(e: &Expr) -> bool {
    match e {
        Expr::Lit(l) => match &l.lit {
            syn::Lit::Int(i) => i.suffix().is_empty(),
            syn::Lit::Float(f) => f.suffix().is_empty(),
            _ => false,
        },
        Expr::Paren(p) => bare_number(&p.expr),
        Expr::Unary(u) => matches!(u.op, UnOp::Neg(_)) && bare_number(&u.expr),
        Expr::Binary(b) => bare_number(&b.left) && bare_number(&b.right),
        _ => false,
    }
}

/// The names a format string takes by itself: `"{n}"`, `"{fast:?}"`.
fn captures(lit: &str) -> Vec<String> {
    let mut out = Vec::new();
    if !lit.contains('"') {
        return out;
    }
    let c: Vec<char> = lit.chars().collect();
    let mut i = 0;
    while i < c.len() {
        if c[i] == '{' {
            if i + 1 < c.len() && c[i + 1] == '{' {
                i += 2;
                continue;
            }
            let mut j = i + 1;
            let mut name = String::new();
            while j < c.len() && (c[j].is_alphanumeric() || c[j] == '_') {
                name.push(c[j]);
                j += 1;
            }
            if !name.is_empty() && !name.chars().next().unwrap().is_ascii_digit() && j < c.len() && (c[j] == '}' || c[j] == ':') {
                out.push(name);
            }
            i = j;
        }
        i += 1;
    }
    out
}

fn expr_attrs(e: &Expr) -> &[syn::Attribute] {
    match e {
        Expr::If(x) => &x.attrs,
        Expr::ForLoop(x) => &x.attrs,
        Expr::While(x) => &x.attrs,
        Expr::Block(x) => &x.attrs,
        Expr::Assign(x) => &x.attrs,
        Expr::Call(x) => &x.attrs,
        Expr::Return(x) => &x.attrs,
        Expr::Break(x) => &x.attrs,
        Expr::Continue(x) => &x.attrs,
        Expr::Match(x) => &x.attrs,
        Expr::Loop(x) => &x.attrs,
        Expr::MethodCall(x) => &x.attrs,
        Expr::Macro(x) => &x.attrs,
        _ => &[],
    }
}

fn pat_has_ref_mut(p: &Pat) -> bool {
    match p {
        Pat::Ident(i) => (i.by_ref.is_some() && i.mutability.is_some()) || i.subpat.as_ref().map(|(_, s)| pat_has_ref_mut(s)).unwrap_or(false),
        Pat::Or(o) => o.cases.iter().any(pat_has_ref_mut),
        Pat::Paren(x) => pat_has_ref_mut(&x.pat),
        Pat::Reference(r) => r.mutability.is_some() || pat_has_ref_mut(&r.pat),
        Pat::Slice(s) => s.elems.iter().any(pat_has_ref_mut),
        Pat::Struct(s) => s.fields.iter().any(|f| pat_has_ref_mut(&f.pat)),
        Pat::Tuple(t) => t.elems.iter().any(pat_has_ref_mut),
        Pat::TupleStruct(t) => t.elems.iter().any(pat_has_ref_mut),
        Pat::Type(t) => pat_has_ref_mut(&t.pat),
        _ => false,
    }
}

fn single_ident(e: &Expr) -> Option<String> {
    if let Expr::Path(p) = e {
        if p.qself.is_none() && p.path.leading_colon.is_none() && p.path.segments.len() == 1 && p.path.segments[0].arguments.is_none() {
            return Some(p.path.segments[0].ident.to_string());
        }
    }
    None
}

pub fn pat_idents(p: &Pat, out: &mut Vec<String>) {
    match p {
        Pat::Ident(i) => {
            let n = i.ident.to_string();
            // an upper-case name in a pattern is a constant or a variant, not a binding
            if !n.chars().next().map(|c| c.is_ascii_uppercase()).unwrap_or(false) {
                out.push(n);
            }
            if let Some((_, sub)) = &i.subpat {
                pat_idents(sub, out);
            }
        }
        Pat::Or(o) => o.cases.iter().for_each(|c| pat_idents(c, out)),
        Pat::Paren(x) => pat_idents(&x.pat, out),
        Pat::Reference(r) => pat_idents(&r.pat, out),
        Pat::Slice(s) => s.elems.iter().for_each(|c| pat_idents(c, out)),
        Pat::Struct(s) => s.fields.iter().for_each(|f| pat_idents(&f.pat, out)),
        Pat::Tuple(t) => t.elems.iter().for_each(|c| pat_idents(c, out)),
        Pat::TupleStruct(t) => t.elems.iter().for_each(|c| pat_idents(c, out)),
        Pat::Type(t) => pat_idents(&t.pat, out),
        _ => {}
    }
}

const PURE_METHODS: &[&str] = &[
    "is_some", "is_none", "len", "is_empty", "abs", "min", "max", "sqrt", "clone", "copied", "cloned", "unwrap_or", "as_ref", "as_str", "contains", "get", "first",
    "floor", "ceil", "round", "signum", "to_bits", "is_finite", "is_nan", "powi", "sin", "cos", "atan2", "hypot", "eq", "ne", "starts_with", "ends_with", "is_ok", "is_err",
    "contains_key", "has", "trunc", "clamp", "to_string", "as_deref", "unwrap_or_default", "is_some_and", "is_none_or",
];

const PARSED_MACROS: &[&str] = &["vec", "format", "assert", "assert_eq", "assert_ne", "debug_assert", "debug_assert_eq", "debug_assert_ne", "println", "eprintln", "print", "eprint", "panic", "write", "writeln", "matches", "unreachable"];

impl<'a> Pass<'a> {
    pub fn new(src: &'a str, file: &'a str, module: &'a str, rows: &'a Rows, take: &'a HashSet<String>, fns: &'a FnTable, module_fns: &'a HashMap<String, HashMap<String, usize>>, pure_fns: &'a HashSet<(String, String)>, scan_only: bool) -> Self {
        let mut line_starts = vec![0];
        for (i, b) in src.bytes().enumerate() {
            if b == b'\n' {
                line_starts.push(i + 1);
            }
        }
        Pass { src, file, module, rows, take, fns, module_fns, pure_fns, scan_only, edits: Vec::new(), report: Report::default(), scopes: Vec::new(), impl_ty: None, paren_inner: HashMap::new(), delim: HashSet::new(), receiver: HashSet::new(), claimed: HashSet::new(), in_test: 0, fn_bodies: HashSet::new(), logic_operand: HashSet::new(), line_starts }
    }

    fn line_of(&self, at: usize) -> usize {
        match self.line_starts.binary_search(&at) {
            Ok(i) => i + 1,
            Err(i) => i,
        }
    }
    fn place(&self, at: usize) -> String {
        format!("{}:{}", self.file, self.line_of(at))
    }
    fn line_text(&self, at: usize) -> &str {
        let l = self.line_of(at) - 1;
        let s = self.line_starts[l];
        let e = self.line_starts.get(l + 1).copied().unwrap_or(self.src.len());
        self.src[s..e].trim()
    }
    fn left(&mut self, at: usize, why: &str) {
        let t: String = self.line_text(at).chars().take(200).collect();
        self.report.left.push(format!("{} [{}] {}", self.place(at), why, t));
    }
    fn r<T: Spanned>(&self, t: &T) -> Range<usize> {
        t.span().byte_range()
    }
    fn key<T: Spanned>(&self, t: &T) -> (usize, usize) {
        let r = t.span().byte_range();
        (r.start, r.end)
    }
    /// a range with the mark behind it, where its last token is a marked literal
    fn full(&self, r: Range<usize>) -> Range<usize> {
        if self.src[r.end..].starts_with(MARK) {
            r.start..r.end + MARK.len()
        } else {
            r
        }
    }
    fn text(&self, r: Range<usize>) -> &str {
        &self.src[r]
    }
    fn edit(&mut self, r: Range<usize>, text: String, rule: &'static str) {
        if self.scan_only {
            return;
        }
        self.edits.push(Edit { start: r.start, end: r.end, text, rule });
    }

    // ---- scopes ----
    fn bind(&mut self, name: String, kind: Bind, decl: Option<Range<usize>>) {
        if let Some(s) = self.scopes.last_mut() {
            s.push(Binding { name, kind, uses: 0, decl });
        }
    }
    fn bind_pat(&mut self, p: &Pat) {
        let mut v = Vec::new();
        pat_idents(p, &mut v);
        for n in v {
            self.bind(n, Bind::Other, None);
        }
    }
    fn lookup(&self, name: &str) -> Option<&Binding> {
        for s in self.scopes.iter().rev() {
            for b in s.iter().rev() {
                if b.name == name {
                    return Some(b);
                }
            }
        }
        None
    }
    fn used(&mut self, name: &str) {
        for s in self.scopes.iter_mut().rev() {
            for b in s.iter_mut().rev() {
                if b.name == name {
                    b.uses += 1;
                    return;
                }
            }
        }
    }
    fn push(&mut self) {
        self.scopes.push(Vec::new());
    }
    fn pop(&mut self) {
        let Some(s) = self.scopes.pop() else { return };
        for b in s {
            if b.uses > 0 || b.name.starts_with('_') {
                continue;
            }
            let Some(d) = b.decl else { continue };
            match b.kind {
                Bind::BoolLit(_) | Bind::Alias(_, false) => {
                    let r = self.stmt_lines(d, false);
                    self.edit(r, String::new(), "a let nothing reads");
                }
                _ => {}
            }
        }
    }

    fn type_bind(&self, t: &Type) -> Bind {
        let mut inner = t;
        loop {
            match inner {
                Type::Reference(r) => {
                    inner = &r.elem;
                }
                Type::Paren(p) => inner = &p.elem,
                _ => break,
            }
        }
        if let Type::Path(p) = inner {
            if let Some(s) = p.path.segments.last() {
                let n = s.ident.to_string();
                if n == "Game" {
                    return Bind::Game;
                }
                if n == "Self" {
                    if let Some(t) = &self.impl_ty {
                        if t == "Game" {
                            return Bind::Game;
                        }
                        if let Some(path) = self.rows.type_path.get(t) {
                            return Bind::Param(KPath { ty: t.clone(), path: path.clone() });
                        }
                    }
                }
                if let Some(path) = self.rows.type_path.get(&n) {
                    return Bind::Param(KPath { ty: n, path: path.clone() });
                }
            }
        }
        Bind::Other
    }

    fn bind_sig(&mut self, sig: &syn::Signature) {
        for a in &sig.inputs {
            match a {
                FnArg::Receiver(_) => {
                    let k = match &self.impl_ty {
                        Some(t) if t == "Game" => Bind::Game,
                        Some(t) => match self.rows.type_path.get(t) {
                            Some(p) => Bind::Param(KPath { ty: t.clone(), path: p.clone() }),
                            None => Bind::Other,
                        },
                        None => Bind::Other,
                    };
                    self.bind("self".into(), k, None);
                }
                FnArg::Typed(t) => {
                    let kind = self.type_bind(&t.ty);
                    let mut v = Vec::new();
                    pat_idents(&t.pat, &mut v);
                    if v.len() == 1 && matches!(&*t.pat, Pat::Ident(_)) {
                        self.bind(v.remove(0), kind, None);
                    } else {
                        for n in v {
                            self.bind(n, Bind::Other, None);
                        }
                    }
                }
            }
        }
    }

    // ---- what an expression is ----
    fn is_game(&self, e: &Expr) -> bool {
        match e {
            Expr::Paren(p) => self.is_game(&p.expr),
            Expr::Unary(u) if matches!(u.op, UnOp::Deref(_)) => self.is_game(&u.expr),
            Expr::Reference(r) => self.is_game(&r.expr),
            _ => match single_ident(e) {
                Some(n) => matches!(self.lookup(&n), Some(Binding { kind: Bind::Game, .. })),
                None => false,
            },
        }
    }

    /// The knob table or row an expression names, if it names one.
    fn resolve(&self, e: &Expr) -> Option<KPath> {
        match e {
            Expr::Paren(p) => self.resolve(&p.expr),
            Expr::Unary(u) if matches!(u.op, UnOp::Deref(_)) => self.resolve(&u.expr),
            Expr::Field(f) => {
                let Member::Named(m) = &f.member else { return None };
                let m = m.to_string();
                if m == "k" && self.is_game(&f.base) {
                    return Some(KPath { ty: "Knobs".into(), path: Vec::new() });
                }
                let base = self.resolve(&f.base)?;
                let fields = self.rows.tree.get(&base.ty)?;
                let (_, t) = fields.iter().find(|(n, _)| *n == m)?;
                let mut path = base.path.clone();
                path.push(m);
                Some(KPath { ty: t.clone(), path })
            }
            _ => {
                let n = single_ident(e)?;
                match self.lookup(&n) {
                    Some(Binding { kind: Bind::Alias(kp, _), .. }) => Some(kp.clone()),
                    Some(Binding { kind: Bind::Param(kp), .. }) => Some(kp.clone()),
                    _ => None,
                }
            }
        }
    }
    /// The name an expression is rooted in: `k` of `k.on`, `(*k).on`, `k.a.b`.
    fn root_name(e: &Expr) -> Option<String> {
        match e {
            Expr::Field(f) => Self::root_name(&f.base),
            Expr::Paren(p) => Self::root_name(&p.expr),
            Expr::Unary(u) if matches!(u.op, UnOp::Deref(_)) => Self::root_name(&u.expr),
            Expr::Reference(r) => Self::root_name(&r.expr),
            Expr::MethodCall(m) if m.method == "clone" && m.args.is_empty() => Self::root_name(&m.receiver),
            _ => single_ident(e),
        }
    }
    fn is_table(&self, k: &KPath) -> bool {
        self.rows.tree.contains_key(&k.ty)
    }
    /// A `.k.<table>…` chain off a root this pass cannot type: (the knob path, is it a table, is the root one plain name).
    fn unknown_root_path(&self, e: &Expr) -> Option<(String, bool, bool)> {
        let mut names: Vec<String> = Vec::new();
        let mut cur = e;
        let simple;
        loop {
            match cur {
                Expr::Field(f) => {
                    let Member::Named(m) = &f.member else { return None };
                    let m = m.to_string();
                    if m == "k" {
                        simple = single_ident(&f.base).is_some();
                        break;
                    }
                    names.push(m);
                    cur = &f.base;
                }
                Expr::Paren(p) => cur = &p.expr,
                Expr::Unary(u) if matches!(u.op, UnOp::Deref(_)) => cur = &u.expr,
                _ => return None,
            }
        }
        if names.is_empty() {
            // a bare `x.k`: other structs have a field of that name, and nothing here says which this is
            return None;
        }
        names.reverse();
        let mut ty = "Knobs".to_string();
        for n in &names {
            let fs = self.rows.tree.get(&ty)?;
            ty = fs.iter().find(|(f, _)| f == n)?.1.clone();
        }
        Some((names.join("."), self.rows.tree.contains_key(&ty), simple))
    }
    /// The same, where it is a row this run folds.
    fn unknown_root_row(&self, e: &Expr) -> Option<String> {
        match self.unknown_root_path(e) {
            Some((p, false, _)) if self.take.contains(&p) => Some(p),
            _ => None,
        }
    }
    /// A write through a root this pass cannot type keeps the row (or the whole table) out, like any write.
    /// Inside a unit test it is the test's own game, and the test rules speak instead.
    fn foreign_write(&mut self, target: &Expr, what: &str) -> bool {
        if self.in_test > 0 || self.resolve(target).is_some() {
            return false;
        }
        let Some((p, table, _)) = self.unknown_root_path(target) else { return false };
        let at = self.r(target).start;
        let line: String = self.line_text(at).chars().take(160).collect();
        let note = format!("{} {what}, through a name this program cannot type: {}", self.place(at), line);
        if table {
            self.report.table_writes.entry(p).or_default().push(note);
        } else {
            self.report.writes.entry(p).or_default().push(note);
        }
        true
    }

    fn marked_bool(&self, e: &Expr) -> Option<bool> {
        match e {
            Expr::Paren(p) => self.marked_bool(&p.expr),
            Expr::Lit(l) => match &l.lit {
                syn::Lit::Bool(b) if self.src[self.r(e).end..].starts_with(MARK) => Some(b.value),
                _ => None,
            },
            _ => None,
        }
    }
    fn lit(b: bool) -> String {
        format!("{b}{MARK}")
    }

    /// The function of this crate a call names: (module, function). Only where the path leaves no doubt: a bare name
    /// or `super::name` for this module's own function, `crate::<module>::name` for another's, and in both cases a
    /// name the module defines exactly once (a private twin in another part, or a `cfg` pair, is a doubt).
    fn call_target(&self, c: &syn::ExprCall) -> Option<(String, String)> {
        let Expr::Path(p) = &*c.func else { return None };
        if p.qself.is_some() || p.path.leading_colon.is_some() {
            return None;
        }
        let segs: Vec<String> = p.path.segments.iter().map(|s| s.ident.to_string()).collect();
        let (module, name) = match segs.as_slice() {
            [n] => {
                if self.lookup(n).is_some() {
                    return None;
                }
                (self.module.to_string(), n.clone())
            }
            [a, n] if a == "super" || a == "self" => (self.module.to_string(), n.clone()),
            [a, m, n] if a == "crate" => (m.clone(), n.clone()),
            _ => return None,
        };
        if self.module_fns.get(&module).and_then(|s| s.get(&name)).copied() != Some(1) {
            return None;
        }
        Some((module, name))
    }
    fn call_fact(&self, c: &syn::ExprCall) -> Option<FnFact> {
        self.fns.get(&self.call_target(c)?).copied()
    }

    /// No effect when evaluated, by its shape alone. (It may leave out a panic: an index out of range.)
    pub fn pure(&self, e: &Expr) -> bool {
        match e {
            Expr::Lit(_) | Expr::Path(_) => true,
            Expr::Field(f) => self.pure(&f.base),
            Expr::Index(i) => self.pure(&i.expr) && self.pure(&i.index),
            Expr::Unary(u) => self.pure(&u.expr),
            Expr::Binary(b) => {
                !matches!(b.op, BinOp::AddAssign(_) | BinOp::SubAssign(_) | BinOp::MulAssign(_) | BinOp::DivAssign(_) | BinOp::RemAssign(_) | BinOp::BitXorAssign(_) | BinOp::BitAndAssign(_) | BinOp::BitOrAssign(_) | BinOp::ShlAssign(_) | BinOp::ShrAssign(_))
                    && self.pure(&b.left)
                    && self.pure(&b.right)
            }
            Expr::Paren(p) => self.pure(&p.expr),
            Expr::Cast(c) => self.pure(&c.expr),
            Expr::Reference(r) => r.mutability.is_none() && self.pure(&r.expr),
            Expr::Tuple(t) => t.elems.iter().all(|x| self.pure(x)),
            Expr::Array(a) => a.elems.iter().all(|x| self.pure(x)),
            Expr::MethodCall(m) => {
                let name = m.method.to_string();
                // (`min` and `max` of two numbers; with no argument they drain an iterator)
                PURE_METHODS.contains(&name.as_str()) && !((name == "min" || name == "max") && m.args.len() != 1) && self.pure(&m.receiver) && m.args.iter().all(|x| self.pure(x))
            }
            Expr::Call(c) => {
                if !c.args.iter().all(|x| self.pure(x)) {
                    return false;
                }
                if self.call_fact(c).is_some() || self.call_target(c).map(|t| self.pure_fns.contains(&t)).unwrap_or(false) {
                    return true;
                }
                // the vector and number helpers hold no state
                if let Expr::Path(p) = &*c.func {
                    let segs: Vec<String> = p.path.segments.iter().map(|s| s.ident.to_string()).collect();
                    if segs.len() >= 2 && (segs[segs.len() - 2] == "v3" || segs[segs.len() - 2] == "m") {
                        return true;
                    }
                    if segs.len() == 1 && (segs[0] == "Some" || segs[0] == "Ok" || segs[0] == "Err") {
                        return true;
                    }
                }
                false
            }
            _ => false,
        }
    }

    // ---- lines and statements ----
    fn line_start(&self, at: usize) -> usize {
        self.line_starts[self.line_of(at) - 1]
    }
    fn line_end(&self, at: usize) -> usize {
        self.line_starts.get(self.line_of(at)).copied().unwrap_or(self.src.len())
    }
    fn indent_of(&self, at: usize) -> &str {
        let s = self.line_start(at);
        let line = &self.src[s..self.line_end(at)];
        &line[..line.len() - line.trim_start().len()]
    }
    /// A statement's range widened to the whole lines it owns; with `comments`, the comment lines right above it too.
    fn stmt_lines(&self, r: Range<usize>, comments: bool) -> Range<usize> {
        let ls = self.line_start(r.start);
        let before = &self.src[ls..r.start];
        let le = self.line_end(r.end.max(1) - 1);
        let after = self.src[r.end..le].trim();
        if !before.trim().is_empty() || !(after.is_empty() || after.starts_with("//")) {
            // it shares its line: take it, and the blanks beside it
            let mut end = r.end;
            let mut start = r.start;
            if after.is_empty() {
                while start > ls && matches!(self.src.as_bytes()[start - 1], b' ' | b'\t') {
                    start -= 1;
                }
            } else if self.src[end..].starts_with(' ') {
                end += 1;
            }
            return start..end;
        }
        let mut start = ls;
        if comments {
            while start > 0 {
                let ps = self.line_start(start - 1);
                let l = self.src[ps..start].trim();
                if l.starts_with("//") && !l.starts_with("//!") {
                    start = ps;
                } else {
                    break;
                }
            }
        }
        start..le
    }
    fn emptied(&self, b: &Block) -> bool {
        b.stmts.is_empty() && self.src[self.r(b)].contains(MARK)
    }
    /// A block's statements as they would stand in its place, one level out. With `semi_after_tail`, a tail
    /// expression that is no block of its own gets the `;` a statement needs.
    fn spliced(&self, b: &Block, at: usize, semi_after_tail: bool) -> String {
        let r = self.r(b);
        let mut inner = self.src[r.start + 1..r.end - 1].to_string();
        if semi_after_tail {
            if let Some(Stmt::Expr(e, None)) = b.stmts.last() {
                if !is_block_like(e) {
                    let te = self.full(self.r(e)).end - (r.start + 1);
                    inner.insert(te, ';');
                }
            }
        }
        if !inner.contains('\n') {
            return inner.trim().to_string();
        }
        let lines: Vec<&str> = inner.lines().collect();
        let mut lo = 0;
        let mut hi = lines.len();
        while lo < hi && lines[lo].trim().is_empty() {
            lo += 1;
        }
        while hi > lo && lines[hi - 1].trim().is_empty() {
            hi -= 1;
        }
        let body = &lines[lo..hi];
        // a first line that began on the brace's own line has no indentation of its own
        let first_inline = lo == 0;
        let min = body.iter().enumerate().filter(|(i, l)| !l.trim().is_empty() && !(first_inline && *i == 0)).map(|(_, l)| l.len() - l.trim_start().len()).min().unwrap_or(0);
        let to = self.indent_of(at);
        let mut s = String::new();
        for (i, l) in body.iter().enumerate() {
            if i == 0 {
                s.push_str(l.trim_start());
            } else {
                s.push('\n');
                if !l.trim().is_empty() {
                    s.push_str(to);
                    s.push_str(if l.len() >= min && l[..min].trim().is_empty() { &l[min..] } else { l.trim_start() });
                }
            }
        }
        s
    }
    /// The names a block's own statements declare, and whether it holds items.
    fn declares(b: &Block) -> (Vec<String>, bool) {
        let mut v = Vec::new();
        let mut items = false;
        for s in &b.stmts {
            match s {
                Stmt::Local(l) => pat_idents(&l.pat, &mut v),
                Stmt::Item(_) => items = true,
                Stmt::Macro(_) => items = true,
                _ => {}
            }
        }
        (v, items)
    }
    fn mentions(&self, r: Range<usize>, names: &[String]) -> bool {
        if names.is_empty() {
            return false;
        }
        let Ok(ts) = self.src[r].parse::<TokenStream>() else { return true };
        fn walk(ts: TokenStream, names: &[String]) -> bool {
            for t in ts {
                match t {
                    TokenTree::Ident(i) => {
                        let s = i.to_string();
                        if names.iter().any(|n| *n == s) {
                            return true;
                        }
                    }
                    TokenTree::Group(g) => {
                        if walk(g.stream(), names) {
                            return true;
                        }
                    }
                    // a name a format string takes by itself: "{n}"
                    TokenTree::Literal(l) => {
                        if captures(&l.to_string()).iter().any(|c| names.iter().any(|n| n == c)) {
                            return true;
                        }
                    }
                    _ => {}
                }
            }
            false
        }
        walk(ts, names)
    }
    /// A string that runs over lines: moving its lines would change what it says.
    fn multiline_literal(&self, r: Range<usize>) -> bool {
        let Ok(ts) = self.src[r].parse::<TokenStream>() else { return true };
        fn walk(ts: TokenStream) -> bool {
            for t in ts {
                match t {
                    TokenTree::Literal(l) => {
                        if l.span().start().line != l.span().end().line {
                            return true;
                        }
                    }
                    TokenTree::Group(g) => {
                        if walk(g.stream()) {
                            return true;
                        }
                    }
                    _ => {}
                }
            }
            false
        }
        walk(ts)
    }

    /// `if <yes or no> {A} else {B}` standing as a statement of `parent` at `i`.
    fn fold_if_stmt(&mut self, parent: &Block, i: usize, ifx: &syn::ExprIf, semi: bool, on: bool) {
        let stmt_r = self.r(&parent.stmts[i]);
        let is_last = i + 1 == parent.stmts.len();
        let chosen: Option<&Expr> = if on { None } else { ifx.else_branch.as_ref().map(|(_, e)| &**e) };
        let block: Option<&Block> = if on {
            Some(&ifx.then_branch)
        } else {
            match chosen {
                Some(Expr::Block(b)) if b.label.is_none() => Some(&b.block),
                _ => None,
            }
        };
        match (block, chosen) {
            (Some(b), _) if b.stmts.is_empty() => self.delete_stmt(parent, i),
            (Some(b), _) => {
                let (names, items) = Self::declares(b);
                let rest = stmt_r.end..self.r(parent).end - 1;
                let clash = items || self.mentions(rest.clone(), &names) || self.multiline_literal(self.r(b));
                if clash {
                    let t = self.text(self.r(b)).to_string();
                    self.edit(stmt_r, t, "if: the side kept, in its braces");
                    return;
                }
                let tail_stays = is_last && !semi;
                if tail_stays && ifx.else_branch.is_some() && matches!(b.stmts.last(), Some(Stmt::Expr(x, None)) if bare_number(x)) {
                    self.left(stmt_r.start, "the side kept ends in a bare number, whose type the other side may have given: it stays");
                    return;
                }
                let mut text = self.spliced(b, stmt_r.start, !tail_stays);
                let mut end = stmt_r.end;
                let last = b.stmts.last().unwrap();
                if diverges(last) {
                    if matches!(last, Stmt::Expr(Expr::Return(r), _) if r.expr.is_none()) && !text.trim_end().ends_with(MARK) {
                        text.push_str(MARK);
                    }
                    // what follows can never run: it goes, unless an item stands there
                    let mut to_end = is_last;
                    if !is_last && !parent.stmts[i + 1..].iter().any(|s| matches!(s, Stmt::Item(_))) {
                        end = self.r(parent.stmts.last().unwrap()).end;
                        to_end = true;
                    }
                    // a bare `return;` that is now the last word of a function's body says nothing
                    if to_end && self.fn_bodies.contains(&self.key(parent)) && matches!(last, Stmt::Expr(Expr::Return(r), _) if r.expr.is_none()) {
                        let t = text.trim_end().trim_end_matches(MARK).trim_end();
                        if let Some(t) = t.strip_suffix("return;").or(t.strip_suffix("return")) {
                            let t = t.trim_end();
                            text = if t.is_empty() && i == 0 { MARK.to_string() } else { t.to_string() };
                        }
                    }
                }
                self.edit(stmt_r.start..end, text, "if: the side kept");
            }
            (None, Some(e)) => {
                // `else if …`: the rest of the chain stands in its place
                let t = self.text(self.full(self.r(e))).to_string();
                let t = if semi && !is_block_like(e) { format!("{t};") } else { t };
                self.edit(stmt_r, t, "if: the rest of the chain");
            }
            (None, None) => self.delete_stmt(parent, i),
        }
    }

    fn delete_stmt(&mut self, parent: &Block, i: usize) {
        if parent.stmts.len() == 1 {
            let r = self.r(parent);
            self.edit(r.start + 1..r.end - 1, MARK.to_string(), "the block's one statement, deleted");
        } else {
            let r = self.stmt_lines(self.r(&parent.stmts[i]), true);
            self.edit(r, String::new(), "a statement deleted");
        }
    }

    fn negated(&self, cond: &Expr) -> Option<String> {
        fn has_let(e: &Expr) -> bool {
            match e {
                Expr::Let(_) => true,
                Expr::Binary(b) => has_let(&b.left) || has_let(&b.right),
                Expr::Paren(p) => has_let(&p.expr),
                _ => false,
            }
        }
        if has_let(cond) {
            return None;
        }
        Some(match cond {
            Expr::Unary(u) if matches!(u.op, UnOp::Not(_)) => match &*u.expr {
                Expr::Paren(p) => self.text(self.full(self.r(&p.expr))).to_string(),
                x => self.text(self.full(self.r(x))).to_string(),
            },
            x if is_atomic(x) && !is_block_like(x) => format!("!{}", self.text(self.full(self.r(x)))),
            x => format!("!({})", self.text(self.full(self.r(x)))),
        })
    }

    fn macro_tokens(&mut self, mac: &syn::Macro) {
        let name = mac.path.segments.last().map(|s| s.ident.to_string()).unwrap_or_default();
        // a name a format string takes by itself (`"{n}"`) is a use no expression shows
        for t in flat(mac.tokens.clone()) {
            if let TokenTree::Literal(l) = t {
                for c in captures(&l.to_string()) {
                    self.used(&c);
                }
            }
        }
        if PARSED_MACROS.contains(&name.as_str()) {
            if name == "matches" {
                // `matches!(value, pattern if guard)`: the guard is code, under the pattern's names
                let parts = mac.parse_body_with(|input: syn::parse::ParseStream| {
                    let e: Expr = input.parse()?;
                    let _: Token![,] = input.parse()?;
                    let p = Pat::parse_multi_with_leading_vert(input)?;
                    let g: Option<Expr> = if input.peek(Token![if]) {
                        let _: Token![if] = input.parse()?;
                        Some(input.parse()?)
                    } else {
                        None
                    };
                    let _: Option<Token![,]> = input.parse()?;
                    if !input.is_empty() {
                        return Err(input.error("more than a value, a pattern and a guard"));
                    }
                    Ok((e, p, g))
                });
                if let Ok((e, p, g)) = parts {
                    self.delim.insert(self.key(&e));
                    self.visit_expr(&e);
                    if let Some(g) = g {
                        self.push();
                        self.bind_pat(&p);
                        self.delim.insert(self.key(&g));
                        self.visit_expr(&g);
                        self.pop();
                    }
                    return;
                }
            } else if let Ok(args) = mac.parse_body_with(Punctuated::<Expr, Token![,]>::parse_terminated) {
                for e in &args {
                    match e {
                        // `name = value` in a format's arguments
                        Expr::Assign(a) => {
                            self.delim.insert(self.key(&*a.right));
                            self.visit_expr(&a.right);
                        }
                        _ => {
                            self.delim.insert(self.key(e));
                            self.visit_expr(e);
                        }
                    }
                }
                return;
            }
        }
        // not read as code: every name in it counts as used, and a knob it names may be written there
        let at = self.r(mac).start;
        self.unread_tokens(mac.tokens.clone(), at, &format!("inside `{name}!`, which is not read as code"));
    }

    /// Tokens this pass cannot read as code (a macro's own words, a `macro_rules!` body).
    fn unread_tokens(&mut self, ts: TokenStream, at: usize, what: &str) {
        let toks: Vec<TokenTree> = flat(ts);
        let mut i = 0;
        while i < toks.len() {
            match &toks[i] {
                TokenTree::Ident(id) => {
                    let n = id.to_string();
                    // `<anything>.k.<table>…` or `<alias>.<row>`: taken for a write
                    let mut chain: Vec<String> = Vec::new();
                    let mut j = i + 1;
                    while j + 1 < toks.len() {
                        match (&toks[j], &toks[j + 1]) {
                            (TokenTree::Punct(p), TokenTree::Ident(f)) if p.as_char() == '.' => {
                                chain.push(f.to_string());
                                j += 2;
                            }
                            _ => break,
                        }
                    }
                    let after_dot = i > 0 && matches!(&toks[i - 1], TokenTree::Punct(p) if p.as_char() == '.');
                    let start: Option<KPath> = if n == "k" && after_dot {
                        Some(KPath { ty: "Knobs".into(), path: vec![] })
                    } else if after_dot {
                        None
                    } else {
                        match self.lookup(&n) {
                            Some(Binding { kind: Bind::Alias(k, _), .. }) | Some(Binding { kind: Bind::Param(k), .. }) => Some(k.clone()),
                            _ => None,
                        }
                    };
                    if let Some(mut kp) = start {
                        let from = kp.path.len();
                        for f in chain.iter() {
                            let Some(fs) = self.rows.tree.get(&kp.ty) else { break };
                            let Some((_, t)) = fs.iter().find(|(x, _)| x == f) else { break };
                            kp.path.push(f.clone());
                            kp.ty = t.clone();
                        }
                        if kp.path.len() > from || !kp.path.is_empty() {
                            let line: String = self.line_text(at).chars().take(160).collect();
                            let note = format!("{} named {what}: {}", self.place(at), line);
                            let p = kp.path.join(".");
                            if self.in_test > 0 {
                                if self.take.contains(&p) {
                                    self.left(at, &format!("{p} named {what}"));
                                }
                            } else if self.is_table(&kp) {
                                self.report.table_writes.entry(p).or_default().push(note);
                            } else {
                                self.report.writes.entry(p).or_default().push(note);
                            }
                        }
                    }
                    self.used(&n);
                }
                TokenTree::Literal(l) => {
                    for c in captures(&l.to_string()) {
                        self.used(&c);
                    }
                }
                _ => {}
            }
            i += 1;
        }
    }

    fn knob_write(&mut self, target: &Expr, what: &str) -> bool {
        let Some(kp) = self.resolve(target) else { return false };
        let at = self.r(target).start;
        let line: String = self.line_text(at).chars().take(160).collect();
        let note = format!("{} {what}: {}", self.place(at), line);
        if self.is_table(&kp) {
            self.report.table_writes.entry(kp.path.join(".")).or_default().push(note);
        } else {
            self.report.writes.entry(kp.path.join(".")).or_default().push(note);
        }
        true
    }

    fn row_read(&mut self, e: &Expr, kp: &KPath) -> bool {
        let p = kp.path.join(".");
        let Some(row) = self.rows.by_path.get(&p) else { return false };
        if !self.take.contains(&p) || row.bool_value.is_none() {
            *self.report.other_reads.entry(p).or_default() += 1;
            return false;
        }
        *self.report.read_files.entry(p.clone()).or_default().entry(self.file.to_string()).or_default() += 1;
        *self.report.reads.entry(p.clone()).or_default() += 1;
        if let Some(n) = Self::root_name(e) {
            if matches!(self.lookup(&n), Some(Binding { kind: Bind::Param(_), .. })) {
                self.left(self.r(e).start, &format!("{p} read through a parameter typed with its table's struct: the caller may hand in a copy of its own, so it stays"));
                return false;
            }
        }
        let r = self.r(e);
        self.edit(r, format!("{}{MARK}", row.value), "a settled row read");
        true
    }
}

fn flat(ts: TokenStream) -> Vec<TokenTree> {
    let mut v = Vec::new();
    for t in ts {
        match t {
            TokenTree::Group(g) => {
                v.push(TokenTree::Punct(proc_macro2::Punct::new(';', proc_macro2::Spacing::Alone)));
                v.extend(flat(g.stream()));
                v.push(TokenTree::Punct(proc_macro2::Punct::new(';', proc_macro2::Spacing::Alone)));
            }
            t => v.push(t),
        }
    }
    v
}

impl<'ast, 'a> Visit<'ast> for Pass<'a> {
    fn visit_item_impl(&mut self, i: &'ast syn::ItemImpl) {
        let was = self.impl_ty.take();
        if let Type::Path(p) = &*i.self_ty {
            self.impl_ty = p.path.segments.last().map(|s| s.ident.to_string());
        }
        visit::visit_item_impl(self, i);
        self.impl_ty = was;
    }
    fn visit_item_mod(&mut self, m: &'ast syn::ItemMod) {
        let test = m.attrs.iter().any(|a| a.path().is_ident("cfg") && a.meta.require_list().map(|l| l.tokens.to_string().contains("test")).unwrap_or(false));
        if test {
            self.in_test += 1;
        }
        visit::visit_item_mod(self, m);
        if test {
            self.in_test -= 1;
        }
    }
    fn visit_item_macro(&mut self, m: &'ast syn::ItemMacro) {
        // a `macro_rules!` body: what it writes it writes wherever it is used
        let at = self.r(m).start;
        self.unread_tokens(m.mac.tokens.clone(), at, "in a macro's own body");
    }
    fn visit_trait_item_fn(&mut self, f: &'ast syn::TraitItemFn) {
        let saved = std::mem::take(&mut self.scopes);
        self.push();
        self.bind_sig(&f.sig);
        if let Some(b) = &f.default {
            self.visit_block(b);
        }
        self.pop();
        self.scopes = saved;
    }
    fn visit_item_fn(&mut self, f: &'ast syn::ItemFn) {
        let test = f.attrs.iter().any(|a| a.path().is_ident("cfg") && a.meta.require_list().map(|l| l.tokens.to_string().contains("test")).unwrap_or(false));
        if test {
            self.in_test += 1;
        }
        let saved = std::mem::take(&mut self.scopes);
        self.push();
        self.bind_sig(&f.sig);
        if matches!(f.sig.output, syn::ReturnType::Default) {
            self.fn_bodies.insert(self.key(&*f.block));
        }
        self.visit_block(&f.block);
        self.pop();
        self.scopes = saved;
        if test {
            self.in_test -= 1;
        }
    }
    fn visit_impl_item_fn(&mut self, f: &'ast syn::ImplItemFn) {
        let saved = std::mem::take(&mut self.scopes);
        self.push();
        self.bind_sig(&f.sig);
        self.visit_block(&f.block);
        self.pop();
        self.scopes = saved;
    }

    fn visit_block(&mut self, b: &'ast Block) {
        self.push();
        // a function written inside the block shadows the module's of that name
        for s in &b.stmts {
            if let Stmt::Item(syn::Item::Fn(f)) = s {
                self.bind(f.sig.ident.to_string(), Bind::Other, None);
            }
        }
        for (i, s) in b.stmts.iter().enumerate() {
            // an attribute on a statement (`#[cfg(test)] if …`) is part of what it means: no rule touches it
            let plain = match s {
                Stmt::Expr(e, _) => expr_attrs(e).is_empty(),
                _ => true,
            };
            match s {
                _ if !plain => {}
                Stmt::Expr(Expr::If(ifx), semi) => {
                    if let Some(on) = self.marked_bool(&ifx.cond) {
                        self.claimed.insert(self.key(ifx));
                        self.fold_if_stmt(b, i, ifx, semi.is_some(), on);
                    } else if self.emptied(&ifx.then_branch) {
                        self.claimed.insert(self.key(ifx));
                        match &ifx.else_branch {
                            None => {
                                if self.pure(&ifx.cond) {
                                    self.delete_stmt(b, i);
                                } else {
                                    self.left(self.r(ifx).start, "an `if` that guards nothing now, its test not plainly free of effects");
                                }
                            }
                            Some((_, e)) => match &**e {
                                Expr::Block(eb) if self.emptied(&eb.block) => {
                                    if self.pure(&ifx.cond) {
                                        self.delete_stmt(b, i);
                                    } else {
                                        self.left(self.r(ifx).start, "an `if` and `else` that guard nothing now, the test not plainly free of effects");
                                    }
                                }
                                Expr::Block(eb) => match self.negated(&ifx.cond) {
                                    Some(n) => {
                                        let r = self.r(ifx).start..self.r(&eb.block).start;
                                        self.edit(r, format!("if {n} "), "if: only the else is left");
                                    }
                                    None => self.left(self.r(ifx).start, "an `if let` whose own side is empty now"),
                                },
                                _ => self.left(self.r(ifx).start, "an `if` whose own side is empty now, before an `else if`"),
                            },
                        }
                    }
                }
                Stmt::Expr(Expr::ForLoop(f), _) if self.emptied(&f.body) => {
                    // (walking a thing can be an effect of its own: an iterator that rolls the die as it goes)
                    self.left(self.r(f).start, "a loop over nothing now: it stays, for what walking it may do");
                }
                Stmt::Expr(Expr::Block(x), _) if x.label.is_none() && self.emptied(&x.block) => self.delete_stmt(b, i),
                Stmt::Expr(Expr::While(w), _) if self.emptied(&w.body) => self.left(self.r(w).start, "a `while` with nothing in it now"),
                Stmt::Expr(Expr::Assign(a), Some(_)) => {
                    // a write of the row's own value off a root this pass cannot type (a test's own game): a dead store
                    if self.in_test > 0 && self.resolve(&a.left).is_none() && matches!(self.unknown_root_path(&a.left), Some((_, false, true))) {
                        if let Some(p) = self.unknown_root_row(&a.left) {
                            let page = self.rows.by_path.get(&p).and_then(|r| r.bool_value);
                            if matches!(&*a.right, Expr::Lit(l) if matches!(&l.lit, syn::Lit::Bool(b) if Some(b.value) == page)) {
                                self.delete_stmt(b, i);
                            }
                        }
                    }
                }
                Stmt::Expr(Expr::Call(c), _) => {
                    if self.call_fact(c) == Some(FnFact::NoOp) && c.args.iter().all(|a| self.pure(a)) {
                        self.claimed.insert(self.key(c));
                        self.delete_stmt(b, i);
                    }
                }
                _ => {}
            }
            match s {
                Stmt::Expr(e, None) if i + 1 == b.stmts.len() => {
                    self.delim.insert(self.key(e));
                }
                Stmt::Expr(e, Some(_)) => {
                    self.delim.insert(self.key(e));
                }
                _ => {}
            }
            self.visit_stmt(s);
        }
        self.pop();
    }

    fn visit_stmt_macro(&mut self, m: &'ast syn::StmtMacro) {
        self.macro_tokens(&m.mac);
    }

    fn visit_local(&mut self, l: &'ast syn::Local) {
        let mut kind = Bind::Other;
        let mut decl = None;
        let simple = match &l.pat {
            Pat::Ident(i) if i.subpat.is_none() && i.by_ref.is_none() => Some((i.ident.to_string(), i.mutability.is_some())),
            Pat::Type(t) => match &*t.pat {
                Pat::Ident(i) if i.subpat.is_none() && i.by_ref.is_none() => Some((i.ident.to_string(), i.mutability.is_some())),
                _ => None,
            },
            _ => None,
        };
        let mut alias_init = false;
        if let (Pat::Tuple(pt), Some(init)) = (&l.pat, &l.init) {
            if let (Expr::Tuple(et), true, true) = (&*init.expr, init.diverge.is_none(), l.attrs.is_empty()) {
                let plain = |p: &Pat| matches!(p, Pat::Ident(i) if i.subpat.is_none() && i.by_ref.is_none());
                if pt.elems.len() == et.elems.len() && pt.elems.len() > 1 && pt.elems.iter().all(plain) {
                    let settled: Vec<usize> = (0..pt.elems.len()).filter(|&i| self.marked_bool(&et.elems[i]).is_some() && matches!(&pt.elems[i], Pat::Ident(p) if p.mutability.is_none())).collect();
                    if !settled.is_empty() {
                        let ind = self.indent_of(self.r(l).start).to_string();
                        let mut out = String::new();
                        for &i in &settled {
                            out.push_str(&format!("let {} = {};\n{ind}", self.text(self.r(&pt.elems[i])), self.text(self.full(self.r(&et.elems[i])))));
                        }
                        let rest: Vec<usize> = (0..pt.elems.len()).filter(|i| !settled.contains(i)).collect();
                        let pats: Vec<&str> = rest.iter().map(|&i| self.text(self.r(&pt.elems[i]))).collect();
                        let vals: Vec<&str> = rest.iter().map(|&i| self.text(self.full(self.r(&et.elems[i])))).collect();
                        match rest.len() {
                            0 => out = out.trim_end().to_string(),
                            1 => out.push_str(&format!("let {} = {};", pats[0], vals[0])),
                            _ => out.push_str(&format!("let ({}) = ({});", pats.join(", "), vals.join(", "))),
                        }
                        // (only where the statement stands alone on its line: the new lines take its indent)
                        let r = self.r(l);
                        if self.src[self.line_start(r.start)..r.start].trim().is_empty() {
                            self.edit(r, out, "a tuple let: the settled name on its own");
                        }
                    }
                }
            }
        }
        if let Some(init) = &l.init {
            self.delim.insert(self.key(&*init.expr));
            if pat_has_ref_mut(&l.pat) && !self.knob_write(&init.expr, "taken by `ref mut`") {
                self.foreign_write(&init.expr, "taken by `ref mut`");
            }
            if let Some((_, is_mut)) = &simple {
                // `let k = g.k.cover;` · `let k = &g.k.cover;` · `let k = g.k.cover.clone();` · `let k = &mut g.k.cover;`
                let mut e: &Expr = &init.expr;
                let mut by_mut = false;
                loop {
                    match e {
                        Expr::Reference(r) => {
                            by_mut |= r.mutability.is_some();
                            e = &r.expr;
                        }
                        Expr::Paren(p) => e = &p.expr,
                        Expr::MethodCall(m) if m.method == "clone" && m.args.is_empty() => e = &m.receiver,
                        _ => break,
                    }
                }
                if init.diverge.is_none() {
                    if let Some(kp) = self.resolve(e) {
                        if self.is_table(&kp) {
                            if by_mut {
                                let at = self.r(&*init.expr).start;
                                let line: String = self.line_text(at).chars().take(160).collect();
                                let note = format!("{} borrowed `&mut`: {}", self.place(at), line);
                                self.report.table_writes.entry(kp.path.join(".")).or_default().push(note);
                            }
                            if !*is_mut && !matches!(Self::root_name(e).and_then(|n| self.lookup(&n).map(|b| matches!(b.kind, Bind::Param(_)))), Some(true)) {
                                kind = Bind::Alias(kp, by_mut);
                                decl = if by_mut || !l.attrs.is_empty() { None } else { Some(self.r(l)) };
                                alias_init = true;
                                // (an alias of an alias uses the first)
                                if let Some(n) = Self::root_name(e) {
                                    self.used(&n);
                                }
                            }
                        }
                    } else if self.is_game(e) {
                        // `let v = &mut *g;`: another name for the game
                        kind = Bind::Game;
                    } else if !*is_mut && l.attrs.is_empty() {
                        if let Some(b) = self.marked_bool(&init.expr) {
                            kind = Bind::BoolLit(b);
                            decl = Some(self.r(l));
                        }
                    }
                }
            }
            if !alias_init {
                self.visit_expr(&init.expr);
            }
            if let Some((_, d)) = &init.diverge {
                self.visit_expr(d);
            }
        }
        // `let v: &mut Game = …;`
        if let (Bind::Other, Pat::Type(t)) = (&kind, &l.pat) {
            if matches!(self.type_bind(&t.ty), Bind::Game) {
                kind = Bind::Game;
            }
        }
        match simple {
            Some((n, _)) => self.bind(n, kind, decl),
            None => self.bind_pat(&l.pat),
        }
    }

    fn visit_expr_closure(&mut self, c: &'ast syn::ExprClosure) {
        self.push();
        for p in &c.inputs {
            // `|v: &mut Game| …`: the game under another name
            match p {
                Pat::Type(t) if matches!(&*t.pat, Pat::Ident(_)) => {
                    let kind = self.type_bind(&t.ty);
                    let mut v = Vec::new();
                    pat_idents(&t.pat, &mut v);
                    for n in v {
                        self.bind(n, kind.clone(), None);
                    }
                }
                _ => self.bind_pat(p),
            }
        }
        self.delim.insert(self.key(&*c.body));
        self.visit_expr(&c.body);
        self.pop();
    }
    fn visit_arm(&mut self, a: &'ast syn::Arm) {
        self.push();
        self.bind_pat(&a.pat);
        if let Some((if_tok, g)) = &a.guard {
            // `pattern if true =>`: the guard says nothing (one that is settled no is left: its arm never matches)
            if self.marked_bool(g) == Some(true) && a.attrs.is_empty() {
                let from = self.r(&a.pat).end;
                let to = self.full(self.r(&**g)).end;
                let _ = if_tok;
                self.edit(from..to, String::new(), "a match guard that is always yes");
            }
            self.visit_expr(g);
        }
        self.delim.insert(self.key(&*a.body));
        self.visit_expr(&a.body);
        self.pop();
    }
    fn visit_expr_for_loop(&mut self, f: &'ast syn::ExprForLoop) {
        self.visit_expr(&f.expr);
        self.push();
        self.bind_pat(&f.pat);
        self.visit_block(&f.body);
        self.pop();
    }
    fn visit_expr_while(&mut self, w: &'ast syn::ExprWhile) {
        self.push();
        self.delim.insert(self.key(&*w.cond));
        self.visit_expr(&w.cond);
        self.visit_block(&w.body);
        self.pop();
    }
    fn visit_expr_let(&mut self, l: &'ast syn::ExprLet) {
        self.visit_expr(&l.expr);
        self.bind_pat(&l.pat);
    }
    fn visit_expr_if(&mut self, ifx: &'ast syn::ExprIf) {
        // the chain behind it: `else if <yes or no> {…}`, an `else` the fold emptied
        if let Some((else_tok, e)) = &ifx.else_branch {
            let then_end = self.r(&ifx.then_branch).end;
            match &**e {
                Expr::If(inner) if !inner.attrs.is_empty() || !ifx.attrs.is_empty() => {}
                Expr::If(inner) => {
                    if let Some(on) = self.marked_bool(&inner.cond) {
                        self.claimed.insert(self.key(inner));
                        let ir = self.r(inner);
                        if on {
                            let t = self.text(self.r(&inner.then_branch)).to_string();
                            self.edit(ir, t, "else if: its side kept as the else");
                        } else {
                            match &inner.else_branch {
                                None => self.edit(then_end..ir.end, String::new(), "else if: never"),
                                Some((_, e2)) => {
                                    let t = self.text(self.r(&**e2)).to_string();
                                    self.edit(ir, t, "else if: its else kept");
                                }
                            }
                        }
                    } else if self.emptied(&inner.then_branch) && inner.else_branch.is_none() {
                        self.claimed.insert(self.key(inner));
                        if self.pure(&inner.cond) {
                            self.edit(then_end..self.r(inner).end, String::new(), "else if: guards nothing");
                        } else {
                            self.left(self.r(inner).start, "an `else if` that guards nothing now, its test not plainly free of effects");
                        }
                    }
                }
                Expr::Block(b) if self.emptied(&b.block) && !self.emptied(&ifx.then_branch) => {
                    let _ = else_tok;
                    self.edit(then_end..self.r(b).end, String::new(), "else: emptied");
                }
                _ => {}
            }
        }
        self.push();
        self.delim.insert(self.key(&*ifx.cond));
        self.visit_expr(&ifx.cond);
        self.visit_block(&ifx.then_branch);
        self.pop();
        if let Some((_, e)) = &ifx.else_branch {
            self.visit_expr(e);
        }
    }

    fn visit_expr(&mut self, e: &'ast Expr) {
        match e {
            Expr::Field(_) => {
                if let Some(kp) = self.resolve(e) {
                    if !self.is_table(&kp) {
                        *self.report.leaf_reads.entry(kp.path.join(".")).or_default() += 1;
                        if self.row_read(e, &kp) {
                            return;
                        }
                    } else {
                        *self.report.table_uses.entry(kp.path.join(".")).or_default() += 1;
                    }
                    // a table, or a row the fold does not take: only its names are walked
                    if let Some(n) = Self::root_name(e) {
                        self.used(&n);
                    }
                    return;
                }
                if let Some(p) = self.unknown_root_row(e) {
                    self.left(self.r(e).start, &format!("{p} read off a root this program does not know for the game"));
                }
            }
            Expr::Path(_) => {
                if let Some(n) = single_ident(e) {
                    if let Some(Binding { kind: Bind::BoolLit(b), .. }) = self.lookup(&n) {
                        let b = *b;
                        self.edit(self.r(e), Self::lit(b), "a name for a settled yes or no");
                    }
                    self.used(&n);
                }
                return;
            }
            Expr::Assign(a) => {
                if self.knob_write(&a.left, "assigned") {
                    self.delim.insert(self.key(&*a.right));
                    self.visit_expr(&a.right);
                    return;
                }
                self.delim.insert(self.key(&*a.right));
                if matches!(&*a.left, Expr::Unary(u) if matches!(u.op, UnOp::Deref(_)) && self.is_game(&u.expr)) {
                    let at = self.r(e).start;
                    let line: String = self.line_text(at).chars().take(160).collect();
                    let note = format!("{} the whole game assigned: {}", self.place(at), line);
                    self.report.table_writes.entry(String::new()).or_default().push(note);
                }
                // `v.k = Knobs::page()` through a name this pass cannot type: every table at once
                if self.in_test == 0 && matches!(&*a.left, Expr::Field(f) if matches!(&f.member, Member::Named(m) if m == "k") && !self.is_game(&f.base)) && self.text(self.r(&*a.right)).contains("Knobs") {
                    let at = self.r(e).start;
                    let line: String = self.line_text(at).chars().take(160).collect();
                    let note = format!("{} every table assigned, through a name this program cannot type: {}", self.place(at), line);
                    self.report.table_writes.entry(String::new()).or_default().push(note);
                }
                if self.foreign_write(&a.left, "assigned") {
                    self.visit_expr(&a.right);
                    return;
                }
                if let Some(p) = self.unknown_root_row(&a.left) {
                    let page = self.rows.by_path.get(&p).and_then(|r| r.bool_value);
                    if !matches!(&*a.right, Expr::Lit(l) if matches!(&l.lit, syn::Lit::Bool(b) if Some(b.value) == page)) {
                        self.left(self.r(e).start, &format!("{p} is set to another value here, off a game this program cannot type (a test of the side that is gone)"));
                    }
                    self.visit_expr(&a.right);
                    return;
                }
            }
            Expr::Reference(r) if r.mutability.is_some() => {
                if self.knob_write(&r.expr, "borrowed `&mut`") || self.foreign_write(&r.expr, "borrowed `&mut`") {
                    return;
                }
            }
            Expr::Unary(u) if matches!(u.op, UnOp::Not(_)) => {
                if let Some(b) = self.marked_bool(&u.expr) {
                    self.edit(self.full(self.r(e)), Self::lit(!b), "not");
                }
            }
            Expr::Binary(b) => {
                let assign = matches!(b.op, BinOp::AddAssign(_) | BinOp::SubAssign(_) | BinOp::MulAssign(_) | BinOp::DivAssign(_) | BinOp::RemAssign(_) | BinOp::BitXorAssign(_) | BinOp::BitAndAssign(_) | BinOp::BitOrAssign(_) | BinOp::ShlAssign(_) | BinOp::ShrAssign(_));
                if assign && (self.knob_write(&b.left, "assigned") || self.foreign_write(&b.left, "assigned")) {
                    self.visit_expr(&b.right);
                    return;
                }
                let whole = self.full(self.r(e));
                let and = matches!(b.op, BinOp::And(_));
                let or = matches!(b.op, BinOp::Or(_));
                if and || or {
                    for x in [&*b.left, &*b.right] {
                        if let Expr::Paren(_) = x {
                            self.logic_operand.insert(self.key(x));
                        }
                    }
                    let l = self.marked_bool(&b.left);
                    let r = self.marked_bool(&b.right);
                    // the value that decides the whole: false for `&&`, true for `||`
                    let decides = or;
                    let mut out: Option<(String, bool)> = None; // (text, is an operand's own text)
                    if let Some(lv) = l {
                        out = Some(if lv == decides { (Self::lit(decides), false) } else { (self.text(self.full(self.r(&*b.right))).to_string(), true) });
                    } else if let Some(rv) = r {
                        if rv != decides {
                            out = Some((self.text(self.full(self.r(&*b.left))).to_string(), true));
                        } else if self.pure(&b.left) {
                            out = Some((Self::lit(decides), false));
                        } else {
                            self.left(whole.start, "a test that is settled only after a call: the call's effects may matter, so it stays");
                        }
                    }
                    if let Some((mut text, operand)) = out {
                        // brackets a hand put round the pair go with it, where one plain operand is left
                        let kept = if l.is_some() { &*b.right } else { &*b.left };
                        let mut range = whole.clone();
                        let mut range_key = self.key(e);
                        if let Some(pr) = self.paren_inner.get(&self.key(e)).cloned() {
                            // (a comparison or a `!` binds tighter than the `&&` or `||` the brackets stand under)
                            let tight = self.logic_operand.contains(&(pr.start, pr.end)) && match kept {
                                Expr::Binary(k) => matches!(k.op, BinOp::Eq(_) | BinOp::Ne(_) | BinOp::Lt(_) | BinOp::Le(_) | BinOp::Gt(_) | BinOp::Ge(_)),
                                Expr::Unary(_) => true,
                                _ => false,
                            };
                            if !operand || tight || (is_atomic(kept) && !is_block_like(kept)) || self.delim.contains(&(pr.start, pr.end)) {
                                range_key = (pr.start, pr.end);
                                range = pr;
                            }
                        }
                        // and the operand's own brackets, where it now stands alone in a place that needs none
                        if operand && self.delim.contains(&range_key) {
                            if let Expr::Paren(pk) = kept {
                                text = self.text(self.full(self.r(&*pk.expr))).to_string();
                            }
                        }
                        self.edit(range, text, if and { "and" } else { "or" });
                    }
                } else if let BinOp::Eq(_) | BinOp::Ne(_) = b.op {
                    // `x == true`
                    if self.marked_bool(&b.left).is_some() || self.marked_bool(&b.right).is_some() {
                        self.left(whole.start, "a settled yes or no compared with a value");
                    }
                }
            }
            Expr::Paren(p) => {
                let inner = self.key(&*p.expr);
                self.paren_inner.insert(inner, self.r(e));
                // a marked literal alone in brackets a hand wrote: the brackets go
                if self.marked_bool(&p.expr).is_some() && matches!(&*p.expr, Expr::Lit(_)) {
                    let t = self.text(self.full(self.r(&*p.expr))).to_string();
                    self.edit(self.r(e), t, "brackets round a literal");
                    return;
                }
            }
            Expr::Call(c) => {
                for a in &c.args {
                    self.delim.insert(self.key(a));
                }
                if !self.claimed.contains(&self.key(c)) {
                    if let Some(f) = self.call_fact(c) {
                        if c.args.iter().all(|a| self.pure(a)) {
                            match f {
                                FnFact::Const(b) => {
                                    self.edit(self.r(e), Self::lit(b), "a call of a function that is one yes or no");
                                }
                                FnFact::NoOp => {
                                    self.edit(self.r(e), "()".to_string(), "a call of a function that does nothing");
                                }
                            }
                        } else {
                            self.left(self.r(e).start, "a call of a function the fold emptied, its arguments not plainly free of effects");
                        }
                    }
                }
            }
            Expr::MethodCall(m) => {
                // a method on a whole table may write it (`&mut self` shows nowhere at the call)
                if m.method != "clone" {
                    if let Some(kp) = self.resolve(&m.receiver) {
                        if self.is_table(&kp) {
                            self.knob_write(&m.receiver, &format!("`.{}()` called on it", m.method));
                        }
                    } else if matches!(self.unknown_root_path(&m.receiver), Some((_, true, _))) {
                        self.foreign_write(&m.receiver, &format!("`.{}()` called on it", m.method));
                    }
                }
                self.receiver.insert(self.key(&*m.receiver));
                for a in &m.args {
                    self.delim.insert(self.key(a));
                }
            }
            Expr::Return(r) => {
                if let Some(x) = &r.expr {
                    self.delim.insert(self.key(&**x));
                }
            }
            Expr::Break(r) => {
                if let Some(x) = &r.expr {
                    self.delim.insert(self.key(&**x));
                }
            }
            Expr::Index(i) => {
                self.delim.insert(self.key(&*i.index));
            }
            Expr::Array(a) => {
                for x in &a.elems {
                    self.delim.insert(self.key(x));
                }
            }
            Expr::Tuple(a) => {
                for x in &a.elems {
                    self.delim.insert(self.key(x));
                }
            }
            Expr::Struct(s) => {
                for f in &s.fields {
                    if f.colon_token.is_none() {
                        // `Foo { fast }`: the name is the field and the value at once
                        if let Some(n) = single_ident(&f.expr) {
                            if let Some(Binding { kind: Bind::BoolLit(b), .. }) = self.lookup(&n) {
                                let b = *b;
                                self.edit(self.r(&f.expr), format!("{n}: {}", Self::lit(b)), "a name for a settled yes or no");
                            }
                            self.used(&n);
                            continue;
                        }
                    }
                    self.delim.insert(self.key(&f.expr));
                    self.visit_expr(&f.expr);
                }
                if let Some(r) = &s.rest {
                    self.visit_expr(r);
                }
                return;
            }
            Expr::Macro(m) => {
                self.macro_tokens(&m.mac);
                return;
            }
            Expr::Match(m) => {
                if m.arms.iter().any(|a| pat_has_ref_mut(&a.pat)) && !self.knob_write(&m.expr, "taken by `ref mut`") {
                    self.foreign_write(&m.expr, "taken by `ref mut`");
                }
                self.delim.insert(self.key(&*m.expr));
                if self.marked_bool(&m.expr).is_some() {
                    self.left(self.r(e).start, "a `match` on a settled yes or no");
                }
            }
            Expr::If(ifx) => {
                // an `if` standing as a value
                if !self.claimed.contains(&self.key(ifx)) && ifx.attrs.is_empty() && self.marked_bool(&ifx.cond).is_none() {
                    // `if c { true } else { true }`: the test decides nothing, and goes where it plainly has no effects
                    let one = |b: &Block| -> Option<bool> { match b.stmts.as_slice() { [Stmt::Expr(x, None)] => self.marked_bool(x), _ => None } };
                    if let (Some(a), Some((_, els))) = (one(&ifx.then_branch), &ifx.else_branch) {
                        if let Expr::Block(eb) = &**els {
                            if one(&eb.block) == Some(a) && eb.label.is_none() && self.pure(&ifx.cond) {
                                self.edit(self.r(e), Self::lit(a), "if: both sides the same yes or no");
                            }
                        }
                    }
                }
                if !self.claimed.contains(&self.key(ifx)) && ifx.attrs.is_empty() {
                    if let Some(on) = self.marked_bool(&ifx.cond) {
                        let whole = self.r(e);
                        let kept_block: Option<&Block> = if on { Some(&ifx.then_branch) } else { match ifx.else_branch.as_ref().map(|(_, x)| &**x) { Some(Expr::Block(b)) => Some(&b.block), _ => None } };
                        let kept_tail: Option<&Expr> = kept_block.and_then(|b| b.stmts.last()).and_then(|s| match s { Stmt::Expr(x, None) => Some(x), _ => None });
                        // (where the side that goes ends in a bare number of the same kind, neither side gave the type: nothing is lost)
                        let gone_block: Option<&Block> = if on { match ifx.else_branch.as_ref().map(|(_, x)| &**x) { Some(Expr::Block(b)) => Some(&b.block), _ => None } } else { Some(&ifx.then_branch) };
                        let gone_tail: Option<&Expr> = gone_block.and_then(|b| b.stmts.last()).and_then(|s| match s { Stmt::Expr(x, None) => Some(x), _ => None });
                        let same_kind = match (kept_tail, gone_tail) {
                            (Some(a), Some(b)) => bare_number(a) && bare_number(b) && has_float(a) == has_float(b),
                            _ => false,
                        };
                        if ifx.else_branch.is_some() && kept_tail.map(bare_number).unwrap_or(false) && !same_kind {
                            self.left(whole.start, "the side kept is a bare number, whose type the other side may have given: it stays");
                            visit::visit_expr(self, e);
                            return;
                        }
                        let chosen: Option<&Expr> = if on { None } else { ifx.else_branch.as_ref().map(|(_, x)| &**x) };
                        let block: Option<&Block> = if on {
                            Some(&ifx.then_branch)
                        } else {
                            match chosen {
                                Some(Expr::Block(b)) if b.label.is_none() => Some(&b.block),
                                _ => None,
                            }
                        };
                        let text = match (block, chosen) {
                            (Some(b), _) => match (b.stmts.len(), b.stmts.last()) {
                                (1, Some(Stmt::Expr(x, None))) => {
                                    let t = self.text(self.full(self.r(x))).to_string();
                                    if (is_atomic(x) && !is_block_like(x)) || self.delim.contains(&(whole.start, whole.end)) {
                                        t
                                    } else {
                                        format!("({t})")
                                    }
                                }
                                _ => self.text(self.r(b)).to_string(),
                            },
                            (None, Some(x)) => self.text(self.full(self.r(x))).to_string(),
                            (None, None) => "()".to_string(),
                        };
                        // (`(if K { 1.0 } else { 0.0 })` kept as `1.0`: the parentheses were the if's)
                        let atom = matches!(block.map(|b| (b.stmts.len(), b.stmts.last())), Some((1, Some(Stmt::Expr(x, None)))) if is_atomic(x) && !is_block_like(x));
                        match self.paren_inner.get(&(whole.start, whole.end)).cloned() {
                            Some(pr) if atom => self.edit(pr, text, "if as a value: the side kept"),
                            _ => self.edit(whole, text, "if as a value: the side kept"),
                        }
                    }
                }
            }
            _ => {}
        }
        visit::visit_expr(self, e);
    }
}

/// The free functions of a body file whose body is a plain reading: `let`s of expressions with no effects, then
/// one such expression. `pass` is a scan pass over the same file (for what its calls name and what is known pure).
/// A function under a `cfg` is not judged: its twin may differ.
pub fn pure_fns_of(file: &syn::File, pass: &mut Pass) -> Vec<String> {
    let mut out = Vec::new();
    for it in &file.items {
        let syn::Item::Fn(f) = it else { continue };
        let n = f.block.stmts.len();
        if n == 0 || f.attrs.iter().any(|a| a.path().is_ident("cfg")) {
            continue;
        }
        // (its own parameters shadow the module's functions: `roll()` may be a closure handed in)
        pass.push();
        pass.bind_sig(&f.sig);
        let ok = f.block.stmts.iter().enumerate().all(|(i, s)| match s {
            Stmt::Local(l) => i + 1 < n && matches!(&l.pat, Pat::Ident(p) if p.mutability.is_none() && p.by_ref.is_none()) && matches!(&l.init, Some(init) if init.diverge.is_none() && pass.pure(&init.expr)),
            Stmt::Expr(e, None) => i + 1 == n && pass.pure(e),
            _ => false,
        });
        pass.scopes.pop();
        if ok {
            out.push(f.sig.ident.to_string());
        }
    }
    out
}

/// What the functions of one body file have become: (name, fact) for each free function whose whole body the fold settled.
pub fn fn_facts(src: &str, file: &syn::File) -> Vec<(String, FnFact)> {
    let mut out = Vec::new();
    for it in &file.items {
        let syn::Item::Fn(f) = it else { continue };
        if f.attrs.iter().any(|a| a.path().is_ident("cfg")) {
            continue;
        }
        let name = f.sig.ident.to_string();
        let body = &src[f.block.span().byte_range()];
        let unit = matches!(f.sig.output, syn::ReturnType::Default);
        let is_bool = matches!(&f.sig.output, syn::ReturnType::Type(_, t) if matches!(&**t, Type::Path(p) if p.path.is_ident("bool")));
        let marked = body.contains(MARK);
        if !marked {
            continue;
        }
        match f.block.stmts.as_slice() {
            [] if unit => out.push((name, FnFact::NoOp)),
            [Stmt::Expr(Expr::Return(r), _)] if unit && r.expr.is_none() => out.push((name, FnFact::NoOp)),
            [one] if is_bool => {
                let e: &Expr = match one {
                    Stmt::Expr(Expr::Return(syn::ExprReturn { expr: Some(e), .. }), _) => e,
                    Stmt::Expr(e, None) => e,
                    _ => continue,
                };
                if let Expr::Lit(l) = e {
                    if let syn::Lit::Bool(b) = &l.lit {
                        if src[e.span().byte_range().end..].starts_with(MARK) {
                            out.push((name, FnFact::Const(b.value)));
                        }
                    }
                }
            }
            _ => {}
        }
    }
    out
}

/// The edits of a pass that hold no other edit of it, last first, so they can be applied as they come.
pub fn innermost(mut edits: Vec<Edit>) -> Vec<Edit> {
    edits.sort_by(|a, b| a.start.cmp(&b.start).then(b.end.cmp(&a.end)));
    edits.dedup_by(|a, b| a.start == b.start && a.end == b.end);
    let mut keep: Vec<Edit> = Vec::new();
    for i in 0..edits.len() {
        let e = &edits[i];
        let holds_other = edits.iter().enumerate().any(|(j, o)| j != i && o.start >= e.start && o.end <= e.end && !(o.start == e.start && o.end == e.end));
        if holds_other {
            continue;
        }
        if let Some(last) = keep.last() {
            if e.start < last.end {
                continue;
            }
        }
        keep.push(e.clone());
    }
    keep.reverse();
    keep
}

pub fn apply(src: &str, edits: &[Edit]) -> String {
    let mut s = src.to_string();
    for e in edits {
        s.replace_range(e.start..e.end, &e.text);
    }
    s
}
