//! The types of `close`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum CloseKind {
    #[default] Behind,
    Side,
    Front,
    Seat,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum CloseId {
    #[default] Behind,
    Front,
    Left,
    Right,
}
impl CloseId {
    pub const fn as_str(self) -> &'static str {
        match self {
            CloseId::Behind => "behind",
            CloseId::Front => "front",
            CloseId::Left => "left",
            CloseId::Right => "right",
        }
    }
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct CloseWindow {
    pub id: crate::close::CloseId,
    pub kind: crate::close::CloseKind,
    pub mid: f64,
    pub half: f64,
    pub cos_half: f64,
    pub reach: f64,
    pub grab: bool,
    pub strike: bool,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct CloseOffer {
    pub guard: GuardId,
    pub win: Option<crate::close::CloseWindow>,
    pub dist: f64,
    pub in_reach: bool,
    pub clear: bool,
    pub grab: bool,
    pub strike: bool,
    pub can: bool,
    pub why: String,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct Seat {
    pub at: V3,
    pub face: f64,
    pub hold: bool,
    pub down: bool,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct State {
    pub t: f64,
    pub best: Option<crate::close::CloseOffer>,
    pub mode: Option<GuardId>,
    pub mode_d: f64,
    pub rmb_t: f64,
}

#[derive(Clone, Debug, Default)]
pub struct SITCLIP {
    pub clip: String,
    pub loop_: bool,
    pub rate: f64,
    pub in_: f64,
    pub out: f64,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum NoteWhat {
    Grab,
    Seat,
    Take,
    Unseat,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum CloseRmb {
    Press,
    Tap,
    Hold,
}

#[derive(Clone, Debug, Default)]
pub struct CloseRowsRowOver {
    pub kind: Option<crate::close::CloseRowsRowOverKind>,
    pub pos: Option<V3>,
    pub line1: Option<String>,
    pub line2: Option<String>,
    pub in_reach: Option<bool>,
    pub off: Option<bool>,
    pub progress: Option<f64>,
    pub target: Option<FixtureId>,
    pub door: Option<DoorId>,
    pub guard: Option<GuardId>,
    pub verb: Option<crate::game::Verb>,
    pub running: Option<bool>,
    pub close: Option<crate::close::CloseKind>,
    pub past: Option<bool>,
    pub framed: Option<bool>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum CloseRowsRowOverKind {
    Cam,
    Guard,
    Breaker,
    Door,
    Light,
    Body,
    Objective,
    Held,
    Wall,
    Switch,
    Charge,
    Cancel,
}
impl CloseRowsRowOverKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            CloseRowsRowOverKind::Cam => "cam",
            CloseRowsRowOverKind::Guard => "guard",
            CloseRowsRowOverKind::Breaker => "breaker",
            CloseRowsRowOverKind::Door => "door",
            CloseRowsRowOverKind::Light => "light",
            CloseRowsRowOverKind::Body => "body",
            CloseRowsRowOverKind::Objective => "objective",
            CloseRowsRowOverKind::Held => "held",
            CloseRowsRowOverKind::Wall => "wall",
            CloseRowsRowOverKind::Switch => "switch",
            CloseRowsRowOverKind::Charge => "charge",
            CloseRowsRowOverKind::Cancel => "cancel",
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct CloseCivGate {
    pub behind: bool,
    pub near: bool,
    pub kind: crate::close::CloseKind,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct CloseReady {
    pub kind: crate::character::ReadyKind,
    pub at: V3,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct CloseStation {
    pub pos: V3,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum CloseTookWhat {
    Grab,
    Take,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes)]
pub struct CloseMod {
    pub state: JsMap<PlayerId, crate::close::State>,
    pub seated: Option<JsMap<GuardId, crate::close::Seat>>,
}
impl CloseMod {
    pub fn blank() -> Self {
        CloseMod {
            state: JsMap::new(),
            seated: None,
        }
    }
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct CloseStats {
    pub takes: [i32; 4],
    pub grabs: [i32; 4],
    pub blows: [i32; 2],
    pub ready_frames: i32,
    pub mode_frames: i32,
    pub dark_passes: i32,
    pub seats: i32,
    pub unseats: i32,
}

pub const DEGR: f64 = 0.017453292519943295;

pub const BEHIND_COS: f64 = -0.25;

pub fn sit_clip() -> crate::close::SITCLIP { crate::close::SITCLIP { clip: String::from("Sitting_Idle_Loop"), loop_: true, rate: 1.0, in_: 0.7, out: 0.8 } }
