//! The types of `pressure`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: Man, src/game/pressure.ts:233)
#[derive(Clone, Copy, Debug)]
pub struct Man {
    pub d: RoomfightDefId,
    pub role: crate::pressure::ManRole,
    pub door: DoorId,
    pub spot: V3,
    pub from: Option<RoomfightRoomId>,
    pub moving: bool,
    pub set: bool,
    pub set_at: f64,
    pub tries: f64,
    pub step_t: f64,
    pub small0: f64,
    pub cue_t: f64,
    pub free_at: f64,
    pub land_t: f64,
    pub arrived: bool,
    pub heard: bool,
    pub fell: bool,
    pub replant_t: f64,
    pub fought: bool,
    pub out: bool,
    pub ask_t: Option<f64>,
    pub charged: Option<bool>,
    pub ear_t: Option<f64>,
}
impl Man {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Man {
            d: RoomfightDefId(0),
            role: crate::pressure::ManRole::Press,
            door: DoorId(0),
            spot: [0.0; 3],
            from: None,
            moving: false,
            set: false,
            set_at: 0.0,
            tries: 0.0,
            step_t: 0.0,
            small0: 0.0,
            cue_t: 0.0,
            free_at: 0.0,
            land_t: 0.0,
            arrived: false,
            heard: false,
            fell: false,
            replant_t: 0.0,
            fought: false,
            out: false,
            ask_t: None,
            charged: None,
            ear_t: None,
        }
    }
}
impl Default for Man { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum ManRole {
    Press,
    Flank,
}
impl ManRole {
    pub const fn as_str(self) -> &'static str {
        match self {
            ManRole::Press => "press",
            ManRole::Flank => "flank",
        }
    }
}

/// (TypeScript: Press, src/game/pressure.ts:248)
#[derive(Clone, Debug)]
pub struct Press {
    pub id: f64,
    pub room: RoomfightRoomId,
    pub at: V3,
    pub t0: f64,
    pub phase: crate::pressure::PressPhase,
    pub team_in: bool,
    pub set_at: f64,
    pub lone_at: Option<f64>,
    pub sprung: Option<bool>,
    pub listen_at: f64,
    pub moving: bool,
    pub word_at: f64,
    pub go_at: f64,
    pub n0: f64,
    pub lone: bool,
    pub men: Vec<PressureManId>,
}
impl Press {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Press {
            id: 0.0,
            room: RoomfightRoomId(0),
            at: [0.0; 3],
            t0: 0.0,
            phase: crate::pressure::PressPhase::Go,
            team_in: false,
            set_at: 0.0,
            lone_at: None,
            sprung: None,
            listen_at: 0.0,
            moving: false,
            word_at: 0.0,
            go_at: 0.0,
            n0: 0.0,
            lone: false,
            men: Vec::new(),
        }
    }
}
impl Default for Press { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum PressPhase {
    Go,
    Word,
    Gather,
}
impl PressPhase {
    pub const fn as_str(self) -> &'static str {
        match self {
            PressPhase::Go => "go",
            PressPhase::Word => "word",
            PressPhase::Gather => "gather",
        }
    }
}

/// (TypeScript: Out, src/game/pressure.ts:250)
#[derive(Clone, Copy, Debug)]
pub struct Out {
    pub door: Option<DoorId>,
    pub def: RoomfightDefId,
    pub moving: bool,
    pub plant_t: f64,
    pub fought: bool,
    pub step_t: f64,
}
impl Out {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Out {
            door: None,
            def: RoomfightDefId(0),
            moving: false,
            plant_t: 0.0,
            fought: false,
            step_t: 0.0,
        }
    }
}
impl Default for Out { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Warn, src/game/pressure.ts:268)
#[derive(Clone, Copy, Debug)]
pub struct Warn {
    pub phase: crate::pressure::WarnPhase,
    pub t0: f64,
    pub by: RoomfightDefId,
    pub call_t: f64,
    pub last_t: f64,
    pub room: Option<RoomfightRoomId>,
    pub hit_t: Option<f64>,
}
impl Warn {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Warn {
            phase: crate::pressure::WarnPhase::Heard,
            t0: 0.0,
            by: RoomfightDefId(0),
            call_t: 0.0,
            last_t: 0.0,
            room: None,
            hit_t: None,
        }
    }
}
impl Default for Warn { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum WarnPhase {
    Heard,
    Called,
    Over,
}
impl WarnPhase {
    pub const fn as_str(self) -> &'static str {
        match self {
            WarnPhase::Heard => "heard",
            WarnPhase::Called => "called",
            WarnPhase::Over => "over",
        }
    }
}

/// (TypeScript: State, src/game/pressure.ts:277)
#[derive(Clone, Debug)]
pub struct State {
    pub presses: Vec<crate::pressure::Press>,
    pub of: JsMap<RoomfightDefId, PressureManId>,
    pub outs: Vec<crate::pressure::Out>,
    pub serial: f64,
    pub stale: bool,
    pub think: JsMap<RoomfightDefId, crate::pressure::StateThinkValue>,
    pub stay: JsMap<RoomfightDefId, RoomfightRoomId>,
    pub called: JsMap<RoomfightDefId, crate::pressure::StateCalledValue>,
    pub stood_t: f64,
    pub sent: JsSet<RoomfightRoomId>,
    pub done_at: JsMap<RoomfightDefId, f64>,
    pub last: JsMap<RoomfightDefId, f64>,
    pub tried: JsMap<RoomfightDefId, f64>,
    pub ear: JsMap<RoomfightDefId, crate::pressure::StateEarValue>,
    pub warn: Option<crate::pressure::Warn>,
    pub told: JsMap<RoomfightDefId, crate::pressure::StateToldValue>,
    pub heard: Vec<crate::pressure::StateHeardItem>,
    pub ev_seen: JsSet<u32>,
    pub marks: JsMap<DoorId, crate::pressure::StateMarksValue>,
    pub bangs: JsMap<(DoorId, crate::teams::Team), bool>,
    pub last_held: JsSet<RoomfightDefId>,
    pub word: Option<crate::pressure::StateWord>,
    pub swept: f64,
    pub look: Option<crate::pressure::StateLook>,
    pub out: Option<Vec<RoomfightDefId>>,
    pub look_lost: bool,
    pub look_t: f64,
    pub stats: crate::pressure::StateStats,
}
impl State {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        State {
            presses: Vec::new(),
            of: JsMap::new(),
            outs: Vec::new(),
            serial: 0.0,
            stale: false,
            think: JsMap::new(),
            stay: JsMap::new(),
            called: JsMap::new(),
            stood_t: 0.0,
            sent: JsSet::new(),
            done_at: JsMap::new(),
            last: JsMap::new(),
            tried: JsMap::new(),
            ear: JsMap::new(),
            warn: None,
            told: JsMap::new(),
            heard: Vec::new(),
            ev_seen: JsSet::new(),
            marks: JsMap::new(),
            bangs: JsMap::new(),
            last_held: JsSet::new(),
            word: None,
            swept: 0.0,
            look: None,
            out: None,
            look_lost: false,
            look_t: 0.0,
            stats: crate::pressure::StateStats::zeroed(),
        }
    }
}
impl Default for State { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/pressure.ts:281 State.think, src/game/pressure.ts:281)
#[derive(Clone, Copy, Debug)]
pub struct StateThinkValue {
    pub room: RoomfightRoomId,
    pub at: f64,
    pub held: Option<f64>,
}
impl StateThinkValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        StateThinkValue {
            room: RoomfightRoomId(0),
            at: 0.0,
            held: None,
        }
    }
}
impl Default for StateThinkValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/pressure.ts:281 State.called, src/game/pressure.ts:281)
#[derive(Clone, Copy, Debug)]
pub struct StateCalledValue {
    pub t: RoomfightRoomId,
    pub t_2: f64,
}
impl StateCalledValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        StateCalledValue {
            t: RoomfightRoomId(0),
            t_2: 0.0,
        }
    }
}
impl Default for StateCalledValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/pressure.ts:285 State.ear, src/game/pressure.ts:285)
#[derive(Clone, Copy, Debug)]
pub struct StateEarValue {
    pub t: f64,
    pub i: f64,
    pub at: f64,
}
impl StateEarValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        StateEarValue {
            t: 0.0,
            i: 0.0,
            at: 0.0,
        }
    }
}
impl Default for StateEarValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/pressure.ts:288 State.told, src/game/pressure.ts:288)
#[derive(Clone, Copy, Debug)]
pub struct StateToldValue {
    pub t: RoomfightRoomId,
    pub at: V3,
    pub t_2: f64,
}
impl StateToldValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        StateToldValue {
            t: RoomfightRoomId(0),
            at: [0.0; 3],
            t_2: 0.0,
        }
    }
}
impl Default for StateToldValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/pressure.ts:291 State.heard, src/game/pressure.ts:291)
#[derive(Clone, Copy, Debug)]
pub struct StateHeardItem {
    pub t: f64,
    pub p: V3,
}
impl StateHeardItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        StateHeardItem {
            t: 0.0,
            p: [0.0; 3],
        }
    }
}
impl Default for StateHeardItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/pressure.ts:291 State.marks, src/game/pressure.ts:291)
#[derive(Clone, Copy, Debug)]
pub struct StateMarksValue {
    pub r: Option<RoomfightRoomId>,
    pub team: crate::teams::Team,
    pub own: bool,
}
impl StateMarksValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        StateMarksValue {
            r: None,
            team: crate::teams::Team::Guard,
            own: false,
        }
    }
}
impl Default for StateMarksValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/pressure.ts:293 State.word, src/game/pressure.ts:293)
#[derive(Clone, Copy, Debug)]
pub struct StateWord {
    pub t: f64,
    pub p: V3,
}
impl StateWord {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        StateWord {
            t: 0.0,
            p: [0.0; 3],
        }
    }
}
impl Default for StateWord { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/pressure.ts:293 State.look, src/game/pressure.ts:293)
#[derive(Clone, Debug)]
pub struct StateLook {
    pub at: V3,
    pub go_t: f64,
    pub w: f64,
    pub men: Vec<RoomfightDefId>,
}
impl StateLook {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        StateLook {
            at: [0.0; 3],
            go_t: 0.0,
            w: 0.0,
            men: Vec::new(),
        }
    }
}
impl Default for StateLook { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/pressure.ts:294 State.stats, src/game/pressure.ts:251)
#[derive(Clone, Copy, Debug)]
pub struct StateStats {
    pub press_formed: f64,
    pub press_men: f64,
    pub press_sieges: f64,
    pub press_converge: f64,
    pub press_words: f64,
    pub press_lone: f64,
    pub press_arrived: f64,
    pub press_late: f64,
    pub press_landed: f64,
    pub press_cue_first: f64,
    pub press_heard: f64,
    pub press_lead: f64,
    pub flank_sent: f64,
    pub flank_set: f64,
    pub flank_arrived: f64,
    pub flank_late: f64,
    pub flank_landed: f64,
    pub flank_cue_first: f64,
    pub flank_heard: f64,
    pub flank_lead: f64,
    pub fb_push: f64,
    pub fb_last: f64,
    pub fb_early: f64,
    pub fb_planted: f64,
    pub fb_fought: f64,
    pub fb_told: f64,
    pub warn_calls: f64,
    pub warn_stood_down: f64,
    pub warn_hits: f64,
    pub warn_lead: f64,
    pub second_stood: f64,
    pub second_calls: f64,
    pub pairs_told: f64,
    pub pairs_waited: f64,
    pub pairs_went: f64,
    pub stun_marked: f64,
    pub stun_held: f64,
    pub stun_held_in: f64,
    pub stun_late: f64,
    pub stun_off: f64,
    pub stun_bangs: f64,
    pub stun_empty: f64,
    pub last_held: f64,
    pub look_called: f64,
    pub look_men: f64,
    pub look_off: f64,
}
impl StateStats {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        StateStats {
            press_formed: 0.0,
            press_men: 0.0,
            press_sieges: 0.0,
            press_converge: 0.0,
            press_words: 0.0,
            press_lone: 0.0,
            press_arrived: 0.0,
            press_late: 0.0,
            press_landed: 0.0,
            press_cue_first: 0.0,
            press_heard: 0.0,
            press_lead: 0.0,
            flank_sent: 0.0,
            flank_set: 0.0,
            flank_arrived: 0.0,
            flank_late: 0.0,
            flank_landed: 0.0,
            flank_cue_first: 0.0,
            flank_heard: 0.0,
            flank_lead: 0.0,
            fb_push: 0.0,
            fb_last: 0.0,
            fb_early: 0.0,
            fb_planted: 0.0,
            fb_fought: 0.0,
            fb_told: 0.0,
            warn_calls: 0.0,
            warn_stood_down: 0.0,
            warn_hits: 0.0,
            warn_lead: 0.0,
            second_stood: 0.0,
            second_calls: 0.0,
            pairs_told: 0.0,
            pairs_waited: 0.0,
            pairs_went: 0.0,
            stun_marked: 0.0,
            stun_held: 0.0,
            stun_held_in: 0.0,
            stun_late: 0.0,
            stun_off: 0.0,
            stun_bangs: 0.0,
            stun_empty: 0.0,
            last_held: 0.0,
            look_called: 0.0,
            look_men: 0.0,
            look_off: 0.0,
        }
    }
}
impl Default for StateStats { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/pressure.ts:223 PRESSURE_HOOKS, src/game/pressure.ts:223)
#[derive(Clone, Copy, Debug)]
pub struct PRESSUREHOOKS {
}
impl PRESSUREHOOKS {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PRESSUREHOOKS {
        }
    }
}
impl Default for PRESSUREHOOKS { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum HookedWhy {
    Push,
    Door,
    Seen,
}
impl HookedWhy {
    pub const fn as_str(self) -> &'static str {
        match self {
            HookedWhy::Push => "push",
            HookedWhy::Door => "door",
            HookedWhy::Seen => "seen",
        }
    }
}

/// (TypeScript: inline at src/game/pressure.ts:224 hooked ->, src/game/pressure.ts:224)
#[derive(Clone, Copy, Debug)]
pub struct Hooked {
    pub spot: V3,
    pub door: Option<DoorId>,
}
impl Hooked {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Hooked {
            spot: [0.0; 3],
            door: None,
        }
    }
}
impl Default for Hooked { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/pressure.ts:317 target ->, src/game/pressure.ts:317)
#[derive(Clone, Copy, Debug)]
pub struct Target {
    pub t: RoomfightRoomId,
    pub at: V3,
}
impl Target {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Target {
            t: RoomfightRoomId(0),
            at: [0.0; 3],
        }
    }
}
impl Default for Target { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/pressure.ts:418 gatherSpot ->, src/game/pressure.ts:418)
#[derive(Clone, Copy, Debug)]
pub struct GatherSpot {
    pub spot: V3,
    pub l: f64,
}
impl GatherSpot {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GatherSpot {
            spot: [0.0; 3],
            l: 0.0,
        }
    }
}
impl Default for GatherSpot { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/pressure.ts:441 standOffSpot ->, src/game/pressure.ts:441)
#[derive(Clone, Copy, Debug)]
pub struct StandOffSpot {
    pub spot: V3,
    pub l: f64,
}
impl StandOffSpot {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        StandOffSpot {
            spot: [0.0; 3],
            l: 0.0,
        }
    }
}
impl Default for StandOffSpot { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/pressure.ts:522 teamPlaced ->, src/game/pressure.ts:522)
#[derive(Clone, Copy, Debug)]
pub struct TeamPlaced {
    pub known: bool,
    pub moved: bool,
}
impl TeamPlaced {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        TeamPlaced {
            known: false,
            moved: false,
        }
    }
}
impl Default for TeamPlaced { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/pressure.ts:935 aim ->, src/game/pressure.ts:935)
#[derive(Clone, Copy, Debug)]
pub struct Aim {
    pub t: RoomfightRoomId,
    pub at: V3,
}
impl Aim {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Aim {
            t: RoomfightRoomId(0),
            at: [0.0; 3],
        }
    }
}
impl Default for Aim { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct PressureMod {
    /// (src/game/pressure.ts:296 STATES)
    pub states: Option<crate::pressure::State>,
    /// (src/game/pressure.ts:303 KITTED)
    pub kitted: JsSet<GuardId>,
    /// (src/game/pressure.ts:333 PICKETS)
    pub pickets: JsMap<GuardId, RoomfightRoomId>,
    /// (src/game/pressure.ts:333 PICKET_WENT)
    pub picket_went: JsSet<GuardId>,
}
impl PressureMod {
    pub fn blank() -> Self {
        PressureMod {
            states: None,
            kitted: JsSet::new(),
            pickets: JsMap::new(),
            picket_went: JsSet::new(),
        }
    }
}

/// (src/game/pressure.ts:223 PRESSURE_HOOKS: a table nothing writes)
pub fn pressure_hooks() -> crate::pressure::PRESSUREHOOKS { crate::pressure::PRESSUREHOOKS {  } }

/// (src/game/pressure.ts:304 ROOM_CELLS)
pub const ROOM_CELLS: f64 = 500.0;

/// (src/game/pressure.ts:305 STAND_OFF_BAND)
pub const STAND_OFF_BAND: f64 = 7.0;

/// (src/game/pressure.ts:305 STAND_OFF_TRIES)
pub const STAND_OFF_TRIES: f64 = 10.0;

/// (src/game/pressure.ts:306 PAIR_WAIT_SECS)
pub const PAIR_WAIT_SECS: f64 = 6.0;

/// (src/game/pressure.ts:307 CUE_FIRST_SECS)
pub const CUE_FIRST_SECS: f64 = 1.0;

/// (src/game/pressure.ts:334 PICKET_DOOR_M)
pub const PICKET_DOOR_M: f64 = 2.5;

/// (src/game/pressure.ts:994 CALLED_SECS)
pub const CALLED_SECS: f64 = 30.0;

/// (src/game/pressure.ts:1097 KEPT: a table nothing writes)
pub fn kept() -> [String; 21] { [String::from("on"), String::from("press"), String::from("flank"), String::from("fallBack"), String::from("second"), String::from("secondKeeps"), String::from("secondSet"), String::from("maxMen"), String::from("keepRoom"), String::from("keeperLeft"), String::from("organiseK"), String::from("gather"), String::from("warned"), String::from("standPress"), String::from("picket"), String::from("byWay"), String::from("wayOut"), String::from("rescueCrew"), String::from("drilled"), String::from("squadDrill"), String::from("sweep")] }

/// (src/game/pressure.ts:1098 LS)
pub const LS: &str = "twopointsix.pressure";

/// (src/game/pressure.ts:1099 PRESSURE_DEFAULTS: a table nothing writes)
pub fn pressure_defaults() -> crate::knobs::PressureK { crate::knobs::page_pressure() }
