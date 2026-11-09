//! What a seat is told of its two rings and of a tap with no row. The squad's ring is the menu's record: its rows, each row's why and words, the quick word and the lit wedge are the
//! tick's (`orders/words.rs`), and which row is on which digit is the tick's own function of that record (`squad_orders_seats`).
use super::*;
use crate::game::{Game, InteractableKind};
use crate::seats::SeatRef;
use crate::orders::OrderTarget;
use alloc::{format, string::String, vec::Vec};

fn said(s: &Option<String>) -> bool { s.as_ref().is_some_and(|x| !x.is_empty()) }
pub(crate) fn tone_word(t: crate::orders::Tone) -> Tone {
    match t { crate::orders::Tone::Red => Tone::Red, crate::orders::Tone::Blue => Tone::Blue, crate::orders::Tone::Gold => Tone::Gold }
}

/// the squad's ring, while it is up
pub(super) fn ring_view(g: &Game, seat: &SeatRef) -> Option<RingView> {
    let (r, m) = (seat.orders.cmd_ring?, seat.orders.menu.as_ref()?);
    if m.words.len() != m.rows.len() { return None; }   // (its rows have not been worded yet: a frame's input does it)
    let more = r.more.unwrap_or(false);
    let seats = crate::orders::squad_orders_seats(g, m, Some(r));
    let mut wedges = Vec::new();
    for d in 1..=g.k.cmdring.wedges as usize {
        let i = seats.row.get(d).copied().unwrap_or(-1.0);
        if d as f64 == seats.more {
            wedges.push(RingWedge { key: d as f64, text: String::from("more"), off: false, opens: true });
        } else if m.page.is_none() && !more && d == 1 {
            if said(&m.quick) { wedges.push(RingWedge { key: 1.0, text: m.quick.clone().unwrap(), off: false, opens: false }); }
        } else if i >= 0.0 && (i as usize) < m.rows.len() {
            let at = i as usize;
            wedges.push(RingWedge { key: d as f64, text: m.words[at].clone(), off: m.why.get(at).is_some_and(said), opens: said(&g.tab.orders_order_def[m.rows[at]].opens) });
        }
    }
    let what = match &m.target {
        OrderTarget::None(n) => n.man.map_or(String::new(), |man| g.gd[man].callsign.clone()),
        t => crate::orders::squad_orders_hover_words(g, Some(t)),
    };
    let who = crate::orders::squad_orders_who_tag(g, seat).tag;
    let caption = if !what.is_empty() && what != who { format!("{} \u{2192} {}", who, what) } else { who };
    let pin = r.at.map(|at| RingPin { at, ox: r.ox.unwrap_or(0.0), oy: r.oy.unwrap_or(0.0), edge_x: g.k.cmdring.edge_x * r.z, edge_y: g.k.cmdring.edge_y * r.z });
    Some(RingView { x: r.ax, y: r.ay, zoom: r.z, lit: r.lit, tone: crate::orders::squad_orders_tone_of(g, seat, &m.men).map(tone_word), caption, wedges, pin })
}

/// a row's words as his ring has them: the first thing a key does on it (`row_opts`, the panel's own reading), less what follows a dash
fn row_words(line2: &str) -> String {
    let opts = crate::interact::row_opts(line2);
    let Some(o) = opts.iter().find(|o| !o.key.is_empty()).or(opts.first()) else { return String::new() };
    String::from(before_dash(&o.text).trim())
}
/// up to the first dash standing between spaces
fn before_dash(s: &str) -> &str {
    let cs: Vec<(usize, char)> = s.char_indices().collect();
    for (k, &(i, c)) in cs.iter().enumerate() {
        if k > 0 && cs[k - 1].1.is_whitespace() && matches!(c, '—' | '–' | '-') && cs.get(k + 1).is_some_and(|x| x.1.is_whitespace()) { return s[..i].trim_end(); }
    }
    s
}

/// his own ring, while it is up: a wedge a row on a digit, and the whistle's the last one
pub(super) fn hand_ring_view(g: &Game, seat: &SeatRef) -> Option<HandRingView> {
    let pl = &g.pl[seat.player];
    let r = pl.sel.ring.as_ref()?;
    if !pl.sel.open { return None; }
    let n = g.k.cmdring.wedges;
    let mut wedges = Vec::new();
    for (i, it) in seat.nearby.iter().take(n as usize).enumerate() {
        if !crate::interact::wedged(it) { continue; }
        // (how a switch or the mains STAND is no reason its row cannot be given)
        let stands = matches!(it.kind, InteractableKind::Switch | InteractableKind::Breaker);
        wedges.push(RingWedge { key: (i + 1) as f64, text: row_words(&it.line2), off: (it.off && !stands) || !it.in_reach, opens: false });
    }
    if crate::interact::whistle_wedge(g, seat) {
        wedges.push(RingWedge { key: n, text: String::from("whistle"), off: crate::whistle::whistle_ready_in_of(g, seat) > 0.0, opens: false });
    }
    let lit = if r.whistle.unwrap_or(false) { n }
        else if seat.hover.as_ref().is_some_and(crate::interact::wedged) && (pl.near_sel as f64) < n { pl.near_sel as f64 + 1.0 }
        else { 0.0 };
    let pin = r.at.map(|at| RingPin { at, ox: r.ox.unwrap_or(0.0), oy: r.oy.unwrap_or(0.0), edge_x: g.k.cmdring.edge_x * r.z, edge_y: g.k.cmdring.edge_y * r.z });
    Some(HandRingView { x: r.ax, y: r.ay, zoom: r.z, lit, wedges, pin })
}

/// the word of a tap that found no row, for as long as it is up and is still what a tap would say: a step into reach, or away, takes it down
pub(super) fn empty_view(g: &Game, seat: &SeatRef) -> Option<EmptyView> {
    let pl = &g.pl[seat.player];
    if g.puppet.is_some() || pl.down || pl.sel.word.is_empty() || !seat.nearby.is_empty() { return None; }
    let age = g.time - pl.sel.word_t;
    if !(age >= 0.0 && age < crate::handring::EMPTY_SHOW_SECS) { return None; }
    let t = crate::interact::empty_thing(g, seat);
    if crate::interact::empty_word(t.as_ref()) != pl.sel.word { return None; }
    Some(match t {
        Some(t) => { let far = t.why == crate::handring::empty_why().far.as_str(); EmptyView { word: pl.sel.word.clone(), kind: crate::interact::thing_kind(t.it.kind), dist: if far { Some(t.d) } else { None } } }
        None => EmptyView { word: pl.sel.word.clone(), kind: ThingKind::Wall, dist: None },
    })
}
