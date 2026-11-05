use super::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

use crate::character::{Bone, DoorPush};
use crate::doors::{door_angle, door_dir, door_last_who, door_max_a, door_min_a, door_off, door_pos, door_pull_t, door_swing};
use crate::doors::{doors_boot_holds, doors_crack, doors_has_closer, doors_locked_out, doors_rattle, doors_side, doors_use};
use crate::game::{GameDoorHandRecsValue, GameSetBlackoutCause, GuardState, Interactable, InteractableKind, Verb};

/// (doors.ts:1183 Doors.USE_REACH) a static of the class the skeleton has no entry for
const DOORS_USE_REACH: f64 = 1.35; // SKELETON-GAP: crate::doors::DOORS_USE_REACH (doors.ts:1183)
fn seam_use_reach() -> f64 { DOORS_USE_REACH }

/// the row a click with empty hands is on: the first guard in reach who is no hostage and nobody the snatch has out (game.ts:2192)
fn click_row(g: &mut Game) -> Option<Interactable> {
    let mut i = 0;
    while i < g.nearby.len() {
        if g.nearby[i].kind == InteractableKind::Guard && g.nearby[i].in_reach {
            let taken = match g.nearby[i].guard {
                Some(gd) => crate::rescue::rescue_is_hostage(g, gd) || crate::snatchout::snatch_out_owns(g, gd),
                None => false,
            };
            if !taken { return Some(g.nearby[i].clone()); }
        }
        i += 1;
    }
    None
}

/// the way in is read where he stands: Hold.from, off his body now (game.ts:2197)
fn hold_from_here(g: &mut Game, pl: PlayerId) {
    let c = g.pl[pl].ch;
    let p = g.ch[c].pos;
    if let Some(h) = g.pl[pl].holding.as_mut() { h.from = Some([p[0], p[1], p[2]]); }
}

/// whether the man in his arm is this one: pl.holding?.g === gd
fn holds(g: &Game, pl: PlayerId, gd: GuardId) -> bool {
    match g.pl[pl].holding.as_ref() { Some(h) => h.g == gd, None => false }
}

/// this frame's verb of his, done (game.ts Game.doorActs)
pub fn game_door_acts(g: &mut crate::game::Game, dt: f64, input: InputId) {
    let pl = g.player;
    let pc = g.pl[pl].ch;
    let act: Option<Interactable> = if g.pl[pl].down { None } else { g.act.clone() };
    let verb: Option<Verb> = act.as_ref().and_then(|a| a.verb);
    let kind: Option<InteractableKind> = act.as_ref().map(|a| a.kind);
    let act_guard: Option<GuardId> = act.as_ref().and_then(|a| a.guard);
    let act_door: Option<DoorId> = act.as_ref().and_then(|a| a.door);
    let act_close: Option<crate::close::CloseKind> = act.as_ref().and_then(|a| a.close);

    if kind == Some(InteractableKind::Body) && act_guard.is_some() {
        if verb == Some(Verb::Drop) {
            if g.pl[pl].dragging.is_some() && !crate::hands::haul_set_down(g) { crate::player::toggle_drag(g, None); }
        } else if verb == Some(Verb::Drag) {
            crate::player::toggle_drag(g, act_guard);
        }
    }
    if kind == Some(InteractableKind::Breaker) && !crate::handwork::breaker_start(g, verb == Some(Verb::BreakerOn)) {
        if verb == Some(Verb::BreakerOn) && crate::sim_lightshome::mains_active(g) && !crate::sim_lightshome::mains_permanent(g) {
            crate::game::game_end_blackout(g);
        } else if verb == Some(Verb::BreakerOff) && !crate::sim_lightshome::mains_active(g) {
            crate::game::game_set_blackout(g, 86400.0, GameSetBlackoutCause::Hand);
        }
    }
    crate::handwork::breaker_tick(g, dt);
    if kind == Some(InteractableKind::Cam) && verb == Some(Verb::TakeCam) { crate::stickycam::take_cam(g, pl); }
    if kind == Some(InteractableKind::Charge) { crate::ownblow::own_blow_act(g); }
    if kind == Some(InteractableKind::Switch) {
        if let Some(v) = verb { crate::lightswitch::switch_act(g, pl, v); }
    }

    // the grab and the takedown, with one gate
    {
        let free = g.pl[pl].takedown.is_none() && !g.pl[pl].down && g.pl[pl].holding.is_none() && !crate::samstun::sam_staggered(g, None) && g.pl[pl].breaker_move.is_none();
        let bare = crate::close::close_bare(g);
        let two = crate::close::close_lethal(g) && !bare;
        let knife = !bare && crate::input::input_lmb_hit(g, input) && !crate::stalk::stalk_has(g);
        let in_reach: Option<Interactable> = if g.pl[pl].holstered { click_row(g) } else { None };
        let reach_guard: Option<GuardId> = in_reach.as_ref().and_then(|r| r.guard);
        let choke_on = match reach_guard {
            Some(gd) if g.gd[gd].civ.is_none() && crate::rails::rails_offers(g, "grab") => {
                let p = g.ch[pc].pos;
                crate::close::close_at(g, gd, &p).grab
            }
            _ => false,
        };
        let rmb_on = (two || bare) && g.pl[pl].holstered && free;
        let rmb_down = crate::input::input_rmb(g, input);
        let rmb_hit = crate::input::input_rmb_hit(g, input);
        let dropped = crate::input::input_hit(g, input, "Blur") || crate::input::input_hit(g, input, "Paused");
        let rb = crate::close::close_rmb(g, rmb_on, rmb_down, rmb_hit, dt, in_reach.is_some(), choke_on, dropped);
        let blow = knife || ((two || bare) && rb.is_some());
        let clicked: Option<Interactable> = if blow { in_reach } else { None };
        let via_click: Option<GuardId> = clicked.as_ref().and_then(|c| c.guard);
        let clicked_close = clicked.as_ref().and_then(|c| c.close);
        let stalked = crate::stalk::stalk_grab(g);
        if let (Some(st), true) = (stalked, free) {
            crate::player::start_grab(g, st, None);
            if holds(g, pl, st) { hold_from_here(g, pl); }
        } else if let (Some(vc), true, Some(crate::close::CloseRmb::Hold)) = (via_click, free, rb) {
            let keyed = crate::stalk::stalk_key_can(g, vc);
            crate::player::start_grab(g, vc, clicked_close);
            if holds(g, pl, vc) {
                if keyed {
                    crate::stalk::stalk_key_took(g);
                    hold_from_here(g, pl);
                }
                let tap = g.k.close.rmb_tap_secs;
                if let Some(h) = g.pl[pl].holding.as_mut() {
                    h.rmb_choke = Some(1.0);
                    h.rmb_t = Some(tap);
                }
            }
        } else if let (Some(vc), true) = (via_click, free) {
            let lethal = two && knife && g.gd[vc].civ.is_none();
            let loud = !knife && rb == Some(crate::close::CloseRmb::Tap) && choke_on;
            crate::player::start_takedown(g, vc, clicked_close, Some(lethal), Some(loud));
        } else if let (Some(InteractableKind::Guard), Some(Verb::Grab), Some(ag), true) = (kind, verb, act_guard, free) {
            let keyed = crate::stalk::stalk_key_can(g, ag);
            crate::player::start_grab(g, ag, act_close);
            if keyed && holds(g, pl, ag) {
                crate::stalk::stalk_key_took(g);
                hold_from_here(g, pl);
            }
        }
    }

    // a door in reach, one row per thing it can be made to do
    if let (Some(InteractableKind::Door), Some(d)) = (kind, act_door) {
        let pos = g.ch[pc].pos;
        let id = g.ch[pc].id;
        let keys = g.pl[pl].keys.clone();
        match verb {
            Some(Verb::Push) | Some(Verb::Pull) | Some(Verb::Unlock) => {
                if !crate::hands::door_move_begin(g, d) {
                    g.pl[pl].door_move = None;
                    let pos = g.ch[pc].pos;
                    let keys = g.pl[pl].keys.clone();
                    doors_use(g, d, pos, id, Some(&keys));
                }
            }
            Some(Verb::Ease) => {
                if !doors_locked_out(g, d, pos, Some(&keys)) {
                    g.pl[pl].door_move = None;
                    g.pl[pl].easing = Some(d);
                    g.pl[pl].ease_stall_t = 0.0;
                    let a = door_angle(g, d);
                    g.pl[pl].ease_best = m::abs(a);
                }
            }
            Some(Verb::Pick) => {
                if doors_locked_out(g, d, pos, Some(&keys)) { crate::player::start_picking(g, d); }
            }
            Some(Verb::Kick) => {
                if doors_boot_holds(g, d, pos) {
                    crate::game::game_msg(g, "it opens toward you, so a kick won't move it");
                } else {
                    crate::player::start_kick(g, d, None, None);
                }
            }
            Some(Verb::Rattle) => {
                doors_rattle(g, d, id, None);
                let steel = g.dr[d].def.steel.unwrap_or(false);
                crate::game::game_msg(g, if steel { "the stair door is bolted from the landing side with no keyhole on this side, so it's not your way out tonight" } else { "it's locked. pick it (quiet, slow) or kick it in (loud)" });
            }
            Some(Verb::Code) => { crate::devices::start_coding(g, d); }
            Some(Verb::Nocode) => crate::devices::deny_pad(g, d),
            Some(Verb::Breach) | Some(Verb::Hammer) => crate::bigkit::sam_big_start(g, d),
            Some(Verb::Wand) => crate::player::start_wand(g, d),
            _ => {}
        }
    }

    // the leaf he is easing: on round at the crack rate while he stays within reach of it
    if let Some(d) = g.pl[pl].easing {
        let pos = g.ch[pc].pos;
        let dp = door_pos(g, d);
        let in_reach = m::min(v3::dist_xz(dp, pos), v3::dist_xz(g.dr[d].frame_centre, pos)) < seam_use_reach() + 0.25
            && !g.pl[pl].down && g.pl[pl].holding.is_none() && g.pl[pl].takedown.is_none() && g.pl[pl].kick.is_none();
        let keys = g.pl[pl].keys.clone();
        if !in_reach || doors_locked_out(g, d, pos, Some(&keys)) {
            g.pl[pl].easing = None;
        } else {
            let id = g.ch[pc].id;
            doors_crack(g, d, pos, id, m::min(dt, 0.05), Some(&keys), Some(crate::player::EASE_RATE), Some(crate::player::EASE_MAX));
            let angle = door_angle(g, d);
            let a1 = m::abs(angle);
            let stop = if angle < 0.0 { -door_min_a(g, d) } else { door_max_a(g, d) };
            if a1 >= m::min(crate::player::EASE_MAX, stop) - 1e-3 {
                g.pl[pl].easing = None;
            } else if a1 <= g.pl[pl].ease_best + 1e-5 {
                g.pl[pl].ease_stall_t += dt;
                if g.pl[pl].ease_stall_t > 0.5 {
                    g.pl[pl].easing = None;
                    crate::game::game_msg(g, "it won't go any further");
                }
            } else {
                g.pl[pl].ease_stall_t = 0.0;
                g.pl[pl].ease_best = a1;
            }
        }
    }
    if g.pl[pl].door_move.is_some() { crate::hands::door_move_tick(g, dt); }
}

/// what the leaves did to him once they have swung (game.ts Game.doorAfter)
pub fn game_door_after(g: &mut crate::game::Game, own: Option<&mut crate::doors::DoorContact>, bx: f64, bz: f64) {
    let own: Option<&crate::doors::DoorContact> = own.as_deref();
    let pl = g.player;
    let pc = g.pl[pl].ch;
    if own.is_some() {
        crate::player::settle_player(g);
        crate::player::settle_held(g);
        let p = g.ch[pc].pos;
        if m::abs(p[0] - bx) + m::abs(p[2] - bz) > 0.005 { crate::character::character_update(g, pc, 0.0); }
    }
    if g.pl[pl].door_head != 0.0 && g.pl[pl].door_head_t != g.time {
        if g.ch[pc].anim.look_yaw_extra == g.pl[pl].door_head { g.ch[pc].anim.look_yaw_extra = 0.0; }
        g.pl[pl].door_head = 0.0;
    }
    if let Some(o) = own {
        if o.run.is_some() { crate::sprintdoor::sprint_door_after(g, pl, o); }
    }
    if own.is_some() {
        if let Some(kd) = crate::sprintdoor::sprint_door_kick(g, pl) { crate::player::start_kick(g, kd, Some(true), None); }
    }
    if let Some(d) = own.and_then(|o| o.blocked_by) {
        let p = &g.pl[pl];
        if p.sprinting && p.kick.is_none() && p.picking.is_none() && p.coding.is_none() && p.takedown.is_none() && p.holding.is_none() && g.puppet.is_none() {
            let u = door_dir(g, d);
            let (pos, vel) = (g.ch[pc].pos, g.ch[pc].vel);
            let into = -(-u[1] * vel[0] + u[0] * vel[2]) * doors_side(g, d, pos);
            if into > 1.5 && !doors_boot_holds(g, d, pos) { crate::player::start_kick(g, d, None, None); }
        }
    }
}

/// one row of the walk over who has hands (game.ts:2269)
#[derive(Clone, Copy)]
struct WhoRow { c: CharId, can: bool, right_free: bool, gd: Option<GuardId>, pl: Option<PlayerId> }

/// `before[k]`, with the language's answer past the end (undefined: a NaN by the time it is subtracted)
fn was_at(before: &[f64], k: Option<usize>) -> f64 {
    match k { Some(i) if i < before.len() => before[i], _ => m::NAN }
}

/// what spot() closes over: the leaf a frame on, his side of it, his shoulders (game.ts:2316-2322)
#[derive(Clone, Copy)]
struct Leaf { u: [f64; 2], fx: f64, fz: f64, lo: f64, hi: f64, crack: bool, chest: V3, rgt: [f64; 2], arm_l: Option<V3>, arm_r: Option<V3> }

/// where one hand goes on the leaf
#[derive(Clone, Copy, Debug)]
struct Spot { p: V3, reach: bool, dist: f64 }

/// the spot for one hand (`right`: the right), off that shoulder joint (game.ts:2329 spot)
fn spot(l: &Leaf, right: bool) -> Spot {
    let (u, rgt, chest, fx, fz, lo, hi) = (l.u, l.rgt, l.chest, l.fx, l.fz, l.lo, l.hi);
    let side = if right { 0.2 } else { -0.2 };
    let sh = match if right { l.arm_r } else { l.arm_l } {
        Some(s) => s,
        None => [chest[0] + rgt[0] * side, chest[1] + 0.15, chest[2] + rgt[1] * side],
    };
    let ex = fx - sh[0];
    let ez = fz - sh[2];
    let b = u[0] * ex + u[1] * ez;
    let dh = m::hyp2(ex - u[0] * b, ez - u[1] * b);
    let y = v3::clamp(sh[1] - (if l.crack { v3::lerp(0.22, 0.1, v3::smoothstep(0.32, 0.5, dh)) } else { 0.1 }), 0.85, 1.36);
    let inb = if right { -0.05 } else { 0.05 };
    let mut s = v3::clamp((sh[0] + rgt[0] * inb - fx) * u[0] + (sh[2] + rgt[1] * inb - fz) * u[1] + (if l.crack { 0.08 } else { 0.0 }), lo, hi);
    let mut reach = false;
    let rp = 0.5;
    let dy = y - sh[1];
    let r2 = rp * rp - dy * dy - dh * dh;
    if r2 > 0.0 {
        let half = m::sqrt(r2) - 0.03;
        let s0 = -b - half;
        let s1 = -b + half;
        if half > 0.0 && s1 >= lo && s0 <= hi {
            s = v3::clamp(s, m::max(s0, lo), m::min(s1, hi));
            reach = true;
        }
    }
    if !reach { s = v3::clamp(-b, lo, hi); }
    let m2 = 0.33 * 0.33 - dy * dy - dh * dh;
    if m2 > 0.0 {
        let q = m::sqrt(m2);
        if m::abs(s + b) < q {
            s = if -b + q <= hi { -b + q } else if -b - q >= lo { -b - q } else { s };
        }
    }
    let p: V3 = [fx + u[0] * s, y, fz + u[1] * s];
    Spot { p, reach, dist: m::hyp3(p[0] - sh[0], p[1] - sh[1], p[2] - sh[2]) }
}

/// who is pushing which leaf open right now, and where on it his free hand goes (game.ts Game.handsOnDoors)
pub fn game_hands_on_doors(g: &mut crate::game::Game, before: &[f64]) {
    const FOLLOW: f64 = 0.12;
    const FADE: f64 = 0.4;
    let now = g.time;
    let list: Vec<DoorId> = g.doors.list.clone();
    let mut who: Vec<WhoRow> = Vec::new();
    for s in g.seats.clone() {
        let pl = crate::cmd::seat_man(g, s);
        let p = &g.pl[pl];
        who.push(WhoRow {
            c: p.ch,
            pl: Some(pl),
            gd: None,
            can: !p.down && p.dragging.is_none() && p.holding.is_none() && p.kick.is_none() && p.picking.is_none() && p.wanding.is_none() && p.takedown.is_none(),
            right_free: p.holstered,
        });
    }
    for i in 0..g.guards.len() {
        let gd = g.guards[i];
        let m_ = &g.gd[gd];
        who.push(WhoRow {
            c: m_.ch,
            gd: Some(gd),
            pl: None,
            can: m_.state != GuardState::Dead && m_.held.is_none(),
            right_free: m_.state == GuardState::Patrol && !m_.drawn && m_.dropped.is_some(),
        });
    }
    for cv in crate::civilians::civilians_shown(g) {
        let m_ = &g.gd[cv];
        let c = m_.ch;
        who.push(WhoRow { c, gd: None, pl: None, can: m_.state != GuardState::Dead && m_.held.is_none() && g.ch[c].anim.act.is_none(), right_free: true });
    }
    for c in g.door_hand_recs.keys() {
        if !who.iter().any(|w| w.c == c) { g.door_hand_recs.delete(c); }
    }
    for w in who.iter().copied() {
        let WhoRow { c, can, right_free, gd, pl } = w;
        let player = pl.is_some();
        let cpos = g.ch[c].pos;
        let cvel = g.ch[c].vel;
        let cid = g.ch[c].id;
        // a copy of the row, written back wherever it changes
        let mut rec: Option<GameDoorHandRecsValue> = g.door_hand_recs.get(c).copied();
        if can {
            let mut hit: Option<DoorId> = None;
            let (mut crack, mut hold_only, mut move_) = (false, false, false);
            let speed = m::hyp2(cvel[0], cvel[2]);
            if let Some(p) = pl {
                if let Some(dd) = g.pl[p].door_move.as_ref().map(|dm| dm.d) {
                    if !door_off(g, dd) { hit = Some(dd); crack = true; move_ = true; }
                }
            }
            for k in 0..list.len() {
                if hit.is_some() { break; }
                let d = list[k];
                let hx = g.dr[d].def.hinge[0];
                let hz = g.dr[d].def.hinge[1];
                let wd = g.dr[d].def.width;
                if door_off(g, d) { continue; }
                let vx = cpos[0] - hx;
                let vz = cpos[2] - hz;
                if vx * vx + vz * vz > m::pow(wd + 1.2, 2.0) { continue; }
                if let Some(p) = pl {
                    if g.pl[p].easing == Some(d) { hit = Some(d); crack = true; break; }
                }
                let u = door_dir(g, d);
                let along = vx * u[0] + vz * u[1];
                let perp = u[0] * vz - u[1] * vx;
                let t = v3::clamp(along, 0.0, wd);
                let dist = m::hyp2(vx - u[0] * t, vz - u[1] * t);
                let thick = g.dr[d].thick;
                let touching = along > 0.14 && along < wd + 0.3 && dist < 0.34 + thick / 2.0 + 0.05;
                let sgn = if perp >= 0.0 { -1.0 } else { 1.0 };
                let angle = door_angle(g, d);
                let da = angle - was_at(before, Some(k));
                let holding = touching && doors_has_closer(g, d) && m::abs(angle) > 0.1 && m::sign(angle) == sgn && speed < 0.4;
                let pushing = touching && da * sgn > 1e-4;
                if pushing || holding { hit = Some(d); hold_only = !pushing; break; }
                let swing = door_swing(g, d);
                let mine = sgn * swing < 0.0 && angle * swing < 1.35;
                if mine && door_pull_t(g, d) > 0.0 && door_last_who(g, d) == cid && dist < 0.9 { hit = Some(d); break; }
                if !touching && speed > 0.3 {
                    let wx = cvel[0] / speed;
                    let wz = cvel[2] / speed;
                    let closing = -(wx * -u[1] + wz * u[0]) * m::sign(perp);
                    if closing > 0.4 {
                        let t_hit = (m::abs(perp) - 0.34 - thick / 2.0) / (closing * speed);
                        let ah = along + (wx * u[0] + wz * u[1]) * speed * m::max(0.0, t_hit);
                        let at_stop = if sgn < 0.0 { angle <= door_min_a(g, d) + 0.02 } else { angle >= door_max_a(g, d) - 0.02 };
                        if t_hit >= 0.0 && t_hit < 0.4 && ah > 0.2 && ah < wd + 0.1 && (!at_stop || mine) {
                            let locked = match pl {
                                Some(p) => { let keys = g.pl[p].keys.clone(); doors_locked_out(g, d, cpos, Some(&keys)) }
                                None => false,
                            };
                            if !locked { hit = Some(d); }
                        }
                    }
                }
            }
            if let (None, Some(gd)) = (hit, gd) {
                if let Some(th) = crate::hands::team_door_hand(g, gd) { hit = Some(th); crack = true; move_ = true; }
            }
            if let Some(h) = hit {
                let r = match rec {
                    Some(mut r) => {
                        if r.door != h { r.hand = -1.0; r.t0 = now; }
                        r.door = h;
                        r.t = now;
                        r.crack = crack;
                        r.hold_only = Some(hold_only);
                        r.move_ = Some(move_);
                        r
                    }
                    None => GameDoorHandRecsValue { door: h, t: now, t0: now, crack, hand: -1.0, hold_only: Some(hold_only), move_: Some(move_) },
                };
                g.door_hand_recs.set(c, r);
                rec = Some(r);
            }
        } else if rec.is_some() {
            g.door_hand_recs.delete(c);
            rec = None;
        }
        if let Some(r) = rec {
            let h = g.dr[r.door].def.hinge;
            if m::hyp2(cpos[0] - h[0], cpos[2] - h[1]) > g.dr[r.door].def.width + 1.2 {
                g.door_hand_recs.delete(c);
                rec = None;
            }
        }
        let gone = match rec { None => true, Some(r) => now - r.t > FOLLOW + FADE };
        if gone {
            if rec.is_some() { g.door_hand_recs.delete(c); }
            let wh = if can { crate::workthings::work_hand(g, c) } else { None };
            let wj = if wh.is_some() { crate::workthings::work_job(g, c) } else { None };
            let worked = match wh.as_ref() {
                Some(h) => {
                    let hand = crate::handwork::TeamWorkHandH { p: h.p, n: h.n, w: h.w };
                    let verb = wj.as_ref().map(|j| j.verb);
                    let kind = if verb == Some(crate::workthings::WorkVerb::Drive) || verb == Some(crate::workthings::WorkVerb::Defuse) { crate::handwork::TeamWorkHandKind::Take } else { crate::handwork::TeamWorkHandKind::Lever };
                    let secs = wj.as_ref().map_or(0.0, |j| j.secs);
                    let need = wj.as_ref().map_or(1.0, |j| j.need);
                    let dir = if verb == Some(crate::workthings::WorkVerb::BreakerOn) { -1.0 } else { 1.0 };
                    crate::handwork::team_work_hand(g, c, Some(hand), Some(kind), Some(secs), Some(need), Some(dir))
                }
                None => false,
            };
            g.ch[c].anim.door_push = if worked { None } else { wh };
            continue;
        }
        let mut r = rec.unwrap();
        let d = r.door;
        let hinge = g.dr[d].def.hinge;
        let (hx, hz) = (hinge[0], hinge[1]);
        let wd = g.dr[d].def.width;
        let k = list.iter().position(|x| *x == d);
        let angle = door_angle(g, d);
        let da = v3::clamp(angle - was_at(before, k), -0.3, 0.3);
        let (min_a, max_a) = (door_min_a(g, d), door_max_a(g, d));
        let a = g.dr[d].def.closed_dir + v3::clamp(angle + da, min_a, max_a);
        let u: [f64; 2] = [m::cos(a), m::sin(a)];
        let px = cpos[0] - hx;
        let pz = cpos[2] - hz;
        let face = if u[0] * pz - u[1] * px >= 0.0 { 1.0 } else { -1.0 };
        let nx = -u[1] * face;
        let nz = u[0] * face;
        let yaw = g.ch[c].body_yaw;
        let rgt: [f64; 2] = [-m::cos(yaw), m::sin(yaw)];
        let chest: V3 = match g.ch[c].bones.get(Bone::Chest) { Some(b) => *b, None => [cpos[0], cpos[1] + 1.25, cpos[2]] };
        let off = g.dr[d].thick / 2.0;
        let leaf = Leaf {
            u,
            fx: hx + nx * off,
            fz: hz + nz * off,
            lo: 0.3,
            hi: wd - (if r.crack { 0.1 } else { 0.2 }),
            crack: r.crack,
            chest,
            rgt,
            arm_l: g.ch[c].bones.get(Bone::UpperArmL).copied(),
            arm_r: g.ch[c].bones.get(Bone::UpperArmR).copied(),
        };
        let mut right: bool;
        let mut sp: Spot;
        if r.hand == -1.0 || (r.hand == 1.0 && !right_free) {
            right = right_free && u[0] * rgt[0] + u[1] * rgt[1] > 0.0;
            sp = spot(&leaf, right);
            if !sp.reach && (right || right_free) {
                let so = spot(&leaf, !right);
                if so.reach || so.dist < sp.dist - 0.12 { right = !right; sp = so; }
            }
            let sx = sp.p[0] - cpos[0];
            let sz = sp.p[2] - cpos[2];
            let sl = js::or(m::hyp2(sx, sz), 1.0);
            let settled = if r.move_.unwrap_or(false) {
                (sx * m::sin(yaw) + sz * m::cos(yaw)) / sl > 0.5
            } else {
                -nx * m::sin(yaw) - nz * m::cos(yaw) > 0.5 && now - r.t0 > 0.25
            };
            if settled {
                r.hand = if right { 1.0 } else { 0.0 };
                g.door_hand_recs.set(c, r);
            }
        } else {
            right = r.hand == 1.0;
            sp = spot(&leaf, right);
        }
        g.ch[c].anim.door_push = Some(DoorPush {
            p: sp.p,
            n: [nx, 0.0, nz],
            w: if now - r.t <= FOLLOW { 1.0 } else { 0.0 },
            hand: if r.hand == -1.0 { -1.0 } else if right { 1.0 } else { 0.0 },
            hold_only: r.hold_only,
            reach: r.move_,
        });
    }
}

/// loose furniture: the same contact pattern as the doors, after everyone has moved (game.ts Game.updateProps)
pub fn game_update_props(g: &mut crate::game::Game, dt: f64) {
    let seats: Vec<SeatId> = g.seats.clone();
    let mut chars: Vec<CharId> = Vec::new();
    for s in seats.iter().copied() {
        let pl = crate::cmd::seat_man(g, s);
        if !g.pl[pl].down { chars.push(g.pl[pl].ch); }
    }
    for i in 0..g.guards.len() {
        let gd = g.guards[i];
        if g.gd[gd].state != GuardState::Dead && !crate::doze::doze_in_chair(g, gd) { chars.push(g.gd[gd].ch); }
    }
    for cv in crate::civilians::civilians_shown(g) {
        if g.gd[cv].state != GuardState::Dead && !crate::civilians::civilian_seated(g, cv) && crate::civilians::civilian_pushes_props(g, cv) { chars.push(g.gd[cv].ch); }
    }
    let mut contacts: Vec<crate::props::PropContact> = Vec::with_capacity(chars.len());
    for c in chars.iter().copied() {
        let man = crate::seats::man_of(g, Some(c));
        let quiet = crate::game::quiet_man(g, man);
        contacts.push(crate::props::PropContact { pos: g.ch[c].pos, vel: g.ch[c].vel, radius: 0.3, who: g.ch[c].id, mass: 75.0, quiet });
    }
    let before: Vec<[f64; 2]> = chars.iter().map(|c| [g.ch[*c].pos[0], g.ch[*c].pos[2]]).collect();
    let list: Vec<DoorId> = g.doors.list.clone();
    let mut any_off = false;
    for d in list.iter().copied() {
        if door_off(g, d) { any_off = true; break; }
    }
    let leaves: Vec<BoxId> = if any_off {
        let mut l = Vec::new();
        for d in list.iter().copied() {
            if !door_off(g, d) { l.push(g.dr[d].box_); }
        }
        l
    } else {
        g.col.dynamic_boxes.clone()
    };
    crate::props::prop_system_update(g, dt, &mut contacts, &leaves);
    // a contact's pos is the man's own in the TypeScript: what the step moved him by is his (props.ts takeBack writes x and z)
    for (i, c) in chars.iter().enumerate() {
        g.ch[*c].pos[0] = contacts[i].pos[0];
        g.ch[*c].pos[2] = contacts[i].pos[2];
    }
    crate::game::game_roll_tick(g, dt);
    for s in seats.iter().copied() {
        let pl = crate::cmd::seat_man(g, s);
        if !g.pl[pl].down {
            crate::seats::with_seat(g, pl, &mut |g: &mut Game| {
                crate::player::settle_player(g);
                crate::player::settle_held(g);
            });
        }
    }
    for (i, c) in chars.iter().enumerate() {
        let p = g.ch[*c].pos;
        if m::abs(p[0] - before[i][0]) + m::abs(p[2] - before[i][1]) > 0.005 { crate::character::character_update(g, *c, 0.0); }
    }
}

/// (game.ts Game.dropThrowable)
pub fn game_drop_throwable(g: &mut crate::game::Game, kind: crate::game::ThrowKind, p: V3) { crate::combat::drop_throwable(g, kind, p) }

/// (game.ts Game.throwCount)
pub fn game_throw_count(g: &mut crate::game::Game, kind: crate::game::ThrowKind) -> f64 { crate::player::throw_count(g, kind) }

/// (game.ts Game.objectivePos)
pub fn game_objective_pos(g: &mut crate::game::Game) -> Option<V3> { crate::mission::objective_pos(g) }

/// (game.ts Game.objectiveText)
pub fn game_objective_text(g: &mut crate::game::Game) -> String { crate::mission::objective_text(g) }

#[cfg(test)]
mod tests {
    use super::*;
    /// spot() against the TypeScript's own bits (the closure of game.ts:2329 run under bun with EXACT on)
    #[test]
    fn spot_is_the_typescripts_to_the_bit() {
        let cases: [(Leaf, bool, [u64; 3], bool, u64); 6] = [
            (Leaf { u: [0.8, 0.6], fx: 1.1, fz: 2.3, lo: 0.3, hi: 0.8, crack: true, chest: [1.4, 1.25, 2.0], rgt: [0.6, -0.8], arm_l: Some([1.3, 1.4, 2.1]), arm_r: Some([1.6, 1.42, 2.2]) }, false, [0x3ff570a3d70a3d71, 0x3ff2e147ae147ae1, 0x4003d70a3d70a3d7], true, 0x3fdc37d6d52bdddb),
            (Leaf { u: [0.8, 0.6], fx: 1.1, fz: 2.3, lo: 0.3, hi: 0.7, crack: false, chest: [1.4, 1.25, 2.0], rgt: [0.6, -0.8], arm_l: Some([1.3, 1.4, 2.1]), arm_r: Some([1.6, 1.42, 2.2]) }, true, [0x3ff5f3b645a1cac2, 0x3ff51eb851eb851e, 0x4004083126e978d5], true, 0x3fd925e3f96b21b3),
            (Leaf { u: [-0.6, 0.8], fx: 0.25, fz: -1.7, lo: 0.3, hi: 0.8, crack: true, chest: [0.31, 0.9, -1.52], rgt: [-0.28, 0.96], arm_l: None, arm_r: None }, true, [0xbf7beb5ec62d21c0, 0x3feb333333333333, 0xbff5b8a40a2af706], true, 0x3fd51eb851eb851f),
            (Leaf { u: [0.28, -0.96], fx: 3.3, fz: 3.4, lo: 0.3, hi: 0.7, crack: false, chest: [0.2, 1.3, 0.1], rgt: [1.0, 0.0], arm_l: None, arm_r: None }, false, [0x400bf7ced916872b, 0x3ff5999999999999, 0x4005d2f1a9fbe76c], false, 0x40117fbd56adbaa6),
            (Leaf { u: [1.0, 0.0], fx: 0.0, fz: 0.0, lo: 0.3, hi: 0.8, crack: true, chest: [0.5, 1.1, 0.2], rgt: [0.0, 1.0], arm_l: Some([0.5, 1.25, 0.18]), arm_r: None }, false, [0x3fe55d3abc2fc066, 0x3ff07ae147ae147b, 0x0000000000000000], true, 0x3fd51eb851eb851f),
            (Leaf { u: [0.0, 1.0], fx: 0.05, fz: 0.6, lo: 0.3, hi: 0.7, crack: false, chest: [0.1, 1.2, 0.72], rgt: [1.0, 0.0], arm_l: None, arm_r: Some([0.11, 1.38, 0.73]) }, true, [0x3fa999999999999a, 0x3ff47ae147ae147a, 0x3ff09e8b360b33a2], true, 0x3fd51eb851eb8520),
        ];
        for (l, right, p, reach, dist) in cases.iter() {
            let s = spot(l, *right);
            assert_eq!([s.p[0].to_bits(), s.p[1].to_bits(), s.p[2].to_bits()], *p);
            assert_eq!(s.reach, *reach);
            assert_eq!(s.dist.to_bits(), *dist);
        }
    }
}
