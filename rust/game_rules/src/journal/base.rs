//! What the parts of `journal` share, and what it hands to the host.
#![allow(unused_variables)]
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};


/// (src/game/journal.ts:759 SessionRecorder.cut)
pub fn session_recorder_cut(g: &mut crate::game::Game, why: &str, lost: Option<bool>) { g.out.push(crate::out_events::OutEvent::SessionRecorderCut(crate::out_events::OutSessionRecorderCut { why: String::from(why), lost: lost })); }
