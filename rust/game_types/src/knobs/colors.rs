//! THE PAINT OF DEVICES AND SMALL FITTINGS: the color of each box that makes them up, and what their little lights glow.
//! The picture alone. The camera's cone is the one real lamp here, a light among the level's lights.
use crate::bytes::ToBytes;
use crate::js::JsMap;
use crate::V3;
use alloc::string::String;
use game_derive::Knobs;

#[derive(Clone, Debug, ToBytes, Knobs)]
pub struct ColorsK {
    /// the wall camera's LED: at rest (this and the next blink once the camera has gone off idle)
    #[k([1.6, 0.08, 0.05])] pub camera_led: V3,
    /// … while it's seeing someone, × 0.4‥1.6 as its count nears `tune.camera_suspicion_secs`
    #[k([14.0, 0.7, 0.3])] pub camera_led_hot: V3,
    /// the wall camera's body (the box that pans) · the fixed arm behind it
    #[k([0.16, 0.17, 0.19])] pub camera: V3,
    #[k([0.05, 0.05, 0.06])] pub camera_dark: V3,
    /// the door keypad's case
    #[k([0.22, 0.23, 0.25])] pub keypad: V3,
    /// the two little boxes at the ends of a laser tripwire
    #[k([0.14, 0.14, 0.16])] pub tripwire: V3,
    /// a keypad's LED (drawn at half this): door unlocked · door locked · someone working the code (flashes). Red is also a tripwire emitter's flash for 1.5 s after a trip, at × 2 and × 0.2
    #[k([0.3, 5.0, 0.6])] pub light_green: V3,
    #[k([5.0, 0.3, 0.15])] pub light_red: V3,
    #[k([5.0, 2.4, 0.2])] pub light_amber: V3,
    /// the sticky camera: its body · its lens · its lens while it's sending to the player
    #[k([0.05, 0.05, 0.055])] pub sticky_cam: V3,
    #[k([0.012, 0.012, 0.014])] pub sticky_lens: V3,
    #[k([0.42, 0.05, 0.04])] pub sticky_lens_live: V3,
    /// a light switch: its wall plate · its rocker · the small locator lamp under it, which glows only while the light is off
    #[k([0.62, 0.6, 0.55])] pub switch_plate: V3,
    #[k([0.74, 0.72, 0.66])] pub switch_rocker: V3,
    #[k([1.6, 0.8, 0.16])] pub switch_glow: V3,
    /// the pale green-white of float glass seen through its edge: what a cut plate's edge is drawn toward (`shards.cut_edge`), and its resting face (`shards.cut_rest_body`)
    #[k([0.74, 0.94, 0.86])] pub glass_edge: V3,
    /// what an opening torn in a wall is framed with, unless the wall is wood (a wooden one's frame is the wall's paint × 0.8)
    #[k([0.5, 0.52, 0.54])] pub torn_steel: V3,
    /// the lens of a carried rifle's light, × the rig's `lens_glow`, when the kit has that light on
    #[k([0.9, 0.96, 1.0])] pub rail_lens: V3,
    /// the rescue flow's glowing strip across a corridor's opening, by the opening's state, × `rescueflow.glow` (a hot one also beats)
    #[k({ let mut t = JsMap::new(); t.set(String::from("unknown"), [1.0, 0.55, 0.08]); t.set(String::from("looked"), [0.6, 0.62, 0.5]); t.set(String::from("hot"), [2.6, 0.12, 0.05]); t.set(String::from("held"), [0.12, 0.5, 1.6]); t.set(String::from("cleared"), [0.1, 1.0, 0.3]); t })] pub openings: crate::js::JsMap<alloc::string::String, V3>,
    /// the lamp a wall camera throws where it looks
    pub cone: ColorsKCone,
}

/// A CAMERA'S CONE: a real spot lamp on the camera, lit while it works and `tune.device_cones` is on.
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct ColorsKCone {
    /// how bright the lamp is
    #[k(1.1)] pub intensity: f64,
    /// its color
    #[k([1.0, 0.12, 0.07])] pub color: V3,
    /// the lamp's range as a share of the camera's own range
    #[k(0.7)] pub range_k: f64,
    /// how much it shows in the air (1 = fully, 0 = not at all)
    #[k(1.0)] pub volumetric: f64,
    /// degrees: the most its outer half-angle can be, however wide the camera sees (the inner one is 6 less, at least 1)
    #[k(24.0)] pub outer_max_deg: f64,
    /// the lamp's own radius (m)
    #[k(0.12)] pub radius: f64,
}

