//! What the parts of `snow_marks` share, and what it hands to the host.
#![allow(unused_variables)]
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};


pub fn clear_snow_marks(g: &mut crate::game::Game) { g.out.push(crate::out_events::OutEvent::ClearSnowMarks); }
