//! The types of `verdict`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, ToBytes)]
pub struct Verdict {
    pub run: Option<u32>,
    pub t0: f64,
    pub origin: V3,
    pub order0: Option<u32>,
    pub origins: Vec<V3>,
    pub fired0: f64,
    pub resumed: bool,
    pub cheat: bool,
    pub found_at: f64,
    pub dark_at: f64,
    pub dark_when_found: bool,
    pub through: bool,
    pub wall_serial: f64,
    pub holes: Vec<V3>,
    pub tracks: JsMap<crate::Num, crate::verdict::Track>,
    pub foes: JsSet<GuardId>,
    pub fell: JsSet<GuardId>,
    pub fell_at: Vec<f64>,
    pub res: Option<crate::verdict::RunVerdict>,
    pub card_at: f64,
    pub kept: Option<crate::verdict::KeptNote>,
}
impl Verdict {
    /// what a new one holds
    pub fn blank() -> Self {
        Verdict {
            run: None,
            t0: f64::NAN,
            origin: [0.0, 0.0, 0.0],
            order0: None,
            origins: Vec::new(),
            fired0: 0.0,
            resumed: false,
            cheat: false,
            found_at: -1.0,
            dark_at: -1.0,
            dark_when_found: false,
            through: false,
            wall_serial: 0.0,
            holes: Vec::new(),
            tracks: JsMap::new(),
            foes: JsSet::new(),
            fell: JsSet::new(),
            fell_at: Vec::new(),
            res: None,
            card_at: -1.0,
            kept: None,
        }
    }
}
impl Default for Verdict { fn default() -> Self { Self::blank() } }

#[derive(Clone, Copy, Debug, Default)]
pub struct VerdictRun {
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

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum ManState {
    #[default] Dead,
    Out,
    Up,
}
impl ManState {
    pub const fn as_str(self) -> &'static str {
        match self {
            ManState::Dead => "dead",
            ManState::Out => "out",
            ManState::Up => "up",
        }
    }
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

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes)]
pub enum RunVerdictKind {
    Nightbreach,
    Roomclear,
    Rescue,
    Quiet,
}
impl RunVerdictKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            RunVerdictKind::Nightbreach => "nightbreach",
            RunVerdictKind::Roomclear => "roomclear",
            RunVerdictKind::Rescue => "rescue",
            RunVerdictKind::Quiet => "quiet",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes)]
pub enum Approach {
    Doors,
    Quiet,
    Walls,
    Dark,
}
impl Approach {
    pub const fn as_str(self) -> &'static str {
        match self {
            Approach::Doors => "doors",
            Approach::Quiet => "quiet",
            Approach::Walls => "walls",
            Approach::Dark => "dark",
        }
    }
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
    pub streak: f64,
    pub fresh: bool,
    pub gold: bool,
    pub ways: crate::verdict::KeptNoteWays,
    pub lesson: Option<bool>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct KeptNoteWays {
    pub doors: Option<crate::verdict::KeptNoteWaysDoors>,
    pub quiet: Option<crate::verdict::KeptNoteWaysDoors>,
    pub walls: Option<crate::verdict::KeptNoteWaysDoors>,
    pub dark: Option<crate::verdict::KeptNoteWaysDoors>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct KeptNoteWaysDoors {
    pub first: bool,
    pub clean: bool,
}

#[derive(Clone, Debug, Default)]
pub struct VerdictOnlyMateH {
    pub killer_id: f64,
    pub down_by: Option<String>,
    pub down_kind: Option<String>,
}

#[derive(Clone, Debug, Default)]
pub struct VerdictEnded {
    pub won: bool,
    pub word: String,
    pub why: String,
}

#[derive(Clone, Debug, Default)]
pub struct VerdictCloseEnd {
    pub won: bool,
    pub word: String,
    pub why: String,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum VerdictApproachKind {
    Nightbreach,
    Roomclear,
    Rescue,
    Quiet,
}
impl VerdictApproachKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            VerdictApproachKind::Nightbreach => "nightbreach",
            VerdictApproachKind::Roomclear => "roomclear",
            VerdictApproachKind::Rescue => "rescue",
            VerdictApproachKind::Quiet => "quiet",
        }
    }
}

/// (src/game/verdict.ts:97 PATH_CAP)
pub const PATH_CAP: f64 = 1500.0;

/// (src/game/verdict.ts:97 PATH_SECS)
pub const PATH_SECS: f64 = 0.5;

/// (src/game/verdict.ts:97 HOLE_M)
pub const HOLE_M: f64 = 0.9;
