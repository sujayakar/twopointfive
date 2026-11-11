//! THE WAY BEHIND A STACK (docs/agents/door-tools.md): a man who kept it and was left outside.
use game::orders::Who;
use game::rearwatch::RearPin;
use game_start::play::{Opts, Play};

const SEEDS: [u32; 3] = [5, 6, 7];

/// THE RULE: a man left outside whose eyes the door tools turned in has them spare again only while he keeps his way. One who has had no line to it for longer than the
/// moment allowed is let off, and his order's eyes are as the door tools left them.
/// A FAILURE MEANS: his eyes are made spare before it is asked whether he keeps the way, and stay spare on that order for good.
#[test]
fn a_man_let_off_his_way_is_not_left_with_spare_eyes() {
    for s in SEEDS {
        let mut p = Play::mission("killhouse.json", "?levelFile=killhouse.json&roomclear&squad4", &Opts { seed: s, ..Default::default() });
        p.sam_unseen();
        p.run(0.5);
        let at = { let q = p.sam_at(); [q[0], q[1] + 2.0] };
        p.order("move", at, Who::All);
        let m = p.mates()[0];
        p.until(6.0, |p| {
            let o = &p.g.groups[p.g.group].orders;
            o.asg.get(m).is_some_and(|&a| p.g.tab.task[a].post.is_some())
        }).expect("he has an order with a mark");
        let key = p.g.group;
        let a = *p.g.groups[p.g.group].orders.asg.get(m).unwrap();
        let post = p.g.tab.task[a].post.unwrap();
        // the farthest way with a door in it, off its hinges; he had it through a door that was going, and last had a line to it long ago
        let far = [post[0] + 500.0, 1.1, post[2]];
        let o = game::cover::cover_ready(&mut p.g).into_iter().find(|&o| p.g.tab.cover_opening[o].door.is_some() && game::rearwatch::rear_sees(&p.g, post, o, far).is_none()).expect("a way he cannot see");
        let door = p.g.tab.cover_opening[o].door.unwrap();
        p.g.dr[door].leaf.off = true;
        p.g.tab.task[a].spare = Some(false);
        let pin = RearPin { o, front: far, door, side: 1.0, op: None, seen_t: -100.0, a, going: Some(true), raised: None, out: None, to: None };
        p.g.m.rearwatch.books.get_mut(key).expect("the squad's book").pins.set(m, pin);
        p.frame();
        assert!(p.g.m.rearwatch.books.get(key).unwrap().pins.get(m).is_none(), "seed {s}: he is let off it");
        assert_ne!(p.g.tab.task[a].spare, Some(true), "seed {s}: his eyes are as the door tools left them");
    }
}
