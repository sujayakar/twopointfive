//! THE FOG OF WAR as the picture draws it: the eyes and their field, the look's numbers, the lamps as the team last knew them, and which statics are too
//! thin to cut a sight line. The rules of the fog (who is seen, what is remembered) are `fogofwar`'s; this reads what they left.
use crate::Row;
use alloc::vec::Vec;
use game::game::Game;
use game::ids::*;
use game::{fogofwar, js, m, seats};

const DEG: f64 = core::f64::consts::PI / 180.0;
/// eyes the fog's programs take
pub const EYES: usize = 7;
/// lamps the lit fog takes (FOG_LIGHTS)
pub const LIGHTS: usize = 48;
/// words of a bit a static (FOG_THIN_WORDS: 4096 boxes)
pub const THIN_WORDS: usize = 128;

section! {
    /// the fog: one row
    pub struct Fog {
        /// the fog is drawn; 0 = every other field is 0
        pub on: u32,
        /// live eyes
        pub count: u32,
        /// an eye: x y z, then the man's id (0 = none)
        pub eyes: [f32; 28],
        /// where it looks in plan (x, z; unit), the cosine of its half field (-2 = all round), its all-round near field in metres
        pub looks: [f32; 28],
        pub dim: f32,
        pub desat: f32,
        pub top_pad: f32,
        /// the band the fog comes in over across a sight edge, metres
        pub edge: f32,
        /// the wedge's soft edge: the cosines of the half field less and plus half the soft band, and the metres the near field's rim comes in over
        pub soft_in: f32,
        pub soft_out: f32,
        pub near_soft: f32,
        pub flat: f32,
        /// the lamps' strength in the fog (0 = none), the fill, the level under which a lamp is not traced, and the fill's corner shade and its reach
        pub lit: f32,
        pub lit_fill: f32,
        pub lit_cut: f32,
        pub lit_ao: f32,
        pub lit_ao_m: f32,
        pub light_count: u32,
        /// the lamps, as the scene's own light record is packed: 16 words a lamp
        pub lights: [u32; 768],
        /// the full picture under the fog, and its fill
        pub full: u32,
        pub full_flat: f32,
        /// smoke and dust are drawn over floor nobody sees
        pub air: u32,
        /// thin things do not cut a sight line
        pub thin: u32,
        /// what lies still stays as last seen
        pub kept: u32,
        /// which statics are thin, a bit a box, and the list they were read off: its geometry serial and its length
        pub thin_bits: [u32; 128],
        pub thin_serial: u32,
        pub thin_count: u32,
    }
}

pub fn fill(g: &Game, out: &mut Vec<Fog>, bared: bool) {
    out.push(Fog::ZERO);
    // (the Team & kit picture's wand row has the fog on for the game alone: the picture has none)
    if bared || !fogofwar::fog_of_war_active(g) || g.kit_stage.is_some() { return; }
    let (f, k) = (out.last_mut().unwrap(), &g.k.fog);
    f.on = 1;
    f.dim = k.dim as f32;
    f.desat = k.desat as f32;
    f.top_pad = k.top_pad as f32;
    f.edge = m::max(0.0, k.edge) as f32;
    f.flat = m::max(0.0, k.flat) as f32;
    f.full = 1;
    f.full_flat = m::max(0.0, k.flat * k.full_fill) as f32;
    f.air = 1;
    f.kept = 1;
    eyes(g, f);
    lamps(g, f);
    thin(g, f);
}

fn eyes(g: &Game, f: &mut Fog) {
    let k = &g.k.fog;
    let (fov, near) = (fogofwar::field_deg(g), fogofwar::field_near(g));
    let all_round = fov >= 360.0;
    let half = |deg: f64| if deg >= 360.0 { -2.0 } else { m::cos(m::max(1.0, deg) * 0.5 * DEG) };
    // (the picture's soft edges are the cone's alone: a plain wedge keeps its hard line)
    let soft = if k.cone && !all_round { m::max(0.0, m::min(m::min(k.cone_soft_deg, fov), 360.0 - fov)) } else { 0.0 };
    if soft > 0.0 {
        f.soft_in = m::cos(m::max(0.0, fov * 0.5 - soft * 0.5) * DEG) as f32;
        f.soft_out = m::cos(m::min(180.0, fov * 0.5 + soft * 0.5) * DEG) as f32;
    }
    if k.cone && !all_round { f.near_soft = m::max(0.0, k.cone_near_soft_m) as f32; }
    f.count = g.fog_of_war.eyes.len() as u32;
    for (i, e) in g.fog_of_war.eyes.iter().take(EYES).enumerate() {
        let l = js::or(m::hyp2(e.look[0], e.look[2]), 1.0);
        f.eyes[i * 4..i * 4 + 4].copy_from_slice(&[e.at[0] as f32, e.at[1] as f32, e.at[2] as f32, e.id as f32]);
        // (an eye with its own field: the wand's lens)
        f.looks[i * 4..i * 4 + 4].copy_from_slice(&[(e.look[0] / l) as f32, (e.look[2] / l) as f32, half(e.fov.unwrap_or(fov)) as f32, e.near.unwrap_or(near) as f32]);
    }
}

/// every lamp the team has as lit at its own full strength, then the sun or moon; more than the block holds: the ones whose reach is nearest the player
fn lamps(g: &Game, f: &mut Fog) {
    let k = &g.k.fog;
    if !(k.lit > 0.0) || !(k.flat > 0.0) { return; }
    f.lit = k.lit as f32;
    f.lit_fill = m::max(0.0, k.flat * k.lit_fill) as f32;
    f.lit_cut = m::max(0.0, k.lit_cut) as f32;
    f.lit_ao = m::min(1.0, m::max(0.0, k.lit_ao)) as f32;
    f.lit_ao_m = m::max(0.0, k.lit_ao_m) as f32;
    let me = g.player_pos(seats::host_man(g));
    let mut pick: Vec<(LightId, f64, f64)> = Vec::with_capacity(LIGHTS);
    let mut take = |l: LightId, strength: f64, far: bool| if strength > 0.0 {
        let row = &g.tab.light[l];
        pick.push((l, strength, if far { -1.0 } else { m::max(0.0, game::v3::dist_xz(row.pos, me) - row.range) }));
    };
    for &l in &g.engine.lights.lights {
        let row = &g.tab.light[l];
        if row.kind == 2.0 && row.ttl < 0.0 && row.transient != Some(true) && row.enabled { take(l, row.intensity, true); }
    }
    let down = g.light_home.mains.active;
    for &t in &g.targets {
        let row = &g.tab.fixture[t];
        // (no record = the team has not met it yet: as it stands)
        let known = match g.fog_of_war.lamp_recs.get(t) { Some(lit) => *lit, None => fogofwar::fog_of_war_lamp_lit(g, t) };
        if !known || (row.mains && down) { continue; }
        if let Some(l) = row.light { take(l, match g.tab.light[l].steady { Some(ref s) => s.intensity, None => g.tab.fixture[t].base_intensity }, false); }
        for al in row.area_lights.iter().flatten() { take(al.light, al.base, false); }
    }
    if pick.len() > LIGHTS {
        js::sort_by(&mut pick, |a, b| a.2 - b.2);
        pick.truncate(LIGHTS);
    }
    f.light_count = pick.len() as u32;
    for (i, &(l, strength, _)) in pick.iter().enumerate() {
        // a fire's lamp wanders with its flames and is as big as they are: the fog has it where the level hung it, the size the level gave it
        let row = &g.tab.light[l];
        let (pos, radius) = match row.steady { Some(ref s) => (s.pos, s.radius), None => (row.pos, row.radius) };
        f.lights[i * 16..i * 16 + 16].copy_from_slice(&game::render_lights::pack_light(row, pos, radius, strength, row.kind as u32, 0.0, 0));
    }
}

fn thin(g: &Game, f: &mut Fog) {
    let st = &g.engine.world.statics;
    f.thin = 1;
    f.thin_serial = g.engine.world.geom_serial as u32;
    f.thin_count = st.len() as u32;
    let flag = js::to_i32(game::scene_boxes::box_flag().thin);
    for (i, &b) in st.iter().take(THIN_WORDS * 32).enumerate() {
        if js::to_i32(g.tab.boxes[b].flags) & flag != 0 { f.thin_bits[i >> 5] |= 1 << (i & 31); }
    }
}
