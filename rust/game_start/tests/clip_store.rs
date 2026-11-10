//! The clip store (rust/sim/src/clips.rs) is the process's and a game's clips are one set in it, however many games a process stands: a tool that stands hundreds
//! of games must not fill it. What is held after the first game is all that is ever held.
use game_start::play::{Opts, Play};
use game_start::scene::{Axis, Scene};

#[test]
fn forty_games_hold_one_set_of_clips() {
    let held = || { let t = ::sim::clips::table_read(); (t.n_clips, t.n_tracks, t.used) };
    let writes = || ::sim::clips::table_read().begins;
    let scene = || { let mut s = Scene::new("clip-store"); s.room("a", 10.0, 10.0, 18.0, 16.0).door("out", Axis::Z, 18.0, 12.4); s.bulb(14.0, 13.0).sam(12.0, 13.0); s };
    let first = { let _p = Play::scene(&scene(), &Opts::default()); held() };
    assert!(first.0 > 0, "a game stood and held no clips");
    let wrote = writes();
    assert!(wrote > 0, "a game stood and wrote no clip");
    for n in 0..39 {
        let _p = Play::scene(&scene(), &Opts::default());
        assert_eq!(held(), first, "game {} filled the store", n + 2);
        assert_eq!(writes(), wrote, "game {} wrote a clip again", n + 2);
    }
}
