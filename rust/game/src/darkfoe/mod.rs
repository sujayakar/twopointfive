pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{frac, id_share, dark_foe_carries, dark_foe_ruled, dark_foe_lit, mine, place_only, floor_light, dark_foe_blind, learns, has_word, click, lit_in_view, dark_foe_sees, in_room, threat_of, fenced, way_to, dark_foe_spot, walk, dark_foe_kept, dark_foe_tick, dark_foe_of, dark_foe_summary};
