//! Close-up takedowns: which move a blow is, where its hands go, how the man falls, and who breathes out cold. (takedown.ts)
use super::*;                      // your own module: its types, its other parts' functions
use crate::game::Game;
use crate::ids::*;
use crate::js::{self, JsMap, JsSet};
use crate::m;
use crate::v3::{self, Quat, V3};
use alloc::{boxed::Box, string::String, vec, vec::Vec};

/// Whether a half of the takedowns shows: every switch is settled on, only a show turns them off. (takedown.ts live)
pub fn live(g: &mut crate::game::Game, half: crate::takedown::Half) -> bool {
    !g.touring
}

/// How much of the choke's lowering the hold's two poses show. (takedown.ts takeLower)
pub fn take_lower(g: &mut crate::game::Game) -> f64 {
    if live(g, Half::Lower) { 1.0 } else { 0.0 }
}

/// Whether the hold's poses show the kill as a move. (takedown.ts takeKill)
pub fn take_kill(g: &mut crate::game::Game) -> bool {
    live(g, Half::Kill)
}

/// Whether the hold's poses show the choke's beats. (takedown.ts takeBeats)
pub fn take_beats(g: &mut crate::game::Game) -> bool {
    live(g, Half::ChokeBeats)
}

/// Whether the hold's poses show the choke's haul. (takedown.ts takeHaul)
pub fn take_haul(g: &mut crate::game::Game) -> bool {
    live(g, Half::Haul)
}

/// Which move a blow is, by the side it comes from and the hand that does it. (takedown.ts blowStyle)
pub fn blow_style(kind: Option<crate::close::CloseKind>, lethal: bool) -> crate::character::BlowStyle {
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

/// How the man a blow lands on goes down. (takedown.ts fallStyle)
pub fn fall_style(style: crate::character::BlowStyle) -> crate::character::FallStyle {
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

/// The way a body faces, on the floor. (takedown.ts fwd)
pub fn fwd(g: &mut crate::game::Game, c_2: CharId) -> V3 {
    fwd_of(g.ch[c_2].body_yaw)
}

/// The way a yaw faces, on the floor. (takedown.ts fwd)
fn fwd_of(yaw: f64) -> V3 {
    [m::sin(yaw), 0.0, m::cos(yaw)]
}

/// Where on the man each move's hands go, off his bones as they stand: the blow's own point, the free hand's. (takedown.ts marks)
pub fn marks(g: &mut crate::game::Game, by: CharId, vic: CharId, style: crate::character::BlowStyle) -> (V3, Option<V3>) {
    use crate::character::Bone;
    let neck = g.ch[vic].bones.get(Bone::Neck).copied();
    let head = g.ch[vic].bones.get(Bone::Head).copied();
    marks_of(g.ch[by].pos, g.ch[by].body_yaw, g.ch[vic].pos, g.ch[vic].body_yaw, neck, head, style)
}

/// The arithmetic of `marks`, from what it reads off the two men. (takedown.ts marks)
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

/// No headroom over him: a crouched Sam does not rise into the blow; everybody else stands. (takedown.ts mayRise)
pub fn may_rise(g: &mut crate::game::Game, by: CharId) -> bool {
    let p = g.player;
    !(by == g.pl[p].ch && g.pl[p].crouch && !crate::player::can_stand(g))
}

/// Sam's blow begins: his move by the side and the hand, and what the man does before it lands. (takedown.ts takeBegin)
pub fn take_begin(g: &mut crate::game::Game, by: CharId, gd: GuardId, kind: Option<crate::close::CloseKind>, lethal: bool) {
    use crate::character::{BlowLive, BlowOpts, BlowStyle, TakenStyle};
    if !live(g, Half::Blow) { return; }
    let style = blow_style(kind, lethal);
    let vic = g.gd[gd].ch;
    let (at, at2) = marks(g, by, vic, style);
    let opts = BlowOpts { hit: None, end: None, live: Some(BlowLive { by, vic, style }), rise: Some(may_rise(g, by)), tail: Some(true) };
    crate::character::anim_blow(g, by, style, at, at2, Some(&opts));
    g.stats.takedown.blows += 1;
    if !live(g, Half::Victim) { return; }
    if style == BlowStyle::Mouth {
        crate::character::anim_taken(g, vic, TakenStyle::Arch, Some(0.12));
    } else if style == BlowStyle::Chop {
        crate::character::anim_taken(g, vic, TakenStyle::Hunch, Some(SAM_HIT - 0.045));
    } else if style == BlowStyle::Palm {
        crate::character::anim_taken(g, vic, TakenStyle::Snap, Some(SAM_HIT - 0.04));
    }
}

/// How far a fall wants clear floor, by its style and which way: direction, metres. (takedown.ts fallWay)
pub fn fall_way(g: &mut crate::game::Game, vic: CharId, from: Option<V3>, style: crate::character::FallStyle) -> (V3, f64) {
    fall_way_of(g.ch[vic].pos, g.ch[vic].body_yaw, from, style)
}

/// The arithmetic of `fall_way`, from what it reads off the man. (takedown.ts fallWay)
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

/// A fall is asked for: its style, where the man who did it stands, and whether the level has room for it. (takedown.ts setFall)
pub fn set_fall(g: &mut crate::game::Game, vic: CharId, style: crate::character::FallStyle, by: Option<CharId>, ahead: Option<f64>) {
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

/// Is a line along the fall's way, at height `y` and `len` long, stopped by something solid. (takedown.ts setFall's shut)
fn shut(g: &mut Game, p: V3, d: V3, y: f64, len: f64) -> bool {
    crate::scene_collision::solid_blocked(g, [p[0], y, p[2]], [p[0] + d[0] * len, y, p[2] + d[2] * len], None)
}

/// Has the level floor for the whole of him laid `a` further along: three lines, then the strip's volume. (takedown.ts setFall's fits)
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

/// The blow lands, the frame he is put down: how he falls. `how` is a move with no blow of its own. (takedown.ts takeFall)
pub fn take_fall(g: &mut crate::game::Game, gd: GuardId, kind: Option<crate::close::CloseKind>, lethal: bool, how: Option<crate::takedown::TakeFallHow>) {
    use crate::character::FallStyle;
    use crate::close::CloseKind;
    if !live(g, Half::Fall) { return; }
    let led = if how == Some(TakeFallHow::Kill) && !live(g, Half::Kill) { None } else { how };
    let style = if led == Some(TakeFallHow::Choke) {
        FallStyle::Sink
    } else if led == Some(TakeFallHow::Whip) || led == Some(TakeFallHow::Kill) {
        FallStyle::Fold
    } else if kind == Some(CloseKind::Seat) {
        FallStyle::Slump
    } else {
        fall_style(blow_style(kind, lethal))
    };
    let vic = g.gd[gd].ch;
    let by = g.pl[g.player].ch;
    let ahead = if led == Some(TakeFallHow::Choke) { g.k.hold.lay_ahead } else { 0.0 };
    set_fall(g, vic, style, Some(by), Some(ahead));
}

/// The partner's hands go on: the chop from behind, the palm from the front. (takedown.ts partnerBegin)
pub fn partner_begin(g: &mut crate::game::Game, mate: GuardId, foe: GuardId, front: bool) {
    use crate::character::{BlowLive, BlowOpts, BlowStyle, TakenStyle};
    if !live(g, Half::Partner) { return; }
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
    if live(g, Half::Victim) {
        let delay = g.k.takedown.partner_hit;
        crate::character::anim_taken(g, vic, if front { TakenStyle::Snap } else { TakenStyle::Hunch }, Some(delay));
    }
}

/// The partner's hands are called off before the man is out: both moves let go of, eased. (takedown.ts partnerCancel)
pub fn partner_cancel(g: &mut crate::game::Game, mate: GuardId, foe: GuardId) {
    let (by, vic) = (g.gd[mate].ch, g.gd[foe].ch);
    crate::character::anim_blow_cancel(g, by);
    crate::character::anim_taken_cancel(g, vic);
}

/// The man goes down by the partner's move, before their killGuard. (takedown.ts partnerFall)
pub fn partner_fall(g: &mut crate::game::Game, foe: GuardId, front: bool, mate: Option<GuardId>) {
    use crate::character::FallStyle;
    if !live(g, Half::Partner) || !live(g, Half::Fall) { return; }
    let vic = g.gd[foe].ch;
    let by = mate.map(|who| g.gd[who].ch);
    set_fall(g, vic, if front { FallStyle::Topple } else { FallStyle::Fold }, by, None);
}

/// Once a frame: the hold's fallback words, and who breathes: every man and civilian out cold. (takedown.ts takedownTick)
pub fn takedown_tick(g: &mut crate::game::Game) {
    g.m.character.hold_show.kill = live(g, Half::Kill);
    g.m.character.hold_show.beats = live(g, Half::ChokeBeats);
    let on = live(g, Half::Breathe);
    if !on && !g.m.takedown.breathing { return; }
    let mut n: i32 = 0;
    for i in 0..g.guards.len() {
        let gd = g.guards[i];
        breathe_each(g, gd, on, &mut n);
    }
    for i in 0..g.civilians.len() {
        let gd = g.civilians[i];
        breathe_each(g, gd, on, &mut n);
    }
    g.m.takedown.breathing = n != 0;
    g.stats.takedown.breathing = n;
}

/// One man's breath: on while he lies out cold, let go otherwise. (takedown.ts takedownTick's each)
fn breathe_each(g: &mut Game, gd: GuardId, on: bool, n: &mut i32) {
    use crate::game::{DownKind, GuardState};
    let c = g.gd[gd].ch;
    let down_kind = g.gd[gd].down_kind;
    if on
        && g.ch[c].ragdoll.is_some()
        && g.gd[gd].state == GuardState::Dead
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

    const BY_POS: V3 = [1.25, 0.0, -3.5];
    const BY_YAW: f64 = 0.7;
    const VIC_POS: V3 = [2.125, 0.03, -1.75];
    const VIC_YAW: f64 = -2.3;
    const NECK: V3 = [2.1, 1.48, -1.7];
    const HEAD: V3 = [2.12, 1.62, -1.68];

    fn bits(v: V3) -> [u64; 3] { [v[0].to_bits(), v[1].to_bits(), v[2].to_bits()] }

    /// The TypeScript's own bits (bun, EXACT on), for each move with and without bones.
    #[test]
    fn marks_match_the_typescript() {
        type Row = (bool, BlowStyle, [u64; 3], Option<[u64; 3]>);
        let rows: [Row; 8] = [
            (true, BlowStyle::Chop, [0x40008caffb63905a, 0x3ff87ae147ae147b, 0xbffc33a66f3d8f86], Some([0x400133aeea115ee9, 0x3ff47ae147ae147b, 0xbffe1d39e1e6931d])),
            (true, BlowStyle::Palm, [0x40005d0f9e32d4bb, 0x3ff8f5c28f5c28f6, 0xbffc577e4ac889d6], None),
            (true, BlowStyle::Mouth, [0x4000cccccccccccd, 0x3ff7ae147ae147ae, 0xbffb333333333333], Some([0x40006d1870723689, 0x3ff9eb851eb851ec, 0xbffc42406b49c054])),
            (true, BlowStyle::Slash, [0x4000cccccccccccd, 0x3ff851eb851eb852, 0xbffb333333333333], None),
            (false, BlowStyle::Chop, [0x4000bfe32e96c38d, 0x3ff947ae147ae148, 0xbffd00733c0a5c52], Some([0x400166e21d44921c, 0x3ff547ae147ae148, 0xbffeea06aeb35fea])),
            (false, BlowStyle::Palm, [0x4000674d0ed6abc5, 0x3ff970a3d70a3d70, 0xbffd76369cb40ef5], None),
            (false, BlowStyle::Mouth, [0x4001000000000000, 0x3ff87ae147ae147b, 0xbffc000000000000], Some([0x40007755e1160d93, 0x3ffa666666666666, 0xbffd60f8bd354573])),
            (false, BlowStyle::Sweep, [0x4001000000000000, 0x3ff91eb851eb851f, 0xbffc000000000000], None),
        ];
        for (bones, style, a, b) in rows {
            let (neck, head) = if bones { (Some(NECK), Some(HEAD)) } else { (None, None) };
            let (at, at2) = marks_of(BY_POS, BY_YAW, VIC_POS, VIC_YAW, neck, head, style);
            assert_eq!(bits(at), a, "{:?} {}", style, bones);
            assert_eq!(at2.map(bits), b, "{:?} {}", style, bones);
        }
    }

    /// The TypeScript's own bits: away from the man who did it, unless he stands on him; fold and sink always along the victim's facing.
    #[test]
    fn fall_way_matches_the_typescript() {
        let away = [0x3fdc9f25c5bfedd9, 0, 0x3fec9f25c5bfedd9];
        let facing = [0xbfe7dcd12d582f24, 0, 0xbfe5522217302fe0];
        let (far, near): (Option<V3>, Option<V3>) = (Some(BY_POS), Some([2.13, 0.0, -1.76]));
        let rows: [(Option<V3>, FallStyle, [u64; 3], f64); 5] = [
            (far, FallStyle::Topple, away, 1.5),
            (far, FallStyle::Cut, away, 0.9),
            (far, FallStyle::Fold, facing, 0.7),
            (near, FallStyle::Topple, facing, 1.5),
            (None, FallStyle::Cut, facing, 0.9),
        ];
        for (from, style, d, len) in rows {
            let (got, reach) = fall_way_of(VIC_POS, VIC_YAW, from, style);
            assert_eq!(bits(got), d, "{:?}", style);
            assert_eq!(reach.to_bits(), f64::to_bits(len));
        }
    }

    fn man(g: &mut Game, state: crate::game::GuardState, down: Option<crate::game::DownKind>, flat: bool) -> (GuardId, CharId) {
        let c = g.ch.push(crate::character::Character::blank());
        if flat { g.ch[c].ragdoll = Some(RagdollId(0)); }
        let mut row = crate::game::Guard::blank();
        row.ch = c;
        row.state = state;
        row.down_kind = down;
        (g.gd.push(row), c)
    }

    /// Who breathes: out cold and flat, guard or civilian; not the dead, not one still falling. A show lets every one go, once, and then costs nothing.
    #[test]
    fn the_out_cold_breathe_and_a_show_lets_them_go() {
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
        assert!(g.m.takedown.breathing && g.m.character.hold_show.kill && g.m.character.hold_show.beats);

        g.touring = true;
        takedown_tick(&mut g);
        assert!(g.ch[c_struck].breathe.is_none() && g.ch[c_choked].breathe.is_none());
        assert_eq!(g.stats.takedown.breathing, 0);
        assert!(!g.m.takedown.breathing && !g.m.character.hold_show.kill && !g.m.character.hold_show.beats);

        // Nobody breathing and the switch off: the walk is not taken, so a breath set by hand stays.
        g.ch[c_struck].breathe = Some(crate::character::CharacterBreathe { breath_hz: 1.0, breath_rise: 1.0, breath_in: 1.0 });
        takedown_tick(&mut g);
        assert!(g.ch[c_struck].breathe.is_some());
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
