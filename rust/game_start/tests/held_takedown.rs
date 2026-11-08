//! TAKEDOWNS AS MOVES: a partner's blow lands inside his takedown, and a choke lowers a man to the floor.
mod kit_ask;
use game::ids::GuardId;
use game_start::play::{dist, Button, Opts, Play};
use game_start::scene::{Axis, Scene};
use kit_ask::KitAsk;

const EAST: f64 = std::f64::consts::FRAC_PI_2;
const FOE: [f64; 2] = [22.0, 17.6];

/// The office with Sam and one partner on the aisle three metres behind a defender's back, the floor silent.
fn pair(seed: u32) -> (Play, GuardId, GuardId) {
    let mut p = Play::mission("office.json", "?levelFile=office.json&roomclear=10&squad=1", &Opts { seed, ..Default::default() });
    p.sam_unhurt();
    p.quiet_pair();
    let (m, foes) = (p.mates()[0], p.men_on_the_floor());
    let foe = foes[0];
    p.drop_all_but(&foes[..1]);
    p.man_stands(m, [19.0, 17.6]);
    p.sam_put([11.0, 18.5]);
    for _ in 0..10 { p.pin_man(foe, FOE, EAST, game::game::GuardState::Patrol, 0.0); p.frame(); }
    (p, m, foe)
}

/// THE RULE: told 'take him now', the partner's hands go on, the blow lands, and he is counted out only after it: from the blow until then he is on his feet and his head moves more than 10 cm; one blow is counted.
/// THE CONTROL: the man nobody is told to take stays up, and his head moves less than that in four seconds of standing.
/// A FAILURE MEANS: a partner's takedown is a man falling over with another man nearby.
#[test]
fn a_partners_blow_lands_inside_his_takedown() {
    for seed in [10, 11, 12] {
        let (mut p, _, foe) = pair(seed);
        assert_eq!(p.take_him_now(foe), None, "seed {seed}: 'take him now' refused");
        let (mut hands, mut landed, mut head_at, mut moved) = (None, None, None, 0.0f64);
        let (mut on_feet, mut out) = (true, None);
        let hit = p.partner_hit_secs();
        for _ in 0..(16 * 60) {
            p.pin_man(foe, FOE, EAST, game::game::GuardState::Patrol, 0.0);
            p.frame();
            let t = p.time();
            if hands.is_none() && p.partner_blows() == 1 { hands = Some(t); }
            if let (Some(h), None) = (hands, landed) { if t - h >= hit { landed = Some(t); head_at = Some(p.head(foe)); } }
            if !p.up(foe) { out = Some(t); break; }
            if let Some(h0) = head_at {
                let h = p.head(foe);
                moved = moved.max(((h[0] - h0[0]).powi(2) + (h[1] - h0[1]).powi(2) + (h[2] - h0[2]).powi(2)).sqrt());
                on_feet &= p.lowest_joint(p.body_of(foe)) > 0.05;
            }
        }
        let (landed, out) = (landed.unwrap_or_else(|| panic!("seed {seed}: his hands never went on")), out.unwrap_or_else(|| panic!("seed {seed}: he was never out")));
        assert!(landed < out, "seed {seed}: the blow at {landed:.2}, out at {out:.2}");
        assert!(on_feet && moved > 0.1, "seed {seed}: on his feet {on_feet}, his head moved {:.1} cm", moved * 100.0);
        assert_eq!(p.partner_blows(), 1, "seed {seed}: blows counted");
    }
    let (mut p, _, foe) = pair(10);
    let h0 = p.head(foe);
    for _ in 0..(4 * 60) { p.pin_man(foe, FOE, EAST, game::game::GuardState::Patrol, 0.0); p.frame(); }
    let h = p.head(foe);
    assert!(p.up(foe) && p.partner_blows() == 0, "control: nobody told, and he went down");
    assert!(dist([h[0], h[2]], [h0[0], h0[2]]) < 0.1, "control: nobody near him and his head moved");
}

/// Sam a body's length behind a calm man, empty handed, with him in the arm.
fn in_the_arm(s: u32) -> (Play, GuardId) {
    let mut sc = Scene::new("choke");
    sc.room("room", 10.0, 10.0, 26.0, 18.0).door("south", Axis::X, 18.0, 20.0).bulb(16.0, 14.0);
    sc.sentry("man", [18.0, 14.0], [26.0, 14.0]).sam(17.2, 14.0);
    let mut p = Play::scene(&sc, &Opts { seed: s, ..Default::default() });
    p.sam_unhurt();
    p.sam_arms("nothing");
    let man = p.guard("man");
    p.sam_grabs(man);
    p.until(2.5, |p| p.in_the_arm()).unwrap_or_else(|| panic!("seed {s}: the grab never settled"));
    (p, man)
}

/// THE RULE: the right button kept down lowers him: nothing of either body but the feet comes within 8 cm of the floor on the way, and he is out at the end of it.
/// THE CONTROL: let go part way down, both stand as they did in the hold a second later, neither pelvis more than 2 cm a frame nor a joint of Sam's 4, and the hold goes on.
/// A FAILURE MEANS: a choke sinks a man through the floor, or letting go pops both bodies back to their feet.
#[test]
fn a_choke_lowers_him_and_eased_off_both_stand_back_up() {
    for s in [1, 2, 3] {
        let (mut p, man) = in_the_arm(s);
        let (him, sam) = (p.body_of(man), p.body_of_sam());
        p.run(0.2);
        p.press(Button::Right);
        let mut low = f64::INFINITY;
        for _ in 0..(8 * 60) {
            p.frame();
            if !p.up(man) { break; }
            low = low.min(p.lowest_joint(him)).min(p.lowest_joint(sam));
        }
        assert!(!p.up(man), "seed {s}: eight seconds on the button and he is up");
        assert!(low >= 0.08, "seed {s}: a joint {:.1} cm off the floor", low * 100.0);
        drop(p);

        let (mut p, man) = in_the_arm(s);
        let (him, sam) = (p.body_of(man), p.body_of_sam());
        p.run(0.2);
        let (his0, sam0) = (p.hips_of(him)[1], p.hips_of(sam)[1]);
        p.press(Button::Right);
        p.until(6.0, |p| his0 - p.hips_of(him)[1] > 0.1).unwrap_or_else(|| panic!("seed {s}: he never dipped"));
        p.release(Button::Right);
        let (mut was_him, mut was_sam, mut joints) = (p.hips_of(him), p.hips_of(sam), p.joints(sam));
        let (mut hips, mut sam_joint) = (0.0f64, 0.0f64);
        for _ in 0..60 {
            p.frame();
            let (a, b, j) = (p.hips_of(him), p.hips_of(sam), p.joints(sam));
            let d = |x: [f64; 3], y: [f64; 3]| ((x[0] - y[0]).powi(2) + (x[1] - y[1]).powi(2) + (x[2] - y[2]).powi(2)).sqrt();
            hips = hips.max(d(a, was_him)).max(d(b, was_sam));
            for (x, y) in j.iter().zip(joints.iter()) { sam_joint = sam_joint.max(d(*x, *y)); }
            (was_him, was_sam, joints) = (a, b, j);
        }
        assert!(p.up(man) && p.in_the_arm(), "seed {s}: eased off, the hold goes on");
        assert!((p.hips_of(him)[1] - his0).abs() <= 0.03 && (p.hips_of(sam)[1] - sam0).abs() <= 0.03, "seed {s}: his hips {:.3} from {his0:.3}, Sam's {:.3} from {sam0:.3}", p.hips_of(him)[1], p.hips_of(sam)[1]);
        assert!(hips <= 0.02, "seed {s}: a pelvis {:.1} cm in a frame", hips * 100.0);
        assert!(sam_joint <= 0.04, "seed {s}: a joint of Sam's {:.1} cm in a frame", sam_joint * 100.0);
    }
}
