//! The types of `teams`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Team {
    Guard,
    Operative,
    Hostile,
    Player,
    Civilian,
}
impl Team {
    pub const fn as_str(self) -> &'static str {
        match self {
            Team::Guard => "guard",
            Team::Operative => "operative",
            Team::Hostile => "hostile",
            Team::Player => "player",
            Team::Civilian => "civilian",
        }
    }
}

/// (TypeScript: a union)
#[derive(Clone, Copy, Debug)]
pub enum HostileToO {
    Guard(GuardId),
    Player,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct TeamsMod {
    /// (src/game/teams.ts:23 HOSTILE)
    pub hostile: JsMap<crate::teams::Team, JsMap<crate::teams::Team, bool>>,
    /// (src/game/teams.ts:40 staffOps)
    pub staff_ops: f64,
}
impl TeamsMod {
    pub fn blank() -> Self {
        TeamsMod {
            hostile: { let mut t = JsMap::new(); t.set(crate::teams::Team::Guard, { let mut t = JsMap::new(); t.set(crate::teams::Team::Guard, false); t.set(crate::teams::Team::Operative, false); t.set(crate::teams::Team::Hostile, false); t.set(crate::teams::Team::Player, true); t.set(crate::teams::Team::Civilian, false); t }); t.set(crate::teams::Team::Operative, { let mut t = JsMap::new(); t.set(crate::teams::Team::Guard, false); t.set(crate::teams::Team::Operative, false); t.set(crate::teams::Team::Hostile, true); t.set(crate::teams::Team::Player, false); t.set(crate::teams::Team::Civilian, false); t }); t.set(crate::teams::Team::Hostile, { let mut t = JsMap::new(); t.set(crate::teams::Team::Guard, false); t.set(crate::teams::Team::Operative, true); t.set(crate::teams::Team::Hostile, false); t.set(crate::teams::Team::Player, true); t.set(crate::teams::Team::Civilian, false); t }); t.set(crate::teams::Team::Player, { let mut t = JsMap::new(); t.set(crate::teams::Team::Guard, true); t.set(crate::teams::Team::Operative, false); t.set(crate::teams::Team::Hostile, true); t.set(crate::teams::Team::Player, false); t.set(crate::teams::Team::Civilian, false); t }); t.set(crate::teams::Team::Civilian, { let mut t = JsMap::new(); t.set(crate::teams::Team::Guard, false); t.set(crate::teams::Team::Operative, false); t.set(crate::teams::Team::Hostile, false); t.set(crate::teams::Team::Player, false); t.set(crate::teams::Team::Civilian, false); t }); t },
            staff_ops: 0.0,
        }
    }
}

/// (src/game/teams.ts:20 TEAMS: a table nothing writes)
pub fn teams() -> Vec<String> { alloc::vec![String::from("guard"), String::from("operative"), String::from("hostile"), String::from("player"), String::from("civilian")] }
