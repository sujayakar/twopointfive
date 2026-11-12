//! The types of `ui_missions`.
use crate::Words;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Words)]
pub enum Lean {
    Quiet,
    Loud,
    Either,
}
