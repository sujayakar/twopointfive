pub use ::game_types::ragdoll::types;
pub use types::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, vec::Vec};

const N: usize = RAG_COUNT as usize;
/// collision radii (m)
const RADIUS: [f64; N] = [0.12, 0.13, 0.07, 0.11, 0.07, 0.07, 0.05, 0.05, 0.05, 0.05, 0.09, 0.09, 0.065, 0.065, 0.06, 0.06];
/// masses (only the ratios matter)
const MASS: [f64; N] = [12.0, 15.0, 3.0, 5.0, 3.0, 3.0, 2.0, 2.0, 1.0, 1.0, 6.0, 6.0, 4.0, 4.0, 2.0, 2.0];
/// the rigid cluster (shape matched): pelvis, spine, neck, shoulders, hips
const TRUNK: [usize; 7] = [0, 1, 2, 4, 5, 10, 11];
const TRUNK_SET: [u8; N] = [1, 1, 1, 0, 1, 1, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0];
/// stiff sticks: neck-head, upper arms, forearms, thighs, shins
const STICKS: [(usize, usize); 9] = [(2, 3), (4, 6), (6, 8), (5, 7), (7, 9), (10, 12), (12, 14), (11, 13), (13, 15)];
/// rows of STICKS whose stretch counts as 'the body is wedged' while dragged
const LEG_STICKS: [usize; 4] = [5, 6, 7, 8];
/// folding stops: minimum shoulder-hand / hip-ankle distance (m)
const MINDIST: [(usize, usize, f64); 4] = [(4, 8, 0.16), (5, 9, 0.16), (10, 14, 0.30), (11, 15, 0.30)];
/// keep-aparts (sphere-sphere at k × the radius sum); limb first
const APART: [(usize, usize, f64); 20] = [(12, 13, 0.9), (14, 15, 0.9), (12, 15, 0.9), (13, 14, 0.9), (6, 0, 0.6), (6, 1, 0.6), (7, 0, 0.6), (7, 1, 0.6), (8, 0, 0.6), (8, 1, 0.6), (8, 2, 0.6), (8, 3, 0.7), (9, 0, 0.6), (9, 1, 0.6), (9, 2, 0.6), (9, 3, 0.7), (8, 9, 0.8), (6, 7, 0.8), (4, 3, 0.9), (5, 3, 0.9)];
/// knee direction rule over (hip, knee, ankle)
const KNEES: [(usize, usize, usize); 2] = [(10, 12, 14), (11, 13, 15)];

/// cos 55°, the game's own: the skeleton's const of this name is one unit in the last place off it (see notes, ## Skeleton), and this one shadows it.
const NECK_CONE_COS: f64 = 0.5735764363510462;

/// wd.gather(…, this.near): the collision grid's indices of the statics round the body.
fn world_gather(g: &mut Game, x0: f64, z0: f64, x1: f64, z1: f64, y0: f64, y1: f64, out: &mut Vec<i32>) -> f64 {
    crate::scene_collision::gather(g, x0, z0, x1, z1, y0, y1, out)
}

/// `b.c`, `b.h`, `b.yaw` of a box. Vec<BoxId>` and no table of Box rows ("BoxId: no table here"), while the index says `g.col.boxes[id].c`. Once the rows have a table
/// this is the row's three fields.
fn seam_box_row(g: &Game, b: BoxId) -> (V3, V3, f64) {
    let x = &g.tab.boxes[b];
    (x.c, x.h, x.yaw)
}

/// A body seeded from pts (RAG_COUNT world positions) and vel (m/s) or none.
pub(super) fn ragdoll_new(g: &mut crate::game::Game, pts: &[V3], vel: Option<&[V3]>, world: Option<&crate::ragdoll::RagdollWorld>) -> RagdollId {
    let id = g.tab.ragdoll.push(crate::ragdoll::Ragdoll::blank());
    let this = &mut g.tab.ragdoll[id];
    this.world = world.is_some();
    for i in 0..N {
        let p = pts[i];
        this.x[i * 3] = p[0]; this.x[i * 3 + 1] = m::max(p[1], RADIUS[i]); this.x[i * 3 + 2] = p[2];
        this.w[i] = 1.0 / MASS[i];
    }
    for s in 0..STICKS.len() { this.stick_len[s] = rd_dist(this, STICKS[s].0 as f64, STICKS[s].1 as f64); }
    // trunk rest shape about its mass centroid
    let (mut mass, mut cx, mut cy, mut cz) = (0.0, 0.0, 0.0, 0.0);
    for &i in TRUNK.iter() {
        let ms = MASS[i];
        mass += ms; cx += this.x[i * 3] * ms; cy += this.x[i * 3 + 1] * ms; cz += this.x[i * 3 + 2] * ms;
    }
    this.trunk_mass = mass; cx /= mass; cy /= mass; cz /= mass;
    this.prev_centre[0] = cx; this.centre[0] = cx;
    this.prev_centre[1] = cy; this.centre[1] = cy;
    this.prev_centre[2] = cz; this.centre[2] = cz;
    for (k, &i) in TRUNK.iter().enumerate() {
        this.r0[k * 3] = this.x[i * 3] - cx; this.r0[k * 3 + 1] = this.x[i * 3 + 1] - cy; this.r0[k * 3 + 2] = this.x[i * 3 + 2] - cz;
    }
    // anatomical frame: left = hips + shoulders line, up = pelvis → neck (made orthogonal), forward = left × up
    let x = &this.x;
    let (lx, ly, lz) = ((x[30] - x[33]) + (x[12] - x[15]), (x[31] - x[34]) + (x[13] - x[16]), (x[32] - x[35]) + (x[14] - x[17]));
    let ll = js::or(m::hyp3(lx, ly, lz), 1.0);
    let (sx, sy, sz) = (lx / ll, ly / ll, lz / ll);
    let (mut ux, mut uy, mut uz) = (x[6] - x[0], x[7] - x[1], x[8] - x[2]);
    let d = ux * sx + uy * sy + uz * sz;
    ux -= sx * d; uy -= sy * d; uz -= sz * d;
    let ul = js::or(m::hyp3(ux, uy, uz), 1.0);
    ux /= ul; uy /= ul; uz /= ul;
    this.up0 = [ux, uy, uz];
    this.fwd0 = [sy * uz - sz * uy, sz * ux - sx * uz, sx * uy - sy * ux];
    let (ca, sa) = (m::cos(47.5 * m::PI / 180.0), m::sin(47.5 * m::PI / 180.0));
    this.hip_axis0 = [-ux * ca + this.fwd0[0] * sa, -uy * ca + this.fwd0[1] * sa, -uz * ca + this.fwd0[2] * sa];
    // a hand or an elbow posed inside a wall at death is walked back toward the pelvis until clear
    ragdoll_gather_near(g, id);
    let this = &mut g.tab.ragdoll[id];
    for i in 1..N {
        let mut k = 0;
        while k < 12 && rd_buried(this, i as f64) {
            let x = &mut this.x;
            let (dx, dy, dz) = (x[0] - x[i * 3], x[1] - x[i * 3 + 1], x[2] - x[i * 3 + 2]);
            let l = js::or(m::hyp3(dx, dy, dz), 1.0);
            x[i * 3] += dx / l * 0.03; x[i * 3 + 1] += dy / l * 0.03; x[i * 3 + 2] += dz / l * 0.03;
            k += 1;
        }
    }
    for i in 0..N * 3 { this.px[i] = this.x[i]; }
    if let Some(vel) = vel {
        for i in 0..N {
            let v = vel[i];
            this.px[i * 3] -= v[0] * H; this.px[i * 3 + 1] -= v[1] * H; this.px[i * 3 + 2] -= v[2] * H;
        }
    }
    this.snap.copy_from_slice(&this.x);
    id
}

/// (Ragdoll.point)
pub fn ragdoll_point(g: &crate::game::Game, id: RagdollId, i: f64, out: Option<&mut V3>) -> V3 { rd_point(&g.tab.ragdoll[id], i, out) }
/// (Ragdoll.radius)
pub fn ragdoll_radius(g: &crate::game::Game, id: RagdollId, i: f64) -> f64 { rd_radius(&g.tab.ragdoll[id], i) }
/// (Ragdoll.addVelocity)
pub(super) fn ragdoll_add_velocity(g: &mut crate::game::Game, id: RagdollId, i: f64, v: V3, scale: Option<f64>) { rd_add_velocity(&mut g.tab.ragdoll[id], i, v, scale) }
/// (Ragdoll.addRigidVelocity)
pub fn ragdoll_add_rigid_velocity(g: &mut crate::game::Game, id: RagdollId, v: V3, omega: V3) { rd_add_rigid_velocity(&mut g.tab.ragdoll[id], v, omega) }
/// (Ragdoll.nudge)
pub fn ragdoll_nudge(g: &mut crate::game::Game, id: RagdollId, i: f64, dx: f64, dz: f64) { rd_nudge(&mut g.tab.ragdoll[id], i, dx, dz) }
/// (Ragdoll.wake)
pub fn ragdoll_wake(g: &mut crate::game::Game, id: RagdollId) { rd_wake(&mut g.tab.ragdoll[id]) }
/// (Ragdoll.setPin)
pub fn ragdoll_set_pin(g: &mut crate::game::Game, id: RagdollId, i: f64, target: V3) { rd_set_pin(&mut g.tab.ragdoll[id], i, target) }
/// (Ragdoll.unpin)
pub(super) fn ragdoll_unpin(g: &mut crate::game::Game, id: RagdollId, i: f64) { rd_unpin(&mut g.tab.ragdoll[id], i) }
/// (Ragdoll.isPinned)
pub(super) fn ragdoll_is_pinned(g: &crate::game::Game, id: RagdollId, i: f64) -> bool { rd_is_pinned(&g.tab.ragdoll[id], i) }
/// (Ragdoll.clearPins)
pub fn ragdoll_clear_pins(g: &mut crate::game::Game, id: RagdollId) { rd_clear_pins(&mut g.tab.ragdoll[id]) }
/// (Ragdoll.ride)
pub fn ragdoll_ride(g: &mut crate::game::Game, id: RagdollId, about: V3, dx: f64, dz: f64, dyaw: f64) { rd_ride(&mut g.tab.ragdoll[id], about, dx, dz, dyaw) }
/// (Ragdoll.takeImpact)
pub fn ragdoll_take_impact(g: &mut crate::game::Game, id: RagdollId, min: Option<f64>, gap: Option<f64>) -> f64 { rd_take_impact(&mut g.tab.ragdoll[id], min, gap) }
/// Distance between two particles.
fn rd_dist(this: &crate::ragdoll::Ragdoll, i: f64, j: f64) -> f64 {
    let (i, j) = (i as usize, j as usize);
    let x = &this.x;
    m::hyp3(x[j * 3] - x[i * 3], x[j * 3 + 1] - x[i * 3 + 1], x[j * 3 + 2] - x[i * 3 + 2])
}

/// World position of particle i (into `out` if given, and returned).
fn rd_point(this: &crate::ragdoll::Ragdoll, i: f64, out: Option<&mut V3>) -> V3 {
    let i = i as usize;
    let p = [this.x[i * 3], this.x[i * 3 + 1], this.x[i * 3 + 2]];
    if let Some(o) = out { *o = p; }
    p
}

/// Collision radius of particle i.
fn rd_radius(this: &crate::ragdoll::Ragdoll, i: f64) -> f64 {
    RADIUS[i as usize]
}

/// Add velocity (m/s) to one particle. Wakes the body.
fn rd_add_velocity(this: &mut crate::ragdoll::Ragdoll, i: f64, v: V3, scale: Option<f64>) {
    let i = i as usize;
    let scale = scale.unwrap_or(1.0);
    this.px[i * 3] -= v[0] * scale * H; this.px[i * 3 + 1] -= v[1] * scale * H; this.px[i * 3 + 2] -= v[2] * scale * H;
    rd_wake(this);
    this.awake = 0.0;
}

/// Add a rigid velocity field to the whole body: v + ω × (p − trunk centre).
fn rd_add_rigid_velocity(this: &mut crate::ragdoll::Ragdoll, v: V3, omega: V3) {
    let c = this.centre;
    for i in 0..N {
        let o = i * 3;
        let (rx, ry, rz) = (this.x[o] - c[0], this.x[o + 1] - c[1], this.x[o + 2] - c[2]);
        this.px[o] -= (v[0] + omega[1] * rz - omega[2] * ry) * H;
        this.px[o + 1] -= (v[1] + omega[2] * rx - omega[0] * rz) * H;
        this.px[o + 2] -= (v[2] + omega[0] * ry - omega[1] * rx) * H;
    }
    rd_wake(this);
    this.awake = 0.0;
}

/// Displace one particle in the ground plane; wakes it if it actually moved.
fn rd_nudge(this: &mut crate::ragdoll::Ragdoll, i: f64, dx: f64, dz: f64) {
    if m::abs(dx) + m::abs(dz) < 1e-4 { return; }
    let i = i as usize;
    this.x[i * 3] += dx; this.x[i * 3 + 2] += dz; this.px[i * 3] += dx; this.px[i * 3 + 2] += dz;
    rd_wake(this);
}

/// Simulate again from now; does not reset the MAX_AWAKE budget.
fn rd_wake(this: &mut crate::ragdoll::Ragdoll) {
    if !this.sleeping { return; }
    this.sleeping = false;
    this.snap_t = 0.0;
    this.snap.copy_from_slice(&this.x);
}

/// Hold particle i at `target` (moved there kinematically, ≤ PIN_STEP per substep).
fn rd_set_pin(this: &mut crate::ragdoll::Ragdoll, i: f64, target: V3) {
    let i = i as usize;
    this.pin_t[i * 3] = target[0]; this.pin_t[i * 3 + 1] = m::max(target[1], RADIUS[i]); this.pin_t[i * 3 + 2] = target[2];
    if this.pinned[i] == 0 { this.pinned[i] = 1; this.w[i] = 0.0; this.any_pin = true; }
    rd_wake(this);
}

/// Let ONE pinned particle go.
fn rd_unpin(this: &mut crate::ragdoll::Ragdoll, i: f64) {
    let i = i as usize;
    if this.pinned[i] == 0 { return; }
    this.pinned[i] = 0;
    this.w[i] = 1.0 / MASS[i];
    let mut any = false;
    for k in 0..N { if this.pinned[k] != 0 { any = true; break; } }
    if !any { this.any_pin = false; this.strain = 0.0; this.ride_owed = false; }
    rd_wake(this);
}

/// Is this particle pinned.
fn rd_is_pinned(this: &crate::ragdoll::Ragdoll, i: f64) -> bool {
    this.pinned[i as usize] != 0
}

/// Let every pin go.
fn rd_clear_pins(this: &mut crate::ragdoll::Ragdoll) {
    if !this.any_pin { return; }
    for i in 0..N { this.pinned[i] = 0; this.w[i] = 1.0 / MASS[i]; }
    this.any_pin = false;
    this.strain = 0.0;
    this.ride_owed = false;
}

/// A CARRIED body rides its carrier: over the next step the free particles take his step and turn.
fn rd_ride(this: &mut crate::ragdoll::Ragdoll, about: V3, dx: f64, dz: f64, dyaw: f64) {
    if this.ride_owed {
        let k = m::min(1.0, MAX_STEP / m::max(1e-9, m::hyp2(this.ride_v[2], this.ride_v[3]) + m::abs(this.ride_v[4])));
        rd_ride_by(this, k);
        this.ride_owed = false;
    }
    let r = &mut this.ride_v;
    r[0] = about[0]; r[1] = about[2]; r[2] = dx; r[3] = dz; r[4] = dyaw;
    this.ride_owed = true;
}

/// Take the free particles through share `k` of what is owed.
fn rd_ride_by(this: &mut crate::ragdoll::Ragdoll, k: f64) {
    let (ax, az, sx, sz, co, si) = (this.ride_v[0], this.ride_v[1], this.ride_v[2] * k, this.ride_v[3] * k, m::cos(this.ride_v[4] * k), m::sin(this.ride_v[4] * k));
    let (x, px) = (&mut this.x, &mut this.px);
    for i in 0..N {
        if this.pinned[i] != 0 { continue; }
        let o = i * 3;
        let (mut ux, mut uz) = (x[o] - ax, x[o + 2] - az);
        x[o] = ax + ux * co + uz * si + sx; x[o + 2] = az - ux * si + uz * co + sz;
        ux = px[o] - ax; uz = px[o + 2] - az;
        px[o] = ax + ux * co + uz * si + sx; px[o + 2] = az - ux * si + uz * co + sz;
    }
    this.ride_v[0] += sx;
    this.ride_v[1] += sz;
}

/// Hardest trunk / head landing worth hearing since the last one reported; 0 otherwise.
fn rd_take_impact(this: &mut crate::ragdoll::Ragdoll, min: Option<f64>, gap: Option<f64>) -> f64 {
    let (min, gap) = (min.unwrap_or(1.4), gap.unwrap_or(0.25));
    let v = this.impact_v;
    this.impact_v = 0.0;
    if v < min || this.since_impact < gap { return 0.0; }
    this.since_impact = 0.0;
    v
}

/// Advance by dt (s): fixed 180 Hz substeps from an accumulator, at most MAX_SUBSTEPS a call.
pub(super) fn ragdoll_step(g: &mut crate::game::Game, id: RagdollId, dt: f64) {
    let this = &mut g.tab.ragdoll[id];
    this.since_impact += dt;
    if this.sleeping { return; }
    this.acc = m::min(this.acc + dt, H * MAX_SUBSTEPS);
    // the carrier's step and turn this frame, shared over the substeps it gets, a share never above MAX_STEP at a limb's end
    let mut n = if this.ride_owed { m::floor((this.acc + 1e-9) / H) } else { 0.0 };
    let share = if js::truthy(n) { m::min(1.0 / n, MAX_STEP / m::max(1e-9, m::hyp2(this.ride_v[2], this.ride_v[3]) + m::abs(this.ride_v[4]))) } else { 0.0 };
    while g.tab.ragdoll[id].acc >= H - 1e-9 {
        let this = &mut g.tab.ragdoll[id];
        this.acc -= H;
        if n > 0.0 { rd_ride_by(this, share); n -= 1.0; }
        ragdoll_substep(g, id);
        let this = &mut g.tab.ragdoll[id];
        if this.sleeping { this.acc = 0.0; break; }
    }
    if js::truthy(share) { g.tab.ragdoll[id].ride_owed = false; }
}

/// One 180 Hz substep: integrate, gather, project, contacts, sleep.
fn ragdoll_substep(g: &mut crate::game::Game, id: RagdollId) {
    rd_integrate(&mut g.tab.ragdoll[id]);
    // 2. what is around the body this substep
    ragdoll_gather_near(g, id);
    rd_solve(&mut g.tab.ragdoll[id]);
}

/// The substep's first step.
fn rd_integrate(this: &mut crate::ragdoll::Ragdoll) {
    // 1. integrate (Verlet), gravity, damping, travel cap; pinned particles travel toward their target, carrying no inertia
    let (c, pc) = (this.centre, this.prev_centre);
    let trunk_v = m::hyp3(c[0] - pc[0], c[1] - pc[1], c[2] - pc[2]) / H;
    this.prev_centre = c;
    let damp = if this.any_pin { DAMP_PINNED } else { DAMP };
    let damp_limb = if this.any_pin { DAMP_PINNED } else if trunk_v < SETTLE_V { DAMP_SETTLE } else { DAMP };
    let grav = GRAV * H * H;
    {
        let (x, px) = (&mut this.x, &mut this.px);
        for i in 0..N {
            let o = i * 3;
            if this.pinned[i] != 0 {
                let pin_t = &this.pin_t;
                let (mut dx, mut dy, mut dz) = (pin_t[o] - x[o], pin_t[o + 1] - x[o + 1], pin_t[o + 2] - x[o + 2]);
                let l = m::hyp3(dx, dy, dz);
                if l > PIN_STEP { let s = PIN_STEP / l; dx *= s; dy *= s; dz *= s; }
                x[o] += dx; x[o + 1] += dy; x[o + 2] += dz;
                px[o] = x[o]; px[o + 1] = x[o + 1]; px[o + 2] = x[o + 2];
                continue;
            }
            let d = if TRUNK_SET[i] != 0 { damp } else { damp_limb };
            let (mut vx, mut vy, mut vz) = ((x[o] - px[o]) * d, (x[o + 1] - px[o + 1]) * d - grav, (x[o + 2] - px[o + 2]) * d);
            let l = m::hyp3(vx, vy, vz);
            if l > MAX_STEP { let s = MAX_STEP / l; vx *= s; vy *= s; vz *= s; }
            px[o] = x[o]; px[o + 1] = x[o + 1]; px[o + 2] = x[o + 2];
            x[o] += vx; x[o + 1] += vy; x[o + 2] += vz;
        }
    }
}

/// The substep after the gather: constraints, contacts, sleep.
fn rd_solve(this: &mut crate::ragdoll::Ragdoll) {
    this.pen.fill(0.0);
    this.cn.fill(0.0);
    // 3. project constraints, collisions last so nothing ends an iteration inside geometry
    for _ in 0..ITERS as usize {
        for s in 0..STICKS.len() {
            let len = this.stick_len[s];
            rd_distance(this, STICKS[s].0 as f64, STICKS[s].1 as f64, len, len);
        }
        for s in 0..MINDIST.len() { rd_distance(this, MINDIST[s].0 as f64, MINDIST[s].1 as f64, MINDIST[s].2, 1e9); }
        for s in 0..APART.len() {
            let (a, b, k) = APART[s];
            rd_apart(this, a as f64, b as f64, (RADIUS[a] + RADIUS[b]) * k);
        }
        let (hip_axis, up) = (this.hip_axis0, this.up0);
        rd_cone(this, 10.0, 12.0, hip_axis, HIP_CONE_COS);
        rd_cone(this, 11.0, 13.0, hip_axis, HIP_CONE_COS);
        rd_cone(this, 2.0, 3.0, up, NECK_CONE_COS);
        for s in 0..KNEES.len() { rd_knee(this, KNEES[s].0 as f64, KNEES[s].1 as f64, KNEES[s].2 as f64); }
        rd_shape_match(this);
        for i in 0..N { rd_collide(this, i as f64); }
    }
    // 4. contacts: kill the velocity still heading into the surface, Coulomb friction on the slide, note trunk / head landings
    for i in 0..N {
        let pen = this.pen[i];
        if this.pinned[i] != 0 || pen <= 0.0 { continue; }
        let o = i * 3;
        let (x, px) = (&this.x, &mut this.px);
        let (mut vx, mut vy, mut vz) = (x[o] - px[o], x[o + 1] - px[o + 1], x[o + 2] - px[o + 2]);
        let (mut nx, mut ny, mut nz) = (this.cn[o], this.cn[o + 1], this.cn[o + 2]);
        let nl = m::hyp3(nx, ny, nz);
        if nl < 1e-9 { continue; }
        nx /= nl; ny /= nl; nz /= nl;
        let mut vn = vx * nx + vy * ny + vz * nz;
        if i <= 3 {
            let hit = m::max(-vn, 0.0) / H;
            if hit > this.impact_v { this.impact_v = hit; }
        }
        if vn < 0.0 { vx -= nx * vn; vy -= ny * vn; vz -= nz * vn; vn = 0.0; }
        let (tx, ty, tz) = (vx - nx * vn, vy - ny * vn, vz - nz * vn);
        let tl = m::hyp3(tx, ty, tz);
        if tl > 1e-12 {
            let keep = m::max(0.0, 1.0 - MU * pen / tl);
            vx = nx * vn + tx * keep; vy = ny * vn + ty * keep; vz = nz * vn + tz * keep;
        }
        px[o] = x[o] - vx; px[o + 1] = x[o + 1] - vy; px[o + 2] = x[o + 2] - vz;
    }
    // 5. how stretched the legs are; sleep once nothing has really moved for a while (or it has been at it far too long)
    let mut strain = 0.0;
    if this.any_pin {
        for &s in LEG_STICKS.iter() {
            let d = rd_dist(this, STICKS[s].0 as f64, STICKS[s].1 as f64);
            strain = m::max(strain, m::abs(d - this.stick_len[s]));
        }
    }
    this.strain = strain;
    if this.any_pin { this.awake = 0.0; this.snap_t = 0.0; return; }
    this.awake += H;
    this.snap_t += H;
    if this.snap_t >= SLEEP_WINDOW {
        let mut still = true;
        let mut i = 0;
        while i < N * 3 {
            let (x, sx) = (&this.x, &this.snap);
            if m::abs(x[i] - sx[i]) + m::abs(x[i + 1] - sx[i + 1]) + m::abs(x[i + 2] - sx[i + 2]) > SLEEP_DISP { still = false; break; }
            i += 3;
        }
        if still || this.awake > MAX_AWAKE {
            this.sleeping = true;
            for i in 0..N * 3 { this.px[i] = this.x[i]; }
        }
        this.snap.copy_from_slice(&this.x);
        this.snap_t = 0.0;
    }
}

/// Clamp |xj − xi| into [lo, hi], corrections split by inverse mass.
fn rd_distance(this: &mut crate::ragdoll::Ragdoll, i: f64, j: f64, lo: f64, hi: f64) {
    let (i, j) = (i as usize, j as usize);
    let (a, b) = (i * 3, j * 3);
    let x = &mut this.x;
    let (dx, dy, dz) = (x[b] - x[a], x[b + 1] - x[a + 1], x[b + 2] - x[a + 2]);
    let l = m::hyp3(dx, dy, dz);
    if l < 1e-9 { return; }
    let target = if l < lo { lo } else if l > hi { hi } else { l };
    if target == l { return; }
    let (wi, wj) = (this.w[i], this.w[j]);
    let ws = wi + wj;
    if ws <= 0.0 { return; }
    let k = (l - target) / (l * ws);
    x[a] += dx * k * wi; x[a + 1] += dy * k * wi; x[a + 2] += dz * k * wi;
    x[b] -= dx * k * wj; x[b + 1] -= dy * k * wj; x[b + 2] -= dz * k * wj;
}

/// Sphere-sphere keep-apart of limb particle i off particle j, logged as a contact on the limb.
fn rd_apart(this: &mut crate::ragdoll::Ragdoll, i: f64, j: f64, lo: f64) {
    let (iu, ju) = (i as usize, j as usize);
    let (a, b) = (iu * 3, ju * 3);
    let x = &mut this.x;
    let (dx, dy, dz) = (x[a] - x[b], x[a + 1] - x[b + 1], x[a + 2] - x[b + 2]);
    let l = m::hyp3(dx, dy, dz);
    if l >= lo || l < 1e-9 { return; }
    let (wi, wj) = (this.w[iu], this.w[ju]);
    let ws = wi + wj;
    if ws <= 0.0 { return; }
    let k = (lo - l) / (l * ws);
    x[a] += dx * k * wi; x[a + 1] += dy * k * wi; x[a + 2] += dz * k * wi;
    x[b] -= dx * k * wj; x[b + 1] -= dy * k * wj; x[b + 2] -= dz * k * wj;
    rd_contact(this, i, dx / l, dy / l, dz / l, (lo - l) * wi / ws);
}

/// Keep the direction i→j within acos(cosMax) of a trunk-relative axis.
fn rd_cone(this: &mut crate::ragdoll::Ragdoll, i: f64, j: f64, axis0: V3, cos_max: f64) {
    let (i, j) = (i as usize, j as usize);
    let (a, b) = (i * 3, j * 3);
    let (x, rr) = (&mut this.x, &this.r);
    let (ax, ay, az) = (
        rr[0] * axis0[0] + rr[1] * axis0[1] + rr[2] * axis0[2],
        rr[3] * axis0[0] + rr[4] * axis0[1] + rr[5] * axis0[2],
        rr[6] * axis0[0] + rr[7] * axis0[1] + rr[8] * axis0[2],
    );
    let (dx, dy, dz) = (x[b] - x[a], x[b + 1] - x[a + 1], x[b + 2] - x[a + 2]);
    let l = m::hyp3(dx, dy, dz);
    if l < 1e-6 { return; }
    let (ux, uy, uz) = (dx / l, dy / l, dz / l);
    let c = ux * ax + uy * ay + uz * az;
    if c >= cos_max { return; }
    let (mut qx, mut qy, mut qz) = (ux - ax * c, uy - ay * c, uz - az * c);
    let ql = m::hyp3(qx, qy, qz);
    if ql < 1e-6 { return; }
    let sin_max = m::sqrt(1.0 - cos_max * cos_max);
    qx *= sin_max / ql; qy *= sin_max / ql; qz *= sin_max / ql;
    // nearest direction on the cone, same length; move j there (and i back) by inverse mass
    let (ex, ey, ez) = ((ax * cos_max + qx) * l - dx, (ay * cos_max + qy) * l - dy, (az * cos_max + qz) * l - dz);
    let (wi, wj) = (this.w[i], this.w[j]);
    let ws = wi + wj;
    if ws <= 0.0 { return; }
    x[b] += ex * wj / ws; x[b + 1] += ey * wj / ws; x[b + 2] += ez * wj / ws;
    x[a] -= ex * wi / ws; x[a + 1] -= ey * wi / ws; x[a + 2] -= ez * wi / ws;
}

/// Knee k must lie on the trunk's forward side of the hip-ankle line, projected with inverse-mass weights.
fn rd_knee(this: &mut crate::ragdoll::Ragdoll, hi: f64, ki: f64, ai: f64) {
    let (hi, ki, ai) = (hi as usize, ki as usize, ai as usize);
    let (a, b, c) = (hi * 3, ki * 3, ai * 3);
    let (x, rr, f0) = (&mut this.x, &this.r, this.fwd0);
    let (fx, fy, fz) = (
        rr[0] * f0[0] + rr[1] * f0[1] + rr[2] * f0[2],
        rr[3] * f0[0] + rr[4] * f0[1] + rr[5] * f0[2],
        rr[6] * f0[0] + rr[7] * f0[1] + rr[8] * f0[2],
    );
    let (ux, uy, uz) = (x[c] - x[a], x[c + 1] - x[a + 1], x[c + 2] - x[a + 2]);
    let l2 = ux * ux + uy * uy + uz * uz;
    if l2 < 1e-8 { return; }
    let mut t = ((x[b] - x[a]) * ux + (x[b + 1] - x[a + 1]) * uy + (x[b + 2] - x[a + 2]) * uz) / l2;
    t = if t < 0.0 { 0.0 } else if t > 1.0 { 1.0 } else { t };
    let cc = (x[b] - x[a] - ux * t) * fx + (x[b + 1] - x[a + 1] - uy * t) * fy + (x[b + 2] - x[a + 2] - uz * t) * fz;
    if cc >= 0.0 { return; }
    let (wh, wk, wa) = (this.w[hi], this.w[ki], this.w[ai]);
    let den = wk + wh * (1.0 - t) * (1.0 - t) + wa * t * t;
    if den <= 0.0 { return; }
    let s = -cc / den;
    let (dk, dh, da) = (wk * s, -wh * (1.0 - t) * s, -wa * t * s);
    x[b] += fx * dk; x[b + 1] += fy * dk; x[b + 2] += fz * dk;
    x[a] += fx * dh; x[a + 1] += fy * dh; x[a + 2] += fz * dh;
    x[c] += fx * da; x[c + 1] += fy * da; x[c + 2] += fz * da;
}

/// Rigid trunk: best-fit rotation of the cluster against its bind shape, then every member snapped to centroid + R·rest offset.
fn rd_shape_match(this: &mut crate::ragdoll::Ragdoll) {
    let mass = this.trunk_mass;
    let (mut cx, mut cy, mut cz) = (0.0, 0.0, 0.0);
    for k in 0..TRUNK.len() {
        let (o, ms) = (TRUNK[k] * 3, MASS[TRUNK[k]]);
        cx += this.x[o] * ms; cy += this.x[o + 1] * ms; cz += this.x[o + 2] * ms;
    }
    cx /= mass; cy /= mass; cz /= mass;
    this.centre[0] = cx; this.centre[1] = cy; this.centre[2] = cz;
    this.a.fill(0.0);
    for k in 0..TRUNK.len() {
        let (o, q, ms) = (TRUNK[k] * 3, k * 3, MASS[TRUNK[k]]);
        let (px, py, pz) = ((this.x[o] - cx) * ms, (this.x[o + 1] - cy) * ms, (this.x[o + 2] - cz) * ms);
        let (aa, r0) = (&mut this.a, &this.r0);
        aa[0] += px * r0[q]; aa[1] += px * r0[q + 1]; aa[2] += px * r0[q + 2];
        aa[3] += py * r0[q]; aa[4] += py * r0[q + 1]; aa[5] += py * r0[q + 2];
        aa[6] += pz * r0[q]; aa[7] += pz * r0[q + 1]; aa[8] += pz * r0[q + 2];
    }
    extract_rotation(&this.a, &mut this.rot, 10.0);
    quat_to_mat(this.rot, &mut this.r);
    for k in 0..TRUNK.len() {
        let (o, q) = (TRUNK[k] * 3, k * 3);
        let (rx, ry, rz) = (this.r0[q], this.r0[q + 1], this.r0[q + 2]);
        let (x, rr) = (&mut this.x, &this.r);
        x[o] = cx + rr[0] * rx + rr[1] * ry + rr[2] * rz;
        x[o + 1] = cy + rr[3] * rx + rr[4] * ry + rr[5] * rz;
        x[o + 2] = cz + rr[6] * rx + rr[7] * ry + rr[8] * rz;
    }
}

/// Statics whose footprint overlaps the body's box (± a margin) and door leaves within reach, with their yaw cos/sin.
fn ragdoll_gather_near(g: &mut crate::game::Game, id: RagdollId) {
    let this = &mut g.tab.ragdoll[id];
    this.n_near = 0.0;
    this.n_dyn = 0.0;
    if !this.world { return; }
    let (mut x0, mut x1, mut y1, mut z0, mut z1) = (1e9, -1e9, -1e9, 1e9, -1e9);
    for i in 0..N {
        let o = i * 3;
        let x = &this.x;
        if x[o] < x0 { x0 = x[o]; }
        if x[o] > x1 { x1 = x[o]; }
        if x[o + 1] > y1 { y1 = x[o + 1]; }
        if x[o + 2] < z0 { z0 = x[o + 2]; }
        if x[o + 2] > z1 { z1 = x[o + 2]; }
    }
    let mg = 0.25;
    x0 -= mg; x1 += mg; z0 -= mg; z1 += mg; y1 += mg;
    // (the list is lent to the grid: the row cannot be held across a call that takes g)
    let mut near = core::mem::take(&mut this.near);
    let n = world_gather(g, x0, z0, x1, z1, Y_MIN, y1, &mut near) as usize;
    let mut kept = 0;
    for k in 0..n {   // keep only footprints that overlap the body's box
        let (bc, bh, yaw) = seam_box_row(g, g.col.boxes[near[k] as usize]);   // (near holds places in the list wd.boxes)
        let (c, s) = (m::cos(yaw), m::sin(yaw));
        let (ex, ez) = (m::abs(c) * bh[0] + m::abs(s) * bh[2], m::abs(s) * bh[0] + m::abs(c) * bh[2]);
        if bc[0] + ex < x0 || bc[0] - ex > x1 || bc[2] + ez < z0 || bc[2] - ez > z1 || bc[1] - bh[1] > y1 { continue; }
        near[kept] = near[k];
        let this = &mut g.tab.ragdoll[id];
        this.near_cs[kept * 2] = c; this.near_cs[kept * 2 + 1] = s;
        let o = kept * 6;
        this.near_b[o] = bc[0]; this.near_b[o + 1] = bc[1]; this.near_b[o + 2] = bc[2];
        this.near_b[o + 3] = bh[0]; this.near_b[o + 4] = bh[1]; this.near_b[o + 5] = bh[2];
        kept += 1;
    }
    let this = &mut g.tab.ragdoll[id];
    this.near = near;
    this.n_near = kept as f64;
    let room = this.dyn_.len();
    let mut nd = 0;
    let mut j = 0;
    while j < g.col.dynamic_boxes.len() && nd < room {
        let (bc, bh, yaw) = seam_box_row(g, g.col.dynamic_boxes[j]);
        let reach = m::hyp2(bh[0], bh[2]);
        j += 1;
        if bc[0] + reach < x0 || bc[0] - reach > x1 || bc[2] + reach < z0 || bc[2] - reach > z1 { continue; }
        let this = &mut g.tab.ragdoll[id];
        this.dyn_[nd] = (j - 1) as i32;
        this.dyn_cs[nd * 2] = m::cos(yaw); this.dyn_cs[nd * 2 + 1] = m::sin(yaw);
        let o = nd * 6;
        this.dyn_b[o] = bc[0]; this.dyn_b[o + 1] = bc[1]; this.dyn_b[o + 2] = bc[2];
        this.dyn_b[o + 3] = bh[0]; this.dyn_b[o + 4] = bh[1]; this.dyn_b[o + 5] = bh[2];
        nd += 1;
        this.n_dyn = nd as f64;
    }
}

/// True if particle i's centre is inside one of the gathered statics (bind-time check only).
fn rd_buried(this: &crate::ragdoll::Ragdoll, i: f64) -> bool {
    if !this.world { return false; }
    let o = i as usize * 3;
    let x = &this.x;
    for k in 0..this.n_near as usize {
        // (near_b is gatherNear's flat copy of the very box the TypeScript reads here: called only just after a gather, with no game step between)
        let q = k * 6;
        let b = &this.near_b;
        let (c, s) = (this.near_cs[k * 2], this.near_cs[k * 2 + 1]);
        let (dx, dy, dz) = (x[o] - b[q], x[o + 1] - b[q + 1], x[o + 2] - b[q + 2]);
        if m::abs(c * dx - s * dz) < b[q + 3] && m::abs(dy) < b[q + 4] && m::abs(s * dx + c * dz) < b[q + 5] { return true; }
    }
    false
}

/// Push particle i out of the floor, the nearby statics and door leaves; records contact depth / normal.
fn rd_collide(this: &mut crate::ragdoll::Ragdoll, i: f64) {
    let o = i as usize * 3;
    let r = RADIUS[i as usize];
    if this.x[o + 1] < r {
        let pen = r - this.x[o + 1];
        rd_contact(this, i, 0.0, 1.0, 0.0, pen);
        this.x[o + 1] = r;
    }
    if !this.world { return; }
    // (the flat copies are lent out of `this` for the walk: sphereBox takes them beside `this`, and reads nothing else of them)
    let near_b = core::mem::take(&mut this.near_b);
    for k in 0..this.n_near as usize {
        let (c, s) = (this.near_cs[k * 2], this.near_cs[k * 2 + 1]);
        rd_sphere_box(this, i, r, &near_b, (k * 6) as f64, c, s);
    }
    this.near_b = near_b;
    let dyn_b = core::mem::take(&mut this.dyn_b);
    for k in 0..this.n_dyn as usize {
        let (c, s) = (this.dyn_cs[k * 2], this.dyn_cs[k * 2 + 1]);
        rd_sphere_box(this, i, r, &dyn_b, (k * 6) as f64, c, s);
    }
    this.dyn_b = dyn_b;
}

/// Note a contact on particle i: deepest depth, summed normal.
fn rd_contact(this: &mut crate::ragdoll::Ragdoll, i: f64, nx: f64, ny: f64, nz: f64, pen: f64) {
    let i = i as usize;
    if pen > this.pen[i] { this.pen[i] = pen; }
    let o = i * 3;
    this.cn[o] += nx * pen; this.cn[o + 1] += ny * pen; this.cn[o + 2] += nz * pen;
}

/// Sphere (particle i, radius r) against a yawed box; b[q‥q+5] = the box's centre and half-sizes.
fn rd_sphere_box(this: &mut crate::ragdoll::Ragdoll, i: f64, r: f64, b: &[f64], q: f64, c_2: f64, s: f64) {
    let (c, q) = (c_2, q as usize);
    let o = i as usize * 3;
    let (h0, h1, h2) = (b[q + 3], b[q + 4], b[q + 5]);
    let (dx, dy, dz) = (this.x[o] - b[q], this.x[o + 1] - b[q + 1], this.x[o + 2] - b[q + 2]);
    let (lx, ly, lz) = (c * dx - s * dz, dy, s * dx + c * dz);
    if m::abs(lx) > h0 + r || m::abs(ly) > h1 + r || m::abs(lz) > h2 + r { return; }
    let qx = if lx < -h0 { -h0 } else if lx > h0 { h0 } else { lx };
    let qy = if ly < -h1 { -h1 } else if ly > h1 { h1 } else { ly };
    let qz = if lz < -h2 { -h2 } else if lz > h2 { h2 } else { lz };
    let (ddx, ddy, ddz) = (lx - qx, ly - qy, lz - qz);
    let d2 = ddx * ddx + ddy * ddy + ddz * ddz;
    if d2 >= r * r { return; }
    let (nlx, nly, nlz, pen);
    if d2 > 1e-12 {
        let d = m::sqrt(d2);
        nlx = ddx / d; nly = ddy / d; nlz = ddz / d; pen = r - d;
    } else {
        // centre inside: out through a face the previous position was outside of, else the nearest face
        let (pdx, pdy, pdz) = (this.px[o] - b[q], this.px[o + 1] - b[q + 1], this.px[o + 2] - b[q + 2]);
        this.exit_l[0] = lx; this.exit_l[1] = ly; this.exit_l[2] = lz;
        this.exit_p[0] = c * pdx - s * pdz; this.exit_p[1] = pdy; this.exit_p[2] = s * pdx + c * pdz;
        let (mut best_a, mut best_s, mut best) = (-1i32, 1.0, 1e9);
        let mut pass = 0;
        while pass < 2 && best_a < 0 {
            for a in 0..3usize {
                for sg in [-1.0f64, 1.0] {
                    if pass == 0 && sg * this.exit_p[a] < b[q + 3 + a] - 1e-9 { continue; }   // first pass: only faces the previous position was beyond
                    let fd = b[q + 3 + a] - sg * this.exit_l[a];
                    if fd < best { best = fd; best_a = a as i32; best_s = sg; }
                }
            }
            pass += 1;
        }
        nlx = if best_a == 0 { best_s } else { 0.0 };
        nly = if best_a == 1 { best_s } else { 0.0 };
        nlz = if best_a == 2 { best_s } else { 0.0 };
        pen = best + r;
    }
    let (nx, ny, nz) = (c * nlx + s * nlz, nly, -s * nlx + c * nlz);
    this.x[o] += nx * pen; this.x[o + 1] += ny * pen; this.x[o + 2] += nz * pen;
    rd_contact(this, i, nx, ny, nz, pen);
}

/// Rotational part of the 3×3 matrix A (row-major), refined from the warm start q in place (Müller et al. 2016).
fn extract_rotation(a: &[f64], q: &mut Quat, iters: f64) {
    let mut rr = [0.0; 9];   // (erR: module scratch in the TypeScript)
    let mut it = 0.0;
    while it < iters {
        it += 1.0;
        quat_mat(*q, &mut rr);
        // columns j of R and A: (R[j], R[3+j], R[6+j])
        let (mut ox, mut oy, mut oz, mut dot) = (0.0, 0.0, 0.0, 0.0);
        for j in 0..3 {
            let (rx, ry, rz, ax, ay, az) = (rr[j], rr[3 + j], rr[6 + j], a[j], a[3 + j], a[6 + j]);
            ox += ry * az - rz * ay; oy += rz * ax - rx * az; oz += rx * ay - ry * ax; dot += rx * ax + ry * ay + rz * az;
        }
        let inv = 1.0 / (m::abs(dot) + 1e-9);
        ox *= inv; oy *= inv; oz *= inv;
        let wl = m::hyp3(ox, oy, oz);
        if wl < 1e-9 { break; }
        let (ha, sn, cw);
        ha = wl * 0.5; sn = m::sin(ha) / wl; cw = m::cos(ha);
        let (bx, by, bz) = (ox * sn, oy * sn, oz * sn);   // q ← axisAngle(ω/|ω|, |ω|) · q
        let (ax, ay, az, aw) = (q[0], q[1], q[2], q[3]);
        let nx = cw * ax + bx * aw + by * az - bz * ay;
        let ny = cw * ay - bx * az + by * aw + bz * ax;
        let nz = cw * az + bx * ay - by * ax + bz * aw;
        let nw = cw * aw - bx * ax - by * ay - bz * az;
        let l = js::or(m::hyp4(nx, ny, nz, nw), 1.0);
        q[0] = nx / l; q[1] = ny / l; q[2] = nz / l; q[3] = nw / l;
    }
}

/// Row-major 3×3 rotation matrix of unit quaternion q into out.
fn quat_to_mat(q: Quat, out: &mut Vec<f64>) {
    quat_mat(q, out.as_mut_slice());
}

/// quatToMat onto any row of nine: extractRotation's scratch is a local array, so a substep still allocates nothing.
fn quat_mat(q: Quat, out: &mut [f64]) {
    let (x, y, z, w) = (q[0], q[1], q[2], q[3]);
    let (xx, yy, zz, xy, xz, yz, wx, wy, wz) = (x * x * 2.0, y * y * 2.0, z * z * 2.0, x * y * 2.0, x * z * 2.0, y * z * 2.0, w * x * 2.0, w * y * 2.0, w * z * 2.0);
    out[0] = 1.0 - yy - zz; out[1] = xy - wz; out[2] = xz + wy;
    out[3] = xy + wz; out[4] = 1.0 - xx - zz; out[5] = yz - wx;
    out[6] = xz - wy; out[7] = yz + wx; out[8] = 1.0 - xx - yy;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trunk_set_is_trunk() {
        for i in 0..N { assert_eq!(TRUNK_SET[i] != 0, TRUNK.contains(&i)); }
    }
}
