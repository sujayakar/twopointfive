//! THE AIM CHANNELS: a turn of the picture's men never snaps. A row's aim is a list of legs eased on a channel (a teammate's, made as his row begins; Sam's is the stage's own), and Sam's two yaws are stepped after each
//! tick toward what the game wrote. A teammate's yaws have the game's own turn weight, at the picture's numbers. And idle life: a man in no row lets his gun wander.
use super::light::{KIT_VIEW_LOOP as K, KIT_VIEW_QUIET as QK};
use super::types::*;
use super::walk::{home_of, flat};
use crate::game::Game;
use crate::ids::*;
use crate::m;

pub(super) const R: f64 = m::PI / 180.0;

pub(super) fn wrap(a: f64) -> f64 { m::atan2(m::sin(a), m::cos(a)) }
pub(super) fn off(a: f64, b: f64) -> f64 { m::abs(wrap(a - b)) }
pub(super) fn clamp(x: f64, lo: f64, hi: f64) -> f64 { m::max(lo, m::min(hi, x)) }
pub(super) fn chan(y: f64) -> Chan { Chan { v: y, want: y, from: y, to: y, ..Chan::default() } }
fn ease_t(a: f64) -> f64 { if a <= 45.0 * R + 1e-6 { K.ease[0] } else { K.ease[1] } }
/// the channel to `to`, eased from where and how fast it is, never faster than `cap` (rad/s): at least `min_t` seconds (under 0: the calls' own for the angle)
pub(super) fn own(now: f64, c: &mut Chan, to: f64, cap: f64, min_t: f64) {
    if c.own && off(to, c.to) < 0.2 * R { return; }
    let d = wrap(to - c.v);
    let a = m::abs(d);
    c.own = true; c.to = to; c.from = c.v; c.d = d; c.v0 = c.w; c.t0 = now;
    c.t = m::max(m::max(m::max(0.05, if min_t < 0.0 { ease_t(a) } else { min_t }), 1.875 * a / cap), 2.0 * K.ramp * m::abs(c.w) / cap);
}
pub(super) fn arrived(now: f64, c: &Chan) -> bool { c.own && now >= c.t0 + c.t }
pub(super) fn chan_tick(now: f64, c: &mut Chan, cap: f64, dt: f64) {
    if c.own {
        let u = (now - c.t0) / c.t;
        if u >= 1.0 { c.from = wrap(c.from + c.d); c.v = c.from; c.d = 0.0; c.w = 0.0; c.v0 = 0.0; return; }
        let (u2, u3, u4, u5) = (u * u, u * u * u, u * u * u * u, u * u * u * u * u);
        c.v = c.from + c.d * (10.0 * u3 - 15.0 * u4 + 6.0 * u5) + c.v0 * c.t * (u - 6.0 * u3 + 8.0 * u4 - 3.0 * u5);
        c.w = c.d / c.t * (30.0 * u2 - 60.0 * u3 + 30.0 * u4) + c.v0 * (1.0 - 18.0 * u2 + 32.0 * u3 - 15.0 * u4);
        return;
    }
    if !(dt > 0.0) { return; }
    // (a trapezoid: up to the rate, and down in time to stop on it)
    let dd = wrap(c.want - c.v);
    let acc = cap / K.ramp;
    let vt = m::sign(dd) * m::min(cap, m::sqrt(2.0 * acc * m::abs(dd)));
    c.w += clamp(vt - c.w, -acc * dt, acc * dt);
    let st = c.w * dt;
    if m::abs(dd) < 1e-4 { c.v = wrap(c.v + dd); c.w = 0.0; }
    else if st * dd > 0.0 && m::abs(st) >= m::abs(dd) { c.v = wrap(c.v + dd); c.w = dd / dt; }
    else { c.v = wrap(c.v + st); }
}
/// the channel a row's aim runs on
pub(super) fn ch_of<'a>(st: &'a mut KitStage, b: &'a mut Beat) -> &'a mut Chan { if b.man.is_some() { &mut b.ch } else { &mut st.a.sam_ch } }
/// the legs, one after another. True when the last has been held.
pub(super) fn aim_legs(g: &Game, st: &mut KitStage, b: &mut Beat) -> bool {
    let Some(l) = b.legs.get(b.leg).copied() else { return true };
    let now = g.time;
    own(now, ch_of(st, b), l.yaw, l.cap, l.min_t);
    if !arrived(now, ch_of(st, b)) { return false; }
    if b.leg_t.is_nan() { b.leg_t = now; }
    if now - b.leg_t < l.hold { return false; }
    b.leg += 1;
    b.leg_t = f64::NAN;
    b.leg >= b.legs.len()
}
/// his rest direction where he stands: a teammate's place's yaw, Sam's the foot of the door
pub(super) fn rest_yaw(g: &Game, st: &KitStage, b: &Beat) -> f64 {
    match b.man { Some(x) => home_of(g, st, x).1, None => crate::v3::yaw_to(g.ch[b.ch_id].pos, st.foot) }
}
/// no quiet row asks more than `max_off` off his rest direction
pub(super) fn lim(rest: f64, y: f64) -> f64 { rest + clamp(wrap(y - rest), -K.max_off * R, K.max_off * R) }
/// a quiet row's tick: the legs, then the leg back, and its end
pub(super) fn quiet_tick(g: &Game, st: &mut KitStage, b: &mut Beat) -> Step {
    if b.phase != "down" {
        if !aim_legs(g, st, b) { return Step::On; }
        if b.turn { b.marks.push("turn"); } else if b.lit { b.marks.push("lit"); }
        b.phase = "down"; b.pt = g.time;
        b.lit = false;
        b.h_to = if b.man.is_some() { 1.1 } else { 0.0 };
        let yaw = rest_yaw(g, st, b);
        b.legs = alloc::vec![Leg { yaw, cap: (if b.turn { K.body_rate } else { K.gun_rate }) * R, min_t: -1.0, hold: 0.0 }];
        b.leg = 0;
        b.leg_t = f64::NAN;
        return Step::On;
    }
    if aim_legs(g, st, b) { Step::Done } else { Step::On }
}
/// what a row's tick says
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Step { On, Done }

/// the aim put out: the actor's aim point on his channel's yaw (a teammate's watch, Sam's scripted aim), Sam's channel kept on his gun between his rows
pub(super) fn aims(g: &mut Game, st: &mut KitStage) {
    let now = g.time;
    let dt = st.a.step_dt;
    let mine = st.a.beat.as_ref().map_or(false, |b| b.man.is_none() && (b.quiet || b.spot.is_some()));
    if !mine {
        let G = st.a.aim;
        st.a.sam_ch.v = G.v; st.a.sam_ch.want = G.v; st.a.sam_ch.w = G.w; st.a.sam_ch.own = false;
        st.a.sam_h += (0.0 - st.a.sam_h) * m::min(1.0, dt * 8.0);
    }
    st.a.beam = None;
    let Some(mut b) = st.a.beat.take() else { return };
    let p = g.ch[b.ch_id].pos;
    if b.man.is_none() && !b.quiet {
        if let Some(sp) = b.spot {
            let from = match b.post { Some(q) => [q[0], 0.0, q[1]], None => p };
            own(now, &mut st.a.sam_ch, crate::v3::yaw_to(from, sp), K.gun_rate * R, -1.0);
        }
    }
    let cap = (if b.turn { K.body_rate } else { K.gun_rate }) * R;
    chan_tick(now, ch_of(st, &mut b), cap, dt);
    if b.quiet || (b.man.is_none() && b.spot.is_some()) {
        let y = ch_of(st, &mut b).v;
        let d = if b.quiet { 3.0 } else { m::max(1.0, flat(b.spot.unwrap(), [p[0], p[2]])) };
        match b.man {
            Some(x) => {
                let hy = home_of(g, st, x).1;
                let o = wrap(y - hy);
                let a = m::abs(o);
                // (under `torso` off his rest: the chest alone, his feet still; a quarter turn: all of him)
                let face = if b.turn { y } else if a <= K.torso * R { hy } else { hy + m::sign(o) * (a - K.torso * R) };
                let at: Option<V3> = if b.turn { None } else { Some([p[0] + m::sin(y) * d, b.h_to, p[2] + m::cos(y) * d]) };
                if let Some(w) = st.a.walks.iter_mut().find(|w| w.man == x) { w.watch = at; w.face = Some(face); }
                if let Some(a) = crate::orders::squad_orders_of(&crate::seats::here(g), x) {
                    let r = &mut g.tab.orders_assignment[a];
                    if r.order == "cover" {
                        r.watch = at; r.face = Some(face);
                        r.upper = if at.is_some() { crate::character::UpperMode::Aim } else { crate::character::UpperMode::None };
                        r.stance = if at.is_some() { crate::character::Stance::None } else { crate::character::Stance::LowReady };
                    }
                }
            }
            None => {
                if let Some(mut led) = st.a.led {
                    st.a.sam_h += (b.h_to - st.a.sam_h) * m::min(1.0, dt * 6.0);
                    let h = if b.quiet { st.a.sam_h } else { 1.0 };
                    // (what is thrown goes where his hand is: on its spot, once the can is in it)
                    let spot = b.spot.filter(|_| b.phase == "throw" && matches!(b.row, Act::ThrowSmoke | Act::ThrowFlash | Act::ThrowFrag));
                    led.at = Some(spot.unwrap_or([p[0] + m::sin(y) * d, h, p[2] + m::cos(y) * d]));
                    st.a.led = Some(led);
                }
            }
        }
    }
    st.a.beat = Some(b);
}

/// Sam's two yaws: each channel's step toward what the game last wrote, by the time that step took. Written back on him.
pub(super) fn yaws(g: &mut Game, st: &mut KitStage, dt: f64) {
    let now = g.time;
    let c = g.pl[g.player].ch;
    st.a.body.want = g.ch[c].body_yaw;
    st.a.aim.want = g.ch[c].aim_yaw;
    if !st.a.sam_at.is_nan() { st.a.aim.want = st.a.sam_at; }
    // (under a loud row's cost ALL of him comes round to the door, at the cost's own rate)
    let t = if !st.a.cost_t.is_nan() { QK.cost.turn } else { 0.0 };
    if t > 0.0 { st.a.body.want = crate::v3::yaw_to(g.ch[c].pos, st.foot); }
    chan_tick(now, &mut st.a.body, m::max(K.body_rate, t) * R, dt);
    chan_tick(now, &mut st.a.aim, m::max(K.gun_rate, t) * R, dt);
    g.ch[c].body_yaw = st.a.body.v;
    g.ch[c].aim_yaw = st.a.aim.v;
}

fn scan(g: &Game, st: &mut KitStage, key: Option<GuardId>, i: usize, ok: bool, v0: f64) -> f64 {
    let now = g.time;
    let dt = st.a.step_dt;
    let hold = st.veil_asked.is_some() || st.a.life_still;
    let at = match st.a.scans.iter().position(|s| s.key == key) {
        Some(k) => k,
        None => { st.a.scans.push(Scan { key, k: i * 3, from: v0, to: v0, t0: -1e9, next: now + 0.6 + 0.9 * i as f64, v: v0, on: false }); st.a.scans.len() - 1 }
    };
    let s = &mut st.a.scans[at];
    if !ok { s.on = false; s.from = 0.0; s.to = 0.0; s.v = 0.0; s.next = now + 0.5 + 0.4 * i as f64; return 0.0; }
    // (under a dissolve, and through the quiet picture's idle beat, where every man freezes: as he stands)
    if hold { s.t0 += dt; s.next += dt; return s.v; }
    s.on = true;
    if now >= s.next {
        s.from = s.v;
        s.to = K.scan_deg[s.k % K.scan_deg.len()] * R;
        s.t0 = now;
        s.next = now + K.scan_gap[(s.k + i) % K.scan_gap.len()];
        s.k += 1;
    }
    let u = clamp((now - s.t0) / K.scan_ease, 0.0, 1.0);
    let u3 = u * u * u;
    s.v = s.from + (s.to - s.from) * (10.0 * u3 - 15.0 * u3 * u + 6.0 * u3 * u * u);
    s.v
}
/// IDLE LIFE: a man who is in no row, on his place, lets his gun wander a few degrees either side of his rest aim: a fixed list of angles and waits, no dice, the game's clock. Nobody's feet move.
/// Sam at rest is not aimed through his cursor: his rest aim is written to his aim's channel after the step.
pub(super) fn idle(g: &mut Game, st: &mut KitStage, cmd: &crate::cmd::PlayerCmd) {
    let free = st.a.led.is_none() && !st.none;
    let now = g.time;
    let (b_man, b_team, b_moved, b_sam) = match st.a.beat.as_ref() {
        Some(b) => (b.man, super::acts::row(b.row).team, b.moved.clone(), b.man.is_none()),
        None => (None, false, alloc::vec::Vec::new(), false),
    };
    let busy = st.a.beat.is_some();
    for i in 0..st.order.len() {
        let x = st.order[i];
        let c = g.gd[x].ch;
        let (hp, hy) = home_of(g, st, x);
        let p = g.ch[c].pos;
        let was = st.a.scans.iter().find(|s| s.key == Some(x)).map_or(false, |s| s.on);
        let o = wrap(g.ch[c].aim_yaw - hy);
        let a = crate::orders::squad_orders_of(&crate::seats::here(g), x);
        let walked = st.a.walks.iter().find(|w| w.man == x).map_or(false, |w| !w.lane && w.k + 1 >= w.pts.len() && now >= w.at);
        let cover = a.map_or(false, |a| g.tab.orders_assignment[a].order == "cover");
        let ok = free && !(busy && (b_man == Some(x) || b_team || b_moved.contains(&x))) && walked && cover && flat(p, hp) <= K.on_slot && !st.a.swaps.iter().any(|w| w.man == x);
        let v = scan(g, st, Some(x), i + 1, ok, if m::abs(o) <= 8.0 * R { o } else { 0.0 });
        if ok {
            g.tab.orders_assignment[a.unwrap()].watch = Some([p[0] + m::sin(hy + v) * 3.0, K.scan_y, p[2] + m::cos(hy + v) * 3.0]);
        } else if was && cover && b_man != Some(x) {
            g.tab.orders_assignment[a.unwrap()].watch = None;
        }
    }
    let c = g.pl[g.player].ch;
    let p = g.ch[c].pos;
    let rest = !(busy && b_sam && !b_team);
    let ok = free && !(busy && b_sam) && flat(p, st.sam) <= K.on_slot;
    let v = scan(g, st, None, 0, ok, 0.0);
    let y = crate::v3::yaw_to(p, st.foot) + v;
    let d = flat(st.foot, [p[0], p[2]]);
    st.a.sam_at = if rest { y } else { f64::NAN };
    if !rest { return; }
    // (the cursor is still put on that line, at the first of three places along it whose ray ends on something that stands: so that the game's own "a friend is under the cursor" has no body to find)
    let id = g.ch[c].id;
    let n = g.engine.world.statics.len() as f64;
    let mut chose = None;
    for k in [1.0, 0.6, 0.3] {
        let at: V3 = [p[0] + m::sin(y) * d * k, 0.0, p[2] + m::cos(y) * d * k];
        chose = Some(at);
        let Some((mx, my)) = super::stage::cursor_of(cmd, at) else { continue };
        let Some(cam) = g.host_cam else { break };
        let r = crate::camera::follow_camera_ray(g, cam, mx / m::max(1.0, cmd.w), my / m::max(1.0, cmd.h));
        match crate::scene_boxes::box_world_raycast(g, r.ro, r.rd, Some(200.0), None, Some(id), None) { None => break, Some(h) if h.index < n => break, _ => {} }
    }
    st.a.cursor = chose;
}
