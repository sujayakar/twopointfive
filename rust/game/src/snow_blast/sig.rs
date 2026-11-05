#![allow(unused_variables)]
//! Every function and method of `snow_blast`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/snowBlast.ts:85 stateOf)
pub fn state_of<'g>(g: &'g mut crate::game::Game) -> &'g mut crate::snow_blast::State { todo!() }

/// (src/game/snowBlast.ts:94 shownOf)
pub fn shown_of(g: &mut crate::game::Game) -> &mut crate::snow_blast::SnowBlastShown { todo!() }

/// (src/game/snowBlast.ts:96 owedOf)
pub fn owed_of(b: &mut crate::snow_blast::SnowBlastShown, live: &[crate::snow_blast::Live]) { todo!() }

/// (src/game/snowBlast.ts:107 owing)
pub fn owing(g: &mut crate::game::Game) -> Option<crate::snow_blast::Owing> { todo!() }

/// (src/game/snowBlast.ts:114 attachSnowBlast)
pub fn attach_snow_blast(g: &mut crate::game::Game) { todo!() }

/// (src/game/snowBlast.ts:122 clearSnowBlast)
pub fn clear_snow_blast(g: &mut crate::game::Game) { todo!() }

/// (src/game/snowBlast.ts:127 stepSnowDigs)
pub fn step_snow_digs(g: &mut crate::game::Game, dt: f64) -> Option<[f64; 4]> { todo!() }

/// (src/game/snowBlast.ts:142 snowBlastAt)
pub fn snow_blast_at(g: &mut crate::game::Game, at: &[f64], radius: f64) -> bool { todo!() }

/// (src/game/snowBlast.ts:168 stepSnowBlast)
pub fn step_snow_blast(g: &mut crate::game::Game, dt: f64) { todo!() }

/// (src/game/snowBlast.ts:176 snowBlastFeed)
pub fn snow_blast_feed(g: &mut crate::game::Game, c_2: &mut Vec<f32>) { todo!() }

/// (src/game/snowBlast.ts:189 snowBlastPuffs)
pub fn snow_blast_puffs(g: &mut crate::game::Game, pp: &mut crate::render_particles::Particles, room: f64) -> f64 { todo!() }

/// (src/game/snowBlast.ts:204 snowBlastThrow)
pub fn snow_blast_throw(g: &mut crate::game::Game, pp: &mut crate::render_particles::Particles, room: f64, size: f64, size_jit: f64) -> f64 { todo!() }
