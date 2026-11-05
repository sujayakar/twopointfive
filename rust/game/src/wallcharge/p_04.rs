//! src/game/wallcharge.ts lines 1139 to 1234
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// holeAt's `open`: no solid thing on the rays through the wall at a man's shin and chest
fn hole_open(g: &mut Game, w: &WallSpot, t: f64, du: f64) -> bool {
    let (a, b) = (wall_pt(g, w, du, FACE, Some(0.35)), wall_pt(g, w, du, -(t + FACE), Some(0.35)));
    if crate::scene_collision::solid_blocked(g, a, b, None) { return false; }
    let (a, b) = (wall_pt(g, w, du, FACE, Some(1.3)), wall_pt(g, w, du, -(t + FACE), Some(1.3)));
    !crate::scene_collision::solid_blocked(g, a, b, None)
}
/// holeAt's `through`: floor at both mouths
fn hole_through(g: &mut Game, w: &WallSpot, t: f64, du: f64) -> bool {
    let mouth_in = g.k.wallcharge.mouth_in;
    let far = xz(wall_pt(g, w, du, -(t + mouth_in), None));
    if crate::scene_collision::nav_grid_is_blocked(g, far[0], far[1]) { return false; }
    let near = xz(wall_pt(g, w, du, 0.35, None));
    !crate::scene_collision::nav_grid_is_blocked(g, near[0], near[1])
}
/// holeAt's `shut`: an open run ends at `to`; if a man fits, the way through nearest its middle
fn hole_shut(g: &mut Game, w: &WallSpot, t: f64, step: f64, run: &mut f64, best: &mut Option<WallChargeHoleAt>, to: f64) {
    if run.is_nan() { return; }
    let a = *run - step / 2.0;
    let b = to - step / 2.0;
    let mid = (a + b) / 2.0;
    *run = f64::NAN;
    if b - a < 0.7 { return; }
    let mut k = 0.0;
    while mid - k >= a + 0.35 - 1e-6 {
        let (dus, n) = if js::truthy(k) { ([mid - k, mid + k], 2) } else { ([mid, mid], 1) };
        for &du in &dus[..n] {
            if hole_through(g, w, t, du) {
                let h = WallChargeHoleAt { du, half: m::min(du - a, b - du) };
                let better = match best { None => true, Some(was) => m::abs(h.du) < m::abs(was.du) };
                if better { *best = Some(h); }
                return;
            }
        }
        k += step;
    }
}

/// The gap a man fits through nearest the charge, read off the wall as it stands (wallcharge.ts WallCharge.holeAt)
pub fn wall_charge_hole_at(g: &mut crate::game::Game, w: &crate::wallcharge::WallSpot) -> Option<crate::wallcharge::WallChargeHoleAt> {
    let t = thick(g, w);
    let (step, r) = (0.125, 2.5);
    let mut best: Option<WallChargeHoleAt> = None;
    let mut run = f64::NAN;
    let (bc, bh) = (g.col.boxes[w.box_].c, g.col.boxes[w.box_].h);
    let c = if w.along_x { bc[0] } else { bc[2] };
    let l = if w.along_x { bh[0] } else { bh[2] };
    let lo = m::max(-r, c - l - w.u);
    let hi = m::min(r, c + l - w.u);
    let mut du = lo + step / 2.0;
    while du <= hi + 1e-6 {
        if hole_open(g, w, t, du) {
            if run.is_nan() { run = du; }
        } else {
            hole_shut(g, w, t, step, &mut run, &mut best, du);
        }
        du += step;
    }
    hole_shut(g, w, t, step, &mut run, &mut best, du);
    best
}

/// WALLSHOT: the shock with no flash (wallcharge.ts WallCharge.shock)
pub fn wall_charge_shock(g: &mut crate::game::Game, gd: GuardId, secs: f64, at: V3) {
    let now = g.time;
    g.gd[gd].stagger_until = m::max(g.gd[gd].stagger_until, now + secs);
    g.gd[gd].react_t = m::max(g.gd[gd].react_t, secs);
    let fa = g.gd[gd].weapon;
    g.fa[fa].burst = None;
    g.gd[gd].awareness = m::max(g.gd[gd].awareness, 0.6);
    if g.gd[gd].state != crate::game::GuardState::Alert { g.gd[gd].last_known = Some(v3::copy(at)); }
    g.gd[gd].path = Vec::new();
    g.gd[gd].path_goal = None;
    g.gd[gd].speed = 0.0;
    let c = g.gd[gd].ch;
    crate::character::anim_hit(g, c);
}

/// Flash-blind (wallcharge.ts WallCharge.dazzle)
pub fn wall_charge_dazzle(g: &mut crate::game::Game, gd: GuardId, secs: f64, at: V3) {
    let now = g.time;
    g.gd[gd].dazzled_until = m::max(g.gd[gd].dazzled_until, now + secs);
    g.gd[gd].react_t = m::max(g.gd[gd].react_t, secs);
    g.gd[gd].awareness = m::max(g.gd[gd].awareness, 0.6);
    if g.gd[gd].state != crate::game::GuardState::Alert { g.gd[gd].last_known = Some(v3::copy(at)); }
    if g.ai_enabled { g.gd[gd].witness.dazzled += 1.0; }
    g.gd[gd].path = Vec::new();
    g.gd[gd].path_goal = None;
    g.gd[gd].speed = 0.0;
    let c = g.gd[gd].ch;
    crate::character::anim_hit(g, c);
}

/// The charge on the wall: a dark slab and a lit tell, one-frame boxes (wallcharge.ts WallCharge.draw)
pub fn wall_charge_draw(g: &mut crate::game::Game, c_2: WallChargeId) {
    let height = g.k.wallcharge.height;
    let w = g.tab.wallcharge_set_wall_charge[c_2].wall.clone();
    let p = wall_pt(g, &w, 0.0, 0.022, Some(height));
    let yaw = if w.along_x { 0.0 } else { m::PI / 2.0 };
    let n = wall_pt(g, &w, 0.0, 1.0, None);
    let o = wall_pt(g, &w, 0.0, 0.0, None);
    let flag = crate::scene_boxes::box_flag();
    let slab = crate::scene_boxes::make_box(g, &crate::scene_boxes::MakeBoxP {
        c: p,
        h: [0.07, 0.11, 0.02],
        yaw: Some(yaw),
        albedo: Some([0.09, 0.1, 0.08]),
        flags: Some(flag.dynamic),
        ..Default::default()
    });
    g.fx.push(crate::game::GameFxItem { box_: slab, ttl: 1e-3 });
    let tell = crate::scene_boxes::make_box(g, &crate::scene_boxes::MakeBoxP {
        c: [p[0] + (n[0] - o[0]) * 0.022, p[1] + 0.07, p[2] + (n[2] - o[2]) * 0.022],
        h: [0.012, 0.012, 0.004],
        yaw: Some(yaw),
        albedo: Some([0.9, 0.12, 0.08]),
        emissive: Some([2.2, 0.2, 0.1]),
        flags: Some((js::to_i32(flag.dynamic) | js::to_i32(flag.no_shadow)) as f64),
        ..Default::default()
    });
    g.fx.push(crate::game::GameFxItem { box_: tell, ttl: 1e-3 });
}

/// Is this set charge the squad's key's to blow: one the squad set, or his own while an order of theirs is on that wall (wallcharge.ts WallCharge.keyIs)
pub fn wall_charge_key_is(g: &mut crate::game::Game, c_2: WallChargeId) -> bool {
    let c = &g.tab.wallcharge_set_wall_charge[c_2];
    c.by != crate::ownblow::SAM_BY
        || g.wall_charge.ops.iter().any(|&o| same_wall(&g.tab.wallcharge_wall_op[o].wall, &c.wall))
}

/// The nearest charge his own hands set (wallcharge.ts WallCharge.samSet)
pub fn wall_charge_sam_set(g: &mut crate::game::Game) -> Option<WallChargeId> {
    if !g.wall_charge.seated { return None; }
    wall_charge_nearest(g, Some(&mut |g: &mut Game, c: WallChargeId| g.tab.wallcharge_set_wall_charge[c].by == crate::ownblow::SAM_BY))
}

/// Where a set charge sits (wallcharge.ts WallCharge.samAt)
pub fn wall_charge_sam_at(g: &mut crate::game::Game, c_2: WallChargeId) -> V3 {
    let w = g.tab.wallcharge_set_wall_charge[c_2].wall.clone();
    let height = g.k.wallcharge.height;
    wall_pt(g, &w, 0.0, 0.0, Some(height))
}

/// Who stands inside its radius, in words (wallcharge.ts WallCharge.samWords)
pub fn wall_charge_sam_words(g: &mut crate::game::Game, c_2: WallChargeId) -> String {
    wall_charge_close_words(g, Some(c_2), None)
}

/// Would it put HIM down where he stands (wallcharge.ts WallCharge.samFatal)
pub fn wall_charge_sam_fatal(g: &mut crate::game::Game, c_2: WallChargeId) -> bool {
    if g.god_mode { return false; }
    let p = g.ch[g.pl[g.player].ch].pos;
    let w = g.tab.wallcharge_set_wall_charge[c_2].wall.clone();
    let at = wall_pt(g, &w, 0.0, 0.0, None);
    let off = wall_off(g, &w, p);
    let t = thick(g, &w);
    v3::dist_xz(p, at) < (if off < -t / 2.0 { g.k.doortools.knock_m } else { g.k.wallkill.near_m })
}

/// With the cursor on a partition and Sam at its foot, the marker and its row (wallcharge.ts WallCharge.samRows)
pub fn wall_charge_sam_rows(g: &mut crate::game::Game, p: V3, busy: bool, row: &mut dyn FnMut(&mut crate::game::Game, crate::game::Interactable, crate::game::Verb, String, Option<String>, Option<crate::wallcharge::WallChargeSamRowsRowOver>) -> crate::game::Interactable, offer: &mut dyn FnMut(&mut crate::game::Game, crate::game::Interactable, Vec<crate::game::Interactable>)) {
    g.wall_charge.offered = None;
    g.wall_charge.marker = None;
    if !wall_charge_on(g) || !g.wall_charge.seated { return; }
    let pl = g.player;
    let f = g.k.doortools.blow_key.replacen("Key", "", 1);
    let key = crate::ownblow::blow_key_word(false, &f);
    let sam_wall = g.wall_charge.sam.as_ref().map(|s| s.wall.clone());
    let wall = match sam_wall {
        Some(w) => Some(w),
        None => {
            let aim = g.aim_point;
            match wall_at(g, aim, Some(p)) {
                Some(w) => Some(wall_charge_settle(g, &w)),
                None => None,
            }
        }
    };
    let w = match wall { Some(w) if w.ok => w, _ => return };
    let c = wall_charge_charge_on(g, &w);
    let set_off = g.k.wallcharge.set_off;
    let foot = wall_pt(g, &w, 0.0, set_off, None);
    let d = v3::dist_xz(p, foot);
    if d > g.k.wallcharge.sam_marker_m || (c.is_none() && g.wall_charge.sam.is_none() && wall_charge_open_at(g, &w)) { return; }
    if c.is_none() && g.wall_charge.sam.is_none() && !wall_charge_affordable(g) { return; }
    let near = d < g.k.wallcharge.sam_reach && wall_off(g, &w, p) > 0.0 && {
        let to = wall_pt(g, &w, 0.0, 0.1, Some(0.9));
        !crate::scene_collision::solid_blocked(g, [p[0], 0.9, p[2]], to, None)
    };
    let free = !g.pl[pl].sprinting && g.pl[pl].dragging.is_none() && g.pl[pl].holding.is_none() && g.pl[pl].takedown.is_none();
    let reach = near && free;
    let line1 = if c.is_some() { "wall · charge set" } else if w.outer == Some(true) { "wall · outer wall" } else { "wall · partition" };
    let line2 = if reach { "" } else if near { "hands full" } else { "step closer" };
    let height = g.k.wallcharge.height;
    let pos = wall_pt(g, &w, 0.0, 0.05, Some(height));
    let mut it = crate::game::Interactable {
        kind: crate::game::InteractableKind::Wall,
        pos,
        line1: String::from(line1),
        line2: String::from(line2),
        in_reach: reach,
        off: false,
        ..Default::default()
    };
    if g.wall_charge.sam.is_some() {
        it.in_reach = true;
        g.wall_charge.marker = Some(it.clone());
        offer(g, it, Vec::new());
        return;
    }
    if let Some(c) = c {
        let key_is = wall_charge_key_is(g, c);
        let word = crate::ownblow::blow_key_word(key_is, &f);
        let close = wall_charge_close_words(g, Some(c), None);
        it.line2 = alloc::format!("{}  blow the charge{}", word, close);
        offer(g, it, Vec::new());
        return;
    }
    let mut verbs: Vec<crate::game::Interactable> = Vec::new();
    let going_on = reach && g.wall_charge.ops.iter().any(|&o| {
        let op = &g.tab.wallcharge_wall_op[o];
        op.phase == Phase::Place && same_wall(&op.wall, &w)
    });
    if going_on {
        it.line2 = String::from("a charge is going on it");
    } else if reach && !busy {
        let words = wall_charge_row_words(g);
        let v = row(g, it.clone(), crate::game::Verb::Charge, words, Some(alloc::format!("{}  blows it once set", key)), None);
        verbs.push(v);
        g.wall_charge.offered = Some(w);
    }
    offer(g, it, verbs);
}

/// The row performed starts it; he stands on it setSecs; a step, a sprint, full hands or another verb breaks it off (wallcharge.ts WallCharge.samTick)
pub fn wall_charge_sam_tick(g: &mut crate::game::Game) {
    let pl = g.player;
    let ch = g.pl[pl].ch;
    let p = g.ch[ch].pos;
    let height = g.k.wallcharge.height;
    let has_act = g.act.is_some();
    let mine = match &g.act {
        Some(a) => a.kind == crate::game::InteractableKind::Wall && a.verb == Some(crate::game::Verb::Charge),
        None => false,
    };
    if mine && g.wall_charge.sam.is_none() {
        if let Some(wall) = g.wall_charge.offered.clone() {
            let index = wall.index;
            g.wall_charge.sam = Some(WallChargeSam { wall, t0: g.time, at: [p[0], 0.0, p[2]] });
            wall_charge_note(g, &alloc::format!("the player sets one on wall {}", index));
            return;
        }
    }
    let s = match g.wall_charge.sam.clone() { Some(s) => s, None => return };
    if g.pl[pl].down || g.pl[pl].sprinting || g.pl[pl].dragging.is_some() || g.pl[pl].holding.is_some() || g.pl[pl].takedown.is_some()
        || v3::dist_xz(p, s.at) > g.k.wallcharge.sam_shift || (has_act && !mine) {
        wall_charge_cancel(g);
        return;
    }
    if crate::workstow::sam_stowing(g) {
        let now = g.time;
        if let Some(sam) = g.wall_charge.sam.as_mut() { sam.t0 = now; }
        return;
    }
    let set_secs = g.k.doortools.set_secs;
    let at = wall_pt(g, &s.wall, 0.0, 0.0, Some(height));
    let hand = if crate::game::player_pistol_up(g, pl) { Some(0.0) } else { None };
    let el = g.time - s.t0;
    crate::handwork::charge_hands(g, ch, at, wall_n(&s.wall), el, set_secs, hand);
    if g.time - s.t0 < set_secs { return; }
    g.wall_charge.sam = None;
    if wall_charge_open_at(g, &s.wall) || wall_charge_charge_on(g, &s.wall).is_some() { return; }
    let paid = wall_charge_take(g);
    if paid < 0.0 {
        let words = wall_charge_none_words(g);
        crate::game::game_msg(g, words.strip_suffix('.').unwrap_or(&words));
        return;
    }
    let index = s.wall.index;
    let free = if g.wall_charge.free_set { Some(true) } else { None };
    let row = SetWallCharge { wall: s.wall, t: g.time, by: String::from(crate::ownblow::SAM_BY), paid, free };
    let id = g.tab.wallcharge_set_wall_charge.push(row);
    g.wall_charge.set.push(id);
    g.wall_charge.stats.by_sam += 1.0;
    crate::audio_iface::game_audio_like_play(g, "lockPick", Some(at), Some(0.6), None);
    crate::furnblast::furniture_warm(g);
    let f = g.k.doortools.blow_key.replacen("Key", "", 1);
    let tail = crate::ownblow::blow_tail(false, &f, Some(". "));
    crate::game::game_msg(g, &alloc::format!("charge set on the wall{}", tail));
    wall_charge_note(g, &alloc::format!("set by the player on wall {}", index));
}

/// Is Sam setting one right now (wallcharge.ts WallCharge.setting)
pub fn wall_charge_setting(g: &mut crate::game::Game) -> bool {
    g.wall_charge.sam.is_some()
}

/// The charge Sam is setting by hand, for whoever stands him at it. `at` is a copy: see notes, player.ts:2145 (wallcharge.ts WallCharge.samJob)
pub fn wall_charge_sam_job(g: &mut crate::game::Game) -> Option<crate::wallcharge::WallChargeSamJob> {
    let height = g.k.wallcharge.height;
    let (wall, at) = match g.wall_charge.sam.as_ref() { Some(s) => (s.wall.clone(), s.at), None => return None };
    Some(WallChargeSamJob { p: wall_pt(g, &wall, 0.0, 0.0, Some(height)), n: wall_n(&wall), at })
}

/// Broken off, nothing spent; true: there was one to break off (wallcharge.ts WallCharge.cancel)
pub fn wall_charge_cancel(g: &mut crate::game::Game) -> bool {
    if g.wall_charge.sam.is_none() { return false; }
    g.wall_charge.sam = None;
    crate::game::game_msg(g, "charge not set");
    true
}

/// The one row while he is at it (wallcharge.ts WallCharge.runningRow)
pub fn wall_charge_running_row(g: &mut crate::game::Game, running: &mut dyn FnMut(&mut crate::game::Game, crate::game::Interactable, crate::game::Verb, String, f64) -> crate::game::Interactable) -> crate::game::Interactable {
    let s = g.wall_charge.sam.clone().unwrap();
    let height = g.k.wallcharge.height;
    let it = match g.wall_charge.marker.clone() {
        Some(it) => it,
        None => crate::game::Interactable {
            kind: crate::game::InteractableKind::Wall,
            pos: wall_pt(g, &s.wall, 0.0, 0.05, Some(height)),
            line1: String::from(if s.wall.outer == Some(true) { "wall · outer wall" } else { "wall · partition" }),
            line2: String::new(),
            in_reach: true,
            off: false,
            ..Default::default()
        },
    };
    let progress = (g.time - s.t0) / g.k.doortools.set_secs;
    running(g, it, crate::game::Verb::Charge, String::from("setting the charge… — Space or a step breaks it off"), progress)
}

/// A point's x and z (wallcharge.ts xz)
pub fn xz(p: V3) -> [f64; 2] {
    [p[0], p[2]]
}
