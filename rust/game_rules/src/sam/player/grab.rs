//! HIS HANDS ON A MAN: struck down, or taken hold of.
use super::*;
use crate::close::CloseKind;
use crate::game::{Deed, Game, GuardState};
use crate::ids::*;
use crate::m;
use crate::v3::{self, V3};
use alloc::vec::Vec;

/// The way in a strike or a grab is made by: the one asked, else what he could do from where he stands.
fn way_in(g: &Game, gd: GuardId, way: Option<CloseKind>) -> CloseKind {
    if let Some(way) = way { return way; }
    let pos = g.player_pos(g.player);
    let o = crate::close::close_at(g, gd, &pos[..]);
    crate::close::close_kind_of(Some(&o))
}

/// Silent takedown: lock in on the guard, throw the blow, he drops with a thud only the room hears.
pub fn start_takedown(g: &mut crate::game::Game, gd: GuardId, way: Option<CloseKind>, lethal: Option<bool>, loud: Option<bool>) {
    let (lethal, loud) = (lethal == Some(true), loud == Some(true));
    let p = g.player;
    let (c, gc) = (g.pl[p].ch, g.gd[gd].ch);
    if crate::condition::down(g, gc) || crate::condition::down(g, c) { return; }
    let kind = way_in(g, gd, way);
    let side_on = kind != CloseKind::Behind;
    let (a, b) = (g.ch[c].pos, g.ch[gc].pos);
    let from = if side_on { v3::wrap_angle(v3::yaw_to(b, a) - g.ch[gc].body_yaw) } else { 0.0 };
    hands_to_man(g, p);
    let mut td = crate::game::PlayerTakedown { g: gd, t: 0.0, kind: None, from: None, alert: None, yaw: None, at: None, lethal: None, loud: None };
    if side_on {
        td.kind = Some(kind);
        td.from = Some(from);
        td.alert = Some(g.gd[gd].state == GuardState::Alert);
        td.at = Some(crate::close::close_station(g, gd, kind, from).pos);
    }
    if loud { td.loud = Some(true); }
    if lethal { td.lethal = Some(true); }
    crate::deed::begin(g, Deed::Takedown(td));
    let blow = if kind == CloseKind::Front { crate::character::StrikeKind::Jab } else { crate::character::StrikeKind::Cross };
    crate::character::anim_strike(g, c, Some(blow), if lethal { Some(true) } else { None });
    crate::takedown::take_begin(g, c, gd, Some(kind), lethal);
    crate::close::close_took(crate::close::CloseTookWhat::Take);
    let man = &mut g.gd[gd];
    man.speed = 0.0;
    man.react_t = m::max(man.react_t, 1.0);
    g.ch[gc].vel = [0.0, 0.0, 0.0];
    g.gd[gd].drop_path();
    let at = v3::add(g.ch[gc].pos, [0.0, 1.3, 0.0]);
    crate::audio_iface::game_audio_like_play(g, "bodyHit", Some(at), Some(if loud { 0.8 } else { 0.55 }), Some(&crate::audio_iface::PlayOpts::Rate { rate: 0.8 }));
}
/// Both his hands go to a man.
fn hands_to_man(g: &mut Game, p: PlayerId) {
    if let Some(w) = g.pl[p].wall.as_mut() { w.drawn = false; }
    crate::deed::hands_free(g);
}

/// One tick of the takedown: hold station on the man, land the blow at its beat, give control back as it recovers.
pub(super) fn update_takedown(g: &mut crate::game::Game, dt: f64) {
    let p = g.player;
    let Some(td) = g.pl[p].takedown().copied() else { return; };
    let gd = td.g;
    if !crate::game::game_on_floor(g, gd) { g.pl[p].takedown_over(); return; }
    let (c, gc) = (g.pl[p].ch, g.gd[gd].ch);
    g.ch[c].vel = [0.0, 0.0, 0.0];
    let yaw = take_station(g, p, &td, dt);
    let ch = &mut g.ch[c];
    ch.body_yaw = v3::approach_angle(ch.body_yaw, yaw, 14.0 * dt);
    ch.aim_yaw = ch.body_yaw;
    ch.aim_pitch = v3::damp(ch.aim_pitch, 0.1, 8.0, dt);
    let t = td.t + dt;
    if let Some(row) = g.pl[p].takedown_mut() { row.t = t; }
    if t >= 0.28 && !crate::condition::down(g, gc) { blow_lands(g, &td, yaw); }
    if t > 0.62 { g.pl[p].takedown_over(); }
}
/// While he is up: behind him, or where the blow came in from, through Sam's own collision. Returns the way Sam faces.
fn take_station(g: &mut Game, p: PlayerId, td: &crate::game::PlayerTakedown, dt: f64) -> f64 {
    let (c, gc) = (g.pl[p].ch, g.gd[td.g].ch);
    let his = g.ch[gc].body_yaw;
    let side_on = td.kind.is_some();
    if crate::condition::down(g, gc) { return if side_on { td.yaw.unwrap_or(g.ch[c].body_yaw) } else { his }; }
    let b = g.ch[gc].pos;
    let at = match (td.kind, td.at) {
        (None, _) => [b[0] - m::sin(his) * 0.55, 0.0, b[2] - m::cos(his) * 0.55],
        (Some(_), Some(at)) => at,
        (Some(kind), None) => crate::close::close_station(g, td.g, kind, td.from.unwrap_or(0.0)).pos,
    };
    let to = v3::lerp3(g.ch[c].pos, at, 1.0 - m::exp(-14.0 * dt));
    crate::character::pos_replace(g, c, to);
    collide_player(g, if g.pl[p].crouch { 1.0 } else { 1.7 });
    if !side_on { return his; }
    let (a, b) = (g.ch[gc].pos, g.ch[c].pos);
    let yaw = v3::yaw_to(b, a);
    if let Some(t) = g.pl[p].takedown_mut() { t.yaw = Some(yaw); }
    yaw
}
/// He goes down: how loudly, how he falls, and what those who saw it saw.
fn blow_lands(g: &mut Game, td: &crate::game::PlayerTakedown, yaw: f64) {
    let gd = td.g;
    let (lethal, loud) = (td.lethal == Some(true), td.loud == Some(true));
    let alert = if td.kind.is_some() { td.alert == Some(true) } else { g.gd[gd].state == GuardState::Alert };
    let level = crate::close::close_noise(g, td.kind.unwrap_or(CloseKind::Behind), alert) + (if loud { g.k.close.rmb_loud } else { 0.0 });
    if td.kind.is_some() && crate::close::close_seated(g, gd) { crate::close::close_unseat(g, gd, None); }
    crate::takedown::take_fall(g, gd, td.kind, lethal, None);
    let way: V3 = [m::sin(yaw), 0.0, m::cos(yaw)];
    if lethal {
        crate::combat::kill_guard(g, gd, way, Some(true), Some(crate::game::DownKind::Shot), Some(crate::game::DownBy::Player), crate::game::With::Knife);
        seen_struck(g, gd);
    } else {
        crate::combat::kill_guard(g, gd, way, Some(true), None, None, crate::game::With::Hand);
    }
    prop_noise(g, g.pos_of(gd), level);
}
/// A man killed by hand this moment was seen struck, not shot.
fn seen_struck(g: &mut Game, gd: GuardId) {
    use crate::game::WitnessSawActKind as Saw;
    let (now, victim) = (g.time, g.gd[gd].callsign.clone());
    for &o in &g.guards {
        if let Some(w) = g.gd[o].witness.saw_act.as_mut().filter(|w| w.kind == Saw::Shot && w.t == now && w.victim == victim) { w.kind = Saw::Struck; }
    }
}

/// Which hold a grab made now would be to begin with.
pub fn grab_variant(g: &crate::game::Game, seat: &crate::seats::SeatRef) -> crate::character::HoldVariant {
    if crate::game::player_pistol_up(g, seat.player) { crate::character::HoldVariant::Gun } else { crate::character::HoldVariant::Arm }
}

/// He takes hold of a man: drops whatever his hands had, and the man is frozen for the beat the grab closes in. From here he is `held`.
pub fn start_grab(g: &mut crate::game::Game, gd: GuardId, way: Option<CloseKind>) {
    let p = g.player;
    let (c, gc) = (g.pl[p].ch, g.gd[gd].ch);
    let kind = way_in(g, gd, way);
    let man = &g.gd[gd];
    if matches!(man.state, GuardState::Dead | GuardState::Alert) || man.held.is_some() || g.cond[c].down || g.pl[p].planted() { return; }
    if g.pl[p].crouch && !can_stand(g) { return; }
    if crate::game::player_rifle_up(g, p) { set_holstered(g, true, Some(true)); }
    let variant = grab_variant(g, &crate::seats::here(g));
    hands_to_man(g, p);
    let pl = &mut g.pl[p];
    pl.crouch = false;
    pl.speed_sm = 0.0;
    pl.backpedal = false;
    g.ch[c].vel = [0.0, 0.0, 0.0];
    g.ch[c].anim.reverse = false;
    cut_off(g, gd);
    if crate::close::close_seated(g, gd) && g.gd[gd].civ.is_none() { crate::close::close_unseat(g, gd, None); }
    let since = g.time;
    g.gd[gd].held = Some(crate::game::GuardHeld { by: crate::game::GuardHeldBy::Player, since, variant });
    if g.gd[gd].civ.is_some() { crate::civilians::civilian_grabbed(g, gd); }
    let talk = g.gd[gd].talk;
    g.tab.game_talk[talk].held_since = since;
    let serial = g.serial();
    crate::deed::begin(g, Deed::Hold(Hold {
        g: gd, variant, phase: crate::character::HoldPhase::Grab, t: 0.0, since, wind_t: -1.0, swap_t: f64::INFINITY,
        remarked: false, gun_out: 0.0, fire_queued: false, shot_t: -1.0, ask: None, after: None, serial, ..Hold::default()
    }));
    crate::disarm::disarm_begin(g);
    crate::close::close_took(crate::close::CloseTookWhat::Grab);
    g.arms[c].fire_cd = m::max(g.arms[c].fire_cd, g.k.hold.secs.grab);
    let chest = v3::add(g.ch[gc].pos, [0.0, 1.4, 0.0]);
    crate::disarm::disarm_catch(g, chest);
    prop_noise(g, g.ch[gc].pos, GRAB_NOISE);
    crate::chronicle::chron_grab(g, gd);
}

/// The man grabbed is cut off from all he was at: his words, his part in a clearing, his script, his way, and the flashlight in his hand.
fn cut_off(g: &mut Game, gd: GuardId) {
    let now = g.time;
    if g.gd[gd].bubble.as_ref().is_some_and(|b| now - b.t < b.dur) { g.gd[gd].bubble = None; }
    leaves_clearing(g, gd);
    if g.gd[gd].script.is_some() {
        let after = match g.gd[gd].state { s @ (GuardState::Patrol | GuardState::Search) => s, _ => GuardState::Suspicious };
        crate::squad::end_guard_script(g, gd, Some(after));
    }
    g.gd[gd].drop_path();
    let gc = g.gd[gd].ch;
    g.ch[gc].vel = [0.0, 0.0, 0.0];
    g.ch[gc].anim.look_yaw_extra = 0.0;
    let man = &mut g.gd[gd];
    man.speed = 0.0;
    man.body_duty = None;
    man.pieing = None;
    man.search_plan = None;
    man.muzzle_down = false;
    man.stall_ref = None;
    man.sweep = 0.0;
    man.light_lit_at_loss = g.tab.light[man.flashlight].enabled;
    if crate::guards::in_hand(g, gd) == Some(crate::guards::InHand::Torch) {
        let beam = g.gd[gd].beam_dir;
        let item = crate::combat::drop_from_hand(g, gc, crate::combat::DropFromHandKind::Torch, beam, [0.0, 0.0, 0.0], Some(false), Some(true), None);
        g.gd[gd].dropped = Some(item);
        g.gd[gd].torch_lost_at = g.time;
    }
}
/// He was clearing a room with another: the clearing is off, and the other, with nobody answering him, comes to look.
fn leaves_clearing(g: &mut Game, gd: GuardId) {
    let Some((a, b)) = g.clearing.as_ref().filter(|cl| cl.stage != crate::squad::ClearStage::Done && (cl.a == gd || cl.b == Some(gd))).map(|cl| (cl.a, cl.b)) else { return };
    crate::squad::clearing_cancel(g);
    g.clearing = None;
    let Some(o) = (if a == gd { b } else { Some(a) }) else { return };
    if crate::condition::down(g, g.gd[o].ch) || g.gd[o].held.is_some() { return; }
    crate::mind::set_state(g, o, GuardState::Suspicious, "player");
    crate::mind::raise(g, o, 0.6, "player");
    let at = g.pos_of(gd);
    crate::belief::word(g, o, at);
    g.gd[o].react_t = 0.7;
    g.gd[o].search_t = 0.0;
    g.gd[o].drop_path();
    g.gd[o].search_plan = None;
    g.gd[o].pieing = None;
}
