//! src/game/weapons.ts lines 1 to 350
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// The same gun with its suppressor off: every field of `of`'s row, with `over`'s laid on top. (weapons.ts bareRow)
pub fn bare_row(of: &crate::weapons::WeaponDef, over: &crate::weapons::BareRowOver) -> crate::weapons::WeaponDef {
    let mut row = of.clone();
    if let Some(v) = over.id { row.id = v; }
    if let Some(v) = over.enabled { row.enabled = v; }
    if let Some(v) = &over.name { row.name = v.clone(); }
    if let Some(v) = &over.short { row.short = v.clone(); }
    if let Some(v) = &over.calibre { row.calibre = v.clone(); }
    if let Some(v) = over.cls { row.cls = v; }
    if let Some(v) = over.mag_capacity { row.mag_capacity = v; }
    if let Some(v) = over.spare_mags { row.spare_mags = v; }
    if let Some(v) = over.top_up { row.top_up = v; }
    if let Some(v) = &over.reload { row.reload = v.clone(); }
    if let Some(v) = &over.modes { row.modes = v.clone(); }
    if let Some(v) = over.cd { row.cd = v; }
    if let Some(v) = over.cd_jitter { row.cd_jitter = v; }
    if let Some(v) = &over.burst { row.burst = Some(v.clone()); }
    if let Some(v) = &over.auto { row.auto = Some(v.clone()); }
    if let Some(v) = &over.report { row.report = v.clone(); }
    if let Some(v) = over.suppressed { row.suppressed = v; }
    if let Some(v) = over.muzzle_fx { row.muzzle_fx = v; }
    if let Some(v) = over.prop_impulse { row.prop_impulse = v; }
    if let Some(v) = &over.ai_spread { row.ai_spread = v.clone(); }
    if let Some(v) = over.damage { row.damage = over_damage(v); }
    if let Some(v) = over.muzzle_ahead { row.muzzle_ahead = v; }
    if let Some(v) = over.rig { row.rig = over_rig(v); }
    if let Some(v) = over.handling { row.handling = Some(v); }
    if let Some(v) = over.light { row.light = over_light(v); }
    if let Some(v) = over.drop { row.drop = v; }
    if let Some(v) = &over.tracer { row.tracer = Some(v.clone()); }
    if let Some(v) = &over.pellets { row.pellets = Some(v.clone()); }
    if let Some(v) = &over.hands { row.hands = Some(v.clone()); }
    if let Some(v) = over.launcher { row.launcher = Some(v); }
    if let Some(v) = over.weight_kg { row.weight_kg = v; }
    if let Some(v) = over.bulk { row.bulk = v; }
    if let Some(v) = &over.own { row.own = Some(v.clone()); }
    row
}

/// A row's turn weight: its own `handling`, else its class's, else a carbine's. (weapons.ts handlingOf)
pub fn handling_of(def: &crate::weapons::WeaponDef) -> f64 {
    if let Some(h) = def.handling { return h; }
    let table = crate::weapons::handling_by_class();
    if let Some(h) = table.get(String::from(def.cls.as_str())) { return *h; }
    *table.get(String::from(crate::weapons::WeaponClass::Carbine.as_str())).unwrap()
}

/// `over.damage`'s word as the row's type has it (the generator makes one enum a holder).
fn over_damage(v: crate::weapons::BareRowOverDamage) -> crate::weapons::WeaponDefDamage {
    match v {
        crate::weapons::BareRowOverDamage::OneShot => crate::weapons::WeaponDefDamage::OneShot,
    }
}

/// `over.rig`'s word as the row's type has it.
fn over_rig(v: crate::weapons::BareRowOverRig) -> crate::weapons::WeaponDefRig {
    match v {
        crate::weapons::BareRowOverRig::Pistol => crate::weapons::WeaponDefRig::Pistol,
        crate::weapons::BareRowOverRig::Carbine => crate::weapons::WeaponDefRig::Carbine,
        crate::weapons::BareRowOverRig::GuardPistol => crate::weapons::WeaponDefRig::GuardPistol,
        crate::weapons::BareRowOverRig::Sc20k => crate::weapons::WeaponDefRig::Sc20k,
    }
}

/// `over.light`'s word as the row's type has it.
fn over_light(v: crate::weapons::BareRowOverLight) -> crate::weapons::WeaponDefLight {
    match v {
        crate::weapons::BareRowOverLight::None => crate::weapons::WeaponDefLight::None,
        crate::weapons::BareRowOverLight::Rail => crate::weapons::WeaponDefLight::Rail,
        crate::weapons::BareRowOverLight::Handguard => crate::weapons::WeaponDefLight::Handguard,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::weapons::{BareRowOver, WeaponDef, WeaponDefReport, WeaponId, WeaponsMod};

    fn shown(row: &WeaponDef) -> String { alloc::format!("{:?}", row) }

    fn table_row(id: WeaponId) -> WeaponDef { WeaponsMod::blank().weapons.get(id).unwrap().clone() }

    /// weapons.ts:328: the two callers' literals rebuild the rows the skeleton's table holds, field for field.
    #[test]
    fn bare_row_rebuilds_the_tables_bare_rows() {
        let fn2000 = table_row(WeaponId::Fn2000);
        let over = BareRowOver {
            id: Some(WeaponId::Mk18bare), name: Some(fn2000.short.clone()), suppressed: Some(false),
            report: Some(WeaponDefReport { level: 1.0, loud: true, voice: String::from("carbineReport") }),
            muzzle_fx: Some(crate::effects::MuzzleFxKey::Carbine), prop_impulse: Some(6.0), weight_kg: Some(3.4),
            ..Default::default()
        };
        assert_eq!(shown(&bare_row(&fn2000, &over)), shown(&table_row(WeaponId::Mk18bare)));

        let fiveseven = table_row(WeaponId::Fiveseven);
        let over = BareRowOver {
            id: Some(WeaponId::G19bare), name: Some(fiveseven.short.clone()), suppressed: Some(false),
            report: Some(WeaponDefReport { level: 1.0, loud: true, voice: String::from("pistolLoud") }),
            muzzle_fx: Some(crate::effects::MuzzleFxKey::PistolBare), prop_impulse: Some(6.0), weight_kg: Some(0.8),
            ..Default::default()
        };
        assert_eq!(shown(&bare_row(&fiveseven, &over)), shown(&table_row(WeaponId::G19bare)));
    }

    /// An empty `over` is the row, id and all.
    #[test]
    fn bare_row_with_nothing_over_is_the_row() {
        let m870 = table_row(WeaponId::M870);
        assert_eq!(shown(&bare_row(&m870, &BareRowOver::default())), shown(&m870));
    }

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
