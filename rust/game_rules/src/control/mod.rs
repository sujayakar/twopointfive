//! WHO HAS A MAN'S BODY THIS TICK: one ordered list of rules, one arbiter that asks them, one record of the answer.
//!
//! A rule that wants a man's body is a line of `rules!` below and a file of this folder. The order of the lines is the rank: the arbiter asks
//! from the top and the first rule that answers has him. What it decided is written to the man's `Control`, once a tick, so "why is he doing
//! that" is read in one place (`control_line`, `control_summary`, and the runner's CONTROL line of `control_tally`).
//!
//! A RULE IS FOUR FUNCTIONS AND A ROW, each with one job:
//!   `Row`         what the rule keeps of one man (`()` where its own module still keeps it), at `state_mut(g).rows.<rule>.at(gd)`. Made by
//!                 `Default`, cloned with the game.
//!   `transition`  moves the rule's own state for this man: a clock runs out, he arrives, a spot is found for him.
//!   `propose`     what his body should do this tick if the rule has him, or `None`. It takes `&Game`: the compiler holds that asking a rule
//!                 changes nothing.
//!   `commit`      hears what his body made of it (he arrived, he is going nowhere, the script as the walk left it).
//!   `lost`        hears that it had him last tick and does not this tick, whatever took him.
//! Today a rule's `transition` runs only when no rule above it has him, just before its `propose`, as the one function it was cut from did.
//!
//! WHAT A RULE MAY ASK FOR (`Intent`), and where in a man's frame it is asked (`Stage`):
//!   `Stage::Body`  before he senses anything. `Intent::Script`: his whole body follows a script for the tick and nothing else of his frame
//!                  runs (guards::update_guard's `lent`). Below these rules stands his own script (`gd.script`, which 71 files read and so
//!                  stays where it is), and below that his own state.
//!   `Stage::Cover` in the fight, after his senses and his state. `Intent::Cover`: he gets down where he is or goes crouched to a spot, his
//!                  gun lowered toward the danger, no round fired, his hands free to change magazine (update_guard's `alert`, its first test).
//! A rule asked at one stage answers with that stage's intent. Dead and held men are recorded from update_guard's `incapable`.
//!
//! TO ADD A RULE: one file here (its `Row` and four functions) and one line in `rules!` at its rank. Nothing else in the game names it.
//!
//! A body has three channels (`Channel`): legs, aim, hands. Every intent here takes all three; the record is kept by channel so that a rule
//! which wants only one (the fight wants aim while an order has his legs) can be granted it without a change of shape.
use crate::game::{Game, GuardScript};
use crate::ids::*;
use crate::V3;
use alloc::{format, string::String, vec::Vec};

/// what of a man a rule may be granted
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Channel { Legs, Aim, Hands }

/// where in a man's frame a rule is asked
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Stage { Body, Cover }

/// what a rule asks of a man's body for a tick
#[derive(Clone, Copy, Debug)]
pub enum Intent {
    /// his whole body follows this script
    Script(GuardScript),
    /// he takes cover in the fight
    Cover(TakeCover),
}

/// A man taking cover in a fight: down where he stands or off to a spot, crouched, his gun lowered toward the danger; he fires no round and
/// his hands may change magazine.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct TakeCover {
    /// where to go; `None`: get down where he stands
    pub to: Option<V3>,
    /// how fast he goes there
    pub pace: f64,
    /// how low: 0 standing, 1 down
    pub crouch: f64,
    /// his next round waits at least this long
    pub fire_wait: f64,
    /// where the danger is, where the rule knows it better than his own belief does
    pub threat: Option<V3>,
}

/// what his body made of an intent
#[derive(Clone, Copy, Debug)]
pub enum Outcome {
    /// the script as the walk left it (`None`: the walk ended it)
    Script(Option<GuardScript>),
    /// of `TakeCover`: he is down where he stood · he is at the spot · he has stopped getting nearer · he is on his way
    Down,
    Arrived,
    GoingNowhere,
    Moving,
}

/// a rule of the list
pub struct Rule {
    pub id: RuleId,
    pub name: &'static str,
    pub stage: Stage,
    pub transition: fn(&mut Game, GuardId, f64),
    pub propose: fn(&Game, GuardId) -> Option<Intent>,
    pub commit: fn(&mut Game, GuardId, Outcome),
    pub lost: fn(&mut Game, GuardId),
}

/// a rule's own row for each man, by his id: made by `Default` when first asked for
#[derive(Clone, Debug, Default)]
pub struct PerMan<T> { rows: Vec<T> }
impl<T: Default + Clone> PerMan<T> {
    pub fn at(&mut self, gd: GuardId) -> &mut T {
        let i = gd.ix();
        if self.rows.len() <= i { self.rows.resize(i + 1, T::default()); }
        &mut self.rows[i]
    }
    pub fn get(&self, gd: GuardId) -> Option<&T> { self.rows.get(gd.ix()) }
    /// every man's row, by his id
    pub fn iter(&self) -> impl Iterator<Item = (GuardId, &T)> { self.rows.iter().enumerate().map(|(i, r)| (GuardId::from_ix(i), r)) }
}

/// The list's own code: for each line the rule's module, its id, the table of its rows (`state_mut(g).rows.<rule>`) and its row of `RULES`.
macro_rules! rules {
    ($($m:ident: $name:literal, $stage:ident;)*) => {
        $(pub mod $m;)*
        /// a rule, by its file's name
        #[allow(non_camel_case_types)]
        #[derive(Clone, Copy, PartialEq, Eq, Debug)]
        pub enum RuleId { $($m),* }
        /// every rule's rows
        #[derive(Clone, Debug, Default)]
        pub struct Rows { $(pub $m: PerMan<$m::Row>,)* }
        pub const RULES: &[Rule] = &[$(Rule { id: RuleId::$m, name: $name, stage: Stage::$stage, transition: $m::transition, propose: $m::propose, commit: $m::commit, lost: $m::lost }),*];
    };
}

// THE LIST. The order of the lines is the rank.
rules! {
    frag: "a live grenade", Body;
    runner: "broke and ran", Body;
    morale: "shaken by a blast", Body;
    startle: "startled", Body;
    duck: "head down under fire", Cover;
}

/// who can have a man, strongest first. The rules stand between `Held` and `Script` in the list's own order, but for the rules of
/// `Stage::Cover`, which are asked of a man nobody above `Own` has.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Holder {
    /// dead: nothing moves him
    Dead,
    /// held by a man's hands (a hostage hold, a grab)
    Held,
    /// a rule of the list
    Rule(RuleId),
    /// his own script (`gd.script`): an order, a drill, a mission's staging
    Script,
    /// nobody: his own state decides (patrol, suspicious, search, alert)
    Own,
}

/// one man's record: who has each channel, since when, and which rules were asked and said no this tick
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Control {
    /// by `Channel as usize`
    pub holder: [Holder; 3],
    /// the game's clock when that channel last changed hands
    pub since: [f64; 3],
    /// a bit a rule (`RuleId as u32`): asked this tick and did not want him. A rule below the winner of its stage is not asked.
    pub declined: u32,
    /// the game's clock when the record was last written (once a tick a man)
    pub t: f64,
}
impl Control {
    pub const fn blank() -> Self { Control { holder: [Holder::Own; 3], since: [0.0; 3], declined: 0, t: -1.0 } }
}

/// WHO HOLDS A MAN TO HIS POST. `hold` (he does not leave where he stands) and `pinned` (the room fight has him this tick) are flags of the
/// man that two dozen modules write. They are written through `set_hold` and `set_pinned`, which note the writer: the module that last
/// CHANGED the flag, and when. Nothing in the tick reads the note; it is for `control_line`, and it is where a post will come to be given
/// and released by rank.
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct PostNote {
    pub hold_by: &'static str,
    pub hold_since: f64,
    pub pinned_by: &'static str,
    pub pinned_since: f64,
}

/// `g.gd[gd].hold = on`, by the module `by`
pub fn set_hold(g: &mut Game, gd: GuardId, on: bool, by: &'static str) {
    let now = g.time;
    if g.gd[gd].hold != on {
        let p = state_mut(g).post.at(gd);
        p.hold_by = by; p.hold_since = now;
    }
    g.gd[gd].hold = on;
}

/// `g.gd[gd].pinned = on`, by the module `by`
pub fn set_pinned(g: &mut Game, gd: GuardId, on: bool, by: &'static str) {
    let now = g.time;
    if g.gd[gd].pinned != on {
        let p = state_mut(g).post.at(gd);
        p.pinned_by = by; p.pinned_since = now;
    }
    g.gd[gd].pinned = on;
}

/// every man's record and every rule's rows
#[derive(Clone, Debug, Default)]
pub struct ControlState {
    men: Vec<Option<Control>>,
    /// the rules' own rows a man
    pub rows: Rows,
    /// how many man-ticks each rule has had a man: a tally for the checks, by `RuleId as usize`
    pub had: Vec<u32>,
    /// who last changed each man's post flags
    pub post: PerMan<PostNote>,
    /// who last moved each man's awareness and state (mind.rs)
    pub mind: PerMan<crate::mind::Note>,
}
impl ControlState {
    pub fn blank() -> Self { ControlState::default() }
    pub fn of(&self, gd: GuardId) -> Option<&Control> { self.men.get(gd.ix()).and_then(|c| c.as_ref()) }
    fn row(&mut self, gd: GuardId) -> &mut Control {
        let i = gd.ix();
        if self.men.len() <= i { self.men.resize(i + 1, None); }
        self.men[i].get_or_insert(Control::blank())
    }
}

/// this system's state, which the game holds for it and only this module can name (`g.ctl`, crate::owned)
pub fn state(g: &Game) -> Option<&ControlState> { g.ctl.get() }
pub fn state_mut(g: &mut Game) -> &mut ControlState { g.ctl.at() }

/// THE ARBITER, over `rules` at one stage of a man's frame: ask from the top; the first that answers has him. The caller acts on the answer
/// and writes the record (`record`).
pub fn grant_from(g: &mut Game, gd: GuardId, dt: f64, stage: Stage, rules: &[Rule]) -> Option<(RuleId, Intent)> {
    if stage == Stage::Body { state_mut(g).row(gd).declined = 0; }
    for r in rules.iter().filter(|r| r.stage == stage) {
        (r.transition)(g, gd, dt);
        if let Some(intent) = (r.propose)(g, gd) { return Some((r.id, intent)); }
        state_mut(g).row(gd).declined |= 1 << r.id as u32;
    }
    None
}

/// the arbiter over the game's list
pub fn grant(g: &mut Game, gd: GuardId, dt: f64, stage: Stage) -> Option<(RuleId, Intent)> { grant_from(g, gd, dt, stage, RULES) }

/// WRITE THE RECORD: `to` has every channel of him this tick. Once a tick a man. A rule that had him and no longer does is told (`lost`).
pub fn record(g: &mut Game, gd: GuardId, to: Holder) {
    let now = g.time;
    let c = state_mut(g).row(gd);
    let was = c.holder[0];
    let first = c.t < 0.0;
    for ch in 0..3 {
        if c.holder[ch] != to || first { c.holder[ch] = to; c.since[ch] = now; }
    }
    c.t = now;
    if let Holder::Rule(r) = to {
        let i = r as usize;
        let had = &mut state_mut(g).had;
        if had.len() <= i { had.resize(i + 1, 0); }
        had[i] += 1;
    }
    if let Holder::Rule(r) = was {
        if was != to { (RULES[r as usize].lost)(g, gd); }
    }
}

/// The end of a man's frame: where no rule and no script took him this tick, his own state had him.
pub fn settle(g: &mut Game, gd: GuardId) {
    let now = g.time;
    if state(g).and_then(|s| s.of(gd)).map_or(true, |c| c.t != now) { record(g, gd, Holder::Own); }
}

/// tell rule `r` what his body made of its intent
pub fn commit(g: &mut Game, gd: GuardId, r: RuleId, o: Outcome) { (RULES[r as usize].commit)(g, gd, o) }

/// a rule's name, as its line of the list gives it
pub fn rule_name(r: RuleId) -> &'static str { RULES[r as usize].name }

fn holder_name(h: Holder) -> &'static str {
    match h {
        Holder::Dead => "dead",
        Holder::Held => "held",
        Holder::Rule(r) => rule_name(r),
        Holder::Script => "his script",
        Holder::Own => "his own",
    }
}

/// one man's line: `Alvarez: startled for 0.4 s · said no: a live grenade, broke and ran, shaken by a blast`
pub fn control_line(g: &Game, gd: GuardId) -> String {
    let who = &g.gd[gd].callsign;
    let Some(c) = state(g).and_then(|s| s.of(gd)) else { return format!("{}: not yet updated", who) };
    let [legs, aim, hands] = c.holder;
    let mut s = if legs == aim && aim == hands {
        format!("{}: {} for {:.1} s", who, holder_name(legs), g.time - c.since[0])
    } else {
        format!("{}: legs {} · aim {} · hands {}", who, holder_name(legs), holder_name(aim), holder_name(hands))
    };
    let no: Vec<&str> = RULES.iter().filter(|r| c.declined & (1 << r.id as u32) != 0).map(|r| r.name).collect();
    if !no.is_empty() { s.push_str(" · said no: "); s.push_str(&no.join(", ")); }
    let (m, p) = (&g.gd[gd], state(g).and_then(|s| s.post.get(gd)).copied().unwrap_or_default());
    let by = |who: &'static str| if who.is_empty() { "the start" } else { who };
    if m.hold { s.push_str(&format!(" · held to his post by {} for {:.1} s", by(p.hold_by), g.time - p.hold_since)); }
    if m.pinned { s.push_str(&format!(" · pinned by {} for {:.1} s", by(p.pinned_by), g.time - p.pinned_since)); }
    s
}

/// every man of the floor, a line each
pub fn control_summary(g: &Game) -> String {
    g.guards.iter().map(|&gd| control_line(g, gd)).collect::<Vec<_>>().join("\n")
}

/// how many man-ticks each rule has had a man, a rule a clause: `a live grenade 212 · broke and ran 0 · … · head down under fire 31`
pub fn control_tally(g: &Game) -> String {
    RULES.iter().map(|r| format!("{} {}", r.name, state(g).and_then(|s| s.had.get(r.id as usize)).copied().unwrap_or(0))).collect::<Vec<_>>().join(" · ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::boxed::Box;

    fn stand() -> (Box<Game>, GuardId) {
        let mut g = Box::new(Game::blank());
        let gd = g.gd.push(crate::game::Guard::blank());
        g.guards.push(gd);
        (g, gd)
    }
    fn still(_: &mut Game, _: GuardId, _: f64) {}
    fn never_moved(_: &mut Game, _: GuardId, _: f64) { panic!("the state of a rule below the winner was moved") }
    fn no(_: &Game, _: GuardId) -> Option<Intent> { None }
    fn yes(_: &Game, _: GuardId) -> Option<Intent> { Some(Intent::Script(GuardScript { speed: Some(7.0), ..Default::default() })) }
    fn down(_: &Game, _: GuardId) -> Option<Intent> { Some(Intent::Cover(TakeCover { to: None, pace: 0.0, crouch: 1.0, fire_wait: 0.0, threat: None })) }
    fn never(_: &Game, _: GuardId) -> Option<Intent> { panic!("a rule below the winner was asked") }
    fn heard(_: &mut Game, _: GuardId, _: Outcome) {}
    fn let_go(_: &mut Game, _: GuardId) {}
    fn rule(id: RuleId, stage: Stage, propose: fn(&Game, GuardId) -> Option<Intent>) -> Rule {
        Rule { id, name: "a test's rule", stage, transition: if propose as usize == never as usize { never_moved } else { still }, propose, commit: heard, lost: let_go }
    }

    #[test]
    fn the_first_rule_that_answers_has_him_and_those_below_are_neither_asked_nor_moved() {
        let (mut g, gd) = stand();
        let rules = [rule(RuleId::frag, Stage::Body, no), rule(RuleId::runner, Stage::Body, yes), rule(RuleId::morale, Stage::Body, never)];
        match grant_from(&mut g, gd, 0.016, Stage::Body, &rules) {
            Some((RuleId::runner, Intent::Script(s))) => assert_eq!(s.speed, Some(7.0)),
            _ => panic!("the second rule should have him"),
        }
        assert_eq!(state(&g).unwrap().of(gd).unwrap().declined, 1 << RuleId::frag as u32);
    }

    #[test]
    fn a_post_flag_notes_who_last_changed_it() {
        let (mut g, gd) = stand();
        g.gd[gd].callsign = String::from("Ruiz");
        g.time = 2.0; set_hold(&mut g, gd, true, "roomfight");
        g.time = 3.0; set_hold(&mut g, gd, true, "orders");      // the same again: not a change
        g.time = 4.0; set_pinned(&mut g, gd, true, "orders");
        let p = *state(&g).unwrap().post.get(gd).unwrap();
        assert_eq!((p.hold_by, p.hold_since, p.pinned_by, p.pinned_since), ("roomfight", 2.0, "orders", 4.0));
        assert!(g.gd[gd].hold && g.gd[gd].pinned);
        g.time = 5.0; settle(&mut g, gd);
        assert_eq!(control_line(&g, gd), "Ruiz: his own for 0.0 s · held to his post by roomfight for 3.0 s · pinned by orders for 1.0 s");
        g.time = 6.0; set_hold(&mut g, gd, false, "pressure");
        assert_eq!(state(&g).unwrap().post.get(gd).unwrap().hold_by, "pressure");
        assert!(!g.gd[gd].hold);
    }

    #[test]
    fn a_stage_asks_only_its_own_rules() {
        let (mut g, gd) = stand();
        let rules = [rule(RuleId::frag, Stage::Body, never), rule(RuleId::duck, Stage::Cover, down)];
        assert!(matches!(grant_from(&mut g, gd, 0.016, Stage::Cover, &rules), Some((RuleId::duck, Intent::Cover(_)))));
        let rules = [rule(RuleId::frag, Stage::Body, no), rule(RuleId::duck, Stage::Cover, never)];
        assert!(grant_from(&mut g, gd, 0.016, Stage::Body, &rules).is_none());
    }

    #[test]
    fn the_record_is_written_once_a_tick_and_since_moves_only_when_he_changes_hands() {
        let (mut g, gd) = stand();
        g.time = 1.0; record(&mut g, gd, Holder::Rule(RuleId::duck)); settle(&mut g, gd);
        g.time = 2.0; record(&mut g, gd, Holder::Rule(RuleId::duck)); settle(&mut g, gd);
        let c = *state(&g).unwrap().of(gd).unwrap();
        assert_eq!((c.holder, c.since), ([Holder::Rule(RuleId::duck); 3], [1.0; 3]));
        g.time = 3.0; settle(&mut g, gd);
        let c = *state(&g).unwrap().of(gd).unwrap();
        assert_eq!((c.holder, c.since), ([Holder::Own; 3], [3.0; 3]));
        assert_eq!(state(&g).unwrap().had[RuleId::duck as usize], 2);
    }

    #[test]
    fn the_games_list_is_the_order_update_guard_asked_them_in() {
        let body: Vec<&str> = RULES.iter().filter(|r| r.stage == Stage::Body).map(|r| r.name).collect();
        assert_eq!(body, ["a live grenade", "broke and ran", "shaken by a blast", "startled"]);
        assert_eq!(RULES.iter().find(|r| r.stage == Stage::Cover).map(|r| r.name), Some("head down under fire"));
        for (i, r) in RULES.iter().enumerate() { assert_eq!(r.id as usize, i, "a rule's id is its place in the list"); }
    }

    #[test]
    fn a_man_the_morale_rule_drives_is_lent_its_script_and_the_line_says_so() {
        let (mut g, gd) = stand();
        g.gd[gd].callsign = String::from("Alvarez");
        g.time = 5.0;
        assert!(grant(&mut g, gd, 0.016, Stage::Body).is_none());
        let mut row = crate::morale::Mind::zeroed();
        row.act = crate::morale::MoraleAct::Hold;
        row.drive = true; row.drive_t = 5.0; row.script.speed = Some(2.5);
        g.m.morale.state_of.set(gd, row);
        match grant(&mut g, gd, 0.016, Stage::Body) {
            Some((r, Intent::Script(s))) => { assert_eq!(r, RuleId::morale); assert_eq!(s.speed, Some(2.5)); record(&mut g, gd, Holder::Rule(r)); }
            _ => panic!("the morale rule should have him"),
        }
        assert_eq!(control_line(&g, gd), "Alvarez: shaken by a blast for 0.0 s · said no: a live grenade, broke and ran");
    }
}
pub use ::game_types::control::types;
pub use types::*;
