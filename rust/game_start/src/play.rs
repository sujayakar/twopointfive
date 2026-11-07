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
}
impl Default for Opts { fn default() -> Self { Opts { seed: 1, squad: 4, cpu_senses: true } } }

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
    /// each man's last bubble heard: when it was said, and its words
    spoke: Vec<(GuardId, f64, String)>,
    /// where the hand's cursor points in the world, kept there as the camera moves (None: where the host left it)
    cursor: Option<[f64; 3]>,
    _one: MutexGuard<'static, ()>,
}

fn start(data: &Json, room_clear: bool, o: &Opts) -> Play {
    // (a second game stood by the thread that holds the first would wait on itself for ever: said, not waited for)
    if HOLDS.with(|h| h.get()) { panic!("one game at a time: this test still holds a Play. Drop it (`drop(p)`, or end its block) before standing another"); }
    // (a test that failed while it held the lock left it poisoned and its game dead: the next one stands on a block zeroed anew)
    let one = ONE_GAME.lock().unwrap_or_else(|p| p.into_inner());
    let ch = CHARACTER.get_or_init(files::CharacterFiles::read);
    let scenario = if room_clear { Some(game::scene_scenario_list::ScenarioSpec { kind: game::scene_scenario_list::ScenarioSpecKind::Roomclear, squad: o.squad as f64, level: None, seed: None }) } else { None };
    let Stood { mut game, host, .. } = stand::stand(data, ch.assets(), &StartOpts { seed: o.seed, scenario, cpu_senses: o.cpu_senses, ..Default::default() });
    game.out.clear();
    HOLDS.with(|h| h.set(true));   // (only once a Play exists to put it down: a stand that panicked holds nothing)
    let mut p = Play { g: game, host, frames: 0, said: Vec::new(), sounds: Vec::new(), cursor: None, spoke: Vec::new(), _one: one };
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
        for e in out { if let E::GameAudioLikePlay(p) = e { self.sounds.push(Sound { t, name: p.name.clone(), at: p.pos }); } }
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

    /// A live fragmentation grenade lands at `at` (the sandbox's own call, `Game.dropThrowable`: an armed can as if just thrown from where Sam stands).
    pub fn frag_lands(&mut self, at: [f64; 2]) { game::game::game_drop_throwable(&mut self.g, game::game::ThrowKind::Frag, [at[0], 0.0, at[1]]); }
    /// how long a thrown grenade's fuse is, seconds
    pub fn frag_fuse(&self) -> f64 { self.g.k.frag.fuse }
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
    /// Sam's teammates, in the order the game made them
    pub fn mates(&self) -> Vec<GuardId> { self.g.guards.iter().copied().filter(|&gd| self.g.gd[gd].team == game::teams::Team::Operative).collect() }
    /// every man of the game's list in a line, for a failure's message
    pub fn roll(&self) -> String {
        self.g.guards.iter().map(|&gd| { let o = &self.g.gd[gd]; let p = self.at(gd); format!("{} {:?} {:?} at ({:.1}, {:.1})", o.callsign, o.team, o.state, p[0], p[1]) }).collect::<Vec<_>>().join("; ")
    }

    // ---- what a man is doing, read off the game

    /// where a man stands, in plan
    pub fn at(&self, gd: GuardId) -> [f64; 2] { let p = self.g.ch[self.g.gd[gd].ch].pos; [p[0], p[2]] }
    /// where Sam stands
    pub fn sam_at(&self) -> [f64; 2] { let p = self.g.ch[self.g.pl[self.g.player].ch].pos; [p[0], p[2]] }
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
    /// is he down low (crouched or lower: what a man does behind cover, or with a grenade near)
    pub fn low(&self, gd: GuardId) -> bool { self.g.ch[self.g.gd[gd].ch].anim.crouch_target >= 0.5 }
    /// is he in the stack's stance (set on a door)
    pub fn stacked(&self, gd: GuardId) -> bool { self.g.ch[self.g.gd[gd].ch].anim.stance == game::character::Stance::Stack }
    /// is the man on his feet (not dead, not down)
    pub fn up(&self, gd: GuardId) -> bool { self.g.gd[gd].state != GuardState::Dead && self.g.gd[gd].down_kind.is_none() }
}

impl Drop for Play { fn drop(&mut self) { HOLDS.with(|h| h.set(false)); } }

/// metres between two places in plan
pub fn dist(a: [f64; 2], b: [f64; 2]) -> f64 { ((a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2)).sqrt() }
