//! THE AUTOSAVE: each thing that asks for one asks once, and nothing is written while he is compromised.
mod kit_ask;
use game_start::play::{Opts, Play, DT};
use kit_ask::KitAsk;

/// The frames of `secs` as a host runs them, its keys of what was saved going on `saved`.
fn run(p: &mut Play, secs: f64, saved: &mut Vec<String>) {
    for _ in 0..(secs * 60.0).round() as usize {
        p.frame();
        saved.extend(p.told.drain(..).filter_map(|t| t.kept));
    }
}

/// THE RULE: a room entered asks for an autosave once, written after the debounce; the same room again asks nothing; another inside the least gap waits past the gap, then lands; a man alert holds it, and when he is calm it lands. The slots rotate.
/// A FAILURE MEANS: the game writes a save with a man on to Sam, which is a death loop, or a run goes long with none.
#[test]
fn each_trigger_fires_once_and_nothing_is_written_while_compromised() {
    let mut p = Play::level("office.json", false, &Opts { seed: 515, ..Default::default() });
    p.sam_a_ghost();
    p.sam_unhurt();
    p.no_civilians();
    let men = p.g.guards.clone();
    p.all_but_held(men[0]);
    p.man_posted(men[0]);
    let (mut saved, (debounce, gap)) = (Vec::new(), p.save_clocks());
    run(&mut p, 0.5, &mut saved);
    assert!(saved.is_empty(), "the staging: saved at the start: {saved:?}");
    p.sam_put([8.0, 8.0]);
    run(&mut p, DT, &mut saved);
    assert!(p.save_waiting().map_or(false, |w| w.contains("room")), "the conference room asked for nothing: {:?}", p.save_waiting());
    run(&mut p, debounce + 0.5, &mut saved);
    assert_eq!(saved.len(), 1, "the room's ask wrote {saved:?}");
    p.sam_put([9.5, 8.5]);
    run(&mut p, 0.5, &mut saved);
    assert_eq!(p.save_waiting(), None, "the same room again asked");
    p.sam_put([20.0, 11.0]);
    run(&mut p, DT, &mut saved);
    assert!(p.save_waiting().is_some(), "the corridor asked nothing");
    run(&mut p, debounce + 0.5, &mut saved);
    assert_eq!(saved.len(), 1, "a save landed inside the least gap");
    run(&mut p, gap, &mut saved);
    assert_eq!(saved.len(), 2, "the deferred ask never landed");
    assert_ne!(saved[0], saved[1], "the slots do not rotate: {saved:?}");
    run(&mut p, gap + 1.0, &mut saved);
    let n = saved.len();
    p.man_up_on_a_post(men[0], [30.0, 6.0]);
    p.sam_put([20.0, 16.0]);
    run(&mut p, DT, &mut saved);
    assert!(p.save_waiting().is_some(), "the cubicles asked nothing");
    run(&mut p, 8.0, &mut saved);
    assert_eq!(saved.len(), n, "a save was written with a man alert: {saved:?}");
    p.man_calmed(men[0]);
    run(&mut p, 1.0, &mut saved);
    assert_eq!(saved.len(), n + 1, "the control: calmed, the waiting ask did not land: {saved:?}");
}
