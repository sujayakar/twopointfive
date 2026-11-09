//! ONE LIST DECIDES WHOSE MAN HE IS (rust/game_rules/src/owner/ `kept`): a rule takes a man only if nobody above it in the list has him, and whatever is below does not keep him from it.
use game::ids::{GuardId, RoomfightDefId};
use game::owner::{self, by, By, TASKERS};
use game_start::play::{Opts, Play};

/// the block's room clear, whose defenders are in squads, stood and still
fn block(seed: u32) -> Play {
    let mut p = Play::mission("block.json", "?levelFile=block.json&roomclear&squad4", &Opts { seed, ..Default::default() });
    p.run(0.5);
    p
}
fn def_of(p: &Play, m: GuardId) -> RoomfightDefId { p.g.fight.defs.iter().copied().find(|&d| p.g.tab.roomfight_def[d].gd == m).expect("a defender of a room") }

/// `ask` of one man under each claim of the list in turn: the taskers that kept him from it
fn keeps(p: &mut Play, m: GuardId, mut ask: impl FnMut(&mut Play, GuardId) -> bool) -> Vec<&'static str> {
    let t = p.time();
    let mut out = Vec::new();
    for tk in TASKERS {
        if owner::has(&p.g, m, tk.by) { continue; }   // (his squad has him at rest already)
        owner::claim(&mut p.g, m, tk.by, t, None);
        if !ask(p, m) { out.push(tk.name); }
        owner::release(&mut p.g, m, tk.by, t);
    }
    out
}
fn above(b: By) -> Vec<&'static str> { TASKERS.iter().filter(|t| t.by.rank() > b.rank()).map(|t| t.name).collect() }

/// THE RULE: his squad may send him, the lights may, unless somebody above them in the list has him, and only then.
/// A FAILURE MEANS: a rule keeps a list of its own again, and the lists disagree (before: a squad counted on a man a blast had ducked, or one running from a can).
#[test]
fn the_list_and_nothing_else_keeps_a_man_from_a_rule() {
    for s in [1, 2, 3] {
        let mut p = block(s);
        let q = p.g.m.enemysquads.states.as_ref().expect("the block's men are in squads").squads[0];
        let m = p.g.tab.enemysquads_squad[q].men[0];
        assert!(game::enemysquads::home(&mut p.g, q).contains(&def_of(&p, m)), "seed {s}: the staging is wrong: his squad may not send him at rest");
        let squad = keeps(&mut p, m, |p, m| { let d = def_of(p, m); game::enemysquads::home(&mut p.g, q).contains(&d) });
        assert_eq!(squad, above(by::SQUAD_LEG), "seed {s}: who keeps a man from being sent by his squad");
        let d = def_of(&p, m);
        if game::powerout::power_out_kept(&mut p.g, d).is_none() {
            let lights = keeps(&mut p, m, |p, m| { let d = def_of(p, m); game::powerout::power_out_kept(&mut p.g, d).is_none() });
            assert_eq!(lights, above(by::BREAKER), "seed {s}: who keeps a man from the breaker");
        }
        let net = keeps(&mut p, m, |p, m| game::netquiet::free(&mut p.g, m));
        assert_eq!(net, above(by::LAST_MAN), "seed {s}: who keeps the last man from his act");
    }
}
