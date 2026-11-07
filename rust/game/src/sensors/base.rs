//! What the parts of `sensors` share, and what it hands to the host.
#![allow(unused_variables)]
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};


/// INVENTED: one ask in the frame's table of the light meter, kept on g.probe so a checkpoint takes it back with the game (render/probe.ts:17 Ask, :22). `pos` is a line's a, `b` its other end, None for a light ask.
#[derive(Clone, Debug)]
pub struct ProbeAsk { pub key: String, pub pos: V3, pub skip_owner: f64, pub b: Option<V3>, pub rank: f64 }
