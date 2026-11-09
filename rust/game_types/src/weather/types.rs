//! The types of `weather`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: WeatherSnap, src/game/weather.ts:121)
#[derive(Clone, Debug, ToBytes)]
pub struct WeatherSnap {
    pub kind: Option<crate::weather_kinds::WeatherKind>,
    pub thunder_until: Option<f64>,
    pub on: bool,
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
impl WeatherSnap {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WeatherSnap {
            kind: None,
            thunder_until: None,
            on: false,
            storm: 0.0,
            rain_rate: 0.0,
            wind: [0.0; 2],
            wet_gloss: false,
            wet: 0.0,
            next_flash_at: 0.0,
            thunders: Vec::new(),
            last_bark_t: 0.0,
            rng: 0.0,
            stats: crate::weather::WeatherSnapStats::zeroed(),
        }
    }
}
impl Default for WeatherSnap { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Thunder, src/game/weather.ts:133)
#[derive(Clone, Copy, Debug, ToBytes)]
pub struct Thunder {
    pub at: f64,
    pub dist_km: f64,
    pub bearing: f64,
    pub loud: f64,
}
impl Thunder {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Thunder {
            at: 0.0,
            dist_km: 0.0,
            bearing: 0.0,
            loud: 0.0,
        }
    }
}
impl Default for Thunder { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/weather.ts:121 WeatherSnap.stats, src/game/weather.ts:121)
#[derive(Clone, Copy, Debug, ToBytes)]
pub struct WeatherSnapStats {
    pub flashes: f64,
    pub thunders: f64,
    pub barks: f64,
}
impl WeatherSnapStats {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WeatherSnapStats {
            flashes: 0.0,
            thunders: 0.0,
            barks: 0.0,
        }
    }
}
impl Default for WeatherSnapStats { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Weather, src/game/weather.ts:135)
#[derive(Clone, Debug, ToBytes)]
pub struct Weather {
    pub on: bool,
    pub kind_now: Option<crate::weather_kinds::WeatherKind>,
    pub roof_rects: Vec<[f64; 4]>,
    pub fog_spec: Option<crate::weather_kinds::FogSpec>,
    pub fog_word: f64,
    pub fog_glow: f64,
    pub wind: [f64; 2],
    pub wet_gloss: bool,
    pub flash: Option<crate::weather::Flash>,
    pub stats: crate::weather::WeatherStats,
    pub suspended: bool,
    pub touched: Vec<crate::weather::WeatherTouchedItem>,
    pub gloss_was: Option<bool>,
    pub snow_rate: f64,
    pub own: bool,
    pub fits: Option<bool>,
    pub wet_on: bool,
    pub lot: Option<BoxId>,
}
impl Weather {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
    pub fn blank() -> Self {
        Weather {
            on: true,
            kind_now: None,
            roof_rects: Vec::new(),
            fog_spec: None,
            fog_word: 0.0,
            fog_glow: 1.0,
            wind: [1.1, 0.45],
            wet_gloss: true,
            flash: None,
            stats: crate::weather::WeatherStats { flashes: 0.0, thunders: 0.0, barks: 0.0, last_thunder: None },
            suspended: false,
            touched: Vec::new(),
            gloss_was: None,
            snow_rate: 0.0,
            own: false,
            fits: None,
            wet_on: false,
            lot: None,
        }
    }
}
impl Default for Weather { fn default() -> Self { Self::blank() } }

/// (TypeScript: Flash, src/game/weather.ts:132)
#[derive(Clone, Debug, ToBytes)]
pub struct Flash {
    pub light: LightId,
    pub stroke: crate::weather::Stroke,
    pub t: f64,
    pub peak: f64,
    pub dir: V3,
    pub bearing: f64,
    pub dist_km: f64,
}
impl Flash {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Flash {
            light: LightId(0),
            stroke: crate::weather::Stroke::zeroed(),
            t: 0.0,
            peak: 0.0,
            dir: [0.0; 3],
            bearing: 0.0,
            dist_km: 0.0,
        }
    }
}
impl Default for Flash { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Stroke, src/game/weather.ts:124)
#[derive(Clone, Debug, ToBytes)]
pub struct Stroke {
    pub pulses: Vec<crate::weather::StrokePulsesItem>,
}
impl Stroke {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Stroke {
            pulses: Vec::new(),
        }
    }
}
impl Default for Stroke { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/weather.ts:124 Stroke.pulses, src/game/weather.ts:124)
#[derive(Clone, Copy, Debug, ToBytes)]
pub struct StrokePulsesItem {
    pub t0: f64,
    pub amp: f64,
    pub tau: f64,
}
impl StrokePulsesItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        StrokePulsesItem {
            t0: 0.0,
            amp: 0.0,
            tau: 0.0,
        }
    }
}
impl Default for StrokePulsesItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/weather.ts:191 Weather.stats, src/game/weather.ts:191)
#[derive(Clone, Debug, ToBytes)]
pub struct WeatherStats {
    pub flashes: f64,
    pub thunders: f64,
    pub barks: f64,
    pub last_thunder: Option<crate::weather::WeatherStatsLastThunder>,
}
impl WeatherStats {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WeatherStats {
            flashes: 0.0,
            thunders: 0.0,
            barks: 0.0,
            last_thunder: None,
        }
    }
}
impl Default for WeatherStats { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/weather.ts:191 WeatherStats.lastThunder, src/game/weather.ts:191 WeatherStats.lastThunder)
#[derive(Clone, Debug, ToBytes)]
pub struct WeatherStatsLastThunder {
    pub at: f64,
    pub dist_km: f64,
    pub bearing: f64,
    pub loud: f64,
    pub played_at: f64,
    pub name: String,
    pub gain: f64,
}
impl WeatherStatsLastThunder {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WeatherStatsLastThunder {
            at: 0.0,
            dist_km: 0.0,
            bearing: 0.0,
            loud: 0.0,
            played_at: 0.0,
            name: String::new(),
            gain: 0.0,
        }
    }
}
impl Default for WeatherStatsLastThunder { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/weather.ts:198 Weather.touched, src/game/weather.ts:198)
#[derive(Clone, Copy, Debug, ToBytes)]
pub struct WeatherTouchedItem {
    pub box_: BoxId,
    pub albedo: V3,
    pub flags: f64,
    pub rough: Option<f64>,
    pub aniso: Option<bool>,
}
impl WeatherTouchedItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WeatherTouchedItem {
            box_: BoxId(0),
            albedo: [0.0; 3],
            flags: 0.0,
            rough: None,
            aniso: None,
        }
    }
}
impl Default for WeatherTouchedItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/weather.ts:63 LEGACY, src/game/weather.ts:63)
#[derive(Clone, Copy, Debug)]
pub struct LEGACY {
    pub storm: f64,
    pub rain: f64,
    pub wind: [f64; 2],
    pub cloud: f64,
}
impl LEGACY {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        LEGACY {
            storm: 0.0,
            rain: 0.0,
            wind: [0.0; 2],
            cloud: 0.0,
        }
    }
}
impl Default for LEGACY { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/weather.ts:332 strike(opts), src/game/weather.ts:332)
#[derive(Clone, Copy, Debug)]
pub struct WeatherStrikeOpts {
    pub bearing: Option<f64>,
    pub elev_deg: Option<f64>,
    pub dist_km: Option<f64>,
    pub peak: Option<f64>,
}
impl WeatherStrikeOpts {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        WeatherStrikeOpts {
            bearing: None,
            elev_deg: None,
            dist_km: None,
            peak: None,
        }
    }
}
impl Default for WeatherStrikeOpts { fn default() -> Self { Self::zeroed() } }

/// (src/game/weather.ts:63 LEGACY: a table nothing writes)
pub fn legacy() -> crate::weather::LEGACY { crate::weather::LEGACY { storm: 0.7, rain: 0.55, wind: [1.1, 0.45], cloud: 0.95 } }

/// (src/game/weather.ts:64 NO_RECT: a table nothing writes)
pub fn no_rect() -> [f64; 4] { [1000000.0, 1000000.0, -1000000.0, -1000000.0] }

/// (src/game/weather.ts:102 INTERIOR: a table nothing writes)
pub fn interior() -> [f64; 4] { [4.0, 4.0, 36.0, 24.0] }

/// (src/game/weather.ts:105 ANNEX: a table nothing writes)
pub fn annex() -> [f64; 4] { [1.1, 9.33, 4.0, 12.87] }

/// (src/game/weather.ts:106 RAIN_TOP)
pub const RAIN_TOP: f64 = 7.0;

/// (src/game/weather.ts:107 RAIN_V)
pub const RAIN_V: f64 = 9.0;

/// (src/game/weather.ts:108 WET_ROUGH)
pub const WET_ROUGH: f64 = 0.22;

/// (src/game/weather.ts:109 WET_ALBEDO)
pub const WET_ALBEDO: f64 = 0.5;

/// (src/game/weather.ts:110 FLASH_COLOR: a table nothing writes)
pub fn flash_color() -> V3 { [0.78, 0.86, 1.0] }

/// (src/game/weather.ts:111 FLASH_SECS)
pub const FLASH_SECS: f64 = 0.5;

/// (src/game/weather.ts:113 SKY_FLASH_K)
pub const SKY_FLASH_K: f64 = 0.045;

/// (src/game/weather.ts:114 THUNDER_LINES: a table nothing writes)
pub fn thunder_lines() -> Vec<String> { alloc::vec![String::from("lovely night for it."), String::from("that one was close."), String::from("glad I'm not out in that."), String::from("it's really coming down."), String::from("storm's right on top of us.")] }

/// (src/game/weather.ts:116 THUNDER_OUT)
pub const THUNDER_OUT: &str = "no let-up, is there.";

/// (src/game/weather.ts:116 THUNDER_OUT_GUIDE)
pub const THUNDER_OUT_GUIDE: &str = "it's not letting up, huh.";

/// (src/game/weather.ts:117 BARK_COOLDOWN)
pub const BARK_COOLDOWN: f64 = 45.0;

/// (src/game/weather.ts:118 BARK_CHANCE)
pub const BARK_CHANCE: f64 = 0.35;
