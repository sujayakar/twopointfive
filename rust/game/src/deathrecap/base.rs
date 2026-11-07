//! What the parts of `deathrecap` share, and what it hands to the host.
#![allow(unused_variables)]
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};


/// (src/game/deathrecap.ts:125 recapRound)
pub fn recap_round(g: &mut crate::game::Game, victim: Option<CharId>, was_down: bool, shooter: CharId, muzzle: V3, hit: V3, def: &crate::weapons::WeaponDef, blind: Option<bool>) { g.out.push(crate::out_events::OutEvent::RecapRound(crate::out_events::OutRecapRound { victim: victim, was_down: was_down, shooter: shooter, muzzle: muzzle, hit: hit, def: def.id, blind: blind })); }
