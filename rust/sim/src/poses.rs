//! THE POSES OF A GAME (src/anim/skeleton.ts Pose, PoseSet) — every local pose a character keeps from one tick to the next, in the game's block (state.rs), so
//! that a layer of the animator written here reads and writes them with no crossing and no copy, and a rollback puts them back with everything else.
//!
//! A POOL: a pose is ten columns of singles (skeleton.rs: tx ty tz | qx qy qz qw | sx sy sz, a node a row, the rows whole vectors of four: `singles`),
//! handed out one after another as the host asks, and
//! known by WHERE IT STARTS, a count of singles from the start of this part (so the host needs no number of this file's to find it). No ONE pose is handed back: a man keeps his for as long as he stands.
//! Men who leave a game for good (a restart makes its guards anew) would fill the pool in time, so whenever a man whose poses are here is no longer on the
//! game's lists the host takes every pose out, EMPTIES it (`clear_of`) and puts back the men who are there (game/posehome.ts): one rule for whoever left,
//! however he left. A pool FULL of men who are there is not emptied: the pose that did not fit stays the host's own array and is asked of the bench.
//! A pose is stored as the TypeScript's Float32Array stores it (single.rs).
use crate::single::S;
use crate::skeleton::singles;
use crate::state::state;
use core::mem::size_of;

/// singles the pool holds (2 MB a game: a man of the game keeps some seven poses of his rig's 55 nodes, 16 KB, so some 130 men). Every pose starts on a
/// 16-byte boundary: the part does (state.rs), HEAD is 16 bytes, and a pose is a whole number of vectors
pub const POOL: usize = 1 << 19;
/// the bytes before `data`
pub const HEAD: usize = 16;

#[repr(C)]
pub struct Poses {
    /// how many bytes of this part are in use, these included (0 in a block nobody has asked a pose of): what the host carries
    pub live: u32,
    pub spare: [u32; 3],
    pub data: [S; POOL],
}
const _: () = assert!(size_of::<Poses>() == HEAD + POOL * 4);

/// singles of `data` handed out so far
#[inline(always)]
pub fn used(p: &Poses) -> usize {
    let live = p.live as usize;
    if live <= HEAD {
        0
    } else {
        (live - HEAD) / 4
    }
}
/// room for a pose of `nodes` nodes in game `g`'s pool, zeros: where it starts (singles from the start of the part), or `u32::MAX` when the pool is full
/// (the pose stays the host's own)
pub fn new_of(g: usize, nodes: usize) -> u32 {
    let p = &mut state(g).poses;
    let (at, n) = (used(p), singles(nodes));
    if nodes == 0 || n > POOL - at {
        return u32::MAX;
    }
    let mut i = at;
    while i < at + n {
        p.data[i] = S::ZERO;
        i += 1;
    }
    p.live = (HEAD + (at + n) * 4) as u32;
    (HEAD / 4 + at) as u32
}
/// game `g`'s pool empty again, zeros (the host has taken every pose out of it first)
pub fn clear_of(g: usize) {
    let p = &mut state(g).poses;
    let u = used(p);
    let mut i = 0;
    while i < u {
        p.data[i] = S::ZERO;
        i += 1;
    }
    p.live = 0;
}
/// a pose of `nodes` nodes starting at single `at` of `data` is one this pool has handed out (whole, inside what is in use)
#[inline(always)]
pub fn holds(p: &Poses, at: usize, nodes: usize) -> bool {
    let u = used(p);
    at <= u && singles(nodes) <= u - at
}
