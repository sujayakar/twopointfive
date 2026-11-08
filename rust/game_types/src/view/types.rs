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

/// what a place in the kit's strip holds
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum KitThing {
    /// his hands, empty
    Hands,
    Pistol,
    Rifle,
    Smoke,
    Flash,
    Frag,
}

/// a place in the kit's strip, what the key brings
#[derive(Clone, Debug)]
pub struct KitSlot {
    pub thing: crate::view::KitThing,
    /// the code of the key that brings it
    pub key: String,
    /// a gun's short name, a can's word (empty hands have none)
    pub name: String,
    /// a gun's rounds ready, how many cans
    pub count: f64,
    /// this kit has none of it: the place is kept, so the keys after it keep their numbers
    pub absent: bool,
    /// a can's place with none left
    pub spent: bool,
    /// it is what is up: the box under the strip is about it
    pub up: bool,
    /// its key is a row of the setup that is open
    pub taken: bool,
}
impl Default for KitSlot { fn default() -> Self { KitSlot { thing: KitThing::Hands, key: String::new(), name: String::new(), count: 0.0, absent: false, spent: false, up: false, taken: false } } }

/// a position a thing on a gun can be set to, in the HUD's words
#[derive(Clone, Debug, Default)]
pub struct KitWord {
    pub text: String,
    /// it is the one that is up
    pub on: bool,
    /// a load he is out of
    pub out: bool,
}

/// the pistol's OCP
#[derive(Clone, Debug, Default)]
pub struct KitOcp {
    /// 1 full
    pub charge: f64,
    pub ready: bool,
}

/// the gun that is up, as its box shows it
#[derive(Clone, Debug, Default)]
pub struct KitGun {
    /// rounds in the magazine
    pub mag: f64,
    /// what the magazine holds
    pub capacity: f64,
    /// a round is in the chamber
    pub chamber: bool,
    /// rounds in the gun
    pub ready: f64,
    /// none: it will not fire
    pub dry: bool,
    pub reloading: bool,
    /// the selector's position, on a gun that has two
    pub selector: Option<String>,
    /// the load on the launcher
    pub load: Option<crate::view::KitWord>,
    /// the light is on
    pub light: bool,
    pub ocp: Option<crate::view::KitOcp>,
    /// the rounds in each spare magazine, fullest first
    pub spares: Vec<f64>,
    /// a gun fed a shell at a time: the shells he carries
    pub loose: Option<f64>,
}

/// a row of the setup that is open
#[derive(Clone, Debug, Default)]
pub struct KitSetupRow {
    /// the code of its key
    pub key: String,
    pub words: Vec<crate::view::KitWord>,
}

/// the setup of the gun in his hands, while it is open
#[derive(Clone, Debug, Default)]
pub struct KitSetup {
    pub rows: Vec<crate::view::KitSetupRow>,
    /// the key that backs out of it
    pub back: String,
}

/// the big slot: what is worked at a door, on the door's own key
#[derive(Clone, Debug, Default)]
pub struct KitBig {
    pub name: String,
    /// where it is worked
    pub place: String,
    /// a door in reach offers it now
    pub offered: bool,
}

/// WHAT A SEAT'S MAN CARRIES AND THE STATE HE IS IN, AS THE KIT'S COLUMN SHOWS IT
#[derive(Clone, Debug, Default)]
pub struct KitView {
    /// the strip, in its order (no places: a game no host steps)
    pub slots: Vec<crate::view::KitSlot>,
    /// the gun that is up
    pub gun: Option<crate::view::KitGun>,
    /// the setup, while it is open
    pub setup: Option<crate::view::KitSetup>,
    /// the codes of the keys the setup would take now
    pub takes: Vec<String>,
    pub big: Option<crate::view::KitBig>,
    pub hits_left: f64,
    pub hits_max: f64,
    pub down: bool,
    pub god: bool,
    /// whether the goggles are on (None: a lesson has not taught their key)
    pub night_vision: Option<bool>,
    /// the word for the keys he took off a man (None: he has none)
    pub key_tag: Option<String>,
}

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
    /// what he carries and the state he is in
    pub kit: crate::view::KitView,
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
