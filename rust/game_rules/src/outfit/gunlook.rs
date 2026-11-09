pub use ::game_types::gunlook::types;
pub use types::*;
use crate::game::Game;
use crate::ids::*;
use crate::v3::{self, Quat, V3};
use alloc::string::String;

/// What a row has of its own; None: its stand-in fields are the gun.
pub fn gun_own(g: &crate::game::Game, def: &crate::weapons::WeaponDef) -> Option<crate::weapons::WeaponDefOwn> {
    def.own.clone()
}

/// The voice its report is played with.
pub fn voice_of(g: &crate::game::Game, def: &crate::weapons::WeaponDef) -> String {
    match gun_own(g, def).and_then(|own| own.voice) {
        Some(voice) => voice,
        None => def.report.voice.clone(),
    }
}

/// The flame and the flash's light gain it fires with.
pub fn flame_of(g: &crate::game::Game, def: &crate::weapons::WeaponDef) -> crate::effects::MuzzleFxKey {
    match gun_own(g, def).and_then(|own| own.flame) {
        Some(flame) => flame,
        None => def.muzzle_fx,
    }
}

/// A palm under a handguard, fingers wrapped across it; `z`: how far down the bore the wrist is.
fn under(z: f64, x: Option<f64>, y: Option<f64>) -> crate::gunlook::LongModelGripL {
    let x = x.unwrap_or(0.099);
    let y = y.unwrap_or(-0.059);
    LongModelGripL { off: [x, y, z], fingers: [-0.48, -0.11, 0.19], back: [0.45, -0.85, -0.25] }
}

/// A long gun's model by its id; None: the group draws its own parts.
pub(super) fn long_model(g: &crate::game::Game, id_2: Option<crate::weapons::GunModelId>) -> Option<crate::gunlook::LongModel> {
    long_row(id_2?)
}

/// A hand gun's model by its id.
pub(super) fn pistol_model(g: &crate::game::Game, id_2: Option<crate::weapons::GunModelId>) -> Option<crate::gunlook::PistolModel> {
    pistol_row(id_2?)
}

/// Which models are on a man, from the rows he carries: the one in his hands and the others.
pub fn gun_look_of(g: &crate::game::Game, into: Option<crate::gunlook::GunLook>, in_hands: Option<&crate::weapons::WeaponDef>, other: Option<&crate::weapons::WeaponDef>, third: Option<&crate::weapons::WeaponDef>) -> Option<crate::gunlook::GunLook> {
    use crate::weapons::{WeaponClass, WeaponDefRig, WeaponId};
    let mut l = into.unwrap_or(GunLook { hand: None, long: None, side: None, bare_long: None, bare_side: None });
    l.hand = in_hands.and_then(|d| d.own.as_ref()).map(|own| own.model);
    l.long = None;
    l.side = None;
    l.bare_long = Some(false);
    l.bare_side = Some(false);
    let in_hands_id = in_hands.map(|d| d.id);
    for d in [third, other, in_hands] {
        let Some(d) = d else { continue };
        let model = d.own.as_ref().map(|own| own.model);
        // bare: the two groups that draw a can, on a row that says it has none
        let bare = model.is_none() && !d.suppressed && (d.rig == WeaponDefRig::Sc20k || d.rig == WeaponDefRig::Pistol);
        if d.cls == WeaponClass::Pistol {
            l.side = model;
            l.bare_side = Some(bare);
        } else if !(Some(d.id) == in_hands_id && third.is_some() && d.id == WeaponId::M870) {
            // the big slot's pump gun in his hands is not his long gun: that one, `third`, is on its sling
            l.long = model;
            l.bare_long = Some(bare);
        }
    }
    Some(l)
}

/// LONG_MODELS[id], from the skeleton's table.
fn long_row(id: crate::weapons::GunModelId) -> Option<LongModel> {
    use crate::weapons::GunModelId as Id;
    let t = long_models();
    match id {
        Id::M870 => t.m870,
        Id::Mk12 => t.mk12,
        Id::Mp7 => t.mp7,
        Id::Mk46 => t.mk46,
        Id::M1014 => t.m1014,
        Id::Hk45 => t.hk45,
        Id::G18 => t.g18,
    }
}

/// PISTOL_MODELS[id], from the skeleton's table.
fn pistol_row(id: crate::weapons::GunModelId) -> Option<PistolModel> {
    use crate::weapons::GunModelId as Id;
    let t = pistol_models();
    match id {
        Id::M870 => t.m870,
        Id::Mk12 => t.mk12,
        Id::Mp7 => t.mp7,
        Id::Mk46 => t.mk46,
        Id::M1014 => t.m1014,
        Id::Hk45 => t.hk45,
        Id::G18 => t.g18,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::weapons::GunModelId as Id;
    use crate::weapons::{WeaponDef, WeaponId as W};

    fn row(g: &Game, id: W) -> WeaponDef { g.m.weapons.weapons.get(id).unwrap().clone() }

    /// A FAILURE MEANS: a gun is drawn twice over, or finds no model at all.
    #[test]
    fn a_gun_is_a_long_gun_or_a_pistol_never_both() {
        for id in [Id::Mk12, Id::Mp7, Id::Mk46, Id::M1014, Id::M870] { assert!(long_row(id).is_some() && pistol_row(id).is_none()); }
        for id in [Id::Hk45, Id::G18] { assert!(pistol_row(id).is_some() && long_row(id).is_none()); }
    }

    /// (his hands, his other gun, the big slot's) to (in hand, long, side, long bare, side bare)
    fn look(a: Option<W>, b: Option<W>, c: Option<W>) -> (Option<Id>, Option<Id>, Option<Id>, bool, bool) {
        let g = Game::blank();
        let (a, b, c) = (a.map(|x| row(&g, x)), b.map(|x| row(&g, x)), c.map(|x| row(&g, x)));
        let l = gun_look_of(&g, None, a.as_ref(), b.as_ref(), c.as_ref()).unwrap();
        (l.hand, l.long, l.side, l.bare_long.unwrap(), l.bare_side.unwrap())
    }

    /// A FAILURE MEANS: a man's guns change as he swaps: the long gun and the sidearm are the same two whichever is up.
    #[test]
    fn a_man_shows_his_long_gun_and_his_sidearm_whichever_he_holds() {
        assert_eq!(look(Some(W::Mk12), Some(W::Hk45), None), (Some(Id::Mk12), Some(Id::Mk12), Some(Id::Hk45), false, false));
        assert_eq!(look(Some(W::Hk45), Some(W::Mk12), None), (Some(Id::Hk45), Some(Id::Mk12), Some(Id::Hk45), false, false));
    }

    /// A FAILURE MEANS: the rifle vanishes while he works the big slot's pump gun: the pump in his hands is his long gun only if nothing is on the sling.
    #[test]
    fn the_big_slots_pump_gun_in_his_hands_leaves_his_long_gun_on_its_sling() {
        assert_eq!(look(Some(W::M870), None, Some(W::Mk12)), (Some(Id::M870), Some(Id::Mk12), None, false, false));
        assert_eq!(look(Some(W::M870), None, None), (Some(Id::M870), Some(Id::M870), None, false, false));
        assert_eq!(look(Some(W::M870), Some(W::Hk45), Some(W::Mp7)), (Some(Id::M870), Some(Id::Mp7), Some(Id::Hk45), false, false));
    }

    /// A FAILURE MEANS: a gun issued without a can is drawn with one, or the other way round.
    #[test]
    fn a_gun_that_has_no_can_is_drawn_bare() {
        assert_eq!(look(Some(W::Mk18bare), Some(W::G19bare), None), (None, None, None, true, true));
        assert_eq!(look(Some(W::Fn2000), Some(W::Fiveseven), None), (None, None, None, false, false));
    }

    /// A FAILURE MEANS: the last man's look is left on a man with nothing in his hands.
    #[test]
    fn a_look_handed_in_is_written_over_whole() {
        let g = Game::blank();
        let stale = GunLook { hand: Some(Id::G18), long: Some(Id::G18), side: Some(Id::G18), bare_long: Some(true), bare_side: Some(true) };
        let l = gun_look_of(&g, Some(stale), None, None, None).unwrap();
        assert_eq!((l.hand, l.long, l.side, l.bare_long, l.bare_side), (None, None, None, Some(false), Some(false)));
    }

    /// A FAILURE MEANS: a gun sounds or flashes as another: a gun with a model of its own has its own voice and flame, the can is heard, and the rest speak by their row.
    #[test]
    fn a_gun_sounds_and_flashes_as_its_own_or_by_its_row() {
        let g = Game::blank();
        let says = |id: W| { let d = row(&g, id); (voice_of(&g, &d), String::from(flame_of(&g, &d).as_str())) };
        let pair = |v: &str, f: &str| (String::from(v), String::from(f));
        assert_eq!(says(W::Mk12), pair("mk12Suppressed", "mk12"));
        assert_eq!(says(W::G18), pair("g18Report", "g18"));
        assert_eq!(says(W::Fiveseven), pair("pistolSuppressed", "pistolSuppressed"));
        assert_eq!(says(W::G19bare), pair("pistolLoud", "pistolBare"));
        assert_eq!(says(W::Fn2000), pair("rifleSuppressed", "sc20k"));
        assert_eq!(says(W::Mk18bare), pair("carbineReport", "carbine"));
    }
}
