//! The numbers of kitload.
use crate::bytes::ToBytes;
use crate::js::*;
use alloc::string::String;
use game_derive::Knobs;

#[derive(Clone, Debug, ToBytes, Knobs)]
pub struct KitloadK {
    /// THE TWO SCORES PULL APART, on purpose: a long gun is mostly SHAPE (it costs hiding, little pace), what fills a pouch is mostly WEIGHT (it costs pace,
    /// little hiding), and the big thing on the back is weight first. So "the long gun and empty pouches" is quick but seen, "the pistol and full pouches"
    /// is hidden but slow, and neither bar says what the other does.
    /// A GUN WEIGHS WHAT ITS ROW SAYS (weapons.ts `weightKg`, `bulk`: the data every gun carries for this): a point of mobility per kilogram, and of
    ///  concealment a pistol's worth for any gun + so much a step of bulk (0 a pistol … 4 a belt-fed gun). The issue carbine comes to about 0.14 / 0.38, a
    ///  pistol 0.03 / 0.04, the short gun between them 0.08 / 0.21, the belt-fed gun 0.28 / 0.72
    #[k(0.035)] pub gun_kg: f64,
    #[k([0.04, 0.17])] pub gun_bulk: [f64; 2],
    /// one of each small thing, by the card's field (about its real weight: a smoke can and a door charge are the heavy ones). A man's four pouches full
    ///  cost his legs about three times what his long gun does: emptying them is a trade worth making without changing the gun
    #[k({ let mut t = JsMap::new(); t.set(String::from("smoke"), [0.1, 0.02]); t.set(String::from("bangs"), [0.07, 0.02]); t.set(String::from("frags"), [0.08, 0.02]); t.set(String::from("shells"), [0.07, 0.01]); t.set(String::from("charges"), [0.1, 0.02]); t.set(String::from("wand"), [0.07, 0.02]); t.set(String::from("cams"), [0.04, 0.01]); t })] pub small: JsMap<String, [f64; 2]>,
    /// the big thing on his back, by what the big slot's row calls it (kit.ts bigOf)
    #[k({ let mut t = JsMap::new(); t.set(String::from("shotgun"), [0.18, 0.1]); t.set(String::from("hammer"), [0.3, 0.1]); t.set(String::from("wall"), [0.24, 0.06]); t })] pub big: JsMap<String, [f64; 2]>,
    /// how much of a point of mobility is pace: his legs are × 1 + (mobility − his issue's) × pace
    #[k(0.4)] pub pace: f64,
    /// … of a point of concealment is the notice he builds in an eye: × 1 − (concealment − his issue's) × seen; and a footfall's level, the same way
    #[k(0.6)] pub seen: f64,
    #[k(0.7)] pub heard: f64,
    /// how much of his pace factor YOUR SPRINT keeps (a flat-out run is legs more than load: 0.5 = half the walk's gain or loss)
    #[k(0.5)] pub sprint: f64,
    /// the most a TEAMMATE's legs are ever asked for with a light card (m/s): a purposeful walk still (guards.ts PACE.max is 2.2), well under the sprint clip
    ///  (character.ts BLEND: past 2.7)
    #[k(2.4)] pub pace_top: f64,
}
