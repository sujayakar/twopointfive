//! src/game/squadnet.ts lines 1 to 165
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// A fresh set of the net's counters. (squadnet.ts newStats)
pub fn new_stats() -> crate::squadnet::SquadNetStats {
    crate::squadnet::SquadNetStats { lines: 0.0, downs: 0.0, whys: 0.0, quiets: 0.0, clears: 0.0, ends: 0.0 }
}

/// The net as a game starts it; the class's constructor adds nothing to its initialisers. (squadnet.ts SquadNet.constructor)
pub fn squad_net_new(g: &mut crate::game::Game) -> crate::squadnet::SquadNet {
    crate::squadnet::SquadNet::blank()
}

/// A fresh encounter: everything but `last_t`, which the TypeScript leaves standing. (squadnet.ts SquadNet.reset)
pub fn squad_net_reset(g: &mut crate::game::Game) {
    let n = &mut g.squad_net;
    n.rows = Vec::new();
    n.down = Vec::new();
    n.seeded = false;
    n.click_due = false;
    n.down_due = false;
    n.stats = new_stats();
}

/// A teammate said a line with his head off your screen: the net clicks. (squadnet.ts SquadNet.line)
pub fn squad_net_line(g: &mut crate::game::Game) {
    g.squad_net.stats.lines += 1.0;
    g.squad_net.click_due = true;
}

/// Once a frame, after everybody has moved: who went down this frame, rows past their time, the one sound owed. (squadnet.ts SquadNet.tick)
pub fn squad_net_tick(g: &mut crate::game::Game) {
    let t = g.time;
    if t < g.squad_net.last_t { squad_net_reset(g); }
    g.squad_net.last_t = t;
    let squad = crate::seats::team_squad(g);
    if !g.squad_net.down.is_empty() {
        let kept: Vec<String> = g.squad_net.down.iter()
            .filter(|c| squad.iter().any(|&gd| g.gd[gd].callsign == **c && g.gd[gd].state == crate::game::GuardState::Dead))
            .cloned().collect();
        g.squad_net.down = kept;
    }
    for &gd in &squad {
        if g.gd[gd].state != crate::game::GuardState::Dead || g.squad_net.down.contains(&g.gd[gd].callsign) { continue; }
        let callsign = g.gd[gd].callsign.clone();
        g.squad_net.down.push(callsign.clone());
        if !g.squad_net.seeded { continue; }
        let p = g.ch[g.gd[gd].ch].pos;
        let dur = g.k.squadnet.down_secs;
        g.squad_net.rows.push(crate::squadnet::NetRow { text: alloc::format!("{callsign} is down."), at: [p[0], p[1], p[2]], t, dur });
        g.squad_net.stats.downs += 1.0;
        g.squad_net.down_due = true;
    }
    g.squad_net.seeded = true;
    if !g.squad_net.rows.is_empty() && t - g.squad_net.rows[0].t > g.squad_net.rows[0].dur {
        g.squad_net.rows.retain(|r| t - r.t <= r.dur);
    }
    if g.squad_net.down_due {
        if g.k.squadnet.down_gain > 0.0 {
            let (gain, rate) = (g.k.squadnet.down_gain, g.k.squadnet.down_rate);
            crate::audio_iface::game_audio_like_play(g, "radio", None, Some(gain), Some(&crate::audio_iface::PlayOpts::Rate { rate }));
        }
    } else if g.squad_net.click_due && g.k.squadnet.click > 0.0 {
        let (gain, rate) = (g.k.squadnet.click, g.k.squadnet.click_rate);
        crate::audio_iface::game_audio_like_play(g, "radio", None, Some(gain), Some(&crate::audio_iface::PlayOpts::Rate { rate }));
    }
    g.squad_net.click_due = false;
    g.squad_net.down_due = false;
}

/// What a quick-key pick that is not a plain open adds: the end of its row and the answer. (squadnet.ts whyWords)
pub fn why_words(g: &mut crate::game::Game, p: &crate::autopilot::AutoPick) -> Option<crate::squadnet::WhyWords> {
    use crate::autopilot::AutoPickWhy;
    if p.word || !matches!(p.t, crate::orders::OrderTarget::Door(_)) { return None; }
    let id = p.id.as_deref();
    let (row, ack) = if id == Some("bangClear") && p.why == AutoPickWhy::Contact {
        ("they see one inside", "banging it. i see one in there.")
    } else if id == Some("bangClear") && p.why == AutoPickWhy::Unknown {
        ("nobody's been in there", "banging it. nobody's been in there.")
    } else if id == Some("charge") && p.why == AutoPickWhy::Shut {
        ("they can't open it", "we can't open it. setting a charge.")
    } else if id == Some("openClear") && p.kept == Some(true) {
        ("they're saving the last bang", "going in. i'm saving the last bang.")
    } else {
        return None;
    };
    Some(crate::squadnet::WhyWords { row: String::from(row), ack: String::from(ack) })
}

/// The regex `/[^s]s$/`: ends in one s, not two, with something before it ('bunks are', 'boss's').
fn ends_plural(name: &str) -> bool {
    let mut back = name.chars().rev();
    back.next() == Some('s') && matches!(back.next(), Some(c) if c != 's')
}

/// The call a lead makes when an entry is over. None: nothing is said. (squadnet.ts roomCall)
pub fn room_call(g: &mut crate::game::Game, name: &str, left: f64, inside: bool, seen: bool) -> Option<String> {
    if !inside && left <= 0.0 {
        g.squad_net.stats.clears += 1.0;
        return Some(alloc::format!("{name} clear."));
    }
    if left > 0.0 && !seen {
        g.squad_net.stats.quiets += 1.0;
        let verb = if ends_plural(name) { " are" } else { "'s" };
        return Some(alloc::format!("{name}{verb} quiet. we didn't see all of it."));
    }
    None
}

/// The call at the end of an entry into a room the mission has no name for; `open` = open floor. (squadnet.ts plainCall)
pub fn plain_call(g: &mut crate::game::Game, open: bool, left: f64, inside: bool, seen: bool) -> Option<String> {
    let call = if open {
        if seen { None } else { Some(String::from("clear so far.")) }
    } else {
        room_call(g, "room", left, inside, seen)
    };
    if call.is_some() { g.squad_net.stats.ends += 1.0; }
    call
}

/// Does a man at p stand in that room's flood, a doorway of the room's own counted. (squadnet.ts inFlood)
pub fn in_flood(g: &mut crate::game::Game, r: &crate::squadnet::InFloodR, p: V3) -> bool {
    let cell = g.col.nav.cell;
    let c = crate::scene_collision::nav_grid_idx(g, m::floor(p[0] / cell), m::floor(p[2] / cell));
    if r.cells.has(crate::Num(c)) { return true; }
    match crate::guards::door_cells(g).get(crate::Num(c)) {
        Some(d) => r.entries.iter().any(|e| e.door == *d),
        None => false,
    }
}

/// Where a far line is drawn, in the view's pixels; None: he is where it would be drawn from. `SQUADNET.edge` is settled on. (squadnet.ts netPlace)
pub fn net_place(g: &mut crate::game::Game, w: f64, h: f64, px: f64, py: f64, mx: f64, my: f64, bw: f64, bh: f64, hits: &mut dyn FnMut(&mut crate::game::Game, f64, f64) -> bool) -> Option<[f64; 2]> {
    let clamp = crate::v3::clamp;
    let l = m::hyp2(mx - px, my - py);
    if !(l > 1.0) { return None; }
    let (x0, y0) = (bw / 2.0 + 6.0, bh + 12.0);
    let (x1, y1) = (m::max(x0, w - bw / 2.0 - 6.0), m::max(y0, h - 12.0));
    let short = m::min(w, h);
    let pad = m::min_n(&[g.k.squadnet.edge_in * short, (x1 - x0) / 2.0, (y1 - y0) / 2.0]);
    let (bx0, bx1, by0, by1) = (x0 + pad, x1 - pad, y0 + pad, y1 - pad);
    let (cx, cy) = ((bx0 + bx1) / 2.0, (by0 + by1) / 2.0);
    let (mut x, mut y, mut side) = (clamp(mx, bx0, bx1), clamp(my, by0, by1), true);
    if x != mx || y != my {
        let ins = NET_INSET * short;
        let ax = clamp(px, m::min(bx0 + ins, cx), m::max(bx1 - ins, cx));
        let ay = clamp(py, m::min(by0 + ins, cy), m::max(by1 - ins, cy));
        let (dx, dy) = (mx - ax, my - ay);
        let tx = if dx > 0.0 { (bx1 - ax) / dx } else if dx < 0.0 { (bx0 - ax) / dx } else { f64::INFINITY };
        let ty = if dy > 0.0 { (by1 - ay) / dy } else if dy < 0.0 { (by0 - ay) / dy } else { f64::INFINITY };
        let t = m::min(tx, ty);
        x = clamp(ax + dx * t, bx0, bx1);
        y = clamp(ay + dy * t, by0, by1);
        side = tx <= ty;
    }
    if !hits(g, x, y) { return Some([x, y]); }
    if !side {
        for k in 1..=2 {
            let yi = y + (if y > cy { -1.0 } else { 1.0 }) * NET_ROW * k as f64;
            if yi >= by0 && yi <= by1 && !hits(g, x, yi) { return Some([x, yi]); }
        }
    }
    let (v, lo, hi) = if side { (y, by0, by1) } else { (x, bx0, bx1) };
    let first = if v > (lo + hi) / 2.0 { -1.0 } else { 1.0 };
    let mut k = 1;
    while k as f64 * NET_ROW <= hi - lo {
        for s in [first, -first] {
            let u = v + s * NET_ROW * k as f64;
            if u >= lo && u <= hi && !hits(g, if side { x } else { u }, if side { u } else { y }) {
                return Some(if side { [x, u] } else { [u, y] });
            }
        }
        k += 1;
    }
    Some([x, y])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::autopilot::{AutoPick, AutoPickWhy};
    use crate::orders::{OrderTarget, OrderTargetDoor};

    fn on_big_stack(f: fn()) {
        extern crate std;
        std::thread::Builder::new().stack_size(256 << 20).spawn(f).unwrap().join().unwrap();
    }

    fn pick(id: &str, why: AutoPickWhy, kept: Option<bool>) -> AutoPick {
        AutoPick {
            id: Some(String::from(id)),
            why,
            kept,
            t: OrderTarget::Door(OrderTargetDoor { at: [0.0; 3], door: DoorId(0), side: 1.0 }),
            ..Default::default()
        }
    }

    #[test]
    fn the_quiet_call_agrees_in_number_as_the_regex_does() {
        for (name, plural) in [("bunks", true), ("server room", false), ("boss", false), ("s", false), ("", false), ("as", true)] {
            assert_eq!(ends_plural(name), plural, "{name:?}");
        }
    }

    #[test]
    fn room_call_and_its_counters() {
        on_big_stack(|| {
            let g = &mut Box::new(Game::blank());
            assert_eq!(room_call(g, "mess", 0.0, false, false).as_deref(), Some("mess clear."));
            assert_eq!(room_call(g, "bunks", 3.0, false, false).as_deref(), Some("bunks are quiet. we didn't see all of it."));
            assert_eq!(room_call(g, "server room", 3.0, true, false).as_deref(), Some("server room's quiet. we didn't see all of it."));
            assert_eq!(room_call(g, "mess", 0.0, true, false), None);
            assert_eq!(room_call(g, "mess", 3.0, true, true), None);
            assert_eq!(room_call(g, "mess", 3.0, false, true), None);
            assert_eq!((g.squad_net.stats.clears, g.squad_net.stats.quiets), (1.0, 2.0));
        });
    }

    #[test]
    fn plain_call_counts_only_what_it_says() {
        on_big_stack(|| {
            let g = &mut Box::new(Game::blank());
            assert_eq!(plain_call(g, true, 0.0, true, false).as_deref(), Some("clear so far."));
            assert_eq!(plain_call(g, true, 0.0, false, true), None);
            assert_eq!(plain_call(g, false, 0.0, false, false).as_deref(), Some("room clear."));
            assert_eq!(plain_call(g, false, 3.0, false, false).as_deref(), Some("room's quiet. we didn't see all of it."));
            assert_eq!(plain_call(g, false, 3.0, true, true), None);
            assert_eq!(g.squad_net.stats.ends, 3.0);
        });
    }

    #[test]
    fn why_words_says_only_the_reasons() {
        on_big_stack(|| {
            let g = &mut Box::new(Game::blank());
            let ack = |g: &mut Game, p: &AutoPick| why_words(g, p).map(|w| w.ack);
            assert_eq!(ack(g, &pick("bangClear", AutoPickWhy::Contact, None)).as_deref(), Some("banging it. i see one in there."));
            assert_eq!(ack(g, &pick("bangClear", AutoPickWhy::Unknown, None)).as_deref(), Some("banging it. nobody's been in there."));
            assert_eq!(ack(g, &pick("charge", AutoPickWhy::Shut, None)).as_deref(), Some("we can't open it. setting a charge."));
            assert_eq!(ack(g, &pick("openClear", AutoPickWhy::Known, Some(true))).as_deref(), Some("going in. i'm saving the last bang."));
            assert_eq!(ack(g, &pick("openClear", AutoPickWhy::Known, Some(false))), None);
            assert_eq!(ack(g, &pick("bangClear", AutoPickWhy::Known, None)), None);
            let mut said = pick("bangClear", AutoPickWhy::Contact, None);
            said.word = true;
            assert_eq!(ack(g, &said), None);
            let mut floor = pick("bangClear", AutoPickWhy::Contact, None);
            floor.t = OrderTarget::Floor(Default::default());
            assert_eq!(ack(g, &floor), None);
        });
    }

    #[test]
    fn net_place_has_the_typescripts_bits() {
        on_big_stack(|| {
            let g = &mut Box::new(Game::blank());
            // (w, h, px, py, mx, my, bw, bh), how many first asks hit, the TypeScript's answer (SQUADNET.edge on, under bun, EXACT on)
            let cases: [([f64; 8], i32, Option<[u64; 2]>); 9] = [
                ([1600.0, 900.0, 800.0, 450.0, 1000.0, 500.0, 120.0, 20.0], 0, Some([0x408f400000000000, 0x407f400000000000])),
                ([1600.0, 900.0, 800.0, 450.0, 800.5, 450.5, 120.0, 20.0], 0, None),
                ([1600.0, 900.0, 800.0, 450.0, 3333.3, 611.7, 137.5, 22.0], 0, Some([0x4097450000000000, 0x407edfea3677d46d])),
                ([1600.0, 900.0, 800.0, 450.0, -901.1, 123.4, 137.5, 22.0], 2, Some([0x405bb00000000000, 0x40723ab254e82d63])),
                ([1600.0, 900.0, 812.7, 450.0, 901.9, -777.7, 137.5, 22.0], 1, Some([0x408a42798cec0728, 0x4058000000000000])),
                ([1600.0, 900.0, 812.7, 450.0, 901.9, -777.7, 137.5, 22.0], 3, Some([0x408972798cec0728, 0x4051800000000000])),
                ([1280.0, 720.0, 30.25, 700.5, 99.5, 5000.125, 90.0, 18.0], 5, Some([0x406db4b28397e8c8, 0x408539999999999a])),
                ([640.0, 360.0, 320.0, 180.0, 4000.0, 180.0, 90.0, 18.0], 100000, Some([0x4081f4cccccccccd, 0x4066800000000000])),
                ([100.0, 60.0, 50.0, 30.0, 900.0, -900.0, 300.0, 50.0], 0, Some([0x4063800000000000, 0x404f000000000000])),
            ];
            for (a, n, want) in cases {
                let mut asked = 0;
                let mut hits = |_: &mut Game, _: f64, _: f64| { asked += 1; asked <= n };
                let got = net_place(g, a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7], &mut hits);
                assert_eq!(got.map(|r| [r[0].to_bits(), r[1].to_bits()]), want, "{a:?} {n}");
            }
        });
    }

    #[test]
    fn reset_keeps_last_t() {
        on_big_stack(|| {
            let g = &mut Box::new(Game::blank());
            g.squad_net.last_t = 7.0;
            g.squad_net.seeded = true;
            squad_net_line(g);
            assert!(g.squad_net.click_due);
            squad_net_reset(g);
            assert!(!g.squad_net.click_due && !g.squad_net.seeded);
            assert_eq!((g.squad_net.stats.lines, g.squad_net.last_t), (0.0, 7.0));
        });
    }
}
