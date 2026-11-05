#![allow(unused_variables)]
//! Every function and method of `quietfog`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/quietfog.ts:46 soloFogRow)
pub fn solo_fog_row(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/quietfog.ts:50 soloFogLink)
pub fn solo_fog_link(g: &mut crate::game::Game, q: crate::Opaque) -> bool { todo!() }

/// (src/game/quietfog.ts:55 quietWand)
pub fn quiet_wand(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/quietfog.ts:64 isRung)
pub fn is_rung(v: Option<&str>) -> bool { todo!() }

/// (src/game/quietfog.ts:66 quietFogWords)
pub fn quiet_fog_words(rung: Option<&str>) -> String { todo!() }

/// (src/game/quietfog.ts:72 quietFogDefault)
pub fn quiet_fog_default(g: &mut crate::game::Game) -> crate::quietfog::QuietRung { todo!() }

/// (src/game/quietfog.ts:74 quietFog)
pub fn quiet_fog(g: &mut crate::game::Game) -> crate::quietfog::QuietRung { todo!() }

/// (src/game/quietfog.ts:89 quietCamsDefault)
pub fn quiet_cams_default(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/quietfog.ts:90 quietCams)
pub fn quiet_cams(g: &mut crate::game::Game) -> bool { todo!() }

/// (src/game/quietfog.ts:95 setQuietCams)
pub fn set_quiet_cams(g: &mut crate::game::Game, on: bool, keep: Option<bool>) { todo!() }

/// (src/game/quietfog.ts:102 quietCamsOfQuery)
pub fn quiet_cams_of_query(g: &mut crate::game::Game, q: crate::Opaque, lesson: Option<bool>) -> f64 { todo!() }

/// (src/game/quietfog.ts:109 setQuietFog)
pub fn set_quiet_fog(g: &mut crate::game::Game, rung: crate::quietfog::QuietRung, keep: Option<bool>) { todo!() }

/// (src/game/quietfog.ts:120 quietFogForget)
pub fn quiet_fog_forget(g: &mut crate::game::Game) { todo!() }

/// (src/game/quietfog.ts:123 quietFogRow)
pub fn quiet_fog_row(g: &mut crate::game::Game, e: Option<&crate::quietfog::QuietFogRowE>) -> bool { todo!() }

/// (src/game/quietfog.ts:127 quietFogHref)
pub fn quiet_fog_href(g: &mut crate::game::Game, href: &str, quiet: bool) -> String { todo!() }

/// (src/game/quietfog.ts:141 quietFogOfQuery)
pub fn quiet_fog_of_query(g: &mut crate::game::Game, q: crate::Opaque, lesson: Option<bool>) -> crate::quietfog::QuietRung { todo!() }

/// (src/game/quietfog.ts:150 lift)
pub fn lift(g: &mut crate::game::Game, h: &mut crate::quietfog::Held) { todo!() }

/// (src/game/quietfog.ts:156 quietFogStart)
pub fn quiet_fog_start(g: &mut crate::game::Game, q: crate::Opaque, rung: crate::quietfog::QuietRung) { todo!() }

/// (src/game/quietfog.ts:164 quietFogEnd)
pub fn quiet_fog_end(g: &mut crate::game::Game) { todo!() }

/// (src/game/quietfog.ts:175 quietFogAside)
pub fn quiet_fog_aside<T: Clone + core::fmt::Debug>(g: &mut crate::game::Game, fn_: &mut dyn FnMut(&mut crate::game::Game) -> T) -> T { todo!() }

/// (src/game/quietfog.ts:184 quietFogHeld)
pub fn quiet_fog_held(g: &mut crate::game::Game) -> bool { todo!() }
