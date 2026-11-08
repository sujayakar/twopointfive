//! The numbers the kit picture's staging reads: the lamps, the frame's reach, the quiet picture. A host's table: no rule of the game reads it, so it is no knob of the game's.
use crate::V3;

pub struct Lamp { pub kind: f64, pub color: V3, pub intensity: f64, pub range: f64, pub inner_deg: f64, pub outer_deg: f64, pub radius: f64, pub volumetric: f64, pub broad: bool }
pub struct LampFixture { pub half: V3, pub glow: f64 }
pub struct SamMark { pub on: bool, pub intensity: f64, pub inner_deg: f64, pub outer_deg: f64, pub volumetric: f64, pub aim: V3 }
pub struct Light { pub porch: Lamp, pub fixture: LampFixture, pub fill: Lamp, pub room_lights: f64, pub moon_scale: f64, pub sky_scale: f64, pub sam: SamMark }
/// The light: a porch lamp over the door for the key, a wide cool fill from the yard, the building's own lights out, the moon and sky raised a little, Sam's weapon light dim at the door's foot
pub const KIT_VIEW_LIGHT: Light = Light {
    porch: Lamp { kind: 3.0, color: [1.0, 0.76, 0.5], intensity: 15.0, range: 14.0, inner_deg: 0.0, outer_deg: 90.0, radius: 0.28, volumetric: 0.0, broad: true },
    fixture: LampFixture { half: [0.17, 0.045, 0.05], glow: 5.0 },
    fill: Lamp { kind: 3.0, color: [0.85, 0.9, 1.0], intensity: 2.5, range: 24.0, inner_deg: 0.0, outer_deg: 90.0, radius: 1.5, volumetric: 0.0, broad: true },
    room_lights: 0.0, moon_scale: 1.65, sky_scale: 1.6,
    sam: SamMark { on: true, intensity: 9.0, inner_deg: 8.0, outer_deg: 24.0, volumetric: 0.25, aim: [0.0, 0.0, 0.0] },
};

pub struct Stage { pub body: f64, pub reach: f64 }
/// a man's reach about his own middle, what the frame's fit keeps inside the window with him; and across the picture, on a frame hung on the door
pub const KIT_VIEW_STAGE: Stage = Stage { body: 0.45, reach: 0.33 };

pub struct Loop { pub gun_rate: f64, pub body_rate: f64, pub ramp: f64, pub pace: f64, pub flash_max: f64, pub stroke_k: f64, pub stroke_min: f64 }
/// the gun's and the body's turn (deg/s) and how long each takes to reach it · a man's walk to his place · the brightest flash the picture draws · a lightning stroke's share of the night's own light
pub const KIT_VIEW_LOOP: Loop = Loop { gun_rate: 150.0, body_rate: 100.0, ramp: 0.15, pace: 1.3, flash_max: 18.0, stroke_k: 0.4, stroke_min: 0.5 };

pub struct QuietOwn { pub back: f64, pub side: f64, pub up: f64, pub color: V3, pub intensity: f64, pub range: f64, pub radius: f64 }
pub struct QuietFrame { pub lamp: f64, pub boots: f64, pub spine: f64 }
pub struct Quiet {
    pub on: bool, pub lamp: Lamp, pub tip: f64, pub fill_color: V3, pub fill_intensity: f64, pub moon_scale: f64, pub moon_tint: V3, pub sky_scale: f64, pub roofs: bool, pub roof_eave: f64, pub frame: QuietFrame,
    pub ajar_deg: f64, pub inside: f64, pub own: QuietOwn, pub sam_along: f64, pub mate_gap: f64, pub step_back: f64,
}
/// THE QUIET PICTURE: one crouched figure in a small warm pool under the door's lamp, the yard cool and dim, the door a hand ajar on a dim warm room
pub const KIT_VIEW_QUIET: Quiet = Quiet {
    on: true,
    lamp: Lamp { kind: 1.0, color: [1.0, 0.76, 0.5], intensity: 16.0, range: 5.0, inner_deg: 20.0, outer_deg: 24.0, radius: 0.03, volumetric: 0.0, broad: false }, tip: 0.25,
    fill_color: [0.4, 0.6, 1.0], fill_intensity: 0.8, moon_scale: 0.5, moon_tint: [0.4, 0.62, 1.0], sky_scale: 1.2,
    roofs: true, roof_eave: 0.15,
    frame: QuietFrame { lamp: 0.3, boots: 0.67, spine: 0.55 },
    ajar_deg: 16.0, inside: 0.15, own: QuietOwn { back: 0.4, side: 0.1, up: 1.9, color: [1.0, 0.72, 0.42], intensity: 8.0, range: 8.0, radius: 0.02 },
    sam_along: 1.1, mate_gap: 1.0, step_back: 0.5,
};
