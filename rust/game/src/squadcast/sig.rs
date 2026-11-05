#![allow(unused_variables)]
//! Every function and method of `squadcast`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/squadcast.ts:19 squadCast)
pub fn squad_cast(g: &mut crate::game::Game, of: &crate::orders::OtherCast) -> crate::squadcast::SquadCast { todo!() }

/// INVENTED, for a second cast of the squad's code (squadcast.ts's Proxy): swaps cast.orders with g.orders and cast.tools with g.door_tools, sets g.in_cast, runs f, swaps back. Every method of the cast's pair runs inside it; fields of the pair can be read on the held value. door_tools_new must run inside it too, with the cast's orders already on Game, or its rows land in the player's menu (squadcast.ts:25-26). The holder takes its cast out (Option::take) for the call and puts it back. A table kept per view (seats::view_key) keeps a row of its own for a cast.
pub fn with_cast<R>(g: &mut crate::game::Game, cast: &mut crate::squadcast::SquadCast, f: &mut dyn FnMut(&mut crate::game::Game) -> R) -> R { todo!() }

/// INVENTED: a ring of keys as a door asks it. every_key() is the set holding "*" (squadcast.ts:17 is a set whose has() is always true): a ring opens what it names, and everything if it names "*"
pub fn ring_has(ring: &JsSet<String>, key: &str) -> bool { todo!() }

