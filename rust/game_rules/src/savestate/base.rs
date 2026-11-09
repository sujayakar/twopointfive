//! What the parts of `savestate` share, and what it hands to the host.
#![allow(unused_variables)]
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};


pub fn checkpoints_request(g: &mut crate::game::Game, why: &str, label: &str) { g.out.push(crate::out_events::OutEvent::CheckpointsRequest(crate::out_events::OutCheckpointsRequest { why: String::from(why), label: String::from(label) })); }








pub fn checkpoints_keep(g: &mut crate::game::Game, label: &str) { g.out.push(crate::out_events::OutEvent::CheckpointsKeep(crate::out_events::OutCheckpointsKeep { label: String::from(label) })); }











