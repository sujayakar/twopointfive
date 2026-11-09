pub use ::game_types::nightbreach::types;
pub use types::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{format, string::String, vec, vec::Vec};

// ---- private helpers

/// The four-man squad's budget of characters: the seats' men, the four and the layout's five defenders (nightbreach.ts SQUAD4_BUDGET: ten in a game of one).
pub(crate) fn squad4_budget(g: &crate::game::Game) -> f64 { crate::seats::budget_with_seats(g, SQUAD4_BUDGET) }

/// `T !== applied`, an identity of the level's own rows (a structure's team start, one of its other ways in, the night breach's own): here the row
/// whole, by value. Another start with the same place for Sam is ANOTHER start in the TypeScript and writes the knobs again (a knob moved by hand
/// since is put back), so every field is asked, not only the two the write copies. Two rows of one level alike in every field would still read as
/// one: that wants the row's place kept beside `applied`, which the generated type has no field for.
fn same_team(a: &crate::scene_scenarios::TeamStart, b: &crate::scene_scenarios::TeamStart) -> bool {
    a.sam == b.sam && a.sam_yaw == b.sam_yaw && a.ops == b.ops && a.ops_yaw == b.ops_yaw
}

/// `layout?.team`: the one layout is the room clear's, and its team is a getter asked at each read.
fn layout_team(g: &mut Game, layout: Option<&NightBreachLayout>) -> Option<crate::scene_scenarios::TeamStart> {
    if layout.is_some() { crate::roomclear::room_clear_scenario_start_now(g) } else { None }
}

/// `ordersOf(g, gd).has(gd)`: orders_of answers whose book it is, and the has is asked over that man's seat.
fn in_his_book(g: &mut Game, gd: GuardId) -> bool {
    let man = crate::seats::orders_of(g, gd);
    let s = crate::seats::seat_of_man(g, man);
    crate::seats::with_seat(g, s, &mut |v| crate::orders::squad_orders_has(v, &crate::seats::here(v), gd))
}

/// Teammate i's mark; Err is the TypeScript's throw.
fn mark_of(
    own: Option<&crate::scene_scenarios::TeamStart>,
    plan: Option<&crate::scene_level::BreachPlan>,
    team: Option<&crate::scene_scenarios::TeamStart>,
    i: usize,
) -> Result<V3, String> {
    let own = own.and_then(|s| s.ops.as_ref()).and_then(|o| o.get(i).copied());
    let from = plan.and_then(|p| p.from.as_ref()).and_then(|f| f.get(i).copied());
    let stack = plan.and_then(|p| p.stack.get(i).copied());
    let start = team.and_then(|s| s.ops.as_ref()).and_then(|o| o.get(i).copied());
    own.or(from).or(stack).or(start).ok_or_else(|| format!("no mark for teammate {} (the level's team start, or its breach plan)", i + 1))
}

/// The search of standBy; Err is the TypeScript's throw.
fn stand_by_checked(g: &mut Game, lead: V3, taken: &[V3], yaw: f64) -> Result<V3, String> {
    let mut best: Option<V3> = None;
    let mut bs = f64::INFINITY;
    let want: V3 = [lead[0] - 1.2 * m::sin(yaw), 0.0, lead[2] - 1.2 * m::cos(yaw)];
    let mut r = 0.0;
    while r <= 3.01 {
        let turns = if js::truthy(r) { 12 } else { 1 };
        for k in 0..turns {
            let a = (k as f64) * m::PI / 6.0;
            let q: V3 = [want[0] + m::sin(a) * r, 0.0, want[2] + m::cos(a) * r];
            if crate::scene_collision::nav_grid_is_blocked(g, q[0], q[2])
                || taken.iter().any(|t| v3::dist_xz(*t, q) < 0.9)
                || crate::scene_collision::solid_blocked(g, [lead[0], 0.45, lead[2]], [q[0], 0.45, q[2]], None)
            {
                continue;
            }
            let s = r + 0.1 * v3::dist_xz(q, lead);
            if s < bs {
                bs = s;
                best = Some(q);
            }
        }
        if best.is_some() { break; }
        r += 0.3;
    }
    best.ok_or_else(|| String::from("no free floor behind the approach marks for the second pair"))
}

/// The staging whole; Err is what the TypeScript's stage throws to onEncounterRestart.
fn stage_checked(g: &mut Game) -> Result<(), String> {
    let layout = g.night_breach.layout.clone();
    night_breach_apply_team(g, layout.as_ref());
    let plan = g.level.breach.clone();
    let team = night_breach_team_of(g, layout.as_ref());
    let own_team = layout_team(g, layout.as_ref());
    if g.night_breach.drill.is_some() { crate::breach::breach_drill_end(g); }
    g.night_breach.drill = None;
    g.night_breach.ops = Vec::new();
    g.night_breach.hostiles = Vec::new();

    let n = g.k.nightbreach.squad;
    let four = n > 2.0;
    let layout_budget = if layout.is_some() { crate::roomclear::room_clear_scenario_budget(g) } else { None };
    let budget = if four { Some(m::max(squad4_budget(g), layout_budget.unwrap_or(0.0))) } else { layout_budget };
    let yaw = team.as_ref().and_then(|s| s.ops_yaw).unwrap_or(m::PI / 2.0);
    if !four {
        for i in 0..core::cmp::min(2, n as usize) {
            let pos = mark_of(own_team.as_ref(), plan.as_ref(), team.as_ref(), i)?;
            let name = if i == 0 { "Blue One" } else { "Blue Two" };
            let opts = crate::guards::UnitOpts {
                team: crate::teams::Team::Operative,
                pos,
                yaw: Some(yaw),
                name: Some(String::from(name)),
                light: Some(true),
                pair: Some(crate::game::PairId::Blue),
                ..Default::default()
            };
            let man = crate::guards::spawn_unit(g, &opts)?;
            g.night_breach.ops.push(man);
        }
    } else {
        let mut at: Vec<V3> = Vec::new();
        for i in 0..2 { at.push(v3::copy(mark_of(own_team.as_ref(), plan.as_ref(), team.as_ref(), i)?)); }
        let more = own_team.as_ref().and_then(|s| s.ops.clone());
        for i in 0..core::cmp::min(2, (n - 2.0) as usize) {
            let given = more.as_ref().and_then(|o| o.get(2 + i).copied());
            let spot = match given {
                Some(q) => v3::copy(q),
                None => {
                    let mut taken = at.clone();
                    taken.push(g.k.nightbreach.sam_at);
                    stand_by_checked(g, at[i], &taken, yaw)?
                }
            };
            at.push(spot);
        }
        for (i, man) in crate::nightbreach::squad4().iter().take(n as usize).enumerate() {
            let opts = crate::guards::UnitOpts {
                team: crate::teams::Team::Operative,
                pos: at[i],
                yaw: Some(yaw),
                name: Some(man.name.clone()),
                light: Some(true),
                pair: Some(man.pair),
                tint: Some(man.tint),
                tint2: Some(man.tint2),
                budget,
                ..Default::default()
            };
            let spawned = crate::guards::spawn_unit(g, &opts)?;
            g.night_breach.ops.push(spawned);
        }
    }
    if layout.is_none() && plan.is_none() { return Err(String::from("no breach plan")); }
    let marks: Vec<NightBreachMark> = if layout.is_some() {
        crate::roomclear::room_clear_scenario_deal(g)
    } else {
        plan.as_ref().unwrap().hostiles.iter()
            .map(|h| NightBreachMark { pos: h.pos, aim_at: h.aim_at, name: None, drill: None })
            .collect()
    };
    // (THE FLOOR HOLDS SIXTEEN WITH EVERY SEAT COUNTED: the team stands first, then as many defenders, in the order of their marks, as that
    //  leaves room for. A game of one has room for every garrison a level may hold, so it stands as ever; a game of two on a full floor
    //  stands one defender fewer, where it stood seventeen and the picture held sixteen.)
    let marks: Vec<NightBreachMark> = { let room = crate::seats::floor_room(g); marks.into_iter().take(m::max(0.0, room) as usize).collect() };
    g.night_breach.marks = marks.clone();
    for (i, h) in marks.iter().enumerate() {
        let name = match &h.name { Some(s) => s.clone(), None => format!("Tango {}", i + 1) };
        let opts = crate::guards::UnitOpts {
            team: crate::teams::Team::Hostile,
            pos: h.pos,
            yaw: Some(v3::yaw_to(h.pos, h.aim_at)),
            name: Some(name),
            light: Some(false),
            budget,
            ..Default::default()
        };
        let man = crate::guards::spawn_unit(g, &opts)?;
        g.night_breach.hostiles.push(man);
        crate::control::set_hold(g, man, true, "nightbreach");
        crate::control::set_pinned(g, man, true, "nightbreach");
    }
    let live = staged_live();
    g.night_breach.bang_taken = false;
    g.night_breach.bang_by = None;
    g.night_breach.blind = true;
    g.night_breach.bursts = g.night_breach.hostiles.iter().map(|_| BlindBurst { until: -1.0, n: 0.0, next: 0.0 }).collect();
    g.night_breach.blind_rounds = 0.0;
    crate::roomfight::room_fight_reset(g);
    g.fight.blind_fire = true;
    let drilled = plan.is_some() && g.night_breach.ops.len() >= 2;
    g.night_breach.no_drill = !drilled;
    if let Some(p) = plan.as_ref() {
        if !drilled {
            for id in [p.door.clone(), p.next.door.clone()] {
                let d = if !id.is_empty() { crate::doors::doors_by_name(g, &id) } else { None };
                if let Some(d) = d {
                    crate::doors::door_reset(g, d);
                    crate::doors::door_set_angle(g, d, 0.0);
                    crate::doors::door_set_vel(g, d, 0.0);
                    crate::doors::door_set_latched(g, d, true);
                    crate::doors::door_set_closing(g, d, false);
                }
            }
        } else {
            let men: Vec<GuardId> = g.night_breach.hostiles.iter().enumerate()
                .filter(|(i, _)| marks[*i].drill != Some(false))
                .map(|(_, h)| *h)
                .collect();
            let (a, b) = (g.night_breach.ops[0], g.night_breach.ops[1]);
            let drill = crate::breach::breach_drill_new(g, p, a, b, &men, live);
            g.night_breach.drill = Some(drill);
            crate::breach::breach_drill_start(g);
        }
    }
    let standing: Vec<GuardId> = g.night_breach.ops.iter().skip(if drilled { 2 } else { 0 }).copied().collect();
    for gd in standing {
        crate::mind::set_state(g, gd, crate::game::GuardState::Patrol, "nightbreach");
        crate::mind::set(g, gd, 0.0, "nightbreach");
        crate::control::set_hold(g, gd, false, "nightbreach");
        crate::control::set_pinned(g, gd, false, "nightbreach");
        let c = g.gd[gd].ch;
        let patch = crate::squad::OrderPatch {
            goal: Some(None),
            face: Some(Some(g.ch[c].body_yaw)),
            stance: Some(Some(crate::character::Stance::LowReady)),
            upper: Some(Some(crate::character::UpperMode::None)),
            aim_at: Some(None),
            light: Some(Some(false)),
            crouch: Some(Some(false)),
            ..Default::default()
        };
        crate::squad::order(g, gd, patch);
    }
    let (sam_at, sam_yaw) = (g.k.nightbreach.sam_at, g.k.nightbreach.sam_yaw);
    let mut each = |g: &mut Game, pl: PlayerId| {
        if g.pl[pl].has_rifle { g.pl[pl].slot = crate::game::SlotKind::Rifle; }
        g.pl[pl].holstered = false;
    };
    crate::seats::stage_men(g, sam_at, sam_yaw, Some(&mut each));
    g.night_breach.staged += 1.0;
    g.night_breach.beats_seen = 0.0;
    g.night_breach.said.clear();
    g.night_breach.cut = true;
    if layout.is_some() { crate::roomclear::room_clear_scenario_on_staged(g); }
    let men: Vec<crate::gunroles::DealMan> = g.night_breach.hostiles.iter().enumerate()
        .map(|(i, gd)| crate::gunroles::DealMan { gd: *gd, watch: marks[i].aim_at, hostage: None })
        .collect();
    crate::gunroles::gun_deal(g, &men);
    let ops = g.night_breach.ops.clone();
    crate::kit::kit_stage(g, &ops);
    crate::issue::issue_stage(g);
    crate::startfile::team_starts_fallen_in(g, &ops);
    night_breach_note(g, "staged");
    Ok(())
}

/// `live`'s one home is the drill: with no drill it is the staging's fresh one, no orders given.
fn live_of(g: &Game) -> crate::breach::BreachLive {
    g.night_breach.drill.as_ref().map_or_else(staged_live, |d| d.live)
}

/// What a staging opens with, the settled rows at the page's values.
fn staged_live() -> crate::breach::BreachLive {
    crate::breach::BreachLive { orders: 0.0, stun: true }
}

/// `live.orders = n`.
fn live_set_orders(g: &mut Game, orders: f64) {
    if let Some(d) = g.night_breach.drill.as_mut() { d.live.orders = orders; }
}

/// `live.stun = false`.
fn live_stun_off(g: &mut Game) {
    if let Some(d) = g.night_breach.drill.as_mut() { d.live.stun = false; }
}

/// Every man of the list down or fled.
fn all_taken(g: &Game) -> bool {
    for h in g.night_breach.hostiles.clone() {
        if !(crate::condition::down(g, g.gd[h].ch) || g.gd[h].fled) { return false; }
    }
    true
}

/// endGuardScript's word for a man's own state.
fn script_state(s: crate::game::GuardState) -> Option<crate::squad::EndGuardScriptState> {
    match s {
        crate::game::GuardState::Patrol => Some(crate::squad::EndGuardScriptState::Patrol),
        crate::game::GuardState::Suspicious => Some(crate::squad::EndGuardScriptState::Suspicious),
        crate::game::GuardState::Alert => Some(crate::squad::EndGuardScriptState::Alert),
        crate::game::GuardState::Search => Some(crate::squad::EndGuardScriptState::Search),
        crate::game::GuardState::Dead => None,
    }
}

// ---- the skeleton's functions

/// One defender's blind fire this frame; true when a round left the barrel.
pub(super) fn blind_turn(g: &mut crate::game::Game, h: GuardId, k: f64, b: &mut crate::nightbreach::BlindBurst, door: V3) -> bool {
    let bl = crate::nightbreach::blind();
    if crate::condition::down(g, g.gd[h].ch) || !crate::control::man::blind(g, h) { return false; }
    if g.cond[g.gd[h].ch].blind_until > b.until + 1e-6 {
        b.until = g.cond[g.gd[h].ch].blind_until;
        b.n = 0.0;
        b.next = g.time + bl.after + k * bl.stagger;
    }
    if g.gd[h].held.is_none() { crate::belief::cover(g, h, [door[0], 0.0, door[2]]); }
    if b.n >= bl.rounds || g.time < b.next { return false; }
    b.n += 1.0;
    b.next = g.time + bl.every;
    let c = g.gd[h].ch;
    if g.gd[h].held.is_some() || crate::control::man::staggered(g, h) || g.ch[c].gun_dir[1] < -bl.low_gun { return false; }
    if g.tune.suppression && (g.cond[g.gd[h].ch].sup.v >= g.k.suppression.pinned || g.cond[g.gd[h].ch].sup.latched || g.cond[g.gd[h].ch].sup.down_t > 0.0) { return false; }
    let weapon = g.gd[h].weapon;
    if !crate::weapons::firearm_fire(g, weapon) { return false; }
    let target = v3::mad(g.ch[c].muzzle, g.ch[c].gun_dir, 8.0);
    let opts = crate::combat::FireOpts { steady: Some(true), spread: Some(bl.spread), blind: Some(true), ..Default::default() };
    crate::combat::fire_weapon(g, c, target, false, Some(&opts));
    true
}

/// The two men F's drill moves.
pub fn night_breach_drill_ops(g: &crate::game::Game) -> Vec<GuardId> {
    if g.night_breach.no_drill { return Vec::new(); }
    g.night_breach.ops.iter().take(2).copied().collect()
}

/// A new one: its initialisers.
pub fn night_breach_new(g: &crate::game::Game) -> crate::nightbreach::NightBreach {
    NightBreach::blank()
}

/// The team start in force.
fn night_breach_team_of(g: &mut crate::game::Game, layout: Option<&crate::nightbreach::NightBreachLayout>) -> Option<crate::scene_scenarios::TeamStart> {
    if let Some(t) = layout_team(g, layout) { return Some(t); }
    g.level.scenarios.as_ref().and_then(|s| s.night_breach.as_ref()).map(|n| n.team.clone())
}

/// Sam's start knobs take the layout's team start, once per start.
fn night_breach_apply_team(g: &mut crate::game::Game, layout: Option<&crate::nightbreach::NightBreachLayout>) {
    let Some(t) = night_breach_team_of(g, layout) else { return };
    if g.m.nightbreach.applied.as_ref().is_some_and(|a| same_team(a, &t)) { return; }
    g.k.nightbreach.sam_at = t.sam;
    g.k.nightbreach.sam_yaw = t.sam_yaw;
    g.m.nightbreach.applied = Some(t);
}

/// Why it cannot start right now.
pub(super) fn night_breach_can_start(g: &mut crate::game::Game, layout: Option<&crate::nightbreach::NightBreachLayout>) -> Option<String> {
    let t = night_breach_team_of(g, layout);
    if g.quiet.active { return Some(String::from("a quiet mission is up on this floor (quit it first)")); }
    let Some(t) = t else { return Some(String::from("this level does not say where the team starts (its scenarios block)")) };
    let brings_marks = layout.is_some() && t.ops.as_ref().is_some_and(|o| o.len() >= 2);
    if g.level.breach.is_none() && !brings_marks { return Some(String::from("this level has no breach plan")); }
    if let Some(door) = g.level.breach.as_ref().map(|p| p.door.clone()) {
        if crate::doors::doors_by_name(g, &door).is_none() { return Some(format!("the plan's door '{}' is not in the level", door)); }
    }
    if !g.tune.teams { return Some(String::from("sides are switched off (tune.teams)")); }
    if !g.ai_enabled { return Some(String::from("the AI is switched off (sandbox: no AI) — nobody would move")); }
    if let Some(r) = g.director.run {
        let run = &g.tab.director_run[r];
        if !run.ended { return Some(format!("the level script's program '{}' is running", run.name)); }
    }
    if g.puppet.is_some() { return Some(String::from("nobody is playing")); }
    if g.guards.iter().any(|x| g.gd[*x].unit.is_some()) { return Some(String::from("a spawned unit is already on the floor")); }
    None
}

/// Start the scenario; the reason when refused or when the staging failed.
pub(super) fn night_breach_start(g: &mut crate::game::Game, layout: Option<&crate::nightbreach::NightBreachLayout>) -> Option<String> {
    if g.night_breach.active { return None; }
    if let Some(why) = night_breach_can_start(g, layout) { return Some(why); }
    g.night_breach.layout = layout.cloned();
    night_breach_apply_team(g, layout);
    night_breach_bench(g);
    g.night_breach.active = true;
    g.night_breach.staged = 0.0;
    g.night_breach.log.clear();
    g.night_breach.failure = None;
    crate::game::game_restart_encounter(g);
    if g.night_breach.active { return None; }
    Some(g.night_breach.failure.clone().unwrap_or_else(|| String::from("the staging failed")))
}

/// Stop it: the cast gone, the shift back, a fresh night encounter.
fn night_breach_stop(g: &mut crate::game::Game) {
    if !g.night_breach.active { return; }
    g.night_breach.active = false;
    g.night_breach.layout = None;
    if g.night_breach.drill.is_some() { crate::breach::breach_drill_end(g); }
    g.night_breach.drill = None;
    let mut cast = g.night_breach.ops.clone();
    cast.extend_from_slice(&g.night_breach.hostiles);
    for gd in cast { crate::guards::despawn_unit(g, gd); }
    g.night_breach.ops = Vec::new();
    g.night_breach.hostiles = Vec::new();
    g.night_breach.marks = Vec::new();
    if let Some(benched) = g.night_breach.benched.take() {
        for gd in benched.iter() {
            let l = g.gd[*gd].flashlight;
            if !g.engine.lights.lights.contains(&l) { crate::render_lights::light_set_add(g, l); }
        }
        let mut guards = benched;
        guards.extend_from_slice(&g.guards);
        g.guards = guards;
    }
    crate::game::game_restart_encounter(g);
}

/// A man stood on this floor after the staging joins the defenders.
pub fn night_breach_adopt(g: &mut crate::game::Game, gd: GuardId, mark: &crate::nightbreach::NightBreachMark) {
    if !g.night_breach.active || g.night_breach.hostiles.contains(&gd) { return; }
    g.night_breach.hostiles.push(gd);
    g.night_breach.marks.push(mark.clone());
    g.night_breach.bursts.push(BlindBurst { until: -1.0, n: 0.0, next: 0.0 });
}

/// Stage it afresh: a restart of the encounter, which stages.
pub(super) fn night_breach_restage(g: &mut crate::game::Game) {
    if g.night_breach.active { crate::game::game_restart_encounter(g); }
}

/// restartEncounter's last act: stage, and a staging that fails ends the scenario.
pub fn night_breach_on_encounter_restart(g: &mut crate::game::Game) {
    if !g.night_breach.active { return; }
    if let Err(e) = stage_checked(g) {
        let failure = format!("the staging failed: {}", e);
        g.night_breach.failure = Some(failure.clone());
        night_breach_stop(g);
    }
}

/// The level's own men off the guard list, torches out of the light set.
fn night_breach_bench(g: &mut crate::game::Game) {
    let staff: Vec<GuardId> = g.guards.iter().filter(|x| g.gd[**x].unit.is_none()).copied().collect();
    if staff.is_empty() { return; }
    for gd in staff.iter() {
        let l = g.gd[*gd].flashlight;
        g.tab.light[l].enabled = false;
        crate::render_lights::light_set_remove(g, l);
    }
    let mut benched = g.night_breach.benched.take().unwrap_or_default();
    benched.extend_from_slice(&staff);
    g.night_breach.benched = Some(benched);
    g.guards = g.guards.iter().filter(|x| g.gd[**x].unit.is_some()).copied().collect();
}

/// A place for a man of the second pair.
pub fn night_breach_stand_by(g: &mut crate::game::Game, lead: V3, taken: &[V3], yaw: Option<f64>) -> V3 {
    let yaw = yaw.unwrap_or(m::PI / 2.0);
    match stand_by_checked(g, lead, taken, yaw) {
        Ok(q) => q,
        Err(e) => panic!("{}", e),
    }
}

/// What happened when, on the game clock.
fn night_breach_note(g: &mut crate::game::Game, what: &str) {
    g.night_breach.log.push(NightBreachLogItem { what: String::from(what) });
    if g.night_breach.log.len() > 400 { g.night_breach.log.drain(0..100); }
}

/// A line a living teammate says once a staging; with nobody up it is not said.
fn night_breach_once_by_a_man(g: &mut crate::game::Game, key: &str, text: &str) {
    if !g.night_breach.said.has(String::from(key)) {
        g.night_breach.said.add(String::from(key));
        crate::squadnet::team_says(g, None, text);
    }
}

/// What a teammate says when the man the drill leaned on has fallen and the room is the player's now.
pub fn night_breach_down_line(g: &crate::game::Game) -> String {
    let down = night_breach_drill_ops(g).into_iter().find(|&o| crate::condition::down(g, g.gd[o].ch));
    format!("{}'s down at the door. it's on you.", down.map_or("the point man", |o| g.gd[o].callsign.as_str()))
}

/// Sam's word (F): the first stacks the pair, the second sends them in.
fn night_breach_order(g: &mut crate::game::Game) -> Option<crate::nightbreach::NightBreachOrder> {
    if !g.night_breach.active || g.night_breach.drill.is_none() { return None; }
    if crate::seats::all_down(g) { return None; }
    if g.night_breach.drill.as_ref().unwrap().phase == crate::breach::Phase::Done { return None; }
    if live_of(g).orders == 0.0 {
        live_set_orders(g, 1.0);
        night_breach_note(g, "order:stack");
        return Some(NightBreachOrder::Stack);
    }
    if live_of(g).orders == 1.0 {
        live_set_orders(g, 2.0);
        night_breach_note(g, "order:go");
        crate::seats::each_seat(g, &mut |v| crate::holdfire::hold_fire_drill(v, None));
        if live_of(g).stun {
            let by = crate::seats::first_of(g, &mut |v| {
                if crate::doortools::door_tools_take(v, crate::doortools::DoorToolsTakeWhat::Bang) { Some(v.seat) } else { None }
            });
            g.night_breach.bang_by = by;
            if by.is_some() {
                g.night_breach.bang_taken = true;
            } else {
                live_stun_off(g);
                let by = night_breach_drill_ops(g).first().copied();
                crate::interact::say_why(g, by, crate::view::ThingKind::Door, "no bangs left. going in without one.", "no bangs left");
            }
        }
        return Some(NightBreachOrder::Go);
    }
    None
}

/// Is the drill moving the pair right now.
pub fn night_breach_owns_pair(g: &crate::game::Game) -> bool {
    if !g.night_breach.active { return false; }
    match g.night_breach.drill.as_ref() {
        Some(d) => d.phase != crate::breach::Phase::Wait && d.phase != crate::breach::Phase::Done,
        None => false,
    }
}

/// A menu order came for a man the drill is moving: let go of the pair, or say why not.
pub fn night_breach_call_off(g: &mut crate::game::Game) -> Option<String> {
    if !g.night_breach.active || g.night_breach.drill.is_none() { return None; }
    match night_breach_state(g) {
        NightBreachState::Stacking | NightBreachState::Set => {
            let d = g.night_breach.drill.as_mut().unwrap();
            d.phase = crate::breach::Phase::Wait;
            d.t_stack = f64::NAN;
            d.t_set = f64::NAN;
            live_set_orders(g, 0.0);
            night_breach_note(g, "order:off");
            if g.night_breach.bang_taken {
                g.night_breach.bang_taken = false;
                if let Some(s) = g.night_breach.bang_by {
                    crate::seats::with_seat(g, s, &mut |v| crate::doortools::door_tools_refund(v, crate::doortools::DoorToolsRefundWhat::Bang));
                }
                g.night_breach.bang_by = None;
            }
            for o in night_breach_drill_ops(g) {
                if !crate::condition::down(g, g.gd[o].ch) && g.gd[o].script.is_some() {
                    crate::squad::order(g, o, crate::squad::OrderPatch { goal: Some(None), ..Default::default() });
                }
            }
            None
        }
        NightBreachState::Going => Some(String::from("we're committed. we're going in.")),
        NightBreachState::Inside => {
            let d = g.night_breach.drill.as_mut().unwrap();
            d.phase = crate::breach::Phase::Done;
            d.t_done = d.t;
            d.beats.push(crate::breach::BreachDrillBeatsItem { what: String::from("recalled") });
            None
        }
        _ => None,
    }
}

/// Once a frame from Game.update, after the Director and before the guards move.
pub fn night_breach_tick(g: &mut crate::game::Game, dt: f64, input: InputId, cam: Option<CameraId>) {
    if !g.night_breach.active { return; }
    if g.night_breach.cut && cam.is_some() {
        g.night_breach.cut = false;
        crate::seats::cut_to_men(g);
    }
    let drilled = g.night_breach.drill.is_some();
    if drilled {
        for s in g.seats.clone() {
            let man = crate::cmd::seat_man(g, s);
            let seat_input = g.tab.cmd_seat[s].input;
            if !crate::condition::down(g, g.pl[man].ch) && crate::input::input_hit(g, seat_input, "KeyF") {
                night_breach_order(g);
                break;
            }
        }
    }
    let own = crate::roomfight::room_fight_on(g);
    if !own {
        let n = g.night_breach.hostiles.len();
        for i in 0..n {
            let Some(h) = g.night_breach.hostiles.get(i).copied() else { break };
            if crate::condition::down(g, g.gd[h].ch) { continue; }
            if g.gd[h].script.is_some() {
                let was = script_state(g.gd[h].state);
                crate::squad::end_guard_script(g, h, was);
            }
            crate::control::set_hold(g, h, true, "nightbreach");
            crate::control::set_pinned(g, h, true, "nightbreach");
            let Some(aim) = g.night_breach.marks.get(i).map(|m| m.aim_at) else { continue };
            if g.gd[h].state == crate::game::GuardState::Patrol && g.gd[h].awareness < 0.3 && g.gd[h].held.is_none() {
                let c = g.gd[h].ch;
                g.ch[c].body_yaw = v3::yaw_to(g.ch[c].pos, aim);
            }
        }
    }
    if drilled { crate::breach::breach_drill_tick(g, dt); }
    if own {
        g.night_breach.blind_rounds = g.fight.blind_rounds;
    } else {
        night_breach_blind_fire(g);
    }
    if !drilled { return; }
    loop {
        let seen = g.night_breach.beats_seen as usize;
        let what = match g.night_breach.drill.as_ref().and_then(|d| d.beats.get(seen)) {
            Some(b) => b.what.clone(),
            None => break,
        };
        night_breach_note(g, &what);
        g.night_breach.beats_seen += 1.0;
    }
    match night_breach_state(g) {
        NightBreachState::Recalled | NightBreachState::Failed => {
            for o in night_breach_drill_ops(g) {
                if !crate::condition::down(g, g.gd[o].ch) && g.gd[o].script.is_some() && !in_his_book(g, o) {
                    crate::squad::end_guard_script(g, o, Some(crate::squad::EndGuardScriptState::Search));
                    crate::control::set_hold(g, o, true, "nightbreach");
                    crate::control::set_pinned(g, o, true, "nightbreach");
                }
            }
            if night_breach_state(g) == NightBreachState::Recalled {
                night_breach_once_by_a_man(g, "recalled", "calling it off. we're with you.");
            } else if g.night_breach.layout.is_none() {
                let line = night_breach_down_line(g);
                night_breach_once_by_a_man(g, "failed", &line);
            }
        }
        _ => {}
    }
}

/// Every dazzled defender's burst this frame.
fn night_breach_blind_fire(g: &mut crate::game::Game) {
    if !g.ai_enabled { return; }
    let n = g.night_breach.hostiles.len();
    for k in 0..n {
        let Some(h) = g.night_breach.hostiles.get(k).copied() else { break };
        let (Some(mut b), Some(door)) = (g.night_breach.bursts.get(k).copied(), g.night_breach.marks.get(k).map(|m| m.aim_at)) else { continue };
        let fired = blind_turn(g, h, k as f64, &mut b, door);
        if let Some(row) = g.night_breach.bursts.get_mut(k) { *row = b; }
        if fired { g.night_breach.blind_rounds += 1.0; }
    }
}

/// The HUD's objective while the night breach is up by itself.
pub(super) fn night_breach_objective(g: &crate::game::Game) -> Option<crate::nightbreach::NightBreachObjective> {
    if !g.night_breach.active || g.night_breach.layout.is_some() || g.night_breach.hostiles.is_empty() { return None; }
    let taken = all_taken(g);
    let plan = g.level.breach.as_ref().map(|p| (p.door.clone(), p.room.clone()));
    let room = plan.as_ref().and_then(|p| p.1.clone()).unwrap_or_else(|| String::from("room"));
    let text = if taken { format!("{} taken — press shift+enter to restart", room) } else { format!("objective ▸ take the {}", room) };
    let pos = if taken {
        None
    } else {
        match plan.as_ref() {
            Some(p) => crate::doors::doors_by_name(g, &p.0).map(|d| g.dr[d].frame_centre),
            None => None,
        }
    };
    Some(NightBreachObjective { text, pos })
}

/// Where the breach stands, in one word.
pub fn night_breach_state(g: &crate::game::Game) -> crate::nightbreach::NightBreachState {
    if !g.night_breach.active { return NightBreachState::Off; }
    let Some(d) = g.night_breach.drill.as_ref() else {
        return if g.night_breach.no_drill { NightBreachState::Staged } else { NightBreachState::Off };
    };
    match d.phase {
        crate::breach::Phase::Done => {
            if d.beats.iter().any(|b| b.what == "clear") {
                NightBreachState::Clear
            } else if d.beats.iter().any(|b| b.what == "recalled") {
                NightBreachState::Recalled
            } else {
                NightBreachState::Failed
            }
        }
        crate::breach::Phase::Wait => NightBreachState::Staged,
        crate::breach::Phase::Approach => NightBreachState::Stacking,
        crate::breach::Phase::Stack => {
            if d.t_set.is_nan() {
                NightBreachState::Stacking
            } else if live_of(g).orders >= 2.0 {
                NightBreachState::Going
            } else {
                NightBreachState::Set
            }
        }
        crate::breach::Phase::Enter | crate::breach::Phase::Clear => NightBreachState::Inside,
        _ => NightBreachState::Going,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scene_scenarios::TeamStart;

    #[test]
    fn a_teammates_mark_comes_from_the_layout_then_the_plan_then_the_start() {
        let start = TeamStart { sam: [0.0; 3], sam_yaw: 0.0, ops: Some(vec![[1.0, 0.0, 1.0], [2.0, 0.0, 2.0]]), ops_yaw: None };
        assert_eq!(mark_of(None, None, Some(&start), 1), Ok([2.0, 0.0, 2.0]));
        assert_eq!(mark_of(None, None, Some(&start), 2), Err(String::from("no mark for teammate 3 (the level's team start, or its breach plan)")));
        let mut plan = crate::scene_level::BreachPlan::default();
        plan.stack = [[3.0, 0.0, 3.0], [4.0, 0.0, 4.0]];
        assert_eq!(mark_of(None, Some(&plan), Some(&start), 0), Ok([3.0, 0.0, 3.0]));
        plan.from = Some([[5.0, 0.0, 5.0], [6.0, 0.0, 6.0]]);
        assert_eq!(mark_of(None, Some(&plan), Some(&start), 1), Ok([6.0, 0.0, 6.0]));
        let own = TeamStart { sam: [0.0; 3], sam_yaw: 0.0, ops: Some(vec![[7.0, 0.0, 7.0]]), ops_yaw: None };
        assert_eq!(mark_of(Some(&own), Some(&plan), Some(&start), 0), Ok([7.0, 0.0, 7.0]));
        assert_eq!(mark_of(Some(&own), Some(&plan), Some(&start), 1), Ok([6.0, 0.0, 6.0]));
    }

    #[test]
    fn two_starts_are_the_same_when_every_row_is() {
        let a = TeamStart { sam: [1.0, 0.0, 2.0], sam_yaw: 0.5, ops: None, ops_yaw: None };
        assert!(same_team(&a, &a.clone()));
        assert!(!same_team(&a, &TeamStart { sam_yaw: 0.25, ..a.clone() }));
        assert!(!same_team(&a, &TeamStart { sam: [1.0, 0.0, 3.0], ..a.clone() }));
        assert!(!same_team(&a, &TeamStart { ops: Some(alloc::vec![[9.0, 0.0, 9.0]]), ..a.clone() }));   // (another row of the level: another start, though Sam stands the same)
    }

    /// The burst's beats up to the first round: what is written lands in the caller's row.
    #[test]
    fn a_new_dazzle_starts_a_burst_and_a_turn_waits_for_its_moment() {
        let mut g = Game::blank();
        let bl = crate::nightbreach::blind();
        let mut man = crate::game::Guard::blank();
        man.ch = crate::character::man_new(&mut g, crate::character::Character::blank());
        g.cond[man.ch].blind_until = 3.0;
        man.held = Some(crate::game::GuardHeld::default());   // (cover at :114 is another part's: in Sam's arm it is not asked)
        let h = g.gd.push(man);
        g.time = 1.0;
        let mut b = BlindBurst { until: -1.0, n: 5.0, next: 0.0 };
        assert!(!blind_turn(&mut g, h, 2.0, &mut b, [4.0, 1.0, 6.0]));
        assert_eq!((b.until, b.n), (3.0, 0.0));
        assert_eq!(b.next.to_bits(), (1.0 + bl.after + 2.0 * bl.stagger).to_bits());
        g.time = 3.0;
        let mut over = b;
        assert!(!blind_turn(&mut g, h, 2.0, &mut over, [4.0, 1.0, 6.0]));
        assert_eq!(over.next.to_bits(), b.next.to_bits());
    }

    /// The word for each phase of the drill.
    #[test]
    fn the_state_word_follows_the_drill() {
        use crate::breach::{BreachDrill, BreachDrillBeatsItem, Phase};
        let mut g = Game::blank();
        assert_eq!(night_breach_state(&mut g), NightBreachState::Off);
        g.night_breach.active = true;
        assert_eq!(night_breach_state(&mut g), NightBreachState::Off);
        g.night_breach.no_drill = true;
        assert_eq!(night_breach_state(&mut g), NightBreachState::Staged);
        let mut d = BreachDrill::blank();
        d.phase = Phase::Stack;
        g.night_breach.drill = Some(d);
        assert_eq!(night_breach_state(&mut g), NightBreachState::Stacking);
        g.night_breach.drill.as_mut().unwrap().t_set = 1.0;
        assert_eq!(night_breach_state(&mut g), NightBreachState::Set);
        g.night_breach.drill.as_mut().unwrap().live = crate::breach::BreachLive { orders: 2.0, stun: true };
        assert_eq!(night_breach_state(&mut g), NightBreachState::Going);
        let d = g.night_breach.drill.as_mut().unwrap();
        d.phase = Phase::Done;
        assert_eq!(night_breach_state(&mut g), NightBreachState::Failed);
        let d = g.night_breach.drill.as_mut().unwrap();
        d.beats.push(BreachDrillBeatsItem { what: String::from("recalled") });
        assert_eq!(night_breach_state(&mut g), NightBreachState::Recalled);
        let d = g.night_breach.drill.as_mut().unwrap();
        d.beats.push(BreachDrillBeatsItem { what: String::from("clear") });
        assert_eq!(night_breach_state(&mut g), NightBreachState::Clear);
    }

    /// Four hundred kept, then a hundred dropped from the front.
    #[test]
    fn the_log_drops_its_oldest_hundred_past_four_hundred() {
        let mut g = Game::blank();
        for _ in 0..400 { night_breach_note(&mut g, "x"); }
        assert_eq!(g.night_breach.log.len(), 400);
        night_breach_note(&mut g, "y");
        assert_eq!(g.night_breach.log.len(), 301);
        assert_eq!(g.night_breach.log[300].what, "y");
    }
}
