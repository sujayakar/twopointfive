//! The numbers of wallbreak.
use crate::bytes::ToBytes;
use crate::js::*;
use alloc::string::String;
use game_derive::Knobs;

/// ------------------------------------------------------------------ knobs
#[derive(Clone, Debug, ToBytes, Knobs)]
pub struct WallsK {
    /// what counts as a partition wall
    #[k(0.17)] pub wall_max_thick: f64,
    #[k(2.2)] pub wall_min_top: f64,
    #[k(0.5)] pub wall_min_len: f64,
    /// m: a wall column / a cover cell (the nearest whole division of the box)
    #[k(0.5)] pub col: f64,
    /// m: what stands of a column after a port
    #[k(0.7)] pub sill: f64,
    /// rifle rounds into ONE cell at chaos 0.5 before it goes (a pistol round counts pistolK); × sillK for a wall's sill
    #[k({ let mut t = JsMap::new(); t.set(String::from("drywall"), 6.0); t.set(String::from("wood"), 9.0); t.set(String::from("cardboard"), 3.0); t.set(String::from("fabric"), 4.0); t })] pub tough: JsMap<String, f64>,
    #[k(1.5)] pub sill_k: f64,
    #[k(0.5)] pub pistol_k: f64,
    /// cover: tall enough to matter, big enough to stand behind
    #[k(0.45)] pub cover_min_top: f64,
    #[k(0.4)] pub cover_min_len: f64,
    #[k(48.0)] pub cover_max_cells: f64,
    /// a blast of radius R (× power): columns within holeK·R go whole, within portK·R lose their upper; cover cells within coverK·R go; a hole is widened to minHole
    ///  columns when the charge lay within contactM of the wall (a man must fit); doorK·R is the tear distance handed to the doors' own g.doors.blast
    pub blast: WallsKBlast,
    /// caps
    #[k(512.0)] pub max_pieces: f64,
    #[k(256.0)] pub max_changes: f64,
    #[k(2.0)] pub chunks_per_cell: f64,
    #[k(0.5)] pub fall_max_vol: f64,
    #[k(48.0)] pub falls_per_change: f64,
    /// what counts as a BOARD hung on a wall for a charge set on it (`hungAt`): no deeper off the face than `deep`, at least `long` along the wall and `tall` (m)
    pub hung: WallsKHung,
}

/// a blast of radius R (× power): columns within holeK·R go whole, within portK·R lose their upper; cover cells within coverK·R go; a hole is widened to minHole
///  columns when the charge lay within contactM of the wall (a man must fit); doorK·R is the tear distance handed to the doors' own g.doors.blast
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct WallsKBlast {
    #[k(2.2)] pub radius: f64,
    #[k(0.45)] pub hole_k: f64,
    #[k(0.8)] pub port_k: f64,
    #[k(0.7)] pub cover_k: f64,
    #[k(0.6)] pub door_k: f64,
    #[k(3.0)] pub min_hole: f64,
    #[k(0.8)] pub contact_m: f64,
}

/// what counts as a BOARD hung on a wall for a charge set on it (`hungAt`): no deeper off the face than `deep`, at least `long` along the wall and `tall` (m)
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct WallsKHung {
    #[k(0.1)] pub deep: f64,
    #[k(0.5)] pub long: f64,
    #[k(0.3)] pub tall: f64,
}
