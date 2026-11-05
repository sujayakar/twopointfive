//! src/game/guards.ts lines 2119 to 2553
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

// ---- the seam left: the one step the skeleton still cannot do (notes.md, ## Skeleton)

/// g.audio.play(name, pos, gain, { rate }): opts' values are `crate::Opaque`, which cannot hold the rate.
fn seam_audio_play_rate(g: &mut Game, name: &str, pos: Option<V3>, gain: f64, rate: f64) { todo!("needs: game_audio_like_play's opts able to hold a rate") }

// ---- small helpers: the TypeScript's own repeated phrases

/// `c.bones.head ?? v3.add(c.pos, [0, 1.65, 0])`
fn eye_of_char(g: &Game, c: CharId) -> V3 {
    match g.ch[c].bones.get(crate::character::Bone::Head).copied() {
        Some(h) => h,
        None => v3::add(g.ch[c].pos, [0.0, 1.65, 0.0]),
    }
}
/// `c.bones.head ?? c.pos`
fn head_or_pos(g: &Game, c: CharId) -> V3 {
    match g.ch[c].bones.get(crate::character::Bone::Head).copied() {
        Some(h) => h,
        None => g.ch[c].pos,
    }
}
/// `gd.path = []; gd.pathGoal = null;`: a whole write of both, so their identity words move with them.
fn clear_path(g: &mut Game, gd: GuardId) {
    let m = &mut g.gd[gd];
    m.path = Vec::new();
    m.path_serial = m.path_serial.wrapping_add(1);
    m.path_goal = None;
    m.path_goal_orbit = false;
}
/// `gd.lastKnown = at`: a whole write of the fix.
fn set_fix(g: &mut Game, gd: GuardId, at: V3) {
    let m = &mut g.gd[gd];
    m.last_known = Some(at);
    m.fix_stamp = m.fix_stamp.wrapping_add(1);
}
/// `gd.speed = damp(gd.speed, 0, rate, dt)`
fn brake(g: &mut Game, gd: GuardId, rate: f64, dt: f64) {
    g.gd[gd].speed = v3::damp(g.gd[gd].speed, 0.0, rate, dt);
}
/// `const own = gd.script; gd.script = lent; gd.pieing = null; gd.aim.clear(); runGuardScript(g, gd, dt); gd.script = own;`
/// Gives back what the run left in the lent script: its lender reads that (conventions 4, an object lent), and each `*_script_ran` below puts it home.
fn run_lent_script(g: &mut Game, gd: GuardId, lent: crate::game::GuardScript, dt: f64) -> Option<crate::game::GuardScript> {
    let own = g.gd[gd].script.take();
    g.gd[gd].script = Some(lent);
    g.gd[gd].pieing = None;
    crate::aim::aim_state_clear(&mut g.gd[gd].aim);
    crate::squad::run_guard_script(g, gd, dt);
    core::mem::replace(&mut g.gd[gd].script, own)
}
/// frag.ts reads `arrived` off the script it lent: it goes back on the first book that holds one for him, the one fragFlee found.
fn frag_script_ran(g: &mut Game, gd: GuardId, ran: Option<crate::game::GuardScript>) {
    let Some(ran) = ran else { return };
    let Some(books) = g.m.frag.books.as_mut() else { return };
    for l in books.iter_mut() {
        if let Some(s) = l.flee.get_mut(gd) { if s.is_some() { *s = Some(ran); return; } }
    }
}
/// runner.ts: the same, on his run.
fn runner_script_ran(g: &mut Game, gd: GuardId, ran: Option<crate::game::GuardScript>) {
    if let (Some(ran), Some(r)) = (ran, g.m.runner.state_of.get_mut(gd)) { r.script = ran; }
}
/// morale.ts: the same, on his mind.
fn morale_script_ran(g: &mut Game, gd: GuardId, ran: Option<crate::game::GuardScript>) {
    if let (Some(ran), Some(r)) = (ran, g.m.morale.state_of.get_mut(gd)) { r.script = ran; }
}
/// startle.ts: the same; startleScript reads `arrived` off it next frame.
fn startle_script_ran(g: &mut Game, gd: GuardId, ran: Option<crate::game::GuardScript>) {
    if let (Some(ran), Some(r)) = (ran, g.m.startle.state_of.get_mut(gd)) { r.script = Some(ran); }
}

/// One man's frame: senses, state, the state's behaviour, aim, feet, pose.   (guards.ts updateGuard)
pub fn update_guard(g: &mut crate::game::Game, gd: GuardId, dt: f64) {
    use crate::game::GuardState as St;
    use crate::character::{Bone, Stance, UpperMode};
    let c = g.gd[gd].ch;
    let route = g.gd[gd].route;
    crate::character::character_turn_begin(g, c);
    player_ground_vel(g);
    if g.gd[gd].state == St::Dead {
        let m = &mut g.gd[gd];
        m.muzzle_down = false; m.pieing = None; m.search_plan = None; m.held = None; m.standoff = None; m.speed = 0.0;
        g.ch[c].vel = [0.0, 0.0, 0.0];
        g.ch[c].anim.speed = 0.0;
        crate::aim::aim_state_clear(&mut g.gd[gd].aim);
        crate::character::character_update(g, c, dt);
        crate::game::game_body_thud(g, c);
        attach_flashlight(g, gd, Some(dt));
        return;
    }
    if g.gd[gd].held.is_some() {
        {
            let m = &mut g.gd[gd];
            m.pieing = None; m.search_plan = None; m.body_duty = None; m.standoff = None; m.muzzle_down = false; m.saw_player_this_frame = false;
        }
        clear_path(g, gd);
        g.gd[gd].stall_ref = None;
        crate::aim::aim_state_clear(&mut g.gd[gd].aim);
        crate::character::character_update(g, c, dt);
        attach_flashlight(g, gd, Some(dt));
        let prev_phase = g.gd[gd].step_phase;
        g.gd[gd].step_phase = g.ch[c].anim.phase;
        if g.gd[gd].speed > 0.4 && ((prev_phase < 0.5) != (g.gd[gd].step_phase < 0.5)) {
            let p = g.ch[c].pos;
            crate::floor::tread(g, p, 0.22, false);
        }
        g.gd[gd].last_pos = g.ch[c].pos;
        g.m.guards.held_last_frame.add(gd);
        return;
    }
    if g.gd[gd].gun_lost_at >= 0.0 { gun_back(g, gd); }
    if g.m.guards.held_last_frame.delete(gd) {
        g.m.guards.unstick_legs.delete(gd);
        unstick_begin(g, gd);
    }
    let staff = g.gd[gd].team == crate::teams::Team::Guard;
    if !staff { unit_light_meter(g, gd, dt); }
    if let Some(fs) = crate::frag::frag_flee(g, gd) { let ran = run_lent_script(g, gd, fs, dt); frag_script_ran(g, gd, ran); return; }
    if let Some(rs) = crate::runner::runner_script(g, gd) { let ran = run_lent_script(g, gd, rs, dt); runner_script_ran(g, gd, ran); return; }
    if let Some(ms) = crate::morale::morale_script(g, gd) { let ran = run_lent_script(g, gd, ms, dt); morale_script_ran(g, gd, ran); return; }
    if let Some(ss) = crate::startle::startle_script(g, gd) { let ran = run_lent_script(g, gd, ss, dt); startle_script_ran(g, gd, ran); return; }
    if g.gd[gd].script.is_some() {
        {
            let m = &mut g.gd[gd];
            m.pieing = None; m.search_plan = None; m.standoff = None;
            crate::aim::aim_state_clear(&mut m.aim);
            let s = m.script.as_ref().unwrap();
            let drawn = s.over.is_some() || s.upper.clone().unwrap_or(UpperMode::Aim) == UpperMode::Aim;
            m.drawn = drawn;
        }
        crate::squad::run_guard_script(g, gd, dt);
        return;
    }
    let lvl = escalation_of(g);
    g.gd[gd].drawn = lvl >= 1.0 || !staff;
    // --- perception ---
    let mut seen = false;
    let mut dist = 99.0;
    if g.ai_enabled && !crate::doze::doze_deaf(g, gd) {
        let ear = crate::doze::doze_ear(g, gd);
        hearing_check(g, gd, Some(ear[0]), Some(true), Some(ear[1]));
        if staff && crate::teams::is_hostile(g, crate::teams::Team::Guard, crate::teams::Team::Operative) {
            acquire_foe(g, gd);
            if g.gd[gd].foe.is_none() { g.gd[gd].foe = Some(crate::game::Foe::Player); }
        } else if !staff { acquire_foe(g, gd); }
        let f = g.gd[gd].foe.clone();
        let fc = crate::teams::foe_char(g, gd);
        let (visible, d) = match (&f, fc) {
            (Some(crate::game::Foe::Player), _) => { let v = can_see(g, gd); (v.visible, v.dist) }
            (Some(_), Some(fch)) => { let v = can_see_char(g, gd, fch, false); (v.visible, v.dist) }
            _ => { let v = no_sight(); (v.visible, v.dist) }
        };
        dist = d;
        if let (true, Some(fch)) = (visible, fc) {
            let rate = match &f {
                Some(crate::game::Foe::Guard(o)) => sight_gain_on(g, gd, *o, dist),
                _ => sight_gain(g, gd, dist),
            };
            if rate > 0.05 {
                let pp = g.ch[fch].pos;
                let now = g.time;
                let since = now - g.gd[gd].sight_t;
                let sp = g.gd[gd].sight_pos;
                g.gd[gd].sight_vel = if since > 1e-4 && since < 0.1 { [(pp[0] - sp[0]) / since, 0.0, (pp[2] - sp[2]) / since] } else { [0.0, 0.0, 0.0] };
                let was = g.gd[gd].awareness;
                g.gd[gd].awareness = v3::clamp(was + rate * dt, 0.0, 1.0);
                if !far_faint(g, gd, dist, was) {
                    let gain = g.gd[gd].awareness - was;
                    if !crate::attention::attention_aside(g, gd, pp, gain) { set_fix(g, gd, pp); }
                }
                g.gd[gd].sight_pos = pp;
                g.gd[gd].sight_t = now;
                seen = rate > 0.25;
                if seen { g.gd[gd].last_seen_t = now; }
                let m = &mut g.gd[gd];
                m.alert_best = 1e9; m.alert_stall_t = 0.0; m.alert_ref = None; m.chase_t = 0.0;
            }
        }
        if standoff_perceive(g, gd, Some(dt)) { seen = true; }
        g.gd[gd].saw_player_this_frame = seen && matches!(f, Some(crate::game::Foe::Player));
        g.gd[gd].saw_foe_this_frame = seen;
        let now = g.time;
        if g.gd[gd].sight_t < now && g.gd[gd].sight_t > g.gd[gd].led_t {
            let fresh = now - g.gd[gd].sight_t < 0.25;
            let on_fix = match g.gd[gd].last_known { Some(lk) => v3::dist_xz(lk, g.gd[gd].sight_pos) < 0.01, None => false };
            if let (true, true, Some(pc)) = (fresh, on_fix, fc) {
                let eye = eye_of_char(g, c);
                let chest = match g.ch[pc].bones.get(Bone::Chest).copied() { Some(b) => b, None => v3::add(g.ch[pc].pos, [0.0, 1.1, 0.0]) };
                if eye_blocked(g, gd, eye, chest) {
                    let head = match g.ch[pc].bones.get(Bone::Head).copied() { Some(b) => b, None => v3::add(g.ch[pc].pos, [0.0, 1.6, 0.0]) };
                    if eye_blocked(g, gd, eye, head) {
                        let (sp, sv) = (g.gd[gd].sight_pos, g.gd[gd].sight_vel);
                        let led = lead_fix(g, sp, sv);
                        set_fix(g, gd, led);
                    }
                }
            }
            g.gd[gd].led_t = g.gd[gd].sight_t;
        }
        // a downed colleague in view: the finder's call and the floor's short script for a found body
        if staff && g.gd[gd].state != St::Alert && g.gd[gd].body_duty.is_none() && now - g.gd[gd].saw_body_t > 20.0 {
            if let Some(o) = body_in_view(g, gd) {
                let body = body_pos(g, o);
                let eye = eye_of_char(g, c);
                g.gd[gd].saw_body_t = now;
                g.gd[gd].awareness = m::max(g.gd[gd].awareness, 0.7);
                set_fix(g, gd, body);
                g.gd[gd].react_t = 1.2;
                let victim = g.gd[o].callsign.clone();
                let breathing = crate::game::is_breathing(g, o);
                g.gd[gd].witness.saw_body = Some(crate::game::WitnessSawBody { victim: victim.clone(), t: now, breathing });
                let men = g.guards.clone();   // (handed on read-only: net_quiet_body cannot take a list that lives in g)
                crate::netquiet::net_quiet_body(g, Some(gd), &men);
                let mut before = 0;
                for i in 0..g.guards.len() { let x = g.guards[i]; if g.gd[x].state == St::Dead && g.gd[x].found { before += 1; } }
                let (mut n, mut cold, mut hurt) = (0, 0, 0);
                let mut i = 0;
                while i < g.guards.len() {
                    let x = g.guards[i];
                    i += 1;
                    if g.gd[x].state != St::Dead || g.gd[x].found { continue; }
                    let with_him = x == o || (v3::dist_xz(body_pos(g, x), body) < 3.0 && { let to = v3::add(body_pos(g, x), [0.0, 0.3, 0.0]); !crate::scene_collision::segment_blocked(g, eye, to, None, None, None) });
                    if !with_him { continue; }
                    g.gd[x].found = true;
                    crate::mission::tally(g, crate::mission::TallyK::Bodies);
                    let _ = crate::chronicle::chron_body_found(g, x, gd);
                    crate::tactics::tactics_body_found(g, gd, x);
                    n += 1;
                    if crate::game::is_breathing(g, x) { cold += 1; }
                    if g.gd[x].wound.is_some() { hurt += 1; }
                }
                if g.gd[gd].state == St::Patrol || g.gd[gd].state == St::Search {
                    g.gd[gd].state = St::Suspicious;
                    g.gd[gd].search_t = 0.0;
                    clear_path(g, gd);
                }
                let in_here = if !crate::dialogue::is_office(g) { "someone's here" } else { "someone's in here" };
                let who = crate::teams::spoken_name(g, o).unwrap_or_else(|| String::from("one of ours"));
                let line = if hurt > 0 {
                    if n > 1 {
                        alloc::format!("{} men down — {}. {} — radioing it in", n, if hurt == n { "shot, still breathing" } else { "one's shot and breathing" }, in_here)
                    } else {
                        alloc::format!("it's {} — he's shot, still breathing. {} — radioing it in", who, in_here)
                    }
                } else if n > 1 {
                    if cold == n {
                        alloc::format!("{} men down — out cold, {} breathing. {} — radioing it in", n, if n > 2 { "all" } else { "both" }, in_here)
                    } else {
                        alloc::format!("{} men down! — radioing it in", n)
                    }
                } else if cold > 0 {
                    alloc::format!("it's {} — he's down, out cold but breathing. someone dropped him — radioing it in", who)
                } else {
                    String::from("man down! — radioing it in")
                };
                crate::game::game_say(g, gd, &line, Some(true), None, None);
                crate::audio_iface::game_audio_like_play(g, "radio", Some(eye), Some(0.9), None);
                if !g.quiet_utility {
                    g.alarm.pos[0] = body[0]; g.alarm.pos[1] = 0.0; g.alarm.pos[2] = body[2]; g.alarm.placed = true;
                    let to = if lvl >= 1.0 || g.alarm.episode { 2.0 } else { 1.0 };
                    raise_escalation(g, to, Some(gd), Some(crate::guards::RaiseEscalationCause::Body), Some(before > 0));
                    g.gd[gd].task = None;
                    g.gd[gd].body_duty = Some(crate::game::GuardBodyDuty { body: o, role: crate::game::GuardBodyDutyRole::Finder, stage: crate::game::GuardBodyDutyStage::To, t: 0.0, t0: g.time, spot: None });
                }
                let mut i = 0;
                while i < g.guards.len() {
                    let other = g.guards[i];
                    i += 1;
                    if other == gd || g.gd[other].state == St::Dead { continue; }
                    g.gd[other].awareness = m::max(g.gd[other].awareness, 0.45);
                    let breathing = crate::game::is_breathing(g, o);
                    let t = g.time;
                    g.gd[other].witness.called_to_body = Some(crate::game::WitnessCalledToBody { victim: victim.clone(), t, breathing });
                    if g.gd[other].task.is_some() { continue; }
                    set_fix(g, other, body);
                    let x = &g.gd[other];
                    if !g.quiet_utility && !x.hold && x.held.is_none() && x.body_duty.is_none() && (x.state == St::Patrol || x.state == St::Suspicious || x.script.is_some()) {
                        g.gd[other].body_duty = Some(crate::game::GuardBodyDuty { body: o, role: crate::game::GuardBodyDutyRole::Called, stage: crate::game::GuardBodyDutyStage::To, t: 0.0, t0: t, spot: None });
                    }
                }
            }
        }
        // lockdown pair: the man he trails has started at something: he takes it as seriously, a step to that man's side of the spot
        if let (true, Some(l)) = (lvl >= 2.0, g.gd[gd].leader) {
            if g.gd[gd].state == St::Patrol && !g.gd[gd].hold && g.gd[l].state == St::Suspicious && g.gd[l].react_t > 0.0 && g.gd[l].last_known.is_some() && g.gd[l].task.is_none() {
                let lk = g.gd[l].last_known.unwrap();
                let lc = g.gd[l].ch;
                let f = crate::character::character_forward(g, lc);
                let side: V3 = [lk[0] + f[2] * 1.1, 0.0, lk[2] - f[0] * 1.1];
                g.gd[gd].state = St::Suspicious;
                g.gd[gd].awareness = m::max(m::max(g.gd[gd].awareness, m::min(g.gd[l].awareness, 0.6)), 0.34);
                g.gd[gd].react_t = g.gd[l].react_t + 0.3;
                g.gd[gd].search_t = 0.0;
                let fix = if crate::scene_collision::nav_grid_walkable(g, lk, side) { side } else { lk };
                set_fix(g, gd, fix);
                clear_path(g, gd);
            }
        }
        // a kicked-in lock seen up close on a calm pass: one look per door
        if staff && g.gd[gd].state == St::Patrol && !g.gd[gd].hold && !g.quiet_utility {
            let mut i = 0;
            while i < g.doors.list.len() {
                let d = g.doors.list[i];
                i += 1;
                if !crate::doors::door_lock_broken(g, d) || crate::doors::door_noticed(g, d) { continue; }
                let f = g.dr[d].frame_centre;
                let pos = g.ch[c].pos;
                let (dx, dz) = (f[0] - pos[0], f[2] - pos[2]);
                let dd = m::hyp2(dx, dz);
                if dd > 2.2 || dd < 1e-3 { continue; }
                let look = g.ch[c].chest_dir;
                if (look[0] * dx + look[2] * dz) / dd < m::cos(75.0 * v3::DEG) { continue; }
                let off = crate::doors::door_off(g, d);
                let spot: V3 = if off {
                    [f[0] - dx / dd * 0.45, 0.0, f[2] - dz / dd * 0.45]
                } else {
                    let along = g.dr[d].def.width * 0.5;
                    crate::doors::doors_work_spot(g, d, pos, Some(0.45), Some(along)).pos
                };
                let eye = eye_of_char(g, c);
                if crate::scene_collision::segment_blocked(g, eye, [spot[0], 1.0, spot[2]], None, None, None) { continue; }
                let t = g.time;
                crate::doors::door_set_noticed(g, d, true);
                crate::doors::door_set_noticed_t(g, d, t);
                g.gd[gd].state = St::Suspicious;
                g.gd[gd].awareness = m::max(g.gd[gd].awareness, 0.45);
                g.gd[gd].react_t = 1.1;
                g.gd[gd].search_t = 0.0;
                set_fix(g, gd, spot);
                clear_path(g, gd);
                crate::game::game_say(g, gd, if off { "this door's off its hinges…" } else { "this lock's been kicked in…" }, None, None, None);
                seam_audio_play_rate(g, "radio", Some(eye), 0.4, 1.1);
                break;
            }
        }
        let drain = dt * (match g.gd[gd].state { St::Alert => 0.03, St::Patrol => 0.1, _ => 0.06 });
        if !seen { g.gd[gd].awareness = m::max(0.0, g.gd[gd].awareness - drain); }
    } else {
        g.gd[gd].awareness = m::max(0.0, g.gd[gd].awareness - dt * 0.5);
    }
    // --- state transitions ---
    let prev = g.gd[gd].state;
    if g.gd[gd].awareness >= 0.85 && g.gd[gd].state != St::Alert {
        reset_chase(g, gd);
        g.gd[gd].state = St::Alert;
        g.gd[gd].react_t = m::max(g.gd[gd].react_t, 0.35);
        g.gd[gd].fire_cd = 0.6 + crate::math_dice::sim_random(g) * 0.4;
        let now = g.time;
        let radio_recent = match &g.gd[gd].bubble { Some(b) => b.radio && now - b.t < 1.0, None => false };
        if g.gd[gd].standoff.is_none() && !radio_recent {
            let logged = g.messages.iter().any(|m| now >= m.t && now - m.t < 1.0 && m.text.ends_with(": CONTACT!"));
            crate::game::game_say(g, gd, "CONTACT!", Some(false), Some(false), Some(!logged));
        }
        let at = head_or_pos(g, c);
        crate::audio_iface::game_audio_like_play(g, "radio", Some(at), Some(0.8), None);
        if g.time - g.last_sting_t > 8.0 {
            crate::audio_iface::game_audio_like_play(g, "alertSting", None, Some(0.75), None);
            g.last_sting_t = g.time;
        }
        if g.ai_enabled {
            let by = match alert_cause(g, gd, seen) {
                crate::guards::AlertCause::Body => crate::game::WitnessAlertedBy::Body,
                crate::guards::AlertCause::Radio => crate::game::WitnessAlertedBy::Radio,
                crate::guards::AlertCause::Sound => crate::game::WitnessAlertedBy::Sound,
                crate::guards::AlertCause::Sight => crate::game::WitnessAlertedBy::Sight,
            };
            g.gd[gd].witness.alerted_by = Some(by);
            g.gd[gd].witness.alert_t = g.time;
        }
        let _ = crate::chronicle::chron_contact(g, gd);
    }
    if g.ai_enabled && g.gd[gd].awareness > g.gd[gd].witness.peak_awareness { g.gd[gd].witness.peak_awareness = g.gd[gd].awareness; }
    if g.gd[gd].awareness >= 0.3 && g.gd[gd].state == St::Patrol {
        g.gd[gd].state = St::Suspicious;
        g.gd[gd].search_t = 0.0;
        if g.gd[gd].body_duty.is_some() {
            g.gd[gd].react_t = 0.5 + crate::math_dice::sim_random(g) * 0.5;
        } else {
            g.gd[gd].react_t = 0.9;
            if g.gd[gd].team != crate::teams::Team::Operative && !crate::attention::attention_again(g, gd) {
                crate::game::game_say(g, gd, "huh?", None, None, None);
            }
            let at = head_or_pos(g, c);
            seam_audio_play_rate(g, "radio", Some(at), 0.45, 1.15);
        }
    }
    crate::startle::startle_sight(g, gd, prev, seen);
    if prev != g.gd[gd].state {
        clear_path(g, gd);
        g.gd[gd].stall_ref = None;
        if g.gd[gd].state != St::Suspicious { g.gd[gd].body_duty = None; }
    }
    crate::attention::attention_tick(g, gd);
    if g.gd[gd].state != St::Search { g.gd[gd].search_plan = None; }
    let pair = standoff_upkeep(g, gd);
    let now = g.time;
    crate::game::guard_track_fix(g, gd, now);
    let dazzled = g.time < g.gd[gd].dazzled_until;
    if dazzled { g.gd[gd].react_t = m::max(g.gd[gd].react_t, g.gd[gd].dazzled_until - g.time); }
    let staggered = g.time < g.gd[gd].stagger_until;
    if staggered { g.gd[gd].react_t = m::max(g.gd[gd].react_t, g.gd[gd].stagger_until - g.time); }
    { let st = crate::startle::startle_left(g, gd); if st > 0.0 { g.gd[gd].react_t = m::max(g.gd[gd].react_t, st); } }
    crate::aim::track_guard(g, gd, dt);
    let (p0x, p0z) = (g.ch[c].pos[0], g.ch[c].pos[2]);

    // --- behaviour ---
    g.ch[c].anim.look_yaw_extra = 0.0;
    g.gd[gd].look_phase += dt;
    let mut move_speed = 0.0;
    let mut upper = UpperMode::None;
    let mut aim_at: Option<V3> = None;
    let mut crouch = 0.0;
    let mut duck = false;
    let mut pied = false;
    let state_now = g.gd[gd].state;
    'behaviour: {
        match state_now {
            St::Patrol => {
                let target = g.tab.route[route].points.get(g.gd[gd].wp).copied();
                if g.gd[gd].drawn { upper = UpperMode::Aim; }
                if g.gd[gd].body_duty.is_some() && g.gd[gd].awareness < 0.3 { g.gd[gd].body_duty = None; }
                if matches!(g.gd[gd].task, Some(crate::game::GuardTask::CheckOn(_))) { g.gd[gd].task = None; }
                let walk = g.tune.guard_walk * (if lvl >= 1.0 { 1.15 } else { 1.0 });
                if g.gd[gd].hold {
                    g.gd[gd].sweep = v3::damp(g.gd[gd].sweep, m::sin(g.gd[gd].look_phase * 0.5) * 0.2, 2.0, dt);
                    g.ch[c].vel = [0.0, 0.0, 0.0];
                    brake(g, gd, 8.0, dt);
                } else if lvl >= 2.0 && g.gd[gd].leader.is_some() {
                    partner_watch(g, gd);
                    if follow_leader(g, gd, dt, walk) { move_speed = g.gd[gd].speed; }
                } else if lvl >= 2.0 && g.gd[gd].post.is_some() {
                    if hold_post(g, gd, dt, walk) { move_speed = g.gd[gd].speed; }
                } else if g.tune.needs && crate::needs::needs_patrol(g, gd, dt, walk) {
                    move_speed = g.gd[gd].speed;
                } else if crate::doze::doze_patrol(g, gd, dt, walk) {
                    move_speed = g.gd[gd].speed;
                } else if g.gd[gd].wait > 0.0 {
                    g.gd[gd].wait -= dt;
                    g.gd[gd].sweep = m::sin(g.gd[gd].look_phase * 0.9) * 0.7;
                    g.ch[c].anim.look_yaw_extra = m::sin(g.gd[gd].look_phase * 1.3) * 0.5;
                    if g.gd[gd].wait <= 0.0 { g.gd[gd].wp = (g.gd[gd].wp + 1) % g.tab.route[route].points.len(); }
                } else {
                    go_to(g, gd, target.expect("a walked route has this point"), None);
                    let mut pace = walk;
                    if lvl >= 2.0 {
                        for i in 0..g.guards.len() {
                            let o = g.guards[i];
                            if g.gd[o].leader == Some(gd) && g.gd[o].state == St::Patrol && v3::dist(g.ch[g.gd[o].ch].pos, g.ch[c].pos) > 4.0 { pace = walk * 0.55; }
                        }
                    }
                    if follow_path(g, gd, dt, pace) {
                        let w = g.tab.route[route].wait.get(g.gd[gd].wp).copied().unwrap_or(0.0);
                        g.gd[gd].wait = w * (if lvl >= 1.0 { 0.6 } else { 1.0 });
                        if g.gd[gd].wait <= 0.0 { g.gd[gd].wp = (g.gd[gd].wp + 1) % g.tab.route[route].points.len(); }
                    }
                    let k = if g.gd[gd].drawn { 0.4 } else { 0.25 };
                    g.gd[gd].sweep = v3::damp(g.gd[gd].sweep, m::sin(g.gd[gd].look_phase * 0.7) * k, 2.0, dt);
                    move_speed = g.gd[gd].speed;
                }
                g.ch[c].aim_yaw = g.ch[c].body_yaw + g.gd[gd].sweep;
                let low = if g.gd[gd].drawn { -0.26 } else { -0.35 };
                g.ch[c].aim_pitch = v3::damp(g.ch[c].aim_pitch, low, 3.0, dt);
                if (lvl >= 2.0 || crate::doors::doors_stay(g)) && !g.gd[gd].hold && staff { lockdown_doors(g, gd, Some(lvl < 2.0)); }
                crate::whistle::whistle_patrol_tick(g, gd);
            }
            St::Suspicious => {
                use crate::game::{GuardTask, GuardTaskArm0Kind};
                upper = if hand_carry(g, gd) == crate::guards::HandCarry::Torch { UpperMode::Torch } else { UpperMode::Aim };
                if matches!(g.gd[gd].task, Some(GuardTask::CheckOn(_))) { check_on_tick(g, gd); }
                let task_at = match &g.gd[gd].task {
                    Some(GuardTask::Arm0(t)) => Some(t.pos),
                    Some(GuardTask::CheckOn(t)) => Some(t.pos),
                    None => None,
                };
                if let Some(at) = task_at {
                    let off = match g.gd[gd].last_known { Some(lk) => v3::dist(lk, at) > 0.5, None => true };
                    if off && g.gd[gd].awareness < 0.45 { set_fix(g, gd, at); }
                    g.gd[gd].awareness = m::max(g.gd[gd].awareness, 0.2);
                    if !matches!(g.gd[gd].task, Some(GuardTask::CheckOn(_))) && g.gd[gd].search_t > 4.0 { g.gd[gd].task = None; }
                }
                let duty = if g.gd[gd].body_duty.is_some() { body_aftermath(g, gd, dt) } else { None };
                let breaker_task = |g: &Game| match &g.gd[gd].task { Some(GuardTask::Arm0(t)) => t.kind == GuardTaskArm0Kind::Breaker, _ => false };
                if let Some(d) = duty {
                    move_speed = d.move_;
                    aim_at = d.aim;
                } else if g.gd[gd].react_t > 0.0 {
                    g.gd[gd].react_t -= dt;
                    brake(g, gd, 8.0, dt);
                    if let Some(lk) = g.gd[gd].last_known { aim_at = Some(v3::add(lk, [0.0, 1.0, 0.0])); }
                } else if g.gd[gd].last_known.is_some() && g.gd[gd].hold {
                    brake(g, gd, 8.0, dt);
                    g.ch[c].vel = [0.0, 0.0, 0.0];
                    aim_at = Some(v3::add(g.gd[gd].last_known.unwrap(), [0.0, 1.0, 0.0]));
                    g.gd[gd].search_t += dt;
                    if g.gd[gd].search_t > 6.0 { g.gd[gd].awareness = m::min(g.gd[gd].awareness, 0.1); }
                } else {
                    let lever = if breaker_task(g) { crate::handwork::lever_stand(g, gd, dt) } else { None };
                    if let Some(l) = lever {
                        upper = UpperMode::None;
                        g.gd[gd].speed = l.speed;
                        move_speed = l.speed;
                        g.ch[c].vel = [0.0, 0.0, 0.0];
                        aim_at = None;
                        g.ch[c].aim_yaw = l.yaw;
                        g.ch[c].anim.look_yaw_extra = 0.0;
                        g.gd[gd].stall_ref = None;
                    } else if let Some(lk) = g.gd[gd].last_known {
                        go_to(g, gd, lk, None);
                        let arrived = follow_path(g, gd, dt, g.tune.guard_investigate);
                        move_speed = g.gd[gd].speed;
                        // (gd.lastKnown read again: the walk's gates may have written a new fix)
                        aim_at = g.gd[gd].last_known.map(|k| v3::add(k, [0.0, 0.9, 0.0]));
                        if arrived && crate::attention::attention_dwell(g, gd) {
                            g.gd[gd].stall_ref = None;
                            g.gd[gd].sweep = m::sin(g.gd[gd].look_phase * 1.1) * g.k.attention.dwell_sweep;
                            aim_at = None;
                            g.ch[c].aim_yaw = g.ch[c].body_yaw + g.gd[gd].sweep;
                        } else if arrived {
                            g.gd[gd].stall_ref = None;
                            g.gd[gd].search_t += dt;
                            g.gd[gd].sweep = m::sin(g.gd[gd].look_phase * 1.1) * 1.1;
                            g.ch[c].anim.look_yaw_extra = m::sin(g.gd[gd].look_phase * 1.9) * 0.4;
                            aim_at = None;
                            g.ch[c].aim_yaw = g.ch[c].body_yaw + g.gd[gd].sweep;
                            if g.gd[gd].search_t > 5.0 { g.gd[gd].awareness = m::min(g.gd[gd].awareness, 0.2); }
                        } else {
                            g.gd[gd].search_t = 0.0;
                            if crate::whistle::whistle_soft(g, gd) { g.gd[gd].awareness = m::max(g.gd[gd].awareness, g.k.whistle.walk_floor); }
                            crate::attention::attention_walk(g, gd);
                            let free = match &g.gd[gd].task { None => true, Some(GuardTask::CheckOn(_)) => true, _ => false };
                            if free && not_getting_anywhere(g, gd, dt, 4.0) {
                                if g.gd[gd].task.is_some() {
                                    advance_check_on(g, gd);
                                } else {
                                    let here = g.ch[c].pos;
                                    set_fix(g, gd, here);
                                    g.gd[gd].stall_ref = None;
                                    clear_path(g, gd);
                                }
                            }
                        }
                    }
                }
                if g.gd[gd].awareness < 0.12 {
                    let d = g.gd[gd].body_duty.as_ref().map(|d| d.body);
                    g.gd[gd].state = St::Patrol;
                    clear_path(g, gd);
                    g.gd[gd].body_duty = None;
                    let player_down = g.pl[g.player].down;
                    if matches!(g.gd[gd].foe, Some(crate::game::Foe::Player)) && player_down {
                        if !g.stood_down {
                            g.stood_down = true;
                            let line = if g.gd[gd].unit.is_some() { "he's down — hold where you are" } else { "back to your posts" };
                            crate::game::game_say(g, gd, line, Some(true), None, None);
                        }
                    } else if let Some(b) = d {
                        if !g.gd[b].body_closed && g.gd[b].body_visits > 0.0 && v3::dist_xz(g.ch[c].pos, body_pos(g, b)) < 12.0 {
                            g.gd[b].body_closed = true;
                            let line = if crate::game::is_breathing(g, b) {
                                alloc::format!("no sign of him. {} is breathing, he'll be okay — whoever dropped him is still close", g.gd[b].callsign)
                            } else {
                                String::from("no sign of him — eyes open, he's close")
                            };
                            crate::game::game_say(g, gd, &line, None, None, None);
                        }
                    } else if !staff {
                        // a unit calms without the staff's platitudes
                    } else {
                        let p = g.ch[c].pos;
                        if near_known_body(g, p, Some(4.0)).is_none() {
                            let now = g.time;
                            let radio_recent = match &g.gd[gd].bubble { Some(b) => b.radio && now - b.t < 4.0, None => false };
                            if !radio_recent && !crate::attention::attention_told(g, gd) {
                                let line = if lvl >= 1.0 { "clear here — staying on it" } else { "must have been nothing" };
                                crate::game::game_say(g, gd, line, None, None, None);
                            }
                        }
                    }
                }
                g.ch[c].aim_pitch = v3::damp(g.ch[c].aim_pitch, -0.12, 3.0, dt);
            }
            St::Alert => {
                use crate::suppression::HeadDown;
                upper = UpperMode::Aim;
                note_door_crossings(g, gd);
                let pc = crate::teams::foe_char(g, gd);
                let down = crate::teams::foe_down(g, gd);
                let can_shoot = pc.is_some() && !down && !dazzled && !staggered && (g.time - g.gd[gd].last_seen_t < 0.5);
                if g.gd[gd].react_t > 0.0 || !can_shoot { g.gd[gd].shot_blocked_t = 0.0; g.gd[gd].line_in_t = 0.0; }
                if g.gd[gd].react_t > 0.0 || down || g.gd[gd].pinned { g.gd[gd].stall_ref = None; g.gd[gd].chase_t = 0.0; }
                let sup_now = if g.tune.suppression && !crate::gunhold::gun_hold_runs(g, gd) { crate::suppression::head_down(g, gd, dt) } else { HeadDown::None };
                let in_standoff = g.gd[gd].standoff.is_some() && pair.is_some();
                if !matches!(sup_now, HeadDown::None) {
                    duck = true;
                    g.gd[gd].shot_blocked_t = 0.0; g.gd[gd].line_in_t = 0.0; g.gd[gd].chase_t = 0.0;
                    g.gd[gd].fire_cd = m::max(g.gd[gd].fire_cd, g.k.suppression.head_down_delay);
                    if g.gd[gd].react_t > 0.0 { g.gd[gd].react_t -= dt; }
                    if g.gd[gd].reload_t >= 0.0 { tick_reload_beat(g, gd, dt); }
                    let src = g.gd[gd].sup.src.filter(|_| g.time - g.gd[gd].sup.src_t < g.k.suppression.src_memory_secs);
                    let threat = match (src, g.gd[gd].last_known, pc) {
                        (Some(s), _, _) => s,
                        (None, Some(lk), _) => lk,
                        (None, None, Some(p)) => g.ch[p].pos,
                        (None, None, None) => aim_ahead(g, c),
                    };
                    aim_at = Some([threat[0], 1.1, threat[2]]);
                    if matches!(sup_now, HeadDown::Down) {
                        crouch = 1.0;
                        brake(g, gd, 8.0, dt);
                        g.ch[c].vel = [0.0, 0.0, 0.0];
                        g.gd[gd].stall_ref = None;
                    } else {
                        crouch = g.k.suppression.displace_crouch;
                        let cover = g.gd[gd].sup.cover.expect("a man shifting has his cover spot");
                        go_to(g, gd, cover, None);
                        let pace = g.tune.guard_investigate * g.k.suppression.displace_pace;
                        let arrived = follow_path(g, gd, dt, pace);
                        move_speed = g.gd[gd].speed;
                        if arrived { crate::suppression::end_displace(g, gd, "there"); }
                        else if not_getting_anywhere(g, gd, dt, 1.5) { crate::suppression::end_displace(g, gd, "going nowhere"); }
                        else if g.time - g.gd[gd].sup.displaced_at > g.k.suppression.displace_cap_secs { crate::suppression::end_displace(g, gd, "out of time"); }
                    }
                } else if g.gd[gd].react_t > 0.0 {
                    g.gd[gd].react_t -= dt;
                    let lk = g.gd[gd].last_known;
                    aim_at = if (dazzled || g.time - g.gd[gd].last_seen_t > 0.5) && lk.is_some() {
                        Some(v3::add(lk.unwrap(), [0.0, 1.1, 0.0]))
                    } else if let Some(p) = pc {
                        Some(v3::add(g.ch[p].pos, [0.0, 1.2, 0.0]))
                    } else { None };
                    brake(g, gd, 8.0, dt);
                } else if in_standoff {
                    let r = standoff_alert(g, gd, dt, pair.unwrap());
                    move_speed = r.move_;
                    aim_at = Some(r.aim);
                    crouch = r.crouch;
                    g.gd[gd].shot_blocked_t = 0.0;
                    g.gd[gd].line_in_t = 0.0;
                } else if can_shoot {
                    let pc = pc.unwrap();
                    let crouched = crate::teams::foe_crouched(g, gd);
                    let trunk = match g.ch[pc].bones.get(Bone::Spine2).copied() { Some(b) => Some(b), None => g.ch[pc].bones.get(Bone::Hips).copied() };
                    let pp = g.ch[pc].pos;
                    let mut aim: V3 = [match trunk { Some(t) => t[0], None => pp[0] }, pp[1] + (if crouched { 0.8 } else { 1.25 }), match trunk { Some(t) => t[2], None => pp[2] }];
                    let pinned = g.gd[gd].pinned;
                    let mut blocked = false;
                    if (!pinned || pane_first(g, c, aim)) && !clear_shot(g, c, aim) {
                        let head = match g.ch[pc].bones.get(Bone::Head).copied() { Some(b) => b, None => v3::add(pp, [0.0, if crouched { 1.05 } else { 1.6 }, 0.0]) };
                        if clear_shot(g, c, head) { aim = head; } else { blocked = true; }
                    }
                    aim_at = Some(aim);
                    if !blocked && !pinned && crate::aim::bore_fouled(g, gd) { blocked = true; }
                    if blocked {
                        let who = match &g.gd[gd].foe {
                            Some(crate::game::Foe::Player) => Some(crate::coverfire::CoverSeenWho::Player),
                            Some(crate::game::Foe::Guard(o)) => Some(crate::coverfire::CoverSeenWho::Guard(*o)),
                            None => None,
                        };
                        if crate::coverfire::cover_seen(g, gd, who, aim) { blocked = false; }
                    }
                    let here = g.ch[c].pos;
                    let gun_yaw = crate::character::character_gun_yaw(g, c);
                    let facing_err = m::abs(v3::wrap_angle(m::atan2(aim[0] - here[0], aim[2] - here[2]) - gun_yaw));
                    let walled = !pinned && crate::gunwall::gun_wall_check(g, gd, Some(crate::gunwall::GunUp::Aim), Some(aim), dt, Some(true));
                    let poked = !pinned && (crate::combat::arm_through(g, c) || walled);
                    if poked && facing_err < 0.3 { blocked = true; }
                    g.gd[gd].shot_blocked_t = if blocked { g.gd[gd].shot_blocked_t + dt } else { 0.0 };
                    let in_line = muzzle_check(g, gd, Some(aim), dt, None);
                    let crossing_man = match in_line { Some(o) => { let v = g.ch[g.gd[o].ch].vel; m::hyp2(v[0], v[2]) > 0.4 } None => false };
                    g.gd[gd].line_in_t = if in_line.is_some() { g.gd[gd].line_in_t + dt } else { 0.0 };
                    let (mut closing, mut stepping) = (false, false);
                    if !pinned && (dist > 9.0 || blocked) && !crate::suppression::holding_cover(g, gd) {
                        let braking = crate::aim::aim_brake(g, gd, dist, blocked);
                        if braking && g.gd[gd].speed < 0.3 {
                            brake(g, gd, 6.0, dt);
                            g.ch[c].vel = [0.0, 0.0, 0.0];
                            closing = true;
                        } else {
                            go_to(g, gd, g.ch[pc].pos, None);
                            let pace = if braking { 0.0 } else if crate::suppression::flinching(g, gd) { g.tune.guard_investigate } else { g.tune.guard_chase };
                            closing = !follow_path(g, gd, dt, pace);
                            if braking { brake(g, gd, 6.0, dt); }
                        }
                        move_speed = g.gd[gd].speed;
                    } else if !pinned && g.gd[gd].muzzle_down && !(crossing_man && g.gd[gd].line_in_t < 0.45) {
                        let st = line_step(g, gd, aim, in_line, dt);
                        if matches!(st, crate::guards::LineStep::Step) {
                            stepping = true;
                            move_speed = g.gd[gd].speed;
                        } else if matches!(st, crate::guards::LineStep::None) && dist > LINE_PUSH_MIN_M {
                            go_to(g, gd, g.ch[pc].pos, None);
                            follow_path(g, gd, dt, g.tune.guard_investigate);
                            move_speed = g.gd[gd].speed;
                        } else {
                            brake(g, gd, 6.0, dt);
                            g.ch[c].vel = [0.0, 0.0, 0.0];
                        }
                    } else if crate::doorway::off_threshold(g, gd, Some(aim), dt, true) {
                        move_speed = g.gd[gd].speed;
                    } else {
                        brake(g, gd, 6.0, dt);
                        g.ch[c].vel = [0.0, 0.0, 0.0];
                    }
                    if !(blocked && !closing) {
                        if !stepping { g.gd[gd].stall_ref = None; }
                        g.gd[gd].chase_t = 0.0;
                    } else if g.gd[gd].shot_blocked_t > 4.0 && !pinned {
                        drop_to_search(g, gd, Some("no shot — lost him"));
                        g.gd[gd].awareness = m::min(g.gd[gd].awareness, 0.5);
                        break 'behaviour;
                    }
                    g.gd[gd].fire_cd -= dt;
                    let twisted = move_speed > 0.3 && m::abs(v3::wrap_angle(g.ch[c].body_yaw - g.ch[c].aim_yaw)) > 0.9;
                    let shield = g.tune.muzzle_discipline && shield_up(g);
                    if shield { g.gd[gd].fire_cd = m::max(g.gd[gd].fire_cd, SHIELD_RELEASE_BEAT_MIN + dt); }
                    let weapon = g.gd[gd].weapon;
                    if crate::gunslots::side_locked(g, gd) {
                        // his hands are between his two guns, or off his gun for a charge: no round, no magazine
                    } else if g.gd[gd].reload_t >= 0.0 {
                        tick_reload_beat(g, gd, dt);
                    } else if g.m.weapons.weapons.get(g.fa[weapon].def).expect("a gun's def is a row").burst.is_some() {
                        let muzzle_down = g.gd[gd].muzzle_down;
                        let start_ok = !shield && facing_err < 0.25 && !blocked && !twisted && !poked && !muzzle_down && crate::aim::aim_ready(g, gd);
                        let round_ok = !shield && !twisted && !poked && !muzzle_down;
                        burst_trigger(g, gd, dt, aim, start_ok, round_ok, None);
                    } else if !shield && g.gd[gd].fire_cd <= 0.0 && facing_err < 0.25 && !blocked && !twisted && !poked && !g.gd[gd].muzzle_down
                        && !round_past_colleague(g, gd, aim, None, None) && crate::aim::aim_ready(g, gd) && crate::weapons::firearm_fire(g, weapon) {
                        let (cd, jitter) = { let w = g.m.weapons.weapons.get(g.fa[weapon].def).expect("a gun's def is a row"); (w.cd, w.cd_jitter) };
                        let mods = crate::suppression::guard_fire_mods(g, gd);
                        g.gd[gd].fire_cd = (cd + crate::math_dice::sim_random(g) * jitter) * mods.gap;
                        let round = crate::aim::guard_round(g, gd, mods.spread, None);
                        crate::combat::fire_weapon(g, c, aim, false, Some(&round));
                        crate::aim::note_bore(g, gd, aim);
                        if crate::weapons::firearm_rounds_ready(g, weapon) == 0.0 { start_reload_beat(g, gd); }
                    }
                } else if let (true, Some(pc)) = (down, pc) {
                    let body = g.ch[pc].pos;
                    let here = g.ch[c].pos;
                    let d_b = m::hyp2(body[0] - here[0], body[2] - here[2]);
                    if d_b > 2.2 {
                        go_to(g, gd, body, None);
                        follow_path(g, gd, dt, g.tune.guard_investigate);
                        move_speed = g.gd[gd].speed;
                    } else {
                        brake(g, gd, 6.0, dt);
                        g.ch[c].vel = [0.0, 0.0, 0.0];
                    }
                    aim_at = Some(v3::add(body, [0.0, 0.3, 0.0]));
                    g.gd[gd].search_t += dt;
                    if g.gd[gd].search_t > 4.0 && !g.called_in && staff {
                        g.called_in = true;
                        let line = if !crate::dialogue::is_office(g) { "target is down. send someone over." } else { "target is down. send someone up." };
                        crate::game::game_say(g, gd, line, Some(true), None, None);
                        let at = head_or_pos(g, c);
                        seam_audio_play_rate(g, "radio", Some(at), 0.5, 0.95);
                    }
                    if g.gd[gd].search_t > 14.0 {
                        g.gd[gd].state = St::Suspicious;
                        g.gd[gd].awareness = 0.3;
                        g.gd[gd].search_t = 0.0;
                        g.gd[gd].react_t = 0.0;
                        set_fix(g, gd, body);
                    }
                } else {
                    aim_at = crate::coverfire::cover_blind(g, gd, dt);
                    if aim_at.is_some() {
                        brake(g, gd, 6.0, dt);
                        g.ch[c].vel = [0.0, 0.0, 0.0];
                    } else if let Some(forced) = crate::doorforce::door_force(g, gd, dt) {
                        aim_at = Some(forced.aim);
                        move_speed = forced.move_;
                    } else if let (Some(lk), true) = (g.gd[gd].last_known, g.gd[gd].pinned) {
                        let at = v3::add(lk, [0.0, 1.1, 0.0]);
                        aim_at = Some(at);
                        if crate::doorway::off_threshold(g, gd, Some(at), dt, false) {
                            move_speed = g.gd[gd].speed;
                        } else {
                            brake(g, gd, 6.0, dt);
                            g.ch[c].vel = [0.0, 0.0, 0.0];
                        }
                    } else if g.gd[gd].last_known.is_some() {
                        let tac = if g.tune.tactics { crate::tactics::tactics_alert(g, gd, dt) } else { None };
                        let (mv, at, pd, cr) = match tac {
                            Some(r) => (r.move_, r.aim, r.pied, r.crouch),
                            None => { let pace = g.tune.guard_chase; let r = chase_fix(g, gd, dt, pace); (r.move_, r.aim, r.pied, r.crouch) }
                        };
                        move_speed = mv; aim_at = at; pied = pd; crouch = cr;
                    } else if !g.gd[gd].pinned && !g.gd[gd].hold && g.time - g.gd[gd].last_seen_t > 2.0 {
                        drop_to_search(g, gd, None);
                    }
                }
            }
            St::Search => {
                use crate::guards::SearchPlanPhase as Ph;
                upper = UpperMode::Aim;
                if crate::teams::foe_down(g, gd) {
                    g.gd[gd].state = St::Alert;
                    g.gd[gd].react_t = 0.0;
                    g.gd[gd].search_t = 0.0;
                    g.gd[gd].search_plan = None;
                    break 'behaviour;
                }
                g.gd[gd].search_t += dt;
                g.gd[gd].sweep = m::sin(g.gd[gd].look_phase * 1.2) * 1.3;
                g.ch[c].anim.look_yaw_extra = m::sin(g.gd[gd].look_phase * 2.1) * 0.45;
                let mut sweep_k = 1.0;
                // P is gd.searchPlan itself: search_plan_for hands back a copy, so what it returns stands in the table and every read below goes there
                let dealt = search_plan_for(g, gd);
                g.gd[gd].search_plan = Some(dealt);
                let phase0 = g.gd[gd].search_plan.as_ref().unwrap().phase;
                if phase0 != Ph::Walk || dazzled { brake(g, gd, 6.0, dt); }
                search_probes(g, gd);
                let phase1 = g.gd[gd].search_plan.as_ref().unwrap().phase;
                if dazzled {
                    // rooted and sweeping (above)
                } else if phase1 == Ph::Sweep {
                    if g.gd[gd].search_t > SEARCH_SWEEP0_SECS { pick_next_spot(g, gd); }
                } else if phase1 == Ph::Walk {
                    let at = { let p = g.gd[gd].search_plan.as_ref().unwrap(); p.current as usize };
                    let s_pos = g.gd[gd].search_plan.as_ref().unwrap().spots[at].pos;
                    go_to(g, gd, s_pos, None);
                    let pie = pie_doorway(g, gd, dt, g.tune.guard_investigate, false);
                    pied = true;
                    let arrived = follow_path(g, gd, dt, pie.speed);
                    move_speed = g.gd[gd].speed;
                    sweep_k = 0.5;
                    let d_s = v3::dist_xz(g.ch[c].pos, s_pos);
                    let eye = eye_of_char(g, c);
                    let in_sight = d_s < 4.5 && !crate::scene_collision::segment_blocked(g, eye, [s_pos[0], 0.4, s_pos[2]], None, None, None);
                    if let Some(look) = pie.look {
                        aim_at = Some(look);
                        g.ch[c].anim.look_yaw_extra = 0.0;
                    } else if in_sight {
                        aim_at = Some([s_pos[0], 0.35, s_pos[2]]);
                        g.ch[c].anim.look_yaw_extra = 0.0;
                    }
                    if (in_sight && d_s < SEARCH_LOOK_DIST_M) || arrived {
                        let p = g.gd[gd].search_plan.as_mut().unwrap();
                        p.phase = Ph::Look;
                        p.phase_t = 0.0;
                        clear_path(g, gd);
                        g.gd[gd].stall_ref = None;
                    } else if not_getting_anywhere(g, gd, dt, 3.0) {
                        let by = g.ch[c].id;
                        let s = &mut g.gd[gd].search_plan.as_mut().unwrap().spots[at];
                        s.unreachable = true;
                        s.by = by;
                        pick_next_spot(g, gd);
                    }
                } else if phase1 == Ph::Look {
                    let at = g.gd[gd].search_plan.as_ref().unwrap().current as usize;
                    let s_pos = g.gd[gd].search_plan.as_ref().unwrap().spots[at].pos;
                    g.gd[gd].search_plan.as_mut().unwrap().phase_t += dt;
                    let phase_t = g.gd[gd].search_plan.as_ref().unwrap().phase_t;
                    brake(g, gd, 8.0, dt);
                    g.ch[c].vel = [0.0, 0.0, 0.0];
                    g.ch[c].anim.look_yaw_extra = 0.0;
                    let w = m::sin(phase_t * 4.0) * 0.35;
                    let to = v3::sub(s_pos, g.ch[c].pos);
                    let l = js::or(m::hyp2(to[0], to[2]), 1.0);
                    aim_at = Some([s_pos[0] - to[2] / l * w, 0.3, s_pos[2] + to[0] / l * w]);
                    if phase_t > SEARCH_LOOK_SECS {
                        let by = g.ch[c].id;
                        let p = g.gd[gd].search_plan.as_mut().unwrap();
                        p.spots[at].visited = true;
                        p.spots[at].by = by;
                        p.visits += 1.0;
                        pick_next_spot(g, gd);
                    }
                }
                let phase_end = g.gd[gd].search_plan.as_ref().unwrap().phase;
                let search_t = g.gd[gd].search_t;
                let over = search_t > SEARCH_MIN_SECS && (phase_end == Ph::Done || search_t > SEARCH_CAP_SECS);
                if over {
                    let here = g.ch[c].pos;
                    set_fix(g, gd, here);
                    g.gd[gd].state = St::Suspicious;
                    g.gd[gd].awareness = 0.25;
                    g.gd[gd].search_t = 0.0;
                    g.gd[gd].react_t = 0.0;
                    g.gd[gd].search_plan = None;
                    let line = if staff { "returning to patrol" } else if crate::teams::defender_words(g, gd) { "lost them." } else { "lost him." };
                    crate::game::game_say(g, gd, line, None, None, None);
                    let _ = crate::chronicle::chron_calm(g, gd);
                }
                if aim_at.is_none() {
                    g.ch[c].aim_yaw = g.ch[c].body_yaw + g.gd[gd].sweep * sweep_k;
                    g.ch[c].aim_pitch = v3::damp(g.ch[c].aim_pitch, -0.1, 3.0, dt);
                }
            }
            St::Dead => {}
        }
    }
    if !pied || (g.gd[gd].state != St::Alert && g.gd[gd].state != St::Search) { g.gd[gd].pieing = None; }
    let eyes_on_foe = g.gd[gd].state == St::Alert && aim_at.is_some() && g.time - g.gd[gd].last_seen_t < 0.5;
    if g.tune.tactics && !eyes_on_foe {
        if let Some(tl) = crate::tactics::tactics_look(g, gd) { aim_at = Some(tl.aim); crouch = m::max(crouch, tl.crouch); }
    }
    if !eyes_on_foe {
        if let Some(dl) = crate::darkdoor::dark_door_look(g, gd) { aim_at = Some(dl); }
    }
    if let Some(nq) = crate::netquiet::net_quiet_pose(g, gd, dt, eyes_on_foe) {
        if let Some(a) = nq.aim { aim_at = Some(a); }
        crouch = m::max(crouch, nq.crouch);
    }
    if let Some(a) = aim_at {
        let pos = g.ch[c].pos;
        let yaw_to = m::atan2(a[0] - pos[0], a[2] - pos[2]);
        g.ch[c].aim_yaw = v3::approach_angle(g.ch[c].aim_yaw, yaw_to, 6.0 * dt);
        let gun_y = pos[1] + 1.4;
        let hd = m::hyp2(a[0] - pos[0], a[2] - pos[2]);
        g.ch[c].aim_pitch = v3::damp(g.ch[c].aim_pitch, m::atan2(a[1] - gun_y, m::max(hd, 0.5)), 5.0, dt);
        let off = v3::wrap_angle(g.ch[c].aim_yaw - g.ch[c].body_yaw);
        if move_speed < 0.3 && m::abs(off) > 0.5 { g.ch[c].body_yaw = v3::approach_angle(g.ch[c].body_yaw, g.ch[c].aim_yaw, 5.0 * dt); }
    }
    let off = v3::wrap_angle(g.ch[c].aim_yaw - g.ch[c].body_yaw);
    if m::abs(off) > 1.5 { g.ch[c].aim_yaw = g.ch[c].body_yaw + m::sign(off) * 1.5; }
    collide_guard(g, gd, p0x, p0z);
    if crate::startle::startle_holds(g, gd) {
        upper = UpperMode::None;
        duck = duck || hand_carry(g, gd) != crate::guards::HandCarry::Torch;
    }
    let is_aim = upper == UpperMode::Aim;
    if g.gd[gd].muzzle_eval_t != g.time {
        let to = if is_aim { Some(match aim_at { Some(a) => a, None => aim_ahead(g, c) }) } else { None };
        muzzle_check(g, gd, to, dt, None);
    }
    if !g.gd[gd].pinned {
        let hot = g.gd[gd].state == St::Alert;
        crate::gunwall::gun_wall_check(g, gd, if is_aim { Some(crate::gunwall::GunUp::Aim) } else { None }, aim_at, dt, Some(hot));
    }
    if g.time < g.gd[gd].door_down_until { crouch = 1.0; }
    let lowered = g.gd[gd].muzzle_down || duck || staggered;
    {
        let a = &mut g.ch[c].anim;
        a.speed = move_speed;
        a.upper = if lowered { UpperMode::None } else { upper };
        a.crouch_target = crouch;
        a.lean = 0.0;
        a.stance = if lowered { Stance::LowReady } else { Stance::None };
    }
    crate::gunwall::gun_wall_pose(g, c);
    {
        let (vel, by) = (g.ch[c].vel, g.ch[c].body_yaw);
        g.ch[c].anim.reverse = move_speed > 0.2 && vel[0] * m::sin(by) + vel[2] * m::cos(by) < -0.3 * move_speed;
    }
    crate::character::character_update(g, c, dt);
    attach_flashlight(g, gd, Some(dt));
    let prev_phase = g.gd[gd].step_phase;
    g.gd[gd].step_phase = g.ch[c].anim.phase;
    if move_speed > 0.4 && ((prev_phase < 0.5) != (g.gd[gd].step_phase < 0.5)) {
        let pos = g.ch[c].pos;
        if g.k.tapstep.on { crate::tapstep::tap_hears(g, &pos); }
        let base = crate::teampace::team_tread(g, gd, move_speed, 0.33);
        let loud = crate::floor::tread(g, pos, base, false);
        if !staff {
            let (time, who, serial) = (g.time, g.ch[c].id, g.serial());
            g.events.push(crate::game::GameEvent { kind: crate::director::EventKind::Step, pos, time, loud: false, level: Some(loud), who: Some(who), serial, ..crate::game::GameEvent::zeroed() });
        }
    }
}
