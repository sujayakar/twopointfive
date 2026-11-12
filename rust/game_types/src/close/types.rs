//! The types of `close`.
use crate::{Blank, Words, bytes::ToBytes, ids::*, js::*, V3};
use alloc::string::String;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum CloseKind {
    #[default] Behind,
    Side,
    Front,
    Seat,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default, Words)]
pub enum CloseId {
    #[default] Behind,
    Front,
    Left,
    Right,
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
pub enum CloseRmb {
    Press,
    Tap,
    Hold,
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
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct CloseMod {
    pub state: JsMap<PlayerId, crate::close::State>,
    pub seated: Option<JsMap<GuardId, crate::close::Seat>>,
}

pub const DEGR: f64 = 0.017453292519943295;

pub const BEHIND_COS: f64 = -0.25;

pub fn sit_clip() -> crate::close::SITCLIP { crate::close::SITCLIP { clip: String::from("Sitting_Idle_Loop"), loop_: true, rate: 1.0, in_: 0.7, out: 0.8 } }
