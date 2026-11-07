//! The types of `view`: what a machine is shown of the game beside what it draws (rust/game_rules/src/view).
#![allow(unused_imports)]
use crate::{ids::*, js::*, Quat, V3};
use alloc::{string::String, vec::Vec};

/// whose words the mission's line is
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum MissionOf {
    /// the stealth mission's own stages
    Stages,
    /// a level's program while it runs
    Script,
    Lesson,
    Defuse,
    Snatch,
    Roomclear,
    Rescue,
    Nightbreach,
}

/// the mission's one line, and the place its marker stands on
#[derive(Clone, Debug)]
pub struct MissionView {
    pub of: crate::view::MissionOf,
    pub text: String,
    pub pos: Option<V3>,
}
impl Default for MissionView { fn default() -> Self { MissionView { of: MissionOf::Stages, text: String::new(), pos: None } } }

/// a tutorial mission's one big line
#[derive(Clone, Debug, Default)]
pub struct RailPrompt {
    pub line: String,
    /// which step's pip is lit (from 1), of how many
    pub n: u32,
    pub of: u32,
    /// the few words under the line after a tap the step refused
    pub note: Option<String>,
    /// it asks for nothing: what the men are doing, or a step that waits on the level's script. No mark to press on
    pub watch: bool,
}

/// a tutorial step's card: why, where the line says what
#[derive(Clone, Debug, Default)]
pub struct RailCard {
    pub title: Option<String>,
    pub text: String,
}

/// what a tutorial mission's frame lets the player do and tells him, for the step that is up
#[derive(Clone, Debug, Default)]
pub struct RailsView {
    /// (None: the run is over)
    pub prompt: Option<crate::view::RailPrompt>,
    pub card: Option<crate::view::RailCard>,
    /// the place the step marks: a cursor on it is on the place, whatever stands between
    pub mark: Option<V3>,
    /// the step's mark is an arrow, pointing here
    pub arrow: Option<V3>,
    /// the player's own men on the floor, by their characters' ids: the big line keeps clear of them
    pub line_men: Vec<f64>,
    /// the actions that work now (rails.ts ACTS). An action not here is locked, and the HUD names no key of it
    pub open: Vec<String>,
    /// the verbs the interaction key offers rows of. (None: every row)
    pub rows: Option<Vec<String>>,
    /// the only orders on the squad's menu. (None: every order)
    pub orders: Option<Vec<String>>,
    /// a step has taken the held click off him: nothing says he can stalk a man
    pub stalk_shut: bool,
    /// the step teaches the squad's key held: the key's tap is not offered
    pub teaches_hold: bool,
    /// no teammate shows an aiming laser on this mission
    pub no_lasers: bool,
    /// nobody on the floor is the player: where the view looks. (None: he is on the floor)
    pub sky: Option<V3>,
}

/// the lesson that is live on the mission that is up
#[derive(Clone, Debug, Default)]
pub struct LessonView {
    /// the step that is up, from 0, and how many there are
    pub step: u32,
    pub of: u32,
    /// its id. (None: every step is done)
    pub id: Option<String>,
    /// every step is done
    pub finished: bool,
    /// the run is over, whichever way
    pub over: bool,
    /// the step's deed is done and the moment is playing out
    pub settling: bool,
    /// the step asks him to move the view: nothing else moves it meanwhile
    pub asks_view: bool,
    /// (None: a lesson with no frame over it)
    pub rails: Option<crate::view::RailsView>,
}

/// WHAT A SEAT'S HUD IS TOLD OF THE GAME AS A TICK LEFT IT: a section for each kind of thing a HUD words. Nothing in it is kept by the game.
#[derive(Clone, Debug, Default)]
pub struct TickView {
    /// (None: a show runs)
    pub mission: Option<crate::view::MissionView>,
    /// (None: no lesson is live)
    pub lesson: Option<crate::view::LessonView>,
}
