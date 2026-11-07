//! The stalk: the last metres to a man's back as one held click (src/game/stalk.ts). The state row is keyed by `g.player`.
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// A new row, every key from birth. (stalk.ts fresh)
pub fn fresh() -> crate::stalk::St {
    St { on: None, grab: None, t: -1.0, wait: Some(false), armed: Some(false), stick: None, orbit_r: None, orbit_t: None, orbit_ask: None, broke: None }
}

/// This seat's row, made if it is not there. (stalk.ts stateOf)
pub fn state_of<'g>(g: &'g mut crate::game::Game) -> &'g mut crate::stalk::St {
    let me = g.player;
    g.m.stalk.state.get_or_insert_with(me, fresh)
}

/// The stalk is over, for `why` if one was on. (stalk.ts end)
pub fn end(g: &mut crate::game::Game, pl: PlayerId, why: crate::stalk::Why) {
    let r = g.m.stalk.state.get_mut(pl).unwrap();
    if r.on.is_some() {
        r.armed = Some(false);
        r.on = None;
        g.stats.stalk.out[why as usize] += 1;
        g.stats.stalk.last = String::from(why.as_str());
    }
    let r = g.m.stalk.state.get_mut(pl).unwrap();
    r.orbit_r = None;
    r.orbit_t = None;
    r.orbit_ask = None;
}

/// His bearing to a point, off his facing (0 = dead ahead). (stalk.ts bearingOf)
pub fn bearing_of(g: &mut crate::game::Game, gd: GuardId, p: &[f64]) -> f64 {
    let c = g.gd[gd].ch;
    let pos = g.ch[c].pos;
    v3::wrap_angle(m::atan2(p[0] - pos[0], p[2] - pos[2]) - g.ch[c].body_yaw)
}

/// Are Sam's hands free for it at all: empty, and nothing else has him. (stalk.ts hands)
pub fn hands(g: &mut crate::game::Game) -> bool {
    let me = g.player;
    crate::close::close_live(g)
        && g.puppet.is_none()
        && g.pl[me].holstered
        && !g.pl[me].down
        && g.pl[me].holding.is_none()
        && g.pl[me].takedown.is_none()
        && g.pl[me].dragging.is_none()
        && g.pl[me].kick.is_none()
        && g.pl[me].picking.is_none()
        && g.pl[me].wanding.is_none()
        && g.pl[me].wall.is_none()
        && !crate::game::player_list_open(g, me)
        && able(g)
}

/// Is the held click his to begin one with. (stalk.ts free)
pub fn free(g: &mut crate::game::Game) -> bool {
    hands(g) && !shut(g)
}

/// A tutorial step has taken the click. (stalk.ts shut)
pub fn shut(g: &mut crate::game::Game) -> bool {
    crate::rails::rails_shuts(g, "stalk")
}

/// The gate game.ts puts on a grab: not folded by a blast, no hand on a breaker's lever. (stalk.ts able)
pub fn able(g: &mut crate::game::Game) -> bool {
    let me = g.player;
    !crate::samstun::sam_staggered(g, None) && g.pl[me].breaker_move.is_none()
}

/// How far off a press can begin one. (stalk.ts beginM)
pub fn begin_m(g: &mut crate::game::Game) -> f64 {
    m::max(g.k.close.mode_m, g.k.stalk.low_begin_m)
}

/// How far off one is given up. (stalk.ts outM)
pub fn out_m(g: &mut crate::game::Game) -> f64 {
    m::max(g.k.close.mode_out_m, g.k.stalk.low_out_m)
}

/// (stalk.ts stalkBeginM)
pub fn stalk_begin_m(g: &mut crate::game::Game) -> f64 {
    begin_m(g)
}

/// (stalk.ts stalkOutM)
pub fn stalk_out_m(g: &mut crate::game::Game) -> f64 {
    out_m(g)
}

/// Is the held click the ONE way to grab a man from behind: on the page, only on a wall. (stalk.ts stalkClickOnly)
pub fn stalk_click_only(g: &mut crate::game::Game) -> bool {
    let me = g.player;
    g.pl[me].wall.is_some() && !shut(g)
}

/// Why this man cannot be stalked from `p` right now, or None when he can (`begin`: a press's stricter test). (stalk.ts outOf)
pub fn out_of(g: &mut crate::game::Game, gd: GuardId, p: &[f64], begin: bool) -> Option<crate::stalk::Why> {
    if g.gd[gd].civ.is_some() || !crate::close::close_subject(g, gd) || g.gd[gd].state == crate::game::GuardState::Alert {
        return Some(Why::Him);
    }
    let pos = g.ch[g.gd[gd].ch].pos;
    let d = m::hyp2(p[0] - pos[0], p[2] - pos[2]);
    let limit = if begin { begin_m(g) } else { out_m(g) };
    if d >= limit {
        return Some(Why::Far);
    }
    let edge = if begin { g.k.stalk.begin_cos } else { g.k.stalk.out_cos };
    if m::cos(bearing_of(g, gd, p)) > edge {
        return Some(Why::Turned);
    }
    if !crate::close::clear_between(g, &pos, p) {
        return Some(Why::Wall);
    }
    None
}

/// Could a press on this man begin one from where Sam stands. (stalk.ts stalkCould)
pub fn stalk_could(g: &mut crate::game::Game, gd: GuardId) -> bool {
    if !free(g) {
        return false;
    }
    let p = g.ch[g.pl[g.player].ch].pos;
    out_of(g, gd, &p, true).is_none()
}

/// The man a press at the cursor is on: the reticle's if he can be stalked, else the nearest within `cursorM` of the floor point. (stalk.ts pressed)
pub fn pressed(g: &mut crate::game::Game) -> Option<GuardId> {
    let p = g.ch[g.pl[g.player].ch].pos;
    let a = g.aim_point;
    if let Some(ag) = g.aim_guard {
        if out_of(g, ag, &p, true).is_none() {
            return Some(ag);
        }
    }
    let mut best: Option<GuardId> = None;
    let mut bd = g.k.stalk.cursor_m;
    let mut i = 0;
    while i < g.guards.len() {
        let gd = g.guards[i];
        i += 1;
        let pos = g.ch[g.gd[gd].ch].pos;
        let d = m::hyp2(a[0] - pos[0], a[2] - pos[2]);
        if d < bd && out_of(g, gd, &p, true).is_none() {
            best = Some(gd);
            bd = d;
        }
    }
    best
}

/// May the wait at a corner be his this frame: on a wall (not the stack), hands empty, room to come up. (stalk.ts cornerGate)
pub fn corner_gate(g: &mut crate::game::Game) -> bool {
    let me = g.player;
    let (kind, drawn) = match &g.pl[me].wall {
        None => return false,
        Some(w) => (w.kind, w.drawn),
    };
    !(kind == Some(crate::player::WallHoldKind::Stack)
        || drawn
        || !crate::close::close_live(g)
        || g.puppet.is_some()
        || !g.pl[me].holstered
        || g.pl[me].down
        || g.pl[me].holding.is_some()
        || g.pl[me].takedown.is_some()
        || g.pl[me].dragging.is_some()
        || g.pl[me].kick.is_some()
        || g.pl[me].picking.is_some()
        || g.pl[me].wanding.is_some()
        || crate::game::player_list_open(g, me)
        || !able(g)
        || shut(g)
        || (g.pl[me].crouch && !crate::player::can_stand(g)))
}

/// The man the wait at a corner takes this frame: the nearest who has come past him. (stalk.ts cornerMan)
pub fn corner_man(g: &mut crate::game::Game) -> Option<GuardId> {
    if !corner_gate(g) {
        return None;
    }
    let p = g.ch[g.pl[g.player].ch].pos;
    let cos = m::cos(g.k.stalk.corner_deg * m::PI / 180.0);
    let mut best: Option<GuardId> = None;
    let mut bd = g.k.stalk.corner_m;
    let mut i = 0;
    while i < g.guards.len() {
        let gd = g.guards[i];
        i += 1;
        if g.gd[gd].civ.is_some() || !crate::close::close_subject(g, gd) || g.gd[gd].state == crate::game::GuardState::Alert {
            continue;
        }
        let pos = g.ch[g.gd[gd].ch].pos;
        let d = m::hyp2(p[0] - pos[0], p[2] - pos[2]);
        if d < bd && m::cos(bearing_of(g, gd, &p)) <= cos && crate::close::clear_between(g, &pos, &p) {
            best = Some(gd);
            bd = d;
        }
    }
    best
}

/// The animator's soft duck, away from this man. (stalk.ts duck)
pub fn duck(g: &mut crate::game::Game, gd: GuardId, k: f64) {
    let c = g.pl[g.player].ch;
    let p = g.ch[c].pos;
    let pos = g.ch[g.gd[gd].ch].pos;
    crate::character::anim_flinch_from(g, c, Some([pos[0] - p[0], 0.0, pos[2] - p[2]]), Some(k), Some(true));
}

/// The man who turned off his back: if he comes on round to face where Sam stood inside `joltSecs`, or is onto him, Sam ducks. (stalk.ts brokeDuck)
pub fn broke_duck(g: &mut crate::game::Game, pl: PlayerId) {
    let row = *g.m.stalk.state.get(pl).unwrap();
    let b = match row.broke {
        Some(b) => b,
        None => return,
    };
    let p = g.ch[g.pl[pl].ch].pos;
    if g.time - b.t > g.k.stalk.jolt_secs
        || row.on.is_some()
        || !free(g)
        || !crate::close::close_subject(g, b.gd)
        || m::hyp2(p[0] - b.at[0], p[2] - b.at[2]) > 1.0
    {
        g.m.stalk.state.get_mut(pl).unwrap().broke = None;
        return;
    }
    let pos = g.ch[g.gd[b.gd].ch].pos;
    if !crate::close::clear_between(g, &pos, &p) {
        return;
    }
    if g.gd[b.gd].state == crate::game::GuardState::Alert {
        duck(g, b.gd, 1.0);
        g.m.stalk.state.get_mut(pl).unwrap().broke = None;
    } else if m::abs(bearing_of(g, b.gd, &b.at)) <= g.k.stalk.jolt_face * m::PI / 180.0 {
        let jolt = g.k.stalk.jolt;
        duck(g, b.gd, jolt);
        g.m.stalk.state.get_mut(pl).unwrap().broke = None;
    }
}

/// The polish, up or down together, and the hold's share of it. (stalk.ts stalkPolish)
pub fn stalk_polish(g: &mut crate::game::Game, up: bool) {
    let (turn_in, corner_show, yank_with, catch_long, jolt) = if up {
        let u = polish_up();
        (u.turn_in, u.corner_show, u.yank_with, u.catch_long, u.jolt)
    } else {
        let u = polish_down();
        (u.turn_in, u.corner_show, u.yank_with, u.catch_long, u.jolt)
    };
    g.k.stalk.turn_in = turn_in;
    g.k.stalk.corner_show = corner_show;
    g.k.stalk.yank_with = yank_with;
    g.k.stalk.catch_long = catch_long;
    g.k.stalk.jolt = jolt;
    crate::character::hold_polish(g, up);
}

/// Once a frame from updatePlayer, before his keys become a step: what the stalk asks of his feet, or None = his own. (stalk.ts stalkStep)
pub fn stalk_step(g: &mut crate::game::Game, lmb: bool, lmb_hit: bool, own_keys: bool, stick: Option<crate::stalk::StalkStick>, keys: Option<crate::stalk::StalkKeys>) -> Option<crate::stalk::StalkMove> {
    let me = g.player;
    let now = g.time;
    let r = state_of(g);
    r.t = now;
    r.grab = None;
    r.wait = Some(false);
    broke_duck(g, me);
    if !free(g) {
        end(g, me, Why::Hands);
        if lmb || lmb_hit {
            let wait = corner_gate(g);
            g.m.stalk.state.get_mut(me).unwrap().wait = Some(wait);
            if let Some(gd) = corner_man(g) {
                g.m.stalk.state.get_mut(me).unwrap().grab = Some(gd);
                g.stats.stalk.grabs += 1;
                g.stats.stalk.corner += 1;
                g.stats.stalk.last = String::from("corner");
            }
        }
        return None;
    }
    if lmb_hit {
        g.m.stalk.state.get_mut(me).unwrap().armed = Some(true);
    } else if !lmb {
        g.m.stalk.state.get_mut(me).unwrap().armed = Some(false);
    }
    // a press made with a move key still down waits for the key
    let had = g.m.stalk.state.get(me).unwrap().on;
    if had.is_none() && g.m.stalk.state.get(me).unwrap().armed == Some(true) && !own_keys {
        if let Some(gd) = pressed(g) {
            let r = g.m.stalk.state.get_mut(me).unwrap();
            r.on = Some(gd);
            r.armed = Some(false);
            r.stick = stick.map(|k| [k.x, k.y]);
            g.stats.stalk.begun += 1;
        }
    }
    let gd = g.m.stalk.state.get(me).unwrap().on?;
    if !lmb && !lmb_hit {
        end(g, me, Why::Let);
        return None;
    }
    let round = own_keys && keys.is_some();
    if own_keys && !round {
        end(g, me, Why::Keys);
        return None;
    }
    let p = g.ch[g.pl[me].ch].pos;
    if let Some(why) = out_of(g, gd, &p, false) {
        // broken off by the man himself: at once when he is onto Sam; after a turn, only if he comes on round (brokeDuck). Not on the frame a stalk began.
        if had == Some(gd) && g.k.stalk.jolt > 0.0 {
            if why == Why::Him {
                duck(g, gd, 1.0);
            } else if why == Why::Turned {
                let t = g.time;
                g.m.stalk.state.get_mut(me).unwrap().broke = Some(StBroke { gd, t, at: [p[0], p[1], p[2]] });
            }
        }
        end(g, me, why);
        broke_duck(g, me);
        return None;
    }
    let c = g.gd[gd].ch;
    let gp = g.ch[c].pos;
    let at: V3 = [gp[0], gp[1] + 1.2, gp[2]];
    let dist = m::hyp2(p[0] - gp[0], p[2] - gp[2]);
    let (grab_m, key_m) = (g.k.stalk.grab_m, g.k.stalk.key_m);
    let mut hold: Option<f64> = None; // keeping station: how far outside `keyM` he is, −1‥1
    if dist < grab_m {
        let o = crate::close::close_at(g, gd, &p);
        let offered = o.can && o.grab;
        let wait = offered || o.win.as_ref().map_or(false, |w| w.grab);
        if wait && !offered && !round {
            return Some(StalkMove { mx: 0.0, mz: 0.0, pace: 0.0, at, keep: None });
        }
        if wait {
            hold = Some(v3::clamp((dist - key_m) / m::max(1e-3, grab_m - key_m), -1.0, 1.0));
        }
    }
    if let (true, Some(k)) = (round, keys) {
        let l = js::or(m::hyp2(k.mx, k.mz), 1.0);
        let pace = g.k.stalk.pace;
        return Some(orbited(g, me, gd, &p, at, dist, k.mx / l, k.mz / l, pace, k.dt));
    }
    if g.k.stalk.drag {
        if let (Some(st), Some(mid)) = (stick, g.m.stalk.state.get(me).unwrap().stick) {
            return Some(dragged(g, me, gd, &p, at, dist, st, mid, 1.0 / 60.0));
        }
    }
    // toward the point behind his back
    let yaw = g.ch[c].body_yaw;
    let station = g.k.stalk.station_m;
    let bx = gp[0] - m::sin(yaw) * station - p[0];
    let bz = gp[2] - m::cos(yaw) * station - p[2];
    let bl = js::or(m::hyp2(bx, bz), 1.0);
    let vel = g.ch[c].vel;
    let his = m::hyp2(vel[0], vel[2]);
    if let Some(hold) = hold {
        let ease = g.k.stalk.close_min * hold;
        let pace = m::min(g.k.stalk.pace, m::max(0.0, his + (if m::abs(ease) < 0.02 { 0.0 } else { ease })));
        return Some(StalkMove { mx: bx / bl, mz: bz / bl, pace, at, keep: Some(true) });
    }
    let pace = m::min(g.k.stalk.pace, his + g.k.stalk.close_min + v3::clamp(dist - grab_m, 0.0, 4.0) * g.k.stalk.low_k);
    Some(StalkMove { mx: bx / bl, mz: bz / bl, pace, at, keep: None })
}

/// The stick's step: the mouse's way from the press as a way across the floor, taken round the man. (stalk.ts dragged)
pub fn dragged(g: &mut crate::game::Game, pl: PlayerId, gd: GuardId, p: &[f64], at: V3, dist: f64, st: crate::stalk::StalkStick, mid: [f64; 2], dt: f64) -> crate::stalk::StalkMove {
    let (dx, dy) = (st.x - mid[0], st.y - mid[1]);
    let mag = m::hyp2(dx, dy);
    let k = v3::clamp((mag - g.k.stalk.drag_dead) / m::max(1.0, g.k.stalk.drag_full - g.k.stalk.drag_dead), 0.0, 1.0);
    if k <= 0.0 {
        return StalkMove { mx: 0.0, mz: 0.0, pace: 0.0, at, keep: Some(true) };
    }
    // screen down is the camera's back
    let mut mx = st.right[0] * dx - st.fwd[0] * dy;
    let mut mz = st.right[2] * dx - st.fwd[2] * dy;
    let l = js::or(m::hyp2(mx, mz), 1.0);
    mx /= l;
    mz /= l;
    let pace = g.k.stalk.pace * k;
    orbited(g, pl, gd, p, at, dist, mx, mz, pace, dt)
}

/// The share of a way into the man left to it at `dist`: all until 20 cm outside `keyM`, none from 4 cm outside. (stalk.ts intoKeep)
pub fn into_keep(g: &mut crate::game::Game, dist: f64) -> f64 {
    v3::clamp((dist - g.k.stalk.key_m - 0.04) / 0.16, 0.0, 1.0)
}

/// A way across the floor taken round the man, split as first asked into its share toward him and its share round him. (stalk.ts orbited)
pub fn orbited(g: &mut crate::game::Game, pl: PlayerId, gd: GuardId, p: &[f64], at: V3, dist: f64, dx: f64, dz: f64, pace: f64, dt: f64) -> crate::stalk::StalkMove {
    let c = g.gd[gd].ch;
    let gp = g.ch[c].pos;
    let d = m::max(1e-3, dist);
    let (ux, uz) = ((gp[0] - p[0]) / d, (gp[2] - p[2]) / d); // toward him
    let now = g.time;
    let row = *g.m.stalk.state.get(pl).unwrap();
    // now < orbitT: a load has put the clock back
    let stale = match (row.orbit_r, row.orbit_t) {
        (Some(_), Some(t)) => now - t > 0.1 || now < t,
        _ => true,
    };
    let swung = match row.orbit_ask {
        Some(a) => dx * a[0] + dz * a[1] < m::cos(m::PI / 6.0),
        None => true,
    };
    if stale || swung {
        let r0 = dx * ux + dz * uz;
        let sn = v3::clamp(g.k.stalk.orbit_snap, 0.0, 0.9);
        let a0 = m::abs(r0);
        let r1 = if a0 <= sn { 0.0 } else { m::sign(r0) * (a0 - sn) / (1.0 - sn) }; // nearly across him is across him
        let t1 = m::sqrt(m::max(0.0, 1.0 - r1 * r1));
        g.m.stalk.state.get_mut(pl).unwrap().orbit_ask = Some([dx, dz, r1, if ux * dz - uz * dx >= 0.0 { t1 } else { -t1 }]);
    }
    let ask = g.m.stalk.state.get(pl).unwrap().orbit_ask.unwrap();
    let mut r = ask[2];
    let mut tx = -uz * ask[3];
    let mut tz = ux * ask[3];
    if r > 0.0 {
        r *= into_keep(g, dist); // the last 20 cm into him taken off
    }
    let tl = m::hyp2(tx, tz);
    if tl > 1e-4 {
        let ahead = 0.05 + 0.1 * pace; // a tenth of a second of the way round: the ease of his stop carries him about that far
        let c0 = m::cos(bearing_of(g, gd, p));
        let c1 = m::cos(bearing_of(g, gd, &[p[0] + tx / tl * ahead, 0.0, p[2] + tz / tl * ahead]));
        if c1 > c0 && c1 > m::min(g.k.stalk.out_cos, crate::close::BEHIND_COS) - g.k.stalk.orbit_edge {
            tx = 0.0;
            tz = 0.0;
        }
    }
    let was = if stale { dist } else { row.orbit_r.unwrap() };
    let held = v3::clamp(was - r * pace * dt, m::max(g.k.stalk.key_m, dist - 0.15), dist + 0.15);
    {
        let w = g.m.stalk.state.get_mut(pl).unwrap();
        w.orbit_r = Some(held);
        w.orbit_t = Some(now);
    }
    let pull = v3::clamp(g.k.stalk.orbit_k * (dist - held), -1.0, 1.0); // the circle is round the man where he is
    let hv = g.ch[c].vel;
    let vx = (tx + ux * r + ux * pull) * pace + hv[0];
    let vz = (tz + uz * r + uz * pull) * pace + hv[2];
    let v = m::hyp2(vx, vz);
    if v < 1e-3 {
        return StalkMove { mx: 0.0, mz: 0.0, pace: 0.0, at, keep: Some(true) };
    }
    StalkMove { mx: vx / v, mz: vz / v, pace: m::min(v, g.k.stalk.pace + m::hyp2(hv[0], hv[2])), at, keep: Some(true) }
}

/// (from − d) / (from − to), held to 0‥1: straight by the distance. (stalk.ts stalkTell `by`)
fn by(d: f64, from: f64, to: f64) -> f64 {
    v3::clamp((from - d) / m::max(0.05, from - to), 0.0, 1.0)
}

/// The man Sam's body speaks of, where the hands go, and how far he is sunk. (stalk.ts stalkTell)
pub fn stalk_tell(g: &mut crate::game::Game) -> Option<crate::stalk::StalkTell> {
    if !free(g) {
        return None;
    }
    let me = g.player;
    let p = g.ch[g.pl[me].ch].pos;
    let mut found = g.m.stalk.state.get(me).and_then(|s| s.on);
    let mut bd = m::INF;
    if found.is_none() {
        let mut i = 0;
        while i < g.guards.len() {
            let x = g.guards[i];
            i += 1;
            let pos = g.ch[g.gd[x].ch].pos;
            let d = m::hyp2(p[0] - pos[0], p[2] - pos[2]);
            if d < bd && d < begin_m(g) && out_of(g, x, &p, true).is_none() {
                found = Some(x);
                bd = d;
            }
        }
    }
    let gd = found?;
    let c = g.gd[gd].ch;
    let gp = g.ch[c].pos;
    let d = m::hyp2(p[0] - gp[0], p[2] - gp[2]);
    let neck = g.ch[c].bones.get(crate::character::Bone::Neck).or(g.ch[c].bones.get(crate::character::Bone::Head)).copied();
    let at: V3 = match neck {
        Some(n) => [n[0], n[1], n[2]],
        None => [gp[0], gp[1] + 1.45, gp[2]],
    };
    let low = if g.pl[me].crouch { by(d, g.k.stalk.low_from, m::max(g.k.stalk.grab_m, g.k.stalk.low_by)) } else { 0.0 }; // from his crouch: a man walked up on standing is walked up on standing
    Some(StalkTell { gd, at, reach: 0.0, rise: 0.0, low })
}

/// The wait at a corner on his body: where his hands point, how much of the grab's reach, how far he is sunk. (stalk.ts stalkWait)
pub fn stalk_wait(g: &mut crate::game::Game) -> Option<crate::stalk::StalkWait> {
    let me = g.player;
    let row = *g.m.stalk.state.get(me)?;
    let n = g.pl[me].wall.as_ref().map(|w| w.n);
    let n = match n {
        Some(n) if row.wait == Some(true) => n,
        _ => return None,
    };
    if g.time - row.t > 0.1 || !corner_gate(g) {
        return None;
    }
    let p = g.ch[g.pl[me].ch].pos;
    let mut best: Option<GuardId> = None;
    let mut bd = g.k.stalk.corner_see;
    let mut i = 0;
    while i < g.guards.len() {
        let gd = g.guards[i];
        i += 1;
        if g.gd[gd].civ.is_some() || !crate::close::close_subject(g, gd) || g.gd[gd].state == crate::game::GuardState::Alert {
            continue;
        }
        let pos = g.ch[g.gd[gd].ch].pos;
        let d = m::hyp2(p[0] - pos[0], p[2] - pos[2]);
        if d < bd && crate::close::clear_between(g, &pos, &p) {
            best = Some(gd);
            bd = d;
        }
    }
    let r = g.k.stalk.corner_reach;
    let at: V3 = match best {
        Some(b) => {
            let c = g.gd[b].ch;
            match g.ch[c].bones.get(crate::character::Bone::Neck).or(g.ch[c].bones.get(crate::character::Bone::Head)).copied() {
                Some(neck) => [neck[0], neck[1], neck[2]],
                None => [g.ch[c].pos[0], 1.45, g.ch[c].pos[2]],
            }
        }
        None => [p[0] + n[0] * 0.8, 1.35, p[2] + n[2] * 0.8],
    };
    let near = if best.is_some() { v3::clamp((g.k.stalk.corner_see - bd) / m::max(0.05, g.k.stalk.corner_see - g.k.stalk.corner_m), 0.0, 1.0) } else { 0.0 };
    Some(StalkWait { at, reach: r[0] + (r[1] - r[0]) * near, coil: g.k.stalk.corner_coil })
}

/// A man is being stalked, or the stalk has just become a grab: this frame's left click is no blow. (stalk.ts stalkHas)
pub fn stalk_has(g: &mut crate::game::Game) -> bool {
    match g.m.stalk.state.get(g.player) {
        Some(s) => s.t == g.time && (s.on.is_some() || s.grab.is_some()),
        None => false,
    }
}

/// The man the stalk reached this frame: the grab to take now. (stalk.ts stalkGrab)
pub fn stalk_grab(g: &mut crate::game::Game) -> Option<GuardId> {
    match g.m.stalk.state.get(g.player) {
        Some(s) if s.t == g.time => s.grab,
        _ => None,
    }
}

/// The row's grab on this man is about to be taken: is it the stalk's? A question only. (stalk.ts stalkKeyCan)
pub fn stalk_key_can(g: &mut crate::game::Game, gd: GuardId) -> bool {
    let me = g.player;
    if g.gd[gd].civ.is_some() || !hands(g) || (g.pl[me].crouch && !crate::player::can_stand(g)) {
        return false;
    }
    let p = g.ch[g.pl[me].ch].pos;
    out_of(g, gd, &p, false).is_none()
}

/// The grab took him: the stalk on him, if any, is over, counted as a grab. (stalk.ts stalkKeyTook)
pub fn stalk_key_took(g: &mut crate::game::Game) {
    let r = state_of(g);
    r.on = None;
    r.armed = Some(false);
    g.stats.stalk.grabs += 1;
    g.stats.stalk.last = String::from("grab");
}

/// The man being stalked right now. (stalk.ts stalkOn)
pub fn stalk_on(g: &mut crate::game::Game) -> Option<GuardId> {
    g.m.stalk.state.get(g.player).and_then(|s| s.on)
}

/// A reset of the floor's men, a load: nothing is stalked. (stalk.ts stalkReset)
pub fn stalk_reset(g: &mut crate::game::Game) {
    let me = g.player;
    if let Some(r) = g.m.stalk.state.get_mut(me) {
        *r = fresh();
    }
}

/// The step in's ease, 0‥1 → 0‥1: a quadratic off the mark, then a cubic ease-out joined with one slope. (stalk.ts stepEase)
pub fn step_ease(u: f64) -> f64 {
    let v = v3::clamp(u, 0.0, 1.0);
    let pp = 0.15;
    let b = 1.0 / ((1.0 - pp) * (1.0 - pp) * (1.0 + 0.5 * pp));
    let a = 3.0 * b * (1.0 - pp) * (1.0 - pp) / (2.0 * pp);
    let w = 1.0 - v;
    if v < pp { a * v * v } else { 1.0 - b * (w * w * w) }
}

#[cfg(test)]
mod tests {
    /// The TypeScript's own bits (stalk.ts stepEase under bun, EXACT on): input, output.
    #[test]
    fn step_ease_matches_the_typescript() {
        const BITS: [(u64, u64); 13] = [
            (0xbff0000000000000, 0x0000000000000000),
            (0x0000000000000000, 0x0000000000000000),
            (0x3fa999999999999a, 0x3f97d05f417d05f5),
            (0x3fb999999999999a, 0x3fb7d05f417d05f5),
            (0x3fc3126e978d4fdf, 0x3fca6f4720a4e828),
            (0x3fc3333333333333, 0x3fcaca6b29aca6b0),
            (0x3fc999999999999a, 0x3fd5cf81c2e64528),
            (0x3fd5555555555555, 0x3fe3cadb78fcb402),
            (0x3fe0000000000000, 0x3fead994970a9bf2),
            (0x3fe8000000000000, 0x3fef5b3292e1537e),
            (0x3feccccccccccccd, 0x3feff573e070b991),
            (0x3ff0000000000000, 0x3ff0000000000000),
            (0x4000000000000000, 0x3ff0000000000000),
        ];
        for (u, want) in BITS {
            assert_eq!(super::step_ease(f64::from_bits(u)).to_bits(), want, "u bits {u:x}");
        }
    }
}
