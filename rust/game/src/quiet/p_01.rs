//! src/game/quiet.ts lines 1 to 363
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// the switches as they stand now   (quiet.ts heldNow)
pub fn held_now(g: &mut crate::game::Game) -> crate::quiet::Held {
    crate::quiet::Held { pairup: g.k.pairup.on, pair: g.k.pair.on, sneak: g.k.sneak.on, spotted: g.k.spotted.on, silent: g.k.silent.on, silent_start: g.k.silent.start }
}

/// the switches written   (quiet.ts heldSet)
pub fn held_set(g: &mut crate::game::Game, h: crate::quiet::Held) {
    g.k.pairup.on = h.pairup;
    g.k.pair.on = h.pair;
    g.k.sneak.on = h.sneak;
    g.k.spotted.on = h.spotted;
    g.k.silent.on = h.silent;
    g.k.silent.start = h.silent_start;
}

/// out through this exit: past its point going its own way, inside its width   (quiet.ts outThrough)
pub fn out_through(x: &crate::scene_scenarios::QuietExit, p: V3) -> bool {
    let dx = p[0] - x.x;
    let dz = p[2] - x.z;
    let along = dx * x.out[0] + dz * x.out[1];
    let side = m::abs(dx * x.out[1] - dz * x.out[0]);
    along > 0.0 && along < EXIT_DEPTH && side < x.half_w.unwrap_or(EXIT_HALF_W)
}

/// a point a stride past the exit, on its own line   (quiet.ts exitPoint)
pub fn exit_point(x: &crate::scene_scenarios::QuietExit, past: Option<f64>) -> V3 {
    let past = past.unwrap_or(0.9);
    [x.x + x.out[0] * past, 0.0, x.z + x.out[1] * past]
}

/// a new mission, inert. The constructor's `current()` only ever handed back another Game's switches: the switches are this Game's here   (quiet.ts QuietMission)
pub fn quiet_mission_new(g: &mut crate::game::Game) -> crate::quiet::QuietMission {
    crate::quiet::QuietMission::blank()
}

/// the block in force   (quiet.ts QuietMission.block)
pub fn quiet_mission_block(g: &mut crate::game::Game) -> Option<crate::scene_scenarios::QuietData> {
    if g.quiet.active { g.quiet.data.clone() } else { None }
}

/// the start dealt   (quiet.ts QuietMission.start_)
pub fn quiet_mission_start_(g: &mut crate::game::Game) -> Option<crate::scene_scenarios::QuietStart> {
    if !g.quiet.active { return None; }
    let at = g.quiet.deal.start as usize;
    g.quiet.data.as_ref().and_then(|q| q.starts.get(at).cloned())
}

/// the spot dealt   (quiet.ts QuietMission.spot)
pub fn quiet_mission_spot(g: &mut crate::game::Game) -> Option<crate::scene_scenarios::QuietSpot> {
    if !g.quiet.active { return None; }
    let at = g.quiet.deal.spot as usize;
    g.quiet.data.as_ref().and_then(|q| q.spots.get(at).cloned())
}

/// the ways out   (quiet.ts QuietMission.exits)
pub fn quiet_mission_exits(g: &mut crate::game::Game) -> Vec<crate::scene_scenarios::QuietExit> {
    if !g.quiet.active { return Vec::new(); }
    match g.quiet.data.as_ref() { Some(q) => q.exits.clone(), None => Vec::new() }
}

/// what is to be taken   (quiet.ts QuietMission.thing)
pub fn quiet_mission_thing(g: &mut crate::game::Game) -> String {
    match g.quiet.data.as_ref().and_then(|q| q.thing.as_ref()) { Some(t) => t.clone(), None => String::from("the drive") }
}

/// the routes manned on this run, by name   (quiet.ts QuietMission.patrol)
pub fn quiet_mission_patrol(g: &mut crate::game::Game) -> Vec<String> {
    if !g.quiet.active { return Vec::new(); }
    let q = match g.quiet.data.as_ref() { Some(q) => q, None => return Vec::new() };
    let at = g.quiet.deal.patrol;   // (−1: past every list, so every route)
    if at >= 0.0 {
        if let Some(set) = q.patrols.as_ref().and_then(|p| p.get(at as usize)) { return set.clone(); }
    }
    g.level.routes.iter().map(|&r| g.tab.route[r].name.clone()).collect()
}

/// why it cannot start right now   (quiet.ts QuietMission.canStart)
pub fn quiet_mission_can_start(g: &mut crate::game::Game, squad: Option<f64>) -> Option<String> {
    let squad = squad.unwrap_or(0.0);
    let q = match g.level.scenarios.as_ref().and_then(|s| s.quiet.as_ref()) {
        Some(q) => q,
        None => return Some(String::from("this level carries no quiet mission (its scenarios.quiet block)")),
    };
    if q.starts.is_empty() || q.spots.is_empty() || q.exits.is_empty() { return Some(String::from("its quiet mission wants at least one start, one spot and one exit")); }
    if !g.ai_enabled { return Some(String::from("the AI is switched off (sandbox: no AI) — nobody would walk his route")); }
    if js::truthy(squad) && !g.tune.teams { return Some(String::from("sides are switched off (tune.teams): no partner")); }
    if let Some(r) = g.director.run.as_ref() {
        if !r.ended { return Some(format!("the level script's program '{}' is running", r.name)); }
    }
    if g.touring || g.quiet_utility || g.puppet.is_some() { return Some(String::from("a show is running")); }
    if g.night_breach.active || g.rescue.active { return Some(String::from("another scenario is staged")); }
    if g.guards.iter().any(|&x| g.gd[x].unit.is_some()) { return Some(String::from("a spawned unit is already on the floor")); }
    None
}

/// Start it: the deal, a fresh encounter on it, the partner beside Sam. The reason when refused or when the staging failed   (quiet.ts QuietMission.start)
pub fn quiet_mission_start(g: &mut crate::game::Game, pin: Option<f64>, squad: Option<f64>) -> Option<String> {
    let squad = squad.unwrap_or(0.0);
    if g.quiet.active { return None; }
    if let Some(why) = quiet_mission_can_start(g, Some(squad)) { return Some(why); }
    g.quiet.data = g.level.scenarios.as_ref().and_then(|s| s.quiet.clone());
    g.quiet.pin = pin;
    g.quiet.squad = squad;
    g.quiet.failure = None;
    g.quiet.staged = 0.0;
    g.quiet.log.clear();
    g.quiet.authored = Some(crate::quiet::QuietMissionAuthored { spawn: g.level.player_spawn, mission: g.level.mission.clone() });
    quiet_mission_current(g);
    if js::truthy(squad) {
        g.quiet.was = Some(held_now(g));
        g.m.pairup.pair_held.row = Some(g.k.pairup.on);
        holder_take(g);
        g.k.pairup.on = true;
        crate::pairup::pair_up_apply(g, None);
        g.k.silent.on = true;
        g.k.silent.start = true;
        g.quiet.mine = Some(held_now(g));
        crate::teams::staff_vs_operatives(g, true);
    }
    g.quiet.active = true;
    g.stats.quiet.starts += 1;
    crate::game::game_restart_encounter(g);   // → beforeRestart deals and writes the level; onEncounterRestart stages
    if g.quiet.active { None } else { Some(g.quiet.failure.clone().unwrap_or_else(|| String::from("the staging failed"))) }
}

/// Stop it: the partner gone, every route manned, the authored mission and spawn back, a fresh night encounter   (quiet.ts QuietMission.stop)
pub fn quiet_mission_stop(g: &mut crate::game::Game) {
    if !g.quiet.active { return; }
    g.quiet.active = false;
    quiet_mission_put_back(g);
    quiet_mission_release(g);
    g.quiet.ops = Vec::new();
    g.quiet.data = None;
    g.quiet.pending = None;
    crate::game::game_restart_encounter(g);
}

/// the level as authored   (quiet.ts QuietMission.putBack)
pub fn quiet_mission_put_back(g: &mut crate::game::Game) {
    quiet_mission_unbench(g);
    if let Some(a) = g.quiet.authored.take() {
        g.level.player_spawn = a.spawn;
        g.level.mission = a.mission;
    }
    quiet_mission_rack(g);
}

/// THE RUN IS OVER: the switches it took put back, each only while it still stands as the run wrote it   (quiet.ts QuietMission.release)
pub fn quiet_mission_release(g: &mut crate::game::Game) {
    if let (Some(w), Some(mine)) = (g.quiet.was, g.quiet.mine) {
        let n = held_now(g);
        held_set(g, released(w, mine, n));
        crate::teams::staff_vs_operatives(g, false);
        g.m.pairup.pair_held.row = None;
    }
    g.quiet.was = None;
    g.quiet.mine = None;
    g.m.quiet.holder = None;
}

/// each switch: what stood before the run where it still stands as the run wrote it, else the hand that moved it   (quiet.ts QuietMission.release)
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

/// `holder = this`. The field's type keeps a copy (see notes): only whether it is set is ever read   (quiet.ts holder)
fn holder_take(g: &mut crate::game::Game) {
    g.m.quiet.holder = Some(g.quiet.clone());
}

/// THIS GAME IS THE ONE RUNNING: its own run, if up, takes the switches again. No other Game's run can hold them here: they are this Game's   (quiet.ts QuietMission.current)
pub fn quiet_mission_current(g: &mut crate::game::Game) {
    if g.m.quiet.holder.is_some() { return; }
    if g.quiet.active && js::truthy(g.quiet.squad) {
        if let Some(mine) = g.quiet.mine {
            g.quiet.was = Some(held_now(g));
            g.m.pairup.pair_held.row = Some(g.k.pairup.on);
            held_set(g, mine);
            crate::teams::staff_vs_operatives(g, true);
            holder_take(g);
        }
    }
}

/// deal afresh on the same run   (quiet.ts QuietMission.restage)
pub fn quiet_mission_restage(g: &mut crate::game::Game) {
    if g.quiet.active { crate::game::game_restart_encounter(g); }
}

/// restartEncounter's FIRST act: the deal, written into the level before the fresh Player and Guards read it   (quiet.ts QuietMission.beforeRestart)
pub fn quiet_mission_before_restart(g: &mut crate::game::Game) {
    if !g.quiet.active { return; }
    let q = match g.quiet.data.clone() { Some(q) => q, None => return };
    quiet_mission_unbench(g);
    g.quiet.deal = match g.quiet.pending { Some(d) => d, None => quiet_mission_deal_one(g, &q) };
    g.quiet.pending = None;
    g.stats.quiet.deals += 1;
    let s = &q.starts[g.quiet.deal.start as usize];
    let p = &q.spots[g.quiet.deal.spot as usize];
    let x = &q.exits[0];
    g.level.player_spawn = [s.sam[0], 0.0, s.sam[2]];
    g.level.mission = crate::quiet::QuietMissionAuthoredMission {
        server_room: crate::quiet::QuietMissionAuthoredMissionServerRoom { x0: p.area.x0, x1: p.area.x1, z0: p.area.z0, z1: p.area.z1 },
        entry_door: p.door.clone(),
        rack: crate::quiet::QuietMissionAuthoredMissionRack { index: p.rack_index, front: p.front, half_w: p.half_w },
        exfil_door: x.door.clone(),
        exfil_x: x.x,
        exfil_z: x.z,
        exfil_out: Some([x.out[0], x.out[1]]),
    };
    quiet_mission_rack(g);
    // the routes this run does not man: off the guard list, their torches out of the light set
    let manned = quiet_mission_patrol(g);
    let off: Vec<GuardId> = g.guards.iter().copied().filter(|&x| g.gd[x].unit.is_none() && !manned.contains(&g.tab.route[g.gd[x].route].name)).collect();
    for &gd in &off {
        let l = g.gd[gd].flashlight;
        seam_light_off(g, l);
        crate::render_lights::light_set_remove(&mut g.engine.lights, l);
    }
    if !off.is_empty() { g.guards.retain(|x| !off.contains(x)); }
    g.quiet.benched = off;
}

/// `gd.flashlight.enabled = false` (quiet.ts:185)
fn seam_light_off(g: &mut crate::game::Game, l: LightId) {
    todo!("needs: a light's fields behind LightId (RtLight.enabled): LightId has no table")
}

/// restartEncounter's last act: Sam faced as the start has him, the partner staged. A staging that fails ends the mission cleanly   (quiet.ts QuietMission.onEncounterRestart)
pub fn quiet_mission_on_encounter_restart(g: &mut crate::game::Game) {
    if !g.quiet.active { return; }
    if let Err(e) = stage_or_why(g) {
        let failure = format!("the staging failed: {}", e);
        g.quiet.failure = Some(failure.clone());
        crate::stickycam::sticky_visit_set(g, 0.0);
        quiet_mission_stop(g);
        crate::game::game_msg(g, &format!("quiet mission — {}", failure));
    }
}

/// once a frame: the camera put on a dealt start, the partner told to follow the first tick the orders have him   (quiet.ts QuietMission.tick)
pub fn quiet_mission_tick(g: &mut crate::game::Game) {
    if !g.quiet.active { return; }
    if js::truthy(g.quiet.squad) {
        if let Some(o) = g.quiet.ops.first().copied() {
            if !g.guards.contains(&o) {   // despawned by something short of a restart: a full restart, which stages
                crate::game::game_restart_encounter(g);
                if !g.quiet.active { return; }
            }
        }
    }
    if g.quiet.cut {   // (`cam` is never null: the one caller is Game.update, whose cam is required)
        g.quiet.cut = false;
        let target = v3::copy(crate::game::game_follow_pos(g, None));
        let first = g.seats[0];   // `cam = S[0].cam` (game.ts:1728)
        cut_lens(&mut g.tab.cmd_seat[first].cam, target);
    }
    if !g.quiet.fall_in { return; }
    let m = match g.quiet.ops.first().copied() { Some(m) => m, None => return };
    if g.gd[m].state == crate::game::GuardState::Dead || !crate::orders::squad_orders_active(g) { return; }
    g.quiet.fall_in = false;
    let at = g.ch[g.pl[g.player].ch].pos;
    let target = crate::orders::squad_orders_target_at(g, at);
    let why = crate::orders::squad_orders_give(g, "fallIn", Some(&target), Some(crate::orders::SquadOrdersGiveWho::All), None);
    match why {
        Some(w) => quiet_mission_note(g, &format!("fall in refused: {}", w)),
        None => quiet_mission_note(g, "partner following"),
    }
}

/// `cam.target = v3.copy(p); cam.desired = v3.copy(cam.target)`: a whole write of `desired` is what Seat.takeCut watches for, here by its serial   (quiet.ts:201)
fn cut_lens(lens: &mut crate::camera::FollowCamera, p: V3) {
    lens.target = p;
    lens.desired = p;
    lens.desired_serial = lens.desired_serial.wrapping_add(1);
}

/// the deal as a checkpoint files it   (quiet.ts QuietMission.snapshot)
pub fn quiet_mission_snapshot(g: &mut crate::game::Game) -> Option<crate::quiet::QuietSnap> {
    if g.quiet.active { Some(crate::quiet::QuietSnap { deal: g.quiet.deal, pin: g.quiet.pin, squad: g.quiet.squad }) } else { None }
}

/// a checkpoint's deal, for the restart its load is about to make; ignored out of range   (quiet.ts QuietMission.restoreDeal)
pub fn quiet_mission_restore_deal(g: &mut crate::game::Game, s: Option<crate::quiet::QuietSnap>) {
    if !g.quiet.active { return; }
    let (q, s) = match (g.quiet.data.as_ref(), s) { (Some(q), Some(s)) => (q, s), _ => return };
    let d = s.deal;
    let sets = q.patrols.as_ref().map_or(0, |p| p.len());
    let ok = is_integer(d.start) && d.start >= 0.0 && d.start < q.starts.len() as f64
        && is_integer(d.spot) && d.spot >= 0.0 && d.spot < q.spots.len() as f64
        && is_integer(d.patrol) && d.patrol >= -1.0 && d.patrol < sets as f64;
    if ok { g.quiet.pending = Some(crate::quiet::QuietDeal { start: d.start, spot: d.spot, patrol: if sets > 0 { m::max(0.0, d.patrol) } else { -1.0 } }); }
}

/// how many men a deal's patrol set mans on this level (−1: not a deal its block deals)   (quiet.ts QuietMission.rosterOf)
pub fn quiet_mission_roster_of(g: &mut crate::game::Game, d: Option<crate::quiet::QuietDeal>) -> f64 {
    let q = match g.quiet.data.as_ref().or(g.level.scenarios.as_ref().and_then(|s| s.quiet.as_ref())) { Some(q) => q, None => return -1.0 };
    let d = match d { Some(d) => d, None => return -1.0 };
    if !(is_integer(d.start) && d.start >= 0.0 && d.start < q.starts.len() as f64 && is_integer(d.spot) && d.spot >= 0.0 && d.spot < q.spots.len() as f64) { return -1.0; }
    let sets = match q.patrols.as_ref() { Some(p) if !p.is_empty() => p, _ => return g.level.routes.len() as f64 };
    if !is_integer(d.patrol) || d.patrol < 0.0 { return -1.0; }
    match sets.get(d.patrol as usize) { Some(s) => s.len() as f64, None => -1.0 }
}

/// Number.isInteger
fn is_integer(x: f64) -> bool {
    x.is_finite() && m::floor(x) == x
}

/// a number as a template string writes it (these are indices and authored marks: no exponents to mind)
fn num(x: f64) -> String {
    if x == 0.0 { String::from("0") } else { format!("{}", x) }
}

/// the run in a line   (quiet.ts QuietMission.summary)
pub fn quiet_mission_summary(g: &mut crate::game::Game) -> String {
    if !g.quiet.active || g.quiet.data.is_none() {
        let carries = g.level.scenarios.as_ref().map_or(false, |s| s.quiet.is_some());
        return String::from(if carries { "off · this level carries one" } else { "off · this level carries none" });
    }
    let s = quiet_mission_start_(g).unwrap();
    let p = quiet_mission_spot(g).unwrap();
    let thing = quiet_mission_thing(g);
    let patrol = quiet_mission_patrol(g).join(", ");
    let q = g.quiet.data.as_ref().unwrap();
    let d = g.quiet.deal;
    let pinned = match g.quiet.pin { None => String::from("dealt"), Some(n) => format!("pinned {}", num(n)) };
    let name = match s.name.as_deref() { Some(n) if !n.is_empty() => format!(" ({})", n), _ => String::new() };
    let sets = match q.patrols.as_ref() { Some(l) => format!("{}/{}", num(d.patrol + 1.0), l.len()), None => String::from("all") };
    let who = if patrol.is_empty() { "nobody" } else { patrol.as_str() };
    let plural = if q.exits.len() == 1 { "" } else { "s" };
    let partner = if js::truthy(g.quiet.squad) {
        let down = g.quiet.ops.first().map_or(false, |&o| g.gd[o].state == crate::game::GuardState::Dead);
        format!("partner {}", if down { "down" } else { "up" })
    } else {
        String::from("alone")
    };
    format!(
        "on · {} · start {}/{}{} · {} in {}, {} ({}/{}) · patrol {}: {} · {} way{} out · {}",
        pinned, num(d.start + 1.0), q.starts.len(), name, thing, p.holder, p.place, num(d.spot + 1.0), q.spots.len(), sets, who, q.exits.len(), plural, partner
    )
}

/// a line of the mission's own log   (quiet.ts QuietMission.note)
pub fn quiet_mission_note(g: &mut crate::game::Game, s: &str) {
    let line = format!("{:.2} {}", g.time, s);
    g.quiet.log.push(line);
    if g.quiet.log.len() > 60 { g.quiet.log.drain(0..30); }
}

/// one deal. SAMESTART.on is settled on: a mission with no pin is the first of each list, with the three draws made all the same   (quiet.ts QuietMission.dealOne)
pub fn quiet_mission_deal_one(g: &mut crate::game::Game, q: &crate::scene_scenarios::QuietData) -> crate::quiet::QuietDeal {
    let sets = q.patrols.as_ref().map(|p| p.len());
    match g.quiet.pin {
        None => {
            crate::math_dice::sim_random(g);
            crate::math_dice::sim_random(g);
            crate::math_dice::sim_random(g);
            crate::quiet::QuietDeal { start: 0.0, spot: 0.0, patrol: if sets.map_or(false, |n| n > 0) { 0.0 } else { -1.0 } }
        }
        Some(pin) => pinned_deal(pin, q.starts.len(), q.spots.len(), sets),
    }
}

/// a pinned deal, off a stream of its own: three draws whatever the lists' lengths   (quiet.ts QuietMission.dealOne)
fn pinned_deal(pin: f64, starts: usize, spots: usize, sets: Option<usize>) -> crate::quiet::QuietDeal {
    let mut die = crate::dice::Die::new((js::to_i32(pin) ^ 0x51ed270b) as u32);   // mulberry32's stream: `roll(dieOf(seed))`, draw for draw (dice.ts)
    let mut pick = |n: f64| m::min(n - 1.0, m::floor(die.roll() * n));
    let start = pick(starts as f64);
    let spot = pick(spots as f64);
    let patrol = pick(m::max(1.0, sets.unwrap_or(1) as f64));
    crate::quiet::QuietDeal { start, spot, patrol: if sets.map_or(false, |n| n > 0) { patrol } else { -1.0 } }
}

/// the benched routes' men back on the guard list, in route order, torches back in the light set   (quiet.ts QuietMission.unbench)
pub fn quiet_mission_unbench(g: &mut crate::game::Game) {
    if g.quiet.benched.is_empty() { return; }
    let benched = g.quiet.benched.clone();
    for &gd in &benched {
        let l = g.gd[gd].flashlight;
        if !g.engine.lights.lights.contains(&l) { crate::render_lights::light_set_add(&mut g.engine.lights, l); }
    }
    let mut staff: Vec<(usize, GuardId)> = g.guards.iter().copied().filter(|&x| g.gd[x].unit.is_none()).chain(benched.iter().copied()).map(|x| (g.gd[x].route_i, x)).collect();
    js::sort_by(&mut staff, |a, b| a.0 as f64 - b.0 as f64);
    let mut guards: Vec<GuardId> = staff.iter().map(|s| s.1).collect();
    guards.extend(g.guards.iter().copied().filter(|&x| g.gd[x].unit.is_some()));
    g.guards = guards;
    g.quiet.benched = Vec::new();
}

/// what goes dark when the thing comes out: the switchables named for the mission's holder   (quiet.ts QuietMission.rack)
pub fn quiet_mission_rack(g: &mut crate::game::Game) {
    let n = num(g.level.mission.rack.index);
    let leds = format!("rack{}_leds", n);
    let glow = format!("rack{}_glow", n);
    g.rack_targets = g.targets.iter().copied().filter(|&t| { let name = &g.tab.sim_lightshome_fixture[t].name; *name == leds || *name == glow }).collect();
}

/// the staging, for whoever can take its failure   (quiet.ts QuietMission.stage)
fn stage_or_why(g: &mut crate::game::Game) -> Result<(), String> {
    let s = quiet_mission_start_(g).unwrap();
    let pl = g.pl[g.player].ch;
    g.quiet.ops = Vec::new();
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
            ..crate::guards::UnitOpts::zeroed()
        });
        g.gd[man].state = crate::game::GuardState::Patrol;
        g.gd[man].awareness = 0.0;
        g.gd[man].hold = false;
        g.gd[man].pinned = false;
        let face = g.ch[g.gd[man].ch].body_yaw;
        crate::squad::order(g, man, crate::squad::OrderPatch {
            goal: Some(None),
            face: Some(Some(face)),
            stance: Some(Some(crate::character::Stance::LowReady)),
            upper: Some(Some(crate::character::UpperMode::None)),
            aim_at: Some(None),
            light: Some(Some(false)),
            crouch: Some(Some(false)),
            ..crate::squad::OrderPatch::zeroed()
        });
        g.quiet.ops.push(man);
        g.quiet.fall_in = true;
        g.stats.quiet.mates += 1;
        let ops = g.quiet.ops.clone();
        crate::kit::kit_stage(g, &ops);
    }
    g.quiet.staged += 1.0;
    g.quiet.cut = true;
    let line = format!("staged: {}", quiet_mission_summary(g));
    quiet_mission_note(g, &line);
    let p = quiet_mission_spot(g).unwrap();
    let thing = quiet_mission_thing(g);
    let tail = if js::truthy(g.quiet.squad) { "Your partner is with you and staying silent. X for his orders" } else { "You're alone" };
    crate::game::game_msg(g, &format!("{} — {} is in {}. {}", p.place, thing, p.holder, tail));
    Ok(())
}

/// Sam faced as the start has him, the partner staged. A failure here is the TypeScript's throw: only `onEncounterRestart` takes it   (quiet.ts QuietMission.stage)
pub fn quiet_mission_stage(g: &mut crate::game::Game) {
    if let Err(why) = stage_or_why(g) { panic!("{}", why); }
}

/// the free floor nearest a stride behind Sam, walkable from him; else the fault an author is told   (quiet.ts QuietMission.behind)
fn behind_or_why(g: &mut crate::game::Game, s: &crate::scene_scenarios::QuietStart) -> Result<V3, String> {
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
        None => Err(format!("no free floor beside the start at ({}, {}) for the partner — give the start a 'mate' mark", num(s.sam[0]), num(s.sam[2]))),
    }
}

/// the free floor nearest a stride behind Sam, walkable from him   (quiet.ts QuietMission.behind)
pub fn quiet_mission_behind(g: &mut crate::game::Game, s: &crate::scene_scenarios::QuietStart) -> V3 {
    match behind_or_why(g, s) { Ok(p) => p, Err(why) => panic!("{}", why) }
}

/// THE FLOOR A MAN STANDS ON AT A HOLDER'S FACE: the nearest free cell with a clear line to the face and, given the room, inside it   (quiet.ts holderStand)
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
    crate::quiet::HolderStand { pos: best, ok: best.is_some() && clear && inside, clear, inside }
}

/// the quiet mission that is up on this game   (quiet.ts quietOf)
pub fn quiet_of(g: &mut crate::game::Game) {
    todo!("needs: quietOf's return value: the skeleton returns nothing; a ONCE class's `T | null` should be bool (g.quiet.active)")
}

/// `thing.replace(/^(the|a|an) /i, '')`
fn bare_of(thing: &str) -> &str {
    for lead in ["the ", "a ", "an "] {
        if let Some(head) = thing.get(..lead.len()) {
            if head.eq_ignore_ascii_case(lead) { return &thing[lead.len()..]; }
        }
    }
    thing
}

/// THE WORDS, off the level's data   (quiet.ts quietWords)
pub fn quiet_words(g: &mut crate::game::Game) -> Option<crate::quiet::QuietWords> {
    if !g.quiet.active { return None; }
    let p = quiet_mission_spot(g).unwrap();
    let thing = quiet_mission_thing(g);
    let exits = quiet_mission_exits(g);
    let n = exits.len();
    let outs = if n == 1 {
        exits[0].name.clone()
    } else if n == 2 {
        format!("{} or {}", exits[0].name, exits[1].name)
    } else {
        let head: Vec<&str> = exits[..n - 1].iter().map(|x| x.name.as_str()).collect();
        format!("{} or {}", head.join(", "), exits[n - 1].name)
    };
    let bare = String::from(bare_of(&thing));
    Some(crate::quiet::QuietWords { place: p.place, holder: p.holder, thing, outs, bare })
}

/// the exit Sam went out by (the nearest whose line he is past)   (quiet.ts exitTaken)
pub fn exit_taken(g: &mut crate::game::Game, p: V3) -> Option<crate::scene_scenarios::QuietExit> {
    if !g.quiet.active { return None; }
    let mut best = None;
    let mut bd = m::INF;
    for x in quiet_mission_exits(g) {
        if out_through(&x, p) {
            let d = m::hyp2(p[0] - x.x, p[2] - x.z);
            if d < bd { bd = d; best = Some(x); }
        }
    }
    best
}

/// the way out nearest `p` on foot-less distance   (quiet.ts nearestExit)
pub fn nearest_exit(g: &mut crate::game::Game, p: V3) -> Option<crate::scene_scenarios::QuietExit> {
    if !g.quiet.active { return None; }
    let mut best = None;
    let mut bd = m::INF;
    for x in quiet_mission_exits(g) {
        let d = m::hyp2(p[0] - x.x, p[2] - x.z);
        if d < bd { bd = d; best = Some(x); }
    }
    best
}

/// a point as a fault line writes it   (quiet.ts checkQuiet `at`)
fn at(p: V3) -> String {
    format!("({}, {})", num(p[0]), num(p[2]))
}

/// free floor, or a fault said   (quiet.ts checkQuiet `free`)
fn free(g: &mut crate::game::Game, out: &mut Vec<String>, what: &str, p: V3) -> bool {
    let b = crate::scene_collision::nav_grid_is_blocked(g, p[0], p[2]);
    if b { out.push(format!("{} {} is not free floor a man can stand on", what, at(p))); }
    !b
}

/// a way on foot   (quiet.ts checkQuiet `way`)
fn way(g: &mut crate::game::Game, a: V3, b: V3) -> bool {
    crate::scene_collision::nav_grid_find_path(g, a, b, Some(20000.0), None).map_or(false, |path| !path.is_empty())
}

/// a route by its name; the last of a name, as a Map built from the list has it   (quiet.ts checkQuiet `routes`)
fn route_named(g: &crate::game::Game, name: &str) -> Option<RouteId> {
    g.level.routes.iter().rev().copied().find(|&r| g.tab.route[r].name == name)
}

/// THE BLOCK AGAINST THE BUILT FLOOR: what the level file's validator cannot say. `g` is a Game stood up on the level, nothing started; the faults, each a line an author can act on   (quiet.ts checkQuiet)
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
                out.push(format!("{}.mate stands on top of sam ({:.2} m): give them a body's room", w, v3::dist_xz(s.sam, mate)));
            }
        }
    }
    let mut fronts: Vec<Option<V3>> = Vec::new();
    for (i, p) in q.spots.iter().enumerate() {
        let w = format!("spots[{}] '{}'", i, p.place);
        let a = p.area;
        let e = 0.3;
        if p.front[0] < a.x0 - e || p.front[0] > a.x1 + e || p.front[2] < a.z0 - e || p.front[2] > a.z1 + e {
            out.push(format!("{}.front {} is outside its area (x {}‥{}, z {}‥{})", w, at(p.front), num(a.x0), num(a.x1), num(a.z0), num(a.z1)));
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
            if far > 4.0 { out.push(format!("{}: its door '{}' is {:.1} m from its point — the marker would point at the wrong door", w, x.door, far)); }
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
                        out.push(format!("starts[{}].{} {} is in plain view of '{}' on his first waypoint {}, {:.1} m off — the run would begin seen", i, who, at(p), name, at(first), d));
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
        QuietExit { x: 1.5, z: -2.25, out: [0.6, 0.8], ..QuietExit::zeroed() }
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
            let d = pinned_deal(pin, s, p, q);
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

    #[test]
    fn the_bare_word_loses_only_a_leading_article() {
        assert_eq!(bare_of("the drive"), "drive");
        assert_eq!(bare_of("The Drive"), "Drive");
        assert_eq!(bare_of("An envelope"), "envelope");
        assert_eq!(bare_of("a key"), "key");
        assert_eq!(bare_of("another key"), "another key");
        assert_eq!(bare_of("theatre"), "theatre");
    }
}
