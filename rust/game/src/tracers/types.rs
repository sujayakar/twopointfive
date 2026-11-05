//! The types of `tracers`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: Tracers, src/game/tracers.ts:58)
#[derive(Clone, Debug)]
pub struct Tracers {
    pub pool: Vec<crate::tracers::Tracer>,
    pub live: f64,
    pub lit: f64,
    pub spawned: f64,
    pub seed: f64,
    pub lights: Option<crate::render_lights::LightSet>,
    pub params: crate::tracers::TracersParams,
}
impl Tracers {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
    pub fn blank() -> Self {
        Tracers {
            pool: Vec::new(),
            live: 0.0,
            lit: 0.0,
            spawned: 0.0,
            seed: 1.0,
            lights: None,
            params: crate::tracers::TracersParams { speed: 280.0, min_life: 0.042, shutter: 0.011, thickness: 0.012, emissive: 60.0, min_path: 0.5, light_intensity: 4.0, light_range: 4.0, light_radius: 0.06, ember_sparks: 3.0, ember_speed: 5.0, ember_cone: 0.9, ember_life: 0.3, ember_intensity: 2.5, ember_range: 2.5, ember_ttl: 0.06, max_live: 24.0, max_lights: 6.0 },
        }
    }
}
impl Default for Tracers { fn default() -> Self { Self::blank() } }

/// (TypeScript: Tracer, src/game/tracers.ts:39)
#[derive(Clone, Copy, Debug)]
pub struct Tracer {
    pub alive: bool,
    pub from: V3,
    pub dir: V3,
    pub l: f64,
    pub s: f64,
    pub v: f64,
    pub age: f64,
    pub frames: f64,
    pub color: V3,
    pub owner: f64,
    pub impact: crate::tracers::TracerImpact,
    pub arrived: bool,
    pub light: Option<LightId>,
    pub box_: BoxId,
    pub lamp: LightId,
}
impl Tracer {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Tracer {
            alive: false,
            from: [0.0; 3],
            dir: [0.0; 3],
            l: 0.0,
            s: 0.0,
            v: 0.0,
            age: 0.0,
            frames: 0.0,
            color: [0.0; 3],
            owner: 0.0,
            impact: crate::tracers::TracerImpact::None,
            arrived: false,
            light: None,
            box_: BoxId(0),
            lamp: LightId(0),
        }
    }
}
impl Default for Tracer { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum TracerImpact {
    None,
    Solid,
    Flesh,
}
impl TracerImpact {
    pub const fn as_str(self) -> &'static str {
        match self {
            TracerImpact::None => "none",
            TracerImpact::Solid => "solid",
            TracerImpact::Flesh => "flesh",
        }
    }
}

/// (TypeScript: inline at src/game/tracers.ts:69 Tracers.params, src/game/effects.ts:156)
#[derive(Clone, Copy, Debug)]
pub struct TracersParams {
    pub speed: f64,
    pub min_life: f64,
    pub shutter: f64,
    pub thickness: f64,
    pub emissive: f64,
    pub min_path: f64,
    pub light_intensity: f64,
    pub light_range: f64,
    pub light_radius: f64,
    pub ember_sparks: f64,
    pub ember_speed: f64,
    pub ember_cone: f64,
    pub ember_life: f64,
    pub ember_intensity: f64,
    pub ember_range: f64,
    pub ember_ttl: f64,
    pub max_live: f64,
    pub max_lights: f64,
}
impl TracersParams {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        TracersParams {
            speed: 0.0,
            min_life: 0.0,
            shutter: 0.0,
            thickness: 0.0,
            emissive: 0.0,
            min_path: 0.0,
            light_intensity: 0.0,
            light_range: 0.0,
            light_radius: 0.0,
            ember_sparks: 0.0,
            ember_speed: 0.0,
            ember_cone: 0.0,
            ember_life: 0.0,
            ember_intensity: 0.0,
            ember_range: 0.0,
            ember_ttl: 0.0,
            max_live: 0.0,
            max_lights: 0.0,
        }
    }
}
impl Default for TracersParams { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: SparkSink, src/game/tracers.ts:20)
#[derive(Clone, Copy, Debug)]
pub struct SparkSink {
}
impl SparkSink {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SparkSink {
        }
    }
}
impl Default for SparkSink { fn default() -> Self { Self::zeroed() } }

/// (src/game/tracers.ts:56 LAMP_TTL)
pub const LAMP_TTL: f64 = 30.0;
