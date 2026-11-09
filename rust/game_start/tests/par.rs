//! THE PAR CLOCK IS THE GAME'S (rust/game_rules/src/missions/par.rs): a real level with a `scenarios.par` block, its room clear stood by the Rust start, and
//! the clock read off the game's own record. Each test holds one rule over three seeds beside a control that differs in the one thing the rule is
//! about. Before the tick kept the clock, the page's TypeScript did, on its copy of the game: nothing here could have been asked of the game at all.
use game::game::DownBy;
use game::orders::Who;
use game::par::{Medal, PAR_START_M};
use game::roomclear::RoomClearEnd;
use game_start::play::{dist, Opts, Play};

const LEVEL: &str = "killhouse.json";
/// a level with a room clear and no par block
const NO_PAR: &str = "kit-yard.json";
const SEEDS: [u32; 3] = [1, 2, 3];

/// the kill house with its room clear up and one frame run: the clock is armed by the first tick
fn stood(seed: u32, seats: u32) -> Play { let mut p = Play::level(LEVEL, true, &Opts { seed, seats, ..Default::default() }); p.frame(); p }
fn par_lines(p: &Play) -> usize { p.log().iter().filter(|m| m.starts_with("par ▸")).count() }

/// A FAILURE MEANS: the clock runs while nobody has done anything (reading the house from the start line is meant to be free), or it does not
/// arm on a level that has a par, or it arms on a level that has none.
#[test]
fn the_clock_is_armed_and_waits_while_nobody_does_anything() {
    for seed in SEEDS {
        let mut p = stood(seed, 1);
        assert!(p.par_live(), "seed {}: a level with a par block and its room clear up has a par live", seed);
        p.run(2.0);
        assert!(p.par_started().is_none(), "seed {}: two seconds on the mark with nothing ordered: the clock has not started", seed);
        assert_eq!(p.par_counts(), (0, 0));
        let r = p.objective();
        assert!(r.starts_with("par ") && r.contains(" / "), "seed {}: the objective row leads with the three bars: {:?}", seed, r);
        drop(p);
        // CONTROL: a level whose file carries no par. Nothing is armed, an order starts nothing, and the row is the room clear's own.
        let mut q = Play::level(NO_PAR, true, &Opts { seed, ..Default::default() });
        q.run(2.0);
        let at = q.sam_at();
        q.order("fallIn", at, Who::All);
        q.run(0.5);
        assert!(!q.par_live() && q.par_started().is_none() && q.par_counts() == (0, 0), "seed {}: {} has no par: no clock is armed, and an order starts none", seed, NO_PAR);
        assert!(!q.objective().starts_with("par ") && !q.objective().starts_with("⏱"), "seed {}: …and the row is the room clear's own: {:?}", seed, q.objective());
    }
}

/// A FAILURE MEANS: an order to the squad no longer counts as "the first thing done", or the clock starts at another moment than the tick the order
/// was given in.
#[test]
fn the_squads_first_order_starts_the_clock() {
    for seed in SEEDS {
        let mut p = stood(seed, 1);
        p.run(0.5 + 0.25 * seed as f64);
        assert!(p.par_started().is_none(), "seed {}: not started before the order", seed);
        let at = p.sam_at();
        p.order("fallIn", at, Who::All);
        p.frame();
        let t = p.time();
        assert_eq!(p.par_started(), Some(t), "seed {}: the order started the clock, at the tick that saw it", seed);
        assert_eq!(p.par_counts().0, 1);
        assert_eq!(dist(p.sam_at(), at), 0.0, "seed {}: Sam has not left his mark: it was the order", seed);
        p.run(1.5);
        let row = p.objective();
        assert!(row.starts_with("⏱ 0:01 → "), "seed {}: a second and a half on, the row leads with the running clock: {:?}", seed, row);
    }
}

/// A FAILURE MEANS: a player's own stride no longer starts the clock, or a shuffle on the mark does (the start is `PAR_START_M` off the mark the
/// staging stood him on, measured on the floor).
#[test]
fn a_stride_off_the_mark_starts_the_clock_and_a_shuffle_does_not() {
    for seed in SEEDS {
        let mut p = stood(seed, 1);
        let mark = p.sam_at();
        // CONTROL: 0.6 to 0.8 of the distance is a shuffle
        p.seat_steps(0, PAR_START_M * (0.5 + 0.1 * seed as f64));
        p.frame();
        assert!(dist(p.sam_at(), mark) < PAR_START_M && p.par_started().is_none(), "seed {}: a shuffle of {} m is not a start", seed, dist(p.sam_at(), mark));
        p.seat_steps(0, PAR_START_M * 0.8);
        p.frame();
        assert!(dist(p.sam_at(), mark) > PAR_START_M, "seed {}: he is past the distance", seed);
        assert_eq!(p.par_started(), Some(p.time()), "seed {}: past {} m from the mark the clock runs, from that tick", seed, PAR_START_M);
    }
}

/// A GAME OF TWO: the clock is one, and the first thing EITHER player does starts it. A FAILURE MEANS: the second seat's man or his orders are
/// not looked at (each page used to time its own seat, and two machines of one game showed two clocks), or a seat that has done nothing starts it.
#[test]
fn in_a_game_of_two_either_players_first_move_starts_the_one_clock() {
    for seed in SEEDS {
        for by_order in [false, true] {
            let mut p = stood(seed, 2);
            let (first, second) = (p.seat_at(0), p.seat_at(1));
            assert!(dist(first, second) > 0.0, "seed {}: two players stand in two places", seed);
            // CONTROL: two players on their marks start nothing, and the second's shuffle does not either
            p.run(1.0);
            p.seat_steps(1, PAR_START_M * 0.6);
            p.run(0.5);
            assert!(p.par_live() && p.par_started().is_none(), "seed {}: nobody has done anything: the clock waits (the second player shuffled {} m)", seed, dist(p.seat_at(1), second));
            if by_order { let at = p.seat_at(1); p.seat_orders(1, "fallIn", at, Who::All); } else { p.seat_steps(1, PAR_START_M * 0.8); }
            p.frame();
            assert_eq!(p.par_started(), Some(p.time()), "seed {}: the second player's {} started the clock, at that tick", seed, if by_order { "order" } else { "stride" });
            assert_eq!(dist(p.seat_at(0), first), 0.0, "seed {}: the first player never moved: it was the second", seed);
            assert_eq!(p.par_counts().0, 1, "seed {}: one clock, started once", seed);
            p.run(1.0);
            p.seat_steps(0, PAR_START_M * 2.0);
            p.run(0.5);
            assert_eq!(p.par_counts().0, 1, "seed {}: the first player moving later starts nothing again", seed);
        }
    }
}

/// THE ARITHMETIC, on the kill house's own numbers. A FAILURE MEANS: a medal is no longer judged on the whole seconds the clock shows, a man lost is
/// not priced at `loss_secs`, or the line no longer says what the next medal needed.
#[test]
fn the_medal_is_judged_on_the_whole_seconds_shown() {
    use game::par::{medal_of, par_result};
    let p = stood(1, 1);
    let d = p.par_bars().expect("the kill house carries a par");
    drop(p);
    assert!(d.gold < d.silver && d.silver < d.bronze && d.loss_secs > 0.0);
    for (total, want) in [(d.gold, Medal::Gold), (d.gold + 1.0, Medal::Silver), (d.silver, Medal::Silver), (d.silver + 1.0, Medal::Bronze), (d.bronze, Medal::Bronze), (d.bronze + 1.0, Medal::None)] {
        assert_eq!(medal_of(&d, total), want, "a total of {} s", total);
    }
    let r = par_result(&d, d.gold + 0.4, 0.0, 4.0);
    assert!(r.medal == Medal::Gold && r.shown == d.gold && r.next.is_none() && !r.line.contains("was"), "{} s reads {:?}", d.gold + 0.4, r.line);
    let (a, b) = (par_result(&d, d.gold - 1.0, 0.0, 4.0), par_result(&d, d.gold - 1.0, 2.0, 4.0));
    assert!(a.medal == Medal::Gold && a.penalty == 0.0 && b.penalty == 2.0 * d.loss_secs && b.total == d.gold - 1.0 + 2.0 * d.loss_secs && b.medal != Medal::Gold);
    assert!(b.line.contains("(2 lost)"), "{:?}", b.line);
    let c = par_result(&d, d.silver + 3.0, 0.0, 4.0);
    assert!(c.medal == Medal::Bronze && c.next.map(|n| (n.medal, n.short)) == Some((Medal::Silver, 3.0)) && c.line.contains(" · BRONZE · silver was "), "{:?}", c.line);
    let (fast, clean) = (par_result(&d, d.gold - 10.0, 2.0, 4.0), par_result(&d, d.gold - 10.0 + 2.0 * d.loss_secs - 1.0, 0.0, 4.0));
    assert!(fast.total > clean.total, "a run that loses two men ranks under a clean one that is slower by less than two penalties");
    assert_eq!(game::par::clock(65.4), "1:05");
}

/// A FAILURE MEANS: the run's end is no longer timed at the room clear's own end, the result is said more than once or not at all, a man lost is
/// not priced, or the result is not the game's own record (the page would have nothing to draw).
#[test]
fn the_last_defender_down_ends_the_run_with_a_result_said_once() {
    for seed in SEEDS {
        for lose_a_man in [false, true] {
            let mut p = stood(seed, 1);
            let d = p.par_bars().expect("the kill house carries a par");
            let at = p.sam_at();
            p.order("fallIn", at, Who::All);
            p.run(1.0 + seed as f64);
            let t0 = p.par_started().unwrap_or_else(|| panic!("seed {}: the order started no clock", seed));
            assert!(p.par_result().is_none() && par_lines(&p) == 0, "seed {}: no result while defenders stand", seed);
            if lose_a_man { let m = p.mates()[0]; p.falls(m, DownBy::Guard); }
            for h in p.hostiles() { p.falls(h, DownBy::Player); }
            p.until(20.0, |p| p.room_clear_end().is_some()).unwrap_or_else(|| panic!("seed {}: every defender is down and the room clear never ended", seed));
            let (end, end_at) = p.room_clear_end().unwrap();
            assert_eq!(end, RoomClearEnd::Taken, "seed {}", seed);
            p.frame();
            let r = p.par_result().unwrap_or_else(|| panic!("seed {}: the room clear is taken and the game holds no result", seed));
            assert!((r.secs - (end_at - t0)).abs() < 1e-9, "seed {}: the clock stopped at the room clear's own end ({} s)", seed, r.secs);
            // (the men lost are the teammates down at the end, whoever put them there: the one this test dropped, and any a defender got first)
            let down = p.mates().iter().filter(|&&m| p.state(m) == game::game::GuardState::Dead).count() as f64;
            assert_eq!(r.lost, down, "seed {}: every teammate down at the end is counted", seed);
            assert!(!lose_a_man || r.lost >= 1.0, "seed {}: the man this test dropped is one of them", seed);
            assert_eq!(r.penalty, r.lost * d.loss_secs);
            assert_eq!(r.total, r.secs + r.penalty);
            assert_eq!(r.medal, game::par::medal_of(&d, r.total));
            assert_eq!(r.medal == Medal::Gold, r.next.is_none(), "seed {}: nothing stands above gold", seed);
            assert_eq!(par_lines(&p), 1, "seed {}: the line is on the log once: {:?}", seed, p.log());
            assert!(p.log().iter().any(|m| *m == r.line));
            assert_eq!(r.line.contains("lost"), r.lost > 0.0, "seed {}: {:?}", seed, r.line);
            let said = p.objective();
            let medal = if r.medal == Medal::None { String::from("no medal") } else { r.medal.as_str().to_uppercase() };
            assert!(said.starts_with("structure taken · ") && said.contains(&medal), "seed {}: the row words the result: {:?}", seed, said);
            p.run(3.0);
            assert_eq!(p.par_counts(), (1, 1), "seed {}: started once, finished once", seed);
            assert_eq!(p.par_result().map(|x| x.secs), Some(r.secs), "seed {}: the result stands", seed);
            assert!(par_lines(&p) <= 1, "seed {}: and is not said again (the log keeps five lines: it may have scrolled off)", seed);
        }
    }
}
