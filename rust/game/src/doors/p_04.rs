//! src/game/doors.ts lines 1040 to 1192
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// seconds of cylinder work to pick a lock (doors.ts Doors.PICK_SECS)
const PICK_SECS: f64 = 4.2;
/// hearing levels of a pick's click and of the cylinder going over (doors.ts Doors.PICK_LEVEL, UNLOCK_LEVEL)
const PICK_LEVEL: f64 = 0.16;
const UNLOCK_LEVEL: f64 = 0.3;

/// A door's sound, heard and played: what the Game hands the doors as `emit`, called by name (game.ts:1325).
fn emit(g: &mut Game, d: DoorId, sound: crate::doors::DoorSound, pos: V3, who: f64, level: f64) {
    crate::game::game_on_door_event(g, crate::doors::DoorEvent { door: d, sound, pos, who, level });
}

/// How far a leaf stands open the way a shove would send it; one-way, simply how far open.   (doors.ts Doors.openFor)
pub fn doors_open_for(g: &mut crate::game::Game, d: DoorId, s: f64) -> f64 {
    m::abs(door_angle(g, d))
}

/// Pull a leaf to from wherever it stands: a stiff sprung close the latch catches.   (doors.ts Doors.pullTo)
pub fn doors_pull_to(g: &mut crate::game::Game, d: DoorId, who: f64) {
    if door_latched(g, d) || door_off(g, d) { return; }
    door_set_closing(g, d, true);
    door_set_swept(g, d, false);
    let vel = -m::sign(door_angle(g, d)) * 1.2;
    door_set_vel(g, d, vel);
    door_set_last_who(g, d, who);
}

/// A hand on the handle from the face it opens into: unlatched with its creak, brought round at the pull's pace.   (doors.ts Doors.pull)
pub fn doors_pull(g: &mut crate::game::Game, d: DoorId, who: f64, quiet: Option<bool>) {
    let quiet = quiet.unwrap_or(false);
    if door_off(g, d) { return; }
    if door_latched(g, d) || door_rests_to(g, d) {
        door_set_latched(g, d, false);
        if door_creak_armed(g, d) {
            let pos = door_pos(g, d);
            emit(g, d, crate::doors::DoorSound::Creak, pos, who, if quiet { 0.12 } else { 0.3 });
            door_set_creak_armed(g, d, false);
        }
    }
    door_set_closing(g, d, false);
    door_set_swept(g, d, false);
    let swing = door_swing(g, d);
    let vel = swing * m::max(door_vel(g, d) * door_swing(g, d), if quiet { PULL_VEL_QUIET } else { PULL_VEL });
    door_set_vel(g, d, vel);
    door_set_pull_t(g, d, PULL_SECS);
    let quiet_t = m::min(door_quiet_t(g, d), -1.0);
    door_set_quiet_t(g, d, quiet_t);
    door_set_last_who(g, d, who);
}

/// Try the lever against the bolt: a clack, and the marker's flash, which is also the rate limit.   (doors.ts Doors.rattle)
pub fn doors_rattle(g: &mut crate::game::Game, d: DoorId, who: f64, level: Option<f64>) {
    let level = level.unwrap_or(0.12);
    if door_rattle_t(g, d) > 0.0 || door_off(g, d) { return; }
    door_set_rattle_t(g, d, 0.6);
    door_set_last_who(g, d, who);
    let pos = door_pos(g, d);
    emit(g, d, crate::doors::DoorSound::Rattle, pos, who, level);
}

/// The root `stand` off the leaf, `along` from the hinge, on the side `s`, and the yaw back into the leaf.   (doors.ts workSpot's sums)
fn work_spot_at(hinge: [f64; 2], u: [f64; 2], s: f64, stand: f64, along: f64) -> (V3, f64) {
    let [hx, hz] = hinge;
    let (nx, nz) = (-u[1] * s, u[0] * s);
    ([hx + u[0] * along + nx * stand, 0.0, hz + u[1] * along + nz * stand], m::atan2(-nx, -nz))
}

/// Where to work d's lock from `from`'s side.   (doors.ts Doors.workSpot)
pub fn doors_work_spot(g: &mut crate::game::Game, d: DoorId, from: V3, stand: Option<f64>, along: Option<f64>) -> crate::doors::DoorsWorkSpot {
    let stand = stand.unwrap_or(0.5);
    let along = along.unwrap_or(g.dr[d].def.width - 0.14);
    let hinge = g.dr[d].def.hinge;
    let u = door_dir(g, d);
    let s = doors_side(g, d, from);
    let (pos, yaw) = work_spot_at(hinge, u, s, stand, along);
    crate::doors::DoorsWorkSpot { pos, yaw }
}

/// A kick's station along the leaf: the runner's own, kept on the leaf's outer half.   (doors.ts kickSpot's sums)
fn kick_along(hinge: [f64; 2], u: [f64; 2], width: f64, from: V3) -> f64 {
    let along = (from[0] - hinge[0]) * u[0] + (from[2] - hinge[1]) * u[1];
    v3::clamp(along, width * 0.5, width - 0.18)
}

/// Where a kick is thrown from: square to the leaf, by the lock.   (doors.ts Doors.kickSpot)
pub fn doors_kick_spot(g: &mut crate::game::Game, d: DoorId, from: V3) -> crate::doors::DoorsKickSpot {
    let u = door_dir(g, d);
    let (hinge, width) = (g.dr[d].def.hinge, g.dr[d].def.width);
    let spot = doors_work_spot(g, d, from, Some(0.76), Some(kick_along(hinge, u, width, from)));
    crate::doors::DoorsKickSpot { pos: spot.pos, yaw: spot.yaw }
}

/// The kick from beside the door's sums: past the latch edge, behind the wall, turned to the lock.   (doors.ts kickSpotSide's sums)
fn side_spot_at(hinge: [f64; 2], u: [f64; 2], s: f64, w: f64, past: f64, out: f64) -> (V3, f64) {
    let [hx, hz] = hinge;
    let (nx, nz) = (-u[1] * s, u[0] * s);
    let pos: V3 = [hx + u[0] * (w + past) + nx * out, 0.0, hz + u[1] * (w + past) + nz * out];
    let (lx, lz) = (hx + u[0] * (w - 0.2), hz + u[1] * (w - 0.2));
    (pos, m::atan2(lx - pos[0], lz - pos[2]))
}

/// The same kick thrown from beside the door. Geometry only: whether there is floor there is the caller's to ask.   (doors.ts Doors.kickSpotSide)
pub fn doors_kick_spot_side(g: &mut crate::game::Game, d: DoorId, from: V3) -> crate::doors::DoorsKickSpotSide {
    let hinge = g.dr[d].def.hinge;
    let u = door_dir(g, d);
    let s = doors_side(g, d, from);
    let w = g.dr[d].def.width;
    let k = side_kick();
    let (pos, yaw) = side_spot_at(hinge, u, s, w, k.past, k.out);
    crate::doors::DoorsKickSpotSide { pos, yaw }
}

/// How far along the frame line from the hinge, and how far off it, a point stands.   (doors.ts Doors.frameCoords)
pub fn doors_frame_coords(g: &mut crate::game::Game, d: DoorId, p: V3) -> crate::doors::DoorsFrameCoords {
    let u = door_dir(g, d);
    let hinge = g.dr[d].def.hinge;
    let (dx, dz) = (p[0] - hinge[0], p[2] - hinge[1]);
    crate::doors::DoorsFrameCoords { along: dx * u[0] + dz * u[1], off: m::abs(-dx * u[1] + dz * u[0]) }
}

/// Seconds to the next pick click after click number `n`: an uneven rhythm of about 0.7 s.   (doors.ts Doors.pick)
fn pick_gap(n: f64) -> f64 {
    0.55 + 0.3 * js::rem(n * 0.618034, 1.0)
}

/// One frame of working the lock. True on the frame it opens, and for a lock already open.   (doors.ts Doors.pick)
pub fn doors_pick(g: &mut crate::game::Game, d: DoorId, who: f64, dt: f64) -> bool {
    if !door_locked(g, d) { return true; }
    if g.dr[d].def.steel.unwrap_or(false) { return false; }
    if door_pick(g, d) == 0.0 {
        door_set_pick_clock(g, d, 0.3);
    } else if door_pick_idle(g, d) > 0.15 {
        let clock = m::min(door_pick_clock(g, d), 0.3);
        door_set_pick_clock(g, d, clock);
    }
    door_set_picking(g, d, true);
    door_set_last_who(g, d, who);
    door_set_quiet_t(g, d, 0.0);
    let pick = m::min(1.0, door_pick(g, d) + dt / PICK_SECS);
    door_set_pick(g, d, pick);
    let clock = door_pick_clock(g, d) - dt;
    door_set_pick_clock(g, d, clock);
    if door_pick_clock(g, d) <= 0.0 {
        let n = door_pick_n(g, d) + 1.0;
        door_set_pick_n(g, d, n);
        door_set_pick_clock(g, d, pick_gap(n));
        let pos = door_pos(g, d);
        emit(g, d, crate::doors::DoorSound::Pick, pos, who, PICK_LEVEL);
    }
    if door_pick(g, d) < 1.0 { return false; }
    door_set_locked(g, d, false);
    door_set_pick(g, d, 0.0);
    door_set_picking(g, d, false);
    door_set_noticed(g, d, false);
    let pos = door_pos(g, d);
    emit(g, d, crate::doors::DoorSound::Unlock, pos, who, UNLOCK_LEVEL);
    true
}

/// A boot beside the lock, or (`lock`) what destroys the lock itself. Returns whether the door gave.   (doors.ts Doors.kickIn)
pub fn doors_kick_in(g: &mut crate::game::Game, d: DoorId, from: V3, who: f64, lock: Option<bool>) -> bool {
    let lock = lock.unwrap_or(false);
    if door_off(g, d) { return false; }
    if g.dr[d].def.steel.unwrap_or(false) {
        door_set_rattle_t(g, d, 0.6);
        door_set_last_who(g, d, who);
        let pos = door_pos(g, d);
        emit(g, d, crate::doors::DoorSound::Clang, pos, who, 1.0);
        return false;
    }
    let held = doors_boot_holds(g, d, from);
    if held && !lock {
        door_set_rattle_t(g, d, 0.6);
        door_set_last_who(g, d, who);
        let pos = door_pos(g, d);
        emit(g, d, crate::doors::DoorSound::Thud, pos, who, 1.0);
        return false;
    }
    if !door_lock_broken(g, d) { door_set_noticed(g, d, false); }
    door_set_locked(g, d, false);
    door_set_lock_broken(g, d, true);
    door_set_pick(g, d, 0.0);
    door_set_latched(g, d, false);
    door_set_closing(g, d, false);
    door_set_swept(g, d, false);
    door_set_quiet_t(g, d, -0.5);
    door_set_last_who(g, d, who);
    door_set_creak_armed(g, d, false);
    if held {
        // the lock destroyed from the face it opens toward: nothing drove the leaf
    } else if m::abs(door_angle(g, d)) < 1.4 {
        let vel = -doors_side(g, d, from) * 9.0;
        door_set_vel(g, d, vel);
        door_set_since_bash(g, d, 0.0);
        if doors_throws(g) && { let dir = m::sign(door_vel(g, d)); doors_opening(g, d, dir) } {
            let vel = door_vel(g, d);
            doors_driven(g, d, who, vel, crate::doors::DoorsDrivenKind::Boot, false, 0.0);
        }
    }
    let pos = door_pos(g, d);
    emit(g, d, crate::doors::DoorSound::Kick, pos, who, 1.0);
    true
}

/// The doors' side of a blast: the leaves it took off. The Game's `blaster`, called by name (game.ts:1335).   (doors.ts Doors.blast)
pub fn doors_blast(g: &mut crate::game::Game, at: V3, tear_m: f64, who: f64) -> Vec<DoorId> {
    crate::doorblast::door_blast(g, at, tear_m, who)
}

/// A noise of d's that is not a leaf's own doing, heard and played as any other.   (doors.ts Doors.sound)
pub fn doors_sound(g: &mut crate::game::Game, d: DoorId, sound: crate::doors::DoorSound, pos: V3, who: f64, level: f64) {
    emit(g, d, sound, pos, who, level);
}

/// Keeps nothing: the one subscriber there is (game.ts:1337) is called by name from `doors_torn`.   (doors.ts Doors.onTorn)
pub fn doors_on_torn(g: &mut crate::game::Game, fn_: &mut dyn FnMut(&mut crate::game::Game, DoorId, f64, crate::doors::DoorsOnTornFnCause)) -> crate::FnField {
    crate::FnField
}

/// The subscriber game.ts:1337 hangs on the doors: a torn leaf is a row in the destruction's change feed.
fn torn_feed(g: &mut Game, d: DoorId, who: f64, cause: crate::doors::DoorsTornCause) {
    let c = g.dr[d].frame_centre;
    let r = g.dr[d].def.width / 2.0 + 0.1;
    let row = crate::wallbreak::WallBreakNoteC {
        kind: crate::wallbreak::LevelChangeKind::Door,
        cause: match cause {
            crate::doors::DoorsTornCause::Blast => crate::wallbreak::WallBreakNoteCCause::Blast,
            crate::doors::DoorsTornCause::Fire => crate::wallbreak::WallBreakNoteCCause::Fire,
        },
        id: g.doors.list.iter().position(|x| *x == d).map_or(-1.0, |i| i as f64),
        name: Some(g.dr[d].def.name.clone()),
        pos: [c[0], c[1], c[2]],
        rect: [c[0] - r, c[2] - r, c[0] + r, c[2] + r],
        opened: Vec::new(),
        walkable: true,
        sight: true,
        dead: true,
        who,
    };
    crate::wallbreak::wall_break_note(g, &row);
}

/// Told of every leaf that comes off, by whatever road, on the frame it leaves its frame.   (doors.ts Doors.torn)
pub fn doors_torn(g: &mut crate::game::Game, d: DoorId, who: f64, cause: Option<crate::doors::DoorsTornCause>) {
    torn_feed(g, d, who, cause.unwrap_or(crate::doors::DoorsTornCause::Blast));
}

/// The first of `dists` out from the door's middle on `side` that has floor.   (doors.ts threadPath's `pick`)
fn thread_pick(cx: f64, cz: f64, n: [f64; 2], dists: &[f64], side: f64, free: &mut dyn FnMut(f64, f64) -> bool) -> Option<V3> {
    for &dd in dists {
        let q: V3 = [cx + n[0] * dd * side, 0.0, cz + n[1] * dd * side];
        if free(q[0], q[2]) { return Some(q); }
    }
    None
}

/// What the leg prev→p needs inserted for one door: where along the leg it crosses, and the points; None if it does not cross or needs none.   (doors.ts threadPath's walk)
fn thread_door(hinge: [f64; 2], closed_dir: f64, width: f64, prev: V3, p: V3, free: &mut dyn FnMut(f64, f64) -> bool) -> Option<(f64, Vec<V3>)> {
    let [hx, hz] = hinge;
    let u: [f64; 2] = [m::cos(closed_dir), m::sin(closed_dir)];
    let n: [f64; 2] = [-u[1], u[0]];
    let (ax, az, bx, bz) = (prev[0] - hx, prev[2] - hz, p[0] - hx, p[2] - hz);
    let (da, db) = (ax * n[0] + az * n[1], bx * n[0] + bz * n[1]);
    if da * db >= 0.0 || m::abs(da - db) < 1e-6 { return None; }
    let t = da / (da - db);
    let (ix, iz) = (ax + (bx - ax) * t, az + (bz - az) * t);
    let along = ix * u[0] + iz * u[1];
    if along < -0.1 || along > width + 0.1 { return None; }
    let mid = width * 0.55;
    let sgn = if da < 0.0 { 1.0 } else { -1.0 };
    let (cx, cz) = (hx + u[0] * mid, hz + u[1] * mid);
    let (before, beyond) = (-da * sgn, db * sgn);
    let mut pts: Vec<V3> = Vec::new();
    if before > 0.75 + 0.5 {
        if let Some(a) = thread_pick(cx, cz, n, &[0.75, 0.55], -sgn, free) { pts.push(a); }
    }
    if beyond > 1.35 + 0.5 {
        if let Some(b) = thread_pick(cx, cz, n, &[1.35, 1.1, 0.85, 0.6], sgn, free) { pts.push(b); }
    }
    if pts.is_empty() { None } else { Some((t, pts)) }
}

/// Insert approach and exit points where a path crosses a doorway, so men take doors head-on and clear the swung leaf before turning.   (doors.ts Doors.threadPath)
pub fn doors_thread_path(g: &mut crate::game::Game, path: &[V3], start: V3, is_free: Option<&mut dyn FnMut(&mut crate::game::Game, f64, f64) -> bool>) -> Vec<V3> {
    if path.is_empty() { return Vec::new(); }
    let mut is_free = is_free;
    let mut out: Vec<V3> = Vec::new();
    let mut prev = start;
    for &p in path {
        let mut ins: Vec<(f64, Vec<V3>)> = Vec::new();
        let mut i = 0;
        while i < g.doors.list.len() {
            let d = g.doors.list[i];
            i += 1;
            let (hinge, closed_dir, width) = (g.dr[d].def.hinge, g.dr[d].def.closed_dir, g.dr[d].def.width);
            let mut free = |x: f64, z: f64| match is_free.as_mut() { Some(f) => f(&mut *g, x, z), None => true };
            if let Some(row) = thread_door(hinge, closed_dir, width, prev, p, &mut free) { ins.push(row); }
        }
        js::sort_by(&mut ins, |p0, p1| p0.0 - p1.0);
        for row in &ins { out.extend_from_slice(&row.1); }
        out.push(p);
        prev = p;
    }
    out
}

/// Every leaf's box and its two handles, for the picture's list (the fog's doorBoxes asks it).   (doors.ts Doors.boxes)
pub fn doors_boxes(g: &mut crate::game::Game, out: &mut Vec<BoxId>) {
    for d in g.doors.list.iter() {
        let dr = &g.dr[*d];
        out.push(dr.box_);
        out.push(dr.handle_boxes[0]);
        out.push(dr.handle_boxes[1]);
    }
}

/// Leaf boxes, for registering as dynamic occluders with the collision layer.   (doors.ts Doors.leafBoxes)
pub fn doors_leaf_boxes(g: &mut crate::game::Game) -> Vec<BoxId> {
    g.doors.list.iter().map(|d| g.dr[*d].box_).collect()
}

/// `fn_` asked aside of every door here.   (doors.ts Doors.posAside)
pub fn doors_pos_aside<T: Clone + core::fmt::Debug>(g: &mut crate::game::Game, fn_: &mut dyn FnMut(&mut crate::game::Game) -> T) -> T {
    let mut list = g.doors.list.clone();
    door_pos_aside(g, &mut list, fn_)
}

#[cfg(test)]
mod tests {
    use super::*;

    // every number below is the TypeScript's own, printed under bun with EXACT.on from doors.ts on a fresh `new Doors`
    const A: ([f64; 2], f64, f64) = ([1.25, -3.5], 0.7, 0.95);
    const C: ([f64; 2], f64, f64) = ([-1.0, -0.8], 0.7, 0.95);
    const U_A: [u64; 2] = [0x3fe87996529f9d92, 0x3fe49d6e694619b8];

    fn u_a() -> [f64; 2] { [f64::from_bits(U_A[0]), f64::from_bits(U_A[1])] }
    fn v_bits(v: V3) -> [u64; 3] { [v[0].to_bits(), v[1].to_bits(), v[2].to_bits()] }

    #[test]
    fn work_spot_has_the_typescripts_bits() {
        let (pos, yaw) = work_spot_at(A.0, u_a(), 1.0, 0.5, A.2 - 0.14);
        assert_eq!(v_bits(pos), [0x3ff8c2347a581444, 0, 0xc004c41f2bdfc5d1]);
        assert_eq!(yaw.to_bits(), 0x40038861baaa937e);
        let (pos, yaw) = work_spot_at(A.0, u_a(), -1.0, 0.5, A.2 - 0.14);
        assert_eq!(v_bits(pos), [0x40018875d77d9090, 0, 0xc00ae284c087ad35]);
        assert_eq!(yaw.to_bits(), 0xbfe6666666666666);
    }

    #[test]
    fn kick_spot_has_the_typescripts_bits() {
        let along = kick_along(A.0, u_a(), A.2, [2.13, 0.0, -1.7]);
        let (pos, yaw) = work_spot_at(A.0, u_a(), 1.0, 0.76, along);
        assert_eq!(v_bits(pos), [0x3ff596d3c20bc618, 0, 0xc00361a20646bd1c]);
        assert_eq!(yaw.to_bits(), 0x40038861baaa937e);
        let along = kick_along(A.0, u_a(), A.2, [0.31, 0.0, -4.9]);
        let (pos, _) = work_spot_at(A.0, u_a(), -1.0, 0.76, along);
        assert_eq!(v_bits(pos), [0x4000d2c01be4c0e1, 0, 0xc00e33c43fff4853]);
    }

    #[test]
    fn side_spot_has_the_typescripts_bits() {
        let k = side_kick();
        let (pos, yaw) = side_spot_at(A.0, u_a(), 1.0, A.2, k.past, k.out);
        assert_eq!(v_bits(pos), [0x3ffda0b2c9e3ae9c, 0, 0xc0023148d3bdbf18]);
        assert_eq!(yaw.to_bits(), 0xc008d48958d01119);
        let (pos, yaw) = side_spot_at(A.0, u_a(), -1.0, A.2, k.past, k.out);
        assert_eq!(v_bits(pos), [0x40047ba48eb1eb2e, 0, 0xc008ec522a7670d4]);
        assert_eq!(yaw.to_bits(), 0xbff7014a5d4e9e67);
    }

    #[test]
    fn pick_gap_has_the_typescripts_bits() {
        let want = [0x3fe7887af8c4a25cu64, 0x3fe3ddc2be561185, 0x3fe9cca41d811a47, 0x3fe621ebe3128970];
        for (i, w) in want.iter().enumerate() { assert_eq!(pick_gap((i + 1) as f64).to_bits(), *w); }
    }

    /// One leg through door A's and door C's frames, doors asked in the list's order (A then C), points in travel order (C then A).
    fn thread(free: &mut dyn FnMut(f64, f64) -> bool) -> Vec<V3> {
        let (start, p): (V3, V3) = ([-3.0, 0.0, 2.3], [3.53, 0.0, -5.5]);
        let mut ins: Vec<(f64, Vec<V3>)> = Vec::new();
        for door in [A, C] { if let Some(row) = thread_door(door.0, door.1, door.2, start, p, free) { ins.push(row); } }
        js::sort_by(&mut ins, |p0, p1| p0.0 - p1.0);
        let mut out = Vec::new();
        for row in &ins { out.extend_from_slice(&row.1); }
        out
    }
    const C_IN: [u64; 3] = [0xbff15626eeb13a66, 0, 0x3fbc3862d0bedf7c];
    const C_OUT: [u64; 3] = [0x3fd13c9a65c7e8d4, 0, 0xbff7ef57a9b54448];
    const A_IN: [u64; 3] = [0x3ff2a9d9114ec59a, 0, 0xc004b7d68313a29e];
    const A_OUT: [u64; 3] = [0x400427934cb8fd1a, 0, 0xc010c8a2b73a1ddf];

    #[test]
    fn thread_sorts_two_doors_into_travel_order() {
        let got: Vec<[u64; 3]> = thread(&mut |_, _| true).into_iter().map(v_bits).collect();
        assert_eq!(got, vec![C_IN, C_OUT, A_IN, A_OUT]);
    }

    #[test]
    fn thread_tries_the_shorter_steps_and_gives_up_on_no_floor() {
        let mut asked: Vec<[u64; 2]> = Vec::new();
        let got: Vec<[u64; 3]> = thread(&mut |x, z| {
            asked.push([x.to_bits(), z.to_bits()]);
            m::hyp2(x - 2.1, z + 3.9) > 1.1
        }).into_iter().map(v_bits).collect();
        assert_eq!(got, vec![C_IN, C_OUT, A_IN]);
        assert_eq!(asked, vec![
            [A_IN[0], A_IN[2]],
            [A_OUT[0], A_OUT[2]],
            [0x4002ddbc66249b7f, 0xc01004d604a520f2],
            [0x400193e57f9039e3, 0xc00e8212a420480b],
            [0x40004a0e98fbd848, 0xc00cfa793ef64e32],
            [C_IN[0], C_IN[2]],
            [C_OUT[0], C_OUT[2]],
        ]);
    }

    #[test]
    fn thread_leaves_a_leg_that_does_not_cross() {
        assert!(thread_door(A.0, A.1, A.2, [6.0, 0.0, 3.0], [2.5, 0.0, -2.0], &mut |_, _| true).is_none());
    }
}
