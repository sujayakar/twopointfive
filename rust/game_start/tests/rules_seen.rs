//! WHAT A MAN'S EYE TAKES IN: the cone and its reach, what hides a man from it, and how fast a man it has builds notice. Each test stands a few men and a lamp,
//! asks the eye the question the game asks it, and holds the answer beside a control that differs in the one thing the rule is about.
mod world_ask;
use game::ids::GuardId;
use game_start::play::{nudge, Opts, Play};
use game_start::scene::{Axis, Scene};
use world_ask::{a_box, scene_plus, scene_with, WorldAsk, YardAsk};

const SEEDS: [u32; 3] = [1, 2, 3];
fn seed(seed: u32) -> Opts { Opts { seed, ..Default::default() } }

/// A long hall, a sentry at its west end looking east down its length, Sam by him. A bulb every six metres, or none lit.
fn hall(s: u32, lit: bool) -> Scene {
    let z = 13.0 + 0.4 * nudge(s, 0);
    let mut sc = Scene::new("hall");
    sc.room("hall", 2.0, 9.0, 38.0, 17.0).door("gate", Axis::X, 17.0, 20.0);
    for x in [8.0, 14.0, 20.0, 26.0, 32.0, 37.0] { if lit { sc.bulb(x, z); } else { sc.bulb_off(x, z); } }
    sc.sentry("post", [4.25, z], [38.0, z]).sam(10.0, z);
    sc
}
/// A hall stood: the men do not think, so that every question finds the eye where the scene put it.
fn still(mut p: Play) -> (Play, GuardId) {
    p.sam_unhurt();
    p.g.ai_enabled = false;
    let m = p.guard("post");
    p.run(0.5);
    (p, m)
}
/// Sam is put here, and stands long enough for the light meter to settle on the place.
fn stands(p: &mut Play, at: [f64; 2]) { p.sam_put(at); p.run(1.0); }
/// The eye's own question.
fn sees(p: &mut Play, m: GuardId) -> bool { let c = p.sam_body(); game::guards::can_see_char(&mut p.g, m, c).visible }
/// A place `metres` from the man, `deg` degrees round from where he looks.
fn off_his_look(p: &Play, m: GuardId, metres: f64, deg: f64) -> [f64; 2] {
    let (at, l) = (p.at(m), game::guards::look_of(&p.g, p.g.gd[m].ch));
    let y = l[0].atan2(l[2]) + deg.to_radians();
    [at[0] + metres * y.sin(), at[1] + metres * y.cos()]
}

/// A can venting here, by the call every source of smoke in the rules goes by.
fn can(p: &mut Play, at: [f64; 3], secs: f64) {
    let e = game::smoke_types::SmokeEmitter { pos: at, dir: [0.0, 1.0, 0.0], speed: 1.9, radius: 0.2, density: 95.0, temperature: 1.6, ttl: secs, ..Default::default() };
    game::smoke_types::smoke_system_like_emit(&mut p.g, &e);
}
/// His calm head looks about, so a question of the cone is asked for some seconds: does he ever see Sam, standing here.
fn ever_sees(p: &mut Play, m: GuardId, at: [f64; 2], secs: f64) -> bool {
    stands(p, at);
    p.until(secs, |p| sees(p, m)).is_some()
}

/// THE RULE: a man in full light is seen out to the far reach and no further, and a man in the dark out to the short reach and no further.
/// A FAILURE MEANS: anyone in the cone is seen whatever the distance, so a level has no ground too far off to cross.
#[test]
fn a_man_past_the_eyes_reach_is_not_seen() {
    for s in SEEDS {
        let z = 13.0 + 0.4 * nudge(s, 0);
        for (lit, reach) in [(true, Reach::Far), (false, Reach::Short)] {
            let (mut p, m) = still(Play::scene(&hall(s, lit), &seed(s)));
            let k = p.g.k.guard_eye;
            let (metres, x0) = (if matches!(reach, Reach::Far) { k.far_range } else { k.range }, p.at(m)[0]);
            let inside = ever_sees(&mut p, m, [x0 + metres - 2.5, z], 6.0);
            let light = p.sam_light();
            assert!(if lit { light >= k.far_hi } else { light < k.far_lo }, "seed {s}: the staging: the light on him is {light:.2}");
            assert!(inside, "seed {s}, lit {lit}: 2.5 m inside the {metres} m reach he is never seen");
            let outside = ever_sees(&mut p, m, [x0 + metres + 2.5, z], 6.0);
            assert!(!outside, "seed {s}, lit {lit}: 2.5 m past the {metres} m reach he is seen");
        }
    }
}
enum Reach { Far, Short }

/// THE RULE: close by, an eye takes in far more of the room than its cone: a man well out to the side is seen near and not further off, and one square behind is not seen at all.
/// A FAILURE MEANS: a man can stand at a guard's elbow unseen, or the near field has grown to the whole circle.
#[test]
fn close_by_the_eye_sees_wide_and_further_off_it_does_not() {
    for s in SEEDS {
        let (mut p, m) = still(Play::scene(&hall(s, true), &seed(s)));
        let k = p.g.k.guard_eye;
        let side = (k.half_deg + k.near_deg) / 2.0;
        assert!(side > k.half_deg + 15.0, "the staging: {side} degrees is not well outside his cone");
        let question = |p: &mut Play, metres: f64, deg: f64| -> bool {
            let mut seen = 0;
            for flip in [1.0, -1.0, 1.0, -1.0] {
                let at = off_his_look(p, m, metres, deg * flip);
                if !(9.3..16.7).contains(&at[1]) { continue; }
                p.sam_put(at);
                p.frame();
                p.frame();
                if sees(p, m) { seen += 1; }
            }
            seen > 0
        };
        assert!(question(&mut p, k.near_m - 1.0, side), "seed {s}: {side} degrees round and {} m off he is not seen", k.near_m - 1.0);
        assert!(!question(&mut p, k.near_m + 1.0, side), "seed {s}: {side} degrees round and {} m off he is seen", k.near_m + 1.0);
        assert!(!question(&mut p, k.near_m - 1.0, 170.0), "seed {s}: square behind him and {} m off he is seen", k.near_m - 1.0);
    }
}

/// THE RULE: a man whose chest is behind a counter and whose head is over it is seen; crouched behind it he is not, and nor is he behind a full wall.
/// A FAILURE MEANS: a head over cover hides the man, so a man who looks over a counter at a guard is safe from him.
#[test]
fn a_man_is_seen_by_his_head_over_a_counter() {
    for s in SEEDS {
        let z = 13.0 + 0.4 * nudge(s, 0);
        let counter = |top: f64| a_box("counter", [11.45, top / 2.0, z], [0.15, top / 2.0, 3.0], "white", &[]);
        let (mut p, m) = still(scene_with(&hall(s, true), vec![counter(1.38)], &seed(s)));
        let seen = ever_sees(&mut p, m, [12.0, z], 6.0);
        let (c, eye) = (p.sam_body(), p.joint(p.g.gd[m].ch, game::character::Bone::Head));
        let (chest, head) = (p.joint(c, game::character::Bone::Chest), p.joint(c, game::character::Bone::Head));
        assert!(game::scene_collision::segment_blocked(&mut p.g, eye, chest, None, None, None), "seed {s}: the staging: the counter does not cover his chest {chest:?}");
        assert!(!game::scene_collision::segment_blocked(&mut p.g, eye, head, None, None, None), "seed {s}: the staging: the counter covers his head {head:?}");
        assert!(seen, "seed {s}: his head stands over the counter and he is never seen");
        p.sam_crouched(true);
        assert!(p.until(6.0, |p| sees(p, m)).is_none(), "seed {s}: crouched behind the counter he is seen");
        drop(p);
        let (mut p, m) = still(scene_with(&hall(s, true), vec![counter(2.6)], &seed(s)));
        assert!(!ever_sees(&mut p, m, [12.0, z], 6.0), "seed {s}: behind a counter as high as a wall he is seen");
    }
}

fn camera() -> game_start::json::Json {
    use game_start::json::{b, n, nums, obj, s as text};
    obj(vec![("id", text("cam")), ("kind", text("camera")), ("pos", nums(&[7.0, 2.6, 13.0])), ("yaw", n(std::f64::consts::FRAC_PI_2)), ("pitch", n(-0.2)), ("fovDeg", n(70.0)), ("range", n(14.0)), ("mains", b(false))])
}

/// THE RULE: a camera's picture has a top and a bottom: a lit man right under its lens, in line with it on the floor plan, is out of the picture; the same man further down the room is in it.
/// A FAILURE MEANS: the player cannot walk under a camera, the one place its wedge on the floor does not reach.
#[test]
fn a_man_under_a_camera_is_out_of_its_picture() {
    for s in SEEDS {
        let z = 13.0 + 0.3 * nudge(s, 0);
        let scene = || {
            let mut sc = Scene::new("under-the-lens");
            sc.room("room", 6.0, 8.0, 30.0, 18.0).door("yard", Axis::Z, 6.0, 15.4).bulb(8.0, z).bulb(13.0, z);
            sc.sentry("post", [28.25, 9.25], [30.0, 9.25]);
            sc
        };
        let mut sc = scene();
        sc.sam(7.9, 13.0);
        let mut p = scene_plus(&sc, vec![], vec![camera()], &seed(s));
        p.sam_unhurt();
        if let Some(t) = p.always(3.5, |p| p.camera_seen() < 0.05) { panic!("seed {s}: under the lens the camera had him at {t:.2} s, seen {:.2}, Sam's light {:.2}", p.camera_seen(), p.sam_light()); }
        assert!(p.sam_light() > 0.6, "seed {s}: the staging: Sam's light {:.2}", p.sam_light());
        drop(p);
        let mut sc = scene();
        sc.sam(13.0, 13.0);
        let mut p = scene_plus(&sc, vec![], vec![camera()], &seed(s));
        p.sam_unhurt();
        assert!(p.until(3.5, |p| p.camera_seen() > 0.2).is_some(), "seed {s}: down the room in its picture and it has nothing, Sam's light {:.2}", p.sam_light());
    }
}

/// THE RULE: thick smoke between a man and an eye hides him whatever the line says, and thin smoke does not hide him but slows the notice he builds.
/// A FAILURE MEANS: a smoke can buys the team nothing against an eye, or smoke counts only once it is thick.
#[test]
fn smoke_hides_a_man_and_thin_smoke_slows_the_eye() {
    for s in SEEDS {
        let z = 13.0 + 0.4 * nudge(s, 0);
        let (mut p, m) = still(Play::scene(&hall(s, true), &seed(s)));
        assert!(ever_sees(&mut p, m, [14.0, z], 6.0), "seed {s}: CONTROL: in clear air he is seen");
        let d = 9.0;
        let per = |p: &mut Play| { let l = p.sam_light(); game::guards::sight_gain(&mut p.g, m, d) / l.powf(1.5) };
        let clear = per(&mut p);
        can(&mut p, [9.0, 0.2, z], 30.0);
        let (mut thin, mut thick) = (0, 0);
        for _ in 0..(14 * 60) {
            p.frame();
            let t = p.g.gd[m].smoke_trans;
            if (0.35..0.6).contains(&t) {
                thin += 1;
                let (share, top) = (per(&mut p) / clear, (t - 0.3) / 0.5);
                assert!(share < top + 0.25, "seed {s}: {t:.2} of the light gets through and the notice runs at {share:.2} of clear air's");
            }
            if t < 0.28 {
                thick += 1;
                assert!(!sees(&mut p, m), "seed {s}: only {t:.2} of the light gets through and he sees Sam");
            }
        }
        assert!(thin > 0 && thick > 60, "seed {s}: the staging: {thin} frames of thin smoke and {thick} of thick");
    }
}

/// THE RULE: a man down low builds notice slower than the same man standing in the same place and the same light. CONTROL: a man who stands again is back where he was.
/// A FAILURE MEANS: crouching buys nothing against an eye.
#[test]
fn a_man_down_low_is_noticed_slower() {
    for s in SEEDS {
        let z = 13.0 + 0.4 * nudge(s, 0);
        let (mut p, m) = still(Play::scene(&hall(s, true), &seed(s)));
        stands(&mut p, [12.0, z]);
        let per = |p: &mut Play| { let l = p.sam_light(); game::guards::sight_gain(&mut p.g, m, 8.0) / l.powf(1.5) };
        let up = per(&mut p);
        p.sam_crouched(true);
        p.run(0.5);
        let low = per(&mut p);
        p.sam_crouched(false);
        p.run(0.5);
        let again = per(&mut p);
        assert!(low < 0.85 * up, "seed {s}: down low {low:.3} a second, on his feet {up:.3}");
        assert!(low > 0.5 * up && (again - up).abs() < 0.1 * up, "seed {s}: down low {low:.3}, on his feet {up:.3}, on his feet again {again:.3}");
    }
}

/// THE RULE: a lit man at a guard's elbow is noticed at the near rate, faster than the light and the distance would make it just beyond; in the pitch dark only touching him does that.
/// A FAILURE MEANS: a man can walk up to a guard in the light and strike at leisure, or a dark room's guard knows who is beside him before touch.
#[test]
fn at_his_elbow_a_man_is_noticed_at_once() {
    for s in SEEDS {
        let (mut p, m) = still(Play::scene(&hall(s, true), &seed(s)));
        let touch = p.g.k.close.touch_m;
        let at = |p: &Play, metres: f64| { let q = p.at(m); [q[0] + metres, q[1]] };
        let there = at(&p, 1.3);
        stands(&mut p, there);
        let near = game::guards::sight_gain(&mut p.g, m, 1.3);
        let there = at(&p, 2.2);
        stands(&mut p, there);
        let beyond = game::guards::sight_gain(&mut p.g, m, 2.2);
        assert!(near >= 3.4 && beyond < 0.75 * near, "seed {s}: in the light at 1.3 m {near:.2} a second, at 2.2 m {beyond:.2}");
        drop(p);
        let (mut p, m) = still(Play::scene(&hall(s, false), &seed(s)));
        let there = at(&p, 1.3);
        stands(&mut p, there);
        assert!(p.sam_light() < p.g.k.close.dark_at, "seed {s}: the staging: Sam's light {:.2}", p.sam_light());
        let near = game::guards::sight_gain(&mut p.g, m, 1.3);
        let touching = game::guards::sight_gain(&mut p.g, m, touch * 0.8);
        assert!(touching >= 3.4 && near < 0.2, "seed {s}: in the dark, touching him {touching:.2} a second, at 1.3 m {near:.2}");
    }
}

/// A sentry looks east, a colleague stands 5 m in front of him and falls, in the light, with or without a wall between them.
fn body_ahead(s: u32, wall: bool) -> (Play, GuardId, GuardId) {
    let z = 13.0 + 0.4 * nudge(s, 0);
    let mut sc = Scene::new("a-body-ahead");
    sc.room("hall", 2.0, 9.0, 24.0, 17.0).door("gate", Axis::X, 17.0, 20.0).bulb(6.0, z).bulb(12.0, z);
    if wall { sc.wall_z(8.0, 9.0, 17.0); }
    sc.sentry("post", [4.25, z], [24.0, z]).sentry("other", [9.25, z], [24.0, z]);
    let (mut p, m) = still(Play::scene(&sc, &seed(s)));
    let o = p.guard("other");
    p.falls(o, game::game::DownBy::Player);
    p.run(1.0);
    (p, m, o)
}

/// THE RULE: a guard finds a colleague's body he has a line to, and not one behind a wall.
/// A FAILURE MEANS: every body within twelve metres of a guard is found through the walls, so a player can hide nothing.
#[test]
fn a_body_behind_a_wall_is_not_found() {
    for s in SEEDS {
        let (mut p, m, o) = body_ahead(s, false);
        assert!(p.until(6.0, |p| game::guards::body_seen(&mut p.g, m, o, None)).is_some(), "seed {s}: CONTROL: a body in the open 5 m in front of him and in the light is not seen");
        drop(p);
        let (mut p, m, o) = body_ahead(s, true);
        assert!(p.until(6.0, |p| game::guards::body_seen(&mut p.g, m, o, None)).is_none(), "seed {s}: a body behind a wall is seen");
    }
}

/// THE RULE: once the alarm is up an eye is keener by the alarm's own share. CONTROL: below the alarm's first full stage it builds as it did.
/// A FAILURE MEANS: the alarm has stopped costing the player anything at the eye.
#[test]
fn the_alarm_makes_every_eye_keener() {
    for s in SEEDS {
        let z = 13.0 + 0.4 * nudge(s, 0);
        let (mut p, m) = still(Play::scene(&hall(s, true), &seed(s)));
        stands(&mut p, [14.0, z]);
        let calm = game::guards::sight_gain(&mut p.g, m, 9.0);
        game::guards::raise_escalation(&mut p.g, 0.5, None, None, None);
        let half = game::guards::sight_gain(&mut p.g, m, 9.0);
        game::guards::raise_escalation(&mut p.g, 1.0, None, None, None);
        let raised = game::guards::sight_gain(&mut p.g, m, 9.0);
        assert!(raised > calm * (1.0 + (game::guards::KEEN - 1.0) * 0.6), "seed {s}: the alarm up {raised:.3} a second, calm {calm:.3} (keener by {})", game::guards::KEEN);
        assert!((half - calm).abs() < 0.02 * calm, "seed {s}: under the alarm's first stage {half:.3}, calm {calm:.3}");
    }
}

/// A sentry who thinks, his attention on nothing; Sam stood where nobody counts him. A clack ahead takes it.
fn sentry_drawn(s: u32, sam: fn(&Play, GuardId) -> [f64; 2]) -> (f64, f64) {
    let mut p = Play::scene(&hall(s, true), &seed(s));
    p.sam_unhurt();
    p.sam_unseen();
    let m = p.guard("post");
    p.run(0.5);
    let at = sam(&p, m);
    stands(&mut p, at);
    let before = game::guards::sight_gain(&mut p.g, m, 8.0);
    let thing = p.ahead(m, 6.0, 0.0);
    p.clack_at(thing);
    p.run(0.15);
    assert!(p.held(m).is_some(), "seed {s}: the staging: the clack took nothing");
    (before, game::guards::sight_gain(&mut p.g, m, 8.0))
}

/// THE RULE: a man's attention on a thing narrows his eye: a man well to the side of it builds notice at the side's share; a man beyond it, on the line he looks along, as he did.
/// A FAILURE MEANS: a lure no longer lets anybody slip past behind the guard it took.
#[test]
fn what_he_does_not_attend_to_he_notices_less() {
    for s in SEEDS {
        let (before, after) = sentry_drawn(s, |p, m| p.ahead(m, 0.5, 3.5));
        let side = game::knobs::AttentionK::page().side_k;
        assert!(after < before * (side + 1.0) / 2.0, "seed {s}: to the side of what took him {after:.3} a second, before it {before:.3} (the side's share is {side})");
        let (before, after) = sentry_drawn(s, |p, m| p.ahead(m, 9.0, 0.0));
        assert!(after > before * 0.8, "seed {s}: CONTROL: on the line to what took him {after:.3} a second, before it {before:.3}");
    }
}

/// THE RULE: a man with a long gun up looks down his sights, not along his head, so the edge of his cone is the same on either side of his gun.
/// A FAILURE MEANS: the eye has gone back to the head, which a man with a rifle up carries off to one side of his gun.
#[test]
fn a_man_with_a_rifle_up_looks_along_it() {
    for s in SEEDS {
        let mut sc = Scene::new("gun-line");
        sc.room("hall", 10.0, 8.0, 24.0, 22.0).door("east", Axis::Z, 24.0, 14.4).door("south", Axis::X, 22.0, 20.4).bulb(16.0, 15.0).moon();
        sc.team([8.0, 25.0], 0.0, &[[6.0, 25.0], [6.0, 26.5], [8.0, 26.5], [4.0, 26.0]]);
        sc.defender("hall", [12.25, 15.25], "east");
        let mut p = Play::scene(&sc, &seed(s));
        p.sam_unhurt();
        let d = p.defender_at([12.25, 15.25]);
        let (mut frames, mut head_off) = (0, 0.0f64);
        let off = |a: f64, b: f64| { let e = (a - b).rem_euclid(std::f64::consts::TAU); e.min(std::f64::consts::TAU - e).to_degrees() };
        p.run(0.5);
        for _ in 0..(4 * 60) {
            p.frame();
            let c = p.g.gd[d].ch;
            if p.g.ch[c].anim.long_shown != Some(game::character::LongStance::Shoulder) { continue; }
            frames += 1;
            let (h, g, gun) = (game::character::character_head_dir(&p.g, c), game::guards::look_of(&p.g, c), game::character::character_gun_yaw(&p.g, c));
            head_off = head_off.max(off(h[0].atan2(h[2]), gun));
            let look_off = off(g[0].atan2(g[2]), gun);
            assert!(look_off < 0.75, "seed {s}: his look is {look_off:.1} degrees off his gun");
        }
        assert!(frames > 120 && head_off > 1.5, "seed {s}: the staging: {frames} frames with his rifle up, his head at most {head_off:.1} degrees off it");
    }
}

/// A dim hall: one bulb at its far end, twelve metres from where Sam walks.
fn dim(s: u32) -> Scene {
    let z = 13.0 + 0.4 * nudge(s, 0);
    let mut sc = Scene::new("dim");
    sc.room("hall", 2.0, 5.0, 38.0, 21.0).door("gate", Axis::X, 21.0, 20.0).bulb(36.0, z);
    sc.sentry("post", [4.25, z], [38.0, z]).sam(24.0, 8.0);
    sc
}

/// THE RULE: in poor light a man who walks across an eye is noticed faster than the same man standing still in the same place, and the eye reads his walk off the ground he covers.
/// A FAILURE MEANS: a man in the shadows is as safe walking as standing still, and stillness is worth nothing.
#[test]
fn in_poor_light_a_man_who_moves_is_noticed_faster() {
    for s in SEEDS {
        let z = 13.0 + 0.4 * nudge(s, 0);
        let (mut p, m) = still(Play::scene(&dim(s), &seed(s)));
        p.point_at([30.0, z]);
        p.walk_keys_to(Some([24.0, 19.0]));
        let (mut walking, mut speed) = (0.0, 0.0);
        for _ in 0..(120) {
            p.hold_legs();
            p.frame();
            if p.sam_at()[1] > 15.0 { break; }
            let v = game::guards::player_ground_vel(&mut p.g);
            speed = v[0].hypot(v[2]);
            let l = p.sam_light();
            walking = game::guards::sight_gain(&mut p.g, m, 9.0) / l.powf(1.5);
        }
        p.walk_keys_to(None);
        p.run(1.0);
        let l = p.sam_light();
        assert!((0.01..game::guards::MOTION_LIT_LO).contains(&l), "seed {s}: the staging: Sam's light is {l:.3}");
        let standing = game::guards::sight_gain(&mut p.g, m, 9.0) / l.powf(1.5);
        assert!(walking > 1.5 * standing, "seed {s}: walking {walking:.3}, standing {standing:.3}, by the light");
        assert!(speed > 1.0, "seed {s}: the eye reads a walker at {speed:.2} m/s");
    }
}
