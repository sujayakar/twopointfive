//! What the parts of `render_reach_keep` share, and what it hands to the host.
#![allow(unused_variables)]
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};


/// (src/render/reachKeep.ts:115 ReachKeep.changed)
pub fn reach_keep_changed(g: &mut crate::game::Game, rect: Option<[f64; 4]>, hold: Option<f64>) { g.out.push(crate::out_events::OutEvent::ReachKeepChanged(crate::out_events::OutReachKeepChanged { rect: rect, hold: hold })); }
