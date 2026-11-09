//! A MOMENT OF A GAME, whole: where a walk of a tape may come back to. It is the game as it stood and everything this module holds of it (its saves, what was said), so
//! what is handed over again from there plays the same run. A save put back (saves.rs `back`) is another thing: it goes on from now, and no serial is dealt twice.
use crate::saves::hold;
use game::savestate::control::{self, Kept};
use std::cell::RefCell;

struct Moment { of: u32, game: control::Moment, shelf: Vec<(String, Kept)>, said: Vec<String> }

thread_local! {
    /// the moments held, by number less one
    static MOMENTS: RefCell<Vec<Option<Moment>>> = const { RefCell::new(Vec::new()) };
}

/// the moment game `h` stands at, between two ticks; 0 where it cannot be held (a hands-off player has the seat: what he is about is no part of the game)
#[no_mangle]
pub extern "C" fn face_moment(h: u32) -> u32 {
    let Some(m) = super::with(h, |x| x.hands.is_none().then(|| {
        let stand = control::stood(&mut x.g);
        Moment { of: h, game: control::Moment::of(&x.g), shelf: control::kept_of(stand), said: x.said.clone() }
    })) else { return 0 };
    MOMENTS.with(|v| { let mut v = v.borrow_mut(); match v.iter().position(|x| x.is_none()) { Some(i) => { v[i] = Some(m); i as u32 + 1 } None => { v.push(Some(m)); v.len() as u32 } } })
}
/// game `h` stands at its moment `k` again, also where it was lost to a panic since; 0 = it is no moment of that game
#[no_mangle]
pub extern "C" fn face_moment_back(h: u32, k: u32) -> u32 {
    let Some((shelf, said)) = MOMENTS.with(|v| v.borrow().get((k as usize).wrapping_sub(1)).and_then(|m| m.as_ref()).filter(|m| m.of == h).map(|m| {
        // (a game lost to a panic left its number and its slot: it stands in them)
        let slot = m.game.game.sim;
        super::stand_if_lost(h, || m.game.laid(slot));
        super::with(h, |x| { let g = m.game.laid(x.g.sim); hold(x, g); x.hands = None; });
        (m.shelf.clone(), m.said.clone())
    })) else { return 0 };
    super::with(h, |x| { control::kept_again(control::stood(&mut x.g), &shelf); x.said = said; });
    1
}
#[no_mangle]
pub extern "C" fn face_moment_drop(k: u32) {
    MOMENTS.with(|v| if let Some(m) = v.borrow_mut().get_mut((k as usize).wrapping_sub(1)) { *m = None; });
}
/// the moments of a game that is let go
pub fn off(h: u32) { MOMENTS.with(|v| for m in v.borrow_mut().iter_mut() { if m.as_ref().is_some_and(|m| m.of == h) { *m = None; } }) }
