//! What a mission can be played with, as the screen before it asks (loadout.ts squadChoices, noteOf, rosterOf, the notes of a kit that is not yours).
use crate::game::{Game, PairId};
use crate::scene_scenarios::{MissionGroup, ScenarioEntry, ScenarioEntryKind};
use alloc::{format, string::String, vec::Vec};

/// The kinds of mission the screen prepares: a scenario's, or the plain night, which stages nobody.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LoadoutKind { Nightbreach, Roomclear, Rescue, Quiet, Solo }

/// The mission the screen prepares: a scenario's row, or the solo night's stand-in.
#[derive(Clone, Debug)]
pub struct LoadoutEntry {
    pub id: String,
    pub kind: LoadoutKind,
    pub squad: f64,
    pub level: Option<String>,
    pub group: Option<MissionGroup>,
    pub only: Option<String>,
}

/// The solo night's place on the list of missions.
const SOLO_ID: &str = "solo-night";

impl From<ScenarioEntry> for LoadoutEntry {
    fn from(e: ScenarioEntry) -> LoadoutEntry {
        let kind = match e.kind {
            ScenarioEntryKind::Nightbreach => LoadoutKind::Nightbreach,
            ScenarioEntryKind::Roomclear => LoadoutKind::Roomclear,
            ScenarioEntryKind::Rescue => LoadoutKind::Rescue,
            ScenarioEntryKind::Quiet => LoadoutKind::Quiet,
        };
        LoadoutEntry { id: e.id, kind, squad: e.squad, level: e.level, group: e.group, only: e.only }
    }
}

/// A mission by its id, off the game's own list.
pub fn entry_of(id: &str) -> Option<LoadoutEntry> {
    if id == SOLO_ID {
        return Some(LoadoutEntry { id: String::from(SOLO_ID), kind: LoadoutKind::Solo, squad: 0.0, level: None, group: None, only: None });
    }
    crate::scene_scenario_list::all_missions().into_iter().find(|e| e.id == id).map(LoadoutEntry::from)
}

/// One size as a mission can (or cannot) stage it.
#[derive(Clone, Debug)]
pub struct SquadChoice {
    pub n: f64,
    pub ok: bool,
    /// the mission's own size: what its row loads with nothing changed
    pub own: bool,
    /// why it is greyed (ok false), or what is different about it (ok true; empty: nothing to say)
    pub note: String,
}

const SIZES: [f64; 5] = [0.0, 1.0, 2.0, 3.0, 4.0];
const SOLO_ONLY: &str = "The solo night is yours alone: nobody else is on the roster.";
const QUIET_ONLY: &str = "A quiet mission is for one or two: you, or you and one partner.";
const LESSON_ONLY: &str = "A tutorial mission is played at its own size.";
const LESSON_KIT: &str = "A lesson is played with the kit as issued.";
const MISSION_KIT: &str = "This mission issues its own kit, whatever this machine keeps";

fn is_lesson(e: &LoadoutEntry) -> bool { e.group == Some(MissionGroup::Tutorial) }

/// Which sizes a mission can stage, in the order of the sizes.
pub(in crate::outfit) fn squad_choices(e: &LoadoutEntry) -> Vec<SquadChoice> {
    SIZES.iter().map(|&n| {
        let no = if is_lesson(e) && n != e.squad { LESSON_ONLY }
            else if let Some(only) = e.only.as_deref().filter(|_| n != e.squad) { only }
            else if e.kind == LoadoutKind::Solo && n > 0.0 { SOLO_ONLY }
            else if e.kind == LoadoutKind::Quiet && n > 1.0 { QUIET_ONLY }
            else { "" };
        SquadChoice { n, ok: no.is_empty(), own: n == e.squad, note: if no.is_empty() { String::from(note_of(e.kind, n, e.level.is_none(), e.squad)) } else { String::from(no) } }
    }).collect()
}

/// What a size changes on a kind of mission; `office`: the bundled level, the one with a breach plan.
fn note_of(kind: LoadoutKind, n: f64, office: bool, own: f64) -> &'static str {
    use LoadoutKind::{Quiet, Rescue, Solo};
    if kind == Solo { return "Alone on the office floor."; }
    if kind == Quiet {
        if n == 0.0 { return "Alone: your own kit, and nobody to cover a door."; }
        return if n == 1.0 { "One partner, silent from the start: he follows low in your steps and takes a man on your mark (X on the man, then F)." } else { "" };
    }
    let rescue = kind == Rescue;
    if n == 0.0 { return if rescue { "Alone: nobody to stack or flow with you. The clock is the same." } else { "Alone: no orders to give. Every room is yours to take." }; }
    if n == 1.0 { return if rescue { "One man: he takes one door on your F; nobody for the side door." } else if office { "One man: no F drill (it takes two). He takes your orders with X." } else { "One man: he takes your orders with X." }; }
    if n == 3.0 { return if rescue { "Three: the red pair and one blue man." } else { "Three: the red pair and one blue man. Orders go to all, a pair or one man." }; }
    if n == 4.0 && rescue { return if own == 4.0 { "Four: a red pair and a blue pair. A pair for each door." } else { "Four: a red pair and a blue pair. Nobody has balanced the rescue for four." }; }
    if n == 2.0 && rescue && own == 4.0 { return "The pair: a man for each door. This floor was laid out for four."; }
    ""
}

/// One teammate as a size stages him.
#[derive(Clone, Copy, Debug)]
pub struct RosterMan { pub name: &'static str, pub pair: PairId }

const PAIR: [RosterMan; 2] = [RosterMan { name: "Blue One", pair: PairId::Blue }, RosterMan { name: "Blue Two", pair: PairId::Blue }];
const FOUR: [RosterMan; 4] = [
    RosterMan { name: "Red One", pair: PairId::Red },
    RosterMan { name: "Red Two", pair: PairId::Red },
    RosterMan { name: "Blue One", pair: PairId::Blue },
    RosterMan { name: "Blue Two", pair: PairId::Blue },
];

/// The men of a size, in staging order: up to two are the pair, three or four the four-man squad's first three or all of it.
pub(in crate::outfit) fn roster_of(n: f64) -> &'static [RosterMan] {
    let n = n as usize;
    if n > 2 { &FOUR[..n] } else { &PAIR[..n] }
}

/// A size's note with no kit of his own in it.
fn not_yours(note: &str) -> String { note.replacen("your own kit, and nobody", "nobody", 1) }

fn joined(note: String, then: &str) -> String { if note.is_empty() { String::from(then) } else { format!("{note} {then}") } }

/// A size's note on a lesson played as issued.
pub(in crate::outfit) fn lesson_note(note: &str) -> String { joined(not_yours(note), LESSON_KIT) }

/// A size's note on a mission that issues its own kit, with who carries what.
pub fn issue_note(note: &str, who: &str) -> String {
    let said = if who.is_empty() { format!("{MISSION_KIT}.") } else { format!("{MISSION_KIT}: {who}.") };
    joined(not_yours(note), &said)
}

/// The row is played with the kit as issued, whatever this machine keeps: a lesson.
pub fn kit_as_issued(e: &LoadoutEntry) -> bool { is_lesson(e) }

/// The five sizes a screen with no mission offers: all of them, with nothing to say.
pub(in crate::outfit) fn any_size() -> Vec<SquadChoice> { SIZES.iter().map(|&n| SquadChoice { n, ok: true, own: false, note: String::new() }).collect() }
