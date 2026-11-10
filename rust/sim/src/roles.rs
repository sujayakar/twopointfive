//! THE NAMED NODES OF A RIG, its lists of nodes and its numbers, by the animator's names for them: written with the rig, as it is put on the bench.
//!
//! A ROLE is a node by name (`role(SPINE1)`), `NO_ROLE` until a rig is on the bench (skeleton.rs).
//!
//! Each is a copy, never state: whoever puts a rig on the bench writes them before anybody asks.

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

table! { Role, ROLES, ROLE_NAMES;
    SPINE1 = "spine1",
    SPINE2 = "spine2",
    SPINE3 = "spine3",
    NECK = "neck",
    HEAD = "head",
    SHOULDER_L = "shoulderL",
    SHOULDER_R = "shoulderR",
    UPPER_ARM_L = "upperArmL",
    UPPER_ARM_R = "upperArmR",
    HIPS = "hips",
    THIGH_L = "thighL",
    SHIN_L = "shinL",
    FOOT_L = "footL",
    THIGH_R = "thighR",
    SHIN_R = "shinR",
    FOOT_R = "footR",
    TOE_L = "toeL",
    TOE_R = "toeR",
    HAND_R = "handR",
    HAND_L = "handL",
    FOREARM_L = "forearmL",
    FOREARM_R = "forearmR",
    MIDDLE1_R = "middle1R",
}

// A LIST is some nodes of the rig in the rig's own parents-first order, by the animator's name for it (character.ts fkList: joints a layer reads, with every
// ancestor of theirs, so that a run forward of the list alone gives those joints what a whole run would). Written with the roles.
table! { List, LISTS, LIST_NAMES;
    FK_ARMS = "fkArms",
    FK_LEGS = "fkLegs",
    FK_HEAD = "fkHead",
    DIGITS_L = "digitsL",
    DIGITS_R = "digitsR",
}

// A rig's NUMBERS are what the animator worked out once from the character file and reads every frame (character.ts cast.nums: a table of doubles by the
// animator's name for it). Written with the roles, for the rig on the bench.
table! { Num, NUMS, NUM_NAMES;
    JOG_SEAM = "jogSeam",
}

/// a role nobody has written a node for
pub const NO_ROLE: u32 = u32::MAX;

static mut ROLE: [u32; ROLES] = [NO_ROLE; ROLES];
/// a list holds no more nodes than a rig the bench holds (skeleton.rs MAX_NODES)
pub const LIST_MAX: usize = 128;
static mut LIST: [[u32; LIST_MAX]; LISTS] = [[0; LIST_MAX]; LISTS];
static mut LIST_LEN: [u32; LISTS] = [0; LISTS];
/// a table of a rig's numbers holds no more doubles than this (a table that is longer is a rig the module's layers are not asked about)
pub const NUM_MAX: usize = 1024;
static mut NUM: [[f64; NUM_MAX]; NUMS] = [[0.0; NUM_MAX]; NUMS];
static mut NUM_LEN: [u32; NUMS] = [0; NUMS];

/// the node of the rig on the bench that plays role `i`
#[inline(always)]
pub fn role(i: usize) -> usize {
    unsafe { (*core::ptr::addr_of!(ROLE))[i] as usize }
}
/// list `i`: its nodes (none until the host has written it; never more than it holds)
#[inline(always)]
pub fn list(i: usize) -> &'static [u32] {
    unsafe {
        let n = (*core::ptr::addr_of!(LIST_LEN))[i] as usize;
        &(&(*core::ptr::addr_of!(LIST))[i])[..if n > LIST_MAX { 0 } else { n }]
    }
}
/// table `i` of the rig's numbers (none until the host has written it; never more than it holds)
#[inline(always)]
pub fn nums(i: usize) -> &'static [f64] {
    unsafe {
        let n = (*core::ptr::addr_of!(NUM_LEN))[i] as usize;
        &(&(*core::ptr::addr_of!(NUM))[i])[..if n > NUM_MAX { 0 } else { n }]
    }
}
/// where a table is and how long: 2 the roles' nodes (u32) · 3 the roles' names (bytes) ·
/// 4 the lists' nodes (u32, LIST_MAX a list) · 5 the lists' names (bytes) · 6 the lists' lengths (u32) · 7 the rig's numbers (doubles, NUM_MAX a table) ·
/// 8 their names (bytes) · 9 their lengths (u32)
pub fn addr(table: u32) -> usize {
    match table {
        2 => core::ptr::addr_of!(ROLE) as usize,
        3 => ROLE_NAMES.as_ptr() as usize,
        4 => core::ptr::addr_of!(LIST) as usize,
        5 => LIST_NAMES.as_ptr() as usize,
        6 => core::ptr::addr_of!(LIST_LEN) as usize,
        7 => core::ptr::addr_of!(NUM) as usize,
        8 => NUM_NAMES.as_ptr() as usize,
        9 => core::ptr::addr_of!(NUM_LEN) as usize,
        _ => 0,
    }
}
pub fn len(table: u32) -> usize {
    match table {
        2 => ROLES,
        3 => ROLE_NAMES.len(),
        4 => LISTS * LIST_MAX,
        5 => LIST_NAMES.len(),
        6 => LISTS,
        7 => NUMS * NUM_MAX,
        8 => NUM_NAMES.len(),
        9 => NUMS,
        _ => 0,
    }
}
