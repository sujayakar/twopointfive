pub mod types;
pub use types::*;
pub mod sig;
pub use sig::*;
// the parts' bodies: a named re-export wins over the stubs of sig.rs
pub mod p_01;
pub use p_01::{sigs, near3, glows, fits, find_car_lamps, car_lamp_targets, car_lamps_tape, lamp_glass, dead_lamp, car_lamp_paint};
