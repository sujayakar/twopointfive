//! THE BRING-UP'S OWN PART of cast: bodies for skeleton stubs the first tick reached (bringup-log.md says which is a boundary no-op and which is translated)
#![allow(unused_imports, unused_variables)]
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

// cast.ts WHOLE but castWear (the viewer's). Triaged dress, but dress.ts says "what each WEARS is still dealt: his kit is placed off it": Character.parts feeds
// packside's boxes, which are in the frame's list. No die is drawn.

/// Of a cast's defenders, the ones dressed for a building.   (cast.ts officeDress; the memo by the parts array's identity is not kept: a pure function of `parts`)
pub fn office_dress(g: &mut Game, parts: &[String], list: &[AnimGltfCastVariantId]) -> Vec<AnimGltfCastVariantId> {
    let field = field_parts();
    let mut bits: u32 = 0;
    for (i, n) in parts.iter().enumerate() { if field.contains(n) { bits |= 1u32 << (i as u32 & 31); } }
    let keep: Vec<AnimGltfCastVariantId> = list.iter().copied().filter(|v| js::to_u32(g.tab.anim_gltf_cast_variant[*v].mask) & bits == 0).collect();
    if !keep.is_empty() { keep } else { list.to_vec() }
}
/// The teammate at a place of the roster.   (cast.ts operatorAt)
pub fn operator_at(g: &mut Game, cast: AnimGltfCastInfoId, place: f64) -> Option<AnimGltfCastVariantId> {
    let ops = crate::anim_gltf::cast_side(g, cast, crate::anim_gltf::CastSide::Operator);
    if ops.is_empty() { return None; }
    Some(if ops.len() > 1 { ops[1 + (place as usize) % (ops.len() - 1)] } else { ops[0] })
}
fn pick(list: &[AnimGltfCastVariantId], id: f64, skip: usize) -> Option<AnimGltfCastVariantId> {
    if list.is_empty() { return None; }
    let id = id as usize;
    Some(if list.len() > skip { list[skip + id % (list.len() - skip)] } else { list[id % list.len()] })
}
fn defenders(g: &mut Game, cast: AnimGltfCastInfoId) -> Vec<AnimGltfCastVariantId> {
    let d = crate::anim_gltf::cast_side(g, cast, crate::anim_gltf::CastSide::Defender);
    if crate::scene_look::look_office(g) { let parts = g.tab.anim_gltf_cast_info[cast].parts.clone(); office_dress(g, &parts, &d) } else { d }
}
/// The variant this man wears (None with no cast loaded).   (cast.ts castVariantOf)
pub fn cast_variant_of(g: &mut Game, c_2: CharId) -> Option<AnimGltfCastVariantId> {
    use crate::anim_gltf::CastSide;
    use crate::teams::Team;
    let cast = g.tab.rig[g.ch[c_2].rig].cast?;
    if let Some(had) = g.m.cast.worn.get(c_2) { return Some(*had); }
    let ops = crate::anim_gltf::cast_side(g, cast, CastSide::Operator);
    if crate::seats::man_of(g, Some(c_2)).is_some() { g.m.cast.worn.set(c_2, ops[0]); return Some(ops[0]); }
    let id = g.ch[c_2].id;
    let rec = g.guards.iter().copied().find(|x| g.gd[*x].ch == c_2).or_else(|| g.civilians.iter().copied().find(|x| g.gd[*x].ch == c_2));
    let Some(rec) = rec else { let d = defenders(g, cast); return pick(&d, id, 0); };
    let (team, has_unit, is_trained) = (g.gd[rec].team, g.gd[rec].unit.is_some(), g.gd[rec].trained);
    let trained = is_trained && (team == Team::Guard || (team == Team::Hostile && crate::qrf::qrf_owns(g, rec)));
    let place: Option<usize> = if team == Team::Operative && has_unit && !g.touring {
        g.guards.iter().copied().filter(|x| g.gd[*x].unit.is_some() && g.gd[*x].team == Team::Operative).position(|x| x == rec)
    } else { None };
    let v = if let Some(place) = place { operator_at(g, cast, place as f64) }
        else if team == Team::Operative { pick(&ops, id, 1) }
        else if team == Team::Civilian {
            let any = g.tab.anim_gltf_cast_info[cast].variants.iter().any(|x| g.tab.anim_gltf_cast_variant[*x].side == CastSide::Civilian);
            let list = if any { crate::anim_gltf::cast_side(g, cast, CastSide::Civilian) } else { defenders(g, cast) };
            pick(&list, id, 0)
        } else {
            let list = if trained { crate::anim_gltf::cast_side(g, cast, CastSide::Responder) } else { defenders(g, cast) };
            pick(&list, id, 0)
        };
    if !has_unit || trained || team != Team::Guard { if let Some(v) = v { g.m.cast.worn.set(c_2, v); } }
    v
}
/// Who is drawn in the sneaking suit this frame.   (cast.ts sneakSuit)
pub fn sneak_suit(g: &mut Game, c_2: CharId) -> bool {
    if !g.k.look.on || !g.k.look.suit { return false; }
    // (k: None = no mission; Some(true) = quiet; Some(false) = any other kind)
    let k: Option<bool> = match crate::scene_look::suit_as(g) {
        Some(asked) => asked.map(|_| true),   // (cast.ts:74 `asked !== undefined ? asked : activeScenario(g)`: null = no mission)
        None => crate::scenario_run::active_scenario(g).map(|a| a == crate::scenario_run::ActiveScenario::Quiet),
    };
    if k == Some(false) { return false; }
    if crate::seats::man_of(g, Some(c_2)).is_some() { return true; }
    k == Some(true) && crate::scenario_run::team_of(g).iter().any(|gd| g.gd[*gd].ch == c_2)
}
/// The part mask to draw this man with this frame (0 with no cast loaded).   (cast.ts castParts)
pub fn cast_parts(g: &mut Game, c_2: CharId) -> f64 {
    let Some(v) = cast_variant_of(g, c_2) else { return 0.0; };
    let rig = g.ch[c_2].rig;
    let cast = g.tab.rig[rig].cast.unwrap();
    let vmask = g.tab.anim_gltf_cast_variant[v].mask;
    if sneak_suit(g, c_2) { return crate::anim_gltf::cast_suit(g, cast, vmask); }
    let worn0 = if g.k.packside.on {
        let carries = crate::bigkit::big_carried(g, crate::bigkit::BigCarriedC { id: g.ch[c_2].id }).is_some();
        crate::packside::pack_mask(g, rig, vmask, carries)
    } else { vmask };
    let has = match g.m.cast.has_nvg.get(v) { Some(h) => *h, None => { let h = crate::anim_gltf::cast_goggles(g, cast, vmask, true) != vmask; g.m.cast.has_nvg.set(v, h); h } };
    if !has { return worn0; }
    let mut boxes = crate::seats::man_of(g, Some(c_2)).is_some();
    if !boxes { if let Some(gd) = g.guards.iter().copied().find(|x| g.gd[*x].ch == c_2) { boxes = crate::nvg::nvg_worn(g, gd); } }
    crate::anim_gltf::cast_goggles(g, cast, worn0, boxes)
}
