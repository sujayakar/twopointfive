pub use ::game_types::devices::types;
pub use types::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{format, string::String, vec::Vec};

// ---------------------------------------------------------------- small readers of this file
fn callsign_of(g: &Game, gd: Option<GuardId>) -> Option<String> { gd.map(|x| g.gd[x].callsign.clone()) }
fn data(rows: &[(&str, Option<ChronDeviceDataValue>)]) -> JsMap<String, Option<ChronDeviceDataValue>> {
    let mut o = JsMap::new();
    for (k, v) in rows { o.set(String::from(*k), v.clone()); }
    o
}
fn text(s: &str) -> Option<ChronDeviceDataValue> { Some(ChronDeviceDataValue::Text(String::from(s))) }
fn opt_text(s: &Option<String>) -> Option<ChronDeviceDataValue> { s.as_ref().map(|x| ChronDeviceDataValue::Text(x.clone())) }

// ---------------------------------------------------------------- runtime records
/// The level's cameras, tripwires and keypads from its list; cameras and emitters join the light targets.
pub fn devices_new(g: &mut crate::game::Game) -> crate::devices::Devices {
    let mut this = Devices::blank();
    let defs = g.level.devices.clone().unwrap_or_default();
    let (mut ci, mut li) = (0.0_f64, 0.0_f64);
    for dv in &defs {
        match dv {
            crate::scene_level::DeviceDef::Camera(d) => {
                let dd = crate::scene_level_data::device_defaults();
                let fix = crate::fixtures::Fixture {
                    name: d.name.clone(), group: d.group.clone().unwrap_or(dd.group.clone()), kind: crate::fixtures::FixtureKind::Device, pos: v3::copy(d.pos),
                    on: true, factor: 1.0, mains: d.mains != Some(false), interactive: true, stagger: 1.4 + 0.3 * ci,
                    ..crate::fixtures::Fixture::blank()
                };
                let target = crate::fixtures::fixture_add(g, fix);
                g.targets.push(target);
                let half = d.fov_deg.unwrap_or(dd.fov_deg) / 2.0;
                let k = cone();
                let lamp = g.tab.light.push(crate::render_lights::RtLight { name: format!("{}_cone", d.name), group: String::from("devices"), kind: 1.0, pos: v3::copy(d.pos), dir: [0.0, -1.0, 0.0], color: v3::copy(k.color), intensity: k.intensity, range: d.range.unwrap_or(dd.range) * k.range_k,
                    inner_deg: m::max(1.0, m::min(half, k.outer_max_deg) - 6.0), outer_deg: m::min(half, k.outer_max_deg), radius: k.radius, volumetric: k.volumetric, owner: 0.0, enabled: false, fixture_box: -1.0, flicker: 0.0, ttl: -1.0, age: 0.0, peak_intensity: k.intensity, ..crate::render_lights::RtLight::default() });
                crate::render_lights::light_set_add(g, lamp);
                let cone = lamp;
                ci += 1.0;
                let (rest, period, pitch) = (d.yaw, d.sweep_period.unwrap_or(dd.sweep_period), d.pitch.unwrap_or(dd.pitch));
                let rt = g.tab.devices_camera_rt.push(CameraRt { def: d.clone(), index: ci, target, cone, rest, sweep: d.sweep, period, pitch, yaw: rest, ..CameraRt::blank() });
                this.cameras.push(rt);
            }
            crate::scene_level::DeviceDef::Laser(d) => {
                let dd = crate::scene_level_data::device_defaults();
                let fix = crate::fixtures::Fixture {
                    name: d.name.clone(), group: d.group.clone().unwrap_or(dd.group.clone()), kind: crate::fixtures::FixtureKind::Device, pos: v3::copy(d.from),
                    on: true, factor: 1.0, mains: d.mains != Some(false), interactive: true, stagger: 1.1 + 0.3 * li,
                    ..crate::fixtures::Fixture::blank()
                };
                let target = crate::fixtures::fixture_add(g, fix);
                g.targets.push(target);
                let seg = v3::sub(d.to, d.from);
                let len = v3::len(seg);
                li += 1.0;
                let rt = g.tab.devices_laser_rt.push(LaserRt { def: d.clone(), target, dir: v3::scale(seg, 1.0 / m::max(len, 1e-6)), len, cut: len, ..LaserRt::blank() });
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
/// The keypad on this door, if it is a keypad door.
pub fn devices_pad_of(g: &crate::game::Game, d: DoorId) -> Option<crate::devices::KeypadRt> {
    g.devices.pad_by_door.get(d).cloned()
}
/// `speaker` has just said the code of `lock_id` aloud.
pub fn devices_overhear_said(g: &mut crate::game::Game, speaker: GuardId, lock_id: &str) -> bool {
    overhear(g, speaker, lock_id)
}
/// The keypad whose door stands nearest a floor point.
pub fn devices_pad_near(g: &crate::game::Game, p: V3) -> Option<crate::devices::KeypadRt> {
    let mut best: Option<usize> = None;
    let mut bd = f64::INFINITY;
    for (i, k) in g.devices.keypads.iter().enumerate() {
        let dd = v3::dist_xz(g.dr[k.door].frame_centre, p);
        if dd < bd { bd = dd; best = Some(i); }
    }
    best.map(|i| g.devices.keypads[i].clone())
}
/// Powered, unbroken, not knocked out: the device works this frame.
fn alive(g: &crate::game::Game, t: FixtureId) -> bool {
    g.tab.fixture[t].factor > 0.05 && !g.tab.fixture[t].broken
}
/// Live play: devices sense.
fn sensing(g: &crate::game::Game) -> bool {
    g.ai_enabled && g.tune.devices && !g.staged.facts.fight
}
/// Lit-seconds per second a camera's integrator fills at.
fn lens_gain(g: &crate::game::Game, seat: &crate::seats::SeatRef, dist: f64, range: f64, smoke_trans: f64) -> f64 {
    let pl = seat.player;
    let light = g.shown[g.pl[pl].ch].visibility;
    let dist_f = v3::clamp(1.0 - (dist - 3.0) / m::max(1.0, range - 3.0), 0.2, 1.0);
    let crouch = if g.pl[pl].crouch { 0.75 } else { 1.0 };
    m::pow(light, 1.5) * dist_f * crouch * v3::clamp((smoke_trans - 0.3) / 0.5, 0.0, 1.0) * crate::kitload::load_seen(g, g.pl[seat.player].ch)
}
/// The lens's look direction at yaw / pitch.
pub fn lens_dir(yaw: f64, pitch: f64) -> V3 {
    let cp = m::cos(pitch);
    [m::sin(yaw) * cp, m::sin(pitch), m::cos(yaw) * cp]
}
/// Pan the cameras, run their pictures, run the beams, key the code being entered.
pub fn update_devices(g: &mut crate::game::Game, dt: f64) {
    let live = sensing(g);
    for c in g.devices.cameras.clone() { update_camera(g, c, dt, live); }
    for l in g.devices.lasers.clone() { let on = live && g.tune.lasers; update_laser(g, l, dt, on); }
    crate::seats::each_seat(g, &mut |v: &mut Game| update_coding(v, dt));
}
/// One camera's frame: the pan, the feed dropping, the picture, what it does about it.
pub fn update_camera(g: &mut crate::game::Game, c_2: CameraRtId, dt: f64, live: bool) {
    let c = c_2;
    let d = g.tab.devices_camera_rt[c].def.clone();
    let index = g.tab.devices_camera_rt[c].index;
    let cones = g.tune.device_cones;
    let (ok, down) = crate::fixtures::camera_pan(g, c, dt, live, cones);
    if down && live { device_down(g, &d.name, &format!("camera {}", index), d.pos); }
    if !ok || !live { return; }
    let dd = crate::scene_level_data::device_defaults();
    let range = d.range.unwrap_or(dd.range);
    let half = d.fov_deg.unwrap_or(dd.fov_deg) / 2.0;
    let look = lens_dir(g.tab.devices_camera_rt[c].yaw, d.pitch.unwrap_or(dd.pitch));
    let e = crate::guards::Eye { pos: d.pos, look, range, half_deg: half, near_m: 0.0, near_deg: half, v_half_deg: Some(half) };
    // (every player the lens can see: the feed goes by the one it sees best; none in sight, by the nearest, as a lens with nobody in it)
    let first = crate::seats::nearest_or_first(g, &d.pos);
    let (mut pc, mut seen, mut gain) = (g.pl[first].ch, false, 0.0);
    for pl in crate::seats::men(g) {
        if crate::condition::down(g, g.pl[pl].ch) { continue; }
        let c = g.pl[pl].ch;
        let chest = g.ch[c].bone_or(crate::character::Bone::Chest, 1.1);
        let air = crate::sense::clear_between(g, d.pos, chest);
        let vis = crate::guards::eye_contact(g, e, c, false, air, None);
        if !vis.visible { continue; }
        let got = lens_gain(g, &crate::seats::seat(g, crate::seats::seat_of_man(g, pl)), vis.dist, range, air);
        if !seen || got > gain { (pc, seen, gain) = (c, true, got); }
    }
    let at = g.ch[pc].pos;
    let (suspect, alarm) = crate::fixtures::camera_see(g, c, dt, seen, gain, at);
    if suspect { camera_suspect(g, c); }
    if alarm { camera_alarm(g, c); }
}
/// The picture has movement: a man sent, the radio, the debrief.
fn camera_suspect(g: &mut crate::game::Game, c_2: CameraRtId) {
    let c = c_2;
    let index = g.tab.devices_camera_rt[c].index;
    let name = g.tab.devices_camera_rt[c].def.name.clone();
    let seen = g.tab.devices_camera_rt[c].last_seen.unwrap();
    let where_ = crate::chronicle::room_of(g, seen);
    let sent = send_to_look(g, seen);
    let sign = callsign_of(g, sent);
    let tail = match &sign { Some(s) => format!(" — {}, go and look", s), None => String::new() };
    control(g, &format!("camera {} has movement in the {}{}", index, where_, tail));
    let row = data(&[("camera", text(&name)), ("n", Some(ChronDeviceDataValue::Num(index))), ("sent", opt_text(&sign))]);
    chron_device(g, ChronDeviceKind::CameraSpotted, &row, Some(seen));
}
/// The picture has him: the floor's fix, everybody sharp, the floor up a step.
fn camera_alarm(g: &mut crate::game::Game, c_2: CameraRtId) {
    let c = c_2;
    let index = g.tab.devices_camera_rt[c].index;
    let name = g.tab.devices_camera_rt[c].def.name.clone();
    let p = g.tab.devices_camera_rt[c].last_seen.unwrap();
    let where_ = crate::chronicle::room_of(g, p);
    control(g, &format!("camera {}: that's him — the {}. all posts, the {}", index, where_, where_));
    g.alarm.pos[0] = p[0];
    g.alarm.pos[1] = 0.0;
    g.alarm.pos[2] = p[2];
    g.alarm.placed = true;
    for i in 0..g.guards.len() {
        let id = g.guards[i];
        if !crate::condition::down(g, g.gd[id].ch) && g.gd[id].held.is_none() && g.gd[id].unit.is_none() {
            crate::belief::word(g, id, p);
            crate::mind::raise(g, id, 0.5, "devices");
            if g.gd[id].state == crate::game::GuardState::Patrol {
                crate::mind::set_state(g, id, crate::game::GuardState::Suspicious, "devices");
                g.gd[id].react_t = 0.8;
                g.gd[id].search_t = 0.0;
                g.gd[id].drop_path();
            }
        }
    }
    let to = if crate::guards::escalation_of(g) >= 1.0 { 2.0 } else { 1.0 };
    crate::guards::raise_escalation(g, to, None, Some(crate::guards::RaiseEscalationCause::Device), None);
    let row = data(&[("camera", text(&name)), ("n", Some(ChronDeviceDataValue::Num(index)))]);
    chron_device(g, ChronDeviceKind::CameraAlarm, &row, Some(p));
}

/// One beam's frame: where it ends, what breaks it, the trip.
fn update_laser(g: &mut crate::game::Game, l: LaserRtId, dt: f64, live: bool) {
    let d = g.tab.devices_laser_rt[l].def.clone();
    let (dir, len) = (g.tab.devices_laser_rt[l].dir, g.tab.devices_laser_rt[l].len);
    let (ok, down) = crate::fixtures::laser_open(g, l);
    if down && sensing(g) {
        let room = crate::chronicle::room_of(g, d.from);
        device_down(g, &d.name, &format!("the {} beam", room), d.from);
    }
    if !ok { return; }
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
    let mut by: Option<PlayerId> = None;
    let mut at: Option<V3> = None;
    let mut cut = reach;
    for pl in crate::seats::men(g) {
        if who.is_some() { break; }
        if !crate::condition::down(g, g.pl[pl].ch) && !g.player_invisible {
            let c = g.pl[pl].ch;
            at = body_breaks(g, c, d.from, end);
            if at.is_some() { who = Some("player"); by = Some(pl); }
        }
        if who.is_none() {
            if let Some(dg) = g.pl[pl].dragging() {
                let c = g.gd[dg].ch;
                at = body_breaks(g, c, d.from, end);
                if at.is_some() { who = Some("body"); }
            }
        }
    }
    if who.is_none() {
        let back = m::max(dt, 1.0 / 60.0);
        for it in g.items.items.clone() {
            let kind = g.tab.item[it].kind;
            if g.tab.item[it].settled || (kind != crate::throwables::ItemKind::Smoke && kind != crate::throwables::ItemKind::Flash && kind != crate::throwables::ItemKind::Frag) { continue; }
            let (ip, iv) = (g.tab.item[it].pos, g.tab.item[it].vel);
            let prev = [ip[0] - iv[0] * back, ip[1] - iv[1] * back, ip[2] - iv[2] * back];
            let q = seg_seg_closest(prev, ip, d.from, end);
            if q.dist < g.tab.item[it].radius + BEAM_MARGIN { who = Some(kind.as_str()); at = Some(q.on_b); break; }
        }
    }
    if let Some(a) = at { cut = m::min(cut, v3::dist(d.from, a)); }
    for o in g.guards.clone() {
        if crate::condition::down(g, g.gd[o].ch) { continue; }
        let c = g.gd[o].ch;
        let t = crate::character::character_ray_hit(g, c, d.from, dir, cut);
        if t >= 0.0 { cut = m::min(cut, t); }
    }
    for o in crate::civilians::civilian_bodies(g) {
        let c = g.gd[o].ch;
        let t = crate::character::character_ray_hit(g, c, d.from, dir, cut);
        if t >= 0.0 { cut = m::min(cut, t); }
    }
    if crate::fixtures::laser_close(g, l, live, cut, who.is_some()) {
        laser_trip(g, l, who.unwrap(), by, at.unwrap());
    }
}
/// Does any capsule of `c` cross the beam a→b: the point on the beam where it does.
fn body_breaks(g: &crate::game::Game, c_2: CharId, a: V3, b: V3) -> Option<V3> {
    let h = g.ch[c_2].bone(crate::character::Bone::Hips)?;
    let along = v3::clamp(((h[0] - a[0]) * (b[0] - a[0]) + (h[2] - a[2]) * (b[2] - a[2])) / m::max(1e-6, m::pow(v3::dist_xz(a, b), 2.0)), 0.0, 1.0);
    if v3::dist_xz(h, v3::lerp3(a, b, along)) > 1.6 { return None; }
    let caps = crate::character::character_capsules(g, c_2);
    for k in &caps.caps {
        let q = seg_seg_closest([k[0], k[1], k[2]], [k[4], k[5], k[6]], a, b);
        if q.dist < k[3] + BEAM_MARGIN { return Some(q.on_b); }
    }
    None
}
/// A beam broken: the chirp, the radio, a man sent, by the beam's level.
fn laser_trip(g: &mut crate::game::Game, l: LaserRtId, who: &str, by: Option<PlayerId>, at: V3) {
    let d = g.tab.devices_laser_rt[l].def.clone();
    let where_ = crate::chronicle::room_of(g, at);
    let level: String = match d.alarm { Some(x) => String::from(x.as_str()), None => crate::scene_level_data::device_defaults().alarm };
    crate::audio_iface::game_audio_like_play(g, "alarmChirp", Some(d.from), Some(0.7), None);
    if let Some(pl) = by { g.pl[pl].noise = m::max(g.pl[pl].noise, 0.3); }
    let sent: Option<GuardId>;
    if level == "alarm" {
        control(g, &format!("beam broken in the {} — that's an intruder. weapons out", where_));
        g.alarm.pos[0] = at[0];
        g.alarm.pos[1] = 0.0;
        g.alarm.pos[2] = at[2];
        g.alarm.placed = true;
        sent = send_to_look(g, at);
        let to = if crate::guards::escalation_of(g) >= 1.0 { 2.0 } else { 1.0 };
        crate::guards::raise_escalation(g, to, None, Some(crate::guards::RaiseEscalationCause::Device), None);
    } else {
        sent = send_to_look(g, at);
        let tail = match callsign_of(g, sent) { Some(s) => format!(" — {}, check it", s), None => String::from(" — somebody check it") };
        control(g, &format!("beam broken in the {}{}", where_, tail));
    }
    let sign = callsign_of(g, sent);
    let row = data(&[("laser", text(&d.name)), ("by", text(who)), ("level", text(&level)), ("sent", opt_text(&sign))]);
    chron_device(g, ChronDeviceKind::LaserTripped, &row, Some(at));
}
/// A camera or an emitter shot dead: control loses the feed and sends the nearest calm man to look at it.
fn device_down(g: &mut crate::game::Game, name: &str, spoken: &str, pos: V3) {
    let seed = g.fx_seed;
    g.fx_seed += 1.0;
    crate::sparks::sparks_emit_burst(g, pos, crate::sparks::SparksEmitBurstO { count: 14.0, speed: 2.2, spread: 1.0, lift: 0.5, life: 0.5 }, js::to_u32(seed * 7919.0) as f64);
    let floor = crate::scene_collision::nav_grid_nearest_free_point(g, pos[0], pos[2]).unwrap_or([pos[0], 0.0, pos[2]]);
    let sent = send_to_look(g, floor);
    let sign = callsign_of(g, sent);
    let tail = match &sign { Some(s) => format!(" — {}, get eyes on it", s), None => String::from(" — somebody get eyes on it") };
    control(g, &format!("{} just went dark{}", spoken, tail));
    let row = data(&[("device", text(name)), ("sent", opt_text(&sign))]);
    chron_device(g, ChronDeviceKind::DeviceDown, &row, Some(floor));
}
/// The nearest calm, free staff man is sent to `pos` on control's word: a task that survives the suspicion decay.
pub fn send_to_look(g: &mut crate::game::Game, pos: V3) -> Option<GuardId> {
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
        g.gd[id].task = Some(crate::game::GuardTask::Arm0(crate::game::GuardTaskArm0 { kind: crate::game::GuardTaskArm0Kind::Device, pos: v3::copy(goal) }));
        crate::mind::fix(g, id, Some(v3::copy(goal)), "devices");
        crate::mind::raise(g, id, errand().awareness, "devices");
        crate::mind::set_state(g, id, crate::game::GuardState::Suspicious, "devices");
        g.gd[id].react_t = errand().react_t;
        g.gd[id].search_t = 0.0;
        g.gd[id].drop_path();
    }
    crate::game::game_say(g, id, "copy, control — on my way", Some(true), None);
    let c = g.gd[id].ch;
    let at = g.ch[c].bone(crate::character::Bone::Head).unwrap_or(g.ch[c].pos);
    crate::audio_iface::game_audio_like_play(g, "radio", Some(at), Some(0.5), None);
    Some(id)
}
/// Control's voice on the net.
fn control(g: &mut crate::game::Game, text: &str) {
    crate::cues::cues_radio(g, &format!("CONTROL — {}", text));
    crate::audio_iface::game_audio_like_play(g, "radio", None, Some(0.55), None);
    let mut row: JsMap<String, Option<crate::chronicle::ChronicleDataValue>> = JsMap::new();
    row.set(String::from("who"), Some(crate::chronicle::ChronicleDataValue::Text(String::from("CONTROL"))));
    row.set(String::from("text"), Some(crate::chronicle::ChronicleDataValue::Text(String::from(text))));
    crate::chronicle::chronicle(g, crate::chronicle::ChronKind::Radio, Some(&row), None);
}
/// The debrief's rows, through the mission's own live gate.
fn chron_device(g: &mut crate::game::Game, kind: crate::devices::ChronDeviceKind, data: &JsMap<String, Option<crate::devices::ChronDeviceDataValue>>, pos: Option<V3>) -> Option<crate::chronicle::ChronEvent> {
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
/// The player learns the code of lock `lock_id`; whether it was news.
pub fn learn_code(g: &mut crate::game::Game, lock_id: &str, how: crate::devices::LearnCodeHow, from: Option<GuardId>) -> bool {
    let pl = g.player;
    if g.pl[pl].codes.holds(lock_id) { return false; }
    let k = match g.devices.keypads.iter().find(|k| k.def.name == lock_id) { Some(k) => k.clone(), None => return false };
    g.pl[pl].codes.add(String::from(lock_id));
    let sign = callsign_of(g, from);
    let door_says = pad_door_name(g, &k);
    // (not with a man in his arm: Sam's bubble holds back the next question, and the man's own answer has just said it)
    if g.pl[pl].holding().is_none() {
        let time = g.time;
        crate::player::say_over_sam(g, &format!("got the {} code.", door_says), time);
    }
    let door_name = g.dr[k.door].def.name.clone();
    let row = data(&[("door", text(&door_name)), ("how", text(how.as_str())), ("from", opt_text(&sign))]);
    let at = from.map(|f| g.pos_of(f));
    chron_device(g, ChronDeviceKind::CodeLearned, &row, at);
    true
}
/// A man has just said the code aloud: the player learns it iff it carried to him; whether he now knows it.
fn overhear(g: &mut crate::game::Game, speaker: GuardId, lock_id: &str) -> bool {
    let mut any = false;
    crate::seats::each_seat(g, &mut |g| { if overhear_here(g, speaker, lock_id, any) { any = true; } });
    any
}
/// … by the man of the seat that stands; `told`: another man has already learnt it just now, and the telling is not said again
fn overhear_here(g: &mut crate::game::Game, speaker: GuardId, lock_id: &str, told: bool) -> bool {
    let pl = g.player;
    if crate::condition::down(g, g.pl[pl].ch) || g.pl[pl].codes.holds(lock_id) { return g.pl[pl].codes.holds(lock_id); }
    let mouth = g.ch[g.gd[speaker].ch].bone_or(crate::character::Bone::Head, 1.6);
    let ear_up = if g.pl[pl].crouch { 1.0 } else { 1.6 };
    let ear = g.ch[g.pl[pl].ch].bone_or(crate::character::Bone::Head, ear_up);
    let prop = crate::weather::hear_path(g, mouth, ear, None);
    if prop.path_len > g.k.overhear.path_m || prop.carry < g.k.overhear.carry { return false; }
    if told { g.pl[pl].codes.add(String::from(lock_id)); return true; }
    learn_code(g, lock_id, LearnCodeHow::Overheard, Some(speaker)) || g.pl[pl].codes.holds(lock_id)
}
/// The digits as said ('7-3-9-1').
pub fn spell(code: &str) -> String {
    let mut out = String::new();
    for (i, ch) in code.chars().enumerate() {
        if i > 0 { out.push('-'); }
        out.push(ch);
    }
    out
}
/// What a pad's door is called on the HUD.
pub fn pad_door_name(g: &crate::game::Game, k: &crate::devices::KeypadRt) -> String {
    let name = &g.dr[k.door].def.name;
    if name == "server_manager" { String::from("side door") } else { format!("{} door", name.replace('_', " ")) }
}
/// Is `d` a keypad door locked against the player where he stands, and does he know its code.
pub fn pad_state(g: &crate::game::Game, seat: &crate::seats::SeatRef, d: DoorId, from: V3) -> Option<crate::devices::PadState> {
    let pad = devices_pad_of(g, d)?;
    if !crate::doors::doors_locked_out(g, d, from, None) { return None; }
    let pl = seat.player;
    let known = g.pl[pl].codes.has(g.dr[d].lock_id.clone());
    Some(PadState { pad, known })
}
/// The 'code' row: start keying it in. HANDWORK settled on: every key beeps under his finger (pad_hands), no first beep here.
pub fn start_coding(g: &mut crate::game::Game, d: DoorId) -> bool {
    let pl = g.player;
    let here = g.player_pos(pl);
    let st = pad_state(g, &crate::seats::here(g), d, here);
    match st {
        Some(s) if s.known && g.pl[pl].coding().is_none() && !g.cond[g.pl[pl].ch].down => {}
        _ => return false,
    }
    crate::deed::begin(g, crate::game::Deed::Code(crate::game::Coding { door: d, t: 0.0 }));
    g.pl[pl].hand_wait = 0.0;
    g.sets[g.seat].hand.f_held = true;
    true
}
/// The 'nocode' row: a deny tone at the plate, a hair on the meters, a word.
pub fn deny_pad(g: &mut crate::game::Game, d: DoorId) {
    let k = match devices_pad_of(g, d) { Some(k) => k, None => return };
    crate::audio_iface::game_audio_like_play(g, "deny", Some(k.def.pos), Some(0.45), None);
    let pl = g.player;
    let c = g.pl[pl].ch;
    let (time, who, serial) = (g.time, g.ch[c].id, g.serial());
    g.events.push(crate::game::GameEvent { kind: crate::director::EventKind::Door, pos: [k.def.pos[0], 0.0, k.def.pos[2]], time, loud: false, level: Some(DENY_LEVEL), who: Some(who), serial, ..crate::game::GameEvent::default() });
    g.pl[pl].noise = m::max(g.pl[pl].noise, DENY_LEVEL);
}
/// Once a frame: the code going in, the door on the last key; broken off if he leaves or the door stops being against him.
fn update_coding(g: &mut crate::game::Game, dt: f64) {
    let pl = g.player;
    let Some(&crate::game::Coding { door: d, t: before }) = g.pl[pl].coding() else { return };
    let pad = devices_pad_of(g, d);
    let p = g.player_pos(pl);
    let door_at = crate::doors::door_pos(g, d);
    let in_reach = m::min(v3::dist_xz(door_at, p), v3::dist_xz(g.dr[d].frame_centre, p)) < PAD_REACH + 0.25;
    let k = match pad {
        Some(k) if !crate::condition::down(g, g.pl[pl].ch) && in_reach => k,
        _ => { g.pl[pl].coding_over(); return; }
    };
    if !crate::doors::doors_locked_out(g, d, p, None) { g.pl[pl].coding_over(); return; }
    // his finger on the pad's own keys
    let hands = crate::handwork::PadHandsPad { def: crate::handwork::PadHandsPadDef { pos: k.def.pos, code: k.def.code.clone(), yaw: k.def.yaw }, n: k.n };
    let more = crate::handwork::pad_hands(g, &hands, before, dt, CODE_SECS);
    if before + more < CODE_SECS {
        if let Some(c) = g.pl[pl].coding_mut() { c.t = before + more; }
        return;
    }
    // accepted
    g.pl[pl].coding_over();
    crate::audio_iface::game_audio_like_play(g, "padOpen", Some(k.def.pos), Some(0.6), None);
    let mut keys: JsSet<String> = JsSet::new();
    keys.add(g.dr[d].lock_id.clone());
    let who = g.player_id(pl);
    crate::doors::doors_use(g, d, p, who, Some(&keys));
    let door_name = g.dr[d].def.name.clone();
    let row = data(&[("door", text(&door_name))]);
    let centre = g.dr[d].frame_centre;
    chron_device(g, ChronDeviceKind::CodeEntered, &row, Some(centre));
}

// ---------------------------------------------------------------- the boxes of the devices (a rule reads them: a ray takes a camera's body and bracket)
/// The devices' boxes into the frame's list, keyed by ordinal.
pub fn device_boxes(g: &mut crate::game::Game, out: &mut Vec<AnyBoxId>) {
    let n0 = out.len();
    device_boxes_raw(g, out);
    let base = crate::scene_boxes::mkey().device;
    for k in n0..out.len() { g.tab.edit(out[k]).mkey = Some(base + (k - n0) as f64); }
}
/// A camera's bracket, body, lens and LED; a pad's plate, keys and LED; a beam's housings, LED and line.
fn device_boxes_raw(g: &mut crate::game::Game, out: &mut Vec<AnyBoxId>) {
    use crate::scene_boxes::{make_box_frame as make_box, MakeBoxP};   // (every box here is pushed straight onto the frame's list)
    let fl = crate::scene_boxes::box_flag();
    let dyn_ = fl.dynamic;
    let deco = js::or_bits(fl.dynamic, fl.no_shadow);
    for c in g.devices.cameras.clone() {
        let d = g.tab.devices_camera_rt[c].def.clone();
        let target = g.tab.devices_camera_rt[c].target;
        let ok = alive(g, target);
        let pitch = d.pitch.unwrap_or(crate::scene_level_data::device_defaults().pitch);
        let yaw = g.tab.devices_camera_rt[c].yaw;
        let look = lens_dir(yaw, pitch);
        let back: V3 = [-m::sin(d.yaw), 0.0, -m::cos(d.yaw)];
        out.push(make_box(g, &MakeBoxP { c: v3::mad(d.pos, back, 0.16), h: [0.025, 0.025, 0.09], yaw: Some(d.yaw), albedo: Some(cam_dark()), flags: Some(dyn_), ..MakeBoxP::default() }).into());
        let x = v3::normalize(v3::cross([0.0, 1.0, 0.0], look));
        let y = v3::cross(look, x);
        out.push(make_box(g, &MakeBoxP { c: v3::mad(d.pos, look, -0.04), h: [0.055, 0.05, 0.1], yaw: Some(yaw), rot: Some(v3::quat::from_basis(x, y, look)), albedo: Some(cam_col()), flags: Some(dyn_), ..MakeBoxP::default() }).into());
        out.push(make_box(g, &MakeBoxP { c: v3::mad(d.pos, look, 0.075), h: [0.03, 0.03, 0.02], yaw: Some(yaw), rot: Some(v3::quat::from_basis(x, y, look)), albedo: Some([0.02, 0.02, 0.025]), flags: Some(deco), ..MakeBoxP::default() }).into());
        let seen = g.tab.devices_camera_rt[c].seen;
        let hot = seen > 0.02;
        let blink = if g.tab.devices_camera_rt[c].state != crate::fixtures::CameraState::Idle { if m::sin(g.time * 18.0) > 0.0 { 1.0 } else { 0.15 } } else { 1.0 };
        let led: V3 = if !ok { [0.0, 0.0, 0.0] } else {
            v3::scale(if hot { led_hot() } else { led_idle() }, blink * (if hot { v3::clamp(0.4 + seen / m::max(0.1, g.tune.camera_suspicion_secs), 0.4, 1.6) } else { 1.0 }))
        };
        out.push(make_box(g, &MakeBoxP { c: v3::add(v3::mad(d.pos, look, 0.02), v3::scale(y, 0.062)), h: [0.012, 0.008, 0.012], yaw: Some(yaw), rot: Some(v3::quat::from_basis(x, y, look)), albedo: Some([0.05, 0.0, 0.0]), emissive: Some(led), flags: Some(deco), ..MakeBoxP::default() }).into());
    }
    for k in g.devices.keypads.clone() {
        let d = &k.def;
        let n = k.n;
        let yaw = d.yaw;
        out.push(make_box(g, &MakeBoxP { c: v3::mad(d.pos, n, 0.012), h: [0.045, 0.065, 0.012], yaw: Some(yaw), albedo: Some(pad_col()), flags: Some(deco), ..MakeBoxP::default() }).into());
        out.push(make_box(g, &MakeBoxP { c: v3::add(v3::mad(d.pos, n, 0.026), [0.0, -0.015, 0.0]), h: [0.03, 0.035, 0.003], yaw: Some(yaw), albedo: Some([0.06, 0.06, 0.07]), stuff: Some(crate::scene_stuff::BoxStuff::Metal), flags: Some(deco), ..MakeBoxP::default() }).into());
        let locked = g.dr[k.door].leaf.locked && g.dr[k.door].leaf.latched;
        let busy = crate::seats::men(g).iter().any(|&pl| g.pl[pl].coding().map(|c| c.door) == Some(k.door));
        let led: V3 = if busy { v3::scale(amber(), if m::sin(g.time * 30.0) > 0.0 { 1.0 } else { 0.2 }) } else if locked { red() } else { green() };
        out.push(make_box(g, &MakeBoxP { c: v3::add(v3::mad(d.pos, n, 0.026), [0.0, 0.045, 0.0]), h: [0.012, 0.006, 0.003], yaw: Some(yaw), albedo: Some([0.04, 0.04, 0.04]), emissive: Some(v3::scale(led, 0.5)), flags: Some(deco), ..MakeBoxP::default() }).into());
    }
    for l in g.devices.lasers.clone() {
        let d = g.tab.devices_laser_rt[l].def.clone();
        let (target, dir, len) = (g.tab.devices_laser_rt[l].target, g.tab.devices_laser_rt[l].dir, g.tab.devices_laser_rt[l].len);
        let ok = alive(g, target);
        let yaw = v3::yaw_of(dir);
        out.push(make_box(g, &MakeBoxP { c: v3::mad(d.from, dir, -0.03), h: [0.03, 0.035, 0.03], yaw: Some(yaw), albedo: Some(emit_col()), flags: Some(deco), ..MakeBoxP::default() }).into());
        out.push(make_box(g, &MakeBoxP { c: v3::mad(d.to, dir, 0.03), h: [0.03, 0.035, 0.03], yaw: Some(yaw), albedo: Some(emit_col()), flags: Some(deco), ..MakeBoxP::default() }).into());
        let (last, cut) = (g.tab.devices_laser_rt[l].last_trip_at, g.tab.devices_laser_rt[l].cut);
        let glow: V3 = if ok { if g.time - last < 1.5 { v3::scale(red(), if m::sin(g.time * 40.0) > 0.0 { 2.0 } else { 0.2 }) } else { [1.5, 0.1, 0.05] } } else { [0.0, 0.0, 0.0] };
        out.push(make_box(g, &MakeBoxP { c: v3::add(v3::mad(d.from, dir, -0.03), [0.0, 0.043, 0.0]), h: [0.008, 0.005, 0.008], yaw: Some(yaw), albedo: Some([0.05, 0.0, 0.0]), emissive: Some(glow), flags: Some(deco), ..MakeBoxP::default() }).into());
        if ok && cut > 0.05 {
            let mut beam = crate::rig_props::Carried::default();
            crate::combat::beam_boxes(d.from, dir, cut, &mut beam, cut < len - 0.05, false, false);
            beam.laid(g, out);
        }
    }
}

// ---------------------------------------------------------------- geometry
/// Closest points between segments p and b: the distance and the point on b.
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

    fn near(a: f64, b: f64) -> bool { (a - b).abs() < 1e-12 }
    fn near3(a: V3, b: V3) -> bool { v3::dist(a, b) < 1e-12 }

    /// A laser is broken by how near its beam comes to a man's limb: on lines that pass each other, that is where they cross over.
    #[test]
    fn the_nearest_points_of_two_crossing_lines_are_where_they_cross() {
        let r = seg_seg_closest([-1.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, -1.0, 2.0], [0.0, 1.0, 2.0]);
        assert!(near(r.dist, 2.0));
        assert!(near3(r.on_b, [0.0, 0.0, 2.0]));
    }

    /// Off the end of either line, the end is the nearest it has.
    #[test]
    fn the_nearest_point_stops_at_the_ends_of_the_line() {
        // the second line lies beyond the first's end, in line with it
        let r = seg_seg_closest([0.0; 3], [1.0, 0.0, 0.0], [3.0, 0.0, 0.0], [5.0, 0.0, 0.0]);
        assert!(near(r.dist, 2.0));
        assert!(near3(r.on_b, [3.0, 0.0, 0.0]));
        let r = seg_seg_closest([0.0; 3], [1.0, 0.0, 0.0], [-5.0, 0.0, 0.0], [-3.0, 0.0, 0.0]);
        assert!(near(r.dist, 3.0));
        assert!(near3(r.on_b, [-3.0, 0.0, 0.0]));
        // the second line stops short of the first: its own end is the nearest it has
        let r = seg_seg_closest([5.0, 1.0, 5.0], [6.0, 1.0, 5.5], [0.0, 1.0, 0.0], [1.0, 1.0, 0.2]);
        assert!(near3(r.on_b, [1.0, 1.0, 0.2]));
    }

    /// Two level lines one above the other are a metre apart wherever they are measured; a line of no length is a point; the distance does not depend on which line is asked about first.
    #[test]
    fn parallel_lines_and_points_have_a_distance_too() {
        let r = seg_seg_closest([0.0; 3], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [1.0, 1.0, 0.0]);
        assert!(near(r.dist, 1.0));
        assert!(near(r.on_b[1], 1.0));
        let (p, q): (V3, V3) = ([0.1, 0.2, 0.3], [1.0, 2.0, 3.0]);
        let r = seg_seg_closest(p, p, q, q);
        assert!(near(r.dist, v3::dist(p, q)));
        assert_eq!(r.on_b, q);
        let r = seg_seg_closest([0.0; 3], [0.0; 3], [1.0, 2.0, 3.0], [1.0, 2.0, 3.0]);
        assert_eq!(r.on_b, [1.0, 2.0, 3.0]);
        let a = seg_seg_closest(p, p, [-1.0, 0.5, 0.25], [4.0, 0.75, 0.125]);
        let b = seg_seg_closest([-1.0, 0.5, 0.25], [4.0, 0.75, 0.125], p, p);
        assert!(near(a.dist, b.dist));
        assert_eq!(b.on_b, p);
    }

    /// A camera's lens looks where its yaw and pitch say, in the game's axes: yaw 0 is +z, a quarter turn is +x, pitch is up.
    #[test]
    fn a_lens_looks_by_its_yaw_and_pitch() {
        assert!(near3(lens_dir(0.0, 0.0), [0.0, 0.0, 1.0]));
        assert!(near3(lens_dir(m::PI / 2.0, 0.0), [1.0, 0.0, 0.0]));
        assert!(near3(lens_dir(1.3, m::PI / 2.0), [0.0, 1.0, 0.0]));
        let d = lens_dir(0.7, -0.35);
        assert!(near(v3::len(d), 1.0));
        assert!(d[1] < 0.0 && near(m::atan2(d[0], d[2]), 0.7));
    }

    /// A code is read out digit by digit.
    #[test]
    fn a_code_is_spelled_digit_by_digit() { assert_eq!(spell("7391"), "7-3-9-1"); }
}
