//! src/game/squad.ts lines 449 to 665
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// `this` of a Clearing method: the one at `g.clearing`; every caller has just tested it, and nothing a method calls clears it.
macro_rules! cl { ($g:ident) => { $g.clearing.as_mut().unwrap() } }

/// seam: squad.ts:616 g.audio.play('radio', at, 0.35, { rate: 1.1 }): `opts` of game_audio_like_play holds `crate::Opaque`, which cannot carry a rate
fn seam_audio_play_rate(g: &mut Game, name: &str, at: V3, gain: f64, rate: f64) { todo!("needs: game_audio_like_play's opts to carry a rate") }

/// g.blackout.active (game.ts blackout, lightshome.ts Mains.active)
fn blackout_active(g: &mut Game) -> bool {
    crate::sim_lightshome::mains_active(g)
}
/// gd.task.pos, which every arm of the union has
fn task_pos(t: &crate::game::GuardTask) -> V3 {
    match t {
        crate::game::GuardTask::Arm0(x) => x.pos,
        crate::game::GuardTask::CheckOn(x) => x.pos,
    }
}
/// a man's head, or `pos` when no pose has written it (`c.bones.head ?? …`)
fn bone_or(g: &Game, c: CharId, bone: crate::character::Bone, or: V3) -> V3 {
    match g.ch[c].bones.get(bone) { Some(p) => *p, None => or }
}
/// Clearing.name's body, with no `g`: a plan is often borrowed out of `g` when it is named
fn plan_name(p: Option<&crate::scene_level::RoomClearPlan>) -> String {
    match p {
        Some(p) => match &p.room { Some(r) => r.clone(), None => p.door.clone() },
        None => String::from("room"),
    }
}
/// the tick's `rank` table
fn rank(r: Perceive) -> i32 {
    match r { Perceive::Alert => 3, Perceive::Body => 2, Perceive::Suspicious => 1 }
}

/// The senses updateGuard would have run for a man the runner drives: what they came to this frame. (squad.ts perceive)
pub fn perceive(g: &mut crate::game::Game, gd: GuardId, dt: f64, before: f64) -> Option<crate::squad::Perceive> {
    crate::guards::hearing_check(g, gd, None, None, None);
    let vis = crate::guards::can_see(g, gd);
    let mut seen = false;
    let pc = g.pl[g.player].ch;
    if vis.visible {
        let rate = crate::guards::sight_gain(g, gd, vis.dist);
        if rate > 0.05 {
            g.gd[gd].awareness = v3::clamp(g.gd[gd].awareness + rate * dt, 0.0, 1.0);
            g.gd[gd].last_known = Some(v3::copy(g.ch[pc].pos));
            seen = rate > 0.25;
            if seen { g.gd[gd].last_seen_t = g.time; }
        }
    }
    g.gd[gd].saw_player_this_frame = seen;
    let pp = g.ch[pc].pos;
    let gp = g.ch[g.gd[gd].ch].pos;
    if !g.pl[g.player].down && !g.player_invisible && m::hyp2(pp[0] - gp[0], pp[2] - gp[2]) < 0.62 {
        g.gd[gd].awareness = 1.0;
        g.gd[gd].last_known = Some(v3::copy(pp));
        g.gd[gd].last_seen_t = g.time;
        seen = true;
    }
    if g.gd[gd].awareness > g.gd[gd].witness.peak_awareness { g.gd[gd].witness.peak_awareness = g.gd[gd].awareness; }
    if g.time - g.gd[gd].saw_body_t > 20.0 && g.gd[gd].awareness < 0.85 {
        if let Some(o) = crate::guards::body_in_view(g, gd) {
            g.gd[gd].awareness = m::max(g.gd[gd].awareness, 0.5);
            let oc = g.gd[o].ch;
            let h = bone_or(g, oc, crate::character::Bone::Hips, g.ch[oc].pos);
            g.gd[gd].last_known = Some([h[0], 0.0, h[2]]);
            return Some(Perceive::Body);
        }
    }
    if !seen { g.gd[gd].awareness = m::max(0.0, g.gd[gd].awareness - dt * 0.06); }
    let aw = g.gd[gd].awareness;
    if aw >= 0.85 { Some(Perceive::Alert) } else if aw >= SUSPECT && aw > before { Some(Perceive::Suspicious) } else { None }
}

/// Is p inside the plan's bounds, grown by pad? (squad.ts inBounds)
pub fn in_bounds(p: V3, b: Option<crate::scene_level::BreachPlanBounds>, pad: Option<f64>) -> bool {
    let pad = pad.unwrap_or(0.0);
    match b {
        Some(b) => p[0] > b.x0 - pad && p[0] < b.x1 + pad && p[2] > b.z0 - pad && p[2] < b.z1 + pad,
        None => false,
    }
}

/// Metres of nav path from a to b, Infinity if there is none. (squad.ts pathLen)
pub fn path_len(g: &mut crate::game::Game, a: V3, b: V3) -> f64 {
    let p = match crate::scene_collision::nav_grid_find_path(g, a, b, None, None) { Some(p) => p, None => return f64::INFINITY };
    let mut l = 0.0;
    let mut q = a;
    for r in p { l += flat(q, r); q = r; }
    l
}

/// The lockdown pair's job, not yet at `g.clearing`: the caller puts it there. (squad.ts Clearing.constructor)
pub fn clearing_new(g: &mut crate::game::Game, rooms: &[Vec<crate::scene_level::RoomClearPlan>], a: GuardId, b: Option<GuardId>) -> crate::squad::Clearing {
    let mut c = Clearing::blank();
    c.rooms = rooms.to_vec();
    c.a = a;
    c.b = b;
    c.dark = blackout_active(g);
    c.cur = rooms.first().and_then(|v| v.first()).cloned();
    c
}

/// Seconds the whole job may take before they are put back on the corridor. (squad.ts Clearing.cap)
pub fn clearing_cap(g: &mut crate::game::Game) -> f64 {
    42.0 * m::max(1.0, cl!(g).rooms.len() as f64) + 15.0
}

/// A plan's name for the lines. (squad.ts Clearing.name)
pub fn clearing_name(g: &mut crate::game::Game, p: Option<&crate::scene_level::RoomClearPlan>) -> String {
    plan_name(p)
}

/// the summary's `who`
fn summary_who(g: &Game, gd: GuardId) -> String {
    match g.level.routes.get(g.gd[gd].route_i) {
        Some(r) => g.tab.route[*r].name.clone(),
        None => format!("guard {}", g.ch[g.gd[gd].ch].id - 10.0),
    }
}

/// One line for the panel: which room, what stage, who, what is next. (squad.ts Clearing.summary)
pub fn clearing_summary(g: &mut crate::game::Game) -> String {
    let c = cl!(g).clone();
    let r = match &c.cur { Some(r) if c.stage != ClearStage::Done => r, _ => return String::new() };
    let next: Vec<String> = c.rooms.iter().skip(c.room_i as usize + 1).map(|v| plan_name(v.first())).collect();
    let next = next.join(", ");
    let by = if c.stage != ClearStage::Muster { format!(" (by the {} door)", r.door) } else { String::new() };
    let what = match (c.stage, c.drill) {
        (ClearStage::Drill, Some(d)) => g.tab.squad_room_clear[d].phase.as_str(),
        _ => c.stage.as_str(),
    };
    let pair = match c.b { Some(b) => format!(" + {}", summary_who(g, b)), None => String::from(" (alone)") };
    let then = if next.is_empty() { String::new() } else { format!(" · then {}", next) };
    format!("clearing: {}{} · {} · {}{}{} · {} s", plan_name(Some(r)), by, what, summary_who(g, c.a), pair, then, m::round(c.total))
}

/// Hand whoever of ours is still scripted back to the AI calm. (squad.ts Clearing.cancel)
pub fn clearing_cancel(g: &mut crate::game::Game) {
    for gd in clearing_men(g) {
        if g.gd[gd].script.is_some() && g.guards.contains(&gd) { end_guard_script(g, gd, Some(EndGuardScriptState::Patrol)); }
    }
    let c = cl!(g);
    c.stage = ClearStage::Done;
    c.drill = None;
}

/// The pair, or the one. (squad.ts Clearing.men)
pub fn clearing_men(g: &mut crate::game::Game) -> Vec<GuardId> {
    let c = cl!(g);
    match c.b { Some(b) => vec![c.a, b], None => vec![c.a] }
}

/// The point man's line, if he lives. (squad.ts Clearing.say)
pub fn clearing_say(g: &mut crate::game::Game, text: &str, radio: Option<bool>) {
    let a = cl!(g).a;
    if g.gd[a].state != crate::game::GuardState::Dead {
        crate::game::game_say(g, a, text, Some(radio.unwrap_or(false)), None, None);
    }
}

/// Once a frame after the guards have moved. False when it is over: the caller drops it. (squad.ts Clearing.tick)
pub fn clearing_tick(g: &mut crate::game::Game, dt: f64) -> bool {
    let (a, b) = { let c = cl!(g); (c.a, c.b) };
    if cl!(g).stage == ClearStage::Done { return false; }
    if !g.guards.contains(&a) || b.map_or(false, |b| !g.guards.contains(&b)) { cl!(g).stage = ClearStage::Done; return false; }
    if g.quiet_utility || !g.ai_enabled || crate::guards::escalation_of(g) < 2.0 || g.pl[g.player].down { clearing_cancel(g); return false; }
    { let c = cl!(g); c.t += dt; c.total += dt; }
    if g.gd[a].state == crate::game::GuardState::Dead || b.map_or(false, |b| g.gd[b].state == crate::game::GuardState::Dead) {
        if cl!(g).stage != ClearStage::Muster {
            let dead = if g.gd[a].state == crate::game::GuardState::Dead { a } else { b.unwrap() };
            let left = if dead == a { b } else { Some(a) };
            if let Some(left) = left {
                if g.gd[left].state != crate::game::GuardState::Dead {
                    end_guard_script(g, left, Some(EndGuardScriptState::Suspicious));
                    g.gd[left].awareness = m::max(g.gd[left].awareness, 0.6);
                    g.gd[left].last_known = Some(v3::copy(g.ch[g.gd[dead].ch].pos));
                    g.gd[left].react_t = 0.7;
                    g.gd[left].search_t = 0.0;
                    crate::game::game_say(g, left, "hey — you with me? …say something.", None, None, None);
                }
            }
        }
        cl!(g).stage = ClearStage::Done;
        return false;
    }
    if cl!(g).stage == ClearStage::Muster { return clearing_muster(g); }

    let mut worst: Option<Perceive> = None;
    let mut fix: Option<V3> = None;
    let mut top = -1.0;
    let men = clearing_men(g);
    for (i, &gd) in men.iter().enumerate() {
        let before = cl!(g).aw[i];
        let r = perceive(g, gd, dt, before);
        cl!(g).aw[i] = g.gd[gd].awareness;
        if let Some(r) = r {
            if worst.map_or(true, |w| rank(r) > rank(w)) { worst = Some(r); }
            if g.gd[gd].awareness > top && g.gd[gd].last_known.is_some() { top = g.gd[gd].awareness; fix = g.gd[gd].last_known; }
        }
    }
    if worst.is_none() {
        let mut i = 0;
        while i < g.guards.len() {
            let o = g.guards[i];
            i += 1;
            if o != a && Some(o) != b && g.gd[o].state == crate::game::GuardState::Alert && g.gd[o].last_known.is_some() { worst = Some(Perceive::Alert); fix = g.gd[o].last_known; break; }
        }
    }
    if worst == Some(Perceive::Alert) {
        for &gd in &men { let to = fix.or(g.gd[gd].last_known); break_off(g, gd, 0.9, to); }
        cl!(g).stage = ClearStage::Done;
        return false;
    }
    if worst == Some(Perceive::Body) {
        for &gd in &men {
            end_guard_script(g, gd, Some(EndGuardScriptState::Suspicious));
            g.gd[gd].awareness = m::max(g.gd[gd].awareness, 0.5);
            if let Some(f) = fix { g.gd[gd].last_known = Some(v3::copy(f)); }
            g.gd[gd].react_t = 0.6;
            g.gd[gd].search_t = 0.0;
        }
        cl!(g).stage = ClearStage::Done;
        return false;
    }
    if worst.is_some() {
        let to = fix.or(g.gd[a].last_known);
        break_off(g, a, SUSPECT + 0.05, to);
        if let Some(b) = b {
            end_guard_script(g, b, Some(EndGuardScriptState::Suspicious));
            g.gd[b].awareness = m::max(g.gd[b].awareness, SUSPECT);
            if let Some(f) = fix { g.gd[b].last_known = Some(v3::copy(f)); }
            g.gd[b].react_t = 1.2;
            g.gd[b].search_t = 0.0;
        }
        cl!(g).stage = ClearStage::Done;
        return false;
    }
    let dark = cl!(g).dark;
    if blackout_active(g) && !dark {
        for &gd in &men {
            let (aw, to) = match &g.gd[gd].task { Some(t) => (0.42, Some(task_pos(t))), None => (0.34, None) };
            break_off(g, gd, aw, to);
        }
        cl!(g).stage = ClearStage::Done;
        return false;
    }
    if cl!(g).total > clearing_cap(g) {
        let w = crate::words::wg(g, &"to", &"on");
        let text = format!("that's long enough — back {} the corridor{}", w, if b.is_some() { ", stay paired" } else { "" });
        clearing_say(g, &text, None);
        clearing_cancel(g);
        return false;
    }
    match cl!(g).stage {
        ClearStage::Approach => {
            if clearing_read_doors(g) { return true; }
            let (bounds, stack0) = { let p = cl!(g).cur.as_ref().unwrap(); (p.bounds, p.stack[0]) };
            for (i, &gd) in men.iter().enumerate() {
                loop {
                    let pos = g.ch[g.gd[gd].ch].pos;
                    let w = &mut cl!(g).wps[i];
                    if !(w.len() > 1 && flat(pos, w[0]) < SETTLE + 0.5) { break; }
                    w.remove(0);
                    let goal = w[0];
                    order(g, gd, OrderPatch { goal: Some(Some(goal)), ..Default::default() });
                }
            }
            let apos = g.ch[g.gd[a].ch].pos;
            let bpos = b.map(|b| g.ch[g.gd[b].ch].pos);
            let (wa_len, wb_len, wa_end, wb_end) = {
                let c = cl!(g);
                (c.wps[0].len(), c.wps[1].len(), c.wps[0].last().copied(), c.wps[1].last().copied())
            };
            let da = flat(apos, wa_end.unwrap());
            let db = match bpos { Some(p) => flat(p, wb_end.unwrap()), None => 0.0 };
            if let Some(bg) = b {
                if g.gd[bg].script.is_some() {
                    let gap = flat(apos, bpos.unwrap());
                    let ahead = wb_len < wa_len || (wb_len == wa_len && db < da - 0.6);
                    let speed = APPROACH_SPEED * (if ahead || gap < 1.3 { 0.5 } else if gap > 4.5 { 1.3 } else if gap > 2.8 { 1.1 } else { 0.94 });
                    g.gd[bg].script.as_mut().unwrap().speed = Some(speed);
                }
            }
            let out = wa_len <= 1 && !in_bounds(apos, bounds, Some(0.35)) && bpos.map_or(true, |p| !in_bounds(p, bounds, Some(0.35)));
            let c = cl!(g);
            if da < 0.9 && c.t_near.is_nan() { c.t_near = c.t; }
            let (t, t_near, eta) = (c.t, c.t_near, c.eta);
            if out && ((da < 0.9 && (db < 1.0 || t > t_near + 3.5)) || (t > eta && flat(apos, stack0) < 5.0)) {
                clearing_start_drill(g);
            } else if t > eta {
                let text = format!("can't get to the {} — leave it", plan_name(cl!(g).cur.as_ref()));
                clearing_say(g, &text, None);
                return clearing_next_room(g, Some(false));
            }
        }
        ClearStage::Drill => {
            let d = {
                let c = cl!(g);
                c.t_drill += dt;
                c.drill.unwrap()
            };
            let t_drill = cl!(g).t_drill;
            room_clear_tick(g, d, t_drill, dt);
            if !room_clear_intact(g, d) { clearing_cancel(g); return false; }
            let phase = g.tab.squad_room_clear[d].phase;
            if (phase == RoomClearPhase::Stack && t_drill > 1.3) || phase == RoomClearPhase::Breach { clearing_read_doors(g); }
            let complete = room_clear_complete(g, d);
            if complete || t_drill > DRILL_CAP {
                let cleared = complete || g.tab.squad_room_clear[d].phase == RoomClearPhase::Resolve;
                return clearing_next_room(g, Some(cleared));
            }
        }
        _ => {}
    }
    true
}

/// the muster's `free`
fn muster_free(g: &Game, gd: GuardId) -> bool {
    let man = &g.gd[gd];
    man.task.is_none() && man.script.is_none()
        && (man.state == crate::game::GuardState::Patrol || (man.state == crate::game::GuardState::Suspicious && man.body_duty.as_ref().map_or(false, |d| d.stage == crate::game::GuardBodyDutyStage::Fan)))
}
/// the muster's `busy`
fn muster_busy(g: &Game, gd: Option<GuardId>) -> bool {
    match gd { Some(gd) => g.gd[gd].body_duty.is_some() || g.gd[gd].task.is_some(), None => false }
}

/// Nobody is scripted yet: wait for both to be calm and free, then the word and off. (squad.ts Clearing.muster)
pub fn clearing_muster(g: &mut crate::game::Game) -> bool {
    let (a, b, t) = { let c = cl!(g); (c.a, c.b, c.t) };
    if t > 1.2 && muster_free(g, a) && b.map_or(true, |b| muster_free(g, b)) {
        clearing_approach(g);
        let text = format!("{}clearing the {}", if b.is_some() { "with me — " } else { "" }, plan_name(cl!(g).cur.as_ref()));
        clearing_say(g, &text, None);
        return true;
    }
    if t > (if muster_busy(g, Some(a)) || muster_busy(g, b) { 50.0 } else { 15.0 }) {
        let text = format!("clear of the {} called off — the pair never freed up", plan_name(cl!(g).cur.as_ref()));
        crate::game::game_note(g, &text);
        cl!(g).stage = ClearStage::Done;
        return false;
    }
    true
}

/// Doors within reach of either man that give the intruder away: said once a door, and that door's room jumps the queue. True: the approach was re-planned. (squad.ts Clearing.readDoors)
pub fn clearing_read_doors(g: &mut crate::game::Game) -> bool {
    let mut at = 0;
    while at < g.doors.list.len() {
        let d = g.doors.list[at];
        at += 1;
        let how = match crate::guards::door_tamper(g, d) { Some(h) => h, None => continue };
        if crate::doors::door_noticed(g, d) { continue; }
        let forced = how == crate::guards::DoorTamper::Forced;
        let mut by: Option<GuardId> = None;
        let men = if cl!(g).stage == ClearStage::Drill { vec![cl!(g).a] } else { clearing_men(g) };
        for gd in men {
            let c = g.gd[gd].ch;
            let pos = g.ch[c].pos;
            if flat(pos, g.dr[d].frame_centre) > (if forced { 3.0 } else { 1.5 }) { continue; }
            let along = g.dr[d].def.width * 0.5;
            let spot = crate::doors::doors_work_spot(g, d, pos, Some(0.45), Some(along)).pos;
            let eye = bone_or(g, c, crate::character::Bone::Head, v3::add(pos, [0.0, 1.65, 0.0]));
            if !crate::scene_collision::segment_blocked(g, eye, [spot[0], 1.0, spot[2]], None, None, None) { by = Some(gd); break; }
        }
        let by = match by { Some(by) => by, None => continue };
        crate::doors::door_set_noticed(g, d, true);
        let now = g.time;
        crate::doors::door_set_noticed_t(g, d, now);
        let mut line = String::from(if forced { "this one's been forced — someone's through here" } else { "lock's been picked — he's been through here" });
        let mut replanned = false;
        let room = match &g.level.clear_plans {
            Some(plans) => plans.iter().find(|p| p.door == g.dr[d].def.name).map(|p| plan_name(Some(p))),
            None => None,
        };
        if let Some(room) = room.filter(|r| !r.is_empty()) {
            let room_i = cl!(g).room_i;
            let idx = match cl!(g).rooms.iter().position(|v| plan_name(v.first()) == room) { Some(i) => i as f64, None => -1.0 };
            if idx < 0.0 || idx > room_i {
                let variants = if idx > room_i {
                    cl!(g).rooms.remove(idx as usize)
                } else {
                    match &g.level.clear_plans {
                        Some(plans) => plans.iter().filter(|p| plan_name(Some(p)) == room).cloned().collect(),
                        None => Vec::new(),
                    }
                };
                let c = cl!(g);
                if c.stage == ClearStage::Approach {
                    let at_room = core::cmp::min(room_i as usize, c.rooms.len());
                    c.rooms.insert(at_room, variants);
                    clearing_approach(g);
                    line.push_str(" — this one first.");
                    replanned = true;
                } else {
                    let at_room = core::cmp::min(room_i as usize + 1, c.rooms.len());
                    c.rooms.insert(at_room, variants);
                    line.push_str(" — that one next.");
                }
            }
        }
        crate::game::game_say(g, by, &line, None, None, None);
        let c = g.gd[by].ch;
        let at_ear = bone_or(g, c, crate::character::Bone::Head, g.ch[c].pos);
        seam_audio_play_rate(g, "radio", at_ear, 0.35, 1.1);
        return replanned;
    }
    false
}

/// the approach's `nearest`: which of ps is the shortest walk from `at`; the first when there is no choice
fn nearest(g: &mut Game, ps: &[crate::scene_level::RoomClearPlan], at: V3, to: fn(&crate::scene_level::RoomClearPlan) -> V3) -> usize {
    let mut best = 0;
    let mut bl = f64::INFINITY;
    if ps.len() > 1 {
        for (i, p) in ps.iter().enumerate() {
            let l = path_len(g, at, to(p));
            if l < bl { bl = l; best = i; }
        }
    }
    best
}
fn to_kick_from(p: &crate::scene_level::RoomClearPlan) -> V3 { p.kick_from }
fn to_stack(p: &crate::scene_level::RoomClearPlan) -> V3 { p.stack[0] }

/// The walk to a room's stack: the nearest variant, out through the room they stand in first, each to his own mark. (squad.ts Clearing.approach)
pub fn clearing_approach(g: &mut crate::game::Game) {
    let a = cl!(g).a;
    let from = g.ch[g.gd[a].ch].pos;
    let variants = { let c = cl!(g); c.rooms[c.room_i as usize].clone() };
    let here: Vec<crate::scene_level::RoomClearPlan> = match &g.level.clear_plans {
        Some(plans) => plans.iter().filter(|p| p.bounds.is_some() && in_bounds(from, p.bounds, None)).cloned().collect(),
        None => Vec::new(),
    };
    let p: crate::scene_level::RoomClearPlan;
    let mut via: Vec<V3> = Vec::new();
    if !here.is_empty() && plan_name(here.first()) == plan_name(variants.first()) {
        p = variants[nearest(g, &variants, from, to_kick_from)].clone();
        via.push(p.kick_from);
    } else {
        p = variants[nearest(g, &variants, from, to_stack)].clone();
        let path = if !here.is_empty() { crate::scene_collision::nav_grid_find_path(g, from, p.stack[0], None, None) } else { None };
        if let Some(path) = path {
            if path.iter().any(|q| in_bounds(*q, p.bounds, Some(-0.3))) {
                via.push(here[nearest(g, &here, from, to_kick_from)].kick_from);
            }
        }
    }
    {
        let c = cl!(g);
        c.cur = Some(p.clone());
        c.stage = ClearStage::Approach;
        c.t = 0.0;
        c.t_near = f64::NAN;
        c.drill = None;
    }
    let u = v3::normalize([p.stack[1][0] - p.stack[0][0], 0.0, p.stack[1][2] - p.stack[0][2]]);
    let fc = match crate::doors::doors_by_name(g, &p.door) { Some(d) => g.dr[d].frame_centre, None => p.kick_from };
    let off: V3 = [p.stack[0][0] - fc[0], 0.0, p.stack[0][2] - fc[2]];
    let n = v3::normalize(v3::mad(off, u, -v3::dot(off, u)));
    let men = clearing_men(g);
    for (i, &gd) in men.iter().enumerate() {
        {
            let man = &mut g.gd[gd];
            man.state = crate::game::GuardState::Search;
            man.react_t = 0.0;
            man.search_t = 0.0;
            man.hold = false;
            man.pinned = false;
            man.body_duty = None;
            man.awareness = m::min(man.awareness, 0.25);
        }
        cl!(g).aw[i] = g.gd[gd].awareness;
        let mark = p.stack[i];
        let mut end: V3 = v3::copy(mark);
        let sides: &[f64] = if i > 0 { &[0.7, 0.0] } else { &[0.0] };
        'pick: for &side in sides {
            for &k in &[1.6, 1.1, 0.6] {
                let q: V3 = [mark[0] + u[0] * k + n[0] * side, 0.0, mark[2] + u[2] * k + n[2] * side];
                if !crate::scene_collision::nav_grid_is_blocked(g, q[0], q[2]) && !crate::scene_collision::solid_blocked(g, [mark[0], 0.45, mark[2]], [q[0], 0.45, q[2]], None) {
                    end = q;
                    break 'pick;
                }
            }
        }
        let goal = {
            let w = &mut cl!(g).wps[i];
            w.clear();
            for q in &via { w.push(v3::copy(*q)); }
            w.push(end);
            w[0]
        };
        order(g, gd, OrderPatch {
            goal: Some(Some(goal)),
            speed: Some(Some(APPROACH_SPEED * (if i > 0 { 0.96 } else { 1.0 }))),
            face: Some(None),
            strafe: Some(Some(false)),
            aim_at: Some(None),
            stance: Some(Some(crate::character::Stance::LowReady)),
            upper: Some(Some(crate::character::UpperMode::None)),
            crouch: Some(Some(false)),
            ..Default::default()
        });
    }
    let walk = cl!(g).wps[0].clone();
    let mut l = 0.0;
    let mut q = from;
    for w in walk {
        l += m::min(path_len(g, q, w), flat(q, w) * 3.0);
        q = w;
    }
    cl!(g).eta = l / APPROACH_SPEED + 8.0;
}

/// Hand the pair to the drill. (squad.ts Clearing.startDrill)
pub fn clearing_start_drill(g: &mut crate::game::Game) {
    let (plan, a, b) = { let c = cl!(g); (c.cur.clone().unwrap(), c.a, c.b) };
    let d = room_clear_new(g, &plan, a, b, Some(RoomClearNewOpts { live: Some(true) }));
    cl!(g).drill = Some(d);
    room_clear_start(g, d);
    let c = cl!(g);
    c.aw = vec![0.0, 0.0];
    c.t_drill = 0.0;
    c.stage = ClearStage::Drill;
    c.t = 0.0;
}

/// On to the next room, or done: back to the paired patrol, calm. `cleared` goes in the lockdown's ledger. (squad.ts Clearing.nextRoom)
pub fn clearing_next_room(g: &mut crate::game::Game, cleared: Option<bool>) -> bool {
    let cleared = cleared.unwrap_or(true);
    let now = g.time;
    if cleared {
        if let Some(cur) = &cl!(g).cur {
            let name = plan_name(Some(cur));
            g.cleared_at.set(name, now);
        }
    }
    cl!(g).room_i += 1.0;
    let (more, count, has_b) = { let c = cl!(g); (c.rooms.get(c.room_i as usize).is_some(), c.rooms.len(), c.b.is_some()) };
    if !more {
        let w = crate::words::wg(g, &"to", &"on");
        let text = format!("{} — back {} the corridor{}", if count > 1 { "that's the near rooms done" } else { "room done" }, w, if has_b { ", stay paired" } else { "" });
        clearing_say(g, &text, None);
        clearing_cancel(g);
        return false;
    }
    clearing_approach(g);
    let c = cl!(g);
    let text = format!("{}the {}", if c.room_i == (c.rooms.len() as f64) - 1.0 { "last one — " } else { "moving to " }, plan_name(c.cur.as_ref()));
    clearing_say(g, &text, None);
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scene_level::BreachPlanBounds;

    #[test]
    fn in_bounds_is_strict_and_grows_by_pad() {
        let b = BreachPlanBounds { x0: 0.0, x1: 2.0, z0: 0.0, z1: 2.0 };
        assert!(in_bounds([1.0, 0.0, 1.0], Some(b), None));
        assert!(!in_bounds([0.0, 0.0, 1.0], Some(b), None));              // on the edge is out
        assert!(in_bounds([-0.1, 0.0, 1.0], Some(b), Some(0.35)));
        assert!(!in_bounds([0.2, 0.0, 1.0], Some(b), Some(-0.3)));        // a negative pad shrinks it (the walk through the room's test)
        assert!(!in_bounds([1.0, 0.0, 1.0], None, Some(100.0)));          // no bounds: never in
    }

    #[test]
    fn worst_news_ranks_alert_over_body_over_suspicious() {
        assert!(rank(Perceive::Alert) > rank(Perceive::Body));
        assert!(rank(Perceive::Body) > rank(Perceive::Suspicious));
    }
}
