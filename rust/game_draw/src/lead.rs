//! ONE MAN DRAWN AHEAD OF THE TICK: a networked host's own man, where the records the game has not had yet take him (the host says where: net.rs `face_ahead`). Everything of his that is
//! drawn is moved as one body from where the tick has him, a move and a turn about the vertical through his feet: his bones, his proxies, what he wears and carries, the lights in his hands, where he is
//! looked for in the dark. His pose is the tick's, and what he has done at a place (a round in flight, a flash, a charge) stays where it was done.

/// a man's place further off than this in one tick was a cut, and he is not flown to it
const SNAP: f64 = 1.5;
/// a motion key's kind that is a man's own box (src/scene/boxes.ts MKEY.rig)
const RIG: u32 = 4;

/// where a man stands: his feet, and the way he aims
#[derive(Clone, Copy, Default)]
pub struct Stand { pub pos: [f64; 3], pub yaw: f64 }
/// where the tick has him, and where he is drawn
type Pair = (Stand, Stand);

#[derive(Default)]
pub struct Lead { id: u32, have: u32, before: Pair, now: Pair }

impl Lead {
    /// the end of a tick: where it has man `id`, and where his records take him
    pub fn tick(&mut self, id: u32, from: Stand, to: Stand) {
        if id != self.id { *self = Lead { id, ..Lead::default() }; }
        self.before = self.now;
        self.now = (from, to);
        self.have = (self.have + 1).min(2);
    }
    /// nobody is ahead
    pub fn rest(&mut self) { *self = Lead::default(); }
    /// the move on a frame `a` of the way from the tick before to the last
    pub fn at(&self, a: f64) -> Option<Moved> {
        if self.have == 0 || self.id & 0xff == 0 { return None; }
        let mix = |p: Stand, q: Stand| {
            let d: [f64; 3] = core::array::from_fn(|k| q.pos[k] - p.pos[k]);
            if self.have < 2 || a >= 1.0 || d[0] * d[0] + d[1] * d[1] + d[2] * d[2] > SNAP * SNAP { return q; }
            Stand { pos: core::array::from_fn(|k| p.pos[k] + d[k] * a), yaw: p.yaw + crate::turn(p.yaw, q.yaw) * a }
        };
        let (from, to) = (mix(self.before.0, self.now.0), mix(self.before.1, self.now.1));
        let turn = crate::turn(from.yaw, to.yaw);
        Some(Moved { id: self.id, from: from.pos, to: to.pos, turn, c: game::m::cos(turn), s: game::m::sin(turn) })
    }
}

/// a man moved as one body
pub struct Moved { pub id: u32, from: [f64; 3], to: [f64; 3], pub turn: f64, c: f64, s: f64 }
impl Moved {
    /// a direction of his, turned
    pub fn dir(&self, d: [f64; 3]) -> [f64; 3] { [d[0] * self.c + d[2] * self.s, d[1], d[2] * self.c - d[0] * self.s] }
    /// a point of his, moved
    pub fn pt(&self, p: [f64; 3]) -> [f64; 3] {
        let d = self.dir(core::array::from_fn(|k| p[k] - self.from[k]));
        core::array::from_fn(|k| self.to[k] + d[k])
    }
    pub fn pt32(&self, p: [f32; 3]) -> [f32; 3] { self.pt(p.map(|v| v as f64)).map(|v| v as f32) }
    /// a turn of his, turned
    pub fn rot(&self, q: [f64; 4]) -> [f64; 4] {
        let (s, c) = (game::m::sin(self.turn / 2.0), game::m::cos(self.turn / 2.0));
        [c * q[0] + s * q[2], c * q[1] + s * q[3], c * q[2] - s * q[0], c * q[3] - s * q[1]]
    }
    /// is the box of this motion key one he wears or carries
    pub fn wears(&self, key: u32) -> bool { key != crate::boxes::NO_KEY && key >> 24 == RIG && key >> 8 & 0xff == self.id & 0xff }
    /// is this light one in his hands: his, and with no life of its own to run out
    pub fn carries(&self, owner: u32, ttl: f32) -> bool { owner & 0xff == self.id & 0xff && !(ttl >= 0.0) }
    /// a matrix of his (its columns, one after another), moved
    pub fn mat(&self, m: &mut [f32]) {
        for col in 0..4 {
            let v = [m[col * 4], m[col * 4 + 1], m[col * 4 + 2]];
            let v = if col == 3 { self.pt32(v) } else { self.dir(v.map(|x| x as f64)).map(|x| x as f32) };
            m[col * 4..col * 4 + 3].copy_from_slice(&v);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn stand(x: f64, z: f64, yaw: f64) -> Stand { Stand { pos: [x, 0.0, z], yaw } }

    /// THE RULE: a man ahead is his own body moved and turned about his feet, and half way between two ticks he is half way between their two leads.
    #[test]
    fn he_is_moved_as_one_body() {
        let mut l = Lead::default();
        assert!(l.at(1.0).is_none());
        l.tick(7, stand(1.0, 0.0, 0.0), stand(1.5, 0.0, 0.0));
        l.tick(7, stand(2.0, 0.0, 0.0), stand(2.5, 1.0, core::f64::consts::FRAC_PI_2));
        let m = l.at(1.0).unwrap();
        assert_eq!(m.pt([2.0, 0.0, 0.0]), [2.5, 0.0, 1.0], "his feet are where the lead has them");
        // (a quarter turn of the yaw takes what is before him, +z, to +x)
        let p = m.pt([2.0, 1.4, 0.5]);
        assert!((p[0] - 3.0).abs() < 1e-9 && (p[1] - 1.4).abs() < 1e-9 && (p[2] - 1.0).abs() < 1e-9, "{p:?}");
        let mut mat = [0.0f32; 16];
        for k in 0..4 { mat[k * 5] = 1.0; }
        (mat[12], mat[13], mat[14]) = (2.0, 1.4, 0.5);
        m.mat(&mut mat);
        assert!((mat[12] - 3.0).abs() < 1e-5 && (mat[14] - 1.0).abs() < 1e-5 && (mat[2] + 1.0).abs() < 1e-5, "{mat:?}");
        let half = l.at(0.5).unwrap();
        assert_eq!(half.pt([1.5, 0.0, 0.0]), [2.0, 0.0, 0.5]);
        l.tick(9, stand(0.0, 0.0, 0.0), stand(0.0, 0.0, 0.0));
        assert!(l.at(0.5).unwrap().wears(4 << 24 | 9 << 8 | 3) && !l.at(0.5).unwrap().wears(2 << 24 | 9 << 8), "another man begins afresh");
        l.rest();
        assert!(l.at(1.0).is_none());
    }
}
