//! The types of `view`: what a machine is shown of the game beside what it draws (rust/game_rules/src/tick/view).
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// whose words the mission's line is
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes)]
pub enum MissionOf {
    /// the stealth mission's own stages
    Stages,
    /// a level's program while it runs
    Script,
    Lesson,
    Roomclear,
    Rescue,
    Nightbreach,
}

/// the mission's one line, and the place its marker stands on
#[derive(Clone, Debug, ToBytes)]
pub struct MissionView {
    pub of: crate::view::MissionOf,
    pub text: String,
    pub pos: Option<V3>,
}
impl Default for MissionView { fn default() -> Self { MissionView { of: MissionOf::Stages, text: String::new(), pos: None } } }

/// what a row of the radio strip is
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes)]
pub enum RadioKind {
    /// a line on a net
    Net,
    /// a shout that reached an ear of the team's: his words alone
    Heard,
}

/// a row of the radio strip
#[derive(Clone, Debug, ToBytes)]
pub struct RadioRow {
    pub kind: crate::view::RadioKind,
    pub text: String,
    /// seconds since it was said
    pub age: f64,
}
impl Default for RadioRow { fn default() -> Self { RadioRow { kind: RadioKind::Net, text: String::new(), age: 0.0 } } }

/// how a line sounds
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes)]
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
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes)]
pub enum BubblePlace {
    /// over him, and nowhere while that is off the screen
    Over,
    /// over him while that is on the screen, else at the edge of the view on his side
    OverOrEdge,
    /// at the edge of the view on his side, wherever he is
    Edge,
}

/// a line being said on the floor
#[derive(Clone, Debug, ToBytes)]
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
/// a tutorial mission's one big line
#[derive(Clone, Debug, Default, ToBytes)]
pub struct RailPrompt {
    pub line: String,
    /// which step's pip is lit (from 1), of how many
    pub n: u32,
    pub of: u32,
    /// the few words under the line after a tap the step refused
    pub note: Option<String>,
    /// it asks for nothing: what the men are doing, or a step that waits on the level's script. No mark to press on
    pub watch: bool,
}

/// a tutorial step's card: why, where the line says what
#[derive(Clone, Debug, Default, ToBytes)]
pub struct RailCard {
    pub title: Option<String>,
    pub text: String,
}

/// what a tutorial mission's frame lets the player do and tells him, for the step that is up
#[derive(Clone, Debug, Default, ToBytes)]
pub struct RailsView {
    /// (None: the run is over)
    pub prompt: Option<crate::view::RailPrompt>,
    pub card: Option<crate::view::RailCard>,
    /// the step's mark is an arrow, pointing here
    pub arrow: Option<V3>,
    /// the player's own men on the floor, by their characters' ids: the big line keeps clear of them
    pub line_men: Vec<f64>,
    /// the actions that work now (rails.ts ACTS). An action not here is locked, and the HUD names no key of it
    pub open: Vec<String>,
    /// the verbs the interaction key offers rows of. (None: every row)
    pub rows: Option<Vec<String>>,
    /// the only orders on the squad's menu. (None: every order)
    pub orders: Option<Vec<String>>,
    /// a step has taken the held click off him: nothing says he can stalk a man
    pub stalk_shut: bool,
    /// the step teaches the squad's key held: the key's tap is not offered
    pub teaches_hold: bool,
    /// no teammate shows an aiming laser on this mission
    pub no_lasers: bool,
}

/// the lesson that is live on the mission that is up
#[derive(Clone, Debug, Default, ToBytes)]
pub struct LessonView {
    /// the step that is up, from 0, and how many there are
    pub step: u32,
    pub of: u32,
    /// its id. (None: every step is done)
    pub id: Option<String>,
    /// every step is done
    pub finished: bool,
    /// the run is over, whichever way
    pub over: bool,
    /// the step's deed is done and the moment is playing out
    pub settling: bool,
    /// the step asks him to move the view: nothing else moves it meanwhile
    pub asks_view: bool,
    /// (None: a lesson with no frame over it)
    pub rails: Option<crate::view::RailsView>,
}

/// what a place in the kit's strip holds
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes)]
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
#[derive(Clone, Debug, ToBytes)]
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
#[derive(Clone, Debug, Default, ToBytes)]
pub struct KitWord {
    pub text: String,
    /// it is the one that is up
    pub on: bool,
    /// a load he is out of
    pub out: bool,
}

/// the pistol's OCP
#[derive(Clone, Debug, Default, ToBytes)]
pub struct KitOcp {
    /// 1 full
    pub charge: f64,
    pub ready: bool,
}

/// the gun that is up, as its box shows it
#[derive(Clone, Debug, Default, ToBytes)]
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
#[derive(Clone, Debug, Default, ToBytes)]
pub struct KitSetupRow {
    /// the code of its key
    pub key: String,
    pub words: Vec<crate::view::KitWord>,
}

/// the setup of the gun in his hands, while it is open
#[derive(Clone, Debug, Default, ToBytes)]
pub struct KitSetup {
    pub rows: Vec<crate::view::KitSetupRow>,
    /// the key that backs out of it
    pub back: String,
}

/// the big slot: what is worked at a door, on the door's own key
#[derive(Clone, Debug, Default, ToBytes)]
pub struct KitBig {
    pub name: String,
    /// where it is worked
    pub place: String,
    /// a door in reach offers it now
    pub offered: bool,
}

/// WHAT A SEAT'S MAN CARRIES AND THE STATE HE IS IN, AS THE KIT'S COLUMN SHOWS IT
#[derive(Clone, Debug, Default, ToBytes)]
pub struct KitView {
    /// the strip, in its order (no places: a game no host steps)
    pub slots: Vec<crate::view::KitSlot>,
    /// the gun that is up
    pub gun: Option<crate::view::KitGun>,
    /// the setup, while it is open
    pub setup: Option<crate::view::KitSetup>,
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
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes)]
pub enum HeadTone {
    Calm,
    /// out cold, and breathing
    Ko,
    Dead,
    Alert,
    Sus,
}

/// a man on the floor, as the HUD marks him
#[derive(Clone, Debug, ToBytes)]
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
#[derive(Clone, Copy, Debug, Default, ToBytes)]
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
#[derive(Clone, Copy, Debug, Default, ToBytes)]
pub struct CamMark {
    pub at: V3,
    /// it is the one sending a picture
    pub sends: bool,
}

/// a defender's live grenade the team has been cued on (the cue is one a grenade, and made about the first man: in a game of two the second is told of the same)
#[derive(Clone, Copy, Debug, Default, ToBytes)]
pub struct FragMark {
    pub at: V3,
    /// seconds left on its fuse
    pub left: f64,
}

/// WHAT THE HUD MARKS ON THE FLOOR AND SAYS OF THE SEAT'S MAN
#[derive(Clone, Debug, Default, ToBytes)]
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
    /// what the hearing model has his feet on (None: the floors are all one)
    pub floor: Option<crate::view::FloorWord>,
}

/// what the auto-aim has snapped to
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes)]
pub enum RingOn {
    /// nothing: the bare point
    Free,
    /// a light or a fixture
    Light,
    /// a man
    Guard,
}

/// the aim model's cone at the ring
#[derive(Clone, Copy, Debug, Default, ToBytes)]
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
#[derive(Clone, Copy, Debug, ToBytes)]
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
#[derive(Clone, Copy, Debug, Default, ToBytes)]
pub struct GunDot {
    pub at: V3,
    pub shut: bool,
}

/// what a thing in the air would reach, flat on the floor round a point
#[derive(Clone, Copy, Debug, Default, ToBytes)]
pub struct ThrowReachView {
    pub at: V3,
    pub kill: f64,
    pub hurt: f64,
    pub stun: f64,
}

/// the arc he is aiming
#[derive(Clone, Debug, Default, ToBytes)]
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
#[derive(Clone, Debug, Default, ToBytes)]
pub struct AimView {
    pub ring: Option<crate::view::AimRing>,
    pub dot: Option<crate::view::GunDot>,
    /// the word at the reticle: the 40 mm's arc is too short to arm
    pub too_close: bool,
    pub throw: Option<crate::view::ThrowView>,
}

/// what a man wears and carries that the drawer draws
#[derive(Clone, Copy, Debug, ToBytes)]
pub struct WornView {
    /// his character's id
    pub man: f64,
    pub goggles: bool,
    pub big: Option<crate::bigkit::BigCarried>,
}
impl Default for WornView { fn default() -> Self { WornView { man: -1.0, goggles: false, big: None } } }

/// WHAT A SEAT'S HUD IS TOLD OF THE GAME AS A TICK LEFT IT: a section for each kind of thing a HUD words. Nothing in it is kept by the game.
#[derive(Clone, Debug, Default, ToBytes)]
pub struct TickView {
    pub mission: crate::view::MissionView,
    /// the lesson that is live. (None: none is)
    pub lesson: Option<crate::view::LessonView>,
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
    /// the squad's words (None: no squad answers to the seat)
    pub squad: Option<crate::view::SquadView>,
    /// what a set charge and a held threshold say to this seat, squad or none: a key's line, or a warning under no key
    pub corner: Vec<crate::view::Note>,
    /// the throws of the seat's men from the order until they leave the hand, and the preview of the row lit on its ring
    pub throws: Vec<crate::view::ThrowLine>,
    /// the squad's ring, while it is up
    pub ring: Option<crate::view::RingView>,
    /// his own ring, while it is up
    pub hand: Option<crate::view::HandRingView>,
    /// what a tap of the key that found no row says, while it is still up and still so
    pub empty: Option<crate::view::EmptyView>,
    /// the plan on the floor, the seat's and each other player's: empty with nothing to show
    pub plan: crate::view::PlanView,
    /// the run is live (no show, no menu's game, not over)
    pub live: bool,
    /// a quiet mission's words for its room, its holder, its thing and its ways out
    pub quiet: Option<crate::view::QuietView>,
    /// his own list: what the key would do now, and the things it could be done to
    pub interact: crate::view::InteractView,
    /// the alarm's stage (None: calm)
    pub alarm: Option<crate::view::AlarmView>,
    /// the mission's stage
    pub stage: crate::view::Stage,
    /// the rule tables' numbers that say how the HUD lays out what it is told
    pub look: crate::view::HudLook,
    /// each man whose attention has been taken, as the floor shows it
    pub attention: Vec<crate::view::AttentionLine>,
}

/// which pair of the squad a man is of
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes)]
pub enum Pair {
    Red,
    Blue,
}

/// one man's line on the squad's block
#[derive(Clone, Debug, Default, ToBytes)]
pub struct MateLine {
    /// his character's id
    pub man: f64,
    /// his callsign
    pub name: String,
    /// what he is doing, and what he covers
    pub doing: String,
    /// among the men the next order goes to, while not all of them are
    pub addressed: bool,
    /// his pair, while the squad is in pairs
    pub pair: Option<crate::view::Pair>,
    pub down: bool,
    /// the card he was staged with in a line ('' = none)
    pub kit: String,
}

/// a line of the squad's block
#[derive(Clone, Debug, Default, ToBytes)]
pub struct Note {
    /// the key that does it ('' = none): the default key's letter, the page names the player's own
    pub key: String,
    pub text: String,
}

/// WHAT A SEAT'S HUD IS TOLD OF ITS SQUAD
#[derive(Clone, Debug, Default, ToBytes)]
pub struct SquadView {
    /// loud or quiet, at a glance
    pub posture: Option<String>,
    /// who the next order goes to
    pub who: String,
    /// not everybody is addressed
    pub picked: bool,
    pub mates: Vec<crate::view::MateLine>,
    /// the block's notes, in its order: the door tools', the word's, the pair's mark, the smoke, the wall charges'
    pub notes: Vec<crate::view::Note>,
    /// the quick key's line for what the cursor is on (None: it would give nothing, a menu is up, or the mission has not taught it)
    pub quick: Option<crate::view::Note>,
}

/// what kind of thing an order would be about
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes)]
pub enum UnderKind {
    Floor,
    Door,
    /// a man of the squad's own, or the squad itself (no man)
    None,
    Foe,
}
/// a wall a charge might go on, by its place in the level's boxes
#[derive(Clone, Debug, Default, ToBytes)]
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
#[derive(Clone, Debug, ToBytes)]
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
#[derive(Clone, Debug, Default, ToBytes)]
pub struct CursorView {
    /// (None: no squad answers to this seat)
    pub under: Option<crate::view::Under>,
}

/// WHAT A HOST KNOWS THAT THE GAME HAS NOT SEEN YET: where the cursor is, and through which camera, as the frame is drawn (the game's own rows are of its last
/// step). Nothing else: no key, no click, no clock of the machine.
#[derive(Clone, Debug, Default, ToBytes)]
pub struct HostNow {
    /// the cursor and the picture it is in, in the page's pixels
    pub mouse: [f64; 2],
    pub client: [f64; 2],
    pub view_proj: crate::v3::m4::Mat4,
    pub inv_view_proj: crate::v3::m4::Mat4,
}

/// one row of the card's timeline, in words
#[derive(Clone, Debug, ToBytes)]
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

/// WHAT THE END CARD SHOWS OF A RUN: asked once, as the card opens. Large, so it is no section of `TickView`.
#[derive(Clone, Debug, Default, ToBytes)]
pub struct DebriefView {
    pub debrief: crate::chronicle::Debrief,
    pub rows: Vec<crate::view::DebriefRow>,
    /// rows the timeline's caps refused
    pub dropped: f64,
    /// where the seat's man went: x, z, t a point
    pub path: Vec<f32>,
    /// who of ours fell and how (None: somebody still plays, or nobody of ours went down)
    pub recap: Option<crate::deathrecap::DeathRecap>,
}

/// WHAT A MACHINE KEEPS OF A RUN: asked as a try begins and as the run ends. The game says what there is to keep, and a page keeps it.
#[derive(Clone, Debug, Default, ToBytes)]
pub struct RunView {
    /// the try has begun (an order, a stride, a round), on a run that may be kept
    pub begun: bool,
    /// the run is over
    pub over: bool,
    /// a cheat was on during it
    pub void: bool,
    /// a lesson: nothing of it is kept
    pub lesson: bool,
    /// the mission's id, as the list has it, and the level of difficulty
    pub mission: String,
    pub level: String,
    /// how it ended: won, the way it was won (None: it was not), nobody of ours lost, its seconds
    pub won: bool,
    pub way: Option<crate::verdict::Approach>,
    pub clean: bool,
    pub secs: f64,
    /// what its time is held against (the level's file, the way in, how many came, the difficulty), and the total on the par clock (None: no par, or not taken)
    pub course: String,
    pub par: Option<f64>,
}

/// a card's two weights: how readily its man moves, and how little of him is seen
#[derive(Clone, Copy, Debug, Default, ToBytes)]
pub struct CardLoad {
    pub mob: f64,
    pub con: f64,
}

/// a bar of what a gun is good at, 0 to 1, and where the gun he is issued stands on it
#[derive(Clone, Copy, Debug, Default, ToBytes)]
pub struct CardBar {
    pub now: f64,
    pub issued: f64,
}

/// a gun a slot could hold
#[derive(Clone, Debug, Default, ToBytes)]
pub struct CardPick {
    pub id: String,
    pub word: String,
    pub note: String,
    /// why it may not be picked (empty: it may)
    pub why: String,
    /// it is the one on the card
    pub now: bool,
    /// it is drawn with its suppressor
    pub can: bool,
    /// the kind of gun it is drawn as (empty: none)
    pub icon: String,
    /// none where it is no gun
    pub bars: Vec<f64>,
}

/// what a row of a card is
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes)]
pub enum CardKind {
    Count,
    Flag,
    Choice,
}

/// a row of a man's card
#[derive(Clone, Debug, ToBytes)]
pub struct CardRow {
    pub field: String,
    pub label: String,
    pub kind: crate::view::CardKind,
    /// what it is at, by its own id
    pub value: String,
    /// how many (a count), or 1 (a thing he has)
    pub n: f64,
    pub word: String,
    /// what that changes in a run
    pub note: String,
    /// what it fills and its rate (empty: it is no pouch)
    pub cost: String,
    /// the gun's row it rides under (empty: its own)
    pub under: String,
    /// it is one of the two colors
    pub look: bool,
    /// he carries none of it
    pub none: bool,
    /// the gun leaves no choice
    pub fixed: bool,
    /// it is not as he is issued
    pub changed: bool,
    /// he carries it and the mission has no use for it
    pub idle: bool,
    /// why the step to the value before (after) is refused; empty: it is not
    pub less: String,
    pub more: String,
    /// the gun of the row wears a suppressor
    pub can: bool,
    /// the kind of gun it is drawn as (empty: none, or no gun's row)
    pub icon: String,
    /// how loud the thing is when used: 0 none, 1 nearby, 2 the floor, 3 a blast
    pub noise: f64,
    /// what it takes of the two weights, or what one more would
    pub takes: Option<crate::view::CardLoad>,
    pub would: Option<crate::view::CardLoad>,
    /// a gun's: reach, volume, handling, quiet
    pub bars: Vec<crate::view::CardBar>,
    /// the guns the slot could hold
    pub picks: Vec<crate::view::CardPick>,
}
impl Default for CardRow { fn default() -> Self { CardRow { field: String::new(), label: String::new(), kind: CardKind::Count, value: String::new(), n: 0.0, word: String::new(), note: String::new(), cost: String::new(), under: String::new(), look: false, none: false, fixed: false, changed: false, idle: false, less: String::new(), more: String::new(), can: false, icon: String::new(), noise: 0.0, takes: None, would: None, bars: Vec::new(), picks: Vec::new() } } }

/// pouches, full and in all
#[derive(Clone, Copy, Debug, Default, ToBytes)]
pub struct CardPouches {
    pub used: f64,
    pub total: f64,
}

/// a man's card as it is shown: the rows it has as it stands, a gun before what rides on it
#[derive(Clone, Debug, Default, ToBytes)]
pub struct CardView {
    pub who: f64,
    pub changed: bool,
    pub pouches: crate::view::CardPouches,
    /// the big slot (None: the table gives none)
    pub big: Option<crate::view::CardPouches>,
    /// what the squad carries between them (None: no limit, or not one of those who come)
    pub squad: Option<crate::view::CardPouches>,
    pub load: crate::view::CardLoad,
    pub load_issued: crate::view::CardLoad,
    pub rows: Vec<crate::view::CardRow>,
    /// the whole card is as issued
    pub issued: bool,
    /// why it may not be put back as issued (empty: it may)
    pub reset_why: String,
    /// a card is copied
    pub copied: bool,
    /// he wears the copied card
    pub clip: bool,
    /// why the copied card may not be put on him (empty: it may, or nothing is copied)
    pub paste_why: String,
    /// the card in a line, and what was left behind of a kept one
    pub words: String,
    /// the colors picked (None: as issued)
    pub suit: Option<V3>,
    pub trim: Option<V3>,
}

/// a text this machine keeps, by the name it is kept under
#[derive(Clone, Debug, Default, ToBytes)]
pub struct KeptText {
    pub key: String,
    /// empty: kept no longer
    pub text: String,
}

/// what a press on a card did: taken, or why not; what it walked past; the rows it moved; and what is kept from now
#[derive(Clone, Debug, Default, ToBytes)]
pub struct Pressed {
    pub ok: bool,
    pub why: String,
    pub passed: String,
    pub changed: Vec<String>,
    pub kept: Vec<crate::view::KeptText>,
}

/// a size of the team
#[derive(Clone, Debug, Default, ToBytes)]
pub struct TeamSize {
    pub n: f64,
    /// it can be picked on this mission
    pub ok: bool,
    /// it is the mission's own
    pub own: bool,
    pub words: String,
    /// why it can not, or what it changes
    pub note: String,
}

/// a named kit of the squad's row
#[derive(Clone, Debug, Default, ToBytes)]
pub struct TeamKit {
    pub id: String,
    pub name: String,
    /// the cards are this kit
    pub now: bool,
    /// it is offered at this size
    pub offered: bool,
}

/// a thing a man carries, as his chip draws it
#[derive(Clone, Debug, Default, ToBytes)]
pub struct TeamThing {
    pub field: String,
    pub value: String,
    pub label: String,
    /// how many (0: not a thing that is counted)
    pub n: f64,
    pub can: bool,
    pub icon: String,
}

/// a place of the team
#[derive(Clone, Debug, Default, ToBytes)]
pub struct TeamMan {
    /// somebody comes in it
    pub here: bool,
    pub title: String,
    pub sub: String,
    /// "you", "red" or "blue"
    pub pair: String,
    /// his card is not as issued
    pub changed: bool,
    /// his kit in a line: his card's, or what the mission hands him
    pub words: String,
    pub pouches: crate::view::CardPouches,
    /// the gun in his hands: its row's id, the kind it is drawn as, and whether it wears its suppressor
    pub gun: String,
    pub icon: String,
    pub can: bool,
    /// he wears the copied card
    pub clip: bool,
    pub things: Vec<crate::view::TeamThing>,
    /// the card he will be staged with, as a card is kept
    pub worn: String,
}

/// the team as it would go on the mission the cards are for
#[derive(Clone, Debug, Default, ToBytes)]
pub struct TeamView {
    pub size: f64,
    pub sizes: Vec<crate::view::TeamSize>,
    /// the size's line, with what a mission that issues its own kit says
    pub note: String,
    /// there are cards to change (not on a lesson, nor where the mission issues the kit)
    pub cards: bool,
    /// the mission is played with the kit as issued
    pub as_issued: bool,
    /// what the squad's pouches hold between them (empty: no limit, or nobody comes)
    pub squad: String,
    /// what is carried between them
    pub pool: String,
    /// the mission hands out wall charges of its own
    pub walls: bool,
    pub kits: Vec<crate::view::TeamKit>,
    /// you first, then the four places
    pub men: Vec<crate::view::TeamMan>,
    /// on your card, and between the others
    pub frags: f64,
    pub smoke: f64,
    pub bangs: f64,
    /// the link Deploy loads
    pub link: String,
}

/// one thing a setting can be
#[derive(Clone, Debug, Default, ToBytes)]
pub struct SettingChoice {
    pub id: String,
    pub label: String,
    /// what it does
    pub title: String,
    pub on: bool,
    /// it is offered on this mission
    pub shown: bool,
}

/// a setting of a mission before it is played: the difficulty, one of its levers, the hour, the weather, the way in
#[derive(Clone, Debug, Default, ToBytes)]
pub struct Setting {
    pub key: String,
    pub label: String,
    /// it changes something on this mission
    pub live: bool,
    /// it is drawn for this mission
    pub shown: bool,
    /// a lever moved off its level's rung by hand
    pub moved: bool,
    /// what the choice that is on does
    pub note: String,
    pub choices: Vec<crate::view::SettingChoice>,
}

/// a mission as the list has it
#[derive(Clone, Debug, Default, ToBytes)]
pub struct StartRow {
    pub id: String,
    pub title: String,
    /// "solo" for the plain solo night, else its scenario's kind
    pub kind: String,
    pub group: String,
    pub group_title: String,
    pub group_line: String,
    pub place: String,
    pub blurb: String,
    pub lean: String,
    pub lean_words: String,
    /// its level's file
    pub level: String,
    pub level_words: String,
    pub squad: f64,
    pub team_words: String,
    pub team_tag: String,
    /// nobody has balanced it, and the short list does not name it
    pub unplayed: bool,
    /// cut from the lists (its own address still finds it)
    pub off_list: bool,
    /// it is played with the kit as issued
    pub as_issued: bool,
    pub lesson: bool,
    /// two can play it
    pub two: bool,
    /// the levels' ways in, by name (none: it has one)
    pub ways: Vec<String>,
    /// the short list's row of it, and its one line (empty: not on the short list)
    pub pick: String,
    pub line: String,
    /// the team's size the short list starts it with (-1: the mission's own)
    pub pick_squad: f64,
    /// the link the row has
    pub link: String,
}

/// the list of missions, in the order the page draws them
#[derive(Clone, Debug, Default, ToBytes)]
pub struct StartView {
    pub rows: Vec<crate::view::StartRow>,
    /// the mission the address names (empty: none)
    pub named: String,
    /// the short list's row the address is (empty: none)
    pub pick: String,
    /// the address starts something, so the list is not the way in
    pub playing: bool,
    /// the row a visit that names none and has started none opens on: the short list's first
    pub first: String,
}

/// a mission as its row of the table has it, for who reads the rows themselves and draws no card. What a row does not say is empty
#[derive(Clone, Debug, Default, ToBytes)]
pub struct MissionRow {
    pub id: String,
    pub title: String,
    /// "solo" for the plain solo night, else its scenario's kind
    pub kind: String,
    pub squad: f64,
    /// the group it is under
    pub group: String,
    /// its level's file
    pub level: String,
    /// the hour it is always started in
    pub hour: String,
    pub place: String,
    pub blurb: String,
    /// a lesson's: where we are and why, and what it teaches
    pub scene: String,
    pub teaches: Vec<String>,
    /// it deploys at its own size and no other: what the other sizes say
    pub only: String,
    /// how many hold it
    pub defenders: f64,
    /// they are in squads
    pub squads: bool,
    /// its level's ways in, by name (none: it has one)
    pub ways: Vec<String>,
    /// nobody has balanced it
    pub untuned: bool,
    /// cut from the lists
    pub off_list: bool,
    /// the short list's one line of it
    pub line: String,
    /// the address that is this row
    pub link: String,
}

/// every mission there is: the plain solo night, then the tables in their order
#[derive(Clone, Debug, Default, ToBytes)]
pub struct MissionsView {
    pub rows: Vec<crate::view::MissionRow>,
}

/// a mission's sheet
#[derive(Clone, Debug, Default, ToBytes)]
pub struct SheetView {
    pub id: String,
    pub settings: Vec<crate::view::Setting>,
    /// the link the row has, and the link its team is deployed by (the short list's own where the settings are still its own), without the team's size
    pub link: String,
    pub base: String,
    /// the settings are the short list's for this mission
    pub as_pick: bool,
    /// its kind's gear screen is the quiet one
    pub quiet: bool,
}

/// what a mission's page says of it while it loads
#[derive(Clone, Debug, Default, ToBytes)]
pub struct BriefView {
    /// the row of the list (empty: the address starts none, and the rest is empty too)
    pub id: String,
    pub title: String,
    /// what it is ('solo' for the plain night)
    pub kind: String,
    /// the heading it is under
    pub group_title: String,
    pub place: String,
    pub blurb: String,
    pub lean_words: String,
    /// the team as this address deploys it
    pub team: String,
    /// its level's file
    pub level: String,
    /// a lesson's: where we are and why
    pub scene: String,
    /// a lesson's: what it teaches
    pub teaches: Vec<String>,
    /// the way in the address asks for: 0 the level's own, k its k-th
    pub way: f64,
}

/// what the pause and end cards say of the short list, on a page one of its rows started
#[derive(Clone, Debug, Default, ToBytes)]
pub struct ShortView {
    /// the row the address is (empty: none, and the rest is empty too)
    pub now: String,
    /// that row starts a scenario (the solo night's starts none)
    pub now_scenario: bool,
    /// the row after it (empty: it was the last)
    pub next: String,
    /// the mission that row starts
    pub next_mission: String,
    /// what the card calls it, with a difficulty the player chose carried on
    pub title: String,
    /// its one plain line
    pub line: String,
    /// the page load that starts it
    pub link: String,
    /// the same row at the other difficulty (empty: it has one)
    pub other_link: String,
}

/// what is thrown
#[derive(Clone, Copy, Debug, Default, PartialEq, ToBytes)]
pub enum ThrowKind { #[default] Frag, Stun, Shell }

/// a throw of a man of the seat's, ordered or only offered by the lit row: who, his walk, the arc, where it rests and what it reaches there
#[derive(Clone, Debug, Default, ToBytes)]
pub struct ThrowLine {
    /// his character's id
    pub man: f64,
    pub kind: crate::view::ThrowKind,
    /// the floor he throws from
    pub from: V3,
    pub points: Vec<V3>,
    pub land: V3,
    /// his way there from where he is (None: he throws where he stands)
    pub walk: Option<Vec<V3>>,
    pub preview: bool,
    /// metres it kills in and wounds in (0: none)
    pub kill: f64,
    pub wound: f64,
    /// his pair, by name (None: he is in none)
    pub tone: Option<crate::view::Tone>,
}

/// a colour by its name
#[derive(Clone, Copy, Debug, Default, PartialEq, ToBytes)]
pub enum Tone { #[default] Red, Blue, Gold }

/// one wedge of a ring, on its digit
#[derive(Clone, Debug, Default, ToBytes)]
pub struct RingWedge {
    pub key: f64,
    pub text: String,
    /// it cannot be given now
    pub off: bool,
    /// it turns the ring to another page
    pub opens: bool,
}

/// a ring that stays on its point: the place, where its middle stands from it, and how near the view's edge it may stand (px)
#[derive(Clone, Copy, Debug, Default, ToBytes)]
pub struct RingPin {
    pub at: V3,
    pub ox: f64,
    pub oy: f64,
    pub edge_x: f64,
    pub edge_y: f64,
}

/// the squad's ring as it stands
#[derive(Clone, Debug, Default, ToBytes)]
pub struct RingView {
    pub x: f64,
    pub y: f64,
    pub zoom: f64,
    /// the lit wedge's digit (0: none)
    pub lit: f64,
    pub tone: Option<crate::view::Tone>,
    /// who it goes to, and what it was opened on
    pub caption: String,
    pub wedges: Vec<crate::view::RingWedge>,
    pub pin: Option<crate::view::RingPin>,
}

/// his own ring as it stands
#[derive(Clone, Debug, Default, ToBytes)]
pub struct HandRingView {
    pub x: f64,
    pub y: f64,
    pub zoom: f64,
    pub lit: f64,
    pub wedges: Vec<crate::view::RingWedge>,
    pub pin: Option<crate::view::RingPin>,
}

/// a tap's answer: what it says, what it was beside (its kind's word, 'wall': nothing), and how far off a door a step short is
#[derive(Clone, Debug, Default, ToBytes)]
pub struct EmptyView {
    pub word: String,
    pub kind: crate::view::ThingKind,
    pub dist: Option<f64>,
}

/// the way an order walks a man, for the HUD to draw: his own path where it is this order's, else the stops it goes through, each leg of which is a question of the floor (`WorldAsk::way`)
#[derive(Clone, Debug, Default, ToBytes)]
pub struct PlanWay {
    /// his feet
    pub from: V3,
    /// his own path from where he is on it, where it is this order's and runs all the way
    pub own: Option<Vec<V3>>,
    /// where the order walks him through, the mark last
    pub stops: Vec<V3>,
}

/// a man with a mark
#[derive(Clone, Debug, Default, ToBytes)]
pub struct PlanMan {
    pub man: f64,
    pub tag: String,
    pub tone: Option<crate::view::Tone>,
    /// seconds since the order came
    pub age: f64,
    pub way: Option<crate::view::PlanWay>,
    pub mark: V3,
    pub there: bool,
    /// the fight has him: drawn dim
    pub held: bool,
    /// the yaw he watches
    pub aim: Option<f64>,
    /// the point he watches
    pub watch: Option<V3>,
    pub on_door: bool,
}

/// who an open menu goes to
#[derive(Clone, Copy, Debug, Default, ToBytes)]
pub struct PlanWho {
    pub man: f64,
    pub at: V3,
    pub tone: Option<crate::view::Tone>,
}

/// what waits on a key
#[derive(Clone, Copy, Debug, Default, PartialEq, ToBytes)]
pub enum WaitWhy { #[default] Stack, Tool, Charge, Thr, Drill, Wall, Pair }

/// how a door's key is drawn: none · it waits on the key and is not set · they stand there and the press sends them
#[derive(Clone, Copy, Debug, Default, PartialEq, ToBytes)]
pub enum KeyCap { #[default] None, Dark, Lit }

/// something waiting on the player's key: a doorway's two jambs (a wall's foot), whose it is, the key that sends it and whether that key would send this one now
#[derive(Clone, Debug, Default, ToBytes)]
pub struct PlanDoor {
    pub door: Option<String>,
    pub a: V3,
    pub b: V3,
    pub tone: Option<crate::view::Tone>,
    pub key: Option<String>,
    pub now: bool,
    pub cap: crate::view::KeyCap,
    pub why: crate::view::WaitWhy,
}

/// a throw on the plan: as `ThrowLine`, the arc on along the floor to its rest
#[derive(Clone, Debug, Default, ToBytes)]
pub struct PlanThrow {
    pub man: f64,
    pub tag: String,
    pub tone: Option<crate::view::Tone>,
    pub kind: crate::view::ThrowKind,
    pub preview: bool,
    pub from: V3,
    pub walk: Vec<V3>,
    pub arc: Vec<V3>,
    pub land: V3,
    pub reach: f64,
}

/// one seat's plan
#[derive(Clone, Debug, Default, ToBytes)]
pub struct SeatPlan {
    pub men: Vec<crate::view::PlanMan>,
    pub who: Vec<crate::view::PlanWho>,
    pub you: Option<V3>,
    pub doors: Vec<crate::view::PlanDoor>,
    pub throws: Vec<crate::view::PlanThrow>,
    /// the seat is reading the plan: false, `men` holds only orders just given, going out with their age
    pub mode: bool,
}

/// the plan on the floor: the seat's own whole, then each other player's, what is only its own player's left out and a wait the seat's own lists left to that
#[derive(Clone, Debug, Default, ToBytes)]
pub struct PlanView {
    pub own: crate::view::SeatPlan,
    pub others: Vec<crate::view::SeatPlan>,
}

/// a quiet mission's own words
#[derive(Clone, Debug, Default, ToBytes)]
pub struct QuietView {
    pub place: String,
    pub holder: String,
    pub thing: String,
    pub outs: String,
    pub bare: String,
}

/// what sort of thing the key is about
#[derive(Clone, Copy, Debug, Default, PartialEq, ToBytes)]
pub enum ThingKind { #[default] Cam, Guard, Breaker, Door, Light, Body, Objective, Held, Wall, Switch, Charge, Cancel }

/// the mission's stage
#[derive(Clone, Copy, Debug, Default, PartialEq, ToBytes)]
pub enum Stage { Drive, #[default] Infiltrate, Exfil, Done, Failed }

/// what a held attention is: he goes over, he only looks, or another rule has lent it
#[derive(Clone, Copy, Debug, Default, PartialEq, ToBytes)]
pub enum AttentionMode { #[default] Go, Look, Lent }

/// what a floor is, as a man's feet are heard on it
#[derive(Clone, Copy, Debug, Default, PartialEq, ToBytes)]
pub enum FloorWord { #[default] Carpet, Lino, Stone, Tile, Asphalt, Grass }

/// the key that backs out
#[derive(Clone, Copy, Debug, Default, PartialEq, ToBytes)]
pub enum BackKey { Tab, #[default] Esc }

/// a line of a row of his list: the key that does it ('' = a line no key gives), by its default name for the page to name as bound
#[derive(Clone, Debug, Default, ToBytes)]
pub struct RowOpt {
    pub key: String,
    pub text: String,
}

/// a row of his list
#[derive(Clone, Debug, Default, ToBytes)]
pub struct ListRow {
    /// what sort of thing it is about
    pub kind: crate::view::ThingKind,
    /// the thing's name and state
    pub label: String,
    pub opts: Vec<crate::view::RowOpt>,
    pub in_reach: bool,
    /// a thrown breaker, a light that is out
    pub off: bool,
    /// how far off, in metres, out of reach
    pub dist: Option<f64>,
    /// how far a running verb has got, 0 to 1
    pub progress: Option<f64>,
    /// about the same thing as the row above it
    pub cont: bool,
}

/// the thing the highlighted row is about, which the HUD rings
#[derive(Clone, Debug, Default, ToBytes)]
pub struct ThingRing {
    pub kind: crate::view::ThingKind,
    pub at: V3,
    /// the man it is on, whose figure it rides between ticks, `up` over his feet (None: a thing that stays at `at`, a body with them)
    pub man: Option<f64>,
    pub up: f64,
    pub off: bool,
    pub in_reach: bool,
    pub progress: Option<f64>,
}

/// a thing the key could act on, whether or not it is in reach
#[derive(Clone, Debug, Default, ToBytes)]
pub struct ThingDot {
    pub kind: crate::view::ThingKind,
    pub at: V3,
    /// as on `ThingRing`
    pub man: Option<f64>,
    pub up: f64,
    pub off: bool,
    /// about a man the fog hides from the seat's team
    pub hidden: bool,
    /// the highlighted row is about it, and the ring is on it
    pub ringed: bool,
}

/// his list as the panel shows it, and the things on the floor it is about
#[derive(Clone, Debug, Default, ToBytes)]
pub struct InteractView {
    pub rows: Vec<crate::view::ListRow>,
    /// the highlighted row among them (-1: none)
    pub lit: f64,
    /// how many were left off the end
    pub more: f64,
    /// the selector is open: he holds the key and the list is its rows
    pub open: bool,
    /// the panel is the corner's three things (a puppet keeps the list as it was)
    pub corner: bool,
    /// (None: no thing, or a row about none)
    pub ring: Option<crate::view::ThingRing>,
    pub dots: Vec<crate::view::ThingDot>,
}

/// the alarm's stage
#[derive(Clone, Debug, Default, ToBytes)]
pub struct AlarmView {
    /// 1 heightened, 2 lockdown
    pub level: f64,
    /// seconds left on its clock (0: none is running)
    pub left: f64,
    /// an episode is running
    pub live: bool,
}

/// how a ring with one row is drawn: the numbers the game reads the cursor by (ringlone)
#[derive(Clone, Debug, Default, ToBytes)]
pub struct LoneLook {
    pub r: f64,
    pub h: f64,
    pub pad: f64,
    /// a letter's width
    pub letter: f64,
    /// the most letters a pill is drawn for
    pub max: f64,
    pub lead: f64,
}

/// the numbers of a ring as the game reads a cursor by them
#[derive(Clone, Debug, Default, ToBytes)]
pub struct RingLook {
    /// the places on the wheel
    pub places: f64,
    pub hole: f64,
    pub rim: f64,
    /// how far each way from its middle its words may lie
    pub box_x: f64,
    pub box_y: f64,
    /// a ring with one row is a pill, on these numbers
    pub lone: crate::view::LoneLook,
}

/// the plan on the floor's numbers
#[derive(Clone, Debug, Default, ToBytes)]
pub struct PlanLook {
    pub fresh_secs: f64,
    pub walk_dim: f64,
    pub rest_dim: f64,
    pub ring_m: f64,
    pub sector_m: f64,
    pub sector_deg: f64,
    pub reask_secs: f64,
    /// how strong another player's plan is drawn (0: not at all)
    pub others: f64,
}

/// where a line from off the screen goes
#[derive(Clone, Debug, Default, ToBytes)]
pub struct NetLook {
    /// at the edge of the view on his side (false: near you)
    pub edge: bool,
    pub near: f64,
    pub edge_in: f64,
}

/// a held attention's numbers
#[derive(Clone, Debug, Default, ToBytes)]
pub struct AttentionLook {
    pub ring_m: f64,
    pub way_reask: f64,
}

/// the rule tables' numbers that say how the HUD lays out what it is told. Whatever a rule of the tick reads is read here too, so a page holds no second copy of it
#[derive(Clone, Debug, Default, ToBytes)]
pub struct HudLook {
    pub ring: crate::view::RingLook,
    pub plan: crate::view::PlanLook,
    pub net: crate::view::NetLook,
    pub attention: crate::view::AttentionLook,
    /// a tutorial's big line keeps clear of your men
    pub line_clear: bool,
    /// a tutorial step's mark gives way to a ring over it
    pub mark_clear: bool,
    /// the key that backs out of the selector
    pub back: crate::view::BackKey,
}

/// one man's held attention: a thread from his feet to the place and a ring there
#[derive(Clone, Debug, Default, ToBytes)]
pub struct AttentionLine {
    /// his character's id
    pub man: f64,
    pub at: V3,
    /// how much of the ring is still to run (1 while he walks to it or it is lent)
    pub left: f64,
    pub mode: crate::view::AttentionMode,
    /// he has said he is done
    pub ending: bool,
    /// not his first inside the memory
    pub worn: bool,
    /// he is on his way over to it
    pub walking: bool,
    /// the fog hides him from the team
    pub hidden: bool,
    /// the thread is the way he will walk, not a straight line
    pub walks: bool,
    /// that way, where it is his own path, from where he is on it. (None: the floor is asked)
    pub own: Option<Vec<V3>>,
}
