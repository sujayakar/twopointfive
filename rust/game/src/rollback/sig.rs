#![allow(unused_variables)]
//! Every function and method of `rollback`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// INVENTED, in place of the walker: a save of this game. Takes g.saves' shelf out first so a save does not hold every earlier one, clones Game, puts it back (savestate.ts:877: MemShelf is the host's)
pub fn take_game(g: &mut crate::game::Game) -> alloc::boxed::Box<crate::game::Game> { todo!() }

/// INVENTED, in place of the walker: this game becomes the save. What stays as it is now, not as saved: g.saves' shelf and the host's fields (rollback.ts:55 HOST_FIELDS: engine, audio, smoke, journal; HostOwned at game.ts:300, render/probe.ts:24, sensors.ts:73). savestate.ts:1090's e.snap.game !== g has no meaning here: a save is of this game
pub fn put_game(g: &mut crate::game::Game, save: &crate::game::Game) { todo!() }

