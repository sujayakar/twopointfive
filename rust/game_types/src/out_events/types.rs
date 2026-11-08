//! The types of `out_events`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// what a rule did that the host has to show or play: pushed by the boundary functions, drained by the host after each step (bridge/out-dispatch.ts)
#[derive(Clone, Debug)]
pub enum OutEvent {
    CheckpointsKeep(crate::out_events::OutCheckpointsKeep),
    CheckpointsRequest(crate::out_events::OutCheckpointsRequest),
    DecalsAdd(crate::out_events::OutDecalsAdd),
    DecalsClear,
    DecalsTouch,
    GameAudioLikeFootstep(crate::out_events::OutGameAudioLikeFootstep),
    GameAudioLikePlay(crate::out_events::OutGameAudioLikePlay),
    GameAudioLikeRain(crate::out_events::OutGameAudioLikeRain),
    HostApiRestartPicture(crate::out_events::OutHostApiRestartPicture),
    MothsClear,
    ParticlesEmit(crate::out_events::OutParticlesEmit),
    ParticlesImpulse(crate::out_events::OutParticlesImpulse),
    ParticlesSetHeat(crate::out_events::OutParticlesSetHeat),
    ReachKeepChanged(crate::out_events::OutReachKeepChanged),
    SmokeSystemLikeClearAll,
    SmokeSystemLikeEmit(crate::out_events::OutSmokeSystemLikeEmit),
    SmokeSystemLikeKeepHot(crate::out_events::OutSmokeSystemLikeKeepHot),
    SmokeSystemLikeObstaclesChanged(crate::out_events::OutSmokeSystemLikeObstaclesChanged),
    SmokeSystemLikePush(crate::out_events::OutSmokeSystemLikePush),
    WallWearReset,
    WallWearRound(crate::out_events::OutWallWearRound),
    MessBurst(crate::out_events::OutMessBurst),
    MessOpened(crate::out_events::OutMessOpened),
    MessWall(crate::out_events::OutMessWall),
    MessEvicted(crate::out_events::OutMessEvicted),
    BangScorch(crate::out_events::OutBangScorch),
    BulletMark(crate::out_events::OutBulletMark),
    ChipMark(crate::out_events::OutChipMark),
    KickScuff(crate::out_events::OutKickScuff),
    ClearSnowBlast,
    ClearSnowMarks,
    ClearSnowTracks,
    Tap(crate::out_events::OutTap),
    WallFeel(crate::out_events::OutWallFeel),
    EndCard,
    SnowBlast(crate::out_events::OutSnowBlast),
    SmokeSystemLikeRound(crate::out_events::OutSmokeSystemLikeRound),
}

#[derive(Clone, Debug)]
pub struct OutCheckpointsKeep {
    pub label: String,
}

#[derive(Clone, Debug)]
pub struct OutCheckpointsRequest {
    pub why: String,
    pub label: String,
}

#[derive(Clone, Debug)]
pub struct OutDecalsAdd {
    pub kind: crate::render_decals::DecalKind,
    pub pos: V3,
    pub normal: V3,
    pub size: Option<f64>,
    pub opts: Option<crate::render_decals::DecalOpts>,
}

#[derive(Clone, Debug)]
pub struct OutGameAudioLikeFootstep {
    pub pos: V3,
    pub loudness: f64,
    pub is_player: bool,
    pub mat: Option<crate::floor::FloorMat>,
}

#[derive(Clone, Debug)]
pub struct OutGameAudioLikePlay {
    pub name: String,
    pub pos: Option<V3>,
    pub gain: Option<f64>,
    pub opts: Option<crate::out_events::OutPlayOpts>,
}

#[derive(Clone, Debug)]
pub struct OutGameAudioLikeRain {
    pub level: f64,
}

#[derive(Clone, Debug)]
pub struct OutHostApiRestartPicture {
    pub why: String,
}

#[derive(Clone, Debug)]
pub struct OutParticlesEmit {
    pub kind: crate::render_particles::PKind,
    pub pos: V3,
    pub dir: V3,
    pub count: f64,
    pub o: crate::render_particles::EmitOpts,
    pub ours: bool,
}

#[derive(Clone, Debug)]
pub struct OutParticlesImpulse {
    pub pos: V3,
    pub radius: f64,
    pub strength: f64,
    pub o: Option<crate::render_particles::ImpulseOpts>,
}

#[derive(Clone, Debug)]
pub struct OutParticlesSetHeat {
    pub srcs: Vec<crate::smoke_types::WindGpuSource>,
}

#[derive(Clone, Debug)]
pub struct OutReachKeepChanged {
    pub rect: Option<[f64; 4]>,
    pub hold: Option<f64>,
}

#[derive(Clone, Debug)]
pub struct OutSmokeSystemLikeEmit {
    pub e: crate::smoke_types::SmokeEmitter,
}

#[derive(Clone, Debug)]
pub struct OutSmokeSystemLikeKeepHot {
    pub heat: f64,
}

#[derive(Clone, Debug)]
pub struct OutSmokeSystemLikeObstaclesChanged {
    pub c_2: V3,
    pub r: f64,
}

#[derive(Clone, Debug)]
pub struct OutSmokeSystemLikePush {
    pub p: crate::smoke_types::SmokePush,
}

#[derive(Clone, Debug, Default)]
pub struct OutSmokeSystemLikeRound {
    pub from: V3,
    pub dir: V3,
    pub len: f64,
}

#[derive(Clone, Debug)]
pub struct OutWallWearRound {
    pub box_: BoxId,
    pub surf: crate::scene_materials::Surf,
    pub key: f64,
    pub counted: f64,
    pub frac: f64,
    pub hit_p: V3,
    pub dir: V3,
}


#[derive(Clone, Debug)]
pub struct OutMessBurst {
    pub at: V3,
    pub size: Option<f64>,
}

#[derive(Clone, Debug)]
pub struct OutMessOpened {
    pub rect: [f64; 4],
}

/// A resting piece the debris pool took: what the host needs to lay its mark, by value, since its slot is taken before the host reads this.
#[derive(Clone, Debug)]
pub struct OutMessEvicted {
    pub c: V3,
    pub h: V3,
    pub yaw: f64,
    pub rot: Option<Quat>,
    pub mat: Option<crate::break_table::Mat>,
    pub kind: crate::debris::ChunkKind,
}

#[derive(Clone, Debug)]
pub struct OutMessWall {
    pub p: V3,
    pub n: V3,
    pub thick: f64,
    pub through: bool,
}

#[derive(Clone, Debug)]
pub struct OutBangScorch {
    pub at: V3,
}

#[derive(Clone, Debug)]
pub struct OutBulletMark {
    pub hit_p: V3,
    pub n: V3,
    pub s: Option<crate::marks::Surface2>,
    pub def: crate::weapons::WeaponId,
}

#[derive(Clone, Debug)]
pub struct OutChipMark {
    pub p: V3,
    pub n: V3,
    pub s: Option<crate::marks::Surface2>,
    pub size: Option<f64>,
}

#[derive(Clone, Debug)]
pub struct OutKickScuff {
    pub d: DoorId,
    pub from: V3,
}

#[derive(Clone, Debug)]
pub struct OutTap {
    pub row: u32,
    pub args: Vec<u8>,
    pub answer: Vec<u8>,
}

#[derive(Clone, Debug)]
pub struct OutWallFeel {
    pub at: V3,
}

#[derive(Clone, Debug, Default)]
pub struct OutSnowBlast {
    pub at: V3,
    pub radius: f64,
}

/// (audio_iface.ts PlayOpts: an object with some of these)
#[derive(Clone, Debug, Default)]
pub struct OutPlayOpts {
    pub rate: Option<f64>,
    pub duration: Option<f64>,
    pub lowpass: Option<f64>,
    pub pan: Option<f64>,
    pub wet: Option<f64>,
    pub tok: Option<f64>,
}

