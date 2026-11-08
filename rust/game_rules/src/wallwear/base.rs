//! What the parts of `wallwear` share, and what it hands to the host.
#![allow(unused_variables)]
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};
use crate::out_events::*;
use crate::render_decals::{mark_asked, Ask};


/// (src/game/wallwear.ts:58 WallWear.round)
pub fn wall_wear_round(g: &mut crate::game::Game, box_: BoxId, surf: crate::scene_materials::Surf, key: f64, counted: f64, frac: f64, hit_p: V3, dir: V3) { let e = OutWallWearRound { box_, surf, key, counted, frac, hit_p, dir }; let ours = g.engine.particles.as_ref().is_some_and(|p| p.ours); mark_asked(g, Ask::Round(e.clone(), ours)); g.out.push(OutEvent::WallWearRound(e)); }

/// (src/game/wallwear.ts:88 WallWear.reset)
pub fn wall_wear_reset(g: &mut crate::game::Game) { mark_asked(g, Ask::Reset); g.out.push(OutEvent::WallWearReset); }
