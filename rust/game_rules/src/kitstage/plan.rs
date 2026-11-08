//! The kit picture's numbers, and what of the plan needs no level file: the fourth man's acting place and the porch lamp on the photocell. Pure.
use crate::m;
use super::types::{V2, V3};

/// The nav bake's numbers: its cell, the circle, how far it must be moved to count, the heights, the grid, the collision cells the boxes are filed in
pub struct KitPlanNav { pub cell: f64, pub r: f64, pub moved: f64, pub y0: f64, pub y1: f64, pub w: usize, pub h: usize, pub coarse: f64 }
/// A roof, as the weather has it: the band its slab lies in, half a size at least, the most that count, the pad round each
pub struct KitPlanRoof { pub y0: f64, pub y1: f64, pub half: f64, pub most: usize, pub pad: f64 }
pub struct KitPlanNumbers {
    /// a wall man off the wall's face · between men · the first man past the jamb · Sam off the face · his turn onto the door · no nearer the anchor's middle than · what the old slots were laid at
    pub off_face: f64, pub gap: f64, pub jamb: f64, pub sam_off_face: f64, pub sam_turn_deg: f64, pub sam_clear: f64, pub authored: f64,
    /// a leaf's open stop where the door writes none
    pub leaf_stop: f64,
    /// the camera
    pub yaw: f64, pub pitch: f64, pub dist: f64, pub dist_per_m: f64,
    /// a man: the radius he is clear of a box by · no two men nearer than · his feet, belt and head for the camera's lines
    pub man: f64, pub apart: f64, pub heights: [f64; 3],
    /// the card switch's lane, m out from the file · the throws' spots in from the anchor's line, first choice and nearest
    pub lane: f64, pub in_m: f64, pub in_min: f64,
    /// the stage the camera must see: where the fourth man goes round Sam, his acting place (past Sam's jamb, off the line), its clearance and step, how far past the floor; a THING (what
    /// goes whole when a piece of it is in the way) is no taller and no longer than; what tops out under `low_top` hides no man; a prop goes with a box this near it
    pub round: f64, pub act4: V2, pub act_clear: f64, pub act_step: f64, pub stage_out: f64, pub thing_top: f64, pub thing_half: f64, pub low_top: f64, pub prop_near: f64,
    pub nav: KitPlanNav,
    /// depths are walked this far at most, in steps of · a threshold's shut cells are passed over up to
    pub reach: f64, pub step: f64, pub sill: f64,
    /// a wall box: no thicker across the line than · a cover: its underside this high and this wide · a wall a man stacks on comes down to his shoulder · a neighbouring door on the line within
    pub wall_half: f64, pub cover_y: f64, pub cover_half: f64, pub wall_foot: f64, pub door_on: f64,
    /// the lamps: the porch lamp off the face, up, under the wall's top · its box off the face and under the lamp · the fill out from the face, up, back from the free floor's end ·
    /// at a gap, how far past the jamb the lamp hangs and how tall its carrier must be
    pub porch_out: f64, pub porch_up: f64, pub porch_under: f64, pub fix_out: f64, pub fix_down: f64, pub porch_aim: V3, pub fill_out: f64, pub fill_up: f64, pub fill_back: f64, pub fill_aim: V3,
    pub porch_hand: f64, pub porch_carry: f64,
    pub roof: KitPlanRoof,
    /// a pane to shoot through, as the child's gun row picks it: in the anchor's own wall within `pane_wall`, along it within `pane_along`, its bottom no higher and top no lower than `pane_y`;
    /// a man's gun is at `gun_y`; he stands this far off it, the first that is free
    pub gun_y: f64, pub pane_off: [f64; 4], pub pane_wall: f64, pub pane_along: f64, pub pane_y: V2,
    /// the wedge behind the wall: this far behind the far face, this high; a lamp lights it from no further off than
    pub lit_in: f64, pub lit_y: f64, pub lit_reach: f64,
}
/// the picture's numbers
pub const KIT_PLAN: KitPlanNumbers = KitPlanNumbers {
    off_face: 0.70, gap: 1.1, jamb: 0.45, sam_off_face: 1.08, sam_turn_deg: 35.0, sam_clear: 1.8, authored: 0.9,
    leaf_stop: 1.92,
    yaw: 1.206, pitch: 0.96, dist: 8.8, dist_per_m: 0.85,
    man: 0.3, apart: 0.9, heights: [0.3, 1.0, 1.7],
    lane: 0.7, in_m: 2.0, in_min: 1.6,
    round: 0.75, act4: [0.2, 0.9], act_clear: 0.7, act_step: 0.05, stage_out: 0.3, thing_top: 2.2, thing_half: 3.0, low_top: 0.25, prop_near: 0.8,
    nav: KitPlanNav { cell: 0.5, r: 0.34, moved: 0.005, y0: 0.25, y1: 1.5, w: 80, h: 56, coarse: 1.0 },
    reach: 20.0, step: 0.25, sill: 1.2,
    wall_half: 0.6, cover_y: 1.9, cover_half: 0.5, wall_foot: 1.5, door_on: 0.15,
    porch_out: 0.55, porch_up: 2.4, porch_under: 0.2, fix_out: 0.05, fix_down: 0.13, porch_aim: [0.0, 2.35, 0.0], fill_out: 6.5, fill_up: 2.0, fill_back: 0.6, fill_aim: [0.6, 0.8, 0.9],
    porch_hand: 0.2, porch_carry: 2.0,
    roof: KitPlanRoof { y0: 2.75, y1: 3.45, half: 0.45, most: 4, pad: 0.3 },
    gun_y: 1.3, pane_off: [1.0, 1.3, 0.8, 1.6], pane_wall: 0.5, pane_along: 6.0, pane_y: [1.5, 1.2],
    lit_in: 1.0, lit_y: 1.0, lit_reach: 6.0,
};
const K: &KitPlanNumbers = &KIT_PLAN;

/// THE FOURTH MAN'S PLACE WHILE HIS CARD IS OPEN: past Sam's jamb by `act4[0]`, `act4[1]` off the anchor's line, on Sam's side of it. Past a wide opening Sam stands at his plain distance and that
/// place is two men in one; where it is nearer anybody than `clear` (none: `act_clear`) it is slid along the wall toward the anchor's middle, `act_step` at a time, to the first that is clear
/// and, a `free` given, on free floor. The slots are never moved for it. Clear as written: the place as written, to the bit.
pub fn act4(mid: V2, s: V2, n: V2, width: f64, slots: &[V3], free: Option<&dyn Fn(f64, f64) -> bool>, clear: Option<f64>) -> V2 {
    let clear = clear.unwrap_or(K.act_clear);
    let hw = width / 2.0;
    let at = |al: f64| -> V2 { [mid[0] + s[0] * al + n[0] * K.act4[1], mid[1] + s[1] * al + n[1] * K.act4[1]] };
    let others = &slots[..slots.len().min(4)];
    let ok = |p: V2| others.iter().all(|q| m::hyp2(p[0] - q[0], p[1] - q[1]) >= clear - 1e-9);
    let a0 = -(hw + K.act4[0]);
    let p0 = at(a0);
    if ok(p0) { return p0; }
    let mut k = 1.0;
    while a0 + k * K.act_step <= 1e-9 {
        let p = at(a0 + k * K.act_step);
        if ok(p) && free.map_or(true, |f| f(p[0], p[1])) { return p; }
        k += 1.0;
    }
    p0
}

/// THE PORCH LAMP IS ON THE LEVEL'S PHOTOCELL: how bright it burns. `lamps_out` = the hour has the level's outside lamps out. Under the open sky the lamp is then at the way's `light.porch.day`
/// (not written: 0); a way under a roof is as the game leaves indoors at every hour: `lit`.
pub fn porch_lit(lit: f64, day: Option<f64>, indoors: bool, lamps_out: bool) -> f64 { if lamps_out && !indoors { day.unwrap_or(0.0) } else { lit } }
