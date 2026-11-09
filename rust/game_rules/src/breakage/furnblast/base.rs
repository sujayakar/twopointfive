//! What the parts of `furnblast` share, and what it hands to the host.
#![allow(unused_variables)]
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};


/// INVENTED, for FURN_BREAK[cls] (src/game/furnblast.ts:104): the row of a class, owned, so a caller can hold it across a call that takes g. Nothing writes the table.
pub fn furn_break_row(cls: crate::scene_materials::BreakClass) -> crate::furnblast::BreakRow { crate::furnblast::furn_break().into_iter().find(|e| e.0 == cls).unwrap().1 }

