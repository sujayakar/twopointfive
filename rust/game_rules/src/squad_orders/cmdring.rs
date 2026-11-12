pub use ::game_types::cmdring::types;
pub use types::*;
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use alloc::{string::String, vec, vec::Vec};

/// A place held `m` in from both ends of `0 … max`; a view too small to hold it leaves it alone.
fn cl(v: f64, m_: f64, max: f64) -> f64 {
    if max > 2.0 * m_ { m::max(m_, m::min(max - m_, v)) } else { v }
}
/// `(r.listed ?? r.targets).includes('none')`
fn lists_none(listed: &Option<Vec<String>>, targets: &[String]) -> bool {
    listed.as_deref().unwrap_or(targets).iter().any(|t| t == "none")
}

/// How much larger than CMDRING's px a ring stands in a view this size. The tick's ringOpen reads it, so it is translated.
pub(super) fn ring_zoom(g: &crate::game::Game, w: f64, h: f64) -> f64 {
    zoom_of(&g.k.cmdring, w, h)
}
/// ringZoom on the knobs alone.
fn zoom_of(c: &crate::knobs::CmdringK, w: f64, h: f64) -> f64 {
    m::max(1.0, m::min(w / c.ref_[0], h / c.ref_[1]))
}

/// ringOpen with the cursor in hand.
fn ring_open_at(c: &crate::knobs::CmdringK, mouse_x: f64, mouse_y: f64, w: f64, h: f64, time: f64, lit: f64) -> RingState {
    let z = if js::truthy(w) && js::truthy(h) { zoom_of(c, w, h) } else { 1.0 };
    let ax = cl(mouse_x, c.edge_x * z, w);
    let ay = cl(mouse_y, c.edge_y * z, h);
    RingState {
        ax,
        ay,
        z,
        t0: time,
        lit,
        armed: m::hyp2(mouse_x - ax, mouse_y - ay) < c.flick_px * z,
        free: None,
        more: None,
        at: None,
        ox: None,
        oy: None,
        mx: None,
        my: None,
        clear: None,
        pt: None,
    }
}
/// A ring opened at the cursor, whole inside the view's edge; a cursor then out of its middle lights nothing until it has come back.
pub fn ring_open(g: &crate::game::Game, input: InputId, w: f64, h: f64, time: f64, lit: Option<f64>) -> crate::cmdring::RingState {
    let lit = lit.unwrap_or(1.0);
    let (mouse_x, mouse_y) = (g.tab.input[input].mouse_x, g.tab.input[input].mouse_y);
    ring_open_at(&g.k.cmdring, mouse_x, mouse_y, w, h, time, lit)
}

/// ringPin on the ring itself.
fn ring_pin_on(c: &crate::knobs::CmdringK, r: &mut RingState, sx: f64, sy: f64, w: f64, h: f64) {
    r.ax = cl(sx + r.ox.unwrap_or(0.0), c.edge_x * r.z, w);
    r.ay = cl(sy + r.oy.unwrap_or(0.0), c.edge_y * r.z, h);
}
/// The ring put back over its place, whole inside the view's edge.
pub fn ring_pin(g: &crate::game::Game, r: &mut crate::cmdring::RingState, sx: f64, sy: f64, w: f64, h: f64) {
    ring_pin_on(&g.k.cmdring, r, sx, sy, w, h);
}

/// ringWant on the ring itself, with the cursor and `pill` (the cursor on the lone row's pill) in hand.
fn ring_want_on(c: &crate::knobs::CmdringK, r: &mut RingState, mouse_x: f64, mouse_y: f64, digit: f64, time: f64, lone: f64, shapes: f64, pill: bool) -> f64 {
    let (wedges, flick_px, tap_secs) = (c.wedges, c.flick_px, c.tap_secs);
    let (dx, dy) = (mouse_x - r.ax, mouse_y - r.ay);
    let d = m::hyp2(dx, dy);
    let out = d >= flick_px * r.z;
    let early = time - r.t0 < tap_secs - 1e-6;
    let at = wedge_at(dx, dy, wedges);
    let on = shapes != 0.0
        && (if js::truthy(lone) {
            pill
        } else {
            d >= crate::ringlone::RING_HOLE * r.z && {
                let bx = crate::ringlone::ring_box();
                m::abs(dx) <= bx.x * r.z
                    && m::abs(dy) <= bx.y * r.z
                    && (js::to_i32(shapes) >> (js::to_u32(at) & 31)) & 1 == 1
            }
        });
    if js::truthy(shapes) && !on {
        r.clear = Some(true);
    }
    if early {
        if out {
            r.armed = false;
        }
    } else if !out {
        r.armed = true;
    }
    if js::truthy(digit) {
        r.armed = false;
        r.clear = Some(false);
        return if digit <= wedges { digit } else { 0.0 };
    }
    if early {
        return 0.0;
    }
    if on && r.clear == Some(true) {
        return js::or(lone, at);
    }
    if !out || !r.armed {
        return 0.0;
    }
    if pill { lone } else { at }
}
/// The wedge this frame asks for (0: none): a digit's own, which disarms the cursor, else the one the cursor is out toward or on.
pub fn ring_want(g: &crate::game::Game, r: &mut crate::cmdring::RingState, input: InputId, digit: f64, time: f64, lone: Option<f64>, shapes: Option<f64>) -> f64 {
    let (lone, shapes) = (lone.unwrap_or(0.0), shapes.unwrap_or(0.0));
    let (mouse_x, mouse_y) = (g.tab.input[input].mouse_x, g.tab.input[input].mouse_y);
    let wedges = g.k.cmdring.wedges;
    let pill = lone > 0.0 && crate::ringlone::on_pill(lone, wedges, (mouse_x - r.ax) / r.z, (mouse_y - r.ay) / r.z);
    ring_want_on(&g.k.cmdring, r, mouse_x, mouse_y, digit, time, lone, shapes, pill)
}

/// Which row is on which wedge: FLAT_TOP's rows on their wedges, the rest on what is left or behind one 'more'.
pub(super) fn flat_seats(g: &crate::game::Game, rows: &[crate::cmdring::FlatRow], kind: &str, lead: f64) -> crate::cmdring::FlatSeats {
    flat_seats_of(g.k.cmdring.wedges, rows, kind, lead)
}
/// flatSeats on the number of wedges alone.
fn flat_seats_of(wedges: f64, rows: &[FlatRow], kind: &str, lead: f64) -> FlatSeats {
    let w = wedges as usize;
    let mut row: Vec<f64> = vec![-1.0; w + 1];
    let (flat_top, flat_left) = (flat_top(), flat_left());
    let top = flat_top.of(kind);
    let left = flat_left.of(kind);
    let mut rest: Vec<f64> = Vec::new();
    let mut last: Vec<f64> = Vec::new();
    if lead >= 0.0 {
        row[1] = lead;
    }
    for (i, r) in rows.iter().enumerate() {
        let i = i as f64;
        if r.word || left.map_or(false, |l| l.contains(&r.id)) {
            continue;
        }
        let k = top.and_then(|t| t.get(r.id.clone())).copied().unwrap_or(0.0);
        if k >= 2.0 && k <= wedges && row[k as usize] < 0.0 {
            row[k as usize] = i;
            continue;
        }
        if i == lead {
            continue;
        }
        if r.free && lists_none(&r.listed, &r.targets) {
            last.push(i);
        } else {
            rest.push(i);
        }
    }
    let mut spare: Vec<usize> = Vec::new();
    for d in 2..=w {
        if row[d] < 0.0 {
            spare.push(d);
        }
    }
    if rest.len() + last.len() <= spare.len() {
        for (n, i) in rest.iter().enumerate() {
            row[spare[n]] = *i;
        }
        for (n, i) in last.iter().enumerate() {
            row[spare[spare.len() - last.len() + n]] = *i;
        }
        return FlatSeats { row, more: 0.0, over: Vec::new() };
    }
    for &d in &spare {
        if d < w && !rest.is_empty() {
            row[d] = rest.remove(0);
        }
    }
    let over: Vec<f64> = if rest.len() + last.len() > w {
        rest.truncate(w);
        rest
    } else {
        rest.extend_from_slice(&last);
        rest
    };
    if over.len() == 1 {
        row[w] = over[0];
        return FlatSeats { row, more: 0.0, over: Vec::new() };
    }
    let more = if over.is_empty() { 0.0 } else { wedges };
    FlatSeats { row, more, over }
}

/// How long the key is kept down before its ring goes up (CMDRING.stay, the page's way: a ring that stays).
pub(super) fn ring_hold_secs(g: &crate::game::Game) -> f64 {
    g.k.cmdring.stay_hold_secs
}

/// One frame of the latch of a ring that stays.
pub(super) fn ring_latch(r: &mut crate::cmdring::RingState, key_down: bool, key_hit: bool, pointed: bool) -> crate::cmdring::RingLatch {
    if r.free == Some(true) {
        return if key_hit { RingLatch::Give } else { RingLatch::Stay };
    }
    if key_down {
        return RingLatch::Hold;
    }
    if pointed {
        return RingLatch::Give;
    }
    r.free = Some(true);
    RingLatch::Stay
}

/// The wedge a direction points at: 1 at the top, clockwise, `n` of them.
fn wedge_at(dx: f64, dy: f64, n: f64) -> f64 {
    let k = m::round(m::atan2(dx, -dy) / (2.0 * m::PI / n));
    js::rem(js::rem(k, n) + n, n) + 1.0
}

/// each kind of target's rows that hold a place at the top of the ring, by the place
fn flat_top() -> JsMap<String, JsMap<String, f64>> {
    let place = |rows: &[(&str, f64)]| { let mut t = JsMap::new(); for &(id, at) in rows { t.set(String::from(id), at); } t };
    let mut t = JsMap::new();
    t.set(String::from("floor"), place(&[(crate::orders::rows::cover::ORDER.id, 2.0), (crate::orders::rows::fallin::ORDER.id, 3.0), (crate::orders::rows::hold::ORDER.id, 4.0)]));
    t.set(String::from("door"), place(&[(crate::orders::rows::cover::ORDER.id, 2.0), (crate::orders::rows::openclear::ORDER.id, 3.0), (crate::orders::rows::bangclear::ORDER.id, 4.0)]));
    t.set(String::from("none"), place(&[(crate::orders::rows::fallin::ORDER.id, 3.0), (crate::orders::rows::hold::ORDER.id, 4.0)]));
    t.set(String::from("foe"), JsMap::new());
    t
}

/// each kind of target's rows that go last
fn flat_left() -> JsMap<String, Vec<String>> {
    let mut t = JsMap::new();
    t.set(String::from("door"), alloc::vec![String::from(crate::orders::rows::fallin::ORDER.id), String::from(crate::orders::rows::hold::ORDER.id)]);
    t
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The page's own row of knobs: these tests reach no game.
    fn page() -> crate::knobs::CmdringK { crate::knobs::CmdringK::page() }

    /// A failure means the cursor lights a wedge it is not out toward.
    #[test]
    fn a_direction_points_at_its_wedge_the_first_at_the_top_going_clockwise() {
        for n in [5.0, 8.0] {
            let mut k = 0.0;
            while k < n {
                let a = k * 2.0 * m::PI / n;
                assert_eq!(wedge_at(30.0 * m::sin(a), -30.0 * m::cos(a), n), k + 1.0, "wedge {k} of {n}");
                k += 1.0;
            }
        }
        assert_eq!(wedge_at(10.0, -29.5, 5.0), 1.0);    // a little off its middle is still that wedge
    }

    #[test]
    fn a_ring_stands_larger_in_a_larger_view_and_opens_whole_inside_it() {
        let c = page();
        let (w, h) = (c.ref_[0] * 2.0, c.ref_[1] * 2.0);
        assert_eq!(zoom_of(&c, w, h), 2.0);
        assert_eq!(zoom_of(&c, w, c.ref_[1]), 1.0);    // the narrower way decides
        assert_eq!(zoom_of(&c, 640.0, 360.0), 1.0);    // never under its own size

        // at the middle of the view: at the cursor, and the cursor ready to flick
        let r = ring_open_at(&c, w / 2.0, h / 2.0, w, h, 3.5, 1.0);
        assert_eq!((r.ax, r.ay, r.z, r.armed, r.t0, r.lit), (w / 2.0, h / 2.0, 2.0, true, 3.5, 1.0));
        // in a corner: held in by its edge at its own size, and the cursor, now out of its middle, lights nothing until it has come back
        let r = ring_open_at(&c, w - 40.0, 20.0, w, h, 0.0, 1.0);
        assert_eq!((r.ax, r.ay, r.armed), (w - c.edge_x * 2.0, c.edge_y * 2.0, false));
        // a view too small to hold it leaves it where the cursor is; a view not known yet is its own size
        let r = ring_open_at(&c, 100.0, 60.0, 200.0, 100.0, 0.0, 1.0);
        assert_eq!((r.ax, r.ay, r.armed), (100.0, 60.0, true));
        assert_eq!(ring_open_at(&c, 50.0, 50.0, 0.0, 0.0, 0.0, 1.0).z, 1.0);
    }

    fn base(armed: bool) -> RingState {
        RingState { ax: 500.0, ay: 400.0, z: 1.5, t0: 10.0, lit: 1.0, armed, ..RingState::default() }
    }

    #[test]
    fn a_pinned_ring_stays_over_its_place_and_whole_inside_the_view() {
        let c = page();
        let (w, h) = (1920.0, 1080.0);
        let mut r = base(false);
        r.ox = Some(3.0);
        r.oy = Some(-4.0);
        ring_pin_on(&c, &mut r, 960.0, 540.0, w, h);
        assert_eq!((r.ax, r.ay), (963.0, 536.0));
        ring_pin_on(&c, &mut r, 10.0, 10.0, w, h);
        assert_eq!((r.ax, r.ay), (c.edge_x * r.z, c.edge_y * r.z));
        ring_pin_on(&c, &mut r, w, h, w, h);
        assert_eq!((r.ax, r.ay), (w - c.edge_x * r.z, h - c.edge_y * r.z));
    }

    /// The ring is at (500, 400), opened at time 10, with a middle of 36 px at its size and no wedge drawn. (answer, armed after, clear after)
    fn want(mx: f64, my: f64, digit: f64, t: f64, armed: bool) -> (f64, bool, Option<bool>) {
        let mut r = base(armed);
        let got = ring_want_on(&page(), &mut r, mx, my, digit, t, 0.0, 0.0, false);
        (got, r.armed, r.clear)
    }

    /// A failure means a tap of the key gives an order, or a cursor resting out of a new ring's middle gives one.
    #[test]
    fn a_flick_out_of_the_middle_picks_a_wedge_once_the_tap_is_over() {
        assert_eq!(want(540.0, 400.0, 0.0, 10.2, true), (2.0, true, None));
        assert_eq!(want(500.0, 300.0, 0.0, 10.2, true), (1.0, true, None));
        // the tap's last moment: nothing, and a cursor out by then is not ready; at its end, the wedge
        assert_eq!(want(420.0, 470.0, 0.0, 10.149999, true), (0.0, false, None));
        assert_eq!(want(420.0, 470.0, 0.0, 10.15, true), (4.0, true, None));
        // not ready: nothing, until it has been back in the middle
        assert_eq!(want(540.0, 400.0, 0.0, 10.2, false), (0.0, false, None));
        assert_eq!(want(500.0, 400.0, 0.0, 10.2, false), (0.0, true, None));
    }

    #[test]
    fn a_digit_picks_its_wedge_at_once_and_the_cursor_must_come_back() {
        assert_eq!(want(510.0, 400.0, 3.0, 10.0, true), (3.0, false, Some(false)));
        assert_eq!(want(510.0, 400.0, 7.0, 10.0, true), (0.0, false, Some(false)));    // there is no seventh wedge
    }

    /// As `want`, on a ring that draws the wedges of `shapes` (a bit a wedge).
    fn on_drawn(mx: f64, t: f64, armed: bool, clear: Option<bool>, shapes: f64) -> (f64, bool, Option<bool>) {
        let mut r = RingState { clear, ..base(armed) };
        let got = ring_want_on(&page(), &mut r, mx, 400.0, 0.0, t, 0.0, shapes, false);
        (got, r.armed, r.clear)
    }

    /// Wedge 2 lies to the right of the middle. A failure means a ring opened under a resting cursor picks what it happens to be on.
    #[test]
    fn on_a_drawn_wedge_the_cursor_picks_it_once_it_has_been_off_one() {
        assert_eq!(on_drawn(540.0, 10.2, false, Some(true), 4.0), (2.0, false, Some(true)));      // been off one: picked, ready or not
        assert_eq!(on_drawn(540.0, 10.2, false, None, 4.0), (0.0, false, None));                  // never been off one
        assert_eq!(on_drawn(540.0, 10.2, false, Some(false), 4.0), (0.0, false, Some(false)));    // a digit took that back
        assert_eq!(on_drawn(540.0, 10.05, true, Some(true), 4.0), (0.0, false, Some(true)));      // not during the tap
        assert_eq!(on_drawn(510.0, 10.2, false, None, 4.0), (0.0, true, Some(true)));             // in the hole: off every wedge, and ready
        assert_eq!(on_drawn(900.0, 10.2, false, Some(true), 4.0), (0.0, false, Some(true)));      // past the ring's own box
        assert_eq!(on_drawn(540.0, 10.2, true, None, 8.0), (2.0, true, Some(true)));              // where nothing is drawn a flick still goes, and that was off one
    }

    fn fr(id: &str) -> FlatRow { FlatRow { id: String::from(id), targets: vec![String::from("floor")], ..FlatRow::default() } }
    fn fp(id: &str) -> FlatRow { FlatRow { free: true, targets: vec![String::from("none")], ..fr(id) } }
    fn frs(ids: &[&str]) -> Vec<FlatRow> { ids.iter().map(|id| fr(id)).collect() }
    fn with_p(mut rows: Vec<FlatRow>, p: &str) -> Vec<FlatRow> { rows.push(fp(p)); rows }

    /// A kind of target has rows that hold a place: a player learns where cover is. (rows, target, the lead row, row, 'more' wedge, what is behind it)
    #[test]
    fn the_ring_keeps_each_target_s_rows_in_their_places() {
        let worded = FlatRow { word: true, ..fr("b") };
        let cases: Vec<(&str, Vec<FlatRow>, &str, f64, [f64; 6], f64, Vec<f64>)> = vec![
            ("the floor's own places, the rest behind more", with_p(frs(&["a", "cover", "fallIn", "hold", "b"]), "silent"), "floor", 0., [-1., 0., 1., 2., 3., -1.], 5., vec![4., 5.]),
            ("a door's own places, and fall in and hold are not on its ring", frs(&["a", "cover", "openClear", "bangClear", "fallIn", "hold", "b"]), "door", 0., [-1., 0., 1., 2., 3., 6.], 0., vec![]),
            ("a target with no places fills from wedge 2", frs(&["a", "b", "c", "d", "e"]), "wall", 0., [-1., 0., 1., 2., 3., 4.], 0., vec![]),
            ("as many as there are wedges: no more", with_p(frs(&["a", "b", "c", "d"]), "p"), "wall", 0., [-1., 0., 1., 2., 3., 4.], 0., vec![]),
            ("a place is held whatever the list's order", frs(&["a", "hold", "b", "cover"]), "floor", 0., [-1., 0., 3., 2., 1., -1.], 0., vec![]),
            ("two over go behind more", frs(&["a", "b", "c", "d", "e", "f"]), "wall", 0., [-1., 0., 1., 2., 3., -1.], 5., vec![4., 5.]),
            ("behind more is a ring's worth at most, the postures left off first", with_p(frs(&["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"]), "p"), "wall", 0., [-1., 0., 1., 2., 3., -1.], 5., vec![4., 5., 6., 7., 8.]),
            ("postures take the last wedges", vec![fr("a"), fp("p1"), fp("p2"), fr("b"), fr("c")], "wall", 0., [-1., 0., 3., 4., 1., 2.], 0., vec![]),
            ("no rows, no lead: nothing", vec![], "floor", -1., [-1., -1., -1., -1., -1., -1.], 0., vec![]),
            ("the word's row takes no wedge", vec![worded, fr("c"), fr("d")], "foe", 1., [-1., 1., 2., -1., -1., -1.], 0., vec![]),
        ];
        for (what, rows, kind, lead, row, more, over) in &cases {
            let got = flat_seats_of(5.0, rows, kind, *lead);
            assert_eq!(got.row.as_slice(), row.as_slice(), "{what}");
            assert_eq!((got.more, &got.over), (*more, over), "{what}");
        }
    }

    /// Down: hold. Up with a wedge lit: give it. Up with nothing lit: the ring stays, free, until the key is hit again.
    #[test]
    fn a_ring_that_stays_latches_when_the_key_comes_up_with_nothing_lit() {
        let mut r = base(false);
        assert_eq!((ring_latch(&mut r, true, false, false), r.free), (RingLatch::Hold, None));
        assert_eq!((ring_latch(&mut r, false, false, true), r.free), (RingLatch::Give, None));
        assert_eq!((ring_latch(&mut r, false, false, false), r.free), (RingLatch::Stay, Some(true)));
        assert_eq!((ring_latch(&mut r, false, false, false), r.free), (RingLatch::Stay, Some(true)));
        assert_eq!((ring_latch(&mut r, true, true, false), r.free), (RingLatch::Give, Some(true)));
    }
}
