//! src/game/close.ts lines 463 to 487
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// Once a frame before the guards: the seated men kept on their marks while calm, stood up when not.   (close.ts closeSeatTick)
pub fn close_seat_tick(g: &mut crate::game::Game, dt: f64) {
    match &g.m.close.seated {
        Some(seated) if !seated.is_empty() => {}
        _ => return,
    }
    let mut at = 0;
    // A live walk: closeUnseat deletes the row the walk stands on, as under the TypeScript's `for … of` over a Map.
    while let Some(gd) = g.m.close.seated.as_ref().and_then(|seated| seated.next_key(&mut at)) {
        if !g.guards.contains(&gd) || !calm(g, gd) {
            close_unseat(g, gd, None);
            continue;
        }
        let st = seam_seat_row(g, gd);
        let c = g.gd[gd].ch;
        g.ch[c].body_yaw = v3::approach_angle(g.ch[c].body_yaw, st.face, 3.0 * dt);
        let settled = m::abs(v3::wrap_angle(g.ch[c].body_yaw - st.face)) < 0.35;
        let tx = st.at[0] + m::sin(st.face) * crate::civilians::SIT_ADVANCE;
        let tz = st.at[2] + m::cos(st.face) * crate::civilians::SIT_ADVANCE;
        let k = 1.0 - m::exp(-5.0 * dt);
        if settled {
            g.ch[c].pos[0] += (tx - g.ch[c].pos[0]) * k;
            g.ch[c].pos[2] += (tz - g.ch[c].pos[2]) * k;
            let clip = sit_clip();
            g.ch[c].anim.act = Some(crate::character::AnimAct { clip: clip.clip, loop_: Some(clip.loop_), rate: Some(clip.rate), in_: Some(clip.in_), out: Some(clip.out) });
        }
        g.gd[gd].speed = 0.0;
        g.ch[c].vel = [0.0, 0.0, 0.0];
        g.gd[gd].path = Vec::new();
        g.gd[gd].path_goal = None;
        let down = settled && crate::character::anim_act_shown(g, c) > 0.5;
        seam_seat_set_down(g, gd, down);
    }
}

/// Everybody up, this seat's close-in memory gone.   (close.ts closeReset)
pub fn close_reset(g: &mut crate::game::Game) {
    let seated = g.m.close.seated.as_ref().map(|seated| seated.keys()).unwrap_or_default();
    for gd in seated {
        close_unseat(g, gd, None);
    }
    let player = g.player;
    let Some(s) = g.m.close.state.get_mut(player) else { return };
    s.best = None;
    s.t = -1.0;
    s.mode = None;
    s.mode_d = f64::INFINITY;
    s.rmb_t = -1.0;
}

/// The debug panel's line on the close-in mode. `CLOSE.on` is settled on.   (close.ts closeSummary)
pub fn close_summary(g: &mut crate::game::Game) -> String {
    let now = g.time;
    let s = state_of(g);
    let (mode, mode_d) = (s.mode, s.mode_d);
    let o = if s.t == now { s.best.clone() } else { None };
    let in_mode = match mode {
        Some(gd) => alloc::format!("in the mode on {} {} m", g.gd[gd].callsign, to_fixed_2(mode_d)),
        None => String::from("not in the mode"),
    };
    let best = match &o {
        Some(o) => alloc::format!("{} {} {} m", g.gd[o.guard].callsign, close_kind_of(Some(o)).as_str(), to_fixed_2(o.dist)),
        None => String::from("\u{2014}"),
    };
    let (t, gr) = (g.stats.close.takes, g.stats.close.grabs);
    let seated = g.m.close.seated.as_ref().map_or(0, |seated| seated.len());
    alloc::format!(
        "on \u{b7} {} \u{b7} now {} \u{b7} takes b{} s{} f{} c{} \u{b7} grabs b{} c{} \u{b7} seated {}",
        in_mode,
        best,
        t[CloseKind::Behind as usize],
        t[CloseKind::Side as usize],
        t[CloseKind::Front as usize],
        t[CloseKind::Seat as usize],
        gr[CloseKind::Behind as usize],
        gr[CloseKind::Seat as usize],
        seated,
    )
}

/// A seated man's row of SEATED (close.ts:148 `Map<Guard, Seat>`).
fn seam_seat_row(g: &mut crate::game::Game, gd: GuardId) -> crate::close::Seat {
    todo!("needs: g.m.close.seated's value to be close::Seat, not cmd's SeatId")
}

/// The write `st.down = …` through the row (close.ts:474).
fn seam_seat_set_down(g: &mut crate::game::Game, gd: GuardId, down: bool) {
    todo!("needs: g.m.close.seated's value to be close::Seat, not cmd's SeatId")
}

/// `x.toFixed(2)`: the nearest hundredth, and of two equally near the larger (Rust's `{:.2}` takes the even one, on an exact tie: an odd number of eighths)
fn to_fixed_2(x: f64) -> String {
    if x.is_nan() {
        return String::from("NaN");
    }
    let a = m::abs(x);
    let sign = if x < 0.0 { "-" } else { "" };
    if !a.is_finite() {
        return alloc::format!("{}Infinity", sign);
    }
    let eighths = a * 8.0;
    let tie = m::floor(eighths) == eighths && js::rem(eighths, 2.0) == 1.0;
    if tie {
        alloc::format!("{}{:.2}", sign, m::floor(a * 100.0 + 0.5) / 100.0)
    } else {
        alloc::format!("{}{:.2}", sign, a)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_fixed_2_matches_the_typescript() {
        // the language's own answers (bun: x.toFixed(2))
        let cases: [(f64, &str); 18] = [
            (0.125, "0.13"), (0.375, "0.38"), (0.625, "0.63"), (0.875, "0.88"), (1.125, "1.13"), (2.5, "2.50"),
            (0.005, "0.01"), (0.015, "0.01"), (1.005, "1.00"), (0.0, "0.00"), (-0.0, "0.00"), (-0.125, "-0.13"),
            (-0.001, "-0.00"), (12.345, "12.35"), (1.999, "2.00"), (0.994999, "0.99"), (f64::INFINITY, "Infinity"), (100.125, "100.13"),
        ];
        for (x, want) in cases {
            assert_eq!(to_fixed_2(x), want, "{}", x);
        }
    }
}
