//! THE DOORS' STEP: every leaf of a level for one tick. What was written onto it since the last one, the men and the bodies in its swing (who turns it, whom
//! it moves, whom it passes), its free swing, its stops, its closer, its latch and its lock's clocks; and then where its box stands.
use super::*;
use crate::game::Game;
use crate::ids::*;
use crate::m::{self, cos, exp, floor, hyp2 as hypot2, sin, sqrt};
use crate::v3::{clamp, V3};
use alloc::{vec, vec::Vec};
use ::sim::{collide, world::{asking, js_abs, js_max, js_min, World}};
use DoorDriveKind::*;
mod meet;

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
    w: &'a World,
    dt: f64,
    n: usize,
    nc: usize,
    foot: f64,
    settle_y1: f64,
    /// where each leaf that moved now stands, and whether it seals its doorway
    placed: Vec<Option<(crate::doors::DoorPoseAt, bool)>>,
    ev: Vec<(usize, DoorEvent)>,
    rounds: f64,
}

/// one leaf's tick: what its stages tell each other
#[derive(Default)]
struct Turn {
    i: usize,
    /// the way it opens
    sw: f64,
    /// a boot or a run has it flying · somebody has eased it further open · it is on its way open
    kicked: bool,
    eased: bool,
    comes: bool,
    /// a man is on it, has turned it, or presses at it; a key holder was the one; a body lies against it
    touched: bool,
    pushed: bool,
    pressed: bool,
    key_touched: bool,
    body_block: bool,
    /// the way a key holder turns it
    push_dir: f64,
    /// one it was closing on pulls it to · a run took it open
    yank: bool,
    ran: bool,
    /// those walking through it, a bit a man, and the contact that last walked it open
    here: i32,
    walker: Option<usize>,
}

impl Step<'_> {
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
    /// a leaf found on the far side of its frame is turned back to the side it opens to
    fn keep_its_side(&mut self, i: usize) {
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
        let pt = asking().pt;
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

    /// The leaf where it now stands, and its handles
    fn place(&mut self, i: usize) {
        if self.leaf[i].off {
            return;
        }
        self.keep_its_side(i);
        self.leaf[i].vel_seen = self.leaf[i].vel;
        self.leaf[i].angle_seen = self.leaf[i].angle;
        let rt = self.leaf[i].rattle_t;
        let jig = if rt > 0.35 { sin(rt * 90.0) * 0.005 } else { 0.0 };
        let d = self.dr[i];
        let p = pose_of(d.def.hinge, d.def.width, d.def.closed_dir, d.thick, d.height, self.leaf[i].angle + jig);
        self.placed[i] = Some((p, js_abs(self.leaf[i].angle) < 0.3));
    }

    /// the step of door i
    fn door(&mut self, i: usize) {
        if self.leaf[i].off { return self.lies_off(i); }
        self.swept_until_written(i);
        self.keep_its_side(i);
        let sw = self.swing(i);
        self.speed_written(i, sw);
        let eased = (self.leaf[i].angle - self.leaf[i].angle_seen) * sw > 1e-4;
        if eased { self.leaf[i].open_t = 0.0; }
        self.drive_written(i);
        let kicked = self.leaf[i].since_bash < 0.6;
        if kicked && !self.leaf[i].latched { self.flies(i); }
        let comes = self.leaf[i].vel * sw > 0.0 || (self.leaf[i].angle - self.leaf[i].angle_seen) * sw > 1e-4;
        let mut t = Turn { i, sw, kicked, eased, comes, ..Turn::default() };
        self.stand_back(&t);
        self.reach_for(&mut t);
        self.meets(&mut t);
        self.hands_clocks(&t);
        self.swings(&t);
        self.lock_clocks(i);
        if let Some(d) = &mut self.leaf[i].drive {
            d.t += self.dt;
            if let Some(w) = t.walker.filter(|&w| d.kind == Body && d.who == self.con[w].who) { (d.px, d.pz) = (self.con[w].pos[0], self.con[w].pos[2]); }
        }
        self.place(i);
    }
    /// off its hinges: it reads as its slab lies
    fn lies_off(&mut self, i: usize) {
        let l = &mut self.leaf[i];
        (l.vel, l.latched, l.closing) = (0.0, false, false);
        if let Some(a) = self.slab[i] { l.angle = a; }
    }
    /// what a key holder's push swept it to is so until somebody writes on it
    fn swept_until_written(&mut self, i: usize) {
        let l = &mut self.leaf[i];
        if l.vel != l.vel_seen || l.angle != l.angle_seen { l.swept = false; }
    }
    /// a speed written on it since its last step: a shove on the face it opens toward is a pull; so is one open by a man who stands at that face, and by anybody else it is a push, just now
    fn speed_written(&mut self, i: usize, sw: f64) {
        let l = &mut self.leaf[i];
        if l.vel == l.vel_seen || l.closing || l.latched { return; }
        if l.vel * sw < 0.0 {
            if l.angle * sw < SHOVE_OPENS_BELOW {
                l.vel = -l.vel;
                l.pull_t = PULL_SECS;
            }
            return;
        }
        if !(l.vel * sw > 0.0) { return; }
        let (last, d) = (l.last_who, &self.dr[i].def);
        let by = self.con[..self.nc].iter().find(|k| k.weak != Some(true) && k.who == last);
        let pulls = by.is_some_and(|k| self.pull_face(i, k.pos[0], k.pos[2]) && hypot2(k.pos[0] - d.hinge[0], k.pos[2] - d.hinge[1]) < d.width + 1.2);
        let l = &mut self.leaf[i];
        if pulls { l.pull_t = PULL_SECS; } else { l.open_t = 0.0; }
    }
    /// whose drive that writing is: a swing it was given, or a hand turning it
    fn drive_written(&mut self, i: usize) {
        let l = &mut self.leaf[i];
        let pulled = l.pull_t > 0.0;
        if pulled { l.drive = None; }
        if l.closing || l.latched || pulled { return; }
        let (vel, turn) = (l.vel, l.angle - l.angle_seen);
        let swung = vel != l.vel_seen && vel != 0.0;
        let drive = l.drive;
        if swung && self.opening(i, js_sign(vel)) {
            if drive.is_some_and(|d| d.t == 0.0 && d.kind == Boot) { return; }
            let m = self.man_last(i);
            let l = &mut self.leaf[i];
            l.driven(l.last_who, vel, Swing, false, 0.0, m);
            return;
        }
        let rate = js_abs(turn) / js_max(self.dt, 1e-3);
        let goes_on = drive.is_some_and(|d| d.kind == Hand && d.t < DRIVEN_SECS);
        if turn != 0.0 && rate >= (if goes_on { 0.5 } else { 1.0 }) * self.k.hand_rate && rate <= TURN_RATE && self.opening(i, js_sign(turn)) {
            let m = self.man_last(i);
            let l = &mut self.leaf[i];
            l.driven(l.last_who, turn, Hand, false, rate, m);
        }
    }
    /// a kicked leaf moves before anybody meets it, and stops at the first it presses on
    fn flies(&mut self, i: usize) {
        let from = self.leaf[i].angle;
        let to = clamp(from + self.leaf[i].vel * self.dt, self.min_a(i), self.max_a(i));
        let total = to - from;
        let n = -floor(-(js_abs(total) / KICK_SUBSTEP));
        if n <= 1.0 {
            self.leaf[i].angle = to;
            return;
        }
        let mut k = 0.0;
        while k < n {
            self.leaf[i].angle += total / n;
            if self.leaf_presses(i) { break; }
            k += 1.0;
        }
    }
    /// who is still in it, how long it has been pressed, and whether it is let go of
    fn hands_clocks(&mut self, t: &Turn) {
        let dt = self.dt;
        let l = &mut self.leaf[t.i];
        l.since_bash += dt;
        l.passers &= t.here;
        l.press_t = if t.pressed { l.press_t + dt } else { 0.0 };
        l.open_t += dt;
        if l.solid_t > 0.0 { l.solid_t = js_max(0.0, l.solid_t - SOLID_FADE * dt); }
        if t.yank {
            l.closing = true;
            if js_sign(l.angle) * l.vel > -3.0 { l.vel = -js_sign(l.angle) * 3.0; }
            l.quiet_t = 0.0;
        } else if t.touched {
            l.quiet_t = 0.0;
            l.closing = false;
        } else if !t.body_block {
            l.quiet_t += dt;
        }
    }
    /// its free swing with friction, its stops, its closer and its latch
    fn swings(&mut self, t: &Turn) {
        let (i, dt) = (t.i, self.dt);
        if (!t.touched || t.kicked || t.yank) && !self.leaf[i].latched {
            if !t.kicked { self.leaf[i].angle += self.leaf[i].vel * dt; }
            self.leaf[i].vel *= exp(-2.2 * dt);
            self.at_a_stop(t);
            let l = &mut self.leaf[i];
            if l.closing && !t.body_block { l.vel += (-l.angle * 5.0 - l.vel * 4.2) * dt; }
            if t.body_block { l.vel *= exp(-12.0 * dt); }
            if js_abs(l.angle) < 0.012 && js_abs(l.vel) < 0.35 && l.quiet_t > 0.3 && l.passers == 0 { self.comes_to(i); }
        }
        if self.leaf[i].quiet_t > 1.2 { self.leaf[i].creak_armed = true; }
    }
    /// at its frame it stops dead, at its far stop it comes back a little; hard enough either bangs
    fn at_a_stop(&mut self, t: &Turn) {
        let i = t.i;
        let (lo, hi) = (self.min_a(i), self.max_a(i));
        if !(self.leaf[i].angle >= hi || self.leaf[i].angle <= lo) { return; }
        let lim = if self.leaf[i].angle >= hi { hi } else { lo };
        let vel = self.leaf[i].vel;
        self.leaf[i].angle = lim;
        let hard = if lim == 0.0 { vel * t.sw < -2.5 } else { js_abs(vel) > 2.5 };
        if hard {
            self.emit(i, DoorSound::Bang, self.leaf[i].last_who, js_min(1.0, js_abs(vel) / 8.0));
        }
        if lim == 0.0 {
            if vel * t.sw < 0.0 { self.leaf[i].vel = 0.0; }
        } else if js_sign(lim) * vel > 0.0 {
            self.leaf[i].vel = -vel * 0.25;
        }
    }
    /// shut: on its latch, unless a push only swept it to
    fn comes_to(&mut self, i: usize) {
        let l = &mut self.leaf[i];
        (l.angle, l.vel, l.creak_armed) = (0.0, 0.0, true);
        if l.swept && !l.closing { return; }
        (l.latched, l.closing, l.swept, l.pull_t) = (true, false, false, 0.0);
        self.emit(i, DoorSound::Latch, self.leaf[i].last_who, 0.16);
    }
    /// a pick left alone slips back; a rattle and a pull wear off
    fn lock_clocks(&mut self, i: usize) {
        let dt = self.dt;
        let l = &mut self.leaf[i];
        if l.picking {
            (l.picking, l.pick_idle) = (false, 0.0);
        } else if l.pick > 0.0 {
            l.pick_idle += dt;
            if l.pick_idle > 2.0 { l.pick = js_max(0.0, l.pick - dt / 4.0); }
        }
        if l.rattle_t > 0.0 { l.rattle_t = js_max(0.0, l.rattle_t - dt); }
        if l.pull_t > 0.0 { l.pull_t = js_max(0.0, l.pull_t - dt); }
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
        k: g.k.doorthrow, world: crate::scene_world::world(), w: g.tab.boxes.world(), dt, n, nc,
        foot: crate::scene_collision::foot_of(FLOOR_Y0), settle_y1: if g.pl[man].crouch { 1.0 } else { 1.7 },
        placed: vec![None; n], ev: Vec::new(), rounds: 0.0,
    };
    for i in 0..n { s.door(i); }
    let (leaf, placed, ev, rounds) = (s.leaf, s.placed, s.ev, s.rounds);
    for (i, &d) in ids.iter().enumerate() { g.dr[d].leaf = leaf[i]; }
    g.doors.rounds += rounds;
    // (a leaf is put where it now stands as its own sounds have been heard: a listener may ask the world)
    let mut heard = ev.iter().peekable();
    for i in 0..n {
        while let Some((_, e)) = heard.next_if(|e| e.0 == i) {
            doors_sound(g, e.door, e.sound, e.pos, e.who, e.level);
        }
        if let Some((p, seal)) = &placed[i] { super::leaf::leaf_stands(g, ids[i], p, *seal); }
    }
}

/// somebody drove `d` open
pub fn doors_driven(g: &mut Game, d: DoorId, who: f64, dir: f64, kind: DoorDriveKind, quiet: bool, rate: f64) {
    let man = crate::doorthrow::door_man(g, who) as i32;
    g.dr[d].leaf.driven(who, dir, kind, quiet, rate, man);
}
