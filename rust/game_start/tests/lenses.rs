//! THE EYES SAM CAN PUT WHERE HE IS NOT: a camera stuck to a wall, the wand under a door.
mod stealth_ask;
use game::game::{InteractableKind, Verb};
use game::ids::GuardId;
use game_start::play::{Opts, Play};
use stealth_ask::StealthAsk;

const SEEDS: [u32; 3] = [21, 22, 23];

/// A quiet mission alone under the fog, the men still, with cameras in hand and none of the mission's own: a man nobody sees, and a wall in his sight.
fn a_man_nobody_sees(s: u32) -> (Play, GuardId, [f64; 3], [f64; 3], [f64; 3]) {
    let mut p = Play::mission("police-post.json", "?levelFile=police-post.json&quiet=1&quietfog=team&cams=0", &Opts { seed: s, ..Default::default() });
    p.sam_unhurt();
    p.g.ai_enabled = false;
    p.run(0.5);
    let pl = p.g.player;
    game::stickycam::sticky_issue(&mut p.g, pl, 2.0, None);
    for m in p.g.guards.clone() {
        if p.up(m) && !p.fog_saw(m) { if let Some((at, n, from)) = p.wall_for(m) { return (p, m, at, n, from); } }
    }
    panic!("seed {s}: no man nobody sees with a wall in his sight");
}
fn stick(p: &mut Play, at: [f64; 3], n: [f64; 3], from: [f64; 3]) -> bool { p.camera_sent(from, [at[0] - n[0] * 0.5, at[1], at[2] - n[2] * 0.5]) }

/// THE RULE: a stuck camera shows who it has a line to, inside its wedge: the man nobody saw is seen once it has stuck, it is one more eye, the floor in front of it is in its wedge and what
/// is behind its wall is not. CONTROL: before it has stuck he is hidden.
/// A FAILURE MEANS: the camera shows nothing, or shows the whole floor.
#[test]
fn a_stuck_camera_shows_who_it_has_a_line_to_inside_its_wedge() {
    for s in SEEDS {
        let (mut p, m, at, n, from) = a_man_nobody_sees(s);
        let eyes = p.fog_eyes();
        assert!(!p.fog_saw(m), "seed {s}: staging: he is not seen");
        assert!(stick(&mut p, at, n, from), "seed {s}: the camera did not stick");
        p.run(0.4);
        assert!(p.fog_saw(m), "seed {s}: the camera has a line to him, and he is not seen");
        assert_eq!(p.fog_eyes(), eyes + 1, "seed {s}: one more eye");
        let off = |k: f64| [at[0] + n[0] * k, at[1], at[2] + n[2] * k];
        assert!(p.in_newest_wedge(off(3.0)) && !p.in_newest_wedge(off(-3.0)), "seed {s}: the floor in front of its wall is in its wedge, what is behind it is not");
    }
}

/// THE RULE: taken back it goes on the launcher again: at it there is a row, the row takes it off the wall, its eye goes and the man is in the fog again.
/// A FAILURE MEANS: a camera spent cannot be got back, or it goes on seeing from the player's pocket.
#[test]
fn a_camera_taken_back_goes_on_the_launcher_again() {
    for s in SEEDS {
        let (mut p, m, at, n, from) = a_man_nobody_sees(s);
        let (out0, load0) = p.cams();
        let start = p.sam_at();
        assert!(stick(&mut p, at, n, from), "seed {s}: the camera did not stick");
        p.run(0.3);
        let eyes = p.fog_eyes();
        assert!(p.fog_saw(m), "seed {s}: CONTROL: stuck, it has him");
        assert_eq!(p.cams().0, out0 + 1, "seed {s}: one out");
        p.sam_put(p.free_near([at[0] + n[0] * 0.8, at[2] + n[2] * 0.8], 0.6));
        p.cursor_on(at);
        p.run(0.3);
        assert!(p.row_taken(InteractableKind::Cam, Verb::TakeCam), "seed {s}: standing at it there is no row");
        p.run(0.3);
        assert_eq!(p.cams(), (out0, load0 + 1.0), "seed {s}: off the wall and on the launcher (the test sent it by the launcher's call, which spends none)");
        assert_eq!(p.fog_eyes(), eyes - 1, "seed {s}: its eye is gone");
        p.sam_put(start);
        p.run(0.5);
        assert!(!p.fog_saw(m), "seed {s}: taken back and Sam gone from there, and he is still seen");
    }
}

/// THE RULE: at a shut door under the fog the wand shows the room behind it, inside its wedge. The man behind the door is hidden before the look and through the feed; seen with the lens through and on him;
/// hidden again with the lens turned away; seen once it is turned back.
/// A FAILURE MEANS: the wand does not show the room, or shows all of it, or shows it before it is through.
#[test]
fn at_a_shut_door_under_the_fog_the_wand_shows_the_room_behind_it_inside_its_wedge() {
    for s in [3u32, 4, 5] {
        let mut p = Play::mission("office.json", "?levelFile=office.json&roomclear&squad4", &Opts { seed: s, ..Default::default() });
        p.sam_unhurt();
        p.run(0.5);
        p.g.ai_enabled = false;
        let door = p.door_at("server");
        let t = p.hostiles_by([door[0], door[1] - 3.0])[0];
        p.g.pl[p.g.player].wand = true;
        p.sam_put([door[0], door[1] + 0.9]);
        p.cursor_on([door[0], 1.0, door[1]]);
        p.run(0.5);
        assert!(p.fogged(t), "seed {s}: the man behind the shut door is hidden before the look");
        assert!(p.row_taken(InteractableKind::Door, Verb::Wand), "seed {s}: no wand row on the shut door: {:?}", p.g.nearby.iter().map(|r| r.line2.clone()).collect::<Vec<_>>());
        let him = p.at(t);
        p.point_at(him);
        p.run(0.1);
        assert!(p.fogged(t), "seed {s}: he is shown at the start of the feed");
        p.until(4.0, |p| p.lens_through() == Some(true)).unwrap_or_else(|| panic!("seed {s}: the lens never got through: {:?}", p.lens_through()));
        p.run(1.5);
        assert!(p.eye_is_lens() && !p.fogged(t), "seed {s}: the lens is through and on him, and he is hidden");
        let away = [door[0] + (door[0] - him[0]) * 1.0, him[1]];
        p.point_at(away);
        p.run(2.5);
        assert!(p.fogged(t), "seed {s}: the lens turned to the other side keeps him");
        p.point_at(him);
        p.run(2.5);
        assert!(!p.fogged(t), "seed {s}: turned back it has lost him");
    }
}
