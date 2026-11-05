#![allow(unused_variables)]
//! Every function and method of `blast_feel`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/blastFeel.ts:84 loadFeel)
pub fn load_feel(g: &mut crate::game::Game) { todo!() }

/// (src/game/blastFeel.ts:93 stateOf)
pub fn state_of(g: &mut crate::game::Game) -> crate::blast_feel::State { todo!() }

/// (src/game/blastFeel.ts:95 hashU)
pub fn hash_u(n: f64) -> f64 { todo!() }

/// (src/game/blastFeel.ts:96 dice)
pub fn dice(s: &mut crate::blast_feel::State) -> f64 { todo!() }

/// (src/game/blastFeel.ts:98 live)
pub fn live(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/blastFeel.ts:99 thick)
pub fn thick(g: &mut crate::game::Game, w: &crate::wallcharge::WallSpot) -> f64 { todo!() }

/// (src/game/blastFeel.ts:100 near)
pub fn near(a: V3, b: V3, m: f64) -> bool { todo!() }

/// (src/game/blastFeel.ts:102 burstAt)
pub fn burst_at(g: &mut crate::game::Game, w: &crate::wallcharge::WallSpot) -> V3 { todo!() }

/// (src/game/blastFeel.ts:104 holed)
pub fn holed(g: &mut crate::game::Game, w: &crate::wallcharge::WallSpot) -> bool { todo!() }

/// (src/game/blastFeel.ts:105 later)
pub fn later(g: &mut crate::game::Game, pl: PlayerId, secs: f64, run: &mut dyn FnMut(&mut crate::game::Game)) { todo!() }

/// (src/game/blastFeel.ts:109 wallReport)
pub fn wall_report(g: &mut crate::game::Game, at: V3) -> bool { todo!() }

/// (src/game/blastFeel.ts:110 wallReportOf)
pub fn wall_report_of(g: &mut crate::game::Game, at: V3) -> bool { todo!() }

/// (src/game/blastFeel.ts:129 blastFeelTick)
pub fn blast_feel_tick(g: &mut crate::game::Game) { todo!() }

/// (src/game/blastFeel.ts:130 blastFeelTickRaw)
pub fn blast_feel_tick_raw(g: &mut crate::game::Game) { todo!() }

/// (src/game/blastFeel.ts:144 kickOf)
pub fn kick_of(g: &mut crate::game::Game, pl: PlayerId) -> Option<V3> { todo!() }

/// (src/game/blastFeel.ts:162 report)
pub fn report(g: &mut crate::game::Game, pl: PlayerId, at: V3) { todo!() }

/// (src/game/blastFeel.ts:175 emit)
pub fn emit(g: &mut crate::game::Game, pl: PlayerId, p: &mut crate::render_particles::Particles, kind: crate::render_particles::PKind, pos: V3, dir: V3, count: f64, o: crate::render_particles::EmitOpts) { todo!() }

/// (src/game/blastFeel.ts:181 fireball)
pub fn fireball(g: &mut crate::game::Game, pl: PlayerId, w: &crate::wallcharge::WallSpot, open: bool, n: V3) { todo!() }

/// (src/game/blastFeel.ts:196 ringStage)
pub fn ring_stage(g: &mut crate::game::Game, pl: PlayerId, k: f64, faces: &[crate::blast_feel::Face]) { todo!() }

/// (src/game/blastFeel.ts:213 ceiling)
pub fn ceiling(g: &mut crate::game::Game, pl: PlayerId, faces: &[crate::blast_feel::Face]) { todo!() }
