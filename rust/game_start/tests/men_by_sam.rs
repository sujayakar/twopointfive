//! A TEAMMATE'S FEET: round a chair, round the player, and his gun on the walk.
mod doors_ask;
use doors_ask::*;
use game::ids::*;
use game::orders::Who;
use game_start::play::{dist, nudge, Opts, Play};

fn four(seed: u32) -> Opts { Opts { seed, ..Default::default() } }
fn hush(p: &mut Play) { let keep = p.defender_at([24.25, 12.25]); p.drop_all_but(&[keep]); p.pin_man(keep, [24.25, 12.25], 0.0, game::game::GuardState::Patrol, 0.0); }

/// Sam standing on the open yard, one man `d` metres from him on the far side from the others, and an order that takes that man across him. How far Sam was shoved, and did the man get past him.
fn across_sam(seed: u32, order: &str, d: f64) -> (f64, bool) {
    let (s, _) = yard(seed);
    let mut p = Play::scene(&s, &four(seed));
    p.sam_unhurt();
    p.run(0.2);
    hush(&mut p);
    let sam = [18.0 + nudge(seed, 70), 22.0];
    p.sam_put([sam[0] + 30.0, sam[1]]);
    p.run(0.1);
    let men = p.men();
    p.man_stands(men[0], [sam[0] - d, sam[1]]);
    for (i, &m) in men.iter().enumerate().skip(1) { p.man_stands(m, [sam[0] + 2.2 + 0.45 * i as f64, sam[1] + (i as f64 - 2.0) * 0.9]); }
    p.sam_put(sam);
    p.run(2.0 / 60.0);
    p.man_stands(men[0], [sam[0] - d, sam[1]]);
    let from = p.sam_at();
    if order == "fallIn" { p.order("fallIn", sam, Who::All); } else { p.order_floor("move", [sam[0] + 1.5, sam[1]], Who::Num(0.0)); }
    let (mut shoved, mut past) = (0.0f64, false);
    for _ in 0..300 {
        p.frame();
        shoved = shoved.max(dist(p.sam_at(), from));
        if p.at(men[0])[0] > sam[0] + 0.2 { past = true; }
    }
    (shoved, past)
}

/// THE RULE: teammates falling in never shove you. 'Fall in' with a man a pace from the player and his place on the player's far side: the player is not moved, and the man goes round him. The control is a man on another order walking into the standing player: he is still shouldered, which says the line reads a shove.
/// A FAILURE MEANS: the player is pushed about by his own file.
#[test]
fn a_man_falling_in_goes_round_the_player_and_never_shoves_him() {
    for s in SEEDS {
        let (shoved, past) = across_sam(s, "fallIn", 0.58);
        assert!(shoved < 0.01, "seed {s}: he shoved the player {shoved:.3} m");
        assert!(past, "seed {s}: he got round him to his place");
        let (shoved, _) = across_sam(s, "move", 0.58);
        assert!(shoved > 0.1, "seed {s}: the control: a man on a move order shouldered him {shoved:.3} m");
    }
}

/// where the replay had the chair and the man who walked through it, the floor in front of the door he was making for, and the room beyond it (the compound's sala side door)
const CHAIR: [f64; 2] = [22.76, 10.12];
const FROM: [f64; 2] = [22.55, 9.04];
const FRONT: [f64; 2] = [23.25, 10.9];
const BEYOND: [f64; 2] = [22.46, 12.13];

/// One man sent past a chair: the most it moved, and how long he took, or None.
fn past_the_chair(seed: u32, chair_at: [f64; 2], from: [f64; 2], to: [f64; 2], secs: f64) -> (f64, Option<f64>) {
    let mut p = Play::mission("compound.json", "?levelFile=compound.json&roomclear&squad4", &four(seed));
    p.sam_unhurt();
    p.run(0.2);
    p.drop_all_but(&[]);
    let chair = p.prop_near(CHAIR);
    p.order("hold", p.sam_at(), Who::All);
    p.run(3.0 / 60.0);
    p.prop_put(chair, chair_at);
    let man = p.men()[0];
    p.man_stands(man, from);
    p.run(2.0 / 60.0);
    let c0 = p.prop_at(chair);
    p.order_floor("move", to, Who::Num(0.0));
    let mut moved = 0.0f64;
    let t = p.until(secs, |p| { moved = moved.max(dist(p.prop_at(chair), c0)); if to == BEYOND { p.at(man)[1] > 11.9 } else { dist(p.at(man), to) < 0.5 } });
    p.run(0.5);
    (moved.max(dist(p.prop_at(chair), c0)), t)
}

/// THE RULE: a teammate's way goes round a chair where there is floor to. Found in a replay: a chair a stride and a half short of the door, shoved ahead of a man into the doorway, where it stood pinned against the jamb and the next man walked on the spot. The control is the chair square in the doorway with no floor round it: he is not kept out, it is pushed through.
/// A FAILURE MEANS: the men plough furniture into doorways, or a chair can lock them out.
#[test]
fn a_teammate_walks_round_a_chair_and_is_not_kept_out_by_one() {
    for s in SEEDS {
        let (moved, t) = past_the_chair(s, CHAIR, FROM, FRONT, 10.0);
        assert!(moved <= 0.05 && t.is_some(), "seed {s}: the chair shoved {moved:.2} m, there after {t:?}");
        let (moved, t) = past_the_chair(s, [23.3, 11.0], [23.3, 9.6], BEYOND, 12.0);
        assert!(moved > 0.2 && t.is_some(), "seed {s}: the only way: the chair pushed {moved:.2} m, beyond the door after {t:?}");
    }
}

/// THE RULE: a man walking to a stack does not point his gun through the wall beside him: at most five in a hundred of the walking man-frames have something standing inside his light's near distance along his gun, with the gun neither the way he walks nor on a point he can see. The control is the same men with the same reader on a gun laid into the wall: it reads walled.
/// A FAILURE MEANS: an order hands a walking man the door he is sent to and he carries his gun on it through whatever stands between.
#[test]
fn a_man_walking_to_a_stack_does_not_point_his_gun_through_the_wall() {
    const ON_POINT: f64 = 0.35;
    for s in [1, 2] {
        let mut p = Play::mission("compound.json", "?levelFile=compound.json&roomclear&squad4", &four(s));
        p.sam_unhurt();
        p.run(0.5);
        let sam = p.sam_at();
        let mut doors: Vec<DoorId> = p.doors();
        doors.sort_by(|a, b| dist(p.frame_at(*a), sam).partial_cmp(&dist(p.frame_at(*b), sam)).unwrap());
        p.drop_all_but(&[]);
        p.order("fallIn", sam, Who::All);
        p.run(6.0);
        let men = p.men();
        let mut on = None;
        for &d in &doors { let c = p.frame_at(d); if p.try_order("stack", c, Who::All).is_none() { on = Some(d); break; } }
        let on = on.expect("a door takes the stack");
        let reach = 1.5;
        let (mut walk, mut walled) = (0, 0);
        let mut trail: Vec<Vec<[f64; 2]>> = vec![Vec::new(); men.len()];
        for _ in 0..30 * 60 {
            p.frame();
            for (i, &m) in men.iter().enumerate() {
                let tr = &mut trail[i];
                tr.push(p.at(m));
                if tr.len() > 11 { tr.remove(0); }
                let Some(post) = p.post_of(m) else { continue };
                if p.arrived(m) || dist(p.at(m), post) < 1.5 || tr.len() <= 10 || dist(tr[0], p.at(m)) < 0.08 { continue; }
                walk += 1;
                let along = off_by(p.gun_yaw(m), yaw_to(tr[0], p.at(m))) <= ON_POINT;
                let on_point = p.watch_of(m).is_some_and(|w| { let (a, l) = (p.at(m), dist(p.at(m), w)); let k = ((l - 0.5) / l.max(1e-6)).max(0.0); off_by(p.gun_yaw(m), yaw_to(a, w)) <= ON_POINT && p.clear_line(a, [a[0] + (w[0] - a[0]) * k, a[1] + (w[1] - a[1]) * k]) });
                if !along && !on_point && p.gun_walled(m, reach) { walled += 1; }
            }
            if men.iter().all(|&m| p.arrived(m)) { break; }
        }
        assert!(walk >= 100, "seed {s}: only {walk} walking man-frames to judge");
        assert!(walled * 100 <= walk * 5, "seed {s}: {walled} of {walk} walking man-frames with the gun into a wall");
        // the control: the reader says yes of a gun laid into a wall
        let m = men[0];
        let (a, b) = (p.on_face(on, -0.7, 0.4), p.on_face(on, -0.7, -0.4));
        p.man_stands(m, a);
        p.turned_to(m, b);
        assert!(p.gun_walled(m, reach), "seed {s}: the control: a gun laid into the wall beside the door reads walled");
    }
}
