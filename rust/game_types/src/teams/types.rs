//! The types of `teams`.
#![allow(unused_imports)]
use crate::{Blank, Words, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, ToBytes, Default, Words)]
pub enum Team {
    #[default] Guard,
    Operative,
    Hostile,
    Player,
    Civilian,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct TeamsMod {
    #[is({ let mut t = JsMap::new(); t.set(crate::teams::Team::Guard, { let mut t = JsMap::new(); t.set(crate::teams::Team::Guard, false); t.set(crate::teams::Team::Operative, false); t.set(crate::teams::Team::Hostile, false); t.set(crate::teams::Team::Player, true); t.set(crate::teams::Team::Civilian, false); t }); t.set(crate::teams::Team::Operative, { let mut t = JsMap::new(); t.set(crate::teams::Team::Guard, false); t.set(crate::teams::Team::Operative, false); t.set(crate::teams::Team::Hostile, true); t.set(crate::teams::Team::Player, false); t.set(crate::teams::Team::Civilian, false); t }); t.set(crate::teams::Team::Hostile, { let mut t = JsMap::new(); t.set(crate::teams::Team::Guard, false); t.set(crate::teams::Team::Operative, true); t.set(crate::teams::Team::Hostile, false); t.set(crate::teams::Team::Player, true); t.set(crate::teams::Team::Civilian, false); t }); t.set(crate::teams::Team::Player, { let mut t = JsMap::new(); t.set(crate::teams::Team::Guard, true); t.set(crate::teams::Team::Operative, false); t.set(crate::teams::Team::Hostile, true); t.set(crate::teams::Team::Player, false); t.set(crate::teams::Team::Civilian, false); t }); t.set(crate::teams::Team::Civilian, { let mut t = JsMap::new(); t.set(crate::teams::Team::Guard, false); t.set(crate::teams::Team::Operative, false); t.set(crate::teams::Team::Hostile, false); t.set(crate::teams::Team::Player, false); t.set(crate::teams::Team::Civilian, false); t }); t })] pub hostile: JsMap<crate::teams::Team, JsMap<crate::teams::Team, bool>>,
    pub staff_ops: f64,
}

