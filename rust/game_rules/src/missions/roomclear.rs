pub use ::game_types::roomclear::types;
pub use types::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{format, string::String, vec, vec::Vec};

/// `nav.isBlocked` over the game's nav grid
fn blocked(g: &mut Game, x: f64, z: f64) -> bool {
    crate::scene_collision::nav_grid_is_blocked(g, x, z)
}
/// `inBounds(p, rect)`: a structure's Rect is the same four numbers as the plan's bounds the squad's function takes
fn in_rect(p: V3, r: crate::scene_scenarios::Rect) -> bool {
    crate::squad::in_bounds(p, Some(crate::scene_level::BreachPlanBounds { x0: r.x0, x1: r.x1, z0: r.z0, z1: r.z1 }), None)
}
/// `h.state !== 'dead' && !h.fled`
fn holds(g: &Game, h: GuardId) -> bool {
    !crate::condition::down(g, g.gd[h].ch) && !g.gd[h].fled
}
/// `nb.ops.filter(o => o.state !== 'dead').length`
fn ops_up(g: &Game) -> usize {
    g.night_breach.ops.iter().filter(|o| !crate::condition::down(g, g.gd[**o].ch)).count()
}
/// the structure as checkStructure's parameter has it
fn check_s_of(s: &crate::scene_scenarios::StructureData) -> crate::roomclear::CheckStructureS {
    crate::roomclear::CheckStructureS { rooms: s.rooms.clone(), team: s.team.clone(), starts: s.starts.clone(), max_men: s.max_men, squads: s.squads.clone() }
}
/// `T.ops && T.ops.length >= 2`
fn two_ops(t: &crate::scene_scenarios::TeamStart) -> bool {
    t.ops.as_ref().map_or(false, |o| o.len() >= 2)
}

/// the placement seed a fresh start deals for a Game's die
fn fresh_seed_of(dice_seed: f64) -> f64 {
    1.0 + js::rem(js::to_u32(dice_seed) as f64, 9973.0)
}

/// WHO STANDS WHERE, a pure function of (seed, attempt)
/// The rooms are the level's own (`level.scenarios.roomClear`), handed in: this module holds no level's rooms of its own.
fn place_defenders(seed: f64, attempt: Option<f64>, s: &crate::roomclear::PlaceDefendersS) -> Vec<crate::roomclear::Placement> {
    let attempt = attempt.unwrap_or(1.0);
    let authored = seed == 0.0 && attempt <= 1.0;
    let word = m::imul(js::to_i32(seed), 0x9E3779B1u32 as i32) ^ m::imul(js::to_i32(attempt), 0x85EBCA6Bu32 as i32);
    // the TypeScript's mulberry32(word): crate::dice::Die is the same stream, made and spent in this call
    let mut die = crate::dice::Die::new(word as u32);
    for _ in 0..4 {
        die.roll();
    }
    let mut out: Vec<crate::roomclear::Placement> = Vec::new();
    for r in &s.rooms {
        let mut idx: Vec<usize> = (0..r.marks.len()).collect();
        if !authored {
            let mut i = idx.len() as isize - 1;
            while i > 0 {
                let j = m::floor(die.roll() * (i as f64 + 1.0)) as usize;
                idx.swap(i as usize, j);
                i -= 1;
            }
        }
        idx.truncate(r.garrison as usize);
        js::sort_by(&mut idx, |a, b| *a as f64 - *b as f64);
        for k in idx {
            let mk = &r.marks[k];
            let name = format!("Tango {}", out.len() + 1);
            out.push(crate::roomclear::Placement { room: r.id.clone(), mark: k as f64, pos: mk.pos, aim_at: mk.aim_at, name });
        }
    }
    out
}

/// The structure against the level it stands in, every problem in words (empty = sound)
pub fn check_structure(g: &mut crate::game::Game, s: Option<&crate::roomclear::CheckStructureS>) -> Vec<String> {
    let s: crate::roomclear::CheckStructureS = match s {
        Some(s) => s.clone(),
        None => match g.level.scenarios.as_ref().and_then(|c| c.room_clear.as_ref()) {
            Some(d) => check_s_of(d),
            None => return vec![String::from("this level has no room clear structure (its scenarios.roomClear block)")],
        },
    };
    let mut bad: Vec<String> = Vec::new();
    if g.level.breach.is_none() && !s.team.as_ref().map_or(false, two_ops) {
        bad.push(String::from("the level has no breach plan, and the structure does not say where the team starts (team.sam, team.ops)"));
    }
    if s.starts.as_ref().map_or(false, |v| !v.is_empty()) && s.team.is_none() {
        bad.push(String::from("starts: other ways in are given beside the structure's own team start, never without one"));
    }
    // where the team starts: the structure's own start, then each other way in
    let mut starts: Vec<Option<crate::scene_scenarios::TeamStart>> = vec![s.team.clone()];
    for t in s.starts.iter().flatten() {
        starts.push(Some(t.clone()));
    }
    for (k, t) in starts.iter().enumerate() {
        let t = match t {
            Some(t) => t,
            None => continue,
        };
        let start = if k > 0 { format!("team start {}", k + 1) } else { String::from("team start") };
        let its = if k > 0 { format!("the team's start {}", k + 1) } else { String::from("the team's start") };
        if k > 0 && !two_ops(t) {
            bad.push(format!("{}: needs the teammates' marks (ops, two at least)", start));
        }
        let mut who: Vec<(String, V3)> = vec![(String::from("Player"), t.sam)];
        for (i, q) in t.ops.iter().flatten().enumerate() {
            who.push((format!("teammate {}", i + 1), *q));
        }
        for (name, p) in &who {
            if blocked(g, p[0], p[2]) {
                bad.push(format!("{}, {} ({}, {}): not on floor a man can stand on", start, name, js::num_str(p[0]), js::num_str(p[2])));
            } else if name != "Player" && crate::scene_collision::nav_grid_find_path(g, t.sam, *p, None, None).is_none() {
                bad.push(format!("{}, {}: no way on foot from the player's mark", start, name));
            }
        }
        for r in &s.rooms {
            if !r.marks.is_empty() && !blocked(g, t.sam[0], t.sam[2]) && !blocked(g, r.marks[0].pos[0], r.marks[0].pos[2]) && crate::scene_collision::nav_grid_find_path(g, t.sam, r.marks[0].pos, None, None).is_none() {
                bad.push(format!("{}: no way on foot from {}", r.name, its));
            }
        }
    }
    let men = s.rooms.iter().fold(0.0, |n, r| n + r.garrison);
    let most = crate::scene_scenarios::SQUAD_MAX_MEN;
    match s.max_men.filter(|x| js::truthy(*x)) {
        // a structure made for more: the staging lifts the four-man squad's budget to fit
        Some(mm) if men <= m::min(most, mm) => {}
        Some(mm) if men > 5.0 => {
            let more = if mm > most { format!(" and at most {} fit beside the player and the four (sixteen characters are skinned)", js::num_str(most)) } else { String::new() };
            bad.push(format!("{} defenders: the structure's maxMen is {}{}", js::num_str(men), js::num_str(mm), more));
        }
        _ => {
            if men > 5.0 {
                bad.push(format!("{} defenders: only five fit beside the player and the pair (eight characters cast shadows)", js::num_str(men)));
            }
        }
    }
    let squads = crate::roomclear::CheckSquadsS { rooms: s.rooms.clone(), max_men: s.max_men, squads: s.squads.clone() };
    let of_squads = check_squads(g, &squads);
    bad.extend(of_squads);
    if !s.rooms.iter().any(|r| r.doors.len() > 1) {
        bad.push(String::from("no room has more than one way in"));
    }
    for r in &s.rooms {
        if r.garrison < 1.0 {
            bad.push(format!("{}: nobody holds it", r.name));
        }
        if (r.marks.len() as f64) < r.garrison {
            bad.push(format!("{}: {} marks for {} men", r.name, r.marks.len(), js::num_str(r.garrison)));
        }
        for d in &r.doors {
            if crate::doors::doors_by_name(g, &d.id).is_none() {
                bad.push(format!("{}: no door '{}' in the level", r.name, d.id));
            }
            // (a plan is for the room its door is entered INTO; the one door two rooms share has none)
            if let Some(b) = door_plan(g, &d.id).and_then(|p| p.bounds) {
                let b = crate::scene_scenarios::Rect { x0: b.x0, x1: b.x1, z0: b.z0, z1: b.z1 };
                if !same_bounds(b, r.bounds) {
                    bad.push(format!("{}: its bounds are not the level's clear plan's for '{}'", r.name, d.id));
                }
            }
        }
        for (i, mk) in r.marks.iter().enumerate() {
            let at = format!("{} mark {} ({}, {})", r.name, i, js::num_str(mk.pos[0]), js::num_str(mk.pos[2]));
            if !in_rect(mk.pos, r.bounds) {
                bad.push(format!("{}: outside the room", at));
            }
            if blocked(g, mk.pos[0], mk.pos[2]) {
                bad.push(format!("{}: not on floor a man can stand on", at));
            }
            let mut prop: Option<(f64, String)> = None;
            for k in 0..g.props.props.len() {
                let row = &g.tab.props_prop[g.props.props[k]];
                let gap = prop_gap(crate::roomclear::PropGapP { home: row.home, hx: row.hx, hz: row.hz }, mk.pos);
                if gap < MARK_CLEAR_M {
                    prop = Some((gap, row.def.name.clone()));
                    break;
                }
            }
            if let Some((gap, name)) = prop {
                bad.push(format!("{}: {} m from {} — he would stand in it", at, js::to_fixed(gap, 2), name));
            }
            if !r.doors.iter().any(|d| d.id == mk.watch) {
                bad.push(format!("{}: watches '{}', which is no door of this room", at, mk.watch));
            }
            if wall_between(g, [mk.pos[0], 1.6, mk.pos[2]], mk.aim_at) {
                bad.push(format!("{}: no line from his eyes to the point he covers", at));
            }
        }
    }
    bad
}

/// The squads block against the floor. Empty = sound; no squads block = nothing to say.
fn check_squads(g: &mut crate::game::Game, s: &crate::roomclear::CheckSquadsS) -> Vec<String> {
    let squads = match &s.squads {
        Some(q) if !q.is_empty() => q,
        _ => return Vec::new(),
    };
    let mut bad: Vec<String> = Vec::new();
    let mut owner: JsMap<String, String> = JsMap::new();
    let mut ids: JsSet<String> = JsSet::new();
    let men = s.rooms.iter().fold(0.0, |n, r| n + r.garrison);
    let cap = s.max_men.unwrap_or(5.0);
    for q in squads {
        let w = format!("squad '{}'", q.id);
        if ids.has(q.id.clone()) {
            bad.push(format!("{}: two squads share the id", w));
        }
        ids.add(q.id.clone());
        let mut mine: Vec<&crate::scene_scenarios::StructureRoom> = Vec::new();
        for id in q.rooms.iter().flatten() {
            let r = s.rooms.iter().find(|x| x.id == *id);
            match r {
                None => bad.push(format!("{}: no room '{}' in the structure", w, id)),
                Some(r) => {
                    if let Some(was) = owner.get(id.clone()) {
                        bad.push(format!("{}: room '{}' is already squad '{}'s", w, id, was));
                    } else {
                        owner.set(id.clone(), q.id.clone());
                    }
                    mine.push(r);
                }
            }
        }
        let late = q.arrive.is_some();
        if if late { !mine.is_empty() } else { mine.is_empty() } {
            bad.push(if late { format!("{}: a late squad holds no rooms", w) } else { format!("{}: holds no room", w) });
        }
        if let Some(rally) = q.rally {
            if blocked(g, rally[0], rally[2]) {
                bad.push(format!("{}: its rally point ({}, {}) is not on floor a man can stand on", w, js::num_str(rally[0]), js::num_str(rally[2])));
            } else if !late && !mine.iter().any(|r| in_rect(rally, r.bounds)) {
                bad.push(format!("{}: its rally point ({}, {}) is inside none of its rooms", w, js::num_str(rally[0]), js::num_str(rally[2])));
            }
        }
        for id in q.barricade.iter().flatten() {
            if !mine.iter().any(|r| r.doors.iter().any(|d| d.id == *id)) {
                bad.push(format!("{}: '{}' is no door of its rooms — it cannot wedge it", w, id));
            }
        }
        let a = match &q.arrive {
            Some(a) => a,
            None => continue,
        };
        let door = crate::doors::doors_by_name(g, &a.door);
        if door.is_none() {
            bad.push(format!("{}: no door '{}' in the level to arrive by", w, a.door));
        }
        if !(a.men >= 1.0) || (a.spawn.len() as f64) < a.men {
            bad.push(format!("{}: {} spawn points for {} men", w, a.spawn.len(), js::num_str(a.men)));
        }
        if a.men > cap {
            bad.push(format!("{}: {} men can never fit maxMen {}", w, js::num_str(a.men), js::num_str(cap)));
        }
        for (i, p) in a.spawn.iter().enumerate() {
            let p = *p;
            let at = format!("{} spawn {} ({}, {})", w, i, js::num_str(p[0]), js::num_str(p[2]));
            if let Some(inside) = s.rooms.iter().find(|r| in_rect(p, r.bounds)) {
                bad.push(format!("{}: inside the {} — a late squad starts outside every room", at, inside.name));
            }
            if blocked(g, p[0], p[2]) {
                bad.push(format!("{}: not on floor a man can stand on", at));
            } else if let Some(d) = door {
                let mut joined = false;
                for side in door_sides(g, d) {
                    if !blocked(g, side[0], side[2]) && crate::scene_collision::nav_grid_find_path(g, p, side, None, None).is_some() {
                        joined = true;
                        break;
                    }
                }
                if !joined {
                    bad.push(format!("{}: no way on foot to '{}'", at, a.door));
                }
            }
        }
    }
    for r in &s.rooms {
        if !owner.has(r.id.clone()) {
            bad.push(format!("the {} is in no squad (with a squads block every room is in exactly one)", r.name));
        }
    }
    if men > cap {
        bad.push(format!("{} men in the rooms' garrisons: more than maxMen {}", js::num_str(men), js::num_str(cap)));
    }
    bad
}

/// the floor a stride either side of a door's opening
fn door_sides(g: &crate::game::Game, d: DoorId) -> Vec<V3> {
    let f = crate::guards::frame_of(g, d);
    let cx = f.hx + f.ux * f.w / 2.0;
    let cz = f.hz + f.uz * f.w / 2.0;
    vec![[cx + f.nx * 0.9, 0.0, cz + f.nz * 0.9], [cx - f.nx * 0.9, 0.0, cz - f.nz * 0.9]]
}

/// the same four numbers
fn same_bounds(a: crate::scene_scenarios::Rect, b: crate::scene_scenarios::Rect) -> bool {
    a.x0 == b.x0 && a.x1 == b.x1 && a.z0 == b.z0 && a.z1 == b.z1
}

/// a floor point's distance to a prop's footprint where it RESTS (0 = inside it)
pub fn prop_gap(p: crate::roomclear::PropGapP, at: V3) -> f64 {
    let dx = at[0] - p.home[0];
    let dz = at[2] - p.home[1];
    let c = m::cos(p.home[2]);
    let s = m::sin(p.home[2]);
    m::hyp2(m::max(0.0, m::abs(c * dx - s * dz) - p.hx), m::max(0.0, m::abs(s * dx + c * dz) - p.hz))
}

/// the sight line a→b against the level's STATICS alone
pub fn wall_between(g: &mut crate::game::Game, a: V3, b: V3) -> bool {
    crate::scene_collision::segment_blocked_over(g, &crate::ask::Over::statics(), a, b, None, None)
}

/// the level's own stack-and-enter plan for a door, where it has one
pub fn door_plan(g: &crate::game::Game, door: &str) -> Option<crate::scene_level::RoomClearPlan> {
    if let Some(p) = g.level.clear_plans.as_ref().and_then(|v| v.iter().find(|p| p.door == door)) {
        return Some(p.clone());
    }
    match &g.level.breach {
        Some(b) if b.door == door => Some(plan_of_breach(b)),
        _ => None,
    }
}
/// a BreachPlan is a RoomClearPlan with more on it: the part of it doorPlan's callers read
fn plan_of_breach(b: &crate::scene_level::BreachPlan) -> crate::scene_level::RoomClearPlan {
    crate::scene_level::RoomClearPlan {
        door: b.door.clone(),
        room: b.room.clone(),
        breach: b.breach.map(|k| match k {
            crate::scene_level::BreachPlanBreach::Kick => crate::scene_level::RoomClearPlanBreach::Kick,
            crate::scene_level::BreachPlanBreach::Open => crate::scene_level::RoomClearPlanBreach::Open,
        }),
        bounds: b.bounds,
        from: b.from,
        stack: b.stack,
        stack_face: b.stack_face,
        stack_side: b.stack_side,
        stack_aim: b.stack_aim,
        kick_from: b.kick_from,
        kick_face: b.kick_face,
        fling: b.fling,
        entry: b.entry.clone(),
        pod_face: b.pod_face,
        entry_aim: b.entry_aim.clone(),
        search_path: b.search_path.clone(),
        search_face: b.search_face,
        search_aim_walk: b.search_aim_walk.clone(),
        search_aim_there: b.search_aim_there.clone(),
        under_aim: b.under_aim.clone(),
        up_aim: b.up_aim,
        form_up: b.form_up.clone(),
        form_face: b.form_face,
        lines: b.lines.clone(),
    }
}

/// the structure of the level this game stands on
pub fn room_clear_scenario_structure(g: &crate::game::Game) -> Option<crate::scene_scenarios::StructureData> {
    g.level.scenarios.as_ref().and_then(|c| c.room_clear.clone())
}

/// where the staging stood the first player: the start of the way taken, else the team's (None: not up)
pub fn room_clear_scenario_start_at(g: &crate::game::Game) -> Option<V3> {
    if !room_clear_scenario_active(g) { return None; }
    let s = room_clear_scenario_structure(g)?;
    let way = g.room_clear.way;
    let by_way = if way > 0.0 { s.starts.as_ref().and_then(|v| v.get(way as usize - 1)).cloned() } else { None };
    by_way.or(s.team).map(|t| t.sam)
}

/// THE WAY IN A LINK'S `way` ASKS FOR: 0 the level's own, k its k-th other. `way=N` is the level's Nth; `way=1`, no word and a word that is no number are its own. Any level: past its last is `way_taken`'s.
pub fn way_asked(word: Option<&str>) -> usize {
    let Some(w) = word.filter(|w| !w.is_empty() && w.bytes().all(|c| c.is_ascii_digit())) else { return 0 };
    w.parse::<u32>().unwrap_or(u32::MAX).saturating_sub(1) as usize
}
/// …of a whole address: only a room clear has ways. A rescue, the night breach, a quiet mission and the solo night have the level's one start, whatever `way` the link still carries.
pub fn way_of_address(q: &crate::query::Query) -> usize {
    if crate::scene_scenario_list::kind_of(q) == Some(crate::scene_scenario_list::ScenarioSpecKind::Roomclear) { way_asked(q.get("way")) } else { 0 }
}
/// THE WAY TAKEN of the `ways` a level has, its own among them: past the last = the last.
pub fn way_taken(asked: usize, ways: usize) -> usize { asked.min(ways.saturating_sub(1)) }

/// the way in a staging made now would take: the one the address asked for, on this level
fn room_clear_scenario_way_now(g: &mut crate::game::Game) -> f64 {
    let asked = g.stood.way;
    let s = room_clear_scenario_structure(g);
    let n = match &s {
        Some(s) if s.team.is_some() => 1 + s.starts.as_ref().map_or(0, |v| v.len()),
        _ => 1,
    };
    way_taken(asked, n) as f64
}

/// the team start a staging made now stands the team on
pub(super) fn room_clear_scenario_start_now(g: &mut crate::game::Game) -> Option<crate::scene_scenarios::TeamStart> {
    let s = room_clear_scenario_structure(g);
    let k = room_clear_scenario_way_now(g);
    let s = s?;
    if k > 0.0 {
        if let Some(t) = s.starts.as_ref().and_then(|v| v.get(k as usize - 1)) {
            return Some(t.clone());
        }
    }
    s.team
}

/// the scenario, its layout named
pub fn room_clear_scenario_new(g: &crate::game::Game) -> crate::roomclear::RoomClearScenario {
    crate::roomclear::RoomClearScenario::blank()
}

/// the layout's budget getter: a structure with maxMen holds Sam, the four and its men
pub(super) fn room_clear_scenario_budget(g: &crate::game::Game) -> Option<f64> {
    let n = match room_clear_scenario_structure(g) {
        Some(s) if s.max_men.map_or(false, js::truthy) => s.rooms.iter().fold(0.0, |k, r| k + r.garrison),
        _ => 0.0,
    };
    if n > 5.0 { Some(crate::seats::budget_with_seats(g, 1.0 + 4.0 + n)) } else { None }   // (a man a seat, the four and its men)
}

/// up: the night breach is staged on THIS scenario's floor
pub fn room_clear_scenario_active(g: &crate::game::Game) -> bool {
    // (`nb.layout === this.layout`: this scenario's is the only layout there is)
    g.night_breach.active && g.night_breach.layout.is_some()
}

/// why it cannot start right now (None = it can)
fn room_clear_scenario_can_start(g: &mut crate::game::Game) -> Option<String> {
    if g.night_breach.active {
        return Some(String::from("the night breach is already staged — stop it first"));
    }
    let s = match room_clear_scenario_structure(g) {
        Some(s) => s,
        None => return Some(String::from("this level has no room clear structure (its scenarios.roomClear block)")),
    };
    let layout = g.room_clear.layout.clone();
    if let Some(why) = crate::nightbreach::night_breach_can_start(g, Some(&layout)) {
        return Some(why);
    }
    check_structure(g, Some(&check_s_of(&s))).into_iter().next()
}

/// Start it (`seed`: the placement to deal, written to the knob). Returns the reason when refused, or when the staging failed.
pub(super) fn room_clear_scenario_start(g: &mut crate::game::Game, seed: Option<f64>) -> Option<String> {
    let want = match seed {
        Some(x) if x.is_finite() => Some(m::floor(x)),
        _ => None,
    };
    if room_clear_scenario_active(g) {
        if let Some(want) = want {
            g.k.roomclear.seed = want;
            g.room_clear.fresh = 0.0;
            // (against the seed ON THE FLOOR, not the knob)
            if want != g.room_clear.seed || room_clear_scenario_way_now(g) != g.room_clear.way {
                room_clear_scenario_restage(g);
            }
        }
        return None;
    }
    let seed0 = g.k.roomclear.seed;
    let fresh0 = g.room_clear.fresh;
    if let Some(want) = want {
        g.k.roomclear.seed = want;
    }
    g.room_clear.fresh = match (want, g.dice.as_ref()) {
        (None, Some(d)) => fresh_seed_of(d.seed),
        _ => 0.0,
    };
    if let Some(why) = room_clear_scenario_can_start(g) {
        g.k.roomclear.seed = seed0;
        g.room_clear.fresh = fresh0;
        return Some(why);
    }
    g.room_clear.attempt = 0.0;
    let name = room_clear_scenario_structure(g).map_or(String::new(), |s| s.name);
    crate::difficulty::difficulty_here(g, &name);
    let layout = g.room_clear.layout.clone();
    let r = crate::nightbreach::night_breach_start(g, Some(&layout));
    if r.is_some() || !room_clear_scenario_active(g) {
        room_clear_scenario_clear(g);
        return Some(r.unwrap_or_else(|| String::from("the staging failed")));
    }
    None
}

/// Stage it afresh
pub fn room_clear_scenario_restage(g: &mut crate::game::Game) {
    if room_clear_scenario_active(g) {
        crate::nightbreach::night_breach_restage(g);
    }
}

/// the score wiped
fn room_clear_scenario_clear(g: &mut crate::game::Game) {
    crate::enemysquads::squads_stop(g);
    let s = &mut g.room_clear;
    s.rooms = Vec::new();
    s.placement = Vec::new();
    s.end = None;
    s.end_at = f64::NAN;
    s.end_why = String::new();
    s.end_say = String::from("team wiped");
    s.pair_said = false;
    s.entry_said = false;
    s.team_said = false;
}

/// what happened when, on the game clock
fn room_clear_scenario_note(g: &crate::game::Game, what: &str) {
}

/// the night breach's staging asks who the defenders are: every staging is the knob's placement, first attempt (SAMESTART)
pub(super) fn room_clear_scenario_deal(g: &mut crate::game::Game) -> Vec<crate::nightbreach::NightBreachMark> {
    g.room_clear.way = room_clear_scenario_way_now(g);
    let seed = g.k.roomclear.seed;
    g.room_clear.seed = seed;
    g.room_clear.attempt = 1.0;
    let rooms = room_clear_scenario_structure(g).expect("the room clear dealt on a level with no structure").rooms;
    let placement = place_defenders(seed, Some(1.0), &crate::roomclear::PlaceDefendersS { rooms });
    let drill = room_clear_scenario_drill_room(g).map(|r| r.id);
    let marks = placement
        .iter()
        .map(|p| crate::nightbreach::NightBreachMark { pos: p.pos, aim_at: p.aim_at, name: Some(p.name.clone()), drill: Some(drill.as_ref() == Some(&p.room)) })
        .collect();
    g.room_clear.placement = placement;
    marks
}

/// the room behind the breach plan's door
fn room_clear_scenario_drill_room(g: &crate::game::Game) -> Option<crate::scene_scenarios::StructureRoom> {
    let door = g.level.breach.as_ref()?.door.clone();
    room_clear_scenario_structure(g)?.rooms.into_iter().find(|r| r.doors.iter().any(|d| d.id == door))
}

/// … and tells it the cast stands: every way in shut, each room given its men, the score wiped
pub(super) fn room_clear_scenario_on_staged(g: &mut crate::game::Game) {
    let s = room_clear_scenario_structure(g).expect("the room clear staged on a level with no structure");
    let mut ways: JsSet<String> = JsSet::new();
    for r in &s.rooms {
        for d in &r.doors {
            ways.add(d.id.clone());
        }
    }
    for id in ways.keys() {
        if let Some(d) = crate::doors::doors_by_name(g, &id) {
            crate::doors::door_reset(g, d);
            crate::doors::door_set_angle(g, d, 0.0);
            crate::doors::door_set_vel(g, d, 0.0);
            crate::doors::door_set_latched(g, d, true);
            crate::doors::door_set_closing(g, d, false);
        }
    }
    let dealt = core::mem::take(&mut g.room_clear.placement);
    room_clear_scenario_clear(g);
    let dealt_rooms: Vec<String> = dealt.iter().map(|p| p.room.clone()).collect();
    g.room_clear.placement = dealt;
    let hostiles = g.night_breach.hostiles.clone();
    g.room_clear.rooms = s
        .rooms
        .iter()
        .map(|def| crate::roomclear::RoomState {
            def: def.clone(),
            defenders: hostiles.iter().enumerate().filter(|(i, _)| dealt_rooms.get(*i) == Some(&def.id)).map(|(_, h)| *h).collect(),
            cleared: false,
            cleared_at: f64::NAN,
        })
        .collect();
    let lesson = g.level.scenarios.as_ref().map_or(false, |c| c.lesson.is_some());
    let cut = crate::squaddeal::squad_deal(g, Some(&s), &dealt_rooms, Some(lesson));
    crate::enemysquads::squads_stage(g, cut.as_ref(), &hostiles, &dealt_rooms);
    let line = format!("staged:{}/{}", js::num_str(g.room_clear.seed), js::num_str(g.room_clear.attempt));
    room_clear_scenario_note(g, &line);
}

/// Once a frame, right after the night breach's own tick: the rooms' state and the two ends
pub fn room_clear_scenario_tick(g: &mut crate::game::Game, _dt: f64) {
    if !room_clear_scenario_active(g) {
        if !g.room_clear.rooms.is_empty() {
            room_clear_scenario_clear(g);
        }
        return;
    }
    if g.room_clear.rooms.is_empty() || g.night_breach.hostiles.len() != g.room_clear.placement.len() {
        return;
    }
    crate::enemysquads::squads_tick(g);
    let live: Vec<GuardId> = g.night_breach.hostiles.iter().copied().filter(|h| holds(g, *h)).collect();
    let mut i = 0;
    while i < g.room_clear.rooms.len() {
        let bounds = g.room_clear.rooms[i].def.bounds;
        let c = !g.room_clear.rooms[i].defenders.iter().any(|h| holds(g, *h)) && !live.iter().any(|h| g.gd[*h].held.is_none() && in_rect(g.ch[g.gd[*h].ch].pos, bounds));
        if c == g.room_clear.rooms[i].cleared {
            i += 1;
            continue;
        }
        let now = g.time;
        g.room_clear.rooms[i].cleared = c;
        g.room_clear.rooms[i].cleared_at = if c { now } else { f64::NAN };
        let id = g.room_clear.rooms[i].def.id.clone();
        room_clear_scenario_note(g, &format!("{}:{}", if c { "cleared" } else { "lost" }, id));
        i += 1;
    }
    let pair_up = ops_up(g);
    let ops = g.night_breach.ops.len();
    let play = g.room_clear.end.is_none() && !crate::seats::all_down(g) && !live.is_empty();
    if !g.room_clear.pair_said && pair_up == 0 && ops > 0 {
        g.room_clear.pair_said = true;
        room_clear_scenario_note(g, "pair:down");
    }
    if !g.room_clear.entry_said && crate::nightbreach::night_breach_state(g) == crate::nightbreach::NightBreachState::Failed {
        g.room_clear.entry_said = true;
        if pair_up > 0 && play {
            let line = crate::nightbreach::night_breach_down_line(g);
            crate::squadnet::team_says(g, None, &line);
        }
    }
    if g.room_clear.end.is_some() {
        return;
    }
    if crate::seats::all_down(g) {
        // (first: the last defender and Sam down inside one frame is no win)
        room_clear_scenario_finish(g, crate::roomclear::RoomClearEnd::Wiped, "player down");
    } else if crate::rails::rails_ends_now(g) {
        room_clear_scenario_finish(g, crate::roomclear::RoomClearEnd::Taken, "the mission is done");
    } else if crate::rails::rails_holds_end(g) {
        // (the mission is still on its steps: its end is theirs)
    } else if live.is_empty() && !crate::qrf::response_up(g) {
        let team = if ops > 0 { format!("{} of {} teammates up", pair_up, ops) } else { String::from("alone") };
        let why = format!("{} rooms clear, {}", g.room_clear.rooms.len(), team);
        room_clear_scenario_finish(g, crate::roomclear::RoomClearEnd::Taken, &why);
    } else if live.is_empty() && !g.room_clear.team_said {
        g.room_clear.team_said = true;
        room_clear_scenario_note(g, "team-up");
    }
}

/// A man the night breach adopted after the staging is this structure's too
pub fn room_clear_scenario_adopt(g: &mut crate::game::Game, gd: GuardId, p: &crate::roomclear::Placement) {
    let r = g.room_clear.rooms.iter().position(|r| r.def.id == p.room);
    if !room_clear_scenario_active(g) {
        return;
    }
    let r = match r {
        Some(r) => r,
        None => return,
    };
    if g.room_clear.rooms[r].defenders.contains(&gd) {
        return;
    }
    g.room_clear.placement.push(p.clone());
    g.room_clear.rooms[r].defenders.push(gd);
}

/// an end, and what it is called
fn room_clear_scenario_finish(g: &mut crate::game::Game, end: crate::roomclear::RoomClearEnd, why: &str) {
    use crate::roomclear::RoomClearEnd::Wiped;
    g.room_clear.end = Some(end);
    g.room_clear.end_at = g.time;
    g.room_clear.end_why = String::from(why);
    room_clear_scenario_note(g, &format!("end:{}", end.as_str()));
    let down = end == Wiped && crate::seats::all_down(g) && ops_up(g) > 0;
    g.room_clear.end_say = String::from(if down { "player down" } else { "team wiped" });
}

/// the room a floor point is in
pub fn room_clear_scenario_room_at(g: &crate::game::Game, p: V3) -> Option<String> {
    room_clear_scenario_structure(g)?.rooms.into_iter().find(|r| in_rect(p, r.bounds)).map(|r| r.id)
}

/// The mission's line while it is up (None: not its to say): the rooms, ticked off as they clear, and a marker on the first still held. A read, worded as
/// things stand each time it is asked.
pub(super) fn room_clear_scenario_objective(g: &crate::game::Game, seat: &crate::seats::SeatRef) -> Option<crate::roomclear::RoomClearScenarioObjective> {
    if !room_clear_scenario_active(g) || g.room_clear.rooms.is_empty() {
        return None;
    }
    let hint = if g.room_clear.end.is_some() { None } else { crate::laststand::last_stand_hint(g) };
    let rooms = &g.room_clear.rooms;
    let said = |r: &crate::roomclear::RoomState| hint.as_ref().map_or(false, |h| !r.cleared && h.rooms.has(r.def.id.clone()));
    let spots: &[crate::laststand::Spot] = hint.as_ref().map_or(&[], |h| &h.spots[..]);
    let next = match rooms.iter().find(|r| said(r)) {
        Some(r) => Some(r.def.bounds),
        None if !spots.is_empty() => None,
        None => room_clear_scenario_first_open(g).map(|r| r.def.bounds),
    };
    let spot = if next.is_some() { None } else { spots.first() };
    let end = g.room_clear.end;
    let text = match end {
        Some(crate::roomclear::RoomClearEnd::Taken) => String::from("structure taken — press shift+enter to restart"),
        Some(crate::roomclear::RoomClearEnd::Wiped) => {
            let key = if crate::condition::down(g, g.pl[seat.player].ch) { "enter" } else { "shift+enter" };
            format!("{} — press {} to restart", g.room_clear.end_say, key)
        }
        None => {
            let names: Vec<String> = rooms
                .iter()
                .map(|r| {
                    let tick = if r.cleared { "✓ " } else { "" };
                    if said(r) { format!("{}[{}]", tick, r.def.name) } else { format!("{}{}", tick, r.def.name) }
                })
                .collect();
            let mut text = format!("objective ▸ clear the structure: {}", names.join(" · "));
            for s in spots {
                text.push_str(&format!(" · [{}]", s.name));
            }
            if g.room_clear.team_said {
                let team = crate::qrf::response_team(g);
                text.push_str(&format!(" · [{}]", team));
            }
            text
        }
    };
    let pos: Option<V3> = if end.is_some() {
        None
    } else if let Some(b) = next {
        Some([(b.x0 + b.x1) / 2.0, 0.0, (b.z0 + b.z1) / 2.0])
    } else {
        spot.map(|s| [s.pos[0], 0.0, s.pos[2]])
    };
    Some(crate::roomclear::RoomClearScenarioObjective { text, pos })
}

/// the room the marker stands on: the first still held, on a second way in the one nearest that start
fn room_clear_scenario_first_open(g: &crate::game::Game) -> Option<crate::roomclear::RoomState> {
    let open: Vec<crate::roomclear::RoomState> = g.room_clear.rooms.iter().filter(|r| !r.cleared).cloned().collect();
    let way = g.room_clear.way;
    let t = if way > 0.0 {
        room_clear_scenario_structure(g).and_then(|s| s.starts).and_then(|v| v.into_iter().nth(way as usize - 1))
    } else {
        None
    };
    let t = match t {
        Some(t) if open.len() >= 2 => t,
        _ => return open.into_iter().next(),
    };
    let d = |r: &crate::roomclear::RoomState| {
        let b = r.def.bounds;
        m::hyp2((b.x0 + b.x1) / 2.0 - t.sam[0], (b.z0 + b.z1) / 2.0 - t.sam[2])
    };
    let mut best = 0;
    for k in 1..open.len() {
        if d(&open[k]) < d(&open[best]) {
            best = k;
        }
    }
    open.into_iter().nth(best)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn room(id: &str, garrison: f64, marks: usize) -> crate::scene_scenarios::StructureRoom {
        let mut r = crate::scene_scenarios::StructureRoom::default();
        r.id = String::from(id);
        r.name = String::from(id);
        r.garrison = garrison;
        for i in 0..marks {
            let mut mk = crate::scene_scenarios::StructureMark::default();
            mk.pos = [i as f64 + 0.5, 0.0, i as f64 * 2.0 + 0.25];
            mk.aim_at = [i as f64, 1.1, -(i as f64)];
            mk.watch = format!("d{}", i);
            r.marks.push(mk);
        }
        r
    }

    /// bun on the TypeScript's placeDefenders, EXACT on: room, mark and Tango number of each man
    #[test]
    fn the_deal_is_the_typescripts() {
        let s = crate::roomclear::PlaceDefendersS { rooms: vec![room("a", 2.0, 5), room("b", 1.0, 3), room("c", 3.0, 3), room("d", 3.0, 7)] };
        let want: [(f64, f64, &str); 8] = [
            (0.0, 1.0, "a01,a12,b03,c04,c15,c26,d07,d18,d29"),
            (0.0, 2.0, "a11,a22,b13,c04,c15,c26,d17,d28,d59"),
            (1.0, 1.0, "a21,a32,b03,c04,c15,c26,d17,d38,d59"),
            (7.0, 3.0, "a11,a42,b13,c04,c15,c26,d07,d48,d69"),
            (-5.0, 2.0, "a11,a22,b23,c04,c15,c26,d17,d58,d69"),
            (9973.0, 1.0, "a01,a22,b23,c04,c15,c26,d07,d28,d69"),
            (123456789.0, 4.0, "a11,a42,b13,c04,c15,c26,d27,d38,d49"),
            (3.9, 1.5, "a11,a42,b23,c04,c15,c26,d07,d18,d39"),
        ];
        for (seed, attempt, line) in want {
            let got: Vec<String> = place_defenders(seed, Some(attempt), &s).iter().map(|p| format!("{}{}{}", p.room, p.mark, &p.name[6..])).collect();
            assert_eq!(got.join(","), line, "seed {} attempt {}", seed, attempt);
        }
    }

    /// bun on the TypeScript's freshSeedOf
    #[test]
    fn the_fresh_readings_are_the_typescripts() {
        let want: [(f64, f64); 7] = [(0.0, 1.0), (1.0, 2.0), (9972.0, 9973.0), (9973.0, 1.0), (4294967296.0, 1.0), (-1.0, 5089.0), (3.7e9, 7028.0)];
        for (x, seed) in want {
            assert_eq!(fresh_seed_of(x), seed);
        }
    }

    /// bun on the TypeScript's propGap, EXACT on: the doubles' bits
    #[test]
    fn the_gap_is_the_typescripts_bits() {
        let cases: [([f64; 3], f64, f64, V3, u64); 3] = [
            ([1.5, -2.25, 0.7], 0.4, 0.9, [3.1, 0.0, 0.2], 0x40000971d6510133),
            ([0.0, 0.0, 0.0], 0.5, 0.5, [0.1, 0.0, 0.1], 0),
            ([10.125, 4.0, 2.356], 1.2, 0.3, [12.0, 0.0, 6.5], 0x3ffe6214913a134b),
        ];
        for (home, hx, hz, at, bits) in cases {
            assert_eq!(prop_gap(crate::roomclear::PropGapP { home, hx, hz }, at).to_bits(), bits);
        }
    }
}
