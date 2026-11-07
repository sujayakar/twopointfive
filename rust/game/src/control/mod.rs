//! WHO HAS A MAN'S BODY THIS TICK: one ordered list of rules, one arbiter that asks them, one record of the answer.
//!
//! A rule that wants a man's body for a tick is a row of `BODY_RULES`. The order of the rows is the rank: the arbiter asks from the top and the
//! first rule that answers has him. What it decided is written to the man's `Control`, so "why is he doing that" is read in one place
//! (`control_line`, `control_summary`).
//!
//! The list today holds the rules `update_guard` used to try by hand, in the order it tried them: a live grenade, broke and ran, shaken by a
//! blast, startled. Below them stands his own script (`gd.script`, which 71 files read and so stays where it is), and below that the man's own
//! state (patrol, suspicious, search, alert). Dead and held are noted from `update_guard`'s first lines.
//!
//! TO ADD A RULE THAT TAKES A MAN'S BODY: write its two functions in its own module and add one row to `BODY_RULES` at its rank.
//! `ask` says what his body should do this tick if the rule has him (`None`: it does not want him); `ran` hears what the body made of it
//! (`arrived`, the walk's clocks), so the rule can keep it for next tick.
//!
//! A body has three channels (`Channel`): legs, aim, hands. A rule of `BODY_RULES` takes all three. A rule of `LEGS_RULES` takes his legs
//! while his own fight keeps his aim and his hands: he gets down, or goes to a spot crouched, with his gun on the danger. It is asked where
//! `update_guard` asked whether he had his head down (in the alert branch, before anything else), and the first row there is that rule.
//!
//! TO ADD A RULE THAT TAKES A MAN'S LEGS: a file of this folder with `ask` and `ran`, and one row in `LEGS_RULES` at its rank.
use crate::game::{Game, GuardScript};
use crate::ids::*;
use crate::V3;
use alloc::{format, string::String, vec::Vec};

pub mod duck;

/// what of a man a rule may be granted
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Channel { Legs, Aim, Hands }
pub const CHANNELS: [Channel; 3] = [Channel::Legs, Channel::Aim, Channel::Hands];

/// who can have a man, strongest first. The four of `BODY_RULES` stand between `Held` and `Script` in the list's own order.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Holder {
    /// dead: nothing moves him
    Dead,
    /// held by a man's hands (a hostage hold, a grab)
    Held,
    /// row `i` of `BODY_RULES`
    Rule(u8),
    /// row `i` of `LEGS_RULES` (his legs only)
    Legs(u8),
    /// his own script (`gd.script`): an order, a drill, a mission's staging
    Script,
    /// nobody: his own state decides (patrol, suspicious, search, alert)
    Own,
}

/// a rule that can take a man's whole body for a tick
pub struct BodyRule {
    pub name: &'static str,
    /// what his body should do this tick, if this rule has him. Asked only when no rule above answered.
    pub ask: fn(&mut Game, GuardId) -> Option<GuardScript>,
    /// the script as the body left it after the tick (`None`: the run ended it)
    pub ran: fn(&mut Game, GuardId, Option<GuardScript>),
}

/// THE LIST. The order of the rows is the rank.
pub const BODY_RULES: [BodyRule; 4] = [
    BodyRule { name: "a live grenade", ask: crate::frag::frag_flee, ran: crate::frag::frag_script_ran },
    BodyRule { name: "broke and ran", ask: crate::runner::runner_script, ran: crate::runner::runner_script_ran },
    BodyRule { name: "shaken by a blast", ask: crate::morale::morale_script, ran: crate::morale::morale_script_ran },
    BodyRule { name: "startled", ask: crate::startle::startle_script, ran: crate::startle::startle_script_ran },
];

/// what a rule wants of a man's legs for a tick, while his own fight keeps his aim and his hands
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct LegsWant {
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

/// what his legs made of it
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum LegsOutcome { Down, Arrived, GoingNowhere, Moving }

/// a rule that can take a man's legs for a tick
pub struct LegsRule {
    pub name: &'static str,
    /// what his legs should do this tick, if this rule has them (`dt` the tick's length). Asked only when no rule above answered.
    pub ask: fn(&mut Game, GuardId, f64) -> Option<LegsWant>,
    pub ran: fn(&mut Game, GuardId, LegsOutcome),
}

/// THE LIST OF LEGS RULES. The order of the rows is the rank.
pub const LEGS_RULES: [LegsRule; 1] = [
    LegsRule { name: "head down under fire", ask: duck::ask, ran: duck::ran },
];

/// one man's record: who has each channel, since when, and which rules were asked and said no this tick
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Control {
    /// by `Channel as usize`
    pub holder: [Holder; 3],
    /// the game's clock when that channel last changed hands
    pub since: [f64; 3],
    /// a bit a row of `BODY_RULES`: asked this tick and did not want him. Rules below the winner are not asked.
    pub declined: u32,
    /// the same of `LEGS_RULES`
    pub legs_declined: u32,
    /// the game's clock at the last grant
    pub t: f64,
    /// who had his legs before this tick's grant, and since when: a legs rule that has them again keeps its `since`
    legs_was: (Holder, f64),
}
impl Control {
    pub const fn blank() -> Self { Control { holder: [Holder::Own; 3], since: [0.0; 3], declined: 0, legs_declined: 0, t: -1.0, legs_was: (Holder::Own, 0.0) } }
}

/// every man's record, by his id
#[derive(Clone, Debug, Default)]
pub struct ControlState {
    men: Vec<Option<Control>>,
}
impl ControlState {
    pub const fn blank() -> Self { ControlState { men: Vec::new() } }
    pub fn of(&self, gd: GuardId) -> Option<&Control> { self.men.get(gd.ix()).and_then(|c| c.as_ref()) }
}

/// what the arbiter granted
pub enum Grant {
    /// row `rule` of `BODY_RULES` has him, and this is what his body does
    Lent { rule: usize, script: GuardScript },
    /// his own script has him
    Script,
    /// nobody has him: his own state decides
    Own,
}

/// write the record: every channel to `to`
pub fn note(g: &mut Game, gd: GuardId, to: Holder, declined: u32) {
    let now = g.time;
    let i = gd.ix();
    if g.ctl.men.len() <= i { g.ctl.men.resize(i + 1, None); }
    let c = g.ctl.men[i].get_or_insert(Control::blank());
    c.legs_was = (c.holder[Channel::Legs as usize], c.since[Channel::Legs as usize]);
    c.legs_declined = 0;
    for ch in 0..3 {
        if c.holder[ch] != to || c.t < 0.0 { c.holder[ch] = to; c.since[ch] = now; }
    }
    c.declined = declined;
    c.t = now;
}

/// THE ARBITER, over `rules`: ask from the top; the first that answers has him.
pub fn grant_from(g: &mut Game, gd: GuardId, rules: &[BodyRule]) -> Grant {
    let mut declined = 0u32;
    for (i, r) in rules.iter().enumerate() {
        if let Some(script) = (r.ask)(g, gd) {
            note(g, gd, Holder::Rule(i as u8), declined);
            return Grant::Lent { rule: i, script };
        }
        declined |= 1 << i;
    }
    if g.gd[gd].script.is_some() { note(g, gd, Holder::Script, declined); return Grant::Script; }
    note(g, gd, Holder::Own, declined);
    Grant::Own
}

/// the arbiter over the game's list
pub fn grant(g: &mut Game, gd: GuardId) -> Grant { grant_from(g, gd, &BODY_RULES) }

/// THE ARBITER OF A MAN'S LEGS, over `rules`: ask from the top; the first that answers has his legs this tick. For a man `grant` left his own.
pub fn grant_legs_from(g: &mut Game, gd: GuardId, dt: f64, rules: &[LegsRule]) -> Option<(usize, LegsWant)> {
    let mut declined = 0u32;
    let mut won = None;
    for (i, r) in rules.iter().enumerate() {
        if let Some(want) = (r.ask)(g, gd, dt) { won = Some((i, want)); break; }
        declined |= 1 << i;
    }
    let now = g.time;
    if let Some(c) = g.ctl.men.get_mut(gd.ix()).and_then(|c| c.as_mut()) {
        c.legs_declined = declined;
        if let Some((i, _)) = won {
            let to = Holder::Legs(i as u8);
            let l = Channel::Legs as usize;
            c.since[l] = if c.legs_was.0 == to { c.legs_was.1 } else { now };
            c.holder[l] = to;
        }
    }
    won
}

/// the arbiter of a man's legs over the game's list
pub fn grant_legs(g: &mut Game, gd: GuardId, dt: f64) -> Option<(usize, LegsWant)> { grant_legs_from(g, gd, dt, &LEGS_RULES) }

fn holder_name(h: Holder) -> &'static str {
    match h {
        Holder::Dead => "dead",
        Holder::Held => "held",
        Holder::Rule(i) => BODY_RULES.get(i as usize).map_or("a rule", |r| r.name),
        Holder::Legs(i) => LEGS_RULES.get(i as usize).map_or("a rule", |r| r.name),
        Holder::Script => "his script",
        Holder::Own => "his own",
    }
}

/// one man's line: `Alvarez: startled for 0.4 s · said no: a live grenade, broke and ran, shaken by a blast`
pub fn control_line(g: &Game, gd: GuardId) -> String {
    let who = &g.gd[gd].callsign;
    let Some(c) = g.ctl.of(gd) else { return format!("{}: not yet updated", who) };
    let [legs, aim, hands] = c.holder;
    let mut s = if legs == aim && aim == hands {
        format!("{}: {} for {:.1} s", who, holder_name(legs), g.time - c.since[0])
    } else {
        format!("{}: legs {} · aim {} · hands {}", who, holder_name(legs), holder_name(aim), holder_name(hands))
    };
    let mut no: Vec<&str> = BODY_RULES.iter().enumerate().filter(|(i, _)| c.declined & (1 << i) != 0).map(|(_, r)| r.name).collect();
    no.extend(LEGS_RULES.iter().enumerate().filter(|(i, _)| c.legs_declined & (1 << i) != 0).map(|(_, r)| r.name));
    if !no.is_empty() { s.push_str(" · said no: "); s.push_str(&no.join(", ")); }
    s
}

/// every man of the floor, a line each
pub fn control_summary(g: &Game) -> String {
    g.guards.iter().map(|&gd| control_line(g, gd)).collect::<Vec<_>>().join("\n")
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
    fn no(_: &mut Game, _: GuardId) -> Option<GuardScript> { None }
    fn yes(_: &mut Game, _: GuardId) -> Option<GuardScript> { Some(GuardScript { speed: Some(7.0), ..Default::default() }) }
    fn never(_: &mut Game, _: GuardId) -> Option<GuardScript> { panic!("a rule below the winner was asked") }
    fn ran(_: &mut Game, _: GuardId, _: Option<GuardScript>) {}

    #[test]
    fn the_first_rule_that_answers_has_him_and_those_below_are_not_asked() {
        let (mut g, gd) = stand();
        g.time = 3.0;
        let rules = [BodyRule { name: "a", ask: no, ran }, BodyRule { name: "b", ask: yes, ran }, BodyRule { name: "c", ask: never, ran }];
        match grant_from(&mut g, gd, &rules) {
            Grant::Lent { rule, script } => { assert_eq!(rule, 1); assert_eq!(script.speed, Some(7.0)); }
            _ => panic!("rule b should have him"),
        }
        let c = *g.ctl.of(gd).unwrap();
        assert_eq!(c.holder, [Holder::Rule(1); 3]);
        assert_eq!(c.declined, 0b1);
        assert_eq!(c.since, [3.0; 3]);
    }

    #[test]
    fn his_script_stands_below_the_rules_and_his_own_state_below_that() {
        let (mut g, gd) = stand();
        let rules = [BodyRule { name: "a", ask: no, ran }];
        assert!(matches!(grant_from(&mut g, gd, &rules), Grant::Own));
        assert_eq!(g.ctl.of(gd).unwrap().holder, [Holder::Own; 3]);
        g.gd[gd].script = Some(GuardScript::default());
        g.time = 1.0;
        assert!(matches!(grant_from(&mut g, gd, &rules), Grant::Script));
        let c = *g.ctl.of(gd).unwrap();
        assert_eq!((c.holder, c.since, c.declined), ([Holder::Script; 3], [1.0; 3], 0b1));
    }

    #[test]
    fn since_moves_only_when_a_channel_changes_hands() {
        let (mut g, gd) = stand();
        let rules = [BodyRule { name: "a", ask: yes, ran }];
        g.time = 1.0; let _ = grant_from(&mut g, gd, &rules);
        g.time = 2.0; let _ = grant_from(&mut g, gd, &rules);
        assert_eq!(g.ctl.of(gd).unwrap().since, [1.0; 3]);
        g.time = 3.0; let _ = grant_from(&mut g, gd, &[]);
        assert_eq!(g.ctl.of(gd).unwrap().since, [3.0; 3]);
    }

    fn legs_no(_: &mut Game, _: GuardId, _: f64) -> Option<LegsWant> { None }
    fn legs_yes(_: &mut Game, _: GuardId, _: f64) -> Option<LegsWant> { Some(LegsWant { to: None, pace: 0.0, crouch: 1.0, fire_wait: 0.0, threat: None }) }
    fn legs_ran(_: &mut Game, _: GuardId, _: LegsOutcome) {}

    #[test]
    fn a_legs_rule_takes_his_legs_and_leaves_his_aim_and_keeps_its_since_from_tick_to_tick() {
        let (mut g, gd) = stand();
        g.gd[gd].callsign = String::from("Ruiz");
        let rules = [LegsRule { name: "x", ask: legs_no, ran: legs_ran }, LegsRule { name: "y", ask: legs_yes, ran: legs_ran }];
        g.time = 1.0;
        assert!(matches!(grant_from(&mut g, gd, &[]), Grant::Own));
        assert_eq!(grant_legs_from(&mut g, gd, 0.016, &rules).map(|w| w.0), Some(1));
        g.time = 2.0;
        assert!(matches!(grant_from(&mut g, gd, &[]), Grant::Own));
        assert_eq!(grant_legs_from(&mut g, gd, 0.016, &rules).map(|w| w.0), Some(1));
        let c = *g.ctl.of(gd).unwrap();
        assert_eq!(c.holder, [Holder::Legs(1), Holder::Own, Holder::Own]);
        assert_eq!((c.since[0], c.legs_declined), (1.0, 0b1));
        g.time = 3.0;
        assert!(matches!(grant_from(&mut g, gd, &[]), Grant::Own));
        assert!(grant_legs_from(&mut g, gd, 0.016, &[]).is_none());
        let c = *g.ctl.of(gd).unwrap();
        assert_eq!((c.holder[0], c.since[0]), (Holder::Own, 3.0));
    }

    #[test]
    fn the_games_list_is_the_order_update_guard_tried_them_in() {
        let names: Vec<&str> = BODY_RULES.iter().map(|r| r.name).collect();
        assert_eq!(names, ["a live grenade", "broke and ran", "shaken by a blast", "startled"]);
    }

    #[test]
    fn a_man_the_morale_rule_drives_is_lent_its_script_and_the_line_says_so() {
        let (mut g, gd) = stand();
        g.gd[gd].callsign = String::from("Alvarez");
        g.time = 5.0;
        assert!(matches!(grant(&mut g, gd), Grant::Own));
        let mut row = crate::morale::Mind::zeroed();
        row.act = crate::morale::MoraleAct::Hold;
        row.drive = true; row.drive_t = 5.0; row.script.speed = Some(2.5);
        g.m.morale.state_of.set(gd, row);
        match grant(&mut g, gd) {
            Grant::Lent { rule, script } => { assert_eq!(BODY_RULES[rule].name, "shaken by a blast"); assert_eq!(script.speed, Some(2.5)); }
            _ => panic!("the morale rule should have him"),
        }
        assert_eq!(control_line(&g, gd), "Alvarez: shaken by a blast for 0.0 s · said no: a live grenade, broke and ran");
    }
}
