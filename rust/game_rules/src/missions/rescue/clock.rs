use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// where a round at this man goes: his chest bone, else above his feet
fn chest_of(g: &Game, gd: GuardId) -> V3 {
    let c = g.gd[gd].ch;
    g.ch[c].bone_or(crate::character::Bone::Chest, 0.6)
}

/// the men of a list still on their feet, in the list's order
fn standing(g: &Game, l: &[GuardId]) -> Vec<GuardId> {
    l.iter().copied().filter(|&x| !crate::condition::down(g, g.gd[x].ch)).collect()
}

/// the hostage room's rectangle as inBounds takes it: the same four numbers under the other name of the type
fn plan_bounds(b: Option<crate::scene_scenarios::Rect>) -> Option<crate::scene_level::BreachPlanBounds> {
    b.map(|r| crate::scene_level::BreachPlanBounds { x0: r.x0, x1: r.x1, z0: r.z0, z1: r.z1 })
}

/// the room's rectangle, now
fn room(g: &Game) -> Option<crate::scene_level::BreachPlanBounds> {
    plan_bounds(rescue_bounds(g))
}

/// `${h.civ?.name ?? 'a hostage'}`
fn hostage_name(g: &Game, h: GuardId) -> String {
    match g.gd[h].civ {
        Some(c) => g.cv[c].name.clone(),
        None => String::from("a hostage"),
    }
}

/// the captor's clock, and the act it ends in: his gun on a hostage, the round, the next man's turn
pub(super) fn rescue_clock(g: &mut crate::game::Game, dt: f64) {
    if g.rescue.t0.is_nan() || rescue_over(g) { return; }
    if let Some(a) = g.rescue.act.clone() {
        the_act(g, a);
        return;
    }
    let Some(cap) = rescue_captor(g) else {
        rescue_set_step(g, RescueStep::Stopped);
        return;
    };
    new_man_takes_up_the_count(g, cap);
    g.rescue.cap_by = Some(cap);
    if crate::control::man::blind(g, cap) || crate::control::man::staggered(g, cap) { return; }
    g.rescue.resolve = m::min(1.0, g.rescue.resolve + dt / g.rescue.resolve_secs);
    let r = g.rescue.resolve;
    rescue_set_step(g, if r >= 0.8 { RescueStep::AboutTo } else if r >= 0.4 { RescueStep::Counting } else { RescueStep::Shouting });
    if g.time >= g.rescue.bark_at && !crate::condition::down(g, g.gd[cap].ch) { shout_it_out(g, cap); }
    if g.rescue.resolve >= 1.0 { take_aim_at_a_hostage(g, cap); }
}

/// The act under way: it breaks with him or her out of it; when its moment comes, the round.
fn the_act(g: &mut Game, a: RescueAct) {
    let (by, v) = (a.by, a.victim);
    if crate::condition::down(g, g.gd[by].ch) || crate::control::man::blind(g, by) || crate::control::man::staggered(g, by) || g.gd[by].held.is_some() || crate::condition::down(g, g.gd[v].ch) {
        the_act_breaks(g);
        return;
    }
    if g.time < a.at { return; }
    if crate::rescueflush::rescue_flush_victims(g, by, &[v]).is_empty() {
        the_act_breaks(g);
        return;
    }
    let (chest, took) = fire_on_her(g, by, v);
    rescue_end_act(g);
    if let Some(aim) = rescue_data(g).map(|d| d.room.door_aim) {
        let p = g.pos_of(by);
        let b = room(g);
        if crate::squad::in_bounds(p, b, None) { crate::belief::cover(g, by, [aim[0], 0.0, aim[2]]); }
    }
    crate::mind::set(g, by, 1.0, "rescue");
    if took { rescue_note(g, "act:blocked"); return; }
    the_hostage_is_shot(g, by, v, chest);
}

/// The act ends with nobody shot.
fn the_act_breaks(g: &mut Game) {
    rescue_end_act(g);
    rescue_note(g, "act:broken");
}

/// The round at her chest, loaded if it has to be: where it was aimed, and whether Sam or a teammate took it instead.
fn fire_on_her(g: &mut Game, by: GuardId, v: GuardId) -> (V3, bool) {
    let c = g.gd[by].ch;
    let chest = chest_of(g, v);
    let weapon = g.gd[by].weapon;
    if !crate::weapons::firearm_fire(g, weapon) {
        crate::weapons::firearm_rearm(g, weapon);
        crate::weapons::firearm_fire(g, weapon);
    }
    let pl = g.player;
    let ops = g.staged.team.clone();
    let up_before = standing(g, &ops).len();
    let flash = g.pl[pl].hit_flash;
    let down = crate::condition::down(g, g.pl[pl].ch);
    crate::combat::fire_weapon(g, c, chest, Some(&crate::combat::FireOpts { steady: Some(true), spread: Some(0.01), ..Default::default() }));
    let ops = g.staged.team.clone();
    let took = crate::condition::down(g, g.pl[pl].ch) != down || g.pl[pl].hit_flash > flash || standing(g, &ops).len() < up_before;
    (chest, took)
}

/// She dies; another hostage cries out, and the next man's count starts well along.
fn the_hostage_is_shot(g: &mut Game, by: GuardId, v: GuardId, chest: V3) {
    g.rescue.executed.add(v);
    let dir = v3::normalize(v3::sub(chest, g.ch[g.gd[by].ch].muzzle));
    crate::combat::kill_guard(g, v, dir, Some(false), Some(crate::game::DownKind::Shot), Some(crate::game::DownBy::Guard), crate::game::With::Hand);
    let o = g.staged.captives.iter().copied().find(|&h| h != v && !crate::condition::down(g, g.gd[h].ch));
    if let Some(o) = o { crate::civilians::hostage_say(g, o, crate::dialogue::HostageMoment::Shot, Some(v)); }
    g.rescue.murmur_at = g.time + 6.0;
    g.rescue.resolve = m::max(0.0, 1.0 - g.k.rescue.second_hostage_secs / g.rescue.resolve_secs);
}

/// A man who came to the room for this starts his count well along; any other new captor at least as far as the second hostage's count.
fn new_man_takes_up_the_count(g: &mut Game, cap: GuardId) {
    if g.rescue.cap_by != Some(cap) && crate::rescueflush::rescue_flush_came(g, cap) {
        g.rescue.resolve = m::max(0.0, 1.0 - g.k.rescueflush.kill_secs / g.rescue.resolve_secs);
    } else if g.rescue.cap_by.is_some() && g.rescue.cap_by != Some(cap) {
        g.rescue.resolve = m::min(g.rescue.resolve, m::max(0.0, 1.0 - g.k.rescue.second_hostage_secs / g.rescue.resolve_secs));
    }
}

/// His line for where the count has got to, and the next one due the later the count.
fn shout_it_out(g: &mut Game, cap: GuardId) {
    g.rescue.bark_at = g.time + 5.0 + g.rescue.resolve * 2.0;
    let line = match g.rescue.step {
        RescueStep::AboutTo => "last chance! I mean it!",
        RescueStep::Counting => "I'm counting! back off!",
        _ => "stay out! I'll do it!",
    };
    crate::game::game_say(g, cap, line, None, None);
}

/// The count is up: he leaves the room fight's roster and aims at the nearest hostage he can shoot.
fn take_aim_at_a_hostage(g: &mut Game, cap: GuardId) {
    let hs = g.staged.captives.clone();
    let alive = standing(g, &hs);
    if alive.is_empty() { return; }
    let mine = crate::rescueflush::rescue_flush_victims(g, cap, &alive);
    if mine.is_empty() { return; }
    let cp = g.pos_of(cap);
    let mut v = mine[0];
    for &b in &mine[1..] {
        let (pa, pb) = (g.pos_of(v), g.pos_of(b));
        if !(v3::dist_xz(pa, cp) <= v3::dist_xz(pb, cp)) { v = b; }
    }
    let at = chest_of(g, v);
    let def = crate::roomfight::def_of(g, cap);
    if let Some(d) = def { g.fight.defs.retain(|&x| x != d); }
    let held = g.gd[cap].duty.holds;
    crate::duty::hold(g, cap, false, "rescue");
    if g.gd[cap].script.is_some() {
        let st = g.gd[cap].state;
        crate::squad::end_guard_script(g, cap, Some(st));
    }
    let vp = g.pos_of(v);
    let cp = g.pos_of(cap);
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
    crate::game::game_say(g, cap, "that's it — time's up!", None, None);
    crate::civilians::hostage_say(g, v, crate::dialogue::HostageMoment::Aimed, None);
}

/// the act is over, one way or the other: the man off the scenario's script and back on the room fight's roster as it had him
fn rescue_end_act(g: &mut crate::game::Game) {
    let a = match g.rescue.act.take() {
        Some(a) => a,
        None => return,
    };
    let by = a.by;
    if !crate::condition::down(g, g.gd[by].ch) && g.gd[by].script.is_some() {
        crate::squad::end_guard_script(g, by, Some(crate::game::GuardState::Alert));
    }
    if !crate::condition::down(g, g.gd[by].ch) { crate::duty::hold(g, by, a.hold, "rescue"); }
    if let Some(d) = a.def {
        if g.guards.contains(&by) && !g.fight.defs.iter().any(|&x| g.tab.roomfight_def[x].gd == by) {
            g.fight.defs.push(d);
        }
    }
}

/// the captor's clock moves to a step: noted, and two of the steps told to the player
fn rescue_set_step(g: &mut crate::game::Game, s: crate::rescue::RescueStep) {
    if s == g.rescue.step { return; }
    g.rescue.step = s;
    rescue_note(g, &alloc::format!("step:{}", s.as_str()));
}

/// the hostages' count, the room secured and held, the run's end
pub(super) fn rescue_outcome(g: &mut crate::game::Game) {
    if rescue_over(g) { return; }
    let mut i = 0;
    while i < g.staged.captives.len() {
        let h = g.staged.captives[i];
        i += 1;
        if !crate::condition::down(g, g.gd[h].ch) || g.rescue.counted.has(h) { continue; }
        g.rescue.counted.add(h);
        let callsign = g.gd[h].callsign.clone();
        let name = hostage_name(g, h);
        if g.rescue.executed.has(h) {
            rescue_note(g, &alloc::format!("executed:{}", callsign));
        } else if g.staged.foes.iter().any(|&x| g.char_id(x) == g.gd[h].killer_id) {
            rescue_note(g, &alloc::format!("killed:{}", callsign));
        } else {
            rescue_note(g, &alloc::format!("teamfire:{}", callsign));
            rescue_end(g, crate::rescue::RescueResultState::Lost, &alloc::format!("{} was hit by your own fire", name));
            return;
        }
    }
    let hs = g.staged.captives.clone();
    let alive = standing(g, &hs).len();
    if alive == 0 {
        rescue_end(g, crate::rescue::RescueResultState::Lost, if hs.len() == 2 { "both hostages are dead" } else if hs.len() == 1 { "the hostage is dead" } else { "every hostage is dead" });
        return;
    }
    let pl = g.player;
    if crate::condition::down(g, g.pl[pl].ch) { rescue_end(g, crate::rescue::RescueResultState::Lost, "you are down"); return; }
    let b = room(g);
    let pad = if g.rescue.secured_at.is_nan() { 0.0 } else { 0.4 };
    let theirs = g.staged.foes.iter().any(|&h| !crate::condition::down(g, g.gd[h].ch) && g.gd[h].held.is_none() && crate::squad::in_bounds(g.pos_of(h), b, None));
    let ours = (!crate::condition::down(g, g.pl[pl].ch) && crate::squad::in_bounds(g.player_pos(pl), b, Some(pad)))
        || g.staged.team.iter().any(|&o| !crate::condition::down(g, g.gd[o].ch) && crate::squad::in_bounds(g.pos_of(o), b, Some(pad)));
    if let Some(how) = crate::rescueend::rescue_end_won(g) {
        let why = if how == crate::rescueend::RescueEndWon::Out {
            alloc::format!("{} of {} hostages out", alive, hs.len())
        } else {
            alloc::format!("{} of {} hostages alive · building cleared", alive, hs.len())
        };
        rescue_end(g, crate::rescue::RescueResultState::Rescued, &why);
        return;
    }
    let kept = !g.rescue.secured_at.is_nan() && (!theirs || !g.staged.captives.iter().any(|&h| !crate::condition::down(g, g.gd[h].ch) && crate::squad::in_bounds(g.pos_of(h), b, None)));
    if (!theirs && ours) || kept {
        if g.rescue.secured_at.is_nan() {
            g.rescue.secured_at = g.time;
            rescue_note(g, "secured");
            if !g.rescue.said.holds("h:secured") {
                g.rescue.said.add(String::from("h:secured"));
                let h = g.staged.captives.iter().copied().find(|&x| !crate::condition::down(g, g.gd[x].ch));
                if let Some(h) = h {
                    if crate::civilians::hostage_say(g, h, crate::dialogue::HostageMoment::Secured, None) { g.rescue.ack_at = g.time + 2.4; }
                }
            }
        }
    } else if !g.rescue.secured_at.is_nan() {
        g.rescue.secured_at = f64::NAN;
        rescue_note(g, "unsecured");
    }
}

/// the hostages' voices: one low line at a time across the room, off no game die
pub(super) fn rescue_voices(g: &mut crate::game::Game) {
    let b = room(g);
    let hs = g.staged.captives.clone();
    let alive = standing(g, &hs);
    if !g.rescue.ack_at.is_nan() && g.time >= g.rescue.ack_at {
        g.rescue.ack_at = f64::NAN;
        if !alive.is_empty() && !g.rescue.secured_at.is_nan() {
            let there: Vec<GuardId> = g.staged.team.iter().copied().filter(|&o| !crate::condition::down(g, g.gd[o].ch) && crate::squad::in_bounds(g.pos_of(o), b, Some(0.4))).collect();
            let quiet = there.iter().copied().find(|&o| !g.gd[o].bubble.as_ref().map_or(false, |u| g.time - u.t < u.dur));
            if let Some(by) = quiet.or(there.first().copied()) {
                let team = crate::dialogue::hostage_lines_table().team;
                let line = js::rem(g.rescue.staged, team.len() as f64) as usize;
                crate::game::game_say(g, by, &team[line], None, None);
            }
        }
    }
    if rescue_over(g) || g.rescue.act.is_some() || alive.is_empty() || !g.rescue.secured_at.is_nan() { return; }
    if let Some(n) = g.rescue.noise {
        if !g.rescue.said.holds("h:entry") && (crate::squad::in_bounds(n, b, None) || v3::dist_xz(n, rescue_room_point(g, RescueRoomPointWhich::Outside)) < 3.0) {
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
    if g.rescue.said.holds("h:secured") && rescue_captor(g).is_none() { return; }
    if pressed && !g.rescue.said.holds("h:pressed") {
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

/// the run's end: the result kept, the act let go, the card's line
fn rescue_end(g: &mut crate::game::Game, state: crate::rescue::RescueResultState, why: &str) {
    let lost = state == crate::rescue::RescueResultState::Lost;
    let hs = g.staged.captives.clone();
    let left = standing(g, &hs).len();
    let grade = if lost { RescueResultGrade::Failed } else if left == hs.len() { RescueResultGrade::Clean } else { RescueResultGrade::Costly };
    g.staged.over = true;
    g.staged.saved = Some(left as f64);
    g.rescue.result = Some(RescueResult {
        state: if lost { RescueResultState::Lost } else { RescueResultState::Rescued },
        grade,
        why: String::from(why),
    });
    rescue_end_act(g);
    if lost { rescue_note(g, &alloc::format!("lost:{}", why)); } else { rescue_note(g, "rescued"); }
}

/// Its entry's doors, while they are held for the second word, each with the men at it.
pub fn rescue_waits(g: &crate::game::Game) -> Vec<(DoorId, Vec<GuardId>)> {
    if !crate::rescue::rescue_up(g) || g.rescue.result.is_some() { return Vec::new(); }
    let Some(entry) = g.rescue.entry.as_ref().filter(|e| !g.tab.doortools_door_gate[e.gate].go) else { return Vec::new() };
    entry.ops.iter().filter(|e| e.ended.is_none()).map(|e| { let op = &g.tab.doortools_door_op[e.op]; (op.door, op.men.clone()) }).collect()
}

/// How it ended, for the run's card.
pub fn rescue_ended(g: &crate::game::Game) -> Option<(bool, &'static str, String)> {
    g.rescue.result.as_ref().map(|r| if r.state == RescueResultState::Rescued { (true, "RESCUED", r.why.clone()) } else { (false, "LOST", r.why.clone()) })
}

/// the HUD's objective line and marker while it is up: the room, and how far the captor's clock has got, never in seconds
pub(in crate::missions) fn rescue_objective(g: &crate::game::Game, seat: &crate::seats::SeatRef) -> Option<crate::rescue::RescueObjective> {
    if !crate::rescue::rescue_up(g) { return None; }
    let pos: Option<V3> = rescue_bounds(g).map(|r| [(r.x0 + r.x1) / 2.0, 0.0, (r.z0 + r.z1) / 2.0]);
    if let Some(r) = g.rescue.result.clone() {
        let text = if r.state == RescueResultState::Rescued {
            alloc::format!("{} — {} · press shift+enter to restart", if r.grade == RescueResultGrade::Clean { "clean" } else { "costly" }, r.why)
        } else {
            alloc::format!("failed — {} · press {} to restart", r.why, if crate::condition::down(g, g.pl[seat.player].ch) { "enter" } else { "shift+enter" })
        };
        return Some(RescueObjective { text, pos: None });
    }
    let hs = g.staged.captives.clone();
    let alive = standing(g, &hs).len();
    if !g.rescue.secured_at.is_nan() || g.rescue.ending.lead.is_some() {
        let o = crate::rescueend::rescue_end_objective(g, seat);
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

