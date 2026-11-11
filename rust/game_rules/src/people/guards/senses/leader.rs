//! THE MAN HE TRAILS HAS STARTED AT SOMETHING: he takes it as seriously.
use crate::game::Game;
use crate::guards::*;
use crate::ids::*;
use crate::game::GuardState as St;
use crate::m;
use crate::V3;

pub(in crate::people) fn leader(g: &mut Game, gd: GuardId, lvl: f64) {
    // lockdown pair: the man he trails has started at something: he takes it as seriously, a step to that man's side of the spot
    if let (true, Some(l)) = (lvl >= 2.0, g.gd[gd].leader) {
        if g.gd[gd].state == St::Patrol && !g.gd[gd].duty.holds && g.gd[l].state == St::Suspicious && g.gd[l].react_t > 0.0 && g.gd[l].last_known.is_some() && g.gd[l].errand.is_none() {
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
            g.gd[gd].drop_path();
        }
    }
}
