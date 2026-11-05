#![allow(unused_variables)]
//! Every function and method of `takedown`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/takedown.ts:60 live)
pub fn live(g: &mut crate::game::Game, half: crate::takedown::Half) -> bool { todo!() }

/// (src/game/takedown.ts:63 takeLower)
pub fn take_lower(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/takedown.ts:66 takeKill)
pub fn take_kill(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/takedown.ts:67 takeBeats)
pub fn take_beats(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/takedown.ts:69 takeHaul)
pub fn take_haul(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/takedown.ts:72 blowStyle)
pub fn blow_style(kind: Option<crate::close::CloseKind>, lethal: bool) -> crate::character::BlowStyle { todo!() }

/// (src/game/takedown.ts:77 fallStyle)
pub fn fall_style(style: crate::character::BlowStyle) -> crate::character::FallStyle { todo!() }

/// (src/game/takedown.ts:81 fwd)
pub fn fwd(g: &mut crate::game::Game, c_2: CharId) -> V3 { todo!() }

/// (src/game/takedown.ts:84 marks)
pub fn marks(g: &mut crate::game::Game, by: CharId, vic: CharId, style: crate::character::BlowStyle) -> (V3, Option<V3>) { todo!() }

/// (src/game/takedown.ts:94 mayRise)
pub fn may_rise(g: &mut crate::game::Game, by: CharId) -> bool { todo!() }

/// (src/game/takedown.ts:97 takeBegin)
pub fn take_begin(g: &mut crate::game::Game, by: CharId, gd: GuardId, kind: Option<crate::close::CloseKind>, lethal: bool) { todo!() }

/// (src/game/takedown.ts:107 fallWay)
pub fn fall_way(g: &mut crate::game::Game, vic: CharId, from: Option<V3>, style: crate::character::FallStyle) -> (V3, f64) { todo!() }

/// (src/game/takedown.ts:117 setFall)
pub fn set_fall(g: &mut crate::game::Game, vic: CharId, style: crate::character::FallStyle, by: Option<CharId>, ahead: Option<f64>) { todo!() }

/// (src/game/takedown.ts:136 takeFall)
pub fn take_fall(g: &mut crate::game::Game, gd: GuardId, kind: Option<crate::close::CloseKind>, lethal: bool, how: Option<crate::takedown::TakeFallHow>) { todo!() }

/// (src/game/takedown.ts:144 partnerBegin)
pub fn partner_begin(g: &mut crate::game::Game, mate: GuardId, foe: GuardId, front: bool) { todo!() }

/// (src/game/takedown.ts:151 partnerCancel)
pub fn partner_cancel(g: &mut crate::game::Game, mate: GuardId, foe: GuardId) { todo!() }

/// (src/game/takedown.ts:156 partnerFall)
pub fn partner_fall(g: &mut crate::game::Game, foe: GuardId, front: bool, mate: Option<GuardId>) { todo!() }

/// (src/game/takedown.ts:164 takedownTick)
pub fn takedown_tick(g: &mut crate::game::Game) { todo!() }
