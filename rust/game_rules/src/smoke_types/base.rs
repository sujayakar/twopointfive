//! What the parts of `smoke_types` share, and what it hands to the host.
#![allow(unused_variables)]
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};


/// (SmokeSystemLike.round) a round's straight leg through the air, for the host's solver to stir: one line along it
pub fn smoke_system_like_round(g: &mut crate::game::Game, from: V3, dir: V3, len: f64) { g.out.push(crate::out_events::OutEvent::SmokeSystemLikeRound(crate::out_events::OutSmokeSystemLikeRound { from, dir, len })); }
/// (src/smoke/types.ts:65 SmokeSystemLike.push)
pub fn smoke_system_like_push(g: &mut crate::game::Game, p: crate::smoke_types::SmokePush) { g.out.push(crate::out_events::OutEvent::SmokeSystemLikePush(crate::out_events::OutSmokeSystemLikePush { p: p })); }



/// (src/smoke/types.ts:73 SmokeSystemLike.keepHot)
pub fn smoke_system_like_keep_hot(g: &mut crate::game::Game, heat: f64) { g.out.push(crate::out_events::OutEvent::SmokeSystemLikeKeepHot(crate::out_events::OutSmokeSystemLikeKeepHot { heat: heat })); }




/// (src/smoke/types.ts:82 SmokeSystemLike.obstaclesChanged)
pub fn smoke_system_like_obstacles_changed(g: &mut crate::game::Game, c_2: V3, r: f64) { g.out.push(crate::out_events::OutEvent::SmokeSystemLikeObstaclesChanged(crate::out_events::OutSmokeSystemLikeObstaclesChanged { c_2: c_2, r: r })); }
