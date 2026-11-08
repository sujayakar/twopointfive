//! src/game/guards.ts lines 2107 to 2542
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use crate::guards::behave::{Acts, Sensed};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

// ---- small helpers: the TypeScript's own repeated phrases

/// `c.bones.head ?? v3.add(c.pos, [0, 1.65, 0])`
pub(crate) fn eye_of_char(g: &Game, c: CharId) -> V3 {
    match g.ch[c].bones.get(crate::character::Bone::Head).copied() {
        Some(h) => h,
        None => v3::add(g.ch[c].pos, [0.0, 1.65, 0.0]),
    }
}
/// `c.bones.head ?? c.pos`
pub(crate) fn head_or_pos(g: &Game, c: CharId) -> V3 {
    match g.ch[c].bones.get(crate::character::Bone::Head).copied() {
        Some(h) => h,
        None => g.ch[c].pos,
    }
}
/// `gd.path = []; gd.pathGoal = null;`: a whole write of both, so their identity words move with them.
pub(crate) fn clear_path(g: &mut Game, gd: GuardId) {
    let m = &mut g.gd[gd];
    m.path = Vec::new();
    m.path_serial = m.path_serial.wrapping_add(1);
    m.path_goal = None;
    m.path_goal_orbit = false;
}
/// `gd.speed = damp(gd.speed, 0, rate, dt)`
pub(crate) fn brake(g: &mut Game, gd: GuardId, rate: f64, dt: f64) {
    g.gd[gd].speed = v3::damp(g.gd[gd].speed, 0.0, rate, dt);
}
/// `const own = gd.script; gd.script = lent; gd.pieing = null; gd.aim.clear(); runGuardScript(g, gd, dt); gd.script = own;`
/// Gives back what the run left in the lent script: its lender reads that (conventions 4, an object lent), and its `<module>_script_ran` puts it home.
fn run_lent_script(g: &mut Game, gd: GuardId, lent: crate::game::GuardScript, dt: f64) -> Option<crate::game::GuardScript> {
    let own = g.gd[gd].script.take();
    g.gd[gd].script = Some(lent);
    g.gd[gd].pieing = None;
    crate::aim::aim_state_clear(&mut g.gd[gd].aim);
    crate::squad::run_guard_script(g, gd, dt);
    core::mem::replace(&mut g.gd[gd].script, own)
}

/// ONE MAN'S FRAME, part by part. Each part is a function below, in this order.   (guards.ts updateGuard)
///   incapable   dead or held: nothing below runs
///   lent        who has his body (control/): a rule of the list, or his own script; nothing below runs
///   perceive    his ears, his eyes, what they do to his awareness and his fix; a body found; a forced door
///   alertness   his state moves with his awareness; the clocks that floor his reaction
///   behave      his state's behaviour (behave/): where his legs go, what his gun does
///   pose        the looks laid over his aim, the muzzle and the wall, the animator, his light, his footfall
pub fn update_guard(g: &mut crate::game::Game, gd: GuardId, dt: f64) {
    use crate::game::GuardState as St;
    let c = g.gd[gd].ch;
    crate::character::character_turn_begin(g, c);
    player_ground_vel(g);
    if incapable(g, gd, dt) { return; }
    if g.gd[gd].gun_lost_at >= 0.0 { gun_back(g, gd); }
    if g.m.guards.held_last_frame.delete(gd) {
        g.m.guards.unstick_legs.delete(gd);
        unstick_begin(g, gd);
    }
    let staff = g.gd[gd].team == crate::teams::Team::Guard;
    if !staff { unit_light_meter(g, gd, dt); }
    if lent(g, gd, dt) { return; }
    let saw = perceive(g, gd, dt, c, staff);
    let s = alertness(g, gd, dt, c, staff, &saw);
    let acts = crate::guards::behave::act(g, gd, dt, &s);
    pose(g, gd, dt, &s, acts);
    crate::control::settle(g, gd);
}

/// Dead or held: his body is settled here and nothing else of his frame runs.
fn incapable(g: &mut Game, gd: GuardId, dt: f64) -> bool {
    use crate::game::GuardState as St;
    let c = g.gd[gd].ch;
    if g.gd[gd].state == St::Dead {
        crate::control::record(g, gd, crate::control::Holder::Dead);
        let m = &mut g.gd[gd];
        m.muzzle_down = false; m.pieing = None; m.search_plan = None; m.held = None; m.standoff = None; m.speed = 0.0;
        g.ch[c].vel = [0.0, 0.0, 0.0];
        g.ch[c].anim.speed = 0.0;
        crate::aim::aim_state_clear(&mut g.gd[gd].aim);
        crate::character::character_update(g, c, dt);
        crate::game::game_body_thud(g, c);
        attach_flashlight(g, gd, Some(dt));
        return true;
    }
    if g.gd[gd].held.is_some() {
        crate::control::record(g, gd, crate::control::Holder::Held);
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
        return true;
    }
    false
}

/// A rule of the body list has him, or his own script: his body follows it and nothing else of his frame runs.
fn lent(g: &mut Game, gd: GuardId, dt: f64) -> bool {
    use crate::character::UpperMode;
    // WHO HAS HIS BODY (control/): the rules of the list are asked from the top; below them his own script, below that his own state
    if let Some((rule, script)) = crate::control::grant_body(g, gd, dt) {
        crate::control::record(g, gd, crate::control::Holder::Rule(rule));
        let ran = run_lent_script(g, gd, script, dt);
        crate::control::commit(g, gd, rule, crate::control::Outcome::Script(ran));
        return true;
    }
    if g.gd[gd].script.is_some() {
        crate::control::record(g, gd, crate::control::Holder::Script);
        {
            let m = &mut g.gd[gd];
            m.pieing = None; m.search_plan = None; m.standoff = None;
            crate::aim::aim_state_clear(&mut m.aim);
            let s = m.script.as_ref().unwrap();
            let drawn = s.over.is_some() || s.upper.clone().unwrap_or(UpperMode::Aim) == UpperMode::Aim;
            m.drawn = drawn;
        }
        crate::squad::run_guard_script(g, gd, dt);
        return true;
    }
    false
}

/// what his ears and eyes told him this tick: the building's escalation, whether his eyes have his man, and how far
struct Saw { lvl: f64, seen: bool, dist: f64 }

/// His ears and his eyes, and what they do to his awareness and his fix; a downed colleague found; a mate's start taken up; a forced door noticed.
fn perceive(g: &mut Game, gd: GuardId, dt: f64, c: CharId, staff: bool) -> Saw {
    use crate::game::GuardState as St;
    use crate::character::{Bone, Stance, UpperMode};
    let (lvl, mut seen, mut dist);
    lvl = escalation_of(g);
    g.gd[gd].drawn = lvl >= 1.0 || !staff;
    // --- perception ---
    seen = false;
    dist = 99.0;
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
                crate::mind::set(g, gd, v3::clamp(was + rate * dt, 0.0, 1.0), "guards");
                if !far_faint(g, gd, dist, was) {
                    let gain = g.gd[gd].awareness - was;
                    if !crate::attention::attention_aside(g, gd, pp, gain) { crate::mind::fix(g, gd, Some(pp), "guards"); }
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
            let on_fix = fresh && crate::belief::fix_at_sight(g, gd);
            if let (true, Some(pc)) = (on_fix, fc) {
                let eye = eye_of_char(g, c);
                let chest = match g.ch[pc].bones.get(Bone::Chest).copied() { Some(b) => b, None => v3::add(g.ch[pc].pos, [0.0, 1.1, 0.0]) };
                if eye_blocked(g, gd, eye, chest) {
                    let head = match g.ch[pc].bones.get(Bone::Head).copied() { Some(b) => b, None => v3::add(g.ch[pc].pos, [0.0, 1.6, 0.0]) };
                    if eye_blocked(g, gd, eye, head) {
                        let (sp, sv) = (g.gd[gd].sight_pos, g.gd[gd].sight_vel);
                        let led = lead_fix(g, sp, sv);
                        crate::mind::fix(g, gd, Some(led), "guards");
                        g.gd[gd].led_at = Some(led);
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
                crate::mind::raise(g, gd, 0.7, "guards");
                crate::mind::fix(g, gd, Some(body), "guards");
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
                    crate::mind::set_state(g, gd, St::Suspicious, "guards");
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
                    crate::mind::raise(g, other, 0.45, "guards");
                    let breathing = crate::game::is_breathing(g, o);
                    let t = g.time;
                    g.gd[other].witness.called_to_body = Some(crate::game::WitnessCalledToBody { victim: victim.clone(), t, breathing });
                    if g.gd[other].task.is_some() { continue; }
                    crate::belief::word(g, other, body);
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
                crate::mind::set_state(g, gd, St::Suspicious, "guards");
                { let to = m::max(m::max(g.gd[gd].awareness, m::min(g.gd[l].awareness, 0.6)), 0.34); crate::mind::set(g, gd, to, "guards"); }
                g.gd[gd].react_t = g.gd[l].react_t + 0.3;
                g.gd[gd].search_t = 0.0;
                let fix = if crate::scene_collision::nav_grid_walkable(g, lk, side) { side } else { lk };
                crate::mind::fix(g, gd, Some(fix), "guards");
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
                let look = crate::guards::look_of(g, c);
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
                crate::mind::set_state(g, gd, St::Suspicious, "guards");
                crate::mind::raise(g, gd, 0.45, "guards");
                g.gd[gd].react_t = 1.1;
                g.gd[gd].search_t = 0.0;
                crate::mind::fix(g, gd, Some(spot), "guards");
                clear_path(g, gd);
                crate::game::game_say(g, gd, if off { "this door's off its hinges…" } else { "this lock's been kicked in…" }, None, None, None);
                crate::audio_iface::game_audio_like_play(g, "radio", Some(eye), Some(0.4), Some(&crate::audio_iface::PlayOpts::Rate { rate: 1.1 }));
                break;
            }
        }
        let drain = dt * (match g.gd[gd].state { St::Alert => 0.03, St::Patrol => 0.1, _ => 0.06 });
        if !seen { let to = m::max(0.0, g.gd[gd].awareness - drain); crate::mind::set(g, gd, to, "guards"); }
    } else {
        { let to = m::max(0.0, g.gd[gd].awareness - dt * 0.5); crate::mind::set(g, gd, to, "guards"); }
    }
    Saw { lvl, seen, dist }
}

/// His state moves with his awareness (calm, suspicious, alert), the contact call, the clocks that floor his reaction.
fn alertness(g: &mut Game, gd: GuardId, dt: f64, c: CharId, staff: bool, saw: &Saw) -> Sensed {
    use crate::game::GuardState as St;
    let Saw { lvl, seen, dist } = *saw;
    // --- state transitions ---
    let prev = g.gd[gd].state;
    if g.gd[gd].awareness >= 0.85 && g.gd[gd].state != St::Alert {
        reset_chase(g, gd);
        crate::mind::set_state(g, gd, St::Alert, "guards");
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
        crate::mind::set_state(g, gd, St::Suspicious, "guards");
        g.gd[gd].search_t = 0.0;
        if g.gd[gd].body_duty.is_some() {
            g.gd[gd].react_t = 0.5 + crate::math_dice::sim_random(g) * 0.5;
        } else {
            g.gd[gd].react_t = 0.9;
            if g.gd[gd].team != crate::teams::Team::Operative && !crate::attention::attention_again(g, gd) {
                crate::game::game_say(g, gd, "huh?", None, None, None);
            }
            let at = head_or_pos(g, c);
            crate::audio_iface::game_audio_like_play(g, "radio", Some(at), Some(0.45), Some(&crate::audio_iface::PlayOpts::Rate { rate: 1.15 }));
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
    let dazzled = crate::control::man::blind(g, gd);
    if dazzled { g.gd[gd].react_t = m::max(g.gd[gd].react_t, g.gd[gd].dazzled_until - g.time); }
    let staggered = crate::control::man::staggered(g, gd);
    if staggered { g.gd[gd].react_t = m::max(g.gd[gd].react_t, g.gd[gd].stagger_until - g.time); }
    { let st = crate::startle::startle_left(g, gd); if st > 0.0 { g.gd[gd].react_t = m::max(g.gd[gd].react_t, st); } }
    crate::aim::track_guard(g, gd, dt);
    let (p0x, p0z) = (g.ch[c].pos[0], g.ch[c].pos[2]);

    g.ch[c].anim.look_yaw_extra = 0.0;
    g.gd[gd].look_phase += dt;
    Sensed { c, route: g.gd[gd].route, staff, lvl, dist, pair, dazzled, staggered, p0x, p0z }
}

/// The looks laid over his aim, the muzzle and the wall, the animator, his light, his footfall.
fn pose(g: &mut Game, gd: GuardId, dt: f64, s: &Sensed, acts: Acts) {
    use crate::game::GuardState as St;
    use crate::character::{Stance, UpperMode};
    let &Sensed { c, staff, staggered, p0x, p0z, .. } = s;
    let Acts { move_speed, mut upper, mut aim_at, mut crouch, mut duck, pied } = acts;
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
    if crate::startle::startle_gun_down(g, gd) {
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
        let base = crate::teampace::team_tread(g, gd, move_speed, 0.33);
        let loud = crate::floor::tread(g, pos, base, false);
        if !staff {
            let (time, who, serial) = (g.time, g.ch[c].id, g.serial());
            g.events.push(crate::game::GameEvent { kind: crate::director::EventKind::Step, pos, time, loud: false, level: Some(loud), who: Some(who), serial, ..crate::game::GameEvent::zeroed() });
        }
    }
}

