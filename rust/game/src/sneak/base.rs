//! What the parts of `sneak` share, and what it hands to the host.
#![allow(unused_variables)]
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};


/// INVENTED: one way a defender looks in a SneakWalk, as he looks now or as he mostly looks, each also WIDER (sneak.ts:352, :359)
#[derive(Clone, Copy, Debug)]
pub struct SneakWalkLook { pub e: crate::guards::Eye, pub ew: crate::guards::Eye, pub lx: f64, pub lz: f64 }
/// INVENTED: one defender in a SneakWalk (sneak.ts:352, :362). His `kept` lines stay in g.m.sneak.kept_lines, read by his id
#[derive(Clone, Debug)]
pub struct SneakWalkMan { pub h: GuardId, pub x: f64, pub z: f64, pub open: crate::guards::Eye, pub looks: Vec<crate::sneak::SneakWalkLook>, pub line: JsMap<crate::Num, f64> }
/// INVENTED: what the eye test is asked about: a spot a man could stand on, or a man's own body (sneak.ts:266 standIn, :284 exposureOf)
#[derive(Clone, Copy, Debug)]
pub enum Target { Spot { chest: V3, head: V3 }, Man { c: CharId } }
