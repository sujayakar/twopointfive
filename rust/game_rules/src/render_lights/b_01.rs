//! THE BRING-UP'S OWN PART of render_lights: bodies for skeleton stubs the first tick reached (bringup-log.md says which is a boundary no-op and which is translated)
#![allow(unused_imports, unused_variables)]
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// One light's GPU record (common.wgsl struct Light, 16 words), hung at `pos` with that radius. `inten` scales the colour; `kind_word` is the kind with
/// whatever flag bits the caller sets above its low byte, `owner_word` the owner byte with the glass mask above it.   (src/render/lights.ts packLight)
pub fn pack_light(row: &RtLight, pos: V3, radius: f64, inten: f64, kind_word: u32, volumetric: f64, owner_word: u32) -> [u32; 16] {
    const DEG: f64 = core::f64::consts::PI / 180.0;
    let w = |v: f64| (v as f32).to_bits();
    // (a portal has no cone: the two cone words are its own)
    let (w11, w12) = match row.portal.as_ref() {
        Some(p) => (p.half_wide, p.share),
        None => (m::cos(row.outer_deg * DEG), m::cos(m::min(row.inner_deg, row.outer_deg - 0.5) * DEG)),
    };
    [w(pos[0]), w(pos[1]), w(pos[2]), kind_word, w(row.dir[0]), w(row.dir[1]), w(row.dir[2]), w(row.range),
        w(row.color[0] * inten), w(row.color[1] * inten), w(row.color[2] * inten), w(w11), w(w12), w(volumetric), owner_word, w(radius)]
}


// ---- THE LIGHTS' LIST IS THE GAME'S (render/lights.ts LightSet: "this set is the game's and goes into every checkpoint"): its add / remove / flash and the fog's class
// are translated, not no-ops. Round one made `remove` a boundary no-op: a light taken off the list went on lighting the estimate.

/// WHAT A LIGHT IS BESIDE AN HOUR'S (sim/light.rs by_day), by what it is called and how it burns. (render/photometry.ts lightClass)
pub fn light_by_day(l: &crate::render_lights::RtLight) -> u32 {
    use sim::light::{DAY_FACE, DAY_FIXTURE, DAY_FLASH, DAY_HAND, DAY_NONE, DAY_VEHICLE};
    if l.kind == 2.0 || l.transient == Some(true) || l.group == "devices" || l.group == "sky" || l.name.starts_with("kitview") { return DAY_NONE; }
    if l.kind == 3.0 { return DAY_FACE; }
    if l.ttl >= 0.0 || l.group == "tracer" || l.steady.is_some() { return DAY_FLASH; }
    if l.card || l.group == "flashlights" { return DAY_HAND; }
    let body: &str = l.name.split("_lamp").next().unwrap_or("");
    let car = l.name.contains("_lamp") && ["van", "gun_truck", "truck", "pickup", "lorry", "jeep", "car"].iter().any(|v| body.starts_with(v) && body[v.len()..].chars().all(|c| c.is_ascii_alphanumeric() || c == '_'));
    if car { DAY_VEHICLE } else { DAY_FIXTURE }
}

/// (lights.ts LightSet.add)
pub fn light_set_add(g: &mut crate::game::Game, l: LightId) -> LightId { g.engine.lights.lights.push(l); l }
/// (lights.ts LightSet.remove)
pub fn light_set_remove(g: &mut crate::game::Game, l: LightId) {
    if let Some(i) = g.engine.lights.lights.iter().position(|x| *x == l) { g.engine.lights.lights.remove(i); }
}
/// Spawn a short-lived point light (a muzzle flash).   (lights.ts LightSet.flash)
pub fn light_set_flash(g: &mut crate::game::Game, pos: V3, color: V3, intensity: f64, range: f64, ttl: f64, owner: f64) -> LightId {
    let mut l = RtLight::default();
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

/// A flash's envelope for the frame, and its retirement once a frame has shown it. The tick's last line (`game_tick_end`), and the next
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
    let mut l = RtLight::default();
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
