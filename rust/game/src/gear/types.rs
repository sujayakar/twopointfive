//! The types of `gear`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: inline at src/game/gear.ts:27 GEAR_NAMES, src/game/gear.ts:27)
#[derive(Clone, Debug)]
pub struct GEARNAMES {
    pub rifle: String,
    pub rifle_long: String,
    pub pistol: String,
    pub pistol_long: String,
    pub pistol_calibre: String,
}
impl GEARNAMES {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GEARNAMES {
            rifle: String::new(),
            rifle_long: String::new(),
            pistol: String::new(),
            pistol_long: String::new(),
            pistol_calibre: String::new(),
        }
    }
}
impl Default for GEARNAMES { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/gear.ts:29 GEAR_MAG, src/game/gear.ts:29)
#[derive(Clone, Copy, Debug)]
pub struct GEARMAG {
    pub pistol: f64,
}
impl GEARMAG {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GEARMAG {
            pistol: 0.0,
        }
    }
}
impl Default for GEARMAG { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/gear.ts:31 OLD_NAMES, src/game/gear.ts:31)
#[derive(Clone, Debug)]
pub struct OLDNAMES {
    pub rifle: String,
    pub rifle_long: String,
    pub pistol: String,
    pub pistol_long: String,
    pub pistol_calibre: String,
}
impl OLDNAMES {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        OLDNAMES {
            rifle: String::new(),
            rifle_long: String::new(),
            pistol: String::new(),
            pistol_long: String::new(),
            pistol_calibre: String::new(),
        }
    }
}
impl Default for OLDNAMES { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/gear.ts:33 plainShort(def), src/game/gear.ts:33)
#[derive(Clone, Debug)]
pub struct PlainShortDef {
    pub id: String,
    pub short: String,
}
impl PlainShortDef {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PlainShortDef {
            id: String::new(),
            short: String::new(),
        }
    }
}
impl Default for PlainShortDef { fn default() -> Self { Self::zeroed() } }

/// (src/game/gear.ts:27 GEAR_NAMES: a table nothing writes)
pub fn gear_names() -> crate::gear::GEARNAMES { crate::gear::GEARNAMES { rifle: String::from("Mk 18"), rifle_long: String::from("Mk 18 (suppressed)"), pistol: String::from("Glock 19"), pistol_long: String::from("Glock 19 (suppressed)"), pistol_calibre: String::from("9×19 mm") } }

/// (src/game/gear.ts:29 GEAR_MAG: a table nothing writes)
pub fn gear_mag() -> crate::gear::GEARMAG { crate::gear::GEARMAG { pistol: 15.0 } }

/// (src/game/gear.ts:31 OLD_NAMES: a table nothing writes)
pub fn old_names() -> crate::gear::OLDNAMES { crate::gear::OLDNAMES { rifle: String::from("FN2000"), rifle_long: String::from("FN2000 (suppressed)"), pistol: String::from("Five-seveN"), pistol_long: String::from("FN Five-seveN (suppressed)"), pistol_calibre: String::from("5.7×28 mm") } }
