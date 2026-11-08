//! THE FLOOR'S ROOM CLEARS: what has just been called clear is not sent round again.
mod kit_ask;
use game_start::play::{Opts, Play};
use kit_ask::KitAsk;

/// THE RULE: a room called clear is not sent round again within a minute, unless the fix is inside it; a minute on it is dealt again.
/// A FAILURE MEANS: two men walk the same empty rooms for ever, or a noise from inside a room just cleared goes unanswered.
#[test]
fn a_room_called_clear_is_not_sent_round_again_within_a_minute() {
    const FAR: [f64; 2] = [4.9, 23.2];
    let mut p = Play::level("office.json", false, &Opts { seed: 61, ..Default::default() });
    p.sam_a_ghost();
    p.sam_unhurt();
    p.run(0.5);
    let first = p.clear_called_at([12.0, 11.2], FAR);
    assert!(first.len() >= 2, "a fix on the corridor plans the rooms nearest it: {first:?}");
    p.until(160.0, |p| !p.clearing()).expect("the clear finishes inside 160 s");
    let again = p.clear_called_at([12.0, 11.2], FAR);
    assert!(!again.is_empty() && !again.iter().any(|r| first.contains(r)), "the same fix straight after: {again:?}, cleared {first:?}");
    p.clear_called_off();
    let inside = p.clear_called_at([8.0, 7.0], FAR);
    assert_eq!(inside.first().map(String::as_str), Some("conference room"), "a fix INSIDE a room called clear brings it straight back: {inside:?}");
    p.clear_called_off();
    p.run(65.0);
    let later = p.clear_called_at([12.0, 11.2], FAR);
    assert!(first.iter().all(|r| later.contains(r)), "the control: 65 s on the same fix plans them again: {later:?}, first {first:?}");
}
