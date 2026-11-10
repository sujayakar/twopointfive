//! THE DOORS' STEP: every leaf of a level for one tick. What was written onto it since the last one, the men and the bodies in its swing (who turns it, whom
//! it moves, whom it passes), its free swing, its stops, its closer, its latch and its lock's clocks; and then where its box stands.
use super::*;
use crate::game::Game;
use crate::ids::*;
use crate::m::{self, cos, exp, floor, hyp2 as hypot2, sin, sqrt};
use crate::v3::{clamp, V3};
use alloc::{vec, vec::Vec};
use ::sim::{collide, world::{asking, js_abs, js_max, js_min, world, REC, R_C, R_COS, R_SIN, R_YAW}};
use DoorDriveKind::*;

const SOLID_SLIDE: f64 = 2.2;
const SOLID_TRIES: f64 = 3.0;
const SOLID_FADE: f64 = 0.5;
const SOLID_SKIN: f64 = 0.02;
const SOLID_RATE: f64 = 18.0;
const SOLID_REACH: f64 = 0.6;
const SOLID_INTO: f64 = 0.4;
const SOLID_BACK: f64 = 3.0;
const PRESS_SECS: f64 = 0.08;
const DRIVEN_VEL: f64 = 0.5;
const STOP_BODY: f64 = 0.3;
const ROUND_SLIDE: f64 = 1.5;
const ROUND_SECS: f64 = 1.2;
const ROUND_STILL: f64 = 1.0;
const SHOVE_OPENS_BELOW: f64 = 1.2;
const SHOVE_RATE: f64 = 7.2;
const TURN_RATE: f64 = 15.0;
const KICK_SUBSTEP: f64 = 0.16;
const MAN_SIDE: i32 = 3;
const MAN_FOLDED: i32 = 4;
const MAN_BRACED: i32 = 8;
const MAN_CALM: i32 = 16;
const MAN_HELD: i32 = 32;
/// a standing man's radius against the statics, in a door frame, and how the one eases into the other
const BODY_R: f64 = 0.42;
const DOOR_R: f64 = 0.3;
const DOOR_FIT_LAT: [f64; 2] = [0.12, 0.5];
const DOOR_FIT_OUT: [f64; 2] = [0.55, 1.0];
/// a box that seals sound
const SOUND_SEAL: u32 = 1 << 16;

/// the step's answer of `sign`: 0 for 0
fn js_sign(x: f64) -> f64 { m::sign(x) }
fn side_of(x: f64) -> f64 { if x >= 0.0 { 1.0 } else { -1.0 } }

/// what the game was asked of a contact before the step
struct Asked { man: i32, opens: u64 }
/// a leaf driven by somebody
pub(crate) trait Driven { fn driven(&mut self, who: f64, dir: f64, kind: DoorDriveKind, quiet: bool, rate: f64, m: i32); }
impl Driven for Leaf {
    fn driven(&mut self, who: f64, dir: f64, kind: DoorDriveKind, mut quiet: bool, rate: f64, m: i32) {
        let s = side_of(dir);
        if kind != Boot && m & MAN_CALM != 0 {
            quiet = true;
        }
        let held = matches!(kind, Body | Hand);
        let (angle, side) = (self.angle, (m & MAN_SIDE) as f64);
        match &mut self.drive {
            Some(d) if d.who == who && d.dir == s && d.kind == kind => {
                if !held || d.t >= DRIVEN_SECS { d.hit = -1.0; }
                (d.t, d.quiet, d.rate, d.side) = (0.0, quiet, rate, side);
                if !held { d.from = angle; }   // (a walk or a hand goes on: one drive, from where it began)
            }
            d => *d = Some(DoorDrive { who, dir: s, kind, t: 0.0, from: angle, quiet, side, rate, px: f64::NAN, pz: f64::NAN, hit: -1.0 }),
        }
    }
}
/// one tick's step
struct Step<'a> {
    ids: &'a [DoorId],
    dr: Vec<&'a Door>,
    leaf: Vec<Leaf>,
    con: &'a mut [DoorContact],
    asked: Vec<Asked>,
    /// the angle a leaf off its hinges reads as · what the game knows of the man last on it, where the step could ask
    slab: Vec<Option<f64>>,
    man_last: Vec<Option<i32>>,
    k: crate::knobs::DoorthrowK,
    world: crate::scene_world::WORLD,
    w: usize,
    dt: f64,
    n: usize,
    nc: usize,
    foot: f64,
    settle_y1: f64,
    placed: Vec<Option<[V3; 2]>>,
    ev: Vec<(usize, DoorEvent)>,
    rounds: f64,
}

impl Step<'_> {
    /// how it is being driven (all zeros: not)
    fn drive(&self, i: usize) -> DoorDrive {
        self.leaf[i].drive.unwrap_or_default()
    }
    #[inline]
    fn swing(&self, i: usize) -> f64 {
        let l = &self.leaf[i];
        if l.clearway != 0.0 { l.clearway } else { l.dealt }
    }
    #[inline]
    fn min_a(&self, i: usize) -> f64 {
        if self.swing(i) > 0.0 {
            0.0
        } else {
            self.dr[i].min_auth
        }
    }
    #[inline]
    fn max_a(&self, i: usize) -> f64 {
        if self.swing(i) < 0.0 {
            0.0
        } else {
            self.dr[i].max_auth
        }
    }
    fn keep(&mut self, i: usize) {
        if self.leaf[i].angle * self.swing(i) < 0.0 {
            self.leaf[i].angle = clamp(-self.leaf[i].angle, self.min_a(i), self.max_a(i));
            self.leaf[i].vel = -self.leaf[i].vel;
        }
    }
    #[inline]
    fn dir(&self, i: usize) -> (f64, f64) {
        let a = self.dr[i].def.closed_dir + self.leaf[i].angle;
        (cos(a), sin(a))
    }
    #[inline]
    fn rests_to(&self, i: usize) -> bool {
        self.leaf[i].swept && !self.leaf[i].latched && self.leaf[i].angle == 0.0 && self.leaf[i].vel == 0.0
    }
    fn pull_face(&self, i: usize, x: f64, z: f64) -> bool {
        let c = self.dr[i].def.closed_dir;
        ((x - self.dr[i].def.hinge[0]) * -sin(c) + (z - self.dr[i].def.hinge[1]) * cos(c)) * self.swing(i) >= 0.0
    }
    fn opening(&self, i: usize, dir: f64) -> bool {
        dir == self.swing(i)
    }
    /// where an event says the door is: its hinge and half its width along the leaf, as it stands
    fn pos(&self, i: usize) -> (f64, f64) {
        let u = self.dir(i);
        (self.dr[i].def.hinge[0] + u.0 * self.dr[i].def.width * 0.5, self.dr[i].def.hinge[1] + u.1 * self.dr[i].def.width * 0.5)
    }
    fn emit(&mut self, i: usize, sound: DoorSound, who: f64, level: f64) {
        let (x, z) = self.pos(i);
        self.ev.push((i, DoorEvent { door: self.ids[i], sound, pos: [x, 1.0, z], who, level }));
    }
    fn rattle(&mut self, i: usize, who: f64, level: f64) {
        if self.leaf[i].rattle_t > 0.0 || self.leaf[i].off {
            return;
        }
        self.leaf[i].rattle_t = 0.6;
        self.leaf[i].last_who = who;
        self.emit(i, DoorSound::Rattle, who, level);
    }
    fn pass_bit(&self, c: usize) -> i32 {
        let who = self.con[c].who;
        if who < 0.0 {
            0
        } else if who < 32.0 {
            1i32.wrapping_shl(who as u32)
        } else if self.con[c].key == Some(true) {
            1i32 << (2 + ((who as i64 - 32) % 8) as u32)
        } else {
            0
        }
    }
    /// does this contact's key ring get it through door i's lock
    #[inline]
    fn opens(&self, c: usize, i: usize) -> bool {
        self.con[c].key == Some(true) || self.asked[c].opens >> i & 1 != 0
    }
    #[inline]
    fn man_of(&self, c: usize) -> i32 {
        self.asked[c].man
    }

    // ---- the floor (the host's two functions, over the statics of this world)
    /// (x, z) pushed out of the statics over [0.2, y1]
    fn collide(&self, x: &mut f64, z: &mut f64, r: f64, y1: f64, iterations: u32) -> bool {
        let hit = collide::circle(self.w, *x, *z, r, self.foot, y1, iterations) != 0;
        let pt = asking(self.w).pt;
        *x = pt[0];
        *z = pt[1];
        hit
    }
    fn clamp_to_world(&self, x: &mut f64, z: &mut f64, r: f64) -> bool {
        let nx = js_min(self.world.max[0] - r, js_max(self.world.min[0] + r, *x));
        let nz = js_min(self.world.max[2] - r, js_max(self.world.min[2] + r, *z));
        let moved = nx != *x || nz != *z;
        *x = nx;
        *z = nz;
        moved
    }
    /// Is there floor for a body of radius r at (x, z)
    fn standable(&self, x: f64, z: f64, r: f64) -> bool {
        let (mut qx, mut qz) = (x, z);
        self.collide(&mut qx, &mut qz, js_min(r, 0.3), 1.5, 1);
        hypot2(qx - x, qz - z) < 0.02 && !self.clamp_to_world(&mut qx, &mut qz, 0.3)
    }
    /// A standing man's radius at p (a door frame lets him through narrower)
    fn body_r(&self, x: f64, z: f64) -> f64 {
        let mut k = 0.0;
        for i in 0..self.n {
            let (dx, dz) = (x - self.dr[i].frame_centre[0], z - self.dr[i].frame_centre[2]);
            if dx * dx + dz * dz > 1.3 {
                continue;
            }
            let c = self.dr[i].def.closed_dir;
            let (ux, uz) = (cos(c), sin(c));
            let (lat, out) = (js_abs(dx * ux + dz * uz), js_abs(dz * ux - dx * uz));
            let kl = 1.0 - clamp((lat - DOOR_FIT_LAT[0]) / (DOOR_FIT_LAT[1] - DOOR_FIT_LAT[0]), 0.0, 1.0);
            let ko = 1.0 - clamp((out - DOOR_FIT_OUT[0]) / (DOOR_FIT_OUT[1] - DOOR_FIT_OUT[0]), 0.0, 1.0);
            k = js_max(k, kl * kl * (3.0 - 2.0 * kl) * ko * ko * (3.0 - 2.0 * ko));
        }
        BODY_R - (BODY_R - DOOR_R) * k
    }
    /// Where the statics let a man moved to (x, z) stand
    fn settle(&self, x: &mut f64, z: &mut f64) {
        let y1 = self.settle_y1;
        for _ in 0..4 {
            let r = self.body_r(*x, *z);
            self.collide(x, z, r, y1, 6);
            if !self.clamp_to_world(x, z, BODY_R) {
                break;
            }
        }
    }

    fn walked(&mut self, i: usize, c: usize, dir: f64, lever: f64) {
        let who = self.con[c].who;
        let d = self.leaf[i].drive.filter(|d| d.who == who);
        if d.is_some_and(|d| d.kind == Boot) && self.leaf[i].since_bash < 0.6 {
            return;
        }
        let d = d.filter(|d| d.kind == Body && d.dir == side_of(dir) && d.t <= 2.0 * self.dt + 1e-6 && d.px == d.px);
        let rate = d.map_or(0.0, |d| js_min(hypot2(self.con[c].pos[0] - d.px, self.con[c].pos[2] - d.pz) / js_max(self.dt, 1e-3) / lever, TURN_RATE));
        let quiet = self.con[c].quiet;
        let m = self.man_of(c);
        self.leaf[i].driven(who, dir, Body, quiet, rate, m);
    }
    #[allow(clippy::too_many_arguments)]
    fn carry(&mut self, c: usize, i: usize, pen: f64, dist: f64, qx: f64, qz: f64, perp: f64) -> bool {
        let (t_rate, t_slide) = (self.k.rate, self.k.slide);
        if pen > t_rate * self.dt {
            return false;
        }
        let u = self.dir(i);
        let (mut nx, mut nz, mut n) = (qx, qz, dist);
        if n < 1e-4 {
            let s = side_of(perp);
            nx = -u.1 * s;
            nz = u.0 * s;
            n = 1.0;
        }
        let k = (pen + 0.001) / n;
        let sl = t_slide * self.dt;
        let (x, z) = (self.con[c].pos[0] + nx * k, self.con[c].pos[2] + nz * k);
        let r = self.con[c].radius;
        if self.standable(x + u.0 * sl, z + u.1 * sl, r) {
            self.con[c].pos[0] = x + u.0 * sl;
            self.con[c].pos[2] = z + u.1 * sl;
        } else if self.standable(x, z, r) {
            self.con[c].pos[0] = x;
            self.con[c].pos[2] = z;
        } else {
            return false;
        }
        self.con[c].moved = Some(true);
        true
    }
    #[allow(clippy::too_many_arguments)]
    fn solid_carry(&mut self, c: usize, i: usize, pen: f64, dist: f64, qx: f64, qz: f64, perp: f64) -> bool {
        if pen > SOLID_RATE * self.dt {
            return false;
        }
        let (hx, hz, wd) = (self.dr[i].def.hinge[0], self.dr[i].def.hinge[1], self.dr[i].def.width);
        let u = self.dir(i);
        let (mut nx, mut nz, mut n) = (qx, qz, dist);
        if n < 1e-4 {
            let s = side_of(perp);
            nx = -u.1 * s;
            nz = u.0 * s;
            n = 1.0;
        }
        let k = (pen + 0.001) / n;
        let own = self.con[c].vel.map_or(0.0, |v| js_max(0.0, -(v[0] * u.0 + v[2] * u.1)));
        let sl = (SOLID_SLIDE + own) * self.dt;
        let (mut p0, mut p2) = (self.con[c].pos[0] + nx * k + u.0 * sl, self.con[c].pos[2] + nz * k + u.1 * sl);
        let r = self.con[c].radius;
        self.settle(&mut p0, &mut p2);
        let (px, pz) = (p0 - hx, p2 - hz);
        let t = clamp(px * u.0 + pz * u.1, 0.0, wd);
        let left = r + self.dr[i].thick / 2.0 - hypot2(px - u.0 * t, pz - u.1 * t);
        if left > SOLID_SKIN {
            return false;
        }
        self.con[c].pos[0] = p0;
        self.con[c].pos[2] = p2;
        true
    }
    fn leaf_presses(&self, i: usize) -> bool {
        let (hx, hz, wd) = (self.dr[i].def.hinge[0], self.dr[i].def.hinge[1], self.dr[i].def.width);
        let ht = self.dr[i].thick / 2.0;
        let u = self.dir(i);
        if self.leaf[i].pull_t > 0.0 {
            return false;
        }
        let passers = self.leaf[i].passers;
        for c in 0..self.nc {
            if self.con[c].weak == Some(true) {
                continue;
            }
            let r = self.con[c].radius;
            let (px, pz) = (self.con[c].pos[0] - hx, self.con[c].pos[2] - hz);
            let reach = wd + r + 0.1;
            if px * px + pz * pz > reach * reach {
                continue;
            }
            if self.pass_bit(c) & passers != 0 {
                continue;
            }
            let along = px * u.0 + pz * u.1;
            if along <= 0.14 || along >= wd + r {
                continue;
            }
            let t = clamp(along, 0.0, wd);
            let dist = hypot2(px - u.0 * t, pz - u.1 * t);
            if r + ht - dist > 0.0 {
                return true;
            }
        }
        false
    }

    /// The leaf's row of the box world written where it now stands, and its handles handed back
    fn place(&mut self, i: usize) {
        if self.leaf[i].off {
            return;
        }
        self.keep(i);
        self.leaf[i].vel_seen = self.leaf[i].vel;
        self.leaf[i].angle_seen = self.leaf[i].angle;
        let rt = self.leaf[i].rattle_t;
        let jig = if rt > 0.35 { sin(rt * 90.0) * 0.005 } else { 0.0 };
        let d = self.dr[i];
        let p = pose_of(d.def.hinge, d.def.width, d.def.closed_dir, d.thick, d.height, self.leaf[i].angle + jig);
        let seal = js_abs(self.leaf[i].angle) < 0.3;
        // the box world's row (list 0 of the leaves: the game's own, a door at its own index)
        let o = world(self.w);
        let b = &mut o.leaf[i * REC..i * REC + REC];
        b[R_C..R_C + 3].copy_from_slice(&p.c);
        b[R_COS] = cos(p.yaw);
        b[R_SIN] = sin(p.yaw);
        b[R_YAW] = p.yaw;
        o.leaf_flags[i] = if seal { o.leaf_flags[i] | SOUND_SEAL } else { o.leaf_flags[i] & !SOUND_SEAL };
        self.placed[i] = Some(p.handles);
    }

    /// the step of door i
    fn door(&mut self, i: usize) {
        let dt = self.dt;
        if self.leaf[i].off {
            self.leaf[i].vel = 0.0;
            self.leaf[i].latched = false;
            self.leaf[i].closing = false;
            if let Some(a) = self.slab[i] {
                self.leaf[i].angle = a;
            }
            return;
        }
        let (hx, hz, wd) = (self.dr[i].def.hinge[0], self.dr[i].def.hinge[1], self.dr[i].def.width);
        let ht = self.dr[i].thick / 2.0;
        let sw = self.swing(i);
        if self.leaf[i].swept && (self.leaf[i].vel != self.leaf[i].vel_seen || self.leaf[i].angle != self.leaf[i].angle_seen) {
            self.leaf[i].swept = false;
        }
        let mut eased = false;
        {
            self.keep(i);
            if self.leaf[i].vel != self.leaf[i].vel_seen && !self.leaf[i].closing && !self.leaf[i].latched {
                if self.leaf[i].vel * sw < 0.0 {
                    if self.leaf[i].angle * sw < SHOVE_OPENS_BELOW {
                        self.leaf[i].vel = -self.leaf[i].vel;
                        self.leaf[i].pull_t = PULL_SECS;
                    }
                } else if self.leaf[i].vel * sw > 0.0 {
                    let last = self.leaf[i].last_who;
                    let mut by = usize::MAX;
                    for c in 0..self.nc {
                        if self.con[c].weak != Some(true) && self.con[c].who == last {
                            by = c;
                            break;
                        }
                    }
                    if by != usize::MAX && self.pull_face(i, self.con[by].pos[0], self.con[by].pos[2]) && hypot2(self.con[by].pos[0] - hx, self.con[by].pos[2] - hz) < wd + 1.2 {
                        self.leaf[i].pull_t = PULL_SECS;
                    } else {
                        self.leaf[i].open_t = 0.0;
                    }
                }
            }
            if (self.leaf[i].angle - self.leaf[i].angle_seen) * sw > 1e-4 {
                self.leaf[i].open_t = 0.0;
                eased = true;
            }
        }
        {
            let da = self.leaf[i].angle - self.leaf[i].angle_seen;
            let pulled = self.leaf[i].pull_t > 0.0;
            let rate = js_abs(da) / js_max(dt, 1e-3);
            if pulled && self.leaf[i].drive.is_some() {
                self.leaf[i].drive = None;
            }
            let vel = self.leaf[i].vel;
            let free = !self.leaf[i].closing && !self.leaf[i].latched && !pulled;
            if vel != self.leaf[i].vel_seen && vel != 0.0 && free && self.opening(i, js_sign(vel)) {
                if !(self.leaf[i].drive.is_some() && self.drive(i).t == 0.0 && self.drive(i).kind == Boot) {
                    let m = self.man_last(i);
                    let l = &mut self.leaf[i];
                    l.driven(l.last_who, vel, Swing, false, 0.0, m);
                }
            } else if da != 0.0
                && rate >= (if self.leaf[i].drive.is_some() && self.drive(i).kind == Hand && self.drive(i).t < DRIVEN_SECS { 0.5 } else { 1.0 }) * self.k.hand_rate
                && rate <= TURN_RATE
                && free
                && self.opening(i, js_sign(da))
            {
                let m = self.man_last(i);
                let l = &mut self.leaf[i];
                    l.driven(l.last_who, da, Hand, false, rate, m);
            }
        }
        let kicked = self.leaf[i].since_bash < 0.6;
        if kicked && !self.leaf[i].latched {
            let total = clamp(self.leaf[i].angle + self.leaf[i].vel * dt, self.min_a(i), self.max_a(i)) - self.leaf[i].angle;
            let n = -floor(-(js_abs(total) / KICK_SUBSTEP));
            if n <= 1.0 {
                self.leaf[i].angle = clamp(self.leaf[i].angle + self.leaf[i].vel * dt, self.min_a(i), self.max_a(i));
            } else {
                let mut k = 0.0;
                while k < n {
                    let a0 = self.leaf[i].angle;
                    self.leaf[i].angle = a0 + total / n;
                    if self.leaf_presses(i) {
                        break;
                    }
                    k += 1.0;
                }
            }
        }
        let (mut touched, mut yank, mut pushed, mut body_block) = (false, false, false, false);
        let (mut push_dir, mut key_touched) = (0.0, false);
        let mut here: i32 = 0;
        let mut pressed = false;
        let mut ran = false;
        let mut walker = usize::MAX;
        let comes = self.leaf[i].vel * sw > 0.0 || (self.leaf[i].angle - self.leaf[i].angle_seen) * sw > 1e-4;
        // (DOORWAY.solid) he stands back from its swing
        if self.leaf[i].pull_t > 0.0 && comes && !self.leaf[i].off && !self.leaf[i].latched && self.leaf[i].solid_t <= SOLID_TRIES - 1.0 {
            for c in 0..self.nc {
                    if self.con[c].solid != Some(true) || self.con[c].weak == Some(true) {
                    continue;
                }
                let th = self.dr[i].def.closed_dir;
                let (ax, az) = (cos(th), sin(th));
                let (cx, cz, r) = (self.con[c].pos[0], self.con[c].pos[2], self.con[c].radius);
                let (px, pz) = (cx - hx, cz - hz);
                let a = px * ax + pz * az;
                let o = sw * (ax * pz - az * px);
                let rr = wd + r + self.dr[i].thick / 2.0 + SOLID_SKIN;
                if o <= 0.0 || a < 0.0 || a * a + o * o >= rr * rr {
                    continue;
                }
                let u = self.dir(i);
                if side_of(u.0 * pz - u.1 * px) != sw {
                    continue;
                }
                let (bx, bz) = (-az * sw, ax * sw);
                let back = sqrt(rr * rr - a * a) - o;
                let aside = sqrt(rr * rr - o * o) - a;
                let way = if self.free(cx + bx * back, cz + bz * back) {
                    1
                } else if self.free(cx + ax * aside, cz + az * aside) {
                    2
                } else {
                    0
                };
                if way == 0 {
                    continue;
                }
                let (vx, vz) = self.con[c].vel.map_or((0.0, 0.0), |v| (v[0], v[2]));
                let own = if self.con[c].vel.is_some() { if way == 1 { js_max(0.0, -(bx * vx + bz * vz)) } else { js_max(0.0, -(ax * vx + az * vz)) } } else { 0.0 };
                let step = js_min(if way == 1 { back } else { aside }, (SOLID_BACK + own) * dt);
                let (mut p0, mut p2) = if way == 1 { (cx + bx * step, cz + bz * step) } else { (cx + ax * step, cz + az * step) };
                self.settle(&mut p0, &mut p2);
                self.con[c].pos[0] = p0;
                self.con[c].pos[2] = p2;
            }
        }
        // (DOORWAY.solid) his hand reaches it before his body
        if self.leaf[i].pull_t <= 0.0
            && !self.leaf[i].off
            && self.leaf[i].solid_t <= SOLID_TRIES - 1.0
            && self.leaf[i].open_t >= DRIVEN_SECS
            && self.leaf[i].vel * sw <= DRIVEN_VEL
            && self.leaf[i].angle * sw < js_min(1.35, 0.8 * js_abs(if sw > 0.0 { self.max_a(i) } else { self.min_a(i) }))
        {
            for c in 0..self.nc {
                    if self.con[c].solid != Some(true) || self.con[c].vel.is_none() || self.con[c].bash || self.con[c].quiet || self.con[c].weak == Some(true) {
                    continue;
                }
                let u = self.dir(i);
                let (r, who) = (self.con[c].radius, self.con[c].who);
                let (px, pz) = (self.con[c].pos[0] - hx, self.con[c].pos[2] - hz);
                let along = px * u.0 + pz * u.1;
                let perp = u.0 * pz - u.1 * px;
                let side = side_of(perp);
                let gap = js_abs(perp) - r - self.dr[i].thick / 2.0;
                if side != sw || gap <= 0.0 || gap > SOLID_REACH || along <= 0.14 || along >= wd {
                    continue;
                }
                if self.leaf[i].locked && self.leaf[i].latched && !self.opens(c, i) && side == self.dr[i].key_side {
                    continue;
                }
                let (vx, vz) = self.con[c].vel.map_or((0.0, 0.0), |v| (v[0], v[2]));
                let into = -side * (u.0 * vz - u.1 * vx);
                let sp = hypot2(vx, vz);
                if into < SOLID_INTO || into < 0.5 * sp {
                    continue;
                }
                let meets = along + (vx * u.0 + vz * u.1) * (gap / into);
                if meets <= 0.0 || meets >= wd {
                    continue;
                }
                pressed = true;
                if self.leaf[i].press_t < PRESS_SECS {
                    continue;
                }
                let quiet = self.con[c].quiet;
                if self.leaf[i].latched || self.rests_to(i) {
                    self.leaf[i].latched = false;
                    if self.leaf[i].creak_armed {
                        self.emit(i, DoorSound::Creak, who, if quiet { 0.12 } else { 0.3 });
                        self.leaf[i].creak_armed = false;
                    }
                }
                self.leaf[i].vel = sw * js_max(self.leaf[i].vel * sw, if quiet { PULL_VEL_QUIET } else { PULL_VEL });
                self.leaf[i].closing = false;
                self.leaf[i].swept = false;
                self.leaf[i].pull_t = PULL_SECS;
                touched = true;
                pushed = true;
                self.leaf[i].last_who = who;
            }
        }
        for pass in 0..3u32 {
            for c in 0..self.nc {
                    let (weak, key, bash, quiet) = (self.con[c].weak == Some(true), self.con[c].key == Some(true), self.con[c].bash, self.con[c].quiet);
                if (if weak { 2 } else if key { 0 } else { 1 }) != pass {
                    continue;
                }
                let (r, who) = (self.con[c].radius, self.con[c].who);
                let (vx, vz) = (self.con[c].pos[0] - hx, self.con[c].pos[2] - hz);
                let reach = wd + r + 0.1;
                if vx * vx + vz * vz > reach * reach {
                    continue;
                }
                if weak && self.con[c].pos[1] - r > self.dr[i].height {
                    continue;
                }
                let solid = self.con[c].solid == Some(true);
                for _it in 0..3 {
                    let u = self.dir(i);
                    let (px, pz) = (self.con[c].pos[0] - hx, self.con[c].pos[2] - hz);
                    let along = px * u.0 + pz * u.1;
                    let perp = u.0 * pz - u.1 * px;
                    let t = clamp(along, 0.0, wd);
                    let (qx, qz) = (px - u.0 * t, pz - u.1 * t);
                    let dist = hypot2(qx, qz);
                    let pen = r + ht - dist;
                    if pen <= (if weak { 0.002 } else { 0.0 }) {
                        break;
                    }
                    let bit = if !weak { self.pass_bit(c) } else { 0 };
                    let man = if !weak {
                        self.con[c].man = Some(self.man_of(c) as f64);
                        self.man_of(c)
                    } else {
                        0
                    };
                    let folded = man & MAN_FOLDED != 0;
                    if bit & self.leaf[i].passers != 0 {
                        here |= bit;
                        break;
                    }
                    if self.leaf[i].pull_t > 0.0 && !self.leaf[i].latched && !weak {
                        if solid && self.leaf[i].solid_t <= SOLID_TRIES - 1.0 {
                            if side_of(perp) == sw && comes && along > 0.14 && along < wd + r {
                                if self.solid_carry(c, i, pen, dist, qx, qz, perp) {
                                    if self.leaf[i].vel * sw > PULL_VEL {
                                        self.leaf[i].vel = sw * PULL_VEL;
                                    }
                                    break;
                                }
                                self.leaf[i].solid_t += 1.0;
                                self.leaf[i].angle -= sw * js_min(self.leaf[i].angle * sw, (pen + 0.001) / js_max(0.18, t));
                                self.leaf[i].vel = 0.0;
                                self.leaf[i].pull_t = 0.0;
                                break;
                            }
                        } else {
                            if bit != 0 {
                                self.leaf[i].passers |= bit;
                                here |= bit;
                            }
                            break;
                        }
                    }
                    if weak && (self.leaf[i].latched || pushed || kicked || eased || (self.leaf[i].pull_t > 0.0)) {
                        let k = pen / js_max(dist, 1e-4);
                        self.con[c].pos[0] += qx * k;
                        self.con[c].pos[2] += qz * k;
                        break;
                    }
                    // (DOORTHROW) a leaf somebody else drove open toward him
                    let into = side_of(perp);
                    let dr = !weak && self.leaf[i].drive.is_some();
                    let toward = dr && self.drive(i).who != who && self.drive(i).dir == into;
                    let at_him = toward && !self.drive(i).quiet && (self.drive(i).kind == Boot || self.drive(i).side == 0.0 || self.drive(i).side != (man & MAN_SIDE) as f64);
                    let met = dr && self.drive(i).hit == who && self.drive(i).t < self.k.again;
                    if dr && toward && !self.leaf[i].closing && along > 0.14 && along < wd + r {
                        let vin = into * self.leaf[i].vel * t;
                        let kind = self.drive(i).kind;
                        let tier = if kind == Boot {
                            DoorStrikeTier::Boot
                        } else if kicked {
                            DoorStrikeTier::Slam
                        } else {
                            DoorStrikeTier::Open
                        };
                        let held = !kicked && (kind == Body || kind == Hand);
                        let fresh = held && self.drive(i).t < DRIVEN_SECS;
                        let stands = !folded && man & (if tier == DoorStrikeTier::Open { MAN_HELD } else { MAN_BRACED }) == 0;
                        let min_speed = self.k.min_speed;
                        if at_him && self.drive(i).hit != who && stands && (if held { fresh && self.drive(i).rate * t >= min_speed } else { vin >= min_speed }) {
                            let n = if dist > 1e-4 { 1.0 / dist } else { 0.0 };
                            let dir = if n != 0.0 && n == n { [qx * n, qz * n] } else { [-u.1 * into, u.0 * into] };
                            let d = self.drive(i);
                            let floor = tier == DoorStrikeTier::Boot && js_abs(self.leaf[i].angle - d.from) < self.k.floor_rad;
                            self.con[c].struck = Some(DoorStrike { door: self.ids[i], by: d.who, tier, floor, dir });
                            if let Some(d) = &mut self.leaf[i].drive { d.hit = who; }
                            if self.carry(c, i, pen, dist, qx, qz, perp) {
                                break;
                            }
                        } else if (folded || met) && (vin >= 0.2 || fresh) && self.carry(c, i, pen, dist, qx, qz, perp) {
                            break;
                        }
                    }
                    if weak {
                        body_block = true;
                    } else {
                        touched = true;
                        if key || !key_touched {
                            self.leaf[i].last_who = who;
                        }
                        if key {
                            key_touched = true;
                        }
                    }
                    if !weak && !self.opens(c, i) && self.leaf[i].locked && self.leaf[i].latched && side_of(perp) == self.dr[i].key_side && along > 0.14 && along < wd + r {
                        let k = shove(pen, dt, true) / js_max(dist, 1e-4);
                        self.con[c].pos[0] += qx * k;
                        self.con[c].pos[2] += qz * k;
                        self.con[c].blocked_by = Some(self.ids[i]);
                        if self.leaf[i].creak_armed && pen > 0.015 {
                            self.leaf[i].creak_armed = false;
                            self.rattle(i, who, if quiet { 0.08 } else { 0.12 });
                        }
                        break;
                    }
                    let limp = folded || met;
                    let pulling = key && !limp && !bash && self.leaf[i].closing && js_abs(self.leaf[i].angle) > 0.35;
                    let has_hand = self.con[c].hand.is_some();
                    let run = self.con[c].run;
                    let has_run = run.is_some();
                    if along > 0.14 && along < wd + r {
                        let sgn = if perp >= 0.0 { -1.0 } else { 1.0 };
                        let lever = js_max(0.18, t);
                        let wide = js_min(1.35, 0.8 * js_abs(if sw > 0.0 { self.max_a(i) } else { self.min_a(i) }));
                        let presses = !weak && !limp && sgn * sw < 0.0 && self.leaf[i].angle * sw < wide && self.leaf[i].open_t >= DRIVEN_SECS && self.leaf[i].vel * sw <= DRIVEN_VEL;
                        if presses {
                            pressed = true;
                            let hauled = bash && !has_hand && self.leaf[i].since_bash > 0.9;
                            if hauled || self.leaf[i].press_t >= (self.con[c].hand.unwrap_or(PRESS_SECS)) {
                                if hauled {
                                    self.leaf[i].latched = false;
                                    self.leaf[i].vel = sw * (run.map_or(7.5, |r| r.rip));
                                    self.leaf[i].since_bash = 0.0;
                                    if has_run {
                                        self.con[c].ripped = Some(self.ids[i]);
                                    }
                                    self.emit(i, DoorSound::Bash, who, run.map_or(1.0, |r| r.rip_level));
                                } else {
                                    if self.leaf[i].latched || self.rests_to(i) {
                                        self.leaf[i].latched = false;
                                        if self.leaf[i].creak_armed {
                                            self.emit(i, DoorSound::Creak, who, if quiet { 0.12 } else { 0.3 });
                                            self.leaf[i].creak_armed = false;
                                        }
                                    }
                                    self.leaf[i].vel = sw * js_max(self.leaf[i].vel * sw, if quiet { PULL_VEL_QUIET } else { PULL_VEL });
                                }
                                self.leaf[i].closing = false;
                                self.leaf[i].swept = false;
                                self.leaf[i].pull_t = PULL_SECS;
                                touched = true;
                                pushed = true;
                                if key || !key_touched {
                                    self.leaf[i].last_who = who;
                                }
                                if key {
                                    key_touched = true;
                                }
                                if bit != 0 && !(solid && self.leaf[i].solid_t <= SOLID_TRIES - 1.0) {
                                    self.leaf[i].passers |= bit;
                                    here |= bit;
                                }
                                break;
                            }
                        }
                        // (DoorContact.round) he goes round a leaf standing wide open
                        if self.con[c].round == Some(true)
                            && !bash
                            && !self.leaf[i].latched
                            && !self.leaf[i].closing
                            && sgn * sw < 0.0
                            && self.leaf[i].angle * sw >= wide
                            && js_abs(self.leaf[i].vel) < ROUND_STILL
                            && self.leaf[i].open_t >= DRIVEN_SECS
                        {
                            if !pressed && self.leaf[i].press_t == 0.0 {
                                self.rounds += 1.0;
                            }
                            pressed = true;
                            if self.leaf[i].press_t < ROUND_SECS {
                                self.leaf[i].vel *= 0.5;
                                let k = shove(pen, dt, true) / js_max(dist, 1e-4);
                                let sl = ROUND_SLIDE * dt;
                                self.con[c].pos[0] += qx * k + u.0 * sl;
                                self.con[c].pos[2] += qz * k + u.1 * sl;
                                break;
                            }
                        }
                        let max_turn = TURN_RATE * dt;
                        let raw = sgn * (pen / lever) * 1.05;
                        let capped = js_abs(raw) > max_turn;
                        let want = self.leaf[i].angle + (if capped { js_sign(raw) * max_turn } else { raw });
                        if pulling && js_abs(want) > js_abs(self.leaf[i].angle) {
                            if bit != 0 {
                                self.leaf[i].passers |= bit;
                                here |= bit;
                            }
                            yank = true;
                            break;
                        }
                        let against = push_dir != 0.0 && sgn != push_dir;
                        if !weak && !key && against && !folded && self.opening(i, sgn) {
                            self.walked(i, c, sgn, lever);
                            walker = c;
                        }
                        if !weak && !key && against {
                            let k = shove(pen, dt, false) / js_max(dist, 1e-4);
                            self.con[c].pos[0] += qx * k;
                            self.con[c].pos[2] += qz * k;
                            break;
                        }
                        let na = clamp(want, self.min_a(i), self.max_a(i));
                        let moved = na - self.leaf[i].angle;
                        if js_abs(moved) > 1e-5 {
                            let to = self.rests_to(i);
                            if !weak && !folded && self.opening(i, js_sign(moved)) && !(self.leaf[i].drive.is_some() && self.drive(i).who != who && self.drive(i).t == 0.0 && self.drive(i).kind != Body) {
                                self.walked(i, c, moved, lever);
                                walker = c;
                            }
                            if !weak {
                                pushed = true;
                                if key {
                                    push_dir = sgn;
                                }
                                if moved * sw > 0.0 {
                                    self.leaf[i].open_t = 0.0;
                                }
                                self.leaf[i].swept = key && !bash && js_abs(na) < js_abs(self.leaf[i].angle);
                            }
                            if self.leaf[i].latched || to {
                                self.leaf[i].latched = false;
                                if self.leaf[i].creak_armed && !bash {
                                    self.emit(i, DoorSound::Creak, who, if quiet { 0.12 } else { 0.3 });
                                    self.leaf[i].creak_armed = false;
                                }
                            }
                            if bash && !(presses && has_hand) && self.leaf[i].since_bash > 0.9 && js_abs(self.leaf[i].angle) < 1.2 {
                                let open = has_run && sgn * sw > 0.0;
                                self.leaf[i].vel = sgn * (if open { run.map_or(7.5, |r| r.bash) } else { 7.5 });
                                self.leaf[i].since_bash = 0.0;
                                if open {
                                    self.con[c].bashed = Some(self.ids[i]);
                                    self.con[c].bashed_at = Some(self.leaf[i].angle);
                                    ran = true;
                                }
                                self.emit(i, DoorSound::Bash, who, 1.0);
                            } else if !kicked && !ran {
                                self.leaf[i].vel = if weak { self.leaf[i].vel * 0.5 } else { self.leaf[i].vel * 0.5 + (moved / js_max(dt, 1e-3)) * 0.5 };
                            }
                            self.leaf[i].angle = na;
                        }
                        if js_abs(want - na) > 1e-5 && (na == 0.0 || pen > r - STOP_BODY) {
                            let k = shove(if na == 0.0 { pen } else { pen - js_max(0.0, r - STOP_BODY) }, dt, false) / js_max(dist, 1e-4);
                            self.con[c].pos[0] += qx * k;
                            self.con[c].pos[2] += qz * k;
                            if key && !limp && !bash && !presses && na != 0.0 {
                                if bit != 0 && !self.leaf[i].latched && js_abs(self.leaf[i].angle) > 0.5 {
                                    self.leaf[i].passers |= bit;
                                    here |= bit;
                                }
                            }
                        }
                        if capped {
                            break;
                        }
                    } else {
                        // the hinge region, or beyond the tip: a post
                        if pulling {
                            if bit != 0 {
                                self.leaf[i].passers |= bit;
                                here |= bit;
                            }
                            yank = true;
                            break;
                        }
                        let k = shove(pen, dt, false) / js_max(dist, 1e-4);
                        self.con[c].pos[0] += qx * k;
                        self.con[c].pos[2] += qz * k;
                        if key && !limp && !bash && along >= wd && !self.leaf[i].latched && js_abs(self.leaf[i].angle) > 0.5 {
                            if bit != 0 {
                                self.leaf[i].passers |= bit;
                                here |= bit;
                            }
                        }
                        break;
                    }
                }
            }
        }
        self.leaf[i].since_bash += dt;
        self.leaf[i].passers &= here;
        {
            self.leaf[i].press_t = if pressed { self.leaf[i].press_t + dt } else { 0.0 };
            self.leaf[i].open_t += dt;
            if self.leaf[i].solid_t > 0.0 {
                self.leaf[i].solid_t = js_max(0.0, self.leaf[i].solid_t - SOLID_FADE * dt);
            }
        }
        if yank {
            self.leaf[i].closing = true;
            if js_sign(self.leaf[i].angle) * self.leaf[i].vel > -3.0 {
                self.leaf[i].vel = -js_sign(self.leaf[i].angle) * 3.0;
            }
            self.leaf[i].quiet_t = 0.0;
        } else if touched {
            self.leaf[i].quiet_t = 0.0;
            self.leaf[i].closing = false;
        } else if !body_block {
            self.leaf[i].quiet_t += dt;
        }
        // free swing with friction, limit stops
        if (!touched || kicked || yank) && !self.leaf[i].latched {
            if !kicked {
                self.leaf[i].angle += self.leaf[i].vel * dt;
            }
            self.leaf[i].vel *= exp(-2.2 * dt);
            let (lo, hi) = (self.min_a(i), self.max_a(i));
            if self.leaf[i].angle >= hi || self.leaf[i].angle <= lo {
                let lim = if self.leaf[i].angle >= hi { hi } else { lo };
                self.leaf[i].angle = lim;
                let vel = self.leaf[i].vel;
                if lim == 0.0 {
                    if vel * sw < -2.5 {
                        self.emit(i, DoorSound::Bang, self.leaf[i].last_who, js_min(1.0, js_abs(vel) / 8.0));
                    }
                    if vel * sw < 0.0 {
                        self.leaf[i].vel = 0.0;
                    }
                } else {
                    if js_abs(vel) > 2.5 {
                        self.emit(i, DoorSound::Bang, self.leaf[i].last_who, js_min(1.0, js_abs(vel) / 8.0));
                    }
                    self.leaf[i].vel = if js_sign(lim) * vel > 0.0 { -vel * 0.25 } else { vel };
                }
            }
            let closing = self.leaf[i].closing;
            if closing && !body_block {
                let acc = -self.leaf[i].angle * 5.0 - self.leaf[i].vel * 4.2;
                self.leaf[i].vel += acc * dt;
            }
            if body_block {
                self.leaf[i].vel *= exp(-12.0 * dt);
            }
            if js_abs(self.leaf[i].angle) < 0.012 && js_abs(self.leaf[i].vel) < 0.35 && self.leaf[i].quiet_t > 0.3 && self.leaf[i].passers == 0 {
                if self.leaf[i].swept && !self.leaf[i].closing {
                    self.leaf[i].angle = 0.0;
                    self.leaf[i].vel = 0.0;
                    self.leaf[i].creak_armed = true;
                } else {
                    self.leaf[i].angle = 0.0;
                    self.leaf[i].vel = 0.0;
                    self.leaf[i].latched = true;
                    self.leaf[i].creak_armed = true;
                    self.leaf[i].closing = false;
                    self.leaf[i].swept = false;
                    self.leaf[i].pull_t = 0.0;
                    self.emit(i, DoorSound::Latch, self.leaf[i].last_who, 0.16);
                }
            }
        }
        if self.leaf[i].quiet_t > 1.2 {
            self.leaf[i].creak_armed = true;
        }
        // the lock
        if self.leaf[i].picking {
            self.leaf[i].picking = false;
            self.leaf[i].pick_idle = 0.0;
        } else if self.leaf[i].pick > 0.0 {
            self.leaf[i].pick_idle += dt;
            if self.leaf[i].pick_idle > 2.0 {
                self.leaf[i].pick = js_max(0.0, self.leaf[i].pick - dt / 4.0);
            }
        }
        if self.leaf[i].rattle_t > 0.0 {
            self.leaf[i].rattle_t = js_max(0.0, self.leaf[i].rattle_t - dt);
        }
        if self.leaf[i].pull_t > 0.0 {
            self.leaf[i].pull_t = js_max(0.0, self.leaf[i].pull_t - dt);
        }
        if let Some(d) = &mut self.leaf[i].drive {
            d.t += dt;
            if walker != usize::MAX && d.kind == Body && d.who == self.con[walker].who {
                (d.px, d.pz) = (self.con[walker].pos[0], self.con[walker].pos[2]);
            }
        }
        self.place(i);
    }
    /// what the game knows of the man who last had door i (handed in; a step that asks where the host saw no rule that could is wrong, and says so)
    fn man_last(&mut self, i: usize) -> i32 {
        self.man_last[i].expect("the man of a door's last hand was not asked of the game")
    }
    /// the step-back's test: do the statics let him stand at (x, z)
    fn free(&self, x: f64, z: f64) -> bool {
        let (mut qx, mut qz) = (x, z);
        self.settle(&mut qx, &mut qz);
        hypot2(qx - x, qz - z) < 0.05
    }
}

/// the correction a leaf applies to a living contact this frame
#[inline]
fn shove(pen: f64, dt: f64, exact: bool) -> f64 {
    if exact {
        pen + 0.001
    } else {
        js_min(pen + 0.001, SHOVE_RATE * dt)
    }
}


/// Resolve contacts, integrate swing, closer, latch; then what the leaves made heard and where their boxes stand.
pub fn doors_update(g: &mut Game, dt: f64, contacts: &mut Vec<DoorContact>) {
    for c in contacts.iter_mut() {
        c.blocked_by = None;
        if c.run.is_some() { c.bashed = None; c.ripped = None; }
        c.struck = None;
        if c.moved == Some(true) { c.moved = Some(false); }
        c.man = None;
    }
    if g.doors.list.is_empty() { return; }   // (a level with no door has no step, only that reset)
    let leaves = g.col.dynamic_boxes.clone();
    debug_assert!(leaves.len() == g.doors.list.len() && g.doors.list.iter().zip(leaves.iter()).all(|(&d, &b)| g.dr[d].box_ == b), "the leaves in the box world are not this level's doors");
    crate::scene_collision::rules_world(g, &leaves);
    let (ids, n, nc) = (g.doors.list.clone(), g.doors.list.len(), contacts.len());
    let slab: Vec<Option<f64>> = ids.iter().map(|&d| g.dr[d].slab.map(|s| s.ang)).collect();
    // (asked only where a rule of this step can want it: something was written onto the leaf since the last one)
    let man_last = ids.iter().map(|&d| {
        let l = g.dr[d].leaf;
        if l.vel != l.vel_seen || l.angle != l.angle_seen || l.angle * door_swing(g, d) < 0.0 { Some(crate::doorthrow::door_man(g, l.last_who) as i32) } else { None }
    }).collect();
    let asked = contacts.iter().map(|k| Asked {
        man: if k.weak != Some(true) { crate::doorthrow::door_man(g, k.who) as i32 } else { 0 },
        opens: k.keys.as_ref().map_or(0, |keys| ids.iter().enumerate().filter(|(_, &d)| crate::squadcast::ring_has(keys, &g.dr[d].lock_id)).fold(0, |b, (i, _)| b | 1u64 << i)),
    }).collect();
    let man = crate::cmd::seat_man(g, g.seats[0]);   // (the head of the span the settled man is pushed out of the statics over, of the first seat's man)
    let mut s = Step {
        dr: ids.iter().map(|&d| &g.dr[d]).collect(), leaf: ids.iter().map(|&d| g.dr[d].leaf).collect(), ids: &ids, con: contacts, asked, slab, man_last,
        k: g.k.doorthrow, world: crate::scene_world::world(), w: g.col.mirror.slot as usize, dt, n, nc,
        foot: crate::scene_collision::foot_of(FLOOR_Y0), settle_y1: if g.pl[man].crouch { 1.0 } else { 1.7 },
        placed: vec![None; n], ev: Vec::new(), rounds: 0.0,
    };
    for i in 0..n { s.door(i); }
    let (leaf, placed, ev, rounds) = (s.leaf, s.placed, s.ev, s.rounds);
    for (i, &d) in ids.iter().enumerate() { g.dr[d].leaf = leaf[i]; }
    g.doors.rounds += rounds;
    // (a listener may ask the world, which copies the leaves from their boxes over the rows the step placed: so the rows are read first)
    let rows: Vec<Option<Vec<f64>>> = (0..n).map(|i| placed[i].map(|_| crate::scene_box_mirror::box_mirror_leaf_row(g, i as f64))).collect();
    if !ev.is_empty() { crate::scene_box_mirror::box_mirror_leaves_turned(g); }
    let mut heard = ev.iter().peekable();
    for i in 0..n {
        while let Some((_, e)) = heard.next_if(|e| e.0 == i) {
            doors_sound(g, e.door, e.sound, e.pos, e.who, e.level);
        }
        let (Some(r), Some(handles)) = (&rows[i], placed[i]) else { continue };
        let (b, hb) = (g.dr[ids[i]].box_, g.dr[ids[i]].handle_boxes.clone());
        let yaw = crate::scene_box_mirror::box_mirror_follow_row(g, b, r);
        for s in 0..2 { let h = &mut g.tab.boxes[hb[s]]; h.c = handles[s]; h.yaw = yaw; }
    }
    if !ev.is_empty() { crate::scene_box_mirror::box_mirror_leaves_turned(g); }
}

/// somebody drove `d` open
pub fn doors_driven(g: &mut Game, d: DoorId, who: f64, dir: f64, kind: DoorDriveKind, quiet: bool, rate: f64) {
    let man = crate::doorthrow::door_man(g, who) as i32;
    g.dr[d].leaf.driven(who, dir, kind, quiet, rate, man);
}
