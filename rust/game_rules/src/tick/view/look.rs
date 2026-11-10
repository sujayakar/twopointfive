//! The numbers of the rule tables that say how a HUD lays out what it is told: the ring's by which the game reads a cursor, the plan's, a far line's. They are the game's own rows, so
//! a page holds no second copy of a number a rule reads and a knob turned reaches the picture.
use super::*;
use crate::game::Game;

/// the look, as the tables stand
pub(super) fn hud_look(g: &Game) -> HudLook {
    let (k, b, l) = (&g.k, crate::ringlone::ring_box(), crate::ringlone::lone());
    HudLook {
        ring: RingLook {
            places: k.cmdring.wedges, hole: crate::ringlone::RING_HOLE, rim: crate::ringlone::RING_RIM, box_x: b.x, box_y: b.y,
            lone: LoneLook { r: l.r, h: l.h, pad: l.pad, letter: l.char, max: l.max, lead: l.lead },
        },
        plan: PlanLook { fresh_secs: k.plan.fresh_secs, walk_dim: k.plan.walk_dim, rest_dim: k.plan.rest_dim, ring_m: k.plan.ring_m, sector_m: k.plan.sector_m, sector_deg: k.plan.sector_deg, reask_secs: k.plan.reask_secs, others: k.plan.others },
        net: NetLook { edge: k.squadnet.edge, near: k.squadnet.near, edge_in: k.squadnet.edge_in },
        attention: AttentionLook { ring_m: k.attention.ring_m, way_reask: k.attention.way_reask },
        line_clear: k.rails.line_clear,
        mark_clear: k.rails.mark_clear,
    }
}
