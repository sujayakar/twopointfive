//! What the parts of `snow_tracks` share, and what it hands to the host.
#![allow(unused_variables)]
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};


/// (src/game/snowTracks.ts:175 clearSnowTracks)
pub fn clear_snow_tracks(g: &mut crate::game::Game, t: &crate::snow_tracks::SnowTracks) { g.out.push(crate::out_events::OutEvent::ClearSnowTracks); }
