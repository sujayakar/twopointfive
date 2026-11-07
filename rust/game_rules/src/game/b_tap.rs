//! COUNTERS FOR THE LOOP (tools/blitz): how often a run reaches a place where a mend was made by reading and no run had shown it. Not state of the
//! game: no rule reads them, no export carries them, and the runner prints them on its last line (`TAPS`). A count of a process, not of a game.
use core::sync::atomic::{AtomicU32, Ordering};

/// the places, by name (the runner's line reads `name count`)
pub const NAMES: [&str; 18] = [
    "pie_ref_asked",            // guards.ts pieDoorway: a man slicing a doorway is asked whether his path is still the one he began on (`gd.path !== P.pathRef`)
    "pie_ref_new_path",         // … and it is another path
    "pie_ref_new_empty_path",   // … another path that is EMPTY (`gd.path = []`: the whole writes that did not move the serial before the one setter)
    "pie_dropped_by_ref",       // … and the pie was dropped for it
    "doorway_walk_asked",       // doorway.ts: a man waiting at a jamb is asked whether he was given a new walk
    "doorway_walk_new",         // … and he was
    "doorpath_arr_asked",       // doorpath.ts:336: is his path the one this walk laid
    "doorpath_arr_new",         // … and it is not
    "standoff_orbit_asked",     // guards.ts:1163: a man backing off in a standoff is asked whether his path goal IS the back-off point
    "standoff_orbit_relaid",    // … and it is not: the step is laid again
    "standoff_orbit_flag_up_goal_moved",   // … not, though the flag was up: the goal no longer held that point's numbers
    "path_goal_set_with_orbit_up",         // the one setter of a path goal called while the standoff's flag stood (the write that used to leave it up)
    "loud_boot",                // breach.ts: the drill's kick heard as a loud event (each needs a serial of its own)
    "digin_keeper_off_heard",   // digin.ts:705: with DIGIN.keeper off (the difficulty's hard) an idle man who heard a fight is asked whether a press left him to keep his room
    "digin_keeper_lets_lie",    // … and it did: heard, and let lie (he does not start to think of digging in)
    "digin_keeper_while_thinking",   // digin.ts:716: made the keeper while he thought: back to idle, no dig-in
    "sticky_eyes_cone",         // stickycam.ts:195: a live stuck camera is an eye of the fog under the fog's cone
    "sticky_eyes_cone_narrows", // … and the field is narrower than the camera's own wedge (the read changes the eye)
];
static T: [AtomicU32; 18] = [AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0),
    AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0)];

pub const PIE_REF_ASKED: usize = 0;
pub const PIE_REF_NEW_PATH: usize = 1;
pub const PIE_REF_NEW_EMPTY_PATH: usize = 2;
pub const PIE_DROPPED_BY_REF: usize = 3;
pub const DOORWAY_WALK_ASKED: usize = 4;
pub const DOORWAY_WALK_NEW: usize = 5;
pub const DOORPATH_ARR_ASKED: usize = 6;
pub const DOORPATH_ARR_NEW: usize = 7;
pub const STANDOFF_ORBIT_ASKED: usize = 8;
pub const STANDOFF_ORBIT_RELAID: usize = 9;
pub const STANDOFF_ORBIT_FLAG_UP_GOAL_MOVED: usize = 10;
pub const PATH_GOAL_SET_WITH_ORBIT_UP: usize = 11;
pub const LOUD_BOOT: usize = 12;
pub const DIGIN_KEEPER_OFF_HEARD: usize = 13;
pub const DIGIN_KEEPER_LETS_LIE: usize = 14;
pub const DIGIN_KEEPER_WHILE_THINKING: usize = 15;
pub const STICKY_EYES_CONE: usize = 16;
pub const STICKY_EYES_CONE_NARROWS: usize = 17;

#[inline]
pub fn tap(i: usize) { T[i].fetch_add(1, Ordering::Relaxed); }
/// every count, in `NAMES`' order
pub fn read() -> [u32; 18] { let mut o = [0u32; 18]; for i in 0..18 { o[i] = T[i].load(Ordering::Relaxed); } o }
