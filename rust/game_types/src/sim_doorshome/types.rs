//! The types of `sim_doorshome`.
#![allow(unused_imports)]
use crate::{bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, ToBytes)]
pub struct DoorStore {
    pub base: f64,
    pub stride: f64,
    /// (holds nothing: in no bytes)
    #[bytes(skip)]
    pub home: crate::Opaque,
    pub cap: f64,
}
impl DoorStore {
    /// what a new one holds
    pub fn blank() -> Self {
        DoorStore {
            base: 0.0,
            stride: 64.0,
            home: crate::Opaque,
            cap: 0.0,
        }
    }
}
impl Default for DoorStore { fn default() -> Self { Self::blank() } }

#[derive(Clone, Copy, Debug, Default)]
pub struct DoorIo {
    pub x: crate::sim_sim::SimExports,
    pub def: f64,
    pub out: f64,
    pub con: f64,
    pub ev: f64,
    pub par: f64,
    pub max_contacts: f64,
    pub max_events: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct DC {
    pub angle: f64,
    pub vel: f64,
    pub quiet_t: f64,
    pub latched: f64,
    pub last_who: f64,
    pub since_bash: f64,
    pub closing: f64,
    pub locked: f64,
    pub lock_broken: f64,
    pub noticed: f64,
    pub noticed_t: f64,
    pub pick: f64,
    pub picking: f64,
    pub pick_idle: f64,
    pub pick_clock: f64,
    pub pick_n: f64,
    pub rattle_t: f64,
    pub guard_t: f64,
    pub passers: f64,
    pub dealt: f64,
    pub outward: f64,
    pub clearway: f64,
    pub pull_t: f64,
    pub vel_seen: f64,
    pub swept: f64,
    pub solid_t: f64,
    pub press_t: f64,
    pub open_t: f64,
    pub mirrored: f64,
    pub angle_seen: f64,
    pub off: f64,
    pub shot: f64,
    pub creak_armed: f64,
    pub drive: f64,
    pub drive_who: f64,
    pub drive_dir: f64,
    pub drive_kind: f64,
    pub drive_t: f64,
    pub drive_from: f64,
    pub drive_quiet: f64,
    pub drive_side: f64,
    pub drive_rate: f64,
    pub drive_px: f64,
    pub drive_pz: f64,
    pub drive_hit: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct DO {
    pub placed: f64,
    pub h1x: f64,
    pub h1z: f64,
    pub h2x: f64,
    pub h2z: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct DK {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub radius: f64,
    pub flags: f64,
    pub who: f64,
    pub hand: f64,
    pub run_bash: f64,
    pub run_rip: f64,
    pub run_level: f64,
    pub vx: f64,
    pub vz: f64,
    pub man: f64,
    pub opens: f64,
    pub opens_hi: f64,
    pub blocked_by: f64,
    pub bashed: f64,
    pub ripped: f64,
    pub bashed_at: f64,
    pub struck: f64,
    pub struck_by: f64,
    pub struck_tier: f64,
    pub struck_floor: f64,
    pub struck_dx: f64,
    pub struck_dz: f64,
    pub moved: f64,
    pub man_asked: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct DE {
    pub door: f64,
    pub sound: f64,
    pub who: f64,
    pub level: f64,
    pub px: f64,
    pub pz: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct DP {
    pub dt: f64,
    pub doors: f64,
    pub contacts: f64,
    pub flags: f64,
    pub hand_rate: f64,
    pub min_speed: f64,
    pub floor_rad: f64,
    pub rate: f64,
    pub slide: f64,
    pub again: f64,
    pub wmin_x: f64,
    pub wmax_x: f64,
    pub wmin_z: f64,
    pub wmax_z: f64,
    pub foot: f64,
    pub settle_y1: f64,
    pub events: f64,
    pub carried: f64,
    pub stopped: f64,
    pub passed: f64,
    pub reached: f64,
    pub held: f64,
    pub aside: f64,
    pub rounds: f64,
    pub wrong: f64,
    pub turned: f64,
}

#[derive(Clone, Debug, Default)]
pub struct SimDoorshomeModDOORSSTEPPED {
    pub there: f64,
    pub here: f64,
    pub turned: f64,
    pub noises: f64,
    pub both: f64,
    pub differ: f64,
    pub first: String,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug)]
pub struct SimDoorshomeMod {
    /// (src/sim/doorshome.ts:71 IO)
    pub io: Option<crate::sim_doorshome::DoorIo>,
    /// (src/sim/doorshome.ts:83 DOORS_STEPPED)
    pub doors_stepped: crate::sim_doorshome::SimDoorshomeModDOORSSTEPPED,
}
impl SimDoorshomeMod {
    pub fn blank() -> Self {
        SimDoorshomeMod {
            io: None,
            doors_stepped: crate::sim_doorshome::SimDoorshomeModDOORSSTEPPED { there: 0.0, here: 0.0, turned: 0.0, noises: 0.0, both: 0.0, differ: 0.0, first: String::from("") },
        }
    }
}

/// (src/sim/doorshome.ts:16 DOOR_CAP)
pub const DOOR_CAP: f64 = 64.0;

/// (src/sim/doorshome.ts:18 DC: a table nothing writes)
pub fn dc() -> crate::sim_doorshome::DC { crate::sim_doorshome::DC { angle: 0.0, vel: 1.0, quiet_t: 2.0, latched: 3.0, last_who: 4.0, since_bash: 5.0, closing: 6.0, locked: 7.0, lock_broken: 8.0, noticed: 9.0, noticed_t: 10.0, pick: 11.0, picking: 12.0, pick_idle: 13.0, pick_clock: 14.0, pick_n: 15.0, rattle_t: 16.0, guard_t: 17.0, passers: 18.0, dealt: 19.0, outward: 20.0, clearway: 21.0, pull_t: 22.0, vel_seen: 23.0, swept: 24.0, solid_t: 25.0, press_t: 26.0, open_t: 27.0, mirrored: 28.0, angle_seen: 29.0, off: 30.0, shot: 31.0, creak_armed: 32.0, drive: 33.0, drive_who: 34.0, drive_dir: 35.0, drive_kind: 36.0, drive_t: 37.0, drive_from: 38.0, drive_quiet: 39.0, drive_side: 40.0, drive_rate: 41.0, drive_px: 42.0, drive_pz: 43.0, drive_hit: 44.0 } }

/// (src/sim/doorshome.ts:24 DRIVE_KINDS: a table nothing writes)
pub fn drive_kinds() -> [String; 4] { [String::from("boot"), String::from("swing"), String::from("body"), String::from("hand")] }

/// (src/sim/doorshome.ts:26 DOOR_SOUNDS: a table nothing writes)
pub fn door_sounds() -> [String; 10] { [String::from("creak"), String::from("bang"), String::from("latch"), String::from("bash"), String::from("rattle"), String::from("pick"), String::from("unlock"), String::from("kick"), String::from("clang"), String::from("thud")] }

/// (src/sim/doorshome.ts:28 STRIKE_TIERS: a table nothing writes)
pub fn strike_tiers() -> [String; 3] { [String::from("open"), String::from("slam"), String::from("boot")] }

/// (src/sim/doorshome.ts:50 DO: a table nothing writes)
pub fn do_() -> crate::sim_doorshome::DO { crate::sim_doorshome::DO { placed: 0.0, h1x: 1.0, h1z: 2.0, h2x: 3.0, h2z: 4.0 } }

/// (src/sim/doorshome.ts:52 DK: a table nothing writes)
pub fn dk() -> crate::sim_doorshome::DK { crate::sim_doorshome::DK { x: 0.0, y: 1.0, z: 2.0, radius: 3.0, flags: 4.0, who: 5.0, hand: 6.0, run_bash: 7.0, run_rip: 8.0, run_level: 9.0, vx: 10.0, vz: 11.0, man: 12.0, opens: 13.0, opens_hi: 14.0, blocked_by: 15.0, bashed: 16.0, ripped: 17.0, bashed_at: 18.0, struck: 19.0, struck_by: 20.0, struck_tier: 21.0, struck_floor: 22.0, struck_dx: 23.0, struck_dz: 24.0, moved: 25.0, man_asked: 26.0 } }

/// (src/sim/doorshome.ts:56 DE: a table nothing writes)
pub fn de() -> crate::sim_doorshome::DE { crate::sim_doorshome::DE { door: 0.0, sound: 1.0, who: 2.0, level: 3.0, px: 4.0, pz: 5.0 } }

/// (src/sim/doorshome.ts:57 DOOR_EV)
pub const DOOR_EV: f64 = 6.0;

/// (src/sim/doorshome.ts:59 DP: a table nothing writes)
pub fn dp() -> crate::sim_doorshome::DP { crate::sim_doorshome::DP { dt: 0.0, doors: 1.0, contacts: 2.0, flags: 3.0, hand_rate: 4.0, min_speed: 5.0, floor_rad: 6.0, rate: 7.0, slide: 8.0, again: 9.0, wmin_x: 10.0, wmax_x: 11.0, wmin_z: 12.0, wmax_z: 13.0, foot: 14.0, settle_y1: 15.0, events: 16.0, carried: 17.0, stopped: 18.0, passed: 19.0, reached: 20.0, held: 21.0, aside: 22.0, rounds: 23.0, wrong: 24.0, turned: 25.0 } }

/// (src/sim/doorshome.ts:63 MAN_UNASKED)
pub const MAN_UNASKED: f64 = -1.0;

/// (src/sim/doorshome.ts:66 DOOR_OUTS)
pub const DOOR_OUTS: f64 = 5.0;

