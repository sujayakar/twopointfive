//! The types of `weapons`.
#![allow(unused_imports)]
use crate::{Blank, Words, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// WHICH OF THE THREE A GUN IS: a Pistol's and a Rifle's own rows are fields of the one Firearm (g.fa)
#[derive(Clone, Copy, PartialEq, Eq, Debug, ToBytes)]
pub enum GunKind { Plain, Pistol, Rifle }
/// WHAT A RELOAD TELLS ITS OWNER AS IT GOES (weapons.ts ReloadEvents, callbacks in the TypeScript: player.ts:362 and :381 are the only callers that give any). The body matches on it at each stage
#[derive(Clone, Debug, ToBytes)]
pub enum ReloadEvents {
    None,
    /// the sidearm's: the three sounds and the magazine that falls. `fill` what it drops (feedPlan's drop, or 0)
    Pistol { ch: CharId, fill: f64 },
    /// the rifle's: as the sidearm's at its own rates, or `own`'s sounds at the cues
    Rifle { ch: CharId, feed: Option<FeedPlan> },
}
/// what gunreload.ts feedPlan returns: own sounds at the cues, their times, what an empty one leaving the gun is drawn as
#[derive(Clone, Debug, ToBytes)]
pub struct FeedPlan { pub cues: Option<Vec<FeedCue>>,  pub drop: Option<f64> }
#[derive(Clone, Debug, ToBytes)]
pub struct FeedCue { pub t: f64, pub sfx: String, pub gain: f64, pub rate: f64 }

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default, Words)]
pub enum GunModelId {
    #[default] M870,
    Mk12,
    Mp7,
    Mk46,
    M1014,
    Hk45,
    G18,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, ToBytes, Default, Words)]
pub enum WeaponId {
    #[default] Fiveseven,
    Guard9mm,
    Aks74u,
    Ak74m,
    Akms,
    Fn2000,
    M870,
    Mk12,
    Mp7,
    Mk46,
    M1014,
    Hk45,
    G18,
    Pkm,
    Mk18bare,
    G19bare,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default, Words)]
pub enum FireMode {
    #[default] Semi,
    Auto,
    Burst,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct WeaponDef {
    pub id: crate::weapons::WeaponId,
    pub enabled: bool,
    pub short: String,
    pub calibre: String,
    pub cls: crate::weapons::WeaponClass,
    pub mag_capacity: f64,
    pub spare_mags: f64,
    pub top_up: bool,
    pub reload: crate::weapons::WeaponDefReload,
    pub modes: Vec<crate::weapons::FireMode>,
    pub cd: f64,
    pub cd_jitter: f64,
    pub burst: Option<crate::weapons::BurstSpec>,
    pub auto: Option<crate::weapons::AutoSpec>,
    pub report: crate::weapons::WeaponDefReport,
    pub suppressed: bool,
    pub muzzle_fx: crate::effects::MuzzleFxKey,
    pub prop_impulse: f64,
    pub ai_spread: crate::weapons::AiSpread,
    pub muzzle_ahead: f64,
    pub rig: crate::weapons::WeaponDefRig,
    pub handling: Option<f64>,
    pub drop: crate::slotsetup::SetupGun,
    pub tracer: Option<crate::weapons::TracerSpec>,
    pub pellets: Option<crate::weapons::PelletSpec>,
    pub hands: Option<crate::weapons::WeaponDefHands>,
    pub launcher: Option<bool>,
    pub weight_kg: f64,
    pub bulk: f64,
    pub own: Option<crate::weapons::WeaponDefOwn>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default, Words)]
pub enum WeaponClass {
    #[default] Pistol,
    Carbine,
    Rifle,
    Shotgun,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct WeaponDefReload {
    pub dur: f64,
    pub out: Option<f64>,
    pub in_: Option<f64>,
    pub rack: Option<f64>,
    pub shell: Option<crate::gunreload::ShellRowShell>,
    /// INVENTED by a review: see the proposals
    pub exact: Option<bool>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct BurstSpec {
    pub min: f64,
    pub max: f64,
    pub interval: f64,
    pub gap_min: f64,
    pub gap_max: f64,
    pub spread_per_round: f64,
    pub spread_cap: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct AutoSpec {
    pub interval: f64,
    pub spread_per_round: f64,
    pub spread_cap: f64,
    pub reset_after: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct WeaponDefReport {
    pub level: f64,
    pub loud: bool,
    pub voice: String,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct AiSpread {
    pub base: f64,
    pub move_k: f64,
    pub move_cap: f64,
    pub range_from: f64,
    pub range_k: f64,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum WeaponDefRig {
    #[default] Pistol,
    Carbine,
    GuardPistol,
    Sc20k,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct TracerSpec {
    pub every: f64,
    pub last_n: f64,
    pub color: V3,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct PelletSpec {
    pub n: f64,
    pub cone: f64,
    pub drop_m: f64,
    pub need: f64,
    pub breach: Option<crate::weapons::PelletSpecBreach>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct PelletSpecBreach {
    pub reach_m: f64,
    pub latch_m: f64,
    pub need: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct WeaponDefHands {
    pub still: f64,
    pub moving: f64,
    pub crouch: Option<f64>,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct WeaponDefOwn {
    pub model: crate::weapons::GunModelId,
    pub voice: Option<String>,
    pub flame: Option<crate::effects::MuzzleFxKey>,
}

#[derive(Clone, Copy, Debug, ToBytes, Blank)]
pub struct Ocp {
    #[is(1.0)] pub charge: f64,
    #[is(0.28)] pub recharge_per_sec: f64,
    #[is(1.0)] pub cost: f64,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Words)]
pub enum RifleAttachment {
    Ring,
    He40,
    Cam,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct BurstState {
    pub left: f64,
    pub next_t: f64,
    pub idx: f64,
    pub t: f64,
    pub off: Option<V3>,
}

#[derive(Clone, Debug, ToBytes, Blank)]
pub struct Firearm {
    #[is(1.0)] pub chamber: f64,
    pub infinite: bool,
    pub mag: Option<f64>,
    pub spare: Vec<f64>,
    #[is(-1.0)] pub reload_t: f64,
    pub stage: f64,
    #[is(crate::weapons::ReloadEvents::None)] pub ev: crate::weapons::ReloadEvents,
    pub beat: Option<crate::weapons::WeaponDefReload>,
    pub cues: Option<Vec<f64>>,
    pub cue_i: usize,
    pub feed_n: f64,
    pub feed_port: bool,
    pub shell_t: Option<Vec<f64>>,
    pub shell_i: usize,
    pub shell_port: bool,
    pub feed_pull: bool,
    pub shots_fired: f64,
    pub burst: Option<crate::weapons::BurstState>,
    pub bursts: f64,
    #[is(-100.0)] pub last_burst_t: f64,
    #[is(crate::weapons::FireMode::Semi)] pub mode: crate::weapons::FireMode,
    pub auto_idx: f64,
    pub auto_since: f64,
    #[is(crate::weapons::WeaponId::Fiveseven)] pub def: crate::weapons::WeaponId,
    /// which of the three a gun is (TypeScript: the classes Firearm, Pistol, Rifle)
    #[is(crate::weapons::GunKind::Plain)] pub kind: crate::weapons::GunKind,
    /// Pistol and Rifle
    pub light_on: bool,
    /// Rifle
    pub attachments: Vec<crate::weapons::RifleAttachment>,
    /// Rifle
    pub att_idx: i32,
    /// Rifle: from g.k at new
    pub rings: f64,
    /// Rifle
    pub ring_cd: f64,
    /// Rifle: from g.k at new
    pub shells: f64,
    /// Rifle
    pub cams: f64,
}

#[derive(Clone, Debug, Default)]
pub struct FirearmStartReloadShells {
    pub times: Vec<f64>,
    pub port: bool,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct WeaponsMod {
    #[is({ let mut t = JsMap::new(); t.set(crate::weapons::WeaponId::Fiveseven, crate::weapons::WeaponDef { id: crate::weapons::WeaponId::Fiveseven, enabled: true, short: String::from("Glock 19"), calibre: String::from("9×19 mm"), cls: crate::weapons::WeaponClass::Pistol, mag_capacity: 15.0, spare_mags: 3.0, top_up: true, reload: crate::weapons::WeaponDefReload { dur: 1.6, out: Some(0.38), in_: Some(1.02), rack: Some(1.38), shell: None, exact: None }, modes: alloc::vec![crate::weapons::FireMode::Semi], cd: 0.22, cd_jitter: 0.0, burst: None, auto: None, report: crate::weapons::WeaponDefReport { level: 0.6, loud: false, voice: String::from("pistolSuppressed") }, suppressed: true, muzzle_fx: crate::effects::MuzzleFxKey::PistolSuppressed, prop_impulse: 4.0, ai_spread: crate::weapons::AiSpread { base: 0.05, move_k: 0.014, move_cap: 0.06, range_from: 8.0, range_k: 0.006 }, muzzle_ahead: 0.26, rig: crate::weapons::WeaponDefRig::Pistol, handling: None, drop: crate::slotsetup::SetupGun::Pistol, tracer: None, pellets: None, hands: None, launcher: None, weight_kg: 0.9, bulk: 0.0, own: None }); t.set(crate::weapons::WeaponId::Guard9mm, crate::weapons::WeaponDef { id: crate::weapons::WeaponId::Guard9mm, enabled: true, short: String::from("sidearm"), calibre: String::from("9×19 mm"), cls: crate::weapons::WeaponClass::Pistol, mag_capacity: 12.0, spare_mags: f64::INFINITY, top_up: false, reload: crate::weapons::WeaponDefReload { dur: 1.7, out: None, in_: None, rack: None, shell: None, exact: None }, modes: alloc::vec![crate::weapons::FireMode::Semi], cd: 0.55, cd_jitter: 0.5, burst: None, auto: None, report: crate::weapons::WeaponDefReport { level: 1.0, loud: true, voice: String::from("pistolLoud") }, suppressed: false, muzzle_fx: crate::effects::MuzzleFxKey::PistolBare, prop_impulse: 6.0, ai_spread: crate::weapons::AiSpread { base: 0.05, move_k: 0.014, move_cap: 0.06, range_from: 8.0, range_k: 0.006 }, muzzle_ahead: 0.26, rig: crate::weapons::WeaponDefRig::GuardPistol, handling: None, drop: crate::slotsetup::SetupGun::Pistol, tracer: None, pellets: None, hands: None, launcher: None, weight_kg: 0.95, bulk: 0.0, own: None }); t.set(crate::weapons::WeaponId::Aks74u, crate::weapons::WeaponDef { id: crate::weapons::WeaponId::Aks74u, enabled: true, short: String::from("AKS-74U"), calibre: String::from("5.45×39 mm"), cls: crate::weapons::WeaponClass::Carbine, mag_capacity: 30.0, spare_mags: f64::INFINITY, top_up: false, reload: crate::weapons::WeaponDefReload { dur: 2.6, out: Some(0.7), in_: Some(1.7), rack: Some(2.2), shell: None, exact: None }, modes: alloc::vec![crate::weapons::FireMode::Auto, crate::weapons::FireMode::Semi], cd: 0.1, cd_jitter: 0.0, burst: Some(crate::weapons::BurstSpec { min: 2.0, max: 4.0, interval: 0.1, gap_min: 0.9, gap_max: 1.6, spread_per_round: 0.03, spread_cap: 0.16 }), auto: None, report: crate::weapons::WeaponDefReport { level: 1.0, loud: true, voice: String::from("carbineReport") }, suppressed: false, muzzle_fx: crate::effects::MuzzleFxKey::Carbine, prop_impulse: 6.0, ai_spread: crate::weapons::AiSpread { base: 0.035, move_k: 0.014, move_cap: 0.06, range_from: 8.0, range_k: 0.006 }, muzzle_ahead: 0.5, rig: crate::weapons::WeaponDefRig::Carbine, handling: None, drop: crate::slotsetup::SetupGun::Rifle, tracer: Some(crate::weapons::TracerSpec { every: 3.0, last_n: 3.0, color: [1.0, 0.42, 0.12] }), pellets: None, hands: None, launcher: None, weight_kg: 3.0, bulk: 2.0, own: None }); t.set(crate::weapons::WeaponId::Fn2000, crate::weapons::WeaponDef { id: crate::weapons::WeaponId::Fn2000, enabled: true, short: String::from("Mk 18"), calibre: String::from("5.56×45 mm"), cls: crate::weapons::WeaponClass::Rifle, mag_capacity: 30.0, spare_mags: 2.0, top_up: true, reload: crate::weapons::WeaponDefReload { dur: 2.4, out: Some(0.5), in_: Some(1.5), rack: Some(2.0), shell: None, exact: None }, modes: alloc::vec![crate::weapons::FireMode::Semi, crate::weapons::FireMode::Auto], cd: 0.2, cd_jitter: 0.0, burst: None, auto: Some(crate::weapons::AutoSpec { interval: 0.1, spread_per_round: 0.012, spread_cap: 0.08, reset_after: 0.25 }), report: crate::weapons::WeaponDefReport { level: 1.0, loud: true, voice: String::from("rifleSuppressed") }, suppressed: true, muzzle_fx: crate::effects::MuzzleFxKey::Sc20k, prop_impulse: 5.0, ai_spread: crate::weapons::AiSpread { base: 0.035, move_k: 0.014, move_cap: 0.06, range_from: 8.0, range_k: 0.006 }, muzzle_ahead: 0.52, rig: crate::weapons::WeaponDefRig::Sc20k, handling: Some(0.8), drop: crate::slotsetup::SetupGun::Rifle, tracer: None, pellets: None, hands: None, launcher: None, weight_kg: 3.9, bulk: 2.0, own: None }); t.set(crate::weapons::WeaponId::M870, crate::weapons::WeaponDef { id: crate::weapons::WeaponId::M870, enabled: true, short: String::from("M870"), calibre: String::from("12 gauge"), cls: crate::weapons::WeaponClass::Shotgun, mag_capacity: 6.0, spare_mags: 3.0, top_up: true, reload: crate::weapons::WeaponDefReload { dur: 3.2, out: Some(0.4), in_: Some(2.5), rack: Some(2.9), shell: Some(crate::gunreload::ShellRowShell { lead: 0.4, each: 0.35, tail: 0.7, bolt: 0.4 }), exact: None }, modes: alloc::vec![crate::weapons::FireMode::Semi], cd: 0.9, cd_jitter: 0.2, burst: None, auto: None, report: crate::weapons::WeaponDefReport { level: 1.0, loud: true, voice: String::from("carbineReport") }, suppressed: false, muzzle_fx: crate::effects::MuzzleFxKey::Carbine, prop_impulse: 2.0, ai_spread: crate::weapons::AiSpread { base: 0.035, move_k: 0.014, move_cap: 0.06, range_from: 8.0, range_k: 0.006 }, muzzle_ahead: 0.5, rig: crate::weapons::WeaponDefRig::Carbine, handling: None, drop: crate::slotsetup::SetupGun::Rifle, tracer: None, pellets: Some(crate::weapons::PelletSpec { n: 9.0, cone: 0.06, drop_m: 8.0, need: 4.0, breach: Some(crate::weapons::PelletSpecBreach { reach_m: 2.0, latch_m: 0.5, need: 4.0 }) }), hands: None, launcher: None, weight_kg: 3.4, bulk: 3.0, own: Some(crate::weapons::WeaponDefOwn { model: crate::weapons::GunModelId::M870, voice: Some(String::from("m870Report")), flame: None }) }); t.set(crate::weapons::WeaponId::Mk12, crate::weapons::WeaponDef { id: crate::weapons::WeaponId::Mk12, enabled: true, short: String::from("Mk 12"), calibre: String::from("5.56×45 mm"), cls: crate::weapons::WeaponClass::Rifle, mag_capacity: 20.0, spare_mags: 3.0, top_up: true, reload: crate::weapons::WeaponDefReload { dur: 2.6, out: Some(0.55), in_: Some(1.65), rack: Some(2.2), shell: None, exact: None }, modes: alloc::vec![crate::weapons::FireMode::Semi], cd: 0.3, cd_jitter: 0.15, burst: None, auto: None, report: crate::weapons::WeaponDefReport { level: 1.0, loud: true, voice: String::from("rifleSuppressed") }, suppressed: true, muzzle_fx: crate::effects::MuzzleFxKey::Sc20k, prop_impulse: 5.0, ai_spread: crate::weapons::AiSpread { base: 0.018, move_k: 0.02, move_cap: 0.08, range_from: 16.0, range_k: 0.003 }, muzzle_ahead: 0.52, rig: crate::weapons::WeaponDefRig::Sc20k, handling: Some(0.62), drop: crate::slotsetup::SetupGun::Rifle, tracer: None, pellets: None, hands: Some(crate::weapons::WeaponDefHands { still: 0.45, moving: 1.4, crouch: None }), launcher: Some(false), weight_kg: 5.0, bulk: 3.0, own: Some(crate::weapons::WeaponDefOwn { model: crate::weapons::GunModelId::Mk12, voice: Some(String::from("mk12Suppressed")), flame: Some(crate::effects::MuzzleFxKey::Mk12) }) }); t.set(crate::weapons::WeaponId::Mp7, crate::weapons::WeaponDef { id: crate::weapons::WeaponId::Mp7, enabled: true, short: String::from("MP7"), calibre: String::from("4.6×30 mm"), cls: crate::weapons::WeaponClass::Carbine, mag_capacity: 40.0, spare_mags: 3.0, top_up: true, reload: crate::weapons::WeaponDefReload { dur: 2.0, out: Some(0.4), in_: Some(1.25), rack: Some(1.65), shell: None, exact: None }, modes: alloc::vec![crate::weapons::FireMode::Auto, crate::weapons::FireMode::Semi], cd: 0.12, cd_jitter: 0.0, burst: Some(crate::weapons::BurstSpec { min: 2.0, max: 4.0, interval: 0.06666666666666667, gap_min: 0.5, gap_max: 0.9, spread_per_round: 0.02, spread_cap: 0.12 }), auto: Some(crate::weapons::AutoSpec { interval: 0.06666666666666667, spread_per_round: 0.008, spread_cap: 0.07, reset_after: 0.25 }), report: crate::weapons::WeaponDefReport { level: 0.8, loud: false, voice: String::from("pistolSuppressed") }, suppressed: true, muzzle_fx: crate::effects::MuzzleFxKey::PistolSuppressed, prop_impulse: 3.0, ai_spread: crate::weapons::AiSpread { base: 0.045, move_k: 0.008, move_cap: 0.03, range_from: 7.0, range_k: 0.012 }, muzzle_ahead: 0.4, rig: crate::weapons::WeaponDefRig::Sc20k, handling: Some(0.95), drop: crate::slotsetup::SetupGun::Rifle, tracer: None, pellets: None, hands: Some(crate::weapons::WeaponDefHands { still: 1.6, moving: 0.7, crouch: None }), launcher: Some(false), weight_kg: 2.3, bulk: 1.0, own: Some(crate::weapons::WeaponDefOwn { model: crate::weapons::GunModelId::Mp7, voice: Some(String::from("mp7Suppressed")), flame: Some(crate::effects::MuzzleFxKey::Mp7) }) }); t.set(crate::weapons::WeaponId::Mk46, crate::weapons::WeaponDef { id: crate::weapons::WeaponId::Mk46, enabled: true, short: String::from("Mk 46"), calibre: String::from("5.56×45 mm"), cls: crate::weapons::WeaponClass::Rifle, mag_capacity: 100.0, spare_mags: 1.0, top_up: true, reload: crate::weapons::WeaponDefReload { dur: 6.0, out: Some(1.2), in_: Some(4.4), rack: Some(5.4), shell: None, exact: None }, modes: alloc::vec![crate::weapons::FireMode::Auto], cd: 0.08333333333333333, cd_jitter: 0.0, burst: Some(crate::weapons::BurstSpec { min: 5.0, max: 9.0, interval: 0.08333333333333333, gap_min: 0.5, gap_max: 1.0, spread_per_round: 0.012, spread_cap: 0.14 }), auto: Some(crate::weapons::AutoSpec { interval: 0.08333333333333333, spread_per_round: 0.004, spread_cap: 0.06, reset_after: 0.3 }), report: crate::weapons::WeaponDefReport { level: 1.0, loud: true, voice: String::from("carbineReport") }, suppressed: false, muzzle_fx: crate::effects::MuzzleFxKey::Carbine, prop_impulse: 6.0, ai_spread: crate::weapons::AiSpread { base: 0.05, move_k: 0.03, move_cap: 0.12, range_from: 10.0, range_k: 0.004 }, muzzle_ahead: 0.52, rig: crate::weapons::WeaponDefRig::Sc20k, handling: Some(0.5), drop: crate::slotsetup::SetupGun::Rifle, tracer: Some(crate::weapons::TracerSpec { every: 5.0, last_n: 0.0, color: [1.0, 0.25, 0.12] }), pellets: None, hands: Some(crate::weapons::WeaponDefHands { still: 1.5, moving: 2.4, crouch: Some(0.5) }), launcher: Some(false), weight_kg: 8.0, bulk: 4.0, own: Some(crate::weapons::WeaponDefOwn { model: crate::weapons::GunModelId::Mk46, voice: Some(String::from("mk46Report")), flame: Some(crate::effects::MuzzleFxKey::Mk46) }) }); t.set(crate::weapons::WeaponId::M1014, crate::weapons::WeaponDef { id: crate::weapons::WeaponId::M1014, enabled: true, short: String::from("M1014"), calibre: String::from("12 gauge"), cls: crate::weapons::WeaponClass::Shotgun, mag_capacity: 7.0, spare_mags: 3.0, top_up: true, reload: crate::weapons::WeaponDefReload { dur: 3.6, out: Some(0.4), in_: Some(2.9), rack: Some(3.3), shell: Some(crate::gunreload::ShellRowShell { lead: 0.4, each: 0.36, tail: 0.7, bolt: 0.4 }), exact: None }, modes: alloc::vec![crate::weapons::FireMode::Semi], cd: 0.28, cd_jitter: 0.1, burst: None, auto: None, report: crate::weapons::WeaponDefReport { level: 1.0, loud: true, voice: String::from("carbineReport") }, suppressed: false, muzzle_fx: crate::effects::MuzzleFxKey::Carbine, prop_impulse: 2.0, ai_spread: crate::weapons::AiSpread { base: 0.035, move_k: 0.014, move_cap: 0.06, range_from: 8.0, range_k: 0.006 }, muzzle_ahead: 0.5, rig: crate::weapons::WeaponDefRig::Carbine, handling: Some(0.72), drop: crate::slotsetup::SetupGun::Rifle, tracer: None, pellets: Some(crate::weapons::PelletSpec { n: 9.0, cone: 0.045, drop_m: 10.0, need: 4.0, breach: None }), hands: None, launcher: Some(false), weight_kg: 3.9, bulk: 3.0, own: Some(crate::weapons::WeaponDefOwn { model: crate::weapons::GunModelId::M1014, voice: Some(String::from("m1014Report")), flame: Some(crate::effects::MuzzleFxKey::M1014) }) }); t.set(crate::weapons::WeaponId::Hk45, crate::weapons::WeaponDef { id: crate::weapons::WeaponId::Hk45, enabled: true, short: String::from("HK45"), calibre: String::from(".45 ACP"), cls: crate::weapons::WeaponClass::Pistol, mag_capacity: 8.0, spare_mags: 3.0, top_up: true, reload: crate::weapons::WeaponDefReload { dur: 1.7, out: Some(0.4), in_: Some(1.1), rack: Some(1.45), shell: None, exact: None }, modes: alloc::vec![crate::weapons::FireMode::Semi], cd: 0.3, cd_jitter: 0.0, burst: None, auto: None, report: crate::weapons::WeaponDefReport { level: 0.45, loud: false, voice: String::from("pistolSuppressed") }, suppressed: true, muzzle_fx: crate::effects::MuzzleFxKey::PistolSuppressed, prop_impulse: 5.0, ai_spread: crate::weapons::AiSpread { base: 0.05, move_k: 0.014, move_cap: 0.06, range_from: 8.0, range_k: 0.006 }, muzzle_ahead: 0.26, rig: crate::weapons::WeaponDefRig::Pistol, handling: None, drop: crate::slotsetup::SetupGun::Pistol, tracer: None, pellets: None, hands: None, launcher: None, weight_kg: 1.2, bulk: 0.0, own: Some(crate::weapons::WeaponDefOwn { model: crate::weapons::GunModelId::Hk45, voice: Some(String::from("hk45Suppressed")), flame: Some(crate::effects::MuzzleFxKey::Hk45) }) }); t.set(crate::weapons::WeaponId::G18, crate::weapons::WeaponDef { id: crate::weapons::WeaponId::G18, enabled: true, short: String::from("Glock 18"), calibre: String::from("9×19 mm"), cls: crate::weapons::WeaponClass::Pistol, mag_capacity: 33.0, spare_mags: 2.0, top_up: true, reload: crate::weapons::WeaponDefReload { dur: 1.8, out: Some(0.4), in_: Some(1.15), rack: Some(1.5), shell: None, exact: None }, modes: alloc::vec![crate::weapons::FireMode::Auto, crate::weapons::FireMode::Semi], cd: 0.2, cd_jitter: 0.0, burst: Some(crate::weapons::BurstSpec { min: 3.0, max: 5.0, interval: 0.05, gap_min: 0.5, gap_max: 0.9, spread_per_round: 0.03, spread_cap: 0.18 }), auto: Some(crate::weapons::AutoSpec { interval: 0.05, spread_per_round: 0.02, spread_cap: 0.16, reset_after: 0.25 }), report: crate::weapons::WeaponDefReport { level: 1.0, loud: true, voice: String::from("pistolLoud") }, suppressed: false, muzzle_fx: crate::effects::MuzzleFxKey::PistolBare, prop_impulse: 6.0, ai_spread: crate::weapons::AiSpread { base: 0.06, move_k: 0.014, move_cap: 0.06, range_from: 8.0, range_k: 0.006 }, muzzle_ahead: 0.26, rig: crate::weapons::WeaponDefRig::GuardPistol, handling: None, drop: crate::slotsetup::SetupGun::Pistol, tracer: None, pellets: None, hands: None, launcher: None, weight_kg: 1.0, bulk: 0.0, own: Some(crate::weapons::WeaponDefOwn { model: crate::weapons::GunModelId::G18, voice: Some(String::from("g18Report")), flame: Some(crate::effects::MuzzleFxKey::G18) }) }); t.set(crate::weapons::WeaponId::Ak74m, crate::weapons::WeaponDef { id: crate::weapons::WeaponId::Ak74m, enabled: false, short: String::from("AK-74M"), calibre: String::from("5.45×39 mm"), cls: crate::weapons::WeaponClass::Rifle, mag_capacity: 30.0, spare_mags: f64::INFINITY, top_up: false, reload: crate::weapons::WeaponDefReload { dur: 2.6, out: Some(0.7), in_: Some(1.7), rack: Some(2.2), shell: None, exact: None }, modes: alloc::vec![crate::weapons::FireMode::Auto, crate::weapons::FireMode::Semi], cd: 0.1, cd_jitter: 0.0, burst: Some(crate::weapons::BurstSpec { min: 2.0, max: 4.0, interval: 0.1, gap_min: 0.9, gap_max: 1.6, spread_per_round: 0.025, spread_cap: 0.16 }), auto: None, report: crate::weapons::WeaponDefReport { level: 1.0, loud: true, voice: String::from("carbineReport") }, suppressed: false, muzzle_fx: crate::effects::MuzzleFxKey::Carbine, prop_impulse: 6.0, ai_spread: crate::weapons::AiSpread { base: 0.03, move_k: 0.014, move_cap: 0.06, range_from: 8.0, range_k: 0.006 }, muzzle_ahead: 0.66, rig: crate::weapons::WeaponDefRig::Carbine, handling: None, drop: crate::slotsetup::SetupGun::Rifle, tracer: Some(crate::weapons::TracerSpec { every: 3.0, last_n: 3.0, color: [1.0, 0.42, 0.12] }), pellets: None, hands: None, launcher: None, weight_kg: 3.6, bulk: 3.0, own: None }); t.set(crate::weapons::WeaponId::Akms, crate::weapons::WeaponDef { id: crate::weapons::WeaponId::Akms, enabled: true, short: String::from("AKMS"), calibre: String::from("7.62×39 mm"), cls: crate::weapons::WeaponClass::Rifle, mag_capacity: 30.0, spare_mags: f64::INFINITY, top_up: false, reload: crate::weapons::WeaponDefReload { dur: 2.6, out: Some(0.7), in_: Some(1.7), rack: Some(2.2), shell: None, exact: None }, modes: alloc::vec![crate::weapons::FireMode::Auto, crate::weapons::FireMode::Semi], cd: 0.1, cd_jitter: 0.0, burst: Some(crate::weapons::BurstSpec { min: 2.0, max: 3.0, interval: 0.1, gap_min: 0.9, gap_max: 1.6, spread_per_round: 0.04, spread_cap: 0.16 }), auto: None, report: crate::weapons::WeaponDefReport { level: 1.0, loud: true, voice: String::from("carbineReport") }, suppressed: false, muzzle_fx: crate::effects::MuzzleFxKey::PistolBare, prop_impulse: 6.0, ai_spread: crate::weapons::AiSpread { base: 0.04, move_k: 0.014, move_cap: 0.06, range_from: 8.0, range_k: 0.006 }, muzzle_ahead: 0.62, rig: crate::weapons::WeaponDefRig::Carbine, handling: None, drop: crate::slotsetup::SetupGun::Rifle, tracer: None, pellets: None, hands: None, launcher: None, weight_kg: 3.5, bulk: 3.0, own: None }); t.set(crate::weapons::WeaponId::Pkm, crate::weapons::WeaponDef { id: crate::weapons::WeaponId::Pkm, enabled: true, short: String::from("PKM"), calibre: String::from("7.62×54 mm R"), cls: crate::weapons::WeaponClass::Rifle, mag_capacity: 100.0, spare_mags: f64::INFINITY, top_up: false, reload: crate::weapons::WeaponDefReload { dur: 6.5, out: Some(1.4), in_: Some(5.0), rack: Some(5.9), shell: None, exact: None }, modes: alloc::vec![crate::weapons::FireMode::Auto], cd: 0.1, cd_jitter: 0.0, burst: Some(crate::weapons::BurstSpec { min: 6.0, max: 10.0, interval: 0.1, gap_min: 0.5, gap_max: 1.0, spread_per_round: 0.012, spread_cap: 0.14 }), auto: None, report: crate::weapons::WeaponDefReport { level: 1.0, loud: true, voice: String::from("carbineReport") }, suppressed: false, muzzle_fx: crate::effects::MuzzleFxKey::Carbine, prop_impulse: 6.0, ai_spread: crate::weapons::AiSpread { base: 0.05, move_k: 0.03, move_cap: 0.12, range_from: 10.0, range_k: 0.004 }, muzzle_ahead: 0.62, rig: crate::weapons::WeaponDefRig::Carbine, handling: Some(0.45), drop: crate::slotsetup::SetupGun::Rifle, tracer: Some(crate::weapons::TracerSpec { every: 5.0, last_n: 0.0, color: [1.0, 0.42, 0.12] }), pellets: None, hands: None, launcher: None, weight_kg: 7.5, bulk: 4.0, own: None }); t.set(crate::weapons::WeaponId::Mk18bare, crate::weapons::WeaponDef { id: crate::weapons::WeaponId::Mk18bare, enabled: true, short: String::from("Mk 18"), calibre: String::from("5.56×45 mm"), cls: crate::weapons::WeaponClass::Rifle, mag_capacity: 30.0, spare_mags: 2.0, top_up: true, reload: crate::weapons::WeaponDefReload { dur: 2.4, out: Some(0.5), in_: Some(1.5), rack: Some(2.0), shell: None, exact: None }, modes: alloc::vec![crate::weapons::FireMode::Semi, crate::weapons::FireMode::Auto], cd: 0.2, cd_jitter: 0.0, burst: None, auto: Some(crate::weapons::AutoSpec { interval: 0.1, spread_per_round: 0.012, spread_cap: 0.08, reset_after: 0.25 }), report: crate::weapons::WeaponDefReport { level: 1.0, loud: true, voice: String::from("carbineReport") }, suppressed: false, muzzle_fx: crate::effects::MuzzleFxKey::Carbine, prop_impulse: 6.0, ai_spread: crate::weapons::AiSpread { base: 0.035, move_k: 0.014, move_cap: 0.06, range_from: 8.0, range_k: 0.006 }, muzzle_ahead: 0.52, rig: crate::weapons::WeaponDefRig::Sc20k, handling: Some(0.8), drop: crate::slotsetup::SetupGun::Rifle, tracer: None, pellets: None, hands: None, launcher: None, weight_kg: 3.4, bulk: 2.0, own: None }); t.set(crate::weapons::WeaponId::G19bare, crate::weapons::WeaponDef { id: crate::weapons::WeaponId::G19bare, enabled: true, short: String::from("Glock 19"), calibre: String::from("9×19 mm"), cls: crate::weapons::WeaponClass::Pistol, mag_capacity: 15.0, spare_mags: 3.0, top_up: true, reload: crate::weapons::WeaponDefReload { dur: 1.6, out: Some(0.38), in_: Some(1.02), rack: Some(1.38), shell: None, exact: None }, modes: alloc::vec![crate::weapons::FireMode::Semi], cd: 0.22, cd_jitter: 0.0, burst: None, auto: None, report: crate::weapons::WeaponDefReport { level: 1.0, loud: true, voice: String::from("pistolLoud") }, suppressed: false, muzzle_fx: crate::effects::MuzzleFxKey::PistolBare, prop_impulse: 6.0, ai_spread: crate::weapons::AiSpread { base: 0.05, move_k: 0.014, move_cap: 0.06, range_from: 8.0, range_k: 0.006 }, muzzle_ahead: 0.26, rig: crate::weapons::WeaponDefRig::Pistol, handling: None, drop: crate::slotsetup::SetupGun::Pistol, tracer: None, pellets: None, hands: None, launcher: None, weight_kg: 0.8, bulk: 0.0, own: None }); t })] pub weapons: JsMap<crate::weapons::WeaponId, crate::weapons::WeaponDef>,
}

/// a class's turn weight
pub fn handling_by_class(class: crate::weapons::WeaponClass) -> f64 {
    use crate::weapons::WeaponClass as C;
    match class {
        C::Pistol => 1.0,
        C::Carbine => 0.82,
        C::Rifle => 0.7,
        C::Shotgun => 0.75,
    }
}
