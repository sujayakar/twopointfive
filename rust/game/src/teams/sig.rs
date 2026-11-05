#![allow(unused_variables)]
//! Every function and method of `teams`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/teams.ts:30 isHostile)
pub fn is_hostile(g: &mut crate::game::Game, a: crate::teams::Team, b: crate::teams::Team) -> bool { todo!() }

/// (src/game/teams.ts:33 defenderWords)
pub fn defender_words(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/teams.ts:35 spokenName)
pub fn spoken_name(g: &mut crate::game::Game, o: GuardId) -> Option<String> { todo!() }

/// (src/game/teams.ts:41 staffVsOperatives)
pub fn staff_vs_operatives(g: &mut crate::game::Game, on: bool) { todo!() }

/// (src/game/teams.ts:47 teamOfChar)
pub fn team_of_char(g: &mut crate::game::Game, c_2: CharId) -> crate::teams::Team { todo!() }

/// (src/game/teams.ts:52 teamOfId)
pub fn team_of_id(g: &mut crate::game::Game, id_2: f64) -> crate::teams::Team { todo!() }

/// (src/game/teams.ts:59 hostileTo)
pub fn hostile_to(g: &mut crate::game::Game, gd: GuardId, o: crate::teams::HostileToO) -> bool { todo!() }

/// (src/game/teams.ts:62 foeChar)
pub fn foe_char(g: &mut crate::game::Game, gd: GuardId) -> Option<CharId> { todo!() }

/// (src/game/teams.ts:64 foeDown)
pub fn foe_down(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/teams.ts:66 foeCrouched)
pub fn foe_crouched(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }
