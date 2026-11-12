//! THE NAMED NODES OF A RIG and its lists of nodes: what a layer of the animator (rust/sim layers) knows them by.
use alloc::vec::Vec;

macro_rules! roles {
    ($n:ident, $names:ident; $($name:ident = $word:literal,)*) => {
        #[allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]
        #[repr(usize)]
        enum Place { $($name,)* N }
        $(pub const $name: usize = Place::$name as usize;)*
        pub const $n: usize = Place::N as usize;
        /// each by the word its node is kept under as a rig is read
        pub const $names: [&str; $n] = [$($word,)*];
    };
}
roles! { ROLES, ROLE_NAMES;
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

// A LIST is some nodes of the rig, parents first: joints a layer reads with every ancestor of theirs, so that a run forward of the list alone gives those joints what a whole run
// would; or a hand's fifteen finger joints.
pub const FK_ARMS: usize = 0;
pub const FK_LEGS: usize = 1;
pub const FK_HEAD: usize = 2;
pub const DIGITS_L: usize = 3;
pub const DIGITS_R: usize = 4;

/// which node of a rig plays which part: worked out as the rig is read
#[derive(Clone, Debug, Default)]
pub struct Roles { pub nodes: [u32; ROLES], pub lists: [Vec<u32>; 5] }
