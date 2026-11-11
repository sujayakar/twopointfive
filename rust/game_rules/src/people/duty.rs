//! WHAT A MAN IS HELD TO: one record a man (`Duty`, on his row), whoever he is and whoever gave it him, and the ways it is written.
pub use ::game_types::duty::types;
pub use types::*;
use crate::game::Game;
use crate::ids::*;

/// He is held where he is, or let go, by the module `by`.
pub fn hold(g: &mut Game, gd: GuardId, on: bool, by: &'static str) {
    let (now, d) = (g.time, &mut g.gd[gd].duty);
    if d.holds != on { d.holds_by = Giver { by, since: now }; }
    d.holds = on;
}

/// He fights from where he is held, or may close, by the module `by`.
pub fn pin(g: &mut Game, gd: GuardId, on: bool, by: &'static str) {
    let (now, d) = (g.time, &mut g.gd[gd].duty);
    if d.pinned != on { d.pinned_by = Giver { by, since: now }; }
    d.pinned = on;
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::boxed::Box;

    #[test]
    fn a_word_of_it_notes_who_last_changed_it() {
        let mut g = Box::new(Game::blank());
        let gd = g.gd.push(crate::game::Guard::blank());
        assert!(g.gd[gd].duty.free());
        g.time = 2.0; hold(&mut g, gd, true, "roomfight");
        g.time = 3.0; hold(&mut g, gd, true, "orders");      // the same again: not a change
        g.time = 4.0; pin(&mut g, gd, true, "orders");
        let d = g.gd[gd].duty;
        assert_eq!((d.holds_by, d.pinned_by), (Giver { by: "roomfight", since: 2.0 }, Giver { by: "orders", since: 4.0 }));
        assert!(d.holds && d.pinned && !g.gd[gd].duty.free());
        g.time = 6.0; hold(&mut g, gd, false, "pressure");
        assert_eq!(g.gd[gd].duty.holds_by.by, "pressure");
        assert!(!g.gd[gd].duty.holds && !g.gd[gd].duty.free());   // (pinned yet)
    }
}
