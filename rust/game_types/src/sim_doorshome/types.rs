//! The types of `sim_doorshome`.
#![allow(unused_imports)]
use crate::{Blank, bytes::ToBytes, ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, ToBytes)]
pub struct DoorStore {
}
impl DoorStore {
    /// what a new one holds
    pub fn blank() -> Self {
        DoorStore {
        }
    }
}
impl Default for DoorStore { fn default() -> Self { Self::blank() } }

#[derive(Clone, Copy, Debug, Default)]
pub struct DoorIo {
    pub def: f64,
    pub out: f64,
    pub con: f64,
    pub ev: f64,
    pub par: f64,
    pub max_contacts: f64,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct DC {
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
    pub z: f64,
    pub man: f64,
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
    pub carried: f64,
    pub stopped: f64,
    pub passed: f64,
    pub reached: f64,
    pub held: f64,
    pub aside: f64,
    pub rounds: f64,
    pub turned: f64,
}

#[derive(Clone, Debug, Default)]
pub struct SimDoorshomeModDOORSSTEPPED {
    pub there: f64,
    pub here: f64,
    pub turned: f64,
    pub noises: f64,
}

/// the state this module keeps at its top level
#[derive(Clone, Debug, Blank)]
pub struct SimDoorshomeMod {
    pub io: Option<crate::sim_doorshome::DoorIo>,
    #[is(crate::sim_doorshome::SimDoorshomeModDOORSSTEPPED { there: 0.0, here: 0.0, turned: 0.0, noises: 0.0 })] pub doors_stepped: crate::sim_doorshome::SimDoorshomeModDOORSSTEPPED,
}

pub const DOOR_CAP: f64 = 64.0;

pub fn dc() -> crate::sim_doorshome::DC { crate::sim_doorshome::DC { drive_who: 34.0, drive_dir: 35.0, drive_kind: 36.0, drive_t: 37.0, drive_from: 38.0, drive_quiet: 39.0, drive_side: 40.0, drive_rate: 41.0, drive_px: 42.0, drive_pz: 43.0, drive_hit: 44.0 } }

pub fn drive_kinds() -> [String; 4] { [String::from("boot"), String::from("swing"), String::from("body"), String::from("hand")] }

pub fn door_sounds() -> [String; 10] { [String::from("creak"), String::from("bang"), String::from("latch"), String::from("bash"), String::from("rattle"), String::from("pick"), String::from("unlock"), String::from("kick"), String::from("clang"), String::from("thud")] }

pub fn strike_tiers() -> [String; 3] { [String::from("open"), String::from("slam"), String::from("boot")] }

pub fn do_() -> crate::sim_doorshome::DO { crate::sim_doorshome::DO { placed: 0.0, h1x: 1.0, h1z: 2.0, h2x: 3.0, h2z: 4.0 } }

pub fn dk() -> crate::sim_doorshome::DK { crate::sim_doorshome::DK { x: 0.0, z: 2.0, man: 12.0, blocked_by: 15.0, bashed: 16.0, ripped: 17.0, bashed_at: 18.0, struck: 19.0, struck_by: 20.0, struck_tier: 21.0, struck_floor: 22.0, struck_dx: 23.0, struck_dz: 24.0, moved: 25.0, man_asked: 26.0 } }

pub fn de() -> crate::sim_doorshome::DE { crate::sim_doorshome::DE { door: 0.0, sound: 1.0, who: 2.0, level: 3.0, px: 4.0, pz: 5.0 } }

pub const DOOR_EV: f64 = 6.0;

pub fn dp() -> crate::sim_doorshome::DP { crate::sim_doorshome::DP { carried: 17.0, stopped: 18.0, passed: 19.0, reached: 20.0, held: 21.0, aside: 22.0, rounds: 23.0, turned: 25.0 } }

pub const MAN_UNASKED: f64 = -1.0;

pub const DOOR_OUTS: f64 = 5.0;

