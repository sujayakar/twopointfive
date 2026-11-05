//! THE BRING-UP'S OWN PART of host_api: bodies for skeleton stubs the first tick reached (bringup-log.md says which is a boundary no-op and which is translated)
#![allow(unused_imports, unused_variables)]
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// BOUNDARY NO-OP (the picture's / the host's): (src/host/api.ts:75 HostApi.invalidateWorldCache)
pub fn host_api_invalidate_world_cache(g: &mut crate::game::Game) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/host/api.ts:78 HostApi.restartPicture)
pub fn host_api_restart_picture(g: &mut crate::game::Game, why: &str) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/host/api.ts:83 HostApi.setMeshShadows)
pub fn host_api_set_mesh_shadows(g: &mut crate::game::Game, m: Option<&crate::render_mesh_shadows::MeshShadows>) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/host/api.ts:85 HostApi.setRigFit)
pub fn host_api_set_rig_fit(g: &mut crate::game::Game, fit: Option<&crate::render_sdfrig::RigFit>) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/host/api.ts:86 HostApi.setParticles)
pub fn host_api_set_particles(g: &mut crate::game::Game) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/host/api.ts:87 HostApi.setDecals)
pub fn host_api_set_decals(g: &mut crate::game::Game) {}

/// BOUNDARY NO-OP (the picture's / the host's): (src/host/api.ts:97 HostApi.probe)
pub fn host_api_probe(g: &mut crate::game::Game) {}

// ---- BRING-UP, ROUND TWO: WHAT THE HOST HOLDS THAT A TICK READS AND THE BRIDGE DOES NOT CARRY (`Game.engine` is "the picture's or the host's" to it).
// BRIDGE-ASK: carry g.engine.lights (lights and ir as lists of LightId in the set's own order, fog_team), g.engine.world's lists and counts (statics, max_boxes,
// geom_serial, glass_statics, glass_statics_n, glass_list) and g.engine.settings (plain numbers), or have the host fill them through a call like this one.
/// The engine's lists and settings from a host's text: a line an item, `<name> <numbers…>`. Names: lights, ir, fog_team (on), fog_owners, fog_lights, world_statics
/// (how many: the level's own list), max_boxes, geom_serial, glass_statics, glass_statics_n, glass_list, effective_scale, and `set <key> <numbers…>` for a setting.
/// Answers the lines it did not know. Give `max_boxes` before `world_statics`.
pub fn host_engine_fill(g: &mut crate::game::Game, text: &str) -> Vec<String> {
    let mut unknown = Vec::new();
    let mut fresh: Vec<LightId> = Vec::new();
    for line in text.lines() {
        let mut w = line.split_whitespace();
        let Some(name) = w.next() else { continue; };
        if name == "set" {
            let Some(key) = w.next() else { continue; };
            let v: Vec<f64> = w.filter_map(|x| x.parse().ok()).collect();
            if !crate::root_settings::b_01::settings_set(&mut g.engine.settings, key, &v) { unknown.push(alloc::format!("set {}", key)); }
            continue;
        }
        if name == "light_new" {
            // a light of the engine's list that the bridge carried no row of (its walk never reached it): name, group, then its numbers in tc2.ts's order
            let (nm, gp) = (w.next().unwrap_or("-"), w.next().unwrap_or("-"));
            let v: Vec<f64> = w.filter_map(|x| x.parse().ok()).collect();
            let mut l = crate::render_lights::RtLight::zeroed();
            l.name = String::from(if nm == "-" { "" } else { nm }); l.group = String::from(if gp == "-" { "" } else { gp });
            l.kind = v[0]; l.pos = [v[1], v[2], v[3]]; l.dir = [v[4], v[5], v[6]]; l.color = [v[7], v[8], v[9]]; l.intensity = v[10]; l.range = v[11]; l.inner_deg = v[12]; l.outer_deg = v[13];
            l.radius = v[14]; l.volumetric = v[15]; l.owner = v[16]; l.enabled = v[17] != 0.0; l.fixture_box = v[18]; l.flicker = v[19]; l.ttl = v[20]; l.age = v[21]; l.peak_intensity = v[22]; l.broad = v[23] != 0.0;
            fresh.push(g.tab.light.push(l));
            continue;
        }
        let toks: Vec<&str> = w.clone().collect();
        let v: Vec<f64> = w.filter_map(|x| x.parse().ok()).collect();
        let ids = |_v: &[f64]| -> Vec<LightId> { toks.iter().map(|x| match x.strip_prefix('n') { Some(k) => fresh[k.parse::<usize>().unwrap()], None => LightId::from_ix(x.parse::<usize>().unwrap()) }).collect() };
        match name {
            "lights" => g.engine.lights.lights = ids(&v),
            "ir" => g.engine.lights.ir = ids(&v),
            "fog_team" => g.engine.lights.fog_team.on = v.first().copied().unwrap_or(0.0) != 0.0,
            "fog_owners" => { g.engine.lights.fog_team.owners = JsSet::new(); for x in &v { g.engine.lights.fog_team.owners.add(crate::Num(*x)); } }
            "fog_lights" => { g.engine.lights.fog_team.lights = JsSet::new(); for x in ids(&v) { g.engine.lights.fog_team.lights.add(x); } }
            // (…and the list as the host's last `upload` left it before any tick: the statics alone, with the cursor's ray grid over them — boxes.ts list)
            "world_statics" => { let n = v[0] as usize; g.engine.world.statics = g.col.boxes[..n.min(g.col.boxes.len())].to_vec(); g.engine.world.dynamics = Vec::new(); g.engine.world.statics_dirty = true; crate::scene_boxes::box_world_list(g); }
            "max_boxes" => g.engine.world.max_boxes = v[0],
            "geom_serial" => g.engine.world.geom_serial = v[0],
            "glass_statics" => g.engine.world.glass_statics = v,
            "glass_statics_n" => g.engine.world.glass_statics_n = v[0],
            "glass_list" => g.engine.world.glass_list = v,
            "effective_scale" => g.engine.effective_scale = v[0],
            _ => unknown.push(String::from(name)),
        }
    }
    unknown
}

/// The headless host's `engine.world.upload()` between two ticks, as much of it as the game reads: the boxes listed again (boxes.ts relist; the pack is the picture's).
pub fn host_world_upload(g: &mut crate::game::Game) { crate::scene_boxes::box_world_list(g); }
