//! WHAT IS MADE BEHIND THE LOADING IS NO STATE: the table of the light under the roofs is made by a host a slice at a time while it waits for something else, and a step that comes before the last
//! slice makes what it wants. So a run is the same run however much of it was made: all of it, none, or some metres with steps between.
use game::scene_collision::light_ready;
use game_start::play::{Opts, Play};

fn stand(id: &str, ready: bool) -> Play {
    let e = game::scene_scenario_list::scenarios().into_iter().find(|e| e.id == id).unwrap();
    let link = game::scene_scenario_list::scenario_query(&e);
    Play::mission(&e.level.unwrap_or("office.json".into()), &link, &Opts { seed: 11, ready, ..Default::default() })
}
/// what is made of the table
#[derive(Clone, Copy, PartialEq)]
enum Made { Whole, Nothing, /// so many lines before each step
    Lines(usize) }
/// every part's word, at the stand and every tenth of a second of the run
fn played(id: &str, made: Made) -> Vec<Vec<u32>> {
    // (the table outlives its game, and is of the statics that stand: another level's go first)
    drop(stand("killhouse-four", true));
    let mut p = stand(id, made == Made::Whole);
    // (not asked of nothing made: the asking looks at the statics)
    if made != Made::Nothing { assert_eq!(light_ready(&p.g, 0), made == Made::Whole, "{id}: the test starves nothing"); }
    let mut out = vec![p.words()];
    for i in 0..60 {
        if let Made::Lines(n) = made { light_ready(&p.g, n); }
        p.frame();
        if i % 6 == 5 { out.push(p.words()); }
    }
    assert!(light_ready(&p.g, 0), "{id}: a second of play and the table still wants something");
    out
}

#[test]
fn a_run_is_the_same_whatever_was_made_before_its_first_step() {
    for id in ["room-clear-four", "compound-four", "night-breach"] {
        let whole = played(id, Made::Whole);
        assert!(whole == played(id, Made::Nothing), "{id}: nothing was made before the first step, and the run is another");
        assert!(whole == played(id, Made::Lines(400)), "{id}: some lines before each step, and the run is another");
    }
}
