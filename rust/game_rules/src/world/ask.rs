//! THE WORLD AS A QUESTION TAKES IT: the boxes as they stand, or as they would be. A question of the box world (a line, a round's line, a ray) is asked
//! `over` one of these, and the level is not changed to ask it: no list of leaves is taken out and put back, no flag is marked.
//!
//!   `Over::OWN`                the level as it stands, with its leaves
//!   `Over::statics()`          its statics alone: no door leaf is in the way
//!   `Over::but(&[leaf])`       as it stands, but these leaves let through (a leaf that is coming apart, the asker's own door)
//!   `.walls_open(g, &walls)`   as it would be once these walls had opened: what stands of them let through, what would be left of them there
//!
//! The questions: `scene_collision::{segment_blocked_over, solid_blocked_over, shot_blocked_over, raycast_over}`. A question takes the game shared: asking changes nothing.

use crate::game::Game;
use crate::ids::*;
use crate::m;
use crate::v3::V3;
use alloc::vec::Vec;

/// which door leaves are in a question's way
#[derive(Clone, Debug, PartialEq)]
pub enum Leaves {
    /// the game's own, as they stand
    Own,
    /// none
    None,
    /// the game's own but these
    But(Vec<BoxId>),
}

#[derive(Clone, Debug)]
pub struct Over {
    pub leaves: Leaves,
    /// statics let through, by their place in the level's boxes
    pub open: Vec<u32>,
    /// boxes that are none of the level's and stand for this question
    pub extra: Vec<BoxId>,
}

impl Over {
    pub const OWN: Over = Over { leaves: Leaves::Own, open: Vec::new(), extra: Vec::new() };
    pub fn statics() -> Over { Over { leaves: Leaves::None, ..Over::OWN } }
    pub fn but(leaves: &[BoxId]) -> Over { Over { leaves: if leaves.is_empty() { Leaves::Own } else { Leaves::But(leaves.to_vec()) }, ..Over::OWN } }
    pub fn but_door(g: &Game, d: DoorId) -> Over { Over::but(&[g.dr[d].box_]) }
    /// …as it would be once `walls` had opened
    pub fn walls_open(mut self, g: &Game, walls: &[crate::wallbreak::WallOpening]) -> Over {
        for w in walls {
            for b in &w.standing { if let Some(i) = g.col.boxes.iter().position(|x| x == b) { self.open.push(i as u32); } }
            self.extra.extend_from_slice(&w.left);
        }
        self
    }
    /// nothing of it differs from the level as it stands
    pub fn plain(&self) -> bool { self.leaves == Leaves::Own && self.open.is_empty() && self.extra.is_empty() }
}

// ---- what a host keeps between its questions

/// the ways round an asker has found this tick, by the two cells of the floor they run between
#[derive(Clone, Debug, Default)]
pub struct Ways { t: f64, of: Vec<([i32; 4], crate::scene_collision::Propagation)> }

/// how a sound at `src` reaches `dst`, for a host's own ears: one search for every way round between the same two places of the floor in one tick (`kept`: the asker's, never the
/// game's), for a burst of sounds behind one wall
pub fn hear_kept(g: &Game, kept: &mut Ways, src: V3, dst: V3) -> crate::scene_collision::Propagation {
    if kept.t != g.time { kept.t = g.time; kept.of.clear(); }
    let key = [src[0], src[2], dst[0], dst[2]].map(|v| m::floor(v / g.col.nav.cell) as i32);
    crate::scene_collision::heard_by(g, src, dst, |g, find| {
        if let Some((_, r)) = kept.of.iter().find(|(k, _)| *k == key) { return *r; }
        let r = find(g);
        if kept.of.len() < 64 { kept.of.push((key, r)); }
        r
    })
}
