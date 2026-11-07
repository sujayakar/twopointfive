//! THE PICTURE'S SIDE OF THE FOG OF WAR: the renderer's block, the lit fog's list of lamps, a lamp's face as the team remembers it, and the list a
//! viewer's draw with the fog off is made from (that draw itself is the page's: it lifts the fog on its own copy of the game, between two steps).
//! Everything here is computed FROM the game's state and read by nothing in it: no rule calls into this file, and what it writes is the block
//! (`fog_picture.frame`), the lit fog's list and counts, the kept faces, the viewer's list, and the renderer's own fields of a light and a box
//! (`fog`, `fog_ghost`, `fog_emit`, `fog_mark`: the upload alone reads these).
//!
//! What the team knows (who is seen, which doors, lamps and loose things are remembered and how, who the team is) is the rest of fogofwar/ and is the
//! game's: every machine of one game holds it the same. This file reads it: `lamp_recs` for a lamp, `hidden_chars` for a man, `eyes` for the block.
use super::*;
use crate::game::Game;
use crate::ids::*;
use crate::js;
use crate::m;
use crate::v3::V3;
use alloc::{vec, vec::Vec};

/// a box's row
fn bx<'g>(g: &'g mut Game, b: BoxId) -> &'g mut crate::scene_boxes::Box { &mut g.tab.boxes[b] }

/// THE RENDERER'S BLOCK for this frame: what the fog's pass and the sight table draw from (render/fogofwar.ts FogFrame). Called at the end of the game's
/// pass, once a tick, on every machine: it is computed from the game's state and the page hands it to the renderer as it comes.
pub fn fill(g: &mut crate::game::Game) {
    // (the look's four rows, desat, edge, air and kept, are settled at the page's values: there is no other look to put back, and the count only runs down)
    if g.fog_picture.look_frames > 0.0 {
        g.fog_picture.look_frames -= 1.0;
    }
    let k = g.k.fog;
    {
        let n = g.fog_of_war.eyes.len() as f64;
        let f = &mut g.fog_picture.frame;
        f.on = true; f.count = n; f.dim = k.dim; f.desat = k.desat; f.top_pad = k.top_pad; f.edge = m::max(0.0, k.edge);
    }
    let fov = crate::fogofwar::field_deg(g);
    let near = crate::fogofwar::field_near(g);
    let all_round = fov >= 360.0;
    let cos_half = if all_round { -2.0 } else { m::cos(m::max(1.0, fov) * 0.5 * DEG) };
    // (the picture's soft edges are the cone's alone: a plain wedge keeps its hard line)
    let soft = if k.cone && !all_round { m::max(0.0, m::min(m::min(k.cone_soft_deg, fov), 360.0 - fov)) } else { 0.0 };
    full_block(g);
    g.fog_picture.frame.full = true;
    g.fog_picture.frame.flat = m::max(0.0, k.flat);
    lit_block(g);
    g.fog_picture.frame.air = true;
    g.fog_picture.frame.kept = true;
    thin_block(g);
    let n_eyes = crate::render_fogofwar::FOG_EYES as usize;
    let pic = core::mem::take(&mut g.fog_of_war.eyes);
    {
        let f = &mut g.fog_picture.frame;
        f.soft_in = if soft > 0.0 { m::cos(m::max(0.0, fov * 0.5 - soft * 0.5) * DEG) } else { 0.0 };
        f.soft_out = if soft > 0.0 { m::cos(m::min(180.0, fov * 0.5 + soft * 0.5) * DEG) } else { 0.0 };
        f.near_soft = if k.cone && !all_round { m::max(0.0, k.cone_near_soft_m) } else { 0.0 };
        if f.eyes.len() < n_eyes * 4 { f.eyes.resize(n_eyes * 4, 0.0); }
        if f.looks.len() < n_eyes * 4 { f.looks.resize(n_eyes * 4, 0.0); }
        for i in 0..n_eyes {
            let o = i * 4;
            let Some(e) = pic.get(i) else {
                for w in 0..4 { f.eyes[o + w] = 0.0; f.looks[o + w] = 0.0; }
                continue;
            };
            f.eyes[o] = e.at[0] as f32; f.eyes[o + 1] = e.at[1] as f32; f.eyes[o + 2] = e.at[2] as f32; f.eyes[o + 3] = e.id as f32;
            let l = js::or(m::hyp2(e.look[0], e.look[2]), 1.0);
            f.looks[o] = (e.look[0] / l) as f32; f.looks[o + 1] = (e.look[2] / l) as f32;
            // (an eye with its own field: the wand's lens)
            f.looks[o + 2] = (match e.fov { None => cos_half, Some(v) if v >= 360.0 => -2.0, Some(v) => m::cos(m::max(1.0, v) * 0.5 * DEG) }) as f32;
            f.looks[o + 3] = e.near.unwrap_or(near) as f32;
        }
    }
    g.fog_of_war.eyes = pic;
}

/// The lights' classes for this frame, painted: the level's lamps the team has in another state than they are in. Out but remembered lit = lit where nobody
/// sees; lit but remembered out = lit only where the team sees. (What a man carries who is not on the team, and every flash not a teammate's own, take
/// their class from the team's list, which the pass filled: render_lights::light_set_fog_class.)   (fogofwar.ts FogOfWar.fullBlock, its second half)
fn full_block(g: &mut crate::game::Game) {
    g.fog_picture.full_stats.hidden = 0.0;
    g.fog_picture.full_stats.ghosts = 0.0;
    g.fog_picture.full_stats.held = 0.0;
    g.fog_picture.frame.full_flat = m::max(0.0, g.k.fog.flat * g.k.fog.full_fill);
    let down = crate::sim_lightshome::mains_active(g);
    let es = g.engine.settings.emissive_scale;
    let mut i = 0;
    while i < g.targets.len() {
        let t = g.targets[i];
        i += 1;
        let (light, areas, mains) = { let row = &g.tab.sim_lightshome_fixture[t]; (row.light, row.area_lights.clone(), row.mains) };
        if light.is_none() && areas.is_none() { continue; }
        let areas = areas.unwrap_or_default();
        if let Some(l) = light { g.tab.light[l].fog = Some(0.0); g.tab.light[l].fog_ghost = Some(0.0); }
        for al in &areas { g.tab.light[al.light].fog = Some(0.0); g.tab.light[al.light].fog_ghost = Some(0.0); }
        fog_of_war_lamp_face(g, t, None);
        let lit = fog_of_war_lamp_lit(g, t);
        let known = g.fog_of_war.lamp_recs.get(t).copied().unwrap_or(lit);
        if known == lit {
            if lit { fog_of_war_face_seen(g, t); }
            continue;
        }
        g.fog_picture.full_stats.held += 1.0;
        if lit || !(mains && down) { fog_of_war_lamp_face(g, t, Some(!lit)); }
        if lit {
            if let Some(l) = light { g.tab.light[l].fog = Some(1.0); }
            for al in &areas { g.tab.light[al.light].fog = Some(1.0); }
        } else if !(mains && down) {
            g.fog_picture.full_stats.ghosts += 1.0;
            if let Some(l) = light {
                let ghost = match g.tab.light[l].steady { Some(ref s) => s.intensity, None => crate::sim_lightshome::fixture_base_intensity(g, t) };
                g.tab.light[l].fog_ghost = Some(ghost);
            }
            for al in &areas { g.tab.light[al.light].fog_ghost = Some(al.base * es); }
        }
    }
    let mut i = 0;
    while i < g.engine.lights.lights.len() {
        let l = g.engine.lights.lights[i];
        i += 1;
        if crate::render_lights::light_set_fog_class(g, l) == 1.0 { g.fog_picture.full_stats.hidden += 1.0; }
    }
}

/// A lamp for the lit fog's list: nothing with no strength; `far` = the sun or moon, which stays when the list is cut.   (fogofwar.ts FogOfWar.lampTake)
fn lamp_take(g: &mut crate::game::Game, l: LightId, i: f64, far: bool) {
    if !(i > 0.0) { return; }
    let host = crate::seats::host_man(g);
    let pp = g.ch[g.pl[host].ch].pos;
    let d = if far {
        -1.0
    } else {
        let row = &g.tab.light[l];
        m::max(0.0, m::hyp2(row.pos[0] - pp[0], row.pos[2] - pp[2]) - row.range)
    };
    let n = g.fog_picture.lamp_n as usize;
    let take = crate::fogofwar::FogOfWarLampPickItem { l, i, d };
    if n < g.fog_picture.lamp_pick.len() { g.fog_picture.lamp_pick[n] = take; } else { g.fog_picture.lamp_pick.push(take); }
    g.fog_picture.lamp_n += 1.0;
}

/// The lit fog's rows of the renderer's block: the levels, and every lamp the team has as lit at its own full strength, then the sun or moon. More than the block holds: the ones whose reach is nearest the player.   (fogofwar.ts FogOfWar.litBlock)
fn lit_block(g: &mut crate::game::Game) {
    g.fog_picture.lit_stats.lamps = 0.0;
    g.fog_picture.lit_stats.lights = 0.0;
    g.fog_picture.lit_stats.held = 0.0;
    let fog = (g.k.fog.lit, g.k.fog.flat, g.k.fog.lit_fill, g.k.fog.lit_cut, g.k.fog.lit_ao, g.k.fog.lit_ao_m);
    if !(fog.0 > 0.0) || !(fog.1 > 0.0) {
        g.fog_picture.frame.lit = 0.0;
        g.fog_picture.frame.light_count = 0.0;
        return;
    }
    g.fog_picture.frame.lit = fog.0;
    g.fog_picture.frame.lit_fill = m::max(0.0, fog.1 * fog.2);
    g.fog_picture.frame.lit_cut = m::max(0.0, fog.3);
    g.fog_picture.frame.lit_ao = m::min(1.0, m::max(0.0, fog.4));
    g.fog_picture.frame.lit_ao_m = m::max(0.0, fog.5);
    g.fog_picture.lamp_n = 0.0;
    let mut i = 0;
    while i < g.engine.lights.lights.len() {
        let l = g.engine.lights.lights[i];
        i += 1;
        let row = &g.tab.light[l];
        if row.kind == 2.0 && row.ttl < 0.0 && row.transient != Some(true) && row.enabled {
            let strength = row.intensity;
            lamp_take(g, l, strength, true);
        }
    }
    let down = crate::sim_lightshome::mains_active(g);
    let es = g.engine.settings.emissive_scale;
    let mut i = 0;
    while i < g.targets.len() {
        let t = g.targets[i];
        i += 1;
        let (light, areas, mains) = { let row = &g.tab.sim_lightshome_fixture[t]; (row.light, row.area_lights.clone(), row.mains) };
        if light.is_none() && areas.is_none() { continue; }
        g.fog_picture.lit_stats.lamps += 1.0;
        let lit = fog_of_war_lamp_lit(g, t);
        let rec = g.fog_of_war.lamp_recs.get(t).copied().unwrap_or(lit);
        if rec != lit { g.fog_picture.lit_stats.held += 1.0; }
        if !rec || (mains && down) { continue; }
        if let Some(l) = light {
            let strength = match g.tab.light[l].steady { Some(ref s) => s.intensity, None => crate::sim_lightshome::fixture_base_intensity(g, t) };
            lamp_take(g, l, strength, false);
        }
        for al in areas.unwrap_or_default() { lamp_take(g, al.light, al.base * es, false); }
    }
    let mut n = g.fog_picture.lamp_n;
    if n > crate::render_fogofwar::FOG_LIGHTS {
        g.fog_picture.lamp_pick.truncate(n as usize);
        js::sort_by(&mut g.fog_picture.lamp_pick, |a, b| a.d - b.d);
        n = crate::render_fogofwar::FOG_LIGHTS;
    }
    let mut lights = core::mem::take(&mut g.fog_picture.frame.lights);
    let mut lights_u = core::mem::take(&mut g.fog_picture.frame.lights_u);
    let mut k = 0;
    while (k as f64) < n {
        let (l, strength) = { let p = g.fog_picture.lamp_pick[k]; (p.l, p.i) };
        // a fire's lamp wanders with its flames and is as big as they are: the fog has it where the level hung it, the size the level gave it
        let (live, size, steady, kind) = { let row = &g.tab.light[l]; (row.pos, row.radius, row.steady.as_ref().map(|s| (s.pos, s.radius)), row.kind) };
        if let Some((pos, radius)) = steady { g.tab.light[l].pos = pos; g.tab.light[l].radius = radius; }
        crate::render_lights::pack_light(g, &mut lights, &mut lights_u, k as f64 * 16.0, l, strength, kind, 0.0, 0.0);
        g.tab.light[l].pos = live;
        g.tab.light[l].radius = size;
        k += 1;
    }
    g.fog_picture.frame.lights = lights;
    g.fog_picture.frame.lights_u = lights_u;
    g.fog_picture.frame.light_count = n;
    g.fog_picture.lit_stats.lights = n;
    g.fog_picture.lamp_pick.clear();   // (made again every frame, by the distance from the man this machine shows: nothing of it is the game's)
}

/// The picture's half of FOG.thin: the row, and which statics are marked, as bits by box index. Filled every frame: a breached wall's studs come and go. (FogOfWar.thinBlock)
fn thin_block(g: &mut crate::game::Game) {
    g.fog_picture.frame.thin = true;
    if !g.fog_picture.frame.thin { return; }
    for w in g.fog_picture.frame.thin_bits.iter_mut() { *w = 0; }
    g.fog_picture.frame.thin_serial = g.engine.world.geom_serial;
    let st = g.engine.world.statics.clone();
    g.fog_picture.frame.thin_count = st.len() as f64;
    let thin = js::to_i32(crate::scene_boxes::box_flag().thin);
    let n = core::cmp::min(st.len(), g.fog_picture.frame.thin_bits.len() * 32);
    for i in 0..n {
        if js::to_i32(bx(g, st[i]).flags) & thin != 0 { g.fog_picture.frame.thin_bits[i >> 5] |= 1u32 << (i & 31); }
    }
}

/// The fog has gone off: the block says so, and every class and kept face is taken back off the lights and the lamps.   (fogofwar.ts FogOfWar.fullClear, and the pass's own head)
pub fn off(g: &mut crate::game::Game) {
    let f = &mut g.fog_picture.frame;
    f.on = false; f.count = 0.0; f.lit = 0.0; f.light_count = 0.0; f.full = false;
    g.fog_picture.face_recs.clear();
    if !g.fog_of_war.live { return; }
    let mut i = 0;
    while i < g.engine.lights.lights.len() {
        let l = g.engine.lights.lights[i];
        i += 1;
        g.tab.light[l].fog = Some(0.0);
        g.tab.light[l].fog_ghost = Some(0.0);
    }
    fog_of_war_faces_lift(g);
    g.fog_picture.full_stats.hidden = 0.0;
    g.fog_picture.full_stats.ghosts = 0.0;
    g.fog_picture.full_stats.held = 0.0;
}

/// A fresh encounter: no list kept for a viewer's draw, no lamp's face kept.   (fogofwar.ts FogOfWar.reset, the picture's two)
pub fn reset(g: &mut crate::game::Game) {
    g.fog_picture.raw_at = -1.0;
    g.fog_picture.face_recs.clear();
}

/// The picture's face of a lamp: its boxes drawn lit or dark through `fog_emit` / `fog_mark`, which the upload alone reads; `lit` None = as it stands.   (fogofwar.ts FogOfWar.lampFace)
pub fn fog_of_war_lamp_face(g: &mut crate::game::Game, t: FixtureId, lit: Option<bool>) {
    let rec = if lit == Some(true) { g.fog_picture.face_recs.get(t).cloned() } else { None };
    let boxes = g.tab.sim_lightshome_fixture[t].boxes.clone();
    let base = g.tab.sim_lightshome_fixture[t].base_emissive;
    let flag = crate::scene_boxes::box_flag();
    let shows = js::to_i32(flag.area_lit) | js::to_i32(flag.no_shadow);
    let mut i = 0;
    while i < boxes.len() {
        let at = i;
        i += 1;
        let b = match g.engine.world.statics.get(boxes[at] as usize) { Some(&b) => b, None => continue };
        let q = bx(g, b);
        let on = match lit {
            Some(on) => on,
            None => {
                if q.fog_emit.is_some() { q.fog_emit = None; q.fog_mark = None; }
                continue;
            }
        };
        if js::to_i32(q.flags) & shows == 0 { continue; }
        let kept = rec.as_ref().and_then(|r| r.get(at)).copied();
        let (emit, mark) = face_of(on, kept, base);
        q.fog_emit = Some(emit);
        q.fog_mark = Some(mark);
    }
}

/// What a lamp's box is drawn as: lit, the glow kept if it had one and the level's own if not; out, dark.   (fogofwar.ts FogOfWar.lampFace)
fn face_of(on: bool, kept: Option<V3>, base: V3) -> (V3, f64) {
    if !on { return (dark(), 0.0); }
    (match kept { Some(k) if k[0] + k[1] + k[2] > 0.0 => k, _ => base }, 1.0)
}

/// A lamp the team knows lit: its face as it glows now is kept, the brightest of the last moments.   (fogofwar.ts FogOfWar.faceSeen)
pub fn fog_of_war_face_seen(g: &mut crate::game::Game, t: FixtureId) {
    let boxes = g.tab.sim_lightshome_fixture[t].boxes.clone();
    if boxes.is_empty() { return; }
    let n = boxes.len();
    g.fog_picture.face_recs.get_or_insert_with(t, || vec![[0.0; 3]; n]);
    let mut i = 0;
    while i < n {
        let at = i;
        i += 1;
        let b = match g.engine.world.statics.get(boxes[at] as usize) { Some(&b) => b, None => continue };
        let e = bx(g, b).emissive;
        let k = &mut g.fog_picture.face_recs.get_mut(t).unwrap()[at];
        k[0] = m::max(k[0] * FACE_HOLD, e[0]);
        k[1] = m::max(k[1] * FACE_HOLD, e[1]);
        k[2] = m::max(k[2] * FACE_HOLD, e[2]);
    }
}

/// Every kept face taken off.   (fogofwar.ts FogOfWar.facesLift)
pub fn fog_of_war_faces_lift(g: &mut crate::game::Game) {
    let mut i = 0;
    while i < g.targets.len() {
        let t = g.targets[i];
        i += 1;
        fog_of_war_lamp_face(g, t, None);
    }
}

/// The frame's list of boxes as it stands BEFORE the fog cuts it, kept for a viewer's draw with the fog off: the doors live in place of the copies the
/// fog keeps of them. The page reads the list off its copy of the game (fogofwar.ts `raw`); nothing is kept where no viewer has asked. Called just
/// before `fog_of_war_things`.   (fogofwar.ts FogOfWar.things, its head)
pub fn keep_raw(g: &mut crate::game::Game, list: &[BoxId]) {
    if !(g.fog_picture.viewer && fog_of_war_active(g)) { return; }
    let mut raw = core::mem::take(&mut g.fog_picture.raw_list);
    raw.clear();
    let mut i = 0;
    if js::truthy(g.fog_of_war.door_n) {
        let from = g.fog_of_war.door_from as usize;
        while i < from { raw.push(list[i]); i += 1; }
        crate::doors::doors_boxes(g, &mut raw);
        i = (g.fog_of_war.door_from + g.fog_of_war.door_n) as usize;
    }
    while i < list.len() { raw.push(list[i]); i += 1; }
    g.fog_picture.raw_list = raw;
    g.fog_picture.raw_at = g.fog_of_war.frame_no;
}

/// The characters to draw, of the frame's list.   (fogofwar.ts FogOfWar.filter)
pub fn fog_of_war_filter(g: &crate::game::Game, all: &[CharId]) -> Vec<CharId> {
    all.iter().copied().filter(|&c| !g.fog_of_war.hidden_chars.has(c)).collect()
}

/// The engine replay's word: a few frames under the page's fog look. (FogOfWar.pageLook)
pub fn fog_of_war_page_look(g: &mut crate::game::Game) {
    g.fog_picture.look_frames = 3.0;
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A lit lamp's face: what was kept if it glowed, the level's own if not; dark when out.
    #[test]
    fn a_face_is_kept_lit_or_dark() {
        let base = [7.0, 7.0, 7.0];
        assert_eq!(face_of(true, None, base), (base, 1.0));
        assert_eq!(face_of(true, Some([0.0; 3]), base), (base, 1.0));
        assert_eq!(face_of(true, Some([2.0, 1.0, 0.5]), base), ([2.0, 1.0, 0.5], 1.0));
        assert_eq!(face_of(false, Some([2.0, 1.0, 0.5]), base), ([0.0; 3], 0.0));
    }
}
