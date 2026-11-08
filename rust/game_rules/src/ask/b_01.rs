//! THE WORLD AS A QUESTION TAKES IT: the boxes as they stand, or as they would be. A question of the box world (a line, a round's line, a ray) is asked
//! `over` one of these, and the level is not changed to ask it: no list of leaves is taken out and put back, no flag is marked.
//!
//!   `Over::OWN`                the level as it stands, with its leaves
//!   `Over::statics()`          its statics alone: no door leaf is in the way
//!   `Over::but(&[leaf])`       as it stands, but these leaves let through (a leaf that is coming apart, the asker's own door)
//!   `.walls_open(g, &walls)`   as it would be once these walls had opened: what stands of them let through, what would be left of them there
//!
//! The questions: `scene_collision::{segment_blocked_over, solid_blocked_over, shot_blocked_over, raycast_over}`.
use crate::game::Game;
use crate::ids::*;
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

// ---- asked of the game shared

/// what a line of sight, or of anything else, is stopped by
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Line {
    /// what an eye does not pass: glass is passed
    Sight,
    /// anything solid, whole glass with it
    Solid,
}

/// the ways round an asker has found this tick, by the two cells of the floor they run between
#[derive(Clone, Debug, Default)]
pub struct Ways { t: f64, of: Vec<([i32; 4], crate::scene_collision::Propagation)> }

/// A QUESTION OF THE BOX WORLD BY SOMEBODY WHO MAY NOT WRITE THE GAME: the view, the page's camera, its sound. The copy must stand (it does from a tick's end to the
/// next one's start; `scene_collision::stands` after a writer that put it down), be this game's and of this moment, and owe nothing: else the asking is
/// refused by name, and never answered from other boxes.
pub struct WorldAsk<'g> {
    g: &'g Game,
    over: Over,
}
impl<'g> WorldAsk<'g> {
    /// the game it asks of
    pub fn shared(&self) -> &'g Game { self.g }
    pub fn of(g: &'g Game) -> WorldAsk<'g> { WorldAsk::try_of(g).unwrap_or_else(|why| panic!("a question of the box world asked of the game shared: {}", why)) }
    /// …or why it may not be asked
    pub fn try_of(g: &'g Game) -> Result<WorldAsk<'g>, &'static str> {
        crate::scene_box_mirror::box_mirror_stands(g)?;
        Ok(WorldAsk { g, over: Over::OWN })
    }
    /// the same, of another world
    pub fn over(self, over: Over) -> WorldAsk<'g> { WorldAsk { over, ..self } }
    /// is a→b stopped
    pub fn line(&self, a: crate::V3, b: crate::V3, what: Line) -> bool {
        let through = match what { Line::Sight => crate::scene_boxes::box_flag().glass, Line::Solid => 0.0 };
        crate::scene_collision::line_shared(self.g, &self.over, a, b, 0.02, through)
    }
    /// the nearest thing along a ray: how far, the face it is entered by, which (a static's place, 1e6 + a leaf's, 2e6 + a box's the question added)
    pub fn ray(&self, o: crate::V3, d: crate::V3, max: f64) -> Option<crate::scene_collision::Raycast> { crate::scene_collision::ray_shared(self.g, &self.over, o, d, max) }
    /// …one search for every way round between the same two places of the floor in one tick (`kept`: the asker's, never the game's): a burst of sounds behind one wall
    pub fn hear_kept(&self, kept: &mut Ways, src: crate::V3, dst: crate::V3) -> crate::scene_collision::Propagation {
        if kept.t != self.g.time { kept.t = self.g.time; kept.of.clear(); }
        let key = [src[0], src[2], dst[0], dst[2]].map(|v| crate::m::floor(v / self.g.col.nav.cell) as i32);
        crate::scene_collision::heard_shared_by(self.g, src, dst, |g, find| {
            if let Some((_, r)) = kept.of.iter().find(|(k, _)| *k == key) { return *r; }
            let r = find(g);
            if kept.of.len() < 64 { kept.of.push((key, r)); }
            r
        })
    }
    /// the way on the floor from a to b, as a man would be walked it, with what the module's search has as its scratch (None: the floor has none)
    pub fn way(&self, a: crate::V3, b: crate::V3, max_iter: u32) -> Option<alloc::vec::Vec<crate::V3>> { crate::scene_collision::way_shared(self.g, a, b, max_iter) }
    /// how a sound at `src` reaches `dst`, of the level as it stands
    pub fn hear(&self, src: crate::V3, dst: crate::V3) -> crate::scene_collision::Propagation { crate::scene_collision::heard_shared(self.g, src, dst) }
}

/// WHO ASKS A RULE'S QUESTIONS OF THE BOX WORLD. One body for a rule that the tick runs and a HUD is told (a can's roll, a panel between two points): the tick asks as
/// itself, paying what the copy owes as it goes; a question of the game shared asks through `WorldAsk`, the copy standing. A rule takes `impl Asker` and reads the game
/// by `a.game()`.
pub trait Asker {
    fn game(&self) -> &Game;
    /// the nearest thing along a ray
    fn ray(&mut self, o: crate::V3, d: crate::V3, max: f64) -> Option<crate::scene_collision::Raycast>;
    /// a door leaf across a→b; with `need`, one carrying those flags
    fn leaf_across(&mut self, a: crate::V3, b: crate::V3, need: Option<f64>) -> bool;
    /// the statics in a span: how many, their places in `out`
    fn gather(&mut self, x0: f64, z0: f64, x1: f64, z1: f64, y0: f64, y1: f64, out: &mut Vec<i32>) -> f64;
    /// the nearest part of the furniture along a ray: how far, and the face
    fn thing_ray(&mut self, o: crate::V3, d: crate::V3, max: f64) -> Option<(f64, crate::V3)>;
    /// how boggy the ground is at a place, 0 to 1
    fn bog_at(&mut self, x: f64, y: f64, z: f64) -> f64;
}
impl Asker for Game {
    fn game(&self) -> &Game { self }
    fn ray(&mut self, o: crate::V3, d: crate::V3, max: f64) -> Option<crate::scene_collision::Raycast> { crate::scene_collision::raycast(self, o, d, max) }
    fn leaf_across(&mut self, a: crate::V3, b: crate::V3, need: Option<f64>) -> bool { crate::scene_collision::segment_blocked_dynamic(self, a, b, need) }
    fn gather(&mut self, x0: f64, z0: f64, x1: f64, z1: f64, y0: f64, y1: f64, out: &mut Vec<i32>) -> f64 { crate::scene_collision::gather(self, x0, z0, x1, z1, y0, y1, out) }
    fn thing_ray(&mut self, o: crate::V3, d: crate::V3, max: f64) -> Option<(f64, crate::V3)> { crate::props::prop_system_raycast(self, o, d, max).map(|x| (x.t, x.n)) }
    fn bog_at(&mut self, x: f64, y: f64, z: f64) -> f64 { crate::snow_bog::snow_bog_at(self, x, y, z) }
}
impl<'g> Asker for WorldAsk<'g> {
    fn game(&self) -> &Game { self.g }
    fn ray(&mut self, o: crate::V3, d: crate::V3, max: f64) -> Option<crate::scene_collision::Raycast> { WorldAsk::ray(self, o, d, max) }
    fn leaf_across(&mut self, a: crate::V3, b: crate::V3, need: Option<f64>) -> bool { crate::scene_collision::leaf_shared(self.g, a, b, need) }
    fn gather(&mut self, x0: f64, z0: f64, x1: f64, z1: f64, y0: f64, y1: f64, out: &mut Vec<i32>) -> f64 { crate::scene_collision::gather_here(self.g, x0, z0, x1, z1, y0, y1, out) }
    fn thing_ray(&mut self, o: crate::V3, d: crate::V3, max: f64) -> Option<(f64, crate::V3)> { crate::scene_collision::things_shared(self.g, o, d, max) }
    fn bog_at(&mut self, x: f64, y: f64, z: f64) -> f64 { crate::snow_bog::snow_bog_shared(self.g, x, y, z) }
}
