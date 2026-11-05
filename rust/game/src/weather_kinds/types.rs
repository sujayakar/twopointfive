//! The types of `weather_kinds`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum WeatherKind {
    Clear,
    Overcast,
    Rain,
    Downpour,
    Fog,
    Dust,
    Snow,
}
impl WeatherKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            WeatherKind::Clear => "clear",
            WeatherKind::Overcast => "overcast",
            WeatherKind::Rain => "rain",
            WeatherKind::Downpour => "downpour",
            WeatherKind::Fog => "fog",
            WeatherKind::Dust => "dust",
            WeatherKind::Snow => "snow",
        }
    }
}

/// (TypeScript: FogSpec, src/game/weatherKinds.ts:19)
#[derive(Clone, Copy, Debug)]
pub struct FogSpec {
    pub sigma: f64,
    pub h: f64,
    pub leak: f64,
    pub tint: V3,
    pub gain: f64,
}
impl FogSpec {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        FogSpec {
            sigma: 0.0,
            h: 0.0,
            leak: 0.0,
            tint: [0.0; 3],
            gain: 0.0,
        }
    }
}
impl Default for FogSpec { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: WeatherPreset, src/game/weatherKinds.ts:25)
#[derive(Clone, Debug)]
pub struct WeatherPreset {
    pub title: String,
    pub words: String,
    pub storm: f64,
    pub rain: f64,
    pub wind: [f64; 2],
    pub cloud: f64,
    pub fog: Option<crate::weather_kinds::FogSpec>,
    pub snow: Option<f64>,
}
impl WeatherPreset {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WeatherPreset {
            title: String::new(),
            words: String::new(),
            storm: 0.0,
            rain: 0.0,
            wind: [0.0; 2],
            cloud: 0.0,
            fog: None,
            snow: None,
        }
    }
}
impl Default for WeatherPreset { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/weatherKinds.ts:159 WEATHER_BOOT, src/game/weatherKinds.ts:159)
#[derive(Clone, Copy, Debug)]
pub struct WeatherKindsModWEATHERBOOT {
    pub kind: Option<crate::weather_kinds::WeatherKind>,
    pub locked: bool,
    pub packed: bool,
}
impl WeatherKindsModWEATHERBOOT {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WeatherKindsModWEATHERBOOT {
            kind: None,
            locked: false,
            packed: false,
        }
    }
}
impl Default for WeatherKindsModWEATHERBOOT { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: a union)
#[derive(Clone, Debug)]
pub enum WeatherOfSearchSearch {
    Text(String),
    URLSearchParams(crate::Opaque),
}

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct WeatherKindsMod {
    /// (src/game/weatherKinds.ts:39 WEATHER_PRESETS)
    pub weather_presets: JsMap<crate::weather_kinds::WeatherKind, crate::weather_kinds::WeatherPreset>,
    /// (src/game/weatherKinds.ts:159 WEATHER_BOOT)
    pub weather_boot: crate::weather_kinds::WeatherKindsModWEATHERBOOT,
}
impl WeatherKindsMod {
    pub fn blank() -> Self {
        WeatherKindsMod {
            weather_presets: { let mut t = JsMap::new(); t.set(crate::weather_kinds::WeatherKind::Clear, crate::weather_kinds::WeatherPreset { title: String::from("Clear"), words: String::from("dry and still, so sound carries and nothing hides you but the dark"), storm: 0.0, rain: 0.0, wind: [0.35, 0.15], cloud: 0.05, fog: None, snow: None }); t.set(crate::weather_kinds::WeatherKind::Overcast, crate::weather_kinds::WeatherPreset { title: String::from("Overcast"), words: String::from("dry under a low sky. At night it's the same as clear"), storm: 0.0, rain: 0.0, wind: [0.8, 0.3], cloud: 0.9, fog: None, snow: None }); t.set(crate::weather_kinds::WeatherKind::Rain, crate::weather_kinds::WeatherPreset { title: String::from("Rain"), words: String::from("steady rain, so footsteps and doors carry about half as far outdoors"), storm: 0.25, rain: 0.55, wind: [1.1, 0.45], cloud: 0.95, fog: None, snow: None }); t.set(crate::weather_kinds::WeatherKind::Downpour, crate::weather_kinds::WeatherPreset { title: String::from("Downpour"), words: String::from("a tropical downpour with thunder. It drowns out small sounds, and lightning lights everyone up"), storm: 1.0, rain: 1.0, wind: [2.6, 1.0], cloud: 1.0, fog: Some(crate::weather_kinds::FogSpec { sigma: 0.02, h: 6.0, leak: 0.15, tint: [0.85, 0.9, 1.0], gain: 0.75 }), snow: None }); t.set(crate::weather_kinds::WeatherKind::Fog, crate::weather_kinds::WeatherPreset { title: String::from("Fog"), words: String::from("ground fog, so nobody can see past about twenty meters outdoors"), storm: 0.0, rain: 0.0, wind: [0.2, 0.1], cloud: 0.7, fog: Some(crate::weather_kinds::FogSpec { sigma: 0.113, h: 2.5, leak: 0.15, tint: [0.9, 0.95, 1.0], gain: 1.0 }), snow: None }); t.set(crate::weather_kinds::WeatherKind::Dust, crate::weather_kinds::WeatherPreset { title: String::from("Dust"), words: String::from("dry wind and dust in the air, so it's harder to see far and beams stand out"), storm: 0.0, rain: 0.0, wind: [3.2, 1.2], cloud: 0.25, fog: Some(crate::weather_kinds::FogSpec { sigma: 0.03, h: 8.0, leak: 0.3, tint: [1.0, 0.8, 0.55], gain: 1.0 }), snow: None }); t.set(crate::weather_kinds::WeatherKind::Snow, crate::weather_kinds::WeatherPreset { title: String::from("Snow"), words: String::from("snow falling on snow: the ground is bright, the air is quiet"), storm: 0.0, rain: 0.0, wind: [0.8, 0.3], cloud: 0.9, fog: None, snow: Some(1.48) }); t },
            weather_boot: crate::weather_kinds::WeatherKindsModWEATHERBOOT { kind: None, locked: false, packed: true },
        }
    }
}

/// (src/game/weatherKinds.ts:14 MAX_ROOFS)
pub const MAX_ROOFS: f64 = 4.0;

/// (src/game/weatherKinds.ts:17 WEATHER_KINDS: a table nothing writes)
pub fn weather_kinds() -> Vec<String> { alloc::vec![String::from("clear"), String::from("overcast"), String::from("rain"), String::from("downpour"), String::from("fog"), String::from("dust")] }

/// (src/game/weatherKinds.ts:106 SNOW_LEVELS: a table nothing writes)
pub fn snow_levels() -> Vec<String> { alloc::vec![String::from("pass"), String::from("tut-sky"), String::from("tut-team"), String::from("tut-quiet"), String::from("tut-road")] }

/// (src/game/weatherKinds.ts:144 ROOF_Q)
pub const ROOF_Q: f64 = 0.2;

/// (src/game/weatherKinds.ts:173 WEATHER_KEY)
pub const WEATHER_KEY: &str = "weather";
