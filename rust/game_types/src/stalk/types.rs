//! The types of `stalk`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
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

/// (TypeScript: St, src/game/stalk.ts:137)
#[derive(Clone, Copy, Debug)]
pub struct St {
    pub on: Option<GuardId>,
    pub grab: Option<GuardId>,
    pub t: f64,
    pub wait: Option<bool>,
    pub armed: Option<bool>,
    pub stick: Option<[f64; 2]>,
    pub orbit_r: Option<f64>,
    pub orbit_t: Option<f64>,
    pub orbit_ask: Option<[f64; 4]>,
    pub broke: Option<crate::stalk::StBroke>,
}
impl St {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        St {
            on: None,
            grab: None,
            t: 0.0,
            wait: None,
            armed: None,
            stick: None,
            orbit_r: None,
            orbit_t: None,
            orbit_ask: None,
            broke: None,
        }
    }
}
impl Default for St { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/stalk.ts:141 St.broke, src/game/stalk.ts:141)
#[derive(Clone, Copy, Debug)]
pub struct StBroke {
    pub gd: GuardId,
    pub t: f64,
    pub at: V3,
}
impl StBroke {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        StBroke {
            gd: GuardId(0),
            t: 0.0,
            at: [0.0; 3],
        }
    }
}
impl Default for StBroke { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: StalkMove, src/game/stalk.ts:231)
#[derive(Clone, Copy, Debug)]
pub struct StalkMove {
    pub mx: f64,
    pub mz: f64,
    pub pace: f64,
    pub at: V3,
    pub keep: Option<bool>,
}
impl StalkMove {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        StalkMove {
            mx: 0.0,
            mz: 0.0,
            pace: 0.0,
            at: [0.0; 3],
            keep: None,
        }
    }
}
impl Default for StalkMove { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: StalkStick, src/game/stalk.ts:235)
#[derive(Clone, Copy, Debug)]
pub struct StalkStick {
    pub x: f64,
    pub y: f64,
    pub fwd: V3,
    pub right: V3,
}
impl StalkStick {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        StalkStick {
            x: 0.0,
            y: 0.0,
            fwd: [0.0; 3],
            right: [0.0; 3],
        }
    }
}
impl Default for StalkStick { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: StalkKeys, src/game/stalk.ts:237)
#[derive(Clone, Copy, Debug)]
pub struct StalkKeys {
    pub mx: f64,
    pub mz: f64,
    pub dt: f64,
}
impl StalkKeys {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        StalkKeys {
            mx: 0.0,
            mz: 0.0,
            dt: 0.0,
        }
    }
}
impl Default for StalkKeys { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/stalk.ts:225 POLISH_UP, src/game/stalk.ts:225)
#[derive(Clone, Copy, Debug)]
pub struct POLISHUP {
    pub turn_in: bool,
    pub corner_show: bool,
    pub yank_with: bool,
    pub catch_long: f64,
    pub jolt: f64,
}
impl POLISHUP {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        POLISHUP {
            turn_in: false,
            corner_show: false,
            yank_with: false,
            catch_long: 0.0,
            jolt: 0.0,
        }
    }
}
impl Default for POLISHUP { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/stalk.ts:226 POLISH_DOWN, src/game/stalk.ts:226)
#[derive(Clone, Copy, Debug)]
pub struct POLISHDOWN {
    pub turn_in: bool,
    pub corner_show: bool,
    pub yank_with: bool,
    pub catch_long: f64,
    pub jolt: f64,
}
impl POLISHDOWN {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        POLISHDOWN {
            turn_in: false,
            corner_show: false,
            yank_with: false,
            catch_long: 0.0,
            jolt: 0.0,
        }
    }
}
impl Default for POLISHDOWN { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/stalk.ts:309 stalkTell ->, src/game/stalk.ts:309)
#[derive(Clone, Copy, Debug)]
pub struct StalkTell {
    pub gd: GuardId,
    pub at: V3,
    pub reach: f64,
    pub rise: f64,
    pub low: f64,
}
impl StalkTell {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        StalkTell {
            gd: GuardId(0),
            at: [0.0; 3],
            reach: 0.0,
            rise: 0.0,
            low: 0.0,
        }
    }
}
impl Default for StalkTell { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/stalk.ts:322 stalkWait ->, src/game/stalk.ts:322)
#[derive(Clone, Copy, Debug)]
pub struct StalkWait {
    pub at: V3,
    pub reach: f64,
    pub coil: f64,
}
impl StalkWait {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        StalkWait {
            at: [0.0; 3],
            reach: 0.0,
            coil: 0.0,
        }
    }
}
impl Default for StalkWait { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
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
