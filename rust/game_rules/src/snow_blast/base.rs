//! What the parts of `snow_blast` share, and what it hands to the host.
#![allow(unused_variables)]
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};


pub fn clear_snow_blast(g: &mut crate::game::Game) { g.out.push(crate::out_events::OutEvent::ClearSnowBlast); }
