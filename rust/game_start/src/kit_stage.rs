//! THE KIT PICTURE'S STAGE, AT A START: the words of a link that carries `kitview` read into the level that is built, the mission's words and the men's dress; and the plan made once the mission is up.
use crate::json::{self, Json};
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
    let asks_none = game::scene_scenario_list::kind_of(&k.q).is_none();
    let mut st = Box::new(KitStage { quiet_kind: quiet_of(&k.q), asks_none, still: k.q.has("stillkit"), ..KitStage::default() });
    let way = kitstage::staged_way(g, asks_none);
    let block = k.data.get("scenarios").get("kitView");
    let y = way.and_then(|w| match block.get("ways") { Json::Arr(a) => a.get(w), _ => None });
    if let (Some(_w), Some(y)) = (way, y) {
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
    let sam = g.player_pos(pl);
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

/// where Sam starts on a way, as the file writes it: a room clear's team or its nth start, the night breach's team; a level with neither has the level's own spawn
fn team_start(data: &Json, way: usize) -> [f64; 2] {
    let s = data.get("scenarios");
    let (rc, nb) = (s.get("roomClear"), s.get("nightBreach"));
    let team = if !rc.get("team").is_null() { rc.get("team") } else { nb.get("team") };
    let at = if way == 0 { team } else { let w = rc.get("starts").at(way - 1); if w.is_null() { team } else { w } };
    let sp = if at.get("sam").is_null() { data.get("mission").get("playerSpawn") } else { at.get("sam") };
    let v = sp.nums();
    [v[0], v[2]]
}
/// THE PLAN OF A WAY, ASKED OF A LEVEL'S FILE WITH NO GAME STOOD (tools/qa/kitview-plan.ts, the level author's check): the level as a start would build it for the way, and the plan as `make_plan` makes it, on the
/// free floor of the file's boxes with what the plan sinks taken off it. `level` is what the picture would load (null: the file as it is). `ref_` is a team start and not the spawn alone: on a way that writes no
/// `side` the start is what says which side the team is on, and the spawn is another way's or the plain night's.
#[cfg(feature = "std")]
pub fn plan_of_file(file: &Json, way: usize) -> Json {
    let set = kit_set::set_level(file, way);
    let data = if matches!(set, Cow::Owned(_)) { set } else { kit_set::add_level(file, way) };
    let (level, block) = (PlanLevel::of(&data), PlanBlock::of(data.get("scenarios").get("kitView")));
    let w = Way::of(data.get("scenarios").get("kitView").get("ways").at(way));
    let Some(geo) = kp::geometry(&level, Some(&block), &kp::way_built(&w), team_start(&data, way)) else { return Json::Null };
    let sunk = RefCell::new(alloc::collections::BTreeSet::<usize>::new());
    let floor = RefCell::new(kp::floor(&geo.level.boxes, None));
    let grid: String = { let (f, n) = (floor.borrow(), &kitstage::KIT_PLAN.nav); (0..n.h).flat_map(|j| (0..n.w).map(move |i| (i, j))).map(|(i, j)| if f.blocked((i as f64 + 0.5) * n.cell, (j as f64 + 0.5) * n.cell) { '1' } else { '0' }).collect() };
    let mut plan = {
        let free = |x: f64, z: f64| !floor.borrow().blocked(x, z);
        let mut sink = |h: &[usize]| { sunk.borrow_mut().extend(h.iter().copied()); *floor.borrow_mut() = kp::floor(&geo.level.boxes, Some(&sunk.borrow())); };
        kp::plan_child(&geo.level, &w, geo.ref_, &free, &mut sink)
    };
    plan.set = geo.set.clone();
    let free = |x: f64, z: f64| !floor.borrow().blocked(x, z);
    let a4 = kp::act4(plan.anchor.mid, plan.s, plan.anchor.n, plan.anchor.width, &plan.slots, Some(&free), None);
    let (k, made) = (&kitstage::KIT_PLAN, match data { Cow::Owned(ref o) => o.clone(), Cow::Borrowed(_) => Json::Null });
    let gone: Vec<Json> = sunk.borrow().iter().map(|&i| json::n(i as f64)).collect();
    json::obj(alloc::vec![
        ("plan", kp::planned_json(&plan)),
        ("level", made),
        ("sunk", json::arr(gone)),
        ("act4", json::nums(&a4)), ("act4Free", json::b(free(a4[0], a4[1]))),
        ("litInside", json::arr(kp::lit_inside(&geo.level.lights, plan.anchor.mid, plan.anchor.n, plan.wall.back).iter().map(|l| json::obj(alloc::vec![("id", json::s(&l.id)), ("group", json::s(&l.group))])).collect())),
        ("boxes", json::arr(geo.level.boxes.iter().map(|b| json::obj(alloc::vec![("id", json::s(&b.id)), ("group", b.group.as_ref().map_or(Json::Null, |g| json::s(g))), ("top", json::n(b.c[1] + b.h[1]))])).collect())),
        ("grid", json::s(&grid)),
        ("numbers", json::obj(alloc::vec![("gap", json::n(k.gap)), ("apart", json::n(k.apart)), ("actClear", json::n(k.act_clear)), ("lowTop", json::n(k.low_top)), ("cell", json::n(k.nav.cell)), ("w", json::n(k.nav.w as f64)), ("h", json::n(k.nav.h as f64))])),
    ])
}
