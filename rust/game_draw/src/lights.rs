//! THE LIGHTS, in the order that decides which of them the renderer has room for; the infrared ones after the rest. A light that moves stands between where the
//! two ticks left it on a frame drawn between them. A flash that lived one tick inside a frame of several is drawn once all the same.
use alloc::vec::Vec;
use game::game::Game;
use game::ids::*;

section! {
    /// a light
    pub struct Light {
        /// which light it is, for as long as it burns
        pub key: u32,
        /// 0 a point, 1 a spot, 2 the sky's, 3 a panel
        pub kind: u32,
        /// what it is by day (render/photometry.ts lightClass): 0 drawn as it is, 1 a fixture, 2 in a hand, 3 a vehicle's, 4 a flash, 5 a glowing face
        pub class: u32,
        /// 1 it burns · 2 transient · 4 broad · 8 a carried spot · 16 infrared · 32 it moves (a hand's, Sam's own)
        pub says: u32,
        pub owner: u32,
        /// the marked box and the mark it is the light of (under 0: none)
        pub mark: i32,
        pub pos: [f32; 3],
        pub dir: [f32; 3],
        pub color: [f32; 3],
        pub intensity: f32,
        pub peak: f32,
        pub range: f32,
        pub inner: f32,
        pub outer: f32,
        pub radius: f32,
        pub volumetric: f32,
        pub flicker: f32,
        pub ttl: f32,
        pub age: f32,
        /// its class under the fog (0 none, 1 only what the team sees, 2 only what it does not), and the strength it is remembered at where it is out
        pub fog: f32,
        pub ghost: f32,
    }
}

const SNAP: f64 = 1.5;
/// ticks a flash nobody has seen is kept for (the most a frame runs)
const HELD: u32 = 4;

#[derive(Default)]
pub struct Lights {
    /// drawn with the fog off: each as it is
    pub bared: bool,
    /// as they are drawn
    pub rows: Vec<Light>,
    now: Vec<Light>,
    before: Vec<Light>,
    /// the timed lights no frame has shown yet, and for how many ticks
    unseen: Vec<(u32, u32)>,
    have: u32,
}

impl Lights {
    pub fn cut(&mut self) { self.have = 0; self.unseen.clear(); }
    pub fn hold(&mut self) { self.have = self.have.min(1); }

    /// a tick has run (`again`: the tick already taken)
    pub fn take(&mut self, g: &Game, again: bool) {
        if !again { core::mem::swap(&mut self.now, &mut self.before); }
        self.now.clear();
        let l = &g.engine.lights;
        for &id in &l.lights { self.now.push(row(g, id, false, self.bared)); }
        for &id in &l.ir { self.now.push(row(g, id, true, self.bared)); }
        // (a flash that went out before any frame showed it is shown by the next, if that comes within a hitch's worth of ticks)
        let spent: Vec<Light> = self.before.iter().filter(|b| self.unseen.iter().any(|u| u.0 == b.key && u.1 < HELD) && b.age >= b.ttl && !self.now.iter().any(|n| n.key == b.key)).copied().collect();
        if !again {
            for u in &mut self.unseen { u.1 += 1; }
            self.unseen.retain(|u| u.1 <= HELD);
            for n in &self.now { if n.ttl >= 0.0 && n.says & 1 != 0 && n.intensity > 0.0 && !self.before.iter().any(|b| b.key == n.key) { self.unseen.push((n.key, 0)); } }
        }
        let at = self.now.iter().position(|n| n.says & 16 != 0).unwrap_or(self.now.len());
        for (k, s) in spent.into_iter().enumerate() { self.now.insert(at + k, s); }
        if again { return; }
        self.have = (self.have + 1).min(2);
        self.before.sort_unstable_by_key(|b| b.key);
    }

    /// the lights in the hands of a man drawn ahead of the tick
    pub fn lead(&mut self, by: &crate::lead::Moved) {
        for r in &mut self.rows { if by.carries(r.owner, r.ttl) { r.pos = by.pt32(r.pos); r.dir = by.dir(r.dir.map(|v| v as f64)).map(|v| v as f32); } }
    }
    pub fn frame(&mut self, a: f64) {
        self.rows.clone_from(&self.now);
        self.unseen.clear();
        if a >= 1.0 || self.have < 2 { return; }
        for r in &mut self.rows {
            let Ok(i) = self.before.binary_search_by_key(&r.key, |b| b.key) else { continue };
            let p = &self.before[i];
            let d: [f32; 3] = core::array::from_fn(|k| r.pos[k] - p.pos[k]);
            if (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]) as f64 > SNAP * SNAP { continue; }
            let a = a as f32;
            r.pos = core::array::from_fn(|k| p.pos[k] + d[k] * a);
            let v: [f32; 3] = core::array::from_fn(|k| p.dir[k] + (r.dir[k] - p.dir[k]) * a);
            let n = game::m::sqrt((v[0] * v[0] + v[1] * v[1] + v[2] * v[2]) as f64) as f32;
            if n > 1e-9 { r.dir = v.map(|x| x / n); }
        }
    }
}

fn row(g: &Game, id: LightId, ir: bool, bared: bool) -> Light {
    let l = &g.tab.light[id];
    let f3 = |v: [f64; 3]| v.map(|x| x as f32);
    let hand = l.group == "flashlights";
    Light {
        key: id.ix() as u32,
        kind: l.kind as u32,
        class: game::render_lights::light_by_day(l),
        says: l.enabled as u32 | ((l.ttl >= 0.0 || l.transient == Some(true)) as u32) << 1 | (l.broad as u32) << 2 | (l.card as u32) << 3 | (ir as u32) << 4 | ((hand || l.group == "player") as u32) << 5,
        owner: l.owner as u32,
        mark: l.mark_ref.map_or(-1, |m| m as i32),
        pos: f3(l.pos), dir: f3(l.dir), color: f3(l.color),
        intensity: l.intensity as f32, peak: l.peak_intensity as f32, range: l.range as f32, inner: l.inner_deg as f32, outer: l.outer_deg as f32, radius: l.radius as f32,
        volumetric: l.volumetric as f32, flicker: l.flicker as f32, ttl: l.ttl as f32, age: l.age as f32,
        fog: if ir || bared { 0.0 } else { game::render_lights::light_set_fog_class(g, id) as f32 },
        ghost: if bared { 0.0 } else { l.fog_ghost.unwrap_or(0.0) as f32 },
    }
}
