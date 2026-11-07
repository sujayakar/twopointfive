//! What the parts of `marks` share, and what it hands to the host.
#![allow(unused_variables)]
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};


/// (src/game/marks.ts:39 bulletMark)
pub fn bullet_mark(g: &mut crate::game::Game, hit_p: V3, n: V3, s: Option<crate::marks::Surface2>, def: &crate::weapons::WeaponDef) { g.out.push(crate::out_events::OutEvent::BulletMark(crate::out_events::OutBulletMark { hit_p: hit_p, n: n, s: s, def: def.id })); }

/// (src/game/marks.ts:51 bangScorch)
pub fn bang_scorch(g: &mut crate::game::Game, at: V3) { g.out.push(crate::out_events::OutEvent::BangScorch(crate::out_events::OutBangScorch { at: at })); }

/// (src/game/marks.ts:90 kickScuff)
pub fn kick_scuff(g: &mut crate::game::Game, d: DoorId, from: V3) { g.out.push(crate::out_events::OutEvent::KickScuff(crate::out_events::OutKickScuff { d: d, from: from })); }

/// (src/game/marks.ts:104 chipMark)
pub fn chip_mark(g: &mut crate::game::Game, p: V3, n: V3, s: Option<crate::marks::Surface2>, size: Option<f64>) { g.out.push(crate::out_events::OutEvent::ChipMark(crate::out_events::OutChipMark { p: p, n: n, s: s, size: size })); }
