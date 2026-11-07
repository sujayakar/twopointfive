//! THE FLOOR'S MESS IS THE HOST'S: the host lays each mark on its decals and keeps the count (src/game/floormess.ts). The Rust game only says what happened, and no rule reads a mark.
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

/// A resting piece the debris pool took: the crushed mark of it where it lay (a pane's fallen cell leaves none). (src/game/floormess.ts messEvicted)
pub fn mess_evicted(g: &mut crate::game::Game, c_2: DebrisChunkId) {
    let c = &g.tab.debris_chunk[c_2];
    if c.glass.is_some() { return; }
    let (mat, kind, at) = (c.mat.as_ref().map(|m| m.mat), c.kind, c.box_);
    let b = &g.tab.boxes[at];
    let e = crate::out_events::OutMessEvicted { c: b.c, h: b.h, yaw: b.yaw, rot: b.rot, mat, kind };
    g.out.push(crate::out_events::OutEvent::MessEvicted(e));
}
