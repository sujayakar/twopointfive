pub use ::game_types::quiet::types;
pub use types::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{format, string::String, vec, vec::Vec};

/// the switches as they stand now
fn held_now(g: &mut crate::game::Game) -> crate::quiet::Held {
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

/// a new mission, inert. The constructor's `current()` only ever handed back another Game's switches: the switches are this Game's here
pub fn quiet_mission_new(g: &crate::game::Game) -> crate::quiet::QuietMission {
    crate::quiet::QuietMission::blank()
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

/// why it cannot start right now
fn quiet_mission_can_start(g: &crate::game::Game, squad: Option<f64>) -> Option<String> {
    let squad = squad.unwrap_or(0.0);
    let q = match g.level.scenarios.as_ref().and_then(|s| s.quiet.as_ref()) {
        Some(q) => q,
        None => return Some(String::from("this level carries no quiet mission (its scenarios.quiet block)")),
    };
    if q.starts.is_empty() || q.spots.is_empty() || q.exits.is_empty() { return Some(String::from("its quiet mission wants at least one start, one spot and one exit")); }
    if !g.ai_enabled { return Some(String::from("the AI is switched off (sandbox: no AI) — nobody would walk his route")); }
    if js::truthy(squad) && !g.tune.teams { return Some(String::from("sides are switched off (tune.teams): no partner")); }
    if let Some(r) = g.director.run {
        if !g.tab.director_run[r].ended { return Some(format!("the level script's program '{}' is running", g.tab.director_run[r].name)); }
    }
    if g.puppet.is_some() { return Some(String::from("nobody is playing")); }
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
    g.quiet.staged = 0.0;
    g.quiet.log.clear();
    g.quiet.authored = Some(crate::quiet::QuietMissionAuthored { spawn: g.level.player_spawn, errand: g.level.errand.clone() });
    quiet_mission_current(g);
    if js::truthy(squad) {
        g.quiet.was = Some(held_now(g));
        g.m.pairup.pair_held.row = Some(g.k.pairup.on);
        g.m.quiet.holder = true;
        g.k.pairup.on = true;
        crate::pairup::pair_up_apply(g, None);
        g.k.silent.on = true;
        g.k.silent.start = true;
        g.quiet.mine = Some(held_now(g));
        crate::teams::staff_vs_operatives(g, true);
    }
    g.staged.begin(crate::scene_scenarios::MissionKind::Quiet, crate::mission::Facts { unseen: true, ..Default::default() });
    g.stats.quiet.starts += 1;
    crate::game::game_restart_encounter(g);   // → beforeRestart deals and writes the level; onEncounterRestart stages
    if crate::quiet::quiet_of(g) { None } else { Some(g.quiet.failure.clone().unwrap_or_else(|| String::from("the staging failed"))) }
}

/// Stop it: the partner gone, every route manned, the authored mission and spawn back, a fresh night encounter
fn quiet_mission_stop(g: &mut crate::game::Game) {
    if !crate::quiet::quiet_of(g) { return; }
    g.staged.down();
    quiet_mission_put_back(g);
    quiet_mission_release(g);
    g.staged.team = Vec::new();
    g.quiet.data = None;
    g.quiet.pending = None;
    crate::game::game_restart_encounter(g);
}

/// the level as authored
fn quiet_mission_put_back(g: &mut crate::game::Game) {
    quiet_mission_unbench(g);
    if let Some(a) = g.quiet.authored.take() {
        g.level.player_spawn = a.spawn;
        g.level.errand = a.errand;
    }
    quiet_mission_rack(g);
}

/// THE RUN IS OVER: the switches it took put back, each only while it still stands as the run wrote it
fn quiet_mission_release(g: &mut crate::game::Game) {
    if let (Some(w), Some(mine)) = (g.quiet.was, g.quiet.mine) {
        let n = held_now(g);
        held_set(g, released(w, mine, n));
        crate::teams::staff_vs_operatives(g, false);
        g.m.pairup.pair_held.row = None;
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
            g.m.pairup.pair_held.row = Some(g.k.pairup.on);
            held_set(g, mine);
            crate::teams::staff_vs_operatives(g, true);
            g.m.quiet.holder = true;
        }
    }
}

/// restartEncounter's FIRST act: the deal, written into the level before the fresh Player and Guards read it
pub fn quiet_mission_before_restart(g: &mut crate::game::Game) {
    if !crate::quiet::quiet_of(g) { return; }
    let q = match g.quiet.data.clone() { Some(q) => q, None => return };
    quiet_mission_unbench(g);
    g.quiet.deal = match g.quiet.pending { Some(d) => d, None => quiet_mission_deal_one(g, &q) };
    g.quiet.pending = None;
    g.stats.quiet.deals += 1;
    let s = &q.starts[g.quiet.deal.start as usize];
    let p = &q.spots[g.quiet.deal.spot as usize];
    g.level.player_spawn = [s.sam[0], 0.0, s.sam[2]];
    g.level.errand = crate::mission::Errand {
        place: p.place.clone(), area: p.area, door: p.door.clone(),
        thing: q.thing.clone().unwrap_or_else(|| String::from("the drive")), holder: p.holder.clone(), index: p.rack_index, front: p.front, half_w: p.half_w, told: p.wired.unwrap_or(false),
        exits: q.exits.clone(),
    };
    quiet_mission_rack(g);
    // the routes this run does not man: off the guard list, their torches out of the light set
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

/// restartEncounter's last act: Sam faced as the start has him, the partner staged. A staging that fails ends the mission cleanly
pub fn quiet_mission_on_encounter_restart(g: &mut crate::game::Game) {
    if !crate::quiet::quiet_of(g) { return; }
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
    let at = g.ch[g.pl[g.player].ch].pos;
    let target = crate::orders::squad_orders_target_at(g, at);
    let why = crate::orders::squad_orders_give(g, crate::orders::rows::fallin::ORDER.id, Some(&target), Some(crate::orders::Who::All), None);
    match why {
        Some(w) => quiet_mission_note(g, &format!("fall in refused: {}", w)),
        None => quiet_mission_note(g, "partner following"),
    }
}

/// `cam.target = v3.copy(p); cam.desired = v3.copy(cam.target)`: a whole write of `desired` is what Seat.takeCut watches for, here by its serial
fn cut_lens(lens: &mut crate::camera::FollowCamera, p: V3) {
    lens.target = p;
    lens.desired = p;
    lens.desired_serial = lens.desired_serial.wrapping_add(1);
}

/// the run in a line
fn quiet_mission_summary(g: &crate::game::Game) -> String {
    if !crate::quiet::quiet_of(g) || g.quiet.data.is_none() {
        let carries = g.level.scenarios.as_ref().map_or(false, |s| s.quiet.is_some());
        return String::from(if carries { "off · this level carries one" } else { "off · this level carries none" });
    }
    let s = quiet_mission_start_(g).unwrap();
    let e = &g.level.errand;
    let patrol = quiet_mission_patrol(g).join(", ");
    let q = g.quiet.data.as_ref().unwrap();
    let d = g.quiet.deal;
    let pinned = match g.quiet.pin { None => String::from("dealt"), Some(n) => format!("pinned {}", js::num_str(n)) };
    let name = match s.name.as_deref() { Some(n) if !n.is_empty() => format!(" ({})", n), _ => String::new() };
    let sets = match q.patrols.as_ref() { Some(l) => format!("{}/{}", js::num_str(d.patrol + 1.0), l.len()), None => String::from("all") };
    let who = if patrol.is_empty() { "nobody" } else { patrol.as_str() };
    let plural = if q.exits.len() == 1 { "" } else { "s" };
    let partner = if js::truthy(g.quiet.squad) {
        let down = g.staged.team.first().map_or(false, |&o| crate::condition::down(g, g.gd[o].ch));
        format!("partner {}", if down { "down" } else { "up" })
    } else {
        String::from("alone")
    };
    format!(
        "on · {} · start {}/{}{} · {} in {}, {} ({}/{}) · patrol {}: {} · {} way{} out · {}",
        pinned, js::num_str(d.start + 1.0), q.starts.len(), name, e.thing, e.holder, e.place, js::num_str(d.spot + 1.0), q.spots.len(), sets, who, q.exits.len(), plural, partner
    )
}

/// a line of the mission's own log
fn quiet_mission_note(g: &mut crate::game::Game, s: &str) {
    let line = format!("{} {}", js::to_fixed(g.time, 2), s);
    g.quiet.log.push(line);
    if g.quiet.log.len() > 60 { g.quiet.log.drain(0..30); }
}

/// one deal: the pinned seed's, and with no pin the first of each list
fn quiet_mission_deal_one(g: &crate::game::Game, q: &crate::scene_scenarios::QuietData) -> crate::quiet::QuietDeal {
    let sets = q.patrols.as_ref().map(|p| p.len());
    match g.quiet.pin {
        None => crate::quiet::QuietDeal { start: 0.0, spot: 0.0, patrol: if sets.map_or(false, |n| n > 0) { 0.0 } else { -1.0 } },
        Some(pin) => pinned_deal(crate::math_dice::mulberry32(pin_seed(pin) as f64), q.starts.len(), q.spots.len(), sets),
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
    g.rack_targets = g.targets.iter().copied().filter(|&t| { let name = &g.tab.sim_lightshome_fixture[t].name; *name == leds || *name == glow }).collect();
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
            team: crate::teams::Team::Operative,
            pos: at,
            yaw: Some(s.sam_yaw),
            name: Some(String::from("Blue One")),
            light: Some(false),
            pair: Some(crate::game::PairId::Blue),
            ..crate::guards::UnitOpts::default()
        })?;
        crate::mind::set_state(g, man, crate::game::GuardState::Patrol, "quiet");
        crate::mind::set(g, man, 0.0, "quiet");
        crate::control::set_hold(g, man, false, "quiet");
        crate::control::set_pinned(g, man, false, "quiet");
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
        g.stats.quiet.mates += 1;
        let ops = g.staged.team.clone();
        crate::kit::kit_stage(g, &ops);
    }
    g.quiet.staged += 1.0;
    g.quiet.cut = true;
    let line = format!("staged: {}", quiet_mission_summary(g));
    quiet_mission_note(g, &line);
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
pub fn holder_stand(g: &mut crate::game::Game, front: V3, area: Option<crate::quiet::HolderStandArea>) -> crate::quiet::HolderStand {
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
fn way(g: &mut crate::game::Game, a: V3, b: V3) -> bool {
    crate::scene_collision::nav_grid_find_path(g, a, b, Some(20000.0), None).map_or(false, |path| !path.is_empty())
}

/// a route by its name; the last of a name, as a Map built from the list has it
fn route_named(g: &crate::game::Game, name: &str) -> Option<RouteId> {
    g.level.routes.iter().rev().copied().find(|&r| g.tab.route[r].name == name)
}

/// THE BLOCK AGAINST THE BUILT FLOOR: what the level file's validator cannot say. `g` is a Game stood up on the level, nothing started; the faults, each a line an author can act on
pub fn check_quiet(g: &mut crate::game::Game) -> Vec<String> {
    let q = match g.level.scenarios.as_ref().and_then(|s| s.quiet.clone()) { Some(q) => q, None => return vec![String::from("the level carries no scenarios.quiet block")] };
    let mut out: Vec<String> = Vec::new();
    for (i, s) in q.starts.iter().enumerate() {
        let w = match s.name.as_deref() { Some(n) if !n.is_empty() => format!("starts[{}] '{}'", i, n), _ => format!("starts[{}]", i) };
        free(g, &mut out, &format!("{}.sam", w), s.sam);
        if let Some(mate) = s.mate {
            if free(g, &mut out, &format!("{}.mate", w), mate) && !crate::scene_collision::nav_grid_walkable(g, s.sam, mate) && !way(g, s.sam, mate) {
                out.push(format!("{}.mate {} cannot be walked to from sam {}", w, at(mate), at(s.sam)));
            }
            if v3::dist_xz(s.sam, mate) < 0.7 {
                out.push(format!("{}.mate stands on top of sam ({} m): give them a body's room", w, js::to_fixed(v3::dist_xz(s.sam, mate), 2)));
            }
        }
    }
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
    let mut outs: Vec<Option<V3>> = Vec::new();
    for (i, x) in q.exits.iter().enumerate() {
        let w = format!("exits[{}] '{}'", i, x.name);
        let p = exit_point(x, Some(0.6));
        if !free(g, &mut out, &format!("{}: the floor a stride past it", w), p) { outs.push(None); continue; }
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
    for (i, s) in q.starts.iter().enumerate() {
        for (j, f) in fronts.iter().enumerate() {
            let f = match *f { Some(f) => f, None => continue };
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
    // no start in the first sight of a man on his route's first waypoint: the eye's reach and a clear line, whichever way he faces
    let sets: Vec<Vec<String>> = match q.patrols.as_ref() { Some(p) => p.clone(), None => vec![g.level.routes.iter().map(|&r| g.tab.route[r].name.clone()).collect()] };
    let mut seen: JsSet<String> = JsSet::new();
    for set in &sets {
        for name in set {
            let r = match route_named(g, name) { Some(r) => r, None => continue };
            let first = match g.tab.route[r].points.first().copied() { Some(p) => p, None => continue };
            let eye: V3 = [first[0], 1.6, first[2]];
            for (i, s) in q.starts.iter().enumerate() {
                for (who, p) in [("sam", Some(s.sam)), ("mate", s.mate)] {
                    let p = match p { Some(p) => p, None => continue };
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
    if let Some(n) = q.needs.as_ref() {
        for (k, s) in [("coffee", &n.coffee), ("smoke", &n.smoke), ("water", &n.water), ("stretch", &n.stretch)] {
            if let Some(s) = s.as_ref() {
                free(g, &mut out, &format!("needs.{} '{}'", k, s.name), s.pos);
                if let Some(l) = s.loop_.as_ref() {
                    for (i, &p) in l.iter().enumerate() { free(g, &mut out, &format!("needs.{}.loop[{}]", k, i), p); }
                }
            }
        }
    }
    let empty: Vec<crate::scene_scenarios::QuietStand> = Vec::new();
    let phones = q.phones.as_ref().unwrap_or(&empty);
    let wayouts = q.wayouts.as_ref().unwrap_or(&empty);
    for (i, s) in phones.iter().enumerate() { free(g, &mut out, &format!("phones[{}] '{}'", i, s.name), s.pos); }
    for (i, s) in wayouts.iter().enumerate() { free(g, &mut out, &format!("wayouts[{}] '{}'", i, s.name), s.pos); }
    let civs = g.level.civilians.as_ref().map_or(0, |c| c.len());
    if (!phones.is_empty() || !wayouts.is_empty()) && civs == 0 { out.push(String::from("phones / wayouts are given but the level has no civilians: nobody will use them")); }
    if civs > 0 && wayouts.is_empty() && phones.is_empty() && g.level.routes.is_empty() { out.push(String::from("the level has civilians and gives them no phone, no way out and no guard to run to")); }
    out
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

    #[test]
    fn exit_point_has_the_typescripts_bits() {
        let p = exit_point(&exit(), None);
        assert_eq!(p[0].to_bits(), 0x400051eb851eb852);
        assert_eq!(p[2].to_bits(), 0xbff87ae147ae147a);
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

    /// the picks are bun's, off the TypeScript's own mulberry32
    #[test]
    fn a_pinned_deal_deals_what_the_typescript_deals() {
        for (pin, s, p, q, a, b, c) in [
            (7.0, 3, 2, Some(4), 0.0, 0.0, 1.0),
            (0.0, 1, 1, None, 0.0, 0.0, -1.0),
            (-5.0, 5, 4, Some(2), 4.0, 0.0, 1.0),
            (2147483648.5, 2, 3, Some(0), 1.0, 1.0, -1.0),
        ] {
            // mulberry32's stream is `Die`'s, draw for draw; `mulberry32` itself is a stub in this crate
            let d = pinned_deal(crate::dice::Die::new(pin_seed(pin) as u32), s, p, q);
            assert_eq!((d.start, d.spot, d.patrol), (a, b, c), "pin {}", pin);
        }
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
