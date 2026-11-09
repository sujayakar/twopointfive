use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// A row's turn weight: its own `handling`, else its class's, else a carbine's.
pub fn handling_of(def: &crate::weapons::WeaponDef) -> f64 {
    if let Some(h) = def.handling { return h; }
    let table = crate::weapons::handling_by_class();
    if let Some(h) = table.get(String::from(def.cls.as_str())) { return *h; }
    *table.get(String::from(crate::weapons::WeaponClass::Carbine.as_str())).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::weapons::{WeaponDef, WeaponId, WeaponsMod};

    fn table_row(id: WeaponId) -> WeaponDef { WeaponsMod::blank().weapons.get(id).unwrap().clone() }

    /// weapons.ts:345: the row's own number, then its class's; `??` lets a 0 stand.
    #[test]
    fn handling_reads_the_row_then_the_class() {
        assert_eq!(handling_of(&table_row(WeaponId::Fn2000)), 0.8);
        assert_eq!(handling_of(&table_row(WeaponId::Fiveseven)), 1.0);
        assert_eq!(handling_of(&table_row(WeaponId::Aks74u)), 0.82);
        assert_eq!(handling_of(&table_row(WeaponId::Akms)), 0.7);
        assert_eq!(handling_of(&table_row(WeaponId::M870)), 0.75);
        let mut row = table_row(WeaponId::Akms);
        row.handling = Some(0.0);
        assert_eq!(handling_of(&row), 0.0);
    }
}
