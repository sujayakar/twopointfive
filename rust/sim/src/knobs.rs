//! THE KNOBS AND THE NAMED NODES RUST CODE READS — one list of each, by the name the TypeScript has for it.
//!
//! A knob is a number of a table the game registers (src/game/knobs.ts knobRegistry: `MV.gait.lean`), tuned in the panel, carried by a recording, put on or
//! off by a stand. It is CONFIG, the same for every game in the process, and no state of a game: so it is not in a game's block (state.rs) and no snapshot
//! carries it. Code here reads a knob by its line below (`k(GAIT_LEAN)`); the host reads the list once (`names`), finds each path in the registry (a path
//! that is not a number there is an error as the module starts work, and a red line in tools/qa/rust-check.ts), and writes every value as a tick opens
//! (src/game/rustknobs.ts). A part of the game that wants a knob in Rust adds a line; nothing else is written for it.
//!
//! A ROLE is a node of a character's rig by the name the animator has for it (src/game/character.ts `N`: `spine1` …). The host writes the node's index for
//! each line of that list when it puts a rig on the bench (skeleton.rs): `role(SPINE1)`. `NO_ROLE` until then.
//!
//! Both tables are copies the host proves, never state: what was written for the last tick is what this tick's first reader would find, so the host writes
//! before it asks.

macro_rules! table {
    ($ix:ident, $n:ident, $names:ident; $($name:ident = $path:literal,)*) => {
        #[allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]
        #[repr(usize)]
        enum $ix { $($name,)* N }
        $(pub const $name: usize = $ix::$name as usize;)*
        /// how many lines the list has
        pub const $n: usize = $ix::N as usize;
        /// the list as the host reads it: a name a line, in the order of the values
        pub const $names: &str = concat!($($path, "\n",)*);
    };
}

table! { Knob, KNOBS, KNOB_NAMES;
    MOVES_ON = "MOVES.on",
    MOVES_RECOIL = "MOVES.recoil",
    GAIT_LEAN = "MV.gait.lean",
    GAIT_AIM_LEAN = "MV.gait.aimLean",
    GAIT_BACK = "MV.gait.back",
    GAIT_CHIN = "MV.gait.chin",
    GAIT_SLOUCH_SWAY = "MV.gait.slouchSway",
    GAIT_SLOUCH_ROLL = "MV.gait.slouchRoll",
}

table! { Role, ROLES, ROLE_NAMES;
    SPINE1 = "spine1",
    SPINE2 = "spine2",
    SPINE3 = "spine3",
    NECK = "neck",
    HEAD = "head",
}

/// a role nobody has written a node for
pub const NO_ROLE: u32 = u32::MAX;

static mut KNOB: [f64; KNOBS] = [0.0; KNOBS];
static mut ROLE: [u32; ROLES] = [NO_ROLE; ROLES];

/// knob `i` of the list (a `pub const` above)
#[inline(always)]
pub fn k(i: usize) -> f64 {
    unsafe { (*core::ptr::addr_of!(KNOB))[i] }
}
/// a switch of a table that keeps its switches as numbers (MOVES: on above a half)
#[inline(always)]
pub fn on(i: usize) -> bool {
    k(i) > 0.5
}
/// the node of the rig on the bench that plays role `i`
#[inline(always)]
pub fn role(i: usize) -> usize {
    unsafe { (*core::ptr::addr_of!(ROLE))[i] as usize }
}
/// where a table is and how long: 0 the knobs' values (doubles) · 1 the knobs' names (bytes) · 2 the roles' nodes (u32) · 3 the roles' names (bytes)
pub fn addr(table: u32) -> usize {
    match table {
        0 => core::ptr::addr_of!(KNOB) as usize,
        1 => KNOB_NAMES.as_ptr() as usize,
        2 => core::ptr::addr_of!(ROLE) as usize,
        3 => ROLE_NAMES.as_ptr() as usize,
        _ => 0,
    }
}
pub fn len(table: u32) -> usize {
    match table {
        0 => KNOBS,
        1 => KNOB_NAMES.len(),
        2 => ROLES,
        3 => ROLE_NAMES.len(),
        _ => 0,
    }
}
