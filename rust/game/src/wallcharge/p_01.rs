//! src/game/wallcharge.ts lines 1 to 420
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, format, string::String, vec, vec::Vec};

/// How near a point passes a line on the floor, a to b.   (wallcharge.ts nearSeg)
pub fn near_seg(q: V3, a: V3, b: V3) -> f64 {
    let dx = b[0] - a[0];
    let dz = b[2] - a[2];
    let l = dx * dx + dz * dz;
    let k = if l > 1e-9 { m::max(0.0, m::min(1.0, ((q[0] - a[0]) * dx + (q[2] - a[2]) * dz) / l)) } else { 0.0 };
    m::hyp2(q[0] - a[0] - dx * k, q[2] - a[2] - dz * k)
}

/// The wall's thickness.   (wallcharge.ts thick)
pub fn thick(g: &mut crate::game::Game, w: &crate::wallcharge::WallSpot) -> f64 {
    let h = g.col.boxes[w.box_].h;
    2.0 * (if w.along_x { h[2] } else { h[0] })
}

/// Where the charge sits: on the near face, at the charge's height.   (wallcharge.ts chargeAt)
pub fn charge_at(g: &mut crate::game::Game, w: &crate::wallcharge::WallSpot) -> V3 {
    let height = g.k.wallcharge.height;
    wall_pt(g, w, 0.0, FACE, Some(height))
}

/// A point by the wall: `du` along it from the charge, `off` out from the charge's face (the far face is at −thick), at height y.   (wallcharge.ts wallPt)
pub fn wall_pt(g: &mut crate::game::Game, w: &crate::wallcharge::WallSpot, du: f64, off: f64, y: Option<f64>) -> V3 {
    let y = y.unwrap_or(0.0);
    let b = &g.col.boxes[w.box_];
    if w.along_x { [w.u + du, y, b.c[2] + w.side * (b.h[2] + off)] } else { [b.c[0] + w.side * (b.h[0] + off), y, w.u + du] }
}

/// The wall's unit normal on the charge's side.   (wallcharge.ts wallN)
pub fn wall_n(w: &crate::wallcharge::WallSpot) -> V3 {
    if w.along_x { [0.0, 0.0, w.side] } else { [w.side, 0.0, 0.0] }
}

/// Metres out from the charge's face (negative: in the wall, or past it).   (wallcharge.ts wallOff)
pub fn wall_off(g: &mut crate::game::Game, w: &crate::wallcharge::WallSpot, p: V3) -> f64 {
    let b = &g.col.boxes[w.box_];
    if w.along_x { (p[2] - b.c[2]) * w.side - b.h[2] } else { (p[0] - b.c[0]) * w.side - b.h[0] }
}

/// Metres along the wall from the charge.   (wallcharge.ts wallAlong)
pub fn wall_along(w: &crate::wallcharge::WallSpot, p: V3) -> f64 {
    (if w.along_x { p[0] } else { p[2] }) - w.u
}

/// Two spots on the one wall, near enough to be one place.   (wallcharge.ts sameWall)
pub fn same_wall(a: &crate::wallcharge::WallSpot, b: &crate::wallcharge::WallSpot) -> bool {
    a.index == b.index && m::abs(a.u - b.u) < 1.6
}

/// Is the wall charge live in this game right now: the walls' switch, outside a show, chaos above 0.   (wallcharge.ts wallChargeOn)
pub fn wall_charge_on(g: &mut crate::game::Game) -> bool {
    crate::wallbreak::wall_break_on(g) && crate::destruction::chaos_of(g) > 0.0
}

/// The faces of a wall the open sky is over: 0 a wall inside, 1 a building's outer wall, 2 a yard wall. Kept per static.   (wallcharge.ts skyFaces)
pub fn sky_faces(g: &mut crate::game::Game, index: f64) -> f64 {
    if let Some(&n) = g.m.wallcharge.sky_memo.get_or_insert_with(JsMap::new).get(crate::Num(index)) {
        return n;
    }
    let b = BoxId(index as u32);
    let (c, h) = (g.col.boxes[b].c, g.col.boxes[b].h);
    let along_x = h[0] >= h[2];
    let hu = if along_x { h[0] } else { h[2] };
    let ht = if along_x { h[2] } else { h[0] };
    let (sky_off, sky_along) = (sky_off(), sky_along());
    let mut n = 0.0;
    for sg in [1.0, -1.0] {
        let mut sky = 0;
        for &off in &sky_off {
            for &t in &sky_along {
                let d = sg * (ht + off);
                let p: V3 = if along_x { [c[0] + t * hu, 1.5, c[2] + d] } else { [c[0] + d, 1.5, c[2] + t * hu] };
                if !crate::scene_collision::solid_blocked(g, p, [p[0], SKY_TOP, p[2]], Some(0.02)) {
                    sky += 1;
                }
            }
        }
        if sky * 2 > sky_off.len() * sky_along.len() {
            n += 1.0;
        }
    }
    g.m.wallcharge.sky_memo.get_or_insert_with(JsMap::new).set(crate::Num(index), n);
    n
}

/// `fn_` asked aside: what it adds to the memo of sky faces is taken out again, so a wall is in it only once the simulation has asked.   (wallcharge.ts skyAside)
pub fn sky_aside<T: Clone + core::fmt::Debug>(g: &mut crate::game::Game, fn_: &mut dyn FnMut(&mut crate::game::Game) -> T) -> T {
    let had = g.m.wallcharge.sky_memo.as_ref().map(|memo| memo.len());
    let out = fn_(g);
    match had {
        None => g.m.wallcharge.sky_memo = None,
        Some(n) => {
            if let Some(memo) = g.m.wallcharge.sky_memo.as_mut() {
                if memo.len() > n {
                    for (i, k) in memo.keys().into_iter().enumerate() {
                        if i >= n {
                            memo.delete(k);
                        }
                    }
                }
            }
        }
    }
    out
}

/// `doesOf(b)` of a box: the row of its word, else the default row. The same answer as does_of of an object holding that word.   (materials.ts doesOf)
fn does_of_box(g: &mut Game, b: BoxId) -> crate::scene_materials::Does {
    let word = g.col.boxes[b].does.map(|d| d.as_str());
    crate::scene_materials::does_of_word(g, word)
}

/// Is structure static `index` an outer wall a charge cuts: not steel, thin enough, long enough, adoptable, the sky over one face or both.   (wallcharge.ts outerWall)
pub fn outer_wall(g: &mut crate::game::Game, index: f64) -> bool {
    let b = BoxId(index as u32);
    let h = g.col.boxes[b].h;
    let thin = 2.0 * m::min(h[0], h[2]);
    let long = 2.0 * m::max(h[0], h[2]);
    if thin > g.k.wallcharge.outer_thick
        || long < g.k.wallcharge.outer_min_len
        || g.col.boxes[b].metal == Some(true)
        || does_of_box(g, b).surf == crate::scene_materials::Surf::Metal
        || !crate::wallbreak::wall_break_adoptable(g, index)
    {
        return false;
    }
    let sky = sky_faces(g, index);
    sky == 1.0 || sky == 2.0
}

/// Is the charge's own side indoors: a roof over the floor at its foot, by nine upward rays off the charge's face.   (wallcharge.ts chargeIndoors)
pub fn charge_indoors(g: &mut crate::game::Game, w: &crate::wallcharge::WallSpot) -> bool {
    let sky_off = sky_off();
    let mut roof = 0;
    for &off in &sky_off {
        for du in [-1.0, 0.0, 1.0] {
            let p = wall_pt(g, w, du, off, Some(1.5));
            if crate::scene_collision::solid_blocked(g, p, [p[0], SKY_TOP, p[2]], Some(0.02)) {
                roof += 1;
            }
        }
    }
    roof * 2 > sky_off.len() * 3
}

/// The safety radius at a place: how far the squad stands off, and how far the blast reaches on its own side.   (wallcharge.ts safeRadius)
pub fn safe_radius(g: &mut crate::game::Game, w: &crate::wallcharge::WallSpot) -> crate::wallcharge::WallChargeAwayMarksValueR {
    let indoors = charge_indoors(g, w);
    let i = if indoors { 1 } else { 0 };
    crate::wallcharge::WallChargeAwayMarksValueR { standoff: g.k.wallcharge.safe_standoff[i], reach: g.k.wallcharge.safe_reach[i], indoors }
}

/// The room a hole in `w` at `du` opens into (0: none): the first room a point beyond the far face stands in, a pace to either hand.   (wallcharge.ts roomBeyond)
pub fn room_beyond(g: &mut crate::game::Game, w: &crate::wallcharge::WallSpot, du: f64, t: f64) -> f64 {
    let mut d = 0.45;
    while d <= 1.95 {
        for s in [0.0, 0.5, -0.5] {
            let at = wall_pt(g, w, du + s, -(t + d), None);
            let r = crate::cleared::cleared_rooms_room_at(g, at);
            if js::truthy(r) {
                return r;
            }
        }
        d += 0.25;
    }
    0.0
}

/// The wall at a point: the full-height unrotated wall whose footprint the point is on, a partition or structure with the word for what it is.   (wallcharge.ts wallAt)
pub fn wall_at(g: &mut crate::game::Game, at: V3, from: Option<V3>) -> Option<crate::wallcharge::WallSpot> {
    let from = match from {
        Some(f) => f,
        None => {
            let c = g.pl[g.player].ch;
            g.ch[c].pos
        }
    };
    let n = m::min(g.destr.walls.authored, g.col.boxes.len() as f64) as usize;
    let f = crate::scene_boxes::box_flag();
    let skip = js::to_i32(f.glass) | js::to_i32(f.no_primary) | js::to_i32(f.no_shadow) | js::to_i32(f.dynamic);
    let mut best: Option<crate::wallcharge::WallSpot> = None;
    let mut bd = g.k.wallcharge.pick_m + 1e-6;
    for i in 0..n {
        let index = i as f64;
        let b = BoxId(i as u32);
        let (c, h) = (g.col.boxes[b].c, g.col.boxes[b].h);
        let dx = m::max_n(&[c[0] - h[0] - at[0], 0.0, at[0] - c[0] - h[0]]);
        if dx > bd {
            continue;
        }
        let dz = m::max_n(&[c[2] - h[2] - at[2], 0.0, at[2] - c[2] - h[2]]);
        if dz > bd {
            continue;
        }
        let bb = crate::wallbreak::wall_break_breakable_at(g, index);
        let passed = match &bb {
            Some(x) => x.kind != crate::wallbreak::BKind::Wall,
            None => (js::to_i32(g.col.boxes[b].flags) & skip) != 0,
        };
        if passed {
            continue; // (a partition already holed is hidden: its pieces stand in for it, and it is still the wall)
        }
        if g.col.boxes[b].rot.is_some() || m::abs(m::sin(g.col.boxes[b].yaw)) > 1e-6 {
            continue;
        }
        let sx = h[0] * 2.0;
        let sz = h[2] * 2.0;
        let long = m::max(sx, sz);
        let thin = m::min(sx, sz);
        let mut why = String::new();
        let part = match &bb {
            Some(x) => !x.hard.unwrap_or(false),
            None => false,
        };
        if !part {
            // structure: full height off the floor, long enough to be a wall, in a wall's stuff; said so in words
            if c[1] - h[1] > 0.1 || c[1] + h[1] < g.k.walls.wall_min_top || long < 0.8 || thin > 1.2 {
                continue;
            }
            let s = does_of_box(g, b).surf;
            if s != crate::scene_materials::Surf::Drywall && s != crate::scene_materials::Surf::Concrete && s != crate::scene_materials::Surf::Metal {
                continue;
            }
            why = String::from(if s == crate::scene_materials::Surf::Metal || g.col.boxes[b].metal == Some(true) {
                "steel. a charge won't cut it."
            } else if thin > g.k.walls.wall_max_thick || s == crate::scene_materials::Surf::Concrete {
                "concrete. a charge won't cut it."
            } else {
                "outer wall. a charge won't cut it."
            });
        }
        let d = m::hyp2(dx, dz) - (if bb.is_some() { 1e-3 } else { 0.0 });
        if d >= bd {
            continue;
        }
        // (an outer wall takes a charge; one already holed is the engine's by now, and still it)
        let outer = !part && (if bb.is_some() { true } else { outer_wall(g, index) });
        // (with outer walls taking a charge, 'outer wall' is no reason)
        if !part && !outer && !why.starts_with("steel") && thin <= g.k.wallcharge.outer_thick {
            if long < g.k.wallcharge.outer_min_len {
                why = String::from("that wall's too short for a charge.");
            } else if why.starts_with("outer") {
                why = String::from("a charge won't cut that one.");
            }
        }
        let along_x = sx >= sz;
        let cu = if along_x { c[0] } else { c[2] };
        let half = long / 2.0;
        let end = m::min(g.k.wallcharge.end_m, half);
        let perp = if along_x { from[2] - c[2] } else { from[0] - c[0] };
        bd = d;
        best = Some(crate::wallcharge::WallSpot {
            index,
            box_: b,
            along_x,
            u: v3::clamp(if along_x { at[0] } else { at[2] }, cu - half + end, cu + half - end),
            side: if perp >= 0.0 { 1.0 } else { -1.0 },
            ok: part || outer,
            why: if part || outer { String::new() } else { why },
            outer: if outer { Some(true) } else { None },
        });
    }
    best
}

/// A floor target whose point is on a wall carries the wall (nothing when the module is not live).   (wallcharge.ts wallTarget)
pub fn wall_target(g: &mut crate::game::Game, at: V3, base: &crate::orders::OrderTarget) -> crate::orders::OrderTarget {
    let crate::orders::OrderTarget::Floor(floor) = base else { return base.clone() };
    if !wall_charge_on(g) {
        return base.clone();
    }
    match wall_at(g, at, None) {
        Some(w) => crate::orders::OrderTarget::Floor(crate::orders::OrderTargetFloor { wall: Some(w), ..floor.clone() }),
        None => base.clone(),
    }
}

/// The rows of a wall's menu: the floor's, its fifth turned to 'wall tools ▸', and the wall tools' own page.   (wallcharge.ts wallRows)
pub fn wall_rows(g: &mut crate::game::Game, t: &crate::orders::OrderTarget, page: Option<&str>, rows: &[OrdersOrderDefId]) -> Vec<OrdersOrderDefId> {
    let crate::orders::OrderTarget::Floor(floor) = t else { return rows.to_vec() };
    if floor.wall.is_none() || !wall_charge_on(g) || !g.wall_charge.seated {
        return rows.to_vec();
    }
    let direct = g.k.wallcharge.menu == crate::knobs::WallchargeKMenu::Direct;
    let (row, opener) = (g.wall_charge.row, g.wall_charge.opener);
    let page = match page {
        Some(p) => p,
        None => {
            // (the floor's own fifth row stays on the wall's menu, below, with no digit)
            let mut out = Vec::new();
            let mut fifth = Vec::new();
            for &d in rows {
                if g.tab.orders_order_def[d].key == Some(5.0) {
                    fifth.push(d);
                } else {
                    out.push(d);
                }
            }
            out.push(if direct { row } else { opener });
            for d in fifth {
                out.push(wall_charge_undigit(g, d));
            }
            return out;
        }
    };
    if page == WALL_PAGE && !direct {
        vec![row]
    } else {
        rows.to_vec()
    }
}

/// Sam's own row at the foot of a partition his cursor is on.   (wallcharge.ts wallChargeRows)
pub fn wall_charge_rows(g: &mut crate::game::Game, p: V3, busy: bool, row: &mut dyn FnMut(&mut crate::game::Game, crate::game::Interactable, crate::game::Verb, String, Option<String>, Option<crate::wallcharge::WallChargeRowsRowOver>) -> crate::game::Interactable, offer: &mut dyn FnMut(&mut crate::game::Game, crate::game::Interactable, Vec<crate::game::Interactable>)) {
    let mut sam_row = |g: &mut Game, base: crate::game::Interactable, verb: crate::game::Verb, text: String, hints: Option<String>, over: Option<crate::wallcharge::WallChargeSamRowsRowOver>| row(g, base, verb, text, hints, over.map(over_of));
    wall_charge_sam_rows(g, p, busy, &mut sam_row, offer);
}

/// `Partial<Interactable>` came out as two types, the one samRows hands and the one its caller's `row` takes: the same fields.
fn over_of(o: crate::wallcharge::WallChargeSamRowsRowOver) -> crate::wallcharge::WallChargeRowsRowOver {
    use crate::wallcharge::WallChargeRowsRowOverKind as To;
    use crate::wallcharge::WallChargeSamRowsRowOverKind as From;
    crate::wallcharge::WallChargeRowsRowOver {
        kind: o.kind.map(|k| match k {
            From::Cam => To::Cam,
            From::Guard => To::Guard,
            From::Breaker => To::Breaker,
            From::Door => To::Door,
            From::Light => To::Light,
            From::Body => To::Body,
            From::Objective => To::Objective,
            From::Held => To::Held,
            From::Wall => To::Wall,
            From::Switch => To::Switch,
            From::Charge => To::Charge,
            From::Cancel => To::Cancel,
        }),
        pos: o.pos,
        line1: o.line1,
        line2: o.line2,
        in_reach: o.in_reach,
        off: o.off,
        progress: o.progress,
        target: o.target,
        door: o.door,
        guard: o.guard,
        verb: o.verb,
        running: o.running,
        close: o.close,
        past: o.past,
        framed: o.framed,
    }
}

/// The module and its two rows; the HUD's notes are asked of it from here on. The closures are enums now: orders.ts's callers match them.   (wallcharge.ts WallCharge.constructor)
pub fn wall_charge_new(g: &mut crate::game::Game) -> crate::wallcharge::WallCharge {
    let mut wc = crate::wallcharge::WallCharge::blank();
    wc.knobs = g.k.wallcharge.clone();
    wc.opener = g.tab.orders_order_def.push(crate::orders::OrderDef {
        id: String::from(WALL_PAGE),
        label: crate::orders::OrderLabel::Text { text: String::from("wall tools") },
        targets: vec![crate::orders::TargetKind::Floor],
        listed: Some(Vec::new()),
        opens: Some(String::from(WALL_PAGE)),
        key: Some(5.0),
        issue: crate::orders::OrderIssue::Nothing,
        ..crate::orders::OrderDef::zeroed()
    });
    wc.row = g.tab.orders_order_def.push(crate::orders::OrderDef {
        id: String::from(WALL_ORDER),
        label: crate::orders::OrderLabel::WallRow,
        targets: vec![crate::orders::TargetKind::Floor],
        listed: Some(Vec::new()),
        ack: Some(vec![String::from("copy."), String::from("on it.")]),
        gone: Some(crate::orders::OrderGone::WallRow),
        can: Some(crate::orders::OrderCan::WallRow),
        valid: Some(crate::orders::OrderValid::WallRow),
        issue: crate::orders::OrderIssue::WallRow,
        ..crate::orders::OrderDef::zeroed()
    });
    g.orders.notes.push(crate::orders::OrderNoteFn::WallCharge);
    wc
}

#[cfg(test)]
mod tests {
    use super::*;

    fn spot(along_x: bool, index: f64, u: f64, side: f64) -> crate::wallcharge::WallSpot {
        crate::wallcharge::WallSpot { along_x, index, u, side, ..crate::wallcharge::WallSpot::zeroed() }
    }

    /// A blank game with one wall in it, at c with half-sizes h; the spot's box_ is that wall.
    fn game_with_wall(c: V3, h: V3) -> (alloc::boxed::Box<Game>, BoxId) {
        let mut g = alloc::boxed::Box::new(Game::blank());
        let b = g.col.boxes.push(crate::scene_boxes::Box { c, h, ..crate::scene_boxes::Box::zeroed() });
        (g, b)
    }

    /// bits from the TypeScript's own line under bun with EXACT.on
    #[test]
    fn near_seg_has_the_typescripts_bits() {
        let rows: [(V3, V3, V3, u64); 5] = [
            ([1.3, 0.0, 2.7], [0.1, 0.0, 0.2], [5.9, 0.0, -3.3], 0x40062f4726276cc9),
            ([-7.25, 1.0, 3.125], [0.1, 0.0, 0.2], [0.1, 0.0, 0.2], 0x401fa47d26daf9af),
            ([9.1, 0.0, 9.2], [0.0, 0.0, 0.0], [1.1, 0.0, 0.7], 0x40275861532b8544),
            ([-4.4, 0.0, 0.3], [-3.3, 0.0, 1.7], [2.2, 0.0, -6.6], 0x3ffb0b672582b9e9),
            ([0.2, 0.0, 0.4], [0.0, 0.0, 0.0], [0.4, 0.0, 0.8], 0x0000000000000000),
        ];
        for (q, a, b, bits) in rows {
            assert_eq!(near_seg(q, a, b).to_bits(), bits);
        }
    }

    #[test]
    fn same_wall_is_one_index_within_a_stretch() {
        assert!(same_wall(&spot(true, 4.0, 1.0, 1.0), &spot(true, 4.0, 2.5, -1.0)));
        assert!(!same_wall(&spot(true, 4.0, 1.0, 1.0), &spot(true, 4.0, 2.6, 1.0)));
        assert!(!same_wall(&spot(true, 4.0, 1.0, 1.0), &spot(true, 5.0, 1.0, 1.0)));
    }

    #[test]
    fn along_and_normal_follow_the_walls_run() {
        assert_eq!(wall_along(&spot(true, 0.0, 1.5, 1.0), [4.0, 0.0, 9.0]), 2.5);
        assert_eq!(wall_along(&spot(false, 0.0, 1.5, 1.0), [4.0, 0.0, 9.0]), 7.5);
        assert_eq!(wall_n(&spot(true, 0.0, 0.0, -1.0)), [0.0, 0.0, -1.0]);
        assert_eq!(wall_n(&spot(false, 0.0, 0.0, -1.0)), [-1.0, 0.0, 0.0]);
    }

    /// A point set off the wall reads back the same offset and place along it; the far face is at −thick.
    #[test]
    fn a_wall_point_round_trips() {
        let (mut g, b) = game_with_wall([10.0, 1.5, -4.0], [3.0, 1.5, 0.125]);
        let w = crate::wallcharge::WallSpot { box_: b, ..spot(true, 0.0, 11.0, -1.0) };
        assert_eq!(thick(&mut g, &w), 0.25);
        let p = wall_pt(&mut g, &w, 0.5, 1.25, Some(1.0));
        assert_eq!(p, [11.5, 1.0, -5.375]);
        assert_eq!(wall_off(&mut g, &w, p), 1.25);
        assert_eq!(wall_along(&w, p), 0.5);
        let far = wall_pt(&mut g, &w, 0.0, -0.25, None);
        assert_eq!(far, [11.0, 0.0, -3.875]);

        let (mut g, b) = game_with_wall([2.0, 1.5, 6.0], [0.125, 1.5, 3.0]);
        let w = crate::wallcharge::WallSpot { box_: b, ..spot(false, 0.0, 7.0, 1.0) };
        assert_eq!(thick(&mut g, &w), 0.25);
        let p = wall_pt(&mut g, &w, -1.0, 0.5, None);
        assert_eq!(p, [2.625, 0.0, 6.0]);
        assert_eq!(wall_off(&mut g, &w, p), 0.5);
    }

    /// What is asked aside leaves the memo as it was; a memo that did not exist does not exist after.
    #[test]
    fn sky_aside_takes_out_what_it_added() {
        let mut g = alloc::boxed::Box::new(Game::blank());
        let got = sky_aside(&mut g, &mut |g: &mut Game| {
            g.m.wallcharge.sky_memo.get_or_insert_with(JsMap::new).set(crate::Num(1.0), 1.0);
            7
        });
        assert_eq!(got, 7);
        assert!(g.m.wallcharge.sky_memo.is_none());

        g.m.wallcharge.sky_memo.get_or_insert_with(JsMap::new).set(crate::Num(2.0), 2.0);
        sky_aside(&mut g, &mut |g: &mut Game| {
            let memo = g.m.wallcharge.sky_memo.as_mut().unwrap();
            memo.set(crate::Num(3.0), 0.0);
            memo.set(crate::Num(4.0), 1.0);
        });
        let memo = g.m.wallcharge.sky_memo.as_ref().unwrap();
        assert_eq!(memo.keys(), vec![crate::Num(2.0)]);
    }
}
