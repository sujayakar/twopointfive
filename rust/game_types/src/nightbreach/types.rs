//! The types of `nightbreach`.
#![allow(unused_imports)]
use crate::{Blank, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct BlindBurst {
    pub until: f64,
    pub n: f64,
    pub next: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct NightBreachMark {
    pub pos: V3,
    pub aim_at: V3,
    pub name: Option<String>,
    pub drill: Option<bool>,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct NightBreachLayout {
    pub name: String,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum NightBreachState {
    Clear,
    Set,
    Failed,
    Off,
    Going,
    Staged,
    Stacking,
    Inside,
    Recalled,
}

#[derive(Clone, Debug, ToBytes, Blank)]
pub struct NightBreach {
    pub active: bool,
    pub drill: Option<crate::breach::BreachDrill>,
    pub ops: Vec<GuardId>,
    pub hostiles: Vec<GuardId>,
    pub layout: Option<crate::nightbreach::NightBreachLayout>,
    pub marks: Vec<crate::nightbreach::NightBreachMark>,
    pub staged: f64,
    pub log: Vec<crate::nightbreach::NightBreachLogItem>,
    pub no_drill: bool,
    pub bang_taken: bool,
    pub bang_by: Option<SeatId>,
    pub blind: bool,
    pub bursts: Vec<crate::nightbreach::BlindBurst>,
    pub blind_rounds: f64,
    pub benched: Option<Vec<GuardId>>,
    pub beats_seen: f64,
    pub said: JsSet<String>,
    pub cut: bool,
    pub failure: Option<String>,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct NightBreachLogItem {
    pub what: String,
}

#[derive(Clone, Debug, Default)]
pub struct SQUAD4Item {
    pub name: String,
    pub pair: crate::game::PairId,
    pub tint: V3,
    pub tint2: V3,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct BLIND {
    pub after: f64,
    pub stagger: f64,
    pub rounds: f64,
    pub every: f64,
    pub spread: f64,
    pub low_gun: f64,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum NightBreachOrder {
    Stack,
    Go,
}

#[derive(Clone, Debug, Default)]
pub struct NightBreachObjective {
    pub text: String,
    pub pos: Option<V3>,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct NightbreachMod {
    /// (src/game/nightbreach.ts:140 applied)
    #[is({ let k = crate::knobs::NightbreachK::page(); Some(crate::scene_scenarios::TeamStart { sam: k.sam_at, sam_yaw: k.sam_yaw, ops: None, ops_yaw: None }) })] pub applied: Option<crate::scene_scenarios::TeamStart>,
}

/// (src/game/nightbreach.ts:88 SQUAD4: a table nothing writes)
pub fn squad4() -> Vec<crate::nightbreach::SQUAD4Item> { alloc::vec![crate::nightbreach::SQUAD4Item { name: String::from("Red One"), pair: crate::game::PairId::Red, tint: [0.2, 0.085, 0.075], tint2: [0.55, 0.07, 0.05] }, crate::nightbreach::SQUAD4Item { name: String::from("Red Two"), pair: crate::game::PairId::Red, tint: [0.2, 0.085, 0.075], tint2: [0.55, 0.07, 0.05] }, crate::nightbreach::SQUAD4Item { name: String::from("Blue One"), pair: crate::game::PairId::Blue, tint: [0.06, 0.075, 0.13], tint2: [0.07, 0.2, 0.6] }, crate::nightbreach::SQUAD4Item { name: String::from("Blue Two"), pair: crate::game::PairId::Blue, tint: [0.06, 0.075, 0.13], tint2: [0.07, 0.2, 0.6] }] }

/// (src/game/nightbreach.ts:96 SQUAD4_BUDGET)
pub const SQUAD4_BUDGET: f64 = 10.0;

/// (src/game/nightbreach.ts:99 NUMBER: a table nothing writes)
pub fn number() -> Vec<String> { alloc::vec![String::from("no"), String::from("one"), String::from("two"), String::from("three"), String::from("four"), String::from("five"), String::from("six")] }

/// (src/game/nightbreach.ts:104 BLIND: a table nothing writes)
pub fn blind() -> crate::nightbreach::BLIND { crate::nightbreach::BLIND { after: 0.3, stagger: 0.07, rounds: 12.0, every: 0.1, spread: 0.3, low_gun: 0.21 } }
