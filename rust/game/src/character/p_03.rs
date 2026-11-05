//! src/game/character.ts lines 693 to 1031
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// One step of longTuck's `to`: t[i] toward v by w.
fn tuck_to(t: &mut Vec<f64>, w: f64, i: usize, v: f64) {
    t[i] += (v - t[i]) * w;
}

/// A long carriage's targets pulled toward WALL_TUCK.long by w (0‥1), in place. (character.ts longTuck)
pub fn long_tuck(t: &mut Vec<f64>, gun_wall: f64) {
    let w = v3::clamp(gun_wall, 0.0, 1.0);
    let c = lc();
    let wall = wall_tuck();
    let tl = &wall.long;
    tuck_to(t, w, c.side as usize, tl.side);
    tuck_to(t, w, c.up as usize, tl.up);
    tuck_to(t, w, c.fwd as usize, tl.fwd);
    tuck_to(t, w, c.pitch as usize, tl.pitch);
    tuck_to(t, w, c.yaw as usize, tl.yaw);
    tuck_to(t, w, c.aim_k as usize, tl.aim_k);
    for k in 0..3 {
        tuck_to(t, w, c.elbow_r as usize + k, tl.elbow_r[k]);
        tuck_to(t, w, c.elbow_l as usize + k, tl.elbow_l[k]);
    }
    t[c.fwd as usize] -= v3::clamp(gun_wall - 1.0, 0.0, 1.0) * wall.over;
}

/// The animator's tables of numbers by the name rust/sim reads them under: tooling, and game/rustknobs.ts's first word `ANIM`. (character.ts animTables)
pub fn anim_tables() -> JsMap<String, crate::character::AnimTablesValue> {
    todo!("needs: a value that can hold a table; AnimTablesValue's object arm carries nothing, so a path under ANIM cannot be read through this map")
}

pub mod fold_with {
    /// Who else raises the fold: a function in a table, written by wounds.ts:43 to read WOUND.on, which this signature has no `g` to reach. fold_on
    /// reads the switch directly. (character.ts FOLD_WITH.on)
    pub fn on() -> bool {
        todo!("needs: g, for WOUND.on (wounds.ts:43); fold_on reads it directly and nothing else calls this")
    }
}

/// Is the stagger's fold up: `too` is the figure's own foldToo; while MOVES.fold is 0 the wounds switch raises it too. (character.ts foldOn)
pub fn fold_on(g: &mut Game, too: Option<bool>) -> bool {
    let too = too.unwrap_or(false);
    g.k.moves.clutch > 0.5 && (g.k.moves.fold > 0.5 || (g.k.moves.fold == 0.0 && (too || g.k.wound.on)))
}

/// The master switch of the motion pass, for the strips' tool. (character.ts setMoves)
pub fn set_moves(g: &mut Game, on: bool) {
    g.k.moves.on = if on { 1.0 } else { 0.0 };
}

/// One part of the motion pass: on? (character.ts mvOn)
pub fn mv_on(g: &mut Game, k: crate::character::MvOnK) -> bool {
    use crate::character::MvOnK;
    let t = &g.k.moves;
    let v = match k {
        MvOnK::On => return true,
        MvOnK::Reload => t.reload,
        MvOnK::Fold => t.fold,
        MvOnK::Steady => t.steady,
        MvOnK::Toss => t.toss,
        MvOnK::Entry => t.entry,
        MvOnK::Flinch => t.flinch,
        MvOnK::Fall => t.fall,
        MvOnK::Squeeze => t.squeeze,
        MvOnK::Snap => t.snap,
        MvOnK::Recoil => t.recoil,
        MvOnK::Dazzle => t.dazzle,
        MvOnK::Gait => t.gait,
        MvOnK::Bound => t.bound,
        MvOnK::Clutch => t.clutch,
        MvOnK::EntryEyes => t.entry_eyes,
        MvOnK::SteadyRun => t.steady_run,
    };
    v > 0.5
}

/// A read of an ArrayLike: past its end is undefined, which is NaN in the sum.
fn q_at(a: &[f64], n: f64) -> f64 {
    a.get(n as usize).copied().unwrap_or(m::NAN)
}

/// The angle between the rotation at a[i] (its four numbers `s` apart) and the one at b[k‥k+3]. (character.ts qAng)
pub fn q_ang(a: &[f64], i: f64, s: f64, b: &[f64], k: f64) -> f64 {
    let d = m::abs(q_at(a, i) * q_at(b, k) + q_at(a, i + s) * q_at(b, k + 1.0) + q_at(a, i + 2.0 * s) * q_at(b, k + 2.0) + q_at(a, i + 3.0 * s) * q_at(b, k + 3.0));
    if d >= 1.0 { 0.0 } else { 2.0 * m::acos(d) }
}

/// MOVES.entry's lane: something (dx, dz) from a man stands before his muzzle. (character.ts inDipLane)
pub fn in_dip_lane(dx: f64, dz: f64, gun_yaw: f64) -> bool {
    let l = mv2().dip.lane;
    let fx = m::sin(gun_yaw);
    let fz = m::cos(gun_yaw);
    let along = dx * fx + dz * fz;
    along > l[0] && along < l[1] && m::abs(dx * fz - dz * fx) < l[2]
}

/// MOVES.reload's marks on a long gun, off the trigger hand's wrist in the gun's frame. (character.ts reloadMarks)
pub fn reload_marks(back: bool) -> crate::character::MV2ReloadBox {
    let r = mv2().reload;
    if !back {
        return r.box_;
    }
    let b = r.back;
    crate::character::MV2ReloadBox { mag: b.mag, strip: b.strip, handle: b.handle, pull: b.pull, hf: b.hf, hb: b.hb }
}

/// A row of TURN by its TypeScript key (`Object.hasOwn(TURN, k)`, `TURN[k]`); None = no such key.
fn turn_row<'a>(t: &'a mut crate::knobs::TurnK, k: &str) -> Option<&'a mut f64> {
    match k {
        "on" => Some(&mut t.on),
        "aim" => Some(&mut t.aim),
        "aimAcc" => Some(&mut t.aim_acc),
        "body" => Some(&mut t.body),
        "bodyAcc" => Some(&mut t.body_acc),
        "bodyShare" => Some(&mut t.body_share),
        "green" => Some(&mut t.green),
        "lead" => Some(&mut t.lead),
        "leadRate" => Some(&mut t.lead_rate),
        "jump" => Some(&mut t.jump),
        _ => None,
    }
}

/// `Number(string)`: blanks trimmed, nothing is 0, 0x / 0o / 0b unsigned, else a decimal; NaN when it is none. Rust's "inf" and "nan" are not finite
/// either, and the only reader asks for a finite number.
fn js_number(s: &str) -> f64 {
    let s = s.trim_matches(|c: char| (c.is_whitespace() && c != '\u{85}') || c == '\u{feff}');
    if s.is_empty() {
        return 0.0;
    }
    let radix = match s.get(..2) {
        Some("0x") | Some("0X") => 16,
        Some("0o") | Some("0O") => 8,
        Some("0b") | Some("0B") => 2,
        _ => 0,
    };
    if radix == 0 {
        return s.parse::<f64>().unwrap_or(m::NAN);
    }
    let digits = &s[2..];
    if digits.is_empty() {
        return m::NAN;
    }
    let mut r = 0.0;
    for c in digits.chars() {
        match c.to_digit(radix) {
            Some(d) => r = r * radix as f64 + d as f64,
            None => return m::NAN,
        }
    }
    r
}

/// setTurn on the table itself.
fn set_turn_on(t: &mut crate::knobs::TurnK, spec: &str, dflt: f64) -> bool {
    let mut parts = spec.split(',');
    let head = parts.next().unwrap_or("");
    let mut ok = true;
    let feel = turn_feel_table();
    let rates = match head {
        "light" => Some((feel.light.aim, feel.light.aim_acc, feel.light.body, feel.light.body_acc)),
        "medium" => Some((feel.medium.aim, feel.medium.aim_acc, feel.medium.body, feel.medium.body_acc)),
        "heavy" => Some((feel.heavy.aim, feel.heavy.aim_acc, feel.heavy.body, feel.heavy.body_acc)),
        _ => None,
    };
    if let Some((aim, aim_acc, body, body_acc)) = rates {
        t.aim = aim;
        t.aim_acc = aim_acc;
        t.body = body;
        t.body_acc = body_acc;
        t.on = 1.0;
    } else if head == "0" {
        t.on = 0.0;
    } else if head == "1" {
        t.on = 1.0;
    } else {
        t.on = dflt;
        ok = head.is_empty();
    }
    for kv in parts {
        let mut pair = kv.split('=');
        let k = pair.next().unwrap_or("");
        let n = match pair.next() {
            Some(v) if !v.is_empty() => js_number(v),
            _ => m::NAN,
        };
        match turn_row(t, k) {
            Some(row) if n.is_finite() => *row = n,
            _ => ok = false,
        }
    }
    ok
}

/// THE one reading of the turn switch: `0` | `1` | a TURN_FEEL name, then any `,key=number` pairs over TURN's own keys; an empty head = `dflt`. False when
/// it was not understood. (character.ts setTurn)
pub fn set_turn(g: &mut Game, spec: &str, dflt: Option<f64>) -> bool {
    let dflt = dflt.unwrap_or(1.0);
    set_turn_on(&mut g.k.turn, spec, dflt)
}

/// turnFeel on the table itself.
fn feel_name(t: &crate::knobs::TurnK) -> &'static str {
    let feel = turn_feel_table();
    if feel.light.aim == t.aim && feel.light.aim_acc == t.aim_acc && feel.light.body == t.body && feel.light.body_acc == t.body_acc {
        return "light";
    }
    if feel.medium.aim == t.aim && feel.medium.aim_acc == t.aim_acc && feel.medium.body == t.body && feel.medium.body_acc == t.body_acc {
        return "medium";
    }
    if feel.heavy.aim == t.aim && feel.heavy.aim_acc == t.aim_acc && feel.heavy.body == t.body && feel.heavy.body_acc == t.body_acc {
        return "heavy";
    }
    "own numbers"
}

/// Which setting is live, in words: a TURN_FEEL name, or 'own numbers'. (character.ts turnFeel)
pub fn turn_feel(g: &mut Game) -> String {
    String::from(feel_name(&g.k.turn))
}

#[cfg(test)]
mod tests {
    use super::*;

    // The expected bits are the TypeScript's own, run under bun with EXACT.on.
    #[test]
    fn q_ang_has_the_typescripts_bits() {
        let a = [0.1, 0.0, 0.2, 0.0, 0.3, 0.0, 0.9273618495495703, 0.0];
        let b = [0.5, -0.5, 0.5, 0.5];
        assert_eq!(q_ang(&a, 0.0, 2.0, &b, 0.0).to_bits(), 0x3fff1a4dc03e2d76);
        let c = [9.0, 0.2, 0.4, 0.4, 0.7999999999999999];
        let e = [7.0, 0.1, 0.7, 0.1, 0.6999999999999999];
        assert_eq!(q_ang(&c, 1.0, 1.0, &e, 1.0).to_bits(), 0x3fecdd9f8f922ea1);
        assert_eq!(q_ang(&[0.0, 0.0, 0.0, 1.0], 0.0, 1.0, &[0.0, 0.0, 0.0, 1.0], 0.0).to_bits(), 0);
        assert_eq!(q_ang(&[0.0, 0.0, 0.0, -0.5], 0.0, 1.0, &[0.1, 0.2, 0.3, 0.9], 0.0).to_bits(), 0x4001aa1c65a25825);
    }

    #[test]
    fn long_tuck_has_the_typescripts_bits() {
        let run = |gun_wall: f64| -> Vec<u64> {
            let mut t: Vec<f64> = (0..21).map(|i| 0.1 * (i as f64) - 0.7).collect();
            long_tuck(&mut t, gun_wall);
            t.iter().map(|x| x.to_bits()).collect()
        };
        let part: [u64; 21] = [0xbfe010cb295e9e1b, 0xbfddfe5c91d14e3c, 0xbfcf8ef34d6a161e, 0xc03c5f3b645a1cab, 0xbfc83126e978d4fc, 0x4015b22d0e560418, 0xbfb9999999999990, 0x3ca0000000000000, 0x3fb99999999999a0, 0x3fc999999999999c, 0x3fd3333333333334, 0xbf9a1cac083126d0, 0xbfac28f5c28f5c10, 0x3fcb126e978d4fe2, 0x3fe5374bc6a7ef9f, 0x3fc126e978d4fdf4, 0x3fde5e353f7ced93, 0x3ff0000000000001, 0x3ff199999999999a, 0x3ff3333333333334, 0x3ff4cccccccccccd];
        assert_eq!(run(0.37), part);
        let over: [u64; 21] = [0xbfc51eb851eb8520, 0xbfcf5c28f5c28f5c, 0x3fbced916872b020, 0xc053000000000000, 0x0000000000000000, 0x402e000000000000, 0xbfb9999999999990, 0x3ca0000000000000, 0x3fb99999999999a0, 0x3fc999999999999c, 0x3fd3333333333334, 0xbfe8000000000000, 0xbff0000000000000, 0xbfdccccccccccccc, 0x3fe3333333333333, 0xbff0000000000000, 0xbfd0000000000000, 0x3ff0000000000001, 0x3ff199999999999a, 0x3ff3333333333334, 0x3ff4cccccccccccd];
        assert_eq!(run(1.6), over);
    }

    #[test]
    fn q_ang_past_the_end_is_nan() {
        assert!(q_ang(&[0.0], 0.0, 1.0, &[0.0, 0.0, 0.0, 1.0], 0.0).is_nan());
    }

    #[test]
    fn dip_lane_answers_as_the_typescript() {
        let cases = [
            (0.0, 1.0, 0.0, true),
            (0.3, 1.0, 0.2, true),
            (0.34, 1.0, 0.0, false),
            (1.0, 1.0, 0.7853981633974483, true),
            (0.0, 0.25, 0.0, false),
            (0.0, 1.9, 0.0, false),
            (-1.0, 0.5, -1.5707963267948966, false),
        ];
        for (dx, dz, y, want) in cases {
            assert_eq!(in_dip_lane(dx, dz, y), want);
        }
    }

    #[test]
    fn number_reads_as_the_language_does() {
        for (s, want) in [("7", 7.0), (" 7 ", 7.0), ("0x10", 16.0), ("1e3", 1000.0), (".5", 0.5), ("5.", 5.0), ("+2", 2.0), ("-0.25", -0.25), (" ", 0.0), ("0b11", 3.0), ("0o7", 7.0)] {
            assert_eq!(js_number(s), want);
        }
        for s in ["abc", "1_0", "-0x10", "0x"] {
            assert!(js_number(s).is_nan());
        }
        assert!(!js_number("Infinity").is_finite());
    }

    fn turn_light() -> crate::knobs::TurnK {
        let mut t = crate::knobs::page_turn();
        assert!(set_turn_on(&mut t, "light", 1.0));
        t
    }

    #[test]
    fn set_turn_takes_a_name_a_switch_and_pairs() {
        let mut t = turn_light();
        assert_eq!(feel_name(&t), "light");
        assert!(set_turn_on(&mut t, "heavy", 1.0));
        assert_eq!((t.aim, t.body_acc, t.on), (7.0, 30.0, 1.0));
        assert_eq!(feel_name(&t), "heavy");
        assert!(set_turn_on(&mut t, "0", 1.0));
        assert_eq!((t.on, t.aim), (0.0, 7.0));
        assert!(set_turn_on(&mut t, "1,aim=9", 1.0));
        assert_eq!((t.on, t.aim), (1.0, 9.0));
        assert_eq!(feel_name(&t), "own numbers");
        assert!(set_turn_on(&mut t, ",leadRate=3", 0.0));
        assert_eq!((t.on, t.lead_rate), (0.0, 3.0));
    }

    #[test]
    fn set_turn_says_false_and_takes_the_default_on_a_bad_head() {
        let mut t = turn_light();
        assert!(!set_turn_on(&mut t, "hevy", 0.0));
        assert_eq!(t.on, 0.0);
        assert_eq!(feel_name(&t), "light");
    }

    #[test]
    fn set_turn_refuses_a_bad_pair_and_keeps_going() {
        for bad in ["1,aimm=7", "1,aim=", "1,aim", "1,aim=abc", "1,aim=Infinity"] {
            let mut t = turn_light();
            assert!(!set_turn_on(&mut t, bad, 1.0));
            assert_eq!(t.aim, 11.0);
        }
        let mut t = turn_light();
        assert!(!set_turn_on(&mut t, "1,nope=1,aim=9", 1.0));
        assert_eq!(t.aim, 9.0);
    }

    #[test]
    fn reload_marks_pick_the_gun() {
        assert_eq!(reload_marks(false).mag, [0.085, -0.17, 0.125]);
        assert_eq!(reload_marks(true).mag, [0.085, -0.15, -0.115]);
    }
}
