//! The types of `render_particles`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum PKind {
    Rain,
    Dust,
    Casing,
    Sheet,
    Spark,
    Debris,
    Shard,
}
impl PKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            PKind::Rain => "rain",
            PKind::Dust => "dust",
            PKind::Casing => "casing",
            PKind::Sheet => "sheet",
            PKind::Spark => "spark",
            PKind::Debris => "debris",
            PKind::Shard => "shard",
        }
    }
}

/// (TypeScript: RainLook, src/render/particles.ts:73)
#[derive(Clone, Copy, Debug)]
pub struct RainLook {
    pub iso: f64,
    pub fwd_k: f64,
    pub fwd_g: f64,
    pub glint_k: f64,
    pub glint_rough: f64,
    pub glint_cap: f64,
    pub gain: f64,
    pub tail_keep: f64,
    pub refract: f64,
    pub refract_reach_m: f64,
    pub refract_taps: f64,
    pub refract_clamp: f64,
    pub refract_depth: f64,
    pub refract_paint: f64,
    pub len_jit: f64,
    pub bright_jit: f64,
    pub min_px: f64,
    pub max_px: f64,
    pub boost_cap: f64,
    pub near_fade_m: f64,
    pub bead_amp: f64,
    pub wind_k: f64,
    pub sheet_amp: f64,
    pub sheet_len: f64,
    pub band_off_x: f64,
    pub band_off_z: f64,
}
impl RainLook {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RainLook {
            iso: 0.0,
            fwd_k: 0.0,
            fwd_g: 0.0,
            glint_k: 0.0,
            glint_rough: 0.0,
            glint_cap: 0.0,
            gain: 0.0,
            tail_keep: 0.0,
            refract: 0.0,
            refract_reach_m: 0.0,
            refract_taps: 0.0,
            refract_clamp: 0.0,
            refract_depth: 0.0,
            refract_paint: 0.0,
            len_jit: 0.0,
            bright_jit: 0.0,
            min_px: 0.0,
            max_px: 0.0,
            boost_cap: 0.0,
            near_fade_m: 0.0,
            bead_amp: 0.0,
            wind_k: 0.0,
            sheet_amp: 0.0,
            sheet_len: 0.0,
            band_off_x: 0.0,
            band_off_z: 0.0,
        }
    }
}
impl Default for RainLook { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: KindParams, src/render/particles.ts:85)
#[derive(Clone, Copy, Debug)]
pub struct KindParams {
    pub gravity: f64,
    pub drag: f64,
    pub restitution: f64,
    pub keep: f64,
    pub sleep_v: f64,
    pub hold_or_fade: f64,
    pub shutter_or_growth: f64,
    pub bright_or_opacity: f64,
}
impl KindParams {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        KindParams {
            gravity: 0.0,
            drag: 0.0,
            restitution: 0.0,
            keep: 0.0,
            sleep_v: 0.0,
            hold_or_fade: 0.0,
            shutter_or_growth: 0.0,
            bright_or_opacity: 0.0,
        }
    }
}
impl Default for KindParams { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: EmitOpts, src/render/particles.ts:104)
#[derive(Clone, Copy, Debug)]
pub struct EmitOpts {
    pub speed: f64,
    pub life: f64,
    pub size: f64,
    pub lift: Option<f64>,
    pub pos_jit: Option<f64>,
    pub vadd: Option<V3>,
    pub seed: Option<f64>,
}
impl EmitOpts {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        EmitOpts {
            speed: 0.0,
            life: 0.0,
            size: 0.0,
            lift: None,
            pos_jit: None,
            vadd: None,
            seed: None,
        }
    }
}
impl Default for EmitOpts { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: EmitRec, src/render/particles.ts:111)
#[derive(Clone, Copy, Debug)]
pub struct EmitRec {
    pub kind: crate::render_particles::PKind,
    pub count: f64,
}
impl EmitRec {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        EmitRec {
            kind: crate::render_particles::PKind::Rain,
            count: 0.0,
        }
    }
}
impl Default for EmitRec { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: ImpulseOpts, src/render/particles.ts:115)
#[derive(Clone, Debug)]
pub struct ImpulseOpts {
    pub up: Option<f64>,
    pub bias: Option<V3>,
    pub kinds: Option<Vec<crate::render_particles::PKind>>,
    pub occlude: Option<bool>,
    pub eye: Option<V3>,
    pub seed: Option<f64>,
}
impl ImpulseOpts {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ImpulseOpts {
            up: None,
            bias: None,
            kinds: None,
            occlude: None,
            eye: None,
            seed: None,
        }
    }
}
impl Default for ImpulseOpts { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: ImpRec, src/render/particles.ts:117)
#[derive(Clone, Copy, Debug)]
pub struct ImpRec {
    pub pos: V3,
    pub radius: f64,
    pub strength: f64,
    pub up: f64,
    pub bias: V3,
    pub eye: V3,
    pub kinds: f64,
    pub flags: f64,
    pub seed: f64,
}
impl ImpRec {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ImpRec {
            pos: [0.0; 3],
            radius: 0.0,
            strength: 0.0,
            up: 0.0,
            bias: [0.0; 3],
            eye: [0.0; 3],
            kinds: 0.0,
            flags: 0.0,
            seed: 0.0,
        }
    }
}
impl Default for ImpRec { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: ParticleStats, src/render/particles.ts:133)
#[derive(Clone, Copy, Debug)]
pub struct ParticleStats {
    pub kicked: f64,
    pub wind_woken: f64,
    pub winds: f64,
    pub dropped_recs: f64,
    pub dropped_births: f64,
    pub imp_queued: f64,
    pub imp_merged: f64,
    pub imp_dropped: f64,
    pub imp_refused: f64,
}
impl ParticleStats {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ParticleStats {
            kicked: 0.0,
            wind_woken: 0.0,
            winds: 0.0,
            dropped_recs: 0.0,
            dropped_births: 0.0,
            imp_queued: 0.0,
            imp_merged: 0.0,
            imp_dropped: 0.0,
            imp_refused: 0.0,
        }
    }
}
impl Default for ParticleStats { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: RainStats, src/render/particles.ts:145)
#[derive(Clone, Copy, Debug)]
pub struct RainStats {
    pub live: f64,
    pub born: f64,
    pub records: f64,
    pub dropped_recs: f64,
    pub dropped_births: f64,
    pub emitted: f64,
}
impl RainStats {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RainStats {
            live: 0.0,
            born: 0.0,
            records: 0.0,
            dropped_recs: 0.0,
            dropped_births: 0.0,
            emitted: 0.0,
        }
    }
}
impl Default for RainStats { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Particles, src/render/particles.ts:154)
#[derive(Clone, Debug)]
pub struct Particles {
    pub params: JsMap<crate::render_particles::PKind, crate::render_particles::KindParams>,
    pub shard_rest_frost: bool,
    pub collide: bool,
    pub spark_bounce: bool,
    pub stats: crate::render_particles::ParticleStats,
    pub winds: Vec<crate::smoke_types::WindGpuSource>,
    pub wind_knobs: [f64; 3],
    pub shed_sparks: f64,
    pub queue: Vec<crate::render_particles::EmitRec>,
    pub serial: f64,
    pub imps: Vec<crate::render_particles::ImpRec>,
    pub imp_n: f64,
    pub imp_serial: f64,
    pub head_mono: Vec<f64>,
    pub tail_mono: Vec<f64>,
    pub fifo: Vec<Vec<crate::render_particles::ParticlesFifoItemItem>>,
    pub rain_queue: Vec<crate::render_particles::EmitRec>,
    pub rain_head: f64,
    pub rain_tail: f64,
    pub rain_fifo: Vec<crate::render_particles::ParticlesRainFifoItem>,
    pub rain_win: [f64; 4],
    pub rain_stats: crate::render_particles::RainStats,
    pub rain_look: crate::render_particles::RainLook,
    pub rain_scene_copy: crate::render_particles::ParticlesRainSceneCopy,
    pub scene_dummy: crate::Opaque,
    pub scene_dummy_view: crate::Opaque,
    pub scene_samp: crate::Opaque,
    pub grain_cfg: Vec<f32>,
    pub grain_buf: crate::Opaque,
    pub born: f64,
    pub live_total: f64,
    pub pool: crate::Opaque,
    pub emit_buf: crate::Opaque,
    pub l_sim: crate::Opaque,
    pub l_draw: crate::Opaque,
    pub sim_mod: crate::Opaque,
    pub draw_mod: crate::Opaque,
    pub sim_lay: crate::Opaque,
    pub draw_lay: crate::Opaque,
    pub place_none: crate::Opaque,
    pub device: crate::Opaque,
    pub ours: bool,
    pub heat: Vec<crate::smoke_types::WindGpuSource>,
}
impl Particles {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
    pub fn blank() -> Self {
        Particles {
            params: { let mut t = JsMap::new(); t.set(crate::render_particles::PKind::Spark, crate::render_particles::KindParams { gravity: 16.0, drag: 1.6, restitution: 0.25, keep: 0.7, sleep_v: 0.0, hold_or_fade: 0.15, shutter_or_growth: 0.017, bright_or_opacity: 2.6 }); t.set(crate::render_particles::PKind::Debris, crate::render_particles::KindParams { gravity: 9.8, drag: 0.9, restitution: 0.3, keep: 0.55, sleep_v: 0.35, hold_or_fade: 3.0, shutter_or_growth: 0.008, bright_or_opacity: 1.0 }); t.set(crate::render_particles::PKind::Dust, crate::render_particles::KindParams { gravity: 0.35, drag: 6.0, restitution: 0.5, keep: 1.0, sleep_v: 0.05, hold_or_fade: 1.3, shutter_or_growth: 0.6, bright_or_opacity: 0.3 }); t.set(crate::render_particles::PKind::Casing, crate::render_particles::KindParams { gravity: 9.8, drag: 0.45, restitution: 0.42, keep: 0.72, sleep_v: 0.4, hold_or_fade: 4.0, shutter_or_growth: 0.008, bright_or_opacity: 1.0 }); t.set(crate::render_particles::PKind::Sheet, crate::render_particles::KindParams { gravity: 9.8, drag: 0.5, restitution: 1.5, keep: 9.0, sleep_v: 0.28, hold_or_fade: 4.0, shutter_or_growth: 25.0, bright_or_opacity: 0.3 }); t.set(crate::render_particles::PKind::Rain, crate::render_particles::KindParams { gravity: 9.8, drag: 1.088888888888889, restitution: 6.0, keep: 0.06, sleep_v: 0.2, hold_or_fade: 0.09, shutter_or_growth: 0.03333333333333333, bright_or_opacity: 1.0 }); t.set(crate::render_particles::PKind::Shard, crate::render_particles::KindParams { gravity: 9.8, drag: 0.5, restitution: 0.4, keep: 0.78, sleep_v: 0.3, hold_or_fade: 5.0, shutter_or_growth: 0.07, bright_or_opacity: 0.1 }); t },
            shard_rest_frost: true,
            collide: true,
            spark_bounce: true,
            stats: crate::render_particles::ParticleStats { kicked: 0.0, wind_woken: 0.0, winds: 0.0, dropped_recs: 0.0, dropped_births: 0.0, imp_queued: 0.0, imp_merged: 0.0, imp_dropped: 0.0, imp_refused: 0.0 },
            winds: Vec::new(),
            wind_knobs: [0.35, 2.0, 0.15],
            shed_sparks: 0.0,
            queue: Vec::new(),
            serial: 1.0,
            imps: alloc::vec![crate::render_particles::ImpRec { pos: [0.0, 0.0, 0.0], radius: 0.0, strength: 0.0, up: 0.0, bias: [0.0, 0.0, 0.0], eye: [0.0, 0.0, 0.0], kinds: 0.0, flags: 0.0, seed: 0.0 }; 16],
            imp_n: 0.0,
            imp_serial: 1.0,
            head_mono: alloc::vec![0.0, 0.0],
            tail_mono: alloc::vec![0.0, 0.0],
            fifo: alloc::vec![Vec::new(), Vec::new()],
            rain_queue: Vec::new(),
            rain_head: 0.0,
            rain_tail: 0.0,
            rain_fifo: Vec::new(),
            rain_win: [16384.0, 16384.0, 0.0, 0.0],
            rain_stats: crate::render_particles::RainStats { live: 0.0, born: 0.0, records: 0.0, dropped_recs: 0.0, dropped_births: 0.0, emitted: 0.0 },
            rain_look: crate::render_particles::RainLook::zeroed(),
            rain_scene_copy: crate::render_particles::ParticlesRainSceneCopy::None,
            scene_dummy: crate::Opaque,
            scene_dummy_view: crate::Opaque,
            scene_samp: crate::Opaque,
            grain_cfg: alloc::vec![0.0; 28],
            grain_buf: crate::Opaque,
            born: 0.0,
            live_total: 0.0,
            pool: crate::Opaque,
            emit_buf: crate::Opaque,
            l_sim: crate::Opaque,
            l_draw: crate::Opaque,
            sim_mod: crate::Opaque,
            draw_mod: crate::Opaque,
            sim_lay: crate::Opaque,
            draw_lay: crate::Opaque,
            place_none: crate::Opaque,
            device: crate::Opaque,
            ours: false,
            heat: Vec::new(),
        }
    }
}
impl Default for Particles { fn default() -> Self { Self::blank() } }

/// (TypeScript: inline at src/render/particles.ts:193 Particles.fifo, src/render/particles.ts:193)
#[derive(Clone, Copy, Debug)]
pub struct ParticlesFifoItemItem {
    pub expire: f64,
    pub end: f64,
}
impl ParticlesFifoItemItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ParticlesFifoItemItem {
            expire: 0.0,
            end: 0.0,
        }
    }
}
impl Default for ParticlesFifoItemItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/render/particles.ts:195 Particles.rainFifo, src/render/particles.ts:195)
#[derive(Clone, Copy, Debug)]
pub struct ParticlesRainFifoItem {
    pub expire: f64,
    pub end: f64,
}
impl ParticlesRainFifoItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ParticlesRainFifoItem {
            expire: 0.0,
            end: 0.0,
        }
    }
}
impl Default for ParticlesRainFifoItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum ParticlesRainSceneCopy {
    None,
    Gbaa,
    Copy,
}
impl ParticlesRainSceneCopy {
    pub const fn as_str(self) -> &'static str {
        match self {
            ParticlesRainSceneCopy::None => "none",
            ParticlesRainSceneCopy::Gbaa => "gbaa",
            ParticlesRainSceneCopy::Copy => "copy",
        }
    }
}

/// (TypeScript: inline at src/render/particles.ts:47 RAIN_RING, src/render/particles.ts:47)
#[derive(Clone, Copy, Debug)]
pub struct RAINRING {
    pub start: f64,
    pub size: f64,
}
impl RAINRING {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RAINRING {
            start: 0.0,
            size: 0.0,
        }
    }
}
impl Default for RAINRING { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/render/particles.ts:74 RAIN_LOOK_DEFAULTS, ../.npm-global/lib/node_modules/typescript/lib/lib.es5.d.ts:1597)
#[derive(Clone, Copy, Debug)]
pub struct RAINLOOKDEFAULTS {
    pub iso: f64,
    pub fwd_k: f64,
    pub fwd_g: f64,
    pub glint_k: f64,
    pub glint_rough: f64,
    pub glint_cap: f64,
    pub gain: f64,
    pub tail_keep: f64,
    pub refract: f64,
    pub refract_reach_m: f64,
    pub refract_taps: f64,
    pub refract_clamp: f64,
    pub refract_depth: f64,
    pub refract_paint: f64,
    pub len_jit: f64,
    pub bright_jit: f64,
    pub min_px: f64,
    pub max_px: f64,
    pub boost_cap: f64,
    pub near_fade_m: f64,
    pub bead_amp: f64,
    pub wind_k: f64,
    pub sheet_amp: f64,
    pub sheet_len: f64,
    pub band_off_x: f64,
    pub band_off_z: f64,
}
impl RAINLOOKDEFAULTS {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RAINLOOKDEFAULTS {
            iso: 0.0,
            fwd_k: 0.0,
            fwd_g: 0.0,
            glint_k: 0.0,
            glint_rough: 0.0,
            glint_cap: 0.0,
            gain: 0.0,
            tail_keep: 0.0,
            refract: 0.0,
            refract_reach_m: 0.0,
            refract_taps: 0.0,
            refract_clamp: 0.0,
            refract_depth: 0.0,
            refract_paint: 0.0,
            len_jit: 0.0,
            bright_jit: 0.0,
            min_px: 0.0,
            max_px: 0.0,
            boost_cap: 0.0,
            near_fade_m: 0.0,
            bead_amp: 0.0,
            wind_k: 0.0,
            sheet_amp: 0.0,
            sheet_len: 0.0,
            band_off_x: 0.0,
            band_off_z: 0.0,
        }
    }
}
impl Default for RAINLOOKDEFAULTS { fn default() -> Self { Self::zeroed() } }

/// (src/render/particles.ts:29 particlesSrc: a table nothing writes)
pub fn particles_src() -> String { todo!() }

/// (src/render/particles.ts:36 P_KINDS: a table nothing writes)
pub fn p_kinds() -> Vec<String> { alloc::vec![String::from("spark"), String::from("debris"), String::from("dust"), String::from("casing"), String::from("sheet"), String::from("rain"), String::from("shard")] }

/// (src/render/particles.ts:37 KIND_ID: a table nothing writes)
pub fn kind_id() -> JsMap<String, f64> { { let mut t = JsMap::new(); t.set(String::from("spark"), 0.0); t.set(String::from("debris"), 1.0); t.set(String::from("dust"), 2.0); t.set(String::from("casing"), 3.0); t.set(String::from("sheet"), 4.0); t.set(String::from("rain"), 5.0); t.set(String::from("shard"), 6.0); t } }

/// (src/render/particles.ts:41 P_CAP)
pub const P_CAP: f64 = 16384.0;

/// (src/render/particles.ts:46 RAIN_CAP)
pub const RAIN_CAP: f64 = 16384.0;

/// (src/render/particles.ts:47 RAIN_RING: a table nothing writes)
pub fn rain_ring() -> crate::render_particles::RAINRING { crate::render_particles::RAINRING { start: 16384.0, size: 16384.0 } }

/// (src/render/particles.ts:49 RAIN_MAX_BIRTHS)
pub const RAIN_MAX_BIRTHS: f64 = 640.0;

/// (src/render/particles.ts:51 RAIN_MAX_RECS)
pub const RAIN_MAX_RECS: f64 = 12.0;

/// (src/render/particles.ts:52 MAX_EMITS)
pub const MAX_EMITS: f64 = 128.0;

/// (src/render/particles.ts:53 MAX_NEW_PER_FRAME)
pub const MAX_NEW_PER_FRAME: f64 = 6144.0;

/// (src/render/particles.ts:57 MAX_QUEUE)
pub const MAX_QUEUE: f64 = 512.0;

/// (src/render/particles.ts:62 MAX_IMPULSES)
pub const MAX_IMPULSES: f64 = 16.0;

/// (src/render/particles.ts:62 IMPULSE_MERGE_M)
pub const IMPULSE_MERGE_M: f64 = 1.0;

/// (src/render/particles.ts:64 IMPULSE_MERGE_CAP)
pub const IMPULSE_MERGE_CAP: f64 = 2.0;

/// (src/render/particles.ts:65 P_BYTES)
pub const P_BYTES: f64 = 80.0;

/// (src/render/particles.ts:65 HDR_BYTES)
pub const HDR_BYTES: f64 = 64.0;

/// (src/render/particles.ts:69 REC_BYTES)
pub const REC_BYTES: f64 = 112.0;

/// (src/render/particles.ts:69 BLOCK_HDR_BYTES)
pub const BLOCK_HDR_BYTES: f64 = 384.0;

/// (src/render/particles.ts:69 IMP_BYTES)
pub const IMP_BYTES: f64 = 64.0;

/// (src/render/particles.ts:69 IMP_OFFSET)
pub const IMP_OFFSET: f64 = 14720.0;

/// (src/render/particles.ts:69 BLOCK_TAIL_AT)
pub const BLOCK_TAIL_AT: f64 = 15744.0;

/// (src/render/particles.ts:69 BLOCK_TAIL_BYTES)
pub const BLOCK_TAIL_BYTES: f64 = 112.0;

/// (src/render/particles.ts:69 BLOCK_BYTES)
pub const BLOCK_BYTES: f64 = 15856.0;

/// (src/render/particles.ts:74 RAIN_LOOK_DEFAULTS: a table nothing writes)
pub fn rain_look_defaults() -> crate::render_particles::RAINLOOKDEFAULTS { todo!() }

/// (src/render/particles.ts:86 KIND_DEFAULTS: a table nothing writes)
pub fn kind_defaults() -> JsMap<String, crate::render_particles::KindParams> { { let mut t = JsMap::new(); t.set(String::from("spark"), crate::render_particles::KindParams { gravity: 16.0, drag: 1.6, restitution: 0.25, keep: 0.7, sleep_v: 0.0, hold_or_fade: 0.15, shutter_or_growth: 0.017, bright_or_opacity: 2.6 }); t.set(String::from("debris"), crate::render_particles::KindParams { gravity: 9.8, drag: 0.9, restitution: 0.3, keep: 0.55, sleep_v: 0.35, hold_or_fade: 3.0, shutter_or_growth: 0.008, bright_or_opacity: 1.0 }); t.set(String::from("dust"), crate::render_particles::KindParams { gravity: 0.35, drag: 6.0, restitution: 0.5, keep: 1.0, sleep_v: 0.05, hold_or_fade: 1.3, shutter_or_growth: 0.6, bright_or_opacity: 0.3 }); t.set(String::from("casing"), crate::render_particles::KindParams { gravity: 9.8, drag: 0.45, restitution: 0.42, keep: 0.72, sleep_v: 0.4, hold_or_fade: 4.0, shutter_or_growth: 0.008, bright_or_opacity: 1.0 }); t.set(String::from("sheet"), crate::render_particles::KindParams { gravity: 9.8, drag: 0.5, restitution: 1.5, keep: 9.0, sleep_v: 0.28, hold_or_fade: 4.0, shutter_or_growth: 25.0, bright_or_opacity: 0.3 }); t.set(String::from("rain"), crate::render_particles::KindParams { gravity: 9.8, drag: 1.088888888888889, restitution: 6.0, keep: 0.06, sleep_v: 0.2, hold_or_fade: 0.09, shutter_or_growth: 0.03333333333333333, bright_or_opacity: 1.0 }); t.set(String::from("shard"), crate::render_particles::KindParams { gravity: 9.8, drag: 0.5, restitution: 0.4, keep: 0.78, sleep_v: 0.3, hold_or_fade: 5.0, shutter_or_growth: 0.07, bright_or_opacity: 0.1 }); t } }

/// (src/render/particles.ts:118 KINDS_DEFAULT)
pub const KINDS_DEFAULT: f64 = 94.0;

/// (src/render/particles.ts:121 NO_OPTS: a table nothing writes)
pub fn no_opts() -> crate::render_particles::ImpulseOpts { todo!() }

/// (src/render/particles.ts:121 ZERO3: a table nothing writes)
pub fn zero3() -> V3 { [0.0, 0.0, 0.0] }
