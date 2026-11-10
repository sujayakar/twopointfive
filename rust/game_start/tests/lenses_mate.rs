//! THE TEAM'S WAND AND THE LAUNCHER'S CAMERA. A teammate looks under a door only on order; a camera sticks to a wall and drops off a leaf.
mod stealth_ask;
use game::ids::GuardId;
use game_start::play::{dist, Opts, Play};
use stealth_ask::StealthAsk;

const SEEDS: [u32; 3] = [1, 2, 3];

/// The office's room clear with Red One's card carrying the wand, and the man behind the server door.
fn carried(s: u32) -> (Play, GuardId, GuardId) {
    use game::kit::{KitField, KitValue};
    let mut p = Play::mission("office.json", "?levelFile=office.json&roomclear&squad4", &Opts { seed: s, ..Default::default() });
    p.g.m.kit.deal_own = true;
    if !game::kit::kit_set(&mut p.g, 1.0, KitField::Wand, &KitValue::Bool(true)) {
        let bangs = p.g.m.kit.kit.members[1].bangs;
        assert!(game::kit::kit_set(&mut p.g, 1.0, KitField::Bangs, &KitValue::Num((bangs - 1.0).max(0.0))) && game::kit::kit_set(&mut p.g, 1.0, KitField::Wand, &KitValue::Bool(true)));
    }
    assert!(game::savestate::control::restart(&mut p.g));
    p.sam_unhurt();
    p.g.k.fog.own = false;
    p.g.k.fog.last_seen = 0.0;
    p.run(0.6);
    let him = p.mates()[0];
    assert!(game::wandorder::carries(&p.g, him), "seed {s}: he does not carry it");
    p.g.ai_enabled = false;
    let door = p.door_at("server");
    let t = p.hostiles_by(door)[0];
    (p, him, t)
}
fn press_v(p: &mut Play) { let k = p.g.k.wand.order_key.clone(); p.key_down(&k); p.frame(); p.key_up(&k); }

/// THE RULE: carried and never ordered, nobody kneels, no lens is among the fog's eyes, the man behind the door is not seen, and V with no menu orders nobody (it whistles). X on the door and then V: the man who carries it takes the order, walks up, feeds it, and only then his lens is an eye and the man behind the door seen.
/// A FAILURE MEANS: a teammate looks under doors on his own, or the order does nothing.
#[test]
fn a_teammate_looks_under_a_door_only_on_order() {
    for s in SEEDS {
        let (mut p, him, t) = carried(s);
        for _ in 0..300 { p.frame(); assert!(p.wand_jobs().is_empty() && p.mates_lenses() == 0 && !p.fog_saw(t), "seed {s}: never ordered, and a lens or a sight of him"); }
        press_v(&mut p);
        p.frame();
        assert!(p.wand_jobs().is_empty(), "seed {s}: V with no menu ordered somebody");
        p.run(2.5);
        let d = game::doors::doors_by_name(&p.g, "server").expect("the server door");
        let c = p.g.dr[d].frame_centre;
        let tgt = game::orders::squad_orders_target_at(&mut p.g, [c[0], 0.0, c[2]]);
        assert!(game::orders::squad_orders_open(&mut p.g, Some(&tgt), None, None), "seed {s}: the door's menu would not open");
        press_v(&mut p);
        assert_eq!(p.wand_jobs(), vec![him], "seed {s}: X on the door, then V");
        p.until(25.0, |p| { let lens = p.mates_lenses() > 0; if !lens { assert!(!p.fog_saw(t), "seed {s}: he was seen before the lens was through"); } lens }).unwrap_or_else(|| panic!("seed {s}: 25 s and no lens: {}", p.why(him)));
        p.until(6.0, |p| p.fog_saw(t)).unwrap_or_else(|| panic!("seed {s}: the lens is through and the man behind the door is not seen"));
    }
}

/// THE RULE: the launcher's camera sticks where it strikes a wall, drops to the floor from a leaf, and at a man staggers him (never a knockout) and drops at his feet.
/// A FAILURE MEANS: the camera floats on a door, or knocks a man out.
#[test]
fn the_launchers_camera_sticks_to_a_wall_and_drops_off_a_leaf_or_a_man() {
    for s in SEEDS {
        let mut p = Play::mission("police-post.json", "?levelFile=police-post.json&quiet=1&quietfog=team&cams=0", &Opts { seed: s, ..Default::default() });
        p.sam_unhurt();
        p.g.ai_enabled = false;
        p.run(0.5);
        let pl = p.g.player;
        game::stickycam::sticky_issue(&mut p.g, pl, 3.0, None);
        let (m, wall, n, from) = p.g.guards.clone().into_iter().find_map(|m| p.wall_for(m).map(|(a, b, c)| (m, a, b, c))).unwrap_or_else(|| panic!("seed {s}: no man with a wall in his sight"));
        assert!(p.camera_sent(from, [wall[0] - n[0] * 0.5, wall[1], wall[2] - n[2] * 0.5]), "seed {s}: it did not stick to the wall");
        let (at, si, rest) = p.last_cam();
        assert!(rest && si >= 0.0 && dist([at[0], at[2]], [wall[0], wall[2]]) < 0.2, "seed {s}: on the wall {at:?} {si}, aimed at {wall:?}");

        let name = p.door_names().into_iter().find(|n| p.leaf(n).1.abs() < 0.05).expect("a shut door");
        let d = game::doors::doors_by_name(&p.g, &name).expect("a shut door");
        let c = p.g.dr[d].frame_centre;
        let mut left = None;
        for k in 0..8 {
            let a = k as f64 * std::f64::consts::FRAC_PI_4;
            let dir = [a.sin(), 0.0, a.cos()];
            let start = [c[0] - dir[0] * 1.2, 1.2, c[2] - dir[2] * 1.2];
            if game::scene_collision::raycast(&mut p.g, start, dir, 2.0).is_some_and(|h| h.index >= 1_000_000.0) {
                p.camera_sent(start, [c[0] + dir[0], 1.2, c[2] + dir[2]]);
                left = Some(p.last_cam());
                break;
            }
        }
        let (at, si, rest) = left.unwrap_or_else(|| panic!("seed {s}: no bearing at the leaf"));
        assert!(rest && si == -1.0 && at[1] < 0.3, "seed {s}: off a leaf it rests at {at:?} on {si}");

        let spot = p.at(m);
        // (the same wall's air: `wall_for` has a line from just off the wall to his chest)
        let start = [wall[0] + n[0] * 0.4, 1.25, wall[2] + n[2] * 0.4];
        let before = p.g.time;
        p.camera_sent(start, [spot[0], 1.25, spot[1]]);
        let (at, si, rest) = p.last_cam();
        assert!(rest && si == -1.0 && at[1] < 0.3 && dist([at[0], at[2]], spot) < 1.0, "seed {s}: at a man it rests at {at:?} on {si}");
        assert!(p.g.cond[p.g.gd[m].ch].fold_until > before, "seed {s}: he took no stagger");
        assert!(p.up(m), "seed {s}: a camera knocked him out");
    }
}
