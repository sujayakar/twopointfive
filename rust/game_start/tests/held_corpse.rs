//! A FOUND BODY AND A TAMPERED DOOR: what the floor says, and what it does next.
mod kit_ask;
use game::game::{DownBy, DownKind};
use game_start::play::{Opts, Play};
use kit_ask::KitAsk;

const FAR: [f64; 2] = [4.9, 23.2];
const STOCK: [&str; 2] = ["clear here — staying on it", "must have been nothing"];

fn office(seed: u32) -> Play {
    let mut p = Play::level("office.json", false, &Opts { seed, ..Default::default() });
    p.sam_a_ghost();
    p.sam_unhurt();
    p.sam_put(FAR);
    p.run(0.5);
    p
}

/// THE RULE: on a floor on edge a man taken down quietly is found, the floor sends men round for him, and nobody within four metres of him says the stock words of a noise that came to nothing.
/// THE CONTROL: the same floor with nobody down is sent nowhere and finds nothing.
/// A FAILURE MEANS: a guard stands by a dead friend and says "must have been nothing", or a body changes nothing.
#[test]
fn a_found_body_moves_the_floor_and_silences_the_stock_words() {
    for seed in [32, 33, 34] {
        let mut p = office(seed);
        p.floor_on_edge();
        p.run(5.0);
        let v = p.men_on_the_floor()[0];
        p.falls_as(v, DownKind::Struck, true, DownBy::Player);
        let body = p.at(v);
        let n = p.said_so_far();
        p.until(150.0, |p| p.body_found(v)).unwrap_or_else(|| panic!("seed {seed}: nobody finds him in 150 s"));
        let moved = p.until(25.0, |p| p.clearing());
        p.run(10.0);
        assert!(moved.is_some(), "seed {seed}: found, and the floor sends nobody round");
        for (who, text) in p.said_since(n) {
            let at = p.at(who);
            if ((at[0] - body[0]).powi(2) + (at[1] - body[1]).powi(2)).sqrt() < 4.0 { assert!(!STOCK.contains(&text.as_str()), "seed {seed}: said by the body: {text}"); }
        }
    }
    let mut p = office(32);
    p.floor_on_edge();
    p.run(5.0);
    let v = p.men_on_the_floor()[0];
    assert!(p.always(25.0, |p| !p.clearing() && !p.body_found(v)).is_none(), "control: with nobody down the floor goes round for nobody");
}

/// THE RULE: the stock words are for a suspicion that came to nothing: a man with a found body beside him does not say them.
/// THE CONTROL: the same man with the same suspicion and nobody down says them.
/// A FAILURE MEANS: the words never come at all, and the test above says nothing.
#[test]
fn the_stock_words_are_kept_for_a_noise_that_came_to_nothing() {
    let run = |body: bool, seed: u32| -> Vec<String> {
        let mut p = office(seed);
        let men = p.men_on_the_floor();
        let (a, other) = (men[0], men[1]);
        p.drop_all_but(&[a, other]);
        let at = p.at(a);
        if body {
            p.man_stands(other, [at[0] + 1.5, at[1]]);
            p.falls_as(other, DownKind::Struck, true, DownBy::Player);
        }
        let n = p.said_so_far();
        p.man_suspects(a, at);
        p.run(1.0);
        p.until(30.0, |p| p.said_since(n).iter().any(|(_, t)| STOCK.contains(&t.as_str())));
        p.said_since(n).into_iter().filter(|(w, _)| *w == a).map(|(_, t)| t).collect()
    };
    for seed in [1, 2, 3] {
        let bare = run(false, seed);
        assert!(bare.iter().any(|t| STOCK.contains(&t.as_str())), "seed {seed}, control: suspicion with nothing there ends in the stock words: {bare:?}");
        let by_body = run(true, seed);
        assert!(!by_body.iter().any(|t| STOCK.contains(&t.as_str())), "seed {seed}: with a friend down beside him: {by_body:?}");
    }
}

/// Run a clear called at `fix` until it has started on a room: the room drilled first, and every line said.
fn first_drilled(p: &mut Play, fix: [f64; 2]) -> (Option<String>, Vec<String>) {
    let n = p.said_so_far();
    let plan = p.clear_called_at(fix, FAR);
    assert!(!plan.is_empty(), "a fix on the corridor plans rooms");
    p.until(100.0, |p| p.clear_drilling().is_some());
    let first = p.clear_drilling();
    p.run(0.5);
    (first, p.said_since(n).into_iter().map(|(_, t)| t).collect())
}

/// THE RULE: a clear that passes a picked door goes through it first and says so, ahead of the room it planned first.
/// THE CONTROL: the same fix, the door untouched: the planned room comes first and nobody speaks of a lock.
/// A FAILURE MEANS: a door the player has picked tells the floor nothing.
#[test]
fn a_picked_door_jumps_the_queue_and_is_remarked_on() {
    const EAST: [f64; 2] = [22.5, 11.3];
    for seed in [41, 42, 43] {
        let mut p = office(seed);
        let (first, said) = first_drilled(&mut p, EAST);
        assert_ne!(first.as_deref(), Some("server room"), "seed {seed}, control: the room nearest the fix comes first");
        assert!(!said.iter().any(|t| t.contains("picked")), "seed {seed}, control: {said:?}");
        drop(p);

        let mut p = office(seed);
        let d = p.door("server");
        p.door_found_picked(d);
        let (first, said) = first_drilled(&mut p, EAST);
        assert_eq!(first.as_deref(), Some("server room"), "seed {seed}: the picked door's room goes first: {said:?}");
        assert!(said.iter().any(|t| t.contains("picked")), "seed {seed}: nobody remarks on the lock: {said:?}");
    }
}

/// THE RULE: a door kicked in is called forced, and its room is taken first.
/// THE CONTROL: the same fix with the lock whole has another room first.
/// A FAILURE MEANS: a broken door reads the same as a whole one to the men who walk past it.
#[test]
fn a_door_kicked_in_goes_first_and_is_called_forced() {
    const WEST: [f64; 2] = [8.0, 11.2];
    for seed in [44, 45, 46] {
        let mut p = office(seed);
        let (first, _) = first_drilled(&mut p, WEST);
        assert_ne!(first.as_deref(), Some("storage room"), "seed {seed}, control");
        drop(p);

        let mut p = office(seed);
        let d = p.door("storage");
        p.door_found_forced(d);
        let (first, said) = first_drilled(&mut p, WEST);
        assert_eq!(first.as_deref(), Some("storage room"), "seed {seed}: {said:?}");
        assert!(said.iter().any(|t| t.contains("forced")), "seed {seed}: nobody says it was forced: {said:?}");
    }
}
