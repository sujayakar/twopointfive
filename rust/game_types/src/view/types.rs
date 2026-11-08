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

/// the colour a mark over a man is drawn in
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum HeadTone {
    Calm,
    /// out cold, and breathing
    Ko,
    Dead,
    Alert,
    Sus,
}

/// a man on the floor, as the HUD marks him
#[derive(Clone, Debug)]
pub struct ManMark {
    /// his character's id
    pub man: f64,
    /// the glyphs over his head ('': no mark)
    pub text: String,
    pub tone: crate::view::HeadTone,
    /// a civilian: the small mark of their own
    pub civ: bool,
    /// the fog hides him from this seat's team (a picture with the fog lifted is a viewer's own, and shows him all the same)
    pub hidden: bool,
    /// the AI's own words after the mark, with the debug labels on ('': off)
    pub debug: String,
}
impl Default for ManMark { fn default() -> Self { ManMark { man: -1.0, text: String::new(), tone: HeadTone::Calm, civ: false, hidden: false, debug: String::new() } } }

/// where an enemy who has gone out of the team's sight was last seen
#[derive(Clone, Copy, Debug, Default)]
pub struct SeenMark {
    /// his character's id
    pub man: f64,
    pub at: V3,
    /// seconds since
    pub age: f64,
    /// seconds the mark lasts
    pub life: f64,
}

/// a stuck camera of the seat's own, while the fog is live
#[derive(Clone, Copy, Debug, Default)]
pub struct CamMark {
    pub at: V3,
    /// it is the one sending a picture
    pub sends: bool,
}

/// a defender's live grenade the team has been cued on (the cue is one a grenade, and made about the first man: in a game of two the second is told of the same)
#[derive(Clone, Copy, Debug, Default)]
pub struct FragMark {
    pub at: V3,
    /// seconds left on its fuse
    pub left: f64,
}

/// WHAT THE HUD MARKS ON THE FLOOR AND SAYS OF THE SEAT'S MAN
#[derive(Clone, Debug, Default)]
pub struct MarksView {
    /// every man on the floor the HUD may mark, hidden or not: guards first, then the civilians shown
    pub men: Vec<crate::view::ManMark>,
    pub seen: Vec<crate::view::SeenMark>,
    pub cams: Vec<crate::view::CamMark>,
    pub frags: Vec<crate::view::FragMark>,
    /// how blind he is: 1 for the body of a dazzle, easing to 0
    pub blind: f64,
    /// the light meter, 0 to 1
    pub light: f64,
    /// the light on him as it was measured
    pub irradiance: f64,
    /// the noise he is making, 0 to 1 and over
    pub noise: f64,
    /// the man he is stalking (his character's id)
    pub stalk: Option<f64>,
    /// he waits at a corner for a man to come round it
    pub stalk_waits: bool,
}

/// what the auto-aim has snapped to
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum RingOn {
    /// nothing: the bare point
    Free,
    /// a light or a fixture
    Light,
    /// a man
    Guard,
}

/// the aim model's cone at the ring
#[derive(Clone, Copy, Debug, Default)]
pub struct AimCone {
    /// its half width, radians
    pub spread: f64,
    /// how settled his track is, 0 to 1
    pub track: f64,
    /// metres from his hand to the ring
    pub range: f64,
    /// its terms, for the debugger's row: his hands, how far the track is from settled, the man crossing, his feet
    pub hands: f64,
    pub settle: f64,
    pub crossing: f64,
    pub feet: f64,
}

/// the gun's ring, when nothing Space acts on holds the reticle
#[derive(Clone, Copy, Debug)]
pub struct AimRing {
    /// where it stands: where the gun points while it swings, else what it snapped to, else the cursor's point
    pub at: V3,
    pub on: crate::view::RingOn,
    /// the circle is still swinging on to the cursor
    pub trail: bool,
    /// his gun cannot get a round out from here
    pub shut: bool,
    /// what it will settle on, marked while it swings (bare floor needs none)
    pub wish: Option<V3>,
    /// (None: the aim model is not up)
    pub cone: Option<crate::view::AimCone>,
}
impl Default for AimRing { fn default() -> Self { AimRing { at: [0.0; 3], on: RingOn::Free, trail: false, shut: false, wish: None, cone: None } } }

/// the gun's own small circle, while the reticle's ring is on a thing Space acts on: a round goes there
#[derive(Clone, Copy, Debug, Default)]
pub struct GunDot {
    pub at: V3,
    pub shut: bool,
}

/// what a thing in the air would reach, flat on the floor round a point
#[derive(Clone, Copy, Debug, Default)]
pub struct ThrowReachView {
    pub at: V3,
    pub kill: f64,
    pub hurt: f64,
    pub stun: f64,
}

/// the arc he is aiming
#[derive(Clone, Debug, Default)]
pub struct ThrowView {
    pub points: Vec<V3>,
    pub land: V3,
    /// a 40 mm arc too short to arm
    pub short: bool,
    /// where it comes to rest (None: the launcher's round bursts or sticks where it lands)
    pub rest: Option<crate::player::ThrowRest>,
    pub reach: Option<crate::view::ThrowReachView>,
}

/// WHAT THE HUD DRAWS OF THE SEAT'S AIM
#[derive(Clone, Debug, Default)]
pub struct AimView {
    pub ring: Option<crate::view::AimRing>,
    pub dot: Option<crate::view::GunDot>,
    /// the word at the reticle: the 40 mm's arc is too short to arm
    pub too_close: bool,
    pub throw: Option<crate::view::ThrowView>,
}

/// what a man wears and carries that the drawer draws
#[derive(Clone, Copy, Debug)]
pub struct WornView {
    /// his character's id
    pub man: f64,
    pub goggles: bool,
    pub big: Option<crate::bigkit::BigCarried>,
}
impl Default for WornView { fn default() -> Self { WornView { man: -1.0, goggles: false, big: None } } }

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
    /// what the HUD marks
    pub marks: crate::view::MarksView,
    /// what the HUD draws of his aim
    pub aim: crate::view::AimView,
    /// the men who wear or carry something the drawer draws: nobody else is listed
    pub worn: Vec<crate::view::WornView>,
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
