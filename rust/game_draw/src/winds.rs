//! THE AIR AS THE PICTURE IS TOLD IT: the wind's sources that blow, each list as the tick left it (`winds` for the smoke's solver, `gusts` for the particles, `heat` for the
//! falling snow), the storm's own numbers and the roofs the weather is cut by. Whole every tick. What the air then does is the picture's own (its field, its smoke).
use crate::Row;
use alloc::vec::Vec;
use game::game::Game;
use game::ids::*;
use game::out_events::OutEvent;
use game::smoke_types::{WindGpuSource, WindGpuSourceKind};
use game::wind::{WindSource, WindSourceKind};

section! {
    /// a source of moving air
    pub struct Wind {
        /// 0 a jet, 1 the storm
        pub kind: u32,
        /// 1 the storm is a field cut roof by roof · 2 the air of a fire
        pub says: u32,
        /// which particles it moves, a bit a kind (0: the usual)
        pub kinds: u32,
        /// the roofs it blows between: its first row in `wind_roofs`, and how many (0: its two rectangles)
        pub roofs_at: u32,
        pub roofs: u32,
        pub pos: [f32; 3],
        pub dir: [f32; 3],
        pub speed: f32,
        pub reach: f32,
        pub tan_spread: f32,
        pub r0: f32,
        pub d_max: f32,
        /// NaN: not said
        pub floor_y: f32,
        pub interior: [f32; 4],
        pub annex: [f32; 4],
        /// the sphere it reaches: middle, radius
        pub bound: [f32; 4],
    }
}
section! {
    /// the weather's air: one row
    pub struct Air {
        /// the wind's master (0: still air), how fast the solver's air answers it, and the particles' three numbers
        pub master: f32,
        pub relax: f32,
        pub knobs: [f32; 3],
        /// the storm: how fast it blows now, which way, its gust, and the share of its speed at the ground (0: no storm source)
        pub storm_speed: f32,
        pub storm_dir: [f32; 3],
        pub storm_gust: f32,
        /// how far from a roof the storm's field has come in whole (m)
        pub storm_lee: f32,
        /// a rain falls by the level's own roofs (`roofs`), else by the two rectangles
        pub level_roofs: u32,
        /// the two rectangles
        pub interior: [f32; 4],
        pub annex: [f32; 4],
    }
}
section! {
    /// the corners of a rectangle on the plan: x0 z0 x1 z1
    pub struct Rect {
        pub r: [f32; 4],
    }
}

#[derive(Default)]
pub struct Winds {
    pub active: Vec<Wind>,
    pub gusts: Vec<Wind>,
    pub heat: Vec<Wind>,
    pub air: Vec<Air>,
    pub wind_roofs: Vec<Rect>,
    pub roofs: Vec<Rect>,
    /// the level's roofs as built, whatever the weather is cut by; the doors' leaves where they hang, seen or not; the panes the air goes through
    pub ceilings: Vec<Rect>,
    pub leaves: Vec<crate::boxes::Leaf>,
    pub open: Vec<crate::boxes::Word>,
    of: (usize, f64),
}

impl Winds {
    /// `again`: the tick already taken, and what it told its host was handed on
    pub fn take(&mut self, g: &Game, again: bool) {
        (self.active.clear(), self.gusts.clear(), self.air.clear(), self.wind_roofs.clear(), self.roofs.clear());
        for &id in &g.wind.active { let w = own(&g.tab.wind_wind_source[id], &mut self.wind_roofs); self.active.push(w); }
        for id in gusts(g) { let w = own(&g.tab.wind_wind_source[id], &mut self.wind_roofs); self.gusts.push(w); }
        // (the heat of the fires is said by the tick that moved the clock: a tick that said none has none)
        if !again {
            self.heat.clear();
            for e in g.out.iter().rev() { if let OutEvent::ParticlesSetHeat(h) = e { for s in &h.srcs { if !crate::far_for_a_fire(g, s.pos) { self.heat.push(gpu(s)); } } break; } }
        }
        for r in g.weather.roof_rects.iter() { self.roofs.push(Rect { r: r.map(|x| x as f32) }); }
        self.stands(g);
        let f4 = |v: [f64; 4]| v.map(|x| x as f32);
        let st = game::wind::wind_by_name(g, "storm").map(|id| &g.tab.wind_wind_source[id]);
        let mut a = Air::ZERO;
        a.master = game::wind::wind_of(g) as f32;
        a.relax = g.k.wind.smoke_relax as f32;
        a.knobs = [g.k.wind.sheet_wake, g.k.wind.sheet_kick, g.k.wind.debris_k].map(|x| x as f32);
        if let Some(s) = st {
            a.storm_speed = s.speed as f32;
            a.storm_dir = s.dir.map(|x| x as f32);
            a.storm_gust = s.gust as f32;
        }
        a.storm_lee = ::sim::wind::SHAPE[4] as f32;
        a.level_roofs = game::weather::weather_level_roofs(g) as u32;
        a.interior = f4(game::weather::interior());
        a.annex = f4(game::weather::annex());
        self.air.push(a);
    }
}

impl Winds {
    /// what stands in the air's way that the boxes' rows do not say
    fn stands(&mut self, g: &Game) {
        (self.leaves.clear(), self.open.clear());
        let (statics, all) = (&g.engine.world.statics, &g.tab.boxes);
        // (the roofs as built move only with the list)
        let list = (statics.len(), g.engine.world.geom_serial);
        if self.of != list {
            self.of = list; self.ceilings.clear();
            ::sim::weather::roofs_begin();
            for &id in statics { let b = &all[id]; ::sim::weather::roofs_box(b.c[0], b.c[1], b.c[2], b.h[0], b.h[1], b.h[2], game::scene_world::world().ceiling_y); }
            let mut v = [0.0; ::sim::weather::ROOFS * 4];
            let n = ::sim::weather::roofs_end(&mut v);
            for i in 0..n { self.ceilings.push(Rect { r: [v[i * 4] as f32, v[i * 4 + 1] as f32, v[i * 4 + 2] as f32, v[i * 4 + 3] as f32] }); }
        }
        for &d in &g.doors.list { let b = &all[g.dr[d].box_]; if b.rot.is_none() { self.leaves.push(crate::boxes::Leaf { c: b.c.map(|x| x as f32), h: b.h.map(|x| x as f32), yaw: b.yaw as f32, flags: b.flags as i64 as u32, key: d.ix() as u32 }); } }
        if ::sim::state::state_read(g.sim).panes.used != 0 {
            let glass = game::scene_boxes::box_flag().glass as u32;
            for (i, &id) in statics.iter().enumerate() { if all[id].flags as i64 as u32 & glass != 0 && ::sim::panes::open_fraction(g.sim, i as u32) > g.k.air_sim.glass_open { self.open.push(crate::boxes::Word { w: i as u32 }); } }
        }
    }
}

/// The sources the particles feel, as the picture picks them: the jets near where it looks, nearest first when there are more than it has room for, then the storm
fn gusts(g: &Game) -> Vec<WindWindSourceId> {
    let (tab, f, near_m) = (&g.tab.wind_wind_source, crate::looks_at(g), g.k.wind.near_m);
    let dist = |s: WindWindSourceId| game::v3::dist(tab[s].bound.c, f);
    let mut jets: Vec<_> = g.wind.active.iter().copied().filter(|&s| tab[s].kind == WindSourceKind::Jet && dist(s) < near_m + tab[s].bound.r).collect();
    let storm = g.wind.active.iter().copied().find(|&s| tab[s].kind == WindSourceKind::Storm);
    let most = game::smoke_types::WIND_GPU_MAX as usize - storm.is_some() as usize;
    if jets.len() > most {
        jets.sort_by(|&a, &b| dist(a).total_cmp(&dist(b)));
        jets.truncate(most);
    }
    jets.extend(storm);
    jets
}

fn own(s: &WindSource, roofs: &mut Vec<Rect>) -> Wind {
    let mut w = Wind::ZERO;
    w.kind = (s.kind == WindSourceKind::Storm) as u32;
    w.says = (s.field == Some(true)) as u32 | (s.heat == Some(true)) as u32 * 2;
    w.kinds = s.kinds.map_or(0, |k| k as u32);
    if let Some(r) = s.roofs.as_ref() {
        w.roofs_at = roofs.len() as u32;
        w.roofs = r.len() as u32;
        for q in r { roofs.push(Rect { r: [q[0] as f32, q[1] as f32, q[2] as f32, q[3] as f32] }); }
    }
    w.pos = s.pos.map(|x| x as f32);
    w.dir = s.dir.map(|x| x as f32);
    (w.speed, w.reach, w.tan_spread, w.r0, w.d_max) = (s.speed as f32, s.reach as f32, s.tan_spread as f32, s.r0 as f32, s.d_max as f32);
    w.floor_y = s.floor_y.map_or(f32::NAN, |y| y as f32);
    w.interior = s.interior.map_or([0.0; 4], |r| r.map(|x| x as f32));
    w.annex = s.annex.map_or([0.0; 4], |r| r.map(|x| x as f32));
    w.bound = [s.bound.c[0] as f32, s.bound.c[1] as f32, s.bound.c[2] as f32, s.bound.r as f32];
    w
}

fn gpu(s: &WindGpuSource) -> Wind {
    let mut w = Wind::ZERO;
    w.kind = matches!(s.kind, WindGpuSourceKind::Storm) as u32;
    w.says = (s.field == Some(true)) as u32 | (s.heat == Some(true)) as u32 * 2;
    w.kinds = s.kinds.map_or(0, |k| k as u32);
    w.pos = s.pos.map(|x| x as f32);
    w.dir = s.dir.map(|x| x as f32);
    (w.speed, w.reach, w.tan_spread, w.r0, w.d_max) = (s.speed as f32, s.reach as f32, s.tan_spread as f32, s.r0 as f32, s.d_max as f32);
    w.floor_y = s.floor_y.map_or(f32::NAN, |y| y as f32);
    w.bound = [s.bound.c[0] as f32, s.bound.c[1] as f32, s.bound.c[2] as f32, s.bound.r as f32];
    w
}
