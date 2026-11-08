//! The types of `hours`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: words)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, ToBytes)]
pub enum HourId {
    Overcast,
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

/// (TypeScript: HourDef, src/game/hours.ts:47)
#[derive(Clone, Debug, ToBytes)]
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
impl HourDef {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        HourDef {
            id: crate::hours::HourId::Overcast,
            label: String::new(),
            clock: String::new(),
            words: String::new(),
            sun: crate::hours::HourDefSun::zeroed(),
            sky: crate::hours::HourDefSky::zeroed(),
            haze: 0.0,
            exposure: 0.0,
            lamps: false,
            cloud: None,
        }
    }
}
impl Default for HourDef { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/hours.ts:56 HourDef.sun, src/game/hours.ts:56)
#[derive(Clone, Copy, Debug, ToBytes)]
pub struct HourDefSun {
    pub bearing_deg: f64,
    pub elev_deg: f64,
    pub color: V3,
    pub intensity: f64,
    pub radius: f64,
    pub volumetric: f64,
}
impl HourDefSun {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        HourDefSun {
            bearing_deg: 0.0,
            elev_deg: 0.0,
            color: [0.0; 3],
            intensity: 0.0,
            radius: 0.0,
            volumetric: 0.0,
        }
    }
}
impl Default for HourDefSun { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/hours.ts:57 HourDef.sky, src/game/hours.ts:57)
#[derive(Clone, Copy, Debug, ToBytes)]
pub struct HourDefSky {
    pub zenith: V3,
    pub horizon: V3,
}
impl HourDefSky {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        HourDefSky {
            zenith: [0.0; 3],
            horizon: [0.0; 3],
        }
    }
}
impl Default for HourDefSky { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Ray, src/game/hours.ts:114)
#[derive(Clone, Debug)]
pub struct Ray {
    pub d: V3,
    pub c6: Vec<f64>,
}
impl Ray {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Ray {
            d: [0.0; 3],
            c6: Vec::new(),
        }
    }
}
impl Default for Ray { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Open, src/game/hours.ts:115)
#[derive(Clone, Copy, Debug)]
pub struct Open {
    pub ray: usize,
    pub len: f64,
    pub sun_k: f64,
    pub sky_k: f64,
}
impl Open {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Open {
            ray: 0,
            len: 0.0,
            sun_k: 0.0,
            sky_k: 0.0,
        }
    }
}
impl Default for Open { fn default() -> Self { Self::zeroed() } }

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
    pub fn blank() -> Self { SkyReach { wrote: -1.0, to_sun: [0.0; 3], box_: crate::hours::SkyStateBox::zeroed(), cells: JsMap::new(), fine: JsMap::new() } }
}
impl Default for SkyReach { fn default() -> Self { Self::blank() } }

/// (TypeScript: inline at src/game/hours.ts:118 SkyState.box, src/game/hours.ts:118)
#[derive(Clone, Copy, Debug)]
pub struct SkyStateBox {
    pub x0: f64,
    pub x1: f64,
    pub z0: f64,
    pub z1: f64,
    pub top: f64,
}
impl SkyStateBox {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SkyStateBox {
            x0: 0.0,
            x1: 0.0,
            z0: 0.0,
            z1: 0.0,
            top: 0.0,
        }
    }
}
impl Default for SkyStateBox { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/hours.ts:121 SkyState.dome, src/game/hours.ts:121)
#[derive(Clone, Copy, Debug)]
pub struct SkyStateDome {
    pub zenith: V3,
    pub horizon: V3,
}
impl SkyStateDome {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SkyStateDome {
            zenith: [0.0; 3],
            horizon: [0.0; 3],
        }
    }
}
impl Default for SkyStateDome { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: HourState, src/game/hours.ts:124)
#[derive(Clone, Debug, ToBytes)]
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
impl HourState {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        HourState {
            sun: LightId(0),
            def: crate::hours::HourDef::zeroed(),
            sun_k: 0.0,
            given: false,
            soft: 0.0,
            sky_k: 0.0,
            was_off: false,
            moons: Vec::new(),
            lamps: JsMap::new(),
            storm: None,
            to_sun: [0.0; 3],
        }
    }
}
impl Default for HourState { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/hours.ts:79 NIGHT, src/game/hours.ts:79)
#[derive(Clone, Debug)]
pub struct NIGHT {
    pub label: String,
    pub clock: String,
    pub words: String,
}
impl NIGHT {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        NIGHT {
            label: String::new(),
            clock: String::new(),
            words: String::new(),
        }
    }
}
impl Default for NIGHT { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes)]
pub struct HoursMod {
    /// (src/game/hours.ts:140 states)
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

/// (src/game/hours.ts:45 HOUR_IDS: a table nothing writes)
pub fn hour_ids() -> Vec<String> { alloc::vec![String::from("dawn"), String::from("midday"), String::from("overcast"), String::from("dusk"), String::from("night")] }

/// (src/game/hours.ts:65 HOURS: a table nothing writes)
pub fn hours() -> JsMap<String, crate::hours::HourDef> { { let mut t = JsMap::new(); t.set(String::from("dawn"), crate::hours::HourDef { id: crate::hours::HourId::Dawn, label: String::from("Dawn"), clock: String::from("06:40"), words: String::from("early light, with a low orange sun, long shadows across the yard and the lights still on"), sun: crate::hours::HourDefSun { bearing_deg: 105.0, elev_deg: 22.0, color: [1.0, 0.62, 0.38], intensity: 2.6, radius: 0.02, volumetric: 1.2 }, sky: crate::hours::HourDefSky { zenith: [0.16, 0.22, 0.38], horizon: [0.56, 0.42, 0.34] }, haze: 0.05, exposure: 1.9, lamps: true, cloud: None }); t.set(String::from("midday"), crate::hours::HourDef { id: crate::hours::HourId::Midday, label: String::from("Midday"), clock: String::from("12:30"), words: String::from("a high white sun, with hard short shadows, every yard in plain sight and the rooms dark by contrast"), sun: crate::hours::HourDefSun { bearing_deg: 195.0, elev_deg: 56.0, color: [1.0, 0.97, 0.9], intensity: 6.0, radius: 0.012, volumetric: 0.6 }, sky: crate::hours::HourDefSky { zenith: [0.3, 0.45, 0.8], horizon: [0.7, 0.76, 0.84] }, haze: 0.03, exposure: 1.7, lamps: false, cloud: None }); t.set(String::from("overcast"), crate::hours::HourDef { id: crate::hours::HourId::Overcast, label: String::from("Grey day"), clock: String::from("15:30"), words: String::from("an overcast afternoon, with flat gray light, almost no shadows and nothing to hide in outdoors"), sun: crate::hours::HourDefSun { bearing_deg: 240.0, elev_deg: 50.0, color: [0.92, 0.95, 1.0], intensity: 1.0, radius: 0.25, volumetric: 0.0 }, sky: crate::hours::HourDefSky { zenith: [0.55, 0.58, 0.63], horizon: [0.6, 0.62, 0.65] }, haze: 0.045, exposure: 2.0, lamps: false, cloud: Some(0.9) }); t.set(String::from("dusk"), crate::hours::HourDef { id: crate::hours::HourId::Dusk, label: String::from("Dusk"), clock: String::from("17:00"), words: String::from("late light, with a low red sun, the lights coming on and the yard half in shadow"), sun: crate::hours::HourDefSun { bearing_deg: 255.0, elev_deg: 22.0, color: [1.0, 0.5, 0.26], intensity: 2.2, radius: 0.02, volumetric: 1.2 }, sky: crate::hours::HourDefSky { zenith: [0.13, 0.18, 0.36], horizon: [0.6, 0.38, 0.24] }, haze: 0.045, exposure: 2.0, lamps: true, cloud: None }); t } }

/// (src/game/hours.ts:79 NIGHT: a table nothing writes)
pub fn night() -> crate::hours::NIGHT { crate::hours::NIGHT { label: String::from("Night"), clock: String::from("02:00"), words: String::from("the level as it was built, with the moon, the lights outside and the storm") } }

/// (src/game/hours.ts:90 ALIAS: a table nothing writes)
pub fn alias() -> JsMap<String, String> { { let mut t = JsMap::new(); t.set(String::from("day"), String::from("midday")); t.set(String::from("noon"), String::from("midday")); t.set(String::from("afternoon"), String::from("overcast")); t.set(String::from("sunrise"), String::from("dawn")); t.set(String::from("sunset"), String::from("dusk")); t.set(String::from("evening"), String::from("dusk")); t } }

/// (src/game/hours.ts:145 SUN_NAME)
pub const SUN_NAME: &str = "sun";

/// (src/game/hours.ts:145 SUN_GROUP)
pub const SUN_GROUP: &str = "hour";

/// (src/game/hours.ts:204 SOFT_RADIUS)
pub const SOFT_RADIUS: f64 = 0.25;

/// (src/game/hours.ts:205 GREY: a table nothing writes)
pub fn grey() -> V3 { [0.948, 1.0, 1.086] }

/// (src/game/hours.ts:205 GREY_LUM)
pub const GREY_LUM: f64 = 0.9951539999999999;

/// (src/game/hours.ts:248 DEG)
pub const DEG: f64 = 0.017453292519943295;

/// (src/game/hours.ts:304 N6: a table nothing writes)
pub fn n6() -> Vec<V3> { alloc::vec![[1.0, 0.0, 0.0], [-1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, -1.0, 0.0], [0.0, 0.0, 1.0], [0.0, 0.0, -1.0]] }

/// (src/game/hours.ts:307 SKY: a table nothing writes)
pub fn sky() -> Vec<crate::hours::Ray> { { fn add(to: &mut Vec<crate::hours::Ray>, n6: &[V3], elev: f64, n: usize, w: f64, turn: f64) { for i in 0..n { let a = (i as f64 + turn) / n as f64 * 2.0 * crate::m::PI; let d: V3 = [crate::m::cos(a) * crate::m::cos(elev), crate::m::sin(elev), crate::m::sin(a) * crate::m::cos(elev)]; let mut c6 = Vec::with_capacity(6); for nn in n6.iter() { c6.push(crate::m::max(0.0, crate::v3::dot(*nn, d)) * w); } to.push(crate::hours::Ray { d, c6 }); } } fn s(deg: f64) -> f64 { crate::m::sin(deg * crate::hours::DEG) } let n6 = crate::hours::n6(); let mut sky = Vec::with_capacity(15); add(&mut sky, &n6, 90.0 * crate::hours::DEG, 1, 2.0 * crate::m::PI * (1.0 - s(70.0)), 0.0); add(&mut sky, &n6, 50.0 * crate::hours::DEG, 6, 2.0 * crate::m::PI * (s(70.0) - s(32.0)) / 6.0, 0.0); add(&mut sky, &n6, 16.0 * crate::hours::DEG, 8, 2.0 * crate::m::PI * s(32.0) / 8.0, 0.5); sky } }

/// (src/game/hours.ts:307 GND: a table nothing writes)
pub fn gnd() -> Vec<crate::hours::Ray> { { let n6 = crate::hours::n6(); let elev = crate::hours::GND_ELEV; let w = 2.0 * crate::m::PI * crate::m::sin(25.0 * crate::hours::DEG) / 8.0; let n = 8usize; let mut gnd = Vec::with_capacity(8); for i in 0..n { let a = (i as f64 + 0.25) / n as f64 * 2.0 * crate::m::PI; let d: V3 = [crate::m::cos(a) * crate::m::cos(elev), crate::m::sin(elev), crate::m::sin(a) * crate::m::cos(elev)]; let mut c6 = Vec::with_capacity(6); for nn in n6.iter() { c6.push(crate::m::max(0.0, crate::v3::dot(*nn, d)) * w); } gnd.push(crate::hours::Ray { d, c6 }); } gnd } }

/// (src/game/hours.ts:308 GND_ELEV)
pub const GND_ELEV: f64 = -0.20943951023931956;

/// (src/game/hours.ts:308 CELL)
pub const CELL: f64 = 0.5;

/// (src/game/hours.ts:308 CELL_Y)
pub const CELL_Y: f64 = 1.3;

/// (src/game/hours.ts:308 DOOR_M)
pub const DOOR_M: f64 = 7.0;

/// (src/game/hours.ts:308 GROUND_ALBEDO)
pub const GROUND_ALBEDO: f64 = 0.25;

/// (src/game/hours.ts:341 NO_BOXES: a table nothing writes)
pub fn no_boxes() -> Vec<BoxId> { Vec::new() }
