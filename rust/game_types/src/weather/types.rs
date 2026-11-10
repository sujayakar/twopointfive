//! The types of `weather`.
#![allow(unused_imports)]
use crate::{Blank, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Debug, ToBytes, Default)]
pub struct WeatherSnap {
    pub kind: Option<crate::weather_kinds::WeatherKind>,
    pub thunder_until: Option<f64>,
    pub storm: f64,
    pub rain_rate: f64,
    pub wind: [f64; 2],
    pub wet_gloss: bool,
    pub wet: f64,
    pub next_flash_at: f64,
    pub thunders: Vec<crate::weather::Thunder>,
    pub last_bark_t: f64,
    pub rng: f64,
    pub stats: crate::weather::WeatherSnapStats,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct Thunder {
    pub at: f64,
    pub dist_km: f64,
    pub bearing: f64,
    pub loud: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct WeatherSnapStats {
    pub flashes: f64,
    pub thunders: f64,
    pub barks: f64,
}

#[derive(Clone, Debug, ToBytes, Blank)]
pub struct Weather {
    #[is(true)] pub on: bool,
    pub kind_now: Option<crate::weather_kinds::WeatherKind>,
    pub roof_rects: Vec<[f64; 4]>,
    pub fog_spec: Option<crate::weather_kinds::FogSpec>,
    pub fog_word: f64,
    pub storm: f64,
    pub rain_rate: f64,
    /// the rain falling on the game this tick (0 with the storm off or held for the day): what the frame draws, and what soaks a man
    pub rain_now: f64,
    /// how wet the lot is
    pub wet: f64,
    /// game time until which a thunderclap is drowning the small noises
    #[is(-1e9)] pub thunder_until: f64,
    #[is(-1.0)] pub next_flash_at: f64,
    #[is(-1e9)] pub last_bark_t: f64,
    /// on its way, oldest first
    pub thunders: Vec<crate::weather::Thunder>,
    /// the storm's own die: never the game's, whose stream a storm must not shift
    pub die: u32,
    #[is(1.0)] pub fog_glow: f64,
    #[is([1.1, 0.45])] pub wind: [f64; 2],
    #[is(true)] pub wet_gloss: bool,
    pub flash: Option<crate::weather::Flash>,
    #[is(crate::weather::WeatherStats { flashes: 0.0, thunders: 0.0, barks: 0.0 })] pub stats: crate::weather::WeatherStats,
    pub touched: Vec<crate::weather::WeatherTouchedItem>,
    pub snow_rate: f64,
    pub own: bool,
    pub fits: Option<bool>,
    pub wet_on: bool,
    pub lot: Option<BoxId>,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Flash {
    pub light: LightId,
    pub stroke: crate::weather::Stroke,
    pub t: f64,
    pub peak: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Stroke {
    pub pulses: Vec<crate::weather::StrokePulsesItem>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct StrokePulsesItem {
    pub t0: f64,
    pub amp: f64,
    pub tau: f64,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct WeatherStats {
    pub flashes: f64,
    pub thunders: f64,
    pub barks: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct WeatherTouchedItem {
    pub box_: BoxId,
    pub albedo: V3,
    pub flags: f64,
    pub rough: Option<f64>,
    pub aniso: Option<bool>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct LEGACY {
    pub storm: f64,
    pub rain: f64,
    pub wind: [f64; 2],
    pub cloud: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct WeatherStrikeOpts {
    pub bearing: Option<f64>,
    pub elev_deg: Option<f64>,
    pub dist_km: Option<f64>,
    pub peak: Option<f64>,
}

pub fn legacy() -> crate::weather::LEGACY { crate::weather::LEGACY { storm: 0.7, rain: 0.55, wind: [1.1, 0.45], cloud: 0.95 } }

pub fn no_rect() -> [f64; 4] { [1000000.0, 1000000.0, -1000000.0, -1000000.0] }

pub fn interior() -> [f64; 4] { [4.0, 4.0, 36.0, 24.0] }

pub fn annex() -> [f64; 4] { [1.1, 9.33, 4.0, 12.87] }

pub const RAIN_TOP: f64 = 7.0;

pub const RAIN_V: f64 = 9.0;

pub const WET_ROUGH: f64 = 0.22;

pub const WET_ALBEDO: f64 = 0.5;

pub fn flash_color() -> V3 { [0.78, 0.86, 1.0] }

pub const FLASH_SECS: f64 = 0.5;

pub const SKY_FLASH_K: f64 = 0.045;

pub fn thunder_lines() -> Vec<String> { alloc::vec![String::from("lovely night for it."), String::from("that one was close."), String::from("glad I'm not out in that."), String::from("it's really coming down."), String::from("storm's right on top of us.")] }

pub const THUNDER_OUT: &str = "it's not letting up, huh.";

pub const BARK_COOLDOWN: f64 = 45.0;

pub const BARK_CHANCE: f64 = 0.35;
