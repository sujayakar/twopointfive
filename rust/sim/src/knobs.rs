//! THE KNOBS AND THE NAMED NODES RUST CODE READS — one list of each, by the name the TypeScript has for it.
//!
//! A knob is a number of a table the game registers (src/game/knobs.ts knobRegistry: `MV.gait.lean`), tuned in the panel, carried by a recording, put on or
//! off by a stand. It is CONFIG, the same for every game in the process, and no state of a game: so it is not in a game's block (state.rs) and no snapshot
//! carries it. Code here reads a knob by its line below (`k(GAIT_LEAN)`); the host reads the list once (`names`), finds each path in the registry (a path
//! that is not a number there is an error as the module starts work, and a red line in tools/qa/rust-check.ts), and writes every value as a tick opens
//! (src/game/rustknobs.ts). A part of the game that wants a knob in Rust adds a line; nothing else is written for it.
//! A path that starts `ANIM.` is a number of the animator's own tables (character.ts animTables: authored, in no registry, tuned by no panel), read the same way.
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
    LONG_STEADY_RUN_YAW = "ANIM.LONG_STEADY.runYaw",
    HARDBREATH_ON = "HARDBREATH.on",
    HARDBREATH_CHEST_AMP = "HARDBREATH.chestAmp",
    HARDBREATH_CHEST_RATE = "HARDBREATH.chestRate",
    HARDBREATH_CHEST_UP = "HARDBREATH.chestUp",
    HARDBREATH_CHEST_DOWN = "HARDBREATH.chestDown",
    LIFE_ON = "ANIM.LIFE.on",
    LIFE_BREATH_RATE = "ANIM.LIFE.breathRate",
    LIFE_BREATH_RATE_RUN = "ANIM.LIFE.breathRateRun",
    LIFE_BREATH_AMP = "ANIM.LIFE.breathAmp",
    LIFE_BREATH_AMP_RUN = "ANIM.LIFE.breathAmpRun",
    LIFE_MASS_GAIN = "ANIM.LIFE.massGain",
    LIFE_MASS_BACK = "ANIM.LIFE.massBack",
    LIFE_MASS_FWD = "ANIM.LIFE.massFwd",
    LIFE_MASS_DIP = "ANIM.LIFE.massDip",
    LIFE_MASS_DIP_MAX = "ANIM.LIFE.massDipMax",
    LIFE_TIP_GAIN = "ANIM.LIFE.tipGain",
    LIFE_TIP_BACK = "ANIM.LIFE.tipBack",
    LIFE_TIP_FWD = "ANIM.LIFE.tipFwd",
    LIFE_REST_AFTER = "ANIM.LIFE.restAfter",
    LIFE_SHIFT_MIN = "ANIM.LIFE.shiftMin",
    LIFE_SHIFT_SPAN = "ANIM.LIFE.shiftSpan",
    LIFE_SHIFT_TRAVEL = "ANIM.LIFE.shiftTravel",
    LIFE_SHIFT_IN = "ANIM.LIFE.shiftIn",
    LIFE_SHIFT_OUT = "ANIM.LIFE.shiftOut",
    LIFE_HIP_DROP = "ANIM.LIFE.hipDrop",
    LIFE_SPINE_COUNTER = "ANIM.LIFE.spineCounter",
    LIFE_NECK_COUNTER = "ANIM.LIFE.neckCounter",
    FEET_ON = "ANIM.FEET.on",
    FEET_SWING = "ANIM.FEET.swing",
    FEET_LIFT = "ANIM.FEET.lift",
    FEET_DIST = "ANIM.FEET.dist",
    FEET_TURN_DEG = "ANIM.FEET.turnDeg",
    FEET_PIVOT = "ANIM.FEET.pivot",
    FEET_LAG_CAP_DEG = "ANIM.FEET.lagCapDeg",
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
    DIP_RATE_IN = "ANIM.MV2.dip.rateIn",
    DIP_RATE_OUT = "ANIM.MV2.dip.rateOut",
    BOB_KEEP = "ANIM.BOB.keep",
    SPRINT_WARP_STRIDE = "ANIM.SPRINT_WARP.stride",
    TOE_FLOOR_Y = "ANIM.TOE_FLOOR_Y",
    SOFT_KNEE_FROM = "ANIM.SOFT_KNEE.from",
    SOFT_KNEE_TO = "ANIM.SOFT_KNEE.to",
    RUN_LINGER_ON_0 = "ANIM.RUN_LINGER.on.0",
    RUN_LINGER_ON_1 = "ANIM.RUN_LINGER.on.1",
    RUN_LINGER_OFF_0 = "ANIM.RUN_LINGER.off.0",
    RUN_LINGER_OFF_1 = "ANIM.RUN_LINGER.off.1",
    RUN_LINGER_BLEED = "ANIM.RUN_LINGER.bleed",
    SNEAK_STRIDE = "ANIM.SNEAK.stride",
    SNEAK_PACE_STRIDE = "ANIM.SNEAK.paceStride",
    SNEAK_DROP = "ANIM.SNEAK.drop",
    SNEAK_PACE_DROP = "ANIM.SNEAK.paceDrop",
    SNEAK_SOFT_DROP = "ANIM.SNEAK.softDrop",
    SNEAK_LEAN = "ANIM.SNEAK.lean",
    SNEAK_PACE_LEAN = "ANIM.SNEAK.paceLean",
    DASH_STRIDE = "DASH.stride",
    DASH_DROP = "DASH.drop",
    DASH_LEAN = "DASH.lean",
    STEADY_ROLL = "MV.steady.roll",
    STEADY_SWAY = "MV.steady.sway",
    STEADY_FEET = "MV.steady.feet",
    STEADY_KNEE = "MV.steady.knee",
    STEADY_KNEE_LOW = "MV.steady.kneeLow",
    STEADY_KNEE_OUT = "MV.steady.kneeOut",
    STEADY_REACH = "MV.steady.reach",
    CREEP_DROP_0 = "ANIM.CREEP.drop.0",
    CREEP_DROP_1 = "ANIM.CREEP.drop.1",
    CREEP_BACK_0 = "ANIM.CREEP.back.0",
    CREEP_BACK_1 = "ANIM.CREEP.back.1",
    CREEP_LEAN_0 = "ANIM.CREEP.lean.0",
    CREEP_LEAN_1 = "ANIM.CREEP.lean.1",
    CREEP_LOOK = "ANIM.CREEP.look",
    HEFT_DT = "ANIM.CREEP.heft.dt",
    HEFT_STRIDE = "ANIM.CREEP.heft.stride",
    HEFT_LIFT = "ANIM.CREEP.heft.lift",
    HEFT_RUN = "ANIM.CREEP.heft.run",
    HEFT_TRUNK_0 = "ANIM.CREEP.heft.trunk.0",
    HEFT_TRUNK_1 = "ANIM.CREEP.heft.trunk.1",
    HEFT_ARM_0 = "ANIM.CREEP.heft.arm.0",
    HEFT_ARM_1 = "ANIM.CREEP.heft.arm.1",
    HEFT_BODY_0 = "ANIM.CREEP.heft.body.0",
    HEFT_BODY_1 = "ANIM.CREEP.heft.body.1",
    HEFT_GO_0 = "ANIM.CREEP.heft.go.0",
    HEFT_GO_1 = "ANIM.CREEP.heft.go.1",
    HEFT_BREATH = "ANIM.CREEP.heft.breath",
    HEFT_BREATH_HZ = "ANIM.CREEP.heft.breathHz",
    KICK_IMPACT = "ANIM.KICK_IMPACT",
    KICK_PLANT = "ANIM.KICK_PLANT",
    WP_BACK_0 = "ANIM.WALL_POSE.back.0",
    WP_BACK_1 = "ANIM.WALL_POSE.back.1",
    WP_BACK_2 = "ANIM.WALL_POSE.back.2",
    WP_LEAN_0 = "ANIM.WALL_POSE.lean.0",
    WP_LEAN_1 = "ANIM.WALL_POSE.lean.1",
    WP_DROP_0 = "ANIM.WALL_POSE.drop.0",
    WP_DROP_1 = "ANIM.WALL_POSE.drop.1",
    WP_CHEST_TURN = "ANIM.WALL_POSE.chestTurn",
    WP_GAZE = "ANIM.WALL_POSE.gaze",
    WP_HEAD_PITCH = "ANIM.WALL_POSE.headPitch",
    WP_LEAD_0 = "ANIM.WALL_POSE.feet.lead.0",
    WP_LEAD_1 = "ANIM.WALL_POSE.feet.lead.1",
    WP_LEAD_2 = "ANIM.WALL_POSE.feet.lead.2",
    WP_TRAIL_0 = "ANIM.WALL_POSE.feet.trail.0",
    WP_TRAIL_1 = "ANIM.WALL_POSE.feet.trail.1",
    WP_TRAIL_2 = "ANIM.WALL_POSE.feet.trail.2",
    WP_LEAD_C_0 = "ANIM.WALL_POSE.feet.leadCrouch.0",
    WP_LEAD_C_1 = "ANIM.WALL_POSE.feet.leadCrouch.1",
    WP_LEAD_C_2 = "ANIM.WALL_POSE.feet.leadCrouch.2",
    WP_TRAIL_C_0 = "ANIM.WALL_POSE.feet.trailCrouch.0",
    WP_TRAIL_C_1 = "ANIM.WALL_POSE.feet.trailCrouch.1",
    WP_TRAIL_C_2 = "ANIM.WALL_POSE.feet.trailCrouch.2",
    WP_TRAIL_HEEL = "ANIM.WALL_POSE.feet.trailHeel",
    WP_SLIDE_LINE = "ANIM.WALL_POSE.slide.line",
    WP_SLIDE_NARROW = "ANIM.WALL_POSE.slide.narrow",
    WP_PEEK_LEAN = "ANIM.WALL_POSE.peek.lean",
    WP_PEEK_SHIFT = "ANIM.WALL_POSE.peek.shift",
    WP_PEEK_TURN = "ANIM.WALL_POSE.peek.turn",
    WP_PEEK_GAZE = "ANIM.WALL_POSE.peek.gaze",
    WP_PEEK_TILT = "ANIM.WALL_POSE.peek.tilt",
    WG_SHARE_0 = "ANIM.WALL_POSE.gait.share.0",
    WG_SHARE_1 = "ANIM.WALL_POSE.gait.share.1",
    WG_STEP_0 = "ANIM.WALL_POSE.gait.step.0",
    WG_STEP_1 = "ANIM.WALL_POSE.gait.step.1",
    WG_DUTY = "ANIM.WALL_POSE.gait.duty",
    WG_AHEAD_0 = "ANIM.WALL_POSE.gait.ahead.0",
    WG_AHEAD_1 = "ANIM.WALL_POSE.gait.ahead.1",
    WG_APART_0 = "ANIM.WALL_POSE.gait.apart.0",
    WG_APART_1 = "ANIM.WALL_POSE.gait.apart.1",
    WG_NEAR_0 = "ANIM.WALL_POSE.gait.near.0",
    WG_NEAR_1 = "ANIM.WALL_POSE.gait.near.1",
    WG_FAR_0 = "ANIM.WALL_POSE.gait.far.0",
    WG_FAR_1 = "ANIM.WALL_POSE.gait.far.1",
    WG_LIFT_0 = "ANIM.WALL_POSE.gait.lift.0",
    WG_LIFT_1 = "ANIM.WALL_POSE.gait.lift.1",
    WG_HEEL_0 = "ANIM.WALL_POSE.gait.heel.0",
    WG_HEEL_1 = "ANIM.WALL_POSE.gait.heel.1",
    WG_TOE_0 = "ANIM.WALL_POSE.gait.toe.0",
    WG_TOE_1 = "ANIM.WALL_POSE.gait.toe.1",
    WG_KNEE_ROOM_0 = "ANIM.WALL_POSE.gait.kneeRoom.0",
    WG_KNEE_ROOM_1 = "ANIM.WALL_POSE.gait.kneeRoom.1",
    WG_HIPS_0 = "ANIM.WALL_POSE.gait.hips.0",
    WG_HIPS_1 = "ANIM.WALL_POSE.gait.hips.1",
    WG_BOB_0 = "ANIM.WALL_POSE.gait.bob.0",
    WG_BOB_1 = "ANIM.WALL_POSE.gait.bob.1",
    WS_ON = "WALLSNAP.on",
    WS_POSE = "WALLSNAP.pose",
    WS_DIP = "WALLSNAP.dip",
    WS_NOD = "WALLSNAP.nod",
    WS_STEP_LIFT = "WALLSNAP.stepLift",
    WS_STEP_TRAIL = "WALLSNAP.stepTrail",
    WALL_ROOT_OFF = "ANIM.WALL_ROOT_OFF",
    WALL_GAIT_REST = "ANIM.WALL_GAIT_REST",
    WATCH_YAW = "WATCH.yaw",
    WATCH_DOWN = "WATCH.down",
    WATCH_UP = "WATCH.up",
    WATCH_SHARE_0 = "WATCH.share.0",
    WATCH_SHARE_1 = "WATCH.share.1",
    WATCH_SHARE_2 = "WATCH.share.2",
    WATCH_RATE = "WATCH.rate",
    WATCH_W_RATE = "WATCH.wRate",
    WATCH_LEAN = "WATCH.lean",
    WATCH_DROP = "WATCH.drop",
    WATCH_AWAY = "WATCH.away",
    WATCH_SLIP_RATE = "WATCH.slipRate",
    WATCH_SQUEEZE = "WATCH.squeeze",
    MOVES_DAZZLE = "MOVES.dazzle",
    FLINCH_AMP_0 = "MV.flinch.amp.0",
    FLINCH_AMP_1 = "MV.flinch.amp.1",
    FLINCH_DUR_0 = "MV.flinch.dur.0",
    FLINCH_DUR_1 = "MV.flinch.dur.1",
    FLINCH_PITCH = "MV.flinch.pitch",
    FLINCH_HEAD = "MV.flinch.head",
    FLINCH_AWAY = "MV.flinch.away",
    FLINCH_DROP = "MV.flinch.drop",
    FLINCH_SIDE = "MV.flinch.side",
    FLINCH_ENV_0_0 = "ANIM.MV_FLINCH_ENV.0.0",
    FLINCH_ENV_0_1 = "ANIM.MV_FLINCH_ENV.0.1",
    FLINCH_ENV_1_0 = "ANIM.MV_FLINCH_ENV.1.0",
    FLINCH_ENV_1_1 = "ANIM.MV_FLINCH_ENV.1.1",
    FLINCH_ENV_2_0 = "ANIM.MV_FLINCH_ENV.2.0",
    FLINCH_ENV_2_1 = "ANIM.MV_FLINCH_ENV.2.1",
    FLINCH_ENV_3_0 = "ANIM.MV_FLINCH_ENV.3.0",
    FLINCH_ENV_3_1 = "ANIM.MV_FLINCH_ENV.3.1",
    FLINCH_ENV_4_0 = "ANIM.MV_FLINCH_ENV.4.0",
    FLINCH_ENV_4_1 = "ANIM.MV_FLINCH_ENV.4.1",
    HIT_DUR = "MV.hit.dur",
    HIT_PITCH = "MV.hit.pitch",
    HIT_YAW = "MV.hit.yaw",
    HIT_ROLL = "MV.hit.roll",
    HIT_SHOVE = "MV.hit.shove",
    HIT_DROP = "MV.hit.drop",
    HIT_ENV_0_0 = "ANIM.MV_HIT_ENV.0.0",
    HIT_ENV_0_1 = "ANIM.MV_HIT_ENV.0.1",
    HIT_ENV_1_0 = "ANIM.MV_HIT_ENV.1.0",
    HIT_ENV_1_1 = "ANIM.MV_HIT_ENV.1.1",
    HIT_ENV_2_0 = "ANIM.MV_HIT_ENV.2.0",
    HIT_ENV_2_1 = "ANIM.MV_HIT_ENV.2.1",
    HIT_ENV_3_0 = "ANIM.MV_HIT_ENV.3.0",
    HIT_ENV_3_1 = "ANIM.MV_HIT_ENV.3.1",
    HIT_ENV_4_0 = "ANIM.MV_HIT_ENV.4.0",
    HIT_ENV_4_1 = "ANIM.MV_HIT_ENV.4.1",
    HIT_ENV_5_0 = "ANIM.MV_HIT_ENV.5.0",
    HIT_ENV_5_1 = "ANIM.MV_HIT_ENV.5.1",
    START_AMP_0 = "ANIM.MV_STARTLE.amp.0",
    START_AMP_1 = "ANIM.MV_STARTLE.amp.1",
    START_DUR_0 = "ANIM.MV_STARTLE.dur.0",
    START_DUR_1 = "ANIM.MV_STARTLE.dur.1",
    START_REAR = "ANIM.MV_STARTLE.rear",
    START_TURN = "ANIM.MV_STARTLE.turn",
    START_LEAN = "ANIM.MV_STARTLE.lean",
    START_HEAD = "ANIM.MV_STARTLE.head",
    START_HEAD_TURN = "ANIM.MV_STARTLE.headTurn",
    START_BACK = "ANIM.MV_STARTLE.back",
    START_DROP = "ANIM.MV_STARTLE.drop",
    START_GUN_AMP = "ANIM.MV_STARTLE.gunAmp",
    START_GUN_REAR = "ANIM.MV_STARTLE.gunRear",
    START_GUN_BACK = "ANIM.MV_STARTLE.gunBack",
    START_BEHIND_TURN = "ANIM.MV_STARTLE.behindTurn",
    START_BEHIND_HEAD = "ANIM.MV_STARTLE.behindHead",
    START_BEHIND_PITCH = "ANIM.MV_STARTLE.behindPitch",
    START_ENV_0_0 = "ANIM.MV_STARTLE.env.0.0",
    START_ENV_0_1 = "ANIM.MV_STARTLE.env.0.1",
    START_ENV_1_0 = "ANIM.MV_STARTLE.env.1.0",
    START_ENV_1_1 = "ANIM.MV_STARTLE.env.1.1",
    START_ENV_2_0 = "ANIM.MV_STARTLE.env.2.0",
    START_ENV_2_1 = "ANIM.MV_STARTLE.env.2.1",
    START_ENV_3_0 = "ANIM.MV_STARTLE.env.3.0",
    START_ENV_3_1 = "ANIM.MV_STARTLE.env.3.1",
    START_ENV_4_0 = "ANIM.MV_STARTLE.env.4.0",
    START_ENV_4_1 = "ANIM.MV_STARTLE.env.4.1",
    DAZZLE_AMP_0 = "MV.dazzle.amp.0",
    DAZZLE_AMP_1 = "MV.dazzle.amp.1",
    DAZZLE_HUNCH = "MV.dazzle.hunch",
    DAZZLE_AWAY = "MV.dazzle.away",
    DAZZLE_ROLL = "MV.dazzle.roll",
    DAZZLE_HEAD = "MV.dazzle.head",
    DAZZLE_HEAD_AWAY = "MV.dazzle.headAway",
    DAZZLE_DROP = "MV.dazzle.drop",
    DAZZLE_BACK = "MV.dazzle.back",
    DAZZLE_SWAY_X = "MV.dazzle.swayX",
    DAZZLE_SWAY_Z = "MV.dazzle.swayZ",
    DAZZLE_RATE_IN = "MV.dazzle.rateIn",
    DAZZLE_RATE_OUT = "MV.dazzle.rateOut",
    FOLD_RATE = "ANIM.MV2.clutch.fold.rate",
    FOLD_PITCH = "ANIM.MV2.clutch.fold.pitch",
    FOLD_PITCH_TOO = "ANIM.MV2.clutch.fold.pitchToo",
    FOLD_HEAD = "ANIM.MV2.clutch.fold.head",
    FOLD_DROP = "ANIM.MV2.clutch.fold.drop",
    FOLD_SWAY = "ANIM.MV2.clutch.fold.sway",
    FOLD_HZ = "ANIM.MV2.clutch.fold.hz",
    HURT_RATE = "ANIM.MV2.hurt.rate",
    HURT_PITCH = "ANIM.MV2.hurt.pitch",
    HURT_LEAN = "ANIM.MV2.hurt.lean",
    HURT_HEAD = "ANIM.MV2.hurt.head",
    HURT_HEAD_BRACE = "ANIM.MV2.hurt.headBrace",
    HURT_DROP = "ANIM.MV2.hurt.drop",
    HURT_SWAY = "ANIM.MV2.hurt.sway",
    HURT_HZ = "ANIM.MV2.hurt.hz",
    HURT_BRACE = "ANIM.MV2.hurt.brace",
    HURT_STEP_PITCH = "ANIM.MV2.hurt.stepPitch",
    HURT_STEP_ROLL = "ANIM.MV2.hurt.stepRoll",
    HURT_STEP_YAW = "ANIM.MV2.hurt.stepYaw",
    HURT_STEP_HEAD = "ANIM.MV2.hurt.stepHead",
    HURT_STEP_DROP = "ANIM.MV2.hurt.stepDrop",
    HURT_STEP_SIDE = "ANIM.MV2.hurt.stepSide",
    SHAKEN_RATE_IN = "ANIM.MV2.shaken.rateIn",
    SHAKEN_RATE_OUT = "ANIM.MV2.shaken.rateOut",
    SHAKEN_PITCH = "ANIM.MV2.shaken.pitch",
    SHAKEN_HEAD = "ANIM.MV2.shaken.head",
    SHAKEN_DROP = "ANIM.MV2.shaken.drop",
    SHAKEN_BRACE = "ANIM.MV2.shaken.brace",
    TOSS_AFTER = "ANIM.TOSS_AFTER",
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
