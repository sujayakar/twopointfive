pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{bare_row, handling_of};
pub mod p_02;
pub use p_02::{firearm_shell_fed, firearm_reload_row, firearm_new, firearm_name, firearm_cycle_mode, firearm_automatic, firearm_auto_spread, firearm_auto_fired, firearm_auto_idle, firearm_bottomless, firearm_reloading, firearm_rounds_ready, firearm_spent, firearm_can_fire, firearm_spare_sorted, firearm_fire, firearm_mag_room, firearm_start_reload, firearm_cancel_reload, firearm_plan_shells, firearm_plan_feed, firearm_feed_to, firearm_feed_shell, firearm_rearm, firearm_fit, firearm_update, pistol_new, rifle_new, rifle_attachment, rifle_left, rifle_cycle_attachment, ocp_ready, ocp_use, ocp_update};
