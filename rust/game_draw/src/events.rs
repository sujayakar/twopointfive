//! WHAT A TICK DID THAT IS DRAWN ONCE: each kind of event a section of its own, each row with the id that names it (`event!`). A host plays an id once; one that
//! does not come again when its tick is run again un-happened. Nothing that is still there a moment later is an event: the picture is made right by the other
//! sections alone, and these only make it prompt.
//!
//! What the air and the smoke are asked for is two things, whoever asks: a push of the air, and a source of smoke. A source that also drives the air names its
//! push.
use alloc::vec::Vec;
use game::effects::Track;
use game::game::Game;
use game::out_events::OutEvent as E;
use game::smoke_types::{SmokeEmitterLattice, SmokeEmitterMode, SmokePush, SmokePushLattice};

/// no number said: the reader's own
const NONE: f32 = f32::NAN;
fn opt(v: Option<f64>) -> f32 { v.map_or(NONE, |x| x as f32) }
fn f3(v: [f64; 3]) -> [f32; 3] { v.map(|x| x as f32) }

event! {
    /// particles thrown
    pub struct Emit {
        /// render/particles.ts kinds, by their number in PKINDS; how many; and 1 = the team's own doing
        pub kind: u32,
        pub count: u32,
        pub ours: u32,
        /// EMIT_SHAPES, one more than its number (0: not said); the owner, and the seed (not said: 0xffffffff)
        pub shape: u32,
        pub owner: u32,
        pub seed: u32,
        pub pos: [f32; 3],
        pub dir: [f32; 3],
        pub speed: f32,
        pub life: f32,
        pub size: f32,
        /// spread, speed_jit, life_jit, size_jit, lift, radius, pos_jit, top, aspect, tumble, nrm_jit, steam, ember (NaN: not said)
        pub more: [f32; 13],
        pub vadd: [f32; 3],
        pub color: [f32; 3],
    }
}
event! {
    /// what lies about is kicked
    pub struct Kick {
        /// a bit a kind (0: the usual); 1 = a wall between stops it
        pub kinds: u32,
        pub occlude: u32,
        pub seed: u32,
        pub pos: [f32; 3],
        pub radius: f32,
        pub strength: f32,
        pub up: f32,
        pub bias: [f32; 3],
        pub eye: [f32; 3],
    }
}
event! {
    /// the air is pushed
    pub struct AirPush {
        /// 0 at a point, 1 along a line, 2 by a standing body, 3 a jet, 4 outward
        pub shape: u32,
        pub pos: [f32; 3],
        /// the velocity the air is taken to, and which way the shape lies
        pub vel: [f32; 3],
        pub dir: [f32; 3],
        pub radius: f32,
        /// half the line, or half the body's height
        pub len: f32,
        /// how hard (a second, at the middle), and how much the air is made to part there (NaN: not said)
        pub hard: f32,
        pub expand: f32,
        /// how long it goes on (0: the one tick)
        pub ttl: f32,
        /// 0 every lattice, 1 the fine one, 2 the coarse
        pub lattice: u32,
    }
}
event! {
    /// smoke comes from somewhere, for a while
    pub struct SmokeSource {
        /// SMOKE_KINDS
        pub kind: u32,
        /// the push that goes with it, one more than its serial in this tick (0: none)
        pub push: u32,
        /// what it rides, if anything (`anchors`; 0: it stays)
        pub anchor: u32,
        /// 1 the picture's alone · 2 gone when what it rides is · 4 its ride's direction is the smoke's velocity
        pub says: u32,
        pub lattice: u32,
        pub pos: [f32; 3],
        pub radius: f32,
        pub density: f32,
        pub heat: f32,
        pub ttl: f32,
        pub age: f32,
        /// NaN: not said
        pub attack: f32,
        pub jitter: f32,
        pub prio: f32,
        pub min_cells: f32,
        pub tint: [f32; 3],
    }
}
section! {
    /// where something rides, for as long as a source rides it
    pub struct Anchor {
        pub key: u32,
        pub pos: [f32; 3],
        pub dir: [f32; 3],
    }
}

#[derive(Default)]
pub struct Events {
    pub emits: Vec<Emit>,
    pub kicks: Vec<Kick>,
    pub pushes: Vec<AirPush>,
    pub sources: Vec<SmokeSource>,
    pub anchors: Vec<Anchor>,
    /// what the live sources ride: the anchor's key, the thing, and how long it is asked for
    rides: Vec<(u32, Track, f64)>,
    keys: u32,
}

impl Events {
    pub fn cut(&mut self) { self.rides.clear(); }

    /// what the tick that has just run told its host
    pub fn take(&mut self, g: &Game) {
        self.emits.clear();
        self.kicks.clear();
        self.pushes.clear();
        self.sources.clear();
        let tick = crate::tick(g);
        for e in &g.out { self.one(tick, g, e); }
        self.anchors.clear();
        let mut rides = core::mem::take(&mut self.rides);
        rides.retain(|(key, t, until)| {
            if g.time > *until { return false; }
            let Some((pos, dir)) = game::effects::track_at(g, t) else { return false };
            self.anchors.push(Anchor { key: *key, pos: f3(pos), dir: f3(dir) });
            true
        });
        self.rides = rides;
    }

    fn one(&mut self, tick: u32, g: &Game, e: &E) {
        match e {
            E::ParticlesEmit(e) => {
                let (o, serial) = (&e.o, self.emits.len() as u32);
                self.emits.push(Emit {
                    tick, serial, kind: e.kind as u32, count: e.count as u32, ours: e.ours as u32, shape: o.shape.as_ref().map_or(0, |s| *s as u32 + 1), owner: o.owner.unwrap_or(0.0) as u32,
                    seed: o.seed.map_or(u32::MAX, |s| s as u32), pos: f3(e.pos), dir: f3(e.dir), speed: o.speed as f32, life: o.life as f32, size: o.size as f32,
                    more: [o.spread, o.speed_jit, o.life_jit, o.size_jit, o.lift, o.radius, o.pos_jit, o.top, o.aspect, o.tumble, o.nrm_jit, o.steam, o.ember].map(opt),
                    vadd: o.vadd.map_or([NONE; 3], f3), color: o.color.map_or([NONE; 3], f3),
                });
            }
            E::ParticlesImpulse(e) => {
                let serial = self.kicks.len() as u32;
                let o = e.o.as_ref();
                self.kicks.push(Kick {
                    tick, serial, kinds: o.and_then(|o| o.kinds.as_ref()).map_or(0, |k| k.iter().fold(0, |m, k| m | 1 << *k as u32)), occlude: o.map_or(false, |o| o.occlude == Some(true)) as u32,
                    seed: o.and_then(|o| o.seed).map_or(u32::MAX, |s| s as u32), pos: f3(e.pos), radius: e.radius as f32, strength: e.strength as f32, up: opt(o.and_then(|o| o.up)),
                    bias: o.and_then(|o| o.bias).map_or([NONE; 3], f3), eye: o.and_then(|o| o.eye).map_or([NONE; 3], f3),
                });
            }
            E::SmokeSystemLikePush(e) => self.push(tick, &e.p),
            E::SmokeSystemLikeEmit(e) => {
                let e = &e.e;
                let lattice = match e.lattice { Some(SmokeEmitterLattice::Fine) => 1, Some(SmokeEmitterLattice::Coarse) => 2, None => 0 };
                // what it does to the air
                let air = e.speed != 0.0 || e.push.is_some() || e.expand.is_some();
                let push = if air {
                    let serial = self.pushes.len() as u32;
                    let burst = matches!(e.mode, Some(SmokeEmitterMode::Burst));
                    self.pushes.push(AirPush { tick, serial, shape: if burst { 4 } else { 3 }, pos: f3(e.pos), vel: f3(e.dir.map(|d| d * e.speed)), dir: f3(e.dir), radius: e.radius as f32, len: 0.0, hard: opt(e.push), expand: opt(e.expand), ttl: e.ttl as f32, lattice });
                    serial + 1
                } else { 0 };
                let anchor = match &e.track { None => 0, Some(t) => { self.keys += 1; self.rides.push((self.keys, t.clone(), g.time + e.ttl)); self.keys } };
                let serial = self.sources.len() as u32;
                self.sources.push(SmokeSource {
                    tick, serial, kind: e.kind as u32, push, anchor, says: (e.look == Some(true)) as u32 | (e.confined == Some(true)) as u32 * 2 | (e.track_vel == Some(true)) as u32 * 4, lattice,
                    pos: f3(e.pos), radius: e.radius as f32, density: e.density as f32, heat: e.temperature as f32, ttl: e.ttl as f32, age: e.age as f32,
                    attack: opt(e.attack), jitter: opt(e.jitter), prio: opt(e.prio), min_cells: opt(e.min_voxels), tint: e.tint.map_or([NONE; 3], f3),
                });
            }
            _ => {}
        }
    }

    fn push(&mut self, tick: u32, p: &SmokePush) {
        let serial = self.pushes.len() as u32;
        let (shape, len) = match (p.line, p.body) { (Some(l), _) if l > 0.0 => (1, l), (_, Some(b)) => (2, b), _ => (0, 0.0) };
        self.pushes.push(AirPush { tick, serial, shape, pos: f3(p.pos), vel: f3(p.dir.map(|d| d * p.speed)), dir: f3(p.dir), radius: p.radius as f32, len: len as f32, hard: opt(p.push), expand: NONE, ttl: 0.0, lattice: match p.lattice { Some(SmokePushLattice::Fine) => 1, Some(SmokePushLattice::Coarse) => 2, None => 0 } });
    }
}
