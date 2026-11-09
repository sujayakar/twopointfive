//! The types of `verdict`.
#![allow(unused_imports)]
use crate::{Blank, bytes::ToBytes, Words, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, ToBytes, Blank)]
pub struct Verdict {
    pub run: Option<u32>,
    #[is(f64::NAN)] pub t0: f64,
    pub origin: V3,
    pub order0: Option<u32>,
    pub origins: Vec<V3>,
    pub fired0: f64,
    pub resumed: bool,
    pub cheat: bool,
    #[is(-1.0)] pub found_at: f64,
    #[is(-1.0)] pub dark_at: f64,
    pub dark_when_found: bool,
    pub through: bool,
    pub wall_serial: f64,
    pub holes: Vec<V3>,
    pub tracks: JsMap<crate::Num, crate::verdict::Track>,
    pub foes: JsSet<GuardId>,
    pub fell: JsSet<GuardId>,
    pub fell_at: Vec<f64>,
    pub res: Option<crate::verdict::RunVerdict>,
    #[is(-1.0)] pub card_at: f64,
    pub kept: Option<crate::verdict::KeptNote>,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Track {
    pub id: f64,
    pub name: String,
    pub tone: Option<crate::host_findme::OutlineHue>,
    pub human: bool,
    pub gd: Option<GuardId>,
    pub state: crate::verdict::ManState,
    pub down_at: f64,
    pub down_room: String,
    pub down_by: String,
    pub down_pos: Option<[f64; 2]>,
    pub drops: Vec<f64>,
    pub path: Vec<f64>,
    pub every: f64,
    pub next: f64,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default, Words)]
pub enum ManState {
    #[default] Dead,
    Out,
    Up,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct RunVerdict {
    pub kind: Option<crate::verdict::RunVerdictKind>,
    pub won: bool,
    pub word: String,
    pub why: String,
    pub secs: f64,
    pub approach: Option<crate::verdict::Approach>,
    pub void: bool,
    pub clean: bool,
    pub men: Vec<crate::verdict::VerdictMan>,
    pub lost: f64,
    pub of: f64,
    pub foes: crate::verdict::RunVerdictFoes,
    pub hostages: Option<crate::verdict::RunVerdictHostages>,
    pub start: [f64; 2],
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Words)]
pub enum RunVerdictKind {
    Nightbreach,
    Roomclear,
    Rescue,
    Quiet,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Words)]
pub enum Approach {
    Doors,
    Quiet,
    Walls,
    Dark,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct VerdictMan {
    pub id: f64,
    pub name: String,
    pub tone: Option<crate::host_findme::OutlineHue>,
    pub human: bool,
    pub state: crate::verdict::ManState,
    pub down_at: f64,
    pub down_room: String,
    pub down_by: String,
    pub down_pos: Option<[f64; 2]>,
    pub drops: Vec<f64>,
    pub path: Vec<f64>,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct RunVerdictFoes {
    pub down: f64,
    pub of: f64,
    pub fell: Vec<f64>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct RunVerdictHostages {
    pub alive: f64,
    pub of: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct KeptNote {
}

