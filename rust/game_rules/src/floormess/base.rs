//! THE FLOOR'S MESS is one arm of the marks (b_01.rs lays them, render_decals set.rs says when). The out events are the page with a copy of the game's, which lays its own (src/game/floormess.ts). No rule reads a mark.
#![allow(unused_variables)]
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};
use crate::out_events::*;
use crate::render_decals::{mark_asked, Ask};


/// (src/game/floormess.ts:108 messBurst)
pub fn mess_burst(g: &mut crate::game::Game, at: V3, size: Option<f64>) { let e = OutMessBurst { at, size }; mark_asked(g, Ask::Burst(e.clone())); g.out.push(OutEvent::MessBurst(e)); }

/// (src/game/floormess.ts:115 messWall)
pub fn mess_wall(g: &mut crate::game::Game, p: V3, n: V3, thick: f64, through: bool) { let e = OutMessWall { p, n, thick, through }; mark_asked(g, Ask::Wall(e.clone())); g.out.push(OutEvent::MessWall(e)); }

/// (src/game/floormess.ts:123 messOpened)
pub fn mess_opened(g: &mut crate::game::Game, rect: [f64; 4]) { let e = OutMessOpened { rect }; mark_asked(g, Ask::Opened(e.clone())); g.out.push(OutEvent::MessOpened(e)); }

/// A resting piece the debris pool took: the crushed mark of it where it lay (a pane's fallen cell leaves none), asked by value, since the slot is another's when the tick is done. (src/game/floormess.ts messEvicted)
pub fn mess_evicted(g: &mut crate::game::Game, c_2: DebrisChunkId) {
    let c = &g.tab.debris_chunk[c_2];
    if c.glass.is_some() { return; }
    let (mat, kind, at) = (c.mat.as_ref().map(|m| m.mat), c.kind, c.box_);
    let b = &g.tab.boxes[at];
    let e = OutMessEvicted { c: b.c, h: b.h, yaw: b.yaw, rot: b.rot, mat, kind };
    mark_asked(g, Ask::Evicted(e.clone()));
    g.out.push(OutEvent::MessEvicted(e));
}
