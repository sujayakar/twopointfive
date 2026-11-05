//! src/game/gunlook.ts lines 1 to 208
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// The switch as every reader asks it: settled on, on the page.   (gunlook.ts gunLookOn)
pub fn gun_look_on(g: &mut crate::game::Game) -> bool { true }

/// The shells key under it: settled on, on the page.   (gunlook.ts gunShellsOn)
pub fn gun_shells_on(g: &mut crate::game::Game) -> bool { true }

/// What a row has of its own; None: its stand-in fields are the gun.   (gunlook.ts gunOwn)
pub fn gun_own(g: &mut crate::game::Game, def: &crate::weapons::WeaponDef) -> Option<crate::weapons::WeaponDefOwn> {
    def.own.clone()
}

/// The voice its report is played with.   (gunlook.ts voiceOf)
pub fn voice_of(g: &mut crate::game::Game, def: &crate::weapons::WeaponDef) -> String {
    match gun_own(g, def).and_then(|own| own.voice) {
        Some(voice) => voice,
        None => def.report.voice.clone(),
    }
}

/// The flame and the flash's light gain it fires with.   (gunlook.ts flameOf)
pub fn flame_of(g: &mut crate::game::Game, def: &crate::weapons::WeaponDef) -> crate::effects::MuzzleFxKey {
    match gun_own(g, def).and_then(|own| own.flame) {
        Some(flame) => flame,
        None => def.muzzle_fx,
    }
}

/// A palm under a handguard, fingers wrapped across it; `z`: how far down the bore the wrist is.   (gunlook.ts under)
pub fn under(z: f64, x: Option<f64>, y: Option<f64>) -> crate::gunlook::LongModelGripL {
    let x = x.unwrap_or(0.099);
    let y = y.unwrap_or(-0.059);
    LongModelGripL { off: [x, y, z], fingers: [-0.48, -0.11, 0.19], back: [0.45, -0.85, -0.25] }
}

/// A long gun's model by its id; None: the group draws its own parts.   (gunlook.ts longModel)
pub fn long_model(g: &mut crate::game::Game, id_2: Option<crate::weapons::GunModelId>) -> Option<crate::gunlook::LongModel> {
    long_row(id_2?)
}

/// A hand gun's model by its id.   (gunlook.ts pistolModel)
pub fn pistol_model(g: &mut crate::game::Game, id_2: Option<crate::weapons::GunModelId>) -> Option<crate::gunlook::PistolModel> {
    pistol_row(id_2?)
}

/// Which models are on a man, from the rows he carries: the one in his hands and the others.   (gunlook.ts gunLookOf)
pub fn gun_look_of(g: &mut crate::game::Game, into: Option<crate::gunlook::GunLook>, in_hands: Option<&crate::weapons::WeaponDef>, other: Option<&crate::weapons::WeaponDef>, third: Option<&crate::weapons::WeaponDef>) -> Option<crate::gunlook::GunLook> {
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

    /// FNV-1a over 64-bit words, the same fold the TypeScript's tables were put through to get the constants below.
    struct Fold(u64);
    impl Fold {
        fn word(&mut self, w: u64) {
            self.0 ^= w;
            self.0 = self.0.wrapping_mul(0x0000_0100_0000_01b3);
        }
        fn num(&mut self, x: f64) { self.word(x.to_bits()); }
        fn v3(&mut self, v: V3) { for x in v { self.num(x); } }
        fn text(&mut self, s: &str) {
            self.word(s.len() as u64);
            for b in s.bytes() { self.word(b as u64); }
        }
        fn seg(&mut self, s: &Seg) {
            self.num(s.l0); self.num(s.l1); self.num(s.thick); self.num(s.tall);
            match s.adj {
                Some(a) => { self.word(1); self.v3(a.off); self.v3(a.rot); }
                None => self.word(0),
            }
        }
        fn seg_opt(&mut self, s: &Option<Seg>) {
            match s {
                Some(s) => { self.word(1); self.seg(s); }
                None => self.word(0),
            }
        }
    }

    const LONG: [Id; 5] = [Id::Mk12, Id::Mp7, Id::Mk46, Id::M1014, Id::M870];
    const PISTOL: [Id; 2] = [Id::Hk45, Id::G18];

    /// Every number and word of the long guns' table, bit for bit against gunlook.ts LONG_MODELS.
    #[test]
    fn long_table_is_the_typescripts_bits() {
        let mut f = Fold(0xcbf2_9ce4_8422_2325);
        for id in LONG {
            let l = long_row(id).unwrap();
            f.num(l.ahead);
            f.word(l.post as u64);
            f.v3(l.grip_l.off); f.v3(l.grip_l.fingers); f.v3(l.grip_l.back);
            match l.hinge { Some(h) => { f.word(1); f.v3(h); } None => f.word(0) }
            f.seg_opt(&l.lens);
            match l.lens_ahead { Some(a) => { f.word(1); f.num(a); } None => f.word(0) }
            f.word(l.parts.len() as u64);
            for p in &l.parts {
                f.seg(&p.s);
                f.text(p.col.as_str());
                match p.is { Some(i) => { f.word(1); f.text(i.as_str()); } None => f.word(0) }
            }
        }
        assert_eq!(f.0, LONG_BITS);
    }

    /// … and of the hand guns' (gunlook.ts PISTOL_MODELS).
    #[test]
    fn pistol_table_is_the_typescripts_bits() {
        let mut f = Fold(0xcbf2_9ce4_8422_2325);
        for id in PISTOL {
            let p = pistol_row(id).unwrap();
            f.seg(&p.slide); f.seg(&p.grip);
            f.seg_opt(&p.can); f.seg_opt(&p.mag);
            f.text(p.frame.as_str());
            match p.can_col { Some(c) => { f.word(1); f.text(c.as_str()); } None => f.word(0) }
        }
        assert_eq!(f.0, PISTOL_BITS);
    }

    /// A long id has no pistol row and a pistol id no long row, as the two Partial tables have it.
    #[test]
    fn the_two_tables_do_not_overlap() {
        for id in LONG { assert!(pistol_row(id).is_none()); }
        for id in PISTOL { assert!(long_row(id).is_none()); }
    }

    /// The defaults of `under` are the handguard's, the wrist's z is the caller's.
    #[test]
    fn under_takes_its_defaults() {
        let u = under(0.149, None, None);
        assert_eq!(u.off[0].to_bits(), 0.099f64.to_bits());
        assert_eq!(u.off[1].to_bits(), (-0.059f64).to_bits());
        assert_eq!(u.off[2].to_bits(), 0.149f64.to_bits());
        assert_eq!(u.fingers, long_row(Id::M1014).unwrap().grip_l.fingers);
        assert_eq!(u.back, long_row(Id::M1014).unwrap().grip_l.back);
    }

    use crate::weapons::{WeaponDef, WeaponId as W};

    fn row(g: &Game, id: W) -> WeaponDef { g.m.weapons.weapons.get(id).unwrap().clone() }

    /// Who carries which: rows of the page's WEAPONS, each line printed by gunlook.ts gunLookOf under bun with GUNLOOK.on.
    #[test]
    fn gun_look_of_answers_as_the_typescript() {
        let mut g = crate::game::Game::blank();
        let cases: [(W, Option<W>, Option<W>, Option<Id>, Option<Id>, Option<Id>, bool, bool); 10] = [
            (W::M870, None, Some(W::Mk12), Some(Id::M870), Some(Id::Mk12), None, false, false),
            (W::M870, None, None, Some(Id::M870), Some(Id::M870), None, false, false),
            (W::Mk18bare, Some(W::G19bare), None, None, None, None, true, true),
            (W::G19bare, Some(W::Mk18bare), None, None, None, None, true, true),
            (W::Mk12, Some(W::Hk45), None, Some(Id::Mk12), Some(Id::Mk12), Some(Id::Hk45), false, false),
            (W::Hk45, Some(W::Mk12), None, Some(Id::Hk45), Some(Id::Mk12), Some(Id::Hk45), false, false),
            (W::G18, Some(W::M1014), Some(W::M870), Some(Id::G18), Some(Id::M1014), Some(Id::G18), false, false),
            (W::Fn2000, Some(W::Fiveseven), None, None, None, None, false, false),
            (W::M870, Some(W::Hk45), Some(W::Mp7), Some(Id::M870), Some(Id::Mp7), Some(Id::Hk45), false, false),
            (W::Mk46, Some(W::M870), Some(W::M870), Some(Id::Mk46), Some(Id::Mk46), None, false, false),
        ];
        for (a, b, c, hand, long, side, bare_long, bare_side) in cases {
            let (a, b, c) = (row(&g, a), b.map(|x| row(&g, x)), c.map(|x| row(&g, x)));
            let l = gun_look_of(&mut g, None, Some(&a), b.as_ref(), c.as_ref()).unwrap();
            assert_eq!((l.hand, l.long, l.side, l.bare_long, l.bare_side), (hand, long, side, Some(bare_long), Some(bare_side)), "{:?}", a.id);
        }
        // nobody in his hands; and a look handed in is written over whole
        let stale = GunLook { hand: Some(Id::G18), long: Some(Id::G18), side: Some(Id::G18), bare_long: Some(true), bare_side: Some(true) };
        let l = gun_look_of(&mut g, Some(stale), None, None, None).unwrap();
        assert_eq!((l.hand, l.long, l.side, l.bare_long, l.bare_side), (None, None, None, Some(false), Some(false)));
    }

    /// Every row's voice and flame, printed by gunlook.ts voiceOf and flameOf under bun with GUNLOOK.on.
    #[test]
    fn voice_and_flame_of_every_row() {
        let mut g = crate::game::Game::blank();
        let rows: [(W, &str, &str); 16] = [
            (W::Fiveseven, "pistolSuppressed", "pistolSuppressed"), (W::Guard9mm, "pistolLoud", "pistolBare"),
            (W::Aks74u, "carbineReport", "carbine"), (W::Fn2000, "rifleSuppressed", "sc20k"),
            (W::M870, "m870Report", "carbine"), (W::Mk12, "mk12Suppressed", "mk12"),
            (W::Mp7, "mp7Suppressed", "mp7"), (W::Mk46, "mk46Report", "mk46"),
            (W::M1014, "m1014Report", "m1014"), (W::Hk45, "hk45Suppressed", "hk45"),
            (W::G18, "g18Report", "g18"), (W::Ak74m, "carbineReport", "carbine"),
            (W::Akms, "carbineReport", "pistolBare"), (W::Pkm, "carbineReport", "carbine"),
            (W::Mk18bare, "carbineReport", "carbine"), (W::G19bare, "pistolLoud", "pistolBare"),
        ];
        for (id, voice, flame) in rows {
            let d = row(&g, id);
            assert_eq!(voice_of(&mut g, &d), voice, "{:?}", id);
            assert_eq!(flame_of(&mut g, &d).as_str(), flame, "{:?}", id);
        }
    }

    // Printed by folding gunlook.ts's own tables under bun.
    const LONG_BITS: u64 = 0xf017_d0b1_2669_69ea;
    const PISTOL_BITS: u64 = 0x4910_88bb_c144_89b8;
}
