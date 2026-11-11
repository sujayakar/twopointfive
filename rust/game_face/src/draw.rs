//! WHAT A HOST DRAWS AND SHOWS OF THE GAME (the crate game_draw): made from the game as a tick left it, which is as it was.
//!
//!   face_draw(h, want) -> p            WHAT THE LAST TICK LEFT TO BE DRAWN (the crate game_draw): the sections of `want` made again from the game as it
//!                                      stands, which is as it was. `p` is a table of two words a section, its address and its rows; they lie here
//!                                      until the next call. Once a tick.
//!   face_draw_over(h, want) -> p       …the same, of a tick that is not the first a host steps before it draws a frame: what moves is blended over them all
//!   face_frame(h, a, x, y, z) -> p     A FRAME IS DRAWN, `a` of the way from the tick before to the last (1 = where the last left everything), looking at
//!                                      x y z: the rows that move stand there, and the table again
//!   face_draw_look(h, p, n)                 what the page says of its look (game_draw::LOOK numbers), once and when it changes
//!   face_draw_lead(h, id, p)           once a tick, of a networked host: its own man's place ahead of the tick (game_draw::lead); id 0 = nobody is ahead
//!   face_draw_as(h, bared, tube)       a viewer's own picture of a run (Drawn::view_as); from the first call the game keeps what such a picture is made from, tick by tick
//!   face_draw_hold(h)                  a tick of the host's in which the game was not stepped: what moves stands where the last tick left it
//!   face_draw_cut(h)                   the picture goes on from where the game stands now, and blends from nothing before it (ticks were stepped that were not drawn)
//!   face_traffic(h)                    the floors' traffic map at face_out (game_draw::traffic), worked out as it is asked: once, as a level opens
//!   face_draw_name(h, i)               a name that crosses by its number (a sound's), at face_out
//!   face_draw_layout() -> stamp        the stamp of everything declared that crosses (and its text at face_out): a page whose half is of another is refused

#[no_mangle]
pub extern "C" fn face_draw(h: u32, want: u32) -> *const u32 {
    super::with(h, |x| { watch(x, want); x.drawn.take(&x.g, want); x.drawn.table().as_ptr() })
}
#[no_mangle]
pub extern "C" fn face_draw_over(h: u32, want: u32) -> *const u32 {
    super::with(h, |x| { watch(x, want); x.drawn.over(&x.g, want); x.drawn.table().as_ptr() })
}
#[no_mangle]
pub extern "C" fn face_frame(h: u32, a: f64, x: f64, y: f64, z: f64) -> *const u32 {
    super::with(h, |k| { k.drawn.frame(&k.g, a, [x, y, z]); k.drawn.table().as_ptr() })
}
/// before a take: the ways of a game just put back's smoke are asked; and what a viewer's picture shows that the game lists nowhere is made of it
fn watch(x: &mut super::Held, want: u32) {
    if want & game_draw::want::smoke != 0 && x.drawn.smoke.unsaid() { game::sense::smoke_ways(&mut x.g); }
    x.drawn.watch(&x.g);
}
/// # Safety
/// `p` names `n` numbers the host wrote into memory from face_alloc.
#[no_mangle]
pub unsafe extern "C" fn face_draw_look(h: u32, p: *const f64, n: usize) {
    if n != game_draw::LOOK { panic!("face_draw_look: {} numbers, and a look is {}", n, game_draw::LOOK); }
    let v = std::slice::from_raw_parts(p, n);
    super::with(h, |k| k.drawn.look(v));
}
/// # Safety
/// `p` names eight numbers the host wrote into memory from face_alloc: where the tick has the man (his feet, the way he aims), and where he is drawn.
#[no_mangle]
pub unsafe extern "C" fn face_draw_lead(h: u32, id: u32, p: *const f64) {
    let stand = |v: &[f64]| game_draw::lead::Stand { pos: [v[0], v[1], v[2]], yaw: v[3] };
    super::with(h, |x| if id == 0 { x.drawn.lead.rest() } else { let v = std::slice::from_raw_parts(p, 8); x.drawn.lead.tick(id, stand(&v[..4]), stand(&v[4..])) });
}
/// `tube`: 0 as the run had it, 1 on, 2 off
#[no_mangle]
pub extern "C" fn face_draw_as(h: u32, bared: u32, tube: u32) {
    super::with(h, |x| { x.g.fog_picture.viewer = true; x.drawn.view_as(bared != 0, match tube { 1 => Some(true), 2 => Some(false), _ => None }); });
}
#[no_mangle]
pub extern "C" fn face_draw_hold(h: u32) { super::with(h, |x| x.drawn.hold()); }
#[no_mangle]
pub extern "C" fn face_draw_cut(h: u32) { super::with(h, |x| x.drawn.cut()); }
#[no_mangle]
pub extern "C" fn face_traffic(h: u32) {
    let b = super::with_ref(h, |x| game_draw::traffic::map(&x.g));
    super::out(b);
}
#[no_mangle]
pub extern "C" fn face_draw_name(h: u32, i: usize) {
    let b = super::with(h, |k| k.drawn.events.names.get(i).cloned().unwrap_or_default());
    super::out(b.into_bytes());
}
#[no_mangle]
pub extern "C" fn face_draw_layout() -> u32 {
    let t = game_draw::layout() + &game_start::host::crosses::text();
    let s = game::crosses::stamp(&t);
    super::out(t.into_bytes());
    s
}
