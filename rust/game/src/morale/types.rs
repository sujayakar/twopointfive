//! The types of `morale`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Training {
    None,
    Trained,
    Difficulty,
    Green,
    Mixed,
}
impl Training {
    pub const fn as_str(self) -> &'static str {
        match self {
            Training::None => "none",
            Training::Trained => "trained",
            Training::Difficulty => "difficulty",
            Training::Green => "green",
            Training::Mixed => "mixed",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Nerve {
    Steady,
    Normal,
    Brittle,
}
impl Nerve {
    pub const fn as_str(self) -> &'static str {
        match self {
            Nerve::Steady => "steady",
            Nerve::Normal => "normal",
            Nerve::Brittle => "brittle",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum MoraleAct {
    Down,
    Reload,
    Burst,
    None,
    Back,
    Hold,
    Stand,
    Flinch,
    Duck,
    Bait,
}
impl MoraleAct {
    pub const fn as_str(self) -> &'static str {
        match self {
            MoraleAct::Down => "down",
            MoraleAct::Reload => "reload",
            MoraleAct::Burst => "burst",
            MoraleAct::None => "none",
            MoraleAct::Back => "back",
            MoraleAct::Hold => "hold",
            MoraleAct::Stand => "stand",
            MoraleAct::Flinch => "flinch",
            MoraleAct::Duck => "duck",
            MoraleAct::Bait => "bait",
        }
    }
}

/// (TypeScript: Mind, src/game/morale.ts:124)
#[derive(Clone, Copy, Debug)]
pub struct Mind {
    pub shock: f64,
    pub alone: bool,
    pub dark: bool,
    pub lit: f64,
    pub ask: f64,
    pub rung: f64,
    pub green: bool,
    pub leader_down: bool,
    pub fall: f64,
    pub lost: bool,
    pub by: Option<GuardId>,
    pub heard: f64,
    pub said: bool,
    pub broke_t: f64,
    pub act: crate::morale::MoraleAct,
    pub t: f64,
    pub hole: Option<V3>,
    pub goal: Option<V3>,
    pub rounds: f64,
    pub next: f64,
    pub calls: f64,
    pub call_t: f64,
    pub baits: f64,
    pub at_hole: bool,
    pub bark: f64,
    pub shout: bool,
    pub drive: bool,
    pub drive_t: f64,
    pub script: crate::game::GuardScript,
}
impl Mind {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Mind {
            shock: 0.0,
            alone: false,
            dark: false,
            lit: 0.0,
            ask: 0.0,
            rung: 0.0,
            green: false,
            leader_down: false,
            fall: 0.0,
            lost: false,
            by: None,
            heard: 0.0,
            said: false,
            broke_t: 0.0,
            act: crate::morale::MoraleAct::Down,
            t: 0.0,
            hole: None,
            goal: None,
            rounds: 0.0,
            next: 0.0,
            calls: 0.0,
            call_t: 0.0,
            baits: 0.0,
            at_hole: false,
            bark: 0.0,
            shout: false,
            drive: false,
            drive_t: 0.0,
            script: crate::game::GuardScript::zeroed(),
        }
    }
}
impl Default for Mind { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Blast, src/game/morale.ts:129)
#[derive(Clone, Copy, Debug)]
pub struct Blast {
    pub at: V3,
    pub t: f64,
    pub who: f64,
}
impl Blast {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Blast {
            at: [0.0; 3],
            t: 0.0,
            who: 0.0,
        }
    }
}
impl Default for Blast { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum LineKind {
    Clear,
    Out,
    Lost,
    Spent,
    Order,
    Call,
    Again,
    Duck,
    Okay,
}
impl LineKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            LineKind::Clear => "clear",
            LineKind::Out => "out",
            LineKind::Lost => "lost",
            LineKind::Spent => "spent",
            LineKind::Order => "order",
            LineKind::Call => "call",
            LineKind::Again => "again",
            LineKind::Duck => "duck",
            LineKind::Okay => "okay",
        }
    }
}

/// (TypeScript: Kept, src/game/morale.ts:565)
#[derive(Clone, Copy, Debug)]
pub struct Kept {
    pub on: Option<bool>,
    pub training: Option<crate::morale::Training>,
    pub nerve: Option<crate::morale::Nerve>,
}
impl Kept {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Kept {
            on: None,
            training: None,
            nerve: None,
        }
    }
}
impl Default for Kept { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: MoraleSnap, src/game/morale.ts:583)
#[derive(Clone, Debug)]
pub struct MoraleSnap {
    pub men: Vec<Option<Vec<f64>>>,
    pub holes: Vec<Option<Vec<f64>>>,
    pub blasts: Vec<Vec<f64>>,
}
impl MoraleSnap {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MoraleSnap {
            men: Vec::new(),
            holes: Vec::new(),
            blasts: Vec::new(),
        }
    }
}
impl Default for MoraleSnap { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/morale.ts:110 MORALE_LOG, src/game/morale.ts:110)
#[derive(Clone, Copy, Debug)]
pub struct MoraleModMORALELOGItem {
    pub id: f64,
    pub t: f64,
    pub from: V3,
    pub at: V3,
    pub of: crate::morale::MoraleModMORALELOGItemOf,
}
impl MoraleModMORALELOGItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MoraleModMORALELOGItem {
            id: 0.0,
            t: 0.0,
            from: [0.0; 3],
            at: [0.0; 3],
            of: crate::morale::MoraleModMORALELOGItemOf::Dust,
        }
    }
}
impl Default for MoraleModMORALELOGItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum MoraleModMORALELOGItemOf {
    Dust,
    Bait,
}
impl MoraleModMORALELOGItemOf {
    pub const fn as_str(self) -> &'static str {
        match self {
            MoraleModMORALELOGItemOf::Dust => "dust",
            MoraleModMORALELOGItemOf::Bait => "bait",
        }
    }
}

/// (TypeScript: inline at src/game/morale.ts:140 LINES, src/game/morale.ts:140)
#[derive(Clone, Debug)]
pub struct LINESValue {
    pub say: Vec<String>,
    pub he: Option<Vec<String>>,
    pub they: Option<Vec<String>>,
    pub dark: Option<Vec<String>>,
    pub lit: Option<Vec<String>>,
    pub now: Option<Vec<String>>,
}
impl LINESValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LINESValue {
            say: Vec::new(),
            he: None,
            they: None,
            dark: None,
            lit: None,
            now: None,
        }
    }
}
impl Default for LINESValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/morale.ts:217 mixAt, src/game/morale.ts:217)
#[derive(Clone, Debug)]
pub struct MoraleModMixAtValue {
    pub key: String,
    pub mix: crate::morale::MoraleModMixAtValueMix,
}
impl MoraleModMixAtValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MoraleModMixAtValue {
            key: String::new(),
            mix: crate::morale::MoraleModMixAtValueMix::Hard,
        }
    }
}
impl Default for MoraleModMixAtValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum MoraleModMixAtValueMix {
    Hard,
    Mixed,
}
impl MoraleModMixAtValueMix {
    pub const fn as_str(self) -> &'static str {
        match self {
            MoraleModMixAtValueMix::Hard => "hard",
            MoraleModMixAtValueMix::Mixed => "mixed",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum MixNow {
    Hard,
    Mixed,
}
impl MixNow {
    pub const fn as_str(self) -> &'static str {
        match self {
            MixNow::Hard => "hard",
            MixNow::Mixed => "mixed",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum TrainingOf {
    Steady,
    Green,
}
impl TrainingOf {
    pub const fn as_str(self) -> &'static str {
        match self {
            TrainingOf::Steady => "steady",
            TrainingOf::Green => "green",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum RoundOf {
    Dust,
    Bait,
}
impl RoundOf {
    pub const fn as_str(self) -> &'static str {
        match self {
            RoundOf::Dust => "dust",
            RoundOf::Bait => "bait",
        }
    }
}

/// (TypeScript: inline at src/game/morale.ts:545 moraleOf ->, ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1597)
#[derive(Clone, Copy, Debug)]
pub struct MoraleOf {
    pub drive: bool,
    pub heard: f64,
    pub t: f64,
    pub lost: bool,
    pub next: f64,
    pub hole: Option<V3>,
    pub goal: Option<V3>,
    pub act: crate::morale::MoraleAct,
    pub lit: f64,
    pub said: bool,
    pub alone: bool,
    pub ask: f64,
    pub by: Option<GuardId>,
    pub green: bool,
    pub dark: bool,
    pub rounds: f64,
    pub fall: f64,
    pub drive_t: f64,
    pub calls: f64,
    pub shock: f64,
    pub rung: f64,
    pub leader_down: bool,
    pub broke_t: f64,
    pub call_t: f64,
    pub baits: f64,
    pub at_hole: bool,
    pub bark: f64,
    pub shout: bool,
}
impl MoraleOf {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        MoraleOf {
            drive: false,
            heard: 0.0,
            t: 0.0,
            lost: false,
            next: 0.0,
            hole: None,
            goal: None,
            act: crate::morale::MoraleAct::Down,
            lit: 0.0,
            said: false,
            alone: false,
            ask: 0.0,
            by: None,
            green: false,
            dark: false,
            rounds: 0.0,
            fall: 0.0,
            drive_t: 0.0,
            calls: 0.0,
            shock: 0.0,
            rung: 0.0,
            leader_down: false,
            broke_t: 0.0,
            call_t: 0.0,
            baits: 0.0,
            at_hole: false,
            bark: 0.0,
            shout: false,
        }
    }
}
impl Default for MoraleOf { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum MoraleKeepRow {
    On,
    Training,
    Nerve,
}
impl MoraleKeepRow {
    pub const fn as_str(self) -> &'static str {
        match self {
            MoraleKeepRow::On => "on",
            MoraleKeepRow::Training => "training",
            MoraleKeepRow::Nerve => "nerve",
        }
    }
}

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct MoraleMod {
    /// (src/game/morale.ts:110 MORALE_LOG)
    pub morale_log: Vec<crate::morale::MoraleModMORALELOGItem>,
    /// (src/game/morale.ts:127 stateOf)
    pub state_of: JsMap<GuardId, crate::morale::Mind>,
    /// (src/game/morale.ts:130 blastsOf)
    pub blasts_of: Option<Vec<crate::morale::Blast>>,
    /// (src/game/morale.ts:217 mixAt)
    pub mix_at: Option<crate::morale::MoraleModMixAtValue>,
}
impl MoraleMod {
    pub fn blank() -> Self {
        MoraleMod {
            morale_log: Vec::new(),
            state_of: JsMap::new(),
            blasts_of: None,
            mix_at: None,
        }
    }
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct MoraleStats {
    pub blasts: i32,
    pub fed: i32,
    pub ducks: i32,
    pub broke: i32,
    pub rounds: i32,
    pub calls: i32,
    pub shouts: i32,
    pub baits: i32,
    pub reloads: i32,
    pub orders: i32,
    pub falls: i32,
    pub flinches: i32,
}

/// (src/game/morale.ts:99 TRAININGS: a table nothing writes)
pub fn trainings() -> Vec<String> { alloc::vec![String::from("difficulty"), String::from("green"), String::from("mixed"), String::from("trained")] }

/// (src/game/morale.ts:101 NERVES: a table nothing writes)
pub fn nerves() -> Vec<String> { alloc::vec![String::from("brittle"), String::from("normal"), String::from("steady")] }

/// (src/game/morale.ts:105 MORALE_AT: a table nothing writes)
pub fn morale_at() -> JsMap<String, String> { { let mut t = JsMap::new(); t.set(String::from("standard"), String::from("mixed")); t.set(String::from("hard"), String::from("hard")); t } }

/// (src/game/morale.ts:107 ALERT_AW)
pub const ALERT_AW: f64 = 0.85;

/// (src/game/morale.ts:115 ACTS: a table nothing writes)
pub fn acts() -> Vec<String> { alloc::vec![String::from("none"), String::from("duck"), String::from("down"), String::from("back"), String::from("burst"), String::from("hold"), String::from("bait"), String::from("reload"), String::from("stand"), String::from("flinch")] }

/// (src/game/morale.ts:140 LINES: a table nothing writes)
pub fn lines() -> JsMap<String, crate::morale::LINESValue> { { let mut t = JsMap::new(); t.set(String::from("duck"), crate::morale::LINESValue { say: alloc::vec![String::from("down! get down!"), String::from("get down — stay down!"), String::from("heads down!")], he: None, they: None, dark: None, lit: None, now: None }); t.set(String::from("call"), crate::morale::LINESValue { say: Vec::new(), he: Some(alloc::vec![String::from("he's coming in — I've got nobody in here!"), String::from("anyone on this net? he's right on top of me!")]), they: Some(alloc::vec![String::from("they're coming in — I've got nobody in here!"), String::from("anyone on this net? they're right on top of me!")]), dark: Some(alloc::vec![String::from("can't see a thing — somebody get in here!")]), lit: Some(alloc::vec![String::from("right in my room — somebody get in here!")]), now: None }); t.set(String::from("again"), crate::morale::LINESValue { say: alloc::vec![String::from("is anybody there? answer me!"), String::from("nobody's coming. nobody's coming—"), String::from("please — somebody answer!")], he: None, they: None, dark: None, lit: None, now: None }); t.set(String::from("out"), crate::morale::LINESValue { say: alloc::vec![String::from("I'm out! I'm out!"), String::from("empty — I'm empty!"), String::from("I'm dry! changing!")], he: None, they: None, dark: None, lit: None, now: None }); t.set(String::from("spent"), crate::morale::LINESValue { say: alloc::vec![String::from("no — no, stay back!"), String::from("keep away from me!"), String::from("don't — don't come in!")], he: None, they: None, dark: None, lit: None, now: Some(alloc::vec![String::from("no — no, stay back!"), String::from("stay away from me!"), String::from("don't — don't come in!")]) }); t.set(String::from("clear"), crate::morale::LINESValue { say: alloc::vec![String::from("breach on my room. I'm alone in here.")], he: Some(alloc::vec![String::from("he's breached. I'm on my own in here."), String::from("he's through. holding it on my own.")]), they: Some(alloc::vec![String::from("they've breached. I'm on my own in here."), String::from("they're through. holding it on my own.")]), dark: None, lit: None, now: None }); t.set(String::from("lost"), crate::morale::LINESValue { say: alloc::vec![String::from("they got him! they got him!"), String::from("he's down. oh no, he's down."), String::from("we've lost him! what do we do?")], he: None, they: None, dark: None, lit: None, now: None }); t.set(String::from("order"), crate::morale::LINESValue { say: alloc::vec![String::from("get up. watch that hole. breathe."), String::from("on your feet. eyes on that hole."), String::from("up. gun on the hole. breathe.")], he: None, they: None, dark: None, lit: None, now: None }); t.set(String::from("okay"), crate::morale::LINESValue { say: alloc::vec![String::from("okay — okay."), String::from("yeah. yeah, I'm up."), String::from("I'm up. I'm up.")], he: None, they: None, dark: None, lit: None, now: None }); t } }

/// (src/game/morale.ts:564 VISIT)
pub const VISIT: &str = "twopointsix.morale.visit";
