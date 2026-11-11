use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};


/// THE SIDE IT OPENS INTO: the sign of every angle it can stand at; the page's DOORWAY: exitsOut off, leafWay on
pub fn door_swing(g: &crate::game::Game, id: DoorId) -> f64 {
    let c = g.dr[id].leaf.clearway;
    if js::truthy(c) { c } else { g.dr[id].leaf.dealt }
}

/// its lower stop: one-way, 0 on the side it does not open into
pub fn door_min_a(g: &crate::game::Game, id: DoorId) -> f64 {
    if door_swing(g, id) > 0.0 { 0.0 } else { g.dr[id].min_auth }
}
/// its upper stop
pub fn door_max_a(g: &crate::game::Game, id: DoorId) -> f64 {
    if door_swing(g, id) < 0.0 { 0.0 } else { g.dr[id].max_auth }
}

/// come to rest in its frame by a body's sweep: to, not latched
pub(super) fn door_rests_to(g: &crate::game::Game, id: DoorId) -> bool {
    g.dr[id].leaf.swept && !g.dr[id].leaf.latched && g.dr[id].leaf.angle == 0.0 && g.dr[id].leaf.vel == 0.0
}

/// a door of the level: its row of the columns set as authored, its boxes made
fn door_new(g: &mut crate::game::Game, def: &crate::doors::DoorDef, index: Option<f64>) -> DoorId {
    let index = index.unwrap_or(0.0);
    let mut door = crate::doors::Door::blank();
    door.i = index;
    door.def = def.clone();
    let id = g.dr.push(door);
    let height = def.height.unwrap_or(crate::scene_door_size::LEAF_H);
    let thick = def.thickness.unwrap_or(0.05);
    let min_auth = def.min_angle.unwrap_or(-1.92);
    let max_auth = def.max_angle.unwrap_or(1.92);
    g.dr[id].height = height;
    g.dr[id].thick = thick;
    g.dr[id].min_auth = min_auth;
    g.dr[id].max_auth = max_auth;
    let dealt = match def.swing { Some(s) => s, None => js::or(m::sign(def.angle.unwrap_or(0.0)), if max_auth >= -min_auth { 1.0 } else { -1.0 }) };   // (until dealSwings: the way it is authored standing open, else its wider side)
    g.dr[id].leaf.dealt = dealt;
    g.dr[id].leaf.locked = def.locked == Some(true);
    g.dr[id].key_side = def.key_side.unwrap_or(1.0);
    g.dr[id].lock_id = match &def.lock { Some(l) => l.clone(), None => String::from(FLOOR_KEY) };
    let (sided, lo, hi) = (door_sided(g, id, def.angle.unwrap_or(0.0)), door_min_a(g, id), door_max_a(g, id));
    g.dr[id].leaf.angle = v3::clamp(sided, lo, hi);
    let a = g.dr[id].leaf.angle;
    g.dr[id].leaf.latched = m::abs(a) < 1e-3;
    let leaf_h: V3 = [def.width / 2.0 - 0.01, height / 2.0, thick / 2.0];
    g.dr[id].leaf_h = leaf_h;
    let flag = crate::scene_boxes::box_flag();
    let steel = def.steel == Some(true);
    let marks = if steel { None } else { Some(crate::scene_marking_defaults::door_leaf_marks(leaf_h, None).into()) };
    let leaf = crate::scene_boxes::make_box(g, &crate::scene_boxes::MakeBoxP {
        c: [0.0, 0.0, 0.0],
        h: leaf_h,
        albedo: Some(def.albedo.unwrap_or([0.42, 0.3, 0.19])),
        flags: Some(flag.dynamic),
        rough: Some(if steel { 0.3 } else { 0.55 }),
        metal: if steel { Some(true) } else { None },
        marks,
        ..crate::scene_boxes::MakeBoxP::default()
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
            ..crate::scene_boxes::MakeBoxP::default()
        }));
    }
    g.dr[id].handle_boxes = handles;
    let marks = g.tab.boxes[leaf].marks.clone();
    g.dr[id].leaf_marks = marks;
    g.dr[id].frame_centre = [def.hinge[0] + m::cos(def.closed_dir) * def.width * 0.5, 1.0, def.hinge[1] + m::sin(def.closed_dir) * def.width * 0.5];
    door_place(g, id);
    id
}

/// Leaf centre (1 m up) at the current angle
pub fn door_pos(g: &crate::game::Game, id: DoorId) -> V3 {
    if g.dr[id].leaf.off { return g.dr[id].frame_centre; }   // (off its hinges: 'the door' is its doorway)
    let u = door_dir(g, id);
    let (hinge, width) = (g.dr[id].def.hinge, g.dr[id].def.width);
    [hinge[0] + u[0] * width * 0.5, 1.0, hinge[1] + u[1] * width * 0.5]
}
/// 'Closed enough' that using it means opening it
pub fn door_is_closed(g: &crate::game::Game, id: DoorId) -> bool { m::abs(g.dr[id].leaf.angle) < 0.25 }
pub fn door_dir(g: &crate::game::Game, id: DoorId) -> [f64; 2] {
    let a = g.dr[id].def.closed_dir + g.dr[id].leaf.angle;
    [m::cos(a), m::sin(a)]
}
/// Back to the level as authored
pub fn door_reset(g: &mut crate::game::Game, id: DoorId) {
    if g.dr[id].leaf.off {   // hung again, as authored
        g.dr[id].slab = None;
        let (b, h, marks) = (g.dr[id].box_, g.dr[id].leaf_h, g.dr[id].leaf_marks.clone());
        let mut leaf = g.tab.boxes.edit(b);
        leaf.h = h;
        leaf.marks = marks;
        leaf.rot = None;
        drop(leaf);
        for s in 0..g.dr[id].handle_boxes.len() { let hb = g.dr[id].handle_boxes[s]; g.tab.boxes.edit(hb).rot = None; }
    }
    let d = &mut g.dr[id];
    // (which side it opens into and how far it was picked stay: they are the level's and the night's)
    d.leaf = Leaf { dealt: d.leaf.dealt, clearway: d.leaf.clearway, pick_n: d.leaf.pick_n, locked: d.def.locked == Some(true), ..Leaf::blank() };
    let authored = d.def.angle.unwrap_or(0.0);
    let (sided, lo, hi) = (door_sided(g, id, authored), door_min_a(g, id), door_max_a(g, id));
    let leaf = &mut g.dr[id].leaf;
    leaf.angle = v3::clamp(sided, lo, hi);
    leaf.latched = m::abs(leaf.angle) < 1e-3;
    door_place(g, id);
}
/// Deal the side it opens into: a leaf authored standing open the other way is stood as far open this way
pub fn door_set_swing(g: &mut crate::game::Game, id: DoorId, s: f64, clearway: Option<f64>) {
    g.dr[id].leaf.dealt = s;
    g.dr[id].leaf.clearway = clearway.unwrap_or(0.0);
    door_keep(g, id);
    let a = g.dr[id].leaf.angle;
    g.dr[id].leaf.latched = m::abs(a) < 1e-3;
    door_place(g, id);
}
/// One-way: an angle on the side it does not open into is the same opening on the side it does, its velocity turned with it
fn door_keep(g: &mut crate::game::Game, id: DoorId) {
    if g.dr[id].leaf.angle * door_swing(g, id) < 0.0 {
        let (a, lo, hi) = (g.dr[id].leaf.angle, door_min_a(g, id), door_max_a(g, id));
        let leaf = &mut g.dr[id].leaf;
        leaf.angle = v3::clamp(-a, lo, hi);
        leaf.vel = -leaf.vel;
    }
}
/// an angle (or a turn rate) as this door can have it
fn door_sided(g: &crate::game::Game, id: DoorId, a: f64) -> f64 {
    if a * door_swing(g, id) < 0.0 { -a } else { a }
}
/// the leaf's box and handles where its angle stands them
pub fn door_place(g: &mut crate::game::Game, id: DoorId) {
    if g.dr[id].leaf.off { return; }   // (the slab's box is the door blast's to pose)
    door_keep(g, id);
    let (v, a) = (g.dr[id].leaf.vel, g.dr[id].leaf.angle);
    g.dr[id].leaf.vel_seen = v;
    g.dr[id].leaf.angle_seen = a;
    let rattle = g.dr[id].leaf.rattle_t;
    let jig = if rattle > 0.35 { m::sin(rattle * 90.0) * 0.005 } else { 0.0 };   // the handle being tried: the leaf works a hair in its frame
    let p = door_pose_at(g, id, a + jig);
    leaf_stands(g, id, &p, m::abs(a) < 0.3);
}
/// the leaf's box and its handles put at `p` (an open leaf standing in the room does not seal the doorway acoustically)
pub(super) fn leaf_stands(g: &mut Game, id: DoorId, p: &crate::doors::DoorPoseAt, seals: bool) {
    let d = &g.dr[id];
    let seal = js::to_i32(crate::scene_boxes::box_flag().sound_seal);
    let mut leaf = g.tab.boxes.edit(d.box_);
    (leaf.c, leaf.yaw) = (p.c, p.yaw);
    let flags = js::to_i32(leaf.flags);
    leaf.flags = (if seals { flags | seal } else { flags & !seal }) as f64;
    drop(leaf);
    for (&hb, &c) in d.handle_boxes.iter().zip(&p.handles) {
        let mut handle = g.tab.boxes.edit(hb);
        (handle.c, handle.yaw) = (c, p.yaw);
    }
}
/// Where the leaf's box and its two handles stand with the leaf at `angle`
pub fn door_pose_at(g: &crate::game::Game, id: DoorId, angle: f64) -> crate::doors::DoorPoseAt {
    let d = &g.dr[id];
    pose_of(d.def.hinge, d.def.width, d.def.closed_dir, d.thick, d.height, angle)
}
pub(super) fn pose_of(hinge: [f64; 2], w: f64, closed_dir: f64, thick: f64, height: f64, angle: f64) -> crate::doors::DoorPoseAt {
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

/// the level's doors. `emit` is not kept: the word is `Doors.sound`'s to say, by name
pub fn doors_new(g: &mut crate::game::Game, defs: &[crate::doors::DoorDef], emit: &mut dyn FnMut(&mut crate::game::Game, crate::doors::DoorEvent)) -> crate::doors::Doors {
    assert!(defs.len() <= DOOR_CAP, "a level of {} doors: the game holds {}", defs.len(), DOOR_CAP);
    let mut list = Vec::with_capacity(defs.len());
    for (i, d) in defs.iter().enumerate() {
        list.push(door_new(g, d, Some(i as f64)));
    }
    let key = crate::scene_boxes::mkey().door;
    for (i, &d) in list.iter().enumerate() {   // motion identity for the leaf and its handles
        let (b, h0, h1) = (g.dr[d].box_, g.dr[d].handle_boxes[0], g.dr[d].handle_boxes[1]);
        g.tab.boxes.edit(b).mkey = Some(key + i as f64 * 4.0);
        g.tab.boxes.edit(h0).mkey = Some(key + i as f64 * 4.0 + 1.0);
        g.tab.boxes.edit(h1).mkey = Some(key + i as f64 * 4.0 + 2.0);
    }
    crate::doors::Doors { list, ..crate::doors::Doors::blank() }
}
pub fn doors_by_name(g: &crate::game::Game, n: &str) -> Option<DoorId> {
    for i in 0..g.doors.list.len() {
        let d = g.doors.list[i];
        if g.dr[d].def.name == n { return Some(d); }
    }
    None
}
/// Is `from` on d's PULL face: the side it opens into
pub fn doors_pull_face(g: &crate::game::Game, d: DoorId, from: V3) -> bool {
    let (c, hinge) = (g.dr[d].def.closed_dir, g.dr[d].def.hinge);
    ((from[0] - hinge[0]) * -m::sin(c) + (from[2] - hinge[1]) * m::cos(c)) * door_swing(g, d) >= 0.0
}
/// WOULD THE FRAME HOLD A BOOT thrown at d from `from`
pub fn doors_boot_holds(g: &crate::game::Game, d: DoorId, from: V3) -> bool {
    !g.dr[d].leaf.off && g.dr[d].def.steel != Some(true) && m::abs(g.dr[d].leaf.angle) < SHUT_HOME && doors_pull_face(g, d, from)
}
/// THE HOLD TEST for a wedge or dragged furniture put against `d` by a man at `from`
pub fn doors_wedge_holds(g: &crate::game::Game, d: DoorId, from: V3) -> bool {
    doors_pull_face(g, d, from)
}
/// THE DOOR AS A HINGED THING: its jambs, the side it opens into, the floor its leaf sweeps
pub fn doors_swing_of(g: &crate::game::Game, d: DoorId) -> crate::doors::DoorSwing {
    let (hx, hz, w, c) = (g.dr[d].def.hinge[0], g.dr[d].def.hinge[1], g.dr[d].def.width, g.dr[d].def.closed_dir);
    let s = door_swing(g, d);
    let open = m::abs(if s > 0.0 { g.dr[d].max_auth } else { g.dr[d].min_auth });
    crate::doors::DoorSwing { hinge: [hx, 0.0, hz], swing: s, open, radius: w, closed_dir: c, one_way: true }
}
/// how far (x, z) is from the leaf standing at world angle `a`
fn swing_seg(s: &crate::doors::DoorSwing, x: f64, z: f64, a: f64) -> f64 {
    let (hx, hz) = (s.hinge[0], s.hinge[2]);
    let (vx, vz) = (m::cos(a), m::sin(a));
    let t = v3::clamp((x - hx) * vx + (z - hz) * vz, 0.0, s.radius);
    m::hyp2(x - hx - vx * t, z - hz - vz * t)
}
/// is the floor point inside the area the leaf sweeps, with `pad` m of slack
pub fn door_swing_sweeps(s: &crate::doors::DoorSwing, x: f64, z: f64, pad: Option<f64>) -> bool {
    let pad = pad.unwrap_or(0.0);
    let (px, pz) = (x - s.hinge[0], z - s.hinge[2]);
    let r = m::hyp2(px, pz);
    if r > s.radius + pad { return false; }
    let (c, sw, open) = (s.closed_dir, s.swing, s.open);
    let mut th = (m::atan2(pz, px) - c) * sw;
    th -= m::round(th / (2.0 * m::PI)) * 2.0 * m::PI;
    if th < 0.0 && open > m::PI && th + 2.0 * m::PI <= open { th += 2.0 * m::PI; }   // the point's bearing from the closed leaf, the way it opens
    (th >= 0.0 && th <= open) || swing_seg(s, x, z, c) <= pad || swing_seg(s, x, z, c + sw * open) <= pad
}
/// Which face of d the point is on: +1 the CCW side of the leaf, -1 the other
pub fn doors_side(g: &crate::game::Game, d: DoorId, from: V3) -> f64 {
    let u = door_dir(g, d);
    let (px, pz) = (from[0] - g.dr[d].def.hinge[0], from[2] - g.dr[d].def.hinge[1]);
    if u[0] * pz - u[1] * px >= 0.0 { 1.0 } else { -1.0 }
}
/// The lock is against you: latched, still locked, you on the keyed side, and nothing on your ring fits it
// (a ring is asked through `group::ring_has`, the one way to ask it: a cast that carries the building's keys holds the ring that answers
//  yes to every lock, and a plain `has` of the lock's name answers no for it: the response team's lead would boot a
//  door his keys open and not go to the handle)
pub fn doors_locked_out(g: &crate::game::Game, d: DoorId, from: V3, keys: Option<&JsSet<String>>) -> bool {
    g.dr[d].leaf.locked && g.dr[d].leaf.latched && doors_side(g, d, from) == g.dr[d].key_side && !(match keys { Some(k) => crate::group::ring_has(k, &g.dr[d].lock_id), None => false })
}
/// The lock is against you BUT for the key you carry
pub fn doors_key_opens(g: &crate::game::Game, d: DoorId, from: V3, keys: Option<&JsSet<String>>) -> bool {
    doors_locked_out(g, d, from, None) && (match keys { Some(k) => crate::group::ring_has(k, &g.dr[d].lock_id), None => false })
}

#[cfg(test)]
mod tests {
    extern crate std;
    use super::*;
    fn bits(x: &[f64]) -> Vec<u64> { x.iter().map(|v| v.to_bits()).collect() }

    /// the pinned bits of a door (hinge [1.25, -3.5], closed at 0.7, 0.93 wide, 0.06 thick, 2.05 tall) posed at 0.37
    #[test]
    fn the_pose_is_the_typescripts_to_the_bit() {
        let p = pose_of([1.25, -3.5], 0.93, 0.7, 0.06, 2.05, 0.37);
        let got = [p.c[0], p.c[1], p.c[2], p.yaw, p.handles[0][0], p.handles[0][1], p.handles[0][2], p.handles[1][0], p.handles[1][1], p.handles[1][2]];
        assert_eq!(bits(&got), [0x3ff7a6213404e158, 0x3ff08f5c28f5c28f, 0xc008aaa8cb200044, 0xbff11eb851eb851e, 0x3ff93a063e610b0e, 0x3ff0000000000000, 0xc00639c318a617ee, 0x3ffae92f9c7699be, 0x3ff0000000000000, 0xc006afc1e6a1520a]);
    }

    /// the pinned answers of that door with swing -1
    #[test]
    fn the_sweep_answers_as_the_typescript_does() {
        let s = crate::doors::DoorSwing { hinge: [1.25, 0.0, -3.5], swing: -1.0, open: 1.92, radius: 0.93, closed_dir: 0.7, one_way: true };
        for (x, z, pad, want) in [(1.761, -3.331, 0.0, true), (1.6, -3.2, 0.0, false), (1.25, -2.3, 0.2, false), (1.25, -4.2, 0.1, false), (2.0, -2.8, 0.05, false), (2.4, -3.1, 0.1, false), (0.8, -3.9, 0.05, false), (1.3, -3.9, 0.2, true)] {
            assert_eq!(door_swing_sweeps(&s, x, z, Some(pad)), want, "{x} {z} {pad}");
        }
        assert_eq!(door_swing_sweeps(&s, 1.761, -3.331, None), door_swing_sweeps(&s, 1.761, -3.331, Some(0.0)));   // (pad = 0)
    }

    /// a level with no door has no step: only the reset of what a leaf did to each contact
    #[test]
    fn with_no_door_the_contacts_are_only_reset() {
        std::thread::Builder::new().stack_size(1 << 30).spawn(|| {
            let mut g = alloc::boxed::Box::new(Game::blank());
            let d = DoorId(3);
            let hit = crate::doors::DoorStrike { door: d, ..crate::doors::DoorStrike::default() };
            let run = crate::doors::DoorContactRun { bash: 1.0, rip: 2.0, rip_level: 3.0 };
            let full = crate::doors::DoorContact { blocked_by: Some(d), bashed: Some(d), ripped: Some(d), bashed_at: Some(0.5), struck: Some(hit), moved: Some(true), man: Some(7.0), ..crate::doors::DoorContact::default() };
            let mut v = vec![crate::doors::DoorContact { run: Some(run), ..full.clone() }, full];
            doors_update(&mut g, 0.016, &mut v);
            let (a, b) = (&v[0], &v[1]);
            assert!(a.blocked_by.is_none() && a.bashed.is_none() && a.ripped.is_none() && a.struck.is_none() && a.man.is_none());
            assert_eq!(a.moved, Some(false));
            assert_eq!(a.bashed_at, Some(0.5));   // (the step leaves it)
            assert!(b.blocked_by.is_none() && b.bashed.is_some() && b.ripped.is_some());   // (no run: not his to clear)
        }).unwrap().join().unwrap();
    }
}
