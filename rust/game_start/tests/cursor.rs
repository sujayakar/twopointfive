//! WHAT THE CURSOR IS ON (rust/game_rules/src/view cursor_view, orders target_under): asked of the game shared, of any seat, by a cursor and a camera as they stand.
use game::view::UnderKind;
use game_start::play::{dist, Opts, Play};

const SEEDS: [u32; 3] = [1, 2, 3];
fn stood(seed: u32, seats: u32) -> Play { let mut p = Play::level("kit-yard.json", true, &Opts { seed, seats, ..Default::default() }); p.run(1.0); p }
/// every word of the game's state, a word a part: the game's own parts, then rust/sim's block
fn words(p: &mut Play) -> Vec<u32> {
    let (mut w, mut words) = (game::io::W::new(), Vec::new());
    game::state_words::state_words(&mut p.g, &mut w, &mut words);
    words.extend(game_start::block::part_hashes(0));
    words
}

/// A FAILURE MEANS: the HUD is told the cursor is on another thing than an order given there would be about.
#[test]
fn the_hud_is_told_what_an_order_there_would_be_about() {
    for seed in SEEDS {
        let mut p = stood(seed, 1);
        let mut kinds = std::collections::BTreeMap::new();
        let mut n = 0;
        for k in 0..160 {
            // points of the floor and a man's height, in rings about Sam
            let (r, th) = (1.0 + (k % 16) as f64 * 0.7, k as f64 * 0.83);
            let s = p.sam_at();
            let at = [s[0] + r * th.cos(), if k % 3 == 0 { 1.1 } else { 0.02 }, s[1] + r * th.sin()];
            let Some(v) = p.pointed_at(0, at) else { continue };
            n += 1;
            let u = v.under.expect("a squad answers to Sam");
            *kinds.entry(format!("{:?}", u.kind)).or_insert(0) += 1;
            // …the same cursor through the order menu's own question, inside the game
            let (input, cam, canvas) = (p.host.input, p.host.cam, p.host.canvas);
            let (sx, sy, _) = game::camera::follow_camera_project(&p.g, cam, at, canvas.client_width, canvas.client_height, None, None);
            p.g.tab.input[input].mouse_x = sx; p.g.tab.input[input].mouse_y = sy;
            let t = game::orders::squad_orders_target_under(&mut p.g, input, cam, canvas);
            let o = game::view::under(&p.g, &t);
            assert_eq!((format!("{:?}", u.kind), u.at.map(f64::to_bits), u.door.clone(), u.man, u.wall.as_ref().map(|w| w.index)), (format!("{:?}", o.kind), o.at.map(f64::to_bits), o.door, o.man, o.wall.map(|w| w.index)), "seed {} point {:?}", seed, at);
        }
        assert!(n > 60 && kinds.len() >= 2, "seed {}: {} points in the picture, kinds {:?}", seed, n, kinds);
        // a teammate under the cursor is the squad's own, a door its door
        let m = p.mates()[0];
        let at = p.at(m);
        if let Some(v) = p.pointed_at(0, [at[0], 1.2, at[1]]) { assert_eq!(v.under.unwrap().kind, UnderKind::None, "seed {}: the cursor on a teammate", seed); }
        let d = p.g.doors.list.iter().copied().min_by(|&a, &b| { let f = |d: game::ids::DoorId| { let c = p.g.dr[d].frame_centre; dist([c[0], c[2]], p.sam_at()) }; f(a).partial_cmp(&f(b)).unwrap() }).unwrap();
        let c = p.g.dr[d].frame_centre;
        if let Some(v) = p.pointed_at(0, [c[0], 1.0, c[2]]) { let u = v.under.unwrap(); assert!(u.kind == UnderKind::Door && u.door.as_deref() == Some(p.g.dr[d].def.name.as_str()) && u.side != 0.0, "seed {}: the cursor on a doorway: {:?}", seed, u); }
    }
}

/// A FAILURE MEANS: pointing changes the game, or a cursor can stop it (a panic in a question stops play): any cursor, on the picture or off it.
#[test]
fn any_cursor_is_answered_and_pointing_changes_nothing() {
    for seed in SEEDS {
        let mut p = stood(seed, 1);
        let before = words(&mut p);
        let mut roll = seed as u64 * 0x9e37_79b9 + 7;
        let mut r = || { roll ^= roll << 13; roll ^= roll >> 7; roll ^= roll << 17; (roll % 100_000) as f64 / 100_000.0 };
        for k in 0..400 {
            let mouse = match k % 8 { 0 => [f64::NAN, 10.0], 1 => [-5000.0, 1e7], _ => [r() * 1600.0, r() * 900.0] };
            p.pointed(0, mouse);
        }
        let after = words(&mut p);
        let parts: Vec<&str> = (0..game::state_words::STATE_NAMES.len()).filter(|&i| after[i] != before[i]).map(|i| game::state_words::STATE_NAMES[i]).collect();
        assert!(parts.is_empty() && before == after, "seed {}: pointing moved {:?}", seed, parts);
    }
}

/// A GAME OF TWO: each cursor is answered for the player whose it is. A FAILURE MEANS: the second player's own body, or the face of a door, is told as the first's.
#[test]
fn in_a_game_of_two_each_cursor_is_its_own_players() {
    for seed in SEEDS {
        let mut p = stood(seed, 2);
        let (a, b) = (p.seat_at(0), p.seat_at(1));
        assert!(dist(a, b) > 0.3, "seed {}: two men", seed);
        // the cursor on the second player's chest: to him it is himself, and nobody; to the first it is not the squad itself at his own feet
        let Some(v) = p.pointed_at(1, [b[0], 1.1, b[1]]) else { panic!("seed {}: he is not in the picture", seed) };
        let u = v.under.expect("a squad answers to the second player too");
        assert!(u.kind == UnderKind::None && u.man < 0.0 && dist([u.at[0], u.at[2]], b) < 1e-9, "seed {}: his own body under the cursor is himself: {:?}", seed, u);
        let Some(w) = p.pointed_at(0, [a[0], 1.1, a[1]]) else { panic!("seed {}: the first is not in the picture", seed) };
        let u = w.under.unwrap();
        assert!(u.kind == UnderKind::None && u.man < 0.0 && dist([u.at[0], u.at[2]], a) < 1e-9, "seed {}: and the first's is his: {:?}", seed, u);
    }
}
