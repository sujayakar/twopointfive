//! A COLLEAGUE ON THE FLOOR: the finder's call, the alarm, and who is called to it.
use crate::game::Game;
use crate::guards::*;
use crate::ids::*;
use crate::game::GuardState as St;
use crate::v3;
use alloc::string::String;

pub fn body(g: &mut Game, gd: GuardId, c: CharId, staff: bool, lvl: f64) {
    let now = g.time;
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
            let (mut n, mut cold) = (0, 0);
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
            }
            if g.gd[gd].state == St::Patrol || g.gd[gd].state == St::Search {
                crate::mind::set_state(g, gd, St::Suspicious, "guards");
                g.gd[gd].search_t = 0.0;
                clear_path(g, gd);
            }
            let line = words(g, o, n, cold);
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
}

/// what the finder calls in: `n` men here, `cold` of them breathing
fn words(g: &mut Game, o: GuardId, n: i32, cold: i32) -> String {
    let in_here = if !crate::dialogue::is_office(g) { "someone's here" } else { "someone's in here" };
    let who = crate::teams::spoken_name(g, o).unwrap_or_else(|| String::from("one of ours"));
    if n > 1 {
        if cold == n {
            alloc::format!("{} men down — out cold, {} breathing. {} — radioing it in", n, if n > 2 { "all" } else { "both" }, in_here)
        } else {
            alloc::format!("{} men down! — radioing it in", n)
        }
    } else if cold > 0 {
        alloc::format!("it's {} — he's down, out cold but breathing. someone dropped him — radioing it in", who)
    } else {
        String::from("man down! — radioing it in")
    }
}
