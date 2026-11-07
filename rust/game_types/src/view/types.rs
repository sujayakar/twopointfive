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

/// what kind of thing an order would be about
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum UnderKind {
    Floor,
    Door,
    /// a man of the squad's own, or the squad itself (no man)
    None,
    Foe,
}
/// a wall a charge might go on, by its place in the level's boxes
#[derive(Clone, Debug, Default)]
pub struct UnderWall {
    pub index: f64,
    pub along_x: bool,
    pub u: f64,
    pub side: f64,
    pub ok: bool,
    pub why: String,
    pub outer: bool,
}
/// what a point or the cursor is on, as an order would be about it
#[derive(Clone, Debug)]
pub struct Under {
    pub kind: crate::view::UnderKind,
    pub at: V3,
    /// a door, by its name, and which face of it
    pub door: Option<String>,
    pub side: f64,
    /// a man, by his character's id (−1: none)
    pub man: f64,
    pub wall: Option<crate::view::UnderWall>,
    pub thing: Option<crate::workthings::WorkThing>,
}
impl Default for Under { fn default() -> Self { Under { kind: UnderKind::None, at: [0.0; 3], door: None, side: 0.0, man: -1.0, wall: None, thing: None } } }

/// WHAT A SEAT'S HUD IS TOLD OF WHAT ITS CURSOR IS ON, as the frame is drawn
#[derive(Clone, Debug, Default)]
pub struct CursorView {
    /// (None: no squad answers to this seat)
    pub under: Option<crate::view::Under>,
}

/// WHAT A HOST KNOWS THAT THE GAME HAS NOT SEEN YET: where the cursor is, and through which camera, as the frame is drawn (the game's own rows are of its last
/// step). Nothing else: no key, no click, no clock of the machine.
#[derive(Clone, Debug, Default)]
pub struct HostNow {
    /// the cursor and the picture it is in, in the page's pixels
    pub mouse: [f64; 2],
    pub client: [f64; 2],
    pub view_proj: crate::v3::m4::Mat4,
    pub inv_view_proj: crate::v3::m4::Mat4,
}
