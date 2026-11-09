//! The numbers of squaddeal.
use crate::bytes::ToBytes;
use alloc::string::String;
use alloc::vec::Vec;
use game_derive::Knobs;

/// the switch and its numbers: on (?nosquaddeal = a level with no squads block is one pool,
///  as it was) and by the shipped stand; the harness's frozen stand leaves it off (TPS_SQUADDEAL=1 turns it on). UNTUNED by play.
#[derive(Clone, Debug, ToBytes, Knobs)]
pub struct SquaddealK {
    /// the most men that are ONE squad · the most that are two (more: three)
    #[k(5.0)] pub one: f64,
    #[k(8.0)] pub two: f64,
    /// what their radio calls them, in order along the house
    #[k(alloc::vec![String::from("Alpha"), String::from("Bravo"), String::from("Charlie")])] pub names: Vec<String>,
}
