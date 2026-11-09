//! WHAT A SEAT'S HUD IS TOLD OF ITS OWN KEY (`view::tick_view`, `interact`): the rows the panel lists, which one is lit and the things they are about. The rows are the tick's
//! (`seat.nearby`), read and worded here, and asking writes nothing.
use crate::game::{Game, Interactable, InteractableKind as K};
use crate::seats::SeatRef;
use crate::view::{InteractView, ListRow, RowOpt, ThingDot, ThingKind, ThingRing};
use alloc::{format, string::String, vec::Vec};

/// the blocks the list draws before it says '+N more': the rows a tick builds, the cancel row while the selector is open, and one spare
const MAX_ROWS: usize = 5;

/// up to the first run of two spaces with something on each side: the key, and what it does
fn key_and_words(seg: &str) -> Option<(&str, &str)> {
    let cs: Vec<(usize, char)> = seg.char_indices().collect();
    let mut k = 1;
    while k < cs.len() {
        if cs[k].1.is_whitespace() && !cs[k - 1].1.is_whitespace() {
            let mut e = k;
            while e < cs.len() && cs[e].1.is_whitespace() { e += 1; }
            if e - k >= 2 && e < cs.len() { return Some((&seg[..cs[k].0], &seg[cs[e].0..])); }
            k = e;
        } else { k += 1; }
    }
    None
}
/// A row's second line is authored as 'key  action · key  action · hint': split back into its lines. 'F or sprint into it  kick…' keeps F as the key and puts the other way on a hint line under it
pub fn row_opts(line2: &str) -> Vec<RowOpt> {
    let mut out = Vec::new();
    let cs: Vec<(usize, char)> = line2.char_indices().collect();
    let mut segs: Vec<&str> = Vec::new();
    let (mut from, mut k) = (0, 0);
    while k < cs.len() {
        // (a dot with space on each side parts two lines)
        if cs[k].1 == '·' && k > 0 && cs[k - 1].1.is_whitespace() && cs.get(k + 1).is_some_and(|x| x.1.is_whitespace()) {
            segs.push(&line2[from..cs[k].0]);
            from = cs[k + 1].0;
        }
        k += 1;
    }
    segs.push(&line2[from..]);
    for raw in segs {
        let seg = raw.trim();
        if seg.is_empty() { continue; }
        let Some((key, words)) = key_and_words(seg) else { out.push(RowOpt { key: String::new(), text: String::from(seg) }); continue; };
        let other = key.find(" or ").filter(|&i| i > 0 && !key[..i].contains(char::is_whitespace) && key.len() > i + 4);
        out.push(RowOpt { key: String::from(other.map_or(key, |i| &key[..i])), text: String::from(words.trim()) });
        if let Some(i) = other { out.push(RowOpt { key: String::new(), text: format!("or {}", &key[i + 4..]) }); }
    }
    out
}

/// the kind in the view's word
pub fn thing_kind(k: K) -> ThingKind {
    match k {
        K::Cam => ThingKind::Cam, K::Guard => ThingKind::Guard, K::Breaker => ThingKind::Breaker, K::Door => ThingKind::Door, K::Light => ThingKind::Light, K::Body => ThingKind::Body,
        K::Objective => ThingKind::Objective, K::Held => ThingKind::Held, K::Wall => ThingKind::Wall, K::Switch => ThingKind::Switch, K::Charge => ThingKind::Charge, K::Cancel => ThingKind::Cancel,
    }
}
/// the man a thing rides, and how far over his feet: a body stays where it fell, as the tick has it
fn rides(g: &Game, it: &Interactable) -> (Option<f64>, f64) {
    match it.guard {
        Some(gd) if it.kind != K::Body => { let c = &g.ch[g.gd[gd].ch]; (Some(c.id), it.pos[1] - c.pos[1]) }
        _ => (None, 0.0),
    }
}

/// the list for `seat`. With the ring standing for the list the corner shows one line, what the key would do now (the highlighted row, or a running verb's own; none while the
/// ring's lit wedge is the whistle), and under it the first line no key gives; a puppet keeps the whole list
pub fn interact_view(g: &Game, seat: &SeatRef) -> InteractView {
    let pl = &g.pl[seat.player];
    let (near, p) = (seat.nearby, g.ch[pl.ch].pos);
    let hv = seat.hover.as_ref().map(|_| pl.near_sel as usize).filter(|&i| i < near.len());
    let lead = hv.filter(|&i| { let r = &near[i]; r.kind != K::Cancel && (r.verb.is_some() || r.running == Some(true)) && !pl.sel.ring.as_ref().is_some_and(|x| x.whistle == Some(true)) });
    let whole = !g.k.handring.on || g.puppet.is_some();
    let list: Vec<usize> = if whole { (0..near.len()).collect() } else {
        lead.into_iter().chain((0..near.len()).filter(|&i| near[i].verb.is_none() && Some(i) != lead).take(1)).collect()
    };
    let cap = MAX_ROWS + if list.iter().any(|&i| near[i].kind == K::Cam) { 1 } else { 0 };   // (a stuck camera's 'take it back' is one row past every row the list had before it: it is always drawn)
    let rows = list.iter().take(cap).enumerate().map(|(at, &i)| {
        let it = &near[i];
        ListRow {
            kind: thing_kind(it.kind),
            label: it.line1.clone(),
            opts: if it.kind == K::Cancel { Vec::new() } else { row_opts(&it.line2) },
            in_reach: it.in_reach,
            off: it.off,
            dist: if it.in_reach || it.kind == K::Light || it.kind == K::Cancel { None } else { Some(crate::player::thing_dist(g, it, p)) },
            progress: it.progress,
            cont: at > 0 && it.kind != K::Cancel && { let up = &near[list[at - 1]]; up.kind == it.kind && crate::player::who_of(g, up) == crate::player::who_of(g, it) },
        }
    }).collect();
    // (under the corner only the key's own line is ever lit: a line no key gives is no choice)
    let lit = if whole { hv.and_then(|i| list.iter().position(|&x| x == i)) } else if lead.is_some() { Some(0) } else { None };
    // (the cancel row has no thing, and a charge to blow is on his list wherever he stands: the ring stays the gun's)
    let ring = seat.hover.as_ref().filter(|h| h.kind != K::Cancel && h.kind != K::Charge).map(|h| {
        let (man, up) = rides(g, h);
        ThingRing { kind: thing_kind(h.kind), at: h.pos, man, up, off: h.off, in_reach: h.in_reach, progress: h.progress }
    });
    // (a living man carries his state's mark: no dot)
    let dots = seat.interactables.iter().filter(|it| it.kind != K::Guard).map(|it| {
        let (man, up) = rides(g, it);
        ThingDot {
            kind: thing_kind(it.kind), at: it.pos, man, up, off: it.off,
            hidden: it.guard.is_some_and(|gd| crate::fogofwar::fog_of_war_hides(g, gd)),
            ringed: ring.as_ref().is_some_and(|r| r.at == it.pos),
        }
    }).collect();
    InteractView {
        rows,
        lit: lit.filter(|&i| i < cap).map_or(-1.0, |i| i as f64),
        more: list.len().saturating_sub(cap) as f64,
        open: pl.sel.open && pl.sel.ring.is_none(),
        corner: g.puppet.is_none(),
        ring,
        dots,
    }
}
