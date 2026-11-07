//! What the parts of `floormess` share, and what it hands to the host.
#![allow(unused_variables)]
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};


/// (src/game/floormess.ts:108 messBurst)
pub fn mess_burst(g: &mut crate::game::Game, at: V3, size: Option<f64>) { g.out.push(crate::out_events::OutEvent::MessBurst(crate::out_events::OutMessBurst { at: at, size: size })); }

/// (src/game/floormess.ts:115 messWall)
pub fn mess_wall(g: &mut crate::game::Game, p: V3, n: V3, thick: f64, through: bool) { g.out.push(crate::out_events::OutEvent::MessWall(crate::out_events::OutMessWall { p: p, n: n, thick: thick, through: through })); }

/// (src/game/floormess.ts:123 messOpened)
pub fn mess_opened(g: &mut crate::game::Game, rect: [f64; 4]) { g.out.push(crate::out_events::OutEvent::MessOpened(crate::out_events::OutMessOpened { rect: rect })); }
