//! src/game/devices.ts lines 1 to 438
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

// ---------------------------------------------------------------- what the skeleton cannot give yet (each one line in notes.md)
/// devices.ts:92 `g.engine.lights.add({...})`: the skeleton has no way to make a lamp's record (no RtLight, LightId has no table)
#[allow(clippy::too_many_arguments)]
fn seam_add_cone(g: &mut Game, name: &str, group: &str, kind: f64, pos: V3, dir: V3, color: V3, intensity: f64, range: f64, inner_deg: f64, outer_deg: f64, radius: f64, volumetric: f64, owner: f64, enabled: bool, fixture_box: f64, flicker: f64, ttl: f64, age: f64, peak_intensity: f64) -> Option<LightId> { todo!("needs: a maker of an RtLight") }
/// devices.ts:369 `out[k].mkey = …`: a made box has no setter (BoxId's table is "none", so the index's g.none[id].mkey names nothing on Game)
fn seam_set_mkey(g: &mut Game, b: BoxId, mkey: f64) { todo!("needs: a setter of a box's mkey") }

// ---------------------------------------------------------------- small readers of this file
/// `men(g)`: the skeleton's stub also takes the `Pick<Game, 'seats' | 'player'>`, here made from g
fn men_of(g: &mut Game) -> Vec<PlayerId> {
    let picked = crate::seats::MenG { player: g.player, seats: g.seats.clone() };
    crate::seats::men(g, &picked)
}
/// `c.bones.<b> ?? v3.add(c.pos, off)`
fn bone_or(g: &Game, c: CharId, b: crate::character::Bone, off: V3) -> V3 {
    match g.ch[c].bones.get(b) { Some(p) => *p, None => v3::add(g.ch[c].pos, off) }
}
fn callsign_of(g: &Game, gd: Option<GuardId>) -> Option<String> { gd.map(|x| g.gd[x].callsign.clone()) }
/// `CameraMachine.yaw`, read off rust/sim's column (lights.md)
fn cam_yaw(g: &Game, slot: f64) -> f64 { sim::state::state(g.sim).lights.cam_yaw[slot as usize] }
fn cam_seen(g: &Game, slot: f64) -> f64 { sim::state::state(g.sim).lights.cam_seen[slot as usize] }
fn cam_idle(g: &Game, slot: f64) -> bool { sim::state::state(g.sim).lights.cam_state[slot as usize] == sim::lights::S_IDLE }
/// `CameraMachine.lastSeen`
fn cam_last_seen(g: &Game, slot: f64) -> Option<V3> {
    let l = &sim::state::state(g.sim).lights;
    let s = slot as usize;
    if l.cam_bits[s] & sim::lights::C_LAST != 0 { Some([l.cam_lx[s], l.cam_ly[s], l.cam_lz[s]]) } else { None }
}
fn data(rows: &[(&str, Option<ChronDeviceDataValue>)]) -> JsMap<String, Option<ChronDeviceDataValue>> {
    let mut o = JsMap::new();
    for (k, v) in rows { o.set(String::from(*k), v.clone()); }
    o
}
fn text(s: &str) -> Option<ChronDeviceDataValue> { Some(ChronDeviceDataValue::Text(String::from(s))) }
fn opt_text(s: &Option<String>) -> Option<ChronDeviceDataValue> { s.as_ref().map(|x| ChronDeviceDataValue::Text(x.clone())) }
fn bits_or(a: f64, b: f64) -> f64 { (js::to_i32(a) | js::to_i32(b)) as f64 }

// ---------------------------------------------------------------- runtime records
/// A camera of the level: its machine is rust/sim's row `slot`.   (devices.ts CameraRt)
pub fn camera_rt_new(g: &mut crate::game::Game, slot: f64, st: crate::Opaque, def: &crate::devices::CameraDef2, index: f64, target: FixtureId, cone: Option<LightId>, smoke_key: &str) -> CameraRtId {
    g.tab.devices_camera_rt.push(CameraRt { smoke_trans: 1.0, def: def.clone(), index, target, cone, smoke_key: String::from(smoke_key), slot, st })
}
/// A tripwire of the level: its machine is rust/sim's row `slot`.   (devices.ts LaserRt)
pub fn laser_rt_new(g: &mut crate::game::Game, slot: f64, st: crate::Opaque, def: &crate::devices::LaserDef2, index: f64, target: FixtureId, dir: V3, len: f64) -> LaserRtId {
    g.tab.devices_laser_rt.push(LaserRt { def: def.clone(), index, target, dir, len, slot, st })
}
/// Append a report to the capped log with its sequence number.   (devices.ts Devices.note)
pub fn devices_note(g: &mut crate::game::Game, r: &crate::devices::DevicesNoteR) {
    g.devices.seq += 1.0;
    let what = match r.what {
        DevicesNoteRWhat::Dead => DeviceLogRowWhat::Dead,
        DevicesNoteRWhat::Code => DeviceLogRowWhat::Code,
        DevicesNoteRWhat::Alarm => DeviceLogRowWhat::Alarm,
        DevicesNoteRWhat::Trip => DeviceLogRowWhat::Trip,
        DevicesNoteRWhat::Suspicion => DeviceLogRowWhat::Suspicion,
        DevicesNoteRWhat::Deny => DeviceLogRowWhat::Deny,
        DevicesNoteRWhat::Overheard => DeviceLogRowWhat::Overheard,
    };
    let row = DeviceLogRow { seq: g.devices.seq, t: g.time, device: r.device.clone(), what, where_: r.where_.clone(), sent: r.sent.clone() };
    g.devices.log.push(row);
    if g.devices.log.len() > 48 { let over = g.devices.log.len() - 48; g.devices.log.drain(0..over); }
}
/// The level's cameras, tripwires and keypads from its list; cameras and emitters join the light targets.   (devices.ts Devices.constructor)
pub fn devices_new(g: &mut crate::game::Game) -> crate::devices::Devices {
    let mut this = Devices::blank();
    let defs = g.level.devices.clone().unwrap_or_default();
    let (mut ci, mut li) = (0.0_f64, 0.0_f64);
    for dv in &defs {
        match dv {
            crate::scene_level::DeviceDef::Camera(d) => {
                let dd = crate::scene_level_data::device_defaults();
                let fix = crate::sim_lightshome::FixtureDef {
                    name: d.name.clone(), group: d.group.clone().unwrap_or(dd.group.clone()), kind: crate::sim_lightshome::FixtureDefKind::Device, pos: v3::copy(d.pos), light: None, boxes: Vec::new(), base_emissive: [0.0, 0.0, 0.0], base_intensity: 0.0,
                    on: true, broken: false, disabled_until: -1.0, fluorescent_flicker: false, factor: 1.0, mains: d.mains != Some(false), interactive: true, stagger: 1.4 + 0.3 * ci, area_lights: None, lens: None,
                };
                let target = crate::sim_lightshome::light_home_add(g, &fix);
                g.targets.push(target);
                let half = d.fov_deg.unwrap_or(dd.fov_deg) / 2.0;
                let cone = seam_add_cone(g, &format!("{}_cone", d.name), "devices", 1.0, v3::copy(d.pos), [0.0, -1.0, 0.0], v3::copy(cone().color), cone().intensity, d.range.unwrap_or(dd.range) * cone().range_k,
                    m::max(1.0, m::min(half, cone().outer_max_deg) - 6.0), m::min(half, cone().outer_max_deg), cone().radius, cone().volumetric, 0.0, false, -1.0, 0.0, -1.0, 0.0, cone().intensity);
                let slot = crate::sim_lightshome::light_home_camera(g, target, d.yaw, d.sweep, d.sweep_period.unwrap_or(dd.sweep_period), d.pitch.unwrap_or(dd.pitch), d.pos);
                ci += 1.0;
                let rt = camera_rt_new(g, slot, crate::Opaque, d, ci, target, cone, &format!("smokeCam{}", ci));
                this.cameras.push(rt);
            }
            crate::scene_level::DeviceDef::Laser(d) => {
                let dd = crate::scene_level_data::device_defaults();
                let fix = crate::sim_lightshome::FixtureDef {
                    name: d.name.clone(), group: d.group.clone().unwrap_or(dd.group.clone()), kind: crate::sim_lightshome::FixtureDefKind::Device, pos: v3::copy(d.from), light: None, boxes: Vec::new(), base_emissive: [0.0, 0.0, 0.0], base_intensity: 0.0,
                    on: true, broken: false, disabled_until: -1.0, fluorescent_flicker: false, factor: 1.0, mains: d.mains != Some(false), interactive: true, stagger: 1.1 + 0.3 * li, area_lights: None, lens: None,
                };
                let target = crate::sim_lightshome::light_home_add(g, &fix);
                g.targets.push(target);
                let seg = v3::sub(d.to, d.from);
                let len = v3::len(seg);
                let slot = crate::sim_lightshome::light_home_laser(g, target, len);
                li += 1.0;
                let rt = laser_rt_new(g, slot, crate::Opaque, d, li, target, v3::scale(seg, 1.0 / m::max(len, 1e-6)), len);
                this.lasers.push(rt);
            }
            crate::scene_level::DeviceDef::Keypad(d) => {
                let door = match crate::doors::doors_by_name(g, &d.door) {
                    Some(x) => x,
                    None => panic!("keypad '{}': no door '{}'", d.name, d.door),
                };
                let k = KeypadRt { def: d.clone(), door, n: [m::sin(d.yaw), 0.0, m::cos(d.yaw)] };
                this.keypads.push(k.clone());
                this.pad_by_door.set(door, k);
            }
        }
    }
    this
}
/// The keypad on this door, if it is a keypad door.   (devices.ts Devices.padOf)
pub fn devices_pad_of(g: &mut crate::game::Game, d: DoorId) -> Option<crate::devices::KeypadRt> {
    g.devices.pad_by_door.get(d).cloned()
}
/// `speaker` has just said the code of `lock_id` aloud.   (devices.ts Devices.overhearSaid)
pub fn devices_overhear_said(g: &mut crate::game::Game, speaker: GuardId, lock_id: &str) -> bool {
    overhear(g, speaker, lock_id)
}
/// The keypad whose door stands nearest a floor point.   (devices.ts Devices.padNear)
pub fn devices_pad_near(g: &mut crate::game::Game, p: V3) -> Option<crate::devices::KeypadRt> {
    let mut best: Option<usize> = None;
    let mut bd = f64::INFINITY;
    for (i, k) in g.devices.keypads.iter().enumerate() {
        let dd = v3::dist_xz(g.dr[k.door].frame_centre, p);
        if dd < bd { bd = dd; best = Some(i); }
    }
    best.map(|i| g.devices.keypads[i].clone())
}
/// A fresh encounter: integrators, states, trip clocks, the log (`seq` runs on).   (devices.ts Devices.reset)
pub fn devices_reset(g: &mut crate::game::Game) {
    for c in g.devices.cameras.clone() {
        let yaw = g.tab.devices_camera_rt[c].def.yaw;
        crate::sim_lightshome::camera_machine_put(g, c, yaw, 0.0, 0.0, crate::sim_lightshome::CameraState::Idle, -100.0, None, false);
    }
    for l in g.devices.lasers.clone() { crate::sim_lightshome::laser_machine_put(g, l, -100.0, 0.0, false, false); }
    g.devices.log.clear();
    g.devices.radio_count = 0.0;
}

/// Powered, unbroken, not knocked out: the device works this frame.   (devices.ts alive)
pub fn alive(g: &mut crate::game::Game, t: FixtureId) -> bool {
    crate::sim_lightshome::fixture_factor(g, t) > 0.05 && !crate::sim_lightshome::fixture_broken(g, t)
}
/// Live play: devices sense.   (devices.ts sensing)
pub fn sensing(g: &mut crate::game::Game) -> bool {
    g.ai_enabled && !g.quiet_utility && !g.touring && g.tune.devices && !g.night_breach.active && !g.rescue.active
}
/// Lit-seconds per second a camera's integrator fills at.   (devices.ts lensGain)
pub fn lens_gain(g: &mut crate::game::Game, dist: f64, range: f64, smoke_trans: f64) -> f64 {
    let pl = g.player;
    let light = g.pl[pl].visibility;
    let dist_f = v3::clamp(1.0 - (dist - 3.0) / m::max(1.0, range - 3.0), 0.2, 1.0);
    let crouch = if g.pl[pl].crouch { 0.75 } else { 1.0 };
    m::pow(light, 1.5) * dist_f * crouch * v3::clamp((smoke_trans - 0.3) / 0.5, 0.0, 1.0) * crate::kitload::load_seen(g, crate::kitload::LoadSeenWho::Player)
}
/// The lens's look direction at yaw / pitch.   (devices.ts lensDir)
pub fn lens_dir(yaw: f64, pitch: f64) -> V3 {
    let cp = m::cos(pitch);
    [m::sin(yaw) * cp, m::sin(pitch), m::cos(yaw) * cp]
}
/// Where a panning camera points at phase u: eased there and back between the sweep's two yaws.   (devices.ts sweepYaw)
pub fn sweep_yaw(d: &crate::devices::CameraDef2, phase: f64) -> f64 {
    let sw = match d.sweep { Some(x) => x, None => return d.yaw };
    let s = 0.5 - 0.5 * m::cos(phase * m::PI * 2.0);
    v3::lerp(sw[0], sw[1], s)
}

/// Pan the cameras, run their pictures, run the beams, key the code being entered.   (devices.ts updateDevices)
pub fn update_devices(g: &mut crate::game::Game, dt: f64) {
    let live = sensing(g);
    for c in g.devices.cameras.clone() { update_camera(g, c, dt, live); }
    for l in g.devices.lasers.clone() { let on = live && g.tune.lasers; update_laser(g, l, dt, on); }
    if g.seats.len() == 1 { update_coding(g, dt); } else {
        for pl in men_of(g) { crate::seats::with_seat(g, pl, &mut |v: &mut Game| update_coding(v, dt)); }
    }
}
/// One camera's frame: the pan, the feed dropping, the picture, what it does about it.   (devices.ts updateCamera)
pub fn update_camera(g: &mut crate::game::Game, c_2: CameraRtId, dt: f64, live: bool) {
    let c = c_2;
    let d = g.tab.devices_camera_rt[c].def.clone();
    let (slot, index) = (g.tab.devices_camera_rt[c].slot, g.tab.devices_camera_rt[c].index);
    let pan = sim::lights::camera_pan(g.sim, slot as usize, dt, live, g.tune.device_cones);
    if pan & sim::lights::CAM_DOWN != 0 && live { device_down(g, &d.name, &format!("camera {}", index), d.pos); }
    if pan & sim::lights::CAM_OK == 0 || !live { return; }
    let dd = crate::scene_level_data::device_defaults();
    let pl = if g.seats.len() == 1 { g.player } else {
        match crate::seats::nearest_man(g, &d.pos) { Some(p) => p, None => men_of(g)[0] }
    };
    let pc = g.pl[pl].ch;
    let range = d.range.unwrap_or(dd.range);
    let half = d.fov_deg.unwrap_or(dd.fov_deg) / 2.0;
    let look = lens_dir(cam_yaw(g, slot), d.pitch.unwrap_or(dd.pitch));
    let e = crate::guards::Eye { pos: d.pos, look, range, half_deg: half, near_m: 0.0, near_deg: half, v_half_deg: Some(half) };
    let key = g.tab.devices_camera_rt[c].smoke_key.clone();
    if let Some(tr) = crate::sensors::sensors_read(g, &key) { g.tab.devices_camera_rt[c].smoke_trans = tr; }
    let chest = bone_or(g, pc, crate::character::Bone::Chest, [0.0, 1.1, 0.0]);
    let smoke = g.tab.devices_camera_rt[c].smoke_trans;
    let air = smoke * crate::weather::air_trans(g, d.pos, chest);
    let vis = crate::guards::eye_contact(g, e, pc, true, false, air, None);
    if vis.in_cone { let chest = bone_or(g, pc, crate::character::Bone::Chest, [0.0, 1.1, 0.0]); crate::sensors::sensors_ask_segment(g, &key, d.pos, chest); }
    let gain = if vis.visible { lens_gain(g, vis.dist, range, air) } else { 0.0 };
    let r = sim::lights::camera_see(g.sim, slot as usize, g.time, dt, vis.visible, gain, g.ch[pc].pos, g.tune.camera_suspicion_secs, g.tune.camera_alarm_secs);
    if r & sim::lights::CAM_SUSPECT != 0 { camera_suspect(g, c); }
    if r & sim::lights::CAM_ALARM != 0 { camera_alarm(g, c); }
}
/// The picture has movement: a man sent, the radio, the debrief.   (devices.ts cameraSuspect)
pub fn camera_suspect(g: &mut crate::game::Game, c_2: CameraRtId) {
    let c = c_2;
    let (slot, index) = (g.tab.devices_camera_rt[c].slot, g.tab.devices_camera_rt[c].index);
    let name = g.tab.devices_camera_rt[c].def.name.clone();
    let seen = cam_last_seen(g, slot).unwrap();
    let where_ = crate::chronicle::room_of(g, seen);
    let sent = send_to_look(g, seen, &format!("camera {}", index));
    let sign = callsign_of(g, sent);
    let tail = match &sign { Some(s) => format!(" — {}, go and look", s), None => String::new() };
    control(g, &format!("camera {} has movement in the {}{}", index, where_, tail));
    let row = data(&[("camera", text(&name)), ("n", Some(ChronDeviceDataValue::Num(index))), ("sent", opt_text(&sign))]);
    chron_device(g, ChronDeviceKind::CameraSpotted, &row, Some(seen));
    devices_note(g, &DevicesNoteR { device: name, what: DevicesNoteRWhat::Suspicion, where_, sent: sign });
}
/// The picture has him: the floor's fix, everybody sharp, the floor up a step.   (devices.ts cameraAlarm)
pub fn camera_alarm(g: &mut crate::game::Game, c_2: CameraRtId) {
    let c = c_2;
    let (slot, index) = (g.tab.devices_camera_rt[c].slot, g.tab.devices_camera_rt[c].index);
    let name = g.tab.devices_camera_rt[c].def.name.clone();
    let p = cam_last_seen(g, slot).unwrap();
    let where_ = crate::chronicle::room_of(g, p);
    control(g, &format!("camera {}: that's him — the {}. all posts, the {}", index, where_, where_));
    g.alarm.pos[0] = p[0];
    g.alarm.pos[1] = 0.0;
    g.alarm.pos[2] = p[2];
    g.alarm.placed = true;
    for i in 0..g.guards.len() {
        let id = g.guards[i];
        let gd = &mut g.gd[id];
        if gd.state != crate::game::GuardState::Dead && gd.held.is_none() && gd.unit.is_none() {
            gd.last_known = Some(v3::copy(p));
            gd.awareness = m::max(gd.awareness, 0.5);
            if gd.state == crate::game::GuardState::Patrol {
                gd.state = crate::game::GuardState::Suspicious;
                gd.react_t = 0.8;
                gd.search_t = 0.0;
                gd.path = Vec::new();
                gd.path_goal = None;
            }
        }
    }
    let to = if crate::guards::escalation_of(g) >= 1.0 { 2.0 } else { 1.0 };
    crate::guards::raise_escalation(g, to, None, Some(crate::guards::RaiseEscalationCause::Device), None);
    let row = data(&[("camera", text(&name)), ("n", Some(ChronDeviceDataValue::Num(index)))]);
    chron_device(g, ChronDeviceKind::CameraAlarm, &row, Some(p));
    devices_note(g, &DevicesNoteR { device: name, what: DevicesNoteRWhat::Alarm, where_, sent: None });
}

/// One beam's frame: where it ends, what breaks it, the trip.   (devices.ts updateLaser)
pub fn update_laser(g: &mut crate::game::Game, l: LaserRtId, dt: f64, live: bool) {
    let d = g.tab.devices_laser_rt[l].def.clone();
    let (slot, dir, len) = (g.tab.devices_laser_rt[l].slot, g.tab.devices_laser_rt[l].dir, g.tab.devices_laser_rt[l].len);
    let open = sim::lights::laser_open(g.sim, slot as usize);
    if open & sim::lights::LASER_DOWN != 0 && sensing(g) {
        let room = crate::chronicle::room_of(g, d.from);
        device_down(g, &d.name, &format!("the {} beam", room), d.from);
    }
    if open & sim::lights::LASER_OK == 0 { return; }
    let mut reach = len;
    let no_shadow = crate::scene_boxes::box_flag().no_shadow;
    if let Some(wh) = crate::scene_boxes::box_world_raycast(g, v3::mad(d.from, dir, 0.03), dir, Some(reach), Some(no_shadow), None, None) { reach = m::min(reach, wh.t + 0.03); }
    if let Some(ph) = crate::props::prop_system_raycast(g, d.from, dir, reach) { reach = m::min(reach, ph.t); }
    // under the fog the sensor asks the floor, not the picture
    let mut sense = reach;
    if live && crate::fogofwar::fog_of_war_active(g) {
        sense = len;
        if let Some(sh) = crate::scene_collision::raycast(g, v3::mad(d.from, dir, 0.03), dir, sense) { sense = m::min(sense, sh.t + 0.03); }
        if let Some(sp) = crate::props::prop_system_raycast(g, d.from, dir, sense) { sense = m::min(sense, sp.t); }
    }
    let end = v3::mad(d.from, dir, sense);
    let mut who: Option<&'static str> = None;
    let mut at: Option<V3> = None;
    let mut cut = reach;
    for pl in men_of(g) {
        if who.is_some() { break; }
        if !g.pl[pl].down && !g.player_invisible {
            let c = g.pl[pl].ch;
            at = body_breaks(g, c, d.from, end);
            if at.is_some() { who = Some("player"); }
        }
        if who.is_none() {
            if let Some(dg) = g.pl[pl].dragging {
                let c = g.gd[dg].ch;
                at = body_breaks(g, c, d.from, end);
                if at.is_some() { who = Some("body"); }
            }
        }
    }
    if who.is_none() {
        let back = m::max(dt, 1.0 / 60.0);
        for it in g.items.items.clone() {
            let kind = crate::throwables::item_kind(g, it);
            if crate::throwables::item_settled(g, it) || (kind != crate::throwables::ItemKind::Smoke && kind != crate::throwables::ItemKind::Flash && kind != crate::throwables::ItemKind::Frag) { continue; }
            let (ip, iv) = (crate::throwables::item_pos(g, it), crate::throwables::item_vel(g, it));
            let prev = [ip[0] - iv[0] * back, ip[1] - iv[1] * back, ip[2] - iv[2] * back];
            let q = seg_seg_closest(prev, ip, d.from, end);
            if q.dist < crate::throwables::item_radius(g, it) + BEAM_MARGIN { who = Some(kind.as_str()); at = Some(q.on_b); break; }
        }
    }
    if let Some(a) = at { cut = m::min(cut, v3::dist(d.from, a)); }
    for o in g.guards.clone() {
        if g.gd[o].state == crate::game::GuardState::Dead { continue; }
        let c = g.gd[o].ch;
        let t = crate::character::character_ray_hit(g, c, d.from, dir, cut);
        if t >= 0.0 { cut = m::min(cut, t); }
    }
    for o in crate::civilians::civilian_bodies(g) {
        let c = g.gd[o].ch;
        let t = crate::character::character_ray_hit(g, c, d.from, dir, cut);
        if t >= 0.0 { cut = m::min(cut, t); }
    }
    if sim::lights::laser_close(g.sim, slot as usize, g.time, live, cut, sense, who.is_some()) & sim::lights::LASER_TRIP != 0 {
        laser_trip(g, l, who.unwrap(), at.unwrap());
    }
}
/// Does any capsule of `c` cross the beam a→b: the point on the beam where it does.   (devices.ts bodyBreaks)
pub fn body_breaks(g: &mut crate::game::Game, c_2: CharId, a: V3, b: V3) -> Option<V3> {
    let h = match g.ch[c_2].bones.get(crate::character::Bone::Hips) { Some(h) => *h, None => return None };
    let along = v3::clamp(((h[0] - a[0]) * (b[0] - a[0]) + (h[2] - a[2]) * (b[2] - a[2])) / m::max(1e-6, m::pow(v3::dist_xz(a, b), 2.0)), 0.0, 1.0);
    if v3::dist_xz(h, v3::lerp3(a, b, along)) > 1.6 { return None; }
    let caps = crate::character::character_capsules(g, c_2);
    for k in &caps.caps {
        let q = seg_seg_closest([k[0], k[1], k[2]], [k[4], k[5], k[6]], a, b);
        if q.dist < k[3] + BEAM_MARGIN { return Some(q.on_b); }
    }
    None
}
/// A beam broken: the chirp, the radio, a man sent, by the beam's level.   (devices.ts laserTrip)
pub fn laser_trip(g: &mut crate::game::Game, l: LaserRtId, who: &str, at: V3) {
    let d = g.tab.devices_laser_rt[l].def.clone();
    let where_ = crate::chronicle::room_of(g, at);
    let level: String = match d.alarm { Some(x) => String::from(x.as_str()), None => crate::scene_level_data::device_defaults().alarm };
    crate::audio_iface::game_audio_like_play(g, "alarmChirp", Some(d.from), Some(0.7), None);
    if who == "player" { let pl = g.player; g.pl[pl].noise = m::max(g.pl[pl].noise, 0.3); }
    let sent: Option<GuardId>;
    if level == "alarm" {
        control(g, &format!("beam broken in the {} — that's an intruder. weapons out", where_));
        g.alarm.pos[0] = at[0];
        g.alarm.pos[1] = 0.0;
        g.alarm.pos[2] = at[2];
        g.alarm.placed = true;
        sent = send_to_look(g, at, "the beam");
        let to = if crate::guards::escalation_of(g) >= 1.0 { 2.0 } else { 1.0 };
        crate::guards::raise_escalation(g, to, None, Some(crate::guards::RaiseEscalationCause::Device), None);
    } else {
        sent = send_to_look(g, at, "the beam");
        let tail = match callsign_of(g, sent) { Some(s) => format!(" — {}, check it", s), None => String::from(" — somebody check it") };
        control(g, &format!("beam broken in the {}{}", where_, tail));
    }
    let sign = callsign_of(g, sent);
    let row = data(&[("laser", text(&d.name)), ("by", text(who)), ("level", text(&level)), ("sent", opt_text(&sign))]);
    chron_device(g, ChronDeviceKind::LaserTripped, &row, Some(at));
    devices_note(g, &DevicesNoteR { device: d.name, what: DevicesNoteRWhat::Trip, where_, sent: sign });
}
/// A camera or an emitter shot dead: control loses the feed and sends the nearest calm man to look at it.   (devices.ts deviceDown)
pub fn device_down(g: &mut crate::game::Game, name: &str, spoken: &str, pos: V3) {
    let seed = g.fx_seed;
    g.fx_seed += 1.0;
    crate::sparks::sparks_emit_burst(g, pos, crate::sparks::SparksEmitBurstO { count: 14.0, speed: 2.2, spread: 1.0, lift: 0.5, life: 0.5 }, js::to_u32(seed * 7919.0) as f64);
    let floor = crate::scene_collision::nav_grid_nearest_free_point(g, pos[0], pos[2]).unwrap_or([pos[0], 0.0, pos[2]]);
    let sent = send_to_look(g, floor, spoken);
    let sign = callsign_of(g, sent);
    let tail = match &sign { Some(s) => format!(" — {}, get eyes on it", s), None => String::from(" — somebody get eyes on it") };
    control(g, &format!("{} just went dark{}", spoken, tail));
    let row = data(&[("device", text(name)), ("sent", opt_text(&sign))]);
    chron_device(g, ChronDeviceKind::DeviceDown, &row, Some(floor));
    let where_ = crate::chronicle::room_of(g, floor);
    devices_note(g, &DevicesNoteR { device: String::from(name), what: DevicesNoteRWhat::Dead, where_, sent: sign });
}
/// The nearest calm, free staff man is sent to `pos` on control's word: a task that survives the suspicion decay.   (devices.ts sendToLook)
pub fn send_to_look(g: &mut crate::game::Game, pos: V3, what: &str) -> Option<GuardId> {
    let goal = match crate::scene_collision::nav_grid_nearest_free_point(g, pos[0], pos[2]) { Some(p) => p, None => v3::copy(pos) };
    let mut best: Option<GuardId> = None;
    let mut bd = f64::INFINITY;
    for &id in &g.guards {
        let gd = &g.gd[id];
        if gd.state != crate::game::GuardState::Patrol || gd.hold || gd.task.is_some() || gd.held.is_some() || gd.script.is_some() || gd.unit.is_some() || gd.body_duty.is_some() { continue; }
        let dd = v3::dist(g.ch[gd.ch].pos, goal);
        if dd < bd { bd = dd; best = Some(id); }
    }
    let id = best?;
    {
        let gd = &mut g.gd[id];
        gd.task = Some(crate::game::GuardTask::Arm0(crate::game::GuardTaskArm0 { kind: crate::game::GuardTaskArm0Kind::Device, pos: v3::copy(goal) }));
        gd.last_known = Some(v3::copy(goal));
        gd.awareness = m::max(gd.awareness, errand().awareness);
        gd.state = crate::game::GuardState::Suspicious;
        gd.react_t = errand().react_t;
        gd.search_t = 0.0;
        gd.path = Vec::new();
        gd.path_goal = None;
    }
    crate::game::game_say(g, id, "copy, control — on my way", Some(true), None, None);
    let c = g.gd[id].ch;
    let at = match g.ch[c].bones.get(crate::character::Bone::Head) { Some(p) => *p, None => g.ch[c].pos };
    crate::audio_iface::game_audio_like_play(g, "radio", Some(at), Some(0.5), None);
    Some(id)
}
/// Control's voice on the net.   (devices.ts control)
pub fn control(g: &mut crate::game::Game, text: &str) {
    crate::game::game_msg(g, &format!("radio: CONTROL — {}", text));
    crate::audio_iface::game_audio_like_play(g, "radio", None, Some(0.55), None);
    g.devices.radio_count += 1.0;
    let mut row: JsMap<String, Option<crate::chronicle::ChronicleDataValue>> = JsMap::new();
    row.set(String::from("who"), Some(crate::chronicle::ChronicleDataValue::Text(String::from("CONTROL"))));
    row.set(String::from("text"), Some(crate::chronicle::ChronicleDataValue::Text(String::from(text))));
    crate::chronicle::chronicle(g, crate::chronicle::ChronKind::Radio, Some(&row), None);
}
/// The debrief's rows, through the mission's own live gate.   (devices.ts chronDevice)
pub fn chron_device(g: &mut crate::game::Game, kind: crate::devices::ChronDeviceKind, data: &JsMap<String, Option<crate::devices::ChronDeviceDataValue>>, pos: Option<V3>) -> Option<crate::chronicle::ChronEvent> {
    use crate::chronicle::{ChronKind, ChronicleDataValue as V, ChronicleNDataValue as N};
    let k = match kind {
        ChronDeviceKind::CameraSpotted => ChronKind::CameraSpotted,
        ChronDeviceKind::CameraAlarm => ChronKind::CameraAlarm,
        ChronDeviceKind::LaserTripped => ChronKind::LaserTripped,
        ChronDeviceKind::DeviceDown => ChronKind::DeviceDown,
        ChronDeviceKind::CodeLearned => ChronKind::CodeLearned,
        ChronDeviceKind::CodeEntered => ChronKind::CodeEntered,
    };
    if kind == ChronDeviceKind::CameraSpotted || kind == ChronDeviceKind::LaserTripped {
        let mut row: JsMap<String, Option<N>> = JsMap::new();
        for (key, v) in data.iter() {
            row.set(key, v.as_ref().map(|x| match x { ChronDeviceDataValue::Text(s) => N::Text(s.clone()), ChronDeviceDataValue::Num(n) => N::Num(*n), ChronDeviceDataValue::Bool(b) => N::Bool(*b) }));
        }
        crate::chronicle::chronicle_n(g, k, crate::chronicle::ChronCountKey::Devices, Some(&row), pos)
    } else {
        let mut row: JsMap<String, Option<V>> = JsMap::new();
        for (key, v) in data.iter() {
            row.set(key, v.as_ref().map(|x| match x { ChronDeviceDataValue::Text(s) => V::Text(s.clone()), ChronDeviceDataValue::Num(n) => V::Num(*n), ChronDeviceDataValue::Bool(b) => V::Bool(*b) }));
        }
        crate::chronicle::chronicle(g, k, Some(&row), pos)
    }
}

// ---------------------------------------------------------------- keypads
/// The player learns the code of lock `lock_id`; whether it was news.   (devices.ts learnCode)
pub fn learn_code(g: &mut crate::game::Game, lock_id: &str, how: crate::devices::LearnCodeHow, from: Option<GuardId>) -> bool {
    let pl = g.player;
    if g.pl[pl].codes.has(String::from(lock_id)) { return false; }
    let k = match g.devices.keypads.iter().find(|k| k.def.name == lock_id) { Some(k) => k.clone(), None => return false };
    g.pl[pl].codes.add(String::from(lock_id));
    let sign = callsign_of(g, from);
    let who = match how {
        LearnCodeHow::Told => format!("{} gave up", match &sign { Some(s) => s.as_str(), None => "he" }),
        LearnCodeHow::Overheard => String::from("overheard"),
    };
    let door_says = pad_door_name(g, &k);
    crate::game::game_msg(g, &format!("{} the {} code — {}", who, door_says, spell(&k.def.code)));
    let door_name = g.dr[k.door].def.name.clone();
    let row = data(&[("door", text(&door_name)), ("how", text(how.as_str())), ("from", opt_text(&sign))]);
    let at = from.map(|f| g.ch[g.gd[f].ch].pos);
    chron_device(g, ChronDeviceKind::CodeLearned, &row, at);
    let here = g.ch[g.pl[pl].ch].pos;
    let where_ = crate::chronicle::room_of(g, here);
    let what = match how { LearnCodeHow::Told => DevicesNoteRWhat::Code, LearnCodeHow::Overheard => DevicesNoteRWhat::Overheard };
    devices_note(g, &DevicesNoteR { device: String::from(lock_id), what, where_, sent: sign });
    true
}
/// A man has just said the code aloud: the player learns it iff it carried to him; whether he now knows it.   (devices.ts overhear)
pub fn overhear(g: &mut crate::game::Game, speaker: GuardId, lock_id: &str) -> bool {
    let pl = g.player;
    if g.pl[pl].down || g.pl[pl].codes.has(String::from(lock_id)) { return g.pl[pl].codes.has(String::from(lock_id)); }
    let mouth = bone_or(g, g.gd[speaker].ch, crate::character::Bone::Head, [0.0, 1.6, 0.0]);
    let ear_up = if g.pl[pl].crouch { 1.0 } else { 1.6 };
    let ear = bone_or(g, g.pl[pl].ch, crate::character::Bone::Head, [0.0, ear_up, 0.0]);
    let prop = crate::weather::hear_path(g, mouth, ear, None);
    if prop.path_len > g.k.overhear.path_m || prop.carry < g.k.overhear.carry { return false; }
    learn_code(g, lock_id, LearnCodeHow::Overheard, Some(speaker)) || g.pl[pl].codes.has(String::from(lock_id))
}
/// The digits as said ('7-3-9-1').   (devices.ts spell)
pub fn spell(code: &str) -> String {
    let mut out = String::new();
    for (i, ch) in code.chars().enumerate() {
        if i > 0 { out.push('-'); }
        out.push(ch);
    }
    out
}
/// What a pad's door is called on the HUD.   (devices.ts padDoorName)
pub fn pad_door_name(g: &mut crate::game::Game, k: &crate::devices::KeypadRt) -> String {
    let name = &g.dr[k.door].def.name;
    if name == "server_manager" { String::from("side door") } else { format!("{} door", name.replace('_', " ")) }
}
/// Is `d` a keypad door locked against the player where he stands, and does he know its code.   (devices.ts padState)
pub fn pad_state(g: &mut crate::game::Game, d: DoorId, from: V3) -> Option<crate::devices::PadState> {
    let pad = devices_pad_of(g, d)?;
    if !crate::doors::doors_locked_out(g, d, from, None) { return None; }
    let pl = g.player;
    let known = g.pl[pl].codes.has(g.dr[d].lock_id.clone());
    Some(PadState { pad, known })
}
/// The 'code' row: start keying it in. HANDWORK settled on: every key beeps under his finger (pad_hands), no first beep here.   (devices.ts startCoding)
pub fn start_coding(g: &mut crate::game::Game, d: DoorId) -> bool {
    let pl = g.player;
    let here = g.ch[g.pl[pl].ch].pos;
    let st = pad_state(g, d, here);
    let p = &mut g.pl[pl];
    match st {
        Some(s) if s.known && p.coding.is_none() && !p.down => {}
        _ => return false,
    }
    p.coding = Some(d);
    p.code_t = 0.0;
    p.f_held = true;
    p.hand_wait = 0.0;
    true
}
/// The 'nocode' row: a deny tone at the plate, a hair on the meters, a word.   (devices.ts denyPad)
pub fn deny_pad(g: &mut crate::game::Game, d: DoorId) {
    let k = match devices_pad_of(g, d) { Some(k) => k, None => return };
    crate::audio_iface::game_audio_like_play(g, "deny", Some(k.def.pos), Some(0.45), None);
    let pl = g.player;
    let c = g.pl[pl].ch;
    let (time, who, serial) = (g.time, g.ch[c].id, g.serial());
    g.events.push(crate::game::GameEvent { kind: crate::director::EventKind::Door, pos: [k.def.pos[0], 0.0, k.def.pos[2]], time, loud: false, level: Some(DENY_LEVEL), who: Some(who), serial, ..crate::game::GameEvent::zeroed() });
    g.pl[pl].noise = m::max(g.pl[pl].noise, DENY_LEVEL);
    let door_says = pad_door_name(g, &k);
    crate::game::game_msg(g, &format!("the {} pad needs a code you don't have, but somebody on this floor knows it", door_says));
    let here = g.ch[c].pos;
    let where_ = crate::chronicle::room_of(g, here);
    devices_note(g, &DevicesNoteR { device: k.def.name, what: DevicesNoteRWhat::Deny, where_, sent: None });
}
/// Break off keying.   (devices.ts stopCoding)
pub fn stop_coding(g: &mut crate::game::Game) {
    let pl = g.player;
    if g.pl[pl].coding.is_none() { return; }
    g.pl[pl].coding = None;
    g.pl[pl].code_t = 0.0;
}
/// Once a frame: the code going in, the door on the last key; broken off if he leaves or the door stops being against him.   (devices.ts updateCoding)
pub fn update_coding(g: &mut crate::game::Game, dt: f64) {
    let pl = g.player;
    let d = match g.pl[pl].coding { Some(d) => d, None => return };
    let pad = devices_pad_of(g, d);
    let p = g.ch[g.pl[pl].ch].pos;
    let door_at = crate::doors::door_pos(g, d);
    let in_reach = m::min(v3::dist_xz(door_at, p), v3::dist_xz(g.dr[d].frame_centre, p)) < PAD_REACH + 0.25;
    let k = match pad {
        Some(k) if !g.pl[pl].down && in_reach && g.pl[pl].holding.is_none() && g.pl[pl].takedown.is_none() => k,
        _ => { stop_coding(g); return; }
    };
    if !crate::doors::doors_locked_out(g, d, p, None) { stop_coding(g); return; }
    // HANDWORK settled on: his finger on the pad's own keys (the page's branch; the plain clock is not translated)
    let before = g.pl[pl].code_t;
    let hands = crate::handwork::PadHandsPad { def: crate::handwork::PadHandsPadDef { pos: k.def.pos, code: k.def.code.clone(), yaw: k.def.yaw }, n: k.n };
    let more = crate::handwork::pad_hands(g, &hands, dt, CODE_SECS);
    g.pl[pl].code_t = before + more;
    if g.pl[pl].code_t < CODE_SECS { return; }
    // accepted
    g.pl[pl].coding = None;
    g.pl[pl].code_t = 0.0;
    crate::audio_iface::game_audio_like_play(g, "padOpen", Some(k.def.pos), Some(0.6), None);
    let mut keys: JsSet<String> = JsSet::new();
    keys.add(g.dr[d].lock_id.clone());
    let who = g.ch[g.pl[pl].ch].id;
    crate::doors::doors_use(g, d, p, who, Some(&keys));
    let door_says = pad_door_name(g, &k);
    crate::game::game_msg(g, &format!("{} — the {} takes it", spell(&k.def.code), door_says));
    let door_name = g.dr[d].def.name.clone();
    let row = data(&[("door", text(&door_name))]);
    let centre = g.dr[d].frame_centre;
    chron_device(g, ChronDeviceKind::CodeEntered, &row, Some(centre));
    let where_ = crate::chronicle::room_of(g, p);
    devices_note(g, &DevicesNoteR { device: k.def.name, what: DevicesNoteRWhat::Code, where_, sent: None });
}

// ---------------------------------------------------------------- the boxes of the devices (a rule reads them: world.raycast takes a camera's body and bracket; see notes)
/// The devices' boxes into the frame's list, keyed by ordinal.   (devices.ts deviceBoxes)
pub fn device_boxes(g: &mut crate::game::Game, out: &mut Vec<BoxId>) {
    let n0 = out.len();
    device_boxes_raw(g, out);
    let base = crate::scene_boxes::mkey().device;
    for k in n0..out.len() { seam_set_mkey(g, out[k], base + (k - n0) as f64); }
}
/// A camera's bracket, body, lens and LED; a pad's plate, keys and LED; a beam's housings, LED and line.   (devices.ts deviceBoxesRaw)
pub fn device_boxes_raw(g: &mut crate::game::Game, out: &mut Vec<BoxId>) {
    use crate::scene_boxes::{make_box, MakeBoxP};
    let fl = crate::scene_boxes::box_flag();
    let dyn_ = fl.dynamic;
    let deco = bits_or(fl.dynamic, fl.no_shadow);
    for c in g.devices.cameras.clone() {
        let d = g.tab.devices_camera_rt[c].def.clone();
        let (target, slot) = (g.tab.devices_camera_rt[c].target, g.tab.devices_camera_rt[c].slot);
        let ok = alive(g, target);
        let pitch = d.pitch.unwrap_or(crate::scene_level_data::device_defaults().pitch);
        let yaw = cam_yaw(g, slot);
        let look = lens_dir(yaw, pitch);
        let back: V3 = [-m::sin(d.yaw), 0.0, -m::cos(d.yaw)];
        out.push(make_box(g, &MakeBoxP { c: v3::mad(d.pos, back, 0.16), h: [0.025, 0.025, 0.09], yaw: Some(d.yaw), albedo: Some(cam_dark()), flags: Some(dyn_), ..MakeBoxP::zeroed() }));
        let x = v3::normalize(v3::cross([0.0, 1.0, 0.0], look));
        let y = v3::cross(look, x);
        out.push(make_box(g, &MakeBoxP { c: v3::mad(d.pos, look, -0.04), h: [0.055, 0.05, 0.1], yaw: Some(yaw), rot: Some(v3::quat::from_basis(x, y, look)), albedo: Some(cam_col()), flags: Some(dyn_), ..MakeBoxP::zeroed() }));
        out.push(make_box(g, &MakeBoxP { c: v3::mad(d.pos, look, 0.075), h: [0.03, 0.03, 0.02], yaw: Some(yaw), rot: Some(v3::quat::from_basis(x, y, look)), albedo: Some([0.02, 0.02, 0.025]), flags: Some(deco), ..MakeBoxP::zeroed() }));
        let seen = cam_seen(g, slot);
        let hot = seen > 0.02;
        let blink = if !cam_idle(g, slot) { if m::sin(g.time * 18.0) > 0.0 { 1.0 } else { 0.15 } } else { 1.0 };
        let led: V3 = if !ok { [0.0, 0.0, 0.0] } else {
            v3::scale(if hot { led_hot() } else { led_idle() }, blink * (if hot { v3::clamp(0.4 + seen / m::max(0.1, g.tune.camera_suspicion_secs), 0.4, 1.6) } else { 1.0 }))
        };
        out.push(make_box(g, &MakeBoxP { c: v3::add(v3::mad(d.pos, look, 0.02), v3::scale(y, 0.062)), h: [0.012, 0.008, 0.012], yaw: Some(yaw), rot: Some(v3::quat::from_basis(x, y, look)), albedo: Some([0.05, 0.0, 0.0]), emissive: Some(led), flags: Some(deco), ..MakeBoxP::zeroed() }));
    }
    for k in g.devices.keypads.clone() {
        let d = &k.def;
        let n = k.n;
        let yaw = d.yaw;
        out.push(make_box(g, &MakeBoxP { c: v3::mad(d.pos, n, 0.012), h: [0.045, 0.065, 0.012], yaw: Some(yaw), albedo: Some(pad_col()), flags: Some(deco), ..MakeBoxP::zeroed() }));
        out.push(make_box(g, &MakeBoxP { c: v3::add(v3::mad(d.pos, n, 0.026), [0.0, -0.015, 0.0]), h: [0.03, 0.035, 0.003], yaw: Some(yaw), albedo: Some([0.06, 0.06, 0.07]), stuff: Some(crate::scene_stuff::BoxStuff::Metal), flags: Some(deco), ..MakeBoxP::zeroed() }));
        let locked = crate::doors::door_locked(g, k.door) && crate::doors::door_latched(g, k.door);
        let busy = men_of(g).iter().any(|&pl| g.pl[pl].coding == Some(k.door));
        let led: V3 = if busy { v3::scale(amber(), if m::sin(g.time * 30.0) > 0.0 { 1.0 } else { 0.2 }) } else if locked { red() } else { green() };
        out.push(make_box(g, &MakeBoxP { c: v3::add(v3::mad(d.pos, n, 0.026), [0.0, 0.045, 0.0]), h: [0.012, 0.006, 0.003], yaw: Some(yaw), albedo: Some([0.04, 0.04, 0.04]), emissive: Some(v3::scale(led, 0.5)), flags: Some(deco), ..MakeBoxP::zeroed() }));
    }
    for l in g.devices.lasers.clone() {
        let d = g.tab.devices_laser_rt[l].def.clone();
        let (target, slot, dir, len) = (g.tab.devices_laser_rt[l].target, g.tab.devices_laser_rt[l].slot as usize, g.tab.devices_laser_rt[l].dir, g.tab.devices_laser_rt[l].len);
        let ok = alive(g, target);
        let yaw = v3::yaw_of(dir);
        out.push(make_box(g, &MakeBoxP { c: v3::mad(d.from, dir, -0.03), h: [0.03, 0.035, 0.03], yaw: Some(yaw), albedo: Some(emit_col()), flags: Some(deco), ..MakeBoxP::zeroed() }));
        out.push(make_box(g, &MakeBoxP { c: v3::mad(d.to, dir, 0.03), h: [0.03, 0.035, 0.03], yaw: Some(yaw), albedo: Some(emit_col()), flags: Some(deco), ..MakeBoxP::zeroed() }));
        let (last, cut) = { let c = &sim::state::state(g.sim).lights; (c.las_last[slot], c.las_cut[slot]) };
        let glow: V3 = if ok { if g.time - last < 1.5 { v3::scale(red(), if m::sin(g.time * 40.0) > 0.0 { 2.0 } else { 0.2 }) } else { [1.5, 0.1, 0.05] } } else { [0.0, 0.0, 0.0] };
        out.push(make_box(g, &MakeBoxP { c: v3::add(v3::mad(d.from, dir, -0.03), [0.0, 0.043, 0.0]), h: [0.008, 0.005, 0.008], yaw: Some(yaw), albedo: Some([0.05, 0.0, 0.0]), emissive: Some(glow), flags: Some(deco), ..MakeBoxP::zeroed() }));
        if ok && cut > 0.05 { crate::combat::beam_boxes(g, d.from, dir, cut, out, cut < len - 0.05, None, None); }
    }
}

// ---------------------------------------------------------------- geometry
/// Closest points between segments p and b: the distance and the point on b.   (devices.ts segSegClosest)
pub fn seg_seg_closest(p0: V3, p1: V3, b0: V3, b1: V3) -> crate::devices::SegSegClosest {
    let (d1, d2, r) = (v3::sub(p1, p0), v3::sub(b1, b0), v3::sub(p0, b0));
    let (a, e, f) = (v3::dot(d1, d1), v3::dot(d2, d2), v3::dot(d2, r));
    let (s, t);
    if a <= 1e-9 && e <= 1e-9 {
        s = 0.0;
        t = 0.0;
    } else if a <= 1e-9 {
        s = 0.0;
        t = v3::clamp(f / e, 0.0, 1.0);
    } else {
        let c = v3::dot(d1, r);
        if e <= 1e-9 {
            t = 0.0;
            s = v3::clamp(-c / a, 0.0, 1.0);
        } else {
            let bb = v3::dot(d1, d2);
            let denom = a * e - bb * bb;
            let s0 = if denom > 1e-9 { v3::clamp((bb * f - c * e) / denom, 0.0, 1.0) } else { 0.0 };
            let t0 = (bb * s0 + f) / e;
            if t0 < 0.0 {
                t = 0.0;
                s = v3::clamp(-c / a, 0.0, 1.0);
            } else if t0 > 1.0 {
                t = 1.0;
                s = v3::clamp((bb - c) / a, 0.0, 1.0);
            } else {
                t = t0;
                s = s0;
            }
        }
    }
    let on_p = v3::mad(p0, d1, s);
    let on_b = v3::mad(b0, d2, t);
    SegSegClosest { dist: v3::dist(on_p, on_b), on_b }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn b(x: u64) -> f64 { f64::from_bits(x) }
    fn v(a: [u64; 3]) -> V3 { [b(a[0]), b(a[1]), b(a[2])] }
    /// the TypeScript's own bits (bun, EXACT.on) for segSegClosest: both degenerate ends, both clamps, the interior
    #[test]
    fn seg_seg_closest_bits() {
        let rows: &[([u64; 12], u64, [u64; 3])] = &[
            ([0x3fd3333333333333, 0x3ff199999999999a, 0xc00599999999999a, 0x3ffe666666666666, 0x3fd999999999999a, 0x400a666666666666, 0xbffb333333333333, 0x3feccccccccccccd, 0x3fc999999999999a, 0x400199999999999a, 0x3ff4cccccccccccd, 0x3fe6666666666666], 0x3fdde6f64ace9ee3, [0x3ff1845888329deb, 0x3ff2fc81dc14f200, 0x3fe1ddab0c9ac367]),
            ([0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x3ff0000000000000, 0x4000000000000000, 0x4008000000000000, 0x3ff0000000000000, 0x4000000000000000, 0x4008000000000000], 0x400deeea11683f48, [0x3ff0000000000000, 0x4000000000000000, 0x4008000000000000]),
            ([0x3fb999999999999a, 0x3fc999999999999a, 0x3fd3333333333333, 0x3fb999999999999a, 0x3fc999999999999a, 0x3fd3333333333333, 0xbff0000000000000, 0x3fe0000000000000, 0x3fd0000000000000, 0x4010000000000000, 0x3fe8000000000000, 0x3fc0000000000000], 0x3fd73896fd98220d, [0x3fb4936122ee4ba0, 0x3fe1ba856837e3ac, 0x3fcc8af52f9038a8]),
            ([0xbff0000000000000, 0x3fe0000000000000, 0x3fd0000000000000, 0x4010000000000000, 0x3fe8000000000000, 0x3fc0000000000000, 0x3fb999999999999a, 0x3fc999999999999a, 0x3fd3333333333333, 0x3fb999999999999a, 0x3fc999999999999a, 0x3fd3333333333333], 0x3fd73896fd98220d, [0x3fb999999999999a, 0x3fc999999999999a, 0x3fd3333333333333]),
            ([0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x3ff0000000000000, 0x0000000000000000, 0x0000000000000000, 0x0000000000000000, 0x3ff0000000000000, 0x0000000000000000, 0x3ff0000000000000, 0x3ff0000000000000, 0x0000000000000000], 0x3ff0000000000000, [0x0000000000000000, 0x3ff0000000000000, 0x0000000000000000]),
            ([0x4014000000000000, 0x3ff0000000000000, 0x4014000000000000, 0x4018000000000000, 0x3ff0000000000000, 0x4016000000000000, 0x0000000000000000, 0x3ff0000000000000, 0x0000000000000000, 0x3ff0000000000000, 0x3ff0000000000000, 0x3fc999999999999a], 0x4018fe2812a529ee, [0x3ff0000000000000, 0x3ff0000000000000, 0x3fc999999999999a]),
            ([0xc014000000000000, 0x3ff0000000000000, 0xc014000000000000, 0xc018000000000000, 0x3ff0000000000000, 0xc016000000000000, 0x0000000000000000, 0x3ff0000000000000, 0x0000000000000000, 0x3ff0000000000000, 0x3ff0000000000000, 0x3fc999999999999a], 0x401c48c6001f0ac0, [0x0000000000000000, 0x3ff0000000000000, 0x0000000000000000]),
            ([0x3fd5555555555555, 0x3fe5555555555555, 0x3fc2492492492492, 0x3ff4924924924925, 0x3fbc71c71c71c71c, 0x3ffaaaaaaaaaaaab, 0x3feb13b13b13b13b, 0xbfc6969696969697, 0x3fb999999999999a, 0x3fc999999999999a, 0x3fd79435e50d7943, 0xbfe9611a7b9611a8], 0x3fe8cfdc3a074f36, [0x3fe0c962285d3dc4, 0x3fb83e9a25d9c5c2, 0xbfd60bcc76863110]),
        ];
        for (i, d, on) in rows {
            let r = seg_seg_closest(v([i[0], i[1], i[2]]), v([i[3], i[4], i[5]]), v([i[6], i[7], i[8]]), v([i[9], i[10], i[11]]));
            assert_eq!(r.dist.to_bits(), *d);
            assert_eq!([r.on_b[0].to_bits(), r.on_b[1].to_bits(), r.on_b[2].to_bits()], *on);
        }
    }
    #[test]
    fn lens_dir_bits() {
        let rows: &[(u64, u64, [u64; 3])] = &[
            (0x3fe6666666666666, 0xbfd6666666666666, [0x3fe35d799669bc02, 0xbfd5f209a5390a8b, 0x3fe6fdb8bda90d24]),
            (0xc000cccccccccccd, 0x3fc999999999999a, [0xbfeb1274263933bd, 0x3fc96dff233dd2bc, 0xbfdfaa8573ca52b8]),
            (0x4008000000000000, 0xbff199999999999a, [0x3fb0630e90ddbe73, 0xbfec84c54c2d6338, 0xbfdcbd588d04503c]),
        ];
        for (y, p, o) in rows {
            let r = lens_dir(b(*y), b(*p));
            assert_eq!([r[0].to_bits(), r[1].to_bits(), r[2].to_bits()], *o);
        }
    }
    #[test]
    fn sweep_yaw_bits() {
        let mut d = CameraDef2::zeroed();
        d.yaw = 0.5;
        assert_eq!(sweep_yaw(&d, 0.37), 0.5);
        d.sweep = Some([-1.2, 0.8]);
        let rows: &[(u64, u64)] = &[
            (0x0000000000000000, 0xbff3333333333333),
            (0x3fd0000000000000, 0xbfc999999999999c),
            (0x3fd7ae147ae147ae, 0x3fdf02d1dd55735c),
            (0x3feccccccccccccd, 0xbff024ef00ff2d86),
        ];
        for (ph, o) in rows { assert_eq!(sweep_yaw(&d, b(*ph)).to_bits(), *o); }
    }
    #[test]
    fn spell_digits() { assert_eq!(spell("7391"), "7-3-9-1"); }
}
