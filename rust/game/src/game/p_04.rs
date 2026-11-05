//! src/game/game.ts lines 962 to 1199
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// How many lights still wait for their reach mask, queued or held.   (game.ts Game.reachPending)
pub fn game_reach_pending(g: &mut crate::game::Game) -> f64 {
    (g.reach_queue.len() + g.reach_held.len()) as f64
}

/// A directional light's reach mask, for the ReSTIR estimator. Picture-only and not translated (triage: dress); see notes.   (game.ts Game.buildDirReach)
pub fn game_build_dir_reach(g: &mut crate::game::Game, l: LightId, only: Option<&crate::render_reach::ReachOnly>) {
    todo!("needs: a decision: buildDirReach and sunReach in notInTheGame, or RtLight's kind, dir and reachDir on a LightId")
}

/// The mains, read: nothing to hand back, a reader calls `sim_lightshome::mains_*(g)`, always live.   (game.ts Game.blackout)
pub fn game_blackout(g: &mut crate::game::Game) {}

/// How long a man of theirs has been at the panel, read.   (game.ts Game.breakerFixT)
pub fn game_breaker_fix_t(g: &mut crate::game::Game) -> f64 {
    crate::sim_lightshome::mains_fix_t(g)
}
