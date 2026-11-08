//! WHAT THE PICTURE IS TOLD (rust/game_draw), of games that stand. That a section says what the game has is each section's own test (`fog.rs` holds the fog's);
//! these hold what is true of all of them: drawing a game changes nothing of it, what is drawn between two ticks stands between them, and nothing is drawn
//! through a cut. `hold draw` draws every frame of a set of plays and says what it cost.
use game_draw::{want, Drawn};
use game_start::play::{Opts, Play};

const ALL: u32 = u32::MAX;
fn office(seed: u32) -> Play { Play::mission("office.json", "?levelFile=office.json&roomclear&squad4", &Opts { seed, ..Default::default() }) }
fn man(d: &Drawn, id: u32) -> Option<[f32; 3]> { let i = d.men.men.iter().position(|m| m.id == id)?; Some([d.men.bones[i].w[12], d.men.bones[i].w[13], d.men.bones[i].w[14]]) }
fn head(d: &Drawn) -> &game_draw::boxes::Head { &d.boxes.head[0] }

/// THE RULE: every section is made of the game shared, so the game drawn after every frame is, in every part of its state, the game nobody drew.
/// A FAILURE MEANS: a reader a section calls has come to write through the shared game (rust/sim's statics, a memo in a cell).
#[test]
fn drawing_a_game_is_no_part_of_it() {
    let played = |draw: bool| {
        let mut p = office(5);
        let mut d = Drawn::default();
        (0..240).map(|k| {
            if k == 60 { let at = p.sam_at(); game::game::game_drop_throwable(&mut p.g, game::game::ThrowKind::Frag, [at[0] + 3.0, 0.0, at[1]]); }
            p.g.god_mode = true;
            p.frame();
            if draw { let focus = p.g.aim_point; d.take(&p.g, ALL); d.frame(&p.g, 0.5, focus); d.frame(&p.g, 1.0, focus); }
            p.words()
        }).collect::<Vec<_>>()
    };
    let (plain, drawn) = (played(false), played(true));
    assert_eq!(plain.iter().zip(&drawn).position(|(a, b)| a != b), None, "the drawn game parts from the one nobody drew");
}

/// THE RULE: half way between two ticks a man stands half way between where the two left him, his proxies with him; at the whole he stands where the last left him.
#[test]
fn a_frame_between_two_ticks_stands_between_them() {
    let mut p = office(6);
    let mut d = Drawn::default();
    p.key_down("KeyW");
    p.run(0.5);
    let id = p.g.ch[p.g.pl[p.g.player].ch].id as u32;
    let focus = p.g.aim_point;
    d.take(&p.g, ALL);
    d.frame(&p.g, 1.0, focus);
    let before = man(&d, id).expect("Sam is drawn");
    p.frame();
    d.take(&p.g, ALL);
    d.frame(&p.g, 1.0, focus);
    let after = man(&d, id).unwrap();
    d.frame(&p.g, 0.5, focus);
    let half = man(&d, id).unwrap();
    assert!((0..3).any(|k| (after[k] - before[k]).abs() > 1e-3), "Sam walks, and his bones did not move in a tick: {before:?} {after:?}");
    for k in 0..3 { assert!((half[k] - (before[k] + after[k]) / 2.0).abs() < 1e-4, "half way {k}: {} is not between {} and {}", half[k], before[k], after[k]); }
    d.frame(&p.g, 1.0, focus);
    assert_eq!(man(&d, id).unwrap(), after, "asked again at the whole he stands where the last tick left him");
}

/// THE RULE: after a cut (a load, a rollback, another seat) the next frame between two ticks is the last tick's: nothing moves through it.
#[test]
fn nothing_is_drawn_through_a_cut() {
    let mut p = office(7);
    let mut d = Drawn::default();
    p.key_down("KeyW");
    p.run(0.5);
    let id = p.g.ch[p.g.pl[p.g.player].ch].id as u32;
    let focus = p.g.aim_point;
    d.take(&p.g, ALL);
    p.frame();
    d.cut();
    d.take(&p.g, ALL);
    d.frame(&p.g, 1.0, focus);
    let whole = man(&d, id).unwrap();
    d.frame(&p.g, 0.5, focus);
    assert_eq!(man(&d, id).unwrap(), whole, "he is drawn between a tick before the cut and the one after it");
}

/// THE RULE: the boxes' lists agree with their head, every static is told as changed once, and a frame whose boxes stand still keeps its grid.
#[test]
fn the_boxes_say_what_they_hold() {
    let mut p = office(5);
    let mut d = Drawn::default();
    p.run(0.2);
    d.take(&p.g, ALL);
    d.frame(&p.g, 1.0, [20.0, 0.0, 14.0]);
    let n = p.g.engine.world.statics.len() as u32;
    assert_eq!((head(&d).statics, head(&d).from, head(&d).to), (n, 0, n), "the first frame tells every static");
    assert_eq!(head(&d).count as usize, d.boxes.geo.len());
    assert_eq!((head(&d).visible + head(&d).visible_cut) as usize, d.boxes.visible.len());
    assert!(head(&d).grid_moved == 1 && head(&d).items > 0 && d.boxes.grid.items.len() == 2 * head(&d).items as usize);
    assert!(d.boxes.visible.iter().all(|w| (w.w & 0xffff) < head(&d).count), "a drawn box is past the list's end");
    d.frame(&p.g, 1.0, [20.0, 0.0, 14.0]);
    assert_eq!((head(&d).from, head(&d).to, head(&d).grid_moved, head(&d).fresh), (0, 0, 0, 0), "the same tick drawn again tells nothing anew");
    // (a section asked for alone is made, and no other)
    let mut one = Drawn::default();
    one.take(&p.g, want::lights);
    one.frame(&p.g, 1.0, [0.0; 3]);
    assert!(!one.lights.rows.is_empty() && one.boxes.geo.is_empty() && one.men.men.is_empty());
}

/// THE RULE: the smoke the picture is bent toward is the smoke eyes see through: read through its rows, a line lets through what the game says it does; a game put back says nothing of it until its ways are known again, and then the smoke that stands there.
/// A FAILURE MEANS: the drawn smoke follows something no eye looks through.
#[test]
fn the_smoke_drawn_is_bent_toward_the_smoke_eyes_see_through() {
    use game::smoke_types::SmokeEmitter;
    let mut p = office(5);
    p.sam_unhurt();
    p.run(0.2);
    let mut d = Drawn::default();
    d.take(&p.g, want::smoke);
    assert!(d.smoke.bricks.is_empty(), "no smoke, and a brick of it is told");
    let [x, z] = p.sam_at();
    let can = SmokeEmitter { pos: [x, 0.2, z], dir: [0.0, 1.0, 0.0], speed: 1.9, radius: 0.2, density: 95.0, temperature: 1.6, ttl: 20.0, ..Default::default() };
    game::smoke_types::smoke_system_like_emit(&mut p.g, &can);
    let lines = |x: f64, z: f64| (0..60).map(move |i| { let (a, y) = (i as f64 * 0.7, 0.3 + 0.35 * (i % 8) as f64); let (c, s) = (game::m::cos(a), game::m::sin(a)); ([x - 4.0 * c, y, z - 4.0 * s], [x + 4.0 * c, y, z + 4.0 * s]) });
    // the game's answer: the cell each quarter metre lies in, read from the rows
    let read = |d: &Drawn, a: [f64; 3], b: [f64; 3]| {
        let h = d.smoke.head[0];
        let len = game::v3::len(game::v3::sub(b, a));
        let n = (len * 4.0).ceil();
        let sum: f64 = (0..n as usize).map(|i| {
            let q: [f64; 3] = core::array::from_fn(|k| (a[k] + (b[k] - a[k]) * ((i as f64 + 0.5) / n)) / 0.5);
            let [cx, cy, cz] = q.map(|v| v.floor() as i64);
            if cx < 0 || cz < 0 || !(0..8).contains(&cy) || cx >= 4 * h.wide as i64 || cz >= 4 * h.deep as i64 { return 0.0; }
            let row = d.smoke.at[(cz / 4 * h.wide as i64 + cx / 4) as usize].w as usize;
            if row == 0 { 0.0 } else { d.smoke.bricks[row - 1].rho[((cy * 4 + cz % 4) * 4 + cx % 4) as usize] as f64 }
        }).sum();
        (-6.0 * sum * len / n).exp()
    };
    let agree = |d: &Drawn, p: &Play, when: &str| {
        let mut hidden = 0;
        for (a, b) in lines(x, z) {
            let (told, game) = (read(d, a, b), game::sense::smoke_between(&p.g, a, b));
            assert!((told - game).abs() < 1e-4, "{when}: the rows let {told:.4} through {a:?} {b:?} and the game {game:.4}");
            hidden += (game < 0.3) as u32;
        }
        hidden
    };
    p.run(2.0);
    let then = p.g.clone();
    let kept = game_start::block::bytes(p.g.sim).to_vec();
    d.take(&p.g, want::smoke);
    let early = agree(&d, &p, "two seconds in");
    let ways = |d: &Drawn| { let n = |bit: u32| d.smoke.bricks.iter().map(|b| b.open.iter().map(|w| (w & (bit * 0x0101_0101)).count_ones()).sum::<u32>()).sum::<u32>(); [n(1), n(2), n(4), n(8)] };
    let of = 128 * d.smoke.bricks.len() as u32;
    let told = ways(&d);
    assert!(told[3] == of && told[..3].iter().all(|n| *n > of / 4 && *n < of), "an office of rooms, and of {of} cells these ways are open: {told:?}");
    p.run(6.0);
    d.take(&p.g, want::smoke);
    let late = agree(&d, &p, "eight seconds in");
    assert!(late > early, "the scene: the cloud was to grow ({early} of 60 lines hidden, then {late})");
    p.g = then;
    sim::state::put(p.g.sim, &kept);
    d.cut();
    d.take(&p.g, want::smoke);
    assert!(d.smoke.head.is_empty(), "put back, and its ways are told before they were asked");
    p.run(0.1);
    d.take(&p.g, want::smoke);
    assert!(agree(&d, &p, "put back").abs_diff(early) < late - early, "the cloud put back is not the one of two seconds in");
    let of = 128 * d.smoke.bricks.len() as u32;
    assert!(ways(&d)[3] == of && ways(&d)[0] > of / 4, "put back, and the ways are not told");
}

/// THE RULE: a source says whether the game holds its smoke, and the game holds what it says.
#[test]
fn a_source_says_whether_the_game_holds_its_smoke() {
    use game::smoke_types::SmokeEmitter;
    let mut p = office(5);
    p.run(0.2);
    let can = SmokeEmitter { pos: [8.0, 0.2, 8.0], dir: [0.0, 1.0, 0.0], speed: 1.9, radius: 0.2, density: 95.0, temperature: 1.6, ttl: 20.0, ..Default::default() };
    for (e, held, what) in [
        (can.clone(), true, "a canister"),
        (SmokeEmitter { look: Some(true), ..can.clone() }, false, "a look of one"),
        (SmokeEmitter { radius: 0.08, density: 24.0, ttl: 0.08, ..can.clone() }, false, "a shot's gas"),
    ] {
        let before = sim::bricks::emitting(p.g.sim);
        game::smoke_types::smoke_system_like_emit(&mut p.g, &e);
        game::out_events::push_smoke_system_like_emit(&mut p.g, &e);
        assert_eq!(sim::bricks::emitting(p.g.sim) > before, held, "{what}: the game's own smoke");
        let mut d = Drawn::default();
        d.take(&p.g, want::events);
        assert_eq!(d.events.sources.last().expect("the source is told").says & 8 != 0, held, "{what}: what the picture is told");
    }
}

/// THE RULE: a tracer's head that stops in something solid throws its ember, once, as it arrives: the sparks the table asks for. In a man, or on nothing, it just ends.
#[test]
fn a_tracer_that_stops_in_something_solid_throws_its_ember() {
    use game::tracers::TracerImpact;
    for (impact, solid) in [(TracerImpact::Solid, true), (TracerImpact::Flesh, false), (TracerImpact::None, false)] {
        let mut p = office(6);
        p.run(0.2);
        let want_sparks = p.g.tracers.params.ember_sparks;
        assert!(want_sparks > 0.0, "the table asks for no ember");
        game::tracers::tracers_spawn(&mut p.g, [8.0, 1.4, 8.0], [1.0, 0.0, 0.0], 6.0, impact, [1.0, 0.6, 0.3], 0.0);
        let mut out = Vec::new();
        for _ in 0..4 { game::tracers::tracers_update(&mut p.g, 0.1, &mut out, None); }
        let mut d = Drawn::default();
        d.take(&p.g, want::events);
        let spark = game::render_particles::PKind::Spark as u32;
        let (sparks, thrown) = d.events.emits.iter().filter(|e| e.kind == spark).fold((0.0, 0), |(n, k), e| (n + e.count as f64, k + 1));
        assert_eq!((sparks, thrown), if solid { (want_sparks, 1) } else { (0.0, 0) }, "{impact:?}");
    }
}
