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

/// what a row of the radio strip is
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum RadioKind {
    /// a line on a net
    Net,
    /// a shout that reached an ear of the team's: his words alone
    Heard,
}

/// a row of the radio strip
#[derive(Clone, Debug)]
pub struct RadioRow {
    pub kind: crate::view::RadioKind,
    pub text: String,
    /// seconds since it was said
    pub age: f64,
}
impl Default for RadioRow { fn default() -> Self { RadioRow { kind: RadioKind::Net, text: String::new(), age: 0.0 } } }

/// how a line sounds
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum BubbleTone {
    Calm,
    /// a man who may find you out
    Sus,
    /// the red of a fight
    Alert,
    /// the seat's own man speaking
    Own,
}

/// where a line goes
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum BubblePlace {
    /// over him, and nowhere while that is off the screen
    Over,
    /// over him while that is on the screen, else at the edge of the view on his side
    OverOrEdge,
    /// at the edge of the view on his side, wherever he is
    Edge,
}

/// a line being said on the floor
#[derive(Clone, Debug)]
pub struct BubbleView {
    /// his character's id (None: a man who went down, called from `at`)
    pub man: Option<f64>,
    pub text: String,
    /// the words at the edge of the view, with whose they are
    pub edge: String,
    /// seconds since it was said
    pub age: f64,
    /// seconds it stands
    pub dur: f64,
    /// it went over the net
    pub radio: bool,
    pub tone: crate::view::BubbleTone,
    pub place: crate::view::BubblePlace,
    /// where he stands (or fell)
    pub at: V3,
}
impl Default for BubbleView { fn default() -> Self { BubbleView { man: None, text: String::new(), edge: String::new(), age: 0.0, dur: 0.0, radio: false, tone: BubbleTone::Calm, place: BubblePlace::Over, at: [0.0, 0.0, 0.0] } } }

/// WHAT A SEAT'S HUD IS TOLD OF THE GAME AS A TICK LEFT IT: a section for each kind of thing a HUD words. Nothing in it is kept by the game.
#[derive(Clone, Debug, Default)]
pub struct TickView {
    /// (None: a show runs)
    pub mission: Option<crate::view::MissionView>,
    /// the radio strip, oldest first
    pub radio: Vec<crate::view::RadioRow>,
    /// what is being said on the floor
    pub bubbles: Vec<crate::view::BubbleView>,
    /// the seat's own man (his character's id)
    pub me: f64,
    /// the room the seat's man stands in, by the level's own name for it
    pub place: String,
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
