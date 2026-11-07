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

/// each row on a seat's menu as it stands: which row of the table, its id, digit, page and targets
fn menu_of(g: &Game, seat: &SeatRef) -> Vec<(game::ids::OrdersOrderDefId, String)> {
    seat.orders.defs.iter().map(|&d| { let r = &g.tab.orders_order_def[d]; (d, format!("{} {:?} {:?} {:?}", r.id, r.key, r.page, r.targets)) }).collect()
}

/// THE RULE: a row of the orders' menu belongs to the seat that lists it. Two players' menus hold no row of the table between them, and what is
/// written on one seat's rows (a digit dealt, a page, a target that comes with the pairs) is not on the other's.
/// A FAILURE MEANS: a row is found by its id alone, so the two menus are one set of rows and one player's menu changes the other's.
#[test]
fn a_row_of_the_menu_is_one_seats() {
    for s in SEEDS {
        let mut p = Play::mission("office.json", "?levelFile=office.json&roomclear&squad4", &Opts { seed: s, seats: 2, ..Default::default() });
        p.run(1.0);
        let (first, second) = (p.g.seats[0], p.g.seats[1]);
        let mine = menu_of(&p.g, &seats::seat(&p.g, first));
        let theirs = menu_of(&p.g, &seats::seat(&p.g, second));
        let both: Vec<_> = mine.iter().filter(|(d, _)| theirs.iter().any(|(x, _)| x == d)).collect();
        assert!(!mine.is_empty() && !theirs.is_empty() && both.is_empty(), "seed {s}: {} rows are on both seats' menus: {:?}", both.len(), both);
        // the second seat's fall in is given a target and a digit of its own: the first seat's menu is as it was
        seats::with_seat(&mut p.g, second, &mut |g| {
            let d = g.orders.defs.iter().copied().find(|&d| g.tab.orders_order_def[d].id == "fallIn").expect("a squad's menu has fall in");
            g.tab.orders_order_def[d].targets.push(game::orders::TargetKind::Foe);
            g.tab.orders_order_def[d].key = None;
            game::orders::squad_orders_register(g, d, None);
        });
        assert_eq!(menu_of(&p.g, &seats::seat(&p.g, first)), mine, "seed {s}: the second seat's menu was written and the first seat's changed");
        assert!(menu_of(&p.g, &seats::seat(&p.g, second)) != theirs, "seed {s}: the second seat's menu did not change");
    }
}

/// THE RULE: both players give orders for forty seconds and the table of rows does not grow.
#[test]
fn two_seats_orders_make_no_new_row() {
    for s in SEEDS {
        let mut p = Play::mission("office.json", "?levelFile=office.json&roomclear&squad4", &Opts { seed: s, seats: 2, ..Default::default() });
        p.run(1.0);
        let n = p.menu_rows().len();
        let doors: Vec<[f64; 2]> = p.g.doors.list.iter().map(|&d| { let c = p.g.dr[d].frame_centre; [c[0], c[2]] }).collect();
        for k in 0..20 {
            for seat in p.g.seats.clone() {
                let d = doors[(k * 7 + seat.0 as usize * 3) % doors.len()];
                seats::with_seat(&mut p.g, seat, &mut |g| {
                    let t = game::orders::squad_orders_target_at(g, [d[0], 0.0, d[1]]);
                    let _ = game::orders::squad_orders_give(g, ["openClear", "stack", "fallIn", "hold"][k % 4], Some(&t), Some(Who::All), None);
                });
            }
            p.run(2.0);
        }
        let rows = p.menu_rows();
        assert_eq!(rows.len(), n, "seed {s}: forty seconds of both seats' orders made {} rows: {:?}", rows.len() - n, &rows[n..]);
    }
}
