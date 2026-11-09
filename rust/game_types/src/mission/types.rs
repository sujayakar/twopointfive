//! The types of `mission`.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

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

/// (src/game/mission.ts:62 DOWN_CARD_SECS)
pub const DOWN_CARD_SECS: f64 = 2.6;
