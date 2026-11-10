//! WHAT AN OPEN MENU SAYS AT ONE MOMENT: each row's words and the quick word the ring's first wedge says, read together as the menu opens or is read again
//! (`squad_orders_refresh`) and at the start of each frame's input while its ring is up. Which row is on which wedge is worked out over these strings (`squad_orders_seats`), by the tick and
//! by a HUD alike. Neither is on the wire: they are read again from the rest of the state.
use crate::game::Game;

/// the menu's words and quick word, read as things are
pub(super) fn read_words(g: &mut Game) {
    let Some(m) = g.orders.menu.clone() else { return };
    let mt = super::squad_orders_menu_target(&crate::seats::here(g)).unwrap();
    let words = m.rows.iter().map(|&d| super::squad_orders_label_of(g, d, &mt)).collect();
    let quick = if m.page.is_none() { super::squad_orders_menu_quick(g, &m) } else { None };
    if let Some(up) = g.orders.menu.as_mut() { up.words = words; up.quick = quick; }
}
