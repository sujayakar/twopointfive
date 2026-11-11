use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::scene_level::Level;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// Makes the game in `g`: the caller has set `g.engine`, `g.level`, `g.smoke` and `g.sim` on a blank Game, and made the level's boxes as rows listed in `g.col.boxes` in the level's order.
/// Each part stands on those before it: what a part draws of the die, and the order the lists are filled in, hang on this order.
pub(in crate::tick) fn game_new(g: &mut crate::game::Game, ch: RigId, opts: Option<crate::game::GameNewOpts>) {
    let level = g.level.clone();   // (read below while `g` is lent; no writer of the level's fields stands in the constructor)
    g.rig = ch;
    g.light_home = crate::fixtures::LightHome::blank();
    // the level's boxes are g.col.boxes, as the caller left them. `ids[i]` is what `level.boxes[i]` names
    let ids: Vec<BoxId> = g.col.boxes.clone();
    if let Some(over) = crate::levelfit::level_over(g, &level_counts_of(&level, &ids)) { panic!("{}", over); }
    let n_seats = opts.and_then(|o| o.seats).unwrap_or(1.0);
    stand_world(g, &ids, n_seats);
    stand_player(g, ch, &level);
    stand_lamps(g, &level, &ids);
    stand_doors(g, &level);
    g.devices = crate::devices::devices_new(g);   // cameras + emitters join `targets`, before the authored switch states are said, below
    stand_guards(g, &level);
    stand_glass_and_air(g);
    crate::game::game_stand_lights(g);   // the lamps the fixtures feed, as rows in the list's order (rust/sim/src/lights.rs); a fixture's authored bit is said by its `add` now
    crate::civilians::spawn_civilians(g);   // after the shift, whose construction and dice they must not disturb
    stand_rules(g);
    stand_orders(g);
    crate::seats::stand_men(g);   // a second seat's man and his light, made last
    g.director = crate::director::director_new(g, level.script.clone());   // the level's script: its define pass run and linted here, programs start on demand
    crate::glassbreak::glass_break_stood(g);   // which glass each pane is, settled as the level stands
    let stood = stood_of(g, &crate::seats::here(g), &level);
    if let Some(over) = crate::levelfit::stood_over(g, &stood) { panic!("{}", over); }
}

/// the seats, what this machine kept, and the level's boxes as a thing to walk into
fn stand_world(g: &mut Game, ids: &[BoxId], n_seats: f64) {
    g.seats = crate::seats::seats_stand(g, n_seats);
    g.local_seat = Some(g.seats[0]);
    // what this machine kept.
    crate::difficulty::levers_fog(g);
    crate::destruction::load_chaos(g);
    g.tune.wind = g.k.wind.def;   // (loadWind's first line; the rest of it is storage and it left the game)
    g.col = crate::scene_collision::new(g, ids);
}

/// the first player, the loose furniture and thrown things, the weather, and the player's own light
fn stand_player(g: &mut Game, ch: RigId, level: &Level) {
    let (men, fn2000) = (g.men, g.fn2000);
    let spawn = level.player_spawn;
    g.player = crate::game::player_new(g, men, ch, spawn, Some(fn2000), None);   // (first.player: g.seat is seats[0] all through the constructor)
    // loose furniture and thrown things: never collision occluders
    g.props = crate::props::prop_system_new(g, &level.props);
    g.items = crate::throwables::Throwables::blank();
    g.tracers = crate::tracers::tracers_new(g, None);   // (its lights and its sparks are reached by path)
    g.weather = crate::weather::weather_new(g);
    let p = g.player;
    let player_light = g.tune.player_light;
    let rail = crate::rig_props::rail_light_def(g, crate::consts::PLAYER_ID, player_light);
    g.pl[p].light = crate::render_lights::light_set_add(g, rail);
}

/// the lamps: every fixture a light or a glowing box is fed by, the area lights and lit markings of the glowing ones, the breaker and its beacons, and the
/// cells under a roof
fn stand_lamps(g: &mut Game, level: &Level, ids: &[BoxId]) {
    // (contactor groups restrike staggered, by order of first appearance)
    let mut stagger: JsMap<String, f64> = JsMap::new();
    analytic_fixtures(g, level, ids, &mut stagger);
    glowing_fixtures(g, level, ids, &mut stagger);
    let car_lamps = crate::carlamps::car_lamp_targets(g, ids);   // (the lamps of the level's cars: one target each, no die, none on the mains)
    for def in car_lamps {
        let t = crate::fixtures::fixture_add(g, def);
        g.targets.push(t);
    }
    let targets: Vec<FixtureId> = g.targets.clone();
    for &t in targets.iter() { panel_lights(g, ids, t); }
    for &t in targets.iter() { mark_lights(g, ids, t); }
    for b in level.beacons.iter() { mark_area_lit(g, ids[b.box_ as usize]); }   // dome glow is screen-only; the beam is the spot light
    breaker_and_beacons(g, level);
    // the cells under a roof (the game's own rules read them)
    let statics: Vec<BoxId> = g.engine.world.statics.clone();
    let interior = crate::render_reach::build_interior_cells(g, &statics);
    g.reach_interior = Some(interior);
}

/// a fixture for each light of the level's own that can be switched or shot out
fn analytic_fixtures(g: &mut Game, level: &Level, ids: &[BoxId], group_stagger: &mut JsMap<String, f64>) {
    let lights: Vec<LightId> = g.engine.lights.lights.clone();
    for &l in lights.iter() {
        let group = g.tab.light[l].group.clone();
        if g.tab.light[l].ttl >= 0.0 || g.tab.light[l].kind == 2.0 || group == "player" || group == "flashlights" { continue; }
        let name = g.tab.light[l].name.clone();
        let mains = light_def_mains(level, &name);
        let fixture_box = g.tab.light[l].fixture_box;
        let base_emissive = if fixture_box >= 0.0 { g.tab.boxes[ids[fixture_box as usize]].emissive } else { [0.0, 0.0, 0.0] };
        let def = crate::fixtures::Fixture {
            name,
            pos: v3::copy(g.tab.light[l].pos),
            light: Some(l),
            boxes: if fixture_box >= 0.0 { vec![fixture_box] } else { Vec::new() },
            base_emissive,
            base_intensity: g.tab.light[l].intensity,
            on: g.tab.light[l].enabled,
            factor: 1.0,
            mains,
            stagger: stagger_of(g, group_stagger, &group),
            interactive: true,
            kind: crate::fixtures::FixtureKind::Analytic,
            group,
            ..crate::fixtures::Fixture::blank()
        };
        let t = crate::fixtures::fixture_add(g, def);
        g.targets.push(t);
    }
}

/// a fixture for each glowing box
fn glowing_fixtures(g: &mut Game, level: &Level, ids: &[BoxId], group_stagger: &mut JsMap<String, f64>) {
    for sw in level.switchables.iter() {
        let pos = g.tab.boxes[ids[sw.boxes[0] as usize]].c;
        let def = crate::fixtures::Fixture {
            name: sw.name.clone(),
            group: sw.group.clone(),
            kind: crate::fixtures::FixtureKind::Emissive,
            pos,
            boxes: sw.boxes.clone(),
            base_emissive: sw.base_emissive,
            on: sw.on,
            flicker: sw.flicker == Some(true),
            factor: if sw.on { 1.0 } else { 0.0 },
            mains: sw.mains != Some(false),
            stagger: stagger_of(g, group_stagger, &sw.group),
            interactive: sw.group != "server_leds" && sw.group != "server_glow",
            ..crate::fixtures::Fixture::blank()
        };
        let t = crate::fixtures::fixture_add(g, def);
        g.targets.push(t);
    }
}

/// flat emissive fixtures light the scene through analytic one-sided area lights; their boxes keep glowing, rays ignore the emissive term
fn panel_lights(g: &mut Game, ids: &[BoxId], t: FixtureId) {
    let (kind, group, name, boxes, e) = {
        let f = &g.tab.fixture[t];
        (f.kind, f.group.clone(), f.name.clone(), f.boxes.clone(), f.base_emissive)
    };
    if kind != crate::fixtures::FixtureKind::Emissive { return; }
    for &bi in boxes.iter() {
        let (h, c, yaw) = { let b = &g.tab.boxes[ids[bi as usize]]; (b.h, b.c, b.yaw) };
        let Some(panel) = crate::levelfit::flat_panel(&group, &h) else { continue };   // (the one rule for which fixture box is an area light; the lamp bound asks it too)
        let (thin, a0, a1, area) = (panel.thin as usize, panel.a0 as usize, panel.a1 as usize, panel.area);
        // emitting side: ceiling fixtures face down; wall / desk fixtures whichever side is open
        let n: V3 = if thin == 1 {
            [0.0, if c[1] > 1.5 { -1.0 } else { 1.0 }, 0.0]
        } else {
            let (cs, sn) = (m::cos(yaw), m::sin(yaw));
            let ax: V3 = if thin == 0 { [cs, 0.0, -sn] } else { [sn, 0.0, cs] };
            let front = !crate::scene_collision::segment_blocked(g, c, v3::mad(c, ax, h[thin] + 0.35), None, None, None);
            let back = !crate::scene_collision::segment_blocked(g, c, v3::mad(c, ax, -(h[thin] + 0.35)), None, None, None);
            if front && !back { ax } else if !front && back { v3::scale(ax, -1.0) } else { ax }
        };
        let maxc = m::max_n(&[e[0], e[1], e[2], 1e-3]);
        let i0 = maxc * area;   // radiance x area = on-axis intensity
        let on = g.tab.fixture[t].on;
        let light = add_light(g, &format!("{}_area{}", name, bi as i64), &group, 3.0, v3::mad(c, n, h[thin] + 0.07), n, [e[0] / maxc, e[1] / maxc, e[2] / maxc], i0,
            v3::clamp(m::sqrt(i0 / 0.0015), 3.0, 26.0), 0.0, 90.0,
            m::max(0.1, m::min(m::sqrt(area / m::PI), m::min(h[a0], h[a1]) + 0.02)), 0.0, on, i0, true, None);
        g.tab.fixture[t].area_lights.get_or_insert_with(Vec::new).push(crate::fixtures::FixtureLight { light, base: i0 });
        mark_area_lit(g, ids[bi as usize]);
    }
}

/// emissive markings are lights: a switchable box's marking with `emit` registers an emitter over its rect, on the same target's supply
fn mark_lights(g: &mut Game, ids: &[BoxId], t: FixtureId) {
    let (kind, group, name, boxes) = {
        let f = &g.tab.fixture[t];
        (f.kind, f.group.clone(), f.name.clone(), f.boxes.clone())
    };
    if kind != crate::fixtures::FixtureKind::Emissive || group == "emergency" { return; }
    for &bi in boxes.iter() {
        let (marks, tilted, albedo, c, h, yaw) = { let b = &g.tab.boxes[ids[bi as usize]]; (b.marks.clone(), b.rot.is_some(), b.albedo, b.c, b.h, b.yaw) };
        let Some(marks) = marks else { continue };
        if marks.is_empty() || tilted { continue; }
        let mk_texel = g.level.mark_texel.unwrap_or(crate::scene_markings::MARKING_TEXEL);
        for (k, mk) in marks.iter().enumerate() {
            let st = crate::scene_markings::mark_stats(mk, albedo, Some(mk_texel));
            if !js::truthy(st.emask) { continue; }
            let maxc = m::max_n(&[st.mean_l[0], st.mean_l[1], st.mean_l[2]]);
            let i0 = maxc * st.area;
            if st.area < 0.01 || i0 < crate::game::MARK_LIGHT_MIN_I0 { continue; }   // LED rows, UI bars, buttons: they glow, they light nothing
            let r = crate::scene_markings::mark_world_rect(mk, c, h, yaw, Some(mk_texel));
            let n = r.n;
            let on = g.tab.fixture[t].on;
            let light = add_light(g, &format!("{}_mark{}_{}", name, bi as i64, k), &group, 3.0, v3::mad(r.centre, n, 0.07), n, [st.mean_l[0] / maxc, st.mean_l[1] / maxc, st.mean_l[2] / maxc], i0,
                v3::clamp(m::sqrt(i0 / 0.0015), 3.0, 26.0), 0.0, 90.0,
                m::max(0.1, m::min(m::sqrt(st.area / m::PI), m::min(r.hu, r.hv) + 0.02)), 0.0, on, i0, true,
                if bi < 4096.0 && k < 4 { Some(((js::to_i32(bi) << 2) | k as i32) as f64) } else { None });
            g.tab.fixture[t].area_lights.get_or_insert_with(Vec::new).push(crate::fixtures::FixtureLight { light, base: i0 });
            g.m.scene_markings.area_lit_marks.add(crate::scene_markings::mark_key(mk));
        }
    }
}

/// mains breaker (OCP: temporary blackout, bullet: permanent) and the emergency beacons it brings up
fn breaker_and_beacons(g: &mut Game, level: &Level) {
    let breaker = crate::fixtures::Fixture {
        name: String::from("breaker"),
        group: String::from("mains"),
        kind: crate::fixtures::FixtureKind::Breaker,
        pos: level.breaker.pos,
        boxes: vec![level.breaker.boxes[1]],
        base_emissive: [0.3, 4.0, 0.6],
        on: true,
        factor: 1.0,
        interactive: true,
        ..crate::fixtures::Fixture::blank()
    };
    let t = crate::fixtures::fixture_add(g, breaker);
    g.targets.push(t);
    for (i, b) in level.beacons.iter().enumerate() {
        let mut target: Option<FixtureId> = None;
        for &t in g.targets.iter() {
            let f = &g.tab.fixture[t];
            if f.kind == crate::fixtures::FixtureKind::Emissive && f.boxes.contains(&b.box_) { target = Some(t); break; }
        }
        let Some(target) = target else { panic!("beacon {}: no switchable owns box {}", i, b.box_) };
        let (name, group) = { let f = &g.tab.fixture[target]; (f.name.clone(), f.group.clone()) };
        let light = add_light(g, &name, &group, 1.0, b.pos, [1.0, 0.0, 0.0], [1.0, 0.22, 0.06], 55.0, 14.0, 9.0, 24.0, 0.06, 1.0, false, 55.0, false, None);
        g.beacons.push(crate::game::GameBeaconsItem { light, phase: i as f64 * 1.7, target });
    }
}

/// doors: leaves are dynamic occluders for sight / sound / projectiles
fn stand_doors(g: &mut Game, level: &Level) {
    let door_defs: Vec<crate::doors::DoorDef> = level.doors.iter().map(|d| crate::doors::DoorDef {
        name: d.name.clone(),
        hinge: d.hinge,
        closed_dir: d.closed_dir,
        width: d.width,
        min_angle: d.min_angle,
        max_angle: d.max_angle,
        angle: d.angle,
        swing: d.swing,
        albedo: if d.exterior == Some(true) { Some([0.2, 0.22, 0.25]) } else { None },
        closer: d.closer,
        exterior: d.exterior,
        locked: d.locked,
        key_side: d.key_side,
        lock: d.lock.clone(),
        steel: d.steel,
        height: None,
        thickness: None,
    }).collect();
    g.doors = crate::doors::doors_new(g, &door_defs, &mut game_on_door_event);
    crate::doors::paint_leaves(g);   // (a level's leaves wear its paint from their first moment: doors/paint.rs)
    let list: Vec<DoorId> = g.doors.list.clone();
    g.props.doorways = list.iter().map(|&d| {
        let (cc, dir, width) = (g.dr[d].frame_centre, g.dr[d].def.closed_dir, g.dr[d].def.width);
        crate::props::PropSystemDoorwaysItem { cx: cc[0], cz: cc[2], ux: m::cos(dir), uz: m::sin(dir), half: width / 2.0 - 0.02 }
    }).collect();
    let office = crate::dialogue::is_office(g);
    crate::doornames::name_doors(g, office, &list);
    g.door_deals = crate::doorswing::deal_swings(g);   // the side each leaf opens into: geometry only, no dice
    let leaves = crate::doors::doors_leaf_boxes(g);
    g.tab.boxes.leaves(&leaves);
    crate::snow_bog::lie(g);
    g.col.dynamic_boxes = leaves;
}

/// the guards, each with his torch on his route, and the lamps of the racks the errand is at
fn stand_guards(g: &mut Game, level: &Level) {
    let (men, rig) = (g.men, g.rig);
    let tints: [V3; 3] = [[0.42, 0.36, 0.25], [0.3, 0.34, 0.4], [0.38, 0.28, 0.28]];
    for (i, &r) in level.routes.iter().enumerate() {
        let id = 10.0 + i as f64;
        let (fi, f0, f1) = (g.tune.flash_intensity, g.tune.flash_inner, g.tune.flash_outer);
        let def = crate::rig_props::torch_light_def(g, &format!("flashlight{}", i), id, fi, f0, f1);
        let fl = crate::render_lights::light_set_add(g, def);
        let gd = crate::game::guard_new(g, men, rig, id, r, i as i32, fl, tints[i % tints.len()], None);
        g.guards.push(gd);
    }
    crate::opening::opening_start(g);   // a man whose route starts on top of the way in is found down his beat instead
    let (leds, glow) = (format!("rack{}_leds", level.errand.index), format!("rack{}_glow", level.errand.index));
    let targets: Vec<FixtureId> = g.targets.clone();
    g.rack_targets = targets.iter().copied().filter(|&t| { let f = &g.tab.fixture[t]; f.name == leds || f.name == glow }).collect();
}

/// what breaks and what drifts
fn stand_glass_and_air(g: &mut Game) {
    g.destr = crate::destruction::destruction_new(g);   // after the targets and devices exist: a lit screen dies through its target
    g.glass_break = crate::glassbreak::glass_break_new(g);
    crate::winbars::boot_bars(g);
    g.shards = crate::shards::glass_shards_new(g);
    g.wind = crate::wind::wind_new(g);            // after the doors exist
    g.air_dust = crate::air_dust::air_dust_new(g);   // after the weather, the doors and the collision exist
    g.moths = crate::moths::moths_new(g);   // moths at the lamps: after the targets (the lamps are light targets)
}

/// the squad's fight, its cues and its book, and the missions' own rules
fn stand_rules(g: &mut Game) {
    g.fog_of_war = crate::fogofwar::fog_of_war_new();
    g.fog_picture = crate::fogofwar::FogPicture::blank();
    g.staged = crate::mission::Staged::default();
    g.rails = crate::rails::rails_new();
    g.lesson = crate::lesson::lesson_new();
    g.night_breach = crate::nightbreach::night_breach_new();
    g.room_clear = crate::roomclear::room_clear_scenario_new();
    stand_each_seat(g, &mut |v: &mut Game| { v.groups[v.group].orders = crate::orders::squad_orders_new(v); });
    g.fight = crate::roomfight::room_fight_new();
    g.cues = crate::cues::cues_new();
    g.squad_net = crate::squadnet::squad_net_new();
    g.cover = crate::cover::cover_new(g);
    g.entry = crate::entrypoints::entry_points_new();
    g.cleared = crate::cleared::cleared_rooms_new();
    g.rescue = crate::rescue::rescue_new();
}

/// each seat's own hands on its squad: the door tools after the orders (their rows are registered on the orders' menu), and what spends their charges after them
fn stand_orders(g: &mut Game) {
    stand_each_seat(g, &mut |v: &mut Game| { v.groups[v.group].tools = crate::doortools::door_tools_new(v); });   // after the orders: its rows are registered on their menu
    crate::pairup::pair_up_apply(g, None);
    stand_each_seat(g, &mut |v: &mut Game| { v.pair = crate::pairorders::pair_orders_new(v); });   // (the people and the note are each seat's; the jobs are one book)
    stand_each_seat(g, &mut |v: &mut Game| { v.sets[v.seat].work = crate::workthings::work_things_new(v); });
    stand_each_seat(g, &mut |v: &mut Game| { v.sets[v.seat].wall_charge = crate::wallcharge::wall_charge_new(v); });   // after the door tools: it spends their charges
    stand_each_seat(g, &mut |v: &mut Game| { crate::shutdoor::register_shut_door(v); });
}

/// `engine.lights.add` of the constructor's literals: owner 0, fixtureBox -1, flicker 0, ttl -1, age 0.
fn add_light(g: &mut Game, name: &str, group: &str, kind: f64, pos: V3, dir: V3, color: V3, intensity: f64, range: f64, inner_deg: f64, outer_deg: f64, radius: f64, volumetric: f64, enabled: bool, peak_intensity: f64, broad: bool, mark_ref: Option<f64>) -> LightId {
    let l = g.tab.light.push(crate::render_lights::RtLight {
        name: String::from(name),
        group: String::from(group),
        kind,
        pos,
        dir,
        color,
        intensity,
        range,
        inner_deg,
        outer_deg,
        radius,
        volumetric,
        owner: 0.0,
        enabled,
        fixture_box: -1.0,
        ttl: -1.0,
        age: 0.0,
        peak_intensity,
        broad,
        mark_ref,
        ..crate::render_lights::RtLight::default()
    });
    crate::render_lights::light_set_add(g, l)
}

/// `b.flags |= BoxFlag.AreaLit`: dome and panel glow is the screen's, the light is the analytic one.
fn mark_area_lit(g: &mut Game, id: BoxId) {
    let bit = js::to_i32(crate::scene_boxes::box_flag().area_lit);
    let mut b = g.tab.boxes.edit(id);
    b.flags = (js::to_i32(b.flags) | bit) as f64;
}

fn level_counts_of(level: &crate::scene_level::Level, ids: &[BoxId]) -> crate::levelfit::LevelCountsLevel {
    crate::levelfit::LevelCountsLevel {
        doors: vec![crate::Opaque; level.doors.len()],
        props: stood_level_props(level),
        boxes: ids.to_vec(),
        lights: Some(vec![crate::Opaque; level.lights.len()]),
        switchables: Some(stood_level_switchables(level)),
        beacons: Some(vec![crate::Opaque; level.beacons.len()]),
        devices: stood_level_devices(level),
    }
}

fn stood_level_props(level: &crate::scene_level::Level) -> Vec<crate::levelfit::StoodLevelPropsItem> {
    level.props.iter().map(|p| crate::levelfit::StoodLevelPropsItem { parts: vec![crate::Opaque; p.parts.len()] }).collect()
}

fn stood_level_switchables(level: &crate::scene_level::Level) -> Vec<crate::levelfit::StoodLevelSwitchablesItem> {
    level.switchables.iter().map(|s| crate::levelfit::StoodLevelSwitchablesItem { boxes: s.boxes.clone(), group: s.group.clone() }).collect()
}

fn stood_level_devices(level: &crate::scene_level::Level) -> Option<Vec<crate::levelfit::StoodLevelDevicesItem>> {
    use crate::scene_level::DeviceDef;
    level.devices.as_ref().map(|list| list.iter().map(|d| crate::levelfit::StoodLevelDevicesItem {
        kind: String::from(match d { DeviceDef::Camera(_) => "camera", DeviceDef::Keypad(_) => "keypad", DeviceDef::Laser(_) => "laser" }),
    }).collect())
}

///  What levelfit reads of the game as it stands.
fn stood_of(g: &Game, seat: &crate::seats::SeatRef, level: &crate::scene_level::Level) -> crate::levelfit::Stood {
    crate::levelfit::Stood {
        level: crate::levelfit::StoodLevel {
            doors: vec![crate::Opaque; level.doors.len()],
            props: stood_level_props(level),
            boxes: g.col.boxes.clone(),
            lights: Some(vec![crate::Opaque; level.lights.len()]),
            switchables: Some(stood_level_switchables(level)),
            beacons: Some(vec![crate::Opaque; level.beacons.len()]),
            devices: stood_level_devices(level),
            civilians: level.civilians.as_ref().map(|c| vec![crate::Opaque; c.len()]),
        },
        col: crate::levelfit::StoodCol { boxes: g.col.boxes.clone(), dynamic_boxes: g.col.dynamic_boxes.clone() },
        props: Some(crate::levelfit::StoodProps {
            all: g.props.all.iter().map(|&p| crate::levelfit::StoodPropsAllItem { boxes: vec![crate::Opaque; g.tab.props_prop[p].boxes.len()] }).collect(),
        }),
        player: Some(seat.player),
        guards: Some(g.guards.clone()),
        civilians: Some(g.civilians.clone()),
        seats: Some(g.seats.clone()),
    }
}

/// `for (const s of this.seats) s.view.x = ...`: a pass about each seat.
fn stand_each_seat(g: &mut Game, f: &mut dyn FnMut(&mut Game)) {
    for s in g.seats.clone() { crate::seats::with_seat(g, s, &mut *f); }
}

///  A light's def by name, the last of a name winning as a Map's does: is it on the mains.
fn light_def_mains(level: &crate::scene_level::Level, name: &str) -> bool {
    match level.lights.iter().rev().find(|d| d.name == name) {
        Some(d) => d.mains != Some(false),
        None => true,
    }
}

///  A contactor group's restrike stagger: the die is drawn at the group's first appearance.
fn stagger_of(g: &mut Game, stagger: &mut JsMap<String, f64>, grp: &str) -> f64 {
    if !stagger.holds(grp) {
        let v = stagger.len() as f64 * 0.45 + crate::dice::draw_world(g, "game::stagger_of#1") * 0.5;
        stagger.set(String::from(grp), v);
    }
    *stagger.of(grp).unwrap()
}

/// A man says something.
pub fn game_say(g: &mut crate::game::Game, g_2: GuardId, text: &str, radio: Option<bool>, force: Option<bool>) {
    crate::guards::say(g, g_2, text, radio, force);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn def(name: &str, mains: Option<bool>) -> crate::scene_level::LightDef {
        crate::scene_level::LightDef { name: String::from(name), mains, ..crate::scene_level::LightDef::default() }
    }

    /// `new Map(level.lights.map(d => [d.name, d]))`: the last of a name wins, and a light with no def or no word is on the mains.
    #[test]
    fn a_lights_mains_is_the_last_def_of_its_name() {
        let mut level = crate::scene_level::Level::default();
        level.lights = vec![def("a", Some(false)), def("b", Some(false)), def("a", None)];
        assert!(light_def_mains(&level, "a"));
        assert!(!light_def_mains(&level, "b"));
        assert!(light_def_mains(&level, "nobody"));
    }
}
