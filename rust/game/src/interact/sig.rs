#![allow(unused_variables)]
//! Every function and method of `interact`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/interact.ts:44 emptyThing)
pub fn empty_thing(g: &mut crate::game::Game) -> Option<crate::interact::EmptyThing> { todo!() }

/// (src/game/interact.ts:65 emptyWord)
pub fn empty_word(g: &mut crate::game::Game, t: Option<&crate::interact::EmptyThing>) -> String { todo!() }

/// (src/game/interact.ts:73 moveMask)
pub fn move_mask(g: &mut crate::game::Game, input: InputId) -> f64 { todo!() }

/// (src/game/interact.ts:77 runningVerb)
pub fn running_verb(g: &mut crate::game::Game) -> String { todo!() }

/// (src/game/interact.ts:94 cancelSustained)
pub fn cancel_sustained(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/interact.ts:109 wedged)
pub fn wedged(r: &crate::game::Interactable) -> bool { todo!() }

/// (src/game/interact.ts:111 rowWords)
pub fn row_words(r: &crate::game::Interactable) -> String { todo!() }

/// (src/game/interact.ts:113 wedgeWords)
pub fn wedge_words(r: &crate::game::Interactable) -> String { todo!() }

/// (src/game/interact.ts:124 whistleWedge)
pub fn whistle_wedge(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/interact.ts:127 ringMode)
pub fn ring_mode(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/interact.ts:128 ringStays)
pub fn ring_stays(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/interact.ts:130 wedgeDigit)
pub fn wedge_digit(g: &mut crate::game::Game, input: InputId) -> bool { todo!() }

/// (src/game/interact.ts:132 give)
pub fn give(g: &mut crate::game::Game, r: Option<&crate::game::Interactable>) { todo!() }

/// (src/game/interact.ts:140 handRingView)
pub fn hand_ring_view(g: &mut crate::game::Game) -> Option<crate::cmdring::RingView> { todo!() }

/// (src/game/interact.ts:150 handRingKeys)
pub fn hand_ring_keys(g: &mut crate::game::Game) -> f64 { todo!() }

/// (src/game/interact.ts:157 escapeInteract)
pub fn escape_interact(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/interact.ts:164 handsBusy)
pub fn hands_busy(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/interact.ts:166 backableInteract)
pub fn backable_interact(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/interact.ts:168 choosable)
pub fn choosable(rows: &[crate::game::Interactable]) -> bool { todo!() }

/// (src/game/interact.ts:169 openSelector)
pub fn open_selector(g: &mut crate::game::Game) { todo!() }

/// (src/game/interact.ts:176 shutList)
pub fn shut_list(g: &mut crate::game::Game) { todo!() }

/// (src/game/interact.ts:182 closeSelector)
pub fn close_selector(g: &mut crate::game::Game, spent: bool) { todo!() }

/// (src/game/interact.ts:189 ringSpot)
pub fn ring_spot(g: &mut crate::game::Game) -> V3 { todo!() }

/// (src/game/interact.ts:193 ringOnThing)
pub fn ring_on_thing(g: &mut crate::game::Game, r: &mut crate::cmdring::RingState, input: InputId, cam: CameraId, view: crate::host_hud::View, open: Option<bool>) -> bool { todo!() }

/// (src/game/interact.ts:205 updateInteract)
pub fn update_interact(g: &mut crate::game::Game, dt: f64, input: InputId, view: Option<crate::host_hud::View>, cam: Option<CameraId>) { todo!() }
