//! What the parts of `host_api` share, and what it hands to the host.
#![allow(unused_variables)]
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};


pub fn host_api_restart_picture(g: &mut crate::game::Game, why: &str) { g.out.push(crate::out_events::OutEvent::HostApiRestartPicture(crate::out_events::OutHostApiRestartPicture {  })); }
