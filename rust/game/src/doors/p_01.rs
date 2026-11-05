//! src/game/doors.ts lines 1 to 296
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// A door's slab as its row holds it: the handle's ID is the door's number, and the row is found at each read.   (doors.ts SlabRow.constructor)
pub fn slab_row_new(g: &mut crate::game::Game, door: f64) -> SlabRowId {
    SlabRowId(door as u32)
}

/// One double of the slab's row, as it stands now.   (doors.ts SlabRow.g)
pub fn slab_row_g(g: &mut crate::game::Game, id: SlabRowId, k: crate::doors::SlabRowGK) -> f64 {
    let r = row_now(g, id);
    slab_col(&::sim::state::state(g.sim).slabs, k, r)
}

/// The slab row's word of truths.   (doors.ts SlabRow.f)
pub fn slab_row_f(g: &mut crate::game::Game, id: SlabRowId) -> f64 {
    let r = row_now(g, id);
    ::sim::state::state(g.sim).slabs.flags[r] as f64
}

/// Its centre.   (doors.ts SlabRow.p)
pub fn slab_row_p(g: &mut crate::game::Game, id: SlabRowId) -> [f64; 3] {
    let x = slab_row_g(g, id, SlabRowGK::Px);
    let y = slab_row_g(g, id, SlabRowGK::Py);
    let z = slab_row_g(g, id, SlabRowGK::Pz);
    [x, y, z]
}

/// Its velocity.   (doors.ts SlabRow.v)
pub fn slab_row_v(g: &mut crate::game::Game, id: SlabRowId) -> [f64; 3] {
    let x = slab_row_g(g, id, SlabRowGK::Vx);
    let y = slab_row_g(g, id, SlabRowGK::Vy);
    let z = slab_row_g(g, id, SlabRowGK::Vz);
    [x, y, z]
}

/// The world angle of its width axis.   (doors.ts SlabRow.a)
pub fn slab_row_a(g: &mut crate::game::Game, id: SlabRowId) -> f64 {
    slab_row_g(g, id, SlabRowGK::A)
}

/// Its turn.   (doors.ts SlabRow.spin)
pub fn slab_row_spin(g: &mut crate::game::Game, id: SlabRowId) -> f64 {
    slab_row_g(g, id, SlabRowGK::Spin)
}

/// 0 upright to π/2 flat.   (doors.ts SlabRow.tip)
pub fn slab_row_tip(g: &mut crate::game::Game, id: SlabRowId) -> f64 {
    slab_row_g(g, id, SlabRowGK::Tip)
}

/// How fast it is tipping.   (doors.ts SlabRow.tipV)
pub fn slab_row_tip_v(g: &mut crate::game::Game, id: SlabRowId) -> f64 {
    slab_row_g(g, id, SlabRowGK::TipV)
}

/// The slab's own clock.   (doors.ts SlabRow.t)
pub fn slab_row_t(g: &mut crate::game::Game, id: SlabRowId) -> f64 {
    slab_row_g(g, id, SlabRowGK::T)
}

/// The angle the door reads as while off.   (doors.ts SlabRow.ang)
pub fn slab_row_ang(g: &mut crate::game::Game, id: SlabRowId) -> f64 {
    slab_row_g(g, id, SlabRowGK::Ang)
}

/// The clock before which a slab with no clear spot is not searched for again.   (doors.ts SlabRow.wedged)
pub fn slab_row_wedged(g: &mut crate::game::Game, id: SlabRowId) -> f64 {
    slab_row_g(g, id, SlabRowGK::Wedged)
}

/// The way its top fell: +1 the CCW normal of its width axis, −1 the other.   (doors.ts SlabRow.side)
pub fn slab_row_side(g: &mut crate::game::Game, id: SlabRowId) -> f64 {
    let f = slab_row_f(g, id);
    if flag(f, crate::sim_slabshome::slf().back) { -1.0 } else { 1.0 }
}

/// Lying at rest.   (doors.ts SlabRow.rest)
pub fn slab_row_rest(g: &mut crate::game::Game, id: SlabRowId) -> bool {
    let f = slab_row_f(g, id);
    flag(f, crate::sim_slabshome::slf().rest)
}

/// Slammed flat.   (doors.ts SlabRow.landed)
pub fn slab_row_landed(g: &mut crate::game::Game, id: SlabRowId) -> bool {
    let f = slab_row_f(g, id);
    flag(f, crate::sim_slabshome::slf().landed)
}

/// Whose doing it was.   (doors.ts SlabRow.who)
pub fn slab_row_who(g: &mut crate::game::Game, id: SlabRowId) -> f64 {
    let r = row_now(g, id);
    ::sim::state::state(g.sim).slabs.who[r] as f64
}

/// The lock rail of a leaf gunfire took apart: half width, half length, where its handles are.   (doors.ts SlabRow.bits)
pub fn slab_row_bits(g: &mut crate::game::Game, id: SlabRowId) -> Option<[f64; 4]> {
    let f = slab_row_f(g, id);
    if !flag(f, crate::sim_slabshome::slf().bits) {
        return None;
    }
    let w = slab_row_g(g, id, SlabRowGK::Bw);
    let l = slab_row_g(g, id, SlabRowGK::Bl);
    let x = slab_row_g(g, id, SlabRowGK::Bx);
    let y = slab_row_g(g, id, SlabRowGK::By);
    Some([w, l, x, y])
}

/// A door's drive as its columns hold it: the handle's ID is the door's number.   (doors.ts DriveCols.constructor)
pub fn drive_cols_new(g: &mut crate::game::Game, i: f64) -> DriveColsId {
    DriveColsId(i as u32)
}

/// One column of this door.   (doors.ts DriveCols.g)
pub fn drive_cols_g(g: &mut crate::game::Game, id: DriveColsId, k: f64) -> f64 {
    ::sim::state::state(g.sim).doors.c[k as usize][id.0 as usize]
}

/// Write one column of this door.   (doors.ts DriveCols.p)
pub fn drive_cols_p(g: &mut crate::game::Game, id: DriveColsId, k: f64, v: f64) {
    ::sim::state::state(g.sim).doors.c[k as usize][id.0 as usize] = v;
}

/// The man who drove it.   (doors.ts DriveCols.who)
pub fn drive_cols_who(g: &mut crate::game::Game, id: DriveColsId) -> f64 {
    drive_cols_g(g, id, crate::sim_doorshome::dc().drive_who)
}

/// (doors.ts DriveCols.who)
pub fn drive_cols_set_who(g: &mut crate::game::Game, id: DriveColsId, v: f64) {
    drive_cols_p(g, id, crate::sim_doorshome::dc().drive_who, v)
}

/// The sign of the turn.   (doors.ts DriveCols.dir)
pub fn drive_cols_dir(g: &mut crate::game::Game, id: DriveColsId) -> f64 {
    drive_cols_g(g, id, crate::sim_doorshome::dc().drive_dir)
}

/// (doors.ts DriveCols.dir)
pub fn drive_cols_set_dir(g: &mut crate::game::Game, id: DriveColsId, v: f64) {
    drive_cols_p(g, id, crate::sim_doorshome::dc().drive_dir, v)
}

/// How it was driven.   (doors.ts DriveCols.kind)
pub fn drive_cols_kind(g: &mut crate::game::Game, id: DriveColsId) -> crate::doors::DriveColsKind {
    let n = drive_cols_g(g, id, crate::sim_doorshome::dc().drive_kind);
    kind_at(n)
}

/// (doors.ts DriveCols.kind)
pub fn drive_cols_set_kind(g: &mut crate::game::Game, id: DriveColsId, v: crate::doors::DriveColsSetKindV) {
    drive_cols_p(g, id, crate::sim_doorshome::dc().drive_kind, place_of(v))
}

/// Seconds since.   (doors.ts DriveCols.t)
pub fn drive_cols_t(g: &mut crate::game::Game, id: DriveColsId) -> f64 {
    drive_cols_g(g, id, crate::sim_doorshome::dc().drive_t)
}

/// (doors.ts DriveCols.t)
pub fn drive_cols_set_t(g: &mut crate::game::Game, id: DriveColsId, v: f64) {
    drive_cols_p(g, id, crate::sim_doorshome::dc().drive_t, v)
}

/// The angle it stood at.   (doors.ts DriveCols.from)
pub fn drive_cols_from(g: &mut crate::game::Game, id: DriveColsId) -> f64 {
    drive_cols_g(g, id, crate::sim_doorshome::dc().drive_from)
}

/// (doors.ts DriveCols.from)
pub fn drive_cols_set_from(g: &mut crate::game::Game, id: DriveColsId, v: f64) {
    drive_cols_p(g, id, crate::sim_doorshome::dc().drive_from, v)
}

/// A crouched, silent or calm man's.   (doors.ts DriveCols.quiet)
pub fn drive_cols_quiet(g: &mut crate::game::Game, id: DriveColsId) -> bool {
    drive_cols_g(g, id, crate::sim_doorshome::dc().drive_quiet) != 0.0
}

/// (doors.ts DriveCols.quiet)
pub fn drive_cols_set_quiet(g: &mut crate::game::Game, id: DriveColsId, v: bool) {
    drive_cols_p(g, id, crate::sim_doorshome::dc().drive_quiet, if v { 1.0 } else { 0.0 })
}

/// Whose man he was (MAN_SIDE).   (doors.ts DriveCols.side)
pub fn drive_cols_side(g: &mut crate::game::Game, id: DriveColsId) -> f64 {
    drive_cols_g(g, id, crate::sim_doorshome::dc().drive_side)
}

/// (doors.ts DriveCols.side)
pub fn drive_cols_set_side(g: &mut crate::game::Game, id: DriveColsId, v: f64) {
    drive_cols_p(g, id, crate::sim_doorshome::dc().drive_side, v)
}

/// Rad/s a hand or a stride was turning it at.   (doors.ts DriveCols.rate)
pub fn drive_cols_rate(g: &mut crate::game::Game, id: DriveColsId) -> f64 {
    drive_cols_g(g, id, crate::sim_doorshome::dc().drive_rate)
}

/// (doors.ts DriveCols.rate)
pub fn drive_cols_set_rate(g: &mut crate::game::Game, id: DriveColsId, v: f64) {
    drive_cols_p(g, id, crate::sim_doorshome::dc().drive_rate, v)
}

/// Where he stood when the last update ended, x.   (doors.ts DriveCols.px)
pub fn drive_cols_px(g: &mut crate::game::Game, id: DriveColsId) -> f64 {
    drive_cols_g(g, id, crate::sim_doorshome::dc().drive_px)
}

/// (doors.ts DriveCols.px)
pub fn drive_cols_set_px(g: &mut crate::game::Game, id: DriveColsId, v: f64) {
    drive_cols_p(g, id, crate::sim_doorshome::dc().drive_px, v)
}

/// Where he stood when the last update ended, z.   (doors.ts DriveCols.pz)
pub fn drive_cols_pz(g: &mut crate::game::Game, id: DriveColsId) -> f64 {
    drive_cols_g(g, id, crate::sim_doorshome::dc().drive_pz)
}

/// (doors.ts DriveCols.pz)
pub fn drive_cols_set_pz(g: &mut crate::game::Game, id: DriveColsId, v: f64) {
    drive_cols_p(g, id, crate::sim_doorshome::dc().drive_pz, v)
}

/// The man it last struck, −1 nobody.   (doors.ts DriveCols.hit)
pub fn drive_cols_hit(g: &mut crate::game::Game, id: DriveColsId) -> f64 {
    drive_cols_g(g, id, crate::sim_doorshome::dc().drive_hit)
}

/// (doors.ts DriveCols.hit)
pub fn drive_cols_set_hit(g: &mut crate::game::Game, id: DriveColsId, v: f64) {
    drive_cols_p(g, id, crate::sim_doorshome::dc().drive_hit, v)
}

/// A door's drive as a plain record, or None: `door_drive` already hands out a copy.   (doors.ts driveOf)
pub fn drive_of(g: &mut crate::game::Game, d: DoorId) -> Option<crate::doors::DoorDrive> {
    door_drive(g, d)
}

/// `T.rowOf(this.door)`, for a read of the row: a SlabRow is good only while its door has a slab (Door.slab answers null otherwise).
fn row_now(g: &mut crate::game::Game, id: SlabRowId) -> usize {
    let r = crate::sim_slabshome::slab_store_row_of(g, id.0 as f64);
    debug_assert!(r >= 0.0, "a SlabRow read with no slab on its door");
    r as usize
}

/// `d[b8 + O[k] + r]`: the word names the column of rust/sim's Slabs.
fn slab_col(s: &::sim::slabs::Slabs, k: crate::doors::SlabRowGK, r: usize) -> f64 {
    match k {
        SlabRowGK::Px => s.px[r],
        SlabRowGK::Py => s.py[r],
        SlabRowGK::Pz => s.pz[r],
        SlabRowGK::Vx => s.vx[r],
        SlabRowGK::Vy => s.vy[r],
        SlabRowGK::Vz => s.vz[r],
        SlabRowGK::A => s.a[r],
        SlabRowGK::Spin => s.spin[r],
        SlabRowGK::Tip => s.tip[r],
        SlabRowGK::TipV => s.tip_v[r],
        SlabRowGK::T => s.t[r],
        SlabRowGK::Ang => s.ang[r],
        SlabRowGK::Wedged => s.wedged[r],
        SlabRowGK::Bw => s.bw[r],
        SlabRowGK::Bl => s.bl[r],
        SlabRowGK::Bx => s.bx[r],
        SlabRowGK::By => s.by[r],
    }
}

/// `flags & bit` as a condition.
fn flag(f: f64, bit: f64) -> bool {
    (js::to_i32(f) & js::to_i32(bit)) != 0
}

/// `DRIVE_KINDS[n]`: the column holds the word's place in sim/doorshome.ts DRIVE_KINDS, not the Rust enum's order.
fn kind_at(n: f64) -> crate::doors::DriveColsKind {
    match n as usize {
        0 => DriveColsKind::Boot,
        1 => DriveColsKind::Swing,
        2 => DriveColsKind::Body,
        3 => DriveColsKind::Hand,
        _ => unreachable!("a drive's kind is a place in DRIVE_KINDS"),
    }
}

/// `DRIVE_KINDS.indexOf(v)`.
fn place_of(v: crate::doors::DriveColsSetKindV) -> f64 {
    match v {
        DriveColsSetKindV::Boot => 0.0,
        DriveColsSetKindV::Swing => 1.0,
        DriveColsSetKindV::Body => 2.0,
        DriveColsSetKindV::Hand => 3.0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The column's numbers are DRIVE_KINDS' places, the same as rust/sim's BOOT = 0 and on; the enums are declared in another order.
    #[test]
    fn drive_kinds_keep_their_places() {
        let words = crate::sim_doorshome::drive_kinds();
        for (i, w) in words.iter().enumerate() {
            assert_eq!(kind_at(i as f64).as_str(), w.as_str());
        }
        for v in [DriveColsSetKindV::Boot, DriveColsSetKindV::Swing, DriveColsSetKindV::Body, DriveColsSetKindV::Hand] {
            assert_eq!(words[place_of(v) as usize].as_str(), v.as_str());
        }
    }

    /// A word is a 32-bit AND, whatever else is set in it, and SLF's bits are rust/sim's.
    #[test]
    fn flags_are_bits_of_the_word() {
        assert!(flag(12.0, 4.0));
        assert!(!flag(11.0, 4.0));
        assert!(flag(4294967295.0, 8.0));
        let l = crate::sim_slabshome::slf();
        assert_eq!([l.rest, l.landed, l.back, l.bits], [::sim::slabs::F_REST as f64, ::sim::slabs::F_LANDED as f64, ::sim::slabs::F_BACK as f64, ::sim::slabs::F_BITS as f64]);
    }

    /// Each word reads its own column: rust/sim's fields written in SLAB_F64's order, each word found at its place in that list.
    #[test]
    fn a_word_reads_its_own_column() {
        let mut s: ::sim::slabs::Slabs = unsafe { core::mem::zeroed() };
        let cols: [&mut [f64; ::sim::slabs::CAP]; 17] = [&mut s.px, &mut s.py, &mut s.pz, &mut s.vx, &mut s.vy, &mut s.vz, &mut s.a, &mut s.spin, &mut s.tip, &mut s.tip_v, &mut s.t, &mut s.ang, &mut s.wedged, &mut s.bw, &mut s.bl, &mut s.bx, &mut s.by];
        for (i, c) in cols.into_iter().enumerate() {
            c[3] = (i + 1) as f64;
        }
        let names = crate::sim_slabshome::slab_f64();
        for k in [SlabRowGK::Px, SlabRowGK::Py, SlabRowGK::Pz, SlabRowGK::Vx, SlabRowGK::Vy, SlabRowGK::Vz, SlabRowGK::A, SlabRowGK::Spin, SlabRowGK::Tip, SlabRowGK::TipV, SlabRowGK::T, SlabRowGK::Ang, SlabRowGK::Wedged, SlabRowGK::Bw, SlabRowGK::Bl, SlabRowGK::Bx, SlabRowGK::By] {
            let at = names.iter().position(|n| n.as_str() == k.as_str()).unwrap();
            assert_eq!(slab_col(&s, k, 3), (at + 1) as f64, "{}", k.as_str());
            assert_eq!(slab_col(&s, k, 2), 0.0);
        }
    }

    /// A drive's accessors write the door's own cell of rust/sim's columns, and DC's numbers are rust/sim's.
    #[test]
    fn a_drive_writes_through_to_the_block() {
        use ::sim::doors as d;
        let mut g = crate::game::Game::blank();
        g.sim = 0;
        let id = drive_cols_new(&mut g, 5.0);
        drive_cols_set_who(&mut g, id, 1.0);
        drive_cols_set_dir(&mut g, id, -1.0);
        drive_cols_set_kind(&mut g, id, DriveColsSetKindV::Hand);
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
        assert_eq!(drive_cols_kind(&mut g, id), DriveColsKind::Hand);
        assert!(drive_cols_quiet(&mut g, id));
        assert_eq!(drive_cols_hit(&mut g, id), 11.0);
        assert_eq!(crate::doors::drive_cols(), [d::DRIVE_WHO, d::DRIVE_DIR, d::DRIVE_KIND, d::DRIVE_T, d::DRIVE_FROM, d::DRIVE_QUIET, d::DRIVE_SIDE, d::DRIVE_RATE, d::DRIVE_PX, d::DRIVE_PZ, d::DRIVE_HIT].map(|k| k as f64));
    }
}
