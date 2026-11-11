//! A MAN OR A BODY AGAINST A LEAF: who stands back from its swing, whose hand is on it before his body, who turns it, whom it moves or strikes, and who walks through.
use super::*;

/// nearer the hinge than this, or past the tip, a leaf is a post and not a lever
const LEVER_FROM: f64 = 0.14;
/// a run or a bash with no speed of its own
const BASH_VEL: f64 = 7.5;

/// where something stands by a leaf: the leaf's own line, how far along it from the hinge and how far across, and the line's nearest point
struct Near {
    u: (f64, f64),
    along: f64,
    perp: f64,
    /// `along`, on the leaf
    t: f64,
    /// from the nearest point to it, and how far
    qx: f64,
    qz: f64,
    dist: f64,
}
/// a contact that overlaps a leaf
struct Meet {
    c: usize,
    who: f64,
    r: f64,
    near: Near,
    /// how deep
    pen: f64,
    /// a body, with nobody standing in it · a key holder · running or kicking · easing by · moved by the leaf as a wall would
    weak: bool,
    key: bool,
    bash: bool,
    quiet: bool,
    solid: bool,
    /// the passer's bit, the game's words of the man, and whether he is folded
    bit: i32,
    man: i32,
    folded: bool,
    /// this leaf struck him a moment ago
    hit: bool,
    /// on the length of it that turns
    on_leaf: bool,
}
impl Meet {
    /// the way out of the leaf, and how much of it clears him
    fn out(&self) -> (f64, f64, f64) {
        let Near { u, perp, qx, qz, dist, .. } = self.near;
        if dist >= 1e-4 { return (qx, qz, (self.pen + 0.001) / dist); }
        let s = side_of(perp);
        (-u.1 * s, u.0 * s, self.pen + 0.001)
    }
    /// he works no leaf: folded, or this one has just struck him
    fn limp(&self) -> bool { self.folded || self.hit }
}

impl Step<'_> {
    fn near(&self, i: usize, x: f64, z: f64) -> Near {
        let d = &self.dr[i].def;
        let u = self.dir(i);
        let (px, pz) = (x - d.hinge[0], z - d.hinge[1]);
        let along = px * u.0 + pz * u.1;
        let t = clamp(along, 0.0, d.width);
        let (qx, qz) = (px - u.0 * t, pz - u.1 * t);
        Near { u, along, perp: u.0 * pz - u.1 * px, t, qx, qz, dist: hypot2(qx, qz) }
    }
    fn near_contact(&self, i: usize, c: usize) -> Near { self.near(i, self.con[c].pos[0], self.con[c].pos[2]) }
    fn on_leaf(&self, i: usize, along: f64, r: f64) -> bool { along > LEVER_FROM && along < self.dr[i].def.width + r }
    /// could this contact touch it at all, whichever way it stands
    fn in_reach(&self, i: usize, c: usize) -> bool {
        let d = &self.dr[i].def;
        let (dx, dz) = (self.con[c].pos[0] - d.hinge[0], self.con[c].pos[2] - d.hinge[1]);
        let reach = d.width + self.con[c].radius + 0.1;
        !(dx * dx + dz * dz > reach * reach)
    }
    fn weak(&self, c: usize) -> bool { self.con[c].weak == Some(true) }
    /// a man the leaf moves as a wall would, while it has tries left to
    fn solid_tries(&self, i: usize) -> bool { self.leaf[i].solid_t <= SOLID_TRIES - 1.0 }
    /// as wide as a hand still presses it
    fn wide(&self, t: &Turn) -> f64 {
        js_min(1.35, 0.8 * js_abs(if t.sw > 0.0 { self.max_a(t.i) } else { self.min_a(t.i) }))
    }
    /// is it locked to him, on the side its key is for
    fn refuses(&self, i: usize, c: usize, side: f64) -> bool {
        self.leaf[i].locked && self.leaf[i].latched && !self.opens(c, i) && side == self.dr[i].key_side
    }
    fn push(&mut self, m: &Meet, exact: bool, pen: f64) {
        let k = shove(pen, self.dt, exact) / js_max(m.near.dist, 1e-4);
        self.con[m.c].pos[0] += m.near.qx * k;
        self.con[m.c].pos[2] += m.near.qz * k;
    }
    fn passes(&mut self, t: &mut Turn, bit: i32) {
        self.leaf[t.i].passers |= bit;
        t.here |= bit;
    }
    /// he has it: a key holder before anybody else
    fn had_by(&mut self, t: &mut Turn, m: &Meet) {
        if m.key || !t.key_touched { self.leaf[t.i].last_who = m.who; }
        t.key_touched |= m.key;
    }
    /// off its latch, creaking as it goes
    fn unlatch(&mut self, i: usize, who: f64, quiet: bool, heard: bool) {
        self.leaf[i].latched = false;
        if self.leaf[i].creak_armed && heard {
            self.emit(i, DoorSound::Creak, who, if quiet { 0.12 } else { 0.3 });
            self.leaf[i].creak_armed = false;
        }
    }
    /// drawn open by its handle
    fn drawn(&mut self, t: &Turn, who: f64, quiet: bool) {
        if self.leaf[t.i].latched || self.rests_to(t.i) { self.unlatch(t.i, who, quiet, true); }
        self.leaf[t.i].vel = t.sw * js_max(self.leaf[t.i].vel * t.sw, if quiet { PULL_VEL_QUIET } else { PULL_VEL });
    }
    /// a hand has it and it is on its way open
    fn in_hand(&mut self, t: &mut Turn) {
        let l = &mut self.leaf[t.i];
        (l.closing, l.swept, l.pull_t) = (false, false, PULL_SECS);
        (t.touched, t.pushed) = (true, true);
    }

    /// does it press on anybody where it stands
    pub(super) fn leaf_presses(&self, i: usize) -> bool {
        if self.leaf[i].pull_t > 0.0 { return false; }
        (0..self.nc).any(|c| {
            if self.weak(c) || !self.in_reach(i, c) || self.pass_bit(c) & self.leaf[i].passers != 0 { return false; }
            let (r, n) = (self.con[c].radius, self.near_contact(i, c));
            self.on_leaf(i, n.along, r) && r + self.dr[i].thick / 2.0 - n.dist > 0.0
        })
    }

    // ---- before he is against it
    /// (DOORWAY.solid) he stands back from its swing
    pub(super) fn stand_back(&mut self, t: &Turn) {
        if !(self.leaf[t.i].pull_t > 0.0) || !t.comes || self.leaf[t.i].latched || !self.solid_tries(t.i) { return; }
        for c in 0..self.nc {
            if self.con[c].solid == Some(true) && !self.weak(c) { self.stands_back(t, c); }
        }
    }
    fn stands_back(&mut self, t: &Turn, c: usize) {
        let d = &self.dr[t.i].def;
        let (ax, az) = (cos(d.closed_dir), sin(d.closed_dir));
        let (cx, cz, r) = (self.con[c].pos[0], self.con[c].pos[2], self.con[c].radius);
        let (px, pz) = (cx - d.hinge[0], cz - d.hinge[1]);
        let a = px * ax + pz * az;
        let o = t.sw * (ax * pz - az * px);
        let rr = d.width + r + self.dr[t.i].thick / 2.0 + SOLID_SKIN;
        if o <= 0.0 || a < 0.0 || a * a + o * o >= rr * rr { return; }
        let u = self.dir(t.i);
        if side_of(u.0 * pz - u.1 * px) != t.sw { return; }
        // straight back off the door, or aside past its tip, whichever the floor lets him
        let back = (-az * t.sw, ax * t.sw, sqrt(rr * rr - a * a) - o);
        let aside = (ax, az, sqrt(rr * rr - o * o) - a);
        let Some((wx, wz, room)) = [back, aside].into_iter().find(|&(wx, wz, room)| self.free(cx + wx * room, cz + wz * room)) else { return };
        let own = self.con[c].vel.map_or(0.0, |v| js_max(0.0, -(wx * v[0] + wz * v[2])));
        let step = js_min(room, (SOLID_BACK + own) * self.dt);
        let (mut x, mut z) = (cx + wx * step, cz + wz * step);
        self.settle(&mut x, &mut z);
        self.con[c].pos[0] = x;
        self.con[c].pos[2] = z;
    }
    /// (DOORWAY.solid) his hand reaches it before his body
    pub(super) fn reach_for(&mut self, t: &mut Turn) {
        let l = &self.leaf[t.i];
        if l.pull_t > 0.0 || !self.solid_tries(t.i) || l.open_t < DRIVEN_SECS || l.vel * t.sw > DRIVEN_VEL || !(l.angle * t.sw < self.wide(t)) { return; }
        for c in 0..self.nc {
            let k = &self.con[c];
            if k.solid == Some(true) && k.vel.is_some() && !k.bash && !k.quiet && !self.weak(c) { self.reaches(t, c); }
        }
    }
    fn reaches(&mut self, t: &mut Turn, c: usize) {
        let (i, wd) = (t.i, self.dr[t.i].def.width);
        let (r, who, quiet) = (self.con[c].radius, self.con[c].who, self.con[c].quiet);
        let n = self.near_contact(i, c);
        let side = side_of(n.perp);
        let gap = js_abs(n.perp) - r - self.dr[i].thick / 2.0;
        if side != t.sw || gap <= 0.0 || gap > SOLID_REACH || n.along <= LEVER_FROM || n.along >= wd { return; }
        if self.refuses(i, c, side) { return; }
        // he walks at it, and meets it on its length
        let (vx, vz) = self.con[c].vel.map_or((0.0, 0.0), |v| (v[0], v[2]));
        let into = -side * (n.u.0 * vz - n.u.1 * vx);
        if into < SOLID_INTO || into < 0.5 * hypot2(vx, vz) { return; }
        let meets = n.along + (vx * n.u.0 + vz * n.u.1) * (gap / into);
        if meets <= 0.0 || meets >= wd { return; }
        t.pressed = true;
        if self.leaf[i].press_t < PRESS_SECS { return; }
        self.drawn(t, who, quiet);
        self.in_hand(t);
        self.leaf[i].last_who = who;
    }

    // ---- against it
    /// everybody against it: key holders, then any man, then bodies
    pub(super) fn meets(&mut self, t: &mut Turn) {
        for pass in 0..3u32 {
            for c in 0..self.nc {
                let rank = if self.weak(c) { 2 } else if self.con[c].key == Some(true) { 0 } else { 1 };
                if rank != pass || !self.in_reach(t.i, c) || (self.weak(c) && self.con[c].pos[1] - self.con[c].radius > self.dr[t.i].height) { continue; }
                for _ in 0..3 {
                    let Some(m) = self.meet(t.i, c) else { break };
                    if self.met(t, &m) { break; }
                }
            }
        }
    }
    /// this contact against it as they now stand; None clear of it
    fn meet(&mut self, i: usize, c: usize) -> Option<Meet> {
        let (weak, r, who, near) = (self.weak(c), self.con[c].radius, self.con[c].who, self.near_contact(i, c));
        let pen = r + self.dr[i].thick / 2.0 - near.dist;
        if pen <= (if weak { 0.002 } else { 0.0 }) { return None; }
        let (mut bit, mut man) = (0, 0);
        if !weak {
            (bit, man) = (self.pass_bit(c), self.man_of(c));
            self.con[c].man = Some(man as f64);
        }
        let k = &self.con[c];
        Some(Meet {
            c, who, r, pen, weak, bit, man, key: k.key == Some(true), bash: k.bash, quiet: k.quiet, solid: k.solid == Some(true), folded: man & MAN_FOLDED != 0,
            hit: !weak && self.leaf[i].drive.is_some_and(|d| d.hit == who && d.t < self.k.again), on_leaf: self.on_leaf(i, near.along, r), near,
        })
    }
    /// one go at it; true when this contact and the leaf are done with each other
    fn met(&mut self, t: &mut Turn, m: &Meet) -> bool {
        let i = t.i;
        if m.bit & self.leaf[i].passers != 0 {
            t.here |= m.bit;
            return true;
        }
        if self.leaf[i].pull_t > 0.0 && !self.leaf[i].latched && !m.weak && self.opened_over(t, m) { return true; }
        if m.weak && (self.leaf[i].latched || t.pushed || t.kicked || t.eased || self.leaf[i].pull_t > 0.0) {
            let k = m.pen / js_max(m.near.dist, 1e-4);
            self.con[m.c].pos[0] += m.near.qx * k;
            self.con[m.c].pos[2] += m.near.qz * k;
            return true;
        }
        if self.thrown_at(t, m) { return true; }
        if m.weak {
            t.body_block = true;
        } else {
            t.touched = true;
            self.had_by(t, m);
        }
        if !m.weak && m.on_leaf && self.refuses(i, m.c, side_of(m.near.perp)) {
            self.locked_to(t, m);
            return true;
        }
        // one it is closing on, who pulls it to after him
        let pulling = m.key && !m.limp() && !m.bash && self.leaf[i].closing && js_abs(self.leaf[i].angle) > 0.35;
        if m.on_leaf { self.turns_it(t, m, pulling) } else { self.by_the_post(t, m, pulling) }
    }
    /// a leaf on its way open by a hand: it moves a man it would move as a wall, or stops on him; any other walks through it
    fn opened_over(&mut self, t: &mut Turn, m: &Meet) -> bool {
        let i = t.i;
        if !m.solid || !self.solid_tries(i) {
            self.passes(t, m.bit);
            return true;
        }
        if side_of(m.near.perp) != t.sw || !t.comes || !m.on_leaf { return false; }
        if self.solid_carry(i, m) {
            if self.leaf[i].vel * t.sw > PULL_VEL { self.leaf[i].vel = t.sw * PULL_VEL; }
            return true;
        }
        let l = &mut self.leaf[i];
        l.solid_t += 1.0;
        l.angle -= t.sw * js_min(l.angle * t.sw, (m.pen + 0.001) / js_max(0.18, m.near.t));
        (l.vel, l.pull_t) = (0.0, 0.0);
        true
    }
    /// (DOORTHROW) a leaf somebody else drove open toward him: it strikes him, or carries one it struck or who is folded; true when it carried him
    fn thrown_at(&mut self, t: &Turn, m: &Meet) -> bool {
        let (i, into) = (t.i, side_of(m.near.perp));
        let Some(d) = self.leaf[i].drive.filter(|_| !m.weak) else { return false };
        if d.who == m.who || d.dir != into || self.leaf[i].closing || !m.on_leaf { return false; }
        let at_him = !d.quiet && (d.kind == Boot || d.side == 0.0 || d.side != (m.man & MAN_SIDE) as f64);
        let tier = if d.kind == Boot { DoorStrikeTier::Boot } else if t.kicked { DoorStrikeTier::Slam } else { DoorStrikeTier::Open };
        let held = !t.kicked && (d.kind == Body || d.kind == Hand);
        let fresh = held && d.t < DRIVEN_SECS;
        let speed = into * self.leaf[i].vel * m.near.t;
        let stands = !m.folded && m.man & (if tier == DoorStrikeTier::Open { MAN_HELD } else { MAN_BRACED }) == 0;
        let hard = if held { fresh && d.rate * m.near.t >= self.k.min_speed } else { speed >= self.k.min_speed };
        if at_him && d.hit != m.who && stands && hard {
            let n = if m.near.dist > 1e-4 { 1.0 / m.near.dist } else { 0.0 };
            let dir = if n != 0.0 && n == n { [m.near.qx * n, m.near.qz * n] } else { [-m.near.u.1 * into, m.near.u.0 * into] };
            let floor = tier == DoorStrikeTier::Boot && js_abs(self.leaf[i].angle - d.from) < self.k.floor_rad;
            self.con[m.c].struck = Some(DoorStrike { door: self.ids[i], by: d.who, tier, floor, dir });
            if let Some(d) = &mut self.leaf[i].drive { d.hit = m.who; }
            return self.carry(m);
        }
        m.limp() && (speed >= 0.2 || fresh) && self.carry(m)
    }
    fn locked_to(&mut self, t: &Turn, m: &Meet) {
        self.push(m, true, m.pen);
        self.con[m.c].blocked_by = Some(self.ids[t.i]);
        if self.leaf[t.i].creak_armed && m.pen > 0.015 {
            self.leaf[t.i].creak_armed = false;
            self.rattle(t.i, m.who, if m.quiet { 0.08 } else { 0.12 });
        }
    }
    /// the hinge region, or beyond the tip
    fn by_the_post(&mut self, t: &mut Turn, m: &Meet, pulling: bool) -> bool {
        if pulling {
            self.passes(t, m.bit);
            t.yank = true;
            return true;
        }
        self.push(m, false, m.pen);
        let l = &self.leaf[t.i];
        if m.key && !m.limp() && !m.bash && m.near.along >= self.dr[t.i].def.width && !l.latched && js_abs(l.angle) > 0.5 { self.passes(t, m.bit); }
        true
    }
    /// on its length: his hand opens it, he goes round it, pulls it to, gives way to a key holder, or his weight turns it
    fn turns_it(&mut self, t: &mut Turn, m: &Meet, pulling: bool) -> bool {
        let i = t.i;
        let sgn = if m.near.perp >= 0.0 { -1.0 } else { 1.0 };
        let lever = js_max(0.18, m.near.t);
        let l = &self.leaf[i];
        let presses = !m.weak && !m.limp() && sgn * t.sw < 0.0 && l.angle * t.sw < self.wide(t) && l.open_t >= DRIVEN_SECS && l.vel * t.sw <= DRIVEN_VEL;
        if presses && self.pressed_open(t, m) { return true; }
        if self.goes_round(t, m, sgn) { return true; }
        let max_turn = TURN_RATE * self.dt;
        let raw = sgn * (m.pen / lever) * 1.05;
        let capped = js_abs(raw) > max_turn;
        let want = self.leaf[i].angle + (if capped { js_sign(raw) * max_turn } else { raw });
        if pulling && js_abs(want) > js_abs(self.leaf[i].angle) {
            self.passes(t, m.bit);
            t.yank = true;
            return true;
        }
        if !m.weak && !m.key && t.push_dir != 0.0 && sgn != t.push_dir {   // a key holder has it going the other way
            if !m.folded && self.opening(i, sgn) { self.walked(t, m, sgn, lever); }
            self.push(m, false, m.pen);
            return true;
        }
        let to = clamp(want, self.min_a(i), self.max_a(i));
        if js_abs(to - self.leaf[i].angle) > 1e-5 { self.turned(t, m, to, sgn, presses); }
        if js_abs(want - to) > 1e-5 && (to == 0.0 || m.pen > m.r - STOP_BODY) {   // it is at its stop: he is
            self.push(m, false, if to == 0.0 { m.pen } else { m.pen - js_max(0.0, m.r - STOP_BODY) });
            let l = &self.leaf[i];
            if m.key && !m.limp() && !m.bash && !presses && to != 0.0 && !l.latched && js_abs(l.angle) > 0.5 { self.passes(t, m.bit); }
        }
        capped
    }
    /// a hand on the face it opens toward: pressed long enough it is drawn open, or ripped by one at a run; true once it is
    fn pressed_open(&mut self, t: &mut Turn, m: &Meet) -> bool {
        let i = t.i;
        let (hand, run) = (self.con[m.c].hand, self.con[m.c].run);
        t.pressed = true;
        let hauled = m.bash && hand.is_none() && self.leaf[i].since_bash > 0.9;
        if !hauled && !(self.leaf[i].press_t >= hand.unwrap_or(PRESS_SECS)) { return false; }
        if hauled {
            self.leaf[i].latched = false;
            self.leaf[i].vel = t.sw * run.map_or(BASH_VEL, |r| r.rip);
            self.leaf[i].since_bash = 0.0;
            if run.is_some() { self.con[m.c].ripped = Some(self.ids[i]); }
            self.emit(i, DoorSound::Bash, m.who, run.map_or(1.0, |r| r.rip_level));
        } else {
            self.drawn(t, m.who, m.quiet);
        }
        self.in_hand(t);
        self.had_by(t, m);
        if !(m.solid && self.solid_tries(i)) { self.passes(t, m.bit); }
        true
    }
    /// (DoorContact.round) he goes round a leaf standing wide open
    fn goes_round(&mut self, t: &mut Turn, m: &Meet, sgn: f64) -> bool {
        let l = &self.leaf[t.i];
        let open = !l.latched && !l.closing && l.angle * t.sw >= self.wide(t) && js_abs(l.vel) < ROUND_STILL && l.open_t >= DRIVEN_SECS;
        if self.con[m.c].round != Some(true) || m.bash || !open || !(sgn * t.sw < 0.0) { return false; }
        if !t.pressed && self.leaf[t.i].press_t == 0.0 { self.rounds += 1.0; }
        t.pressed = true;
        if !(self.leaf[t.i].press_t < ROUND_SECS) { return false; }
        self.leaf[t.i].vel *= 0.5;
        let k = shove(m.pen, self.dt, true) / js_max(m.near.dist, 1e-4);
        let sl = ROUND_SLIDE * self.dt;
        self.con[m.c].pos[0] += m.near.qx * k + m.near.u.0 * sl;
        self.con[m.c].pos[2] += m.near.qz * k + m.near.u.1 * sl;
        true
    }
    /// his weight or his boot turns it to `to`
    fn turned(&mut self, t: &mut Turn, m: &Meet, to: f64, sgn: f64, presses: bool) {
        let i = t.i;
        let moved = to - self.leaf[i].angle;
        let rested = self.rests_to(i);
        let anothers = self.leaf[i].drive.is_some_and(|d| d.who != m.who && d.t == 0.0 && d.kind != Body);
        if !m.weak && !m.folded && self.opening(i, js_sign(moved)) && !anothers { self.walked(t, m, moved, js_max(0.18, m.near.t)); }
        if !m.weak {
            t.pushed = true;
            if m.key { t.push_dir = sgn; }
            let l = &mut self.leaf[i];
            if moved * t.sw > 0.0 { l.open_t = 0.0; }
            l.swept = m.key && !m.bash && js_abs(to) < js_abs(l.angle);
        }
        if self.leaf[i].latched || rested { self.unlatch(i, m.who, m.quiet, !m.bash); }
        if m.bash && !(presses && self.con[m.c].hand.is_some()) && self.leaf[i].since_bash > 0.9 && js_abs(self.leaf[i].angle) < 1.2 {
            self.bashed(t, m, sgn);
        } else if !t.kicked && !t.ran {
            let vel = self.leaf[i].vel;
            self.leaf[i].vel = if m.weak { vel * 0.5 } else { vel * 0.5 + (moved / js_max(self.dt, 1e-3)) * 0.5 };
        }
        self.leaf[i].angle = to;
    }
    fn bashed(&mut self, t: &mut Turn, m: &Meet, sgn: f64) {
        let i = t.i;
        let run = self.con[m.c].run.filter(|_| sgn * t.sw > 0.0);
        self.leaf[i].vel = sgn * run.map_or(BASH_VEL, |r| r.bash);
        self.leaf[i].since_bash = 0.0;
        if run.is_some() {
            self.con[m.c].bashed = Some(self.ids[i]);
            self.con[m.c].bashed_at = Some(self.leaf[i].angle);
            t.ran = true;
        }
        self.emit(i, DoorSound::Bash, m.who, 1.0);
    }
    /// he walks it open: a drive of his body's, at the rate his own feet give it
    fn walked(&mut self, t: &mut Turn, m: &Meet, dir: f64, lever: f64) {
        t.walker = Some(m.c);
        let (i, pos) = (t.i, self.con[m.c].pos);
        let d = self.leaf[i].drive.filter(|d| d.who == m.who);
        if d.is_some_and(|d| d.kind == Boot) && self.leaf[i].since_bash < 0.6 { return; }
        let d = d.filter(|d| d.kind == Body && d.dir == side_of(dir) && d.t <= 2.0 * self.dt + 1e-6 && d.px == d.px);
        let rate = d.map_or(0.0, |d| js_min(hypot2(pos[0] - d.px, pos[2] - d.pz) / js_max(self.dt, 1e-3) / lever, TURN_RATE));
        self.leaf[i].driven(m.who, dir, Body, m.quiet, rate, m.man);
    }
    /// moved out of it, and along it where the floor lets him; false where he can't be moved that far
    fn carry(&mut self, m: &Meet) -> bool {
        if m.pen > self.k.rate * self.dt { return false; }
        let (nx, nz, k) = m.out();
        let (u, sl, r) = (m.near.u, self.k.slide * self.dt, m.r);
        let (x, z) = (self.con[m.c].pos[0] + nx * k, self.con[m.c].pos[2] + nz * k);
        let Some((x, z)) = [(x + u.0 * sl, z + u.1 * sl), (x, z)].into_iter().find(|&(x, z)| self.standable(x, z, r)) else { return false };
        self.con[m.c].pos[0] = x;
        self.con[m.c].pos[2] = z;
        self.con[m.c].moved = Some(true);
        true
    }
    /// moved out of it and along it as a wall would move him; false where the statics leave him in it
    fn solid_carry(&mut self, i: usize, m: &Meet) -> bool {
        if m.pen > SOLID_RATE * self.dt { return false; }
        let (nx, nz, k) = m.out();
        let u = m.near.u;
        let own = self.con[m.c].vel.map_or(0.0, |v| js_max(0.0, -(v[0] * u.0 + v[2] * u.1)));
        let sl = (SOLID_SLIDE + own) * self.dt;
        let (mut x, mut z) = (self.con[m.c].pos[0] + nx * k + u.0 * sl, self.con[m.c].pos[2] + nz * k + u.1 * sl);
        self.settle(&mut x, &mut z);
        if m.r + self.dr[i].thick / 2.0 - self.near(i, x, z).dist > SOLID_SKIN { return false; }
        self.con[m.c].pos[0] = x;
        self.con[m.c].pos[2] = z;
        true
    }
}
