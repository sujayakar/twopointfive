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

/// the host's frame rate: a tick is a sixtieth of a second
pub const DT: f64 = 1.0 / 60.0;

/// What a test says of a stand beside the scene.
#[derive(Clone, Debug)]
pub struct Opts {
    /// the seed of the game's die: a test that holds over several seeds holds a rule, one seed holds a run
    pub seed: u32,
    /// how many teammates a room clear stages beside Sam (2 or 4)
    pub squad: u32,
    /// THE SENSES WORKED OUT BY THE GAME ITSELF (game/sense.ts SENSE.cpu, with the sky's light in the estimate): what a host with no picture
    /// has for eyes. On here, because with them off no light meter is ever fed and "lit" and "dark" are one thing. The page plays with them off.
    pub cpu_senses: bool,
    /// how many players (1 or 2): each has a seat, his own man and his share of the squad. The hand's keys and cursor are the first seat's.
    pub seats: u32,
    /// THE ROWS THAT ONLY DRESS THE PICTURE (game/dress.ts DRESS), as a page has them: the debris pool tumbling a break's pieces, above all. Off (the default) as
    /// on any host that draws nothing: a thrown piece is then only its one landing told to the floor, and the pool stays empty.
    pub dress: bool,
}
impl Default for Opts { fn default() -> Self { Opts { seed: 1, squad: 4, cpu_senses: true, seats: 1, dress: false } } }

/// What stands on a level's floor: its boxes (the round things, the boxes that stand shaped and the pieces of trim among them), and who and what
/// the game made of the file's civilians, devices and vents.
#[derive(Debug, PartialEq)]
pub struct Floor { pub boxes: usize, pub round: usize, pub shaped: usize, pub trim: usize, pub civilians: usize, pub cameras: usize, pub lasers: usize, pub keypads: usize, pub vents: usize, pub daylight: bool }

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
    /// EVERY LINE A MAN HAS SPOKEN, in order. Read off each man's speech bubble after every frame (`guards::say` writes one for every line it
    /// lets through, whoever asked for it, and none for a line it drops), so a man who speaks twice inside one frame is heard once.
    pub said: Vec<Said>,
    /// every sound the game told its host to play (`audio.play`: its name, when, and where it was, None for a sound with no place). Footsteps
    /// are another call of the host's and are not kept.
    pub sounds: Vec<Sound>,
    /// when the game told its host to open the end card, on the game's clock
    pub end_cards: Vec<f64>,
    /// each man's last bubble heard: when it was said, and its words
    spoke: Vec<(GuardId, f64, String)>,
    /// where the hand's cursor points in the world, kept there as the camera moves (None: where the host left it)
    cursor: Option<[f64; 3]>,
    _one: MutexGuard<'static, ()>,
}

fn start(data: &Json, room_clear: bool, o: &Opts) -> Play { start_by(data, room_clear, None, o) }
fn start_by(data: &Json, room_clear: bool, link: Option<&str>, o: &Opts) -> Play {
    // (a second game stood by the thread that holds the first would wait on itself for ever: said, not waited for)
    if HOLDS.with(|h| h.get()) { panic!("one game at a time: this test still holds a Play. Drop it (`drop(p)`, or end its block) before standing another"); }
    // (a test that failed while it held the lock left it poisoned and its game dead: the next one stands on a block zeroed anew)
    let one = ONE_GAME.lock().unwrap_or_else(|p| p.into_inner());
    let ch = CHARACTER.get_or_init(files::CharacterFiles::read);
    let scenario = if room_clear { Some(game::scene_scenario_list::ScenarioSpec { kind: game::scene_scenario_list::ScenarioSpecKind::Roomclear, squad: o.squad as f64, level: None, seed: None }) } else { None };
    let Stood { mut game, host, .. } = stand::stand(data, ch.assets(), &StartOpts { seed: o.seed, seats: o.seats, scenario, link: link.map(String::from), cpu_senses: o.cpu_senses, dress: o.dress, ..Default::default() });
    game.out.clear();
    HOLDS.with(|h| h.set(true));   // (only once a Play exists to put it down: a stand that panicked holds nothing)
    let mut p = Play { g: game, host, frames: 0, said: Vec::new(), sounds: Vec::new(), end_cards: Vec::new(), cursor: None, spoke: Vec::new(), _one: one };
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

    /// one frame of the host; what the step told its host is kept (`said`, `sounds`)
    pub fn frame(&mut self) {
        use game::out_events::OutEvent as E;
        if let Some(at) = self.cursor {
            let (w, h) = (self.host.canvas.client_width, self.host.canvas.client_height);
            let (sx, sy, front) = game::camera::follow_camera_project(&mut self.g, self.host.cam, at, w, h, None, None);
            if front { self.g.tab.input[self.host.input].mouse_x = sx; self.g.tab.input[self.host.input].mouse_y = sy; }
        }
        let out = stand::frame(&mut self.g, &self.host, DT);
        self.frames += 1;
        let t = self.g.time;
        for e in out {
            match e {
                E::GameAudioLikePlay(p) => self.sounds.push(Sound { t, name: p.name.clone(), at: p.pos }),
                E::EndCard => self.end_cards.push(t),
                _ => {}
            }
        }
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
        let seats = self.g.seats.clone();
        let canvas = self.host.canvas;
        let cmds: Vec<game::cmd::PlayerCmd> = seats.iter().map(|&s| {
            let (input, cam) = (self.g.tab.cmd_seat[s].input, self.g.tab.cmd_seat[s].cam);
            game::seats::with_seat(&mut self.g, s, &mut |g| game::cmd::sample_cmd(g, input, cam, canvas, None))
        }).collect();
        game::seats::show_seat(&mut self.g, seats[shown]);
        game::game::game_step_seats(&mut self.g, DT, &cmds);
        let now = self.g.time;
        game::render_lights::light_set_update(&mut self.g, DT, now);
        game::host_api::host_world_upload(&mut self.g);
        game::seats::show_seat(&mut self.g, seats[0]);
        self.g.out.clear();
        self.frames += 1;
    }
    /// Every part of the game's state, a word a part (`game::state_words::STATE_NAMES` names them): what two machines of one game must agree on.
    pub fn words(&mut self) -> Vec<u32> {
        let (mut w, mut out) = (game::io::W::new(), Vec::new());
        game::state_words::state_words(&mut self.g, &mut w, &mut out);
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
    /// where a seat's player stands
    pub fn seat_at(&mut self, seat: usize) -> [f64; 2] { let s = self.g.seats[seat]; game::seats::with_seat(&mut self.g, s, &mut |g| { let p = g.ch[g.pl[g.player].ch].pos; [p[0], p[2]] }) }
    /// A MAN GOES DOWN by combat's own call (`killGuard`: shot, by `by`'s hand), for a test of what follows a death and not of the shot.
    pub fn falls(&mut self, gd: GuardId, by: game::game::DownBy) { game::combat::kill_guard(&mut self.g, gd, [1.0, 0.0, 0.0], Some(false), Some(game::game::DownKind::Shot), Some(by), None, None); }

    /// A live fragmentation grenade lands at `at` (the sandbox's own call, `Game.dropThrowable`: an armed can as if just thrown from where Sam stands).
    pub fn frag_lands(&mut self, at: [f64; 2]) { game::game::game_drop_throwable(&mut self.g, game::game::ThrowKind::Frag, [at[0], 0.0, at[1]]); }
    /// how long a thrown grenade's fuse is, seconds
    pub fn frag_fuse(&self) -> f64 { self.g.k.frag.fuse }

    /// A flash lands at `at` (the sandbox's own call, as `frag_lands`): it goes off after its fuse.
    pub fn flash_lands(&mut self, at: [f64; 2]) { game::game::game_drop_throwable(&mut self.g, game::game::ThrowKind::Flash, [at[0], 0.0, at[1]]); }

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
        let mut v: Vec<GuardId> = self.g.guards.iter().copied().filter(|&gd| self.g.gd[gd].team == game::teams::Team::Hostile && self.g.gd[gd].state != GuardState::Dead).collect();
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
    /// where a defuse level's charges are, in the file's order
    pub fn charges(&self) -> Vec<[f64; 2]> { self.g.level.scenarios.as_ref().and_then(|s| s.defuse.as_ref()).map_or(Vec::new(), |d| d.devices.iter().map(|x| [x.pos[0], x.pos[2]]).collect()) }
    /// the objective row, as a HUD is given it
    pub fn objective(&self) -> String { self.view().mission.map_or(String::new(), |m| m.text) }
    /// WHAT A HUD IS TOLD of the game as it stands (`view::tick_view`: the record the page draws, asked of the game and never kept by it)
    pub fn view(&self) -> game::view::TickView { self.view_of(0) }
    /// …and what another player's HUD is told (0 = the first)
    pub fn view_of(&self, seat: usize) -> game::view::TickView { game::view::tick_view(&self.g, &game::seats::seat(&self.g, self.g.seats[seat])) }
    /// the lines on the game's log now, oldest first (it keeps the last five)
    pub fn log(&self) -> Vec<&str> { self.g.messages.iter().map(|m| m.text.as_str()).collect() }
    /// how the room clear ended and when, on the game's clock (None: it is on)
    pub fn room_clear_end(&self) -> Option<(game::roomclear::RoomClearEnd, f64)> { self.g.room_clear.end.map(|e| (e, self.g.room_clear.end_at)) }
    /// the men of the other side who are up
    pub fn hostiles(&self) -> Vec<GuardId> { let at = self.sam_at(); self.hostiles_by(at) }
    /// A ROUND FROM A MAN by combat's own call (`fire_weapon`, steady and with no spread) at a point of the floor, chest high.
    pub fn man_fires_at(&mut self, gd: GuardId, at: [f64; 2]) {
        let c = self.g.gd[gd].ch;
        let o = game::combat::FireOpts { steady: Some(true), spread: Some(0.0), ..game::combat::FireOpts::zeroed() };
        game::combat::fire_weapon(&mut self.g, c, [at[0], 1.2, at[1]], false, Some(&o));
    }
    /// A MAN GOES DOWN by combat's own call, in any of its ways (`quiet`: put out with no noise).
    pub fn falls_as(&mut self, gd: GuardId, how: game::game::DownKind, quiet: bool, by: game::game::DownBy) { game::combat::kill_guard(&mut self.g, gd, [1.0, 0.0, 0.0], Some(quiet), Some(how), Some(by), None, None); }
    /// the player of a seat takes a round's hit from the way `from_dir`, in his own seat, until he has none left (combat's `hit_player`: nobody fired it)
    pub fn seat_goes_down(&mut self, seat: usize) {
        let s = self.g.seats[seat];
        game::seats::with_seat(&mut self.g, s, &mut |g| { while !g.pl[g.player].down { game::combat::hit_player(g, [1.0, 0.0, 0.0]); } });
    }
    /// is a seat's player down
    pub fn seat_down(&mut self, seat: usize) -> bool { let s = self.g.seats[seat]; game::seats::with_seat(&mut self.g, s, &mut |g| g.pl[g.player].down) }
    /// a seat's man's character id
    pub fn seat_id(&mut self, seat: usize) -> f64 { let s = self.g.seats[seat]; game::seats::with_seat(&mut self.g, s, &mut |g| g.ch[g.pl[g.player].ch].id) }
    /// a man's character id
    pub fn id_of(&self, gd: GuardId) -> f64 { self.g.ch[self.g.gd[gd].ch].id }
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
        let h = &mut self.g.gd[gd];
        (h.hold, h.pinned, h.state, h.awareness, h.path_goal) = (true, true, state, aware, None);
        h.path.clear();
    }
    /// Every man of the other side but these is down.
    pub fn drop_all_but(&mut self, keep: &[GuardId]) {
        for gd in self.g.guards.clone() {
            if self.g.gd[gd].team != game::teams::Team::Operative && !keep.contains(&gd) && self.up(gd) { game::combat::kill_guard(&mut self.g, gd, [0.0, 0.0, 1.0], Some(true), None, None, None, None); }
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
            vents: g.level.vents.as_ref().map_or(0, |v| v.len()), daylight: g.level.daylight.is_some(),
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
    pub fn sam_blind(&self) -> bool { self.g.time < self.g.pl[self.g.player].dazzled_until }
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
    pub fn sam_light(&self) -> f64 { self.g.pl[self.g.player].visibility }
    /// how fast he is moving over the floor, metres a second
    pub fn speed(&self, gd: GuardId) -> f64 { let v = self.g.ch[self.g.gd[gd].ch].vel; (v[0] * v[0] + v[2] * v[2]).sqrt() }
    /// Can he not see: a flash or a blast has his eyes for now.
    pub fn blind(&self, gd: GuardId) -> bool { self.g.time < self.g.gd[gd].dazzled_until }

    /// The spot the rounds past him have sent him to, while he is on his way or about to go (the suppression rule's cover spot); `None`: not sent.
    pub fn sent_to_cover(&self, gd: GuardId) -> Option<[f64; 2]> { self.g.gd[gd].sup.cover.map(|p| [p[0], p[2]]) }

    /// is he down low (crouched or lower: what a man does behind cover, or with a grenade near)
    pub fn low(&self, gd: GuardId) -> bool { self.g.ch[self.g.gd[gd].ch].anim.crouch_target >= 0.5 }
    /// is he in the stack's stance (set on a door)
    pub fn stacked(&self, gd: GuardId) -> bool { self.g.ch[self.g.gd[gd].ch].anim.stance == game::character::Stance::Stack }
    /// is the man on his feet (not dead, not down)
    pub fn up(&self, gd: GuardId) -> bool { self.g.gd[gd].state != GuardState::Dead && self.g.gd[gd].down_kind.is_none() }
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
    let parts: Vec<&str> = game::state_words::STATE_NAMES.iter().zip(a[k].iter().zip(&b[k])).filter(|(_, (x, y))| x != y).map(|(n, _)| *n).collect();
    Some((k, parts.join(", ")))
}

/// metres between two places in plan
pub fn dist(a: [f64; 2], b: [f64; 2]) -> f64 { ((a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2)).sqrt() }
