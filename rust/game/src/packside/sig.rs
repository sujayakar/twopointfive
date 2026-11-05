#![allow(unused_variables)]
//! Every function and method of `packside`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/packside.ts:53 memoOf)
pub fn memo_of(g: &mut crate::game::Game, ch: &crate::packside::PackChar) -> Option<crate::packside::Memo> { todo!() }

/// (src/game/packside.ts:73 packOf)
pub fn pack_of(g: &mut crate::game::Game, ch: &crate::packside::PackChar, mask: f64) -> Option<crate::packside::PackExt> { todo!() }

/// (src/game/packside.ts:81 packMask)
pub fn pack_mask(g: &mut crate::game::Game, ch: &crate::packside::PackChar, mask: f64, carries: bool) -> f64 { todo!() }

/// (src/game/packside.ts:95 sideBind)
pub fn side_bind(g: &mut crate::game::Game, e: crate::packside::PackExt, what: crate::packside::SideBindWhat, dims: crate::packside::SideBindDims) -> crate::packside::SidePlace { todo!() }

/// (src/game/packside.ts:109 backLine)
pub fn back_line(ch: &crate::packside::PackChar, m: &mut crate::packside::Memo, x: f64) -> Vec<crate::packside::BackPt> { todo!() }

/// (src/game/packside.ts:121 sidePitch)
pub fn side_pitch(g: &mut crate::game::Game, ch: &crate::packside::PackChar, p: crate::packside::SidePlace, jd: &[f64]) -> f64 { todo!() }

/// (src/game/packside.ts:133 sidePitched)
pub fn side_pitched(p: crate::packside::SidePlace, th: f64) -> crate::packside::SidePlace { todo!() }

/// (src/game/packside.ts:140 sideSkinned)
pub fn side_skinned(p: crate::packside::SidePlace, jd: &[f64], at: f64) -> crate::packside::SidePlace { todo!() }

/// (src/game/packside.ts:146 chestJoint)
pub fn chest_joint(g: &mut crate::game::Game, ch: &crate::packside::PackChar) -> f64 { todo!() }

/// (src/game/packside.ts:148 sideLocal)
pub fn side_local(g: &mut crate::game::Game, ch: &crate::packside::PackChar, p: crate::packside::SidePlace) -> Option<crate::packside::SidePlace> { todo!() }
