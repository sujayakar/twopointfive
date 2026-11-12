//! WHAT A HOST DRAWS AND SHOWS OF THE GAME (the crate game_draw): made from the game as a tick left it, which is as it was.
use super::Held;
use game::handed::{LeadNow, PageLook};
use game_draw::lead::Stand;
use game_start::host::field::Laid;

fn table(x: &mut Held) -> usize { x.drawn.table().as_ptr() as usize }
pub fn draw(x: &mut Held, want: u32) -> usize { watch(x, want); x.drawn.take(&x.g, want); table(x) }
pub fn draw_over(x: &mut Held, want: u32) -> usize { watch(x, want); x.drawn.over(&x.g, want); table(x) }
pub fn frame(x: &mut Held, between: f64, focus: [f64; 3]) -> usize { x.drawn.frame(&x.g, between, focus); table(x) }
/// before a take: the ways of a game just put back's smoke are asked; and what a viewer's picture shows that the game lists nowhere is made of it
fn watch(x: &mut Held, want: u32) {
    if want & game_draw::want::smoke != 0 && x.drawn.smoke.unsaid() { game::sense::smoke_ways(&mut x.g); }
    x.drawn.watch(&x.g);
}
pub fn draw_look(x: &mut Held, Laid(look): Laid<PageLook>) { x.drawn.look(&look); }
pub fn draw_lead(x: &mut Held, lead: Option<Laid<LeadNow>>) {
    match lead { None => x.drawn.lead.rest(), Some(Laid(l)) => x.drawn.lead.tick(l.man as u32, Stand { pos: l.at, yaw: l.yaw }, Stand { pos: l.to, yaw: l.to_yaw }) }
}
pub fn draw_as(x: &mut Held, bared: bool, tube: Option<bool>) { x.g.fog_picture.viewer = true; x.drawn.view_as(bared, tube); }
pub fn draw_hold(x: &mut Held) { x.drawn.hold(); }
pub fn draw_cut(x: &mut Held) { x.drawn.cut(); }
pub fn traffic(x: &Held) -> Vec<u8> { game_draw::traffic::map(&x.g) }
