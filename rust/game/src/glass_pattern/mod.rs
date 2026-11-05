pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{gp_hash, gp_jit, wrap, gp_bound, gp_angle, gp_rho, gp_chord_pt, gp_reach, gp_merged, gp_split, gp_cut, gp_nearest, beyond, gp_sector, gp_sector_band, gp_fid, gp_unfid, gp_classify, clip, isect, rect_poly, gp_region, gp_fragments, line_dist, gp_crack_at, gp_dice_centre, gp_dice_clump, gp_dice_key, gp_dice_poly, gp_dice_edge, gp_hull, gp_clip_rect, gp_poly_area};
