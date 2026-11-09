//! The types of `weather_kinds`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, ToBytes)]
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

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct FogSpec {
    pub sigma: f64,
    pub h: f64,
    pub leak: f64,
    pub tint: V3,
    pub gain: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
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

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes)]
pub struct WeatherKindsMod {
    pub weather_presets: JsMap<crate::weather_kinds::WeatherKind, crate::weather_kinds::WeatherPreset>,
}
impl WeatherKindsMod {
    pub fn blank() -> Self {
        WeatherKindsMod {
            weather_presets: { let mut t = JsMap::new(); t.set(crate::weather_kinds::WeatherKind::Clear, crate::weather_kinds::WeatherPreset { title: String::from("Clear"), words: String::from("dry and still, so sound carries and nothing hides you but the dark"), storm: 0.0, rain: 0.0, wind: [0.35, 0.15], cloud: 0.05, fog: None, snow: None }); t.set(crate::weather_kinds::WeatherKind::Overcast, crate::weather_kinds::WeatherPreset { title: String::from("Overcast"), words: String::from("dry under a low sky. At night it's the same as clear"), storm: 0.0, rain: 0.0, wind: [0.8, 0.3], cloud: 0.9, fog: None, snow: None }); t.set(crate::weather_kinds::WeatherKind::Rain, crate::weather_kinds::WeatherPreset { title: String::from("Rain"), words: String::from("steady rain, so footsteps and doors carry about half as far outdoors"), storm: 0.25, rain: 0.55, wind: [1.1, 0.45], cloud: 0.95, fog: None, snow: None }); t.set(crate::weather_kinds::WeatherKind::Downpour, crate::weather_kinds::WeatherPreset { title: String::from("Downpour"), words: String::from("a tropical downpour with thunder. It drowns out small sounds, and lightning lights everyone up"), storm: 1.0, rain: 1.0, wind: [2.6, 1.0], cloud: 1.0, fog: Some(crate::weather_kinds::FogSpec { sigma: 0.02, h: 6.0, leak: 0.15, tint: [0.85, 0.9, 1.0], gain: 0.75 }), snow: None }); t.set(crate::weather_kinds::WeatherKind::Fog, crate::weather_kinds::WeatherPreset { title: String::from("Fog"), words: String::from("ground fog, so nobody can see past about twenty meters outdoors"), storm: 0.0, rain: 0.0, wind: [0.2, 0.1], cloud: 0.7, fog: Some(crate::weather_kinds::FogSpec { sigma: 0.113, h: 2.5, leak: 0.15, tint: [0.9, 0.95, 1.0], gain: 1.0 }), snow: None }); t.set(crate::weather_kinds::WeatherKind::Dust, crate::weather_kinds::WeatherPreset { title: String::from("Dust"), words: String::from("dry wind and dust in the air, so it's harder to see far and beams stand out"), storm: 0.0, rain: 0.0, wind: [3.2, 1.2], cloud: 0.25, fog: Some(crate::weather_kinds::FogSpec { sigma: 0.03, h: 8.0, leak: 0.3, tint: [1.0, 0.8, 0.55], gain: 1.0 }), snow: None }); t.set(crate::weather_kinds::WeatherKind::Snow, crate::weather_kinds::WeatherPreset { title: String::from("Snow"), words: String::from("snow falling on snow: the ground is bright, the air is quiet"), storm: 0.0, rain: 0.0, wind: [0.8, 0.3], cloud: 0.9, fog: None, snow: Some(1.48) }); t },
        }
    }
}

pub fn weather_kinds() -> Vec<String> { alloc::vec![String::from("clear"), String::from("overcast"), String::from("rain"), String::from("downpour"), String::from("fog"), String::from("dust")] }

pub fn snow_levels() -> Vec<String> { alloc::vec![String::from("pass"), String::from("tut-team"), String::from("tut-quiet"), String::from("tut-road")] }

pub const ROOF_Q: f64 = 0.2;

pub const WEATHER_KEY: &str = "weather";
