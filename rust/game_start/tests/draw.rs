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
            p.frame_shown(|p| if draw { let focus = p.g.sets[p.g.seat].aim_point; d.take(&p.g, ALL); d.frame(&p.g, 0.5, focus); d.frame(&p.g, 1.0, focus); });
            p.words()
        }).collect::<Vec<_>>()
    };
    let (plain, drawn) = (played(false), played(true));
    assert_eq!(plain.iter().zip(&drawn).position(|(a, b)| a != b), None, "the drawn game parts from the one nobody drew");
}

/// THE RULE: a run watched without the fog and through a tube, what the men the fog leaves out carry and the players' lines made for that picture every frame, is the run nobody watched; and the
/// picture has those men and what they carry.
#[test]
fn watching_a_run_is_no_part_of_it() {
    let played = |watch: bool| {
        let mut p = office(5);
        let mut d = Drawn::default();
        let mut more = (0, 0);
        let words = (0..240).map(|k| {
            if k == 60 { let pl = p.g.player; p.g.pl[pl].nv = true; }
            p.g.god_mode = true;
            if watch { p.g.fog_picture.viewer = true; }
            p.frame();
            if watch {
                d.view_as(true, Some(true));
                game::scene_collision::stands(&mut p.g);
                d.watch(&p.g);
                game::scene_collision::unstand(&mut p.g);
                let focus = p.g.sets[p.g.seat].aim_point;
                d.take(&p.g, ALL);
                d.frame(&p.g, 1.0, focus);
                let (men, own) = (d.men.men.len(), d.boxes.own_ids().count());
                d.view_as(false, None);
                d.watch(&p.g);
                d.take(&p.g, ALL);
                d.frame(&p.g, 1.0, focus);
                more = (more.0.max(men - d.men.men.len()), more.1.max(own.saturating_sub(d.boxes.own_ids().count())));
            }
            p.words()
        }).collect::<Vec<_>>();
        (words, more)
    };
    let ((plain, _), (watched, more)) = (played(false), played(true));
    assert_eq!(plain.iter().zip(&watched).position(|(a, b)| a != b), None, "the watched run parts from the one nobody watched");
    assert!(more.0 >= 2 && more.1 >= 2 * more.0, "men the fog leaves out, and the boxes of them: {more:?}");
}

/// THE RULE: half way between two ticks a man stands half way between where the two left him, his proxies with him; at the whole he stands where the last left him.
#[test]
fn a_frame_between_two_ticks_stands_between_them() {
    let mut p = office(6);
    let mut d = Drawn::default();
    p.key_down("KeyW");
    p.run(0.5);
    let id = p.g.ch[p.g.pl[p.g.player].ch].id as u32;
    let focus = p.g.sets[p.g.seat].aim_point;
    d.take(&p.g, ALL);
    d.frame(&p.g, 1.0, focus);
    let before = man(&d, id).expect("Sam is drawn");
    p.frame_shown(|p| { d.take(&p.g, ALL); d.frame(&p.g, 1.0, focus); });
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
    let focus = p.g.sets[p.g.seat].aim_point;
    d.take(&p.g, ALL);
    p.frame_shown(|p| { d.cut(); d.take(&p.g, ALL); d.frame(&p.g, 1.0, focus); });
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

/// THE RULE: the smoke the picture is bent toward is the smoke eyes see through: read through its rows, a line lets through what the game says it does; of a game put back its ways are asked before it is first drawn, so the picture is told the whole of the smoke that stands there at once; and the asking is no part of the game.
/// A FAILURE MEANS: the drawn smoke follows something no eye looks through; a load is drawn with no smoke, or another's; or a game that was drawn goes on otherwise than one nobody drew.
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
    let then = game::savestate::control::Moment::of(&p.g);
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
    let mut again = |p: &mut Play, asked: bool| {
        p.g = then.stands(p.g.sim);
        d.cut();
        if asked {
            assert!(d.smoke.unsaid());
            game::sense::smoke_ways(&mut p.g);
            d.take(&p.g, want::smoke);
            assert!(!d.smoke.unsaid() && !d.smoke.head.is_empty(), "put back, and nothing is told of the smoke that stands there");
            assert_eq!(agree(&d, p, "put back"), early, "the cloud put back is not the one of two seconds in");
            assert_eq!(ways(&d), told, "the ways put back are not the ones of two seconds in");
        }
        (0..60).map(|_| { p.frame(); p.words() }).collect::<Vec<_>>()
    };
    let (plain, asked) = (again(&mut p, false), again(&mut p, true));
    assert_eq!(plain.iter().zip(&asked).position(|(a, b)| a != b), None, "the game whose ways were asked parts from the one nobody asked");
}

/// THE RULE: a picture of a game put back is told what lets smoke out there, where and as old as it is, once for every source: at its first tick however often that is told, with the ids it had, and
/// never again; one that this very tick began is told by its own event alone.
/// A FAILURE MEANS: a can thrown before a save stands without its plume after the load, or pours twice.
#[test]
fn a_game_put_back_tells_what_lets_smoke_out() {
    let mut p = office(5);
    p.sam_unhurt();
    p.run(0.2);
    let mut d = Drawn::default();
    let [x, z] = p.sam_at();
    game::game::game_drop_throwable(&mut p.g, game::game::ThrowKind::Smoke, [x + 1.0, 0.0, z]);
    let told = |d: &Drawn| d.events.sources.iter().filter(|s| s.says & 8 != 0).map(|s| (s.tick, s.serial, s.pos.map(f32::to_bits), s.radius, s.density, s.ttl, s.age)).collect::<Vec<_>>();
    let lets = |p: &Play| sim::bricks::letting(p.g.sim).map(|e| e.at.map(|v| (v as f32).to_bits())).collect::<Vec<_>>();
    let mut first = Vec::new();
    for _ in 0..300 { p.frame_shown(|p| { d.take(&p.g, want::events); first = told(&d); }); if !first.is_empty() { break; } }
    assert_eq!(first.len(), 1, "the can's own event");
    let began = game::savestate::control::Moment::of(&p.g);
    // (the tick that began it, put back: as its host keeps it, with nothing left to say)
    p.g = began.stands(p.g.sim);
    p.g.out.clear();
    d.cut();
    d.take(&p.g, want::events);
    let at_once = told(&d);
    assert!(at_once.len() == 1 && at_once[0].2 == first[0].2, "put back at the tick it began: {at_once:?}");
    p.run(4.0);
    let then = game::savestate::control::Moment::of(&p.g);
    let of = lets(&p);
    p.run(3.0);
    p.g = then.stands(p.g.sim);
    p.g.out.clear();
    d.cut();
    d.take(&p.g, want::events);
    let again = told(&d);
    assert_eq!(again.iter().map(|s| s.2).collect::<Vec<_>>(), of, "the sources told are not the game's");
    let can = again.iter().find(|s| s.2 == first[0].2).expect("the can pours on");
    assert!((can.4 - first[0].4).abs() < 1e-3 * first[0].4 && can.3 == first[0].3 && can.5 == first[0].5, "what it lets out: {can:?}, and as it began {:?}", first[0]);
    assert!((can.6 - 4.0).abs() < 0.15, "four seconds old: {}", can.6);
    d.take(&p.g, want::events);
    assert_eq!(told(&d), again, "the same tick told again");
    p.frame_shown(|p| d.take(&p.g, want::events));
    assert!(told(&d).is_empty(), "the tick after: {:?}", told(&d));
}

/// THE RULE: what sounds for as long as it goes on is told as it stands, every tick and at the first of a game put back: a fire that burns, a can that pours with its age and what it has left; one that
/// is over is told no more. No event begins or ends one.
/// A FAILURE MEANS: after a load a can pours or a fire burns in silence, or hisses on where there is none.
#[test]
fn what_goes_on_is_heard_as_it_stands() {
    let mut p = Play::level("pass.json", false, &Opts::default());
    p.g.god_mode = true;
    p.run(0.2);
    let mut d = Drawn::default();
    let heard = |d: &Drawn, name: &str| d.events.lasting.iter().filter(|l| d.events.names[l.name as usize] == name).map(|l| (l.id, l.at, if l.age.is_nan() { -1.0 } else { l.age }, if l.left.is_nan() { -1.0 } else { l.left })).collect::<Vec<_>>();
    d.take(&p.g, want::events);
    let fires = heard(&d, "fireBed");
    assert!(!fires.is_empty() && heard(&d, "smokeHiss").is_empty(), "the pass has a brazier that burns, and no can: {fires:?}");
    let [x, z] = p.sam_at();
    game::game::game_drop_throwable(&mut p.g, game::game::ThrowKind::Smoke, [x + 1.0, 0.0, z]);
    let mut events = Vec::new();
    let mut step = |p: &mut Play, d: &mut Drawn, n: usize| for _ in 0..n { p.frame_shown(|p| d.take(&p.g, want::events)); events.extend(d.events.sounds.iter().map(|s| d.events.names[s.name as usize].clone())); };
    step(&mut p, &mut d, 300);
    let at = heard(&d, "smokeHiss");
    assert!(at.len() == 1 && at[0].2 > 1.0 && (at[0].2 + at[0].3 - 32.0).abs() < 0.1, "the can, its age and what it has left: {at:?}");
    let then = game::savestate::control::Moment::of(&p.g);
    step(&mut p, &mut d, 32 * 60);
    assert!(heard(&d, "smokeHiss").is_empty() && heard(&d, "fireBed").len() == fires.len(), "burnt out, and the brazier burns on");
    assert!(events.iter().any(|n| n == "smokePop") && !events.iter().any(|n| n == "smokeHiss" || n == "fireBed"), "told by events: {events:?}");
    p.g = then.stands(p.g.sim);
    d.cut();
    d.take(&p.g, want::events);
    assert_eq!(heard(&d, "smokeHiss"), at, "put back, and the can is not heard as it stood");
    assert_eq!(heard(&d, "fireBed"), fires);
    drop(p);
    // what hums in the office's rooms, off its own file: with the mains out the wind alone
    let mut p = office(5);
    p.run(0.2);
    let hums = |p: &Play, d: &mut Drawn| { d.take(&p.g, want::events); ["hum", "servers", "buzz", "wind"].map(|n| heard(d, n).len()) };
    let mut d = Drawn::default();
    assert_eq!(hums(&p, &mut d), [1, 1, 2, 2]);
    game::lighting::set_blackout(&mut p.g, 9.0, game::lighting::SetBlackoutCause::Ocp);
    p.run(0.1);
    assert_eq!(hums(&p, &mut d), [0, 0, 0, 2], "with the mains out");
}

/// THE RULE: the game's smoke has a color by its bricks: dust's where a blast threw it, a can's grey everywhere else, less of it where the two mix; and a game put back has it as it stood.
/// A FAILURE MEANS: a wall's dust hangs grey after a load.
#[test]
fn the_smoke_keeps_its_color() {
    let mut p = office(5);
    p.sam_unhurt();
    p.run(0.2);
    let mut d = Drawn::default();
    let [x, z] = p.sam_at();
    let dyes = |d: &Drawn| d.smoke.bricks.iter().map(|b| b.dye).collect::<Vec<_>>();
    let can = game::smoke_types::SmokeEmitter { pos: [x, 0.2, z], dir: [0.0, 1.0, 0.0], speed: 1.9, radius: 0.2, density: 95.0, temperature: 1.6, ttl: 4.0, ..Default::default() };
    game::smoke_types::smoke_system_like_emit(&mut p.g, &can);
    p.run(1.5);
    d.take(&p.g, want::smoke);
    assert!(!dyes(&d).is_empty() && dyes(&d).iter().all(|w| *w == 0), "a can's smoke has no color of its own: {:x?}", dyes(&d));
    let dust = game::smoke_types::SmokeEmitter { tint: Some([0.8, 0.6, 0.4]), ..can.clone() };
    game::smoke_types::smoke_system_like_emit(&mut p.g, &dust);
    p.run(1.0);
    d.take(&p.g, want::smoke);
    let mixed = dyes(&d);
    let blue = mixed.iter().map(|w| w >> 24).max().unwrap();
    assert!(mixed.iter().all(|w| w & 255 == 0) && (10..153).contains(&blue), "dust let into a can's smoke, at most part of it: {mixed:x?}");
    assert!(mixed.iter().all(|w| (w >> 24) >= (w >> 16 & 255) && (w >> 16 & 255) >= (w >> 8 & 255)), "its color is the dust's: {mixed:x?}");
    let then = game::savestate::control::Moment::of(&p.g);
    p.run(6.0);
    d.take(&p.g, want::smoke);
    assert!(dyes(&d).iter().filter(|w| **w != 0).count() > mixed.iter().filter(|w| **w != 0).count(), "the smoke that spread took its color along: {:x?}", dyes(&d));
    p.g = then.stands(p.g.sim);
    d.cut();
    game::sense::smoke_ways(&mut p.g);
    d.take(&p.g, want::smoke);
    assert_eq!(dyes(&d), mixed, "put back, and its color is not as it stood");
    // (a source that still lets out is told again in its color)
    d.take(&p.g, want::events);
    assert!(d.events.sources.iter().any(|s| s.tint == [0.8, 0.6, 0.4]) && d.events.sources.iter().any(|s| s.tint[0].is_nan()), "{:?}", d.events.sources.iter().map(|s| s.tint).collect::<Vec<_>>());
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
        for _ in 0..4 { game::tracers::tracers_update(&mut p.g, 0.1, &mut out); }
        let mut d = Drawn::default();
        d.take(&p.g, want::events);
        let spark = game::render_particles::PKind::Spark as u32;
        let (sparks, thrown) = d.events.emits.iter().filter(|e| e.kind == spark).fold((0.0, 0), |(n, k), e| (n + e.count as f64, k + 1));
        assert_eq!((sparks, thrown), if solid { (want_sparks, 1) } else { (0.0, 0) }, "{impact:?}");
    }
}
