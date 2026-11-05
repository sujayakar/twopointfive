pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{flags, name_of, look_of, pair_orders_new, pair_orders_up, pair_orders_quiet, pair_orders_is_foe, pair_orders_noticed, pair_orders_waiting, pair_orders_job_of, pair_orders_gripped, pair_orders_caught, pair_orders_finishing, pair_orders_last_paces, pair_orders_gone, pair_orders_sees, pair_orders_rear, pair_orders_note, pair_orders_rows, pair_orders_go_row, pair_orders_quiet_first, pair_orders_seat, pair_orders_reset, pair_orders_input, pair_orders_tick, pair_orders_mark, pair_orders_watcher, pair_orders_body_lit, pair_orders_looks_back, pair_orders_held, pair_orders_stand, pair_orders_yaw_of, pair_orders_back, pair_orders_back_open, pair_orders_behind, pair_orders_stage, pair_orders_take_tick, pair_orders_clear_of, pair_orders_stalk, pair_orders_lit_cell, pair_orders_lit_at, pair_orders_dark_spot, pair_orders_dark_tick, pair_orders_summary};
pub mod p_02;
pub use p_02::{pair_passes, pair_grip_body, pair_gripped, pair_holds};
