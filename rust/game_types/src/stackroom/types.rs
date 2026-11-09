//! The types of `stackroom`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// what stackRoomBookKeep took, to put the room rule's book back once after an ask made only to be looked at. Forget: there was no book, so drop it. Back: forget the floors and doorway cells read since; floors marks which faces of which doors had been read (a floor is never rewritten once read, so marking is the same as the TypeScript's copied pairs, without cloning every Floor's W*H cells on each trial). sig and doors: if the book no longer has them it was rebuilt during the ask, and the TypeScript's restored old book would be rebuilt empty at the next bookOf, so set books to None
#[derive(Clone, Debug)]
pub enum StackRoomBookKeep {
    Forget,
    Back { sig: f64, doors: f64, near: bool, floors: JsMap<DoorId, [bool; 2]> },
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Floor {
    pub seed: f64,
    pub dist: Vec<i16>,
    pub cells: Vec<f64>,
}

#[derive(Clone, Debug, ToBytes, Default)]
pub struct Book {
    pub sig: f64,
    pub doors: f64,
    pub near: Option<Vec<u8>>,
    pub floors: JsMap<DoorId, [Option<Option<crate::stackroom::Floor>>; 2]>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct NextRoom {
    pub pos: V3,
    pub face: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct StackRoomProbe {
    pub floor: bool,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, ToBytes)]
pub struct StackroomMod {
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

pub const ON_LINE: f64 = 0.02;

pub const SAME_M: f64 = 0.1;
