//! The types of `weapons`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// WHICH OF THE THREE A GUN IS: a Pistol's and a Rifle's own rows are fields of the one Firearm (g.fa)
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum GunKind { Plain, Pistol, Rifle }
/// WHAT A RELOAD TELLS ITS OWNER AS IT GOES (weapons.ts ReloadEvents, callbacks in the TypeScript: player.ts:362 and :381 are the only callers that give any). The body matches on it at each stage
#[derive(Clone, Debug)]
pub enum ReloadEvents {
    None,
    /// the sidearm's: the three sounds and the magazine that falls. `fill` what it drops (feedPlan's drop, or 0)
    Pistol { ch: CharId, fill: f64 },
    /// the rifle's: as the sidearm's at its own rates, or `own`'s sounds at the cues
    Rifle { ch: CharId, feed: Option<FeedPlan> },
}
/// what gunreload.ts feedPlan returns: own sounds at the cues, their times, what an empty one leaving the gun is drawn as
#[derive(Clone, Debug)]
pub struct FeedPlan { pub cues: Option<Vec<FeedCue>>, pub times: Vec<f64>, pub drop: Option<f64> }
/// (gunreload.ts FeedCue)
#[derive(Clone, Debug)]
pub struct FeedCue { pub t: f64, pub sfx: String, pub gain: f64, pub rate: f64 }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum GunModelId {
    M870,
    Mk12,
    Mp7,
    Mk46,
    M1014,
    Hk45,
    G18,
}
impl GunModelId {
    pub const fn as_str(self) -> &'static str {
        match self {
            GunModelId::M870 => "m870",
            GunModelId::Mk12 => "mk12",
            GunModelId::Mp7 => "mp7",
            GunModelId::Mk46 => "mk46",
            GunModelId::M1014 => "m1014",
            GunModelId::Hk45 => "hk45",
            GunModelId::G18 => "g18",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum WeaponId {
    Fiveseven,
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
impl WeaponId {
    pub const fn as_str(self) -> &'static str {
        match self {
            WeaponId::Fiveseven => "fiveseven",
            WeaponId::Guard9mm => "guard9mm",
            WeaponId::Aks74u => "aks74u",
            WeaponId::Ak74m => "ak74m",
            WeaponId::Akms => "akms",
            WeaponId::Fn2000 => "fn2000",
            WeaponId::M870 => "m870",
            WeaponId::Mk12 => "mk12",
            WeaponId::Mp7 => "mp7",
            WeaponId::Mk46 => "mk46",
            WeaponId::M1014 => "m1014",
            WeaponId::Hk45 => "hk45",
            WeaponId::G18 => "g18",
            WeaponId::Pkm => "pkm",
            WeaponId::Mk18bare => "mk18bare",
            WeaponId::G19bare => "g19bare",
        }
    }
}

/// (TypeScript: WeaponDef, src/game/weapons.ts:55)
#[derive(Clone, Debug)]
pub struct WeaponDef {
    pub id: crate::weapons::WeaponId,
    pub enabled: bool,
    pub name: String,
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
    pub damage: crate::weapons::WeaponDefDamage,
    pub muzzle_ahead: f64,
    pub rig: crate::weapons::WeaponDefRig,
    pub handling: Option<f64>,
    pub light: crate::weapons::WeaponDefLight,
    pub drop: crate::slotsetup::SetupGun,
    pub tracer: Option<crate::weapons::TracerSpec>,
    pub pellets: Option<crate::weapons::PelletSpec>,
    pub hands: Option<crate::weapons::WeaponDefHands>,
    pub launcher: Option<bool>,
    pub weight_kg: f64,
    pub bulk: f64,
    pub own: Option<crate::weapons::WeaponDefOwn>,
}
impl WeaponDef {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WeaponDef {
            id: crate::weapons::WeaponId::Fiveseven,
            enabled: false,
            name: String::new(),
            short: String::new(),
            calibre: String::new(),
            cls: crate::weapons::WeaponClass::Pistol,
            mag_capacity: 0.0,
            spare_mags: 0.0,
            top_up: false,
            reload: crate::weapons::WeaponDefReload::zeroed(),
            modes: Vec::new(),
            cd: 0.0,
            cd_jitter: 0.0,
            burst: None,
            auto: None,
            report: crate::weapons::WeaponDefReport::zeroed(),
            suppressed: false,
            muzzle_fx: crate::effects::MuzzleFxKey::Mk12,
            prop_impulse: 0.0,
            ai_spread: crate::weapons::AiSpread::zeroed(),
            damage: crate::weapons::WeaponDefDamage::OneShot,
            muzzle_ahead: 0.0,
            rig: crate::weapons::WeaponDefRig::Pistol,
            handling: None,
            light: crate::weapons::WeaponDefLight::None,
            drop: crate::slotsetup::SetupGun::Pistol,
            tracer: None,
            pellets: None,
            hands: None,
            launcher: None,
            weight_kg: 0.0,
            bulk: 0.0,
            own: None,
        }
    }
}
impl Default for WeaponDef { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum WeaponClass {
    Pistol,
    Carbine,
    Rifle,
    Shotgun,
}
impl WeaponClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            WeaponClass::Pistol => "pistol",
            WeaponClass::Carbine => "carbine",
            WeaponClass::Rifle => "rifle",
            WeaponClass::Shotgun => "shotgun",
        }
    }
}

/// (TypeScript: inline at src/game/weapons.ts:70 WeaponDef.reload, src/game/weapons.ts:70)
#[derive(Clone, Copy, Debug)]
pub struct WeaponDefReload {
    pub dur: f64,
    pub out: Option<f64>,
    pub in_: Option<f64>,
    pub rack: Option<f64>,
    pub shell: Option<crate::gunreload::ShellRowShell>,
}
impl WeaponDefReload {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WeaponDefReload {
            dur: 0.0,
            out: None,
            in_: None,
            rack: None,
            shell: None,
        }
    }
}
impl Default for WeaponDefReload { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum FireMode {
    Semi,
    Auto,
    Burst,
}
impl FireMode {
    pub const fn as_str(self) -> &'static str {
        match self {
            FireMode::Semi => "semi",
            FireMode::Auto => "auto",
            FireMode::Burst => "burst",
        }
    }
}

/// (TypeScript: BurstSpec, src/game/weapons.ts:32)
#[derive(Clone, Copy, Debug)]
pub struct BurstSpec {
    pub min: f64,
    pub max: f64,
    pub interval: f64,
    pub gap_min: f64,
    pub gap_max: f64,
    pub spread_per_round: f64,
    pub spread_cap: f64,
}
impl BurstSpec {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BurstSpec {
            min: 0.0,
            max: 0.0,
            interval: 0.0,
            gap_min: 0.0,
            gap_max: 0.0,
            spread_per_round: 0.0,
            spread_cap: 0.0,
        }
    }
}
impl Default for BurstSpec { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: AutoSpec, src/game/weapons.ts:43)
#[derive(Clone, Copy, Debug)]
pub struct AutoSpec {
    pub interval: f64,
    pub spread_per_round: f64,
    pub spread_cap: f64,
    pub reset_after: f64,
}
impl AutoSpec {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AutoSpec {
            interval: 0.0,
            spread_per_round: 0.0,
            spread_cap: 0.0,
            reset_after: 0.0,
        }
    }
}
impl Default for AutoSpec { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/weapons.ts:84 WeaponDef.report, src/game/weapons.ts:84)
#[derive(Clone, Debug)]
pub struct WeaponDefReport {
    pub level: f64,
    pub loud: bool,
    pub voice: String,
}
impl WeaponDefReport {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WeaponDefReport {
            level: 0.0,
            loud: false,
            voice: String::new(),
        }
    }
}
impl Default for WeaponDefReport { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: AiSpread, src/game/weapons.ts:27)
#[derive(Clone, Copy, Debug)]
pub struct AiSpread {
    pub base: f64,
    pub move_k: f64,
    pub move_cap: f64,
    pub range_from: f64,
    pub range_k: f64,
}
impl AiSpread {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        AiSpread {
            base: 0.0,
            move_k: 0.0,
            move_cap: 0.0,
            range_from: 0.0,
            range_k: 0.0,
        }
    }
}
impl Default for AiSpread { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum WeaponDefDamage {
    OneShot,
}
impl WeaponDefDamage {
    pub const fn as_str(self) -> &'static str {
        match self {
            WeaponDefDamage::OneShot => "oneShot",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum WeaponDefRig {
    Pistol,
    Carbine,
    GuardPistol,
    Sc20k,
}
impl WeaponDefRig {
    pub const fn as_str(self) -> &'static str {
        match self {
            WeaponDefRig::Pistol => "pistol",
            WeaponDefRig::Carbine => "carbine",
            WeaponDefRig::GuardPistol => "guardPistol",
            WeaponDefRig::Sc20k => "sc20k",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum WeaponDefLight {
    None,
    Rail,
    Handguard,
}
impl WeaponDefLight {
    pub const fn as_str(self) -> &'static str {
        match self {
            WeaponDefLight::None => "none",
            WeaponDefLight::Rail => "rail",
            WeaponDefLight::Handguard => "handguard",
        }
    }
}

/// (TypeScript: TracerSpec, src/game/weapons.ts:45)
#[derive(Clone, Copy, Debug)]
pub struct TracerSpec {
    pub every: f64,
    pub last_n: f64,
    pub color: V3,
}
impl TracerSpec {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        TracerSpec {
            every: 0.0,
            last_n: 0.0,
            color: [0.0; 3],
        }
    }
}
impl Default for TracerSpec { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: PelletSpec, src/game/weapons.ts:53)
#[derive(Clone, Copy, Debug)]
pub struct PelletSpec {
    pub n: f64,
    pub cone: f64,
    pub drop_m: f64,
    pub need: f64,
    pub breach: Option<crate::weapons::PelletSpecBreach>,
}
impl PelletSpec {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PelletSpec {
            n: 0.0,
            cone: 0.0,
            drop_m: 0.0,
            need: 0.0,
            breach: None,
        }
    }
}
impl Default for PelletSpec { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/weapons.ts:53 PelletSpec.breach, src/game/weapons.ts:53)
#[derive(Clone, Copy, Debug)]
pub struct PelletSpecBreach {
    pub reach_m: f64,
    pub latch_m: f64,
    pub need: f64,
}
impl PelletSpecBreach {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PelletSpecBreach {
            reach_m: 0.0,
            latch_m: 0.0,
            need: 0.0,
        }
    }
}
impl Default for PelletSpecBreach { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/weapons.ts:111 WeaponDef.hands, src/game/weapons.ts:111)
#[derive(Clone, Copy, Debug)]
pub struct WeaponDefHands {
    pub still: f64,
    pub moving: f64,
    pub crouch: Option<f64>,
}
impl WeaponDefHands {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WeaponDefHands {
            still: 0.0,
            moving: 0.0,
            crouch: None,
        }
    }
}
impl Default for WeaponDefHands { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/weapons.ts:120 WeaponDef.own, src/game/weapons.ts:120)
#[derive(Clone, Debug)]
pub struct WeaponDefOwn {
    pub model: crate::weapons::GunModelId,
    pub voice: Option<String>,
    pub flame: Option<crate::effects::MuzzleFxKey>,
}
impl WeaponDefOwn {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WeaponDefOwn {
            model: crate::weapons::GunModelId::M870,
            voice: None,
            flame: None,
        }
    }
}
impl Default for WeaponDefOwn { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Ocp, src/game/weapons.ts:542)
#[derive(Clone, Copy, Debug)]
pub struct Ocp {
    pub charge: f64,
    pub recharge_per_sec: f64,
    pub cost: f64,
}
impl Ocp {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
    pub fn blank() -> Self {
        Ocp {
            charge: 1.0,
            recharge_per_sec: 0.28,
            cost: 1.0,
        }
    }
}
impl Default for Ocp { fn default() -> Self { Self::blank() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum RifleAttachment {
    Ring,
    He40,
    Cam,
}
impl RifleAttachment {
    pub const fn as_str(self) -> &'static str {
        match self {
            RifleAttachment::Ring => "ring",
            RifleAttachment::He40 => "he40",
            RifleAttachment::Cam => "cam",
        }
    }
}

/// (TypeScript: BurstState, src/game/weapons.ts:37)
#[derive(Clone, Copy, Debug)]
pub struct BurstState {
    pub left: f64,
    pub n: f64,
    pub next_t: f64,
    pub idx: f64,
    pub t: f64,
    pub off: Option<V3>,
}
impl BurstState {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BurstState {
            left: 0.0,
            n: 0.0,
            next_t: 0.0,
            idx: 0.0,
            t: 0.0,
            off: None,
        }
    }
}
impl Default for BurstState { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Firearm, src/game/weapons.ts:353)
#[derive(Clone, Debug)]
pub struct Firearm {
    pub chamber: f64,
    pub infinite: bool,
    pub mag: Option<f64>,
    pub spare: Vec<f64>,
    pub reload_t: f64,
    pub stage: f64,
    pub ev: crate::weapons::ReloadEvents,
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
    pub last_burst_t: f64,
    pub mode: crate::weapons::FireMode,
    pub auto_idx: f64,
    pub auto_since: f64,
    pub def: crate::weapons::WeaponId,
    /// which of the three a gun is (TypeScript: the classes Firearm, Pistol, Rifle)
    pub kind: crate::weapons::GunKind,
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
impl Firearm {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
    pub fn blank() -> Self {
        Firearm {
            chamber: 1.0,
            infinite: false,
            mag: None,
            spare: Vec::new(),
            reload_t: -1.0,
            stage: 0.0,
            ev: crate::weapons::ReloadEvents::None,
            beat: None,
            cues: None,
            cue_i: 0,
            feed_n: 0.0,
            feed_port: false,
            shell_t: None,
            shell_i: 0,
            shell_port: false,
            feed_pull: false,
            shots_fired: 0.0,
            burst: None,
            bursts: 0.0,
            last_burst_t: -100.0,
            mode: crate::weapons::FireMode::Semi,
            auto_idx: 0.0,
            auto_since: 0.0,
            def: crate::weapons::WeaponId::Fiveseven,
            kind: crate::weapons::GunKind::Plain,
            light_on: false,
            attachments: alloc::vec![crate::weapons::RifleAttachment::Ring, crate::weapons::RifleAttachment::He40],
            att_idx: 0,
            rings: 0.0,
            ring_cd: 0.0,
            shells: 0.0,
            cams: 0.0,
        }
    }
}
impl Default for Firearm { fn default() -> Self { Self::blank() } }

/// (TypeScript: inline at src/game/weapons.ts:324 bareRow(over), ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1583)
#[derive(Clone, Debug)]
pub struct BareRowOver {
    pub id: Option<crate::weapons::WeaponId>,
    pub enabled: Option<bool>,
    pub name: Option<String>,
    pub short: Option<String>,
    pub calibre: Option<String>,
    pub cls: Option<crate::weapons::WeaponClass>,
    pub mag_capacity: Option<f64>,
    pub spare_mags: Option<f64>,
    pub top_up: Option<bool>,
    pub reload: Option<crate::weapons::WeaponDefReload>,
    pub modes: Option<Vec<crate::weapons::FireMode>>,
    pub cd: Option<f64>,
    pub cd_jitter: Option<f64>,
    pub burst: Option<crate::weapons::BurstSpec>,
    pub auto: Option<crate::weapons::AutoSpec>,
    pub report: Option<crate::weapons::WeaponDefReport>,
    pub suppressed: Option<bool>,
    pub muzzle_fx: Option<crate::effects::MuzzleFxKey>,
    pub prop_impulse: Option<f64>,
    pub ai_spread: Option<crate::weapons::AiSpread>,
    pub damage: Option<crate::weapons::BareRowOverDamage>,
    pub muzzle_ahead: Option<f64>,
    pub rig: Option<crate::weapons::BareRowOverRig>,
    pub handling: Option<f64>,
    pub light: Option<crate::weapons::BareRowOverLight>,
    pub drop: Option<crate::slotsetup::SetupGun>,
    pub tracer: Option<crate::weapons::TracerSpec>,
    pub pellets: Option<crate::weapons::PelletSpec>,
    pub hands: Option<crate::weapons::WeaponDefHands>,
    pub launcher: Option<bool>,
    pub weight_kg: Option<f64>,
    pub bulk: Option<f64>,
    pub own: Option<crate::weapons::WeaponDefOwn>,
}
impl BareRowOver {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BareRowOver {
            id: None,
            enabled: None,
            name: None,
            short: None,
            calibre: None,
            cls: None,
            mag_capacity: None,
            spare_mags: None,
            top_up: None,
            reload: None,
            modes: None,
            cd: None,
            cd_jitter: None,
            burst: None,
            auto: None,
            report: None,
            suppressed: None,
            muzzle_fx: None,
            prop_impulse: None,
            ai_spread: None,
            damage: None,
            muzzle_ahead: None,
            rig: None,
            handling: None,
            light: None,
            drop: None,
            tracer: None,
            pellets: None,
            hands: None,
            launcher: None,
            weight_kg: None,
            bulk: None,
            own: None,
        }
    }
}
impl Default for BareRowOver { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum BareRowOverDamage {
    OneShot,
}
impl BareRowOverDamage {
    pub const fn as_str(self) -> &'static str {
        match self {
            BareRowOverDamage::OneShot => "oneShot",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum BareRowOverRig {
    Pistol,
    Carbine,
    GuardPistol,
    Sc20k,
}
impl BareRowOverRig {
    pub const fn as_str(self) -> &'static str {
        match self {
            BareRowOverRig::Pistol => "pistol",
            BareRowOverRig::Carbine => "carbine",
            BareRowOverRig::GuardPistol => "guardPistol",
            BareRowOverRig::Sc20k => "sc20k",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum BareRowOverLight {
    None,
    Rail,
    Handguard,
}
impl BareRowOverLight {
    pub const fn as_str(self) -> &'static str {
        match self {
            BareRowOverLight::None => "none",
            BareRowOverLight::Rail => "rail",
            BareRowOverLight::Handguard => "handguard",
        }
    }
}

/// (TypeScript: inline at src/game/weapons.ts:427 startReload(shells), src/game/weapons.ts:427)
#[derive(Clone, Debug)]
pub struct FirearmStartReloadShells {
    pub times: Vec<f64>,
    pub port: bool,
}
impl FirearmStartReloadShells {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FirearmStartReloadShells {
            times: Vec::new(),
            port: false,
        }
    }
}
impl Default for FirearmStartReloadShells { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct WeaponsMod {
    /// (src/game/weapons.ts:325 WEAPONS)
    pub weapons: JsMap<crate::weapons::WeaponId, crate::weapons::WeaponDef>,
}
impl WeaponsMod {
    pub fn blank() -> Self {
        WeaponsMod {
            weapons: { let mut t = JsMap::new(); t.set(crate::weapons::WeaponId::Fiveseven, crate::weapons::WeaponDef { id: crate::weapons::WeaponId::Fiveseven, enabled: true, name: String::from("Glock 19 (suppressed)"), short: String::from("Glock 19"), calibre: String::from("9×19 mm"), cls: crate::weapons::WeaponClass::Pistol, mag_capacity: 15.0, spare_mags: 3.0, top_up: true, reload: crate::weapons::WeaponDefReload { dur: 1.6, out: Some(0.38), in_: Some(1.02), rack: Some(1.38), shell: None }, modes: alloc::vec![crate::weapons::FireMode::Semi], cd: 0.22, cd_jitter: 0.0, burst: None, auto: None, report: crate::weapons::WeaponDefReport { level: 0.6, loud: false, voice: String::from("pistolSuppressed") }, suppressed: true, muzzle_fx: crate::effects::MuzzleFxKey::PistolSuppressed, prop_impulse: 4.0, ai_spread: crate::weapons::AiSpread { base: 0.05, move_k: 0.014, move_cap: 0.06, range_from: 8.0, range_k: 0.006 }, damage: crate::weapons::WeaponDefDamage::OneShot, muzzle_ahead: 0.26, rig: crate::weapons::WeaponDefRig::Pistol, handling: None, light: crate::weapons::WeaponDefLight::Rail, drop: crate::slotsetup::SetupGun::Pistol, tracer: None, pellets: None, hands: None, launcher: None, weight_kg: 0.9, bulk: 0.0, own: None }); t.set(crate::weapons::WeaponId::Guard9mm, crate::weapons::WeaponDef { id: crate::weapons::WeaponId::Guard9mm, enabled: true, name: String::from("9 mm service pistol"), short: String::from("sidearm"), calibre: String::from("9×19 mm"), cls: crate::weapons::WeaponClass::Pistol, mag_capacity: 12.0, spare_mags: f64::INFINITY, top_up: false, reload: crate::weapons::WeaponDefReload { dur: 1.7, out: None, in_: None, rack: None, shell: None }, modes: alloc::vec![crate::weapons::FireMode::Semi], cd: 0.55, cd_jitter: 0.5, burst: None, auto: None, report: crate::weapons::WeaponDefReport { level: 1.0, loud: true, voice: String::from("pistolLoud") }, suppressed: false, muzzle_fx: crate::effects::MuzzleFxKey::PistolBare, prop_impulse: 6.0, ai_spread: crate::weapons::AiSpread { base: 0.05, move_k: 0.014, move_cap: 0.06, range_from: 8.0, range_k: 0.006 }, damage: crate::weapons::WeaponDefDamage::OneShot, muzzle_ahead: 0.26, rig: crate::weapons::WeaponDefRig::GuardPistol, handling: None, light: crate::weapons::WeaponDefLight::Rail, drop: crate::slotsetup::SetupGun::Pistol, tracer: None, pellets: None, hands: None, launcher: None, weight_kg: 0.95, bulk: 0.0, own: None }); t.set(crate::weapons::WeaponId::Aks74u, crate::weapons::WeaponDef { id: crate::weapons::WeaponId::Aks74u, enabled: true, name: String::from("AKS-74U carbine"), short: String::from("AKS-74U"), calibre: String::from("5.45×39 mm"), cls: crate::weapons::WeaponClass::Carbine, mag_capacity: 30.0, spare_mags: f64::INFINITY, top_up: false, reload: crate::weapons::WeaponDefReload { dur: 2.6, out: Some(0.7), in_: Some(1.7), rack: Some(2.2), shell: None }, modes: alloc::vec![crate::weapons::FireMode::Auto, crate::weapons::FireMode::Semi], cd: 0.1, cd_jitter: 0.0, burst: Some(crate::weapons::BurstSpec { min: 2.0, max: 4.0, interval: 0.1, gap_min: 0.9, gap_max: 1.6, spread_per_round: 0.03, spread_cap: 0.16 }), auto: None, report: crate::weapons::WeaponDefReport { level: 1.0, loud: true, voice: String::from("carbineReport") }, suppressed: false, muzzle_fx: crate::effects::MuzzleFxKey::Carbine, prop_impulse: 6.0, ai_spread: crate::weapons::AiSpread { base: 0.035, move_k: 0.014, move_cap: 0.06, range_from: 8.0, range_k: 0.006 }, damage: crate::weapons::WeaponDefDamage::OneShot, muzzle_ahead: 0.5, rig: crate::weapons::WeaponDefRig::Carbine, handling: None, light: crate::weapons::WeaponDefLight::Handguard, drop: crate::slotsetup::SetupGun::Rifle, tracer: Some(crate::weapons::TracerSpec { every: 3.0, last_n: 3.0, color: [1.0, 0.42, 0.12] }), pellets: None, hands: None, launcher: None, weight_kg: 3.0, bulk: 2.0, own: None }); t.set(crate::weapons::WeaponId::Fn2000, crate::weapons::WeaponDef { id: crate::weapons::WeaponId::Fn2000, enabled: true, name: String::from("Mk 18 (suppressed)"), short: String::from("Mk 18"), calibre: String::from("5.56×45 mm"), cls: crate::weapons::WeaponClass::Rifle, mag_capacity: 30.0, spare_mags: 2.0, top_up: true, reload: crate::weapons::WeaponDefReload { dur: 2.4, out: Some(0.5), in_: Some(1.5), rack: Some(2.0), shell: None }, modes: alloc::vec![crate::weapons::FireMode::Semi, crate::weapons::FireMode::Auto], cd: 0.2, cd_jitter: 0.0, burst: None, auto: Some(crate::weapons::AutoSpec { interval: 0.1, spread_per_round: 0.012, spread_cap: 0.08, reset_after: 0.25 }), report: crate::weapons::WeaponDefReport { level: 1.0, loud: true, voice: String::from("rifleSuppressed") }, suppressed: true, muzzle_fx: crate::effects::MuzzleFxKey::Sc20k, prop_impulse: 5.0, ai_spread: crate::weapons::AiSpread { base: 0.035, move_k: 0.014, move_cap: 0.06, range_from: 8.0, range_k: 0.006 }, damage: crate::weapons::WeaponDefDamage::OneShot, muzzle_ahead: 0.52, rig: crate::weapons::WeaponDefRig::Sc20k, handling: Some(0.8), light: crate::weapons::WeaponDefLight::Handguard, drop: crate::slotsetup::SetupGun::Rifle, tracer: None, pellets: None, hands: None, launcher: None, weight_kg: 3.9, bulk: 2.0, own: None }); t.set(crate::weapons::WeaponId::M870, crate::weapons::WeaponDef { id: crate::weapons::WeaponId::M870, enabled: true, name: String::from("M870 breaching shotgun"), short: String::from("M870"), calibre: String::from("12 gauge"), cls: crate::weapons::WeaponClass::Shotgun, mag_capacity: 6.0, spare_mags: 3.0, top_up: true, reload: crate::weapons::WeaponDefReload { dur: 3.2, out: Some(0.4), in_: Some(2.5), rack: Some(2.9), shell: Some(crate::gunreload::ShellRowShell { lead: 0.4, each: 0.35, tail: 0.7, bolt: 0.4 }) }, modes: alloc::vec![crate::weapons::FireMode::Semi], cd: 0.9, cd_jitter: 0.2, burst: None, auto: None, report: crate::weapons::WeaponDefReport { level: 1.0, loud: true, voice: String::from("carbineReport") }, suppressed: false, muzzle_fx: crate::effects::MuzzleFxKey::Carbine, prop_impulse: 2.0, ai_spread: crate::weapons::AiSpread { base: 0.035, move_k: 0.014, move_cap: 0.06, range_from: 8.0, range_k: 0.006 }, damage: crate::weapons::WeaponDefDamage::OneShot, muzzle_ahead: 0.5, rig: crate::weapons::WeaponDefRig::Carbine, handling: None, light: crate::weapons::WeaponDefLight::Handguard, drop: crate::slotsetup::SetupGun::Rifle, tracer: None, pellets: Some(crate::weapons::PelletSpec { n: 9.0, cone: 0.06, drop_m: 8.0, need: 4.0, breach: Some(crate::weapons::PelletSpecBreach { reach_m: 2.0, latch_m: 0.5, need: 4.0 }) }), hands: None, launcher: None, weight_kg: 3.4, bulk: 3.0, own: Some(crate::weapons::WeaponDefOwn { model: crate::weapons::GunModelId::M870, voice: Some(String::from("m870Report")), flame: None }) }); t.set(crate::weapons::WeaponId::Mk12, crate::weapons::WeaponDef { id: crate::weapons::WeaponId::Mk12, enabled: true, name: String::from("Mk 12 SPR (suppressed)"), short: String::from("Mk 12"), calibre: String::from("5.56×45 mm"), cls: crate::weapons::WeaponClass::Rifle, mag_capacity: 20.0, spare_mags: 3.0, top_up: true, reload: crate::weapons::WeaponDefReload { dur: 2.6, out: Some(0.55), in_: Some(1.65), rack: Some(2.2), shell: None }, modes: alloc::vec![crate::weapons::FireMode::Semi], cd: 0.3, cd_jitter: 0.15, burst: None, auto: None, report: crate::weapons::WeaponDefReport { level: 1.0, loud: true, voice: String::from("rifleSuppressed") }, suppressed: true, muzzle_fx: crate::effects::MuzzleFxKey::Sc20k, prop_impulse: 5.0, ai_spread: crate::weapons::AiSpread { base: 0.018, move_k: 0.02, move_cap: 0.08, range_from: 16.0, range_k: 0.003 }, damage: crate::weapons::WeaponDefDamage::OneShot, muzzle_ahead: 0.52, rig: crate::weapons::WeaponDefRig::Sc20k, handling: Some(0.62), light: crate::weapons::WeaponDefLight::Handguard, drop: crate::slotsetup::SetupGun::Rifle, tracer: None, pellets: None, hands: Some(crate::weapons::WeaponDefHands { still: 0.45, moving: 1.4, crouch: None }), launcher: Some(false), weight_kg: 5.0, bulk: 3.0, own: Some(crate::weapons::WeaponDefOwn { model: crate::weapons::GunModelId::Mk12, voice: Some(String::from("mk12Suppressed")), flame: Some(crate::effects::MuzzleFxKey::Mk12) }) }); t.set(crate::weapons::WeaponId::Mp7, crate::weapons::WeaponDef { id: crate::weapons::WeaponId::Mp7, enabled: true, name: String::from("MP7A1 (suppressed)"), short: String::from("MP7"), calibre: String::from("4.6×30 mm"), cls: crate::weapons::WeaponClass::Carbine, mag_capacity: 40.0, spare_mags: 3.0, top_up: true, reload: crate::weapons::WeaponDefReload { dur: 2.0, out: Some(0.4), in_: Some(1.25), rack: Some(1.65), shell: None }, modes: alloc::vec![crate::weapons::FireMode::Auto, crate::weapons::FireMode::Semi], cd: 0.12, cd_jitter: 0.0, burst: Some(crate::weapons::BurstSpec { min: 2.0, max: 4.0, interval: 0.06666666666666667, gap_min: 0.5, gap_max: 0.9, spread_per_round: 0.02, spread_cap: 0.12 }), auto: Some(crate::weapons::AutoSpec { interval: 0.06666666666666667, spread_per_round: 0.008, spread_cap: 0.07, reset_after: 0.25 }), report: crate::weapons::WeaponDefReport { level: 0.8, loud: false, voice: String::from("pistolSuppressed") }, suppressed: true, muzzle_fx: crate::effects::MuzzleFxKey::PistolSuppressed, prop_impulse: 3.0, ai_spread: crate::weapons::AiSpread { base: 0.045, move_k: 0.008, move_cap: 0.03, range_from: 7.0, range_k: 0.012 }, damage: crate::weapons::WeaponDefDamage::OneShot, muzzle_ahead: 0.4, rig: crate::weapons::WeaponDefRig::Sc20k, handling: Some(0.95), light: crate::weapons::WeaponDefLight::Handguard, drop: crate::slotsetup::SetupGun::Rifle, tracer: None, pellets: None, hands: Some(crate::weapons::WeaponDefHands { still: 1.6, moving: 0.7, crouch: None }), launcher: Some(false), weight_kg: 2.3, bulk: 1.0, own: Some(crate::weapons::WeaponDefOwn { model: crate::weapons::GunModelId::Mp7, voice: Some(String::from("mp7Suppressed")), flame: Some(crate::effects::MuzzleFxKey::Mp7) }) }); t.set(crate::weapons::WeaponId::Mk46, crate::weapons::WeaponDef { id: crate::weapons::WeaponId::Mk46, enabled: true, name: String::from("Mk 46 light machine gun"), short: String::from("Mk 46"), calibre: String::from("5.56×45 mm"), cls: crate::weapons::WeaponClass::Rifle, mag_capacity: 100.0, spare_mags: 1.0, top_up: true, reload: crate::weapons::WeaponDefReload { dur: 6.0, out: Some(1.2), in_: Some(4.4), rack: Some(5.4), shell: None }, modes: alloc::vec![crate::weapons::FireMode::Auto], cd: 0.08333333333333333, cd_jitter: 0.0, burst: Some(crate::weapons::BurstSpec { min: 5.0, max: 9.0, interval: 0.08333333333333333, gap_min: 0.5, gap_max: 1.0, spread_per_round: 0.012, spread_cap: 0.14 }), auto: Some(crate::weapons::AutoSpec { interval: 0.08333333333333333, spread_per_round: 0.004, spread_cap: 0.06, reset_after: 0.3 }), report: crate::weapons::WeaponDefReport { level: 1.0, loud: true, voice: String::from("carbineReport") }, suppressed: false, muzzle_fx: crate::effects::MuzzleFxKey::Carbine, prop_impulse: 6.0, ai_spread: crate::weapons::AiSpread { base: 0.05, move_k: 0.03, move_cap: 0.12, range_from: 10.0, range_k: 0.004 }, damage: crate::weapons::WeaponDefDamage::OneShot, muzzle_ahead: 0.52, rig: crate::weapons::WeaponDefRig::Sc20k, handling: Some(0.5), light: crate::weapons::WeaponDefLight::None, drop: crate::slotsetup::SetupGun::Rifle, tracer: Some(crate::weapons::TracerSpec { every: 5.0, last_n: 0.0, color: [1.0, 0.25, 0.12] }), pellets: None, hands: Some(crate::weapons::WeaponDefHands { still: 1.5, moving: 2.4, crouch: Some(0.5) }), launcher: Some(false), weight_kg: 8.0, bulk: 4.0, own: Some(crate::weapons::WeaponDefOwn { model: crate::weapons::GunModelId::Mk46, voice: Some(String::from("mk46Report")), flame: Some(crate::effects::MuzzleFxKey::Mk46) }) }); t.set(crate::weapons::WeaponId::M1014, crate::weapons::WeaponDef { id: crate::weapons::WeaponId::M1014, enabled: true, name: String::from("M1014 shotgun"), short: String::from("M1014"), calibre: String::from("12 gauge"), cls: crate::weapons::WeaponClass::Shotgun, mag_capacity: 7.0, spare_mags: 3.0, top_up: true, reload: crate::weapons::WeaponDefReload { dur: 3.6, out: Some(0.4), in_: Some(2.9), rack: Some(3.3), shell: Some(crate::gunreload::ShellRowShell { lead: 0.4, each: 0.36, tail: 0.7, bolt: 0.4 }) }, modes: alloc::vec![crate::weapons::FireMode::Semi], cd: 0.28, cd_jitter: 0.1, burst: None, auto: None, report: crate::weapons::WeaponDefReport { level: 1.0, loud: true, voice: String::from("carbineReport") }, suppressed: false, muzzle_fx: crate::effects::MuzzleFxKey::Carbine, prop_impulse: 2.0, ai_spread: crate::weapons::AiSpread { base: 0.035, move_k: 0.014, move_cap: 0.06, range_from: 8.0, range_k: 0.006 }, damage: crate::weapons::WeaponDefDamage::OneShot, muzzle_ahead: 0.5, rig: crate::weapons::WeaponDefRig::Carbine, handling: Some(0.72), light: crate::weapons::WeaponDefLight::Handguard, drop: crate::slotsetup::SetupGun::Rifle, tracer: None, pellets: Some(crate::weapons::PelletSpec { n: 9.0, cone: 0.045, drop_m: 10.0, need: 4.0, breach: None }), hands: None, launcher: Some(false), weight_kg: 3.9, bulk: 3.0, own: Some(crate::weapons::WeaponDefOwn { model: crate::weapons::GunModelId::M1014, voice: Some(String::from("m1014Report")), flame: Some(crate::effects::MuzzleFxKey::M1014) }) }); t.set(crate::weapons::WeaponId::Hk45, crate::weapons::WeaponDef { id: crate::weapons::WeaponId::Hk45, enabled: true, name: String::from("HK45 (suppressed)"), short: String::from("HK45"), calibre: String::from(".45 ACP"), cls: crate::weapons::WeaponClass::Pistol, mag_capacity: 8.0, spare_mags: 3.0, top_up: true, reload: crate::weapons::WeaponDefReload { dur: 1.7, out: Some(0.4), in_: Some(1.1), rack: Some(1.45), shell: None }, modes: alloc::vec![crate::weapons::FireMode::Semi], cd: 0.3, cd_jitter: 0.0, burst: None, auto: None, report: crate::weapons::WeaponDefReport { level: 0.45, loud: false, voice: String::from("pistolSuppressed") }, suppressed: true, muzzle_fx: crate::effects::MuzzleFxKey::PistolSuppressed, prop_impulse: 5.0, ai_spread: crate::weapons::AiSpread { base: 0.05, move_k: 0.014, move_cap: 0.06, range_from: 8.0, range_k: 0.006 }, damage: crate::weapons::WeaponDefDamage::OneShot, muzzle_ahead: 0.26, rig: crate::weapons::WeaponDefRig::Pistol, handling: None, light: crate::weapons::WeaponDefLight::Rail, drop: crate::slotsetup::SetupGun::Pistol, tracer: None, pellets: None, hands: None, launcher: None, weight_kg: 1.2, bulk: 0.0, own: Some(crate::weapons::WeaponDefOwn { model: crate::weapons::GunModelId::Hk45, voice: Some(String::from("hk45Suppressed")), flame: Some(crate::effects::MuzzleFxKey::Hk45) }) }); t.set(crate::weapons::WeaponId::G18, crate::weapons::WeaponDef { id: crate::weapons::WeaponId::G18, enabled: true, name: String::from("Glock 18"), short: String::from("Glock 18"), calibre: String::from("9×19 mm"), cls: crate::weapons::WeaponClass::Pistol, mag_capacity: 33.0, spare_mags: 2.0, top_up: true, reload: crate::weapons::WeaponDefReload { dur: 1.8, out: Some(0.4), in_: Some(1.15), rack: Some(1.5), shell: None }, modes: alloc::vec![crate::weapons::FireMode::Auto, crate::weapons::FireMode::Semi], cd: 0.2, cd_jitter: 0.0, burst: Some(crate::weapons::BurstSpec { min: 3.0, max: 5.0, interval: 0.05, gap_min: 0.5, gap_max: 0.9, spread_per_round: 0.03, spread_cap: 0.18 }), auto: Some(crate::weapons::AutoSpec { interval: 0.05, spread_per_round: 0.02, spread_cap: 0.16, reset_after: 0.25 }), report: crate::weapons::WeaponDefReport { level: 1.0, loud: true, voice: String::from("pistolLoud") }, suppressed: false, muzzle_fx: crate::effects::MuzzleFxKey::PistolBare, prop_impulse: 6.0, ai_spread: crate::weapons::AiSpread { base: 0.06, move_k: 0.014, move_cap: 0.06, range_from: 8.0, range_k: 0.006 }, damage: crate::weapons::WeaponDefDamage::OneShot, muzzle_ahead: 0.26, rig: crate::weapons::WeaponDefRig::GuardPistol, handling: None, light: crate::weapons::WeaponDefLight::Rail, drop: crate::slotsetup::SetupGun::Pistol, tracer: None, pellets: None, hands: None, launcher: None, weight_kg: 1.0, bulk: 0.0, own: Some(crate::weapons::WeaponDefOwn { model: crate::weapons::GunModelId::G18, voice: Some(String::from("g18Report")), flame: Some(crate::effects::MuzzleFxKey::G18) }) }); t.set(crate::weapons::WeaponId::Ak74m, crate::weapons::WeaponDef { id: crate::weapons::WeaponId::Ak74m, enabled: false, name: String::from("AK-74M rifle"), short: String::from("AK-74M"), calibre: String::from("5.45×39 mm"), cls: crate::weapons::WeaponClass::Rifle, mag_capacity: 30.0, spare_mags: f64::INFINITY, top_up: false, reload: crate::weapons::WeaponDefReload { dur: 2.6, out: Some(0.7), in_: Some(1.7), rack: Some(2.2), shell: None }, modes: alloc::vec![crate::weapons::FireMode::Auto, crate::weapons::FireMode::Semi], cd: 0.1, cd_jitter: 0.0, burst: Some(crate::weapons::BurstSpec { min: 2.0, max: 4.0, interval: 0.1, gap_min: 0.9, gap_max: 1.6, spread_per_round: 0.025, spread_cap: 0.16 }), auto: None, report: crate::weapons::WeaponDefReport { level: 1.0, loud: true, voice: String::from("carbineReport") }, suppressed: false, muzzle_fx: crate::effects::MuzzleFxKey::Carbine, prop_impulse: 6.0, ai_spread: crate::weapons::AiSpread { base: 0.03, move_k: 0.014, move_cap: 0.06, range_from: 8.0, range_k: 0.006 }, damage: crate::weapons::WeaponDefDamage::OneShot, muzzle_ahead: 0.66, rig: crate::weapons::WeaponDefRig::Carbine, handling: None, light: crate::weapons::WeaponDefLight::Handguard, drop: crate::slotsetup::SetupGun::Rifle, tracer: Some(crate::weapons::TracerSpec { every: 3.0, last_n: 3.0, color: [1.0, 0.42, 0.12] }), pellets: None, hands: None, launcher: None, weight_kg: 3.6, bulk: 3.0, own: None }); t.set(crate::weapons::WeaponId::Akms, crate::weapons::WeaponDef { id: crate::weapons::WeaponId::Akms, enabled: true, name: String::from("AKMS rifle"), short: String::from("AKMS"), calibre: String::from("7.62×39 mm"), cls: crate::weapons::WeaponClass::Rifle, mag_capacity: 30.0, spare_mags: f64::INFINITY, top_up: false, reload: crate::weapons::WeaponDefReload { dur: 2.6, out: Some(0.7), in_: Some(1.7), rack: Some(2.2), shell: None }, modes: alloc::vec![crate::weapons::FireMode::Auto, crate::weapons::FireMode::Semi], cd: 0.1, cd_jitter: 0.0, burst: Some(crate::weapons::BurstSpec { min: 2.0, max: 3.0, interval: 0.1, gap_min: 0.9, gap_max: 1.6, spread_per_round: 0.04, spread_cap: 0.16 }), auto: None, report: crate::weapons::WeaponDefReport { level: 1.0, loud: true, voice: String::from("carbineReport") }, suppressed: false, muzzle_fx: crate::effects::MuzzleFxKey::PistolBare, prop_impulse: 6.0, ai_spread: crate::weapons::AiSpread { base: 0.04, move_k: 0.014, move_cap: 0.06, range_from: 8.0, range_k: 0.006 }, damage: crate::weapons::WeaponDefDamage::OneShot, muzzle_ahead: 0.62, rig: crate::weapons::WeaponDefRig::Carbine, handling: None, light: crate::weapons::WeaponDefLight::None, drop: crate::slotsetup::SetupGun::Rifle, tracer: None, pellets: None, hands: None, launcher: None, weight_kg: 3.5, bulk: 3.0, own: None }); t.set(crate::weapons::WeaponId::Pkm, crate::weapons::WeaponDef { id: crate::weapons::WeaponId::Pkm, enabled: true, name: String::from("PKM machine gun"), short: String::from("PKM"), calibre: String::from("7.62×54 mm R"), cls: crate::weapons::WeaponClass::Rifle, mag_capacity: 100.0, spare_mags: f64::INFINITY, top_up: false, reload: crate::weapons::WeaponDefReload { dur: 6.5, out: Some(1.4), in_: Some(5.0), rack: Some(5.9), shell: None }, modes: alloc::vec![crate::weapons::FireMode::Auto], cd: 0.1, cd_jitter: 0.0, burst: Some(crate::weapons::BurstSpec { min: 6.0, max: 10.0, interval: 0.1, gap_min: 0.5, gap_max: 1.0, spread_per_round: 0.012, spread_cap: 0.14 }), auto: None, report: crate::weapons::WeaponDefReport { level: 1.0, loud: true, voice: String::from("carbineReport") }, suppressed: false, muzzle_fx: crate::effects::MuzzleFxKey::Carbine, prop_impulse: 6.0, ai_spread: crate::weapons::AiSpread { base: 0.05, move_k: 0.03, move_cap: 0.12, range_from: 10.0, range_k: 0.004 }, damage: crate::weapons::WeaponDefDamage::OneShot, muzzle_ahead: 0.62, rig: crate::weapons::WeaponDefRig::Carbine, handling: Some(0.45), light: crate::weapons::WeaponDefLight::None, drop: crate::slotsetup::SetupGun::Rifle, tracer: Some(crate::weapons::TracerSpec { every: 5.0, last_n: 0.0, color: [1.0, 0.42, 0.12] }), pellets: None, hands: None, launcher: None, weight_kg: 7.5, bulk: 4.0, own: None }); t.set(crate::weapons::WeaponId::Mk18bare, crate::weapons::WeaponDef { id: crate::weapons::WeaponId::Mk18bare, enabled: true, name: String::from("Mk 18"), short: String::from("Mk 18"), calibre: String::from("5.56×45 mm"), cls: crate::weapons::WeaponClass::Rifle, mag_capacity: 30.0, spare_mags: 2.0, top_up: true, reload: crate::weapons::WeaponDefReload { dur: 2.4, out: Some(0.5), in_: Some(1.5), rack: Some(2.0), shell: None }, modes: alloc::vec![crate::weapons::FireMode::Semi, crate::weapons::FireMode::Auto], cd: 0.2, cd_jitter: 0.0, burst: None, auto: Some(crate::weapons::AutoSpec { interval: 0.1, spread_per_round: 0.012, spread_cap: 0.08, reset_after: 0.25 }), report: crate::weapons::WeaponDefReport { level: 1.0, loud: true, voice: String::from("carbineReport") }, suppressed: false, muzzle_fx: crate::effects::MuzzleFxKey::Carbine, prop_impulse: 6.0, ai_spread: crate::weapons::AiSpread { base: 0.035, move_k: 0.014, move_cap: 0.06, range_from: 8.0, range_k: 0.006 }, damage: crate::weapons::WeaponDefDamage::OneShot, muzzle_ahead: 0.52, rig: crate::weapons::WeaponDefRig::Sc20k, handling: Some(0.8), light: crate::weapons::WeaponDefLight::Handguard, drop: crate::slotsetup::SetupGun::Rifle, tracer: None, pellets: None, hands: None, launcher: None, weight_kg: 3.4, bulk: 2.0, own: None }); t.set(crate::weapons::WeaponId::G19bare, crate::weapons::WeaponDef { id: crate::weapons::WeaponId::G19bare, enabled: true, name: String::from("Glock 19"), short: String::from("Glock 19"), calibre: String::from("9×19 mm"), cls: crate::weapons::WeaponClass::Pistol, mag_capacity: 15.0, spare_mags: 3.0, top_up: true, reload: crate::weapons::WeaponDefReload { dur: 1.6, out: Some(0.38), in_: Some(1.02), rack: Some(1.38), shell: None }, modes: alloc::vec![crate::weapons::FireMode::Semi], cd: 0.22, cd_jitter: 0.0, burst: None, auto: None, report: crate::weapons::WeaponDefReport { level: 1.0, loud: true, voice: String::from("pistolLoud") }, suppressed: false, muzzle_fx: crate::effects::MuzzleFxKey::PistolBare, prop_impulse: 6.0, ai_spread: crate::weapons::AiSpread { base: 0.05, move_k: 0.014, move_cap: 0.06, range_from: 8.0, range_k: 0.006 }, damage: crate::weapons::WeaponDefDamage::OneShot, muzzle_ahead: 0.26, rig: crate::weapons::WeaponDefRig::Pistol, handling: None, light: crate::weapons::WeaponDefLight::Rail, drop: crate::slotsetup::SetupGun::Pistol, tracer: None, pellets: None, hands: None, launcher: None, weight_kg: 0.8, bulk: 0.0, own: None }); t },
        }
    }
}

/// (src/game/weapons.ts:124 AI_PISTOL_SPREAD: a table nothing writes)
pub fn ai_pistol_spread() -> crate::weapons::AiSpread { crate::weapons::AiSpread { base: 0.05, move_k: 0.014, move_cap: 0.06, range_from: 8.0, range_k: 0.006 } }

/// (src/game/weapons.ts:126 ROWS: a table nothing writes)
pub fn rows() -> JsMap<String, crate::weapons::WeaponDef> { { let mut t = JsMap::new(); t.set(String::from("fiveseven"), crate::weapons::WeaponDef { id: crate::weapons::WeaponId::Fiveseven, enabled: true, name: String::from("Glock 19 (suppressed)"), short: String::from("Glock 19"), calibre: String::from("9×19 mm"), cls: crate::weapons::WeaponClass::Pistol, mag_capacity: 15.0, spare_mags: 3.0, top_up: true, reload: crate::weapons::WeaponDefReload { dur: 1.6, out: Some(0.38), in_: Some(1.02), rack: Some(1.38), shell: None }, modes: alloc::vec![crate::weapons::FireMode::Semi], cd: 0.22, cd_jitter: 0.0, burst: None, auto: None, report: crate::weapons::WeaponDefReport { level: 0.6, loud: false, voice: String::from("pistolSuppressed") }, suppressed: true, muzzle_fx: crate::effects::MuzzleFxKey::PistolSuppressed, prop_impulse: 4.0, ai_spread: crate::weapons::AiSpread { base: 0.05, move_k: 0.014, move_cap: 0.06, range_from: 8.0, range_k: 0.006 }, damage: crate::weapons::WeaponDefDamage::OneShot, muzzle_ahead: 0.26, rig: crate::weapons::WeaponDefRig::Pistol, handling: None, light: crate::weapons::WeaponDefLight::Rail, drop: crate::slotsetup::SetupGun::Pistol, tracer: None, pellets: None, hands: None, launcher: None, weight_kg: 0.9, bulk: 0.0, own: None }); t.set(String::from("guard9mm"), crate::weapons::WeaponDef { id: crate::weapons::WeaponId::Guard9mm, enabled: true, name: String::from("9 mm service pistol"), short: String::from("sidearm"), calibre: String::from("9×19 mm"), cls: crate::weapons::WeaponClass::Pistol, mag_capacity: 12.0, spare_mags: f64::INFINITY, top_up: false, reload: crate::weapons::WeaponDefReload { dur: 1.7, out: None, in_: None, rack: None, shell: None }, modes: alloc::vec![crate::weapons::FireMode::Semi], cd: 0.55, cd_jitter: 0.5, burst: None, auto: None, report: crate::weapons::WeaponDefReport { level: 1.0, loud: true, voice: String::from("pistolLoud") }, suppressed: false, muzzle_fx: crate::effects::MuzzleFxKey::PistolBare, prop_impulse: 6.0, ai_spread: crate::weapons::AiSpread { base: 0.05, move_k: 0.014, move_cap: 0.06, range_from: 8.0, range_k: 0.006 }, damage: crate::weapons::WeaponDefDamage::OneShot, muzzle_ahead: 0.26, rig: crate::weapons::WeaponDefRig::GuardPistol, handling: None, light: crate::weapons::WeaponDefLight::Rail, drop: crate::slotsetup::SetupGun::Pistol, tracer: None, pellets: None, hands: None, launcher: None, weight_kg: 0.95, bulk: 0.0, own: None }); t.set(String::from("aks74u"), crate::weapons::WeaponDef { id: crate::weapons::WeaponId::Aks74u, enabled: true, name: String::from("AKS-74U carbine"), short: String::from("AKS-74U"), calibre: String::from("5.45×39 mm"), cls: crate::weapons::WeaponClass::Carbine, mag_capacity: 30.0, spare_mags: f64::INFINITY, top_up: false, reload: crate::weapons::WeaponDefReload { dur: 2.6, out: Some(0.7), in_: Some(1.7), rack: Some(2.2), shell: None }, modes: alloc::vec![crate::weapons::FireMode::Auto, crate::weapons::FireMode::Semi], cd: 0.1, cd_jitter: 0.0, burst: Some(crate::weapons::BurstSpec { min: 2.0, max: 4.0, interval: 0.1, gap_min: 0.9, gap_max: 1.6, spread_per_round: 0.03, spread_cap: 0.16 }), auto: None, report: crate::weapons::WeaponDefReport { level: 1.0, loud: true, voice: String::from("carbineReport") }, suppressed: false, muzzle_fx: crate::effects::MuzzleFxKey::Carbine, prop_impulse: 6.0, ai_spread: crate::weapons::AiSpread { base: 0.035, move_k: 0.014, move_cap: 0.06, range_from: 8.0, range_k: 0.006 }, damage: crate::weapons::WeaponDefDamage::OneShot, muzzle_ahead: 0.5, rig: crate::weapons::WeaponDefRig::Carbine, handling: None, light: crate::weapons::WeaponDefLight::Handguard, drop: crate::slotsetup::SetupGun::Rifle, tracer: Some(crate::weapons::TracerSpec { every: 3.0, last_n: 3.0, color: [1.0, 0.42, 0.12] }), pellets: None, hands: None, launcher: None, weight_kg: 3.0, bulk: 2.0, own: None }); t.set(String::from("fn2000"), crate::weapons::WeaponDef { id: crate::weapons::WeaponId::Fn2000, enabled: true, name: String::from("Mk 18 (suppressed)"), short: String::from("Mk 18"), calibre: String::from("5.56×45 mm"), cls: crate::weapons::WeaponClass::Rifle, mag_capacity: 30.0, spare_mags: 2.0, top_up: true, reload: crate::weapons::WeaponDefReload { dur: 2.4, out: Some(0.5), in_: Some(1.5), rack: Some(2.0), shell: None }, modes: alloc::vec![crate::weapons::FireMode::Semi, crate::weapons::FireMode::Auto], cd: 0.2, cd_jitter: 0.0, burst: None, auto: Some(crate::weapons::AutoSpec { interval: 0.1, spread_per_round: 0.012, spread_cap: 0.08, reset_after: 0.25 }), report: crate::weapons::WeaponDefReport { level: 1.0, loud: true, voice: String::from("rifleSuppressed") }, suppressed: true, muzzle_fx: crate::effects::MuzzleFxKey::Sc20k, prop_impulse: 5.0, ai_spread: crate::weapons::AiSpread { base: 0.035, move_k: 0.014, move_cap: 0.06, range_from: 8.0, range_k: 0.006 }, damage: crate::weapons::WeaponDefDamage::OneShot, muzzle_ahead: 0.52, rig: crate::weapons::WeaponDefRig::Sc20k, handling: Some(0.8), light: crate::weapons::WeaponDefLight::Handguard, drop: crate::slotsetup::SetupGun::Rifle, tracer: None, pellets: None, hands: None, launcher: None, weight_kg: 3.9, bulk: 2.0, own: None }); t.set(String::from("m870"), crate::weapons::WeaponDef { id: crate::weapons::WeaponId::M870, enabled: true, name: String::from("M870 breaching shotgun"), short: String::from("M870"), calibre: String::from("12 gauge"), cls: crate::weapons::WeaponClass::Shotgun, mag_capacity: 6.0, spare_mags: 3.0, top_up: true, reload: crate::weapons::WeaponDefReload { dur: 3.2, out: Some(0.4), in_: Some(2.5), rack: Some(2.9), shell: Some(crate::gunreload::ShellRowShell { lead: 0.4, each: 0.35, tail: 0.7, bolt: 0.4 }) }, modes: alloc::vec![crate::weapons::FireMode::Semi], cd: 0.9, cd_jitter: 0.2, burst: None, auto: None, report: crate::weapons::WeaponDefReport { level: 1.0, loud: true, voice: String::from("carbineReport") }, suppressed: false, muzzle_fx: crate::effects::MuzzleFxKey::Carbine, prop_impulse: 2.0, ai_spread: crate::weapons::AiSpread { base: 0.035, move_k: 0.014, move_cap: 0.06, range_from: 8.0, range_k: 0.006 }, damage: crate::weapons::WeaponDefDamage::OneShot, muzzle_ahead: 0.5, rig: crate::weapons::WeaponDefRig::Carbine, handling: None, light: crate::weapons::WeaponDefLight::Handguard, drop: crate::slotsetup::SetupGun::Rifle, tracer: None, pellets: Some(crate::weapons::PelletSpec { n: 9.0, cone: 0.06, drop_m: 8.0, need: 4.0, breach: Some(crate::weapons::PelletSpecBreach { reach_m: 2.0, latch_m: 0.5, need: 4.0 }) }), hands: None, launcher: None, weight_kg: 3.4, bulk: 3.0, own: Some(crate::weapons::WeaponDefOwn { model: crate::weapons::GunModelId::M870, voice: Some(String::from("m870Report")), flame: None }) }); t.set(String::from("mk12"), crate::weapons::WeaponDef { id: crate::weapons::WeaponId::Mk12, enabled: true, name: String::from("Mk 12 SPR (suppressed)"), short: String::from("Mk 12"), calibre: String::from("5.56×45 mm"), cls: crate::weapons::WeaponClass::Rifle, mag_capacity: 20.0, spare_mags: 3.0, top_up: true, reload: crate::weapons::WeaponDefReload { dur: 2.6, out: Some(0.55), in_: Some(1.65), rack: Some(2.2), shell: None }, modes: alloc::vec![crate::weapons::FireMode::Semi], cd: 0.3, cd_jitter: 0.15, burst: None, auto: None, report: crate::weapons::WeaponDefReport { level: 1.0, loud: true, voice: String::from("rifleSuppressed") }, suppressed: true, muzzle_fx: crate::effects::MuzzleFxKey::Sc20k, prop_impulse: 5.0, ai_spread: crate::weapons::AiSpread { base: 0.018, move_k: 0.02, move_cap: 0.08, range_from: 16.0, range_k: 0.003 }, damage: crate::weapons::WeaponDefDamage::OneShot, muzzle_ahead: 0.52, rig: crate::weapons::WeaponDefRig::Sc20k, handling: Some(0.62), light: crate::weapons::WeaponDefLight::Handguard, drop: crate::slotsetup::SetupGun::Rifle, tracer: None, pellets: None, hands: Some(crate::weapons::WeaponDefHands { still: 0.45, moving: 1.4, crouch: None }), launcher: Some(false), weight_kg: 5.0, bulk: 3.0, own: Some(crate::weapons::WeaponDefOwn { model: crate::weapons::GunModelId::Mk12, voice: Some(String::from("mk12Suppressed")), flame: Some(crate::effects::MuzzleFxKey::Mk12) }) }); t.set(String::from("mp7"), crate::weapons::WeaponDef { id: crate::weapons::WeaponId::Mp7, enabled: true, name: String::from("MP7A1 (suppressed)"), short: String::from("MP7"), calibre: String::from("4.6×30 mm"), cls: crate::weapons::WeaponClass::Carbine, mag_capacity: 40.0, spare_mags: 3.0, top_up: true, reload: crate::weapons::WeaponDefReload { dur: 2.0, out: Some(0.4), in_: Some(1.25), rack: Some(1.65), shell: None }, modes: alloc::vec![crate::weapons::FireMode::Auto, crate::weapons::FireMode::Semi], cd: 0.12, cd_jitter: 0.0, burst: Some(crate::weapons::BurstSpec { min: 2.0, max: 4.0, interval: 0.06666666666666667, gap_min: 0.5, gap_max: 0.9, spread_per_round: 0.02, spread_cap: 0.12 }), auto: Some(crate::weapons::AutoSpec { interval: 0.06666666666666667, spread_per_round: 0.008, spread_cap: 0.07, reset_after: 0.25 }), report: crate::weapons::WeaponDefReport { level: 0.8, loud: false, voice: String::from("pistolSuppressed") }, suppressed: true, muzzle_fx: crate::effects::MuzzleFxKey::PistolSuppressed, prop_impulse: 3.0, ai_spread: crate::weapons::AiSpread { base: 0.045, move_k: 0.008, move_cap: 0.03, range_from: 7.0, range_k: 0.012 }, damage: crate::weapons::WeaponDefDamage::OneShot, muzzle_ahead: 0.4, rig: crate::weapons::WeaponDefRig::Sc20k, handling: Some(0.95), light: crate::weapons::WeaponDefLight::Handguard, drop: crate::slotsetup::SetupGun::Rifle, tracer: None, pellets: None, hands: Some(crate::weapons::WeaponDefHands { still: 1.6, moving: 0.7, crouch: None }), launcher: Some(false), weight_kg: 2.3, bulk: 1.0, own: Some(crate::weapons::WeaponDefOwn { model: crate::weapons::GunModelId::Mp7, voice: Some(String::from("mp7Suppressed")), flame: Some(crate::effects::MuzzleFxKey::Mp7) }) }); t.set(String::from("mk46"), crate::weapons::WeaponDef { id: crate::weapons::WeaponId::Mk46, enabled: true, name: String::from("Mk 46 light machine gun"), short: String::from("Mk 46"), calibre: String::from("5.56×45 mm"), cls: crate::weapons::WeaponClass::Rifle, mag_capacity: 100.0, spare_mags: 1.0, top_up: true, reload: crate::weapons::WeaponDefReload { dur: 6.0, out: Some(1.2), in_: Some(4.4), rack: Some(5.4), shell: None }, modes: alloc::vec![crate::weapons::FireMode::Auto], cd: 0.08333333333333333, cd_jitter: 0.0, burst: Some(crate::weapons::BurstSpec { min: 5.0, max: 9.0, interval: 0.08333333333333333, gap_min: 0.5, gap_max: 1.0, spread_per_round: 0.012, spread_cap: 0.14 }), auto: Some(crate::weapons::AutoSpec { interval: 0.08333333333333333, spread_per_round: 0.004, spread_cap: 0.06, reset_after: 0.3 }), report: crate::weapons::WeaponDefReport { level: 1.0, loud: true, voice: String::from("carbineReport") }, suppressed: false, muzzle_fx: crate::effects::MuzzleFxKey::Carbine, prop_impulse: 6.0, ai_spread: crate::weapons::AiSpread { base: 0.05, move_k: 0.03, move_cap: 0.12, range_from: 10.0, range_k: 0.004 }, damage: crate::weapons::WeaponDefDamage::OneShot, muzzle_ahead: 0.52, rig: crate::weapons::WeaponDefRig::Sc20k, handling: Some(0.5), light: crate::weapons::WeaponDefLight::None, drop: crate::slotsetup::SetupGun::Rifle, tracer: Some(crate::weapons::TracerSpec { every: 5.0, last_n: 0.0, color: [1.0, 0.25, 0.12] }), pellets: None, hands: Some(crate::weapons::WeaponDefHands { still: 1.5, moving: 2.4, crouch: Some(0.5) }), launcher: Some(false), weight_kg: 8.0, bulk: 4.0, own: Some(crate::weapons::WeaponDefOwn { model: crate::weapons::GunModelId::Mk46, voice: Some(String::from("mk46Report")), flame: Some(crate::effects::MuzzleFxKey::Mk46) }) }); t.set(String::from("m1014"), crate::weapons::WeaponDef { id: crate::weapons::WeaponId::M1014, enabled: true, name: String::from("M1014 shotgun"), short: String::from("M1014"), calibre: String::from("12 gauge"), cls: crate::weapons::WeaponClass::Shotgun, mag_capacity: 7.0, spare_mags: 3.0, top_up: true, reload: crate::weapons::WeaponDefReload { dur: 3.6, out: Some(0.4), in_: Some(2.9), rack: Some(3.3), shell: Some(crate::gunreload::ShellRowShell { lead: 0.4, each: 0.36, tail: 0.7, bolt: 0.4 }) }, modes: alloc::vec![crate::weapons::FireMode::Semi], cd: 0.28, cd_jitter: 0.1, burst: None, auto: None, report: crate::weapons::WeaponDefReport { level: 1.0, loud: true, voice: String::from("carbineReport") }, suppressed: false, muzzle_fx: crate::effects::MuzzleFxKey::Carbine, prop_impulse: 2.0, ai_spread: crate::weapons::AiSpread { base: 0.035, move_k: 0.014, move_cap: 0.06, range_from: 8.0, range_k: 0.006 }, damage: crate::weapons::WeaponDefDamage::OneShot, muzzle_ahead: 0.5, rig: crate::weapons::WeaponDefRig::Carbine, handling: Some(0.72), light: crate::weapons::WeaponDefLight::Handguard, drop: crate::slotsetup::SetupGun::Rifle, tracer: None, pellets: Some(crate::weapons::PelletSpec { n: 9.0, cone: 0.045, drop_m: 10.0, need: 4.0, breach: None }), hands: None, launcher: Some(false), weight_kg: 3.9, bulk: 3.0, own: Some(crate::weapons::WeaponDefOwn { model: crate::weapons::GunModelId::M1014, voice: Some(String::from("m1014Report")), flame: Some(crate::effects::MuzzleFxKey::M1014) }) }); t.set(String::from("hk45"), crate::weapons::WeaponDef { id: crate::weapons::WeaponId::Hk45, enabled: true, name: String::from("HK45 (suppressed)"), short: String::from("HK45"), calibre: String::from(".45 ACP"), cls: crate::weapons::WeaponClass::Pistol, mag_capacity: 8.0, spare_mags: 3.0, top_up: true, reload: crate::weapons::WeaponDefReload { dur: 1.7, out: Some(0.4), in_: Some(1.1), rack: Some(1.45), shell: None }, modes: alloc::vec![crate::weapons::FireMode::Semi], cd: 0.3, cd_jitter: 0.0, burst: None, auto: None, report: crate::weapons::WeaponDefReport { level: 0.45, loud: false, voice: String::from("pistolSuppressed") }, suppressed: true, muzzle_fx: crate::effects::MuzzleFxKey::PistolSuppressed, prop_impulse: 5.0, ai_spread: crate::weapons::AiSpread { base: 0.05, move_k: 0.014, move_cap: 0.06, range_from: 8.0, range_k: 0.006 }, damage: crate::weapons::WeaponDefDamage::OneShot, muzzle_ahead: 0.26, rig: crate::weapons::WeaponDefRig::Pistol, handling: None, light: crate::weapons::WeaponDefLight::Rail, drop: crate::slotsetup::SetupGun::Pistol, tracer: None, pellets: None, hands: None, launcher: None, weight_kg: 1.2, bulk: 0.0, own: Some(crate::weapons::WeaponDefOwn { model: crate::weapons::GunModelId::Hk45, voice: Some(String::from("hk45Suppressed")), flame: Some(crate::effects::MuzzleFxKey::Hk45) }) }); t.set(String::from("g18"), crate::weapons::WeaponDef { id: crate::weapons::WeaponId::G18, enabled: true, name: String::from("Glock 18"), short: String::from("Glock 18"), calibre: String::from("9×19 mm"), cls: crate::weapons::WeaponClass::Pistol, mag_capacity: 33.0, spare_mags: 2.0, top_up: true, reload: crate::weapons::WeaponDefReload { dur: 1.8, out: Some(0.4), in_: Some(1.15), rack: Some(1.5), shell: None }, modes: alloc::vec![crate::weapons::FireMode::Auto, crate::weapons::FireMode::Semi], cd: 0.2, cd_jitter: 0.0, burst: Some(crate::weapons::BurstSpec { min: 3.0, max: 5.0, interval: 0.05, gap_min: 0.5, gap_max: 0.9, spread_per_round: 0.03, spread_cap: 0.18 }), auto: Some(crate::weapons::AutoSpec { interval: 0.05, spread_per_round: 0.02, spread_cap: 0.16, reset_after: 0.25 }), report: crate::weapons::WeaponDefReport { level: 1.0, loud: true, voice: String::from("pistolLoud") }, suppressed: false, muzzle_fx: crate::effects::MuzzleFxKey::PistolBare, prop_impulse: 6.0, ai_spread: crate::weapons::AiSpread { base: 0.06, move_k: 0.014, move_cap: 0.06, range_from: 8.0, range_k: 0.006 }, damage: crate::weapons::WeaponDefDamage::OneShot, muzzle_ahead: 0.26, rig: crate::weapons::WeaponDefRig::GuardPistol, handling: None, light: crate::weapons::WeaponDefLight::Rail, drop: crate::slotsetup::SetupGun::Pistol, tracer: None, pellets: None, hands: None, launcher: None, weight_kg: 1.0, bulk: 0.0, own: Some(crate::weapons::WeaponDefOwn { model: crate::weapons::GunModelId::G18, voice: Some(String::from("g18Report")), flame: Some(crate::effects::MuzzleFxKey::G18) }) }); t.set(String::from("ak74m"), crate::weapons::WeaponDef { id: crate::weapons::WeaponId::Ak74m, enabled: false, name: String::from("AK-74M rifle"), short: String::from("AK-74M"), calibre: String::from("5.45×39 mm"), cls: crate::weapons::WeaponClass::Rifle, mag_capacity: 30.0, spare_mags: f64::INFINITY, top_up: false, reload: crate::weapons::WeaponDefReload { dur: 2.6, out: Some(0.7), in_: Some(1.7), rack: Some(2.2), shell: None }, modes: alloc::vec![crate::weapons::FireMode::Auto, crate::weapons::FireMode::Semi], cd: 0.1, cd_jitter: 0.0, burst: Some(crate::weapons::BurstSpec { min: 2.0, max: 4.0, interval: 0.1, gap_min: 0.9, gap_max: 1.6, spread_per_round: 0.025, spread_cap: 0.16 }), auto: None, report: crate::weapons::WeaponDefReport { level: 1.0, loud: true, voice: String::from("carbineReport") }, suppressed: false, muzzle_fx: crate::effects::MuzzleFxKey::Carbine, prop_impulse: 6.0, ai_spread: crate::weapons::AiSpread { base: 0.03, move_k: 0.014, move_cap: 0.06, range_from: 8.0, range_k: 0.006 }, damage: crate::weapons::WeaponDefDamage::OneShot, muzzle_ahead: 0.66, rig: crate::weapons::WeaponDefRig::Carbine, handling: None, light: crate::weapons::WeaponDefLight::Handguard, drop: crate::slotsetup::SetupGun::Rifle, tracer: Some(crate::weapons::TracerSpec { every: 3.0, last_n: 3.0, color: [1.0, 0.42, 0.12] }), pellets: None, hands: None, launcher: None, weight_kg: 3.6, bulk: 3.0, own: None }); t.set(String::from("akms"), crate::weapons::WeaponDef { id: crate::weapons::WeaponId::Akms, enabled: true, name: String::from("AKMS rifle"), short: String::from("AKMS"), calibre: String::from("7.62×39 mm"), cls: crate::weapons::WeaponClass::Rifle, mag_capacity: 30.0, spare_mags: f64::INFINITY, top_up: false, reload: crate::weapons::WeaponDefReload { dur: 2.6, out: Some(0.7), in_: Some(1.7), rack: Some(2.2), shell: None }, modes: alloc::vec![crate::weapons::FireMode::Auto, crate::weapons::FireMode::Semi], cd: 0.1, cd_jitter: 0.0, burst: Some(crate::weapons::BurstSpec { min: 2.0, max: 3.0, interval: 0.1, gap_min: 0.9, gap_max: 1.6, spread_per_round: 0.04, spread_cap: 0.16 }), auto: None, report: crate::weapons::WeaponDefReport { level: 1.0, loud: true, voice: String::from("carbineReport") }, suppressed: false, muzzle_fx: crate::effects::MuzzleFxKey::PistolBare, prop_impulse: 6.0, ai_spread: crate::weapons::AiSpread { base: 0.04, move_k: 0.014, move_cap: 0.06, range_from: 8.0, range_k: 0.006 }, damage: crate::weapons::WeaponDefDamage::OneShot, muzzle_ahead: 0.62, rig: crate::weapons::WeaponDefRig::Carbine, handling: None, light: crate::weapons::WeaponDefLight::None, drop: crate::slotsetup::SetupGun::Rifle, tracer: None, pellets: None, hands: None, launcher: None, weight_kg: 3.5, bulk: 3.0, own: None }); t.set(String::from("pkm"), crate::weapons::WeaponDef { id: crate::weapons::WeaponId::Pkm, enabled: true, name: String::from("PKM machine gun"), short: String::from("PKM"), calibre: String::from("7.62×54 mm R"), cls: crate::weapons::WeaponClass::Rifle, mag_capacity: 100.0, spare_mags: f64::INFINITY, top_up: false, reload: crate::weapons::WeaponDefReload { dur: 6.5, out: Some(1.4), in_: Some(5.0), rack: Some(5.9), shell: None }, modes: alloc::vec![crate::weapons::FireMode::Auto], cd: 0.1, cd_jitter: 0.0, burst: Some(crate::weapons::BurstSpec { min: 6.0, max: 10.0, interval: 0.1, gap_min: 0.5, gap_max: 1.0, spread_per_round: 0.012, spread_cap: 0.14 }), auto: None, report: crate::weapons::WeaponDefReport { level: 1.0, loud: true, voice: String::from("carbineReport") }, suppressed: false, muzzle_fx: crate::effects::MuzzleFxKey::Carbine, prop_impulse: 6.0, ai_spread: crate::weapons::AiSpread { base: 0.05, move_k: 0.03, move_cap: 0.12, range_from: 10.0, range_k: 0.004 }, damage: crate::weapons::WeaponDefDamage::OneShot, muzzle_ahead: 0.62, rig: crate::weapons::WeaponDefRig::Carbine, handling: Some(0.45), light: crate::weapons::WeaponDefLight::None, drop: crate::slotsetup::SetupGun::Rifle, tracer: Some(crate::weapons::TracerSpec { every: 5.0, last_n: 0.0, color: [1.0, 0.42, 0.12] }), pellets: None, hands: None, launcher: None, weight_kg: 7.5, bulk: 4.0, own: None }); t } }

/// (src/game/weapons.ts:342 HANDLING_BY_CLASS: a table nothing writes)
pub fn handling_by_class() -> JsMap<String, f64> { { let mut t = JsMap::new(); t.set(String::from("pistol"), 1.0); t.set(String::from("carbine"), 0.82); t.set(String::from("rifle"), 0.7); t.set(String::from("shotgun"), 0.75); t } }
