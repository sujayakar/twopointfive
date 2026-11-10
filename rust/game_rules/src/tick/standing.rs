//! THE MEN: a stand's lines from the level as it is built to the game the first step runs on, the mission up. A stand runs them once on the level it has just built and a restart again on the
//! level's image, so a mission starts over by the very lines it started by.
use crate::game::Game;
use crate::ids::*;

/// What the lines are told: all of it is known before the first man is made, and none of it changes for the life of a stand
#[derive(Clone)]
pub struct Men {
    pub rig: RigId,
    /// how many players, for the life of the game
    pub seats: f64,
    /// the words the mission is started by, as a click on its row has them; none = a game stood with no address
    pub words: Option<crate::query::Query>,
    /// the mission to start where there are no words
    pub scenario: Option<crate::scene_scenario_list::ScenarioSpec>,
}

/// THE HOST'S OWN HALF of a stood game: the Input a hand writes, the camera that follows, the canvas's size.
#[derive(Clone, Copy, Debug)]
pub struct Host { pub input: InputId, pub cam: CameraId, pub canvas: crate::host_hud::View }

/// the state that still moves, at home in the game's block (state.ts settle, as `Game.stepSeats` says it when a tick opens: the die, the pose pool, the smoke grid)
pub fn settle(g: &mut Game) {
    crate::math_dice::use_dice(g);
    crate::posehome::pose_home_tend(g);
    // what the TypeScript's stores say of themselves in the block's head as they are made or moved in, and the Rust stores,
    // which hold their part of the block from the start, do not: the pose pool (2), the doors (4), the furniture (32) and the men's
    // columns (256) are at home. No rule reads these bits; the block is then the block the TypeScript start leaves, to the byte.
    ::sim::state::state(g.sim).head.homes |= 2 | 4 | 32 | 256;
}

/// the canvas of a host that draws nothing
pub fn canvas() -> crate::host_hud::View { crate::host_hud::View { client_width: 1600.0, client_height: 1000.0, width: 1600.0, height: 1000.0 } }

/// THE HOST'S OWN INPUT AND CAMERA for a game that stands: a detached Input with the cursor where the harness always put it,
/// a canvas of 1600 by 1000, a follow camera on the level's spawn, and the box world listed once.
pub fn host_new(g: &mut Game) -> Host {
    let input = crate::input::input_detached(g);
    g.tab.input[input].mouse_x = 800.0;
    g.tab.input[input].mouse_y = 500.0;
    let canvas = canvas();
    let cam = g.tab.camera.push(crate::camera::FollowCamera::blank());
    g.tab.camera[cam].target = g.level.player_spawn;
    g.tab.camera[cam].aspect = canvas.width / canvas.height;
    crate::camera::follow_camera_rebuild(g, cam);
    crate::scene_boxes::box_world_listed(g);
    // (the host's camera and keys are handed to the game here, once, as standGame hands a TypeScript host's: game.ts hostCam)
    g.host_cam = Some(cam);
    g.host_input = Some(input);
    Host { input, cam, canvas }
}

/// THE MEN ON THE LEVEL (the die is in the game already): the constructor, the hour, the host's rows, the mission. The reason where the mission refused to start
pub fn stand_men(g: &mut Game, men: &Men) -> Option<alloc::string::String> {
    let spec = men.words.as_ref().map_or_else(|| men.scenario.clone(), crate::scene_scenario_list::spec_of);
    // (a quiet mission's deal says where the men stand and what they make for: they read it from the level as they are made)
    if let Some(s) = spec.filter(|s| s.kind == crate::scene_scenarios::MissionKind::Quiet) { crate::quiet::quiet_mission_deal(g, s.seed, men.seats); }
    crate::game::game_new(g, men.rig, Some(crate::game::GameNewOpts { seats: Some(men.seats) }));
    settle(g);
    // A GAME STOOD WITH NO ADDRESS is the harness's plain stand (standGame with `harness`): the solo night's card goes on at the stand, by the
    // harness's own flag. A game stood FROM AN ADDRESS has no such flag, as a page has none: nothing is
    // put on at the stand, and the address's own question is asked once its mission is staged (`start_at_address`)
    if men.words.is_none() {
        g.stood.solo_stand = true;
        crate::kit::kit_solo_stand(g, None);
    }
    // the visit's hour laid over the level's own night (standGame's line: the hour is the address's, read before the game was made)
    { let hour = g.stood.hour; if hour != crate::hours::HourId::Night { crate::hours::apply_hour(g, hour); } }
    host_new(g);
    let why = if let Some(q) = men.words.as_ref() {
        crate::scenario_run::start_at_address(g, q).why
    } else if let Some(spec) = men.scenario.as_ref() {
        crate::scenario_run::start_scenario(g, spec)
    } else { return None };
    settle(g);   // (the men the mission made: their poses go home as a tick's opening would take them)
    crate::lighting::update_targets(g, 0.0);   // the lights painted as they now stand: what the first tick reads before its own paint
    crate::fire::update_fire(g, 0.0);
    why
}
