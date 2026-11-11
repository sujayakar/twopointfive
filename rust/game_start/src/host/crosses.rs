//! WHAT CROSSES BETWEEN A PAGE AND THE GAME beside what is drawn: the commands, the asks and the types of the answers, for the program that writes the page's half
//! (`half`) and for the stamp both halves are held to.
use super::{command::COMMANDS, desk};
use alloc::{format, string::String, vec::Vec};
use game::crosses::{asked_text, Crosses, Ty, Types};
use game::view::{CursorView, DebriefView, RunView, TickView};

/// what an answer is of, and every type that names
pub fn answers() -> (Vec<Ty>, Types) {
    let mut t = Types::default();
    let mut whole: Vec<Ty> = [TickView::ty, CursorView::ty, DebriefView::ty, RunView::ty].into_iter().map(|ty| ty(&mut t)).collect();
    whole.extend(desk::answers(&mut t));
    (whole, t)
}
/// all of it as text
pub fn text() -> String {
    let told = desk::answers(&mut Types::default());
    let asks: String = desk::ASKS.iter().zip(told).map(|(a, t)| format!("{} {t}\n", a.name)).collect();
    format!("{}\n{}{asks}\n{}", asked_text(COMMANDS), asked_text(desk::ASKS), answers().1.text())
}
