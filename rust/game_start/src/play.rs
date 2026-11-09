//! A STOOD GAME IN A TEST'S HANDS: stand a scene or a level, run it for a while, ask what a man did. Everything here goes through the start
//! (`stand.rs`) and the host's own frame; nothing writes the game's state behind a rule's back. What a test asks of a man it asks through the
//! readers at the foot of this file, so that when a field moves (the AI's rework will move several) one line here follows it and no test does.
//!
//! ONE GAME AT A TIME IN A PROCESS. rust/sim keeps its blocks and several tables in statics, so two games on two threads would write each
//! other's state: `stand` takes a lock that the `Play` holds until it is dropped, and `cargo test`'s threads take turns.
use crate::json::Json;
use crate::scene::Scene;
use crate::stand::{self, Host, StartOpts, Stood};
use crate::{files, json};
use game::game::{Game, GuardState};
use game::ids::*;
use std::string::String;
use std::sync::{Mutex, MutexGuard, OnceLock};
use std::vec::Vec;

static ONE_GAME: Mutex<()> = Mutex::new(());
std::thread_local! { static HOLDS: std::cell::Cell<bool> = const { std::cell::Cell::new(false) }; }
static CHARACTER: OnceLock<files::CharacterFiles> = OnceLock::new();

/// the level file of every room clear of the game's mission list, each once, by name: a new row is here with nothing to write
pub fn room_clear_files() -> Vec<String> {
    let mut files: Vec<String> = game::scene_scenario_list::scenarios().into_iter().filter(|e| e.kind == game::scene_scenarios::MissionKind::Roomclear).map(|e| e.level.unwrap_or_else(|| String::from("office.json"))).collect();
    files.sort();
    files.dedup();
    files
}

/// the host's frame rate: a tick is a sixtieth of a second
pub const DT: f64 = 1.0 / 60.0;

/// What a test says of a stand beside the scene.
#[derive(Clone, Debug)]
pub struct Opts {
    /// the seed of the game's die: a test that holds over several seeds holds a rule, one seed holds a run
    pub seed: u32,
    /// how many teammates a room clear stages beside Sam (2 or 4)
    pub squad: u32,
    /// how many players (1 or 2): each has a seat, his own man and his share of the squad. The hand's keys and cursor are the first seat's.
    pub seats: u32,
    /// THE ROWS THAT ONLY DRESS THE PICTURE, as a page has them: the debris pool tumbling a break's pieces, above all. Off (the default) as
    /// on any host that draws nothing: a thrown piece is then only its one landing told to the floor, and the pool stays empty.
    pub dress: bool,
    /// a mission the game refuses is said (`Play.refused`) and no stop
    pub may_refuse: bool,
}
impl Default for Opts { fn default() -> Self { Opts { seed: 1, squad: 4, seats: 1, dress: false, may_refuse: false } } }

/// What stands on a level's floor: its boxes (the round things, the boxes that stand shaped and the pieces of trim among them), and who and what
/// the game made of the file's civilians, devices and vents.
#[derive(Debug, PartialEq)]
pub struct Floor { pub boxes: usize, pub round: usize, pub shaped: usize, pub trim: usize, pub civilians: usize, pub cameras: usize, pub lasers: usize, pub keypads: usize, pub vents: usize }

/// a mouse button, by its bit in the Input
#[derive(Clone, Copy, Debug)]
pub enum Button { Left = 1, Middle = 2, Right = 4 }

/// a line a man said
#[derive(Clone, Debug)]
pub struct Said { pub t: f64, pub who: GuardId, pub text: String, pub radio: bool }
/// a sound the game asked for
#[derive(Clone, Debug)]
pub struct Sound { pub t: f64, pub name: String, pub at: Option<[f64; 3]> }

/// A game a test is playing. It holds the one-game lock until dropped.
pub struct Play {
    pub g: std::boxed::Box<Game>,
    pub host: Host,
    /// frames run so far
    pub frames: usize,
    /// what each step's end told this host (game::savestate::control), until read
    pub told: Vec<game::savestate::control::Told>,
    /// EVERY LINE A MAN HAS SPOKEN, in order. Read off each man's speech bubble after every frame (`guards::say` writes one for every line it
    /// lets through, whoever asked for it, and none for a line it drops), so a man who speaks twice inside one frame is heard once.
    pub said: Vec<Said>,
    /// every sound the game told its host to play (`audio.play`: its name, when, and where it was, None for a sound with no place). Footsteps
    /// are another call of the host's and are not kept.
    pub sounds: Vec<Sound>,
    /// when the end card came due (what a page opens it on: game_draw `ended`), on the game's clock
    pub end_cards: Vec<f64>,
    ended: bool,
    /// every smoke event the game told its host of, in order: what it wants of the host's solver
    pub smoke: Vec<game::out_events::OutEvent>,
    /// each man's last bubble heard: when it was said, and its words
    spoke: Vec<(GuardId, f64, String)>,
    /// where the hand's cursor points in the world, kept there as the camera moves (None: where the host left it). The host's and no part of the game: whoever puts a game back puts this back too
    pub cursor: Option<[f64; 3]>,
    /// the game's own words where the address's mission was refused (`Opts.may_refuse`)
    pub refused: Option<String>,
    _one: MutexGuard<'static, ()>,
}

fn start(data: &Json, room_clear: bool, o: &Opts) -> Play { start_by(data, room_clear, None, o) }
fn start_by(data: &Json, room_clear: bool, link: Option<&str>, o: &Opts) -> Play {
    // (a second game stood by the thread that holds the first would wait on itself for ever: said, not waited for)
    if HOLDS.with(|h| h.get()) { panic!("one game at a time: this test still holds a Play. Drop it (`drop(p)`, or end its block) before standing another"); }
    // (a test that failed while it held the lock left it poisoned and its game dead: the next one stands on a block zeroed anew)
    let one = ONE_GAME.lock().unwrap_or_else(|p| p.into_inner());
    let ch = CHARACTER.get_or_init(files::CharacterFiles::read);
    let scenario = if room_clear { Some(game::scene_scenario_list::ScenarioSpec { kind: game::scene_scenarios::MissionKind::Roomclear, squad: o.squad as f64, level: None, seed: None }) } else { None };
    let Stood { mut game, host, refused, .. } = stand::stand(data, ch.assets(), &StartOpts { seed: o.seed, seats: o.seats, scenario, link: link.map(String::from), dress: o.dress, may_refuse: o.may_refuse, ..Default::default() });
    game.out.clear();
    HOLDS.with(|h| h.set(true));   // (only once a Play exists to put it down: a stand that panicked holds nothing)
    let mut p = Play { g: game, host, frames: 0, told: Vec::new(), said: Vec::new(), sounds: Vec::new(), end_cards: Vec::new(), ended: false, smoke: Vec::new(), cursor: None, refused, spoke: Vec::new(), _one: one };
    p.hear();
    p
}

impl Play {
    /// Stand a scene: its level file through the start, and its room clear started where it says a team.
    pub fn scene(s: &Scene, o: &Opts) -> Play {
        // (through the text, as a file would come: what stands is what the file says)
        let data = json::parse(&s.level_text()).unwrap_or_else(|e| panic!("the scene's own level file does not read back: {}", e));
        start(&data, s.is_room_clear(), o)
    }
    /// Stand a level of `levels/` (or a path), as its file has it; `room_clear` starts that mission on it.
    pub fn level(name: &str, room_clear: bool, o: &Opts) -> Play {
        let data = json::parse(&files::level_text(name)).unwrap_or_else(|e| panic!("{}: {}", name, e));
        start(&data, room_clear, o)
    }

    /// Stand a level of `levels/` and start a mission on it FROM AN ADDRESS'S WORDS, as a click on a row of the mission list does
    /// (`?levelFile=depot.json&roomclear&squad4`): the row's own hour and weather, the fog of war, a quiet mission's rung, then the mission.
    pub fn mission(name: &str, link: &str, o: &Opts) -> Play {
        let data = json::parse(&files::level_text(name)).unwrap_or_else(|e| panic!("{}: {}", name, e));
        start_by(&data, false, Some(link), o)
    }
    /// The same on a level file's tree a caller has already read (and perhaps changed).
    pub fn mission_of(data: &Json, link: &str, o: &Opts) -> Play { start_by(data, false, Some(link), o) }

    /// one frame of the host; what the step told its host is kept (`said`, `sounds`)
    pub fn frame(&mut self) { self.frame_shown(|_| {}); }
    /// …with what a host does between the tick and the emptying of its out list: draws, as a page does, while `g.out` still holds what was told
    pub fn frame_shown(&mut self, show: impl FnOnce(&mut Play)) {
        use game::out_events::OutEvent as E;
        if let Some(at) = self.cursor {
            let (w, h) = (self.host.canvas.client_width, self.host.canvas.client_height);
            let (sx, sy, front) = game::camera::follow_camera_project(&mut self.g, self.host.cam, at, w, h, None, None);
            if front { self.g.tab.input[self.host.input].mouse_x = sx; self.g.tab.input[self.host.input].mouse_y = sy; }
        }
        stand::step(&mut self.g, &self.host, DT);
        let asked = game::savestate::control::asked(&self.g.out);
        let told = game::savestate::control::after_step(&mut self.g, DT, asked);
        if told.kept.is_some() || told.back || told.said.is_some() { self.told.push(told); }
        self.frames += 1;
        show(self);
        let out = core::mem::take(&mut self.g.out);
        let t = self.g.time;
        for e in out {
            match e {
                E::GameAudioLikePlay(p) => self.sounds.push(Sound { t, name: p.name.clone(), at: p.pos }),
                E::SmokeSystemLikeEmit(_) | E::SmokeSystemLikePush(_) | E::SmokeSystemLikeRound(_) => self.smoke.push(e),
                _ => {}
            }
        }
        let ended = game_draw::view::ended(&self.g) != 0;
        if ended && !self.ended { self.end_cards.push(t); }
        self.ended = ended;
        self.hear();
    }
    /// every man's bubble that is new since the last frame goes on `said`
    fn hear(&mut self) {
        for i in 0..self.g.guards.len() {
            let gd = self.g.guards[i];
            let Some(b) = self.g.gd[gd].bubble.as_ref() else { continue };
            let at = self.spoke.iter().position(|x| x.0 == gd);
            if let Some(k) = at { if self.spoke[k].1 == b.t && self.spoke[k].2 == b.text { continue; } }
            self.said.push(Said { t: b.t, who: gd, text: b.text.clone(), radio: b.radio });
            match at { Some(k) => self.spoke[k] = (gd, b.t, b.text.clone()), None => self.spoke.push((gd, b.t, b.text.clone())) }
        }
    }
    /// ONE FRAME OF A NETWORKED MACHINE THAT SHOWS THE SEAT `shown`: it is handed every seat's command and steps on them all, so two machines' games
    /// get the same hands and only the shown seat differs. Each seat's command is sampled from its own Input through its own camera (idle, unless
    /// a test writes them). Between frames the first seat stands on the game, for the test to read.
    pub fn net_frame(&mut self, shown: usize) {
        let cmds = self.net_sample();
        self.net_step(&cmds, shown);
    }
    /// each seat's command, sampled from its own Input through its own camera
    pub fn net_sample(&mut self) -> Vec<game::cmd::PlayerCmd> {
        let seats = self.g.seats.clone();
        let canvas = self.host.canvas;
        seats.iter().map(|&s| {
            let (input, cam) = (self.g.tab.cmd_seat[s].input, self.g.tab.cmd_seat[s].cam);
            game::seats::with_seat(&mut self.g, s, &mut |g| game::cmd::sample_cmd(g, input, cam, canvas, None))
        }).collect()
    }
    /// the game steps on every seat's command, on a machine that shows the seat `shown`
    pub fn net_step(&mut self, cmds: &[game::cmd::PlayerCmd], shown: usize) {
        self.net_tick(cmds, shown);
        self.g.out.clear();
    }
    /// … with what the step told its host still on the game
    fn net_tick(&mut self, cmds: &[game::cmd::PlayerCmd], shown: usize) {
        let seats = self.g.seats.clone();
        game::seats::show_seat(&mut self.g, seats[shown]);
        game::game::game_step_seats(&mut self.g, DT, cmds);
        game::seats::show_seat(&mut self.g, seats[0]);
        self.frames += 1;
    }
    /// WHAT THE PAGE DOES WITH A TICK'S CHECKPOINTS (rust/game_face page::saves), as far as the play goes: what the tick asked of the controller, and a checkpoint taken. Nothing is kept: nobody here goes back.
    fn host_saves(&mut self) {
        use game::out_events::OutEvent as E;
        use game::savestate::control::{self, Do};
        let asked: Vec<E> = self.g.out.iter().filter(|e| matches!(e, E::CheckpointsKeep(_) | E::CheckpointsRequest(_))).cloned().collect();
        for e in asked { match e { E::CheckpointsKeep(e) => control::keep(&mut self.g, &e.label), E::CheckpointsRequest(e) => control::request(&mut self.g, &e.why, &e.label), _ => {} } }
        if let Some(Do::Keep { label, .. }) = control::tick(&mut self.g, DT) { let _ = game::chronicle::chron_checkpoint(&mut self.g, game::chronicle::ChronCheckpointWhat::Save, Some(&label)); }
    }
    /// what a host does round a step with hands on its seats: each seat's camera after its man, the end of its input's frame
    pub fn lens_and_keys(&mut self) {
        for s in self.g.seats.clone() {
            let (input, cam) = (self.g.tab.cmd_seat[s].input, self.g.tab.cmd_seat[s].cam);
            let (follow, aim) = game::seats::with_seat(&mut self.g, s, &mut |g| (game::game::game_follow_pos(g, None), g.aim_point));
            game::camera::follow_camera_update(&mut self.g, cam, DT, follow, Some(aim));
            game::input::input_end_frame(&mut self.g, input);
        }
    }
    /// ONE TICK WITH A SCRIPT'S HANDS ON THE FIRST SEAT, by the road the page's own goes with hands on a seat (face_hand, face_step): each seat sampled, his hand written over the first seat's
    /// with the box world's copy standing for his questions, the game stepped on them, its checkpoints, the cameras after their men.
    pub fn played_by(&mut self, script: &mut dyn crate::player::Script) {
        self.lens_of(script, 0);
        let mut cmds = self.net_sample();
        self.hand_by(script, &mut cmds[0]);
        self.net_tick(&cmds, 0);
        self.host_saves();
        self.g.out.clear();
        self.hear();
        self.lens_and_keys();
    }
    /// his hand over what the host sampled for his seat, with the box world's copy standing for his questions
    pub fn hand_by(&mut self, script: &mut dyn crate::player::Script, cmd: &mut game::cmd::PlayerCmd) {
        if game::ask::WorldAsk::try_of(&self.g).is_err() { game::scene_collision::stands(&mut self.g); }
        game::scene_box_mirror::box_mirror_things(&mut self.g);
        script.hand(&self.g, cmd, DT);
    }
    /// the view of the seat turned and pulled back as he asked since his last hand
    pub fn lens_of(&mut self, script: &mut dyn crate::player::Script, seat: usize) {
        let [yaw, zoom] = script.lens();
        let cam = self.g.tab.cmd_seat[self.g.seats[seat]].cam;
        if yaw != 0.0 { game::camera::follow_camera_rotate(&mut self.g, cam, yaw); }
        if zoom != 0.0 { game::camera::follow_camera_zoom(&mut self.g, cam, zoom); }
    }
    /// … until he has nothing more to do, for at most `secs`: did he get there
    pub fn played_out(&mut self, script: &mut dyn crate::player::Script, secs: f64) -> bool {
        for _ in 0..(secs * 60.0).round() as usize { if script.over() { return true; } self.played_by(script); }
        script.over()
    }
    /// Every part of the game's state, a word a part (`parts::names` names them), rust/sim's block with the rest: what two machines of one game must agree on.
    pub fn words(&mut self) -> Vec<u32> {
        let (mut w, mut out) = (game::bytes::W::new(), Vec::new());
        crate::parts::words(&mut self.g, &mut w, &mut out);
        out
    }
    /// Run `secs` of game time.
    pub fn run(&mut self, secs: f64) { for _ in 0..(secs * 60.0).round() as usize { self.frame(); } }
    /// Run until `p` holds, for at most `secs`: the seconds it took, or None if it never held. `p` is asked before the first frame and after
    /// every frame of the `secs`.
    pub fn until(&mut self, secs: f64, mut p: impl FnMut(&mut Play) -> bool) -> Option<f64> {
        let n = (secs * 60.0).round() as usize;
        for k in 0..=n { if p(self) { return Some(k as f64 * DT); } if k < n { self.frame(); } }
        None
    }
    /// Run `secs` and hold `p` before the first frame and after every frame of them: None if it held throughout, else the second it first failed.
    pub fn always(&mut self, secs: f64, mut p: impl FnMut(&mut Play) -> bool) -> Option<f64> {
        let n = (secs * 60.0).round() as usize;
        for k in 0..=n { if !p(self) { return Some(k as f64 * DT); } if k < n { self.frame(); } }
        None
    }
    /// game time, in seconds
    pub fn time(&self) -> f64 { self.g.time }

    // ---- the hand: Sam is moved as a player moves him, through the host's Input

    /// The cursor on the place `at` (chest height), held there as the camera moves.
    pub fn point_at(&mut self, at: [f64; 2]) { self.cursor = Some([at[0], 1.2, at[1]]); }
    /// The same, at any height: the cursor is laid on that point of the world, and the ray from the camera goes through it to whatever it meets first.
    pub fn cursor_on(&mut self, at: [f64; 3]) { self.cursor = Some(at); }
    /// A mouse button goes down and stays down.
    pub fn press(&mut self, b: Button) { let i = &mut self.g.tab.input[self.host.input]; i.buttons = ((i.buttons as u32) | b as u32) as f64; i.clicked = ((i.clicked as u32) | b as u32) as f64; }
    /// …and comes up.
    pub fn release(&mut self, b: Button) { let i = &mut self.g.tab.input[self.host.input]; i.buttons = ((i.buttons as u32) & !(b as u32)) as f64; }
    /// A key goes down (its code as the page has it: "KeyW", "ShiftLeft") and stays down.
    pub fn key_down(&mut self, code: &str) { let i = &mut self.g.tab.input[self.host.input]; i.keys.add(String::from(code)); i.pressed.add(String::from(code)); }
    /// …and comes up.
    pub fn key_up(&mut self, code: &str) { self.g.tab.input[self.host.input].keys.delete(String::from(code)); }
    /// Sam's rifle comes to his hands, if it is not there (the page's key 2; on a night he starts with his guns put away).
    pub fn sam_takes_his_rifle(&mut self) {
        let pl = self.g.player;
        if self.g.pl[pl].slot == game::game::SlotKind::Rifle && !self.g.pl[pl].holstered { return; }
        self.key_down("Digit2");
        self.run(0.1);
        self.key_up("Digit2");
        self.run(0.7);
    }
    /// Sam fires ONE ROUND of his rifle at `at`: the rifle in his hands, the cursor on the place for `settle` seconds, the trigger pulled and let
    /// go. (The right button is not touched: with the rifle up it is the launcher's, and letting it go sends the launcher's load.)
    pub fn sam_fires_at(&mut self, at: [f64; 2], settle: f64) {
        self.sam_takes_his_rifle();
        self.point_at(at);
        self.run(settle);
        self.press(Button::Left);
        self.frame();
        self.frame();
        self.release(Button::Left);
        self.run(0.1);
    }

    // ---- what a test does to the world, each through the game's own call

    /// AN ORDER, by the menu's own call (`SquadOrders.give`): the order's id as the menu has it ("stack", "fallIn", "openClear"), on what stands
    /// at `at` (a door, a man, the floor: `SquadOrders.targetAt`), to the whole squad or one pair. An order the game refuses fails the test with the game's reason.
    pub fn order(&mut self, id: &str, at: [f64; 2], who: game::orders::Who) {
        let t = game::orders::squad_orders_target_at(&mut self.g, [at[0], 0.0, at[1]]);
        if let Some(why) = game::orders::squad_orders_give(&mut self.g, id, Some(&t), Some(who), None) { panic!("the order '{}' at ({}, {}) was refused: {}", id, at[0], at[1], why); }
    }

    /// the same, where the place may be one the order is not for (a man stood in a doorway): the game's reason if it refused
    pub fn tries_order(&mut self, id: &str, at: [f64; 2], who: game::orders::Who) -> Option<String> {
        let t = game::orders::squad_orders_target_at(&mut self.g, [at[0], 0.0, at[1]]);
        game::orders::squad_orders_give(&mut self.g, id, Some(&t), Some(who), None)
    }

    /// THE SAME ORDER FROM ANOTHER SEAT (0 = the first player): given as that player gives it, on his own menu and to his own men.
    pub fn seat_orders(&mut self, seat: usize, id: &str, at: [f64; 2], who: game::orders::Who) {
        let s = self.g.seats[seat];
        let why = game::seats::with_seat(&mut self.g, s, &mut |g| { let t = game::orders::squad_orders_target_at(g, [at[0], 0.0, at[1]]); game::orders::squad_orders_give(g, id, Some(&t), Some(who), None) });
        if let Some(why) = why { panic!("seat {}'s order '{}' at ({}, {}) was refused: {}", seat, id, at[0], at[1], why); }
    }
    /// A PLAYER IS SET DOWN `metres` along x from where he stands (the sandbox's own teleport, `Game.teleportPlayer`, in his seat): a stride of a
    /// known length, where walking him there by his keys would be a test of the keys.
    pub fn seat_steps(&mut self, seat: usize, metres: f64) {
        let s = self.g.seats[seat];
        game::seats::with_seat(&mut self.g, s, &mut |g| { let at = g.ch[g.pl[g.player].ch].pos; game::game::game_teleport_player(g, [at[0] + metres, 0.0, at[2]]); });
    }
    /// A PLAYER IS SET DOWN at a place of the floor (the same teleport).
    pub fn seat_stands(&mut self, seat: usize, at: [f64; 2]) {
        let s = self.g.seats[seat];
        game::seats::with_seat(&mut self.g, s, &mut |g| game::game::game_teleport_player(g, [at[0], 0.0, at[1]]));
    }
    /// a man of the list set down at a place: a test of what he sees from there, where walking him is a test of his legs
    pub fn man_stands(&mut self, gd: GuardId, at: [f64; 2]) { let c = self.g.gd[gd].ch; self.g.ch[c].pos[0] = at[0]; self.g.ch[c].pos[2] = at[1]; }
    /// the way in the room clear that is up stood its team on: 0 the level's own, k its k-th other
    pub fn way_taken(&self) -> usize { self.g.staged.way as usize }
    /// where each teammate the mission staged stands (x, z), in the order it staged them
    pub fn team_at(&self) -> Vec<[f64; 2]> { self.g.staged.team.iter().map(|&m| { let p = self.g.ch[self.g.gd[m].ch].pos; [p[0], p[2]] }).collect() }
    /// the restart a player presses: the same mission staged again
    pub fn restaged(&mut self) { game::savestate::control::restart(&mut self.g); }
    /// where a seat's player stands
    pub fn seat_at(&mut self, seat: usize) -> [f64; 2] { let s = self.g.seats[seat]; game::seats::with_seat(&mut self.g, s, &mut |g| { let p = g.ch[g.pl[g.player].ch].pos; [p[0], p[2]] }) }
    /// A MAN GOES DOWN by combat's own call (`killGuard`: shot, by `by`'s hand), for a test of what follows a death and not of the shot.
    pub fn falls(&mut self, gd: GuardId, by: game::game::DownBy) { game::combat::kill_guard(&mut self.g, gd, [1.0, 0.0, 0.0], Some(false), Some(game::game::DownKind::Shot), Some(by), None); }

    /// A live fragmentation grenade lands at `at` (the sandbox's own call, `Game.dropThrowable`: an armed can as if just thrown from where Sam stands).
    pub fn frag_lands(&mut self, at: [f64; 2]) { game::game::game_drop_throwable(&mut self.g, game::game::ThrowKind::Frag, [at[0], 0.0, at[1]]); }
    /// how long a thrown grenade's fuse is, seconds
    pub fn frag_fuse(&self) -> f64 { self.g.k.frag.fuse }

    /// A flash lands at `at` (the sandbox's own call, as `frag_lands`): it goes off after its fuse.
    pub fn flash_lands(&mut self, at: [f64; 2]) { game::game::game_drop_throwable(&mut self.g, game::game::ThrowKind::Flash, [at[0], 0.0, at[1]]); }

    /// Sam is nobody's enemy to see (the page's own option)
    pub fn sam_unseen(&mut self) { self.g.player_invisible = true; }
    /// Nothing hurts Sam from here on (the sandbox's switch): for a test that is not about him.
    pub fn sam_unhurt(&mut self) { self.g.god_mode = true; }
    /// the lines `gd` has said so far
    pub fn lines(&self, gd: GuardId) -> Vec<&str> { self.said.iter().filter(|s| s.who == gd).map(|s| s.text.as_str()).collect() }

    // ---- who is who

    /// the guard who walks the route `name` (a night's `patrol` or `sentry`)
    pub fn guard(&self, name: &str) -> GuardId {
        self.g.guards.iter().copied().find(|&gd| self.g.tab.route[self.g.gd[gd].route].name == name && self.g.gd[gd].unit.is_none())
            .unwrap_or_else(|| panic!("no guard walks a route '{}' (the guards: {})", name, self.roll()))
    }
    /// the men of the other side who are up, nearest `at` first (a room clear's defenders are dealt to their rooms' marks by the die: a test
    /// finds the man at a mark by where he stands)
    pub fn hostiles_by(&self, at: [f64; 2]) -> Vec<GuardId> {
        let mut v: Vec<GuardId> = self.g.guards.iter().copied().filter(|&gd| self.g.gd[gd].team == game::teams::Team::Hostile && !game::condition::down(&self.g, self.g.gd[gd].ch)).collect();
        v.sort_by(|a, b| dist(self.at(*a), at).partial_cmp(&dist(self.at(*b), at)).unwrap());
        v
    }
    /// the defender posted at the mark `at` (within a metre of it as the game stands)
    pub fn defender_at(&self, at: [f64; 2]) -> GuardId {
        let v = self.hostiles_by(at);
        match v.first() { Some(&gd) if dist(self.at(gd), at) < 1.0 => gd, _ => panic!("no defender stands within a metre of ({}, {}) (the guards: {})", at[0], at[1], self.roll()) }
    }
    /// THE PAR CLOCK, as the game keeps it. Is one live (the level has a par and its room clear is up) …
    pub fn par_live(&self) -> bool { self.g.par.live }
    /// … the level's bars (gold, silver, bronze, seconds a man lost costs) …
    pub fn par_bars(&self) -> Option<game::scene_scenarios::ParData> { game::par::par_data(&self.g) }
    /// … when it started on the game's clock (None: armed and waiting for somebody to do something) …
    pub fn par_started(&self) -> Option<f64> { if game::par::par_started(&self.g) { Some(self.g.par.t0) } else { None } }
    /// … how many times a clock has started and a run has been filed, over the game's life …
    pub fn par_counts(&self) -> (i32, i32) { (self.g.stats.par.started, self.g.stats.par.finished) }
    /// … and the finished run (None: not taken yet, or wiped).
    pub fn par_result(&self) -> Option<game::par::ParResult> { game::par::par_result_now(&self.g).cloned() }
    /// how many door charges a seat's squad has set and not blown
    pub fn seat_door_charges(&self, seat: usize) -> usize { game::seats::seat(&self.g, self.g.seats[seat]).door_tools.set.len() }
    /// how many wall charges the first player's squad has set and not blown
    pub fn wall_charges(&self) -> usize { self.g.wall_charge.set.len() }
    /// THE SQUAD IS TOLD TO SET A WALL CHARGE at a place of a wall, by the order's own call, carrying what it needs. An order the game refuses fails the test with the game's reason.
    pub fn orders_wall_charge(&mut self, at: [f64; 3]) {
        game::wallcharge::wall_charge_raise_carried(&mut self.g, 2.0);
        if let Some(why) = game::wallcharge::wall_charge_order(&mut self.g, at, None) { panic!("the wall charge was refused: {}", why); }
    }
    /// a place of the floor `off` metres out from the first wall charge's face, on the side it was set from
    pub fn off_the_wall_charge(&self, off: f64) -> [f64; 2] { let w = self.g.tab.wallcharge_set_wall_charge[self.g.wall_charge.set[0]].wall.clone(); let at = game::wallcharge::wall_pt(&self.g, &w, 0.0, off, None); [at[0], at[2]] }
    /// the objective row, as a HUD is given it
    pub fn objective(&self) -> String { self.view().mission.text }
    /// WHAT A HUD IS TOLD of the game as it stands (`view::tick_view`: the record the page draws, asked of the game and never kept by it)
    pub fn view(&self) -> game::view::TickView { self.view_of(0) }
    /// WHAT A HUD IS TOLD OF ITS CURSOR with the cursor on a point of the world, through the host's camera as it stands (None: the point is not in the picture)
    pub fn pointed_at(&self, seat: usize, at: [f64; 3]) -> Option<game::view::CursorView> {
        let (w, h) = (self.host.canvas.client_width, self.host.canvas.client_height);
        let (sx, sy, front) = game::camera::follow_camera_project(&self.g, self.host.cam, at, w, h, None, None);
        if front && sx >= 0.0 && sx < w && sy >= 0.0 && sy < h { Some(self.pointed(seat, [sx, sy])) } else { None }
    }
    /// …with the cursor at a place of the picture
    pub fn pointed(&self, seat: usize, mouse: [f64; 2]) -> game::view::CursorView {
        let now = game::view::HostNow { mouse, ..game::view::host_now(&self.g, self.host.input, self.host.cam, self.host.canvas) };
        game::view::cursor_view(&self.g, &game::seats::seat(&self.g, self.g.seats[seat]), &now)
    }
    /// …and what another player's HUD is told (0 = the first)
    pub fn view_of(&self, seat: usize) -> game::view::TickView { game::view::tick_view(&self.g, &game::seats::seat(&self.g, self.g.seats[seat])) }
    /// how the room clear ended and when, on the game's clock (None: it is on)
    pub fn room_clear_end(&self) -> Option<(game::roomclear::RoomClearEnd, f64)> { self.g.room_clear.end.map(|e| (e, self.g.room_clear.end_at)) }
    /// the men of the other side who are up
    pub fn hostiles(&self) -> Vec<GuardId> { let at = self.sam_at(); self.hostiles_by(at) }
    /// A ROUND FROM A MAN by combat's own call (`fire_weapon`, steady and with no spread) at a point of the floor, chest high.
    pub fn man_fires_at(&mut self, gd: GuardId, at: [f64; 2]) {
        let c = self.g.gd[gd].ch;
        let o = game::combat::FireOpts { steady: Some(true), spread: Some(0.0), ..game::combat::FireOpts::default() };
        game::combat::fire_weapon(&mut self.g, c, [at[0], 1.2, at[1]], Some(&o));
    }
    /// A MAN GOES DOWN by combat's own call, in any of its ways (`quiet`: put out with no noise).
    pub fn falls_as(&mut self, gd: GuardId, how: game::game::DownKind, quiet: bool, by: game::game::DownBy) { game::combat::kill_guard(&mut self.g, gd, [1.0, 0.0, 0.0], Some(quiet), Some(how), Some(by), None); }
    /// the player of a seat takes a round's hit from the way `from_dir`, in his own seat, until he has none left (combat's `hit_player`: nobody fired it)
    pub fn seat_goes_down(&mut self, seat: usize) {
        let s = self.g.seats[seat];
        game::seats::with_seat(&mut self.g, s, &mut |g| { while !game::condition::down(g, g.pl[g.player].ch) { game::combat::hit_player(g, [1.0, 0.0, 0.0]); } });
    }
    /// is a seat's player down
    pub fn seat_down(&mut self, seat: usize) -> bool { let s = self.g.seats[seat]; game::seats::with_seat(&mut self.g, s, &mut |g| game::condition::down(g, g.pl[g.player].ch)) }
    /// a seat's man's character id
    pub fn seat_id(&mut self, seat: usize) -> f64 { let s = self.g.seats[seat]; game::seats::with_seat(&mut self.g, s, &mut |g| g.ch[g.pl[g.player].ch].id) }
    /// a man's character id
    pub fn id_of(&self, gd: GuardId) -> f64 { self.g.ch[self.g.gd[gd].ch].id }
    /// what the end card shows the seat of this number
    pub fn debrief_view(&self, seat: usize) -> game::view::DebriefView { game::view::debrief_view(&self.g, &game::seats::seat(&self.g, self.g.seats[seat])) }
    /// THE RECAP, as the game keeps it. What the end card may show (None: somebody still plays, or nobody of ours went down) …
    pub fn recap_card(&self) -> Option<game::deathrecap::DeathRecap> { game::deathrecap::recap_card(&self.g) }
    /// … every one of ours lost so far this run, whoever still plays: never for a screen …
    pub fn recap_deaths(&self) -> Vec<game::deathrecap::RecapDeath> { self.g.recap.deaths.iter().cloned().collect() }
    /// … the squad's men lost, as the squad knows them …
    pub fn recap_lost(&self) -> Vec<game::deathrecap::LostMan> { self.g.recap.lost_live.clone() }
    /// … and the word a man has over his head of the cues' own (the recap's side call goes there; his own lines do not).
    pub fn cue_over(&self, gd: GuardId) -> Option<String> { self.g.cues.bubbles.get(gd).map(|b| b.text.clone()) }
    /// A man held where a test wants him, his back the way it says, as suspicious or alert as it says: for a rule that is not about his senses. Each frame.
    pub fn pin_man(&mut self, gd: GuardId, at: [f64; 2], yaw: f64, state: GuardState, aware: f64) {
        let c = self.g.gd[gd].ch;
        self.g.ch[c].pos = [at[0], 0.0, at[1]];
        self.g.ch[c].vel = [0.0; 3];
        self.g.ch[c].body_yaw = yaw;
        self.g.ch[c].aim_yaw = yaw;
        self.g.cond[c].down = state == GuardState::Dead;
        let h = &mut self.g.gd[gd];
        (h.hold, h.pinned, h.state, h.awareness, h.path_goal) = (true, true, state, aware, None);
        h.path.clear();
    }
    /// Every man of the other side but these is down.
    pub fn drop_all_but(&mut self, keep: &[GuardId]) {
        for gd in self.g.guards.clone() {
            if self.g.gd[gd].team != game::teams::Team::Operative && !keep.contains(&gd) && self.up(gd) { game::combat::kill_guard(&mut self.g, gd, [0.0, 0.0, 1.0], Some(true), None, None, None); }
        }
    }
    /// Sam's teammates, in the order the game made them
    pub fn mates(&self) -> Vec<GuardId> { self.g.guards.iter().copied().filter(|&gd| self.g.gd[gd].team == game::teams::Team::Operative).collect() }
    /// every man of the game's list in a line, for a failure's message
    pub fn roll(&self) -> String {
        self.g.guards.iter().map(|&gd| { let o = &self.g.gd[gd]; let p = self.at(gd); format!("{} {:?} {:?} at ({:.1}, {:.1})", o.callsign, o.team, o.state, p[0], p[1]) }).collect::<Vec<_>>().join("; ")
    }

    /// what stands on the level's floor, counted off the game (for a test of the start itself)
    pub fn floor(&self) -> Floor {
        let g = &self.g;
        let count = |f: &dyn Fn(&game::scene_boxes::Box) -> bool| g.col.boxes.iter().filter(|&&b| f(&g.tab.boxes[b])).count();
        Floor {
            boxes: g.col.boxes.len(), round: count(&|b| b.round == Some(true)), shaped: count(&|b| b.shaped == Some(true)), trim: count(&|b| b.piece == Some(true)),
            civilians: g.civilians.len(), cameras: g.devices.cameras.len(), lasers: g.devices.lasers.len(), keypads: g.devices.keypads.len(),
            vents: g.level.vents.as_ref().map_or(0, |v| v.len()),
        }
    }

    // ---- what a man is doing, read off the game

    /// where a man stands, in plan
    pub fn at(&self, gd: GuardId) -> [f64; 2] { let p = self.g.ch[self.g.gd[gd].ch].pos; [p[0], p[2]] }
    /// where Sam stands
    pub fn sam_at(&self) -> [f64; 2] { let p = self.g.ch[self.g.pl[self.g.player].ch].pos; [p[0], p[2]] }
    /// Sam is PUT at `at` (the sandbox's own call, `Game.teleportPlayer`): for a test whose staging is a place and not the walk to it.
    pub fn sam_put(&mut self, at: [f64; 2]) { game::game::game_teleport_player(&mut self.g, [at[0], 0.0, at[1]]); }
    /// Can Sam not see: a flash or a blast has his eyes for now.
    pub fn sam_blind(&self) -> bool { self.g.time < self.g.cond[self.g.pl[self.g.player].ch].blind_until }
    /// Every row the orders' table holds, by its id, in the table's order: on the menu or off it.
    pub fn menu_rows(&self) -> Vec<String> { self.g.tab.orders_order_def.iter().map(|(_, r)| r.id.clone()).collect() }
    /// The rows on the menu now, by id, in the menu's order.
    pub fn menu(&self) -> Vec<String> { self.g.orders.defs.iter().map(|&d| self.g.tab.orders_order_def[d].id.clone()).collect() }
    /// is the order of that id on the squad's menu now
    pub fn on_menu(&self, id: &str) -> bool { self.menu().iter().any(|r| r == id) }
    /// the living men of the red pair, and of the blue (the menu's own call, `SquadOrders.men`)
    pub fn red(&self) -> Vec<GuardId> { game::orders::squad_orders_men(&self.g, &game::seats::here(&self.g), Some(game::orders::SquadOrdersMenSel::Red), None) }
    pub fn blue(&self) -> Vec<GuardId> { game::orders::squad_orders_men(&self.g, &game::seats::here(&self.g), Some(game::orders::SquadOrdersMenSel::Blue), None) }
    /// The rule of the list that has him this tick, by the words of its line ("head down under fire"); None when his script or his own
    /// state has him.
    pub fn rule(&self, gd: GuardId) -> Option<&'static str> {
        use game::control::{rule_name, state, Holder};
        match state(&self.g)?.of(gd)?.holder[0] { Holder::Rule(r) => Some(rule_name(r)), _ => None }
    }
    /// why he is doing that, in a line: who has him and since when, which rules said no, who holds him to his post
    pub fn why(&self, gd: GuardId) -> String { game::control::control_line(&self.g, gd) }
    /// the way a man's body faces, radians: 0 looks along +z (south), a quarter turn (π/2) along +x (east), π north (his forward is (sin, cos))
    pub fn facing(&self, gd: GuardId) -> f64 { self.g.ch[self.g.gd[gd].ch].body_yaw }
    /// calm, suspicious, alert, searching or dead
    pub fn state(&self, gd: GuardId) -> GuardState { self.g.gd[gd].state }
    /// how sure he is that somebody is there, 0 to 1
    pub fn awareness(&self, gd: GuardId) -> f64 { self.g.gd[gd].awareness }
    /// where he believes his enemy is
    pub fn fix(&self, gd: GuardId) -> Option<[f64; 2]> { self.g.gd[gd].last_known.map(|p| [p[0], p[2]]) }
    /// how lit Sam is, as the light meter has it (0 dark to 1)
    pub fn sam_light(&self) -> f64 { self.g.shown[self.g.pl[self.g.player].ch].visibility }
    /// how fast he is moving over the floor, metres a second
    pub fn speed(&self, gd: GuardId) -> f64 { let v = self.g.ch[self.g.gd[gd].ch].vel; (v[0] * v[0] + v[2] * v[2]).sqrt() }
    /// Can he not see: a flash or a blast has his eyes for now.
    pub fn blind(&self, gd: GuardId) -> bool { self.g.time < self.g.cond[self.g.gd[gd].ch].blind_until }

    /// are his hands changing magazine, and how much of it is left, in seconds
    pub fn reloading(&self, gd: GuardId) -> Option<f64> { if game::control::man::changing_magazine(&self.g, gd) { Some(self.g.gd[gd].reload_t) } else { None } }
    /// how many rounds have left the gun in his hands since it was made
    pub fn rounds_fired(&self, gd: GuardId) -> f64 { self.g.fa[self.g.gd[gd].weapon].shots_fired }
    /// Staging: his gun has this many rounds in it and no more, one of them in the chamber.
    pub fn gun_has(&mut self, gd: GuardId, rounds: f64) { let w = self.g.gd[gd].weapon; let chamber = rounds.min(1.0); self.g.fa[w].chamber = chamber; self.g.fa[w].mag = Some(rounds - chamber); }

    /// Staging: he cannot see for that many seconds.
    pub fn blinded(&mut self, gd: GuardId, secs: f64) { self.g.cond[self.g.gd[gd].ch].blind_until = self.g.time + secs; }
    /// Staging: the gun in his hands is the only one he has.
    pub fn no_second_gun(&mut self, gd: GuardId) { self.g.gd[gd].second = None; }
    /// has he no hand for his gun: reeling, in somebody's arm, a hostage in his own, his hands on a job
    pub fn no_hand_for_it(&self, gd: GuardId) -> bool { use game::control::man::*; staggered(&self.g, gd) || held(&self.g, gd) || in_a_standoff(&self.g, gd) || game::gunslots::side_locked(&self.g, gd) }

    /// how many rounds he has heard fired, of those that mean anything to him
    pub fn shots_heard(&self, gd: GuardId) -> f64 { self.g.gd[gd].witness.heard.shots }
    /// has he had his man in his eyes within the last `secs` seconds
    pub fn saw_within(&self, gd: GuardId, secs: f64) -> bool { game::control::man::saw_his_man_within(&self.g, gd, secs) }

    /// where the open ways are, within `reach` of a man of the squad, onto floor nobody of it has been on, and whether a gun of it is on each
    pub fn watched(&mut self, reach: f64) -> Vec<([f64; 2], bool)> { game::cover::cover_watched(&mut self.g, reach).into_iter().map(|(o, on)| { let a = self.g.tab.cover_opening[o].at; ([a[0], a[2]], on) }).collect() }

    /// how far a man's gun is off a place, in degrees
    pub fn gun_off(&self, gd: GuardId, at: [f64; 2]) -> f64 {
        let (q, yaw) = (self.at(gd), self.g.ch[self.g.gd[gd].ch].aim_yaw);
        let turn = yaw - (at[0] - q[0]).atan2(at[1] - q[1]);
        ((turn + std::f64::consts::PI).rem_euclid(std::f64::consts::TAU) - std::f64::consts::PI).abs().to_degrees()
    }

    /// the mark his order has for him, if it has one
    pub fn mark_of(&self, gd: GuardId) -> Option<[f64; 2]> { let a = game::orders::squad_orders_of(&self.g, &game::seats::here(&self.g), gd)?; self.g.tab.orders_assignment[a].post.map(|p| [p[0], p[2]]) }

    /// The spot the rounds past him have sent him to, while he is on his way or about to go (the suppression rule's cover spot); `None`: not sent.
    pub fn sent_to_cover(&self, gd: GuardId) -> Option<[f64; 2]> { self.g.cond[self.g.gd[gd].ch].sup.cover.map(|p| [p[0], p[2]]) }

    /// Sam's body: what the readers of a pose below are asked about
    pub fn sam_body(&self) -> game::ids::CharId { self.g.pl[self.g.player].ch }
    /// where a joint of that body stands in the world this frame
    pub fn joint(&self, c: game::ids::CharId, j: game::character::Bone) -> [f64; 3] { *self.g.ch[c].bones.get(j).expect("the joint") }
    /// every joint of that body, in one order
    pub fn joints_named(&self, c: game::ids::CharId) -> Vec<(game::character::Bone, [f64; 3])> { let b = &self.g.ch[c].bones; b.keys().into_iter().map(|k| (k, *b.get(k).unwrap())).collect() }
    /// which way that body faces
    pub fn body_yaw(&self, c: game::ids::CharId) -> f64 { self.g.ch[c].body_yaw }
    /// the line of the gun in that body's hands: its muzzle and which way it points
    pub fn gun_line(&self, c: game::ids::CharId) -> ([f64; 3], [f64; 3]) { (self.g.ch[c].muzzle, self.g.ch[c].gun_dir) }
    /// tell that body to lean round an edge (-1 to 1; 0 lets it come back), and how far it is shown as leaning now
    pub fn lean(&mut self, c: game::ids::CharId, by: f64) { self.g.ch[c].anim.lean = by; }
    pub fn lean_shown(&self, c: game::ids::CharId) -> f64 { self.g.ch[c].anim.lean_sm }

    /// is he down low (crouched or lower: what a man does behind cover, or with a grenade near)
    pub fn low(&self, gd: GuardId) -> bool { self.g.ch[self.g.gd[gd].ch].anim.crouch_target >= 0.5 }
    /// is he in the stack's stance (set on a door)
    pub fn stacked(&self, gd: GuardId) -> bool { self.g.ch[self.g.gd[gd].ch].anim.stance == game::character::Stance::Stack }
    /// is the man on his feet (not dead, not down)
    pub fn up(&self, gd: GuardId) -> bool { !game::condition::down(&self.g, self.g.gd[gd].ch) && self.g.gd[gd].down_kind.is_none() }
}

impl Drop for Play { fn drop(&mut self) { HOLDS.with(|h| h.set(false)); } }

/// A number in -1..1 that is a fixed function of the seed and of `k` (which of a scene's numbers it moves): a test moves a mark, a range, a
/// door or a landing point by it, so that three seeds are three different fights.
pub fn nudge(seed: u32, k: u32) -> f64 {
    let mut x = seed.wrapping_mul(0x9E37_79B9) ^ k.wrapping_mul(0x85EB_CA6B);
    x ^= x >> 16; x = x.wrapping_mul(0x7FEB_352D); x ^= x >> 15; x = x.wrapping_mul(0x846C_A68B); x ^= x >> 16;
    x as f64 / 2_147_483_648.0 - 1.0
}

/// The first frame at which two games' words differ, and the parts of the state that do.
pub fn parting(a: &[Vec<u32>], b: &[Vec<u32>]) -> Option<(usize, String)> {
    let k = a.iter().zip(b).position(|(x, y)| x != y)?;
    Some((k, crate::parts::differ(&crate::parts::names(), &a[k], &b[k]).join(", ")))
}

/// metres between two places in plan
pub fn dist(a: [f64; 2], b: [f64; 2]) -> f64 { ((a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2)).sqrt() }
