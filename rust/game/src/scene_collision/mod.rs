pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod b_01;
pub mod p_01;
pub use p_01::{line_box, foot_band, no_number, rows_how, handed_back, new, add_static, foot_cells, foot_entries, truncate_statics, tick_open, stand, rules_world, rules_statics, rule_open, rule, rules_panes, fell_why, foot_of, tick_shut, window, ready, lone, wrote, aside, gather, gather_ask, gather_here, circle_touches, circle_touches_ask, circle_touches_here, collide_circle, collide_circle_ask, collide_circle_here, clamp_to_world, glass_open, glass_rule, glass_panes, glass_wrote, glass_viewed, pane_open, pane_ask, pane_fell, segment_blocked_dynamic, segment_blocked_dynamic_ask, things_ray, things_ask, things_both, compared, seg_clear, ray_box_hits, dynamic_hit, segment_blocked, segment_blocked_ask, own_lines, segment_blocked_here, solid_blocked, shot_blocked, raycast, raycast_ask, raycast_here, sense, sense_panes, eye_line, sight_open, sight_eye, eyes_see, eye_contact, eye_contact_ask, lamps_begin, lamps_over, lamps_part, lamp, lamps_on, blobs_begin, blobs_fit, blob, blobs_along, sense_for, grid_step, sound_paths_new, sound_paths_tick, sound_paths_invalidate, sound_paths_propagate, sound_paths_propagate_ask, nav_grid_new, nav_grid_rebake, nav_grid_idx, nav_grid_cell_blockers, nav_grid_is_blocked, nav_grid_find_path, nav_grid_there, nav_grid_find_path_ask, nav_grid_price_these, nav_grid_nearest_free, nav_grid_nearest_free_point, nav_grid_walkable, clampi};
