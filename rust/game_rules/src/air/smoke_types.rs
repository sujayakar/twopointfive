#![allow(unused_imports, unused_variables)]
#![allow(unused_variables)]
pub use ::game_types::smoke_types::types;
pub use types::*;
use crate::game::Game;
use crate::v3::{self, Quat, V3};

/// (SmokeSystemLike.round) a round's straight leg through the air, for the host's solver to stir: one line along it
pub fn smoke_system_like_round(g: &mut crate::game::Game, from: V3, dir: V3, len: f64) { g.out.push(crate::out_events::OutEvent::SmokeSystemLikeRound(crate::out_events::OutSmokeSystemLikeRound { from, dir, len })); }

// THE SOLVER'S OBJECT IS ALSO WHERE THE GAME'S OWN SMOKE IS FED (crate::sense). 
// NOTHING COMES BACK FROM THE PICTURE: the tick says what smoke it wants (an emission, a round's line, a push, a clear) as out events, always, and the host's solver
// takes it or drops it by what it is doing and by its own budget.
pub fn smoke_system_like_emit(g: &mut crate::game::Game, e: &crate::smoke_types::SmokeEmitter) { crate::sense::smoke_emit(g, e); }

/// how many more emitters would still be packed this step: the game's own count where it owns the smoke, else the host's solver's (none here)
pub fn smoke_system_like_budget(g: &crate::game::Game) -> f64 { crate::sense::smoke_budget(g) }

/// BOUNDARY NO-OP (the picture's / the host's):
pub fn smoke_system_like_clear_all(g: &mut crate::game::Game) { crate::sense::smoke_clear(g); }

