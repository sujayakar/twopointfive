#![allow(unused_variables)]
//! Every function and method of `squadnet`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/squadnet.ts:64 newStats)
pub fn new_stats() -> crate::squadnet::SquadNetStats { todo!() }

/// (src/game/squadnet.ts:70 SquadNet.constructor)
pub fn squad_net_new(g: &mut crate::game::Game) -> crate::squadnet::SquadNet { todo!() }

/// (src/game/squadnet.ts:71 SquadNet.reset)
pub fn squad_net_reset(g: &mut crate::game::Game) { todo!() }

/// (src/game/squadnet.ts:74 SquadNet.line)
pub fn squad_net_line(g: &mut crate::game::Game) { todo!() }

/// (src/game/squadnet.ts:77 SquadNet.tick)
pub fn squad_net_tick(g: &mut crate::game::Game) { todo!() }

/// (src/game/squadnet.ts:98 whyWords)
pub fn why_words(g: &mut crate::game::Game, p: &crate::autopilot::AutoPick) -> Option<crate::squadnet::WhyWords> { todo!() }

/// (src/game/squadnet.ts:110 roomCall)
pub fn room_call(g: &mut crate::game::Game, name: &str, left: f64, inside: bool, seen: bool) -> Option<String> { todo!() }

/// (src/game/squadnet.ts:120 plainCall)
pub fn plain_call(g: &mut crate::game::Game, open: bool, left: f64, inside: bool, seen: bool) -> Option<String> { todo!() }

/// (src/game/squadnet.ts:128 inFlood)
pub fn in_flood(g: &mut crate::game::Game, r: &crate::squadnet::InFloodR, p: V3) -> bool { todo!() }

/// (src/game/squadnet.ts:148 netPlace)
pub fn net_place(g: &mut crate::game::Game, w: f64, h: f64, px: f64, py: f64, mx: f64, my: f64, bw: f64, bh: f64, hits: &mut dyn FnMut(&mut crate::game::Game, f64, f64) -> bool) -> Option<[f64; 2]> { todo!() }
