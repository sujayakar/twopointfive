pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{load_shards, save_shards, shards_live, u32, cell_name};
pub mod p_02;
pub use p_02::{glass_shards_pending, glass_shards_litter, glass_shards_new, glass_shards_release, glass_shards_spall, glass_shards_take, glass_shards_pane_at, glass_shards_dice_pane, glass_shards_plates, glass_shards_cut_plates, glass_shards_plate, glass_shards_crumble, glass_shards_crumbs, glass_shards_evicted, glass_shards_gone, glass_shards_landed, glass_shards_rested, glass_shards_shed, glass_shards_shed_cut, glass_shards_shatter, glass_shards_crossed, glass_shards_budget, glass_shards_emit, glass_shards_send, glass_shards_fall_of, glass_shards_schedule, glass_shards_landing, glass_shards_sound, glass_shards_heard, glass_shards_deposit, glass_shards_underfoot, glass_shards_step, glass_shards_update, glass_shards_on_release, glass_shards_reset, glass_shards_clear, glass_shards_live, glass_shards_plate_count, glass_shards_under_albedo, glass_shards_note, glass_shards_summary, glass_shards_test};
pub mod p_03;
pub use p_03::{clamp_i, edge_colour, cut_face_albedo, plate_albedo, frame_of, poly_area, tidy_poly, poly_centroid, clip_half, cell_parts, cut_plate_box, signed_area, fragment_of_box};
