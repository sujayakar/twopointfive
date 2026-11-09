//! The types of `gear`.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, Default)]
pub struct GEARNAMES {
    pub rifle: String,
    pub rifle_long: String,
    pub pistol: String,
    pub pistol_long: String,
    pub pistol_calibre: String,
}

#[derive(Clone, Debug, Default)]
pub struct OLDNAMES {
    pub rifle: String,
    pub rifle_long: String,
    pub pistol: String,
    pub pistol_long: String,
    pub pistol_calibre: String,
}

/// (src/game/gear.ts:27 GEAR_NAMES: a table nothing writes)
pub fn gear_names() -> crate::gear::GEARNAMES { crate::gear::GEARNAMES { rifle: String::from("Mk 18"), rifle_long: String::from("Mk 18 (suppressed)"), pistol: String::from("Glock 19"), pistol_long: String::from("Glock 19 (suppressed)"), pistol_calibre: String::from("9×19 mm") } }

/// (src/game/gear.ts:31 OLD_NAMES: a table nothing writes)
pub fn old_names() -> crate::gear::OLDNAMES { crate::gear::OLDNAMES { rifle: String::from("FN2000"), rifle_long: String::from("FN2000 (suppressed)"), pistol: String::from("Five-seveN"), pistol_long: String::from("FN Five-seveN (suppressed)"), pistol_calibre: String::from("5.7×28 mm") } }
