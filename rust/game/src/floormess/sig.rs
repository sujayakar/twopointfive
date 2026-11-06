#![allow(unused_variables)]
//! Every function and method of `floormess`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/floormess.ts:33 on)
pub fn on(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/floormess.ts:35 H)
pub fn h(x: f64, z: f64, k: f64) -> f64 { todo!() }

/// (src/game/floormess.ts:36 lookOf)
pub fn look_of(d: &crate::floormess::LookOfD) -> f64 { todo!() }

/// (src/game/floormess.ts:38 takes)
pub fn takes(g: &mut crate::game::Game, b: BoxId) -> bool { todo!() }

/// (src/game/floormess.ts:44 fit)
pub fn fit(g: &mut crate::game::Game, x: f64, z: f64, want: f64) -> f64 { todo!() }

/// (src/game/floormess.ts:58 patch)
pub fn patch(g: &mut crate::game::Game, x: f64, z: f64, want: f64, strength: f64, k: f64, heart: Option<bool>) -> bool { todo!() }

/// (src/game/floormess.ts:87 spread)
pub fn spread(g: &mut crate::game::Game, x: f64, z: f64, s: crate::floormess::Spread, size: f64, face: Option<[f64; 2]>) { todo!() }

/// (src/game/floormess.ts:108 messBurst)
pub fn mess_burst(g: &mut crate::game::Game, at: V3, size: Option<f64>) { g.out.push(crate::out_events::OutEvent::MessBurst(crate::out_events::OutMessBurst { at: at, size: size })); }

/// (src/game/floormess.ts:115 messWall)
pub fn mess_wall(g: &mut crate::game::Game, p: V3, n: V3, thick: f64, through: bool) { g.out.push(crate::out_events::OutEvent::MessWall(crate::out_events::OutMessWall { p: p, n: n, thick: thick, through: through })); }

/// (src/game/floormess.ts:123 messOpened)
pub fn mess_opened(g: &mut crate::game::Game, rect: [f64; 4]) { g.out.push(crate::out_events::OutEvent::MessOpened(crate::out_events::OutMessOpened { rect: rect })); }

/// (src/game/floormess.ts:130 messEvicted)
pub fn mess_evicted(g: &mut crate::game::Game, c_2: DebrisChunkId) { todo!() }
