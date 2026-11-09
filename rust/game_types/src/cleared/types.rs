//! The types of `cleared`.
#![allow(unused_imports)]
use crate::{Blank, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// a walk's price list, nav.findPath's penalty: a cell of a room nobody has been in costs `dear`, bar the walker's own room and his mark's; `also` rooms cost `dear` whatever is known. The price reads the room map and known rooms live from g.cleared and the grid from g.col.nav (cleared.ts dearFor)
#[derive(Clone, Debug)]
pub enum DearPrice {
    Rooms { dear: f64, from_room: f64, mark_rooms: Vec<f64>, also: Vec<f64> },
}

#[derive(Clone, Debug, ToBytes, Blank)]
pub struct ClearedRooms {
    pub room_of_cell: Option<Vec<u16>>,
    #[is(alloc::vec![0; 1])] pub known_room: Vec<u8>,
    #[is(alloc::vec![0.0])] pub sizes: Vec<f64>,
    #[is(-1.0)] pub last: f64,
    pub seeds: JsMap<DoorId, [f64; 2]>,
    #[is(crate::cleared::ClearedRoomsStats { round: 0.0, through: 0.0, face: 0.0, mouth: 0.0 })] pub stats: crate::cleared::ClearedRoomsStats,
    pub known: f64,
}

#[derive(Clone, Copy, Debug, ToBytes, Default)]
pub struct ClearedRoomsStats {
    pub round: f64,
    pub through: f64,
    pub face: f64,
    pub mouth: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ClearedRoomsUnknownEyesItem {
    pub x: f64,
    pub z: f64,
}

/// (src/game/cleared.ts:28 ROOM_MAX_CELLS)
pub const ROOM_MAX_CELLS: f64 = 2400.0;
