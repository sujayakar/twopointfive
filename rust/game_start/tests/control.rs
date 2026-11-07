//! WHO HAS A MAN (rust/game/src/control/): tests that stand a scene and hold what the list of rules decides. README.md beside this crate says
//! how such a test is written; each holds a rule over three seeds, each seed another scene.
use game::game::GuardState;
use game_start::play::{dist, Opts, Play};
use game_start::scene::{Axis, Scene};

const SEEDS: [u32; 3] = [1, 2, 3];
fn seed(seed: u32) -> Opts { Opts { seed, ..Default::default() } }
/// a number in -1..1 that is a fixed function of the seed and of `k`
fn nudge(seed: u32, k: u32) -> f64 {
    let mut x = seed.wrapping_mul(0x9E37_79B9) ^ k.wrapping_mul(0x85EB_CA6B);
    x ^= x >> 16; x = x.wrapping_mul(0x7FEB_352D); x ^= x >> 15; x = x.wrapping_mul(0x846C_A68B); x ^= x >> 16;
    x as f64 / 2_147_483_648.0 - 1.0
}

/// A lit room 14 m by 8 m with nothing in it: a sentry at its west end looking east, and Sam standing 6 to 7 m in front of him, in the open.
fn open_room(seed: u32) -> Scene {
    let mut s = Scene::new("open-room");
    s.room("room", 10.0, 10.0, 24.0, 18.0).door("south", Axis::X, 18.0, 20.0);
    s.bulb(14.0, 14.0).bulb(20.0, 14.0);
    s.sentry("post", [11.5, 14.25], [24.0, 14.25]).sam(18.0 + 0.5 * nudge(seed, 0), 14.25 + 0.75 * nudge(seed, 1));
    s
}

/// what the game's own line says of who has the man (control::control_line), and its first clause alone: `Kowalski: his own for 1.0 s`
fn line(p: &Play, gd: game::ids::GuardId) -> String { game::control::control_line(&p.g, gd) }
fn has_him(p: &Play, gd: game::ids::GuardId) -> String { line(p, gd).split(" · ").next().unwrap_or("").to_string() }
fn reloading(p: &Play, gd: game::ids::GuardId) -> bool { p.g.gd[gd].reload_t >= 0.0 }

/// THE RULE: a guard in a fight who shoots his gun dry at a man he can see, with nowhere near to hide, kneels where he is while he changes
/// magazine, and stands to the fight again when it is in. While he was shooting he stood.
/// A FAILURE MEANS: the rule `changing magazine in cover` no longer takes a man whose reload begins in sight of his man, or the list no
/// longer asks it, or the record no longer says who has him.
#[test]
fn a_guard_who_runs_dry_in_the_open_kneels_to_change_magazine() {
    for s in SEEDS {
        let mut p = Play::scene(&open_room(s), &seed(s));
        p.g.god_mode = true;   // (the page's own option: Sam is to be shot at for a whole magazine and more)
        let post = p.guard("post");
        p.until(6.0, |p| p.state(post) == GuardState::Alert).unwrap_or_else(|| panic!("seed {s}: he never saw Sam {:.1} m in front of him in light ({})", dist(p.at(post), p.sam_at()), p.roll()));
        // he shoots; while he does nobody has him but himself, and he stands
        let mut stood_shooting = 0;
        let dry = p.until(40.0, |p| { if p.state(post) == GuardState::Alert && !reloading(p, post) && !p.low(post) && has_him(p, post).contains("his own") { stood_shooting += 1; } reloading(p, post) })
            .unwrap_or_else(|| panic!("seed {s}: in 40 s of the fight his gun never ran dry ({})", line(&p, post)));
        assert!(stood_shooting > 30, "seed {s}: before the reload he was to be on his feet in his own fight; that held for {stood_shooting} frames of {:.1} s", dry);
        // the reload: the rule has him and he is down, within a quarter second, and for most of it
        let (mut frames, mut held, mut low) = (0, 0, 0);
        let took = p.until(8.0, |p| { if reloading(p, post) { frames += 1; if has_him(p, post).contains("changing magazine in cover") { held += 1; } if p.low(post) { low += 1; } } !reloading(p, post) })
            .unwrap_or_else(|| panic!("seed {s}: the reload never ended ({})", line(&p, post)));
        assert!(frames > 30, "seed {s}: a reload of {frames} frames is no reload");
        assert!(held * 10 >= frames * 8, "seed {s}: the rule was to have him for the reload; it had him {held} of {frames} frames ({took:.2} s). At the end: {}", line(&p, post));
        assert!(low * 10 >= frames * 7, "seed {s}: he was to be down for the reload; he was {low} of {frames} frames");
        assert!(dist(p.at(post), [11.5, 14.25]) < 7.0 && p.up(post), "seed {s}: he is still in the room and on his feet's count ({})", p.roll());
        // the magazine is in: the rule lets him go within a moment
        p.until(1.0, |p| !has_him(p, post).contains("changing magazine in cover")).unwrap_or_else(|| panic!("seed {s}: a second after the reload the rule still has him ({})", line(&p, post)));
    }
}
