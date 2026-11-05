pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{state_of, pic_of, picture_reset, flat, floor, name_of, up, pos_of, gone, made_out, fighter, foes_of, learn, look, look_sam, hear, on_net, tell, picture_tell, picture_tick, ideas_of, idea_weight, idea_reach, maybe_of, line_to, exposed_to, exposed_at, exposure_asker, believed_of, picture_quiet, exposed_for, known_places, any_idea, safest_sets, cover_near, picture_step, picture_step_of};
pub mod p_02;
pub use p_02::{picture_summary, idea_of};
