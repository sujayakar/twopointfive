pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{net_quiet_lines, flat, live, fed, loud_of, side, ours, on_net, lost_to, mind, shock, pay, owe, speak, net_up, quiet_for, net_quiet_asked, net_quiet_closed, body_of, net_quiet_body, foe_in_view, net_quiet_fall, noise_of, whipping, arc_at, low_spot, free, last_kind, knows_squad, mine, posted, net_quiet_tick, rouse, ease, wary, settle};
pub mod p_02;
pub use p_02::{watch, net_quiet_holds, net_quiet_look, net_quiet_pose, net_quiet_spot, net_quiet_of, net_quiet_summary, n3};
