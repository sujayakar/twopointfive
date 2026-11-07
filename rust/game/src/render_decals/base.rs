//! What the parts of `render_decals` share, and what it hands to the host.
#![allow(unused_variables)]
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};


/// (src/render/decals.ts:153 Decals.add)
pub fn decals_add(g: &mut crate::game::Game, kind: crate::render_decals::DecalKind, pos: V3, normal: V3, size: Option<f64>, opts: Option<crate::render_decals::DecalOpts>) -> Option<crate::render_decals::Decal> { g.out.push(crate::out_events::OutEvent::DecalsAdd(crate::out_events::OutDecalsAdd { kind: kind, pos: pos, normal: normal, size: size, opts: opts })); None }

/// (src/render/decals.ts:178 Decals.clear)
pub fn decals_clear(g: &mut crate::game::Game) { g.out.push(crate::out_events::OutEvent::DecalsClear); }


/// (src/render/decals.ts:182 Decals.touch)
pub fn decals_touch(g: &mut crate::game::Game) { g.out.push(crate::out_events::OutEvent::DecalsTouch); }
