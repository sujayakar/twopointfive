//! THE OPEN FIRES: what the composite draws of each and what the picture's own smoke and air are lit from. The lamp, its flicker and the coals are the rules' (the
//! lights and the boxes say them). Every number here is a closed form of the game's clock, so a load or a rollback draws the same flame.
use crate::Row;
use alloc::vec::Vec;
use game::fire::{burns, fire_flicker, fire_look_smoke, fire_seed, flame_wind, heat_of, is_fire, mouth_of};
use game::game::Game;
use game::m;

section! {
    /// a fire that burns, or that the fog's picture has burning still
    pub struct Fire {
        /// its place among the level's fires, which is who it is
        pub ix: u32,
        /// it burns (else it is out, and only the fog's picture has it)
        pub lit: u32,
        /// its class under the fog: 0 drawn anywhere · 1 only where the team sees · 2 only where it does not
        pub fog: u32,
        /// its own number 0‥1 off where it stands
        pub seed: f32,
        /// its mouth and how wide: the middle of the glowing box's top, the flame's radius, the bed of coals'
        pub pos: [f32; 3],
        pub r: f32,
        pub bed: f32,
        /// how high a calm flame stands, how hot it burns, the clock it is drawn at, how its lamp stands to the authored one
        pub h: f32,
        pub heat: f32,
        pub time: f32,
        pub flick: f32,
        /// what leans the flame (capped), and the air at it as it blows
        pub wind: [f32; 2],
        pub blows: [f32; 3],
    }
}
section! {
    /// the numbers every fire is drawn by: one row, on a level that has a fire
    pub struct FireTable {
        /// fire.wgsl FireLook, in its order
        pub look: [f32; 16],
        /// how far off the view a flame is drawn, how many, the flame's height and how far the wind leans it
        pub draw_r: f32,
        pub max: f32,
        pub height: f32,
        pub tall: f32,
        pub lean: f32,
        /// its smoke: on, up, radius, cells, density, temp, speed, push, jitter, every, view_r, lean
        pub smoke: [f32; 12],
        /// the flame as gas: on, up, radius, cells, density, temp, speed, push, jitter, every, view_r, keep, ease
        pub gas: [f32; 13],
        /// one column: up, radius, density, soot_ease, puff_every, puff_secs, puff_up, puff_radius, puff_off, puff_density, puff_temp, puff_speed, puff_push, puff_expand
        pub column: [f32; 14],
        pub soot: [f32; 3],
    }
}

/// `want`: every fire's row, and the table where there is one
pub fn fill(g: &Game, out: &mut Vec<Fire>, table: &mut Vec<FireTable>, bared: bool) {
    table.clear();
    let time = g.time;
    for (ix, &t) in g.targets.iter().filter(|&&t| is_fire(g, t)).enumerate() {
        let l = g.tab.sim_lightshome_fixture[t].light.unwrap();
        let (lamp, lit) = (&g.tab.light[l], burns(g, t));
        // (out, and nobody on the team saw it go out: where the team does not see, the fog's picture has its lamp lit still and its flame on it)
        if !lit && (bared || !(lamp.fog_ghost.unwrap_or(0.0) > 0.0)) { continue; }
        let (mouth, seed) = (mouth_of(g, t), fire_seed(g, t));
        let w = flame_wind(g, mouth.pos);
        out.push(Fire {
            ix: ix as u32, lit: lit as u32, fog: if !lit { 2 } else if lamp.fog == Some(1.0) && !bared { 1 } else { 0 }, seed: seed as f32,
            pos: mouth.pos.map(|x| x as f32), r: mouth.r as f32, bed: (mouth.lid * g.k.fire.bed_r) as f32, h: g.k.fire.height as f32, heat: if lit { heat_of(g, t) as f32 } else { 1.0 },
            time: (time % 4096.0) as f32, flick: fire_flicker(g, seed, time) as f32, wind: w.wind.map(|x| x as f32), blows: w.wv.map(|x| x as f32),
        });
    }
    if g.targets.iter().any(|&t| is_fire(g, t)) { table.push(numbers(g)); }
}

fn numbers(g: &Game) -> FireTable {
    let (k, c) = (&g.k.fire, fire_look_smoke());
    let (l, s, a) = (&k.look, &k.smoke, &k.gas);
    let on = |b: bool| if b { 1.0 } else { 0.0 };
    let mut r = FireTable::ZERO;
    r.look = [l.gain, l.core, l.sigma, l.speed, l.tall, l.torn, l.plume, l.shimmer, l.soot, l.breathe, a.ref_, a.lo, l.react, l.lean, l.steps, 0.0].map(|x| x as f32);
    (r.draw_r, r.max, r.height, r.tall, r.lean) = (k.draw_r as f32, m::max(0.0, m::min(4.0, m::floor(k.max))) as f32, k.height as f32, l.tall as f32, l.lean as f32);
    r.smoke = [on(s.on), s.up, s.radius, s.cells, s.density, s.temp, s.speed, s.push, s.jitter, s.every, s.view_r, s.lean].map(|x| x as f32);
    r.gas = [on(a.on), a.up, a.radius, a.cells, a.density, a.temp, a.speed, a.push, a.jitter, a.every, a.view_r, a.keep, a.ease].map(|x| x as f32);
    r.column = [c.up, c.radius, c.density, c.soot_ease, c.puff_every, c.puff_secs, c.puff_up, c.puff_radius, c.puff_off, c.puff_density, c.puff_temp, c.puff_speed, c.puff_push, c.puff_expand].map(|x| x as f32);
    r.soot = c.soot.map(|x| x as f32);
    r
}
