use crate::v3::dist_xz;
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// which plan the man's role on the roster belongs to (`st.roles.get(m)?.plan`)
fn role_plan(g: &mut Game, m: GuardId) -> Option<TacticsPlanId> {
    let r = *s(g).roles.get(m)?;
    Some(g.tab.tactics_role[r].plan)
}

/// `plan.roles.get(m)?.kind`
fn plan_role_kind(g: &Game, plan: TacticsPlanId, m: GuardId) -> Option<RoleKind> {
    let r = *g.tab.tactics_plan[plan].roles.get(m)?;
    Some(g.tab.tactics_role[r].kind)
}

fn aim_at_fix(out: &mut TacMove, fix: V3) {
    if out.aim.is_none() {
        out.aim = Some([fix[0], 1.1, fix[2]]);
    }
}

fn head_or_pos(g: &Game, gd: GuardId) -> V3 {
    let c = g.gd[gd].ch;
    match g.ch[c].bones.get(crate::character::Bone::Head) {
        Some(p) => *p,
        None => g.ch[c].pos,
    }
}

/// what tacticsPenalty's answer charges for stepping into cell (ix, iz)   (tactics.ts:361 tacticsPenalty, the closure it returns)
pub fn tactics_penalty_price(p: &crate::tactics::TacticsPenalty, ix: f64, iz: f64) -> f64 {
    let TacticsPenalty::Cells { w, door_idx, door_mult, avoid } = p;
    let i = crate::Num(iz * *w + ix);
    let mut price = 1.0;
    if door_idx.as_ref().map_or(false, |d| d.has(i)) {
        price = *door_mult;
    }
    if avoid.as_ref().map_or(false, |a| a.has(i)) {
        price = m::max(price, CORRIDOR_COST);
    }
    price
}

/// Once a frame: keep every plan honest and deal new ones.
fn tick_plans(g: &mut crate::game::Game) {
    // (a plan ended earlier in this walk is walked too: its row is still there and every man has come off it)
    for plan in s(g).plans.clone() { keep_plan(g, plan); }
    if g.time - s(g).deal_fail_t >= REPLAN_GAP_SECS { deal_plans(g); }
}

/// ONE PLAN: the unfit come off it, the fix is the freshest among its men, and the roles are dealt again when the fix moves, a wall opens, or the presser is lost or pinned.
fn keep_plan(g: &mut Game, plan: TacticsPlanId) {
    let now = g.time;
    drop_unfit(g, plan);
    let live = g.tab.tactics_plan[plan].members.clone();
    let row = &g.tab.tactics_plan[plan];
    if now - row.t0 > PLAN_CAP_SECS || live.is_empty() || (live.len() < 2 && row.phase == PlanPhase::Approach) { return end_plan(g, plan, None); }
    g.tab.tactics_plan[plan].contact = live.iter().any(|&m| in_contact(g, m));
    let mut fresh = live[0];
    for &m in &live { if g.gd[m].fix_t > g.gd[fresh].fix_t { fresh = m; } }
    let fix = g.gd[fresh].last_known.unwrap();
    let dealt = g.tab.tactics_plan[plan].replanned_at;
    let again = now - dealt > REPLAN_GAP_SECS;
    if again && dist_xz(fix, g.tab.tactics_plan[plan].target) > REPLAN_M { return fix_moved(g, plan, &live, fresh, fix); }
    if g.tab.tactics_plan[plan].phase != PlanPhase::Approach { return; }
    let team = g.tab.tactics_plan[plan].team;
    if again && crate::wallreact::wall_react_replan_2(g, team, dealt, &live) { return re_deal(g, plan, &live, fix); }
    let Some(presser) = live.iter().copied().find(|&m| plan_role_kind(g, plan, m) == Some(RoleKind::Presser)) else { return re_deal(g, plan, &live, fix) };
    if again && suppressed(g, presser) { flanker_presses(g, plan, &live, presser, fix); }
}
/// The men no longer fit for it, or on another plan, come off it.
fn drop_unfit(g: &mut Game, plan: TacticsPlanId) {
    for m in g.tab.tactics_plan[plan].members.clone() {
        if fit_for_role(g, m) && role_plan(g, m) == Some(plan) { continue; }
        g.tab.tactics_plan[plan].members.retain(|x| *x != m);
        if role_plan(g, m) == Some(plan) { s(g).roles.delete(m); }
        g.tab.tactics_plan[plan].roles.delete(m);
    }
}
/// The freshest fix among them has moved: everybody's fix becomes it, it is said, and the roles are dealt again to those who are far from it.
fn fix_moved(g: &mut Game, plan: TacticsPlanId, live: &[GuardId], fresh: GuardId, fix: V3) {
    for &m in live { if m != fresh { crate::belief::word(g, m, fix); } }
    if g.time - g.tab.tactics_plan[plan].last_radio_t > 4.0 {
        let who = if crate::teams::defender_words(g, fresh) { "they've" } else { "he's" };
        let place = crate::dialogue::place_name(g, fix);
        crate::game::game_say(g, fresh, &alloc::format!("{} moved — {}", who, place), Some(true), None);
        g.tab.tactics_plan[plan].last_radio_t = g.time;
    }
    let far: Vec<GuardId> = live.iter().copied().filter(|&m| dist_xz(g.pos_of(m), fix) > JOIN_M && !in_contact(g, m)).collect();
    end_plan(g, plan, None);
    if far.len() >= 2 { plan_approach(g, &far, fix, None); }
}
/// The presser pinned by fire, a flanker steadier: the flanker presses from where he is.
fn flanker_presses(g: &mut Game, plan: TacticsPlanId, live: &[GuardId], presser: GuardId, fix: V3) {
    let steadier = g.cond[g.gd[presser].ch].sup.v - 0.05;
    let Some(fl) = live.iter().copied().find(|&m| plan_role_kind(g, plan, m) == Some(RoleKind::Flanker) && g.cond[g.gd[m].ch].sup.v < steadier && !suppressed(g, m)) else { return };
    end_plan(g, plan, None);
    let Some(p2) = plan_approach(g, live, fix, Some(fl)) else { return };
    // (no waiting for anybody: he goes as soon as he is at his corner)
    if let Some(&r) = g.tab.tactics_plan[p2].roles.get(fl) { g.tab.tactics_role[r].hold_t = g.tune.flank_wait; }
}

/// NEW PLANS: unplanned free alert men of one side, no eyes on their man, more than JOIN_M off a fix they share. A live plan of his side on (about) a man's fix takes him in by a re-deal.
fn deal_plans(g: &mut Game) {
    let now = g.time;
    let mut cands = Vec::new();
    for m in g.guards.clone() {
        if fit_for_role(g, m) && !s(g).roles.has(m) && !s(g).gates.has(m) && !in_contact(g, m) && dist_xz(g.pos_of(m), g.gd[m].last_known.unwrap()) > JOIN_M { cands.push(m); }
    }
    let mut used: JsSet<GuardId> = JsSet::new();
    for &seed in &cands {
        if used.has(seed) { continue; }
        let fix = g.gd[seed].last_known.unwrap();
        let team = g.team(seed);
        let near = s(g).plans.clone().into_iter().find(|&p| {
            let row = &g.tab.tactics_plan[p];
            row.team == team && row.phase == PlanPhase::Approach && dist_xz(row.target, fix) <= SHARE_M && now - row.replanned_at > REPLAN_GAP_SECS
        });
        if let Some(near) = near {
            let mut men: Vec<GuardId> = g.tab.tactics_plan[near].members.iter().copied().filter(|&m| !in_contact(g, m)).collect();
            men.push(seed);
            used.add(seed);
            end_plan(g, near, None);
            plan_approach(g, &men, fix, None);
            continue;
        }
        let leader = g.gd[seed].leader;
        let group: Vec<GuardId> = cands.iter().copied().filter(|&o| {
            !used.has(o) && g.team(o) == team && (o == seed || dist_xz(g.gd[o].last_known.unwrap(), fix) <= SHARE_M || g.gd[o].leader == Some(seed) || leader == Some(o))
        }).collect();
        if group.len() < 2 { continue; }
        for &o in &group {
            used.add(o);
            if o != seed && dist_xz(g.gd[o].last_known.unwrap(), fix) > SHARE_M { crate::belief::word(g, o, fix); }
        }
        plan_approach(g, &group, fix, None);
    }
}

/// the men not fighting by sight are dealt roles again; the plan they were in is over
fn re_deal(g: &mut Game, plan: TacticsPlanId, live: &[GuardId], fix: V3) {
    let rest: Vec<GuardId> = live.iter().copied().filter(|&m| !in_contact(g, m)).collect();
    end_plan(g, plan, None);
    if rest.len() >= 2 { plan_approach(g, &rest, fix, None); }
}

/// walk / jog / run to `goal` the way the chase always did; true once there
fn advance(g: &mut crate::game::Game, gd: GuardId, dt: f64, goal: V3, pace: f64, out: &mut crate::tactics::TacMove) -> bool {
    crate::guards::go_to(g, gd, goal, None);
    let eyes = in_contact(g, gd);
    let pie = crate::guards::pie_doorway(g, gd, dt, pace, eyes);
    out.pied = true;
    let arrived = crate::guards::follow_path(g, gd, dt, pie.speed);
    out.move_ = g.gd[gd].speed;
    if pie.look.is_some() {
        out.aim = pie.look;
    }
    arrived
}

/// planted, low, chest and gun toward `at`
fn kneel_toward(g: &mut crate::game::Game, gd: GuardId, dt: f64, at: V3, out: &mut crate::tactics::TacMove, crouch: Option<f64>) {
    let crouch = crouch.unwrap_or(1.0);
    let c = g.gd[gd].ch;
    g.gd[gd].speed = v3::damp(g.gd[gd].speed, 0.0, 8.0, dt);
    g.ch[c].vel = [0.0, 0.0, 0.0];
    g.gd[gd].stall_ref = None;
    out.move_ = 0.0;
    out.crouch = crouch;
    out.aim = Some([at[0], 1.05, at[2]]);
    let p = g.ch[c].pos;
    g.ch[c].body_yaw = v3::approach_angle(g.ch[c].body_yaw, v3::yaw_to(p, at), 4.0 * dt);
}

/// the flankers of a plan are all set (or there are none), or somebody has eyes on the man
fn ready_to_go(g: &crate::game::Game, plan: TacticsPlanId) -> bool {
    if g.tab.tactics_plan[plan].contact {
        return true;
    }
    let pr = presser_of(g, plan);
    let roles: Vec<(GuardId, TacticsRoleId)> = g.tab.tactics_plan[plan].roles.iter().map(|(m, r)| (m, *r)).collect();
    for (m, r) in roles {
        if !g.tab.tactics_plan[plan].members.contains(&m) || in_contact(g, m) {
            continue;
        }
        let (kind, holding) = (g.tab.tactics_role[r].kind, g.tab.tactics_role[r].holding);
        if kind == RoleKind::Flanker && !holding {
            return false;
        }
        if kind == RoleKind::Second {
            if let Some(pr) = pr {
                if dist_xz(g.pos_of(m), g.pos_of(pr)) > BEHIND_M + 1.5 {
                    return false;
                }
            }
        }
    }
    true
}

/// some flanker of the plan is not yet set but still covering ground toward his corner
fn flankers_running(g: &crate::game::Game, plan: TacticsPlanId) -> bool {
    let row = &g.tab.tactics_plan[plan];
    for (m, r) in row.roles.iter() {
        let r = &g.tab.tactics_role[*r];
        if r.kind == RoleKind::Flanker && row.members.contains(&m) && !r.holding && r.ground.unwrap_or(0.0) > 1.2 && g.time - r.t0 > 1.0 {
            return true;
        }
    }
    false
}

/// why everybody pushes
#[derive(Clone, Copy, PartialEq)]
enum Go { Set, Contact, TimeUp }

/// the word: everybody pushes
fn go_now(g: &mut crate::game::Game, plan: TacticsPlanId, caller: GuardId, why: Go) {
    if g.tab.tactics_plan[plan].phase != PlanPhase::Approach {
        return;
    }
    g.tab.tactics_plan[plan].phase = PlanPhase::Push;
    let roles: Vec<TacticsRoleId> = g.tab.tactics_plan[plan].roles.values().copied().collect();
    for r in roles {
        g.tab.tactics_role[r].holding = false;
    }
    let line = match why { Go::Contact => "eyes on — go go go", Go::TimeUp => "can't wait — pushing, go", Go::Set => "go go go" };
    crate::game::game_say(g, caller, line, Some(true), None);
    g.tab.tactics_plan[plan].last_radio_t = g.time;
    for m in g.tab.tactics_plan[plan].members.clone() {
        g.gd[m].drop_path();
        g.gd[m].stall_ref = None;
    }
}

/// the man reaches the fix: off the roster into the search everybody always dropped to
fn arrive(g: &mut crate::game::Game, gd: GuardId, role: TacticsRoleId) {
    let plan = g.tab.tactics_role[role].plan;
    g.tab.tactics_role[role].arrived = true;
    if g.tab.tactics_plan[plan].first_arrive_t < 0.0 {
        g.tab.tactics_plan[plan].first_arrive_t = g.time;
    }
    s(g).roles.delete(gd);
    let row = &mut g.tab.tactics_plan[plan];
    row.roles.delete(gd);
    row.members.retain(|m| *m != gd);
    let bark = if g.tab.tactics_role[role].kind == RoleKind::Presser || g.tab.tactics_plan[plan].members.is_empty() { Some("not here — spread out, check your side") } else { None };
    crate::guards::drop_to_search(g, gd, bark);
    if g.tab.tactics_plan[plan].members.is_empty() {
        end_plan(g, plan, None);
    }
}

/// a man's move on his role
fn move_role(g: &mut crate::game::Game, gd: GuardId, role: TacticsRoleId, dt: f64) -> crate::tactics::TacMove {
    let plan = g.tab.tactics_role[role].plan;
    let mut out = TacMove { move_: 0.0, aim: None, pied: false, crouch: 0.0 };
    if g.tab.tactics_plan[plan].phase == PlanPhase::Push {
        push(g, gd, role, dt, &mut out);
        return out;
    }
    match g.tab.tactics_role[role].kind {
        RoleKind::Presser => press(g, gd, role, dt, &mut out),
        RoleKind::Flanker => flank(g, gd, role, dt, &mut out),
        RoleKind::Second => second(g, gd, role, dt, &mut out),
    }
    out
}

/// Everybody at the run to the fix itself; the second keeps his station behind the presser on the way.
fn push(g: &mut Game, gd: GuardId, role: TacticsRoleId, dt: f64, out: &mut TacMove) {
    let (plan, kind) = (g.tab.tactics_role[role].plan, g.tab.tactics_role[role].kind);
    let fix = g.tab.tactics_plan[plan].target;
    let c = g.gd[gd].ch;
    if let (RoleKind::Second, Some(pr)) = (kind, presser_of(g, plan)) {
        let pr_arrived = role_of_man(g, pr).map_or(false, |r| g.tab.tactics_role[r].arrived);
        if pr != gd && !pr_arrived && dist_xz(g.ch[c].pos, g.pos_of(pr)) < BEHIND_M { return kneel_toward(g, gd, dt, fix, out, Some(0.0)); }
    }
    let run = g.tune.guard_chase;
    if advance(g, gd, dt, fix, run, out) || dist_xz(g.ch[c].pos, fix) < ARRIVE_M || crate::guards::not_getting_anywhere(g, gd, dt, ROLE_STALL_SECS) { arrive(g, gd, role); }
    aim_at_fix(out, fix);
}

/// To his corner at a jog, or as far as he gets, and there on a knee until the flankers are set, the fight is on, or he has waited long enough.
fn press(g: &mut Game, gd: GuardId, role: TacticsRoleId, dt: f64, out: &mut TacMove) {
    let plan = g.tab.tactics_role[role].plan;
    let fix = g.tab.tactics_plan[plan].target;
    let c = g.gd[gd].ch;
    if !g.tab.tactics_role[role].holding {
        let hold = g.tab.tactics_role[role].hold_at.unwrap_or(fix);
        let jog = g.tune.guard_investigate;
        if advance(g, gd, dt, hold, jog, out) || dist_xz(g.ch[c].pos, hold) < 0.5 {
            g.tab.tactics_role[role].holding = true;
            g.gd[gd].drop_path();
        } else if crate::guards::not_getting_anywhere(g, gd, dt, ROLE_STALL_SECS) {
            holds_here(g, gd, role);
        }
        return aim_at_fix(out, fix);
    }
    kneel_toward(g, gd, dt, fix, out, None);
    if !flankers_running(g, plan) { g.tab.tactics_role[role].hold_t += dt; }
    if ready_to_go(g, plan) {
        let why = if g.tab.tactics_plan[plan].contact { Go::Contact } else { Go::Set };
        go_now(g, plan, gd, why);
    } else if g.tab.tactics_role[role].hold_t >= g.tune.flank_wait {
        go_now(g, plan, gd, Go::TimeUp);
    }
}

/// The long way round, at the run, or as far round as he gets; he says he is set.
fn flank(g: &mut Game, gd: GuardId, role: TacticsRoleId, dt: f64, out: &mut TacMove) {
    let plan = g.tab.tactics_role[role].plan;
    let fix = g.tab.tactics_plan[plan].target;
    let c = g.gd[gd].ch;
    let set_at = g.tab.tactics_role[role].hold_at.unwrap_or(fix);
    if !g.tab.tactics_role[role].said_set && (dist_xz(g.ch[c].pos, set_at) <= SET_M || g.tab.tactics_role[role].holding) {
        g.tab.tactics_role[role].said_set = true;
        crate::game::game_say(g, gd, "set", Some(true), None);
        g.tab.tactics_plan[plan].last_radio_t = g.time;
    }
    if g.tab.tactics_role[role].holding {
        kneel_toward(g, gd, dt, fix, out, None);
        g.tab.tactics_role[role].hold_t += dt;
        return;
    }
    let p = g.ch[c].pos;
    let r = &mut g.tab.tactics_role[role];
    r.ground = Some(r.ground.unwrap_or(0.0) * m::exp(-dt / 1.5) + dist_xz(p, r.last_pos.unwrap_or(p)));
    r.last_pos = Some(v3::copy(p));
    let run = g.tune.guard_chase;
    if advance(g, gd, dt, set_at, run, out) || dist_xz(g.ch[c].pos, set_at) < 0.5 {
        g.tab.tactics_role[role].holding = true;
        g.gd[gd].drop_path();
    } else if crate::guards::not_getting_anywhere(g, gd, dt, ROLE_STALL_SECS) {
        holds_here(g, gd, role);
        if !g.tab.tactics_role[role].said_set {
            g.tab.tactics_role[role].said_set = true;
            crate::game::game_say(g, gd, "set — can't get any farther around", Some(true), None);
        }
    }
    aim_at_fix(out, fix);
}

/// Up the presser's line, BEHIND him: follow the man, stop three metres short of him, kneel when he kneels.
fn second(g: &mut Game, gd: GuardId, role: TacticsRoleId, dt: f64, out: &mut TacMove) {
    let plan = g.tab.tactics_role[role].plan;
    let fix = g.tab.tactics_plan[plan].target;
    let Some(pr) = presser_of(g, plan) else { return aim_at_fix(out, fix) };
    let goal = g.pos_of(pr);
    let gap = dist_xz(g.pos_of(gd), goal);
    let pr_holding = g.tab.tactics_plan[plan].roles.get(pr).map_or(false, |r| g.tab.tactics_role[*r].holding);
    let moving = g.gd[gd].path.len() > 0 && g.gd[gd].path_i < g.gd[gd].path.len() && g.gd[gd].speed > 0.2;
    if gap > BEHIND_M + (if moving { 0.0 } else { BEHIND_SLACK_M }) {
        let pace = if gap > BEHIND_M + 3.0 { g.tune.guard_chase } else { g.tune.guard_investigate };
        advance(g, gd, dt, goal, pace, out);
        aim_at_fix(out, fix);
    } else {
        g.gd[gd].drop_path();
        kneel_toward(g, gd, dt, fix, out, Some(if pr_holding { 1.0 } else { 0.0 }));
    }
}
/// as far as he gets is his corner
fn holds_here(g: &mut Game, gd: GuardId, role: TacticsRoleId) {
    let here = g.pos_of(gd);
    let r = &mut g.tab.tactics_role[role];
    r.holding = true;
    r.hold_at = Some(here);
    g.gd[gd].drop_path();
}

fn presser_of(g: &crate::game::Game, plan: TacticsPlanId) -> Option<GuardId> {
    let row = &g.tab.tactics_plan[plan];
    for (m, r) in row.roles.iter() {
        if g.tab.tactics_role[*r].kind == RoleKind::Presser && row.members.contains(&m) {
            return Some(m);
        }
    }
    None
}

fn role_of_man(g: &mut crate::game::Game, gd: GuardId) -> Option<TacticsRoleId> {
    s(g).roles.get(gd).copied()
}

/// Is the man this guard is after ARMED, as far as he can know
fn armed_intruder(g: &crate::game::Game, seat: &crate::seats::SeatRef, gd: GuardId) -> bool {
    if !crate::teams::foe_is_player(g, gd) { return g.gd[gd].foe.is_some(); }
    let now = g.time;
    let (shots, last_shot_t, last_t) = {
        let h = &g.gd[gd].witness.heard;
        (h.shots, h.last_shot_t, h.last_t)
    };
    if shots > 0.0 && last_shot_t >= 0.0 && now - last_shot_t < ARMED_SHOT_SECS {
        return true;
    }
    let player = seat.player;
    let last_fire_t = g.pl[player].last_fire_t;
    if last_fire_t >= 0.0 && now - last_fire_t < ARMED_SHOT_SECS && last_t >= 0.0 && now - last_t < ARMED_SHOT_SECS && last_t >= last_fire_t - 1.0 {
        return true;
    }
    crate::game::player_gun_in_hand(g, player).is_some() && g.gd[gd].sight_t >= 0.0 && now - g.gd[gd].sight_t < ARMED_SEEN_SECS
}

/// nobody else of his side is alert (and free): he is on his own with it
fn alone(g: &crate::game::Game, gd: GuardId) -> bool {
    !g.guards.iter().any(|&o| o != gd && g.team(o) == g.team(gd) && g.gd[o].state == crate::game::GuardState::Alert && g.gd[o].held.is_none() && !g.gd[o].hold)
}

/// The corner he holds, and whether it has the line on the fix
fn lone_corner(g: &mut crate::game::Game, gd: GuardId, fix: V3) -> crate::tactics::LoneCorner {
    let from = g.pos_of(gd);
    let pen = tactics_penalty(g, gd, fix);
    let path = match &pen {
        Some(p) => {
            let mut price = |_: &Game, ix: f64, iz: f64| tactics_penalty_price(p, ix, iz);
            crate::scene_collision::nav_grid_find_path(g, from, fix, Some(6000.0), Some(&mut price))
        }
        None => crate::scene_collision::nav_grid_find_path(g, from, fix, Some(6000.0), None),
    }
    .unwrap_or_default();
    let samples = sample_path(from, &path, Some(0.3));
    let mut found: Option<V3> = None;
    for &p in &samples {
        if dist_xz(p, fix) < LONE_HOLD_MIN_M {
            break;
        }
        if los_open(g, p, fix) {
            found = Some(p);
            break;
        }
    }
    let Some(cand) = found else {
        return LoneCorner { p: point_short_of_end(from, &path, LONE_HOLD_MIN_M + 0.5) };
    };
    // onto a wall: the free cell within LONE_COVER_R of it with the most statics about it that still sees the fix
    let cs = g.col.nav.cell;
    let (w_cells, h_cells) = (g.col.nav.w, g.col.nav.h);
    let mut best = cand;
    let mut best_is_cand = true; // (the TypeScript's `best !== cand` is of the object, not its numbers)
    let mut bw = walls_about(g, cand);
    let mut bd = 0.0;
    let r = m::ceil(LONE_COVER_R / cs) as i64;
    let (cx, cz) = (m::floor(cand[0] / cs), m::floor(cand[2] / cs));
    for dz in -r..=r {
        for dx in -r..=r {
            let (x, z) = (cx + dx as f64, cz + dz as f64);
            if x < 0.0 || z < 0.0 || x >= w_cells || z >= h_cells {
                continue;
            }
            let i = crate::scene_collision::nav_grid_idx(g, x, z) as usize;
            if g.col.nav.blocked[i] != 0 {
                continue;
            }
            let q: V3 = [(x + 0.5) * cs, 0.0, (z + 0.5) * cs];
            let d = dist_xz(q, cand);
            if d > LONE_COVER_R || dist_xz(q, fix) < LONE_HOLD_MIN_M {
                continue;
            }
            let w = walls_about(g, q);
            if w < 2.0 {
                continue;
            }
            if !(w > bw || (w == bw && !best_is_cand && d < bd)) {
                continue; // more wall about it wins; as much, the nearer
            }
            let can_step = crate::scene_collision::nav_grid_walkable(g, cand, q) || !crate::scene_collision::solid_blocked(g, [cand[0], 0.5, cand[2]], [q[0], 0.5, q[2]], None);
            if !can_step || !los_open(g, q, fix) {
                continue;
            }
            best = q;
            best_is_cand = false;
            bw = w;
            bd = d;
        }
    }
    LoneCorner { p: best }
}

/// The call over the net, once per hold; his side comes
fn lone_call(g: &mut crate::game::Game, gd: GuardId, l: &mut crate::tactics::Lone) {
    s(g);
    l.called = true;
    s(g).last_lone_call_t = g.time;
    let place = crate::dialogue::place_name(g, l.fix);
    // (WORDS.on is settled on)
    let line = if crate::teams::foe_is_player(g, gd) && g.gd[gd].unit.is_none() {
        alloc::format!("he's armed — {}. holding my corner, get a second man to me", place)
    } else {
        alloc::format!("armed contact, {} — holding here, need another gun on it", place)
    };
    crate::game::game_say(g, gd, &line, Some(true), None);
    let at = head_or_pos(g, gd);
    crate::audio_iface::game_audio_like_play(g, "radio", Some(at), Some(0.7), None);
    let team = g.team(gd);
    let list = g.guards.clone();
    for o in list {
        let man = &g.gd[o];
        if o == gd || g.cond[man.ch].down || man.held.is_some() || man.hold || man.pinned || g.ch[man.ch].team != team || man.script.is_some() {
            continue;
        }
        let was_calm = matches!(man.state, crate::game::GuardState::Patrol | crate::game::GuardState::Suspicious | crate::game::GuardState::Search);
        crate::mind::raise(g, o, 0.92, "tactics");
        if was_calm || g.gd[o].last_known.is_none() {
            crate::belief::word(g, o, l.fix);
        }
        if was_calm {
            let near = m::min(1.2, dist_xz(g.pos_of(o), g.pos_of(gd)) / 25.0);
            let die = crate::dice::draw_of(g, "tactics::lone_call#1", gd);
            g.gd[o].react_t = m::max(g.gd[o].react_t, 0.25 + near + die * 0.4);
        }
        // (a unit with nobody yet takes his colleague's man)
        if g.gd[o].foe.is_none() {
            if let Some(foe) = crate::teams::foe_guard(g, gd) {
                if !crate::rails::rails_down(g, foe) {
                    let c = g.gd[foe].ch;
                    crate::mind::foe(g, o, Some(c), "tactics");
                }
            }
        }
    }
}

/// The lone man's move: to his corner, the hold, then on alone at a walk
fn move_lone(g: &mut crate::game::Game, gd: GuardId, l: &mut crate::tactics::Lone, dt: f64) -> Option<crate::tactics::TacMove> {
    let c = g.gd[gd].ch;
    let mut out = TacMove { move_: 0.0, aim: None, pied: false, crouch: 0.0 };
    if l.released {
        // on alone, carefully: the chase as it always was, at a walk
        let walk = g.tune.guard_walk;
        let r = crate::guards::chase_fix(g, gd, dt, walk);
        return Some(TacMove { move_: r.move_, aim: r.aim, pied: r.pied, crouch: 0.0 });
    }
    let hold = l.hold_at.unwrap();
    if l.hold_t <= 0.0 && dist_xz(g.ch[c].pos, hold) > 0.2 {
        let pace = g.tune.guard_investigate;
        let there = advance(g, gd, dt, hold, pace, &mut out);
        if there || crate::guards::not_getting_anywhere(g, gd, dt, ROLE_STALL_SECS) {
            l.hold_at = Some(v3::copy(g.ch[c].pos));
            l.hold_t = 1e-3;
            g.gd[gd].drop_path();
        }
        aim_at_fix(&mut out, l.fix);
        return Some(out);
    }
    if l.hold_t <= 0.0 {
        l.hold_t = 1e-3;
        g.gd[gd].drop_path();
    }
    kneel_toward(g, gd, dt, l.fix, &mut out, None);
    l.hold_t += dt;
    if !l.called && l.hold_t > 0.4 {
        lone_call(g, gd, l);
    }
    if l.hold_t >= g.tune.lone_wait || (l.hold_t > 1.0 && !alone(g, gd)) {
        l.released = true;
        g.gd[gd].drop_path();
        crate::guards::reset_chase(g, gd);
    }
    Some(out)
}

/// moveLone on a lent row, written back into `st.lone` after it
fn move_lone_row(g: &mut Game, gd: GuardId, mut l: Lone, dt: f64) -> Option<TacMove> {
    let out = move_lone(g, gd, &mut l, dt);
    if let Some(row) = s(g).lone.get_mut(gd) {
        *row = l;
    }
    out
}

/// guards.ts, the alert branch with a fix and no eyes on his man: his role, else the lone man's angle, else null = the chase
pub fn tactics_alert(g: &mut crate::game::Game, gd: GuardId, dt: f64) -> Option<crate::tactics::TacMove> {
    if !tactics_on(g) || g.gd[gd].last_known.is_none() || g.gd[gd].pinned || g.gd[gd].hold {
        return None;
    }
    s(g);
    if let Some(&role) = s(g).roles.get(gd) {
        if g.tab.tactics_plan[g.tab.tactics_role[role].plan].phase != PlanPhase::Done {
            return Some(move_role(g, gd, role, dt));
        }
    }
    if s(g).gates.has(gd) {
        // on a kill-zone doorway's drill: the gate inside followPath has his feet; nothing else of ours meanwhile
        s(g).lone.delete(gd);
        let chase = g.tune.guard_chase;
        let r = crate::guards::chase_fix(g, gd, dt, chase);
        return Some(TacMove { move_: r.move_, aim: r.aim, pied: r.pied, crouch: r.crouch });
    }
    // the lone man: alert alone on an armed man's fix more than LONE_FIGHT_M off
    let fix = g.gd[gd].last_known.unwrap();
    let mut row = s(g).lone.get(gd).cloned();
    if let Some(l) = &row {
        if dist_xz(l.fix, fix) > REPLAN_M {
            s(g).lone.delete(gd); // (a new place: a new corner, or none)
            row = None;
        }
    }
    if let Some(mut l) = row.clone() {
        if l.released && !crate::teams::foe_down(g, gd) {
            // going on carefully: at a walk all the way in
            if dist_xz(l.fix, fix) > 0.05 {
                l.fix = v3::copy(fix);
            }
            return move_lone_row(g, gd, l, dt);
        }
    }
    let applies = armed_intruder(g, &crate::seats::here(g), gd) && dist_xz(g.pos_of(gd), fix) > LONE_FIGHT_M && !crate::teams::foe_down(g, gd) && (alone(g, gd) || row.is_some());
    if !applies {
        if row.is_some() {
            s(g).lone.delete(gd);
        }
        return None;
    }
    let l = match row {
        None => {
            let cn = lone_corner(g, gd, fix);
            let l = Lone { fix: v3::copy(fix), hold_at: Some(cn.p), hold_t: 0.0, called: false, released: false };
            s(g).lone.set(gd, l.clone());
            g.gd[gd].drop_path();
            g.gd[gd].stall_ref = None;
            l
        }
        Some(mut l) => {
            if dist_xz(l.fix, fix) > 0.05 {
                l.fix = v3::copy(fix);
            }
            l
        }
    };
    move_lone_row(g, gd, l, dt)
}

/// Once a frame, before the guards move: expire what has lapsed, read this frame's rounds, keep the plans and gates honest, deal new plans.
pub fn tactics_tick(g: &mut crate::game::Game) {
    if !tactics_on(g) {
        let busy = g.m.tactics.states.as_ref().map_or(false, |st| !st.plans.is_empty() || st.roles.len() > 0 || st.lone.len() > 0 || st.gates.len() > 0);
        if busy {
            for plan in s(g).plans.clone() {
                end_plan(g, plan, None);
            }
            let st = s(g);
            st.roles.clear();
            st.lone.clear();
            st.gates.clear();
        }
        return;
    }
    s(g);
    // housekeeping: men no longer in the list, records past their time
    for m in s(g).lone.keys() {
        if !g.guards.contains(&m) || g.gd[m].state != crate::game::GuardState::Alert || s(g).roles.has(m) {
            s(g).lone.delete(m);
        }
    }
    for m in s(g).cleared.keys() {
        if !g.guards.contains(&m) || crate::condition::down(g, g.gd[m].ch) {
            s(g).cleared.delete(m);
        }
    }
    let now = g.time;
    let recs = &g.tab.tactics_danger_rec;
    g.m.tactics.states.as_mut().unwrap().danger.retain(|r| recs[*r].until > now || now - recs[*r].until < 60.0);
    scan_rounds(g);
    tick_gates(g);
    tick_plans(g);
}

/// how far along hinge → latch
pub(super) fn along_of(g: &crate::game::Game, d: DoorId, p: V3) -> f64 {
    let f = crate::guards::frame_of(g, d);
    (p[0] - f.hx) * f.ux + (p[2] - f.hz) * f.uz
}

/// a point in the door's frame: `a` along hinge → latch, `b` out from the frame line on side sg
pub(super) fn door_pt(g: &crate::game::Game, d: DoorId, a: f64, b: f64, sg: f64, y: Option<f64>) -> V3 {
    let y = y.unwrap_or(0.0);
    let f = crate::guards::frame_of(g, d);
    [f.hx + f.ux * a + f.nx * sg * b, y, f.hz + f.uz * a + f.nz * sg * b]
}

/// the level's clear plan for this door from side sg with its stack past the jamb at `near_a`
pub(super) fn plan_for(g: &crate::game::Game, d: DoorId, sg: f64, near_a: f64) -> Option<crate::scene_level::RoomClearPlan> {
    let n = g.level.clear_plans.as_ref().map_or(0, |v| v.len());
    for i in 0..n {
        let (same_door, stack0) = {
            let p = &g.level.clear_plans.as_ref().unwrap()[i];
            (p.door == g.dr[d].def.name, p.stack[0])
        };
        if !same_door {
            continue;
        }
        if crate::handleside::side_at(g, d, stack0) != sg {
            continue;
        }
        let a = along_of(g, d, stack0);
        if (near_a <= 0.0) != (a < g.dr[d].def.width / 2.0) {
            continue;
        }
        return Some(g.level.clear_plans.as_ref().unwrap()[i].clone());
    }
    None
}

/// where the k-th man (0 lead, 1 second) stacks, and the facing there
fn stack_spot(g: &crate::game::Game, d: DoorId, sg: f64, near_a: f64, k: f64, plan: Option<&crate::scene_level::RoomClearPlan>) -> crate::tactics::StackSpot {
    let out = if near_a <= 0.0 { -1.0 } else { 1.0 }; // which way along the frame line the wall runs from the near jamb
    let jamb = door_pt(g, d, near_a, STACK_OFF_M, sg, None);
    let face_of = |p: V3| v3::yaw_to(p, jamb);
    if let Some(plan) = plan {
        let p = plan.stack[k as usize];
        if !crate::scene_collision::nav_grid_is_blocked(g, p[0], p[2]) {
            return StackSpot { pos: v3::copy(p), face: plan.stack_face[k as usize] };
        }
    }
    for along in [STACK_ALONG_M, 1.2, 0.7, 1.6] {
        for off in [STACK_OFF_M, 0.8, 1.05] {
            let p = door_pt(g, d, near_a + out * (along + k * STACK_GAP_M), off, sg, None);
            if !crate::scene_collision::nav_grid_is_blocked(g, p[0], p[2]) {
                return StackSpot { pos: p, face: face_of(p) };
            }
        }
    }
    let f = crate::guards::frame_of(g, d);
    let p = crate::scene_collision::nav_grid_nearest_free_point(g, jamb[0] + out * f.ux * (1.0 + k), jamb[2] + out * f.uz * (1.0 + k)).unwrap_or(jamb);
    let pos = [p[0], 0.0, p[2]];
    StackSpot { pos, face: face_of(pos) }
}

/// the slice: stations across the opening as far back as the floor allows, each with the point in the room his muzzle is on
pub(super) fn slice_stations(g: &crate::game::Game, d: DoorId, sg: f64, near_a: f64) -> Option<crate::tactics::SliceStations> {
    let w = g.dr[d].def.width;
    let dir = if near_a <= 0.0 { 1.0 } else { -1.0 };
    let a0 = near_a - dir * 0.2;
    let span = w + 0.4;
    for b in slice_back() {
        let mut pts: Vec<V3> = Vec::new();
        let mut ok = true;
        let mut k = 0.0;
        while k <= SLICE_STEPS && ok {
            let p = door_pt(g, d, a0 + dir * span * (k / SLICE_STEPS), b, sg, None);
            let shut = crate::scene_collision::nav_grid_is_blocked(g, p[0], p[2]) || (!pts.is_empty() && !crate::scene_collision::nav_grid_walkable(g, pts[pts.len() - 1], p));
            if shut {
                ok = false;
            } else {
                pts.push(p);
            }
            k += 1.0;
        }
        if !ok {
            continue;
        }
        let mut look = Vec::with_capacity(pts.len());
        for &p in &pts {
            let a = along_of(g, d, p);
            look.push(door_pt(g, d, w / 2.0 + (w / 2.0 - a) * 0.8, -2.2, sg, Some(1.1)));
        }
        return Some(SliceStations { pts, look });
    }
    None
}

/// the entry legs for man k: the plan's beyond the frame line, else a stride and a half in and a leg to his side
pub(super) fn entry_legs(g: &crate::game::Game, d: DoorId, sg: f64, near_a: f64, k: f64, plan: Option<&crate::scene_level::RoomClearPlan>) -> Vec<V3> {
    let f = crate::guards::frame_of(g, d);
    let w = g.dr[d].def.width;
    if let Some(plan) = plan {
        let mut legs = Vec::new();
        for &p in &plan.entry[k as usize] {
            if ((p[0] - f.hx) * f.nx + (p[2] - f.hz) * f.nz) * sg < -0.2 && !crate::scene_collision::nav_grid_is_blocked(g, p[0], p[2]) {
                legs.push(v3::copy(p));
            }
        }
        if !legs.is_empty() {
            return legs;
        }
    }
    let p1 = door_pt(g, d, w / 2.0, -ENTRY_IN_M, sg, None);
    let (inx, inz) = (-f.nx * sg, -f.nz * sg);
    let toward = (if near_a <= 0.0 { 1.0 } else { -1.0 }) * (if k == 0.0 { 1.0 } else { -1.0 }); // + = toward the latch end of the frame
    let ang = ENTRY_SPLIT_DEG * v3::DEG;
    let (ca, sa) = (m::cos(ang), m::sin(ang));
    let mut cands: Vec<V3> = [1.0, -1.0].iter().map(|&s| [p1[0] + (inx * ca - inz * sa * s) * ENTRY_LEG_M, 0.0, p1[2] + (inx * sa * s + inz * ca) * ENTRY_LEG_M]).collect();
    js::sort_by(&mut cands, |p, q| ((q[0] - p1[0]) * f.ux + (q[2] - p1[2]) * f.uz) * toward - ((p[0] - p1[0]) * f.ux + (p[2] - p1[2]) * f.uz) * toward);
    let mut legs: Vec<V3> = Vec::new();
    if !crate::scene_collision::nav_grid_is_blocked(g, p1[0], p1[2]) {
        legs.push(p1);
    } else if let Some(q) = crate::scene_collision::nav_grid_nearest_free_point(g, p1[0], p1[2]) {
        legs.push([q[0], 0.0, q[2]]);
    }
    for p2 in cands {
        if !crate::scene_collision::nav_grid_is_blocked(g, p2[0], p2[2]) && (legs.is_empty() || crate::scene_collision::nav_grid_walkable(g, legs[legs.len() - 1], p2)) {
            legs.push(p2);
            break;
        }
    }
    legs
}

/// Step him along a local path to `target` at `speed`; true once there.
pub(super) fn step_gate(g: &mut crate::game::Game, gd: GuardId, gate: &mut crate::tactics::Gate, dt: f64, target: V3, speed: f64, face: Option<f64>) -> bool {
    let c = g.gd[gd].ch;
    let speed = crate::condition::condition_pace(g, g.gd[gd].ch, speed);
    // (the TypeScript asks `pathFor !== target` of the array; every caller hands a held point, so its numbers say the same)
    if gate.path.is_none() || gate.path_for != Some(target) {
        let from = g.ch[c].pos;
        let mut path = crate::scene_collision::nav_grid_find_path(g, from, target, None, None).unwrap_or_else(|| vec![v3::copy(target)]);
        if path.last().map_or(true, |last| dist_xz(*last, target) > 0.05) {
            path.push(v3::copy(target));
        }
        gate.path = Some(path);
        gate.path_i = 0.0;
        gate.path_for = Some(target);
    }
    let path_len = gate.path.as_ref().unwrap().len();
    while (gate.path_i as usize) < path_len {
        let i = gate.path_i as usize;
        let wp = gate.path.as_ref().unwrap()[i];
        let p = g.ch[c].pos;
        let (dx, dz) = (wp[0] - p[0], wp[2] - p[2]);
        let d = m::hyp2(dx, dz);
        if d < (if i == path_len - 1 { 0.12 } else { 0.4 }) {
            gate.path_i += 1.0;
            continue;
        }
        g.gd[gd].speed = v3::damp(g.gd[gd].speed, speed * v3::clamp(d / 0.6, 0.3, 1.0), 6.0, dt);
        let now_speed = g.gd[gd].speed;
        let step = m::min(d, now_speed * dt);
        g.ch[c].pos[0] += dx / d * step;
        g.ch[c].pos[2] += dz / d * step;
        g.ch[c].vel = [dx / d * now_speed, 0.0, dz / d * now_speed];
        g.ch[c].body_yaw = v3::approach_angle(g.ch[c].body_yaw, face.unwrap_or_else(|| m::atan2(dx, dz)), 6.0 * dt);
        break;
    }
    let mut arrived = false;
    if (gate.path_i as usize) >= path_len {
        arrived = true;
        g.gd[gd].speed = v3::damp(g.gd[gd].speed, 0.0, 10.0, dt);
        g.ch[c].vel = [0.0, 0.0, 0.0];
        if let Some(face) = face {
            g.ch[c].body_yaw = v3::approach_angle(g.ch[c].body_yaw, face, 5.0 * dt);
        }
    }
    arrived || dist_xz(g.ch[c].pos, target) < 0.15
}

/// stood still on the mark, turning to `face`
pub(super) fn stand_gate(g: &mut crate::game::Game, gd: GuardId, dt: f64, face: Option<f64>) {
    let c = g.gd[gd].ch;
    g.gd[gd].speed = v3::damp(g.gd[gd].speed, 0.0, 10.0, dt);
    g.ch[c].vel = [0.0, 0.0, 0.0];
    if let Some(face) = face {
        g.ch[c].body_yaw = v3::approach_angle(g.ch[c].body_yaw, face, 5.0 * dt);
    }
}

/// the other man on this doorway for this side, by role (a copy of his gate)
pub(super) fn gate_mate(g: &mut crate::game::Game, gate: &crate::tactics::Gate, role: crate::tactics::GateRole, except: GuardId) -> Option<crate::tactics::GateMate> {
    s(g);
    let want = role;
    let team = g.tab.tactics_danger_rec[gate.rec].team;
    let st = g.m.tactics.states.as_ref().unwrap();
    for (o, h) in st.gates.iter() {
        if o != except && h.door == gate.door && g.tab.tactics_danger_rec[h.rec].team == team && h.role == want && !crate::condition::down(g, g.gd[o].ch) {
            return Some(GateMate { gd: o, g: h.clone() });
        }
    }
    None
}

/// The lead's call, once per danger record, and the summons of the nearest free man of his side
pub(super) fn gate_call(g: &mut crate::game::Game, gd: GuardId, gate: &mut crate::tactics::Gate) {
    let rec = gate.rec;
    if g.tab.tactics_danger_rec[rec].called { return; }
    g.tab.tactics_danger_rec[rec].called = true;
    let line = call_line(g, gate);
    crate::game::game_say(g, gd, &line, Some(true), None);
    let radio_at = head_or_pos(g, gd);
    crate::audio_iface::game_audio_like_play(g, "radio", Some(radio_at), Some(0.7), None);
    let Some(best) = whom_to_call(g, gd, gate.door) else { return };
    let marks = plan_for(g, gate.door, gate.side, gate.near_a);
    let spot = stack_spot(g, gate.door, gate.side, gate.near_a, 1.0, marks.as_ref()).pos;
    comes_to(g, best, spot);
    g.tab.tactics_danger_rec[rec].summoned = Some(best);
    let name_said = crate::teams::spoken_name(g, gd).map_or(false, |n| !n.is_empty());
    let answer = if name_said { alloc::format!("copy — coming to you, {}", g.gd[gd].callsign) } else { String::from("copy — coming to you") };
    crate::game::game_say(g, best, &answer, None, None);
}
/// What he says of that doorway, and why.
fn call_line(g: &Game, gate: &Gate) -> String {
    let door = door_in_words(g, gate.door);
    let (why, named) = (g.tab.tactics_danger_rec[gate.rec].why, g.tab.tactics_danger_rec[gate.rec].victim.clone());
    if why == crate::tactics::DangerWhy::Rounds { return alloc::format!("taking rounds through {} — nobody through it, on the wall with me", door); }
    let man = named.as_ref().filter(|v| !v.is_empty()).and_then(|v| g.guards.iter().copied().find(|&o| g.gd[o].callsign == *v));
    let victim = match man {
        Some(m) => crate::teams::spoken_name(g, m),
        None => named,
    };
    match victim.as_deref() {
        Some(v) if !v.is_empty() => {
            let place = if !door.ends_with("door") { door.clone() } else { alloc::format!("{}way", door) };
            alloc::format!("{}'s down in {} — hold short of it, stack on me", v, place)
        }
        _ => alloc::format!("man down at {} — hold, don't go through", door),
    }
}
/// The doorway as they say it.
fn door_in_words(g: &Game, door: DoorId) -> String {
    if crate::dialogue::is_office(g) {
        let wrote = crate::doornames::door_written_name(g, door);
        return match wrote.as_deref() {
            Some(w) if !w.is_empty() => alloc::format!("the {}", w),
            _ => alloc::format!("the {} door", door_label(g, door)),
        };
    }
    let centre = g.dr[door].frame_centre;
    let at = crate::dialogue::own_place(g, centre);
    let said = match crate::doornames::door_said_name(g, door) {
        Some(n) => Some(n),
        None if crate::doornames::shorthand_id(&g.dr[door].def.name) => None,
        None => Some(crate::orders::door_words(g, door)),
    };
    match (said.as_deref(), &at) {
        (Some(s), _) if !s.is_empty() => alloc::format!("the {}", s),
        (_, Some(a)) if a.d < 1.5 => alloc::format!("{} door", a.rect.name),
        _ => String::from("this door"),
    }
}
/// The nearest free man of his side. (A man fighting his own fight elsewhere keeps it)
fn whom_to_call(g: &mut Game, gd: GuardId, door: DoorId) -> Option<GuardId> {
    let (team, here, centre) = (g.team(gd), g.pos_of(gd), g.dr[door].frame_centre);
    let mut best: Option<GuardId> = None;
    let mut bd = SUMMON_M;
    for o in g.guards.clone() {
        if o == gd || crate::condition::down(g, g.gd[o].ch) || g.team(o) != team || !free_man(g, o) || s(g).gates.has(o) || s(g).roles.has(o) || in_contact(g, o) { continue; }
        if g.gd[o].state == crate::game::GuardState::Alert && g.gd[o].last_known.is_some_and(|lk| dist_xz(lk, centre) > 6.0) { continue; }
        let dd = dist_xz(g.pos_of(o), here);
        if dd < bd {
            bd = dd;
            best = Some(o);
        }
    }
    best
}
/// He comes, wary, to that mark: whatever he was at is over.
fn comes_to(g: &mut Game, o: GuardId, spot: V3) {
    use crate::game::GuardState as S;
    crate::mind::fix(g, o, Some(v3::copy(spot)), "tactics");
    crate::mind::raise(g, o, 0.5, "tactics");
    let man = &mut g.gd[o];
    man.search_t = 0.0;
    man.path = Vec::new();
    man.path_serial = man.path_serial.wrapping_add(1);
    man.path_goal = None;
    man.path_goal_orbit = false;
    man.stall_ref = None;
    man.body_duty = None;
    man.search_plan = None;
    man.task = None;
    match man.state {
        S::Patrol => {
            crate::mind::set_state(g, o, S::Suspicious, "tactics");
            g.gd[o].react_t = 0.5;
        }
        S::Search => {
            crate::mind::set_state(g, o, S::Suspicious, "tactics");
            g.gd[o].react_t = 0.3;
            g.gd[o].pieing = None; // (his own update may already have run this frame: the doorway he was slicing is let go of here)
        }
        _ => g.gd[o].react_t = m::max(g.gd[o].react_t, 0.3),
    }
}

/// a man's way to a dangerous doorway: the side of it he is on, and whether he is going to it and not through
#[derive(Clone, Copy)]
struct Nears { side: f64, goal_near: bool }

/// Does this man's way take him through (or up to) a doorway dangerous to his side he has not yet taken as a drill: the gate has him.
fn try_engage(g: &mut crate::game::Game, gd: GuardId, recs: &[TacticsDangerRecId]) -> Option<crate::tactics::Gate> {
    s(g);
    for &rec in recs {
        let d = g.tab.tactics_danger_rec[rec].door;
        if cleared_for(g, gd, d) { continue; }
        let Some(n) = nears(g, gd, rec) else { continue };
        let Some(gate) = gate_at(g, gd, rec, n) else { continue };
        s(g).gates.set(gd, gate.clone());
        return Some(gate);
    }
    None
}
/// HIS WAY TAKES HIM TO IT: the rest of his path crosses it within GATE_DETECT_M of walk, or ends within GATE_GOAL_M of its middle a few strides ahead of him, or he is the man the
/// lead's call fetched, coming up on the marks.
fn nears(g: &Game, gd: GuardId, rec: TacticsDangerRecId) -> Option<Nears> {
    let d = g.tab.tactics_danger_rec[rec].door;
    let centre = g.dr[d].frame_centre;
    let here = g.pos_of(gd);
    let (mut acc, mut prev) = (0.0, here);
    let mut i = g.gd[gd].path_i;
    while i < g.gd[gd].path.len() && acc <= GATE_DETECT_M {
        let wp = g.gd[gd].path[i];
        let l = dist_xz(prev, wp);
        if let Some(x) = crate::guards::crosses_door(g, prev, wp, d) {
            if acc + l * x.t <= GATE_DETECT_M { return Some(Nears { side: x.side, goal_near: false }); }
        }
        acc += l;
        prev = wp;
        i += 1;
    }
    let to_it = g.gd[gd].path_goal.is_some_and(|goal| dist_xz(goal, centre) < GATE_GOAL_M && dist_xz(here, goal) < GATE_DETECT_M + 1.0 && dist_xz(here, centre) < GATE_DETECT_M + GATE_GOAL_M);
    let fetched = g.tab.tactics_danger_rec[rec].summoned == Some(gd) && dist_xz(here, centre) < GATE_DETECT_M + 2.5;
    if !to_it && !fetched { return None; }
    Some(Nears { side: crate::handleside::side_at(g, d, here), goal_near: true })
}
/// His gate there: behind the man of his side who leads on his side of it, or the lead. (None: lanes.rs stops him short, that jamb or the floor beyond the doorway being in the line of guns his side knows of)
fn gate_at(g: &mut Game, gd: GuardId, rec: TacticsDangerRecId, n: Nears) -> Option<Gate> {
    let d = g.tab.tactics_danger_rec[rec].door;
    let width = g.dr[d].def.width;
    let lead = lead_there(g, gd, d);
    let (role, side, near_a) = match lead {
        Some((_, side, near_a)) => (GateRole::Second, side, near_a),
        None => (GateRole::Lead, n.side, near_jamb(g, gd, d, n.side)),
    };
    let plan = plan_for(g, d, side, near_a);
    let ss = stack_spot(g, d, side, near_a, if role == GateRole::Lead { 0.0 } else { 1.0 }, plan.as_ref());
    if crate::lanes::lane_covers(g, gd, ss.pos) { return None; }
    let far = door_pt(g, d, width / 2.0, 1.2, -side, None);
    let has_can = stuns_of(g, gd) > 0.0;
    let centre = g.dr[d].frame_centre;
    if crate::lanes::lane_door(g, gd, far, centre, has_can) { return None; }
    Some(Gate {
        door: d, rec, phase: GatePhase::Stack, t0: g.time, side, near_a, spot: ss.pos, face: ss.face, role, lead: lead.map(|l| l.0), cross_at: -1.0, toss_t: -1.0, land_t: -1.0, bang_t: -1.0,
        goal_near: n.goal_near, action: GateAction::Slice, t: 0.0, slice_pts: Vec::new(), slice_look: Vec::new(), slice_i: 0.0, slice_t: 0.0, legs: Vec::new(), leg_i: 0.0, look: None, crouch: 0.0,
        path: None, path_i: 0.0, path_for: None, can: None, opened: false, crossed: false, hs: None, thrower: None,
    })
}
/// The man of his side who leads at that doorway, on the side of it he is on himself, and has not gone in. (A lead on the FAR side of it is no man to stack behind: this man leads his own side)
fn lead_there(g: &Game, gd: GuardId, d: DoorId) -> Option<(GuardId, f64, f64)> {
    let at = g.pos_of(gd);
    let (team, here) = (g.team(gd), crate::handleside::side_at(g, d, at));
    let st = g.m.tactics.states.as_ref().unwrap();
    let found = st.gates.iter().find(|(o, h)| {
        *o != gd && h.door == d && g.tab.tactics_danger_rec[h.rec].team == team && h.role == GateRole::Lead && !crate::condition::down(g, g.gd[*o].ch) && h.phase != GatePhase::Cross
            && h.phase != GatePhase::Done && h.side == here
    });
    found.map(|(o, h)| (o, h.side, h.near_a))
}
/// The jamb he comes to first, not one a leaf's latch side keeps him from.
fn near_jamb(g: &Game, gd: GuardId, d: DoorId, side: f64) -> f64 {
    let width = g.dr[d].def.width;
    let at = g.pos_of(gd);
    if along_of(g, d, at) >= width / 2.0 { return width; }
    let stack = door_pt(g, d, width + STACK_ALONG_M, STACK_OFF_M, side, None);
    if crate::handleside::latch_first(g, d, stack) { width } else { 0.0 }
}

/// guards.ts followPath, first thing: true = the gate has this man's feet this frame. The gate is lent to runGate and written back.
pub fn door_gate(g: &mut crate::game::Game, gd: GuardId, dt: f64) -> bool {
    if !tactics_on(g) {
        return false;
    }
    {
        let m = &g.gd[gd];
        if m.script.is_some() || m.held.is_some() || m.hold || m.pinned || g.cond[m.ch].down || (m.state == crate::game::GuardState::Patrol && !m.drawn) {
            return false;
        }
    }
    s(g);
    // (the row is lent to runGate and written back: Gate has no ID, and gateMate must still find it in the table)
    let mut gate = match s(g).gates.get(gd).cloned() {
        Some(gate) => gate,
        None => {
            let recs = danger_doors(g, Some(g.team(gd)));
            if recs.is_empty() || in_contact(g, gd) {
                return false;
            }
            match try_engage(g, gd, &recs) {
                Some(gate) => gate,
                None => return false,
            }
        }
    };
    let held = run_gate(g, gd, &mut gate, dt);
    // (a release inside took his row off: nothing to write back to)
    if let Some(row) = s(g).gates.get_mut(gd) {
        *row = gate;
    }
    held
}

/// a charge in the air or on the floor when its thrower leaves the drill goes off on its own short fuse
pub(super) fn loose_can(g: &mut crate::game::Game, gate: &mut crate::tactics::Gate) {
    if let Some(can) = gate.can {
        if !g.tab.item[can].fired {
            let fuse = g.tab.item[can].fuse;
            g.tab.item[can].fuse = m::min(fuse, TOSS_FUSE);
        }
    }
}

/// a man off his gate, what he has done of the drill counted on the doorway's record
pub(super) fn release_gate(g: &mut crate::game::Game, gd: GuardId, gate: &mut crate::tactics::Gate) {
    s(g).gates.delete(gd);
    loose_can(g, gate);
    let door = gate.door;
    let rec = gate.rec;
    if gate.phase != GatePhase::Stack {
        let until = g.tab.tactics_danger_rec[rec].until;
        set_cleared(g, gd, door, m::max(until, g.time) + 1.0);
    }
    if gate.role == GateRole::Lead && !gate.crossed && (gate.phase == GatePhase::Slice || gate.phase == GatePhase::Done) {
        // looked into and not taken: the man stacked behind him has seen it sliced from a metre away
        if let Some(mate) = gate_mate(g, gate, crate::tactics::GateRole::Second, gd) {
            if mate.g.phase == GatePhase::Stacked {
                s(g).gates.delete(mate.gd);
                let until = g.tab.tactics_danger_rec[rec].until;
                set_cleared(g, mate.gd, door, m::max(until, g.time) + 1.0);
                g.gd[mate.gd].drop_path();
                g.gd[mate.gd].stall_ref = None;
                let mc = g.gd[mate.gd].ch;
                g.ch[mc].anim.crouch_target = 0.0;
            }
        }
    }
    if gate.crossed {
        let now = g.time;
        let row = &mut g.tab.tactics_danger_rec[rec];
        row.until = if row.banged { m::min(row.until, now) } else { m::min(row.until, now + 6.0) };
    }
    g.gd[gd].drop_path();
    g.gd[gd].stall_ref = None;
    let c = g.gd[gd].ch;
    g.ch[c].anim.crouch_target = 0.0;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cells(a: &[f64]) -> JsSet<crate::Num> {
        let mut s = JsSet::new();
        for &x in a {
            s.add(crate::Num(x));
        }
        s
    }

    #[test]
    fn a_cell_costs_the_dearer_of_its_two_lists() {
        // a grid 4 wide: cell (1, 1) is 5, (2, 1) is 6, (3, 1) is 7
        let p = TacticsPenalty::Cells { w: 4.0, door_idx: Some(cells(&[5.0, 7.0])), door_mult: 20.0, avoid: Some(cells(&[6.0, 7.0])) };
        assert_eq!(tactics_penalty_price(&p, 1.0, 1.0), 20.0);
        assert_eq!(tactics_penalty_price(&p, 2.0, 1.0), CORRIDOR_COST);
        assert_eq!(tactics_penalty_price(&p, 3.0, 1.0), 20.0);
        assert_eq!(tactics_penalty_price(&p, 0.0, 0.0), 1.0);
    }

    #[test]
    fn a_price_with_no_lists_is_plain_floor() {
        let p = TacticsPenalty::Cells { w: 4.0, door_idx: None, door_mult: 20.0, avoid: None };
        assert_eq!(tactics_penalty_price(&p, 3.0, 2.0), 1.0);
    }
}
