//! THE BRING-UP'S OWN PART of render_lights: bodies for skeleton stubs the first tick reached (bringup-log.md says which is a boundary no-op and which is translated)
#![allow(unused_imports, unused_variables)]
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// BOUNDARY NO-OP (the picture's / the host's): (src/render/lights.ts:113 packLight)
pub fn pack_light(g: &mut crate::game::Game, f: &mut Vec<f32>, u: &mut Vec<u32>, o: f64, l: LightId, inten: f64, kind_word: f64, volumetric: f64, owner_word: f64) {}


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
pub fn light_set_fog_class(g: &mut crate::game::Game, l: LightId) -> f64 {
    let row = &g.tab.light[l];
    if let Some(f) = row.fog { if js::truthy(f) { return f; } }
    let t = &g.engine.lights.fog_team;
    let by = js::or(row.fog_by.unwrap_or(0.0), row.owner);
    let other = if by != 0.0 { !t.owners.has(crate::Num(by)) } else { row.ttl >= 0.0 || row.transient.unwrap_or(false) };
    if t.on && row.kind != 2.0 && !t.lights.has(l) && other { 1.0 } else { 0.0 }
}
