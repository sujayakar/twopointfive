//! The numbers of needs.
use crate::bytes::ToBytes;
use crate::js::*;
use game_derive::Knobs;

/// ================================================================ numbers
#[derive(Clone, Debug, ToBytes, Knobs)]
pub struct NeedsK {
    /// × every man's fill rate (1 = as dealt: a coffee fills in 6–9 min of calm patrol, a smoke in 8–12, water in 6–10, the stretch in 5–7)
    #[k(1.0)] pub rate_k: f64,
    /// men off their routes at once, floor-wide (telegraphing counts as off: the slot is his from the watch check on)
    #[k(1.0)] pub max_away: f64,
    /// the telegraph, seconds before he walks: the whole lead (the watch check + glance open it), when he says it, when he rubs his neck and turns to go
    #[k(20.0)] pub announce_lead: f64,
    #[k(12.0)] pub say_lead: f64,
    #[k(5.0)] pub stretch_lead: f64,
    /// a colleague his raised voice reaches is told to his face and answers `reply_after` s later: this many metres of open air along the SOUND'S PATH (× its carry;
    ///  the whistle's model); nobody that near and it goes over the net
    #[k(14.0)] pub partner_m: f64,
    #[k(1.5)] pub reply_after: f64,
    /// the dwell per kind (s): standing at the counter / outside / at the cooler; the stretch's is the lap itself (0 = no standing about)
    #[k({ let mut t = JsMap::new(); t.set(crate::needs::NeedKind::Coffee, 25.0); t.set(crate::needs::NeedKind::Smoke, 40.0); t.set(crate::needs::NeedKind::Water, 12.0); t.set(crate::needs::NeedKind::Stretch, 0.0); t })] pub dwell: JsMap<crate::needs::NeedKind, f64>,
    /// a trip only ever starts from a waypoint whose authored wait is at least this (a place he stands at anyway: legible), and — fromNearest — only from the one of
    ///  those nearest the spot by path (so the absence per man and kind is one number); false = any waiting waypoint, the moment the need is full
    #[k(2.0)] pub min_wait: f64,
    /// a kind whose place is nearer than this (m, by path) to the waypoint he would leave from is not dealt him at all: his route covers it already (Novak waits beside the cooler)
    #[k(4.0)] pub min_trip_m: f64,
    /// what a need is put back to when a trip is broken off, and what every need of his is capped at once he has been through an alarm (adrenaline: nobody wants a
    ///  coffee straight after a manhunt)
    #[k(0.5)] pub after_cancel: f64,
    #[k(0.3)] pub after_alert: f64,
    /// seconds after coming back before HE may go again on any need, and the least gap between any two departures floor-wide
    #[k(90.0)] pub rest_secs: f64,
    #[k(30.0)] pub floor_gap_secs: f64,
    /// the walk there and back: × the patrol pace (1 = exactly the route's walk); seconds of no headway on a leg before the trip is given up where he stands
    #[k(1.0)] pub pace_k: f64,
    #[k(6.0)] pub stuck_secs: f64,
}
