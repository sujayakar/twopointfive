use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// A door's drive as its columns hold it: the handle's ID is the door's number.
pub(super) fn drive_cols_new(g: &crate::game::Game, i: f64) -> DriveColsId {
    DriveColsId(i as u32)
}

/// One column of this door.
fn drive_cols_g(g: &crate::game::Game, id: DriveColsId, k: f64) -> f64 {
    ::sim::state::state_read(g.sim).doors.c[k as usize][id.0 as usize]
}

/// Write one column of this door.
fn drive_cols_p(g: &mut crate::game::Game, id: DriveColsId, k: f64, v: f64) {
    ::sim::state::state(g.sim).doors.c[k as usize][id.0 as usize] = v;
}

fn drive_cols_set_who(g: &mut crate::game::Game, id: DriveColsId, v: f64) {
    drive_cols_p(g, id, crate::sim_doorshome::dc().drive_who, v)
}

fn drive_cols_set_dir(g: &mut crate::game::Game, id: DriveColsId, v: f64) {
    drive_cols_p(g, id, crate::sim_doorshome::dc().drive_dir, v)
}

/// How it was driven.
fn drive_cols_kind(g: &mut crate::game::Game, id: DriveColsId) -> crate::doors::DoorDriveKind {
    let n = drive_cols_g(g, id, crate::sim_doorshome::dc().drive_kind);
    kind_at(n)
}

/// Answers what it wrote: the skeleton's signature gives the setter the word back.
fn drive_cols_set_kind(g: &mut crate::game::Game, id: DriveColsId, v: crate::doors::DoorDriveKind) -> crate::doors::DoorDriveKind {
    drive_cols_p(g, id, crate::sim_doorshome::dc().drive_kind, place_of(v));
    v
}

fn drive_cols_set_t(g: &mut crate::game::Game, id: DriveColsId, v: f64) {
    drive_cols_p(g, id, crate::sim_doorshome::dc().drive_t, v)
}

fn drive_cols_set_from(g: &mut crate::game::Game, id: DriveColsId, v: f64) {
    drive_cols_p(g, id, crate::sim_doorshome::dc().drive_from, v)
}

/// A crouched, silent or calm man's.
fn drive_cols_quiet(g: &mut crate::game::Game, id: DriveColsId) -> bool {
    drive_cols_g(g, id, crate::sim_doorshome::dc().drive_quiet) != 0.0
}

fn drive_cols_set_quiet(g: &mut crate::game::Game, id: DriveColsId, v: bool) {
    drive_cols_p(g, id, crate::sim_doorshome::dc().drive_quiet, if v { 1.0 } else { 0.0 })
}

fn drive_cols_set_side(g: &mut crate::game::Game, id: DriveColsId, v: f64) {
    drive_cols_p(g, id, crate::sim_doorshome::dc().drive_side, v)
}

fn drive_cols_set_rate(g: &mut crate::game::Game, id: DriveColsId, v: f64) {
    drive_cols_p(g, id, crate::sim_doorshome::dc().drive_rate, v)
}

fn drive_cols_set_px(g: &mut crate::game::Game, id: DriveColsId, v: f64) {
    drive_cols_p(g, id, crate::sim_doorshome::dc().drive_px, v)
}

fn drive_cols_set_pz(g: &mut crate::game::Game, id: DriveColsId, v: f64) {
    drive_cols_p(g, id, crate::sim_doorshome::dc().drive_pz, v)
}

/// The man it last struck, −1 nobody.
fn drive_cols_hit(g: &mut crate::game::Game, id: DriveColsId) -> f64 {
    drive_cols_g(g, id, crate::sim_doorshome::dc().drive_hit)
}

fn drive_cols_set_hit(g: &mut crate::game::Game, id: DriveColsId, v: f64) {
    drive_cols_p(g, id, crate::sim_doorshome::dc().drive_hit, v)
}

/// `DRIVE_KINDS[n]`: the column holds the word's place in sim/doorshome.ts DRIVE_KINDS, not the Rust enum's order.
fn kind_at(n: f64) -> crate::doors::DoorDriveKind {
    match n as usize {
        0 => DoorDriveKind::Boot,
        1 => DoorDriveKind::Swing,
        2 => DoorDriveKind::Body,
        3 => DoorDriveKind::Hand,
        _ => unreachable!("a drive's kind is a place in DRIVE_KINDS"),
    }
}

/// `DRIVE_KINDS.indexOf(v)`.
fn place_of(v: crate::doors::DoorDriveKind) -> f64 {
    match v {
        DoorDriveKind::Boot => 0.0,
        DoorDriveKind::Swing => 1.0,
        DoorDriveKind::Body => 2.0,
        DoorDriveKind::Hand => 3.0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A drive's accessors write the door's own cell of rust/sim's columns, and DC's numbers are rust/sim's.
    #[test]
    fn a_drive_writes_through_to_the_block() {
        use ::sim::doors as d;
        let mut g = crate::game::Game::blank();
        g.sim = 0;
        let id = drive_cols_new(&mut g, 5.0);
        drive_cols_set_who(&mut g, id, 1.0);
        drive_cols_set_dir(&mut g, id, -1.0);
        assert_eq!(drive_cols_set_kind(&mut g, id, DoorDriveKind::Hand), DoorDriveKind::Hand);
        drive_cols_set_t(&mut g, id, 4.0);
        drive_cols_set_from(&mut g, id, 5.0);
        drive_cols_set_quiet(&mut g, id, true);
        drive_cols_set_side(&mut g, id, 7.0);
        drive_cols_set_rate(&mut g, id, 8.0);
        drive_cols_set_px(&mut g, id, 9.0);
        drive_cols_set_pz(&mut g, id, 10.0);
        drive_cols_set_hit(&mut g, id, 11.0);
        let c = &::sim::state::state(0).doors.c;
        assert_eq!(
            [d::DRIVE_WHO, d::DRIVE_DIR, d::DRIVE_KIND, d::DRIVE_T, d::DRIVE_FROM, d::DRIVE_QUIET, d::DRIVE_SIDE, d::DRIVE_RATE, d::DRIVE_PX, d::DRIVE_PZ, d::DRIVE_HIT].map(|k| c[k][5]),
            [1.0, -1.0, 3.0, 4.0, 5.0, 1.0, 7.0, 8.0, 9.0, 10.0, 11.0]
        );
        assert_eq!(c[d::DRIVE_WHO][4], 0.0);
        assert_eq!(drive_cols_kind(&mut g, id), DoorDriveKind::Hand);
        assert!(drive_cols_quiet(&mut g, id));
        assert_eq!(drive_cols_hit(&mut g, id), 11.0);
        assert_eq!(crate::doors::drive_cols(), [d::DRIVE_WHO, d::DRIVE_DIR, d::DRIVE_KIND, d::DRIVE_T, d::DRIVE_FROM, d::DRIVE_QUIET, d::DRIVE_SIDE, d::DRIVE_RATE, d::DRIVE_PX, d::DRIVE_PZ, d::DRIVE_HIT].map(|k| k as f64));
    }
}
