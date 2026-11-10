//! What a seat is told of its men's throws: from the order until the thing leaves his hand (`squadthrow`'s walks, `squad40`'s turns), and the plan the lit row of its ring
//! stands on, which the menu's own frame keeps. Only read: nothing is worked out here.
use super::*;
use crate::game::{Game, GuardState};
use crate::squadthrow::SquadThrowKind;
use crate::v3;
use alloc::{vec, vec::Vec};

fn line(g: &Game, gd: crate::ids::GuardId, kind: ThrowKind, from: crate::V3, points: &[crate::V3], land: crate::V3, walk: Option<Vec<crate::V3>>, preview: bool) -> ThrowLine {
    let (kill, wound) = if kind == ThrowKind::Stun { (0.0, 0.0) } else { (g.k.frag.lethal_m, g.k.frag.wound_m) };
    let tone = crate::orders::squad_orders_pair_of(g, gd).map(|p| match p { crate::game::PairId::Red => Tone::Red, crate::game::PairId::Blue => Tone::Blue });
    ThrowLine { man: g.ch[g.gd[gd].ch].id, kind, from, points: points.to_vec(), land, walk, preview, kill, wound, tone }
}
/// a 40 mm's shot: he fires it where he stands
fn shell(g: &Game, a: &crate::squad40::Shot, preview: bool) -> ThrowLine {
    let q = g.ch[g.gd[a.gd].ch].pos;
    line(g, a.gd, ThrowKind::Shell, [q[0], 0.0, q[2]], &a.sol.points, a.burst, None, preview)
}
fn kind_word(k: SquadThrowKind) -> ThrowKind { if k == SquadThrowKind::Frag { ThrowKind::Frag } else { ThrowKind::Stun } }

/// the seat's throws, in the order they were given, then the lit row's
pub fn throws_view(g: &Game, seat: &crate::seats::SeatRef) -> Vec<ThrowLine> {
    let mut out = Vec::new();
    let key = crate::seats::view_key(g, seat);
    if let Some(s) = g.m.squadthrow.states.get(key) {
        for gd in s.walks.keys() {
            let w = s.walks.get(gd).unwrap();
            if w.thrown_t >= 0.0 || crate::condition::down(g, g.gd[w.gd].ch) || crate::orders::squad_orders_of(g, seat, w.gd) != w.a || !g.guards.contains(&w.gd) { continue; }
            let p = g.ch[g.gd[w.gd].ch].pos;
            let (mut bi, mut bd) = (0usize, f64::INFINITY);
            for (i, &q) in w.line.iter().enumerate() { let d = v3::dist_xz(q, p); if d < bd { bd = d; bi = i; } }
            let walk = if v3::dist_xz(p, w.post) > 0.4 {
                let mut way = vec![v3::copy(p)];
                if bi + 1 < w.line.len() { way.extend_from_slice(&w.line[bi + 1..]); }
                way.push(w.post);
                Some(way)
            } else { None };
            out.push(line(g, w.gd, kind_word(w.kind), w.post, &w.arc.sol.points, w.arc.rest, walk, false));
        }
        if let Some((men, at, kind)) = crate::squadthrow::lit(g, seat) {
            if let Some(p) = s.plans.get(kind) {
                if let (Some(th), None, true, true) = (&p.thrower, &p.refuse, p.at == at, p.men == men) {
                    let q = g.ch[g.gd[th.gd].ch].pos;
                    out.push(line(g, th.gd, kind_word(kind), th.post.unwrap_or([q[0], 0.0, q[2]]), &th.sol.points, th.rest, p.walk.clone(), true));
                }
            }
        }
    }
    if let Some(s) = g.m.squad40.states.get(key) {
        for k in s.turns.keys() {
            let w = s.turns.get(k).unwrap();
            if w.fired_t < 0.0 && !crate::condition::down(g, g.gd[w.gd].ch) && crate::orders::squad_orders_of(g, seat, w.gd) == w.a { out.push(shell(g, &w.shot, false)); }
        }
        if let (Some((men, at)), Some(p)) = (crate::squad40::lit(g, seat), s.plan.as_ref()) {
            if let (Some(a), None, true) = (&p.shot, &p.no, p.at == at && p.men == men) { out.push(shell(g, a, true)); }
        }
    }
    out
}
