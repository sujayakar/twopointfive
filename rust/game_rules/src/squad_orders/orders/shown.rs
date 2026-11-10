//! THE TICK KEEPS WHAT IT SHOWS. What a row or a key would do is worked out where it is done, by the tick, and kept on the seat; the panel and the view read
//! it and ask nothing. So the words and the press cannot part, and drawing is no part of the game.
use super::*;
use crate::game::Game;
use crate::ids::*;

/// Once a frame for each seat, after the menu's own input.
pub fn keep(g: &mut Game, input: InputId, cam: CameraId, canvas: crate::host_hud::View) {
    g.sets[g.seat].orders.shown = Shown { quick: quick(g, input, cam, canvas) };
}

/// the quick key's line for what this seat's cursor is on, as its press would take it: the row's words, then what it is on
fn quick(g: &mut Game, input: InputId, cam: CameraId, canvas: crate::host_hud::View) -> Option<alloc::string::String> {
    if g.sets[g.seat].orders.menu.is_some() || !squad_orders_active(g, &crate::seats::here(g)) { return None; }
    crate::scene_collision::stands(g);
    let now = crate::view::host_now(g, input, cam, canvas);
    let t = hover(g, &crate::seats::here(g), &now)?;
    let words = crate::autopilot::auto_hint(g, &t, None)?;
    Some(if matches!(t, OrderTarget::None(OrderTargetNone { man: None, .. })) { words } else { alloc::format!("{} · {}", words, squad_orders_hover_words(g, Some(&t))) })
}
