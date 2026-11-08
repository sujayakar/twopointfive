//! WHAT THE PICTURE IS TOLD OF THE WEATHER, THE AIR AND THE FIRES (rust/game_draw): drawn on levels that have them. What each number is against the copy of the game is
//! tools/draw/weather-check.ts's; these hold what is true on any host.
use game_draw::Drawn;
use game_start::play::{Opts, Play};

const ALL: u32 = u32::MAX;
fn pass(seed: u32) -> Play { Play::mission("pass.json", "?levelFile=pass.json", &Opts { seed, ..Default::default() }) }

/// THE RULE: a fire's row is the mouth the rule has, on the clock the rule has; asked twice of one tick it is the same row.
#[test]
fn a_fire_is_told_with_its_mouth_and_clock() {
    let mut p = pass(3);
    let t = (0..p.g.targets.len()).map(|i| p.g.targets[i]).find(|&t| game::fire::is_fire(&p.g, t)).expect("the pass has a brazier");
    for _ in 0..30 { p.frame(); }
    let mut d = Drawn::default();
    d.take(&p.g, ALL);
    assert!(!d.fires.is_empty(), "a burning fire has a row");
    assert_eq!(d.fire_table.len(), 1, "a level with a fire has one table");
    let (m, r) = (game::fire::mouth_of(&p.g, t), d.fires[0]);
    for k in 0..3 { assert!((r.pos[k] as f64 - m.pos[k]).abs() < 1e-5, "mouth {k}: {} {}", r.pos[k], m.pos[k]); }
    assert!((r.time as f64 - p.g.time % 4096.0).abs() < 1e-3);
    assert!(r.flick >= p.g.k.fire.least as f32 && r.flick <= p.g.k.fire.most as f32, "flicker in its table: {}", r.flick);
    let before = d.fires.clone();
    d.take(&p.g, ALL);
    assert_eq!(before, d.fires);
}

/// THE RULE: the lists the wind leaves are whole every tick: the storm over a storm's level, what blows and the air's numbers.
#[test]
fn the_winds_are_whole_every_tick() {
    let mut p = pass(4);
    p.g.m.dress.dress.wind = true;   // (a host that draws has the wind's row up)
    let mut d = Drawn::default();
    for _ in 0..30 { p.frame(); d.take(&p.g, ALL); }
    assert_eq!(d.winds.air.len(), 1);
    assert_eq!(d.winds.active.len(), p.g.wind.active.len());
    assert!(d.winds.air[0].master > 0.0, "the pass has its wind");
    assert!(d.winds.active.iter().any(|w| w.kind == 1), "the storm blows on the pass");
}
