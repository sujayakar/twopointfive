//! SEARCH: he has lost his man and works through the spots he could be.
use super::{Acts, Sensed};
use crate::game::Game;
use crate::guards::*;
use crate::ids::*;
use crate::js;
use crate::m;
use crate::v3;
use crate::game::GuardState as St;
use crate::character::UpperMode;
use crate::guards::SearchPlanPhase as Ph;

pub(in crate::people) fn search(g: &mut Game, gd: GuardId, dt: f64, s: &Sensed) -> Acts {
    let &Sensed { c, staff, dazzled, .. } = s;
    let mut a = Acts { upper: UpperMode::Aim, ..Acts::still() };
    if crate::teams::foe_down(g, gd) {
        crate::mind::set_state(g, gd, St::Alert, "guards");
        g.gd[gd].react_t = 0.0;
        g.gd[gd].search_t = 0.0;
        g.gd[gd].search_plan = None;
        return a;
    }
    g.gd[gd].search_t += dt;
    g.gd[gd].sweep = m::sin(g.gd[gd].look_phase * 1.2) * 1.3;
    g.ch[c].anim.look_yaw_extra = m::sin(g.gd[gd].look_phase * 2.1) * 0.45;
    let mut sweep_k = 1.0;
    // P is gd.searchPlan itself: search_plan_for leaves it in the table and every read below goes there
    search_plan_for(g, gd);
    let phase0 = g.gd[gd].search_plan.as_ref().unwrap().phase;
    if phase0 != Ph::Walk || dazzled { brake(g, gd, 6.0, dt); }
    search_probes(g, gd);
    let phase1 = g.gd[gd].search_plan.as_ref().unwrap().phase;
    if dazzled {
        // rooted and sweeping (above)
    } else if phase1 == Ph::Sweep {
        if g.gd[gd].search_t > SEARCH_SWEEP0_SECS { pick_next_spot(g, gd); }
    } else if phase1 == Ph::Walk {
        walks_to_the_spot(g, gd, c, dt, &mut a);
        sweep_k = 0.5;
    } else if phase1 == Ph::Look {
        looks_at_the_spot(g, gd, c, dt, &mut a);
    }
    let phase_end = g.gd[gd].search_plan.as_ref().unwrap().phase;
    let search_t = g.gd[gd].search_t;
    if search_t > SEARCH_MIN_SECS && (phase_end == Ph::Done || search_t > SEARCH_CAP_SECS) { gives_it_up(g, gd, c, staff); }
    if a.aim_at.is_none() {
        g.ch[c].aim_yaw = g.ch[c].body_yaw + g.gd[gd].sweep * sweep_k;
        g.ch[c].aim_pitch = v3::damp(g.ch[c].aim_pitch, -0.1, 3.0, dt);
    }
    a
}

/// To the spot, pieing the doorway on the way, his eyes on it when he can; near enough he looks at it, and one he cannot reach is marked and passed over.
fn walks_to_the_spot(g: &mut Game, gd: GuardId, c: CharId, dt: f64, a: &mut Acts) {
    let at = g.gd[gd].search_plan.as_ref().unwrap().current as usize;
    let s_pos = g.gd[gd].search_plan.as_ref().unwrap().spots[at].pos;
    go_to(g, gd, s_pos, None);
    let pie = pie_doorway(g, gd, dt, g.k.tune.guard_investigate, false);
    a.pied = true;
    let arrived = follow_path(g, gd, dt, pie.speed);
    a.move_speed = g.gd[gd].speed;
    let d_s = v3::dist_xz(g.ch[c].pos, s_pos);
    let eye = eye_of_char(g, c);
    let in_sight = d_s < 4.5 && !crate::scene_collision::segment_blocked(g, eye, [s_pos[0], 0.4, s_pos[2]], None, None, None);
    if let Some(look) = pie.look {
        a.aim_at = Some(look);
        g.ch[c].anim.look_yaw_extra = 0.0;
    } else if in_sight {
        a.aim_at = Some([s_pos[0], 0.35, s_pos[2]]);
        g.ch[c].anim.look_yaw_extra = 0.0;
    }
    if (in_sight && d_s < SEARCH_LOOK_DIST_M) || arrived {
        let p = g.gd[gd].search_plan.as_mut().unwrap();
        p.phase = Ph::Look;
        p.phase_t = 0.0;
        g.gd[gd].drop_path();
        g.gd[gd].stall_ref = None;
    } else if not_getting_anywhere(g, gd, dt, 3.0) {
        let by = g.ch[c].id;
        let spot = &mut g.gd[gd].search_plan.as_mut().unwrap().spots[at];
        spot.unreachable = true;
        spot.by = by;
        pick_next_spot(g, gd);
    }
}

/// He stands and plays his gun over the spot for `SEARCH_LOOK_SECS`, then it counts as seen and he goes on to the next.
fn looks_at_the_spot(g: &mut Game, gd: GuardId, c: CharId, dt: f64, a: &mut Acts) {
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
    a.aim_at = Some([s_pos[0] - to[2] / l * w, 0.3, s_pos[2] + to[0] / l * w]);
    if phase_t > SEARCH_LOOK_SECS {
        let by = g.ch[c].id;
        let p = g.gd[gd].search_plan.as_mut().unwrap();
        p.spots[at].visited = true;
        p.spots[at].by = by;
        pick_next_spot(g, gd);
    }
}

/// The search has run its course: he stays suspicious where he stands and says he has lost him.
fn gives_it_up(g: &mut Game, gd: GuardId, c: CharId, staff: bool) {
    let here = g.ch[c].pos;
    crate::mind::fix(g, gd, Some(here), "guards");
    crate::mind::set_state(g, gd, St::Suspicious, "guards");
    crate::mind::set(g, gd, 0.25, "guards");
    g.gd[gd].search_t = 0.0;
    g.gd[gd].react_t = 0.0;
    g.gd[gd].search_plan = None;
    let line = if staff { "returning to patrol" } else if crate::teams::defender_words(g, gd) { "lost them." } else { "lost him." };
    crate::game::game_say(g, gd, line, None, None);
    let _ = crate::chronicle::chron_calm(g, gd);
}
