//! The numbers of glass.
use crate::bytes::ToBytes;
use game_derive::Knobs;

/// the numbers
#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct GlassK {
    /// straight-line horizon of a round smacking a pane (m): inside it the usual propagation decides how much arrives
    #[k(30.0)] pub hear_m: f64,
    /// event level: a bullet into glass is a report of its own. Distance and how the way carries scale it into a man's hearing, and his awareness goes toward min(0.85, 0.4 + that)
    #[k(0.85)] pub hear_level: f64,
    /// a stun charge within this of a pane's nearest point stars it (overpressure) …
    #[k(2.5)] pub bang_m: f64,
    /// … with this many impact stars per pane, at points seeded off where the charge went off
    #[k(2.0)] pub bang_cracks: f64,
    /// a hit's crack star reaches 7.5‥20 cm from the hole (15‥40 cm across), drawn off the seed
    #[k(0.075)] pub r_min: f64,
    #[k(0.2)] pub r_max: f64,
    /// s, ONLY while live: across the whole floor no two hit cracks START closer than this (the hearing event is unaffected); 0 = every one sounds
    #[k(0.045)] pub crack_gap: f64,
    /// s, ONLY while glass breaking is live: one 'glass' hearing event per pane per SHOOTER per gap (the crack itself still sounds per round, softer inside it); 0 = every hit its own event
    #[k(0.2)] pub live_gap: f64,
    /// m off each face of the pane those two points stand: past one 0.5 m walk-grid cell, because a routed sound is searched — and kept for half a second — per CELL, and a point sharing the struck point's cell would be handed that point's answer
    #[k(0.55)] pub ear_off: f64,
}
