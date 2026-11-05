pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{structure_of_level, levers_now, fog_off_now, any_set, is_rung, preset_levers, difficulty_here, same, tables_of, difficulty_now, levers_shown, tables_stand, cone_of, level_of_three, fog_stands, shown_whole, levers_set, levers_base, levers_mixed, write, store_name, kept_hidden, store_levers, set_difficulty, set_base, adopt, own, set_lever, levers_fog, set_cone_visit, cone_visit_now, difficulty_custom, init_difficulty, kept_levers, levers_kept, difficulty_href, levers_href, squads_of, squads_held, difficulty_words, lever_words, levers_line};
