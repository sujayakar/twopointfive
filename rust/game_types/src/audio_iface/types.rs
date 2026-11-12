//! The types of `audio_iface`.
#![allow(unused_imports)]
use crate::{Words, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// A SOUND THAT GOES ON in a place, as the mixer holds it for as long as it does
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Words)]
pub enum LastingSfx { ChairRoll, FireBed, SmokeHiss, Hum, Servers, Buzz, Wind }
impl From<crate::scene_level::HumKind> for LastingSfx {
    fn from(k: crate::scene_level::HumKind) -> LastingSfx {
        use crate::scene_level::HumKind as K;
        match k { K::Hum => LastingSfx::Hum, K::Servers => LastingSfx::Servers, K::Buzz => LastingSfx::Buzz, K::Wind => LastingSfx::Wind }
    }
}

/// A SOUND THE GAME OR ITS MIXER PLAYS: every name there is, once. It crosses by its number; the page has the words in this order (`SFX`, `SfxName` in src/draw/names.ts, written by `half`) and a voice for each (src/audio/sfx.ts).
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, ToBytes, Words)]
pub enum Sfx {
    PistolSuppressed,
    PistolLoud,
    CarbineReport,
    RifleSuppressed,
    Launcher,
    DryFire,
    MagOut,
    MagIn,
    Rack,
    MagDrop,
    FootSoft,
    FootHard,
    Ocp,
    NvOn,
    NvOff,
    Click,
    Throw,
    CanisterBounce,
    BulletImpact,
    BodyHit,
    BodyFall,
    LightBreak,
    LightOff,
    Deny,
    Equip,
    SmokePop,
    AlertSting,
    Radio,
    DoorCreak,
    DoorBang,
    DoorLatch,
    DoorBash,
    DoorRattle,
    LockPick,
    LockOpen,
    LockBreak,
    PowerDown,
    PowerUp,
    StunBang,
    EarRing,
    PropScrape,
    PropThud,
    GrabScuffle,
    ChokeStrain,
    ChokeLimp,
    WhipThud,
    Shove,
    Murmur,
    HeldProtest,
    Whistle,
    StairDoor,
    StairBoots,
    CasingTink,
    ThunderNear,
    ThunderFar,
    GlassCrack,
    GlassTinkle,
    GlassShatter,
    GlassCrunch,
    /// a keypad key (rust/game_rules/src/stealth/devices.rs)
    PadBeep,
    /// a keypad accepting the code (rust/game_rules/src/stealth/devices.rs)
    PadOpen,
    /// a tripwire's emitter chirping as it reports (rust/game_rules/src/stealth/devices.rs)
    AlarmChirp,
    /// a wall charge going off (rust/game_rules/src/breakage/blast_feel): the crack
    WallCrack,
    /// …the thump
    WallThump,
    /// …the wall coming down
    WallDebris,
    /// the player's back landing on a wall (rust/game_rules/src/sam/player/wall.rs `wall_voice`)
    WallOn,
    /// …and his stepping off it
    WallOff,
    /// the grab as three beats: the catch
    GrabCatch,
    /// …the gun twisted out of his hand
    GunStrip,
    /// …the gun let down onto the floor
    GunDown,
    /// the guns a card's two slots offer, each its own report (rust/game_rules/src/outfit/gunlook.rs), and the pump gun's
    Mk12Suppressed,
    Mp7Suppressed,
    Mk46Report,
    M1014Report,
    M870Report,
    Hk45Suppressed,
    G18Report,
    /// how they are fed, a belt under a feed cover and shells one at a time (rust/game_rules/src/guns/gunreload.rs)
    BeltCoverUp,
    BeltBoxOff,
    BeltBoxOn,
    BeltLay,
    BeltCoverDown,
    ChargeLmg,
    ShellIn,
    BoltDrop,
    /// a chair coasting on its casters (rust/game_rules/src/fittings/props.rs CASTOR): looped between `ROLL_LOOP`'s two marks (src/audio/sfx.ts) for as long as it rolls
    ChairRoll,
    /// an open fire (rust/game_rules/src/air/fire.rs): its low bed, looped between `FIRE_LOOP`'s two marks (src/audio/sfx.ts) for as long as it burns
    FireBed,
    /// …and one crack of the wood
    FireCrack,
}

/// what a call of GameAudioLike.play asks beyond its name, place and gain, by the shapes the calls have. None: no opts, and the mixer draws its own rate
#[derive(Clone, Debug)]
pub enum PlayOpts {
    Rate { rate: f64 },
    Thunder { lowpass: f64, rate: f64, pan: f64, wet: f64 },
}

