//! THE BRING-UP'S OWN PART of frag: bodies for skeleton stubs a tick reached (bringup2-log.md says which is a boundary no-op and which is translated)
#![allow(unused_imports, unused_variables)]
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// the flee script frag.ts lent for the frame, as the run left it, put back in the book it came from: the first live grenade's book
/// that has a script for him, as frag_flee finds it (conventions 4).
pub fn frag_script_ran(g: &mut crate::game::Game, gd: GuardId, ran: Option<crate::game::GuardScript>) {
    let Some(ran) = ran else { return };
    let Some(books) = g.m.frag.books.as_mut() else { return };
    for b in books.iter_mut() { if let Some(row) = b.flee.get_mut(gd) { if row.is_some() { *row = Some(ran); return; } } }
}
