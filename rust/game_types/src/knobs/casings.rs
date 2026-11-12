//! THE EMPTY CASE EACH GUN THROWS: one particle a shot, and one small sound where it comes down. The picture and the ear alone (`casing_fx`): no guard hears it.
//! How long a case lasts and how loud it lands are in the `pfx` table.
use crate::bytes::ToBytes;
use crate::js::JsMap;
use crate::V3;
use game_derive::Knobs;

#[derive(Clone, Debug, ToBytes, Knobs)]
pub struct CasingsK {
    /// one row a gun; a gun with no row throws no case
    #[k({ let mut t = JsMap::new(); t.set(crate::weapons::WeaponId::Fiveseven, CasingsKRow { color: [0.72, 0.5, 0.16], diameter: 0.008, from: crate::particle_fx::CasingSpecFrom::Hand, ahead: 0.07, up: 0.055, right: 0.02, dir: [1.0, 0.9, -0.25], speed: 2.6, tink: 0.5 }); t.set(crate::weapons::WeaponId::Guard9mm, CasingsKRow { color: [0.72, 0.5, 0.16], diameter: 0.0095, from: crate::particle_fx::CasingSpecFrom::Hand, ahead: 0.07, up: 0.055, right: 0.02, dir: [1.0, 0.8, -0.2], speed: 3.0, tink: 0.5 }); t.set(crate::weapons::WeaponId::Aks74u, CasingsKRow { color: [0.2, 0.19, 0.12], diameter: 0.01, from: crate::particle_fx::CasingSpecFrom::Hand, ahead: 0.16, up: 0.07, right: 0.03, dir: [1.0, 0.45, 0.5], speed: 4.8, tink: 0.55 }); t.set(crate::weapons::WeaponId::Ak74m, CasingsKRow { color: [0.2, 0.19, 0.12], diameter: 0.01, from: crate::particle_fx::CasingSpecFrom::Hand, ahead: 0.16, up: 0.07, right: 0.03, dir: [1.0, 0.45, 0.5], speed: 4.8, tink: 0.55 }); t.set(crate::weapons::WeaponId::Akms, CasingsKRow { color: [0.2, 0.19, 0.12], diameter: 0.0113, from: crate::particle_fx::CasingSpecFrom::Hand, ahead: 0.16, up: 0.07, right: 0.03, dir: [1.0, 0.45, 0.5], speed: 4.5, tink: 0.55 }); t.set(crate::weapons::WeaponId::Fn2000, CasingsKRow { color: [0.72, 0.5, 0.16], diameter: 0.0095, from: crate::particle_fx::CasingSpecFrom::Muzzle, ahead: -0.16, up: -0.01, right: 0.045, dir: [0.35, -0.15, 1.0], speed: 1.1, tink: 0.4 }); t.set(crate::weapons::WeaponId::Mk12, CasingsKRow { color: [0.72, 0.5, 0.16], diameter: 0.0095, from: crate::particle_fx::CasingSpecFrom::Hand, ahead: 0.16, up: 0.07, right: 0.03, dir: [1.0, 0.5, 0.15], speed: 3.4, tink: 0.5 }); t.set(crate::weapons::WeaponId::Mp7, CasingsKRow { color: [0.72, 0.5, 0.16], diameter: 0.0075, from: crate::particle_fx::CasingSpecFrom::Hand, ahead: 0.1, up: 0.06, right: 0.02, dir: [1.0, 0.7, -0.1], speed: 3.0, tink: 0.5 }); t.set(crate::weapons::WeaponId::Mk46, CasingsKRow { color: [0.72, 0.5, 0.16], diameter: 0.0095, from: crate::particle_fx::CasingSpecFrom::Hand, ahead: 0.16, up: 0.02, right: 0.03, dir: [1.0, -0.1, 0.1], speed: 3.0, tink: 0.45 }); t.set(crate::weapons::WeaponId::M1014, CasingsKRow { color: [0.55, 0.06, 0.04], diameter: 0.02, from: crate::particle_fx::CasingSpecFrom::Hand, ahead: 0.14, up: 0.07, right: 0.03, dir: [1.0, 0.35, 0.1], speed: 2.6, tink: 0.5 }); t.set(crate::weapons::WeaponId::Hk45, CasingsKRow { color: [0.72, 0.5, 0.16], diameter: 0.012, from: crate::particle_fx::CasingSpecFrom::Hand, ahead: 0.07, up: 0.055, right: 0.02, dir: [1.0, 0.8, -0.2], speed: 2.6, tink: 0.5 }); t.set(crate::weapons::WeaponId::G18, CasingsKRow { color: [0.72, 0.5, 0.16], diameter: 0.0095, from: crate::particle_fx::CasingSpecFrom::Hand, ahead: 0.07, up: 0.055, right: 0.02, dir: [1.0, 0.8, -0.2], speed: 3.0, tink: 0.5 }); t.set(crate::weapons::WeaponId::Pkm, CasingsKRow { color: [0.2, 0.19, 0.12], diameter: 0.0124, from: crate::particle_fx::CasingSpecFrom::Hand, ahead: 0.16, up: 0.05, right: -0.03, dir: [-1.0, 0.2, 0.1], speed: 3.6, tink: 0.5 }); t.set(crate::weapons::WeaponId::Mk18bare, CasingsKRow { color: [0.72, 0.5, 0.16], diameter: 0.0095, from: crate::particle_fx::CasingSpecFrom::Muzzle, ahead: -0.16, up: -0.01, right: 0.045, dir: [0.35, -0.15, 1.0], speed: 1.1, tink: 0.4 }); t.set(crate::weapons::WeaponId::G19bare, CasingsKRow { color: [0.72, 0.5, 0.16], diameter: 0.008, from: crate::particle_fx::CasingSpecFrom::Hand, ahead: 0.07, up: 0.055, right: 0.02, dir: [1.0, 0.9, -0.25], speed: 2.6, tink: 0.5 }); t })] pub of: JsMap<crate::weapons::WeaponId, CasingsKRow>,
}

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct CasingsKRow {
    /// the case's color
    #[k([0.0; 3])] pub color: V3,
    /// its width as drawn (m)
    #[k(0.0)] pub diameter: f64,
    /// where the three offsets below start: `Hand` = the shooter's right hand (the muzzle if he has no such bone) · `Muzzle` = the muzzle
    #[k(crate::particle_fx::CasingSpecFrom::Hand)] pub from: crate::particle_fx::CasingSpecFrom,
    /// meters from there to the ejection port: along the barrel (toward the target; less than 0 = back from there) · up · to the gun's right
    #[k(0.0)] pub ahead: f64,
    #[k(0.0)] pub up: f64,
    #[k(0.0)] pub right: f64,
    /// which way it flies, in the gun's own frame: right · up · along the barrel. Only the direction counts, not the length (the case also gets ± 0.25 radians of scatter)
    #[k([0.0; 3])] pub dir: V3,
    /// m/s it leaves the port at, ± 30%, on top of the shooter's own velocity
    #[k(0.0)] pub speed: f64,
    /// seconds until the sound of it landing, which plays on the floor under the port + `dir` × `speed` × this (a guess: the flying particle isn't asked). 0 = at once, under the port
    #[k(0.0)] pub tink: f64,
}
