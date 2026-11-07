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
    pub module_fns: &'a HashMap<String, HashSet<String>>,
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
    matches!(s, Stmt::Expr(Expr::Return(_) | Expr::Break(_) | Expr::Continue(_), _))
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
    "is_some", "is_none", "len", "is_empty", "abs", "min", "max", "sqrt", "clone", "copied", "cloned", "unwrap_or", "as_ref", "as_str", "contains", "get", "first", "last",
    "floor", "ceil", "round", "signum", "to_bits", "is_finite", "is_nan", "powi", "sin", "cos", "atan2", "hypot", "eq", "ne", "starts_with", "ends_with", "is_ok", "is_err",
    "contains_key", "has", "trunc", "clamp", "to_string", "as_deref", "unwrap_or_default", "is_some_and", "is_none_or",
];

const PARSED_MACROS: &[&str] = &["vec", "format", "assert", "assert_eq", "assert_ne", "debug_assert", "debug_assert_eq", "debug_assert_ne", "println", "eprintln", "print", "eprint", "panic", "write", "writeln", "matches", "unreachable"];

impl<'a> Pass<'a> {
    pub fn new(src: &'a str, file: &'a str, module: &'a str, rows: &'a Rows, take: &'a HashSet<String>, fns: &'a FnTable, module_fns: &'a HashMap<String, HashSet<String>>, pure_fns: &'a HashSet<(String, String)>, scan_only: bool) -> Self {
        let mut line_starts = vec![0];
        for (i, b) in src.bytes().enumerate() {
            if b == b'\n' {
                line_starts.push(i + 1);
            }
        }
        Pass { src, file, module, rows, take, fns, module_fns, pure_fns, scan_only, edits: Vec::new(), report: Report::default(), scopes: Vec::new(), impl_ty: None, paren_inner: HashMap::new(), delim: HashSet::new(), receiver: HashSet::new(), claimed: HashSet::new(), fn_bodies: HashSet::new(), logic_operand: HashSet::new(), line_starts }
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
        let mut is_mut = false;
        loop {
            match inner {
                Type::Reference(r) => {
                    is_mut |= r.mutability.is_some();
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
                            return Bind::Alias(KPath { ty: t.clone(), path: path.clone() }, is_mut);
                        }
                    }
                }
                if let Some(path) = self.rows.type_path.get(&n) {
                    return Bind::Alias(KPath { ty: n, path: path.clone() }, is_mut);
                }
            }
        }
        Bind::Other
    }

    fn bind_sig(&mut self, sig: &syn::Signature) {
        for a in &sig.inputs {
            match a {
                FnArg::Receiver(r) => {
                    let k = match &self.impl_ty {
                        Some(t) if t == "Game" => Bind::Game,
                        Some(t) => match self.rows.type_path.get(t) {
                            Some(p) => Bind::Alias(KPath { ty: t.clone(), path: p.clone() }, r.mutability.is_some()),
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
                    _ => None,
                }
            }
        }
    }
    fn is_table(&self, k: &KPath) -> bool {
        self.rows.tree.contains_key(&k.ty)
    }
    /// A `.k.` chain off a root this pass does not know for the game, that would be a settled row off the game.
    fn unknown_root_row(&self, e: &Expr) -> Option<String> {
        let mut names: Vec<String> = Vec::new();
        let mut cur = e;
        loop {
            match cur {
                Expr::Field(f) => {
                    let Member::Named(m) = &f.member else { return None };
                    let m = m.to_string();
                    if m == "k" {
                        break;
                    }
                    names.push(m);
                    cur = &f.base;
                }
                Expr::Paren(p) => cur = &p.expr,
                _ => return None,
            }
        }
        names.reverse();
        let mut ty = "Knobs".to_string();
        for n in &names {
            let fs = self.rows.tree.get(&ty)?;
            ty = fs.iter().find(|(f, _)| f == n)?.1.clone();
        }
        let p = names.join(".");
        if self.take.contains(&p) {
            Some(p)
        } else {
            None
        }
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

    /// The function of this crate a call names: (module, function).
    fn call_target(&self, c: &syn::ExprCall) -> Option<(String, String)> {
        let Expr::Path(p) = &*c.func else { return None };
        if p.qself.is_some() {
            return None;
        }
        let segs: Vec<String> = p.path.segments.iter().map(|s| s.ident.to_string()).collect();
        let name = segs.last()?.clone();
        let module = if segs.len() == 1 {
            if self.lookup(&name).is_some() {
                return None;
            }
            if self.module_fns.get(self.module).map(|s| s.contains(&name)).unwrap_or(false) {
                self.module.to_string()
            } else {
                return None;
            }
        } else {
            let m = &segs[segs.len() - 2];
            if m == "super" || m == "self" {
                self.module.to_string()
            } else {
                m.clone()
            }
        };
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
            Expr::Reference(r) => self.pure(&r.expr),
            Expr::Tuple(t) => t.elems.iter().all(|x| self.pure(x)),
            Expr::Array(a) => a.elems.iter().all(|x| self.pure(x)),
            Expr::MethodCall(m) => PURE_METHODS.contains(&m.method.to_string().as_str()) && self.pure(&m.receiver) && m.args.iter().all(|x| self.pure(x)),
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
                    _ => {}
                }
            }
            false
        }
        walk(ts, names)
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
                let clash = items || self.mentions(rest.clone(), &names);
                if clash {
                    let t = self.text(self.r(b)).to_string();
                    self.edit(stmt_r, t, "if: the side kept, in its braces");
                    return;
                }
                let tail_stays = is_last && !semi;
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
        // not read as code: every name in it counts as used, and a row read in it is left by name
        let at = self.r(mac).start;
        let toks: Vec<TokenTree> = flat(mac.tokens.clone());
        let mut i = 0;
        while i < toks.len() {
            if let TokenTree::Ident(id) = &toks[i] {
                let n = id.to_string();
                let mut chain = vec![n.clone()];
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
                let base: Option<KPath> = match self.lookup(&n) {
                    Some(Binding { kind: Bind::Game, .. }) if chain.len() > 1 && chain[1] == "k" => Some(KPath { ty: "Knobs".into(), path: vec![] }),
                    Some(Binding { kind: Bind::Alias(k, _), .. }) => Some(k.clone()),
                    _ => None,
                };
                let skip = if matches!(self.lookup(&n), Some(Binding { kind: Bind::Game, .. })) { 2 } else { 1 };
                if let Some(mut kp) = base {
                    for f in chain.iter().skip(skip) {
                        let Some(fs) = self.rows.tree.get(&kp.ty) else { break };
                        let Some((_, t)) = fs.iter().find(|(x, _)| x == f) else { break };
                        kp.path.push(f.clone());
                        kp.ty = t.clone();
                    }
                    let p = kp.path.join(".");
                    if self.take.contains(&p) {
                        self.left(at, &format!("{p} read inside `{name}!`, which is not read as code"));
                    }
                }
                self.used(&n);
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
        *self.report.reads.entry(p).or_default() += 1;
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
    fn visit_item_fn(&mut self, f: &'ast syn::ItemFn) {
        let saved = std::mem::take(&mut self.scopes);
        self.push();
        self.bind_sig(&f.sig);
        if matches!(f.sig.output, syn::ReturnType::Default) {
            self.fn_bodies.insert(self.key(&*f.block));
        }
        self.visit_block(&f.block);
        self.pop();
        self.scopes = saved;
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
        for (i, s) in b.stmts.iter().enumerate() {
            match s {
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
                    if self.pure(&f.expr) {
                        self.delete_stmt(b, i);
                    } else {
                        self.left(self.r(f).start, "a loop over nothing now, what it walks not plainly free of effects");
                    }
                }
                Stmt::Expr(Expr::Block(x), _) if x.label.is_none() && self.emptied(&x.block) => self.delete_stmt(b, i),
                Stmt::Expr(Expr::While(w), _) if self.emptied(&w.body) => self.left(self.r(w).start, "a `while` with nothing in it now"),
                Stmt::Expr(Expr::Assign(a), Some(_)) => {
                    // a write of the row's own value off a root this pass cannot type (a test's own game): a dead store
                    if self.resolve(&a.left).is_none() {
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
        if let Some(init) = &l.init {
            self.delim.insert(self.key(&*init.expr));
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
                            if !*is_mut {
                                kind = Bind::Alias(kp, by_mut);
                                decl = if by_mut { None } else { Some(self.r(l)) };
                                alias_init = true;
                            }
                        }
                    } else if !*is_mut {
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
        match simple {
            Some((n, _)) => self.bind(n, kind, decl),
            None => self.bind_pat(&l.pat),
        }
    }

    fn visit_expr_closure(&mut self, c: &'ast syn::ExprClosure) {
        self.push();
        for p in &c.inputs {
            self.bind_pat(p);
        }
        self.delim.insert(self.key(&*c.body));
        self.visit_expr(&c.body);
        self.pop();
    }
    fn visit_arm(&mut self, a: &'ast syn::Arm) {
        self.push();
        self.bind_pat(&a.pat);
        if let Some((_, g)) = &a.guard {
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
                        if self.row_read(e, &kp) {
                            return;
                        }
                    }
                    // a table, or a row the fold does not take: only its names are walked
                    let mut cur = e;
                    while let Expr::Field(f) = cur {
                        cur = &f.base;
                    }
                    if let Some(n) = single_ident(cur) {
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
                if self.knob_write(&r.expr, "borrowed `&mut`") {
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
                if assign && self.knob_write(&b.left, "assigned") {
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
                self.delim.insert(self.key(&*m.expr));
                if self.marked_bool(&m.expr).is_some() {
                    self.left(self.r(e).start, "a `match` on a settled yes or no");
                }
            }
            Expr::If(ifx) => {
                // an `if` standing as a value
                if !self.claimed.contains(&self.key(ifx)) {
                    if let Some(on) = self.marked_bool(&ifx.cond) {
                        let whole = self.r(e);
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
                        self.edit(whole, text, "if as a value: the side kept");
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
pub fn pure_fns_of(file: &syn::File, pass: &Pass) -> Vec<String> {
    let mut out = Vec::new();
    for it in &file.items {
        let syn::Item::Fn(f) = it else { continue };
        let n = f.block.stmts.len();
        if n == 0 {
            continue;
        }
        let ok = f.block.stmts.iter().enumerate().all(|(i, s)| match s {
            Stmt::Local(l) => i + 1 < n && matches!(&l.pat, Pat::Ident(p) if p.mutability.is_none() && p.by_ref.is_none()) && matches!(&l.init, Some(init) if init.diverge.is_none() && pass.pure(&init.expr)),
            Stmt::Expr(e, None) => i + 1 == n && pass.pure(e),
            _ => false,
        });
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
