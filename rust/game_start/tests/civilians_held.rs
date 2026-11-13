//! A CIVILIAN ON THE FLOOR: frightened she runs to a phone, once; in Sam's arm she talks, told to stay down she stays down, and she tells the first man who finds her.
mod stealth_ask;
use game::civilians::{CivPanicWhat, CivState};
use game::game::{DownBy, DownKind, GuardState, InteractableKind, Verb, With};
use game::ids::GuardId;
use game_start::play::{Opts, Play};
use stealth_ask::StealthAsk;

const SEEDS: [u32; 3] = [1, 2, 3];

/// The office as it stands, Sam out on the verge where nobody sees him.
fn office(seed: u32) -> (Play, GuardId) {
    let mut p = Play::level("office.json", false, &Opts { seed, ..Default::default() });
    p.sam_unhurt();
    p.run(0.1);
    p.sam_put([2.0, 2.0]);
    p.g.player_invisible = true;
    let marta = p.g.civilians[0];
    (p, marta)
}

/// THE RULE: a frightened civilian runs for the nearer phone and rings it in: on the line inside twenty-five seconds, the line takes its seconds, one control line and one man sent however long she stands there, then she cowers.
/// CONTROL: a civilian nobody frightened rings nobody.
/// A FAILURE MEANS: a witness can't call the floor up, or she calls it up over and over.
#[test]
fn a_frightened_civilian_rings_it_in_once_and_then_cowers() {
    for s in SEEDS {
        let (mut p, m) = office(s);
        p.run(3.0);
        let line = p.g.k.civ.phone_secs;
        let word = game::civilians::civ_panic(&mut p.g, Some(m), Some(CivPanicWhat::Phone));
        assert!(word.contains("runs for"), "seed {s}: she was not sent: {word}");
        p.until(25.0, |p| p.civ(m).state == CivState::Phone).unwrap_or_else(|| panic!("seed {s}: twenty-five seconds and she is not on a phone: {:?}", p.civ(m).state));
        let t = p.until(line + 1.0, |p| p.civ(m).phoned).unwrap_or_else(|| panic!("seed {s}: the call never landed"));
        assert!((t - line).abs() < 0.1, "seed {s}: on the line {t:.2} s, the rule asks {line}");
        p.until(25.0, |p| p.civ(m).state == CivState::Cower).unwrap_or_else(|| panic!("seed {s}: she never cowered: {:?}", p.civ(m).state));
        p.run(5.0);
        let on_way: Vec<_> = p.said.iter().filter(|l| l.radio && l.text.contains("on my way")).map(|l| l.who).collect();
        assert_eq!(on_way.len(), 1, "seed {s}: the men sent: {on_way:?}");
        assert_eq!(p.g.stats.civilians.phoned, 1, "seed {s}: calls");
        drop(p);

        let (mut p, m) = office(s);
        p.run(15.0);
        assert!(!p.civ(m).phoned && p.said.iter().all(|l| !l.text.contains("on my way")), "seed {s}: CONTROL: frightened by nothing she rang it in");
    }
}

/// THE RULE: a civilian in Sam's arm talks, told to stay down she stays down, out of his arm and tells nobody for a quiet minute; the calm man stood beside her asks, and she tells him: he goes looking.
/// CONTROL is the minute itself: the man told is the one who found her, not the time.
/// A FAILURE MEANS: a civilian let go calls the floor up at once, or nobody ever finds out.
#[test]
fn a_held_civilian_talks_stays_down_and_tells_the_first_man_who_finds_her() {
    for s in SEEDS {
        let (mut p, m) = office(s);
        p.run(2.0);
        let men = p.g.guards.clone();
        for (i, &o) in men.iter().enumerate() { p.parked(o); p.man_stands(o, [37.5, 6.0 + i as f64 * 1.5]); }
        p.g.player_invisible = false;
        let at = p.at(m);
        let c = p.g.gd[m].ch;
        let y = p.g.ch[c].body_yaw;
        p.sam_put([at[0] - y.sin() * 0.7, at[1] - y.cos() * 0.7]);
        let me = p.g.player;
        p.g.pl[me].holstered = true;
        p.run(0.05);
        assert!(p.grab_row(m), "seed {s}: from close behind her there is no grab");
        game::player::start_grab(&mut p.g, m);
        p.run(1.4);
        assert!(p.in_arm() == Some(m), "seed {s}: she is not in his arm");
        for _ in 0..2 {
            assert!(p.row_taken(InteractableKind::Held, Verb::Question), "seed {s}: no row to ask her");
            p.run(4.0);
        }
        assert!(p.g.pl[me].codes.has(String::from("pad_side")), "seed {s}: asked twice, the side door's digits are not Sam's");
        assert!(p.row_taken(InteractableKind::Held, Verb::Comply), "seed {s}: no row to tell her to stay down");
        p.run(0.05);
        assert!(p.in_arm().is_none() && p.civ(m).complied && p.civ(m).state == CivState::Cower, "seed {s}: told to stay down: {:?}", p.civ(m).state);
        p.sam_put([2.0, 2.0]);
        p.g.player_invisible = true;
        p.run(60.0);
        assert!(!p.civ(m).reported && p.civ(m).state == CivState::Cower, "seed {s}: a quiet minute on she has told, or moved: {:?}", p.civ(m).state);
        let n = men[1];
        let at = p.at(m);
        p.let_go(n);
        p.pin_man(n, [at[0], at[1] + 1.5], std::f64::consts::PI, GuardState::Patrol, 0.0);
        p.let_go(n);
        p.until(9.0, |p| p.civ(m).reported).unwrap_or_else(|| panic!("seed {s}: a calm man beside her and she told him nothing: {:?} {}", p.civ(m).state, p.why(n)));
        p.run(0.2);
        assert!(p.state(n) != GuardState::Patrol, "seed {s}: told, he went on with his round");
    }
}

/// THE RULE: a civilian on the floor is found by the eye that finds a man there: in the dark 7 m off nobody makes either out, with the light on both are found.
/// A FAILURE MEANS: she is found in the dark by a man who walks past a colleague of his there.
#[test]
fn a_civilian_on_the_floor_is_found_as_a_man_is() {
    for civilian in [true, false] {
        for lit in [false, true] {
            let (mut p, _) = office(1);
            let men = p.g.guards.clone();
            let dark = |p: &mut Play| for &x in &men { let lamp = p.g.gd[x].flashlight; p.g.tab.light[lamp].enabled = false; };
            dark(&mut p);
            for (i, &x) in men.iter().enumerate() { p.man_stands(x, [1.5, 4.0 + i as f64]); }
            p.run(0.1);
            let lies = |p: &mut Play, at: [f64; 2]| { let e = game::guards::light_estimate_at(&mut p.g, [at[0], 0.35, at[1]], None, None, None); game::game::game_visibility_of(&p.g, e) };
            let mut spot: Option<(f64, [f64; 2], [f64; 2])> = None;
            for x in 8..36 { for z in 4..24 {
                let (at, from) = ([x as f64 + 0.25, z as f64 + 0.25], [x as f64 + 7.25, z as f64 + 0.25]);
                if from[0] > 36.0 || [at, from].iter().any(|q| game::scene_collision::nav_grid_is_blocked(&p.g, q[0], q[1])) || game::scene_collision::segment_blocked(&p.g, [from[0], 1.65, from[1]], [at[0], 0.4, at[1]], None, None, None) { continue; }
                let v = lies(&mut p, at);
                if spot.map_or(true, |s| if lit { v > s.0 } else { v < s.0 }) { spot = Some((v, at, from)); }
            } }
            let (light, at, from) = spot.expect("a clear 7 m of the floor");
            assert!(if lit { light > 0.4 } else { light < 0.02 }, "the staging is wrong: the floor there reads {light:.2}");
            let (finder, body) = (men[0], if civilian { p.g.civilians[0] } else { men[1] });
            p.man_stands(body, at);
            p.frame();
            p.man_stands(body, at);
            p.frame();
            if civilian { game::civilians::civilian_down(&mut p.g, body, [1.0, 0.0, 0.0], true, DownKind::Choked, DownBy::Player, With::Hand); } else { p.falls_as(body, DownKind::Choked, true, DownBy::Player); }
            let found = p.until(5.0, |p| { dark(p); p.pin_man(finder, from, -std::f64::consts::FRAC_PI_2, GuardState::Patrol, 0.0); p.g.gd[body].found });
            assert_eq!(found.is_some(), lit, "a {} on the floor, {}: found after {found:?}", if civilian { "civilian" } else { "man" }, if lit { "lit" } else { "in the dark" });
        }
    }
}
