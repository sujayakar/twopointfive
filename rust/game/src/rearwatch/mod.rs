pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{book_of, is_open_way, rear_ways, rear_watch_tick, note, room_keep_tick, walk_to, room_kept, held_by_other, rear_sees, rear_pin_of, rear_pin, rear_holds, rear_last, rear_watcher, rear_watch_log, rear_watch_rows, rear_watch_summary};
