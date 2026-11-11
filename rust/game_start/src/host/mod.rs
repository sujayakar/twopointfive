//! WHAT A HOST DOES ROUND A STEP OF THE GAME, in one place for every host: what a player or a panel did that is no input (command), what a machine keeps (kept), the cards before a game (desk),
//! the knobs by name (knobs, shown, words), the panel's actions (actions); and the list of what crosses (crosses). Nothing here touches a file or a clock.
pub mod actions;
pub mod command;
pub mod crosses;
pub mod desk;
pub mod kept;
pub mod knobs;
pub mod shown;
pub mod words;

use game::difficulty::{self, Difficulty, DifficultyAsked};
use game::game::Game;
use game::query::Query;
use kept::Kept;

/// WHAT A HOST HANDS A START, ON THE BLANK GAME, BEFORE ANYTHING IS READ. The one place a game is laid what a machine keeps and what its address says, and the address is always over what is kept.
/// In order: the kept rows and picks, the address's numbers, the difficulty, the levers, the cards, the quiet rung and cameras.
pub fn lay(g: &mut Game, kept: &Kept, address: Option<&str>) {
    let words = address.unwrap_or("");
    let q = Query::parse(words);
    for (name, v) in &kept.rows {
        let Some(r) = shown::ROWS.iter().find(|r| r.kept && r.name == name) else { continue };
        knobs::set(g, r.name, v.max(r.lo).min(r.hi) / r.scale);
    }
    for (name, word) in &kept.picks { actions::act(g, name, 0.0, word); }
    words::apply(g, words);
    // A lesson is taught on the standard rules, whatever this machine keeps, and its address reads no lever. A lever kept or said stands over the level, and the level the address says is the level
    // itself: it is laid first and takes no kept lever with it.
    let lesson = game::scenario_run::lesson_of_address(&q).is_some();
    let level = difficulty::difficulty_asked(g, &q, lesson).and_then(|a| a.level);
    let hard = level.map_or(kept.difficulty == Some(true) && (!lesson || q.has("lessonhard")), |l| l == Difficulty::Hard);
    if hard { difficulty::set_difficulty(g, Difficulty::Hard, Some(false)); }
    if !lesson { for (key, rung) in &kept.levers { difficulty::set_lever(g, *key, if rung.is_empty() { None } else { Some(rung) }, Some(false), None); } }
    if let Some(a) = difficulty::difficulty_asked(g, &q, lesson) { difficulty::difficulty_lay(g, &DifficultyAsked { level: None, ..a }); }
    if let Some(cards) = kept.cards.as_deref().filter(|_| !q.has("nokit")) { kept::read(g, cards); }
    if let Some(rung) = &kept.quiet_fog { game::quietfog::quiet_fog_kept(g, rung); }
    if let Some(on) = kept.quiet_cams { game::quietfog::quiet_cams_kept(g, on); }
}
