//! THE BOX WORLD, ASKED BY THE PAGE: the camera's, the sound's, the cursor's questions. Nothing is kept.
use std::cell::RefCell;

thread_local! {
    /// the ways round a sound's path found this tick, and for which game
    static WAYS: RefCell<(u32, game::ask::Ways)> = RefCell::new((0, Default::default()));
}
/// is the line a→b stopped: `what` 0 = to an eye (glass is passed), 1 = by anything solid
#[no_mangle]
pub extern "C" fn face_line(h: u32, what: u32, ax: f64, ay: f64, az: f64, bx: f64, by: f64, bz: f64) -> u32 {
    super::with_ref(h, |x| game::scene_collision::segment_blocked(&x.g, [ax, ay, az], [bx, by, bz], None, if what == 0 { None } else { Some(0.0) }, None)) as u32
}
/// the nearest thing along a ray, within `max` (d a unit vector): 0 nothing · 1, and at face_out five doubles: how far, the face's normal, which (a static's place, 1e6 +
/// a leaf's)
#[no_mangle]
pub extern "C" fn face_ray(h: u32, ox: f64, oy: f64, oz: f64, dx: f64, dy: f64, dz: f64, max: f64) -> u32 {
    let Some(r) = super::with_ref(h, |x| game::scene_collision::raycast_host(&x.g, &game::ask::Over::OWN, [ox, oy, oz], [dx, dy, dz], max)) else { return 0 };
    super::out([r.t, r.n[0], r.n[1], r.n[2], r.index].iter().flat_map(|v| v.to_le_bytes()).collect());
    1
}
/// the light that stands on a point (game::sense::light_asked): `p`, `n` name the owners whose carried lamps are left out, as doubles. Nothing is kept and no apply follows.
/// # Safety
/// `p`, `n` name bytes the host wrote into memory from face_alloc.
#[no_mangle]
pub unsafe extern "C" fn face_light(h: u32, x: f64, y: f64, z: f64, steady: u32, leave: f64, p: *const u8, n: usize) -> f64 {
    let owners: Vec<f64> = super::bytes(p, n).chunks_exact(8).map(|b| f64::from_le_bytes(b.try_into().unwrap())).collect();
    super::with(h, |w| game::sense::light_asked(&mut w.g, [x, y, z], &owners, steady != 0, leave))
}
/// how a sound at `s` reaches a listener at `d`: at face_out ten doubles: how far it comes, through where (x y z), from where it seems to come (x y z), how muffled (0 1),
/// how much of it carries, and 2 direct + 1 routed. A way round is searched for once a tick between the same two places of the floor; the game remembers none of it.
#[no_mangle]
pub extern "C" fn face_hear(h: u32, sx: f64, sy: f64, sz: f64, dx: f64, dy: f64, dz: f64) {
    // (taken out for the question, so a panic inside leaves no borrow behind)
    let mut k = WAYS.with(|k| std::mem::take(&mut *k.borrow_mut()));
    if k.0 != h { k = (h, Default::default()); }
    let p = super::with_ref(h, |x| game::ask::hear_kept(&x.g, &mut k.1, [sx, sy, sz], [dx, dy, dz]));
    WAYS.with(|w| *w.borrow_mut() = k);
    let how = if p.direct { 2.0 } else { 0.0 } + if p.routed { 1.0 } else { 0.0 };
    super::out([p.path_len, p.via[0], p.via[1], p.via[2], p.apparent[0], p.apparent[1], p.apparent[2], p.muffle, p.carry, how].iter().flat_map(|v| v.to_le_bytes()).collect());
}
/// the way on the floor from a to b: at face_out, two doubles (x z) a corner, the count returned; -1 where the floor has none. A search of the module's, kept by nobody here
#[no_mangle]
pub extern "C" fn face_way(h: u32, ax: f64, az: f64, bx: f64, bz: f64, max_iter: u32) -> i32 {
    let Some(w) = super::with_ref(h, |x| game::scene_collision::way_host(&x.g, [ax, 0.0, az], [bx, 0.0, bz], max_iter)) else { return -1 };
    super::out(w.iter().flat_map(|p| [p[0].to_le_bytes(), p[2].to_le_bytes()].concat()).collect());
    w.len() as i32
}
