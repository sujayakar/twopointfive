//! The types of `hours`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes, Default)]
pub enum HourId {
    #[default] Overcast,
    Night,
    Dawn,
    Midday,
    Dusk,
}
impl HourId {
    pub const fn as_str(self) -> &'static str {
        match self {
            HourId::Overcast => "overcast",
            HourId::Night => "night",
            HourId::Dawn => "dawn",
            HourId::Midday => "midday",
            HourId::Dusk => "dusk",
        }
    }
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct HourDef {
    pub id: crate::hours::HourId,
    pub label: String,
    pub clock: String,
    pub words: String,
    pub sun: crate::hours::HourDefSun,
    pub sky: crate::hours::HourDefSky,
    pub haze: f64,
    pub exposure: f64,
    pub lamps: bool,
    pub cloud: Option<f64>,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct HourDefSun {
    pub bearing_deg: f64,
    pub elev_deg: f64,
    pub color: V3,
    pub intensity: f64,
    pub radius: f64,
    pub volumetric: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct HourDefSky {
    pub zenith: V3,
    pub horizon: V3,
}

#[derive(Clone, Debug, Default)]
pub struct Ray {
    pub d: V3,
    pub c6: Vec<f64>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Open {
    pub ray: usize,
    pub len: f64,
    pub sun_k: f64,
    pub sky_k: f64,
}

/// WHAT OF THE SKY REACHES THE LEVEL'S CELLS, asked of a cell when a point in it is first asked about and kept until a static is written or the sun (or the
/// moon) stands elsewhere. Made again from the world, so it is no state: it is on no wire and in no word
#[derive(Clone, Debug)]
pub struct SkyReach {
    /// the box world's count of its statics' writes these were asked at
    pub wrote: f64,
    pub to_sun: V3,
    pub box_: crate::hours::SkyStateBox,
    pub cells: JsMap<crate::Num, Vec<crate::hours::Open>>,
    pub fine: JsMap<crate::Num, Vec<crate::hours::Open>>,
}
impl SkyReach {
    /// of no world yet
    pub fn blank() -> Self { SkyReach { wrote: -1.0, to_sun: [0.0; 3], box_: crate::hours::SkyStateBox::default(), cells: JsMap::new(), fine: JsMap::new() } }
}
impl Default for SkyReach { fn default() -> Self { Self::blank() } }

#[derive(Clone, Copy, Debug, Default)]
pub struct SkyStateBox {
    pub x0: f64,
    pub x1: f64,
    pub z0: f64,
    pub z1: f64,
    pub top: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SkyStateDome {
    pub zenith: V3,
    pub horizon: V3,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct HourState {
    pub sun: LightId,
    pub def: crate::hours::HourDef,
    pub sun_k: f64,
    pub given: bool,
    pub soft: f64,
    pub sky_k: f64,
    pub was_off: bool,
    pub moons: Vec<LightId>,
    pub lamps: JsMap<FixtureId, bool>,
    pub storm: Option<crate::weather::WeatherSnap>,
    pub to_sun: V3,
}

#[derive(Clone, Debug, Default)]
pub struct NIGHT {
    pub label: String,
    pub clock: String,
    pub words: String,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes)]
pub struct HoursMod {
    pub states: Option<crate::hours::HourState>,
    /// (made again from the world: in no bytes)
    #[bytes(skip)]
    pub reach: crate::hours::SkyReach,
}
impl HoursMod {
    pub fn blank() -> Self {
        HoursMod {
            states: None,
            reach: crate::hours::SkyReach::blank(),
        }
    }
}

pub fn hour_ids() -> Vec<String> { alloc::vec![String::from("dawn"), String::from("midday"), String::from("overcast"), String::from("dusk"), String::from("night")] }

pub fn hours() -> JsMap<String, crate::hours::HourDef> { { let mut t = JsMap::new(); t.set(String::from("dawn"), crate::hours::HourDef { id: crate::hours::HourId::Dawn, label: String::from("Dawn"), clock: String::from("06:40"), words: String::from("early light, with a low orange sun, long shadows across the yard and the lights still on"), sun: crate::hours::HourDefSun { bearing_deg: 105.0, elev_deg: 22.0, color: [1.0, 0.62, 0.38], intensity: 2.6, radius: 0.02, volumetric: 1.2 }, sky: crate::hours::HourDefSky { zenith: [0.16, 0.22, 0.38], horizon: [0.56, 0.42, 0.34] }, haze: 0.05, exposure: 1.9, lamps: true, cloud: None }); t.set(String::from("midday"), crate::hours::HourDef { id: crate::hours::HourId::Midday, label: String::from("Midday"), clock: String::from("12:30"), words: String::from("a high white sun, with hard short shadows, every yard in plain sight and the rooms dark by contrast"), sun: crate::hours::HourDefSun { bearing_deg: 195.0, elev_deg: 56.0, color: [1.0, 0.97, 0.9], intensity: 6.0, radius: 0.012, volumetric: 0.6 }, sky: crate::hours::HourDefSky { zenith: [0.3, 0.45, 0.8], horizon: [0.7, 0.76, 0.84] }, haze: 0.03, exposure: 1.7, lamps: false, cloud: None }); t.set(String::from("overcast"), crate::hours::HourDef { id: crate::hours::HourId::Overcast, label: String::from("Grey day"), clock: String::from("15:30"), words: String::from("an overcast afternoon, with flat gray light, almost no shadows and nothing to hide in outdoors"), sun: crate::hours::HourDefSun { bearing_deg: 240.0, elev_deg: 50.0, color: [0.92, 0.95, 1.0], intensity: 1.0, radius: 0.25, volumetric: 0.0 }, sky: crate::hours::HourDefSky { zenith: [0.55, 0.58, 0.63], horizon: [0.6, 0.62, 0.65] }, haze: 0.045, exposure: 2.0, lamps: false, cloud: Some(0.9) }); t.set(String::from("dusk"), crate::hours::HourDef { id: crate::hours::HourId::Dusk, label: String::from("Dusk"), clock: String::from("17:00"), words: String::from("late light, with a low red sun, the lights coming on and the yard half in shadow"), sun: crate::hours::HourDefSun { bearing_deg: 255.0, elev_deg: 22.0, color: [1.0, 0.5, 0.26], intensity: 2.2, radius: 0.02, volumetric: 1.2 }, sky: crate::hours::HourDefSky { zenith: [0.13, 0.18, 0.36], horizon: [0.6, 0.38, 0.24] }, haze: 0.045, exposure: 2.0, lamps: true, cloud: None }); t } }

pub fn night() -> crate::hours::NIGHT { crate::hours::NIGHT { label: String::from("Night"), clock: String::from("02:00"), words: String::from("the level as it was built, with the moon, the lights outside and the storm") } }

pub fn alias() -> JsMap<String, String> { { let mut t = JsMap::new(); t.set(String::from("day"), String::from("midday")); t.set(String::from("noon"), String::from("midday")); t.set(String::from("afternoon"), String::from("overcast")); t.set(String::from("sunrise"), String::from("dawn")); t.set(String::from("sunset"), String::from("dusk")); t.set(String::from("evening"), String::from("dusk")); t } }

pub const SUN_NAME: &str = "sun";

pub const SUN_GROUP: &str = "hour";

pub const SOFT_RADIUS: f64 = 0.25;

pub fn grey() -> V3 { [0.948, 1.0, 1.086] }

pub const GREY_LUM: f64 = 0.9951539999999999;

pub const DEG: f64 = 0.017453292519943295;

pub fn n6() -> Vec<V3> { alloc::vec![[1.0, 0.0, 0.0], [-1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, -1.0, 0.0], [0.0, 0.0, 1.0], [0.0, 0.0, -1.0]] }

pub fn sky() -> Vec<crate::hours::Ray> { { fn add(to: &mut Vec<crate::hours::Ray>, n6: &[V3], elev: f64, n: usize, w: f64, turn: f64) { for i in 0..n { let a = (i as f64 + turn) / n as f64 * 2.0 * crate::m::PI; let d: V3 = [crate::m::cos(a) * crate::m::cos(elev), crate::m::sin(elev), crate::m::sin(a) * crate::m::cos(elev)]; let mut c6 = Vec::with_capacity(6); for nn in n6.iter() { c6.push(crate::m::max(0.0, crate::v3::dot(*nn, d)) * w); } to.push(crate::hours::Ray { d, c6 }); } } fn s(deg: f64) -> f64 { crate::m::sin(deg * crate::hours::DEG) } let n6 = crate::hours::n6(); let mut sky = Vec::with_capacity(15); add(&mut sky, &n6, 90.0 * crate::hours::DEG, 1, 2.0 * crate::m::PI * (1.0 - s(70.0)), 0.0); add(&mut sky, &n6, 50.0 * crate::hours::DEG, 6, 2.0 * crate::m::PI * (s(70.0) - s(32.0)) / 6.0, 0.0); add(&mut sky, &n6, 16.0 * crate::hours::DEG, 8, 2.0 * crate::m::PI * s(32.0) / 8.0, 0.5); sky } }

pub fn gnd() -> Vec<crate::hours::Ray> { { let n6 = crate::hours::n6(); let elev = crate::hours::GND_ELEV; let w = 2.0 * crate::m::PI * crate::m::sin(25.0 * crate::hours::DEG) / 8.0; let n = 8usize; let mut gnd = Vec::with_capacity(8); for i in 0..n { let a = (i as f64 + 0.25) / n as f64 * 2.0 * crate::m::PI; let d: V3 = [crate::m::cos(a) * crate::m::cos(elev), crate::m::sin(elev), crate::m::sin(a) * crate::m::cos(elev)]; let mut c6 = Vec::with_capacity(6); for nn in n6.iter() { c6.push(crate::m::max(0.0, crate::v3::dot(*nn, d)) * w); } gnd.push(crate::hours::Ray { d, c6 }); } gnd } }

pub const GND_ELEV: f64 = -0.20943951023931956;

pub const CELL: f64 = 0.5;

pub const CELL_Y: f64 = 1.3;

pub const DOOR_M: f64 = 7.0;

pub const GROUND_ALBEDO: f64 = 0.25;

