//! src/game/game.ts lines 1271 to 1582
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// A cut: `lens.target = copy(at); lens.desired = copy(at)`. A whole write of `desired` is what Seat.takeCut watches for, here by its serial.   (game.ts:1485, :1536, :1554)
fn cut_lens(g: &mut Game, cam: CameraId, at: V3) {
    let lens = &mut g.tab.camera[cam];
    lens.target = at;
    lens.desired = at;
    lens.desired_serial += 1;
}

/// What a door's noise does: the man who made it is that loud, the sound plays, and it is filed for the men to hear. (game.ts Game.onDoorEvent)
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
        DoorSound::Pick => { let rate = 0.9 + crate::math_dice::sim_random(g) * 0.25; audio_play_rate(g, "lockPick", e.pos, 0.5, rate); }
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
            crate::destruction::destruction_thud(g, e.pos, Some(1.2), Some(e.who));
        }
        DoorSound::Thud => {
            // a boot at the face a shut door opens toward: the slam of it, nothing splintered
            audio_play_rate(g, "doorBash", e.pos, 1.0, 0.8);
            if let Some(p) = crate::seats::man_by_id(g, e.who) {
                let tool = match &g.pl[p].kick { Some(k) => k.tool.is_some(), None => false };
                if !tool { game_msg(g, "it opens toward you, so the frame took the kick. that was loud"); }
            }
        }
        DoorSound::Clang => {
            // a boot on the steel stair door: the ring of it, no keep to splinter
            audio_play_rate(g, "doorBash", e.pos, 1.0, 0.7);
            audio_play_rate(g, "stairDoor", e.pos, 0.55, 1.3);
            if crate::seats::is_man_id(g, e.who) { game_msg(g, "it's a steel fire door, so it won't give. that was loud"); }
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
        ..crate::game::GameEvent::zeroed()
    });
}

/// The rolling chairs' voices, once a step right after the props have moved; nothing in the simulation reads this and no die is drawn. (game.ts Game.rollTick)
pub fn game_roll_tick(g: &mut crate::game::Game, dt: f64) {
    if g.rolls.is_empty() { return; }
    let voice = crate::game::roll_voice();
    let mut i = g.rolls.len();
    while i > 0 {
        i -= 1;
        let (p, rx, rz) = { let r = &g.rolls[i]; (r.p, r.x, r.z) };
        let here = g.props.props.contains(&p);
        let (px, pz) = (crate::props::prop_x(g, p), crate::props::prop_z(g, p));
        let mut v = if here && dt > 0.0 { m::hyp2(px - rx, pz - rz) / dt } else { 0.0 };
        if !v.is_finite() { v = 0.0; }
        g.rolls[i].x = px;
        g.rolls[i].z = pz;
        g.rolls[i].k = m::min(1.0, v / voice.full_v);
        let rolling = crate::props::prop_rolling(g, p);
        if here && rolling && v > voice.min_v {
            g.rolls[i].move_t = g.time;
            if !js::truthy(g.rolls[i].voice) {
                g.roll_seq += 1.0;
                let tok = g.roll_seq;
                g.rolls[i].voice = tok;
                crate::audio_iface::game_audio_like_play(g, "chairRoll", Some([px, 0.05, pz]), Some(voice.gain), Some(&crate::audio_iface::PlayOpts::ChairRoll { tok }));   // (the `follow` closure: audio_follow_now answers by the token)
            }
        } else if js::truthy(g.rolls[i].voice) && (!here || g.time - g.rolls[i].move_t > voice.grace || g.time < g.rolls[i].move_t) {
            g.rolls[i].voice = 0.0;
        }
        if !js::truthy(g.rolls[i].voice) && !(here && rolling) { g.rolls.remove(i); }
    }
}

/// Furniture noise: a scrape or a knock, heard like door sounds and blamed on whoever moved it. A castor chair coasting is voiced by the roll tick and heard by nobody. (game.ts Game.onPropEvent)
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
    let jitter = 0.95 + crate::math_dice::sim_random(g) * 0.1;
    if e.sound == PropEventSound::Thud {
        audio_play_rate(g, "propThud", e.pos, v3::clamp(0.35 + 0.15 * e.speed, 0.3, 1.0), thud_rate * jitter);
        crate::destruction::destruction_thud(g, e.pos, Some(0.4 + 0.3 * e.speed), Some(e.who));
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
        ..crate::game::GameEvent::zeroed()
    });
}

/// A fresh shift: every system's memory of the old one goes, the response team first. (game.ts Game.resetGuards)
pub fn game_reset_guards(g: &mut crate::game::Game) {
    crate::powerout::power_out_reset(g);
    crate::rescuecheck::rescue_check_reset(g);
    crate::doze::doze_reset(g, None);
    crate::seats::each_seat(g, &mut |v: &mut Game| {
        crate::close::close_reset(v);
        crate::stalk::stalk_reset(v);
        crate::tapstep::tap_reset(v);
    });
    crate::roomfight::room_fight_reset(g);
    crate::cover::cover_reset(g);
    crate::entrypoints::entry_points_reset(g);
    crate::cleared::cleared_rooms_reset(g);
    crate::doorpath::door_path_reset(g);
    crate::wallreact::wall_react_reset_2(g);
    crate::coverfire::coverfire_reset(g);
    crate::doorforce::door_force_reset(g);
    crate::qrf::reset_qrf(g);
    crate::guards::reset_guards(g);
    crate::civilians::reset_civilians(g);
    crate::tactics::tactics_reset(g);
    crate::lanes::lanes_reset(g);
    crate::picture::picture_reset(g);
}

/// Panel: one level up by hand. (game.ts Game.escalate)
pub fn game_escalate(g: &mut crate::game::Game) {
    let to = if g.escalation >= 1.0 { 2.0 } else { 1.0 };
    crate::guards::raise_escalation(g, to, None, None, None);
}

/// Panel: straight back to calm. (game.ts Game.standDown)
pub fn game_stand_down(g: &mut crate::game::Game) {
    crate::guards::stand_down_escalation(g);
}

/// Panel: the human shield's standoff. (game.ts Game.standoffSummary)
pub fn game_standoff_summary(g: &mut crate::game::Game) -> String {
    crate::guards::standoff_summary(g)
}

/// Panel: what each unfound body reads on the light meter. (game.ts Game.bodyLightSummary)
pub fn game_body_light_summary(g: &mut crate::game::Game) -> String {
    crate::guards::body_light_summary(g)
}

/// Is this record somebody on the floor right now: a man of the shift, a unit or a civilian? (game.ts Game.onFloor)
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

/// One exchange of the interrogation: the man answers from his own state, his line goes up, and the door's key or the pad's code comes with what he gives up. (game.ts Game.interrogate)
pub fn game_interrogate(g: &mut crate::game::Game, gd: GuardId, rnd: Option<&mut dyn FnMut(&mut crate::game::Game) -> f64>) -> Option<crate::dialogue::DialoguePick> {
    let mut rnd = rnd;
    let me = g.player;
    if g.pl[me].down || g.gd[gd].state == crate::game::GuardState::Dead || !(g.guards.contains(&gd) || g.civilians.contains(&gd)) { return None; }
    let held = match &g.pl[me].holding { Some(h) => Some(h.g), None => None };
    if held != Some(gd) {
        // not the man in his arm: in reach and nothing solid between, statics or a leaf
        let (a, b) = (g.ch[g.pl[me].ch].pos, g.ch[g.gd[gd].ch].pos);
        if v3::dist_xz(a, b) > crate::dialogue::INTERROGATE_REACH || crate::scene_collision::solid_blocked(g, [a[0], 1.0, a[2]], [b[0], 1.0, b[2]], None) { return None; }
    }
    let pick = if g.gd[gd].civ.is_some() { crate::civilians::civilian_answer(g, gd, lend_rnd(&mut rnd)) } else { crate::dialogue::interrogate(g, gd, lend_rnd(&mut rnd)) };
    let Some(pick) = pick else { return None };
    crate::guards::say(g, gd, &pick.text, Some(false), Some(true), None);   // (forced: a held man's only voice is his answers)
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

/// Let go of the man in Sam's arm at once, no shove. (game.ts Game.freeHeld)
pub fn game_free_held(g: &mut crate::game::Game, why: crate::game::GameFreeHeldWhy) {
    let why = match why {
        crate::game::GameFreeHeldWhy::Shot => crate::player::FreeHeldWhy::Shot,
        crate::game::GameFreeHeldWhy::Cancel => crate::player::FreeHeldWhy::Cancel,
    };
    crate::player::free_held(g, why);
}

/// The living man nearest Sam within `reach` metres; `foe`: only men hostile to the player. (game.ts Game.nearestLivingGuard)
pub fn game_nearest_living_guard(g: &crate::game::Game, seat: &crate::seats::SeatRef, reach: Option<f64>, foe: Option<bool>) -> Option<GuardId> {
    let foe = foe.unwrap_or(false);
    let mut best: Option<GuardId> = None;
    let mut bd = reach.unwrap_or(m::INF);
    for gd in g.guards.clone() {
        if g.gd[gd].state == crate::game::GuardState::Dead || (foe && !crate::teams::hostile_to(g, gd, crate::teams::HostileToO::Player)) { continue; }
        let d = v3::dist_xz(g.ch[g.gd[gd].ch].pos, g.ch[g.pl[seat.player].ch].pos);
        if d <= bd { bd = d; best = Some(gd); }
    }
    best
}

/// Sandbox: lean on whoever is within reach; a note if nobody is. (game.ts Game.interrogateNearest)
pub fn game_interrogate_nearest(g: &mut crate::game::Game, rnd: Option<&mut dyn FnMut(&mut crate::game::Game) -> f64>) -> Option<crate::dialogue::DialoguePick> {
    if g.pl[g.player].down { game_note(g, "you're down \u{2014} nobody answers to a man on the floor"); return None; }
    let Some(gd) = game_nearest_living_guard(g, &crate::seats::here(g), Some(crate::dialogue::INTERROGATE_REACH), Some(true)) else {
        game_note(g, &format!("nobody hostile within {} m to lean on \u{2014} walk up to a living guard", js::num_str(crate::dialogue::INTERROGATE_REACH)));
        return None;
    };
    let pick = game_interrogate(g, gd, rnd);
    if pick.is_none() {
        let line = format!("{} is in reach but there is something solid between you", g.gd[gd].callsign);
        game_note(g, &line);
    }
    pick
}

/// Panel: the interrogation engine's read of the man nearest Sam, at any distance. (game.ts Game.dialogueSummary)
pub fn game_dialogue_summary(g: &mut crate::game::Game) -> String {
    let Some(gd) = game_nearest_living_guard(g, &crate::seats::here(g), Some(m::INF), Some(true)) else { return String::from("dialogue: nobody left standing"); };
    let d = v3::dist_xz(g.ch[g.gd[gd].ch].pos, g.ch[g.pl[g.player].ch].pos);
    let s = crate::dialogue::dialogue_summary(g, gd);
    if d <= crate::dialogue::INTERROGATE_REACH { format!("dialogue \u{25B8} {} \u{B7} in reach", s) } else { format!("dialogue \u{25B8} {} \u{B7} {} m off", s, js::to_fixed(d, 1)) }
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

/// (game.ts clearNearestRoom)
pub fn game_clear_nearest_room(g: &mut crate::game::Game) { crate::guards::force_clear(g) }

/// Camera follow position: a spectated guard, a rail's look, `at`, else the player.   (game.ts followPos)
pub fn game_follow_pos(g: &mut crate::game::Game, at: Option<V3>) -> V3 {
    let gd = if g.spectate >= 0.0 { g.guards.get(g.spectate as usize).copied() } else { None };
    if let Some(gd) = gd { let c = g.gd[gd].ch; return g.ch[c].pos; }
    if let Some(p) = crate::rails::rails_look(g) { return p; }
    if let Some(a) = at { return a; }
    let c = g.pl[g.player].ch;
    g.ch[c].pos
}

/// (game.ts teleportPlayer)
pub fn game_teleport_player(g: &mut crate::game::Game, p: V3) { crate::player::teleport_player(g, p) }

/// The thud of a ragdoll's trunk or head meeting something, polled after the body's update.   (game.ts bodyThud)
pub fn game_body_thud(g: &mut crate::game::Game, c_2: CharId) {
    let hit = match g.ch[c_2].ragdoll { Some(r) => crate::ragdoll::ragdoll_take_impact(g, r, None, None), None => 0.0 };
    if hit > 1.4 {
        let at = match g.ch[c_2].bones.get(crate::character::Bone::Spine2) { Some(b) => *b, None => g.ch[c_2].pos };
        crate::audio_iface::game_audio_like_play(g, "bodyFall", Some(at), Some(v3::clamp(0.35 + hit * 0.13, 0.4, 1.0)), None);
        crate::destruction::destruction_thud(g, at, Some(0.5 + hit * 0.15), None);
    }
}

/// (game.ts repairLights)
pub fn game_repair_lights(g: &mut crate::game::Game) { crate::lighting::repair_lights(g) }

/// Everything still in the air or the sound queue from what just happened: fresh guards must not inherit it.   (game.ts clearAftermath)
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

/// The FN2000 in or out of the kit, applied to the man standing there now.   (game.ts setLoadout)
pub fn game_set_loadout(g: &mut crate::game::Game, with_rifle: bool) {
    g.fn2000 = with_rifle;
    let pl = g.player;
    if !with_rifle && g.pl[pl].slot == crate::game::SlotKind::Rifle {
        g.pl[pl].slot = crate::game::SlotKind::Pistol;
        let rifle = g.pl[pl].rifle;
        if crate::weapons::firearm_reloading(g, rifle) {
            let c = g.pl[pl].ch;
            crate::character::anim_cancel_reload(g, c);
        }
        crate::weapons::firearm_cancel_reload(g, rifle);
    }
    g.pl[pl].has_rifle = with_rifle;
}

/// Was this fixture's switch on in the level as authored.   (game.ts authoredOn)
pub fn game_authored_on(g: &mut crate::game::Game, t: FixtureId) -> bool { crate::sim_lightshome::fixture_authored(g, t) }

/// The lights' rows stood on the list as it is: as the level is built, and again when a fixture is hung after that.   (game.ts standLights)
pub fn game_stand_lights(g: &mut crate::game::Game) {
    let list: Vec<LightId> = g.engine.lights.lights.clone();
    let targets: Vec<FixtureId> = g.targets.clone();
    let beacons: Vec<crate::sim_lightshome::BeaconFact> = g.beacons.iter().map(|b| crate::sim_lightshome::BeaconFact { light: b.light, phase: b.phase, target: b.target }).collect();
    let cones: Vec<crate::sim_lightshome::LightHomeStandConesItem> = g.devices.cameras.iter().map(|&c| { let r = &g.tab.devices_camera_rt[c]; crate::sim_lightshome::LightHomeStandConesItem { light: r.cone, target: r.target, slot: r.slot } }).collect();
    crate::sim_lightshome::light_home_stand(g, &list, &targets, &beacons, &cones);
}

/// The level as authored: every switch back, every door, the marks, the panes, the debris.   (game.ts resetLevelState)
pub fn game_reset_level_state(g: &mut crate::game::Game) {
    let targets = g.targets.clone();
    for t in targets {
        let a = crate::sim_lightshome::fixture_authored(g, t);
        crate::sim_lightshome::light_home_in(g, crate::sim_lightshome::light_in().on, Some(t), Some(if a { 1.0 } else { 0.0 }), None, None, None);
    }
    let doors = g.doors.list.clone();
    for d in doors { crate::doors::door_reset(g, d); }
    crate::render_decals::decals_clear(g);
    crate::glass::reset_glass(g);
    crate::glasspen::reset_glass_pen(g);
    crate::winbars::reset_bars(g);
    crate::glassbreak::glass_break_reset(g);   // (before the destruction's: the walls' nav re-bake in there must see whole panes)
    crate::destruction::destruction_reset(g);
    crate::shards::glass_shards_reset(g);
    crate::wet::clear_wet_prints(g);
    crate::snow_blast::clear_snow_blast(g);
    crate::snow_marks::clear_snow_marks(g);
    if let Some(mut t) = g.snow_tracks.take() {   // (the stub takes g and the tracks inside it: out of g for the call)
        crate::snow_tracks::clear_snow_tracks(g, &t);
        g.snow_tracks = Some(t);
    }
    crate::hours::hour_after_reset(g);
    crate::air_dust::air_dust_reset(g);
}

/// (game.ts crackTest)
pub fn game_crack_test(g: &mut crate::game::Game) -> f64 { crate::glass::crack_test(g) }

/// (game.ts wetSummary)
pub fn game_wet_summary(g: &crate::game::Game) -> String { crate::wet::wet_summary(g) }

/// The clocks the last try left running, as a new Game has them.   (game.ts cleanSlate)
pub fn game_clean_slate(g: &mut crate::game::Game) {
    g.alarm.raised_at = -100.0;
    g.alarm.pos = [0.0, 0.0, 0.0];
    let seats = g.seats.clone();
    for &s in &seats { crate::seats::with_seat(g, s, &mut |g| g.torch_on_t = -100.0); }
    g.last_sting_t = -100.0;
    crate::gunwall::gun_wall_fresh(g);
    crate::seats::each_seat(g, &mut |v: &mut Game| crate::guards::ground_fresh(v));
    crate::wind::wind_fresh(g);
}

/// The player's restart: restartEncounter, the camera cut to the fresh man, the host told to start the picture's histories from nothing.   (game.ts restart)
pub fn game_restart(g: &mut crate::game::Game, said: Option<&str>) {
    game_restart_encounter(g);
    if let Some(s) = said { if !s.is_empty() { crate::game::game_msg(g, s); } }   // (after it: restartEncounter takes the old encounter's lines off the HUD)
    if !g.touring && !crate::director::director_showing(g) && g.spectate < 0.0 {
        let seats = g.seats.clone();
        for s in seats {
            let lens = g.tab.cmd_seat[s].cam;
            let p = crate::seats::with_seat(g, s, &mut |g| game_follow_pos(g, None));
            cut_lens(g, lens, p);
        }
        g.restart_cut = true;
        g.stats.cleanrestart.cuts += 1;
    }
    crate::host_api::host_api_restart_picture(g, "level restarted");
    g.stats.cleanrestart.pictures += 1;
}

/// Restart the encounter: a fresh Player at the insertion point with full kit, fresh Guards on their routes, lights repaired, the level as authored.   (game.ts restartEncounter)
pub fn game_restart_encounter(g: &mut crate::game::Game) {
    crate::math_dice::use_dice(g);
    crate::sensors::sensors_own_reset(g);
    crate::quiet::quiet_mission_current(g);
    g.stats.cleanrestart.restarts += 1;
    g.stats.cleanrestart.lines += g.messages.len() as i32;
    g.messages.clear();   // (the lines of the encounter that ended go before anything below speaks)
    crate::quiet::quiet_mission_before_restart(g);   // deals this encounter's start BEFORE the fresh Player and Guards read it
    let seats = g.seats.clone();
    for &s in &seats {
        let old = crate::cmd::seat_man(g, s);
        let light = g.pl[old].light;
        let c = g.pl[old].ch;
        crate::sim_men_store::man_leaves(g, c);   // (his row is free for the fresh man)
        let spawn = g.level.player_spawn;
        let index = g.tab.cmd_seat[s].index;
        let at = crate::seats::spot_beside(g, spawn, index);
        let fn2000 = g.fn2000;
        let (men, rig) = (g.men, g.rig);
        let pl = crate::game::player_new(g, men, rig, at, Some(fn2000), Some(index));
        crate::seats::with_seat(g, s, &mut |g| {
            g.player = pl;
            crate::seats::b_01::view_rekey(g, old, pl);   // (what the modules keep for this seat's view stays the seat's: seats.ts SEAT_TABLES)
            g.pl[pl].light = light;
            crate::kit::kit_fresh(g);
            let c = g.pl[pl].ch;
            crate::character::character_update(g, c, 0.0);   // pose and bones exist before this frame's capsule upload
        });
    }
    crate::cues::cues_reset(g);
    crate::squadnet::squad_net_reset(g);
    crate::seats::each_seat(g, &mut |v: &mut Game| { crate::holdfire::hold_fire_reset(v); crate::showwhy::reset_show_why(v); });
    crate::dialogue::ambient_restart(g);
    crate::fogofwar::fog_of_war_reset(g);
    crate::game::game_reset_guards(g);
    if let Some(last) = g.messages.last() { if last.text == "guards reset" { g.messages.pop(); } }
    if crate::lighting::fresh_lights(g) { g.stats.cleanrestart.relit += 1; }
    game_reset_level_state(g);
    crate::lightswitch::light_switch_reset(g);
    crate::props::prop_system_reset(g);
    for r in g.rolls.iter_mut() { r.voice = 0.0; }
    g.rolls.clear();
    g.events.clear();
    for &s in &seats { crate::seats::with_seat(g, s, &mut |g| g.use_door = None); }
    g.called_in = false;
    g.stood_down = false;
    // anything still in the air or venting belongs to the encounter that just ended
    game_clear_aftermath(g);
    crate::smoke_types::smoke_system_like_clear_all(g);
    crate::out_events::push_smoke_system_like_clear_all(g);   // (game.ts:1505: the host's solver hears it too)
    if g.cpu_smoke.grid.is_some() { crate::sense::smoke_grid_renew(g); }
    g.engine.host_books = None;
    for &s in &seats {
        crate::seats::with_seat(g, s, &mut |g| { g.hover = None; g.interactables = Vec::new(); g.nearby = Vec::new(); g.act = None; });
    }
    game_clean_slate(g);
    crate::devices::devices_reset(g);
    let t = g.time;
    g.mission = crate::game::new_mission(t);
    g.mission.serial = g.serial();   // (a whole write of g.mission is a new run: dialogue.ts:1717 and lesson.ts:320 ask "the same run?" by identity)
    g.chronicle = crate::chronicle::Chronicle::blank();
    crate::director::director_on_encounter_restart(g);
    crate::nightbreach::night_breach_on_encounter_restart(g);
    crate::rescue::rescue_on_encounter_restart(g);
    crate::quiet::quiet_mission_on_encounter_restart(g);
    crate::lighting::update_targets(g, 0.0);   // the lights painted as they now stand: what the next tick reads before its own paint
    crate::fire::update_fire(g, 0.0);
}

/// (game.ts setBlackout)
pub fn game_set_blackout(g: &mut crate::game::Game, seconds: f64, cause: crate::game::GameSetBlackoutCause) {
    let cause = match cause {
        crate::game::GameSetBlackoutCause::Shot => crate::lighting::SetBlackoutCause::Shot,
        crate::game::GameSetBlackoutCause::Ocp => crate::lighting::SetBlackoutCause::Ocp,
        crate::game::GameSetBlackoutCause::Debug => crate::lighting::SetBlackoutCause::Debug,
        crate::game::GameSetBlackoutCause::Hand => crate::lighting::SetBlackoutCause::Hand,
    };
    crate::lighting::set_blackout(g, seconds, cause)
}

/// (game.ts endBlackout)
pub fn game_end_blackout(g: &mut crate::game::Game) { crate::lighting::end_blackout(g) }

/// `cause` defaults to 'shot' here, as the TypeScript's parameter does.   (game.ts lightEvent)
pub fn game_light_event(g: &mut crate::game::Game, t: FixtureId, cause: Option<crate::game::GameLightEventCause>, sw_at: Option<V3>) {
    let cause = match cause.unwrap_or(crate::game::GameLightEventCause::Shot) {
        crate::game::GameLightEventCause::Shot => crate::lighting::LightEventCause::Shot,
        crate::game::GameLightEventCause::Ocp => crate::lighting::LightEventCause::Ocp,
    };
    crate::lighting::light_event(g, t, Some(cause), sw_at)
}

/// (game.ts ocp)
pub fn game_ocp(g: &mut crate::game::Game, t: Option<FixtureId>) { crate::lighting::ocp(g, t) }

/// (game.ts toggleDrag)
pub fn game_toggle_drag(g: &mut crate::game::Game, which: Option<GuardId>) { crate::player::toggle_drag(g, which) }

// ------------------------------------------------------------ update

/// One tick as a host calls it: its live input and view sampled into the tick's record, the sim run off that record alone.   (game.ts update)
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
    crate::director::director_inject_input(g, input);
    let into = g.cmd_buf.take();
    let c = crate::seats::with_seat(g, me, &mut |g| crate::cmd::sample_cmd(g, input, cam, canvas, into.as_ref()));
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
    game_host_after(g, input, cam, canvas);
}

/// What the tick hands back for the camera of the seat this host shows, and that seat's live view put back.   (game.ts hostAfter)
pub fn game_host_after(g: &mut crate::game::Game, input: InputId, cam: CameraId, canvas: crate::host_hud::View) {
    let me = game_host_seat(g);
    let me_cam = g.tab.cmd_seat[me].cam;
    let (peek, kick) = (g.tab.camera[me_cam].peek_offset, g.tab.camera[me_cam].kick);
    g.tab.camera[cam].peek_offset = peek;
    g.tab.camera[cam].kick = kick;
    if let Some(cut) = crate::cmd::seat_take_cut(g, me) { cut_lens(g, cam, cut); }
    let vp = g.tab.camera[cam].view_proj;
    crate::seats::with_seat(g, me, &mut |g| g.view_proj = Some(crate::game::GameViewProj::Float32Array(vp.to_vec())));
    crate::rails::rails_host_view(g, cam);
    crate::seats::with_seat(g, me, &mut |g| crate::orders::squad_orders_host_view(g, input, cam, canvas));
}

/// The tick for a game of one seat.   (game.ts step)
pub fn game_step(g: &mut crate::game::Game, dt: f64, cmd: &crate::cmd::PlayerCmd) {
    let buf = vec![cmd.clone()];
    game_step_seats(g, dt, &buf);
}

/// The local seat, or the first for a host that shows nobody.   (game.ts hostSeat)
pub fn game_host_seat(g: &crate::game::Game) -> SeatId { g.local_seat.unwrap_or(g.seats[0]) }

/// The world asked aside: `fn_` leaves nothing on the game's scratch.   (game.ts aside)
pub fn game_aside<T: Clone + core::fmt::Debug>(g: &mut crate::game::Game, fn_: &mut dyn FnMut(&mut crate::game::Game) -> T) -> T {
    // (a fresh table each ask: game::AsideOwn kept only the lens. An empty table is a clean one, collision.ts aside)
    let mut col_own = crate::scene_collision::AsideOwn::zeroed();
    crate::scene_collision::aside(g, &mut col_own, fn_)
}

/// A lens of the host's to put a record's view on while it asks aside (cmd.ts loadLens): scratch, in no snapshot.   (game.ts asideLens)
pub fn game_aside_lens(g: &mut crate::game::Game) -> CameraId {
    if let Some(lens) = g.alone_own.lens { return lens; }
    let lens = g.tab.camera.push(crate::camera::FollowCamera::blank());
    g.alone_own.lens = Some(lens);
    lens
}
