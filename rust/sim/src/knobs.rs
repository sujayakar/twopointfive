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
    MOVES_STEADY = "MOVES.steady",
    MOVES_STEADY_RUN = "MOVES.steadyRun",
    LONG_STEADY_RUN_YAW = "LONG_STEADY.runYaw",
    HARDBREATH_ON = "HARDBREATH.on",
    HARDBREATH_CHEST_AMP = "HARDBREATH.chestAmp",
    HARDBREATH_CHEST_RATE = "HARDBREATH.chestRate",
    HARDBREATH_CHEST_UP = "HARDBREATH.chestUp",
    HARDBREATH_CHEST_DOWN = "HARDBREATH.chestDown",
    LIFE_ON = "LIFE.on",
    LIFE_BREATH_RATE = "LIFE.breathRate",
    LIFE_BREATH_RATE_RUN = "LIFE.breathRateRun",
    LIFE_BREATH_AMP = "LIFE.breathAmp",
    LIFE_BREATH_AMP_RUN = "LIFE.breathAmpRun",
    LIFE_MASS_GAIN = "LIFE.massGain",
    LIFE_MASS_BACK = "LIFE.massBack",
    LIFE_MASS_FWD = "LIFE.massFwd",
    LIFE_MASS_DIP = "LIFE.massDip",
    LIFE_MASS_DIP_MAX = "LIFE.massDipMax",
    LIFE_TIP_GAIN = "LIFE.tipGain",
    LIFE_TIP_BACK = "LIFE.tipBack",
    LIFE_TIP_FWD = "LIFE.tipFwd",
    LIFE_REST_AFTER = "LIFE.restAfter",
    LIFE_SHIFT_MIN = "LIFE.shiftMin",
    LIFE_SHIFT_SPAN = "LIFE.shiftSpan",
    LIFE_SHIFT_TRAVEL = "LIFE.shiftTravel",
    LIFE_SHIFT_IN = "LIFE.shiftIn",
    LIFE_SHIFT_OUT = "LIFE.shiftOut",
    LIFE_HIP_DROP = "LIFE.hipDrop",
    LIFE_SPINE_COUNTER = "LIFE.spineCounter",
    LIFE_NECK_COUNTER = "LIFE.neckCounter",
    FEET_ON = "FEET.on",
    FEET_SWING = "FEET.swing",
    FEET_LIFT = "FEET.lift",
    FEET_DIST = "FEET.dist",
    FEET_TURN_DEG = "FEET.turnDeg",
    FEET_PIVOT = "FEET.pivot",
    FEET_LAG_CAP_DEG = "FEET.lagCapDeg",
    MOVES_ENTRY = "MOVES.entry",
    MOVES_GAIT = "MOVES.gait",
    RECOIL_BACK_TAU_0 = "MV.recoil.backTau.0",
    RECOIL_BACK_TAU_1 = "MV.recoil.backTau.1",
    RECOIL_CLIMB_TAU_0 = "MV.recoil.climbTau.0",
    RECOIL_CLIMB_TAU_1 = "MV.recoil.climbTau.1",
    RECOIL_ROCK_TAU_0 = "MV.recoil.rockTau.0",
    RECOIL_ROCK_TAU_1 = "MV.recoil.rockTau.1",
    GAIT_DROP = "MV.gait.drop",
    GAIT_AIM_DROP = "MV.gait.aimDrop",
    GAIT_BOB = "MV.gait.bob",
    DIP_RATE_IN = "MV2.dip.rateIn",
    DIP_RATE_OUT = "MV2.dip.rateOut",
    BOB_KEEP = "BOB.keep",
    SPRINT_WARP_STRIDE = "SPRINT_WARP.stride",
    TOE_FLOOR_Y = "TOE_FLOOR_Y",
    SOFT_KNEE_FROM = "SOFT_KNEE.from",
    SOFT_KNEE_TO = "SOFT_KNEE.to",
    RUN_LINGER_ON_0 = "RUN_LINGER.on.0",
    RUN_LINGER_ON_1 = "RUN_LINGER.on.1",
    RUN_LINGER_OFF_0 = "RUN_LINGER.off.0",
    RUN_LINGER_OFF_1 = "RUN_LINGER.off.1",
    RUN_LINGER_BLEED = "RUN_LINGER.bleed",
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
}

// A LIST is some nodes of the rig in the rig's own parents-first order, by the animator's name for it (character.ts fkList: joints a layer reads, with every
// ancestor of theirs, so that a run forward of the list alone gives those joints what a whole run would). Written with the roles.
table! { List, LISTS, LIST_NAMES;
    FK_ARMS = "fkArms",
    FK_LEGS = "fkLegs",
}

/// a role nobody has written a node for
pub const NO_ROLE: u32 = u32::MAX;

static mut KNOB: [f64; KNOBS] = [0.0; KNOBS];
static mut ROLE: [u32; ROLES] = [NO_ROLE; ROLES];
/// a list holds no more nodes than a rig the bench holds (skeleton.rs MAX_NODES)
pub const LIST_MAX: usize = 128;
static mut LIST: [[u32; LIST_MAX]; LISTS] = [[0; LIST_MAX]; LISTS];
static mut LIST_LEN: [u32; LISTS] = [0; LISTS];

/// knob `i` of the list (a `pub const` above)
#[inline(always)]
pub fn k(i: usize) -> f64 {
    unsafe { (*core::ptr::addr_of!(KNOB))[i] }
}
/// a switch the TypeScript reads as `X.on ? … : …` (a boolean, written as 1 or 0: true unless nothing or no number)
#[inline(always)]
pub fn truthy(i: usize) -> bool {
    let v = k(i);
    v != 0.0 && v == v
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
/// list `i`: its nodes (none until the host has written it; never more than it holds)
#[inline(always)]
pub fn list(i: usize) -> &'static [u32] {
    unsafe {
        let n = (*core::ptr::addr_of!(LIST_LEN))[i] as usize;
        &(&(*core::ptr::addr_of!(LIST))[i])[..if n > LIST_MAX { 0 } else { n }]
    }
}
/// where a table is and how long: 0 the knobs' values (doubles) · 1 the knobs' names (bytes) · 2 the roles' nodes (u32) · 3 the roles' names (bytes) ·
/// 4 the lists' nodes (u32, LIST_MAX a list) · 5 the lists' names (bytes) · 6 the lists' lengths (u32)
pub fn addr(table: u32) -> usize {
    match table {
        0 => core::ptr::addr_of!(KNOB) as usize,
        1 => KNOB_NAMES.as_ptr() as usize,
        2 => core::ptr::addr_of!(ROLE) as usize,
        3 => ROLE_NAMES.as_ptr() as usize,
        4 => core::ptr::addr_of!(LIST) as usize,
        5 => LIST_NAMES.as_ptr() as usize,
        6 => core::ptr::addr_of!(LIST_LEN) as usize,
        _ => 0,
    }
}
pub fn len(table: u32) -> usize {
    match table {
        0 => KNOBS,
        1 => KNOB_NAMES.len(),
        2 => ROLES,
        3 => ROLE_NAMES.len(),
        4 => LISTS * LIST_MAX,
        5 => LIST_NAMES.len(),
        6 => LISTS,
        _ => 0,
    }
}
