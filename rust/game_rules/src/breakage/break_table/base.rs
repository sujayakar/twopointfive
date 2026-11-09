//! What the parts of `break_table` share, and what it hands to the host.
#![allow(unused_variables)]
#[allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};


/// INVENTED, for BREAK[mat]: the row of a material. Rows are read only by key, nothing writes them, and they do not borrow from g, so a row may be held across a call that takes g.
pub(in crate::breakage) fn break_row(mat: crate::break_table::Mat) -> &'static crate::break_table::BreakRow { &crate::break_table::break_().iter().find(|e| e.0 == mat).unwrap().1 }
