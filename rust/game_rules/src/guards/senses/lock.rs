//! A LOCK KICKED IN, seen up close by a calm man of the staff: one look a door.
use crate::game::Game;
use crate::guards::*;
use crate::ids::*;
use crate::game::GuardState as St;
use crate::m;
use crate::v3::{self, V3};

pub fn lock(g: &mut Game, gd: GuardId, c: CharId, staff: bool) {
    // a kicked-in lock seen up close on a calm pass: one look per door
    if staff && g.gd[gd].state == St::Patrol && !g.gd[gd].hold && !g.quiet_utility {
        let mut i = 0;
        while i < g.doors.list.len() {
            let d = g.doors.list[i];
            i += 1;
            if !g.dr[d].leaf.lock_broken || g.dr[d].leaf.noticed { continue; }
            let f = g.dr[d].frame_centre;
            let pos = g.ch[c].pos;
            let (dx, dz) = (f[0] - pos[0], f[2] - pos[2]);
            let dd = m::hyp2(dx, dz);
            if dd > 2.2 || dd < 1e-3 { continue; }
            let look = crate::guards::look_of(g, c);
            if (look[0] * dx + look[2] * dz) / dd < m::cos(75.0 * v3::DEG) { continue; }
            let off = g.dr[d].leaf.off;
            let spot: V3 = if off {
                [f[0] - dx / dd * 0.45, 0.0, f[2] - dz / dd * 0.45]
            } else {
                let along = g.dr[d].def.width * 0.5;
                crate::doors::doors_work_spot(g, d, pos, Some(0.45), Some(along)).pos
            };
            let eye = eye_of_char(g, c);
            if crate::scene_collision::segment_blocked(g, eye, [spot[0], 1.0, spot[2]], None, None, None) { continue; }
            let t = g.time;
            g.dr[d].leaf.noticed = true;
            g.dr[d].leaf.noticed_t = t;
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
}
