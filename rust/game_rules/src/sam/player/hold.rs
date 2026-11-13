use super::*;                      // your own module: its types, its other parts' functions
use crate::character::{HoldPhase, HoldVariant};
use crate::game::Game;
use crate::ids::*;
use crate::js;
use crate::m;
use crate::v3::{self, V3};
use alloc::{string::String, vec::Vec};

/// the hold of this seat: re-read at every use, never held across a call that takes `g`
macro_rules! hold {
    ($g:ident, $p:expr) => {
        $g.pl[$p].holding_mut().unwrap()
    };
}

/// `g.events.push({ kind: 'prop', pos, time, loud: false, level, who: <Sam's body> })`, and `pl.noise` raised to it: what the room hears of a hold's beat
pub(super) fn prop_noise(g: &mut Game, pos: V3, level: f64) {
    let p = g.player;
    let c = g.pl[p].ch;
    let (time, who, serial) = (g.time, g.ch[c].id, g.serial());
    g.events.push(crate::game::GameEvent {
        kind: crate::director::EventKind::Prop,
        pos,
        time,
        loud: false,
        level: Some(level),
        who: Some(who),
        serial,
        ..crate::game::GameEvent::default()
    });
    g.pl[p].noise = m::max(g.pl[p].noise, level);
}

/// Off the hold: `gd.held`, the animator's pose, the dialogue's clock. (chokeOut, killOut, whipOut, handBack's first line)
fn unhook(g: &mut Game, gd: GuardId) {
    let hc = g.gd[gd].ch;
    let talk = g.gd[gd].talk;
    g.gd[gd].held = None;
    g.ch[hc].anim.held_pose = None;
    g.tab.game_talk[talk].held_since = -1.0;
}

/// Let go at once: the man is handed straight back, Sam's side cleared.
pub fn free_held(g: &mut crate::game::Game, why: crate::player::FreeHeldWhy) {
    let pl = g.player;
    let c = g.pl[pl].ch;
    let Some(h) = g.pl[pl].holding() else { return };
    let h_g = h.g;
    let shot = why == crate::player::FreeHeldWhy::Shot;
    hand_back(g, h_g, if shot { 0.6 } else { 0.3 });
    g.pl[pl].holding_over();
    g.ch[c].anim.hold_pose = None;
    if shot {
        let hc = g.gd[h_g].ch;
        let at = v3::add(g.ch[hc].pos, [0.0, 1.3, 0.0]);
        crate::audio_iface::game_audio_like_play(g, crate::audio_iface::Sfx::Shove, Some(at), Some(0.3), None);
    }
}

/// The hold simply ceases: Sam's side cleared, and his if he is still about.
pub(super) fn drop_hold(g: &mut crate::game::Game) {
    let pl = g.player;
    let c = g.pl[pl].ch;
    let Some(gd) = g.pl[pl].holding().map(|h| h.g) else { return };
    if g.gd[gd].held.as_ref().map_or(false, |h| h.by == crate::game::GuardHeldBy::Player) {
        g.gd[gd].held = None;
    }
    let gc = g.gd[gd].ch;
    g.ch[gc].anim.held_pose = None;
    if !crate::condition::down(g, g.gd[gd].ch) {
        let talk = g.gd[gd].talk;
        g.tab.game_talk[talk].held_since = -1.0;
    }
    g.pl[pl].holding_over();
    g.ch[c].anim.hold_pose = None;
}

/// One tick with a man in his arm: the beat's clock and what is asked of it, his rows, the gun, then the pair's movement.
pub(super) fn update_hold(g: &mut Game, t: Tick) {
    let Tick { pl, c, dt, .. } = t;
    let gd = hold!(g, pl).g;
    if at_gunpoint(g, pl) && t.hit(g, crate::input::Key::L) { gun_light_key(g, pl, t.gun); }
    nv_key(g, Some(t.input), dt);
    let his = g.gd[gd].held.as_ref().is_some_and(|h| h.by == crate::game::GuardHeldBy::Player);
    if !his || !crate::game::game_on_floor(g, gd) || crate::condition::down(g, g.gd[gd].ch) {
        drop_hold(g);
        crate::character::character_update(g, c, dt);
        return;
    }
    let h = hold!(g, pl);
    h.t += dt;
    h.swap_t += dt;
    g.arms[c].fire_cd -= dt;
    let (speed, own) = if hold!(g, pl).phase == HoldPhase::Grab {
        (0.0, Some(grab_tick(g, t, gd)))
    } else {
        match held_tick(g, t, gd) { Some(speed) => (speed, None), None => { hold_tail(g, dt, 0.0, None); return; } }
    };
    g.pl[pl].sprinting = false;
    g.pl[pl].crouch = false;
    hold_tail(g, dt, speed, own);
}
fn at_gunpoint(g: &Game, pl: PlayerId) -> bool { g.pl[pl].holding().is_some_and(|h| h.variant == HoldVariant::Gun) }

/// The grab: in to HOLD.dist behind him on HIS facing, squaring up to his back, through Sam's own collision. Returns the pace his feet show.
fn grab_tick(g: &mut Game, t: Tick, gd: GuardId) -> f64 {
    let Tick { pl, c, dt, .. } = t;
    let hc = g.gd[gd].ch;
    let yaw = g.ch[hc].body_yaw;
    let dist = g.k.hold.dist;
    let want: V3 = [g.ch[hc].pos[0] - m::sin(yaw) * dist, 0.0, g.ch[hc].pos[2] - m::cos(yaw) * dist];
    g.ch[c].vel = [0.0, 0.0, 0.0];
    let (own, u) = match hold!(g, pl).from {
        Some(from) if g.k.stalk.catch_secs > 0.0 => catch_step(g, t, gd, from, want),
        _ => {
            let to = v3::lerp3(g.ch[c].pos, want, 1.0 - m::exp(-14.0 * dt));
            crate::character::pos_replace(g, c, to);
            (-1.0, -1.0)
        }
    };
    collide_player(g, 1.7);
    // (the sneak's step in: he comes round to the man's back THROUGH the step)
    let ch = &mut g.ch[c];
    ch.body_yaw = match hold!(g, pl).yaw0 {
        Some(yaw0) if u >= 0.0 && u < 1.0 => { let e = v3::clamp(u / 0.85, 0.0, 1.0); yaw0 + v3::wrap_angle(yaw - yaw0) * e * e * (3.0 - 2.0 * e) }
        _ => v3::approach_angle(ch.body_yaw, yaw, 14.0 * dt),
    };
    ch.aim_yaw = v3::approach_angle(ch.aim_yaw, ch.body_yaw, 12.0 * dt);
    ch.aim_pitch = v3::damp(ch.aim_pitch, 0.0, 8.0, dt);
    g.gd[gd].speed = 0.0;
    g.ch[hc].vel = [0.0, 0.0, 0.0];
    let rmb = t.rmb(g);
    let h = hold!(g, pl);
    if !rmb && h.rmb_choke.is_some_and(js::truthy) { h.rmb_choke = None; }   // (the button let go while the grab was still landing: a plain hold)
    if h.t >= g.k.hold.secs.grab { grabbed(g, pl, gd); }
    own
}
/// He has him.
fn grabbed(g: &mut Game, pl: PlayerId, gd: GuardId) {
    let (c, hc, time) = (g.pl[pl].ch, g.gd[gd].ch, g.time);
    set_phase(g, HoldPhase::Held);
    hold!(g, pl).since = time;
    if let Some(h) = g.gd[gd].held.as_mut() { h.since = time; }
    let talk = g.gd[gd].talk;
    g.tab.game_talk[talk].held_since = time;
    let yaw = g.ch[hc].body_yaw;
    g.ch[c].aim_yaw = yaw;
    g.ch[c].body_yaw = yaw;
    let at = v3::add(g.ch[hc].pos, [0.0, 1.5, 0.0]);
    crate::audio_iface::game_audio_like_play(g, crate::audio_iface::Sfx::HeldProtest, Some(at), Some(0.18), None);
}

/// The sneak's catch: the step in, then the yank back. Returns the pace his feet show and how far through the step he is.
fn catch_step(g: &mut Game, t: Tick, gd: GuardId, from: V3, want: V3) -> (f64, f64) {
    let Tick { pl, c, dt, .. } = t;
    if hold!(g, pl).step_m.is_none() {
        let yaw0 = g.ch[c].body_yaw;
        let h = hold!(g, pl);
        h.step_m = Some(v3::dist_xz(want, from));
        h.yaw0 = Some(yaw0);
    }
    let k = &g.k.stalk;
    let (catch_stride, stretch) = (k.catch_stride, v3::clamp(hold!(g, pl).step_m.unwrap() / m::max(0.1, k.catch_m), 1.0, k.catch_long));
    let secs = m::min(g.k.stalk.catch_secs * stretch, g.k.hold.secs.grab - 0.08);
    let u = v3::clamp(hold!(g, pl).t / secs, 0.0, 1.0);
    let was = g.ch[c].pos;
    let to = if u >= 1.0 { yank(g, pl, gd, want, secs) } else { want };
    // (by the step's own delta from where the collision last left him, never the straight line's point)
    let (e, e0) = (crate::stalk::step_ease(u), hold!(g, pl).step_e.unwrap_or(0.0));
    hold!(g, pl).step_e = Some(e);
    let d = v3::sub(v3::lerp3(from, to, e), v3::lerp3(from, to, e0));
    let at = g.ch[c].pos;
    let next = if u >= 1.0 { v3::lerp3(at, to, 1.0 - m::exp(-14.0 * dt)) } else { [at[0] + d[0], at[1], at[2] + d[2]] };
    crate::character::pos_replace(g, c, next);
    let own = if u >= 1.0 { 0.0 } else { m::min(catch_stride, v3::dist_xz(next, was) / m::max(dt, 1e-4)) };
    (own, u)
}
/// The step has landed: he pulls the man back off his feet as far as there is room, and goes back WITH him. Returns where he is to stand.
fn yank(g: &mut Game, pl: PlayerId, gd: GuardId, want: V3, after: f64) -> V3 {
    let (c, hc, time) = (g.pl[pl].ch, g.gd[gd].ch, g.time);
    let back = { let yaw = g.ch[hc].body_yaw; [-m::sin(yaw), 0.0, -m::cos(yaw)] };
    let pulls = g.k.stalk.yank > 0.0 && g.gd[gd].civ.is_none();
    if hold!(g, pl).hit_at.is_none() {
        hold!(g, pl).hit_at = Some(time);
        if pulls {
            let (far, room) = (g.k.stalk.yank, g.k.stalk.yank_room);
            let hit = crate::scene_collision::raycast(g, [want[0], 1.0, want[2]], back, far + room);
            let h = hold!(g, pl);
            h.yank_m = Some(hit.map_or(far, |hit| m::max(0.0, hit.t - room)));
            h.yanked = Some(0.0);
        }
    }
    if !pulls { return want; }
    let grab = g.k.hold.secs.grab;
    let h = hold!(g, pl);
    let v = v3::clamp((h.t - after) / m::max(0.05, grab - after), 0.0, 1.0);
    let y = h.yank_m.unwrap_or(m::NAN) * (1.0 - (1.0 - v) * (1.0 - v));
    let d = y - h.yanked.unwrap_or(m::NAN);
    if !(d > 0.0) { return want; }
    h.yanked = Some(y);
    for id in [hc, c] {
        let p = g.ch[id].pos;
        crate::character::pos_replace(g, id, [p[0] + back[0] * d, p[1], p[2] + back[2] * d]);
    }
    [want[0] + back[0] * d, 0.0, want[2] + back[2] * d]
}

/// With him held: what is asked of him and done to him, the gun, and where the pair goes. Returns their pace; None: the hold ended.
fn held_tick(g: &mut Game, t: Tick, gd: GuardId) -> Option<f64> {
    let Tick { pl, .. } = t;
    let k = g.k.hold.secs;
    if hands_swap(g, t) { swap_hold(g); }
    let gun = at_gunpoint(g, pl);
    let swapping = hold!(g, pl).swap_t < k.swap;
    if !beats(g, t, gd, gun, swapping) { return None; }
    choke_button(g, t, gd, gun, swapping);
    held_speech(g, pl);
    if beat_done(g, t, gd) { return None; }
    let gun_held = gun && hold!(g, pl).phase == HoldPhase::Held;
    gun_out(g, t, gun_held, swapping);
    // (what the rest of the tick reads of the hold is read BEFORE the round: a round can end the hold)
    let h = hold!(g, pl);
    let (out, hit_at, free, fires) = (h.gun_out, h.hit_at, h.phase == HoldPhase::Held, gun_held && h.fire_queued && h.gun_out > 0.8);
    if fires { held_round(g, t, gd); }
    pair_turns(g, t, gd, gun, out, hit_at);
    Some(pair_walks(g, t, gd, free))
}

/// E, or the key of the other hand's tool: the other hold, once the hand is done with the last.
fn hands_swap(g: &mut Game, t: Tick) -> bool {
    let h = hold!(g, t.pl);
    let (ready, key) = (h.phase == HoldPhase::Held && h.swap_t >= g.k.hold.secs.swap, if h.variant == HoldVariant::Gun { crate::handslot::HANDSLOT_KEY } else { crate::input::Key::Digit1 });
    ready && (t.hit(g, crate::input::Key::E) || t.hit(g, key)) && !crate::game::player_choosing(g, t.pl) && g.sets[g.seat].hand.sel.down_t < 0.0
}

/// His rows, done this tick, and the right button: let go of, questioned, the choke or the whip wound up and begun. False: the hold ended.
fn beats(g: &mut Game, t: Tick, gd: GuardId, gun: bool, swapping: bool) -> bool {
    use crate::game::Verb;
    let pl = t.pl;
    if hold!(g, pl).phase != HoldPhase::Held { return true; }
    let act = g.sets[g.seat].act.as_ref().filter(|a| a.kind == crate::game::InteractableKind::Held).and_then(|a| a.verb);
    let civ = g.gd[gd].civ.is_some();
    let rmb = t.rmb_hit(g) && !gun && !swapping && hold!(g, pl).wind_t < 0.0 && !civ;
    if act == Some(Verb::Comply) && civ && !swapping && crate::civilians::civilian_comply(g) { return false; }
    if act == Some(Verb::Question) && !swapping && !gun { ask_held(g); }
    let h = hold!(g, pl);
    if rmb { h.rmb_choke = Some(1.0); h.rmb_t = Some(0.0); }
    if swapping { return true; }
    if act == Some(if gun { Verb::Whip } else { Verb::Choke }) { h.wind_t = 0.0; }
    if h.wind_t < 0.0 { return true; }
    h.wind_t += m::min(t.dt, 0.05);
    if h.wind_t >= g.k.hold.secs.choke_hold {
        h.wind_t = -1.0;
        if gun { start_whip(g); } else { start_choke(g); }
    }
    true
}

/// The right button kept down is the choke for as long as it is down, begun the tick the arm can; a tap of it is the whole choke.
fn choke_button(g: &mut Game, t: Tick, gd: GuardId, gun: bool, swapping: bool) {
    let pl = t.pl;
    let down = t.rmb(g);
    let h = hold!(g, pl);
    if !h.rmb_choke.is_some_and(js::truthy) { return; }
    let down_for = h.rmb_t.unwrap_or(0.0) + t.dt;
    h.rmb_t = Some(down_for);
    if h.rmb_choke == Some(1.0) {
        if h.phase == HoldPhase::Held && !gun && !swapping && g.gd[gd].civ.is_none() {
            h.wind_t = -1.0;
            start_choke(g);
            let h = hold!(g, pl);
            h.rmb_choke = if h.phase == HoldPhase::Choke { Some(2.0) } else { None };
        } else if !down {
            h.rmb_choke = None;
        }
    } else if h.phase != HoldPhase::Choke {
        h.rmb_choke = None;
    }
    if !down && hold!(g, pl).rmb_choke == Some(2.0) {
        if down_for > g.k.close.rmb_tap_secs + 1e-6 { ease_off_hold(g); }
        hold!(g, pl).rmb_choke = None;
    }
}

/// Sam's line has hung its beat: the man answers it. A remark that waited for the man to finish is said once his answer is down and no question is going up.
fn held_speech(g: &mut Game, pl: PlayerId) {
    let time = g.time;
    let h = hold!(g, pl);
    if h.phase != HoldPhase::Held { return; }
    if h.ask.is_some_and(|a| time + 1e-6 >= a.t + a.dur) { h.ask = None; answer_held(g); }
    let h = hold!(g, pl);
    if h.after.is_none() || h.phase != HoldPhase::Held || h.ask.is_some() { return; }
    let h = h.clone();
    if !held_talk(g, &crate::seats::here(g), &h).speaking { say_after(g); }
}

/// The choke or the whip under way: heard, and once it has run its time the man is out. True: he is.
fn beat_done(g: &mut Game, t: Tick, gd: GuardId) -> bool {
    let k = g.k.hold.secs;
    let h = hold!(g, t.pl);
    let (phase, at) = (h.phase, h.t);
    if phase == HoldPhase::Choke && at < k.choke - 0.3 && m::floor(at / CHOKE_GRAIN) > m::floor((at - t.dt) / CHOKE_GRAIN) {
        let from = v3::add(g.pos_of(gd), [0.0, 1.5, 0.0]);
        crate::audio_iface::game_audio_like_play(g, crate::audio_iface::Sfx::ChokeStrain, Some(from), Some(0.25), None);
    }
    match phase {
        HoldPhase::Choke if at >= k.choke => choke_out(g),
        HoldPhase::Whip if at >= k.whip => whip_out(g),
        _ => return false,
    }
    true
}

/// A click queues one round, and the pistol comes off his head for it; with no round called for it goes back.
fn gun_out(g: &mut Game, t: Tick, held: bool, swapping: bool) {
    let Tick { pl, c, dt, .. } = t;
    let time = g.time;
    if !held { hold!(g, pl).fire_queued = false; }
    if held && !swapping {
        if t.lmb_hit(g) && !hold!(g, pl).fire_queued && g.arms[c].fire_cd <= 0.0 {
            if crate::weapons::firearm_can_fire(g, t.gun) { hold!(g, pl).fire_queued = true; } else { dry_fire(g, c); }
        }
        if t.rmb_hit(g) { ocp_on_reticle(g); }
    }
    let h = hold!(g, pl);
    let out = held && (h.fire_queued || (h.shot_t >= 0.0 && time - h.shot_t < 0.9));
    h.gun_out = v3::damp(h.gun_out, if out { 1.0 } else { 0.0 }, if out { 30.0 } else { 6.0 }, dt);
}
/// The round, never at the man in the arm.
fn held_round(g: &mut Game, t: Tick, gd: GuardId) {
    let Tick { pl, c, .. } = t;
    hold!(g, pl).fire_queued = false;
    if !crate::weapons::firearm_fire(g, t.gun) { return; }
    let time = g.time;
    g.arms[c].fire_cd = HOLD_FIRE_CD;
    g.pl[pl].last_fire_t = time;
    hold!(g, pl).shot_t = time;
    let seat = crate::seats::here(g);
    let at: V3 = match (seat.aim_target, seat.aim_guard) {
        (Some(f), _) => g.tab.fixture[f].pos,
        (None, Some(a)) if a != gd && !crate::condition::down(g, g.gd[a].ch) => man_aim(g, &seat, a, 1.25),
        _ => free_aim(g, &seat, Some(0.6)),
    };
    let spread = HOLD_SPREAD + crate::suppression::sam_spread(g, &seat) + crate::aim::sam_aim(g);
    crate::combat::fire_weapon(g, c, at, Some(&crate::combat::FireOpts { spread: Some(spread), ..Default::default() }));
}

/// The pair comes round toward the cursor, no faster than he can turn a man; after the sneak's catch, once that has settled.
fn pair_turns(g: &mut Game, t: Tick, gd: GuardId, gun: bool, gun_out: f64, hit_at: Option<f64>) {
    let Tick { c, dt, .. } = t;
    let at = g.sets[g.seat].aim_point;
    let (dx, dz) = (at[0] - g.ch[c].pos[0], at[2] - g.ch[c].pos[2]);
    let far = m::hyp2(dx, dz);
    let settled = hit_at.map_or(1.0, |hit| v3::smoothstep(0.3, 0.75, g.time - hit));
    let (pitch, rate) = if gun { (held_pitch(g, c, gd, far, at) * gun_out, 12.0) } else { (-0.05, 8.0) };
    let ch = &mut g.ch[c];
    if far > 0.6 { ch.body_yaw = v3::approach_angle(ch.body_yaw, m::atan2(dx, dz), HOLD_TURN * settled * dt); }
    ch.aim_yaw = ch.body_yaw;
    ch.aim_pitch = v3::damp(ch.aim_pitch, pitch, rate, dt);
}
/// The pistol's pitch onto what the reticle has, past the man in the arm.
fn held_pitch(g: &Game, c: CharId, gd: GuardId, far: f64, at: V3) -> f64 {
    let seat = crate::seats::here(g);
    let gun_y = g.ch[c].pos[1] + 1.5;
    let y = match (seat.aim_target, seat.aim_guard) {
        (Some(f), _) => v3::clamp(g.tab.fixture[f].pos[1], 0.3, 2.95),
        (None, Some(a)) if a != gd => man_aim(g, &seat, a, 1.2)[1],
        _ => if level_on(g) { gun_y } else { at[1] + 0.03 },
    };
    v3::clamp(m::atan2(y - gun_y, m::max(far, 0.5)), -45.0 * v3::DEG, 45.0 * v3::DEG)
}

/// The pair walks, slower backward; rooted while the choke or the whip is on. Returns their pace.
fn pair_walks(g: &mut Game, t: Tick, gd: GuardId, free: bool) -> f64 {
    let Tick { pl, c, dt, .. } = t;
    let hc = g.gd[gd].ch;
    let [mut x, mut z] = if free { keys_held(g, t.input, t.cam) } else { [0.0, 0.0] };
    let l = m::hyp2(x, z);
    if l > 0.0 { x /= l; z /= l; }
    let f = crate::character::character_forward(g, c);
    let back = l > 0.0 && x * f[0] + z * f[2] < -0.3;
    let pace = if l == 0.0 { 0.0 } else { (if back { HOLD_PACE_BACK } else { HOLD_PACE }) * g.pl[pl].pace };
    let most = crate::kitload::sam_load_pace(g, c, pace, None);
    let p = &mut g.pl[pl];
    p.speed_sm = v3::damp(p.speed_sm, most, if most > p.speed_sm { 6.0 } else { 12.0 }, dt);
    let sm = p.speed_sm;
    let was = g.ch[c].pos;
    g.ch[c].vel = if l > 0.0 { [x * sm, 0.0, z * sm] } else { v3::scale(g.ch[c].vel, m::exp(-14.0 * dt)) };
    let to = v3::mad(was, g.ch[c].vel, dt);
    crate::character::pos_replace(g, c, to);
    collide_player(g, 1.7);
    keep_off_guards(g, dt, 1.7, None);
    keep_held_off_guards(g, dt);
    place_held(g);
    let (at, vel) = (g.ch[c].pos, g.ch[c].vel);
    let going = m::hyp2(vel[0], vel[2]);
    let speed = m::min(v3::dist_xz(at, was) / m::max(dt, 1e-4), going);
    let backpedal = speed > 0.2 && (vel[0] * f[0] + vel[2] * f[2]) / m::max(1e-4, going) < -0.3;
    g.pl[pl].backpedal = backpedal;
    g.ch[c].anim.reverse = backpedal;
    g.ch[hc].vel = vel;
    g.gd[gd].speed = speed;
    g.ch[hc].body_yaw = g.ch[c].body_yaw;
    speed
}

/// The end of a hold frame, Sam's side, shared with the frames that end the hold: both animators told the beat, the rail light, Sam's step events.
fn hold_tail(g: &mut crate::game::Game, dt: f64, speed: f64, own: Option<f64>) {
    use crate::character::{Stance, UpperMode};
    let own = own.unwrap_or(-1.0);
    let p = g.player;
    let c = g.pl[p].ch;
    {
        let an = &mut g.ch[c].anim;
        an.speed = if own >= 0.0 { own } else { speed };
        an.crouch_target = 0.0;
        an.upper = UpperMode::None;
        an.stance = Stance::None;
        an.look_yaw_extra = 0.0;
    }
    let has = g.pl[p].holding().is_some();
    let low = if has { 1.0 } else { 0.0 };
    let beats = has;
    let haul = has;
    let h = g.pl[p].holding().cloned();
    let strip = match h.as_ref().and_then(|h| h.strip) {
        Some(s) => Some(crate::disarm::disarm_pose(s)),
        None => None,
    };
    let time = g.time;
    let yank = match h.as_ref().and_then(|h| h.hit_at) {
        Some(at) if time - at < 1.0 => Some(time - at),
        _ => None,
    };
    let Some(h) = h else {
        g.ch[c].anim.hold_pose = None;
        tail_end(g, dt, speed, c);
        return;
    };
    let mut pose = crate::character::HoldPose {
        yank,
        step: if h.from.is_some() && h.phase == crate::character::HoldPhase::Grab { Some(h.t) } else { None },
        man: None,
        pull: None,
        phase: h.phase,
        t: h.t,
        variant: Some(h.variant),
        gun_out: Some(h.gun_out),
        gun_pitch: Some(g.ch[c].aim_pitch),
        low: Some(low),
        beats: Some(beats),
        haul: Some(haul),
        strip,
    };
    let gd = h.g;
    let hc = g.gd[gd].ch;
    // HOLD.catchShape: the arm's keys ride the man
    if h.phase == crate::character::HoldPhase::Grab {
        pose.man = Some(g.ch[hc].pos);
    }
    // STALK.yankWith: his feet step the haul he goes back with
    if let Some(y) = h.yanked {
        pose.pull = Some(y);
    }
    g.ch[c].anim.hold_pose = Some(pose);
    let backpedal = g.pl[p].backpedal;
    {
        let an = &mut g.ch[hc].anim;
        an.speed = speed;
        an.reverse = backpedal;
        an.crouch_target = 0.0;
        an.upper = UpperMode::None;
        an.stance = Stance::None;
        an.look_yaw_extra = 0.0;
        an.deliberate = 0.0;
    }
    let agitation = crate::dialogue::agitation_of(g, gd);
    let bare = Some(true);
    let shot = if h.shot_t >= 0.0 { time - h.shot_t } else { -1.0 };
    let armed = crate::guards::in_hand(g, gd) == Some(crate::guards::InHand::Weapon);
    g.ch[hc].anim.held_pose = Some(crate::character::HeldPose {
        yank,
        // HOLD.catchShape: fed only with it up
        pull: h.yanked,
        phase: h.phase,
        t: h.t,
        agitation,
        variant: Some(h.variant),
        bare,
        shot: Some(shot),
        armed: Some(armed),
        low: Some(low),
        beats: Some(beats),
        haul: Some(haul),
        strip: strip.map(|s| crate::character::StripPose { t: s.t, at: None }),
    });
    // where his torch points: up over Sam's arm at the ceiling, wandering with the struggle
    let torch = crate::guards::hand_carry(g, gd) == crate::guards::InHand::Torch;
    let tt = time - h.since;
    let yaw_to = g.ch[hc].body_yaw + (if torch { 0.07 * m::sin(tt * 0.9) + 0.04 * m::sin(tt * 2.3 + 1.0) } else { 0.0 });
    g.ch[hc].aim_yaw = v3::approach_angle(g.ch[hc].aim_yaw, yaw_to, 4.0 * dt);
    let pitch_to = if torch { 0.55 + 0.25 * m::sin(tt * 1.3 + 2.0) + 0.1 * m::sin(tt * 3.1) } else { 0.0 };
    g.ch[hc].aim_pitch = v3::damp(g.ch[hc].aim_pitch, pitch_to, 6.0, dt);
    tail_end(g, dt, speed, c);
}

/// holdTail's last lines, with a hold or without one: the sling, the body's bake, the rail light, the dragging shuffle.
fn tail_end(g: &mut Game, dt: f64, speed: f64, c: CharId) {
    arm_rifle(g, false, None);
    crate::character::character_update(g, c, dt);
    pose_gun_light(g);
    let phase = g.ch[c].anim.phase;
    if footfall_due(g, dt, phase, speed) != 0.0 { let level = 0.26 * crate::kitload::load_heard(g, c); tread_heard(g, c, level); }
}

/// A new beat of the hold: its clock from nought, nothing chosen, nothing queued, Sam's question dead.
fn set_phase(g: &mut crate::game::Game, phase: crate::character::HoldPhase) {
    let p = g.player;
    let Some(h) = g.pl[p].holding_mut() else { return };
    h.phase = phase;
    h.t = 0.0;
    h.wind_t = -1.0;
    h.fire_queued = false;
    cancel_ask(g);
}

/// Is the choke or the whip under way, or chosen?
pub(in crate::sam) fn hold_works(h: &Hold) -> bool {
    h.wind_t >= 0.0 || matches!(h.phase, crate::character::HoldPhase::Choke | crate::character::HoldPhase::Whip)
}

/// Stop the choke / the whip under way (a Space tap, a step, the back key): the hold goes on and he felt what nearly happened; a choke merely chosen is just un-chosen.
pub(in crate::sam) fn ease_off_hold(g: &mut crate::game::Game) {
    use crate::character::HoldPhase as Ph;
    let p = g.player;
    let Some(h) = g.pl[p].holding_mut() else { return };
    let gd = h.g;
    if h.phase != Ph::Choke && h.phase != Ph::Whip {
        h.wind_t = -1.0;
        return;
    }
    set_phase(g, Ph::Held);
    let (talk, time) = (g.gd[gd].talk, g.time);
    g.tab.game_talk[talk].abort_t = time;
    let hc = g.gd[gd].ch;
    let at = v3::add(g.ch[hc].pos, [0.0, 1.4, 0.0]);
    crate::audio_iface::game_audio_like_play(g, crate::audio_iface::Sfx::PropScrape, Some(at), Some(0.1), Some(&crate::audio_iface::PlayOpts::Rate { rate: 2.2 }));
}

/// Sam's unfinished question dies in his mouth, and a remark waiting its turn with it; an answer the man has begun runs on.
fn cancel_ask(g: &mut crate::game::Game) {
    let p = g.player;
    let time = g.time;
    let Some(h) = g.pl[p].holding_mut() else { return };
    h.after = None;
    if h.ask.is_none() {
        return;
    }
    h.ask = None;
    if let Some(b) = g.pl[p].bubble.as_mut() {
        b.dur = m::min(b.dur, time - b.t + 0.2);
    }
}

/// The waiting remark goes up over Sam: his beat of a bubble, the murmur by the man's ear. Nobody answers it.
fn say_after(g: &mut crate::game::Game) {
    let p = g.player;
    let Some(a) = g.pl[p].holding_mut().and_then(|h| h.after.take()) else { return };
    let time = g.time;
    say_over_sam(g, &a.text, time);
}

/// A remark of Sam's up over him: bubble, murmur. (sayAfter, the pistol line in swapHold, a code he has learnt)
pub fn say_over_sam(g: &mut Game, text: &str, time: f64) {
    let p = g.player;
    let c = g.pl[p].ch;
    g.pl[p].bubble = Some(crate::game::PlayerBubble { text: String::from(text), t: time, dur: sam_beat(js::utf16_len(text)), radio: false });
    let at = v3::add(g.ch[c].pos, [0.0, 1.6, 0.0]);
    crate::audio_iface::game_audio_like_play(g, crate::audio_iface::Sfx::Murmur, Some(at), Some(0.2), None);
}

/// E with the man already in the arm: the OTHER hold from here, and HOLD.secs.swap of lockout begun.
fn swap_hold(g: &mut crate::game::Game) {
    use crate::character::HoldVariant as Var;
    let p = g.player;
    let swap = g.k.hold.secs.swap;
    let Some(h) = g.pl[p].holding_mut() else { return };
    if h.phase != crate::character::HoldPhase::Held {
        return;
    }
    let gd = h.g;
    let to = if h.variant == Var::Gun { Var::Arm } else { Var::Gun };
    h.variant = to;
    h.swap_t = 0.0;
    h.wind_t = -1.0;
    h.fire_queued = false;
    h.gun_out = 0.0;
    if let Some(held) = g.gd[gd].held.as_mut() {
        held.variant = to;
    }
    g.arms[g.pl[p].ch].fire_cd = m::max(g.arms[g.pl[p].ch].fire_cd, swap);
    cancel_ask(g);
    if to == Var::Gun {
        g.pl[p].slot = crate::game::SlotKind::Pistol;
        set_holstered(g, false, Some(true));
        let h = hold!(g, p).clone();
        if !h.remarked && !held_talk(g, &crate::seats::here(g), &h).speaking && g.gd[gd].civ.is_none() {
            if let Some(line) = crate::dialogue::sam_remark_for(g, gd, None) {
                hold!(g, p).remarked = true;
                let time = g.time;
                say_over_sam(g, &line.text, time);
            }
        }
    } else {
        set_holstered(g, true, Some(true));
    }
    crate::audio_iface::game_audio_like_play(g, crate::audio_iface::Sfx::Equip, None, Some(0.45), None);
}

/// HOLD.secs.chokeHold after the choke was chosen: the elbow starts to close.
fn start_choke(g: &mut crate::game::Game) {
    use crate::character::HoldPhase as Ph;
    let p = g.player;
    let Some(gd) = g.pl[p].holding().map(|h| h.g) else { return };
    let hc = g.gd[gd].ch;
    set_phase(g, Ph::Choke);
    let at = v3::add(g.ch[hc].pos, [0.0, 1.4, 0.0]);
    crate::audio_iface::game_audio_like_play(g, crate::audio_iface::Sfx::BodyHit, Some(at), Some(0.3), Some(&crate::audio_iface::PlayOpts::Rate { rate: 0.55 }));
    let pos = v3::copy(g.ch[hc].pos);
    prop_noise(g, pos, CHOKE_NOISE);
}

/// HOLD.secs.chokeHold after the whip was chosen: the pistol comes off his head and cocks back; the crack is whipOut's.
fn start_whip(g: &mut crate::game::Game) {
    let p = g.player;
    if g.pl[p].holding().is_none() {
        return;
    }
    set_phase(g, crate::character::HoldPhase::Whip);
    hold!(g, p).gun_out = 0.0;
    let c = g.pl[p].ch;
    let at = v3::add(g.ch[c].pos, [0.0, 1.5, 0.0]);
    crate::audio_iface::game_audio_like_play(g, crate::audio_iface::Sfx::PropScrape, Some(at), Some(0.12), Some(&crate::audio_iface::PlayOpts::Rate { rate: 2.6 }));
}

/// The man rigid in front, HOLD.dist ahead on Sam's facing, kept out of the statics; what the walls take off his station Sam gives up too.
fn place_held(g: &mut crate::game::Game) {
    let p = g.player;
    let c = g.pl[p].ch;
    let Some(gd) = g.pl[p].holding().map(|h| h.g) else { return };
    let hc = g.gd[gd].ch;
    let r = g.ch[hc].radius;
    let dist = g.k.hold.dist;
    for _ in 0..3 {
        let f = crate::character::character_forward(g, c);
        let wx = g.ch[c].pos[0] + f[0] * dist;
        let wz = g.ch[c].pos[2] + f[2] * dist;
        let mut hp: V3 = [wx, 0.0, wz];
        crate::scene_collision::collide_circle(g, &mut hp, r, 0.2, 1.5, Some(4.0));
        crate::scene_collision::clamp_to_world(&mut hp, r);
        g.ch[hc].pos = hp;
        let dx = hp[0] - wx;
        let dz = hp[2] - wz;
        if dx * dx + dz * dz < 1e-8 {
            break;
        }
        g.ch[c].pos[0] += dx;
        g.ch[c].pos[2] += dz;
        collide_player(g, 1.7);
    }
}

/// Living guards are as solid to the man in his arm as to Sam: one closer than KEEP_OFF to the held man's centre pushes the PAIR back.
fn keep_held_off_guards(g: &mut crate::game::Game, dt: f64) {
    let p = g.player;
    let c = g.pl[p].ch;
    let Some(held) = g.pl[p].holding().map(|h| h.g) else { return };
    let dist = g.k.hold.dist;
    let f = crate::character::character_forward(g, c);
    let hx = g.ch[c].pos[0] + f[0] * dist;
    let hz = g.ch[c].pos[2] + f[2] * dist;
    let mut i = 0;
    while i < g.guards.len() {
        let o = g.guards[i];
        i += 1;
        if o == held || crate::condition::down(g, g.gd[o].ch) || g.gd[o].held.is_some() {
            continue;
        }
        let oc = g.gd[o].ch;
        let dx = hx - g.ch[oc].pos[0];
        let dz = hz - g.ch[oc].pos[2];
        let l = m::hyp2(dx, dz);
        if l >= KEEP_OFF || l < 1e-4 {
            continue;
        }
        if crate::noshove::falling_in_yields(g, &crate::seats::here(g), o) {
            continue;
        }
        let (bx, bz) = (g.ch[c].pos[0], g.ch[c].pos[2]);
        let push = m::min(KEEP_OFF - l, 6.0 * dt);
        g.ch[c].pos[0] += dx / l * push;
        g.ch[c].pos[2] += dz / l * push;
        collide_player(g, 1.7);
        let (ndx, ndz) = (g.ch[c].pos[0] - bx, g.ch[c].pos[2] - bz);
        crate::noshove::no_shove_note(g, ndx, ndz, None);
    }
}

/// Where the exchange with the man in his arm stands. `speaking` says the man's bubble is his answer: a write goes to `g.gd[h.g].bubble`.
pub(super) fn held_talk(g: &crate::game::Game, seat: &crate::seats::SeatRef, h: &crate::player::Hold) -> crate::player::HeldTalk {
    let gd = h.g;
    let talk = g.gd[gd].talk;
    let last_t = g.tab.game_talk[talk].last_t;
    let time = g.time;
    let speaking = match &g.gd[gd].bubble {
        Some(b) if last_t >= 0.0 && m::abs(b.t - last_t) < 1e-6 && time - b.t < b.dur => Some(b.clone()),
        _ => None,
    };
    let fresh = last_t < 0.0 || last_t < h.since - 1e-6;
    let asking = h.ask.is_some();
    let sam_up = asking
        || match &g.pl[seat.player].bubble {
            Some(sb) => sb.t >= h.since - 1e-6 && time - sb.t < sb.dur - 1e-9,
            None => false,
        };
    let ready = !sam_up && (fresh || speaking.is_none() || time + 1e-6 >= last_t + INTERROGATE_GAP);
    let settled = ready
        && match &speaking {
            None => true,
            Some(s) => time + 1e-6 >= s.t + s.dur - ANSWER_TAIL,
        };
    crate::player::HeldTalk { asking, sam_up, ready, settled, speaking: speaking.is_some() }
}

/// A tap on his row, one EXCHANGE: Sam's line first, up for its beat; the answer follows when it ends.
fn ask_held(g: &mut crate::game::Game) {
    let p = g.player;
    let Some(h) = g.pl[p].holding().cloned() else { return };
    let gd = h.g;
    let talk = held_talk(g, &crate::seats::here(g), &h);
    if talk.sam_up {
        return;
    }
    let time = g.time;
    if !talk.ready {
        if talk.speaking {
            if let Some(b) = g.gd[gd].bubble.as_mut() {
                b.dur = m::min(b.dur, time - b.t + 0.3);
            }
        }
        return;
    }
    if talk.speaking {
        if let Some(b) = g.gd[gd].bubble.as_mut() {
            b.dur = m::min(b.dur, time - b.t);
        }
    }
    let text = if g.gd[gd].civ.is_some() {
        crate::civilians::civilian_sam_line(g, gd).text
    } else {
        crate::dialogue::sam_line_for(g, gd, h.variant, None).text
    };
    let dur = sam_beat(js::utf16_len(&text));
    let time = g.time;
    g.pl[p].bubble = Some(crate::game::PlayerBubble { text: text.clone(), t: time, dur, radio: false });
    {
        let h = hold!(g, p);
        h.ask = Some(crate::player::HoldAsk { t: time, dur });
        h.after = None;
    }
    let c = g.pl[p].ch;
    let at = v3::add(g.ch[c].pos, [0.0, 1.6, 0.0]);
    crate::audio_iface::game_audio_like_play(g, crate::audio_iface::Sfx::Murmur, Some(at), Some(0.2), None);
}

/// Sam's line has hung its beat: the man answers it, and the muffled sound of it under the arm.
fn answer_held(g: &mut crate::game::Game) {
    let p = g.player;
    let Some(gd) = g.pl[p].holding().map(|h| h.g) else { return };
    let pick = crate::game::game_interrogate(g, gd, None);
    if pick.is_some() {
        let hc = g.gd[gd].ch;
        let at = v3::add(g.ch[hc].pos, [0.0, 1.5, 0.0]);
        crate::audio_iface::game_audio_like_play(g, crate::audio_iface::Sfx::HeldProtest, Some(at), Some(0.16), None);
    }
}

/// The man has given the keyed door up: Sam takes what is on his belt; with the man in his arm, Sam's word on it follows the man's line down.
pub fn take_key(g: &mut crate::game::Game, gd: GuardId, rnd: Option<&mut dyn FnMut(&mut crate::game::Game) -> f64>) {
    let p = g.player;
    let mut fresh: Vec<String> = Vec::new();
    for k in g.gd[gd].keys.keys() {
        if !g.pl[p].keys.has(k.clone()) {
            fresh.push(k);
        }
    }
    g.gd[gd].keys.clear();
    if fresh.is_empty() {
        return;
    }
    for k in fresh {
        g.pl[p].keys.add(k);
    }
    let hc = g.gd[gd].ch;
    let at = v3::add(g.ch[hc].pos, [0.0, 1.0, 0.0]);
    crate::audio_iface::game_audio_like_play(g, crate::audio_iface::Sfx::LockPick, Some(at), Some(0.45), Some(&crate::audio_iface::PlayOpts::Rate { rate: 1.6 }));
    crate::mission::tally(g, crate::mission::TallyK::Keys);
    crate::chronicle::chron_key(g, gd);
    let variant = match g.pl[p].holding() {
        Some(h) if h.g == gd => Some(h.variant),
        _ => None,
    };
    if let Some(variant) = variant {
        if let Some(line) = crate::dialogue::sam_key_line_for(g, gd, variant, rnd) {
            if let Some(h) = g.pl[p].holding_mut() {
                h.after = Some(crate::player::HoldAfter { text: line.text });
            }
        }
    }
}

/// After the door leaves have had their say: whatever a leaf shoved the held man off his station by, the PAIR gives. Not during the reach.
pub fn settle_held(g: &mut crate::game::Game) {
    let p = g.player;
    let gd = match g.pl[p].holding() {
        Some(h) if h.phase != crate::character::HoldPhase::Grab => h.g,
        _ => return,
    };
    let c = g.pl[p].ch;
    let hc = g.gd[gd].ch;
    let f = crate::character::character_forward(g, c);
    let dist = g.k.hold.dist;
    let hp = g.ch[hc].pos;
    let dx = hp[0] - (g.ch[c].pos[0] + f[0] * dist);
    let dz = hp[2] - (g.ch[c].pos[2] + f[2] * dist);
    if dx * dx + dz * dz < 1e-6 {
        return;
    }
    g.ch[c].pos[0] += dx;
    g.ch[c].pos[2] += dz;
    collide_player(g, 1.7);
    place_held(g);
    crate::character::character_update(g, hc, 0.0);
    crate::character::character_update(g, c, 0.0);
}

/// The choke run through HOLD.secs.choke: he goes limp in the arm and folds forward at Sam's feet, out cold.
fn choke_out(g: &mut crate::game::Game) {
    let p = g.player;
    let Some(gd) = g.pl[p].holding().map(|h| h.g) else { return };
    let c = g.pl[p].ch;
    let hc = g.gd[gd].ch;
    let f = crate::character::character_forward(g, c);
    unhook(g, gd);
    let at = v3::add(g.ch[hc].pos, [0.0, 1.4, 0.0]);
    crate::audio_iface::game_audio_like_play(g, crate::audio_iface::Sfx::ChokeLimp, Some(at), Some(0.35), None);
    crate::takedown::take_fall(g, gd, None, false, Some(crate::takedown::TakeFallHow::Choke));
    crate::combat::kill_guard(g, gd, f, Some(true), Some(crate::game::DownKind::Choked), Some(crate::game::DownBy::Player), crate::game::With::Hand);
    let pos = v3::copy(g.ch[hc].pos);
    prop_noise(g, pos, FOLD_NOISE);
    g.pl[p].holding_over();
}

/// The whip run through HOLD.secs.whip: the pistol lands behind his right ear and he drops out of the arm, the loudest thing a hold does.
fn whip_out(g: &mut crate::game::Game) {
    let p = g.player;
    let Some(gd) = g.pl[p].holding().map(|h| h.g) else { return };
    let c = g.pl[p].ch;
    let hc = g.gd[gd].ch;
    let f = crate::character::character_forward(g, c);
    let dir = v3::normalize([f[0] + f[2] * 0.45, 0.0, f[2] - f[0] * 0.45]);
    let head = g.ch[hc].bone_or(crate::character::Bone::Head, 1.6);
    unhook(g, gd);
    crate::takedown::take_fall(g, gd, None, false, Some(crate::takedown::TakeFallHow::Whip));
    crate::combat::kill_guard(g, gd, dir, Some(true), Some(crate::game::DownKind::Struck), Some(crate::game::DownBy::Player), crate::game::With::Whip);
    crate::audio_iface::game_audio_like_play(g, crate::audio_iface::Sfx::WhipThud, Some(head), Some(0.7), None);
    let pos = v3::copy(g.ch[hc].pos);
    prop_noise(g, pos, WHIP_NOISE);
    g.pl[p].holding_over();
    g.arms[g.pl[p].ch].fire_cd = m::max(g.arms[g.pl[p].ch].fire_cd, 0.5);
}

/// He comes out of the arm his own man and an instant witness; a man already down is only unhooked.
pub(super) fn hand_back(g: &mut crate::game::Game, gd: GuardId, stagger: f64) {
    let mut stagger = stagger;
    let hc = g.gd[gd].ch;
    unhook(g, gd);
    g.gd[gd].speed = 0.0;
    g.ch[hc].vel = [0.0, 0.0, 0.0];
    g.gd[gd].drop_path();
    g.gd[gd].body_duty = None;
    g.gd[gd].stall_ref = None;
    g.ch[hc].aim_pitch = 0.0;
    if crate::condition::down(g, g.gd[gd].ch) {
        return;
    }
    if g.gd[gd].gun_lost_at >= 0.0 {
        // HOLD.dropGun: the gun knocked out of his hand at the grab is the first thing he goes for, at the price of a longer stagger
        crate::guards::gun_back(g, gd);
        stagger += g.k.hold.regrab;
    }
    let p = g.player;
    let pc = g.pl[p].ch;
    let pp = g.ch[pc].pos;
    let time = g.time;
    let seq = g.event_seq;
    {
        crate::mind::set(g, gd, 1.0, "player");
        crate::mind::fix(g, gd, Some(v3::copy(pp)), "player");
        g.gd[gd].last_seen_t = time;
        g.gd[gd].sight_pos = v3::copy(pp);
        g.gd[gd].sight_vel = [0.0, 0.0, 0.0];
        g.gd[gd].sight_t = time;
        g.gd[gd].led_t = time;
        g.gd[gd].react_t = m::max(g.gd[gd].react_t, stagger);
        g.gd[gd].heard_up_to = seq;
        g.gd[gd].missed_at = -1.0;
        g.gd[gd].missing_raised = false;
    }
    crate::guards::reset_chase(g, gd);
    g.gd[gd].search_t = 0.0;
    crate::character::anim_hit(g, hc);
}

#[cfg(test)]
mod tests {
    use super::*;

    /// How long Sam's line hangs goes by its letters: a dash or an arrow in it is one, not its bytes.
    #[test]
    fn a_line_is_counted_in_letters_not_bytes() {
        assert_eq!(js::utf16_len("you ▸ — é"), 9.0);
    }

}
