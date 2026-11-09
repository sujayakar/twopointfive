//! src/sim/lightshome.ts lines 1 to 277
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

use ::sim::lights::{
    self as rs, Lights, BEACONS, CAMERAS, C_DEAD, C_LAST, C_SAW, C_SWEEP, LASERS, L_DEAD, L_HELD, M_ACTIVE, M_PERMANENT, ROWS, R_AREA, R_BEACON, R_CONE, R_ON,
    R_OWN, R_PASSING, R_ROLE, R_SCREEN, R_STEADY, S_ALARM, S_IDLE, S_SUSPECT, TARGETS, T_AUTHORED, T_BROKEN, T_EMISSIVE, T_FLICKER, T_HAND, T_MAINS, T_ON,
};

/// this game's part of rust/sim's block (no borrow of `g`: it may be held across a read of `g`, never across a call that writes the same part)
#[inline(always)]
fn lights(g: &Game) -> &'static mut Lights {
    &mut ::sim::state::state(g.sim).lights
}
/// the same part, to read
#[inline(always)]
fn lights_read(g: &Game) -> &'static Lights {
    &::sim::state::state_read(g.sim).lights
}
/// a fixture's place in the columns
#[inline(always)]
fn fix_i(g: &Game, id: FixtureId) -> usize {
    g.tab.sim_lightshome_fixture[id].i as usize
}
/// a camera's place in the columns
#[inline(always)]
fn cam_slot(g: &Game, id: CameraRtId) -> usize {
    g.tab.devices_camera_rt[id].slot as usize
}
/// a tripwire's place in the columns
#[inline(always)]
fn las_slot(g: &Game, id: LaserRtId) -> usize {
    g.tab.devices_laser_rt[id].slot as usize
}

/// a lamp's row, as an index (`RtLight.row`; undefined = none)
#[inline(always)]
fn lamp_row(g: &Game, l: LightId) -> Option<usize> {
    g.tab.light[l].row.map(|r| r as usize)
}

/// Where the columns stand is the block's own struct here: what is left of the TypeScript's reading of them is that the module and this file agree on how much a part holds. (lightshome.ts columns)
pub fn columns(g: &crate::game::Game) {
    let (cols, caps) = (light_cols().len(), light_caps());
    if rs::cap(14) != cols || rs::cap(0) as f64 != caps.targets || rs::cap(1) as f64 != caps.rows || rs::cap(2) as f64 != caps.beacons || rs::cap(3) as f64 != caps.cameras || rs::cap(4) as f64 != caps.lasers {
        panic!("rust/sim lays the lights out as {} columns and the rules as {}", rs::cap(14), cols);
    }
}

/// A fixture of a game: what the level says of it; its state is in the block. (lightshome.ts Fixture.constructor)
pub fn fixture_new(g: &mut crate::game::Game, i: f64, st: crate::Opaque, def: &crate::sim_lightshome::FixtureDef) -> FixtureId {
    let kind = match def.kind {
        FixtureDefKind::Analytic => FixtureKind::Analytic,
        FixtureDefKind::Emissive => FixtureKind::Emissive,
        FixtureDefKind::Breaker => FixtureKind::Breaker,
        FixtureDefKind::Device => FixtureKind::Device,
    };
    g.tab.sim_lightshome_fixture.push(Fixture {
        name: def.name.clone(),
        group: def.group.clone(),
        kind,
        pos: def.pos,
        light: def.light,
        boxes: def.boxes.clone(),
        base_emissive: def.base_emissive,
        mains: def.mains,
        interactive: def.interactive,
        area_lights: def.area_lights.as_ref().map(|v| v.iter().map(|a| FixtureAreaLightsItem { light: a.light, base: a.base }).collect()),
        lens: def.lens,
        i,
    })
}

/// its switch (lightshome.ts Fixture.on)
pub fn fixture_on(g: &crate::game::Game, id: FixtureId) -> bool {
    lights_read(g).bits[fix_i(g, id)] & T_ON != 0
}

/// (lightshome.ts Fixture.broken)
pub fn fixture_broken(g: &crate::game::Game, id: FixtureId) -> bool {
    lights_read(g).bits[fix_i(g, id)] & T_BROKEN != 0
}

/// an OCP's blind runs to here, and a strike's restrike from here; −1: neither (lightshome.ts Fixture.disabledUntil)
pub fn fixture_disabled_until(g: &crate::game::Game, id: FixtureId) -> f64 {
    lights_read(g).blind_until[fix_i(g, id)]
}

/// what it burns at this tick, 0‥1 (lightshome.ts Fixture.factor)
pub fn fixture_factor(g: &crate::game::Game, id: FixtureId) -> f64 {
    lights_read(g).factor[fix_i(g, id)]
}

/// the game time a hand put it out at its wall switch; None: no hand did (lightshome.ts Fixture.handOff)
pub fn fixture_hand_off(g: &crate::game::Game, id: FixtureId) -> Option<f64> {
    let (l, i) = (lights_read(g), fix_i(g, id));
    if l.bits[i] & T_HAND != 0 { Some(l.hand[i]) } else { None }
}

/// on in the level as authored (lightshome.ts Fixture.authored)
pub fn fixture_authored(g: &crate::game::Game, id: FixtureId) -> bool {
    lights_read(g).bits[fix_i(g, id)] & T_AUTHORED != 0
}

/// (lightshome.ts Fixture.baseIntensity)
pub fn fixture_base_intensity(g: &crate::game::Game, id: FixtureId) -> f64 {
    lights_read(g).base[fix_i(g, id)]
}

/// (lightshome.ts Fixture.stagger)
pub fn fixture_stagger(g: &crate::game::Game, id: FixtureId) -> f64 {
    lights_read(g).stagger[fix_i(g, id)]
}

/// (lightshome.ts Mains.active)
pub fn mains_active(g: &crate::game::Game) -> bool {
    lights_read(g).mains & M_ACTIVE != 0
}

/// (lightshome.ts Mains.permanent)
pub fn mains_permanent(g: &crate::game::Game) -> bool {
    lights_read(g).mains & M_PERMANENT != 0
}

/// (lightshome.ts Mains.until)
pub fn mains_until(g: &crate::game::Game) -> f64 {
    lights_read(g).until
}

/// (lightshome.ts Mains.since)
pub fn mains_since(g: &crate::game::Game) -> f64 {
    lights_read(g).since
}

/// (lightshome.ts Mains.restoredAt)
pub fn mains_restored_at(g: &crate::game::Game) -> f64 {
    lights_read(g).restored_at
}

/// how long a man of theirs has been at the panel (lightshome.ts Mains.fixT)
pub fn mains_fix_t(g: &crate::game::Game) -> f64 {
    lights_read(g).fix_t
}

/// A camera's own machine is the base of its CameraRt: this stands the row, whose other fields camera_rt_new fills. (lightshome.ts CameraMachine.constructor)
pub fn camera_machine_new(g: &mut crate::game::Game, slot: f64, st: crate::Opaque) -> CameraRtId {
    let mut row = crate::devices::CameraRt::blank();
    row.slot = slot;
    row.st = st;
    g.tab.devices_camera_rt.push(row)
}

/// the lens's yaw this frame (lightshome.ts CameraMachine.yaw)
pub fn camera_machine_yaw(g: &crate::game::Game, id: CameraRtId) -> f64 {
    lights_read(g).cam_yaw[cam_slot(g, id)]
}

/// the integrator: lit-seconds of Sam in the picture (lightshome.ts CameraMachine.seen)
pub fn camera_machine_seen(g: &crate::game::Game, id: CameraRtId) -> f64 {
    lights_read(g).cam_seen[cam_slot(g, id)]
}

/// idle → suspect → alarm, by the module's number (the enum's order is not that) (lightshome.ts CameraMachine.state)
pub fn camera_machine_state(g: &crate::game::Game, id: CameraRtId) -> crate::sim_lightshome::CameraState {
    match lights_read(g).cam_state[cam_slot(g, id)] {
        S_IDLE => CameraState::Idle,
        S_SUSPECT => CameraState::Suspect,
        S_ALARM => CameraState::Alarm,
        n => panic!("a camera in state {} of the module", n),
    }
}

/// (lightshome.ts CameraMachine.reportedAt)
pub fn camera_machine_reported_at(g: &crate::game::Game, id: CameraRtId) -> f64 {
    lights_read(g).cam_reported[cam_slot(g, id)]
}

/// where the picture last had him (lightshome.ts CameraMachine.lastSeen)
pub fn camera_machine_last_seen(g: &crate::game::Game, id: CameraRtId) -> Option<V3> {
    let (l, s) = (lights_read(g), cam_slot(g, id));
    if l.cam_bits[s] & C_LAST != 0 { Some([l.cam_lx[s], l.cam_ly[s], l.cam_lz[s]]) } else { None }
}

/// a dead camera has been called in, once per death (lightshome.ts CameraMachine.deadCalled)
pub fn camera_machine_dead_called(g: &crate::game::Game, id: CameraRtId) -> bool {
    lights_read(g).cam_bits[cam_slot(g, id)] & C_DEAD != 0
}

/// all of it as a file has it, or a fresh encounter (lightshome.ts CameraMachine.put)
pub fn camera_machine_put(g: &crate::game::Game, id: CameraRtId, yaw: f64, phase: f64, seen: f64, state: crate::sim_lightshome::CameraState, reported_at: f64, last_seen: Option<V3>, dead_called: bool) {
    let (l, s) = (lights(g), cam_slot(g, id));
    l.cam_yaw[s] = yaw;
    l.cam_phase[s] = phase;
    l.cam_seen[s] = seen;
    l.cam_reported[s] = reported_at;
    l.cam_state[s] = match state {
        CameraState::Idle => S_IDLE,
        CameraState::Suspect => S_SUSPECT,
        CameraState::Alarm => S_ALARM,
    };
    let at = last_seen.unwrap_or([0.0, 0.0, 0.0]);
    l.cam_lx[s] = at[0];
    l.cam_ly[s] = at[1];
    l.cam_lz[s] = at[2];
    l.cam_bits[s] = (l.cam_bits[s] & C_SWEEP) | if last_seen.is_some() { C_LAST } else { 0 } | if dead_called { C_DEAD } else { 0 };
}

/// A tripwire's own machine is the base of its LaserRt: this stands the row, whose other fields laser_rt_new fills. (lightshome.ts LaserMachine.constructor)
pub fn laser_machine_new(g: &mut crate::game::Game, slot: f64, st: crate::Opaque) -> LaserRtId {
    let mut row = crate::devices::LaserRt::blank();
    row.slot = slot;
    row.st = st;
    g.tab.devices_laser_rt.push(row)
}

/// (lightshome.ts LaserMachine.lastTripAt)
pub fn laser_machine_last_trip_at(g: &crate::game::Game, id: LaserRtId) -> f64 {
    lights_read(g).las_last[las_slot(g, id)]
}

/// (lightshome.ts LaserMachine.trips)
pub fn laser_machine_trips(g: &crate::game::Game, id: LaserRtId) -> f64 {
    lights_read(g).las_trips[las_slot(g, id)] as f64
}

/// drawn length this frame (lightshome.ts LaserMachine.cut)
pub fn laser_machine_cut(g: &crate::game::Game, id: LaserRtId) -> f64 {
    lights_read(g).las_cut[las_slot(g, id)]
}

/// (lightshome.ts LaserMachine.deadCalled)
pub fn laser_machine_dead_called(g: &crate::game::Game, id: LaserRtId) -> bool {
    lights_read(g).las_bits[las_slot(g, id)] & L_DEAD != 0
}

/// something is breaking it right now (lightshome.ts LaserMachine.held)
pub fn laser_machine_held(g: &crate::game::Game, id: LaserRtId) -> bool {
    lights_read(g).las_bits[las_slot(g, id)] & L_HELD != 0
}

/// the machine's own as a file has it, or a fresh encounter (lightshome.ts LaserMachine.put)
pub fn laser_machine_put(g: &crate::game::Game, id: LaserRtId, last_trip_at: f64, trips: f64, dead_called: bool, held: bool) {
    let (l, s) = (lights(g), las_slot(g, id));
    l.las_last[s] = last_trip_at;
    l.las_trips[s] = js::to_u32(trips);
    l.las_bits[s] = if dead_called { L_DEAD } else { 0 } | if held { L_HELD } else { 0 };
}

/// One game's lights: the block's part is this game's for its life, and the mains as a new level has them. (lightshome.ts LightHome.constructor)
pub fn light_home_new(g: &crate::game::Game, st: crate::Opaque) -> crate::sim_lightshome::LightHome {
    columns(g);
    let bit = LIGHTS_BIT as u32;
    let head = &mut ::sim::state::state(g.sim).head;
    if head.homes & bit != 0 {
        panic!("two stores of the lights over one game's block: a part of a block is one store's");
    }
    head.homes |= bit;
    let fresh = light_in().fresh;
    light_home_in(g, fresh, None, None, None, None, None);
    LightHome { lamps: Vec::new(), st }
}

/// A fixture more (the level as it is built). (lightshome.ts LightHome.add)
pub fn light_home_add(g: &mut crate::game::Game, def: &crate::sim_lightshome::FixtureDef) -> FixtureId {
    let l = lights(g);
    let n = l.n as usize;
    if n >= TARGETS {
        panic!("a level of more than {} light fixtures got past game/levelfit.ts", TARGETS);
    }
    l.n = (n + 1) as u32;
    l.bits[n] = if def.on { T_ON | T_AUTHORED } else { 0 }
        | if def.broken { T_BROKEN } else { 0 }
        | if def.fluorescent_flicker { T_FLICKER } else { 0 }
        | if def.mains { T_MAINS } else { 0 }
        | if def.kind == FixtureDefKind::Emissive { T_EMISSIVE } else { 0 };
    l.blind_until[n] = def.disabled_until;
    l.factor[n] = def.factor;
    l.stagger[n] = def.stagger;
    l.base[n] = def.base_intensity;
    let st = g.light_home.st;
    fixture_new(g, n as f64, st, def)
}

/// A camera more, at rest (`sweep`: its two yaws, none = it does not pan): its machine and the level's facts of it. (lightshome.ts LightHome.camera)
pub fn light_home_camera(g: &crate::game::Game, target: FixtureId, rest: f64, sweep: Option<[f64; 2]>, period: f64, pitch: f64, pos: V3) -> f64 {
    let t = fix_i(g, target) as u32;
    let l = lights(g);
    let s = l.cameras as usize;
    if s >= CAMERAS {
        panic!("a level of more than {} cameras got past game/levelfit.ts", CAMERAS);
    }
    l.cameras = (s + 1) as u32;
    l.cam_target[s] = t;
    l.cam_bits[s] = if sweep.is_some() { C_SWEEP } else { 0 };
    l.cam_rest[s] = rest;
    l.cam_a[s] = match sweep { Some(w) => w[0], None => 0.0 };
    l.cam_b[s] = match sweep { Some(w) => w[1], None => 0.0 };
    l.cam_period[s] = period;
    l.cam_pitch[s] = pitch;
    l.cam_px[s] = pos[0];
    l.cam_py[s] = pos[1];
    l.cam_pz[s] = pos[2];
    l.cam_yaw[s] = rest;
    l.cam_reported[s] = -100.0;
    s as f64
}

/// A tripwire more. (lightshome.ts LightHome.laser)
pub fn light_home_laser(g: &crate::game::Game, target: FixtureId, len: f64) -> f64 {
    let t = fix_i(g, target) as u32;
    let l = lights(g);
    let s = l.lasers as usize;
    if s >= LASERS {
        panic!("a level of more than {} tripwires got past game/levelfit.ts", LASERS);
    }
    l.lasers = (s + 1) as u32;
    l.las_target[s] = t;
    l.las_last[s] = -100.0;
    l.las_cut[s] = len;
    l.las_sense[s] = len;
    s as f64
}

/// a lamp's row, which the module writes: it must have one (lightshome.ts stand's rowOf)
fn row_of(g: &Game, l: LightId) -> u32 {
    match lamp_row(g, l) {
        Some(r) => r as u32,
        None => panic!("the lamp '{}' has an owner or is not in the list, and the module writes it: a beacon's spot and a camera's cone must each have a row", g.tab.light[l].name),
    }
}

/// The rows, stood once as the level is built, in the list's order: every lamp a fixture feeds gets one and no other lamp has a row. (lightshome.ts LightHome.stand)
pub fn light_home_stand(g: &mut crate::game::Game, list: &[LightId], targets: &[FixtureId], beacons: &[crate::sim_lightshome::BeaconFact], cones: &[crate::sim_lightshome::LightHomeStandConesItem]) {
    let mut who: JsMap<LightId, (u32, FixtureId, f64)> = JsMap::new();
    for &t in targets {
        let f = &g.tab.sim_lightshome_fixture[t];
        if let Some(l) = f.light {
            who.set(l, (R_OWN, t, 0.0));
        }
        if let Some(area) = &f.area_lights {
            for a in area {
                who.set(a.light, (R_AREA, t, a.base));
            }
        }
    }
    for b in beacons {
        let base = g.tab.light[b.light].peak_intensity;
        who.set(b.light, (R_BEACON, b.target, base));
    }
    for c in cones {
        who.set(c.light, (R_CONE, c.target, 0.0));
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
            panic!("a level of more than {} lamps on its fixtures got past game/levelfit.ts", ROWS);
        }
        g.tab.light[l].row = Some(r as f64);
        g.light_home.lamps.push(l);
        let i = fix_i(g, t) as u32;
        let monitors = g.tab.sim_lightshome_fixture[t].group == "monitors";
        let day = crate::render_lights::light_by_day(&g.tab.light[l]) << sim::lights::R_DAY;
        let rows = lights(g);
        rows.row_base[r] = base;
        rows.row_target[r] = i;
        rows.row_bits[r] = role | if role == R_BEACON { R_PASSING } else { 0 } | if role == R_AREA && monitors { R_SCREEN } else { 0 } | day;
        light_home_feed(g, l);
        r += 1;
    }
    lights(g).rows = r as u32;
    for (i, b) in beacons.iter().enumerate() {
        lights(g).beacon_phase[i] = b.phase;
        let row = row_of(g, b.light);
        let t = fix_i(g, b.target) as u32;
        let rows = lights(g);
        rows.beacon_row[i] = row;
        rows.beacon_target[i] = t;
    }
    lights(g).beacons = beacons.len() as u32;
    for c in cones {
        let row = row_of(g, c.light);
        lights(g).cam_row[c.slot as usize] = row;
    }
}

/// A lamp's whole record into its row, as the lamp stands. Nothing for a lamp with no row. (lightshome.ts LightHome.feed)
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

/// …of a lamp somebody moves each tick (a fire's): only while it burns; out, its record is the picture's alone. (lightshome.ts LightHome.feedBurning)
pub fn light_home_feed_burning(g: &crate::game::Game, l: LightId) {
    if g.tab.light[l].enabled {
        light_home_feed(g, l);
        return;
    }
    if let Some(r) = lamp_row(g, l) {
        lights(g).row_bits[r] &= !R_ON;
    }
}

/// Every event from outside: a kind, a fixture where it has one, its numbers. → the kind's own answer. (lightshome.ts LightHome.in)
pub fn light_home_in(g: &crate::game::Game, kind: f64, t: Option<FixtureId>, a: Option<f64>, b: Option<f64>, c_2: Option<f64>, d: Option<f64>) -> f64 {
    let i = match t {
        Some(t) => fix_i(g, t),
        None => 0,
    };
    rs::input(g.sim, kind as u32, i, a.unwrap_or(0.0), b.unwrap_or(0.0), c_2.unwrap_or(0.0), d.unwrap_or(0.0)) as f64
}

/// A wall switch's circuit flipped. (lightshome.ts LightHome.flip)
pub fn light_home_flip(g: &crate::game::Game, targets: &[FixtureId], on: bool, time: f64, strike: bool) {
    // rust/sim takes the circuit from its own scratch table, as the TypeScript writes it (one thread: see notes)
    let ids = unsafe { core::slice::from_raw_parts_mut(rs::ids_addr() as *mut u32, targets.len()) };
    for (i, &t) in targets.iter().enumerate() {
        ids[i] = fix_i(g, t) as u32;
    }
    rs::flip(g.sim, on, time, strike, targets.len());
}

/// What the module worked out, to what the picture reads: a lamp's burning, and a beacon's turn. (lightshome.ts LightHome.paint)
pub fn light_home_paint(g: &mut crate::game::Game) {
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

/// A camera before its picture, and its cone to what the picture reads. → CAM bits. (lightshome.ts LightHome.cameraPan)
pub fn light_home_camera_pan(g: &mut crate::game::Game, slot: f64, cone: LightId, dt: f64, live: bool, cones: bool) -> f64 {
    let out = rs::camera_pan(g.sim, slot as usize, dt, live, cones);
    if let Some(r) = lamp_row(g, cone) {
        let c = lights_read(g);
        let lamp = &mut g.tab.light[cone];
        lamp.enabled = c.row_bits[r] & R_ON != 0;
        lamp.pos = [c.x[r], c.y[r], c.z[r]];
        lamp.dir = [c.dx[r], c.dy[r], c.dz[r]];
    }
    out as f64
}

/// …and after it. → CAM bits. (lightshome.ts LightHome.cameraSee)
pub fn light_home_camera_see(g: &crate::game::Game, slot: f64, time: f64, dt: f64, visible: bool, gain: f64, p: V3, suspicion: f64, alarm: f64) -> f64 {
    rs::camera_see(g.sim, slot as usize, time, dt, visible, gain, p, suspicion, alarm) as f64
}

/// A tripwire before what is in its beam. → LASER bits. (lightshome.ts LightHome.laserOpen)
pub fn light_home_laser_open(g: &crate::game::Game, slot: f64) -> f64 {
    rs::laser_open(g.sim, slot as usize) as f64
}

/// …and after it. → LASER bits. (lightshome.ts LightHome.laserClose)
pub fn light_home_laser_close(g: &crate::game::Game, slot: f64, time: f64, live: bool, cut: f64, sense: f64, held: bool) -> f64 {
    rs::laser_close(g.sim, slot as usize, time, live, cut, sense, held) as f64
}

/// The block (a check's, and the reader's `rowsOf`): `st` is what the TypeScript's getter hands back; the slot it stands for is `g.sim`. (lightshome.ts LightHome.block)
pub fn light_home_block(g: &crate::game::Game) -> crate::Opaque {
    g.light_home.st
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate std;

    fn def(on: bool, kind: FixtureDefKind) -> FixtureDef {
        FixtureDef { on, kind, disabled_until: -1.0, factor: 0.5, stagger: 0.25, base_intensity: 7.0, mains: true, ..FixtureDef::default() }
    }

    /// what this file writes and reads of the columns, on one slot of its own (one test: a slot's block is one game's)
    #[test]
    fn the_columns_read_back_as_written() {
        std::thread::Builder::new().stack_size(64 << 20).spawn(|| {
            let mut g = Box::new(Game::blank());
            g.sim = 3;
            let g = &mut *g;

            g.light_home = light_home_new(g, crate::Opaque);
            assert_eq!(::sim::state::state(3).head.homes & 512, 512);
            assert_eq!(light_home_block(g), crate::Opaque);
            assert_eq!((mains_active(g), mains_until(g), mains_since(g), mains_restored_at(g)), (false, -1.0, -1.0, -100.0));

            let a = light_home_add(g, &def(true, FixtureDefKind::Analytic));
            let b = light_home_add(g, &def(false, FixtureDefKind::Emissive));
            assert_eq!((g.tab.sim_lightshome_fixture[a].i, g.tab.sim_lightshome_fixture[b].i), (0.0, 1.0));
            assert_eq!(g.tab.sim_lightshome_fixture[b].kind, FixtureKind::Emissive);
            assert_eq!((fixture_on(g, a), fixture_on(g, b)), (true, false));
            assert_eq!((lights(g).bits[0], lights(g).bits[1]), (T_ON | T_AUTHORED | T_MAINS, T_MAINS | T_EMISSIVE));
            assert_eq!((fixture_factor(g, a), fixture_stagger(g, a), fixture_base_intensity(g, a), fixture_disabled_until(g, a)), (0.5, 0.25, 7.0, -1.0));

            // a kind and a fixture reach the right row; the default numbers are zeros
            light_home_in(g, light_in().on, Some(b), Some(1.0), None, None, None);
            assert!(fixture_on(g, b));
            // on in the level is authored, by `add`; the event still says it of one fixture
            assert!(fixture_authored(g, a) && !fixture_authored(g, b));
            light_home_in(g, light_in().authored, Some(b), Some(1.0), None, None, None);
            light_home_in(g, light_in().authored, Some(a), Some(0.0), None, None, None);
            assert!(fixture_authored(g, b) && !fixture_authored(g, a));

            // the circuit reaches the module by its scratch table
            light_home_flip(g, &[a, b], false, 12.5, true);
            assert_eq!((fixture_on(g, a), fixture_on(g, b), fixture_hand_off(g, a), fixture_hand_off(g, b)), (false, false, Some(12.5), Some(12.5)));

            // a camera: the slot is what is handed back, a sweep's yaws are kept, the bits survive a `put`
            let s = light_home_camera(g, b, 0.75, Some([0.1, 0.2]), 6.0, -0.3, [1.0, 2.0, 3.0]);
            assert_eq!(s, 0.0);
            assert_eq!(lights(g).cam_target[0], 1);
            let c = camera_machine_new(g, s, crate::Opaque);
            assert_eq!((camera_machine_yaw(g, c), camera_machine_reported_at(g, c), camera_machine_state(g, c)), (0.75, -100.0, CameraState::Idle));
            camera_machine_put(g, c, 1.0, 0.5, 2.0, CameraState::Alarm, 9.0, Some([4.0, 5.0, 6.0]), true);
            assert_eq!(lights(g).cam_state[0], S_ALARM);
            assert_eq!(camera_machine_state(g, c), CameraState::Alarm);
            assert_eq!(camera_machine_last_seen(g, c), Some([4.0, 5.0, 6.0]));
            assert!(camera_machine_dead_called(g, c));
            assert_eq!(lights(g).cam_bits[0], C_SWEEP | C_LAST | C_DEAD);
            camera_machine_put(g, c, 1.0, 0.5, 2.0, CameraState::Suspect, 9.0, None, false);
            assert_eq!(camera_machine_state(g, c), CameraState::Suspect);
            assert_eq!(camera_machine_last_seen(g, c), None);
            assert_eq!(lights(g).cam_bits[0], C_SWEEP);

            // a tripwire
            let s = light_home_laser(g, a, 4.5);
            let t = laser_machine_new(g, s, crate::Opaque);
            assert_eq!((laser_machine_last_trip_at(g, t), laser_machine_cut(g, t), lights(g).las_sense[0]), (-100.0, 4.5, 4.5));
            laser_machine_put(g, t, 3.0, 2.0, true, false);
            assert_eq!((laser_machine_trips(g, t), laser_machine_dead_called(g, t), laser_machine_held(g, t)), (2.0, true, false));
            laser_machine_put(g, t, 3.0, 2.0, false, true);
            assert_eq!((laser_machine_dead_called(g, t), laser_machine_held(g, t)), (false, true));
        }).unwrap().join().unwrap();
    }

    /// a lamp of the list, as a level's lights hang it
    fn lamp(g: &mut Game, name: &str, on: bool) -> LightId {
        let mut l = crate::render_lights::RtLight::default();
        l.name = String::from(name);
        l.enabled = on;
        l.ttl = -1.0;
        g.tab.light.push(l)
    }

    /// what a row's bits say beside what the lamp is beside an hour · that
    fn role(b: u32) -> u32 { b & !(7 << sim::lights::R_DAY) }
    fn day(b: u32) -> u32 { b >> sim::lights::R_DAY & 7 }

    /// the rows: stood in the list's order, written from the lamp's record, and painted back (lightshome.ts stand, feed, feedBurning, paint, cameraPan's cone)
    #[test]
    fn the_rows_stand_and_paint() {
        std::thread::Builder::new().stack_size(64 << 20).spawn(|| {
            let mut g = Box::new(Game::blank());
            g.sim = 2;
            let g = &mut *g;
            g.light_home = light_home_new(g, crate::Opaque);

            let (own, panel, none, owned, spot, cone) = (lamp(g, "own", true), lamp(g, "panel", true), lamp(g, "none", true), lamp(g, "owned", true), lamp(g, "spot", true), lamp(g, "cone", true));
            g.tab.light[owned].owner = 7.0;
            g.tab.light[spot].peak_intensity = 9.0;
            g.tab.light[own].pos = [1.0, 2.0, 3.0];
            g.tab.light[own].intensity = 5.0;
            g.tab.light[own].steady = Some(crate::render_lights::RtLightSteady { intensity: 4.0, pos: [6.0, 7.0, 8.0], radius: 0.5, ..crate::render_lights::RtLightSteady::default() });

            let a = light_home_add(g, &FixtureDef { light: Some(own), ..def(true, FixtureDefKind::Analytic) });
            let b = light_home_add(g, &FixtureDef { group: String::from("monitors"), area_lights: Some(vec![FixtureDefAreaLightsItem { light: panel, base: 2.5 }]), ..def(true, FixtureDefKind::Emissive) });
            let c = light_home_add(g, &def(true, FixtureDefKind::Analytic));
            let s = light_home_camera(g, c, 0.0, None, 6.0, 0.0, [0.0, 0.0, 0.0]);
            let (fa, fb, fc) = (&g.tab.sim_lightshome_fixture[a], &g.tab.sim_lightshome_fixture[b], &g.tab.sim_lightshome_fixture[c]);
            assert!(fa.area_lights.is_none() && fb.area_lights.is_some() && fc.light.is_none());

            // the list's order deals the rows, not the roles' order; a lamp nothing feeds has none, owner or not
            light_home_stand(g, &[spot, none, own, owned, panel, cone], &[a, b, c], &[BeaconFact { light: spot, phase: 0.25, target: a }], &[LightHomeStandConesItem { light: cone, target: c, slot: s }]);
            let rows = |g: &Game, l: LightId| g.tab.light[l].row;
            assert_eq!((rows(g, spot), rows(g, none), rows(g, own), rows(g, owned), rows(g, panel), rows(g, cone)), (Some(0.0), None, Some(1.0), None, Some(2.0), Some(3.0)));
            assert_eq!(g.light_home.lamps, vec![spot, own, panel, cone]);

            let l = lights(g);
            assert_eq!(l.rows, 4);
            assert_eq!(role(l.row_bits[0]), R_BEACON | R_PASSING | R_ON);
            assert_eq!(role(l.row_bits[1]), R_OWN | R_ON | R_STEADY);
            assert_eq!(role(l.row_bits[2]), R_AREA | R_SCREEN | R_ON);
            assert_eq!(role(l.row_bits[3]), R_CONE | R_ON);
            // (beside an hour, a lamp with a steady self burns as a fire does)
            assert_eq!((day(l.row_bits[0]), day(l.row_bits[1])), (sim::light::DAY_FIXTURE, sim::light::DAY_FLASH));
            assert_eq!((l.row_base[0], l.row_base[1], l.row_base[2], l.row_base[3]), (9.0, 0.0, 2.5, 0.0));
            assert_eq!((l.row_target[0], l.row_target[1], l.row_target[2], l.row_target[3]), (0, 0, 1, 2));
            assert_eq!((l.x[1], l.y[1], l.z[1], l.intensity[1]), (1.0, 2.0, 3.0, 5.0));
            assert_eq!((l.sx[1], l.sy[1], l.sz[1], l.s_radius[1], l.s_intensity[1]), (6.0, 7.0, 8.0, 0.5, 4.0));
            assert_eq!((l.beacons, l.beacon_phase[0], l.beacon_row[0], l.beacon_target[0], l.cam_row[0]), (1, 0.25, 0, 0, 3));

            // a lamp out: its record stays out of the block, and only the row's bit goes down
            g.tab.light[own].enabled = false;
            g.tab.light[own].intensity = 99.0;
            light_home_feed_burning(g, own);
            let l = lights(g);
            assert_eq!((role(l.row_bits[1]), l.intensity[1]), (R_OWN | R_STEADY, 5.0));
            // lit again: the whole record; the steady self gone takes its columns to zeros and its bit down
            g.tab.light[own].enabled = true;
            g.tab.light[own].steady = None;
            light_home_feed_burning(g, own);
            let l = lights(g);
            assert_eq!((role(l.row_bits[1]), l.intensity[1], l.sx[1], l.s_intensity[1]), (R_OWN | R_ON, 99.0, 0.0, 0.0));
            assert_eq!(day(l.row_bits[1]), sim::light::DAY_FIXTURE);
            // a lamp with no row: nothing, and no fault
            light_home_feed(g, none);
            light_home_feed_burning(g, none);

            // paint: what the module worked out goes to the lamps, by role; a cone's row is not painted
            let l = lights(g);
            l.intensity[0] = 11.0;
            l.dx[0] = 0.5;
            l.dy[0] = 0.25;
            l.dz[0] = 0.75;
            l.intensity[1] = 12.0;
            l.dx[1] = 0.125;
            l.intensity[2] = 13.0;
            l.row_bits[2] &= !R_ON;
            l.intensity[3] = 14.0;
            g.tab.light[own].dir = [0.0, -1.0, 0.0];
            g.tab.light[own].peak_intensity = 1.0;
            light_home_paint(g);
            assert_eq!((g.tab.light[spot].intensity, g.tab.light[spot].dir, g.tab.light[spot].peak_intensity), (11.0, [0.5, 0.25, 0.75], 9.0));
            assert_eq!((g.tab.light[own].intensity, g.tab.light[own].dir, g.tab.light[own].peak_intensity), (12.0, [0.0, -1.0, 0.0], 1.0));
            assert_eq!((g.tab.light[panel].intensity, g.tab.light[panel].peak_intensity, g.tab.light[panel].enabled), (13.0, 13.0, false));
            assert_eq!(g.tab.light[cone].intensity, 0.0);

            // the cone copy: what camera_pan left in the row reaches the lamp; a cone with no row stays as it stood
            let bits = light_home_camera_pan(g, s, cone, 0.0, true, true);
            let l = lights(g);
            assert_eq!((g.tab.light[cone].enabled, g.tab.light[cone].pos, g.tab.light[cone].dir), (l.row_bits[3] & R_ON != 0, [l.x[3], l.y[3], l.z[3]], [l.dx[3], l.dy[3], l.dz[3]]));
            assert_eq!(g.tab.light[cone].dir, [0.0, 0.0, 1.0]);
            g.tab.light[none].dir = [1.0, 0.0, 0.0];
            assert_eq!(light_home_camera_pan(g, s, none, 0.0, true, true), bits);
            assert_eq!(g.tab.light[none].dir, [1.0, 0.0, 0.0]);
        }).unwrap().join().unwrap();
    }

    /// a fixture's lamp with an owner, or a cone with no row, is a fault by name, not a lamp left out (lightshome.ts:189, :197; slots 0 and 1: the block has four, each one game's)
    #[test]
    fn stand_refuses_by_name() {
        fn said(slot: usize, make: fn(&mut Game)) -> String {
            let e = std::thread::Builder::new().stack_size(64 << 20).spawn(move || {
                let mut g = Box::new(Game::blank());
                g.sim = slot;
                let g = &mut *g;
                g.light_home = light_home_new(g, crate::Opaque);
                make(g);
            }).unwrap().join().unwrap_err();
            e.downcast_ref::<String>().unwrap().clone()
        }
        let m = said(0, |g| {
            let l = lamp(g, "owned", true);
            g.tab.light[l].owner = 7.0;
            let f = light_home_add(g, &FixtureDef { light: Some(l), ..def(true, FixtureDefKind::Analytic) });
            light_home_stand(g, &[l], &[f], &[], &[]);
        });
        assert!(m.starts_with("the lamp 'owned' has an owner, and a fixture feeds it"), "{}", m);
        let m = said(1, |g| {
            let l = lamp(g, "lost", true);
            let f = light_home_add(g, &def(true, FixtureDefKind::Device));
            light_home_stand(g, &[], &[f], &[], &[LightHomeStandConesItem { light: l, target: f, slot: 0.0 }]);
        });
        assert!(m.starts_with("the lamp 'lost' has an owner or is not in the list"), "{}", m);
    }
}
