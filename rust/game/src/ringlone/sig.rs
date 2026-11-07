#![allow(unused_variables)]
//! Every function and method of `ringlone`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/game/ringlone.ts:45 wedgeMid)
pub fn wedge_mid(key: f64, n: f64) -> f64 { todo!() }

/// (src/game/ringlone.ts:47 loneWedge)
pub fn lone_wedge(g: &mut crate::game::Game, v: Option<&crate::cmdring::RingView>) -> Option<crate::cmdring::RingViewWedgesItem> { todo!() }

/// (src/game/ringlone.ts:49 loneKey)
pub fn lone_key(g: &mut crate::game::Game, keys: &mut dyn FnMut(&mut crate::game::Game) -> f64) -> f64 { todo!() }

/// (src/game/ringlone.ts:51 wedgeWords)
pub fn wedge_words(w: &crate::ringlone::Wedge) -> String { todo!() }

/// (src/game/ringlone.ts:58 boxOf)
pub fn box_of(key: f64, n: f64, text_w: f64, tall: Option<bool>) -> crate::ringlone::BoxOf { todo!() }

/// (src/game/ringlone.ts:66 loneLines)
pub fn lone_lines(g: &mut crate::game::Game, w: &crate::ringlone::Wedge) -> Vec<String> { todo!() }

/// (src/game/ringlone.ts:76 wordsW)
pub fn words_w(s: &str) -> f64 { todo!() }

/// (src/game/ringlone.ts:78 lonePill)
pub fn lone_pill(g: &mut crate::game::Game, w: &crate::ringlone::Wedge, n: f64) -> crate::ringlone::LonePill { todo!() }

/// (src/game/ringlone.ts:86 pillBox)
pub fn pill_box(g: &mut crate::game::Game, key: f64, n: f64) -> crate::ringlone::BoxOf { todo!() }

/// (src/game/ringlone.ts:88 onPill)
pub fn on_pill(g: &mut crate::game::Game, key: f64, n: f64, x: f64, y: f64) -> bool { todo!() }
