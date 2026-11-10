//! The word's key: the word, while anything of the men is waiting on it; else a set charge's trigger.
use super::*;

pub fn door_tools_input(g: &mut Game, input: InputId) {
    door_tools_sync(g);
    if !crate::input::input_hit(g, input, &g.k.doortools.blow_key.clone()) { return; }
    if crate::condition::down(g, g.pl[g.player].ch) {
        if !keyed(g).is_empty() { key_taken(g, input); }
        return;
    }
    let big = crate::orders::squad_orders_big(g, &crate::seats::here(g));
    if if big { word_of_pairs(g, input) } else { word_of_pair(g, input) } { return; }
    if charge_on_key(g, input) { return; }
    if drill_has_key(g) { key_taken(g, input); }
}
/// The press is spent: nothing after this one hears the key.
fn key_taken(g: &mut Game, input: InputId) {
    let key = g.k.doortools.blow_key.clone();
    g.tab.input[input].pressed.delete(key);
}
/// The word was given a moment ago: the rest of a double tap.
fn word_tapped(g: &Game) -> bool { g.time - g.sets[g.seat].door_tools.word_key_t < g.k.doortools.word_tap }
fn word_given(g: &mut Game) { g.sets[g.seat].door_tools.word_key_t = g.time; }
/// The word's row of the menu that is up.
fn word_row(g: &Game) -> Option<usize> {
    let menu = g.sets[g.seat].orders.menu.as_ref()?;
    menu.rows.iter().position(|&d| g.tab.orders_order_def[d].word)
}
fn give_go(g: &mut Game, men: &[GuardId]) -> bool {
    let p = sam_pos(g, &crate::seats::here(g));
    crate::orders::squad_orders_close(g);
    let t = crate::orders::OrderTarget::None(crate::orders::OrderTargetNone { at: [p[0], 0.0, p[2]], man: None });
    let opts = crate::orders::GiveOpts { men: Some(men.to_vec()), ..Default::default() };
    crate::orders::squad_orders_give(g, crate::orders::rows::go::ORDER.id, Some(&t), None, Some(&opts)).is_none()
}

/// Two men's word: its row of the menu that is up, a first can still out, a hold on the word, then their stacks and the men who ran from a grenade.
fn word_of_pair(g: &mut Game, input: InputId) -> bool {
    if let Some(i) = word_row(g) {
        key_taken(g, input);
        crate::orders::squad_orders_pick(g, Some(i as f64));
        return true;
    }
    if word_tapped(g) { key_taken(g, input); return true; }
    let men = crate::orders::squad_orders_recipients(g, &crate::seats::here(g), None, None);
    if door_tools_pair_word(g, &men) {
        key_taken(g, input);
        crate::orders::squad_orders_close(g);
        return true;
    }
    let men = match g.sets[g.seat].orders.menu.as_ref().map(|x| x.target.clone()) {
        Some(t) => crate::orders::squad_orders_recipients(g, &crate::seats::here(g), Some(&t), None),
        None => door_tools_word_men(g, &crate::seats::here(g)),
    };
    if door_tools_stack_waits(g, &crate::seats::here(g), &men).is_empty() && door_tools_frag_held(g, &men).is_empty() { return false; }
    key_taken(g, input);
    if give_go(g, &men) {
        word_given(g);
        trigger_free_keyed(g);
    }
    true
}

/// The word of more than two: the menu's row, or whatever of theirs waits on it; then a first can still out.
fn word_of_pairs(g: &mut Game, input: InputId) -> bool {
    use crate::nightbreach::{night_breach_drill_ops, night_breach_state, NightBreachState as NB};
    let row = word_row(g);
    let men = match g.sets[g.seat].orders.menu.as_ref().filter(|_| row.is_some()).map(|x| x.target.clone()) {
        Some(t) => crate::orders::squad_orders_recipients(g, &crate::seats::here(g), Some(&t), None),
        None => door_tools_word_men(g, &crate::seats::here(g)),
    };
    let again = !word_tapped(g) && !door_tools_frag_held(g, &men).is_empty();
    let waiting = !door_tools_waiting_any(g, &crate::seats::here(g), &men).is_empty() || again;
    if waiting || row.is_some() {
        // (the drill's own men, set on their doors: its key is heard as well)
        let drill = waiting && matches!(night_breach_state(g), NB::Set | NB::Stacking) && any_shared(&night_breach_drill_ops(g), &men);
        if !drill { key_taken(g, input); }
        match row { Some(i) => { crate::orders::squad_orders_pick(g, Some(i as f64)); } None => { give_go(g, &men); } }
        if waiting { word_given(g); }
        return true;
    }
    let men = crate::orders::squad_orders_recipients(g, &crate::seats::here(g), None, None);
    if !door_tools_word_early(g, &men) { return false; }
    key_taken(g, input);
    crate::orders::squad_orders_close(g);
    true
}

/// The set charges his key sets off.
fn keyed(g: &Game) -> Vec<ChargeId> {
    let seat = crate::seats::here(g);
    seat.door_tools.set.iter().copied().filter(|&c| door_tools_key_is(g, &seat, c)).collect()
}
/// Those of them that are on no word still to go.
fn free_keyed(g: &Game) -> Vec<ChargeId> { keyed(g).into_iter().filter(|&c| !door_tools_on_pending_word(g, &crate::seats::here(g), c)).collect() }
pub(super) fn trigger_free_keyed(g: &mut Game) { for c in free_keyed(g) { door_tools_trigger(g, Some(c)); } }
/// The nearest of his goes; one on a word still to go keeps the key for it.
fn charge_on_key(g: &mut Game, input: InputId) -> bool {
    if keyed(g).is_empty() { return false; }
    key_taken(g, input);
    let free = free_keyed(g);
    if free.is_empty() { return true; }
    let near = nearest_of(g, free, sam_pos(g, &crate::seats::here(g)));
    door_tools_trigger(g, near);
    true
}

/// A drill not begun, with its men on orders of ours or the word just given: the press is not its 'stack up' as well.
fn drill_has_key(g: &Game) -> bool {
    use crate::nightbreach::{night_breach_drill_ops, night_breach_state, NightBreachState};
    if night_breach_state(g) != NightBreachState::Staged { return false; }
    let drill = night_breach_drill_ops(g);
    word_tapped(g) || g.sets[g.seat].door_tools.ops.iter().any(|&o| any_shared(&drill, &dop!(g, o).men))
}
