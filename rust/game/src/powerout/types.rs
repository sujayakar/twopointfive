//! The types of `powerout`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: Calm, src/game/powerout.ts:94)
#[derive(Clone, Debug)]
pub struct Calm {
    pub awareness: f64,
    pub hold: bool,
    pub pinned: bool,
    pub station: V3,
    pub home: V3,
    pub room: Option<RoomfightRoomId>,
    pub ev: Vec<f64>,
    pub watch: f64,
    pub want: f64,
    pub pt: V3,
}
impl Calm {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Calm {
            awareness: 0.0,
            hold: false,
            pinned: false,
            station: [0.0; 3],
            home: [0.0; 3],
            room: None,
            ev: Vec::new(),
            watch: 0.0,
            want: 0.0,
            pt: [0.0; 3],
        }
    }
}
impl Default for Calm { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Phase {
    Out,
    Back,
    Wait,
    There,
}
impl Phase {
    pub const fn as_str(self) -> &'static str {
        match self {
            Phase::Out => "out",
            Phase::Back => "back",
            Phase::Wait => "wait",
            Phase::There => "there",
        }
    }
}

/// (TypeScript: Man, src/game/powerout.ts:96)
#[derive(Clone, Debug)]
pub struct Man {
    pub gd: GuardId,
    pub role: crate::powerout::ManRole,
    pub phase: crate::powerout::Phase,
    pub goal: V3,
    pub from: V3,
    pub watch: Option<V3>,
    pub tries: f64,
    pub step_t: f64,
    pub heard0: f64,
    pub there_t: f64,
    pub calm: Option<crate::powerout::Calm>,
}
impl Man {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Man {
            gd: GuardId(0),
            role: crate::powerout::ManRole::Cover,
            phase: crate::powerout::Phase::Out,
            goal: [0.0; 3],
            from: [0.0; 3],
            watch: None,
            tries: 0.0,
            step_t: 0.0,
            heard0: 0.0,
            there_t: 0.0,
            calm: None,
        }
    }
}
impl Default for Man { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum ManRole {
    Cover,
    Lever,
}
impl ManRole {
    pub const fn as_str(self) -> &'static str {
        match self {
            ManRole::Cover => "cover",
            ManRole::Lever => "lever",
        }
    }
}

/// (TypeScript: PowerOutEvent, src/game/powerout.ts:97)
#[derive(Clone, Debug)]
pub struct PowerOutEvent {
    pub t: f64,
    pub what: crate::powerout::PowerOutEventWhat,
    pub who: String,
    pub why: String,
}
impl PowerOutEvent {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PowerOutEvent {
            t: 0.0,
            what: crate::powerout::PowerOutEventWhat::Stay,
            who: String::new(),
            why: String::new(),
        }
    }
}
impl Default for PowerOutEvent { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum PowerOutEventWhat {
    Stay,
    Lost,
    Home,
    Go,
    There,
    Said,
    Noticed,
    Nobody,
    Abort,
    Wrecked,
    Restored,
    Overdue,
}
impl PowerOutEventWhat {
    pub const fn as_str(self) -> &'static str {
        match self {
            PowerOutEventWhat::Stay => "stay",
            PowerOutEventWhat::Lost => "lost",
            PowerOutEventWhat::Home => "home",
            PowerOutEventWhat::Go => "go",
            PowerOutEventWhat::There => "there",
            PowerOutEventWhat::Said => "said",
            PowerOutEventWhat::Noticed => "noticed",
            PowerOutEventWhat::Nobody => "nobody",
            PowerOutEventWhat::Abort => "abort",
            PowerOutEventWhat::Wrecked => "wrecked",
            PowerOutEventWhat::Restored => "restored",
            PowerOutEventWhat::Overdue => "overdue",
        }
    }
}

/// (TypeScript: State, src/game/powerout.ts:98)
#[derive(Clone, Debug)]
pub struct State {
    pub since: f64,
    pub decide_at: f64,
    pub go_at: f64,
    pub way_m: f64,
    pub overdue_at: f64,
    pub fell: Option<V3>,
    pub seen: Vec<crate::powerout::StateSeenItem>,
    pub look_at: f64,
    pub men: Vec<crate::powerout::Man>,
    pub decided: bool,
    pub word: bool,
    pub parties: f64,
    pub lost: bool,
    pub events: Vec<crate::powerout::PowerOutEvent>,
}
impl State {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        State {
            since: 0.0,
            decide_at: 0.0,
            go_at: 0.0,
            way_m: 0.0,
            overdue_at: 0.0,
            fell: None,
            seen: Vec::new(),
            look_at: 0.0,
            men: Vec::new(),
            decided: false,
            word: false,
            parties: 0.0,
            lost: false,
            events: Vec::new(),
        }
    }
}
impl Default for State { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/powerout.ts:106 State.seen, src/game/powerout.ts:106)
#[derive(Clone, Copy, Debug)]
pub struct StateSeenItem {
    pub i: f64,
    pub lit: f64,
}
impl StateSeenItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        StateSeenItem {
            i: 0.0,
            lit: 0.0,
        }
    }
}
impl Default for StateSeenItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: CalmSnap, src/game/powerout.ts:222)
#[derive(Clone, Debug)]
pub struct CalmSnap {
    pub hold: bool,
    pub home: V3,
    pub watch: f64,
    pub awareness: f64,
    pub pinned: bool,
    pub station: V3,
    pub ev: Vec<f64>,
    pub want: f64,
    pub pt: V3,
    pub room: f64,
}
impl CalmSnap {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CalmSnap {
            hold: false,
            home: [0.0; 3],
            watch: 0.0,
            awareness: 0.0,
            pinned: false,
            station: [0.0; 3],
            ev: Vec::new(),
            want: 0.0,
            pt: [0.0; 3],
            room: 0.0,
        }
    }
}
impl Default for CalmSnap { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: PowerOutSnap, src/game/powerout.ts:403)
#[derive(Clone, Debug)]
pub struct PowerOutSnap {
    pub since: f64,
    pub decide_at: f64,
    pub go_at: f64,
    pub way_m: f64,
    pub overdue_at: f64,
    pub fell: Option<V3>,
    pub seen: Vec<crate::powerout::PowerOutSnapSeenItem>,
    pub look_at: f64,
    pub decided: bool,
    pub word: bool,
    pub parties: f64,
    pub lost: bool,
    pub men: Vec<crate::powerout::PowerOutSnapMenItem>,
}
impl PowerOutSnap {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PowerOutSnap {
            since: 0.0,
            decide_at: 0.0,
            go_at: 0.0,
            way_m: 0.0,
            overdue_at: 0.0,
            fell: None,
            seen: Vec::new(),
            look_at: 0.0,
            decided: false,
            word: false,
            parties: 0.0,
            lost: false,
            men: Vec::new(),
        }
    }
}
impl Default for PowerOutSnap { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/powerout.ts:403 PowerOutSnap.seen, src/game/powerout.ts:403)
#[derive(Clone, Copy, Debug)]
pub struct PowerOutSnapSeenItem {
    pub i: f64,
    pub lit: f64,
}
impl PowerOutSnapSeenItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PowerOutSnapSeenItem {
            i: 0.0,
            lit: 0.0,
        }
    }
}
impl Default for PowerOutSnapSeenItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/powerout.ts:404 PowerOutSnap.men, src/game/powerout.ts:404)
#[derive(Clone, Debug)]
pub struct PowerOutSnapMenItem {
    pub i: f64,
    pub role: crate::powerout::PowerOutSnapMenItemRole,
    pub phase: crate::powerout::Phase,
    pub goal: V3,
    pub from: V3,
    pub watch: Option<V3>,
    pub tries: f64,
    pub step_t: f64,
    pub heard0: f64,
    pub there_t: f64,
    pub calm: Option<crate::powerout::CalmSnap>,
}
impl PowerOutSnapMenItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PowerOutSnapMenItem {
            i: 0.0,
            role: crate::powerout::PowerOutSnapMenItemRole::Cover,
            phase: crate::powerout::Phase::Out,
            goal: [0.0; 3],
            from: [0.0; 3],
            watch: None,
            tries: 0.0,
            step_t: 0.0,
            heard0: 0.0,
            there_t: 0.0,
            calm: None,
        }
    }
}
impl Default for PowerOutSnapMenItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum PowerOutSnapMenItemRole {
    Cover,
    Lever,
}
impl PowerOutSnapMenItemRole {
    pub const fn as_str(self) -> &'static str {
        match self {
            PowerOutSnapMenItemRole::Cover => "cover",
            PowerOutSnapMenItemRole::Lever => "lever",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum EventWhat {
    Stay,
    Lost,
    Home,
    Go,
    There,
    Said,
    Noticed,
    Nobody,
    Abort,
    Wrecked,
    Restored,
    Overdue,
}
impl EventWhat {
    pub const fn as_str(self) -> &'static str {
        match self {
            EventWhat::Stay => "stay",
            EventWhat::Lost => "lost",
            EventWhat::Home => "home",
            EventWhat::Go => "go",
            EventWhat::There => "there",
            EventWhat::Said => "said",
            EventWhat::Noticed => "noticed",
            EventWhat::Nobody => "nobody",
            EventWhat::Abort => "abort",
            EventWhat::Wrecked => "wrecked",
            EventWhat::Restored => "restored",
            EventWhat::Overdue => "overdue",
        }
    }
}

/// (TypeScript: inline at src/game/powerout.ts:183 wayTo ->, src/game/powerout.ts:183)
#[derive(Clone, Debug)]
pub struct WayTo {
    pub path: Vec<V3>,
    pub l: f64,
}
impl WayTo {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WayTo {
            path: Vec::new(),
            l: 0.0,
        }
    }
}
impl Default for WayTo { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/powerout.ts:190 coverSpot ->, src/game/powerout.ts:190)
#[derive(Clone, Copy, Debug)]
pub struct CoverSpot {
    pub spot: V3,
    pub watch: V3,
}
impl CoverSpot {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CoverSpot {
            spot: [0.0; 3],
            watch: [0.0; 3],
        }
    }
}
impl Default for CoverSpot { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct PoweroutMod {
    /// (src/game/powerout.ts:112 STATES)
    pub states: Option<crate::powerout::State>,
}
impl PoweroutMod {
    pub fn blank() -> Self {
        PoweroutMod {
            states: None,
        }
    }
}
