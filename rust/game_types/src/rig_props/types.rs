//! The types of `rig_props`.
#![allow(unused_imports)]
use crate::{Blank, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, Default)]
pub struct Seg {
    pub l0: f64,
    pub l1: f64,
    pub thick: f64,
    pub tall: f64,
    pub adj: Option<crate::rig_props::Adj>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Adj {
    pub off: V3,
    pub rot: V3,
}

#[derive(Clone, Copy, Debug, Default)]
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

#[derive(Clone, Copy, Debug, Default)]
pub struct RigGoggles {
    pub on: crate::rig_props::RigGogglesOn,
    pub off: crate::rig_props::RigGogglesOff,
    pub tubes: crate::rig_props::RigGogglesTubes,
    pub on_adj: Option<crate::rig_props::Adj>,
    pub off_adj: Option<crate::rig_props::Adj>,
    pub housing_adj: Option<crate::rig_props::Adj>,
    pub lens_adj: Option<crate::rig_props::Adj>,
    pub follow_pitch: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct RigGogglesOn {
    pub fwd: f64,
    pub up: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct RigGogglesOff {
    pub fwd: f64,
    pub up: f64,
}

#[derive(Clone, Copy, Debug, Default)]
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

#[derive(Clone, Copy, Debug, Default)]
pub struct RigGogglesTubesBridge {
    pub at: V3,
    pub half: V3,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct RigGogglesTubesArm {
    pub at: V3,
    pub half: V3,
}

#[derive(Clone, Copy, Debug, Default)]
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

#[derive(Clone, Copy, Debug, Default)]
pub struct RigPistolRail {
    pub l0: f64,
    pub l1: f64,
    pub thick: f64,
    pub tall: f64,
    pub adj: Option<crate::rig_props::Adj>,
    pub under: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct RigPistolLens {
    pub l0: f64,
    pub l1: f64,
    pub thick: f64,
    pub tall: f64,
    pub adj: Option<crate::rig_props::Adj>,
    pub glow: V3,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct RigTorch {
    pub body: crate::rig_props::Seg,
    pub lens: crate::rig_props::Seg,
    pub lens_glow: f64,
    pub light_ahead: f64,
    pub adj: Option<crate::rig_props::Adj>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct RigGuardPistol {
    pub slide: crate::rig_props::Seg,
    pub grip: crate::rig_props::Seg,
    pub lens: crate::rig_props::Seg,
    pub light_ahead: f64,
    pub adj: Option<crate::rig_props::Adj>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct RigKnife {
    pub blade: crate::rig_props::Seg,
    pub edge: crate::rig_props::Seg,
    pub guard: crate::rig_props::Seg,
    pub handle: crate::rig_props::Seg,
    pub grip: crate::rig_props::RigKnifeGrip,
    pub adj: Option<crate::rig_props::Adj>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct RigKnifeGrip {
    pub fwd: f64,
    pub palm: f64,
    pub side: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct RigSheath {
    pub at: V3,
    pub along: V3,
    pub out: V3,
    pub body: crate::rig_props::Seg,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct RigCan {
    pub sink: f64,
    pub tilt: f64,
    pub body: crate::rig_props::Seg,
    pub band: crate::rig_props::Seg,
    pub adj: Option<crate::rig_props::Adj>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct RigHolster {
    pub down: f64,
    pub out: f64,
    pub fwd: f64,
    pub adj: Option<crate::rig_props::Adj>,
}

#[derive(Clone, Copy, Debug, Default)]
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

#[derive(Clone, Copy, Debug, Default)]
pub struct RigCarbineGripL {
    pub off: V3,
    pub fingers: V3,
    pub back: V3,
}

#[derive(Clone, Copy, Debug, Default)]
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

#[derive(Clone, Copy, Debug, Default)]
pub struct RigSc20kGripL {
    pub off: V3,
    pub fingers: V3,
    pub back: V3,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct RigSc20kSlung {
    pub off: V3,
    pub bore: V3,
    pub roll: f64,
    pub follow: f64,
    pub adj: Option<crate::rig_props::Adj>,
}

#[derive(Clone, Copy, Debug, Default)]
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

#[derive(Clone, Copy, Debug, Default)]
pub struct Frame {
    pub o: V3,
    pub x: V3,
    pub y: V3,
    pub z: V3,
}

#[derive(Clone, Debug, Default)]
pub struct RigNodeMeta {
}

#[derive(Clone, Copy, Debug, Default)]
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

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum RigOptsWeapon {
    Pistol,
    Carbine,
    Sc20k,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum RigOptsRifle {
    Hand,
    Slung,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct KitDraw {
    pub pistol: Option<f64>,
    pub rifle: Option<f64>,
    pub roll: Option<f64>,
    pub front: Option<bool>,
    pub rest: Option<f64>,
    pub no_mag: Option<bool>,
    pub mag_l: Option<crate::slotsetup::SetupGun>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum RigOptsBig {
    Shotgun,
    Hammer,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct COL {
    pub gun: V3,
    pub can: V3,
    pub torch: V3,
    pub lens_body: V3,
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

#[derive(Clone, Copy, Debug, Default)]
pub struct GRIPFIT {
    pub can: crate::rig_props::GRIPFITCan,
    pub pistol: V3,
    pub long: V3,
    pub fore: crate::rig_props::GRIPFITFore,
    pub mag: crate::rig_props::GRIPFITMag,
    pub guard_pistol: V3,
    pub torch: crate::rig_props::GRIPFITTorch,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct GRIPFITCan {
    pub fwd: f64,
    pub palm: f64,
    pub side: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct GRIPFITFore {
    pub back: f64,
    pub lift: f64,
    pub down: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct GRIPFITMag {
    pub fwd: f64,
    pub palm: f64,
    pub side: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct GRIPFITTorch {
    pub fwd: f64,
    pub palm: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SLINGREST {
    pub pivot: f64,
    pub len: f64,
    pub clear: f64,
    pub max: f64,
    pub soft: f64,
    pub n: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Bigrig {
    pub hammer: crate::rig_props::BigrigHammer,
    pub shotgun: crate::rig_props::BigrigShotgun,
    pub clear: crate::rig_props::BigrigClear,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct BigrigHammer {
    pub handle: crate::rig_props::Seg,
    pub head: crate::rig_props::Seg,
    pub grip: crate::rig_props::BigrigHammerGrip,
    pub hands: f64,
    pub head_at: f64,
    pub back: crate::rig_props::RigSc20kSlung,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct BigrigHammerGrip {
    pub fwd: f64,
    pub palm: f64,
    pub side: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct BigrigShotgun {
    pub back: crate::rig_props::RigSc20kSlung,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct BigrigClear {
    pub hammer: f64,
    pub shotgun: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct PistolFrameP {
    pub adj: Option<crate::rig_props::Adj>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct PistolFrame {
    pub hand_f: crate::rig_props::Frame,
    pub g: crate::rig_props::Frame,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct KitGunFrameP {
    pub adj: Option<crate::rig_props::Adj>,
}

#[derive(Clone, Debug, Default)]
pub struct LongLookOf {
    pub id: crate::weapons::GunModelId,
    pub l: crate::gunlook::LongModel,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct HolsterBoxesParts {
    pub slide: crate::rig_props::Seg,
    pub grip: crate::rig_props::Seg,
    pub suppressor: Option<crate::rig_props::Seg>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct CarbineFrame {
    pub hand_f: crate::rig_props::Frame,
    pub g: crate::rig_props::Frame,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct LongGripOfG {
    pub off: V3,
    pub fingers: V3,
    pub back: V3,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SlingFrontHand {
    pub at: V3,
    pub fing: V3,
    pub back: V3,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SlungBase {
    pub o: V3,
    pub z: V3,
    pub cf: crate::rig_props::Frame,
    pub want: f64,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Sc20kFrameWhere {
    Hand,
    Slung,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Sc20kFrame {
    pub g: crate::rig_props::Frame,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Sc20kBoxesWhere {
    Hand,
    Slung,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum LongHoldWeapon {
    Carbine,
    Sc20k,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Sc20kLightPose {
    pub pos: V3,
    pub dir: V3,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct CarbineLightPose {
    pub pos: V3,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct RigLightPose {
    pub pos: V3,
    pub dir: V3,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum RigTorchPosWeapon {
    Pistol,
    Carbine,
    Sc20k,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct TorchCarryDirLow {
    pub tan: f64,
    pub rate: f64,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum BigClearOfKind {
    Shotgun,
    Hammer,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum PackSideDimsWhat {
    Rifle,
    Shotgun,
    Hammer,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct PackSideDims {
    pub grip: Option<f64>,
    pub thick: f64,
    pub head: Option<V3>,
    pub rear: Option<f64>,
    pub top: Option<f64>,
    pub len: f64,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum PackSideBindWhat {
    Rifle,
    Shotgun,
    Hammer,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum PackSideWhat {
    Rifle,
    Shotgun,
    Hammer,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct BigBackOf {
    pub o: V3,
    pub z: V3,
    pub cf: crate::rig_props::Frame,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum BigBoxesKind {
    Shotgun,
    Hammer,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum HungBoxesWeapon {
    Carbine,
    Sc20k,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes, Blank)]
pub struct RigPropsMod {
    pub cur_owner: f64,
}

/// the table, for who takes it whole
pub fn rig() -> crate::rig_props::Rig { RIG.clone() }
pub static RIG: crate::rig_props::Rig = crate::rig_props::Rig { goggles: crate::rig_props::RigGoggles { on: crate::rig_props::RigGogglesOn { fwd: 0.105, up: 0.1 }, off: crate::rig_props::RigGogglesOff { fwd: 0.075, up: 0.185 }, tubes: crate::rig_props::RigGogglesTubes { side: 0.032, down: -0.012, half: 0.015, back: 0.06, front: 0.012, glass: 0.003, bridge: crate::rig_props::RigGogglesTubesBridge { at: [0.0, 0.012, -0.03], half: [0.03, 0.007, 0.012] }, arm: crate::rig_props::RigGogglesTubesArm { at: [0.0, 0.034, -0.045], half: [0.009, 0.018, 0.009] } }, on_adj: Some(crate::rig_props::Adj { off: [-0.004, 0.04, 0.02], rot: [4.761, 0.0, 1.1] }), off_adj: Some(crate::rig_props::Adj { off: [-0.006, 0.048, 0.032], rot: [-17.2, 0.0, 0.7] }), housing_adj: None, lens_adj: None, follow_pitch: 1.0 }, pistol: crate::rig_props::RigPistol { slide: crate::rig_props::Seg { l0: 0.02, l1: 0.21, thick: 0.016, tall: 0.026, adj: None }, suppressor: crate::rig_props::Seg { l0: 0.21, l1: 0.36, thick: 0.019, tall: 0.019, adj: None }, grip: crate::rig_props::Seg { l0: 0.0, l1: 0.095, thick: 0.014, tall: 0.017, adj: Some(crate::rig_props::Adj { off: [0.0, -0.024, 0.05], rot: [108.0, 0.0, 0.0] }) }, rail: crate::rig_props::RigPistolRail { l0: 0.09, l1: 0.2, thick: 0.013, tall: 0.012, adj: Some(crate::rig_props::Adj { off: [0.0, 0.0, 0.064], rot: [0.0, 0.0, 0.0] }), under: 0.034 }, lens: crate::rig_props::RigPistolLens { l0: 0.2, l1: 0.215, thick: 0.014, tall: 0.013, adj: None, glow: [14.0, 14.0, 13.0] }, light_under: 0.038, light_ahead: 0.2, adj: Some(crate::rig_props::Adj { off: [0.039, 0.08, 0.035], rot: [-1.3, 0.0, 0.0] }) }, torch: crate::rig_props::RigTorch { body: crate::rig_props::Seg { l0: -0.06, l1: 0.15, thick: 0.022, tall: 0.022, adj: None }, lens: crate::rig_props::Seg { l0: 0.15, l1: 0.18, thick: 0.026, tall: 0.026, adj: None }, lens_glow: 40.0, light_ahead: 0.18, adj: Some(crate::rig_props::Adj { off: [0.076, 0.04, 0.092], rot: [0.0, 6.7, 0.0] }) }, guard_pistol: crate::rig_props::RigGuardPistol { slide: crate::rig_props::Seg { l0: 0.02, l1: 0.2, thick: 0.016, tall: 0.026, adj: Some(crate::rig_props::Adj { off: [0.028, -0.034, 0.073], rot: [11.2, 0.0, 0.5] }) }, grip: crate::rig_props::Seg { l0: 0.0, l1: 0.095, thick: 0.014, tall: 0.017, adj: Some(crate::rig_props::Adj { off: [0.028, -0.06, 0.118], rot: [119.0, 0.0, 0.5] }) }, lens: crate::rig_props::Seg { l0: 0.2, l1: 0.23, thick: 0.015, tall: 0.015, adj: Some(crate::rig_props::Adj { off: [0.029, -0.064, 0.023], rot: [11.8, 0.0, 0.0] }) }, light_ahead: 0.25, adj: Some(crate::rig_props::Adj { off: [0.031, 0.105, -0.05], rot: [-8.6, 0.0, 0.0] }) }, knife: crate::rig_props::RigKnife { blade: crate::rig_props::Seg { l0: 0.045, l1: 0.215, thick: 0.002, tall: 0.009, adj: Some(crate::rig_props::Adj { off: [0.0, 0.003, 0.0], rot: [0.0, 0.0, 0.0] }) }, edge: crate::rig_props::Seg { l0: 0.045, l1: 0.215, thick: 0.002, tall: 0.003, adj: Some(crate::rig_props::Adj { off: [0.0, -0.009, 0.0], rot: [0.0, 0.0, 0.0] }) }, guard: crate::rig_props::Seg { l0: 0.04, l1: 0.048, thick: 0.005, tall: 0.019, adj: None }, handle: crate::rig_props::Seg { l0: -0.065, l1: 0.045, thick: 0.011, tall: 0.013, adj: None }, grip: crate::rig_props::RigKnifeGrip { fwd: 0.08, palm: 0.03, side: 0.0 }, adj: None }, sheath: crate::rig_props::RigSheath { at: [0.07, -0.085, 0.2], along: [0.26, 1.0, -0.2], out: [0.0, 0.18, 1.0], body: crate::rig_props::Seg { l0: 0.046, l1: 0.228, thick: 0.006, tall: 0.016, adj: None } }, can: crate::rig_props::RigCan { sink: -0.02, tilt: 0.35, body: crate::rig_props::Seg { l0: -0.02, l1: 0.12, thick: 0.032, tall: 0.032, adj: None }, band: crate::rig_props::Seg { l0: 0.085, l1: 0.109, thick: 0.034, tall: 0.034, adj: None }, adj: Some(crate::rig_props::Adj { off: [0.0, 0.0, 0.0], rot: [0.0, 0.0, 0.0] }) }, holster: crate::rig_props::RigHolster { down: 0.06, out: 0.115, fwd: 0.01, adj: Some(crate::rig_props::Adj { off: [0.0, 0.0, 0.0], rot: [0.0, 0.0, 0.0] }) }, carbine: crate::rig_props::RigCarbine { receiver: crate::rig_props::Seg { l0: -0.09, l1: 0.155, thick: 0.017, tall: 0.032, adj: Some(crate::rig_props::Adj { off: [0.0, -0.012, 0.0], rot: [0.0, 0.0, 0.0] }) }, handguard: crate::rig_props::Seg { l0: 0.155, l1: 0.275, thick: 0.02, tall: 0.025, adj: Some(crate::rig_props::Adj { off: [0.0, -0.008, 0.0], rot: [0.0, 0.0, 0.0] }) }, barrel: crate::rig_props::Seg { l0: 0.275, l1: 0.34, thick: 0.007, tall: 0.007, adj: None }, muzzle: crate::rig_props::Seg { l0: 0.34, l1: 0.41, thick: 0.015, tall: 0.015, adj: None }, mag: crate::rig_props::Seg { l0: 0.0, l1: 0.11, thick: 0.013, tall: 0.028, adj: Some(crate::rig_props::Adj { off: [0.0, -0.04, 0.085], rot: [78.0, 0.0, 0.0] }) }, mag2: crate::rig_props::Seg { l0: 0.0, l1: 0.1, thick: 0.013, tall: 0.027, adj: Some(crate::rig_props::Adj { off: [0.0, -0.143, 0.106], rot: [60.0, 0.0, 0.0] }) }, grip: crate::rig_props::Seg { l0: 0.0, l1: 0.09, thick: 0.013, tall: 0.016, adj: Some(crate::rig_props::Adj { off: [0.0, -0.045, -0.015], rot: [108.0, 0.0, 0.0] }) }, stock_top: crate::rig_props::Seg { l0: -0.32, l1: -0.09, thick: 0.007, tall: 0.008, adj: Some(crate::rig_props::Adj { off: [0.0, 0.005, 0.0], rot: [0.0, 0.0, 0.0] }) }, stock_low: crate::rig_props::Seg { l0: 0.0, l1: 0.235, thick: 0.007, tall: 0.008, adj: Some(crate::rig_props::Adj { off: [0.0, -0.04, -0.09], rot: [171.0, 0.0, 0.0] }) }, light: crate::rig_props::Seg { l0: 0.18, l1: 0.265, thick: 0.011, tall: 0.011, adj: Some(crate::rig_props::Adj { off: [-0.032, -0.006, 0.0], rot: [0.0, 0.0, 0.0] }) }, lens: crate::rig_props::Seg { l0: 0.265, l1: 0.28, thick: 0.012, tall: 0.012, adj: Some(crate::rig_props::Adj { off: [-0.032, -0.006, 0.0], rot: [0.0, 0.0, 0.0] }) }, lens_glow: 40.0, light_ahead: 0.285, grip_l: crate::rig_props::RigCarbineGripL { off: [0.099, -0.059, 0.149], fingers: [-0.48, -0.11, 0.19], back: [0.45, -0.85, -0.25] }, adj: Some(crate::rig_props::Adj { off: [0.06, 0.08, 0.09], rot: [0.0, 0.0, 0.0] }) }, sc20k: crate::rig_props::RigSc20k { hull: crate::rig_props::Seg { l0: -0.29, l1: 0.12, thick: 0.024, tall: 0.04, adj: Some(crate::rig_props::Adj { off: [0.0, -0.012, 0.0], rot: [0.0, 0.0, 0.0] }) }, butt: crate::rig_props::Seg { l0: -0.31, l1: -0.29, thick: 0.026, tall: 0.048, adj: Some(crate::rig_props::Adj { off: [0.0, -0.018, 0.0], rot: [0.0, 0.0, 0.0] }) }, mag: crate::rig_props::Seg { l0: 0.0, l1: 0.13, thick: 0.012, tall: 0.03, adj: Some(crate::rig_props::Adj { off: [0.0, -0.045, -0.12], rot: [84.0, 0.0, 0.0] }) }, grip: crate::rig_props::Seg { l0: 0.0, l1: 0.09, thick: 0.013, tall: 0.016, adj: Some(crate::rig_props::Adj { off: [0.0, -0.045, -0.015], rot: [108.0, 0.0, 0.0] }) }, fore: crate::rig_props::Seg { l0: 0.12, l1: 0.3, thick: 0.022, tall: 0.03, adj: Some(crate::rig_props::Adj { off: [0.0, -0.014, 0.0], rot: [0.0, 0.0, 0.0] }) }, suppressor: crate::rig_props::Seg { l0: 0.3, l1: 0.43, thick: 0.019, tall: 0.019, adj: None }, rail: crate::rig_props::Seg { l0: -0.16, l1: 0.16, thick: 0.011, tall: 0.008, adj: Some(crate::rig_props::Adj { off: [0.0, 0.036, 0.0], rot: [0.0, 0.0, 0.0] }) }, optic: crate::rig_props::Seg { l0: -0.04, l1: 0.08, thick: 0.017, tall: 0.02, adj: Some(crate::rig_props::Adj { off: [0.0, 0.062, 0.0], rot: [0.0, 0.0, 0.0] }) }, foregrip: crate::rig_props::Seg { l0: 0.0, l1: 0.085, thick: 0.012, tall: 0.012, adj: Some(crate::rig_props::Adj { off: [0.0, -0.044, 0.2], rot: [90.0, 0.0, 0.0] }) }, light: crate::rig_props::Seg { l0: 0.19, l1: 0.27, thick: 0.011, tall: 0.011, adj: Some(crate::rig_props::Adj { off: [-0.034, -0.008, 0.0], rot: [0.0, 0.0, 0.0] }) }, lens: crate::rig_props::Seg { l0: 0.27, l1: 0.285, thick: 0.012, tall: 0.012, adj: Some(crate::rig_props::Adj { off: [-0.034, -0.008, 0.0], rot: [0.0, 0.0, 0.0] }) }, lens_glow: 40.0, light_ahead: 0.29, grip_l: crate::rig_props::RigSc20kGripL { off: [0.06, -0.085, 0.145], fingers: [-0.3, 0.05, 0.95], back: [0.95, -0.15, 0.25] }, slung: crate::rig_props::RigSc20kSlung { off: [-0.02, 0.17, -0.18], bore: [0.5, -0.866, 0.2], roll: 180.0, follow: 0.5, adj: None }, adj: Some(crate::rig_props::Adj { off: [0.06, 0.08, 0.09], rot: [0.0, 0.0, 0.0] }) }, m4: crate::rig_props::RigM4 { hull: crate::rig_props::Seg { l0: -0.075, l1: 0.115, thick: 0.017, tall: 0.036, adj: Some(crate::rig_props::Adj { off: [0.0, -0.012, 0.0], rot: [0.0, 0.0, 0.0] }) }, butt: crate::rig_props::Seg { l0: -0.31, l1: -0.19, thick: 0.017, tall: 0.036, adj: Some(crate::rig_props::Adj { off: [0.0, -0.02, 0.0], rot: [0.0, 0.0, 0.0] }) }, mag: crate::rig_props::Seg { l0: 0.0, l1: 0.14, thick: 0.011, tall: 0.03, adj: Some(crate::rig_props::Adj { off: [0.0, -0.045, 0.075], rot: [80.0, 0.0, 0.0] }) }, fore: crate::rig_props::Seg { l0: 0.115, l1: 0.3, thick: 0.022, tall: 0.022, adj: Some(crate::rig_props::Adj { off: [0.0, -0.003, 0.0], rot: [0.0, 0.0, 0.0] }) }, rail: crate::rig_props::Seg { l0: -0.07, l1: 0.3, thick: 0.009, tall: 0.005, adj: Some(crate::rig_props::Adj { off: [0.0, 0.027, 0.0], rot: [0.0, 0.0, 0.0] }) }, optic: crate::rig_props::Seg { l0: 0.0, l1: 0.07, thick: 0.016, tall: 0.02, adj: Some(crate::rig_props::Adj { off: [0.0, 0.052, 0.0], rot: [0.0, 0.0, 0.0] }) }, tube: crate::rig_props::Seg { l0: -0.19, l1: -0.075, thick: 0.012, tall: 0.012, adj: Some(crate::rig_props::Adj { off: [0.0, -0.002, 0.0], rot: [0.0, 0.0, 0.0] }) }, peq: crate::rig_props::Seg { l0: 0.215, l1: 0.29, thick: 0.017, tall: 0.011, adj: Some(crate::rig_props::Adj { off: [0.0, 0.043, 0.0], rot: [0.0, 0.0, 0.0] }) }, gl: crate::rig_props::Seg { l0: 0.14, l1: 0.3, thick: 0.02, tall: 0.024, adj: Some(crate::rig_props::Adj { off: [0.0, -0.05, 0.0], rot: [0.0, 0.0, 0.0] }) } } };

pub fn col() -> crate::rig_props::COL { crate::rig_props::COL { gun: [0.05, 0.05, 0.055], can: [0.09, 0.09, 0.095], torch: [0.16, 0.16, 0.17], lens_body: [0.3, 0.3, 0.3], goggles_housing: [0.04, 0.045, 0.04], blade: [0.11, 0.11, 0.12], edge: [0.34, 0.34, 0.36], knife_grip: [0.02, 0.02, 0.022], sheath: [0.028, 0.03, 0.026], furniture: [0.2, 0.085, 0.05], anodised: [0.03, 0.03, 0.033], fde: [0.2, 0.155, 0.1], dark_glass: [0.015, 0.02, 0.022], haft: [0.3, 0.22, 0.07], sledge: [0.04, 0.04, 0.045], brass: [0.5, 0.36, 0.12], shell: [0.26, 0.04, 0.03], cloth: [0.11, 0.1, 0.065], polymer: [0.045, 0.05, 0.046], dark_metal: [0.026, 0.026, 0.03] } }

pub const KNIFE_TAG: &str = "knife";

pub const SHEATH_TAG: &str = "sheath";

pub const SC20K_TAG: &str = "sc20k";

pub const PROP_FLAGS: f64 = 8.0;

pub const GLOW_FLAGS: f64 = 10.0;

pub fn no_glow() -> V3 { [0.0, 0.0, 0.0] }

pub const NV_SWING: f64 = 0.018;

pub fn gripfit() -> crate::rig_props::GRIPFIT { crate::rig_props::GRIPFIT { can: crate::rig_props::GRIPFITCan { fwd: 0.106, palm: 0.047, side: 0.0 }, pistol: [-0.006, 0.0, 0.023], long: [-0.031, 0.0, 0.037], fore: crate::rig_props::GRIPFITFore { back: 0.062, lift: 0.011, down: 0.015 }, mag: crate::rig_props::GRIPFITMag { fwd: 0.095, palm: 0.028, side: 0.03 }, guard_pistol: [-0.026, -0.005, 0.035], torch: crate::rig_props::GRIPFITTorch { fwd: 0.1, palm: 0.036 } } }

pub fn adj_none() -> crate::rig_props::Adj { crate::rig_props::Adj { off: [0.0, 0.0, 0.0], rot: [0.0, 0.0, 0.0] } }

pub const GUNMODEL_TAG: &str = "gunmodel";

pub const COVER_OPEN: f64 = 75.0;

pub fn grip_hung() -> crate::rig_props::Adj { crate::rig_props::Adj { off: [0.0, -0.024, 0.05], rot: [108.0, 0.0, 0.0] } }

pub fn sling_front() -> crate::rig_props::RigSc20kSlung { crate::rig_props::RigSc20kSlung { off: [-0.07, 0.0, 0.2], bore: [0.55, -0.82, 0.12], roll: 180.0, follow: 0.85, adj: None } }

pub fn sling_rest() -> crate::rig_props::SLINGREST { crate::rig_props::SLINGREST { pivot: 0.28, len: 0.7, clear: 0.11, max: 1.4, soft: 0.1, n: 6.0 } }

pub fn rail_lens_color() -> V3 { [0.9, 0.96, 1.0] }

pub const SLUNG_CAST_OUT: f64 = 0.03;

pub const BARE_BARREL: f64 = 0.008;

pub const TORCH_CARRY_RATE: f64 = 13.0;

pub static BIGRIG: crate::rig_props::Bigrig = crate::rig_props::Bigrig { hammer: crate::rig_props::BigrigHammer { handle: crate::rig_props::Seg { l0: -0.46, l1: 0.265, thick: 0.02, tall: 0.02, adj: None }, head: crate::rig_props::Seg { l0: 0.265, l1: 0.335, thick: 0.085, tall: 0.045, adj: None }, grip: crate::rig_props::BigrigHammerGrip { fwd: 0.08, palm: 0.03, side: 0.0 }, hands: 0.36, head_at: 0.3, back: crate::rig_props::RigSc20kSlung { off: [-0.055, 0.01, -0.19], bore: [-0.42, 0.91, 0.0], roll: 90.0, follow: 0.5, adj: None } }, shotgun: crate::rig_props::BigrigShotgun { back: crate::rig_props::RigSc20kSlung { off: [-0.02, 0.17, -0.18], bore: [0.5, -0.866, 0.2], roll: 180.0, follow: 0.5, adj: None } }, clear: crate::rig_props::BigrigClear { hammer: 0.11, shotgun: 0.075 } };

pub const BIG_TAG: &str = "big";
