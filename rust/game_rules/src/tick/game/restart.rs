use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// A cut: `lens.target = copy(at); lens.desired = copy(at)`. A whole write of `desired` is what Seat.takeCut watches for, here by its serial.
fn cut_lens(g: &mut Game, cam: CameraId, at: V3) {
    let lens = &mut g.tab.camera[cam];
    lens.target = at;
    lens.desired = at;
    lens.desired_serial += 1;
}

/// What a door's noise does: the man who made it is that loud, the sound plays, and it is filed for the men to hear.
pub fn game_on_door_event(g: &mut crate::game::Game, e: crate::doors::DoorEvent) {
    use crate::doors::DoorSound;
    if let Some(pl) = crate::seats::man_by_id(g, e.who) { g.pl[pl].noise = m::max(g.pl[pl].noise, e.level); }
    match e.sound {
        DoorSound::Creak => audio_play(g, "doorCreak", e.pos, 0.5 + 0.5 * e.level / 0.3),
        DoorSound::Bang => audio_play(g, "doorBang", e.pos, 0.9),
        DoorSound::Latch => audio_play(g, "doorLatch", e.pos, 0.6),
        DoorSound::Bash => {
            // (under 1: a leaf ripped round by its handle at a run, the same slam, lighter and shorter)
            if e.level < 1.0 { audio_play_rate(g, "doorBash", e.pos, 0.6, 1.15); } else { audio_play(g, "doorBash", e.pos, 0.9); }
        }
        DoorSound::Rattle => audio_play(g, "doorRattle", e.pos, 0.35 + 2.0 * e.level),
        DoorSound::Pick => { let rate = 0.9 + crate::dice::draw_world(g, "game::game_on_door_event#1") * 0.25; audio_play_rate(g, "lockPick", e.pos, 0.5, rate); }
        DoorSound::Unlock => audio_play(g, "lockOpen", e.pos, 0.4 + 1.3 * e.level),
        DoorSound::Kick => {
            audio_play_rate(g, "doorBash", e.pos, 1.0, 0.9);
            audio_play(g, "lockBreak", e.pos, 0.95);
            let kicker = match crate::seats::man_by_id(g, e.who) {
                Some(p) => Some(g.pl[p].ch),
                None => g.guards.iter().map(|&x| g.gd[x].ch).find(|&c| g.ch[c].id == e.who),
            };
            let from = match kicker { Some(c) => g.ch[c].pos, None => e.pos };
            crate::particle_fx::kick_dust_fx(g, e.door, from);
            if crate::doors::door_lock_broken(g, e.door) {
                let from = match kicker { Some(c) => g.ch[c].pos, None => e.pos };
                crate::particle_fx::lock_splinter_fx(g, e.door, from);
            }
            crate::destruction::destruction_thud(g, e.pos, Some(1.2));
        }
        DoorSound::Thud => {
            // a boot at the face a shut door opens toward: the slam of it, nothing splintered
            audio_play_rate(g, "doorBash", e.pos, 1.0, 0.8);
        }
        DoorSound::Clang => {
            // a boot on the steel stair door: the ring of it, no keep to splinter
            audio_play_rate(g, "doorBash", e.pos, 1.0, 0.7);
            audio_play_rate(g, "stairDoor", e.pos, 0.55, 1.3);
        }
    }
    // heard: a kick is its own kind, floor-wide; everything else a 'door' noise with a level, placed at the leaf
    let kicked = e.sound == DoorSound::Kick;
    let loud = (e.sound == DoorSound::Bash && e.level >= 1.0) || kicked || e.sound == DoorSound::Clang || e.sound == DoorSound::Thud || (e.sound == DoorSound::Bang && e.level > 0.6);
    let (time, serial) = (g.time, g.serial());
    g.events.push(crate::game::GameEvent {
        kind: if kicked { crate::director::EventKind::Kick } else { crate::director::EventKind::Door },
        pos: [e.pos[0], 0.0, e.pos[2]],
        time,
        loud,
        level: Some(e.level),
        who: Some(e.who),
        serial,
        ..crate::game::GameEvent::default()
    });
}

/// The rolling chairs that are heard (`voice`, a number of its own each time one sets off), once a step right after the props have moved; nothing in the simulation reads this and no die is drawn.
pub(super) fn game_roll_tick(g: &mut crate::game::Game, dt: f64) {
    if g.rolls.is_empty() { return; }
    let voice = crate::game::roll_voice();
    let mut i = g.rolls.len();
    while i > 0 {
        i -= 1;
        let (p, rx, rz) = { let r = &g.rolls[i]; (r.p, r.x, r.z) };
        let here = g.props.props.contains(&p);
        let (px, pz) = (g.tab.props_prop[p].x, g.tab.props_prop[p].z);
        let mut v = if here && dt > 0.0 { m::hyp2(px - rx, pz - rz) / dt } else { 0.0 };
        if !v.is_finite() { v = 0.0; }
        g.rolls[i].x = px;
        g.rolls[i].z = pz;
        g.rolls[i].k = m::min(1.0, v / voice.full_v);
        let rolling = g.tab.props_prop[p].rolling;
        if here && rolling && v > voice.min_v {
            g.rolls[i].move_t = g.time;
            if !js::truthy(g.rolls[i].voice) {
                g.roll_seq += 1.0;
                g.rolls[i].voice = g.roll_seq;
            }
        } else if js::truthy(g.rolls[i].voice) && (!here || g.time - g.rolls[i].move_t > voice.grace || g.time < g.rolls[i].move_t) {
            g.rolls[i].voice = 0.0;
        }
        if !js::truthy(g.rolls[i].voice) && !(here && rolling) { g.rolls.remove(i); }
    }
}

/// Furniture noise: a scrape or a knock, heard like door sounds and blamed on whoever moved it. A castor chair coasting is voiced by the roll tick and heard by nobody.
pub fn game_on_prop_event(g: &mut crate::game::Game, e: crate::props::PropEvent) {
    use crate::props::{PropEventRoll, PropEventSound};
    if e.sound == PropEventSound::Roll {
        if e.roll == Some(PropEventRoll::Start) && !g.rolls.iter().any(|r| r.p == e.prop) {
            g.rolls.push(crate::game::Roll { p: e.prop, x: e.pos[0], z: e.pos[2], k: 0.0, move_t: -1e9, voice: 0.0 });   // (where it was as it started: the step may have moved it on)
        }
        return;
    }
    if let Some(pl) = crate::seats::man_by_id(g, e.who) { g.pl[pl].noise = m::max(g.pl[pl].noise, e.level); }
    let (thud_rate, scrape_rate) = { let t = &g.tab.props_prop[e.prop].tune; (t.thud_rate, t.scrape_rate) };
    let jitter = 0.95 + crate::dice::draw_world(g, "game::game_on_prop_event#1") * 0.1;
    if e.sound == PropEventSound::Thud {
        audio_play_rate(g, "propThud", e.pos, v3::clamp(0.35 + 0.15 * e.speed, 0.3, 1.0), thud_rate * jitter);
        crate::destruction::destruction_thud(g, e.pos, Some(0.4 + 0.3 * e.speed));
    } else if !e.rolling {
        audio_play_rate(g, "propScrape", e.pos, v3::clamp(0.25 + 0.18 * e.speed, 0.25, 1.0), scrape_rate * jitter);
    }
    let (time, serial) = (g.time, g.serial());
    g.events.push(crate::game::GameEvent {
        kind: crate::director::EventKind::Prop,
        pos: v3::copy(e.pos),
        time,
        loud: e.level > 0.7,
        level: Some(e.level),
        who: Some(e.who),
        serial,
        ..crate::game::GameEvent::default()
    });
}

/// Panel: one level up by hand.
pub fn game_escalate(g: &mut crate::game::Game) {
    let to = if g.escalation >= 1.0 { 2.0 } else { 1.0 };
    crate::guards::raise_escalation(g, to, None, None, None);
}

/// Is this record somebody on the floor right now: a man of the shift, a unit or a civilian?
pub fn game_on_floor(g: &crate::game::Game, gd: GuardId) -> bool {
    g.guards.contains(&gd) || g.civilians.contains(&gd)
}

/// The die to pass on: the caller's own, or none for the game's. (`rnd` handed to two calls in turn)
fn lend_rnd<'a>(rnd: &'a mut Option<&mut dyn FnMut(&mut Game) -> f64>) -> Option<&'a mut dyn FnMut(&mut Game) -> f64> {
    match rnd {
        Some(f) => Some(&mut **f),
        None => None,
    }
}

/// One exchange of the interrogation: the man answers from his own state, his line goes up, and the door's key or the pad's code comes with what he gives up.
pub fn game_interrogate(g: &mut crate::game::Game, gd: GuardId, rnd: Option<&mut dyn FnMut(&mut crate::game::Game) -> f64>) -> Option<crate::dialogue::DialoguePick> {
    let mut rnd = rnd;
    let me = g.player;
    if crate::condition::down(g, g.pl[me].ch) || crate::condition::down(g, g.gd[gd].ch) || !(g.guards.contains(&gd) || g.civilians.contains(&gd)) { return None; }
    let held = match &g.pl[me].holding { Some(h) => Some(h.g), None => None };
    if held != Some(gd) {
        // not the man in his arm: in reach and nothing solid between, statics or a leaf
        let (a, b) = (g.ch[g.pl[me].ch].pos, g.ch[g.gd[gd].ch].pos);
        if v3::dist_xz(a, b) > crate::dialogue::INTERROGATE_REACH || crate::scene_collision::solid_blocked(g, [a[0], 1.0, a[2]], [b[0], 1.0, b[2]], None) { return None; }
    }
    let pick = if g.gd[gd].civ.is_some() { crate::civilians::civilian_answer(g, gd, lend_rnd(&mut rnd)) } else { crate::dialogue::interrogate(g, gd, lend_rnd(&mut rnd)) };
    let Some(pick) = pick else { return None };
    crate::guards::say(g, gd, &pick.text, Some(false), Some(true));   // (forced: a held man's only voice is his answers)
    if let Some(bubble) = g.gd[gd].bubble.as_mut() {
        bubble.dur = v3::clamp(1.2 + 0.07 * pick.text.encode_utf16().count() as f64, 2.5, 6.5);   // held speech runs longer than a bark's cap
    }
    let _ = crate::chronicle::chron_interrogated(g, gd, pick.kind.map(|k| k.as_str()), pick.stage_n);   // ahead of the key coming off his belt, its own row
    if pick.kind == Some(crate::dialogue::IntelKind::Door) {
        crate::player::take_key(g, gd, lend_rnd(&mut rnd));
    } else if pick.kind == Some(crate::dialogue::IntelKind::Code) {
        let at = g.ch[g.gd[gd].ch].pos;
        if let Some(k) = crate::devices::devices_pad_near(g, at) {
            let _ = crate::devices::learn_code(g, &k.def.name, crate::devices::LearnCodeHow::Told, Some(gd));
        }
    }
    Some(pick)
}

/// Let go of the man in Sam's arm at once, no shove.
pub fn game_free_held(g: &mut crate::game::Game, why: crate::player::FreeHeldWhy) {
    let why = why;
    crate::player::free_held(g, why);
}

/// `g.audio.play(name, pos, gain)`.
fn audio_play(g: &mut Game, name: &str, pos: V3, gain: f64) {
    crate::audio_iface::game_audio_like_play(g, name, Some(pos), Some(gain), None);
}

/// `g.audio.play(name, pos, gain, { rate })`.
fn audio_play_rate(g: &mut Game, name: &str, pos: V3, gain: f64, rate: f64) {
    crate::audio_iface::game_audio_like_play(g, name, Some(pos), Some(gain), Some(&crate::audio_iface::PlayOpts::Rate { rate }));
}

// ------------------------------------------------------------ the panel's handles and small doors

pub fn game_clear_nearest_room(g: &mut crate::game::Game) { crate::guards::force_clear(g) }

/// Camera follow position: a spectated guard, `at`, else the player.
pub fn game_follow_pos(g: &crate::game::Game, at: Option<V3>) -> V3 {
    let gd = if g.spectate >= 0.0 { g.guards.get(g.spectate as usize).copied() } else { None };
    if let Some(gd) = gd { let c = g.gd[gd].ch; return g.ch[c].pos; }
    if let Some(a) = at { return a; }
    let c = g.pl[g.player].ch;
    g.ch[c].pos
}

pub fn game_teleport_player(g: &mut crate::game::Game, p: V3) { crate::player::teleport_player(g, p) }

/// The thud of a ragdoll's trunk or head meeting something, polled after the body's update.
pub fn game_body_thud(g: &mut crate::game::Game, c_2: CharId) {
    let hit = match g.ch[c_2].ragdoll { Some(r) => crate::ragdoll::ragdoll_take_impact(g, r, None, None), None => 0.0 };
    if hit > 1.4 {
        let at = match g.ch[c_2].bones.get(crate::character::Bone::Spine2) { Some(b) => *b, None => g.ch[c_2].pos };
        crate::audio_iface::game_audio_like_play(g, "bodyFall", Some(at), Some(v3::clamp(0.35 + hit * 0.13, 0.4, 1.0)), None);
        crate::destruction::destruction_thud(g, at, Some(0.5 + hit * 0.15));
    }
}

pub fn game_repair_lights(g: &mut crate::game::Game) { crate::lighting::repair_lights(g) }

/// Everything still in the air or the sound queue from what just happened: fresh guards must not inherit it.
pub fn game_clear_aftermath(g: &mut crate::game::Game) {
    crate::frag::frag_reset(g);
    crate::morale::morale_reset(g, None);
    g.items.items.clear();
    g.rings.clear();
    g.shells.clear();
    crate::sparks::sparks_clear(g);
    crate::tracers::tracers_clear(g);
    crate::particle_fx::clear_particle_fx(g);
    crate::moths::moths_clear(g);
    crate::destruction::destruction_clear_moving(g);
    crate::shards::glass_shards_clear(g);
    g.events.clear();
    let seats = g.seats.clone();
    for s in seats {
        let pl = crate::cmd::seat_man(g, s);
        g.pl[pl].pending_throw = None;
        g.pl[pl].throw_held = false;
        g.pl[pl].throw_preview = None;
    }
}

/// Was this fixture's switch on in the level as authored.
pub fn game_authored_on(g: &crate::game::Game, t: FixtureId) -> bool { g.tab.fixture[t].authored }

/// The lights' rows stood on the list as it is: as the level is built, and again when a fixture is hung after that.
pub fn game_stand_lights(g: &mut crate::game::Game) {
    let list: Vec<LightId> = g.engine.lights.lights.clone();
    let targets: Vec<FixtureId> = g.targets.clone();
    let cones: Vec<(LightId, FixtureId)> = g.devices.cameras.iter().map(|&c| { let r = &g.tab.devices_camera_rt[c]; (r.cone, r.target) }).collect();
    crate::fixtures::light_home_stand(g, &list, &targets, &cones);
}

/// The player's restart, from outside a step: the run's first step stands again with what an evening keeps (savestate/control.rs). In a step it is asked for, and made as the step ends.
pub fn game_restart(g: &mut crate::game::Game) {
    crate::savestate::control::restart(g);
}

/// Every view is cut to its fresh man, unless a spectator has the camera.
pub fn game_cut_to_men(g: &mut crate::game::Game) {
    if g.spectate >= 0.0 { return; }
    let seats = g.seats.clone();
    for s in seats {
        let lens = g.tab.cmd_seat[s].cam;
        let p = crate::seats::with_seat(g, s, &mut |g| game_follow_pos(g, None));
        cut_lens(g, lens, p);
    }
}

///
pub fn game_set_blackout(g: &mut crate::game::Game, seconds: f64, cause: crate::lighting::SetBlackoutCause) {
    let cause = cause;
    crate::lighting::set_blackout(g, seconds, cause)
}

pub fn game_end_blackout(g: &mut crate::game::Game) { crate::lighting::end_blackout(g) }

/// `cause` defaults to 'shot' here, as the TypeScript's parameter does.
pub fn game_light_event(g: &mut crate::game::Game, t: FixtureId, cause: Option<crate::lighting::LightEventCause>, sw_at: Option<V3>) {
    let cause = cause.unwrap_or(crate::lighting::LightEventCause::Shot);
    crate::lighting::light_event(g, t, Some(cause), sw_at)
}

pub fn game_ocp(g: &mut crate::game::Game, t: Option<FixtureId>) { crate::lighting::ocp(g, t) }

pub fn game_toggle_drag(g: &mut crate::game::Game, which: Option<GuardId>) { crate::player::toggle_drag(g, which) }

// ------------------------------------------------------------ update

/// One tick as a host calls it: its live input and view sampled into the tick's record, the sim run off that record alone.
pub fn game_update(g: &mut crate::game::Game, dt: f64, input: InputId, cam: CameraId, canvas: crate::host_hud::View) {
    crate::math_dice::use_dice(g);
    crate::quiet::quiet_mission_current(g);
    let me = game_host_seat(g);
    if g.restart_cut {
        g.restart_cut = false;
        let cut = crate::cmd::seat_take_cut(g, me);
        if let Some(cut) = cut {
            if g.tab.camera[cam].pin.is_none() {
                cut_lens(g, cam, cut);
                crate::camera::follow_camera_rebuild(g, cam);
            }
        }
    }
    g.cmd_buf.take();
    let c = crate::seats::with_seat(g, me, &mut |g| crate::cmd::sample_cmd(g, input, cam, canvas));
    g.cmd_buf = Some(c.clone());
    if g.seats.len() == 1 {
        game_step(g, dt, &c);
    } else {
        let seats = g.seats.clone();
        let mut cmds = Vec::with_capacity(seats.len());
        for s in seats {
            if Some(s) == g.local_seat { cmds.push(c.clone()); } else { cmds.push(crate::cmd::seat_handed(g, s, &c)); }
        }
        game_step_seats(g, dt, &cmds);
    }
    game_host_after(g, input, cam);
}

/// What the tick hands back for the camera of the seat this host shows, and that seat's live view put back.
fn game_host_after(g: &mut crate::game::Game, input: InputId, cam: CameraId) {
    let me = game_host_seat(g);
    let me_cam = g.tab.cmd_seat[me].cam;
    let (peek, kick) = (g.tab.camera[me_cam].peek_offset, g.tab.camera[me_cam].kick);
    g.tab.camera[cam].peek_offset = peek;
    g.tab.camera[cam].kick = kick;
    if let Some(cut) = crate::cmd::seat_take_cut(g, me) { cut_lens(g, cam, cut); }
    let vp = g.tab.camera[cam].view_proj;
    crate::seats::with_seat(g, me, &mut |g| g.view_proj = Some(crate::game::GameViewProj::Float32Array(vp.to_vec())));
    crate::seats::with_seat(g, me, &mut |g| crate::orders::squad_orders_host_view(g, input));
}

/// The tick for a game of one seat.
pub fn game_step(g: &mut crate::game::Game, dt: f64, cmd: &crate::cmd::PlayerCmd) {
    let buf = vec![cmd.clone()];
    game_step_seats(g, dt, &buf);
}

/// The local seat, or the first for a host that shows nobody.
pub fn game_host_seat(g: &crate::game::Game) -> SeatId { g.local_seat.unwrap_or(g.seats[0]) }

/// The world asked aside: `fn_` leaves nothing on the game's scratch.
pub(in crate::tick) fn game_aside<T: Clone + core::fmt::Debug>(g: &mut crate::game::Game, fn_: &mut dyn FnMut(&mut crate::game::Game) -> T) -> T {
    // (a fresh table each ask: game::AsideOwn kept only the lens. An empty table is a clean one, collision.ts aside)
    let mut col_own = crate::scene_collision::AsideOwn::default();
    crate::scene_collision::aside(g, &mut col_own, fn_)
}

/// A lens of the host's to put a record's view on while it asks aside: scratch, in no snapshot.
pub fn game_aside_lens(g: &mut crate::game::Game) -> CameraId {
    if let Some(lens) = g.alone_own.lens { return lens; }
    let lens = g.tab.camera.push(crate::camera::FollowCamera::blank());
    g.alone_own.lens = Some(lens);
    lens
}
