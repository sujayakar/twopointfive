//! The types of `doornames`.
use crate::{Blank, bytes::ToBytes, ids::*, js::*};
use alloc::{string::String, vec::Vec};

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct DoornamesMod {
    pub written: JsMap<DoorId, String>,
    pub said_up: JsMap<DoorId, String>,
}

pub fn ordinals() -> Vec<String> { alloc::vec![String::from("first"), String::from("second"), String::from("third"), String::from("fourth"), String::from("fifth")] }
