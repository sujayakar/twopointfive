//! src/sim/doorshome.ts lines 1 to 84
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// The doors' columns in the game's block: says how many doors there are and where the part starts; `home` is `g.sim`. (doorshome.ts DoorStore.constructor)
pub fn door_store_new(g: &crate::game::Game, home: crate::Opaque, cap: f64) -> crate::sim_doorshome::DoorStore {
    if cap > DOOR_CAP {
        panic!("a level of {} doors: the game holds {} (sim/doorshome.ts DOOR_CAP, rust/sim/src/doors.rs CAP)", cap, DOOR_CAP);
    }
    let base = (js::to_u32(crate::sim_state::sim_part(crate::sim_state::P_DOORS)) >> 3) as f64;
    DoorStore { base, stride: DOOR_CAP, home, cap }
}

/// Where the doors' hand-over tables are and how much they hold, asked once. An index is a byte address / 8. (doorshome.ts doorIo)
pub fn door_io(g: &mut crate::game::Game) -> crate::sim_doorshome::DoorIo {
    if let Some(io) = g.m.sim_doorshome.io {
        return io;
    }
    let x = crate::sim_sim::sim();
    if module_disagrees() {
        panic!(
            "rust/sim keeps {} doors of {} columns and the rules {} of {}",
            ::sim::doors::cap(0), ::sim::doors::cap(5), DOOR_CAP, DOOR_COLS
        );
    }
    let io = ask_io(x);
    g.m.sim_doorshome.io = Some(io);
    io
}

/// how many doubles wide a table of column numbers is: the Rust of `Object.keys(T).length`, these tables being all doubles
fn width<T>() -> f64 {
    (core::mem::size_of::<T>() / 8) as f64
}

/// doorIo's test that the module and this file count alike
fn module_disagrees() -> bool {
    ::sim::doors::cap(0) as f64 != DOOR_CAP
        || ::sim::doors::cap(5) as f64 != DOOR_COLS
        || ::sim::doors::cap(4) as f64 != DOOR_PARS
        || width::<DO>() != DOOR_OUTS
        || width::<DK>() != DOOR_CONS
}

/// doorIo's asking, of rust/sim's doors.rs
fn ask_io(x: crate::sim_sim::SimExports) -> crate::sim_doorshome::DoorIo {
    DoorIo {
        x,
        def: ::sim::doors::addr(0) as f64 / 8.0,
        out: ::sim::doors::addr(1) as f64 / 8.0,
        con: ::sim::doors::addr(2) as f64 / 8.0,
        ev: ::sim::doors::addr(3) as f64 / 8.0,
        par: ::sim::doors::addr(4) as f64 / 8.0,
        max_contacts: ::sim::doors::cap(2) as f64,
        max_events: ::sim::doors::cap(3) as f64,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn this_file_counts_as_the_module_does() {
        assert!(!module_disagrees());
    }

    #[test]
    fn an_index_is_an_address_over_eight_exactly() {
        let io = ask_io(crate::sim_sim::SimExports::default());
        assert_eq!((io.par * 8.0) as usize, ::sim::doors::addr(4));
        assert_eq!((io.def * 8.0) as usize, ::sim::doors::addr(0));
        assert_eq!(io.max_contacts as usize, ::sim::doors::MAXC);
    }
}
