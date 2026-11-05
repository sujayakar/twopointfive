//! src/game/rescue.ts lines 1 to 437
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

// ── seam: the step skeleton 3 cannot take (notes.md, ## Skeleton) ──

/// (rescue.ts:351 gd.flashlight.enabled = false): a light has no table and no accessor
fn seam_light_disable(g: &mut Game, l: LightId) { todo!("needs: a light's enabled by its LightId") }

// ── small helpers ──

/// a number as a template string writes it, for the ordinary range of a mark's coordinates
fn num(x: f64) -> String { if x == 0.0 { String::from("0") } else { format!("{}", x) } }
/// `x.toFixed(2)` of a number >= 0: JavaScript takes the larger on an exact tie (a tie is an odd count of eighths), Rust the even
fn to_fixed2(x: f64) -> String {
    let e = x * 8.0;
    if m::floor(e) == e && js::rem(e, 2.0) == 1.0 { format!("{:.2}", x + 0.001) } else { format!("{:.2}", x) }
}
fn rect_of(b: crate::scene_level::BreachPlanBounds) -> crate::scene_scenarios::Rect {
    crate::scene_scenarios::Rect { x0: b.x0, x1: b.x1, z0: b.z0, z1: b.z1 }
}
fn plan_bounds_of(b: crate::scene_scenarios::Rect) -> crate::scene_level::BreachPlanBounds {
    crate::scene_level::BreachPlanBounds { x0: b.x0, x1: b.x1, z0: b.z0, z1: b.z1 }
}
/// `g.level.clearPlans?.find(p => p.door === door)?.bounds`
fn clear_plan_bounds(g: &Game, door: &str) -> Option<crate::scene_scenarios::Rect> {
    g.level.clear_plans.as_ref()?.iter().find(|p| p.door == door).and_then(|p| p.bounds).map(rect_of)
}
fn same_team_start(a: &crate::scene_scenarios::TeamStart, b: &crate::scene_scenarios::TeamStart) -> bool {
    a.sam == b.sam && a.sam_yaw == b.sam_yaw && a.ops == b.ops && a.ops_yaw == b.ops_yaw
}

/// orders.ts doorWords, by id, or the name its level wrote for it (rescue.ts doorName)
pub fn door_name(g: &mut crate::game::Game, id_2: &str) -> String {
    let d = crate::doors::doors_by_name(g, id_2);
    let written = match d { Some(d) => crate::doornames::door_written_name(g, d), None => None };
    match written { Some(s) => s, None => crate::orders::door_id_words(g, id_2) }
}

/// Distance on the floor (rescue.ts flat)
pub fn flat(a: V3, b: V3) -> f64 { m::hyp2(a[0] - b[0], a[2] - b[2]) }

/// An angle back into -pi..pi (rescue.ts wrap)
pub fn wrap(a: f64) -> f64 { m::atan2(m::sin(a), m::cos(a)) }

/// checkRescue's `free`: a man can be stood there. False = the place itself is wrong.
fn check_free(g: &mut Game, bad: &mut Vec<String>, what: &str, p: V3) -> bool {
    let mut q: V3 = [p[0], 0.0, p[2]];
    crate::scene_collision::collide_circle(g, &mut q, 0.3, 0.2, 1.5, None);
    let pushed = m::hyp2(q[0] - p[0], q[2] - p[2]);
    let near = crate::scene_collision::nav_grid_nearest_free_point(g, p[0], p[2]);
    let mut prop = None;
    for i in 0..g.props.props.len() {
        let pr = g.props.props[i];
        let row = &g.tab.props_prop[pr];
        let gap = crate::roomclear::prop_gap(crate::roomclear::PropGapP { home: row.home, hx: row.hx, hz: row.hz }, p);
        if gap < 0.3 { prop = Some(pr); break; }
    }
    if let Some(pr) = prop { bad.push(format!("{} ({}, {}): standing in {}", what, num(p[0]), num(p[2]), g.tab.props_prop[pr].def.name)); }
    let far = match near { None => true, Some(n) => m::hyp2(n[0] - p[0], n[2] - p[2]) > 0.8 };
    let why = if pushed > 0.15 { format!("in a wall or fixed furniture (pushed {} m out)", to_fixed2(pushed)) } else if far { String::from("no walkable floor within a step") } else { String::new() };
    if !why.is_empty() { bad.push(format!("{} ({}, {}): {}", what, num(p[0]), num(p[2]), why)); }
    why.is_empty()
}
/// checkRescue's `way`: joined on foot
fn check_way(g: &mut Game, a: V3, b: V3) -> bool {
    let aa = crate::scene_collision::nav_grid_nearest_free_point(g, a[0], a[2]);
    let bb = crate::scene_collision::nav_grid_nearest_free_point(g, b[0], b[2]);
    match (aa, bb) { (Some(aa), Some(bb)) => crate::scene_collision::nav_grid_find_path(g, aa, bb, None, None).is_some(), _ => false }
}
/// checkRescue's `line`: his eyes reach the point he covers
fn check_line(g: &mut Game, bad: &mut Vec<String>, what: &str, mk: crate::scene_scenarios::ScenarioMark) {
    if crate::roomclear::wall_between(g, [mk.pos[0], 1.6, mk.pos[2]], mk.aim_at) { bad.push(format!("{}: no line from his eyes to the point he covers", what)); }
}

/// The rescue's data against the floor it is laid on. Empty = sound. (rescue.ts checkRescue)
pub fn check_rescue(g: &mut crate::game::Game, s: Option<&crate::scene_scenarios::RescueData>) -> Vec<String> {
    let given = match s { Some(s) => Some(s.clone()), None => g.level.scenarios.as_ref().and_then(|c| c.rescue.clone()) };
    let Some(s) = given else { return vec![String::from("this level has no hostage rescue data (its scenarios.rescue block)")]; };
    let mut bad: Vec<String> = Vec::new();
    let bb = match s.room.bounds { Some(b) => Some(b), None => clear_plan_bounds(g, &s.room.door) };
    if crate::doors::doors_by_name(g, &s.room.door).is_none() { bad.push(format!("no door '{}' in the level (the hostage room's)", s.room.door)); }
    if bb.is_none() { bad.push(format!("the hostage room has no bounds: neither room.bounds nor a clear plan for '{}'", s.room.door)); }
    let ops: Vec<V3> = s.team.ops.clone().unwrap_or_default();
    if ops.len() < 2 { bad.push(String::from("team.ops: the pair needs two marks")); }
    let sam_ok = check_free(g, &mut bad, "team start, the player", s.team.sam);
    for (i, p) in ops.iter().enumerate() {
        if check_free(g, &mut bad, &format!("team start, teammate {}", i + 1), *p) && sam_ok && !check_way(g, s.team.sam, *p) {
            bad.push(format!("team start, teammate {}: no way on foot from the player's mark", i + 1));
        }
    }
    if s.hostages.is_empty() { bad.push(String::from("no hostages")); }
    for (i, h) in s.hostages.iter().enumerate() {
        let what = format!("hostage {}", i + 1);
        if check_free(g, &mut bad, &what, h.pos) {
            if let Some(b) = bb {
                if !crate::squad::in_bounds(h.pos, Some(plan_bounds_of(b)), None) { bad.push(format!("{} ({}, {}): outside the hostage room", what, num(h.pos[0]), num(h.pos[2]))); }
            }
        }
    }
    let mut men: Vec<(String, crate::scene_scenarios::ScenarioMark, bool)> = vec![
        (String::from("the captor"), s.captor, true),
        (String::from("the door man, inside"), s.door_man.inside, true),
        (String::from("the door man, outside"), s.door_man.outside, false),
    ];
    if let Some(guards) = &s.guards { for (i, r) in guards.iter().enumerate() { men.push((format!("room guard {}", i + 1), *r, true)); } }
    for (i, r) in s.roamers.iter().enumerate() { men.push((format!("roamer {}", i + 1), *r, false)); }
    for (what, mk, inside) in men.iter() {
        if !check_free(g, &mut bad, what, mk.pos) { continue; }
        if let Some(b) = bb {
            if *inside != crate::squad::in_bounds(mk.pos, Some(plan_bounds_of(b)), None) {
                bad.push(format!("{} ({}, {}): {} the hostage room", what, num(mk.pos[0]), num(mk.pos[2]), if *inside { "outside" } else { "inside" }));
            }
        }
        if what != "the door man, outside" { check_line(g, &mut bad, what, *mk); }
        if sam_ok && !check_way(g, s.team.sam, mk.pos) { bad.push(format!("{}: no way on foot from the team's start", what)); }
    }
    if let Some(side) = &s.side {
        if crate::doors::doors_by_name(g, &side.door).is_none() { bad.push(format!("no door '{}' in the level (the side door)", side.door)); }
        if let Some(from) = &side.from {
            if crate::doors::doors_by_name(g, &from.door).is_none() { bad.push(format!("no door '{}' in the level (the room the side door is taken from)", from.door)); }
        }
        for (k, list) in [("stack", &side.stack), ("standoff", &side.standoff)] {
            if let Some(list) = list {
                for (i, p) in list.iter().enumerate() { check_free(g, &mut bad, &format!("side door {} mark {}", k, i + 1), *p); }
            }
        }
    }
    if let Some(flow) = &s.flow {
        for o in flow.route.iter() {
            if !flow.regions.iter().any(|r| r.id == o.region) { bad.push(format!("route opening '{}': no region '{}'", o.id, o.region)); }
        }
        let was = crate::rescueflow::rescue_flow_check(g);
        for w in was { bad.push(format!("route \u{2014} {}", w)); }
    }
    bad
}

/// A new one: the field initialisers, and its three helpers. (rescue.ts Rescue.constructor)
pub fn rescue_new(g: &mut crate::game::Game) -> crate::rescue::Rescue {
    let mut r = Rescue::blank();
    r.flow = Box::new(crate::rescueflow::rescue_flow_new(g));
    r.ending = crate::rescueend::rescue_end_new(g);
    r.flush = crate::rescueflush::rescue_flush_new(g);
    r
}

/// What the scenario takes from the level it stands on. (rescue.ts Rescue.data)
pub fn rescue_data(g: &mut crate::game::Game) -> Option<crate::scene_scenarios::RescueData> {
    g.level.scenarios.as_ref().and_then(|c| c.rescue.clone())
}

/// The hostage room's floor inside its walls. (rescue.ts Rescue.bounds)
pub fn rescue_bounds(g: &mut crate::game::Game) -> Option<crate::scene_scenarios::Rect> {
    let s = rescue_data(g);
    if let Some(b) = s.as_ref().and_then(|s| s.room.bounds) { return Some(b); }
    if let Some(p) = &g.rescue.plan { return p.bounds.map(rect_of); }
    match &s { Some(s) => clear_plan_bounds(g, &s.room.door), None => None }
}

/// The room the side door is taken from. (rescue.ts Rescue.fromBounds)
pub fn rescue_from_bounds(g: &mut crate::game::Game) -> Option<crate::scene_scenarios::Rect> {
    let s = rescue_data(g)?;
    let f = s.side.as_ref()?.from.as_ref()?;
    match f.bounds { Some(b) => Some(b), None => clear_plan_bounds(g, &f.door) }
}

fn out_of_snap(x: f64) -> f64 {
    if m::abs(x) < 1e-9 { 0.0 } else if m::abs(m::abs(x) - 1.0) < 1e-9 { m::sign(x) } else { x }
}
/// `dist` out of a door's frame on the side away from a room. (rescue.ts Rescue.outOf)
pub fn rescue_out_of(g: &mut crate::game::Game, d: DoorId, room: Option<crate::scene_scenarios::Rect>, dist: f64) -> V3 {
    let th = g.dr[d].def.closed_dir;
    let s = match room {
        Some(r) => -crate::orders::side_of(g, d, [(r.x0 + r.x1) / 2.0, 0.0, (r.z0 + r.z1) / 2.0]),
        None => 1.0,
    };
    let fc = g.dr[d].frame_centre;
    [fc[0] + out_of_snap(-m::sin(th)) * s * dist, 0.0, fc[2] + out_of_snap(m::cos(th)) * s * dist]
}

/// The floor just outside the hostage room's main door, or the point its first man in holds. (rescue.ts Rescue.roomPoint)
pub fn rescue_room_point(g: &mut crate::game::Game, which: crate::rescue::RescueRoomPointWhich) -> V3 {
    let s = rescue_data(g).unwrap();
    let given = match which { RescueRoomPointWhich::Outside => s.room.outside, RescueRoomPointWhich::Hold => s.room.hold };
    if let Some(p) = given { return v3::copy(p); }
    let d = crate::doors::doors_by_name(g, &s.room.door);
    let a = s.room.door_aim;
    let k = if which == RescueRoomPointWhich::Outside { 1.0 } else { 0.4 };
    let Some(d) = d else { return [a[0], 0.0, a[2]]; };
    let b = rescue_bounds(g);
    let o = rescue_out_of(g, d, b, k);
    let fc = g.dr[d].frame_centre;
    [a[0] + (o[0] - fc[0]), 0.0, a[2] + (o[2] - fc[2])]
}

/// Why it cannot start right now (None = it can). (rescue.ts Rescue.canStart)
pub fn rescue_can_start(g: &mut crate::game::Game) -> Option<String> {
    let s = rescue_data(g);
    if g.quiet.active { return Some(String::from("a quiet mission is up on this floor (quit it first)")); }
    let Some(s) = s else { return Some(String::from("this level has no hostage rescue data (its scenarios.rescue block)")); };
    if s.team.ops.as_ref().map_or(0, |o| o.len()) < 2 { return Some(String::from("this level's rescue block has no start marks for the pair (team.ops needs two)")); }
    if rescue_bounds(g).is_none() { return Some(format!("this level has no clear plan for the '{}' door", s.room.door)); }
    if crate::doors::doors_by_name(g, &s.room.door).is_none() { return Some(format!("the door '{}' is not in the level", s.room.door)); }
    if !g.tune.teams { return Some(String::from("sides are switched off (tune.teams)")); }
    if !g.ai_enabled { return Some(String::from("the AI is switched off (sandbox: no AI) \u{2014} nobody would move")); }
    if let Some(run) = &g.director.run {
        if !run.ended { return Some(format!("the level script's program '{}' is running", run.name)); }
    }
    if g.touring || g.quiet_utility || g.puppet.is_some() { return Some(String::from("a show is running")); }
    if g.night_breach.active {
        return Some(String::from(if crate::roomclear::room_clear_scenario_active(g) { "the room clear is staged" } else { "the night breach is staged" }));
    }
    if g.guards.iter().any(|x| g.gd[*x].unit.is_some()) { return Some(String::from("a spawned unit is already on the floor")); }
    None
}

/// Start it. Returns the reason when refused or when the staging failed. (rescue.ts Rescue.start)
pub fn rescue_start(g: &mut crate::game::Game) -> Option<String> {
    if g.rescue.active { return None; }
    if let Some(why) = rescue_can_start(g) { return Some(why); }
    {
        let t = rescue_data(g).unwrap().team;
        let same = g.m.rescue.applied.as_ref().map_or(false, |a| same_team_start(a, &t));
        if !same {
            let ops = t.ops.clone().unwrap();
            g.k.rescue.sam_at = t.sam;
            g.k.rescue.sam_yaw = t.sam_yaw;
            g.k.rescue.ops_at = [ops[0], ops[1]];
            g.m.rescue.applied = Some(t);
        }
    }
    rescue_bench(g);
    g.rescue.active = true;
    g.rescue.staged = 0.0;
    g.rescue.log.clear();
    g.rescue.failure = None;
    crate::game::game_restart_encounter(g);
    if g.rescue.active { None } else { Some(g.rescue.failure.clone().unwrap_or_else(|| String::from("the staging failed"))) }
}

/// Stop it: the cast gone, the shift back, a fresh night encounter. (rescue.ts Rescue.stop)
pub fn rescue_stop(g: &mut crate::game::Game) {
    if !g.rescue.active { return; }
    g.rescue.active = false;
    rescue_clear_cast(g);
    if let Some(mut benched) = g.rescue.benched.take() {
        for gd in benched.iter() {
            let l = g.gd[*gd].flashlight;
            if !g.engine.lights.lights.contains(&l) { crate::render_lights::light_set_add(&mut g.engine.lights, l); }
        }
        benched.extend_from_slice(&g.guards);
        g.guards = benched;
    }
    crate::game::game_restart_encounter(g);
    let text = if !g.level.routes.is_empty() { "hostage rescue off \u{2014} the night shift is back" } else { "hostage rescue off" };
    crate::game::game_msg(g, text);
}

/// Stage it afresh: a restart of the encounter, which stages. (rescue.ts Rescue.restage)
pub fn rescue_restage(g: &mut crate::game::Game) {
    if g.rescue.active { crate::game::game_restart_encounter(g); }
}

/// restartEncounter's last act: stages on the fresh encounter; a staging that fails ends it cleanly. (rescue.ts Rescue.onEncounterRestart)
pub fn rescue_on_encounter_restart(g: &mut crate::game::Game) {
    if !g.rescue.active { return; }
    if let Err(e) = stage_tried(g) {
        let failure = format!("the staging failed: {}", e);
        g.rescue.failure = Some(failure.clone());
        rescue_stop(g);
        crate::game::game_msg(g, &format!("hostage rescue \u{2014} {}", failure));
    }
}

/// Who has the pair. (rescue.ts Rescue.mode)
pub fn rescue_mode(g: &mut crate::game::Game) -> crate::rescue::RescueMode {
    if g.rescue.entry.is_some() { RescueMode::Door } else { RescueMode::Free }
}

/// F's entry has this man right now. (rescue.ts Rescue.owns)
pub fn rescue_owns(g: &mut crate::game::Game, gd: GuardId) -> bool {
    if !g.rescue.active { return false; }
    let Some(e) = &g.rescue.entry else { return false; };
    e.ops.iter().any(|x| x.ended.is_none() && g.tab.doortools_door_op[x.op].men.contains(&gd))
}

/// Call F's entry off: its men stop where they are. Always None. (rescue.ts Rescue.callOff)
pub fn rescue_call_off(g: &mut crate::game::Game) -> Option<String> {
    if !g.rescue.active { return None; }
    let Some(men) = g.rescue.entry.as_ref().map(|e| e.men.clone()) else { return None; };
    for o in men {
        if g.gd[o].state != crate::game::GuardState::Dead && rescue_owns(g, o) { rescue_post(g, o, None, None); }
    }
    rescue_note(g, "drill:called-off");
    rescue_end_entry(g, false);
    None
}

/// Is this civilian one of the scenario's hostages. (rescue.ts Rescue.isHostage)
pub fn rescue_is_hostage(g: &mut crate::game::Game, gd: GuardId) -> bool {
    g.rescue.active && g.rescue.hostages.contains(&gd)
}

/// The shift off the floor, lights out, kept for stop(). (rescue.ts Rescue.bench)
pub fn rescue_bench(g: &mut crate::game::Game) {
    let staff: Vec<GuardId> = g.guards.iter().copied().filter(|x| g.gd[*x].unit.is_none()).collect();
    if staff.is_empty() { return; }
    for gd in staff.iter() {
        let l = g.gd[*gd].flashlight;
        seam_light_disable(g, l);
        crate::render_lights::light_set_remove(&mut g.engine.lights, l);
    }
    let mut benched = g.rescue.benched.take().unwrap_or_default();
    benched.extend_from_slice(&staff);
    g.rescue.benched = Some(benched);
    let kept: Vec<GuardId> = g.guards.iter().copied().filter(|x| g.gd[*x].unit.is_some()).collect();
    g.guards = kept;
}

/// The cast gone and everything kept about it forgotten. (rescue.ts Rescue.clearCast)
pub fn rescue_clear_cast(g: &mut crate::game::Game) {
    let mut men = g.rescue.ops.clone();
    men.extend_from_slice(&g.rescue.hostiles);
    for gd in men {
        if g.gd[gd].script.is_some() { crate::squad::end_guard_script(g, gd, Some(crate::squad::EndGuardScriptState::Patrol)); }
        crate::guards::despawn_unit(g, gd);
    }
    let hostages = g.rescue.hostages.clone();
    g.civilians.retain(|c| !hostages.contains(c));
    g.rescue.ops = Vec::new();
    g.rescue.hostiles = Vec::new();
    g.rescue.hostages = Vec::new();
    g.rescue.entry = None;
    g.rescue.defs.clear();
    g.rescue.act = None;
    g.rescue.search_t.clear();
    g.rescue.search_a.clear();
    crate::rescueflow::rescue_flow_reset(g);
    crate::rescueend::rescue_end_reset(g);
    crate::rescueflush::rescue_flush_reset(g);
    rescue_marks(g, false);
}

/// The scenario's own additions to the squad's door work: the side door's marks and the room's two doors linked. (rescue.ts Rescue.marks)
pub fn rescue_marks(g: &mut crate::game::Game, on: bool) {
    let s = rescue_data(g);
    let side = s.as_ref().and_then(|s| s.side.clone());
    let fd = match &s { Some(s) => crate::doors::doors_by_name(g, &s.room.door), None => None };
    let sd = match &side { Some(x) => crate::doors::doors_by_name(g, &x.door), None => None };
    let mut i = g.orders.door_marks.len();
    while i > 0 {
        i -= 1;
        if sd == Some(g.orders.door_marks[i].door) { g.orders.door_marks.remove(i); }
    }
    g.door_tools.linked.retain(|d| Some(*d) != fd && Some(*d) != sd);
    g.door_tools.lights = None;
    if !on { return; }
    if let (Some(sd), Some(x)) = (sd, side.as_ref()) {
        if let Some(stack) = x.stack.as_ref().filter(|st| !st.is_empty()) {
            let face = rescue_face_of(g, sd);
            g.orders.door_marks.push(crate::orders::SquadOrdersDoorMarksItem {
                door: sd, side: face, stack: stack.clone(),
                standoff: x.standoff.clone().unwrap_or_else(|| stack.clone()),
            });
        }
    }
    if let Some(d) = fd { g.door_tools.linked.push(d); }
    if let Some(d) = sd { g.door_tools.linked.push(d); }
}

/// The face of one of the room's doors that is not the room's. (rescue.ts Rescue.faceOf)
pub fn rescue_face_of(g: &mut crate::game::Game, d: DoorId) -> f64 {
    match rescue_bounds(g) {
        Some(b) => -crate::orders::side_of(g, d, [(b.x0 + b.x1) / 2.0, 0.0, (b.z0 + b.z1) / 2.0]),
        None => 1.0,
    }
}

/// One of the cast before he is spawned (rescue.ts stage's `cast`)
struct CastRow { role: Role, in_room: bool, mark: crate::scene_scenarios::ScenarioMark, name: String }

/// Everybody on his mark on the fresh encounter. (rescue.ts Rescue.stage)
pub fn rescue_stage(g: &mut crate::game::Game) {
    if let Err(e) = stage_tried(g) { panic!("{}", e); }
}

/// The staging; Err is the message of what the TypeScript's staging throws.
fn stage_tried(g: &mut Game) -> Result<(), String> {
    let Some(s) = rescue_data(g) else { return Err(String::from("the level has no hostage rescue data")); };
    g.rescue.ops = Vec::new();
    g.rescue.hostiles = Vec::new();
    g.rescue.hostages = Vec::new();
    g.rescue.entry = None;
    g.rescue.defs.clear();
    g.rescue.called = false;
    g.rescue.search_t.clear();
    g.rescue.search_a.clear();
    crate::rescueflow::rescue_flow_reset(g);
    crate::rescueend::rescue_end_reset(g);
    crate::rescueflush::rescue_flush_reset(g);
    g.rescue.plan = g.level.clear_plans.as_ref().and_then(|l| l.iter().find(|p| p.door == s.room.door).cloned());
    rescue_marks(g, true);
    {
        let sv = match s.side.as_ref().and_then(|x| x.from.as_ref()) { Some(f) => crate::doors::doors_by_name(g, &f.door), None => None };
        g.rescue.server_out = match sv {
            Some(sv) => { let b = rescue_from_bounds(g); Some(rescue_out_of(g, sv, b, 1.0)) }
            None => None,
        };
    }
    crate::roomfight::room_fight_reset(g);
    g.fight.blind_fire = true;
    // the hostage room's door shut on its latch (the night leaves it ajar)
    let Some(d) = crate::doors::doors_by_name(g, &s.room.door) else { return Err(String::from("Cannot read properties of undefined (reading 'reset')")); };
    crate::doors::door_reset(g, d);
    crate::doors::door_set_angle(g, d, 0.0);
    crate::doors::door_set_vel(g, d, 0.0);
    crate::doors::door_set_latched(g, d, true);
    crate::doors::door_set_closing(g, d, false);
    // the side door from the server room: shut, and for the scenario not locked
    let side = match &s.side { Some(x) => crate::doors::doors_by_name(g, &x.door), None => None };
    if let Some(sd) = side {
        crate::doors::door_reset(g, sd);
        crate::doors::door_set_angle(g, sd, 0.0);
        crate::doors::door_set_vel(g, sd, 0.0);
        crate::doors::door_set_latched(g, sd, true);
        crate::doors::door_set_closing(g, sd, false);
        crate::doors::door_set_locked(g, sd, false);
    }
    // the opposition: captor, door man, roamers
    let mut cast: Vec<CastRow> = vec![CastRow { role: Role::Captor, in_room: true, mark: s.captor, name: String::from("Captor") }];
    if g.k.rescue.door_man == crate::knobs::RescueKDoorMan::Inside {
        cast.push(CastRow { role: Role::Door, in_room: true, mark: s.door_man.inside, name: String::from("Door man") });
    } else {
        cast.push(CastRow { role: Role::Door, in_room: false, mark: s.door_man.outside, name: String::from("Door man") });
    }
    if let Some(guards) = &s.guards {
        for (i, mk) in guards.iter().enumerate() { cast.push(CastRow { role: Role::Guard, in_room: true, mark: *mk, name: format!("Room guard {}", i + 1) }); }
    }
    let len = s.roamers.len() as f64;
    if let Some(c) = &s.cast {
        // samestart.ts on the page: the first n marks as written, the die drawn all the same
        let n = m::max(0.0, m::min(len, m::round(c.roamers))) as usize;
        for _ in 0..n { crate::math_dice::sim_random(g); }
        for i in 0..n { cast.push(CastRow { role: Role::Roamer, in_room: false, mark: s.roamers[i], name: format!("Roamer {}", i + 1) }); }
    } else {
        crate::math_dice::sim_random(g);   // (seedRoamers's draw: samestart.ts keeps the swap off and the stream the same)
        let n = m::max(0.0, m::min(len, m::round(g.k.rescue.roamers))) as usize;
        for i in 0..n { cast.push(CastRow { role: Role::Roamer, in_room: false, mark: s.roamers[i % s.roamers.len()], name: format!("Roamer {}", i + 1) }); }
    }
    let budget = 1.0 + cast.len() as f64;
    for (k, c) in cast.iter().enumerate() {
        let gd = crate::guards::spawn_unit(g, &crate::guards::UnitOpts {
            team: crate::teams::Team::Hostile, pos: c.mark.pos, yaw: Some(v3::yaw_to(c.mark.pos, c.mark.aim_at)),
            name: Some(c.name.clone()), light: Some(false), budget: Some(budget), ..crate::guards::UnitOpts::zeroed()
        });
        g.gd[gd].hold = true;
        g.gd[gd].pinned = true;
        g.rescue.hostiles.push(gd);
        g.rescue.defs.set(gd, Def {
            role: c.role, in_room: c.in_room, mark: DefMark { pos: c.mark.pos, aim_at: c.mark.aim_at },
            learned: f64::NAN, set: false, saw_foe: false, released: false,
            burst: crate::nightbreach::BlindBurst { until: -1.0, n: 0.0, next: 0.0 },
            k: k as f64, crew: DefCrew::Empty, look: None, spot: None, noise0: None,
        });
    }
    let deal: Vec<crate::gunroles::DealMan> = g.rescue.hostiles.iter().enumerate().map(|(k, gd)| crate::gunroles::DealMan {
        gd: *gd, watch: cast[k].mark.aim_at, hostage: Some(cast[k].role == Role::Captor),
    }).collect();
    crate::gunroles::gun_deal(g, &deal);
    let cap_set = g.k.rescue.start == crate::knobs::RescueKStart::Hot;
    if cap_set {
        let captor = g.rescue.hostiles[0];
        g.rescue.defs.get_mut(captor).unwrap().set = true;
    }
    // the team at the lobby arch
    let yaw = match s.team.ops_yaw { Some(y) => y, None => m::PI / 2.0 };
    let mut at: Vec<V3> = vec![g.k.rescue.ops_at[0], g.k.rescue.ops_at[1]];
    let squad = g.k.rescue.squad;
    let mut i = 2usize;
    while (i as f64) < squad {
        let mine = s.team.ops.as_ref().and_then(|o| o.get(i).copied());
        let p = match mine {
            Some(p) => v3::copy(p),
            None => {
                let mut taken = at.clone();
                taken.push(g.k.rescue.sam_at);
                crate::nightbreach::night_breach_stand_by(g, at[i - 2], &taken, Some(yaw))
            }
        };
        at.push(p);
        i += 1;
    }
    let mut i = 0usize;
    while (i as f64) < squad {
        let row = if squad > 2.0 { crate::nightbreach::squad4().get(i).cloned() } else { None };
        let opts = match row {
            Some(r) => {
                let n = 2.0 + g.guards.len() as f64;
                crate::guards::UnitOpts {
                    team: crate::teams::Team::Operative, pos: at[i], yaw: Some(yaw), name: Some(r.name), light: Some(false),
                    pair: Some(if r.pair == "red" { crate::game::PairId::Red } else { crate::game::PairId::Blue }), tint: Some(r.tint), tint2: Some(r.tint2),
                    budget: Some(m::max(crate::nightbreach::SQUAD4_BUDGET, n)), ..crate::guards::UnitOpts::zeroed()
                }
            }
            None => crate::guards::UnitOpts {
                team: crate::teams::Team::Operative, pos: at[i], yaw: Some(yaw),
                name: Some(String::from(if i == 0 { "Blue One" } else { "Blue Two" })), light: Some(false),
                budget: Some(2.0 + g.guards.len() as f64), ..crate::guards::UnitOpts::zeroed()
            },
        };
        let gd = crate::guards::spawn_unit(g, &opts);
        g.gd[gd].hold = true;
        g.gd[gd].pinned = true;
        g.gd[gd].trained = true;
        g.rescue.ops.push(gd);
        i += 1;
    }
    // the hostages: the building's own two civilians' names and clothes when the level has them
    let defs = g.level.civilians.clone().unwrap_or_default();
    for (i, h) in s.hostages.iter().enumerate() {
        let cd = defs.get(i);
        let name = match cd {
            Some(c) => c.name.clone(),
            None => match ["Marta", "Owen", "Ines"].get(i) { Some(n) => String::from(*n), None => format!("Hostage {}", i + 1) },
        };
        let kind = match cd {
            Some(c) => match c.kind {
                crate::scene_level::CivilianDefKind::Cleaner => crate::civilians::SpawnHostageDefKind::Cleaner,
                crate::scene_level::CivilianDefKind::Worker => crate::civilians::SpawnHostageDefKind::Worker,
            },
            None => if i == 0 { crate::civilians::SpawnHostageDefKind::Cleaner } else { crate::civilians::SpawnHostageDefKind::Worker },
        };
        let tint = match cd {
            Some(c) => c.tint,
            None => if i == 0 { [0.3, 0.42, 0.5] } else if i == 1 { [0.5, 0.48, 0.42] } else { [0.46, 0.33, 0.36] },
        };
        let hostage = crate::civilians::spawn_hostage(g, &crate::civilians::SpawnHostageDef {
            name, kind, tint, tint2: cd.and_then(|c| c.tint2), pos: h.pos, face: h.face,
        });
        g.rescue.hostages.push(hostage);
    }
    g.rescue.resolve = 0.0;
    crate::math_dice::sim_random(g);   // (samestart.ts on: the clock is the knob's to the second, the die drawn all the same)
    g.rescue.resolve_secs = m::max(5.0, g.k.rescue.resolve_secs);
    g.rescue.t0 = f64::NAN;
    g.rescue.door_orders = 0.0;
    g.rescue.blind_rounds = 0.0;
    g.rescue.result = None;
    g.rescue.act = None;
    g.rescue.cap_by = None;
    g.rescue.executed.clear();
    g.rescue.counted.clear();
    g.rescue.secured_at = f64::NAN;
    g.rescue.step = RescueStep::Waiting;
    g.rescue.bark_at = 0.0;
    g.rescue.noise = None;
    g.rescue.alarm_at = f64::NAN;
    g.rescue.way = None;
    g.rescue.last_order = g.orders.log.last().map(|o| o.serial);
    let pl = g.player;
    let sam_at = g.k.rescue.sam_at;
    crate::game::game_teleport_player(g, v3::copy(sam_at));
    let c = g.pl[pl].ch;
    let sam_yaw = g.k.rescue.sam_yaw;
    g.ch[c].aim_yaw = sam_yaw;
    g.ch[c].body_yaw = sam_yaw;
    if g.pl[pl].has_rifle { g.pl[pl].slot = crate::game::SlotKind::Rifle; }
    g.pl[pl].holstered = false;
    g.rescue.staged += 1.0;
    g.rescue.said.clear();
    g.rescue.cut = true;
    g.rescue.murmur_at = g.time + 4.0;
    g.rescue.murmur_n = 0.0;
    g.rescue.ack_at = f64::NAN;
    let ops = g.rescue.ops.clone();
    crate::kit::kit_stage(g, &ops);
    rescue_note(g, "staged");
    if cap_set {
        let callsign = g.gd[g.rescue.hostiles[0]].callsign.clone();
        rescue_note(g, &format!("set:{}", callsign));
    }
    let count = s.hostages.len();
    let many = number().get(count).cloned().unwrap_or_else(|| format!("{}", count));
    let who = format!("{} hostage{} in the {}", many, if count == 1 { "" } else { "s" }, rescue_place(g));
    let n = g.rescue.ops.len();
    let fall = if n > 2 { ", and the squad falls in" } else if n == 2 { ", and the pair falls in" } else if n > 0 { ", and your teammate falls in" } else { "" };
    let text = if g.k.rescue.start == crate::knobs::RescueKStart::Hot {
        format!("hostage rescue. you hear shots inside. {}. the clock starts when you move{}. {}", who, fall, if n > 0 { "Press F to bang and clear, or X for orders (door tools on any door)" } else { "You're alone" })
    } else {
        format!("hostage rescue. {}. nobody knows you're here yet. {}", who, if n > 0 { "Press F to stack up and F again to go, or X for orders" } else { "You're alone" })
    };
    crate::game::game_msg(g, &text);
    Ok(())
}

/// The hostage room in words. (rescue.ts Rescue.place)
pub fn rescue_place(g: &mut crate::game::Game) -> String {
    let Some(s) = rescue_data(g) else { return String::from("hostage room"); };
    match s.room.where_.as_deref() {
        Some(w) if !w.is_empty() => format!("{}, {}", s.room.name, w),
        _ => s.room.name,
    }
}

/// What happened when, on the game clock. (rescue.ts Rescue.note)
pub fn rescue_note(g: &mut crate::game::Game, what: &str) {
    let t = g.time;
    g.rescue.log.push(RescueLogItem { t, what: String::from(what) });
    if g.rescue.log.len() > 600 { g.rescue.log.drain(0..150); }
}

#[cfg(test)]
mod tests {
    use super::*;
    // the bits are the TypeScript's own, under bun with EXACT.on = true
    #[test]
    fn flat_and_wrap_are_the_typescripts_bits() {
        assert_eq!(flat([7.6, 0.0, 13.4], [-3.25, 9.0, 0.1]).to_bits(), 0x40312a0e3d2cb01b);
        assert_eq!(flat([1.0, 0.0, 2.0], [1.0, 5.0, 2.0]).to_bits(), 0);
        assert_eq!(wrap(4.0).to_bits(), 0xc00243f6a8885a30);
        assert_eq!(wrap(-7.5).to_bits(), 0xbff37812aeef4b9e);
        assert_eq!(wrap(0.3).to_bits(), 0x3fd3333333333333);
    }
    #[test]
    fn to_fixed_as_the_language_has_it() {
        for (x, s) in [(0.125, "0.13"), (0.375, "0.38"), (0.625, "0.63"), (1.005, "1.00"), (0.155, "0.15"), (2.5, "2.50")] {
            assert_eq!(to_fixed2(x), s);
        }
    }
}
