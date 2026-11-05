//! src/game/player.ts lines 1610 to 2026
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// the hold of this seat, where the TypeScript keeps `H`: re-read at every use, never held across a call that takes `g`
macro_rules! hold {
    ($g:ident, $p:ident) => {
        $g.pl[$p].holding.as_mut().unwrap()
    };
}

/// `g.audio.play(name, pos, gain, { rate })`: `game_audio_like_play`'s opts are a map of `Opaque`, a unit struct, which cannot hold the rate
fn seam_audio_play_rate(g: &mut Game, name: &str, pos: V3, gain: f64, rate: f64) {
    todo!("needs: a rate in game_audio_like_play's opts")
}

/// JavaScript's `s.length`: UTF-16 units, not bytes (Sam's lines hold dashes and arrows)
fn js_len(s: &str) -> f64 {
    s.encode_utf16().count() as f64
}

/// `g.events.push({ kind: 'prop', pos, time, loud: false, level, who: <Sam's body> })`, and `pl.noise` raised to it: what the room hears of a hold's beat
fn prop_noise(g: &mut Game, pos: V3, level: f64) {
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
        ..crate::game::GameEvent::zeroed()
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

/// One frame with a man in his arm: the beat's clock and inputs, his rows, the gun, then the pair's movement.   (player.ts updateHold)
pub fn update_hold(g: &mut crate::game::Game, dt: f64, input: InputId) {
    let p = g.player;
    let c = g.pl[p].ch;
    let gd = hold!(g, p).g;
    let hc = g.gd[gd].ch;
    let mut gun = hold!(g, p).variant == crate::character::HoldVariant::Gun;
    if gun && crate::input::input_hit(g, input, "KeyL") && crate::kit::kit_you_light(g, p) {
        let f = g.pl[p].pistol;
        g.fa[f].light_on = !g.fa[f].light_on;
        let muzzle = g.ch[c].muzzle;
        crate::audio_iface::game_audio_like_play(g, "click", Some(muzzle), Some(0.6), None);
    }
    nv_key(g, Some(input), dt);
    let by_player = match &g.gd[gd].held {
        Some(h) => h.by == crate::game::GuardHeldBy::Player,
        None => false,
    };
    if !crate::game::game_on_floor(g, gd) || g.gd[gd].state == crate::game::GuardState::Dead || !by_player {
        drop_hold(g);
        crate::character::character_update(g, c, dt);
        return;
    }
    {
        let h = hold!(g, p);
        h.t += dt;
        h.swap_t += dt;
    }
    g.pl[p].fire_cd -= dt;
    let k = g.k.hold.secs;
    let dist = g.k.hold.dist;
    let mut speed = 0.0;
    let mut own = -1.0;
    let mut step_u = -1.0;
    if hold!(g, p).phase == crate::character::HoldPhase::Grab {
        // easing in to HOLD.dist behind him on HIS facing, squaring up to his back, through Sam's own collision
        let yaw = g.ch[hc].body_yaw;
        let (sy, cy) = (m::sin(yaw), m::cos(yaw));
        let want: V3 = [g.ch[hc].pos[0] - sy * dist, 0.0, g.ch[hc].pos[2] - cy * dist];
        let from = hold!(g, p).from;
        match from {
            Some(from) if g.k.stalk.catch_secs > 0.0 => {
                // stalk.ts: the step in, then the yank
                if hold!(g, p).step_m.is_none() {
                    let yaw0 = g.ch[c].body_yaw;
                    let h = hold!(g, p);
                    h.step_m = Some(m::hyp2(want[0] - from[0], want[2] - from[2]));
                    h.yaw0 = Some(yaw0);
                }
                let step_m = hold!(g, p).step_m.unwrap();
                let tt = m::min(g.k.stalk.catch_secs * v3::clamp(step_m / m::max(0.1, g.k.stalk.catch_m), 1.0, g.k.stalk.catch_long), k.grab - 0.08);
                let u = v3::clamp(hold!(g, p).t / tt, 0.0, 1.0);
                let was = g.ch[c].pos; // (c.pos is only ever replaced between here and its read, never written in place)
                let mut to = want;
                step_u = u;
                if u >= 1.0 && g.k.stalk.yank > 0.0 && g.gd[gd].civ.is_none() {
                    if hold!(g, p).hit_at.is_none() {
                        let (time, yank, room) = (g.time, g.k.stalk.yank, g.k.stalk.yank_room);
                        hold!(g, p).hit_at = Some(time);
                        let hit = crate::scene_collision::raycast(g, [want[0], 1.0, want[2]], [-sy, 0.0, -cy], yank + room);
                        let h = hold!(g, p);
                        h.yank_m = Some(match hit {
                            Some(hit) => m::max(0.0, hit.t - room),
                            None => yank,
                        });
                        h.yanked = Some(0.0);
                    }
                    let (ht, yank_m, yanked) = {
                        let h = hold!(g, p);
                        (h.t, h.yank_m.unwrap_or(m::NAN), h.yanked.unwrap_or(m::NAN))
                    };
                    let v = v3::clamp((ht - tt) / m::max(0.05, k.grab - tt), 0.0, 1.0);
                    let y = yank_m * (1.0 - (1.0 - v) * (1.0 - v));
                    let d = y - yanked;
                    if d > 0.0 {
                        let hp = g.ch[hc].pos;
                        g.ch[hc].pos = [hp[0] - sy * d, hp[1], hp[2] - cy * d];
                        hold!(g, p).yanked = Some(y);
                        to = [want[0] - sy * d, 0.0, want[2] - cy * d];
                        // STALK.yankWith: he goes back WITH the man
                        let cp = g.ch[c].pos;
                        g.ch[c].pos = [cp[0] - sy * d, cp[1], cp[2] - cy * d];
                    }
                } else if u >= 1.0 && hold!(g, p).hit_at.is_none() {
                    let time = g.time;
                    hold!(g, p).hit_at = Some(time);
                }
                // by the step's own delta from where the collision last left him, never the straight line's point
                let e = crate::stalk::step_ease(u);
                let e0 = hold!(g, p).step_e.unwrap_or(0.0);
                let d0 = v3::sub(v3::lerp3(from, to, e), v3::lerp3(from, to, e0));
                hold!(g, p).step_e = Some(e);
                let cp = g.ch[c].pos;
                g.ch[c].pos = if u >= 1.0 { v3::lerp3(cp, to, 1.0 - m::exp(-14.0 * dt)) } else { [cp[0] + d0[0], cp[1], cp[2] + d0[2]] };
                let now = g.ch[c].pos;
                own = if u >= 1.0 { 0.0 } else { m::min(g.k.stalk.catch_stride, m::hyp2(now[0] - was[0], now[2] - was[2]) / m::max(dt, 1e-4)) };
                g.ch[c].vel = [0.0, 0.0, 0.0];
                collide_player(g, 1.7);
            }
            _ => {
                g.ch[c].vel = [0.0, 0.0, 0.0];
                let cp = g.ch[c].pos;
                g.ch[c].pos = v3::lerp3(cp, want, 1.0 - m::exp(-14.0 * dt));
                collide_player(g, 1.7);
            }
        }
        // the stalk's step in: he comes round to the man's back THROUGH the step (STALK.turnIn)
        let yaw0 = hold!(g, p).yaw0;
        match yaw0 {
            Some(yaw0) if step_u >= 0.0 && step_u < 1.0 => {
                let e = v3::clamp(step_u / 0.85, 0.0, 1.0);
                g.ch[c].body_yaw = yaw0 + v3::wrap_angle(yaw - yaw0) * e * e * (3.0 - 2.0 * e);
            }
            _ => g.ch[c].body_yaw = v3::approach_angle(g.ch[c].body_yaw, yaw, 14.0 * dt),
        }
        g.ch[c].aim_yaw = v3::approach_angle(g.ch[c].aim_yaw, g.ch[c].body_yaw, 12.0 * dt);
        g.ch[c].aim_pitch = v3::damp(g.ch[c].aim_pitch, 0.0, 8.0, dt);
        g.gd[gd].speed = 0.0;
        g.ch[hc].vel = [0.0, 0.0, 0.0];
        // CLOSE.rmbHold: the button let go while the grab was still landing: a plain hold
        if hold!(g, p).rmb_choke.map_or(false, js::truthy) && !crate::input::input_rmb(g, input) {
            hold!(g, p).rmb_choke = None;
        }
        if hold!(g, p).t >= k.grab {
            set_phase(g, crate::character::HoldPhase::Held);
            let time = g.time;
            hold!(g, p).since = time;
            if let Some(h) = g.gd[gd].held.as_mut() {
                h.since = time;
            }
            let talk = g.gd[gd].talk;
            g.tab.game_talk[talk].held_since = time;
            let his_yaw = g.ch[hc].body_yaw;
            g.ch[c].aim_yaw = his_yaw;
            g.ch[c].body_yaw = his_yaw;
            let at = v3::add(g.ch[hc].pos, [0.0, 1.5, 0.0]);
            crate::audio_iface::game_audio_like_play(g, "heldProtest", Some(at), Some(0.18), None);
            let callsign = g.gd[gd].callsign.clone();
            if let Some(cv) = g.gd[gd].civ {
                let (label, her, she) = (g.cv[cv].label.clone(), g.cv[cv].her.clone(), g.cv[cv].she.clone());
                let text = if gun {
                    format!("got {callsign}, the {label}. press Space to tell {her} to stay down and quiet (lets {her} go) · hold Space for the pistol-whip · press E to holster and question {her}")
                } else {
                    format!("got {callsign}, the {label}. press Space to ask what {she} knows · hold Space for 'stay down, stay quiet' or the choke · knocking out a civilian will cost you")
                };
                crate::game::game_msg(g, &text);
            } else if !hold!(g, p).rmb_choke.map_or(false, js::truthy) {
                let text = if gun {
                    format!("got {callsign} with the pistol at his head. click to fire past him, press Space to pistol-whip him, or E to holster it and question him")
                } else {
                    format!("got {callsign}. press Space to make him talk, hold Space for the choke, or press 1 to put the gun to his head")
                };
                crate::game::game_msg(g, &text);
            }
        }
    } else {
        use crate::character::HoldPhase as Ph;
        // ---- E: the other hold (held only, and not while the hand is still between tools from the last)
        if hold!(g, p).phase == Ph::Held
            && hold!(g, p).swap_t >= k.swap
            && (crate::input::input_hit(g, input, "KeyE") || crate::input::input_hit(g, input, if gun { crate::handslot::HANDSLOT_KEY } else { "Digit1" }))
            && !crate::game::player_choosing(g, p)
            && g.pl[p].sel.down_t < 0.0
        {
            swap_hold(g);
            gun = hold!(g, p).variant == crate::character::HoldVariant::Gun;
        }
        let swapping = hold!(g, p).swap_t < k.swap;
        // ---- his rows, performed this frame (interact.ts → Game.act)
        let act = match &g.act {
            Some(a) if a.kind == crate::game::InteractableKind::Held => a.verb,
            _ => None,
        };
        use crate::game::Verb;
        // ---- the two buttons with him in the arm (CLOSE.heldClicks): both in one frame choose nothing
        let (mut clk_kill, mut clk_choke) = (false, false);
        if hold!(g, p).phase == Ph::Held && !gun && !swapping && hold!(g, p).wind_t < 0.0 && g.gd[gd].civ.is_none() && !g.touring {
            if crate::input::input_lmb_hit(g, input) && !crate::close::close_bare(g) {
                if !crate::input::input_rmb_hit(g, input) {
                    clk_kill = true;
                }
            } else if crate::input::input_rmb_hit(g, input) {
                clk_choke = true;
            }
        }
        if clk_kill {
            start_kill(g);
        }
        if hold!(g, p).phase == Ph::Held {
            if act == Some(Verb::Comply) && g.gd[gd].civ.is_some() && !swapping {
                if crate::civilians::civilian_comply(g) {
                    hold_tail(g, dt, 0.0, None);
                    return;
                }
            }
            if act == Some(Verb::Question) && !swapping {
                if gun {
                    let text = format!("press E to holster the gun first, then Space to question {}", g.gd[gd].callsign);
                    crate::game::game_msg(g, &text);
                } else {
                    ask_held(g);
                }
            }
            if clk_choke {
                // CLOSE.rmbHold: in the arm too the right button kept down is the choke for as long as it is down, and a tap of it the whole choke
                let h = hold!(g, p);
                h.rmb_choke = Some(1.0);
                h.rmb_t = Some(0.0);
            }
            if !swapping && ((act == Some(Verb::Choke) && !gun) || (act == Some(Verb::Whip) && gun)) {
                hold!(g, p).wind_t = 0.0;
            }
            if hold!(g, p).wind_t >= 0.0 && !swapping {
                hold!(g, p).wind_t += m::min(dt, 0.05);
                if hold!(g, p).wind_t >= k.choke_hold {
                    hold!(g, p).wind_t = -1.0;
                    if gun {
                        start_whip(g);
                    } else {
                        start_choke(g);
                    }
                }
            }
        }
        if hold!(g, p).rmb_choke.map_or(false, js::truthy) {
            // CLOSE.rmbHold: the arm closes the frame it can, with no commit beat: the button is the commitment
            let rmb_t = hold!(g, p).rmb_t.unwrap_or(0.0) + dt;
            hold!(g, p).rmb_t = Some(rmb_t);
            if hold!(g, p).rmb_choke == Some(1.0) {
                if hold!(g, p).phase == Ph::Held && !gun && !swapping && g.gd[gd].civ.is_none() && !g.touring {
                    hold!(g, p).wind_t = -1.0;
                    start_choke(g);
                    let h = hold!(g, p);
                    h.rmb_choke = if h.phase == Ph::Choke { Some(2.0) } else { None };
                } else if !crate::input::input_rmb(g, input) {
                    hold!(g, p).rmb_choke = None;
                }
            } else if hold!(g, p).phase != Ph::Choke {
                hold!(g, p).rmb_choke = None;
            }
            if hold!(g, p).rmb_choke == Some(2.0) && !crate::input::input_rmb(g, input) {
                if hold!(g, p).rmb_t.unwrap_or(m::NAN) > g.k.close.rmb_tap_secs + 1e-6 {
                    ease_off_hold(g);
                }
                hold!(g, p).rmb_choke = None;
            }
        }
        // Sam's line has hung its beat: the man answers it
        let ask_due = {
            let time = g.time;
            let h = hold!(g, p);
            h.phase == Ph::Held && h.ask.map_or(false, |a| time + 1e-6 >= a.t + a.dur)
        };
        if ask_due {
            hold!(g, p).ask = None;
            answer_held(g);
        }
        // a remark that waited for the man to finish: the answer is down and no question is going up
        if hold!(g, p).after.is_some() && hold!(g, p).phase == Ph::Held && hold!(g, p).ask.is_none() {
            let h = hold!(g, p).clone();
            if held_talk(g, &h).speaking.is_none() {
                say_after(g);
            }
        }
        let (phase_now, t_now) = {
            let h = hold!(g, p);
            (h.phase, h.t)
        };
        if phase_now == Ph::Choke && t_now < k.choke - 0.3 && m::floor(t_now / CHOKE_GRAIN) > m::floor((t_now - dt) / CHOKE_GRAIN) {
            let at = v3::add(g.ch[hc].pos, [0.0, 1.5, 0.0]);
            crate::audio_iface::game_audio_like_play(g, "chokeStrain", Some(at), Some(0.25), None);
        }
        if phase_now == Ph::Choke && t_now >= k.choke {
            choke_out(g);
            hold_tail(g, dt, 0.0, None);
            return;
        }
        if phase_now == Ph::Whip && t_now >= k.whip {
            whip_out(g);
            hold_tail(g, dt, 0.0, None);
            return;
        }
        let killing = hold_killing(hold!(g, p));
        if killing && t_now >= kill_secs(g) {
            // the clock's LAST frame of the knife is drawn on Sam, and the hold lets go after it
            hold_tail(g, dt, 0.0, None);
            kill_out(g);
            g.ch[c].anim.hold_pose = None;
            return;
        }
        // ---- the gun: a click queues one round, the pistol comes off his head and it goes, never at the man in the arm
        let pistol = g.pl[p].pistol;
        let gun_held = gun && phase_now == Ph::Held;
        if gun_held {
            if !swapping && crate::input::input_lmb_hit(g, input) && !hold!(g, p).fire_queued && g.pl[p].fire_cd <= 0.0 {
                if crate::weapons::firearm_can_fire(g, pistol) {
                    hold!(g, p).fire_queued = true;
                } else {
                    g.pl[p].fire_cd = 0.25;
                    let muzzle = g.ch[c].muzzle;
                    crate::audio_iface::game_audio_like_play(g, "dryFire", Some(muzzle), Some(0.7), None);
                    crate::game::game_msg(g, "click — empty, and no hand free to reload");
                }
            }
            if !swapping && crate::input::input_rmb_hit(g, input) && (crate::lighting::ocpable(g) || (g.aim_target.is_none() && g.aim_guard.is_none())) {
                let target = g.aim_target;
                crate::game::game_ocp(g, target);
            }
            let time = g.time;
            let h = hold!(g, p);
            let present = h.fire_queued || (h.shot_t >= 0.0 && time - h.shot_t < 0.9);
            h.gun_out = v3::damp(h.gun_out, if present { 1.0 } else { 0.0 }, if present { 30.0 } else { 6.0 }, dt);
        } else {
            let h = hold!(g, p);
            h.fire_queued = false;
            h.gun_out = v3::damp(h.gun_out, 0.0, 6.0, dt);
        }
        // what the rest of the frame reads of H, read BEFORE the round: fireWeapon can end the hold (a blast reaching Sam: combat.ts freeHeld), and the TypeScript's H lives on detached
        let (gun_out, hit_at, phase_after, fire_now) = {
            let h = hold!(g, p);
            (h.gun_out, h.hit_at, h.phase, gun_held && h.fire_queued && h.gun_out > 0.8)
        };
        if fire_now {
            hold!(g, p).fire_queued = false;
            if crate::weapons::firearm_fire(g, pistol) {
                let time = g.time;
                g.pl[p].fire_cd = HOLD_FIRE_CD;
                g.pl[p].last_fire_t = time;
                hold!(g, p).shot_t = time;
                let ag = match g.aim_guard {
                    Some(a) if a != gd && g.gd[a].state != crate::game::GuardState::Dead => Some(a),
                    _ => None,
                };
                let tgt: V3 = if let Some(t) = g.aim_target {
                    v3::copy(g.tab.sim_lightshome_fixture[t].pos)
                } else if let Some(a) = ag {
                    man_aim(g, a, 1.25)
                } else {
                    free_aim(g, Some(0.6))
                };
                let (spread_sup, spread_aim) = (crate::suppression::sam_spread(g), crate::aim::sam_aim(g));
                let opts = crate::combat::FireOpts { spread: Some(HOLD_SPREAD + spread_sup + spread_aim), ..crate::combat::FireOpts::zeroed() };
                crate::combat::fire_weapon(g, c, tgt, true, Some(&opts));
                if crate::weapons::firearm_rounds_ready(g, pistol) == 0.0 {
                    crate::game::game_msg(g, "empty — and no hand free to reload");
                }
            }
        }
        // ---- turning: the pair comes round toward the cursor, rate-limited
        let ap = g.aim_point;
        let tx = ap[0] - g.ch[c].pos[0];
        let tz = ap[2] - g.ch[c].pos[2];
        let ad = m::hyp2(tx, tz);
        // STALK.yankWith: the stalk's catch settles before the pair comes round
        let settle = match hit_at {
            Some(at) => v3::smoothstep(0.3, 0.75, g.time - at),
            None => 1.0,
        };
        if ad > 0.6 {
            g.ch[c].body_yaw = v3::approach_angle(g.ch[c].body_yaw, m::atan2(tx, tz), HOLD_TURN * settle * dt);
        }
        g.ch[c].aim_yaw = g.ch[c].body_yaw;
        if gun {
            let ty = if let Some(t) = g.aim_target {
                v3::clamp(g.tab.sim_lightshome_fixture[t].pos[1], 0.3, 2.95)
            } else {
                match g.aim_guard {
                    Some(a) if a != gd => match (g.gd[a].wound.is_some(), g.aim_snap) {
                        (true, Some(snap)) => snap[1],
                        _ => man_aim(g, a, 1.2)[1],
                    },
                    _ => {
                        if level_on(g) {
                            g.ch[c].pos[1] + 1.5
                        } else {
                            ap[1] + 0.03
                        }
                    }
                }
            };
            let want = v3::clamp(m::atan2(ty - (g.ch[c].pos[1] + 1.5), m::max(ad, 0.5)), -45.0 * v3::DEG, 45.0 * v3::DEG) * gun_out;
            g.ch[c].aim_pitch = v3::damp(g.ch[c].aim_pitch, want, 12.0, dt);
        } else {
            g.ch[c].aim_pitch = v3::damp(g.ch[c].aim_pitch, -0.05, 8.0, dt);
        }
        // ---- the pair's movement (rooted while the choke / the whip is on; feet still while the selector is open)
        let (mut mx, mut mz) = (0.0, 0.0);
        if phase_after == Ph::Held {
            let basis = crate::camera::follow_camera_planar_basis(g);
            let (fwd, right) = (basis.fwd, basis.right);
            if !crate::game::player_list_open(g, p) {
                if crate::input::input_down(g, input, "KeyW") {
                    mx += fwd[0];
                    mz += fwd[2];
                }
                if crate::input::input_down(g, input, "KeyS") {
                    mx -= fwd[0];
                    mz -= fwd[2];
                }
                if crate::input::input_down(g, input, "KeyD") {
                    mx += right[0];
                    mz += right[2];
                }
                if crate::input::input_down(g, input, "KeyA") {
                    mx -= right[0];
                    mz -= right[2];
                }
            }
            if let Some(puppet) = &g.puppet {
                mx = 0.0;
                mz = 0.0;
                if let Some(gl) = puppet.goal {
                    let dx = gl[0] - g.ch[c].pos[0];
                    let dz = gl[2] - g.ch[c].pos[2];
                    let dl = m::hyp2(dx, dz);
                    if dl > 0.1 {
                        mx = dx / dl;
                        mz = dz / dl;
                    }
                }
            }
        }
        let ml = m::hyp2(mx, mz);
        if ml > 0.0 {
            mx /= ml;
            mz /= ml;
        }
        let f = crate::character::character_forward(g, c);
        let back = ml > 0.0 && mx * f[0] + mz * f[2] < -0.3; // into the rear half: hauling him backwards
        let walk = g.puppet.as_ref().and_then(|x| x.walk).unwrap_or(1.0);
        let pace_in = if ml == 0.0 { 0.0 } else { (if back { HOLD_PACE_BACK } else { HOLD_PACE }) * g.pl[p].pace * walk };
        let max_speed = crate::kitload::sam_load_pace(g, pace_in, None);
        g.pl[p].speed_sm = v3::damp(g.pl[p].speed_sm, max_speed, if max_speed > g.pl[p].speed_sm { 6.0 } else { 12.0 }, dt);
        if ml > 0.0 {
            let sm = g.pl[p].speed_sm;
            g.ch[c].vel = [mx * sm, 0.0, mz * sm];
        } else {
            g.ch[c].vel = v3::scale(g.ch[c].vel, m::exp(-14.0 * dt));
        }
        let (bx, bz) = (g.ch[c].pos[0], g.ch[c].pos[2]);
        g.ch[c].pos = v3::mad(g.ch[c].pos, g.ch[c].vel, dt);
        collide_player(g, 1.7);
        keep_off_guards(g, dt, 1.7, None);
        keep_held_off_guards(g, dt);
        place_held(g);
        let (cp, cv) = (g.ch[c].pos, g.ch[c].vel);
        speed = m::min(m::hyp2(cp[0] - bx, cp[2] - bz) / m::max(dt, 1e-4), m::hyp2(cv[0], cv[2]));
        let backpedal = speed > 0.2 && (cv[0] * f[0] + cv[2] * f[2]) / m::max(1e-4, m::hyp2(cv[0], cv[2])) < -0.3;
        g.pl[p].backpedal = backpedal;
        g.ch[c].anim.reverse = backpedal;
        g.ch[hc].vel = v3::copy(cv);
        g.gd[gd].speed = speed;
        g.ch[hc].body_yaw = g.ch[c].body_yaw;
        if let Some(h) = g.gd[gd].held.as_mut() {
            h.reverse = backpedal;
        }
    }
    g.pl[p].sprinting = false;
    g.pl[p].crouch = false;
    hold_tail(g, dt, speed, Some(own));
}

/// The end of a hold frame, Sam's side, shared with the frames that end the hold: both animators told the beat, the rail light, Sam's step events.   (player.ts holdTail)
pub fn hold_tail(g: &mut crate::game::Game, dt: f64, speed: f64, own: Option<f64>) {
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
    let has = g.pl[p].holding.is_some();
    let low = if has { crate::takedown::take_lower(g) } else { 0.0 };
    let kill = if has { crate::takedown::take_kill(g) } else { false };
    let beats = if has { crate::takedown::take_beats(g) } else { false };
    let haul = if has { crate::takedown::take_haul(g) } else { false };
    let h = g.pl[p].holding.clone();
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
        kill: Some(kill),
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
    let bare = if crate::close::close_bare(g) { Some(true) } else { None };
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
        kill: Some(kill),
        beats: Some(beats),
        haul: Some(haul),
        strip: strip.map(|s| crate::character::StripPose { t: s.t, at: None }),
    });
    if let Some(held) = g.gd[gd].held.as_mut() {
        held.phase = h.phase;
    }
    // where his torch points: up over Sam's arm at the ceiling, wandering with the struggle
    let torch = crate::guards::hand_carry(g, gd) == crate::guards::HandCarry::Torch;
    let tt = time - h.since;
    let yaw_to = g.ch[hc].body_yaw + (if torch { 0.07 * m::sin(tt * 0.9) + 0.04 * m::sin(tt * 2.3 + 1.0) } else { 0.0 });
    g.ch[hc].aim_yaw = v3::approach_angle(g.ch[hc].aim_yaw, yaw_to, 4.0 * dt);
    let pitch_to = if torch { 0.55 + 0.25 * m::sin(tt * 1.3 + 2.0) + 0.1 * m::sin(tt * 3.1) } else { 0.0 };
    g.ch[hc].aim_pitch = v3::damp(g.ch[hc].aim_pitch, pitch_to, 6.0, dt);
    tail_end(g, dt, speed, c);
}

/// holdTail's last lines, with a hold or without one: the sling, the body's bake, the rail light, the dragging shuffle.   (player.ts holdTail)
fn tail_end(g: &mut Game, dt: f64, speed: f64, c: CharId) {
    let p = g.player;
    arm_rifle(g, false, None);
    crate::character::character_update(g, c, dt);
    pose_gun_light(g);
    let phase = g.ch[c].anim.phase;
    if js::truthy(footfall_due(g, dt, phase, speed)) {
        let heard = crate::kitload::load_heard(g, crate::kitload::LoadHeardWho::Player);
        let at = g.ch[c].pos;
        let loud = crate::floor::tread(g, at, 0.26 * heard, true);
        let (pos, time, serial) = (v3::copy(g.ch[c].pos), g.time, g.serial());
        g.events.push(crate::game::GameEvent {
            kind: crate::director::EventKind::Step,
            pos,
            time,
            loud: false,
            level: Some(loud),
            serial,
            ..crate::game::GameEvent::zeroed()
        });
        g.pl[p].noise = m::max(g.pl[p].noise, loud);
    }
}

/// A new beat of the hold: its clock from nought, nothing chosen, nothing queued, Sam's question dead.   (player.ts setPhase)
pub fn set_phase(g: &mut crate::game::Game, phase: crate::character::HoldPhase) {
    let p = g.player;
    let Some(h) = g.pl[p].holding.as_mut() else { return };
    h.phase = phase;
    h.t = 0.0;
    h.wind_t = -1.0;
    h.fire_queued = false;
    let gd = h.g;
    if let Some(held) = g.gd[gd].held.as_mut() {
        held.phase = phase;
    }
    cancel_ask(g);
}

/// Stop the choke / the whip under way (a Space tap, a step, Esc): the hold goes on and he felt what nearly happened; a choke merely chosen is just un-chosen.   (player.ts easeOffHold)
pub fn ease_off_hold(g: &mut crate::game::Game) {
    use crate::character::HoldPhase as Ph;
    let p = g.player;
    let Some(h) = g.pl[p].holding.as_mut() else { return };
    let gd = h.g;
    if h.phase != Ph::Choke && h.phase != Ph::Whip {
        h.wind_t = -1.0;
        return;
    }
    let was = h.phase;
    set_phase(g, Ph::Held);
    let (talk, time) = (g.gd[gd].talk, g.time);
    g.tab.game_talk[talk].abort_t = time;
    let text = if was == Ph::Whip {
        format!("checked it — the pistol goes back to {}'s head", g.gd[gd].callsign)
    } else {
        format!("eased off — {} is still with you", g.gd[gd].callsign)
    };
    crate::game::game_msg(g, &text);
    let hc = g.gd[gd].ch;
    let at = v3::add(g.ch[hc].pos, [0.0, 1.4, 0.0]);
    seam_audio_play_rate(g, "propScrape", at, 0.1, 2.2);
}

/// Sam's unfinished question dies in his mouth, and a remark waiting its turn with it; an answer the man has begun runs on.   (player.ts cancelAsk)
pub fn cancel_ask(g: &mut crate::game::Game) {
    let p = g.player;
    let time = g.time;
    let Some(h) = g.pl[p].holding.as_mut() else { return };
    h.after = None;
    if h.ask.is_none() {
        return;
    }
    h.ask = None;
    if let Some(b) = g.pl[p].bubble.as_mut() {
        b.dur = m::min(b.dur, time - b.t + 0.2);
    }
}

/// The waiting remark goes up over Sam: his beat of a bubble, the murmur by the man's ear, the log's 'you ▸'. Nobody answers it.   (player.ts sayAfter)
pub fn say_after(g: &mut crate::game::Game) {
    let p = g.player;
    let Some(a) = g.pl[p].holding.as_mut().and_then(|h| h.after.take()) else { return };
    let time = g.time;
    say_over_sam(g, &a.text, time);
}

/// A remark of Sam's up over him: bubble, murmur, log. (sayAfter, and the pistol line in swapHold)
fn say_over_sam(g: &mut Game, text: &str, time: f64) {
    let p = g.player;
    let c = g.pl[p].ch;
    g.pl[p].bubble = Some(crate::game::PlayerBubble { text: String::from(text), t: time, dur: sam_beat(js_len(text)), radio: false });
    let at = v3::add(g.ch[c].pos, [0.0, 1.6, 0.0]);
    crate::audio_iface::game_audio_like_play(g, "murmur", Some(at), Some(0.2), None);
    crate::game::game_note(g, &format!("you ▸ {text}"));
}

/// E with the man already in the arm: the OTHER hold from here, and HOLD.secs.swap of lockout begun.   (player.ts swapHold)
pub fn swap_hold(g: &mut crate::game::Game) {
    use crate::character::HoldVariant as Var;
    let p = g.player;
    let swap = g.k.hold.secs.swap;
    let Some(h) = g.pl[p].holding.as_mut() else { return };
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
    g.pl[p].fire_cd = m::max(g.pl[p].fire_cd, swap);
    cancel_ask(g);
    let callsign = g.gd[gd].callsign.clone();
    if to == Var::Gun {
        g.pl[p].slot = crate::game::SlotKind::Pistol;
        set_holstered(g, false, Some(true));
        let h = hold!(g, p).clone();
        if !h.remarked && held_talk(g, &h).speaking.is_none() && g.gd[gd].civ.is_none() {
            if let Some(line) = crate::dialogue::sam_remark_for(g, gd, None) {
                hold!(g, p).remarked = true;
                let time = g.time;
                say_over_sam(g, &line.text, time);
            }
        }
        crate::game::game_msg(g, &format!("the pistol is at {callsign}'s head. click to fire past him, press Space to pistol-whip him, or E to holster it and question him"));
    } else {
        set_holstered(g, true, Some(true));
        let text = if crate::close::close_bare(g) {
            format!("holstered, with both hands on {callsign} again. press Space to question him, or hold Space for the choke")
        } else {
            format!("holstered, with the knife back at {callsign}'s throat. press Space to question him, or hold Space for the choke")
        };
        crate::game::game_msg(g, &text);
    }
    crate::audio_iface::game_audio_like_play(g, "equip", None, Some(0.45), None);
}

/// HOLD.secs.chokeHold after the choke was chosen: the elbow starts to close.   (player.ts startChoke)
pub fn start_choke(g: &mut crate::game::Game) {
    use crate::character::HoldPhase as Ph;
    let p = g.player;
    let Some(gd) = g.pl[p].holding.as_ref().map(|h| h.g) else { return };
    let hc = g.gd[gd].ch;
    set_phase(g, Ph::Choke);
    let at = v3::add(g.ch[hc].pos, [0.0, 1.4, 0.0]);
    seam_audio_play_rate(g, "bodyHit", at, 0.3, 0.55);
    let pos = v3::copy(g.ch[hc].pos);
    prop_noise(g, pos, CHOKE_NOISE);
}

/// HOLD.secs.chokeHold after the whip was chosen: the pistol comes off his head and cocks back; the crack is whipOut's.   (player.ts startWhip)
pub fn start_whip(g: &mut crate::game::Game) {
    let p = g.player;
    if g.pl[p].holding.is_none() {
        return;
    }
    set_phase(g, crate::character::HoldPhase::Whip);
    hold!(g, p).gun_out = 0.0;
    let c = g.pl[p].ch;
    let at = v3::add(g.ch[c].pos, [0.0, 1.5, 0.0]);
    seam_audio_play_rate(g, "propScrape", at, 0.12, 2.6);
}

/// The knife's clock in the hold.   (player.ts killSecs)
pub fn kill_secs(g: &mut crate::game::Game) -> f64 {
    g.k.hold.secs.kill
}

/// Seconds into the knife's clock at which the blade is in: the hold broken from there on leaves the man dead all the same.   (player.ts killCut)
pub fn kill_cut(g: &mut crate::game::Game) -> f64 {
    g.k.hold.kill.hit * kill_secs(g)
}

/// The knife is going in on the man in his arm: a commit nothing eases off.   (player.ts holdKilling)
pub fn hold_killing(h: &crate::player::Hold) -> bool {
    h.phase == crate::character::HoldPhase::Kill
}

/// The left click with a man in the arm: the knife, committed from this frame; he is dead when its clock runs out.   (player.ts startKill)
pub fn start_kill(g: &mut crate::game::Game) {
    let p = g.player;
    let Some(gd) = g.pl[p].holding.as_ref().map(|h| h.g) else { return };
    let hc = g.gd[gd].ch;
    set_phase(g, crate::character::HoldPhase::Kill);
    let at = v3::add(g.ch[hc].pos, [0.0, 1.3, 0.0]);
    seam_audio_play_rate(g, "bodyHit", at, 0.55, 0.8);
}

/// The man rigid in front, HOLD.dist ahead on Sam's facing, kept out of the statics; what the walls take off his station Sam gives up too.   (player.ts placeHeld)
pub fn place_held(g: &mut crate::game::Game) {
    let p = g.player;
    let c = g.pl[p].ch;
    let Some(gd) = g.pl[p].holding.as_ref().map(|h| h.g) else { return };
    let hc = g.gd[gd].ch;
    let r = g.ch[hc].radius;
    let dist = g.k.hold.dist;
    for _ in 0..3 {
        let f = crate::character::character_forward(g, c);
        let wx = g.ch[c].pos[0] + f[0] * dist;
        let wz = g.ch[c].pos[2] + f[2] * dist;
        let mut hp: V3 = [wx, 0.0, wz];
        crate::scene_collision::collide_circle(g, &mut hp, r, 0.2, 1.5, Some(4.0));
        crate::scene_collision::clamp_to_world(g, &mut hp, r);
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

/// Living guards are as solid to the man in his arm as to Sam: one closer than KEEP_OFF to the held man's centre pushes the PAIR back.   (player.ts keepHeldOffGuards)
pub fn keep_held_off_guards(g: &mut crate::game::Game, dt: f64) {
    let p = g.player;
    let c = g.pl[p].ch;
    let Some(held) = g.pl[p].holding.as_ref().map(|h| h.g) else { return };
    let dist = g.k.hold.dist;
    let f = crate::character::character_forward(g, c);
    let hx = g.ch[c].pos[0] + f[0] * dist;
    let hz = g.ch[c].pos[2] + f[2] * dist;
    let mut i = 0;
    while i < g.guards.len() {
        let o = g.guards[i];
        i += 1;
        if o == held || g.gd[o].state == crate::game::GuardState::Dead || g.gd[o].held.is_some() {
            continue;
        }
        let oc = g.gd[o].ch;
        let dx = hx - g.ch[oc].pos[0];
        let dz = hz - g.ch[oc].pos[2];
        let l = m::hyp2(dx, dz);
        if l >= KEEP_OFF || l < 1e-4 {
            continue;
        }
        if crate::noshove::no_shove_man(g, o) {
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

/// Where the exchange with the man in his arm stands. `speaking` is a COPY of `gd.bubble`: a write goes to `g.gd[h.g].bubble`.   (player.ts heldTalk)
pub fn held_talk(g: &mut crate::game::Game, h: &crate::player::Hold) -> crate::player::HeldTalk {
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
        || match &g.pl[g.player].bubble {
            Some(sb) => sb.t >= h.since - 1e-6 && time - sb.t < sb.dur - 1e-9,
            None => false,
        };
    let ready = !sam_up && (fresh || speaking.is_none() || time + 1e-6 >= last_t + INTERROGATE_GAP);
    let settled = ready
        && match &speaking {
            None => true,
            Some(s) => time + 1e-6 >= s.t + s.dur - ANSWER_TAIL,
        };
    crate::player::HeldTalk { asking, sam_up, ready, settled, speaking }
}

/// A tap on his row, one EXCHANGE: Sam's line first, up for its beat; the answer follows when it ends.   (player.ts askHeld)
pub fn ask_held(g: &mut crate::game::Game) {
    let p = g.player;
    let Some(h) = g.pl[p].holding.clone() else { return };
    let gd = h.g;
    let talk = held_talk(g, &h);
    if talk.sam_up {
        return;
    }
    let time = g.time;
    if !talk.ready {
        if talk.speaking.is_some() {
            if let Some(b) = g.gd[gd].bubble.as_mut() {
                b.dur = m::min(b.dur, time - b.t + 0.3);
            }
        }
        return;
    }
    if talk.speaking.is_some() {
        if let Some(b) = g.gd[gd].bubble.as_mut() {
            b.dur = m::min(b.dur, time - b.t);
        }
    }
    let text = if g.gd[gd].civ.is_some() {
        crate::civilians::civilian_sam_line(g, gd).text
    } else {
        crate::dialogue::sam_line_for(g, gd, h.variant, None).text
    };
    let dur = sam_beat(js_len(&text));
    let time = g.time;
    g.pl[p].bubble = Some(crate::game::PlayerBubble { text: text.clone(), t: time, dur, radio: false });
    {
        let h = hold!(g, p);
        h.ask = Some(crate::player::HoldAsk { t: time, dur });
        h.after = None;
    }
    let c = g.pl[p].ch;
    let at = v3::add(g.ch[c].pos, [0.0, 1.6, 0.0]);
    crate::audio_iface::game_audio_like_play(g, "murmur", Some(at), Some(0.2), None);
    crate::game::game_note(g, &format!("you ▸ {text}"));
}

/// Sam's line has hung its beat: the man answers it, and the muffled sound of it under the arm.   (player.ts answerHeld)
pub fn answer_held(g: &mut crate::game::Game) {
    let p = g.player;
    let Some(gd) = g.pl[p].holding.as_ref().map(|h| h.g) else { return };
    let pick = crate::game::game_interrogate(g, gd, None);
    if pick.is_some() {
        let hc = g.gd[gd].ch;
        let at = v3::add(g.ch[hc].pos, [0.0, 1.5, 0.0]);
        crate::audio_iface::game_audio_like_play(g, "heldProtest", Some(at), Some(0.16), None);
    }
}

/// The man has given the keyed door up: Sam takes what is on his belt; with the man in his arm, Sam's word on it follows the man's line down.   (player.ts takeKey)
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
    seam_audio_play_rate(g, "lockPick", at, 0.45, 1.6);
    crate::mission::tally(g, crate::mission::TallyK::Keys);
    crate::chronicle::chron_key(g, gd);
    let text = format!("you took {}'s key. keyed doors will open for you now", g.gd[gd].callsign);
    crate::game::game_msg(g, &text);
    let variant = match &g.pl[p].holding {
        Some(h) if h.g == gd => Some(h.variant),
        _ => None,
    };
    if let Some(variant) = variant {
        if let Some(line) = crate::dialogue::sam_key_line_for(g, gd, variant, rnd) {
            if let Some(h) = g.pl[p].holding.as_mut() {
                h.after = Some(crate::player::HoldAfter { text: line.text, id: line.id });
            }
        }
    }
}

/// After the door leaves have had their say: whatever a leaf shoved the held man off his station by, the PAIR gives. Not during the reach.   (player.ts settleHeld)
pub fn settle_held(g: &mut crate::game::Game) {
    let p = g.player;
    let gd = match &g.pl[p].holding {
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

/// The choke run through HOLD.secs.choke: he goes limp in the arm and folds forward at Sam's feet, out cold.   (player.ts chokeOut)
pub fn choke_out(g: &mut crate::game::Game) {
    let p = g.player;
    let Some(gd) = g.pl[p].holding.as_ref().map(|h| h.g) else { return };
    let c = g.pl[p].ch;
    let hc = g.gd[gd].ch;
    let f = crate::character::character_forward(g, c);
    unhook(g, gd);
    let at = v3::add(g.ch[hc].pos, [0.0, 1.4, 0.0]);
    crate::audio_iface::game_audio_like_play(g, "chokeLimp", Some(at), Some(0.35), None);
    crate::takedown::take_fall(g, gd, None, false, Some(crate::takedown::TakeFallHow::Choke));
    crate::combat::kill_guard(g, gd, f, Some(true), Some(crate::game::DownKind::Choked), Some(crate::game::DownBy::Player), None, None);
    let pos = v3::copy(g.ch[hc].pos);
    prop_noise(g, pos, FOLD_NOISE);
    g.pl[p].holding = None;
}

/// The knife run through its clock: he is dead in the arm and let fall at Sam's feet.   (player.ts killOut)
pub fn kill_out(g: &mut crate::game::Game) {
    let p = g.player;
    let Some(gd) = g.pl[p].holding.as_ref().map(|h| h.g) else { return };
    let c = g.pl[p].ch;
    let hc = g.gd[gd].ch;
    let f = crate::character::character_forward(g, c);
    unhook(g, gd);
    let at = v3::add(g.ch[hc].pos, [0.0, 1.4, 0.0]);
    crate::audio_iface::game_audio_like_play(g, "chokeLimp", Some(at), Some(0.35), None);
    crate::takedown::take_fall(g, gd, None, true, Some(crate::takedown::TakeFallHow::Kill));
    crate::combat::kill_guard(g, gd, f, Some(true), Some(crate::game::DownKind::Shot), Some(crate::game::DownBy::Player), Some("knife — dead"), None);
    // startTakedown's own rewrite for the knife: a man who watched saw him DROPPED, not shot
    let time = g.time;
    let callsign = g.gd[gd].callsign.clone();
    let mut i = 0;
    while i < g.guards.len() {
        let o = g.guards[i];
        i += 1;
        if let Some(w) = g.gd[o].witness.saw_act.as_mut() {
            if w.kind == crate::game::WitnessSawActKind::Shot && w.t == time && w.victim == callsign {
                w.kind = crate::game::WitnessSawActKind::Struck;
            }
        }
    }
    let pos = v3::copy(g.ch[hc].pos);
    prop_noise(g, pos, FOLD_NOISE);
    g.pl[p].holding = None;
}

/// The whip run through HOLD.secs.whip: the pistol lands behind his right ear and he drops out of the arm, the loudest thing a hold does.   (player.ts whipOut)
pub fn whip_out(g: &mut crate::game::Game) {
    let p = g.player;
    let Some(gd) = g.pl[p].holding.as_ref().map(|h| h.g) else { return };
    let c = g.pl[p].ch;
    let hc = g.gd[gd].ch;
    let f = crate::character::character_forward(g, c);
    let dir = v3::normalize([f[0] + f[2] * 0.45, 0.0, f[2] - f[0] * 0.45]);
    let head = match g.ch[hc].bones.get(crate::character::Bone::Head) {
        Some(h) => *h,
        None => v3::add(g.ch[hc].pos, [0.0, 1.6, 0.0]),
    };
    unhook(g, gd);
    crate::takedown::take_fall(g, gd, None, false, Some(crate::takedown::TakeFallHow::Whip));
    crate::combat::kill_guard(g, gd, dir, Some(true), Some(crate::game::DownKind::Struck), Some(crate::game::DownBy::Player), Some("pistol-whipped — out cold, and that was not quiet"), None);
    crate::audio_iface::game_audio_like_play(g, "whipThud", Some(head), Some(0.7), None);
    let pos = v3::copy(g.ch[hc].pos);
    prop_noise(g, pos, WHIP_NOISE);
    g.pl[p].holding = None;
    g.pl[p].fire_cd = m::max(g.pl[p].fire_cd, 0.5);
}

/// He comes out of the arm his own man and an instant witness; a man already down is only unhooked.   (player.ts handBack)
pub fn hand_back(g: &mut crate::game::Game, gd: GuardId, stagger: f64) {
    let mut stagger = stagger;
    let hc = g.gd[gd].ch;
    unhook(g, gd);
    g.gd[gd].speed = 0.0;
    g.ch[hc].vel = [0.0, 0.0, 0.0];
    g.gd[gd].path = Vec::new();
    g.gd[gd].path_serial = g.gd[gd].path_serial.wrapping_add(1);
    g.gd[gd].path_goal = None;
    g.gd[gd].path_goal_orbit = false;
    g.gd[gd].body_duty = None;
    g.gd[gd].stall_ref = None;
    g.ch[hc].aim_pitch = 0.0;
    if g.gd[gd].state == crate::game::GuardState::Dead {
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
        let m_ = &mut g.gd[gd];
        m_.awareness = 1.0;
        m_.last_known = Some(v3::copy(pp));
        m_.fix_stamp = m_.fix_stamp.wrapping_add(1);
        m_.last_seen_t = time;
        m_.sight_pos = v3::copy(pp);
        m_.sight_vel = [0.0, 0.0, 0.0];
        m_.sight_t = time;
        m_.led_t = time;
        m_.react_t = m::max(m_.react_t, stagger);
        m_.heard_up_to = seq;
        m_.missed_at = -1.0;
        m_.missing_raised = false;
    }
    crate::guards::reset_chase(g, gd);
    g.gd[gd].search_t = 0.0;
    crate::character::anim_hit(g, hc);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_line_is_measured_as_javascript_measures_it() {
        assert_eq!(js_len("you ▸ — é"), 9.0);
        assert_eq!(js_len("😀"), 2.0);
    }

    #[test]
    fn the_knife_is_only_the_kill_phase() {
        let mut h = Hold::zeroed();
        h.phase = crate::character::HoldPhase::Kill;
        assert!(hold_killing(&h));
        h.phase = crate::character::HoldPhase::Choke;
        assert!(!hold_killing(&h));
    }
}
