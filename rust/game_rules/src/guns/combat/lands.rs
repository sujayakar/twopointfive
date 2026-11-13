//! What a round does where it ends: to the panes and walls it went through, to what it came to, to whom it hit, and what is seen and heard of that.
use super::*;
use crate::fixtures::FixtureKind;
use crate::game::Game;
use crate::ids::*;
use crate::m;
use crate::v3::{self, V3};
use alloc::{string::String, vec::Vec};

/// Everything a round's end does, in order.
pub(super) fn lands(g: &mut Game, r: &mut Round, f: &Flight, arm: Option<AcrossArm>, e: &End) {
    leaves_holes(g, r, f, e);
    if let Some(shell) = r.shell.as_deref_mut() {
        let (index, at) = match arm { Some(a) => (a.index, a.at), None => (e.stop.filter(|_| e.bare).map_or(-1.0, |s| s.index), e.at) };
        crate::shotgun::pellet_end(shell, index, e.t, at);
    }
    if r.third {
        crate::suppression::note_round(g, r.shooter, r.muzzle, e.at, e.hit);
        knocks_thing(g, r, f, e);
        stirs_smoke(g, f, e);
    }
    let unseen = e.hit.and_then(|c| crate::character::guard_of(g, c)).is_some_and(|gd| crate::fogofwar::fog_of_war_hidden(g, gd));
    if unseen { crate::fogofwar::fog_of_war_hush(g); }   // (from here on: what is made of a man the fog hides is nobody's to be shown)
    if r.lead { flashes(g, r, e, unseen); }
    strikes(g, r, f, e);
    traces(g, r, f, e);
    tells_hit(g, r, e);
    if r.by_player {
        puts_out(g, r, f, e);
    } else if !f.through {
        breaks_fixtures(g, f, e);
    }
    if r.lead { reports(g, r, e); }
}

/// The bars settled, the panes and walls it went through holed, and the pane it was spent in.
fn leaves_holes(g: &mut Game, r: &mut Round, f: &Flight, e: &End) {
    let who = r.who;
    if !f.bars.is_empty() { crate::winbars::bars_settle(g, &f.bars, e.t, who, r.lead); }
    r.with_glass(|sh| crate::glasspen::commit_crossings(g, &f.path.cross, e.t, who, sh));
    if let Some(w) = &f.walls { crate::wallpen::commit_walls(g, w, e.t, r.shooter, &r.def, r.lead, r.third, r.shell.as_deref_mut()); }
    if let Some(s) = e.stop.filter(|s| e.bare && s.index < 1e6) {
        r.with_glass(|sh| crate::glasspen::spent_in_pane(g, s.index, e.at, e.dir, f.energy(e.t), who, sh));
    }
}

/// The loose thing it came to takes its push.
fn knocks_thing(g: &mut Game, r: &Round, f: &Flight, e: &End) {
    let Some(h) = e.thing else { return };
    let j = r.def.prop_impulse * (f.energy(h.t) / f.e0);
    let d = f.way(h.t);
    crate::props::prop_system_apply_impulse(g, h.prop, f.at(h.t), [d[0] * j, 0.0, d[2] * j], r.who, Some(true));
}

/// A round through the air: each straight leg is one line for the host's solver, which stirs its smoke along it or has none there to stir.
fn stirs_smoke(g: &mut Game, f: &Flight, e: &End) {
    let far = m::min(e.t, 25.0);
    for k in &f.path.segs {
        let (a, b) = (m::max(k.t0, 0.3), m::min(k.t0 + k.len, far));
        if b > a { crate::smoke_types::smoke_system_like_round(g, v3::mad(k.o, k.d, a - k.t0), k.d, b - a); }
    }
}

/// The flash of it landing: the shooter's light, for the fog, unless it is in a man the fog hides.
fn flashes(g: &mut Game, r: &Round, e: &End, unseen: bool) {
    let l = crate::render_lights::light_set_flash(g, v3::mad(e.at, e.dir, -0.05), [1.0, 0.8, 0.5], 1.5, 2.5, 0.05, 0.0);
    if !unseen { crate::fogofwar::fog_of_war_light_by(g, l, Some(r.who)); }
}

/// The level's side of it: the sound and the mark (a pane makes its own), what it knocks out, and the destruction's share.
fn strikes(g: &mut Game, r: &Round, f: &Flight, e: &End) {
    let pane = e.stop.filter(|s| !f.through && s.index < 1e6).is_some_and(|s| crate::glass::is_pane(g, g.col.boxes.get(s.index as usize).copied()));
    if e.hit.is_none() && !pane && r.lead { crate::audio_iface::game_audio_like_play(g, crate::audio_iface::Sfx::BulletImpact, Some(e.at), Some(0.8), None); }
    if !e.bare { return; }
    let met = met(f, e);
    if !pane {
        let surface = match (met, e.thing) {
            (_, Some(h)) => Some(crate::marks::Surface2 { box_: h.box_, dynamic: true }),
            (Some(_), None) => f.path.st.filter(|s| s.t <= e.t + 1e-3).and_then(|s| crate::marks::static_surface(g, s.index)),
            (None, None) => None,
        };
        crate::marks::bullet_mark(g, e.at, met.map_or([0.0, 1.0, 0.0], |h| h.n), surface, &r.def);
    }
    let Some(met) = met else { return };
    crate::particle_fx::impact_fx(g, e.at, e.dir, met, &r.def);
    if r.third { crate::destruction::destruction_round(g, r.shooter, e.at, e.dir, &met, &r.def); }
}

/// What of the level it came to: the loose thing, or else the static.
fn met(f: &Flight, e: &End) -> Option<RoundMet> {
    if let Some(h) = e.thing { return Some(RoundMet { n: h.n, index: None, prop: Some(h.prop), box_: Some(h.box_) }); }
    f.path.st.map(|s| RoundMet { n: s.n, index: Some(s.index), prop: None, box_: None })
}

/// A tracer, the straight way from the muzzle to where it ended.
fn traces(g: &mut Game, r: &Round, f: &Flight, e: &End) {
    use crate::tracers::TracerImpact;
    if r.def.tracer.is_none() || !g.k.tune.tracers { return; }
    let gun = firearm_of(g, r.shooter);
    let (along, far) = if f.bent() { (v3::normalize(v3::sub(e.at, r.muzzle)), v3::dist(r.muzzle, e.at)) } else { (r.dir, e.t) };
    let impact = if e.hit.is_some() {
        TracerImpact::Flesh
    } else if f.through || met(f, e).is_some() {
        TracerImpact::Solid
    } else {
        TracerImpact::None
    };
    crate::tracers::tracers_round(g, Some(gun), &r.def, r.muzzle, along, far, impact, r.who, None);
}

/// The man it found, and what is kept of it: whose side he was on, whom it went past, how he died.
fn tells_hit(g: &mut Game, r: &mut Round, e: &End) {
    if r.ours && e.hit.is_some_and(|c| crate::friendlyfire::on_our_side(g, c)) { crate::friendlyfire::ff_note(g, r.shooter); }
    if r.by_player && r.lead { crate::friendlyfire::ff_past(g, r.muzzle, e.at, e.hit); }
    let was_down = e.hit.is_some_and(|c| crate::condition::down(g, c));
    crate::deathrecap::recap_round(g, e.hit, was_down, r.shooter, r.muzzle, e.at, &r.def, r.blind);
    let Some(c) = e.hit else { return };
    let into = crate::shotgun::RoundIn { c, shooter: r.shooter, shield: Some(c) == shield_of(g), dir: e.dir, hit_p: e.at };
    match r.shell.as_deref_mut() {
        None => round_in(g, &into),
        Some(shell) => { if crate::shotgun::pellet_drops(g, shell, c, e.t, e.dir) { shell.late.push(into); } }
    }
}

/// A player's round that meets a fixture: a fire stirred, a light put out.
fn puts_out(g: &mut Game, r: &Round, f: &Flight, e: &End) {
    let targets = g.targets.clone();
    if let Some(s) = e.stop.filter(|s| s.index >= 0.0) {
        if let Some(&t) = targets.iter().find(|&&t| crate::fire::fire_holds(g, t) && crate::fire::fire_struck(g, t, e.at, s.index)) { crate::fire::fire_stir(g, t); }
        if let Some(&t) = targets.iter().find(|&&t| lamp_live(g, t) && lamp_own(g, t).contains(&s.index)) { return shoot_out(g, t, r.muzzle); }
    }
    let legs: Vec<_> = f.path.segs.iter().filter(|s| s.t0 <= e.t).map(|s| crate::aimcost::RoundLeg { o: s.o, d: s.d, end: m::min(s.len, e.t - s.t0) }).collect();
    if let Some(&t) = targets.iter().find(|&&t| lamp_grazed(g, t, &legs)) { shoot_out(g, t, r.muzzle); }
}

/// The lamp a round comes near enough to put out.
fn lamp_grazed(g: &mut Game, t: FixtureId, legs: &[crate::aimcost::RoundLeg]) -> bool {
    let (pos, kind) = (g.tab.fixture[t].pos, g.tab.fixture[t].kind);
    if !lamp_live(g, t) || crate::aimcost::legs_near(pos, legs) > 3.0 { return false; }
    let slack = if kind == FixtureKind::Breaker { 0.08 } else { 0.3 };
    let boxes: Vec<Option<BoxId>> = lamp_own(g, t).iter().map(|&b| g.engine.world.statics.get(b as usize).copied()).collect();
    let bare = if kind == FixtureKind::Device { g.k.aimcost.bare_device } else { g.k.aimcost.bare };
    let tight = m::min(slack, g.k.aimcost.slack);
    crate::aimcost::lamp_struck(g, pos, &boxes, false, legs, Some(bare), Some(tight))
}

/// A target a round can put out.
fn lamp_live(g: &Game, t: FixtureId) -> bool {
    !(g.tab.fixture[t].broken
        || !g.tab.fixture[t].interactive
        || g.tab.fixture[t].factor <= 0.0 && !g.tab.fixture[t].on && !crate::carlamps::lamp_glass(g, t)
        || crate::fire::fire_holds(g, t))
}

/// Its boxes; the breaker's target lists its LED alone, a round into the panel is a round into the breaker.
fn lamp_own(g: &Game, t: FixtureId) -> &[f64] {
    if g.tab.fixture[t].kind == FixtureKind::Breaker { &g.level.breaker.boxes } else { &g.tab.fixture[t].boxes }
}

/// A shot-out light: the breaker's blackout, a dark lens's glass, or a fixture gone.
fn shoot_out(g: &mut Game, t: FixtureId, muzzle: V3) {
    let kind = g.tab.fixture[t].kind;
    let pos = g.tab.fixture[t].pos;
    if kind == FixtureKind::Breaker {
        crate::render_lights::light_set_flash(g, pos, [0.6, 0.7, 1.0], 10.0, 5.0, 0.2, 0.0);
        crate::audio_iface::game_audio_like_play(g, crate::audio_iface::Sfx::LightBreak, Some(pos), Some(1.0), None);
        crate::game::game_set_blackout(g, f64::INFINITY, crate::lighting::SetBlackoutCause::Shot);
        g.tab.fixture[t].broken = true;
        crate::particle_fx::breaker_spark_fx(g, pos, muzzle);
        return;
    }
    let lens = g.tab.fixture[t].lens;
    if crate::carlamps::dead_lamp(g, t) {
        g.tab.fixture[t].broken = true;
        crate::audio_iface::game_audio_like_play(g, crate::audio_iface::Sfx::LightBreak, Some(pos), Some(0.8), None);
        crate::particle_fx::lens_shatter_fx(g, crate::particle_fx::LensShatterFxT { pos, lens }, muzzle);
        return;
    }
    g.tab.fixture[t].broken = true;
    crate::game::game_light_event(g, t, None, None);
    crate::chronicle::chron_light_shot(g, t);
    crate::render_lights::light_set_flash(g, pos, [0.7, 0.8, 1.0], 6.0, 4.0, 0.12, 0.0);
    crate::audio_iface::game_audio_like_play(g, crate::audio_iface::Sfx::LightBreak, Some(pos), Some(1.0), None);
    if crate::carlamps::lamp_glass(g, t) {
        crate::particle_fx::lens_shatter_fx(g, crate::particle_fx::LensShatterFxT { pos, lens }, muzzle);
    } else if kind != FixtureKind::Device {
        let boxes = g.tab.fixture[t].boxes.clone();
        crate::particle_fx::fixture_shatter_fx(g, &crate::particle_fx::FixtureShatterFxT { pos, kind: String::from(kind.as_str()), boxes }, muzzle);
    }
}

/// Anybody else's: its legs from the far end, for the destruction to break what hangs along them.
fn breaks_fixtures(g: &mut Game, f: &Flight, e: &End) {
    let index = e.stop.map(|s| s.index);
    for s in f.path.segs.iter().rev().filter(|s| s.t0 <= e.t) {
        let last = s.t0 + s.len >= e.t - 1e-6;
        crate::destruction::destruction_fixture_round(g, s.o, s.d, m::min(s.len, e.t - s.t0), index.filter(|_| last));
    }
}

/// The shot itself, for the floor to hear.
fn reports(g: &mut Game, r: &Round, e: &End) {
    let report = &r.def.report;
    let kind = if r.by_player { crate::director::EventKind::Shot } else { crate::director::EventKind::GuardShot };
    let serial = g.serial();
    let ev = crate::game::GameEvent {
        kind,
        pos: g.ch[r.shooter].pos,
        time: g.time,
        loud: report.loud,
        level: Some(report.level),
        weapon: Some(r.def.id),
        from: Some(r.muzzle),
        to: Some(e.at),
        who: Some(r.who),
        blind: r.blind.filter(|b| *b),
        serial,
        ..crate::game::GameEvent::default()
    };
    g.events.push(ev);
    if r.by_player {
        let p = g.player;
        g.pl[p].noise = m::max(g.pl[p].noise, report.level);
        crate::chronicle::chron_shot(g, &crate::gear::plain_short(&r.def), e.hit.is_some(), e.at);
    }
}
