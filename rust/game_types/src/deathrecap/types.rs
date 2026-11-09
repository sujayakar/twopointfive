//! The types of `deathrecap`: every death explains itself (rust/game_rules/src/deathrecap).
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{collections::VecDeque, string::String, vec::Vec};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes)]
pub enum RecapCause {
    Blast,
    Round,
    Other,
    Unknown,
}
impl RecapCause {
    pub const fn as_str(self) -> &'static str {
        match self {
            RecapCause::Blast => "blast",
            RecapCause::Round => "round",
            RecapCause::Other => "other",
            RecapCause::Unknown => "unknown",
        }
    }
}

/// one man of a run, as a take names him
#[derive(Clone, Debug, ToBytes)]
pub struct RecapMan {
    pub id: f64,
    pub name: String,
    pub side: String,
    pub kind: crate::deathrecap::RecapManKind,
}
impl RecapMan {
    pub fn zeroed() -> Self {
        RecapMan {
            id: 0.0,
            name: String::new(),
            side: String::new(),
            kind: crate::deathrecap::RecapManKind::Player,
        }
    }
}
impl Default for RecapMan { fn default() -> Self { Self::zeroed() } }

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes)]
pub enum RecapManKind {
    Player,
    Civ,
    Man,
}
impl RecapManKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            RecapManKind::Player => "player",
            RecapManKind::Civ => "civ",
            RecapManKind::Man => "man",
        }
    }
}

/// One of ours going down. Plan coordinates [x, z], metres; `gaze` in radians, 0 = +z (the aim yaw he had the sample before he fell).
#[derive(Clone, Debug, ToBytes)]
pub struct RecapDeath {
    pub t: f64,
    pub victim: f64,
    pub name: String,
    pub side: String,
    pub player: bool,
    pub at: [f64; 2],
    pub gaze: f64,
    pub cause: crate::deathrecap::RecapCause,
    pub by: f64,
    pub by_name: String,
    pub by_side: String,
    pub by_at: Option<[f64; 2]>,
    pub from: Option<[f64; 2]>,
    pub to: [f64; 2],
    pub weapon: String,
    pub dist: f64,
    pub off: f64,
    pub blind: bool,
    pub loud: bool,
    /// this death's number in the run, the record's own (a take is of one death, asked by this): dealt by the record and not by `g.serial()`, which the
    /// recap may not move
    pub serial: u32,
}
impl RecapDeath {
    pub fn zeroed() -> Self {
        RecapDeath {
            t: 0.0,
            victim: 0.0,
            name: String::new(),
            side: String::new(),
            player: false,
            at: [0.0; 2],
            gaze: 0.0,
            cause: crate::deathrecap::RecapCause::Blast,
            by: 0.0,
            by_name: String::new(),
            by_side: String::new(),
            by_at: None,
            from: None,
            to: [0.0; 2],
            weapon: String::new(),
            dist: 0.0,
            off: 0.0,
            blind: false,
            loud: false,
            serial: 0,
        }
    }
}
impl Default for RecapDeath { fn default() -> Self { Self::zeroed() } }

/// a round, or a burst. `at` = the shooter's feet
#[derive(Clone, Debug, ToBytes)]
pub struct RecapShot {
    pub t: f64,
    pub kind: crate::deathrecap::RecapShotKind,
    pub who: f64,
    pub from: [f64; 2],
    pub to: Option<[f64; 2]>,
    pub at: Option<[f64; 2]>,
    pub weapon: Option<String>,
    pub loud: Option<bool>,
}
impl RecapShot {
    pub fn zeroed() -> Self {
        RecapShot {
            t: 0.0,
            kind: crate::deathrecap::RecapShotKind::Blast,
            who: 0.0,
            from: [0.0; 2],
            to: None,
            at: None,
            weapon: None,
            loud: None,
        }
    }
}
impl Default for RecapShot { fn default() -> Self { Self::zeroed() } }

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes)]
pub enum RecapShotKind {
    Blast,
    Bang,
    Round,
}
impl RecapShotKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            RecapShotKind::Blast => "blast",
            RecapShotKind::Bang => "bang",
            RecapShotKind::Round => "round",
        }
    }
}

/// One sample of the floor. men: MAN_STRIDE floats a man = id, x, z, aim yaw, bits (1 down, 2 roused: alert or searching, 4 crouched). doors: 2 a door = the
/// leaf's direction (NaN = off its hinges).
#[derive(Clone, Debug, ToBytes)]
pub struct RecapFrame {
    pub t: f64,
    pub n: f64,
    pub men: Vec<f32>,
    pub doors: Vec<f32>,
}
impl RecapFrame {
    pub fn zeroed() -> Self {
        RecapFrame {
            t: 0.0,
            n: 0.0,
            men: Vec::new(),
            doors: Vec::new(),
        }
    }
}
impl Default for RecapFrame { fn default() -> Self { Self::zeroed() } }

/// A death with its own last seconds: frames from `t0` to `t1` (the death at `death.t`), the rounds and blasts in them, every one of ours lost in them.
#[derive(Clone, Debug, ToBytes)]
pub struct RecapTake {
    pub death: crate::deathrecap::RecapDeath,
    pub t0: f64,
    pub t1: f64,
    pub cast: Vec<crate::deathrecap::RecapMan>,
    pub doors: Vec<crate::deathrecap::RecapDoor>,
    pub frames: Vec<crate::deathrecap::RecapFrame>,
    pub shots: Vec<crate::deathrecap::RecapShot>,
    pub deaths: Vec<crate::deathrecap::RecapDeath>,
}
impl RecapTake {
    pub fn zeroed() -> Self {
        RecapTake {
            death: crate::deathrecap::RecapDeath::zeroed(),
            t0: 0.0,
            t1: 0.0,
            cast: Vec::new(),
            doors: Vec::new(),
            frames: Vec::new(),
            shots: Vec::new(),
            deaths: Vec::new(),
        }
    }
}
impl Default for RecapTake { fn default() -> Self { Self::zeroed() } }

/// where a door hangs
#[derive(Clone, Copy, Debug, ToBytes)]
pub struct RecapDoor {
    pub hinge: [f64; 2],
    pub width: f64,
}
impl RecapDoor {
    pub fn zeroed() -> Self {
        RecapDoor {
            hinge: [0.0; 2],
            width: 0.0,
        }
    }
}
impl Default for RecapDoor { fn default() -> Self { Self::zeroed() } }

/// One of the squad lost while the run is on, as much as the squad knows: `from` = the way off him to where the round (the blast) came from, a unit vector on
/// the plan to the nearest EIGHTH of the compass: a direction, never a place, and never fine enough to lay a ruler along. None until the squad has it
/// (`known_at`), and for good when nobody still up saw him go down or heard it.
#[derive(Clone, Debug, ToBytes)]
pub struct LostMan {
    pub id: f64,
    pub name: String,
    pub t: f64,
    pub at: [f64; 2],
    pub from: Option<[f64; 2]>,
    pub known_at: f64,
}
impl LostMan {
    pub fn zeroed() -> Self {
        LostMan {
            id: 0.0,
            name: String::new(),
            t: 0.0,
            at: [0.0; 2],
            from: None,
            known_at: 0.0,
        }
    }
}
impl Default for LostMan { fn default() -> Self { Self::zeroed() } }

/// What the record keeps of a man lost beside `Recap.lost_live[i]`, the same man as the squad knows him: the way it came from (handed over as `from` only when the
/// squad has it), whether ears had it, who had a line to him as he went down (the first teammate; every player), and `done` once that is settled either way.
#[derive(Clone, Debug, ToBytes)]
pub struct Lost {
    pub src: Option<[f64; 2]>,
    pub heard: bool,
    pub saw: Option<GuardId>,
    pub players_saw: Vec<PlayerId>,
    pub done: bool,
}

/// the last round into a man who was up
#[derive(Clone, Debug, ToBytes)]
pub struct Round {
    pub t: f64,
    pub by: f64,
    pub by_at: [f64; 2],
    pub from: [f64; 2],
    pub to: [f64; 2],
    pub weapon: String,
    pub blind: bool,
    pub loud: bool,
}
impl Round {
    pub fn zeroed() -> Self {
        Round {
            t: 0.0,
            by: 0.0,
            by_at: [0.0; 2],
            from: [0.0; 2],
            to: [0.0; 2],
            weapon: String::new(),
            blind: false,
            loud: false,
        }
    }
}
impl Default for Round { fn default() -> Self { Self::zeroed() } }

/// a death whose last second has not run yet, and when it will have
#[derive(Clone, Debug, ToBytes)]
pub struct RecapDue {
    pub death: crate::deathrecap::RecapDeath,
    pub at: f64,
}

/// What the end card may show: the takes, and the one it opens on (the last player to go down, else the last of ours).
#[derive(Clone, Debug, Default, ToBytes)]
pub struct DeathRecap {
    pub takes: Vec<crate::deathrecap::RecapTake>,
    pub main: f64,
}

/// THE RECAP'S RECORD: the game's own, written by the tick (`deathrecap::recap_tick`) and by a round landing (`recap_round`), fresh with every mission. Only
/// `lost_live` is on the wire: the rest is handed out once, to the end card. A kept game has none of it (`recap_left_behind`), so a load starts it over: a load is a run
/// in which that death has not happened.
#[derive(Clone, Debug, ToBytes)]
pub struct Recap {
    /// the mission this record is of, by its serial
    pub run: Option<u32>,
    /// every player is down (written every tick, for the HUD: the find-me outline)
    pub all_down: bool,
    pub last_t: f64,
    /// when the next sample of the floor is due
    pub next: f64,
    /// the tick has looked at the floor: who is down from now on went down in front of it (not a body a checkpoint brought back)
    pub primed: bool,
    /// when the run ended (infinity while it has not): what the card may show of a run that is played on past its end stops there
    pub over_at: f64,
    pub frames: VecDeque<crate::deathrecap::RecapFrame>,
    pub shots: VecDeque<crate::deathrecap::RecapShot>,
    /// the last event read, by its serial (serials rise along the game's list of events)
    pub seen_to: u32,
    pub down: JsSet<crate::Num>,
    /// the last round into each man, by his character's id
    pub rounds: JsMap<crate::Num, crate::deathrecap::Round>,
    pub cast: JsMap<crate::Num, crate::deathrecap::RecapMan>,
    pub deaths: VecDeque<crate::deathrecap::RecapDeath>,
    /// how many deaths this record has dealt a number to
    pub deaths_dealt: u32,
    pub due: Vec<crate::deathrecap::RecapDue>,
    pub takes: Vec<crate::deathrecap::RecapTake>,
    /// the squad's men lost so far this run, as the squad knows them: what a HUD may read while the run is on
    pub lost_live: Vec<crate::deathrecap::LostMan>,
    /// the same men by the same index: what the squad does not know
    pub lost: Vec<crate::deathrecap::Lost>,
    pub lost_next: f64,
}
impl Recap {
    /// a mission's record before the tick has looked at it
    pub fn blank() -> Self {
        Recap {
            run: None,
            all_down: false,
            last_t: 0.0,
            next: 0.0,
            primed: false,
            over_at: f64::INFINITY,
            frames: VecDeque::new(),
            shots: VecDeque::new(),
            seen_to: 0,
            down: JsSet::new(),
            rounds: JsMap::new(),
            cast: JsMap::new(),
            deaths: VecDeque::new(),
            deaths_dealt: 0,
            due: Vec::new(),
            takes: Vec::new(),
            lost_live: Vec::new(),
            lost: Vec::new(),
            lost_next: 0.0,
        }
    }
}
impl Default for Recap { fn default() -> Self { Self::blank() } }

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct DeathrecapStats {
    pub deaths: i32,
    pub takes: i32,
    pub rounds: i32,
    pub known: i32,
    pub calls: i32,
}

/// floats a man in a frame
pub const MAN_STRIDE: usize = 5;
