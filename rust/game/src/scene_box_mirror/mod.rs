pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{leaf_wrote, thing_wrote, too_big, mirrors_shut, sense_rows, kind_of, kind_at, same, put, box_mirror_tables, box_mirror_boot, box_mirror_take, box_mirror_open, box_mirror_shut, box_mirror_outgrown, box_mirror_grid, box_mirror_statics, box_mirror_grid_dirty, box_mirror_grid_copy, box_mirror_grid_drop, box_mirror_wrote, box_mirror_debt, box_mirror_glass_begin, box_mirror_glass_end, box_mirror_glass_wrote, box_mirror_glass_is, box_mirror_owes_boxes, box_mirror_snow, box_mirror_again, box_mirror_wrote_grid, box_mirror_panes, box_mirror_pane, box_mirror_viewed, box_mirror_view_in, box_mirror_glass_in, box_mirror_glass_plain, box_mirror_leaves, box_mirror_things, box_mirror_thing_box, box_mirror_glass_bi, box_mirror_glass_t, box_mirror_rules, box_mirror_follow_leaf, box_mirror_follow_row, box_mirror_leaf_row, box_mirror_leaves_turned};
