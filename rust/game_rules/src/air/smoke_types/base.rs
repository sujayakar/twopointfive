//! What the parts of `smoke_types` share, and what it hands to the host.
#![allow(unused_variables)]
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (SmokeSystemLike.round) a round's straight leg through the air, for the host's solver to stir: one line along it
pub fn smoke_system_like_round(g: &mut crate::game::Game, from: V3, dir: V3, len: f64) { g.out.push(crate::out_events::OutEvent::SmokeSystemLikeRound(crate::out_events::OutSmokeSystemLikeRound { from, dir, len })); }

