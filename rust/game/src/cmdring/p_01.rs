//! src/game/cmdring.ts lines 1 to 332
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// A place held `m` in from both ends of `0 … max`; a view too small to hold it leaves it alone.   (cmdring.ts:217 and 223 `cl`)
fn cl(v: f64, m_: f64, max: f64) -> f64 {
    if max > 2.0 * m_ { m::max(m_, m::min(max - m_, v)) } else { v }
}
/// A string as a condition: absent and '' are false.
fn str_truthy(s: &Option<String>) -> bool {
    s.as_deref().map_or(false, |s| !s.is_empty())
}
/// `(r.listed ?? r.targets).includes('none')`
fn lists_none(listed: &Option<Vec<String>>, targets: &[String]) -> bool {
    listed.as_deref().unwrap_or(targets).iter().any(|t| t == "none")
}

/// How much larger than CMDRING's px a ring stands in a view this size. The tick's ringOpen reads it, so it is translated.   (cmdring.ts ringZoom)
pub fn ring_zoom(g: &mut crate::game::Game, w: f64, h: f64) -> f64 {
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
/// A ring opened at the cursor, whole inside the view's edge; a cursor then out of its middle lights nothing until it has come back.   (cmdring.ts ringOpen)
pub fn ring_open(g: &mut crate::game::Game, input: InputId, w: f64, h: f64, time: f64, lit: Option<f64>) -> crate::cmdring::RingState {
    let lit = lit.unwrap_or(1.0);
    let (mouse_x, mouse_y) = (g.tab.input[input].mouse_x, g.tab.input[input].mouse_y);
    ring_open_at(&g.k.cmdring, mouse_x, mouse_y, w, h, time, lit)
}

/// ringPin on the ring itself.
fn ring_pin_on(c: &crate::knobs::CmdringK, r: &mut RingState, sx: f64, sy: f64, w: f64, h: f64) {
    r.ax = cl(sx + r.ox.unwrap_or(0.0), c.edge_x * r.z, w);
    r.ay = cl(sy + r.oy.unwrap_or(0.0), c.edge_y * r.z, h);
}
/// The ring put back over its place, whole inside the view's edge.   (cmdring.ts ringPin)
pub fn ring_pin(g: &mut crate::game::Game, r: &mut crate::cmdring::RingState, sx: f64, sy: f64, w: f64, h: f64) {
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
/// The wedge this frame asks for (0: none): a digit's own, which disarms the cursor, else the one the cursor is out toward or on.   (cmdring.ts ringWant)
pub fn ring_want(g: &mut crate::game::Game, r: &mut crate::cmdring::RingState, input: InputId, digit: f64, time: f64, lone: Option<f64>, shapes: Option<f64>) -> f64 {
    let (lone, shapes) = (lone.unwrap_or(0.0), shapes.unwrap_or(0.0));
    let (mouse_x, mouse_y) = (g.tab.input[input].mouse_x, g.tab.input[input].mouse_y);
    let wedges = g.k.cmdring.wedges;
    let pill = lone > 0.0 && crate::ringlone::on_pill(g, lone, wedges, (mouse_x - r.ax) / r.z, (mouse_y - r.ay) / r.z);
    ring_want_on(&g.k.cmdring, r, mouse_x, mouse_y, digit, time, lone, shapes, pill)
}

/// Which row is on which wedge (CMDRING.only, the page's way): the rows with digits keep them, the rest take the spare wedges, the last spare is 'more'.   (cmdring.ts ringSeats)
pub fn ring_seats(g: &mut crate::game::Game, rows: &[crate::cmdring::SeatRow], place: bool, word_key: &str, first: Option<bool>) -> crate::cmdring::RingSeats {
    ring_seats_of(g.k.cmdring.wedges, rows, place, word_key, first.unwrap_or(true))
}
/// ringSeats on the number of wedges alone.
fn ring_seats_of(wedges: f64, rows: &[SeatRow], place: bool, word_key: &str, first: bool) -> RingSeats {
    let w = wedges as usize;
    let mut row: Vec<f64> = vec![-1.0; w + 1];
    let mut rest: Vec<f64> = Vec::new();
    let mut last: Vec<f64> = Vec::new();
    for (i, r) in rows.iter().enumerate() {
        let i = i as f64;
        if r.hot_key.as_deref() == Some(word_key) {
            continue;
        }
        if place && r.free == Some(true) && !str_truthy(&r.opens) && !str_truthy(&r.hot_key) && lists_none(&r.listed, &r.targets) {
            last.push(i);
            continue;
        }
        match r.key {
            // (a digit that is no whole number finds no wedge: row[k] is undefined there, and undefined < 0 is false)
            Some(k) if k >= 1.0 && k <= wedges && k == m::floor(k) && row[k as usize] < 0.0 => row[k as usize] = i,
            _ => rest.push(i),
        }
    }
    let mut spare: Vec<usize> = Vec::new();
    for d in (if first { 2 } else { 1 })..=w {
        if row[d] < 0.0 {
            spare.push(d);
        }
    }
    if rest.len() <= spare.len() {
        for (n, i) in rest.iter().chain(last.iter()).take(spare.len()).enumerate() {
            row[spare[n]] = *i;
        }
        return RingSeats { row, more: 0.0, over: Vec::new() };
    }
    let more = spare.pop().unwrap_or(w);
    let mut over = rest;
    if row[more] >= 0.0 {
        over.insert(0, row[more]);
        row[more] = -1.0;
    }
    for &d in &spare {
        row[d] = over.remove(0);
    }
    over.truncate(w);
    RingSeats { row, more: more as f64, over }
}

/// Which row is on which wedge of the flat tree: FLAT_TOP's rows on their wedges, the rest on what is left or behind one 'more'.   (cmdring.ts flatSeats)
pub fn flat_seats(g: &mut crate::game::Game, rows: &[crate::cmdring::FlatRow], kind: &str, word_key: &str, lead: f64) -> crate::cmdring::FlatSeats {
    flat_seats_of(g.k.cmdring.wedges, rows, kind, word_key, lead)
}
/// flatSeats on the number of wedges alone.
fn flat_seats_of(wedges: f64, rows: &[FlatRow], kind: &str, word_key: &str, lead: f64) -> FlatSeats {
    let w = wedges as usize;
    let mut row: Vec<f64> = vec![-1.0; w + 1];
    let (flat_top, flat_left) = (crate::cmdring::flat_top(), crate::cmdring::flat_left());
    let top = flat_top.get(String::from(kind));
    let left = flat_left.get(String::from(kind));
    let mut rest: Vec<f64> = Vec::new();
    let mut last: Vec<f64> = Vec::new();
    if lead >= 0.0 {
        row[1] = lead;
    }
    for (i, r) in rows.iter().enumerate() {
        let i = i as f64;
        if str_truthy(&r.opens) || r.hot_key.as_deref() == Some(word_key) || left.map_or(false, |l| l.contains(&r.id)) {
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
        if r.free == Some(true) && !str_truthy(&r.hot_key) && lists_none(&r.listed, &r.targets) {
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

/// How long the key is kept down before its ring goes up (CMDRING.stay, the page's way: a ring that stays).   (cmdring.ts ringHoldSecs)
pub fn ring_hold_secs(g: &mut crate::game::Game) -> f64 {
    g.k.cmdring.stay_hold_secs
}

/// One frame of the latch of a ring that stays.   (cmdring.ts ringLatch)
pub fn ring_latch(r: &mut crate::cmdring::RingState, key_down: bool, key_hit: bool, pointed: bool) -> crate::cmdring::RingLatch {
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

/// The wedge a direction points at: 1 at the top, clockwise, `n` of them.   (cmdring.ts wedgeAt)
pub fn wedge_at(dx: f64, dy: f64, n: f64) -> f64 {
    let k = m::round(m::atan2(dx, -dy) / (2.0 * m::PI / n));
    js::rem(js::rem(k, n) + n, n) + 1.0
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The page's own row of knobs: these tests reach no game.
    fn page() -> crate::knobs::CmdringK { crate::knobs::page_cmdring() }

    // Every expected value below is what src/game/cmdring.ts printed under bun, with EXACT on.
    #[test]
    fn wedge_at_is_the_typescript_bits() {
        for &(dx, dy, n, bits) in &[
            (0.0, -30.0, 5.0, 0x3ff0000000000000u64),
            (30.0, 0.0, 5.0, 0x4000000000000000),
            (0.0, 30.0, 5.0, 0x4010000000000000),
            (-30.0, 0.0, 5.0, 0x4014000000000000),
            (-30.0, -1.0, 5.0, 0x4014000000000000),
            (10.0, -29.5, 5.0, 0x3ff0000000000000),
            (-0.0, 0.0, 5.0, 0x4010000000000000),
            (0.0, 0.0, 5.0, 0x4010000000000000),
            (-12.5, 7.25, 5.0, 0x4010000000000000),
            (28.53, -9.27, 5.0, 0x4000000000000000),
            (17.63, 24.27, 5.0, 0x4008000000000000),
            (-17.63, 24.27, 5.0, 0x4010000000000000),
            (-28.53, -9.28, 5.0, 0x4014000000000000),
            (3.0, 4.0, 8.0, 0x4010000000000000),
        ] {
            assert_eq!(wedge_at(dx, dy, n).to_bits(), bits, "{dx} {dy} {n}");
        }
    }

    #[test]
    fn zoom_open_and_pin_are_the_typescript_bits() {
        {
            let c = page();
            for &(w, h, bits) in &[
                (1920.0, 1080.0, 0x3ff0000000000000u64),
                (3840.0, 2160.0, 0x4000000000000000),
                (1280.0, 720.0, 0x3ff0000000000000),
                (2560.0, 1080.0, 0x3ff0000000000000),
                (3000.0, 2000.0, 0x3ff9000000000000),
            ] {
                assert_eq!(zoom_of(&c, w, h).to_bits(), bits);
            }
            for &(mx, my, w, h, t, lit, ax, ay, z, armed) in &[
                (960.0, 540.0, 1920.0, 1080.0, 3.5, 1.0, 0x408e000000000000u64, 0x4080e00000000000u64, 0x3ff0000000000000u64, true),
                (10.0, 10.0, 1920.0, 1080.0, 1.0, 2.0, 0x4062c00000000000, 0x4059000000000000, 0x3ff0000000000000, false),
                (3800.0, 20.0, 3840.0, 2160.0, 0.0, 1.0, 0x40aba80000000000, 0x4069000000000000, 0x4000000000000000, false),
                (50.0, 50.0, 0.0, 0.0, 0.0, 1.0, 0x4049000000000000, 0x4049000000000000, 0x3ff0000000000000, true),
                (100.0, 60.0, 200.0, 100.0, 0.0, 1.0, 0x4059000000000000, 0x404e000000000000, 0x3ff0000000000000, true),
                (1910.5, 1075.25, 1920.0, 1080.0, 0.1, 3.0, 0x409ba80000000000, 0x408ea00000000000, 0x3ff0000000000000, false),
            ] {
                let r = ring_open_at(&c, mx, my, w, h, t, lit);
                assert_eq!((r.ax.to_bits(), r.ay.to_bits(), r.z.to_bits(), r.armed), (ax, ay, z, armed), "{mx} {my} {w} {h}");
                assert_eq!((r.t0, r.lit), (t, lit));
            }
            for &(sx, sy, w, h, ox, oy, ax, ay) in &[
                (100.0, 50.0, 1920.0, 1080.0, 3.0, -4.0, 0x406c200000000000u64, 0x4062c00000000000u64),
                (960.0, 540.0, 1920.0, 1080.0, 0.0, 0.0, 0x408e000000000000, 0x4080e00000000000),
                (1900.0, 1070.0, 1920.0, 1080.0, 12.5, 9.0, 0x409a7c0000000000, 0x408d100000000000),
                (10.0, 10.0, 200.0, 100.0, 1.0, 1.0, 0x4026000000000000, 0x4026000000000000),
            ] {
                let mut r = base(false);
                r.ox = Some(ox);
                r.oy = Some(oy);
                ring_pin_on(&c, &mut r, sx, sy, w, h);
                assert_eq!((r.ax.to_bits(), r.ay.to_bits()), (ax, ay));
            }
        }
    }

    fn base(armed: bool) -> RingState {
        RingState { ax: 500.0, ay: 400.0, z: 1.5, t0: 10.0, lit: 1.0, armed, ..RingState::zeroed() }
    }

    #[test]
    fn want_by_direction_is_the_typescript() {
        {
            let c = page();
            // (mouse x, y, digit, time, armed before) → (answer, armed after, clear after)
            for &(mx, my, d, t, armed, want, armed2, clear) in &[
                (500.0, 400.0, 0.0, 10.0, false, 0.0, false, None),
                (540.0, 400.0, 0.0, 10.05, true, 0.0, false, None),
                (540.0, 400.0, 0.0, 10.2, false, 0.0, false, None),
                (540.0, 400.0, 0.0, 10.2, true, 2.0, true, None),
                (500.0, 300.0, 0.0, 10.2, true, 1.0, true, None),
                (500.0, 400.0, 0.0, 10.2, false, 0.0, true, None),
                (510.0, 400.0, 3.0, 10.0, true, 3.0, false, Some(false)),
                (510.0, 400.0, 7.0, 10.0, true, 0.0, false, Some(false)),
                (420.0, 470.0, 0.0, 10.149999, true, 0.0, false, None),
                (420.0, 470.0, 0.0, 10.15, true, 4.0, true, None),
            ] {
                let mut r = base(armed);
                let got: f64 = ring_want_on(&c, &mut r, mx, my, d, t, 0.0, 0.0, false);
                assert_eq!((got.to_bits(), r.armed, r.clear), ((want as f64).to_bits(), armed2, clear), "{mx} {my} {d} {t}");
            }
        }
    }

    #[derive(Clone, Default)]
    struct S { key: Option<f64>, hot: Option<&'static str>, opens: Option<&'static str>, free: bool, targets: Option<&'static str>, listed: Option<&'static str> }
    fn seat(s: &S) -> SeatRow {
        SeatRow {
            key: s.key,
            hot_key: s.hot.map(String::from),
            opens: s.opens.map(String::from),
            free: if s.free { Some(true) } else { None },
            targets: vec![String::from(s.targets.unwrap_or("floor"))],
            listed: s.listed.map(|l| vec![String::from(l)]),
        }
    }
    fn k(key: f64) -> S { S { key: Some(key), ..S::default() } }
    fn plain() -> S { S::default() }
    fn posture(key: f64) -> S { S { key: Some(key), free: true, targets: Some("none"), ..S::default() } }

    #[test]
    fn seats_are_the_typescript() {
        {
            let cases: Vec<(Vec<S>, bool, bool, [f64; 6], f64, Vec<f64>)> = vec![
                (vec![k(1.), k(2.), k(3.)], false, true, [-1., 0., 1., 2., -1., -1.], 0., vec![]),
                (vec![k(1.), S { hot: Some("F"), ..k(2.) }, plain(), plain(), plain()], false, true, [-1., 0., 2., 3., 4., -1.], 0., vec![]),
                (vec![k(1.), k(2.), k(3.), k(4.), k(5.), plain(), plain()], false, true, [-1., 0., 1., 2., 3., -1.], 5., vec![4., 5., 6.]),
                (vec![k(1.), k(2.), k(3.), plain(), plain(), plain(), plain()], false, true, [-1., 0., 1., 2., 3., -1.], 5., vec![4., 5., 6.]),
                (vec![plain(), plain(), plain(), plain(), plain(), plain()], false, false, [-1., 0., 1., 2., 3., -1.], 5., vec![4., 5.]),
                (vec![posture(2.), k(3.), plain(), plain()], true, true, [-1., -1., 2., 1., 3., 0.], 0., vec![]),
                (vec![S { targets: Some("floor"), listed: Some("none"), ..posture(2.) }, k(3.), plain()], true, true, [-1., -1., 2., 1., 0., -1.], 0., vec![]),
                (vec![S { hot: Some("G"), ..posture(2.) }, k(3.)], true, true, [-1., -1., 0., 1., -1., -1.], 0., vec![]),
                (vec![S { opens: Some("x"), ..posture(2.) }, k(3.)], true, true, [-1., -1., 0., 1., -1., -1.], 0., vec![]),
                (vec![posture(2.), k(3.)], false, true, [-1., -1., 0., 1., -1., -1.], 0., vec![]),
                (vec![k(2.), k(2.), k(9.), k(0.), k(1.5)], false, true, [-1., -1., 0., 1., 2., -1.], 5., vec![3., 4.]),
                (vec![k(1.), k(2.), k(3.), k(4.), k(5.), k(5.), posture(0.)], true, false, [-1., 0., 1., 2., 3., -1.], 5., vec![4., 5.]),
            ];
            for (n, (rows, place, first, row, more, over)) in cases.iter().enumerate() {
                let rows: Vec<SeatRow> = rows.iter().map(seat).collect();
                let got = ring_seats_of(5.0, &rows, *place, "F", *first);
                assert_eq!(got.row.as_slice(), row.as_slice(), "case {n}");
                assert_eq!((got.more, &got.over), (*more, over), "case {n}");
            }
        }
    }

    fn fr(id: &str) -> FlatRow { FlatRow { id: String::from(id), targets: vec![String::from("floor")], ..FlatRow::zeroed() } }
    fn fp(id: &str) -> FlatRow { FlatRow { free: Some(true), targets: vec![String::from("none")], ..fr(id) } }
    fn frs(ids: &[&str]) -> Vec<FlatRow> { ids.iter().map(|id| fr(id)).collect() }
    fn with_p(mut rows: Vec<FlatRow>, p: &str) -> Vec<FlatRow> { rows.push(fp(p)); rows }

    #[test]
    fn flat_seats_are_the_typescript() {
        {
            let opened = FlatRow { opens: Some(String::from("p")), ..fr("a") };
            let worded = FlatRow { hot_key: Some(String::from("F")), ..fr("b") };
            let cases: Vec<(Vec<FlatRow>, &str, f64, [f64; 6], f64, Vec<f64>)> = vec![
                (with_p(frs(&["a", "cover", "fallIn", "hold", "b"]), "silent"), "floor", 0., [-1., 0., 1., 2., 3., -1.], 5., vec![4., 5.]),
                (frs(&["a", "cover", "openClear", "bangClear", "fallIn", "hold", "b"]), "door", 0., [-1., 0., 1., 2., 3., 6.], 0., vec![]),
                (frs(&["a", "b"]), "wall", 0., [-1., 0., 1., -1., -1., -1.], 0., vec![]),
                (frs(&["a", "b", "c", "d", "e"]), "wall", 0., [-1., 0., 1., 2., 3., 4.], 0., vec![]),
                (with_p(frs(&["a", "b", "c", "d", "e", "f"]), "p"), "wall", 0., [-1., 0., 1., 2., 3., -1.], 5., vec![4., 5., 6.]),
                (with_p(frs(&["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"]), "p"), "wall", 0., [-1., 0., 1., 2., 3., -1.], 5., vec![4., 5., 6., 7., 8.]),
                (with_p(frs(&["a", "b", "c", "d", "e", "f", "g", "h", "i"]), "p"), "wall", 0., [-1., 0., 1., 2., 3., -1.], 5., vec![4., 5., 6., 7., 8.]),
                (with_p(frs(&["fallIn", "hold"]), "silent"), "none", 0., [-1., 0., -1., 0., 1., 2.], 0., vec![]),
                (vec![], "floor", -1., [-1., -1., -1., -1., -1., -1.], 0., vec![]),
                (vec![opened, worded, fr("c"), fr("d")], "foe", 2., [-1., 2., 3., -1., -1., -1.], 0., vec![]),
                (vec![fr("a"), fp("p1"), fp("p2"), fr("b"), fr("c")], "wall", 0., [-1., 0., 3., 4., 1., 2.], 0., vec![]),
                (with_p(frs(&["a", "b", "c", "d"]), "p"), "wall", 0., [-1., 0., 1., 2., 3., 4.], 0., vec![]),
                (with_p(frs(&["a", "b", "c", "d", "e"]), "p"), "wall", 0., [-1., 0., 1., 2., 3., -1.], 5., vec![4., 5.]),
                (frs(&["a", "b", "c", "d", "e", "f"]), "wall", 0., [-1., 0., 1., 2., 3., -1.], 5., vec![4., 5.]),
                (frs(&["cover", "b", "c"]), "floor", 0., [-1., 0., 0., 1., 2., -1.], 0., vec![]),
            ];
            for (n, (rows, kind, lead, row, more, over)) in cases.iter().enumerate() {
                let got = flat_seats_of(5.0, rows, kind, "F", *lead);
                assert_eq!(got.row.as_slice(), row.as_slice(), "case {n}");
                assert_eq!((got.more, &got.over), (*more, over), "case {n}");
            }
        }
    }

    #[test]
    fn want_on_drawn_wedges_is_the_typescript() {
        let c = page();
        // (mouse x, y, digit, time, armed before, clear before, shapes) → (answer, armed after, clear after)
        for &(mx, my, d, t, armed, clear0, shapes, want, armed2, clear) in &[
            (540.0, 400.0, 0.0, 10.2, true, Some(true), 4.0, 2.0, true, Some(true)),
            (540.0, 400.0, 0.0, 10.2, true, None, 4.0, 2.0, true, None),
            (540.0, 400.0, 0.0, 10.2, false, None, 4.0, 0.0, false, None),
            (540.0, 400.0, 0.0, 10.2, false, Some(true), 4.0, 2.0, false, Some(true)),
            (540.0, 400.0, 0.0, 10.2, true, None, 8.0, 2.0, true, Some(true)),
            (510.0, 400.0, 0.0, 10.2, false, Some(true), 4.0, 0.0, true, Some(true)),
            (500.0, 700.0, 0.0, 10.2, false, Some(true), 16.0, 0.0, false, Some(true)),
            (900.0, 400.0, 0.0, 10.2, false, Some(true), 4.0, 0.0, false, Some(true)),
            (540.0, 400.0, 0.0, 10.05, true, Some(true), 4.0, 0.0, false, Some(true)),
            (540.0, 400.0, 3.0, 10.2, true, Some(true), 4.0, 3.0, false, Some(false)),
            (540.0, 400.0, 0.0, 10.2, false, Some(false), 4.0, 0.0, false, Some(false)),
            (420.0, 470.0, 0.0, 10.2, false, Some(true), -1.0, 4.0, false, Some(true)),
            (540.0, 400.0, 0.0, 10.2, false, Some(true), 4294967300.0, 2.0, false, Some(true)),
        ] {
            let mut r = RingState { clear: clear0, ..base(armed) };
            let got: f64 = ring_want_on(&c, &mut r, mx, my, d, t, 0.0, shapes, false);
            assert_eq!((got.to_bits(), r.armed, r.clear), ((want as f64).to_bits(), armed2, clear), "{mx} {my} {d} {t} {shapes}");
        }
    }

    #[test]
    fn latch_is_the_typescript() {
        let mut r = base(false);
        assert_eq!((ring_latch(&mut r, true, false, false), r.free), (RingLatch::Hold, None));
        assert_eq!((ring_latch(&mut r, false, false, true), r.free), (RingLatch::Give, None));
        assert_eq!((ring_latch(&mut r, false, false, false), r.free), (RingLatch::Stay, Some(true)));
        assert_eq!((ring_latch(&mut r, false, false, false), r.free), (RingLatch::Stay, Some(true)));
        assert_eq!((ring_latch(&mut r, true, true, false), r.free), (RingLatch::Give, Some(true)));
    }
}
