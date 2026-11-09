//! WHAT THE PICTURE IS TOLD OF THE WEATHER, THE AIR, THE FIRES AND THE SNOW (rust/game_draw): drawn on levels that have them. These hold what is true on any host; that each number stays what it was is nobody's since the check against the copy went.
use game::out_events::OutEvent;
use game_draw::{want, Drawn};
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

/// THE RULE: a particle pool is handed the wind round the view and the storm last, made of the standing game. No record holds that list.
#[test]
fn the_pool_is_handed_the_storm() {
    let mut p = pass(4);
    p.g.m.dress.dress.wind = true;
    let mut d = Drawn::default();
    for _ in 0..30 { p.frame(); }
    d.take(&p.g, ALL);
    assert!(!d.winds.gusts.is_empty() && d.winds.gusts.len() <= game::smoke_types::WIND_GPU_MAX as usize, "{} rows", d.winds.gusts.len());
    assert_eq!(d.winds.gusts.last().unwrap().kind, 1, "the storm goes last");
}

/// THE RULE: a fire's hot air is told on the ticks the rule files it, and what a restart clears is said in one row of that tick.
#[test]
fn heat_and_what_goes_are_told() {
    let mut p = pass(5);
    p.g.m.dress.dress.wind = true;
    let (mut d, mut hot) = (Drawn::default(), false);
    assert!((0..p.g.targets.len()).any(|i| game::fire::is_fire(&p.g, p.g.targets[i])), "the pass has a brazier");
    for _ in 0..60 { p.g.out = game_start::stand::frame(&mut p.g, &p.host, 1.0 / 60.0); d.take(&p.g, ALL); hot |= !d.winds.heat.is_empty(); p.g.out.clear(); }
    assert!(hot, "beside a burning fire its hot air is told");
    p.g.out = game_start::stand::frame(&mut p.g, &p.host, 1.0 / 60.0);
    p.g.out.clear();
    game::game::game_restart_encounter(&mut p.g);
    d.take(&p.g, ALL);
    assert_eq!(d.resets.len(), 1, "one row a tick");
    let w = d.resets[0].what;
    assert!(w & 4 != 0 && w & 8 != 0 && w & 16 != 0, "the blasts, the marks and the prints go: {w}");
}

/// THE RULE: a level in the snow says what snow its statics are, on another says nothing; its men are told; and the sky says it is a snow level.
#[test]
fn a_snow_level_says_what_lies_on_it() {
    let mut p = pass(5);
    for _ in 0..10 { p.frame(); }
    let mut d = Drawn::default();
    d.take(&p.g, ALL);
    assert_eq!(d.sky[0].snow_level, 1, "the pass is a snow level");
    assert_eq!(d.snow.words.len(), p.g.engine.world.statics.len());
    assert!(d.snow.words.iter().any(|w| w.w == 1), "the pass has fresh snow");
    let me = p.g.ch[p.g.pl[p.g.player].ch].id as u32;
    let f = d.snow.feet.iter().find(|f| f.id == me).expect("Sam stands on it");
    assert_eq!(f.says & game_draw::snow::ALIVE, game_draw::snow::ALIVE);
    drop(p);
    let mut o = Play::mission("office.json", "?levelFile=office.json&roomclear&squad4", &Opts { seed: 5, ..Default::default() });
    for _ in 0..4 { o.frame(); }
    let mut d = Drawn::default();
    d.take(&o.g, ALL);
    assert_eq!(d.sky[0].snow_level, 0);
    assert!(d.snow.words.is_empty() && d.snow.feet.is_empty(), "the office tells nothing of snow");
}

/// THE RULE: a blast over the snow reaches the picture: a row with its id, once. RED BEFORE: `snow_blast_at` did nothing, and a blast left the snow as it lay.
#[test]
fn a_blast_over_the_snow_is_told() {
    let mut p = pass(6);
    for _ in 0..4 { p.frame(); }
    let at = p.sam_at();
    game::game::game_drop_throwable(&mut p.g, game::game::ThrowKind::Frag, [at[0] + 3.0, 0.0, at[1]]);
    let (mut told, mut d) = (0, Drawn::default());
    for _ in 0..360 {
        p.g.god_mode = true;
        p.g.out = game_start::stand::frame(&mut p.g, &p.host, 1.0 / 60.0);
        d.take(&p.g, ALL);
        if p.g.out.iter().any(|e| matches!(e, OutEvent::SnowBlast(_))) { assert!(!d.snow.blasts.is_empty()); }
        told += d.snow.blasts.len();
        p.g.out.clear();
    }
    assert_eq!(told, 1, "a frag went off on the pass: one blast is told, once");
}

/// THE RULE: drawing the rows of the snow is no part of the game.
#[test]
fn the_snow_rows_change_nothing() {
    let played = |draw: bool| { let mut p = pass(7); let mut d = Drawn::default(); (0..120).map(|_| { p.frame(); if draw { d.take(&p.g, want::fires | want::air | want::snow | want::events); } p.words() }).collect::<Vec<_>>() };
    assert_eq!(played(false), played(true));
}

/// THE RULE: what the rain wets is told in the box's rows: under a picked rain the open ground is darker and glossy, and the box in the game is as it was.
#[test]
fn wet_ground_is_the_rows_and_not_the_box() {
    let mut p = Play::mission("compound.json", "?levelFile=compound.json&weather=downpour&roomclear&squad4", &Opts { seed: 8, ..Default::default() });
    for _ in 0..30 { p.frame(); }
    assert!(p.g.weather.wet_on, "a downpour has the ground wet in half a second");
    let mut d = Drawn::default();
    d.take(&p.g, ALL);
    d.frame(&p.g, 1.0, p.g.aim_point);
    let wet = (0..p.g.engine.world.statics.len() as u32).filter(|&i| d.boxes.wet.is(i)).collect::<Vec<_>>();
    assert!(!wet.is_empty(), "a downpour wets something on the compound");
    for &i in &wet {
        let b = &p.g.tab.boxes[p.g.engine.world.statics[i as usize]];
        let m = &d.boxes.mat[i as usize];
        if b.metal != Some(true) && b.wet_k.is_none() { assert!((m.w[0] as f64 - b.albedo[0] * 0.5).abs() < 1e-5, "box {i} is sent darker: {} of {}", m.w[0], b.albedo[0]); }
    }
    // (a wet box whose game box did not move is not packed again: it is told as one row of a tick's changes only when it changed)
    for _ in 0..3 {
        p.frame();
        d.take(&p.g, ALL);
        d.frame(&p.g, 1.0, p.g.aim_point);
        let h = d.boxes.head[0];
        assert!(!wet.iter().any(|&i| i >= h.from && i < h.to), "a wet box is sent again every tick: {}..{}", h.from, h.to);
    }
}
