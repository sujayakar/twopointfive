//! src/sim/lightshome.ts lines 1 to 276
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

use ::sim::lights::{
    self as rs, Lights, BEACONS, CAMERAS, C_DEAD, C_LAST, C_SAW, C_SWEEP, LASERS, L_DEAD, L_HELD, M_ACTIVE, M_PERMANENT, ROWS, R_AREA, R_BEACON, R_CONE, R_ON,
    R_OWN, R_PASSING, R_ROLE, R_SCREEN, R_STEADY, S_ALARM, S_IDLE, S_SUSPECT, TARGETS, T_AUTHORED, T_BROKEN, T_EMISSIVE, T_FLICKER, T_HAND, T_MAINS, T_ON,
};

/// this game's part of rust/sim's block (no borrow of `g`: it may be held across a read of `g`, never across a call that writes the same part)
#[inline(always)]
fn lights(g: &Game) -> &'static mut Lights {
    &mut ::sim::state::state(g.sim).lights
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

// ---- the seams: what the skeleton has no way to reach yet. A lamp (`RtLight`) is a bare `LightId` with no record behind it (notes.md, ## Skeleton).
/// (lightshome.ts:188,190,204,216,252 `l.row`) undefined = none
fn seam_lamp_row(g: &mut Game, l: LightId) -> Option<usize> { todo!("needs: RtLight.row, a lamp's record") }
/// (lightshome.ts:188,190 `l.row = …`)
fn seam_lamp_set_row(g: &mut Game, l: LightId, row: Option<usize>) { todo!("needs: RtLight.row, a lamp's record") }
/// (lightshome.ts:188 `l.owner`)
fn seam_lamp_owner(g: &mut Game, l: LightId) -> f64 { todo!("needs: RtLight.owner, a lamp's record") }
/// (lightshome.ts:196 `l.name`, in the error)
fn seam_lamp_name(g: &mut Game, l: LightId) -> String { todo!("needs: RtLight.name, a lamp's record") }
/// (lightshome.ts:184 `b.light.peakIntensity`)
fn seam_lamp_peak_intensity(g: &mut Game, l: LightId) -> f64 { todo!("needs: RtLight.peakIntensity, a lamp's record") }
/// (lightshome.ts:245 `l.peakIntensity = …`)
fn seam_lamp_set_peak_intensity(g: &mut Game, l: LightId, x: f64) { todo!("needs: RtLight.peakIntensity, a lamp's record") }
/// (lightshome.ts:206 `l.pos`)
fn seam_lamp_pos(g: &mut Game, l: LightId) -> V3 { todo!("needs: RtLight.pos, a lamp's record") }
/// (lightshome.ts:252 `cone.pos = …`)
fn seam_lamp_set_pos(g: &mut Game, l: LightId, x: V3) { todo!("needs: RtLight.pos, a lamp's record") }
/// (lightshome.ts:206 `l.range`)
fn seam_lamp_range(g: &mut Game, l: LightId) -> f64 { todo!("needs: RtLight.range, a lamp's record") }
/// (lightshome.ts:206 `l.radius`)
fn seam_lamp_radius(g: &mut Game, l: LightId) -> f64 { todo!("needs: RtLight.radius, a lamp's record") }
/// (lightshome.ts:206 `l.kind`)
fn seam_lamp_kind(g: &mut Game, l: LightId) -> f64 { todo!("needs: RtLight.kind, a lamp's record") }
/// (lightshome.ts:207 `l.dir`)
fn seam_lamp_dir(g: &mut Game, l: LightId) -> V3 { todo!("needs: RtLight.dir, a lamp's record") }
/// (lightshome.ts:246,252 `l.dir = …`)
fn seam_lamp_set_dir(g: &mut Game, l: LightId, x: V3) { todo!("needs: RtLight.dir, a lamp's record") }
/// (lightshome.ts:207 `l.outerDeg`)
fn seam_lamp_outer_deg(g: &mut Game, l: LightId) -> f64 { todo!("needs: RtLight.outerDeg, a lamp's record") }
/// (lightshome.ts:207 `l.innerDeg`)
fn seam_lamp_inner_deg(g: &mut Game, l: LightId) -> f64 { todo!("needs: RtLight.innerDeg, a lamp's record") }
/// (lightshome.ts:208 `l.color`)
fn seam_lamp_color(g: &mut Game, l: LightId) -> V3 { todo!("needs: RtLight.color, a lamp's record") }
/// (lightshome.ts:208 `l.intensity`)
fn seam_lamp_intensity(g: &mut Game, l: LightId) -> f64 { todo!("needs: RtLight.intensity, a lamp's record") }
/// (lightshome.ts:244 `l.intensity = …`)
fn seam_lamp_set_intensity(g: &mut Game, l: LightId, x: f64) { todo!("needs: RtLight.intensity, a lamp's record") }
/// (lightshome.ts:210,215 `l.enabled`)
fn seam_lamp_enabled(g: &mut Game, l: LightId) -> bool { todo!("needs: RtLight.enabled, a lamp's record") }
/// (lightshome.ts:244,252 `l.enabled = …`)
fn seam_lamp_set_enabled(g: &mut Game, l: LightId, x: bool) { todo!("needs: RtLight.enabled, a lamp's record") }
/// (lightshome.ts:205 `l.steady`) its intensity, pos, radius
fn seam_lamp_steady(g: &mut Game, l: LightId) -> Option<(f64, V3, f64)> { todo!("needs: RtLight.steady, a lamp's record") }

/// Where the columns stand is the block's own struct here: what is left of the TypeScript's reading of them is that the module and this file agree on how much a part holds. (lightshome.ts columns)
pub fn columns(g: &mut crate::game::Game) {
    let (cols, caps) = (light_cols().len(), light_caps());
    if rs::cap(14) != cols || rs::cap(0) as f64 != caps.targets || rs::cap(1) as f64 != caps.rows || rs::cap(2) as f64 != caps.beacons || rs::cap(3) as f64 != caps.cameras || rs::cap(4) as f64 != caps.lasers {
        panic!("the Rust module lays the lights out as {} columns and src/sim/lightshome.ts as {}: bun run tools/sim/build.ts", rs::cap(14), cols);
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
        fluorescent_flicker: def.fluorescent_flicker,
        mains: def.mains,
        interactive: def.interactive,
        area_lights: def.area_lights.as_ref().map(|v| v.iter().map(|a| FixtureAreaLightsItem { light: a.light, base: a.base }).collect()),
        lens: def.lens,
        i,
        st,
    })
}

/// its switch (lightshome.ts Fixture.on)
pub fn fixture_on(g: &mut crate::game::Game, id: FixtureId) -> bool {
    lights(g).bits[fix_i(g, id)] & T_ON != 0
}

/// (lightshome.ts Fixture.broken)
pub fn fixture_broken(g: &mut crate::game::Game, id: FixtureId) -> bool {
    lights(g).bits[fix_i(g, id)] & T_BROKEN != 0
}

/// an OCP's blind runs to here, and a strike's restrike from here; −1: neither (lightshome.ts Fixture.disabledUntil)
pub fn fixture_disabled_until(g: &mut crate::game::Game, id: FixtureId) -> f64 {
    lights(g).blind_until[fix_i(g, id)]
}

/// what it burns at this tick, 0‥1 (lightshome.ts Fixture.factor)
pub fn fixture_factor(g: &mut crate::game::Game, id: FixtureId) -> f64 {
    lights(g).factor[fix_i(g, id)]
}

/// the game time a hand put it out at its wall switch; None: no hand did (lightshome.ts Fixture.handOff)
pub fn fixture_hand_off(g: &mut crate::game::Game, id: FixtureId) -> Option<f64> {
    let (l, i) = (lights(g), fix_i(g, id));
    if l.bits[i] & T_HAND != 0 { Some(l.hand[i]) } else { None }
}

/// on in the level as authored (lightshome.ts Fixture.authored)
pub fn fixture_authored(g: &mut crate::game::Game, id: FixtureId) -> bool {
    lights(g).bits[fix_i(g, id)] & T_AUTHORED != 0
}

/// (lightshome.ts Fixture.baseIntensity)
pub fn fixture_base_intensity(g: &mut crate::game::Game, id: FixtureId) -> f64 {
    lights(g).base[fix_i(g, id)]
}

/// (lightshome.ts Fixture.stagger)
pub fn fixture_stagger(g: &mut crate::game::Game, id: FixtureId) -> f64 {
    lights(g).stagger[fix_i(g, id)]
}

/// The mains, read. (lightshome.ts Mains.constructor)
pub fn mains_new(g: &mut crate::game::Game, st: crate::Opaque) -> crate::sim_lightshome::Mains {
    Mains { st }
}

/// (lightshome.ts Mains.active)
pub fn mains_active(g: &mut crate::game::Game) -> bool {
    lights(g).mains & M_ACTIVE != 0
}

/// (lightshome.ts Mains.permanent)
pub fn mains_permanent(g: &mut crate::game::Game) -> bool {
    lights(g).mains & M_PERMANENT != 0
}

/// (lightshome.ts Mains.until)
pub fn mains_until(g: &mut crate::game::Game) -> f64 {
    lights(g).until
}

/// (lightshome.ts Mains.since)
pub fn mains_since(g: &mut crate::game::Game) -> f64 {
    lights(g).since
}

/// (lightshome.ts Mains.restoredAt)
pub fn mains_restored_at(g: &mut crate::game::Game) -> f64 {
    lights(g).restored_at
}

/// how long a man of theirs has been at the panel (lightshome.ts Mains.fixT)
pub fn mains_fix_t(g: &mut crate::game::Game) -> f64 {
    lights(g).fix_t
}

/// A camera's own machine is the base of its CameraRt: this stands the row, whose other fields camera_rt_new fills. (lightshome.ts CameraMachine.constructor)
pub fn camera_machine_new(g: &mut crate::game::Game, slot: f64, st: crate::Opaque) -> CameraRtId {
    let mut row = crate::devices::CameraRt::blank();
    row.slot = slot;
    row.st = st;
    g.tab.devices_camera_rt.push(row)
}

/// the lens's yaw this frame (lightshome.ts CameraMachine.yaw)
pub fn camera_machine_yaw(g: &mut crate::game::Game, id: CameraRtId) -> f64 {
    lights(g).cam_yaw[cam_slot(g, id)]
}

/// the pan's phase 0‥1 (lightshome.ts CameraMachine.phase)
pub fn camera_machine_phase(g: &mut crate::game::Game, id: CameraRtId) -> f64 {
    lights(g).cam_phase[cam_slot(g, id)]
}

/// the integrator: lit-seconds of Sam in the picture (lightshome.ts CameraMachine.seen)
pub fn camera_machine_seen(g: &mut crate::game::Game, id: CameraRtId) -> f64 {
    lights(g).cam_seen[cam_slot(g, id)]
}

/// idle → suspect → alarm, by the module's number (the enum's order is not that) (lightshome.ts CameraMachine.state)
pub fn camera_machine_state(g: &mut crate::game::Game, id: CameraRtId) -> crate::sim_lightshome::CameraState {
    match lights(g).cam_state[cam_slot(g, id)] {
        S_IDLE => CameraState::Idle,
        S_SUSPECT => CameraState::Suspect,
        S_ALARM => CameraState::Alarm,
        n => panic!("a camera in state {} of the module", n),
    }
}

/// (lightshome.ts CameraMachine.reportedAt)
pub fn camera_machine_reported_at(g: &mut crate::game::Game, id: CameraRtId) -> f64 {
    lights(g).cam_reported[cam_slot(g, id)]
}

/// where the picture last had him (lightshome.ts CameraMachine.lastSeen)
pub fn camera_machine_last_seen(g: &mut crate::game::Game, id: CameraRtId) -> Option<V3> {
    let (l, s) = (lights(g), cam_slot(g, id));
    if l.cam_bits[s] & C_LAST != 0 { Some([l.cam_lx[s], l.cam_ly[s], l.cam_lz[s]]) } else { None }
}

/// (lightshome.ts CameraMachine.sawThisFrame)
pub fn camera_machine_saw_this_frame(g: &mut crate::game::Game, id: CameraRtId) -> bool {
    lights(g).cam_bits[cam_slot(g, id)] & C_SAW != 0
}

/// a dead camera has been called in, once per death (lightshome.ts CameraMachine.deadCalled)
pub fn camera_machine_dead_called(g: &mut crate::game::Game, id: CameraRtId) -> bool {
    lights(g).cam_bits[cam_slot(g, id)] & C_DEAD != 0
}

/// all of it as a file has it, or a fresh encounter (lightshome.ts CameraMachine.put)
pub fn camera_machine_put(g: &mut crate::game::Game, id: CameraRtId, yaw: f64, phase: f64, seen: f64, state: crate::sim_lightshome::CameraState, reported_at: f64, last_seen: Option<V3>, dead_called: bool) {
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
pub fn laser_machine_last_trip_at(g: &mut crate::game::Game, id: LaserRtId) -> f64 {
    lights(g).las_last[las_slot(g, id)]
}

/// (lightshome.ts LaserMachine.trips)
pub fn laser_machine_trips(g: &mut crate::game::Game, id: LaserRtId) -> f64 {
    lights(g).las_trips[las_slot(g, id)] as f64
}

/// drawn length this frame (lightshome.ts LaserMachine.cut)
pub fn laser_machine_cut(g: &mut crate::game::Game, id: LaserRtId) -> f64 {
    lights(g).las_cut[las_slot(g, id)]
}

/// (lightshome.ts LaserMachine.deadCalled)
pub fn laser_machine_dead_called(g: &mut crate::game::Game, id: LaserRtId) -> bool {
    lights(g).las_bits[las_slot(g, id)] & L_DEAD != 0
}

/// something is breaking it right now (lightshome.ts LaserMachine.held)
pub fn laser_machine_held(g: &mut crate::game::Game, id: LaserRtId) -> bool {
    lights(g).las_bits[las_slot(g, id)] & L_HELD != 0
}

/// the machine's own as a file has it, or a fresh encounter (lightshome.ts LaserMachine.put)
pub fn laser_machine_put(g: &mut crate::game::Game, id: LaserRtId, last_trip_at: f64, trips: f64, dead_called: bool, held: bool) {
    let (l, s) = (lights(g), las_slot(g, id));
    l.las_last[s] = last_trip_at;
    l.las_trips[s] = js::to_u32(trips);
    l.las_bits[s] = if dead_called { L_DEAD } else { 0 } | if held { L_HELD } else { 0 };
}

/// One game's lights: the block's part is this game's for its life, and the mains as a new level has them. (lightshome.ts LightHome.constructor)
pub fn light_home_new(g: &mut crate::game::Game, st: crate::Opaque) -> crate::sim_lightshome::LightHome {
    columns(g);
    let bit = LIGHTS_BIT as u32;
    let head = &mut ::sim::state::state(g.sim).head;
    if head.homes & bit != 0 {
        panic!("two stores of the lights over one game's block: a part of a block is one store's");
    }
    head.homes |= bit;
    let mains = mains_new(g, st);
    let fresh = light_in().fresh;
    light_home_in(g, fresh, None, None, None, None, None);
    LightHome { mains, lamps: Vec::new(), st }
}

/// A fixture more (the level as it is built). (lightshome.ts LightHome.add)
pub fn light_home_add(g: &mut crate::game::Game, def: &crate::sim_lightshome::FixtureDef) -> FixtureId {
    let l = lights(g);
    let n = l.n as usize;
    if n >= TARGETS {
        panic!("a level of more than {} light fixtures got past game/levelfit.ts", TARGETS);
    }
    l.n = (n + 1) as u32;
    l.bits[n] = if def.on { T_ON } else { 0 }
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
pub fn light_home_camera(g: &mut crate::game::Game, target: FixtureId, rest: f64, sweep: Option<[f64; 2]>, period: f64, pitch: f64, pos: V3) -> f64 {
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
pub fn light_home_laser(g: &mut crate::game::Game, target: FixtureId, len: f64) -> f64 {
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
fn row_of(g: &mut Game, l: LightId) -> u32 {
    match seam_lamp_row(g, l) {
        Some(r) => r as u32,
        None => {
            let name = seam_lamp_name(g, l);
            panic!("the lamp '{}' has an owner or is not in the list, and the module writes it: a beacon's spot and a camera's cone must each have a row", name);
        }
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
        let base = seam_lamp_peak_intensity(g, b.light);
        who.set(b.light, (R_BEACON, b.target, base));
    }
    for c in cones {
        if let Some(l) = c.light {
            who.set(l, (R_CONE, c.target, 0.0));
        }
    }
    let mut r: usize = 0;
    g.light_home.lamps = Vec::new();
    for &l in list {
        seam_lamp_set_row(g, l, None);
        let w = who.get(l).copied();
        let (role, t, base) = match w {
            Some(w) => w,
            None => continue,
        };
        if js::truthy(seam_lamp_owner(g, l)) {
            continue; // (a lamp with an owner gets no row: `skipOwner` is the question's, and only the question's own lamps have one)
        }
        if r >= ROWS {
            panic!("a level of more than {} lamps on its fixtures got past game/levelfit.ts", ROWS);
        }
        seam_lamp_set_row(g, l, Some(r));
        g.light_home.lamps.push(l);
        let i = fix_i(g, t) as u32;
        let monitors = g.tab.sim_lightshome_fixture[t].group == "monitors";
        let rows = lights(g);
        rows.row_base[r] = base;
        rows.row_target[r] = i;
        rows.row_bits[r] = role | if role == R_BEACON { R_PASSING } else { 0 } | if role == R_AREA && monitors { R_SCREEN } else { 0 };
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
        if let Some(l) = c.light {
            let row = row_of(g, l);
            lights(g).cam_row[c.slot as usize] = row;
        }
    }
}

/// A lamp's whole record into its row, as the lamp stands. Nothing for a lamp with no row. (lightshome.ts LightHome.feed)
pub fn light_home_feed(g: &mut crate::game::Game, l: LightId) {
    let r = match seam_lamp_row(g, l) {
        Some(r) => r,
        None => return,
    };
    let s = seam_lamp_steady(g, l);
    let (pos, range, radius, kind) = (seam_lamp_pos(g, l), seam_lamp_range(g, l), seam_lamp_radius(g, l), seam_lamp_kind(g, l));
    let (dir, outer, inner) = (seam_lamp_dir(g, l), seam_lamp_outer_deg(g, l), seam_lamp_inner_deg(g, l));
    let (color, intensity, enabled) = (seam_lamp_color(g, l), seam_lamp_intensity(g, l), seam_lamp_enabled(g, l));
    let c = lights(g);
    c.x[r] = pos[0];
    c.y[r] = pos[1];
    c.z[r] = pos[2];
    c.range[r] = range;
    c.radius[r] = radius;
    c.kind[r] = kind;
    c.dx[r] = dir[0];
    c.dy[r] = dir[1];
    c.dz[r] = dir[2];
    c.outer[r] = outer;
    c.inner[r] = inner;
    c.r[r] = color[0];
    c.g[r] = color[1];
    c.b[r] = color[2];
    c.intensity[r] = intensity;
    match s {
        Some((s_intensity, s_pos, s_radius)) => {
            c.sx[r] = s_pos[0];
            c.sy[r] = s_pos[1];
            c.sz[r] = s_pos[2];
            c.s_radius[r] = s_radius;
            c.s_intensity[r] = s_intensity;
        }
        None => {
            c.sx[r] = 0.0;
            c.sy[r] = 0.0;
            c.sz[r] = 0.0;
            c.s_radius[r] = 0.0;
            c.s_intensity[r] = 0.0;
        }
    }
    c.row_bits[r] = (c.row_bits[r] & !(R_ON | R_STEADY)) | if enabled { R_ON } else { 0 } | if s.is_some() { R_STEADY } else { 0 };
}

/// …of a lamp somebody moves each tick (a fire's): only while it burns; out, its record is the picture's alone. (lightshome.ts LightHome.feedBurning)
pub fn light_home_feed_burning(g: &mut crate::game::Game, l: LightId) {
    if seam_lamp_enabled(g, l) {
        light_home_feed(g, l);
        return;
    }
    if let Some(r) = seam_lamp_row(g, l) {
        lights(g).row_bits[r] &= !R_ON;
    }
}

/// Every event from outside: a kind, a fixture where it has one, its numbers. → the kind's own answer. (lightshome.ts LightHome.in)
pub fn light_home_in(g: &mut crate::game::Game, kind: f64, t: Option<FixtureId>, a: Option<f64>, b: Option<f64>, c_2: Option<f64>, d: Option<f64>) -> f64 {
    let i = match t {
        Some(t) => fix_i(g, t),
        None => 0,
    };
    rs::input(g.sim, kind as u32, i, a.unwrap_or(0.0), b.unwrap_or(0.0), c_2.unwrap_or(0.0), d.unwrap_or(0.0)) as f64
}

/// A wall switch's circuit flipped. (lightshome.ts LightHome.flip)
pub fn light_home_flip(g: &mut crate::game::Game, targets: &[FixtureId], on: bool, time: f64, strike: bool) {
    // rust/sim takes the circuit from its own scratch table, as the TypeScript writes it (one thread: see notes)
    let ids = unsafe { core::slice::from_raw_parts_mut(rs::ids_addr() as *mut u32, targets.len()) };
    for (i, &t) in targets.iter().enumerate() {
        ids[i] = fix_i(g, t) as u32;
    }
    rs::flip(g.sim, on, time, strike, targets.len());
}

/// One tick. → LIGHT_EV bits. (lightshome.ts LightHome.step)
pub fn light_home_step(g: &mut crate::game::Game, time: f64, dt: f64, near: bool, fumble: f64, emissive_scale: f64) -> f64 {
    rs::step(g.sim, time, dt, near, fumble, emissive_scale) as f64
}

/// What the module worked out, to what the picture reads: a lamp's burning, and a beacon's turn. (lightshome.ts LightHome.paint)
pub fn light_home_paint(g: &mut crate::game::Game) {
    for r in 0..g.light_home.lamps.len() {
        let c = lights(g);
        let b = c.row_bits[r];
        let role = b & R_ROLE;
        if role == R_CONE {
            continue;
        }
        let (i, dir) = (c.intensity[r], [c.dx[r], c.dy[r], c.dz[r]]);
        let l = g.light_home.lamps[r];
        seam_lamp_set_enabled(g, l, b & R_ON != 0);
        seam_lamp_set_intensity(g, l, i);
        if role == R_AREA {
            seam_lamp_set_peak_intensity(g, l, i);
        } else if role == R_BEACON {
            seam_lamp_set_dir(g, l, dir);
        }
    }
}

/// A camera before its picture, and its cone to what the picture reads. → CAM bits. (lightshome.ts LightHome.cameraPan)
pub fn light_home_camera_pan(g: &mut crate::game::Game, slot: f64, cone: Option<LightId>, dt: f64, live: bool, cones: bool) -> f64 {
    let out = rs::camera_pan(g.sim, slot as usize, dt, live, cones);
    if let Some(cone) = cone {
        if let Some(r) = seam_lamp_row(g, cone) {
            let c = lights(g);
            let (on, pos, dir) = (c.row_bits[r] & R_ON != 0, [c.x[r], c.y[r], c.z[r]], [c.dx[r], c.dy[r], c.dz[r]]);
            seam_lamp_set_enabled(g, cone, on);
            seam_lamp_set_pos(g, cone, pos);
            seam_lamp_set_dir(g, cone, dir);
        }
    }
    out as f64
}

/// …and after it. → CAM bits. (lightshome.ts LightHome.cameraSee)
pub fn light_home_camera_see(g: &mut crate::game::Game, slot: f64, time: f64, dt: f64, visible: bool, gain: f64, p: V3, suspicion: f64, alarm: f64) -> f64 {
    rs::camera_see(g.sim, slot as usize, time, dt, visible, gain, p, suspicion, alarm) as f64
}

/// A tripwire before what is in its beam. → LASER bits. (lightshome.ts LightHome.laserOpen)
pub fn light_home_laser_open(g: &mut crate::game::Game, slot: f64) -> f64 {
    rs::laser_open(g.sim, slot as usize) as f64
}

/// …and after it. → LASER bits. (lightshome.ts LightHome.laserClose)
pub fn light_home_laser_close(g: &mut crate::game::Game, slot: f64, time: f64, live: bool, cut: f64, sense: f64, held: bool) -> f64 {
    rs::laser_close(g.sim, slot as usize, time, live, cut, sense, held) as f64
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate std;

    fn def(on: bool, kind: FixtureDefKind) -> FixtureDef {
        FixtureDef { on, kind, disabled_until: -1.0, factor: 0.5, stagger: 0.25, base_intensity: 7.0, mains: true, ..FixtureDef::zeroed() }
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
            assert_eq!((mains_active(g), mains_until(g), mains_since(g), mains_restored_at(g)), (false, -1.0, -1.0, -100.0));

            let a = light_home_add(g, &def(true, FixtureDefKind::Analytic));
            let b = light_home_add(g, &def(false, FixtureDefKind::Emissive));
            assert_eq!((g.tab.sim_lightshome_fixture[a].i, g.tab.sim_lightshome_fixture[b].i), (0.0, 1.0));
            assert_eq!(g.tab.sim_lightshome_fixture[b].kind, FixtureKind::Emissive);
            assert_eq!((fixture_on(g, a), fixture_on(g, b)), (true, false));
            assert_eq!(lights(g).bits[1], T_MAINS | T_EMISSIVE);
            assert_eq!((fixture_factor(g, a), fixture_stagger(g, a), fixture_base_intensity(g, a), fixture_disabled_until(g, a)), (0.5, 0.25, 7.0, -1.0));

            // a kind and a fixture reach the right row; the default numbers are zeros
            light_home_in(g, light_in().on, Some(b), Some(1.0), None, None, None);
            assert!(fixture_on(g, b));
            light_home_in(g, light_in().authored, Some(b), Some(1.0), None, None, None);
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
            assert_eq!(g.tab.devices_camera_rt[c].smoke_trans, 1.0);
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
}
