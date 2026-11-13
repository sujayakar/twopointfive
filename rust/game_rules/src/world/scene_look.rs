pub use ::game_types::scene_look::types;
pub use types::*;
use crate::game::Game;

/// The mission a host asked the men to be dressed for.
// `'quiet' | null | undefined` is Option<Option<SuitAs>>: None = nobody asked, Some(None) = no mission. The module's own row can still say only asked or not.)
pub fn suit_as(g: &Game) -> Option<Option<SuitAs>> { g.m.scene_look.suit_asked.map(|k| match k { SceneLookModSuitAsked::Quiet => Some(SuitAs::Quiet), SceneLookModSuitAsked::NoMission => None }) }
/// Do the men wear the look's dress here.
pub fn look_dress(g: &Game) -> bool { g.m.scene_look.look_place }
/// A place that is not the look's.
pub fn look_office(g: &Game) -> bool { !g.m.scene_look.look_place }
