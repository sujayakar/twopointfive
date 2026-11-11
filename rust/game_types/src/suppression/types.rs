//! The types of `suppression`.
#![allow(unused_imports)]
use crate::{Blank, Words, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, ToBytes, Blank)]
pub struct Sup {
    pub v: f64,
    pub kick: f64,
    pub src: Option<V3>,
    #[is(-1.0)] pub src_id: f64,
    #[is(-100.0)] pub src_t: f64,
    #[is(-100.0)] pub pinned_at: f64,
    pub band: f64,
    pub down_t: f64,
    pub latched: bool,
    pub pinned_t: f64,
    pub cover: Option<V3>,
    #[is(-100.0)] pub displaced_at: f64,
    pub displaces: f64,
    #[is(-1.0)] pub cover_until: f64,
    #[is(-100.0)] pub bark_t: f64,
    pub bark_was: Option<crate::suppression::SupMoment>,
    pub pending: Option<crate::suppression::SupMoment>,
    #[is(-100.0)] pub pending_t: f64,
    pub was_pinned: bool,
    /// getting down where he stands takes him out of the sight of everybody he knows of, and when that was last asked
    pub hides: bool,
    #[is(-100.0)] pub hides_t: f64,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Words)]
pub enum SupMoment {
    Flinch,
    Friendly,
    Pinned,
    Displace,
    Recover,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum HeadDown {
    Down,
    None,
    Displace,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct GuardFireMods {
    pub spread: f64,
    pub gap: f64,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct SuppressionMod {
    pub floor_line_t: Option<f64>,
    pub floor_bark_t: Option<f64>,
    pub last_line: JsMap<GuardId, String>,
}

