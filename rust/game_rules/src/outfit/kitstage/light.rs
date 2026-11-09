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

pub struct Stage { pub body: f64, pub reach: f64, pub gap: f64 }
/// a man's reach about his own middle, what the frame's fit keeps inside the window with him; and across the picture, on a frame hung on the door · between men, what the stack steps back by
pub const KIT_VIEW_STAGE: Stage = Stage { body: 0.45, reach: 0.33, gap: 1.1 };

pub struct Loop {
    /// seconds between repeats: a quiet row, a can, the gun and the light with no tile under the highlight, after a staging; what a damaging row left is looked at this long from its mark
    pub rest_quiet: f64, pub rest_can: f64, pub rest_none: f64, pub rest_stage: f64, pub hold: f64,
    /// THE AIM, deg and deg/s: the gun's rate, the body's, a sweep's; seconds an ease takes at least up to 45 deg and above; seconds a free channel takes to reach its rate; under this off his rest the chest alone turns; no quiet row asks more than this off it
    pub gun_rate: f64, pub body_rate: f64, pub sweep_rate: f64, pub ease: [f64; 2], pub ramp: f64, pub torso: f64, pub max_off: f64,
    /// the gun with no line: its scan to each side, seconds held at the end, the row's cap less 6
    pub gun_scan: f64, pub gun_hold: f64, pub gun_secs: f64,
    /// THE GUN FIRES: a pane within this of the door's middle; posts this far out, at these steps along it; the line no flatter than; a post no nearer another man than; no man within this of the line; rounds [long, pistol, shotgun], seconds between; seconds the gun stays up after the last
    pub pane_reach: f64, pub pane_out: f64, pub pane_off: [f64; 4], pub fire_slant: f64, pub fire_off: f64, pub fire_clear: f64, pub fire_n: [u32; 3], pub fire_gap: [f64; 3], pub fire_hold: f64,
    /// the light: his half step, seconds held at the near end, the leaf pushed to, the beam starts this far down and rises over this once this far off the wall's line, the slice no straighter than, a hand in from each jamb and more on the hinge's, its intensity for the row
    pub light_step: f64, pub light_hold: f64, pub wide_light: f64, pub feet_deg: f64, pub rise_secs: f64, pub wall_min: f64, pub slice_max: f64, pub jamb_in: f64, pub leaf_in: f64, pub beam: f64,
    /// suit and trim: the turn (deg), seconds held, seconds each way at least
    pub turn_deg: f64, pub turn_secs: f64, pub turn_ease: f64,
    /// a throw through the door: the old post, his half step, the spots in from the door's line and along it, the leaf's rate, its angle for a teammate and for Sam, seconds it is given at least and at most, Sam clicks once the can has had this long to reach the fist
    pub post: [f64; 2], pub half: f64, pub in_m: [f64; 3], pub in_along: [f64; 3], pub swing: f64, pub wide: f64, pub wide_sam: f64, pub open_secs: [f64; 2], pub fist_secs: f64,
    /// cans are thrown short: a teammate's smoke spots, Sam's smoke and stun
    pub smoke_m: [f64; 3], pub sam_smoke_m: f64, pub sam_stun_m: f64,
    /// seconds from the bang or the vent to the row's end, a vent shut after, a cut smoke vents nothing for
    pub after_stun: f64, pub after_smoke: f64, pub vent_secs: f64, pub vent_kill: f64,
    /// Sam's own charge: where he sets it from, how long the door is asked; the big tool: no further than this from the frame's middle; his frag: aimed in, the leaf ajar behind it from this long after it left
    pub own_post: [f64; 2], pub own_ask: f64, pub big_reach: f64, pub sam_frag_m: f64, pub ajar: f64, pub ajar_at: f64,
    /// a set charge is blown once clear or this long after · the row runs on this long after its mark
    pub clear_secs: f64, pub aftermath: f64,
    /// the wand: his place, seconds each way, a wide leaf is pulled home first for this long, him this far clear of its sweep
    pub wand_post: [f64; 2], pub wand_pan: f64, pub wand_shut: f64, pub wand_clear: f64,
    /// the sticky camera: his place, rays across the opening, no nearer than, within, the button waited for, looked at once stuck, the leaf's angle, a line no nearer the leaf than
    pub cam_post: [f64; 2], pub cam_rays: usize, pub cam_in: f64, pub cam_reach: f64, pub cam_ask: f64, pub cam_hold: f64, pub cam_wide: f64, pub cam_clear: f64,
    /// the 40 mm: deg out into the yard, seconds held
    pub shell_yard: f64, pub shell_hold: f64,
    /// the wall charge: spots along the stack's wall, blown after, a man further than this off his place goes back under the dissolve, bursts through a kept hole, Sam's half step given up within, the last man waited for
    pub wall_at: [f64; 6], pub wall_clear: f64, pub wall_off: f64, pub hole_bursts: u32, pub sam_give: f64, pub wall_late: f64,
    /// idle life: angles, gaps, ease, eyes
    pub scan_deg: [f64; 8], pub scan_gap: [f64; 7], pub scan_ease: f64, pub scan_y: f64,
    /// walks: a man's pace, the mover's, Sam's share, the lane, the diagonals' reach, the men ahead step back after, the thing begins on arrival or after, the fourth man's round, a way point reached within, on his place within
    pub pace: f64, pub swap_pace: f64, pub sam_walk: f64, pub lane: f64, pub cut_in: f64, pub shuffle: [f64; 2], pub swap_secs: f64, pub round: f64, pub way_m: f64, pub on_slot: f64,
    /// a row that would be refused for a moment is waited for this long
    pub wait: f64,
    /// HOME: a teammate is on his place within · a row of Sam's waits this long · at once further than
    pub home_m: f64, pub home_secs: f64, pub far_m: f64,
    /// Sam's frag: his step, the leaf, its arc lands this far inside, a grenade found nearer the door than this is taken away
    pub frag_half: f64, pub frag_wide: f64, pub frag_land: f64, pub frag_min: f64,
    /// a teammate's gun changed: the model is written this long in, the gun's row begins
    pub swap_at: f64, pub swap_up: f64,
    /// a thing on the floor is taken away once this old
    pub litter: f64,
    /// the brightest flash the picture draws · a lightning stroke's share of the night's own light
    pub flash_max: f64, pub stroke_k: f64, pub stroke_min: f64,
}
/// the loop's own numbers
pub const KIT_VIEW_LOOP: Loop = Loop {
    rest_quiet: 1.5, rest_can: 4.0, rest_none: 3.0, rest_stage: 0.8, hold: 6.0,
    gun_rate: 150.0, body_rate: 100.0, sweep_rate: 30.0, ease: [0.45, 0.65], ramp: 0.15, torso: 35.0, max_off: 60.0,
    gun_scan: 8.0, gun_hold: 0.3, gun_secs: 3.0,
    pane_reach: 6.0, pane_out: 1.6, pane_off: [1.0, -1.0, 1.8, -1.8], fire_slant: 0.5, fire_off: 0.8, fire_clear: 25.0, fire_n: [3, 2, 1], fire_gap: [0.11, 0.24, 0.3], fire_hold: 0.7,
    light_step: 0.4, light_hold: 0.4, wide_light: 1.3, feet_deg: 65.0, rise_secs: 0.85, wall_min: 25.0, slice_max: 60.0, jamb_in: 0.12, leaf_in: 0.25, beam: 30.0,
    turn_deg: 90.0, turn_secs: 0.6, turn_ease: 0.65,
    post: [0.15, 1.3], half: 0.5, in_m: [2.0, 1.6, 2.4], in_along: [0.0, 0.3, -0.3], swing: 3.0, wide: 0.9, wide_sam: 1.3, open_secs: [0.3, 2.5], fist_secs: 0.5,
    smoke_m: [0.6, 0.5, 0.8], sam_smoke_m: 0.2, sam_stun_m: 0.24,
    after_stun: 0.8, after_smoke: 2.5, vent_secs: 5.0, vent_kill: 6.0,
    own_post: [0.15, 1.1], own_ask: 1.5, big_reach: 1.0, sam_frag_m: 1.5, ajar: 0.35, ajar_at: 0.25,
    clear_secs: 1.5, aftermath: 1.6,
    wand_post: [0.0, 0.8], wand_pan: 0.8, wand_shut: 3.0, wand_clear: 0.45,
    cam_post: [0.15, 1.5], cam_rays: 9, cam_in: 0.3, cam_reach: 12.0, cam_ask: 1.5, cam_hold: 1.2, cam_wide: 1.75, cam_clear: 0.25,
    shell_yard: 20.0, shell_hold: 1.2,
    wall_at: [2.6, 3.1, 2.1, 3.6, 1.6, 4.1], wall_clear: 5.0, wall_off: 0.35, hole_bursts: 3, sam_give: 0.5, wall_late: 5.0,
    scan_deg: [5.0, -4.0, 3.0, -6.0, 4.0, -3.0, 6.0, -5.0], scan_gap: [3.4, 4.6, 3.9, 4.3, 3.1, 4.8, 3.6], scan_ease: 1.4, scan_y: 1.2,
    pace: 1.3, swap_pace: 1.6, sam_walk: 0.84, lane: 0.7, cut_in: 0.4, shuffle: [0.25, 0.15], swap_secs: 4.0, round: 0.75, way_m: 0.25, on_slot: 0.2,
    wait: 2.5,
    home_m: 0.08, home_secs: 1.5, far_m: 2.5,
    frag_half: 0.5, frag_wide: 1.5, frag_land: 1.0, frag_min: 1.0,
    swap_at: 0.55, swap_up: 0.85,
    litter: 8.0,
    flash_max: 18.0, stroke_k: 0.4, stroke_min: 0.5,
};

pub struct QuietOwn { pub back: f64, pub side: f64, pub up: f64, pub color: V3, pub intensity: f64, pub range: f64, pub radius: f64 }
pub struct QuietFrame { pub lamp: f64, pub boots: f64, pub spine: f64 }
pub struct Quiet {
    pub on: bool, pub lamp: Lamp, pub tip: f64, pub fill_color: V3, pub fill_intensity: f64, pub moon_scale: f64, pub moon_tint: V3, pub sky_scale: f64, pub roofs: bool, pub roof_eave: f64, pub frame: QuietFrame,
    pub ajar_deg: f64, pub leaf_rate: f64, pub inside: f64, pub own: QuietOwn, pub sam_along: f64, pub mate_gap: f64, pub step_back: f64,
    /// THE SUPPRESSED GUN'S ROW: seconds to rise and how far up, seconds the dark is held, the moon and the fill lifted to over this, he walks this long after the round to the jamb, the far one, the loop rests
    pub rise: f64, pub half: f64, pub hold: f64, pub lift: f64, pub lift_fill: f64, pub lift_secs: f64, pub walk_at: f64, pub jamb_along: f64, pub jamb_out: f64, pub jamb_far: bool, pub gun_rest: f64,
    /// the kill is a cut, and the cold flick at the muzzle
    pub cut: bool, pub flick: QuietFlick,
    /// the suit's lean and hold, a throw's step from the crouch, a gun left out is put away after
    pub lean_m: f64, pub lean_secs: f64, pub throw_step: f64, pub stow: f64,
    pub cost: QuietCost, pub idle: QuietIdle, pub light: QuietLight,
}
pub struct QuietFlick { pub intensity: f64, pub range: f64, pub secs: f64 }
pub struct QuietCostLamp { pub back: f64, pub back_min: f64, pub up: f64, pub side: f64, pub clear: f64, pub past: f64, pub body: f64, pub color: V3, pub intensity: f64, pub range: f64, pub radius: f64 }
/// LOUD KIT'S COST: the room at full for `slit` behind the leaf still ajar, the leaf thrown to `wide_deg` at `swing`, held, the rest after; the blade's share under the slit, the room's and the pool's under the wedge; his aim's height and his turn
pub struct QuietCost { pub slit: f64, pub wide_deg: f64, pub swing: f64, pub hold: f64, pub rest: f64, pub blade: f64, pub room: f64, pub pool: f64, pub aim_y: f64, pub turn: f64, pub lamp: QuietCostLamp }
/// LIFE AT REST: off as shipped; after each of these seconds of rest the warm lamp slides and back, dark for a blink
pub struct QuietIdle { pub on: bool, pub every: [f64; 5], pub slide: f64, pub out: f64, pub stay: f64, pub blink_at: f64, pub blink: f64, pub blink_to: f64, pub gain: f64, pub fade: f64 }
/// Sam's weapon light on a quiet kind: one pass across the door, held, off; aimed this high
pub struct QuietLight { pub sweep: f64, pub hold: f64, pub aim_y: f64 }
/// THE QUIET PICTURE: one crouched figure in a small warm pool under the door's lamp, the yard cool and dim, the door a hand ajar on a dim warm room
pub const KIT_VIEW_QUIET: Quiet = Quiet {
    on: true,
    lamp: Lamp { kind: 1.0, color: [1.0, 0.76, 0.5], intensity: 16.0, range: 5.0, inner_deg: 20.0, outer_deg: 24.0, radius: 0.03, volumetric: 0.0, broad: false }, tip: 0.25,
    fill_color: [0.4, 0.6, 1.0], fill_intensity: 0.8, moon_scale: 0.5, moon_tint: [0.4, 0.62, 1.0], sky_scale: 1.2,
    roofs: true, roof_eave: 0.15,
    frame: QuietFrame { lamp: 0.3, boots: 0.67, spine: 0.55 },
    ajar_deg: 16.0, leaf_rate: 1.2, inside: 0.15, own: QuietOwn { back: 0.4, side: 0.1, up: 1.9, color: [1.0, 0.72, 0.42], intensity: 8.0, range: 8.0, radius: 0.02 },
    sam_along: 1.1, mate_gap: 1.0, step_back: 0.5,
    rise: 0.65, half: 0.7, hold: 4.0, lift: 1.7, lift_fill: 1.3, lift_secs: 1.2, walk_at: 1.0, jamb_along: 0.2, jamb_out: 0.6, jamb_far: true, gun_rest: 3.5,
    cut: true, flick: QuietFlick { intensity: 5.0, range: 3.5, secs: 0.07 },
    lean_m: 0.4, lean_secs: 0.6, throw_step: 1.1, stow: 1.0,
    cost: QuietCost { slit: 0.4, wide_deg: 100.0, swing: 9.0, hold: 1.5, rest: 3.0, blade: 2.0, room: 1.0, pool: 0.0, aim_y: 1.2, turn: 240.0,
        lamp: QuietCostLamp { back: 0.45, back_min: 0.2, up: 1.9, side: 1.2, clear: 0.08, past: 0.25, body: 0.2, color: [1.0, 0.74, 0.46], intensity: 64.0, range: 8.0, radius: 0.01 } },
    idle: QuietIdle { on: false, every: [15.0, 14.0, 16.0, 14.5, 15.5], slide: 0.5, out: 1.2, stay: 0.6, blink_at: 0.5, blink: 0.4, blink_to: 0.1, gain: 1.0, fade: 0.3 },
    light: QuietLight { sweep: 0.5, hold: 0.2, aim_y: 0.9 },
};
