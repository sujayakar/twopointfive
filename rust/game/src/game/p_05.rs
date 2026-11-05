//! src/game/game.ts lines 1199 to 1544
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// Makes a game, in `g`: the caller has set `g.engine`, `g.level`, `g.smoke` and `g.sim` on a blank Game, and takes the made game from the answer (a blank is left in `g`). (game.ts Game.constructor)
pub fn game_new(g: &mut crate::game::Game, ch: RigId, opts: Option<crate::game::GameNewOpts>) -> crate::game::Game {
    let level = g.level.clone();   // (read below while `g` is lent; a write to a box goes through mark_area_lit, to both holders)
    g.rig = ch;
    g.light_home = crate::sim_lightshome::light_home_new(g, crate::Opaque);   // (field initialisers: game.ts:855)
    g.falls = crate::sim_fallshome::fall_store_new(g, crate::Opaque);         // (game.ts:859)
    // the level's boxes as rows of the collision's table: the TypeScript's two names for one list (level.boxes, col.boxes), and every list of boxes below is of these IDs
    let ids: Vec<BoxId> = level.boxes.iter().map(|b| g.col.boxes.push(b.clone())).collect();
    if let Some(over) = crate::levelfit::level_over(g, &level_counts_of(&level, &ids)) { panic!("{}", over); }
    let n_seats = opts.and_then(|o| o.seats).unwrap_or(1.0);
    g.seats = crate::seats::seats_stand(g, n_seats);
    g.local_seat = Some(g.seats[0]);
    crate::difficulty::levers_fog(g, &mut |_g: &mut Game| {});   // (leversFog(loadTeamFeel): the load is panel storage, the lever's hand on FOG.own is the rule)
    let tune_chaos = g.k.chaos.def;   // (loadChaos' first line: its storage half is the host's)
    g.tune.chaos = tune_chaos;
    let tune_wind = g.k.wind.def;     // (loadWind's first line: ditto)
    g.tune.wind = tune_wind;
    let name = crate::scene_level_load::level_name_of(g);
    g.col = crate::scene_collision::new(g, &ids, name.as_deref());
    let (men, fn2000) = (g.men, g.fn2000);
    let spawn = level.player_spawn;
    g.player = crate::game::player_new(g, men, ch, spawn, Some(fn2000), None);   // (first.player: g.seat is seats[0] all through the constructor)
    // loose furniture and thrown things: never collision occluders
    g.props = crate::props::prop_system_new(g, &level.props, &mut game_on_prop_event, Some(crate::Opaque));
    g.items = crate::throwables::throwables_new(g, Some(crate::Opaque));
    g.tracers = crate::tracers::tracers_new(g, None, None);   // (its lights are g.engine.lights, reached by path: see notes)
    let device = g.engine.device;
    g.decals = crate::render_decals::decals_new(g, device);
    if !g.engine.settings.decals { g.decals.enabled = false; }
    g.weather = crate::weather::weather_new(g);
    let p = g.player;
    let player_light = g.tune.player_light;
    let rail = crate::rig_props::rail_light_def(crate::consts::PLAYER_ID, player_light);
    g.pl[p].light = crate::render_lights::light_set_add(&mut g.engine.lights, rail);
    // light targets from analytic lights + switchables; contactor groups restrike staggered by order of first appearance
    let mut group_stagger: JsMap<String, f64> = JsMap::new();
    let lights: Vec<LightId> = g.engine.lights.lights.clone();
    for &l in lights.iter() {
        let group = seam_light_group(g, l);
        if seam_light_ttl(g, l) >= 0.0 || seam_light_kind(g, l) == 2.0 || group == "player" || group == "flashlights" { continue; }
        let name = seam_light_name(g, l);
        let mains = light_def_mains(&level, &name);
        let fixture_box = seam_light_fixture_box(g, l);
        let def = crate::sim_lightshome::FixtureDef {
            name,
            pos: seam_light_pos(g, l),
            light: Some(l),
            boxes: if fixture_box >= 0.0 { vec![fixture_box] } else { Vec::new() },
            base_emissive: if fixture_box >= 0.0 { level.boxes[fixture_box as usize].emissive } else { [0.0, 0.0, 0.0] },
            base_intensity: seam_light_intensity(g, l),
            on: seam_light_enabled(g, l),
            broken: false,
            disabled_until: -1.0,
            fluorescent_flicker: false,
            factor: 1.0,
            mains,
            stagger: stagger_of(g, &mut group_stagger, &group),
            interactive: true,
            kind: crate::sim_lightshome::FixtureDefKind::Analytic,
            group,
            area_lights: None,
            lens: None,
        };
        let t = crate::sim_lightshome::light_home_add(g, &def);
        g.targets.push(t);
    }
    for sw in level.switchables.iter() {
        let b = &level.boxes[sw.boxes[0] as usize];
        let def = crate::sim_lightshome::FixtureDef {
            name: sw.name.clone(),
            group: sw.group.clone(),
            kind: crate::sim_lightshome::FixtureDefKind::Emissive,
            pos: b.c,
            light: None,
            boxes: sw.boxes.clone(),
            base_emissive: sw.base_emissive,
            base_intensity: 0.0,
            on: sw.on,
            broken: false,
            disabled_until: -1.0,
            fluorescent_flicker: sw.flicker == Some(true),
            factor: if sw.on { 1.0 } else { 0.0 },
            mains: sw.mains != Some(false),
            stagger: stagger_of(g, &mut group_stagger, &sw.group),
            interactive: sw.group != "server_leds" && sw.group != "server_glow",
            area_lights: None,
            lens: None,
        };
        let t = crate::sim_lightshome::light_home_add(g, &def);
        g.targets.push(t);
    }
    let car_lamps = crate::carlamps::car_lamp_targets(g, &ids);   // (the lamps of the level's cars: one target each, no die, none on the mains)
    for def in car_lamps.iter() {
        let t = crate::sim_lightshome::light_home_add(g, def);
        g.targets.push(t);
    }
    // flat emissive fixtures light the scene through analytic one-sided area lights; their boxes keep glowing, rays ignore the emissive term
    let targets: Vec<FixtureId> = g.targets.clone();
    for &t in targets.iter() {
        let (kind, group, name, boxes, e) = {
            let f = &g.tab.sim_lightshome_fixture[t];
            (f.kind, f.group.clone(), f.name.clone(), f.boxes.clone(), f.base_emissive)
        };
        if kind != crate::sim_lightshome::FixtureKind::Emissive || group == "server_leds" || group == "emergency" || group == crate::carlamps::VEHICLE_GROUP { continue; }
        for &bi in boxes.iter() {
            let b = &level.boxes[bi as usize];
            let (h, c) = (b.h, b.c);
            let thin: usize = if h[0] <= h[1] && h[0] <= h[2] { 0 } else if h[1] <= h[2] { 1 } else { 2 };
            if h[thin] > 0.06 { continue; }
            let (a0, a1) = ((thin + 1) % 3, (thin + 2) % 3);
            let area = 4.0 * h[a0] * h[a1];
            if area < 0.01 { continue; }
            // emitting side: ceiling fixtures face down; wall / desk fixtures whichever side is open
            let n: V3 = if thin == 1 {
                [0.0, if c[1] > 1.5 { -1.0 } else { 1.0 }, 0.0]
            } else {
                let (cs, sn) = (m::cos(b.yaw), m::sin(b.yaw));
                let ax: V3 = if thin == 0 { [cs, 0.0, -sn] } else { [sn, 0.0, cs] };
                let front = !crate::scene_collision::segment_blocked(g, c, v3::mad(c, ax, h[thin] + 0.35), None, None, None);
                let back = !crate::scene_collision::segment_blocked(g, c, v3::mad(c, ax, -(h[thin] + 0.35)), None, None, None);
                if front && !back { ax } else if !front && back { v3::scale(ax, -1.0) } else { ax }
            };
            let maxc = m::max_n(&[e[0], e[1], e[2], 1e-3]);
            let i0 = maxc * area;   // radiance x area = on-axis intensity
            let on = crate::sim_lightshome::fixture_on(g, t);
            let light = seam_add_light(g, &format!("{}_area{}", name, bi as i64), &group, 3.0, v3::mad(c, n, h[thin] + 0.07), n, [e[0] / maxc, e[1] / maxc, e[2] / maxc], i0,
                v3::clamp(m::sqrt(i0 / 0.0015), 3.0, 26.0), 0.0, 90.0,
                m::max(0.1, m::min(m::sqrt(area / m::PI), m::min(h[a0], h[a1]) + 0.02)), 0.0, on, i0, true, None);
            g.tab.sim_lightshome_fixture[t].area_lights.get_or_insert_with(Vec::new).push(crate::sim_lightshome::FixtureAreaLightsItem { light, base: i0 });
            mark_area_lit(g, bi as usize, ids[bi as usize]);
        }
    }
    // emissive markings are lights: a switchable box's marking with `emit` registers an emitter over its rect, on the same target's supply
    if g.engine.settings.markings {
        for &t in targets.iter() {
            let (kind, group, name, boxes) = {
                let f = &g.tab.sim_lightshome_fixture[t];
                (f.kind, f.group.clone(), f.name.clone(), f.boxes.clone())
            };
            if kind != crate::sim_lightshome::FixtureKind::Emissive || group == "emergency" { continue; }
            for &bi in boxes.iter() {
                let b = &level.boxes[bi as usize];
                let Some(marks) = b.marks.as_ref() else { continue };
                if marks.is_empty() || b.rot.is_some() { continue; }
                let mk_texel = g.engine.settings.marking_texel;
                let (albedo, c, h, yaw) = (b.albedo, b.c, b.h, b.yaw);
                for (k, mk) in marks.iter().enumerate() {
                    let st = crate::scene_markings::mark_stats(mk, albedo, Some(mk_texel));
                    if !js::truthy(st.emask) { continue; }
                    let maxc = m::max_n(&[st.mean_l[0], st.mean_l[1], st.mean_l[2]]);
                    let i0 = maxc * st.area;
                    if st.area < 0.01 || i0 < crate::game::MARK_LIGHT_MIN_I0 { continue; }   // LED rows, UI bars, buttons: they glow, they light nothing
                    let r = crate::scene_markings::mark_world_rect(mk, c, h, yaw, Some(mk_texel));
                    let n = r.n;
                    let on = crate::sim_lightshome::fixture_on(g, t);
                    let bii = bi as i64;
                    let mark_ref = if bi < 4096.0 && k < 4 { Some(((bii << 2) | k as i64) as f64) } else { None };
                    let light = seam_add_light(g, &format!("{}_mark{}_{}", name, bii, k), &group, 3.0, v3::mad(r.centre, n, 0.07), n, [st.mean_l[0] / maxc, st.mean_l[1] / maxc, st.mean_l[2] / maxc], i0,
                        v3::clamp(m::sqrt(i0 / 0.0015), 3.0, 26.0), 0.0, 90.0,
                        m::max(0.1, m::min(m::sqrt(st.area / m::PI), m::min(r.hu, r.hv) + 0.02)), 0.0, on, i0, true, mark_ref);
                    g.tab.sim_lightshome_fixture[t].area_lights.get_or_insert_with(Vec::new).push(crate::sim_lightshome::FixtureAreaLightsItem { light, base: i0 });
                    g.mark_lights.push(crate::game::GameMarkLightsItem { target: name.clone(), box_: bi, face: String::from(mk.face.as_str()), pos: v3::mad(r.centre, n, 0.07), i0, area: st.area, mean_l: st.mean_l });
                }
            }
        }
    }
    for b in level.beacons.iter() { mark_area_lit(g, b.box_ as usize, ids[b.box_ as usize]); }   // dome glow is screen-only; the beam is the spot light
    // mains breaker (OCP: temporary blackout, bullet: permanent) and the emergency beacons it brings up
    let breaker = crate::sim_lightshome::FixtureDef {
        name: String::from("breaker"),
        group: String::from("mains"),
        kind: crate::sim_lightshome::FixtureDefKind::Breaker,
        pos: level.breaker.pos,
        light: None,
        boxes: vec![level.breaker.boxes[1]],
        base_emissive: [0.3, 4.0, 0.6],
        base_intensity: 0.0,
        on: true,
        broken: false,
        disabled_until: -1.0,
        fluorescent_flicker: false,
        factor: 1.0,
        mains: false,
        stagger: 0.0,
        interactive: true,
        area_lights: None,
        lens: None,
    };
    let t = crate::sim_lightshome::light_home_add(g, &breaker);
    g.targets.push(t);
    for (i, b) in level.beacons.iter().enumerate() {
        let mut target: Option<FixtureId> = None;
        for &t in g.targets.iter() {
            let f = &g.tab.sim_lightshome_fixture[t];
            if f.kind == crate::sim_lightshome::FixtureKind::Emissive && f.boxes.contains(&b.box_) { target = Some(t); break; }
        }
        let Some(target) = target else { panic!("beacon {}: no switchable owns box {}", i, b.box_) };
        let (name, group) = { let f = &g.tab.sim_lightshome_fixture[target]; (f.name.clone(), f.group.clone()) };
        let light = seam_add_light(g, &name, &group, 1.0, b.pos, [1.0, 0.0, 0.0], [1.0, 0.22, 0.06], 55.0, 14.0, 9.0, 24.0, 0.06, 1.0, false, 55.0, false, None);
        g.beacons.push(crate::game::GameBeaconsItem { light, phase: i as f64 * 1.7, target });
    }
    // doors: leaves are dynamic occluders for sight / sound / projectiles
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
    g.doors = crate::doors::doors_new(g, &door_defs, &mut game_on_door_event, Some(crate::Opaque));
    let list: Vec<DoorId> = g.doors.list.clone();
    g.props.doorways = list.iter().map(|&d| {
        let (cc, dir, width) = (g.dr[d].frame_centre, g.dr[d].def.closed_dir, g.dr[d].def.width);
        crate::props::PropSystemDoorwaysItem { cx: cc[0], cz: cc[2], ux: m::cos(dir), uz: m::sin(dir), half: width / 2.0 - 0.02 }
    }).collect();
    let office = crate::dialogue::is_office(g);
    crate::doornames::name_doors(g, office, &list);
    g.door_deals = crate::doorswing::deal_swings(g);   // the side each leaf opens into: geometry only, no dice
    let leaves = crate::doors::doors_leaf_boxes(g);
    g.col.dynamic_boxes = leaves;
    let _ = crate::doors::doors_on_torn(g, &mut door_torn_note);   // a torn leaf is a row in the one feed of what the level no longer is
    g.devices = crate::devices::devices_new(g);   // cameras + emitters join `targets`, before the authored switch states are said, below
    // guards
    let tints: [V3; 3] = [[0.42, 0.36, 0.25], [0.3, 0.34, 0.4], [0.38, 0.28, 0.28]];
    for (i, &r) in level.routes.iter().enumerate() {
        let id = 10.0 + i as f64;
        let def = crate::rig_props::torch_light_def(&format!("flashlight{}", i), id, g.tune.flash_intensity, g.tune.flash_inner, g.tune.flash_outer);
        let fl = crate::render_lights::light_set_add(&mut g.engine.lights, def);
        let gd = crate::game::guard_new(g, men, ch, id, r, i as f64, fl, tints[i % tints.len()], None);
        g.guards.push(gd);
    }
    crate::opening::opening_start(g);   // a man whose route starts on top of the way in is found down his beat instead
    let n = level.mission.rack.index;
    let (leds, glow) = (format!("rack{}_leds", n), format!("rack{}_glow", n));
    let targets: Vec<FixtureId> = g.targets.clone();
    g.rack_targets = targets.iter().copied().filter(|&t| { let f = &g.tab.sim_lightshome_fixture[t]; f.name == leds || f.name == glow }).collect();
    g.destr = crate::destruction::destruction_new(g);   // after the targets and devices exist: a lit screen dies through its target
    g.glass_break = crate::glassbreak::glass_break_new(g);
    crate::winbars::boot_bars(g);
    g.shards = crate::shards::glass_shards_new(g);
    g.wind = crate::wind::wind_new(g);            // after the doors exist
    g.air_dust = crate::air_dust::air_dust_new(g);   // after the weather, the doors and the collision exist
    for &t in g.targets.clone().iter() {
        let on = if crate::sim_lightshome::fixture_on(g, t) { 1.0 } else { 0.0 };
        crate::sim_lightshome::light_home_in(g, crate::sim_lightshome::light_in().authored, Some(t), Some(on), None, None, None);   // the level as authored, for resetLevelState()
    }
    let list_now: Vec<LightId> = g.engine.lights.lights.clone();
    let targets_now: Vec<FixtureId> = g.targets.clone();
    let beacons_now: Vec<crate::sim_lightshome::BeaconFact> = g.beacons.iter().map(|b| crate::sim_lightshome::BeaconFact { light: b.light, phase: b.phase, target: b.target }).collect();
    let cones: Vec<crate::sim_lightshome::LightHomeStandConesItem> = g.devices.cameras.iter().map(|&c| { let r = &g.tab.devices_camera_rt[c]; crate::sim_lightshome::LightHomeStandConesItem { light: r.cone, target: r.target, slot: r.slot } }).collect();
    crate::sim_lightshome::light_home_stand(g, &list_now, &targets_now, &beacons_now, &cones);
    crate::civilians::spawn_civilians(g);   // after the shift, whose construction and dice they must not disturb
    g.fog_of_war = crate::fogofwar::fog_of_war_new(g);
    g.quiet = crate::quiet::quiet_mission_new(g);
    g.rails = crate::rails::rails_new(g);
    g.lesson = crate::lesson::lesson_new(g);
    g.snatch = crate::snatch::snatch_new(g);
    g.defuse = crate::defuse::defuse_new(g);
    g.night_breach = crate::nightbreach::night_breach_new(g);
    g.room_clear = crate::roomclear::room_clear_scenario_new(g);
    stand_each_seat(g, &mut |v: &mut Game| { v.orders = crate::orders::squad_orders_new(v, None); });
    g.fight = crate::roomfight::room_fight_new(g);
    g.cues = crate::cues::cues_new(g);
    g.squad_net = crate::squadnet::squad_net_new(g);
    g.cover = crate::cover::cover_new(g);
    g.entry = crate::entrypoints::entry_points_new(g);
    g.cleared = crate::cleared::cleared_rooms_new(g);
    g.rescue = crate::rescue::rescue_new(g);
    stand_each_seat(g, &mut |v: &mut Game| { v.door_tools = crate::doortools::door_tools_new(v); });   // after the orders: its rows are registered on their menu
    crate::pairup::pair_up_apply(g, None);
    g.pair = crate::pairorders::pair_orders_new(g);
    stand_each_seat(g, &mut |v: &mut Game| { v.work = crate::workthings::work_things_new(v); });
    stand_each_seat(g, &mut |v: &mut Game| { v.wall_charge = crate::wallcharge::wall_charge_new(v); });   // after the door tools: it spends their charges
    stand_each_seat(g, &mut |v: &mut Game| { crate::shutdoor::register_shut_door(v); });
    crate::seats::stand_men(g);   // a second seat's man and his light, made last
    g.director = crate::director::director_new(g, level.script.as_ref());   // the level's script: its define pass run and linted here, programs start on demand
    let stood = stood_of(g, &level, &ids);
    let tables = crate::scene_box_mirror::box_mirror_tables(g);
    if let Some(over) = crate::levelfit::stood_over(g, &stood, &tables) { panic!("{}", over); }
    core::mem::replace(g, Game::blank())
}

/// (game.ts constructor, `b.flags |= BoxFlag.AreaLit`) One box in the TypeScript, two holders here: the level's row and the collision's.
fn mark_area_lit(g: &mut Game, bi: usize, id: BoxId) {
    let bit = js::to_i32(crate::scene_boxes::box_flag().area_lit);
    g.level.boxes[bi].flags = (js::to_i32(g.level.boxes[bi].flags) | bit) as f64;
    g.col.boxes[id].flags = (js::to_i32(g.col.boxes[id].flags) | bit) as f64;
}

/// (game.ts:1202 levelOver(level)) What levelfit reads of a level, as the skeleton has it: lengths, a switchable's boxes, a device's kind.
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

/// (game.ts:1389 stoodOver(this, ...)) What levelfit reads of the game as it stands. `boxes`: the level's own rows, which is what col.boxes is in the TypeScript (see notes).
fn stood_of(g: &Game, level: &crate::scene_level::Level, ids: &[BoxId]) -> crate::levelfit::Stood {
    crate::levelfit::Stood {
        level: crate::levelfit::StoodLevel {
            doors: vec![crate::Opaque; level.doors.len()],
            props: stood_level_props(level),
            boxes: ids.to_vec(),
            lights: Some(vec![crate::Opaque; level.lights.len()]),
            switchables: Some(stood_level_switchables(level)),
            beacons: Some(vec![crate::Opaque; level.beacons.len()]),
            devices: stood_level_devices(level),
            civilians: level.civilians.as_ref().map(|c| vec![crate::Opaque; c.len()]),
        },
        col: crate::levelfit::StoodCol { boxes: ids.to_vec(), dynamic_boxes: g.col.dynamic_boxes.clone() },
        props: Some(crate::levelfit::StoodProps {
            all: g.props.all.iter().map(|&p| crate::levelfit::StoodPropsAllItem { boxes: vec![crate::Opaque; g.tab.props_prop[p].boxes.len()] }).collect(),
        }),
        player: Some(g.player),
        guards: Some(g.guards.clone()),
        civilians: Some(g.civilians.clone()),
        seats: Some(g.seats.clone()),
    }
}

/// (game.ts:1373,1381,1384-1386 `for (const s of this.seats) s.view.x = ...`) As the game is built a later seat has no man yet, so with_seat (keyed by a man) cannot name it: its set is swapped onto Game by its seat.
fn stand_each_seat(g: &mut Game, f: &mut dyn FnMut(&mut Game)) {
    for s in g.seats.clone() {
        if g.seat == s { f(g); continue; }   // (the Game's own fields are the first seat's set: seats_stand)
        let mut own = g.tab.cmd_seat[s].own.take().expect("a seat of a game of several has its own set");
        swap_seat_set(g, &mut own);
        f(g);
        swap_seat_set(g, &mut own);
        g.tab.cmd_seat[s].own = Some(own);
    }
}

/// seats.ts SEAT_KEYS, between Game and a seat's set.
fn swap_seat_set(g: &mut Game, own: &mut crate::seats::SeatSet) {
    use core::mem::swap;
    swap(&mut g.seat, &mut own.seat);
    swap(&mut g.player, &mut own.player);
    swap(&mut g.orders, &mut own.orders);
    swap(&mut g.door_tools, &mut own.door_tools);
    swap(&mut g.work, &mut own.work);
    swap(&mut g.wall_charge, &mut own.wall_charge);
    swap(&mut g.aim_point, &mut own.aim_point);
    swap(&mut g.aim_target, &mut own.aim_target);
    swap(&mut g.aim_guard, &mut own.aim_guard);
    swap(&mut g.aim_friend, &mut own.aim_friend);
    swap(&mut g.aim_snap, &mut own.aim_snap);
    swap(&mut g.hover, &mut own.hover);
    swap(&mut g.interactables, &mut own.interactables);
    swap(&mut g.nearby, &mut own.nearby);
    swap(&mut g.act, &mut own.act);
    swap(&mut g.use_door, &mut own.use_door);
    swap(&mut g.view_proj, &mut own.view_proj);
    swap(&mut g.threat_proximity, &mut own.threat_proximity);
    swap(&mut g.torch_on_t, &mut own.torch_on_t);
}

/// `eachSeat(g, v => ...)` of a stood game: each_seat's closure takes two `&mut Game`, so its body is here, through with_seat. (seats.ts eachSeat)
fn on_each_seat(g: &mut Game, f: &mut dyn FnMut(&mut Game)) {
    if g.seats.len() == 1 { f(g); return; }
    for s in g.seats.clone() {
        let man = crate::cmd::seat_man(g, s);
        crate::seats::with_seat(g, man, &mut *f);
    }
}

/// (game.ts constructor, lightDefs) A light's def by name, the last of a name winning as a Map's does: is it on the mains.
fn light_def_mains(level: &crate::scene_level::Level, name: &str) -> bool {
    match level.lights.iter().rev().find(|d| d.name == name) {
        Some(d) => d.mains != Some(false),
        None => true,
    }
}

/// (game.ts constructor, staggerOf) A contactor group's restrike stagger: the die is drawn at the group's first appearance.
fn stagger_of(g: &mut Game, stagger: &mut JsMap<String, f64>, grp: &str) -> f64 {
    if !stagger.has(String::from(grp)) {
        let v = stagger.len() as f64 * 0.45 + crate::math_dice::sim_random(g) * 0.5;
        stagger.set(String::from(grp), v);
    }
    *stagger.get(String::from(grp)).unwrap()
}

/// (game.ts constructor, doors.onTorn) A torn leaf as a row of the destruction's change feed.
fn door_torn_note(g: &mut Game, d: DoorId, who: f64, cause: crate::doors::DoorsOnTornFnCause) {
    let c = g.dr[d].frame_centre;
    let r = g.dr[d].def.width / 2.0 + 0.1;
    let id = match g.doors.list.iter().position(|&x| x == d) { Some(i) => i as f64, None => -1.0 };
    let row = crate::wallbreak::WallBreakNoteC {
        kind: crate::wallbreak::LevelChangeKind::Door,
        cause: match cause {
            crate::doors::DoorsOnTornFnCause::Blast => crate::wallbreak::WallBreakNoteCCause::Blast,
            crate::doors::DoorsOnTornFnCause::Fire => crate::wallbreak::WallBreakNoteCCause::Fire,
        },
        id,
        name: Some(g.dr[d].def.name.clone()),
        pos: [c[0], c[1], c[2]],
        rect: [c[0] - r, c[2] - r, c[0] + r, c[2] + r],
        opened: Vec::new(),
        walkable: true,
        sight: true,
        dead: true,
        who,
    };
    let _ = crate::wallbreak::wall_break_note(g, &row);
}

/// A line for the player's log; the log keeps five. (game.ts Game.msg)
pub fn game_msg(g: &mut crate::game::Game, text: &str) {
    let t = g.time;
    g.messages.push(crate::game::GameMessagesItem { text: String::from(text), t });
    if g.messages.len() > 5 { g.messages.remove(0); }
    crate::cues::cues_raw(g, text);
}

/// A man says something. (game.ts Game.say)
pub fn game_say(g: &mut crate::game::Game, g_2: GuardId, text: &str, radio: Option<bool>, force: Option<bool>, row: Option<bool>) {
    crate::guards::say(g, g_2, text, radio, force, row);
}

/// A utility or sandbox confirmation: the showcase's own plumbing stays out of the log. (game.ts Game.note)
pub fn game_note(g: &mut crate::game::Game, text: &str) {
    if !g.quiet_utility { game_msg(g, text); }
}

/// What a door's noise does: the man who made it is that loud, the sound plays, and it is filed for the men to hear. (game.ts Game.onDoorEvent)
pub fn game_on_door_event(g: &mut crate::game::Game, e: crate::doors::DoorEvent) {
    use crate::doors::DoorSound;
    if let Some(pl) = crate::seats::man_by_id(g, e.who) { g.pl[pl].noise = m::max(g.pl[pl].noise, e.level); }
    match e.sound {
        DoorSound::Creak => audio_play(g, "doorCreak", e.pos, 0.5 + 0.5 * e.level / 0.3),
        DoorSound::Bang => audio_play(g, "doorBang", e.pos, 0.9),
        DoorSound::Latch => audio_play(g, "doorLatch", e.pos, 0.6),
        DoorSound::Bash => {
            // (under 1: a leaf ripped round by its handle at a run, the same slam, lighter and shorter)
            if e.level < 1.0 { seam_audio_play_rate(g, "doorBash", e.pos, 0.6, 1.15); } else { audio_play(g, "doorBash", e.pos, 0.9); }
        }
        DoorSound::Rattle => audio_play(g, "doorRattle", e.pos, 0.35 + 2.0 * e.level),
        DoorSound::Pick => { let rate = 0.9 + crate::math_dice::sim_random(g) * 0.25; seam_audio_play_rate(g, "lockPick", e.pos, 0.5, rate); }
        DoorSound::Unlock => audio_play(g, "lockOpen", e.pos, 0.4 + 1.3 * e.level),
        DoorSound::Kick => {
            seam_audio_play_rate(g, "doorBash", e.pos, 1.0, 0.9);
            audio_play(g, "lockBreak", e.pos, 0.95);
            let kicker = match crate::seats::man_by_id(g, e.who) {
                Some(p) => Some(g.pl[p].ch),
                None => g.guards.iter().map(|&x| g.gd[x].ch).find(|&c| g.ch[c].id == e.who),
            };
            let from = match kicker { Some(c) => g.ch[c].pos, None => e.pos };
            crate::particle_fx::kick_dust_fx(g, e.door, from);
            if crate::doors::door_lock_broken(g, e.door) {
                let from = match kicker { Some(c) => g.ch[c].pos, None => e.pos };
                crate::particle_fx::lock_splinter_fx(g, e.door, from);
            }
            crate::destruction::destruction_thud(g, e.pos, Some(1.2), Some(e.who));
        }
        DoorSound::Thud => {
            // a boot at the face a shut door opens toward: the slam of it, nothing splintered
            seam_audio_play_rate(g, "doorBash", e.pos, 1.0, 0.8);
            if let Some(p) = crate::seats::man_by_id(g, e.who) {
                let tool = match &g.pl[p].kick { Some(k) => k.tool.is_some(), None => false };
                if !tool { game_msg(g, "it opens toward you, so the frame took the kick. that was loud"); }
            }
        }
        DoorSound::Clang => {
            // a boot on the steel stair door: the ring of it, no keep to splinter
            seam_audio_play_rate(g, "doorBash", e.pos, 1.0, 0.7);
            seam_audio_play_rate(g, "stairDoor", e.pos, 0.55, 1.3);
            if crate::seats::is_man_id(g, e.who) { game_msg(g, "it's a steel fire door, so it won't give. that was loud"); }
        }
    }
    // heard: a kick is its own kind, floor-wide; everything else a 'door' noise with a level, placed at the leaf
    let kicked = e.sound == DoorSound::Kick;
    let loud = (e.sound == DoorSound::Bash && e.level >= 1.0) || kicked || e.sound == DoorSound::Clang || e.sound == DoorSound::Thud || (e.sound == DoorSound::Bang && e.level > 0.6);
    let (time, serial) = (g.time, g.serial());
    g.events.push(crate::game::GameEvent {
        kind: if kicked { crate::director::EventKind::Kick } else { crate::director::EventKind::Door },
        pos: [e.pos[0], 0.0, e.pos[2]],
        time,
        loud,
        level: Some(e.level),
        who: Some(e.who),
        serial,
        ..crate::game::GameEvent::zeroed()
    });
}

/// The rolling chairs' voices, once a step right after the props have moved; nothing in the simulation reads this and no die is drawn. (game.ts Game.rollTick)
pub fn game_roll_tick(g: &mut crate::game::Game, dt: f64) {
    if g.rolls.is_empty() { return; }
    let voice = crate::game::roll_voice();
    let mut i = g.rolls.len();
    while i > 0 {
        i -= 1;
        let (p, rx, rz) = { let r = &g.rolls[i]; (r.p, r.x, r.z) };
        let here = g.props.props.contains(&p);
        let (px, pz) = (crate::props::prop_x(g, p), crate::props::prop_z(g, p));
        let mut v = if here && dt > 0.0 { m::hyp2(px - rx, pz - rz) / dt } else { 0.0 };
        if !v.is_finite() { v = 0.0; }
        g.rolls[i].x = px;
        g.rolls[i].z = pz;
        g.rolls[i].k = m::min(1.0, v / voice.full_v);
        let rolling = crate::props::prop_rolling(g, p);
        if here && rolling && v > voice.min_v {
            g.rolls[i].move_t = g.time;
            if !js::truthy(g.rolls[i].voice) {
                g.roll_seq += 1.0;
                let tok = g.roll_seq;
                g.rolls[i].voice = tok;
                seam_audio_play_follow(g, "chairRoll", [px, 0.05, pz], voice.gain, tok, p);
            }
        } else if js::truthy(g.rolls[i].voice) && (!here || g.time - g.rolls[i].move_t > voice.grace || g.time < g.rolls[i].move_t) {
            g.rolls[i].voice = 0.0;
        }
        if !js::truthy(g.rolls[i].voice) && !(here && rolling) { g.rolls.remove(i); }
    }
}

/// Furniture noise: a scrape or a knock, heard like door sounds and blamed on whoever moved it. A castor chair coasting is voiced by the roll tick and heard by nobody. (game.ts Game.onPropEvent)
pub fn game_on_prop_event(g: &mut crate::game::Game, e: crate::props::PropEvent) {
    use crate::props::{PropEventRoll, PropEventSound};
    if e.sound == PropEventSound::Roll {
        if e.roll == Some(PropEventRoll::Start) && !g.rolls.iter().any(|r| r.p == e.prop) {
            g.rolls.push(crate::game::Roll { p: e.prop, x: e.pos[0], z: e.pos[2], k: 0.0, move_t: -1e9, voice: 0.0 });   // (where it was as it started: the step may have moved it on)
        }
        return;
    }
    if let Some(pl) = crate::seats::man_by_id(g, e.who) { g.pl[pl].noise = m::max(g.pl[pl].noise, e.level); }
    let (thud_rate, scrape_rate) = { let t = &g.tab.props_prop[e.prop].tune; (t.thud_rate, t.scrape_rate) };
    let jitter = 0.95 + crate::math_dice::sim_random(g) * 0.1;
    if e.sound == PropEventSound::Thud {
        seam_audio_play_rate(g, "propThud", e.pos, v3::clamp(0.35 + 0.15 * e.speed, 0.3, 1.0), thud_rate * jitter);
        crate::destruction::destruction_thud(g, e.pos, Some(0.4 + 0.3 * e.speed), Some(e.who));
    } else if !e.rolling {
        seam_audio_play_rate(g, "propScrape", e.pos, v3::clamp(0.25 + 0.18 * e.speed, 0.25, 1.0), scrape_rate * jitter);
    }
    let (time, serial) = (g.time, g.serial());
    g.events.push(crate::game::GameEvent {
        kind: crate::director::EventKind::Prop,
        pos: v3::copy(e.pos),
        time,
        loud: e.level > 0.7,
        level: Some(e.level),
        who: Some(e.who),
        serial,
        ..crate::game::GameEvent::zeroed()
    });
}

/// A fresh shift: every system's memory of the old one goes, the response team first. (game.ts Game.resetGuards)
pub fn game_reset_guards(g: &mut crate::game::Game) {
    crate::powerout::power_out_reset(g);
    crate::rescuecheck::rescue_check_reset(g);
    crate::doze::doze_reset(g, None);
    on_each_seat(g, &mut |v: &mut Game| {
        crate::close::close_reset(v);
        crate::stalk::stalk_reset(v);
        crate::tapstep::tap_reset(v);
    });
    crate::roomfight::room_fight_reset(g);
    crate::cover::cover_reset(g);
    crate::entrypoints::entry_points_reset(g);
    crate::cleared::cleared_rooms_reset(g);
    crate::doorpath::door_path_reset(g);
    crate::wallreact::wall_react_reset_2(g);
    crate::coverfire::coverfire_reset(g);
    crate::doorforce::door_force_reset(g);
    crate::qrf::reset_qrf(g);
    crate::guards::reset_guards(g);
    crate::civilians::reset_civilians(g);
    crate::tactics::tactics_reset(g);
    crate::lanes::lanes_reset(g);
    crate::picture::picture_reset(g);
}

/// (game.ts Game.tension)
pub fn game_tension(g: &mut crate::game::Game, theirs: Option<bool>) -> f64 {
    crate::guards::tension(g, theirs)
}

/// One line on the alarm escalation for the panel. (game.ts Game.escalationSummary)
pub fn game_escalation_summary(g: &mut crate::game::Game) -> String {
    crate::guards::escalation_summary(g)
}

/// Panel: one level up by hand. (game.ts Game.escalate)
pub fn game_escalate(g: &mut crate::game::Game) {
    let to = if g.escalation >= 1.0 { 2.0 } else { 1.0 };
    crate::guards::raise_escalation(g, to, None, None, None);
}

/// Panel: straight back to calm. (game.ts Game.standDown)
pub fn game_stand_down(g: &mut crate::game::Game) {
    crate::guards::stand_down_escalation(g);
}

/// Panel: who is clearing what. (game.ts Game.clearingSummary)
pub fn game_clearing_summary(g: &mut crate::game::Game) -> String {
    crate::guards::clearing_summary(g)
}

/// Panel: one line per searching man, and who is pieing which doorway; empty when nobody is. (game.ts Game.searchSummary)
pub fn game_search_summary(g: &mut crate::game::Game) -> String {
    let mut parts: Vec<String> = Vec::new();
    for gd in g.guards.clone() {
        if g.gd[gd].state == crate::game::GuardState::Dead { continue; }
        let n = match g.level.routes.get(g.gd[gd].route_i) {
            Some(&r) => g.tab.route[r].name.clone(),
            None => g.gd[gd].callsign.clone(),
        };
        let s = crate::guards::search_summary(g, gd);
        if !s.is_empty() { parts.push(format!("{}: {}", n, s)); }
        if let Some(pie) = &g.gd[gd].pieing {
            parts.push(format!("{} pies the {} door ({})", n, g.dr[pie.door].def.name, pie.phase.as_str()));
        }
    }
    parts.join(" | ")
}

/// Panel: the human shield's standoff. (game.ts Game.standoffSummary)
pub fn game_standoff_summary(g: &mut crate::game::Game) -> String {
    crate::guards::standoff_summary(g)
}

/// Panel: what each unfound body reads on the light meter. (game.ts Game.bodyLightSummary)
pub fn game_body_light_summary(g: &mut crate::game::Game) -> String {
    crate::guards::body_light_summary(g)
}

/// Panel: everybody's suppression. (game.ts Game.suppressionSummary)
pub fn game_suppression_summary(g: &mut crate::game::Game) -> String {
    crate::suppression::suppression_summary(g)
}

/// (game.ts Game.conditionSummary)
pub fn game_condition_summary(g: &mut crate::game::Game) -> String {
    crate::condition::condition_summary(g)
}

/// Panel: the aim-settle model. (game.ts Game.aimSummary)
pub fn game_aim_summary(g: &mut crate::game::Game) -> String {
    crate::aim::aim_summary(g)
}

/// Panel: the pursuit tactics. (game.ts Game.tacticsSummary)
pub fn game_tactics_summary(g: &mut crate::game::Game) -> String {
    crate::tactics::tactics_summary(g)
}

/// Panel: the needs. (game.ts Game.needsSummary)
pub fn game_needs_summary(g: &mut crate::game::Game) -> String {
    crate::needs::needs_summary(g)
}

/// Panel: the civilians. (game.ts Game.civiliansSummary)
pub fn game_civilians_summary(g: &mut crate::game::Game) -> String {
    crate::civilians::civilians_summary(g)
}

/// Is this record somebody on the floor right now: a man of the shift, a unit or a civilian? (game.ts Game.onFloor)
pub fn game_on_floor(g: &mut crate::game::Game, gd: GuardId) -> bool {
    g.guards.contains(&gd) || g.civilians.contains(&gd)
}

/// Panel: one line per man, with his witness record. (game.ts Game.witnessSummary)
pub fn game_witness_summary(g: &mut crate::game::Game) -> String {
    let temperament = crate::game::temperament();
    let mut parts: Vec<String> = Vec::new();
    for gd in g.guards.clone() {
        let callsign = g.gd[gd].callsign.clone();
        let temper = match temperament.get(callsign.clone()) { Some(t) => t.as_str(), None => "?" };
        let state = if g.gd[gd].state == crate::game::GuardState::Dead {
            if g.gd[gd].wound.is_some() { ", wounded" } else if crate::game::is_breathing(g, gd) { ", out cold" } else { ", dead" }
        } else { "" };
        let w = crate::guards::witness_summary(g, gd);
        parts.push(format!("{} ({}{}): {}", callsign, temper, state, if w.is_empty() { "\u{2014}" } else { &w }));
    }
    parts.join(" | ")
}

/// The die to pass on: the caller's own, or none for the game's. (`rnd` handed to two calls in turn)
fn lend_rnd<'a>(rnd: &'a mut Option<&mut dyn FnMut(&mut Game) -> f64>) -> Option<&'a mut dyn FnMut(&mut Game) -> f64> {
    match rnd {
        Some(f) => Some(&mut **f),
        None => None,
    }
}

/// One exchange of the interrogation: the man answers from his own state, his line goes up, and the door's key or the pad's code comes with what he gives up. (game.ts Game.interrogate)
pub fn game_interrogate(g: &mut crate::game::Game, gd: GuardId, rnd: Option<&mut dyn FnMut(&mut crate::game::Game) -> f64>) -> Option<crate::dialogue::DialoguePick> {
    let mut rnd = rnd;
    let me = g.player;
    if g.pl[me].down || g.gd[gd].state == crate::game::GuardState::Dead || !(g.guards.contains(&gd) || g.civilians.contains(&gd)) { return None; }
    let held = match &g.pl[me].holding { Some(h) => Some(h.g), None => None };
    if held != Some(gd) {
        // not the man in his arm: in reach and nothing solid between, statics or a leaf
        let (a, b) = (g.ch[g.pl[me].ch].pos, g.ch[g.gd[gd].ch].pos);
        if v3::dist_xz(a, b) > crate::dialogue::INTERROGATE_REACH || crate::scene_collision::solid_blocked(g, [a[0], 1.0, a[2]], [b[0], 1.0, b[2]], None) { return None; }
    }
    let pick = if g.gd[gd].civ.is_some() { crate::civilians::civilian_answer(g, gd, lend_rnd(&mut rnd)) } else { crate::dialogue::interrogate(g, gd, lend_rnd(&mut rnd)) };
    let Some(pick) = pick else { return None };
    crate::guards::say(g, gd, &pick.text, Some(false), Some(true), None);   // (forced: a held man's only voice is his answers)
    if let Some(bubble) = g.gd[gd].bubble.as_mut() {
        bubble.dur = v3::clamp(1.2 + 0.07 * pick.text.encode_utf16().count() as f64, 2.5, 6.5);   // held speech runs longer than a bark's cap
    }
    let _ = crate::chronicle::chron_interrogated(g, gd, pick.kind.map(|k| k.as_str()), pick.stage_n);   // ahead of the key coming off his belt, its own row
    if pick.kind == Some(crate::dialogue::IntelKind::Door) {
        crate::player::take_key(g, gd, lend_rnd(&mut rnd));
    } else if pick.kind == Some(crate::dialogue::IntelKind::Code) {
        let at = g.ch[g.gd[gd].ch].pos;
        if let Some(k) = crate::devices::devices_pad_near(g, at) {
            let _ = crate::devices::learn_code(g, &k.def.name, crate::devices::LearnCodeHow::Told, Some(gd));
        }
    }
    Some(pick)
}

/// Let go of the man in Sam's arm at once, no shove. (game.ts Game.freeHeld)
pub fn game_free_held(g: &mut crate::game::Game, why: crate::game::GameFreeHeldWhy) {
    let why = match why {
        crate::game::GameFreeHeldWhy::Shot => crate::player::FreeHeldWhy::Shot,
        crate::game::GameFreeHeldWhy::Cancel => crate::player::FreeHeldWhy::Cancel,
    };
    crate::player::free_held(g, why);
}

/// The living man nearest Sam within `reach` metres; `foe`: only men hostile to the player. (game.ts Game.nearestLivingGuard)
pub fn game_nearest_living_guard(g: &mut crate::game::Game, reach: Option<f64>, foe: Option<bool>) -> Option<GuardId> {
    let foe = foe.unwrap_or(false);
    let mut best: Option<GuardId> = None;
    let mut bd = reach.unwrap_or(m::INF);
    for gd in g.guards.clone() {
        if g.gd[gd].state == crate::game::GuardState::Dead || (foe && !crate::teams::hostile_to(g, gd, crate::teams::HostileToO::Player)) { continue; }
        let d = v3::dist_xz(g.ch[g.gd[gd].ch].pos, g.ch[g.pl[g.player].ch].pos);
        if d <= bd { bd = d; best = Some(gd); }
    }
    best
}

/// Sandbox: lean on whoever is within reach; a note if nobody is. (game.ts Game.interrogateNearest)
pub fn game_interrogate_nearest(g: &mut crate::game::Game, rnd: Option<&mut dyn FnMut(&mut crate::game::Game) -> f64>) -> Option<crate::dialogue::DialoguePick> {
    if g.pl[g.player].down { game_note(g, "you're down \u{2014} nobody answers to a man on the floor"); return None; }
    let Some(gd) = game_nearest_living_guard(g, Some(crate::dialogue::INTERROGATE_REACH), Some(true)) else {
        game_note(g, &format!("nobody hostile within {} m to lean on \u{2014} walk up to a living guard", crate::dialogue::INTERROGATE_REACH));
        return None;
    };
    let pick = game_interrogate(g, gd, rnd);
    if pick.is_none() {
        let line = format!("{} is in reach but there is something solid between you", g.gd[gd].callsign);
        game_note(g, &line);
    }
    pick
}

/// Panel: the interrogation engine's read of the man nearest Sam, at any distance. (game.ts Game.dialogueSummary)
pub fn game_dialogue_summary(g: &mut crate::game::Game) -> String {
    let Some(gd) = game_nearest_living_guard(g, Some(m::INF), Some(true)) else { return String::from("dialogue: nobody left standing"); };
    let d = v3::dist_xz(g.ch[g.gd[gd].ch].pos, g.ch[g.pl[g.player].ch].pos);
    let s = crate::dialogue::dialogue_summary(g, gd);
    if d <= crate::dialogue::INTERROGATE_REACH { format!("dialogue \u{25B8} {} \u{B7} in reach", s) } else { format!("dialogue \u{25B8} {} \u{B7} {:.1} m off", s, d) }
}

/// `g.audio.play(name, pos, gain)`.
fn audio_play(g: &mut Game, name: &str, pos: V3, gain: f64) {
    crate::audio_iface::game_audio_like_play(g, name, Some(pos), Some(gain), None);
}

// ---- seams: each step still blocked in skeleton 3, in the TypeScript's own arguments (notes.md, Skeleton)

fn seam_light_ttl(g: &mut Game, l: LightId) -> f64 { todo!("needs: a light's fields by LightId") }
fn seam_light_kind(g: &mut Game, l: LightId) -> f64 { todo!("needs: a light's fields by LightId") }
fn seam_light_group(g: &mut Game, l: LightId) -> String { todo!("needs: a light's fields by LightId") }
fn seam_light_name(g: &mut Game, l: LightId) -> String { todo!("needs: a light's fields by LightId") }
fn seam_light_pos(g: &mut Game, l: LightId) -> V3 { todo!("needs: a light's fields by LightId") }
fn seam_light_fixture_box(g: &mut Game, l: LightId) -> f64 { todo!("needs: a light's fields by LightId") }
fn seam_light_intensity(g: &mut Game, l: LightId) -> f64 { todo!("needs: a light's fields by LightId") }
fn seam_light_enabled(g: &mut Game, l: LightId) -> bool { todo!("needs: a light's fields by LightId") }
/// engine.lights.add of the literal, whose other fields are always owner 0, fixtureBox -1, flicker 0, ttl -1, age 0.
fn seam_add_light(g: &mut Game, name: &str, group: &str, kind: f64, pos: V3, dir: V3, color: V3, intensity: f64, range: f64, inner_deg: f64, outer_deg: f64, radius: f64, volumetric: f64, enabled: bool, peak_intensity: f64, broad: bool, mark_ref: Option<f64>) -> LightId { todo!("needs: a maker of a light from the literal's fields (light_set_add takes a LightId, light_from_def lacks markRef)") }
/// g.audio.play with `{ rate }`.
fn seam_audio_play_rate(g: &mut Game, name: &str, pos: V3, gain: f64, rate: f64) { todo!("needs: game_audio_like_play's opts is JsMap<String, Opaque>, which cannot hold the number `rate`") }
/// audio.play with `follow`: the voice lives while the roll's voice is still `tok`, and asks the prop where it is and the roll how loud (roll row by its prop).
fn seam_audio_play_follow(g: &mut Game, name: &str, at: V3, gain: f64, tok: f64, p: PropId) { todo!("needs: game_audio_like_play's opts to carry a follow keyed by token (a closure in the TypeScript)") }

#[cfg(test)]
mod tests {
    use super::*;

    fn blank_game() -> Box<Game> { Box::new(Game::blank()) }

    /// A later seat's set is on Game for its pass and back in its row after; the first seat's is the Game's own.
    #[test]
    fn stand_each_seat_walks_every_set() {
        let mut g = blank_game();
        let s0 = g.tab.cmd_seat.push(crate::cmd::Seat::blank());
        let s1 = g.tab.cmd_seat.push(crate::cmd::Seat::blank());
        let mut own = crate::seats::SeatSet::zeroed();
        own.seat = s1;
        g.tab.cmd_seat[s1].own = Some(own);
        g.seats = vec![s0, s1];
        g.seat = s0;
        let mut seen: Vec<SeatId> = Vec::new();
        stand_each_seat(&mut g, &mut |v: &mut Game| {
            seen.push(v.seat);
            v.threat_proximity = if v.seat.ix() == 0 { 10.0 } else { 20.0 };
        });
        assert_eq!(seen, vec![s0, s1]);
        assert_eq!(g.seat, s0);
        assert_eq!(g.threat_proximity, 10.0);
        let own = g.tab.cmd_seat[s1].own.as_ref().unwrap();
        assert_eq!(own.seat, s1);
        assert_eq!(own.threat_proximity, 20.0);
    }

    /// One box in the TypeScript: the bit lands in both holders and leaves the other bits.
    #[test]
    fn mark_area_lit_writes_both_holders() {
        let mut g = blank_game();
        let mut b = crate::scene_boxes::Box::zeroed();
        b.flags = 2.0;
        g.level.boxes.push(b.clone());
        let id = g.col.boxes.push(b);
        mark_area_lit(&mut g, 0, id);
        assert_eq!(g.level.boxes[0].flags, 34.0);
        assert_eq!(g.col.boxes[id].flags, 34.0);
    }
}
