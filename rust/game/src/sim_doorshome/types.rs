//! The types of `sim_doorshome`, written by the skeleton's program from the TypeScript. Never edited by hand: hand.cjs, and run it again.
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// (TypeScript: DoorStore, src/sim/doorshome.ts:35)
#[derive(Clone, Copy, Debug)]
pub struct DoorStore {
    pub base: f64,
    pub stride: f64,
    pub home: crate::Opaque,
    pub cap: f64,
}
impl DoorStore {
    /// what a new one holds: the TypeScript's field initialisers, before its constructor
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

/// (TypeScript: DoorIo, src/sim/doorshome.ts:70)
#[derive(Clone, Copy, Debug)]
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
impl DoorIo {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DoorIo {
            x: crate::sim_sim::SimExports::zeroed(),
            def: 0.0,
            out: 0.0,
            con: 0.0,
            ev: 0.0,
            par: 0.0,
            max_contacts: 0.0,
            max_events: 0.0,
        }
    }
}
impl Default for DoorIo { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/sim/doorshome.ts:18 DC, src/sim/doorshome.ts:18)
#[derive(Clone, Copy, Debug)]
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
    pub pos_angle: f64,
    pub pos_x: f64,
    pub pos_z: f64,
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
impl DC {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DC {
            angle: 0.0,
            vel: 0.0,
            quiet_t: 0.0,
            latched: 0.0,
            last_who: 0.0,
            since_bash: 0.0,
            closing: 0.0,
            locked: 0.0,
            lock_broken: 0.0,
            noticed: 0.0,
            noticed_t: 0.0,
            pick: 0.0,
            picking: 0.0,
            pick_idle: 0.0,
            pick_clock: 0.0,
            pick_n: 0.0,
            rattle_t: 0.0,
            guard_t: 0.0,
            passers: 0.0,
            dealt: 0.0,
            outward: 0.0,
            clearway: 0.0,
            pull_t: 0.0,
            vel_seen: 0.0,
            swept: 0.0,
            solid_t: 0.0,
            press_t: 0.0,
            open_t: 0.0,
            mirrored: 0.0,
            angle_seen: 0.0,
            off: 0.0,
            shot: 0.0,
            creak_armed: 0.0,
            pos_angle: 0.0,
            pos_x: 0.0,
            pos_z: 0.0,
            drive: 0.0,
            drive_who: 0.0,
            drive_dir: 0.0,
            drive_kind: 0.0,
            drive_t: 0.0,
            drive_from: 0.0,
            drive_quiet: 0.0,
            drive_side: 0.0,
            drive_rate: 0.0,
            drive_px: 0.0,
            drive_pz: 0.0,
            drive_hit: 0.0,
        }
    }
}
impl Default for DC { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/sim/doorshome.ts:48 DD, src/sim/doorshome.ts:48)
#[derive(Clone, Copy, Debug)]
pub struct DD {
    pub hx: f64,
    pub hz: f64,
    pub closed: f64,
    pub width: f64,
    pub height: f64,
    pub thick: f64,
    pub min_auth: f64,
    pub max_auth: f64,
    pub key_side: f64,
    pub closer: f64,
    pub slab: f64,
    pub slab_ang: f64,
    pub fcx: f64,
    pub fcz: f64,
    pub man_last: f64,
}
impl DD {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DD {
            hx: 0.0,
            hz: 0.0,
            closed: 0.0,
            width: 0.0,
            height: 0.0,
            thick: 0.0,
            min_auth: 0.0,
            max_auth: 0.0,
            key_side: 0.0,
            closer: 0.0,
            slab: 0.0,
            slab_ang: 0.0,
            fcx: 0.0,
            fcz: 0.0,
            man_last: 0.0,
        }
    }
}
impl Default for DD { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/sim/doorshome.ts:50 DO, src/sim/doorshome.ts:50)
#[derive(Clone, Copy, Debug)]
pub struct DO {
    pub placed: f64,
    pub h1x: f64,
    pub h1z: f64,
    pub h2x: f64,
    pub h2z: f64,
}
impl DO {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DO {
            placed: 0.0,
            h1x: 0.0,
            h1z: 0.0,
            h2x: 0.0,
            h2z: 0.0,
        }
    }
}
impl Default for DO { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/sim/doorshome.ts:52 DK, src/sim/doorshome.ts:52)
#[derive(Clone, Copy, Debug)]
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
impl DK {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DK {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            radius: 0.0,
            flags: 0.0,
            who: 0.0,
            hand: 0.0,
            run_bash: 0.0,
            run_rip: 0.0,
            run_level: 0.0,
            vx: 0.0,
            vz: 0.0,
            man: 0.0,
            opens: 0.0,
            opens_hi: 0.0,
            blocked_by: 0.0,
            bashed: 0.0,
            ripped: 0.0,
            bashed_at: 0.0,
            struck: 0.0,
            struck_by: 0.0,
            struck_tier: 0.0,
            struck_floor: 0.0,
            struck_dx: 0.0,
            struck_dz: 0.0,
            moved: 0.0,
            man_asked: 0.0,
        }
    }
}
impl Default for DK { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/sim/doorshome.ts:54 KF, src/sim/doorshome.ts:54)
#[derive(Clone, Copy, Debug)]
pub struct KF {
    pub bash: f64,
    pub quiet: f64,
    pub weak: f64,
    pub key: f64,
    pub solid: f64,
    pub round: f64,
    pub vel: f64,
    pub run: f64,
    pub hand: f64,
}
impl KF {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        KF {
            bash: 0.0,
            quiet: 0.0,
            weak: 0.0,
            key: 0.0,
            solid: 0.0,
            round: 0.0,
            vel: 0.0,
            run: 0.0,
            hand: 0.0,
        }
    }
}
impl Default for KF { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/sim/doorshome.ts:56 DE, src/sim/doorshome.ts:56)
#[derive(Clone, Copy, Debug)]
pub struct DE {
    pub door: f64,
    pub sound: f64,
    pub who: f64,
    pub level: f64,
    pub fresh: f64,
    pub px: f64,
    pub pz: f64,
}
impl DE {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DE {
            door: 0.0,
            sound: 0.0,
            who: 0.0,
            level: 0.0,
            fresh: 0.0,
            px: 0.0,
            pz: 0.0,
        }
    }
}
impl Default for DE { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/sim/doorshome.ts:59 DP, src/sim/doorshome.ts:59)
#[derive(Clone, Copy, Debug)]
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
impl DP {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DP {
            dt: 0.0,
            doors: 0.0,
            contacts: 0.0,
            flags: 0.0,
            hand_rate: 0.0,
            min_speed: 0.0,
            floor_rad: 0.0,
            rate: 0.0,
            slide: 0.0,
            again: 0.0,
            wmin_x: 0.0,
            wmax_x: 0.0,
            wmin_z: 0.0,
            wmax_z: 0.0,
            foot: 0.0,
            settle_y1: 0.0,
            events: 0.0,
            carried: 0.0,
            stopped: 0.0,
            passed: 0.0,
            reached: 0.0,
            held: 0.0,
            aside: 0.0,
            rounds: 0.0,
            wrong: 0.0,
            turned: 0.0,
        }
    }
}
impl Default for DP { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/sim/doorshome.ts:61 DF, src/sim/doorshome.ts:61)
#[derive(Clone, Copy, Debug)]
pub struct DF {
    pub one_way: f64,
    pub exits_out: f64,
    pub leaf_way: f64,
    pub stay: f64,
    pub throws: f64,
    pub soft: f64,
    pub standable: f64,
    pub settle: f64,
}
impl DF {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        DF {
            one_way: 0.0,
            exits_out: 0.0,
            leaf_way: 0.0,
            stay: 0.0,
            throws: 0.0,
            soft: 0.0,
            standable: 0.0,
            settle: 0.0,
        }
    }
}
impl Default for DF { fn default() -> Self { Self::zeroed() } }

/// (TypeScript: inline at src/sim/doorshome.ts:83 DOORS_STEPPED, src/sim/doorshome.ts:83)
#[derive(Clone, Debug)]
pub struct SimDoorshomeModDOORSSTEPPED {
    pub there: f64,
    pub here: f64,
    pub turned: f64,
    pub noises: f64,
    pub both: f64,
    pub differ: f64,
    pub first: String,
}
impl SimDoorshomeModDOORSSTEPPED {
    /// every field at zero. NOT what a new one holds: the TypeScript gives this type no initial values of its own
    pub fn zeroed() -> Self {
        SimDoorshomeModDOORSSTEPPED {
            there: 0.0,
            here: 0.0,
            turned: 0.0,
            noises: 0.0,
            both: 0.0,
            differ: 0.0,
            first: String::new(),
        }
    }
}
impl Default for SimDoorshomeModDOORSSTEPPED { fn default() -> Self { Self::zeroed() } }

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
pub fn dc() -> crate::sim_doorshome::DC { crate::sim_doorshome::DC { angle: 0.0, vel: 1.0, quiet_t: 2.0, latched: 3.0, last_who: 4.0, since_bash: 5.0, closing: 6.0, locked: 7.0, lock_broken: 8.0, noticed: 9.0, noticed_t: 10.0, pick: 11.0, picking: 12.0, pick_idle: 13.0, pick_clock: 14.0, pick_n: 15.0, rattle_t: 16.0, guard_t: 17.0, passers: 18.0, dealt: 19.0, outward: 20.0, clearway: 21.0, pull_t: 22.0, vel_seen: 23.0, swept: 24.0, solid_t: 25.0, press_t: 26.0, open_t: 27.0, mirrored: 28.0, angle_seen: 29.0, off: 30.0, shot: 31.0, creak_armed: 32.0, pos_angle: 33.0, pos_x: 34.0, pos_z: 35.0, drive: 36.0, drive_who: 37.0, drive_dir: 38.0, drive_kind: 39.0, drive_t: 40.0, drive_from: 41.0, drive_quiet: 42.0, drive_side: 43.0, drive_rate: 44.0, drive_px: 45.0, drive_pz: 46.0, drive_hit: 47.0 } }

/// (src/sim/doorshome.ts:22 DOOR_COLS)
pub const DOOR_COLS: f64 = 48.0;

/// (src/sim/doorshome.ts:24 DRIVE_KINDS: a table nothing writes)
pub fn drive_kinds() -> [String; 4] { [String::from("boot"), String::from("swing"), String::from("body"), String::from("hand")] }

/// (src/sim/doorshome.ts:26 DOOR_SOUNDS: a table nothing writes)
pub fn door_sounds() -> [String; 10] { [String::from("creak"), String::from("bang"), String::from("latch"), String::from("bash"), String::from("rattle"), String::from("pick"), String::from("unlock"), String::from("kick"), String::from("clang"), String::from("thud")] }

/// (src/sim/doorshome.ts:28 STRIKE_TIERS: a table nothing writes)
pub fn strike_tiers() -> [String; 3] { [String::from("open"), String::from("slam"), String::from("boot")] }

/// (src/sim/doorshome.ts:31 DOOR_BIT)
pub const DOOR_BIT: f64 = 4.0;

/// (src/sim/doorshome.ts:48 DD: a table nothing writes)
pub fn dd() -> crate::sim_doorshome::DD { crate::sim_doorshome::DD { hx: 0.0, hz: 1.0, closed: 2.0, width: 3.0, height: 4.0, thick: 5.0, min_auth: 6.0, max_auth: 7.0, key_side: 8.0, closer: 9.0, slab: 10.0, slab_ang: 11.0, fcx: 12.0, fcz: 13.0, man_last: 14.0 } }

/// (src/sim/doorshome.ts:50 DO: a table nothing writes)
pub fn do_() -> crate::sim_doorshome::DO { crate::sim_doorshome::DO { placed: 0.0, h1x: 1.0, h1z: 2.0, h2x: 3.0, h2z: 4.0 } }

/// (src/sim/doorshome.ts:52 DK: a table nothing writes)
pub fn dk() -> crate::sim_doorshome::DK { crate::sim_doorshome::DK { x: 0.0, y: 1.0, z: 2.0, radius: 3.0, flags: 4.0, who: 5.0, hand: 6.0, run_bash: 7.0, run_rip: 8.0, run_level: 9.0, vx: 10.0, vz: 11.0, man: 12.0, opens: 13.0, opens_hi: 14.0, blocked_by: 15.0, bashed: 16.0, ripped: 17.0, bashed_at: 18.0, struck: 19.0, struck_by: 20.0, struck_tier: 21.0, struck_floor: 22.0, struck_dx: 23.0, struck_dz: 24.0, moved: 25.0, man_asked: 26.0 } }

/// (src/sim/doorshome.ts:54 KF: a table nothing writes)
pub fn kf() -> crate::sim_doorshome::KF { crate::sim_doorshome::KF { bash: 1.0, quiet: 2.0, weak: 4.0, key: 8.0, solid: 16.0, round: 32.0, vel: 64.0, run: 128.0, hand: 256.0 } }

/// (src/sim/doorshome.ts:56 DE: a table nothing writes)
pub fn de() -> crate::sim_doorshome::DE { crate::sim_doorshome::DE { door: 0.0, sound: 1.0, who: 2.0, level: 3.0, fresh: 4.0, px: 5.0, pz: 6.0 } }

/// (src/sim/doorshome.ts:57 DOOR_EV)
pub const DOOR_EV: f64 = 7.0;

/// (src/sim/doorshome.ts:59 DP: a table nothing writes)
pub fn dp() -> crate::sim_doorshome::DP { crate::sim_doorshome::DP { dt: 0.0, doors: 1.0, contacts: 2.0, flags: 3.0, hand_rate: 4.0, min_speed: 5.0, floor_rad: 6.0, rate: 7.0, slide: 8.0, again: 9.0, wmin_x: 10.0, wmax_x: 11.0, wmin_z: 12.0, wmax_z: 13.0, foot: 14.0, settle_y1: 15.0, events: 16.0, carried: 17.0, stopped: 18.0, passed: 19.0, reached: 20.0, held: 21.0, aside: 22.0, rounds: 23.0, wrong: 24.0, turned: 25.0 } }

/// (src/sim/doorshome.ts:61 DF: a table nothing writes)
pub fn df() -> crate::sim_doorshome::DF { crate::sim_doorshome::DF { one_way: 1.0, exits_out: 2.0, leaf_way: 4.0, stay: 8.0, throws: 16.0, soft: 32.0, standable: 64.0, settle: 128.0 } }

/// (src/sim/doorshome.ts:63 MAN_UNASKED)
pub const MAN_UNASKED: f64 = -1.0;

/// (src/sim/doorshome.ts:66 DOOR_OUTS)
pub const DOOR_OUTS: f64 = 5.0;

/// (src/sim/doorshome.ts:67 DOOR_CONS)
pub const DOOR_CONS: f64 = 27.0;

/// (src/sim/doorshome.ts:68 DOOR_PARS)
pub const DOOR_PARS: f64 = 32.0;
