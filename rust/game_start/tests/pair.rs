//! THE SILENT PARTNER'S WORDS (rust/game_rules/src/spotted, pairorders, silent `posture_word`, guards `say`): what the squad line says while a
//! defender is onto him, and what he says when he has finished a blown job by hand. The partner is on in the page where a quiet mission brings a
//! squad, and with `?pair`.
use game::game::GuardState;
use game::ids::GuardId;
use game_start::play::{Opts, Play};

const SEEDS: [u32; 3] = [10, 11, 12];
const EAST: f64 = std::f64::consts::FRAC_PI_2;
const AISLE_MID: [f64; 2] = [22.0, 17.6];

/// The office with Sam and one partner on the aisle, the squad silent and falling in on Sam, one defender left on his feet with his back to them.
fn pair(seed: u32) -> (Play, GuardId, GuardId) { pair_with_body(seed, None).0 }
/// The same, and where `body` says the second defender stood there and was dropped.
fn pair_with_body(seed: u32, body: Option<[f64; 2]>) -> ((Play, GuardId, GuardId), Option<GuardId>) {
    let mut p = Play::mission("office.json", "?levelFile=office.json&roomclear&squad=1", &Opts { seed, ..Default::default() });
    p.g.god_mode = true;
    p.g.k.pairup.on = true;   // (what a quiet mission with a squad does, and `?pair`)
    game::pairup::pair_up_apply(&mut p.g, None);
    p.g.k.silent.on = true;
    p.run(0.3);
    let m = p.mates()[0];
    let foes: Vec<GuardId> = p.g.guards.iter().copied().filter(|&gd| p.g.gd[gd].team != game::teams::Team::Operative).collect();
    p.drop_all_but(&foes[..if body.is_some() { 2 } else { 1 }]);
    game::silent::set_silent(&mut p.g, true, None);
    let t1 = foes[0];
    let lying = body.map(|at| {
        // (stood there for his bones to be there, then dropped)
        let t2 = foes[1];
        for _ in 0..3 { let c = p.g.gd[t2].ch; p.g.ch[c].pos = [at[0], 0.0, at[1]]; p.g.gd[t2].hold = true; p.g.gd[t2].pinned = true; pin(&mut p, t1, GuardState::Patrol, 0.0); p.frame(); }
        game::combat::kill_guard(&mut p.g, t2, [0.0, 0.0, 1.0], Some(true), None, None, None, None);
        t2
    });
    pin(&mut p, t1, GuardState::Patrol, 0.0);
    p.sam_put([11.0, 18.5]);
    let c = p.g.gd[m].ch;
    p.g.ch[c].pos = [13.0, 0.0, 17.6];
    for _ in 0..10 { pin(&mut p, t1, GuardState::Patrol, 0.0); p.frame(); }
    let t = game::orders::squad_orders_target_at(&mut p.g, [11.0, 0.0, 18.5]);
    assert_eq!(game::orders::squad_orders_give(&mut p.g, "fallIn", Some(&t), Some(game::orders::Who::All), None), None, "seed {seed}: fall in was refused");
    for _ in 0..30 { pin(&mut p, t1, GuardState::Patrol, 0.0); p.frame(); }
    ((p, m, t1), lying)
}
/// the defender held where he is with his back to the pair: his senses are not what is asked here
fn pin(p: &mut Play, h: GuardId, state: GuardState, aware: f64) { p.pin_man(h, AISLE_MID, EAST, state, aware); }
fn word(p: &mut Play) -> String { game::silent::posture_word(&mut p.g).unwrap_or_default() }

/// THE RULE: the word on the squad line says what the partner is doing about it while it lasts: 'SILENT · low' with a suspicious man about,
/// 'SILENT · contact' once an alert man has them, plain 'SILENT' before.
/// A FAILURE MEANS: the line no longer reads the partner's mode (before: `posture_word` knew only 'SILENT' and 'SILENT · compromised').
#[test]
fn the_squad_line_says_low_and_contact_while_they_last() {
    for s in SEEDS {
        let (mut p, _, t1) = pair(s);
        assert_eq!(word(&mut p), "SILENT", "seed {s}: nobody is onto them yet");
        for _ in 0..60 { pin(&mut p, t1, GuardState::Suspicious, 0.3); p.frame(); }
        assert_eq!(word(&mut p), "SILENT · low", "seed {s}: a suspicious man 10 m off: {}", p.roll());
        // (his first round in their ears makes it 'SILENT · compromised', silent's own word from then on: 'contact' is what stands until it)
        let mut seen = Vec::new();
        for _ in 0..60 { pin(&mut p, t1, GuardState::Alert, 1.0); p.g.gd[t1].foe = Some(game::game::Foe::Player); p.g.gd[t1].last_seen_t = p.g.time; p.frame(); let w = word(&mut p); if !seen.contains(&w) { seen.push(w); } }
        assert!(seen.iter().any(|w| w == "SILENT · contact"), "seed {s}: an alert man has eyes on Sam and the word went {seen:?}: {}", p.roll());
    }
}
