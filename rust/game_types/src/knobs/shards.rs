//! The numbers of shards.
use crate::bytes::ToBytes;
use crate::js::*;
use alloc::string::String;
use game_derive::Knobs;

#[derive(Clone, Debug, ToBytes, Knobs)]
pub struct ShardsK {
    /// ---- the spray (counts per m² of fragment; clamped per fragment below)
    #[k(140.0)] pub mid_per_m2: f64,
    #[k(2.0)] pub mid_min: f64,
    #[k(18.0)] pub mid_max: f64,
    #[k(0.02)] pub mid_size: f64,
    #[k(60.0)] pub mid_life: f64,
    #[k(520.0)] pub fine_per_m2: f64,
    #[k(6.0)] pub fine_min: f64,
    #[k(70.0)] pub fine_max: f64,
    #[k(0.0045)] pub fine_size: f64,
    #[k(22.0)] pub fine_life: f64,
    /// of the fines, thrown back toward the shooter (a round / a blast only)
    #[k(0.18)] pub back_share: f64,
    /// (tuning round two) a ROUND's own piece: at least hitFineMin fines and hitBackShare of them back at the shooter — one round through float glass was nine particles, nothing to see
    #[k(16.0)] pub hit_fine_min: f64,
    #[k(0.4)] pub hit_back_share: f64,
    /// … and a blow that knocks NOTHING out (a laminated web taking a round, a soft hit that only stars the sheet) still spits: spallOut fines off the far face, spallBack off the near one. 0 / 0 = nothing, as it was
    #[k(10.0)] pub spall_out: f64,
    #[k(6.0)] pub spall_back: f64,
    /// (round two: 700 — one 3 m² facade sheet alone overran the live cap and its last clumps fell as nothing)
    #[k(520.0)] pub dice_per_m2: f64,
    #[k(6.0)] pub dice_min: f64,
    #[k(160.0)] pub dice_max: f64,
    #[k(45.0)] pub dice_life: f64,
    /// of finePerM2, the sparkle a diced clump adds (round two: 0.5)
    #[k(0.35)] pub dice_fine_share: f64,
    /// the crater at a blow (a fragment that comes with `grain`): grains per m² of it, and at most
    #[k(20000.0)] pub crater_per_m2: f64,
    #[k(48.0)] pub crater_max: f64,
    /// a round THROUGH a sheet: fines off the far face per 0.1 of energy the glass took (×0.5‥2), and their speed (m/s, + 5 × what the round has left)
    #[k(14.0)] pub exit_fines: f64,
    #[k(4.0)] pub exit_speed: f64,
    /// × every throw speed below (the feel knob)
    #[k(1.0)] pub speed: f64,
    /// ---- budget
    /// live shard particles (estimate; two fifths of the LONG ring's 10 240): counts thin out as it fills — see capFloor below (round two: 2560 — sized so ONE diced facade sheet is born whole)
    #[k(4000.0)] pub live_cap: f64,
    /// (round three) at the cap an emit still gets this many pieces, until the estimate is overCap × liveCap
    #[k(3.0)] pub cap_floor: f64,
    #[k(1.25)] pub over_cap: f64,
    /// (round three) a birth past pressFrom of the cap lives down to pressLife of its life (eased to the cap; 1 = as authored)
    #[k(0.3)] pub press_life: f64,
    #[k(0.35)] pub press_from: f64,
    /// emit records a frame from glass …
    #[k(24.0)] pub recs_per_frame: f64,
    /// … and how many more may WAIT for the next frames (past it a record is trimmed)
    #[k(96.0)] pub defer_max: f64,
    /// ---- plates
    /// m²: a fragment smaller than this is all particles
    #[k(0.012)] pub plate_min_area: f64,
    /// of the debris pool glass may hold
    #[k(0.6)] pub plate_share: f64,
    #[k(0.004)] pub child_min_area: f64,
    /// landing speed (m/s) from which a plate may break / always breaks
    #[k(1.6)] pub shatter_v0: f64,
    #[k(4.0)] pub shatter_v1: f64,
    /// (tuning round five) the most grains a plate sheds along its edges as it comes to rest, and their size in m (0 = none: a clear sheet on a floor is the floor's own colour and could not be seen lying there)
    #[k(12.0)] pub rest_crumbs: f64,
    #[k(0.009)] pub crumb_size: f64,
    /// CUT PLATES AS GLASS (the cut switch only; all picture, no ray — every number authored taste, judged on software-rendered stills, never on a GPU). All five at 0 /
    /// false = the plates as they first went in: an opaque box in the colour of what is under it.
    /// how far a plate's EDGE — its cut faces, 12 mm all round, and the rim below — is drawn toward the pale green-white of float glass seen through its edge (Box.cutCore; 0 = the sheet's own colour)
    #[k(1.0)] pub cut_edge: f64,
    /// the share of a plate's two faces left to what is behind it (a screen door in the cut pass: Box.cutGlass; 0 = opaque faces) The door steps in EIGHTHS (gbuffer_cut.wgsl): the share is rounded to the nearest eighth, so 0.94 and up is a face not drawn at all and under 0.0625 an opaque one.
    #[k(0.7)] pub cut_see: f64,
    /// pixels of rim on those faces drawn as edge: the outline is the bright thing, at any distance (0 = only the real 12 mm edge)
    #[k(1.25)] pub cut_rim: f64,
    /// the face's own colour = the pane's tint × this: dark, so what shows on it is its polish — reflections and glints (0 = the old stand-in: the colour of the surface under it × the tint)
    #[k(0.1)] pub cut_body: f64,
    /// the see-through share of a plate that has come to REST (it is looked at far longer lying than flying, and at 0.7 a lying plate was an outline with nothing inside; 0 = as in flight)
    #[k(0.45)] pub cut_see_rest: f64,
    /// at rest the face's colour is taken this far from the dark body toward the pale edge colour: a faint sheen, a sheet lying flat under the ceiling's light (0 = the dark body, as in flight)
    #[k(0.3)] pub cut_rest_body: f64,
    #[k(2.0)] pub max_gen: f64,
    /// glass on a hard floor: slides on, hops a little
    pub contact: ShardsKContact,
    /// ---- sound + hearing
    #[k(3.0)] pub sounds_per_frame: f64,
    #[k(48.0)] pub pending_max: f64,
    /// s between two plays of one voice (tuning round two: a diced sheet was 40‥80 'glassShatter's inside half a second — a wall of one sample); a landing inside the gap is
    ///  not played, it THICKENS the next one (≤ soundThicken more gain). {} = every landing its own sound, as it was
    #[k({ let mut t = JsMap::new(); t.set(String::from("glassTinkle"), 0.07); t.set(String::from("glassShatter"), 0.11); t })] pub sound_gap: JsMap<String, f64>,
    #[k(0.5)] pub sound_thicken: f64,
    #[k(0.6)] pub sound_owed_secs: f64,
    /// a landing's hearing level (= the base's LANDING.shard), 0.35 when a plate shatters
    #[k(0.3)] pub level: f64,
    /// s between hearing events from spray landings
    #[k(0.4)] pub event_gap: f64,
    /// m² of glass in a ½ m cell that is a full crunch
    #[k(0.012)] pub crunch_full: f64,
    /// a step on it (anybody's): level × (1 + gain·k) + add·k
    #[k(0.6)] pub crunch_gain: f64,
    #[k(0.12)] pub crunch_add: f64,
    /// of a cell's glass left after a step on it
    #[k(0.9)] pub grind: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct ShardsKContact {
    /// glass on a hard floor: slides on, hops a little
    #[k(0.72)] pub keep_floor: f64,
    #[k(0.85)] pub keep_wall: f64,
    #[k(0.3)] pub rest: f64,
}
