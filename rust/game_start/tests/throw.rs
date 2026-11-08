//! A SQUAD THROW IS ONE PLAN (rust/game_rules/src/squadthrow/plan.rs): who throws, from where, where it comes to rest and whom it catches is worked
//! out once, and the menu's row, its refusal, its warning, the picture of it and the order given are all read off that.
use game_start::play::{dist, Opts, Play};

const SEEDS: [u32; 2] = [5, 6];
const LEVELS: [&str; 6] = ["office.json", "block.json", "records.json", "barracks.json", "killhouse.json", "hall.json"];

/// What one held menu showed and what its press gave: the man, the spot he throws from (None: where he stands), where it comes to rest.
type Throw = (game::ids::GuardId, Option<[f64; 2]>, [f64; 2]);

fn flat(p: game::V3) -> [f64; 2] { [p[0], p[2]] }

/// The throws' page opened on `at` with the frag's row lit; false where the menu does not list it.
fn lit(p: &mut Play, at: [f64; 2]) -> bool {
    let t = game::orders::squad_orders_target_at(&mut p.g, [at[0], 0.0, at[1]]);
    if !game::orders::squad_orders_open(&mut p.g, Some(&t), None, None) { return false; }
    game::orders::squad_orders_turn(&mut p.g, Some(game::squadthrow::THROW_PAGE), None);
    let row = p.g.orders.menu.as_ref().and_then(|m| m.rows.iter().position(|&d| p.g.tab.orders_order_def[d].id == "fragThere"));
    match (row, p.g.orders.menu.as_mut()) { (Some(i), Some(m)) => { m.sel = i as f64; true } _ => false }
}

/// What the picture shows of the lit row.
fn shown(p: &mut Play) -> Option<Throw> {
    let v = game::view::throws_view(&p.g, &game::seats::here(&p.g));
    v.into_iter().find(|x| x.preview).map(|x| (*p.g.guards.iter().find(|&&gd| p.id_of(gd) == x.man).unwrap(), x.walk.as_ref().map(|_| flat(x.from)), flat(x.land)))
}

/// the walks to a throw under way, by man: the spot asked for and the spot he walks to
fn walks(p: &Play) -> Vec<(game::ids::GuardId, [f64; 2], [f64; 2])> {
    let Some(s) = p.g.m.squadthrow.states.get(p.g.player) else { return Vec::new() };
    s.walks.keys().into_iter().map(|gd| { let w = s.walks.get(gd).unwrap(); (gd, flat(w.at), flat(w.post)) }).collect()
}

/// What the press gave: the throw just made, or the walk to one that was not under way before it.
fn given(p: &mut Play, thrown: usize, before: &[(game::ids::GuardId, [f64; 2], [f64; 2])]) -> Option<Throw> {
    let log = &p.g.m.squadthrow.squadthrow_log;
    if log.len() > thrown { let x = log.last().unwrap(); let who = p.g.guards.iter().copied().find(|&gd| p.g.gd[gd].callsign == x.who)?; return Some((who, None, flat(x.rest))); }
    let new = walks(p).into_iter().find(|w| !before.contains(w))?;
    let s = p.g.m.squadthrow.states.get(p.g.player)?;
    Some((new.0, Some(new.2), flat(s.walks.get(new.0)?.arc.rest)))
}

/// THE RULE: what is shown is what is given. In a fight, with the throws' page held open a moment on a spot and the squad on the move, the man,
/// the spot and the landing the picture shows are those of the order the press gives; and a press on a row with a throw shown is not refused.
/// A FAILURE MEANS: the picture and the order have gone back to two searches (two kept answers of two ages). (A man may have moved a step between the
/// menu's reading and the press: his own throw is made again from where he is, so landings are held to half a metre. A throw that no longer goes off
/// or would now catch one of ours is another throw, and is counted apart: fewer than one in ten.)
#[test]
fn the_throw_shown_is_the_throw_given() {
    let (mut presses, mut other, mut again) = (0, Vec::new(), 0);
    for n in LEVELS { for s in SEEDS {
        let mut p = Play::mission(n, &format!("?levelFile={n}&roomclear&squad4"), &Opts { seed: s, ..Default::default() });
        p.g.god_mode = true;
        for k in 0..14 {
            let mates: Vec<_> = p.mates().into_iter().filter(|&x| p.up(x)).collect();
            let Some(&m0) = mates.first() else { break };
            let at0 = p.at(m0);
            let Some(h) = p.g.guards.clone().into_iter().filter(|&gd| p.g.gd[gd].team == game::teams::Team::Hostile && p.up(gd)).map(|gd| p.at(gd)).min_by(|a, b| dist(*a, at0).partial_cmp(&dist(*b, at0)).unwrap()) else { break };
            if k % 2 == 0 {
                let d = p.g.doors.list.iter().map(|&d| { let c = p.g.dr[d].frame_centre; [c[0], c[2]] }).min_by(|a, b| dist(*a, h).partial_cmp(&dist(*b, h)).unwrap()).unwrap_or(h);
                let t = game::orders::squad_orders_target_at(&mut p.g, [d[0], 0.0, d[1]]);
                let _ = game::orders::squad_orders_give(&mut p.g, if k % 4 == 0 { "openClear" } else { "move" }, Some(&t), Some(game::orders::Who::All), None);
            }
            p.run(1.2);
            p.g.door_tools.frags = 9.0;
            // a spot of the floor some of them have a throw to, 7 to 11 m from the first of the squad
            let lead = p.at(m0);
            let spot = (0..16).map(|b| { let a = (b as f64 + 0.37 * k as f64) * std::f64::consts::PI / 8.0; let r = 7.0 + (b % 3) as f64 * 2.0; [lead[0] + r * a.sin(), lead[1] + r * a.cos()] })
                .find(|&at| lit(&mut p, at) && shown(&mut p).is_some());
            if spot.is_none() { p.g.orders.menu = None; continue; }
            for _ in 0..(20 + 7 * k) { p.frame(); }
            let Some(i) = p.g.orders.menu.as_ref().map(|m| m.sel) else { continue };
            let Some(was) = shown(&mut p) else { p.g.orders.menu = None; continue };
            let (thrown, before) = (p.g.m.squadthrow.squadthrow_log.len(), walks(&p));
            let refused = game::orders::squad_orders_pick(&mut p.g, Some(i));
            presses += 1;
            match given(&mut p, thrown, &before) {
                Some(now) if now.0 == was.0 && now.1.zip(was.1).map_or(now.1.is_none() == was.1.is_none(), |(a, b)| dist(a, b) < 0.5) && dist(now.2, was.2) < 0.5 => {}
                Some(now) => other.push(format!("{n} seed {s}: shown {was:?}, given {now:?}")),
                None => { again += 1; other.push(format!("{n} seed {s}: shown {was:?}, refused: {refused:?}")); }
            }
        }
    } }
    println!("THROWS {presses} presses with a throw shown, {} not as shown, {again} of them refused", other.len());
    for o in &other { println!("THROWS   {o}"); }
    assert!(presses >= 20, "only {presses} presses had a throw shown: nothing held");
    assert!(other.is_empty(), "{} of {presses} presses did not give the throw that was shown: {}", other.len(), other.join(" · "));
}
