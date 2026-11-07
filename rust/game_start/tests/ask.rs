//! THE WORLD AS A QUESTION TAKES IT (rust/game_rules/src/ask): a line or a ray asked of the level as it stands, of its statics alone, or with a leaf let through,
//! and nothing of the game changed to ask it.
use game::ask::Over;
use game::scene_collision::{raycast_over, segment_blocked_over, shot_blocked_over, solid_blocked_over};
use game_start::play::{Opts, Play};
use game_start::scene::{Axis, Scene};

/// two rooms with a shut door between them, Sam in the one
fn two_rooms(seed: u32) -> Play {
    let mut s = Scene::new("two-rooms");
    s.room("west", 10.0, 10.0, 18.0, 16.0).room("east", 18.0, 10.0, 26.0, 16.0);
    s.door("between", Axis::Z, 18.0, 12.4).door("east-yard", Axis::X, 16.0, 21.4);
    s.bulb(14.0, 13.0).bulb(22.0, 13.0);
    s.sentry("post", [12.25, 13.25], [10.0, 13.25]).sam(23.75, 13.25);
    let mut p = Play::scene(&s, &Opts { seed, ..Default::default() });
    p.run(0.5);
    p
}
/// every word of the game's state, a word a part: the game's own parts, then rust/sim's block
fn words(p: &mut Play) -> Vec<u32> {
    let (mut w, mut words) = (game::io::W::new(), Vec::new());
    game::state_words::state_words(&mut p.g, &mut w, &mut words);
    words.extend(game_start::block::part_hashes(0));
    words
}

/// A FAILURE MEANS: a question asked of the statics alone meets a door's leaf, one asked with a leaf let through meets that leaf (or misses another), or a wall
/// stops being a wall to either.
#[test]
fn a_leaf_is_in_the_way_of_the_level_as_it_stands_and_of_no_question_that_lets_it_through() {
    for seed in [1, 2, 3] {
        let mut p = two_rooms(seed);
        let (between, yard) = (p.g.doors.list[0], p.g.doors.list[1]);
        let c = p.g.dr[between].frame_centre;
        let (a, b) = ([c[0] - 1.5, 1.1, c[2]], [c[0] + 1.5, 1.1, c[2]]);
        // …and beside the doorway, where the wall stands
        let (wa, wb) = ([a[0], 1.1, c[2] + 2.0], [b[0], 1.1, c[2] + 2.0]);
        let (own, statics, but, other) = (Over::OWN, Over::statics(), Over::but_door(&p.g, between), Over::but_door(&p.g, yard));
        let g = &mut p.g;
        assert!(solid_blocked_over(g, &own, a, b, None), "seed {}: the shut leaf stands across the doorway", seed);
        assert!(solid_blocked_over(g, &other, a, b, None), "seed {}: another door's leaf let through, this one still stands", seed);
        for (name, over) in [("its statics alone", &statics), ("its own leaf let through", &but)] {
            assert!(!solid_blocked_over(g, over, a, b, None), "seed {}: asked of {}, the doorway is clear", seed, name);
            assert!(!segment_blocked_over(g, over, a, b, None, None) && !shot_blocked_over(g, over, a, b, None, None), "seed {}: …to an eye and to a round", seed);
            assert!(solid_blocked_over(g, over, wa, wb, None), "seed {}: …and the wall beside it is a wall", seed);
        }
        // the ray: the leaf is the nearest thing met, by its place among the game's own leaves whichever are let through
        let hit = raycast_over(g, &own, a, [1.0, 0.0, 0.0], 3.0).expect("the leaf");
        let leaf = g.dr[between].box_;
        assert_eq!(g.col.dynamic_boxes[(hit.index - 1e6) as usize], leaf, "seed {}", seed);
        assert!(raycast_over(g, &but, a, [1.0, 0.0, 0.0], 3.0).map_or(true, |h| h.t > hit.t + 0.2), "seed {}: let through, the ray flies on", seed);
        let same = raycast_over(g, &other, a, [1.0, 0.0, 0.0], 3.0).expect("the leaf");
        assert_eq!(same.index, hit.index, "seed {}: its place is its own", seed);
    }
}

/// A FAILURE MEANS: a wall let through still stops a line, one added does not, or the two together do not make the level again.
#[test]
fn a_wall_let_through_stops_nothing_and_a_box_added_stops_what_it_would() {
    for seed in [1, 2, 3] {
        let mut p = two_rooms(seed);
        let c = p.g.dr[p.g.doors.list[0]].frame_centre;
        let (a, b) = ([c[0] - 1.0, 1.1, c[2] + 2.0], [c[0] + 1.0, 1.1, c[2] + 2.0]);
        let g = &mut p.g;
        let hit = raycast_over(g, &Over::OWN, a, [1.0, 0.0, 0.0], 2.0).expect("the wall between the rooms");
        let (i, wall) = (hit.index as usize, g.col.boxes[hit.index as usize]);
        assert!(solid_blocked_over(g, &Over::OWN, a, b, None), "seed {}: the wall stands", seed);
        // let through (and whatever the plan of a wall has behind it with it)
        let mut open = Over { open: vec![i as u32], ..Over::OWN };
        for _ in 0..8 { match raycast_over(g, &open, a, [1.0, 0.0, 0.0], 2.0) { Some(h) if h.index < 1e6 => open.open.push(h.index as u32), _ => break } }
        assert!(!solid_blocked_over(g, &open, a, b, None) && !segment_blocked_over(g, &open, a, b, None, None) && !shot_blocked_over(g, &open, a, b, None, None), "seed {}: let through, it stops nothing", seed);
        // …and added as a box of the question's own, it is there again
        let back = Over { extra: vec![wall], ..open.clone() };
        assert!(solid_blocked_over(g, &back, a, b, None), "seed {}: the same box added stops the line", seed);
        let again = raycast_over(g, &back, a, [1.0, 0.0, 0.0], 2.0).expect("the added box");
        assert!((again.t - hit.t).abs() < 1e-12 && again.n == hit.n && again.index >= 2e6, "seed {}: where the wall was, by the same face", seed);
        // with a leaf let through as well: the two are one world
        let d = g.doors.list[0];
        let both = Over { leaves: Over::but_door(g, d).leaves, ..back };
        let (da, db) = ([c[0] - 1.0, 1.1, c[2]], [c[0] + 1.0, 1.1, c[2]]);
        assert!(solid_blocked_over(g, &both, a, b, None) && !solid_blocked_over(g, &both, da, db, None), "seed {}", seed);
    }
}

/// A FAILURE MEANS: asking changes the game. The questions a rule asks of a world it makes up used to take the door leaves out of the game's list, mark walls
/// as passed, and put it all back: a game two machines hold must be the same before and after.
#[test]
fn asking_of_a_world_that_is_not_changes_nothing() {
    for seed in [1, 2, 3] {
        let mut p = two_rooms(seed);
        let d = p.g.doors.list[0];
        let c = p.g.dr[d].frame_centre;
        let wall = raycast_over(&mut p.g, &Over::OWN, [c[0] - 1.0, 1.1, c[2] + 2.0], [1.0, 0.0, 0.0], 2.0).expect("the wall between the rooms").index as usize;
        let box_ = p.g.col.boxes[wall];
        let over = Over { open: vec![wall as u32], extra: vec![box_], ..Over::but_door(&p.g, d) };
        assert!(!over.plain());
        let before = words(&mut p);
        let shots = p.g.stats.glasspen.clone();
        for k in 0..48 {
            let th = k as f64 * 0.131;
            let (from, dir) = ([c[0] - 3.0, 1.2, c[2] + 0.4], [th.cos(), 0.0, th.sin()]);
            let to = [from[0] + 9.0 * dir[0], 1.2, from[2] + 9.0 * dir[2]];
            for w in [&over, &Over::OWN, &Over::statics()] {
                solid_blocked_over(&mut p.g, w, from, to, None);
                segment_blocked_over(&mut p.g, w, from, to, None, None);
                shot_blocked_over(&mut p.g, w, from, to, None, None);
            }
        }
        p.g.stats.glasspen = shots;   // (a tally of a round's lines asked through panes: no part of the world)
        let after = words(&mut p);
        let parts: Vec<&str> = (0..game::state_words::STATE_NAMES.len()).filter(|&i| after[i] != before[i]).map(|i| game::state_words::STATE_NAMES[i]).collect();
        assert!(parts.is_empty() && after == before, "seed {}: asking moved {:?}", seed, parts);
    }
}
