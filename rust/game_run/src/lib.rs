//! WHAT A HOST DOES ROUND A STEP OF THE GAME, in one place for every host: what a player or a panel did that is no input (command), what a machine keeps (kept), the cards before a game (desk),
//! the knobs by name (knobs, shown, words), the panel's actions (actions), a player of nobody's (hands_off), and the two steps below. Nothing here touches a file or a clock.
pub mod actions;
pub mod command;
pub mod desk;
pub mod kept;
pub mod knobs;
pub mod shown;
pub mod words;

/// A SAVE IS ABOUT TO BE KEPT, under this label: the chronicle's row, the tick's last thing. A picture of a game touches nothing else of it: a level's program goes on through a take, as a beat's place is
/// plain data the picture holds.
pub fn checkpoint(g: &mut game::game::Game, label: &str) { let _ = game::chronicle::chron_checkpoint(g, game::chronicle::ChronCheckpointWhat::Save, Some(label)); }

/// AFTER A TICK: a tutorial mission staged again opens on its first view, set with the host's camera; a return on its first line off a rails level. The host's camera is the row the game was handed at its
/// stand: a game that was handed none is refused, and nothing is guessed.
pub fn view(g: &mut game::game::Game) {
    let cam = g.host_cam.expect("the game was handed no host camera at its stand (game_start host_new)");
    game::rails::rails_host_view(g, cam);
}
