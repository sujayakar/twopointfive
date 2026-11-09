//! The types of `doornames`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes)]
pub struct DoornamesMod {
    pub said: JsMap<DoorId, String>,
    pub written: JsMap<DoorId, String>,
    pub said_up: JsMap<DoorId, String>,
}
impl DoornamesMod {
    pub fn blank() -> Self {
        DoornamesMod {
            said: JsMap::new(),
            written: JsMap::new(),
            said_up: JsMap::new(),
        }
    }
}

pub fn ordinals() -> Vec<String> { alloc::vec![String::from("first"), String::from("second"), String::from("third"), String::from("fourth"), String::from("fifth")] }
