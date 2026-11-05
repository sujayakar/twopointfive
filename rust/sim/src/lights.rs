//! THE LEVEL'S LIGHTS (src/game/lighting.ts, the machines of src/game/devices.ts) — whether a fixture is on, broken, blinded, what the mains are doing, the one
//! number a fixture burns at this tick (its `factor`), the lamp each fixture feeds as a light question reads it, and a camera's and a tripwire's own machine.
//!
//! WHAT IS STATE AND WHAT IS NOT. Everything here is the `lights` part of the game's block (state.rs). What men do about the dark is not here (the guards' edge
//! and the errand to the breaker are src/game/lighting.ts's and powerout.ts's), nor how a fixture looks when it breaks (fixtureBreak.ts: dressing, which reads
//! this), nor where a lens looks at a man from (men's bones and Sam's meter are handed into `camera_see`, as numbers: THE MACHINE NEVER ASKS THE LIGHT).
//!
//! ONE WAY IN. `input` takes every event from outside (a round broke it, a hand flipped it, the mains were thrown, a file was loaded) as a kind and a target;
//! `flip` is a wall switch's whole circuit, whose rule reads all its fixtures at once. `step` is the tick's own. tools/qa/lights-lint.ts is red on a line of
//! src that writes any of it another way, and the fields of game.ts LightTarget are read-only to the compiler.
//!
//! WAYS OUT. What a step or a machine did that the game has to answer for is said in the bits it returns (`EV_*`, `CAM_*`, `LASER_*`): the host turns them into
//! the words, sounds, men sent and marks on the debrief that have always gone with them, at the place in the tick they always were.
//!
//! THE LAMPS ARE ROWS IN THE ORDER OF THE ENGINE'S LIST. A row is one lamp a fixture feeds (a fixture's own light, an area light of a flat fixture or a marking,
//! a beacon's spot, a camera's cone), in the order of `engine.lights`, stood once as the level is built (lightshome.ts `stand`). The order is what a sum of
//! floats over them is made of, so nothing sorts them. Lamps nobody's fixture feeds stay the question's own (a man's torch, a flash), and each is written with
//! how many rows stand before it in the list as it stands then.
//!
//! STATEMENT FOR STATEMENT with the TypeScript it replaces: same order, the game's own maths (maths.rs), the die untouched.
use crate::maths::{cos, hypot3, sin};
use crate::state::state;
use crate::world::{js_max, js_min};
use core::mem::{offset_of, size_of};
use prim::floor;

/// fixtures a level holds (src/game/levelfit.ts refuses a level over any of these at load, by name). The fullest level today has 73.
pub const TARGETS: usize = 256;
/// lamps fixtures feed. The fullest level today has 52. The same as light.rs MAX_LAMPS: the sweep and a question hold no more.
pub const ROWS: usize = 256;
/// emergency beacons, cameras, tripwires (fullest level today: 7, 3, 4)
pub const BEACONS: usize = 32;
pub const CAMERAS: usize = 32;
pub const LASERS: usize = 32;

// the mains (`mains`)
pub const M_ACTIVE: u32 = 1;
pub const M_PERMANENT: u32 = 2;
// a fixture's truths (`bits`)
pub const T_ON: u32 = 1;
pub const T_BROKEN: u32 = 2;
pub const T_FLICKER: u32 = 4;
pub const T_MAINS: u32 = 8;
pub const T_HAND: u32 = 16;
pub const T_AUTHORED: u32 = 32;
pub const T_EMISSIVE: u32 = 64;
// a row's truths (`row_bits`): burning (the reader's gate is this, the first self's intensity > 0 and kind != 2: light.rs, as guards.ts lampAsked) · who asks what the PLACE lights leaves it out · a screen's own lamp · what feeds it (two bits) · it has a steady self (`sx` … `s_intensity`, `feed` says; else they are zeros and the first self is both)
pub const R_ON: u32 = 1;
pub const R_PASSING: u32 = 2;
pub const R_SCREEN: u32 = 4;
pub const R_ROLE: u32 = 24;
pub const R_OWN: u32 = 0;
pub const R_AREA: u32 = 8;
pub const R_BEACON: u32 = 16;
pub const R_CONE: u32 = 24;
pub const R_STEADY: u32 = 32;
// a camera's truths (`cam_bits`)
pub const C_SWEEP: u32 = 1;
pub const C_LAST: u32 = 2;
pub const C_SAW: u32 = 4;
pub const C_DEAD: u32 = 8;
// a camera's state, by its place in devices.ts CAMERA_STATES
pub const S_IDLE: u32 = 0;
pub const S_SUSPECT: u32 = 1;
pub const S_ALARM: u32 = 2;
// a tripwire's truths (`las_bits`)
pub const L_DEAD: u32 = 1;
pub const L_HELD: u32 = 2;

// what `step` says happened
pub const EV_RAN_OUT: u32 = 1;
pub const EV_HAND: u32 = 2;
pub const EV_FIXED: u32 = 4;
// what `camera_pan` and `camera_see` say
pub const CAM_OK: u32 = 1;
pub const CAM_DOWN: u32 = 2;
pub const CAM_SUSPECT: u32 = 4;
pub const CAM_ALARM: u32 = 8;
// what `laser_open` and `laser_close` say
pub const LASER_OK: u32 = 1;
pub const LASER_DOWN: u32 = 2;
pub const LASER_TRIP: u32 = 4;

// the kinds of `input`: on (a: 0 or 1) · broken · mended · toggled (the panel's: flipped and mended) · blinded until a · what a hand left (a: when, NaN: nothing) ·
// the level as authored (a: 0 or 1) · its base strength a · all four of a file (a on, b broken, c until, d hand) · the mains tripped at a for b seconds · the mains
// back at a · the mains as a file or a show has them (a: M_ bits, b until, c since, d restored at) · the timer at the panel set to a · the mains as a new level has them ·
// the restrike's delay a (a recording's own)
pub const IN_ON: u32 = 0;
pub const IN_BREAK: u32 = 1;
pub const IN_MEND: u32 = 2;
pub const IN_TOGGLE: u32 = 3;
pub const IN_BLIND: u32 = 4;
pub const IN_HAND: u32 = 5;
pub const IN_AUTHORED: u32 = 6;
pub const IN_BASE: u32 = 7;
pub const IN_PUT: u32 = 8;
pub const IN_TRIP: u32 = 9;
pub const IN_BACK: u32 = 10;
pub const IN_MAINS: u32 = 11;
pub const IN_FIX: u32 = 12;
pub const IN_FRESH: u32 = 13;
pub const IN_STAGGER: u32 = 14;

/// A camera's picture drains this fast once he is out of it (lit-seconds a second: a glimpse and gone is forgotten in a couple of seconds; ducking behind the desk for
/// a beat does not reset a camera that has been watching you walk toward it), and it goes quiet this long after a report, re-arming once drained AND this has passed
const CAMERA_DECAY: f64 = 0.5;
const CAMERA_REARM_SECS: f64 = 20.0;
/// a tripwire, once broken, cannot be broken again for this long (one crossing is one report, not sixty)
const LASER_COOLDOWN: f64 = 4.0;
/// the cone starts this far out of the lens (the beam starts at the lens face; devices.ts CONE's other numbers are the picture's)
const CONE_AHEAD: f64 = 0.1;

/// The lights' part of a game's block (src/sim/lightshome.ts reads it by the module's own column offsets, `cap`).
#[repr(C)]
pub struct Lights {
    /// fixtures, rows, beacons, cameras, tripwires
    pub n: u32,
    pub rows: u32,
    pub beacons: u32,
    pub cameras: u32,
    pub lasers: u32,
    /// M_ bits
    pub mains: u32,
    pub spare: [u32; 2],
    pub since: f64,
    pub until: f64,
    pub restored_at: f64,
    /// how long a man of theirs has been at the panel
    pub fix_t: f64,
    // ---- the fixtures, in the order of Game.targets
    pub blind_until: [f64; TARGETS],
    pub factor: [f64; TARGETS],
    pub stagger: [f64; TARGETS],
    pub base: [f64; TARGETS],
    pub hand: [f64; TARGETS],
    pub bits: [u32; TARGETS],
    // ---- the lamps, in the order of engine.lights: what a question takes (FACE.LAMP's own)
    pub x: [f64; ROWS],
    pub y: [f64; ROWS],
    pub z: [f64; ROWS],
    pub range: [f64; ROWS],
    pub radius: [f64; ROWS],
    pub kind: [f64; ROWS],
    pub dx: [f64; ROWS],
    pub dy: [f64; ROWS],
    pub dz: [f64; ROWS],
    pub outer: [f64; ROWS],
    pub inner: [f64; ROWS],
    pub r: [f64; ROWS],
    pub g: [f64; ROWS],
    pub b: [f64; ROWS],
    pub intensity: [f64; ROWS],
    // ---- … its steady self, where R_STEADY is up: what who asks what the PLACE lights takes (a fire's)
    pub sx: [f64; ROWS],
    pub sy: [f64; ROWS],
    pub sz: [f64; ROWS],
    pub s_radius: [f64; ROWS],
    pub s_intensity: [f64; ROWS],
    // ---- … and what feeds it: an area light's own strength (a beacon's: its peak)
    pub row_base: [f64; ROWS],
    pub row_bits: [u32; ROWS],
    /// the fixture it follows (an index of the columns above; a cone's: its camera's)
    pub row_target: [u32; ROWS],
    // ---- the beacons
    pub beacon_phase: [f64; BEACONS],
    pub beacon_row: [u32; BEACONS],
    pub beacon_target: [u32; BEACONS],
    // ---- the cameras: the machine
    pub cam_yaw: [f64; CAMERAS],
    pub cam_phase: [f64; CAMERAS],
    pub cam_seen: [f64; CAMERAS],
    pub cam_reported: [f64; CAMERAS],
    pub cam_lx: [f64; CAMERAS],
    pub cam_ly: [f64; CAMERAS],
    pub cam_lz: [f64; CAMERAS],
    // ---- … the level's facts of it
    pub cam_rest: [f64; CAMERAS],
    pub cam_a: [f64; CAMERAS],
    pub cam_b: [f64; CAMERAS],
    pub cam_period: [f64; CAMERAS],
    pub cam_pitch: [f64; CAMERAS],
    pub cam_px: [f64; CAMERAS],
    pub cam_py: [f64; CAMERAS],
    pub cam_pz: [f64; CAMERAS],
    pub cam_state: [u32; CAMERAS],
    pub cam_bits: [u32; CAMERAS],
    pub cam_target: [u32; CAMERAS],
    pub cam_row: [u32; CAMERAS],
    // ---- the tripwires
    pub las_last: [f64; LASERS],
    pub las_cut: [f64; LASERS],
    pub las_sense: [f64; LASERS],
    pub las_trips: [u32; LASERS],
    pub las_bits: [u32; LASERS],
    pub las_target: [u32; LASERS],
    pub las_spare: [u32; LASERS],
}
const _: () = assert!(
    size_of::<Lights>()
        == 24 + 8 + 32
            + 5 * TARGETS * 8 + TARGETS * 4
            + 21 * ROWS * 8 + 2 * ROWS * 4
            + BEACONS * 8 + 2 * BEACONS * 4
            + 15 * CAMERAS * 8 + 4 * CAMERAS * 4
            + 3 * LASERS * 8 + 4 * LASERS * 4
);
/// the columns in their order (src/sim/lightshome.ts LIGHT_COLS names them the same, and reads where each is from here)
pub const COLS: [usize; 67] = [
    offset_of!(Lights, n), offset_of!(Lights, rows), offset_of!(Lights, beacons), offset_of!(Lights, cameras), offset_of!(Lights, lasers),
    offset_of!(Lights, mains), offset_of!(Lights, since), offset_of!(Lights, until), offset_of!(Lights, restored_at), offset_of!(Lights, fix_t),
    offset_of!(Lights, blind_until), offset_of!(Lights, factor), offset_of!(Lights, stagger), offset_of!(Lights, base), offset_of!(Lights, hand),
    offset_of!(Lights, bits), offset_of!(Lights, x), offset_of!(Lights, y), offset_of!(Lights, z), offset_of!(Lights, range),
    offset_of!(Lights, radius), offset_of!(Lights, kind), offset_of!(Lights, dx), offset_of!(Lights, dy), offset_of!(Lights, dz),
    offset_of!(Lights, outer), offset_of!(Lights, inner), offset_of!(Lights, r), offset_of!(Lights, g), offset_of!(Lights, b),
    offset_of!(Lights, intensity), offset_of!(Lights, sx), offset_of!(Lights, sy), offset_of!(Lights, sz), offset_of!(Lights, s_radius),
    offset_of!(Lights, s_intensity), offset_of!(Lights, row_base), offset_of!(Lights, row_bits), offset_of!(Lights, row_target),
    offset_of!(Lights, beacon_phase), offset_of!(Lights, beacon_row), offset_of!(Lights, beacon_target), offset_of!(Lights, cam_yaw),
    offset_of!(Lights, cam_phase), offset_of!(Lights, cam_seen), offset_of!(Lights, cam_reported), offset_of!(Lights, cam_lx),
    offset_of!(Lights, cam_ly), offset_of!(Lights, cam_lz), offset_of!(Lights, cam_rest), offset_of!(Lights, cam_a), offset_of!(Lights, cam_b),
    offset_of!(Lights, cam_period), offset_of!(Lights, cam_pitch), offset_of!(Lights, cam_px), offset_of!(Lights, cam_py), offset_of!(Lights, cam_pz),
    offset_of!(Lights, cam_state), offset_of!(Lights, cam_bits), offset_of!(Lights, cam_target), offset_of!(Lights, cam_row),
    offset_of!(Lights, las_last), offset_of!(Lights, las_cut), offset_of!(Lights, las_sense), offset_of!(Lights, las_trips),
    offset_of!(Lights, las_bits), offset_of!(Lights, las_target),
];

/// how many fixtures (0), rows (1), beacons (2), cameras (3), tripwires (4) a part holds, how long it is (13), how many columns it has (14), and where column k starts (100 + k)
pub fn cap(table: u32) -> usize {
    match table {
        0 => TARGETS,
        1 => ROWS,
        2 => BEACONS,
        3 => CAMERAS,
        4 => LASERS,
        13 => size_of::<Lights>(),
        14 => COLS.len(),
        k if k >= 100 && ((k - 100) as usize) < COLS.len() => COLS[(k - 100) as usize],
        _ => 0,
    }
}

/// the first ids handed to `flip`, written by the host before it asks
static mut IDS: [u32; TARGETS] = [0; TARGETS];
/// where those are (a byte address)
pub fn ids_addr() -> usize {
    core::ptr::addr_of!(IDS) as usize
}

#[inline(always)]
fn active(l: &Lights) -> bool {
    l.mains & M_ACTIVE != 0
}
#[inline(always)]
fn permanent(l: &Lights) -> bool {
    l.mains & M_PERMANENT != 0
}

/// lighting.ts supply: 0 while a fixture has no supply, then the fluorescent restrike's flicker for 1.2 s after `dark`, then 1
#[inline]
fn supply(time: f64, dark: f64) -> f64 {
    if time < dark {
        return 0.0;
    }
    if time < dark + 1.2 {
        let x = (time - dark) / 1.2;
        let s = sin(time * 45.0) * sin(time * 13.7 + 1.3);
        return if s > 0.65 - x * 0.9 { 1.0 } else { 0.05 };
    }
    1.0
}

/// the mains back (lighting.ts endBlackout's own numbers)
#[inline]
fn back(l: &mut Lights, time: f64) {
    l.mains = 0;
    l.until = -1.0;
    l.restored_at = time;
    l.fix_t = 0.0;
}

/// write a row's burning: whether it is on, and how strong
#[inline]
fn burn(l: &mut Lights, r: usize, on: bool, intensity: f64) {
    l.intensity[r] = intensity;
    l.row_bits[r] = (l.row_bits[r] & !R_ON) | if on { R_ON } else { 0 };
}

/// EVERY EVENT FROM OUTSIDE. `t` is the fixture (an index of the columns), where the kind has one. The answer is the kind's own: IN_TRIP 0 = refused (the
/// mains are already gone for good), 1 = taken, 3 = taken and the mains were up; IN_BACK 0 = they were up, 1 = back; every other 0.
pub fn input(g: usize, kind: u32, t: usize, a: f64, b: f64, c: f64, d: f64) -> u32 {
    let l = &mut state(g).lights;
    match kind {
        IN_ON => {
            if a != 0.0 {
                l.bits[t] |= T_ON;
            } else {
                l.bits[t] &= !T_ON;
            }
        }
        IN_BREAK => l.bits[t] |= T_BROKEN,
        IN_MEND => l.bits[t] &= !T_BROKEN,
        IN_TOGGLE => {
            l.bits[t] ^= T_ON;
            l.bits[t] &= !T_BROKEN;
        }
        IN_BLIND => l.blind_until[t] = a,
        IN_HAND => set_hand(l, t, a),
        IN_AUTHORED => {
            if a != 0.0 {
                l.bits[t] |= T_AUTHORED;
            } else {
                l.bits[t] &= !T_AUTHORED;
            }
        }
        IN_BASE => l.base[t] = a,
        IN_STAGGER => l.stagger[t] = a,
        IN_PUT => {
            let was = l.bits[t] & !(T_ON | T_BROKEN);
            l.bits[t] = was | if a != 0.0 { T_ON } else { 0 } | if b != 0.0 { T_BROKEN } else { 0 };
            l.blind_until[t] = c;
            set_hand(l, t, d);
        }
        IN_TRIP => {
            let (time, seconds) = (a, b);
            if active(l) && permanent(l) {
                return 0;
            }
            let fresh = !active(l);
            if fresh {
                l.since = time;
            }
            let gone = !seconds.is_finite();
            l.until = if gone { f64::INFINITY } else { js_max(l.until, time + seconds) };
            l.mains = M_ACTIVE | if gone { M_PERMANENT } else { 0 };
            return if fresh { 3 } else { 1 };
        }
        IN_BACK => {
            if !active(l) {
                return 0;
            }
            back(l, a);
            return 1;
        }
        IN_MAINS => {
            l.mains = a as u32;
            l.until = b;
            l.since = c;
            l.restored_at = d;
        }
        IN_FIX => l.fix_t = a,
        IN_FRESH => {
            l.mains = 0;
            l.until = -1.0;
            l.since = -1.0;
            l.restored_at = -100.0;
            l.fix_t = 0.0;
        }
        _ => {}
    }
    0
}
/// what a hand left: NaN says nothing, and leaves zeros (a record that is over)
#[inline]
fn set_hand(l: &mut Lights, t: usize, a: f64) {
    if a == a {
        l.bits[t] |= T_HAND;
        l.hand[t] = a;
    } else {
        l.bits[t] &= !T_HAND;
        l.hand[t] = 0.0;
    }
}

/// A WALL SWITCH: the first `n` fixtures of `IDS`, the room's circuit (lightswitch.ts flipSwitch). On: what a hand put out; in a room no hand put out, the room as
/// authored (all of it, where the level left the whole room dark), a flat fixture striking as it comes back unless `strike` is 0. Off: every one that is on, and
/// the time on each. (Who saw it go is the caller's, which asks before it calls: the factor does not move here.)
pub fn flip(g: usize, on: bool, time: f64, strike: bool, n: usize) {
    let l = &mut state(g).lights;
    let ids = unsafe { &*core::ptr::addr_of!(IDS) };
    if on {
        let (mut by_hand, mut authored) = (false, false);
        for k in 0..n {
            let b = l.bits[ids[k] as usize];
            by_hand = by_hand || (b & T_HAND != 0 && b & T_ON == 0);
            authored = authored || b & T_AUTHORED != 0;
        }
        for k in 0..n {
            let t = ids[k] as usize;
            let b = l.bits[t];
            let mine = if by_hand { b & T_HAND != 0 } else { !authored || b & T_AUTHORED != 0 };
            if mine && b & T_ON == 0 {
                if strike && b & T_EMISSIVE != 0 && b & T_BROKEN == 0 && l.blind_until[t] < 0.0 {
                    l.blind_until[t] = time;
                }
                l.bits[t] |= T_ON;
            }
            set_hand(l, t, f64::NAN);
        }
    } else {
        for k in 0..n {
            let t = ids[k] as usize;
            if l.bits[t] & T_ON == 0 {
                continue;
            }
            l.bits[t] &= !T_ON;
            set_hand(l, t, time);
        }
    }
}

/// ONE TICK OF THE LIGHTS (lighting.ts updateTargets): the mains' own clock and the timer at the panel, the beacons, each fixture's factor and each lamp as it
/// burns. `near`: one of theirs is at the panel (who is the caller's). `es`: the renderer setting an area light's strength is scaled by, an input.
pub fn step(g: usize, time: f64, dt: f64, near: bool, fumble: f64, es: f64) -> u32 {
    let l = &mut state(g).lights;
    let mut ev = 0;
    if active(l) && !permanent(l) && time >= l.until {
        back(l, time);
        ev |= EV_RAN_OUT;
    }
    if active(l) && !permanent(l) && time - l.since > 3.0 {
        l.fix_t = if near { l.fix_t + dt } else { 0.0 };
        if near {
            ev |= EV_HAND;
            if l.fix_t > fumble {
                back(l, time);
                ev |= EV_FIXED;
            }
        }
    }
    // emergency beacons: spin up ~0.8 s after the trip, keep turning for a second after power returns; each is slaved to its dome fixture (whose factor is last frame's)
    let on = if active(l) { time - l.since > 0.8 } else { time - l.restored_at < 1.2 };
    for b in 0..l.beacons as usize {
        let t = l.beacon_target[b] as usize;
        if on {
            l.bits[t] |= T_ON;
        } else {
            l.bits[t] &= !T_ON;
        }
        let r = l.beacon_row[b] as usize;
        let a = time * 3.3 + l.beacon_phase[b];
        let (c, s) = (cos(a), sin(a));
        let h = hypot3(c, -0.5, s);
        let h = if h != 0.0 { h } else { 1.0 };
        l.dx[r] = c / h;
        l.dy[r] = -0.5 / h;
        l.dz[r] = s / h;
        let peak = l.row_base[r];
        let lit = on && l.factor[t] > 0.0;
        burn(l, r, lit, peak);
    }
    for t in 0..l.n as usize {
        let b = l.bits[t];
        let mut f = if b & T_ON != 0 && b & T_BROKEN == 0 { 1.0 } else { 0.0 };
        if f > 0.0 && l.blind_until[t] > 0.0 {
            f *= supply(time, l.blind_until[t]);
            if time > l.blind_until[t] + 1.2 {
                l.blind_until[t] = -1.0;
            }
        }
        if f > 0.0 && b & T_MAINS != 0 {
            f *= if active(l) { 0.0 } else { supply(time, l.restored_at + l.stagger[t]) };
        }
        if f > 0.0 && b & T_FLICKER != 0 {
            let s = sin(time * 3.1) + sin(time * 7.3 + 2.0) * 0.6;
            if s > 1.15 {
                f *= if sin(time * 90.0) > 0.0 { 0.15 } else { 1.0 };
            }
        }
        l.factor[t] = f;
    }
    for r in 0..l.rows as usize {
        let role = l.row_bits[r] & R_ROLE;
        if role == R_OWN {
            let t = l.row_target[r] as usize;
            let f = l.factor[t];
            let i = l.base[t] * f;
            burn(l, r, f > 0.001, i);
        } else if role == R_AREA {
            let f = l.factor[l.row_target[r] as usize];
            let i = l.row_base[r] * f * es;
            burn(l, r, f > 0.001, i);
        }
    }
    ev
}

/// a device works this frame: powered, unbroken, not knocked out (devices.ts alive)
#[inline(always)]
fn alive(l: &Lights, t: usize) -> bool {
    l.factor[t] > 0.05 && l.bits[t] & T_BROKEN == 0
}

/// THE CAMERA'S OWN HALF BEFORE ITS PICTURE (devices.ts updateCamera, down to the picture): the pan, the cone it throws, the feed dropping once, and the picture
/// emptying when it cannot see. `live`: devices sense. `cones`: its cone is shown.
pub fn camera_pan(g: usize, i: usize, dt: f64, live: bool, cones: bool) -> u32 {
    let l = &mut state(g).lights;
    let t = l.cam_target[i] as usize;
    let ok = alive(l, t);
    let sweep = l.cam_bits[i] & C_SWEEP != 0;
    if ok && sweep {
        let u = l.cam_phase[i] + dt / js_max(0.5, l.cam_period[i]);
        l.cam_phase[i] = u - floor(u); // (u ≥ 0: the fraction is exact, as `% 1`)
        let s = 0.5 - 0.5 * cos(l.cam_phase[i] * core::f64::consts::PI * 2.0);
        l.cam_yaw[i] = l.cam_a[i] + (l.cam_b[i] - l.cam_a[i]) * s;
    } else if !sweep {
        l.cam_yaw[i] = l.cam_rest[i];
    }
    let (pitch, yaw) = (l.cam_pitch[i], l.cam_yaw[i]);
    let cp = cos(pitch);
    let look = [sin(yaw) * cp, sin(pitch), cos(yaw) * cp];
    let r = l.cam_row[i] as usize;
    l.dx[r] = look[0];
    l.dy[r] = look[1];
    l.dz[r] = look[2];
    l.x[r] = l.cam_px[i] + look[0] * CONE_AHEAD;
    l.y[r] = l.cam_py[i] + look[1] * CONE_AHEAD;
    l.z[r] = l.cam_pz[i] + look[2] * CONE_AHEAD;
    let i0 = l.intensity[r];
    burn(l, r, ok && cones, i0);
    let mut out = if ok { CAM_OK } else { 0 };
    let broken = l.bits[t] & T_BROKEN != 0;
    if broken && l.cam_bits[i] & C_DEAD == 0 {
        l.cam_bits[i] |= C_DEAD;
        l.cam_seen[i] = 0.0;
        l.cam_state[i] = S_IDLE;
        out |= CAM_DOWN;
    } else if !broken {
        l.cam_bits[i] &= !C_DEAD;
    }
    l.cam_bits[i] &= !C_SAW;
    if !ok || !live {
        l.cam_seen[i] = if ok { js_max(0.0, l.cam_seen[i] - CAMERA_DECAY * dt) } else { 0.0 };
        if !ok {
            l.cam_state[i] = S_IDLE;
        }
    }
    out
}

/// …AND ITS HALF AFTER: what the picture had, handed in. `visible`: the guards' eye test from the lens has him, at `gain` lit-seconds a second (Sam's meter, the
/// caller's) with his feet at `p`. The integrator, then what it does about it; the report's words are the caller's, on the bits.
#[allow(clippy::too_many_arguments)]
pub fn camera_see(g: usize, i: usize, time: f64, dt: f64, visible: bool, gain: f64, p: [f64; 3], suspicion: f64, alarm: f64) -> u32 {
    let l = &mut state(g).lights;
    if visible && gain > 0.02 {
        l.cam_seen[i] = js_min(alarm + 1.0, l.cam_seen[i] + gain * dt);
        l.cam_lx[i] = p[0];
        l.cam_ly[i] = p[1];
        l.cam_lz[i] = p[2];
        l.cam_bits[i] |= C_LAST | C_SAW;
    } else {
        l.cam_seen[i] = js_max(0.0, l.cam_seen[i] - CAMERA_DECAY * dt);
    }
    let mut out = 0;
    let last = l.cam_bits[i] & C_LAST != 0;
    if l.cam_state[i] == S_IDLE && l.cam_seen[i] >= suspicion && last {
        l.cam_state[i] = S_SUSPECT;
        l.cam_reported[i] = time;
        out |= CAM_SUSPECT;
    }
    if l.cam_state[i] != S_ALARM && l.cam_seen[i] >= alarm && last && l.cam_bits[i] & C_SAW != 0 {
        l.cam_state[i] = S_ALARM;
        l.cam_reported[i] = time;
        out |= CAM_ALARM;
    }
    if l.cam_state[i] != S_IDLE && l.cam_seen[i] <= 0.0 && time - l.cam_reported[i] > CAMERA_REARM_SECS {
        l.cam_state[i] = S_IDLE;
    }
    out
}

/// THE TRIPWIRE'S OWN HALF BEFORE WHAT IS IN ITS BEAM (devices.ts updateLaser, its first lines): the feed dropping once, and no beam at all with no power.
pub fn laser_open(g: usize, i: usize) -> u32 {
    let l = &mut state(g).lights;
    let t = l.las_target[i] as usize;
    let ok = alive(l, t);
    let mut out = if ok { LASER_OK } else { 0 };
    let broken = l.bits[t] & T_BROKEN != 0;
    if broken && l.las_bits[i] & L_DEAD == 0 {
        l.las_bits[i] |= L_DEAD;
        out |= LASER_DOWN;
    } else if !broken {
        l.las_bits[i] &= !L_DEAD;
    }
    if !ok {
        l.las_cut[i] = 0.0;
        l.las_sense[i] = 0.0;
        l.las_bits[i] &= !L_HELD;
    }
    out
}
/// …AND AFTER: where the picture of the beam ends and where the sensor's does, and whether something breaks it. The trip is on the EDGE and not again inside the
/// cooldown, and only while devices sense (`live`).
pub fn laser_close(g: usize, i: usize, time: f64, live: bool, cut: f64, sense: f64, held: bool) -> u32 {
    let l = &mut state(g).lights;
    l.las_cut[i] = cut;
    l.las_sense[i] = sense;
    let edge = held && l.las_bits[i] & L_HELD == 0;
    if held {
        l.las_bits[i] |= L_HELD;
    } else {
        l.las_bits[i] &= !L_HELD;
    }
    if !live || !edge || time - l.las_last[i] < LASER_COOLDOWN {
        return 0;
    }
    l.las_last[i] = time;
    l.las_trips[i] += 1;
    LASER_TRIP
}
