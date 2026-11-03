//! THE GAME'S STATE IN THE MODULE — everything of a game that lives in this memory from one call to the next is ONE BLOCK of bytes a game, and it is here.
//!
//! WHY ONE BLOCK. State of the game has to be taken and put back (a rollback, a save kept in memory), put into another game, folded into the state's word and
//! compared (src/game/rollback.ts), and one day sent. A block is all of those with no line a system: the host copies its bytes. So:
//!   · a family of the update that keeps state here adds ONE FIELD to `State` (a struct of flat, fixed-size columns) and its row in `PARTS`. Nothing else in
//!     the crate is state: every other `static mut` is SCRATCH (written and read inside one call) or a COPY the host proves (the box world), and
//!     tools/sim/build.ts is red on a `static mut` it has no row for (STATICS there).
//!   · NO ADDRESS IN A BLOCK, ONLY INDICES. A block is the same bytes in whichever slot it stands, in the host's own memory while another game has the slot,
//!     in a file, on another machine. A pointer kept in one is wrong the moment it moves.
//!   · NO PADDING THE COMPILER CHOSE. Every byte of a block belongs to a named field (a `spare` where the alignment wants one), so the bytes are the state
//!     and nothing else: each part asserts its size is the sum of its fields.
//!   · ZEROS ARE A GAME WITH NOTHING IN IT. A new block is zeros with its head written; a part reads its zeros as "empty".
//!   · no allocation: `GAMES` blocks, a fixed size each, so the module's memory never grows.
//!   · A POOL SAYS HOW MUCH OF IT IS IN USE. A part that hands out room as a game asks (the poses, the smoke grid's bricks) is sized for the largest game, and its FIRST WORD is how
//!     many of its bytes are in use, that word included. The host carries, compares and folds only those: what lies past them is nobody's state.
//!
//! WHO HOLDS A SLOT is the host's business (src/sim/state.ts SimState: a game's block is bound to a slot here, or parked in the host's memory when more than
//! `GAMES` games are alive at once). A call that reads or writes state names its slot, `g`.
use crate::bricks::Smoke;
use crate::dice::Dice;
use crate::poses::Poses;
use crate::doors::Doors;
use crate::falls::Falls;
use core::mem::{offset_of, size_of};

/// games whose blocks the module holds at once
pub const GAMES: usize = 4;

/// The head of a block.
#[repr(C)]
pub struct Head {
    /// the word of the layout this block was written under (`LAYOUT`): the host writes it into a new block
    pub layout: u32,
    /// WHICH SYSTEMS' STATE IS AT HOME HERE, a bit a system (the host's table, src/sim/state.ts): a system whose bit is down keeps its state in the
    /// TypeScript objects, as it did before it moved, and the part it would use here is zeros
    pub homes: u32,
    pub spare: [u32; 2],
}
const _: () = assert!(size_of::<Head>() == 16);

/// A game's state in the module: the head, then a field a family.
#[repr(C, align(16))]
pub struct State {
    pub head: Head,
    pub dice: Dice,
    pub doors: Doors,
    pub falls: Falls,
    pub smoke: Smoke,
    pub poses: Poses,
}

/// A part of the block: where it is, how long, and the version of what is in it (moved on by whoever changes a part's fields without changing its length).
pub struct Part {
    pub off: usize,
    pub len: usize,
    pub ver: u32,
    /// a pool: its first word is how many of its bytes are in use (the host carries only those)
    pub pool: bool,
}
/// THE PARTS, in the order of the fields (src/sim/state.ts SIM_PARTS names them in the same order). THE ORDER IS THE PARKED BLOCK'S COST: a block that
/// gives its slot up is kept by the host as ONE run of bytes from its start to the end of the last word in use, so the room a pool does not use is
/// paid for by every part that stands after it. Parts of a fixed length first, then the pools, the one sized largest LAST.
pub const PARTS: [Part; 6] = [
    Part { off: offset_of!(State, head), len: size_of::<Head>(), ver: 1, pool: false },
    Part { off: offset_of!(State, dice), len: size_of::<Dice>(), ver: 1, pool: false },
    Part { off: offset_of!(State, doors), len: size_of::<Doors>(), ver: 1, pool: false },
    Part { off: offset_of!(State, falls), len: size_of::<Falls>(), ver: 1, pool: false },
    Part { off: offset_of!(State, smoke), len: size_of::<Smoke>(), ver: 1, pool: true },
    Part { off: offset_of!(State, poses), len: size_of::<Poses>(), ver: 1, pool: true },
];
/// a block's length in bytes
pub const LEN: usize = size_of::<State>();
// the parts are the whole block, end to end: no byte between two of them and none after the last
const _: () = {
    let mut at = 0;
    let mut i = 0;
    while i < PARTS.len() {
        assert!(PARTS[i].off == at);
        assert!(PARTS[i].len % 8 == 0);
        at += PARTS[i].len;
        i += 1;
    }
    assert!(at == LEN);
};

/// The layout's word: the parts (where, how long, which version) folded. Two builds with the same word lay a block out alike.
pub const LAYOUT: u32 = {
    let mut h: u32 = 0x811c9dc5;
    let mut i = 0;
    while i < PARTS.len() {
        h = (h ^ PARTS[i].off as u32).wrapping_mul(0x01000193);
        h = (h ^ PARTS[i].len as u32).wrapping_mul(0x01000193);
        h = (h ^ (PARTS[i].ver | (PARTS[i].pool as u32) << 31)).wrapping_mul(0x01000193);
        i += 1;
    }
    h
};

// (zeros are a valid block: every field of every part is a number or an array of numbers)
static mut ALL: [State; GAMES] = unsafe { core::mem::zeroed() };

/// the block in slot `g` (one thread; the host binds a game to a slot before it calls). A slot that is not one TRAPS: answering for another game's
/// block would move that game's state and hand the caller a draw that looks right.
#[inline(always)]
pub fn state(g: usize) -> &'static mut State {
    unsafe { &mut (*core::ptr::addr_of_mut!(ALL))[g] }
}
/// where slot `g`'s block is (a byte address in the module's memory)
pub fn addr(g: usize) -> usize {
    state(g) as *mut State as usize
}
