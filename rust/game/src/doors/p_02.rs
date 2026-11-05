//! src/game/doors.ts lines 296 to 689
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

// ---- the columns: value K of door i is `doors.c[K][i]` in the game's block (rust/sim doors.rs Doors); `i` is the door's own index, never moved
#[inline]
fn col(g: &mut Game, id: DoorId, k: usize) -> f64 {
    let i = g.dr[id].i as usize;
    ::sim::state::state(g.sim).doors.c[k][i]
}
#[inline]
fn set_col(g: &mut Game, id: DoorId, k: usize, v: f64) {
    let i = g.dr[id].i as usize;
    ::sim::state::state(g.sim).doors.c[k][i] = v;
}
#[inline]
fn set_flag(g: &mut Game, id: DoorId, k: usize, v: bool) {
    set_col(g, id, k, if v { 1.0 } else { 0.0 });
}

/// DoorDrive.kind as a column holds it: its place in doorshome.ts DRIVE_KINDS
fn drive_kind_of(n: f64) -> crate::doors::DoorDriveKind {
    match n as i32 {
        0 => crate::doors::DoorDriveKind::Boot,
        1 => crate::doors::DoorDriveKind::Swing,
        2 => crate::doors::DoorDriveKind::Body,
        _ => crate::doors::DoorDriveKind::Hand,
    }
}
fn drive_kind_num(k: crate::doors::DoorDriveKind) -> f64 {
    match k {
        crate::doors::DoorDriveKind::Boot => 0.0,
        crate::doors::DoorDriveKind::Swing => 1.0,
        crate::doors::DoorDriveKind::Body => 2.0,
        crate::doors::DoorDriveKind::Hand => 3.0,
    }
}

/// a door's drive as a plain record, or None   (doors.ts driveOf)
pub fn drive_of(g: &mut crate::game::Game, d: DoorId) -> Option<crate::doors::DoorDrive> {
    door_drive(g, d)
}

/// a column of this door   (doors.ts Door.g)
pub fn door_g(g: &mut crate::game::Game, id: DoorId, k: f64) -> f64 { col(g, id, k as usize) }
/// (doors.ts Door.p)
pub fn door_p(g: &mut crate::game::Game, id: DoorId, k: f64, v: f64) { set_col(g, id, k as usize, v) }

/// (doors.ts Door.angle)
pub fn door_angle(g: &mut crate::game::Game, id: DoorId) -> f64 { col(g, id, ::sim::doors::ANGLE) }
/// (doors.ts Door.angle)
pub fn door_set_angle(g: &mut crate::game::Game, id: DoorId, v: f64) { set_col(g, id, ::sim::doors::ANGLE, v) }
/// (doors.ts Door.vel)
pub fn door_vel(g: &mut crate::game::Game, id: DoorId) -> f64 { col(g, id, ::sim::doors::VEL) }
/// (doors.ts Door.vel)
pub fn door_set_vel(g: &mut crate::game::Game, id: DoorId, v: f64) { set_col(g, id, ::sim::doors::VEL, v) }
/// (doors.ts Door.quietT)
pub fn door_quiet_t(g: &mut crate::game::Game, id: DoorId) -> f64 { col(g, id, ::sim::doors::QUIET_T) }
/// (doors.ts Door.quietT)
pub fn door_set_quiet_t(g: &mut crate::game::Game, id: DoorId, v: f64) { set_col(g, id, ::sim::doors::QUIET_T, v) }
/// (doors.ts Door.latched)
pub fn door_latched(g: &mut crate::game::Game, id: DoorId) -> bool { col(g, id, ::sim::doors::LATCHED) != 0.0 }
/// (doors.ts Door.latched)
pub fn door_set_latched(g: &mut crate::game::Game, id: DoorId, v: bool) { set_flag(g, id, ::sim::doors::LATCHED, v) }
/// (doors.ts Door.lastWho)
pub fn door_last_who(g: &mut crate::game::Game, id: DoorId) -> f64 { col(g, id, ::sim::doors::LAST_WHO) }
/// (doors.ts Door.lastWho)
pub fn door_set_last_who(g: &mut crate::game::Game, id: DoorId, v: f64) { set_col(g, id, ::sim::doors::LAST_WHO, v) }
/// (doors.ts Door.sinceBash)
pub fn door_since_bash(g: &mut crate::game::Game, id: DoorId) -> f64 { col(g, id, ::sim::doors::SINCE_BASH) }
/// (doors.ts Door.sinceBash)
pub fn door_set_since_bash(g: &mut crate::game::Game, id: DoorId, v: f64) { set_col(g, id, ::sim::doors::SINCE_BASH, v) }
/// (doors.ts Door.closing)
pub fn door_closing(g: &mut crate::game::Game, id: DoorId) -> bool { col(g, id, ::sim::doors::CLOSING) != 0.0 }
/// (doors.ts Door.closing)
pub fn door_set_closing(g: &mut crate::game::Game, id: DoorId, v: bool) { set_flag(g, id, ::sim::doors::CLOSING, v) }
/// (doors.ts Door.locked)
pub fn door_locked(g: &mut crate::game::Game, id: DoorId) -> bool { col(g, id, ::sim::doors::LOCKED) != 0.0 }
/// (doors.ts Door.locked)
pub fn door_set_locked(g: &mut crate::game::Game, id: DoorId, v: bool) { set_flag(g, id, ::sim::doors::LOCKED, v) }
/// (doors.ts Door.lockBroken)
pub fn door_lock_broken(g: &mut crate::game::Game, id: DoorId) -> bool { col(g, id, ::sim::doors::LOCK_BROKEN) != 0.0 }
/// (doors.ts Door.lockBroken)
pub fn door_set_lock_broken(g: &mut crate::game::Game, id: DoorId, v: bool) { set_flag(g, id, ::sim::doors::LOCK_BROKEN, v) }
/// (doors.ts Door.noticed)
pub fn door_noticed(g: &mut crate::game::Game, id: DoorId) -> bool { col(g, id, ::sim::doors::NOTICED) != 0.0 }
/// (doors.ts Door.noticed)
pub fn door_set_noticed(g: &mut crate::game::Game, id: DoorId, v: bool) { set_flag(g, id, ::sim::doors::NOTICED, v) }
/// game time of that notice   (doors.ts Door.noticedT)
pub fn door_noticed_t(g: &mut crate::game::Game, id: DoorId) -> f64 { col(g, id, ::sim::doors::NOTICED_T) }
/// (doors.ts Door.noticedT)
pub fn door_set_noticed_t(g: &mut crate::game::Game, id: DoorId, v: f64) { set_col(g, id, ::sim::doors::NOTICED_T, v) }
/// (doors.ts Door.pick)
pub fn door_pick(g: &mut crate::game::Game, id: DoorId) -> f64 { col(g, id, ::sim::doors::PICK) }
/// (doors.ts Door.pick)
pub fn door_set_pick(g: &mut crate::game::Game, id: DoorId, v: f64) { set_col(g, id, ::sim::doors::PICK, v) }
/// (doors.ts Door.picking)
pub fn door_picking(g: &mut crate::game::Game, id: DoorId) -> bool { col(g, id, ::sim::doors::PICKING) != 0.0 }
/// (doors.ts Door.picking)
pub fn door_set_picking(g: &mut crate::game::Game, id: DoorId, v: bool) { set_flag(g, id, ::sim::doors::PICKING, v) }
/// (doors.ts Door.pickIdle)
pub fn door_pick_idle(g: &mut crate::game::Game, id: DoorId) -> f64 { col(g, id, ::sim::doors::PICK_IDLE) }
/// (doors.ts Door.pickIdle)
pub fn door_set_pick_idle(g: &mut crate::game::Game, id: DoorId, v: f64) { set_col(g, id, ::sim::doors::PICK_IDLE, v) }
/// (doors.ts Door.pickClock)
pub fn door_pick_clock(g: &mut crate::game::Game, id: DoorId) -> f64 { col(g, id, ::sim::doors::PICK_CLOCK) }
/// (doors.ts Door.pickClock)
pub fn door_set_pick_clock(g: &mut crate::game::Game, id: DoorId, v: f64) { set_col(g, id, ::sim::doors::PICK_CLOCK, v) }
/// (doors.ts Door.pickN)
pub fn door_pick_n(g: &mut crate::game::Game, id: DoorId) -> f64 { col(g, id, ::sim::doors::PICK_N) }
/// (doors.ts Door.pickN)
pub fn door_set_pick_n(g: &mut crate::game::Game, id: DoorId, v: f64) { set_col(g, id, ::sim::doors::PICK_N, v) }
/// (doors.ts Door.rattleT)
pub fn door_rattle_t(g: &mut crate::game::Game, id: DoorId) -> f64 { col(g, id, ::sim::doors::RATTLE_T) }
/// (doors.ts Door.rattleT)
pub fn door_set_rattle_t(g: &mut crate::game::Game, id: DoorId, v: f64) { set_col(g, id, ::sim::doors::RATTLE_T, v) }
/// (doors.ts Door.guardT)
pub fn door_guard_t(g: &mut crate::game::Game, id: DoorId) -> f64 { col(g, id, ::sim::doors::GUARD_T) }
/// (doors.ts Door.guardT)
pub fn door_set_guard_t(g: &mut crate::game::Game, id: DoorId, v: f64) { set_col(g, id, ::sim::doors::GUARD_T, v) }
/// the keyed men this leaf is passing through: a bit per contact id, as a number   (doors.ts Door.passers)
pub fn door_passers(g: &mut crate::game::Game, id: DoorId) -> f64 { col(g, id, ::sim::doors::PASSERS) }
/// (doors.ts Door.passers)
pub fn door_set_passers(g: &mut crate::game::Game, id: DoorId, v: f64) { set_col(g, id, ::sim::doors::PASSERS, v) }

/// THE SIDE IT OPENS INTO: the sign of every angle it can stand at; the page's DOORWAY: exitsOut off, leafWay on   (doors.ts Door.swing)
pub fn door_swing(g: &mut crate::game::Game, id: DoorId) -> f64 {
    let c = door_clearway(g, id);
    if js::truthy(c) { c } else { door_dealt(g, id) }
}
/// (doors.ts Door.dealt)
pub fn door_dealt(g: &mut crate::game::Game, id: DoorId) -> f64 { col(g, id, ::sim::doors::DEALT) }
/// (doors.ts Door.dealt)
pub fn door_set_dealt(g: &mut crate::game::Game, id: DoorId, v: f64) { set_col(g, id, ::sim::doors::DEALT, v) }
/// (doors.ts Door.outward)
pub fn door_outward(g: &mut crate::game::Game, id: DoorId) -> f64 { col(g, id, ::sim::doors::OUTWARD) }
/// (doors.ts Door.outward)
pub fn door_set_outward(g: &mut crate::game::Game, id: DoorId, v: f64) { set_col(g, id, ::sim::doors::OUTWARD, v) }
/// (doors.ts Door.clearway)
pub fn door_clearway(g: &mut crate::game::Game, id: DoorId) -> f64 { col(g, id, ::sim::doors::CLEARWAY) }
/// (doors.ts Door.clearway)
pub fn door_set_clearway(g: &mut crate::game::Game, id: DoorId, v: f64) { set_col(g, id, ::sim::doors::CLEARWAY, v) }

/// its lower stop: one-way, 0 on the side it does not open into   (doors.ts Door.minA)
pub fn door_min_a(g: &mut crate::game::Game, id: DoorId) -> f64 {
    if door_swing(g, id) > 0.0 { 0.0 } else { g.dr[id].min_auth }
}
/// its upper stop   (doors.ts Door.maxA)
pub fn door_max_a(g: &mut crate::game::Game, id: DoorId) -> f64 {
    if door_swing(g, id) < 0.0 { 0.0 } else { g.dr[id].max_auth }
}

/// (doors.ts Door.pullT)
pub fn door_pull_t(g: &mut crate::game::Game, id: DoorId) -> f64 { col(g, id, ::sim::doors::PULL_T) }
/// (doors.ts Door.pullT)
pub fn door_set_pull_t(g: &mut crate::game::Game, id: DoorId, v: f64) { set_col(g, id, ::sim::doors::PULL_T, v) }
/// (doors.ts Door.seenVel)
pub fn door_seen_vel(g: &mut crate::game::Game, id: DoorId) -> f64 { col(g, id, ::sim::doors::VEL_SEEN) }
/// (doors.ts Door.swept)
pub fn door_swept(g: &mut crate::game::Game, id: DoorId) -> bool { col(g, id, ::sim::doors::SWEPT) != 0.0 }
/// (doors.ts Door.swept)
pub fn door_set_swept(g: &mut crate::game::Game, id: DoorId, v: bool) { set_flag(g, id, ::sim::doors::SWEPT, v) }

/// who last drove it open and how. A COPY of the columns, where the TypeScript hands the live record: a change is written back whole (`door_set_drive`)   (doors.ts Door.drive)
pub fn door_drive(g: &mut crate::game::Game, id: DoorId) -> Option<crate::doors::DoorDrive> {
    if col(g, id, ::sim::doors::DRIVE) == 0.0 { return None; }
    Some(crate::doors::DoorDrive {
        who: col(g, id, ::sim::doors::DRIVE_WHO),
        dir: col(g, id, ::sim::doors::DRIVE_DIR),
        kind: drive_kind_of(col(g, id, ::sim::doors::DRIVE_KIND)),
        t: col(g, id, ::sim::doors::DRIVE_T),
        from: col(g, id, ::sim::doors::DRIVE_FROM),
        quiet: col(g, id, ::sim::doors::DRIVE_QUIET) != 0.0,
        side: col(g, id, ::sim::doors::DRIVE_SIDE),
        rate: col(g, id, ::sim::doors::DRIVE_RATE),
        px: col(g, id, ::sim::doors::DRIVE_PX),
        pz: col(g, id, ::sim::doors::DRIVE_PZ),
        hit: col(g, id, ::sim::doors::DRIVE_HIT),
    })
}
/// a drive that is over leaves nothing: its columns are zeros   (doors.ts Door.drive)
pub fn door_set_drive(g: &mut crate::game::Game, id: DoorId, v: Option<crate::doors::DoorDrive>) {
    let Some(v) = v else {
        set_col(g, id, ::sim::doors::DRIVE, 0.0);
        for k in ::sim::doors::DRIVE_WHO..=::sim::doors::DRIVE_HIT { set_col(g, id, k, 0.0); }
        return;
    };
    set_col(g, id, ::sim::doors::DRIVE_WHO, v.who);
    set_col(g, id, ::sim::doors::DRIVE_DIR, v.dir);
    set_col(g, id, ::sim::doors::DRIVE_KIND, drive_kind_num(v.kind));
    set_col(g, id, ::sim::doors::DRIVE_T, v.t);
    set_col(g, id, ::sim::doors::DRIVE_FROM, v.from);
    set_flag(g, id, ::sim::doors::DRIVE_QUIET, v.quiet);
    set_col(g, id, ::sim::doors::DRIVE_SIDE, v.side);
    set_col(g, id, ::sim::doors::DRIVE_RATE, v.rate);
    set_col(g, id, ::sim::doors::DRIVE_PX, v.px);
    set_col(g, id, ::sim::doors::DRIVE_PZ, v.pz);
    set_col(g, id, ::sim::doors::DRIVE_HIT, v.hit);
    set_col(g, id, ::sim::doors::DRIVE, 1.0);
}

/// come to rest in its frame by a body's sweep: to, not latched   (doors.ts Door.restsTo)
pub fn door_rests_to(g: &mut crate::game::Game, id: DoorId) -> bool {
    door_swept(g, id) && !door_latched(g, id) && door_angle(g, id) == 0.0 && door_vel(g, id) == 0.0
}
/// (doors.ts Door.solidT)
pub fn door_solid_t(g: &mut crate::game::Game, id: DoorId) -> f64 { col(g, id, ::sim::doors::SOLID_T) }
/// (doors.ts Door.solidT)
pub fn door_set_solid_t(g: &mut crate::game::Game, id: DoorId, v: f64) { set_col(g, id, ::sim::doors::SOLID_T, v) }
/// (doors.ts Door.pressT)
pub fn door_press_t(g: &mut crate::game::Game, id: DoorId) -> f64 { col(g, id, ::sim::doors::PRESS_T) }
/// (doors.ts Door.pressT)
pub fn door_set_press_t(g: &mut crate::game::Game, id: DoorId, v: f64) { set_col(g, id, ::sim::doors::PRESS_T, v) }
/// (doors.ts Door.openT)
pub fn door_open_t(g: &mut crate::game::Game, id: DoorId) -> f64 { col(g, id, ::sim::doors::OPEN_T) }
/// (doors.ts Door.openT)
pub fn door_set_open_t(g: &mut crate::game::Game, id: DoorId, v: f64) { set_col(g, id, ::sim::doors::OPEN_T, v) }
/// (doors.ts Door.mirrored)
pub fn door_mirrored(g: &mut crate::game::Game, id: DoorId) -> f64 { col(g, id, ::sim::doors::MIRRORED) }
/// (doors.ts Door.mirrored)
pub fn door_set_mirrored(g: &mut crate::game::Game, id: DoorId, v: f64) { set_col(g, id, ::sim::doors::MIRRORED, v) }
/// off its hinges: to whoever reads a door, a doorway   (doors.ts Door.off)
pub fn door_off(g: &mut crate::game::Game, id: DoorId) -> bool { col(g, id, ::sim::doors::OFF) != 0.0 }
/// (doors.ts Door.off)
pub fn door_set_off(g: &mut crate::game::Game, id: DoorId, v: bool) { set_flag(g, id, ::sim::doors::OFF, v) }

/// the loose slab while the leaf is off. A COPY of the row as it stands now, where the TypeScript hands a view that reads the row at each field   (doors.ts Door.slab)
pub fn door_slab(g: &mut crate::game::Game, id: DoorId) -> Option<crate::doors::SlabSeen> {
    let (i, r) = (g.dr[id].i, g.dr[id].slab_row);
    if crate::sim_slabshome::slab_store_row_of(g, i) < 0.0 { return None; }
    Some(crate::doors::SlabSeen {
        p: slab_row_p(g, r),
        v: slab_row_v(g, r),
        a: slab_row_a(g, r),
        spin: slab_row_spin(g, r),
        tip: slab_row_tip(g, r),
        tip_v: slab_row_tip_v(g, r),
        side: slab_row_side(g, r),
        rest: slab_row_rest(g, r),
        landed: slab_row_landed(g, r),
        who: slab_row_who(g, r),
        t: slab_row_t(g, r),
        ang: slab_row_ang(g, r),
        bits: slab_row_bits(g, r),
        wedged: Some(slab_row_wedged(g, r)),
    })
}
/// `a` and `b` are the same numbers, to the bit (what `s === this.slabRow` asked)
fn same_slab(a: &crate::doors::SlabSeen, b: &crate::doors::SlabSeen) -> bool {
    fn same(x: &[f64], y: &[f64]) -> bool { x.iter().zip(y.iter()).all(|(p, q)| p.to_bits() == q.to_bits()) }
    same(&a.p, &b.p) && same(&a.v, &b.v)
        && same(&[a.a, a.spin, a.tip, a.tip_v, a.side, a.who, a.t, a.ang], &[b.a, b.spin, b.tip, b.tip_v, b.side, b.who, b.t, b.ang])
        && a.rest == b.rest && a.landed == b.landed
        && match (&a.bits, &b.bits) { (Some(x), Some(y)) => same(x, y), (None, None) => true, _ => false }
        && a.wedged.unwrap_or(0.0).to_bits() == b.wedged.unwrap_or(0.0).to_bits()
}
/// sets a row whole, or with None gives the row up. Setting a door's slab to what `door_slab` just answered is nothing   (doors.ts Door.slab)
pub fn door_set_slab(g: &mut crate::game::Game, id: DoorId, s: Option<crate::doors::SlabSeen>) {
    let i = g.dr[id].i;
    let Some(s) = s else { crate::sim_slabshome::slab_store_leave(g, i); return; };
    if let Some(row) = door_slab(g, id) { if same_slab(&s, &row) { return; } }
    let now = crate::sim_slabshome::SlabNow { p: s.p, v: s.v, a: s.a, spin: s.spin, tip: s.tip, tip_v: s.tip_v, side: s.side, rest: s.rest, landed: s.landed, who: s.who, t: s.t, ang: s.ang, wedged: s.wedged, bits: s.bits };
    let r = crate::sim_slabshome::slab_store_enter(g, i, !now.rest);
    crate::sim_slabshome::slab_store_save(g, r, now);
}

/// the rounds' work this leaf has taken   (doors.ts Door.shot)
pub fn door_shot(g: &mut crate::game::Game, id: DoorId) -> f64 { col(g, id, ::sim::doors::SHOT) }
/// (doors.ts Door.shot)
pub fn door_set_shot(g: &mut crate::game::Game, id: DoorId, v: f64) { set_col(g, id, ::sim::doors::SHOT, v) }
/// (doors.ts Door.creakArmed)
pub fn door_creak_armed(g: &mut crate::game::Game, id: DoorId) -> bool { col(g, id, ::sim::doors::CREAK_ARMED) != 0.0 }
/// (doors.ts Door.creakArmed)
pub fn door_set_creak_armed(g: &mut crate::game::Game, id: DoorId, v: bool) { set_flag(g, id, ::sim::doors::CREAK_ARMED, v) }

/// a door of the level: its row of the columns set as authored, its boxes made   (doors.ts Door.constructor)
pub fn door_new(g: &mut crate::game::Game, def: &crate::doors::DoorDef, index: Option<f64>) -> DoorId {
    let index = index.unwrap_or(0.0);
    let mut door = crate::doors::Door::blank();
    door.i = index;
    door.slab_row = slab_row_new(g, index);
    door.drive_cols = drive_cols_new(g, index);
    door.def = def.clone();
    let id = g.dr.push(door);
    // (what the fields' own initialisers were)
    door_set_vel(g, id, 0.0); door_set_quiet_t(g, id, 10.0); door_set_last_who(g, id, -1.0); door_set_since_bash(g, id, 10.0); door_set_closing(g, id, false); door_set_lock_broken(g, id, false);
    door_set_noticed(g, id, false); door_set_noticed_t(g, id, -1000.0); door_set_pick(g, id, 0.0); door_set_picking(g, id, false);
    door_set_pick_idle(g, id, 0.0); door_set_pick_clock(g, id, 0.0); door_set_pick_n(g, id, 0.0); door_set_rattle_t(g, id, 0.0); door_set_guard_t(g, id, -100.0); door_set_passers(g, id, 0.0);
    door_set_outward(g, id, 0.0); door_set_clearway(g, id, 0.0); door_set_pull_t(g, id, 0.0); set_col(g, id, ::sim::doors::VEL_SEEN, 0.0); door_set_swept(g, id, false);
    door_set_drive(g, id, None); door_set_solid_t(g, id, 0.0); door_set_press_t(g, id, 0.0); door_set_open_t(g, id, 10.0); door_set_mirrored(g, id, 0.0); set_col(g, id, ::sim::doors::ANGLE_SEEN, 0.0);
    door_set_off(g, id, false); door_set_shot(g, id, 0.0); door_set_creak_armed(g, id, true); set_col(g, id, ::sim::doors::POS_ANGLE, m::NAN); set_col(g, id, ::sim::doors::POS_X, 0.0); set_col(g, id, ::sim::doors::POS_Z, 0.0);
    let height = def.height.unwrap_or(crate::scene_door_size::LEAF_H);
    let thick = def.thickness.unwrap_or(0.05);
    let min_auth = def.min_angle.unwrap_or(-1.92);
    let max_auth = def.max_angle.unwrap_or(1.92);
    g.dr[id].height = height;
    g.dr[id].thick = thick;
    g.dr[id].min_auth = min_auth;
    g.dr[id].max_auth = max_auth;
    let dealt = match def.swing { Some(s) => s, None => js::or(m::sign(def.angle.unwrap_or(0.0)), if max_auth >= -min_auth { 1.0 } else { -1.0 }) };   // (until dealSwings: the way it is authored standing open, else its wider side)
    door_set_dealt(g, id, dealt);
    door_set_locked(g, id, def.locked == Some(true));
    g.dr[id].key_side = def.key_side.unwrap_or(1.0);
    g.dr[id].lock_id = match &def.lock { Some(l) => l.clone(), None => String::from(FLOOR_KEY) };
    let (sided, lo, hi) = (door_sided(g, id, def.angle.unwrap_or(0.0)), door_min_a(g, id), door_max_a(g, id));
    door_set_angle(g, id, v3::clamp(sided, lo, hi));
    let a = door_angle(g, id);
    door_set_latched(g, id, m::abs(a) < 1e-3);
    let leaf_h: V3 = [def.width / 2.0 - 0.01, height / 2.0, thick / 2.0];
    g.dr[id].leaf_h = leaf_h;
    let flag = crate::scene_boxes::box_flag();
    let steel = def.steel == Some(true);
    let marks = if steel { None } else { Some(crate::scene_marking_defaults::door_leaf_marks(leaf_h, None)) };
    let leaf = crate::scene_boxes::make_box(g, &crate::scene_boxes::MakeBoxP {
        c: [0.0, 0.0, 0.0],
        h: leaf_h,
        albedo: Some(def.albedo.unwrap_or([0.42, 0.3, 0.19])),
        flags: Some(flag.dynamic),
        name: Some(alloc::format!("door_{}", def.name)),
        rough: Some(if steel { 0.3 } else { 0.55 }),
        metal: if steel { Some(true) } else { None },
        marks,
        ..crate::scene_boxes::MakeBoxP::zeroed()
    });
    g.dr[id].box_ = leaf;
    // push plate + lever handle (raster only)
    let handle_flags = (js::to_i32(flag.dynamic) | js::to_i32(flag.no_shadow)) as f64;
    let mut handles = Vec::new();
    for _ in 0..2 {
        handles.push(crate::scene_boxes::make_box(g, &crate::scene_boxes::MakeBoxP {
            c: [0.0, 0.0, 0.0],
            h: [0.06, 0.012, 0.012],
            albedo: Some([0.55, 0.55, 0.52]),
            stuff: Some(crate::scene_stuff::BoxStuff::Paint),
            flags: Some(handle_flags),
            rough: Some(0.25),
            metal: Some(true),
            ..crate::scene_boxes::MakeBoxP::zeroed()
        }));
    }
    g.dr[id].handle_boxes = handles;
    g.dr[id].leaf_marks = g.col.boxes[leaf].marks.clone();
    g.dr[id].frame_centre = [def.hinge[0] + m::cos(def.closed_dir) * def.width * 0.5, 1.0, def.hinge[1] + m::sin(def.closed_dir) * def.width * 0.5];
    door_place(g, id);
    id
}

/// Leaf centre (1 m up) at the current angle, made anew only when the angle has moved   (doors.ts Door.pos)
pub fn door_pos(g: &mut crate::game::Game, id: DoorId) -> V3 {
    if door_off(g, id) { return g.dr[id].frame_centre; }   // (off its hinges: 'the door' is its doorway)
    let a = door_angle(g, id);
    if a != col(g, id, ::sim::doors::POS_ANGLE) {
        set_col(g, id, ::sim::doors::POS_ANGLE, a);
        let u = door_dir(g, id);
        let (hinge, width) = (g.dr[id].def.hinge, g.dr[id].def.width);
        door_kept_pos(g, id, [hinge[0] + u[0] * width * 0.5, 1.0, hinge[1] + u[1] * width * 0.5]);
    }
    g.dr[id].pos_at
}
/// `p` is the kept centre from here on   (doors.ts Door.keptPos)
pub fn door_kept_pos(g: &mut crate::game::Game, id: DoorId, p: V3) {
    g.dr[id].pos_at = p;
    set_col(g, id, ::sim::doors::POS_X, p[0]);
    set_col(g, id, ::sim::doors::POS_Z, p[2]);
}
/// the kept centre as it stands, whatever angle the leaf has now   (doors.ts Door.keptAt)
pub fn door_kept_at(g: &mut crate::game::Game, id: DoorId) -> V3 { g.dr[id].pos_at }
/// `fn_` asked aside: what each of `doors` kept of its centre is put back after   (doors.ts Door.posAside)
pub fn door_pos_aside<T: Clone + core::fmt::Debug>(g: &mut crate::game::Game, doors: &mut Vec<DoorId>, fn_: &mut dyn FnMut(&mut crate::game::Game) -> T) -> T {
    let mut a = Vec::with_capacity(doors.len());
    let mut p = Vec::with_capacity(doors.len());
    for &d in doors.iter() { a.push(col(g, d, ::sim::doors::POS_ANGLE)); p.push(g.dr[d].pos_at); }
    let r = fn_(g);
    for i in 0..doors.len() { set_col(g, doors[i], ::sim::doors::POS_ANGLE, a[i]); door_kept_pos(g, doors[i], p[i]); }
    r
}
/// 'Closed enough' that using it means opening it   (doors.ts Door.isClosed)
pub fn door_is_closed(g: &mut crate::game::Game, id: DoorId) -> bool { m::abs(door_angle(g, id)) < 0.25 }
/// (doors.ts Door.dir)
pub fn door_dir(g: &mut crate::game::Game, id: DoorId) -> [f64; 2] {
    let a = g.dr[id].def.closed_dir + door_angle(g, id);
    [m::cos(a), m::sin(a)]
}
/// Back to the level as authored   (doors.ts Door.reset)
pub fn door_reset(g: &mut crate::game::Game, id: DoorId) {
    door_set_shot(g, id, 0.0);
    if door_off(g, id) {   // hung again, as authored
        door_set_off(g, id, false);
        door_set_slab(g, id, None);
        let (b, h, marks) = (g.dr[id].box_, g.dr[id].leaf_h, g.dr[id].leaf_marks.clone());
        g.col.boxes[b].h = h;
        g.col.boxes[b].marks = marks;
        g.col.boxes[b].rot = None;
        for s in 0..g.dr[id].handle_boxes.len() { let hb = g.dr[id].handle_boxes[s]; g.col.boxes[hb].rot = None; }
    }
    let authored = g.dr[id].def.angle.unwrap_or(0.0);
    let (sided, lo, hi) = (door_sided(g, id, authored), door_min_a(g, id), door_max_a(g, id));
    door_set_angle(g, id, v3::clamp(sided, lo, hi));
    door_set_vel(g, id, 0.0);
    let a = door_angle(g, id);
    door_set_latched(g, id, m::abs(a) < 1e-3);
    door_set_closing(g, id, false); door_set_swept(g, id, false); door_set_quiet_t(g, id, 10.0); door_set_since_bash(g, id, 10.0); door_set_creak_armed(g, id, true); door_set_last_who(g, id, -1.0);
    let locked = g.dr[id].def.locked == Some(true);
    door_set_locked(g, id, locked); door_set_lock_broken(g, id, false); door_set_noticed(g, id, false); door_set_noticed_t(g, id, -1000.0); door_set_pick(g, id, 0.0); door_set_picking(g, id, false);
    door_set_pick_idle(g, id, 0.0); door_set_pick_clock(g, id, 0.0); door_set_rattle_t(g, id, 0.0); door_set_guard_t(g, id, -100.0); door_set_passers(g, id, 0.0);
    door_set_pull_t(g, id, 0.0); door_set_press_t(g, id, 0.0); door_set_solid_t(g, id, 0.0); door_set_open_t(g, id, 10.0); door_set_drive(g, id, None);
    door_place(g, id);
}
/// Deal the side it opens into: a leaf authored standing open the other way is stood as far open this way   (doors.ts Door.setSwing)
pub fn door_set_swing(g: &mut crate::game::Game, id: DoorId, s: f64, outward: Option<f64>, clearway: Option<f64>) {
    door_set_dealt(g, id, s);
    door_set_outward(g, id, outward.unwrap_or(0.0));
    door_set_clearway(g, id, clearway.unwrap_or(0.0));
    door_keep(g, id);
    let a = door_angle(g, id);
    door_set_latched(g, id, m::abs(a) < 1e-3);
    door_place(g, id);
}
/// One-way: an angle on the side it does not open into is the same opening on the side it does, its velocity turned with it   (doors.ts Door.keep)
pub fn door_keep(g: &mut crate::game::Game, id: DoorId) {
    if door_angle(g, id) * door_swing(g, id) < 0.0 {
        let (a, lo, hi) = (door_angle(g, id), door_min_a(g, id), door_max_a(g, id));
        door_set_angle(g, id, v3::clamp(-a, lo, hi));
        let v = door_vel(g, id);
        door_set_vel(g, id, -v);
        let n = door_mirrored(g, id);
        door_set_mirrored(g, id, n + 1.0);
    }
}
/// an angle (or a turn rate) as this door can have it   (doors.ts Door.sided)
pub fn door_sided(g: &mut crate::game::Game, id: DoorId, a: f64) -> f64 {
    if a * door_swing(g, id) < 0.0 { -a } else { a }
}
/// the leaf's box and handles where its angle stands them   (doors.ts Door.place)
pub fn door_place(g: &mut crate::game::Game, id: DoorId) {
    if door_off(g, id) { return; }   // (the slab's box is doorblast.ts's to pose)
    door_keep(g, id);
    let (v, a) = (door_vel(g, id), door_angle(g, id));
    set_col(g, id, ::sim::doors::VEL_SEEN, v);
    set_col(g, id, ::sim::doors::ANGLE_SEEN, a);
    let rattle = door_rattle_t(g, id);
    let jig = if rattle > 0.35 { m::sin(rattle * 90.0) * 0.005 } else { 0.0 };   // the handle being tried: the leaf works a hair in its frame
    let p = door_pose_at(g, id, a + jig);
    let b = g.dr[id].box_;
    g.col.boxes[b].c = p.c;
    g.col.boxes[b].yaw = p.yaw;
    let (flags, seal) = (js::to_i32(g.col.boxes[b].flags), js::to_i32(crate::scene_boxes::box_flag().sound_seal));
    g.col.boxes[b].flags = (if m::abs(a) < 0.3 { flags | seal } else { flags & !seal }) as f64;   // an open leaf standing in the room does not seal the doorway acoustically
    for s in 0..2 {
        let hb = g.dr[id].handle_boxes[s];
        g.col.boxes[hb].c = p.handles[s];
        g.col.boxes[hb].yaw = p.yaw;
    }
    crate::scene_box_mirror::leaf_wrote(g);
}
/// Where the leaf's box and its two handles stand with the leaf at `angle`   (doors.ts Door.poseAt)
pub fn door_pose_at(g: &mut crate::game::Game, id: DoorId, angle: f64) -> crate::doors::DoorPoseAt {
    let d = &g.dr[id];
    pose_of(d.def.hinge, d.def.width, d.def.closed_dir, d.thick, d.height, angle)
}
fn pose_of(hinge: [f64; 2], w: f64, closed_dir: f64, thick: f64, height: f64, angle: f64) -> crate::doors::DoorPoseAt {
    let (hx, hz) = (hinge[0], hinge[1]);
    let a = closed_dir + angle;
    let u = [m::cos(a), m::sin(a)];
    let yaw = -a;   // box local +X maps to world (cos yaw, -sin yaw)
    let n = [-u[1], u[0]];
    let off = thick / 2.0 + 0.03;
    let handle = |side: f64| -> V3 { [hx + u[0] * (w - 0.14) + n[0] * off * side, 1.0, hz + u[1] * (w - 0.14) + n[1] * off * side] };
    crate::doors::DoorPoseAt {
        c: [hx + u[0] * (w * 0.5 + 0.01), height / 2.0 + 0.01, hz + u[1] * (w * 0.5 + 0.01)],
        yaw,
        handles: [handle(1.0), handle(-1.0)],
    }
}

/// the level's doors: their columns a part of the game's block `st`. `emit` is not kept: the word is `Doors.sound`'s to say, by name   (doors.ts Doors.constructor)
pub fn doors_new(g: &mut crate::game::Game, defs: &[crate::doors::DoorDef], emit: &mut dyn FnMut(&mut crate::game::Game, crate::doors::DoorEvent), st: Option<crate::Opaque>) -> crate::doors::Doors {
    let home = st.unwrap_or(crate::Opaque);
    let store = crate::sim_doorshome::door_store_new(g, home, defs.len() as f64);
    let slabs = crate::sim_slabshome::slab_store_new(g, home, defs.len() as f64);
    let mut list = Vec::with_capacity(defs.len());
    for (i, d) in defs.iter().enumerate() {
        list.push(door_new(g, d, Some(i as f64)));
    }
    let key = crate::scene_boxes::mkey().door;
    for (i, &d) in list.iter().enumerate() {   // motion identity for the leaf and its handles (scene/boxes.ts MKEY)
        let (b, h0, h1) = (g.dr[d].box_, g.dr[d].handle_boxes[0], g.dr[d].handle_boxes[1]);
        g.col.boxes[b].mkey = Some(key + i as f64 * 4.0);
        g.col.boxes[h0].mkey = Some(key + i as f64 * 4.0 + 1.0);
        g.col.boxes[h1].mkey = Some(key + i as f64 * 4.0 + 2.0);
    }
    crate::doors::Doors { list, store, slabs, ..crate::doors::Doors::blank() }
}
/// (doors.ts Doors.byName)
pub fn doors_by_name(g: &mut crate::game::Game, n: &str) -> Option<DoorId> {
    for i in 0..g.doors.list.len() {
        let d = g.doors.list[i];
        if g.dr[d].def.name == n { return Some(d); }
    }
    None
}
/// Is `from` on d's PULL face: the side it opens into   (doors.ts Doors.pullFace)
pub fn doors_pull_face(g: &mut crate::game::Game, d: DoorId, from: V3) -> bool {
    let (c, hinge) = (g.dr[d].def.closed_dir, g.dr[d].def.hinge);
    ((from[0] - hinge[0]) * -m::sin(c) + (from[2] - hinge[1]) * m::cos(c)) * door_swing(g, d) >= 0.0
}
/// DOORWAY.frameHolds as it applies right now: no showcase on the stage   (doors.ts Doors.frameRule)
pub fn doors_frame_rule(g: &mut crate::game::Game) -> bool { !doors_staged(g) }
/// WOULD THE FRAME HOLD A BOOT thrown at d from `from`   (doors.ts Doors.bootHolds)
pub fn doors_boot_holds(g: &mut crate::game::Game, d: DoorId, from: V3) -> bool {
    doors_frame_rule(g) && !door_off(g, d) && g.dr[d].def.steel != Some(true) && m::abs(door_angle(g, d)) < SHUT_HOME && doors_pull_face(g, d, from)
}
/// THE HOLD TEST for a wedge or dragged furniture put against `d` by a man at `from`   (doors.ts Doors.wedgeHolds)
pub fn doors_wedge_holds(g: &mut crate::game::Game, d: DoorId, from: V3) -> bool {
    !doors_frame_rule(g) || doors_pull_face(g, d, from)
}
/// THE DOOR AS A HINGED THING: its jambs, the side it opens into, the floor its leaf sweeps   (doors.ts Doors.swingOf)
pub fn doors_swing_of(g: &mut crate::game::Game, d: DoorId) -> crate::doors::DoorSwing {
    let (hx, hz, w, c) = (g.dr[d].def.hinge[0], g.dr[d].def.hinge[1], g.dr[d].def.width, g.dr[d].def.closed_dir);
    let s = door_swing(g, d);
    let (ux, uz) = (m::cos(c), m::sin(c));
    let open = m::abs(if s > 0.0 { g.dr[d].max_auth } else { g.dr[d].min_auth });
    crate::doors::DoorSwing { hinge: [hx, 0.0, hz], handle: [hx + ux * w, 0.0, hz + uz * w], swing: s, into: [-uz * s, ux * s], open, radius: w, closed_dir: c, one_way: true }
}
/// how far (x, z) is from the leaf standing at world angle `a`   (doors.ts Doors.swingOf seg)
fn swing_seg(s: &crate::doors::DoorSwing, x: f64, z: f64, a: f64) -> f64 {
    let (hx, hz) = (s.hinge[0], s.hinge[2]);
    let (vx, vz) = (m::cos(a), m::sin(a));
    let t = v3::clamp((x - hx) * vx + (z - hz) * vz, 0.0, s.radius);
    m::hyp2(x - hx - vx * t, z - hz - vz * t)
}
/// is the floor point inside the area the leaf sweeps, with `pad` m of slack. The closure of the TypeScript's DoorSwing; the skeleton's has no place for it yet (notes)   (doors.ts DoorSwing.sweeps)
pub fn swing_sweeps(s: &crate::doors::DoorSwing, x: f64, z: f64, pad: f64) -> bool {
    let (px, pz) = (x - s.hinge[0], z - s.hinge[2]);
    let r = m::hyp2(px, pz);
    if r > s.radius + pad { return false; }
    let (c, sw, open) = (s.closed_dir, s.swing, s.open);
    let mut th = (m::atan2(pz, px) - c) * sw;
    th -= m::round(th / (2.0 * m::PI)) * 2.0 * m::PI;
    if th < 0.0 && open > m::PI && th + 2.0 * m::PI <= open { th += 2.0 * m::PI; }   // the point's bearing from the closed leaf, the way it opens
    (th >= 0.0 && th <= open) || swing_seg(s, x, z, c) <= pad || swing_seg(s, x, z, c + sw * open) <= pad
}
/// a showcase / the tour / the attract reel has the stage (game.ts:1329)   (doors.ts Doors.staged)
pub fn doors_staged(g: &mut crate::game::Game) -> bool { g.touring }
/// DOORSTAY as it applies right now: leaves stay where they are left   (doors.ts Doors.stay)
pub fn doors_stay(g: &mut crate::game::Game) -> bool { !doors_staged(g) }
/// does this leaf's hydraulic closer work   (doors.ts Doors.hasCloser)
pub fn doors_has_closer(g: &mut crate::game::Game, d: DoorId) -> bool { !doors_stay(g) && g.dr[d].def.closer != Some(false) }
/// Which face of d the point is on: +1 the CCW side of the leaf, -1 the other   (doors.ts Doors.side)
pub fn doors_side(g: &mut crate::game::Game, d: DoorId, from: V3) -> f64 {
    let u = door_dir(g, d);
    let (px, pz) = (from[0] - g.dr[d].def.hinge[0], from[2] - g.dr[d].def.hinge[1]);
    if u[0] * pz - u[1] * px >= 0.0 { 1.0 } else { -1.0 }
}
/// The lock is against you: latched, still locked, you on the keyed side, and nothing on your ring fits it   (doors.ts Doors.lockedOut)
pub fn doors_locked_out(g: &mut crate::game::Game, d: DoorId, from: V3, keys: Option<&JsSet<String>>) -> bool {
    door_locked(g, d) && door_latched(g, d) && doors_side(g, d, from) == g.dr[d].key_side && !(match keys { Some(k) => k.has(g.dr[d].lock_id.clone()), None => false })
}
/// The lock is against you BUT for the key you carry   (doors.ts Doors.keyOpens)
pub fn doors_key_opens(g: &mut crate::game::Game, d: DoorId, from: V3, keys: Option<&JsSet<String>>) -> bool {
    (match keys { Some(k) => k.has(g.dr[d].lock_id.clone()), None => false }) && doors_locked_out(g, d, from, None)
}
/// DOORTHROW as it applies right now: no showcase on the stage   (doors.ts Doors.throws)
pub fn doors_throws(g: &mut crate::game::Game) -> bool { !doors_staged(g) }
/// would a turn of sign `dir` take d further OPEN (one-way: the way it opens)   (doors.ts Doors.opening)
pub fn doors_opening(g: &mut crate::game::Game, d: DoorId, dir: f64) -> bool { dir == door_swing(g, d) }
/// DOORTHROW: somebody drove d open   (doors.ts Doors.driven)
pub fn doors_driven(g: &mut crate::game::Game, d: DoorId, who: f64, dir: f64, kind: crate::doors::DoorsDrivenKind, quiet: bool, rate: f64) {
    let dr = door_drive(g, d);
    let s = if dir >= 0.0 { 1.0 } else { -1.0 };
    let man = crate::doorthrow::door_man(g, who);
    let man_bits = js::to_i32(man);
    let side = (man_bits & js::to_i32(MAN_SIDE)) as f64;
    let mut quiet = quiet;
    if kind != crate::doors::DoorsDrivenKind::Boot && (man_bits & js::to_i32(MAN_CALM)) != 0 { quiet = true; }
    let (body_or_hand, kind_name) = (matches!(kind, crate::doors::DoorsDrivenKind::Body | crate::doors::DoorsDrivenKind::Hand), kind.as_str());
    if let Some(dd) = dr {
        if dd.who == who && dd.dir == s && dd.kind.as_str() == kind_name {
            if !body_or_hand || dd.t >= DRIVEN_SECS { set_col(g, d, ::sim::doors::DRIVE_HIT, -1.0); }
            set_col(g, d, ::sim::doors::DRIVE_T, 0.0);
            set_flag(g, d, ::sim::doors::DRIVE_QUIET, quiet);
            set_col(g, d, ::sim::doors::DRIVE_RATE, rate);
            set_col(g, d, ::sim::doors::DRIVE_SIDE, side);
            if !body_or_hand { let a = door_angle(g, d); set_col(g, d, ::sim::doors::DRIVE_FROM, a); }   // (a walk or a hand goes on: one drive, from where it began)
            return;
        }
    }
    let kind = match kind {
        crate::doors::DoorsDrivenKind::Body => crate::doors::DoorDriveKind::Body,
        crate::doors::DoorsDrivenKind::Hand => crate::doors::DoorDriveKind::Hand,
        crate::doors::DoorsDrivenKind::Boot => crate::doors::DoorDriveKind::Boot,
        crate::doors::DoorsDrivenKind::Swing => crate::doors::DoorDriveKind::Swing,
    };
    let from = door_angle(g, d);
    door_set_drive(g, d, Some(crate::doors::DoorDrive { who, dir: s, kind, t: 0.0, from, quiet, side, rate, px: m::NAN, pz: m::NAN, hit: -1.0 }));
}

/// Resolve contacts, integrate swing, closer, latch: the step in rust/sim, with its hand-over and what it did taken back   (doors.ts Doors.update)
pub fn doors_update(g: &mut crate::game::Game, dt: f64, contacts: &mut Vec<crate::doors::DoorContact>) {
    doors_there(g, contacts);
    for c in contacts.iter_mut() {
        c.blocked_by = None;
        if c.run.is_some() { c.bashed = None; c.ripped = None; }
        c.struck = None;
        if c.moved == Some(true) { c.moved = Some(false); }
        c.man = None;
    }
    if g.doors.list.is_empty() { g.m.sim_doorshome.doors_stepped.here += 1.0; return; }   // (a level with no door has no step, only that reset)
    let slot = g.col.mirror.slot;
    let n = doors_hand_over(g, dt, contacts, slot);
    doors_take_back(g, contacts, n);
}
/// the world the step stands in made ready, with the game's own leaves in list 0: nothing to do with no door in the level. In Rust the step is always rust/sim's, so what was a way back to the TypeScript's own step is an assert   (doors.ts Doors.there)
pub fn doors_there(g: &mut crate::game::Game, contacts: &[crate::doors::DoorContact]) {
    if g.doors.list.is_empty() { return; }
    debug_assert!(contacts.len() <= ::sim::doors::MAXC, "{} contacts at the doors in one step: its hand-over holds {}, and a game holds no more men than that many are of (game/levelfit.ts handOverMost)", contacts.len(), ::sim::doors::MAXC);
    let dyn_ = g.col.dynamic_boxes.clone();
    debug_assert!(dyn_.len() == g.doors.list.len() && g.doors.list.iter().zip(dyn_.iter()).all(|(&d, &b)| g.dr[d].box_ == b), "the leaves in the box world are not this level's doors");
    crate::scene_collision::rules_world(g, &dyn_);
}
/// the hand-over's tables in rust/sim (its `def` is the first field of `Io`, `repr(C)`): it keeps the function that lends them private, and hands out their address
fn step_io() -> &'static mut ::sim::doors::Io {
    unsafe { &mut *(::sim::doors::addr(0) as *mut ::sim::doors::Io) }
}
/// the step's hand-over written (the level's doors, the contacts, the rules' numbers) and the step taken: how many noises it listed   (doors.ts Doors.handOver)
pub fn doors_hand_over(g: &mut crate::game::Game, dt: f64, contacts: &[crate::doors::DoorContact], slot: f64) -> f64 {
    let io = step_io();
    let (n, nc) = (g.doors.list.len(), contacts.len());
    let thr = doors_throws(g);
    for i in 0..n {
        let d = g.doors.list[i];
        let slab = door_slab(g, d);
        {
            let dd = &g.dr[d];
            let f = &dd.def;
            io.def[::sim::doors::D_HX][i] = f.hinge[0];
            io.def[::sim::doors::D_HZ][i] = f.hinge[1];
            io.def[::sim::doors::D_CLOSED][i] = f.closed_dir;
            io.def[::sim::doors::D_WIDTH][i] = f.width;
            io.def[::sim::doors::D_HEIGHT][i] = dd.height;
            io.def[::sim::doors::D_THICK][i] = dd.thick;
            io.def[::sim::doors::D_MIN_AUTH][i] = dd.min_auth;
            io.def[::sim::doors::D_MAX_AUTH][i] = dd.max_auth;
            io.def[::sim::doors::D_KEY_SIDE][i] = dd.key_side;
            io.def[::sim::doors::D_CLOSER][i] = if f.closer != Some(false) { 1.0 } else { 0.0 };
            io.def[::sim::doors::D_SLAB][i] = if slab.is_some() { 1.0 } else { 0.0 };
            io.def[::sim::doors::D_SLAB_ANG][i] = match &slab { Some(s) => s.ang, None => 0.0 };
            io.def[::sim::doors::D_FCX][i] = dd.frame_centre[0];
            io.def[::sim::doors::D_FCZ][i] = dd.frame_centre[2];
        }
        // (Doors.man of the hand last on it: asked of the game only where a rule of this step can want it: something was written onto the leaf since the last one)
        let man_last = if !thr {
            0.0
        } else if door_vel(g, d) != door_seen_vel(g, d) || door_angle(g, d) != col(g, d, ::sim::doors::ANGLE_SEEN) || door_angle(g, d) * door_swing(g, d) < 0.0 {
            let who = door_last_who(g, d);
            crate::doorthrow::door_man(g, who)
        } else {
            crate::sim_doorshome::MAN_UNASKED
        };
        io.def[::sim::doors::D_MAN_LAST][i] = man_last;
    }
    for c in 0..nc {
        let k = &contacts[c];
        let (weak, p) = (k.weak == Some(true), k.pos);
        io.con[::sim::doors::K_X][c] = p[0];
        io.con[::sim::doors::K_Y][c] = p[1];
        io.con[::sim::doors::K_Z][c] = p[2];
        io.con[::sim::doors::K_RADIUS][c] = k.radius;
        io.con[::sim::doors::K_WHO][c] = k.who;
        let flags = (if k.bash { ::sim::doors::KF_BASH } else { 0 })
            | (if k.quiet { ::sim::doors::KF_QUIET } else { 0 })
            | (if weak { ::sim::doors::KF_WEAK } else { 0 })
            | (if k.key == Some(true) { ::sim::doors::KF_KEY } else { 0 })
            | (if k.solid == Some(true) { ::sim::doors::KF_SOLID } else { 0 })
            | (if k.round == Some(true) { ::sim::doors::KF_ROUND } else { 0 })
            | (if k.vel.is_some() { ::sim::doors::KF_VEL } else { 0 })
            | (if k.run.is_some() { ::sim::doors::KF_RUN } else { 0 })
            | (if k.hand.is_some() { ::sim::doors::KF_HAND } else { 0 });
        io.con[::sim::doors::K_FLAGS][c] = flags as f64;
        io.con[::sim::doors::K_HAND][c] = k.hand.unwrap_or(0.0);
        let (run_bash, run_rip, run_level) = match &k.run { Some(r) => (r.bash, r.rip, r.rip_level), None => (0.0, 0.0, 0.0) };
        io.con[::sim::doors::K_RUN_BASH][c] = run_bash;
        io.con[::sim::doors::K_RUN_RIP][c] = run_rip;
        io.con[::sim::doors::K_RUN_LEVEL][c] = run_level;
        io.con[::sim::doors::K_VX][c] = match &k.vel { Some(v) => v[0], None => 0.0 };
        io.con[::sim::doors::K_VZ][c] = match &k.vel { Some(v) => v[2], None => 0.0 };
        io.con[::sim::doors::K_MAN][c] = if thr && !weak { crate::doorthrow::door_man(g, k.who) } else { 0.0 };
        let (mut lo, mut hi) = (0u32, 0u32);
        if let Some(keys) = &k.keys {
            if keys.len() > 0 {
                for i in 0..n {
                    let d = g.doors.list[i];
                    if keys.has(g.dr[d].lock_id.clone()) {
                        if i < 32 { lo |= 1u32 << i; } else { hi |= 1u32 << (i - 32); }
                    }
                }
            }
        }
        io.con[::sim::doors::K_OPENS][c] = lo as f64;
        io.con[::sim::doors::K_OPENS_HI][c] = hi as f64;
        io.con[::sim::doors::K_BLOCKED_BY][c] = -1.0;
        io.con[::sim::doors::K_BASHED][c] = -1.0;
        io.con[::sim::doors::K_RIPPED][c] = -1.0;
        io.con[::sim::doors::K_STRUCK][c] = -1.0;
        io.con[::sim::doors::K_MOVED][c] = 0.0;
        io.con[::sim::doors::K_MAN_ASKED][c] = 0.0;
    }
    io.par[::sim::doors::P_DT] = dt;
    io.par[::sim::doors::P_DOORS] = n as f64;
    io.par[::sim::doors::P_CONTACTS] = nc as f64;
    // (the page's DOORWAY: one-way, exits as dealt, leafWay on; DOORTHROW.soft on. A stepping game always has its floor questions: they are the module's own)
    let flags = ::sim::doors::F_ONE_WAY | ::sim::doors::F_LEAF_WAY
        | (if doors_stay(g) { ::sim::doors::F_STAY } else { 0 })
        | (if thr { ::sim::doors::F_THROWS } else { 0 })
        | ::sim::doors::F_SOFT | ::sim::doors::F_STANDABLE | ::sim::doors::F_SETTLE;
    io.par[::sim::doors::P_FLAGS] = flags as f64;
    io.par[::sim::doors::P_HAND_RATE] = g.k.doorthrow.hand_rate;
    io.par[::sim::doors::P_MIN_SPEED] = g.k.doorthrow.min_speed;
    io.par[::sim::doors::P_FLOOR_RAD] = g.k.doorthrow.floor_rad;
    io.par[::sim::doors::P_RATE] = g.k.doorthrow.rate;
    io.par[::sim::doors::P_SLIDE] = g.k.doorthrow.slide;
    io.par[::sim::doors::P_AGAIN] = g.k.doorthrow.again;
    let world = crate::scene_world::world();
    io.par[::sim::doors::P_WMIN_X] = world.min[0];
    io.par[::sim::doors::P_WMAX_X] = world.max[0];
    io.par[::sim::doors::P_WMIN_Z] = world.min[2];
    io.par[::sim::doors::P_WMAX_Z] = world.max[2];
    io.par[::sim::doors::P_FOOT] = crate::scene_collision::foot_of(g, FLOOR_Y0);
    let first = g.seats[0];   // (game.ts:1334 settleY1: the head of the span the settled man is pushed out of the statics over, of the first seat's man)
    let man = crate::cmd::seat_man(g, first);
    io.par[::sim::doors::P_SETTLE_Y1] = if g.pl[man].crouch { 1.0 } else { 1.7 };
    let ev = ::sim::doors::update(g.sim, slot as usize);
    let wrong = step_io().par[::sim::doors::P_WRONG];
    if js::truthy(wrong) {
        panic!("the doors' step in the Rust module could not be what it was handed ({})", if wrong == 1.0 { "more noises than its list holds" } else if wrong == 2.0 { "the man of a door's last hand was not asked of the game" } else { "more doors or contacts than its tables hold" });
    }
    ev as f64
}

#[cfg(test)]
mod tests {
    extern crate std;
    use super::*;
    fn bits(x: &[f64]) -> Vec<u64> { x.iter().map(|v| v.to_bits()).collect() }

    /// the bits are the TypeScript's own: new Door({ hinge: [1.25, -3.5], closedDir: 0.7, width: 0.93, thickness: 0.06, height: 2.05 }).poseAt(0.37), EXACT on
    #[test]
    fn the_pose_is_the_typescripts_to_the_bit() {
        let p = pose_of([1.25, -3.5], 0.93, 0.7, 0.06, 2.05, 0.37);
        let got = [p.c[0], p.c[1], p.c[2], p.yaw, p.handles[0][0], p.handles[0][1], p.handles[0][2], p.handles[1][0], p.handles[1][1], p.handles[1][2]];
        assert_eq!(bits(&got), [0x3ff7a6213404e158, 0x3ff08f5c28f5c28f, 0xc008aaa8cb200044, 0xbff11eb851eb851e, 0x3ff93a063e610b0e, 0x3ff0000000000000, 0xc00639c318a617ee, 0x3ffae92f9c7699be, 0x3ff0000000000000, 0xc006afc1e6a1520a]);
    }

    /// the answers are the TypeScript's own: Doors.swingOf of that door with swing -1, sweeps(x, z, pad)
    #[test]
    fn the_sweep_answers_as_the_typescript_does() {
        let s = crate::doors::DoorSwing { hinge: [1.25, 0.0, -3.5], handle: [0.0; 3], swing: -1.0, into: [0.0; 2], open: 1.92, radius: 0.93, closed_dir: 0.7, one_way: true };
        for (x, z, pad, want) in [(1.761, -3.331, 0.0, true), (1.6, -3.2, 0.0, false), (1.25, -2.3, 0.2, false), (1.25, -4.2, 0.1, false), (2.0, -2.8, 0.05, false), (2.4, -3.1, 0.1, false), (0.8, -3.9, 0.05, false), (1.3, -3.9, 0.2, true)] {
            assert_eq!(swing_sweeps(&s, x, z, pad), want, "{x} {z} {pad}");
        }
    }

    #[test]
    fn a_drive_kind_is_its_place_in_drive_kinds() {
        for (n, k) in [(0.0, crate::doors::DoorDriveKind::Boot), (1.0, crate::doors::DoorDriveKind::Swing), (2.0, crate::doors::DoorDriveKind::Body), (3.0, crate::doors::DoorDriveKind::Hand)] {
            assert_eq!(drive_kind_of(n), k);
            assert_eq!(drive_kind_num(k), n);
        }
    }

    /// a level with no door has no step: only the reset of what a leaf did to each contact   (doors.ts:640, :761)
    #[test]
    fn with_no_door_the_contacts_are_only_reset() {
        std::thread::Builder::new().stack_size(1 << 30).spawn(|| {
            let mut g = alloc::boxed::Box::new(Game::blank());
            let d = DoorId(3);
            let hit = crate::doors::DoorStrike { door: d, ..crate::doors::DoorStrike::zeroed() };
            let run = crate::doors::DoorContactRun { bash: 1.0, rip: 2.0, rip_level: 3.0 };
            let full = crate::doors::DoorContact { blocked_by: Some(d), bashed: Some(d), ripped: Some(d), bashed_at: Some(0.5), struck: Some(hit), moved: Some(true), man: Some(7.0), ..crate::doors::DoorContact::zeroed() };
            let mut v = vec![crate::doors::DoorContact { run: Some(run), ..full.clone() }, full];
            doors_update(&mut g, 0.016, &mut v);
            let (a, b) = (&v[0], &v[1]);
            assert!(a.blocked_by.is_none() && a.bashed.is_none() && a.ripped.is_none() && a.struck.is_none() && a.man.is_none());
            assert_eq!(a.moved, Some(false));
            assert_eq!(a.bashed_at, Some(0.5));   // (the TypeScript leaves it)
            assert!(b.blocked_by.is_none() && b.bashed.is_some() && b.ripped.is_some());   // (no run: not his to clear)
            assert_eq!(g.m.sim_doorshome.doors_stepped.here, 1.0);
        }).unwrap().join().unwrap();
    }
}
