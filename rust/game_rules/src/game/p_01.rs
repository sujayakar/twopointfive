use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// A man easing past furniture: a player crouched and not dashing; anyone else low.
pub fn quiet_man(g: &crate::game::Game, pl: Option<PlayerId>, c: CharId) -> bool {
    match pl {
        Some(p) => g.pl[p].crouch && !g.pl[p].dash,
        None => g.ch[c].anim.crouch > 0.5,
    }
}

/// Chaos Theory's own words: seen = assault; nothing known = ghost; else panther.
pub fn mission_rating(s: &crate::game::MissionStats) -> crate::game::MissionRating {
    if s.spotted > 0.0 {
        crate::game::MissionRating::Assault
    } else if s.kills == 0.0 && s.alerts == 0.0 && s.bodies == 0.0 {
        crate::game::MissionRating::Ghost
    } else {
        crate::game::MissionRating::Panther
    }
}

/// `m:ss` of a count of seconds.
pub fn fmt_clock(sec: f64) -> String {
    let s = m::max(0.0, m::floor(sec));
    let tail = js::num_str(js::rem(s, 60.0));
    let pad = if tail.len() < 2 { "0" } else { "" };
    alloc::format!("{}:{}{}", js::num_str(m::floor(s / 60.0)), pad, tail)
}

/// The mission thread as a run starts. `serial` is 0 here, having no `g`: the caller that writes `g.mission` whole deals it.
pub fn new_mission(t0: f64) -> crate::game::Mission {
    crate::game::Mission {
        stage: crate::game::MissionStage::Infiltrate,
        t0,
        drive_t: -1.0,
        pulled: false,
        alarm_on: false,
        sweep_at: -1.0,
        spotted_t: -100.0,
        down_card_at: -1.0,
        hands_on: false,
        stats: crate::game::MissionStats {
            qrf_who: None,
            elapsed: 0.0,
            alerts: 0.0,
            spotted: 0.0,
            bodies: 0.0,
            shots: 0.0,
            kills: 0.0,
            knockouts: 0.0,
            keys: 0.0,
            blackout: false,
            sandbox: false,
            qrf: 0.0,
            qrf_at: -1.0,
        },
        serial: 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::{MissionRating, MissionStage};

    /// Expected strings printed by the TypeScript's own fmtClock under bun.
    #[test]
    fn fmt_clock_matches_the_typescript() {
        let cases: &[(f64, &str)] = &[
            (0.0, "0:00"),
            (-0.0, "0:00"),
            (-5.0, "0:00"),
            (0.5, "0:00"),
            (9.99, "0:09"),
            (59.9, "0:59"),
            (60.0, "1:00"),
            (61.0, "1:01"),
            (3725.5, "62:05"),
            (359999.0, "5999:59"),
            (1e15, "16666666666666:40"),
            (f64::NAN, "NaN:NaN"),
            (f64::INFINITY, "Infinity:NaN"),
            (f64::NEG_INFINITY, "0:00"),
            (1e21, "16666666666666666000:40"),
            (6e22, "1e+21:00"),
            (1.2e23, "2e+21:00"),
        ];
        for (x, want) in cases {
            assert_eq!(fmt_clock(*x), *want, "fmt_clock({})", x);
        }
    }

    /// Answers printed by the TypeScript's own missionRating under bun.
    #[test]
    fn mission_rating_matches_the_typescript() {
        let base = new_mission(0.0).stats;
        let with = |f: fn(&mut crate::game::MissionStats)| {
            let mut s = base.clone();
            f(&mut s);
            mission_rating(&s)
        };
        assert_eq!(mission_rating(&base), MissionRating::Ghost);
        assert_eq!(with(|s| { s.spotted = 1.0; s.kills = 1.0; s.alerts = 1.0; s.bodies = 1.0; }), MissionRating::Assault);
        assert_eq!(with(|s| s.kills = 1.0), MissionRating::Panther);
        assert_eq!(with(|s| s.alerts = 1.0), MissionRating::Panther);
        assert_eq!(with(|s| s.bodies = 1.0), MissionRating::Panther);
    }

    #[test]
    fn a_new_mission_holds_the_makers_numbers_not_blanks() {
        let n = new_mission(7.5);
        assert_eq!(n.stage, MissionStage::Infiltrate);
        assert_eq!((n.t0, n.drive_t, n.sweep_at, n.spotted_t, n.down_card_at, n.stats.qrf_at), (7.5, -1.0, -1.0, -100.0, -1.0, -1.0));
        assert_eq!(n.serial, 0);
    }
}
