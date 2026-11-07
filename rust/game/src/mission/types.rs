//! The types of `mission`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum TallyK {
    Keys,
    Spotted,
    Shots,
    Kills,
    Bodies,
    Alerts,
    Knockouts,
}
impl TallyK {
    pub const fn as_str(self) -> &'static str {
        match self {
            TallyK::Keys => "keys",
            TallyK::Spotted => "spotted",
            TallyK::Shots => "shots",
            TallyK::Kills => "kills",
            TallyK::Bodies => "bodies",
            TallyK::Alerts => "alerts",
            TallyK::Knockouts => "knockouts",
        }
    }
}

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct MissionMod {
    /// (src/game/mission.ts:174 doneLine)
    pub done_line: JsMap<u32, String>,
}
impl MissionMod {
    pub fn blank() -> Self {
        MissionMod {
            done_line: JsMap::new(),
        }
    }
}

/// (src/game/mission.ts:62 DOWN_CARD_SECS)
pub const DOWN_CARD_SECS: f64 = 2.6;
