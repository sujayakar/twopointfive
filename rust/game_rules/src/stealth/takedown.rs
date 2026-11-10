//! Close-up takedowns: which move a blow is, where its hands go, how the man falls, and who breathes out cold.

pub use ::game_types::takedown::types;
pub use types::*;
use crate::game::Game;
use crate::ids::*;
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{vec, vec::Vec};

/// Which move a blow is, by the side it comes from and the hand that does it.
fn blow_style(kind: Option<crate::close::CloseKind>, lethal: bool) -> crate::character::BlowStyle {
    use crate::character::BlowStyle;
    use crate::close::CloseKind;
    let rear = kind.is_none() || kind == Some(CloseKind::Behind) || kind == Some(CloseKind::Seat);
    if lethal {
        if kind == Some(CloseKind::Front) { BlowStyle::Slash } else if rear { BlowStyle::Mouth } else { BlowStyle::Sweep }
    } else if rear {
        BlowStyle::Chop
    } else {
        BlowStyle::Palm
    }
}

/// How the man a blow lands on goes down.
fn fall_style(style: crate::character::BlowStyle) -> crate::character::FallStyle {
    use crate::character::{BlowStyle, FallStyle};
    if style == BlowStyle::Chop {
        FallStyle::Fold
    } else if style == BlowStyle::Palm {
        FallStyle::Topple
    } else if style == BlowStyle::Mouth {
        FallStyle::Sink
    } else {
        FallStyle::Cut
    }
}

/// The way a yaw faces, on the floor.
fn fwd_of(yaw: f64) -> V3 {
    [m::sin(yaw), 0.0, m::cos(yaw)]
}

/// Where on the man each move's hands go, off his bones as they stand: the blow's own point, the free hand's.
pub fn marks(g: &mut crate::game::Game, by: CharId, vic: CharId, style: crate::character::BlowStyle) -> (V3, Option<V3>) {
    use crate::character::Bone;
    let neck = g.ch[vic].bones.get(Bone::Neck).copied();
    let head = g.ch[vic].bones.get(Bone::Head).copied();
    marks_of(g.ch[by].pos, g.ch[by].body_yaw, g.ch[vic].pos, g.ch[vic].body_yaw, neck, head, style)
}

/// The arithmetic of `marks`, from what it reads off the two men.
fn marks_of(by_pos: V3, by_yaw: f64, vic_pos: V3, vic_yaw: f64, neck: Option<V3>, head: Option<V3>, style: crate::character::BlowStyle) -> (V3, Option<V3>) {
    use crate::character::BlowStyle;
    let f = fwd_of(vic_yaw);
    let neck = neck.unwrap_or_else(|| v3::add(vic_pos, [0.0, 1.5, 0.0]));
    let head = head.unwrap_or_else(|| v3::add(neck, [0.0, 0.12, 0.0]));
    let to = v3::normalize([by_pos[0] - vic_pos[0], 0.0, by_pos[2] - vic_pos[2] + 1e-6]);
    let left: V3 = [m::cos(by_yaw), 0.0, -m::sin(by_yaw)];
    if style == BlowStyle::Chop {
        return (
            v3::add(v3::mad(neck, to, 0.07), [0.0, 0.05, 0.0]),
            Some(v3::add(v3::mad(v3::mad(neck, to, 0.11), left, 0.13), [0.0, -0.2, 0.0])),
        );
    }
    if style == BlowStyle::Palm {
        return (v3::add(v3::mad(v3::mad(head, f, 0.07), to, 0.05), [0.0, -0.06, 0.0]), None);
    }
    if style == BlowStyle::Mouth {
        return (v3::copy(neck), Some(v3::add(v3::mad(v3::mad(head, f, 0.11), left, 0.02), [0.0, 0.0, 0.0])));
    }
    (v3::add(neck, [0.0, 0.04, 0.0]), None)
}

/// No headroom over him: a crouched Sam does not rise into the blow; everybody else stands.
fn may_rise(g: &mut crate::game::Game, by: CharId) -> bool {
    let p = g.player;
    !(by == g.pl[p].ch && g.pl[p].crouch && !crate::player::can_stand(g))
}

/// Sam's blow begins: his move by the side and the hand, and what the man does before it lands.
pub fn take_begin(g: &mut crate::game::Game, by: CharId, gd: GuardId, kind: Option<crate::close::CloseKind>, lethal: bool) {
    use crate::character::{BlowLive, BlowOpts, BlowStyle, TakenStyle};
    let style = blow_style(kind, lethal);
    let vic = g.gd[gd].ch;
    let (at, at2) = marks(g, by, vic, style);
    let opts = BlowOpts { hit: None, end: None, live: Some(BlowLive { by, vic, style }), rise: Some(may_rise(g, by)), tail: Some(true) };
    crate::character::anim_blow(g, by, style, at, at2, Some(&opts));
    if style == BlowStyle::Mouth {
        crate::character::anim_taken(g, vic, TakenStyle::Arch, Some(0.12));
    } else if style == BlowStyle::Chop {
        crate::character::anim_taken(g, vic, TakenStyle::Hunch, Some(SAM_HIT - 0.045));
    } else if style == BlowStyle::Palm {
        crate::character::anim_taken(g, vic, TakenStyle::Snap, Some(SAM_HIT - 0.04));
    }
}

/// How far a fall wants clear floor, by its style and which way: direction, metres.
fn fall_way(g: &crate::game::Game, vic: CharId, from: Option<V3>, style: crate::character::FallStyle) -> (V3, f64) {
    fall_way_of(g.ch[vic].pos, g.ch[vic].body_yaw, from, style)
}

/// The arithmetic of `fall_way`, from what it reads off the man.
fn fall_way_of(p: V3, yaw: f64, from: Option<V3>, style: crate::character::FallStyle) -> (V3, f64) {
    use crate::character::FallStyle;
    let f = fwd_of(yaw);
    let away: V3 = match from {
        Some(from) if m::hyp2(p[0] - from[0], p[2] - from[2]) > 0.05 => v3::normalize([p[0] - from[0], 0.0, p[2] - from[2]]),
        _ => f,
    };
    if style == FallStyle::Topple {
        (away, 1.5)
    } else if style == FallStyle::Cut {
        (away, 0.9)
    } else {
        (f, 0.7)
    }
}

/// A fall is asked for: its style, where the man who did it stands, and whether the level has room for it.
fn set_fall(g: &mut crate::game::Game, vic: CharId, style: crate::character::FallStyle, by: Option<CharId>, ahead: Option<f64>) {
    use crate::character::FallStyle;
    let ahead = ahead.unwrap_or(0.0);
    let from = by.map(|b| v3::copy(g.ch[b].pos));
    let (d, len_m) = fall_way(g, vic, from, style);
    let p = g.ch[vic].pos;
    let tight = style != FallStyle::Slump && (shut(g, p, d, 0.45, len_m) || shut(g, p, d, 1.1, len_m));
    g.ch[vic].fall_as = Some(if tight && style == FallStyle::Topple { FallStyle::Sink } else { style });
    g.ch[vic].fall_from = from;
    g.ch[vic].fall_tight = tight;
    g.stats.takedown.falls += 1;
    let mut touch: Vec<i32> = vec![0; 4];
    let fall_ahead = if ahead > 0.0 && style == FallStyle::Sink && !tight {
        if fits(g, p, d, ahead, &mut touch) {
            ahead
        } else if fits(g, p, d, ahead / 2.0, &mut touch) {
            ahead / 2.0
        } else {
            0.0
        }
    } else {
        0.0
    };
    g.ch[vic].fall_ahead = fall_ahead;
}

/// Is a line along the fall's way, at height `y` and `len` long, stopped by something solid.
fn shut(g: &mut Game, p: V3, d: V3, y: f64, len: f64) -> bool {
    crate::scene_collision::solid_blocked(g, [p[0], y, p[2]], [p[0] + d[0] * len, y, p[2] + d[2] * len], None)
}

/// Has the level floor for the whole of him laid `a` further along: three lines, then the strip's volume.
fn fits(g: &mut Game, p: V3, d: V3, a: f64, touch: &mut Vec<i32>) -> bool {
    let len = a + LAY_REACH;
    if shut(g, p, d, 0.15, len) || shut(g, p, d, 0.45, len) || shut(g, p, d, 1.1, len) { return false; }
    let mut s = LAY_HALF;
    loop {
        let t = m::min(s, len - LAY_HALF);
        if crate::scene_collision::circle_touches(g, p[0] + d[0] * t, p[2] + d[2] * t, LAY_HALF + 0.05, 0.1, 1.2, touch) != 0.0 { return false; }
        if t >= len - LAY_HALF { return true; }
        s += 0.2;
    }
}

/// The blow lands, the frame he is put down: how he falls. `how` is a move with no blow of its own.
pub fn take_fall(g: &mut crate::game::Game, gd: GuardId, kind: Option<crate::close::CloseKind>, lethal: bool, how: Option<crate::takedown::TakeFallHow>) {
    use crate::character::FallStyle;
    use crate::close::CloseKind;
    let style = if how == Some(TakeFallHow::Choke) {
        FallStyle::Sink
    } else if how == Some(TakeFallHow::Whip) {
        FallStyle::Fold
    } else if kind == Some(CloseKind::Seat) {
        FallStyle::Slump
    } else {
        fall_style(blow_style(kind, lethal))
    };
    let vic = g.gd[gd].ch;
    let by = g.pl[g.player].ch;
    let ahead = if how == Some(TakeFallHow::Choke) { g.k.hold.lay_ahead } else { 0.0 };
    set_fall(g, vic, style, Some(by), Some(ahead));
}

/// The partner's hands go on: the chop from behind, the palm from the front.
pub fn partner_begin(g: &mut crate::game::Game, mate: GuardId, foe: GuardId, front: bool) {
    use crate::character::{BlowLive, BlowOpts, BlowStyle, TakenStyle};
    let style = if front { BlowStyle::Palm } else { BlowStyle::Chop };
    let (by, vic) = (g.gd[mate].ch, g.gd[foe].ch);
    let (at, at2) = marks(g, by, vic, style);
    let opts = BlowOpts {
        hit: Some(g.k.takedown.partner_hit),
        end: Some(m::min(g.k.takedown.partner_end, PARTNER_SECS)),
        live: Some(BlowLive { by, vic, style }),
        rise: None,
        tail: None,
    };
    crate::character::anim_blow(g, by, style, at, at2, Some(&opts));
    g.stats.takedown.partner += 1;
    let delay = g.k.takedown.partner_hit;
    crate::character::anim_taken(g, vic, if front { TakenStyle::Snap } else { TakenStyle::Hunch }, Some(delay));
}

/// The partner's hands are called off before the man is out: both moves let go of, eased.
pub fn partner_cancel(g: &mut crate::game::Game, mate: GuardId, foe: GuardId) {
    let (by, vic) = (g.gd[mate].ch, g.gd[foe].ch);
    crate::character::anim_blow_cancel(g, by);
    crate::character::anim_taken_cancel(g, vic);
}

/// The man goes down by the partner's move, before their killGuard.
pub fn partner_fall(g: &mut crate::game::Game, foe: GuardId, front: bool, mate: Option<GuardId>) {
    use crate::character::FallStyle;
    let vic = g.gd[foe].ch;
    let by = mate.map(|who| g.gd[who].ch);
    set_fall(g, vic, if front { FallStyle::Topple } else { FallStyle::Fold }, by, None);
}

/// Once a frame: the hold's fallback words, and who breathes: every man and civilian out cold.
pub fn takedown_tick(g: &mut crate::game::Game) {
    g.m.character.hold_show.beats = true;
    let mut n: i32 = 0;
    for i in 0..g.guards.len() {
        let gd = g.guards[i];
        breathe_each(g, gd, &mut n);
    }
    for i in 0..g.civilians.len() {
        let gd = g.civilians[i];
        breathe_each(g, gd, &mut n);
    }
    g.stats.takedown.breathing = n;
}

/// One man's breath: on while he lies out cold, let go otherwise.
fn breathe_each(g: &mut Game, gd: GuardId, n: &mut i32) {
    use crate::game::{DownKind, GuardState};
    let c = g.gd[gd].ch;
    let down_kind = g.gd[gd].down_kind;
    if g.ch[c].ragdoll.is_some()
        && crate::condition::down(g, g.gd[gd].ch)
        && (down_kind == Some(DownKind::Struck) || down_kind == Some(DownKind::Choked))
    {
        let k = &g.k.takedown;
        let breath = crate::character::CharacterBreathe { breath_hz: k.breath_hz, breath_rise: k.breath_rise, breath_in: k.breath_in };
        g.ch[c].breathe = Some(breath);
        *n += 1;
    } else if g.ch[c].breathe.is_some() {
        g.ch[c].breathe = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::character::{BlowStyle, FallStyle};
    use crate::close::CloseKind;

    const BY_POS: V3 = [0.0, 0.0, -1.0];
    const VIC_POS: V3 = [0.0, 0.0, 0.0];
    const NECK: V3 = [0.0, 1.48, 0.0];
    const HEAD: V3 = [0.0, 1.62, 0.0];

    // He faces +z with Sam a metre behind him, Sam facing him (Sam's left is +x).
    fn marks_here(style: BlowStyle) -> (V3, Option<V3>) { marks_of(BY_POS, 0.0, VIC_POS, 0.0, Some(NECK), Some(HEAD), style) }

    /// The chop lands on the back of the neck from over it, Sam's other hand low on the man's back, off to Sam's left.
    #[test]
    fn a_chop_lands_on_the_back_of_the_neck_and_the_other_hand_takes_his_back() {
        let (at, other) = marks_here(BlowStyle::Chop);
        assert!(at[2] < NECK[2] && at[1] > NECK[1], "{:?}", at);
        let other = other.unwrap();
        assert!(other[2] < at[2] && other[1] < NECK[1] && other[0] > 0.0, "{:?}", other);
    }

    /// The palm goes to his face from the front, in by the head, with no second hand. It is what a blow from the side or the front is.
    #[test]
    fn a_palm_lands_on_his_face_with_one_hand() {
        let (at, other) = marks_of([0.0, 0.0, 1.0], m::PI, VIC_POS, 0.0, Some(NECK), Some(HEAD), BlowStyle::Palm);
        assert!(at[2] > HEAD[2] && at[1] < HEAD[1] && at[1] > NECK[1], "{:?}", at);
        assert!(other.is_none());
    }

    /// The knife from behind is at his throat, the free hand round the front of his head to the mouth.
    #[test]
    fn the_knife_from_behind_takes_the_throat_and_the_mouth() {
        let (at, other) = marks_here(BlowStyle::Mouth);
        assert_eq!(at, NECK);
        let other = other.unwrap();
        assert!(other[2] > HEAD[2] && other[1] == HEAD[1], "{:?}", other);
    }

    /// A slash or a sweep is one hand, a little over the neck.
    #[test]
    fn a_slash_or_a_sweep_is_one_hand_at_the_neck() {
        for style in [BlowStyle::Slash, BlowStyle::Sweep] {
            let (at, other) = marks_here(style);
            assert_eq!((at[0], at[2]), (NECK[0], NECK[2]));
            assert!(at[1] > NECK[1] && at[1] < HEAD[1]);
            assert!(other.is_none());
        }
    }

    /// Before he has been posed, his neck is taken as a man's height over his feet and his head just above that, so no blow goes to the floor.
    #[test]
    fn a_man_with_no_bones_yet_has_a_neck_and_a_head_where_a_man_has_them() {
        let (at, _) = marks_of(BY_POS, 0.0, [2.0, 0.5, 3.0], 0.0, None, None, BlowStyle::Mouth);
        assert_eq!(at, [2.0, 2.0, 3.0]);
        let (_, other) = marks_of(BY_POS, 0.0, [2.0, 0.5, 3.0], 0.0, None, None, BlowStyle::Mouth);
        assert_eq!(other.unwrap()[1], 2.12);
    }

    /// A shove or a cut falls him away from the man who did it, further for a topple. A fold or a sink goes along his own facing, and so does any fall with nobody there or from a man right on top of him.
    #[test]
    fn he_falls_away_from_the_man_who_did_it_unless_he_folds() {
        let (vic, yaw): (V3, f64) = ([2.0, 0.03, -1.0], 0.5);
        let facing = [m::sin(yaw), 0.0, m::cos(yaw)];
        let by: V3 = [2.0, 0.0, -4.0];
        let away = [0.0, 0.0, 1.0];
        assert_eq!(fall_way_of(vic, yaw, Some(by), FallStyle::Topple), (away, 1.5));
        assert_eq!(fall_way_of(vic, yaw, Some(by), FallStyle::Cut), (away, 0.9));
        assert_eq!(fall_way_of(vic, yaw, Some(by), FallStyle::Fold), (facing, 0.7));
        assert_eq!(fall_way_of(vic, yaw, Some(by), FallStyle::Sink), (facing, 0.7));
        assert_eq!(fall_way_of(vic, yaw, Some([2.01, 0.0, -1.01]), FallStyle::Topple), (facing, 1.5));
        assert_eq!(fall_way_of(vic, yaw, None, FallStyle::Cut), (facing, 0.9));
    }

    fn man(g: &mut Game, state: crate::game::GuardState, down: Option<crate::game::DownKind>, flat: bool) -> (GuardId, CharId) {
        let c = crate::character::man_new(g, crate::character::Character::blank());
        if flat { g.ch[c].ragdoll = Some(RagdollId(0)); }
        let mut row = crate::game::Guard::blank();
        row.ch = c;
        row.state = state;
        g.cond[c].down = state == crate::game::GuardState::Dead;
        row.down_kind = down;
        (g.gd.push(row), c)
    }

    /// Who breathes: out cold and flat, guard or civilian; not the dead, not one still falling.
    #[test]
    fn the_out_cold_breathe() {
        use crate::game::{DownKind, GuardState};
        let mut g = Game::blank();
        let (struck, c_struck) = man(&mut g, GuardState::Dead, Some(DownKind::Struck), true);
        let (choked, c_choked) = man(&mut g, GuardState::Dead, Some(DownKind::Choked), true);
        let (falling, c_falling) = man(&mut g, GuardState::Dead, Some(DownKind::Struck), false);
        let (killed, c_killed) = man(&mut g, GuardState::Dead, None, true);
        g.guards = vec![struck, falling, killed];
        g.civilians = vec![choked];
        g.ch[c_killed].breathe = Some(crate::character::CharacterBreathe { breath_hz: 1.0, breath_rise: 1.0, breath_in: 1.0 });
        takedown_tick(&mut g);
        assert!(g.ch[c_struck].breathe.is_some() && g.ch[c_choked].breathe.is_some());
        assert!(g.ch[c_falling].breathe.is_none() && g.ch[c_killed].breathe.is_none());
        assert_eq!(g.ch[c_struck].breathe.unwrap().breath_hz.to_bits(), g.k.takedown.breath_hz.to_bits());
        assert_eq!(g.stats.takedown.breathing, 2);
        assert!(g.m.character.hold_show.beats);
    }

    #[test]
    fn the_move_follows_side_and_hand_and_the_fall_follows_the_move() {
        let cases = [
            (None, false, BlowStyle::Chop, FallStyle::Fold),
            (Some(CloseKind::Behind), false, BlowStyle::Chop, FallStyle::Fold),
            (Some(CloseKind::Seat), false, BlowStyle::Chop, FallStyle::Fold),
            (Some(CloseKind::Side), false, BlowStyle::Palm, FallStyle::Topple),
            (Some(CloseKind::Front), false, BlowStyle::Palm, FallStyle::Topple),
            (None, true, BlowStyle::Mouth, FallStyle::Sink),
            (Some(CloseKind::Seat), true, BlowStyle::Mouth, FallStyle::Sink),
            (Some(CloseKind::Front), true, BlowStyle::Slash, FallStyle::Cut),
            (Some(CloseKind::Side), true, BlowStyle::Sweep, FallStyle::Cut),
        ];
        for (kind, lethal, blow, fall) in cases {
            assert_eq!(blow_style(kind, lethal), blow);
            assert_eq!(fall_style(blow), fall);
        }
    }
}
