//! WHAT CROSSES BETWEEN A PAGE AND THE GAME beside what is drawn: the commands, the asks and the types of the answers, for the program that writes the page's half
//! (`half`) and for the stamp both halves are held to.
use super::{command::COMMANDS, desk};
use alloc::{format, string::String};
use game::crosses::{asked_text, Crosses, Types};
use game::view::{CursorView, DebriefView, RunView, TickView};

/// every type an answer is of
pub fn types() -> Types {
    let mut t = Types::default();
    for ty in [TickView::ty, CursorView::ty, DebriefView::ty, RunView::ty] { ty(&mut t); }
    desk::answers(&mut t);
    t
}
/// all of it as text
pub fn text() -> String {
    let answers = desk::answers(&mut Types::default());
    let asks: String = desk::ASKS.iter().zip(answers).map(|(a, t)| format!("{} {t}\n", a.name)).collect();
    format!("{}\n{}{asks}\n{}", asked_text(COMMANDS), asked_text(desk::ASKS), types().text())
}
