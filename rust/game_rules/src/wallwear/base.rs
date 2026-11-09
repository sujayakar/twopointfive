//! What the parts of `wallwear` share, and what it hands to the host.
#![allow(unused_variables)]
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};
use crate::out_events::*;
use crate::render_decals::{mark_asked, Ask};


pub fn wall_wear_round(g: &mut crate::game::Game, box_: BoxId, surf: crate::scene_materials::Surf, key: f64, counted: f64, frac: f64, hit_p: V3, dir: V3) { let e = OutWallWearRound { box_, surf, key, counted, frac, hit_p, dir }; let ours = g.fog_of_war.filing_ours; mark_asked(g, Ask::Round(e, ours)); }

pub fn wall_wear_reset(g: &mut crate::game::Game) { mark_asked(g, Ask::Reset); }
