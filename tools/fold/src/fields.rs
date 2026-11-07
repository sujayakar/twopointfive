//! Which field names a body still READS. A name is read wherever `.name` stands outside the left of an
//! assignment, and where a pattern takes it. By name alone: fields of different types that share a name are one.

use std::collections::HashMap;
use syn::visit::{self, Visit};
use syn::{BinOp, Expr, Member};

#[derive(Default)]
struct Reads {
    n: HashMap<String, i64>,
}

impl Reads {
    /// the left of an assignment: its last field is written, what leads to it is read
    fn target(&mut self, e: &Expr) {
        match e {
            Expr::Field(f) => self.visit_expr(&f.base),
            Expr::Paren(p) => self.target(&p.expr),
            _ => self.visit_expr(e),
        }
    }
}

impl<'ast> Visit<'ast> for Reads {
    fn visit_expr(&mut self, e: &'ast Expr) {
        match e {
            Expr::Assign(a) => {
                self.target(&a.left);
                self.visit_expr(&a.right);
                return;
            }
            Expr::Binary(b) if matches!(b.op, BinOp::AddAssign(_) | BinOp::SubAssign(_) | BinOp::MulAssign(_) | BinOp::DivAssign(_) | BinOp::RemAssign(_) | BinOp::BitXorAssign(_) | BinOp::BitAndAssign(_) | BinOp::BitOrAssign(_) | BinOp::ShlAssign(_) | BinOp::ShrAssign(_)) => {
                self.target(&b.left);
                self.visit_expr(&b.right);
                return;
            }
            Expr::Field(f) => {
                if let Member::Named(m) = &f.member {
                    *self.n.entry(m.to_string()).or_default() += 1;
                }
            }
            _ => {}
        }
        visit::visit_expr(self, e);
    }
    fn visit_field_pat(&mut self, f: &'ast syn::FieldPat) {
        if let Member::Named(m) = &f.member {
            *self.n.entry(m.to_string()).or_default() += 1;
        }
        visit::visit_field_pat(self, f);
    }
}

pub fn field_reads(src: &str) -> HashMap<String, i64> {
    let mut r = Reads::default();
    if let Ok(f) = syn::parse_file(src) {
        r.visit_file(&f);
    }
    r.n
}
