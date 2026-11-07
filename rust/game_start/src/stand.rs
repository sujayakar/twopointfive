//! THE START: a level file and the character's files in, a `Game` the tick can run out. `src/host/standup.ts standGame` on a host with no picture
//! (`src/host/stub.ts stubHost`, `tools/qa/headless.ts standUp`), in its order:
//!
//!   the block of rust/sim the game stands in, zeroed under the layout's word   (sim/state.ts SimState.born)
//!   the character's rows                                                       (anim/gltf.ts; `gltf.rs`)
//!   the host: an empty box world and light set, the picture's settings         (host/stub.ts)
//!   the level file read                                                        (scene/levelLoad.ts; `level.rs`)
//!   the level's marks, its statics into the world, its lights into the set     (standup.ts populateHost; `passes.rs`)
//!   the die, seeded: the men's construction dice are its first draws           (standup.ts: newDice, useDice)
//!   the Game's constructor                                                     (game.ts; rust/game `game_new`)
//!   the state that still moves, at home in the block                           (sim/state.ts settle)
//!   the host's own Input and camera, the camera on the level's spawn           (headless.ts standUp)
//!
//! One function, `stand`, for every way a game starts natively: a test's scene (`scene.rs` writes a level file), a level of `levels/`, later a host.
use crate::gltf::{self, GltfFile};
use crate::json::Json;
use crate::{level, passes, settings};
use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use game::game::Game;
use game::ids::*;

/// the character's files (the mannequin, and the cast's meshes over it where given: the page boots with the cast)
#[derive(Clone, Copy)]
pub struct Assets<'a> {
    pub mannequin: GltfFile<'a>,
    /// the cast's meshes (`cast.gltf`); none = everybody is the mannequin
    pub cast: Option<GltfFile<'a>>,
    /// the cast in the look's dress (`cast-gwot.gltf`), which a game under the look stands on where it is given (gltf.ts castFile)
    pub cast_look: Option<GltfFile<'a>>,
}

/// What a start is told beside the level.
#[derive(Clone, Debug)]
pub struct StartOpts {
    /// the seed of the game's own die
    pub seed: u32,
    /// how many players (1 or 2), for the life of the game
    pub seats: u32,
    /// which of rust/sim's blocks the game stands in (one game a block at a time)
    pub slot: usize,
    /// the mission to start on the stood level by the game's own `startScenario`, none = the level as its file has it
    pub scenario: Option<game::scene_scenario_list::ScenarioSpec>,
    /// the rows that only dress the picture up (a page) or down (a host that draws nothing: the moths do not fly, the fixtures do not swing)
    pub dress: bool,
    /// THE SENSES WORKED OUT BY THE GAME ITSELF (game/sense.ts SENSE.cpu, and the sky's light in its estimate, SENSE.moon): what a host with no
    /// picture has for eyes. Off = the page's own stand, where the renderer's light meter answers; on a host with no renderer nothing does, and
    /// every man is in the dark for good (the harness's TPS_CPUSENSE=1 TPS_MOONLIGHT=1)
    pub cpu_senses: bool,
}
impl Default for StartOpts { fn default() -> Self { StartOpts { seed: 20942, seats: 1, slot: 0, scenario: None, dress: false, cpu_senses: false } } }

/// THE HOST'S OWN HALF of a stood game: the Input a hand writes, the camera that follows, the canvas's size (headless.ts standUp: `input`, `cam`, `canvas`).
#[derive(Clone, Copy, Debug)]
pub struct Host { pub input: InputId, pub cam: CameraId, pub canvas: game::host_hud::View }

pub struct Stood { pub game: Box<Game>, pub host: Host, pub level: level::Loaded, pub rig: RigId }

/// a block for a game that had none: zeros under the layout's word (state.ts born; a slot another game stood in is that game's no more)
fn born(slot: usize) {
    let len = core::mem::size_of::<sim::state::State>();
    unsafe { core::ptr::write_bytes(sim::state::addr(slot) as *mut u8, 0, len); }
    sim::state::state(slot).head.layout = sim::state::LAYOUT;
}

/// the level's statics into the host's box world and its lights into the light set (standup.ts populateHost)
fn populate_host(g: &mut Game, boxes: &[BoxId]) {
    let doors = g.level.doors.clone();
    passes::thin_level(g, boxes, &doors);
    passes::bar_level(g, boxes);
    for &b in boxes { game::scene_boxes::box_world_add_static(g, b); }
    for ld in g.level.lights.clone().iter() { let l = game::render_lights::light_from_def(g, ld); game::render_lights::light_set_add(g, l); }
}

/// the state that still moves, at home in the game's block (state.ts settle, as `Game.stepSeats` says it when a tick opens: the die, the pose pool, the smoke grid)
pub fn settle(g: &mut Game) {
    game::math_dice::use_dice(g);
    game::posehome::pose_home_tend(g);
    game::sense::settle_grid(g);
    // what the TypeScript's stores say of themselves in the block's head as they are made or moved in (state.ts keep, settle), and the Rust stores,
    // which hold their part of the block from the start, do not: the pose pool (2), the doors (4), the furniture (32), the thrown things (128) and the
    // men's columns (256) are at home. No rule reads these bits; the block is then the block the TypeScript start leaves, to the byte.
    sim::state::state(g.sim).head.homes |= 2 | 4 | 32 | 128 | 256;
}

/// THE HOST'S OWN INPUT AND CAMERA for a game that stands (headless.ts standUp): a detached Input with the cursor where the harness always put it,
/// a canvas of 1600 by 1000, a follow camera on the level's spawn, and the box world listed once.
pub fn host_new(g: &mut Game) -> Host {
    let input = game::input::input_detached(g);
    g.tab.input[input].mouse_x = 800.0;
    g.tab.input[input].mouse_y = 500.0;
    let canvas = game::host_hud::View { client_width: 1600.0, client_height: 1000.0, width: 1600.0, height: 1000.0 };
    let cam = g.tab.camera.push(game::camera::FollowCamera::blank());
    g.tab.camera[cam].target = g.level.player_spawn;
    g.tab.camera[cam].aspect = canvas.width / canvas.height;
    game::camera::follow_camera_rebuild(g, cam);
    game::host_api::b_01::host_world_upload(g);
    Host { input, cam, canvas }
}

/// STAND A GAME on the level file `data` (the file's text, parsed).
pub fn stand(data: &Json, assets: Assets, o: &StartOpts) -> Stood {
    let mut g = Box::new(Game::blank());
    born(o.slot);
    g.sim = o.slot;
    let cast = if game::scene_look::look_dress(&mut g) { assets.cast_look.or(assets.cast) } else { assets.cast };
    let rig = gltf::character(&mut g, assets.mannequin, cast);
    // the rows that only dress the picture, as this host has them (game/dress.ts DRESS: up on a page, down on a host that draws nothing)
    if !o.dress { g.m.dress.dress = game::dress::DressModDRESS::zeroed(); }
    // the host with no picture: an empty world and light set, the picture's settings, effectiveScale 1, no skin (stub.ts stubHost)
    g.engine.world = game::scene_boxes::box_world_new(&mut g, game::Opaque, None);
    g.engine.lights = game::render_lights::LightSet::blank();
    g.engine.settings = settings::default_settings();
    g.engine.effective_scale = 1.0;
    if o.cpu_senses { g.k.sense.cpu = true; g.k.sense.moon = true; }
    let (dressing, lived) = (g.k.destruction.on, g.k.lived.on);
    let loaded = level::load(&mut g, data, dressing, lived);
    populate_host(&mut g, &loaded.boxes);
    // the die BEFORE the Game is made: its constructor rolls the men's construction dice
    // (it goes home to the block with the rest of the moving state, in `settle`: until then it draws off its own word)
    g.dice = Some(game::math_dice::sim_dice_new(o.seed as f64));
    g.col.boxes = loaded.boxes.clone();   // (`level.boxes`, which the constructor reads as the collision's list)
    game::game::game_new(&mut g, rig, Some(game::game::GameNewOpts { seats: Some(o.seats as f64) }));
    settle(&mut g);
    // the harness's stand (standGame with `harness`): the level's script strict, the solo night's card on a game stood plain
    // (the flag is the page's boot: host/profile.ts shippedBoot raises it, and the page's own plain night asks `kitSoloIf` by its address)
    g.director.strict = true;
    g.m.kit.kit_solo.stand = true;
    game::kit::kit_solo_stand(&mut g, None);
    let host = host_new(&mut g);
    if let Some(spec) = o.scenario.as_ref() {
        if let Some(why) = game::scenario_run::start_scenario(&mut g, spec) { panic!("the game refused the {}: {}", spec.kind.as_str(), why); }
        settle(&mut g);   // (the men the mission made: their poses go home as a tick's opening would take them)
    }
    Stood { game: g, host, level: loaded, rig }
}

/// ONE FRAME OF A HOST WITH NO PICTURE (headless.ts `step`): the game's update on the host's Input and camera, the camera after the man, the lights'
/// envelopes, the box world listed again, the Input's frame ended. Answers WHAT THE STEP TOLD ITS HOST (tools/blitz/OUT-EVENTS.md: the sounds to
/// play, the marks to draw, the lines said): the list is a step's output and the host drains it, or it grows for the life of the game.
pub fn frame(g: &mut Game, h: &Host, dt: f64) -> Vec<game::out_events::OutEvent> {
    game::game::game_update(g, dt, h.input, h.cam, h.canvas);
    let me = game::game::game_host_seat(g);
    let (follow, aim) = game::seats::with_seat(g, me, &mut |g| (game::game::game_follow_pos(g, None), g.aim_point));
    let aim = if g.spectate >= 0.0 { None } else { Some(aim) };
    game::camera::follow_camera_update(g, h.cam, dt, follow, aim);
    let now = g.time;
    game::render_lights::light_set_update(g, dt, now);
    game::host_api::b_01::host_world_upload(g);
    game::input::input_end_frame(g, h.input);
    core::mem::take(&mut g.out)
}

#[allow(dead_code)]
fn unused() -> String { String::new() }
