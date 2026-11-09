//! The types of `stalk`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum Why {
    Keys,
    Wall,
    Let,
    Him,
    Turned,
    Hands,
    Far,
}
impl Why {
    pub const fn as_str(self) -> &'static str {
        match self {
            Why::Keys => "keys",
            Why::Wall => "wall",
            Why::Let => "let",
            Why::Him => "him",
            Why::Turned => "turned",
            Why::Hands => "hands",
            Why::Far => "far",
        }
    }
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct St {
    pub on: Option<GuardId>,
    pub grab: Option<GuardId>,
    pub t: f64,
    pub wait: Option<bool>,
    pub armed: Option<bool>,
    pub orbit_r: Option<f64>,
    pub orbit_t: Option<f64>,
    pub orbit_ask: Option<[f64; 4]>,
    pub broke: Option<crate::stalk::StBroke>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct StBroke {
    pub gd: GuardId,
    pub t: f64,
    pub at: V3,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct StalkMove {
    pub mx: f64,
    pub mz: f64,
    pub pace: f64,
    pub at: V3,
    pub keep: Option<bool>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct StalkKeys {
    pub mx: f64,
    pub mz: f64,
    pub dt: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct POLISHUP {
    pub turn_in: bool,
    pub corner_show: bool,
    pub yank_with: bool,
    pub catch_long: f64,
    pub jolt: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct POLISHDOWN {
    pub turn_in: bool,
    pub corner_show: bool,
    pub yank_with: bool,
    pub catch_long: f64,
    pub jolt: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct StalkTell {
    pub gd: GuardId,
    pub at: V3,
    pub reach: f64,
    pub rise: f64,
    pub low: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct StalkWait {
    pub at: V3,
    pub reach: f64,
    pub coil: f64,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes)]
pub struct StalkMod {
    /// (src/game/stalk.ts:142 STATE)
    pub state: JsMap<PlayerId, crate::stalk::St>,
}
impl StalkMod {
    pub fn blank() -> Self {
        StalkMod {
            state: JsMap::new(),
        }
    }
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct StalkStats {
    pub begun: i32,
    pub grabs: i32,
    pub corner: i32,
    pub out: [i32; 7],
    pub last: String,
}

/// (src/game/stalk.ts:225 POLISH_UP: a table nothing writes)
pub fn polish_up() -> crate::stalk::POLISHUP { crate::stalk::POLISHUP { turn_in: true, corner_show: true, yank_with: true, catch_long: 1.1, jolt: 0.6 } }

/// (src/game/stalk.ts:226 POLISH_DOWN: a table nothing writes)
pub fn polish_down() -> crate::stalk::POLISHDOWN { crate::stalk::POLISHDOWN { turn_in: false, corner_show: false, yank_with: false, catch_long: 1.0, jolt: 0.0 } }
