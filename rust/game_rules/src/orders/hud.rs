//! WHAT A SEAT'S HUD IS TOLD OF ITS SQUAD (`view::tick_view`): who the next order goes to, each man and what he is doing, the block's notes. Each line is worded by the
//! rule it is about, and comes in the list's order (`rows::ROWS`). A read: asking it writes nothing.
use crate::game::Game;
use crate::seats::SeatRef;
use crate::view::{MateLine, Note, Pair, SquadView};

/// the squad's words for the seat (None: no squad answers to it)
pub fn squad_view(g: &Game, seat: &SeatRef) -> Option<SquadView> {
    if !super::squad_orders_active(g, seat) { return None; }
    let squad = super::squad_orders_squad(g, seat);
    let to = super::squad_orders_recipients(g, seat, None, None);
    let up = squad.iter().filter(|&&gd| g.gd[gd].state != crate::game::GuardState::Dead).count();
    let picked = to.len() < up;
    let paired = super::squad_orders_paired(g, seat);
    let mates = squad.iter().map(|&gd| MateLine {
        man: g.ch[g.gd[gd].ch].id,
        name: g.gd[gd].callsign.clone(),
        doing: alloc::format!("{}{}", super::squad_orders_status_of(g, seat, gd), crate::cover::cover_tail(g, gd)),
        addressed: picked && to.contains(&gd),
        pair: if paired { g.gd[gd].unit.as_ref().and_then(|u| u.pair).map(pair_word) } else { None },
        down: g.gd[gd].state == crate::game::GuardState::Dead,
        kit: String::from(crate::kit::kit_line(g, gd)),
    }).collect();
    let notes = super::rows::ROWS.iter().filter_map(|r| r.notes).flat_map(|f| f(g, seat)).collect();
    Some(SquadView {
        posture: if seat.orders.posture.is_some() { crate::silent::posture_word(g, seat) } else { None },
        who: super::squad_orders_who_tag(g, seat).tag,
        picked,
        mates,
        notes,
        quick: quick_note(g, seat),
    })
}
/// the quick key's line on the block: what the tick keeps, and nothing where the mission has not taught the key
fn quick_note(g: &Game, seat: &SeatRef) -> Option<Note> {
    let text = seat.orders.shown.quick.clone()?;
    if crate::rails::rails_locks(g, "order") || (crate::cmdring::ring_hold_secs(g) > 0.0 && crate::rails::rails_teaches_hold(g)) { return None; }
    Some(Note { key: crate::view::key_cap(&g.k.orders.quick_key), text })
}
fn pair_word(p: crate::game::PairId) -> Pair {
    match p { crate::game::PairId::Red => Pair::Red, crate::game::PairId::Blue => Pair::Blue }
}
