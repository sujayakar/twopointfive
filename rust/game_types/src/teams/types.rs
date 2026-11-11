//! The types of `teams`.
use crate::{Words, bytes::ToBytes};

/// Whose man a body is. Two men are enemies when one is ours and the other the place's own or a raider's; a bystander is nobody's enemy.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, ToBytes, Default, Words)]
pub enum Team {
    /// the place's own men, and whom they call
    #[default] Guard,
    /// the players and their squad
    #[word("operative")] Ours,
    Hostile,
    Civilian,
}

impl Team {
    /// against the players
    pub const fn theirs(self) -> bool { matches!(self, Team::Guard | Team::Hostile) }
    pub const fn ours(self) -> bool { matches!(self, Team::Ours) }
    pub const fn foe_of(self, other: Team) -> bool { (self.ours() && other.theirs()) || (self.theirs() && other.ours()) }
}
