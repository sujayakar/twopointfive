//! What the parts of `wallwear` share, and what it hands to the host.
#![allow(unused_variables)]
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};


/// (src/game/wallwear.ts:58 WallWear.round)
pub fn wall_wear_round(g: &mut crate::game::Game, box_: BoxId, surf: crate::scene_materials::Surf, key: f64, counted: f64, frac: f64, hit_p: V3, dir: V3) { g.out.push(crate::out_events::OutEvent::WallWearRound(crate::out_events::OutWallWearRound { box_: box_, surf: surf, key: key, counted: counted, frac: frac, hit_p: hit_p, dir: dir })); }

/// (src/game/wallwear.ts:88 WallWear.reset)
pub fn wall_wear_reset(g: &mut crate::game::Game) { g.out.push(crate::out_events::OutEvent::WallWearReset); }
