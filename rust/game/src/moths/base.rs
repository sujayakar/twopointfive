//! What the parts of `moths` share, and what it hands to the host.
#![allow(unused_variables)]
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};


/// (src/game/moths.ts:48 Moths.clear)
pub fn moths_clear(g: &mut crate::game::Game) { g.out.push(crate::out_events::OutEvent::MothsClear); }
