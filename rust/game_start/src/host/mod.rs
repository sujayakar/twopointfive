//! WHAT A HOST DOES ROUND A STEP OF THE GAME, in one place for every host: what a player or a panel did that is no input (command), what a machine keeps (kept), the cards before a game (desk),
//! the knobs by name (knobs, shown, words), the panel's actions (actions). Nothing here touches a file or a clock.
pub mod actions;
pub mod command;
pub mod desk;
pub mod kept;
pub mod knobs;
pub mod shown;
pub mod words;
