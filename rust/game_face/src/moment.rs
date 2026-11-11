//! A MOMENT OF A GAME, whole: where a walk of a tape may come back to. It is the game as it stood and everything this module holds of it (its saves, what was said), so
//! what is handed over again from there plays the same run. A save put back (saves.rs `back`) is another thing: it goes on from now, and no serial is dealt twice.
use crate::saves::hold;
use crate::Held;
use game::savestate::control::{self, Kept};
use game_start::host::calls::{GameNo, MomentNo};
use std::cell::RefCell;

struct Moment { of: u32, game: control::Moment, shelf: Vec<(String, Kept)>, said: Vec<String> }

thread_local! {
    /// the moments held, by number less one
    static MOMENTS: RefCell<Vec<Option<Moment>>> = const { RefCell::new(Vec::new()) };
}

pub fn moment(x: &mut Held) -> MomentNo {
    if x.hands.is_some() { return 0; }
    let stand = control::stood(&mut x.g);
    let m = Moment { of: x.no, game: control::Moment::of(&x.g), shelf: control::kept_of(stand), said: x.said.clone() };
    MOMENTS.with(|v| { let mut v = v.borrow_mut(); match v.iter().position(|x| x.is_none()) { Some(i) => { v[i] = Some(m); i as u32 + 1 } None => { v.push(Some(m)); v.len() as u32 } } })
}
pub fn moment_back(h: GameNo, k: MomentNo) -> bool {
    let Some((shelf, said)) = MOMENTS.with(|v| v.borrow().get((k as usize).wrapping_sub(1)).and_then(|m| m.as_ref()).filter(|m| m.of == h).map(|m| {
        // (a game lost to a panic left its number and its slot: it stands in them)
        let slot = m.game.game.sim;
        super::stand_if_lost(h, || m.game.laid(slot));
        super::with(h, |x| { let g = m.game.laid(x.g.sim); hold(x, g); x.hands = None; });
        (m.shelf.clone(), m.said.clone())
    })) else { return false };
    super::with(h, |x| { control::kept_again(control::stood(&mut x.g), &shelf); x.said = said; });
    true
}
pub fn moment_drop(k: MomentNo) { MOMENTS.with(|v| if let Some(m) = v.borrow_mut().get_mut((k as usize).wrapping_sub(1)) { *m = None; }); }
/// the moments of a game that is let go
pub fn off(h: u32) { MOMENTS.with(|v| for m in v.borrow_mut().iter_mut() { if m.as_ref().is_some_and(|m| m.of == h) { *m = None; } }) }
