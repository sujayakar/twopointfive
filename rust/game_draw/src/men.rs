//! THE MEN DRAWN THIS TICK: who, dressed how, and his bones. What only the GPU reads of a man (his instance's words, his bones' matrices) is written as it reads
//! them; on a frame between two ticks his bones stand between where the two left them.
use crate::Row;
use alloc::vec::Vec;
use game::anim_gltf::CastSide;
use game::game::{Game, GuardState};
use game::ids::*;
use game::m;

/// bones a man has room for, and men the skinning has room for
const JOINTS: usize = 64;
const MOST: usize = 16;
/// proxies a man (src/scene/boxes.ts CAPS_PER_CHAR)
const CAPS: usize = 15;
const SNAP: f64 = 1.5;
/// a waterline this high is the whole man
const LINE_ALL: f64 = 1e4;

section! {
    /// a man drawn: what the page's own tables dress him by
    pub struct Man {
        pub id: u32,
        /// whose table he wears: 0 a civilian's, 1 an operator's, 2 a defender's, 3 a responder's; and whether he is in the sneaking suit
        pub side: u32,
        pub suit: u32,
        /// the cast's parts he wears, a bit a part (0: every one)
        pub parts: u32,
        pub alive: u32,
        pub tint: [f32; 3],
        pub tint2: [f32; 3],
        /// his height beside the skeleton's
        pub stature: f32,
        /// how much his proxies are darkened by a wet coat (1: dry)
        pub dry: f32,
        pub at: [f32; 3],
        /// where he is drawn standing: between the last two ticks, as his bones are
        pub feet: [f32; 3],
        /// the sphere round his proxies
        pub bound: [f32; 4],
    }
}
section! {
    /// a man's instance, as the skinning reads it: tint, bones' place · tint2, owner · wet coat · its knobs · the waterline, the parts he wears
    pub struct Inst {
        pub w: [u32; 24],
    }
}
section! {
    /// a man's bones: a matrix a bone
    pub struct Bones {
        pub w: [f32; 1024],
    }
}
section! {
    /// one of his proxies: an end and its radius, the other end, and whose it is
    pub struct Cap {
        pub w: [f32; 7],
        pub owner: u32,
    }
}

/// by a man's side: his instance takes its two tints the other way (bit 0 the first, bit 1 the second); said by the page's tables
pub type Swaps = [u32; 4];

#[derive(Default)]
pub struct Men {
    pub swaps: Swaps,
    pub men: Vec<Man>,
    pub insts: Vec<Inst>,
    pub bones: Vec<Bones>,
    pub caps: Vec<Cap>,
    now: Vec<Bones>,
    now_caps: Vec<Cap>,
    before: Vec<(u32, Bones, [f32; 3])>,
    before_caps: Vec<Cap>,
    have: u32,
}

impl Men {
    pub fn cut(&mut self) { self.have = 0; }

    /// a tick has run (`again`: the tick already taken)
    pub fn take(&mut self, g: &Game, again: bool) {
        if !again {
            self.before.clear();
            for (k, r) in self.men.iter().enumerate() { self.before.push((r.id, self.now[k], r.at)); }
            core::mem::swap(&mut self.now_caps, &mut self.before_caps);
            self.have = (self.have + 1).min(2);
        }
        self.men.clear();
        self.insts.clear();
        self.now.clear();
        self.now_caps.clear();
        for c in shown(g).into_iter().take(MOST) { self.man(g, c); }
    }

    fn man(&mut self, g: &Game, id: CharId) {
        let c = &g.ch[id];
        // (a man with no pose yet has no bones to be drawn by)
        if c.skel.joint_data.is_empty() || c.bones.get(game::character::Bone::Hips).is_none() { return; }
        let (side, suit) = dress(g, id);
        let s = game::stature::stature_of(g, id);
        let f3 = |v: [f64; 3]| v.map(|x| x as f32);
        let wet = g.engine.settings.wet_chars && (c.wet > 0.0 || c.beads > 0.0);
        let caps = game::character::character_capsules(g, id);
        let x = |p: [f64; 3]| -> [f64; 3] { if s == 1.0 { p } else { core::array::from_fn(|k| c.pos[k] + (p[k] - c.pos[k]) * s) } };
        let at = self.men.len();
        let b = caps.bound;
        let bc = x([b[0], b[1], b[2]]);
        self.men.push(Man {
            id: c.id as u32, side: side as u32, suit: suit as u32, parts: c.parts as u32, alive: c.alive as u32, tint: f3(c.tint), tint2: f3(c.tint2), stature: s as f32,
            dry: if g.engine.settings.wet_chars && c.wet > 0.0 { 1.0 - 0.35 * c.wet } else { 1.0 } as f32, at: f3(c.pos), feet: f3(c.pos), bound: [bc[0] as f32, bc[1] as f32, bc[2] as f32, (b[3] * s) as f32],
        });
        // ---- his instance
        let w = |v: f64| (v as f32).to_bits();
        let swap = self.swaps[side as usize];
        let (t, t2) = (if swap & 1 != 0 { c.tint2 } else { c.tint }, if swap & 2 != 0 { c.tint2 } else { c.tint });
        let mut i = Inst::ZERO;
        for k in 0..3 { i.w[k] = w(t[k]); i.w[4 + k] = w(t2[k]); }
        i.w[3] = (at * JOINTS) as u32;
        i.w[7] = c.id as u32;
        if wet {
            let k = &g.k.wet;
            let phase = { let p = c.id * 0.7548777; (p - m::floor(p)) * 40.0 };
            let line = if c.wet_line.is_finite() { m::max(0.0, c.wet_line) } else { LINE_ALL };
            let row = [c.wet, c.beads, c.wet_rain, phase, k.bead_density, k.bead_size, k.trail_len, k.rough_wet, k.rough_dry, k.darken, k.darken_mul, k.spec_gain, line, m::max(0.005, k.line_band)];
            for (n, v) in row.iter().enumerate() { i.w[8 + n] = w(*v); }
        }
        i.w[22] = c.parts as u32;
        self.insts.push(i);
        // ---- his bones, scaled about his feet where he is drawn taller or shorter
        let mut bones = Bones::ZERO;
        let src = &c.skel.joint_data[..c.skel.joint_data.len().min(JOINTS * 16)];
        if s == 1.0 { bones.w[..src.len()].copy_from_slice(src); } else {
            let off = c.pos.map(|v| v * (1.0 - s));
            for (o, mat) in src.chunks_exact(16).enumerate() {
                let d = &mut bones.w[o * 16..o * 16 + 16];
                for k in 0..16 { d[k] = if k % 4 == 3 { mat[k] } else { (mat[k] as f64 * s + if k >= 12 { off[k - 12] } else { 0.0 }) as f32 }; }
            }
        }
        self.now.push(bones);
        // ---- his proxies
        for k in 0..CAPS {
            let mut r = Cap { w: [0.0, -100.0, 0.0, 0.0, 0.0, -100.0, 0.0], owner: c.id as u32 };
            if let Some(v) = caps.caps.get(k) {
                let (a, z) = if v[3] > 0.0 { (x([v[0], v[1], v[2]]), x([v[4], v[5], v[6]])) } else { ([v[0], v[1], v[2]], [v[4], v[5], v[6]]) };
                r.w = [a[0] as f32, a[1] as f32, a[2] as f32, (if v[3] > 0.0 { v[3] * s } else { v[3] }) as f32, z[0] as f32, z[1] as f32, z[2] as f32];
            }
            self.now_caps.push(r);
        }
    }

    pub fn frame(&mut self, a: f64) {
        self.bones.clone_from(&self.now);
        self.caps.clone_from(&self.now_caps);
        for r in &mut self.men { r.feet = r.at; }
        if a >= 1.0 || self.have < 2 { return; }
        let a = a as f32;
        for (k, r) in self.men.iter_mut().enumerate() {
            let Some(j) = self.before.iter().position(|b| b.0 == r.id) else { continue };
            let (p, q) = (&self.before[j].1, &mut self.bones[k]);
            let d: [f32; 3] = core::array::from_fn(|i| q.w[12 + i] - p.w[12 + i]);
            if (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]) as f64 > SNAP * SNAP { continue; }
            let was = self.before[j].2;
            for i in 0..3 { r.feet[i] = was[i] + (r.at[i] - was[i]) * a; }
            for i in 0..JOINTS * 16 { q.w[i] = p.w[i] + (q.w[i] - p.w[i]) * a; }
            for i in 0..CAPS {
                let (p, q) = (&self.before_caps[j * CAPS + i], &mut self.caps[k * CAPS + i]);
                if p.w[3] > 0.0 && q.w[3] > 0.0 { for n in [0, 1, 2, 4, 5, 6] { q.w[n] = p.w[n] + (q.w[n] - p.w[n]) * a; } }
            }
        }
    }
}

/// the players' men, one a seat
fn players(g: &Game) -> impl Iterator<Item = CharId> + '_ { g.seats.iter().map(|&s| g.pl[game::cmd::seat_man(g, s)].ch) }

/// the men who are drawn, in the order they take the skinning's places
fn shown(g: &Game) -> Vec<CharId> {
    let all: Vec<CharId> = players(g).chain(g.guards.iter().map(|&gd| g.gd[gd].ch)).chain(game::civilians::civilians_shown(g).iter().map(|&gd| g.gd[gd].ch)).collect();
    let mut drawn = game::fogofwar::fog_of_war_filter(g, &all);
    // (from the sky nobody on the floor is the player)
    if game::rails::rails_sky_on(g) { let me = g.pl[game::seats::host_man(g)].ch; drawn.retain(|c| *c != me); }
    // (more men than have a mesh of their own to throw a shadow by: the men down give up their places first)
    if drawn.len() > 8 && (g.night_breach.active || g.rescue.active) {
        let down = |c: &CharId| g.guards.iter().any(|&gd| g.gd[gd].ch == *c && g.gd[gd].state == GuardState::Dead);
        drawn.sort_by_key(down);
    }
    drawn
}

/// whose table a man is dressed by, and whether he is in the sneaking suit
fn dress(g: &Game, c: CharId) -> (CastSide, bool) {
    if players(g).any(|p| p == c) {
        let side = g.m.cast.worn.get(c).map_or(CastSide::Operator, |&v| g.tab.anim_gltf_cast_variant[v].side);
        return (side, suit(g, c, true));
    }
    if let Some(&gd) = g.guards.iter().find(|&&gd| g.gd[gd].ch == c) { return (game::char_look::side_of_team(g.gd[gd].team, Some(g.gd[gd].trained)), suit(g, c, false)); }
    (CastSide::Civilian, false)
}
fn suit(g: &Game, c: CharId, player: bool) -> bool {
    if !g.k.look.on { return false; }
    let quiet = match game::scene_look::suit_as(g) {
        Some(asked) => asked.map(|_| true),
        None => game::scenario_run::active_scenario(g).map(|a| a == game::scenario_run::ActiveScenario::Quiet),
    };
    quiet != Some(false) && (player || (quiet == Some(true) && game::scenario_run::team_of(g).iter().any(|gd| g.gd[*gd].ch == c)))
}
