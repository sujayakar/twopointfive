//! The types of `cleared`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// a walk's price list, nav.findPath's penalty: a cell of a room nobody has been in costs `dear`, bar the walker's own room and his mark's; `also` rooms cost `dear` whatever is known. The price reads the room map and known rooms live from g.cleared and the grid from g.col.nav (cleared.ts dearFor)
#[derive(Clone, Debug)]
pub enum DearPrice {
    Rooms { dear: f64, from_room: f64, mark_rooms: Vec<f64>, also: Vec<f64> },
}

/// (TypeScript: ClearedRooms, src/game/cleared.ts:30)
#[derive(Clone, Debug)]
pub struct ClearedRooms {
    pub room_of_cell: Option<Vec<u16>>,
    pub known_room: Vec<u8>,
    pub sizes: Vec<f64>,
    pub last: f64,
    pub seeds: JsMap<DoorId, [f64; 2]>,
    pub stats: crate::cleared::ClearedRoomsStats,
    pub known: f64,
}
impl ClearedRooms {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
    pub fn blank() -> Self {
        ClearedRooms {
            room_of_cell: None,
            known_room: alloc::vec![0; 1],
            sizes: alloc::vec![0.0],
            last: -1.0,
            seeds: JsMap::new(),
            stats: crate::cleared::ClearedRoomsStats { round: 0.0, through: 0.0, face: 0.0, mouth: 0.0 },
            known: 0.0,
        }
    }
}
impl Default for ClearedRooms { fn default() -> Self { Self::blank() } }

/// (TypeScript: inline at src/game/cleared.ts:38 ClearedRooms.stats, src/game/cleared.ts:38)
#[derive(Clone, Copy, Debug)]
pub struct ClearedRoomsStats {
    pub round: f64,
    pub through: f64,
    pub face: f64,
    pub mouth: f64,
}
impl ClearedRoomsStats {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ClearedRoomsStats {
            round: 0.0,
            through: 0.0,
            face: 0.0,
            mouth: 0.0,
        }
    }
}
impl Default for ClearedRoomsStats { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/cleared.ts:79 unknownEyes ->, src/game/cleared.ts:79)
#[derive(Clone, Copy, Debug)]
pub struct ClearedRoomsUnknownEyesItem {
    pub x: f64,
    pub z: f64,
}
impl ClearedRoomsUnknownEyesItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        ClearedRoomsUnknownEyesItem {
            x: 0.0,
            z: 0.0,
        }
    }
}
impl Default for ClearedRoomsUnknownEyesItem { fn default() -> Self { Self::zeroed() } }

/// (src/game/cleared.ts:28 ROOM_MAX_CELLS)
pub const ROOM_MAX_CELLS: f64 = 2400.0;
