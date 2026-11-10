//! What a seat's HUD marks and draws of its aim: the marks over men, the last sightings, the cameras and the grenades, the meters, the ring and the arc. Each read here is
//! the rule's own function (asked of the game shared); nothing is kept, nothing is judged twice, and where a place on the screen goes is the HUD's.
use super::*;
use crate::game::{Game, GuardState};
use crate::ids::*;
use crate::js;
use alloc::{format, string::String, vec::Vec};

/// the row of dots for how much has reached a man: one at the least of it, seven at the most ('': nothing worth a mark)
fn wary_dots(awareness: f64) -> String {
    if awareness > 0.05 { "·".repeat(1 + crate::m::floor(awareness * 6.0) as usize) } else { String::new() }
}

/// a man of the floor's mark: which glyphs, which colour, and nothing of where
fn guard_mark(g: &Game, gd: GuardId) -> (String, HeadTone) {
    let m = &g.gd[gd];
    let own = m.team == crate::teams::Team::Operative;
    let (mut text, mut tone) = (String::new(), HeadTone::Calm);
    let zz = crate::doze::doze_mark(g, gd);
    match m.state {
        GuardState::Dead => if crate::game::is_breathing(g, gd) { text = String::from("zz"); tone = HeadTone::Ko; } else { text = String::from("✕"); tone = HeadTone::Dead; },
        GuardState::Alert => { text = String::from(if !own && crate::aim::guard_acquired(g, gd) { "◎!" } else { "!" }); tone = HeadTone::Alert; }
        GuardState::Suspicious | GuardState::Search => if !own { text = String::from("?"); tone = HeadTone::Sus; },
        GuardState::Patrol => if !zz.is_empty() { text = String::from(zz); } else if !own { let d = wary_dots(m.awareness); if !d.is_empty() { text = d; tone = HeadTone::Sus; } },
    }
    // (a man of your own bent away with a forearm over his eyes says it himself)
    if !g.cond[m.ch].down && crate::condition::is_blind(g, m.ch) && !own {
        text.insert(0, '✹');
    }
    (text, tone)
}

/// the debug label's words over a guard
fn guard_debug(g: &Game, gd: GuardId) -> String {
    let m = &g.gd[gd];
    let mut s = format!("{} {}%", m.state.as_str(), js::to_i32(m.awareness * 100.0));
    if crate::condition::is_blind(g, m.ch) { s += &format!(" dazzled {:.1}s", g.cond[m.ch].blind_until - g.time); }
    if let Some(t) = g.m.needs.mem_of.get(gd).and_then(|r| r.trip.as_ref()) {
        use crate::needs::TripPhase as P;
        s += &format!(" {}:{}", t.kind.as_str(), t.phase.as_str());
        if t.phase == P::Telegraph { s += &format!(" {}s", js::num_str(crate::m::ceil(t.depart_at - g.time))); }
        if t.phase == P::Dwell { s += &format!(" {}s", js::num_str(crate::m::ceil(t.dwell_until - g.time))); }
    }
    s
}

/// a civilian's small mark
fn civ_mark(g: &Game, gd: GuardId) -> (String, HeadTone) {
    use crate::civilians::CivState as S;
    let m = &g.gd[gd];
    let st = g.cv[m.civ.unwrap()].state;
    if g.cond[m.ch].down { return if crate::game::is_breathing(g, gd) { (String::from("zz"), HeadTone::Ko) } else { (String::from("✕"), HeadTone::Dead) }; }
    match st {
        S::Startled => (String::from("?"), HeadTone::Sus),
        S::Cower | S::Held => (String::from("·"), HeadTone::Sus),
        S::Work => { let d = wary_dots(m.awareness); if d.is_empty() { (d, HeadTone::Calm) } else { (d, HeadTone::Sus) } }
        S::Phone => (String::from("!☎"), HeadTone::Alert),
        _ => (String::from("!"), HeadTone::Alert),
    }
}

/// the debug label's words over a civilian
fn civ_debug(g: &Game, gd: GuardId) -> String {
    let m = &g.gd[gd];
    let c = &g.cv[m.civ.unwrap()];
    let cause = match c.cause { Some(x) => format!(" [{}{}{}]", x.as_str(), if c.armed { "!" } else { "" }, if c.reported { " told" } else { "" }), None => String::new() };
    format!("{} {}{} {}%", c.name, c.state.as_str(), cause, js::to_i32(m.awareness * 100.0))
}

/// what the HUD marks about `seat`
pub(super) fn marks_view(g: &Game, seat: &crate::seats::SeatRef) -> MarksView {
    let me = &g.pl[seat.player];
    let row = |gd: GuardId, civ: bool| {
        let (text, tone) = if civ { civ_mark(g, gd) } else { guard_mark(g, gd) };
        let debug = if !g.show_debug { String::new() } else if civ { civ_debug(g, gd) } else { guard_debug(g, gd) };
        ManMark { man: g.ch[g.gd[gd].ch].id, text, tone, civ, hidden: crate::fogofwar::fog_of_war_hides(g, gd), debug }
    };
    let mut men: Vec<ManMark> = g.guards.iter().map(|&gd| row(gd, false)).collect();
    men.extend(g.civilians.iter().filter(|&&cv| g.gd[cv].civ.map_or(false, |c| g.cv[c].state != crate::civilians::CivState::Gone)).map(|&cv| row(cv, true)));
    let stalk = crate::stalk::stalk_on(g, seat).map(|gd| g.ch[g.gd[gd].ch].id);
    MarksView {
        men,
        seen: crate::fogofwar::fog_of_war_last_seen(g),
        cams: crate::stickycam::sticky_marks(g, seat.player),
        frags: crate::frag::frag_marks(g),
        blind: crate::samstun::man_blind(g, seat.player, None),
        light: g.shown[me.ch].visibility,
        irradiance: g.shown[me.ch].irradiance,
        noise: me.noise,
        stalk,
        stalk_waits: crate::stalk::stalk_waits(g, seat),
        floor: crate::floor::floor_heard(g, g.ch[me.ch].pos),
    }
}

/// the arc he has up, with where it comes to rest and what it would reach
fn throw_view(g: &Game, seat: &crate::seats::SeatRef) -> Option<ThrowView> {
    let sol = g.sets[seat.id].hand.throw_preview.as_ref().filter(|s| s.points.len() > 1)?;
    let mut world = crate::ask::WorldAsk::try_of(g).ok();
    let rest = world.as_mut().and_then(|w| crate::player::throw_rest(w, seat.player));
    let at = rest.as_ref().map(|r| r.at);
    Some(ThrowView { points: sol.points.clone(), land: sol.land, short: crate::he40::preview_too_close(g, seat.player), reach: crate::frag::throw_reach(g, seat.player, at), rest })
}

/// what the HUD draws of `seat`'s aim
pub(super) fn aim_view(g: &Game, seat: &crate::seats::SeatRef) -> AimView {
    let pl = seat.player;
    let p = &g.pl[pl];
    let gun_out = crate::game::player_gun_in_hand(g, pl).is_some() && !g.cond[p.ch].down;
    let steady = gun_out && !p.sprinting && p.dragging.is_none();
    let model = g.tune.aim_model && steady;
    let swing = if steady { crate::aimweight::aim_gun_point(g, seat) } else { None };
    let trail = swing.is_some() && !crate::aimweight::aim_gun_on(g, pl);
    let shut = steady && crate::ask::WorldAsk::try_of(g).map_or(false, |mut w| crate::player::gun_shut(&mut w, seat));
    let snap = seat.aim_snap;
    let ring = if gun_out && (snap.is_some() || model || swing.is_some()) {
        let at = swing.or(snap).unwrap_or(seat.aim_point);
        let cone = if model {
            let a = crate::aim::player_aim(g, pl);
            let c = &g.ch[p.ch];
            let hand = c.bones.get(crate::character::Bone::HandR).copied().unwrap_or([c.pos[0], c.pos[1] + 1.4, c.pos[2]]);
            Some(AimCone { spread: a.spread, track: a.track, range: crate::v3::dist(at, hand), hands: a.base, settle: a.settle, crossing: a.tgt, feet: a.own })
        } else { None };
        let on = if seat.aim_guard.is_some() { RingOn::Guard } else if snap.is_some() { RingOn::Light } else { RingOn::Free };
        Some(AimRing { at, on, trail, shut, wish: if trail { snap } else { None }, cone })
    } else { None };
    let dot = if trail || shut { Some(GunDot { at: swing.unwrap_or(seat.aim_point), shut }) } else { None };
    AimView { ring, dot, too_close: gun_out && !p.sprinting && crate::he40::he40_ready(g, pl) && crate::he40::preview_too_close(g, pl), throw: throw_view(g, seat) }
}

/// who wears or carries what the drawer draws: the goggles of a teammate, the big slot's tool of anybody
pub(super) fn worn_view(g: &Game) -> Vec<WornView> {
    let man = |id: f64, gd: Option<GuardId>| {
        let goggles = gd.map_or(false, |gd| crate::nvg::nvg_worn(g, gd));
        let big = crate::bigkit::big_carried(g, crate::bigkit::BigCarriedC { id });
        if goggles || big.is_some() { Some(WornView { man: id, goggles, big }) } else { None }
    };
    let seats = g.seats.iter().filter_map(|&s| { let pl = crate::seats::seat(g, s).player; man(g.ch[g.pl[pl].ch].id, None) });
    let guards = g.guards.iter().filter_map(|&gd| man(g.ch[g.gd[gd].ch].id, Some(gd)));
    seats.chain(guards).collect()
}
