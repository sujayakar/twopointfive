//! src/game/game.ts lines 1544 to 1856
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

// ------------------------------------------------------------ seats and the one seam left

/// `s.view` handed to `f`. A game of one seat has no view: its seat's view IS the Game (seats.ts seatsStand).
fn in_seat<R>(g: &mut Game, s: SeatId, mut f: impl FnMut(&mut Game) -> R) -> R {
    if g.seats.len() == 1 { return f(g); }
    let pl = crate::cmd::seat_man(g, s);
    crate::seats::with_seat(g, pl, &mut f)
}
/// `manLeaves(pl.char, pl)` (game.ts:1624): ManCols is an empty struct and neither a Character nor a Player holds one, nothing to name the two objects by.
fn seam_man_leaves(g: &mut Game, c: CharId, pl: PlayerId) { todo!("needs: man_leaves over a CharId and a PlayerId") }

/// A cut: `lens.target = copy(at); lens.desired = copy(at)`. A whole write of `desired` is what Seat.takeCut watches for, here by its serial.   (game.ts:1614, :1665, :1683)
fn cut_lens(lens: &mut crate::camera::FollowCamera, at: V3) {
    lens.target = at;
    lens.desired = at;
    lens.desired_serial += 1;
}

/// the character of a seat's man
fn man_char(g: &mut Game, s: SeatId) -> CharId { let pl = crate::cmd::seat_man(g, s); g.pl[pl].ch }

// ------------------------------------------------------------ the panel's handles and small doors

/// The rooms queued for the pair, the panel's line.   (game.ts clearQueueSummary)
pub fn game_clear_queue_summary(g: &mut crate::game::Game) -> String { crate::guards::clear_queue_summary(g) }

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

/// What the player's feet are on as the hearing model has it.   (game.ts floorUnderPlayer)
pub fn game_floor_under_player(g: &mut crate::game::Game) -> crate::floor::FloorMat {
    let c = g.pl[g.player].ch;
    let p = g.ch[c].pos;
    crate::floor::step_mat(g, p)
}

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
    crate::tracers::tracers_clear(g);
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
    crate::hours::hour_after_reset(g);
    crate::air_dust::air_dust_reset(g);
}

/// (game.ts crackTest)
pub fn game_crack_test(g: &mut crate::game::Game) -> f64 { crate::glass::crack_test(g) }

/// (game.ts glassSummary)
pub fn game_glass_summary(g: &mut crate::game::Game) -> String { crate::glass::glass_summary(g) }

/// (game.ts wetSummary)
pub fn game_wet_summary(g: &mut crate::game::Game) -> String { crate::wet::wet_summary(g) }

/// (game.ts rainSummary)
pub fn game_rain_summary(g: &mut crate::game::Game) -> String { crate::rain_fx::rain_summary(g) }

/// The clocks the last try left running, as a new Game has them.   (game.ts cleanSlate)
pub fn game_clean_slate(g: &mut crate::game::Game) {
    g.alarm.raised_at = -100.0;
    g.alarm.pos = [0.0, 0.0, 0.0];
    let seats = g.seats.clone();
    for &s in &seats { in_seat(g, s, |g| g.torch_on_t = -100.0); }
    g.last_sting_t = -100.0;
    crate::gunwall::gun_wall_fresh(g);
    for &s in &seats { in_seat(g, s, |g| crate::guards::ground_fresh(g)); }
}

/// The player's restart: restartEncounter, the camera cut to the fresh man, the host told to start the picture's histories from nothing.   (game.ts restart)
pub fn game_restart(g: &mut crate::game::Game, said: Option<&str>) {
    game_restart_encounter(g);
    if let Some(s) = said { if !s.is_empty() { crate::game::game_msg(g, s); } }   // (after it: restartEncounter takes the old encounter's lines off the HUD)
    if !g.touring && !crate::director::director_showing(g) && g.spectate < 0.0 {
        let seats = g.seats.clone();
        for s in seats {
            let p = in_seat(g, s, |g| game_follow_pos(g, None));
            cut_lens(&mut g.tab.cmd_seat[s].cam, p);
        }
        g.restart_cut = true;
        g.stats.cleanrestart.cuts += 1;
    }
    crate::host_api::host_api_restart_picture(g, "level restarted");
    g.stats.cleanrestart.pictures += 1;
}

/// Restart the encounter: a fresh Player at the insertion point with full kit, fresh Guards on their routes, lights repaired, the level as authored.   (game.ts restartEncounter)
pub fn game_restart_encounter(g: &mut crate::game::Game) {
    let d = g.dice.clone();
    crate::math_dice::use_dice(g, d);
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
        seam_man_leaves(g, c, old);
        let spawn = g.level.player_spawn;
        let index = g.tab.cmd_seat[s].index;
        let at = crate::seats::spot_beside(g, spawn, index);
        let fn2000 = g.fn2000;
        let (men, rig) = (g.men, g.rig);
        let pl = crate::game::player_new(g, men, rig, at, Some(fn2000), Some(index));
        in_seat(g, s, |g| {
            g.player = pl;
            g.pl[pl].light = light;
            crate::kit::kit_fresh(g);
            let c = g.pl[pl].ch;
            crate::character::character_update(g, c, 0.0);   // pose and bones exist before this frame's capsule upload
        });
    }
    crate::cues::cues_reset(g);
    for &s in &seats {
        in_seat(g, s, |g| { crate::holdfire::hold_fire_reset(g); crate::showwhy::reset_show_why(g); });
    }
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
    for &s in &seats { in_seat(g, s, |g| g.use_door = None); }
    g.called_in = false;
    g.stood_down = false;
    // anything still in the air or venting belongs to the encounter that just ended
    game_clear_aftermath(g);
    crate::smoke_types::smoke_system_like_clear_all(g);
    if g.cpu_smoke.grid.is_some() { crate::sense::smoke_grid_renew(g); }
    g.engine.host_books = None;
    for &s in &seats {
        in_seat(g, s, |g| { g.hover = None; g.interactables = Vec::new(); g.nearby = Vec::new(); g.act = None; });
    }
    game_clean_slate(g);
    crate::devices::devices_reset(g);
    let t = g.time;
    g.mission = crate::game::new_mission(t);
    g.chronicle = crate::chronicle::Chronicle::blank();
    crate::director::director_on_encounter_restart(g);
    crate::nightbreach::night_breach_on_encounter_restart(g);
    crate::rescue::rescue_on_encounter_restart(g);
    crate::quiet::quiet_mission_on_encounter_restart(g);
    crate::lighting::update_targets(g, 0.0);   // the lights painted as they now stand: what the next tick reads before its own paint
    crate::fire::update_fire(g, 0.0);
}

/// (game.ts setGroup)
pub fn game_set_group(g: &mut crate::game::Game, group: &str, on: bool) { crate::lighting::set_group(g, group, on) }

/// (game.ts toggleTarget)
pub fn game_toggle_target(g: &mut crate::game::Game, t: FixtureId) { crate::lighting::toggle_target(g, t) }

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
pub fn game_update(g: &mut crate::game::Game, dt: f64, input: InputId, canvas: crate::host_hud::View) {
    let d = g.dice.clone();
    crate::math_dice::use_dice(g, d);
    crate::quiet::quiet_mission_current(g);
    let me = game_host_seat(g);
    if g.restart_cut {
        g.restart_cut = false;
        if let Some(cut) = crate::cmd::seat_take_cut(g, me) {
            let free = match g.director.cam.as_mut() { Some(cam) if cam.pin.is_none() => { cut_lens(cam, cut); true } _ => false };
            if free { crate::camera::follow_camera_rebuild(g); }
        }
    }
    crate::director::director_inject_input(g, input);
    game_host_before(g, input);
    // (the host's camera stands at g.director.cam already: only the input is handed over)
    crate::director::director_attach(g, &crate::director::DirectorAttachO { cam: None, input: Some(input) });
    let into = g.cmd_buf.take();
    let c = in_seat(g, me, |g| crate::cmd::sample_cmd(g, input, canvas, into.as_ref()));
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
    game_host_after(g, input, canvas);
}

/// What the host's input is told before a step: a held press's cursor is followed off the view only while the creep reads the mouse as a stick.   (game.ts hostBefore)
pub fn game_host_before(g: &mut crate::game::Game, input: InputId) {
    g.tab.input[input].drag_off = g.k.stalk.drag;
}

/// What the tick hands back for the camera of the seat this host shows, and that seat's live view put back.   (game.ts hostAfter)
pub fn game_host_after(g: &mut crate::game::Game, input: InputId, canvas: crate::host_hud::View) {
    let me = game_host_seat(g);
    let (peek, kick) = (g.tab.cmd_seat[me].cam.peek_offset, g.tab.cmd_seat[me].cam.kick);
    if let Some(cam) = g.director.cam.as_mut() { cam.peek_offset = peek; cam.kick = kick; }
    if let Some(cut) = crate::cmd::seat_take_cut(g, me) {
        if let Some(cam) = g.director.cam.as_mut() { cut_lens(cam, cut); }
    }
    if let Some(vp) = g.director.cam.as_ref().map(|cam| cam.view_proj) {
        in_seat(g, me, |g| g.view_proj = Some(crate::game::GameViewProj::Float32Array(vp.to_vec())));
    }
    crate::rails::rails_host_view(g);
    in_seat(g, me, |g| crate::orders::squad_orders_host_view(g, input, canvas));
}

/// The tick for a game of one seat.   (game.ts step)
pub fn game_step(g: &mut crate::game::Game, dt: f64, cmd: &crate::cmd::PlayerCmd) {
    let buf = vec![cmd.clone()];
    game_step_seats(g, dt, &buf);
}

/// The local seat, or the first for a host that shows nobody.   (game.ts hostSeat)
pub fn game_host_seat(g: &mut crate::game::Game) -> SeatId { g.local_seat.unwrap_or(g.seats[0]) }

/// The world asked aside: `fn_` leaves nothing on the game's scratch.   (game.ts aside)
pub fn game_aside<T: Clone + core::fmt::Debug>(g: &mut crate::game::Game, fn_: &mut dyn FnMut(&mut crate::game::Game) -> T) -> T {
    // (a fresh table each ask: game::AsideOwn kept only the lens. An empty table is a clean one, boxes.ts rayAside, collision.ts aside)
    let mut ray_own = crate::scene_boxes::BoxWorldRayAsideOwn::zeroed();
    let mut col_own = crate::scene_collision::AsideOwn::zeroed();
    crate::scene_boxes::box_world_ray_aside(g, &mut ray_own, &mut |g| crate::scene_collision::aside(g, &mut col_own, &mut *fn_))
}

/// A lens of the host's to put a record's view on while it asks aside; it stands at `g.alone_own.lens`.   (game.ts asideLens)
pub fn game_aside_lens(g: &mut crate::game::Game) {
    if g.alone_own.lens.is_none() { g.alone_own.lens = Some(crate::camera::FollowCamera::blank()); }
}

/// THE TICK, off the players' plain records. The order of these lines is the game.   (game.ts stepSeats)
pub fn game_step_seats(g: &mut crate::game::Game, dt: f64, cmds: &[crate::cmd::PlayerCmd]) {
    let n = g.seats.len();
    if cmds.len() != n { panic!("stepSeats: {} record(s) for {} seat(s)", cmds.len(), n); }
    // (inside a tick the Game's own fields of a seat's name answer the FIRST seat's, whoever this host shows: seats.ts seatsStand)
    let first = g.seats[0];
    in_seat(g, first, |g| step_seats_on_first(g, dt, cmds));
}

/// The tick's lines, with the first seat on Game.   (game.ts stepSeats)
fn step_seats_on_first(g: &mut crate::game::Game, dt: f64, cmds: &[crate::cmd::PlayerCmd]) {
    let seats: Vec<SeatId> = g.seats.clone();
    let n = seats.len();
    let me = game_host_seat(g);
    g.ticking = true;
    crate::scene_collision::tick_open(g);
    // a record from anyone made one an honest seat could have sent, before anything reads it
    let mut checked: Vec<crate::cmd::PlayerCmd> = Vec::with_capacity(n);
    for i in 0..n {
        let s = seats[i];
        let near = if g.tab.cmd_seat[s].remote { let c = man_char(g, s); Some(g.ch[c].pos) } else { None };
        let r = crate::cmd::check_cmd(g, Some(&cmds[i]), near);
        if js::truthy(r.fixed) { g.tab.cmd_seat[s].fixed += r.fixed; g.tab.cmd_seat[s].fixed_cmds += 1.0; }
        checked.push(r.cmd);
    }
    let d = g.dice.clone();
    crate::math_dice::use_dice(g, d);
    crate::quiet::quiet_mission_current(g);
    crate::game::game_sense_step(g, dt);
    for i in 0..n { crate::cmd::seat_load(g, seats[i], &checked[i]); }
    let input = g.tab.cmd_seat[seats[0]].input;   // (what a ONE-PLAYER part of the tick reads is the first seat's)
    crate::rails::rails_gate(g, input, dt);
    crate::opening::opening_start(g);
    if n > 1 { crate::seats::deal_seats(g); }
    for &s in &seats {
        let vp = g.tab.cmd_seat[s].cam.view_proj;
        in_seat(g, s, |g| g.view_proj = Some(crate::game::GameViewProj::Float32Array(vp.to_vec())));
    }
    g.time += dt;
    g.ocp_cooldown = m::max(0.0, g.ocp_cooldown - dt);
    let now = g.time;
    crate::scene_collision::sound_paths_tick(g, now);
    // each seat's half of the tick, over that seat's view
    for &s in &seats {
        let (input, canvas) = (g.tab.cmd_seat[s].input, g.tab.cmd_seat[s].canvas);
        in_seat(g, s, |g| {
            crate::player::update_aim(g, input, canvas);
            crate::orders::squad_orders_input(g, dt, input, canvas);
            if crate::doortools::door_tools_menu_key(g, input) { let pl = g.player; g.pl[pl].throw_eaten = true; }
            crate::squadsmoke::squad_smoke_key(g, input);
            crate::wandorder::wand_order_key(g, input);
            crate::doortools::door_tools_big_key(g, input);
            if crate::squadthrow::squad_throw_key(g, input) { let pl = g.player; g.pl[pl].throw_eaten = true; }
            crate::squad40::squad40_tick(g);
            crate::interact::update_interact(g, dt, input, Some(canvas));
            crate::aim::track_player(g, dt);
            crate::game::game_threat_tick(g);
        });
    }
    crate::suppression::update_suppression(g, dt);
    for &s in &seats {
        let input = g.tab.cmd_seat[s].input;
        in_seat(g, s, |g| { crate::player::update_player(g, dt, input); crate::samlook::sam_look_step(g); });
    }
    if !g.rings.is_empty() { crate::launcher::update_rings(g, dt); }
    for &s in &seats {
        let pl = crate::cmd::seat_man(g, s);
        if !g.pl[pl].cams.is_empty() { in_seat(g, s, |g| crate::stickycam::update_stickies(g, dt)); }
    }
    if !g.shells.is_empty() { crate::he40::update_shells(g, dt); }
    g.search_probe_n = 0.0;
    crate::game::game_deal_weapons(g, Some(dt));
    crate::director::director_tick(g, dt, None);
    crate::pairorders::pair_orders_input(g, input);
    for &s in &seats {
        let input = g.tab.cmd_seat[s].input;
        in_seat(g, s, |g| {
            crate::wallcharge::wall_charge_input(g, input);
            crate::doortools::door_tools_input(g, input);
            crate::wallcharge::wall_charge_input_after(g, input);
        });
    }
    crate::nightbreach::night_breach_tick(g, dt, input);
    crate::quiet::quiet_mission_tick(g);
    crate::roomclear::room_clear_scenario_tick(g, dt);
    crate::rescue::rescue_tick(g, dt, input);
    crate::wallreact::wall_react_tick_2(g);
    crate::entrypoints::entry_points_tick(g);
    crate::cleared::cleared_rooms_tick(g);
    for &s in &seats { in_seat(g, s, |g| { crate::doortools::door_tools_tick(g); crate::wallcharge::wall_charge_tick(g); }); }
    for &s in &seats { in_seat(g, s, |g| crate::rearwatch::rear_watch_tick(g)); }
    crate::cover::cover_tick(g);
    for &s in &seats { in_seat(g, s, |g| crate::silent::silent_tick(g)); }
    crate::pairorders::pair_orders_tick(g, dt);
    crate::spotted::spotted_tick(g);
    for &s in &seats { in_seat(g, s, |g| crate::orders::squad_orders_tick(g)); }
    crate::doorpath::door_path_tick(g, dt);
    crate::powerout::power_out_tick(g, dt);
    crate::rescuecheck::rescue_check_tick(g, dt);
    crate::roomfight::room_fight_tick(g, dt);
    for &s in &seats { in_seat(g, s, |g| crate::showwhy::show_why_tick(g)); }
    crate::morale::morale_tick(g, dt);
    crate::flinch::flinch_tick(g, dt);
    crate::runner::runner_tick(g, dt);
    crate::darkdoor::dark_door_tick(g, dt);
    crate::netquiet::net_quiet_tick(g, dt);
    crate::lanes::lanes_tick(g, dt);
    crate::darkfoe::dark_foe_tick(g, dt);
    crate::nvg::nvg_tick(g);
    crate::tactics::tactics_tick(g, dt);
    crate::doze::doze_tick(g, dt);
    crate::close::close_seat_tick(g, dt);
    crate::needs::needs_tick(g, dt);
    crate::frag::frag_throw_tick(g, dt);
    crate::frag::frag_tick(g, dt);
    // (a live index: `for … of` over the list, which a man's update may grow)
    let mut i = 0;
    while i < g.guards.len() {
        let gd = g.guards[i];
        if n == 1 { crate::guards::update_guard(g, gd, dt); } else { let v = crate::seats::view_for(g, gd); crate::seats::with_seat(g, v, &mut |g| crate::guards::update_guard(g, gd, dt)); }
        i += 1;
    }
    crate::picture::picture_tick(g, dt);
    crate::doorpath::door_path_after(g);
    crate::doorforce::door_force_tick(g, dt);
    crate::coverfire::coverfire_tick(g, dt);
    crate::civilians::update_civilians(g, dt);
    crate::guards::update_escalation(g, dt);
    crate::qrf::update_qrf(g, dt);
    crate::game::game_update_doors(g, dt);
    crate::lighting::lens_recheck(g);
    crate::irlight::ir_light_tick(g);
    in_seat(g, seats[0], |g| crate::mission::update_mission(g, dt, input));
    crate::lesson::lesson_tick(g, dt);
    crate::rails::rails_tick(g, dt);
    crate::snatch::snatch_tick(g, dt);
    crate::defuse::defuse_tick(g, dt);
    crate::game::game_update_props(g, dt);
    crate::destruction::destruction_update(g, dt);
    crate::air_dust::air_dust_update(g, dt);
    let now = g.time;
    g.events.retain(|e| now - e.time < 0.5);
    crate::lightswitch::light_switch_tick(g);
    crate::lighting::update_targets(g, dt);
    crate::fire::update_fire(g, dt);
    crate::devices::update_devices(g, dt);
    let yaw = g.tab.cmd_seat[me].cam.yaw;
    crate::weather::weather_update(g, dt, Some(yaw));
    crate::takedown::takedown_tick(g);
    for &s in &seats { in_seat(g, s, |g| crate::disarm::disarm_tick(g, dt)); }
    crate::combat::update_items(g, dt);
    crate::glass::update_glass(g);
    crate::glassbreak::glass_break_update(g, dt);
    crate::shards::glass_shards_update(g, dt);
    crate::fogofwar::fog_of_war_update(g, dt);
    // characters: the seats' men, the guards, civilians last
    let mut all: Vec<CharId> = Vec::new();
    for &s in &seats { let c = man_char(g, s); all.push(c); }
    for j in 0..g.guards.len() { let gd = g.guards[j]; all.push(g.gd[gd].ch); }
    for gd in crate::civilians::civilians_shown(g) { all.push(g.gd[gd].ch); }
    let canvas = g.tab.cmd_seat[me].canvas;
    let statured = crate::game::game_step_fx_and_dyn_boxes(g, &all, dt, canvas);
    crate::game::game_draw_characters(g, &all, statured);
    crate::smokebreak::update_smoke_break(g, dt);
    crate::game::game_light_meter(g, dt);
    for &s in &seats {
        let pl = crate::cmd::seat_man(g, s);
        g.pl[pl].hit_flash = m::max(0.0, g.pl[pl].hit_flash - dt * 3.0);
        in_seat(g, s, |g| g.act = None);   // (this frame's Space verb has been read by everyone who reads it)
    }
    crate::teamtally::team_tally_tick(g);
    g.ticking = false;
    crate::scene_collision::tick_shut(g);
}
