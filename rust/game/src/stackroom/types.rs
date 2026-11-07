//! The types of `stackroom`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// what stackRoomBookKeep took, to put the room rule's book back once after an ask made only to be looked at (stackroom.ts:92). Forget: there was no book, so drop it. Back: forget the floors and doorway cells read since; floors marks which faces of which doors had been read (a floor is never rewritten once read, so marking is the same as the TypeScript's copied pairs, without cloning every Floor's W*H cells on each trial). sig and doors: if the book no longer has them it was rebuilt during the ask, and the TypeScript's restored old book would be rebuilt empty at the next bookOf, so set books to None
#[derive(Clone, Debug)]
pub enum StackRoomBookKeep {
    Forget,
    Back { sig: f64, doors: f64, near: bool, floors: JsMap<DoorId, [bool; 2]> },
}

/// (TypeScript: Floor, src/game/stackroom.ts:85)
#[derive(Clone, Debug)]
pub struct Floor {
    pub seed: f64,
    pub dist: Vec<i16>,
    pub cells: Vec<f64>,
}
impl Floor {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Floor {
            seed: 0.0,
            dist: Vec::new(),
            cells: Vec::new(),
        }
    }
}
impl Default for Floor { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: Book, src/game/stackroom.ts:86)
#[derive(Clone, Debug)]
pub struct Book {
    pub sig: f64,
    pub doors: f64,
    pub near: Option<Vec<u8>>,
    pub floors: JsMap<DoorId, [Option<Option<crate::stackroom::Floor>>; 2]>,
}
impl Book {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        Book {
            sig: 0.0,
            doors: 0.0,
            near: None,
            floors: JsMap::new(),
        }
    }
}
impl Default for Book { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/stackroom.ts:264 stackViewFix(men), src/game/stackroom.ts:264)
#[derive(Clone, Copy, Debug)]
pub struct StackViewFixMenItem {
    pub char: crate::stackroom::StackViewFixMenItemChar,
}
impl StackViewFixMenItem {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        StackViewFixMenItem {
            char: crate::stackroom::StackViewFixMenItemChar::zeroed(),
        }
    }
}
impl Default for StackViewFixMenItem { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/stackroom.ts:264 StackViewFixMenItem.char, src/game/stackroom.ts:264)
#[derive(Clone, Copy, Debug)]
pub struct StackViewFixMenItemChar {
    pub pos: V3,
}
impl StackViewFixMenItemChar {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        StackViewFixMenItemChar {
            pos: [0.0; 3],
        }
    }
}
impl Default for StackViewFixMenItemChar { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/stackroom.ts:350 nextRoom ->, src/game/stackroom.ts:350)
#[derive(Clone, Copy, Debug)]
pub struct NextRoom {
    pub pos: V3,
    pub face: f64,
}
impl NextRoom {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        NextRoom {
            pos: [0.0; 3],
            face: 0.0,
        }
    }
}
impl Default for NextRoom { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/game/stackroom.ts:372 stackRoomProbe ->, src/game/stackroom.ts:374)
#[derive(Clone, Copy, Debug)]
pub struct StackRoomProbe {
    pub floor: bool,
    pub cells: f64,
    pub seed: Option<V3>,
}
impl StackRoomProbe {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        StackRoomProbe {
            floor: false,
            cells: 0.0,
            seed: None,
        }
    }
}
impl Default for StackRoomProbe { fn default() -> Self { Self::zeroed() } }

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct StackroomMod {
    /// (src/game/stackroom.ts:87 books)
    pub books: Option<crate::stackroom::Book>,
}
impl StackroomMod {
    pub fn blank() -> Self {
        StackroomMod {
            books: None,
        }
    }
}

/// the counters this module keeps for the checks
#[derive(Clone, Debug, Default)]
pub struct StackroomStats {
    pub asked: i32,
    pub off: i32,
    pub other: i32,
    pub behind: i32,
    pub rear: i32,
    pub floor: i32,
    pub next: i32,
    pub squeezed: i32,
    pub lane: i32,
    pub none: i32,
    pub kept: i32,
    pub unled: i32,
    pub view_asked: i32,
    pub view_in: i32,
    pub view_across: i32,
    pub view_out: i32,
    pub view_over: i32,
    pub view_left: i32,
    pub view_stood: i32,
}

/// (src/game/stackroom.ts:114 ON_LINE)
pub const ON_LINE: f64 = 0.02;

/// (src/game/stackroom.ts:165 SAME_M)
pub const SAME_M: f64 = 0.1;
