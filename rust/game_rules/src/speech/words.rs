use crate::game::Game;

/// A line (or any value a line is picked by) as the dialogue pass has it; `on` is settled true on the page.
pub fn w<T: Clone + core::fmt::Debug>(g: &mut crate::game::Game, now: &T) -> T { now.clone() }

/// A line as the writing guide has it; `on` and `guide` are settled true on the page.
pub fn wg<T: Clone + core::fmt::Debug>(g: &crate::game::Game, now: &T, was: &T) -> T { now.clone() }
