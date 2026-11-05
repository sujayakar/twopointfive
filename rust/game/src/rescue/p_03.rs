//! src/game/rescue.ts lines 896 to 1058
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// a man's body, where it stands now
fn pos_of(g: &mut Game, gd: GuardId) -> V3 {
    let c = g.gd[gd].ch;
    g.ch[c].pos
}

/// where a round at this man goes: his chest bone, else above his feet   (rescue.ts:907, 932)
fn chest_of(g: &mut Game, gd: GuardId) -> V3 {
    let c = g.gd[gd].ch;
    match g.ch[c].bones.get(crate::character::Bone::Chest).copied() {
        Some(b) => b,
        None => v3::add(g.ch[c].pos, [0.0, 0.6, 0.0]),
    }
}

/// the men of a list still on their feet, in the list's order
fn standing(g: &mut Game, l: &[GuardId]) -> Vec<GuardId> {
    l.iter().copied().filter(|&x| g.gd[x].state != crate::game::GuardState::Dead).collect()
}

/// the hostage room's rectangle as inBounds takes it: the same four numbers under the other name of the type
fn plan_bounds(b: Option<crate::scene_scenarios::Rect>) -> Option<crate::scene_level::BreachPlanBounds> {
    b.map(|r| crate::scene_level::BreachPlanBounds { x0: r.x0, x1: r.x1, z0: r.z0, z1: r.z1 })
}

/// the room's rectangle, now
fn room(g: &mut Game) -> Option<crate::scene_level::BreachPlanBounds> {
    plan_bounds(rescue_bounds(g))
}

/// `${h.civ?.name ?? 'a hostage'}`
fn hostage_name(g: &mut Game, h: GuardId) -> String {
    match g.gd[h].civ {
        Some(c) => g.cv[c].name.clone(),
        None => String::from("a hostage"),
    }
}

/// `x.toFixed(0)`: the nearest whole number, a tie going up, a negative written as the sign and its size. Rust's own `{:.0}` sends a tie to the even one
fn to_fixed_0(x: f64) -> String {
    if x.is_nan() { return String::from("NaN"); }
    let n = m::round(m::abs(x));
    if x < 0.0 { alloc::format!("-{:.0}", n) } else { alloc::format!("{:.0}", n) }
}

/// the state endGuardScript is handed for a man taken off his script in place: his own, a dead man's being 'patrol'   (rescue.ts:938)
fn script_end_state(s: crate::game::GuardState) -> crate::squad::EndGuardScriptState {
    match s {
        crate::game::GuardState::Dead | crate::game::GuardState::Patrol => crate::squad::EndGuardScriptState::Patrol,
        crate::game::GuardState::Suspicious => crate::squad::EndGuardScriptState::Suspicious,
        crate::game::GuardState::Alert => crate::squad::EndGuardScriptState::Alert,
        crate::game::GuardState::Search => crate::squad::EndGuardScriptState::Search,
    }
}

/// the captor's clock, and the act it ends in: his gun on a hostage, the round, the next man's turn   (rescue.ts Rescue.clock)
pub fn rescue_clock(g: &mut crate::game::Game, dt: f64) {
    if g.rescue.t0.is_nan() || rescue_over(g) { return; }
    if let Some(a) = g.rescue.act.clone() {
        let by = a.by;
        if g.gd[by].state == crate::game::GuardState::Dead || g.time < g.gd[by].dazzled_until || g.time < g.gd[by].stagger_until || g.gd[by].held.is_some() || g.gd[a.victim].state == crate::game::GuardState::Dead {
            rescue_end_act(g);
            rescue_note(g, "act:broken");
            return;
        }
        if g.time < a.at { return; }
        if crate::rescueflush::rescue_flush_victims(g, by, &[a.victim]).is_empty() {
            rescue_end_act(g);
            rescue_note(g, "act:broken");
            return;
        }
        let (c, v) = (g.gd[by].ch, a.victim);
        let chest = chest_of(g, v);
        let weapon = g.gd[by].weapon;
        if !crate::weapons::firearm_fire(g, weapon) {
            crate::weapons::firearm_rearm(g, weapon);
            crate::weapons::firearm_fire(g, weapon);
        }
        let pl = g.player;
        let ops = g.rescue.ops.clone();
        let up_before = standing(g, &ops).len();
        let flash = g.pl[pl].hit_flash;
        let down = g.pl[pl].down;
        crate::combat::fire_weapon(g, c, chest, false, Some(&crate::combat::FireOpts { steady: Some(true), spread: Some(0.01), ..Default::default() }));
        let ops = g.rescue.ops.clone();
        let took = g.pl[pl].down != down || g.pl[pl].hit_flash > flash || standing(g, &ops).len() < up_before;
        rescue_end_act(g);
        if let Some(aim) = rescue_data(g).map(|d| d.room.door_aim) {
            let p = pos_of(g, by);
            let b = room(g);
            if crate::squad::in_bounds(p, b, None) { g.gd[by].last_known = Some([aim[0], 0.0, aim[2]]); }
        }
        g.gd[by].awareness = 1.0;
        if took { rescue_note(g, "act:blocked"); return; }
        g.rescue.executed.add(v);
        let dir = v3::normalize(v3::sub(chest, g.ch[c].muzzle));
        crate::combat::kill_guard(g, v, dir, Some(false), Some(crate::game::DownKind::Shot), Some(crate::game::DownBy::Guard), None, None);
        let o = g.rescue.hostages.iter().copied().find(|&h| h != v && g.gd[h].state != crate::game::GuardState::Dead);
        if let Some(o) = o { crate::civilians::hostage_say(g, o, crate::dialogue::HostageMoment::Shot, Some(v)); }
        g.rescue.murmur_at = g.time + 6.0;
        g.rescue.resolve = m::max(0.0, 1.0 - g.k.rescue.second_hostage_secs / g.rescue.resolve_secs);
        return;
    }
    let cap = match rescue_captor(g) {
        Some(c) => c,
        None => { rescue_set_step(g, RescueStep::Stopped); return; }
    };
    if g.rescue.cap_by != Some(cap) && crate::rescueflush::rescue_flush_came(g, cap) {
        g.rescue.resolve = m::max(0.0, 1.0 - g.k.rescueflush.kill_secs / g.rescue.resolve_secs);
    } else if g.rescue.cap_by.is_some() && g.rescue.cap_by != Some(cap) {
        g.rescue.resolve = m::min(g.rescue.resolve, m::max(0.0, 1.0 - g.k.rescue.second_hostage_secs / g.rescue.resolve_secs));
    }
    g.rescue.cap_by = Some(cap);
    if g.time < g.gd[cap].dazzled_until || g.time < g.gd[cap].stagger_until { return; }
    g.rescue.resolve = m::min(1.0, g.rescue.resolve + dt / g.rescue.resolve_secs);
    let r = g.rescue.resolve;
    rescue_set_step(g, if r >= 0.8 { RescueStep::AboutTo } else if r >= 0.4 { RescueStep::Counting } else { RescueStep::Shouting });
    if g.time >= g.rescue.bark_at && g.gd[cap].state != crate::game::GuardState::Dead {
        g.rescue.bark_at = g.time + 5.0 + g.rescue.resolve * 2.0;
        let line = match g.rescue.step {
            RescueStep::AboutTo => "last chance! I mean it!",
            RescueStep::Counting => "I'm counting! back off!",
            _ => "stay out! I'll do it!",
        };
        crate::game::game_say(g, cap, line, None, None, None);
    }
    if g.rescue.resolve >= 1.0 {
        let hs = g.rescue.hostages.clone();
        let alive = standing(g, &hs);
        if alive.is_empty() { return; }
        let mine = crate::rescueflush::rescue_flush_victims(g, cap, &alive);
        if mine.is_empty() { return; }
        let cp = pos_of(g, cap);
        let mut v = mine[0];
        for &b in &mine[1..] {
            let (pa, pb) = (pos_of(g, v), pos_of(g, b));
            if !(flat(pa, cp) <= flat(pb, cp)) { v = b; }
        }
        let at = chest_of(g, v);
        let def = g.fight.defs.iter().copied().find(|&d| g.tab.roomfight_def[d].gd == cap);
        if let Some(d) = def { g.fight.defs.retain(|&x| x != d); }
        let held = g.gd[cap].hold;
        g.gd[cap].hold = false;
        if g.gd[cap].script.is_some() {
            let st = script_end_state(g.gd[cap].state);
            crate::squad::end_guard_script(g, cap, Some(st));
        }
        let vp = pos_of(g, v);
        let cp = pos_of(g, cap);
        crate::squad::order(g, cap, crate::squad::OrderPatch {
            goal: Some(None),
            face: Some(Some(v3::yaw_to(cp, vp))),
            strafe: Some(Some(false)),
            aim_at: Some(Some(v3::copy(at))),
            stance: Some(Some(crate::character::Stance::None)),
            upper: Some(Some(crate::character::UpperMode::Aim)),
            crouch: Some(Some(false)),
            hot: Some(Some(true)),
            ..Default::default()
        });
        g.rescue.act = Some(RescueAct { by: cap, victim: v, at: g.time + g.k.rescue.aim_secs, def, hold: held });
        rescue_note(g, "act");
        crate::game::game_say(g, cap, "that's it — time's up!", None, None, None);
        crate::civilians::hostage_say(g, v, crate::dialogue::HostageMoment::Aimed, None);
    }
}

/// the act is over, one way or the other: the man off the scenario's script and back on the room fight's roster as it had him   (rescue.ts Rescue.endAct)
pub fn rescue_end_act(g: &mut crate::game::Game) {
    let a = match g.rescue.act.take() {
        Some(a) => a,
        None => return,
    };
    let by = a.by;
    if g.gd[by].state != crate::game::GuardState::Dead && g.gd[by].script.is_some() {
        crate::squad::end_guard_script(g, by, Some(crate::squad::EndGuardScriptState::Alert));
    }
    if g.gd[by].state != crate::game::GuardState::Dead { g.gd[by].hold = a.hold; }
    if let Some(d) = a.def {
        if g.guards.contains(&by) && !g.fight.defs.iter().any(|&x| g.tab.roomfight_def[x].gd == by) {
            g.fight.defs.push(d);
        }
    }
}

/// the captor's clock moves to a step: noted, and two of the steps told to the player   (rescue.ts Rescue.setStep)
pub fn rescue_set_step(g: &mut crate::game::Game, s: crate::rescue::RescueStep) {
    if s == g.rescue.step { return; }
    g.rescue.step = s;
    rescue_note(g, &alloc::format!("step:{}", s.as_str()));
    if s == RescueStep::Counting {
        crate::game::game_msg(g, "the captor is counting down");
    } else if s == RescueStep::AboutTo {
        crate::game::game_msg(g, "the captor is about to shoot a hostage");
    }
}

/// the hostages' count, the room secured and held, the run's end   (rescue.ts Rescue.outcome)
pub fn rescue_outcome(g: &mut crate::game::Game) {
    if rescue_over(g) { return; }
    let mut i = 0;
    while i < g.rescue.hostages.len() {
        let h = g.rescue.hostages[i];
        i += 1;
        if g.gd[h].state != crate::game::GuardState::Dead || g.rescue.counted.has(h) { continue; }
        g.rescue.counted.add(h);
        let callsign = g.gd[h].callsign.clone();
        let name = hostage_name(g, h);
        if g.rescue.executed.has(h) {
            rescue_note(g, &alloc::format!("executed:{}", callsign));
            crate::game::game_msg(g, &alloc::format!("{} has been shot", name));
        } else if g.rescue.hostiles.iter().any(|&x| g.ch[g.gd[x].ch].id == g.gd[h].killer_id) {
            rescue_note(g, &alloc::format!("killed:{}", callsign));
            crate::game::game_msg(g, &alloc::format!("{} has been killed", name));
        } else {
            rescue_note(g, &alloc::format!("teamfire:{}", callsign));
            rescue_end(g, RescueEndState::Lost, &alloc::format!("{} was hit by your own fire", name));
            return;
        }
    }
    let hs = g.rescue.hostages.clone();
    let alive = standing(g, &hs).len();
    if alive == 0 {
        rescue_end(g, RescueEndState::Lost, if hs.len() == 2 { "both hostages are dead" } else if hs.len() == 1 { "the hostage is dead" } else { "every hostage is dead" });
        return;
    }
    let pl = g.player;
    if g.pl[pl].down { rescue_end(g, RescueEndState::Lost, "you are down"); return; }
    let b = room(g);
    let pad = if g.rescue.secured_at.is_nan() { 0.0 } else { 0.4 };
    let theirs = g.rescue.hostiles.iter().any(|&h| g.gd[h].state != crate::game::GuardState::Dead && g.gd[h].held.is_none() && crate::squad::in_bounds(g.ch[g.gd[h].ch].pos, b, None));
    let ours = (!g.pl[pl].down && crate::squad::in_bounds(g.ch[g.pl[pl].ch].pos, b, Some(pad)))
        || g.rescue.ops.iter().any(|&o| g.gd[o].state != crate::game::GuardState::Dead && crate::squad::in_bounds(g.ch[g.gd[o].ch].pos, b, Some(pad)));
    if let Some(how) = crate::rescueend::rescue_end_won(g) {
        let why = if how == crate::rescueend::RescueEndWon::Out {
            alloc::format!("{} of {} hostages out", alive, hs.len())
        } else {
            alloc::format!("{} of {} hostages alive · building cleared", alive, hs.len())
        };
        rescue_end(g, RescueEndState::Rescued, &why);
        return;
    }
    let kept = !g.rescue.secured_at.is_nan() && (!theirs || !g.rescue.hostages.iter().any(|&h| g.gd[h].state != crate::game::GuardState::Dead && crate::squad::in_bounds(g.ch[g.gd[h].ch].pos, b, None)));
    if (!theirs && ours) || kept {
        if g.rescue.secured_at.is_nan() {
            g.rescue.secured_at = g.time;
            rescue_note(g, "secured");
            crate::game::game_msg(g, "hostage room secured. now bring them out or clear the building");
            if !g.rescue.said.has(String::from("h:secured")) {
                g.rescue.said.add(String::from("h:secured"));
                let h = g.rescue.hostages.iter().copied().find(|&x| g.gd[x].state != crate::game::GuardState::Dead);
                if let Some(h) = h {
                    if crate::civilians::hostage_say(g, h, crate::dialogue::HostageMoment::Secured, None) { g.rescue.ack_at = g.time + 2.4; }
                }
            }
        }
    } else if !g.rescue.secured_at.is_nan() {
        g.rescue.secured_at = f64::NAN;
        rescue_note(g, "unsecured");
        crate::game::game_msg(g, if theirs { "contact in the hostage room, so it isn't yours yet" } else { "nobody is holding the hostage room" });
    }
}

/// the hostages' voices: one low line at a time across the room, off no game die   (rescue.ts Rescue.voices)
pub fn rescue_voices(g: &mut crate::game::Game) {
    let b = room(g);
    let hs = g.rescue.hostages.clone();
    let alive = standing(g, &hs);
    if !g.rescue.ack_at.is_nan() && g.time >= g.rescue.ack_at {
        g.rescue.ack_at = f64::NAN;
        if !alive.is_empty() && !g.rescue.secured_at.is_nan() {
            let there: Vec<GuardId> = g.rescue.ops.iter().copied().filter(|&o| g.gd[o].state != crate::game::GuardState::Dead && crate::squad::in_bounds(g.ch[g.gd[o].ch].pos, b, Some(0.4))).collect();
            let quiet = there.iter().copied().find(|&o| !g.gd[o].bubble.as_ref().map_or(false, |u| g.time - u.t < u.dur));
            if let Some(by) = quiet.or(there.first().copied()) {
                let team = crate::dialogue::hostage_lines_table().team;
                let line = js::rem(g.rescue.staged, team.len() as f64) as usize;
                crate::game::game_say(g, by, &team[line], None, None, None);
            }
        }
    }
    if rescue_over(g) || g.rescue.act.is_some() || alive.is_empty() || !g.rescue.secured_at.is_nan() { return; }
    if let Some(n) = g.rescue.noise {
        if !g.rescue.said.has(String::from("h:entry")) && (crate::squad::in_bounds(n, b, None) || flat(n, rescue_room_point(g, RescueRoomPointWhich::Outside)) < 3.0) {
            g.rescue.said.add(String::from("h:entry"));
            let h = next_voice(g, &alive);
            let other = alive.iter().copied().find(|&o| o != h);
            crate::civilians::hostage_say(g, h, crate::dialogue::HostageMoment::Entry, other);
            g.rescue.murmur_at = g.time + 6.0;
            return;
        }
    }
    let running = !g.rescue.t0.is_nan();
    let pressed = g.rescue.step == RescueStep::Counting || g.rescue.step == RescueStep::AboutTo;
    if g.rescue.said.has(String::from("h:secured")) && rescue_captor(g).is_none() { return; }
    if pressed && !g.rescue.said.has(String::from("h:pressed")) {
        g.rescue.said.add(String::from("h:pressed"));
        g.rescue.murmur_at = m::min(g.rescue.murmur_at, g.time + 1.5);
    }
    if g.time < g.rescue.murmur_at { return; }
    let h = next_voice(g, &alive);
    let moment = if rescue_captor(g).is_none() { crate::dialogue::HostageMoment::Alone } else if pressed { crate::dialogue::HostageMoment::Threat } else { crate::dialogue::HostageMoment::Wait };
    crate::civilians::hostage_say(g, h, moment, None);
    g.rescue.murmur_at = g.time + (if pressed { 7.0 } else if running { 11.0 } else { 16.0 }) + js::rem(g.rescue.murmur_n, 3.0) * 2.0;
}

/// `alive[this.murmurN++ % alive.length]`
fn next_voice(g: &mut Game, alive: &[GuardId]) -> GuardId {
    let n = g.rescue.murmur_n;
    g.rescue.murmur_n = n + 1.0;
    alive[js::rem(n, alive.len() as f64) as usize]
}

/// the run's end: the result kept, the act let go, the card's line   (rescue.ts Rescue.end)
pub fn rescue_end(g: &mut crate::game::Game, state: crate::rescue::RescueEndState, why: &str) {
    let lost = state == RescueEndState::Lost;
    let ops = g.rescue.ops.clone();
    let team = (if g.pl[g.player].down { 0 } else { 1 }) + standing(g, &ops).len();
    let secs = if g.rescue.t0.is_nan() { 0.0 } else { (if g.rescue.secured_at.is_nan() || lost { g.time } else { g.rescue.secured_at }) - g.rescue.t0 };
    let timed = if g.rescue.t0.is_nan() {
        String::from("room secured before anybody knew")
    } else if g.rescue.secured_at.is_nan() && !lost {
        alloc::format!("{} s after the clock started", to_fixed_0(secs))
    } else {
        alloc::format!("room secured {} s after the clock started", to_fixed_0(secs))
    };
    let hs = g.rescue.hostages.clone();
    let left = standing(g, &hs).len();
    let grade = if lost { RescueResultGrade::Failed } else if left == hs.len() { RescueResultGrade::Clean } else { RescueResultGrade::Costly };
    g.rescue.result = Some(RescueResult {
        state: if lost { RescueResultState::Lost } else { RescueResultState::Rescued },
        grade,
        why: String::from(why),
        hostages: left as f64,
        team: team as f64,
        secs,
    });
    rescue_end_act(g);
    if lost { rescue_note(g, &alloc::format!("lost:{}", why)); } else { rescue_note(g, "rescued"); }
    let line = if lost {
        alloc::format!("FAILED — {}. press Shift+Enter to restart", why)
    } else {
        alloc::format!("{} — {} · team {}/{} up · {}. press Shift+Enter to restart", if grade == RescueResultGrade::Clean { "CLEAN" } else { "COSTLY" }, why, team, 1 + ops.len(), timed)
    };
    crate::game::game_msg(g, &line);
}

/// where it stands, in one word   (rescue.ts Rescue.state)
pub fn rescue_state(g: &mut crate::game::Game) -> crate::rescue::RescueState {
    if !g.rescue.active { return RescueState::Off; }
    if let Some(r) = &g.rescue.result {
        return match r.state {
            RescueResultState::Lost => RescueState::Lost,
            RescueResultState::Rescued => RescueState::Rescued,
        };
    }
    if !g.rescue.secured_at.is_nan() { return RescueState::Secured; }
    if g.rescue.t0.is_nan() { RescueState::Staged } else { RescueState::Running }
}

/// the HUD's objective line and marker while it is up: the room, and how far the captor's clock has got, never in seconds   (rescue.ts Rescue.objective)
pub fn rescue_objective(g: &mut crate::game::Game) -> Option<crate::rescue::RescueObjective> {
    if !g.rescue.active { return None; }
    let pos: Option<V3> = rescue_bounds(g).map(|r| [(r.x0 + r.x1) / 2.0, 0.0, (r.z0 + r.z1) / 2.0]);
    if let Some(r) = g.rescue.result.clone() {
        let text = if r.state == RescueResultState::Rescued {
            alloc::format!("{} — {} · press shift+enter to restart", if r.grade == RescueResultGrade::Clean { "clean" } else { "costly" }, r.why)
        } else {
            alloc::format!("failed — {} · press {} to restart", r.why, if g.pl[g.player].down { "enter" } else { "shift+enter" })
        };
        return Some(RescueObjective { text, pos: None });
    }
    let hs = g.rescue.hostages.clone();
    let alive = standing(g, &hs).len();
    if !g.rescue.secured_at.is_nan() || g.rescue.ending.lead.is_some() {
        let o = crate::rescueend::rescue_end_objective(g);
        let text = if !g.rescue.t0.is_nan() && g.rescue.step != RescueStep::Stopped {
            alloc::format!("{} · the captor is {}", o.text, g.rescue.step.as_str())
        } else {
            o.text
        };
        return Some(RescueObjective { text, pos: o.pos });
    }
    let clock = if g.rescue.t0.is_nan() {
        String::new()
    } else if g.rescue.step == RescueStep::Stopped {
        String::from(" · nobody on them")
    } else {
        alloc::format!(" · the captor is {}", g.rescue.step.as_str())
    };
    let behind = if rescue_dynamic(g) { crate::rescueflow::rescue_flow_line(g) } else { String::new() };
    let text = if !behind.is_empty() {
        alloc::format!("objective ▸ get to the hostages ({} alive){} · {}", alive, clock, behind)
    } else {
        alloc::format!("objective ▸ get to the hostages ({} alive) — {}{}", alive, rescue_place(g), clock)
    };
    Some(RescueObjective { text, pos })
}

/// the captor's clock as the player is shown it   (rescue.ts Rescue.clockStep)
pub fn rescue_clock_step(g: &mut crate::game::Game) -> crate::rescue::RescueStep {
    g.rescue.step
}

/// one line for the panel / the console   (rescue.ts Rescue.summary)
pub fn rescue_summary(g: &mut crate::game::Game) -> String {
    if !g.rescue.active { return String::from("hostage rescue off"); }
    let clock = if g.rescue.t0.is_nan() { String::from("clock not started") } else { alloc::format!("captor {}", g.rescue.step.as_str()) };
    let (hs, ops, foes) = (g.rescue.hostages.clone(), g.rescue.ops.clone(), g.rescue.hostiles.clone());
    let set = foes.iter().filter(|&&h| g.gd[h].state != crate::game::GuardState::Dead && g.rescue.defs.get(h).map_or(false, |d| d.set)).count();
    let state = rescue_state(g);
    let entry = match &g.rescue.entry {
        Some(e) => {
            let each: Vec<String> = e.ops.iter().map(|x| {
                let how = match &x.ended {
                    Some(s) => s.clone(),
                    None => String::from(g.tab.doortools_door_op[x.op].phase.as_str()),
                };
                alloc::format!("{}: {}", if x.side { "side" } else { "front" }, how)
            }).collect();
            alloc::format!(", F's entry ({})", each.join(", "))
        }
        None => String::new(),
    };
    let blind = if js::truthy(g.rescue.blind_rounds) { alloc::format!(" · {} blind rounds", g.rescue.blind_rounds) } else { String::new() };
    let result = match &g.rescue.result {
        Some(r) => alloc::format!(" · {}", r.why),
        None => String::new(),
    };
    let flow = crate::rescueflow::rescue_flow_summary(g);
    alloc::format!(
        "hostage rescue {} · hostages {}/{} · {} · team {}/{} up{} · defenders {}/{} up, {} set{}{} · staged ×{} · {}",
        state.as_str(),
        standing(g, &hs).len(), hs.len(),
        clock,
        standing(g, &ops).len(), ops.len(),
        entry,
        standing(g, &foes).len(), foes.len(),
        set,
        blind,
        result,
        g.rescue.staged,
        flow,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    /// the TypeScript's own answers, from node: [0, -0, 0.4, -0.4, 0.5, 1.5, 2.5, -2.5, 0.49999999999999994, 59.5, 12.499999, 1000.5, 4503599627370497, NaN].map(x => x.toFixed(0))
    #[test]
    fn to_fixed_0_is_the_languages() {
        let cases: [(f64, &str); 14] = [
            (0.0, "0"), (-0.0, "0"), (0.4, "0"), (-0.4, "-0"), (0.5, "1"), (1.5, "2"), (2.5, "3"), (-2.5, "-3"),
            (0.49999999999999994, "0"), (59.5, "60"), (12.499999, "12"), (1000.5, "1001"), (4503599627370497.0, "4503599627370497"), (f64::NAN, "NaN"),
        ];
        for (x, s) in cases { assert_eq!(to_fixed_0(x), s); }
    }
}
