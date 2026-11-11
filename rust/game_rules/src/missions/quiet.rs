pub use ::game_types::quiet::types;
pub use types::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{format, string::String, sync::Arc, vec, vec::Vec};

/// the switches as they stand now
fn held_now(g: &crate::game::Game) -> crate::quiet::Held {
    crate::quiet::Held { pairup: g.k.pairup.on, pair: g.k.pair.on, sneak: g.k.sneak.on, spotted: g.k.spotted.on, silent: g.k.silent.on, silent_start: g.k.silent.start }
}

/// the switches written
fn held_set(g: &mut crate::game::Game, h: crate::quiet::Held) {
    g.k.pairup.on = h.pairup;
    g.k.pair.on = h.pair;
    g.k.sneak.on = h.sneak;
    g.k.spotted.on = h.spotted;
    g.k.silent.on = h.silent;
    g.k.silent.start = h.silent_start;
}

/// out through this exit: past its point going its own way, inside its width
pub(super) fn out_through(x: &crate::scene_scenarios::QuietExit, p: V3) -> bool {
    let dx = p[0] - x.x;
    let dz = p[2] - x.z;
    let along = dx * x.out[0] + dz * x.out[1];
    let side = m::abs(dx * x.out[1] - dz * x.out[0]);
    along > 0.0 && along < EXIT_DEPTH && side < x.half_w.unwrap_or(EXIT_HALF_W)
}

/// a point a stride past the exit, on its own line
pub fn exit_point(x: &crate::scene_scenarios::QuietExit, past: Option<f64>) -> V3 {
    let past = past.unwrap_or(0.9);
    [x.x + x.out[0] * past, 0.0, x.z + x.out[1] * past]
}

/// the start dealt
fn quiet_mission_start_(g: &crate::game::Game) -> Option<crate::scene_scenarios::QuietStart> {
    if !crate::quiet::quiet_of(g) { return None; }
    let at = g.quiet.deal.start as usize;
    g.quiet.data.as_ref().and_then(|q| q.starts.get(at).cloned())
}

/// the routes manned on this run, by name
fn quiet_mission_patrol(g: &crate::game::Game) -> Vec<String> {
    if !crate::quiet::quiet_of(g) { return Vec::new(); }
    let q = match g.quiet.data.as_ref() { Some(q) => q, None => return Vec::new() };
    let at = g.quiet.deal.patrol;   // (−1: past every list, so every route)
    if at >= 0.0 {
        if let Some(set) = q.patrols.as_ref().and_then(|p| p.get(at as usize)) { return set.clone(); }
    }
    g.level.routes.iter().map(|&r| g.tab.route[r].name.clone()).collect()
}

/// why this level can carry none
fn quiet_mission_level_refuses(g: &crate::game::Game) -> Option<String> {
    let q = match g.level.scenarios.as_ref().and_then(|s| s.quiet.as_ref()) {
        Some(q) => q,
        None => return Some(String::from("this level carries no quiet mission (its scenarios.quiet block)")),
    };
    if q.starts.is_empty() || q.spots.is_empty() || q.exits.is_empty() { return Some(String::from("its quiet mission wants at least one start, one spot and one exit")); }
    None
}

/// why it cannot start right now
fn quiet_mission_can_start(g: &crate::game::Game, squad: Option<f64>) -> Option<String> {
    let squad = squad.unwrap_or(0.0);
    if let Some(why) = quiet_mission_level_refuses(g) { return Some(why); }
    if !g.ai_enabled { return Some(String::from("the AI is switched off (sandbox: no AI) — nobody would walk his route")); }
    if js::truthy(squad) && !g.tune.teams { return Some(String::from("sides are switched off (tune.teams): no partner")); }
    if let Some(r) = g.director.run {
        if !g.tab.director_run[r].ended { return Some(format!("the level script's program '{}' is running", g.tab.director_run[r].name)); }
    }
    if crate::nightbreach::night_breach_up(g) || crate::rescue::rescue_up(g) { return Some(String::from("another scenario is staged")); }
    if g.guards.iter().any(|&x| g.gd[x].unit.is_some()) { return Some(String::from("a spawned unit is already on the floor")); }
    None
}

/// Start it: the deal, a fresh encounter on it, the partner beside Sam. The reason when refused or when the staging failed
pub(super) fn quiet_mission_start(g: &mut crate::game::Game, pin: Option<f64>, squad: Option<f64>) -> Option<String> {
    let squad = squad.unwrap_or(0.0);
    if crate::quiet::quiet_of(g) { return None; }
    if let Some(why) = quiet_mission_can_start(g, Some(squad)) { return Some(why); }
    g.quiet.data = g.level.scenarios.as_ref().and_then(|s| s.quiet.clone());
    g.quiet.pin = pin;
    g.quiet.squad = squad;
    g.quiet.failure = None;
    quiet_mission_current(g);
    if js::truthy(squad) {
        g.quiet.was = Some(held_now(g));
        g.m.quiet.holder = true;
        g.k.pairup.on = true;
        crate::pairup::pair_up_apply(g, None);
        g.k.silent.on = true;
        g.k.silent.start = true;
        g.quiet.mine = Some(held_now(g));
    }
    g.staged.begin(crate::scene_scenarios::MissionKind::Quiet, crate::mission::Facts { unseen: true, ..Default::default() });
    quiet_mission_rack(g);
    quiet_mission_off_shift(g);
    quiet_mission_stage(g);
    if crate::quiet::quiet_of(g) { None } else { Some(g.quiet.failure.clone().unwrap_or_else(|| String::from("the staging failed"))) }
}

/// Stop it: the partner gone, every route manned, the authored mission and spawn back
fn quiet_mission_stop(g: &mut crate::game::Game) {
    if !crate::quiet::quiet_of(g) { return; }
    g.staged.down();
    quiet_mission_put_back(g);
    quiet_mission_release(g);
    g.staged.team = Vec::new();
    g.quiet.data = None;
    g.quiet.pending = None;
}

/// the level as authored
fn quiet_mission_put_back(g: &mut crate::game::Game) {
    quiet_mission_unbench(g);
    if let Some(a) = g.quiet.authored.take() {
        let l = Arc::make_mut(&mut g.level);
        l.player_spawn = a.spawn;
        l.errand = a.errand;
    }
    quiet_mission_rack(g);
}

/// THE RUN IS OVER: the switches it took put back, each only while it still stands as the run wrote it
pub fn quiet_mission_release(g: &mut crate::game::Game) {
    if let (Some(w), Some(mine)) = (g.quiet.was, g.quiet.mine) {
        let n = held_now(g);
        held_set(g, released(w, mine, n));
    }
    g.quiet.was = None;
    g.quiet.mine = None;
    g.m.quiet.holder = false;
}

/// each switch: what stood before the run where it still stands as the run wrote it, else the hand that moved it
fn released(w: crate::quiet::Held, mine: crate::quiet::Held, n: crate::quiet::Held) -> crate::quiet::Held {
    let keep = |now: bool, ran: bool, was: bool| if now == ran { was } else { now };
    crate::quiet::Held {
        pairup: keep(n.pairup, mine.pairup, w.pairup),
        pair: keep(n.pair, mine.pair, w.pair),
        sneak: keep(n.sneak, mine.sneak, w.sneak),
        spotted: keep(n.spotted, mine.spotted, w.spotted),
        silent: keep(n.silent, mine.silent, w.silent),
        silent_start: keep(n.silent_start, mine.silent_start, w.silent_start),
    }
}

/// THIS GAME IS THE ONE RUNNING: its own run, if up, takes the switches again. No other Game's run can hold them here: they are this Game's
pub fn quiet_mission_current(g: &mut crate::game::Game) {
    if g.m.quiet.holder { return; }
    if crate::quiet::quiet_of(g) && js::truthy(g.quiet.squad) {
        if let Some(mine) = g.quiet.mine {
            g.quiet.was = Some(held_now(g));
            held_set(g, mine);
            g.m.quiet.holder = true;
        }
    }
}

/// THE DEAL, BEFORE THE MEN ARE MADE: where they stand and what they make for are written into the level they read. Where the level or the seats refuse the mission nothing is written, and the men stand on the level as authored
pub fn quiet_mission_deal(g: &mut crate::game::Game, pin: Option<f64>, seats: f64) {
    if seats > 1.0 && !crate::seats::seats_kind("quiet") || quiet_mission_level_refuses(g).is_some() { return; }
    let q = match g.level.scenarios.as_ref().and_then(|s| s.quiet.clone()) { Some(q) => q, None => return };
    g.quiet.pin = pin;
    g.quiet.deal = match g.quiet.pending.take() { Some(d) => d, None => quiet_mission_deal_one(g, &q) };
    g.quiet.authored = Some(crate::quiet::QuietMissionAuthored { spawn: g.level.player_spawn, errand: g.level.errand.clone() });
    let s = &q.starts[g.quiet.deal.start as usize];
    let p = &q.spots[g.quiet.deal.spot as usize];
    let l = Arc::make_mut(&mut g.level);
    l.player_spawn = [s.sam[0], 0.0, s.sam[2]];
    l.errand = crate::mission::Errand {
        place: p.place.clone(), area: p.area, door: p.door.clone(),
        thing: q.thing.clone().unwrap_or_else(|| String::from("the drive")), holder: p.holder.clone(), index: p.rack_index, front: p.front, half_w: p.half_w, told: p.wired.unwrap_or(false),
        exits: q.exits.clone(),
    };
}

/// the routes this run does not man: off the guard list, their torches out of the light set
fn quiet_mission_off_shift(g: &mut crate::game::Game) {
    let manned = quiet_mission_patrol(g);
    let off: Vec<GuardId> = g.guards.iter().copied().filter(|&x| g.gd[x].unit.is_none() && !manned.contains(&g.tab.route[g.gd[x].route].name)).collect();
    for &gd in &off {
        let l = g.gd[gd].flashlight;
        g.tab.light[l].enabled = false;
        crate::render_lights::light_set_remove(g, l);
    }
    if !off.is_empty() { g.guards.retain(|x| !off.contains(x)); }
    g.quiet.benched = off;
}

/// Sam faced as the start has him, the partner staged. A staging that fails ends the mission cleanly
fn quiet_mission_stage(g: &mut crate::game::Game) {
    if let Err(e) = stage_or_why(g) {
        let failure = format!("the staging failed: {}", e);
        g.quiet.failure = Some(failure.clone());
        crate::stickycam::sticky_visit_set(g, 0.0);
        quiet_mission_stop(g);
    }
}

/// once a frame: the camera put on a dealt start, the partner told to follow the first tick the orders have him
pub fn quiet_mission_tick(g: &mut crate::game::Game, cam: Option<CameraId>) {
    if !crate::quiet::quiet_of(g) { return; }
    if let (true, Some(cam)) = (g.quiet.cut, cam) {   // (with no camera the cut waits for a tick that has one)
        g.quiet.cut = false;
        let target = v3::copy(crate::game::game_follow_pos(g, None));
        cut_lens(&mut g.tab.camera[cam], target);
    }
    if !g.quiet.fall_in { return; }
    let m = match g.staged.team.first().copied() { Some(m) => m, None => return };
    if crate::condition::down(g, g.gd[m].ch) || !crate::orders::squad_orders_active(g, &crate::seats::here(g)) { return; }
    g.quiet.fall_in = false;
    let at = g.player_pos(g.player);
    let target = crate::orders::squad_orders_target_at(g, at);
    crate::orders::squad_orders_give(g, crate::orders::rows::fallin::ORDER.id, Some(&target), Some(crate::orders::Who::All), None);
}

/// `cam.target = v3.copy(p); cam.desired = v3.copy(cam.target)`: a whole write of `desired` is what Seat.takeCut watches for, here by its serial
fn cut_lens(lens: &mut crate::camera::FollowCamera, p: V3) {
    lens.target = p;
    lens.desired = p;
    lens.desired_serial = lens.desired_serial.wrapping_add(1);
}

/// one deal: the pinned seed's, and with no pin the first of each list
fn quiet_mission_deal_one(g: &crate::game::Game, q: &crate::scene_scenarios::QuietData) -> crate::quiet::QuietDeal {
    let sets = q.patrols.as_ref().map(|p| p.len());
    match g.quiet.pin {
        None => crate::quiet::QuietDeal { start: 0.0, spot: 0.0, patrol: if sets.map_or(false, |n| n > 0) { 0.0 } else { -1.0 } },
        Some(pin) => pinned_deal(crate::dice::Die(pin_seed(pin) as u32), q.starts.len(), q.spots.len(), sets),
    }
}

/// `(pin | 0) ^ 0x51ed270b`: a pinned deal's seed
fn pin_seed(pin: f64) -> i32 {
    js::to_i32(pin) ^ 0x51ed270b
}

/// a pinned deal, off a stream of its own: three draws whatever the lists' lengths
fn pinned_deal(mut die: crate::dice::Die, starts: usize, spots: usize, sets: Option<usize>) -> crate::quiet::QuietDeal {
    let mut pick = |n: f64| m::min(n - 1.0, m::floor(die.roll() * n));
    let start = pick(starts as f64);
    let spot = pick(spots as f64);
    let patrol = pick(m::max(1.0, sets.unwrap_or(1) as f64));
    crate::quiet::QuietDeal { start, spot, patrol: if sets.map_or(false, |n| n > 0) { patrol } else { -1.0 } }
}

/// the benched routes' men back on the guard list, in route order, torches back in the light set
fn quiet_mission_unbench(g: &mut crate::game::Game) {
    if g.quiet.benched.is_empty() { return; }
    let benched = g.quiet.benched.clone();
    for &gd in &benched {
        let l = g.gd[gd].flashlight;
        if !g.engine.lights.lights.contains(&l) { crate::render_lights::light_set_add(g, l); }
    }
    let mut staff: Vec<(i32, GuardId)> = g.guards.iter().copied().filter(|&x| g.gd[x].unit.is_none()).chain(benched.iter().copied()).map(|x| (g.gd[x].route_i, x)).collect();
    js::sort_by(&mut staff, |a, b| a.0 as f64 - b.0 as f64);
    let mut guards: Vec<GuardId> = staff.iter().map(|s| s.1).collect();
    guards.extend(g.guards.iter().copied().filter(|&x| g.gd[x].unit.is_some()));
    g.guards = guards;
    g.quiet.benched = Vec::new();
}

/// what goes dark when the thing comes out: the switchables named for the mission's holder
fn quiet_mission_rack(g: &mut crate::game::Game) {
    let n = js::num_str(g.level.errand.index);
    let leds = format!("rack{}_leds", n);
    let glow = format!("rack{}_glow", n);
    g.rack_targets = g.targets.iter().copied().filter(|&t| { let name = &g.tab.fixture[t].name; *name == leds || *name == glow }).collect();
}

/// the staging, for whoever can take its failure
fn stage_or_why(g: &mut crate::game::Game) -> Result<(), String> {
    let s = quiet_mission_start_(g).unwrap();
    let pl = g.pl[g.player].ch;
    g.staged.team = Vec::new();
    g.ch[pl].aim_yaw = s.sam_yaw;
    g.ch[pl].body_yaw = s.sam_yaw;
    crate::character::character_update(g, pl, 0.0);
    if js::truthy(g.quiet.squad) {
        let at = match s.mate { Some(p) => v3::copy(p), None => behind_or_why(g, &s)? };
        let man = crate::guards::spawn_unit(g, &crate::guards::UnitOpts {
            team: crate::teams::Team::Ours,
            pos: at,
            yaw: Some(s.sam_yaw),
            name: Some(String::from("Blue One")),
            light: Some(false),
            pair: Some(crate::game::PairId::Blue),
            ..crate::guards::UnitOpts::default()
        })?;
        crate::mind::set_state(g, man, crate::game::GuardState::Patrol, "quiet");
        crate::mind::set(g, man, 0.0, "quiet");
        crate::duty::hold(g, man, false, "quiet");
        crate::duty::pin(g, man, false, "quiet");
        let face = g.ch[g.gd[man].ch].body_yaw;
        crate::squad::order(g, man, crate::squad::OrderPatch {
            goal: Some(None),
            face: Some(Some(face)),
            stance: Some(Some(crate::character::Stance::LowReady)),
            upper: Some(Some(crate::character::UpperMode::None)),
            aim_at: Some(None),
            light: Some(Some(false)),
            crouch: Some(Some(false)),
            ..crate::squad::OrderPatch::default()
        });
        g.staged.team.push(man);
        g.quiet.fall_in = true;
        let ops = g.staged.team.clone();
        crate::kit::kit_stage(g, &ops);
    }
    g.quiet.cut = true;
    Ok(())
}

/// the free floor nearest a stride behind Sam, walkable from him; else the fault an author is told
fn behind_or_why(g: &crate::game::Game, s: &crate::scene_scenarios::QuietStart) -> Result<V3, String> {
    let back: V3 = [s.sam[0] - m::sin(s.sam_yaw) * 1.1, 0.0, s.sam[2] - m::cos(s.sam_yaw) * 1.1];
    let mut best: Option<V3> = None;
    let mut bs = m::INF;
    let mut dx = -2.0;
    while dx <= 2.0 {
        let mut dz = -2.0;
        while dz <= 2.0 {
            let p: V3 = [back[0] + dx, 0.0, back[2] + dz];
            let skip = crate::scene_collision::nav_grid_is_blocked(g, p[0], p[2]) || v3::dist_xz(p, s.sam) < 0.8 || !crate::scene_collision::nav_grid_walkable(g, s.sam, p);
            if !skip {
                let d = m::hyp2(dx, dz);
                if d < bs { bs = d; best = Some(p); }
            }
            dz += 0.5;
        }
        dx += 0.5;
    }
    match best {
        Some(p) => Ok(p),
        None => Err(format!("no free floor beside the start at ({}, {}) for the partner — give the start a 'mate' mark", js::num_str(s.sam[0]), js::num_str(s.sam[2]))),
    }
}

/// THE FLOOR A MAN STANDS ON AT A HOLDER'S FACE: the nearest free cell with a clear line to the face and, given the room, inside it
pub fn holder_stand(g: &crate::game::Game, front: V3, area: Option<crate::quiet::HolderStandArea>) -> crate::quiet::HolderStand {
    let e = 0.3;
    let mut best: Option<V3> = None;
    let mut bs = m::INF;
    let mut clear = false;
    let mut inside = false;
    for i in -6..=6 {
        for j in -6..=6 {
            let dx = i as f64 * 0.2;
            let dz = j as f64 * 0.2;
            let x = front[0] + dx;
            let z = front[2] + dz;
            if crate::scene_collision::nav_grid_is_blocked(g, x, z) { continue; }
            let c = !crate::scene_collision::solid_blocked(g, [x, 1.0, z], [front[0], 1.0, front[2]], None);
            let n = match area { None => true, Some(a) => x >= a.x0 - e && x <= a.x1 + e && z >= a.z0 - e && z <= a.z1 + e };
            let s = m::hyp2(dx, dz) + (if c { 0.0 } else { 100.0 }) + (if n { 0.0 } else { 10.0 });   // (a clear line first, then the room, then the nearest)
            if s < bs { bs = s; best = Some([x, 0.0, z]); clear = c; inside = n; }
        }
    }
    crate::quiet::HolderStand { pos: best, clear, inside }
}

/// the quiet mission that is up on this game
pub fn quiet_of(g: &crate::game::Game) -> bool {
    g.staged.up == Some(crate::scene_scenarios::MissionKind::Quiet)
}

/// a point as a fault line writes it
fn at(p: V3) -> String {
    format!("({}, {})", js::num_str(p[0]), js::num_str(p[2]))
}

/// free floor, or a fault said
fn free(g: &mut crate::game::Game, out: &mut Vec<String>, what: &str, p: V3) -> bool {
    let b = crate::scene_collision::nav_grid_is_blocked(g, p[0], p[2]);
    if b { out.push(format!("{} {} is not free floor a man can stand on", what, at(p))); }
    !b
}

/// a way on foot
fn way(g: &crate::game::Game, a: V3, b: V3) -> bool {
    crate::scene_collision::nav_grid_find_path(g, a, b, Some(20000.0), None).map_or(false, |path| !path.is_empty())
}

/// a route by its name; the last of a name, as a Map built from the list has it
fn route_named(g: &crate::game::Game, name: &str) -> Option<RouteId> {
    g.level.routes.iter().rev().copied().find(|&r| g.tab.route[r].name == name)
}

/// THE BLOCK AGAINST THE BUILT FLOOR: what the level file's validator cannot say. `g` is a Game stood up on the level, nothing started; the faults, each a line an author can act on
pub fn check_quiet(g: &mut crate::game::Game) -> Vec<String> {
    let Some(q) = g.level.scenarios.as_ref().and_then(|s| s.quiet.clone()) else { return vec![String::from("the level carries no scenarios.quiet block")] };
    let mut out: Vec<String> = Vec::new();
    check_starts(g, &q, &mut out);
    let fronts = check_spots(g, &q, &mut out);
    let outs = check_exits(g, &q, &mut out);
    check_ways_on_foot(g, &q, &fronts, &outs, &mut out);
    check_first_sights(g, &q, &mut out);
    check_needs(g, &q, &mut out);
    check_civilians_places(g, &q, &mut out);
    out
}

/// Each start stands on floor, with the mate a body's room from Sam and a walk to him.
fn check_starts(g: &mut Game, q: &crate::scene_scenarios::QuietData, out: &mut Vec<String>) {
    for (i, s) in q.starts.iter().enumerate() {
        let w = match s.name.as_deref() { Some(n) if !n.is_empty() => format!("starts[{}] '{}'", i, n), _ => format!("starts[{}]", i) };
        free(g, out, &format!("{}.sam", w), s.sam);
        let Some(mate) = s.mate else { continue };
        if free(g, out, &format!("{}.mate", w), mate) && !crate::scene_collision::nav_grid_walkable(g, s.sam, mate) && !way(g, s.sam, mate) {
            out.push(format!("{}.mate {} cannot be walked to from sam {}", w, at(mate), at(s.sam)));
        }
        if v3::dist_xz(s.sam, mate) < 0.7 {
            out.push(format!("{}.mate stands on top of sam ({} m): give them a body's room", w, js::to_fixed(v3::dist_xz(s.sam, mate), 2)));
        }
    }
}

/// Each holder's face is in its area and can be stood at. Where a man stands to take each, if he can.
fn check_spots(g: &Game, q: &crate::scene_scenarios::QuietData, out: &mut Vec<String>) -> Vec<Option<V3>> {
    let mut fronts: Vec<Option<V3>> = Vec::new();
    for (i, p) in q.spots.iter().enumerate() {
        let w = format!("spots[{}] '{}'", i, p.place);
        let a = p.area;
        let e = 0.3;
        if p.front[0] < a.x0 - e || p.front[0] > a.x1 + e || p.front[2] < a.z0 - e || p.front[2] > a.z1 + e {
            out.push(format!("{}.front {} is outside its area (x {}‥{}, z {}‥{})", w, at(p.front), js::num_str(a.x0), js::num_str(a.x1), js::num_str(a.z0), js::num_str(a.z1)));
        }
        let h = holder_stand(g, p.front, Some(crate::quiet::HolderStandArea { x0: a.x0, x1: a.x1, z0: a.z0, z1: a.z1 }));
        match h.pos {
            None => out.push(format!("{}: no free floor within 1.2 m of the holder's face {} — nobody can reach it", w, at(p.front))),
            Some(s) if !h.clear => out.push(format!("{}: every free cell by the holder's face {} has a solid between it and the face (nearest {}) — the face is on the wrong side of the furniture, or inside it", w, at(p.front), at(s))),
            Some(s) if !h.inside => out.push(format!("{}: the floor before the holder {} is outside the room's area — the thing would be taken without the room being reached", w, at(s))),
            Some(_) => {}
        }
        fronts.push(h.pos);
    }
    fronts
}

/// Each exit has floor either side of it and its door by it. The floor a stride past each, if it is free.
fn check_exits(g: &mut Game, q: &crate::scene_scenarios::QuietData, out: &mut Vec<String>) -> Vec<Option<V3>> {
    let mut outs: Vec<Option<V3>> = Vec::new();
    for (i, x) in q.exits.iter().enumerate() {
        let w = format!("exits[{}] '{}'", i, x.name);
        let p = exit_point(x, Some(0.6));
        if !free(g, out, &format!("{}: the floor a stride past it", w), p) { outs.push(None); continue; }
        let inn: V3 = [x.x - x.out[0] * 0.6, 0.0, x.z - x.out[1] * 0.6];
        if crate::scene_collision::nav_grid_is_blocked(g, inn[0], inn[2]) {
            out.push(format!("{}: the floor a stride short of it {} is not free — it cannot be walked out through", w, at(inn)));
        }
        if let Some(d) = crate::doors::doors_by_name(g, &x.door) {
            let c = g.dr[d].frame_centre;
            let far = m::hyp2(c[0] - x.x, c[2] - x.z);
            if far > 4.0 { out.push(format!("{}: its door '{}' is {} m from its point — the marker would point at the wrong door", w, x.door, js::to_fixed(far, 1))); }
        }
        outs.push(Some(p));
    }
    outs
}

/// From every start to every holder, and from every holder to every exit, there is a way on foot.
fn check_ways_on_foot(g: &Game, q: &crate::scene_scenarios::QuietData, fronts: &[Option<V3>], outs: &[Option<V3>], out: &mut Vec<String>) {
    for (i, s) in q.starts.iter().enumerate() {
        for (j, f) in fronts.iter().enumerate() {
            let Some(f) = *f else { continue };
            if !crate::scene_collision::nav_grid_is_blocked(g, s.sam[0], s.sam[2]) && !way(g, s.sam, f) {
                out.push(format!("no way on foot from starts[{}] {} to spots[{}] '{}' {}", i, at(s.sam), j, q.spots[j].place, at(f)));
            }
        }
    }
    for (j, f) in fronts.iter().enumerate() {
        for (k, o) in outs.iter().enumerate() {
            if let (Some(f), Some(o)) = (*f, *o) {
                if !way(g, f, o) { out.push(format!("no way on foot from spots[{}] '{}' to exits[{}] '{}' {}", j, q.spots[j].place, k, q.exits[k].name, at(o))); }
            }
        }
    }
}

/// No start in the first sight of a man on his route's first waypoint: the eye's reach and a clear line, whichever way he faces.
fn check_first_sights(g: &Game, q: &crate::scene_scenarios::QuietData, out: &mut Vec<String>) {
    let sets: Vec<Vec<String>> = match q.patrols.as_ref() { Some(p) => p.clone(), None => vec![g.level.routes.iter().map(|&r| g.tab.route[r].name.clone()).collect()] };
    let mut seen: JsSet<String> = JsSet::new();
    for name in sets.iter().flatten() {
        let Some(r) = route_named(g, name) else { continue };
        let Some(first) = g.tab.route[r].points.first().copied() else { continue };
        let eye: V3 = [first[0], 1.6, first[2]];
        for (i, s) in q.starts.iter().enumerate() {
            for (who, p) in [("sam", Some(s.sam)), ("mate", s.mate)] {
                let Some(p) = p else { continue };
                let k = format!("{}:{}:{}", name, i, who);
                if seen.has(k.clone()) { continue; }
                seen.add(k);
                let d = v3::dist_xz(eye, p);
                if d < FIRST_SIGHT_M && !crate::scene_collision::segment_blocked(g, eye, [p[0], 1.0, p[2]], None, None, None) {
                    out.push(format!("starts[{}].{} {} is in plain view of '{}' on his first waypoint {}, {} m off — the run would begin seen", i, who, at(p), name, at(first), js::to_fixed(d, 1)));
                }
            }
        }
    }
}

/// The places of a guard's needs are on free floor.
fn check_needs(g: &mut Game, q: &crate::scene_scenarios::QuietData, out: &mut Vec<String>) {
    let Some(n) = q.needs.as_ref() else { return };
    for (k, s) in [("coffee", &n.coffee), ("smoke", &n.smoke), ("water", &n.water), ("stretch", &n.stretch)] {
        let Some(s) = s.as_ref() else { continue };
        free(g, out, &format!("needs.{} '{}'", k, s.name), s.pos);
        if let Some(l) = s.loop_.as_ref() {
            for (i, &p) in l.iter().enumerate() { free(g, out, &format!("needs.{}.loop[{}]", k, i), p); }
        }
    }
}

/// The civilians' phones and ways out are on free floor, and civilians and those places come together.
fn check_civilians_places(g: &mut Game, q: &crate::scene_scenarios::QuietData, out: &mut Vec<String>) {
    let phones = q.phones.as_deref().unwrap_or(&[]);
    let wayouts = q.wayouts.as_deref().unwrap_or(&[]);
    for (i, s) in phones.iter().enumerate() { free(g, out, &format!("phones[{}] '{}'", i, s.name), s.pos); }
    for (i, s) in wayouts.iter().enumerate() { free(g, out, &format!("wayouts[{}] '{}'", i, s.name), s.pos); }
    let civs = g.level.civilians.as_ref().map_or(0, |c| c.len());
    if (!phones.is_empty() || !wayouts.is_empty()) && civs == 0 { out.push(String::from("phones / wayouts are given but the level has no civilians: nobody will use them")); }
    if civs > 0 && wayouts.is_empty() && phones.is_empty() && g.level.routes.is_empty() { out.push(String::from("the level has civilians and gives them no phone, no way out and no guard to run to")); }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scene_scenarios::QuietExit;

    fn exit() -> QuietExit {
        QuietExit { x: 1.5, z: -2.25, out: [0.6, 0.8], ..QuietExit::default() }
    }

    /// a cut must be seen by Seat.takeCut, which watches the serial, and leaves the two on one point
    #[test]
    fn a_cut_counts_as_a_whole_write_of_desired() {
        let mut lens = crate::camera::FollowCamera::blank();
        let before = lens.desired_serial;
        cut_lens(&mut lens, [1.0, 2.0, 3.0]);
        assert_eq!(lens.desired_serial, before.wrapping_add(1));
        assert_eq!(lens.target, [1.0, 2.0, 3.0]);
        assert_eq!(lens.desired, [1.0, 2.0, 3.0]);
    }

    /// Where a man is sent to leave by: a stride past the exit, along its own way out.
    #[test]
    fn an_exit_point_is_a_stride_past_along_its_own_way() {
        let p = exit_point(&exit(), None);
        assert!((p[0] - (1.5 + 0.6 * 0.9)).abs() < 1e-12 && (p[2] - (-2.25 + 0.8 * 0.9)).abs() < 1e-12, "{p:?}");
        assert!(out_through(&exit(), p), "and that place is out");
    }

    #[test]
    fn out_through_is_a_strip_past_the_point() {
        let x = exit();
        assert!(out_through(&x, exit_point(&x, Some(1.0))));
        assert!(!out_through(&x, exit_point(&x, Some(-1.0))));   // short of it
        assert!(!out_through(&x, exit_point(&x, Some(13.0))));   // past the strip
        let aside = [x.x + 0.8 * 2.0 + 0.6, 0.0, x.z - 0.6 * 2.0 + 0.8];   // along 1, side 2: outside the default half width
        assert!(!out_through(&x, aside));
        let wide = QuietExit { half_w: Some(3.0), ..exit() };
        assert!(out_through(&wide, aside));
    }

    /// A FAILURE MEANS: a link with a pin does not put a player back on the deal it names: the same pin deals the same, on the lists' own rows, and the patrols' length moves neither of the other two picks.
    #[test]
    fn a_pinned_deal_is_the_same_each_time_and_stays_on_its_lists() {
        let deal = |pin: f64, s, p, q| { let d = pinned_deal(crate::dice::Die(pin_seed(pin) as u32), s, p, q); (d.start, d.spot, d.patrol) };
        let pins = [7.0, 0.0, -5.0, 2147483648.5, 11.0, 23.0];
        for pin in pins {
            let (a, b, c) = deal(pin, 5, 4, Some(3));
            assert_eq!(deal(pin, 5, 4, Some(3)), (a, b, c), "pin {pin}");
            assert!((0.0..5.0).contains(&a) && (0.0..4.0).contains(&b) && (0.0..3.0).contains(&c), "pin {pin}");
            assert_eq!(deal(pin, 5, 4, None), (a, b, -1.0), "no patrols to pick from, pin {pin}");
            assert_eq!(deal(pin, 5, 4, Some(0)), (a, b, -1.0), "an empty list of them, pin {pin}");
            assert_eq!(deal(pin, 1, 1, Some(1)), (0.0, 0.0, 0.0), "one of each, pin {pin}");
        }
        let first = deal(pins[0], 5, 4, Some(3));
        assert!(pins.iter().any(|p| deal(*p, 5, 4, Some(3)) != first), "another pin deals another");
    }

    #[test]
    fn a_switch_moved_by_hand_stays_where_it_was_put() {
        let of = |v: bool| crate::quiet::Held { pairup: v, pair: v, sneak: v, spotted: v, silent: v, silent_start: v };
        let was = of(false);
        let ran = of(true);
        let moved = crate::quiet::Held { pairup: false, ..ran };   // moved off during the run; was off anyway
        let out = released(was, ran, moved);
        assert!(!out.pair && !out.pairup);
        let was_on = crate::quiet::Held { pairup: true, ..was };
        let out = released(was_on, ran, moved);
        assert!(!out.pairup, "the hand's word stands over what stood before the run");
        assert!(!out.silent);
    }
}
