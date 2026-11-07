#![allow(unused_variables)]
//! Every function and method of `kitanim`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/kitanim.ts:87 note)
pub fn note(g: &mut crate::game::Game, row: crate::kitanim::NoteRow) { todo!() }

/// (src/game/kitanim.ts:89 live)
pub fn live(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/kitanim.ts:95 nvHandFree)
pub fn nv_hand_free(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/kitanim.ts:104 kitNv)
pub fn kit_nv(g: &mut crate::game::Game) { todo!() }

/// (src/game/kitanim.ts:112 kitNvEye)
pub fn kit_nv_eye(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/kitanim.ts:124 handsOff)
pub fn hands_off(g: &mut crate::game::Game, an: &crate::character::Anim) -> bool { todo!() }

/// (src/game/kitanim.ts:125 kitInHand)
pub fn kit_in_hand(g: &mut crate::game::Game) -> Option<crate::character::KitItem> { todo!() }

/// (src/game/kitanim.ts:139 kitSlingFront)
pub fn kit_sling_front(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/kitanim.ts:147 kitSlingRest)
pub fn kit_sling_rest(g: &mut crate::game::Game) -> Option<f64> { todo!() }

/// (src/game/kitanim.ts:156 kitCloseness)
pub fn kit_closeness(g: &mut crate::game::Game, d: f64) -> f64 { todo!() }

/// (src/game/kitanim.ts:159 kitTick)
pub fn kit_tick(g: &mut crate::game::Game, late: Option<bool>) { todo!() }

/// (src/game/kitanim.ts:210 kitClick)
pub fn kit_click(g: &mut crate::game::Game, input: InputId) { todo!() }

/// (src/game/kitanim.ts:225 kitLockMax)
pub fn kit_lock_max(kit: &crate::kitanim::KitLockMaxKIT) -> f64 { todo!() }

/// (src/game/kitanim.ts:240 kitShow)
pub fn kit_show(g: &mut crate::game::Game, v: &mut crate::kitanim::KitShowV) -> Option<crate::rig_props::KitDraw> { todo!() }

/// (src/game/kitanim.ts:264 kitGunUp)
pub fn kit_gun_up(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/kitanim.ts:268 kitReload)
pub fn kit_reload(g: &mut crate::game::Game, gun: FirearmId, long: bool) -> Option<crate::kitanim::KitReload> { todo!() }

/// (src/game/kitanim.ts:283 kitSummary)
pub fn kit_summary(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/kitanim.ts:285 kitReloadNote)
pub fn kit_reload_note(g: &mut crate::game::Game, kind: crate::kitanim::ReloadKind, secs: f64) { todo!() }
