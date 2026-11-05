//! src/game/enemysquads.ts lines 1 to 314
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// The one flag over the squads' senses: 'call' = what the page opens as.   (enemysquads.ts senses)
pub fn senses(g: &mut crate::game::Game, v: Option<&str>) {
    let k = &mut g.k.squads;
    k.radio_check = v != Some("0") && v != Some("eyes");
    k.noticed_eyes = v != Some("0") && v != Some("drop");
    k.call_beat = v == Some("call") || v == Some("call-eyes") || v == Some("call-noask") || v == Some("net");
    k.net_ask = v == Some("net");
    if k.call_beat {
        k.call_body = v != Some("call-eyes");
        if v == Some("call-noask") { k.call_cut_ask = [0.0, 0.0]; }
    }
}

/// The net's ask runs on this floor: a floor whose every squad stays in its rooms.   (enemysquads.ts netAsks)
pub fn net_asks(g: &mut crate::game::Game) -> bool {
    match &g.m.enemysquads.states {
        Some(st) => st.stay_ask == Some(true),
        None => false,
    }
}

/// Where the game stands: its room clear structure's name ('' = no squads staged).   (enemysquads.ts squadsWhere)
pub fn squads_where(g: &mut crate::game::Game) -> String {
    match &g.m.enemysquads.states {
        Some(st) => st.s.name.clone(),
        None => String::new(),
    }
}

/// Hard on a squads level: the table while harder defenders are on and squads are staged, with the map's own rows over it.   (enemysquads.ts squadDrill)
pub fn squad_drill(g: &mut crate::game::Game) -> Option<crate::knobs::SquadsK> {
    let name = match &g.m.enemysquads.states { Some(st) => st.s.name.clone(), None => return None };
    if !g.k.pressure.on { return None; }
    let squads = g.k.squads;
    Some(match seam_drill_at_over(g, &name, squads) { Some(merged) => merged, None => squads })
}

/// The panel's line: which of the drill's rules hard has on where the game stands ('' = no squads staged).   (enemysquads.ts squadDrillWords)
pub fn squad_drill_words(g: &mut crate::game::Game) -> String {
    let name = match &g.m.enemysquads.states { Some(st) => st.s.name.clone(), None => return String::new() };
    let k = match squad_drill(g) { Some(k) => k, None => return alloc::format!("{}: standard squads", name) };
    let mut on: Vec<String> = Vec::new();
    if k.drill_net && k.net_k != 1.0 { on.push(alloc::format!("let you commit first (× {} net)", k.net_k)); }
    if k.drill_net && k.reinforce_k != 1.0 { on.push(alloc::format!("own men's beat × {}", k.reinforce_k)); }
    if k.drill_own { on.push(alloc::format!("no man fed into a lost room (mates up {})", k.own_up)); }
    if k.drill_back { on.push(String::from("pair gets back on its own doors")); }
    if g.k.pressure.sweep { on.push(String::from("a pair goes to look when you go quiet")); }
    let squads = g.k.squads;
    let own_set = if seam_drill_at_over(g, &name, squads).is_some() { " — this map's own set" } else { "" };
    let rules = if on.is_empty() { String::from("standard (no rule on here)") } else { on.join(" · ") };
    alloc::format!("{}: hard = {}{}", name, rules, own_set)
}

/// `{ ...over, ...DRILL_AT[name] }`, or None when the map has no row: DrillAtK came out with no rows.   (enemysquads.ts:287 DRILL_AT, :305-306)
fn seam_drill_at_over(g: &mut Game, name: &str, over: crate::knobs::SquadsK) -> Option<crate::knobs::SquadsK> {
    todo!("needs: DRILL_AT's rows by structure name (g.k.drill_at, DrillAtK is an empty struct)")
}
