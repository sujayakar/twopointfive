//! The types of `view`: what a machine is shown of the game beside what it draws (rust/game_rules/src/view).
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// whose words the mission's line is
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum MissionOf {
    /// the stealth mission's own stages
    Stages,
    /// a level's program while it runs
    Script,
    Lesson,
    Defuse,
    Snatch,
    Roomclear,
    Rescue,
    Nightbreach,
}

/// the mission's one line, and the place its marker stands on
#[derive(Clone, Debug)]
pub struct MissionView {
    pub of: crate::view::MissionOf,
    pub text: String,
    pub pos: Option<V3>,
}
impl Default for MissionView { fn default() -> Self { MissionView { of: MissionOf::Stages, text: String::new(), pos: None } } }

/// WHAT A SEAT'S HUD IS TOLD OF THE GAME AS A TICK LEFT IT: a section for each kind of thing a HUD words. Nothing in it is kept by the game.
#[derive(Clone, Debug, Default)]
pub struct TickView {
    /// (None: a show runs)
    pub mission: Option<crate::view::MissionView>,
}

/// one row of the card's timeline, in words
#[derive(Clone, Debug)]
pub struct DebriefRow {
    pub t: f64,
    pub kind: crate::chronicle::ChronKind,
    pub room: String,
    pub pos: V3,
    pub text: String,
    /// the floor's alarm going down: such a row carries no pin
    pub down: bool,
}
impl Default for DebriefRow { fn default() -> Self { DebriefRow { t: 0.0, kind: crate::chronicle::ChronKind::Objective, room: String::new(), pos: [0.0; 3], text: String::new(), down: false } } }

/// a room of the card's floor map, by the name it is drawn under
#[derive(Clone, Debug, Default)]
pub struct DebriefRoom {
    pub name: String,
    pub label: String,
    pub x0: f64,
    pub x1: f64,
    pub z0: f64,
    pub z1: f64,
}

/// WHAT THE END CARD SHOWS OF A RUN: asked once, as the card opens. Large, so it is no section of `TickView`.
#[derive(Clone, Debug, Default)]
pub struct DebriefView {
    pub debrief: crate::chronicle::Debrief,
    pub rows: Vec<crate::view::DebriefRow>,
    /// rows the timeline's caps refused
    pub dropped: f64,
    /// where the seat's man went: x, z, t a point
    pub path: Vec<f32>,
    pub rooms: Vec<crate::view::DebriefRoom>,
    /// who of ours fell and how (None: somebody still plays, or nobody of ours went down)
    pub recap: Option<crate::deathrecap::DeathRecap>,
}
