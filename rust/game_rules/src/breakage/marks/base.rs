//! What the parts of `marks` share, and what it hands to the host.
#![allow(unused_variables)]
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};
use crate::out_events::*;
use crate::render_decals::{mark_asked, Ask};


/// (src/game/marks.ts:39 bulletMark)
pub fn bullet_mark(g: &mut crate::game::Game, hit_p: V3, n: V3, s: Option<crate::marks::Surface2>, def: &crate::weapons::WeaponDef) { let e = OutBulletMark { hit_p, n, s, def: def.id }; mark_asked(g, Ask::Bullet(e)); }

/// (src/game/marks.ts:51 bangScorch)
pub fn bang_scorch(g: &mut crate::game::Game, at: V3) { let e = OutBangScorch { at }; mark_asked(g, Ask::Bang(e)); }

/// (src/game/marks.ts:90 kickScuff)
pub fn kick_scuff(g: &mut crate::game::Game, d: DoorId, from: V3) { let e = OutKickScuff { d, from }; mark_asked(g, Ask::Kick(e)); }

/// (src/game/marks.ts:104 chipMark)
pub fn chip_mark(g: &mut crate::game::Game, p: V3, n: V3, s: Option<crate::marks::Surface2>, size: Option<f64>) { let e = OutChipMark { p, n, s, size }; mark_asked(g, Ask::Chip(e)); }
