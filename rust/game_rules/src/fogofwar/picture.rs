//! THE PICTURE'S SIDE OF THE FOG OF WAR: a lamp's class and its face as the team remembers it, and the list a viewer's draw with the fog off is made from
//! (that draw itself is the page's: it lifts the fog on its own copy of the game, between two steps). What the fog's pass draws from (the eyes, the lit
//! fog's lamps, the look's numbers) is no state at all: rust/game_draw `fog` makes it of a game that stands.
//! Everything here is computed FROM the game's state and read by nothing in it: no rule calls into this file, and what it writes is the counts, the kept
//! faces, the viewer's list, and the renderer's own fields of a light and a box (`fog`, `fog_ghost`, `fog_emit`, `fog_mark`: the upload alone reads these).
//!
//! What the team knows (who is seen, which doors, lamps and loose things are remembered and how, who the team is) is the rest of fogofwar/ and is the
//! game's: every machine of one game holds it the same. This file reads it: `lamp_recs` for a lamp, `hidden_chars` for a man.
use super::*;
use crate::game::Game;
use crate::ids::*;
use crate::js;
use crate::m;
use crate::v3::V3;
use alloc::{vec, vec::Vec};

/// a box's row
fn bx<'g>(g: &'g mut Game, b: impl Into<AnyBoxId>) -> &'g mut crate::scene_boxes::Box { &mut g.tab[b.into()] }

/// Called at the end of the game's pass, once a tick, on every machine.
/// The lights' classes for this frame, painted: the level's lamps the team has in another state than they are in. Out but remembered lit = lit where nobody
/// sees; lit but remembered out = lit only where the team sees. (What a man carries who is not on the team, and every flash not a teammate's own, take
/// their class from the team's list, which the pass filled: render_lights::light_set_fog_class.)   (fogofwar.ts FogOfWar.fullBlock, its second half)
pub fn fill(g: &mut crate::game::Game) {
    g.fog_picture.full_stats.hidden = 0.0;
    g.fog_picture.full_stats.ghosts = 0.0;
    g.fog_picture.full_stats.held = 0.0;
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

/// The fog has gone off: every class and kept face is taken back off the lights and the lamps.   (fogofwar.ts FogOfWar.fullClear, and the pass's own head)
pub fn off(g: &mut crate::game::Game) {
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
/// fog keeps of them. Nothing is kept where no viewer has asked. Called just before `fog_of_war_things`.   (fogofwar.ts FogOfWar.things, its head)
pub fn keep_raw(g: &mut crate::game::Game, list: &[AnyBoxId]) {
    if !g.fog_picture.viewer { return; }
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

/// The engine replay's word for a few frames under the page's fog look: the look's rows are settled at the page's values, and there is no other to put back.   (FogOfWar.pageLook)
pub fn fog_of_war_page_look(g: &crate::game::Game) {}

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
