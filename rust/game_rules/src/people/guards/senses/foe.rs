//! HIS EYES ON HIS MAN: who that is, how much of him his eyes take in, and what it does to his awareness and his fix.
use crate::game::Game;
use crate::guards::*;
use crate::ids::*;
use crate::v3;

/// what his eyes have of his man: whether they have him this frame, how far he is, and his body
pub struct Eyes { pub seen: bool, pub dist: f64, pub body: Option<CharId> }

pub(in crate::people) fn eyes(g: &mut Game, gd: GuardId, dt: f64, staff: bool) -> Eyes {
    let mut seen = false;
    if staff && crate::teams::is_hostile(g, crate::teams::Team::Guard, crate::teams::Team::Operative) {
        acquire_foe(g, gd);
        if g.gd[gd].foe.is_none() { crate::mind::foe(g, gd, Some(crate::game::Foe::Player), "guards"); }
    } else if !staff { acquire_foe(g, gd); }
    let f = g.gd[gd].foe.clone();
    let fc = crate::teams::foe_char(g, gd);
    let (visible, dist) = match (&f, fc) {
        (Some(crate::game::Foe::Player), _) => { let v = can_see(g, gd); (v.visible, v.dist) }
        (Some(_), Some(fch)) => { let v = can_see_char(g, gd, fch); (v.visible, v.dist) }
        _ => { let v = no_sight(); (v.visible, v.dist) }
    };
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
    Eyes { seen, dist, body: fc }
}
