//! What a host is told when it asks, beside the views: the box world's answers, a run's end, its saves, its map, the knobs and the panel's rows, a look ahead. Each is made as it is asked and kept by nobody here.
use crate::{bytes::ToBytes, crosses::Crosses, Words, V3};
use alloc::{string::String, vec::Vec};

/// the nearest thing along a ray: how far, and the normal of the face it is entered by
#[derive(Clone, Copy, Debug, ToBytes, Crosses)]
pub struct Met {
    pub t: f64,
    pub n: V3,
}
/// how a sound reaches: how far it comes, through where, from where it seems to come, how muffled it is (0 to 1) and how much of it carries
#[derive(Clone, Copy, Debug, ToBytes, Crosses)]
pub struct SoundPath {
    pub direct: bool,
    pub routed: bool,
    pub path_len: f64,
    pub via: V3,
    pub apparent: V3,
    pub muffle: f64,
    pub carry: f64,
}

/// WHERE A SEAT'S MAN WOULD BE after records the game has not had (game_rules `walkahead`)
#[derive(Clone, Debug, ToBytes, Crosses)]
pub struct AheadView {
    /// the man, by his character's number, and is he down
    pub id: f64,
    pub down: bool,
    /// where the game has him, and his aim
    pub at: V3,
    pub yaw: f64,
    /// the normal of a wall he holds (x z)
    pub wall: Option<[f64; 2]>,
    /// how near the nearest other body stands (m)
    pub near: f64,
    /// his legs are his own this tick
    pub free: bool,
    /// where the records take him, his aim there, what his seat's camera would follow with him there
    pub pos: V3,
    pub aim_yaw: f64,
    pub follow: V3,
    /// the point the newest record aims at
    pub aim: Option<V3>,
    /// his place after each record
    pub path: Vec<V3>,
}

/// how a hands-off player ends: no defender left · he is down · out of time · nothing left to try
#[derive(Clone, Copy, PartialEq, Eq, Debug, ToBytes, Words, Crosses)]
pub enum HandsEnd {
    Taken,
    Sam,
    Cap,
    Spent,
}
/// how he stands
#[derive(Clone, Copy, Debug, ToBytes, Crosses)]
pub struct HandsSaid {
    /// seconds played, how it ended, orders given, when a round last left a gun
    pub t: f64,
    pub end: Option<HandsEnd>,
    pub given: u32,
    pub shot_at: f64,
    /// the squad's men up and of how many, the defenders left
    pub up: u32,
    pub of: u32,
    pub left: u32,
}

/// the floor as a drawing: every solid thing between knee and head height, as x, z, half width, half depth, yaw in degrees, kind (0 a wall, 1 low, 2 glass), and the walls' bounds (x0 z0 x1 z1)
#[derive(Clone, Debug, ToBytes, Crosses)]
pub struct FloorView {
    pub bounds: [f64; 4],
    pub boxes: Vec<f64>,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug, ToBytes, Words, Crosses)]
pub enum MapKind {
    Player,
    Man,
    Civ,
}
#[derive(Clone, Debug, ToBytes, Crosses)]
pub struct MapMan {
    pub id: f64,
    pub x: f64,
    pub z: f64,
    pub aim: f64,
    pub team: String,
    pub kind: MapKind,
    pub down: bool,
    pub name: String,
    pub state: String,
}
#[derive(Clone, Debug, ToBytes, Crosses)]
pub struct MapDoor {
    /// its hinge, and the way its leaf points from it as it stands
    pub hx: f64,
    pub hz: f64,
    pub ux: f64,
    pub uz: f64,
    pub width: f64,
    pub locked: bool,
    pub broken: bool,
    pub off: bool,
    pub name: String,
}
/// who and what stands on the floor now, everybody, seen or not: the players first, in their seats' order
#[derive(Clone, Debug, ToBytes, Crosses)]
pub struct MapView {
    pub men: Vec<MapMan>,
    pub doors: Vec<MapDoor>,
}

/// a mark of an end card's map: where he came in, what he came for, the ways out
#[derive(Clone, Debug, ToBytes, Crosses)]
pub struct RunMark {
    pub glyph: String,
    pub cls: String,
    pub x: f64,
    pub z: f64,
    pub title: String,
}
/// a course's par, as the run stands against it
#[derive(Clone, Debug, ToBytes, Crosses)]
pub struct ParLine {
    pub medal: crate::par::Medal,
    pub line: String,
}
/// the walked floor, a figure a cell: 0 open ground, 1 indoors, 2 and 3 the same where nothing walks
#[derive(Clone, Debug, ToBytes, Crosses)]
pub struct WalkedFloor {
    pub w: u32,
    pub h: u32,
    pub cell: f64,
    pub rows: String,
}
/// WHAT AN END CARD IS DRAWN FROM beside its debrief
#[derive(Clone, Debug, ToBytes, Crosses)]
pub struct EndView {
    pub verdict: Option<crate::verdict::RunVerdict>,
    pub par: Option<ParLine>,
    /// what the thing to take is in, as the mission calls it (empty: a fight, with no errand)
    pub holder: String,
    pub marks: Vec<RunMark>,
    pub floor: WalkedFloor,
}

/// how a run ended, for who shows no card of it
#[derive(Clone, Debug, ToBytes, Crosses)]
pub struct OutcomeView {
    pub won: bool,
    pub word: String,
    pub why: String,
    pub secs: f64,
    pub approach: Option<crate::verdict::Approach>,
    pub lost: f64,
    pub of: f64,
    pub foes_down: f64,
    pub foes_of: f64,
    pub hostages: Option<crate::verdict::RunVerdictHostages>,
    pub void: bool,
    pub clean: bool,
}

/// a save as a card lays it out
#[derive(Clone, Debug, ToBytes, Crosses)]
pub struct SaveRow {
    pub key: String,
    pub label: String,
    pub slot: crate::savestate::SlotInfoSlot,
    /// the floor came down straight after
    pub regret: bool,
    /// the mission's clock as it was made, and which came after which
    pub mission_t: f64,
    pub saved_at: f64,
}
/// this run's saves: the mission's clock now, would one by hand be made, the key of the one to go back to (empty: none)
#[derive(Clone, Debug, ToBytes, Crosses)]
pub struct SavesView {
    pub elapsed: f64,
    pub hand: bool,
    pub offer: String,
    pub rows: Vec<SaveRow>,
}
/// what the save controller tells the player: its tag while it is up, and each thing it has said since it was last asked
#[derive(Clone, Debug, ToBytes, Crosses)]
pub struct SavesTold {
    pub tag: String,
    pub lines: Vec<String>,
}
/// a game that stands again: its new number, and what it stands from (empty: its run's first step)
#[derive(Clone, Debug, ToBytes, Crosses)]
pub struct Recovered {
    pub game: u32,
    pub label: String,
}

#[derive(Clone, Copy, Debug, ToBytes, Crosses)]
pub struct TallyMan {
    pub state: crate::game::GuardState,
    /// he carries a light, and it is on
    pub lit: bool,
}
/// WHAT A MEASURING TAKE NOTES OF THE GAME: each man by what he is at, the reaction force, the alarm, the run's saves, the mission
#[derive(Clone, Debug, ToBytes, Crosses)]
pub struct Tally {
    pub men: Vec<TallyMan>,
    pub qrf: crate::qrf::QrfState,
    pub escalation: f64,
    pub autosaves: f64,
    pub manual: f64,
    pub loads: f64,
    pub stage: crate::game::MissionStage,
    pub alarm: bool,
    pub alerts: f64,
    /// a room clear is staged
    pub clear: bool,
    /// the kind of mission its verdict is for
    pub kind: Option<crate::scene_scenarios::MissionKind>,
    /// the mission's own counts
    pub stats: crate::game::MissionStats,
}

/// a knob a person may turn, and what it is turned to
#[derive(Clone, Debug, ToBytes, Crosses)]
pub struct KnobRow {
    pub name: String,
    /// it is a yes or no
    pub yes: bool,
    pub value: f64,
}
/// a row of the panel over a number of the game
#[derive(Clone, Copy, Debug, ToBytes, Crosses)]
pub struct ShownRow {
    pub name: &'static str,
    pub label: &'static str,
    pub section: &'static str,
    pub title: &'static str,
    pub lo: f64,
    pub hi: f64,
    pub step: f64,
    /// what the row shows the number at
    pub scale: f64,
    /// the number as the game ships it (NaN: a number a level sets)
    pub def: f64,
    /// it stays turned on this machine · it waits behind 'all knobs'
    pub kept: bool,
    pub adv: bool,
}
/// how a row of the panel that does something is drawn: a button, a button a word, a pick of words, a number beside a button, a pick that is a new page of the address
#[derive(Clone, Copy, PartialEq, Eq, Debug, ToBytes, Words, Crosses)]
pub enum ActionKind {
    Press,
    Each,
    Choice,
    Count,
    Reload,
}
/// a row of the panel that does something to the game
#[derive(Clone, Debug, ToBytes, Crosses)]
pub struct ActionRow {
    pub name: &'static str,
    pub label: &'static str,
    pub section: &'static str,
    pub title: &'static str,
    pub kind: ActionKind,
    /// what is picked of, and (a choice) what it stands at now
    pub words: Vec<String>,
    pub now: String,
    /// (a count)
    pub lo: f64,
    pub hi: f64,
    pub step: f64,
    pub start: f64,
    /// (a reload) the address's word
    pub word: &'static str,
}
