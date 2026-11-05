//! The types of `rig_props`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: Seg, src/game/rigProps.ts:21)
#[derive(Clone, Copy, Debug)]
pub struct Seg {
    pub l0: f64,
    pub l1: f64,
    pub thick: f64,
    pub tall: f64,
    pub adj: Option<crate::rig_props::Adj>,
}
impl Seg {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Seg {
            l0: 0.0,
            l1: 0.0,
            thick: 0.0,
            tall: 0.0,
            adj: None,
        }
    }
}
impl Default for Seg { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Adj, src/game/rigProps.ts:26)
#[derive(Clone, Copy, Debug)]
pub struct Adj {
    pub off: V3,
    pub rot: V3,
}
impl Adj {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Adj {
            off: [0.0; 3],
            rot: [0.0; 3],
        }
    }
}
impl Default for Adj { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Rig, src/game/rigProps.ts:28)
#[derive(Clone, Copy, Debug)]
pub struct Rig {
    pub goggles: crate::rig_props::RigGoggles,
    pub pistol: crate::rig_props::RigPistol,
    pub torch: crate::rig_props::RigTorch,
    pub guard_pistol: crate::rig_props::RigGuardPistol,
    pub knife: crate::rig_props::RigKnife,
    pub sheath: crate::rig_props::RigSheath,
    pub can: crate::rig_props::RigCan,
    pub holster: crate::rig_props::RigHolster,
    pub carbine: crate::rig_props::RigCarbine,
    pub sc20k: crate::rig_props::RigSc20k,
    pub m4: crate::rig_props::RigM4,
}
impl Rig {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Rig {
            goggles: crate::rig_props::RigGoggles::zeroed(),
            pistol: crate::rig_props::RigPistol::zeroed(),
            torch: crate::rig_props::RigTorch::zeroed(),
            guard_pistol: crate::rig_props::RigGuardPistol::zeroed(),
            knife: crate::rig_props::RigKnife::zeroed(),
            sheath: crate::rig_props::RigSheath::zeroed(),
            can: crate::rig_props::RigCan::zeroed(),
            holster: crate::rig_props::RigHolster::zeroed(),
            carbine: crate::rig_props::RigCarbine::zeroed(),
            sc20k: crate::rig_props::RigSc20k::zeroed(),
            m4: crate::rig_props::RigM4::zeroed(),
        }
    }
}
impl Default for Rig { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/rigProps.ts:29 Rig.goggles, src/game/rigProps.ts:29)
#[derive(Clone, Copy, Debug)]
pub struct RigGoggles {
    pub on: crate::rig_props::RigGogglesOn,
    pub off: crate::rig_props::RigGogglesOff,
    pub lens_side: f64,
    pub lens_down: f64,
    pub lens_up: f64,
    pub lens_half: f64,
    pub lens_thick: f64,
    pub housing: crate::rig_props::RigGogglesHousing,
    pub glow_on: V3,
    pub glow_off: V3,
    pub tubes: crate::rig_props::RigGogglesTubes,
    pub on_adj: Option<crate::rig_props::Adj>,
    pub off_adj: Option<crate::rig_props::Adj>,
    pub housing_adj: Option<crate::rig_props::Adj>,
    pub lens_adj: Option<crate::rig_props::Adj>,
    pub follow_pitch: f64,
}
impl RigGoggles {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RigGoggles {
            on: crate::rig_props::RigGogglesOn::zeroed(),
            off: crate::rig_props::RigGogglesOff::zeroed(),
            lens_side: 0.0,
            lens_down: 0.0,
            lens_up: 0.0,
            lens_half: 0.0,
            lens_thick: 0.0,
            housing: crate::rig_props::RigGogglesHousing::zeroed(),
            glow_on: [0.0; 3],
            glow_off: [0.0; 3],
            tubes: crate::rig_props::RigGogglesTubes::zeroed(),
            on_adj: None,
            off_adj: None,
            housing_adj: None,
            lens_adj: None,
            follow_pitch: 0.0,
        }
    }
}
impl Default for RigGoggles { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/rigProps.ts:32 RigGoggles.on, src/game/rigProps.ts:32)
#[derive(Clone, Copy, Debug)]
pub struct RigGogglesOn {
    pub fwd: f64,
    pub up: f64,
}
impl RigGogglesOn {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RigGogglesOn {
            fwd: 0.0,
            up: 0.0,
        }
    }
}
impl Default for RigGogglesOn { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/rigProps.ts:34 RigGoggles.off, src/game/rigProps.ts:34)
#[derive(Clone, Copy, Debug)]
pub struct RigGogglesOff {
    pub fwd: f64,
    pub up: f64,
}
impl RigGogglesOff {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RigGogglesOff {
            fwd: 0.0,
            up: 0.0,
        }
    }
}
impl Default for RigGogglesOff { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/rigProps.ts:40 RigGoggles.housing, src/game/rigProps.ts:40)
#[derive(Clone, Copy, Debug)]
pub struct RigGogglesHousing {
    pub back: f64,
    pub half: V3,
}
impl RigGogglesHousing {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RigGogglesHousing {
            back: 0.0,
            half: [0.0; 3],
        }
    }
}
impl Default for RigGogglesHousing { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/rigProps.ts:47 RigGoggles.tubes, src/game/rigProps.ts:47)
#[derive(Clone, Copy, Debug)]
pub struct RigGogglesTubes {
    pub side: f64,
    pub down: f64,
    pub half: f64,
    pub back: f64,
    pub front: f64,
    pub glass: f64,
    pub bridge: crate::rig_props::RigGogglesTubesBridge,
    pub arm: crate::rig_props::RigGogglesTubesArm,
}
impl RigGogglesTubes {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RigGogglesTubes {
            side: 0.0,
            down: 0.0,
            half: 0.0,
            back: 0.0,
            front: 0.0,
            glass: 0.0,
            bridge: crate::rig_props::RigGogglesTubesBridge::zeroed(),
            arm: crate::rig_props::RigGogglesTubesArm::zeroed(),
        }
    }
}
impl Default for RigGogglesTubes { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/rigProps.ts:47 RigGogglesTubes.bridge, src/game/rigProps.ts:47)
#[derive(Clone, Copy, Debug)]
pub struct RigGogglesTubesBridge {
    pub at: V3,
    pub half: V3,
}
impl RigGogglesTubesBridge {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RigGogglesTubesBridge {
            at: [0.0; 3],
            half: [0.0; 3],
        }
    }
}
impl Default for RigGogglesTubesBridge { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/rigProps.ts:47 RigGogglesTubes.arm, src/game/rigProps.ts:47)
#[derive(Clone, Copy, Debug)]
pub struct RigGogglesTubesArm {
    pub at: V3,
    pub half: V3,
}
impl RigGogglesTubesArm {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RigGogglesTubesArm {
            at: [0.0; 3],
            half: [0.0; 3],
        }
    }
}
impl Default for RigGogglesTubesArm { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/rigProps.ts:56 Rig.pistol, src/game/rigProps.ts:56)
#[derive(Clone, Copy, Debug)]
pub struct RigPistol {
    pub slide: crate::rig_props::Seg,
    pub suppressor: crate::rig_props::Seg,
    pub grip: crate::rig_props::Seg,
    pub rail: crate::rig_props::RigPistolRail,
    pub lens: crate::rig_props::RigPistolLens,
    pub light_under: f64,
    pub light_ahead: f64,
    pub adj: Option<crate::rig_props::Adj>,
}
impl RigPistol {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RigPistol {
            slide: crate::rig_props::Seg::zeroed(),
            suppressor: crate::rig_props::Seg::zeroed(),
            grip: crate::rig_props::Seg::zeroed(),
            rail: crate::rig_props::RigPistolRail::zeroed(),
            lens: crate::rig_props::RigPistolLens::zeroed(),
            light_under: 0.0,
            light_ahead: 0.0,
            adj: None,
        }
    }
}
impl Default for RigPistol { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/rigProps.ts:61 RigPistol.rail, src/game/rigProps.ts:61 RigPistol.rail)
#[derive(Clone, Copy, Debug)]
pub struct RigPistolRail {
    pub l0: f64,
    pub l1: f64,
    pub thick: f64,
    pub tall: f64,
    pub adj: Option<crate::rig_props::Adj>,
    pub under: f64,
}
impl RigPistolRail {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RigPistolRail {
            l0: 0.0,
            l1: 0.0,
            thick: 0.0,
            tall: 0.0,
            adj: None,
            under: 0.0,
        }
    }
}
impl Default for RigPistolRail { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/rigProps.ts:63 RigPistol.lens, src/game/rigProps.ts:63 RigPistol.lens)
#[derive(Clone, Copy, Debug)]
pub struct RigPistolLens {
    pub l0: f64,
    pub l1: f64,
    pub thick: f64,
    pub tall: f64,
    pub adj: Option<crate::rig_props::Adj>,
    pub glow: V3,
}
impl RigPistolLens {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RigPistolLens {
            l0: 0.0,
            l1: 0.0,
            thick: 0.0,
            tall: 0.0,
            adj: None,
            glow: [0.0; 3],
        }
    }
}
impl Default for RigPistolLens { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/rigProps.ts:69 Rig.torch, src/game/rigProps.ts:69)
#[derive(Clone, Copy, Debug)]
pub struct RigTorch {
    pub body: crate::rig_props::Seg,
    pub lens: crate::rig_props::Seg,
    pub lens_glow: f64,
    pub light_ahead: f64,
    pub adj: Option<crate::rig_props::Adj>,
}
impl RigTorch {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RigTorch {
            body: crate::rig_props::Seg::zeroed(),
            lens: crate::rig_props::Seg::zeroed(),
            lens_glow: 0.0,
            light_ahead: 0.0,
            adj: None,
        }
    }
}
impl Default for RigTorch { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/rigProps.ts:71 Rig.guardPistol, src/game/rigProps.ts:71)
#[derive(Clone, Copy, Debug)]
pub struct RigGuardPistol {
    pub slide: crate::rig_props::Seg,
    pub grip: crate::rig_props::Seg,
    pub lens: crate::rig_props::Seg,
    pub light_ahead: f64,
    pub adj: Option<crate::rig_props::Adj>,
}
impl RigGuardPistol {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RigGuardPistol {
            slide: crate::rig_props::Seg::zeroed(),
            grip: crate::rig_props::Seg::zeroed(),
            lens: crate::rig_props::Seg::zeroed(),
            light_ahead: 0.0,
            adj: None,
        }
    }
}
impl Default for RigGuardPistol { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/rigProps.ts:76 Rig.knife, src/game/rigProps.ts:76)
#[derive(Clone, Copy, Debug)]
pub struct RigKnife {
    pub blade: crate::rig_props::Seg,
    pub edge: crate::rig_props::Seg,
    pub guard: crate::rig_props::Seg,
    pub handle: crate::rig_props::Seg,
    pub grip: crate::rig_props::RigKnifeGrip,
    pub adj: Option<crate::rig_props::Adj>,
}
impl RigKnife {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RigKnife {
            blade: crate::rig_props::Seg::zeroed(),
            edge: crate::rig_props::Seg::zeroed(),
            guard: crate::rig_props::Seg::zeroed(),
            handle: crate::rig_props::Seg::zeroed(),
            grip: crate::rig_props::RigKnifeGrip::zeroed(),
            adj: None,
        }
    }
}
impl Default for RigKnife { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/rigProps.ts:76 RigKnife.grip, src/game/rigProps.ts:76)
#[derive(Clone, Copy, Debug)]
pub struct RigKnifeGrip {
    pub fwd: f64,
    pub palm: f64,
    pub side: f64,
}
impl RigKnifeGrip {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RigKnifeGrip {
            fwd: 0.0,
            palm: 0.0,
            side: 0.0,
        }
    }
}
impl Default for RigKnifeGrip { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/rigProps.ts:82 Rig.sheath, src/game/rigProps.ts:82)
#[derive(Clone, Copy, Debug)]
pub struct RigSheath {
    pub at: V3,
    pub along: V3,
    pub out: V3,
    pub body: crate::rig_props::Seg,
}
impl RigSheath {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RigSheath {
            at: [0.0; 3],
            along: [0.0; 3],
            out: [0.0; 3],
            body: crate::rig_props::Seg::zeroed(),
        }
    }
}
impl Default for RigSheath { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/rigProps.ts:85 Rig.can, src/game/rigProps.ts:85)
#[derive(Clone, Copy, Debug)]
pub struct RigCan {
    pub sink: f64,
    pub tilt: f64,
    pub body: crate::rig_props::Seg,
    pub band: crate::rig_props::Seg,
    pub adj: Option<crate::rig_props::Adj>,
}
impl RigCan {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RigCan {
            sink: 0.0,
            tilt: 0.0,
            body: crate::rig_props::Seg::zeroed(),
            band: crate::rig_props::Seg::zeroed(),
            adj: None,
        }
    }
}
impl Default for RigCan { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/rigProps.ts:87 Rig.holster, src/game/rigProps.ts:87)
#[derive(Clone, Copy, Debug)]
pub struct RigHolster {
    pub down: f64,
    pub out: f64,
    pub fwd: f64,
    pub adj: Option<crate::rig_props::Adj>,
}
impl RigHolster {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RigHolster {
            down: 0.0,
            out: 0.0,
            fwd: 0.0,
            adj: None,
        }
    }
}
impl Default for RigHolster { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/rigProps.ts:99 Rig.carbine, src/game/rigProps.ts:99)
#[derive(Clone, Copy, Debug)]
pub struct RigCarbine {
    pub receiver: crate::rig_props::Seg,
    pub handguard: crate::rig_props::Seg,
    pub barrel: crate::rig_props::Seg,
    pub muzzle: crate::rig_props::Seg,
    pub mag: crate::rig_props::Seg,
    pub mag2: crate::rig_props::Seg,
    pub grip: crate::rig_props::Seg,
    pub stock_top: crate::rig_props::Seg,
    pub stock_low: crate::rig_props::Seg,
    pub light: crate::rig_props::Seg,
    pub lens: crate::rig_props::Seg,
    pub lens_glow: f64,
    pub light_ahead: f64,
    pub grip_l: crate::rig_props::RigCarbineGripL,
    pub adj: Option<crate::rig_props::Adj>,
}
impl RigCarbine {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RigCarbine {
            receiver: crate::rig_props::Seg::zeroed(),
            handguard: crate::rig_props::Seg::zeroed(),
            barrel: crate::rig_props::Seg::zeroed(),
            muzzle: crate::rig_props::Seg::zeroed(),
            mag: crate::rig_props::Seg::zeroed(),
            mag2: crate::rig_props::Seg::zeroed(),
            grip: crate::rig_props::Seg::zeroed(),
            stock_top: crate::rig_props::Seg::zeroed(),
            stock_low: crate::rig_props::Seg::zeroed(),
            light: crate::rig_props::Seg::zeroed(),
            lens: crate::rig_props::Seg::zeroed(),
            lens_glow: 0.0,
            light_ahead: 0.0,
            grip_l: crate::rig_props::RigCarbineGripL::zeroed(),
            adj: None,
        }
    }
}
impl Default for RigCarbine { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/rigProps.ts:103 RigCarbine.gripL, src/game/rigProps.ts:103)
#[derive(Clone, Copy, Debug)]
pub struct RigCarbineGripL {
    pub off: V3,
    pub fingers: V3,
    pub back: V3,
}
impl RigCarbineGripL {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RigCarbineGripL {
            off: [0.0; 3],
            fingers: [0.0; 3],
            back: [0.0; 3],
        }
    }
}
impl Default for RigCarbineGripL { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/rigProps.ts:120 Rig.sc20k, src/game/rigProps.ts:120)
#[derive(Clone, Copy, Debug)]
pub struct RigSc20k {
    pub hull: crate::rig_props::Seg,
    pub butt: crate::rig_props::Seg,
    pub mag: crate::rig_props::Seg,
    pub grip: crate::rig_props::Seg,
    pub fore: crate::rig_props::Seg,
    pub suppressor: crate::rig_props::Seg,
    pub rail: crate::rig_props::Seg,
    pub optic: crate::rig_props::Seg,
    pub foregrip: crate::rig_props::Seg,
    pub light: crate::rig_props::Seg,
    pub lens: crate::rig_props::Seg,
    pub lens_glow: f64,
    pub light_ahead: f64,
    pub grip_l: crate::rig_props::RigSc20kGripL,
    pub slung: crate::rig_props::RigSc20kSlung,
    pub adj: Option<crate::rig_props::Adj>,
}
impl RigSc20k {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RigSc20k {
            hull: crate::rig_props::Seg::zeroed(),
            butt: crate::rig_props::Seg::zeroed(),
            mag: crate::rig_props::Seg::zeroed(),
            grip: crate::rig_props::Seg::zeroed(),
            fore: crate::rig_props::Seg::zeroed(),
            suppressor: crate::rig_props::Seg::zeroed(),
            rail: crate::rig_props::Seg::zeroed(),
            optic: crate::rig_props::Seg::zeroed(),
            foregrip: crate::rig_props::Seg::zeroed(),
            light: crate::rig_props::Seg::zeroed(),
            lens: crate::rig_props::Seg::zeroed(),
            lens_glow: 0.0,
            light_ahead: 0.0,
            grip_l: crate::rig_props::RigSc20kGripL::zeroed(),
            slung: crate::rig_props::RigSc20kSlung::zeroed(),
            adj: None,
        }
    }
}
impl Default for RigSc20k { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/rigProps.ts:123 RigSc20k.gripL, src/game/rigProps.ts:123)
#[derive(Clone, Copy, Debug)]
pub struct RigSc20kGripL {
    pub off: V3,
    pub fingers: V3,
    pub back: V3,
}
impl RigSc20kGripL {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RigSc20kGripL {
            off: [0.0; 3],
            fingers: [0.0; 3],
            back: [0.0; 3],
        }
    }
}
impl Default for RigSc20kGripL { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/rigProps.ts:124 RigSc20k.slung, src/game/rigProps.ts:124)
#[derive(Clone, Copy, Debug)]
pub struct RigSc20kSlung {
    pub off: V3,
    pub bore: V3,
    pub roll: f64,
    pub follow: f64,
    pub adj: Option<crate::rig_props::Adj>,
}
impl RigSc20kSlung {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RigSc20kSlung {
            off: [0.0; 3],
            bore: [0.0; 3],
            roll: 0.0,
            follow: 0.0,
            adj: None,
        }
    }
}
impl Default for RigSc20kSlung { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/rigProps.ts:132 Rig.m4, src/game/rigProps.ts:132)
#[derive(Clone, Copy, Debug)]
pub struct RigM4 {
    pub hull: crate::rig_props::Seg,
    pub butt: crate::rig_props::Seg,
    pub mag: crate::rig_props::Seg,
    pub fore: crate::rig_props::Seg,
    pub rail: crate::rig_props::Seg,
    pub optic: crate::rig_props::Seg,
    pub tube: crate::rig_props::Seg,
    pub peq: crate::rig_props::Seg,
    pub gl: crate::rig_props::Seg,
}
impl RigM4 {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RigM4 {
            hull: crate::rig_props::Seg::zeroed(),
            butt: crate::rig_props::Seg::zeroed(),
            mag: crate::rig_props::Seg::zeroed(),
            fore: crate::rig_props::Seg::zeroed(),
            rail: crate::rig_props::Seg::zeroed(),
            optic: crate::rig_props::Seg::zeroed(),
            tube: crate::rig_props::Seg::zeroed(),
            peq: crate::rig_props::Seg::zeroed(),
            gl: crate::rig_props::Seg::zeroed(),
        }
    }
}
impl Default for RigM4 { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: a union)
#[derive(Clone, Debug)]
pub enum TreeValue {
    Num(f64),
    Array(Vec<f64>),
    Arm2(JsMap<String, crate::rig_props::TreeValue>),
}

/// (TypeScript: Frame, src/game/rigProps.ts:303)
#[derive(Clone, Copy, Debug)]
pub struct Frame {
    pub o: V3,
    pub x: V3,
    pub y: V3,
    pub z: V3,
}
impl Frame {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Frame {
            o: [0.0; 3],
            x: [0.0; 3],
            y: [0.0; 3],
            z: [0.0; 3],
        }
    }
}
impl Default for Frame { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: RigNodeMeta, src/game/rigProps.ts:326)
#[derive(Clone, Debug)]
pub struct RigNodeMeta {
    pub path: String,
    pub label: String,
    pub depth: f64,
    pub parent: crate::rig_props::Frame,
    pub frame: crate::rig_props::Frame,
    pub box_start: f64,
    pub box_count: f64,
}
impl RigNodeMeta {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RigNodeMeta {
            path: String::new(),
            label: String::new(),
            depth: 0.0,
            parent: crate::rig_props::Frame::zeroed(),
            frame: crate::rig_props::Frame::zeroed(),
            box_start: 0.0,
            box_count: 0.0,
        }
    }
}
impl Default for RigNodeMeta { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: RigOpts, src/game/rigProps.ts:453)
#[derive(Clone, Copy, Debug)]
pub struct RigOpts {
    pub goggles: bool,
    pub nv: bool,
    pub nv_k: Option<f64>,
    pub slot: f64,
    pub light_on: bool,
    pub is_guard: bool,
    pub torch: Option<bool>,
    pub weapon: Option<crate::rig_props::RigOptsWeapon>,
    pub beam_dir: Option<V3>,
    pub lens_color: Option<V3>,
    pub holster: Option<bool>,
    pub held: Option<crate::game::ThrowKind>,
    pub knife: Option<bool>,
    pub sheath: Option<bool>,
    pub rifle: Option<crate::rig_props::RigOptsRifle>,
    pub sling_front: Option<bool>,
    pub sling_rest: Option<f64>,
    pub rifle_light: Option<bool>,
    pub fit: Option<bool>,
    pub in_hand: Option<bool>,
    pub holster_out: Option<f64>,
    pub kit: Option<crate::rig_props::KitDraw>,
    pub rifle_black: Option<bool>,
    pub big: Option<crate::rig_props::RigOptsBig>,
    pub big_k: Option<f64>,
    pub big_clear: Option<bool>,
    pub hang: Option<bool>,
}
impl RigOpts {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RigOpts {
            goggles: false,
            nv: false,
            nv_k: None,
            slot: 0.0,
            light_on: false,
            is_guard: false,
            torch: None,
            weapon: None,
            beam_dir: None,
            lens_color: None,
            holster: None,
            held: None,
            knife: None,
            sheath: None,
            rifle: None,
            sling_front: None,
            sling_rest: None,
            rifle_light: None,
            fit: None,
            in_hand: None,
            holster_out: None,
            kit: None,
            rifle_black: None,
            big: None,
            big_k: None,
            big_clear: None,
            hang: None,
        }
    }
}
impl Default for RigOpts { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum RigOptsWeapon {
    Pistol,
    Carbine,
    Sc20k,
}
impl RigOptsWeapon {
    pub const fn as_str(self) -> &'static str {
        match self {
            RigOptsWeapon::Pistol => "pistol",
            RigOptsWeapon::Carbine => "carbine",
            RigOptsWeapon::Sc20k => "sc20k",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum RigOptsRifle {
    Hand,
    Slung,
}
impl RigOptsRifle {
    pub const fn as_str(self) -> &'static str {
        match self {
            RigOptsRifle::Hand => "hand",
            RigOptsRifle::Slung => "slung",
        }
    }
}

/// (TypeScript: KitDraw, src/game/rigProps.ts:520)
#[derive(Clone, Copy, Debug)]
pub struct KitDraw {
    pub pistol: Option<f64>,
    pub rifle: Option<f64>,
    pub roll: Option<f64>,
    pub front: Option<bool>,
    pub rest: Option<f64>,
    pub no_mag: Option<bool>,
    pub mag_l: Option<crate::slotsetup::SetupGun>,
}
impl KitDraw {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        KitDraw {
            pistol: None,
            rifle: None,
            roll: None,
            front: None,
            rest: None,
            no_mag: None,
            mag_l: None,
        }
    }
}
impl Default for KitDraw { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum RigOptsBig {
    Shotgun,
    Hammer,
}
impl RigOptsBig {
    pub const fn as_str(self) -> &'static str {
        match self {
            RigOptsBig::Shotgun => "shotgun",
            RigOptsBig::Hammer => "hammer",
        }
    }
}

/// (TypeScript: inline at src/game/rigProps.ts:245 COL, src/game/rigProps.ts:245)
#[derive(Clone, Copy, Debug)]
pub struct COL {
    pub gun: V3,
    pub can: V3,
    pub torch: V3,
    pub lens_body: V3,
    pub goggles_lens: V3,
    pub goggles_housing: V3,
    pub blade: V3,
    pub edge: V3,
    pub knife_grip: V3,
    pub sheath: V3,
    pub furniture: V3,
    pub anodised: V3,
    pub fde: V3,
    pub dark_glass: V3,
    pub haft: V3,
    pub sledge: V3,
    pub brass: V3,
    pub shell: V3,
    pub cloth: V3,
    pub polymer: V3,
    pub dark_metal: V3,
}
impl COL {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        COL {
            gun: [0.0; 3],
            can: [0.0; 3],
            torch: [0.0; 3],
            lens_body: [0.0; 3],
            goggles_lens: [0.0; 3],
            goggles_housing: [0.0; 3],
            blade: [0.0; 3],
            edge: [0.0; 3],
            knife_grip: [0.0; 3],
            sheath: [0.0; 3],
            furniture: [0.0; 3],
            anodised: [0.0; 3],
            fde: [0.0; 3],
            dark_glass: [0.0; 3],
            haft: [0.0; 3],
            sledge: [0.0; 3],
            brass: [0.0; 3],
            shell: [0.0; 3],
            cloth: [0.0; 3],
            polymer: [0.0; 3],
            dark_metal: [0.0; 3],
        }
    }
}
impl Default for COL { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/rigProps.ts:429 GRIPFIT, src/game/rigProps.ts:429)
#[derive(Clone, Copy, Debug)]
pub struct GRIPFIT {
    pub can: crate::rig_props::GRIPFITCan,
    pub pistol: V3,
    pub long: V3,
    pub fore: crate::rig_props::GRIPFITFore,
    pub mag: crate::rig_props::GRIPFITMag,
    pub guard_pistol: V3,
    pub torch: crate::rig_props::GRIPFITTorch,
}
impl GRIPFIT {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GRIPFIT {
            can: crate::rig_props::GRIPFITCan::zeroed(),
            pistol: [0.0; 3],
            long: [0.0; 3],
            fore: crate::rig_props::GRIPFITFore::zeroed(),
            mag: crate::rig_props::GRIPFITMag::zeroed(),
            guard_pistol: [0.0; 3],
            torch: crate::rig_props::GRIPFITTorch::zeroed(),
        }
    }
}
impl Default for GRIPFIT { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/rigProps.ts:430 GRIPFIT.can, src/game/rigProps.ts:430)
#[derive(Clone, Copy, Debug)]
pub struct GRIPFITCan {
    pub fwd: f64,
    pub palm: f64,
    pub side: f64,
}
impl GRIPFITCan {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GRIPFITCan {
            fwd: 0.0,
            palm: 0.0,
            side: 0.0,
        }
    }
}
impl Default for GRIPFITCan { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/rigProps.ts:433 GRIPFIT.fore, src/game/rigProps.ts:433)
#[derive(Clone, Copy, Debug)]
pub struct GRIPFITFore {
    pub back: f64,
    pub lift: f64,
    pub down: f64,
}
impl GRIPFITFore {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GRIPFITFore {
            back: 0.0,
            lift: 0.0,
            down: 0.0,
        }
    }
}
impl Default for GRIPFITFore { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/rigProps.ts:434 GRIPFIT.mag, src/game/rigProps.ts:434)
#[derive(Clone, Copy, Debug)]
pub struct GRIPFITMag {
    pub fwd: f64,
    pub palm: f64,
    pub side: f64,
}
impl GRIPFITMag {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GRIPFITMag {
            fwd: 0.0,
            palm: 0.0,
            side: 0.0,
        }
    }
}
impl Default for GRIPFITMag { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/rigProps.ts:439 GRIPFIT.torch, src/game/rigProps.ts:439)
#[derive(Clone, Copy, Debug)]
pub struct GRIPFITTorch {
    pub fwd: f64,
    pub palm: f64,
}
impl GRIPFITTorch {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        GRIPFITTorch {
            fwd: 0.0,
            palm: 0.0,
        }
    }
}
impl Default for GRIPFITTorch { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/rigProps.ts:817 SLING_REST, src/game/rigProps.ts:817)
#[derive(Clone, Copy, Debug)]
pub struct SLINGREST {
    pub pivot: f64,
    pub len: f64,
    pub clear: f64,
    pub max: f64,
    pub soft: f64,
    pub n: f64,
}
impl SLINGREST {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SLINGREST {
            pivot: 0.0,
            len: 0.0,
            clear: 0.0,
            max: 0.0,
            soft: 0.0,
            n: 0.0,
        }
    }
}
impl Default for SLINGREST { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/rigProps.ts:1020 BIGRIG, src/game/rigProps.ts:1020)
#[derive(Clone, Copy, Debug)]
pub struct BIGRIG {
    pub hammer: crate::rig_props::BIGRIGHammer,
    pub shotgun: crate::rig_props::BIGRIGShotgun,
    pub clear: crate::rig_props::BIGRIGClear,
}
impl BIGRIG {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BIGRIG {
            hammer: crate::rig_props::BIGRIGHammer::zeroed(),
            shotgun: crate::rig_props::BIGRIGShotgun::zeroed(),
            clear: crate::rig_props::BIGRIGClear::zeroed(),
        }
    }
}
impl Default for BIGRIG { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/rigProps.ts:1021 BIGRIG.hammer, src/game/rigProps.ts:1021)
#[derive(Clone, Copy, Debug)]
pub struct BIGRIGHammer {
    pub handle: crate::rig_props::Seg,
    pub head: crate::rig_props::Seg,
    pub grip: crate::rig_props::BIGRIGHammerGrip,
    pub hands: f64,
    pub head_at: f64,
    pub back: crate::rig_props::RigSc20kSlung,
}
impl BIGRIGHammer {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BIGRIGHammer {
            handle: crate::rig_props::Seg::zeroed(),
            head: crate::rig_props::Seg::zeroed(),
            grip: crate::rig_props::BIGRIGHammerGrip::zeroed(),
            hands: 0.0,
            head_at: 0.0,
            back: crate::rig_props::RigSc20kSlung::zeroed(),
        }
    }
}
impl Default for BIGRIGHammer { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/rigProps.ts:1024 BIGRIGHammer.grip, src/game/rigProps.ts:1024)
#[derive(Clone, Copy, Debug)]
pub struct BIGRIGHammerGrip {
    pub fwd: f64,
    pub palm: f64,
    pub side: f64,
}
impl BIGRIGHammerGrip {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BIGRIGHammerGrip {
            fwd: 0.0,
            palm: 0.0,
            side: 0.0,
        }
    }
}
impl Default for BIGRIGHammerGrip { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/rigProps.ts:1027 BIGRIG.shotgun, src/game/rigProps.ts:1027)
#[derive(Clone, Copy, Debug)]
pub struct BIGRIGShotgun {
    pub back: crate::rig_props::RigSc20kSlung,
}
impl BIGRIGShotgun {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BIGRIGShotgun {
            back: crate::rig_props::RigSc20kSlung::zeroed(),
        }
    }
}
impl Default for BIGRIGShotgun { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/rigProps.ts:1028 BIGRIG.clear, src/game/rigProps.ts:1028)
#[derive(Clone, Copy, Debug)]
pub struct BIGRIGClear {
    pub hammer: f64,
    pub shotgun: f64,
}
impl BIGRIGClear {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BIGRIGClear {
            hammer: 0.0,
            shotgun: 0.0,
        }
    }
}
impl Default for BIGRIGClear { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: a union)
#[derive(Clone, Debug)]
pub enum AssignTreeDstValue {
    Num(f64),
    Array(Vec<f64>),
    Arm2(JsMap<String, crate::rig_props::AssignTreeDstValue>),
}

/// (TypeScript: a union)
#[derive(Clone, Debug)]
pub enum AssignTreeSrcValue {
    Num(f64),
    Array(Vec<f64>),
    Arm2(JsMap<String, crate::rig_props::AssignTreeSrcValue>),
}

/// (TypeScript: inline at src/game/rigProps.ts:445 pistolFrame(P), src/game/rigProps.ts:445)
#[derive(Clone, Copy, Debug)]
pub struct PistolFrameP {
    pub adj: Option<crate::rig_props::Adj>,
}
impl PistolFrameP {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PistolFrameP {
            adj: None,
        }
    }
}
impl Default for PistolFrameP { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/rigProps.ts:445 pistolFrame ->, src/game/rigProps.ts:445)
#[derive(Clone, Copy, Debug)]
pub struct PistolFrame {
    pub hand_f: crate::rig_props::Frame,
    pub g: crate::rig_props::Frame,
}
impl PistolFrame {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PistolFrame {
            hand_f: crate::rig_props::Frame::zeroed(),
            g: crate::rig_props::Frame::zeroed(),
        }
    }
}
impl Default for PistolFrame { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/rigProps.ts:543 kitGunFrame(P), src/game/rigProps.ts:543)
#[derive(Clone, Copy, Debug)]
pub struct KitGunFrameP {
    pub adj: Option<crate::rig_props::Adj>,
}
impl KitGunFrameP {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        KitGunFrameP {
            adj: None,
        }
    }
}
impl Default for KitGunFrameP { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/rigProps.ts:658 longLookOf ->, src/game/rigProps.ts:658)
#[derive(Clone, Debug)]
pub struct LongLookOf {
    pub id: crate::weapons::GunModelId,
    pub l: crate::gunlook::LongModel,
}
impl LongLookOf {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LongLookOf {
            id: crate::weapons::GunModelId::M870,
            l: crate::gunlook::LongModel::zeroed(),
        }
    }
}
impl Default for LongLookOf { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/rigProps.ts:673 modelGrip ->, src/game/rigProps.ts:673)
#[derive(Clone, Copy, Debug)]
pub struct ModelGrip {
    pub wrist: V3,
    pub fingers: V3,
    pub back: V3,
}
impl ModelGrip {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ModelGrip {
            wrist: [0.0; 3],
            fingers: [0.0; 3],
            back: [0.0; 3],
        }
    }
}
impl Default for ModelGrip { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/rigProps.ts:684 holsterBoxes(parts), src/game/rigProps.ts:684)
#[derive(Clone, Copy, Debug)]
pub struct HolsterBoxesParts {
    pub slide: crate::rig_props::Seg,
    pub grip: crate::rig_props::Seg,
    pub suppressor: Option<crate::rig_props::Seg>,
}
impl HolsterBoxesParts {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        HolsterBoxesParts {
            slide: crate::rig_props::Seg::zeroed(),
            grip: crate::rig_props::Seg::zeroed(),
            suppressor: None,
        }
    }
}
impl Default for HolsterBoxesParts { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/rigProps.ts:763 carbineFrame ->, src/game/rigProps.ts:763)
#[derive(Clone, Copy, Debug)]
pub struct CarbineFrame {
    pub hand_f: crate::rig_props::Frame,
    pub g: crate::rig_props::Frame,
}
impl CarbineFrame {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CarbineFrame {
            hand_f: crate::rig_props::Frame::zeroed(),
            g: crate::rig_props::Frame::zeroed(),
        }
    }
}
impl Default for CarbineFrame { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/rigProps.ts:791 carbineGrip ->, src/game/rigProps.ts:791)
#[derive(Clone, Copy, Debug)]
pub struct CarbineGrip {
    pub wrist: V3,
    pub fingers: V3,
    pub back: V3,
}
impl CarbineGrip {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CarbineGrip {
            wrist: [0.0; 3],
            fingers: [0.0; 3],
            back: [0.0; 3],
        }
    }
}
impl Default for CarbineGrip { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/rigProps.ts:795 longGripOf(G), src/game/rigProps.ts:795)
#[derive(Clone, Copy, Debug)]
pub struct LongGripOfG {
    pub off: V3,
    pub fingers: V3,
    pub back: V3,
}
impl LongGripOfG {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LongGripOfG {
            off: [0.0; 3],
            fingers: [0.0; 3],
            back: [0.0; 3],
        }
    }
}
impl Default for LongGripOfG { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/rigProps.ts:795 longGripOf ->, src/game/rigProps.ts:795)
#[derive(Clone, Copy, Debug)]
pub struct LongGripOf {
    pub wrist: V3,
    pub fingers: V3,
    pub back: V3,
}
impl LongGripOf {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LongGripOf {
            wrist: [0.0; 3],
            fingers: [0.0; 3],
            back: [0.0; 3],
        }
    }
}
impl Default for LongGripOf { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/rigProps.ts:820 slingFrontHand ->, src/game/rigProps.ts:820)
#[derive(Clone, Copy, Debug)]
pub struct SlingFrontHand {
    pub at: V3,
    pub fing: V3,
    pub back: V3,
}
impl SlingFrontHand {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SlingFrontHand {
            at: [0.0; 3],
            fing: [0.0; 3],
            back: [0.0; 3],
        }
    }
}
impl Default for SlingFrontHand { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/rigProps.ts:840 slungBase ->, src/game/rigProps.ts:840)
#[derive(Clone, Copy, Debug)]
pub struct SlungBase {
    pub o: V3,
    pub z: V3,
    pub cf: crate::rig_props::Frame,
    pub want: f64,
}
impl SlungBase {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SlungBase {
            o: [0.0; 3],
            z: [0.0; 3],
            cf: crate::rig_props::Frame::zeroed(),
            want: 0.0,
        }
    }
}
impl Default for SlungBase { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Sc20kFrameWhere {
    Hand,
    Slung,
}
impl Sc20kFrameWhere {
    pub const fn as_str(self) -> &'static str {
        match self {
            Sc20kFrameWhere::Hand => "hand",
            Sc20kFrameWhere::Slung => "slung",
        }
    }
}

/// (TypeScript: inline at src/game/rigProps.ts:874 sc20kFrame ->, src/game/rigProps.ts:874)
#[derive(Clone, Copy, Debug)]
pub struct Sc20kFrame {
    pub parent: crate::rig_props::Frame,
    pub g: crate::rig_props::Frame,
}
impl Sc20kFrame {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Sc20kFrame {
            parent: crate::rig_props::Frame::zeroed(),
            g: crate::rig_props::Frame::zeroed(),
        }
    }
}
impl Default for Sc20kFrame { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Sc20kBoxesWhere {
    Hand,
    Slung,
}
impl Sc20kBoxesWhere {
    pub const fn as_str(self) -> &'static str {
        match self {
            Sc20kBoxesWhere::Hand => "hand",
            Sc20kBoxesWhere::Slung => "slung",
        }
    }
}

/// (TypeScript: inline at src/game/rigProps.ts:925 sc20kGrip ->, src/game/rigProps.ts:925)
#[derive(Clone, Copy, Debug)]
pub struct Sc20kGrip {
    pub wrist: V3,
    pub fingers: V3,
    pub back: V3,
}
impl Sc20kGrip {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Sc20kGrip {
            wrist: [0.0; 3],
            fingers: [0.0; 3],
            back: [0.0; 3],
        }
    }
}
impl Default for Sc20kGrip { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum LongHoldWeapon {
    Carbine,
    Sc20k,
}
impl LongHoldWeapon {
    pub const fn as_str(self) -> &'static str {
        match self {
            LongHoldWeapon::Carbine => "carbine",
            LongHoldWeapon::Sc20k => "sc20k",
        }
    }
}

/// (TypeScript: inline at src/game/rigProps.ts:945 sc20kLightPose ->, src/game/rigProps.ts:945)
#[derive(Clone, Copy, Debug)]
pub struct Sc20kLightPose {
    pub pos: V3,
    pub dir: V3,
}
impl Sc20kLightPose {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Sc20kLightPose {
            pos: [0.0; 3],
            dir: [0.0; 3],
        }
    }
}
impl Default for Sc20kLightPose { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/rigProps.ts:956 carbineLightPose ->, src/game/rigProps.ts:956)
#[derive(Clone, Copy, Debug)]
pub struct CarbineLightPose {
    pub pos: V3,
    pub dir: V3,
}
impl CarbineLightPose {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        CarbineLightPose {
            pos: [0.0; 3],
            dir: [0.0; 3],
        }
    }
}
impl Default for CarbineLightPose { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/rigProps.ts:964 rigLightPose ->, src/game/rigProps.ts:964)
#[derive(Clone, Copy, Debug)]
pub struct RigLightPose {
    pub pos: V3,
    pub dir: V3,
}
impl RigLightPose {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RigLightPose {
            pos: [0.0; 3],
            dir: [0.0; 3],
        }
    }
}
impl Default for RigLightPose { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum RigTorchPosWeapon {
    Pistol,
    Carbine,
    Sc20k,
}
impl RigTorchPosWeapon {
    pub const fn as_str(self) -> &'static str {
        match self {
            RigTorchPosWeapon::Pistol => "pistol",
            RigTorchPosWeapon::Carbine => "carbine",
            RigTorchPosWeapon::Sc20k => "sc20k",
        }
    }
}

/// (TypeScript: inline at src/game/rigProps.ts:992 torchCarryDir(low), src/game/rigProps.ts:992)
#[derive(Clone, Copy, Debug)]
pub struct TorchCarryDirLow {
    pub tan: f64,
    pub rate: f64,
}
impl TorchCarryDirLow {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        TorchCarryDirLow {
            tan: 0.0,
            rate: 0.0,
        }
    }
}
impl Default for TorchCarryDirLow { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum BigClearOfKind {
    Shotgun,
    Hammer,
}
impl BigClearOfKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            BigClearOfKind::Shotgun => "shotgun",
            BigClearOfKind::Hammer => "hammer",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum PackSideDimsWhat {
    Rifle,
    Shotgun,
    Hammer,
}
impl PackSideDimsWhat {
    pub const fn as_str(self) -> &'static str {
        match self {
            PackSideDimsWhat::Rifle => "rifle",
            PackSideDimsWhat::Shotgun => "shotgun",
            PackSideDimsWhat::Hammer => "hammer",
        }
    }
}

/// (TypeScript: inline at src/game/rigProps.ts:1041 packSideDims ->, src/game/rigProps.ts:1041)
#[derive(Clone, Copy, Debug)]
pub struct PackSideDims {
    pub grip: Option<f64>,
    pub thick: f64,
    pub head: Option<V3>,
    pub rear: Option<f64>,
    pub top: Option<f64>,
    pub len: f64,
}
impl PackSideDims {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        PackSideDims {
            grip: None,
            thick: 0.0,
            head: None,
            rear: None,
            top: None,
            len: 0.0,
        }
    }
}
impl Default for PackSideDims { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum PackSideBindWhat {
    Rifle,
    Shotgun,
    Hammer,
}
impl PackSideBindWhat {
    pub const fn as_str(self) -> &'static str {
        match self {
            PackSideBindWhat::Rifle => "rifle",
            PackSideBindWhat::Shotgun => "shotgun",
            PackSideBindWhat::Hammer => "hammer",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum PackSideWhat {
    Rifle,
    Shotgun,
    Hammer,
}
impl PackSideWhat {
    pub const fn as_str(self) -> &'static str {
        match self {
            PackSideWhat::Rifle => "rifle",
            PackSideWhat::Shotgun => "shotgun",
            PackSideWhat::Hammer => "hammer",
        }
    }
}

/// (TypeScript: inline at src/game/rigProps.ts:1063 bigBackOf ->, src/game/rigProps.ts:1063)
#[derive(Clone, Copy, Debug)]
pub struct BigBackOf {
    pub o: V3,
    pub z: V3,
    pub cf: crate::rig_props::Frame,
}
impl BigBackOf {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BigBackOf {
            o: [0.0; 3],
            z: [0.0; 3],
            cf: crate::rig_props::Frame::zeroed(),
        }
    }
}
impl Default for BigBackOf { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum BigBoxesKind {
    Shotgun,
    Hammer,
}
impl BigBoxesKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            BigBoxesKind::Shotgun => "shotgun",
            BigBoxesKind::Hammer => "hammer",
        }
    }
}

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum HungBoxesWeapon {
    Carbine,
    Sc20k,
}
impl HungBoxesWeapon {
    pub const fn as_str(self) -> &'static str {
        match self {
            HungBoxesWeapon::Carbine => "carbine",
            HungBoxesWeapon::Sc20k => "sc20k",
        }
    }
}

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct RigPropsMod {
    /// (src/game/rigProps.ts:341 curOwner)
    pub cur_owner: f64,
    /// (src/game/rigProps.ts:352 rotOf)
    pub rot_of: Option<crate::rig_props::Frame>,
    /// (src/game/rigProps.ts:352 rotYaw)
    pub rot_yaw: f64,
    /// (src/game/rigProps.ts:352 rotQ)
    pub rot_q: Quat,
}
impl RigPropsMod {
    pub fn blank() -> Self {
        RigPropsMod {
            cur_owner: 0.0,
            rot_of: None,
            rot_yaw: 0.0,
            rot_q: [0.0, 0.0, 0.0, 1.0],
        }
    }
}

/// (src/game/rigProps.ts:136 RIG: a table nothing writes)
pub fn rig() -> crate::rig_props::Rig { crate::rig_props::Rig { goggles: crate::rig_props::RigGoggles { on: crate::rig_props::RigGogglesOn { fwd: 0.105, up: 0.1 }, off: crate::rig_props::RigGogglesOff { fwd: 0.075, up: 0.185 }, lens_side: 0.03, lens_down: -0.012, lens_up: 0.026, lens_half: 0.013, lens_thick: 0.008, housing: crate::rig_props::RigGogglesHousing { back: 0.03, half: [0.055, 0.028, 0.03] }, glow_on: [1.5, 14.0, 3.0], glow_off: [0.35, 3.2, 0.7], tubes: crate::rig_props::RigGogglesTubes { side: 0.032, down: -0.012, half: 0.015, back: 0.06, front: 0.012, glass: 0.003, bridge: crate::rig_props::RigGogglesTubesBridge { at: [0.0, 0.012, -0.03], half: [0.03, 0.007, 0.012] }, arm: crate::rig_props::RigGogglesTubesArm { at: [0.0, 0.034, -0.045], half: [0.009, 0.018, 0.009] } }, on_adj: Some(crate::rig_props::Adj { off: [-0.004, 0.04, 0.02], rot: [4.761, 0.0, 1.1] }), off_adj: Some(crate::rig_props::Adj { off: [-0.006, 0.048, 0.032], rot: [-17.2, 0.0, 0.7] }), housing_adj: None, lens_adj: None, follow_pitch: 1.0 }, pistol: crate::rig_props::RigPistol { slide: crate::rig_props::Seg { l0: 0.02, l1: 0.21, thick: 0.016, tall: 0.026, adj: None }, suppressor: crate::rig_props::Seg { l0: 0.21, l1: 0.36, thick: 0.019, tall: 0.019, adj: None }, grip: crate::rig_props::Seg { l0: 0.0, l1: 0.095, thick: 0.014, tall: 0.017, adj: Some(crate::rig_props::Adj { off: [0.0, -0.024, 0.05], rot: [108.0, 0.0, 0.0] }) }, rail: crate::rig_props::RigPistolRail { l0: 0.09, l1: 0.2, thick: 0.013, tall: 0.012, adj: Some(crate::rig_props::Adj { off: [0.0, 0.0, 0.064], rot: [0.0, 0.0, 0.0] }), under: 0.034 }, lens: crate::rig_props::RigPistolLens { l0: 0.2, l1: 0.215, thick: 0.014, tall: 0.013, adj: None, glow: [14.0, 14.0, 13.0] }, light_under: 0.038, light_ahead: 0.2, adj: Some(crate::rig_props::Adj { off: [0.039, 0.08, 0.035], rot: [-1.3, 0.0, 0.0] }) }, torch: crate::rig_props::RigTorch { body: crate::rig_props::Seg { l0: -0.06, l1: 0.15, thick: 0.022, tall: 0.022, adj: None }, lens: crate::rig_props::Seg { l0: 0.15, l1: 0.18, thick: 0.026, tall: 0.026, adj: None }, lens_glow: 40.0, light_ahead: 0.18, adj: Some(crate::rig_props::Adj { off: [0.076, 0.04, 0.092], rot: [0.0, 6.7, 0.0] }) }, guard_pistol: crate::rig_props::RigGuardPistol { slide: crate::rig_props::Seg { l0: 0.02, l1: 0.2, thick: 0.016, tall: 0.026, adj: Some(crate::rig_props::Adj { off: [0.028, -0.034, 0.073], rot: [11.2, 0.0, 0.5] }) }, grip: crate::rig_props::Seg { l0: 0.0, l1: 0.095, thick: 0.014, tall: 0.017, adj: Some(crate::rig_props::Adj { off: [0.028, -0.06, 0.118], rot: [119.0, 0.0, 0.5] }) }, lens: crate::rig_props::Seg { l0: 0.2, l1: 0.23, thick: 0.015, tall: 0.015, adj: Some(crate::rig_props::Adj { off: [0.029, -0.064, 0.023], rot: [11.8, 0.0, 0.0] }) }, light_ahead: 0.25, adj: Some(crate::rig_props::Adj { off: [0.031, 0.105, -0.05], rot: [-8.6, 0.0, 0.0] }) }, knife: crate::rig_props::RigKnife { blade: crate::rig_props::Seg { l0: 0.045, l1: 0.215, thick: 0.002, tall: 0.009, adj: Some(crate::rig_props::Adj { off: [0.0, 0.003, 0.0], rot: [0.0, 0.0, 0.0] }) }, edge: crate::rig_props::Seg { l0: 0.045, l1: 0.215, thick: 0.002, tall: 0.003, adj: Some(crate::rig_props::Adj { off: [0.0, -0.009, 0.0], rot: [0.0, 0.0, 0.0] }) }, guard: crate::rig_props::Seg { l0: 0.04, l1: 0.048, thick: 0.005, tall: 0.019, adj: None }, handle: crate::rig_props::Seg { l0: -0.065, l1: 0.045, thick: 0.011, tall: 0.013, adj: None }, grip: crate::rig_props::RigKnifeGrip { fwd: 0.08, palm: 0.03, side: 0.0 }, adj: None }, sheath: crate::rig_props::RigSheath { at: [0.07, -0.085, 0.2], along: [0.26, 1.0, -0.2], out: [0.0, 0.18, 1.0], body: crate::rig_props::Seg { l0: 0.046, l1: 0.228, thick: 0.006, tall: 0.016, adj: None } }, can: crate::rig_props::RigCan { sink: -0.02, tilt: 0.35, body: crate::rig_props::Seg { l0: -0.02, l1: 0.12, thick: 0.032, tall: 0.032, adj: None }, band: crate::rig_props::Seg { l0: 0.085, l1: 0.109, thick: 0.034, tall: 0.034, adj: None }, adj: Some(crate::rig_props::Adj { off: [0.0, 0.0, 0.0], rot: [0.0, 0.0, 0.0] }) }, holster: crate::rig_props::RigHolster { down: 0.06, out: 0.115, fwd: 0.01, adj: Some(crate::rig_props::Adj { off: [0.0, 0.0, 0.0], rot: [0.0, 0.0, 0.0] }) }, carbine: crate::rig_props::RigCarbine { receiver: crate::rig_props::Seg { l0: -0.09, l1: 0.155, thick: 0.017, tall: 0.032, adj: Some(crate::rig_props::Adj { off: [0.0, -0.012, 0.0], rot: [0.0, 0.0, 0.0] }) }, handguard: crate::rig_props::Seg { l0: 0.155, l1: 0.275, thick: 0.02, tall: 0.025, adj: Some(crate::rig_props::Adj { off: [0.0, -0.008, 0.0], rot: [0.0, 0.0, 0.0] }) }, barrel: crate::rig_props::Seg { l0: 0.275, l1: 0.34, thick: 0.007, tall: 0.007, adj: None }, muzzle: crate::rig_props::Seg { l0: 0.34, l1: 0.41, thick: 0.015, tall: 0.015, adj: None }, mag: crate::rig_props::Seg { l0: 0.0, l1: 0.11, thick: 0.013, tall: 0.028, adj: Some(crate::rig_props::Adj { off: [0.0, -0.04, 0.085], rot: [78.0, 0.0, 0.0] }) }, mag2: crate::rig_props::Seg { l0: 0.0, l1: 0.1, thick: 0.013, tall: 0.027, adj: Some(crate::rig_props::Adj { off: [0.0, -0.143, 0.106], rot: [60.0, 0.0, 0.0] }) }, grip: crate::rig_props::Seg { l0: 0.0, l1: 0.09, thick: 0.013, tall: 0.016, adj: Some(crate::rig_props::Adj { off: [0.0, -0.045, -0.015], rot: [108.0, 0.0, 0.0] }) }, stock_top: crate::rig_props::Seg { l0: -0.32, l1: -0.09, thick: 0.007, tall: 0.008, adj: Some(crate::rig_props::Adj { off: [0.0, 0.005, 0.0], rot: [0.0, 0.0, 0.0] }) }, stock_low: crate::rig_props::Seg { l0: 0.0, l1: 0.235, thick: 0.007, tall: 0.008, adj: Some(crate::rig_props::Adj { off: [0.0, -0.04, -0.09], rot: [171.0, 0.0, 0.0] }) }, light: crate::rig_props::Seg { l0: 0.18, l1: 0.265, thick: 0.011, tall: 0.011, adj: Some(crate::rig_props::Adj { off: [-0.032, -0.006, 0.0], rot: [0.0, 0.0, 0.0] }) }, lens: crate::rig_props::Seg { l0: 0.265, l1: 0.28, thick: 0.012, tall: 0.012, adj: Some(crate::rig_props::Adj { off: [-0.032, -0.006, 0.0], rot: [0.0, 0.0, 0.0] }) }, lens_glow: 40.0, light_ahead: 0.285, grip_l: crate::rig_props::RigCarbineGripL { off: [0.099, -0.059, 0.149], fingers: [-0.48, -0.11, 0.19], back: [0.45, -0.85, -0.25] }, adj: Some(crate::rig_props::Adj { off: [0.06, 0.08, 0.09], rot: [0.0, 0.0, 0.0] }) }, sc20k: crate::rig_props::RigSc20k { hull: crate::rig_props::Seg { l0: -0.29, l1: 0.12, thick: 0.024, tall: 0.04, adj: Some(crate::rig_props::Adj { off: [0.0, -0.012, 0.0], rot: [0.0, 0.0, 0.0] }) }, butt: crate::rig_props::Seg { l0: -0.31, l1: -0.29, thick: 0.026, tall: 0.048, adj: Some(crate::rig_props::Adj { off: [0.0, -0.018, 0.0], rot: [0.0, 0.0, 0.0] }) }, mag: crate::rig_props::Seg { l0: 0.0, l1: 0.13, thick: 0.012, tall: 0.03, adj: Some(crate::rig_props::Adj { off: [0.0, -0.045, -0.12], rot: [84.0, 0.0, 0.0] }) }, grip: crate::rig_props::Seg { l0: 0.0, l1: 0.09, thick: 0.013, tall: 0.016, adj: Some(crate::rig_props::Adj { off: [0.0, -0.045, -0.015], rot: [108.0, 0.0, 0.0] }) }, fore: crate::rig_props::Seg { l0: 0.12, l1: 0.3, thick: 0.022, tall: 0.03, adj: Some(crate::rig_props::Adj { off: [0.0, -0.014, 0.0], rot: [0.0, 0.0, 0.0] }) }, suppressor: crate::rig_props::Seg { l0: 0.3, l1: 0.43, thick: 0.019, tall: 0.019, adj: None }, rail: crate::rig_props::Seg { l0: -0.16, l1: 0.16, thick: 0.011, tall: 0.008, adj: Some(crate::rig_props::Adj { off: [0.0, 0.036, 0.0], rot: [0.0, 0.0, 0.0] }) }, optic: crate::rig_props::Seg { l0: -0.04, l1: 0.08, thick: 0.017, tall: 0.02, adj: Some(crate::rig_props::Adj { off: [0.0, 0.062, 0.0], rot: [0.0, 0.0, 0.0] }) }, foregrip: crate::rig_props::Seg { l0: 0.0, l1: 0.085, thick: 0.012, tall: 0.012, adj: Some(crate::rig_props::Adj { off: [0.0, -0.044, 0.2], rot: [90.0, 0.0, 0.0] }) }, light: crate::rig_props::Seg { l0: 0.19, l1: 0.27, thick: 0.011, tall: 0.011, adj: Some(crate::rig_props::Adj { off: [-0.034, -0.008, 0.0], rot: [0.0, 0.0, 0.0] }) }, lens: crate::rig_props::Seg { l0: 0.27, l1: 0.285, thick: 0.012, tall: 0.012, adj: Some(crate::rig_props::Adj { off: [-0.034, -0.008, 0.0], rot: [0.0, 0.0, 0.0] }) }, lens_glow: 40.0, light_ahead: 0.29, grip_l: crate::rig_props::RigSc20kGripL { off: [0.06, -0.085, 0.145], fingers: [-0.3, 0.05, 0.95], back: [0.95, -0.15, 0.25] }, slung: crate::rig_props::RigSc20kSlung { off: [-0.02, 0.17, -0.18], bore: [0.5, -0.866, 0.2], roll: 180.0, follow: 0.5, adj: None }, adj: Some(crate::rig_props::Adj { off: [0.06, 0.08, 0.09], rot: [0.0, 0.0, 0.0] }) }, m4: crate::rig_props::RigM4 { hull: crate::rig_props::Seg { l0: -0.075, l1: 0.115, thick: 0.017, tall: 0.036, adj: Some(crate::rig_props::Adj { off: [0.0, -0.012, 0.0], rot: [0.0, 0.0, 0.0] }) }, butt: crate::rig_props::Seg { l0: -0.31, l1: -0.19, thick: 0.017, tall: 0.036, adj: Some(crate::rig_props::Adj { off: [0.0, -0.02, 0.0], rot: [0.0, 0.0, 0.0] }) }, mag: crate::rig_props::Seg { l0: 0.0, l1: 0.14, thick: 0.011, tall: 0.03, adj: Some(crate::rig_props::Adj { off: [0.0, -0.045, 0.075], rot: [80.0, 0.0, 0.0] }) }, fore: crate::rig_props::Seg { l0: 0.115, l1: 0.3, thick: 0.022, tall: 0.022, adj: Some(crate::rig_props::Adj { off: [0.0, -0.003, 0.0], rot: [0.0, 0.0, 0.0] }) }, rail: crate::rig_props::Seg { l0: -0.07, l1: 0.3, thick: 0.009, tall: 0.005, adj: Some(crate::rig_props::Adj { off: [0.0, 0.027, 0.0], rot: [0.0, 0.0, 0.0] }) }, optic: crate::rig_props::Seg { l0: 0.0, l1: 0.07, thick: 0.016, tall: 0.02, adj: Some(crate::rig_props::Adj { off: [0.0, 0.052, 0.0], rot: [0.0, 0.0, 0.0] }) }, tube: crate::rig_props::Seg { l0: -0.19, l1: -0.075, thick: 0.012, tall: 0.012, adj: Some(crate::rig_props::Adj { off: [0.0, -0.002, 0.0], rot: [0.0, 0.0, 0.0] }) }, peq: crate::rig_props::Seg { l0: 0.215, l1: 0.29, thick: 0.017, tall: 0.011, adj: Some(crate::rig_props::Adj { off: [0.0, 0.043, 0.0], rot: [0.0, 0.0, 0.0] }) }, gl: crate::rig_props::Seg { l0: 0.14, l1: 0.3, thick: 0.02, tall: 0.024, adj: Some(crate::rig_props::Adj { off: [0.0, -0.05, 0.0], rot: [0.0, 0.0, 0.0] }) } } } }

/// (src/game/rigProps.ts:245 COL: a table nothing writes)
pub fn col() -> crate::rig_props::COL { crate::rig_props::COL { gun: [0.05, 0.05, 0.055], can: [0.09, 0.09, 0.095], torch: [0.16, 0.16, 0.17], lens_body: [0.3, 0.3, 0.3], goggles_lens: [0.02, 0.05, 0.03], goggles_housing: [0.04, 0.045, 0.04], blade: [0.11, 0.11, 0.12], edge: [0.34, 0.34, 0.36], knife_grip: [0.02, 0.02, 0.022], sheath: [0.028, 0.03, 0.026], furniture: [0.2, 0.085, 0.05], anodised: [0.03, 0.03, 0.033], fde: [0.2, 0.155, 0.1], dark_glass: [0.015, 0.02, 0.022], haft: [0.3, 0.22, 0.07], sledge: [0.04, 0.04, 0.045], brass: [0.5, 0.36, 0.12], shell: [0.26, 0.04, 0.03], cloth: [0.11, 0.1, 0.065], polymer: [0.045, 0.05, 0.046], dark_metal: [0.026, 0.026, 0.03] } }

/// (src/game/rigProps.ts:259 KNIFE_TAG)
pub const KNIFE_TAG: &str = "knife";

/// (src/game/rigProps.ts:261 SHEATH_TAG)
pub const SHEATH_TAG: &str = "sheath";

/// (src/game/rigProps.ts:263 SC20K_TAG)
pub const SC20K_TAG: &str = "sc20k";

/// (src/game/rigProps.ts:264 PROP_FLAGS)
pub const PROP_FLAGS: f64 = 8.0;

/// (src/game/rigProps.ts:265 GLOW_FLAGS)
pub const GLOW_FLAGS: f64 = 10.0;

/// (src/game/rigProps.ts:282 RIG_DEFAULTS: a table nothing writes)
pub fn rig_defaults() -> crate::rig_props::Rig { crate::rig_props::rig() }

/// (src/game/rigProps.ts:349 NO_GLOW: a table nothing writes)
pub fn no_glow() -> V3 { [0.0, 0.0, 0.0] }

/// (src/game/rigProps.ts:374 NV_SWING)
pub const NV_SWING: f64 = 0.018;

/// (src/game/rigProps.ts:429 GRIPFIT: a table nothing writes)
pub fn gripfit() -> crate::rig_props::GRIPFIT { crate::rig_props::GRIPFIT { can: crate::rig_props::GRIPFITCan { fwd: 0.106, palm: 0.047, side: 0.0 }, pistol: [-0.006, 0.0, 0.023], long: [-0.031, 0.0, 0.037], fore: crate::rig_props::GRIPFITFore { back: 0.062, lift: 0.011, down: 0.015 }, mag: crate::rig_props::GRIPFITMag { fwd: 0.095, palm: 0.028, side: 0.03 }, guard_pistol: [-0.026, -0.005, 0.035], torch: crate::rig_props::GRIPFITTorch { fwd: 0.1, palm: 0.036 } } }

/// (src/game/rigProps.ts:533 ADJ_NONE: a table nothing writes)
pub fn adj_none() -> crate::rig_props::Adj { crate::rig_props::Adj { off: [0.0, 0.0, 0.0], rot: [0.0, 0.0, 0.0] } }

/// (src/game/rigProps.ts:644 GUNMODEL_TAG)
pub const GUNMODEL_TAG: &str = "gunmodel";

/// (src/game/rigProps.ts:656 COVER_OPEN)
pub const COVER_OPEN: f64 = 75.0;

/// (src/game/rigProps.ts:702 GRIP_HUNG: a table nothing writes)
pub fn grip_hung() -> crate::rig_props::Adj { crate::rig_props::Adj { off: [0.0, -0.024, 0.05], rot: [108.0, 0.0, 0.0] } }

/// (src/game/rigProps.ts:813 SLING_FRONT: a table nothing writes)
pub fn sling_front() -> crate::rig_props::RigSc20kSlung { crate::rig_props::RigSc20kSlung { off: [-0.07, 0.0, 0.2], bore: [0.55, -0.82, 0.12], roll: 180.0, follow: 0.85, adj: None } }

/// (src/game/rigProps.ts:817 SLING_REST: a table nothing writes)
pub fn sling_rest() -> crate::rig_props::SLINGREST { crate::rig_props::SLINGREST { pivot: 0.28, len: 0.7, clear: 0.11, max: 1.4, soft: 0.1, n: 6.0 } }

/// (src/game/rigProps.ts:828 RAIL_LENS_COLOR: a table nothing writes)
pub fn rail_lens_color() -> V3 { [0.9, 0.96, 1.0] }

/// (src/game/rigProps.ts:839 SLUNG_CAST_OUT)
pub const SLUNG_CAST_OUT: f64 = 0.03;

/// (src/game/rigProps.ts:884 BARE_BARREL)
pub const BARE_BARREL: f64 = 0.008;

/// (src/game/rigProps.ts:920 M4_BOXES)
pub const M4_BOXES: f64 = 14.0;

/// (src/game/rigProps.ts:1002 TORCH_CARRY_RATE)
pub const TORCH_CARRY_RATE: f64 = 13.0;

/// (src/game/rigProps.ts:1020 BIGRIG: a table nothing writes)
pub fn bigrig() -> crate::rig_props::BIGRIG { crate::rig_props::BIGRIG { hammer: crate::rig_props::BIGRIGHammer { handle: crate::rig_props::Seg { l0: -0.46, l1: 0.265, thick: 0.02, tall: 0.02, adj: None }, head: crate::rig_props::Seg { l0: 0.265, l1: 0.335, thick: 0.085, tall: 0.045, adj: None }, grip: crate::rig_props::BIGRIGHammerGrip { fwd: 0.08, palm: 0.03, side: 0.0 }, hands: 0.36, head_at: 0.3, back: crate::rig_props::RigSc20kSlung { off: [-0.055, 0.01, -0.19], bore: [-0.42, 0.91, 0.0], roll: 90.0, follow: 0.5, adj: None } }, shotgun: crate::rig_props::BIGRIGShotgun { back: crate::rig_props::RigSc20kSlung { off: [-0.02, 0.17, -0.18], bore: [0.5, -0.866, 0.2], roll: 180.0, follow: 0.5, adj: None } }, clear: crate::rig_props::BIGRIGClear { hammer: 0.11, shotgun: 0.075 } } }

/// (src/game/rigProps.ts:1059 BIG_TAG)
pub const BIG_TAG: &str = "big";
