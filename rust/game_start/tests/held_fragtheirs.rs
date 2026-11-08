//! A GRENADE AND THE MEN ROUND IT: defenders get off one on their own wits, and a defender's own comes with a cue first and a fuse the team can beat.
mod kit_ask;
use game::ids::GuardId;
use game_start::play::{nudge, Opts, Play};
use game_start::scene::{Axis, Scene};
use kit_ask::KitAsk;

const CAR: [f64; 2] = [27.0, 17.6];
const T1: [f64; 2] = [17.0, 17.4];
const T2: [f64; 2] = [17.9, 18.3];
const SAM: [f64; 2] = [16.4, 18.4];

fn dist(a: [f64; 2], b: [f64; 2]) -> f64 { (a[0] - b[0]).hypot(a[1] - b[1]) }

fn scene(s: u32) -> Play {
    let mut sc = Scene::new("grenade-men");
    let z = 13.0 + 0.3 * nudge(s, 0);
    sc.room("west", 2.0, 10.0, 18.0, 16.0).room("east", 18.0, 10.0, 26.0, 16.0).door("between", Axis::Z, 18.0, 10.4).door("yard", Axis::X, 16.0, 21.4);
    sc.bulb(10.0, 13.0).bulb(22.0, 13.0);
    sc.sentry("open", [12.6, z], [2.0, z]).sentry("walled", [18.6, z], [26.0, z]).sam(25.0, 15.0);
    let mut p = Play::scene(&sc, &Opts { seed: s, ..Default::default() });
    p.sam_unhurt();
    p.sam_a_ghost();
    p.run(0.1);
    p
}

/// THE RULE: with his wits a man by a live grenade gets off it, and lives where the same man with none dies; the man with a wall between has no line to it and stays.
/// THE CONTROL: the same grenade, nobody thinking.
/// A FAILURE MEANS: a defender stands over a live grenade, or a wall teaches a man nothing and he runs from what he cannot see.
#[test]
fn with_his_wits_he_gets_off_a_grenade_and_the_man_behind_the_wall_stays() {
    for s in [1, 2, 3] {
        let mut p = scene(s);
        let (open, walled) = (p.guard("open"), p.guard("walled"));
        let (at, wall0) = ([15.0, p.at(open)[1]], p.at(walled));
        p.frag_lands(at);
        let (_, _, _, fuse) = p.frag_reach();
        p.run(fuse + 0.5);
        assert!(p.up(open), "seed {s}: he has his wits and three seconds, and dies at {:.1} m", dist(p.at(open), at));
        assert!(dist(p.at(walled), wall0) < 1.0, "seed {s}: behind the wall he has no line to it and ran {:.1} m", dist(p.at(walled), wall0));
        drop(p);

        let mut p = scene(s);
        let open = p.guard("open");
        let at = [15.0, p.at(open)[1]];
        p.no_wits();
        p.frag_lands(at);
        p.run(fuse + 0.5);
        assert!(!p.up(open), "seed {s}, control: with no wits he stands there and takes it");
    }
}

struct Throw { shout: Option<f64>, out: Option<f64>, fuse: Option<f64>, marked: bool, told: bool, called: bool, burst: Option<[f64; 2]>, clear: bool, sam_up: bool, own: bool }

/// The corridor: a carrier (or, for the control, a man with nothing to throw) at its east end, staged alert on two teammates bunched ten metres west of him, Sam beside them on his own keys, moving on the cue.
fn at_the_corridor(seed: u32, carrier: bool) -> Throw {
    let mut p = Play::mission("office.json", "?levelFile=office.json&roomclear&squad4", &Opts { seed, ..Default::default() });
    p.run(0.1);
    let (men, ops) = (p.men_on_the_floor(), p.mates());
    let cars = p.frag_carriers();
    let c = men.iter().copied().find(|m| cars.contains(m) == carrier).unwrap_or_else(|| panic!("seed {seed}: no such man ({} carriers)", cars.len()));
    for (i, &h) in men.iter().enumerate() { if h != c { p.man_stands(h, [2.0 + 1.5 * i as f64, 2.0]); p.man_posted(h); p.g.gd[h].dazzled_until = 1e9; } }
    for &x in [c].iter().chain(ops.iter()) { p.man_cannot_fall(x); }
    p.man_stands(ops[0], T1);
    p.man_stands(ops[1], T2);
    for &o in &ops[2..] { p.man_stands(o, [10.0, 26.0]); }
    p.sam_a_ghost();   // (the carrier's gun is on the two he is throwing at: this is about the grenade, not the gunfight round it)
    p.sam_put(SAM);
    p.camera_square();
    p.fights_from(c, CAR, T1, 0.0, 0.0);
    let n = p.said_so_far();
    let mut t = Throw { shout: None, out: None, fuse: None, marked: false, told: false, called: false, burst: None, clear: true, sam_up: true, own: false };
    for _ in 0..(14 * 60) {
        for k in ["KeyW", "KeyA", "KeyS", "KeyD"] { p.key_up(k); }
        if let Some(goal) = p.frag_step_off() {
            let at = p.sam_at();
            let (dx, dz, d) = (goal[0] - at[0], goal[1] - at[1], dist(goal, at));
            if d > 0.2 {
                p.key_down("ShiftLeft");
                if dx / d > 0.38 { p.key_down("KeyD"); } else if dx / d < -0.38 { p.key_down("KeyA"); }
                if dz / d > 0.38 { p.key_down("KeyS"); } else if dz / d < -0.38 { p.key_down("KeyW"); }
            }
        } else { p.key_up("ShiftLeft"); }
        p.frame();
        for (who, text) in p.said_since(n) {
            if who == c && t.shout.is_none() && ["frag out", "grenade out", "fire in the hole"].iter().any(|w| text.contains(w)) { t.shout = Some(p.time()); }
            if who != c && text.contains("grenade!") { t.called = true; }
        }
        if t.out.is_none() { if let Some(f) = p.live_frag_fuse() { t.out = Some(p.time()); t.fuse = Some(f); } }
        t.marked |= !p.view().marks.frags.is_empty();
        t.told |= p.log().iter().any(|l| l.contains("GRENADE"));
        if p.burst_at().is_some() { break; }
    }
    t.burst = p.burst_at();
    if let Some(b) = t.burst {
        let (lethal, ..) = p.frag_reach();
        for &o in &ops[..2] { let at = p.at(o); t.clear &= dist(at, b) > lethal || p.in_a_burst(b, at).0 == 0.0; }
        t.own = p.burst_hits().iter().any(|(team, _)| *team == game::teams::Team::Hostile);
    }
    t.sam_up = !p.sam_down();
    t
}

/// THE RULE: a carrier with two of the team bunched in his view shouts, then lets it go after the wind-up, with the whole fuse on it; the player is told and it is marked; a teammate calls it; his men are clear when it goes; a player who moves on the cue is up; nobody of the thrower's side is hit.
/// THE CONTROL: the same staging with a man dealt no grenade throws nothing.
/// A FAILURE MEANS: a defender's grenade kills from nowhere with nothing to read, the one way this weapon is unfair.
#[test]
fn a_defenders_grenade_gives_a_cue_and_a_fuse_the_team_can_beat() {
    for seed in [3, 5, 8] {
        let t = at_the_corridor(seed, true);
        let (out, shout) = (t.out.unwrap_or_else(|| panic!("seed {seed}: no throw")), t.shout.unwrap_or_else(|| panic!("seed {seed}: no shout")));
        let probe = Play::mission("office.json", "?levelFile=office.json&roomclear&squad4", &Opts { seed, ..Default::default() });
        let (fuse, wind) = probe.their_frag();
        drop(probe);
        assert!(out - shout > wind - 0.1, "seed {seed}: it left his hand {:.2} s after the shout; the wind-up is {wind}", out - shout);
        assert!((t.fuse.unwrap() - fuse).abs() < 0.06, "seed {seed}: fuse {:?} of {fuse}: it was cooked", t.fuse);
        assert!(t.told && t.marked, "seed {seed}: HUD line {} · mark {}", t.told, t.marked);
        assert!(t.called, "seed {seed}: no teammate calls it");
        assert!(t.burst.is_some(), "seed {seed}: it never went off");
        assert!(t.clear, "seed {seed}: a teammate is inside the lethal ring when it goes");
        assert!(t.sam_up, "seed {seed}: a player who moved on the cue is down");
        assert!(!t.own, "seed {seed}: the thrower's own side is hit");
    }
    let c = at_the_corridor(3, false);
    assert!(c.out.is_none() && c.burst.is_none(), "control: a man with no grenade threw one");
}

/// THE RULE: one defender in the table's share carries one, at most the table's maximum, and it is the same men each time the floor is stood.
/// THE CONTROL: a floor with another number of men deals by the same sum, to its own men.
/// A FAILURE MEANS: a replay deals other men the grenades, or every man on the floor has one.
#[test]
fn the_deal_of_grenades_is_the_tables_and_the_same_each_time() {
    let stood = |level: &str, seed: u32| -> (Vec<f64>, usize, f64, f64) {
        let mut p = Play::mission(level, &format!("?levelFile={level}&roomclear&squad4"), &Opts { seed, ..Default::default() });
        p.run(0.1);
        let (n, share, most) = p.frag_deal();
        (p.frag_carriers().iter().map(|&g: &GuardId| p.id_of(g)).collect(), n, share, most)
    };
    let (a, n, share, most) = stood("office.json", 3);
    assert_eq!(a, stood("office.json", 3).0, "the same men each time");
    assert_eq!(a.len() as f64, most.min((n as f64 / share).ceil()), "{n} men, one in {share}, at most {most}");
    assert!(a.len() < n, "{n} men and {} grenades: every man has one", a.len());
    let (b, m, ..) = stood("mousehole.json", 3);
    assert_ne!(n, m, "the two floors differ in the one thing");
    assert_eq!(b.len() as f64, most.min((m as f64 / share).ceil()), "control: {m} men, one in {share}, at most {most}");
}
