//! A TEAMMATE'S TWO GUNS: dry on a man close in, the sidearm finishes it, and it goes back when the floor is quiet.
mod kit_ask;
use game_start::play::{Opts, Play};
use kit_ask::KitAsk;

fn squad(seed: u32) -> Play {
    let mut p = Play::mission("office.json", "?levelFile=office.json&roomclear&squad4", &Opts { seed, ..Default::default() });
    p.sam_unhurt();
    p.sam_a_ghost();
    p.run(0.1);
    p
}

/// THE RULE: every teammate is issued a long gun and a pistol as his second; dry with his man past the table's reach he has no reason to draw, inside it the sidearm is in his hands, the long gun on its sling, his hands busy for less than a magazine change; with a man in front of him it stays; with nobody for the table's lull it goes back, the long gun up full.
/// THE CONTROL: dry at the same spot with his man past the reach: no draw.
/// A FAILURE MEANS: a man changes magazines at arm's length from his target, or the pistol is never put away.
#[test]
fn dry_on_a_man_close_in_he_draws_and_with_nobody_in_front_he_puts_it_back() {
    for seed in [17, 18, 19] {
        let mut p = squad(seed);
        let (near, out, lull) = p.side_table();
        let m = p.mates()[0];
        assert!(!p.pistol_in_hand(m), "seed {seed}: he starts with his long gun up");
        let reload = p.reload_secs(m);
        assert!(out < reload, "seed {seed}: his hands are busy {out} s, longer than the magazine change's {reload} s");
        p.gun_dry(m);
        assert!(!p.goes_to_his_sidearm(m, near + 2.0), "seed {seed}, control: past {near} m he changes magazines");
        assert!(p.goes_to_his_sidearm(m, 4.0), "seed {seed}: at 4 m he goes to the sidearm");
        assert!(!p.goes_to_his_sidearm(m, 4.0), "seed {seed}: a second draw of one gun");
        assert!(p.pistol_in_hand(m) && p.long_gun_hung(m) && p.hands_busy(m), "seed {seed}: pistol {} · hung {} · busy {}", p.pistol_in_hand(m), p.long_gun_hung(m), p.hands_busy(m));
        let t0 = p.time();
        let mut free = None;
        for _ in 0..90 {
            p.man_in_front_of(m);
            p.frame();
            assert!(p.pistol_in_hand(m), "seed {seed}: a man in front of him and the sidearm was taken off him");
            if free.is_none() && !p.hands_busy(m) { free = Some(p.time() - t0); }
        }
        let free = free.unwrap_or_else(|| panic!("seed {seed}: his hands never come free"));
        assert!((free - out).abs() < 0.1, "seed {seed}: free after {free:.2} s, the table says {out}");
        p.gun_dry(m);
        let back = p.until(lull + 1.0, |p| !p.side_out(m)).unwrap_or_else(|| panic!("seed {seed}: the sidearm never goes back"));
        assert!(back > lull - 0.1 && back < lull + 0.2, "seed {seed}: back after {back:.2} s, the table says {lull}");
        assert!(!p.pistol_in_hand(m) && p.gun_full(m) && !p.long_gun_hung(m) && p.hands_busy(m), "seed {seed}: long gun up, full, hands on the change: {} {} {} {}", !p.pistol_in_hand(m), p.gun_full(m), !p.long_gun_hung(m), p.hands_busy(m));
    }
}

/// Two teammates with one round each five metres from three men: seconds from both guns dry to the last man down, sidearms drawn, and the guns they end on.
fn fight(seed: u32, sidearms: bool) -> (Option<f64>, f64, (i32, i32), Vec<(bool, bool)>) {
    let mut p = squad(seed);
    let men = p.men_on_the_floor();
    let (foes, rest) = men.split_at(3);
    for &r in rest { p.man_stands(r, [2.0, 2.0]); p.man_posted(r); p.g.ch[p.g.gd[r].ch].cond.dazzled_until = 1e9; }
    let mates = p.mates();
    let (two, off) = mates.split_at(2);
    for &o in off { p.man_stands(o, [10.0, 26.0]); p.man_posted(o); }
    p.man_stands(two[0], [14.2, 17.95]);
    p.man_stands(two[1], [14.0, 17.25]);
    for (k, &h) in foes.iter().enumerate() { let at = [19.5 + 0.3 * k as f64, 16.8 + 0.5 * k as f64]; p.fights_from(h, at, [14.0, 17.6], 0.0, 0.0); p.man_posted(h); }
    for &o in two { p.man_cannot_fall(o); p.gun_has(o, 1.0); if !sidearms { p.no_second_gun(o); } }
    p.sam_put([13.0, 17.6]);
    let (mut dry_at, mut last_down) = (None, None);
    let reload = p.reload_secs(two[0]);
    for _ in 0..(12 * 60) {
        p.frame();
        if dry_at.is_none() && two.iter().all(|&o| p.pistol_in_hand(o) || p.rounds_ready(o) == 0.0) { dry_at = Some(p.time()); }
        if last_down.is_none() && foes.iter().all(|&h| !p.up(h)) { last_down = Some(p.time()); }
    }
    let gap = dry_at.zip(last_down).map(|(d, l)| l - d);
    (gap, reload, p.sidearm_counts(), two.iter().map(|&o| (p.pistol_in_hand(o), p.rounds_ready(o) > 0.0)).collect())
}

/// THE RULE: dry in a fight at five metres the sidearm finishes it sooner than a magazine change would; the floor quiet, every sidearm is back and the long gun loaded.
/// THE CONTROL: the same two with no sidearm take at least the magazine change to get the next round off.
/// A FAILURE MEANS: the second gun is carried and never used in the moment it is for.
#[test]
fn dry_in_a_fight_the_sidearm_finishes_it() {
    for seed in [17, 18, 19] {
        let (gap, reload, (drawn, back), hands) = fight(seed, true);
        let gap = gap.unwrap_or_else(|| panic!("seed {seed}: the three never all went down"));
        assert!(drawn >= 1, "seed {seed}: no sidearm drawn");
        assert!(gap < reload, "seed {seed}: the last man down {gap:.2} s after the guns ran dry; a magazine change is {reload}");
        assert_eq!(drawn, back, "seed {seed}: drawn and put back");
        assert!(hands.iter().all(|&(pistol, _)| !pistol), "seed {seed}: a pistol still in his hands: {hands:?}");
        assert!(hands.iter().filter(|&&(_, loaded)| loaded).count() as i32 >= drawn, "seed {seed}: a man who drew went back to a long gun with nothing in it: {hands:?}");
    }
    let (gap, reload, (drawn, _), _) = fight(17, false);
    assert_eq!(drawn, 0, "control: he has no sidearm");
    let gap = gap.expect("control: they get there in the end");
    assert!(gap >= reload, "control: with magazines only the last man is down {gap:.2} s after the guns ran dry; a magazine change is {reload}");
}


/// THE RULE: a man left with an empty long gun on a quiet floor changes the magazine then, not when he next needs a round.
/// THE CONTROL: the same man dry with a man five metres in front of him changes it.
/// A FAILURE MEANS: the last man of a fight walks into the next room on an empty gun, and his first pull there is a click and a change.
#[test]
fn dry_on_a_quiet_floor_he_changes_the_magazine() {
    for seed in [17, 18, 19] {
        let mut p = squad(seed);
        let m = p.mates()[0];
        let foes = p.men_on_the_floor();
        p.drop_all_but(&foes[..1]);
        let foe = foes[0];
        p.man_stands(m, [14.0, 17.6]);
        p.sam_put([13.0, 17.6]);
        p.fights_from(foe, [19.5, 17.4], [14.0, 17.6], 0.0, 0.0);
        p.man_posted(foe);
        p.man_cannot_fall(foe);
        for o in p.mates() { if o != m { p.man_stands(o, [10.0, 26.0]); p.man_posted(o); } }
        p.man_cannot_fall(m);
        p.no_second_gun(m);
        p.gun_dry(m);
        let changed = p.until(6.0, |p| p.rounds_ready(m) > 0.0 || p.reloading(m).is_some());
        assert!(changed.is_some(), "seed {seed}, control: dry with a man in front of him he never changes it");
        drop(p);

        let mut p = squad(seed);
        let m = p.mates()[0];
        p.drop_all_but(&[]);
        p.man_stands(m, [14.0, 17.6]);
        p.sam_put([13.0, 17.6]);
        p.run(1.0);
        p.no_second_gun(m);
        p.gun_dry(m);
        let changed = p.until(10.0, |p| p.rounds_ready(m) > 0.0 || p.reloading(m).is_some());
        assert!(changed.is_some(), "seed {seed}: ten seconds on a quiet floor at {} rounds", p.rounds_ready(m));
    }
}
