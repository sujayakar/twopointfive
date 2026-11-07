//! A SEAT IS A VALUE (rust/game_rules/src/seats/seat_ref.rs). A function that holds the game shared and depends on the seat takes a `SeatRef`,
//! so it can be asked about ANY seat of a game nobody may write to: not only the one that happens to stand on it.
//!
//! Before readers took a seat, the only way to ask about the second player's seat was to stand it on the game (`with_seat`, which writes
//! nineteen fields over and back); asked on the game as it stood, a reader silently answered for the first seat.
use game::game::Game;
use game::orders::Who;
use game::seats::{self, SeatRef};
use game_start::play::{Opts, Play};

const SEEDS: [u32; 3] = [5, 6, 7];

/// A handful of readers of one seat: the men an order would go to, where the seat's man is, the wall charge's row on the menu, whether the
/// seat's squad is whole.
type Answers = (Vec<game::ids::GuardId>, [f64; 3], String, bool);
fn answers(g: &Game, seat: &SeatRef) -> Answers {
    (game::orders::squad_orders_recipients(g, seat, None, Some(Who::All)), game::orders::squad_orders_sam_place(g, seat), game::wallcharge::wall_charge_row_words(g, seat), game::orders::squad_orders_whole(g, seat))
}

/// THE RULE: in a game of two players, the readers asked about the second seat while the FIRST stands on the game answer as they do with the
/// second seat stood on it; and the two seats' answers are not the same answers (each has his own man and his own pair).
/// A FAILURE MEANS: a reader has gone back to reading the game's own fields of a seat's name (it answers for whichever seat stands), or
/// `seats::seat` no longer finds a kept seat's things.
#[test]
fn a_reader_asked_about_the_seat_that_does_not_stand_answers_for_that_seat() {
    for s in SEEDS {
        let mut p = Play::mission("office.json", "?levelFile=office.json&roomclear&squad4", &Opts { seed: s, seats: 2, ..Default::default() });
        p.run(1.0);
        let g = &mut *p.g;
        let (first, second) = (g.seats[0], g.seats[1]);
        assert_eq!(g.seat, first, "seed {s}: between ticks the first seat stands on the game");
        let stood = seats::with_seat(g, second, &mut |g| answers(g, &seats::here(g)));
        let asked = answers(g, &seats::seat(g, second));
        assert_eq!(asked, stood, "seed {s}: asked about the second seat with the first standing, the readers do not answer as with the second stood");
        let own = answers(g, &seats::seat(g, first));
        assert_eq!(own, answers(g, &seats::here(g)), "seed {s}: the standing seat by name and by `here` are two answers");
        assert!(own.0 != asked.0 && own.1 != asked.1, "seed {s}: the two seats have one man and one pair between them: {:?} and {:?}", own, asked);
        assert_eq!((own.0.len(), asked.0.len()), (2, 2), "seed {s}: four teammates, two to a seat");
    }
}
