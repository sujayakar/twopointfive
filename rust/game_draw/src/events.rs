//! WHAT A TICK DID THAT IS DRAWN ONCE: each kind of event a section of its own, each row with the id that names it (`event!`). A host plays an id once; one that
//! does not come again when its tick is run again un-happened. Nothing that is still there a moment later is an event: the picture is made right by the other
//! sections alone, and these only make it prompt.
//!
//! What the air and the smoke are asked for is two things, whoever asks: a push of the air, and a source of smoke. A source that also drives the air names its
//! push.
use alloc::string::String;
use alloc::vec::Vec;
use game::effects::Track;
use game::game::Game;
use game::out_events::OutEvent as E;
use game::smoke_types::{SmokeEmitter, SmokeEmitterLattice, SmokeEmitterMode, SmokePush, SmokePushLattice};

/// no number said: the reader's own
const NONE: f32 = f32::NAN;
fn opt(v: Option<f64>) -> f32 { v.map_or(NONE, |x| x as f32) }
fn f3(v: [f64; 3]) -> [f32; 3] { v.map(|x| x as f32) }
/// `speed` m/s along `dir`, which says where and need not be a unit (a fire's leans with the wind): the solver takes the same two apart
fn along(dir: [f64; 3], speed: f64) -> [f32; 3] { let n = (dir[0] * dir[0] + dir[1] * dir[1] + dir[2] * dir[2]).sqrt(); f3(if n > 0.0 { dir.map(|d| d / n * speed) } else { [0.0; 3] }) }

/// a moving man as the air is told of him: from how fast (m/s); his capsule's radius, from `foot` over the floor to `above` over his hips; and a stride's kick, `back` behind and `low` under his hips,
/// `kick` of his speed over a radius of `kick_r`, left out while the sine that picks its side is under `gap`
struct Wake { least: f64, body: f64, foot: f64, above: f64, back: f64, low: f64, kick: f64, kick_r: f64, gap: f64 }
const WAKE: Wake = Wake { least: 0.25, body: 0.24, foot: 0.12, above: 0.5, back: 0.3, low: 0.2, kick: 0.9, kick_r: 0.17, gap: 0.35 };

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
        /// 0 at a point, 1 along a line, 2 by a standing body, 3 a jet, 4 outward, 5 a round's path, 6 a walking man's stride
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
        /// 1 the picture's alone · 2 gone when what it rides is · 4 its ride's direction is the smoke's velocity · 8 the men see through it too · 16 a fire's, for the preset that sheds those
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
event! {
    /// a sound
    pub struct Sound {
        /// its name's number (`face_draw_name`)
        pub name: u32,
        /// it has a place; 0 = it is in the ears
        pub placed: u32,
        pub pos: [f32; 3],
        /// NaN: not said
        pub gain: f32,
        pub rate: f32,
        pub lowpass: f32,
        pub pan: f32,
        pub wet: f32,
    }
}
event! {
    /// a footfall
    pub struct Footfall {
        pub own: u32,
        /// what it landed on, by its number in FLOORS
        pub floor: u32,
        pub pos: [f32; 3],
        pub loud: f32,
    }
}
section! {
    /// a thing that sounds for as long as it goes on, as it stands: a chair that rolls, a fire that burns, a can that pours, what hums in a room
    pub struct Lasting {
        /// the sound's name's number (`face_draw_name`), and which of that name it is
        pub name: u32,
        pub id: u32,
        pub at: [f32; 3],
        pub gain: f32,
        /// how hard it goes, 0 to 1: its level and pitch ride it (a fire at full heat two thirds)
        pub hard: f32,
        /// how long it has gone on, and how much longer it will (NaN: nobody knows)
        pub age: f32,
        pub left: f32,
        /// how far off there is nothing of it (0: as far as it carries)
        pub reach: f32,
    }
}
section! {
    /// what is heard all the while: one row
    pub struct Heard {
        /// the game's clock
        pub time: f32,
        /// the rain's level where the listener stands
        pub rain: f32,
        /// how tense the floor is, by the nerves of those who are not the player's own: what the score follows
        pub tension: f32,
        /// which run this is (a fresh one is the score's cue)
        pub run: u32,
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
    pub sounds: Vec<Sound>,
    pub footfalls: Vec<Footfall>,
    pub lasting: Vec<Lasting>,
    pub heard: Vec<Heard>,
    /// every name a sound has had, by its number
    pub names: Vec<String>,
    rain: f32,
    /// what the live sources ride: the anchor's key, the thing, and how long it is asked for
    rides: Vec<(u32, Track, f64)>,
    keys: u32,
    /// the first tick told since the game was put back
    since: Option<u32>,
}

impl Events {
    pub fn cut(&mut self) { self.rides.clear(); self.since = None; }

    /// what the tick that has just run told its host
    pub fn take(&mut self, g: &Game) {
        self.emits.clear();
        self.kicks.clear();
        self.pushes.clear();
        self.sources.clear();
        self.sounds.clear();
        self.footfalls.clear();
        let tick = crate::tick(g);
        for e in &g.out { self.one(tick, g, e); }
        self.heard.clear();
        self.heard.push(Heard { time: g.time as f32, rain: self.rain, tension: tension(g) as f32, run: g.mission.serial });
        self.lasting(g);
        // (what the marks and the feels threw this tick)
        if let Some(s) = game::render_decals::mark_set(g) { for e in &s.thrown { self.one(tick, g, e); } }
        self.wakes(tick, g);
        // of a game put back, at its first tick told: what lets smoke out since before it
        if *self.since.get_or_insert(tick) == tick {
            for e in game::sense::smoke_sources(g) { if !g.out.iter().any(|o| matches!(o, E::SmokeSystemLikeEmit(n) if n.e.pos == e.pos)) { self.source(tick, g, &e); } }
        }
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
            E::ParticlesEmit(e) => if e.o.ember.is_none() || !crate::far_for_a_fire(g, e.pos) { self.emit(tick, e) },
            E::ParticlesImpulse(e) => {
                let serial = self.kicks.len() as u32;
                let o = e.o.as_ref();
                self.kicks.push(Kick {
                    tick, serial, kinds: o.and_then(|o| o.kinds.as_ref()).map_or(0, |k| k.iter().fold(0, |m, k| m | 1 << *k as u32)), occlude: o.map_or(false, |o| o.occlude == Some(true)) as u32,
                    seed: o.and_then(|o| o.seed).map_or(u32::MAX, |s| s as u32), pos: f3(e.pos), radius: e.radius as f32, strength: e.strength as f32, up: opt(o.and_then(|o| o.up)),
                    bias: o.and_then(|o| o.bias).map_or([NONE; 3], f3), eye: o.and_then(|o| o.eye).map_or([NONE; 3], f3),
                });
            }
            E::GameAudioLikePlay(e) => {
                let name = self.name(&e.name);
                let (o, serial) = (e.opts.as_ref(), self.sounds.len() as u32);
                self.sounds.push(Sound {
                    tick, serial, name, placed: e.pos.is_some() as u32, pos: e.pos.map_or([0.0; 3], f3), gain: opt(e.gain),
                    rate: opt(o.and_then(|o| o.rate)), lowpass: opt(o.and_then(|o| o.lowpass)), pan: opt(o.and_then(|o| o.pan)), wet: opt(o.and_then(|o| o.wet)),
                });
            }
            E::GameAudioLikeFootstep(e) => {
                let serial = self.footfalls.len() as u32;
                self.footfalls.push(Footfall { tick, serial, own: e.is_player as u32, floor: e.mat as u32, pos: f3(e.pos), loud: e.loudness as f32 });
            }
            E::GameAudioLikeRain(e) => self.rain = e.level as f32,
            E::SmokeSystemLikePush(e) => self.push(tick, &e.p),
            E::SmokeSystemLikeRound(e) => { let serial = self.pushes.len() as u32; self.pushes.push(AirPush { tick, serial, shape: 5, pos: f3(e.from), vel: f3(e.dir), dir: f3(e.dir), radius: 0.0, len: e.len as f32, hard: NONE, expand: NONE, ttl: 0.0, lattice: 0 }); }
            E::SmokeSystemLikeEmit(e) => self.source(tick, g, &e.e),
            _ => {}
        }
    }

    /// a sound's name's number
    fn name(&mut self, name: &str) -> u32 { (match self.names.iter().position(|n| n == name) { Some(i) => i, None => { self.names.push(String::from(name)); self.names.len() - 1 } }) as u32 }

    /// what sounds for as long as it goes on
    fn lasting(&mut self, g: &Game) {
        use game::throwables as t;
        const FOREVER: f64 = f64::NAN;
        self.lasting.clear();
        for r in g.rolls.iter().filter(|r| r.voice > 0.0) { self.goes("chairRoll", r.voice as u32, [r.x, 0.05, r.z], game::game::roll_voice().gain, r.k, [FOREVER; 2], 0.0); }
        for (ix, &f) in g.targets.iter().enumerate() {
            if game::fire::is_fire(g, f) && game::fire::burns(g, f) { self.goes("fireBed", ix as u32, game::fire::mouth_of(g, f).pos, g.k.fire.bed, game::fire::heat_of(g, f) * 2.0 / 3.0, [FOREVER; 2], 0.0); }
        }
        for &i in g.items.items.iter().filter(|&&i| t::item_kind(g, i) == t::ItemKind::Smoke && t::item_fired(g, i) && t::item_emit_t(g, i) > 0.0) {
            // (one let go under other numbers than a can's now: nobody knows how long ago)
            let (left, all) = (t::item_emit_t(g, i), game::combat::can_params(g).canister_duration);
            self.goes("smokeHiss", t::item_serial(g, i) as u32, game::v3::add(t::item_pos(g, i), [0.0, 0.1, 0.0]), 1.0, 1.0, [if left > all { FOREVER } else { all - left }, left], 0.0);
        }
        let dark = game::sim_lightshome::mains_active(g);
        for (ix, h) in g.level.hums.iter().flatten().enumerate().filter(|(_, h)| !dark || h.mains == Some(false)) { self.goes(h.kind.as_str(), ix as u32, h.pos, h.level, 1.0, [FOREVER; 2], h.radius); }
    }
    fn goes(&mut self, name: &str, id: u32, at: [f64; 3], gain: f64, hard: f64, [age, left]: [f64; 2], reach: f64) {
        let name = self.name(name);
        self.lasting.push(Lasting { name, id, at: f3(at), gain: gain as f32, hard: hard as f32, age: age as f32, left: left as f32, reach: reach as f32 });
    }

    /// a source of smoke, and what it does to the air
    fn source(&mut self, tick: u32, g: &Game, e: &SmokeEmitter) {
        let lattice = match e.lattice { Some(SmokeEmitterLattice::Fine) => 1, Some(SmokeEmitterLattice::Coarse) => 2, None => 0 };
        let air = e.speed != 0.0 || e.push.is_some() || e.expand.is_some();
        let push = if air {
            let serial = self.pushes.len() as u32;
            let burst = matches!(e.mode, Some(SmokeEmitterMode::Burst));
            self.pushes.push(AirPush { tick, serial, shape: if burst { 4 } else { 3 }, pos: f3(e.pos), vel: along(e.dir, e.speed), dir: f3(e.dir), radius: e.radius as f32, len: 0.0, hard: opt(e.push), expand: opt(e.expand), ttl: e.ttl as f32, lattice });
            serial + 1
        } else { 0 };
        let anchor = match &e.track { None => 0, Some(t) => { self.keys += 1; self.rides.push((self.keys, t.clone(), g.time + e.ttl)); self.keys } };
        let serial = self.sources.len() as u32;
        self.sources.push(SmokeSource {
            tick, serial, kind: e.kind as u32, push, anchor, says: (e.look == Some(true)) as u32 | (e.confined == Some(true)) as u32 * 2 | (e.track_vel == Some(true)) as u32 * 4 | (e.held == Some(true)) as u32 * 8 | (e.fire == Some(true)) as u32 * 16, lattice,
            pos: f3(e.pos), radius: e.radius as f32, density: e.density as f32, heat: e.temperature as f32, ttl: e.ttl as f32, age: e.age as f32,
            attack: opt(e.attack), jitter: opt(e.jitter), prio: opt(e.prio), min_cells: opt(e.min_voxels), tint: e.tint.map_or([NONE; 3], f3),
        });
    }

    fn emit(&mut self, tick: u32, e: &game::out_events::OutParticlesEmit) {
        let (o, serial) = (&e.o, self.emits.len() as u32);
        self.emits.push(Emit {
            tick, serial, kind: e.kind as u32, count: e.count as u32, ours: e.ours as u32, shape: o.shape.as_ref().map_or(0, |s| *s as u32 + 1), owner: o.owner.unwrap_or(0.0) as u32,
            seed: o.seed.map_or(u32::MAX, |s| s as u32), pos: f3(e.pos), dir: f3(e.dir), speed: o.speed as f32, life: o.life as f32, size: o.size as f32,
            more: [o.spread, o.speed_jit, o.life_jit, o.size_jit, o.lift, o.radius, o.pos_jit, o.top, o.aspect, o.tumble, o.nrm_jit, o.steam, o.ember].map(opt),
            vadd: o.vadd.map_or([NONE; 3], f3), color: o.color.map_or([NONE; 3], f3),
        });
    }

    /// MEN WADING THROUGH THE AIR. His body is a standing capsule from his shins to his shoulders: the air inside it goes with him, and the solver sends what he displaces round his sides. His stride
    /// throws the air just behind him out to one side and then the other, a side a step, as his walk cycle has it.
    fn wakes(&mut self, tick: u32, g: &Game) {
        use game::character::Bone;
        let men = g.seats.iter().map(|&s| g.pl[game::cmd::seat_man(g, s)].ch).chain(g.guards.iter().map(|&m| g.gd[m].ch)).chain(game::civilians::civilians_shown(g).into_iter().map(|m| g.gd[m].ch));
        for c in men {
            let q = &g.ch[c];
            let sp = (q.vel[0] * q.vel[0] + q.vel[2] * q.vel[2]).sqrt();
            let Some(&h) = q.bones.get(Bone::Hips) else { continue };
            if sp < WAKE.least { continue; }
            let (hx, hz, top) = (q.vel[0] / sp, q.vel[2] / sp, h[1] + WAKE.above);
            let mut put = |shape: u32, pos: [f64; 3], dir: [f64; 3], speed: f64, radius: f64, len: f64| {
                let serial = self.pushes.len() as u32;
                self.pushes.push(AirPush { tick, serial, shape, pos: f3(pos), vel: along(dir, speed), dir: f3(dir), radius: radius as f32, len: len as f32, hard: NONE, expand: NONE, ttl: 0.0, lattice: 0 });
            };
            put(2, [h[0], (top + WAKE.foot) / 2.0, h[2]], [hx, 0.0, hz], sp, WAKE.body, ((top - WAKE.foot) / 2.0 - WAKE.body).max(0.0));
            let side = (2.0 * core::f64::consts::PI * q.anim.phase).sin();
            if side.abs() > WAKE.gap { let k = side.signum(); put(6, [h[0] - hx * WAKE.back, h[1] - WAKE.low, h[2] - hz * WAKE.back], [-hz * k, 0.0, hx * k], sp * WAKE.kick, WAKE.kick_r, 0.0); }
        }
    }

    fn push(&mut self, tick: u32, p: &SmokePush) {
        let serial = self.pushes.len() as u32;
        let (shape, len) = match (p.line, p.body) { (Some(l), _) if l > 0.0 => (1, l), (_, Some(b)) => (2, b), _ => (0, 0.0) };
        self.pushes.push(AirPush { tick, serial, shape, pos: f3(p.pos), vel: along(p.dir, p.speed), dir: f3(p.dir), radius: p.radius as f32, len: len as f32, hard: opt(p.push), expand: NONE, ttl: 0.0, lattice: match p.lattice { Some(SmokePushLattice::Fine) => 1, Some(SmokePushLattice::Coarse) => 2, None => 0 } });
    }
}

/// how tense the floor is, 0 to 1   (guards.ts tension, by the opposition's nerves)
fn tension(g: &Game) -> f64 {
    use game::game::GuardState as S;
    let level = game::guards::escalation_of(g);
    let mut x = game::m::max(if game::sim_lightshome::mains_active(g) { 0.18 } else { 0.0 }, if level >= 2.0 { 0.22 } else if level >= 1.0 { 0.12 } else { 0.0 });
    for &gd in &g.guards {
        let d = &g.gd[gd];
        let own = d.team == game::teams::Team::Operative;
        if d.state == S::Dead || (own && d.state != S::Alert) { continue; }
        let s = match d.state {
            _ if d.script.is_some() && g.clearing.is_some() => 0.3,
            S::Alert => 1.0,
            S::Search => 0.68,
            S::Suspicious => 0.45 + d.awareness * 0.2,
            _ if own => 0.0,
            _ => d.awareness * 0.35,
        };
        x = game::m::max(x, s);
    }
    x
}
