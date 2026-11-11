//! THE LEVEL'S LIGHTS: whether a fixture is on, broken or blinded, what the mains are doing, the one number a fixture burns at this tick (its `factor`), the lamp each
//! fixture feeds as a light question reads it (rust/sim lights.rs), and a camera's and a tripwire's own machine.
//!
//! What men do about the dark is not here (lighting.rs, powerout.rs), nor how a fixture looks when it breaks (fixture_break.rs), nor where a lens looks at a man from:
//! THE MACHINE NEVER ASKS THE LIGHT.
pub use ::game_types::fixtures::types;
pub use types::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap};
use crate::m;
use crate::v3::V3;
use alloc::{string::String, vec::Vec};
use ::sim::lights::{Lights, ROWS, R_AREA, R_BEACON, R_CONE, R_ON, R_OWN, R_PASSING, R_ROLE, R_SCREEN, R_STEADY};

/// fixtures, beacons, cameras, tripwires a level holds (one over any of these is refused at load, by name)
pub const FIXTURES: usize = 256;
pub const BEACONS: usize = 32;
pub const CAMERAS: usize = 32;
pub const LASERS: usize = 32;

/// A camera's picture drains this fast once he is out of it (lit-seconds a second: a glimpse and gone is forgotten in a couple of seconds; ducking behind the desk for
/// a beat does not reset a camera that has been watching you walk toward it), and it goes quiet this long after a report, re-arming once drained AND this has passed
const CAMERA_DECAY: f64 = 0.5;
const CAMERA_REARM_SECS: f64 = 20.0;
/// a tripwire, once broken, cannot be broken again for this long (one crossing is one report, not sixty)
const LASER_COOLDOWN: f64 = 4.0;
/// the cone starts this far out of the lens (the beam starts at the lens face)
const CONE_AHEAD: f64 = 0.1;

/// the lamps' rows of this game (no borrow of `g`: it may be held across a read of `g`, never across a call that writes them)
#[inline(always)]
fn lights(g: &Game) -> &'static mut Lights {
    &mut ::sim::state::state(g.sim).lights
}
/// the same, to read
#[inline(always)]
fn lights_read(g: &Game) -> &'static Lights {
    &::sim::state::state_read(g.sim).lights
}
/// a lamp's row
#[inline(always)]
fn lamp_row(g: &Game, l: LightId) -> Option<usize> {
    g.tab.light[l].row.map(|r| r as usize)
}

/// A fixture more (the level as it is built): on in the level is authored.
pub fn fixture_add(g: &mut Game, f: Fixture) -> FixtureId {
    if g.tab.fixture.len() >= FIXTURES {
        panic!("a level of more than {} light fixtures got past levelfit", FIXTURES);
    }
    g.tab.fixture.push(Fixture { authored: f.on, ..f })
}

/// a lamp's row, which this writes: it must have one
fn row_of(g: &Game, l: LightId) -> usize {
    match lamp_row(g, l) {
        Some(r) => r,
        None => panic!("the lamp '{}' has an owner or is not in the list, and what feeds it writes its row: a beacon's spot and a camera's cone must each have a row", g.tab.light[l].name),
    }
}

/// The rows, stood once as the level is built, in the list's order: every lamp a fixture feeds gets one and no other lamp has a row.
pub fn light_home_stand(g: &mut crate::game::Game, list: &[LightId], targets: &[FixtureId], cones: &[(LightId, FixtureId)]) {
    let mut who: JsMap<LightId, (u32, FixtureId, f64)> = JsMap::new();
    for &t in targets {
        let f = &g.tab.fixture[t];
        if let Some(l) = f.light {
            who.set(l, (R_OWN, t, 0.0));
        }
        if let Some(area) = &f.area_lights {
            for a in area {
                who.set(a.light, (R_AREA, t, a.base));
            }
        }
    }
    for b in &g.beacons {
        let base = g.tab.light[b.light].peak_intensity;
        who.set(b.light, (R_BEACON, b.target, base));
    }
    for &(light, target) in cones {
        who.set(light, (R_CONE, target, 0.0));
    }
    let mut r: usize = 0;
    g.light_home.lamps = Vec::new();
    for &l in list {
        g.tab.light[l].row = None;
        let w = who.get(l).copied();
        let (role, t, base) = match w {
            Some(w) => w,
            None => continue,
        };
        if js::truthy(g.tab.light[l].owner) {
            panic!("the lamp '{}' has an owner, and a fixture feeds it: `skipOwner` is the question's, and only the question's own lamps have one", g.tab.light[l].name);
        }
        if r >= ROWS {
            panic!("a level of more than {} lamps on its fixtures got past the level's fit", ROWS);
        }
        g.tab.light[l].row = Some(r as f64);
        g.light_home.lamps.push(l);
        let i = t.0;
        let monitors = g.tab.fixture[t].group == "monitors";
        let day = crate::render_lights::light_by_day(&g.tab.light[l]) << sim::lights::R_DAY;
        let rows = lights(g);
        rows.row_base[r] = base;
        rows.row_target[r] = i;
        rows.row_bits[r] = role | if role == R_BEACON { R_PASSING } else { 0 } | if role == R_AREA && monitors { R_SCREEN } else { 0 } | day;
        light_home_feed(g, l);
        r += 1;
    }
    lights(g).rows = r as u32;
    // (a beacon's spot and a camera's cone must each have a row)
    for l in g.beacons.iter().map(|b| b.light).chain(cones.iter().map(|c| c.0)) { row_of(g, l); }
}

/// A lamp's whole record into its row, as the lamp stands. Nothing for a lamp with no row.
pub fn light_home_feed(g: &crate::game::Game, l: LightId) {
    let r = match lamp_row(g, l) {
        Some(r) => r,
        None => return,
    };
    let lamp = &g.tab.light[l];
    let c = lights(g);
    c.x[r] = lamp.pos[0];
    c.y[r] = lamp.pos[1];
    c.z[r] = lamp.pos[2];
    c.range[r] = lamp.range;
    c.radius[r] = lamp.radius;
    c.kind[r] = lamp.kind;
    c.dx[r] = lamp.dir[0];
    c.dy[r] = lamp.dir[1];
    c.dz[r] = lamp.dir[2];
    c.outer[r] = lamp.outer_deg;
    c.inner[r] = lamp.inner_deg;
    c.r[r] = lamp.color[0];
    c.g[r] = lamp.color[1];
    c.b[r] = lamp.color[2];
    c.intensity[r] = lamp.intensity;
    match &lamp.steady {
        Some(s) => {
            c.sx[r] = s.pos[0];
            c.sy[r] = s.pos[1];
            c.sz[r] = s.pos[2];
            c.s_radius[r] = s.radius;
            c.s_intensity[r] = s.intensity;
        }
        None => {
            c.sx[r] = 0.0;
            c.sy[r] = 0.0;
            c.sz[r] = 0.0;
            c.s_radius[r] = 0.0;
            c.s_intensity[r] = 0.0;
        }
    }
    // (a lamp that has come to burn as a fire does is one beside the hour, too)
    if lamp.steady.is_some() != (c.row_bits[r] & R_STEADY != 0) {
        c.row_bits[r] = (c.row_bits[r] & !(7 << sim::lights::R_DAY)) | crate::render_lights::light_by_day(lamp) << sim::lights::R_DAY;
    }
    c.row_bits[r] = (c.row_bits[r] & !(R_ON | R_STEADY)) | if lamp.enabled { R_ON } else { 0 } | if lamp.steady.is_some() { R_STEADY } else { 0 };
}

/// …of a lamp somebody moves each tick (a fire's): only while it burns; out, its record is the picture's alone.
pub(super) fn light_home_feed_burning(g: &crate::game::Game, l: LightId) {
    if g.tab.light[l].enabled {
        light_home_feed(g, l);
        return;
    }
    if let Some(r) = lamp_row(g, l) {
        lights(g).row_bits[r] &= !R_ON;
    }
}

/// what a hand left at a wall switch is over
fn no_hand(f: &mut Fixture) { f.hand_off = None; }

/// A wall switch's circuit flipped. On: what a hand put out; in a room no hand put out, the room as authored (all of it, where the level left the whole room dark), a
/// flat fixture striking as it comes back unless `strike` is down. Off: every one that is on, and the time on each. (Who saw it go is the caller's, which asks
/// before it calls: the factor does not move here.)
pub fn light_home_flip(g: &mut Game, targets: &[FixtureId], on: bool, time: f64, strike: bool) {
    let by_hand = targets.iter().any(|&t| { let f = &g.tab.fixture[t]; f.hand_off.is_some() && !f.on });
    let authored = targets.iter().any(|&t| g.tab.fixture[t].authored);
    for &t in targets {
        let f = &mut g.tab.fixture[t];
        if on {
            let mine = if by_hand { f.hand_off.is_some() } else { !authored || f.authored };
            if mine && !f.on {
                if strike && f.kind == FixtureKind::Emissive && !f.broken && f.blind_until < 0.0 { f.blind_until = time; }
                f.on = true;
            }
            no_hand(f);
        } else if f.on {
            f.on = false;
            f.hand_off = Some(time);
        }
    }
}

/// The mains tripped for `seconds` (never to come back by themselves where that is no number): None = they are already gone for good; else were they up.
pub(super) fn mains_trip(g: &mut Game, seconds: f64) -> Option<bool> {
    let time = g.time;
    let l = &mut g.light_home.mains;
    if l.active && l.permanent { return None; }
    let fresh = !l.active;
    if fresh { l.since = time; }
    let gone = !seconds.is_finite();
    l.until = if gone { f64::INFINITY } else { m::max(l.until, time + seconds) };
    (l.active, l.permanent) = (true, gone);
    Some(fresh)
}
/// The mains back: were they out.
pub(super) fn mains_restore(g: &mut Game) -> bool {
    let was = g.light_home.mains.active;
    if was { back(&mut g.light_home.mains, g.time); }
    was
}
fn back(l: &mut Mains, time: f64) { *l = Mains { active: false, permanent: false, until: -1.0, restored_at: time, fix_t: 0.0, ..*l }; }

/// 0 while a fixture has no supply, then the fluorescent restrike's flicker for 1.2 s after `dark`, then 1
fn supply(time: f64, dark: f64) -> f64 {
    if time < dark { return 0.0; }
    if time < dark + 1.2 {
        let x = (time - dark) / 1.2;
        let s = m::sin(time * 45.0) * m::sin(time * 13.7 + 1.3);
        return if s > 0.65 - x * 0.9 { 1.0 } else { 0.05 };
    }
    1.0
}
/// a bad tube's dip, a blink at a time, and what is left of it in one
const DIP: f64 = 0.15;
fn dip(time: f64) -> bool { m::sin(time * 3.1) + m::sin(time * 7.3 + 2.0) * 0.6 > 1.15 && m::sin(time * 90.0) > 0.0 }
/// a bad tube's steady self: the tube without its dips, for who asks what the place lights
fn tube(l: &mut Lights, r: usize, flicker: bool, time: f64) {
    if !flicker { return; }
    (l.sx[r], l.sy[r], l.sz[r], l.s_radius[r]) = (l.x[r], l.y[r], l.z[r], l.radius[r]);
    l.s_intensity[r] = if dip(time) { l.intensity[r] / DIP } else { l.intensity[r] };
    l.row_bits[r] |= R_STEADY;
}

/// what a tick of the lights did that the game has to answer for
pub(super) struct Stepped { pub ran_out: bool, pub hand: bool, pub fixed: bool }
/// ONE TICK OF THE LIGHTS: the mains' own clock and the timer at the panel, the beacons, each fixture's factor and each lamp as it burns. `near`: one of theirs is at
/// the panel.
pub(super) fn lights_step(g: &mut Game, dt: f64, near: bool, fumble: f64) -> Stepped {
    let time = g.time;
    let rows = lights(g);
    let mut ev = Stepped { ran_out: false, hand: false, fixed: false };
    let l = &mut g.light_home.mains;
    if l.active && !l.permanent && time >= l.until {
        back(l, time);
        ev.ran_out = true;
    }
    if l.active && !l.permanent && time - l.since > 3.0 {
        l.fix_t = if near { l.fix_t + dt } else { 0.0 };
        if near {
            ev.hand = true;
            if l.fix_t > fumble {
                back(l, time);
                ev.fixed = true;
            }
        }
    }
    let l = *l;
    // emergency beacons: spin up ~0.8 s after the trip, keep turning for a second after power returns; each is slaved to its dome fixture (whose factor is last frame's)
    let on = if l.active { time - l.since > 0.8 } else { time - l.restored_at < 1.2 };
    for b in &g.beacons {
        let f = &mut g.tab.fixture[b.target];
        f.on = on;
        let Some(r) = g.tab.light[b.light].row.map(|r| r as usize) else { continue };
        // (off it stands still at its own phase: nothing reads an off lamp's direction)
        let a = if on { time * 3.3 + b.phase } else { b.phase };
        let (c, s) = (m::cos(a), m::sin(a));
        let h = m::hyp3(c, -0.5, s);
        let h = if h != 0.0 { h } else { 1.0 };
        (rows.dx[r], rows.dy[r], rows.dz[r]) = (c / h, -0.5 / h, s / h);
        let peak = rows.row_base[r];
        rows.burn(r, on && f.factor > 0.0, peak);
    }
    for t in g.tab.fixture.ids() {
        let x = &mut g.tab.fixture[t];
        let mut f = if x.on && !x.broken { 1.0 } else { 0.0 };
        if f > 0.0 && x.blind_until > 0.0 {
            f *= supply(time, x.blind_until);
            if time > x.blind_until + 1.2 { x.blind_until = -1.0; }
        }
        if f > 0.0 && x.mains { f *= if l.active { 0.0 } else { supply(time, l.restored_at + x.stagger) }; }
        if f > 0.0 && x.flicker && dip(time) { f *= DIP; }
        x.factor = f;
    }
    for r in 0..rows.rows as usize {
        let role = rows.row_bits[r] & R_ROLE;
        if role != R_OWN && role != R_AREA { continue; }
        let x = &g.tab.fixture[FixtureId(rows.row_target[r])];
        let whole = if role == R_OWN { x.base_intensity } else { rows.row_base[r] };
        rows.burn(r, x.factor > 0.001, whole * x.factor);
        tube(rows, r, x.flicker, time);
    }
    ev
}

/// What the step worked out, to what the picture reads: a lamp's burning, and a beacon's turn.
pub(super) fn light_home_paint(g: &mut crate::game::Game) {
    for r in 0..g.light_home.lamps.len() {
        let c = lights_read(g);
        let b = c.row_bits[r];
        let role = b & R_ROLE;
        if role == R_CONE {
            continue;
        }
        let (i, dir) = (c.intensity[r], [c.dx[r], c.dy[r], c.dz[r]]);
        let l = g.light_home.lamps[r];
        let lamp = &mut g.tab.light[l];
        lamp.enabled = b & R_ON != 0;
        lamp.intensity = i;
        if role == R_AREA {
            lamp.peak_intensity = i;
        } else if role == R_BEACON {
            lamp.dir = dir;
        }
    }
}

/// a device works this frame: powered, unbroken, not knocked out
fn works(g: &Game, t: FixtureId) -> bool { let f = &g.tab.fixture[t]; f.factor > 0.05 && !f.broken }

/// THE CAMERA'S OWN HALF BEFORE ITS PICTURE: the pan, the cone it throws, the feed dropping once, and the picture emptying when it cannot see. `live`: devices sense.
/// `cones`: its cone is shown. Answers whether it works, and whether it has just gone down.
pub fn camera_pan(g: &mut Game, id: CameraRtId, dt: f64, live: bool, cones: bool) -> (bool, bool) {
    let (target, cone) = (g.tab.devices_camera_rt[id].target, g.tab.devices_camera_rt[id].cone);
    let (ok, broken) = (works(g, target), g.tab.fixture[target].broken);
    let c = &mut g.tab.devices_camera_rt[id];
    match c.sweep {
        Some([a, b]) => if ok {
            let u = c.phase + dt / m::max(0.5, c.period);
            c.phase = u - m::floor(u); // (u ≥ 0: the fraction is exact)
            let s = 0.5 - 0.5 * m::cos(c.phase * m::PI * 2.0);
            c.yaw = a + (b - a) * s;
        },
        None => c.yaw = c.rest,
    }
    let cp = m::cos(c.pitch);
    let look = [m::sin(c.yaw) * cp, m::sin(c.pitch), m::cos(c.yaw) * cp];
    if let Some(r) = g.tab.light[cone].row.map(|r| r as usize) {
        let (l, at) = (lights(g), g.tab.devices_camera_rt[id].def.pos);
        (l.dx[r], l.dy[r], l.dz[r]) = (look[0], look[1], look[2]);
        (l.x[r], l.y[r], l.z[r]) = (at[0] + look[0] * CONE_AHEAD, at[1] + look[1] * CONE_AHEAD, at[2] + look[2] * CONE_AHEAD);
        let i0 = l.intensity[r];
        l.burn(r, ok && cones, i0);
        let lamp = &mut g.tab.light[cone];
        (lamp.enabled, lamp.pos, lamp.dir) = (ok && cones, [l.x[r], l.y[r], l.z[r]], look);
    }
    let c = &mut g.tab.devices_camera_rt[id];
    let mut down = false;
    if broken && !c.dead {
        (c.dead, c.seen, c.state) = (true, 0.0, CameraState::Idle);
        down = true;
    } else if !broken {
        c.dead = false;
    }
    c.saw = false;
    if !ok || !live {
        c.seen = if ok { m::max(0.0, c.seen - CAMERA_DECAY * dt) } else { 0.0 };
        if !ok { c.state = CameraState::Idle; }
    }
    (ok, down)
}

/// …AND ITS HALF AFTER: what the picture had. `visible`: the eye test from the lens has him, at `gain` lit-seconds a second, with his feet at `p`. The integrator,
/// then what it does about it: is he suspected, is he reported.
pub fn camera_see(g: &mut Game, id: CameraRtId, dt: f64, visible: bool, gain: f64, p: V3) -> (bool, bool) {
    let (time, suspicion, alarm) = (g.time, g.tune.camera_suspicion_secs, g.tune.camera_alarm_secs);
    let c = &mut g.tab.devices_camera_rt[id];
    if visible && gain > 0.02 {
        c.seen = m::min(alarm + 1.0, c.seen + gain * dt);
        (c.last_seen, c.saw) = (Some(p), true);
    } else {
        c.seen = m::max(0.0, c.seen - CAMERA_DECAY * dt);
    }
    let (mut suspect, mut reported) = (false, false);
    if c.state == CameraState::Idle && c.seen >= suspicion && c.last_seen.is_some() {
        (c.state, c.reported_at, suspect) = (CameraState::Suspect, time, true);
    }
    if c.state != CameraState::Alarm && c.seen >= alarm && c.last_seen.is_some() && c.saw {
        (c.state, c.reported_at, reported) = (CameraState::Alarm, time, true);
    }
    if c.state != CameraState::Idle && c.seen <= 0.0 && time - c.reported_at > CAMERA_REARM_SECS { c.state = CameraState::Idle; }
    (suspect, reported)
}

/// THE TRIPWIRE'S OWN HALF BEFORE WHAT IS IN ITS BEAM: the feed dropping once, and no beam at all with no power. Answers whether it works, and whether it has just gone
/// down.
pub fn laser_open(g: &mut Game, id: LaserRtId) -> (bool, bool) {
    let t = g.tab.devices_laser_rt[id].target;
    let (ok, broken) = (works(g, t), g.tab.fixture[t].broken);
    let l = &mut g.tab.devices_laser_rt[id];
    let down = broken && !l.dead;
    l.dead = broken;
    if !ok { (l.cut, l.held) = (0.0, false); }
    (ok, down)
}
/// …AND AFTER: where the beam ends, and whether something breaks it. The trip is on the EDGE and not again inside the cooldown, and only while devices sense (`live`).
pub fn laser_close(g: &mut Game, id: LaserRtId, live: bool, cut: f64, held: bool) -> bool {
    let time = g.time;
    let l = &mut g.tab.devices_laser_rt[id];
    let edge = held && !l.held;
    (l.cut, l.held) = (cut, held);
    if !live || !edge || time - l.last_trip_at < LASER_COOLDOWN { return false; }
    (l.last_trip_at, l.trips) = (time, l.trips + 1);
    true
}
