//! src/game/doors.ts lines 296 to 689
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// A box's row (`Box.*`, scene/boxes.ts:66). Skeleton 5 has `g.col.boxes: Vec<BoxId>` and no table of Box rows, while the index says `g.col.boxes[id].c`. Once the table is back, the body is that row (and a leaf is a made box: notes)
fn seam_box<'g>(g: &'g mut Game, b: BoxId) -> &'g mut crate::scene_boxes::Box { &mut g.tab.boxes[b] }

/// THE SIDE IT OPENS INTO: the sign of every angle it can stand at; the page's DOORWAY: exitsOut off, leafWay on   (doors.ts Door.swing)
pub fn door_swing(g: &crate::game::Game, id: DoorId) -> f64 {
    let c = g.dr[id].leaf.clearway;
    if js::truthy(c) { c } else { g.dr[id].leaf.dealt }
}

/// its lower stop: one-way, 0 on the side it does not open into   (doors.ts Door.minA)
pub fn door_min_a(g: &crate::game::Game, id: DoorId) -> f64 {
    if door_swing(g, id) > 0.0 { 0.0 } else { g.dr[id].min_auth }
}
/// its upper stop   (doors.ts Door.maxA)
pub fn door_max_a(g: &crate::game::Game, id: DoorId) -> f64 {
    if door_swing(g, id) < 0.0 { 0.0 } else { g.dr[id].max_auth }
}

/// come to rest in its frame by a body's sweep: to, not latched   (doors.ts Door.restsTo)
pub fn door_rests_to(g: &crate::game::Game, id: DoorId) -> bool {
    g.dr[id].leaf.swept && !g.dr[id].leaf.latched && g.dr[id].leaf.angle == 0.0 && g.dr[id].leaf.vel == 0.0
}

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

/// a door of the level: its row of the columns set as authored, its boxes made   (doors.ts Door.constructor)
pub fn door_new(g: &mut crate::game::Game, def: &crate::doors::DoorDef, index: Option<f64>) -> DoorId {
    let index = index.unwrap_or(0.0);
    let mut door = crate::doors::Door::blank();
    door.i = index;
    door.slab_row = slab_row_new(g, index);
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
    let marks = seam_box(g, leaf).marks.clone();
    g.dr[id].leaf_marks = marks;
    g.dr[id].frame_centre = [def.hinge[0] + m::cos(def.closed_dir) * def.width * 0.5, 1.0, def.hinge[1] + m::sin(def.closed_dir) * def.width * 0.5];
    door_place(g, id);
    id
}

/// Leaf centre (1 m up) at the current angle   (doors.ts Door.pos)
pub fn door_pos(g: &crate::game::Game, id: DoorId) -> V3 {
    if g.dr[id].leaf.off { return g.dr[id].frame_centre; }   // (off its hinges: 'the door' is its doorway)
    let u = door_dir(g, id);
    let (hinge, width) = (g.dr[id].def.hinge, g.dr[id].def.width);
    [hinge[0] + u[0] * width * 0.5, 1.0, hinge[1] + u[1] * width * 0.5]
}
/// `p` is the kept centre from here on   (doors.ts Door.keptPos)
pub fn door_kept_pos(g: &mut crate::game::Game, id: DoorId, p: V3) {
    g.dr[id].pos_at = p;
}
/// the kept centre as it stands, whatever angle the leaf has now   (doors.ts Door.keptAt)
pub fn door_kept_at(g: &crate::game::Game, id: DoorId) -> V3 { g.dr[id].pos_at }
/// 'Closed enough' that using it means opening it   (doors.ts Door.isClosed)
pub fn door_is_closed(g: &crate::game::Game, id: DoorId) -> bool { m::abs(g.dr[id].leaf.angle) < 0.25 }
/// (doors.ts Door.dir)
pub fn door_dir(g: &crate::game::Game, id: DoorId) -> [f64; 2] {
    let a = g.dr[id].def.closed_dir + g.dr[id].leaf.angle;
    [m::cos(a), m::sin(a)]
}
/// Back to the level as authored   (doors.ts Door.reset)
pub fn door_reset(g: &mut crate::game::Game, id: DoorId) {
    g.dr[id].leaf.shot = 0.0;
    if g.dr[id].leaf.off {   // hung again, as authored
        g.dr[id].leaf.off = false;
        door_set_slab(g, id, None);
        let (b, h, marks) = (g.dr[id].box_, g.dr[id].leaf_h, g.dr[id].leaf_marks.clone());
        let leaf = seam_box(g, b);
        leaf.h = h;
        leaf.marks = marks;
        leaf.rot = None;
        for s in 0..g.dr[id].handle_boxes.len() { let hb = g.dr[id].handle_boxes[s]; seam_box(g, hb).rot = None; }
    }
    let authored = g.dr[id].def.angle.unwrap_or(0.0);
    let (sided, lo, hi) = (door_sided(g, id, authored), door_min_a(g, id), door_max_a(g, id));
    g.dr[id].leaf.angle = v3::clamp(sided, lo, hi);
    g.dr[id].leaf.vel = 0.0;
    let a = g.dr[id].leaf.angle;
    g.dr[id].leaf.latched = m::abs(a) < 1e-3;
    g.dr[id].leaf.closing = false; g.dr[id].leaf.swept = false; g.dr[id].leaf.quiet_t = 10.0; g.dr[id].leaf.since_bash = 10.0; g.dr[id].leaf.creak_armed = true; g.dr[id].leaf.last_who = -1.0;
    let locked = g.dr[id].def.locked == Some(true);
    g.dr[id].leaf.locked = locked; g.dr[id].leaf.lock_broken = false; g.dr[id].leaf.noticed = false; g.dr[id].leaf.noticed_t = -1000.0; g.dr[id].leaf.pick = 0.0; g.dr[id].leaf.picking = false;
    g.dr[id].leaf.pick_idle = 0.0; g.dr[id].leaf.pick_clock = 0.0; g.dr[id].leaf.rattle_t = 0.0; g.dr[id].leaf.guard_t = -100.0; g.dr[id].leaf.passers = 0;
    g.dr[id].leaf.pull_t = 0.0; g.dr[id].leaf.press_t = 0.0; g.dr[id].leaf.solid_t = 0.0; g.dr[id].leaf.open_t = 10.0; g.dr[id].leaf.drive = None;
    door_place(g, id);
}
/// Deal the side it opens into: a leaf authored standing open the other way is stood as far open this way   (doors.ts Door.setSwing)
pub fn door_set_swing(g: &mut crate::game::Game, id: DoorId, s: f64, outward: Option<f64>, clearway: Option<f64>) {
    g.dr[id].leaf.dealt = s;

    g.dr[id].leaf.clearway = clearway.unwrap_or(0.0);
    door_keep(g, id);
    let a = g.dr[id].leaf.angle;
    g.dr[id].leaf.latched = m::abs(a) < 1e-3;
    door_place(g, id);
}
/// One-way: an angle on the side it does not open into is the same opening on the side it does, its velocity turned with it   (doors.ts Door.keep)
pub fn door_keep(g: &mut crate::game::Game, id: DoorId) {
    if g.dr[id].leaf.angle * door_swing(g, id) < 0.0 {
        let (a, lo, hi) = (g.dr[id].leaf.angle, door_min_a(g, id), door_max_a(g, id));
        g.dr[id].leaf.angle = v3::clamp(-a, lo, hi);
        let v = g.dr[id].leaf.vel;
        g.dr[id].leaf.vel = -v;
        let n = g.dr[id].leaf.mirrored;
        g.dr[id].leaf.mirrored = n + 1.0;
    }
}
/// an angle (or a turn rate) as this door can have it   (doors.ts Door.sided)
pub fn door_sided(g: &crate::game::Game, id: DoorId, a: f64) -> f64 {
    if a * door_swing(g, id) < 0.0 { -a } else { a }
}
/// the leaf's box and handles where its angle stands them   (doors.ts Door.place)
pub fn door_place(g: &mut crate::game::Game, id: DoorId) {
    if g.dr[id].leaf.off { return; }   // (the slab's box is doorblast.ts's to pose)
    door_keep(g, id);
    let (v, a) = (g.dr[id].leaf.vel, g.dr[id].leaf.angle);
    g.dr[id].leaf.vel_seen = v;
    g.dr[id].leaf.angle_seen = a;
    let rattle = g.dr[id].leaf.rattle_t;
    let jig = if rattle > 0.35 { m::sin(rattle * 90.0) * 0.005 } else { 0.0 };   // the handle being tried: the leaf works a hair in its frame
    let p = door_pose_at(g, id, a + jig);
    let b = g.dr[id].box_;
    let seal = js::to_i32(crate::scene_boxes::box_flag().sound_seal);
    let leaf = seam_box(g, b);
    leaf.c = p.c;
    leaf.yaw = p.yaw;
    let flags = js::to_i32(leaf.flags);
    leaf.flags = (if m::abs(a) < 0.3 { flags | seal } else { flags & !seal }) as f64;   // an open leaf standing in the room does not seal the doorway acoustically
    for s in 0..2 {
        let hb = g.dr[id].handle_boxes[s];
        let handle = seam_box(g, hb);
        handle.c = p.handles[s];
        handle.yaw = p.yaw;
    }
    crate::scene_box_mirror::leaf_wrote(g);
}
/// Where the leaf's box and its two handles stand with the leaf at `angle`   (doors.ts Door.poseAt)
pub fn door_pose_at(g: &crate::game::Game, id: DoorId, angle: f64) -> crate::doors::DoorPoseAt {
    let d = &g.dr[id];
    pose_of(d.def.hinge, d.def.width, d.def.closed_dir, d.thick, d.height, angle)
}
pub fn pose_of(hinge: [f64; 2], w: f64, closed_dir: f64, thick: f64, height: f64, angle: f64) -> crate::doors::DoorPoseAt {
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

/// the level's doors: their columns a part of the game's one block, `g.sim`. `emit` is not kept: the word is `Doors.sound`'s to say, by name   (doors.ts Doors.constructor)
pub fn doors_new(g: &mut crate::game::Game, defs: &[crate::doors::DoorDef], emit: &mut dyn FnMut(&mut crate::game::Game, crate::doors::DoorEvent)) -> crate::doors::Doors {
    if defs.len() as f64 > DOOR_CAP { panic!("a level of {} doors: the game holds {}", defs.len(), DOOR_CAP); }
    let slabs = crate::sim_slabshome::slab_store_new(g, crate::Opaque, defs.len() as f64);
    let mut list = Vec::with_capacity(defs.len());
    for (i, d) in defs.iter().enumerate() {
        list.push(door_new(g, d, Some(i as f64)));
    }
    let key = crate::scene_boxes::mkey().door;
    for (i, &d) in list.iter().enumerate() {   // motion identity for the leaf and its handles (scene/boxes.ts MKEY)
        let (b, h0, h1) = (g.dr[d].box_, g.dr[d].handle_boxes[0], g.dr[d].handle_boxes[1]);
        seam_box(g, b).mkey = Some(key + i as f64 * 4.0);
        seam_box(g, h0).mkey = Some(key + i as f64 * 4.0 + 1.0);
        seam_box(g, h1).mkey = Some(key + i as f64 * 4.0 + 2.0);
    }
    crate::doors::Doors { list, slabs, ..crate::doors::Doors::blank() }
}
/// (doors.ts Doors.byName)
pub fn doors_by_name(g: &crate::game::Game, n: &str) -> Option<DoorId> {
    for i in 0..g.doors.list.len() {
        let d = g.doors.list[i];
        if g.dr[d].def.name == n { return Some(d); }
    }
    None
}
/// Is `from` on d's PULL face: the side it opens into   (doors.ts Doors.pullFace)
pub fn doors_pull_face(g: &crate::game::Game, d: DoorId, from: V3) -> bool {
    let (c, hinge) = (g.dr[d].def.closed_dir, g.dr[d].def.hinge);
    ((from[0] - hinge[0]) * -m::sin(c) + (from[2] - hinge[1]) * m::cos(c)) * door_swing(g, d) >= 0.0
}
/// DOORWAY.frameHolds as it applies right now: no showcase on the stage   (doors.ts Doors.frameRule)
pub fn doors_frame_rule(g: &crate::game::Game) -> bool { !doors_staged(g) }
/// WOULD THE FRAME HOLD A BOOT thrown at d from `from`   (doors.ts Doors.bootHolds)
pub fn doors_boot_holds(g: &crate::game::Game, d: DoorId, from: V3) -> bool {
    doors_frame_rule(g) && !g.dr[d].leaf.off && g.dr[d].def.steel != Some(true) && m::abs(g.dr[d].leaf.angle) < SHUT_HOME && doors_pull_face(g, d, from)
}
/// THE HOLD TEST for a wedge or dragged furniture put against `d` by a man at `from`   (doors.ts Doors.wedgeHolds)
pub fn doors_wedge_holds(g: &crate::game::Game, d: DoorId, from: V3) -> bool {
    !doors_frame_rule(g) || doors_pull_face(g, d, from)
}
/// THE DOOR AS A HINGED THING: its jambs, the side it opens into, the floor its leaf sweeps   (doors.ts Doors.swingOf)
pub fn doors_swing_of(g: &crate::game::Game, d: DoorId) -> crate::doors::DoorSwing {
    let (hx, hz, w, c) = (g.dr[d].def.hinge[0], g.dr[d].def.hinge[1], g.dr[d].def.width, g.dr[d].def.closed_dir);
    let s = door_swing(g, d);
    let open = m::abs(if s > 0.0 { g.dr[d].max_auth } else { g.dr[d].min_auth });
    crate::doors::DoorSwing { hinge: [hx, 0.0, hz], swing: s, open, radius: w, closed_dir: c, one_way: true }
}
/// how far (x, z) is from the leaf standing at world angle `a`   (doors.ts Doors.swingOf seg)
fn swing_seg(s: &crate::doors::DoorSwing, x: f64, z: f64, a: f64) -> f64 {
    let (hx, hz) = (s.hinge[0], s.hinge[2]);
    let (vx, vz) = (m::cos(a), m::sin(a));
    let t = v3::clamp((x - hx) * vx + (z - hz) * vz, 0.0, s.radius);
    m::hyp2(x - hx - vx * t, z - hz - vz * t)
}
/// is the floor point inside the area the leaf sweeps, with `pad` m of slack   (doors.ts DoorSwing.sweeps)
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
/// a showcase / the tour / the attract reel has the stage (game.ts:1200)   (doors.ts Doors.staged)
pub fn doors_staged(g: &crate::game::Game) -> bool { g.touring }
/// DOORSTAY as it applies right now: leaves stay where they are left   (doors.ts Doors.stay)
pub fn doors_stay(g: &crate::game::Game) -> bool { !doors_staged(g) }
/// does this leaf's hydraulic closer work   (doors.ts Doors.hasCloser)
pub fn doors_has_closer(g: &crate::game::Game, d: DoorId) -> bool { !doors_stay(g) && g.dr[d].def.closer != Some(false) }
/// Which face of d the point is on: +1 the CCW side of the leaf, -1 the other   (doors.ts Doors.side)
pub fn doors_side(g: &crate::game::Game, d: DoorId, from: V3) -> f64 {
    let u = door_dir(g, d);
    let (px, pz) = (from[0] - g.dr[d].def.hinge[0], from[2] - g.dr[d].def.hinge[1]);
    if u[0] * pz - u[1] * px >= 0.0 { 1.0 } else { -1.0 }
}
/// The lock is against you: latched, still locked, you on the keyed side, and nothing on your ring fits it   (doors.ts Doors.lockedOut)
// (a ring is asked through `squadcast::ring_has`, the one way to ask it: a cast that carries the building's keys holds the ring that answers
//  yes to every lock, squadcast.ts:17 EVERY_KEY, and a plain `has` of the lock's name answered no for it: the response team's lead booted a
//  door his keys open where the TypeScript's lead goes to the handle)
pub fn doors_locked_out(g: &crate::game::Game, d: DoorId, from: V3, keys: Option<&JsSet<String>>) -> bool {
    g.dr[d].leaf.locked && g.dr[d].leaf.latched && doors_side(g, d, from) == g.dr[d].key_side && !(match keys { Some(k) => crate::squadcast::ring_has(k, &g.dr[d].lock_id), None => false })
}
/// The lock is against you BUT for the key you carry   (doors.ts Doors.keyOpens)
pub fn doors_key_opens(g: &crate::game::Game, d: DoorId, from: V3, keys: Option<&JsSet<String>>) -> bool {
    (match keys { Some(k) => crate::squadcast::ring_has(k, &g.dr[d].lock_id), None => false }) && doors_locked_out(g, d, from, None)
}
/// DOORTHROW as it applies right now: no showcase on the stage   (doors.ts Doors.throws)
pub fn doors_throws(g: &crate::game::Game) -> bool { !doors_staged(g) }
/// would a turn of sign `dir` take d further OPEN (one-way: the way it opens)   (doors.ts Doors.opening)
pub fn doors_opening(g: &crate::game::Game, d: DoorId, dir: f64) -> bool { dir == door_swing(g, d) }

/// the world the step stands in made ready, with the game's own leaves in list 0: nothing to do with no door in the level. In Rust the step is always rust/sim's, so what was a way back to the TypeScript's own step is an assert   (doors.ts Doors.there)
pub fn doors_there(g: &mut crate::game::Game) {
    if g.doors.list.is_empty() { return; }
    let dyn_ = g.col.dynamic_boxes.clone();
    debug_assert!(dyn_.len() == g.doors.list.len() && g.doors.list.iter().zip(dyn_.iter()).all(|(&d, &b)| g.dr[d].box_ == b), "the leaves in the box world are not this level's doors");
    crate::scene_collision::rules_world(g, &dyn_);
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
        let s = crate::doors::DoorSwing { hinge: [1.25, 0.0, -3.5], swing: -1.0, open: 1.92, radius: 0.93, closed_dir: 0.7, one_way: true };
        for (x, z, pad, want) in [(1.761, -3.331, 0.0, true), (1.6, -3.2, 0.0, false), (1.25, -2.3, 0.2, false), (1.25, -4.2, 0.1, false), (2.0, -2.8, 0.05, false), (2.4, -3.1, 0.1, false), (0.8, -3.9, 0.05, false), (1.3, -3.9, 0.2, true)] {
            assert_eq!(door_swing_sweeps(&s, x, z, Some(pad)), want, "{x} {z} {pad}");
        }
        assert_eq!(door_swing_sweeps(&s, 1.761, -3.331, None), door_swing_sweeps(&s, 1.761, -3.331, Some(0.0)));   // (pad = 0)
    }

    /// a level with no door has no step: only the reset of what a leaf did to each contact   (doors.ts:640, :761)
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
            assert_eq!(a.bashed_at, Some(0.5));   // (the TypeScript leaves it)
            assert!(b.blocked_by.is_none() && b.bashed.is_some() && b.ripped.is_some());   // (no run: not his to clear)
        }).unwrap().join().unwrap();
    }
}
