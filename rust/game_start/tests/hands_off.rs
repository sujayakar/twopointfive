//! THE HANDS-OFF PLAYER plays a staged room clear from its first step to its end, with nobody at the keys.
use game::scene_scenario_list::{all_missions, pick_search};
use game_start::hands_off::{End, HandsOff, Plan};
use game_start::play::{Opts, Play};

/// A FAILURE MEANS: nothing plays a mission to its end any more, or he gave no order through the keys. (The depot's fight is the shortest there is)
#[test]
fn he_plays_a_room_clear_to_its_end() {
    let e = all_missions().into_iter().find(|e| e.id == "depot-four").expect("the depot is on the list");
    let mut p = Play::mission(e.level.as_deref().unwrap(), &pick_search(&e, ""), &Opts { seed: 9, dress: true, ..Default::default() });
    let mut h = HandsOff::new(p.g.seats[0], Plan { seed: 9, cap: 240.0, ..Default::default() });
    assert!(p.played_out(&mut h, 245.0), "he is still playing: {}", h.trace());
    assert!(h.end().is_some_and(|e| e != End::Cap), "he ran out of time: {}", h.trace());
    assert!(h.given() >= 1 && h.trace().contains("quick"), "no order went through the keys: {}", h.trace());
}
