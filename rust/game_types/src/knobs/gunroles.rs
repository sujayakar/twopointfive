//! The numbers of gunroles.
use crate::bytes::ToBytes;
use game_derive::Knobs;

#[derive(Clone, Copy, Debug, ToBytes, Knobs)]
pub struct GunrolesK {
    /// the share of a cast's riflemen who carry the AKMS (the rest: the short carbine)
    #[k(0.5)] pub akms: f64,
    /// the belt-fed gun: a clear line this long (m) from a post is a lane, for a cast of `beltMin` men or more · a cast of this many men is a big one, lane
    ///  or none · the furthest a line is measured
    #[k(15.0)] pub belt_lane: f64,
    #[k(5.0)] pub belt_min: f64,
    #[k(10.0)] pub belt_squad: f64,
    #[k(40.0)] pub line_max: f64,
    /// a pistol: a post watching something within this (m) · the share of such men who carry one · and of a whole cast at most this share (the man with a
    ///  hostage in his hand is not counted)
    #[k(4.0)] pub pistol_near: f64,
    #[k(0.5)] pub pistol_share: f64,
    #[k(0.25)] pub pistol_max: f64,
    /// the cover's deal: what a far opening is worth to a long gun over a short one, on that deal's scale (cover.ts: a kept opening's `sticky` is 1.2)
    #[k(0.6)] pub lane_k: f64,
    /// orders.ts pair: what a room gun standing behind a long gun in the stack costs, in metres of walking (a pair kept in one file is worth 1000)
    #[k(400.0)] pub file_cost: f64,
}
