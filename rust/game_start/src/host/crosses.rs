//! WHAT CROSSES BETWEEN A PAGE AND THE GAME beside what is drawn: the calls, the commands, the asks and the types of the answers, for the program that writes the page's half
//! (`half`) and for the stamp both halves are held to.
use super::{calls::{self, Back, CALLS}, command::COMMANDS, desk};
use alloc::{format, string::String, vec::Vec};
use game::crosses::{asked_text, Ty, Types};

/// what an ask is answered with, what a call is told, and every type those name
pub fn answers() -> (Vec<Ty>, Vec<Ty>, Types) {
    let mut t = Types::default();
    let told = CALLS.iter().filter_map(|c| match c.back { Back::Told(ty, _) => Some(ty(&mut t)), _ => None }).collect();
    let asked = desk::answers(&mut t);
    (asked, told, t)
}
/// all of it as text
pub fn text() -> String {
    let told = desk::answers(&mut Types::default());
    let asks: String = desk::ASKS.iter().zip(told).map(|(a, t)| format!("{} {t}\n", a.name)).collect();
    format!("{}\n{}\n{}{asks}\n{}", calls::text(), asked_text(COMMANDS), asked_text(desk::ASKS), answers().2.text())
}
