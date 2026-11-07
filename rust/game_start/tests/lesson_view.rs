//! WHAT THE HUD IS TOLD OF A LESSON (the view's `lesson` section: rust/game_rules/src/lesson `lesson_view`, src/rails `rails_view`): the step that is up, its
//! line, its card and its arrow, the actions open, the men the line keeps clear of, the sky seat's look. The page words none of it again.
use game::orders::Who;
use game::view::{LessonView, RailsView};
use game_start::play::{Opts, Play};

const SEEDS: [u32; 3] = [1, 2, 3];

fn mission(name: &str, seed: u32, seats: u32) -> Play {
    Play::mission(&format!("{}.json", name), &format!("?levelFile={}.json&roomclear=1&squad4", name), &Opts { seed, seats, ..Default::default() })
}
fn lesson(p: &Play) -> LessonView { p.view().lesson.expect("a lesson is live") }
fn frame_of(v: &LessonView) -> &RailsView { v.rails.as_ref().expect("the lesson has a frame") }
fn up(p: &Play) -> String { lesson(p).id.unwrap_or_default() }
fn opens(v: &LessonView, a: &str) -> bool { frame_of(v).open.iter().any(|x| x == a) }
/// the step of this id comes up, as it would by the deeds before it
fn put_on(p: &mut Play, id: &str) -> usize {
    let k = game::lesson::lesson_data(&p.g).unwrap().steps.iter().position(|t| t.id == id).unwrap_or_else(|| panic!("no step '{}'", id));
    p.g.lesson.i = k as f64;
    p.frame();
    k
}
fn step_text(p: &Play, id: &str, f: impl Fn(&game::scene_scenarios::LessonStep) -> Option<String>) -> String {
    let t = game::lesson::lesson_data(&p.g).unwrap().steps.into_iter().find(|t| t.id == id).unwrap();
    game::lesson::lesson_line(&p.g, &f(&t).expect("the step has that line"))
}
/// the middle of the men still up, as the view's sky seat rides it
fn middle(p: &Play) -> [f64; 2] {
    let men = p.mates(); let n = men.len() as f64;
    men.iter().fold([0.0, 0.0], |c, &m| { let a = p.at(m); [c[0] + a[0] / n, c[1] + a[1] / n] })
}
fn char_id(p: &Play, seat: usize) -> f64 { let s = p.g.seats[seat]; p.g.ch[p.g.pl[game::seats::seat(&p.g, s).player].ch].id }

/// every word of the game's state, a word a part: the game's own parts, then rust/sim's block
fn words(p: &mut Play) -> Vec<u32> {
    let (mut w, mut words) = (game::io::W::new(), Vec::new());
    game::state_words::state_words(&mut p.g, &mut w, &mut words);
    words.extend(game_start::block::part_hashes(0));
    words
}

/// A FAILURE MEANS: the player reads a step that is not up, or a key that does nothing. Told as the tick moves the lesson: the view asked of the first step, then
/// a step that waits on the level's script (no key to press, so no mark), then one that opens the squad's key, then what the men are doing once it is pressed.
#[test]
fn the_hud_is_told_the_step_that_is_up_and_what_it_opens() {
    for seed in SEEDS {
        let mut p = mission("tut-team", seed, 1);
        p.frame();
        let v = lesson(&p);
        assert!(v.step == 0 && v.of == 12 && v.id.as_deref() == Some("zoom") && v.asks_view && !v.finished && !v.over && !v.settling, "seed {}: {:?}", seed, v);
        let f = frame_of(&v);
        let line = f.prompt.as_ref().expect("the run is on");
        assert!(line.line.starts_with("Hold [Ctrl]") && line.n == 1 && line.of == 12 && !line.watch && line.note.is_none(), "seed {}: {:?}", seed, line);
        assert!(opens(&v, "zoom") && opens(&v, "camera") && !opens(&v, "order") && !opens(&v, "word"), "seed {}: {:?}", seed, f.open);
        assert_eq!(f.orders.as_deref().unwrap().join(" "), "move stack go fallIn", "seed {}", seed);
        assert!(f.mark.is_none() && f.card.is_none() && f.arrow.is_none() && f.sky.is_none() && !f.teaches_hold && !f.stalk_shut, "seed {}: {:?}", seed, f);
        assert_eq!(f.no_lasers, game::lesson::lesson_data(&p.g).unwrap().rails.unwrap().lasers == Some(false), "seed {}", seed);

        // the view turned by the hand: the next step, and it asks for nothing
        p.run(0.2);   // (a view test starts from the view after the run's opening ticks)
        let cam = p.host.cam;
        p.g.tab.camera[cam].distance *= 1.5; p.g.tab.camera[cam].desired_distance *= 1.5;
        p.until(4.0, |p| up(p) == "smoke").unwrap_or_else(|| panic!("seed {}: the zoom never passed the step: {:?}", seed, lesson(&p)));
        let v = lesson(&p);
        assert!(!v.asks_view && v.step == 1, "seed {}: {:?}", seed, v);
        let line = frame_of(&v).prompt.clone().unwrap();
        assert!(line.watch && !line.line.contains('[') && line.n == 2, "seed {}: a step that waits on the script is something to watch: {:?}", seed, line);

        // the step that opens the squad's key: it names the key, and marks the place
        let k = put_on(&mut p, "move");
        let v = lesson(&p);
        assert!(v.step == k as u32 && opens(&v, "order") && frame_of(&v).mark == Some([11.4, 0.02, 16.4]), "seed {}: {:?}", seed, v);
        let line = frame_of(&v).prompt.clone().unwrap();
        assert!(line.line.starts_with("Press [B]") && !line.watch, "seed {}: {:?}", seed, line);

        // the order out: what the men are doing, never the ask again
        p.order("move", [11.4, 16.4], Who::All);
        p.run(0.2);
        let line = frame_of(&lesson(&p)).prompt.clone().unwrap();
        assert!(line.line == "They're using the smoke to get to the yard" && line.watch, "seed {}: {:?}", seed, line);

        // CONTROL: the same level with no mission on it has no lesson live, and a level with no lesson block none either
        drop(p);
        let mut q = Play::level("tut-team.json", false, &Opts { seed, ..Default::default() });
        q.frame();
        assert!(q.view().lesson.is_none(), "seed {}: {:?}", seed, q.view().lesson);
        drop(q);
        let mut q = Play::level("kit-yard.json", true, &Opts { seed, ..Default::default() });
        q.frame();
        assert!(q.view().lesson.is_none(), "seed {}", seed);
    }
}

/// A FAILURE MEANS: a card lands on the deed before it, or a line asks for a key while the moment plays out. The step before a card settles, what its men are
/// doing up and nothing asked; then the card comes with its own step, its words written out.
#[test]
fn a_card_comes_with_its_step_after_the_deed_before_it_has_settled() {
    for seed in SEEDS {
        let mut p = mission("tut-queue", seed, 1);
        p.frame();
        let v = lesson(&p);
        let card = frame_of(&v).card.clone().expect("the first step has a card");
        assert!(!card.text.is_empty() && !card.text.contains('{'), "seed {}: {:?}", seed, card);

        put_on(&mut p, "door1");
        assert!(frame_of(&lesson(&p)).card.is_none(), "seed {}: a step with no card has none", seed);
        let door = game::lesson::lesson_data(&p.g).unwrap().steps.into_iter().find(|t| t.id == "door1").unwrap().at.unwrap();
        p.order("stack", [door[0], door[2]], Who::All);
        p.until(60.0, |p| lesson(p).settling || up(p) != "door1").unwrap_or_else(|| panic!("seed {}: the stack never did the step", seed));
        let v = lesson(&p);
        assert!(v.settling && up(&p) == "door1", "seed {}: the step stays up for the beat: {:?}", seed, v);
        let line = frame_of(&v).prompt.clone().unwrap();
        assert!(line.watch && !line.line.contains('['), "seed {}: nothing is asked while it settles: {:?}", seed, line);
        assert!(frame_of(&v).card.is_none(), "seed {}: the card waits", seed);

        p.until(12.0, |p| up(p) == "door2").unwrap_or_else(|| panic!("seed {}: the next step never came up", seed));
        let v = lesson(&p);
        assert!(!v.settling && frame_of(&v).card.is_some(), "seed {}: {:?}", seed, v);
    }
}

/// A FAILURE MEANS: a step's mark is drawn as a ring where the level says an arrow, or a refused tap says nothing. And the words under the line go after their few seconds.
#[test]
fn an_arrow_and_a_refused_tap_are_told() {
    for seed in SEEDS {
        let mut p = mission("tut-queue", seed, 1);
        p.frame();
        assert!(frame_of(&lesson(&p)).arrow.is_none(), "seed {}", seed);
        put_on(&mut p, "gateB");
        let f = lesson(&p);
        assert!(frame_of(&f).teaches_hold && frame_of(&f).prompt.as_ref().unwrap().note.is_none(), "seed {}: {:?}", seed, f);
        assert!(game::rails::rails_tap_off(&mut p.g), "seed {}: the step refuses the tap", seed);
        p.frame();
        let want = step_text(&p, "gateB", |t| t.no_tap.clone());
        assert_eq!(frame_of(&lesson(&p)).prompt.as_ref().unwrap().note.as_deref(), Some(want.as_str()), "seed {}", seed);
        p.run(3.0);
        assert_eq!(up(&p), "gateB", "seed {}", seed);
        assert!(frame_of(&lesson(&p)).prompt.as_ref().unwrap().note.is_none(), "seed {}: the words go", seed);
        drop(p);

        let mut q = Play::mission("tut-road.json", "?levelFile=tut-road.json&quiet=1", &Opts { seed, ..Default::default() });
        q.frame();
        put_on(&mut q, "wall");
        let f = lesson(&q);
        let toward = game::lesson::lesson_data(&q.g).unwrap().steps.into_iter().find(|t| t.id == "wall").unwrap().toward;
        assert_eq!(frame_of(&f).arrow, toward, "seed {}", seed);
        assert!(frame_of(&f).arrow.is_some(), "seed {}", seed);
    }
}

/// A FAILURE MEANS: the camera of a tutorial with nobody on the floor stands still while the squad walks on (the point it rides was eased only while a step had a
/// mark), or jumps. And nobody on the floor is one of the men the line keeps clear of.
#[test]
fn the_sky_seat_rides_the_squad_as_the_tick_eases_it() {
    for seed in SEEDS {
        let mut p = mission("tut-sky", seed, 1);
        p.run(1.0);
        let v = lesson(&p);
        let f = frame_of(&v);
        assert_eq!(up(&p), "turn", "seed {}", seed);
        assert!(f.mark.is_none(), "seed {}: this step marks nothing", seed);
        let sky = f.sky.expect("the sky seat");
        let men = p.mates();
        assert_eq!(f.line_men.len(), men.len(), "seed {}: {:?}", seed, f.line_men);
        assert!(!f.line_men.contains(&char_id(&p, 0)), "seed {}: nobody on the floor is the player", seed);
        let c = middle(&p);
        assert!((sky[0] - c[0]).hypot(sky[2] - c[1]) < 1.5, "seed {}: the view looks at {:?}, the squad's middle is {:?}", seed, sky, c);
        // (the squad walks with the walk keys down, where nothing else asks where the view looks: it goes with them, a little a tick)
        put_on(&mut p, "move");
        p.run(0.5);
        let from = middle(&p);
        p.order("move", [11.4, 16.4], Who::All);
        p.key_down("KeyD");
        let (mut last, mut most) = (frame_of(&lesson(&p)).sky.unwrap(), 0.0_f64);
        for _ in 0..(8 * 60) { p.frame(); let s = frame_of(&lesson(&p)).sky.unwrap(); most = most.max((s[0] - last[0]).hypot(s[2] - last[2])); last = s; }
        p.key_up("KeyD");
        let (c, off) = (middle(&p), p.g.rails.sky.off);
        let walked = (c[0] - from[0]).hypot(c[1] - from[1]);
        assert!(walked > 2.0, "seed {}: the squad walked {} m", seed, walked);
        assert!((last[0] - off[0] - c[0]).hypot(last[2] - off[2] - c[1]) < 2.0, "seed {}: the view is at {:?} less the hand's {:?}, the squad's middle at {:?}", seed, last, off, c);
        assert!(most < 0.5, "seed {}: the view jumped {} m in a tick", seed, most);
        drop(p);
        // CONTROL: he is on the floor
        let mut q = mission("tut-team", seed, 1);
        q.run(0.5);
        let v = lesson(&q);
        assert!(frame_of(&v).sky.is_none() && frame_of(&v).line_men.contains(&char_id(&q, 0)), "seed {}: {:?}", seed, frame_of(&v));
    }
}

/// A FAILURE MEANS: asking what a HUD is told of a lesson changes the game (the sky seat's look used to ease as it was asked). The same run twice, one asked after
/// every frame and one never, equal in every part of the state after every second.
#[test]
fn asking_about_a_lesson_writes_nothing() {
    for name in ["tut-team", "tut-sky"] {
        for seed in SEEDS {
            let run = |ask: bool| -> Vec<Vec<u32>> {
                let mut p = mission(name, seed, 1);
                let mut out = Vec::new();
                for s in 0..10 {
                    if s == 2 { p.order("move", [11.4, 16.4], Who::All); }
                    for _ in 0..60 { p.frame(); if ask { assert!(p.view().lesson.is_some()); } }
                    out.push(words(&mut p));
                }
                out
            };
            let (never, always) = (run(false), run(true));
            for (s, (a, b)) in never.iter().zip(always.iter()).enumerate() {
                let parts: Vec<usize> = (0..a.len()).filter(|&i| a[i] != b[i]).collect();
                assert!(parts.is_empty(), "{} seed {}: after {} s the asked game differs in parts {:?}", name, seed, s + 1, parts);
            }
        }
    }
}

/// A FAILURE MEANS: a player reads a brace, or the knife the game no longer has. Every line and card of every tutorial mission, written out as the HUD is told it.
#[test]
fn every_line_a_player_reads_is_written_out() {
    let mut read = 0;
    for name in ["tut-road", "tut-team", "tut-sky", "tut-kit", "tut-solo", "tut-squad", "tut-queue", "tut-quiet", "tut-pair"] {
        let p = Play::level(&format!("{}.json", name), false, &Opts::default());
        let l = p.g.level.scenarios.as_ref().and_then(|s| s.lesson.clone()).unwrap_or_else(|| panic!("{} has a lesson", name));
        for t in &l.steps {
            let after = t.after.as_ref();
            for w in [Some(&t.line), t.setup_line.as_ref(), t.ring_line.as_ref(), t.blow_line.as_ref(), t.menu_line.as_ref(), t.page_line.as_ref(), t.flat_line.as_ref(), t.no_tap.as_ref(),
                      after.and_then(|a| a.line.as_ref()), after.and_then(|a| a.word.as_ref()), t.explain.as_ref().map(|e| &e.text)].into_iter().flatten() {
                let said = game::lesson::lesson_line(&p.g, w);
                read += 1;
                assert!(!said.contains('{') && !said.contains('}') && !said.to_lowercase().contains("knife"), "{} '{}': {:?} reads {:?}", name, t.id, w, said);
            }
        }
    }
    assert!(read > 200, "read {} lines", read);
}

/// A GAME OF TWO: each HUD is told about its own man and its own ring, whoever stands on the game. A FAILURE MEANS: the second machine is told the first
/// player's line, or that his man is down when the other's is, or the line keeps clear of a man who is not his.
#[test]
fn in_a_game_of_two_each_hud_is_told_of_its_own_seat() {
    for seed in SEEDS {
        let mut p = mission("tut-queue", seed, 2);
        p.frame();
        put_on(&mut p, "gateB");
        let (a, b) = (p.view_of(0).lesson.unwrap(), p.view_of(1).lesson.unwrap());
        // CONTROL: no ring up, one line
        assert_eq!(frame_of(&a).prompt.as_ref().unwrap().line, frame_of(&b).prompt.as_ref().unwrap().line, "seed {}", seed);
        let (me, him) = (char_id(&p, 0), char_id(&p, 1));
        assert!(frame_of(&a).line_men.contains(&me) && !frame_of(&a).line_men.contains(&him), "seed {}: {:?}", seed, frame_of(&a).line_men);
        assert!(frame_of(&b).line_men.contains(&him) && !frame_of(&b).line_men.contains(&me), "seed {}: {:?}", seed, frame_of(&b).line_men);

        // the second player's ring up on the step's mark: the line is what to press in it, for him alone
        let at = frame_of(&a).mark.unwrap();
        let s = p.g.seats[1];
        game::seats::with_seat(&mut p.g, s, &mut |g| { let t = game::orders::squad_orders_target_at(g, at); game::orders::squad_orders_open(g, Some(&t), Some(true), None); });
        assert!(game::seats::seat(&p.g, s).orders.menu.is_some(), "seed {}: his ring is up", seed);
        let want = step_text(&p, "gateB", |t| t.flat_line.clone());
        assert_eq!(frame_of(&p.view_of(1).lesson.unwrap()).prompt.as_ref().unwrap().line, want, "seed {}: his line", seed);
        assert_ne!(frame_of(&p.view_of(0).lesson.unwrap()).prompt.as_ref().unwrap().line, want, "seed {}: the other's", seed);

        // the second man down: the run is over for him alone
        let him = game::seats::seat(&p.g, s).player;
        p.g.pl[him].down = true;
        let (a, b) = (p.view_of(0).lesson.unwrap(), p.view_of(1).lesson.unwrap());
        assert!(b.over && frame_of(&b).prompt.is_none(), "seed {}: {:?}", seed, b);
        assert!(!a.over && frame_of(&a).prompt.is_some(), "seed {}: {:?}", seed, a);
    }
}
