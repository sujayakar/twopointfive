//! The types of `gear`.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, Default)]
pub struct GEARNAMES {
    pub rifle: String,
    pub pistol: String,
}

#[derive(Clone, Debug, Default)]
pub struct OLDNAMES {
    pub rifle: String,
    pub pistol: String,
}

pub fn gear_names() -> crate::gear::GEARNAMES { crate::gear::GEARNAMES { rifle: String::from("Mk 18"), pistol: String::from("Glock 19") } }

pub fn old_names() -> crate::gear::OLDNAMES { crate::gear::OLDNAMES { rifle: String::from("FN2000"), pistol: String::from("Five-seveN") } }
