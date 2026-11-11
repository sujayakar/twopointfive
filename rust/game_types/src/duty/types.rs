//! The types of `duty`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, Blank, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// WHAT A MAN IS HELD TO, whoever gave it him: one record for every man of any side. A squad's order, a defender's post, a mission's mark and a night man's lockdown all write it
/// (duty.rs), and a rule that asks whether he may leave, where his gun is meant to be or where he may go reads it there.
#[derive(Clone, Copy, Debug, ToBytes, Blank)]
pub struct Duty {
    /// he does not leave where he is of his own accord · who last changed that, and when
    pub holds: bool,
    pub holds_by: Giver,
    /// … and he fights from it
    pub pinned: bool,
    pub pinned_by: Giver,
    /// he stays in his squad's rooms
    pub rooms: bool,
    /// what it has his gun on, this tick
    pub gun: Option<V3>,
}

impl Duty {
    /// nothing holds him or pins him: he goes where his fight takes him
    pub fn free(&self) -> bool { !self.holds && !self.pinned }
}

/// The module that last changed a word of it, and when: read by nothing in the tick, only by what says why he does what he does
#[derive(Clone, Copy, PartialEq, Debug, Default, ToBytes)]
pub struct Giver {
    pub by: &'static str,
    pub since: f64,
}
