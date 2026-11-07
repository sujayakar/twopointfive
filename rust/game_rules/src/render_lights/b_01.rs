//! THE BRING-UP'S OWN PART of render_lights: bodies for skeleton stubs the first tick reached (bringup-log.md says which is a boundary no-op and which is translated)
#![allow(unused_imports, unused_variables)]
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// One light's GPU record (common.wgsl struct Light, 16 words) at word `o`. In the TypeScript `f` and `u` are two views of one buffer; here they are two
/// lists, so every word is written to both, the same bits, and a host may hand either on. `inten` scales the colour; `kind_word` is the kind with whatever
/// flag bits the caller sets above its low byte, `owner_word` the owner byte with the glass mask above it.   (src/render/lights.ts:113 packLight)
pub fn pack_light(g: &crate::game::Game, f: &mut Vec<f32>, u: &mut Vec<u32>, o: f64, l: LightId, inten: f64, kind_word: f64, volumetric: f64, owner_word: f64) {
    const DEG: f64 = core::f64::consts::PI / 180.0;
    let o = o as usize;
    if f.len() < o + 16 { f.resize(o + 16, 0.0); }
    if u.len() < o + 16 { u.resize(o + 16, 0); }
    let row = &g.tab.light[l];
    // (a portal has no cone: the two cone words are its own)
    let (w11, w12) = match row.portal.as_ref() {
        Some(p) => (p.half_wide, p.share),
        None => (m::cos(row.outer_deg * DEG), m::cos(m::min(row.inner_deg, row.outer_deg - 0.5) * DEG)),
    };
    let mut fw = |k: usize, v: f64| { let x = v as f32; f[o + k] = x; u[o + k] = x.to_bits(); };
    fw(0, row.pos[0]); fw(1, row.pos[1]); fw(2, row.pos[2]);
    fw(4, row.dir[0]); fw(5, row.dir[1]); fw(6, row.dir[2]); fw(7, row.range);
    fw(8, row.color[0] * inten); fw(9, row.color[1] * inten); fw(10, row.color[2] * inten); fw(11, w11);
    fw(12, w12); fw(13, volumetric); fw(15, row.radius);
    let mut uw = |k: usize, v: f64| { let x = js::to_u32(v); u[o + k] = x; f[o + k] = f32::from_bits(x); };
    uw(3, kind_word); uw(14, owner_word);
}


// ---- THE LIGHTS' LIST IS THE GAME'S (render/lights.ts LightSet: "this set is the game's and goes into every checkpoint"): its add / remove / flash and the fog's class
// are translated, not no-ops. Round one made `remove` a boundary no-op: a light taken off the list went on lighting the estimate.

/// (lights.ts LightSet.add)
pub fn light_set_add(g: &mut crate::game::Game, l: LightId) -> LightId { g.engine.lights.lights.push(l); l }
/// (lights.ts LightSet.remove)
pub fn light_set_remove(g: &mut crate::game::Game, l: LightId) {
    if let Some(i) = g.engine.lights.lights.iter().position(|x| *x == l) { g.engine.lights.lights.remove(i); }
}
/// Spawn a short-lived point light (a muzzle flash).   (lights.ts LightSet.flash)
pub fn light_set_flash(g: &mut crate::game::Game, pos: V3, color: V3, intensity: f64, range: f64, ttl: f64, owner: f64) -> LightId {
    let mut l = RtLight::zeroed();
    l.name = String::from("flash"); l.group = String::from("transient"); l.kind = 0.0; l.pos = pos; l.dir = [0.0, -1.0, 0.0]; l.color = color; l.intensity = intensity; l.range = range;
    l.inner_deg = 0.0; l.outer_deg = 180.0; l.radius = 0.08; l.volumetric = 1.0; l.owner = owner; l.enabled = true; l.fixture_box = -1.0; l.flicker = 0.0; l.ttl = ttl; l.age = 0.0;
    l.peak_intensity = intensity;
    let id = g.tab.light.push(l);
    light_set_add(g, id)
}
/// The class a light is uploaded with: what was said for it, else 1 for every light of a man not on the team and every flash not a teammate's own; never the sky's.   (lights.ts LightSet.fogClass)
pub fn light_set_fog_class(g: &crate::game::Game, l: LightId) -> f64 {
    let row = &g.tab.light[l];
    if let Some(f) = row.fog { if js::truthy(f) { return f; } }
    let t = &g.engine.lights.fog_team;
    let by = js::or(row.fog_by.unwrap_or(0.0), row.owner);
    let other = if by != 0.0 { !t.owners.has(crate::Num(by)) } else { row.ttl >= 0.0 || row.transient.unwrap_or(false) };
    if t.on && row.kind != 2.0 && !t.lights.has(l) && other { 1.0 } else { 0.0 }
}

/// A flash's envelope for the frame, and its retirement once a frame has shown it. THE HOST calls it between ticks (headless.ts:226, the page's frame), and the
/// tick's light estimate reads what it left: room-clear-four tick 1622, the stun's flash stood at 1600 for good and two men's irradiance was off.
/// `time` is not read (lights.ts:187 `void time`).   (render/lights.ts:174 LightSet.update)
pub fn light_set_update(g: &mut crate::game::Game, dt: f64, time: f64) {
    let f = |x: f64| -m::pow(1.0 - x, 3.2) / 3.2;
    let mut i = g.engine.lights.lights.len();
    while i > 0 {
        i -= 1;
        let id = g.engine.lights.lights[i];
        let l = &mut g.tab.light[id];
        if l.ttl >= 0.0 {
            if l.age >= l.ttl { g.engine.lights.lights.remove(i); continue; }
            let x0 = l.age / l.ttl;
            let x1 = m::min(1.0, (l.age + m::max(dt, 1e-4)) / l.ttl);
            l.intensity = l.peak_intensity * (f(x1) - f(x0)) / m::max(x1 - x0, 1e-6);
            l.age += dt;
        }
    }
}

/// A level's light as the list holds it: the row is made here and NOT added to the list (the caller's `light_set_add`). Fields the TypeScript's literal leaves out are absent.   (render/lights.ts:96 lightFromDef)
pub fn light_from_def(g: &mut crate::game::Game, d: &crate::scene_level::LightDef) -> LightId {
    use crate::scene_level::LightDefKind;
    let mut l = RtLight::zeroed();
    l.name = d.name.clone();
    l.group = d.group.clone();
    l.kind = match d.kind { LightDefKind::Point => 0.0, LightDefKind::Spot => 1.0, LightDefKind::Dir => 2.0 };
    l.pos = v3::copy(d.pos);
    l.dir = v3::normalize(d.dir);
    l.color = v3::copy(d.color);
    l.intensity = d.intensity;
    l.range = d.range;
    l.inner_deg = d.inner_deg;
    l.outer_deg = d.outer_deg;
    l.radius = if d.kind == LightDefKind::Dir { d.radius.unwrap_or(0.0) } else { d.radius.unwrap_or(0.05) };
    l.volumetric = d.volumetric;
    l.owner = 0.0;
    l.enabled = d.enabled;
    l.fixture_box = d.fixture_box.unwrap_or(-1.0);
    l.flicker = 0.0;
    l.ttl = -1.0;
    l.age = 0.0;
    l.peak_intensity = d.intensity;
    l.broad = d.broad.unwrap_or(false);
    g.tab.light.push(l)
}
