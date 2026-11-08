//! THE KIT PICTURE'S STAGE, AT A START: the words of a link that carries `kitview` read into the level that is built, the mission's words and the men's dress; and the plan made once the mission is up.
use crate::json::Json;
use crate::kit_plan::{self as kp, PlanBlock, PlanLevel, Way};
use crate::kit_set::{self, QuietKind};
use alloc::borrow::Cow;
use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use core::cell::RefCell;
use game::game::Game;
use game::query::Query;
use game::kitstage::{self, KitStage, LampOver, SamOver, WayLights};

/// a link that asks for the picture, and what it does to the start
pub struct KitLink<'a> { pub q: Query, pub way: usize, pub on_set: bool, pub data: Cow<'a, Json> }

/// the link's words, where they carry `kitview`: the way it stages, and the level built for it: the set's, or the level with the way's pieces laid in, or the very level handed in
pub fn of_link<'a>(link: Option<&str>, data: &'a Json) -> Option<KitLink<'a>> {
    let q = Query::parse(link?);
    if !q.has("kitview") { return None; }
    let way = kit_set::way_of_link(&q, data);
    let set = kit_set::set_level(data, way);
    let on_set = matches!(set, Cow::Owned(_));
    let data = if on_set { set } else { kit_set::add_level(data, way) };
    Some(KitLink { q, way, on_set, data })
}
/// the quiet kind of the link, `kitloud` putting it off (the page's own word for the picture as it is on a loud mission)
fn quiet_of(q: &Query) -> u8 { if q.has("kitloud") { 0 } else { match kit_set::quiet_kind(q) { Some(QuietKind::Quiet) => 1, Some(QuietKind::Solo) => 2, None => 0 } } }

/// THE DRESS, said before the men are made: on a set of a quiet mission or the solo night, the men as that mission's are, the rifle across the back. A loud mission's set and every other picture: the scenario that is up decides.
pub fn dress(g: &mut Game, k: &KitLink) {
    if k.on_set {
        if let Some(d) = kit_set::set_dress(&k.q) {
            use game::scene_look::SceneLookModSuitAsked as S;
            g.m.scene_look.suit_asked = Some(if d.suit.is_some() { S::Quiet } else { S::NoMission });
            g.k.kitanim.sling = d.sling;
            return;
        }
    }
    if quiet_of(&k.q) != 0 { g.k.kitanim.sling = 2.0; }
}
/// the words the mission is started by: on a set the picture is a room clear whatever the mission's kind
pub fn mission_words(k: &KitLink) -> Query { if k.on_set { kit_set::set_query(&k.q) } else { k.q.clone() } }

fn lamp(j: &Json) -> LampOver {
    let fx = j.get("fixture");
    LampOver { kind: j.get("kind").num(), color: j.get("color").v3(), intensity: j.get("intensity").num(), range: j.get("range").num(), inner_deg: j.get("innerDeg").num(), outer_deg: j.get("outerDeg").num(), radius: j.get("radius").num(), volumetric: j.get("volumetric").num(), broad: j.get("broad").bool(), day: j.get("day").num(), aim: !j.get("aim").is_null(), fix_half: fx.get("half").v3(), fix_glow: fx.get("glow").num() }
}
fn way_lights(y: &Json) -> WayLights {
    let (o, s) = (y.get("light"), y.get("light").get("sam"));
    WayLights { porch: lamp(o.get("porch")), fill: lamp(o.get("fill")), sam: SamOver { on: s.get("on").bool(), intensity: s.get("intensity").num(), inner_deg: s.get("innerDeg").num(), outer_deg: s.get("outerDeg").num(), volumetric: s.get("volumetric").num(), aim: s.get("aim").v3() }, moon_scale: o.get("moonScale").num(), sky_scale: o.get("skyScale").num(), room_lights: o.get("roomLights").num() }
}

/// ONCE THE MISSION IS UP: the plan of the way it stages, made on the game's own floor, and the stage. A mission that did not start, no block, no such way or a set the block lacks: the stage says there is nothing to stage.
pub fn begin(g: &mut Game, k: &KitLink) {
    let asks_none = !["roomclear", "rescue", "nightbreach", "quiet", "squad4"].iter().any(|w| k.q.has(w));
    let mut st = Box::new(KitStage { quiet_kind: quiet_of(&k.q), asks_none, way: k.way, share: 1.0, still: k.q.has("stillkit"), ..KitStage::default() });
    let way = kitstage::staged_way(g, asks_none);
    let block = k.data.get("scenarios").get("kitView");
    let y = way.and_then(|w| match block.get("ways") { Json::Arr(a) => a.get(w), _ => None });
    if let (Some(w), Some(y)) = (way, y) {
        st.way = w;
        st.over = way_lights(y);
        st.way_room_lights = y.get("roomLights").num();
        g.kit_stage = Some(st);
        make_plan(g, &k.data, y);
    } else {
        st.none = true;
        g.kit_stage = Some(st);
        return;
    }
    kitstage::keep_base(g);
    kitstage::stage(g);
}
fn make_plan(g: &mut Game, data: &Json, y: &Json) {
    let (level, block) = (PlanLevel::of(data), PlanBlock::of(data.get("scenarios").get("kitView")));
    let way: Way = Way::of(y);
    let pl = g.player;
    let sam = g.ch[g.pl[pl].ch].pos;
    let Some(geo) = kp::geometry(&level, Some(&block), &kp::way_built(&way), [sam[0], sam[2]]) else { return };
    let cell: RefCell<&mut Game> = RefCell::new(g);
    let free = |x: f64, z: f64| !game::scene_collision::nav_grid_is_blocked(&**cell.borrow(), x, z);
    let mut sink = |h: &[usize]| kitstage::sunk(&mut **cell.borrow_mut(), h);
    let mut plan = kp::plan_child(&geo.level, &way, geo.ref_, &free, &mut sink);
    plan.set = geo.set.clone();
    let g = cell.into_inner();
    let out = kitstage::hour_has_lamps_out(g);
    let day: Vec<String> = kp::day_out(kp::own_lamps(Some(&block), &way), &geo.level.boxes, way.day_lights.as_deref(), plan.indoors, out);
    if let Some(st) = g.kit_stage.as_mut() { st.day_out = day; st.plan = Some(plan); }
}
