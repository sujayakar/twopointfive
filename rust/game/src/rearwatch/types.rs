//! The types of `rearwatch`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: RearPin, src/game/rearwatch.ts:97)
#[derive(Clone, Copy, Debug)]
pub struct RearPin {
    pub o: CoverOpeningId,
    pub front: V3,
    pub door: DoorId,
    pub side: f64,
    pub op: Option<DoorOpId>,
    pub cls: f64,
    pub t0: f64,
    pub seen_t: f64,
    pub a: OrdersAssignmentId,
    pub going: Option<bool>,
    pub raised: Option<bool>,
    pub held: Option<bool>,
    pub late: Option<bool>,
    pub out: Option<bool>,
    pub to: Option<V3>,
}
impl RearPin {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RearPin {
            o: CoverOpeningId(0),
            front: [0.0; 3],
            door: DoorId(0),
            side: 0.0,
            op: None,
            cls: 0.0,
            t0: 0.0,
            seen_t: 0.0,
            a: OrdersAssignmentId(0),
            going: None,
            raised: None,
            held: None,
            late: None,
            out: None,
            to: None,
        }
    }
}
impl Default for RearPin { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: RoomKeep, src/game/rearwatch.ts:101)
#[derive(Clone, Debug)]
pub struct RoomKeep {
    pub pin: crate::rearwatch::RearPin,
    pub room: RoomfightRoomId,
    pub out_t: f64,
    pub behind: Option<Vec<GuardId>>,
    pub at: f64,
}
impl RoomKeep {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RoomKeep {
            pin: crate::rearwatch::RearPin::zeroed(),
            room: RoomfightRoomId(0),
            out_t: 0.0,
            behind: None,
            at: 0.0,
        }
    }
}
impl Default for RoomKeep { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at quiet RoomKeep.room, quiet RoomKeep.room)
#[derive(Clone, Copy, Debug)]
pub struct RoomKeepRoom {
}
impl RoomKeepRoom {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RoomKeepRoom {
        }
    }
}
impl Default for RoomKeepRoom { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Book, src/game/rearwatch.ts:102)
#[derive(Clone, Debug)]
pub struct Book {
    pub pins: JsMap<GuardId, crate::rearwatch::RearPin>,
    pub by_op: JsMap<DoorOpId, GuardId>,
    pub short: JsMap<OrdersAssignmentId, f64>,
    pub let_off: JsSet<OrdersAssignmentId>,
    pub log: Vec<String>,
    pub rows: f64,
    pub keeps: JsMap<GuardId, crate::rearwatch::RoomKeep>,
    pub was: Vec<(GuardId, DoorId, OrdersAssignmentId)>,
    pub shut: JsMap<DoorId, crate::rearwatch::BookShutValue>,
}
impl Book {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Book {
            pins: JsMap::new(),
            by_op: JsMap::new(),
            short: JsMap::new(),
            let_off: JsSet::new(),
            log: Vec::new(),
            rows: 0.0,
            keeps: JsMap::new(),
            was: Vec::new(),
            shut: JsMap::new(),
        }
    }
}
impl Default for Book { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/rearwatch.ts:105 Book.shut, src/game/rearwatch.ts:105)
#[derive(Clone, Copy, Debug)]
pub struct BookShutValue {
    pub o: CoverOpeningId,
    pub room: RoomfightRoomId,
    pub at: V3,
}
impl BookShutValue {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BookShutValue {
            o: CoverOpeningId(0),
            room: RoomfightRoomId(0),
            at: [0.0; 3],
        }
    }
}
impl Default for BookShutValue { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at quiet BookShutValue.room, quiet BookShutValue.room)
#[derive(Clone, Copy, Debug)]
pub struct BookShutValueRoom {
}
impl BookShutValueRoom {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        BookShutValueRoom {
        }
    }
}
impl Default for BookShutValueRoom { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Group, src/game/rearwatch.ts:133)
#[derive(Clone, Debug)]
pub struct Group {
    pub op: Option<DoorOpId>,
    pub door: DoorId,
    pub side: f64,
    pub men: Vec<GuardId>,
    pub marks: Vec<V3>,
    pub cands: Vec<GuardId>,
    pub going: bool,
}
impl Group {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Group {
            op: None,
            door: DoorId(0),
            side: 0.0,
            men: Vec::new(),
            marks: Vec::new(),
            cands: Vec::new(),
            going: false,
        }
    }
}
impl Default for Group { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/rearwatch.ts:116 rearWays ->, src/game/rearwatch.ts:116)
#[derive(Clone, Copy, Debug)]
pub struct RearWaysItem {
    pub o: CoverOpeningId,
    pub front: V3,
    pub cls: f64,
    pub d: f64,
}
impl RearWaysItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        RearWaysItem {
            o: CoverOpeningId(0),
            front: [0.0; 3],
            cls: 0.0,
            d: 0.0,
        }
    }
}
impl Default for RearWaysItem { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct RearwatchMod {
    /// (src/game/rearwatch.ts:106 books)
    pub books: JsMap<PlayerId, crate::rearwatch::Book>,
}
impl RearwatchMod {
    pub fn blank() -> Self {
        RearwatchMod {
            books: JsMap::new(),
        }
    }
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct RearwatchStats {
    pub ways: i32,
    pub pinned: i32,
    pub no_line: i32,
    pub pins: i32,
    pub held: i32,
    pub late: i32,
    pub last: i32,
    pub rooms: i32,
    pub room_late: i32,
    pub room_ways: i32,
    pub room_far: i32,
}

/// (src/game/rearwatch.ts:109 EYE)
pub const EYE: f64 = 1.55;

/// (src/game/rearwatch.ts:109 CHEST)
pub const CHEST: f64 = 1.1;

/// (src/game/rearwatch.ts:110 PRE: a table nothing writes)
pub fn pre() -> Vec<String> { alloc::vec![String::from("stack"), String::from("ready"), String::from("place"), String::from("standoff"), String::from("armed")] }

/// (src/game/rearwatch.ts:110 GOING: a table nothing writes)
pub fn going() -> Vec<String> { alloc::vec![String::from("kick"), String::from("toss"), String::from("fuse"), String::from("wait"), String::from("enter")] }

/// (src/game/rearwatch.ts:110 ENTRIES: a table nothing writes)
pub fn entries() -> Vec<String> { alloc::vec![String::from("openClear"), String::from("bangClear"), String::from("charge")] }

/// (src/game/rearwatch.ts:112 SLOW: a table nothing writes)
pub fn slow() -> Vec<String> { alloc::vec![String::from("toss"), String::from("fuse"), String::from("wait")] }

/// (src/game/rearwatch.ts:112 FORMED_M)
pub const FORMED_M: f64 = 1.5;

/// (src/game/rearwatch.ts:239 FALLING)
pub const FALLING: &str = "falling in";
