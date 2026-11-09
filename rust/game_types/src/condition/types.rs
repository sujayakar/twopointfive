//! The types of `condition`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, Blank, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// HOW A MAN IS: what a blast, a flash, a blow and fire have left on him, whoever he is. A clock that is past is nothing
#[derive(Clone, Copy, Debug, ToBytes, Blank)]
pub struct Cond {
    /// out of it: dead, knocked out, or a player out of hits
    pub down: bool,
    /// a blast reached him and left him up (below 0: none did)
    #[is(-1.0)] pub hurt_at: f64,
    /// blind until, and where the flash was
    #[is(-1.0)] pub blind_until: f64,
    pub blind_from: Option<V3>,
    /// folded until: rooted, gun down, no round
    #[is(-1.0)] pub fold_until: f64,
    /// the fire on him
    pub sup: crate::suppression::Sup,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Condition {
    pub shaken: bool,
    pub blind: bool,
    pub folded: bool,
    pub hurt: bool,
    pub down: bool,
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct ConditionStats {
    pub hurt: i32,
}
