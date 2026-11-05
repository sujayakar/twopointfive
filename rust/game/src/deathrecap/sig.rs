#![allow(unused_variables)]
//! Every function and method of `deathrecap`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/deathrecap.ts:96 fresh)
pub fn fresh(g: &mut crate::game::Game) -> crate::deathrecap::State { todo!() }

/// (src/game/deathrecap.ts:97 stateOf)
pub fn state_of<'g>(g: &'g mut crate::game::Game) -> &'g mut crate::deathrecap::State { todo!() }

/// (src/game/deathrecap.ts:104 ours)
pub fn ours(side: &str) -> bool { todo!() }

/// (src/game/deathrecap.ts:105 sideOf)
pub fn side_of(g: &mut crate::game::Game, gd: GuardId) -> String { todo!() }

/// (src/game/deathrecap.ts:106 xz)
pub fn xz(p: V3) -> [f64; 2] { todo!() }

/// (src/game/deathrecap.ts:109 eachMan)
pub fn each_man(g: &mut crate::game::Game, fn_: &mut dyn FnMut(&mut crate::game::Game, CharId, bool, Option<GuardId>)) { todo!() }

/// (src/game/deathrecap.ts:114 manOf)
pub fn man_of(g: &mut crate::game::Game, c_2: CharId, gd: Option<GuardId>) -> crate::deathrecap::RecapMan { todo!() }

/// (src/game/deathrecap.ts:120 live)
pub fn live(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/deathrecap.ts:125 recapRound)
pub fn recap_round(g: &mut crate::game::Game, victim: Option<CharId>, was_down: bool, shooter: CharId, muzzle: V3, hit: V3, def: &crate::weapons::WeaponDef, blind: Option<bool>) { todo!() }

/// (src/game/deathrecap.ts:138 offLine)
pub fn off_line(p: [f64; 2], a: [f64; 2], b: [f64; 2]) -> f64 { todo!() }

/// (src/game/deathrecap.ts:144 explain)
pub fn explain(g: &mut crate::game::Game, c_2: CharId, gd: Option<GuardId>, gaze: f64) -> crate::deathrecap::RecapDeath { todo!() }

/// (src/game/deathrecap.ts:169 sample)
pub fn sample(g: &mut crate::game::Game) { todo!() }

/// (src/game/deathrecap.ts:185 take)
pub fn take(g: &mut crate::game::Game, death: &crate::deathrecap::RecapDeath) { todo!() }

/// (src/game/deathrecap.ts:202 flush)
pub fn flush(g: &mut crate::game::Game, early: bool) { todo!() }

/// (src/game/deathrecap.ts:207 recapTick)
pub fn recap_tick(g: &mut crate::game::Game) { todo!() }

/// (src/game/deathrecap.ts:250 eighth)
pub fn eighth(dx: f64, dz: f64) -> Option<[f64; 2]> { todo!() }

/// (src/game/deathrecap.ts:255 headOf)
pub fn head_of(g: &mut crate::game::Game, c_2: CharId) -> V3 { todo!() }

/// (src/game/deathrecap.ts:256 squadUp)
pub fn squad_up(g: &mut crate::game::Game, gd: GuardId) -> bool { todo!() }

/// (src/game/deathrecap.ts:258 lineTo)
pub fn line_to(g: &mut crate::game::Game, c_2: CharId, at: [f64; 2]) -> bool { todo!() }

/// (src/game/deathrecap.ts:264 heard)
pub fn heard(g: &mut crate::game::Game, src: V3) -> bool { todo!() }

/// (src/game/deathrecap.ts:282 samSees)
pub fn sam_sees(g: &mut crate::game::Game, at: [f64; 2]) -> bool { todo!() }

/// (src/game/deathrecap.ts:283 manSees)
pub fn man_sees(g: &mut crate::game::Game, pl: PlayerId, at: [f64; 2]) -> bool { todo!() }

/// (src/game/deathrecap.ts:290 sideWord)
pub fn side_word(g: &mut crate::game::Game, pl: PlayerId, u: [f64; 2]) -> String { todo!() }

/// (src/game/deathrecap.ts:297 witness)
pub fn witness(g: &mut crate::game::Game) { todo!() }

/// (src/game/deathrecap.ts:317 lostLive)
pub fn lost_live(g: &mut crate::game::Game) -> Vec<crate::deathrecap::LostMan> { todo!() }

/// (src/game/deathrecap.ts:323 lostShown)
pub fn lost_shown(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/deathrecap.ts:326 lostOf)
pub fn lost_of(g: &mut crate::game::Game, id_2: f64) -> Option<crate::deathrecap::LostMan> { todo!() }

/// (src/game/deathrecap.ts:330 recapOver)
pub fn recap_over(g: &mut crate::game::Game) -> bool { todo!() }
