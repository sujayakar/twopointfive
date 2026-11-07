//! Functions of `fogofwar` that a caller names and nobody has written: a call says so by name.
#![allow(unused_variables)]
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};


/// (src/game/fogofwar.ts:845 FogOfWar.snapshot)
pub fn fog_of_war_snapshot(g: &mut crate::game::Game) -> Option<crate::fogofwar::FogSnap> { crate::no_body("fogofwar::fog_of_war_snapshot") }
/// (src/game/fogofwar.ts:866 FogOfWar.restore)
pub fn fog_of_war_restore(g: &mut crate::game::Game, s: Option<&crate::fogofwar::FogSnap>) { crate::no_body("fogofwar::fog_of_war_restore") }
