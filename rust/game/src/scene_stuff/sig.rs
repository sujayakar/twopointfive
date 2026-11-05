#![allow(unused_variables)]
//! Every function and method of `scene_stuff`, with an empty body. Written by the skeleton's program.
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

/// (src/scene/stuff.ts:118 record)
pub fn record(name: &str, finish: Option<crate::scene_stuff::Finish>, stuff: Option<crate::scene_stuff::BoxStuff>, does: Option<crate::scene_materials::DoesWord>) -> crate::scene_stuff::Paint { todo!() }

/// (src/scene/stuff.ts:120 fin)
pub fn fin(p: &crate::scene_stuff::Paint) -> String { todo!() }

/// (src/scene/stuff.ts:134 refuse)
pub fn refuse(g: &mut crate::game::Game, row: &[f64], was: &crate::scene_stuff::Paint, next: &crate::scene_stuff::Paint) -> bool { todo!() }

/// (src/scene/stuff.ts:140 keep)
pub fn keep(g: &mut crate::game::Game, row: &[f64], next: &crate::scene_stuff::Paint) { todo!() }

/// (src/scene/stuff.ts:153 sayPaint)
pub fn say_paint(g: &mut crate::game::Game, row: &[f64], name: &str, said: crate::scene_stuff::SayPaintSaid) { todo!() }

/// (src/scene/stuff.ts:163 madeOf)
pub fn made_of(g: &mut crate::game::Game, stuff: crate::scene_stuff::BoxStuff, colour: [f64; 3], finish: Option<crate::scene_stuff::MadeOfFinish>) -> [f64; 3] { todo!() }

/// (src/scene/stuff.ts:183 drawnAs)
pub fn drawn_as(g: &mut crate::game::Game, stuff: crate::scene_stuff::BoxStuff, colour: [f64; 3], finish: Option<crate::scene_stuff::MadeOfFinish>) -> [f64; 3] { todo!() }

/// (src/scene/stuff.ts:196 paintOf)
pub fn paint_of(g: &mut crate::game::Game, albedo: Option<&[f64]>) -> Option<crate::scene_stuff::Paint> { todo!() }

/// (src/scene/stuff.ts:211 wordFor)
pub fn word_for(g: &mut crate::game::Game, albedo: Option<&[f64]>, said: Option<crate::scene_stuff::BoxStuff>, does: Option<crate::scene_materials::DoesWord>) -> crate::scene_stuff::WordFor { todo!() }
