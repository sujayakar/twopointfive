pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{mat_of_surf, mat_of, core_of, face_core, lerp, slab_outline, wedge_outline, snapped_outline, strip_outline, outline_of, shape_box, ragged_cut, wall_normal};
pub mod p_02;
pub use p_02::{breaks_new, breaks_hash, breaks_phash, breaks_reset, breaks_small, breaks_rel, breaks_share, breaks_load, breaks_lands, breaks_born, breaks_tag, breaks_landed, breaks_kicked, breaks_evicted, breaks_pick, breaks_dissolve, breaks_tick, breaks_tint, breaks_chips, breaks_fines, breaks_crumble, breaks_refuse, breaks_break_again, breaks_dress, breaks_outline, breaks_shape_loose, breaks_retag, breaks_sheet, breaks_carcass, breaks_kink, breaks_shed_cell, breaks_heap, breaks_end_core, breaks_ragged, breaks_studs, breaks_summary};
