pub use ::game_types::doorblast::types;
pub use types::*;
use crate::game::{Deed, Game};
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

mod splinter;
pub use splinter::splinter;

/// A leaf as it stands: its bearing, along it from its hinge, its middle.
struct Stands {
    a: f64,
    u: [f64; 2],
    c: [f64; 2],
}

fn stands(g: &Game, d: DoorId) -> Stands {
    let [hx, hz] = g.dr[d].def.hinge;
    let half = g.dr[d].def.width * 0.5 + 0.01;
    let a = g.dr[d].def.closed_dir + g.dr[d].leaf.angle;
    let [ux, uz] = [m::cos(a), m::sin(a)];
    Stands { a, u: [ux, uz], c: [hx + ux * half, hz + uz * half] }
}

impl Stands {
    /// Straight off its face, the way `fly` goes.
    fn out(&self, fly: f64) -> [f64; 2] {
        [-self.u[1] * fly, self.u[0] * fly]
    }
}

/// Where the handle is on a leaf `w` by `h`, from its middle.
fn handle_on(w: f64, h: f64) -> [f64; 2] {
    [(w - 0.14) - (w * 0.5 + 0.01), 1.0 - (h / 2.0 + 0.01)]
}

/// A leaf no blast and no round takes: steel, or off already.
fn stays(g: &Game, d: DoorId) -> bool {
    g.dr[d].leaf.off || g.dr[d].def.steel.unwrap_or(false)
}


/// Keeps a line of what the doors' blasts did.
fn note(g: &mut crate::game::Game, s: &str) {
    let line = format!("{}s {}", js::to_fixed(g.time, 2), s);
    let log = &mut g.m.doorblast.doorblast_log;
    log.push(line);
    if log.len() > 200 {
        log.remove(0);
    }
}

/// The slab's centre height with its foot (or its face) on the floor at this much tip.
fn support(g: &crate::game::Game, d: DoorId, tip: f64) -> f64 {
    (g.dr[d].height / 2.0) * m::cos(tip) + (g.dr[d].thick / 2.0) * m::sin(tip) + LIFT
}

/// Does a floor rectangle cut into a static standing in the heights [y0, y1]?
fn overlaps(g: &mut crate::game::Game, x: f64, z: f64, a: f64, hu: f64, hn: f64, y0: f64, y1: f64) -> bool {
    let (ux, uz, su, sn) = (m::cos(a), m::sin(a), hu - 0.012, hn - 0.012);
    let ex = m::abs(ux) * hu + m::abs(uz) * hn;
    let ez = m::abs(uz) * hu + m::abs(ux) * hn;
    let mut ov = ov();
    let n = crate::scene_collision::gather(g, x - ex, z - ez, x + ex, z + ez, y0, y1, &mut ov);
    for i in 0..n as usize {
        let b = g.col.boxes[ov[i] as usize];
        let row = &g.tab.boxes[b];
        let (yaw, bc, bh) = (row.yaw, row.c, row.h);
        let (bx, bz, dx, dz) = (m::cos(yaw), -m::sin(yaw), bc[0] - x, bc[2] - z);
        let c = m::abs(ux * bx + uz * bz);
        let s = m::abs(ux * bz - uz * bx);
        if m::abs(dx * ux + dz * uz) > su + bh[0] * c + bh[2] * s {
            continue;
        }
        if m::abs(-dx * uz + dz * ux) > sn + bh[0] * s + bh[2] * c {
            continue;
        }
        if m::abs(dx * bx + dz * bz) > bh[0] + su * c + sn * s {
            continue;
        }
        if m::abs(-dx * bz + dz * bx) > bh[2] + su * s + sn * c {
            continue;
        }
        return true;
    }
    false
}

/// The nearest spot within maxR where that rectangle stands clear, by rings of sixteen bearings.
fn free_spot(g: &mut crate::game::Game, x: f64, z: f64, a: f64, hu: f64, hn: f64, y0: f64, y1: f64, max_r: f64) -> Option<[f64; 2]> {
    let mut r = 0.03;
    while r <= max_r + 1e-9 {
        for i in 0..16 {
            let th = a + (i as f64) * core::f64::consts::PI / 8.0;
            let px = x + m::cos(th) * r;
            let pz = z + m::sin(th) * r;
            if !overlaps(g, px, pz, a, hu, hn, y0, y1) {
                return Some([px, pz]);
            }
        }
        r += if r < 0.3 { 0.03 } else { 0.06 };
    }
    None
}



/// The doors' side of a blast: every leaf within `tear_m` that it reaches comes off.
pub(super) fn door_blast(g: &mut crate::game::Game, at: V3, tear_m: f64, who: f64) -> Vec<DoorId> {
    let mut out = Vec::new();
    if !(tear_m > 0.0) {
        return out;
    }
    for item in door_blast_reach(g, at, tear_m) {
        if tear_off(g, item.d, at, who, Some(item.k)) {
            out.push(item.d);
        }
    }
    out
}

/// The leaves a blast would take off, each with its strength there, asked without touching anything.
pub fn door_blast_reach(g: &mut crate::game::Game, at: V3, tear_m: f64) -> Vec<crate::doorblast::DoorBlastReachItem> {
    let mut out = Vec::new();
    if !(tear_m > 0.0) {
        return out;
    }
    let list = g.doors.list.clone();
    for d in list {
        if stays(g, d) {
            continue;
        }
        let [hx, hz] = g.dr[d].def.hinge;
        let w = g.dr[d].def.width;
        let (px, pz) = (at[0] - hx, at[2] - hz);
        if px * px + pz * pz > m::pow(w + tear_m + 0.1, 2.0) {
            continue;
        }
        let u = crate::doors::door_dir(g, d);
        let t = v3::clamp(px * u[0] + pz * u[1], 0.0, w);
        let y = v3::clamp(at[1], 0.05, g.dr[d].height);
        let dist = m::max(0.0, m::hyp3(px - u[0] * t, at[1] - y, pz - u[1] * t) - g.dr[d].thick / 2.0);
        if dist > tear_m || !reaches(g, d, at) {
            continue;
        }
        out.push(crate::doorblast::DoorBlastReachItem { d, k: 1.0 - dist / tear_m });
    }
    out
}

/// Has the blast a line to the leaf that no wall is in (the leaves themselves are not walls to it).
fn reaches(g: &mut crate::game::Game, d: DoorId, at: V3) -> bool {
    let [hx, hz] = g.dr[d].def.hinge;
    let w = g.dr[d].def.width;
    let u = crate::doors::door_dir(g, d);
    let s = if u[0] * (at[2] - hz) - u[1] * (at[0] - hx) >= 0.0 { 1.0 } else { -1.0 };
    let off = g.dr[d].thick / 2.0 + 0.08;
    let height = g.dr[d].height;
    let from: V3 = [at[0], m::max(at[1], 0.15), at[2]];
    let walls = crate::ask::Over::statics();
    let mut open = false;
    'lines: for f in [0.5, 0.85, 0.15] {
        for y in [1.0, 0.3, 1.8] {
            let p: V3 = [hx + u[0] * w * f - u[1] * off * s, m::min(y, height - 0.1), hz + u[1] * w * f + u[0] * off * s];
            if v3::dist(from, p) < 0.12 || !crate::scene_collision::solid_blocked_over(g, &walls, from, p, None) {
                open = true;
                break 'lines;
            }
        }
    }
    open
}

/// Sam's hands come off a leaf that has left its frame.
fn hands_off(g: &mut crate::game::Game, d: DoorId) {
    for pl in crate::seats::men(g) {
        let at = match &g.pl[pl].deed {
            Some(Deed::Ease(e)) => e.door,
            Some(Deed::Code(c)) => c.door,
            Some(Deed::Wand(w)) => w.door,
            Some(Deed::Pick(p)) => *p,
            _ => continue,
        };
        if at != d { continue; }
        if g.pl[pl].picking().is_some() { g.pl[pl].speed_sm = 0.0; }
        crate::deed::put_down(g, pl);
    }
}

/// The frame lets go of a leaf: its lock is gone, and `who` is the last to have touched it.
fn frame_lets_go(g: &mut Game, d: DoorId, who: f64) {
    if !g.dr[d].leaf.lock_broken {
        g.dr[d].leaf.noticed = false;
    }
    g.dr[d].leaf.locked = false;
    g.dr[d].leaf.lock_broken = true;
    g.dr[d].leaf.pick = 0.0;
    g.dr[d].leaf.picking = false;
    g.dr[d].leaf.rattle_t = 0.0;
    g.dr[d].leaf.last_who = who;
}

/// A door's state once its leaf is off: wide open and quiet.
fn leaf_is_off(g: &mut Game, d: DoorId, fly: f64) {
    g.dr[d].leaf.off = true;
    g.dr[d].leaf.angle = fly * OFF_ANGLE;
    g.dr[d].leaf.vel = 0.0;
    g.dr[d].leaf.latched = false;
    g.dr[d].leaf.closing = false;
    g.dr[d].leaf.passers = 0;
    g.dr[d].leaf.quiet_t = 10.0;
    g.dr[d].leaf.since_bash = 10.0;
    g.dr[d].leaf.creak_armed = false;
    g.dr[d].leaf.shot = 0.0;
}

/// The leaf comes off, thrown out of its frame away from `from`. False, and nothing done, for steel or one already off.
pub fn tear_off(g: &mut crate::game::Game, d: DoorId, from: V3, who: f64, k: Option<f64>) -> bool {
    let k = k.unwrap_or(1.0);
    if stays(g, d) {
        return false;
    }
    let s = stands(g, d);
    let [hx, hz] = g.dr[d].def.hinge;
    let face = if s.u[0] * (from[2] - hz) - s.u[1] * (from[0] - hx) >= 0.0 { 1.0 } else { -1.0 };
    let fly: f64 = -face;
    frame_lets_go(g, d, who);
    let pos = crate::doors::door_pos(g, d);
    crate::doors::doors_sound(g, d, crate::doors::DoorSound::Kick, pos, who, 1.0);
    let ahead = room_ahead(g, d, &s, fly);
    let back = ahead < g.dr[d].height + g.k.doorblast.room;
    let speed = g.k.doorblast.speed;
    let speed = v3::lerp(speed[0], speed[1], v3::clamp(k, 0.0, 1.0));
    // (no room to fly: pushed at what is in front of it, and over the other way)
    let slab = if back { upright(g, d, &s, s.out(fly), speed * 0.7, -fly) } else { flies_out(g, d, &s, fly, from, speed) };
    leaf_is_off(g, d, fly);
    g.dr[d].slab = Some(crate::doors::Slab { who, ang: g.dr[d].leaf.angle, ..slab });
    hands_off(g, d);
    let said = if back { format!("no room ahead ({} m): falls back", js::to_fixed(ahead, 2)) } else { format!("{} m ahead", js::to_fixed(ahead, 2)) };
    let line = format!("{} off its hinges · k {} · {}", g.dr[d].def.name, js::to_fixed(k, 2), said);
    note(g, &line);
    pose_slab(g, d);
    crate::doors::doors_torn(g, d, who, None);
    true
}

/// How far a leaf could go that way before it met something (its own leaf, which is coming off, is no wall ahead of it).
fn room_ahead(g: &Game, d: DoorId, s: &Stands, fly: f64) -> f64 {
    let ([ux, uz], [cx, cz], [nx, nz], w) = (s.u, s.c, s.out(fly), g.dr[d].def.width);
    let others = crate::ask::Over::but_door(g, d);
    let mut ahead: f64 = 9.0;
    for f in [0.0, -0.4, 0.4] {
        if let Some(h) = crate::scene_collision::raycast_over(g, &others, [cx + ux * w * f + nx * 0.06, 0.2, cz + uz * w * f + nz * 0.06], [nx, 0.0, nz], 9.0) {
            ahead = m::min(ahead, h.t + 0.06);
        }
    }
    ahead
}

/// Out of its frame flat, a little away from the blast, turning and going over.
fn flies_out(g: &Game, d: DoorId, s: &Stands, fly: f64, from: V3, speed: f64) -> crate::doors::Slab {
    let [mut dx, mut dz] = s.out(fly);
    let (rx, rz) = (s.c[0] - from[0], s.c[1] - from[2]);
    let rl = m::hyp2(rx, rz);
    if rl > 0.05 {
        dx += 0.35 * rx / rl;
        dz += 0.35 * rz / rl;
        let l = m::hyp2(dx, dz);
        dx /= l;
        dz /= l;
    }
    crate::doors::Slab { spin: fly * g.k.doorblast.spin, tip_v: g.k.doorblast.tip, ..upright(g, d, s, [dx, dz], speed, fly) }
}

/// An upright slab leaving its frame the way `[dx, dz]`, to go over on `side` and not yet doing so. Whose it is and the angle it leaves its door at are the caller's.
fn upright(g: &Game, d: DoorId, s: &Stands, [dx, dz]: [f64; 2], speed: f64, side: f64) -> crate::doors::Slab {
    crate::doors::Slab {
        p: [s.c[0], support(g, d, 0.0), s.c[1]],
        v: [dx * speed, g.k.doorblast.up, dz * speed],
        a: s.a,
        spin: 0.0,
        tip: 0.0,
        tip_v: 0.0,
        side,
        rest: false,
        landed: false,
        who: 0.0,
        t: 0.0,
        ang: 0.0,
        bits: None,
        wedged: 0.0,
    }
}

/// What a round chips off a leaf.
fn chips(who: f64, albedo: V3, speed: f64, spread: f64, life: f64) -> crate::render_particles::EmitOpts {
    crate::render_particles::EmitOpts {
        speed,
        speed_jit: Some(0.7),
        spread: Some(spread),
        life,
        life_jit: Some(0.3),
        size: 0.006,
        size_jit: Some(0.6),
        aspect: Some(3.0),
        color: Some(v3::copy(albedo)),
        owner: Some(who),
        ..Default::default()
    }
}

/// A burst of debris particles, off `at` the way `toward`.
fn debris_burst(g: &mut Game, at: V3, toward: V3, n: f64, o: crate::render_particles::EmitOpts) {
    crate::render_particles::particles_emit(g, crate::render_particles::PKind::Debris, at, v3::normalize(toward), n, o);
}

/// The rounds' work a leaf stands: DOORSHOT.toll times plasterboard's.
fn leaf_tough(g: &crate::game::Game) -> f64 {
    let drywall = g.k.walls.tough.get(String::from("drywall")).copied().unwrap_or(6.0);
    let wood = g.k.walls.tough.get(String::from("wood")).copied().unwrap_or(9.0);
    m::min(drywall, wood) * m::max(0.1, g.k.doorshot.toll)
}

/// A round met a leaf (dynamic box `index − 1e6`), worth `w` of a wall cell's toll.
pub fn leaf_round(g: &mut crate::game::Game, index: f64, who: f64, hit_p: V3, dir: V3, w: f64) {
    let i = index - 1e6;
    if !(i >= 0.0) || (i as usize) as f64 != i || i as usize >= g.col.dynamic_boxes.len() {
        return;
    }
    let b = g.col.dynamic_boxes[i as usize];
    let d = match g.doors.list.iter().copied().find(|x| g.dr[*x].box_ == b) {
        Some(d) => d,
        None => return,
    };
    if stays(g, d) {
        return;
    }
    let was = g.dr[d].leaf.shot;
    let chaos = crate::destruction::chaos_of(g);
    g.dr[d].leaf.shot = was + w / (2.0 * m::max(1e-3, chaos));
    let share = g.dr[d].leaf.shot / leaf_tough(g);
    if share >= 1.0 {
        splinter(g, d, dir, who, Some(hit_p));
        return;
    }
    if share > 0.5 {
        // past half its toll: every round takes splinters out of its far face
        let way = v3::normalize(dir);
        let through = g.dr[d].thick + 0.02;
        let o = chips(who, g.tab.boxes[b].albedo, g.k.doorshot.chip_v[1], 0.9, 16.0);
        let n = m::round(g.k.doorshot.shed * m::min(1.0, share));
        debris_burst(g, [hit_p[0] + way[0] * through, hit_p[1], hit_p[2] + way[2] * through], dir, n, o);
    }
}

/// Once a frame: every loose slab still moving is stepped and posed, in the order of their doors.
pub fn door_slabs(g: &mut crate::game::Game, dt: f64) {
    for at in 0..g.doors.list.len() {
        let d = g.doors.list[at];
        if g.dr[d].slab.is_some_and(|s| !s.rest) && g.dr[d].leaf.off {
            fly_slab(g, d, dt);
            pose_slab(g, d);
        }
    }
}

/// One slab's step, and what it met said in one place: the tallies, the slam for the floor to hear, the log line.
fn fly_slab(g: &mut crate::game::Game, d: DoorId, dt: f64) {
    let box_ = g.dr[d].box_;
    let but = g.col.dynamic_boxes.iter().position(|x| *x == box_).map_or(-1, |i| i as i32);
    let door = &mut g.dr[d];
    let flight = crate::slab::Flight {
        w: g.tab.boxes.world(),
        height: door.height,
        thick: door.thick,
        hu: door.leaf_h[0],
        closed: door.def.closed_dir,
        fcx: door.frame_centre[0],
        fcz: door.frame_centre[2],
        but,
        tune: &g.k,
        k: &g.k.doorblast,
    };
    let s = door.slab.as_mut().expect("a slab in the air");
    let met = crate::slab::step(&flight, s, dt);
    let s = *s;
    if met.landed {
        let at: V3 = [met.lx, 0.1, met.lz];
        let level = g.k.doorblast.land_level;
        let who = s.who;
        crate::doors::doors_sound(g, d, crate::doors::DoorSound::Bang, at, who, level);
    }
    if met.rested {
        let line = format!("{} slab at rest ({}, {}) after {} s", g.dr[d].def.name, js::to_fixed(s.p[0], 2), js::to_fixed(s.p[2], 2), js::to_fixed(s.t, 2));
        note(g, &line);
    }
}

/// The slab's box (and the two handles on it) where the slab is: tilted while it topples, then an ordinary yaw-only box lying flat.
fn pose_slab(g: &mut crate::game::Game, d: DoorId) {
    let Some(s) = g.dr[d].slab else { return };
    let b = g.dr[d].box_;
    let (w, t_, height) = (g.dr[d].def.width, g.dr[d].thick, g.dr[d].height);
    let (ux, uz) = (m::cos(s.a), m::sin(s.a));
    let [lx, ly] = handle_on(w, height);
    let lz = t_ / 2.0 + 0.03;
    let handles = g.dr[d].handle_boxes.clone();
    let seal = crate::scene_boxes::box_flag().sound_seal;
    {
        let mut row = g.tab.boxes.edit(b);
        row.flags = (js::to_i32(row.flags) & !js::to_i32(seal)) as f64;
        row.yaw = -s.a;
    }
    if s.landed {
        let hw = match s.bits { Some(bits) => bits[0], None => g.dr[d].leaf_h[0] };
        let hl = match s.bits { Some(bits) => bits[1], None => height / 2.0 };
        let mut leaf = g.tab.boxes.edit(b);
        leaf.rot = None;
        leaf.marks = None;
        leaf.h = [hw, t_ / 2.0, hl];
        leaf.c = [s.p[0], t_ / 2.0 + LIFT, s.p[2]];
        drop(leaf);
        let (tx, tz) = (-uz * s.side, ux * s.side);
        let kx = match s.bits { Some(bits) => bits[2], None => lx };
        let ky = match s.bits { Some(bits) => bits[3], None => ly };
        for hb in handles {
            let mut handle = g.tab.boxes.edit(hb);
            handle.rot = None;
            handle.yaw = -s.a;
            handle.c = [s.p[0] + ux * kx + tx * ky, t_ + LIFT + 0.03, s.p[2] + uz * kx + tz * ky];
        }
        return;
    }
    let q = v3::quat::mul(v3::quat::axis_angle([ux, 0.0, uz], s.side * s.tip), v3::quat::yaw(-s.a));
    let c: V3 = [s.p[0], s.p[1], s.p[2]];
    let leaf_h = g.dr[d].leaf_h;
    let mut leaf = g.tab.boxes.edit(b);
    leaf.rot = Some(q);
    leaf.h = leaf_h;
    leaf.c = c;
    drop(leaf);
    for k in 0..2 {
        let z = if k == 0 { lz } else { -lz };
        let mut handle = g.tab.boxes.edit(handles[k]);
        handle.rot = Some(q);
        handle.yaw = -s.a;
        handle.c = v3::add(c, v3::quat::rotate(q, [lx, ly, z]));
    }
}
