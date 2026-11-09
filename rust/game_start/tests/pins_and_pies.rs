//! ROUNDS PAST A MAN PIN HIM, MISSES RAISE SAM'S OWN. HUNTING MEN SLICE THE DOORWAYS ON THEIR PATH.
mod world_ask;
use game::character::Stance;
use game::director::EventKind;
use game::game::GuardState;
use game::ids::GuardId;
use game_start::play::{dist, Button, Opts, Play};
use game_start::scene::{Axis, Scene};
use world_ask::YardAsk;

/// A lit yard: a sentry with a wall a pace behind him, Sam eleven metres off, the man knowing where Sam is. (That rounds past him send him to cover is blind.rs's to hold.)
fn yard(s: u32, rule: bool) -> (Play, GuardId) {
    let mut sc = Scene::new("pinned");
    sc.room("yard", 6.0, 8.0, 30.0, 20.0).door("gate", Axis::X, 20.0, 8.0);
    sc.wall_z(21.2, 12.5, 16.5);
    sc.bulb(10.0, 14.0).bulb(20.0, 14.0).bulb(26.0, 14.0);
    sc.sentry("post", [20.0, 14.0], [10.0, 14.0]).sam(9.0, 14.0);
    let mut p = Play::scene(&sc, &Opts { seed: s, ..Default::default() });
    p.g.god_mode = true; p.g.tune.suppression = rule;
    let gd = p.guard("post");
    p.sam_takes_his_rifle();
    p.run(0.3);
    (p, gd)
}
fn frame_under_fire(p: &mut Play, gd: GuardId, k: usize) {
    let a = p.at(gd);
    if k % 12 == 0 { p.point_at([a[0] + 0.9, a[1] + 1.4]); p.press(Button::Left); }
    if k % 12 == 3 { p.release(Button::Left); }
    p.frame();
}
fn his_shots(p: &Play, gd: GuardId, last: f64) -> usize { let at = p.at(gd); p.g.events.iter().filter(|e| e.kind == EventKind::GuardShot && e.time >= last && dist([e.pos[0], e.pos[2]], at) < 0.1).count() }

/// THE RULE: rounds passing close take a man's scalar over the pin line; he goes down with the gun down, and not one round leaves him while his head is down. CONTROL: the rule off: the scalar rises just the same (it is physics) but he never crouches, and he fires at his gun's cadence.
/// A FAILURE MEANS: fire does not keep a man's head down, or a pinned man shoots back.
#[test]
fn rounds_past_a_man_pin_him() {
    for s in [721u32, 722] {
        let (mut p, gd) = yard(s, true);
        let line = p.g.k.suppression.pinned;
        let (mut crossed, mut down_shots, mut crouched, mut last) = (false, 0, false, p.g.time);
        for k in 0..240 {
            frame_under_fire(&mut p, gd, k);
            let (sup, ch) = (&p.g.gd[gd].sup, p.g.gd[gd].ch);
            crossed |= sup.v >= line;
            let down = sup.down_t > 0.0 || sup.latched;
            crouched |= crossed && down && p.g.ch[ch].anim.crouch_target >= 0.99 && p.g.ch[ch].anim.stance == Stance::LowReady;
            if down { down_shots += his_shots(&p, gd, last); }
            last = p.g.time;
        }
        assert!(crossed, "seed {s}: four seconds of rounds past him and he is at {}", p.g.gd[gd].sup.v);
        assert!(crouched, "seed {s}: pinned and not crouched with the gun down");
        assert_eq!(down_shots, 0, "seed {s}: rounds left him with his head down");
        drop(p);

        let (mut p, gd) = yard(s, false);
        let (mut crossed, mut ever_down, mut last, mut shots) = (false, false, p.g.time, 0);
        for k in 0..240 {
            frame_under_fire(&mut p, gd, k);
            crossed |= p.g.gd[gd].sup.v >= line;
            let ch = p.g.gd[gd].ch;
            ever_down |= p.g.gd[gd].sup.down_t > 0.0 || p.g.ch[ch].anim.crouch_target >= 0.99;
            shots += his_shots(&p, gd, last);
            last = p.g.time;
        }
        assert!(crossed && !ever_down && shots >= 2, "seed {s}: the rule off: crossed {crossed}, down {ever_down}, {shots} rounds from him");
    }
}

/// THE RULE: a man firing at Sam: the rounds that miss raise Sam's own scalar. CONTROL: the same man with no round fired leaves it at nothing; and a round that finds him adds nothing (he is shot, not suppressed).
/// A FAILURE MEANS: the player's aim never opens up under fire, or a hit counts twice.
#[test]
fn misses_raise_sams_own() {
    for s in [731u32, 732] {
        let mut p = Play::level("office.json", false, &Opts { seed: s, ..Default::default() });
        p.g.god_mode = true;
        p.run(0.08);
        let men = p.g.guards.clone();
        for (k, &m) in men.iter().enumerate() { p.parked_far(m, k); }
        let a = men[0];
        p.sam_put([26.0, 11.4]);
        p.frame();
        p.posted_calm(a, [10.0, 11.3], [26.0, 11.4]);
        p.g.pl[p.g.player].visibility = 0.9;
        p.run(0.5);
        assert_eq!(p.g.pl[p.g.player].sup.v, 0.0, "seed {s}: he stands under nothing yet");
        (p.g.gd[a].state, p.g.gd[a].awareness, p.g.gd[a].last_known) = (GuardState::Alert, 1.0, Some([26.0, 0.0, 11.4]));
        (p.g.gd[a].pinned, p.g.gd[a].hold) = (true, false);
        let (mut peak, mut hits, mut misses, mut add_on_hit) = (0.0_f64, 0, 0, 0);
        let half = p.g.k.suppression.half_life;
        let mut last = p.g.time;
        for _ in 0..(60.0 * 6.0) as usize {
            let pl = p.g.player;
            p.g.pl[pl].visibility = 0.9; p.g.gd[a].last_seen_t = p.g.time; p.g.pl[pl].hit_flash = 0.0;
            let before = p.g.pl[pl].sup.v;
            p.frame();
            if his_shots(&p, a, last) > 0 {
                if p.g.pl[pl].hit_flash > 0.9 { hits += 1; if p.g.pl[pl].sup.v > before * (2.0_f64).powf(-(1.0 / 60.0) / half) + 1e-9 { add_on_hit += 1; } } else { misses += 1; }
            }
            last = p.g.time;
            peak = peak.max(p.g.pl[pl].sup.v);
        }
        assert!(misses >= 1 && peak >= 0.25, "seed {s}: {misses} misses, {hits} hits, peak {peak}");
        assert_eq!(add_on_hit, 0, "seed {s}: a round that found him raised it");
    }
}

/// A man hunting: alert, knowing a place he last saw Sam some seconds ago, set down at `start`.
fn hunter(s: u32, start: [f64; 2], fix: [f64; 2], ago: f64) -> (Play, GuardId) {
    let mut p = Play::level("office.json", false, &Opts { seed: s, ..Default::default() });
    p.g.god_mode = true; p.g.player_invisible = true;
    p.sam_put([37.5, 26.5]);
    p.run(0.15);
    let men = p.g.guards.clone();
    for (k, &m) in men.iter().enumerate().skip(1) { p.parked_far(m, k); }
    let gd = men[0];
    p.posted_calm(gd, start, fix);
    let h = &mut p.g.gd[gd];
    (h.heard_up_to, h.state, h.awareness, h.react_t, h.last_known, h.hold) = (1e12, GuardState::Alert, 1.0, 0.0, Some([fix[0], 0.0, fix[1]]), false);
    h.last_seen_t = p.g.time - ago;
    game::guards::reset_chase(&mut p.g, gd);
    (p, gd)
}

/// THE RULE: a hunting man whose way goes through a doorway brakes to a walk at it, slices it with his look across the opening, and goes on. CONTROL: the same door 0.7 s behind Sam, chasing and not clearing: no drill and no brake.
/// A FAILURE MEANS: men sprint blind through every door, or stop to slice a door with their man in sight.
#[test]
fn hunting_men_slice_the_doorways_on_their_path() {
    for (s, door, start, fix) in [(311u32, "conference", [13.0, 11.2], [7.0, 6.5]), (312, "server", [20.0, 11.2], [18.0, 7.0])] {
        let (mut p, gd) = hunter(s, start, fix, 3.0);
        let d = game::doors::doors_by_name(&p.g, door).expect("the door");
        let sill = p.g.dr[d].frame_centre;
        let ch = p.g.gd[gd].ch;
        let (mut phases, mut fastest, mut prev, mut crossed) = (Vec::new(), 0.0_f64, p.g.ch[ch].aim_yaw, false);
        let (mut turned, mut lo, mut hi) = (0.0_f64, 0.0_f64, 0.0_f64);
        for _ in 0..60 * 10 {
            p.frame();
            let at = p.at(gd);
            let d = p.g.ch[ch].aim_yaw - prev;
            turned += d.sin().atan2(d.cos());
            prev = p.g.ch[ch].aim_yaw;
            if let Some(pie) = &p.g.gd[gd].pieing { let w = format!("{:?}", pie.phase); if !phases.contains(&w) { phases.push(w); } if pie.phase != game::guards::PieingPhase::Approach { lo = lo.min(turned); hi = hi.max(turned); } }
            if !crossed && at[1] > sill[2] && dist(at, [sill[0], sill[2]]) < 1.5 { fastest = fastest.max(p.speed(gd)); }
            crossed |= at[1] < sill[2] - 0.5;
            if crossed { break; }
        }
        assert!(crossed, "{door}: he never went through: {} phases {phases:?}", p.roll());
        assert!(phases.iter().any(|w| w == "Slice") && phases.iter().any(|w| w == "Cross"), "{door}: phases {phases:?}");
        assert!(fastest <= p.g.tune.guard_walk + 0.12, "{door}: {fastest} m/s inside 1.5 m of the sill");
        assert!((hi - lo).to_degrees() >= 60.0, "{door}: his look swept {} degrees", (hi - lo).to_degrees());
        drop(p);
    }
    let (mut p, gd) = hunter(313, [14.5, 11.2], [6.8, 7.4], 0.7);
    let d = game::doors::doors_by_name(&p.g, "conference").expect("the door");
    let sill = p.g.dr[d].frame_centre;
    let (mut pies, mut fastest, mut crossed) = (0, 0.0_f64, false);
    for _ in 0..60 * 10 {
        p.g.gd[gd].last_seen_t = p.g.time - 0.7; p.g.gd[gd].awareness = 1.0;
        p.frame();
        if p.g.gd[gd].pieing.is_some() { pies += 1; }
        let at = p.at(gd);
        if at[1] > 10.0 && dist(at, [sill[0], sill[2]]) < 1.5 { fastest = fastest.max(p.speed(gd)); }
        if at[1] < 9.9 { crossed = true; break; }
    }
    assert!(crossed && pies == 0 && fastest > 2.0, "chasing: crossed {crossed}, {pies} frames of a pie, {fastest} m/s at the door");
}
